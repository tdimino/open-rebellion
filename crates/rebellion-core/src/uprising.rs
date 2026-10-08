//! Uprisings and the disaster incident, as recovered from REBEXE.EXE.
//!
//! # Uprising (system `+0x88` bit 2)
//!
//! `FUN_0050b800` runs in every system update. A populated system held by the
//! Alliance or the Empire revolts when its troop surplus (`+0x7c`,
//! `FUN_0050b500`) is negative, and stays in revolt while it remains held and
//! populated. Control does not change: `FUN_0050a130` only records the side in
//! revolt. The revolt ends when a successful Subdue Uprising mission
//! (`FUN_00569c20`) runs `FUN_0050c910` and the regiments at the system cover
//! the garrison requirement without the uprising doubling (`FUN_00559fb0`).
//!
//! While in revolt, `FUN_0050a4a0` enables the uprising incident timer
//! (event `0x38d`, `FUN_00510f20`): every GNPRTB 7701 + rand(0..=7702) ticks
//! (`FUN_00586130`, rescheduled by `FUN_005862a0`) `FUN_0050d030` scores the
//! system and looks the score up in UPRIS1TB and UPRIS2TB. Each outcome code
//! destroys a facility or regiment of the holding side, injures one of its
//! characters, or frees prisoners it holds (`FUN_0050d150`). Active Incite
//! Uprising missions then cut the holder's support (`FUN_0050c9f0`).
//!
//! # Disaster (system `+0x88` bit 18)
//!
//! A galaxy timer (event `0x38f`, `FUN_00556fa0`) fires every GNPRTB 7717 +
//! rand(0..=7718) ticks. `FUN_00556b50` picks an existing system at random, and
//! if it has energy or raw materials `FUN_00511930` erodes both and destroys
//! each facility at the system with GNPRTB 7716 percent.
//!
//! Randomness comes from caller-supplied `[0, 1)` rolls consumed in order. An
//! exhausted slice yields 1.0, which draws the maximum value and fails every
//! percent check.
//!
//! Evidence: `ghidra/notes/uprising-incident.md`.

use std::collections::{HashMap, VecDeque};

use serde::{Deserialize, Serialize};

use crate::dat::Faction;
use crate::economy::{self, EconomyState};
use crate::ids::{CharacterKey, SystemKey, TroopKey};
use crate::missions::{member_skill, ActiveMission, MissionKind, MissionState};
use crate::tick::TickEvent;
use crate::world::{Character, GameWorld, MstbTable, Skill, System};

// ---------------------------------------------------------------------------
// GNPRTB parameters (FUN_0053e390 id -> DAT global)
// ---------------------------------------------------------------------------

/// 7701 (`DAT_006bb420`) = 30: uprising incident timer minimum.
const GNPRTB_UPRISING_INCIDENT_MIN: u16 = 7701;
/// 7702 (`DAT_006bb49c`) = 70: uprising incident timer spread.
const GNPRTB_UPRISING_INCIDENT_SPREAD: u16 = 7702;
/// 7707 (`DAT_006bb39c`) = 1: base of each incident score draw.
const GNPRTB_INCIDENT_DRAW_BASE: u16 = 7707;
/// 7708 (`DAT_006bb418`) = 9: spread of each incident score draw.
const GNPRTB_INCIDENT_DRAW_SPREAD: u16 = 7708;
/// 7680 (`DAT_006bb3dc`) = 2: Empire regiment weight under strong support.
const GNPRTB_STRONG_EMPIRE_TROOP_WEIGHT: u16 = 7680;
/// 7681 (`DAT_006bb400`) = 2: divisor for support changes that favour the
/// holder of a strongly supported system (`FUN_00559be0`).
const GNPRTB_STRONG_SUPPORT_DIVISOR: u16 = 7681;
/// 6144 (`DAT_006bb594`) = 10: divisor of an uprising mission's leadership.
const GNPRTB_UPRISING_MISSION_DIVISOR: u16 = 6144;
/// 6145 (`DAT_006bb52c`) = -2: support change while Incite Uprising is active.
const GNPRTB_INCITE_SUPPORT_DELTA: u16 = 6145;
/// 2565 (`DAT_006b9080`) = 1: minimum injury chance.
pub(crate) const GNPRTB_INJURY_MIN_CHANCE: u16 = 2565;
/// 2566 (`DAT_006b908c`) = 1: injury base.
pub(crate) const GNPRTB_INJURY_BASE: u16 = 2566;
/// 2567 (`DAT_006b90d8`) = 29: injury spread.
pub(crate) const GNPRTB_INJURY_SPREAD: u16 = 2567;
/// 7715 (`DAT_006bb3ec`) = 5: disaster erosion percent per remaining unit.
const GNPRTB_DISASTER_EROSION: u16 = 7715;
/// 7716 (`DAT_006bb43c`) = 10: disaster facility destruction percent.
const GNPRTB_DISASTER_FACILITY_CHANCE: u16 = 7716;
/// 7717 (`DAT_006bb404`) = 1: disaster timer minimum.
const GNPRTB_DISASTER_MIN: u16 = 7717;
/// 7718 (`DAT_006bb41c`) = 399: disaster timer spread.
const GNPRTB_DISASTER_SPREAD: u16 = 7718;
/// 6183 (`DAT_006bb56c`) = 1: diplomacy support gain base, own system.
const GNPRTB_DIPLOMACY_OWN_BASE: u16 = 6183;
/// 6184 (`DAT_006bb5ac`) = 19: diplomacy support gain spread, own system.
const GNPRTB_DIPLOMACY_OWN_SPREAD: u16 = 6184;
/// 6185 (`DAT_006bb51c`) = 1: diplomacy support gain base, neutral system.
const GNPRTB_DIPLOMACY_NEUTRAL_BASE: u16 = 6185;
/// 6186 (`DAT_006bb598`) = 9: diplomacy support gain spread, neutral system.
const GNPRTB_DIPLOMACY_NEUTRAL_SPREAD: u16 = 6186;
/// 6187 (`DAT_006bb5c8`) = 1: subdue support gain base, own system.
const GNPRTB_SUBDUE_OWN_BASE: u16 = 6187;
/// 6188 (`DAT_006bb540`) = 19: subdue support gain spread, own system.
const GNPRTB_SUBDUE_OWN_SPREAD: u16 = 6188;
/// 6189 (`DAT_006bb53c`) = 1: subdue support gain base, neutral system.
const GNPRTB_SUBDUE_NEUTRAL_BASE: u16 = 6189;
/// 6190 (`DAT_006bb580`) = 9: subdue support gain spread, neutral system.
const GNPRTB_SUBDUE_NEUTRAL_SPREAD: u16 = 6190;

/// TROOPSD record 6, Stormtrooper Regiment (TEXTSTRA 9344). `FUN_005091f0`
/// counts Empire regiments of this class against the incident score.
const STORMTROOPER_REGIMENT: u32 = 0x1000_0006;

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

pub use crate::world::FacilityRef;

/// One outcome of an uprising incident (`FUN_0050d150`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncidentLoss {
    /// Code 1: a facility of the holding side is destroyed (reason 8).
    Facility(FacilityRef),
    /// Code 2: a regiment of the holding side is destroyed (reason 8).
    Regiment(TroopKey),
    /// Code 3: a free character of the holding side is injured (slot
    /// `+0x2e4`, `FUN_004ef5f0`). We do not store injury (F-026).
    CharacterInjured {
        character: CharacterKey,
        injury: i32,
    },
    /// Codes 4 and 5: a prisoner the holding side keeps at the system goes
    /// free (slot `+0x214`, `FUN_004ef570`).
    PrisonerFreed(CharacterKey),
}

/// Events emitted by `UprisingSystem`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UprisingEvent {
    /// A held system fell short of its garrison and revolted (`FUN_0050b800`).
    /// The holder keeps control.
    UprisingBegan { system: SystemKey, tick: u64 },
    /// The revolt ended: the system is no longer held or populated
    /// (`FUN_0050b800`), or a Subdue Uprising success found it garrisoned
    /// (`FUN_0050c910`).
    UprisingEnded { system: SystemKey, tick: u64 },
    /// The uprising incident (event `0x38d`, `FUN_0050d030`) resolved.
    UprisingIncident {
        system: SystemKey,
        tick: u64,
        /// The side holding the system.
        side: Faction,
        /// The UPRIS1TB and UPRIS2TB outcome codes.
        codes: [u32; 2],
        losses: Vec<IncidentLoss>,
        /// Change to the holder's support, in points.
        support_delta: i32,
    },
    /// The disaster incident (event `0x38f`, `FUN_00511930`) struck.
    Disaster {
        system: SystemKey,
        tick: u64,
        total_energy: u8,
        raw_materials: u8,
        destroyed: Vec<FacilityRef>,
    },
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

/// Uprisings in progress and the disaster timer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UprisingState {
    /// Systems in revolt (system `+0x88` bit 2).
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    pub active_uprisings: HashMap<SystemKey, ActiveUprising>,
    /// Tick the disaster timer (event `0x38f`) next fires; drawn on the
    /// first advance.
    pub next_disaster_tick: Option<u64>,
}

/// One system in revolt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveUprising {
    /// Game tick when the uprising started.
    pub started_tick: u64,
    /// Tick the uprising incident (event `0x38d`) next fires; drawn on the
    /// next advance when absent.
    pub next_incident_tick: Option<u64>,
}

impl UprisingState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if the system is currently in revolt.
    #[must_use]
    pub fn is_uprising(&self, system: SystemKey) -> bool {
        self.active_uprisings.contains_key(&system)
    }
}

// ---------------------------------------------------------------------------
// Rolls
// ---------------------------------------------------------------------------

/// Caller-supplied `[0, 1)` rolls consumed in order.
struct Rolls<'a> {
    rolls: &'a [f64],
    next: usize,
}

impl<'a> Rolls<'a> {
    fn new(rolls: &'a [f64]) -> Self {
        Self { rolls, next: 0 }
    }

    /// `FUN_0053e290(n)`: uniform over `0..=n`, or `-(0..=-n)` for negative
    /// `n`. An exhausted slice yields `n`.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "the roll is in [0, 1], so the product fits the draw range"
    )]
    fn draw(&mut self, n: i32) -> i32 {
        let roll = self.rolls.get(self.next).copied().unwrap_or(1.0);
        self.next += 1;
        let span = n.unsigned_abs();
        let value = ((roll * f64::from(span + 1)).floor() as u32).min(span);
        if n < 0 {
            -value.cast_signed()
        } else {
            value.cast_signed()
        }
    }

    /// `FUN_0053e2f0(percent)`: a draw over `0..=99` below `percent`.
    fn chance(&mut self, percent: i32) -> bool {
        self.draw(99) < percent
    }
}

impl Draws for Rolls<'_> {
    fn draw(&mut self, n: i32) -> i32 {
        Rolls::draw(self, n)
    }

    fn chance(&mut self, percent: i32) -> bool {
        Rolls::chance(self, percent)
    }
}

/// The original's draws: `FUN_0053e290` and `FUN_0053e2f0`.
pub(crate) trait Draws {
    fn draw(&mut self, n: i32) -> i32;
    fn chance(&mut self, percent: i32) -> bool;

    /// `FUN_00559b80`: one of `count` items, drawing nothing when empty.
    fn pick(&mut self, count: usize) -> Option<usize> {
        let last = i32::try_from(count.checked_sub(1)?).ok()?;
        usize::try_from(self.draw(last)).ok()
    }
}

/// `FUN_0053e990`: with chance `max(min_chance, 100 - combat)` percent, an
/// injury of `rand(chance) + rand(spread) + base`. On a miss the original
/// applies injury 0, which changes nothing, so the roll gives `None`.
pub(crate) fn roll_injury(
    combat: i32,
    min_chance: i32,
    spread: i32,
    base: i32,
    draws: &mut impl Draws,
) -> Option<i32> {
    let chance = min_chance.max(100 - combat);
    draws
        .chance(chance)
        .then(|| draws.draw(chance) + draws.draw(spread) + base)
}

// ---------------------------------------------------------------------------
// UprisingSystem
// ---------------------------------------------------------------------------

/// Stateless uprising and disaster evaluator.
pub struct UprisingSystem;

/// The most draws one incident takes: two score draws, up to four per outcome
/// code (the code 3 pick, chance and two injury draws), and the next delay.
const INCIDENT_ROLLS: usize = 11;

impl UprisingSystem {
    /// The most rolls [`Self::advance`] can draw at `tick`, for the caller to
    /// reserve: one first-timer draw per system that may enter revolt, each
    /// due incident's draws, and the disaster's pick, erosion checks, facility
    /// checks and delays at the richest system.
    #[must_use]
    pub fn roll_budget(state: &UprisingState, world: &GameWorld, tick: u64) -> usize {
        let due = |next: Option<u64>| next.is_some_and(|due| due <= tick);
        let incidents = state
            .active_uprisings
            .values()
            .filter(|uprising| due(uprising.next_incident_tick))
            .count();
        let disaster = if state.next_disaster_tick.is_none() {
            1
        } else if due(state.next_disaster_tick) {
            let richest = world.systems.values().map(|sys| {
                usize::from(sys.raw_materials)
                    + usize::from(sys.total_energy)
                    + sys.defense_facilities.len()
                    + sys.manufacturing_facilities.len()
                    + sys.production_facilities.len()
            });
            2 + richest.max().unwrap_or(0)
        } else {
            0
        };
        world.systems.len() + incidents * INCIDENT_ROLLS + disaster
    }

    /// Start and end uprisings, then fire the uprising and disaster timers
    /// that are due by the last tick in `tick_events`. Each timer fires at
    /// most once per call; one that is still overdue fires on the next call.
    ///
    /// `economy` supplies each system's troop surplus from this tick.
    /// `rolls` are consumed in system order; [`Self::roll_budget`] sizes the
    /// slice so that no draw runs past it.
    pub fn advance(
        state: &mut UprisingState,
        world: &GameWorld,
        economy: &EconomyState,
        missions: &MissionState,
        tick_events: &[TickEvent],
        rolls: &[f64],
    ) -> Vec<UprisingEvent> {
        let Some(last) = tick_events.last() else {
            return Vec::new();
        };
        let tick = last.tick;
        let mut rolls = Rolls::new(rolls);
        let mut events = Vec::new();
        let param = |id| world.gnprtb.value(id, world.difficulty_index);

        for (system, sys) in &world.systems {
            let held = sys.control.faction().is_some_and(|f| f != Faction::Neutral);
            let populated = sys.is_populated && !sys.is_destroyed;
            if state.is_uprising(system) {
                if !held || !populated {
                    state.active_uprisings.remove(&system);
                    events.push(UprisingEvent::UprisingEnded { system, tick });
                }
                continue;
            }
            let short_of_troops = economy
                .per_system
                .get(&system)
                .is_some_and(|eco| eco.summary.troop_surplus < 0);
            if held && populated && short_of_troops {
                state.active_uprisings.insert(
                    system,
                    ActiveUprising {
                        started_tick: tick,
                        next_incident_tick: None,
                    },
                );
                events.push(UprisingEvent::UprisingBegan { system, tick });
            }
        }

        let empty = MstbTable::new(Vec::new());
        let upris1tb = world.mission_tables.get("UPRIS1TB").unwrap_or(&empty);
        let upris2tb = world.mission_tables.get("UPRIS2TB").unwrap_or(&empty);
        let incident_delay = |rolls: &mut Rolls<'_>| {
            timer_delay(
                param(GNPRTB_UPRISING_INCIDENT_MIN),
                param(GNPRTB_UPRISING_INCIDENT_SPREAD),
                rolls,
            )
        };
        for system in world.systems.keys() {
            let Some(uprising) = state.active_uprisings.get_mut(&system) else {
                continue;
            };
            let mut due = match uprising.next_incident_tick {
                Some(due) => due,
                None => tick + incident_delay(&mut rolls),
            };
            // One fire per advance: events apply after `advance` returns, so a
            // second fire now would read the world before the first one's
            // losses. An overdue timer fires again on the next advance.
            if due <= tick {
                if let Some(event) = resolve_incident(
                    world,
                    missions.missions(),
                    system,
                    tick,
                    upris1tb,
                    upris2tb,
                    &mut rolls,
                ) {
                    events.push(event);
                }
                due += incident_delay(&mut rolls);
            }
            uprising.next_incident_tick = Some(due);
        }

        let disaster_delay = |rolls: &mut Rolls<'_>| {
            timer_delay(
                param(GNPRTB_DISASTER_MIN),
                param(GNPRTB_DISASTER_SPREAD),
                rolls,
            )
        };
        let mut due = match state.next_disaster_tick {
            Some(due) => due,
            None => tick + disaster_delay(&mut rolls),
        };
        if due <= tick {
            if let Some(event) = resolve_disaster(world, tick, &mut rolls) {
                events.push(event);
            }
            due += disaster_delay(&mut rolls);
        }
        state.next_disaster_tick = Some(due);

        events
    }
}

/// `FUN_00586130`: `min + rand(0..=spread)`, at least one tick.
fn timer_delay(min: i32, spread: i32, rolls: &mut Rolls<'_>) -> u64 {
    let delay = min + rolls.draw(spread.max(0));
    u64::try_from(delay.max(1)).unwrap_or(1)
}

/// A timer delay of `min + rand(0..=spread)` days from one roll
/// (`FUN_00586130`), as a mission's timer `0x38b` draws it.
pub(crate) fn one_roll_timer_delay(min: i32, spread: i32, roll: f64) -> u64 {
    timer_delay(min, spread, &mut Rolls::new(std::slice::from_ref(&roll)))
}

/// The side holding `sys`, if it is the Alliance or the Empire.
pub(crate) fn holder(sys: &System) -> Option<Faction> {
    sys.control.faction().filter(|f| *f != Faction::Neutral)
}

/// `FUN_00507270`: a side's support at a system, in points.
#[expect(
    clippy::cast_possible_truncation,
    reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
)]
pub(crate) fn support_points(sys: &System, side: Faction) -> i32 {
    let fraction = if side == Faction::Alliance {
        sys.popularity_alliance
    } else {
        sys.popularity_empire
    };
    (fraction * 100.0).round() as i32
}

/// The regiments of `side` on the ground at `sys`.
fn regiments(world: &GameWorld, sys: &System, side: Faction) -> Vec<TroopKey> {
    sys.ground_units
        .iter()
        .copied()
        .filter(|key| {
            world
                .troops
                .get(*key)
                .is_some_and(|t| t.is_alliance == (side == Faction::Alliance))
        })
        .collect()
}

/// The facilities of `side` at `sys`, in family order (defense 0x22..0x25,
/// manufacturing 0x28..0x2a, production 0x2c..0x2d).
fn facilities(world: &GameWorld, sys: &System, side: Option<Faction>) -> Vec<FacilityRef> {
    let ours = |owner: Faction| side.is_none_or(|s| owner == s);
    let defense = sys.defense_facilities.iter().filter(|k| {
        world
            .defense_facilities
            .get(**k)
            .is_some_and(|f| ours(f.side))
    });
    let manufacturing = sys.manufacturing_facilities.iter().filter(|k| {
        world
            .manufacturing_facilities
            .get(**k)
            .is_some_and(|f| ours(f.side))
    });
    let production = sys.production_facilities.iter().filter(|k| {
        world
            .production_facilities
            .get(**k)
            .is_some_and(|f| ours(f.side))
    });
    defense
        .map(|k| FacilityRef::Defense(*k))
        .chain(manufacturing.map(|k| FacilityRef::Manufacturing(*k)))
        .chain(production.map(|k| FacilityRef::Production(*k)))
        .collect()
}

/// `FUN_00520cd0` over each uprising mission at `system`: the average
/// leadership (slot `+0x1f4`, enhanced leadership `+0x88`) of every team,
/// decoy, and captured member (`FUN_00525bb0`), divided by GNPRTB 6144,
/// summed per kind (`FUN_005484d0` writes `+0x54 → +0x74` for Incite
/// Uprising, family 0x56, and `+0x78` for Subdue Uprising, family 0x57).
fn uprising_mission_terms(
    world: &GameWorld,
    missions: &VecDeque<ActiveMission>,
    system: SystemKey,
) -> (i32, i32, bool) {
    let divisor = world
        .gnprtb
        .value(GNPRTB_UPRISING_MISSION_DIVISOR, world.difficulty_index)
        .max(1);
    let (mut incite, mut subdue, mut any_incite) = (0, 0, false);
    for mission in missions {
        if mission.target_system != system {
            continue;
        }
        // port: a member destroyed mid-mission drops out of the count; the
        // original removes a destroyed object from the mission's lists.
        let (total, count) = mission
            .members()
            .filter_map(|member| member_skill(world, member, Skill::Leadership))
            .fold((0_i32, 0_i32), |(total, count), value| {
                (total + value.cast_signed(), count + 1)
            });
        let leadership = if count == 0 { 0 } else { total / count };
        match mission.kind {
            MissionKind::InciteUprising => {
                any_incite = true;
                incite += leadership / divisor;
            }
            MissionKind::SubdueUprising => subdue -= leadership / divisor,
            _ => {}
        }
    }
    (incite, subdue, any_incite)
}

/// `FUN_00559be0` then `FUN_0053e0d0`: the change a support shift of `delta`
/// makes to `side`'s support at `sys`.
fn support_change(world: &GameWorld, sys: &System, side: Faction, delta: i32) -> i32 {
    let difficulty = world.difficulty_index;
    let strong = economy::is_strong_support(sys, &world.gnprtb, difficulty);
    let favours_holder =
        (side == Faction::Alliance && delta > 0) || (side == Faction::Empire && delta < 0);
    let delta = if strong && favours_holder {
        delta
            / world
                .gnprtb
                .value(GNPRTB_STRONG_SUPPORT_DIVISOR, difficulty)
                .max(1)
    } else {
        delta
    };
    let before = support_points(sys, side);
    (before + delta).clamp(0, 100) - before
}

/// `FUN_005091f0`: the Empire's Stormtrooper regiments at `sys`.
pub(crate) fn stormtroopers(world: &GameWorld, sys: &System) -> i32 {
    i32::try_from(
        sys.ground_units
            .iter()
            .filter(|k| {
                world.troops.get(**k).is_some_and(|t| {
                    !t.is_alliance && t.class_dat_id.raw() == STORMTROOPER_REGIMENT
                })
            })
            .count(),
    )
    .unwrap_or(i32::MAX)
}

/// `FUN_0050d030`: score the uprising at `system` and apply both outcome codes.
fn resolve_incident(
    world: &GameWorld,
    missions: &VecDeque<ActiveMission>,
    system: SystemKey,
    tick: u64,
    upris1tb: &MstbTable,
    upris2tb: &MstbTable,
    rolls: &mut impl Draws,
) -> Option<UprisingEvent> {
    let sys = world.systems.get(system)?;
    let side = holder(sys)?;
    let difficulty = world.difficulty_index;
    let param = |id| world.gnprtb.value(id, difficulty);
    let strong = economy::is_strong_support(sys, &world.gnprtb, difficulty);

    // FUN_00559ce0
    let base = param(GNPRTB_INCIDENT_DRAW_BASE);
    let spread = param(GNPRTB_INCIDENT_DRAW_SPREAD);
    let first = rolls.draw(spread) + base;
    let second = rolls.draw(spread) + base;
    let weight = if strong && side == Faction::Empire {
        param(GNPRTB_STRONG_EMPIRE_TROOP_WEIGHT)
    } else {
        1
    };
    let troops = i32::try_from(regiments(world, sys, side).len()).unwrap_or(i32::MAX);
    let stormtroopers = stormtroopers(world, sys);
    let (incite, subdue, any_incite) = uprising_mission_terms(world, missions, system);
    let score = first
        + second
        + economy::uprising_threshold(support_points(sys, side), &world.gnprtb, difficulty)
        - weight * troops
        + incite
        + subdue
        - stormtroopers;
    let codes = [upris1tb.step_lookup(score), upris2tb.step_lookup(score)];

    // FUN_0050d150, once per code, against the world as the earlier code left it.
    let mut losses = Vec::new();
    for code in codes {
        apply_code(world, sys, system, side, code, &mut losses, rolls);
    }

    // FUN_0050c9f0 with FUN_00509b30 (+0x54 → +0x7c).
    let delta = if any_incite {
        param(GNPRTB_INCITE_SUPPORT_DELTA)
    } else {
        0
    };
    let support_delta = support_change(world, sys, side, delta);

    Some(UprisingEvent::UprisingIncident {
        system,
        tick,
        side,
        codes,
        losses,
        support_delta,
    })
}

/// `FUN_0050d150` for one outcome code.
fn apply_code(
    world: &GameWorld,
    sys: &System,
    system: SystemKey,
    side: Faction,
    code: u32,
    losses: &mut Vec<IncidentLoss>,
    rolls: &mut impl Draws,
) {
    let param = |id| world.gnprtb.value(id, world.difficulty_index);
    match code {
        1 => {
            let candidates: Vec<FacilityRef> = facilities(world, sys, Some(side))
                .into_iter()
                .filter(|f| !losses.contains(&IncidentLoss::Facility(*f)))
                .collect();
            if let Some(i) = rolls.pick(candidates.len()) {
                losses.push(IncidentLoss::Facility(candidates[i]));
            }
        }
        2 => {
            let candidates: Vec<TroopKey> = regiments(world, sys, side)
                .into_iter()
                .filter(|t| !losses.contains(&IncidentLoss::Regiment(*t)))
                .collect();
            if let Some(i) = rolls.pick(candidates.len()) {
                losses.push(IncidentLoss::Regiment(candidates[i]));
            }
        }
        3 => {
            let free: Vec<(CharacterKey, i32)> = world
                .characters
                .iter()
                .filter(|(_, c)| {
                    usable_at(c, system)
                        && !c.is_captive
                        && sides_match(c.is_alliance, c.is_empire, side)
                })
                .map(|(k, c)| (k, (c.combat.base + c.combat.variance / 2).cast_signed()))
                .collect();
            if let Some(i) = rolls.pick(free.len()) {
                let (character, combat) = free[i];
                if let Some(injury) = roll_injury(
                    combat,
                    param(GNPRTB_INJURY_MIN_CHANCE),
                    param(GNPRTB_INJURY_SPREAD),
                    param(GNPRTB_INJURY_BASE),
                    rolls,
                ) {
                    losses.push(IncidentLoss::CharacterInjured { character, injury });
                }
            }
        }
        4 | 5 => {
            let prisoners: Vec<CharacterKey> = world
                .characters
                .iter()
                .filter(|(k, c)| {
                    usable_at(c, system)
                        && c.is_captive
                        && c.captured_by == Some(side)
                        && !losses.contains(&IncidentLoss::PrisonerFreed(*k))
                })
                .map(|(k, _)| k)
                .collect();
            if code == 4 {
                if let Some(i) = rolls.pick(prisoners.len()) {
                    losses.push(IncidentLoss::PrisonerFreed(prisoners[i]));
                }
            } else {
                losses.extend(prisoners.into_iter().map(IncidentLoss::PrisonerFreed));
            }
        }
        _ => {}
    }
}

/// `FUN_004f2640(system, 1, side)`: a character the system holds directly and
/// that is usable. The walk covers only the system's own child list
/// (`FUN_00539f70` first child at `+0x28`, `FUN_005c7530` next sibling at
/// `+0x10`), so a fleet's crew and a mission's agents (`FUN_00525bb0` under the
/// mission) are excluded. Mode 1 keeps `+0x50` bit 0, Usable, which
/// `0x004f7b80` sets only for an object that exists (created, not destroyed)
/// and is complete and not en route.
fn usable_at(c: &Character, system: SystemKey) -> bool {
    c.current_system == Some(system)
        && c.current_fleet.is_none()
        && !c.on_mission
        && !c.on_mandatory_mission
        && !c.is_killed
}

fn sides_match(is_alliance: bool, is_empire: bool, side: Faction) -> bool {
    match side {
        Faction::Alliance => is_alliance,
        Faction::Empire => is_empire,
        Faction::Neutral => false,
    }
}

/// `FUN_00556b50`, `FUN_0050cdc0` and `FUN_00511930`.
fn resolve_disaster(world: &GameWorld, tick: u64, rolls: &mut Rolls<'_>) -> Option<UprisingEvent> {
    // +0x50 bit 6 is GameObjExistingNotif (FUN_004f76b0 -> view slot +0x190).
    let existing: Vec<SystemKey> = world
        .systems
        .iter()
        .filter(|(_, s)| !s.is_destroyed)
        .map(|(k, _)| k)
        .collect();
    let system = existing[rolls.pick(existing.len())?];
    let sys = &world.systems[system];
    if sys.total_energy == 0 && sys.raw_materials == 0 {
        return None;
    }
    let param = |id| world.gnprtb.value(id, world.difficulty_index);

    // FUN_00559e10(&raw, &energy)
    let mut raw = i32::from(sys.raw_materials);
    let mut energy = i32::from(sys.total_energy);
    let erosion = param(GNPRTB_DISASTER_EROSION);
    let (mut lost_raw, mut lost_energy) = (0, 0);
    for i in 0..raw.max(energy) {
        if i < raw && rolls.chance((energy - lost_raw - lost_energy + raw) * erosion) {
            lost_raw += 1;
        }
        if i < energy && rolls.chance((raw - lost_raw - lost_energy + energy) * erosion) {
            lost_energy += 1;
        }
    }
    if lost_raw == 0 && lost_energy == 0 {
        if raw != 0 {
            lost_raw = 1;
        } else if energy != 0 {
            lost_energy = 1;
        }
    }
    raw -= lost_raw;
    energy -= lost_energy;
    raw = raw.min(energy);

    // Every facility not en route (+0x50 bit 4, GameObjEnrouteNotif), both
    // sides: manufacturing and production (0x28..0x2f) first, then defense
    // (0x22..0x27), each lost with GNPRTB 7716 percent (reason 0xb). The port
    // has no en-route facility: a facility joins a system list on completion.
    let chance = param(GNPRTB_DISASTER_FACILITY_CHANCE);
    let all = facilities(world, sys, None);
    let (defense, others): (Vec<FacilityRef>, Vec<FacilityRef>) = all
        .into_iter()
        .partition(|f| matches!(f, FacilityRef::Defense(_)));
    let destroyed = others
        .into_iter()
        .chain(defense)
        .filter(|_| rolls.chance(chance))
        .collect();

    Some(UprisingEvent::Disaster {
        system,
        tick,
        total_energy: u8::try_from(energy.max(0)).unwrap_or(u8::MAX),
        raw_materials: u8::try_from(raw.max(0)).unwrap_or(u8::MAX),
        destroyed,
    })
}

// ---------------------------------------------------------------------------
// Subdue Uprising (FUN_00569c20)
// ---------------------------------------------------------------------------

/// Which mission's support gain [`mission_support_gain`] rolls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SupportGain {
    /// `FUN_0055cac0`, GNPRTB 6183..6186.
    Diplomacy,
    /// `FUN_0055cb10`, GNPRTB 6187..6190.
    Subdue,
}

/// `FUN_0055cac0` and `FUN_0055cb10`: the support a mission success by
/// `side` wins at `sys`. On its own side's system: base + rand(0..=spread).
/// On a neutral system (side bits 3, "Neutral" in `FUN_004f8c60`): the
/// neutral pair. At the opponent's system: nothing.
pub(crate) fn mission_support_gain(
    world: &GameWorld,
    sys: &System,
    side: Faction,
    gain: SupportGain,
    draws: &mut impl Draws,
) -> i32 {
    let param = |id| world.gnprtb.value(id, world.difficulty_index);
    let (own, neutral) = match gain {
        SupportGain::Diplomacy => (
            (GNPRTB_DIPLOMACY_OWN_BASE, GNPRTB_DIPLOMACY_OWN_SPREAD),
            (
                GNPRTB_DIPLOMACY_NEUTRAL_BASE,
                GNPRTB_DIPLOMACY_NEUTRAL_SPREAD,
            ),
        ),
        SupportGain::Subdue => (
            (GNPRTB_SUBDUE_OWN_BASE, GNPRTB_SUBDUE_OWN_SPREAD),
            (GNPRTB_SUBDUE_NEUTRAL_BASE, GNPRTB_SUBDUE_NEUTRAL_SPREAD),
        ),
    };
    let (base, spread) = match holder(sys) {
        Some(holder) if holder == side => own,
        Some(_) => return 0,
        None => neutral,
    };
    // FUN_0055cac0 draws the spread before adding the base.
    draws.draw(param(spread)) + param(base)
}

/// `sys` after `FUN_0050c9f0` shifts `side`'s support by `delta` points.
pub(crate) fn with_support_change(
    world: &GameWorld,
    sys: &System,
    side: Faction,
    delta: i32,
) -> System {
    let mut after = sys.clone();
    shift_system(&mut after, side, support_change(world, sys, side, delta));
    after
}

/// `FUN_0050c910`'s test: the regiments at `sys` (`FUN_00504c40`, every
/// side) cover its holder's garrison requirement without the uprising
/// doubling (`FUN_00559fb0`).
pub(crate) fn garrison_covers(world: &GameWorld, sys: &System) -> bool {
    let Some(side) = holder(sys) else {
        return false;
    };
    let difficulty = world.difficulty_index;
    let requirement = economy::garrison_requirement(
        side,
        support_points(sys, side),
        economy::is_strong_support(sys, &world.gnprtb, difficulty),
        false,
        &world.gnprtb,
        difficulty,
    );
    i32::try_from(sys.ground_units.len()).unwrap_or(i32::MAX) >= requirement
}

/// `FUN_0050d030` run by a successful Incite Uprising member
/// (`FUN_00571a60`): the uprising incident at `system`, scored with every
/// mission in `missions`.
pub(crate) fn incite_incident(
    world: &GameWorld,
    missions: &VecDeque<ActiveMission>,
    system: SystemKey,
    tick: u64,
    draws: &mut impl Draws,
) -> Option<UprisingEvent> {
    let empty = MstbTable::new(Vec::new());
    let upris1tb = world.mission_tables.get("UPRIS1TB").unwrap_or(&empty);
    let upris2tb = world.mission_tables.get("UPRIS2TB").unwrap_or(&empty);
    resolve_incident(world, missions, system, tick, upris1tb, upris2tb, draws)
}

/// `FUN_0050c910`: after a Subdue Uprising success, end the revolt at `system`
/// if its regiments (`FUN_00504c40`, every side) cover the garrison
/// requirement without the uprising doubling (`FUN_00559fb0`).
pub fn end_if_garrisoned(
    state: &mut UprisingState,
    world: &GameWorld,
    system: SystemKey,
    tick: u64,
) -> Option<UprisingEvent> {
    if !state.is_uprising(system) {
        return None;
    }
    if !garrison_covers(world, world.systems.get(system)?) {
        return None;
    }
    state.active_uprisings.remove(&system);
    Some(UprisingEvent::UprisingEnded { system, tick })
}

// ---------------------------------------------------------------------------
// Applying events to the world
// ---------------------------------------------------------------------------

/// `FUN_0050c9f0`: shift `side`'s support at `system` by `delta` points
/// (halved for a strongly supported holder, `FUN_00559be0`), clamped to
/// `0..=100`. The other side moves by the opposite amount.
pub fn apply_support_change(world: &mut GameWorld, system: SystemKey, side: Faction, delta: i32) {
    let Some(sys) = world.systems.get(system) else {
        return;
    };
    let change = support_change(world, sys, side, delta);
    shift_support(world, system, side, change);
}

fn shift_support(world: &mut GameWorld, system: SystemKey, side: Faction, points: i32) {
    if let Some(sys) = world.systems.get_mut(system) {
        shift_system(sys, side, points);
    }
}

/// Shift `side`'s support at `sys` by `points`, the other side's by the
/// opposite. port: the port keeps support as a `0..=1` fraction; the
/// original keeps points at `+0x58` (`FUN_00507270`).
#[expect(
    clippy::cast_precision_loss,
    reason = "support changes are at most 100 points"
)]
fn shift_system(sys: &mut System, side: Faction, points: i32) {
    let fraction = points as f32 / 100.0;
    let (ours, theirs) = match side {
        Faction::Alliance => (&mut sys.popularity_alliance, &mut sys.popularity_empire),
        Faction::Empire => (&mut sys.popularity_empire, &mut sys.popularity_alliance),
        Faction::Neutral => return,
    };
    *ours = (*ours + fraction).clamp(0.0, 1.0);
    *theirs = (*theirs - fraction).clamp(0.0, 1.0);
}

/// Apply an event's losses, resource changes and support change to `world`.
/// The native app and the headless integrator share this.
pub fn apply_uprising_event(world: &mut GameWorld, event: &UprisingEvent) {
    match event {
        UprisingEvent::UprisingIncident {
            system,
            side,
            losses,
            support_delta,
            ..
        } => {
            for loss in losses {
                match loss {
                    IncidentLoss::Facility(facility) => remove_facility(world, *system, *facility),
                    IncidentLoss::Regiment(troop) => {
                        if let Some(sys) = world.systems.get_mut(*system) {
                            sys.ground_units.retain(|t| t != troop);
                        }
                        world.troops.remove(*troop);
                    }
                    IncidentLoss::CharacterInjured { .. } => {}
                    IncidentLoss::PrisonerFreed(character) => {
                        if let Some(c) = world.characters.get_mut(*character) {
                            c.is_captive = false;
                            c.captured_by = None;
                            c.capture_tick = None;
                        }
                    }
                }
            }
            shift_support(world, *system, *side, *support_delta);
        }
        UprisingEvent::Disaster {
            system,
            total_energy,
            raw_materials,
            destroyed,
            ..
        } => {
            if let Some(sys) = world.systems.get_mut(*system) {
                sys.total_energy = *total_energy;
                sys.raw_materials = *raw_materials;
            }
            for facility in destroyed {
                remove_facility(world, *system, *facility);
            }
        }
        UprisingEvent::UprisingBegan { .. } | UprisingEvent::UprisingEnded { .. } => {}
    }
}

fn remove_facility(world: &mut GameWorld, system: SystemKey, facility: FacilityRef) {
    world.remove_facility(system, facility);
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dat::{ExplorationStatus, SectorGroup};
    use crate::ids::{DatId, ProductionFacilityKey};
    use crate::missions::MissionFaction;
    use crate::missions::MissionRequest;
    use crate::world::{
        Character, ControlKind, DefenseFacilityInstance, GnprtbEntry, GnprtbParams,
        ManufacturingFacilityInstance, MstbEntry, ProductionFacilityInstance, Sector, SkillPair,
        TroopUnit,
    };

    fn gnprtb_entry(parameter_id: u16, value: i32) -> GnprtbEntry {
        GnprtbEntry {
            parameter_id: u32::from(parameter_id),
            development: value,
            alliance_sp_easy: value,
            alliance_sp_medium: value,
            alliance_sp_hard: value,
            empire_sp_easy: value,
            empire_sp_medium: value,
            empire_sp_hard: value,
            multiplayer: value,
        }
    }

    fn table(rows: &[(i32, u32)]) -> MstbTable {
        MstbTable::new(
            rows.iter()
                .map(|&(threshold, value)| MstbEntry { threshold, value })
                .collect(),
        )
    }

    /// One system held by `side` at `support` percent, with the shipped
    /// GNPRTB values and the shipped UPRIS1TB/UPRIS2TB rows.
    fn world_with(side: Faction, support: f32) -> (GameWorld, SystemKey) {
        let mut world = GameWorld::default();
        world.gnprtb = GnprtbParams::new(
            [
                (GNPRTB_UPRISING_INCIDENT_MIN, 30),
                (GNPRTB_UPRISING_INCIDENT_SPREAD, 70),
                (GNPRTB_INCIDENT_DRAW_BASE, 1),
                (GNPRTB_INCIDENT_DRAW_SPREAD, 9),
                (GNPRTB_STRONG_EMPIRE_TROOP_WEIGHT, 2),
                (GNPRTB_STRONG_SUPPORT_DIVISOR, 2),
                (GNPRTB_UPRISING_MISSION_DIVISOR, 10),
                (GNPRTB_INCITE_SUPPORT_DELTA, -2),
                (GNPRTB_INJURY_MIN_CHANCE, 1),
                (GNPRTB_INJURY_BASE, 1),
                (GNPRTB_INJURY_SPREAD, 29),
                (GNPRTB_DISASTER_EROSION, 5),
                (GNPRTB_DISASTER_FACILITY_CHANCE, 10),
                (GNPRTB_DISASTER_MIN, 1),
                (GNPRTB_DISASTER_SPREAD, 399),
                (GNPRTB_SUBDUE_OWN_BASE, 1),
                (GNPRTB_SUBDUE_OWN_SPREAD, 19),
                (GNPRTB_SUBDUE_NEUTRAL_BASE, 1),
                (GNPRTB_SUBDUE_NEUTRAL_SPREAD, 9),
                (7682, 2),
                (7732, 40),
                (7761, 60),
                (7762, -10),
            ]
            .into_iter()
            .map(|(id, value)| gnprtb_entry(id, value))
            .collect(),
        );
        world
            .mission_tables
            .insert("UPRIS1TB".into(), table(&[(1, 0), (6, 1), (10, 2)]));
        world.mission_tables.insert(
            "UPRIS2TB".into(),
            table(&[(1, 0), (9, 3), (11, 4), (12, 5)]),
        );
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(0x9200_0000),
            name: "Sector".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let (alliance, empire) = if side == Faction::Alliance {
            (support, 1.0 - support)
        } else {
            (1.0 - support, support)
        };
        let system = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0001),
            name: "Naboo".into(),
            sector,
            x: 0,
            y: 0,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: alliance,
            popularity_empire: empire,
            is_populated: true,
            total_energy: 4,
            raw_materials: 4,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(side),
        });
        (world, system)
    }

    fn add_regiment(
        world: &mut GameWorld,
        system: SystemKey,
        class: u32,
        alliance: bool,
    ) -> TroopKey {
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(class),
            is_alliance: alliance,
            regiment_strength: 1,
        });
        world.systems[system].ground_units.push(troop);
        troop
    }

    fn add_mine(world: &mut GameWorld, system: SystemKey, alliance: bool) -> ProductionFacilityKey {
        let mine = world
            .production_facilities
            .insert(ProductionFacilityInstance {
                class_dat_id: DatId::new(0x2c00_0001),
                side: crate::dat::Faction::of_alliance(alliance),
                is_mine: true,
            });
        world.systems[system].production_facilities.push(mine);
        mine
    }

    fn short_of_troops(system: SystemKey, surplus: i32) -> EconomyState {
        let mut economy = EconomyState::default();
        economy
            .per_system
            .entry(system)
            .or_default()
            .summary
            .troop_surplus = surplus;
        economy
    }

    fn in_revolt(system: SystemKey, next_incident_tick: u64) -> UprisingState {
        let mut state = UprisingState {
            next_disaster_tick: Some(u64::MAX),
            ..UprisingState::default()
        };
        state.active_uprisings.insert(
            system,
            ActiveUprising {
                started_tick: 0,
                next_incident_tick: Some(next_incident_tick),
            },
        );
        state
    }

    fn at(tick: u64) -> [TickEvent; 1] {
        [TickEvent { tick }]
    }

    fn incident(events: &[UprisingEvent]) -> (&[u32; 2], &[IncidentLoss], i32) {
        events
            .iter()
            .find_map(|e| match e {
                UprisingEvent::UprisingIncident {
                    codes,
                    losses,
                    support_delta,
                    ..
                } => Some((codes, losses.as_slice(), *support_delta)),
                _ => None,
            })
            .expect("an uprising incident")
    }

    #[test]
    fn a_held_system_short_of_troops_revolts_without_changing_hands() {
        // FUN_0050b800 sets +0x88 bit 2 when the troop surplus (+0x7c) is
        // negative; FUN_0050a130 records the side in revolt, not a new owner.
        let (world, system) = world_with(Faction::Alliance, 0.3);
        let mut state = UprisingState {
            next_disaster_tick: Some(u64::MAX),
            ..UprisingState::default()
        };
        let quiet = UprisingSystem::advance(
            &mut state,
            &world,
            &short_of_troops(system, 0),
            &MissionState::new(),
            &at(1),
            &[],
        );
        assert!(quiet.is_empty());
        assert!(!state.is_uprising(system));

        let events = UprisingSystem::advance(
            &mut state,
            &world,
            &short_of_troops(system, -1),
            &MissionState::new(),
            &at(2),
            &[0.0],
        );
        assert_eq!(
            events,
            vec![UprisingEvent::UprisingBegan { system, tick: 2 }]
        );
        assert!(state.is_uprising(system));
        let mut world = world;
        for event in &events {
            apply_uprising_event(&mut world, event);
        }
        assert_eq!(
            world.systems[system].control,
            ControlKind::Controlled(Faction::Alliance)
        );
    }

    #[test]
    fn a_revolt_outlasts_a_restored_garrison_but_ends_when_the_system_is_lost() {
        // FUN_0050b800 keeps bit 2 once set while the system is populated and
        // held by side 1 or 2, and clears it otherwise.
        let (mut world, system) = world_with(Faction::Empire, 0.3);
        let mut state = in_revolt(system, u64::MAX);
        let kept = UprisingSystem::advance(
            &mut state,
            &world,
            &short_of_troops(system, 5),
            &MissionState::new(),
            &at(1),
            &[],
        );
        assert!(kept.is_empty());
        assert!(state.is_uprising(system));

        world.systems[system].control = ControlKind::Uncontrolled;
        let events = UprisingSystem::advance(
            &mut state,
            &world,
            &short_of_troops(system, 5),
            &MissionState::new(),
            &at(2),
            &[],
        );
        assert_eq!(
            events,
            vec![UprisingEvent::UprisingEnded { system, tick: 2 }]
        );
        assert!(!state.is_uprising(system));
    }

    #[test]
    fn the_incident_timer_waits_thirty_to_one_hundred_ticks_and_repeats() {
        // FUN_00586130: GNPRTB 7701 (30) + rand(0..=7702 (70)); FUN_005862a0
        // reschedules from the fire time.
        let (world, system) = world_with(Faction::Alliance, 0.9);
        let mut state = UprisingState {
            next_disaster_tick: Some(u64::MAX),
            ..UprisingState::default()
        };
        UprisingSystem::advance(
            &mut state,
            &world,
            &short_of_troops(system, -1),
            &MissionState::new(),
            &at(10),
            &[0.0, 0.0],
        );
        assert_eq!(state.active_uprisings[&system].next_incident_tick, Some(40));

        let economy = short_of_troops(system, -1);
        let early = UprisingSystem::advance(
            &mut state,
            &world,
            &economy,
            &MissionState::new(),
            &at(39),
            &[],
        );
        assert!(early.is_empty());
        // Two score draws, then the next delay draws the maximum 70.
        let fired = UprisingSystem::advance(
            &mut state,
            &world,
            &economy,
            &MissionState::new(),
            &at(40),
            &[0.0, 0.0, 0.999],
        );
        assert_eq!(fired.len(), 1);
        assert_eq!(
            state.active_uprisings[&system].next_incident_tick,
            Some(140)
        );
    }

    #[test]
    fn a_low_score_destroys_one_facility_of_the_holder() {
        // FUN_00559ce0 at support 0: draws 1 + 1, threshold ceil(60 / 10) = 6,
        // no troops: score 8, UPRIS1TB code 1, UPRIS2TB code 0. FUN_0050d150
        // code 1 destroys a random facility of the holding side (0x20..0x2f).
        let (mut world, system) = world_with(Faction::Alliance, 0.0);
        let _enemy = add_mine(&mut world, system, false);
        let ours = add_mine(&mut world, system, true);
        let mut state = in_revolt(system, 5);
        let events = UprisingSystem::advance(
            &mut state,
            &world,
            &short_of_troops(system, -1),
            &MissionState::new(),
            &at(5),
            &[0.0, 0.0, 0.0],
        );
        let (codes, losses, support_delta) = incident(&events);
        assert_eq!(codes, &[1, 0]);
        assert_eq!(
            losses,
            &[IncidentLoss::Facility(FacilityRef::Production(ours))]
        );
        assert_eq!(support_delta, 0);

        for event in &events {
            apply_uprising_event(&mut world, event);
        }
        assert!(!world.production_facilities.contains_key(ours));
        assert_eq!(world.systems[system].production_facilities.len(), 1);
    }

    #[test]
    fn stormtroopers_and_regiments_lower_the_incident_score() {
        // FUN_00559ce0 subtracts the holder's regiments (FUN_00509020) and
        // Empire Stormtrooper regiments (FUN_005091f0, class 0x10000006):
        // 2 + 6 - 1 - 1 = 6 is still code 1, and a second regiment makes it
        // 2 + 6 - 2 - 1 = 5, below every UPRIS1TB row that acts.
        let (mut world, system) = world_with(Faction::Empire, 0.0);
        add_regiment(&mut world, system, STORMTROOPER_REGIMENT, false);
        let mut state = in_revolt(system, 5);
        let run = |world: &GameWorld, state: &mut UprisingState| {
            UprisingSystem::advance(
                state,
                world,
                &short_of_troops(system, -1),
                &MissionState::new(),
                &at(5),
                &[0.0, 0.0, 0.0, 0.0],
            )
        };
        let events = run(&world, &mut state);
        assert_eq!(incident(&events).0, &[1, 0]);

        add_regiment(&mut world, system, 0x1000_0008, false);
        let mut state = in_revolt(system, 5);
        let events = run(&world, &mut state);
        assert_eq!(incident(&events).0, &[0, 0]);
    }

    #[test]
    fn a_high_score_destroys_a_regiment_and_frees_every_prisoner_the_holder_keeps() {
        // Maximum draws 10 + 10 plus threshold 6 minus one regiment: 25 gives
        // UPRIS1TB code 2 (a regiment, reason 8) and UPRIS2TB code 5 (every
        // captive at the system gets slot +0x214, FUN_004ef570 -> status 1).
        let (mut world, system) = world_with(Faction::Alliance, 0.0);
        let regiment = add_regiment(&mut world, system, 0x1000_0002, true);
        let prisoner = |world: &mut GameWorld, captor| {
            world.characters.insert(Character {
                is_empire: true,
                is_captive: true,
                captured_by: Some(captor),
                current_system: Some(system),
                ..Character::default()
            })
        };
        let held = prisoner(&mut world, Faction::Alliance);
        let also_held = prisoner(&mut world, Faction::Alliance);
        let other = prisoner(&mut world, Faction::Empire);
        // A captive aboard a fleet belongs to the fleet, not the system.
        let aboard = prisoner(&mut world, Faction::Alliance);
        world.characters[aboard].current_fleet = Some(crate::ids::FleetKey::default());
        let mut state = in_revolt(system, 5);
        let events = UprisingSystem::advance(
            &mut state,
            &world,
            &short_of_troops(system, -1),
            &MissionState::new(),
            &at(5),
            &[0.999, 0.999, 0.0, 0.0],
        );
        let (codes, losses, _) = incident(&events);
        assert_eq!(codes, &[2, 5]);
        assert_eq!(
            losses,
            &[
                IncidentLoss::Regiment(regiment),
                IncidentLoss::PrisonerFreed(held),
                IncidentLoss::PrisonerFreed(also_held)
            ]
        );

        for event in &events {
            apply_uprising_event(&mut world, event);
        }
        assert!(!world.troops.contains_key(regiment));
        assert!(world.systems[system].ground_units.is_empty());
        assert!(!world.characters[held].is_captive);
        assert!(!world.characters[also_held].is_captive);
        assert!(world.characters[other].is_captive);
        assert!(world.characters[aboard].is_captive);
    }

    #[test]
    fn an_injury_uses_the_characters_combat_and_gnprtb_2565_to_2567() {
        // Code 3 (UPRIS2TB score 9..10): FUN_004ef5f0 -> FUN_0053e990 with
        // combat 60: chance max(1, 40) = 40, injury rand(0..=40) + rand(0..=29)
        // + 1. Score 9: draws 1 + 2, threshold 6.
        let (mut world, system) = world_with(Faction::Alliance, 0.0);
        let agent = world.characters.insert(Character {
            is_alliance: true,
            current_system: Some(system),
            combat: crate::world::SkillPair {
                base: 60,
                variance: 0,
            },
            ..Character::default()
        });
        let mut state = in_revolt(system, 5);
        // Score draws 0 and 0.15 (1 + 2); code 1 has no facility to pick;
        // code 3 picks the agent, hits the 40 percent chance, and draws 40
        // then 29.
        let events = UprisingSystem::advance(
            &mut state,
            &world,
            &short_of_troops(system, -1),
            &MissionState::new(),
            &at(5),
            &[0.0, 0.15, 0.0, 0.0, 0.999, 0.999],
        );
        let (codes, losses, _) = incident(&events);
        assert_eq!(codes, &[1, 3]);
        assert_eq!(
            losses,
            &[IncidentLoss::CharacterInjured {
                character: agent,
                injury: 70
            }]
        );
    }

    #[test]
    fn an_active_incite_mission_cuts_the_holders_support_by_two_points() {
        // FUN_005484d0 stores GNPRTB 6145 (-2) in +0x54 -> +0x7c while an
        // Incite Uprising (family 0x56) is active there, and adds the agent's
        // leadership / GNPRTB 6144 to the score; FUN_0050d030 then applies the
        // support change through FUN_0050c9f0.
        let (mut world, system) = world_with(Faction::Alliance, 0.3);
        let agent = world.characters.insert(Character {
            is_empire: true,
            leadership: crate::world::SkillPair {
                base: 40,
                variance: 0,
            },
            ..Character::default()
        });
        let mut missions = MissionState::new();
        missions.dispatch(MissionRequest::single(
            MissionKind::InciteUprising,
            MissionFaction::Empire,
            agent,
            system,
            None,
            0,
        ));
        let mut state = in_revolt(system, 5);
        let events = UprisingSystem::advance(
            &mut state,
            &world,
            &short_of_troops(system, -1),
            &missions,
            &at(5),
            &[0.0, 0.0, 0.0, 0.0],
        );
        // Score 1 + 1 + ceil(30 / 10) + 40 / 10 = 9: codes 1 and 3.
        let (codes, _, support_delta) = incident(&events);
        assert_eq!(codes, &[1, 3]);
        assert_eq!(support_delta, -2);

        for event in &events {
            apply_uprising_event(&mut world, event);
        }
        assert!((world.systems[system].popularity_alliance - 0.28).abs() < 1e-6);
        assert!((world.systems[system].popularity_empire - 0.72).abs() < 1e-6);
    }

    #[test]
    fn a_subdue_success_wins_one_to_twenty_at_home_one_to_ten_when_neutral_else_nothing() {
        // FUN_0055cb10: same side -> GNPRTB 6187 + rand(0..=6188); neutral
        // (side 3, FUN_004f8c60) -> 6189 + rand(0..=6190); otherwise 0.
        let (mut world, system) = world_with(Faction::Alliance, 0.3);
        let gain = |world: &GameWorld, side, roll: f64| {
            let sys = &world.systems[system];
            mission_support_gain(
                world,
                sys,
                side,
                SupportGain::Subdue,
                &mut Rolls::new(&[roll]),
            )
        };
        assert_eq!(gain(&world, Faction::Alliance, 0.0), 1);
        assert_eq!(gain(&world, Faction::Alliance, 0.999), 20);
        assert_eq!(gain(&world, Faction::Empire, 0.5), 0);
        world.systems[system].control = ControlKind::Uncontrolled;
        assert_eq!(gain(&world, Faction::Empire, 0.999), 10);
    }

    #[test]
    fn a_diplomacy_success_wins_its_own_pair_at_home_and_when_neutral() {
        // FUN_0055cac0: same side -> GNPRTB 6183 + rand(0..=6184); neutral
        // (side 3, FUN_004f8c60) -> 6185 + rand(0..=6186); otherwise 0.
        let (mut world, system) = world_with(Faction::Alliance, 0.3);
        // Distinct values tell the Diplomacy ids from Subdue's.
        world.gnprtb = GnprtbParams::new(
            [
                (GNPRTB_DIPLOMACY_OWN_BASE, 2),
                (GNPRTB_DIPLOMACY_OWN_SPREAD, 5),
                (GNPRTB_DIPLOMACY_NEUTRAL_BASE, 3),
                (GNPRTB_DIPLOMACY_NEUTRAL_SPREAD, 4),
            ]
            .into_iter()
            .map(|(id, value)| gnprtb_entry(id, value))
            .collect(),
        );
        let gain = |world: &GameWorld, side, roll: f64| {
            let sys = &world.systems[system];
            mission_support_gain(
                world,
                sys,
                side,
                SupportGain::Diplomacy,
                &mut Rolls::new(&[roll]),
            )
        };
        assert_eq!(gain(&world, Faction::Alliance, 0.0), 2);
        assert_eq!(gain(&world, Faction::Alliance, 0.999), 7);
        assert_eq!(gain(&world, Faction::Empire, 0.999), 0);
        world.systems[system].control = ControlKind::Uncontrolled;
        assert_eq!(gain(&world, Faction::Empire, 0.999), 7);
        assert_eq!(gain(&world, Faction::Empire, 0.0), 3);
    }

    #[test]
    fn a_subdue_success_ends_the_revolt_once_regiments_cover_the_undoubled_garrison() {
        // FUN_0050c910 -> FUN_00559fb0(side, support, strong, 1, 0, regiments):
        // p5 = 0, so no uprising doubling. Support 30: ceil(30 / 10) = 3.
        let (mut world, system) = world_with(Faction::Alliance, 0.3);
        let mut state = in_revolt(system, u64::MAX);
        for _ in 0..2 {
            add_regiment(&mut world, system, 0x1000_0002, true);
        }
        assert_eq!(end_if_garrisoned(&mut state, &world, system, 7), None);
        assert!(state.is_uprising(system));

        add_regiment(&mut world, system, 0x1000_0002, true);
        assert_eq!(
            end_if_garrisoned(&mut state, &world, system, 7),
            Some(UprisingEvent::UprisingEnded { system, tick: 7 })
        );
        assert!(!state.is_uprising(system));
    }

    #[test]
    fn support_changes_that_favour_a_strongly_supported_alliance_are_halved() {
        // FUN_00559be0 divides by GNPRTB 7681 when strong and the change helps
        // side 1 or hurts side 2; FUN_0053e0d0 clamps to 0..=100.
        let (mut world, system) = world_with(Faction::Alliance, 0.5);
        apply_support_change(&mut world, system, Faction::Alliance, 10);
        assert!((world.systems[system].popularity_alliance - 0.55).abs() < 1e-6);
        apply_support_change(&mut world, system, Faction::Alliance, -10);
        assert!((world.systems[system].popularity_alliance - 0.45).abs() < 1e-6);

        let (mut world, system) = world_with(Faction::Alliance, 0.95);
        apply_support_change(&mut world, system, Faction::Alliance, 20);
        assert!((world.systems[system].popularity_alliance - 1.0).abs() < 1e-6);
    }

    #[test]
    fn a_disaster_erodes_resources_and_can_destroy_any_sides_facility() {
        // FUN_00559e10 loses each unit with (remaining units) * GNPRTB 7715
        // percent; FUN_00511930 then destroys each facility, either side, with
        // GNPRTB 7716 = 10 percent. Zero rolls pass every check.
        let (mut world, system) = world_with(Faction::Alliance, 0.5);
        let ours = add_mine(&mut world, system, true);
        let theirs = world.defense_facilities.insert(DefenseFacilityInstance {
            class_dat_id: DatId::new(0x2200_0001),
            side: crate::dat::Faction::Empire,
        });
        world.systems[system].defense_facilities.push(theirs);
        let mut state = UprisingState {
            next_disaster_tick: Some(3),
            ..UprisingState::default()
        };
        let events = UprisingSystem::advance(
            &mut state,
            &world,
            &EconomyState::default(),
            &MissionState::new(),
            &at(3),
            &[0.0; 16],
        );
        assert_eq!(
            events,
            vec![UprisingEvent::Disaster {
                system,
                tick: 3,
                total_energy: 0,
                raw_materials: 0,
                destroyed: vec![FacilityRef::Production(ours), FacilityRef::Defense(theirs)],
            }]
        );
        // The next disaster is 1 + rand(0..=399) ticks later.
        assert_eq!(state.next_disaster_tick, Some(4));

        apply_uprising_event(&mut world, &events[0]);
        assert_eq!(world.systems[system].total_energy, 0);
        assert!(world.systems[system].production_facilities.is_empty());
        assert!(world.systems[system].defense_facilities.is_empty());
    }

    #[test]
    fn a_disaster_that_erodes_nothing_still_costs_one_raw_material() {
        // FUN_00559e10: with no unit lost, one raw material goes (energy only
        // when there is none). The cap at energy is pinned by `eroded(5, 2, ..)`.
        let (world, _system) = world_with(Faction::Alliance, 0.5);
        let mut rolls = Rolls::new(&[0.0]);
        let event = resolve_disaster(&world, 1, &mut rolls);
        // The pick consumed the only roll; every later check fails.
        assert!(matches!(
            event,
            Some(UprisingEvent::Disaster {
                total_energy: 4,
                raw_materials: 3,
                ..
            })
        ));
    }

    #[test]
    fn a_disaster_spares_a_system_without_energy_or_raw_materials() {
        // FUN_0050cdc0 raises the disaster only when +0x5c or +0x64 is non-zero.
        let (mut world, system) = world_with(Faction::Alliance, 0.5);
        world.systems[system].total_energy = 0;
        world.systems[system].raw_materials = 0;
        let mut rolls = Rolls::new(&[0.0]);
        assert_eq!(resolve_disaster(&world, 1, &mut rolls), None);
    }

    /// A roll that `Rolls::chance` draws as exactly `n` (of `0..=99`).
    fn d(n: u32) -> f64 {
        (f64::from(n) + 0.5) / 100.0
    }

    /// Raw materials and energy after a disaster at the only system, with
    /// `rolls` following the system pick.
    fn eroded(raw: u8, energy: u8, rolls: &[f64]) -> (u8, u8) {
        let (mut world, system) = world_with(Faction::Alliance, 0.5);
        world.systems[system].raw_materials = raw;
        world.systems[system].total_energy = energy;
        let mut all = vec![0.0];
        all.extend_from_slice(rolls);
        match resolve_disaster(&world, 1, &mut Rolls::new(&all)) {
            Some(UprisingEvent::Disaster {
                raw_materials,
                total_energy,
                ..
            }) => (raw_materials, total_energy),
            other => panic!("expected a disaster, got {other:?}"),
        }
    }

    #[test]
    fn disaster_erosion_risks_each_unit_at_five_percent_per_remaining_unit() {
        // FUN_00559e10: for i < max(raw, energy), lose raw unit i with chance
        // (energy - lost_raw - lost_energy + raw) * GNPRTB 7715 percent, then
        // energy unit i with (raw - lost_raw - lost_energy + energy) * 7715.
        // Raw 2, energy 3: 25 (pass), 20 (fail at 20), 20 (fail), 20 (pass),
        // 15 (fail at 15); the trailing roll is never drawn.
        assert_eq!(
            eroded(2, 3, &[d(24), d(20), d(25), d(17), d(15), 0.0]),
            (1, 2)
        );
        // Raw 3, energy 3: 30 (fail at 30), 30 (pass), 25 (fail), 25 (pass),
        // then exhausted rolls fail; raw is then capped at energy.
        assert_eq!(eroded(3, 3, &[d(30), d(27), d(30), d(24)]), (1, 1));
    }

    #[test]
    fn a_disaster_that_erodes_nothing_takes_one_raw_material_or_else_one_energy() {
        // FUN_00559e10: when no unit is lost, one raw material goes, or one
        // energy when there is no raw material; raw is then capped at energy.
        // Raw 2, energy 1: three failed checks; energy 1 has no second check.
        assert_eq!(eroded(2, 1, &[d(15), d(15), d(15), 0.0]), (1, 1));
        assert_eq!(eroded(5, 2, &[]), (2, 2));
        assert_eq!(eroded(0, 3, &[]), (0, 2));
        // One energy lost and no raw: no extra raw loss.
        assert_eq!(eroded(1, 3, &[d(20), d(19), d(15), d(15)]), (1, 2));
    }

    #[test]
    fn a_disaster_checks_manufacturing_and_production_before_defense() {
        // FUN_00511930: facilities 0x28..0x2f, then 0x22..0x27, each lost with
        // GNPRTB 7716 = 10 percent (reason 0xb), for either side.
        let (mut world, system) = world_with(Faction::Alliance, 0.5);
        world.systems[system].raw_materials = 0;
        world.systems[system].total_energy = 1;
        let defense = world.defense_facilities.insert(DefenseFacilityInstance {
            class_dat_id: DatId::new(0x2200_0001),
            side: crate::dat::Faction::Alliance,
        });
        world.systems[system].defense_facilities.push(defense);
        let shipyard = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: DatId::new(0x2800_0001),
                side: crate::dat::Faction::Empire,
                is_shipyard: true,
            });
        world.systems[system]
            .manufacturing_facilities
            .push(shipyard);
        let mine = add_mine(&mut world, system, true);
        // Pick, one energy check (5 percent, fails), shipyard (passes), mine
        // (fails at 10), defense (passes).
        let rolls = [0.0, d(5), d(9), d(10), d(0)];
        let event = resolve_disaster(&world, 1, &mut Rolls::new(&rolls)).expect("a disaster");
        assert!(matches!(
            &event,
            UprisingEvent::Disaster { destroyed, total_energy: 0, .. }
                if destroyed == &[FacilityRef::Manufacturing(shipyard), FacilityRef::Defense(defense)]
        ));

        apply_uprising_event(&mut world, &event);
        assert!(world.systems[system].manufacturing_facilities.is_empty());
        assert!(!world.manufacturing_facilities.contains_key(shipyard));
        assert_eq!(world.systems[system].production_facilities, vec![mine]);
    }

    #[test]
    fn an_unpopulated_or_destroyed_system_never_revolts() {
        // FUN_0050b800 requires +0x88 bit 0 (populated); destroyed systems
        // are no longer GameObjExisting.
        let (mut world, system) = world_with(Faction::Alliance, 0.3);
        let revolts = |world: &GameWorld| {
            let mut state = UprisingState {
                next_disaster_tick: Some(u64::MAX),
                ..UprisingState::default()
            };
            UprisingSystem::advance(
                &mut state,
                world,
                &short_of_troops(system, -1),
                &MissionState::new(),
                &at(1),
                &[0.0],
            );
            state.is_uprising(system)
        };
        world.systems[system].is_populated = false;
        assert!(!revolts(&world));
        world.systems[system].is_populated = true;
        world.systems[system].is_destroyed = true;
        assert!(!revolts(&world));
    }

    #[test]
    fn the_first_disaster_is_scheduled_one_to_four_hundred_ticks_ahead() {
        // FUN_00556fa0: event 0x38f at GNPRTB 7717 (1) + rand(0..=7718 (399)).
        let (world, _system) = world_with(Faction::Alliance, 0.9);
        let schedule = |roll: f64| {
            let mut state = UprisingState::default();
            UprisingSystem::advance(
                &mut state,
                &world,
                &EconomyState::default(),
                &MissionState::new(),
                &at(10),
                &[roll],
            );
            state.next_disaster_tick
        };
        assert_eq!(schedule(0.0), Some(11));
        assert_eq!(schedule(0.9999), Some(410));
    }

    #[test]
    fn a_negative_range_draws_between_its_bound_and_zero() {
        // FUN_0053e290(n) for n < 0 returns -rand(1 - n), i.e. -(0..=-n).
        let mut rolls = Rolls::new(&[0.999, 0.0]);
        assert_eq!(rolls.draw(-5), -5);
        assert_eq!(rolls.draw(-5), 0);
    }

    #[test]
    fn uprising_missions_add_incite_and_subtract_subdue_leadership_over_gnprtb_6144() {
        // FUN_005484d0: +0x74 sums Incite Uprising agents' leadership / 6144
        // (10), +0x78 subtracts the same for Subdue Uprising; missions at
        // other systems do not count.
        let (mut world, system) = world_with(Faction::Alliance, 0.3);
        let agent = |world: &mut GameWorld, base, variance| {
            world.characters.insert(Character {
                leadership: SkillPair { base, variance },
                ..Character::default()
            })
        };
        let inciter = agent(&mut world, 25, 0);
        let subduer = agent(&mut world, 30, 22);
        let elsewhere = agent(&mut world, 90, 0);
        let mut missions = MissionState::new();
        missions.dispatch(MissionRequest::single(
            MissionKind::SubdueUprising,
            MissionFaction::Alliance,
            subduer,
            system,
            None,
            0,
        ));
        assert_eq!(
            uprising_mission_terms(&world, missions.missions(), system),
            (0, -4, false)
        );
        missions.dispatch(MissionRequest::single(
            MissionKind::InciteUprising,
            MissionFaction::Empire,
            inciter,
            system,
            None,
            0,
        ));
        missions.dispatch(MissionRequest::single(
            MissionKind::InciteUprising,
            MissionFaction::Empire,
            elsewhere,
            SystemKey::default(),
            None,
            0,
        ));
        assert_eq!(
            uprising_mission_terms(&world, missions.missions(), system),
            (2, -4, true)
        );
    }

    #[test]
    fn an_uprising_mission_counts_the_average_leadership_of_all_its_members() {
        // FUN_00520cd0 averages slot +0x1f4 (leadership) over FUN_00525bb0,
        // every team, decoy, and captured member, before FUN_005484d0
        // divides by GNPRTB 6144 (10). Leadership 20 and 40 average to 30.
        let (mut world, system) = world_with(Faction::Alliance, 0.3);
        let agent = |world: &mut GameWorld, base| {
            world.characters.insert(Character {
                leadership: SkillPair { base, variance: 0 },
                ..Character::default()
            })
        };
        let lead = agent(&mut world, 20);
        let second = agent(&mut world, 40);
        let mut missions = MissionState::new();
        missions.dispatch(MissionRequest {
            team: vec![
                crate::missions::MissionMember::Character(lead),
                crate::missions::MissionMember::Character(second),
            ],
            ..MissionRequest::single(
                MissionKind::SubdueUprising,
                MissionFaction::Alliance,
                lead,
                system,
                None,
                0,
            )
        });

        assert_eq!(
            uprising_mission_terms(&world, missions.missions(), system),
            (0, -3, false)
        );
    }

    #[test]
    fn an_uprising_mission_averages_its_decoys_captured_and_special_forces_too() {
        // FUN_00525bb0 walks the team, decoy, and captured lists, and
        // FUN_00520cd0 reads each member's slot +0x1f4, which a special force
        // answers from its own skill (vtable 0x0065e160). 20, 40, and 60
        // average to 40; FUN_005484d0 divides by GNPRTB 6144 (10).
        use crate::missions::MissionMember;
        let (mut world, system) = world_with(Faction::Alliance, 0.3);
        let agent = |world: &mut GameWorld, base, is_captive| {
            world.characters.insert(Character {
                is_alliance: true,
                is_captive,
                current_system: Some(system),
                leadership: SkillPair { base, variance: 0 },
                ..Character::default()
            })
        };
        let lead = agent(&mut world, 20, false);
        let prisoner = agent(&mut world, 60, true);
        let mut skills = [0; 8];
        skills[crate::world::Skill::Leadership as usize] = 40;
        let unit = world.special_forces.insert(crate::world::SpecialForceUnit {
            class_dat_id: crate::ids::DatId::new(0x3c00_0001),
            is_alliance: true,
            skills,
            on_mission: false,
        });
        world.systems[system].special_forces.push(unit);
        let mut missions = MissionState::new();
        missions
            .dispatch_guarded(
                MissionRequest {
                    decoys: vec![
                        MissionMember::SpecialForce(unit),
                        MissionMember::Character(prisoner),
                    ],
                    ..MissionRequest::single(
                        MissionKind::SubdueUprising,
                        MissionFaction::Alliance,
                        lead,
                        system,
                        None,
                        0,
                    )
                },
                &mut world,
            )
            .expect("dispatch");
        assert_eq!(
            missions.missions()[0].captured,
            vec![MissionMember::Character(prisoner)]
        );

        assert_eq!(
            uprising_mission_terms(&world, missions.missions(), system),
            (0, -4, false)
        );

        // port: a member destroyed mid-mission drops out of the count, so
        // 20 and 40 average to 30 (counting it as 0 would give 20).
        world.characters.remove(prisoner);
        assert_eq!(
            uprising_mission_terms(&world, missions.missions(), system),
            (0, -3, false)
        );
    }

    #[test]
    fn the_average_leadership_is_taken_before_the_divisor() {
        // FUN_00520cd0 divides the integer sum by the count: 19 and 22
        // average to 20, and FUN_005484d0 then divides by GNPRTB 6144 (10)
        // for 2. Dividing each member first would give 1.
        let (mut world, system) = world_with(Faction::Alliance, 0.3);
        let agent = |world: &mut GameWorld, base| {
            world.characters.insert(Character {
                leadership: SkillPair { base, variance: 0 },
                ..Character::default()
            })
        };
        let lead = agent(&mut world, 19);
        let second = agent(&mut world, 22);
        let mut missions = MissionState::new();
        missions.dispatch(MissionRequest {
            team: vec![
                crate::missions::MissionMember::Character(lead),
                crate::missions::MissionMember::Character(second),
            ],
            ..MissionRequest::single(
                MissionKind::InciteUprising,
                MissionFaction::Alliance,
                lead,
                system,
                None,
                0,
            )
        });

        assert_eq!(
            uprising_mission_terms(&world, missions.missions(), system),
            (2, 0, true)
        );
    }

    #[test]
    fn support_changes_that_hurt_a_strongly_supported_empire_are_halved() {
        // FUN_00559be0 divides by GNPRTB 7681 when strong and the change
        // hurts side 2; a gain for side 2 is not divided.
        let (mut world, system) = world_with(Faction::Empire, 0.5);
        apply_support_change(&mut world, system, Faction::Empire, -10);
        assert!((world.systems[system].popularity_empire - 0.45).abs() < 1e-6);
        let (mut world, system) = world_with(Faction::Empire, 0.5);
        apply_support_change(&mut world, system, Faction::Empire, 10);
        assert!((world.systems[system].popularity_empire - 0.6).abs() < 1e-6);
    }

    #[test]
    fn a_strongly_supported_alliance_counts_each_regiment_once() {
        // FUN_00559ce0 weights regiments by GNPRTB 7680 only for side 2.
        // Support 50 (strong above GNPRTB 7732 = 40): draws 5 + 5, threshold
        // ceil(10 / 10) = 1, one regiment: 10, UPRIS1TB code 2, UPRIS2TB 3.
        let (mut world, system) = world_with(Faction::Alliance, 0.5);
        add_regiment(&mut world, system, 0x1000_0002, true);
        let mut state = in_revolt(system, 5);
        let events = UprisingSystem::advance(
            &mut state,
            &world,
            &short_of_troops(system, -1),
            &MissionState::new(),
            &at(5),
            &[0.45, 0.45, 0.0],
        );
        assert_eq!(incident(&events).0, &[2, 3]);
    }

    #[test]
    fn a_strongly_supported_empire_counts_each_regiment_twice() {
        // FUN_00559ce0 weights side 2's regiments by GNPRTB 7680 (2) when
        // strong. Support 50: draws 5 + 5, threshold 1, one regiment weighs 2:
        // 9 gives UPRIS1TB code 1 (10 would give code 2), UPRIS2TB 3.
        let (mut world, system) = world_with(Faction::Empire, 0.5);
        add_regiment(&mut world, system, 0x1000_0008, false);
        let mut state = in_revolt(system, 5);
        let events = UprisingSystem::advance(
            &mut state,
            &world,
            &short_of_troops(system, -1),
            &MissionState::new(),
            &at(5),
            &[0.45, 0.45, 0.0],
        );
        assert_eq!(incident(&events).0, &[1, 3]);
    }

    #[test]
    fn the_incite_support_loss_is_halved_only_for_a_strongly_supported_empire() {
        // FUN_00559be0 divides GNPRTB 6145 (-2) by 7681 (2) when the holder is
        // strong and the change hurts side 2; a strong Alliance loses all 2.
        for (side, expected) in [(Faction::Empire, -1), (Faction::Alliance, -2)] {
            let (mut world, system) = world_with(side, 0.5);
            let agent = world.characters.insert(Character::default());
            let mut missions = MissionState::new();
            missions.dispatch(MissionRequest::single(
                MissionKind::InciteUprising,
                MissionFaction::Empire,
                agent,
                system,
                None,
                0,
            ));
            let mut state = in_revolt(system, 5);
            let events = UprisingSystem::advance(
                &mut state,
                &world,
                &short_of_troops(system, -1),
                &missions,
                &at(5),
                &[0.0, 0.0, 0.0],
            );
            assert_eq!(incident(&events).2, expected, "{side:?}");
        }
    }

    #[test]
    fn the_roll_budget_reserves_the_worst_case_of_a_tick_with_an_incident_and_a_disaster_due() {
        // A draw past the slice yields its maximum, so a slice with spare rolls
        // changes the outcome whenever the budget falls short.
        let (mut world, system) = world_with(Faction::Alliance, 0.0);
        world.systems[system].raw_materials = 12;
        world.systems[system].total_energy = 9;
        add_mine(&mut world, system, true);
        add_mine(&mut world, system, false);
        let defense = world.defense_facilities.insert(DefenseFacilityInstance {
            class_dat_id: DatId::new(0x2200_0001),
            side: crate::dat::Faction::Alliance,
        });
        world.systems[system].defense_facilities.push(defense);
        let yard = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: DatId::new(0x2800_0001),
                side: crate::dat::Faction::Empire,
                is_shipyard: true,
            });
        world.systems[system].manufacturing_facilities.push(yard);
        add_regiment(&mut world, system, 0x1000_0002, true);
        let mut state = in_revolt(system, 5);
        state.next_disaster_tick = Some(5);
        let budget = UprisingSystem::roll_budget(&state, &world, 5);
        // One system, one due incident (11), and the disaster: pick and delay
        // (2), 12 raw, 9 energy and four facilities.
        assert_eq!(budget, 1 + 11 + 2 + 12 + 9 + 4);
        let mut quiet = in_revolt(system, 6);
        quiet.next_disaster_tick = None;
        assert_eq!(UprisingSystem::roll_budget(&quiet, &world, 5), 1 + 1);
        quiet.next_disaster_tick = Some(6);
        assert_eq!(UprisingSystem::roll_budget(&quiet, &world, 5), 1);
        let run = |extra: usize| {
            let mut state = state.clone();
            let events = UprisingSystem::advance(
                &mut state,
                &world,
                &short_of_troops(system, -1),
                &MissionState::new(),
                &at(5),
                &vec![0.0; budget + extra],
            );
            (events, state.next_disaster_tick)
        };
        assert_eq!(run(0), run(100));
    }

    #[test]
    fn an_advance_without_ticks_changes_nothing() {
        let (world, system) = world_with(Faction::Alliance, 0.3);
        let mut state = in_revolt(system, 0);
        let events = UprisingSystem::advance(
            &mut state,
            &world,
            &short_of_troops(system, -1),
            &MissionState::new(),
            &[],
            &[0.0],
        );
        assert!(events.is_empty());
        assert_eq!(state.active_uprisings[&system].next_incident_tick, Some(0));
        assert_eq!(state.next_disaster_tick, Some(u64::MAX));
    }

    #[test]
    fn the_end_check_ignores_a_system_that_is_not_in_revolt() {
        let (world, system) = world_with(Faction::Alliance, 0.3);
        let mut state = UprisingState::default();
        assert_eq!(end_if_garrisoned(&mut state, &world, system, 1), None);
    }

    #[test]
    fn only_empire_stormtrooper_regiments_count_as_stormtroopers() {
        // FUN_005091f0 counts side-2 regiments of class 0x10000006. At an
        // Alliance system with support 0: 2 + 6 - 2 regiments = 6, code 1;
        // the Alliance regiment of that class and an Empire line regiment do
        // not lower it further.
        let (mut world, system) = world_with(Faction::Alliance, 0.0);
        add_regiment(&mut world, system, STORMTROOPER_REGIMENT, true);
        add_regiment(&mut world, system, 0x1000_0002, true);
        add_regiment(&mut world, system, 0x1000_0008, false);
        let mut state = in_revolt(system, 5);
        let events = UprisingSystem::advance(
            &mut state,
            &world,
            &short_of_troops(system, -1),
            &MissionState::new(),
            &at(5),
            &[0.0, 0.0],
        );
        assert_eq!(incident(&events).0, &[1, 0]);
    }

    #[test]
    fn an_injury_falls_only_on_a_free_character_of_the_holder_at_the_system() {
        // FUN_0050d150 code 3 picks among the holder's characters that the
        // system holds directly and that are usable (FUN_004f2640 mode 1,
        // +0x50 bit 0) and not captive (+0xac bit 0): not in a fleet, not on a
        // mission, not killed. The injury uses the effective combat (60):
        // chance 40, injury 40 + 29 + 1.
        let (mut world, system) = world_with(Faction::Alliance, 0.0);
        let at_system = |alliance: bool| Character {
            is_alliance: alliance,
            is_empire: !alliance,
            current_system: Some(system),
            combat: SkillPair {
                base: 50,
                variance: 20,
            },
            ..Character::default()
        };
        let agent = world.characters.insert(at_system(true));
        world.characters.insert(Character {
            current_fleet: Some(crate::ids::FleetKey::default()),
            ..at_system(true)
        });
        world.characters.insert(Character {
            is_captive: true,
            ..at_system(true)
        });
        world.characters.insert(Character {
            is_killed: true,
            ..at_system(true)
        });
        world.characters.insert(Character {
            on_mission: true,
            ..at_system(true)
        });
        world.characters.insert(Character {
            on_mandatory_mission: true,
            ..at_system(true)
        });
        world.characters.insert(at_system(false));
        world.characters.insert(Character {
            current_system: None,
            ..at_system(true)
        });
        let mut state = in_revolt(system, 5);
        // Score draws 1 + 2 (score 9: codes 1 and 3); code 1 has nothing to
        // pick; the last-candidate pick, a 39 under the 40 percent chance,
        // then the largest injury draws.
        let events = UprisingSystem::advance(
            &mut state,
            &world,
            &short_of_troops(system, -1),
            &MissionState::new(),
            &at(5),
            &[0.0, 0.15, 0.999, d(39), 0.999, 0.999],
        );
        let (codes, losses, _) = incident(&events);
        assert_eq!(codes, &[1, 3]);
        assert_eq!(
            losses,
            &[IncidentLoss::CharacterInjured {
                character: agent,
                injury: 70
            }]
        );
    }

    #[test]
    fn code_four_frees_one_prisoner_the_holder_keeps() {
        // FUN_0050d150 code 4 frees one random captive held by the holder;
        // code 5 frees them all. Score 1 + 4 + 6 = 11: UPRIS1TB 2, UPRIS2TB 4.
        let (mut world, system) = world_with(Faction::Alliance, 0.0);
        let prisoner = |world: &mut GameWorld| {
            world.characters.insert(Character {
                is_empire: true,
                is_captive: true,
                captured_by: Some(Faction::Alliance),
                current_system: Some(system),
                ..Character::default()
            })
        };
        let _first = prisoner(&mut world);
        let second = prisoner(&mut world);
        let mut state = in_revolt(system, 5);
        let events = UprisingSystem::advance(
            &mut state,
            &world,
            &short_of_troops(system, -1),
            &MissionState::new(),
            &at(5),
            &[0.0, 0.35, 0.999],
        );
        let (codes, losses, _) = incident(&events);
        assert_eq!(codes, &[2, 4]);
        assert_eq!(losses, &[IncidentLoss::PrisonerFreed(second)]);
    }

    #[test]
    fn a_subdue_mission_lowers_the_incident_score() {
        // FUN_00559ce0 adds +0x78, minus the Subdue agents' leadership / 6144:
        // 2 + 6 - 30 / 10 = 5, below every UPRIS1TB row that acts.
        let (mut world, system) = world_with(Faction::Alliance, 0.0);
        let agent = world.characters.insert(Character {
            is_alliance: true,
            leadership: SkillPair {
                base: 30,
                variance: 0,
            },
            ..Character::default()
        });
        let mut missions = MissionState::new();
        missions.dispatch(MissionRequest::single(
            MissionKind::SubdueUprising,
            MissionFaction::Alliance,
            agent,
            system,
            None,
            0,
        ));
        let mut state = in_revolt(system, 5);
        let events = UprisingSystem::advance(
            &mut state,
            &world,
            &short_of_troops(system, -1),
            &missions,
            &at(5),
            &[0.0, 0.0],
        );
        assert_eq!(incident(&events).0, &[0, 0]);
    }

    #[test]
    fn an_overdue_timer_fires_once_per_advance() {
        // Events apply after advance returns, so a second fire in the same
        // call would read the world before the first one's losses.
        let (world, system) = world_with(Faction::Alliance, 0.0);
        let mut state = in_revolt(system, 1);
        state.next_disaster_tick = Some(1);
        let events = UprisingSystem::advance(
            &mut state,
            &world,
            &short_of_troops(system, -1),
            &MissionState::new(),
            &at(500),
            &[0.0; 16],
        );
        let incidents = events
            .iter()
            .filter(|e| matches!(e, UprisingEvent::UprisingIncident { .. }))
            .count();
        let disasters = events
            .iter()
            .filter(|e| matches!(e, UprisingEvent::Disaster { .. }))
            .count();
        assert_eq!((incidents, disasters), (1, 1));
        assert_eq!(state.active_uprisings[&system].next_incident_tick, Some(31));
        assert_eq!(state.next_disaster_tick, Some(2));
    }
}
