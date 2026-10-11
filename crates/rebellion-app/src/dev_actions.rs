//! The world-changing developer commands: `Post message`, `Battle at`,
//! `Blockade` and `Give Death Star`. Each one goes through the production
//! code that the event it stands for would run (the message builders,
//! `apply_fleet_arrival` as the
//! app's own arrival step calls it, the tick's own combat and blockade
//! checks), so what it shows is the game's
//! behaviour. A command whose precondition fails is refused with the reason
//! and is never faked. `agent_docs/dev-commands.md` lists them.

use rebellion_core::dat::Faction;
use rebellion_core::ids::{FleetKey, SystemKey};
use rebellion_core::movement::{apply_fleet_arrival, ArrivalEvent, MovementState};
use rebellion_core::troop_transport::TroopTransportState;
use rebellion_core::world::GameWorld;
use rebellion_data::text_templates::TextTemplates;
use rebellion_render::message_log::{GameMessage, MessageLog};

use crate::dev_commands::MessageClass;

/// The system named `name`, ignoring case.
pub fn find_system(world: &GameWorld, name: &str) -> Result<SystemKey, String> {
    world
        .systems
        .iter()
        .find(|(_, system)| system.name.eq_ignore_ascii_case(name.trim()))
        .map(|(key, _)| key)
        .ok_or_else(|| format!("no system named {name:?}"))
}

/// Every system by name: its holder (`A`, `E`, `-` for neither) and, after
/// a slash, the sides with fleets there; for choosing a `Battle at` or
/// `Blockade` target. A system with squadrons adds the counts the system
/// holds and its fleets carry, Alliance then Empire.
pub fn list_systems(world: &GameWorld) -> String {
    let mut rows: Vec<String> = world
        .systems
        .iter()
        .map(|(key, system)| {
            let holder = match holder_is_alliance(world, key) {
                Some(true) => "A",
                Some(false) => "E",
                None => "-",
            };
            let fleets = match (side_has_fleet(world, key, true), side_has_fleet(world, key, false)) {
                (true, true) => "AE",
                (true, false) => "A",
                (false, true) => "E",
                (false, false) => "",
            };
            let aboard = |is_alliance: bool| -> u32 {
                system
                    .fleets
                    .iter()
                    .filter_map(|&fleet| world.fleets.get(fleet))
                    .filter(|fleet| fleet.is_alliance == is_alliance)
                    .flat_map(|fleet| &fleet.fighters)
                    .map(|entry| entry.count)
                    .sum()
            };
            let counts = [
                system.squadron_count(true),
                system.squadron_count(false),
                aboard(true),
                aboard(false),
            ];
            let mut row = format!("{} {holder}/{fleets}", system.name);
            if counts.iter().any(|&count| count > 0) {
                row.push_str(&format!(
                    " squadrons {}/{} fleets {}/{}",
                    counts[0], counts[1], counts[2], counts[3]
                ));
            }
            let yards = system
                .manufacturing_facilities
                .iter()
                .filter_map(|&facility| world.manufacturing_facilities.get(facility))
                .filter(|facility| facility.is_shipyard)
                .count();
            if yards > 0 {
                row.push_str(&format!(" yards {yards}"));
            }
            row
        })
        .collect();
    rows.sort();
    rows.join(", ")
}

/// The side holding `system`, or `None` when no single side does.
fn holder_is_alliance(world: &GameWorld, system: SystemKey) -> Option<bool> {
    match world.systems.get(system)?.control.faction() {
        Some(Faction::Alliance) => Some(true),
        Some(Faction::Empire) => Some(false),
        _ => None,
    }
}

/// File a message class at `system` through its builder. Refused when the
/// builder would file nothing on a rail (a neutral system, no blockading
/// fleet, no fleet to arrive).
pub fn post_message(
    world: &GameWorld,
    templates: &TextTemplates,
    log: &mut MessageLog,
    class: MessageClass,
    system: SystemKey,
    tick: u64,
    player_is_alliance: bool,
) -> Result<String, String> {
    let name = world.systems[system].name.clone();
    let holder = holder_is_alliance(world, system);
    let messages: Vec<GameMessage> = match class {
        MessageClass::UprisingBegan | MessageClass::UprisingEnded => crate::uprising_messages(
            world,
            templates,
            system,
            tick,
            class == MessageClass::UprisingBegan,
            String::new(),
        ),
        MessageClass::Blockade => crate::blockade_messages(world, templates, system, tick, String::new()),
        MessageClass::FleetArrival => {
            let (fleet, alliance) = fleet_at(world, system, player_is_alliance)
                .ok_or_else(|| format!("no fleet orbits {name} to arrive"))?;
            let fleet_name = world.fleet_name(fleet).unwrap_or_default().to_owned();
            vec![crate::fleet_arrival_message(
                templates,
                fleet,
                &fleet_name,
                system,
                &name,
                tick,
                alliance,
            )]
        }
        MessageClass::LoyaltyJoins => {
            let holder = holder.ok_or_else(|| format!("{name} has no holder to join"))?;
            crate::loyalty_messages(world, templates, system, tick, Some(holder), None)
        }
        MessageClass::LoyaltyNeutral => {
            let holder = holder.ok_or_else(|| format!("{name} has no holder to leave"))?;
            crate::loyalty_messages(world, templates, system, tick, None, Some(holder))
        }
    };
    let filed: Vec<GameMessage> = messages
        .into_iter()
        .filter(|message| message.rail.is_some() && message.display.is_some())
        .collect();
    if filed.is_empty() {
        return Err(format!(
            "{class:?} at {name} files nothing in the original (neutral system or no fleet)"
        ));
    }
    let titles: Vec<String> = filed
        .iter()
        .filter_map(|message| message.display.as_ref().map(|d| d.title.clone()))
        .collect();
    for message in filed {
        log.push(message);
    }
    Ok(format!("filed {}", titles.join(" | ")))
}

/// A fleet orbiting `system` and its side: the player's first, since only
/// the arriving side files the message.
fn fleet_at(world: &GameWorld, system: SystemKey, player_is_alliance: bool) -> Option<(FleetKey, bool)> {
    let fleets: Vec<(FleetKey, bool)> = world
        .systems
        .get(system)?
        .fleets
        .iter()
        .filter_map(|&key| world.fleets.get(key).map(|fleet| (key, fleet.is_alliance)))
        .collect();
    fleets
        .iter()
        .find(|(_, alliance)| *alliance == player_is_alliance)
        .or_else(|| fleets.first())
        .copied()
}

/// Bring one idle fleet of each side missing from `system` there.
pub fn battle_at(
    world: &mut GameWorld,
    movement: &MovementState,
    troop_transport: &mut TroopTransportState,
    system: SystemKey,
    tick: u64,
) -> Result<String, String> {
    let mut moved = Vec::new();
    for alliance in [true, false] {
        if let Some(note) = bring_side(world, movement, troop_transport, system, alliance, tick)? {
            moved.push(note);
        }
    }
    let held = match holder_is_alliance(world, system) {
        Some(true) => "held by the Alliance",
        Some(false) => "held by the Empire",
        None => "held by neither side",
    };
    Ok(if moved.is_empty() {
        format!("{held}; both sides already orbit it")
    } else {
        format!("{held}; {}", moved.join("; "))
    })
}

/// Bring one idle fleet of the side not holding `system` there, so the
/// simulation forms a blockade. Refused for a neutral system, or where the
/// holder has a fleet (that is a battle, not a blockade).
pub fn blockade(
    world: &mut GameWorld,
    movement: &MovementState,
    troop_transport: &mut TroopTransportState,
    system: SystemKey,
    tick: u64,
) -> Result<String, String> {
    let name = world.systems[system].name.clone();
    let holder = holder_is_alliance(world, system)
        .ok_or_else(|| format!("{name} has no holder to blockade"))?;
    if side_has_fleet(world, system, holder) {
        return Err(format!("the holder has a fleet at {name}; that is a battle"));
    }
    Ok(bring_side(world, movement, troop_transport, system, !holder, tick)?
        .unwrap_or_else(|| "the blockader already orbits it".into()))
}

fn side_has_fleet(world: &GameWorld, system: SystemKey, alliance: bool) -> bool {
    world.systems[system]
        .fleets
        .iter()
        .any(|&key| world.fleets.get(key).is_some_and(|f| f.is_alliance == alliance))
}

/// Move one idle fleet of a side to `system` through `apply_fleet_arrival`:
/// one with capital ships, not in transit, carrying no Death Star (its
/// battle is a separate path). `None` when the side is already there.
fn bring_side(
    world: &mut GameWorld,
    movement: &MovementState,
    troop_transport: &mut TroopTransportState,
    system: SystemKey,
    alliance: bool,
    tick: u64,
) -> Result<Option<String>, String> {
    if side_has_fleet(world, system, alliance) {
        return Ok(None);
    }
    let side = if alliance { "Alliance" } else { "Imperial" };
    let mut idle: Vec<(FleetKey, String)> = world
        .fleets
        .iter()
        .filter(|(key, fleet)| {
            fleet.is_alliance == alliance
                && !fleet.capital_ships.is_empty()
                && !fleet.has_death_star
                && !movement.is_in_transit(*key)
        })
        // A fleet whose system is missing is skipped, not indexed.
        .filter_map(|(key, fleet)| {
            Some((key, world.systems.get(fleet.location)?.name.clone()))
        })
        .collect();
    // Order by the fleet's name for a choice that repeats run to run.
    idle.sort_by_key(|(key, _)| world.fleet_name(*key).unwrap_or_default().to_owned());
    let (fleet, origin_name) = idle
        .into_iter()
        .next()
        .ok_or_else(|| format!("no idle {side} fleet with capital ships"))?;
    let origin = world.fleets[fleet].location;
    apply_fleet_arrival(
        world,
        movement,
        troop_transport,
        &ArrivalEvent {
            fleet,
            tick,
            origin,
            system,
            join: None,
        },
    )
    .ok_or("the arrival did not apply")?;
    let fleet_name = world.fleet_name(fleet).unwrap_or_default().to_owned();
    eprintln!(
        "[dev-command] moved {side} {fleet_name} from {origin_name} to {}",
        world.systems[system].name
    );
    Ok(Some(format!("moved {side} {fleet_name} from {origin_name}")))
}

/// Mark the named Imperial fleet as carrying the Death Star, as a completed
/// construction leaves it (`DeathStarState::death_star_fleet`). Refused for
/// a name no Imperial fleet has, or while another fleet carries it.
pub fn give_death_star(
    world: &mut GameWorld,
    death_star: &mut rebellion_core::death_star::DeathStarState,
    name: &str,
) -> Result<String, String> {
    if let Some(holder) = world.fleets.iter().find(|(_, fleet)| fleet.has_death_star).map(|(key, _)| key) {
        return Err(format!(
            "{} already carries the Death Star",
            world.fleet_name(holder).unwrap_or_default()
        ));
    }
    // Both sides number their fleets, so only Imperial names are searched.
    let fleet = world
        .fleets
        .iter()
        .filter(|(_, fleet)| !fleet.is_alliance)
        .map(|(key, _)| key)
        .find(|&key| {
            world
                .fleet_name(key)
                .is_some_and(|fleet_name| fleet_name.eq_ignore_ascii_case(name.trim()))
        })
        .ok_or_else(|| format!("no Imperial fleet named {name:?}"))?;
    world.fleets[fleet].has_death_star = true;
    death_star.death_star_fleet = Some(fleet);
    let system = world.systems.get(world.fleets[fleet].location).map_or("", |s| s.name.as_str());
    Ok(format!("{} at {system} carries the Death Star", world.fleet_name(fleet).unwrap_or_default()))
}
