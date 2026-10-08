//! Production tactical-battle entry shared by campaign and test-only transports.

use std::collections::HashMap;

use rebellion_core::bombardment::BombardmentSystem;
use rebellion_core::combat::{CombatSide, CombatSystem};
use rebellion_core::dat::Faction;
use rebellion_core::death_star::DeathStarState;
use rebellion_core::ids::{FleetKey, SystemKey, TroopKey};
use rebellion_core::troop_transport::TroopTransportState;
use rebellion_core::victory::{VictoryState, VictorySystem};
use rebellion_core::world::{ControlKind, GameWorld};
use rebellion_render::{
    BattleSession, CombatWinner, GameMessage, GroundCombatState, MessageCategory, MessageLog,
    MessageRail, RailAudience, TacticalState,
};

use crate::GameMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BattleEntry {
    pub system: SystemKey,
    pub attacker: FleetKey,
    pub defender: FleetKey,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BattleReturn {
    pub winner: Option<CombatWinner>,
    pub winner_fleet: Option<FleetKey>,
    pub player_won: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum PostBattleGroundMode<'a> {
    Interactive,
    Automatic { rolls: &'a [f64] },
}

#[derive(Debug, Clone)]
pub(crate) enum PostBattleContinuation {
    Galaxy,
    GroundCombat(GroundCombatState),
}

#[derive(Debug, Clone)]
pub(crate) struct PostBattleOutcome {
    pub continuation: PostBattleContinuation,
    pub landed_regiments: usize,
    pub bombardment_damage: i32,
    pub headquarters_destroyed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BombardmentFollowup {
    pub damage: i32,
    pub headquarters_destroyed: bool,
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
        entry.rng_seed,
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
            rebellion_core::carriage::drop_lost_squadrons(fleet);
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

/// Run the campaign follow-up shared by auto-resolved and played space battles.
///
/// Both paths bombard first, land every surviving regiment carried by the
/// victorious faction, and then either resolve the ground fight immediately or
/// return the production ground-combat state used by the interactive campaign.
pub(crate) fn resolve_post_battle(
    world: &mut GameWorld,
    victory_state: &VictoryState,
    troop_transport: &mut TroopTransportState,
    winner_fleet: FleetKey,
    system: SystemKey,
    tick: u64,
    log: &mut MessageLog,
    ground_mode: PostBattleGroundMode<'_>,
) -> PostBattleOutcome {
    let Some(attacker_is_alliance) = world
        .fleets
        .get(winner_fleet)
        .map(|fleet| fleet.is_alliance)
    else {
        return PostBattleOutcome {
            continuation: PostBattleContinuation::Galaxy,
            landed_regiments: 0,
            bombardment_damage: 0,
            headquarters_destroyed: false,
        };
    };

    let bombardment =
        apply_automatic_bombardment(world, victory_state, winner_fleet, system, tick, log);
    let landed_regiments = land_faction_cargo(
        world,
        troop_transport,
        system,
        attacker_is_alliance,
        log,
        tick,
    );

    let continuation = match ground_mode {
        PostBattleGroundMode::Automatic { rolls } => {
            resolve_ground_after_landing(
                world,
                system,
                attacker_is_alliance,
                landed_regiments,
                rolls,
                tick,
                log,
            );
            PostBattleContinuation::Galaxy
        }
        PostBattleGroundMode::Interactive => {
            let (attacker_troops, defender_troops) =
                ground_rosters(world, system, attacker_is_alliance);
            if !attacker_troops.is_empty() && !defender_troops.is_empty() {
                let system_name = world
                    .systems
                    .get(system)
                    .map_or_else(|| "unknown".to_owned(), |value| value.name.clone());
                PostBattleContinuation::GroundCombat(GroundCombatState::new(
                    system,
                    system_name,
                    attacker_is_alliance,
                    attacker_troops,
                    defender_troops,
                ))
            } else {
                if landed_regiments > 0 && !attacker_troops.is_empty() {
                    apply_system_occupation(
                        world,
                        system,
                        if attacker_is_alliance {
                            Faction::Alliance
                        } else {
                            Faction::Empire
                        },
                        tick,
                        log,
                    );
                }
                PostBattleContinuation::Galaxy
            }
        }
    };

    PostBattleOutcome {
        continuation,
        landed_regiments,
        bombardment_damage: bombardment.damage,
        headquarters_destroyed: bombardment.headquarters_destroyed,
    }
}

/// Persist every tactical regiment's final strength back into the campaign.
/// Destroyed regiments leave both the troop arena and the system roster.
pub(crate) fn apply_tactical_ground_strengths(
    world: &mut GameWorld,
    system: SystemKey,
    regiment_strengths: &[(TroopKey, i16)],
) {
    for &(troop, strength) in regiment_strengths {
        if strength > 0 {
            if let Some(unit) = world.troops.get_mut(troop) {
                unit.regiment_strength = strength;
            }
        } else {
            world.troops.remove(troop);
        }
    }

    let living_ground_units: Vec<_> = world
        .systems
        .get(system)
        .map(|value| {
            value
                .ground_units
                .iter()
                .copied()
                .filter(|troop| {
                    world
                        .troops
                        .get(*troop)
                        .is_some_and(|unit| unit.regiment_strength > 0)
                })
                .collect()
        })
        .unwrap_or_default();
    if let Some(value) = world.systems.get_mut(system) {
        value.ground_units = living_ground_units;
    }
}

pub(crate) fn land_faction_cargo(
    world: &mut GameWorld,
    troop_transport: &mut TroopTransportState,
    system: SystemKey,
    is_alliance: bool,
    log: &mut MessageLog,
    tick: u64,
) -> usize {
    let fleets: Vec<_> = world
        .systems
        .get(system)
        .map(|value| {
            value
                .fleets
                .iter()
                .copied()
                .filter(|fleet| {
                    world
                        .fleets
                        .get(*fleet)
                        .is_some_and(|value| value.is_alliance == is_alliance)
                        && troop_transport.landing_count(*fleet) > 0
                })
                .collect()
        })
        .unwrap_or_default();
    let mut landed = 0;
    for fleet in fleets {
        landed += troop_transport
            .disembark_all(world, fleet, system)
            .map_or(0, |troops| troops.len());
    }
    if landed > 0 {
        let name = world
            .systems
            .get(system)
            .map_or("unknown", |value| value.name.as_str());
        log.push(GameMessage::at_system(
            tick,
            format!("{landed} regiment(s) landed at {name}"),
            MessageCategory::Combat,
            system,
        ));
    }
    landed
}

/// Apply the campaign bombardment that follows an uncontested orbital win.
pub(crate) fn apply_automatic_bombardment(
    world: &mut GameWorld,
    victory_state: &VictoryState,
    fleet: FleetKey,
    system: SystemKey,
    tick: u64,
    log: &mut MessageLog,
) -> BombardmentFollowup {
    if !world.fleets.contains_key(fleet) || !world.systems.contains_key(system) {
        return BombardmentFollowup {
            damage: 0,
            headquarters_destroyed: false,
        };
    }

    let attacker = if world.fleets[fleet].is_alliance {
        Faction::Alliance
    } else {
        Faction::Empire
    };
    let result =
        BombardmentSystem::resolve_bombardment(world, fleet, system, world.difficulty_index, tick);
    let headquarters_destroyed =
        VictorySystem::apply_headquarters_bombardment(victory_state, world, &result, attacker);
    let system_name = world
        .systems
        .get(system)
        .map_or("unknown", |value| value.name.as_str());

    if result.damage > 0 {
        log.push(GameMessage::at_system(
            tick,
            format!(
                "Orbital bombardment at {} - {} damage",
                system_name, result.damage
            ),
            MessageCategory::Combat,
            system,
        ));
    }
    if headquarters_destroyed {
        log.push(
            GameMessage::at_system(
                tick,
                format!("Alliance headquarters destroyed at {system_name}"),
                MessageCategory::Combat,
                system,
            )
            .on_rail(MessageRail::Resource, RailAudience::Both),
        );
    }

    BombardmentFollowup {
        damage: result.damage,
        headquarters_destroyed,
    }
}

pub(crate) fn apply_system_occupation(
    world: &mut GameWorld,
    system: SystemKey,
    winner: Faction,
    tick: u64,
    log: &mut MessageLog,
) {
    let previous = world.systems.get(system).map(|value| value.control);
    if let Some(value) = world.systems.get_mut(system) {
        value.control = ControlKind::Controlled(winner);
    }
    if previous != Some(ControlKind::Controlled(winner)) {
        let name = world
            .systems
            .get(system)
            .map_or("unknown", |value| value.name.as_str());
        log.push(
            GameMessage::at_system(
                tick,
                format!("{name} occupied by {winner:?}"),
                MessageCategory::Combat,
                system,
            )
            .on_rail(MessageRail::PopularSupport, RailAudience::Both),
        );
    }

    for (_, character) in &mut world.characters {
        let is_enemy = match winner {
            Faction::Alliance => character.is_empire,
            Faction::Empire => character.is_alliance,
            Faction::Neutral => false,
        };
        if character.current_system == Some(system) && is_enemy && !character.is_killed {
            character.is_captive = true;
            character.captured_by = Some(winner);
            character.capture_tick = Some(tick);
            character.current_fleet = None;
        }
    }
}

pub(crate) fn resolve_ground_campaign(
    world: &mut GameWorld,
    troop_transport: &mut TroopTransportState,
    system: SystemKey,
    attacker_is_alliance: bool,
    rolls: &[f64],
    tick: u64,
    log: &mut MessageLog,
) {
    let landed = land_faction_cargo(
        world,
        troop_transport,
        system,
        attacker_is_alliance,
        log,
        tick,
    );
    resolve_ground_after_landing(
        world,
        system,
        attacker_is_alliance,
        landed,
        rolls,
        tick,
        log,
    );
}

fn resolve_ground_after_landing(
    world: &mut GameWorld,
    system: SystemKey,
    attacker_is_alliance: bool,
    landed: usize,
    rolls: &[f64],
    tick: u64,
    log: &mut MessageLog,
) {
    let (attacker_troops, defender_troops) = ground_rosters(world, system, attacker_is_alliance);
    let winner = if !attacker_troops.is_empty() && !defender_troops.is_empty() {
        let mut final_winner = CombatSide::Draw;
        let mut rounds = 0_u32;
        while rounds < 256 {
            let result = CombatSystem::resolve_ground(
                world,
                system,
                attacker_is_alliance,
                world.difficulty_index,
                rolls,
                tick,
            );
            let made_progress = result
                .troop_damage
                .iter()
                .any(|event| event.strength_after < event.strength_before);
            final_winner = result.winner;
            rounds += 1;
            rebellion_data::integrator::apply_ground_combat_result_inner(&result, world);
            if final_winner != CombatSide::Draw || !made_progress {
                break;
            }
        }
        let name = world
            .systems
            .get(system)
            .map_or("unknown", |value| value.name.as_str());
        log.push(GameMessage::at_system(
            tick,
            format!("Ground battle at {name}: {final_winner:?} after {rounds} round(s)"),
            MessageCategory::Combat,
            system,
        ));
        match final_winner {
            CombatSide::Attacker => Some(if attacker_is_alliance {
                Faction::Alliance
            } else {
                Faction::Empire
            }),
            CombatSide::Defender => Some(if attacker_is_alliance {
                Faction::Empire
            } else {
                Faction::Alliance
            }),
            CombatSide::Draw => None,
        }
    } else if landed > 0 && !attacker_troops.is_empty() {
        Some(if attacker_is_alliance {
            Faction::Alliance
        } else {
            Faction::Empire
        })
    } else {
        None
    };
    if let Some(winner) = winner {
        apply_system_occupation(world, system, winner, tick, log);
    }
}

fn ground_rosters(
    world: &GameWorld,
    system: SystemKey,
    attacker_is_alliance: bool,
) -> (Vec<(TroopKey, String, i16)>, Vec<(TroopKey, String, i16)>) {
    let mut attacker_index = 0_u32;
    let mut defender_index = 0_u32;
    let mut attacker_troops = Vec::new();
    let mut defender_troops = Vec::new();
    if let Some(value) = world.systems.get(system) {
        for troop_key in &value.ground_units {
            let Some(troop) = world.troops.get(*troop_key) else {
                continue;
            };
            if troop.regiment_strength <= 0 {
                continue;
            }
            if troop.is_alliance == attacker_is_alliance {
                attacker_index += 1;
                attacker_troops.push((
                    *troop_key,
                    format!("Attacker Regiment {attacker_index}"),
                    troop.regiment_strength,
                ));
            } else {
                defender_index += 1;
                defender_troops.push((
                    *troop_key,
                    format!("Defender Regiment {defender_index}"),
                    troop.regiment_strength,
                ));
            }
        }
    }
    (attacker_troops, defender_troops)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::dat::{ExplorationStatus, SectorGroup};
    use rebellion_core::ids::DatId;
    use rebellion_core::world::{
        CapitalShipClass, Character, ControlKind, Fleet, Sector, ShipInstance, System, TroopUnit,
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

    fn post_battle_ground_fixture(
        include_defender: bool,
    ) -> (
        GameWorld,
        VictoryState,
        SystemKey,
        FleetKey,
        TroopKey,
        TroopTransportState,
    ) {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(11),
            name: "Ground Test Sector".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let system = world.systems.insert(System {
            dat_id: DatId::new(12),
            name: "Ground Test System".into(),
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
            control: ControlKind::Controlled(Faction::Empire),
        });
        let ship_class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(13),
            name: "Transport".into(),
            is_alliance: true,
            hull: 100,
            troop_capacity: 1,
            bombardment_modifier: 10,
            ..CapitalShipClass::default()
        });
        let winner_fleet = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ShipInstance::new(ship_class, 100, true)],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        world.systems[system].fleets.push(winner_fleet);

        let attacker = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(14),
            is_alliance: true,
            regiment_strength: 100,
        });
        world.systems[system].ground_units.push(attacker);
        if include_defender {
            let defender = world.troops.insert(TroopUnit {
                class_dat_id: DatId::new(15),
                is_alliance: false,
                regiment_strength: 100,
            });
            world.systems[system].ground_units.push(defender);
        }

        let mut transport = TroopTransportState::default();
        transport
            .embark(&mut world, winner_fleet, &[attacker])
            .expect("fixture troop embarks");
        let victory = VictoryState::new(system, system);
        (world, victory, system, winner_fleet, attacker, transport)
    }

    #[test]
    fn played_space_victory_uses_shared_contested_ground_route() {
        let (mut world, victory, system, winner_fleet, attacker, mut transport) =
            post_battle_ground_fixture(true);
        let mut messages = MessageLog::default();

        let outcome = resolve_post_battle(
            &mut world,
            &victory,
            &mut transport,
            winner_fleet,
            system,
            41,
            &mut messages,
            PostBattleGroundMode::Interactive,
        );

        assert_eq!(outcome.landed_regiments, 1);
        assert_eq!(transport.carried_count(winner_fleet), 0);
        assert!(world.systems[system].ground_units.contains(&attacker));
        let PostBattleContinuation::GroundCombat(ground) = outcome.continuation else {
            panic!("contested landing must enter ground combat");
        };
        assert_eq!(ground.system, system);
        assert!(ground.attacker_is_alliance);
        assert_eq!(ground.regiments.len(), 2);
        assert!(messages
            .messages()
            .iter()
            .any(|message| message.text.contains("regiment(s) landed")));
    }

    // port: a fleet holding regiments loaded by order keeps them aboard
    // (`TroopTransportState::load`).
    #[test]
    fn a_held_fleet_lands_nothing_where_it_loaded() {
        let (mut world, _, system, fleet, attacker, mut transport) =
            post_battle_ground_fixture(false);
        transport
            .disembark_all(&mut world, fleet, system)
            .expect("fixture regiment lands");
        transport
            .load(&mut world, fleet, &[attacker])
            .expect("fixture troop loads");
        let mut messages = MessageLog::default();

        let landed =
            land_faction_cargo(&mut world, &mut transport, system, true, &mut messages, 41);

        assert_eq!(landed, 0);
        assert_eq!(transport.cargo(fleet), &[attacker]);
    }

    #[test]
    fn unopposed_space_victory_uses_shared_occupation_route() {
        let (mut world, victory, system, winner_fleet, attacker, mut transport) =
            post_battle_ground_fixture(false);
        let mut messages = MessageLog::default();

        let outcome = resolve_post_battle(
            &mut world,
            &victory,
            &mut transport,
            winner_fleet,
            system,
            42,
            &mut messages,
            PostBattleGroundMode::Automatic { rolls: &[] },
        );

        assert!(matches!(
            outcome.continuation,
            PostBattleContinuation::Galaxy
        ));
        assert_eq!(outcome.landed_regiments, 1);
        assert!(world.systems[system].ground_units.contains(&attacker));
        assert_eq!(
            world.systems[system].control,
            ControlKind::Controlled(Faction::Alliance)
        );
        assert!(messages.messages().iter().any(|message| {
            message.rail == Some(MessageRail::PopularSupport)
                && message.text.contains("occupied by Alliance")
        }));
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
                    rng_seed: 0x1111_0001,
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
                rng_seed: 0x1111_0002,
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
                rng_seed: 0x1111_0003,
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
        let mut session =
            BattleSession::new(&world, system, attacker, defender, true, 23, 0x1111_0004);
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
