//! Each side's raw and refined materials, and the facility cycles that fill
//! and draw them (`ghidra/notes/top-bar-resource-counters.md`).
//!
//! Every mine, refinery and yard runs a cycle (`FUN_0053aa40`): it waits
//! for its input (state 1), works for a delay (state 2), then pays its
//! output (state 3) and waits again. A mine needs no input and adds one raw
//! unit; a refinery takes one raw and adds one refined; a yard takes one
//! refined and adds one unit of work to its area's build. An idle yard
//! (state 4) draws nothing: its manager starts only as many yards as there
//! are units of work left (`FUN_00529dd0`).
//!
//! A mine's or refinery's delay grows with its share of the side's
//! maintenance load and with low support at its system (`FUN_0055e2b0`).
//! While the side's load exceeds its capacity, one unit is scrapped every
//! 10 days (`FUN_00530350`).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::build_selection::area_family;
use crate::dat::Faction;
use crate::economy::EconomyState;
use crate::ids::{ManufacturingFacilityKey, ProductionFacilityKey, SystemKey};
use crate::manufacturing::{ManufacturingState, ProductionArea};
use crate::mission_detection::MissionRng;
use crate::resources::{maintenance_used, FACILITY_MAINTENANCE_CAPACITY};
use crate::scrap::{overdraft_candidates, ScrapTarget};
use crate::tick::TickEvent;
use crate::uprising::Draws;
use crate::world::GameWorld;

/// GNPRTB 4096: the percentage of a facility's capacity that makes one
/// day of load (`FUN_0055e2b0`: `max(1, capacity × 20 / 100)`, 10).
const GNPRTB_LOAD_STEP: u16 = 4096;
/// GNPRTB 7763: the support at which a facility works at its full rate
/// (`FUN_00559c10`, 100).
const GNPRTB_EFFICIENCY_BASE: u16 = 7763;
/// GNPRTB 7168: days between overdraft scraps (timer `0x382`, record
/// `+0xcc`; `FUN_00532350`).
const GNPRTB_OVERDRAFT_PERIOD: u16 = 7168;

/// Where a facility stands in its cycle: the facility's `+0x58`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CycleStage {
    /// State 4: stopped. Idle yards rest here.
    #[default]
    Idle,
    /// State 1: waiting for its input.
    Waiting,
    /// State 2: working until `ready_tick` (timer `0x394`).
    Working,
}

/// One facility's cycle.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FacilityCycle {
    pub stage: CycleStage,
    /// The tick its work ends, while working.
    pub ready_tick: u64,
    /// `+0x60` bit 4: set when the facility comes online (state 4 → 1), so
    /// its first delay is randomised (`FUN_0053b1d0`, `FUN_0055e2b0`).
    pub first_cycle: bool,
    /// A mine's or refinery's share of its side's maintenance load: the
    /// second half of its `+0x64` pair (`FUN_0052f6b0`).
    pub load: i64,
    /// While waiting for input: its place in its side's request queue
    /// (`FUN_0052fbb0`, `FUN_0052fbd0`; served first in, first out).
    pub request: Option<u64>,
    /// The side it last ran for; a change of hands restarts the cycle.
    pub side: Option<Faction>,
}

/// A side's stockpiles and overdraft timer.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SideStock {
    /// Side `+0x78`: mined material not yet refined.
    pub raw: i64,
    /// Side `+0x7c`: refined material on hand.
    pub refined: i64,
    /// While the load exceeds the capacity (side `+0xa8`), the tick timer
    /// `0x382` next fires.
    pub overdraft_due: Option<u64>,
}

/// The stockpiles and every facility's cycle. Saved with the game.
///
/// hyp: a new game starts with nothing in either stockpile; only the cycle
/// setters and the stream readers write side `+0x78` and `+0x7c`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StockpileState {
    pub alliance: SideStock,
    pub empire: SideStock,
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    production: HashMap<ProductionFacilityKey, FacilityCycle>,
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    yards: HashMap<ManufacturingFacilityKey, FacilityCycle>,
    /// The next request number.
    next_request: u64,
}

impl StockpileState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// A side's stockpiles.
    #[must_use]
    pub fn side(&self, side: Faction) -> &SideStock {
        if side == Faction::Alliance {
            &self.alliance
        } else {
            &self.empire
        }
    }

    /// A side's stockpiles, to change.
    pub fn side_mut(&mut self, side: Faction) -> &mut SideStock {
        if side == Faction::Alliance {
            &mut self.alliance
        } else {
            &mut self.empire
        }
    }

    /// A mine's or refinery's cycle.
    #[must_use]
    pub fn production_cycle(&self, key: ProductionFacilityKey) -> Option<&FacilityCycle> {
        self.production.get(&key)
    }

    /// A yard's cycle.
    #[must_use]
    pub fn yard_cycle(&self, key: ManufacturingFacilityKey) -> Option<&FacilityCycle> {
        self.yards.get(&key)
    }
}

/// What a day of cycles did, for telemetry and messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StockpileEvent {
    /// A finished mine's or refinery's unit went to the other side
    /// (`FUN_005166a0`).
    Diverted {
        system: SystemKey,
        from: Faction,
        refined: bool,
    },
    /// The overdraft timer picked a unit to scrap (`FUN_00530350`). The
    /// caller scraps it (`crate::scrap::scrap`).
    OverdraftScrap { side: Faction, target: ScrapTarget },
}

/// The stockpile phase, run each tick after the economy and before
/// manufacturing completes its units.
pub struct StockpileSystem;

/// The two sides, in the order the port walks them.
const SIDES: [Faction; 2] = [Faction::Alliance, Faction::Empire];

impl StockpileSystem {
    /// Run every tick in `tick_events`. `rolls` holds one roll per tick
    /// (padded with 1.0 by the caller), which seeds that tick's draws.
    /// Yards at `blocked` systems hold their cycles (manual p. 84: building
    /// "is suspended if the system is under blockade").
    pub fn advance(
        state: &mut StockpileState,
        world: &GameWorld,
        manufacturing: &mut ManufacturingState,
        economy: &EconomyState,
        tick_events: &[TickEvent],
        blocked: &std::collections::HashSet<SystemKey>,
        rolls: &[f64],
    ) -> Vec<StockpileEvent> {
        let mut events = Vec::new();
        for (index, tick) in tick_events.iter().enumerate() {
            let roll = rolls.get(index).copied().unwrap_or(1.0);
            let mut draws = MissionRng::seeded(roll, tick.tick);
            Self::day(
                state,
                world,
                manufacturing,
                economy,
                tick.tick,
                blocked,
                &mut draws,
                &mut events,
            );
        }
        events
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "one day over the stockpile, world, manufacturing and economy states"
    )]
    fn day(
        state: &mut StockpileState,
        world: &GameWorld,
        manufacturing: &mut ManufacturingState,
        economy: &EconomyState,
        tick: u64,
        blocked: &std::collections::HashSet<SystemKey>,
        draws: &mut impl Draws,
        events: &mut Vec<StockpileEvent>,
    ) {
        sync_facilities(state, world);
        for side in SIDES {
            let load = maintenance_used(world, manufacturing, side);
            spread_side_load(state, world, side, load);
        }
        // A yard's unit of progress reassigns the yards at once
        // (`FUN_0052a430` → `FUN_00529dd0`), so surplus yards stop before
        // they draw again; new orders are assigned before anything starts.
        finish_work(
            state,
            world,
            manufacturing,
            economy,
            tick,
            blocked,
            draws,
            events,
        );
        assign_yards(state, world, manufacturing, blocked);
        start_work(state, world, tick, blocked, draws);
        for side in SIDES {
            overdraft(state, world, manufacturing, side, tick, draws, events);
        }
    }
}

/// A facility's system, by its key list.
fn production_system(world: &GameWorld, key: ProductionFacilityKey) -> Option<SystemKey> {
    world
        .systems
        .iter()
        .find(|(_, system)| system.production_facilities.contains(&key))
        .map(|(system_key, _)| system_key)
}

/// A yard's system, by its key list.
fn yard_system(world: &GameWorld, key: ManufacturingFacilityKey) -> Option<SystemKey> {
    world
        .systems
        .iter()
        .find(|(_, system)| system.manufacturing_facilities.contains(&key))
        .map(|(system_key, _)| system_key)
}

/// Drop the cycles of facilities that are gone and start those of new ones.
/// A mine or refinery comes online at once (`FUN_0055aa80` → `FUN_0053aac0`:
/// state 4 → 1 for a completed, enabled facility), its first cycle
/// randomised; a yard waits idle for its manager. A facility that changed
/// hands starts again for its new side. port: the port's facilities exist
/// only once completed, and none is disabled.
fn sync_facilities(state: &mut StockpileState, world: &GameWorld) {
    state
        .production
        .retain(|key, _| world.production_facilities.contains_key(*key));
    state
        .yards
        .retain(|key, _| world.manufacturing_facilities.contains_key(*key));
    for (key, facility) in &world.production_facilities {
        let cycle = state.production.entry(key).or_default();
        if cycle.side != Some(facility.side) {
            // A facility with no side stays stopped: `FUN_0053aa40` holds
            // state 4 while its side bits are `0xc0`.
            let held = SIDES.contains(&facility.side);
            *cycle = FacilityCycle {
                stage: if held {
                    CycleStage::Waiting
                } else {
                    CycleStage::Idle
                },
                first_cycle: held,
                side: Some(facility.side),
                ..FacilityCycle::default()
            };
        }
    }
    for (key, yard) in &world.manufacturing_facilities {
        let cycle = state.yards.entry(key).or_default();
        if cycle.side != Some(yard.side) {
            *cycle = FacilityCycle {
                side: Some(yard.side),
                ..FacilityCycle::default()
            };
        }
    }
}

/// Spread the load over facilities that each hold `capacity` and already
/// carry `shares`, in walk order (`FUN_0052f6b0`, `FUN_0052f8f0`). Only the
/// change is dealt: `min(load − allocated, capacity_total − allocated)`, so
/// a facility added without a change of load gets no share. Adding gives
/// each facility `allocated × capacity / total − share + 1` per walk
/// (`FUN_0055a820`), one unit each with equal capacities; removing takes
/// `share − (allocated − 1) × capacity / total` (`FUN_0055a960`), from the
/// fullest first. Walks repeat while some facility changed and has room.
#[must_use]
pub fn spread_load(shares: &[i64], capacity: i64, load: i64) -> Vec<i64> {
    let mut shares = shares.to_vec();
    let total = capacity * i64::try_from(shares.len()).unwrap_or(i64::MAX);
    let mut allocated: i64 = shares.iter().sum();
    let mut delta = (load - allocated).min(total - allocated);
    if delta == 0 || total == 0 {
        return shares;
    }
    let removing = delta < 0;
    delta = delta.abs();
    let mut again = true;
    while again && delta > 0 {
        again = false;
        for share in &mut shares {
            if delta == 0 {
                break;
            }
            if removing {
                let target = (allocated - 1) * capacity / total;
                let amount = (*share - target).clamp(0, delta);
                *share -= amount;
                allocated -= amount;
                delta -= amount;
                if amount > 0 && *share != 0 {
                    again = true;
                }
            } else {
                let target = allocated * capacity / total;
                let amount = (target - *share + 1).clamp(0, delta.min(capacity - *share));
                *share += amount;
                allocated += amount;
                delta -= amount;
                if amount > 0 && *share != capacity {
                    again = true;
                }
            }
        }
    }
    shares
}

/// Spread `load` over the side's mines and, separately, its refineries.
fn spread_side_load(state: &mut StockpileState, world: &GameWorld, side: Faction, load: i64) {
    for mines in [true, false] {
        let keys: Vec<ProductionFacilityKey> = world
            .production_facilities
            .iter()
            .filter(|(_, facility)| facility.side == side && facility.is_mine == mines)
            .map(|(key, _)| key)
            .collect();
        let shares: Vec<i64> = keys
            .iter()
            .map(|key| state.production.get(key).map_or(0, |cycle| cycle.load))
            .collect();
        let spread = spread_load(&shares, FACILITY_MAINTENANCE_CAPACITY, load);
        for (key, share) in keys.iter().zip(spread) {
            if let Some(cycle) = state.production.get_mut(key) {
                cycle.load = share;
            }
        }
    }
}

/// The area whose yards are of `family`.
fn yard_area(family: u8) -> Option<ProductionArea> {
    ProductionArea::ALL
        .into_iter()
        .find(|&area| area_family(area) == family)
}

/// Keep each manager's active yards equal to its work left
/// (`FUN_00529dd0`): start idle yards, the lowest `processing_rate` first;
/// stop waiting yards before working ones, the slowest first. A manager is
/// its system's area; its yards are the system's yards of that area held by
/// the system's holder. Yards at `blocked` systems are left as they are.
fn assign_yards(
    state: &mut StockpileState,
    world: &GameWorld,
    manufacturing: &ManufacturingState,
    blocked: &std::collections::HashSet<SystemKey>,
) {
    for (system_key, system) in &world.systems {
        if blocked.contains(&system_key) {
            continue;
        }
        for area in ProductionArea::ALL {
            let work = manufacturing
                .queue(system_key, area)
                .map_or(0, |queue| queue.work_left() as usize);
            let rate = |key: ManufacturingFacilityKey| {
                world
                    .manufacturing_facilities
                    .get(key)
                    .and_then(|yard| world.buildable_classes.get(&yard.class_dat_id))
                    .map_or(0, |class| class.processing_rate)
            };
            let yards: Vec<ManufacturingFacilityKey> = system
                .manufacturing_facilities
                .iter()
                .copied()
                .filter(|&key| {
                    world.manufacturing_facilities.get(key).is_some_and(|yard| {
                        yard_area(yard.class_dat_id.family()) == Some(area)
                            && system.control.is_controlled_by(yard.side)
                    })
                })
                .collect();
            let stage = |state: &StockpileState, key| {
                state.yards.get(&key).map_or(CycleStage::Idle, |c| c.stage)
            };
            let mut active = yards
                .iter()
                .filter(|&&key| stage(state, key) != CycleStage::Idle)
                .count();
            while active < work {
                // The fastest idle yard; the first in walk order on a tie.
                let Some(&key) = yards
                    .iter()
                    .filter(|&&key| stage(state, key) == CycleStage::Idle)
                    .min_by_key(|&&key| rate(key))
                else {
                    break;
                };
                if let Some(cycle) = state.yards.get_mut(&key) {
                    cycle.stage = CycleStage::Waiting;
                }
                active += 1;
            }
            for stopping in [CycleStage::Waiting, CycleStage::Working] {
                while active > work {
                    // The slowest yard in this stage; the first on a tie.
                    let Some(&key) = yards
                        .iter()
                        .filter(|&&key| stage(state, key) == stopping)
                        .min_by_key(|&&key| std::cmp::Reverse(rate(key)))
                    else {
                        break;
                    };
                    if let Some(cycle) = state.yards.get_mut(&key) {
                        *cycle = FacilityCycle {
                            side: cycle.side,
                            ..FacilityCycle::default()
                        };
                    }
                    active -= 1;
                }
            }
        }
    }
}

/// The side across from `side`.
fn other(side: Faction) -> Faction {
    if side == Faction::Alliance {
        Faction::Empire
    } else {
        Faction::Alliance
    }
}

/// End the work that is due: a mine pays one raw unit, a refinery one
/// refined unit, each to its side or, on a diversion roll, to the other
/// (`FUN_00516360`, `FUN_005166a0`); a yard adds one unit of work to its
/// area's build (`FUN_00530950`). The facility then waits for its next
/// input.
#[expect(
    clippy::too_many_arguments,
    reason = "one phase step over the stockpile, world, manufacturing and economy states"
)]
fn finish_work(
    state: &mut StockpileState,
    world: &GameWorld,
    manufacturing: &mut ManufacturingState,
    economy: &EconomyState,
    tick: u64,
    blocked: &std::collections::HashSet<SystemKey>,
    draws: &mut impl Draws,
    events: &mut Vec<StockpileEvent>,
) {
    let due =
        |cycle: &FacilityCycle| cycle.stage == CycleStage::Working && cycle.ready_tick <= tick;
    let finished: Vec<ProductionFacilityKey> = world
        .production_facilities
        .keys()
        .filter(|key| state.production.get(key).is_some_and(due))
        .collect();
    for key in finished {
        let facility = &world.production_facilities[key];
        let system = production_system(world, key);
        let diverted = system.is_some_and(|system| diversion(economy, system, draws));
        let to = if diverted {
            other(facility.side)
        } else {
            facility.side
        };
        let stock = state.side_mut(to);
        if facility.is_mine {
            stock.raw += 1;
        } else {
            stock.refined += 1;
        }
        if diverted {
            if let Some(system) = system {
                events.push(StockpileEvent::Diverted {
                    system,
                    from: facility.side,
                    refined: !facility.is_mine,
                });
            }
        }
        if let Some(cycle) = state.production.get_mut(&key) {
            cycle.stage = CycleStage::Waiting;
        }
    }
    let finished: Vec<ManufacturingFacilityKey> = world
        .manufacturing_facilities
        .keys()
        .filter(|key| state.yards.get(key).is_some_and(due))
        .collect();
    for key in finished {
        let Some(system) = yard_system(world, key) else {
            continue;
        };
        if blocked.contains(&system) {
            continue;
        }
        let yard = &world.manufacturing_facilities[key];
        if let Some(area) = yard_area(yard.class_dat_id.family()) {
            manufacturing.add_progress(system, area);
        }
        if let Some(cycle) = state.yards.get_mut(&key) {
            cycle.stage = CycleStage::Waiting;
        }
    }
}

/// The chance, in percent, that a finished unit at `system` goes to the
/// other side: the magnitude of its support drift (system `+0x6c`,
/// `FUN_00559c40`), rolled with `FUN_0053e2f0`.
fn diversion(economy: &EconomyState, system: SystemKey, draws: &mut impl Draws) -> bool {
    let percent = economy
        .per_system
        .get(&system)
        .map_or(0, |value| i32::from(value.support_drift).abs());
    percent != 0 && draws.chance(percent)
}

/// The working delay of a mine or refinery (`FUN_0055e2b0`):
/// `(ceil(load / k) + processing_rate) × efficiency / 100`, with
/// `k = max(1, capacity × GNPRTB 4096 / 100)`; a first cycle draws
/// `rand(0..=d) + d / 2`. port: at least one day, as the port's clock
/// counts whole days.
fn production_delay(
    world: &GameWorld,
    key: ProductionFacilityKey,
    cycle: &FacilityCycle,
    draws: &mut impl Draws,
) -> u64 {
    let facility = &world.production_facilities[key];
    let rate = world
        .buildable_classes
        .get(&facility.class_dat_id)
        .map_or(0, |class| i64::from(class.processing_rate));
    let step = (FACILITY_MAINTENANCE_CAPACITY
        * i64::from(world.gnprtb.value(GNPRTB_LOAD_STEP, world.difficulty_index))
        / 100)
        .max(1);
    let efficiency = production_system(world, key)
        .and_then(|system| world.systems.get(system))
        .map_or(100, |system| efficiency(world, system));
    let base = (cycle.load + step - 1) / step + rate;
    let mut delay = base * efficiency / 100;
    if cycle.first_cycle {
        let spread = i32::try_from(delay).unwrap_or(i32::MAX);
        delay = i64::from(draws.draw(spread)) + delay / 2;
    }
    u64::try_from(delay.max(1)).unwrap_or(1)
}

/// A facility's efficiency, its `+0x6c` copied from the system's `+0x70`
/// (`FUN_0050b2c0`): 100 at an unheld system, else
/// `GNPRTB 7763 × 100 / max(support, 1)` for the holder (`FUN_00559c10`).
#[must_use]
pub fn efficiency(world: &GameWorld, system: &crate::world::System) -> i64 {
    let support = match system.control.faction() {
        Some(Faction::Alliance) => system.popularity_alliance,
        Some(Faction::Empire) => system.popularity_empire,
        _ => return 100,
    };
    let base = i64::from(
        world
            .gnprtb
            .value(GNPRTB_EFFICIENCY_BASE, world.difficulty_index)
            .max(1),
    );
    #[expect(
        clippy::cast_possible_truncation,
        reason = "support is a fraction in [0, 1]"
    )]
    let percent = i64::from(((support * 100.0).round() as i32).max(1));
    (base * 100 / percent).max(1)
}

/// Give waiting facilities their input and start their work: a mine takes
/// none (`FUN_00530650`); a refinery takes one raw unit and a yard one
/// refined unit, or join their side's queue and are served in the order
/// they asked (`FUN_005306b0`, `FUN_00530820`, `FUN_0052fbf0`,
/// `FUN_0052fd30`). A yard works for its `processing_rate`
/// (`FUN_0052cc90`).
fn start_work(
    state: &mut StockpileState,
    world: &GameWorld,
    tick: u64,
    blocked: &std::collections::HashSet<SystemKey>,
    draws: &mut impl Draws,
) {
    // Number every new request in walk order.
    let mut next = state.next_request;
    let waiting = |cycle: &FacilityCycle| cycle.stage == CycleStage::Waiting;
    for (key, facility) in &world.production_facilities {
        if let Some(cycle) = state.production.get_mut(&key) {
            if waiting(cycle) && !facility.is_mine && cycle.request.is_none() {
                cycle.request = Some(next);
                next += 1;
            }
        }
    }
    for key in world.manufacturing_facilities.keys() {
        if let Some(cycle) = state.yards.get_mut(&key) {
            if waiting(cycle) && cycle.request.is_none() {
                cycle.request = Some(next);
                next += 1;
            }
        }
    }
    state.next_request = next;

    // Mines start at once.
    let mines: Vec<ProductionFacilityKey> = world
        .production_facilities
        .iter()
        .filter(|(key, facility)| {
            facility.is_mine && state.production.get(key).is_some_and(waiting)
        })
        .map(|(key, _)| key)
        .collect();
    for key in mines {
        let delay = production_delay(world, key, &state.production[&key], draws);
        start(state.production.get_mut(&key), tick + delay);
    }

    // Refineries draw raw, oldest request first.
    let mut refineries: Vec<(u64, ProductionFacilityKey)> = world
        .production_facilities
        .iter()
        .filter_map(|(key, facility)| {
            let cycle = state.production.get(&key)?;
            (!facility.is_mine && waiting(cycle)).then_some((cycle.request?, key))
        })
        .collect();
    refineries.sort_unstable();
    for (_, key) in refineries {
        let side = world.production_facilities[key].side;
        if state.side(side).raw <= 0 {
            continue;
        }
        state.side_mut(side).raw -= 1;
        let delay = production_delay(world, key, &state.production[&key], draws);
        start(state.production.get_mut(&key), tick + delay);
    }

    // Yards draw refined, oldest request first.
    let mut yards: Vec<(u64, ManufacturingFacilityKey)> = world
        .manufacturing_facilities
        .keys()
        .filter_map(|key| {
            let cycle = state.yards.get(&key)?;
            waiting(cycle).then_some((cycle.request?, key))
        })
        .collect();
    yards.sort_unstable();
    for (_, key) in yards {
        let yard = &world.manufacturing_facilities[key];
        let held = yard_system(world, key).is_some_and(|system| blocked.contains(&system));
        if held || state.side(yard.side).refined <= 0 {
            continue;
        }
        state.side_mut(yard.side).refined -= 1;
        let rate = world
            .buildable_classes
            .get(&yard.class_dat_id)
            .map_or(1, |class| u64::from(class.processing_rate.max(1)));
        start(state.yards.get_mut(&key), tick + rate);
    }
}

/// Move a waiting facility to work until `ready`.
fn start(cycle: Option<&mut FacilityCycle>, ready: u64) {
    if let Some(cycle) = cycle {
        cycle.stage = CycleStage::Working;
        cycle.ready_tick = ready;
        cycle.first_cycle = false;
        cycle.request = None;
    }
}

/// The overdraft timer (`0x382`): armed when the side's load exceeds its
/// capacity (`FUN_0052f670`, side `+0xa8`) and disarmed when it fits; each
/// firing, every GNPRTB 7168 days, scraps one random unit that costs
/// maintenance (`FUN_00530350`). hyp: the first firing comes one period
/// after the timer is armed.
fn overdraft(
    state: &mut StockpileState,
    world: &GameWorld,
    manufacturing: &ManufacturingState,
    side: Faction,
    tick: u64,
    draws: &mut impl Draws,
    events: &mut Vec<StockpileEvent>,
) {
    let capacity = crate::resources::maintenance_capacity(world, side);
    let overdrawn = maintenance_used(world, manufacturing, side) > capacity;
    let period = u64::try_from(
        world
            .gnprtb
            .value(GNPRTB_OVERDRAFT_PERIOD, world.difficulty_index)
            .max(1),
    )
    .unwrap_or(1);
    let stock = state.side_mut(side);
    if !overdrawn {
        stock.overdraft_due = None;
        return;
    }
    let due = *stock.overdraft_due.get_or_insert(tick + period);
    if due > tick {
        return;
    }
    stock.overdraft_due = Some(tick + period);
    let candidates = overdraft_candidates(world, side);
    if let Some(index) = draws.pick(candidates.len()) {
        events.push(StockpileEvent::OverdraftScrap {
            side,
            target: candidates[index],
        });
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::dat::{ExplorationStatus, SectorGroup};
    use crate::economy::SystemEconomy;
    use crate::ids::DatId;
    use crate::manufacturing::{BuildableKind, ManufacturingSystem, QueueItem};
    use crate::world::{
        BuildableClass, ControlKind, GnprtbParams, ManufacturingFacilityInstance,
        ProductionFacilityInstance, Sector, System, TroopUnit,
    };

    const MINE: u32 = 0x2c00_0001;
    const REFINERY: u32 = 0x2d00_0002;
    const SHIPYARD: u32 = 0x2800_0001;
    const ADVANCED_SHIPYARD: u32 = 0x2800_0002;
    const REGIMENT: u32 = 0x1000_0001;

    /// One Alliance-held system at full support, with the shipped
    /// parameters (GNPRTB 4096 = 20, 7168 = 10, 7763 = 100) and the
    /// classes: PROFACSD mine and refinery (processing_rate 5), MANFACSD
    /// shipyards (rates 4 and 2), and a regiment costing 10 refined and 25
    /// maintenance.
    fn world() -> (GameWorld, SystemKey) {
        let mut world = GameWorld {
            gnprtb: GnprtbParams::uniform(&[(4096, 20), (7168, 10), (7763, 100)]),
            ..GameWorld::default()
        };
        let class = |processing_rate, refined_material_cost, maintenance_cost| BuildableClass {
            processing_rate,
            refined_material_cost,
            maintenance_cost,
            ..BuildableClass::default()
        };
        world
            .buildable_classes
            .insert(DatId::new(MINE), class(5, 20, 0));
        world
            .buildable_classes
            .insert(DatId::new(REFINERY), class(5, 20, 0));
        world
            .buildable_classes
            .insert(DatId::new(SHIPYARD), class(4, 30, 4));
        world
            .buildable_classes
            .insert(DatId::new(ADVANCED_SHIPYARD), class(2, 30, 4));
        world
            .buildable_classes
            .insert(DatId::new(REGIMENT), class(0, 10, 25));
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(0x8000_0001),
            name: "Core".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let system = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0001),
            name: "Bortras".into(),
            sector,
            x: 0,
            y: 0,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 1.0,
            popularity_empire: 0.0,
            is_populated: true,
            total_energy: 10,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Alliance),
        });
        (world, system)
    }

    fn production(
        world: &mut GameWorld,
        system: SystemKey,
        is_mine: bool,
    ) -> ProductionFacilityKey {
        let side = world.systems[system]
            .control
            .faction()
            .unwrap_or(Faction::Neutral);
        let key = world
            .production_facilities
            .insert(ProductionFacilityInstance {
                class_dat_id: DatId::new(if is_mine { MINE } else { REFINERY }),
                side,
                is_mine,
            });
        world.systems[system].production_facilities.push(key);
        key
    }

    fn yard(world: &mut GameWorld, system: SystemKey, class: u32) -> ManufacturingFacilityKey {
        let key = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: DatId::new(class),
                side: Faction::Alliance,
                is_shipyard: true,
            });
        world.systems[system].manufacturing_facilities.push(key);
        key
    }

    fn regiment(world: &mut GameWorld, system: SystemKey) {
        let key = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(REGIMENT),
            is_alliance: true,
            regiment_strength: 100,
        });
        world.systems[system].ground_units.push(key);
    }

    /// Everything the phase reads and writes, run day by day.
    struct Run {
        state: StockpileState,
        manufacturing: ManufacturingState,
        economy: EconomyState,
        blocked: HashSet<SystemKey>,
        events: Vec<StockpileEvent>,
    }

    impl Run {
        fn new() -> Self {
            Self {
                state: StockpileState::new(),
                manufacturing: ManufacturingState::new(),
                economy: EconomyState::default(),
                blocked: HashSet::new(),
                events: Vec::new(),
            }
        }

        fn day(&mut self, world: &GameWorld, tick: u64) {
            let events = StockpileSystem::advance(
                &mut self.state,
                world,
                &mut self.manufacturing,
                &self.economy,
                &[TickEvent { tick }],
                &self.blocked,
                &[0.5],
            );
            self.events.extend(events);
        }

        /// The Alliance's stockpiles after each day of `days`.
        fn days(
            &mut self,
            world: &GameWorld,
            days: std::ops::RangeInclusive<u64>,
        ) -> Vec<(u64, i64, i64)> {
            days.map(|tick| {
                self.day(world, tick);
                let stock = self.state.side(Faction::Alliance);
                (tick, stock.raw, stock.refined)
            })
            .collect()
        }
    }

    /// The days on which `field` of the Alliance's stockpiles went up.
    fn rises(days: &[(u64, i64, i64)], field: fn(&(u64, i64, i64)) -> i64) -> Vec<u64> {
        days.windows(2)
            .filter(|pair| field(&pair[1]) > field(&pair[0]))
            .map(|pair| pair[1].0)
            .collect()
    }

    // FUN_0055a820: with equal capacities the load is dealt one unit to each
    // facility per walk, the earlier ones holding the extra.
    #[test]
    fn the_load_is_dealt_round_robin_one_unit_at_a_time() {
        assert_eq!(spread_load(&[0, 0, 0], 50, 7), [3, 2, 2]);
    }

    // FUN_0052f6b0: only the change in load is dealt, so a facility added
    // without one gets no share.
    #[test]
    fn a_new_facility_gets_no_share_until_the_load_changes() {
        assert_eq!(spread_load(&[5, 5, 0], 50, 10), [5, 5, 0]);
        assert_eq!(spread_load(&[5, 5, 0], 50, 11), [5, 5, 1]);
    }

    // FUN_0055a960: a falling load leaves the fullest facilities first.
    #[test]
    fn a_falling_load_leaves_the_fullest_facilities_first() {
        assert_eq!(spread_load(&[3, 2, 2], 50, 4), [2, 1, 1]);
    }

    // FUN_0052f6b0: the change is capped at the capacity left.
    #[test]
    fn no_facility_takes_more_than_its_capacity() {
        assert_eq!(spread_load(&[0, 0], 50, 150), [50, 50]);
    }

    // FUN_0052f6b0: the change is min(load − allocated, capacity −
    // allocated), so shares above the capacity are cut back to it.
    #[test]
    fn shares_above_the_capacity_are_cut_back_to_it() {
        assert_eq!(spread_load(&[30, 30], 25, 55), [25, 25]);
    }

    // FUN_00559c10, FUN_0050b2c0: 100 at an unheld system, else
    // GNPRTB 7763 × 100 / support.
    #[test]
    fn efficiency_falls_with_the_holders_support() {
        let (mut world, system) = world();
        assert_eq!(efficiency(&world, &world.systems[system]), 100);
        world.systems[system].popularity_alliance = 0.5;
        assert_eq!(efficiency(&world, &world.systems[system]), 200);
        world.systems[system].control = ControlKind::Uncontrolled;
        assert_eq!(efficiency(&world, &world.systems[system]), 100);
    }

    // FUN_00530670: a mine adds one raw unit per cycle;
    // FUN_0055e2b0: after the first, randomised cycle, a mine with no load
    // works its processing_rate (5) days.
    #[test]
    fn a_mine_adds_one_raw_unit_every_cycle() {
        let (mut world, system) = world();
        production(&mut world, system, true);
        let mut run = Run::new();
        let days = run.days(&world, 1..=30);
        let mined = rises(&days, |day| day.1);
        assert!(
            mined[0] <= 1 + 7,
            "the first cycle is rand(0..=5) + 2: {mined:?}"
        );
        assert!(
            mined.windows(2).all(|pair| pair[1] - pair[0] == 5),
            "{mined:?}"
        );
    }

    // FUN_0055e2b0: every 10 units of load (GNPRTB 4096: 50 × 20 / 100) add a
    // day: a share of 25 makes (3 + 5) days.
    #[test]
    fn a_mines_cycle_lengthens_with_its_share_of_the_load() {
        let (mut world, system) = world();
        let mine = production(&mut world, system, true);
        regiment(&mut world, system);
        let mut run = Run::new();
        let days = run.days(&world, 1..=40);
        assert_eq!(run.state.production_cycle(mine).unwrap().load, 25);
        let mined = rises(&days, |day| day.1);
        assert!(mined.len() >= 3, "{mined:?}");
        assert!(
            mined.windows(2).all(|pair| pair[1] - pair[0] == 8),
            "{mined:?}"
        );
    }

    // FUN_00559c10: half the support doubles a cycle: 5 days become 10.
    #[test]
    fn low_support_slows_a_mine() {
        let (mut world, system) = world();
        world.systems[system].popularity_alliance = 0.5;
        production(&mut world, system, true);
        let mut run = Run::new();
        let days = run.days(&world, 1..=40);
        let mined = rises(&days, |day| day.1);
        assert!(
            mined.windows(2).all(|pair| pair[1] - pair[0] == 10),
            "{mined:?}"
        );
    }

    // FUN_005306b0: a refinery takes one raw unit to start, and
    // FUN_005307e0 adds one refined unit when it is done; nothing is refined
    // that was not mined.
    #[test]
    fn a_refinery_turns_each_raw_unit_into_a_refined_unit() {
        let (mut world, system) = world();
        production(&mut world, system, true);
        production(&mut world, system, false);
        let mut run = Run::new();
        let days = run.days(&world, 1..=40);
        let first_raw = days.iter().position(|day| day.1 > 0 || day.2 > 0).unwrap();
        let first_refined = days.iter().position(|day| day.2 > 0).unwrap();
        assert!(first_refined > first_raw);
        let (_, raw, refined) = *days.last().unwrap();
        assert!(refined > 0);
        // Each refined unit was mined first; the rest wait as raw or in the
        // refinery.
        let mined = rises(&days, |day| day.1).len() as i64;
        assert!(
            refined + raw <= mined + 1,
            "raw {raw} refined {refined} mined {mined}"
        );
    }

    // FUN_00516360, FUN_005166a0: a finished unit at a system whose support
    // drift is 100 always goes to the other side.
    #[test]
    fn a_restless_systems_output_goes_to_the_other_side() {
        let (mut world, system) = world();
        production(&mut world, system, true);
        let mut run = Run::new();
        run.economy.per_system.insert(
            system,
            SystemEconomy {
                support_drift: 100,
                ..SystemEconomy::default()
            },
        );
        run.days(&world, 1..=20);
        assert_eq!(run.state.side(Faction::Alliance).raw, 0);
        assert!(run.state.side(Faction::Empire).raw > 0);
        assert!(run.events.iter().any(|event| matches!(
            event,
            StockpileEvent::Diverted {
                from: Faction::Alliance,
                refined: false,
                ..
            }
        )));
    }

    // FUN_0053aa40: a facility with no side holds state 4.
    #[test]
    fn a_facility_with_no_side_stays_stopped() {
        let (mut world, system) = world();
        world.systems[system].control = ControlKind::Uncontrolled;
        let key = production(&mut world, system, true);
        assert_eq!(world.production_facilities[key].side, Faction::Neutral);
        let mut run = Run::new();
        run.days(&world, 1..=20);
        assert_eq!(
            run.state.production_cycle(key).unwrap().stage,
            CycleStage::Idle
        );
        assert_eq!(run.state.side(Faction::Empire).raw, 0);
        assert_eq!(run.state.side(Faction::Alliance).raw, 0);
    }

    // FUN_00529dd0: with no work the manager keeps every yard stopped, so
    // no refined material is drawn.
    #[test]
    fn an_idle_yard_draws_nothing() {
        let (mut world, system) = world();
        let key = yard(&mut world, system, SHIPYARD);
        let mut run = Run::new();
        run.state.side_mut(Faction::Alliance).refined = 10;
        run.days(&world, 1..=10);
        assert_eq!(run.state.side(Faction::Alliance).refined, 10);
        assert_eq!(run.state.yard_cycle(key).unwrap().stage, CycleStage::Idle);
    }

    // FUN_00529dd0: the manager starts as many yards as units of work, the
    // lowest processing_rate first.
    #[test]
    fn work_starts_the_fastest_idle_yard_first() {
        let (mut world, system) = world();
        let slow = yard(&mut world, system, SHIPYARD);
        let fast = yard(&mut world, system, ADVANCED_SHIPYARD);
        let mut run = Run::new();
        run.state.side_mut(Faction::Alliance).refined = 10;
        let mut ships: slotmap::SlotMap<crate::ids::CapitalShipKey, ()> =
            slotmap::SlotMap::with_key();
        run.manufacturing.enqueue(
            system,
            QueueItem::new(BuildableKind::CapitalShip(ships.insert(())), 1),
        );
        run.day(&world, 1);
        assert_eq!(
            run.state.yard_cycle(fast).unwrap().stage,
            CycleStage::Working
        );
        assert_eq!(run.state.yard_cycle(slow).unwrap().stage, CycleStage::Idle);
        assert_eq!(run.state.side(Faction::Alliance).refined, 9);
    }

    // FUN_00530820, FUN_00530950, FUN_0052a430: each yard cycle draws one
    // refined unit and adds one unit of work; the build takes exactly its
    // cost, and the yards stop when it is done.
    #[test]
    fn a_build_draws_exactly_its_refined_cost_through_its_yards() {
        let (mut world, system) = world();
        yard(&mut world, system, SHIPYARD);
        yard(&mut world, system, ADVANCED_SHIPYARD);
        let mut run = Run::new();
        run.state.side_mut(Faction::Alliance).refined = 10;
        let mut ships: slotmap::SlotMap<crate::ids::CapitalShipKey, ()> =
            slotmap::SlotMap::with_key();
        run.manufacturing.enqueue(
            system,
            QueueItem::new(BuildableKind::CapitalShip(ships.insert(())), 3),
        );
        let mut completed_on = None;
        for tick in 1..=12 {
            run.day(&world, tick);
            let done = ManufacturingSystem::advance(&mut run.manufacturing, &[TickEvent { tick }]);
            if !done.is_empty() && completed_on.is_none() {
                completed_on = Some(tick);
            }
        }
        // Rates 2 and 4 from day 1: progress on days 3, 5 and 5.
        assert_eq!(completed_on, Some(5));
        assert_eq!(run.state.side(Faction::Alliance).refined, 7);
    }

    // FUN_0052fbd0, FUN_0052fd30: a yard with no refined material waits and
    // starts when a unit arrives.
    #[test]
    fn a_yard_waits_for_refined_material() {
        let (mut world, system) = world();
        let key = yard(&mut world, system, SHIPYARD);
        let mut run = Run::new();
        let mut ships: slotmap::SlotMap<crate::ids::CapitalShipKey, ()> =
            slotmap::SlotMap::with_key();
        run.manufacturing.enqueue(
            system,
            QueueItem::new(BuildableKind::CapitalShip(ships.insert(())), 3),
        );
        run.days(&world, 1..=3);
        assert_eq!(
            run.state.yard_cycle(key).unwrap().stage,
            CycleStage::Waiting
        );
        run.state.side_mut(Faction::Alliance).refined = 1;
        run.day(&world, 4);
        assert_eq!(
            run.state.yard_cycle(key).unwrap().stage,
            CycleStage::Working
        );
        assert_eq!(run.state.side(Faction::Alliance).refined, 0);
    }

    // Manual p. 84: building "is suspended if the system is under blockade".
    #[test]
    fn a_blockaded_yard_draws_and_builds_nothing() {
        let (mut world, system) = world();
        yard(&mut world, system, SHIPYARD);
        let mut run = Run::new();
        run.blocked.insert(system);
        run.state.side_mut(Faction::Alliance).refined = 10;
        let mut ships: slotmap::SlotMap<crate::ids::CapitalShipKey, ()> =
            slotmap::SlotMap::with_key();
        run.manufacturing.enqueue(
            system,
            QueueItem::new(BuildableKind::CapitalShip(ships.insert(())), 3),
        );
        run.days(&world, 1..=10);
        assert_eq!(run.state.side(Faction::Alliance).refined, 10);
        let queue = run
            .manufacturing
            .queue(system, ProductionArea::Shipyard)
            .unwrap();
        assert_eq!(queue.work_left(), 3);
    }

    // Manual p. 84: a yard already waiting when the blockade begins does not
    // draw either.
    #[test]
    fn a_waiting_yard_does_not_draw_under_blockade() {
        let (mut world, system) = world();
        let key = yard(&mut world, system, SHIPYARD);
        let mut run = Run::new();
        let mut ships: slotmap::SlotMap<crate::ids::CapitalShipKey, ()> =
            slotmap::SlotMap::with_key();
        run.manufacturing.enqueue(
            system,
            QueueItem::new(BuildableKind::CapitalShip(ships.insert(())), 3),
        );
        run.day(&world, 1);
        assert_eq!(
            run.state.yard_cycle(key).unwrap().stage,
            CycleStage::Waiting
        );
        run.blocked.insert(system);
        run.state.side_mut(Faction::Alliance).refined = 5;
        run.days(&world, 2..=6);
        assert_eq!(run.state.side(Faction::Alliance).refined, 5);
        assert_eq!(
            run.state.yard_cycle(key).unwrap().stage,
            CycleStage::Waiting
        );
    }

    // FUN_0052f670, FUN_00530350: while the load exceeds the capacity, the
    // timer 0x382 scraps one unit that costs maintenance every GNPRTB 7168
    // (10) days; mines and refineries cost none and are never picked.
    #[test]
    fn an_overdrawn_side_scraps_a_unit_every_ten_days() {
        let (mut world, system) = world();
        production(&mut world, system, true);
        regiment(&mut world, system);
        let troop = world.troops.keys().next().unwrap();
        let mut run = Run::new();
        run.days(&world, 1..=10);
        assert!(run
            .events
            .iter()
            .all(|event| !matches!(event, StockpileEvent::OverdraftScrap { .. })));
        run.day(&world, 11);
        assert!(run.events.contains(&StockpileEvent::OverdraftScrap {
            side: Faction::Alliance,
            target: ScrapTarget::Troop(troop),
        }));
        assert_eq!(run.state.side(Faction::Alliance).overdraft_due, Some(21));
    }

    // FUN_0052f670: a side within its capacity has no timer.
    #[test]
    fn a_side_within_its_capacity_scraps_nothing() {
        let (mut world, system) = world();
        production(&mut world, system, true);
        production(&mut world, system, false);
        regiment(&mut world, system);
        let mut run = Run::new();
        run.days(&world, 1..=30);
        assert!(run
            .events
            .iter()
            .all(|event| !matches!(event, StockpileEvent::OverdraftScrap { .. })));
        assert_eq!(run.state.side(Faction::Alliance).overdraft_due, None);
    }
}
