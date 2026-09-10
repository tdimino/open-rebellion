//! Fleet movement system: hyperspace transit between star systems.
//!
//! Fleets travel by issuing a `MovementOrder` which specifies a destination
//! and the total transit duration. Each tick the fleet advances toward the
//! destination; on arrival an `ArrivalEvent` is emitted and the caller applies
//! it through [`apply_fleet_arrival`]. While a fleet is moving, its active
//! order is authoritative and it is absent from every system orbit index.
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
//! use rebellion_core::movement::{
//!     apply_fleet_arrival, begin_fleet_transit, MovementState, MovementSystem,
//! };
//! use rebellion_core::tick::TickEvent;
//!
//! let mut state = MovementState::new();
//! // Dispatch a fleet to move somewhere:
//! // begin_fleet_transit(&mut state, &mut world, fleet_key, dest_key, transit_ticks);
//!
//! let tick_events = vec![TickEvent { tick: 1 }];
//! let arrivals = MovementSystem::advance(&mut state, &tick_events);
//! // for event in &arrivals { apply_fleet_arrival(&mut world, &mut cargo, event); }
//! ```

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::ids::{FleetKey, SystemKey};
use crate::troop_transport::TroopTransportState;
use crate::tick::TickEvent;
use crate::tuning::MovementConfig;
use crate::world::{Fleet, FighterEntry, GameWorld};

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
///
/// Accepts optional `MovementConfig` for tuning. Uses module constants as defaults.
pub fn fleet_transit_ticks(
    fleet: &Fleet,
    world: &GameWorld,
    origin: SystemKey,
    dest: SystemKey,
) -> u32 {
    fleet_transit_ticks_with_config(
        fleet, world, origin, dest,
        DISTANCE_SCALE, MIN_TRANSIT_TICKS, DEFAULT_FIGHTER_HYPERDRIVE,
    )
}

/// Config-aware variant of `fleet_transit_ticks`.
pub fn fleet_transit_ticks_with_config(
    fleet: &Fleet,
    world: &GameWorld,
    origin: SystemKey,
    dest: SystemKey,
    distance_scale: u32,
    min_transit_ticks: u32,
    default_fighter_hyperdrive: u32,
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
        default_fighter_hyperdrive
    } else {
        fleet
            .capital_ships
            .iter()
            .filter(|ship| ship.alive)
            .filter_map(|ship| world.capital_ship_classes.get(ship.class))
            .map(|class| class.hyperdrive)
            .min()
            .unwrap_or(1)
            .max(1) // guard against 0 in DAT data
    };

    let base_ticks =
        ((distance * distance_scale as f64) / slowest_hyperdrive as f64).ceil() as u32;

    // Han Solo speed bonus: best hyperdrive_modifier among fleet characters.
    let han_bonus = fleet
        .characters
        .iter()
        .filter_map(|&ck| world.characters.get(ck))
        .map(|c| c.hyperdrive_modifier.max(0) as u32)
        .max()
        .unwrap_or(0);

    let ticks = base_ticks.saturating_sub(han_bonus);
    ticks.max(min_transit_ticks)
}

// ---------------------------------------------------------------------------
// MovementOrder
// ---------------------------------------------------------------------------

/// An active hyperspace transit order for one fleet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
/// At most one order per fleet. An active order must arrive or be cancelled
/// explicitly before another can be issued, so travel progress cannot be reset
/// accidentally by repeated player or AI dispatch.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MovementState {
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    orders: HashMap<FleetKey, MovementOrder>,
}

impl MovementState {
    pub fn new() -> Self {
        MovementState {
            orders: HashMap::new(),
        }
    }

    /// Issue a movement order if the fleet is not already in transit.
    ///
    /// `transit_ticks` should be computed via `fleet_transit_ticks`.
    /// Returns `true` when the order was accepted. Existing orders are left
    /// unchanged and return `false`.
    pub fn order(
        &mut self,
        fleet: FleetKey,
        origin: SystemKey,
        destination: SystemKey,
        transit_ticks: u32,
    ) -> bool {
        if self.orders.contains_key(&fleet) {
            return false;
        }
        self.orders.insert(
            fleet,
            MovementOrder::new(fleet, origin, destination, transit_ticks),
        );
        true
    }

    /// Cancel a movement order (fleet stays at current location).
    pub fn cancel(&mut self, fleet: FleetKey) -> Option<MovementOrder> {
        self.orders.remove(&fleet)
    }

    /// Cancel all movement orders targeting the given system.
    pub fn cancel_orders_to(&mut self, system: crate::ids::SystemKey) {
        self.orders.retain(|_, order| order.destination != system);
    }

    /// Get the active order for a fleet, if any.
    pub fn get(&self, fleet: FleetKey) -> Option<&MovementOrder> {
        self.orders.get(&fleet)
    }

    /// Whether a fleet currently has an active hyperspace order.
    pub fn is_in_transit(&self, fleet: FleetKey) -> bool {
        self.orders.contains_key(&fleet)
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
/// Apply this event through [`apply_fleet_arrival`] so fleet records and orbit
/// indexes remain canonical.
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

/// Result of applying one arrival to the canonical world fleet indexes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppliedArrival {
    /// Stable fleet identity that remains at the destination.
    pub fleet: FleetKey,
    /// Faction retained before any redundant fleet record is removed.
    pub is_alliance: bool,
    /// Number of compatible fleet records absorbed into `fleet`.
    pub merged_fleets: usize,
}

/// Accepted faction-controlled fleet departure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppliedDeparture {
    pub fleet: FleetKey,
    pub origin: SystemKey,
    pub destination: SystemKey,
    pub transit_ticks: u32,
    pub is_alliance: bool,
}

/// Reason a player-facing fleet dispatch cannot begin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FleetDispatchError {
    MissingFleet,
    MissingOrigin,
    MissingDestination,
    DestinationDestroyed,
    WrongFaction,
    AlreadyInTransit,
    AlreadyAtDestination,
    EmptyFleet,
}

impl fmt::Display for FleetDispatchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::MissingFleet => "fleet no longer exists",
            Self::MissingOrigin => "fleet origin is unavailable",
            Self::MissingDestination => "destination is unavailable",
            Self::DestinationDestroyed => "destination has been destroyed",
            Self::WrongFaction => "fleet is not controlled by the player",
            Self::AlreadyInTransit => "fleet is already in transit",
            Self::AlreadyAtDestination => "fleet is already at the destination",
            Self::EmptyFleet => "fleet has no ships or fighter squadrons",
        };
        formatter.write_str(message)
    }
}

/// Validate a player-facing fleet dispatch without mutating the campaign.
pub fn validate_fleet_dispatch(
    state: &MovementState,
    world: &GameWorld,
    fleet: FleetKey,
    destination: SystemKey,
    expected_is_alliance: bool,
) -> Result<(), FleetDispatchError> {
    let value = world
        .fleets
        .get(fleet)
        .ok_or(FleetDispatchError::MissingFleet)?;
    if value.is_alliance != expected_is_alliance {
        return Err(FleetDispatchError::WrongFaction);
    }
    if state.is_in_transit(fleet) {
        return Err(FleetDispatchError::AlreadyInTransit);
    }
    if !world.systems.contains_key(value.location) {
        return Err(FleetDispatchError::MissingOrigin);
    }
    let destination_system = world
        .systems
        .get(destination)
        .ok_or(FleetDispatchError::MissingDestination)?;
    if destination_system.is_destroyed {
        return Err(FleetDispatchError::DestinationDestroyed);
    }
    if value.location == destination {
        return Err(FleetDispatchError::AlreadyAtDestination);
    }
    if value.is_empty() {
        return Err(FleetDispatchError::EmptyFleet);
    }
    Ok(())
}

/// Validate, time, and begin a faction-controlled fleet departure.
pub fn begin_faction_fleet_transit(
    state: &mut MovementState,
    world: &mut GameWorld,
    fleet: FleetKey,
    destination: SystemKey,
    expected_is_alliance: bool,
    config: &MovementConfig,
) -> Result<AppliedDeparture, FleetDispatchError> {
    validate_fleet_dispatch(state, world, fleet, destination, expected_is_alliance)?;
    let value = world
        .fleets
        .get(fleet)
        .ok_or(FleetDispatchError::MissingFleet)?;
    let origin = value.location;
    let is_alliance = value.is_alliance;
    let transit_ticks = fleet_transit_ticks_with_config(
        value,
        world,
        origin,
        destination,
        config.distance_scale,
        config.min_transit_ticks,
        config.default_fighter_hyperdrive,
    );
    if !begin_fleet_transit(state, world, fleet, destination, transit_ticks) {
        return Err(FleetDispatchError::AlreadyInTransit);
    }
    Ok(AppliedDeparture {
        fleet,
        origin,
        destination,
        transit_ticks,
        is_alliance,
    })
}

/// Begin transit and remove the fleet from its origin's orbit index.
///
/// `Fleet.location` remains the last orbiting system while `MovementOrder` is
/// the authoritative in-transit position. Rejected orders do not change the
/// world index.
pub fn begin_fleet_transit(
    state: &mut MovementState,
    world: &mut GameWorld,
    fleet: FleetKey,
    destination: SystemKey,
    transit_ticks: u32,
) -> bool {
    let origin = match world.fleets.get(fleet) {
        Some(value) => value.location,
        None => return false,
    };
    if !state.order(fleet, origin, destination, transit_ticks) {
        return false;
    }
    if let Some(system) = world.systems.get_mut(origin) {
        system.fleets.retain(|&key| key != fleet);
    }
    true
}

/// Rebuild `System.fleets` so it contains each orbiting fleet exactly once and
/// never contains an in-transit fleet.
pub fn reconcile_fleet_orbits(state: &MovementState, world: &mut GameWorld) {
    let mut orbiting: HashMap<FleetKey, SystemKey> = world
        .fleets
        .iter()
        .filter(|(fleet, _)| !state.is_in_transit(*fleet))
        .map(|(fleet, value)| (fleet, value.location))
        .collect();

    for (system_key, system) in world.systems.iter_mut() {
        system
            .fleets
            .retain(|fleet| orbiting.get(fleet) == Some(&system_key));
        system.fleets.sort_unstable();
        system.fleets.dedup();
        for fleet in &system.fleets {
            orbiting.remove(fleet);
        }
    }

    let mut missing: Vec<_> = orbiting.into_iter().collect();
    missing.sort_unstable_by_key(|(fleet, _)| *fleet);
    for (fleet, system) in missing {
        if let Some(value) = world.systems.get_mut(system) {
            value.fleets.push(fleet);
            value.fleets.sort_unstable();
        }
    }
}

/// Apply one arrival and consolidate anonymous, same-faction task forces.
///
/// Fleets carrying characters or a Death Star remain separate so explicit
/// player task-force identity is preserved. Production and ordinary AI fleets
/// can merge deterministically instead of accumulating one-ship records.
pub fn apply_fleet_arrival(
    world: &mut GameWorld,
    troop_transport: &mut TroopTransportState,
    arrival: &ArrivalEvent,
) -> Option<AppliedArrival> {
    let arriving = world.fleets.get(arrival.fleet)?;
    let is_alliance = arriving.is_alliance;
    let can_merge = arriving.characters.is_empty() && !arriving.has_death_star;

    if let Some(origin) = world.systems.get_mut(arrival.origin) {
        origin.fleets.retain(|&fleet| fleet != arrival.fleet);
    }
    if let Some(fleet) = world.fleets.get_mut(arrival.fleet) {
        fleet.location = arrival.system;
    }

    let mut compatible = Vec::new();
    if can_merge {
        if let Some(destination) = world.systems.get(arrival.system) {
            compatible.extend(destination.fleets.iter().copied().filter(|&fleet| {
                fleet != arrival.fleet
                    && world.fleets.get(fleet).map(|value| {
                        value.location == arrival.system
                            && value.is_alliance == is_alliance
                            && value.characters.is_empty()
                            && !value.has_death_star
                    }).unwrap_or(false)
            }));
        }
    }
    compatible.push(arrival.fleet);
    compatible.sort_unstable();
    compatible.dedup();

    let survivor = compatible[0];
    let absorbed_keys: Vec<_> = compatible
        .iter()
        .copied()
        .filter(|&fleet| fleet != survivor)
        .collect();
    let absorbed: Vec<_> = absorbed_keys
        .iter()
        .filter_map(|&fleet| world.fleets.remove(fleet))
        .collect();
    for &fleet in &absorbed_keys {
        troop_transport.transfer_fleet(fleet, survivor);
    }

    if let Some(fleet) = world.fleets.get_mut(survivor) {
        fleet.location = arrival.system;
        for other in absorbed {
            fleet.capital_ships.extend(other.capital_ships);
            for fighter in other.fighters {
                if let Some(entry) = fleet
                    .fighters
                    .iter_mut()
                    .find(|entry| entry.class == fighter.class)
                {
                    entry.count = entry.count.saturating_add(fighter.count);
                } else {
                    fleet.fighters.push(FighterEntry {
                        class: fighter.class,
                        count: fighter.count,
                    });
                }
            }
        }
    }

    if let Some(destination) = world.systems.get_mut(arrival.system) {
        destination
            .fleets
            .retain(|fleet| !absorbed_keys.contains(fleet) && *fleet != survivor);
        destination.fleets.push(survivor);
        destination.fleets.sort_unstable();
        destination.fleets.dedup();
    }

    Some(AppliedArrival {
        fleet: survivor,
        is_alliance,
        merged_fleets: absorbed_keys.len(),
    })
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
    /// The caller applies each event through [`apply_fleet_arrival`] so world
    /// fleet records and system orbit indexes remain canonical.
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

        // HashMap iteration order is randomized per process. Arrival order
        // mutates per-system fleet vectors downstream, so walk by fleet key.
        let mut fleet_keys: Vec<_> = state.orders.keys().copied().collect();
        fleet_keys.sort_unstable();

        // Advance all orders; collect completed ones.
        let mut completed_keys = Vec::new();
        for fleet_key in fleet_keys {
            let order = state
                .orders
                .get_mut(&fleet_key)
                .expect("movement order key collected from the same map");
            order.ticks_elapsed = order
                .ticks_elapsed
                .saturating_add(tick_count)
                .min(order.transit_ticks);

            if order.is_complete() {
                arrivals.push(ArrivalEvent {
                    fleet: fleet_key,
                    tick: final_tick,
                    origin: order.origin,
                    system: order.destination,
                });
                completed_keys.push(fleet_key);
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
    use crate::world::ControlKind;
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
    fn active_order_rejects_redispatch_without_resetting_progress() {
        let (fleet, origin, dest) = mock_fleet_and_systems();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let dest2 = sys_sm.insert(());

        let mut state = MovementState::new();
        assert!(state.order(fleet, origin, dest, 10));
        MovementSystem::advance(&mut state, &ticks(4));
        let before = state.get(fleet).unwrap().clone();

        assert!(!state.order(fleet, origin, dest2, 20));
        assert_eq!(state.len(), 1);
        assert_eq!(state.get(fleet).unwrap(), &before);
        assert_eq!(state.get(fleet).unwrap().destination, dest);
        assert_eq!(state.get(fleet).unwrap().ticks_elapsed, 4);
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
    fn simultaneous_arrivals_use_stable_fleet_key_order() {
        let mut fleet_sm: slotmap::SlotMap<FleetKey, ()> = slotmap::SlotMap::with_key();
        let fleet_a = fleet_sm.insert(());
        let fleet_b = fleet_sm.insert(());
        let fleet_c = fleet_sm.insert(());
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let origin = sys_sm.insert(());
        let destination = sys_sm.insert(());
        let mut state = MovementState::new();

        for fleet in [fleet_c, fleet_b, fleet_a] {
            state.order(fleet, origin, destination, 1);
        }

        let arrivals = MovementSystem::advance(&mut state, &ticks(1));
        let arrived_fleets: Vec<_> = arrivals.iter().map(|arrival| arrival.fleet).collect();
        assert_eq!(arrived_fleets, vec![fleet_a, fleet_b, fleet_c]);
    }

    // --- Distance-based transit tests ---

    use crate::dat::{ExplorationStatus, SectorGroup};
    use crate::ids::DatId;
    use crate::world::{
        CapitalShipClass, Character, Fleet, ForceTier, GameWorld, Sector, ShipInstance,
        SkillPair, System, TroopUnit,
    };

    fn zero_skill() -> SkillPair {
        SkillPair { base: 0, variance: 0 }
    }

    fn test_character(name: &str, hyperdrive_modifier: i16) -> Character {
        Character {
            name: name.into(),
            is_alliance: true,
            hyperdrive_modifier,
            ..Default::default()
        }
    }

    fn test_ship_class(hyperdrive: u32) -> CapitalShipClass {
        CapitalShipClass {
            name: "TestShip".into(),
            is_alliance: true,
            hull: 100,
            shield_strength: 50,
            sub_light_engine: 5,
            maneuverability: 5,
            hyperdrive,
            troop_capacity: 1,
            ..CapitalShipClass::default()
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

    fn add_test_fleet(
        world: &mut GameWorld,
        system: SystemKey,
        ship_key: crate::ids::CapitalShipKey,
    ) -> FleetKey {
        let fleet = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ShipInstance::new(ship_key, 100, true)],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        world.systems[system].fleets.push(fleet);
        fleet
    }

    #[test]
    fn transit_owns_position_until_arrival() {
        let (mut world, origin, destination) = make_transit_world(0, 0, 30, 40);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let fleet = add_test_fleet(&mut world, origin, ship_key);
        let mut movement = MovementState::new();

        assert!(begin_fleet_transit(
            &mut movement,
            &mut world,
            fleet,
            destination,
            5,
        ));
        assert!(!world.systems[origin].fleets.contains(&fleet));
        reconcile_fleet_orbits(&movement, &mut world);
        assert!(!world.systems[origin].fleets.contains(&fleet));

        let arrival = MovementSystem::advance(&mut movement, &ticks(5)).remove(0);
        let applied = apply_fleet_arrival(
            &mut world,
            &mut TroopTransportState::default(),
            &arrival,
        )
        .unwrap();
        assert_eq!(applied.fleet, fleet);
        assert_eq!(applied.merged_fleets, 0);
        assert_eq!(world.fleets[fleet].location, destination);
        assert_eq!(world.systems[destination].fleets, vec![fleet]);
    }

    #[test]
    fn reconcile_removes_transit_ghosts_and_restores_stationary_fleets() {
        let (mut world, origin, destination) = make_transit_world(0, 0, 30, 40);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let transit = add_test_fleet(&mut world, origin, ship_key);
        let stationary = add_test_fleet(&mut world, destination, ship_key);
        world.systems[origin].fleets.push(stationary);
        world.systems[destination].fleets.clear();

        let mut movement = MovementState::new();
        assert!(movement.order(transit, origin, destination, 5));
        reconcile_fleet_orbits(&movement, &mut world);

        assert!(world.systems[origin].fleets.is_empty());
        assert_eq!(world.systems[destination].fleets, vec![stationary]);
    }

    #[test]
    fn compatible_arrival_merges_into_stable_fleet_identity() {
        let (mut world, origin, destination) = make_transit_world(0, 0, 30, 40);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let survivor = add_test_fleet(&mut world, destination, ship_key);
        let arriving = add_test_fleet(&mut world, origin, ship_key);
        let arrival = ArrivalEvent {
            fleet: arriving,
            tick: 5,
            origin,
            system: destination,
        };
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: true,
            regiment_strength: 100,
        });
        world.systems[origin].ground_units.push(troop);
        let mut transport = TroopTransportState::default();
        transport.embark(&mut world, arriving, &[troop]).unwrap();

        let applied = apply_fleet_arrival(
            &mut world,
            &mut transport,
            &arrival,
        )
        .unwrap();

        assert_eq!(applied.fleet, survivor);
        assert_eq!(applied.merged_fleets, 1);
        assert!(!world.fleets.contains_key(arriving));
        assert_eq!(world.fleets[survivor].ship_count(), 2);
        assert_eq!(world.systems[destination].fleets, vec![survivor]);
        assert_eq!(transport.cargo(survivor), &[troop]);
        assert!(transport.cargo(arriving).is_empty());
    }

    #[test]
    fn character_task_force_remains_separate_on_arrival() {
        let (mut world, origin, destination) = make_transit_world(0, 0, 30, 40);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let stationed = add_test_fleet(&mut world, destination, ship_key);
        let arriving = add_test_fleet(&mut world, origin, ship_key);
        let character = world.characters.insert(test_character("Commander", 0));
        world.fleets[arriving].characters.push(character);
        let arrival = ArrivalEvent {
            fleet: arriving,
            tick: 5,
            origin,
            system: destination,
        };

        let applied = apply_fleet_arrival(
            &mut world,
            &mut TroopTransportState::default(),
            &arrival,
        )
        .unwrap();

        assert_eq!(applied.fleet, arriving);
        assert_eq!(applied.merged_fleets, 0);
        assert!(world.fleets.contains_key(stationed));
        assert!(world.fleets.contains_key(arriving));
        assert_eq!(world.systems[destination].fleets, vec![stationed, arriving]);
    }

    #[test]
    fn faction_dispatch_validates_and_begins_one_authoritative_order() {
        let (mut world, origin, destination) = make_transit_world(0, 0, 30, 40);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let fleet = add_test_fleet(&mut world, origin, ship_key);
        let mut movement = MovementState::new();

        assert_eq!(
            validate_fleet_dispatch(&movement, &world, fleet, destination, false),
            Err(FleetDispatchError::WrongFaction),
        );
        assert_eq!(
            validate_fleet_dispatch(&movement, &world, fleet, origin, true),
            Err(FleetDispatchError::AlreadyAtDestination),
        );

        world.fleets[fleet].capital_ships.clear();
        assert_eq!(
            validate_fleet_dispatch(&movement, &world, fleet, destination, true),
            Err(FleetDispatchError::EmptyFleet),
        );
        world.fleets[fleet]
            .capital_ships
            .push(ShipInstance::new(ship_key, 100, true));

        let sector = world.systems[origin].sector;
        let missing = world.systems.insert(make_system(sector, 60, 80));
        world.systems.remove(missing);
        assert_eq!(
            validate_fleet_dispatch(&movement, &world, fleet, missing, true),
            Err(FleetDispatchError::MissingDestination),
        );

        world.systems[destination].is_destroyed = true;
        assert_eq!(
            validate_fleet_dispatch(&movement, &world, fleet, destination, true),
            Err(FleetDispatchError::DestinationDestroyed),
        );
        world.systems[destination].is_destroyed = false;

        let departure = begin_faction_fleet_transit(
            &mut movement,
            &mut world,
            fleet,
            destination,
            true,
            &MovementConfig::default(),
        )
        .unwrap();
        assert_eq!(departure.origin, origin);
        assert_eq!(departure.destination, destination);
        assert_eq!(departure.transit_ticks, 10);
        assert!(!world.systems[origin].fleets.contains(&fleet));
        assert_eq!(movement.get(fleet).unwrap().destination, destination);
        assert_eq!(
            validate_fleet_dispatch(&movement, &world, fleet, destination, true),
            Err(FleetDispatchError::AlreadyInTransit),
        );
    }

    #[test]
    fn short_distance_clamps_to_min() {
        // 50 units apart, hyperdrive=80 → ceil(50*2/80)=ceil(1.25)=2 → clamped to MIN=10
        let (mut world, origin, dest) = make_transit_world(0, 0, 30, 40); // distance=50
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let fleet = Fleet {
            location: origin,
            capital_ships: vec![ShipInstance::new(ship_key, 100, true)],
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
            capital_ships: vec![ShipInstance::new(ship_key, 100, true)],
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
            capital_ships: vec![ShipInstance::new(ship_key, 100, true)],
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
                ShipInstance::new(fast_key, 100, true),
                ShipInstance::new(slow_key, 100, true),
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
            capital_ships: vec![ShipInstance::new(ship_key, 100, true)],
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
            capital_ships: vec![ShipInstance::new(ship_key, 100, true)],
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
            capital_ships: vec![ShipInstance::new(ship_key, 100, true)],
            fighters: vec![],
            characters: vec![han_key],
            is_alliance: true,
            has_death_star: false,
        };
        assert_eq!(fleet_transit_ticks(&fleet, &world, origin, dest), MIN_TRANSIT_TICKS);
    }
}
