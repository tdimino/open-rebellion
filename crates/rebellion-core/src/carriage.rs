//! Which capital ship carries a fleet's squadrons, regiments and characters.
//!
//! The original's fleet holds only capital ships; squadrons (`FUN_005039d0`),
//! regiments (`FUN_00504c40`) and characters (`FUN_00536da0`) are children of
//! a ship (`ghidra/notes/fleet-window.md`, "Loading a regiment onto a
//! fleet"). The port keeps the rosters on the fleet and records each unit's
//! ship by its tag (`ShipInstance::tag`): `FighterEntry::carrier` and
//! `TroopTransportState`'s carriers. A tag goes with its ship through every
//! split, join and removal, so cargo whose tag is no longer a living ship of
//! the fleet was lost with that ship.

use std::collections::HashMap;

use crate::delivery::DeliveryState;
use crate::ids::{FighterKey, FleetKey, SystemKey};
use crate::manufacturing::{BuildableKind, Destination, ManufacturingState, ProductionArea};
use crate::troop_transport::TroopTransportState;
use crate::world::{CapitalShipClass, FighterEntry, Fleet, GameWorld};

/// What a ship carries in a hangar or a hold: the units whose room
/// `FUN_00500b40` counts per ship.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cargo {
    /// A fighter squadron, against the class's `+0x26c`.
    Squadron,
    /// A troop regiment, against the class's `+0x270`.
    Regiment,
}

impl Cargo {
    /// The cargo a product of `kind` is; `None` for one that rides no ship
    /// (a capital ship is the fleet's own, `FUN_004fe160`).
    #[must_use]
    pub const fn of(kind: BuildableKind) -> Option<Self> {
        match kind {
            BuildableKind::Fighter(_) => Some(Self::Squadron),
            BuildableKind::Troop(_) => Some(Self::Regiment),
            _ => None,
        }
    }
}

/// The squadron and regiment products bound for each ship, by its tag:
/// queued ones (`QueueItem::carrier`) and travelling ones
/// (`Delivery::carrier`). The original creates a product at its order
/// (`FUN_0052a140` -> `FUN_004f7d50`, created and not completed) aboard the
/// ship the order's leg chose (`FUN_005385f0` -> `FUN_00551a20` ->
/// `FUN_00552300`), and the room walk's mode 3 counts every created child
/// (`FUN_00500b40`), so each takes that ship's room until it arrives.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProductRoom {
    squadrons: HashMap<u32, u32>,
    regiments: HashMap<u32, u32>,
}

impl ProductRoom {
    /// The products `manufacturing` queues and `deliveries` carries.
    #[must_use]
    pub fn new(manufacturing: &ManufacturingState, deliveries: &DeliveryState) -> Self {
        Self::collect(manufacturing, deliveries, None)
    }

    /// As [`Self::new`], less what `system`'s `area` queues.
    #[must_use]
    pub fn excluding(
        manufacturing: &ManufacturingState,
        deliveries: &DeliveryState,
        system: SystemKey,
        area: ProductionArea,
    ) -> Self {
        Self::collect(manufacturing, deliveries, Some((system, area)))
    }

    fn collect(
        manufacturing: &ManufacturingState,
        deliveries: &DeliveryState,
        skip: Option<(SystemKey, ProductionArea)>,
    ) -> Self {
        let mut room = Self::default();
        for (at, area, item) in manufacturing.items() {
            if skip != Some((at, area)) {
                room.add(item.kind, item.carrier);
            }
        }
        for delivery in deliveries.en_route() {
            room.add(delivery.kind, delivery.carrier);
        }
        room
    }

    /// Count one more product of `kind` aboard the ship tagged `carrier`.
    pub fn add(&mut self, kind: BuildableKind, carrier: u32) {
        if carrier == 0 {
            return;
        }
        let counts = match Cargo::of(kind) {
            Some(Cargo::Squadron) => &mut self.squadrons,
            Some(Cargo::Regiment) => &mut self.regiments,
            None => return,
        };
        *counts.entry(carrier).or_default() += 1;
    }

    /// Squadron products bound for the ship tagged `tag`.
    #[must_use]
    pub fn squadrons(&self, tag: u32) -> u32 {
        self.squadrons.get(&tag).copied().unwrap_or(0)
    }

    /// Regiment products bound for the ship tagged `tag`.
    #[must_use]
    pub fn regiments(&self, tag: u32) -> u32 {
        self.regiments.get(&tag).copied().unwrap_or(0)
    }
}

/// Squadrons aboard the ship tagged `tag`.
#[must_use]
pub fn squadrons_aboard(fleet: &Fleet, tag: u32) -> u32 {
    if tag == 0 {
        return 0;
    }
    fleet
        .fighters
        .iter()
        .filter(|entry| entry.carrier == tag)
        .map(|entry| entry.count)
        .sum()
}

/// Squadrons on their way to the ship tagged `tag`. The Move reparents a
/// squadron into its ship at the order (`FUN_00556390`), so each is already
/// one of the ship's children.
#[must_use]
pub fn squadrons_bound_for(world: &GameWorld, tag: u32) -> u32 {
    if tag == 0 {
        return 0;
    }
    let count = world
        .squadron_transits
        .iter()
        .filter(|transit| transit.carrier == tag)
        .count();
    u32::try_from(count).unwrap_or(u32::MAX)
}

/// The room of each of `fleet`'s usable ships, by index: `FUN_00552300`
/// walks the destination fleet's ships in mode 1 (`FUN_00502e30(fleet, 1,
/// side)`), so a ship on its own leg from its yard takes nothing. `room` is
/// a ship's capacity less its children (`FUN_00500b40` in mode 3, which
/// counts every created child, those on their way included).
fn rooms(
    world: &GameWorld,
    fleet: FleetKey,
    room: impl Fn(&CapitalShipClass, u32) -> i64,
) -> Vec<(usize, i64)> {
    let Some(value) = world.fleets.get(fleet) else {
        return Vec::new();
    };
    value
        .capital_ships
        .iter()
        .enumerate()
        .filter(|(_, ship)| ship.usable())
        .filter_map(|(index, ship)| {
            let class = world.capital_ship_classes.get(ship.class)?;
            Some((index, room(class, ship.tag)))
        })
        .collect()
}

/// Of the ships with room, the one with the most room left, the earlier in
/// the fleet on a tie: `FUN_00552300` walks the ships, and `FUN_00552b10`
/// keeps a candidate that `FUN_0054fbb0` finds less constrained than the
/// best so far. `None` when no ship has room (`FUN_00552300`, status
/// `0x27`).
fn most_room(rooms: &[(usize, i64)]) -> Option<usize> {
    let mut best: Option<(usize, i64)> = None;
    for &(index, left) in rooms {
        if left > 0 && best.is_none_or(|(_, most)| left > most) {
            best = Some((index, left));
        }
    }
    best.map(|(index, _)| index)
}

/// The ships `count` units board one after another, each taking the room
/// of the last ([`most_room`]); `None` when one of them finds none.
fn plan(mut rooms: Vec<(usize, i64)>, count: usize) -> Option<Vec<usize>> {
    let mut ships = Vec::with_capacity(count);
    for _ in 0..count {
        let index = most_room(&rooms)?;
        if let Some(room) = rooms.iter_mut().find(|(at, _)| *at == index) {
            room.1 -= 1;
        }
        ships.push(index);
    }
    Some(ships)
}

fn squadron_rooms(world: &GameWorld, products: &ProductRoom, fleet: FleetKey) -> Vec<(usize, i64)> {
    let Some(value) = world.fleets.get(fleet) else {
        return Vec::new();
    };
    rooms(world, fleet, |class, tag| {
        i64::from(class.fighter_capacity)
            - i64::from(squadrons_aboard(value, tag))
            - i64::from(squadrons_bound_for(world, tag))
            - i64::from(products.squadrons(tag))
    })
}

fn regiment_rooms(
    world: &GameWorld,
    transport: &TroopTransportState,
    products: &ProductRoom,
    fleet: FleetKey,
) -> Vec<(usize, i64)> {
    rooms(world, fleet, |class, tag| {
        let children = if tag == 0 {
            0
        } else {
            i64::try_from(
                transport.regiments_aboard(fleet, tag) + transport.regiments_bound_for(tag),
            )
            .unwrap_or(i64::MAX)
            .saturating_add(i64::from(products.regiments(tag)))
        };
        i64::from(class.troop_capacity) - children
    })
}

fn cargo_rooms(
    world: &GameWorld,
    transport: &TroopTransportState,
    products: &ProductRoom,
    fleet: FleetKey,
    cargo: Cargo,
) -> Vec<(usize, i64)> {
    match cargo {
        Cargo::Squadron => squadron_rooms(world, products, fleet),
        Cargo::Regiment => regiment_rooms(world, transport, products, fleet),
    }
}

/// The ship a squadron boards when it is ordered onto `fleet`
/// ([`most_room`]; a ship's room is its class's `+0x26c` less the squadrons
/// aboard, on their way to it and built for it).
#[must_use]
pub fn ship_for_squadron(
    world: &GameWorld,
    products: &ProductRoom,
    fleet: FleetKey,
) -> Option<usize> {
    most_room(&squadron_rooms(world, products, fleet))
}

/// The ship a regiment boards when it is ordered onto `fleet`
/// ([`most_room`]; a ship's room is its class's `+0x270` less the regiments
/// aboard, on their way to it and built for it).
#[must_use]
pub fn ship_for_regiment(
    world: &GameWorld,
    transport: &TroopTransportState,
    products: &ProductRoom,
    fleet: FleetKey,
) -> Option<usize> {
    most_room(&regiment_rooms(world, transport, products, fleet))
}

/// The ships `count` units of `cargo` board when ordered onto `fleet`
/// together, one after another: the order's route keeps each container's
/// room and takes every unit's request from it (`FUN_00552dd0` ->
/// `FUN_00550eb0`, which writes the reduced room back), so each unit finds
/// the room the one before left ([`most_room`]). `None` when one finds
/// none.
#[must_use]
pub fn ships_for(
    world: &GameWorld,
    transport: &TroopTransportState,
    products: &ProductRoom,
    fleet: FleetKey,
    cargo: Cargo,
    count: usize,
) -> Option<Vec<usize>> {
    plan(cargo_rooms(world, transport, products, fleet, cargo), count)
}

/// Seat every squadron and regiment product queued for a fleet that has
/// no ship yet: those just ordered, and those a Destination order or a
/// destroy cascade moved. The units of one fleet and cargo take its ships
/// in queue order, each the room the one before left ([`ships_for`]), as
/// the Build's leg builder places them (`FUN_005385f0` ->
/// `FUN_00551a20` -> `FUN_00552300`). The orders refuse a fleet without
/// that room first (`crate::build_selection::order_refusal`), so every one
/// is seated; one that is not stays unseated, and is lost when it arrives
/// (`crate::delivery`).
pub fn seat_products(
    world: &mut GameWorld,
    transport: &TroopTransportState,
    deliveries: &DeliveryState,
    manufacturing: &mut ManufacturingState,
) {
    let mut groups: Vec<(FleetKey, Cargo, usize)> = Vec::new();
    for (_, _, item) in manufacturing.items() {
        let (Some(Destination::Fleet(fleet)), Some(cargo), 0) =
            (item.destination, Cargo::of(item.kind), item.carrier)
        else {
            continue;
        };
        match groups
            .iter_mut()
            .find(|(at, kind, _)| *at == fleet && *kind == cargo)
        {
            Some(group) => group.2 += 1,
            None => groups.push((fleet, cargo, 1)),
        }
    }
    if groups.is_empty() {
        return;
    }
    let products = ProductRoom::new(manufacturing, deliveries);
    let mut seats: Vec<(FleetKey, Cargo, std::vec::IntoIter<u32>)> = Vec::new();
    for (fleet, cargo, count) in groups {
        let Some(ships) = ships_for(world, transport, &products, fleet, cargo, count) else {
            continue;
        };
        let tags: Vec<u32> = ships
            .into_iter()
            .filter_map(|index| tag_ship(world, fleet, index))
            .collect();
        seats.push((fleet, cargo, tags.into_iter()));
    }
    for item in manufacturing.items_mut() {
        let (Some(Destination::Fleet(fleet)), Some(cargo), 0) =
            (item.destination, Cargo::of(item.kind), item.carrier)
        else {
            continue;
        };
        if let Some((_, _, tags)) = seats
            .iter_mut()
            .find(|(at, kind, _)| *at == fleet && *kind == cargo)
        {
            item.carrier = tags.next().unwrap_or(0);
        }
    }
}

/// The fleet whose living ship is tagged `tag`: the fleet a unit aboard
/// that ship is in, wherever the ship has gone.
#[must_use]
pub fn fleet_of_ship(world: &GameWorld, tag: u32) -> Option<FleetKey> {
    if tag == 0 {
        return None;
    }
    world
        .fleets
        .iter()
        .find(|(_, value)| carried_by_living_ship(value, tag))
        .map(|(key, _)| key)
}

/// The living ship tagged `tag`: its fleet and its index there.
#[must_use]
pub fn ship_of_tag(world: &GameWorld, tag: u32) -> Option<(FleetKey, usize)> {
    let fleet = fleet_of_ship(world, tag)?;
    let index = world.fleets[fleet]
        .capital_ships
        .iter()
        .position(|ship| ship.alive && ship.tag == tag)?;
    Some((fleet, index))
}

/// The tag of the fleet's ship at `index`, giving it the next world tag if
/// it has none.
pub fn tag_ship(world: &mut GameWorld, fleet: FleetKey, index: usize) -> Option<u32> {
    let next = world.last_ship_tag + 1;
    let ship = world.fleets.get_mut(fleet)?.capital_ships.get_mut(index)?;
    if ship.tag == 0 {
        ship.tag = next;
        world.last_ship_tag = next;
    }
    Some(ship.tag)
}

/// Give every living untagged ship the next world tag, fleet by fleet. The
/// AI keys its ship records by tag (`FUN_00484630`); port: the original's
/// ships carry an object id from creation, these get one before each AI pass.
pub fn tag_all(world: &mut GameWorld) {
    let fleets: Vec<FleetKey> = world.fleets.keys().collect();
    for fleet in fleets {
        let untagged: Vec<usize> = world.fleets[fleet]
            .capital_ships
            .iter()
            .enumerate()
            .filter(|(_, ship)| ship.alive && ship.tag == 0)
            .map(|(index, _)| index)
            .collect();
        for index in untagged {
            let _ = tag_ship(world, fleet, index);
        }
    }
}

/// Put one squadron of `class` aboard `fleet`'s ship tagged `carrier`.
pub fn board_squadron(world: &mut GameWorld, fleet: FleetKey, class: FighterKey, carrier: u32) {
    let Some(value) = world.fleets.get_mut(fleet) else {
        return;
    };
    if let Some(entry) = value
        .fighters
        .iter_mut()
        .find(|entry| entry.class == class && entry.carrier == carrier)
    {
        entry.count += 1;
    } else {
        value.fighters.push(FighterEntry {
            class,
            count: 1,
            carrier,
        });
    }
}

/// Whether `tag` names a living ship of `fleet`.
#[must_use]
pub fn carried_by_living_ship(fleet: &Fleet, tag: u32) -> bool {
    tag != 0
        && fleet
            .capital_ships
            .iter()
            .any(|ship| ship.alive && ship.tag == tag)
}

/// Whether `tag` names a usable ship of `fleet` (alive, completed and not on
/// its own leg): the ships a mode-1 walk of its squadrons passes
/// (`FUN_005039d0(ship, 1)`).
#[must_use]
pub fn carried_by_usable_ship(fleet: &Fleet, tag: u32) -> bool {
    tag != 0
        && fleet
            .capital_ships
            .iter()
            .any(|ship| ship.usable() && ship.tag == tag)
}

/// Whether something tagged `tag` was lost with a ship: it names a ship the
/// fleet no longer has alive. An untagged object (`tag` 0) is not aboard any
/// ship, so it is the fleet's or the system's own and no ship's loss takes it
/// (the original's squadrons and regiments are children of a ship or of the
/// system, `FUN_005039d0`).
#[must_use]
pub fn lost_with_ship(fleet: &Fleet, tag: u32) -> bool {
    tag != 0 && !carried_by_living_ship(fleet, tag)
}

/// Remove and return the squadron entries whose ship was lost or left the
/// fleet. A destroyed ship's hook `+0x11c` reaches `FUN_004fb520`, which
/// calls slot `+0xb4` on each child; `FUN_004f8710` destroys a completed
/// squadron that is not en route with the ship's own reason
/// (`FUN_004f7ed0`; `ghidra/notes/tactical-results.md` 3, "Regiment").
#[must_use]
pub fn take_lost_squadrons(fleet: &mut Fleet) -> Vec<FighterEntry> {
    let entries = std::mem::take(&mut fleet.fighters);
    let (lost, kept) = entries
        .into_iter()
        .partition(|entry| lost_with_ship(fleet, entry.carrier));
    fleet.fighters = kept;
    lost
}

/// Drop the squadrons whose ship was lost or left the fleet
/// ([`take_lost_squadrons`]). Returns the squadrons lost.
pub fn drop_lost_squadrons(fleet: &mut Fleet) -> u32 {
    take_lost_squadrons(fleet)
        .iter()
        .map(|entry| entry.count)
        .sum()
}

/// The ship a fleet's characters ride aboard: its first usable ship, else
/// its first living one. Characters have unlimited room (`FUN_00500b40`),
/// so every candidate ties and the fleet's first accepting ship wins
/// (`FUN_00552b10`); `FUN_00552300.c:108,147` offers only the usable ships
/// (mode 1), none unfinished or on its own leg (`0x004f7b80`). port: the
/// port keeps no character's ship, so a character whose ship is lost moves
/// to the next one, and with no usable ship stays on the first living one
/// rather than on none.
#[must_use]
pub fn character_ship(fleet: &Fleet) -> Option<usize> {
    fleet
        .capital_ships
        .iter()
        .position(crate::world::ShipInstance::usable)
        .or_else(|| fleet.capital_ships.iter().position(|ship| ship.alive))
}

/// The Fleet window's indicator flags for the fleet's ship at `index`
/// (`FUN_004a66a0`): 1 squadrons, 2 regiments, 4 characters aboard, `0x40`
/// no hyperdrive.
#[must_use]
pub fn ship_flags(
    world: &GameWorld,
    transport: &TroopTransportState,
    fleet: FleetKey,
    index: usize,
) -> u8 {
    let Some(value) = world.fleets.get(fleet) else {
        return 0;
    };
    let Some(ship) = value.capital_ships.get(index).filter(|ship| ship.alive) else {
        return 0;
    };
    let mut flags = 0;
    if squadrons_aboard(value, ship.tag) > 0 {
        flags |= 1;
    }
    if ship.tag != 0 && transport.regiments_aboard(fleet, ship.tag) > 0 {
        flags |= 2;
    }
    if !value.characters.is_empty() && character_ship(value) == Some(index) {
        flags |= 4;
    }
    if world
        .capital_ship_classes
        .get(ship.class)
        .is_some_and(|class| class.hyperdrive == 0)
    {
        flags |= 0x40;
    }
    flags
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::{CapitalShipClass, ShipInstance};

    /// A fleet of ships whose classes hold `capacities` squadrons each.
    fn fleet(world: &mut GameWorld, capacities: &[u32]) -> FleetKey {
        let ships = capacities
            .iter()
            .map(|&fighter_capacity| {
                let class = world.capital_ship_classes.insert(CapitalShipClass {
                    fighter_capacity,
                    ..CapitalShipClass::default()
                });
                ShipInstance::new(class, 10, true)
            })
            .collect();
        world.fleets.insert(Fleet {
            location: Default::default(),
            capital_ships: ships,
            fighters: Vec::new(),
            characters: Vec::new(),
            is_alliance: true,
            bombarding: false,
        })
    }

    fn carriers(world: &GameWorld, fleet: FleetKey) -> Vec<(u32, u32)> {
        world.fleets[fleet]
            .fighters
            .iter()
            .map(|entry| (entry.carrier, entry.count))
            .collect()
    }

    /// `count` squadrons of `class` ordered onto `fleet` one at a time, each
    /// on the ship with the most room (a Move's leg, `FUN_00552300`).
    fn board(world: &mut GameWorld, fleet: FleetKey, class: FighterKey, count: u32) {
        for _ in 0..count {
            let index = ship_for_squadron(world, &ProductRoom::default(), fleet).unwrap();
            let tag = tag_ship(world, fleet, index).unwrap();
            board_squadron(world, fleet, class, tag);
        }
    }

    /// A queue holding `items` at `system`'s shipyard and training area,
    /// each bound for `fleet` aboard the ship it names.
    fn queued(
        system: SystemKey,
        fleet: FleetKey,
        items: &[(BuildableKind, u32)],
    ) -> ManufacturingState {
        let mut manufacturing = ManufacturingState::new();
        for &(kind, carrier) in items {
            let mut item = crate::manufacturing::QueueItem::new(kind, 5)
                .delivered_to(Destination::Fleet(fleet));
            item.carrier = carrier;
            manufacturing
                .queue_mut(system, ProductionArea::of(kind))
                .enqueue(item);
        }
        manufacturing
    }

    // FUN_00552b10 keeps the candidate FUN_0054fbb0 finds less constrained:
    // the most room wins, the earlier ship on a tie.
    #[test]
    fn a_squadron_boards_the_ship_with_the_most_room() {
        let mut world = GameWorld::default();
        let key = fleet(&mut world, &[2, 3]);
        let class = world.fighter_classes.insert(Default::default());
        assert_eq!(
            ship_for_squadron(&world, &ProductRoom::default(), key),
            Some(1)
        );
        board(&mut world, key, class, 1);
        assert_eq!(
            ship_for_squadron(&world, &ProductRoom::default(), key),
            Some(0)
        );
        board(&mut world, key, class, 2);
        let tags: Vec<u32> = world.fleets[key]
            .capital_ships
            .iter()
            .map(|ship| ship.tag)
            .collect();
        assert_eq!(carriers(&world, key), [(tags[1], 2), (tags[0], 1)]);
    }

    // A fleet-bound product is created aboard its ship at the order
    // (FUN_0052a140 -> FUN_004f7d50) and FUN_00500b40's mode 3 counts every
    // created child, so queued and travelling squadrons take that ship's
    // room: here the ship of room 3 holds one queued and one travelling,
    // leaving the ship of room 2 the roomier.
    #[test]
    fn squadron_products_queued_or_travelling_take_their_ships_room() {
        for is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let key = fleet(&mut world, &[2, 3]);
            world.fleets[key].is_alliance = is_alliance;
            let tag = tag_ship(&mut world, key, 1).unwrap();
            let class = world.fighter_classes.insert(Default::default());
            let yard = world.systems.insert(crate::world::System::default());
            let squadron = BuildableKind::Fighter(class);
            let manufacturing = queued(yard, key, &[(squadron, tag)]);
            let mut deliveries = DeliveryState::new();
            deliveries.depart(
                &world,
                &[crate::manufacturing::Departure {
                    origin: yard,
                    destination: Destination::Fleet(key),
                    tick: 0,
                    kind: squadron,
                    carrier: tag,
                }],
            );
            let products = ProductRoom::new(&manufacturing, &deliveries);
            assert_eq!(products.squadrons(tag), 2, "alliance {is_alliance}");
            assert_eq!(
                ship_for_squadron(&world, &products, key),
                Some(0),
                "alliance {is_alliance}"
            );
        }
    }

    // As for squadrons, a regiment product takes the room of the ship its
    // order chose (FUN_00500b40, class +0x270): one queued for the only
    // ship of room 1 leaves no room for a Move.
    #[test]
    fn a_regiment_product_takes_its_ships_room() {
        for is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let key = fleet(&mut world, &[0]);
            world.fleets[key].is_alliance = is_alliance;
            let class = world.fleets[key].capital_ships[0].class;
            world.capital_ship_classes[class].troop_capacity = 1;
            let tag = tag_ship(&mut world, key, 0).unwrap();
            let yard = world.systems.insert(crate::world::System::default());
            let regiment = BuildableKind::Troop(crate::ids::DatId::new(0x1000_0001));
            let manufacturing = queued(yard, key, &[(regiment, tag)]);
            let products = ProductRoom::new(&manufacturing, &DeliveryState::new());
            let transport = TroopTransportState::new();
            assert_eq!(
                ship_for_regiment(&world, &transport, &ProductRoom::default(), key),
                Some(0)
            );
            assert_eq!(
                ship_for_regiment(&world, &transport, &products, key),
                None,
                "alliance {is_alliance}"
            );
        }
    }

    // FUN_005385f0 calls FUN_00551a20 once per unit, and the route's room
    // record loses each unit's request (FUN_00552dd0 -> FUN_00550eb0), so
    // the units of one Build fill the roomiest ships in turn; a capital
    // ship rides no ship.
    #[test]
    fn products_ordered_for_a_fleet_take_its_ships_in_queue_order() {
        for is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let key = fleet(&mut world, &[1, 2]);
            world.fleets[key].is_alliance = is_alliance;
            let class = world.fighter_classes.insert(Default::default());
            let ship = world
                .capital_ship_classes
                .insert(CapitalShipClass::default());
            let yard = world.systems.insert(crate::world::System::default());
            let squadron = BuildableKind::Fighter(class);
            let mut manufacturing = queued(
                yard,
                key,
                &[
                    (squadron, 0),
                    (BuildableKind::CapitalShip(ship), 0),
                    (squadron, 0),
                    (squadron, 0),
                ],
            );
            seat_products(
                &mut world,
                &TroopTransportState::new(),
                &DeliveryState::new(),
                &mut manufacturing,
            );
            let tags: Vec<u32> = world.fleets[key]
                .capital_ships
                .iter()
                .map(|ship| ship.tag)
                .collect();
            let seated: Vec<u32> = manufacturing
                .items()
                .map(|(_, _, item)| item.carrier)
                .collect();
            assert!(tags.iter().all(|&tag| tag != 0), "alliance {is_alliance}");
            assert_eq!(
                seated,
                [tags[1], 0, tags[0], tags[1]],
                "alliance {is_alliance}"
            );
        }
    }

    // FUN_00552b10 keeps the most room, the earlier ship on a tie; regiments
    // ordered together each take the room the one before left.
    #[test]
    fn regiments_ordered_together_fill_the_roomiest_ships_in_turn() {
        for is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let key = fleet(&mut world, &[0, 0, 0]);
            world.fleets[key].is_alliance = is_alliance;
            for (ship, troops) in world.fleets[key].capital_ships.iter().zip([1, 2, 0]) {
                world.capital_ship_classes[ship.class].troop_capacity = troops;
            }
            let transport = TroopTransportState::new();
            let none = ProductRoom::default();
            assert_eq!(
                ships_for(&world, &transport, &none, key, Cargo::Regiment, 3),
                Some(vec![1, 0, 1]),
                "alliance {is_alliance}"
            );
            assert_eq!(
                ships_for(&world, &transport, &none, key, Cargo::Regiment, 4),
                None,
                "alliance {is_alliance}"
            );
        }
    }

    // An untagged squadron is not aboard any ship, so no ship's loss takes
    // it; the bombardment report's lost list and the drop agree.
    #[test]
    fn an_untagged_squadron_is_not_lost_with_a_ship() {
        for is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let key = fleet(&mut world, &[1, 2]);
            world.fleets[key].is_alliance = is_alliance;
            let class = world.fighter_classes.insert(Default::default());
            let [first, second] = [0, 1].map(|index| tag_ship(&mut world, key, index).unwrap());
            for (carrier, count) in [(0, 1), (first, 2), (second, 1)] {
                world.fleets[key].fighters.push(FighterEntry {
                    class,
                    count,
                    carrier,
                });
            }
            world.fleets[key].capital_ships[0].alive = false;

            assert!(lost_with_ship(&world.fleets[key], first), "{is_alliance}");
            assert!(!lost_with_ship(&world.fleets[key], second), "{is_alliance}");
            assert!(!lost_with_ship(&world.fleets[key], 0), "{is_alliance}");
            let lost = take_lost_squadrons(&mut world.fleets[key]);

            assert_eq!(
                lost.iter()
                    .map(|entry| (entry.carrier, entry.count))
                    .collect::<Vec<_>>(),
                [(first, 2)],
                "{is_alliance}"
            );
            assert_eq!(
                carriers(&world, key),
                [(0, 1), (second, 1)],
                "{is_alliance}"
            );
        }
    }

    // FUN_004f8710: squadrons go with a lost ship; those aboard the others stay.
    #[test]
    fn squadrons_aboard_a_lost_ship_are_lost_with_it() {
        let mut world = GameWorld::default();
        let key = fleet(&mut world, &[1, 2]);
        let class = world.fighter_classes.insert(Default::default());
        board(&mut world, key, class, 3);
        world.fleets[key].capital_ships[0].alive = false;
        assert_eq!(drop_lost_squadrons(&mut world.fleets[key]), 1);
        let tag = world.fleets[key].capital_ships[1].tag;
        assert_eq!(carriers(&world, key), [(tag, 2)]);
    }

    // The leg builder places a unit only on its own destination fleet's
    // ships (FUN_00552300 walks that fleet), so products for two fleets,
    // and a squadron and a regiment for one, each take their own room.
    #[test]
    fn products_for_two_fleets_or_two_cargos_take_each_their_own_ships() {
        for is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let first = fleet(&mut world, &[1]);
            let second = fleet(&mut world, &[1]);
            for key in [first, second] {
                world.fleets[key].is_alliance = is_alliance;
                let class = world.fleets[key].capital_ships[0].class;
                world.capital_ship_classes[class].troop_capacity = 1;
            }
            let class = world.fighter_classes.insert(Default::default());
            let squadron = BuildableKind::Fighter(class);
            let regiment = BuildableKind::Troop(crate::ids::DatId::new(0x1000_0001));
            let yard = world.systems.insert(crate::world::System::default());
            let mut manufacturing = queued(yard, first, &[(squadron, 0), (regiment, 0)]);
            let mut other = crate::manufacturing::QueueItem::new(squadron, 5)
                .delivered_to(Destination::Fleet(second));
            other.carrier = 0;
            manufacturing
                .queue_mut(yard, ProductionArea::Shipyard)
                .enqueue(other);
            seat_products(
                &mut world,
                &TroopTransportState::new(),
                &DeliveryState::new(),
                &mut manufacturing,
            );
            let tag = |key: FleetKey| world.fleets[key].capital_ships[0].tag;
            let seated: Vec<u32> = manufacturing
                .items()
                .map(|(_, _, item)| item.carrier)
                .collect();
            assert!(
                tag(first) != 0 && tag(second) != 0,
                "alliance {is_alliance}"
            );
            assert_eq!(
                seated,
                [tag(first), tag(second), tag(first)],
                "alliance {is_alliance}"
            );
        }
    }

    // FUN_00500b40: characters have unlimited room, so the first living
    // ship takes them.
    #[test]
    fn characters_ride_the_first_living_ship() {
        let mut world = GameWorld::default();
        let key = fleet(&mut world, &[0, 0, 0]);
        assert_eq!(character_ship(&world.fleets[key]), Some(0));
        world.fleets[key].capital_ships[0].alive = false;
        assert_eq!(character_ship(&world.fleets[key]), Some(1));
    }

    // FUN_00552300.c:108,147 seats boarders on a fleet's ships in mode 1
    // (FUN_00502e30(fleet, 1)): a ship unfinished (FUN_004f7d50(..,
    // completed 0)) or on its own leg (bit 5, FUN_00556430) is not usable
    // (0x004f7b80), so the first usable ship takes them; with none, the
    // first living one (port: the port keeps no character's ship).
    #[test]
    fn characters_ride_the_first_usable_ship_else_the_first_living_one_for_either_side() {
        for is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let key = fleet(&mut world, &[0, 0, 0]);
            world.fleets[key].is_alliance = is_alliance;
            let ships = &mut world.fleets[key].capital_ships;
            ships[0].transit = Some(crate::world::ShipTransit {
                from: Default::default(),
                to: Default::default(),
                arrival_tick: 9,
            });
            ships[1].completed = false;
            assert_eq!(character_ship(&world.fleets[key]), Some(2), "{is_alliance}");
            world.fleets[key].capital_ships[2].alive = false;
            assert_eq!(character_ship(&world.fleets[key]), Some(0), "{is_alliance}");
        }
    }

    // FUN_00552300 walks a fleet destination's ships with FUN_00502e30(fleet,
    // 1, side), the mode-1 walk of usable ships (FUN_004f6b90): a ship on its
    // own leg from its yard is not one, so no squadron or regiment boards it,
    // however much room it has.
    #[test]
    fn a_ship_on_its_own_leg_takes_no_squadron_or_regiment() {
        for is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let roomy = world.capital_ship_classes.insert(CapitalShipClass {
                fighter_capacity: 4,
                troop_capacity: 4,
                ..CapitalShipClass::default()
            });
            let small = world.capital_ship_classes.insert(CapitalShipClass {
                fighter_capacity: 1,
                troop_capacity: 1,
                ..CapitalShipClass::default()
            });
            let mut travelling = ShipInstance::new(roomy, 10, is_alliance);
            travelling.transit = Some(crate::world::ShipTransit {
                from: SystemKey::default(),
                to: SystemKey::default(),
                arrival_tick: 10,
            });
            let fleet = world.fleets.insert(Fleet {
                location: SystemKey::default(),
                capital_ships: vec![travelling, ShipInstance::new(small, 10, is_alliance)],
                fighters: Vec::new(),
                characters: Vec::new(),
                is_alliance,
                bombarding: false,
            });
            let products = ProductRoom::default();
            let transport = TroopTransportState::default();
            assert_eq!(ship_for_squadron(&world, &products, fleet), Some(1), "{is_alliance}");
            assert_eq!(ship_for_regiment(&world, &transport, &products, fleet), Some(1));
            assert_eq!(
                ships_for(&world, &transport, &products, fleet, Cargo::Squadron, 2),
                None,
                "only the usable ship's room counts"
            );
        }
    }
}
