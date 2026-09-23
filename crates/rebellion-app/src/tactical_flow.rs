//! Production tactical-battle entry shared by campaign and test-only transports.

use std::collections::HashMap;

use rebellion_core::dat::Faction;
use rebellion_core::death_star::DeathStarState;
use rebellion_core::ids::{FleetKey, SystemKey};
use rebellion_core::troop_transport::TroopTransportState;
use rebellion_core::victory::VictoryState;
use rebellion_core::world::GameWorld;
use rebellion_render::{
    BattleSession, CombatWinner, GameMessage, MessageCategory, MessageLog, TacticalState,
};

use crate::GameMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BattleEntry {
    pub system: SystemKey,
    pub attacker: FleetKey,
    pub defender: FleetKey,
    pub player_is_attacker: bool,
    pub tick: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BattleEntryError {
    MissingSystem,
    SameFleet,
    MissingFleet,
    NotOrbitingSystem,
    SameFaction,
    MultipleDeathStars,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BattleReturn {
    pub winner: Option<CombatWinner>,
    pub winner_fleet: Option<FleetKey>,
    pub player_won: bool,
}

/// Compact strategic-world projection used to prove that a played tactical
/// result survives the transition back to the campaign. The fields are all
/// derived from the two source fleets named by the battle session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BattlePersistenceSnapshot {
    pub attacker_present: bool,
    pub defender_present: bool,
    pub attacker_capital_ships: usize,
    pub defender_capital_ships: usize,
    pub attacker_fighter_squadrons: u32,
    pub defender_fighter_squadrons: u32,
    pub attacker_has_death_star: bool,
    pub defender_has_death_star: bool,
}

/// Read the strategic state owned by a tactical session without mutating it.
pub(crate) fn persistence_snapshot(
    session: &BattleSession,
    world: &GameWorld,
) -> BattlePersistenceSnapshot {
    let fleet = |key| {
        world.fleets.get(key).map(|fleet| {
            (
                fleet.capital_ships.len(),
                fleet
                    .fighters
                    .iter()
                    .map(|fighter| fighter.count)
                    .sum::<u32>(),
                fleet.has_death_star,
            )
        })
    };
    let attacker = fleet(session.attacker_fleet);
    let defender = fleet(session.defender_fleet);
    BattlePersistenceSnapshot {
        attacker_present: attacker.is_some(),
        defender_present: defender.is_some(),
        attacker_capital_ships: attacker.map_or(0, |value| value.0),
        defender_capital_ships: defender.map_or(0, |value| value.0),
        attacker_fighter_squadrons: attacker.map_or(0, |value| value.1),
        defender_fighter_squadrons: defender.map_or(0, |value| value.1),
        attacker_has_death_star: attacker.is_some_and(|value| value.2),
        defender_has_death_star: defender.is_some_and(|value| value.2),
    }
}

/// Summarize a completed tactical session without mutating strategic state.
/// Auto-resolve uses this after its result has already been integrated.
pub(crate) fn summarize_results(session: &BattleSession) -> BattleReturn {
    BattleReturn {
        winner: session.winner,
        winner_fleet: match session.winner {
            Some(CombatWinner::Attacker) => Some(session.attacker_fleet),
            Some(CombatWinner::Defender) => Some(session.defender_fleet),
            Some(CombatWinner::Draw) | None => None,
        },
        player_won: match session.winner {
            Some(CombatWinner::Attacker) => session.player_is_attacker,
            Some(CombatWinner::Defender) => !session.player_is_attacker,
            Some(CombatWinner::Draw) | None => false,
        },
    }
}

/// Enter the actual tactical scene after validating the system and both fleets.
/// No state is changed when the request is invalid.
pub(crate) fn begin_player_battle(
    world: &GameWorld,
    entry: BattleEntry,
    tactical: &mut TacticalState,
    cooldowns: &mut HashMap<SystemKey, u64>,
    messages: &mut MessageLog,
    game_mode: &mut GameMode,
) -> Result<(), BattleEntryError> {
    let system = world
        .systems
        .get(entry.system)
        .ok_or(BattleEntryError::MissingSystem)?;
    if entry.attacker == entry.defender {
        return Err(BattleEntryError::SameFleet);
    }
    let attacker = world
        .fleets
        .get(entry.attacker)
        .ok_or(BattleEntryError::MissingFleet)?;
    let defender = world
        .fleets
        .get(entry.defender)
        .ok_or(BattleEntryError::MissingFleet)?;
    if attacker.location != entry.system
        || defender.location != entry.system
        || !system.fleets.contains(&entry.attacker)
        || !system.fleets.contains(&entry.defender)
    {
        return Err(BattleEntryError::NotOrbitingSystem);
    }
    if attacker.is_alliance == defender.is_alliance {
        return Err(BattleEntryError::SameFaction);
    }
    if attacker.has_death_star && defender.has_death_star {
        return Err(BattleEntryError::MultipleDeathStars);
    }

    tactical.begin_battle(
        world,
        entry.system,
        entry.attacker,
        entry.defender,
        entry.player_is_attacker,
        entry.tick,
    );
    cooldowns.insert(entry.system, entry.tick);
    messages.push(GameMessage::at_system(
        entry.tick,
        format!("Battle at {} — entering tactical combat!", system.name),
        MessageCategory::Combat,
        entry.system,
    ));
    *game_mode = GameMode::TacticalCombat;
    Ok(())
}

/// Apply the production tactical session back to the strategic world.
/// Campaign entry and the direct test launcher share this exact identity and
/// loss transport instead of maintaining fixture-specific return logic.
pub(crate) fn apply_results(
    session: &BattleSession,
    world: &mut GameWorld,
    troop_transport: &mut TroopTransportState,
) -> BattleReturn {
    for (fleet_key, is_attacker) in [
        (session.attacker_fleet, true),
        (session.defender_fleet, false),
    ] {
        if let Some(fleet) = world.fleets.get_mut(fleet_key) {
            if fleet.has_death_star {
                fleet.has_death_star = session.death_star.is_some_and(|death_star| {
                    death_star.is_attacker == is_attacker && !death_star.destroyed
                });
            }

            for (roster_index, ship_inst) in fleet.capital_ships.iter_mut().enumerate() {
                if !ship_inst.alive {
                    continue;
                }
                if let Some(result) = session.ships.iter().find(|ship| {
                    ship.is_attacker == is_attacker && ship.fleet_ship_index == roster_index
                }) {
                    ship_inst.hull_current = result.hull_current.clamp(0, result.hull_max);
                    ship_inst.alive = result.alive && ship_inst.hull_current > 0;
                    if !ship_inst.alive {
                        ship_inst.hull_current = 0;
                    }
                }
            }
            fleet.capital_ships.retain(|ship| ship.alive);

            let mut survivors = vec![0_u32; fleet.fighters.len()];
            for fighter in session
                .fighters
                .iter()
                .filter(|fighter| fighter.is_attacker == is_attacker)
            {
                if fighter.squad_count > 0 {
                    if let Some(count) = survivors.get_mut(fighter.fleet_fighter_index) {
                        *count = count.saturating_add(1);
                    }
                }
            }
            for (entry, surviving_squadrons) in fleet.fighters.iter_mut().zip(survivors) {
                entry.count = surviving_squadrons;
            }
        }

        let is_empty = world
            .fleets
            .get(fleet_key)
            .is_none_or(rebellion_core::world::Fleet::is_empty);
        if is_empty {
            let is_loser = match session.winner {
                Some(CombatWinner::Attacker) => fleet_key == session.defender_fleet,
                Some(CombatWinner::Defender) => fleet_key == session.attacker_fleet,
                Some(CombatWinner::Draw) | None => false,
            };
            let capture_data = is_loser
                .then(|| {
                    world.fleets.get(fleet_key).map(|fleet| {
                        let captor = if fleet.is_alliance {
                            Faction::Empire
                        } else {
                            Faction::Alliance
                        };
                        (fleet.characters.clone(), captor, fleet.location)
                    })
                })
                .flatten();
            if let Some((characters, captor, location)) = capture_data {
                for character_key in characters {
                    if let Some(character) = world.characters.get_mut(character_key) {
                        character.is_captive = true;
                        character.captured_by = Some(captor);
                        character.capture_tick = Some(session.start_tick);
                        character.current_system = Some(location);
                        character.current_fleet = None;
                    }
                }
            }
            if let Some(fleet) = world.fleets.get(fleet_key) {
                let location = fleet.location;
                if let Some(system) = world.systems.get_mut(location) {
                    system.fleets.retain(|&key| key != fleet_key);
                }
            }
            world.fleets.remove(fleet_key);
        }
    }
    troop_transport.destroy_untransportable_cargo(world);
    summarize_results(session)
}

/// Clear persistent Death Star pointers after either tactical or automatic
/// combat has removed the manager-owned object from its fleet.
pub(crate) fn reconcile_death_star_result(
    world: &GameWorld,
    death_star: &mut DeathStarState,
    victory: &mut VictoryState,
) -> bool {
    let Some(fleet_key) = death_star.death_star_fleet else {
        return false;
    };
    if world
        .fleets
        .get(fleet_key)
        .is_some_and(|fleet| fleet.has_death_star)
    {
        return false;
    }

    death_star.death_star_fleet = None;
    death_star.shield_generator_active = true;
    victory.death_star_active = false;
    victory.death_star_location = None;
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::dat::{ExplorationStatus, SectorGroup};
    use rebellion_core::ids::DatId;
    use rebellion_core::world::{Character, ControlKind, Fleet, Sector, System};

    fn opposing_empty_fleets() -> (GameWorld, SystemKey, FleetKey, FleetKey) {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(1),
            name: "Test Sector".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let system = world.systems.insert(System {
            dat_id: DatId::new(2),
            name: "Test System".into(),
            sector,
            x: 0,
            y: 0,
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
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Uncontrolled,
        });
        let make_fleet = |is_alliance| Fleet {
            location: system,
            capital_ships: vec![],
            fighters: vec![],
            characters: vec![],
            is_alliance,
            has_death_star: false,
        };
        let attacker = world.fleets.insert(make_fleet(true));
        let defender = world.fleets.insert(make_fleet(false));
        world.systems[system].fleets = vec![attacker, defender];
        (world, system, attacker, defender)
    }

    #[test]
    fn both_player_sides_enter_the_same_production_tactical_path() {
        let (world, system, attacker, defender) = opposing_empty_fleets();
        for player_is_attacker in [true, false] {
            let mut tactical = TacticalState::default();
            let mut cooldowns = HashMap::new();
            let mut messages = MessageLog::default();
            let mut mode = GameMode::Galaxy;
            begin_player_battle(
                &world,
                BattleEntry {
                    system,
                    attacker,
                    defender,
                    player_is_attacker,
                    tick: 17,
                },
                &mut tactical,
                &mut cooldowns,
                &mut messages,
                &mut mode,
            )
            .unwrap();
            assert_eq!(mode, GameMode::TacticalCombat);
            assert_eq!(cooldowns.get(&system), Some(&17));
            let session = tactical.session.as_ref().unwrap();
            assert_eq!(session.attacker_fleet, attacker);
            assert_eq!(session.defender_fleet, defender);
            assert_eq!(session.player_is_attacker, player_is_attacker);
            assert_eq!(session.phase, rebellion_render::BattlePhase::Combat);
            assert!(session.paused, "the original Tactical Display opens paused");
        }
    }

    #[test]
    fn invalid_entry_does_not_mutate_battle_state() {
        let (world, system, attacker, _defender) = opposing_empty_fleets();
        let mut tactical = TacticalState::default();
        let mut cooldowns = HashMap::new();
        let mut messages = MessageLog::default();
        let mut mode = GameMode::Galaxy;
        let result = begin_player_battle(
            &world,
            BattleEntry {
                system,
                attacker,
                defender: attacker,
                player_is_attacker: true,
                tick: 17,
            },
            &mut tactical,
            &mut cooldowns,
            &mut messages,
            &mut mode,
        );
        assert_eq!(result, Err(BattleEntryError::SameFleet));
        assert_eq!(mode, GameMode::Galaxy);
        assert!(tactical.session.is_none());
        assert!(cooldowns.is_empty());
    }

    #[test]
    fn entry_rejects_two_death_stars_without_mutating_battle_state() {
        let (mut world, system, attacker, defender) = opposing_empty_fleets();
        world.fleets[attacker].has_death_star = true;
        world.fleets[defender].has_death_star = true;
        let mut tactical = TacticalState::default();
        let mut cooldowns = HashMap::new();
        let mut messages = MessageLog::default();
        let mut mode = GameMode::Galaxy;

        let result = begin_player_battle(
            &world,
            BattleEntry {
                system,
                attacker,
                defender,
                player_is_attacker: true,
                tick: 17,
            },
            &mut tactical,
            &mut cooldowns,
            &mut messages,
            &mut mode,
        );

        assert_eq!(result, Err(BattleEntryError::MultipleDeathStars));
        assert_eq!(mode, GameMode::Galaxy);
        assert!(tactical.session.is_none());
        assert!(cooldowns.is_empty());
    }

    #[test]
    fn destroyed_losing_fleet_captures_officers_and_clears_fleet_assignment() {
        let (mut world, system, attacker, defender) = opposing_empty_fleets();
        world.fleets[attacker].has_death_star = true;
        let captive = world.characters.insert(Character {
            name: "Defeated Officer".into(),
            is_empire: true,
            current_system: Some(system),
            current_fleet: Some(defender),
            ..Character::default()
        });
        world.fleets[defender].characters.push(captive);
        let mut session = BattleSession::new(&world, system, attacker, defender, true, 23);
        session.winner = Some(CombatWinner::Attacker);

        let before = persistence_snapshot(&session, &world);
        assert!(before.attacker_present);
        assert!(before.defender_present);
        assert!(before.attacker_has_death_star);

        apply_results(&session, &mut world, &mut TroopTransportState::default());

        let after = persistence_snapshot(&session, &world);
        assert!(after.attacker_present);
        assert!(!after.defender_present);
        assert!(after.attacker_has_death_star);

        assert!(world.fleets.contains_key(attacker));
        assert!(!world.fleets.contains_key(defender));
        assert!(world.characters[captive].is_captive);
        assert_eq!(
            world.characters[captive].captured_by,
            Some(Faction::Alliance)
        );
        assert_eq!(world.characters[captive].capture_tick, Some(23));
        assert_eq!(world.characters[captive].current_system, Some(system));
        assert_eq!(world.characters[captive].current_fleet, None);
    }

    #[test]
    fn destroyed_death_star_clears_persistent_manager_and_victory_state() {
        let (mut world, system, attacker, defender) = opposing_empty_fleets();
        world.fleets[attacker].has_death_star = false;
        let mut death_star = DeathStarState {
            death_star_fleet: Some(attacker),
            shield_generator_active: false,
            ..DeathStarState::default()
        };
        let mut victory = VictoryState::new(system, system);
        victory.death_star_active = true;
        victory.death_star_location = Some(system);

        assert!(reconcile_death_star_result(
            &world,
            &mut death_star,
            &mut victory
        ));
        assert_eq!(death_star.death_star_fleet, None);
        assert!(death_star.shield_generator_active);
        assert!(!victory.death_star_active);
        assert_eq!(victory.death_star_location, None);
        assert!(!world.fleets[defender].has_death_star);
    }
}
