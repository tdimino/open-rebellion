//! Blockade mechanics: manufacturing halt and troop regiment losses.
//!
//! A blockade occurs when a hostile fleet is present at a system with no
//! defending fleet. This module:
//!
//! 1. **Determines blockaded systems** each tick from fleet disposition.
//! 2. **Emits `BlockadeEvent`s** when blockade state changes (enter/exit).
//! 3. **Halts manufacturing** — `BlockadeState::is_blockaded()` is consulted
//!    by `ManufacturingSystem::advance` to skip queue advancement at blocked systems.
//! 4. **Destroys in-transit troops** — troops being transported to/from a
//!    system while it is blockaded are lost (event `0x340` from RE,
//!    `TroopRegDestroyedRunningBlockade`).
//!
//! # Architecture
//!
//! Follows the stateless advance pattern:
//! ```text
//! BlockadeSystem::advance(&mut BlockadeState, &world, &[TickEvent])
//!     -> Vec<BlockadeEvent>
//! ```
//! The caller applies `BlockadeEvent::TroopDestroyed` to `GameWorld` (removes
//! the `TroopKey` from the system's `ground_units` and from the arena).
//!
//! # Source
//!
//! Ghidra RE: `economy-systems.md §4`.
//! - `FUN_0050e820` (production eligibility): step 8 — fleet active check gates
//!   manufacturing when a hostile blockader is present.
//! - `FUN_005122d0` (`SystemBlockadeNotif`), event `0x14e` (334).
//! - `FUN_00504a00` (`TroopRegDestroyedRunningBlockade`), event `0x340` (832).
//! - `FUN_00512540` (`BattlePendingCausedCurrentBlockade`): hostile fleet arrival
//!   entails both `BattlePending` and `Blockade` simultaneously.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::ids::{SystemKey, TroopKey};
use crate::tick::TickEvent;
use crate::world::GameWorld;
use crate::world::ControlKind;

// ---------------------------------------------------------------------------
// Event IDs (from RE annotated-functions.md)
// ---------------------------------------------------------------------------

/// `SystemBlockadeNotif` event ID — fires when blockade state changes.
pub const EV_BLOCKADE: u32 = 0x14e; // 334

/// `TroopRegDestroyedRunningBlockade` event ID.
pub const EV_TROOP_BLOCKADE_KILL: u32 = 0x340; // 832

// ---------------------------------------------------------------------------
// BlockadeEvent
// ---------------------------------------------------------------------------

/// Events emitted by `BlockadeSystem::advance` each tick.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockadeEvent {
    /// A system has entered a blockaded state (hostile fleet, no defender).
    ///
    /// Manufacturing is halted for the controlling faction until blockade ends.
    /// Corresponds to event `0x14e` (SystemBlockadeNotif) with state = enter.
    BlockadeStarted {
        system: SystemKey,
        tick: u64,
    },

    /// A system's blockade has ended (defending fleet arrived or attacker withdrew).
    ///
    /// Corresponds to event `0x14e` (SystemBlockadeNotif) with state = exit.
    BlockadeEnded {
        system: SystemKey,
        tick: u64,
    },

    /// A troop regiment was destroyed while being transported through a blockaded system.
    ///
    /// From RE: `FUN_00504a00`, event `0x340`. Troops in transit through a blockaded
    /// system are destroyed, not captured. The caller must remove `troop` from
    /// `GameWorld::troops` and from the system's `ground_units` list.
    TroopDestroyed {
        system: SystemKey,
        troop: TroopKey,
        tick: u64,
    },
}

impl BlockadeEvent {
    pub fn tick(&self) -> u64 {
        match self {
            BlockadeEvent::BlockadeStarted { tick, .. } => *tick,
            BlockadeEvent::BlockadeEnded { tick, .. } => *tick,
            BlockadeEvent::TroopDestroyed { tick, .. } => *tick,
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
    blockaded: HashSet<SystemKey>,
}

impl BlockadeState {
    pub fn new() -> Self {
        BlockadeState {
            blockaded: HashSet::new(),
        }
    }

    /// Returns `true` if `system` is currently blockaded.
    ///
    /// Called by `ManufacturingSystem::advance` to halt production.
    pub fn is_blockaded(&self, system: SystemKey) -> bool {
        self.blockaded.contains(&system)
    }

    /// All currently blockaded systems.
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
    /// - For newly blockaded systems: inspect `ground_units` for faction
    ///   mismatch (troops belonging to the defending faction while a hostile
    ///   blockade is active) and emit `TroopDestroyed` for each.
    ///
    /// Does **not** mutate `GameWorld` — returns events for the caller to apply.
    /// Skips zero-tick frames.
    pub fn advance(
        state: &mut BlockadeState,
        world: &GameWorld,
        tick_events: &[TickEvent],
    ) -> Vec<BlockadeEvent> {
        if tick_events.is_empty() {
            return Vec::new();
        }

        let tick = tick_events.last().unwrap().tick;
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

        // New blockades (entered this tick)
        for &sys_key in &now_blockaded {
            if !state.blockaded.contains(&sys_key) {
                events.push(BlockadeEvent::BlockadeStarted { system: sys_key, tick });

                // Destroy in-transit troops belonging to the defending faction.
                // "In transit" = ground_units at the blockaded system that belong to
                // the faction being blockaded (faction mismatch with blockader).
                if let Some(sys) = world.systems.get(sys_key) {
                    let blockader_is_alliance = Self::blockader_is_alliance(world, sys);
                    for &troop_key in &sys.ground_units {
                        if let Some(troop) = world.troops.get(troop_key) {
                            // Troop is caught in blockade if it belongs to the faction
                            // being blockaded (i.e., same faction as the system's controller,
                            // opposite of the blockader).
                            let troop_is_target = troop.is_alliance != blockader_is_alliance;
                            // Only destroy troops in transit (regiment_strength > 0 but
                            // being transported — for now all ground_units at a newly
                            // blockaded system qualify per RE FUN_00504a00 semantics).
                            if troop_is_target {
                                events.push(BlockadeEvent::TroopDestroyed {
                                    system: sys_key,
                                    troop: troop_key,
                                    tick,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Cleared blockades (ended this tick)
        for &sys_key in &state.blockaded {
            if !now_blockaded.contains(&sys_key) {
                events.push(BlockadeEvent::BlockadeEnded { system: sys_key, tick });
            }
        }

        state.blockaded = now_blockaded;
        events
    }

    // ── Private ───────────────────────────────────────────────────────────

    /// A system is blockaded if it has at least one hostile fleet AND zero
    /// defending fleets.
    ///
    /// "Hostile" is relative to the system's `controlling_faction`. A neutral
    /// system cannot be blockaded (no faction to defend it).
    fn system_is_blockaded(world: &GameWorld, sys: &crate::world::System) -> bool {
        use crate::dat::Faction;

        let controlling = match sys.control {
            ControlKind::Controlled(Faction::Alliance) => true,
            ControlKind::Controlled(Faction::Empire) => false,
            _ => return false, // neutral or contested — no blockade logic
        };

        let hostile_count = sys
            .fleets
            .iter()
            .filter_map(|&fk| world.fleets.get(fk))
            .filter(|f| f.is_alliance != controlling)
            .count();

        let defending_count = sys
            .fleets
            .iter()
            .filter_map(|&fk| world.fleets.get(fk))
            .filter(|f| f.is_alliance == controlling)
            .count();

        hostile_count > 0 && defending_count == 0
    }

    /// Returns `true` if the blockading force is Alliance (used for troop
    /// destruction: Alliance blockader kills Empire troops and vice versa).
    fn blockader_is_alliance(_world: &GameWorld, sys: &crate::world::System) -> bool {
        use crate::dat::Faction;

        // The blockader is the faction opposite the system controller.
        match sys.control {
            ControlKind::Controlled(Faction::Alliance) => false, // Alliance-controlled → Empire blockades
            ControlKind::Controlled(Faction::Empire) => true,    // Empire-controlled → Alliance blockades
            _ => false,
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
        let (world, _sys) = make_world();
        let mut state = BlockadeState::new();
        let events = BlockadeSystem::advance(&mut state, &world, &[]);
        assert!(events.is_empty());
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
        add_fleet(&mut world, sys, true);   // Alliance defender
        add_fleet(&mut world, sys, false);  // Empire hostile

        let mut state = BlockadeState::new();
        let events = BlockadeSystem::advance(&mut state, &world, &[tick(1)]);
        // Defender present — no blockade
        assert!(!events.iter().any(|e| matches!(e, BlockadeEvent::BlockadeStarted { .. })));
        assert!(!state.is_blockaded(sys));
    }

    #[test]
    fn blockade_starts_when_hostile_fleet_uncontested() {
        let (mut world, sys) = make_world();
        add_fleet(&mut world, sys, false); // Empire fleet at Alliance-controlled system

        let mut state = BlockadeState::new();
        let events = BlockadeSystem::advance(&mut state, &world, &[tick(1)]);
        assert!(events.iter().any(|e| matches!(e, BlockadeEvent::BlockadeStarted { .. })));
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
        assert!(events.iter().any(|e| matches!(e, BlockadeEvent::BlockadeEnded { .. })));
        assert!(!state.is_blockaded(sys));
    }

    #[test]
    fn troop_destroyed_on_blockade_start() {
        let (mut world, sys) = make_world();
        let troop = add_troop(&mut world, sys, true); // Alliance troop at Alliance system
        add_fleet(&mut world, sys, false); // Empire blockades

        let mut state = BlockadeState::new();
        let events = BlockadeSystem::advance(&mut state, &world, &[tick(1)]);

        let destroyed: Vec<_> = events
            .iter()
            .filter(|e| matches!(e, BlockadeEvent::TroopDestroyed { .. }))
            .collect();
        assert_eq!(destroyed.len(), 1);
        if let BlockadeEvent::TroopDestroyed { troop: tk, .. } = destroyed[0] {
            assert_eq!(*tk, troop);
        }
    }

    #[test]
    fn enemy_troop_not_destroyed_by_own_blockade() {
        let (mut world, sys) = make_world();
        // Empire troop at Alliance-controlled system — Alliance does not destroy Empire troops
        // via blockade (they're already on the surface, not in transit to help the system)
        add_troop(&mut world, sys, false); // Empire troop
        add_fleet(&mut world, sys, false);  // Empire fleet blockades

        let mut state = BlockadeState::new();
        let events = BlockadeSystem::advance(&mut state, &world, &[tick(1)]);
        // Empire troop not destroyed (same faction as blockader, not in transit for defender)
        assert!(!events.iter().any(|e| matches!(e, BlockadeEvent::TroopDestroyed { .. })));
    }

    #[test]
    fn is_blockaded_reflects_current_state() {
        let (mut world, sys) = make_world();
        add_fleet(&mut world, sys, false);

        let mut state = BlockadeState::new();
        assert!(!state.is_blockaded(sys));
        BlockadeSystem::advance(&mut state, &world, &[tick(1)]);
        assert!(state.is_blockaded(sys));
    }
}
