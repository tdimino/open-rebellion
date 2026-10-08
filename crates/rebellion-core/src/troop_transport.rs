//! Fleet cargo for troop regiments.
//!
//! The original game associates a regiment in transit with a capital-ship
//! transport and limits carried regiments by the ships' `troop_capacity`.
//! Surface garrisons remain in `System::ground_units`; embarked regiments live
//! only in this state until they are landed or their transport is destroyed.
//!
//! A regiment also moves by order (`0x201`): within its system it changes
//! container at once, and to another system it travels on its own at the
//! default speed (`ghidra/notes/regiment-unload.md`).

use std::collections::{HashMap, HashSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::dat::Faction;
use crate::ids::{FleetKey, SystemKey, TroopKey};
use crate::movement::{default_speed, transit_ticks_between, MovementState};
use crate::world::GameWorld;

/// Persistent regiment cargo keyed by its carrying fleet.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TroopTransportState {
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    cargo: HashMap<FleetKey, Vec<TroopKey>>,
    /// Fleets holding regiments loaded by order where they orbit, in key
    /// order. port: the automatic landing skips them until they next arrive
    /// somewhere, so a regiment loaded at its own system stays aboard.
    held: Vec<FleetKey>,
    /// Regiments travelling on their own, in departure order.
    transit: Vec<RegimentTransit>,
    /// Each carried regiment's ship, by its tag (`ShipInstance::tag`; the
    /// original's regiments are children of a ship, `FUN_00504c40`). 0, or
    /// no entry, holds it on the fleet itself.
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    carriers: HashMap<TroopKey, u32>,
}

/// Where a regiment's Move is released: the drop window's `+0x70` object
/// (`ghidra/notes/regiment-unload.md`, "The order").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegimentTarget {
    System(SystemKey),
    Fleet(FleetKey),
}

/// The container a regiment enters: the leg `FUN_00552300` picks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegimentLeg {
    /// A planet's surface.
    Surface(SystemKey),
    /// A fleet's cargo.
    Fleet(FleetKey),
}

/// A regiment travelling to another system (`FUN_00556430`: the in-transit
/// bit, the destination at `+0x3c` and the arrival day at `+0x44`).
///
/// The original moves the regiment into its new container at once and marks
/// it en route. hyp: as for deliveries (`delivery.rs`), the port holds it
/// here and places it on arrival, so no walk of the destination sees it
/// early.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegimentTransit {
    pub troop: TroopKey,
    /// The system it left.
    pub origin: SystemKey,
    /// Where it goes.
    pub leg: RegimentLeg,
    /// The game-day it arrives.
    pub arrival_tick: u64,
}

/// What a regiment's Move did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegimentMove {
    /// The regiment entered its new container at once (same system).
    Placed(RegimentLeg),
    /// The regiment left for another system.
    Departed(RegimentTransit),
    /// The leg is the container the regiment is already in, which
    /// `FUN_004f6fd0` leaves unchanged.
    Unchanged,
}

/// Regiments whose transit ended on one advance.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RegimentArrivals {
    /// Regiments placed in their destination.
    pub arrived: Vec<RegimentTransit>,
    /// Regiments whose destination no longer exists, lost on arrival
    /// (`GameObjDestroyedOnArrivalNotif`, `FUN_004fc080`).
    pub lost: Vec<RegimentTransit>,
}

/// Where a regiment is now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegimentPlace {
    Surface(SystemKey),
    Cargo(FleetKey),
}

/// Why a requested troop transfer could not be applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TroopTransportError {
    MissingFleet,
    MissingSystem,
    NoTroops,
    MissingTroop,
    WrongFaction,
    TroopNotAtFleetSystem,
    AlreadyEmbarked,
    CapacityExceeded {
        capacity: u32,
        requested: u32,
    },
    FleetNotAtDestination,
    /// The destination belongs to another side (`FUN_0053d430` `1`/`0x28`;
    /// `FUN_00553410` `1`/`0x24`).
    OtherSideDestination,
    /// The destination planet is destroyed (`FUN_005073d0`, `1`/`0x22`).
    DestinationDestroyed,
    /// The destination planet is blockaded (`FUN_005073d0`, `0x90`/4).
    DestinationBlockaded,
    /// The destination fleet is in hyperspace (`FUN_00553410`, `1`/`0x21`).
    DestinationInTransit,
    /// The regiment is en route: travelling, or aboard a fleet in hyperspace
    /// (`FUN_00504350` → `FUN_004f9860`, `1`/`1`).
    RegimentEnRoute,
}

impl fmt::Display for TroopTransportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingFleet => formatter.write_str("fleet no longer exists"),
            Self::MissingSystem => formatter.write_str("fleet system no longer exists"),
            Self::NoTroops => formatter.write_str("no troop regiments were selected"),
            Self::MissingTroop => formatter.write_str("troop regiment no longer exists"),
            Self::WrongFaction => {
                formatter.write_str("troop regiment and fleet belong to different factions")
            }
            Self::TroopNotAtFleetSystem => {
                formatter.write_str("troop regiment is not stationed with the fleet")
            }
            Self::AlreadyEmbarked => formatter.write_str("troop regiment is already embarked"),
            Self::CapacityExceeded {
                capacity,
                requested,
            } => write!(
                formatter,
                "troop capacity exceeded: capacity {capacity}, requested {requested}"
            ),
            Self::FleetNotAtDestination => {
                formatter.write_str("fleet is not orbiting the landing system")
            }
            Self::OtherSideDestination => {
                formatter.write_str("the destination belongs to another side")
            }
            Self::DestinationDestroyed => formatter.write_str("the destination is destroyed"),
            Self::DestinationBlockaded => formatter.write_str("the destination is blockaded"),
            Self::DestinationInTransit => {
                formatter.write_str("the destination fleet is in hyperspace")
            }
            Self::RegimentEnRoute => formatter.write_str("the regiment is en route"),
        }
    }
}

impl TroopTransportState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Troops currently carried by `fleet`, in stable key order.
    pub fn cargo(&self, fleet: FleetKey) -> &[TroopKey] {
        self.cargo.get(&fleet).map_or(&[], Vec::as_slice)
    }

    #[must_use]
    pub fn carried_count(&self, fleet: FleetKey) -> usize {
        self.cargo(fleet).len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cargo.is_empty()
    }

    #[must_use]
    pub fn is_embarked(&self, troop: TroopKey) -> bool {
        self.cargo.values().any(|cargo| cargo.contains(&troop))
    }

    /// Regiments the automatic landing may put down from `fleet`: none while
    /// the fleet holds cargo loaded by order at the system it orbits.
    #[must_use]
    pub fn landing_count(&self, fleet: FleetKey) -> usize {
        if self.is_held(fleet) {
            0
        } else {
            self.carried_count(fleet)
        }
    }

    #[must_use]
    pub fn is_held(&self, fleet: FleetKey) -> bool {
        self.held.binary_search(&fleet).is_ok()
    }

    /// Put surface regiments of `fleet`'s system aboard it and hold them
    /// there, as a regiment's Move onto the fleet does within one system
    /// ([`Self::move_regiment`]).
    ///
    /// The refusals are [`Self::embark`]'s: a regiment of the other side, one
    /// in another system, or more than the fleet's room (`FUN_00500b40`:
    /// capacity minus regiments aboard). port: the room is the whole fleet's,
    /// where the original's leg builder fills one ship at a time.
    ///
    /// # Errors
    /// Returns [`Self::embark`]'s error, and then nothing changes.
    pub fn load(
        &mut self,
        world: &mut GameWorld,
        fleet: FleetKey,
        troops: &[TroopKey],
    ) -> Result<(), TroopTransportError> {
        self.embark(world, fleet, troops)?;
        if let Err(index) = self.held.binary_search(&fleet) {
            self.held.insert(index, fleet);
        }
        Ok(())
    }

    /// Whether a regiment's pop-up menu enables Move: a regiment of the
    /// player's side on a planet's surface. The move command's object check
    /// (`FUN_00578c00` → slot `+0x6c`, `FUN_00504350`) is `FUN_004f9860`;
    /// hyp: its side and en-route rules, the rest untraced for a regiment.
    /// port: an embarked regiment's pop-up Move stays disabled; it moves
    /// only by a drag out of the Fleet window's Troops tab.
    #[must_use]
    pub fn regiment_move_enabled(
        &self,
        world: &GameWorld,
        troop: TroopKey,
        expected_is_alliance: bool,
    ) -> bool {
        world
            .troops
            .get(troop)
            .is_some_and(|value| value.is_alliance == expected_is_alliance)
            && !self.is_embarked(troop)
            && regiment_system(world, troop).is_some()
    }

    /// Regiments travelling on their own, in departure order.
    #[must_use]
    pub fn transits(&self) -> &[RegimentTransit] {
        &self.transit
    }

    /// Regiments travelling to board `fleet`. Each already holds its room,
    /// as the original's regiment sits in the ship's container while en
    /// route.
    #[must_use]
    pub fn incoming_count(&self, fleet: FleetKey) -> usize {
        self.transit
            .iter()
            .filter(|transit| transit.leg == RegimentLeg::Fleet(fleet))
            .count()
    }

    /// Carry out a regiment's Move (`0x201`) released on `target`.
    ///
    /// The checks run in the original's order (`ghidra/notes/regiment-unload.md`):
    /// the regiment must not be en route (`FUN_004f9860`); the group check
    /// refuses another side's destination unless it is an existing,
    /// unpopulated system (`FUN_0053d430`); then the leg builder picks the
    /// container (`FUN_00552300`). A planet takes the regiment when it is the
    /// regiment's side's, not destroyed and not blockaded (`FUN_005073d0`).
    /// Otherwise the first fleet of the side there with room takes it
    /// (`FUN_005097d0`, then every fleet). A fleet takes it when it is of the
    /// side, not in hyperspace and has room (`FUN_00500ac0`, `FUN_00500b40`).
    /// The command's own side check (`FUN_00555920` `0x28`) always passes on
    /// such a leg.
    ///
    /// Within one system the regiment changes container at once
    /// (`FUN_00556390`). Across systems it travels at the default speed, the
    /// GNPRTB 1 rate of `FUN_004f63f0`. A regiment boarding a fleet is held
    /// aboard, as by [`Self::load`].
    ///
    /// port: the leg builder's first pick, a fleet with `+0x58` bit 1, is
    /// skipped (who sets that bit is untraced), and so is the side-3
    /// planet with `+0x88` bit 1 that `FUN_005073d0` lets a regiment land on.
    ///
    /// # Errors
    /// Returns the refusal, and then nothing changes.
    pub fn move_regiment(
        &mut self,
        world: &mut GameWorld,
        movement: &MovementState,
        blockaded: &HashSet<SystemKey>,
        troop: TroopKey,
        target: RegimentTarget,
        tick: u64,
    ) -> Result<RegimentMove, TroopTransportError> {
        let is_alliance = world
            .troops
            .get(troop)
            .ok_or(TroopTransportError::MissingTroop)?
            .is_alliance;
        let place = self.regiment_place(world, troop)?;
        let origin = match place {
            RegimentPlace::Surface(system) => system,
            RegimentPlace::Cargo(fleet) => {
                if movement.is_in_transit(fleet) {
                    return Err(TroopTransportError::RegimentEnRoute);
                }
                world
                    .fleets
                    .get(fleet)
                    .ok_or(TroopTransportError::MissingFleet)?
                    .location
            }
        };
        Self::check_group(world, is_alliance, target)?;
        let leg = self.choose_leg(world, movement, blockaded, is_alliance, target)?;
        if let (RegimentPlace::Cargo(current), RegimentLeg::Fleet(fleet)) = (place, leg) {
            if current == fleet {
                return Ok(RegimentMove::Unchanged);
            }
        }
        self.execute(world, troop, place, origin, leg, tick)
    }

    /// Advance travelling regiments to `now`: each whose day has come enters
    /// its destination, or is lost when the destination is gone or the
    /// planet destroyed (`FUN_004fc080`).
    pub fn advance_transit(&mut self, world: &mut GameWorld, now: u64) -> RegimentArrivals {
        let mut result = RegimentArrivals::default();
        let mut remaining = Vec::with_capacity(self.transit.len());
        for transit in std::mem::take(&mut self.transit) {
            if transit.arrival_tick > now {
                remaining.push(transit);
                continue;
            }
            let entered = match transit.leg {
                RegimentLeg::Surface(system) => match world.systems.get_mut(system) {
                    Some(value) if !value.is_destroyed => {
                        insert_sorted(&mut value.ground_units, transit.troop);
                        true
                    }
                    _ => false,
                },
                RegimentLeg::Fleet(fleet) if world.fleets.contains_key(fleet) => {
                    self.board(world, fleet, transit.troop);
                    true
                }
                RegimentLeg::Fleet(_) => false,
            };
            if entered {
                result.arrived.push(transit);
            } else {
                world.troops.remove(transit.troop);
                result.lost.push(transit);
            }
        }
        self.transit = remaining;
        result
    }

    fn regiment_place(
        &self,
        world: &GameWorld,
        troop: TroopKey,
    ) -> Result<RegimentPlace, TroopTransportError> {
        if self.transit.iter().any(|transit| transit.troop == troop) {
            return Err(TroopTransportError::RegimentEnRoute);
        }
        if let Some((&fleet, _)) = self.cargo.iter().find(|(_, cargo)| cargo.contains(&troop)) {
            return Ok(RegimentPlace::Cargo(fleet));
        }
        regiment_system(world, troop)
            .map(RegimentPlace::Surface)
            .ok_or(TroopTransportError::MissingTroop)
    }

    /// `FUN_0053d430`'s side test for an all-regiment group.
    fn check_group(
        world: &GameWorld,
        is_alliance: bool,
        target: RegimentTarget,
    ) -> Result<(), TroopTransportError> {
        match target {
            RegimentTarget::System(system) => {
                let value = world
                    .systems
                    .get(system)
                    .ok_or(TroopTransportError::MissingSystem)?;
                let unpopulated = !value.is_destroyed && !value.is_populated;
                if value.control.is_controlled_by(side(is_alliance)) || unpopulated {
                    Ok(())
                } else {
                    Err(TroopTransportError::OtherSideDestination)
                }
            }
            RegimentTarget::Fleet(fleet) => {
                let value = world
                    .fleets
                    .get(fleet)
                    .ok_or(TroopTransportError::MissingFleet)?;
                if value.is_alliance == is_alliance {
                    Ok(())
                } else {
                    Err(TroopTransportError::WrongFaction)
                }
            }
        }
    }

    /// `FUN_00552300`: the container the regiment enters.
    fn choose_leg(
        &self,
        world: &GameWorld,
        movement: &MovementState,
        blockaded: &HashSet<SystemKey>,
        is_alliance: bool,
        target: RegimentTarget,
    ) -> Result<RegimentLeg, TroopTransportError> {
        match target {
            RegimentTarget::Fleet(fleet) => {
                self.accept_fleet(world, movement, is_alliance, fleet)?;
                Ok(RegimentLeg::Fleet(fleet))
            }
            RegimentTarget::System(system) => {
                let refusal = match Self::accept_planet(world, blockaded, is_alliance, system) {
                    Ok(()) => return Ok(RegimentLeg::Surface(system)),
                    Err(error) => error,
                };
                world.systems[system]
                    .fleets
                    .iter()
                    .copied()
                    .find(|&fleet| {
                        self.accept_fleet(world, movement, is_alliance, fleet)
                            .is_ok()
                    })
                    .map(RegimentLeg::Fleet)
                    .ok_or(refusal)
            }
        }
    }

    /// `FUN_005073d0` for a `0x204` regiment: room is unlimited
    /// (`FUN_00507750`).
    fn accept_planet(
        world: &GameWorld,
        blockaded: &HashSet<SystemKey>,
        is_alliance: bool,
        system: SystemKey,
    ) -> Result<(), TroopTransportError> {
        let value = world
            .systems
            .get(system)
            .ok_or(TroopTransportError::MissingSystem)?;
        if value.is_destroyed {
            Err(TroopTransportError::DestinationDestroyed)
        } else if !value.control.is_controlled_by(side(is_alliance)) {
            Err(TroopTransportError::OtherSideDestination)
        } else if blockaded.contains(&system) {
            Err(TroopTransportError::DestinationBlockaded)
        } else {
            Ok(())
        }
    }

    /// `FUN_00500ac0` and `FUN_00500b40`: a fleet of the side, not in
    /// hyperspace, with room for one more regiment besides those aboard and
    /// those on their way. The room counts every regiment aboard, so a
    /// regiment's own full fleet has none.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Regiment counts are far below u32::MAX."
    )]
    fn accept_fleet(
        &self,
        world: &GameWorld,
        movement: &MovementState,
        is_alliance: bool,
        fleet: FleetKey,
    ) -> Result<(), TroopTransportError> {
        let value = world
            .fleets
            .get(fleet)
            .ok_or(TroopTransportError::MissingFleet)?;
        if value.is_alliance != is_alliance {
            return Err(TroopTransportError::WrongFaction);
        }
        if movement.is_in_transit(fleet) {
            return Err(TroopTransportError::DestinationInTransit);
        }
        let capacity =
            Self::fleet_capacity(world, fleet).ok_or(TroopTransportError::MissingFleet)?;
        let requested = (self.carried_count(fleet) + self.incoming_count(fleet) + 1) as u32;
        if requested > capacity {
            return Err(TroopTransportError::CapacityExceeded {
                capacity,
                requested,
            });
        }
        Ok(())
    }

    /// `FUN_00556390`: take the regiment out of its container, then place it
    /// at once within its system or start its transit.
    fn execute(
        &mut self,
        world: &mut GameWorld,
        troop: TroopKey,
        place: RegimentPlace,
        origin: SystemKey,
        leg: RegimentLeg,
        tick: u64,
    ) -> Result<RegimentMove, TroopTransportError> {
        let destination = match leg {
            RegimentLeg::Surface(system) => system,
            RegimentLeg::Fleet(fleet) => {
                world
                    .fleets
                    .get(fleet)
                    .ok_or(TroopTransportError::MissingFleet)?
                    .location
            }
        };
        match place {
            RegimentPlace::Surface(system) => {
                if let Some(value) = world.systems.get_mut(system) {
                    value.ground_units.retain(|&key| key != troop);
                }
            }
            RegimentPlace::Cargo(fleet) => {
                if let Some(cargo) = self.cargo.get_mut(&fleet) {
                    cargo.retain(|&key| key != troop);
                }
                self.forget_if_empty(fleet);
            }
        }
        if destination == origin {
            match leg {
                RegimentLeg::Surface(system) => {
                    insert_sorted(&mut world.systems[system].ground_units, troop);
                }
                RegimentLeg::Fleet(fleet) => self.board(world, fleet, troop),
            }
            return Ok(RegimentMove::Placed(leg));
        }
        let position = |key: SystemKey| world.systems.get(key).map_or((0, 0), |s| (s.x, s.y));
        let ticks = transit_ticks_between(
            world,
            position(origin),
            position(destination),
            default_speed(world),
        );
        let transit = RegimentTransit {
            troop,
            origin,
            leg,
            arrival_tick: tick + u64::from(ticks),
        };
        self.transit.push(transit.clone());
        Ok(RegimentMove::Departed(transit))
    }

    /// Regiments aboard `fleet`'s ship tagged `tag`.
    #[must_use]
    pub fn regiments_aboard(&self, fleet: FleetKey, tag: u32) -> usize {
        self.cargo(fleet)
            .iter()
            .filter(|troop| self.carrier(**troop) == tag)
            .count()
    }

    /// The tag of the ship `troop` rides aboard; 0 when it is held on its
    /// fleet or not carried.
    #[must_use]
    pub fn carrier(&self, troop: TroopKey) -> u32 {
        self.carriers.get(&troop).copied().unwrap_or(0)
    }

    /// Put `troop` in `fleet`'s cargo aboard the ship with the most room
    /// (`crate::carriage::ship_for_regiment`), or on the fleet when none
    /// has room.
    fn stow(&mut self, world: &mut GameWorld, fleet: FleetKey, troop: TroopKey) {
        let carrier = crate::carriage::ship_for_regiment(world, self, fleet)
                .and_then(|index| crate::carriage::tag_ship(world, fleet, index))
                .unwrap_or(0);
        insert_sorted(self.cargo.entry(fleet).or_default(), troop);
        if carrier == 0 {
            self.carriers.remove(&troop);
        } else {
            self.carriers.insert(troop, carrier);
        }
    }

    /// Put `troop` aboard `fleet` and hold the fleet's cargo there.
    fn board(&mut self, world: &mut GameWorld, fleet: FleetKey, troop: TroopKey) {
        self.stow(world, fleet, troop);
        if let Err(index) = self.held.binary_search(&fleet) {
            self.held.insert(index, fleet);
        }
    }

    /// End `fleet`'s hold. A fleet arriving anywhere releases its cargo to
    /// the automatic landing.
    pub fn release(&mut self, fleet: FleetKey) {
        self.held.retain(|&key| key != fleet);
    }

    fn forget_if_empty(&mut self, fleet: FleetKey) {
        if self.cargo.get(&fleet).is_some_and(Vec::is_empty) {
            self.cargo.remove(&fleet);
        }
        if !self.cargo.contains_key(&fleet) {
            self.release(fleet);
        }
    }

    /// Fleet identities currently carrying at least one regiment.
    #[must_use]
    pub fn fleet_keys(&self) -> Vec<FleetKey> {
        let mut fleets: Vec<_> = self.cargo.keys().copied().collect();
        fleets.sort_unstable();
        fleets
    }

    /// Sum the troop capacity of every living capital ship in `fleet`.
    pub fn fleet_capacity(world: &GameWorld, fleet: FleetKey) -> Option<u32> {
        let fleet = world.fleets.get(fleet)?;
        Some(
            fleet
                .capital_ships
                .iter()
                .filter(|ship| ship.alive)
                .filter_map(|ship| world.capital_ship_classes.get(ship.class))
                .map(|class| class.troop_capacity)
                .fold(0_u32, u32::saturating_add),
        )
    }

    /// Move surface regiments into a fleet after validating the whole request.
    ///
    /// Validation is atomic: no regiment leaves the surface when any requested
    /// key is invalid, duplicated, already carried, off-system, wrong-faction,
    /// or above the living ships' capacity.
    ///
    /// # Errors
    /// Returns a transport error for an empty selection, missing entities, duplicate
    /// or already embarked troops, faction/location mismatches, or insufficient capacity.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
    )]
    pub fn embark(
        &mut self,
        world: &mut GameWorld,
        fleet: FleetKey,
        troops: &[TroopKey],
    ) -> Result<(), TroopTransportError> {
        if troops.is_empty() {
            return Err(TroopTransportError::NoTroops);
        }

        let fleet_value = world
            .fleets
            .get(fleet)
            .ok_or(TroopTransportError::MissingFleet)?;
        let system_key = fleet_value.location;
        let fleet_is_alliance = fleet_value.is_alliance;
        let system = world
            .systems
            .get(system_key)
            .ok_or(TroopTransportError::MissingSystem)?;

        let mut requested_keys = HashSet::new();
        for &troop in troops {
            if !requested_keys.insert(troop) || self.is_embarked(troop) {
                return Err(TroopTransportError::AlreadyEmbarked);
            }
            let value = world
                .troops
                .get(troop)
                .ok_or(TroopTransportError::MissingTroop)?;
            if value.is_alliance != fleet_is_alliance {
                return Err(TroopTransportError::WrongFaction);
            }
            if !system.ground_units.contains(&troop) {
                return Err(TroopTransportError::TroopNotAtFleetSystem);
            }
        }

        let capacity =
            Self::fleet_capacity(world, fleet).ok_or(TroopTransportError::MissingFleet)?;
        let requested = self.carried_count(fleet).saturating_add(troops.len()) as u32;
        if requested > capacity {
            return Err(TroopTransportError::CapacityExceeded {
                capacity,
                requested,
            });
        }

        if let Some(system) = world.systems.get_mut(system_key) {
            system
                .ground_units
                .retain(|troop| !requested_keys.contains(troop));
        }
        for &troop in troops {
            self.stow(world, fleet, troop);
        }
        Ok(())
    }

    /// Land every carried regiment at the fleet's current system.
    ///
    /// # Errors
    /// Returns a transport error if the fleet or destination is missing,
    /// or the fleet is not at the destination.
    pub fn disembark_all(
        &mut self,
        world: &mut GameWorld,
        fleet: FleetKey,
        destination: SystemKey,
    ) -> Result<Vec<TroopKey>, TroopTransportError> {
        let value = world
            .fleets
            .get(fleet)
            .ok_or(TroopTransportError::MissingFleet)?;
        if value.location != destination {
            return Err(TroopTransportError::FleetNotAtDestination);
        }
        let system = world
            .systems
            .get_mut(destination)
            .ok_or(TroopTransportError::MissingSystem)?;

        let mut landed = self.cargo.remove(&fleet).unwrap_or_default();
        self.release(fleet);
        landed.retain(|troop| world.troops.contains_key(*troop));
        landed.sort_unstable();
        landed.dedup();
        system.ground_units.extend(landed.iter().copied());
        system.ground_units.sort_unstable();
        system.ground_units.dedup();
        Ok(landed)
    }

    /// Return a selected set of cargo to the fleet's current system.
    ///
    /// This is primarily the atomic rollback path for a player departure that
    /// becomes invalid after embarkation. Cargo not named in `troops` remains
    /// aboard.
    ///
    /// # Errors
    /// Returns a transport error if the fleet or destination is missing,
    /// or the fleet is not at the destination.
    pub fn disembark_selected(
        &mut self,
        world: &mut GameWorld,
        fleet: FleetKey,
        destination: SystemKey,
        troops: &[TroopKey],
    ) -> Result<Vec<TroopKey>, TroopTransportError> {
        let value = world
            .fleets
            .get(fleet)
            .ok_or(TroopTransportError::MissingFleet)?;
        if value.location != destination {
            return Err(TroopTransportError::FleetNotAtDestination);
        }
        let system = world
            .systems
            .get_mut(destination)
            .ok_or(TroopTransportError::MissingSystem)?;
        let requested: HashSet<_> = troops.iter().copied().collect();
        let mut landed = Vec::new();
        if let Some(cargo) = self.cargo.get_mut(&fleet) {
            cargo.retain(|troop| {
                if requested.contains(troop) {
                    landed.push(*troop);
                    false
                } else {
                    true
                }
            });
        }
        self.forget_if_empty(fleet);
        landed.retain(|troop| world.troops.contains_key(*troop));
        landed.sort_unstable();
        landed.dedup();
        system.ground_units.extend(landed.iter().copied());
        system.ground_units.sort_unstable();
        system.ground_units.dedup();
        Ok(landed)
    }

    /// Preserve cargo identity when compatible fleet records consolidate.
    pub fn transfer_fleet(&mut self, from: FleetKey, to: FleetKey) {
        if from == to {
            return;
        }
        for transit in &mut self.transit {
            if transit.leg == RegimentLeg::Fleet(from) {
                transit.leg = RegimentLeg::Fleet(to);
            }
        }
        self.release(from);
        let Some(mut moved) = self.cargo.remove(&from) else {
            return;
        };
        let cargo = self.cargo.entry(to).or_default();
        cargo.append(&mut moved);
        cargo.sort_unstable();
        cargo.dedup();
    }

    /// Move `troops`, regiments aboard `from`, aboard `to`. A hold at the
    /// loading system goes with them, as both fleets orbit it.
    pub fn transfer_some(&mut self, from: FleetKey, to: FleetKey, troops: &[TroopKey]) {
        let Some(cargo) = self.cargo.get_mut(&from) else {
            return;
        };
        let moved: Vec<TroopKey> = cargo
            .iter()
            .copied()
            .filter(|troop| troops.contains(troop))
            .collect();
        if moved.is_empty() {
            return;
        }
        cargo.retain(|troop| !moved.contains(troop));
        let held = self.is_held(from);
        for troop in moved {
            insert_sorted(self.cargo.entry(to).or_default(), troop);
        }
        if held {
            if let Err(index) = self.held.binary_search(&to) {
                self.held.insert(index, to);
            }
        }
        self.forget_if_empty(from);
    }

    /// Destroy every regiment aboard a fleet that has been destroyed.
    ///
    /// A regiment on its way to board the fleet dies with it: the original
    /// has already put it in the ship's container (`FUN_00556390`).
    pub fn destroy_fleet_cargo(&mut self, world: &mut GameWorld, fleet: FleetKey) -> Vec<TroopKey> {
        let mut cargo = self.cargo.remove(&fleet).unwrap_or_default();
        self.release(fleet);
        self.transit.retain(|transit| {
            let bound = transit.leg == RegimentLeg::Fleet(fleet);
            if bound {
                cargo.push(transit.troop);
            }
            !bound
        });
        for troop in &cargo {
            world.troops.remove(*troop);
        }
        cargo
    }

    /// Destroy one embarked regiment, removing it from its fleet's cargo and
    /// from the arena. Returns `false` if it was not aboard any fleet.
    pub fn destroy_embarked(&mut self, world: &mut GameWorld, troop: TroopKey) -> bool {
        let mut found = false;
        for cargo in self.cargo.values_mut() {
            let before = cargo.len();
            cargo.retain(|&key| key != troop);
            found |= cargo.len() != before;
        }
        if found {
            world.troops.remove(troop);
        }
        found
    }

    /// Remove regiments that no longer have living transport capacity.
    ///
    /// A fleet may survive because an escort remains after every transport is
    /// destroyed. Cargo is kept in stable key order up to the surviving
    /// capacity; the deterministic excess is lost with the transport hulls.
    pub fn destroy_untransportable_cargo(
        &mut self,
        world: &mut GameWorld,
    ) -> Vec<(FleetKey, Vec<TroopKey>)> {
        let mut destroyed = Vec::new();
        for fleet in self.fleet_keys() {
            // hyp: a regiment aboard a ship that was lost or left the fleet
            // is destroyed with it (`crate::carriage::drop_lost_squadrons`).
            let mut lost: Vec<TroopKey> = match world.fleets.get(fleet) {
                Some(value) => self
                    .cargo(fleet)
                    .iter()
                    .copied()
                    .filter(|&troop| {
                        let tag = self.carrier(troop);
                        tag != 0 && !crate::carriage::carried_by_living_ship(value, tag)
                    })
                    .collect(),
                None => Vec::new(),
            };
            if let Some(cargo) = self.cargo.get_mut(&fleet) {
                cargo.retain(|troop| !lost.contains(troop));
            }
            let capacity = Self::fleet_capacity(world, fleet).map(|value| value as usize);
            let excess = match capacity {
                None => self.cargo.remove(&fleet).unwrap_or_default(),
                Some(capacity) => {
                    let cargo = self.cargo.entry(fleet).or_default();
                    if cargo.len() <= capacity {
                        Vec::new()
                    } else {
                        cargo.split_off(capacity)
                    }
                }
            };
            lost.extend(excess);
            if lost.is_empty() {
                continue;
            }
            for troop in &lost {
                world.troops.remove(*troop);
            }
            self.forget_if_empty(fleet);
            destroyed.push((fleet, lost));
        }
        let carried: HashSet<TroopKey> = self.cargo.values().flatten().copied().collect();
        self.carriers.retain(|troop, _| carried.contains(troop));
        destroyed
    }
}

fn side(is_alliance: bool) -> Faction {
    if is_alliance {
        Faction::Alliance
    } else {
        Faction::Empire
    }
}

fn insert_sorted(keys: &mut Vec<TroopKey>, troop: TroopKey) {
    keys.push(troop);
    keys.sort_unstable();
    keys.dedup();
}

/// The system whose surface holds `troop`.
#[must_use]
pub fn regiment_system(world: &GameWorld, troop: TroopKey) -> Option<SystemKey> {
    world
        .systems
        .iter()
        .find(|(_, system)| system.ground_units.contains(&troop))
        .map(|(key, _)| key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dat::Faction;
    use crate::dat::{ExplorationStatus, SectorGroup};
    use crate::ids::DatId;
    use crate::world::{
        CapitalShipClass, ControlKind, Fleet, Sector, ShipInstance, System, TroopUnit,
    };

    fn fixture(capacity: u32) -> (GameWorld, SystemKey, SystemKey, FleetKey, Vec<TroopKey>) {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(1),
            name: "Test".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let make_system = |name: &str| System {
            dat_id: DatId::new(2),
            name: name.into(),
            sector,
            x: 0,
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
        };
        let origin = world.systems.insert(make_system("Origin"));
        let destination = world.systems.insert(make_system("Destination"));
        let ship_class = world.capital_ship_classes.insert(CapitalShipClass {
            name: "Transport".into(),
            is_alliance: true,
            troop_capacity: capacity,
            hull: 100,
            ..CapitalShipClass::default()
        });
        let fleet = world.fleets.insert(Fleet {
            location: origin,
            capital_ships: vec![ShipInstance::new(ship_class, 100, true)],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        world.systems[origin].fleets.push(fleet);

        let troops: Vec<_> = (0..3)
            .map(|index| {
                world.troops.insert(TroopUnit {
                    class_dat_id: DatId::new(0x1000_0001 + index),
                    is_alliance: true,
                    regiment_strength: 100,
                })
            })
            .collect();
        world.systems[origin]
            .ground_units
            .extend(troops.iter().copied());
        (world, origin, destination, fleet, troops)
    }

    #[test]
    fn embark_honors_capacity_and_is_atomic() {
        let (mut world, origin, _, fleet, troops) = fixture(2);
        let mut state = TroopTransportState::new();

        assert_eq!(
            state.embark(&mut world, fleet, &troops),
            Err(TroopTransportError::CapacityExceeded {
                capacity: 2,
                requested: 3,
            })
        );
        assert_eq!(world.systems[origin].ground_units.len(), 3);
        assert!(state.cargo(fleet).is_empty());

        state.embark(&mut world, fleet, &troops[..2]).unwrap();
        assert_eq!(state.cargo(fleet), &troops[..2]);
        assert_eq!(world.systems[origin].ground_units, vec![troops[2]]);
    }

    #[test]
    fn wrong_faction_rejection_leaves_surface_unchanged() {
        let (mut world, origin, _, fleet, troops) = fixture(2);
        world.troops[troops[1]].is_alliance = false;
        let mut state = TroopTransportState::new();

        assert_eq!(
            state.embark(&mut world, fleet, &troops[..2]),
            Err(TroopTransportError::WrongFaction)
        );
        assert_eq!(world.systems[origin].ground_units.len(), 3);
        assert!(state.cargo(fleet).is_empty());
    }

    #[test]
    fn disembark_moves_only_live_cargo_to_destination() {
        let (mut world, _, destination, fleet, troops) = fixture(3);
        let mut state = TroopTransportState::new();
        state.embark(&mut world, fleet, &troops).unwrap();
        world.fleets[fleet].location = destination;
        world.troops.remove(troops[1]);

        let landed = state.disembark_all(&mut world, fleet, destination).unwrap();
        assert_eq!(landed, vec![troops[0], troops[2]]);
        assert_eq!(world.systems[destination].ground_units, landed);
        assert!(state.cargo(fleet).is_empty());
    }

    #[test]
    fn selected_disembark_rolls_back_only_the_requested_regiments() {
        let (mut world, origin, _, fleet, troops) = fixture(3);
        let mut state = TroopTransportState::new();
        state.embark(&mut world, fleet, &troops).unwrap();

        let landed = state
            .disembark_selected(&mut world, fleet, origin, &troops[..2])
            .unwrap();

        assert_eq!(landed, troops[..2]);
        assert_eq!(state.cargo(fleet), &troops[2..]);
        assert_eq!(world.systems[origin].ground_units, troops[..2]);
    }

    #[test]
    fn cargo_transfers_when_fleet_identity_is_consolidated() {
        let (mut world, origin, _, first, troops) = fixture(3);
        let second = world.fleets.insert(Fleet {
            location: origin,
            capital_ships: vec![],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        let mut state = TroopTransportState::new();
        state.embark(&mut world, first, &troops[..2]).unwrap();

        state.transfer_fleet(first, second);
        assert!(state.cargo(first).is_empty());
        assert_eq!(state.cargo(second), &troops[..2]);
    }

    #[test]
    fn destroyed_transport_removes_its_embarked_regiments() {
        let (mut world, _, _, fleet, troops) = fixture(2);
        let mut state = TroopTransportState::new();
        state.embark(&mut world, fleet, &troops[..2]).unwrap();

        assert_eq!(
            state.destroy_fleet_cargo(&mut world, fleet),
            troops[..2].to_vec()
        );
        assert!(!world.troops.contains_key(troops[0]));
        assert!(!world.troops.contains_key(troops[1]));
        assert!(world.troops.contains_key(troops[2]));
    }

    #[test]
    fn orphaned_cargo_is_destroyed_after_fleet_removal() {
        let (mut world, _, _, fleet, troops) = fixture(2);
        let mut state = TroopTransportState::new();
        state.embark(&mut world, fleet, &troops[..2]).unwrap();
        world.fleets.remove(fleet);

        let destroyed = state.destroy_untransportable_cargo(&mut world);

        assert_eq!(destroyed, vec![(fleet, troops[..2].to_vec())]);
        assert!(state.is_empty());
        assert!(!world.troops.contains_key(troops[0]));
        assert!(!world.troops.contains_key(troops[1]));
        assert!(world.troops.contains_key(troops[2]));
    }

    // FUN_00552300/FUN_00552b10: each regiment boards the ship with the
    // most room, the earlier on a tie. hyp: those aboard a lost ship are
    // lost with it.
    #[test]
    fn regiments_board_the_roomiest_ship_and_are_lost_with_it() {
        let (mut world, _, _, fleet, troops) = fixture(3);
        let transport_class = world.fleets[fleet].capital_ships[0].class;
        let second = world.capital_ship_classes.insert(CapitalShipClass {
            name: "Second Transport".into(),
            is_alliance: true,
            troop_capacity: 3,
            hull: 100,
            ..CapitalShipClass::default()
        });
        world.fleets[fleet]
            .capital_ships
            .push(ShipInstance::new(second, 100, true));
        let mut state = TroopTransportState::new();
        state.embark(&mut world, fleet, &troops).unwrap();
        let tags: Vec<u32> = world.fleets[fleet]
            .capital_ships
            .iter()
            .map(|ship| ship.tag)
            .collect();
        assert_eq!(
            troops.iter().map(|&troop| state.carrier(troop)).collect::<Vec<_>>(),
            [tags[0], tags[1], tags[0]]
        );
        world.fleets[fleet]
            .capital_ships
            .iter_mut()
            .find(|ship| ship.class == transport_class)
            .unwrap()
            .alive = false;

        let destroyed = state.destroy_untransportable_cargo(&mut world);

        assert_eq!(state.cargo(fleet), &troops[1..2]);
        assert_eq!(destroyed, vec![(fleet, vec![troops[0], troops[2]])]);
        assert!(world.troops.contains_key(troops[1]));
        assert!(!world.troops.contains_key(troops[0]));
        assert!(!world.troops.contains_key(troops[2]));
    }

    #[test]
    fn cargo_json_order_is_deterministic() {
        let (mut world, origin, _, first, troops) = fixture(3);
        let ship_class = world.fleets[first].capital_ships[0].class;
        let second = world.fleets.insert(Fleet {
            location: origin,
            capital_ships: vec![ShipInstance::new(ship_class, 100, true)],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        let mut state = TroopTransportState::new();
        state.embark(&mut world, second, &troops[..1]).unwrap();
        state.embark(&mut world, first, &troops[1..]).unwrap();

        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(json, serde_json::to_string(&state).unwrap());
        let decoded: TroopTransportState = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.cargo(first), state.cargo(first));
        assert_eq!(decoded.cargo(second), state.cargo(second));
    }
    // port: a regiment loaded by order stays aboard where it loaded
    // (`TroopTransportState::load`, `ghidra/notes/fleet-window.md`).
    #[test]
    fn a_loaded_regiment_is_held_aboard_at_the_system_it_loaded_at() {
        let (mut world, origin, _, fleet, troops) = fixture(2);
        let mut state = TroopTransportState::new();

        state.load(&mut world, fleet, &troops[..1]).unwrap();

        assert_eq!(state.cargo(fleet), &troops[..1]);
        assert_eq!(world.systems[origin].ground_units, troops[1..]);
        assert_eq!(state.landing_count(fleet), 0);
        assert_eq!(state.carried_count(fleet), 1);
    }

    #[test]
    fn embarking_for_a_departure_does_not_hold_the_cargo() {
        let (mut world, _, _, fleet, troops) = fixture(2);
        let mut state = TroopTransportState::new();

        state.embark(&mut world, fleet, &troops[..2]).unwrap();

        assert_eq!(state.landing_count(fleet), 2);
    }

    // FUN_00500b40: the room is the capacity minus the regiments aboard.
    #[test]
    fn a_full_fleet_refuses_a_regiment_and_nothing_moves() {
        let (mut world, origin, _, fleet, troops) = fixture(1);
        let mut state = TroopTransportState::new();
        state.load(&mut world, fleet, &troops[..1]).unwrap();

        assert_eq!(
            state.load(&mut world, fleet, &troops[1..2]),
            Err(TroopTransportError::CapacityExceeded {
                capacity: 1,
                requested: 2,
            })
        );
        assert_eq!(state.cargo(fleet), &troops[..1]);
        assert_eq!(world.systems[origin].ground_units, troops[1..]);
    }

    // embark (the AI's and the departure's path) loads only from the
    // fleet's own system; an order from elsewhere travels (move_regiment).
    #[test]
    fn a_regiment_in_another_system_cannot_be_loaded_directly() {
        let (mut world, origin, destination, fleet, troops) = fixture(2);
        world.systems[origin]
            .ground_units
            .retain(|&key| key != troops[0]);
        world.systems[destination].ground_units.push(troops[0]);
        let mut state = TroopTransportState::new();

        assert_eq!(
            state.load(&mut world, fleet, &troops[..1]),
            Err(TroopTransportError::TroopNotAtFleetSystem)
        );
        assert!(!state.is_held(fleet));
    }

    #[test]
    fn releasing_a_hold_lets_the_cargo_land() {
        let (mut world, _, _, fleet, troops) = fixture(2);
        let mut state = TroopTransportState::new();
        state.load(&mut world, fleet, &troops[..2]).unwrap();

        state.release(fleet);

        assert_eq!(state.landing_count(fleet), 2);
    }

    #[test]
    fn landing_or_losing_the_cargo_ends_the_hold() {
        let (mut world, origin, _, fleet, troops) = fixture(3);
        let mut state = TroopTransportState::new();
        state.load(&mut world, fleet, &troops[..1]).unwrap();
        state.disembark_all(&mut world, fleet, origin).unwrap();
        assert!(!state.is_held(fleet));

        state.load(&mut world, fleet, &troops[..1]).unwrap();
        state
            .disembark_selected(&mut world, fleet, origin, &troops[..1])
            .unwrap();
        assert!(!state.is_held(fleet));

        state.load(&mut world, fleet, &troops[..1]).unwrap();
        state.destroy_fleet_cargo(&mut world, fleet);
        assert!(!state.is_held(fleet));
    }

    #[test]
    fn a_held_fleet_absorbed_into_another_passes_on_no_hold() {
        let (mut world, origin, _, first, troops) = fixture(3);
        let second = world.fleets.insert(Fleet {
            location: origin,
            capital_ships: vec![],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        let mut state = TroopTransportState::new();
        state.load(&mut world, first, &troops[..1]).unwrap();

        state.transfer_fleet(first, second);

        assert!(!state.is_held(first));
        assert!(!state.is_held(second));
        assert_eq!(state.landing_count(second), 1);
    }
    #[test]
    fn a_regiments_move_is_enabled_for_its_side_on_the_surface() {
        let (mut world, _, _, fleet, troops) = fixture(1);
        let mut state = TroopTransportState::new();

        assert!(state.regiment_move_enabled(&world, troops[0], true));
        assert!(!state.regiment_move_enabled(&world, troops[0], false));
        state.load(&mut world, fleet, &troops[..1]).unwrap();
        assert!(!state.regiment_move_enabled(&world, troops[0], true));
    }

    /// Both systems held by the Alliance, the destination 100 units away:
    /// `(100 / GNPRTB 5120) * 100 / 100` = 20 days at the default speed.
    fn alliance_world(capacity: u32) -> (GameWorld, SystemKey, SystemKey, FleetKey, Vec<TroopKey>) {
        let (mut world, origin, destination, fleet, troops) = fixture(capacity);
        for system in [origin, destination] {
            world.systems[system].control = ControlKind::Controlled(Faction::Alliance);
        }
        world.systems[destination].x = 100;
        (world, origin, destination, fleet, troops)
    }

    fn add_fleet(world: &mut GameWorld, system: SystemKey, capacity: u32) -> FleetKey {
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            name: "Carrier".into(),
            is_alliance: true,
            troop_capacity: capacity,
            hull: 100,
            ..CapitalShipClass::default()
        });
        let fleet = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ShipInstance::new(class, 100, true)],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        world.systems[system].fleets.push(fleet);
        fleet
    }

    fn move_to(
        state: &mut TroopTransportState,
        world: &mut GameWorld,
        troop: TroopKey,
        target: RegimentTarget,
    ) -> Result<RegimentMove, TroopTransportError> {
        state.move_regiment(
            world,
            &MovementState::new(),
            &HashSet::new(),
            troop,
            target,
            10,
        )
    }

    // FUN_00556390: within one system the in-transit bit is written clear
    // and the regiment changes container at once.
    #[test]
    fn a_regiment_unloaded_in_orbit_lands_on_its_sides_planet_at_once() {
        let (mut world, origin, _, fleet, troops) = alliance_world(2);
        let mut state = TroopTransportState::new();
        state.load(&mut world, fleet, &troops[..1]).unwrap();

        let moved = move_to(
            &mut state,
            &mut world,
            troops[0],
            RegimentTarget::System(origin),
        );

        assert_eq!(
            moved,
            Ok(RegimentMove::Placed(RegimentLeg::Surface(origin)))
        );
        assert!(world.systems[origin].ground_units.contains(&troops[0]));
        assert!(state.cargo(fleet).is_empty());
        assert!(!state.is_held(fleet));
        assert!(state.transits().is_empty());
    }

    // FUN_00556390: each command moves only its own object.
    #[test]
    fn unloading_one_regiment_leaves_the_rest_aboard_and_held() {
        let (mut world, origin, _, fleet, troops) = alliance_world(2);
        let mut state = TroopTransportState::new();
        state.load(&mut world, fleet, &troops[..2]).unwrap();

        move_to(
            &mut state,
            &mut world,
            troops[0],
            RegimentTarget::System(origin),
        )
        .unwrap();

        assert_eq!(state.cargo(fleet), &troops[1..2]);
        assert!(state.is_held(fleet));
    }

    // FUN_0053d430: 1/0x28 for another side's destination unless it is an
    // existing, unpopulated system.
    #[test]
    fn a_populated_planet_of_another_side_refuses_a_regiment() {
        let (mut world, origin, _, fleet, troops) = alliance_world(2);
        let mut state = TroopTransportState::new();
        state.load(&mut world, fleet, &troops[..1]).unwrap();
        world.systems[origin].control = ControlKind::Controlled(Faction::Empire);

        let moved = move_to(
            &mut state,
            &mut world,
            troops[0],
            RegimentTarget::System(origin),
        );

        assert_eq!(moved, Err(TroopTransportError::OtherSideDestination));
        assert_eq!(state.cargo(fleet), &troops[..1]);
        assert!(!world.systems[origin].ground_units.contains(&troops[0]));
    }

    // FUN_0053d430 passes an unpopulated system; FUN_005073d0 then refuses
    // the planet (FUN_00553410, 1/0x24) and FUN_00552300 tries each fleet
    // of the side there. FUN_00500b40 counts every regiment aboard, the
    // mover too, so its own full fleet has no room.
    #[test]
    fn an_unpopulated_planet_of_another_side_sends_a_regiment_aboard_a_fleet_with_room() {
        let (mut world, origin, _, full, troops) = alliance_world(1);
        let mut state = TroopTransportState::new();
        state.load(&mut world, full, &troops[..1]).unwrap();
        let roomy = add_fleet(&mut world, origin, 2);
        world.systems[origin].control = ControlKind::Controlled(Faction::Empire);
        world.systems[origin].is_populated = false;

        let moved = move_to(
            &mut state,
            &mut world,
            troops[0],
            RegimentTarget::System(origin),
        );

        assert_eq!(moved, Ok(RegimentMove::Placed(RegimentLeg::Fleet(roomy))));
        assert!(state.cargo(full).is_empty());
        assert_eq!(state.cargo(roomy), &troops[..1]);
        assert!(state.is_held(roomy));
    }

    // FUN_005073d0: 0x90/4 for a blockaded planet, which a regiment keeps
    // when no fleet of its side there has room.
    #[test]
    fn a_blockaded_planet_refuses_a_regiment_with_no_room_aboard() {
        let (mut world, origin, _, fleet, troops) = alliance_world(1);
        let mut state = TroopTransportState::new();
        state.load(&mut world, fleet, &troops[..1]).unwrap();
        let blockaded = HashSet::from([origin]);

        let moved = state.move_regiment(
            &mut world,
            &MovementState::new(),
            &blockaded,
            troops[0],
            RegimentTarget::System(origin),
            10,
        );

        assert_eq!(moved, Err(TroopTransportError::DestinationBlockaded));
        assert_eq!(state.cargo(fleet), &troops[..1]);
    }

    // FUN_004f63f0: a regiment's speed is GNPRTB 1 (100), so FUN_00555920
    // never refuses it 1/0x18; FUN_00556430 times the trip with
    // FUN_0055d8c0.
    #[test]
    fn a_regiment_travels_to_another_planet_of_its_side_at_the_default_speed() {
        let (mut world, origin, destination, _, troops) = alliance_world(1);
        let mut state = TroopTransportState::new();

        let moved = move_to(
            &mut state,
            &mut world,
            troops[0],
            RegimentTarget::System(destination),
        );

        let transit = RegimentTransit {
            troop: troops[0],
            origin,
            leg: RegimentLeg::Surface(destination),
            arrival_tick: 30,
        };
        assert_eq!(moved, Ok(RegimentMove::Departed(transit.clone())));
        assert_eq!(state.transits(), std::slice::from_ref(&transit));
        assert_eq!(regiment_system(&world, troops[0]), None);
        assert!(state.advance_transit(&mut world, 29).arrived.is_empty());
        assert_eq!(state.advance_transit(&mut world, 30).arrived, vec![transit]);
        assert!(world.systems[destination].ground_units.contains(&troops[0]));
        assert!(state.transits().is_empty());
    }

    // FUN_004f9860: an en-route object cannot take a Move.
    #[test]
    fn a_travelling_regiment_cannot_move_again() {
        let (mut world, origin, destination, _, troops) = alliance_world(1);
        let mut state = TroopTransportState::new();
        move_to(
            &mut state,
            &mut world,
            troops[0],
            RegimentTarget::System(destination),
        )
        .unwrap();

        let moved = move_to(
            &mut state,
            &mut world,
            troops[0],
            RegimentTarget::System(origin),
        );

        assert_eq!(moved, Err(TroopTransportError::RegimentEnRoute));
    }

    // FUN_004f8240: a regiment is en route while its container is.
    #[test]
    fn a_regiment_aboard_a_fleet_in_hyperspace_cannot_move() {
        let (mut world, origin, destination, fleet, troops) = alliance_world(1);
        let mut state = TroopTransportState::new();
        state.load(&mut world, fleet, &troops[..1]).unwrap();
        let mut movement = MovementState::new();
        movement.order(fleet, origin, destination, 5);

        let moved = state.move_regiment(
            &mut world,
            &movement,
            &HashSet::new(),
            troops[0],
            RegimentTarget::System(origin),
            10,
        );

        assert_eq!(moved, Err(TroopTransportError::RegimentEnRoute));
    }

    // FUN_00500ac0 → FUN_00553410: 1/0x21 for an en-route ship.
    #[test]
    fn a_fleet_in_hyperspace_refuses_a_regiment() {
        let (mut world, origin, destination, fleet, troops) = alliance_world(1);
        let mut state = TroopTransportState::new();
        let mut movement = MovementState::new();
        movement.order(fleet, origin, destination, 5);

        let moved = state.move_regiment(
            &mut world,
            &movement,
            &HashSet::new(),
            troops[0],
            RegimentTarget::Fleet(fleet),
            10,
        );

        assert_eq!(moved, Err(TroopTransportError::DestinationInTransit));
        assert!(world.systems[origin].ground_units.contains(&troops[0]));
    }

    // FUN_00556390 puts the regiment in the ship's container at once, so it
    // fills the room while it travels (FUN_00500b40).
    #[test]
    fn a_regiment_bound_for_a_fleet_takes_its_room_while_it_travels() {
        let (mut world, _, destination, _, troops) = alliance_world(1);
        let fleet = add_fleet(&mut world, destination, 1);
        let mut state = TroopTransportState::new();

        move_to(
            &mut state,
            &mut world,
            troops[0],
            RegimentTarget::Fleet(fleet),
        )
        .unwrap();
        let second = move_to(
            &mut state,
            &mut world,
            troops[1],
            RegimentTarget::Fleet(fleet),
        );

        assert_eq!(
            second,
            Err(TroopTransportError::CapacityExceeded {
                capacity: 1,
                requested: 2,
            })
        );
        assert_eq!(state.incoming_count(fleet), 1);
        state.advance_transit(&mut world, 30);
        assert_eq!(state.cargo(fleet), &troops[..1]);
        assert!(state.is_held(fleet));
    }

    // The regiment rides in the ship's container, so it ends its trip
    // aboard wherever the fleet has gone.
    #[test]
    fn a_regiment_boards_its_fleet_where_the_fleet_has_gone() {
        let (mut world, origin, destination, _, troops) = alliance_world(1);
        let fleet = add_fleet(&mut world, destination, 1);
        let mut state = TroopTransportState::new();
        move_to(
            &mut state,
            &mut world,
            troops[0],
            RegimentTarget::Fleet(fleet),
        )
        .unwrap();
        world.fleets[fleet].location = origin;

        state.advance_transit(&mut world, 30);

        assert_eq!(state.cargo(fleet), &troops[..1]);
    }

    // FUN_004fc080: GameObjDestroyedOnArrivalNotif.
    #[test]
    fn a_regiment_whose_planet_is_destroyed_is_lost_on_arrival() {
        let (mut world, _, destination, _, troops) = alliance_world(1);
        let mut state = TroopTransportState::new();
        move_to(
            &mut state,
            &mut world,
            troops[0],
            RegimentTarget::System(destination),
        )
        .unwrap();
        world.systems[destination].is_destroyed = true;

        let ended = state.advance_transit(&mut world, 30);

        assert_eq!(ended.lost.len(), 1);
        assert!(!world.troops.contains_key(troops[0]));
        assert!(!world.systems[destination].ground_units.contains(&troops[0]));
    }

    // FUN_004fc080: the destination no longer exists on arrival.
    #[test]
    fn a_regiment_whose_fleet_is_gone_is_lost_on_arrival() {
        let (mut world, _, destination, _, troops) = alliance_world(1);
        let fleet = add_fleet(&mut world, destination, 1);
        let mut state = TroopTransportState::new();
        move_to(
            &mut state,
            &mut world,
            troops[0],
            RegimentTarget::Fleet(fleet),
        )
        .unwrap();
        world.fleets.remove(fleet);

        let ended = state.advance_transit(&mut world, 30);

        assert_eq!(ended.lost.len(), 1);
        assert!(state.cargo(fleet).is_empty());
        assert!(!world.troops.contains_key(troops[0]));
    }

    // The regiment is already in the ship's container (FUN_00556390), so
    // it dies with the fleet.
    #[test]
    fn a_regiment_bound_for_a_destroyed_fleet_dies_with_it() {
        let (mut world, _, destination, _, troops) = alliance_world(1);
        let fleet = add_fleet(&mut world, destination, 1);
        let mut state = TroopTransportState::new();
        move_to(
            &mut state,
            &mut world,
            troops[0],
            RegimentTarget::Fleet(fleet),
        )
        .unwrap();

        let destroyed = state.destroy_fleet_cargo(&mut world, fleet);

        assert_eq!(destroyed, vec![troops[0]]);
        assert!(state.transits().is_empty());
        assert!(!world.troops.contains_key(troops[0]));
    }

    #[test]
    fn a_regiment_bound_for_a_merged_fleet_boards_the_survivor() {
        let (mut world, _, destination, _, troops) = alliance_world(1);
        let first = add_fleet(&mut world, destination, 1);
        let second = add_fleet(&mut world, destination, 1);
        let mut state = TroopTransportState::new();
        move_to(
            &mut state,
            &mut world,
            troops[0],
            RegimentTarget::Fleet(first),
        )
        .unwrap();

        state.transfer_fleet(first, second);
        state.advance_transit(&mut world, 30);

        assert_eq!(state.cargo(second), &troops[..1]);
    }

    // FUN_004f6fd0 leaves an object already in the chosen container alone.
    #[test]
    fn moving_a_regiment_onto_the_fleet_it_is_aboard_changes_nothing() {
        let (mut world, _, _, fleet, troops) = alliance_world(2);
        let mut state = TroopTransportState::new();
        state.load(&mut world, fleet, &troops[..1]).unwrap();

        let moved = move_to(
            &mut state,
            &mut world,
            troops[0],
            RegimentTarget::Fleet(fleet),
        );

        assert_eq!(moved, Ok(RegimentMove::Unchanged));
        assert_eq!(state.cargo(fleet), &troops[..1]);
    }

    #[test]
    fn a_regiment_boarding_from_its_planet_is_held_aboard() {
        let (mut world, origin, _, fleet, troops) = alliance_world(2);
        let mut state = TroopTransportState::new();

        let moved = move_to(
            &mut state,
            &mut world,
            troops[0],
            RegimentTarget::Fleet(fleet),
        );

        assert_eq!(moved, Ok(RegimentMove::Placed(RegimentLeg::Fleet(fleet))));
        assert!(!world.systems[origin].ground_units.contains(&troops[0]));
        assert_eq!(state.landing_count(fleet), 0);
    }

    #[test]
    fn a_travelling_regiment_survives_a_save() {
        let (mut world, _, destination, _, troops) = alliance_world(1);
        let mut state = TroopTransportState::new();
        move_to(
            &mut state,
            &mut world,
            troops[0],
            RegimentTarget::System(destination),
        )
        .unwrap();

        let json = serde_json::to_string(&state).unwrap();
        let decoded: TroopTransportState = serde_json::from_str(&json).unwrap();

        assert_eq!(decoded.transits(), state.transits());
    }

    #[test]
    fn a_refused_regiment_move_reads_as_a_reason() {
        // The message log prints it after "Regiment move rejected: ".
        assert_eq!(
            TroopTransportError::OtherSideDestination.to_string(),
            "the destination belongs to another side"
        );
        assert_eq!(
            TroopTransportError::DestinationBlockaded.to_string(),
            "the destination is blockaded"
        );
    }
}
