//! Death Star construction, movement tracking, and planet destruction.
//!
//! Three mechanics in one module:
//!
//! 1. **Construction** — tracks remaining build ticks at a system; emits
//!    `DeathStarEvent::ConstructionCompleted` when done.  The caller then
//!    sets `Fleet::has_death_star = true` on the fleet at that system.
//!
//! 2. **Movement** — Death Star fleet movement reuses `MovementState`
//!    from `movement.rs`.  The Death Star module tracks the active orbital
//!    location for the `VictorySystem` and for the nearby-warning logic.
//!
//! 3. **Planet destruction** — `DeathStarSystem::fire()` checks the preconditions
//!    from `FUN_005617b0` / `FUN_0055f650`:
//!    - Target system is not already destroyed (`!system.is_destroyed`).
//!    - Death Star fleet is present at that system.
//!    - Target is enemy-controlled (Empire Death Star → non-Empire system).
//!    On success emits `PlanetDestroyed`.  Caller sets `system.is_destroyed = true`.
//!
//! # Advance contract
//! `DeathStarSystem::advance()` never mutates `GameWorld`.
//! It returns `Vec<DeathStarEvent>` for the caller to apply.
//!
//! # Source
//! - `ghidra/notes/annotated-functions.md` § FUN_005617b0
//! - `ghidra/notes/economy-systems.md` § SystemDeathStarNearbyNotif
//! - `ghidra/notes/rust-implementation-guide.md` §3.3, §2.4
//! - `entity-system.md §4.2` — alive_flag inverted semantics for systems

use serde::{Deserialize, Serialize};

use crate::dat::Faction;
use crate::ids::{FleetKey, SystemKey};
use crate::tick::TickEvent;
use crate::world::GameWorld;
use crate::world::ControlKind;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Default construction duration in game-days.
///
/// Placeholder — the exact GNPRTB parameter for Death Star construction time
/// has not been isolated from REBEXE.EXE.  1825 = ~5 in-game years at 365 days.
pub const DEATH_STAR_CONSTRUCTION_TICKS: u32 = 1825;

/// Threshold distance (in system-coordinate units) for the
/// `SystemDeathStarNearbyNotif` warning.
///
/// From `FUN_00512480` (51 lines) via economy-systems.md — exact threshold
/// value not yet decompiled; 300 units is a reasonable approximation.
pub const NEARBY_WARNING_RADIUS: u32 = 300;

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

/// Events produced by `DeathStarSystem::advance()` and `DeathStarSystem::fire()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeathStarEvent {
    /// Death Star construction finished at `system`.
    ///
    /// Caller should set `Fleet::has_death_star = true` on the fleet at this
    /// system and update `VictoryState::death_star_active = true`.
    ConstructionCompleted {
        system: SystemKey,
        tick: u64,
    },

    /// The Death Star superlaser fired and destroyed `system`.
    ///
    /// Caller must set `world.systems[system].is_destroyed = true` and update
    /// `VictoryState::death_star_location = Some(system)`.
    PlanetDestroyed {
        system: SystemKey,
        tick: u64,
    },

    /// A Death Star fleet is within `NEARBY_WARNING_RADIUS` of `system`.
    ///
    /// Maps to `SystemDeathStarNearbyNotif` (FUN_00512480).
    /// Used to trigger Alliance intelligence messages.
    NearbyWarning {
        system: SystemKey,
        tick: u64,
    },
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

/// Persistent Death Star construction state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeathStarConstruction {
    /// System where the construction yard is active.
    pub system: SystemKey,
    /// Game-day ticks remaining until completion.
    pub ticks_remaining: u32,
}

/// Persistent Death Star simulation state.
///
/// Held in `main.rs` alongside other simulation states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeathStarState {
    /// Active construction project, if any.
    pub under_construction: Option<DeathStarConstruction>,
    /// The fleet key of the active Death Star, if constructed and deployed.
    pub death_star_fleet: Option<FleetKey>,
}

impl Default for DeathStarState {
    fn default() -> Self {
        DeathStarState {
            under_construction: None,
            death_star_fleet: None,
        }
    }
}

impl DeathStarState {
    /// Add construction delay from sabotage (increases ticks_remaining).
    pub fn add_sabotage_delay(&mut self, ticks: u32) {
        if let Some(ref mut construction) = self.under_construction {
            construction.ticks_remaining = construction.ticks_remaining.saturating_add(ticks);
        }
    }
}

// ---------------------------------------------------------------------------
// System
// ---------------------------------------------------------------------------

/// Stateless Death Star simulation system.
pub struct DeathStarSystem;

impl DeathStarSystem {
    /// Advance Death Star construction and emit nearby-warning events.
    ///
    /// Called each tick by the main loop.  Returns events for the caller to apply.
    ///
    /// # What it does
    /// 1. Decrements `under_construction.ticks_remaining` by the number of ticks
    ///    that elapsed.  When it reaches 0, emits `ConstructionCompleted`.
    /// 2. If `death_star_fleet` is set and present in the world, scans all Alliance
    ///    systems within `NEARBY_WARNING_RADIUS` and emits `NearbyWarning`.
    pub fn advance(
        state: &mut DeathStarState,
        world: &GameWorld,
        tick_events: &[TickEvent],
    ) -> Vec<DeathStarEvent> {
        let mut events = Vec::new();

        if tick_events.is_empty() {
            return events;
        }

        let tick_count = tick_events.len() as u32;
        let last_tick = tick_events.last().unwrap().tick;

        // --- 1. Construction countdown ---
        if let Some(ref mut construction) = state.under_construction {
            construction.ticks_remaining =
                construction.ticks_remaining.saturating_sub(tick_count);
            if construction.ticks_remaining == 0 {
                events.push(DeathStarEvent::ConstructionCompleted {
                    system: construction.system,
                    tick: last_tick,
                });
                // Self-clear to prevent re-emitting ConstructionCompleted every tick.
                state.under_construction = None;
            }
        }

        // --- 2. Nearby-warning scan ---
        if let Some(fleet_key) = state.death_star_fleet {
            if let Some(fleet) = world.fleets.get(fleet_key) {
                let ds_system = fleet.location;
                if let Some(ds_sys) = world.systems.get(ds_system) {
                    let ds_x = ds_sys.x as i32;
                    let ds_y = ds_sys.y as i32;

                    for (sys_key, sys) in world.systems.iter() {
                        // Only warn about Alliance-controlled systems.
                        if sys.control != ControlKind::Controlled(Faction::Alliance) {
                            continue;
                        }
                        if sys_key == ds_system {
                            continue; // already at this system
                        }
                        let dx = sys.x as i32 - ds_x;
                        let dy = sys.y as i32 - ds_y;
                        let dist_sq = (dx * dx + dy * dy) as u64;
                        let radius_sq = (NEARBY_WARNING_RADIUS as u64).pow(2);
                        if dist_sq <= radius_sq {
                            events.push(DeathStarEvent::NearbyWarning {
                                system: sys_key,
                                tick: last_tick,
                            });
                        }
                    }
                }
            }
        }

        events
    }

    /// Attempt to fire the Death Star superlaser at `target_system`.
    ///
    /// Mirrors `FUN_005617b0` + `FUN_0055f650` precondition checks:
    /// - Target must not already be destroyed (`!system.is_destroyed`).
    /// - An Empire Death Star fleet must be present at `target_system`.
    /// - Target must not be Empire-controlled (no self-destruction).
    ///
    /// Returns `Some(PlanetDestroyed)` on success; `None` if preconditions fail.
    /// The caller must apply `world.systems[target].is_destroyed = true` and
    /// update `VictoryState` after receiving this event.
    pub fn fire(
        world: &GameWorld,
        target_system: SystemKey,
        tick: u64,
    ) -> Option<DeathStarEvent> {
        let sys = world.systems.get(target_system)?;

        // Guard: already destroyed.
        if sys.is_destroyed {
            return None;
        }

        // Guard: no self-destruction — Empire Death Star cannot fire on Empire systems.
        if sys.control.is_controlled_by(Faction::Empire) {
            return None;
        }

        // Guard: a Death Star fleet must be present at this system.
        let has_ds = sys
            .fleets
            .iter()
            .filter_map(|&fk| world.fleets.get(fk))
            .any(|f| !f.is_alliance && f.has_death_star);

        if !has_ds {
            return None;
        }

        Some(DeathStarEvent::PlanetDestroyed {
            system: target_system,
            tick,
        })
    }

    /// Start a new construction project at `system`.
    ///
    /// Returns `false` (no-op) if construction is already underway.
    pub fn start_construction(
        state: &mut DeathStarState,
        system: SystemKey,
    ) -> bool {
        if state.under_construction.is_some() {
            return false;
        }
        state.under_construction = Some(DeathStarConstruction {
            system,
            ticks_remaining: DEATH_STAR_CONSTRUCTION_TICKS,
        });
        true
    }

    /// Clear a completed construction project after the caller has handled the
    /// `ConstructionCompleted` event.
    pub fn clear_construction(state: &mut DeathStarState) {
        state.under_construction = None;
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
    use crate::world::{Fleet, GameWorld, Sector, System};

    fn tick(n: u64) -> TickEvent {
        TickEvent { tick: n }
    }

    fn make_world() -> (GameWorld, SystemKey) {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(0x9200_0000),
            name: "Outer Rim".into(),
            group: SectorGroup::RimOuter,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let sys = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0000),
            name: "Alderaan".into(),
            sector,
            x: 100,
            y: 100,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.9,
            popularity_empire: 0.1,
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

    fn add_ds_fleet(world: &mut GameWorld, sys: SystemKey) -> FleetKey {
        let fk = world.fleets.insert(Fleet {
            location: sys,
            capital_ships: vec![],
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            has_death_star: true,
        });
        world.systems.get_mut(sys).unwrap().fleets.push(fk);
        fk
    }

    // ── Construction tests ───────────────────────────────────────────────────

    #[test]
    fn test_construction_countdown_completes() {
        let (world, sys) = make_world();
        let mut state = DeathStarState::default();
        DeathStarSystem::start_construction(&mut state, sys);

        // Advance almost to completion.
        let almost = DEATH_STAR_CONSTRUCTION_TICKS - 1;
        let ticks: Vec<TickEvent> = (1..=almost as u64).map(tick).collect();
        let events = DeathStarSystem::advance(&mut state, &world, &ticks);
        assert!(events.is_empty(), "should not complete yet");
        assert_eq!(state.under_construction.as_ref().unwrap().ticks_remaining, 1);

        // Final tick.
        let events = DeathStarSystem::advance(&mut state, &world, &[tick(DEATH_STAR_CONSTRUCTION_TICKS as u64)]);
        assert!(
            events.iter().any(|e| matches!(e, DeathStarEvent::ConstructionCompleted { .. })),
            "expected ConstructionCompleted"
        );
    }

    #[test]
    fn test_construction_no_double_start() {
        let (_, sys) = make_world();
        let mut state = DeathStarState::default();
        assert!(DeathStarSystem::start_construction(&mut state, sys));
        assert!(!DeathStarSystem::start_construction(&mut state, sys),
            "second start_construction must return false");
    }

    #[test]
    fn test_clear_construction() {
        let (_, sys) = make_world();
        let mut state = DeathStarState::default();
        DeathStarSystem::start_construction(&mut state, sys);
        DeathStarSystem::clear_construction(&mut state);
        assert!(state.under_construction.is_none());
    }

    #[test]
    fn test_no_events_without_ticks() {
        let (world, sys) = make_world();
        let mut state = DeathStarState::default();
        DeathStarSystem::start_construction(&mut state, sys);
        let events = DeathStarSystem::advance(&mut state, &world, &[]);
        assert!(events.is_empty());
    }

    // ── Planet destruction tests ─────────────────────────────────────────────

    #[test]
    fn test_fire_succeeds_on_valid_target() {
        let (mut world, sys) = make_world();
        add_ds_fleet(&mut world, sys);

        let evt = DeathStarSystem::fire(&world, sys, 42);
        assert!(
            matches!(evt, Some(DeathStarEvent::PlanetDestroyed { system, tick: 42 }) if system == sys),
            "expected PlanetDestroyed"
        );
    }

    #[test]
    fn test_fire_blocked_already_destroyed() {
        let (mut world, sys) = make_world();
        add_ds_fleet(&mut world, sys);
        world.systems.get_mut(sys).unwrap().is_destroyed = true;

        assert!(DeathStarSystem::fire(&world, sys, 1).is_none());
    }

    #[test]
    fn test_fire_blocked_no_death_star_fleet() {
        let (mut world, sys) = make_world();
        // Add normal (non-Death Star) Empire fleet.
        let fk = world.fleets.insert(Fleet {
            location: sys,
            capital_ships: vec![],
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            has_death_star: false,
        });
        world.systems.get_mut(sys).unwrap().fleets.push(fk);

        assert!(DeathStarSystem::fire(&world, sys, 1).is_none());
    }

    #[test]
    fn test_fire_blocked_empire_controlled() {
        let (mut world, sys) = make_world();
        world.systems.get_mut(sys).unwrap().control = ControlKind::Controlled(Faction::Empire);
        add_ds_fleet(&mut world, sys);

        assert!(DeathStarSystem::fire(&world, sys, 1).is_none(),
            "Death Star must not fire on Empire-controlled systems");
    }

    // ── Nearby-warning tests ─────────────────────────────────────────────────

    #[test]
    fn test_nearby_warning_emitted_for_close_system() {
        let (mut world, ds_sys) = make_world();
        // Add a second Alliance system within radius.
        let sector = world.systems[ds_sys].sector;
        let nearby_sys = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0001),
            name: "Tatooine".into(),
            sector,
            x: 100 + NEARBY_WARNING_RADIUS as u16 / 2,
            y: 100,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.6,
            popularity_empire: 0.4,
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
        let _ = nearby_sys;

        let fleet_key = add_ds_fleet(&mut world, ds_sys);
        let mut state = DeathStarState {
            under_construction: None,
            death_star_fleet: Some(fleet_key),
        };

        let events = DeathStarSystem::advance(&mut state, &world, &[tick(1)]);
        assert!(
            events.iter().any(|e| matches!(e, DeathStarEvent::NearbyWarning { .. })),
            "expected NearbyWarning for close Alliance system"
        );
    }

    #[test]
    fn test_no_nearby_warning_for_distant_system() {
        let (mut world, ds_sys) = make_world();
        let sector = world.systems[ds_sys].sector;
        // Place a system far outside the warning radius.
        world.systems.insert(System {
            dat_id: DatId::new(0x9000_0001),
            name: "Distant World".into(),
            sector,
            x: 100 + NEARBY_WARNING_RADIUS as u16 * 2,
            y: 100,
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

        let fleet_key = add_ds_fleet(&mut world, ds_sys);
        let mut state = DeathStarState {
            under_construction: None,
            death_star_fleet: Some(fleet_key),
        };

        let events = DeathStarSystem::advance(&mut state, &world, &[tick(1)]);
        let warning_count = events
            .iter()
            .filter(|e| matches!(e, DeathStarEvent::NearbyWarning { .. }))
            .count();
        assert_eq!(warning_count, 0, "distant system should not trigger NearbyWarning");
    }
}
