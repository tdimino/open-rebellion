//! Victory condition detection.
//!
//! Checks for game-ending states each tick:
//! - **HQ capture**: the opposing faction controls the enemy headquarters system
//!   (enemy fleet present, defending fleet absent).
//! - **Death Star fires on Alliance HQ**: `System::is_destroyed` is set at the
//!   Alliance HQ while a Death Star fleet is present — Empire wins.
//! - **Death Star destroyed**: Death Star fleet is gone from its last known
//!   location — Alliance wins.
//!
//! # Architecture
//!
//! Follows the stateless advance pattern:
//! ```text
//! VictorySystem::check(&VictoryState, &world, &[TickEvent]) -> Option<VictoryOutcome>
//! ```
//! Returns `None` every tick until a terminal condition is met, then returns
//! `Some(VictoryOutcome)`. The caller sets `VictoryState::resolved = true` to
//! suppress repeated checks after the first outcome.
//!
//! # Source
//!
//! Ghidra RE: `entity-system.md §4.2` — `SideVictoryConditionsNotif`,
//! `FinalBattle` (`FUN_0054ba00`), event IDs `0x12c`/`0x180`.
//! `IsHeadquarters` flag → `System::is_headquarters`.
//! Death Star: family `0x34`; fires when `alive_flag` bit0 == 0 (INVERTED).

use serde::{Deserialize, Serialize};

use crate::dat::Faction;
use crate::ids::SystemKey;
use crate::tick::TickEvent;
use crate::world::GameWorld;

// ---------------------------------------------------------------------------
// VictoryOutcome
// ---------------------------------------------------------------------------

/// Terminal game states returned by `VictorySystem::check`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VictoryOutcome {
    /// A faction captured the enemy's headquarters system.
    ///
    /// `winner` controls `hq_system`; `loser` has lost their command center.
    HqCaptured {
        winner: Faction,
        loser: Faction,
        hq_system: SystemKey,
    },

    /// The Death Star fired on the Rebel base / Alliance HQ.
    ///
    /// The Empire wins: `target_system` has been destroyed.
    DeathStarVictory { target_system: SystemKey },

    /// The Rebels destroyed the Death Star.
    ///
    /// Alliance wins. `location` is the system where it was destroyed.
    DeathStarDestroyed { location: SystemKey },
}

// ---------------------------------------------------------------------------
// VictoryState
// ---------------------------------------------------------------------------

/// Configuration for victory detection — set at game start.
///
/// Stores the HQ system for each faction and tracks Death Star status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VictoryState {
    /// The Alliance headquarters system key.
    pub alliance_hq: SystemKey,
    /// The Empire headquarters system key.
    pub empire_hq: SystemKey,
    /// When `true`, Death Star win-condition checks are active.
    pub death_star_active: bool,
    /// The system the Death Star is currently orbiting (if active).
    pub death_star_location: Option<SystemKey>,
    /// Set `true` once a `VictoryOutcome` has been returned to suppress
    /// re-firing on subsequent ticks.
    pub resolved: bool,
}

impl VictoryState {
    pub fn new(alliance_hq: SystemKey, empire_hq: SystemKey) -> Self {
        VictoryState {
            alliance_hq,
            empire_hq,
            death_star_active: false,
            death_star_location: None,
            resolved: false,
        }
    }
}

// ---------------------------------------------------------------------------
// VictorySystem
// ---------------------------------------------------------------------------

/// Stateless victory-condition evaluator.
pub struct VictorySystem;

impl VictorySystem {
    /// Evaluate all victory conditions against the current world state.
    ///
    /// Returns `Some(VictoryOutcome)` the first tick a terminal condition is
    /// detected; `None` otherwise. Skips zero-tick frames and already-resolved
    /// games. The caller must set `state.resolved = true` after acting on
    /// a result.
    /// Minimum game tick before victory conditions are evaluated.
    /// Gives both factions time to build forces and prevents instant wins
    /// from aggressive early fleet deployment. In the original game, fleet
    /// transit across the galaxy takes many turns; this serves the same purpose.
    pub const MIN_VICTORY_TICK: u64 = 200;

    pub fn check(
        state: &VictoryState,
        world: &GameWorld,
        tick_events: &[TickEvent],
    ) -> Option<VictoryOutcome> {
        if tick_events.is_empty() || state.resolved {
            return None;
        }

        // Grace period: don't check victory until the game has had time to develop
        let current_tick = tick_events.last().map(|e| e.tick).unwrap_or(0);
        if current_tick < Self::MIN_VICTORY_TICK {
            return None;
        }

        // Death Star check comes first: planet destruction is the terminal event
        // that supersedes any fleet-presence HQ check.
        if state.death_star_active {
            if let Some(outcome) = Self::check_death_star(state, world) {
                return Some(outcome);
            }
        }

        if let Some(outcome) = Self::check_hq_capture(state, world) {
            return Some(outcome);
        }

        None
    }

    // ── Private ───────────────────────────────────────────────────────────

    /// HQ capture: enemy fleet present AND defending fleet absent.
    fn check_hq_capture(state: &VictoryState, world: &GameWorld) -> Option<VictoryOutcome> {
        // Alliance HQ → Empire capture?
        if let Some(sys) = world.systems.get(state.alliance_hq) {
            let empire = Self::fleet_count_at(world, sys, false);
            let alliance = Self::fleet_count_at(world, sys, true);
            if empire > 0 && alliance == 0 {
                return Some(VictoryOutcome::HqCaptured {
                    winner: Faction::Empire,
                    loser: Faction::Alliance,
                    hq_system: state.alliance_hq,
                });
            }
        }

        // Empire HQ → Alliance capture?
        if let Some(sys) = world.systems.get(state.empire_hq) {
            let alliance = Self::fleet_count_at(world, sys, true);
            let empire = Self::fleet_count_at(world, sys, false);
            if alliance > 0 && empire == 0 {
                return Some(VictoryOutcome::HqCaptured {
                    winner: Faction::Alliance,
                    loser: Faction::Empire,
                    hq_system: state.empire_hq,
                });
            }
        }

        None
    }

    /// Death Star fire and destruction conditions.
    ///
    /// Sub-condition 1: Death Star at Alliance HQ and planet is destroyed
    /// (`is_destroyed` set by the caller after combat resolution) → Empire wins.
    ///
    /// Sub-condition 2: No Empire Death Star fleet at `death_star_location`
    /// → assume it was destroyed in combat → Alliance wins.
    fn check_death_star(state: &VictoryState, world: &GameWorld) -> Option<VictoryOutcome> {
        let location = state.death_star_location?;

        // Sub-condition 1: Death Star destroyed Alliance HQ planet
        if location == state.alliance_hq {
            if world
                .systems
                .get(state.alliance_hq)
                .map(|s| s.is_destroyed)
                .unwrap_or(false)
            {
                return Some(VictoryOutcome::DeathStarVictory {
                    target_system: state.alliance_hq,
                });
            }
        }

        // Sub-condition 2: Death Star fleet gone from last known location
        let has_ds_fleet = world
            .systems
            .get(location)
            .map(|sys| {
                sys.fleets
                    .iter()
                    .filter_map(|&fk| world.fleets.get(fk))
                    .any(|f| !f.is_alliance && f.has_death_star)
            })
            .unwrap_or(false);

        if !has_ds_fleet {
            return Some(VictoryOutcome::DeathStarDestroyed { location });
        }

        None
    }

    fn fleet_count_at(
        world: &GameWorld,
        sys: &crate::world::System,
        is_alliance: bool,
    ) -> usize {
        sys.fleets
            .iter()
            .filter_map(|&fk| world.fleets.get(fk))
            .filter(|f| f.is_alliance == is_alliance)
            .count()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dat::ExplorationStatus;
    use crate::ids::DatId;
    use crate::tick::TickEvent;
    use crate::world::{Fleet, GameWorld, Sector, System};
    use crate::dat::SectorGroup;

    fn tick(n: u64) -> TickEvent {
        TickEvent { tick: n }
    }

    /// Build a minimal world with one sector and two systems.
    fn make_world() -> (GameWorld, SystemKey, SystemKey) {
        let mut world = GameWorld::default();

        let sector_key = world.sectors.insert(Sector {
            dat_id: DatId::new(0x9200_0000),
            name: "Test Sector".into(),
            group: SectorGroup::RimOuter,
            x: 0,
            y: 0,
            systems: vec![],
        });

        let a_hq = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0000),
            name: "Yavin IV".into(),
            sector: sector_key,
            x: 100,
            y: 200,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.8,
            popularity_empire: 0.2,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: true,
            is_destroyed: false,
            controlling_faction: Some(Faction::Alliance),
        });

        let e_hq = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0001),
            name: "Coruscant".into(),
            sector: sector_key,
            x: 500,
            y: 500,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.1,
            popularity_empire: 0.9,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: true,
            is_destroyed: false,
            controlling_faction: Some(Faction::Empire),
        });

        (world, a_hq, e_hq)
    }

    fn add_fleet(world: &mut GameWorld, system: SystemKey, is_alliance: bool, has_death_star: bool) -> crate::ids::FleetKey {
        let fk = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![],
            fighters: vec![],
            characters: vec![],
            is_alliance,
            has_death_star,
        });
        world.systems.get_mut(system).unwrap().fleets.push(fk);
        fk
    }

    #[test]
    fn no_outcome_without_ticks() {
        let (world, a, e) = make_world();
        let state = VictoryState::new(a, e);
        assert!(VictorySystem::check(&state, &world, &[]).is_none());
    }

    #[test]
    fn no_outcome_when_resolved() {
        let (world, a, e) = make_world();
        let mut state = VictoryState::new(a, e);
        state.resolved = true;
        assert!(VictorySystem::check(&state, &world, &[tick(VictorySystem::MIN_VICTORY_TICK)]).is_none());
    }

    #[test]
    fn no_outcome_no_fleets() {
        let (world, a, e) = make_world();
        let state = VictoryState::new(a, e);
        assert!(VictorySystem::check(&state, &world, &[tick(VictorySystem::MIN_VICTORY_TICK)]).is_none());
    }

    #[test]
    fn empire_captures_alliance_hq() {
        let (mut world, a, e) = make_world();
        add_fleet(&mut world, a, false, false);

        let state = VictoryState::new(a, e);
        let out = VictorySystem::check(&state, &world, &[tick(VictorySystem::MIN_VICTORY_TICK)]);
        assert!(matches!(
            out,
            Some(VictoryOutcome::HqCaptured { winner: Faction::Empire, loser: Faction::Alliance, .. })
        ));
    }

    #[test]
    fn alliance_captures_empire_hq() {
        let (mut world, a, e) = make_world();
        add_fleet(&mut world, e, true, false);

        let state = VictoryState::new(a, e);
        let out = VictorySystem::check(&state, &world, &[tick(VictorySystem::MIN_VICTORY_TICK)]);
        assert!(matches!(
            out,
            Some(VictoryOutcome::HqCaptured { winner: Faction::Alliance, loser: Faction::Empire, .. })
        ));
    }

    #[test]
    fn contested_hq_no_capture() {
        let (mut world, a, e) = make_world();
        add_fleet(&mut world, a, false, false);
        add_fleet(&mut world, a, true, false);

        let state = VictoryState::new(a, e);
        assert!(VictorySystem::check(&state, &world, &[tick(VictorySystem::MIN_VICTORY_TICK)]).is_none());
    }

    #[test]
    fn death_star_destroys_alliance_hq() {
        let (mut world, a, e) = make_world();
        world.systems.get_mut(a).unwrap().is_destroyed = true;
        add_fleet(&mut world, a, false, true);

        let mut state = VictoryState::new(a, e);
        state.death_star_active = true;
        state.death_star_location = Some(a);

        let out = VictorySystem::check(&state, &world, &[tick(VictorySystem::MIN_VICTORY_TICK)]);
        assert!(matches!(out, Some(VictoryOutcome::DeathStarVictory { .. })));
    }

    #[test]
    fn death_star_destroyed_alliance_wins() {
        let (world, a, e) = make_world();
        // Death Star fleet was at `a` but has been removed — no ds fleet at location

        let mut state = VictoryState::new(a, e);
        state.death_star_active = true;
        state.death_star_location = Some(a);

        let out = VictorySystem::check(&state, &world, &[tick(VictorySystem::MIN_VICTORY_TICK)]);
        assert!(matches!(out, Some(VictoryOutcome::DeathStarDestroyed { .. })));
    }

    #[test]
    fn death_star_inactive_no_check() {
        let (world, a, e) = make_world();
        let state = VictoryState::new(a, e);
        // death_star_active = false by default
        assert!(VictorySystem::check(&state, &world, &[tick(VictorySystem::MIN_VICTORY_TICK)]).is_none());
    }
}
