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
//! that leave, and the characters when the fleet's first living ship
//! leaves. port: squadrons and regiments held on the fleet itself (no
//! ship) stay, but for the regiments the ships that stay have no room for,
//! which go with the ships that leave.

use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::ids::{FleetKey, SystemKey};
use crate::movement::{
    begin_faction_fleet_transit, fleet_move_confirms, validate_fleet_dispatch, FleetDispatchError,
    MovementState,
};
use crate::troop_transport::TroopTransportState;
use crate::world::{CapitalShipClass, Fleet, GameWorld};

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
    /// The movers joined this fleet in its system.
    Joined(FleetKey),
    /// The movers left as this fleet; it joins its target on arrival, when
    /// it has one.
    Departed(FleetKey),
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

/// The mover's ships, checked: every ship for a whole fleet.
fn mover_ships(
    world: &GameWorld,
    source: &Fleet,
    mover: &FleetMover,
) -> Result<Vec<usize>, FleetOrderError> {
    match mover {
        FleetMover::Fleet(_) => Ok((0..source.capital_ships.len()).collect()),
        FleetMover::Ships {
            fleet,
            ships,
            roster: chosen,
        } => selected_ships(
            source,
            ships,
            *chosen,
            roster(world, *fleet).ok_or(FleetOrderError::MissingFleet)?,
        ),
    }
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
    let Some(source) = world.fleets.remove(from) else {
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
    target.has_death_star |= source.has_death_star;
    troop_transport.transfer_fleet(from, to);
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
    let ships_carried_it = holds_death_star_ship(world, from);
    let source = &mut world.fleets[from];
    let characters_leave = crate::carriage::character_ship(source)
        .is_some_and(|first| ships.contains(&first));
    let mut moved = Vec::with_capacity(ships.len());
    for &index in ships.iter().rev() {
        moved.push(source.capital_ships.remove(index));
    }
    moved.reverse();
    let tags: Vec<u32> = moved.iter().map(|ship| ship.tag).filter(|&tag| tag != 0).collect();
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
    // A Death Star among the ships flies under its own flag; a flag with
    // no Death Star ship is the separate tactical object and stays.
    if ships_carried_it {
        let kept = holds_death_star_ship(world, from);
        let gained = holds_death_star_ship(world, to);
        world.fleets[from].has_death_star = kept;
        world.fleets[to].has_death_star |= gained;
    }
    hand_over_excess_cargo(world, troop_transport, from, to);
    world.name_flagship_fleets();
}

/// Whether one of `fleet`'s capital ships is a Death Star.
fn holds_death_star_ship(world: &GameWorld, fleet: FleetKey) -> bool {
    world.fleets.get(fleet).is_some_and(|value| {
        value.capital_ships.iter().any(|ship| {
            world
                .capital_ship_classes
                .get(ship.class)
                .is_some_and(CapitalShipClass::is_death_star)
        })
    })
}

/// After ships leave `from` for `to`: the regiments held on `from` itself
/// that its remaining ships have no room for go with them, the last in key
/// order first, as
/// [`TroopTransportState::destroy_untransportable_cargo`] would drop them.
/// Regiments travelling to board `from` keep their room.
fn hand_over_excess_cargo(
    world: &GameWorld,
    troop_transport: &mut TroopTransportState,
    from: FleetKey,
    to: FleetKey,
) {
    let room = TroopTransportState::fleet_capacity(world, from)
        .map_or(0, |room| usize::try_from(room).unwrap_or(usize::MAX));
    let room = room.saturating_sub(troop_transport.incoming_count(from));
    let aboard = troop_transport
        .cargo(from)
        .iter()
        .filter(|&&troop| troop_transport.carrier(troop) != 0)
        .count();
    let excess: Vec<_> = troop_transport
        .cargo(from)
        .iter()
        .copied()
        .filter(|&troop| troop_transport.carrier(troop) == 0)
        .skip(room.saturating_sub(aboard))
        .collect();
    troop_transport.transfer_some(from, to, &excess);
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
        has_death_star: false,
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
/// names no living ship of the fleet, and one whose fleet's ships changed
/// since it was made.
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
    Ok(split_off(world, troop_transport, fleet, &ships))
}

/// A move whose destination is a fleet (`0x201` against a Fleet window's
/// `+0x70`). In the target's system the movers join it at once; elsewhere
/// they leave as a fleet and join it on arrival (port: the original makes
/// each ship a member at once, in hyperspace until it arrives, manual
/// p. 122).
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
    let departing = depart(
        world,
        movement,
        troop_transport,
        fleet,
        &ships,
        destination,
        expected_is_alliance,
    )?;
    movement.set_join(departing, target);
    Ok(FleetOrderOutcome::Departed(departing))
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
/// the target's system: the order's checks, then the departure's
/// (`validate_fleet_dispatch`) when the target lies in another system.
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
    let (_, destination) = checked_join(world, movement, mover, target, expected_is_alliance)?;
    let fleet = mover.fleet();
    if world.fleets[fleet].location != destination {
        // Ships that leave as a fleet of their own pass the departure's
        // checks as their fleet does: the same side, system and orbit, and
        // living ships, each with a speed (`capital_ship_speed`).
        validate_fleet_dispatch(movement, world, fleet, destination, expected_is_alliance)
            .map_err(FleetOrderError::Dispatch)?;
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
/// capital ships (`FUN_00507750`), so the ships go into a fleet of their
/// own there (`FUN_005097d0`, the bit-1 spare); in their own system that is
/// Create Fleet.
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
) -> Result<FleetOrderOutcome, FleetOrderError> {
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
    let source = &world.fleets[fleet];
    if source.location == system {
        return Ok(FleetOrderOutcome::Created(split_off(
            world,
            troop_transport,
            fleet,
            &ships,
        )));
    }
    depart(
        world,
        movement,
        troop_transport,
        fleet,
        &ships,
        system,
        expected_is_alliance,
    )
    .map(FleetOrderOutcome::Departed)
}

/// Put a split's ships and cargo back where they were in `fleet` and
/// remove the split.
fn undo_split(
    world: &mut GameWorld,
    troop_transport: &mut TroopTransportState,
    split: FleetKey,
    fleet: FleetKey,
    ships: &[usize],
) {
    let Some(value) = world.fleets.remove(split) else {
        return;
    };
    troop_transport.transfer_fleet(split, fleet);
    world.fleets[fleet].has_death_star |= value.has_death_star;
    if let Some(system) = world.systems.get_mut(value.location) {
        system.fleets.retain(|&key| key != split);
    }
    for &character in &value.characters {
        if let Some(record) = world.characters.get_mut(character) {
            record.current_fleet = Some(fleet);
        }
    }
    let source = &mut world.fleets[fleet];
    for (&index, ship) in ships.iter().zip(value.capital_ships) {
        source.capital_ships.insert(index, ship);
    }
    source.fighters.extend(value.fighters);
    source.characters.extend(value.characters);
}

/// Send the ships to `destination` as a fleet of their own. A split that
/// cannot depart is undone, so a refusal changes nothing.
fn depart(
    world: &mut GameWorld,
    movement: &mut MovementState,
    troop_transport: &mut TroopTransportState,
    fleet: FleetKey,
    ships: &[usize],
    destination: SystemKey,
    expected_is_alliance: bool,
) -> Result<FleetKey, FleetOrderError> {
    let whole = ships.len() == world.fleets[fleet].capital_ships.len();
    if whole {
        begin_faction_fleet_transit(movement, world, fleet, destination, expected_is_alliance)
            .map_err(FleetOrderError::Dispatch)?;
        return Ok(fleet);
    }
    let departing = split_off(world, troop_transport, fleet, ships);
    if let Err(error) = validate_fleet_dispatch(
        movement,
        world,
        departing,
        destination,
        expected_is_alliance,
    ) {
        undo_split(world, troop_transport, departing, fleet, ships);
        return Err(FleetOrderError::Dispatch(error));
    }
    begin_faction_fleet_transit(
        movement,
        world,
        departing,
        destination,
        expected_is_alliance,
    )
    .map_err(FleetOrderError::Dispatch)?;
    Ok(departing)
}

#[cfg(test)]
mod tests {
    use super::*;
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
            has_death_star: false,
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
            regiment_strength: 100,
        });
        setup.world.systems[location].ground_units.push(troop);
        transport.embark(&mut setup.world, fleet, &[troop]).unwrap();
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
        setup.world.fleets[mover].has_death_star = true;
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
        assert!(world.fleets[target].has_death_star);
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
                    true
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
                true
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
                true
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

    // FUN_00556390 across systems: the movers travel, and (port) join the
    // target when they arrive.
    #[test]
    fn a_fleet_moved_onto_a_fleet_elsewhere_departs_marked_to_join_it() {
        let mut setup = setup();
        let (here, there) = (setup.here, setup.there);
        let mover = fleet(&mut setup, here, &[10], true);
        let target = fleet(&mut setup, there, &[30], true);
        let mut movement = MovementState::new();

        let outcome = join_fleet(
            &mut setup.world,
            &mut movement,
            &mut TroopTransportState::default(),
            &FleetMover::Fleet(mover),
            target,
            true,
        );

        assert_eq!(outcome, Ok(FleetOrderOutcome::Departed(mover)));
        let order = movement.get(mover).unwrap();
        assert_eq!((order.destination, order.join), (there, Some(target)));
        assert_eq!(hulls(&setup.world, target), [30]);
    }

    #[test]
    fn ships_moved_onto_a_fleet_elsewhere_split_off_and_depart_alone() {
        // FUN_004ffc90 routes each ship; port: the ships leave as one fleet.
        let mut setup = setup();
        let (here, there) = (setup.here, setup.there);
        let mover = fleet(&mut setup, here, &[10, 20], true);
        let target = fleet(&mut setup, there, &[30], true);
        let mut movement = MovementState::new();
        let mut transport = TroopTransportState::default();
        let troop = embark(&mut setup, &mut transport, mover);

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
            true,
        );

        let Ok(FleetOrderOutcome::Departed(departing)) = outcome else {
            panic!("the ship departs: {outcome:?}");
        };
        assert_ne!(departing, mover);
        assert_eq!(hulls(&setup.world, mover), [10]);
        assert_eq!(hulls(&setup.world, departing), [20]);
        assert_eq!(movement.get(departing).unwrap().join, Some(target));
        assert!(!movement.is_in_transit(mover));
        assert_eq!(transport.cargo(mover), [troop]);
        assert!(setup.world.systems[here].fleets.contains(&mover));
        assert!(!setup.world.systems[here].fleets.contains(&departing));
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
            is_alliance: setup.world.fleets[fleet].is_alliance,
            hull: 100,
            hyperdrive: 80,
            troop_capacity,
            ..CapitalShipClass::default()
        });
        setup.world.fleets[fleet].capital_ships[index].class = class;
    }

    // port: the fleet's cargo rides the ships; the excess the staying ships
    // cannot hold would be lost (destroy_untransportable_cargo), so it goes
    // with the ships that leave, the last in key order first.
    #[test]
    fn a_split_hands_the_regiments_the_staying_ships_cannot_carry_to_the_ships_that_leave() {
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
                    regiment_strength: 100,
                });
                setup.world.systems[here].ground_units.push(troop);
                troop
            })
            .collect();
        troops.sort_unstable();
        // Loaded by order where the fleet orbits, so held there.
        transport.load(&mut setup.world, source, &troops).unwrap();
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
        assert!(transport.is_held(created));
        assert!(!transport.is_held(source));
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
        crate::carriage::board_squadrons(&mut setup.world, source, fighter, 2);
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
            world.fleets[fleet].fighters.iter().map(|entry| entry.carrier).collect()
        };
        assert_eq!(squadrons(created), [world.fleets[created].capital_ships[0].tag]);
        assert_eq!(squadrons(source), [world.fleets[source].capital_ships[0].tag]);
        assert_eq!(world.fleets[created].characters, [admiral]);
        assert!(world.fleets[source].characters.is_empty());
        assert_eq!(world.characters[admiral].current_fleet, Some(created));
    }

    // FUN_005073d0 refuses a destroyed destination; port: the undone split
    // gives the cargo it took back.
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
        setup.world.systems[there].is_destroyed = true;
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
        );

        assert!(
            matches!(outcome, Err(FleetOrderError::Dispatch(_))),
            "{outcome:?}"
        );
        assert_eq!(setup.world.fleets.len(), fleets_before);
        assert_eq!(hulls(&setup.world, source), [10, 20]);
        assert_eq!(transport.cargo(source), troops.as_slice());
        assert_eq!(transport.fleet_keys(), [source]);

        // A Death Star among the refused ships keeps its flag on the fleet.
        reclass(&mut setup, source, 1, 2, true);
        setup.world.fleets[source].has_death_star = true;
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
        );
        assert!(refused.is_err());
        assert!(setup.world.fleets[source].has_death_star);
    }

    // port: the Death Star is one of the fleet's capital ships when its
    // class is (CapitalShipClass::is_death_star); its flag follows it.
    #[test]
    fn a_death_star_ship_split_off_takes_the_death_star_flag_with_it() {
        let mut setup = setup();
        let here = setup.here;
        let source = fleet(&mut setup, here, &[10, 20], true);
        reclass(&mut setup, source, 1, 0, true);
        setup.world.fleets[source].has_death_star = true;
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

        assert!(!setup.world.fleets[source].has_death_star);
        assert!(setup.world.fleets[created].has_death_star);

        // Its escort split off leaves the flag where the Death Star is.
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
        assert!(setup.world.fleets[created].has_death_star);
        assert!(!setup.world.fleets[other].has_death_star);
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
                true
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
                true
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
        setup.world.systems[there].is_destroyed = true;
        for mover in [FleetMover::Fleet(source), ship.clone()] {
            assert_eq!(
                check(&setup.world, &mover, away),
                Err(FleetOrderError::Dispatch(
                    FleetDispatchError::DestinationDestroyed
                )),
                "{mover:?}"
            );
        }
        setup.world.systems[there].is_destroyed = false;
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

    // FUN_005073d0: a destroyed destination refuses (0x22); the refused
    // split is undone.
    #[test]
    fn ships_whose_departure_is_refused_stay_where_they_were() {
        let mut setup = setup();
        let (here, there) = (setup.here, setup.there);
        let mover = fleet(&mut setup, here, &[10, 20, 30], true);
        setup.world.systems[there].is_destroyed = true;
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
        );

        assert_eq!(
            outcome,
            Err(FleetOrderError::Dispatch(
                FleetDispatchError::DestinationDestroyed
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

    // FUN_00507750: a system holds no capital ships, so a ship's move to
    // another system departs as a fleet of its own (FUN_005097d0).
    #[test]
    fn a_ships_move_to_another_system_departs_as_a_fleet_of_its_own() {
        let mut setup = setup();
        let (here, there) = (setup.here, setup.there);
        let source = fleet(&mut setup, here, &[10, 20], true);
        let mut movement = MovementState::new();

        let chosen = roster(&setup.world, source).unwrap();
        let outcome = move_ships_to_system(
            &mut setup.world,
            &mut movement,
            &mut TroopTransportState::default(),
            source,
            &[0],
            chosen,
            there,
            true,
        );

        let Ok(FleetOrderOutcome::Departed(departing)) = outcome else {
            panic!("the ship departs: {outcome:?}");
        };
        assert_eq!(hulls(&setup.world, departing), [10]);
        assert_eq!(hulls(&setup.world, source), [20]);
        let order = movement.get(departing).unwrap();
        assert_eq!((order.destination, order.join), (there, None));
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
        );

        let Ok(FleetOrderOutcome::Created(created)) = outcome else {
            panic!("the ship forms a fleet: {outcome:?}");
        };
        assert_eq!(hulls(&setup.world, created), [20]);
        assert!(movement.is_empty());
    }

    #[test]
    fn a_fleet_moved_whole_to_a_fleet_elsewhere_reports_a_refused_departure() {
        // FUN_005073d0: a destroyed destination refuses (0x22).
        let mut setup = setup();
        let (here, there) = (setup.here, setup.there);
        let mover = fleet(&mut setup, here, &[10], true);
        let target = fleet(&mut setup, there, &[30], true);
        setup.world.systems[there].is_destroyed = true;
        let mut movement = MovementState::new();

        assert_eq!(
            join_fleet(
                &mut setup.world,
                &mut movement,
                &mut TroopTransportState::default(),
                &FleetMover::Fleet(mover),
                target,
                true
            ),
            Err(FleetOrderError::Dispatch(
                FleetDispatchError::DestinationDestroyed
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
}
