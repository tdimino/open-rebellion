//! Production tactical-battle entry shared by campaign and test-only transports.

use rebellion_core::carriage::ProductRoom;
use rebellion_core::combat::CombatForce;
use rebellion_core::ids::{FleetKey, SystemKey};
use rebellion_core::movement::MovementState;
use rebellion_core::troop_transport::TroopTransportState;
use rebellion_core::world::GameWorld;
use rebellion_render::{BattleSession, GameMessage, MessageCategory, MessageLog, TacticalState};

use crate::GameMode;

pub(crate) use rebellion_core::tactical::apply::{summarize_results, BattleReturn};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BattleEntry {
    pub system: SystemKey,
    /// Each side: its fleet, if any, and the squadrons the system holds for
    /// it ([`CombatForce`]).
    pub attacker: CombatForce,
    pub defender: CombatForce,
    pub player_is_attacker: bool,
    pub tick: u64,
    pub rng_seed: u32,
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
    let fleet = |key: Option<FleetKey>| {
        world.fleets.get(key?).map(|fleet| {
            (
                fleet.capital_ships.len(),
                fleet
                    .fighters
                    .iter()
                    .map(|fighter| fighter.count)
                    .sum::<u32>(),
                fleet.has_death_star(&world.capital_ship_classes),
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

/// Enter the actual tactical scene after validating the system and both fleets.
/// No state is changed when the request is invalid.
pub(crate) fn begin_player_battle(
    world: &GameWorld,
    manufacturing: Option<&rebellion_core::manufacturing::ManufacturingState>,
    entry: BattleEntry,
    tactical: &mut TacticalState,
    messages: &mut MessageLog,
    game_mode: &mut GameMode,
) -> Result<(), BattleEntryError> {
    let system = world
        .systems
        .get(entry.system)
        .ok_or(BattleEntryError::MissingSystem)?;
    if entry.attacker.fleet.is_some() && entry.attacker.fleet == entry.defender.fleet {
        return Err(BattleEntryError::SameFleet);
    }
    if entry.attacker.is_alliance == entry.defender.is_alliance {
        return Err(BattleEntryError::SameFaction);
    }
    let mut death_stars = 0;
    for force in [entry.attacker, entry.defender] {
        // A side without a fleet fights with the squadrons the system holds
        // for it (FUN_0040b0e0 exports them after the fleets); it must hold some.
        let Some(key) = force.fleet else {
            if system.squadron_count(force.is_alliance) == 0 {
                return Err(BattleEntryError::MissingFleet);
            }
            continue;
        };
        let fleet = world
            .fleets
            .get(key)
            .ok_or(BattleEntryError::MissingFleet)?;
        if fleet.location != entry.system || !system.fleets.contains(&key) {
            return Err(BattleEntryError::NotOrbitingSystem);
        }
        if fleet.is_alliance != force.is_alliance {
            return Err(BattleEntryError::SameFaction);
        }
        // The battle seats a usable Death Star only (`FUN_005003d0(fleet, 1)`,
        // `FUN_0040b0e0.c:175`).
        death_stars += usize::from(fleet.has_usable_death_star(&world.capital_ship_classes));
    }
    if death_stars > 1 {
        return Err(BattleEntryError::MultipleDeathStars);
    }

    tactical.begin_battle_forces(
        world,
        entry.system,
        entry.attacker,
        entry.defender,
        entry.player_is_attacker,
        entry.tick,
        entry.rng_seed,
    );
    // A fleet's retreat place and a computer side's threshold read neither
    // the transit nor the cargo state (squadrons alone do), so empty ones
    // stand in; the player's own side withdraws only at strength 0.
    if let Some(session) = tactical.session.as_mut() {
        if let Some(manufacturing) = manufacturing {
            session.seat_unfinished_station(world, manufacturing);
        }
        rebellion_core::tactical::apply::seat_morale(
            session,
            world,
            &TroopTransportState::default(),
            &MovementState::default(),
            Some(entry.player_is_attacker),
        );
    }
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
    movement: &mut MovementState,
    products: &ProductRoom,
) -> BattleReturn {
    let mut log = Vec::new();
    let before =
        rebellion_data::telemetry::native_battle_begin(session.start_tick, world, session.system);
    let result = rebellion_core::tactical::apply::apply_results(
        session,
        world,
        troop_transport,
        movement,
        products,
        &mut log,
    );
    rebellion_data::telemetry::native_battle_end(
        session.start_tick,
        world,
        session.system,
        before,
        session.combat_tick / 4,
    );
    for line in log {
        macroquad::logging::info!("{}", line);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::dat::{ExplorationStatus, SectorGroup};
    use rebellion_core::ids::DatId;
    use rebellion_core::tactical::CombatWinner;
    use rebellion_core::world::{
        CapitalShipClass, Character, ControlKind, Fleet, Sector, ShipInstance, System,
    };

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
            fighters: Vec::new(),
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
            bombarding: false,
        };
        let attacker = world.fleets.insert(make_fleet(true));
        let defender = world.fleets.insert(make_fleet(false));
        world.systems[system].fleets = vec![attacker, defender];
        (world, system, attacker, defender)
    }

    /// Give `fleet` a Death Star ship: a capital ship of family `0x18`
    /// (`FUN_005003d0`).
    fn give_death_star_ship(world: &mut GameWorld, fleet: FleetKey) {
        let is_alliance = world.fleets[fleet].is_alliance;
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(rebellion_core::world::DEATH_STAR_CLASS_ID),
            family: 0x18,
            is_alliance,
            is_empire: !is_alliance,
            hull: 100,
            ..CapitalShipClass::default()
        });
        world.fleets[fleet]
            .capital_ships
            .push(ShipInstance::new(class, 100, is_alliance));
    }

    #[test]
    fn both_player_sides_enter_the_same_production_tactical_path() {
        let (world, system, attacker, defender) = opposing_empty_fleets();
        for player_is_attacker in [true, false] {
            let mut tactical = TacticalState::default();
            let mut messages = MessageLog::default();
            let mut mode = GameMode::Galaxy;
            begin_player_battle(
                &world,
                None,
                BattleEntry {
                    system,
                    attacker: CombatForce::of_fleet(&world, attacker),
                    defender: CombatForce::of_fleet(&world, defender),
                    player_is_attacker,
                    tick: 17,
                    rng_seed: 0x1111_0001,
                },
                &mut tactical,
                &mut messages,
                &mut mode,
            )
            .unwrap();
            assert_eq!(mode, GameMode::TacticalCombat);
            let session = tactical.session.as_ref().unwrap();
            assert_eq!(session.attacker_fleet, Some(attacker));
            assert_eq!(session.defender_fleet, Some(defender));
            assert_eq!(session.player_is_attacker, player_is_attacker);
            assert_eq!(session.phase, rebellion_render::BattlePhase::Combat);
            assert!(session.paused, "the original Tactical Display opens paused");
        }
    }

    #[test]
    fn invalid_entry_does_not_mutate_battle_state() {
        let (world, system, attacker, _defender) = opposing_empty_fleets();
        let mut tactical = TacticalState::default();
        let mut messages = MessageLog::default();
        let mut mode = GameMode::Galaxy;
        let result = begin_player_battle(
            &world,
            None,
            BattleEntry {
                system,
                attacker: CombatForce::of_fleet(&world, attacker),
                defender: CombatForce::of_fleet(&world, attacker),
                player_is_attacker: true,
                tick: 17,
                rng_seed: 0x1111_0002,
            },
            &mut tactical,
            &mut messages,
            &mut mode,
        );
        assert_eq!(result, Err(BattleEntryError::SameFleet));
        assert_eq!(mode, GameMode::Galaxy);
        assert!(tactical.session.is_none());
    }

    #[test]
    fn entry_rejects_two_death_stars_without_mutating_battle_state() {
        let (mut world, system, attacker, defender) = opposing_empty_fleets();
        give_death_star_ship(&mut world, attacker);
        give_death_star_ship(&mut world, defender);
        let mut tactical = TacticalState::default();
        let mut messages = MessageLog::default();
        let mut mode = GameMode::Galaxy;

        let result = begin_player_battle(
            &world,
            None,
            BattleEntry {
                system,
                attacker: CombatForce::of_fleet(&world, attacker),
                defender: CombatForce::of_fleet(&world, defender),
                player_is_attacker: true,
                tick: 17,
                rng_seed: 0x1111_0003,
            },
            &mut tactical,
            &mut messages,
            &mut mode,
        );

        assert_eq!(result, Err(BattleEntryError::MultipleDeathStars));
        assert_eq!(mode, GameMode::Galaxy);
        assert!(tactical.session.is_none());
    }

    #[test]
    fn destroyed_losing_fleet_leaves_its_officers_adrift_at_the_system_uncaptured() {
        // FUN_004ef2e0 sets a lost container's character adrift
        // (FUN_00534790); the battle's write-back takes no one captive
        // (ghidra/notes/tactical-results.md 3).
        let (mut world, system, attacker, defender) = opposing_empty_fleets();
        give_death_star_ship(&mut world, attacker);
        let captive = world.characters.insert(Character {
            name: "Defeated Officer".into(),
            is_empire: true,
            current_system: Some(system),
            current_fleet: Some(defender),
            ..Character::default()
        });
        world.fleets[defender].characters.push(captive);
        let mut session =
            BattleSession::new(&world, system, attacker, defender, true, 23, 0x1111_0004);
        session.winner = Some(CombatWinner::Attacker);

        let before = persistence_snapshot(&session, &world);
        assert!(before.attacker_present);
        assert!(before.defender_present);
        assert!(before.attacker_has_death_star);

        apply_results(
            &session,
            &mut world,
            &mut TroopTransportState::default(),
            &mut MovementState::new(),
            &ProductRoom::default(),
        );

        let after = persistence_snapshot(&session, &world);
        assert!(after.attacker_present);
        assert!(!after.defender_present);
        assert!(after.attacker_has_death_star);

        assert!(world.fleets.contains_key(attacker));
        assert!(!world.fleets.contains_key(defender));
        assert!(!world.characters[captive].is_captive);
        assert_eq!(world.characters[captive].captured_by, None);
        assert!(world.characters[captive].adrift);
        assert_eq!(world.characters[captive].current_system, Some(system));
        assert_eq!(world.characters[captive].current_fleet, None);
    }
}
