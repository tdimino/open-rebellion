//! Victory condition detection.
//!
//! Checks for game-ending states each tick:
//! - **Standard HQ victory**: the opposing faction controls the enemy
//!   headquarters and holds its two principal leaders captive.
//! - **Headquarters Only victory**: headquarters capture is sufficient.
//! - **Death Star fires on Alliance HQ**: `System::is_destroyed` is set at the
//!   Alliance HQ while a Death Star fleet is present — Empire wins.
//! - **Death Star destroyed**: Death Star fleet is gone from its last known
//!   location — Alliance wins.
//!
//! # Architecture
//!
//! Follows the stateless advance pattern:
//! ```text
//! VictorySystem::check(&VictoryState, &world, &[TickEvent], VictoryConditions)
//!     -> Option<VictoryOutcome>
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
use crate::world::{GameWorld, VictoryConditions};

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
        victory_conditions: VictoryConditions,
    ) -> Option<VictoryOutcome> {
        if tick_events.is_empty() || state.resolved {
            return None;
        }

        // Grace period: don't check victory until the game has had time to develop
        let current_tick = tick_events.last().map(|e| e.tick).unwrap_or(0);
        if current_tick < Self::MIN_VICTORY_TICK {
            return None;
        }

        // Headquarters Only is literal: no leader or Death Star outcome can
        // terminate that campaign. Standard retains the established Death Star
        // paths, checked before an HQ/leader victory.
        if victory_conditions == VictoryConditions::Standard && state.death_star_active {
            if let Some(outcome) = Self::check_death_star(state, world) {
                return Some(outcome);
            }
        }

        if let Some(outcome) = Self::check_hq_capture(state, world) {
            if victory_conditions == VictoryConditions::HeadquartersOnly
                || Self::standard_leaders_captured(&outcome, world)
            {
                return Some(outcome);
            }
        }

        None
    }

    // ── Private ───────────────────────────────────────────────────────────

    /// HQ capture: the opposing faction has completed planetary occupation.
    ///
    /// Orbital supremacy alone is not a capture. System control is changed by
    /// ground combat and persists if the invading fleet later departs.
    fn check_hq_capture(state: &VictoryState, world: &GameWorld) -> Option<VictoryOutcome> {
        // Alliance HQ → Empire capture?
        if let Some(sys) = world.systems.get(state.alliance_hq) {
            if sys.control.is_controlled_by(Faction::Empire) {
                return Some(VictoryOutcome::HqCaptured {
                    winner: Faction::Empire,
                    loser: Faction::Alliance,
                    hq_system: state.alliance_hq,
                });
            }
        }

        // Empire HQ → Alliance capture?
        if let Some(sys) = world.systems.get(state.empire_hq) {
            if sys.control.is_controlled_by(Faction::Alliance) {
                return Some(VictoryOutcome::HqCaptured {
                    winner: Faction::Alliance,
                    loser: Faction::Empire,
                    hq_system: state.empire_hq,
                });
            }
        }

        None
    }

    /// Standard victory also requires the winner to hold both opposing
    /// principal leaders, matching the original game-type rules.
    fn standard_leaders_captured(outcome: &VictoryOutcome, world: &GameWorld) -> bool {
        let (winner, required_names): (Faction, &[&str]) = match outcome {
            VictoryOutcome::HqCaptured {
                winner: Faction::Empire,
                ..
            } => (Faction::Empire, &["Luke Skywalker", "Mon Mothma"]),
            VictoryOutcome::HqCaptured {
                winner: Faction::Alliance,
                ..
            } => (Faction::Alliance, &["Emperor Palpatine", "Darth Vader"]),
            _ => return true,
        };

        required_names.iter().all(|required| {
            world.characters.iter().any(|(_, character)| {
                character.name == *required
                    && character.is_captive
                    && character.captured_by == Some(winner)
            })
        })
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
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dat::ExplorationStatus;
    use crate::dat::SectorGroup;
    use crate::ids::DatId;
    use crate::tick::TickEvent;
    use crate::world::ControlKind;
    use crate::world::{Character, Fleet, GameWorld, Sector, System};

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
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: true,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Alliance),
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
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: true,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Empire),
        });

        (world, a_hq, e_hq)
    }

    fn add_fleet(
        world: &mut GameWorld,
        system: SystemKey,
        is_alliance: bool,
        has_death_star: bool,
    ) -> crate::ids::FleetKey {
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

    fn capture_leader(world: &mut GameWorld, name: &str, captured_by: Faction) {
        world.characters.insert(Character {
            name: name.to_string(),
            is_captive: true,
            captured_by: Some(captured_by),
            ..Character::default()
        });
    }

    #[test]
    fn no_outcome_without_ticks() {
        let (world, a, e) = make_world();
        let state = VictoryState::new(a, e);
        assert!(VictorySystem::check(&state, &world, &[], VictoryConditions::Standard).is_none());
    }

    #[test]
    fn no_outcome_when_resolved() {
        let (world, a, e) = make_world();
        let mut state = VictoryState::new(a, e);
        state.resolved = true;
        assert!(VictorySystem::check(
            &state,
            &world,
            &[tick(VictorySystem::MIN_VICTORY_TICK)],
            VictoryConditions::Standard
        )
        .is_none());
    }

    #[test]
    fn no_outcome_no_fleets() {
        let (world, a, e) = make_world();
        let state = VictoryState::new(a, e);
        assert!(VictorySystem::check(
            &state,
            &world,
            &[tick(VictorySystem::MIN_VICTORY_TICK)],
            VictoryConditions::Standard
        )
        .is_none());
    }

    #[test]
    fn headquarters_only_empire_captures_alliance_hq() {
        let (mut world, a, e) = make_world();
        world.systems.get_mut(a).unwrap().control = ControlKind::Controlled(Faction::Empire);

        let state = VictoryState::new(a, e);
        let out = VictorySystem::check(
            &state,
            &world,
            &[tick(VictorySystem::MIN_VICTORY_TICK)],
            VictoryConditions::HeadquartersOnly,
        );
        assert!(matches!(
            out,
            Some(VictoryOutcome::HqCaptured {
                winner: Faction::Empire,
                loser: Faction::Alliance,
                ..
            })
        ));
    }

    #[test]
    fn headquarters_only_alliance_captures_empire_hq() {
        let (mut world, a, e) = make_world();
        world.systems.get_mut(e).unwrap().control = ControlKind::Controlled(Faction::Alliance);

        let state = VictoryState::new(a, e);
        let out = VictorySystem::check(
            &state,
            &world,
            &[tick(VictorySystem::MIN_VICTORY_TICK)],
            VictoryConditions::HeadquartersOnly,
        );
        assert!(matches!(
            out,
            Some(VictoryOutcome::HqCaptured {
                winner: Faction::Alliance,
                loser: Faction::Empire,
                ..
            })
        ));
    }

    #[test]
    fn contested_hq_no_capture() {
        let (mut world, a, e) = make_world();
        world.systems.get_mut(a).unwrap().control = ControlKind::Contested;

        let state = VictoryState::new(a, e);
        assert!(VictorySystem::check(
            &state,
            &world,
            &[tick(VictorySystem::MIN_VICTORY_TICK)],
            VictoryConditions::HeadquartersOnly
        )
        .is_none());
    }

    #[test]
    fn orbital_supremacy_without_occupation_is_not_hq_capture() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        add_fleet(&mut world, alliance_hq, false, false);

        let state = VictoryState::new(alliance_hq, empire_hq);
        assert!(VictorySystem::check(
            &state,
            &world,
            &[tick(VictorySystem::MIN_VICTORY_TICK)],
            VictoryConditions::HeadquartersOnly,
        )
        .is_none());
    }

    #[test]
    fn occupied_hq_remains_captured_after_invading_fleet_departs() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        world.systems.get_mut(alliance_hq).unwrap().control =
            ControlKind::Controlled(Faction::Empire);

        let state = VictoryState::new(alliance_hq, empire_hq);
        assert!(matches!(
            VictorySystem::check(
                &state,
                &world,
                &[tick(VictorySystem::MIN_VICTORY_TICK)],
                VictoryConditions::HeadquartersOnly,
            ),
            Some(VictoryOutcome::HqCaptured {
                winner: Faction::Empire,
                ..
            })
        ));
    }

    #[test]
    fn death_star_destroys_alliance_hq() {
        let (mut world, a, e) = make_world();
        world.systems.get_mut(a).unwrap().is_destroyed = true;
        add_fleet(&mut world, a, false, true);

        let mut state = VictoryState::new(a, e);
        state.death_star_active = true;
        state.death_star_location = Some(a);

        let out = VictorySystem::check(
            &state,
            &world,
            &[tick(VictorySystem::MIN_VICTORY_TICK)],
            VictoryConditions::Standard,
        );
        assert!(matches!(out, Some(VictoryOutcome::DeathStarVictory { .. })));
    }

    #[test]
    fn death_star_destroyed_alliance_wins() {
        let (world, a, e) = make_world();
        // Death Star fleet was at `a` but has been removed — no ds fleet at location

        let mut state = VictoryState::new(a, e);
        state.death_star_active = true;
        state.death_star_location = Some(a);

        let out = VictorySystem::check(
            &state,
            &world,
            &[tick(VictorySystem::MIN_VICTORY_TICK)],
            VictoryConditions::Standard,
        );
        assert!(matches!(
            out,
            Some(VictoryOutcome::DeathStarDestroyed { .. })
        ));
    }

    #[test]
    fn death_star_inactive_no_check() {
        let (world, a, e) = make_world();
        let state = VictoryState::new(a, e);
        // death_star_active = false by default
        assert!(VictorySystem::check(
            &state,
            &world,
            &[tick(VictorySystem::MIN_VICTORY_TICK)],
            VictoryConditions::Standard
        )
        .is_none());
    }

    #[test]
    fn standard_hq_capture_waits_for_both_alliance_leaders() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        world.systems.get_mut(alliance_hq).unwrap().control =
            ControlKind::Controlled(Faction::Empire);
        capture_leader(&mut world, "Luke Skywalker", Faction::Empire);

        let state = VictoryState::new(alliance_hq, empire_hq);
        let tick = tick(VictorySystem::MIN_VICTORY_TICK);
        assert!(
            VictorySystem::check(&state, &world, &[tick], VictoryConditions::Standard,).is_none()
        );

        capture_leader(&mut world, "Mon Mothma", Faction::Empire);
        assert!(matches!(
            VictorySystem::check(&state, &world, &[tick], VictoryConditions::Standard,),
            Some(VictoryOutcome::HqCaptured {
                winner: Faction::Empire,
                ..
            })
        ));
    }

    #[test]
    fn standard_hq_capture_waits_for_both_empire_leaders() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        world.systems.get_mut(empire_hq).unwrap().control =
            ControlKind::Controlled(Faction::Alliance);
        capture_leader(&mut world, "Emperor Palpatine", Faction::Alliance);
        capture_leader(&mut world, "Darth Vader", Faction::Empire);

        let state = VictoryState::new(alliance_hq, empire_hq);
        let tick = tick(VictorySystem::MIN_VICTORY_TICK);
        assert!(
            VictorySystem::check(&state, &world, &[tick], VictoryConditions::Standard,).is_none()
        );

        let vader = world
            .characters
            .iter()
            .find(|(_, character)| character.name == "Darth Vader")
            .map(|(key, _)| key)
            .unwrap();
        world.characters.get_mut(vader).unwrap().captured_by = Some(Faction::Alliance);
        assert!(matches!(
            VictorySystem::check(&state, &world, &[tick], VictoryConditions::Standard,),
            Some(VictoryOutcome::HqCaptured {
                winner: Faction::Alliance,
                ..
            })
        ));
    }

    #[test]
    fn headquarters_only_ignores_death_star_outcomes() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        world.systems.get_mut(alliance_hq).unwrap().is_destroyed = true;
        add_fleet(&mut world, alliance_hq, false, true);
        add_fleet(&mut world, alliance_hq, true, false);
        let mut state = VictoryState::new(alliance_hq, empire_hq);
        state.death_star_active = true;
        state.death_star_location = Some(alliance_hq);

        assert!(VictorySystem::check(
            &state,
            &world,
            &[tick(VictorySystem::MIN_VICTORY_TICK)],
            VictoryConditions::HeadquartersOnly,
        )
        .is_none());
    }
}
