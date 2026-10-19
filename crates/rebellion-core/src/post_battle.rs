//! The pass after a battle (`FUN_0051ddf0` → `FUN_00516ef0`): the winner's
//! system squadrons are checked in their seat and every fleet and system
//! squadron of the loser looks for a place to go (event `0x307`,
//! `FUN_00515480`). `ghidra/notes/system-squadrons-combat.md`, sections 3
//! and 3b.

use crate::blockade::BlockadeSystem;
use crate::carriage::ProductRoom;
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

/// Trace what the pass did with one object, and for which winner.
fn trace_relocation(
    world: &GameWorld,
    system: SystemKey,
    object: &str,
    outcome: &dyn std::fmt::Debug,
    winner_is_alliance: bool,
) {
    crate::trace::record(crate::trace::TraceMask::COMBAT, "relocation", || {
        serde_json::json!({
            "system": world.systems.get(system).map(|sys| sys.name.clone()),
            "object": object,
            "outcome": format!("{outcome:?}"),
            "winner": if winner_is_alliance { "alliance" } else { "empire" },
        })
    });
}

/// What became of one object the pass handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relocation {
    /// It stays where it is: a squadron's container still accepts it
    /// (chain stage 1), or a fleet's transit to its place was refused. A
    /// fleet never stays in its own system by the chain.
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
pub enum Place {
    System(SystemKey),
    Fleet(FleetKey),
}

/// The kind of object the chain places. A move of kind `0x204` lifts the
/// blockade refusal only for types `0x08`..`0x14`, fleets and regiments
/// (`FUN_005073d0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mover {
    Fleet,
    /// A squadron (`0x1c`), which may also board a fleet (stage 4).
    Squadron,
    /// A regiment (`0x10`-`0x13`).
    Regiment,
    /// Anything else: a ship, special force, facility or character.
    /// hyp: stages 4 and 5 accept none of them; a ship's or character's
    /// boarding of a fleet under `0x204` is not read.
    Other,
}

/// Run the pass for the battle at `system`, won by the side `winner`
/// (`pass_winner`). A squadron that boards a fleet takes a ship with room
/// besides the squadron `products` built for it. Log lines go to `log`.
/// Returns what became of each of the loser's fleets.
#[expect(
    clippy::too_many_arguments,
    reason = "The pass threads the world's movement, transport and product state through each object."
)]
pub fn run_pass(
    world: &mut GameWorld,
    movement: &mut MovementState,
    troop_transport: &mut TroopTransportState,
    products: &ProductRoom,
    system: SystemKey,
    winner_is_alliance: bool,
    tick: u64,
    log: &mut Vec<String>,
) -> Vec<(FleetKey, Relocation)> {
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
                products,
                system,
                class,
                winner_is_alliance,
                tick,
                log,
            );
        }
    }

    // FUN_004f78e0(obj, 1) on every usable fleet (mode 1, `FUN_004ffef0`,
    // `FUN_00516ef0.c:143`) and system squadron of the loser.
    let loser = !winner_is_alliance;
    let fleets: Vec<FleetKey> = world.systems.get(system).map_or_else(Vec::new, |value| {
        value
            .fleets
            .iter()
            .copied()
            .filter(|&fleet| {
                world
                    .fleets
                    .get(fleet)
                    .is_some_and(|f| f.is_alliance == loser && f.usable())
            })
            .collect()
    });
    let mut relocations = Vec::with_capacity(fleets.len());
    for fleet in fleets {
        let outcome = relocate_fleet(world, movement, troop_transport, system, fleet, tick, log);
        trace_relocation(world, system, "fleet", &outcome, winner_is_alliance);
        log.push(format!(
            "[battle] post-battle fleet={fleet:?} outcome={outcome:?}"
        ));
        relocations.push((fleet, outcome));
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
                products,
                system,
                class,
                loser,
                tick,
            );
            trace_relocation(world, system, "squadron", &outcome, winner_is_alliance);
            log.push(format!(
                "[battle] post-battle squadron class={class:?} outcome={outcome:?}"
            ));
            if outcome == Relocation::Stayed {
                // Every squadron of the class finds the same seat.
                break;
            }
        }
    }
    relocations
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
    products: &ProductRoom,
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
        trace_relocation(
            world,
            system,
            "winner_squadron",
            &"Destroyed(container)",
            is_alliance,
        );
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
        products,
        system,
        class,
        is_alliance,
        tick,
    );
    trace_relocation(world, system, "winner_squadron", &outcome, is_alliance);
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
    let Some((is_alliance, death_stars)) = world.fleets.get(fleet).map(|value| {
        (
            value.is_alliance,
            crate::movement::death_stars_in(world, value),
        )
    }) else {
        return Relocation::Stayed;
    };
    // A fleet boards no fleet (stage 4 is a squadron's). Stage 1 is skipped:
    // VM, `empire-day660-deathstar-pre-alert.sav`, a loser at its own held
    // Carida (day 669) and Ando (day 711, auto-resolved) ended at Halowan, so
    // the pass does not keep a fleet in its own system. hyp: the cause is
    // stage 1 refusing the own container (`FUN_00554740`, `FUN_005571c0`);
    // see `ghidra/notes/morale-retreat.md`. A fleet with no other own system
    // is destroyed here only when `place` finds nothing, stages 6 and 7
    // (neutral, then enemy-held systems) included.
    let outcome = match place(
        world,
        troop_transport,
        movement,
        &ProductRoom::default(),
        system,
        None,
        is_alliance,
        Mover::Fleet,
        death_stars,
        true,
    ) {
        None => {
            destroy_fleet(world, troop_transport, fleet, tick);
            Relocation::Destroyed(2)
        }
        Some(Place::System(destination)) => {
            if fleet_speed_is_zero(world, fleet) {
                destroy_fleet(world, troop_transport, fleet, tick);
                Relocation::Destroyed(5)
            } else {
                match begin_faction_fleet_transit(movement, world, fleet, destination, is_alliance)
                {
                    Ok(_) => Relocation::Moved(destination),
                    Err(error) => {
                        trace_relocation(
                            world,
                            system,
                            "fleet",
                            &format_args!("Refused({destination:?}, {error:?})"),
                            !is_alliance,
                        );
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
#[expect(
    clippy::too_many_arguments,
    reason = "The move threads the world's movement, transport and product state."
)]
fn relocate_squadron(
    world: &mut GameWorld,
    movement: &mut MovementState,
    troop_transport: &mut TroopTransportState,
    products: &ProductRoom,
    system: SystemKey,
    class: FighterKey,
    is_alliance: bool,
    tick: u64,
) -> Relocation {
    let Some(found) = place(
        world,
        troop_transport,
        movement,
        products,
        system,
        None,
        is_alliance,
        Mover::Squadron,
        0,
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
    let blockaded = BlockadeSystem::blockaded_now(world);
    match troop_transport.move_squadron(
        world,
        movement,
        &blockaded,
        products,
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
///    not en route, as every object in a battle is; the origin system for
///    an uncompleted product, the transit origin for an en route one
///    (`ghidra/notes/death-star-strategic.md`, "The autoscrap request").
/// 3. Systems of its side that are not blockaded, nearest first
///    (`FUN_00554e80`). The battle system is not offered here (the nearness
///    row lists it first, `FUN_0058b060.c:19-27`, but the VM's withdrawals
///    left it; see [`retreat_destination`]); a squadron's stage 1 keeps it
///    there, a fleet's does not.
/// 4. Its side's fleets at those systems, in the same order
///    (`FUN_00554f80`); only a squadron can board one, on a ship with room
///    besides the squadron `products` built for it.
/// 5. Systems of its side that are blockaded (`FUN_005550e0`): a fleet's
///    or regiment's move lifts the blockade refusal, a squadron's does not
///    (`FUN_005073d0`).
///
/// 6. Neutral systems, side bits 3, nearest first (`FUN_005551e0`), then
/// 7. the other side's systems, nearest first (`FUN_005552e0`; skipped for a
///    neutral object): a fleet only.
///
/// Stages 3 and 5 accept the object only when the system is of the object's
/// side and, unless the object is a fleet, not destroyed (`FUN_00553410`), and has room for the
/// `death_stars` the object carries (`FUN_005571c0` → `FUN_00507ab0`,
/// [`crate::movement::death_star_room`]); its own container already holds
/// them. Stages 6 and 7 take a fleet at a system of any side, destroyed or
/// not, as `FUN_005073d0.c:103-113` resets the refusals for the fleet range;
/// a regiment, squadron or other object keeps them and gets no place there.
/// The destroy branch (`FUN_00515480` branch 6) follows when `None` comes
/// back. The VM or Wine check that confirms stages 6 and 7: a battle lost by
/// a side that holds only the battle system, on a map with neutral and enemy
/// systems; its fleet should end at the nearest neutral one.
///
/// `skip_stage_one` omits stage 1, as the withdrawal's search does
/// (`FUN_00516730.c:156` passes stage-1 flag 0); stages 3 and 5 never offer
/// `from`.
/// "Nearest" is the straight-line distance the original's nearness table
/// sorts by (`FUN_00554150` → `FUN_0058afd0`).
#[expect(
    clippy::too_many_arguments,
    reason = "The chain reads the world, the transport and movement states and the object's place."
)]
fn place(
    world: &GameWorld,
    troop_transport: &TroopTransportState,
    movement: &MovementState,
    products: &ProductRoom,
    from: SystemKey,
    hint: Option<SystemKey>,
    is_alliance: bool,
    mover: Mover,
    death_stars: usize,
    skip_stage_one: bool,
) -> Option<Place> {
    let squadron = mover == Mover::Squadron;
    let lifts_blockade = matches!(mover, Mover::Fleet | Mover::Regiment);
    // `FUN_005073d0.c:94-102` refuses a destroyed system only outside the
    // fleet range: a fleet may retreat to and stay at one.
    let lifts_destroyed = mover == Mover::Fleet;
    let blockaded = |key: SystemKey| {
        world
            .systems
            .get(key)
            .is_some_and(|value| BlockadeSystem::system_is_blockaded(world, value))
    };
    let accepts = |key: SystemKey| {
        world.systems.get(key).is_some_and(|value| {
            (lifts_destroyed || !value.is_destroyed)
                && value.control.is_controlled_by(side(is_alliance))
                && (lifts_blockade || !blockaded(key))
                && (key == from
                    || crate::movement::death_star_room(movement, world, key, death_stars))
        })
    };
    if !skip_stage_one && accepts(from) {
        return Some(Place::System(from));
    }
    if let Some(hint) = hint.filter(|&hint| accepts(hint)) {
        return Some(Place::System(hint));
    }
    let nearest = nearest_systems(world, from);
    if let Some(&key) = nearest.iter().find(|&&key| accepts(key) && !blockaded(key)) {
        return Some(Place::System(key));
    }
    if squadron {
        for &key in &nearest {
            let fleets = world
                .systems
                .get(key)
                .map_or(&[][..], |value| &value.fleets);
            if let Some(&fleet) = fleets.iter().find(|&&fleet| {
                troop_transport.squadron_fleet_accepts(
                    world,
                    movement,
                    products,
                    is_alliance,
                    fleet,
                )
            }) {
                return Some(Place::Fleet(fleet));
            }
        }
        return None;
    }
    if let Some(&key) = nearest.iter().find(|&&key| accepts(key) && blockaded(key)) {
        return Some(Place::System(key));
    }
    // Stages 6 and 7 offer a fleet any system, whoever holds it
    // (`FUN_005073d0.c:103-113` resets the side and destroyed refusals for a
    // fleet); a regiment, squadron or other object keeps them and finds none.
    if mover != Mover::Fleet {
        return None;
    }
    let has_room = |key: SystemKey| crate::movement::death_star_room(movement, world, key, death_stars);
    let held_by = |key: SystemKey| world.systems.get(key).and_then(|value| value.control.faction());
    let own = side(is_alliance);
    let is_enemy = |key: SystemKey| held_by(key) == Some(side(!is_alliance));
    // Stage 6: the neutral systems (side bits 3), nearest first.
    let neutral = nearest
        .iter()
        .find(|&&key| !is_enemy(key) && held_by(key) != Some(own) && has_room(key));
    // Stage 7: the other side's systems, nearest first.
    neutral
        .or_else(|| nearest.iter().find(|&&key| is_enemy(key) && has_room(key)))
        .map(|&key| Place::System(key))
}

/// Where a fleet that withdrew from the battle at `from` goes: the chain
/// without stage 1 (`FUN_00516730` reuses `FUN_005545e0`), never `from`
/// itself. VM, `empire-day660-deathstar-pre-alert.sav`: the Imperial fleet
/// withdrew from the Imperial-held Carida with the Alliance fleet in orbit
/// and went to Halowan after a countdown, though the nearness row lists
/// Carida first at distance 0. hyp: the mechanism is a refusal of the
/// fleet's own system by the candidate test (`FUN_005571c0`) or a blockade
/// bit still set when the retreat system is picked at the battle's start
/// (`FUN_0040b0e0`); settle by breaking on `FUN_005571c0` with the
/// candidate equal to the fleet's container. The chain runs on through
/// stages 5, 6 and 7 (`FUN_005545e0`), so a side that holds no other system
/// retreats to the nearest neutral, else the nearest enemy-held system, and
/// has no place only on a map with no other system. `death_stars` is the
/// room the retreating fleets ask of each candidate.
#[must_use]
pub fn retreat_destination(
    world: &GameWorld,
    troop_transport: &TroopTransportState,
    movement: &MovementState,
    from: SystemKey,
    is_alliance: bool,
    death_stars: usize,
) -> Option<SystemKey> {
    // A fleet boards no fleet (stage 4 is a squadron's), so no product's
    // room is read.
    match place(
        world,
        troop_transport,
        movement,
        &ProductRoom::default(),
        from,
        None,
        is_alliance,
        Mover::Fleet,
        death_stars,
        true,
    )? {
        Place::System(system) => Some(system),
        Place::Fleet(_) => None,
    }
}

/// Where an autoscrap request (`+0x50` bit 12, event `0x307`) sends an
/// object held by `container` (`FUN_00515480` → `FUN_005545e0`): the
/// chain from its container, with `hint` as stage 2. `None` destroys it
/// with reason 2. A squadron goes through [`request_squadron_place`],
/// whose fleets need room; any other object boards no fleet (stage 4 is a
/// squadron's).
#[must_use]
pub fn request_place(
    world: &GameWorld,
    troop_transport: &TroopTransportState,
    movement: &MovementState,
    container: SystemKey,
    hint: Option<SystemKey>,
    is_alliance: bool,
    mover: Mover,
) -> Option<Place> {
    debug_assert!(
        mover != Mover::Squadron,
        "a squadron's request reads the products' room"
    );
    place(
        world,
        troop_transport,
        movement,
        &ProductRoom::default(),
        container,
        hint,
        is_alliance,
        mover,
        0,
        false,
    )
}

/// [`request_place`] for a squadron: a fleet of stage 4 takes it when one
/// of its ships has room besides the squadrons aboard, on their way and
/// the squadron `products` built for it.
#[must_use]
pub fn request_squadron_place(
    world: &GameWorld,
    troop_transport: &TroopTransportState,
    movement: &MovementState,
    products: &ProductRoom,
    container: SystemKey,
    hint: Option<SystemKey>,
    is_alliance: bool,
) -> Option<Place> {
    place(
        world,
        troop_transport,
        movement,
        products,
        container,
        hint,
        is_alliance,
        Mover::Squadron,
        0,
        false,
    )
}

/// Every system but `from`, nearest to it first. The nearness table lists a
/// system among its own row at distance 0 (`FUN_0058b060.c:19-27`), but the
/// withdrawal never lands on the battle system (VM; see
/// [`retreat_destination`]), and a fleet's own container is stage 1's
/// candidate, not stage 3's.
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
        value
            .capital_ships
            .iter()
            .filter(|ship| ship.alive)
            .any(|ship| {
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
pub fn destroy_fleet(
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
            character.leave_abode();
        }
    }
    if let Some(system) = world.systems.get_mut(location) {
        system.fleets.retain(|&key| key != fleet);
    }
    world.remove_fleet(fleet);
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

const fn side(is_alliance: bool) -> Faction {
    if is_alliance {
        Faction::Alliance
    } else {
        Faction::Empire
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::{Character, ControlKind, Fleet, System};

    fn opponent(side: Faction) -> Faction {
        if side == Faction::Empire {
            Faction::Alliance
        } else {
            Faction::Empire
        }
    }

    fn system_at(world: &mut GameWorld, x: u16, control: ControlKind) -> SystemKey {
        world.systems.insert(System {
            x,
            control,
            ..Default::default()
        })
    }

    /// VM, original game, `empire-day660-deathstar-pre-alert.sav`: the
    /// Imperial fleet withdrew from the Imperial-held Carida (day 669, the
    /// Alliance fleet in orbit) after a countdown and the result read "The
    /// Imperial fleet has withdrawn to Halowan". The battle system is never
    /// the retreat system, whoever holds it; an enemy system nearer than the
    /// next own one is refused (`FUN_00553410`).
    #[test]
    fn a_withdrawing_fleet_at_its_own_system_retreats_to_the_nearest_other_own_system() {
        for side in [Faction::Empire, Faction::Alliance] {
            let mut world = GameWorld::default();
            let battle = system_at(&mut world, 0, ControlKind::Controlled(side));
            system_at(&mut world, 10, ControlKind::Controlled(opponent(side)));
            let own = system_at(&mut world, 50, ControlKind::Controlled(side));
            let destination = retreat_destination(
                &world,
                &TroopTransportState::default(),
                &MovementState::default(),
                battle,
                side == Faction::Alliance,
                0,
            );
            assert_eq!(destination, Some(own), "{side:?}");
        }
    }

    #[test]
    fn a_withdrawing_fleet_leaves_a_battle_system_its_side_does_not_hold_for_the_nearest_own() {
        for side in [Faction::Empire, Faction::Alliance] {
            let mut world = GameWorld::default();
            let battle = system_at(&mut world, 0, ControlKind::Controlled(opponent(side)));
            system_at(&mut world, 10, ControlKind::Controlled(opponent(side)));
            let own = system_at(&mut world, 50, ControlKind::Controlled(side));
            let destination = retreat_destination(
                &world,
                &TroopTransportState::default(),
                &MovementState::default(),
                battle,
                side == Faction::Alliance,
                0,
            );
            assert_eq!(destination, Some(own), "{side:?}");
        }
    }

    #[test]
    fn a_side_holding_only_the_battle_system_has_no_system_to_withdraw_to() {
        // With no other system on the map every stage comes up empty
        // (FUN_005545e0 stages 3 to 7); the battle system is never offered.
        for side in [Faction::Empire, Faction::Alliance] {
            let mut world = GameWorld::default();
            let battle = system_at(&mut world, 0, ControlKind::Controlled(side));
            let destination = retreat_destination(
                &world,
                &TroopTransportState::default(),
                &MovementState::default(),
                battle,
                side == Faction::Alliance,
                0,
            );
            assert_eq!(destination, None, "{side:?}");
        }
    }

    #[test]
    fn a_withdrawing_fleet_takes_the_own_system_nearest_by_straight_line_distance() {
        // FUN_00554150 sorts by FUN_0055d860's distance; the battle system
        // sits off the origin so each axis and the sum of squares matter.
        for side in [Faction::Empire, Faction::Alliance] {
            let mut world = GameWorld::default();
            let at = |world: &mut GameWorld, x, y, control| {
                world.systems.insert(System {
                    x,
                    y,
                    control,
                    ..Default::default()
                })
            };
            let battle = at(&mut world, 100, 70, ControlKind::Controlled(opponent(side)));
            for (x, y) in [(100, 124), (36, 87), (66, 67)] {
                at(&mut world, x, y, ControlKind::Controlled(side));
            }
            let nearest = at(&mut world, 131, 83, ControlKind::Controlled(side));
            let destination = retreat_destination(
                &world,
                &TroopTransportState::default(),
                &MovementState::default(),
                battle,
                side == Faction::Alliance,
                0,
            );
            assert_eq!(destination, Some(nearest), "{side:?}");
        }
    }

    #[test]
    fn a_withdrawing_fleet_prefers_an_unblockaded_own_system_to_a_nearer_blockaded_one() {
        // Stage 3 skips a blockaded system (+0x88 bit 0x20); stage 5
        // (FUN_005550e0) takes the blockaded own systems only when no
        // unblockaded one is left.
        for side in [Faction::Empire, Faction::Alliance] {
            let is_alliance = side == Faction::Alliance;
            let mut world = GameWorld::default();
            let battle = system_at(&mut world, 0, ControlKind::Controlled(side));
            let near = system_at(&mut world, 10, ControlKind::Controlled(side));
            let far = system_at(&mut world, 50, ControlKind::Controlled(side));
            let class = world.capital_ship_classes.insert(crate::world::CapitalShipClass {
                family: 0x14,
                ..Default::default()
            });
            let blockader = world.fleets.insert(Fleet {
                location: near,
                is_alliance: !is_alliance,
                capital_ships: vec![crate::world::ShipInstance::new(class, 100, !is_alliance)],
                ..Default::default()
            });
            world.systems[near].fleets.push(blockader);
            let retreat = |world: &GameWorld| {
                retreat_destination(
                    world,
                    &TroopTransportState::default(),
                    &MovementState::default(),
                    battle,
                    is_alliance,
                    0,
                )
            };
            assert!(BlockadeSystem::system_is_blockaded(&world, &world.systems[near]));
            assert_eq!(retreat(&world), Some(far), "{side:?}");
            world.systems[far].control = ControlKind::Controlled(opponent(side));
            assert_eq!(retreat(&world), Some(near), "{side:?} only the blockaded one");
        }
    }

    #[test]
    fn a_withdrawal_countdown_runs_from_the_transit_to_the_retreat_system() {
        // seat_morale: p5 is the transit to p4 (FUN_00516730.c:185-196); the
        // VM's Carida withdrawal waited 16-23 s before the jump.
        use crate::tactical::{apply, BattleSession};
        for side in [Faction::Empire, Faction::Alliance] {
            let is_alliance = side == Faction::Alliance;
            let mut world = GameWorld::default();
            let battle = system_at(&mut world, 0, ControlKind::Controlled(side));
            let refuge = system_at(&mut world, 30, ControlKind::Controlled(side));
            let mut fleet = |own: bool| {
                let class = world.capital_ship_classes.insert(crate::world::CapitalShipClass {
                    dat_id: crate::ids::DatId::new(0x1400_0001),
                    family: 0x14,
                    is_alliance: own,
                    is_empire: !own,
                    hull: 100,
                    ..Default::default()
                });
                world.fleets.insert(Fleet {
                    location: battle,
                    capital_ships: vec![crate::world::ShipInstance::new(class, 100, own)],
                    is_alliance: own,
                    ..Default::default()
                })
            };
            let alliance = fleet(true);
            let empire = fleet(false);
            world.systems[battle].fleets = vec![alliance, empire];
            world.systems[battle].fleets.sort_unstable();
            let mut session = BattleSession::new(&world, battle, alliance, empire, true, 1, 0x5eed);
            apply::seat_morale(
                &mut session,
                &world,
                &TroopTransportState::default(),
                &MovementState::new(),
                Some(true),
            );
            let index = usize::from(!is_alliance);
            assert!(session.morale.retreat_permitted[index], "{side:?}");
            assert!(
                session.morale.withdrawal[index].countdown_milliseconds > 0.0,
                "{side:?} {refuge:?}"
            );
        }
    }

    /// `FUN_005571c0` asks `FUN_00507ab0` for room: a retreat carrying a
    /// Death Star passes over a system of its side that holds one for the
    /// next; one carrying none takes the nearest.
    #[test]
    fn a_retreating_death_star_passes_over_a_system_that_holds_another() {
        for side in [Faction::Empire, Faction::Alliance] {
            let is_alliance = side == Faction::Alliance;
            let mut world = GameWorld::default();
            // The battle system is the other side's, so it is no candidate.
            let battle = system_at(&mut world, 0, ControlKind::Controlled(opponent(side)));
            let near = system_at(&mut world, 10, ControlKind::Controlled(side));
            let far = system_at(&mut world, 50, ControlKind::Controlled(side));
            let station = world
                .capital_ship_classes
                .insert(crate::world::CapitalShipClass {
                    family: 0x19,
                    ..Default::default()
                });
            let other = world.fleets.insert(Fleet {
                location: near,
                is_alliance,
                capital_ships: vec![crate::world::ShipInstance::new(station, 100, is_alliance)],
                ..Default::default()
            });
            world.systems[near].fleets.push(other);
            let retreat = |death_stars| {
                retreat_destination(
                    &world,
                    &TroopTransportState::default(),
                    &MovementState::default(),
                    battle,
                    is_alliance,
                    death_stars,
                )
            };

            assert_eq!(
                (retreat(1), retreat(0)),
                (Some(far), Some(near)),
                "{side:?}"
            );
        }
    }

    #[test]
    fn a_destroyed_fleet_leaves_its_system_and_the_other_side_takes_its_crew() {
        // hyp (destroy_fleet; or-q-emptied-fleet-crew-iz3z): the characters
        // aboard are taken as an emptied loser's fleet's are.
        for is_alliance in [false, true] {
            let mut world = GameWorld::default();
            let here = system_at(&mut world, 0, ControlKind::Uncontrolled);
            let crew = world.characters.insert(Character {
                is_alliance,
                ..Default::default()
            });
            let fleet = world.fleets.insert(Fleet {
                location: here,
                is_alliance,
                characters: vec![crew],
                ..Default::default()
            });
            world.systems[here].fleets.push(fleet);
            world.characters[crew].current_fleet = Some(fleet);
            destroy_fleet(&mut world, &mut TroopTransportState::default(), fleet, 7);
            assert!(!world.fleets.contains_key(fleet));
            assert!(world.systems[here].fleets.is_empty());
            let character = &world.characters[crew];
            assert!(character.is_captive);
            assert_eq!(character.captured_by, Some(side(!is_alliance)));
            assert_eq!(character.capture_tick, Some(7));
            assert_eq!(
                (character.current_system, character.current_fleet),
                (Some(here), None)
            );
        }
    }

    /// A fleet of one ship that can jump, at `at`.
    fn jumping_fleet(world: &mut GameWorld, at: SystemKey, is_alliance: bool) -> FleetKey {
        let class = world.capital_ship_classes.insert(crate::world::CapitalShipClass {
            family: 0x14,
            hull: 100,
            hyperdrive: 1,
            is_alliance,
            is_empire: !is_alliance,
            ..Default::default()
        });
        let fleet = world.fleets.insert(Fleet {
            location: at,
            is_alliance,
            capital_ships: vec![crate::world::ShipInstance::new(class, 100, is_alliance)],
            ..Default::default()
        });
        world.systems[at].fleets.push(fleet);
        fleet
    }

    /// VM, `empire-day660-deathstar-pre-alert.sav`: the Imperial fleets lost
    /// at their own Carida (withdrawn after Take Command, day 669) and at
    /// their own Ando (auto-resolved, day 711, so no tactical Withdraw) and
    /// both results read "The Imperial fleet has withdrawn to Halowan": the
    /// post-battle pass moves a loser out of its own held system, the
    /// tactical jump moves nothing (`FUN_0040c780`).
    #[test]
    fn a_loser_at_its_own_system_leaves_for_the_nearest_other_own_system() {
        for loser in [Faction::Empire, Faction::Alliance] {
            let loser_is_alliance = loser == Faction::Alliance;
            let mut world = GameWorld::default();
            let battle = system_at(&mut world, 0, ControlKind::Controlled(loser));
            system_at(&mut world, 10, ControlKind::Controlled(opponent(loser)));
            let refuge = system_at(&mut world, 30, ControlKind::Controlled(loser));
            let fleet = jumping_fleet(&mut world, battle, loser_is_alliance);
            jumping_fleet(&mut world, battle, !loser_is_alliance);
            let mut movement = MovementState::default();

            let relocations = run_pass(
                &mut world,
                &mut movement,
                &mut TroopTransportState::default(),
                &ProductRoom::default(),
                battle,
                !loser_is_alliance,
                0,
                &mut Vec::new(),
            );

            // The pass takes no "withdrew" input: a loser that never jumped
            // leaves just the same (Wine, day 669, Carida auto-resolved: the
            // fleet arrived at Halowan on day 672 and no battle reopened).
            assert_eq!(relocations, [(fleet, Relocation::Moved(refuge))], "{loser:?}");
            assert!(movement.is_in_transit(fleet), "{loser:?}");
            // With the loser away only the winner is there: the system ends
            // blockaded, as the results text reads.
            assert!(
                BlockadeSystem::system_is_blockaded(&world, &world.systems[battle]),
                "{loser:?}"
            );
        }
    }

    /// Stage 6 (`FUN_005551e0`) takes the systems whose side bits are 3,
    /// nearest first, before stage 7 (`FUN_005552e0`) takes the other side's;
    /// a fleet is accepted at a system of any side (`FUN_005073d0.c:103-113`).
    /// FUN_00516730 runs the same chain from stage 3, so the withdrawal's
    /// retreat system follows it too.
    #[test]
    fn a_side_holding_only_the_battle_system_retreats_to_the_nearest_neutral_system() {
        for side in [Faction::Empire, Faction::Alliance] {
            let is_alliance = side == Faction::Alliance;
            let mut world = GameWorld::default();
            let battle = system_at(&mut world, 0, ControlKind::Controlled(side));
            system_at(&mut world, 10, ControlKind::Controlled(opponent(side)));
            let neutral = system_at(&mut world, 30, ControlKind::Uncontrolled);
            system_at(&mut world, 60, ControlKind::Uncontrolled);
            let retreat = retreat_destination(
                &world,
                &TroopTransportState::default(),
                &MovementState::default(),
                battle,
                is_alliance,
                0,
            );
            assert_eq!(retreat, Some(neutral), "{side:?}");

            let fleet = jumping_fleet(&mut world, battle, is_alliance);
            jumping_fleet(&mut world, battle, !is_alliance);
            let relocations = run_pass(
                &mut world,
                &mut MovementState::default(),
                &mut TroopTransportState::default(),
                &ProductRoom::default(),
                battle,
                !is_alliance,
                0,
                &mut Vec::new(),
            );
            assert_eq!(relocations, [(fleet, Relocation::Moved(neutral))], "{side:?}");
        }
    }

    #[test]
    fn with_no_neutral_system_a_one_system_side_retreats_to_the_nearest_enemy_held_one() {
        for side in [Faction::Empire, Faction::Alliance] {
            let is_alliance = side == Faction::Alliance;
            let mut world = GameWorld::default();
            let battle = system_at(&mut world, 0, ControlKind::Controlled(side));
            let near = system_at(&mut world, 10, ControlKind::Controlled(opponent(side)));
            system_at(&mut world, 60, ControlKind::Controlled(opponent(side)));
            let retreat = retreat_destination(
                &world,
                &TroopTransportState::default(),
                &MovementState::default(),
                battle,
                is_alliance,
                0,
                );
            assert_eq!(retreat, Some(near), "{side:?}");

            let fleet = jumping_fleet(&mut world, battle, is_alliance);
            jumping_fleet(&mut world, battle, !is_alliance);
            let relocations = run_pass(
                &mut world,
                &mut MovementState::default(),
                &mut TroopTransportState::default(),
                &ProductRoom::default(),
                battle,
                !is_alliance,
                0,
                &mut Vec::new(),
            );
            assert_eq!(relocations, [(fleet, Relocation::Moved(near))], "{side:?}");
        }
    }

    /// `FUN_00516ef0.c:143` walks the loser's fleets in mode 1: a fleet whose
    /// only ship is on its own leg is not flagged, so it is neither moved nor
    /// destroyed whatever its speed.
    #[test]
    fn the_pass_leaves_a_loser_fleet_on_its_own_leg_alone() {
        for loser in [Faction::Empire, Faction::Alliance] {
            let loser_is_alliance = loser == Faction::Alliance;
            let mut world = GameWorld::default();
            let battle = system_at(&mut world, 0, ControlKind::Controlled(loser));
            let refuge = system_at(&mut world, 30, ControlKind::Controlled(loser));
            let travelling = jumping_fleet(&mut world, battle, loser_is_alliance);
            world.fleets[travelling].capital_ships[0].transit =
                Some(crate::world::ShipTransit {
                    from: refuge,
                    to: battle,
                    arrival_tick: 9,
                });
            let usable = jumping_fleet(&mut world, battle, loser_is_alliance);
            jumping_fleet(&mut world, battle, !loser_is_alliance);

            let relocations = run_pass(
                &mut world,
                &mut MovementState::default(),
                &mut TroopTransportState::default(),
                &ProductRoom::default(),
                battle,
                !loser_is_alliance,
                0,
                &mut Vec::new(),
            );

            assert_eq!(relocations, [(usable, Relocation::Moved(refuge))], "{loser:?}");
            assert!(world.fleets.contains_key(travelling), "{loser:?}");
            assert!(world.systems[battle].fleets.contains(&travelling), "{loser:?}");
        }
    }

    /// Nothing in `FUN_005551e0` or `FUN_005552e0` skips the fleet's own
    /// system; the refusal of the own container that moved the VM's losers
    /// out of their held systems (`FUN_005571c0`, hyp) is the candidate
    /// test every stage shares, so a loser at a neutral battle system leaves
    /// it for another neutral system, and has no place on a map with none.
    #[test]
    fn a_loser_at_a_neutral_battle_system_leaves_it_for_another_neutral_one() {
        for loser in [Faction::Empire, Faction::Alliance] {
            let loser_is_alliance = loser == Faction::Alliance;
            let mut world = GameWorld::default();
            let battle = system_at(&mut world, 0, ControlKind::Uncontrolled);
            let other = system_at(&mut world, 30, ControlKind::Uncontrolled);
            let fleet = jumping_fleet(&mut world, battle, loser_is_alliance);
            jumping_fleet(&mut world, battle, !loser_is_alliance);

            let relocations = run_pass(
                &mut world,
                &mut MovementState::default(),
                &mut TroopTransportState::default(),
                &ProductRoom::default(),
                battle,
                !loser_is_alliance,
                0,
                &mut Vec::new(),
            );
            assert_eq!(relocations, [(fleet, Relocation::Moved(other))], "{loser:?}");

            let mut alone = GameWorld::default();
            let only = system_at(&mut alone, 0, ControlKind::Uncontrolled);
            let stuck = jumping_fleet(&mut alone, only, loser_is_alliance);
            jumping_fleet(&mut alone, only, !loser_is_alliance);
            let relocations = run_pass(
                &mut alone,
                &mut MovementState::default(),
                &mut TroopTransportState::default(),
                &ProductRoom::default(),
                only,
                !loser_is_alliance,
                0,
                &mut Vec::new(),
            );
            assert_eq!(relocations, [(stuck, Relocation::Destroyed(2))], "{loser:?}");
        }
    }

    /// Stage 6 takes a neutral system only: an own system that cannot take
    /// the retreating Death Star (one per system) is not a stage-6 candidate,
    /// so the fleet passes it for the farther neutral one.
    #[test]
    fn stage_six_takes_only_neutral_systems() {
        for side in [Faction::Empire, Faction::Alliance] {
            let is_alliance = side == Faction::Alliance;
            let mut world = GameWorld::default();
            // The battle system is the other side's, so it is no candidate.
            let battle = system_at(&mut world, 0, ControlKind::Controlled(opponent(side)));
            let near = system_at(&mut world, 10, ControlKind::Controlled(side));
            let neutral = system_at(&mut world, 50, ControlKind::Uncontrolled);
            let station = world
                .capital_ship_classes
                .insert(crate::world::CapitalShipClass {
                    family: 0x19,
                    ..Default::default()
                });
            let other = world.fleets.insert(Fleet {
                location: near,
                is_alliance,
                capital_ships: vec![crate::world::ShipInstance::new(station, 100, is_alliance)],
                ..Default::default()
            });
            world.systems[near].fleets.push(other);
            let retreat = retreat_destination(
                &world,
                &TroopTransportState::default(),
                &MovementState::default(),
                battle,
                is_alliance,
                1,
            );
            assert_eq!(retreat, Some(neutral), "{side:?}");
        }
    }

    /// `FUN_005073d0.c:94-102` refuses a destroyed system only for an object
    /// outside the fleet range, so a fleet may retreat to and stay at an own
    /// destroyed system (`death-star-strategic.md`), a squadron may not.
    #[test]
    fn a_fleet_retreats_to_an_own_destroyed_system_but_a_squadron_does_not() {
        for side in [Faction::Empire, Faction::Alliance] {
            let is_alliance = side == Faction::Alliance;
            let mut world = GameWorld::default();
            let battle = system_at(&mut world, 0, ControlKind::Controlled(opponent(side)));
            let wreck = system_at(&mut world, 10, ControlKind::Controlled(side));
            world.systems[wreck].is_destroyed = true;
            let whole = system_at(&mut world, 50, ControlKind::Controlled(side));
            let retreat = |mover| {
                place(
                    &world,
                    &TroopTransportState::default(),
                    &MovementState::default(),
                    &ProductRoom::default(),
                    battle,
                    None,
                    is_alliance,
                    mover,
                    0,
                    true,
                )
            };
            assert_eq!(retreat(Mover::Fleet), Some(Place::System(wreck)), "{side:?}");
            assert_eq!(retreat(Mover::Squadron), Some(Place::System(whole)), "{side:?}");
        }
    }

    /// Stage 7 (`FUN_005552e0`) tests each enemy-held system through
    /// `FUN_005571c0`, whose placement refuses a system that already holds a
    /// Death Star when the retreating fleet carries one (`FUN_00507ab0`,
    /// `death-star-product.md`): the nearer enemy system without room is
    /// passed over for the farther one with room.
    #[test]
    fn stage_seven_passes_an_enemy_system_with_no_room_for_the_death_star() {
        for side in [Faction::Empire, Faction::Alliance] {
            let is_alliance = side == Faction::Alliance;
            let mut world = GameWorld::default();
            // The side holds only the battle system; nothing neutral exists.
            let battle = system_at(&mut world, 0, ControlKind::Controlled(side));
            let full = system_at(&mut world, 10, ControlKind::Controlled(opponent(side)));
            let roomy = system_at(&mut world, 50, ControlKind::Controlled(opponent(side)));
            let station = world
                .capital_ship_classes
                .insert(crate::world::CapitalShipClass {
                    family: 0x19,
                    ..Default::default()
                });
            let holder = world.fleets.insert(Fleet {
                location: full,
                is_alliance: !is_alliance,
                capital_ships: vec![crate::world::ShipInstance::new(station, 100, !is_alliance)],
                ..Default::default()
            });
            world.systems[full].fleets.push(holder);
            let retreat = |death_stars| {
                retreat_destination(
                    &world,
                    &TroopTransportState::default(),
                    &MovementState::default(),
                    battle,
                    is_alliance,
                    death_stars,
                )
            };

            assert_eq!(retreat(1), Some(roomy), "{side:?}");
            assert_eq!(retreat(0), Some(full), "{side:?}");
        }
    }

    #[test]
    fn a_refused_transit_traces_the_side_that_won_the_battle() {
        for loser in [Faction::Empire, Faction::Alliance] {
            let loser_is_alliance = loser == Faction::Alliance;
            let mut world = GameWorld::default();
            let battle = system_at(&mut world, 0, ControlKind::Controlled(loser));
            let refuge = system_at(&mut world, 30, ControlKind::Controlled(loser));
            let fleet = jumping_fleet(&mut world, battle, loser_is_alliance);
            jumping_fleet(&mut world, battle, !loser_is_alliance);
            let mut movement = MovementState::default();
            // An order already held refuses the new departure.
            assert!(movement.order(fleet, battle, refuge, 5));
            crate::trace::set_mask(crate::trace::TraceMask::COMBAT);
            let _ = crate::trace::drain();

            let relocations = run_pass(
                &mut world,
                &mut movement,
                &mut TroopTransportState::default(),
                &ProductRoom::default(),
                battle,
                !loser_is_alliance,
                0,
                &mut Vec::new(),
            );
            let records = crate::trace::drain();
            crate::trace::set_mask(crate::trace::TraceMask::default());

            assert_eq!(relocations, [(fleet, Relocation::Stayed)], "{loser:?}");
            let refused: Vec<_> = records
                .iter()
                .filter(|record| {
                    record.details["outcome"]
                        .as_str()
                        .is_some_and(|outcome| outcome.starts_with("Refused"))
                })
                .collect();
            assert_eq!(refused.len(), 1, "{loser:?} {records:?}");
            let winner = if loser_is_alliance { "empire" } else { "alliance" };
            assert_eq!(refused[0].details["winner"], winner, "{loser:?}");
        }
    }

    #[test]
    fn a_loser_with_no_other_own_system_finds_no_place() {
        // The chain's "no candidate accepted" (FUN_00515480 branch 6)
        // destroys it with reason 2, reached only when stages 3 to 7 offer
        // nothing: here the map holds no other system.
        for loser in [Faction::Empire, Faction::Alliance] {
            let loser_is_alliance = loser == Faction::Alliance;
            let mut world = GameWorld::default();
            let battle = system_at(&mut world, 0, ControlKind::Controlled(loser));
            let fleet = jumping_fleet(&mut world, battle, loser_is_alliance);

            let relocations = run_pass(
                &mut world,
                &mut MovementState::default(),
                &mut TroopTransportState::default(),
                &ProductRoom::default(),
                battle,
                !loser_is_alliance,
                0,
                &mut Vec::new(),
            );

            assert_eq!(relocations, [(fleet, Relocation::Destroyed(2))], "{loser:?}");
            assert!(!world.fleets.contains_key(fleet), "{loser:?}");
        }
    }

    /// `FUN_00516ef0.c:123-137` checks each of the winner's system squadrons
    /// (`FUN_00556710`) and `:159-174` flags each of the loser's
    /// (`FUN_004f78e0(obj, 1)`), one call per squadron of each class of that
    /// side; every squadron of a class the chain moves leaves on its own.
    /// At a neutral system both sides' squadrons leave for a system of
    /// their own (`FUN_00556710` sends a winner's that the system does not
    /// accept through the chain).
    #[test]
    fn the_pass_handles_each_squadron_of_each_class_of_each_side_once() {
        use crate::world::FighterClass;

        for winner_is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let battle = system_at(&mut world, 0, ControlKind::default());
            for side in [Faction::Alliance, Faction::Empire] {
                system_at(&mut world, 30, ControlKind::Controlled(side));
            }
            let mut classes = Vec::new();
            for is_alliance in [winner_is_alliance, !winner_is_alliance] {
                for count in [2, 1] {
                    let class = world.fighter_classes.insert(FighterClass {
                        squadron_size: 6,
                        is_alliance,
                        ..FighterClass::default()
                    });
                    world.systems[battle].add_squadrons(class, count, is_alliance);
                    classes.push((class, count, is_alliance));
                }
            }
            let mut log = Vec::new();
            run_pass(
                &mut world,
                &mut MovementState::default(),
                &mut TroopTransportState::default(),
                &ProductRoom::default(),
                battle,
                winner_is_alliance,
                0,
                &mut log,
            );
            for (class, count, is_alliance) in classes {
                let kind = if is_alliance == winner_is_alliance {
                    "winner squadron"
                } else {
                    "squadron"
                };
                let lines: Vec<&String> = log
                    .iter()
                    .filter(|line| line.contains(&format!("post-battle {kind} class={class:?} ")))
                    .collect();
                assert_eq!(lines.len(), count as usize, "{winner_is_alliance}: {log:?}");
                assert!(
                    lines.iter().all(|line| line.contains("outcome=Moved")),
                    "{winner_is_alliance}: {log:?}"
                );
            }
            assert!(world.systems[battle].fighters.is_empty(), "{log:?}");
        }
    }
}
