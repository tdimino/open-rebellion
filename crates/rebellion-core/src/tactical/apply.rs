//! Writing a finished battle back to the strategic world.

use crate::dat::Faction;
use crate::ids::{FleetKey, SystemKey};
use crate::movement::MovementState;
use crate::troop_transport::TroopTransportState;
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

/// Seat each side's morale before a battle (`FUN_00599c90.c:99-186`): its
/// retreat permission (`side+0x80`) and its withdrawal threshold
/// (`side+0x24`). A computer side takes `FUN_004e6110`'s threshold
/// (`ai_records::battle_withdraw_threshold`), the human side `FUN_004865b0`'s
/// 0.0 (only a strength of 0 withdraws it). `human_is_attacker` is the side
/// a person commands, `None` when both are computer players.
/// port: a retreat system exists (`FUN_00516730` through `FUN_005545e0`) when
/// `post_battle::retreat_destination` finds a place for the side's fleet; it
/// is the same chain `post_battle::run_pass` sends the loser's fleets by.
pub fn seat_morale(
    session: &mut BattleSession,
    world: &GameWorld,
    troop_transport: &TroopTransportState,
    movement: &MovementState,
    human_is_attacker: Option<bool>,
) {
    for is_attacker in [true, false] {
        let side = usize::from(!is_attacker);
        let is_alliance = session.attacker_is_alliance == is_attacker;
        // FUN_00516730.c:93-114: the search runs for the first usable fleet of
        // the side (`FUN_004ffef0(system, 1, side)`) that holds a usable ship
        // able to travel (`FUN_00502e30(fleet, 1, side)`, slot `+0x34(0)`);
        // its Death Stars are the room each candidate is asked for
        // (`FUN_005513a0.c:68,77`). port: a side with no such fleet asks none
        // (the original then tries a system squadron).
        let death_stars = crate::combat::fighting_fleets(world, session.system, is_alliance)
            .into_iter()
            .filter_map(|key| world.fleets.get(key))
            .find(|fleet| {
                fleet.capital_ships.iter().any(|ship| {
                    ship.usable()
                        && world.capital_ship_classes.get(ship.class).is_some_and(|class| {
                            crate::movement::capital_ship_instance_speed(world, ship, class) != 0
                        })
                })
            })
            .map_or(0, |fleet| crate::movement::death_stars_in(world, fleet));
        let refuge = crate::post_battle::retreat_destination(
            world,
            troop_transport,
            movement,
            session.system,
            is_alliance,
            death_stars,
        );
        session.morale.retreat_permitted[side] = refuge.is_some();
        // The windowed countdown starts at the transit there (p5,
        // `FUN_00516730.c:185-196` through `FUN_0055d8a0` at the default
        // speed), 0 with no retreat system (`FUN_00516730.c:46`).
        let transit = refuge.map_or(0, |refuge| {
            let at = |key: SystemKey| world.systems.get(key).map_or((0, 0), |s| (s.x, s.y));
            crate::movement::transit_ticks_between(
                world,
                at(session.system),
                at(refuge),
                crate::movement::default_speed(world),
            )
        });
        session.morale.withdrawal[side].countdown_milliseconds = transit as f32
            * super::ORIGINAL_WITHDRAWAL_TRANSIT_SCALE
            * super::ORIGINAL_WITHDRAWAL_TRANSIT_MILLISECONDS;
        session.morale.threshold[side] = if human_is_attacker == Some(is_attacker) {
            0.0
        } else {
            crate::ai_records::battle_withdraw_threshold(
                world,
                movement,
                troop_transport,
                session.system,
                if is_alliance {
                    Faction::Alliance
                } else {
                    Faction::Empire
                },
            )
        };
    }
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

/// Where each squadron goes after the battle (`FUN_0040d320.c:54-85`):
/// `None` for one destroyed, `Some(None)` for one that becomes a child of
/// the system, and `Some(Some((fleet, tag)))` for one docked in a ship of
/// its side at the end ([`BattleSession::end_berths`]): a capital ship, or
/// the station's Death Star ship (`FUN_005ba670.c:15-17`). A squadron
/// docked in the other side's carrier falls to the system (the side bits
/// `+0x24 & 0xc0` differ). Docked carriers without a tag get one
/// ([`crate::carriage::tag_ship`]).
pub(super) fn squadron_homes(
    session: &BattleSession,
    world: &mut GameWorld,
) -> Vec<Option<Option<(FleetKey, u32)>>> {
    let berths = session.end_berths();
    session
        .fighters
        .iter()
        .zip(berths)
        .map(|(fighter, berth)| {
            if fighter.squad_count == 0 {
                return None;
            }
            let (fleet, index, is_alliance) = match berth {
                Some(super::TacticalBerth::Ship(ship)) => {
                    let ship = &session.ships[ship];
                    (ship.fleet, ship.fleet_ship_index, ship.identity.is_alliance)
                }
                Some(super::TacticalBerth::Station) => {
                    let Some(station) = session.death_star else {
                        return Some(None);
                    };
                    let Some((fleet, index)) = station.ship else {
                        return Some(None);
                    };
                    (Some(fleet), index, station.is_alliance)
                }
                None => return Some(None),
            };
            let fleet = fleet.filter(|_| is_alliance == fighter.identity.is_alliance);
            Some(fleet.and_then(|fleet| {
                crate::carriage::tag_ship(world, fleet, index).map(|tag| (fleet, tag))
            }))
        })
        .collect()
}

/// Apply a finished battle session back to the strategic world: losses,
/// surviving squadrons, captured characters and emptied fleets. The import
/// (`FUN_0040c780`) moves no ship or fleet, so a ship that jumped stays in
/// its fleet at the system; the loser's fleets leave through the
/// post-battle pass (`FUN_00516ef0.c:142-180`).
/// Played, simulated and unwatched battles all return through here. Log
/// lines (`[battle] withdrawal ...`) are appended to `log`. `products` is
/// the room the squadron and regiment products hold aboard each ship
/// (`crate::carriage::ProductRoom`), which a squadron the post-battle pass
/// moves onto a fleet must leave.
pub fn apply_results(
    session: &BattleSession,
    world: &mut GameWorld,
    troop_transport: &mut TroopTransportState,
    movement: &mut MovementState,
    products: &crate::carriage::ProductRoom,
    log: &mut Vec<String>,
) -> BattleReturn {
    // FUN_00502840, each lost ship's hook +0x11c: its characters go adrift
    // and take the injury roll that may destroy a minor (`crew_fates`),
    // drawn once as the battle ended (`BattleSession::settle_crew`).
    let settled = session
        .settled_crew
        .clone()
        .unwrap_or_else(|| crate::battle_summary::settle_crew(session, world));
    world.battle_rng = settled.rng;
    for (member, fate) in &settled.members {
        match fate {
            crate::battle_summary::CrewFate::Aboard => {}
            crate::battle_summary::CrewFate::Adrift => {
                if let Some(character) = world.characters.get_mut(member.key) {
                    character.adrift = true;
                }
            }
            crate::battle_summary::CrewFate::Killed => {
                if let Some(fleet) = world.fleets.get_mut(member.fleet) {
                    fleet.characters.retain(|&key| key != member.key);
                }
                if let Some(character) = world.characters.get_mut(member.key) {
                    character.mark_killed();
                }
            }
        }
    }
    let homes = squadron_homes(session, world);
    // FUN_0040c780 -> FUN_0040d260 -> FUN_00503370 -> FUN_005032c0: every
    // squadron loses craft as it did in the battle; FUN_0040d320 then makes
    // each surviving squadron left in no hangar a child of the system.
    if let Some(system) = world.systems.get_mut(session.system) {
        for is_alliance in [true, false] {
            let mut entries: Vec<crate::world::SystemSquadrons> = system
                .fighters
                .iter()
                .filter(|entry| entry.is_alliance == is_alliance)
                .map(|entry| crate::world::SystemSquadrons { count: 0, ..entry.clone() })
                .collect();
            for (fighter, _) in session
                .fighters
                .iter()
                .zip(&homes)
                .filter(|(fighter, home)| fighter.identity.is_alliance == is_alliance && **home == Some(None))
            {
                match entries.iter_mut().find(|entry| entry.class == fighter.class_key) {
                    Some(entry) => entry.count += 1,
                    None => entries.push(crate::world::SystemSquadrons {
                        class: fighter.class_key,
                        count: 1,
                        is_alliance,
                    }),
                }
            }
            system.fighters.retain(|entry| entry.is_alliance != is_alliance);
            system.fighters.extend(entries.into_iter().filter(|entry| entry.count > 0));
        }
    }

    // Every fleet of each side fought (FUN_0040b0e0), the lead fleet first.
    let mut fleets: Vec<FleetKey> = Vec::new();
    for fleet in [session.attacker_fleet, session.defender_fleet]
        .into_iter()
        .flatten()
        .chain(session.ships.iter().filter_map(|ship| ship.fleet))
        .chain(session.fighters.iter().filter_map(|fighter| fighter.fleet))
        .chain(session.death_star.and_then(|station| Some(station.ship?.0)))
    {
        if !fleets.contains(&fleet) {
            fleets.push(fleet);
        }
    }

    // FUN_0040d170: the Death Star ship takes the station's remaining hull,
    // truncated (`__ftol`); less is damage (`FUN_00501ae0`, reason 0xc),
    // which destroys it at none, more is repair (`FUN_00501b70`).
    let station = session.death_star.and_then(|station| {
        let (fleet, index) = station.ship?;
        Some((fleet, index, station.hull.max(0.0).trunc() as i32))
    });

    for fleet_key in fleets {
        if let Some(fleet) = world.fleets.get_mut(fleet_key) {
            for (roster_index, ship_inst) in fleet.capital_ships.iter_mut().enumerate() {
                if !ship_inst.alive {
                    continue;
                }
                if let Some(result) = session.ships.iter().find(|ship| {
                    ship.fleet == Some(fleet_key) && ship.fleet_ship_index == roster_index
                }) {
                    // A ship that jumped (state 9) keeps its hull.
                    ship_inst.hull_current = result.hull_current.clamp(0, result.hull_max);
                    ship_inst.alive =
                        (result.alive || result.retreated) && ship_inst.hull_current > 0;
                    if !ship_inst.alive {
                        ship_inst.hull_current = 0;
                    }
                    if ship_inst.alive {
                        // FUN_0040cfc0: the shield, weapon and two further
                        // counters are written back with the drive counters.
                        let hits = result.subsystem_damage;
                        ship_inst.set_damage_counter(0, hits.shields);
                        ship_inst.set_damage_counter(1, hits.weapons);
                        ship_inst.set_damage_counter(2, hits.tractor);
                        ship_inst.set_damage_counter(3, hits.engines);
                        if let Some(class) = world.capital_ship_classes.get(ship_inst.class) {
                            ship_inst.set_hyperdrive_damage_hits(
                                class,
                                result.subsystem_damage.hyperdrive,
                            );
                        }
                    }
                }
            }
            if let Some((_, index, hull)) = station.filter(|&(fleet, ..)| fleet == fleet_key) {
                if let Some(ship) = fleet.capital_ships.get_mut(index) {
                    ship.hull_current = hull;
                    ship.alive = hull > 0;
                }
            }
            fleet.capital_ships.retain(|ship| ship.alive);

            // FUN_0040d320.c:54-85: a squadron docked at the end becomes a
            // child of its carrier's strategic ship, by the ship's tag.
            for entry in &mut fleet.fighters {
                entry.count = 0;
            }
            for (fighter, home) in session.fighters.iter().zip(&homes) {
                let Some(Some((home_fleet, carrier))) = *home else {
                    continue;
                };
                if home_fleet != fleet_key {
                    continue;
                }
                match fleet
                    .fighters
                    .iter_mut()
                    .find(|entry| entry.class == fighter.class_key && entry.carrier == carrier)
                {
                    Some(entry) => entry.count += 1,
                    None => fleet.fighters.push(crate::world::FighterEntry {
                        class: fighter.class_key,
                        count: 1,
                        carrier,
                    }),
                }
            }
            fleet.fighters.retain(|entry| entry.count > 0);
        }

        let is_empty = world
            .fleets
            .get(fleet_key)
            .is_none_or(crate::world::Fleet::is_empty);
        if is_empty {
            // The fleet's characters were children of its lost ships, set
            // adrift by their hook (FUN_004ef2e0 -> FUN_00534790); no path
            // takes them captive. With the fleet gone the system holds them
            // for the adrift sweep (`death_star::sweep_adrift`).
            let held = world
                .fleets
                .get(fleet_key)
                .map(|fleet| (fleet.characters.clone(), fleet.location));
            if let Some((characters, location)) = held {
                for character_key in characters {
                    if let Some(character) = world.characters.get_mut(character_key) {
                        character.adrift = true;
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
            world.remove_fleet(fleet_key);
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
    let pass_winner = crate::post_battle::pass_winner(world, session.system, recorded);
    crate::trace::record(crate::trace::TraceMask::COMBAT, "pass_winner", || {
        serde_json::json!({
            "system": session.system_name,
            "winner": pass_winner.map(|alliance| if alliance { "alliance" } else { "empire" }),
            "source": if recorded.is_some() { "recorded" } else { "holder_tie" },
        })
    });
    let relocations = pass_winner.map_or_else(Vec::new, |winner_is_alliance| {
        crate::post_battle::run_pass(
            world,
            movement,
            troop_transport,
            products,
            session.system,
            winner_is_alliance,
            session.start_tick,
            log,
        )
    });
    // The results name where the pass sent the player's fleet (`0x700d`).
    let player_is_alliance = session.player_is_attacker == session.attacker_is_alliance;
    let withdrawal = relocations.iter().find_map(|&(fleet, outcome)| match outcome {
        crate::post_battle::Relocation::Moved(destination)
            if world
                .fleets
                .get(fleet)
                .is_some_and(|fleet| fleet.is_alliance == player_is_alliance) =>
        {
            Some(destination)
        }
        _ => None,
    });
    troop_transport.destroy_untransportable_cargo(world);
    // A Death Star the battle or its post-battle pass destroyed loses its
    // usable bit (FUN_004f7b80 -> FUN_004f7410), whose event 0x301 reruns
    // bit 11 (FUN_00514510).
    crate::death_star::refresh_sectors(world);
    let mut result = BattleReturn {
        withdrawal,
        ..summarize_results(session)
    };
    // Every fleet a side has at the system fights, so the lead may be
    // destroyed, or left with no usable ship, while another of the winner's
    // fleets holds the system; the post-battle steps (bombardment, landing,
    // occupation) follow the side's first usable one, as the original walks
    // a system's fleets in mode 1 (FUN_00509530, FUN_004ffef0(system, 1,
    // side)).
    if result
        .winner_fleet
        .is_none_or(|fleet| !world.fleets.get(fleet).is_some_and(crate::world::Fleet::usable))
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
                        .is_some_and(|fleet| fleet.is_alliance == winner_is_alliance && fleet.usable())
                })
            });
        }
    }
    result
}
