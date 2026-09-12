//! Production tactical-battle entry shared by campaign and test-only transports.

use std::collections::HashMap;

use rebellion_core::ids::{FleetKey, SystemKey};
use rebellion_core::world::GameWorld;
use rebellion_render::{GameMessage, MessageCategory, MessageLog, TacticalState};

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

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::dat::{ExplorationStatus, SectorGroup};
    use rebellion_core::ids::DatId;
    use rebellion_core::world::{ControlKind, Fleet, Sector, System};

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
}
