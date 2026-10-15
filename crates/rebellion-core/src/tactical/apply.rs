//! Writing a finished battle back to the strategic world.

use crate::dat::Faction;
use crate::death_star::DeathStarState;
use crate::ids::{FleetKey, SystemKey};
use crate::movement::MovementState;
use crate::troop_transport::TroopTransportState;
use crate::victory::VictoryState;
use crate::world::GameWorld;

use super::{BattleSession, CombatWinner};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleReturn {
    pub winner: Option<CombatWinner>,
    pub winner_fleet: Option<FleetKey>,
    pub player_won: bool,
    /// Where the player's fleet that withdrew from the battle was sent.
    pub withdrawal: Option<SystemKey>,
}

/// Summarize a completed tactical session without mutating strategic state.
/// Auto-resolve uses this after its result has already been integrated.
pub fn summarize_results(session: &BattleSession) -> BattleReturn {
    BattleReturn {
        winner: session.winner,
        winner_fleet: match session.winner {
            Some(CombatWinner::Attacker) => session.attacker_fleet,
            Some(CombatWinner::Defender) => session.defender_fleet,
            Some(CombatWinner::Draw) | None => None,
        },
        player_won: match session.winner {
            Some(CombatWinner::Attacker) => session.player_is_attacker,
            Some(CombatWinner::Defender) => !session.player_is_attacker,
            Some(CombatWinner::Draw) | None => false,
        },
        withdrawal: None,
    }
}

/// Apply a finished battle session back to the strategic world: losses,
/// surviving squadrons, captured characters, emptied fleets and withdrawals.
/// Played, simulated and unwatched battles all return through here. Log
/// lines (`[battle] withdrawal ...`) are appended to `log`.
pub fn apply_results(
    session: &BattleSession,
    world: &mut GameWorld,
    troop_transport: &mut TroopTransportState,
    movement: &mut MovementState,
    log: &mut Vec<String>,
) -> BattleReturn {
    let mut withdrawal = None;
    // FUN_0040c780 -> FUN_0040d260 -> FUN_00503370 -> FUN_005032c0: the
    // squadrons the system held lose craft as carried ones do; each surviving
    // squadron returns to the system.
    if let Some(system) = world.systems.get_mut(session.system) {
        for (is_attacker, is_alliance) in [
            (true, session.attacker_is_alliance),
            (false, !session.attacker_is_alliance),
        ] {
            for entry in system
                .fighters
                .iter_mut()
                .filter(|entry| entry.is_alliance == is_alliance)
            {
                entry.count = session
                    .fighters
                    .iter()
                    .filter(|fighter| {
                        fighter.held_by_system
                            && fighter.is_attacker == is_attacker
                            && fighter.class_key == entry.class
                            && fighter.squad_count > 0
                    })
                    .count()
                    .try_into()
                    .unwrap_or(u32::MAX);
            }
        }
        system.fighters.retain(|entry| entry.count > 0);
    }

    // Every fleet of each side fought (FUN_0040b0e0), the lead fleet first.
    let mut fleets: Vec<(FleetKey, bool)> = Vec::new();
    for (fleet, is_attacker) in [
        (session.attacker_fleet, true),
        (session.defender_fleet, false),
    ]
    .into_iter()
    .filter_map(|(fleet, is_attacker)| Some((fleet?, is_attacker)))
    .chain(
        session
            .ships
            .iter()
            .filter_map(|ship| Some((ship.fleet?, ship.is_attacker))),
    )
    .chain(
        session
            .fighters
            .iter()
            .filter_map(|fighter| Some((fighter.fleet?, fighter.is_attacker))),
    ) {
        if !fleets.iter().any(|&(key, _)| key == fleet) {
            fleets.push((fleet, is_attacker));
        }
    }

    for (fleet_key, is_attacker) in fleets {
        // Roster indices, after the losses are removed, of the ships that
        // withdrew, and whether any survivor stayed to fight.
        let mut withdrawn: Vec<usize> = Vec::new();
        let mut any_stayed = false;
        if let Some(fleet) = world.fleets.get_mut(fleet_key) {
            if fleet.has_death_star {
                fleet.has_death_star = session.death_star.is_some_and(|death_star| {
                    death_star.is_attacker == is_attacker && !death_star.destroyed
                });
            }

            let mut left = vec![false; fleet.capital_ships.len()];
            for (roster_index, ship_inst) in fleet.capital_ships.iter_mut().enumerate() {
                if !ship_inst.alive {
                    continue;
                }
                if let Some(result) = session.ships.iter().find(|ship| {
                    ship.fleet == Some(fleet_key) && ship.fleet_ship_index == roster_index
                }) {
                    // A ship that withdrew (FUN_005af7a0) leaves with its hull.
                    left[roster_index] = result.retreated;
                    ship_inst.hull_current = result.hull_current.clamp(0, result.hull_max);
                    ship_inst.alive =
                        (result.alive || result.retreated) && ship_inst.hull_current > 0;
                    if !ship_inst.alive {
                        ship_inst.hull_current = 0;
                    }
                    if ship_inst.alive {
                        if let Some(class) = world.capital_ship_classes.get(ship_inst.class) {
                            ship_inst.set_hyperdrive_damage_hits(
                                class,
                                result.subsystem_damage.hyperdrive,
                            );
                        }
                    }
                }
            }
            for (survivor_index, withdrew) in fleet
                .capital_ships
                .iter()
                .zip(left)
                .filter(|(ship, _)| ship.alive)
                .map(|(_, withdrew)| withdrew)
                .enumerate()
            {
                if withdrew {
                    withdrawn.push(survivor_index);
                } else {
                    any_stayed = true;
                }
            }
            fleet.capital_ships.retain(|ship| ship.alive);

            let mut survivors = vec![0_u32; fleet.fighters.len()];
            for fighter in session
                .fighters
                .iter()
                .filter(|fighter| fighter.fleet == Some(fleet_key))
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
            crate::carriage::drop_lost_squadrons(fleet);
        }

        let is_empty = world
            .fleets
            .get(fleet_key)
            .is_none_or(crate::world::Fleet::is_empty);
        if is_empty {
            let is_loser = match session.winner {
                Some(CombatWinner::Attacker) => !is_attacker,
                Some(CombatWinner::Defender) => is_attacker,
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
        } else if !withdrawn.is_empty() {
            // port: how the original's import splits a fleet whose ships
            // partly withdrew is untraced. The ships that withdrew (and the
            // squadrons aboard them) leave as a fleet of their own, as Create
            // Fleet makes one; the ships that stayed hold the system.
            let leaving = if any_stayed {
                let is_alliance = world.fleets[fleet_key].is_alliance;
                crate::fleet_join::roster(world, fleet_key).and_then(|chosen| {
                    crate::fleet_join::create_fleet(
                        world,
                        movement,
                        troop_transport,
                        fleet_key,
                        &withdrawn,
                        chosen,
                        is_alliance,
                    )
                    .ok()
                })
            } else {
                Some(fleet_key)
            };
            if let Some(leaving) = leaving {
                let destination = withdraw_fleet(world, movement, troop_transport, leaving, log);
                // The results name where the player's fleet went; an equal
                // stalemate withdraws both sides, and the enemy's
                // destination must not take its place.
                if is_attacker == session.player_is_attacker {
                    withdrawal = destination.or(withdrawal);
                }
            }
        }
    }
    // FUN_0051ddf0 → FUN_00516ef0: the winner's system squadrons are
    // checked in their seat and the loser's fleets and squadrons look for a
    // place to go (`post_battle`).
    let recorded = match session.winner {
        Some(CombatWinner::Attacker) => Some(session.attacker_is_alliance),
        Some(CombatWinner::Defender) => Some(!session.attacker_is_alliance),
        Some(CombatWinner::Draw) | None => None,
    };
    if let Some(winner_is_alliance) =
        crate::post_battle::pass_winner(world, session.system, recorded)
    {
        crate::post_battle::run_pass(
            world,
            movement,
            troop_transport,
            session.system,
            winner_is_alliance,
            session.start_tick,
            log,
        );
    }
    troop_transport.destroy_untransportable_cargo(world);
    let mut result = BattleReturn {
        withdrawal,
        ..summarize_results(session)
    };
    // Every fleet a side has at the system fights, so the lead may be
    // destroyed while another of the winner's fleets holds the system; the
    // post-battle steps (bombardment, landing, occupation) follow that one.
    if result
        .winner_fleet
        .is_none_or(|fleet| !world.fleets.contains_key(fleet))
    {
        let winner_is_alliance = match session.winner {
            Some(CombatWinner::Attacker) => Some(session.attacker_is_alliance),
            Some(CombatWinner::Defender) => Some(!session.attacker_is_alliance),
            Some(CombatWinner::Draw) | None => None,
        };
        if let Some(winner_is_alliance) = winner_is_alliance {
            result.winner_fleet = world.systems.get(session.system).and_then(|system| {
                system.fleets.iter().copied().find(|&fleet| {
                    world
                        .fleets
                        .get(fleet)
                        .is_some_and(|fleet| fleet.is_alliance == winner_is_alliance)
                })
            });
        }
    }
    result
}

/// Send a fleet that withdrew from a battle (manual p. 151, Withdraw from
/// Battle; the Battle Alert's Retreat, p. 141) where the retreat chain
/// places it (`post_battle::retreat_destination`). A fleet with no place,
/// or a ship without a hyperdrive, stays.
fn withdraw_fleet(
    world: &mut GameWorld,
    movement: &mut MovementState,
    troop_transport: &TroopTransportState,
    fleet: FleetKey,
    log: &mut Vec<String>,
) -> Option<SystemKey> {
    let value = world.fleets.get(fleet)?;
    let (is_alliance, origin) = (value.is_alliance, value.location);
    let destination = crate::post_battle::retreat_destination(
        world,
        troop_transport,
        movement,
        origin,
        is_alliance,
    );
    if let Some(destination) = destination {
        let departed = crate::movement::begin_faction_fleet_transit(
            movement,
            world,
            fleet,
            destination,
            is_alliance,
        );
        log.push(format!(
            "[battle] withdrawal fleet={fleet:?} destination={destination:?} departed={}",
            departed.is_ok()
        ));
        if departed.is_ok() {
            return Some(destination);
        }
    }
    None
}

/// Clear persistent Death Star pointers after either tactical or automatic
/// combat has removed the manager-owned object from its fleet.
pub fn reconcile_death_star_result(
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
