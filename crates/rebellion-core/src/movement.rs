//! Fleet movement system: hyperspace transit between star systems.
//!
//! Fleets travel by issuing a `MovementOrder` which specifies a destination
//! and the total transit duration. Each tick the fleet advances toward the
//! destination; on arrival an `ArrivalEvent` is emitted and the caller
//! updates `Fleet.location` in `GameWorld`.
//!
//! # Speed model
//!
//! Transit time is based on Euclidean distance between systems:
//! ```text
//! transit_ticks = (distance * DISTANCE_SCALE) / slowest_hyperdrive_rating
//! ```
//! This ensures cross-galaxy trips take ~20+ ticks while intra-sector hops
//! take ~10. Fleets with no capital ships use `DEFAULT_FIGHTER_HYPERDRIVE`.
//! Han Solo's `hyperdrive_modifier` subtracts from the total.
//!
//! # Source
//!
//! Ghidra RE: fleet transit in the original game used direct point-to-point
//! travel (no hyperspace lanes or waypoints). This implementation is faithful.
//!
//! # Usage
//!
//! ```
//! use rebellion_core::movement::{MovementOrder, MovementState, MovementSystem};
//! use rebellion_core::tick::TickEvent;
//!
//! let mut state = MovementState::new();
//! // Dispatch a fleet to move somewhere:
//! // state.order(fleet_key, origin_key, dest_key, transit_ticks);
//!
//! let tick_events = vec![TickEvent { tick: 1 }];
//! let arrivals = MovementSystem::advance(&mut state, &tick_events);
//! // Apply each ArrivalEvent: world.fleets[event.fleet].location = event.system;
//! ```

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ids::{FleetKey, SystemKey};
use crate::tick::TickEvent;
use crate::world::{Fleet, GameWorld};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Multiplier applied to Euclidean distance before dividing by hyperdrive rating.
/// Higher = slower transit. At DISTANCE_SCALE=2, a ~440-unit trip with hyperdrive
/// 80 takes ~11 ticks; a ~900-unit cross-galaxy trip takes ~22 ticks.
pub const DISTANCE_SCALE: u32 = 2;

/// Minimum transit ticks regardless of distance or hyperdrive rating.
pub const MIN_TRANSIT_TICKS: u32 = 10;

/// Effective hyperdrive rating for pure-fighter fleets (no capital ships).
pub const DEFAULT_FIGHTER_HYPERDRIVE: u32 = 60;

// ---------------------------------------------------------------------------
// Speed calculation
// ---------------------------------------------------------------------------

/// Compute transit ticks for a fleet traveling between two systems.
///
/// Uses Euclidean distance between system coordinates:
/// ```text
/// transit_ticks = ceil(distance * DISTANCE_SCALE / slowest_hyperdrive)
/// ```
/// The slowest capital ship in the fleet determines the speed.
/// Han Solo's `hyperdrive_modifier` subtracts from the total.
/// Result is clamped to `MIN_TRANSIT_TICKS`.
pub fn fleet_transit_ticks(
    fleet: &Fleet,
    world: &GameWorld,
    origin: SystemKey,
    dest: SystemKey,
) -> u32 {
    // Euclidean distance between system coordinates.
    let (ox, oy) = world
        .systems
        .get(origin)
        .map(|s| (s.x as f64, s.y as f64))
        .unwrap_or((0.0, 0.0));
    let (dx, dy) = world
        .systems
        .get(dest)
        .map(|s| (s.x as f64, s.y as f64))
        .unwrap_or((0.0, 0.0));
    let distance = ((dx - ox).powi(2) + (dy - oy).powi(2)).sqrt();

    // Slowest ship's hyperdrive rating determines fleet speed.
    let slowest_hyperdrive = if fleet.capital_ships.is_empty() {
        DEFAULT_FIGHTER_HYPERDRIVE
    } else {
        fleet
            .capital_ships
            .iter()
            .filter_map(|entry| world.capital_ship_classes.get(entry.class))
            .map(|class| class.hyperdrive)
            .min()
            .unwrap_or(1)
            .max(1) // guard against 0 in DAT data
    };

    let base_ticks =
        ((distance * DISTANCE_SCALE as f64) / slowest_hyperdrive as f64).ceil() as u32;

    // Han Solo speed bonus: best hyperdrive_modifier among fleet characters.
    let han_bonus = fleet
        .characters
        .iter()
        .filter_map(|&ck| world.characters.get(ck))
        .map(|c| c.hyperdrive_modifier.max(0) as u32)
        .max()
        .unwrap_or(0);

    let ticks = base_ticks.saturating_sub(han_bonus);
    ticks.max(MIN_TRANSIT_TICKS)
}

// ---------------------------------------------------------------------------
// MovementOrder
// ---------------------------------------------------------------------------

/// An active hyperspace transit order for one fleet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementOrder {
    /// The fleet making this transit.
    pub fleet: FleetKey,
    /// System the fleet departed from (used for route visualization).
    pub origin: SystemKey,
    /// System the fleet is heading to.
    pub destination: SystemKey,
    /// Ticks needed to complete the transit.
    pub transit_ticks: u32,
    /// Ticks elapsed since departure.
    pub ticks_elapsed: u32,
}

impl MovementOrder {
    /// Create a new movement order.
    pub fn new(
        fleet: FleetKey,
        origin: SystemKey,
        destination: SystemKey,
        transit_ticks: u32,
    ) -> Self {
        MovementOrder {
            fleet,
            origin,
            destination,
            transit_ticks,
            ticks_elapsed: 0,
        }
    }

    /// Progress fraction in [0.0, 1.0] — 0.0 = just departed, 1.0 = arrived.
    pub fn progress(&self) -> f32 {
        if self.transit_ticks == 0 {
            return 1.0;
        }
        (self.ticks_elapsed as f32 / self.transit_ticks as f32).min(1.0)
    }

    /// True if the fleet has completed transit.
    pub fn is_complete(&self) -> bool {
        self.ticks_elapsed >= self.transit_ticks
    }

    /// Remaining ticks until arrival.
    pub fn ticks_remaining(&self) -> u32 {
        self.transit_ticks.saturating_sub(self.ticks_elapsed)
    }
}

// ---------------------------------------------------------------------------
// MovementState
// ---------------------------------------------------------------------------

/// All active fleet movement orders.
///
/// At most one order per fleet — issuing a new order cancels the previous one.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MovementState {
    orders: HashMap<FleetKey, MovementOrder>,
}

impl MovementState {
    pub fn new() -> Self {
        MovementState {
            orders: HashMap::new(),
        }
    }

    /// Issue a movement order. Replaces any existing order for this fleet.
    ///
    /// `transit_ticks` should be computed via `fleet_transit_ticks`.
    pub fn order(
        &mut self,
        fleet: FleetKey,
        origin: SystemKey,
        destination: SystemKey,
        transit_ticks: u32,
    ) {
        self.orders.insert(
            fleet,
            MovementOrder::new(fleet, origin, destination, transit_ticks),
        );
    }

    /// Cancel a movement order (fleet stays at current location).
    pub fn cancel(&mut self, fleet: FleetKey) -> Option<MovementOrder> {
        self.orders.remove(&fleet)
    }

    /// Get the active order for a fleet, if any.
    pub fn get(&self, fleet: FleetKey) -> Option<&MovementOrder> {
        self.orders.get(&fleet)
    }

    /// All active orders (immutable).
    pub fn orders(&self) -> &HashMap<FleetKey, MovementOrder> {
        &self.orders
    }

    /// All active orders (mutable) — for testing and manual state setup.
    pub fn orders_mut(&mut self) -> &mut HashMap<FleetKey, MovementOrder> {
        &mut self.orders
    }

    pub fn len(&self) -> usize {
        self.orders.len()
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }
}

// ---------------------------------------------------------------------------
// ArrivalEvent
// ---------------------------------------------------------------------------

/// Emitted when a fleet completes hyperspace transit.
///
/// The caller must update `GameWorld`:
/// ```text
/// if let Some(fleet) = world.fleets.get_mut(event.fleet) {
///     fleet.location = event.system;
/// }
/// // Also update the source and destination system's fleet lists.
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrivalEvent {
    /// The fleet that arrived.
    pub fleet: FleetKey,
    /// The game-day on which the fleet arrived.
    pub tick: u64,
    /// The system the fleet departed from.
    pub origin: SystemKey,
    /// The system the fleet arrived at.
    pub system: SystemKey,
}

// ---------------------------------------------------------------------------
// MovementSystem
// ---------------------------------------------------------------------------

/// Stateless system that advances fleet transit orders per tick.
pub struct MovementSystem;

impl MovementSystem {
    /// Advance all active movement orders by the ticks in `tick_events`.
    ///
    /// Returns one `ArrivalEvent` per fleet that completes transit this frame.
    /// The caller must remove arrived fleets from their origin system's fleet
    /// list and add them to the destination system's fleet list in `GameWorld`.
    pub fn advance(
        state: &mut MovementState,
        tick_events: &[TickEvent],
    ) -> Vec<ArrivalEvent> {
        if tick_events.is_empty() {
            return Vec::new();
        }

        let tick_count = tick_events.len() as u32;
        let final_tick = tick_events.last().unwrap().tick;
        let mut arrivals = Vec::new();

        // Advance all orders; collect completed ones.
        let mut completed_keys = Vec::new();
        for (fleet_key, order) in state.orders.iter_mut() {
            order.ticks_elapsed = order
                .ticks_elapsed
                .saturating_add(tick_count)
                .min(order.transit_ticks);

            if order.is_complete() {
                arrivals.push(ArrivalEvent {
                    fleet: *fleet_key,
                    tick: final_tick,
                    origin: order.origin,
                    system: order.destination,
                });
                completed_keys.push(*fleet_key);
            }
        }

        // Remove completed orders.
        for key in completed_keys {
            state.orders.remove(&key);
        }

        arrivals
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tick::TickEvent;

    fn mock_fleet_and_systems() -> (FleetKey, SystemKey, SystemKey) {
        let mut fleet_sm: slotmap::SlotMap<FleetKey, ()> = slotmap::SlotMap::with_key();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let fleet = fleet_sm.insert(());
        let origin = sys_sm.insert(());
        let dest = sys_sm.insert(());
        (fleet, origin, dest)
    }

    fn ticks(n: u64) -> Vec<TickEvent> {
        (1..=n).map(|t| TickEvent { tick: t }).collect()
    }

    // --- MovementOrder ---

    #[test]
    fn progress_starts_at_zero() {
        let (fleet, origin, dest) = mock_fleet_and_systems();
        let order = MovementOrder::new(fleet, origin, dest, 10);
        assert_eq!(order.progress(), 0.0);
        assert!(!order.is_complete());
        assert_eq!(order.ticks_remaining(), 10);
    }

    #[test]
    fn progress_at_halfway() {
        let (fleet, origin, dest) = mock_fleet_and_systems();
        let mut order = MovementOrder::new(fleet, origin, dest, 10);
        order.ticks_elapsed = 5;
        assert!((order.progress() - 0.5).abs() < 1e-6);
        assert_eq!(order.ticks_remaining(), 5);
    }

    #[test]
    fn progress_clamps_at_one() {
        let (fleet, origin, dest) = mock_fleet_and_systems();
        let mut order = MovementOrder::new(fleet, origin, dest, 5);
        order.ticks_elapsed = 10; // overshoot
        assert_eq!(order.progress(), 1.0);
        assert!(order.is_complete());
    }

    // --- MovementState ---

    #[test]
    fn order_and_get() {
        let (fleet, origin, dest) = mock_fleet_and_systems();
        let mut state = MovementState::new();
        state.order(fleet, origin, dest, 10);
        assert_eq!(state.len(), 1);
        assert_eq!(state.get(fleet).unwrap().destination, dest);
    }

    #[test]
    fn cancel_removes_order() {
        let (fleet, origin, dest) = mock_fleet_and_systems();
        let mut state = MovementState::new();
        state.order(fleet, origin, dest, 10);
        let removed = state.cancel(fleet);
        assert!(removed.is_some());
        assert!(state.is_empty());
    }

    #[test]
    fn new_order_replaces_existing() {
        let (fleet, origin, dest) = mock_fleet_and_systems();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let dest2 = sys_sm.insert(());

        let mut state = MovementState::new();
        state.order(fleet, origin, dest, 10);
        state.order(fleet, origin, dest2, 20);
        assert_eq!(state.len(), 1);
        assert_eq!(state.get(fleet).unwrap().destination, dest2);
        assert_eq!(state.get(fleet).unwrap().transit_ticks, 20);
    }

    // --- MovementSystem ---

    #[test]
    fn no_ticks_no_arrivals() {
        let (fleet, origin, dest) = mock_fleet_and_systems();
        let mut state = MovementState::new();
        state.order(fleet, origin, dest, 5);
        let arrivals = MovementSystem::advance(&mut state, &[]);
        assert!(arrivals.is_empty());
        assert_eq!(state.len(), 1); // order still active
    }

    #[test]
    fn partial_advance_does_not_arrive() {
        let (fleet, origin, dest) = mock_fleet_and_systems();
        let mut state = MovementState::new();
        state.order(fleet, origin, dest, 10);
        let arrivals = MovementSystem::advance(&mut state, &ticks(5));
        assert!(arrivals.is_empty());
        assert_eq!(state.get(fleet).unwrap().ticks_elapsed, 5);
    }

    #[test]
    fn advance_to_completion_emits_arrival() {
        let (fleet, origin, dest) = mock_fleet_and_systems();
        let mut state = MovementState::new();
        state.order(fleet, origin, dest, 5);
        let arrivals = MovementSystem::advance(&mut state, &ticks(5));
        assert_eq!(arrivals.len(), 1);
        assert_eq!(arrivals[0].fleet, fleet);
        assert_eq!(arrivals[0].system, dest);
        assert_eq!(arrivals[0].origin, origin);
        assert_eq!(arrivals[0].tick, 5);
        // Order removed on arrival
        assert!(state.is_empty());
    }

    #[test]
    fn overshoot_still_arrives_exactly_once() {
        let (fleet, origin, dest) = mock_fleet_and_systems();
        let mut state = MovementState::new();
        state.order(fleet, origin, dest, 3);
        // 10 ticks for a 3-tick journey
        let arrivals = MovementSystem::advance(&mut state, &ticks(10));
        assert_eq!(arrivals.len(), 1);
        assert!(state.is_empty());
    }

    #[test]
    fn multiple_fleets_advance_independently() {
        let mut fleet_sm: slotmap::SlotMap<FleetKey, ()> = slotmap::SlotMap::with_key();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let fleet_a = fleet_sm.insert(());
        let fleet_b = fleet_sm.insert(());
        let origin = sys_sm.insert(());
        let dest_a = sys_sm.insert(());
        let dest_b = sys_sm.insert(());

        let mut state = MovementState::new();
        state.order(fleet_a, origin, dest_a, 5);
        state.order(fleet_b, origin, dest_b, 10);

        // 5 ticks: fleet_a arrives, fleet_b is at 5/10
        let arrivals = MovementSystem::advance(&mut state, &ticks(5));
        assert_eq!(arrivals.len(), 1);
        assert_eq!(arrivals[0].fleet, fleet_a);
        assert_eq!(state.len(), 1);
        assert_eq!(state.get(fleet_b).unwrap().ticks_elapsed, 5);
    }

    // --- Distance-based transit tests ---

    use crate::dat::{ExplorationStatus, SectorGroup};
    use crate::ids::DatId;
    use crate::world::{
        CapitalShipClass, Character, Fleet, ForceTier, GameWorld, Sector, ShipEntry,
        SkillPair, System,
    };

    fn zero_skill() -> SkillPair {
        SkillPair { base: 0, variance: 0 }
    }

    fn test_character(name: &str, hyperdrive_modifier: i16) -> Character {
        Character {
            dat_id: DatId::new(0),
            name: name.into(),
            is_alliance: true,
            is_empire: false,
            is_major: false,
            diplomacy: zero_skill(),
            espionage: zero_skill(),
            ship_design: zero_skill(),
            troop_training: zero_skill(),
            facility_design: zero_skill(),
            combat: zero_skill(),
            leadership: zero_skill(),
            loyalty: zero_skill(),
            jedi_probability: 0,
            jedi_level: zero_skill(),
            can_be_admiral: false,
            can_be_commander: false,
            can_be_general: false,
            force_tier: ForceTier::None,
            force_experience: 0,
            is_discovered_jedi: false,
            is_unable_to_betray: false,
            is_jedi_trainer: false,
            is_known_jedi: false,
            hyperdrive_modifier,
            enhanced_loyalty: 0,
            on_mission: false,
            on_hidden_mission: false,
            on_mandatory_mission: false,
            captured_by: None,
            capture_tick: None,
            is_captive: false,
            current_system: None,
            current_fleet: None,
        }
    }

    fn test_ship_class(hyperdrive: u32) -> CapitalShipClass {
        CapitalShipClass {
            dat_id: DatId::new(0),
            name: "TestShip".into(),
            is_alliance: true,
            is_empire: false,
            refined_material_cost: 0,
            maintenance_cost: 0,
            research_order: 0,
            research_difficulty: 0,
            hull: 100,
            shield_strength: 50,
            sub_light_engine: 5,
            maneuverability: 5,
            hyperdrive,
            fighter_capacity: 0,
            troop_capacity: 0,
            detection: 0,
            turbolaser_fore: 0,
            turbolaser_aft: 0,
            turbolaser_port: 0,
            turbolaser_starboard: 0,
            ion_cannon_fore: 0,
            ion_cannon_aft: 0,
            ion_cannon_port: 0,
            ion_cannon_starboard: 0,
            laser_cannon_fore: 0,
            laser_cannon_aft: 0,
            laser_cannon_port: 0,
            laser_cannon_starboard: 0,
            shield_recharge_rate: 0,
            damage_control: 0,
            bombardment_modifier: 0,
        }
    }

    fn make_system(sector: crate::ids::SectorKey, x: u16, y: u16) -> System {
        System {
            dat_id: DatId::new(0x9000_0000),
            name: format!("Sys@{},{}", x, y),
            sector,
            x,
            y,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            controlling_faction: None,
        }
    }

    fn make_transit_world(x1: u16, y1: u16, x2: u16, y2: u16) -> (GameWorld, SystemKey, SystemKey) {
        let mut world = GameWorld::default();
        let sk = world.sectors.insert(Sector {
            dat_id: DatId::new(0x9200_0000),
            name: "Test".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let s1 = world.systems.insert(make_system(sk, x1, y1));
        let s2 = world.systems.insert(make_system(sk, x2, y2));
        (world, s1, s2)
    }

    #[test]
    fn short_distance_clamps_to_min() {
        // 50 units apart, hyperdrive=80 → ceil(50*2/80)=ceil(1.25)=2 → clamped to MIN=10
        let (mut world, origin, dest) = make_transit_world(0, 0, 30, 40); // distance=50
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let fleet = Fleet {
            location: origin,
            capital_ships: vec![ShipEntry { class: ship_key, count: 1 }],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        };
        assert_eq!(fleet_transit_ticks(&fleet, &world, origin, dest), MIN_TRANSIT_TICKS);
    }

    #[test]
    fn medium_distance_proportional() {
        // ~440 units apart, hyperdrive=80 → ceil(440*2/80)=ceil(11.0)=11
        let (mut world, origin, dest) = make_transit_world(0, 0, 300, 320); // ~438.6
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let fleet = Fleet {
            location: origin,
            capital_ships: vec![ShipEntry { class: ship_key, count: 1 }],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        };
        let t = fleet_transit_ticks(&fleet, &world, origin, dest);
        assert!(t >= 10 && t <= 12, "expected ~11, got {}", t);
    }

    #[test]
    fn cross_galaxy_takes_many_ticks() {
        // ~900 units apart, hyperdrive=80 → ceil(900*2/80)=ceil(22.5)=23
        let (mut world, origin, dest) = make_transit_world(0, 0, 636, 636); // ~899
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let fleet = Fleet {
            location: origin,
            capital_ships: vec![ShipEntry { class: ship_key, count: 1 }],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        };
        let t = fleet_transit_ticks(&fleet, &world, origin, dest);
        assert!(t >= 20, "cross-galaxy should take 20+ ticks, got {}", t);
    }

    #[test]
    fn fighter_only_fleet_uses_default_hyperdrive() {
        // 300 units, no capital ships → DEFAULT_FIGHTER_HYPERDRIVE=60 → ceil(300*2/60)=10
        let (world, origin, dest) = make_transit_world(0, 0, 180, 240); // distance=300
        let fleet = Fleet {
            location: origin,
            capital_ships: vec![],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        };
        assert_eq!(fleet_transit_ticks(&fleet, &world, origin, dest), MIN_TRANSIT_TICKS);
    }

    #[test]
    fn slow_ship_limits_fleet() {
        // ~440 units, slow ship hyperdrive=20 → ceil(440*2/20)=ceil(44)=44
        let (mut world, origin, dest) = make_transit_world(0, 0, 300, 320); // ~438.6
        let fast_key = world.capital_ship_classes.insert(test_ship_class(80));
        let slow_key = world.capital_ship_classes.insert(test_ship_class(20));
        let fleet = Fleet {
            location: origin,
            capital_ships: vec![
                ShipEntry { class: fast_key, count: 1 },
                ShipEntry { class: slow_key, count: 1 },
            ],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        };
        let t = fleet_transit_ticks(&fleet, &world, origin, dest);
        assert!(t >= 40, "slow ship should dominate, got {}", t);
    }

    #[test]
    fn han_solo_bonus_reduces_ticks() {
        // ~440 units, hyperdrive=80 → base=11, han_bonus=5 → 11-5=6 → clamped to 10
        let (mut world, origin, dest) = make_transit_world(0, 0, 300, 320);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let han_key = world.characters.insert(test_character("Han Solo", 5));
        let fleet = Fleet {
            location: origin,
            capital_ships: vec![ShipEntry { class: ship_key, count: 1 }],
            fighters: vec![],
            characters: vec![han_key],
            is_alliance: true,
            has_death_star: false,
        };
        let t = fleet_transit_ticks(&fleet, &world, origin, dest);
        // base ~11, minus 5 = ~6, clamped to MIN=10
        assert_eq!(t, MIN_TRANSIT_TICKS);
    }

    #[test]
    fn zero_hyperdrive_modifier_no_change() {
        let (mut world, origin, dest) = make_transit_world(0, 0, 300, 320);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let char_key = world.characters.insert(test_character("Regular", 0));
        let fleet = Fleet {
            location: origin,
            capital_ships: vec![ShipEntry { class: ship_key, count: 1 }],
            fighters: vec![],
            characters: vec![char_key],
            is_alliance: true,
            has_death_star: false,
        };
        let t = fleet_transit_ticks(&fleet, &world, origin, dest);
        // No bonus, base ~11
        assert!(t >= 10 && t <= 12, "expected ~11 with no bonus, got {}", t);
    }

    #[test]
    fn han_bonus_clamped_to_min_ticks() {
        // Long trip (~900 units), hyperdrive=80 → base ~23, han_bonus=100 → 0 → clamped to MIN
        let (mut world, origin, dest) = make_transit_world(0, 0, 636, 636);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let han_key = world.characters.insert(test_character("Han Solo", 100));
        let fleet = Fleet {
            location: origin,
            capital_ships: vec![ShipEntry { class: ship_key, count: 1 }],
            fighters: vec![],
            characters: vec![han_key],
            is_alliance: true,
            has_death_star: false,
        };
        assert_eq!(fleet_transit_ticks(&fleet, &world, origin, dest), MIN_TRANSIT_TICKS);
    }
}
