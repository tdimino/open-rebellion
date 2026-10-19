//! Joining and splitting fleets (`ghidra/notes/fleet-join-split.md`).
//!
//! A move whose destination is a fleet hands the mover's capital ships to
//! that fleet (`FUN_004ffc90`, `FUN_004feca0`); a fleet that has held ships
//! disbands when the last one leaves (`FUN_004fe630`). Create Fleet (`0x270`)
//! moves the selected ships into a new fleet in their own system
//! (`FUN_00580b00`, `FUN_00509b40`).
//!
//! The original's fleet holds only capital ships, and fighters, regiments
//! and characters ride aboard them, so they go wherever their ship goes
//! (`crate::carriage`). The port keeps the rosters on the fleet, tagged with
//! their ship: a split takes the squadrons and regiments aboard the ships
//! that leave, and the characters when the ship they ride
//! (`crate::carriage::character_ship`) leaves. A ship moved to another
//! system joins its new fleet at once and travels on its own leg
//! (`ShipInstance::transit`); there is no moving fleet of ships.

use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::ids::{FleetKey, SystemKey};
use crate::movement::{
    capital_ship_instance_speed, death_star_room, fleet_move_confirms, transit_ticks_between,
    FleetDispatchError, MovementState,
};
use crate::troop_transport::TroopTransportState;
use crate::world::{Fleet, GameWorld, ShipTransit};

/// What a fleet order moves: a whole fleet, or some of its capital ships
/// (indices into [`Fleet::capital_ships`], valid for the fleet's
/// [`roster`] when they were chosen).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FleetMover {
    Fleet(FleetKey),
    Ships {
        fleet: FleetKey,
        ships: Vec<usize>,
        roster: u64,
    },
}

impl FleetMover {
    /// The fleet the mover belongs to.
    #[must_use]
    pub fn fleet(&self) -> FleetKey {
        match self {
            Self::Fleet(fleet) | Self::Ships { fleet, .. } => *fleet,
        }
    }
}

/// What a fleet order did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FleetOrderOutcome {
    /// The movers are members of this fleet: the target, or the side's
    /// spare at the destination system; those from another system are on
    /// their own legs to it.
    Joined(FleetKey),
    /// The selected ships now form this new fleet in their system.
    Created(FleetKey),
}

/// Why a fleet order is refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FleetOrderError {
    MissingFleet,
    /// The selection names no living ship of the fleet, or one twice.
    MissingShip,
    /// The fleet's ships changed after the selection was made, so its
    /// indices may name other ships.
    ShipsChanged,
    WrongFaction,
    /// A fleet or ship in hyperspace takes no orders (manual p. 121).
    InTransit,
    /// `FUN_00553aa0`: a move into the mover's own fleet (`0x26`).
    IntoItself,
    /// `FUN_00553410`: the destination fleet is another side's.
    OtherSideDestination,
    /// `FUN_00553410`: the destination fleet is en route.
    DestinationInTransit,
    /// The departure across systems is refused (`FUN_00555920`,
    /// `FUN_005287f0`).
    Dispatch(FleetDispatchError),
    /// `FUN_004f9860.c:45-50`: Create Fleet (`0x270`) names a ship that is
    /// not completed (`+0x50` bit 2), status `1`/`2`.
    Unfinished,
}

impl fmt::Display for FleetOrderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingFleet => formatter.write_str("fleet no longer exists"),
            Self::MissingShip => formatter.write_str("the ship is no longer in the fleet"),
            Self::ShipsChanged => formatter.write_str("the fleet's ships have changed"),
            Self::WrongFaction => formatter.write_str("fleet is not controlled by the player"),
            Self::InTransit => formatter.write_str("fleet is in hyperspace"),
            Self::IntoItself => formatter.write_str("a fleet cannot move into itself"),
            Self::OtherSideDestination => {
                formatter.write_str("the destination belongs to another side")
            }
            Self::DestinationInTransit => {
                formatter.write_str("the destination fleet is in hyperspace")
            }
            Self::Dispatch(error) => error.fmt(formatter),
            Self::Unfinished => formatter.write_str("the ship is still under construction"),
        }
    }
}

/// The player's own fleet, in orbit.
fn movable<'a>(
    world: &'a GameWorld,
    movement: &MovementState,
    fleet: FleetKey,
    expected_is_alliance: bool,
) -> Result<&'a Fleet, FleetOrderError> {
    let value = world
        .fleets
        .get(fleet)
        .ok_or(FleetOrderError::MissingFleet)?;
    if value.is_alliance != expected_is_alliance {
        return Err(FleetOrderError::WrongFaction);
    }
    if movement.is_in_transit(fleet) {
        return Err(FleetOrderError::InTransit);
    }
    Ok(value)
}

/// A fingerprint of `fleet`'s capital ships, in order. A ship order keeps
/// the one it was chosen against: a ship lost or gained since shifts the
/// indices, and the order is refused rather than move another ship.
#[must_use]
pub fn roster(world: &GameWorld, fleet: FleetKey) -> Option<u64> {
    let value = world.fleets.get(fleet)?;
    let mut hasher = DefaultHasher::new();
    for ship in &value.capital_ships {
        ship.class.hash(&mut hasher);
    }
    value.capital_ships.len().hash(&mut hasher);
    Some(hasher.finish())
}

/// The selected ships, sorted, each naming a living ship of the fleet once,
/// against the roster they were chosen from.
fn selected_ships(
    fleet: &Fleet,
    ships: &[usize],
    chosen: u64,
    now: u64,
) -> Result<Vec<usize>, FleetOrderError> {
    if chosen != now {
        return Err(FleetOrderError::ShipsChanged);
    }
    let mut sorted = ships.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    if sorted.is_empty()
        || sorted.len() != ships.len()
        || sorted.iter().any(|&index| {
            !fleet
                .capital_ships
                .get(index)
                .is_some_and(|ship| ship.alive)
        })
    {
        return Err(FleetOrderError::MissingShip);
    }
    Ok(sorted)
}

/// The mover's ships, checked: every ship for a whole fleet. Each is a
/// per-object order of its own (`0x204`, `0x270`), and `FUN_004f9860`
/// refuses one en route (status 4: its own leg) for every order, so the
/// order is refused (`FUN_0053c1a0`).
fn mover_ships(
    world: &GameWorld,
    source: &Fleet,
    mover: &FleetMover,
) -> Result<Vec<usize>, FleetOrderError> {
    let ships = match mover {
        FleetMover::Fleet(_) => (0..source.capital_ships.len()).collect(),
        FleetMover::Ships {
            fleet,
            ships,
            roster: chosen,
        } => selected_ships(
            source,
            ships,
            *chosen,
            roster(world, *fleet).ok_or(FleetOrderError::MissingFleet)?,
        )?,
    };
    if ships.iter().any(|&index| {
        source.capital_ships[index].alive && source.capital_ships[index].transit.is_some()
    }) {
        return Err(FleetOrderError::InTransit);
    }
    Ok(ships)
}

/// `FUN_00553410` for a fleet destination: the player's side, and not en
/// route (`FUN_004fe0d0`).
fn accept_destination<'a>(
    world: &'a GameWorld,
    movement: &MovementState,
    target: FleetKey,
    is_alliance: bool,
) -> Result<&'a Fleet, FleetOrderError> {
    let value = world
        .fleets
        .get(target)
        .ok_or(FleetOrderError::MissingFleet)?;
    if value.is_alliance != is_alliance {
        return Err(FleetOrderError::OtherSideDestination);
    }
    if movement.is_in_transit(target) {
        return Err(FleetOrderError::DestinationInTransit);
    }
    Ok(value)
}

/// Whether a fleet bound for `target` can join it on arriving at `system`:
/// it still exists, orbits there, is of the same side and is not en route.
#[must_use]
pub fn joins_on_arrival(
    world: &GameWorld,
    movement: &MovementState,
    target: FleetKey,
    system: SystemKey,
    is_alliance: bool,
) -> bool {
    accept_destination(world, movement, target, is_alliance)
        .is_ok_and(|value| value.location == system)
}

/// Move `from`'s ships, fighters, characters and cargo into `to` and remove
/// `from`: the fleet has given up its last ship (`FUN_004fe630`). The caller
/// has ended `from`'s transit.
pub fn merge_fleet_into(
    world: &mut GameWorld,
    troop_transport: &mut TroopTransportState,
    from: FleetKey,
    to: FleetKey,
) {
    if from == to || !world.fleets.contains_key(to) {
        return;
    }
    let Some(source) = world.remove_fleet(from) else {
        return;
    };
    if let Some(system) = world.systems.get_mut(source.location) {
        system.fleets.retain(|&fleet| fleet != from);
    }
    for &character in &source.characters {
        if let Some(value) = world.characters.get_mut(character) {
            value.current_fleet = Some(to);
        }
    }
    let target = &mut world.fleets[to];
    target.capital_ships.extend(source.capital_ships);
    for fighter in source.fighters {
        if let Some(entry) = target
            .fighters
            .iter_mut()
            .find(|entry| entry.class == fighter.class && entry.carrier == fighter.carrier)
        {
            entry.count = entry.count.saturating_add(fighter.count);
        } else {
            target.fighters.push(fighter);
        }
    }
    for character in source.characters {
        if !target.characters.contains(&character) {
            target.characters.push(character);
        }
    }
    troop_transport.transfer_fleet(world, from, to);
    world.name_flagship_fleets();
}

/// Move some of a fleet's ships into `to`, or the whole fleet when every
/// ship goes.
fn move_ships_into(
    world: &mut GameWorld,
    troop_transport: &mut TroopTransportState,
    from: FleetKey,
    ships: &[usize],
    to: FleetKey,
) {
    if ships.len() == world.fleets[from].capital_ships.len() {
        merge_fleet_into(world, troop_transport, from, to);
        return;
    }
    let source = &mut world.fleets[from];
    let characters_leave =
        crate::carriage::character_ship(source).is_some_and(|first| ships.contains(&first));
    let mut moved = Vec::with_capacity(ships.len());
    for &index in ships.iter().rev() {
        moved.push(source.capital_ships.remove(index));
    }
    moved.reverse();
    let tags: Vec<u32> = moved
        .iter()
        .map(|ship| ship.tag)
        .filter(|&tag| tag != 0)
        .collect();
    let mut squadrons = Vec::new();
    source.fighters.retain(|entry| {
        let aboard = tags.contains(&entry.carrier);
        if aboard {
            squadrons.push(entry.clone());
        }
        !aboard
    });
    let characters = if characters_leave {
        std::mem::take(&mut source.characters)
    } else {
        Vec::new()
    };
    let regiments: Vec<_> = troop_transport
        .cargo(from)
        .iter()
        .copied()
        .filter(|&troop| tags.contains(&troop_transport.carrier(troop)))
        .collect();
    troop_transport.transfer_some(from, to, &regiments);
    for &character in &characters {
        if let Some(value) = world.characters.get_mut(character) {
            value.current_fleet = Some(to);
        }
    }
    let target = &mut world.fleets[to];
    target.capital_ships.extend(moved);
    target.fighters.extend(squadrons);
    target.characters.extend(characters);
    world.name_flagship_fleets();
}

/// A new, empty fleet of `like`'s side in its system.
fn new_fleet(world: &mut GameWorld, like: FleetKey) -> FleetKey {
    let source = &world.fleets[like];
    let location = source.location;
    let fleet = world.insert_fleet(Fleet {
        location,
        capital_ships: Vec::new(),
        fighters: Vec::new(),
        characters: Vec::new(),
        is_alliance: source.is_alliance,
        bombarding: false,
    });
    if let Some(system) = world.systems.get_mut(location) {
        system.fleets.push(fleet);
        system.fleets.sort_unstable();
    }
    fleet
}

/// The selected ships as a fleet of their own: the source itself when every
/// ship is selected, else a new fleet holding them.
fn split_off(
    world: &mut GameWorld,
    troop_transport: &mut TroopTransportState,
    fleet: FleetKey,
    ships: &[usize],
) -> FleetKey {
    let created = new_fleet(world, fleet);
    move_ships_into(world, troop_transport, fleet, ships, created);
    created
}

/// Create Fleet (`0x270`): the selected ships form a new fleet in their
/// system (`FUN_00580b00` routes them to their own system, where
/// `FUN_00509b40` gives the side's spare fleet). port: the fleet is made
/// when the order runs; the original fills a spare (`FUN_0050be00`).
///
/// # Errors
/// Refuses another side's fleet, a fleet in hyperspace, a selection that
/// names no living ship of the fleet, one whose fleet's ships changed
/// since it was made, and an unfinished ship ([`refuse_unfinished`]).
pub fn create_fleet(
    world: &mut GameWorld,
    movement: &MovementState,
    troop_transport: &mut TroopTransportState,
    fleet: FleetKey,
    ships: &[usize],
    chosen: u64,
    expected_is_alliance: bool,
) -> Result<FleetKey, FleetOrderError> {
    let source = movable(world, movement, fleet, expected_is_alliance)?;
    let ships = mover_ships(
        world,
        source,
        &FleetMover::Ships {
            fleet,
            ships: ships.to_vec(),
            roster: chosen,
        },
    )?;
    refuse_unfinished(world, fleet, &ships)?;
    Ok(split_off(world, troop_transport, fleet, &ships))
}

/// `FUN_004f9860.c:45-50`: Create Fleet (`0x270`) refuses an object that is
/// not completed (`+0x50` bit 2); a Move (`0x204`) is exempt, so an
/// unfinished ship may change fleets, but not form one of its own.
fn refuse_unfinished(
    world: &GameWorld,
    fleet: FleetKey,
    ships: &[usize],
) -> Result<(), FleetOrderError> {
    let unfinished = world.fleets.get(fleet).is_some_and(|value| {
        ships
            .iter()
            .filter_map(|&index| value.capital_ships.get(index))
            .any(|ship| !ship.completed)
    });
    if unfinished {
        return Err(FleetOrderError::Unfinished);
    }
    Ok(())
}

/// A move whose destination is a fleet (`0x201` against a Fleet window's
/// `+0x70`). A fleet mover is first expanded into its ships
/// (`FUN_004ffc90.c:19-38`, mode 3). In the target's system they join it at
/// once; elsewhere each joins it at once too, a completed one in hyperspace
/// on its own leg until it arrives ([`send_ships`]; manual p. 122). The
/// route-1 Empire save, its day-123 Death Star played on: Fleet 1 Moved onto
/// Fleet 7 at Averam was on the order day an ISD in Fleet 7 with its own
/// en-route bit, and Fleet 1 was gone.
///
/// # Errors
/// Refuses another side's fleet, a mover or target in hyperspace, a move
/// into the mover's own fleet, a selection naming no ship of the fleet, and
/// a refused departure.
pub fn join_fleet(
    world: &mut GameWorld,
    movement: &mut MovementState,
    troop_transport: &mut TroopTransportState,
    mover: &FleetMover,
    target: FleetKey,
    expected_is_alliance: bool,
    now: u64,
) -> Result<FleetOrderOutcome, FleetOrderError> {
    let fleet = mover.fleet();
    let (ships, destination) = checked_join(world, movement, mover, target, expected_is_alliance)?;
    if world.fleets[fleet].location == destination {
        match mover {
            FleetMover::Fleet(_) => merge_fleet_into(world, troop_transport, fleet, target),
            FleetMover::Ships { .. } => {
                move_ships_into(world, troop_transport, fleet, &ships, target);
            }
        }
        return Ok(FleetOrderOutcome::Joined(target));
    }
    validate_ships_move(world, movement, fleet, &ships, destination)?;
    send_ships(world, troop_transport, fleet, &ships, target, now);
    Ok(FleetOrderOutcome::Joined(target))
}

/// The mover's checked ships and the target's system.
fn checked_join(
    world: &GameWorld,
    movement: &MovementState,
    mover: &FleetMover,
    target: FleetKey,
    expected_is_alliance: bool,
) -> Result<(Vec<usize>, SystemKey), FleetOrderError> {
    let fleet = mover.fleet();
    let source = movable(world, movement, fleet, expected_is_alliance)?;
    let ships = mover_ships(world, source, mover)?;
    if fleet == target {
        return Err(FleetOrderError::IntoItself);
    }
    let destination = accept_destination(world, movement, target, expected_is_alliance)?.location;
    Ok((ships, destination))
}

/// What [`join_fleet`] would refuse, without changing anything, and else
/// the target's system: the order's checks, then each ship's
/// ([`validate_ships_move`]) when the target lies in another system.
///
/// # Errors
/// [`join_fleet`]'s refusals.
pub fn validate_join(
    world: &GameWorld,
    movement: &MovementState,
    mover: &FleetMover,
    target: FleetKey,
    expected_is_alliance: bool,
) -> Result<SystemKey, FleetOrderError> {
    let (ships, destination) = checked_join(world, movement, mover, target, expected_is_alliance)?;
    let fleet = mover.fleet();
    if world.fleets[fleet].location != destination {
        validate_ships_move(world, movement, fleet, &ships, destination)?;
    }
    Ok(destination)
}

/// Whether a fleet's valid move onto `target` asks for confirmation:
/// Confirmed Move always does, and Move does when the fleet would leave a
/// blockaded system of its side (`FUN_00487cc0`, [`fleet_move_confirms`]).
/// A join within the fleet's own system departs nowhere.
#[must_use]
pub fn join_confirms(
    world: &GameWorld,
    blockaded: &HashSet<SystemKey>,
    fleet: FleetKey,
    target: FleetKey,
    confirmed: bool,
) -> bool {
    if confirmed {
        return true;
    }
    let (Some(mover), Some(target)) = (world.fleets.get(fleet), world.fleets.get(target)) else {
        return false;
    };
    mover.location != target.location && fleet_move_confirms(world, blockaded, fleet, false)
}

/// A capital ship's own move to a system (`0x201`): a system holds no
/// capital ships (`FUN_00507750`), so the ships go into the side's bit-1
/// spare there (`FUN_00552300.c:126-131` -> `FUN_005097d0`,
/// [`GameWorld::spare_fleet_or_new`]) at once, each on its own leg
/// ([`send_ships`]); in their own system that is Create Fleet. The route-1
/// Empire save, its day-123 Death Star played on: Galleon 2 Moved from
/// Coruscant to Averam was on the order day in a new fleet there with its
/// own en-route bit, and the unfinished Death Star Moved there later joined
/// that fleet at once.
///
/// # Errors
/// As [`create_fleet`], and a refused departure.
#[expect(
    clippy::too_many_arguments,
    reason = "The order's state, its ships with their roster, and its destination and side."
)]
pub fn move_ships_to_system(
    world: &mut GameWorld,
    movement: &mut MovementState,
    troop_transport: &mut TroopTransportState,
    fleet: FleetKey,
    ships: &[usize],
    chosen: u64,
    system: SystemKey,
    expected_is_alliance: bool,
    now: u64,
) -> Result<FleetOrderOutcome, FleetOrderError> {
    let ships = validate_ships_to_system(
        world,
        movement,
        fleet,
        ships,
        chosen,
        system,
        expected_is_alliance,
    )?;
    if world.fleets[fleet].location == system {
        return Ok(FleetOrderOutcome::Created(split_off(
            world,
            troop_transport,
            fleet,
            &ships,
        )));
    }
    let spare = world.spare_fleet_or_new(system, world.fleets[fleet].is_alliance);
    send_ships(world, troop_transport, fleet, &ships, spare, now);
    Ok(FleetOrderOutcome::Joined(spare))
}

/// What [`move_ships_to_system`] would refuse, without changing anything,
/// and else the ships it moves: the order's checks, then each ship's
/// ([`validate_ships_move`]) when `system` is another.
///
/// # Errors
/// [`move_ships_to_system`]'s refusals.
pub fn validate_ships_to_system(
    world: &GameWorld,
    movement: &MovementState,
    fleet: FleetKey,
    ships: &[usize],
    chosen: u64,
    system: SystemKey,
    expected_is_alliance: bool,
) -> Result<Vec<usize>, FleetOrderError> {
    let source = movable(world, movement, fleet, expected_is_alliance)?;
    let ships = mover_ships(
        world,
        source,
        &FleetMover::Ships {
            fleet,
            ships: ships.to_vec(),
            roster: chosen,
        },
    )?;
    if world.fleets[fleet].location == system {
        // In their own system the ships form a fleet: Create Fleet.
        refuse_unfinished(world, fleet, &ships)?;
    } else {
        validate_ships_move(world, movement, fleet, &ships, system)?;
    }
    Ok(ships)
}

/// The days a ship's per-object move (`0x204`) to `destination` shows in
/// the confirmation (`FUN_0053d200` → `FUN_00555d30`): none for a ship
/// that is not mobile (`+0x38`, `FUN_004f6410`: unfinished), which changes
/// container at once, else its own leg's at its own speed, as
/// [`send_ships`] gives it. port: none within its own system, as a
/// fleet's line.
#[must_use]
pub fn ship_move_days(
    world: &GameWorld,
    fleet: FleetKey,
    index: usize,
    destination: SystemKey,
) -> Option<u32> {
    let value = world.fleets.get(fleet)?;
    let ship = value.capital_ships.get(index)?;
    if !ship.completed || value.location == destination {
        return Some(0);
    }
    let class = world.capital_ship_classes.get(ship.class)?;
    let (from, to) = (
        world.systems.get(value.location)?,
        world.systems.get(destination)?,
    );
    Some(transit_ticks_between(
        world,
        (from.x, from.y),
        (to.x, to.y),
        capital_ship_instance_speed(world, ship, class),
    ))
}

/// Move (`0x201`) on one capital ship, which reaches it as the per-object
/// `0x204` (`move-order.md`). `FUN_004f9860.c:45-66` checks the object: its
/// side, not destroyed, not en route (bit 4: its own leg or its fleet's);
/// `0x204` is exempt from its completed test, so an unfinished ship may
/// move. The route-1 Empire save, its day-123 Death Star played on: the
/// unfinished Death Star's menu kept Move.
#[must_use]
pub fn ship_move_enabled(
    world: &GameWorld,
    movement: &MovementState,
    fleet: FleetKey,
    index: usize,
    is_alliance: bool,
) -> bool {
    crate::movement::fleet_move_enabled(movement, world, fleet, is_alliance)
        && world
            .fleets
            .get(fleet)
            .and_then(|value| value.capital_ships.get(index))
            .is_some_and(|ship| ship.alive && ship.transit.is_none())
}

/// Create Fleet (`0x270`) on one capital ship: [`ship_move_enabled`]'s
/// checks, and `FUN_004f9860.c:45-50` refuses an object not completed
/// (`+0x50` bit 2) for `0x270`. The same save: the unfinished Death Star's
/// menu greyed Create Fleet, a completed ship's kept it.
#[must_use]
pub fn create_fleet_enabled(
    world: &GameWorld,
    movement: &MovementState,
    fleet: FleetKey,
    index: usize,
    is_alliance: bool,
) -> bool {
    ship_move_enabled(world, movement, fleet, index, is_alliance)
        && world
            .fleets
            .get(fleet)
            .and_then(|value| value.capital_ships.get(index))
            .is_some_and(|ship| ship.completed)
}

/// What an AI order moves: the whole of `fleet` when `tags` is empty,
/// else its ships with these tags. `None` when the fleet is gone.
#[must_use]
pub fn mover_of_tags(world: &GameWorld, fleet: FleetKey, tags: &[u32]) -> Option<FleetMover> {
    if tags.is_empty() {
        return Some(FleetMover::Fleet(fleet));
    }
    let ships = world
        .fleets
        .get(fleet)?
        .capital_ships
        .iter()
        .enumerate()
        .filter(|(_, ship)| ship.tag != 0 && tags.contains(&ship.tag))
        .map(|(index, _)| index)
        .collect();
    Some(FleetMover::Ships {
        fleet,
        ships,
        roster: roster(world, fleet)?,
    })
}

/// The AI's Create Fleet (`0x270`, `crate::ai_regions`): the ships of
/// `fleet` with these tags as a fleet of their own in `to_system`, or in
/// their own system without one ([`move_ships_to_system`]).
///
/// # Errors
/// As [`move_ships_to_system`]; a tag no ship of the fleet carries is
/// left out.
pub fn create_fleet_of_tags(
    world: &mut GameWorld,
    movement: &mut MovementState,
    troop_transport: &mut TroopTransportState,
    fleet: FleetKey,
    tags: &[u32],
    to_system: Option<SystemKey>,
    expected_is_alliance: bool,
    now: u64,
) -> Result<FleetOrderOutcome, FleetOrderError> {
    let source = world
        .fleets
        .get(fleet)
        .ok_or(FleetOrderError::MissingFleet)?;
    let ships: Vec<usize> = source
        .capital_ships
        .iter()
        .enumerate()
        .filter(|(_, ship)| ship.tag != 0 && tags.contains(&ship.tag))
        .map(|(index, _)| index)
        .collect();
    let system = to_system.unwrap_or(source.location);
    let chosen = roster(world, fleet).ok_or(FleetOrderError::MissingFleet)?;
    move_ships_to_system(
        world,
        movement,
        troop_transport,
        fleet,
        &ships,
        chosen,
        system,
        expected_is_alliance,
        now,
    )
}

/// What a Move of `ships` to `destination` in another system would refuse,
/// past [`mover_ships`]'s checks: a deployed ship needs a speed
/// (`FUN_00555920.c:31-33`, `1`/`0x18`), the destination must resolve
/// (`1`/`0x22`), and the Death Stars it brings need room there
/// (`FUN_00551630`, [`crate::movement::death_star_room`]).
fn validate_ships_move(
    world: &GameWorld,
    movement: &MovementState,
    fleet: FleetKey,
    ships: &[usize],
    destination: SystemKey,
) -> Result<(), FleetOrderError> {
    let value = world
        .fleets
        .get(fleet)
        .ok_or(FleetOrderError::MissingFleet)?;
    if !world.systems.contains_key(value.location) {
        return Err(FleetOrderError::Dispatch(FleetDispatchError::MissingOrigin));
    }
    if !world.systems.contains_key(destination) {
        return Err(FleetOrderError::Dispatch(
            FleetDispatchError::MissingDestination,
        ));
    }
    let mut death_stars = 0;
    for ship in ships
        .iter()
        .filter_map(|&index| value.capital_ships.get(index))
    {
        let Some(class) = world.capital_ship_classes.get(ship.class) else {
            continue;
        };
        if ship.completed && capital_ship_instance_speed(world, ship, class) == 0 {
            return Err(FleetOrderError::Dispatch(FleetDispatchError::NoHyperdrive));
        }
        death_stars += usize::from(class.is_death_star());
    }
    if !death_star_room(movement, world, destination, death_stars) {
        return Err(FleetOrderError::Dispatch(FleetDispatchError::DeathStarRoom));
    }
    Ok(())
}

/// Each selected ship moved to `container` in another system: one
/// per-object move each (`0x204`), whose legs `FUN_0053d430` plans before
/// any runs, so all share the container. `FUN_00556390.c:12-28` puts each
/// into it at once; a completed ship (deployed, slot `+0x38`,
/// `FUN_004f6410`) travels there on its own leg at its own speed
/// (`FUN_00556430`: clock + `FUN_0055d8c0(distance, slot +0x34)`,
/// `FUN_00500820`), an unfinished one has no leg. What a ship carries goes
/// with it, and a fleet left with no ship disbands (`FUN_004fe630`). A
/// Death Star's departure clears its usable bit (`FUN_00556390` ->
/// `FUN_004f7640`), whose event `0x301` reruns bit 11 at once
/// (`FUN_00514510.c:149-158`, [`crate::death_star::refresh_sectors`]).
fn send_ships(
    world: &mut GameWorld,
    troop_transport: &mut TroopTransportState,
    fleet: FleetKey,
    ships: &[usize],
    container: FleetKey,
    now: u64,
) {
    let (Some(origin), Some(destination)) = (
        world.fleets.get(fleet).map(|value| value.location),
        world.fleets.get(container).map(|value| value.location),
    ) else {
        return;
    };
    let (Some(from), Some(to)) = (
        world.systems.get(origin).map(|value| (value.x, value.y)),
        world
            .systems
            .get(destination)
            .map(|value| (value.x, value.y)),
    ) else {
        return;
    };
    let mut death_star = false;
    for &index in ships {
        let Some(ship) = world.fleets[fleet].capital_ships.get(index) else {
            continue;
        };
        let Some(class) = world.capital_ship_classes.get(ship.class) else {
            continue;
        };
        if !(ship.alive && ship.completed) {
            continue;
        }
        death_star |= class.is_death_star();
        let speed = capital_ship_instance_speed(world, ship, class);
        let ticks = transit_ticks_between(world, from, to, speed);
        world.fleets[fleet].capital_ships[index].transit = Some(ShipTransit {
            from: origin,
            to: destination,
            arrival_tick: now + u64::from(ticks),
        });
    }
    move_ships_into(world, troop_transport, fleet, ships, container);
    if death_star {
        crate::death_star::refresh_sectors(world);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::carriage::ProductRoom;
    use crate::dat::{ExplorationStatus, SectorGroup};
    use crate::ids::{CapitalShipKey, DatId, TroopKey};
    use crate::world::{
        CapitalShipClass, Character, ControlKind, FighterEntry, Sector, ShipInstance, System,
        TroopUnit,
    };

    struct Setup {
        world: GameWorld,
        here: SystemKey,
        there: SystemKey,
        class: CapitalShipKey,
    }

    fn system(sector: crate::ids::SectorKey, x: u16) -> System {
        System {
            dat_id: DatId::new(0x9000_0000),
            name: format!("Sys{x}"),
            sector,
            x,
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
        }
    }

    fn setup() -> Setup {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(0x9200_0000),
            name: "Test".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let here = world.systems.insert(system(sector, 0));
        let there = world.systems.insert(system(sector, 50));
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            name: "Cruiser".into(),
            is_alliance: true,
            hull: 100,
            hyperdrive: 80,
            troop_capacity: 2,
            ..CapitalShipClass::default()
        });
        Setup {
            world,
            here,
            there,
            class,
        }
    }

    /// A fleet of `hulls.len()` ships at `system`, told apart by hull.
    fn fleet(setup: &mut Setup, system: SystemKey, hulls: &[i32], is_alliance: bool) -> FleetKey {
        let class = setup.class;
        let key = setup.world.fleets.insert(Fleet {
            location: system,
            capital_ships: hulls
                .iter()
                .map(|&hull| ShipInstance::new(class, hull, is_alliance))
                .collect(),
            fighters: vec![],
            characters: vec![],
            is_alliance,
            bombarding: false,
        });
        setup.world.systems[system].fleets.push(key);
        setup.world.systems[system].fleets.sort_unstable();
        key
    }

    fn hulls(world: &GameWorld, fleet: FleetKey) -> Vec<i32> {
        world.fleets[fleet]
            .capital_ships
            .iter()
            .map(|ship| ship.hull_current)
            .collect()
    }

    fn embark(setup: &mut Setup, transport: &mut TroopTransportState, fleet: FleetKey) -> TroopKey {
        let location = setup.world.fleets[fleet].location;
        let troop = setup.world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: true,
        });
        setup.world.systems[location].ground_units.push(troop);
        transport
            .embark(&mut setup.world, &ProductRoom::default(), fleet, &[troop])
            .unwrap();
        troop
    }

    // FUN_004ffc90 → FUN_004feca0: a fleet moved onto a fleet hands it every
    // ship; FUN_004fe630: the emptied fleet disbands. port: the fleet's
    // fighters, characters and cargo go with its ships.
    #[test]
    fn a_fleet_moved_onto_another_in_its_system_hands_over_everything_and_disbands() {
        let mut setup = setup();
        let here = setup.here;
        let mover = fleet(&mut setup, here, &[10, 20], true);
        let target = fleet(&mut setup, here, &[30], true);
        let fighter = setup.world.fighter_classes.insert(Default::default());
        setup.world.fleets[mover].fighters.push(FighterEntry {
            class: fighter,
            count: 2,
            carrier: 0,
        });
        setup.world.fleets[target].fighters.push(FighterEntry {
            class: fighter,
            count: 1,
            carrier: 0,
        });
        let admiral = setup.world.characters.insert(Character {
            name: "Admiral".into(),
            is_alliance: true,
            current_fleet: Some(mover),
            ..Default::default()
        });
        setup.world.fleets[mover].characters.push(admiral);
        reclass(&mut setup, mover, 0, 2, true);
        let mut transport = TroopTransportState::default();
        let troop = embark(&mut setup, &mut transport, mover);
        let mut movement = MovementState::new();

        let outcome = join_fleet(
            &mut setup.world,
            &mut movement,
            &mut transport,
            &FleetMover::Fleet(mover),
            target,
            true,
            0,
        );

        assert_eq!(outcome, Ok(FleetOrderOutcome::Joined(target)));
        let world = &setup.world;
        assert!(!world.fleets.contains_key(mover));
        assert_eq!(world.systems[here].fleets, [target]);
        assert_eq!(hulls(world, target), [30, 10, 20]);
        assert_eq!(world.fleets[target].fighters.len(), 1);
        assert_eq!(world.fleets[target].fighters[0].count, 3);
        assert_eq!(world.fleets[target].characters, [admiral]);
        assert_eq!(world.characters[admiral].current_fleet, Some(target));
        assert!(world.fleet_has_death_star(target));
        assert_eq!(transport.cargo(target), [troop]);
        assert!(movement.is_empty());
    }

    // port: fleet_name_bank's signature names follow their flagship class.
    #[test]
    fn a_fleet_joined_by_its_sides_flagship_class_takes_the_signature_name() {
        let mut setup = setup();
        setup
            .world
            .start_fleet_naming(crate::world::FleetNaming::Canonical);
        let here = setup.here;
        let target = fleet(&mut setup, here, &[30], true);
        let cruiser = setup.world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: crate::fleet_name_bank::MON_CALAMARI_CRUISER,
            ..CapitalShipClass::default()
        });
        let mover = fleet(&mut setup, here, &[10], true);
        setup.world.fleets[mover].capital_ships[0].class = cruiser;
        let mut transport = TroopTransportState::default();

        merge_fleet_into(&mut setup.world, &mut transport, mover, target);

        assert_eq!(setup.world.fleet_name(target), Some("Rebel Command Fleet"));
    }

    // FUN_004feca0 moves ships one by one; port: the fighters and cargo stay
    // with the fleet the ships left.
    // port: the merge's own guard; the callers never pass these.
    #[test]
    fn merging_a_fleet_into_itself_or_a_missing_fleet_changes_nothing() {
        let mut setup = setup();
        let here = setup.here;
        let kept = fleet(&mut setup, here, &[10, 20], true);
        let gone = fleet(&mut setup, here, &[30], true);
        setup.world.fleets.remove(gone);
        setup.world.systems[here].fleets.retain(|&key| key != gone);
        let mut transport = TroopTransportState::default();

        merge_fleet_into(&mut setup.world, &mut transport, kept, kept);
        merge_fleet_into(&mut setup.world, &mut transport, kept, gone);

        assert_eq!(hulls(&setup.world, kept), [10, 20]);
        assert_eq!(setup.world.systems[here].fleets, [kept]);
    }

    #[test]
    fn ships_moved_onto_another_fleet_leave_the_rest_of_their_fleet_behind() {
        let mut setup = setup();
        let here = setup.here;
        let mover = fleet(&mut setup, here, &[10, 20, 30], true);
        let target = fleet(&mut setup, here, &[40], true);
        let mut transport = TroopTransportState::default();
        let troop = embark(&mut setup, &mut transport, mover);

        let chosen = roster(&setup.world, mover).unwrap();
        let outcome = join_fleet(
            &mut setup.world,
            &mut MovementState::new(),
            &mut transport,
            &FleetMover::Ships {
                fleet: mover,
                ships: vec![2, 0],
                roster: chosen,
            },
            target,
            true,
            0,
        );

        assert_eq!(outcome, Ok(FleetOrderOutcome::Joined(target)));
        assert_eq!(hulls(&setup.world, mover), [20]);
        assert_eq!(hulls(&setup.world, target), [40, 10, 30]);
        // The regiment boarded the first of the equal ships (FUN_00552b10)
        // and, a child of that ship, goes where it goes.
        assert_eq!(transport.cargo(target), [troop]);
        assert!(transport.cargo(mover).is_empty());
    }

    // FUN_004fe630: a fleet whose last ship leaves disbands, so moving every
    // ship moves the fleet's contents with them.
    #[test]
    fn moving_every_ship_of_a_fleet_onto_another_disbands_it() {
        let mut setup = setup();
        let here = setup.here;
        let mover = fleet(&mut setup, here, &[10, 20], true);
        let target = fleet(&mut setup, here, &[40], true);
        let mut transport = TroopTransportState::default();
        let troop = embark(&mut setup, &mut transport, mover);

        let chosen = roster(&setup.world, mover).unwrap();
        join_fleet(
            &mut setup.world,
            &mut MovementState::new(),
            &mut transport,
            &FleetMover::Ships {
                fleet: mover,
                ships: vec![0, 1],
                roster: chosen,
            },
            target,
            true,
            0,
        )
        .unwrap();

        assert!(!setup.world.fleets.contains_key(mover));
        assert_eq!(hulls(&setup.world, target), [40, 10, 20]);
        assert_eq!(transport.cargo(target), [troop]);
    }

    // FUN_00553aa0: a move into the mover's own fleet is refused 0x26.
    #[test]
    fn a_fleet_or_its_ships_cannot_move_into_that_fleet() {
        let mut setup = setup();
        let here = setup.here;
        let mover = fleet(&mut setup, here, &[10, 20], true);
        let mut transport = TroopTransportState::default();
        let mut movement = MovementState::new();

        let chosen = roster(&setup.world, mover).unwrap();
        for selection in [
            FleetMover::Fleet(mover),
            FleetMover::Ships {
                fleet: mover,
                ships: vec![1],
                roster: chosen,
            },
        ] {
            assert_eq!(
                join_fleet(
                    &mut setup.world,
                    &mut movement,
                    &mut transport,
                    &selection,
                    mover,
                    true,
                    0
                ),
                Err(FleetOrderError::IntoItself)
            );
        }
        assert_eq!(hulls(&setup.world, mover), [10, 20]);
    }

    // FUN_004fe0d0 → FUN_00553410: another side's fleet, or one en route,
    // refuses the movers.
    #[test]
    fn another_sides_fleet_or_one_in_hyperspace_refuses_a_join() {
        let mut setup = setup();
        let (here, there) = (setup.here, setup.there);
        let mover = fleet(&mut setup, here, &[10], true);
        let enemy = fleet(&mut setup, here, &[20], false);
        let leaving = fleet(&mut setup, here, &[30], true);
        let mut movement = MovementState::new();
        assert!(crate::movement::begin_fleet_transit(
            &mut movement,
            &mut setup.world,
            leaving,
            there,
            5
        ));
        let mut transport = TroopTransportState::default();
        let mut join = |world: &mut GameWorld, target| {
            join_fleet(
                world,
                &mut movement,
                &mut transport,
                &FleetMover::Fleet(mover),
                target,
                true,
                0,
            )
        };

        assert_eq!(
            join(&mut setup.world, enemy),
            Err(FleetOrderError::OtherSideDestination)
        );
        assert_eq!(
            join(&mut setup.world, leaving),
            Err(FleetOrderError::DestinationInTransit)
        );
        assert_eq!(hulls(&setup.world, mover), [10]);
    }

    // Manual p. 121: a fleet in hyperspace cannot receive orders.
    #[test]
    fn a_fleet_in_hyperspace_takes_no_join_or_create_fleet_order() {
        let mut setup = setup();
        let (here, there) = (setup.here, setup.there);
        let mover = fleet(&mut setup, here, &[10, 20], true);
        let target = fleet(&mut setup, there, &[30], true);
        let mut movement = MovementState::new();
        assert!(crate::movement::begin_fleet_transit(
            &mut movement,
            &mut setup.world,
            mover,
            there,
            5
        ));
        let mut transport = TroopTransportState::default();

        assert_eq!(
            join_fleet(
                &mut setup.world,
                &mut movement,
                &mut transport,
                &FleetMover::Fleet(mover),
                target,
                true,
                0
            ),
            Err(FleetOrderError::InTransit)
        );
        let chosen = roster(&setup.world, mover).unwrap();
        assert_eq!(
            create_fleet(
                &mut setup.world,
                &movement,
                &mut transport,
                mover,
                &[0],
                chosen,
                true
            ),
            Err(FleetOrderError::InTransit)
        );
    }

    #[test]
    fn another_sides_fleet_takes_no_join_or_create_fleet_order() {
        // FUN_004f9860: the order's side.
        let mut setup = setup();
        let here = setup.here;
        let mover = fleet(&mut setup, here, &[10, 20], false);
        let target = fleet(&mut setup, here, &[30], false);
        let mut transport = TroopTransportState::default();

        assert_eq!(
            join_fleet(
                &mut setup.world,
                &mut MovementState::new(),
                &mut transport,
                &FleetMover::Fleet(mover),
                target,
                true,
                0
            ),
            Err(FleetOrderError::WrongFaction)
        );
        let chosen = roster(&setup.world, mover).unwrap();
        assert_eq!(
            create_fleet(
                &mut setup.world,
                &MovementState::new(),
                &mut transport,
                mover,
                &[0],
                chosen,
                true
            ),
            Err(FleetOrderError::WrongFaction)
        );
    }

    /// The tick a ship sent from `here` to `there` at `setup.class`'s speed
    /// arrives on, ordered on `now` (`FUN_00556430`: clock +
    /// `FUN_0055d8c0(distance, speed)`).
    fn arrival(setup: &Setup, now: u64) -> u64 {
        let (here, there) = (
            &setup.world.systems[setup.here],
            &setup.world.systems[setup.there],
        );
        let speed = crate::movement::capital_ship_speed(
            &setup.world,
            &setup.world.capital_ship_classes[setup.class],
        );
        now + u64::from(crate::movement::transit_ticks_between(
            &setup.world,
            (here.x, here.y),
            (there.x, there.y),
            speed,
        ))
    }

    // FUN_004ffc90.c:19-38 expands a fleet Moved onto a fleet into its
    // mode-3 ships, and FUN_00556390.c:12-28 reparents each into the target
    // at once; a completed (deployed) ship then travels on its own leg
    // (FUN_00556430), and the emptied fleet disbands (FUN_004fe630). The
    // route-1 Empire save, its day-123 Death Star played on: Fleet 1 (an
    // ISD with its squadrons and troops) Moved onto Fleet 7 at Averam was on
    // the order day an ISD in Fleet 7 with its own en-route bit (+0x50
    // 0x30876), its cargo aboard, and Fleet 1 gone.
    #[test]
    fn a_fleet_moved_onto_a_fleet_elsewhere_hands_it_each_ship_at_once_for_either_side() {
        for is_alliance in [true, false] {
            let mut setup = setup();
            let (here, there) = (setup.here, setup.there);
            let mover = fleet(&mut setup, here, &[10, 20], is_alliance);
            setup.world.fleets[mover].capital_ships[1].completed = false;
            let target = fleet(&mut setup, there, &[30], is_alliance);
            let mut movement = MovementState::new();

            let outcome = join_fleet(
                &mut setup.world,
                &mut movement,
                &mut TroopTransportState::default(),
                &FleetMover::Fleet(mover),
                target,
                is_alliance,
                100,
            );

            assert_eq!(
                outcome,
                Ok(FleetOrderOutcome::Joined(target)),
                "{is_alliance}"
            );
            assert!(!setup.world.fleets.contains_key(mover), "{is_alliance}");
            assert!(movement.is_empty(), "{is_alliance}");
            assert_eq!(hulls(&setup.world, target), [30, 10, 20], "{is_alliance}");
            let ships = &setup.world.fleets[target].capital_ships;
            assert_eq!(
                ships[1].transit,
                Some(crate::world::ShipTransit {
                    from: here,
                    to: there,
                    arrival_tick: arrival(&setup, 100),
                }),
                "{is_alliance}"
            );
            assert_eq!(ships[2].transit, None, "{is_alliance}");
        }
    }

    // The same per-ship rule for a selection: each selected ship joins the
    // target at once, on its own leg, with what it carries; the rest stay.
    #[test]
    fn ships_moved_onto_a_fleet_elsewhere_join_it_at_once_on_their_own_legs_for_either_side() {
        for is_alliance in [true, false] {
            let mut setup = setup();
            let (here, there) = (setup.here, setup.there);
            let mover = fleet(&mut setup, here, &[10, 20], is_alliance);
            let target = fleet(&mut setup, there, &[30], is_alliance);
            let mut movement = MovementState::new();
            let mut transport = TroopTransportState::default();
            let troop = setup.world.troops.insert(TroopUnit {
                class_dat_id: DatId::new(0x1000_0001),
                is_alliance,
            });
            setup.world.systems[here].ground_units.push(troop);
            transport
                .embark(&mut setup.world, &ProductRoom::default(), mover, &[troop])
                .unwrap();
            assert_eq!(
                transport.carrier(troop),
                setup.world.fleets[mover].capital_ships[0].tag,
                "{is_alliance}: the regiment rides the ship that stays"
            );

            let chosen = roster(&setup.world, mover).unwrap();
            let outcome = join_fleet(
                &mut setup.world,
                &mut movement,
                &mut transport,
                &FleetMover::Ships {
                    fleet: mover,
                    ships: vec![1],
                    roster: chosen,
                },
                target,
                is_alliance,
                100,
            );

            assert_eq!(
                outcome,
                Ok(FleetOrderOutcome::Joined(target)),
                "{is_alliance}"
            );
            assert_eq!(hulls(&setup.world, mover), [10], "{is_alliance}");
            assert_eq!(hulls(&setup.world, target), [30, 20], "{is_alliance}");
            assert_eq!(
                setup.world.fleets[target].capital_ships[1]
                    .transit
                    .map(|leg| leg.arrival_tick),
                Some(arrival(&setup, 100)),
                "{is_alliance}"
            );
            assert!(movement.is_empty(), "{is_alliance}");
            assert_eq!(transport.cargo(mover), [troop], "{is_alliance}");
            assert!(
                setup.world.systems[here].fleets.contains(&mover),
                "{is_alliance}"
            );
        }
    }

    /// Give `fleet`'s ship at `index` a class of `troop_capacity` (and the
    /// Death Star's record id when `death_star`).
    fn reclass(
        setup: &mut Setup,
        fleet: FleetKey,
        index: usize,
        troop_capacity: u32,
        death_star: bool,
    ) {
        let class = setup.world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(if death_star {
                crate::world::DEATH_STAR_CLASS_ID
            } else {
                0x2000_0000 + troop_capacity
            }),
            family: if death_star { 0x18 } else { 0x14 },
            is_alliance: setup.world.fleets[fleet].is_alliance,
            hull: 100,
            hyperdrive: 80,
            troop_capacity,
            ..CapitalShipClass::default()
        });
        setup.world.fleets[fleet].capital_ships[index].class = class;
    }

    // FUN_00552300 puts each regiment aboard a ship with room, so when only
    // the ships that leave have room, the split takes every regiment.
    #[test]
    fn a_split_takes_every_regiment_when_only_the_leaving_ships_hold_room() {
        let mut setup = setup();
        let here = setup.here;
        let source = fleet(&mut setup, here, &[10, 20, 30], true);
        reclass(&mut setup, source, 0, 0, false);
        let mut transport = TroopTransportState::default();
        let mut troops: Vec<_> = (0..4)
            .map(|_| {
                let troop = setup.world.troops.insert(TroopUnit {
                    class_dat_id: DatId::new(0x1000_0001),
                    is_alliance: true,
                });
                setup.world.systems[here].ground_units.push(troop);
                troop
            })
            .collect();
        troops.sort_unstable();
        transport
            .load(&mut setup.world, &ProductRoom::default(), source, &troops)
            .unwrap();
        let chosen = roster(&setup.world, source).unwrap();

        let created = create_fleet(
            &mut setup.world,
            &MovementState::new(),
            &mut transport,
            source,
            &[1, 2],
            chosen,
            true,
        )
        .unwrap();

        assert_eq!(hulls(&setup.world, source), [10]);
        assert!(transport.cargo(source).is_empty());
        assert_eq!(transport.cargo(created), troops.as_slice());
        assert!(transport
            .destroy_untransportable_cargo(&mut setup.world)
            .is_empty());
    }

    // FUN_00552b10: each regiment boards the ship with the most room, the
    // earlier on a tie (ships 0, 1, 0 at two each); a split takes those
    // aboard the ships that leave.
    #[test]
    fn a_split_takes_the_regiments_aboard_the_ships_that_leave() {
        let mut setup = setup();
        let here = setup.here;
        let source = fleet(&mut setup, here, &[10, 20], true);
        let mut transport = TroopTransportState::default();
        let aboard: Vec<_> = (0..3)
            .map(|_| embark(&mut setup, &mut transport, source))
            .collect();
        let chosen = roster(&setup.world, source).unwrap();

        let created = create_fleet(
            &mut setup.world,
            &MovementState::new(),
            &mut transport,
            source,
            &[1],
            chosen,
            true,
        )
        .unwrap();

        assert_eq!(transport.cargo(source), [aboard[0], aboard[2]]);
        assert_eq!(transport.cargo(created), [aboard[1]]);
    }

    // The original's squadrons and characters are children of a ship
    // (FUN_005039d0, FUN_00536da0), so they go where it goes; characters
    // ride the first living ship (crate::carriage::character_ship).
    #[test]
    fn a_split_takes_the_squadrons_and_characters_aboard_the_ships_that_leave() {
        let mut setup = setup();
        let here = setup.here;
        let source = fleet(&mut setup, here, &[10, 20], true);
        setup.world.capital_ship_classes[setup.class].fighter_capacity = 1;
        let fighter = setup.world.fighter_classes.insert(Default::default());
        // Each ship has room for one, so a squadron boards each.
        for index in 0..2 {
            let tag = crate::carriage::tag_ship(&mut setup.world, source, index).unwrap();
            crate::carriage::board_squadron(&mut setup.world, source, fighter, tag);
        }
        let admiral = setup.world.characters.insert(Character {
            current_fleet: Some(source),
            ..Character::default()
        });
        setup.world.fleets[source].characters.push(admiral);
        let mut transport = TroopTransportState::default();
        let chosen = roster(&setup.world, source).unwrap();

        let created = create_fleet(
            &mut setup.world,
            &MovementState::new(),
            &mut transport,
            source,
            &[0],
            chosen,
            true,
        )
        .unwrap();

        let world = &setup.world;
        let squadrons = |fleet: FleetKey| -> Vec<u32> {
            world.fleets[fleet]
                .fighters
                .iter()
                .map(|entry| entry.carrier)
                .collect()
        };
        assert_eq!(
            squadrons(created),
            [world.fleets[created].capital_ships[0].tag]
        );
        assert_eq!(
            squadrons(source),
            [world.fleets[source].capital_ships[0].tag]
        );
        assert_eq!(world.fleets[created].characters, [admiral]);
        assert!(world.fleets[source].characters.is_empty());
        assert_eq!(world.characters[admiral].current_fleet, Some(created));
    }

    // A destination that no longer resolves refuses (1/0x22,
    // FUN_00578d00); port: the undone split gives the cargo it took back.
    #[test]
    fn a_refused_departure_gives_back_the_regiments_its_ships_took() {
        let mut setup = setup();
        let (here, there) = (setup.here, setup.there);
        let source = fleet(&mut setup, here, &[10, 20], true);
        reclass(&mut setup, source, 0, 0, false);
        let mut transport = TroopTransportState::default();
        let mut troops: Vec<_> = (0..2)
            .map(|_| embark(&mut setup, &mut transport, source))
            .collect();
        troops.sort_unstable();
        setup.world.systems.remove(there);
        let fleets_before = setup.world.fleets.len();
        let chosen = roster(&setup.world, source).unwrap();

        let outcome = move_ships_to_system(
            &mut setup.world,
            &mut MovementState::new(),
            &mut transport,
            source,
            &[1],
            chosen,
            there,
            true,
            0,
        );

        assert!(
            matches!(outcome, Err(FleetOrderError::Dispatch(_))),
            "{outcome:?}"
        );
        assert_eq!(setup.world.fleets.len(), fleets_before);
        assert_eq!(hulls(&setup.world, source), [10, 20]);
        assert_eq!(transport.cargo(source), troops.as_slice());
        assert_eq!(transport.fleet_keys(), [source]);

        // A Death Star among the refused ships stays in the fleet.
        reclass(&mut setup, source, 1, 2, true);
        let chosen = roster(&setup.world, source).unwrap();
        let refused = move_ships_to_system(
            &mut setup.world,
            &mut MovementState::new(),
            &mut transport,
            source,
            &[1],
            chosen,
            there,
            true,
            0,
        );
        assert!(refused.is_err());
        assert!(setup.world.fleet_has_death_star(source));
    }

    // FUN_005003d0: a fleet has a Death Star when one of its ships is in
    // family [0x18, 0x1c); there is no flag, so the answer follows the ship.
    #[test]
    fn a_death_star_ship_split_off_takes_the_death_star_with_it() {
        let mut setup = setup();
        let here = setup.here;
        let source = fleet(&mut setup, here, &[10, 20], true);
        reclass(&mut setup, source, 1, 0, true);
        let mut transport = TroopTransportState::default();
        let chosen = roster(&setup.world, source).unwrap();

        let created = create_fleet(
            &mut setup.world,
            &MovementState::new(),
            &mut transport,
            source,
            &[1],
            chosen,
            true,
        )
        .unwrap();

        assert!(!setup.world.fleet_has_death_star(source));
        assert!(setup.world.fleet_has_death_star(created));

        // Its escort split off leaves the Death Star where its ship is.
        let escort = fleet(&mut setup, here, &[30], true);
        let ship = setup.world.fleets[escort].capital_ships[0].clone();
        setup.world.fleets[created].capital_ships.push(ship);
        let chosen = roster(&setup.world, created).unwrap();
        let other = create_fleet(
            &mut setup.world,
            &MovementState::new(),
            &mut transport,
            created,
            &[1],
            chosen,
            true,
        )
        .unwrap();
        assert!(setup.world.fleet_has_death_star(created));
        assert!(!setup.world.fleet_has_death_star(other));
    }

    // port: an order keeps the indices it was chosen with; a ship lost or
    // gained since would make them name other ships.
    #[test]
    fn a_ship_order_whose_fleet_changed_since_it_was_chosen_is_refused() {
        let mut setup = setup();
        let here = setup.here;
        let source = fleet(&mut setup, here, &[10, 20, 30], true);
        let target = fleet(&mut setup, here, &[40], true);
        let chosen = roster(&setup.world, source).unwrap();
        setup.world.fleets[source].capital_ships.remove(0);
        let mut transport = TroopTransportState::default();
        let mut movement = MovementState::new();
        let mover = FleetMover::Ships {
            fleet: source,
            ships: vec![1],
            roster: chosen,
        };

        assert_eq!(
            join_fleet(
                &mut setup.world,
                &mut movement,
                &mut transport,
                &mover,
                target,
                true,
                0
            ),
            Err(FleetOrderError::ShipsChanged)
        );
        assert_eq!(
            create_fleet(
                &mut setup.world,
                &movement,
                &mut transport,
                source,
                &[1],
                chosen,
                true
            ),
            Err(FleetOrderError::ShipsChanged)
        );
        assert_eq!(
            move_ships_to_system(
                &mut setup.world,
                &mut movement,
                &mut transport,
                source,
                &[1],
                chosen,
                here,
                true,
                0
            ),
            Err(FleetOrderError::ShipsChanged)
        );
        assert_eq!(hulls(&setup.world, source), [20, 30]);

        // A ship destroyed in place keeps the roster but cannot move.
        setup.world.fleets[source].capital_ships[1].alive = false;
        let chosen = roster(&setup.world, source).unwrap();
        assert_eq!(
            create_fleet(
                &mut setup.world,
                &movement,
                &mut transport,
                source,
                &[1],
                chosen,
                true
            ),
            Err(FleetOrderError::MissingShip)
        );
    }

    #[test]
    fn validating_a_join_reports_its_refusal_or_destination_and_changes_nothing() {
        // FUN_00553aa0, FUN_00553410, FUN_005073d0 as join_fleet.
        let mut setup = setup();
        let (here, there) = (setup.here, setup.there);
        let source = fleet(&mut setup, here, &[10, 20], true);
        let beside = fleet(&mut setup, here, &[30], true);
        let away = fleet(&mut setup, there, &[40], true);
        let movement = MovementState::new();
        let chosen = roster(&setup.world, source).unwrap();
        let ship = FleetMover::Ships {
            fleet: source,
            ships: vec![1],
            roster: chosen,
        };
        let before = setup.world.clone();

        let check = |world: &GameWorld, mover: &FleetMover, target| {
            validate_join(world, &movement, mover, target, true)
        };
        assert_eq!(
            check(&setup.world, &FleetMover::Fleet(source), beside),
            Ok(here)
        );
        assert_eq!(check(&setup.world, &ship, away), Ok(there));
        assert_eq!(
            check(&setup.world, &FleetMover::Fleet(source), source),
            Err(FleetOrderError::IntoItself)
        );
        // FUN_00537180 takes a destroyed system as a fleet's destination
        // (death-star-strategic.md, "Movement").
        setup.world.systems[there].is_destroyed = true;
        for mover in [FleetMover::Fleet(source), ship.clone()] {
            assert_eq!(check(&setup.world, &mover, away), Ok(there), "{mover:?}");
        }
        // A destination that no longer resolves refuses (1/0x22).
        setup.world.systems.remove(there);
        for mover in [FleetMover::Fleet(source), ship.clone()] {
            assert_eq!(
                check(&setup.world, &mover, away),
                Err(FleetOrderError::Dispatch(
                    FleetDispatchError::MissingDestination
                )),
                "{mover:?}"
            );
        }
        assert_eq!(setup.world.fleets.len(), before.fleets.len());
        assert_eq!(
            setup.world.systems[here].fleets,
            before.systems[here].fleets
        );
        assert_eq!(hulls(&setup.world, source), [10, 20]);
    }

    // FUN_00487cc0: Move asks only when leaving a blockaded system of the
    // fleet's side; Confirmed Move always asks.
    #[test]
    fn a_join_asks_first_when_confirmed_or_leaving_a_blockaded_system() {
        let mut setup = setup();
        let (here, there) = (setup.here, setup.there);
        setup.world.systems[here].control = ControlKind::Controlled(crate::dat::Faction::Alliance);
        let mover = fleet(&mut setup, here, &[10], true);
        let beside = fleet(&mut setup, here, &[20], true);
        let away = fleet(&mut setup, there, &[30], true);
        let open = HashSet::new();
        let blockaded = HashSet::from([here]);
        let asks = |blockaded: &HashSet<SystemKey>, target, confirmed| {
            join_confirms(&setup.world, blockaded, mover, target, confirmed)
        };

        assert!(!asks(&open, away, false));
        assert!(asks(&blockaded, away, false));
        assert!(!asks(&blockaded, beside, false));
        assert!(asks(&open, beside, true));
        assert!(!asks(&blockaded, FleetKey::default(), false));
    }

    // A destination that no longer resolves refuses (1/0x22,
    // FUN_00578d00); the refused split is undone.
    #[test]
    fn ships_whose_departure_is_refused_stay_where_they_were() {
        let mut setup = setup();
        let (here, there) = (setup.here, setup.there);
        let mover = fleet(&mut setup, here, &[10, 20, 30], true);
        setup.world.systems.remove(there);
        let fleets_before = setup.world.fleets.len();
        let mut movement = MovementState::new();

        let chosen = roster(&setup.world, mover).unwrap();
        let outcome = move_ships_to_system(
            &mut setup.world,
            &mut movement,
            &mut TroopTransportState::default(),
            mover,
            &[1],
            chosen,
            there,
            true,
            0,
        );

        assert_eq!(
            outcome,
            Err(FleetOrderError::Dispatch(
                FleetDispatchError::MissingDestination
            ))
        );
        assert_eq!(hulls(&setup.world, mover), [10, 20, 30]);
        assert_eq!(setup.world.fleets.len(), fleets_before);
        assert_eq!(setup.world.systems[here].fleets, [mover]);
        assert!(movement.is_empty());
    }

    // FUN_00580b00 → FUN_00509b40: Create Fleet puts the selected ships in a
    // new fleet of their side in their system.
    #[test]
    fn create_fleet_puts_the_selected_ships_in_a_new_fleet_in_their_system() {
        let mut setup = setup();
        let here = setup.here;
        let source = fleet(&mut setup, here, &[10, 20, 30], true);
        let mut transport = TroopTransportState::default();
        let troop = embark(&mut setup, &mut transport, source);

        let chosen = roster(&setup.world, source).unwrap();
        let created = create_fleet(
            &mut setup.world,
            &MovementState::new(),
            &mut transport,
            source,
            &[0, 2],
            chosen,
            true,
        )
        .unwrap();

        let world = &setup.world;
        assert_ne!(created, source);
        assert_eq!(hulls(world, source), [20]);
        assert_eq!(hulls(world, created), [10, 30]);
        assert_eq!(world.fleets[created].location, here);
        assert!(world.fleets[created].is_alliance);
        let mut expected = vec![source, created];
        expected.sort_unstable();
        assert_eq!(world.systems[here].fleets, expected);
        // The regiment rides ship 0 (FUN_00552b10), which leaves.
        assert_eq!(transport.cargo(created), [troop]);
        assert!(transport.cargo(source).is_empty());
    }

    // FUN_004fe630: when every ship leaves, the old fleet disbands and its
    // contents go with the ships.
    #[test]
    fn create_fleet_of_every_ship_replaces_the_fleet_and_keeps_its_contents() {
        let mut setup = setup();
        let here = setup.here;
        let source = fleet(&mut setup, here, &[10, 20], true);
        let mut transport = TroopTransportState::default();
        let troop = embark(&mut setup, &mut transport, source);

        let chosen = roster(&setup.world, source).unwrap();
        let created = create_fleet(
            &mut setup.world,
            &MovementState::new(),
            &mut transport,
            source,
            &[1, 0],
            chosen,
            true,
        )
        .unwrap();

        assert!(!setup.world.fleets.contains_key(source));
        assert_eq!(hulls(&setup.world, created), [10, 20]);
        assert_eq!(setup.world.systems[here].fleets, [created]);
        assert_eq!(transport.cargo(created), [troop]);
    }

    #[test]
    fn create_fleet_refuses_a_selection_that_names_no_ship_of_the_fleet() {
        // port: the selection's own check.
        let mut setup = setup();
        let here = setup.here;
        let source = fleet(&mut setup, here, &[10, 20], true);
        let mut transport = TroopTransportState::default();

        let chosen = roster(&setup.world, source).unwrap();
        for ships in [&[][..], &[2], &[0, 0]] {
            assert_eq!(
                create_fleet(
                    &mut setup.world,
                    &MovementState::new(),
                    &mut transport,
                    source,
                    ships,
                    chosen,
                    true
                ),
                Err(FleetOrderError::MissingShip),
                "{ships:?}"
            );
        }
        assert_eq!(setup.world.fleets.len(), 1);
    }

    // FUN_00507750: a system holds no capital ships, so a ship Moved to
    // another system goes into the side's bit-1 spare there (FUN_00552300
    // -> FUN_005097d0) at once, a completed one on its own leg
    // (FUN_00556390.c:12-28, FUN_00556430) and an unfinished one with none;
    // the spare keeps its bit while it holds ships (FUN_0050bc60 drops it
    // only from a destroyed fleet), so a later Move there joins it, and an
    // emptied fleet disbands (FUN_004fe630). The route-1 Empire save, its
    // day-123 Death Star played on: Galleon 2 Moved alone from Fleet 1 at
    // Coruscant to Averam was on the order day in a new fleet at Averam
    // with its own en-route bit (+0x50 0x30876), clear days later in that
    // fleet (0x30147); the unfinished Death Star Moved there later joined
    // the same fleet at once (0x142).
    #[test]
    fn ships_moved_to_another_system_join_the_sides_spare_there_at_once_for_either_side() {
        for is_alliance in [true, false] {
            let mut setup = setup();
            let (here, there) = (setup.here, setup.there);
            let source = fleet(&mut setup, here, &[10, 20, 30], is_alliance);
            setup.world.fleets[source].capital_ships[2].completed = false;
            let mut movement = MovementState::new();

            let chosen = roster(&setup.world, source).unwrap();
            let outcome = move_ships_to_system(
                &mut setup.world,
                &mut movement,
                &mut TroopTransportState::default(),
                source,
                &[1],
                chosen,
                there,
                is_alliance,
                100,
            );

            let Ok(FleetOrderOutcome::Joined(spare)) = outcome else {
                panic!("{is_alliance}: the ship joins the spare: {outcome:?}");
            };
            assert_eq!(setup.world.fleets[spare].location, there, "{is_alliance}");
            assert!(
                setup.world.systems[there].fleets.contains(&spare),
                "{is_alliance}"
            );
            assert_eq!(hulls(&setup.world, spare), [20], "{is_alliance}");
            assert_eq!(
                setup.world.fleets[spare].capital_ships[0].transit,
                Some(crate::world::ShipTransit {
                    from: here,
                    to: there,
                    arrival_tick: arrival(&setup, 100),
                }),
                "{is_alliance}"
            );
            // FUN_004fe540: no member has arrived, so it is not usable.
            assert!(!setup.world.fleets[spare].usable(), "{is_alliance}");
            assert_eq!(hulls(&setup.world, source), [10, 30], "{is_alliance}");
            assert!(movement.is_empty(), "{is_alliance}");

            let chosen = roster(&setup.world, source).unwrap();
            let outcome = move_ships_to_system(
                &mut setup.world,
                &mut movement,
                &mut TroopTransportState::default(),
                source,
                &[0, 1],
                chosen,
                there,
                is_alliance,
                105,
            );

            assert_eq!(
                outcome,
                Ok(FleetOrderOutcome::Joined(spare)),
                "{is_alliance}"
            );
            assert_eq!(hulls(&setup.world, spare), [20, 10, 30], "{is_alliance}");
            let ships = &setup.world.fleets[spare].capital_ships;
            assert_eq!(
                ships[1].transit.map(|leg| leg.arrival_tick),
                Some(arrival(&setup, 105)),
                "{is_alliance}"
            );
            assert_eq!(ships[2].transit, None, "{is_alliance}");
            assert!(!setup.world.fleets.contains_key(source), "{is_alliance}");
        }
    }

    // FUN_00551630 checks each moving ship's leg against the room of its
    // destination system: one family-0x18 ship a system, either side
    // (FUN_00507ab0, 1 less the count; `movement::death_star_room`).
    #[test]
    fn a_death_star_moved_to_a_system_holding_one_is_refused_for_either_side() {
        for is_alliance in [true, false] {
            let mut setup = setup();
            let (here, there) = (setup.here, setup.there);
            let source = fleet(&mut setup, here, &[10, 20], is_alliance);
            reclass(&mut setup, source, 1, 0, true);
            let other = fleet(&mut setup, there, &[30], !is_alliance);
            reclass(&mut setup, other, 0, 0, true);
            let mut movement = MovementState::new();
            let chosen = roster(&setup.world, source).unwrap();
            let send = |setup: &mut Setup, movement: &mut MovementState, ships: &[usize]| {
                move_ships_to_system(
                    &mut setup.world,
                    movement,
                    &mut TroopTransportState::default(),
                    source,
                    ships,
                    chosen,
                    there,
                    is_alliance,
                    100,
                )
            };

            assert_eq!(
                send(&mut setup, &mut movement, &[1]),
                Err(FleetOrderError::Dispatch(FleetDispatchError::DeathStarRoom)),
                "{is_alliance}"
            );
            assert_eq!(hulls(&setup.world, source), [10, 20], "{is_alliance}");
            assert!(
                matches!(
                    send(&mut setup, &mut movement, &[0]),
                    Ok(FleetOrderOutcome::Joined(_))
                ),
                "{is_alliance}: a ship of another family has room"
            );
        }
    }

    // FUN_004f9860.c:45-66, the object check of every ship order: the
    // ship's side (`fleet_move_enabled`), and not en route (bit 4: its own
    // leg); an object not completed is refused for every order but 0x203,
    // 0x204, 0x215 and 0x241, so Create Fleet (0x270) greys while Move,
    // which reaches each ship as 0x204, does not. The route-1 Empire save,
    // its day-123 Death Star played on: the unfinished Death Star's menu
    // kept Move and greyed Create Fleet; a completed ship kept both.
    #[test]
    fn an_unfinished_ship_may_move_but_not_create_a_fleet_and_one_on_its_own_leg_neither_for_either_side(
    ) {
        for is_alliance in [true, false] {
            let mut setup = setup();
            let here = setup.here;
            let key = fleet(&mut setup, here, &[10, 20, 30], is_alliance);
            setup.world.fleets[key].capital_ships[1].completed = false;
            setup.world.fleets[key].capital_ships[2].transit = Some(crate::world::ShipTransit {
                from: here,
                to: here,
                arrival_tick: 9,
            });
            let movement = MovementState::new();
            let gates = |index| {
                (
                    ship_move_enabled(&setup.world, &movement, key, index, is_alliance),
                    create_fleet_enabled(&setup.world, &movement, key, index, is_alliance),
                )
            };

            assert_eq!(gates(0), (true, true), "{is_alliance}: completed");
            assert_eq!(gates(1), (true, false), "{is_alliance}: unfinished");
            assert_eq!(gates(2), (false, false), "{is_alliance}: on its own leg");
            assert!(
                !ship_move_enabled(&setup.world, &movement, key, 0, !is_alliance),
                "{is_alliance}: another side's ship"
            );
        }
    }

    // FUN_004f9860.c:45-50 refuses an object not completed for Create
    // Fleet (0x270); a ship Moved to its own system forms a fleet there, a
    // Create Fleet, while a Move elsewhere (0x204) is exempt. Nothing
    // changes on a refusal.
    #[test]
    fn create_fleet_and_a_move_to_its_own_system_refuse_an_unfinished_ship_for_either_side() {
        for is_alliance in [true, false] {
            let mut setup = setup();
            let (here, there) = (setup.here, setup.there);
            let source = fleet(&mut setup, here, &[10, 20], is_alliance);
            setup.world.fleets[source].capital_ships[1].completed = false;
            let mut movement = MovementState::new();
            let chosen = roster(&setup.world, source).unwrap();

            assert_eq!(
                create_fleet(
                    &mut setup.world,
                    &movement,
                    &mut TroopTransportState::default(),
                    source,
                    &[0, 1],
                    chosen,
                    is_alliance,
                ),
                Err(FleetOrderError::Unfinished),
                "{is_alliance}"
            );
            for system in [here, there] {
                let result = move_ships_to_system(
                    &mut setup.world,
                    &mut movement,
                    &mut TroopTransportState::default(),
                    source,
                    &[1],
                    chosen,
                    system,
                    is_alliance,
                    100,
                );
                if system == here {
                    assert_eq!(result, Err(FleetOrderError::Unfinished), "{is_alliance}");
                    assert_eq!(hulls(&setup.world, source), [10, 20], "{is_alliance}");
                } else {
                    assert!(
                        matches!(result, Ok(FleetOrderOutcome::Joined(_))),
                        "{is_alliance}: a Move elsewhere is exempt"
                    );
                }
            }
            assert_eq!(hulls(&setup.world, source), [10], "{is_alliance}");
            let chosen = roster(&setup.world, source).unwrap();
            assert!(
                create_fleet(
                    &mut setup.world,
                    &movement,
                    &mut TroopTransportState::default(),
                    source,
                    &[0],
                    chosen,
                    is_alliance,
                )
                .is_ok(),
                "{is_alliance}: a completed ship"
            );
        }
    }

    // A Death Star sent off as a selected ship, by a ship Move or a join
    // elsewhere, leaves its usable bit behind at once (FUN_00556390 ->
    // FUN_004f7640), and event 0x301 reruns bit 11 (FUN_00514510.c:149-158).
    #[test]
    fn a_death_star_sent_off_as_a_ship_clears_its_sectors_bit_at_once_for_either_side() {
        for is_alliance in [true, false] {
            for join in [false, true] {
                let mut setup = setup();
                let (here, there) = (setup.here, setup.there);
                let source = fleet(&mut setup, here, &[10, 20], is_alliance);
                reclass(&mut setup, source, 1, 0, true);
                let target = fleet(&mut setup, there, &[30], is_alliance);
                crate::death_star::refresh_sectors(&mut setup.world);
                assert_eq!(setup.world.death_star_sectors.len(), 1, "{is_alliance}");
                let mut movement = MovementState::new();
                let chosen = roster(&setup.world, source).unwrap();

                let sent = if join {
                    join_fleet(
                        &mut setup.world,
                        &mut movement,
                        &mut TroopTransportState::default(),
                        &FleetMover::Ships {
                            fleet: source,
                            ships: vec![1],
                            roster: chosen,
                        },
                        target,
                        is_alliance,
                        100,
                    )
                } else {
                    move_ships_to_system(
                        &mut setup.world,
                        &mut movement,
                        &mut TroopTransportState::default(),
                        source,
                        &[1],
                        chosen,
                        there,
                        is_alliance,
                        100,
                    )
                };

                assert!(sent.is_ok(), "{is_alliance} {join}");
                assert!(
                    setup.world.death_star_sectors.is_empty(),
                    "{is_alliance} {join}"
                );
            }
        }
    }

    // FUN_004f9860 refuses an object en route (+0x50 bit 4, status 4) for
    // every order, and each ship of a join is a 0x204 of its own
    // (FUN_004ffc90.c:19-38), as each of a ship Move or Create Fleet is; one
    // sub-order's refusal refuses the order (FUN_0053c1a0). So a ship on its
    // own leg joins no fleet even in its own system, nor does the fleet
    // holding it, and it is neither Moved to its system nor split off.
    #[test]
    fn a_ship_on_its_own_leg_takes_no_order_even_in_its_own_system_for_either_side() {
        for is_alliance in [true, false] {
            let mut setup = setup();
            let (here, there) = (setup.here, setup.there);
            let mover = fleet(&mut setup, here, &[10, 20], is_alliance);
            let target = fleet(&mut setup, here, &[30], is_alliance);
            setup.world.fleets[mover].capital_ships[0].transit = Some(crate::world::ShipTransit {
                from: there,
                to: here,
                arrival_tick: 9,
            });
            let movement = MovementState::new();
            let chosen = roster(&setup.world, mover).unwrap();
            let ships = |indices: &[usize]| FleetMover::Ships {
                fleet: mover,
                ships: indices.to_vec(),
                roster: chosen,
            };
            let join = |mover: FleetMover| {
                validate_join(&setup.world, &movement, &mover, target, is_alliance)
            };

            assert_eq!(
                join(ships(&[0])),
                Err(FleetOrderError::InTransit),
                "{is_alliance}"
            );
            assert_eq!(
                join(FleetMover::Fleet(mover)),
                Err(FleetOrderError::InTransit),
                "{is_alliance}: its fleet"
            );
            assert_eq!(
                join(ships(&[1])),
                Ok(here),
                "{is_alliance}: the ship beside it"
            );
            assert_eq!(
                validate_ships_to_system(
                    &setup.world,
                    &movement,
                    mover,
                    &[0],
                    chosen,
                    here,
                    is_alliance
                ),
                Err(FleetOrderError::InTransit),
                "{is_alliance}: Moved to its own system"
            );
            let before = setup.world.fleets.len();
            assert_eq!(
                create_fleet(
                    &mut setup.world,
                    &movement,
                    &mut TroopTransportState::default(),
                    mover,
                    &[0, 1],
                    chosen,
                    is_alliance,
                ),
                Err(FleetOrderError::InTransit),
                "{is_alliance}: split off"
            );
            assert_eq!(setup.world.fleets.len(), before, "{is_alliance}");
        }
    }

    // FUN_0053d200 lists each ship's days from FUN_00555d30: none for a
    // ship that is not mobile (+0x38, FUN_004f6410: unfinished), else its
    // own leg's at its own speed (FUN_00556430's).
    #[test]
    fn a_ships_move_lists_its_own_legs_days_or_none_while_unfinished_for_either_side() {
        for is_alliance in [true, false] {
            let mut setup = setup();
            let (here, there) = (setup.here, setup.there);
            let key = fleet(&mut setup, here, &[10, 20], is_alliance);
            setup.world.fleets[key].capital_ships[1].completed = false;
            let leg = u32::try_from(arrival(&setup, 0)).unwrap();
            let days = |index, to| ship_move_days(&setup.world, key, index, to);

            assert!(leg > 0);
            assert_eq!(days(0, there), Some(leg), "{is_alliance}: completed");
            assert_eq!(days(1, there), Some(0), "{is_alliance}: unfinished");
            assert_eq!(days(0, here), Some(0), "{is_alliance}: its own system");
            assert_eq!(days(2, there), None, "{is_alliance}: no such ship");
        }
    }

    // FUN_00487740 runs the order's validator (+0x1c) before FUN_00487cc0
    // asks, so a Confirmed Move the order would refuse never asks.
    #[test]
    fn a_ships_move_to_a_system_is_checked_without_changing_anything_for_either_side() {
        for is_alliance in [true, false] {
            let mut setup = setup();
            let (here, there) = (setup.here, setup.there);
            let key = fleet(&mut setup, here, &[10, 20], is_alliance);
            let movement = MovementState::new();
            let chosen = roster(&setup.world, key).unwrap();
            let check = |world: &GameWorld, side| {
                validate_ships_to_system(world, &movement, key, &[0], chosen, there, side)
            };

            assert_eq!(
                check(&setup.world, is_alliance),
                Ok(vec![0]),
                "{is_alliance}"
            );
            assert_eq!(
                check(&setup.world, !is_alliance),
                Err(FleetOrderError::WrongFaction),
                "{is_alliance}: another side's"
            );
            setup.world.fleets[key].capital_ships[0].transit = Some(crate::world::ShipTransit {
                from: here,
                to: there,
                arrival_tick: 9,
            });
            assert_eq!(
                check(&setup.world, is_alliance),
                Err(FleetOrderError::InTransit),
                "{is_alliance}: on its own leg"
            );
            assert_eq!(setup.world.fleets.len(), 1, "{is_alliance}: nothing moved");
            assert!(
                setup.world.systems[there].fleets.is_empty(),
                "{is_alliance}"
            );
        }
    }

    #[test]
    fn a_ships_move_to_its_own_system_makes_it_a_fleet_there() {
        // FUN_00507750 refuses the ship; the side's spare takes it.
        let mut setup = setup();
        let here = setup.here;
        let source = fleet(&mut setup, here, &[10, 20], true);
        let mut movement = MovementState::new();

        let chosen = roster(&setup.world, source).unwrap();
        let outcome = move_ships_to_system(
            &mut setup.world,
            &mut movement,
            &mut TroopTransportState::default(),
            source,
            &[1],
            chosen,
            here,
            true,
            0,
        );

        let Ok(FleetOrderOutcome::Created(created)) = outcome else {
            panic!("the ship forms a fleet: {outcome:?}");
        };
        assert_eq!(hulls(&setup.world, created), [20]);
        assert!(movement.is_empty());
    }

    #[test]
    fn a_fleet_moved_whole_to_a_fleet_elsewhere_reports_a_refused_departure() {
        // A destination that no longer resolves refuses (1/0x22,
        // FUN_00578d00).
        let mut setup = setup();
        let (here, there) = (setup.here, setup.there);
        let mover = fleet(&mut setup, here, &[10], true);
        let target = fleet(&mut setup, there, &[30], true);
        setup.world.systems.remove(there);
        let mut movement = MovementState::new();

        assert_eq!(
            join_fleet(
                &mut setup.world,
                &mut movement,
                &mut TroopTransportState::default(),
                &FleetMover::Fleet(mover),
                target,
                true,
                0
            ),
            Err(FleetOrderError::Dispatch(
                FleetDispatchError::MissingDestination
            ))
        );
        assert!(movement.is_empty());
        assert_eq!(setup.world.systems[here].fleets, [mover]);
    }

    #[test]
    fn each_refusal_reads_as_a_message_line() {
        // port: the message log's wording.
        assert_eq!(
            FleetOrderError::IntoItself.to_string(),
            "a fleet cannot move into itself"
        );
        assert_eq!(
            FleetOrderError::OtherSideDestination.to_string(),
            "the destination belongs to another side"
        );
        assert_eq!(
            FleetOrderError::DestinationInTransit.to_string(),
            "the destination fleet is in hyperspace"
        );
        assert_eq!(
            FleetOrderError::InTransit.to_string(),
            "fleet is in hyperspace"
        );
        assert_eq!(
            FleetOrderError::MissingShip.to_string(),
            "the ship is no longer in the fleet"
        );
        assert_eq!(
            FleetOrderError::Dispatch(FleetDispatchError::NoHyperdrive).to_string(),
            "fleet has no capital ship to carry it through hyperspace"
        );
    }

    // FUN_00555920.c:31-33 and FUN_00556390.c:12-14: an object that is not
    // mobile (a fleet's slot +0x38, FUN_004fda10: no ship completed and not
    // on its own leg) changes container at once, so a fleet holding only an
    // unfinished ship joins a fleet elsewhere at once. The route-1 Empire
    // save, its day-123 Death Star played on: Fleet 6, holding only that
    // unfinished Death Star, moved from Coruscant to Averam without going
    // en route.
    #[test]
    fn a_fleet_with_no_usable_ship_moved_onto_a_fleet_elsewhere_joins_it_at_once_for_either_side() {
        for is_alliance in [true, false] {
            let mut setup = setup();
            let (here, there) = (setup.here, setup.there);
            let mover = fleet(&mut setup, here, &[10], is_alliance);
            setup.world.fleets[mover].capital_ships[0].completed = false;
            let target = fleet(&mut setup, there, &[30], is_alliance);
            let mut movement = MovementState::new();

            let outcome = join_fleet(
                &mut setup.world,
                &mut movement,
                &mut TroopTransportState::default(),
                &FleetMover::Fleet(mover),
                target,
                is_alliance,
                0,
            );

            assert_eq!(
                outcome,
                Ok(FleetOrderOutcome::Joined(target)),
                "{is_alliance}"
            );
            assert!(!setup.world.fleets.contains_key(mover), "{is_alliance}");
            assert!(!movement.is_in_transit(mover), "{is_alliance}");
            assert_eq!(hulls(&setup.world, target), [30, 10], "{is_alliance}");
            assert!(
                !setup.world.fleets[target].capital_ships[1].completed,
                "{is_alliance}"
            );
        }
    }

    // The same save played on: a Move of Fleet 1 (a Galleon and the
    // unfinished Death Star) onto Fleet 7 joined the whole fleet, the
    // unfinished ship included, and Fleet 1 disbanded; a Move of the
    // unfinished Death Star alone onto Fleet 1 joined it, still unfinished,
    // and its emptied Fleet 6 disbanded (FUN_004ffc90 walks the movers in
    // mode 3; FUN_004fe630 disbands an emptied fleet).
    #[test]
    fn a_join_carries_an_unfinished_ship_and_disbands_the_emptied_fleet_for_either_side() {
        for is_alliance in [true, false] {
            let mut setup = setup();
            let here = setup.here;
            let mover = fleet(&mut setup, here, &[10, 20], is_alliance);
            setup.world.fleets[mover].capital_ships[1].completed = false;
            let target = fleet(&mut setup, here, &[30], is_alliance);
            let mut movement = MovementState::new();
            let mut transport = TroopTransportState::default();

            let outcome = join_fleet(
                &mut setup.world,
                &mut movement,
                &mut transport,
                &FleetMover::Fleet(mover),
                target,
                is_alliance,
                0,
            );

            assert_eq!(
                outcome,
                Ok(FleetOrderOutcome::Joined(target)),
                "{is_alliance}"
            );
            assert!(!setup.world.fleets.contains_key(mover), "{is_alliance}");
            assert_eq!(hulls(&setup.world, target), [30, 10, 20], "{is_alliance}");
            assert!(
                !setup.world.fleets[target].capital_ships[2].completed,
                "{is_alliance}"
            );

            let lone = fleet(&mut setup, here, &[40], is_alliance);
            setup.world.fleets[lone].capital_ships[0].completed = false;
            let chosen = roster(&setup.world, lone).unwrap();
            let outcome = join_fleet(
                &mut setup.world,
                &mut movement,
                &mut transport,
                &FleetMover::Ships {
                    fleet: lone,
                    ships: vec![0],
                    roster: chosen,
                },
                target,
                is_alliance,
                0,
            );

            assert_eq!(
                outcome,
                Ok(FleetOrderOutcome::Joined(target)),
                "{is_alliance}"
            );
            assert!(!setup.world.fleets.contains_key(lone), "{is_alliance}");
            assert_eq!(
                hulls(&setup.world, target),
                [30, 10, 20, 40],
                "{is_alliance}"
            );
            assert!(
                !setup.world.fleets[target].capital_ships[3].completed,
                "{is_alliance}"
            );
        }
    }
}
