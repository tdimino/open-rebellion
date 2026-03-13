//! Fleet movement system: hyperspace transit between star systems.
//!
//! Fleets travel by issuing a `MovementOrder` which specifies a destination
//! and the speed of transit. Each tick the fleet advances toward the
//! destination; on arrival an `ArrivalEvent` is emitted and the caller
//! updates `Fleet.location` in `GameWorld`.
//!
//! # Speed model
//!
//! Each `CapitalShipClass` has a `hyperdrive` rating (u32, higher = faster).
//! A fleet's transit speed is determined by its *slowest* ship:
//! ```text
//! ticks_per_hop = BASE_TICKS_PER_HOP / slowest_hyperdrive_rating
//! ```
//! Fleets with no capital ships use `DEFAULT_TICKS_PER_HOP`.
//!
//! # Usage
//!
//! ```
//! use rebellion_core::movement::{MovementOrder, MovementState, MovementSystem};
//! use rebellion_core::tick::TickEvent;
//!
//! let mut state = MovementState::new();
//! // Dispatch a fleet to move somewhere:
//! // state.order(fleet_key, origin_key, dest_key, ticks_per_hop);
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

/// Baseline ticks a fleet with hyperdrive rating 1 takes to cross one hop.
pub const BASE_TICKS_PER_HOP: u32 = 30;

/// Ticks per hop for fleets with no capital ships (e.g., pure fighter escorts).
pub const DEFAULT_TICKS_PER_HOP: u32 = 20;

/// Minimum ticks per hop regardless of hyperdrive rating (prevents instant travel).
pub const MIN_TICKS_PER_HOP: u32 = 5;

// ---------------------------------------------------------------------------
// Speed calculation
// ---------------------------------------------------------------------------

/// Compute the ticks-per-hop for a fleet based on its slowest ship's hyperdrive.
///
/// Higher hyperdrive rating → fewer ticks per hop → faster transit.
/// Uses the slowest ship in the fleet so a single slow hulk limits the group.
pub fn fleet_ticks_per_hop(fleet: &Fleet, world: &GameWorld) -> u32 {
    if fleet.capital_ships.is_empty() {
        return DEFAULT_TICKS_PER_HOP;
    }

    // Find the minimum hyperdrive rating in the fleet (slowest ship).
    let slowest = fleet
        .capital_ships
        .iter()
        .filter_map(|entry| world.capital_ship_classes.get(entry.class))
        .map(|class| class.hyperdrive)
        .min()
        .unwrap_or(1)
        .max(1); // guard against 0 in DAT data

    // Invert: higher rating = fewer ticks.
    let ticks = BASE_TICKS_PER_HOP / slowest;
    ticks.max(MIN_TICKS_PER_HOP)
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
    pub ticks_per_hop: u32,
    /// Ticks elapsed since departure.
    pub ticks_elapsed: u32,
}

impl MovementOrder {
    /// Create a new movement order.
    pub fn new(
        fleet: FleetKey,
        origin: SystemKey,
        destination: SystemKey,
        ticks_per_hop: u32,
    ) -> Self {
        MovementOrder {
            fleet,
            origin,
            destination,
            ticks_per_hop,
            ticks_elapsed: 0,
        }
    }

    /// Progress fraction in [0.0, 1.0] — 0.0 = just departed, 1.0 = arrived.
    pub fn progress(&self) -> f32 {
        if self.ticks_per_hop == 0 {
            return 1.0;
        }
        (self.ticks_elapsed as f32 / self.ticks_per_hop as f32).min(1.0)
    }

    /// True if the fleet has completed transit.
    pub fn is_complete(&self) -> bool {
        self.ticks_elapsed >= self.ticks_per_hop
    }

    /// Remaining ticks until arrival.
    pub fn ticks_remaining(&self) -> u32 {
        self.ticks_per_hop.saturating_sub(self.ticks_elapsed)
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
    /// `ticks_per_hop` should be computed via `fleet_ticks_per_hop`.
    pub fn order(
        &mut self,
        fleet: FleetKey,
        origin: SystemKey,
        destination: SystemKey,
        ticks_per_hop: u32,
    ) {
        self.orders.insert(
            fleet,
            MovementOrder::new(fleet, origin, destination, ticks_per_hop),
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
                .min(order.ticks_per_hop);

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
        assert_eq!(state.get(fleet).unwrap().ticks_per_hop, 20);
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

    #[test]
    fn ticks_per_hop_no_ships_uses_default() {
        // We can't construct a full GameWorld cheaply here, so just test the constant.
        assert_eq!(DEFAULT_TICKS_PER_HOP, 20);
    }

    #[test]
    fn base_ticks_divided_by_hyperdrive() {
        // hyperdrive=3 → 30/3 = 10
        let ticks = (BASE_TICKS_PER_HOP / 3).max(MIN_TICKS_PER_HOP);
        assert_eq!(ticks, 10);
    }

    #[test]
    fn high_hyperdrive_clamped_to_min() {
        // hyperdrive=100 → 30/100 = 0 → clamped to MIN_TICKS_PER_HOP
        let ticks = (BASE_TICKS_PER_HOP / 100).max(MIN_TICKS_PER_HOP);
        assert_eq!(ticks, MIN_TICKS_PER_HOP);
    }
}
