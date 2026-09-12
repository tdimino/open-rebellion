//! Victory condition detection.
//!
//! Checks the asymmetric original-game objectives each tick:
//! - **Alliance**: capture Coruscant and, in Standard mode, hold Emperor
//!   Palpatine and Darth Vader captive.
//! - **Empire**: destroy the mobile Alliance headquarters, then occupy its
//!   system, and, in Standard mode, hold Mon Mothma and Luke Skywalker captive.
//! - **Death Star route**: destroying the entire Alliance-HQ system satisfies
//!   the Empire's headquarters objective, but not Standard's leader objective.
//! - **Headquarters Only**: omits only the principal-leader requirements.
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

use crate::bombardment::BombardmentResult;
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
    /// The Alliance captured and controls Coruscant.
    ///
    /// `winner` controls `hq_system`; `loser` has lost their command center.
    HqCaptured {
        winner: Faction,
        loser: Faction,
        hq_system: SystemKey,
    },

    /// The Empire destroyed the mobile Alliance headquarters by bombardment
    /// and subsequently took control of its system.
    HqDestroyed {
        winner: Faction,
        loser: Faction,
        hq_system: SystemKey,
    },

    /// The Death Star destroyed the planet containing the Alliance HQ.
    ///
    /// The Empire wins: `target_system` has been destroyed.
    DeathStarVictory { target_system: SystemKey },
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
    #[must_use]
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
    /// detected; `None` otherwise. Skips frames without a simulation tick and
    /// already-resolved games. The original rules do not impose a minimum day.
    /// The caller must set `state.resolved = true` after acting on a result.
    #[must_use]
    pub fn check(
        state: &VictoryState,
        world: &GameWorld,
        tick_events: &[TickEvent],
        victory_conditions: VictoryConditions,
    ) -> Option<VictoryOutcome> {
        if tick_events.is_empty() || state.resolved {
            return None;
        }

        Self::headquarters_objectives(state, world)
            .into_iter()
            .flatten()
            .find(|outcome| {
                victory_conditions == VictoryConditions::HeadquartersOnly
                    || Self::standard_leaders_captured(outcome, world)
            })
    }

    /// Apply the headquarters-specific effect of a resolved bombardment.
    ///
    /// `System::is_headquarters` represents the surviving mobile Alliance-HQ
    /// facility. Clearing that already-persisted flag records its destruction
    /// without changing the save layout. Occupation is checked separately, so
    /// bombardment alone cannot end the campaign.
    pub fn apply_headquarters_bombardment(
        state: &VictoryState,
        world: &mut GameWorld,
        result: &BombardmentResult,
        attacker: Faction,
    ) -> bool {
        if attacker != Faction::Empire || result.damage <= 0 || result.system != state.alliance_hq {
            return false;
        }

        let Some(system) = world.systems.get_mut(state.alliance_hq) else {
            return false;
        };
        if !system.is_headquarters || system.is_destroyed {
            return false;
        }

        system.is_headquarters = false;
        true
    }

    // ── Private ───────────────────────────────────────────────────────────

    /// Evaluate the two faction-specific headquarters objectives.
    fn headquarters_objectives(
        state: &VictoryState,
        world: &GameWorld,
    ) -> [Option<VictoryOutcome>; 2] {
        // Empire: a destroyed HQ planet completes the objective immediately.
        // Otherwise the HQ facility must already be destroyed and the Empire
        // must control the surviving system.
        let empire = world.systems.get(state.alliance_hq).and_then(|sys| {
            if sys.is_destroyed {
                Some(VictoryOutcome::DeathStarVictory {
                    target_system: state.alliance_hq,
                })
            } else if !sys.is_headquarters && sys.control.is_controlled_by(Faction::Empire) {
                Some(VictoryOutcome::HqDestroyed {
                    winner: Faction::Empire,
                    loser: Faction::Alliance,
                    hq_system: state.alliance_hq,
                })
            } else {
                None
            }
        });

        // Alliance: political control of Coruscant completes the objective.
        let alliance = world.systems.get(state.empire_hq).and_then(|sys| {
            if sys.control.is_controlled_by(Faction::Alliance) {
                Some(VictoryOutcome::HqCaptured {
                    winner: Faction::Alliance,
                    loser: Faction::Empire,
                    hq_system: state.empire_hq,
                })
            } else {
                None
            }
        });

        [empire, alliance]
    }

    /// Standard victory also requires the winner to hold both opposing
    /// principal leaders, matching the original game-type rules.
    fn standard_leaders_captured(outcome: &VictoryOutcome, world: &GameWorld) -> bool {
        let winner = match outcome {
            VictoryOutcome::HqCaptured { winner, .. }
            | VictoryOutcome::HqDestroyed { winner, .. } => *winner,
            VictoryOutcome::DeathStarVictory { .. } => Faction::Empire,
        };
        let required_names: &[&str] = match winner {
            Faction::Alliance => &["Emperor Palpatine", "Darth Vader"],
            Faction::Empire => &["Luke Skywalker", "Mon Mothma"],
            Faction::Neutral => return false,
        };

        required_names.iter().all(|required| {
            world.characters.iter().any(|(_, character)| {
                character.name == *required
                    && character.is_captive
                    && character.captured_by == Some(winner)
            })
        })
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
    use crate::world::{Character, GameWorld, Sector, System};

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
        assert!(
            VictorySystem::check(&state, &world, &[tick(1)], VictoryConditions::Standard).is_none()
        );
    }

    #[test]
    fn occupation_without_destruction_does_not_defeat_alliance() {
        let (mut world, a, e) = make_world();
        world.systems.get_mut(a).unwrap().control = ControlKind::Controlled(Faction::Empire);

        let state = VictoryState::new(a, e);
        assert!(VictorySystem::check(
            &state,
            &world,
            &[tick(1)],
            VictoryConditions::HeadquartersOnly,
        )
        .is_none());
    }

    #[test]
    fn headquarters_only_alliance_captures_coruscant_on_first_tick() {
        let (mut world, a, e) = make_world();
        world.systems.get_mut(e).unwrap().control = ControlKind::Controlled(Faction::Alliance);

        let state = VictoryState::new(a, e);
        let out = VictorySystem::check(
            &state,
            &world,
            &[tick(1)],
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
    fn successful_imperial_bombardment_destroys_alliance_hq_facility() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        let state = VictoryState::new(alliance_hq, empire_hq);
        let result = BombardmentResult {
            system: alliance_hq,
            damage: 12,
            tick: 1,
        };

        assert!(VictorySystem::apply_headquarters_bombardment(
            &state,
            &mut world,
            &result,
            Faction::Empire,
        ));
        assert!(!world.systems[alliance_hq].is_headquarters);
    }

    #[test]
    fn bombardment_requires_empire_damage_against_current_alliance_hq() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        let state = VictoryState::new(alliance_hq, empire_hq);
        let mut result = BombardmentResult {
            system: alliance_hq,
            damage: 0,
            tick: 1,
        };
        assert!(!VictorySystem::apply_headquarters_bombardment(
            &state,
            &mut world,
            &result,
            Faction::Empire,
        ));

        result.damage = 10;
        assert!(!VictorySystem::apply_headquarters_bombardment(
            &state,
            &mut world,
            &result,
            Faction::Alliance,
        ));

        result.system = empire_hq;
        assert!(!VictorySystem::apply_headquarters_bombardment(
            &state,
            &mut world,
            &result,
            Faction::Empire,
        ));
        assert!(world.systems[alliance_hq].is_headquarters);
    }

    #[test]
    fn destroyed_alliance_hq_requires_imperial_control() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        world.systems[alliance_hq].is_headquarters = false;

        let state = VictoryState::new(alliance_hq, empire_hq);
        assert!(VictorySystem::check(
            &state,
            &world,
            &[tick(1)],
            VictoryConditions::HeadquartersOnly,
        )
        .is_none());

        world.systems[alliance_hq].control = ControlKind::Controlled(Faction::Empire);
        assert!(matches!(
            VictorySystem::check(
                &state,
                &world,
                &[tick(1)],
                VictoryConditions::HeadquartersOnly
            ),
            Some(VictoryOutcome::HqDestroyed {
                winner: Faction::Empire,
                loser: Faction::Alliance,
                ..
            })
        ));
    }

    #[test]
    fn bombardment_then_occupation_completes_imperial_hq_objective() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        let state = VictoryState::new(alliance_hq, empire_hq);
        let result = BombardmentResult {
            system: alliance_hq,
            damage: 1,
            tick: 1,
        };

        assert!(VictorySystem::apply_headquarters_bombardment(
            &state,
            &mut world,
            &result,
            Faction::Empire,
        ));
        world.systems[alliance_hq].control = ControlKind::Controlled(Faction::Empire);

        assert!(matches!(
            VictorySystem::check(&state, &world, &[tick(1)], VictoryConditions::HeadquartersOnly),
            Some(VictoryOutcome::HqDestroyed { hq_system, .. }) if hq_system == alliance_hq
        ));
    }

    #[test]
    fn death_star_hq_destruction_satisfies_headquarters_only() {
        let (mut world, a, e) = make_world();
        world.systems.get_mut(a).unwrap().is_destroyed = true;
        let state = VictoryState::new(a, e);

        let out = VictorySystem::check(
            &state,
            &world,
            &[tick(1)],
            VictoryConditions::HeadquartersOnly,
        );
        assert!(matches!(out, Some(VictoryOutcome::DeathStarVictory { .. })));
    }

    #[test]
    fn destruction_of_a_non_hq_planet_is_not_a_victory() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        world.systems[empire_hq].is_destroyed = true;
        let state = VictoryState::new(alliance_hq, empire_hq);

        assert!(VictorySystem::check(
            &state,
            &world,
            &[tick(1)],
            VictoryConditions::HeadquartersOnly,
        )
        .is_none());
    }

    #[test]
    fn death_star_loss_is_nonterminal() {
        let (world, a, e) = make_world();
        let mut state = VictoryState::new(a, e);
        state.death_star_active = true;
        state.death_star_location = Some(a);

        assert!(
            VictorySystem::check(&state, &world, &[tick(1)], VictoryConditions::Standard,)
                .is_none()
        );
    }

    #[test]
    fn standard_destroyed_hq_waits_for_both_alliance_leaders() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        world.systems[alliance_hq].is_headquarters = false;
        world.systems[alliance_hq].control = ControlKind::Controlled(Faction::Empire);
        capture_leader(&mut world, "Luke Skywalker", Faction::Empire);

        let state = VictoryState::new(alliance_hq, empire_hq);
        let tick = tick(1);
        assert!(
            VictorySystem::check(&state, &world, &[tick], VictoryConditions::Standard,).is_none()
        );

        capture_leader(&mut world, "Mon Mothma", Faction::Empire);
        assert!(matches!(
            VictorySystem::check(&state, &world, &[tick], VictoryConditions::Standard,),
            Some(VictoryOutcome::HqDestroyed {
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
        let tick = tick(1);
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
    fn incomplete_imperial_objective_does_not_mask_complete_alliance_victory() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        world.systems[alliance_hq].is_headquarters = false;
        world.systems[alliance_hq].control = ControlKind::Controlled(Faction::Empire);
        world.systems[empire_hq].control = ControlKind::Controlled(Faction::Alliance);
        capture_leader(&mut world, "Emperor Palpatine", Faction::Alliance);
        capture_leader(&mut world, "Darth Vader", Faction::Alliance);

        let state = VictoryState::new(alliance_hq, empire_hq);
        assert!(matches!(
            VictorySystem::check(&state, &world, &[tick(1)], VictoryConditions::Standard),
            Some(VictoryOutcome::HqCaptured {
                winner: Faction::Alliance,
                ..
            })
        ));
    }

    #[test]
    fn standard_death_star_route_waits_for_both_alliance_leaders() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        world.systems.get_mut(alliance_hq).unwrap().is_destroyed = true;
        let state = VictoryState::new(alliance_hq, empire_hq);

        assert!(
            VictorySystem::check(&state, &world, &[tick(1)], VictoryConditions::Standard,)
                .is_none()
        );

        capture_leader(&mut world, "Luke Skywalker", Faction::Empire);
        capture_leader(&mut world, "Mon Mothma", Faction::Empire);
        assert!(matches!(
            VictorySystem::check(&state, &world, &[tick(1)], VictoryConditions::Standard),
            Some(VictoryOutcome::DeathStarVictory { .. })
        ));
    }
}
