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
//! Transit time follows the original's per-object rule (`FUN_00514a60` ->
//! `FUN_00556430` -> `FUN_0055d8c0`, `ghidra/notes/build-delivery.md`):
//! ```text
//! transit_ticks = max(1, isqrt(dx^2 + dy^2) / GNPRTB[5120] * speed / 100)
//! ```
//! A fleet travels at its slowest capital ship's speed; a fleet of fighters
//! alone cannot enter hyperspace. Travel is point to point, with no lanes or
//! waypoints.
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
//! // for event in &arrivals { apply_fleet_arrival(&mut world, &state, &mut cargo, event); }
//! ```

use std::collections::{HashMap, HashSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::dat::Faction;
use crate::ids::{FleetKey, SystemKey};
use crate::tick::TickEvent;
use crate::troop_transport::TroopTransportState;
use crate::world::{CapitalShipClass, FighterClass, Fleet, GameWorld};

// ---------------------------------------------------------------------------
// Transit time
// ---------------------------------------------------------------------------

/// GNPRTB parameter dividing distance in the transit formula (`0x1400`,
/// `DAT_006bb6e8`, loaded by `FUN_0055cb60`).
pub const GNPRTB_TRANSIT_DISTANCE_DIVISOR: u16 = 5120;
/// GNPRTB parameter holding the default object speed (`DAT_006b9050`, loaded
/// by `FUN_0053e0b0`); also the percent base of `FUN_0053e190`.
pub const GNPRTB_DEFAULT_SPEED: u16 = 1;
/// Shipped GNPRTB.DAT value of parameter 5120 in every column, used only when
/// no GNPRTB table is loaded.
const SHIPPED_TRANSIT_DISTANCE_DIVISOR: i64 = 5;
/// Shipped GNPRTB.DAT value of parameter 1 in every column, used only when no
/// GNPRTB table is loaded.
const SHIPPED_DEFAULT_SPEED: i64 = 100;

/// A GNPRTB value, or its shipped value when the table is not loaded.
pub(crate) fn gnprtb_or_shipped(world: &GameWorld, id: u16, shipped: i64) -> i64 {
    match world.gnprtb.value(id, world.difficulty_index) {
        0 => shipped,
        value => i64::from(value),
    }
}

/// Integer square root exactly as `FUN_0053e1d0` computes it: a doubling
/// search for an upper bound, then a bisection that settles on the root.
#[must_use]
pub fn original_isqrt(value: i64) -> i64 {
    if value <= 3 {
        return i64::from(value > 0);
    }
    let mut high = 2;
    if value > 4 {
        loop {
            high *= 2;
            if high * high >= value {
                break;
            }
        }
    }
    let mut low = high / 2;
    let mut upper = high;
    let mut previous = high;
    loop {
        let mid = (low + upper) / 2;
        let square = mid * mid;
        let mut next_upper = mid;
        if square <= value {
            next_upper = upper;
            if square < value {
                low = mid;
            }
        }
        upper = next_upper;
        if mid == previous {
            return mid;
        }
        previous = mid;
    }
}

/// Transit days between two positions at `speed`, as `FUN_0055d8c0` computes
/// them: `(isqrt(dx^2 + dy^2) / GNPRTB[5120]) * speed / 100` in integer steps
/// (`FUN_0053e190` -> `FUN_0053e170` -> `FUN_0053e150`), 0 for the same
/// position, and at least 1 otherwise. `speed` 100 is the default; larger is
/// slower.
#[must_use]
pub fn transit_ticks_between(world: &GameWorld, from: (u16, u16), to: (u16, u16), speed: i64) -> u32 {
    let dx = i64::from(to.0) - i64::from(from.0);
    let dy = i64::from(to.1) - i64::from(from.1);
    let distance = original_isqrt(dx * dx + dy * dy);
    if distance == 0 {
        return 0;
    }
    let divisor = gnprtb_or_shipped(world, GNPRTB_TRANSIT_DISTANCE_DIVISOR, SHIPPED_TRANSIT_DISTANCE_DIVISOR);
    let percent = gnprtb_or_shipped(world, GNPRTB_DEFAULT_SPEED, SHIPPED_DEFAULT_SPEED);
    let ticks = (distance / divisor) * speed / percent;
    u32::try_from(ticks.max(1)).unwrap_or(u32::MAX)
}

/// Speed of one capital ship (`FUN_00500820`): the class `hyperdrive`, else
/// `hyperdrive_if_damaged`, else the GNPRTB 1 default.
///
/// The original subtracts `hyperdrive` when a per-ship damage nibble (ship
/// `+0x64` bits 16..19, `FUN_005011f0`) is set; the port does not model that
/// nibble, so every ship reads as undamaged.
#[must_use]
pub fn capital_ship_speed(world: &GameWorld, class: &CapitalShipClass) -> i64 {
    rated_speed(world, class.hyperdrive, class.hyperdrive_if_damaged)
}

/// Speed of one fighter squadron (`FUN_00502f80`): the same rule as a
/// capital ship over its FIGHTSD ratings, with no damage subtrahend.
#[must_use]
pub fn fighter_speed(world: &GameWorld, class: &FighterClass) -> i64 {
    rated_speed(world, class.hyperdrive, class.hyperdrive_if_damaged)
}

/// The GNPRTB 1 default speed, 100 in every shipped column; regiments and
/// facilities travel at it (`FUN_004f63f0`).
#[must_use]
pub fn default_speed(world: &GameWorld) -> i64 {
    gnprtb_or_shipped(world, GNPRTB_DEFAULT_SPEED, SHIPPED_DEFAULT_SPEED)
}

/// `hyperdrive`, else `hyperdrive_if_damaged`, else the GNPRTB 1 default
/// (`FUN_00500820`, `FUN_00502f80.c:19`).
fn rated_speed(world: &GameWorld, hyperdrive: u32, damaged: u32) -> i64 {
    match (hyperdrive, damaged) {
        (0, 0) => default_speed(world),
        (0, damaged) => i64::from(damaged),
        (hyperdrive, _) => i64::from(hyperdrive),
    }
}

/// Speed of a fleet (`FUN_004fd900`): the slowest, that is the largest, speed
/// among its capital ships; 0 as soon as one member reads 0. `None` when the
/// fleet cannot enter hyperspace because no capital ship carries it
/// (`FUN_004fda10` walks capital ships `0x14..0x1c` only, so fighters alone
/// cannot move).
#[must_use]
pub fn fleet_speed(fleet: &Fleet, world: &GameWorld) -> Option<i64> {
    let mut speeds = fleet
        .capital_ships
        .iter()
        .filter(|ship| ship.alive)
        .filter_map(|ship| world.capital_ship_classes.get(ship.class))
        .map(|class| capital_ship_speed(world, class));
    let first = speeds.next()?;
    Some(speeds.fold(first, |slowest, speed| if slowest == 0 || speed == 0 { 0 } else { slowest.max(speed) }))
}

/// Transit days for a fleet between two systems, or `None` when the fleet
/// cannot enter hyperspace (see [`fleet_speed`]).
#[must_use]
pub fn fleet_transit_ticks(fleet: &Fleet, world: &GameWorld, origin: SystemKey, dest: SystemKey) -> Option<u32> {
    let speed = fleet_speed(fleet, world)?;
    let position = |key: SystemKey| world.systems.get(key).map_or((0, 0), |s| (s.x, s.y));
    Some(transit_ticks_between(world, position(origin), position(dest), speed))
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
    /// The fleet this one joins on arrival, when it was sent onto a fleet
    /// (`ghidra/notes/fleet-join-split.md`).
    pub join: Option<FleetKey>,
}

impl MovementOrder {
    /// Create a new movement order.
    #[must_use]
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
            join: None,
        }
    }

    /// Progress fraction in [0.0, 1.0] — 0.0 = just departed, 1.0 = arrived.
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
    )]
    pub fn progress(&self) -> f32 {
        if self.transit_ticks == 0 {
            return 1.0;
        }
        (self.ticks_elapsed as f32 / self.transit_ticks as f32).min(1.0)
    }

    /// True if the fleet has completed transit.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.ticks_elapsed >= self.transit_ticks
    }

    /// Remaining ticks until arrival.
    #[must_use]
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
    #[must_use]
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

    /// Mark a fleet in transit to join `target` on arrival.
    pub fn set_join(&mut self, fleet: FleetKey, target: FleetKey) {
        if let Some(order) = self.orders.get_mut(&fleet) {
            order.join = Some(target);
        }
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
    #[must_use]
    pub fn get(&self, fleet: FleetKey) -> Option<&MovementOrder> {
        self.orders.get(&fleet)
    }

    /// Whether a fleet currently has an active hyperspace order.
    #[must_use]
    pub fn is_in_transit(&self, fleet: FleetKey) -> bool {
        self.orders.contains_key(&fleet)
    }

    /// All active orders (immutable).
    #[must_use]
    pub fn orders(&self) -> &HashMap<FleetKey, MovementOrder> {
        &self.orders
    }

    /// All active orders (mutable) — for testing and manual state setup.
    pub fn orders_mut(&mut self) -> &mut HashMap<FleetKey, MovementOrder> {
        &mut self.orders
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.orders.len()
    }

    #[must_use]
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
    /// The fleet it was sent to join, if any.
    pub join: Option<FleetKey>,
}

/// Result of applying one arrival to the canonical world fleet indexes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppliedArrival {
    /// The fleet now at the destination: the arrival, or the fleet it joined.
    pub fleet: FleetKey,
    /// The arriving fleet's side.
    pub is_alliance: bool,
    /// 1 when the arrival joined another fleet, else 0.
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
    /// No living capital ship can carry the fleet through hyperspace
    /// (`FUN_004fda10`).
    NoHyperdrive,
    /// A system window drag cannot move a fleet out of a blockaded system
    /// (`FUN_00537180` → `FUN_00552210`: `1`/`1`).
    Blockaded,
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
            Self::NoHyperdrive => "fleet has no capital ship to carry it through hyperspace",
            Self::Blockaded => "fleet is held in its system by a blockade",
        };
        formatter.write_str(message)
    }
}

/// The fleet, when a Move or Confirmed Move order may be built for it: the
/// rows' enabled slot `+0x18` (`FUN_0053c100` → `FUN_00578c00`;
/// `ghidra/notes/move-order.md`). The fleet must resolve (`1`/`0x12`) and pass
/// the object check `FUN_004f9860` through `FUN_004fdc70`: the order's side
/// and not en route. The blockade escape `FUN_005555e0` covers only ALLFACSD
/// objects, so it never refuses a fleet.
fn movable_fleet<'a>(
    state: &MovementState,
    world: &'a GameWorld,
    fleet: FleetKey,
    expected_is_alliance: bool,
) -> Result<&'a Fleet, FleetDispatchError> {
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
    Ok(value)
}

/// Whether the fleet's pop-up menu enables Move and Confirmed Move. No
/// destination is known yet, so a fleet that cannot enter hyperspace is
/// still enabled; the release refuses it (`FUN_00578d00`).
#[must_use]
pub fn fleet_move_enabled(
    state: &MovementState,
    world: &GameWorld,
    fleet: FleetKey,
    expected_is_alliance: bool,
) -> bool {
    movable_fleet(state, world, fleet, expected_is_alliance).is_ok()
}

/// Whether a fleet's valid move asks for confirmation before it departs:
/// `FUN_00487cc0`. Confirmed Move (`0x202`, `confirmed`) always asks. Move
/// (`0x201`) asks only when the fleet's system is blockaded (`+0x88` bit
/// `0x20`) and the system's side bits (`+0x24` bits 6-7, [`ControlKind`](crate::world::ControlKind))
/// equal the fleet's. port: an uprising keeps its faction's side, as
/// [`ControlKind::is_controlled_by`](crate::world::ControlKind::is_controlled_by) reads it.
#[must_use]
pub fn fleet_move_confirms(
    world: &GameWorld,
    blockaded: &HashSet<SystemKey>,
    fleet: FleetKey,
    confirmed: bool,
) -> bool {
    if confirmed {
        return true;
    }
    let Some(value) = world.fleets.get(fleet) else {
        return false;
    };
    let Some(system) = world.systems.get(value.location) else {
        return false;
    };
    let side = if value.is_alliance {
        Faction::Alliance
    } else {
        Faction::Empire
    };
    blockaded.contains(&value.location) && system.control.is_controlled_by(side)
}

/// Validate a player-facing fleet dispatch without mutating the campaign:
/// the move command's validator with a destination, `FUN_00578d00`.
///
/// After [`fleet_move_enabled`]'s checks, the destination must resolve
/// (`1`/`0x22`), and a fleet moving between systems needs a non-zero speed
/// (`FUN_00555920`, `FUN_004fd900`: `1`/`0x18`).
///
/// port: the origin, destroyed-destination and same-system checks are the
/// port's own. The original's validator accepts a move within one system (its
/// leg builders are not read). An empty fleet meets the speed refusal.
///
/// # Errors
/// Returns a dispatch error for missing entities, a faction mismatch, a fleet
/// without a capital ship, an active transit order, or an invalid
/// destination.
pub fn validate_fleet_dispatch(
    state: &MovementState,
    world: &GameWorld,
    fleet: FleetKey,
    destination: SystemKey,
    expected_is_alliance: bool,
) -> Result<(), FleetDispatchError> {
    let value = movable_fleet(state, world, fleet, expected_is_alliance)?;
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
    if fleet_speed(value, world).is_none() {
        return Err(FleetDispatchError::NoHyperdrive);
    }
    Ok(())
}

/// The fleets an order built on a sector window's fleet icon acts on: the
/// system's fleets (`FUN_00512700`, kind `0x10` → `FUN_004ffe70`) of the
/// order's side (`FUN_00553350`), in the system's order
/// (`ghidra/notes/sector-icon-menus.md`).
#[must_use]
pub fn system_side_fleets(
    world: &GameWorld,
    system: SystemKey,
    expected_is_alliance: bool,
) -> Vec<FleetKey> {
    world.systems.get(system).map_or_else(Vec::new, |value| {
        value
            .fleets
            .iter()
            .copied()
            .filter(|fleet| {
                world
                    .fleets
                    .get(*fleet)
                    .is_some_and(|value| value.is_alliance == expected_is_alliance)
            })
            .collect()
    })
}

/// Whether Move and Confirmed Move are enabled for a team of fleets:
/// `FUN_0053c100` → `FUN_0053c4b0` refuses a team with no member (`1`/`0x16`),
/// then asks every sub-order's `+0x18` ([`fleet_move_enabled`]).
#[must_use]
pub fn fleets_move_enabled(
    state: &MovementState,
    world: &GameWorld,
    fleets: &[FleetKey],
    expected_is_alliance: bool,
) -> bool {
    !fleets.is_empty()
        && fleets
            .iter()
            .all(|fleet| fleet_move_enabled(state, world, *fleet, expected_is_alliance))
}

/// Validate a team of fleets' move with a destination: the move order's
/// validator `FUN_0053c1a0` runs every sub-order's `+0x1c`, so one refusal
/// refuses them all.
///
/// # Errors
/// Returns [`FleetDispatchError::MissingFleet`] for an empty team, or the
/// first error [`validate_fleet_dispatch`] returns for a member.
pub fn validate_fleets_dispatch(
    state: &MovementState,
    world: &GameWorld,
    fleets: &[FleetKey],
    destination: SystemKey,
    expected_is_alliance: bool,
) -> Result<(), FleetDispatchError> {
    if fleets.is_empty() {
        return Err(FleetDispatchError::MissingFleet);
    }
    fleets.iter().try_for_each(|fleet| {
        validate_fleet_dispatch(state, world, *fleet, destination, expected_is_alliance)
    })
}

/// Validate the Destination order (`0x214`) a system window drag issues for
/// a fleet: `FUN_00537180` (`ghidra/notes/move-order.md`, "Order 0x214").
///
/// The order never confirms (`FUN_00487cc0` has no case for it), and its
/// enemy-destination refusal (`1`/`0x28`) reads child lists that a sided
/// fleet does not have (`FUN_00528820`). A fleet whose `+0x58` holds its
/// system's blockade bit (`FUN_0050c0b0`) is refused (`FUN_00552210` →
/// `FUN_005287f0`: `1`/`1`), whichever side it is on.
///
/// hyp: a fleet's `+0x58` is otherwise zero, and the order's route checks
/// and the fleet's command slots refuse what [`validate_fleet_dispatch`]
/// refuses.
///
/// # Errors
/// Returns [`FleetDispatchError::Blockaded`] for a fleet in a blockaded
/// system, or any error [`validate_fleet_dispatch`] returns.
pub fn validate_fleet_destination(
    state: &MovementState,
    world: &GameWorld,
    blockaded: &HashSet<SystemKey>,
    fleet: FleetKey,
    destination: SystemKey,
    expected_is_alliance: bool,
) -> Result<(), FleetDispatchError> {
    validate_fleet_dispatch(state, world, fleet, destination, expected_is_alliance)?;
    if world
        .fleets
        .get(fleet)
        .is_some_and(|value| blockaded.contains(&value.location))
    {
        return Err(FleetDispatchError::Blockaded);
    }
    Ok(())
}

/// Validate, time, and begin a faction-controlled fleet departure.
///
/// # Errors
/// Returns a dispatch error when fleet or destination validation fails,
/// or the fleet already has a transit order.
pub fn begin_faction_fleet_transit(
    state: &mut MovementState,
    world: &mut GameWorld,
    fleet: FleetKey,
    destination: SystemKey,
    expected_is_alliance: bool,
) -> Result<AppliedDeparture, FleetDispatchError> {
    validate_fleet_dispatch(state, world, fleet, destination, expected_is_alliance)?;
    let value = world
        .fleets
        .get(fleet)
        .ok_or(FleetDispatchError::MissingFleet)?;
    let origin = value.location;
    let is_alliance = value.is_alliance;
    let transit_ticks = fleet_transit_ticks(value, world, origin, destination)
        .ok_or(FleetDispatchError::NoHyperdrive)?;
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

/// The fleets a window lists at `system`: those in orbit and those en route
/// to it. A move changes the fleet's container at once (`FUN_00556390`,
/// slot `+0xa8`; `ghidra/notes/move-order.md`), so the original's windows
/// list a fleet in hyperspace at its destination, with the en route overlay
/// for `+0x50` bit 4 (`fleet-window.md`). The port keeps the origin in
/// `Fleet.location` and `System.fleets` as the orbit index the simulation
/// reads, so the destination comes from the order.
#[must_use]
pub fn listed_fleets(state: &MovementState, world: &GameWorld, system: SystemKey) -> Vec<FleetKey> {
    let mut fleets: Vec<FleetKey> = world
        .systems
        .get(system)
        .map(|value| value.fleets.clone())
        .unwrap_or_default();
    fleets.extend(
        state
            .orders
            .values()
            .filter(|order| order.destination == system && world.fleets.contains_key(order.fleet))
            .map(|order| order.fleet),
    );
    fleets.sort_unstable();
    fleets.dedup();
    fleets
}

/// The system a window lists `fleet` at: its destination while en route,
/// else where it orbits.
#[must_use]
pub fn listed_location(
    state: &MovementState,
    world: &GameWorld,
    fleet: FleetKey,
) -> Option<SystemKey> {
    state
        .get(fleet)
        .map(|order| order.destination)
        .or_else(|| world.fleets.get(fleet).map(|value| value.location))
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

    for (system_key, system) in &mut world.systems {
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

/// Apply one arrival: the fleet enters its destination's orbit. A fleet sent
/// onto another fleet joins it when that fleet is still there, of its side
/// and not en route (`ghidra/notes/fleet-join-split.md`); otherwise it stays
/// a fleet of its own. Fleets that merely meet stay apart: the original has
/// no rule that merges them.
pub fn apply_fleet_arrival(
    world: &mut GameWorld,
    movement: &MovementState,
    troop_transport: &mut TroopTransportState,
    arrival: &ArrivalEvent,
) -> Option<AppliedArrival> {
    let is_alliance = world.fleets.get(arrival.fleet)?.is_alliance;

    if let Some(origin) = world.systems.get_mut(arrival.origin) {
        origin.fleets.retain(|&fleet| fleet != arrival.fleet);
    }
    if let Some(fleet) = world.fleets.get_mut(arrival.fleet) {
        fleet.location = arrival.system;
    }
    if let Some(destination) = world.systems.get_mut(arrival.system) {
        destination.fleets.push(arrival.fleet);
        destination.fleets.sort_unstable();
        destination.fleets.dedup();
    }
    // port: an arrival ends a hold (`TroopTransportState::load`).
    troop_transport.release(arrival.fleet);

    let target = arrival.join.filter(|&target| {
        target != arrival.fleet
            && crate::fleet_join::joins_on_arrival(
                world,
                movement,
                target,
                arrival.system,
                is_alliance,
            )
    });
    if let Some(target) = target {
        crate::fleet_join::merge_fleet_into(world, troop_transport, arrival.fleet, target);
        return Some(AppliedArrival {
            fleet: target,
            is_alliance,
            merged_fleets: 1,
        });
    }
    Some(AppliedArrival {
        fleet: arrival.fleet,
        is_alliance,
        merged_fleets: 0,
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
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
    )]
    ///
    /// # Panics
    /// Panics if an order key collected for this batch is absent when its order is advanced.
    pub fn advance(state: &mut MovementState, tick_events: &[TickEvent]) -> Vec<ArrivalEvent> {
        let Some(last_tick_event) = tick_events.last() else {
            return Vec::new();
        };

        let tick_count = tick_events.len() as u32;
        let final_tick = last_tick_event.tick;
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
                    join: order.join,
                });
                completed_keys.push(fleet_key);
            }
        }

        // Remove completed orders.
        for key in completed_keys {
            state.orders.remove(&key);
        }

        // A fleet that joins another on arrival comes in after the rest, so
        // a target arriving in the same pass is already in orbit
        // (`apply_fleet_arrival`). port: the order within each group stays
        // by fleet key.
        arrivals.sort_by_key(|arrival| arrival.join.is_some());
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
    use crate::world::{ControlKind, FighterEntry};

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
    #[expect(
        clippy::float_cmp,
        reason = "Transit endpoints are exactly zero and one."
    )]
    fn progress_starts_at_zero() {
        let (fleet, origin, dest) = mock_fleet_and_systems();
        let order = MovementOrder::new(fleet, origin, dest, 10);
        assert_eq!(order.progress(), 0.0);
        assert!(!order.is_complete());
        assert_eq!(order.ticks_remaining(), 10);
    }

    #[test]
    fn progress_reports_half_at_midpoint() {
        let (fleet, origin, dest) = mock_fleet_and_systems();
        let mut order = MovementOrder::new(fleet, origin, dest, 10);
        order.ticks_elapsed = 5;
        assert!((order.progress() - 0.5).abs() < 1e-6);
        assert_eq!(order.ticks_remaining(), 5);
    }

    #[test]
    #[expect(
        clippy::float_cmp,
        reason = "Transit endpoints are exactly zero and one."
    )]
    fn progress_clamps_at_one() {
        let (fleet, origin, dest) = mock_fleet_and_systems();
        let mut order = MovementOrder::new(fleet, origin, dest, 5);
        order.ticks_elapsed = 10; // overshoot
        assert_eq!(order.progress(), 1.0);
        assert!(order.is_complete());
    }

    // --- MovementState ---

    #[test]
    fn ordered_movement_is_retrievable_by_fleet() {
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
        CapitalShipClass, Character, Fleet, GameWorld, Sector, ShipInstance, System, TroopUnit,
    };

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
            &MovementState::new(),
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
    fn a_fleet_in_hyperspace_is_listed_at_its_destination() {
        // move-order.md: FUN_00556390 changes the fleet's container at once,
        // so its destination's windows list it while it travels.
        let (mut world, origin, destination) = make_transit_world(0, 0, 30, 40);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let fleet = add_test_fleet(&mut world, origin, ship_key);
        let stationed = add_test_fleet(&mut world, origin, ship_key);
        let mut movement = MovementState::new();
        assert!(begin_fleet_transit(
            &mut movement,
            &mut world,
            fleet,
            destination,
            5
        ));

        assert_eq!(listed_fleets(&movement, &world, origin), vec![stationed]);
        assert_eq!(listed_fleets(&movement, &world, destination), vec![fleet]);
        assert_eq!(listed_location(&movement, &world, fleet), Some(destination));
        assert_eq!(listed_location(&movement, &world, stationed), Some(origin));

        let arrival = MovementSystem::advance(&mut movement, &ticks(5)).remove(0);
        apply_fleet_arrival(
            &mut world,
            &movement,
            &mut TroopTransportState::default(),
            &arrival,
        );
        assert_eq!(listed_fleets(&movement, &world, destination), vec![fleet]);
        assert!(!movement.is_in_transit(fleet));
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

    // ghidra/notes/fleet-join-split.md: no original rule merges fleets that
    // meet; only a move onto a fleet joins it.
    #[test]
    fn fleets_of_one_side_meeting_on_arrival_stay_apart() {
        let (mut world, origin, destination) = make_transit_world(0, 0, 30, 40);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let stationed = add_test_fleet(&mut world, destination, ship_key);
        let arriving = add_test_fleet(&mut world, origin, ship_key);
        let arrival = ArrivalEvent {
            fleet: arriving,
            tick: 5,
            origin,
            system: destination,
            join: None,
        };
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: true,
            regiment_strength: 100,
        });
        world.systems[origin].ground_units.push(troop);
        let mut transport = TroopTransportState::default();
        transport.embark(&mut world, arriving, &[troop]).unwrap();

        let applied =
            apply_fleet_arrival(&mut world, &MovementState::new(), &mut transport, &arrival)
                .unwrap();

        assert_eq!(applied.fleet, arriving);
        assert_eq!(applied.merged_fleets, 0);
        assert_eq!(world.fleets[stationed].ship_count(), 1);
        assert_eq!(world.fleets[arriving].ship_count(), 1);
        assert_eq!(world.systems[destination].fleets, vec![stationed, arriving]);
        assert_eq!(transport.cargo(arriving), &[troop]);
    }

    // ghidra/notes/fleet-join-split.md: a move onto a fleet in another system
    // ends with the movers in that fleet (FUN_004feca0; port: on arrival).
    #[test]
    fn a_fleet_sent_onto_another_joins_it_on_arrival_with_its_cargo() {
        let (mut world, origin, destination) = make_transit_world(0, 0, 30, 40);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let target = add_test_fleet(&mut world, destination, ship_key);
        let arriving = add_test_fleet(&mut world, origin, ship_key);
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: true,
            regiment_strength: 100,
        });
        world.systems[origin].ground_units.push(troop);
        let mut transport = TroopTransportState::default();
        transport.embark(&mut world, arriving, &[troop]).unwrap();
        let mut movement = MovementState::new();
        assert!(begin_fleet_transit(
            &mut movement,
            &mut world,
            arriving,
            destination,
            5
        ));
        movement.set_join(arriving, target);

        let arrival = MovementSystem::advance(&mut movement, &ticks(5)).remove(0);
        assert_eq!(arrival.join, Some(target));
        let applied = apply_fleet_arrival(&mut world, &movement, &mut transport, &arrival).unwrap();

        assert_eq!(applied.fleet, target);
        assert_eq!(applied.merged_fleets, 1);
        assert!(!world.fleets.contains_key(arriving));
        assert_eq!(world.fleets[target].ship_count(), 2);
        assert_eq!(world.systems[destination].fleets, vec![target]);
        assert_eq!(transport.cargo(target), &[troop]);
    }

    // port: a fleet sent onto another that arrives in the same pass comes in
    // after it, so it finds its target in orbit.
    #[test]
    fn a_fleet_arriving_with_its_target_in_one_pass_still_joins_it() {
        let (mut world, origin, destination) = make_transit_world(0, 0, 30, 40);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        // The joiner has the lower key, so it would come in first by key.
        let arriving = add_test_fleet(&mut world, origin, ship_key);
        let target = add_test_fleet(&mut world, origin, ship_key);
        assert!(arriving < target);
        let mut movement = MovementState::new();
        for fleet in [arriving, target] {
            assert!(begin_fleet_transit(
                &mut movement,
                &mut world,
                fleet,
                destination,
                5
            ));
        }
        movement.set_join(arriving, target);

        let arrivals = MovementSystem::advance(&mut movement, &ticks(5));
        assert_eq!(
            arrivals
                .iter()
                .map(|arrival| arrival.fleet)
                .collect::<Vec<_>>(),
            [target, arriving]
        );
        let mut transport = TroopTransportState::default();
        for arrival in &arrivals {
            apply_fleet_arrival(&mut world, &movement, &mut transport, arrival).unwrap();
        }

        assert!(!world.fleets.contains_key(arriving));
        assert_eq!(world.fleets[target].ship_count(), 2);
        assert_eq!(world.systems[destination].fleets, vec![target]);
    }

    // port: the original's member follows its fleet anywhere; the port's
    // fleet joins only a target still in the system it arrives at.
    #[test]
    fn a_fleet_whose_target_has_left_stays_a_fleet_of_its_own() {
        let (mut world, origin, destination) = make_transit_world(0, 0, 30, 40);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let target = add_test_fleet(&mut world, destination, ship_key);
        let arriving = add_test_fleet(&mut world, origin, ship_key);
        let mut movement = MovementState::new();
        assert!(begin_fleet_transit(
            &mut movement,
            &mut world,
            arriving,
            destination,
            5
        ));
        movement.set_join(arriving, target);
        assert!(begin_fleet_transit(
            &mut movement,
            &mut world,
            target,
            origin,
            50
        ));

        let arrival = MovementSystem::advance(&mut movement, &ticks(5)).remove(0);
        let applied = apply_fleet_arrival(
            &mut world,
            &movement,
            &mut TroopTransportState::default(),
            &arrival,
        )
        .unwrap();

        assert_eq!((applied.fleet, applied.merged_fleets), (arriving, 0));
        assert_eq!(world.fleets[arriving].location, destination);
        assert_eq!(world.fleets[target].ship_count(), 1);
    }

    // port: an arrival ends a hold (`TroopTransportState::load`), so the
    // regiment lands where the fleet goes.
    #[test]
    fn a_held_fleet_arriving_elsewhere_releases_its_cargo_to_land() {
        let (mut world, origin, destination) = make_transit_world(0, 0, 30, 40);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let fleet = add_test_fleet(&mut world, origin, ship_key);
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: true,
            regiment_strength: 100,
        });
        world.systems[origin].ground_units.push(troop);
        let mut transport = TroopTransportState::default();
        transport.load(&mut world, fleet, &[troop]).unwrap();
        let mut movement = MovementState::new();
        assert!(begin_fleet_transit(
            &mut movement,
            &mut world,
            fleet,
            destination,
            5,
        ));
        assert!(transport.is_held(fleet));

        let arrival = MovementSystem::advance(&mut movement, &ticks(5)).remove(0);
        apply_fleet_arrival(&mut world, &movement, &mut transport, &arrival).unwrap();

        assert!(!transport.is_held(fleet));
        assert_eq!(transport.landing_count(fleet), 1);
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
            join: None,
        };

        let applied = apply_fleet_arrival(
            &mut world,
            &MovementState::new(),
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
    fn a_fleet_may_take_a_move_on_its_own_side_outside_hyperspace() {
        // FUN_00578c00 → FUN_004fdc70 → FUN_004f9860: the order's side, and
        // not en route (ghidra/notes/move-order.md, "The move command").
        let (mut world, origin, destination) = make_transit_world(0, 0, 30, 40);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let fleet = add_test_fleet(&mut world, origin, ship_key);
        let mut movement = MovementState::new();

        assert!(fleet_move_enabled(&movement, &world, fleet, true));
        assert!(!fleet_move_enabled(&movement, &world, fleet, false));

        begin_faction_fleet_transit(&mut movement, &mut world, fleet, destination, true)
            .expect("the fleet departs");
        assert!(!fleet_move_enabled(&movement, &world, fleet, true));

        world.fleets.remove(fleet);
        let idle = MovementState::new();
        assert!(!fleet_move_enabled(&idle, &world, fleet, true));
    }

    #[test]
    fn a_fleet_icons_team_is_the_systems_fleets_of_the_orders_side_in_order() {
        // FUN_00512700, kind 0x10: FUN_004ffe70's walk of the system's
        // fleets, kept to the order's side by FUN_00553350.
        let (mut world, origin, destination) = make_transit_world(0, 0, 30, 40);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let first = add_test_fleet(&mut world, origin, ship_key);
        let enemy = add_test_fleet(&mut world, origin, ship_key);
        world.fleets[enemy].is_alliance = false;
        let second = add_test_fleet(&mut world, origin, ship_key);
        add_test_fleet(&mut world, destination, ship_key);

        assert_eq!(system_side_fleets(&world, origin, true), [first, second]);
        assert_eq!(system_side_fleets(&world, origin, false), [enemy]);
    }

    #[test]
    fn a_team_of_fleets_may_move_only_when_every_member_may() {
        // FUN_0053c100 -> FUN_0053c4b0: an empty team is refused 1/0x16, and
        // each sub-order's +0x18 must pass.
        let (mut world, origin, destination) = make_transit_world(0, 0, 30, 40);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let first = add_test_fleet(&mut world, origin, ship_key);
        let second = add_test_fleet(&mut world, origin, ship_key);
        let mut movement = MovementState::new();

        assert!(fleets_move_enabled(&movement, &world, &[first, second], true));
        assert!(!fleets_move_enabled(&movement, &world, &[], true));
        begin_faction_fleet_transit(&mut movement, &mut world, second, destination, true)
            .expect("the second fleet departs");
        assert!(!fleets_move_enabled(&movement, &world, &[first, second], true));
        assert!(fleets_move_enabled(&movement, &world, &[first], true));
    }

    #[test]
    fn one_refused_fleet_refuses_the_whole_teams_move() {
        // FUN_0053c1a0 runs every sub-order's validator +0x1c.
        let (mut world, origin, destination) = make_transit_world(0, 0, 30, 40);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let first = add_test_fleet(&mut world, origin, ship_key);
        let second = add_test_fleet(&mut world, origin, ship_key);
        let movement = MovementState::new();

        assert_eq!(
            validate_fleets_dispatch(&movement, &world, &[first, second], destination, true),
            Ok(())
        );
        world.fleets[second].capital_ships.clear();
        assert_eq!(
            validate_fleets_dispatch(&movement, &world, &[first, second], destination, true),
            Err(FleetDispatchError::NoHyperdrive)
        );
        assert_eq!(
            validate_fleets_dispatch(&movement, &world, &[], destination, true),
            Err(FleetDispatchError::MissingFleet)
        );
    }

    #[test]
    fn a_move_asks_first_only_from_its_own_sides_blockaded_system() {
        // FUN_00487cc0: 0x202 always confirms; 0x201 confirms when the
        // fleet's system has the blockade bit and the fleet's side bits.
        let (mut world, origin, _) = make_transit_world(0, 0, 30, 40);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let fleet = add_test_fleet(&mut world, origin, ship_key);
        let blockaded = HashSet::from([origin]);
        let clear = HashSet::new();
        world.systems[origin].control = ControlKind::Controlled(Faction::Alliance);

        assert!(fleet_move_confirms(&world, &clear, fleet, true));
        assert!(!fleet_move_confirms(&world, &clear, fleet, false));
        assert!(fleet_move_confirms(&world, &blockaded, fleet, false));

        world.systems[origin].control = ControlKind::Uprising(Faction::Alliance);
        assert!(fleet_move_confirms(&world, &blockaded, fleet, false));
        for other in [
            ControlKind::Controlled(Faction::Empire),
            ControlKind::Contested,
            ControlKind::Uncontrolled,
        ] {
            world.systems[origin].control = other;
            assert!(!fleet_move_confirms(&world, &blockaded, fleet, false));
        }
    }

    #[test]
    fn a_drag_never_moves_a_fleet_out_of_a_blockaded_system_on_either_side() {
        // FUN_00537180 → FUN_00552210 → FUN_005287f0: a sided fleet whose
        // +0x58 holds the blockade bit (FUN_0050c0b0) is refused 1/1; an
        // enemy-held destination is not refused (FUN_00528820).
        let (mut world, origin, destination) = make_transit_world(0, 0, 30, 40);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let fleet = add_test_fleet(&mut world, origin, ship_key);
        let movement = MovementState::new();
        let blockaded = HashSet::from([origin]);
        let clear = HashSet::new();
        world.systems[destination].control = ControlKind::Controlled(Faction::Empire);

        assert_eq!(
            validate_fleet_destination(&movement, &world, &clear, fleet, destination, true),
            Ok(()),
        );
        for control in [
            ControlKind::Controlled(Faction::Alliance),
            ControlKind::Controlled(Faction::Empire),
        ] {
            world.systems[origin].control = control;
            assert_eq!(
                validate_fleet_destination(&movement, &world, &blockaded, fleet, destination, true),
                Err(FleetDispatchError::Blockaded),
            );
        }
        assert_eq!(
            validate_fleet_destination(&movement, &world, &blockaded, fleet, destination, false),
            Err(FleetDispatchError::WrongFaction),
        );
    }

    #[test]
    fn a_blockade_refusal_says_why_in_the_message_log() {
        assert_eq!(
            FleetDispatchError::Blockaded.to_string(),
            "fleet is held in its system by a blockade",
        );
    }

    #[test]
    fn a_fleet_that_cannot_enter_hyperspace_may_open_a_move_but_is_refused_its_destination() {
        // The enabled slot reads no speed; the validator with a destination
        // refuses 1/0x18 (FUN_00555920, FUN_004fd900), which FUN_004fda10's
        // capital-ship walk gives a fighter-only fleet.
        let (mut world, origin, destination) = make_transit_world(0, 0, 30, 40);
        let ship_key = world.capital_ship_classes.insert(test_ship_class(80));
        let fleet = add_test_fleet(&mut world, origin, ship_key);
        world.fleets[fleet].capital_ships.clear();
        world.fleets[fleet].fighters.push(FighterEntry {
            class: world.fighter_classes.insert(FighterClass::default()),
            count: 1,
        });
        let movement = MovementState::new();

        assert!(fleet_move_enabled(&movement, &world, fleet, true));
        assert_eq!(
            validate_fleet_dispatch(&movement, &world, fleet, destination, true),
            Err(FleetDispatchError::NoHyperdrive),
        );
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

        // An empty fleet meets the speed refusal (FUN_004fd900: 1/0x18).
        world.fleets[fleet].capital_ships.clear();
        assert_eq!(
            validate_fleet_dispatch(&movement, &world, fleet, destination, true),
            Err(FleetDispatchError::NoHyperdrive),
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
        )
        .unwrap();
        assert_eq!(departure.origin, origin);
        assert_eq!(departure.destination, destination);
        // FUN_0055d8c0: isqrt(30^2 + 40^2) = 50; 50 / 5 * 80 / 100 = 8.
        assert_eq!(departure.transit_ticks, 8);
        assert!(!world.systems[origin].fleets.contains(&fleet));
        assert_eq!(movement.get(fleet).unwrap().destination, destination);
        assert_eq!(
            validate_fleet_dispatch(&movement, &world, fleet, destination, true),
            Err(FleetDispatchError::AlreadyInTransit),
        );
    }

    fn fleet_of(world: &mut GameWorld, location: SystemKey, classes: &[CapitalShipClass]) -> Fleet {
        let capital_ships = classes
            .iter()
            .map(|class| ShipInstance::new(world.capital_ship_classes.insert(class.clone()), 100, true))
            .collect();
        Fleet {
            location,
            capital_ships,
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        }
    }

    // FUN_0053e1d0: doubling search then bisection. It returns floor(sqrt(n))
    // except at powers of four from 4 on, where it returns one less.
    #[test]
    fn the_integer_square_root_is_the_floor_except_one_low_at_powers_of_four() {
        for n in [0_i64, 1, 2, 3, 5, 15, 17, 99, 100, 2_500, 192_400, 299_999] {
            assert_eq!(original_isqrt(n), n.isqrt(), "n = {n}");
        }
        for (n, root) in [(4, 1), (16, 3), (64, 7), (256, 15), (65_536, 255)] {
            assert_eq!(original_isqrt(n), root, "n = {n}");
        }
    }

    // FUN_0055d8c0: (isqrt(d^2) / GNPRTB 5120) * speed / 100, integer steps.
    #[test]
    fn transit_divides_the_integer_distance_by_five_then_scales_by_speed() {
        let (world, _, _) = make_transit_world(0, 0, 0, 0);
        assert_eq!(transit_ticks_between(&world, (0, 0), (30, 40), 100), 10);
        // isqrt(300^2 + 320^2) = 438; 438 / 5 = 87; 87 * 80 / 100 = 69.
        assert_eq!(transit_ticks_between(&world, (0, 0), (300, 320), 80), 69);
        assert_eq!(transit_ticks_between(&world, (300, 320), (0, 0), 80), 69);
    }

    // FUN_0055d8c0: a zero result becomes 1; a zero distance stays 0.
    #[test]
    fn a_short_hop_takes_one_day_and_the_same_position_takes_none() {
        let (world, _, _) = make_transit_world(0, 0, 0, 0);
        assert_eq!(transit_ticks_between(&world, (0, 0), (3, 0), 100), 1);
        assert_eq!(transit_ticks_between(&world, (7, 7), (7, 7), 100), 0);
    }

    // DAT_006bb6e8 (GNPRTB 5120) and DAT_006b9050 (GNPRTB 1) come from the table.
    #[test]
    fn loaded_gnprtb_values_replace_the_shipped_divisor_and_base() {
        let (mut world, _, _) = make_transit_world(0, 0, 0, 0);
        let entry = |id, value| crate::world::GnprtbEntry {
            parameter_id: id,
            development: value,
            alliance_sp_easy: value,
            alliance_sp_medium: value,
            alliance_sp_hard: value,
            empire_sp_easy: value,
            empire_sp_medium: value,
            empire_sp_hard: value,
            multiplayer: value,
        };
        world.gnprtb = crate::world::GnprtbParams::new(vec![entry(5120, 10), entry(1, 50)]);
        // 50 / 10 = 5; 5 * 100 / 50 = 10.
        assert_eq!(transit_ticks_between(&world, (0, 0), (30, 40), 100), 10);
    }

    // FUN_00500820: hyperdrive, else hyperdrive_if_damaged, else GNPRTB 1.
    #[test]
    fn a_ship_without_hyperdrive_uses_its_damaged_rating_then_the_default() {
        let (world, _, _) = make_transit_world(0, 0, 0, 0);
        let class = |hyperdrive, hyperdrive_if_damaged| CapitalShipClass {
            hyperdrive,
            hyperdrive_if_damaged,
            ..test_ship_class(0)
        };
        assert_eq!(capital_ship_speed(&world, &class(80, 120)), 80);
        assert_eq!(capital_ship_speed(&world, &class(0, 120)), 120);
        assert_eq!(capital_ship_speed(&world, &class(0, 0)), 100);
    }

    // FUN_004fd900: the fleet keeps the largest member speed; larger is slower.
    #[test]
    fn the_slowest_ship_sets_the_fleet_speed() {
        let (mut world, origin, dest) = make_transit_world(0, 0, 300, 320);
        let fleet = fleet_of(&mut world, origin, &[test_ship_class(50), test_ship_class(80)]);
        assert_eq!(fleet_speed(&fleet, &world), Some(80));
        assert_eq!(fleet_transit_ticks(&fleet, &world, origin, dest), Some(69));
    }

    // FUN_004fd900 skips members that are not completed; a destroyed ship
    // no longer counts.
    #[test]
    fn a_destroyed_ship_does_not_slow_the_fleet() {
        let (mut world, origin, _) = make_transit_world(0, 0, 300, 320);
        let mut fleet = fleet_of(&mut world, origin, &[test_ship_class(50), test_ship_class(80)]);
        fleet.capital_ships[1].alive = false;
        assert_eq!(fleet_speed(&fleet, &world), Some(50));
    }

    // FUN_004fda10 and FUN_004fd900 walk capital ships (0x14..0x1c) only.
    #[test]
    fn a_fleet_of_fighters_alone_cannot_enter_hyperspace() {
        let (mut world, origin, dest) = make_transit_world(0, 0, 300, 320);
        let mut fleet = fleet_of(&mut world, origin, &[]);
        fleet.fighters.push(FighterEntry { class: world.fighter_classes.insert(Default::default()), count: 1 });
        assert_eq!(fleet_transit_ticks(&fleet, &world, origin, dest), None);
        let key = world.fleets.insert(fleet);
        world.systems[origin].fleets.push(key);
        assert_eq!(
            validate_fleet_dispatch(&MovementState::new(), &world, key, dest, true),
            Err(FleetDispatchError::NoHyperdrive),
        );
    }

    // The Han Solo speed (GNPRTB 3083) is a character's own mission speed
    // (FUN_004ed370, FUN_00542990); fleets take no character into account.
    #[test]
    fn a_character_aboard_does_not_change_fleet_transit() {
        let (mut world, origin, dest) = make_transit_world(0, 0, 300, 320);
        let mut fleet = fleet_of(&mut world, origin, &[test_ship_class(80)]);
        let without = fleet_transit_ticks(&fleet, &world, origin, dest);
        fleet.characters.push(world.characters.insert(test_character("Han Solo", 50)));
        assert_eq!(fleet_transit_ticks(&fleet, &world, origin, dest), without);
    }
}
