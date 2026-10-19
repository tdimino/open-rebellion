//! Blockade mechanics: manufacturing halt and troop regiment losses.
//!
//! A blockade occurs when a hostile fleet is present at a system with no
//! defending fleet (`FUN_0050b8e0`, `ghidra/notes/blockade-bit.md`). This
//! module:
//!
//! 1. **Determines blockaded systems** each tick from fleet disposition.
//! 2. **Emits `BlockadeEvent`s** when blockade state changes (enter/exit).
//! 3. **Halts manufacturing** — `BlockadeState::is_blockaded()` is consulted
//!    by `ManufacturingSystem::advance` to skip queue advancement at blocked systems.
//! 4. **Destroys regiments running the blockade** — a regiment carried into a
//!    blockaded system takes that system's withdraw percent, and rolls against
//!    it when its fleet leaves. Surface garrisons and landed regiments are
//!    never rolled.
//!
//! # Architecture
//!
//! Follows the stateless advance pattern:
//! ```text
//! BlockadeSystem::advance(&mut BlockadeState, &world, &[TickEvent])
//!     -> Vec<BlockadeEvent>
//! BlockadeSystem::running_regiments(&mut BlockadeState, &world, &movement, &transport)
//!     -> Vec<RunningRegiment>
//! BlockadeSystem::resolve_running(&[RunningRegiment], &rolls, tick)
//!     -> Vec<BlockadeEvent>
//! ```
//! The caller applies `BlockadeEvent::TroopDestroyed` to `GameWorld` and to
//! `TroopTransportState` (the regiment is removed from its fleet's cargo and
//! from the arena).
//!
//! # Source
//!
//! Ghidra RE: `economy-systems.md §4`.
//! - `FUN_0050e820` (production eligibility): step 8 — fleet active check gates
//!   manufacturing when a hostile blockader is present.
//! - `FUN_005122d0` (`SystemBlockadeNotif`), event `0x14e` (334).
//! - `FUN_00512540` (`BattlePendingCausedCurrentBlockade`): hostile fleet arrival
//!   entails both `BattlePending` and `Blockade` simultaneously.
//!
//! Running the blockade (recovered 2026-09-26, notes in `ghidra/notes`):
//! - `FUN_0050b310` sets the system's withdraw percent (`+0x74`): 100, unless
//!   the system is blockaded (`+0x88` bit 5) and has no active KDY-150
//!   (family `0x22`, iterator `FUN_00527550`). Then it is
//!   `FUN_0055a020(ships, fighters)` =
//!   `max(0, 100 - ships * GNPRTB[7684] - fighters * GNPRTB[7685])`, counting
//!   every active fleet's capital ships and fighters at the system.
//! - A regiment that becomes usable in a system copies that value (event
//!   0x301's `FUN_00514510` calls the system's `+0xc8` slot `FUN_00508660`,
//!   which calls `FUN_0050c540` then `FUN_00504470`). Later changes to the
//!   system value are not pushed to regiments already there: the system's
//!   `+0x1e4` slot is `FUN_00524b10`,
//!   which only returns 1.
//! - Activating a regiment resets it to 100 (`FUN_004f7410` calls slot `+0x110`
//!   `FUN_00504960`).
//! - When the regiment enters transit (`FUN_004f7640` sets `+0x50` bit 5), slot
//!   `+0x124` `FUN_00504990` keeps it only if `random(0..=99) < percent`
//!   (`FUN_0053e2f0`). Otherwise `FUN_00504400` marks it destroyed, which
//!   raises `TroopRegDestroyedRunningBlockade` (`FUN_00504a00`, event `0x340`)
//!   and removes it through slot `+0x1f0` (`FUN_005049e0`).
//!
//! The blockade support shift, timer `0x386` (`timer-scheduler.md`, "Timer
//! `0x386`"): while a system's blockade bit is set, a per-system timer fires
//! every GNPRTB 7694 (30) days and moves the blockading side's support by +1
//! (7693) when that side holds the majority of support, else by -1 (7695)
//! (`FUN_0050cb20`), through the support setter `FUN_0050c9f0`.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::ids::{DatId, SystemKey, TroopKey};
use crate::movement::MovementState;
use crate::tick::TickEvent;
use crate::troop_transport::TroopTransportState;
use crate::world::ControlKind;
use crate::world::GameWorld;

// ---------------------------------------------------------------------------
// Event IDs (from RE annotated-functions.md)
// ---------------------------------------------------------------------------

/// `SystemBlockadeNotif` event ID — fires when blockade state changes.
/// `FUN_005122d0` (`annotated-functions.md`, "System Blockade Notification").
pub const EV_BLOCKADE: u32 = 0x14e; // 334

/// `TroopRegDestroyedRunningBlockade` event ID, registered by `FUN_00504a00`
/// through `FUN_0053fe40` (`blockade-troop-withdrawal.md`).
pub const EV_TROOP_BLOCKADE_KILL: u32 = 0x340; // 832

/// DEFFACSD record 1, the KDY-150 (TEXTSTRA 8704, family `0x22`). An active one
/// keeps the system's withdraw percent at 100 (`FUN_0050b310`).
pub const KDY_150_CLASS_ID: u32 = 1;

/// GNPRTB 7684 (`0x1e04`): withdraw penalty per capital ship. `FUN_00558bb0`
/// loads it into `DAT_006bb458`.
pub const GNPRTB_WITHDRAW_PENALTY_PER_CAPSHIP: u16 = 7684;

/// GNPRTB 7685 (`0x1e05`): withdraw penalty per fighter squadron. `FUN_00558bb0`
/// loads it into `DAT_006bb3cc`.
pub const GNPRTB_WITHDRAW_PENALTY_PER_FIGHTER: u16 = 7685;

/// `DAT_00661a88`, the constant 100: the full withdraw percent and the bound of
/// the survival roll.
pub const FULL_WITHDRAW_PERCENT: u8 = 100;

/// GNPRTB 7693: timer `0x386`'s shift for a blockader that holds the
/// majority of support (`FUN_0050cb20.c:15-21`).
const GNPRTB_SHIFT_WITH_MAJORITY: u16 = 7693;
/// GNPRTB 7694: timer `0x386`'s delay when the blockader holds the majority;
/// also the record's minimum at creation (`FUN_0050e200.c:50`).
const GNPRTB_DELAY_WITH_MAJORITY: u16 = 7694;
/// GNPRTB 7695: the shift for a blockader without the majority.
const GNPRTB_SHIFT_WITHOUT_MAJORITY: u16 = 7695;
/// GNPRTB 7696: the delay when the blockader lacks the majority
/// (`FUN_005109f0.c:37-47`).
const GNPRTB_DELAY_WITHOUT_MAJORITY: u16 = 7696;

// ---------------------------------------------------------------------------
// BlockadeEvent
// ---------------------------------------------------------------------------

/// Events emitted by `BlockadeSystem::advance` each tick.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockadeEvent {
    /// A system has entered a blockaded state (hostile fleet, no defender).
    ///
    /// Manufacturing is halted for the controlling faction until blockade ends.
    /// Corresponds to event `0x14e` (`SystemBlockadeNotif`) with state = enter.
    BlockadeStarted { system: SystemKey, tick: u64 },

    /// A system's blockade has ended (defending fleet arrived or attacker withdrew).
    ///
    /// Corresponds to event `0x14e` (`SystemBlockadeNotif`) with state = exit.
    BlockadeEnded { system: SystemKey, tick: u64 },

    /// A regiment was destroyed leaving the blockaded `system` it had been
    /// carried into.
    ///
    /// From RE: `FUN_00504990` roll, `FUN_00504a00` notification, event `0x340`.
    /// The caller must remove `troop` from its fleet's cargo and from
    /// `GameWorld::troops`.
    TroopDestroyed {
        system: SystemKey,
        troop: TroopKey,
        tick: u64,
    },

    /// Timer `0x386` fired (`FUN_00565d90` -> `FUN_0050cb20`): `side`, the
    /// blockader, takes `delta` points of support (GNPRTB 7693 or 7695),
    /// before the setter's Death Star halving and clamp. Only a populated
    /// system shifts (`FUN_0050c9f0.c:11`). It raises no event or notice.
    SupportShift {
        system: SystemKey,
        side: crate::dat::Faction,
        delta: i32,
        tick: u64,
    },
}

impl BlockadeEvent {
    #[must_use]
    pub fn tick(&self) -> u64 {
        match self {
            BlockadeEvent::BlockadeStarted { tick, .. }
            | BlockadeEvent::BlockadeEnded { tick, .. }
            | BlockadeEvent::TroopDestroyed { tick, .. }
            | BlockadeEvent::SupportShift { tick, .. } => *tick,
        }
    }
}

// ---------------------------------------------------------------------------
// BlockadeState
// ---------------------------------------------------------------------------

/// Per-system blockade status, maintained across ticks.
///
/// `BlockadeSystem::advance` updates this state and emits change events.
/// `ManufacturingSystem::advance` calls `is_blockaded(system)` to skip
/// queue advancement at blocked systems.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlockadeState {
    /// Systems currently under blockade (hostile fleet present, no defender).
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_set",
        deserialize_with = "crate::serde_ordered::deserialize_hash_set"
    )]
    blockaded: HashSet<SystemKey>,
    /// Last observed position and withdraw percent of each embarked regiment.
    ///
    /// Persisted since save v15. Loading a v14 or older save starts every
    /// embarked regiment at full withdraw percent, which never destroys one
    /// wrongly (FUN_00504960 resets to 100 on activation).
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    embarked: HashMap<TroopKey, EmbarkedRegiment>,
    /// Each system's timer `0x386` record (system `+0x54`+`0x30`). A system
    /// without an entry has the creation minimum, disarmed. The record keeps
    /// its minimum across blockades.
    #[serde(
        default,
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    support_shift: HashMap<SystemKey, SupportShiftTimer>,
}

/// Timer `0x386`'s record and its pending fire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportShiftTimer {
    /// The record's minimum, the delay: GNPRTB 7694 at creation
    /// (`FUN_0050e200.c:50`), then what the support hook last wrote while
    /// blockaded (`FUN_005109f0.c:37-48`). No spread (`FUN_00586130.c:7-8`).
    pub min_delay: u32,
    /// The tick the timer fires; `None` while the blockade bit is clear.
    pub due_tick: Option<u64>,
}

/// An embarked regiment's last observed orbit and its withdraw percent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbarkedRegiment {
    /// The system its fleet orbits, or `None` while in transit.
    pub orbit: Option<SystemKey>,
    /// Chance in percent that it survives leaving `orbit` (regiment `+0x60`).
    pub withdraw_percent: u8,
}

/// A regiment whose fleet has just left a system with a withdraw percent below
/// 100. `BlockadeSystem::resolve_running` decides whether it survives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunningRegiment {
    pub troop: TroopKey,
    pub system: SystemKey,
    pub withdraw_percent: u8,
}

impl BlockadeState {
    #[must_use]
    pub fn new() -> Self {
        BlockadeState {
            blockaded: HashSet::new(),
            embarked: HashMap::new(),
            support_shift: HashMap::new(),
        }
    }

    /// The tracked position and withdraw percent of an embarked regiment.
    #[must_use]
    pub fn embarked_regiment(&self, troop: TroopKey) -> Option<EmbarkedRegiment> {
        self.embarked.get(&troop).copied()
    }

    /// Remove blockade state for a system (used when system is destroyed).
    pub fn clear_blockade(&mut self, system: SystemKey) {
        self.blockaded.remove(&system);
        // hyp: a destroyed system's pending timer goes with it; slot `+0x1c`
        // (`0x00586290`) is not read.
        self.support_shift.remove(&system);
    }

    /// The system's timer `0x386` record, if it was ever armed.
    #[must_use]
    pub fn support_shift_timer(&self, system: SystemKey) -> Option<SupportShiftTimer> {
        self.support_shift.get(&system).copied()
    }

    /// Returns `true` if `system` is currently blockaded.
    ///
    /// Called by `ManufacturingSystem::advance` to halt production.
    #[must_use]
    pub fn is_blockaded(&self, system: SystemKey) -> bool {
        self.blockaded.contains(&system)
    }

    /// All currently blockaded systems.
    #[must_use]
    pub fn blockaded_systems(&self) -> &HashSet<SystemKey> {
        &self.blockaded
    }
}

// ---------------------------------------------------------------------------
// BlockadeSystem
// ---------------------------------------------------------------------------

/// Stateless blockade evaluator.
pub struct BlockadeSystem;

impl BlockadeSystem {
    /// Advance blockade state for all systems.
    ///
    /// For each system in `world`:
    /// - If a hostile fleet is present AND no defending fleet: mark blockaded.
    /// - If previously blockaded but conditions no longer hold: clear blockade.
    /// - Emit `BlockadeStarted` / `BlockadeEnded` on transitions.
    /// - Stationed `ground_units` remain intact; regiments running the blockade
    ///   come from `running_regiments` and `resolve_running`.
    ///
    /// Does **not** mutate `GameWorld` — returns events for the caller to apply.
    /// Skips zero-tick frames.
    pub fn advance(
        state: &mut BlockadeState,
        world: &GameWorld,
        tick_events: &[TickEvent],
    ) -> Vec<BlockadeEvent> {
        let Some(last_tick_event) = tick_events.last() else {
            return Vec::new();
        };

        let tick = last_tick_event.tick;
        let mut events = Vec::new();

        // Compute current blockade set from world fleet disposition
        let now_blockaded: HashSet<SystemKey> = world
            .systems
            .iter()
            .filter_map(|(sys_key, sys)| {
                if Self::system_is_blockaded(world, sys) {
                    Some(sys_key)
                } else {
                    None
                }
            })
            .collect();

        // HashSet iteration order is randomized per process. Events mutate
        // world state downstream, so emit transitions by stable system key.
        let mut newly_blockaded: Vec<_> = now_blockaded
            .difference(&state.blockaded)
            .copied()
            .collect();
        newly_blockaded.sort_unstable();

        // New blockades (entered this tick)
        for sys_key in newly_blockaded {
            Self::trace_transition(world, sys_key, "started");
            events.push(BlockadeEvent::BlockadeStarted {
                system: sys_key,
                tick,
            });
        }

        // Cleared blockades (ended this tick)
        let mut cleared: Vec<_> = state
            .blockaded
            .difference(&now_blockaded)
            .copied()
            .collect();
        cleared.sort_unstable();
        for sys_key in cleared {
            Self::trace_transition(world, sys_key, "ended");
            events.push(BlockadeEvent::BlockadeEnded {
                system: sys_key,
                tick,
            });
        }

        state.blockaded = now_blockaded;
        Self::advance_support_shift(state, world, tick, &mut events);
        events
    }

    /// Timer `0x386` over the blockaded systems, in key order: disarmed when
    /// the bit clears and armed with the record's minimum when it sets
    /// (`FUN_00511300.c:106`, `FUN_0053fa60`), the minimum rewritten by the
    /// majority (`FUN_005109f0.c:36-48`, a changed value restarting the
    /// timer, `FUN_0053fab0.c:12`), then fired when due, the delay added
    /// to the due day (`FUN_005862a0.c:25-30`). port: the original rewrites the
    /// minimum at each support change; the port checks once a tick, after
    /// the last tick's support writes (7694 = 7696 = 30 in shipped data).
    /// port: a delay of 0 or less (no GNPRTB) never arms. port: every due
    /// day one call covers takes that call's majority and delta; the
    /// shipped delays (30) leave at most one due day in a call. hyp: a blockade
    /// standing when a game starts arms at its first tick; the original's
    /// hooks do not run during setup (`DAT_006b90e0`).
    fn advance_support_shift(
        state: &mut BlockadeState,
        world: &GameWorld,
        tick: u64,
        events: &mut Vec<BlockadeEvent>,
    ) {
        let value = |id| world.gnprtb.value(id, world.difficulty_index);
        let creation = value(GNPRTB_DELAY_WITH_MAJORITY);
        let mut systems: Vec<SystemKey> = state
            .support_shift
            .keys()
            .copied()
            .chain(state.blockaded.iter().copied())
            .collect();
        systems.sort_unstable();
        systems.dedup();
        for system in systems {
            let blockaded = state.blockaded.contains(&system);
            let Some(sys) = world.systems.get(system) else {
                continue;
            };
            let timer = state
                .support_shift
                .entry(system)
                .or_insert(SupportShiftTimer {
                    min_delay: u32::try_from(creation).unwrap_or(0),
                    due_tick: None,
                });
            if !blockaded {
                timer.due_tick = None;
                continue;
            }
            if timer.due_tick.is_none() && timer.min_delay > 0 {
                timer.due_tick = Some(tick + u64::from(timer.min_delay));
            }
            let Some(blockader) = Self::blockader(world, sys) else {
                continue;
            };
            let with_majority = Self::support_majority(sys) == Some(blockader);
            let want = u32::try_from(value(if with_majority {
                GNPRTB_DELAY_WITH_MAJORITY
            } else {
                GNPRTB_DELAY_WITHOUT_MAJORITY
            }))
            .unwrap_or(0);
            if want != timer.min_delay {
                timer.min_delay = want;
                timer.due_tick = (want > 0).then(|| tick + u64::from(want));
            }
            // A frame can cover several days: each due day in it fires, and
            // the next is due a delay after it.
            while let Some(due) = timer
                .due_tick
                .filter(|&due| due <= tick && timer.min_delay > 0)
            {
                timer.due_tick = Some(due + u64::from(timer.min_delay));
                let delta = value(if with_majority {
                    GNPRTB_SHIFT_WITH_MAJORITY
                } else {
                    GNPRTB_SHIFT_WITHOUT_MAJORITY
                });
                if sys.is_populated && delta != 0 {
                    events.push(BlockadeEvent::SupportShift {
                        system,
                        side: blockader,
                        delta,
                        tick: due,
                    });
                }
            }
        }
    }

    /// The blockading side: the side of the system's active fleets
    /// (`FUN_00509890.c:19-26`); a blockaded system holds one side's only.
    fn blockader(world: &GameWorld, sys: &crate::world::System) -> Option<crate::dat::Faction> {
        use crate::dat::Faction;
        match Self::presence(world, sys) {
            (true, false) => Some(Faction::Alliance),
            (false, true) => Some(Faction::Empire),
            _ => None,
        }
    }

    /// The side holding the majority of support (`FUN_00509930`: the
    /// Alliance's support at `+0x58` against 100 minus it; 1 Alliance,
    /// 2 Empire, 3 a tie, here `None`).
    fn support_majority(sys: &crate::world::System) -> Option<crate::dat::Faction> {
        use crate::dat::Faction;
        let alliance = crate::uprising::support_points(sys, Faction::Alliance);
        match alliance.cmp(&(100 - alliance)) {
            std::cmp::Ordering::Greater => Some(Faction::Alliance),
            std::cmp::Ordering::Less => Some(Faction::Empire),
            std::cmp::Ordering::Equal => None,
        }
    }

    /// The chance in percent that a regiment carried into `system` now survives
    /// leaving it (`FUN_0050b310`).
    ///
    /// 100 unless the system is blockaded and has no KDY-150. Otherwise
    /// `max(0, 100 - ships * GNPRTB[7684] - fighters * GNPRTB[7685])` over every
    /// fleet at the system and the squadrons it holds, both sides
    /// (`FUN_0055a020`, clamped by
    /// `FUN_0053e120`).
    #[must_use]
    pub fn withdraw_percent(world: &GameWorld, system: SystemKey) -> u8 {
        let Some(sys) = world.systems.get(system) else {
            return FULL_WITHDRAW_PERCENT;
        };
        if !Self::system_is_blockaded(world, sys) {
            return FULL_WITHDRAW_PERCENT;
        }
        let kdy_150 = DatId::new(KDY_150_CLASS_ID);
        let has_kdy_150 = sys
            .defense_facilities
            .iter()
            .filter_map(|&key| world.defense_facilities.get(key))
            .any(|facility| facility.class_dat_id == kdy_150);
        if has_kdy_150 {
            return FULL_WITHDRAW_PERCENT;
        }
        let (ships, fighters) = sys
            .fleets
            .iter()
            .filter_map(|&key| world.fleets.get(key))
            .fold((0_i64, 0_i64), |(ships, fighters), fleet| {
                let squadrons: i64 = fleet.fighters.iter().map(|f| i64::from(f.count)).sum();
                (ships + i64::from(fleet.ship_count()), fighters + squadrons)
            });
        // And the squadrons the system holds, both sides
        // (`blockade-troop-withdrawal.md`).
        let fighters = fighters
            + sys
                .fighters
                .iter()
                .map(|entry| i64::from(entry.count))
                .sum::<i64>();
        let param = |id| i64::from(world.gnprtb.value(id, world.difficulty_index));
        let penalty = ships * param(GNPRTB_WITHDRAW_PENALTY_PER_CAPSHIP)
            + fighters * param(GNPRTB_WITHDRAW_PENALTY_PER_FIGHTER);
        let percent = (i64::from(FULL_WITHDRAW_PERCENT) - penalty).clamp(0, 100);
        u8::try_from(percent).unwrap_or(FULL_WITHDRAW_PERCENT)
    }

    /// Track embarked regiments and return those whose fleet has just left a
    /// system with a withdraw percent below 100.
    ///
    /// A regiment first seen aboard a fleet was loaded from the surface, so it
    /// starts at 100. Arriving in a system copies that system's withdraw
    /// percent; leaving a system reports the regiment for a roll. Regiments
    /// no longer embarked (landed or destroyed) are forgotten, which resets
    /// them to 100 as `FUN_00504960` does on activation.
    pub fn running_regiments(
        state: &mut BlockadeState,
        world: &GameWorld,
        movement: &MovementState,
        transport: &TroopTransportState,
    ) -> Vec<RunningRegiment> {
        let mut running = Vec::new();
        let mut aboard = HashSet::new();
        for fleet_key in transport.fleet_keys() {
            let Some(fleet) = world.fleets.get(fleet_key) else {
                continue;
            };
            let orbit = (!movement.is_in_transit(fleet_key)).then_some(fleet.location);
            for &troop in transport.cargo(fleet_key) {
                aboard.insert(troop);
                let regiment = state.embarked.entry(troop).or_insert(EmbarkedRegiment {
                    orbit,
                    withdraw_percent: FULL_WITHDRAW_PERCENT,
                });
                match (regiment.orbit, orbit) {
                    (previous, current) if previous == current => {}
                    (Some(origin), None) => {
                        if regiment.withdraw_percent < FULL_WITHDRAW_PERCENT {
                            running.push(RunningRegiment {
                                troop,
                                system: origin,
                                withdraw_percent: regiment.withdraw_percent,
                            });
                        }
                    }
                    (_, Some(system)) => {
                        regiment.withdraw_percent = Self::withdraw_percent(world, system);
                    }
                    (None, None) => {}
                }
                regiment.orbit = orbit;
            }
        }
        state.embarked.retain(|troop, _| aboard.contains(troop));
        running
    }

    /// Roll each running regiment against its withdraw percent (`FUN_0053e2f0`).
    ///
    /// `rolls` holds one uniform value in `[0, 1)` per regiment. The draw is
    /// `99 - floor(roll * 100)`, uniform over `0..=99` like `FUN_0053e2e0`, and
    /// the regiment survives when the draw is below its percent. A missing or
    /// padded roll of 1.0 always survives.
    #[must_use]
    pub fn resolve_running(
        running: &[RunningRegiment],
        rolls: &[f64],
        tick: u64,
    ) -> Vec<BlockadeEvent> {
        running
            .iter()
            .zip(rolls)
            .filter_map(|(regiment, &roll)| {
                #[expect(
                    clippy::cast_possible_truncation,
                    reason = "roll is in [0, 1], so the floored product fits in i32"
                )]
                let draw = 99 - (roll * 100.0).floor() as i32;
                crate::trace::record(crate::trace::TraceMask::BLOCKADE, "running_roll", || {
                    serde_json::json!({
                        "troop": format!("{:?}", regiment.troop),
                        "withdraw_percent": regiment.withdraw_percent,
                        "draw": draw,
                        "destroyed": draw >= i32::from(regiment.withdraw_percent),
                    })
                });
                (draw >= i32::from(regiment.withdraw_percent)).then_some(
                    BlockadeEvent::TroopDestroyed {
                        system: regiment.system,
                        troop: regiment.troop,
                        tick,
                    },
                )
            })
            .collect()
    }

    // ── Private ───────────────────────────────────────────────────────────

    /// The systems blockaded now (`+0x88` bit `0x20`), read from the world.
    #[must_use]
    pub fn blockaded_now(world: &GameWorld) -> HashSet<SystemKey> {
        world
            .systems
            .iter()
            .filter(|(_, value)| Self::system_is_blockaded(world, value))
            .map(|(key, _)| key)
            .collect()
    }

    /// Trace a blockade starting or ending with the inputs of the rule.
    fn trace_transition(world: &GameWorld, system: SystemKey, transition: &'static str) {
        crate::trace::record(crate::trace::TraceMask::BLOCKADE, "transition", || {
            let sys = world.systems.get(system);
            let (alliance, empire) = sys.map_or((false, false), |sys| Self::presence(world, sys));
            serde_json::json!({
                "system": sys.map(|sys| sys.name.clone()),
                "transition": transition,
                "control": sys.map(|sys| format!("{:?}", sys.control)),
                "populated": sys.map(|sys| sys.is_populated),
                "alliance_present": alliance,
                "empire_present": empire,
                "withdraw_percent": Self::withdraw_percent(world, system),
            })
        });
    }

    /// Whether the Alliance and the Empire each have a fleet or held
    /// squadrons at the system.
    fn presence(world: &GameWorld, sys: &crate::world::System) -> (bool, bool) {
        sys.fleets
            .iter()
            .filter_map(|&fk| world.fleets.get(fk))
            .fold(
                (sys.squadron_count(true) > 0, sys.squadron_count(false) > 0),
                |(alliance, empire), fleet| {
                    (alliance || fleet.is_alliance, empire || !fleet.is_alliance)
                },
            )
    }

    /// The system's blockade bit (`+0x88` `0x20`), as `FUN_0050b8e0` writes
    /// it (`ghidra/notes/blockade-bit.md`). Both sides present is a battle
    /// (bit `0x1000`, `FUN_00509710`), never a blockade. Otherwise a system
    /// held by one side is blockaded by any fleet of the other, and a
    /// contested one, when populated, by either side's fleets. A neutral
    /// system never is.
    ///
    /// Presence is a side's fleets (`FUN_004ffef0`) or the squadrons the
    /// system holds for it (`FUN_00503a50`). port: every fleet is active.
    /// hyp: `Contested` is the original's side 3, and an `Uprising` system
    /// keeps its holder's side.
    pub(crate) fn system_is_blockaded(world: &GameWorld, sys: &crate::world::System) -> bool {
        use crate::dat::Faction;

        let (alliance, empire) = Self::presence(world, sys);
        if alliance && empire {
            return false;
        }
        match sys.control {
            ControlKind::Contested => sys.is_populated && (alliance || empire),
            control => match control.faction() {
                Some(Faction::Alliance) => empire,
                Some(Faction::Empire) => alliance,
                Some(Faction::Neutral) | None => false,
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dat::{ExplorationStatus, Faction, SectorGroup};
    use crate::ids::DatId;
    use crate::tick::TickEvent;
    use crate::world::{Fleet, GameWorld, Sector, System, TroopUnit};

    fn tick(n: u64) -> TickEvent {
        TickEvent { tick: n }
    }

    fn make_world() -> (GameWorld, SystemKey) {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(0x9200_0000),
            name: "Sector".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let sys = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0000),
            name: "Bespin".into(),
            sector,
            x: 10,
            y: 20,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Alliance),
        });
        (world, sys)
    }

    fn add_fleet(world: &mut GameWorld, system: SystemKey, is_alliance: bool) {
        let fk = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![],
            fighters: vec![],
            characters: vec![],
            is_alliance,
            has_death_star: false,
        });
        world.systems.get_mut(system).unwrap().fleets.push(fk);
    }

    fn add_troop(world: &mut GameWorld, system: SystemKey, is_alliance: bool) -> TroopKey {
        let tk = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1400_0000),
            is_alliance,
            regiment_strength: 100,
        });
        world.systems.get_mut(system).unwrap().ground_units.push(tk);
        tk
    }

    #[test]
    fn no_events_without_ticks() {
        let (mut world, sys) = make_world();
        add_fleet(&mut world, sys, false);
        let mut state = BlockadeState::new();
        let events = BlockadeSystem::advance(&mut state, &world, &[]);
        assert!(events.is_empty());
    }

    #[test]
    fn blockade_transitions_use_stable_system_key_order() {
        let (mut world, system_a) = make_world();
        let sector = world.systems[system_a].sector;
        let system_b = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0001),
            name: "Hoth".into(),
            sector,
            x: 30,
            y: 40,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Alliance),
        });
        add_fleet(&mut world, system_b, false);
        add_fleet(&mut world, system_a, false);

        let mut state = BlockadeState::new();
        let started = BlockadeSystem::advance(&mut state, &world, &[tick(1)]);
        assert_eq!(
            started,
            vec![
                BlockadeEvent::BlockadeStarted {
                    system: system_a,
                    tick: 1,
                },
                BlockadeEvent::BlockadeStarted {
                    system: system_b,
                    tick: 1,
                },
            ]
        );

        world.systems[system_a].fleets.clear();
        world.systems[system_b].fleets.clear();
        let ended = BlockadeSystem::advance(&mut state, &world, &[tick(2)]);
        assert_eq!(
            ended,
            vec![
                BlockadeEvent::BlockadeEnded {
                    system: system_a,
                    tick: 2,
                },
                BlockadeEvent::BlockadeEnded {
                    system: system_b,
                    tick: 2,
                },
            ]
        );
    }

    #[test]
    fn no_blockade_without_hostile_fleet() {
        let (world, _sys) = make_world();
        let mut state = BlockadeState::new();
        let events = BlockadeSystem::advance(&mut state, &world, &[tick(1)]);
        assert!(events.is_empty());
    }

    #[test]
    fn no_blockade_when_contested() {
        let (mut world, sys) = make_world();
        add_fleet(&mut world, sys, true); // Alliance defender
        add_fleet(&mut world, sys, false); // Empire hostile

        let mut state = BlockadeState::new();
        let events = BlockadeSystem::advance(&mut state, &world, &[tick(1)]);
        // Defender present — no blockade
        assert!(!events
            .iter()
            .any(|e| matches!(e, BlockadeEvent::BlockadeStarted { .. })));
        assert!(!state.is_blockaded(sys));
    }

    #[test]
    fn blockade_starts_when_hostile_fleet_uncontested() {
        let (mut world, sys) = make_world();
        add_fleet(&mut world, sys, false); // Empire fleet at Alliance-controlled system

        let mut state = BlockadeState::new();
        let events = BlockadeSystem::advance(&mut state, &world, &[tick(1)]);
        assert!(events
            .iter()
            .any(|e| matches!(e, BlockadeEvent::BlockadeStarted { .. })));
        assert!(state.is_blockaded(sys));
    }

    #[test]
    fn blockade_ends_when_defender_arrives() {
        let (mut world, sys) = make_world();
        add_fleet(&mut world, sys, false); // Empire blockader

        let mut state = BlockadeState::new();
        BlockadeSystem::advance(&mut state, &world, &[tick(1)]); // starts blockade
        assert!(state.is_blockaded(sys));

        // Alliance fleet arrives
        add_fleet(&mut world, sys, true);
        let events = BlockadeSystem::advance(&mut state, &world, &[tick(2)]);
        assert!(events
            .iter()
            .any(|e| matches!(e, BlockadeEvent::BlockadeEnded { .. })));
        assert!(!state.is_blockaded(sys));
    }

    fn blockaded_under(control: ControlKind, alliance_fleet: bool, empire_fleet: bool) -> bool {
        let (mut world, sys) = make_world();
        world.systems[sys].control = control;
        if alliance_fleet {
            add_fleet(&mut world, sys, true);
        }
        if empire_fleet {
            add_fleet(&mut world, sys, false);
        }
        let mut state = BlockadeState::new();
        BlockadeSystem::advance(&mut state, &world, &[tick(1)]);
        state.is_blockaded(sys)
    }

    // FUN_0050b8e0: a populated side-3 system takes the blockade bit from any
    // fleet whose side differs from 3. hyp: `Contested` is side 3.
    #[test]
    fn either_sides_fleets_alone_blockade_a_contested_system() {
        assert!(blockaded_under(ControlKind::Contested, true, false));
        assert!(blockaded_under(ControlKind::Contested, false, true));
        assert!(!blockaded_under(ControlKind::Contested, false, false));
    }

    // FUN_0050b8e0 → FUN_00509710: both sides present sets the battle bit and
    // clears the blockade bit, whoever holds the system.
    #[test]
    fn both_sides_fleets_make_a_battle_not_a_blockade() {
        for control in [
            ControlKind::Contested,
            ControlKind::Controlled(Faction::Alliance),
            ControlKind::Controlled(Faction::Empire),
        ] {
            assert!(!blockaded_under(control, true, true), "{control:?}");
        }
    }

    // FUN_0050b8e0 writes every bit clear for side 0.
    #[test]
    fn a_neutral_system_is_never_blockaded() {
        assert!(!blockaded_under(ControlKind::Uncontrolled, true, false));
        assert!(!blockaded_under(ControlKind::Uncontrolled, false, true));
    }

    // FUN_0050b8e0: a held system is blockaded by the other side's fleet.
    // hyp: an uprising system keeps its holder's side bits.
    #[test]
    fn an_uprising_system_is_blockaded_by_its_holders_enemy() {
        assert!(blockaded_under(
            ControlKind::Uprising(Faction::Empire),
            true,
            false
        ));
        assert!(!blockaded_under(
            ControlKind::Uprising(Faction::Empire),
            false,
            true
        ));
        assert!(blockaded_under(
            ControlKind::Uprising(Faction::Alliance),
            false,
            true
        ));
        assert!(!blockaded_under(
            ControlKind::Uprising(Faction::Alliance),
            true,
            false
        ));
    }

    #[test]
    fn stationed_defending_troop_survives_blockade_start() {
        let (mut world, sys) = make_world();
        let troop = add_troop(&mut world, sys, true); // Alliance troop at Alliance system
        add_fleet(&mut world, sys, false); // Empire blockades

        let mut state = BlockadeState::new();
        let events = BlockadeSystem::advance(&mut state, &world, &[tick(1)]);

        assert!(events.iter().any(|event| matches!(
            event,
            BlockadeEvent::BlockadeStarted { system, .. } if *system == sys
        )));
        assert!(!events.iter().any(|event| matches!(
            event,
            BlockadeEvent::TroopDestroyed { troop: destroyed, .. } if *destroyed == troop
        )));
    }

    // ── Running the blockade (FUN_0050b310, FUN_00504990) ─────────────────

    use crate::movement::MovementState;
    use crate::troop_transport::TroopTransportState;
    use crate::world::{
        CapitalShipClass, DefenseFacilityInstance, FighterClass, FighterEntry, GnprtbEntry,
        GnprtbParams, ShipInstance,
    };

    fn gnprtb_entry(parameter_id: u32, value: i32) -> GnprtbEntry {
        GnprtbEntry {
            parameter_id,
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

    /// An Empire fleet of `ships` transports and `squadrons` fighters, carrying
    /// one Empire regiment, orbits an uncontrolled `origin`. `target` is the
    /// Alliance system from `make_world`, undefended, so the fleet blockades it
    /// on arrival. GNPRTB 7684 = 5 and 7685 = 2 are the shipped values.
    struct Run {
        world: GameWorld,
        origin: SystemKey,
        target: SystemKey,
        fleet: crate::ids::FleetKey,
        troop: TroopKey,
        transport: TroopTransportState,
        state: BlockadeState,
    }

    fn setup_run(ships: usize, squadrons: u32) -> Run {
        let (mut world, target) = make_world();
        world.gnprtb = GnprtbParams::new(vec![
            gnprtb_entry(u32::from(GNPRTB_WITHDRAW_PENALTY_PER_CAPSHIP), 5),
            gnprtb_entry(u32::from(GNPRTB_WITHDRAW_PENALTY_PER_FIGHTER), 2),
        ]);
        let mut origin_system = world.systems[target].clone();
        origin_system.name = "Origin".into();
        origin_system.control = ControlKind::Uncontrolled;
        let origin = world.systems.insert(origin_system);
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            troop_capacity: 1,
            ..CapitalShipClass::default()
        });
        let fighter = world.fighter_classes.insert(FighterClass::default());
        let fleet = world.fleets.insert(Fleet {
            location: origin,
            capital_ships: (0..ships)
                .map(|_| ShipInstance::new(class, 100, false))
                .collect(),
            fighters: vec![FighterEntry {
                class: fighter,
                count: squadrons,
                carrier: 0,
            }],
            characters: vec![],
            is_alliance: false,
            has_death_star: false,
        });
        world.systems[origin].fleets.push(fleet);
        let troop = add_troop(&mut world, origin, false);
        let mut transport = TroopTransportState::default();
        transport.embark(&mut world, fleet, &[troop]).unwrap();
        Run {
            world,
            origin,
            target,
            fleet,
            troop,
            transport,
            state: BlockadeState::new(),
        }
    }

    impl Run {
        /// Observe the regiments with the fleet orbiting its location.
        fn observe_orbiting(&mut self) -> Vec<RunningRegiment> {
            BlockadeSystem::running_regiments(
                &mut self.state,
                &self.world,
                &MovementState::new(),
                &self.transport,
            )
        }

        /// Observe the regiments with the fleet in hyperspace from `from`.
        fn observe_departing(&mut self, from: SystemKey, to: SystemKey) -> Vec<RunningRegiment> {
            let mut movement = MovementState::new();
            assert!(movement.order(self.fleet, from, to, 5));
            self.world.systems[from]
                .fleets
                .retain(|&key| key != self.fleet);
            BlockadeSystem::running_regiments(
                &mut self.state,
                &self.world,
                &movement,
                &self.transport,
            )
        }

        /// Move the fleet into `system`'s orbit.
        fn arrive(&mut self, system: SystemKey) {
            self.world.fleets[self.fleet].location = system;
            self.world.systems[system].fleets.push(self.fleet);
        }

        /// Carry the regiment from the origin into the target and observe it.
        fn carry_into_target(&mut self) {
            assert!(self.observe_orbiting().is_empty());
            assert!(self.observe_departing(self.origin, self.target).is_empty());
            self.arrive(self.target);
            assert!(self.observe_orbiting().is_empty());
        }
    }

    #[test]
    fn a_regiment_carried_into_a_blockaded_system_takes_its_withdraw_percent() {
        // FUN_0050b310 + FUN_0055a020: 100 - 10 ships * 5 - 5 squadrons * 2 = 40,
        // copied into the arriving regiment by FUN_0050c540 -> FUN_00504470.
        let mut run = setup_run(10, 5);
        run.carry_into_target();

        assert_eq!(
            run.state.embarked_regiment(run.troop),
            Some(EmbarkedRegiment {
                orbit: Some(run.target),
                withdraw_percent: 40,
            })
        );
        assert_eq!(
            run.observe_departing(run.target, run.origin),
            vec![RunningRegiment {
                troop: run.troop,
                system: run.target,
                withdraw_percent: 40,
            }]
        );
    }

    #[test]
    fn the_withdraw_percent_never_falls_below_zero() {
        // FUN_0053e120 clamps the FUN_0055a020 result at 0: 100 - 30 * 5 < 0.
        let mut run = setup_run(30, 0);
        run.carry_into_target();
        assert_eq!(
            run.state
                .embarked_regiment(run.troop)
                .map(|r| r.withdraw_percent),
            Some(0)
        );
    }

    #[test]
    fn a_kdy_150_protects_regiments_leaving_a_blockaded_system() {
        // FUN_0050b310 keeps 100 when FUN_00527550 finds an active family-0x22
        // facility (DEFFACSD record 1, KDY-150).
        let mut run = setup_run(10, 5);
        let kdy = run
            .world
            .defense_facilities
            .insert(DefenseFacilityInstance {
                class_dat_id: DatId::new(KDY_150_CLASS_ID),
                side: crate::dat::Faction::Alliance,
            });
        run.world.systems[run.target].defense_facilities.push(kdy);
        run.carry_into_target();

        assert!(run.observe_departing(run.target, run.origin).is_empty());
    }

    #[test]
    fn a_regiment_loaded_at_a_blockaded_system_leaves_it_safely() {
        // Loading onto a fleet does not add the regiment to a new system, so
        // FUN_00508660 never lowers the 100 set when it was activated.
        let mut run = setup_run(10, 5);
        // First seen aboard while already orbiting the blockaded target.
        let (origin, fleet) = (run.origin, run.fleet);
        run.world.systems[origin].fleets.retain(|&key| key != fleet);
        run.arrive(run.target);

        assert!(run.observe_orbiting().is_empty());
        assert_eq!(
            run.state
                .embarked_regiment(run.troop)
                .map(|r| r.withdraw_percent),
            Some(FULL_WITHDRAW_PERCENT)
        );
        assert!(run.observe_departing(run.target, run.origin).is_empty());
    }

    #[test]
    fn landing_resets_a_regiment_to_a_full_withdraw_percent() {
        // Activation resets the percent to 100 (FUN_004f7410 -> FUN_00504960).
        let mut run = setup_run(10, 5);
        run.carry_into_target();
        let landed = run
            .transport
            .disembark_all(&mut run.world, run.fleet, run.target)
            .unwrap();
        assert_eq!(landed, vec![run.troop]);
        assert!(run.observe_orbiting().is_empty());
        assert_eq!(run.state.embarked_regiment(run.troop), None);

        run.transport
            .embark(&mut run.world, run.fleet, &[run.troop])
            .unwrap();
        assert!(run.observe_orbiting().is_empty());
        assert!(run.observe_departing(run.target, run.origin).is_empty());
    }

    #[test]
    fn a_regiment_leaving_an_unblockaded_system_is_never_rolled() {
        let mut run = setup_run(10, 5);
        // An Alliance defender at the target means no blockade.
        add_fleet(&mut run.world, run.target, true);
        run.carry_into_target();
        assert!(run.observe_departing(run.target, run.origin).is_empty());
    }

    #[test]
    fn the_withdraw_roll_keeps_draws_below_the_percent() {
        // FUN_0053e2f0: survive when random(0..=99) < percent. The draw is
        // 99 - floor(roll * 100): 0.605 draws 39, 0.595 draws 40.
        let troop = TroopKey::default();
        let system = SystemKey::default();
        let running = [RunningRegiment {
            troop,
            system,
            withdraw_percent: 40,
        }];
        assert!(BlockadeSystem::resolve_running(&running, &[0.605], 7).is_empty());
        assert_eq!(
            BlockadeSystem::resolve_running(&running, &[0.595], 7),
            vec![BlockadeEvent::TroopDestroyed {
                system,
                troop,
                tick: 7,
            }]
        );
        // The roll pool pads with 1.0, which must never destroy a regiment.
        assert!(BlockadeSystem::resolve_running(&running, &[1.0], 7).is_empty());
        assert!(BlockadeSystem::resolve_running(&running, &[], 7).is_empty());
    }

    #[test]
    fn destroying_a_running_regiment_removes_it_from_cargo_and_the_arena() {
        let mut run = setup_run(1, 0);
        assert!(run.transport.destroy_embarked(&mut run.world, run.troop));
        assert!(run.transport.cargo(run.fleet).is_empty());
        assert!(!run.world.troops.contains_key(run.troop));
        assert!(!run.transport.destroy_embarked(&mut run.world, run.troop));
    }

    // ── Timer 0x386: the blockader's support shift ────────────────────────

    /// The system from `make_world`, held by the side `blockader` fights, at
    /// support `alliance`/`empire` with one `blockader` fleet over it.
    /// GNPRTB 7693-7696 carry the shipped +1, 30, -1, 30.
    fn shift_world(blockader: Faction, alliance: f32, empire: f32) -> (GameWorld, SystemKey) {
        let (mut world, sys) = make_world();
        world.gnprtb = GnprtbParams::new(vec![
            gnprtb_entry(u32::from(GNPRTB_SHIFT_WITH_MAJORITY), 1),
            gnprtb_entry(u32::from(GNPRTB_DELAY_WITH_MAJORITY), 30),
            gnprtb_entry(u32::from(GNPRTB_SHIFT_WITHOUT_MAJORITY), -1),
            gnprtb_entry(u32::from(GNPRTB_DELAY_WITHOUT_MAJORITY), 30),
        ]);
        let held_by = if blockader == Faction::Alliance {
            Faction::Empire
        } else {
            Faction::Alliance
        };
        let held = &mut world.systems[sys];
        held.control = ControlKind::Controlled(held_by);
        held.popularity_alliance = alliance;
        held.popularity_empire = empire;
        add_fleet(&mut world, sys, blockader == Faction::Alliance);
        (world, sys)
    }

    /// The shifts timer `0x386` raises over `ticks`, as (tick, side, delta).
    fn shifts(
        state: &mut BlockadeState,
        world: &GameWorld,
        ticks: std::ops::RangeInclusive<u64>,
    ) -> Vec<(u64, Faction, i32)> {
        ticks
            .flat_map(|n| BlockadeSystem::advance(state, world, &[tick(n)]))
            .filter_map(shift_of)
            .collect()
    }

    fn shift_of(event: BlockadeEvent) -> Option<(u64, Faction, i32)> {
        match event {
            BlockadeEvent::SupportShift {
                side, delta, tick, ..
            } => Some((tick, side, delta)),
            _ => None,
        }
    }

    /// `FUN_0050cb20.c:15-21` adds 7693 when the blockader holds the
    /// majority, and `FUN_005862a0.c:25-30` fires again 7694 days on.
    #[test]
    fn a_blockader_with_the_majority_gains_one_point_every_30_days() {
        for (blockader, alliance) in [(Faction::Empire, 0.3), (Faction::Alliance, 0.7)] {
            let (world, _) = shift_world(blockader, alliance, 1.0 - alliance);
            let mut state = BlockadeState::new();
            assert_eq!(
                shifts(&mut state, &world, 1..=61),
                vec![(31, blockader, 1), (61, blockader, 1)],
                "{blockader:?}"
            );
        }
    }

    /// `FUN_005109f0.c:36-48` rewrites the minimum to 7696 when the blockader
    /// lacks the majority, and the changed value restarts the timer
    /// (`FUN_0053fab0.c:12`): with 7696 = 20 the shifts fall on days 21 and
    /// 41. port: a delay of 0 never arms, at creation or by a rewrite.
    #[test]
    fn a_changed_delay_restarts_the_timer_and_a_zero_delay_never_arms() {
        let params = |with, without| {
            GnprtbParams::new(vec![
                gnprtb_entry(u32::from(GNPRTB_SHIFT_WITH_MAJORITY), 1),
                gnprtb_entry(u32::from(GNPRTB_DELAY_WITH_MAJORITY), with),
                gnprtb_entry(u32::from(GNPRTB_SHIFT_WITHOUT_MAJORITY), -1),
                gnprtb_entry(u32::from(GNPRTB_DELAY_WITHOUT_MAJORITY), without),
            ])
        };
        for (blockader, alliance) in [(Faction::Empire, 0.7), (Faction::Alliance, 0.3)] {
            let (mut world, sys) = shift_world(blockader, alliance, 1.0 - alliance);
            world.gnprtb = params(30, 20);
            let mut state = BlockadeState::new();
            assert_eq!(
                shifts(&mut state, &world, 1..=41),
                vec![(21, blockader, -1), (41, blockader, -1)],
                "{blockader:?}"
            );

            for (with, without) in [(0, 0), (30, 0)] {
                world.gnprtb = params(with, without);
                let mut state = BlockadeState::new();
                assert!(
                    shifts(&mut state, &world, 1..=5).is_empty(),
                    "{blockader:?}"
                );
                assert_eq!(
                    state
                        .support_shift_timer(sys)
                        .and_then(|timer| timer.due_tick),
                    None,
                    "{blockader:?} {with}/{without}"
                );
            }
        }
    }

    /// `FUN_005862a0.c:25-30` adds the delay to the due day, so a frame that
    /// covers several days fires on the due day and keeps the 30-day period.
    #[test]
    fn a_frame_covering_several_days_fires_on_the_due_day() {
        for (blockader, alliance) in [(Faction::Empire, 0.3), (Faction::Alliance, 0.7)] {
            let (world, _) = shift_world(blockader, alliance, 1.0 - alliance);
            let mut state = BlockadeState::new();
            let mut fired = shifts(&mut state, &world, 1..=28);
            let frame: Vec<TickEvent> = (29..=33).map(tick).collect();
            fired.extend(
                BlockadeSystem::advance(&mut state, &world, &frame)
                    .into_iter()
                    .filter_map(shift_of),
            );
            fired.extend(shifts(&mut state, &world, 34..=61));
            assert_eq!(
                fired,
                vec![(31, blockader, 1), (61, blockader, 1)],
                "{blockader:?}"
            );
        }
    }

    /// `FUN_0050cb20.c:15-21` adds 7695 without the majority, every 7696 days
    /// (`FUN_005109f0.c:37-47`).
    #[test]
    fn a_blockader_without_the_majority_loses_one_point_every_30_days() {
        for (blockader, alliance) in [(Faction::Empire, 0.7), (Faction::Alliance, 0.3)] {
            let (world, _) = shift_world(blockader, alliance, 1.0 - alliance);
            let mut state = BlockadeState::new();
            assert_eq!(
                shifts(&mut state, &world, 1..=61),
                vec![(31, blockader, -1), (61, blockader, -1)],
                "{blockader:?}"
            );
        }
    }

    /// `FUN_00509930` returns 3 on a tie, which matches neither blockader.
    #[test]
    fn a_tie_counts_as_no_majority() {
        for blockader in [Faction::Empire, Faction::Alliance] {
            let (world, _) = shift_world(blockader, 0.5, 0.5);
            let mut state = BlockadeState::new();
            assert_eq!(
                shifts(&mut state, &world, 1..=31),
                vec![(31, blockader, -1)],
                "{blockader:?}"
            );
        }
    }

    /// `FUN_00509930` weighs the Alliance's support at `+0x58` against 100
    /// minus it; the port's neutral seeding can leave both fractions equal.
    #[test]
    fn the_majority_weighs_alliance_support_against_its_complement() {
        for (blockader, support) in [(Faction::Alliance, 0.54), (Faction::Empire, 0.46)] {
            let (world, _) = shift_world(blockader, support, support);
            let mut state = BlockadeState::new();
            assert_eq!(
                shifts(&mut state, &world, 1..=31),
                vec![(31, blockader, 1)],
                "{blockader:?}"
            );
        }
    }

    /// Clearing the blockade bit disarms the timer and setting it arms a
    /// full period from that day (`FUN_00511300.c:106`, `FUN_0053fa60`).
    #[test]
    fn ending_the_blockade_disarms_the_shift_and_a_new_one_restarts_it() {
        for blockader in [Faction::Empire, Faction::Alliance] {
            let (mut world, sys) = shift_world(blockader, 0.5, 0.5);
            let mut state = BlockadeState::new();
            assert!(shifts(&mut state, &world, 1..=20).is_empty());
            let fleets = std::mem::take(&mut world.systems[sys].fleets);
            assert!(
                shifts(&mut state, &world, 21..=40).is_empty(),
                "{blockader:?}"
            );
            assert_eq!(
                state
                    .support_shift_timer(sys)
                    .and_then(|timer| timer.due_tick),
                None
            );
            world.systems[sys].fleets = fleets;
            assert_eq!(
                shifts(&mut state, &world, 41..=71),
                vec![(71, blockader, -1)],
                "{blockader:?}"
            );
        }
    }

    /// `FUN_0050c9f0.c:11` shifts only a populated system's support; the
    /// timer still runs.
    #[test]
    fn an_unpopulated_system_takes_no_shift() {
        for blockader in [Faction::Empire, Faction::Alliance] {
            let (mut world, sys) = shift_world(blockader, 0.5, 0.5);
            world.systems[sys].is_populated = false;
            let mut state = BlockadeState::new();
            assert!(
                shifts(&mut state, &world, 1..=61).is_empty(),
                "{blockader:?}"
            );
            assert_eq!(
                state
                    .support_shift_timer(sys)
                    .and_then(|timer| timer.due_tick),
                Some(91)
            );
        }
    }
}
