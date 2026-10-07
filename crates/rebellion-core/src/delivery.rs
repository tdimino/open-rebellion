//! En-route delivery of manufactured objects (F-030).
//!
//! A product whose destination differs from its manufacturing facility starts
//! travelling when it completes (`FUN_0052bee0` sets en route active) and
//! reaches its destination after the per-object transit time
//! (`FUN_00514a60` -> `FUN_00556430` -> `FUN_0055d8c0`). Event `0x387` ends the
//! trip (`FUN_004fb520`). See `ghidra/notes/build-delivery.md`.
//!
//! The original keeps the travelling object in its destination container,
//! marked en route and so not usable (`0x004f7b80`). hyp: the port holds it
//! here instead and places it on arrival, so every walk of the destination
//! skips it, not only the usable-mode walks (the disaster `FUN_00511930` and
//! the incident pick); recovered by tracing which walks use modes 3 and 4.

use serde::{Deserialize, Serialize};

use crate::ids::SystemKey;
use crate::manufacturing::{BuildableKind, CompletionEvent, Departure};
use crate::movement::{capital_ship_speed, default_speed, fighter_speed, transit_ticks_between};
use crate::tick::TickEvent;
use crate::world::GameWorld;

/// One manufactured object travelling to its destination.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Delivery {
    /// The manufacturing system it left.
    pub origin: SystemKey,
    /// Where it goes (`DestinationLocationAtDeparture`, object `+0x3c`).
    pub destination: SystemKey,
    /// The game-day it arrives (`DestinationCount`, object `+0x44`).
    pub arrival_tick: u64,
    /// What was built.
    pub kind: BuildableKind,
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
    /// Deliveries whose destination no longer exists
    /// (`GameObjDestroyedOnArrivalNotif`, event `0x303`, `FUN_004fc080`).
    pub lost: Vec<Delivery>,
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
            let position = |key: SystemKey| world.systems.get(key).map_or((0, 0), |s| (s.x, s.y));
            let ticks = transit_ticks_between(
                world,
                position(departure.origin),
                position(departure.destination),
                product_speed(world, departure.kind),
            );
            self.en_route.push(Delivery {
                origin: departure.origin,
                destination: departure.destination,
                arrival_tick: departure.tick + u64::from(ticks),
                kind: departure.kind,
            });
        }
    }

    /// Deliver every object whose arrival day has come. An object whose
    /// destination is gone or destroyed is lost on arrival.
    pub fn advance(&mut self, world: &GameWorld, tick_events: &[TickEvent]) -> DeliveryAdvance {
        let Some(now) = tick_events.last().map(|event| event.tick) else {
            return DeliveryAdvance::default();
        };
        let mut result = DeliveryAdvance::default();
        let mut remaining = Vec::with_capacity(self.en_route.len());
        for delivery in std::mem::take(&mut self.en_route) {
            if delivery.arrival_tick > now {
                remaining.push(delivery);
            } else if world
                .systems
                .get(delivery.destination)
                .is_none_or(|s| s.is_destroyed)
            {
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
            destination,
            tick,
            kind,
        }
    }

    /// A regiment of a class of its own, distinct per `keys` entry.
    fn troop(keys: &mut slotmap::SlotMap<TroopKey, ()>) -> BuildableKind {
        keys.insert(());
        BuildableKind::Troop(crate::ids::DatId::new(0x1000_0000 | keys.len() as u32))
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

    // FUN_004fc080: an object en route whose destination is gone is lost.
    #[test]
    fn a_delivery_to_a_destroyed_system_is_lost_on_arrival() {
        let (mut world, systems) = world_with_systems(&[(0, 0), (30, 40)]);
        let kind = troop(&mut slotmap::SlotMap::with_key());
        let mut state = DeliveryState::new();
        state.depart(&world, &[departure(systems[0], systems[1], 0, kind)]);
        world.systems[systems[1]].is_destroyed = true;
        let result = state.advance(&world, &[TickEvent { tick: 10 }]);
        assert!(result.arrivals.is_empty());
        assert_eq!(result.lost.len(), 1);
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
