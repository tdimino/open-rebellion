//! The pass after a battle (`FUN_0051ddf0` → `FUN_00516ef0`): the winner's
//! system squadrons are checked in their seat and every fleet and system
//! squadron of the loser looks for a place to go (event `0x307`,
//! `FUN_00515480`). `ghidra/notes/system-squadrons-combat.md`, sections 3
//! and 3b.

use std::collections::HashSet;

use crate::blockade::BlockadeSystem;
use crate::combat::side_present;
use crate::dat::Faction;
use crate::ids::{CharacterKey, FighterKey, FleetKey, SystemKey};
use crate::movement::{begin_faction_fleet_transit, capital_ship_instance_speed, MovementState};
use crate::troop_transport::{RegimentTarget, SquadronSource, TroopTransportState};
use crate::world::GameWorld;

/// The side whose objects stay put (`FUN_00516ef0`): the battle's recorded
/// winner (`FUN_00509ad0`), else, with both sides still present, the side
/// that did not arrive last (`+0x78` bits 6..7, written by `FUN_00508660`:
/// holder 1 gives 2, holders 2 and 0 give 1). With no recorded winner and
/// a side gone there is no pass.
#[must_use]
pub fn pass_winner(world: &GameWorld, system: SystemKey, recorded: Option<bool>) -> Option<bool> {
    if recorded.is_some() {
        return recorded;
    }
    (side_present(world, system, true) && side_present(world, system, false))
        .then(|| world.arrival_holders.get(&system) != Some(&true))
}

/// What became of one object the pass handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relocation {
    /// Its container still accepts it (chain stage 1).
    Stayed,
    /// It left for a system (`FUN_00556390`).
    Moved(SystemKey),
    /// It boarded a fleet of its side (chain stage 4).
    Boarded(FleetKey),
    /// Destroyed (`+0xac(reason)`): 2 when no place accepts it, 5 when it
    /// cannot travel to the place that does.
    Destroyed(u8),
}

/// A place the chain accepted (`FUN_005545e0`'s `D`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Place {
    System(SystemKey),
    Fleet(FleetKey),
}

/// The kind of object the chain places: a fleet, or one squadron the
/// system holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mover {
    Fleet,
    Squadron,
}

/// Run the pass for the battle at `system`, won by the side `winner`
/// (`pass_winner`). Log lines go to `log`.
pub fn run_pass(
    world: &mut GameWorld,
    movement: &mut MovementState,
    troop_transport: &mut TroopTransportState,
    system: SystemKey,
    winner_is_alliance: bool,
    tick: u64,
    log: &mut Vec<String>,
) {
    // FUN_00556710 on each of the winner's system squadrons first.
    for class in squadron_classes(world, system, winner_is_alliance) {
        let count = world.systems.get(system).map_or(0, |value| {
            value
                .fighters
                .iter()
                .filter(|entry| entry.class == class && entry.is_alliance == winner_is_alliance)
                .map(|entry| entry.count)
                .sum()
        });
        for _ in 0..count {
            check_winner_squadron(
                world,
                movement,
                troop_transport,
                system,
                class,
                winner_is_alliance,
                tick,
                log,
            );
        }
    }

    // FUN_004f78e0(obj, 1) on every fleet and system squadron of the loser.
    let loser = !winner_is_alliance;
    let fleets: Vec<FleetKey> = world.systems.get(system).map_or_else(Vec::new, |value| {
        value
            .fleets
            .iter()
            .copied()
            .filter(|&fleet| world.fleets.get(fleet).is_some_and(|f| f.is_alliance == loser))
            .collect()
    });
    for fleet in fleets {
        let outcome = relocate_fleet(world, movement, troop_transport, system, fleet, tick, log);
        log.push(format!("[battle] post-battle fleet={fleet:?} outcome={outcome:?}"));
    }
    for class in squadron_classes(world, system, loser) {
        let count = world.systems.get(system).map_or(0, |value| {
            value
                .fighters
                .iter()
                .filter(|entry| entry.class == class && entry.is_alliance == loser)
                .map(|entry| entry.count)
                .sum()
        });
        for _ in 0..count {
            let outcome = relocate_squadron(
                world,
                movement,
                troop_transport,
                system,
                class,
                loser,
                tick,
            );
            log.push(format!(
                "[battle] post-battle squadron class={class:?} outcome={outcome:?}"
            ));
            if outcome == Relocation::Stayed {
                // Every squadron of the class finds the same seat.
                break;
            }
        }
    }
}

/// `FUN_00556710`: a winner's squadron whose system is destroyed goes with
/// it (`FUN_004f8710`); one the system does not accept, a system of the
/// other side (slot `+0x54`, `FUN_004f6490`), is flagged (`FUN_00503510`)
/// and looks for a place as the loser's do. Placement under route kind
/// `0x3f0` lifts the blockade refusal, so a squadron of the system's side
/// stays.
/// port: stage 4 (an en-route squadron, or one whose origin `+0x38` lies
/// elsewhere, in a system the other side blockades) has no counterpart: the
/// port keeps no origin for a squadron the system holds.
#[expect(
    clippy::too_many_arguments,
    reason = "The pass threads the world's movement and transport state through each object."
)]
fn check_winner_squadron(
    world: &mut GameWorld,
    movement: &mut MovementState,
    troop_transport: &mut TroopTransportState,
    system: SystemKey,
    class: FighterKey,
    is_alliance: bool,
    tick: u64,
    log: &mut Vec<String>,
) {
    let Some(value) = world.systems.get(system) else {
        return;
    };
    if value.is_destroyed {
        if let Some(value) = world.systems.get_mut(system) {
            value.take_squadron(class, is_alliance);
        }
        log.push(format!(
            "[battle] post-battle winner squadron class={class:?} outcome=Destroyed(container)"
        ));
        return;
    }
    if value.control.is_controlled_by(side(is_alliance)) {
        return;
    }
    let outcome = relocate_squadron(
        world,
        movement,
        troop_transport,
        system,
        class,
        is_alliance,
        tick,
    );
    log.push(format!(
        "[battle] post-battle winner squadron class={class:?} outcome={outcome:?}"
    ));
}

/// `FUN_00515480` for a fleet: the chain's place, then the move, or
/// destruction with reason 2 (no place) or 5 (a place elsewhere and speed
/// 0). Then `FUN_004ff690`: each ship of a fleet that did not leave and
/// cannot travel is destroyed with reason 5.
fn relocate_fleet(
    world: &mut GameWorld,
    movement: &mut MovementState,
    troop_transport: &mut TroopTransportState,
    system: SystemKey,
    fleet: FleetKey,
    tick: u64,
    log: &mut Vec<String>,
) -> Relocation {
    let Some(is_alliance) = world.fleets.get(fleet).map(|value| value.is_alliance) else {
        return Relocation::Stayed;
    };
    let outcome = match place(
        world,
        troop_transport,
        movement,
        system,
        is_alliance,
        Mover::Fleet,
        false,
    ) {
        None => {
            destroy_fleet(world, troop_transport, fleet, tick);
            Relocation::Destroyed(2)
        }
        Some(Place::System(destination)) if destination == system => Relocation::Stayed,
        Some(Place::System(destination)) => {
            if fleet_speed_is_zero(world, fleet) {
                destroy_fleet(world, troop_transport, fleet, tick);
                Relocation::Destroyed(5)
            } else {
                match begin_faction_fleet_transit(movement, world, fleet, destination, is_alliance)
                {
                    Ok(_) => Relocation::Moved(destination),
                    Err(error) => {
                        log.push(format!(
                            "[battle] post-battle fleet={fleet:?} destination={destination:?} refused={error:?}"
                        ));
                        Relocation::Stayed
                    }
                }
            }
        }
        // A fleet's container is a system; stage 4 offers it none.
        Some(Place::Fleet(_)) => Relocation::Stayed,
    };
    if !matches!(outcome, Relocation::Moved(_) | Relocation::Destroyed(_)) {
        scrap_stranded_ships(world, troop_transport, fleet, tick);
    }
    outcome
}

/// `FUN_00515480` for one squadron the system holds. A squadron's speed is
/// `FUN_004f63f0`'s 100, so it always travels to an accepted place.
fn relocate_squadron(
    world: &mut GameWorld,
    movement: &mut MovementState,
    troop_transport: &mut TroopTransportState,
    system: SystemKey,
    class: FighterKey,
    is_alliance: bool,
    tick: u64,
) -> Relocation {
    let Some(found) = place(
        world,
        troop_transport,
        movement,
        system,
        is_alliance,
        Mover::Squadron,
        false,
    ) else {
        if let Some(value) = world.systems.get_mut(system) {
            value.take_squadron(class, is_alliance);
        }
        return Relocation::Destroyed(2);
    };
    let target = match found {
        Place::System(destination) if destination == system => return Relocation::Stayed,
        Place::System(destination) => RegimentTarget::System(destination),
        Place::Fleet(fleet) => RegimentTarget::Fleet(fleet),
    };
    let blockaded = blockaded_systems(world);
    match troop_transport.move_squadron(
        world,
        movement,
        &blockaded,
        SquadronSource::System {
            system,
            class,
            is_alliance,
        },
        target,
        tick,
    ) {
        Ok(_) => match found {
            Place::System(destination) => Relocation::Moved(destination),
            Place::Fleet(fleet) => Relocation::Boarded(fleet),
        },
        Err(_) => Relocation::Stayed,
    }
}

/// The retreat-candidate chain `FUN_005545e0`: the first place that would
/// take the object as a move of kind `0x204` (`FUN_005571c0`).
///
/// 1. Its own container, the battle system (`FUN_00554740`).
/// 2. The hint `H` (`FUN_00554ab0`): none for an object that is built and
///    not en route, as every object in a battle is.
/// 3. Systems of its side that are not blockaded, nearest first
///    (`FUN_00554e80`).
/// 4. Its side's fleets at those systems, in the same order
///    (`FUN_00554f80`); only a squadron can board one.
/// 5. Systems of its side that are blockaded (`FUN_005550e0`): a fleet's
///    move lifts the blockade refusal, a squadron's does not
///    (`FUN_005073d0`).
///
/// A system accepts the object only when it is of the object's side and not
/// destroyed (`FUN_00553410`). hyp: stages 6 and 7 (neutral and enemy
/// systems, `FUN_005551e0`, `FUN_005552e0`) accept neither a fleet nor a
/// squadron, since `FUN_00553410` refuses a system of another side.
/// "Nearest" is the straight-line distance the original's nearness table
/// sorts by (`FUN_00554150` → `FUN_0058afd0`).
fn place(
    world: &GameWorld,
    troop_transport: &TroopTransportState,
    movement: &MovementState,
    from: SystemKey,
    is_alliance: bool,
    mover: Mover,
    skip_current: bool,
) -> Option<Place> {
    let squadron = mover == Mover::Squadron;
    let blockaded = |key: SystemKey| {
        world
            .systems
            .get(key)
            .is_some_and(|value| BlockadeSystem::system_is_blockaded(world, value))
    };
    let accepts = |key: SystemKey| {
        world.systems.get(key).is_some_and(|value| {
            !value.is_destroyed
                && value.control.is_controlled_by(side(is_alliance))
                && !(squadron && blockaded(key))
        })
    };
    if !skip_current && accepts(from) {
        return Some(Place::System(from));
    }
    let nearest = nearest_systems(world, from);
    if let Some(&key) = nearest
        .iter()
        .find(|&&key| accepts(key) && !blockaded(key))
    {
        return Some(Place::System(key));
    }
    if squadron {
        for &key in &nearest {
            let fleets = world.systems.get(key).map_or(&[][..], |value| &value.fleets);
            if let Some(&fleet) = fleets.iter().find(|&&fleet| {
                troop_transport
                    .squadron_fleet_accepts(world, movement, is_alliance, fleet)
            }) {
                return Some(Place::Fleet(fleet));
            }
        }
        return None;
    }
    nearest
        .into_iter()
        .find(|&key| accepts(key) && blockaded(key))
        .map(Place::System)
}

/// Where a fleet that withdrew from the battle at `from` goes: the chain
/// without its own container (`FUN_00516730` reuses `FUN_005545e0`).
/// hyp: the withdrawal passes `FUN_005571c0`'s skip-current flag, so the
/// battle system itself is never chosen.
#[must_use]
pub fn retreat_destination(
    world: &GameWorld,
    troop_transport: &TroopTransportState,
    movement: &MovementState,
    from: SystemKey,
    is_alliance: bool,
) -> Option<SystemKey> {
    match place(
        world,
        troop_transport,
        movement,
        from,
        is_alliance,
        Mover::Fleet,
        true,
    )? {
        Place::System(system) => Some(system),
        Place::Fleet(_) => None,
    }
}

/// Every system but `from`, nearest to it first.
fn nearest_systems(world: &GameWorld, from: SystemKey) -> Vec<SystemKey> {
    let Some(origin) = world.systems.get(from) else {
        return Vec::new();
    };
    let (x, y) = (i64::from(origin.x), i64::from(origin.y));
    let mut systems: Vec<(i64, SystemKey)> = world
        .systems
        .iter()
        .filter(|&(key, _)| key != from)
        .map(|(key, value)| {
            let (dx, dy) = (i64::from(value.x) - x, i64::from(value.y) - y);
            (dx * dx + dy * dy, key)
        })
        .collect();
    systems.sort_unstable();
    systems.into_iter().map(|(_, key)| key).collect()
}

/// A fleet's speed is its slowest ship's (`FUN_004fd900`): zero when any
/// living ship cannot travel.
fn fleet_speed_is_zero(world: &GameWorld, fleet: FleetKey) -> bool {
    world.fleets.get(fleet).is_some_and(|value| {
        value.capital_ships.iter().filter(|ship| ship.alive).any(|ship| {
            world
                .capital_ship_classes
                .get(ship.class)
                .is_none_or(|class| capital_ship_instance_speed(world, ship, class) == 0)
        })
    })
}

/// `FUN_004ff690`: each completed ship of the fleet that is not en route and
/// cannot travel is destroyed with reason 5. A fleet left without ships
/// goes with them.
fn scrap_stranded_ships(
    world: &mut GameWorld,
    troop_transport: &mut TroopTransportState,
    fleet: FleetKey,
    tick: u64,
) {
    let stranded: Vec<bool> = world.fleets.get(fleet).map_or_else(Vec::new, |value| {
        value
            .capital_ships
            .iter()
            .map(|ship| {
                ship.alive
                    && world
                        .capital_ship_classes
                        .get(ship.class)
                        .is_some_and(|class| capital_ship_instance_speed(world, ship, class) == 0)
            })
            .collect()
    });
    if !stranded.contains(&true) {
        return;
    }
    if let Some(value) = world.fleets.get_mut(fleet) {
        let mut flags = stranded.iter();
        value
            .capital_ships
            .retain(|_| !flags.next().copied().unwrap_or(false));
        crate::carriage::drop_lost_squadrons(value);
    }
    if world
        .fleets
        .get(fleet)
        .is_some_and(crate::world::Fleet::is_empty)
    {
        destroy_fleet(world, troop_transport, fleet, tick);
    }
}

/// Remove a destroyed fleet. hyp: the characters aboard are taken by the
/// other side, as an emptied loser's fleet's are (`tactical::apply`); what
/// `+0xac(2)` and `+0xac(5)` do to a fleet's children is not traced.
fn destroy_fleet(
    world: &mut GameWorld,
    troop_transport: &mut TroopTransportState,
    fleet: FleetKey,
    tick: u64,
) {
    let Some(value) = world.fleets.get(fleet) else {
        return;
    };
    let (location, is_alliance) = (value.location, value.is_alliance);
    let characters: Vec<CharacterKey> = value.characters.clone();
    let captor = side(!is_alliance);
    for key in characters {
        if let Some(character) = world.characters.get_mut(key) {
            character.is_captive = true;
            character.captured_by = Some(captor);
            character.capture_tick = Some(tick);
            character.current_system = Some(location);
            character.current_fleet = None;
        }
    }
    if let Some(system) = world.systems.get_mut(location) {
        system.fleets.retain(|&key| key != fleet);
    }
    world.fleets.remove(fleet);
    troop_transport.destroy_untransportable_cargo(world);
}

/// The fighter classes the system holds squadrons of for a side.
fn squadron_classes(world: &GameWorld, system: SystemKey, is_alliance: bool) -> Vec<FighterKey> {
    let mut classes: Vec<FighterKey> = world.systems.get(system).map_or_else(Vec::new, |value| {
        value
            .fighters
            .iter()
            .filter(|entry| entry.is_alliance == is_alliance && entry.count > 0)
            .map(|entry| entry.class)
            .collect()
    });
    classes.dedup();
    classes
}

/// The systems blockaded now (`+0x88` bit `0x20`).
fn blockaded_systems(world: &GameWorld) -> HashSet<SystemKey> {
    world
        .systems
        .iter()
        .filter(|(_, value)| BlockadeSystem::system_is_blockaded(world, value))
        .map(|(key, _)| key)
        .collect()
}

const fn side(is_alliance: bool) -> Faction {
    if is_alliance {
        Faction::Alliance
    } else {
        Faction::Empire
    }
}
