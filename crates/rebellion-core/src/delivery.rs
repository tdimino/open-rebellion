//! En-route delivery of manufactured objects (F-030).
//!
//! A product whose destination differs from its manufacturing facility starts
//! travelling when it completes (`FUN_0052bee0` sets en route active) and
//! reaches its destination after the per-object transit time
//! (`FUN_00514a60` -> `FUN_00556430` -> `FUN_0055d8c0`). Event `0x387` ends the
//! trip (`FUN_004fb520`). See `ghidra/notes/build-delivery.md`.
//!
//! The original keeps the travelling object in its destination container,
//! marked en route and so not usable (`0x004f7b80`). A capital ship does so
//! in the port too: it joins its destination fleet when it leaves its yard,
//! with its own leg in `ShipInstance::transit` ([`advance_ship_legs`]);
//! the readers still to audit are in br node
//! `or-port-capital-ship-transit-in-fleet-6t1p`.
//! hyp: the port holds every other product here instead and places it on
//! arrival, so every walk of the destination skips it, not only the
//! usable-mode walks (the disaster `FUN_00511930` and the incident pick);
//! recovered by tracing which walks use modes 3 and 4.

use serde::{Deserialize, Serialize};

use crate::ids::{CapitalShipKey, FleetKey, SystemKey};
use crate::manufacturing::{BuildableKind, CompletionEvent, Departure, Destination};
use crate::movement::{capital_ship_speed, default_speed, fighter_speed, transit_ticks_between};
use crate::tick::TickEvent;
use crate::world::GameWorld;

/// One manufactured object travelling to its destination.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Delivery {
    /// The manufacturing system it left.
    pub origin: SystemKey,
    /// Where it goes (`DestinationLocationAtDeparture`, object `+0x3c`):
    /// a destination fleet's system when it left.
    pub destination: SystemKey,
    /// The destination fleet, whose container holds the object while it
    /// travels and keeps it on arrival.
    #[serde(default)]
    pub fleet: Option<FleetKey>,
    /// The game-day it arrives (`DestinationCount`, object `+0x44`).
    pub arrival_tick: u64,
    /// What was built.
    pub kind: BuildableKind,
    /// The ship a squadron or regiment rides, the one its order chose
    /// (`QueueItem::carrier`); 0 for any other product.
    pub carrier: u32,
    /// The destroyed system the leg still names when a destroy cascade
    /// moved the object on its way: `+0x3c` keeps it, and the arrival
    /// starts the next leg from there ([`DeliveryState::relocate`]).
    #[serde(default)]
    pub via: Option<SystemKey>,
}

/// Objects travelling from their facility to their destination.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeliveryState {
    en_route: Vec<Delivery>,
}

/// What one advance produced.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DeliveryAdvance {
    /// Deliveries that arrived, as completions at their destination.
    pub arrivals: Vec<CompletionEvent>,
    /// Deliveries to a fleet that arrived, for the fleet to take aboard.
    pub boarded: Vec<Boarded>,
    /// Deliveries whose destination no longer exists.
    ///
    /// port: the original's systems are never removed, so it has no such
    /// case; a fleet destination that is gone loses its object
    /// (`GameObjDestroyedOnArrivalNotif`, event `0x303`, `FUN_004fc080`, is
    /// the notice for an object itself destroyed on its way).
    pub lost: Vec<Delivery>,
}

/// A product that arrived aboard a fleet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boarded {
    /// The fleet that holds it: its ship's, for a squadron or regiment.
    pub fleet: FleetKey,
    /// The ship it rides ([`Delivery::carrier`]).
    pub carrier: u32,
    /// The completion at the fleet's system.
    pub completion: CompletionEvent,
}

/// The fleet holding `delivery` now. A squadron or regiment is a child of
/// the ship its order chose, so it is wherever that ship is (`FUN_00556620`
/// makes no new choice), and none without a ship; any other product is its
/// destination fleet's.
fn holder(world: &GameWorld, delivery: &Delivery) -> Option<FleetKey> {
    if crate::carriage::Cargo::of(delivery.kind).is_some() {
        return crate::carriage::fleet_of_ship(world, delivery.carrier);
    }
    delivery
        .fleet
        .filter(|&fleet| world.fleets.contains_key(fleet))
}

/// Speed of a product while it travels (object slot `+0x34`): a capital
/// ship or fighter squadron uses its class ratings (`FUN_00500820`,
/// `FUN_00502f80`); regiments and facilities use the GNPRTB 1 default
/// (`FUN_004f63f0`).
fn product_speed(world: &GameWorld, kind: BuildableKind) -> i64 {
    match kind {
        BuildableKind::CapitalShip(class) => world.capital_ship_classes.get(class).map_or_else(
            || default_speed(world),
            |class| capital_ship_speed(world, class),
        ),
        BuildableKind::Fighter(class) => world
            .fighter_classes
            .get(class)
            .map_or_else(|| default_speed(world), |class| fighter_speed(world, class)),
        _ => default_speed(world),
    }
}

/// Days a product of `kind` takes from `origin` to `destination`
/// (`FUN_00555b30`: none unless both are systems and differ). Build Selection
/// shows it as the best time to deployment (`FUN_00555f90`).
#[must_use]
pub fn transit_days(
    world: &GameWorld,
    kind: BuildableKind,
    origin: SystemKey,
    destination: SystemKey,
) -> u32 {
    if origin == destination {
        return 0;
    }
    let position = |key: SystemKey| world.systems.get(key).map_or((0, 0), |s| (s.x, s.y));
    transit_ticks_between(
        world,
        position(origin),
        position(destination),
        product_speed(world, kind),
    )
}

impl DeliveryState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Objects still travelling, in departure order.
    #[must_use]
    pub fn en_route(&self) -> &[Delivery] {
        &self.en_route
    }

    /// Start each departure on its trip; the arrival day is the completion
    /// day plus the transit time from the facility's system to the
    /// destination (`FUN_00556430`: `+0x44 = clock + ticks`).
    pub fn depart(&mut self, world: &GameWorld, departures: &[Departure]) {
        for departure in departures {
            let (destination, fleet) = match departure.destination {
                Destination::System(system) => (system, None),
                // A fleet already gone leaves the object at its facility,
                // lost on arrival. A squadron or regiment goes to the fleet
                // its ship is in.
                Destination::Fleet(fleet) => {
                    let fleet = match departure.carrier {
                        0 => fleet,
                        tag => crate::carriage::fleet_of_ship(world, tag).unwrap_or(fleet),
                    };
                    (
                        world
                            .fleets
                            .get(fleet)
                            .map_or(departure.origin, |f| f.location),
                        Some(fleet),
                    )
                }
            };
            let position = |key: SystemKey| world.systems.get(key).map_or((0, 0), |s| (s.x, s.y));
            // A special force starts no leg: FUN_005150c0.c:76 hands
            // families 0x30..0x3f to FUN_00545180, which never calls
            // FUN_00556430, so it is at its destination at once.
            let ticks = if matches!(departure.kind, BuildableKind::SpecialForce(_)) {
                0
            } else {
                transit_ticks_between(
                    world,
                    position(departure.origin),
                    position(destination),
                    product_speed(world, departure.kind),
                )
            };
            self.en_route.push(Delivery {
                origin: departure.origin,
                destination,
                fleet,
                arrival_tick: departure.tick + u64::from(ticks),
                kind: departure.kind,
                carrier: departure.carrier,
                via: None,
            });
        }
    }

    /// Take every object travelling into `system`'s own container that a
    /// destroy cascade relocates; those bound for a fleet there stay, as the
    /// fleet's children. A capital ship travels as a child of the spare
    /// fleet, which the cascade skips (`death-star-strategic.md`, "Builds"),
    /// and a special force never travels ([`Self::depart`]).
    pub fn take_bound_for(&mut self, system: SystemKey) -> Vec<Delivery> {
        let (taken, kept) = std::mem::take(&mut self.en_route)
            .into_iter()
            .partition(|delivery| {
                delivery.fleet.is_none()
                    && delivery.destination == system
                    && !matches!(
                        delivery.kind,
                        BuildableKind::CapitalShip(_) | BuildableKind::SpecialForce(_)
                    )
            });
        self.en_route = kept;
        taken
    }

    /// A destroy cascade's relocation of an object on its way
    /// (`FUN_00515480` -> `FUN_00556390`): the object becomes a child of
    /// `destination` at once, but `FUN_00556390.c:17-21` and
    /// `FUN_00556430.c:30` leave the pending timer `0x387`, `+0x3c` and
    /// `+0x44` alone, so it keeps its arrival day. On that day `advance`
    /// starts the next leg from the destroyed system (`FUN_00556620`).
    pub fn relocate(
        &mut self,
        world: &GameWorld,
        mut delivery: Delivery,
        destination: Destination,
    ) {
        let via = delivery.destination;
        (delivery.destination, delivery.fleet) = match destination {
            Destination::System(system) => (system, None),
            Destination::Fleet(fleet) => (
                world.fleets.get(fleet).map_or(via, |f| f.location),
                Some(fleet),
            ),
        };
        delivery.via = Some(via);
        self.en_route.push(delivery);
    }

    /// Deliver every object whose arrival day has come. A destroyed
    /// destination takes it too: the leg never tests the destroyed bit
    /// (`FUN_00556430`, `FUN_00555b30`) and the arrival (timer `0x387` ->
    /// `FUN_00515a50`) only runs `FUN_00556620` and clears en route
    /// (`death-star-strategic.md`, "Arrival at a destroyed system"). One
    /// whose destination is gone is lost ([`DeliveryAdvance::lost`]). One a cascade
    /// relocated starts its next leg on that day instead (event `0x306` ->
    /// `FUN_00556620.c:27-31`): from the destroyed system to its place now,
    /// `FUN_0055d8c0` ticks at its speed, none when both are one system.
    pub fn advance(&mut self, world: &GameWorld, tick_events: &[TickEvent]) -> DeliveryAdvance {
        let Some(now) = tick_events.last().map(|event| event.tick) else {
            return DeliveryAdvance::default();
        };
        let mut result = DeliveryAdvance::default();
        let mut remaining = Vec::with_capacity(self.en_route.len());
        let mut due: std::collections::VecDeque<Delivery> =
            std::mem::take(&mut self.en_route).into();
        while let Some(mut delivery) = due.pop_front() {
            if delivery.arrival_tick <= now {
                if let Some(via) = delivery.via.take() {
                    let to = holder(world, &delivery)
                        .and_then(|fleet| world.fleets.get(fleet))
                        .map_or(delivery.destination, |f| f.location);
                    let ticks = transit_days(world, delivery.kind, via, to);
                    if ticks > 0 {
                        delivery.destination = to;
                        delivery.arrival_tick += u64::from(ticks);
                        due.push_back(delivery);
                        continue;
                    }
                }
            }
            if delivery.arrival_tick > now {
                remaining.push(delivery);
            } else if delivery.fleet.is_some() {
                // The fleet's container, or its ship, held the object all
                // along; it is aboard wherever that is now. hyp: one whose
                // ship was lost on the way is lost with it, as a Move's unit
                // is (`TroopTransportState::advance_transit`); the
                // original's fate for it is untraced (`FUN_004f8710` gives
                // an en route child the autoscrap request,
                // `fleet-join-split.md`).
                match holder(world, &delivery) {
                    Some(fleet) => result.boarded.push(Boarded {
                        fleet,
                        carrier: delivery.carrier,
                        completion: CompletionEvent {
                            system: world.fleets[fleet].location,
                            tick: delivery.arrival_tick,
                            kind: delivery.kind,
                        },
                    }),
                    None => result.lost.push(delivery),
                }
            } else if !world.systems.contains_key(delivery.destination) {
                result.lost.push(delivery);
            } else {
                result.arrivals.push(CompletionEvent {
                    system: delivery.destination,
                    tick: delivery.arrival_tick,
                    kind: delivery.kind,
                });
            }
        }
        self.en_route = remaining;
        result
    }
}

/// A capital ship that ended its own leg in its fleet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShipArrival {
    pub fleet: FleetKey,
    pub class: CapitalShipKey,
    /// The system it reached: its fleet's.
    pub system: SystemKey,
    pub tick: u64,
}

/// End every capital ship's own leg whose day has come (galaxy event
/// `0x387` -> `FUN_0057bf80` -> `FUN_004f7fc0` -> event `0x306` ->
/// `FUN_00515a50`). `FUN_00556620` compares the ship's place, its fleet's
/// system, with the leg's end `+0x3c`: when they differ (the fleet moved on)
/// it starts the next leg from the leg's end to that system at the ship's
/// speed (`FUN_00556620.c:27-31`), else `FUN_004f7640(ship, 0)` clears en
/// route active (`FUN_00515a50.c:63`). A moving fleet's place is its
/// destination (`movement::listed_location`).
pub fn advance_ship_legs(
    world: &mut GameWorld,
    movement: &crate::movement::MovementState,
    tick_events: &[TickEvent],
) -> Vec<ShipArrival> {
    let Some(now) = tick_events.last().map(|event| event.tick) else {
        return Vec::new();
    };
    let mut arrived = Vec::new();
    let keys: Vec<FleetKey> = world.fleets.keys().collect();
    for fleet in keys {
        let Some(here) = crate::movement::listed_location(movement, world, fleet) else {
            continue;
        };
        let count = world.fleets[fleet].capital_ships.len();
        for index in 0..count {
            loop {
                let ship = &world.fleets[fleet].capital_ships[index];
                let Some(leg) = ship
                    .transit
                    .filter(|leg| ship.alive && leg.arrival_tick <= now)
                else {
                    break;
                };
                let class = ship.class;
                let ticks = transit_days(world, BuildableKind::CapitalShip(class), leg.to, here);
                let ship = &mut world.fleets[fleet].capital_ships[index];
                if ticks > 0 {
                    ship.transit = Some(crate::world::ShipTransit {
                        from: leg.to,
                        to: here,
                        arrival_tick: leg.arrival_tick + u64::from(ticks),
                    });
                    continue;
                }
                ship.transit = None;
                arrived.push(ShipArrival {
                    fleet,
                    class,
                    system: here,
                    tick: leg.arrival_tick,
                });
                break;
            }
        }
    }
    arrived
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dat::{ExplorationStatus, SectorGroup};
    use crate::ids::DatId;
    use crate::ids::TroopKey;
    use crate::world::{CapitalShipClass, ControlKind, FighterClass, Sector, System};

    fn world_with_systems(positions: &[(u16, u16)]) -> (GameWorld, Vec<SystemKey>) {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(0x9200_0000),
            name: "Test".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let keys = positions
            .iter()
            .map(|&(x, y)| {
                world.systems.insert(System {
                    dat_id: DatId::new(0x9000_0000),
                    name: format!("Sys@{x},{y}"),
                    sector,
                    x,
                    y,
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
                })
            })
            .collect();
        (world, keys)
    }

    fn departure(
        origin: SystemKey,
        destination: SystemKey,
        tick: u64,
        kind: BuildableKind,
    ) -> Departure {
        Departure {
            origin,
            destination: Destination::System(destination),
            tick,
            kind,
            carrier: 0,
        }
    }

    // A fleet-bound squadron or regiment is a child of the ship its Build
    // chose (FUN_0052a140 -> FUN_004f7d50), so it arrives wherever that
    // ship is (FUN_00556620 makes no new choice). hyp: one whose ship was
    // lost on the way is lost with it; its fate is untraced.
    #[test]
    fn a_product_bound_for_a_ship_arrives_in_the_fleet_that_ship_is_in() {
        for is_alliance in [true, false] {
            let (mut world, systems) = world_with_systems(&[(0, 0), (30, 40)]);
            let class = world
                .capital_ship_classes
                .insert(CapitalShipClass::default());
            let ships = vec![
                crate::world::ShipInstance::new(class, 10, is_alliance),
                crate::world::ShipInstance::new(class, 10, is_alliance),
            ];
            let fleet = world.fleets.insert(crate::world::Fleet {
                location: systems[1],
                is_alliance,
                capital_ships: ships,
                ..crate::world::Fleet::default()
            });
            let tags =
                [0, 1].map(|index| crate::carriage::tag_ship(&mut world, fleet, index).unwrap());
            let squadron =
                BuildableKind::Fighter(world.fighter_classes.insert(FighterClass::default()));
            let mut state = DeliveryState::new();
            let leaving = |carrier| Departure {
                origin: systems[0],
                destination: Destination::Fleet(fleet),
                tick: 0,
                kind: squadron,
                carrier,
            };
            state.depart(&world, &[leaving(tags[1]), leaving(tags[0])]);
            // The second ship forms a fleet of its own; the first is lost.
            let ship = world.fleets[fleet].capital_ships.remove(1);
            let split = world.fleets.insert(crate::world::Fleet {
                location: systems[1],
                is_alliance,
                capital_ships: vec![ship],
                ..crate::world::Fleet::default()
            });
            world.fleets[fleet].capital_ships[0].alive = false;

            let result = state.advance(&world, &[TickEvent { tick: 500 }]);

            let boarded: Vec<_> = result
                .boarded
                .iter()
                .map(|arrived| (arrived.fleet, arrived.carrier, arrived.completion.system))
                .collect();
            assert_eq!(
                boarded,
                [(split, tags[1], systems[1])],
                "alliance {is_alliance}"
            );
            let lost: Vec<u32> = result
                .lost
                .iter()
                .map(|delivery| delivery.carrier)
                .collect();
            assert_eq!(lost, [tags[0]], "alliance {is_alliance}");
        }
    }

    /// A regiment of a class of its own, distinct per `keys` entry.
    fn troop(keys: &mut slotmap::SlotMap<TroopKey, ()>) -> BuildableKind {
        keys.insert(());
        BuildableKind::Troop(crate::ids::DatId::new(0x1000_0000 | keys.len() as u32))
    }

    // FUN_005150c0.c:76 hands a deployed special force (families
    // 0x30..0x3f) to FUN_00545180, which starts no leg (it never calls
    // FUN_00556430): it is at its destination the day it completes.
    #[test]
    fn a_special_force_product_is_at_its_destination_the_day_it_completes() {
        let (world, systems) = world_with_systems(&[(0, 0), (300, 400)]);
        let kind = BuildableKind::SpecialForce(crate::ids::DatId::new(0x3c00_0001));
        let mut state = DeliveryState::new();
        state.depart(&world, &[departure(systems[0], systems[1], 5, kind)]);
        let result = state.advance(&world, &[TickEvent { tick: 5 }]);
        assert_eq!(
            result.arrivals,
            vec![CompletionEvent {
                system: systems[1],
                tick: 5,
                kind,
            }]
        );
        assert!(state.en_route().is_empty());
    }

    // FUN_004f63f0: a regiment travels at the GNPRTB 1 default, 100;
    // FUN_0055d8c0: isqrt(30^2 + 40^2) = 50, 50 / 5 * 100 / 100 = 10 days.
    #[test]
    fn a_regiment_arrives_after_the_default_speed_transit() {
        let (world, systems) = world_with_systems(&[(0, 0), (30, 40)]);
        let kind = troop(&mut slotmap::SlotMap::with_key());
        let mut state = DeliveryState::new();
        state.depart(&world, &[departure(systems[0], systems[1], 7, kind)]);
        assert_eq!(state.en_route()[0].arrival_tick, 17);

        let early = state.advance(&world, &[TickEvent { tick: 16 }]);
        assert!(early.arrivals.is_empty() && early.lost.is_empty());
        let arrived = state.advance(&world, &[TickEvent { tick: 17 }]);
        assert_eq!(
            arrived.arrivals,
            vec![CompletionEvent {
                system: systems[1],
                tick: 17,
                kind
            }]
        );
        assert!(state.en_route().is_empty());
    }

    // FUN_00500820: a capital ship travels at its class hyperdrive.
    #[test]
    fn a_capital_ship_travels_at_its_class_hyperdrive() {
        let (mut world, systems) = world_with_systems(&[(0, 0), (30, 40)]);
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            hyperdrive: 60,
            ..CapitalShipClass::default()
        });
        let mut state = DeliveryState::new();
        state.depart(
            &world,
            &[departure(
                systems[0],
                systems[1],
                0,
                BuildableKind::CapitalShip(class),
            )],
        );
        // 50 / 5 * 60 / 100 = 6.
        assert_eq!(state.en_route()[0].arrival_tick, 6);
    }

    // The leg never tests its destination's destroyed bit (FUN_00556430,
    // FUN_00555b30), and the arrival (timer 0x387 -> FUN_00515a50) only
    // runs FUN_00556620 and clears en route: an object bound for a system
    // destroyed meanwhile arrives there (death-star-strategic.md, "Arrival
    // at a destroyed system").
    #[test]
    fn a_delivery_to_a_destroyed_system_arrives_there() {
        let (mut world, systems) = world_with_systems(&[(0, 0), (30, 40)]);
        let kind = troop(&mut slotmap::SlotMap::with_key());
        let mut state = DeliveryState::new();
        state.depart(&world, &[departure(systems[0], systems[1], 0, kind)]);
        world.systems[systems[1]].is_destroyed = true;
        let result = state.advance(&world, &[TickEvent { tick: 10 }]);
        assert!(result.lost.is_empty());
        assert_eq!(result.arrivals.len(), 1);
        assert_eq!(result.arrivals[0].system, systems[1]);
        assert!(state.en_route().is_empty());
    }

    #[test]
    fn an_advance_without_ticks_delivers_nothing() {
        let (world, systems) = world_with_systems(&[(0, 0), (3, 4)]);
        let kind = troop(&mut slotmap::SlotMap::with_key());
        let mut state = DeliveryState::new();
        state.depart(&world, &[departure(systems[0], systems[1], 0, kind)]);
        assert_eq!(state.advance(&world, &[]), DeliveryAdvance::default());
        assert_eq!(state.en_route().len(), 1);
    }

    // FUN_00556620.c:27-31: on its arrival day an object a cascade moved
    // starts a next leg only when its place now differs from the leg's end
    // (FUN_0055d8c0 ticks, none within one system); one already there
    // arrives that day, in its turn among the day's arrivals.
    #[test]
    fn a_moved_delivery_already_at_its_new_place_arrives_on_its_day_in_its_turn() {
        let (world, systems) = world_with_systems(&[(0, 0), (30, 40)]);
        let mut keys = slotmap::SlotMap::with_key();
        let moved = troop(&mut keys);
        let plain = troop(&mut keys);
        let mut state = DeliveryState::new();
        state.relocate(
            &world,
            Delivery {
                origin: systems[0],
                destination: systems[1],
                fleet: None,
                arrival_tick: 10,
                kind: moved,
                carrier: 0,
                via: None,
            },
            Destination::System(systems[1]),
        );
        state.depart(&world, &[departure(systems[0], systems[1], 0, plain)]);
        assert_eq!(state.en_route()[1].arrival_tick, 10);

        let result = state.advance(&world, &[TickEvent { tick: 10 }]);

        let kinds: Vec<BuildableKind> =
            result.arrivals.iter().map(|arrival| arrival.kind).collect();
        assert_eq!(kinds, [moved, plain]);
        assert!(result.arrivals.iter().all(|arrival| arrival.tick == 10));
        assert!(state.en_route().is_empty());
    }

    #[test]
    fn deliveries_arrive_in_departure_order_and_later_ones_keep_travelling() {
        let (world, systems) = world_with_systems(&[(0, 0), (30, 40), (300, 400)]);
        let mut keys = slotmap::SlotMap::with_key();
        let near = troop(&mut keys);
        let far = troop(&mut keys);
        let mut state = DeliveryState::new();
        state.depart(
            &world,
            &[
                departure(systems[0], systems[2], 0, far),
                departure(systems[0], systems[1], 0, near),
            ],
        );
        let result = state.advance(&world, &[TickEvent { tick: 50 }]);
        assert_eq!(result.arrivals.len(), 1);
        assert_eq!(result.arrivals[0].kind, near);
        assert_eq!(state.en_route()[0].kind, far);
        // 500 / 5 * 100 / 100 = 100.
        assert_eq!(state.en_route()[0].arrival_tick, 100);
    }

    // FUN_00502f80: a squadron travels at its FIGHTSD hyperdrive, else its
    // damaged rating, else the default.
    #[test]
    fn a_fighter_squadron_travels_at_its_class_hyperdrive_then_its_damaged_rating() {
        let (mut world, systems) = world_with_systems(&[(0, 0), (30, 40)]);
        let rated = world.fighter_classes.insert(FighterClass {
            hyperdrive: 40,
            hyperdrive_if_damaged: 90,
            ..FighterClass::default()
        });
        let damaged_only = world.fighter_classes.insert(FighterClass {
            hyperdrive_if_damaged: 70,
            ..FighterClass::default()
        });
        let unrated = world.fighter_classes.insert(FighterClass::default());
        let mut state = DeliveryState::new();
        state.depart(
            &world,
            &[
                departure(systems[0], systems[1], 0, BuildableKind::Fighter(rated)),
                departure(
                    systems[0],
                    systems[1],
                    0,
                    BuildableKind::Fighter(damaged_only),
                ),
                departure(systems[0], systems[1], 0, BuildableKind::Fighter(unrated)),
            ],
        );
        // 50 / 5 = 10 steps, scaled by 40, 70, and the default 100.
        let arrivals: Vec<u64> = state.en_route().iter().map(|d| d.arrival_tick).collect();
        assert_eq!(arrivals, vec![4, 7, 10]);
    }
}
