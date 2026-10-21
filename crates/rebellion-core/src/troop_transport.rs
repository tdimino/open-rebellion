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
//!
//! A fighter squadron moves by the same order: between a fleet and its
//! system's own squadrons, or to another system on its own at its class
//! hyperdrive ([`TroopTransportState::move_squadron`]).

use std::collections::{HashMap, HashSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::carriage::ProductRoom;
use crate::dat::Faction;
use crate::ids::{FighterKey, FleetKey, SystemKey, TroopKey};
use crate::movement::{default_speed, fighter_speed, transit_ticks_between, MovementState};
use crate::world::GameWorld;

/// Persistent regiment cargo keyed by its carrying fleet.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TroopTransportState {
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    cargo: HashMap<FleetKey, Vec<TroopKey>>,
    /// Regiments travelling on their own, in departure order.
    transit: Vec<RegimentTransit>,
    /// Each carried regiment's ship, by its tag (`ShipInstance::tag`; the
    /// original's regiments are children of a ship, `FUN_00504c40`).
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    carriers: HashMap<TroopKey, u32>,
}

/// Where a regiment's Move is released: the drop window's `+0x70` object
/// (`ghidra/notes/regiment-unload.md`, "The order").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    /// The tag of the ship it boards, chosen at the order (`FUN_00552300`);
    /// 0 for a planet's surface.
    pub carrier: u32,
    /// The game-day it arrives.
    pub arrival_tick: u64,
    /// The destroyed system its leg still names after a destroy cascade
    /// moved it on its way: `+0x3c` keeps it, and the arrival day starts
    /// the next leg from there ([`TroopTransportState::reroute_regiment`]).
    #[serde(default)]
    pub via: Option<SystemKey>,
}

/// A squadron a Move takes: one of a fleet's `fighters` entry `index`, or
/// one of a system's own squadrons of a class and side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SquadronSource {
    Fleet {
        fleet: FleetKey,
        index: usize,
    },
    System {
        system: SystemKey,
        class: FighterKey,
        is_alliance: bool,
    },
}

/// A squadron travelling to another system, held and placed on arrival as a
/// regiment is ([`RegimentTransit`]). A [`RegimentLeg::Surface`] leg puts it
/// among the system's own squadrons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SquadronTransit {
    pub class: FighterKey,
    pub is_alliance: bool,
    /// The system it left.
    pub origin: SystemKey,
    pub leg: RegimentLeg,
    /// The tag of the ship it boards, chosen at the order (`FUN_00552300`);
    /// 0 for a planet's surface.
    pub carrier: u32,
    /// The game-day it arrives.
    pub arrival_tick: u64,
    /// The destroyed system its leg still names after a destroy cascade
    /// moved it on its way: `+0x3c` keeps it, and the arrival day starts
    /// the next leg from there ([`TroopTransportState::reroute_regiment`]).
    #[serde(default)]
    pub via: Option<SystemKey>,
}

/// What a squadron's Move did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SquadronMove {
    /// The squadron entered its new container at once (same system).
    Placed(RegimentLeg),
    /// The squadron left for another system.
    Departed(SquadronTransit),
    /// The leg is the fleet the squadron is already in.
    Unchanged,
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
    /// Squadrons placed in their destination.
    pub squadrons_arrived: Vec<SquadronTransit>,
    /// Squadrons whose destination no longer exists, lost on arrival.
    pub squadrons_lost: Vec<SquadronTransit>,
}

/// Where a regiment is now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegimentPlace {
    Surface(SystemKey),
    Cargo(FleetKey),
}

/// A regiment's Move as its checks settle it: where it is, the system it
/// leaves, the container it enters, and for a fleet the ship that takes it.
#[derive(Debug, Clone, Copy)]
struct RegimentPlan {
    place: RegimentPlace,
    origin: SystemKey,
    leg: RegimentLeg,
    ship: Option<usize>,
}

/// A squadron's Move as its checks settle it.
#[derive(Debug, Clone, Copy)]
struct SquadronPlan {
    class: FighterKey,
    is_alliance: bool,
    origin: SystemKey,
    leg: RegimentLeg,
    ship: Option<usize>,
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
    /// The squadron is gone from its fleet or system.
    MissingSquadron,
    /// The squadron is aboard a fleet in hyperspace (`FUN_004f9860`, `1`/`1`).
    SquadronEnRoute,
    /// No ship of the destination fleet has room for a squadron
    /// (`FUN_00552300`, `1`/`0x27`).
    NoSquadronRoom,
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
            Self::MissingSquadron => formatter.write_str("the squadron no longer exists"),
            Self::SquadronEnRoute => formatter.write_str("the squadron is en route"),
            Self::NoSquadronRoom => {
                formatter.write_str("no ship of the destination fleet has room")
            }
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

    /// Put surface regiments of `fleet`'s system aboard it, as a regiment's
    /// Move onto the fleet does within one system ([`Self::move_regiment`]).
    /// They stay aboard until a Move or a Planetary Assault puts them down
    /// (`crate::assault`; `ghidra/notes/ground-assault.md`, "Landing
    /// without the order").
    ///
    /// The refusals are [`Self::embark`]'s: a regiment of the other side, one
    /// in another system, or more than the fleet's room (`FUN_00500b40`:
    /// capacity minus regiments aboard, on their way and built for each
    /// ship).
    ///
    /// # Errors
    /// Returns [`Self::embark`]'s error, and then nothing changes.
    pub fn load(
        &mut self,
        world: &mut GameWorld,
        products: &ProductRoom,
        fleet: FleetKey,
        troops: &[TroopKey],
    ) -> Result<(), TroopTransportError> {
        self.embark(world, products, fleet, troops)
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

    /// The system a travelling regiment stands at: the original moves it
    /// into its new container at once (`FUN_00556430`), so the planet it
    /// lands on or the system of the fleet it boards.
    #[must_use]
    pub fn transit_system(&self, world: &GameWorld, troop: TroopKey) -> Option<SystemKey> {
        let transit = self.transit.iter().find(|transit| transit.troop == troop)?;
        match transit.leg {
            RegimentLeg::Surface(system) => Some(system),
            RegimentLeg::Fleet(fleet) => world.fleets.get(fleet).map(|fleet| fleet.location),
        }
    }

    /// Drop a travelling regiment's transit, as when it is destroyed en
    /// route (`FUN_00530270`).
    pub fn forget_transit(&mut self, troop: TroopKey) {
        self.transit.retain(|transit| transit.troop != troop);
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
    /// GNPRTB 1 rate of `FUN_004f63f0`.
    ///
    /// port: the leg builder's first pick, a fleet with `+0x58` bit 1, is
    /// skipped (who sets that bit is untraced), and so is the side-3
    /// planet with `+0x88` bit 1 that `FUN_005073d0` lets a regiment land on.
    ///
    /// # Errors
    /// Returns the refusal, and then nothing changes.
    #[expect(
        clippy::too_many_arguments,
        reason = "The order reads the world, the movement and blockade states and the products' room."
    )]
    pub fn move_regiment(
        &mut self,
        world: &mut GameWorld,
        movement: &MovementState,
        blockaded: &HashSet<SystemKey>,
        products: &ProductRoom,
        troop: TroopKey,
        target: RegimentTarget,
        tick: u64,
    ) -> Result<RegimentMove, TroopTransportError> {
        let Some(plan) = self.plan_regiment(world, movement, blockaded, products, troop, target)?
        else {
            return Ok(RegimentMove::Unchanged);
        };
        let carrier = match (plan.leg, plan.ship) {
            (RegimentLeg::Fleet(fleet), Some(index)) => {
                crate::carriage::tag_ship(world, fleet, index)
                    .ok_or(TroopTransportError::MissingFleet)?
            }
            _ => 0,
        };
        self.execute(world, troop, plan.place, plan.origin, plan.leg, carrier, tick)
    }

    /// The days a regiment's Move to `target` lists in the confirmation
    /// (`FUN_0053d200` → `FUN_00555d30`): [`Self::move_regiment`]'s checks
    /// without carrying it out (`FUN_00487740` validates before
    /// `FUN_00487cc0` asks), then none within one system, else its trip at
    /// the default speed.
    ///
    /// # Errors
    /// [`Self::move_regiment`]'s refusals.
    pub fn regiment_move_days(
        &self,
        world: &GameWorld,
        movement: &MovementState,
        blockaded: &HashSet<SystemKey>,
        products: &ProductRoom,
        troop: TroopKey,
        target: RegimentTarget,
    ) -> Result<u32, TroopTransportError> {
        let Some(plan) = self.plan_regiment(world, movement, blockaded, products, troop, target)?
        else {
            return Ok(0);
        };
        let destination = leg_system(world, plan.leg)?;
        Ok(leg_ticks(world, plan.origin, destination, default_speed(world)))
    }

    /// [`Self::move_regiment`]'s checks and choices, changing nothing; `None`
    /// when the regiment would stay where it is.
    fn plan_regiment(
        &self,
        world: &GameWorld,
        movement: &MovementState,
        blockaded: &HashSet<SystemKey>,
        products: &ProductRoom,
        troop: TroopKey,
        target: RegimentTarget,
    ) -> Result<Option<RegimentPlan>, TroopTransportError> {
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
        let leg = self.choose_leg(world, movement, blockaded, products, is_alliance, target)?;
        if let (RegimentPlace::Cargo(current), RegimentLeg::Fleet(fleet)) = (place, leg) {
            if current == fleet {
                return Ok(None);
            }
        }
        let ship = match leg {
            RegimentLeg::Surface(_) => None,
            RegimentLeg::Fleet(fleet) => Some(
                crate::carriage::ship_for_regiment(world, self, products, fleet)
                    .ok_or_else(|| self.room_refusal(world, products, fleet, 1))?,
            ),
        };
        Ok(Some(RegimentPlan {
            place,
            origin,
            leg,
            ship,
        }))
    }

    /// Carry out a squadron's Move (`0x201`) released on `target`.
    ///
    /// A squadron (`0x1c`) takes the regiment's path through the order
    /// (`ghidra/notes/regiment-unload.md`) without the regiment's
    /// exceptions:
    /// - the group test (`FUN_0053d430`) refuses another side's system,
    ///   populated or not (`1`/`0x28`);
    /// - a planet takes it when it is the squadron's side's, not destroyed
    ///   and not blockaded (`FUN_005073d0`: the `0x204` lifts cover only
    ///   types `0x08..0x14`); its room is unlimited (`FUN_00507750`);
    /// - otherwise, and for a fleet, a fleet of the side not in hyperspace
    ///   takes it on the ship with the most room (`FUN_00552300`,
    ///   `crate::carriage::ship_for_squadron`), and no room refuses
    ///   (`1`/`0x27`);
    /// - across systems it travels at its class hyperdrive, else its
    ///   damaged rating, else the default (`FUN_00502f80`), which is never
    ///   0, so `FUN_00555920`'s `1`/`0x18` never refuses.
    ///
    /// The ship is chosen at the order and the squadron is its child from
    /// then (`FUN_00556390`), so it takes that ship's room while it travels
    /// and arrives aboard it.
    ///
    /// # Errors
    /// Returns the refusal, and then nothing changes.
    #[expect(
        clippy::too_many_arguments,
        reason = "The order reads the world, the movement and blockade states and the products' room."
    )]
    pub fn move_squadron(
        &mut self,
        world: &mut GameWorld,
        movement: &MovementState,
        blockaded: &HashSet<SystemKey>,
        products: &ProductRoom,
        source: SquadronSource,
        target: RegimentTarget,
        tick: u64,
    ) -> Result<SquadronMove, TroopTransportError> {
        let Some(plan) = self.plan_squadron(world, movement, blockaded, products, source, target)?
        else {
            return Ok(SquadronMove::Unchanged);
        };
        let SquadronPlan {
            class,
            is_alliance,
            origin,
            leg,
            ship,
        } = plan;
        let carrier = match (leg, ship) {
            (RegimentLeg::Fleet(fleet), Some(index)) => {
                crate::carriage::tag_ship(world, fleet, index)
                    .ok_or(TroopTransportError::NoSquadronRoom)?
            }
            _ => 0,
        };
        match source {
            SquadronSource::Fleet { fleet, index } => {
                let value = &mut world.fleets[fleet];
                value.fighters[index].count -= 1;
                if value.fighters[index].count == 0 {
                    value.fighters.remove(index);
                }
            }
            SquadronSource::System { system, .. } => {
                world.systems[system].take_squadron(class, is_alliance);
            }
        }
        Ok(depart_squadron(
            world,
            class,
            is_alliance,
            origin,
            leg,
            carrier,
            tick,
        ))
    }

    /// The days a squadron's Move to `target` lists in the confirmation
    /// (`FUN_0053d200` → `FUN_00555d30`): [`Self::move_squadron`]'s checks
    /// without carrying it out, then none within one system, else its trip
    /// at its class's speed (`FUN_00502f80`).
    ///
    /// # Errors
    /// [`Self::move_squadron`]'s refusals.
    pub fn squadron_move_days(
        &self,
        world: &GameWorld,
        movement: &MovementState,
        blockaded: &HashSet<SystemKey>,
        products: &ProductRoom,
        source: SquadronSource,
        target: RegimentTarget,
    ) -> Result<u32, TroopTransportError> {
        let Some(plan) = self.plan_squadron(world, movement, blockaded, products, source, target)?
        else {
            return Ok(0);
        };
        let destination = leg_system(world, plan.leg)?;
        let speed = world
            .fighter_classes
            .get(plan.class)
            .map_or_else(|| default_speed(world), |value| fighter_speed(world, value));
        Ok(leg_ticks(world, plan.origin, destination, speed))
    }

    /// [`Self::move_squadron`]'s checks and choices, changing nothing;
    /// `None` when the squadron would stay aboard its fleet.
    fn plan_squadron(
        &self,
        world: &GameWorld,
        movement: &MovementState,
        blockaded: &HashSet<SystemKey>,
        products: &ProductRoom,
        source: SquadronSource,
        target: RegimentTarget,
    ) -> Result<Option<SquadronPlan>, TroopTransportError> {
        let (class, is_alliance, origin) = match source {
            SquadronSource::Fleet { fleet, index } => {
                let value = world
                    .fleets
                    .get(fleet)
                    .ok_or(TroopTransportError::MissingFleet)?;
                let entry = value
                    .fighters
                    .get(index)
                    .filter(|entry| entry.count > 0)
                    .ok_or(TroopTransportError::MissingSquadron)?;
                if movement.is_in_transit(fleet) {
                    return Err(TroopTransportError::SquadronEnRoute);
                }
                (entry.class, value.is_alliance, value.location)
            }
            SquadronSource::System {
                system,
                class,
                is_alliance,
            } => {
                let value = world
                    .systems
                    .get(system)
                    .ok_or(TroopTransportError::MissingSystem)?;
                if !value
                    .fighters
                    .iter()
                    .any(|entry| entry.class == class && entry.is_alliance == is_alliance)
                {
                    return Err(TroopTransportError::MissingSquadron);
                }
                (class, is_alliance, system)
            }
        };
        if let RegimentTarget::System(system) = target {
            let value = world
                .systems
                .get(system)
                .ok_or(TroopTransportError::MissingSystem)?;
            if !value.control.is_controlled_by(side(is_alliance)) {
                return Err(TroopTransportError::OtherSideDestination);
            }
        }
        let leg = match target {
            RegimentTarget::Fleet(fleet) => {
                self.accept_squadron_fleet(world, movement, products, is_alliance, fleet)?;
                RegimentLeg::Fleet(fleet)
            }
            RegimentTarget::System(system) => {
                match Self::accept_planet(world, blockaded, is_alliance, system, false) {
                    Ok(()) => RegimentLeg::Surface(system),
                    Err(refusal) => world.systems[system]
                        .fleets
                        .iter()
                        .copied()
                        .find(|&fleet| {
                            self.accept_squadron_fleet(
                                world,
                                movement,
                                products,
                                is_alliance,
                                fleet,
                            )
                            .is_ok()
                        })
                        .map(RegimentLeg::Fleet)
                        .ok_or(refusal)?,
                }
            }
        };
        if let (SquadronSource::Fleet { fleet: current, .. }, RegimentLeg::Fleet(fleet)) =
            (source, leg)
        {
            if current == fleet {
                return Ok(None);
            }
        }
        let ship = match leg {
            RegimentLeg::Surface(_) => None,
            RegimentLeg::Fleet(fleet) => Some(
                crate::carriage::ship_for_squadron(world, products, fleet)
                    .ok_or(TroopTransportError::NoSquadronRoom)?,
            ),
        };
        Ok(Some(SquadronPlan {
            class,
            is_alliance,
            origin,
            leg,
            ship,
        }))
    }

    /// Whether `fleet` would take one more squadron of the side
    /// ([`Self::accept_squadron_fleet`]).
    #[must_use]
    pub fn squadron_fleet_accepts(
        &self,
        world: &GameWorld,
        movement: &MovementState,
        products: &ProductRoom,
        is_alliance: bool,
        fleet: FleetKey,
    ) -> bool {
        self.accept_squadron_fleet(world, movement, products, is_alliance, fleet)
            .is_ok()
    }

    /// `FUN_00500ac0` and `FUN_00500b40` for a squadron: a fleet of the
    /// side, not in hyperspace, with a ship that has room for one more
    /// besides those aboard, on their way to it and built for it
    /// (`crate::carriage::ship_for_squadron`).
    fn accept_squadron_fleet(
        &self,
        world: &GameWorld,
        movement: &MovementState,
        products: &ProductRoom,
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
        crate::carriage::ship_for_squadron(world, products, fleet)
            .map(|_| ())
            .ok_or(TroopTransportError::NoSquadronRoom)
    }

    /// Advance travelling regiments to `now`: each whose day has come enters
    /// its destination, or is lost when the destination is gone or the
    /// planet destroyed (`FUN_004fc080`). One bound for a fleet arrives
    /// aboard the ship chosen at its order, wherever that ship is
    /// (`FUN_00556620` makes no new choice). hyp: one whose ship is lost
    /// meanwhile is lost with it, as cargo aboard a lost ship is
    /// (`crate::carriage::drop_lost_squadrons`); the original's fate for it
    /// is untraced (`ghidra/notes/fleet-join-split.md`). One a destroy
    /// cascade moved on its way starts its next leg on its day instead
    /// (event `0x306` -> `FUN_00556620.c:27-31`): from the destroyed system
    /// to its place now, `FUN_0055d8c0` ticks at its speed, none when both
    /// are one system.
    pub fn advance_transit(&mut self, world: &mut GameWorld, now: u64) -> RegimentArrivals {
        let mut result = RegimentArrivals::default();
        let mut remaining = Vec::with_capacity(self.transit.len());
        for mut transit in std::mem::take(&mut self.transit) {
            if transit.arrival_tick <= now {
                if let Some(via) = transit.via.take() {
                    let to = leg_location(world, transit.leg, transit.carrier).unwrap_or(via);
                    let ticks = leg_ticks(world, via, to, default_speed(world));
                    transit.arrival_tick += u64::from(ticks);
                    transit.origin = via;
                }
            }
            if transit.arrival_tick > now {
                remaining.push(transit);
                continue;
            }
            let entered = match transit.leg {
                RegimentLeg::Surface(system) => match world.systems.get_mut(system) {
                    Some(value) if !value.is_destroyed => {
                        insert_sorted(&mut value.ground_units, transit.troop);
                        crate::support_shift::regiments_changed(world, system);
                        true
                    }
                    _ => false,
                },
                RegimentLeg::Fleet(_) => {
                    match crate::carriage::fleet_of_ship(world, transit.carrier) {
                        Some(fleet) => {
                            self.stow(fleet, transit.troop, transit.carrier);
                            true
                        }
                        None => false,
                    }
                }
            };
            if entered {
                result.arrived.push(transit);
            } else {
                world.troops.remove(transit.troop);
                result.lost.push(transit);
            }
        }
        self.transit = remaining;
        let mut waiting = Vec::with_capacity(world.squadron_transits.len());
        for mut transit in std::mem::take(&mut world.squadron_transits) {
            if transit.arrival_tick <= now {
                if let Some(via) = transit.via.take() {
                    let to = leg_location(world, transit.leg, transit.carrier).unwrap_or(via);
                    let speed = world.fighter_classes.get(transit.class).map_or_else(
                        || default_speed(world),
                        |value| fighter_speed(world, value),
                    );
                    transit.arrival_tick += u64::from(leg_ticks(world, via, to, speed));
                    transit.origin = via;
                }
            }
            if transit.arrival_tick > now {
                waiting.push(transit);
                continue;
            }
            let entered = match transit.leg {
                RegimentLeg::Surface(system) => world
                    .systems
                    .get(system)
                    .is_some_and(|value| !value.is_destroyed),
                RegimentLeg::Fleet(_) => {
                    crate::carriage::fleet_of_ship(world, transit.carrier).is_some()
                }
            };
            if entered {
                place_squadron(
                    world,
                    transit.leg,
                    transit.class,
                    transit.is_alliance,
                    transit.carrier,
                );
                // FUN_00508660 → FUN_0050a1b0: a system records the side of
                // a squadron arriving on it.
                if let RegimentLeg::Surface(system) = transit.leg {
                    world.arrival_holders.insert(system, transit.is_alliance);
                }
                result.squadrons_arrived.push(transit);
            } else {
                result.squadrons_lost.push(transit);
            }
        }
        world.squadron_transits = waiting;
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
        products: &ProductRoom,
        is_alliance: bool,
        target: RegimentTarget,
    ) -> Result<RegimentLeg, TroopTransportError> {
        match target {
            RegimentTarget::Fleet(fleet) => {
                self.accept_fleet(world, movement, products, is_alliance, fleet)?;
                Ok(RegimentLeg::Fleet(fleet))
            }
            RegimentTarget::System(system) => {
                let refusal = match Self::accept_planet(world, blockaded, is_alliance, system, true) {
                    Ok(()) => return Ok(RegimentLeg::Surface(system)),
                    Err(error) => error,
                };
                world.systems[system]
                    .fleets
                    .iter()
                    .copied()
                    .find(|&fleet| {
                        self.accept_fleet(world, movement, products, is_alliance, fleet)
                            .is_ok()
                    })
                    .map(RegimentLeg::Fleet)
                    .ok_or(refusal)
            }
        }
    }

    /// `FUN_005073d0` for a `0x204` move: room is unlimited
    /// (`FUN_00507750`). For a regiment (`[0x10,0x14]`) it lifts
    /// `FUN_00553410`'s side refusal at a side-3 (`+0x24` bits 6-7 `0xc0`)
    /// unpopulated planet (`+0x88` bit 0 clear, bit 1 set by every
    /// system's Populate, `FUN_005081f0`); a squadron keeps it.
    fn accept_planet(
        world: &GameWorld,
        blockaded: &HashSet<SystemKey>,
        is_alliance: bool,
        system: SystemKey,
        regiment: bool,
    ) -> Result<(), TroopTransportError> {
        let value = world
            .systems
            .get(system)
            .ok_or(TroopTransportError::MissingSystem)?;
        let neutral_unpopulated = !value.is_populated
            && !matches!(
                value.control.faction(),
                Some(Faction::Alliance | Faction::Empire)
            );
        if value.is_destroyed {
            Err(TroopTransportError::DestinationDestroyed)
        } else if !(value.control.is_controlled_by(side(is_alliance))
            || (regiment && neutral_unpopulated))
        {
            Err(TroopTransportError::OtherSideDestination)
        } else if blockaded.contains(&system) {
            Err(TroopTransportError::DestinationBlockaded)
        } else {
            Ok(())
        }
    }

    /// `FUN_00500ac0` and `FUN_00500b40`: a fleet of the side, not in
    /// hyperspace, with a ship that has room for one more regiment besides
    /// those aboard, on their way to it and built for it
    /// (`crate::carriage::ship_for_regiment`). The room counts every
    /// regiment aboard, so a regiment's own full fleet has none.
    fn accept_fleet(
        &self,
        world: &GameWorld,
        movement: &MovementState,
        products: &ProductRoom,
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
        crate::carriage::ship_for_regiment(world, self, products, fleet)
            .map(|_| ())
            .ok_or_else(|| self.room_refusal(world, products, fleet, 1))
    }

    /// The refusal of `count` more regiments for `fleet` (`1`/`0x27`): the
    /// fleet's capacity against those aboard, on their way, built for its
    /// ships and asked for.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Regiment counts are far below u32::MAX."
    )]
    fn room_refusal(
        &self,
        world: &GameWorld,
        products: &ProductRoom,
        fleet: FleetKey,
        count: usize,
    ) -> TroopTransportError {
        // The usable ships the load could board (`FUN_00552300.c:108,147`,
        // mode 1; `crate::carriage::ships_for`).
        let usable = world.fleets.get(fleet).map(|value| {
            value
                .capital_ships
                .iter()
                .filter(|ship| ship.usable())
                .collect::<Vec<_>>()
        });
        let built: u32 = usable.as_ref().map_or(0, |ships| {
            ships
                .iter()
                .map(|ship| products.regiments(ship.tag))
                .sum()
        });
        match usable {
            Some(ships) => TroopTransportError::CapacityExceeded {
                capacity: ships
                    .iter()
                    .filter_map(|ship| world.capital_ship_classes.get(ship.class))
                    .map(|class| class.troop_capacity)
                    .fold(0_u32, u32::saturating_add),
                requested: (self.carried_count(fleet) + self.incoming_count(fleet) + count) as u32
                    + built,
            },
            None => TroopTransportError::MissingFleet,
        }
    }

    /// `FUN_00556390`: take the regiment out of its container, then place it
    /// at once within its system or start its transit.
    #[expect(
        clippy::too_many_arguments,
        reason = "Where the regiment is and goes, the ship that takes it and the day are one order's fields."
    )]
    fn execute(
        &mut self,
        world: &mut GameWorld,
        troop: TroopKey,
        place: RegimentPlace,
        origin: SystemKey,
        leg: RegimentLeg,
        carrier: u32,
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
                crate::support_shift::regiments_changed(world, system);
            }
            RegimentPlace::Cargo(fleet) => {
                if let Some(cargo) = self.cargo.get_mut(&fleet) {
                    cargo.retain(|&key| key != troop);
                }
                self.forget_if_empty(fleet);
            }
        }
        Ok(self.depart(world, troop, origin, destination, leg, carrier, tick))
    }

    /// The regiment enters `leg` at once within one system, aboard the ship
    /// tagged `carrier` for a fleet, else travels there from `origin` at the
    /// default speed.
    #[expect(
        clippy::too_many_arguments,
        reason = "The leg, its ship and its timing are one order's fields."
    )]
    fn depart(
        &mut self,
        world: &mut GameWorld,
        troop: TroopKey,
        origin: SystemKey,
        destination: SystemKey,
        leg: RegimentLeg,
        carrier: u32,
        tick: u64,
    ) -> RegimentMove {
        if destination == origin {
            match leg {
                RegimentLeg::Surface(system) => {
                    insert_sorted(&mut world.systems[system].ground_units, troop);
                    crate::support_shift::regiments_changed(world, system);
                }
                RegimentLeg::Fleet(fleet) => self.stow(fleet, troop, carrier),
            }
            return RegimentMove::Placed(leg);
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
            carrier,
            arrival_tick: tick + u64::from(ticks),
            via: None,
        };
        self.transit.push(transit.clone());
        RegimentMove::Departed(transit)
    }

    /// Relocate a regiment travelling to a planet's surface onto
    /// `destination`'s surface (`FUN_00515480` → `FUN_00556390`), as the
    /// destroy cascade's autoscrap request does. The move reparents it at
    /// once but leaves its timer `0x387`, `+0x3c` and `+0x44` alone
    /// (`FUN_00556390.c:17-21`, `FUN_00556430.c:30`): it keeps its arrival
    /// day, and [`Self::advance_transit`] starts the next leg on that day.
    /// `None` when it is not travelling.
    pub fn reroute_regiment(&mut self, troop: TroopKey, destination: SystemKey) -> Option<RegimentMove> {
        let transit = self
            .transit
            .iter_mut()
            .find(|transit| transit.troop == troop)?;
        if let RegimentLeg::Surface(via) = transit.leg {
            transit.via = Some(via);
        }
        transit.leg = RegimentLeg::Surface(destination);
        transit.carrier = 0;
        Some(RegimentMove::Departed(transit.clone()))
    }

    /// Regiments aboard `fleet`'s ship tagged `tag`.
    #[must_use]
    pub fn regiments_aboard(&self, fleet: FleetKey, tag: u32) -> usize {
        self.cargo(fleet)
            .iter()
            .filter(|troop| self.carrier(**troop) == tag)
            .count()
    }

    /// Regiments on their way to the ship tagged `tag`, each already its
    /// child (`FUN_00556390`).
    #[must_use]
    pub fn regiments_bound_for(&self, tag: u32) -> usize {
        if tag == 0 {
            return 0;
        }
        self.transit
            .iter()
            .filter(|transit| transit.carrier == tag)
            .count()
    }

    /// The tag of the ship `troop` rides aboard; 0 when it is not carried.
    #[must_use]
    pub fn carrier(&self, troop: TroopKey) -> u32 {
        self.carriers.get(&troop).copied().unwrap_or(0)
    }

    /// A finished regiment product aboard the ship its order chose: the
    /// original created it there at the order (`FUN_0052a140` ->
    /// `FUN_004f7d50`) and its completion only sets the completed bit
    /// (`FUN_0052bee0`).
    pub fn board_product(&mut self, fleet: FleetKey, troop: TroopKey, carrier: u32) {
        self.stow(fleet, troop, carrier);
    }

    /// Put `troop` in `fleet`'s cargo aboard the ship tagged `carrier`.
    fn stow(&mut self, fleet: FleetKey, troop: TroopKey, carrier: u32) {
        insert_sorted(self.cargo.entry(fleet).or_default(), troop);
        self.carriers.insert(troop, carrier);
    }

    fn forget_if_empty(&mut self, fleet: FleetKey) {
        if self.cargo.get(&fleet).is_some_and(Vec::is_empty) {
            self.cargo.remove(&fleet);
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
    /// or more than the ships' room. Each regiment boards the ship with the
    /// most room left by those before it, those aboard, those on their way
    /// and those built for it (`crate::carriage::ships_for`).
    ///
    /// # Errors
    /// Returns a transport error for an empty selection, missing entities, duplicate
    /// or already embarked troops, faction/location mismatches, or insufficient capacity.
    pub fn embark(
        &mut self,
        world: &mut GameWorld,
        products: &ProductRoom,
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

        let ships = crate::carriage::ships_for(
            world,
            self,
            products,
            fleet,
            crate::carriage::Cargo::Regiment,
            troops.len(),
        )
        .ok_or_else(|| self.room_refusal(world, products, fleet, troops.len()))?;
        let mut carriers = Vec::with_capacity(ships.len());
        for index in ships {
            carriers.push(
                crate::carriage::tag_ship(world, fleet, index)
                    .ok_or(TroopTransportError::MissingFleet)?,
            );
        }

        if let Some(system) = world.systems.get_mut(system_key) {
            system
                .ground_units
                .retain(|troop| !requested_keys.contains(troop));
        }
        crate::support_shift::regiments_changed(world, system_key);
        for (&troop, carrier) in troops.iter().zip(carriers) {
            self.stow(fleet, troop, carrier);
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
        landed.retain(|troop| world.troops.contains_key(*troop));
        landed.sort_unstable();
        landed.dedup();
        system.ground_units.extend(landed.iter().copied());
        system.ground_units.sort_unstable();
        system.ground_units.dedup();
        if !landed.is_empty() {
            crate::support_shift::regiments_changed(world, destination);
        }
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
        if !landed.is_empty() {
            crate::support_shift::regiments_changed(world, destination);
        }
        Ok(landed)
    }

    /// Preserve cargo identity when compatible fleet records consolidate.
    pub fn transfer_fleet(&mut self, world: &mut GameWorld, from: FleetKey, to: FleetKey) {
        if from == to {
            return;
        }
        for transit in &mut self.transit {
            if transit.leg == RegimentLeg::Fleet(from) {
                transit.leg = RegimentLeg::Fleet(to);
            }
        }
        for transit in &mut world.squadron_transits {
            if transit.leg == RegimentLeg::Fleet(from) {
                transit.leg = RegimentLeg::Fleet(to);
            }
        }
        let Some(mut moved) = self.cargo.remove(&from) else {
            return;
        };
        let cargo = self.cargo.entry(to).or_default();
        cargo.append(&mut moved);
        cargo.sort_unstable();
        cargo.dedup();
    }

    /// Move `troops`, regiments aboard `from`, aboard `to`.
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
        for troop in moved {
            insert_sorted(self.cargo.entry(to).or_default(), troop);
        }
        self.forget_if_empty(from);
    }

    /// Destroy every regiment aboard a fleet that has been destroyed.
    ///
    /// A regiment or squadron on its way to board the fleet dies with it:
    /// the original has already put it in the ship's container
    /// (`FUN_00556390`).
    pub fn destroy_fleet_cargo(&mut self, world: &mut GameWorld, fleet: FleetKey) -> Vec<TroopKey> {
        let mut cargo = self.cargo.remove(&fleet).unwrap_or_default();
        self.transit.retain(|transit| {
            let bound = transit.leg == RegimentLeg::Fleet(fleet);
            if bound {
                cargo.push(transit.troop);
            }
            !bound
        });
        world
            .squadron_transits
            .retain(|transit| transit.leg != RegimentLeg::Fleet(fleet));
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
                        crate::carriage::lost_with_ship(value, self.carrier(troop))
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

/// Put one squadron in its leg's container: among the system's own, or
/// aboard the ship tagged `carrier`, in whatever fleet that ship is.
fn place_squadron(
    world: &mut GameWorld,
    leg: RegimentLeg,
    class: FighterKey,
    is_alliance: bool,
    carrier: u32,
) {
    match leg {
        RegimentLeg::Surface(system) => {
            if let Some(value) = world.systems.get_mut(system) {
                value.add_squadrons(class, 1, is_alliance);
            }
        }
        RegimentLeg::Fleet(_) => {
            if let Some(fleet) = crate::carriage::fleet_of_ship(world, carrier) {
                crate::carriage::board_squadron(world, fleet, class, carrier);
            }
        }
    }
}

/// One squadron leaves `origin` for `leg`, aboard the ship tagged `carrier`
/// for a fleet: it enters at once within one system, else travels at its
/// class hyperdrive, else its damaged rating, else the default
/// (`FUN_00502f80`).
fn depart_squadron(
    world: &mut GameWorld,
    class: FighterKey,
    is_alliance: bool,
    origin: SystemKey,
    leg: RegimentLeg,
    carrier: u32,
    tick: u64,
) -> SquadronMove {
    let destination = match leg {
        RegimentLeg::Surface(system) => system,
        RegimentLeg::Fleet(fleet) => world.fleets[fleet].location,
    };
    if destination == origin {
        place_squadron(world, leg, class, is_alliance, carrier);
        return SquadronMove::Placed(leg);
    }
    let speed = world
        .fighter_classes
        .get(class)
        .map_or_else(|| default_speed(world), |value| fighter_speed(world, value));
    let position = |key: SystemKey| world.systems.get(key).map_or((0, 0), |s| (s.x, s.y));
    let ticks = transit_ticks_between(world, position(origin), position(destination), speed);
    let transit = SquadronTransit {
        class,
        is_alliance,
        origin,
        leg,
        carrier,
        arrival_tick: tick + u64::from(ticks),
        via: None,
    };
    world.squadron_transits.push(transit.clone());
    SquadronMove::Departed(transit)
}

/// Relocate the squadron transit at `index` onto `leg`, as the destroy
/// cascade's autoscrap request does (`FUN_00515480` → `FUN_00556390`).
/// A fleet leg takes the ship with the most room, which the chain's fleet
/// test has found (`TroopTransportState::squadron_fleet_accepts`). As for
/// a regiment ([`TroopTransportState::reroute_regiment`]) it keeps its
/// arrival day, and the next leg starts on that day. `None` when there is
/// no such transit, or no ship of the fleet has room, and then the squadron
/// is gone.
pub fn reroute_squadron(
    world: &mut GameWorld,
    products: &ProductRoom,
    index: usize,
    leg: RegimentLeg,
) -> Option<SquadronMove> {
    if index >= world.squadron_transits.len() {
        return None;
    }
    let carrier = match leg {
        RegimentLeg::Surface(_) => 0,
        RegimentLeg::Fleet(fleet) => {
            let Some(carrier) = crate::carriage::ship_for_squadron(world, products, fleet)
                .and_then(|index| crate::carriage::tag_ship(world, fleet, index))
            else {
                world.squadron_transits.remove(index);
                return None;
            };
            carrier
        }
    };
    let transit = &mut world.squadron_transits[index];
    if let RegimentLeg::Surface(via) = transit.leg {
        transit.via = Some(via);
    }
    transit.leg = leg;
    transit.carrier = carrier;
    Some(SquadronMove::Departed(transit.clone()))
}

/// The system a leg ends at now: a surface's, or that of the fleet holding
/// the carrying ship.
fn leg_location(world: &GameWorld, leg: RegimentLeg, carrier: u32) -> Option<SystemKey> {
    match leg {
        RegimentLeg::Surface(system) => Some(system),
        RegimentLeg::Fleet(_) => crate::carriage::fleet_of_ship(world, carrier)
            .and_then(|fleet| world.fleets.get(fleet))
            .map(|fleet| fleet.location),
    }
}

/// The system a leg ends at: a surface's, or the fleet's.
fn leg_system(world: &GameWorld, leg: RegimentLeg) -> Result<SystemKey, TroopTransportError> {
    match leg {
        RegimentLeg::Surface(system) => Ok(system),
        RegimentLeg::Fleet(fleet) => world
            .fleets
            .get(fleet)
            .map(|value| value.location)
            .ok_or(TroopTransportError::MissingFleet),
    }
}

/// `FUN_0055d8c0`'s ticks from `from` to `to` at `speed`; none within one
/// system (`FUN_00556430.c:40-51`).
fn leg_ticks(world: &GameWorld, from: SystemKey, to: SystemKey, speed: i64) -> u32 {
    if from == to {
        return 0;
    }
    let position = |key: SystemKey| world.systems.get(key).map_or((0, 0), |s| (s.x, s.y));
    transit_ticks_between(world, position(from), position(to), speed)
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
            fighters: Vec::new(),
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
            bombarding: false,
        });
        world.systems[origin].fleets.push(fleet);

        let troops: Vec<_> = (0..3)
            .map(|index| {
                world.troops.insert(TroopUnit {
                    class_dat_id: DatId::new(0x1000_0001 + index),
                    is_alliance: true,
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
            state.embark(&mut world, &ProductRoom::default(), fleet, &troops),
            Err(TroopTransportError::CapacityExceeded {
                capacity: 2,
                requested: 3,
            })
        );
        assert_eq!(world.systems[origin].ground_units.len(), 3);
        assert!(state.cargo(fleet).is_empty());

        state
            .embark(&mut world, &ProductRoom::default(), fleet, &troops[..2])
            .unwrap();
        assert_eq!(state.cargo(fleet), &troops[..2]);
        assert_eq!(world.systems[origin].ground_units, vec![troops[2]]);
    }

    #[test]
    fn wrong_faction_rejection_leaves_surface_unchanged() {
        let (mut world, origin, _, fleet, troops) = fixture(2);
        world.troops[troops[1]].is_alliance = false;
        let mut state = TroopTransportState::new();

        assert_eq!(
            state.embark(&mut world, &ProductRoom::default(), fleet, &troops[..2]),
            Err(TroopTransportError::WrongFaction)
        );
        assert_eq!(world.systems[origin].ground_units.len(), 3);
        assert!(state.cargo(fleet).is_empty());
    }

    #[test]
    fn disembark_moves_only_live_cargo_to_destination() {
        let (mut world, _, destination, fleet, troops) = fixture(3);
        let mut state = TroopTransportState::new();
        state
            .embark(&mut world, &ProductRoom::default(), fleet, &troops)
            .unwrap();
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
        state
            .embark(&mut world, &ProductRoom::default(), fleet, &troops)
            .unwrap();

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
            bombarding: false,
        });
        let mut state = TroopTransportState::new();
        state
            .embark(&mut world, &ProductRoom::default(), first, &troops[..2])
            .unwrap();

        state.transfer_fleet(&mut world, first, second);
        assert!(state.cargo(first).is_empty());
        assert_eq!(state.cargo(second), &troops[..2]);
    }

    #[test]
    fn destroyed_transport_removes_its_embarked_regiments() {
        let (mut world, _, _, fleet, troops) = fixture(2);
        let mut state = TroopTransportState::new();
        state
            .embark(&mut world, &ProductRoom::default(), fleet, &troops[..2])
            .unwrap();

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
        state
            .embark(&mut world, &ProductRoom::default(), fleet, &troops[..2])
            .unwrap();
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
        state
            .embark(&mut world, &ProductRoom::default(), fleet, &troops)
            .unwrap();
        let tags: Vec<u32> = world.fleets[fleet]
            .capital_ships
            .iter()
            .map(|ship| ship.tag)
            .collect();
        assert_eq!(
            troops
                .iter()
                .map(|&troop| state.carrier(troop))
                .collect::<Vec<_>>(),
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
            bombarding: false,
        });
        let mut state = TroopTransportState::new();
        state
            .embark(&mut world, &ProductRoom::default(), second, &troops[..1])
            .unwrap();
        state
            .embark(&mut world, &ProductRoom::default(), first, &troops[1..])
            .unwrap();

        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(json, serde_json::to_string(&state).unwrap());
        let decoded: TroopTransportState = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.cargo(first), state.cargo(first));
        assert_eq!(decoded.cargo(second), state.cargo(second));
    }
    // A regiment loaded by order stays aboard where it loaded: nothing lands
    // it but a Move or the Planetary Assault (`ghidra/notes/ground-assault.md`,
    // "Landing without the order"; `ghidra/notes/fleet-window.md`).
    #[test]
    fn a_loaded_regiment_stays_aboard_at_the_system_it_loaded_at() {
        let (mut world, origin, _, fleet, troops) = fixture(2);
        let mut state = TroopTransportState::new();

        state
            .load(&mut world, &ProductRoom::default(), fleet, &troops[..1])
            .unwrap();

        assert_eq!(state.cargo(fleet), &troops[..1]);
        assert_eq!(world.systems[origin].ground_units, troops[1..]);
        assert_eq!(state.carried_count(fleet), 1);
    }

    // FUN_00500b40: the room is the capacity minus the regiments aboard.
    #[test]
    fn a_full_fleet_refuses_a_regiment_and_nothing_moves() {
        let (mut world, origin, _, fleet, troops) = fixture(1);
        let mut state = TroopTransportState::new();
        state
            .load(&mut world, &ProductRoom::default(), fleet, &troops[..1])
            .unwrap();

        assert_eq!(
            state.load(&mut world, &ProductRoom::default(), fleet, &troops[1..2]),
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
            state.load(&mut world, &ProductRoom::default(), fleet, &troops[..1]),
            Err(TroopTransportError::TroopNotAtFleetSystem)
        );
        assert!(state.cargo(fleet).is_empty());
    }

    #[test]
    fn a_regiments_move_is_enabled_for_its_side_on_the_surface() {
        let (mut world, _, _, fleet, troops) = fixture(1);
        let mut state = TroopTransportState::new();

        assert!(state.regiment_move_enabled(&world, troops[0], true));
        assert!(!state.regiment_move_enabled(&world, troops[0], false));
        state
            .load(&mut world, &ProductRoom::default(), fleet, &troops[..1])
            .unwrap();
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
            bombarding: false,
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
            &ProductRoom::default(),
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
        state
            .load(&mut world, &ProductRoom::default(), fleet, &troops[..1])
            .unwrap();

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
        assert!(state.transits().is_empty());
    }

    // FUN_00556390: each command moves only its own object.
    #[test]
    fn unloading_one_regiment_leaves_the_rest_aboard() {
        let (mut world, origin, _, fleet, troops) = alliance_world(2);
        let mut state = TroopTransportState::new();
        state
            .load(&mut world, &ProductRoom::default(), fleet, &troops[..2])
            .unwrap();

        move_to(
            &mut state,
            &mut world,
            troops[0],
            RegimentTarget::System(origin),
        )
        .unwrap();

        assert_eq!(state.cargo(fleet), &troops[1..2]);
    }

    // FUN_0053d430: 1/0x28 for another side's destination unless it is an
    // existing, unpopulated system.
    #[test]
    fn a_populated_planet_of_another_side_refuses_a_regiment() {
        let (mut world, origin, _, fleet, troops) = alliance_world(2);
        let mut state = TroopTransportState::new();
        state
            .load(&mut world, &ProductRoom::default(), fleet, &troops[..1])
            .unwrap();
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
        state
            .load(&mut world, &ProductRoom::default(), full, &troops[..1])
            .unwrap();
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
    }

    // FUN_005073d0 lifts FUN_00553410's 1/0x24 for a regiment (0x204 over
    // [0x10,0x14]) when the planet is side 3 (+0x24 bits 6-7 0xc0), seeded
    // (+0x88 bit 1) and unpopulated (+0x88 bit 0 clear): the AI's Drop
    // target (FUN_004ae440). The regiment lands from its own full fleet.
    #[test]
    fn an_unpopulated_neutral_planet_takes_a_regiment_of_either_side() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, origin, _, fleet, troops) = alliance_world(1);
            set_side(&mut world, fleet, &troops, side);
            let mut state = TroopTransportState::new();
            state
                .load(&mut world, &ProductRoom::default(), fleet, &troops[..1])
                .unwrap();
            world.systems[origin].control = ControlKind::Uncontrolled;
            world.systems[origin].is_populated = false;

            let moved = move_to(
                &mut state,
                &mut world,
                troops[0],
                RegimentTarget::System(origin),
            );

            assert_eq!(
                moved,
                Ok(RegimentMove::Placed(RegimentLeg::Surface(origin))),
                "{side:?}"
            );
            assert!(state.cargo(fleet).is_empty(), "{side:?}");
            assert!(world.systems[origin].ground_units.contains(&troops[0]));
        }
    }

    // FUN_005073d0: the lift needs side 3 and no population; a populated
    // neutral planet keeps FUN_00553410's 1/0x24 (FUN_0053d430 refuses it
    // first, 1/0x28).
    #[test]
    fn a_populated_neutral_planet_refuses_a_regiment_of_either_side() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, origin, _, fleet, troops) = alliance_world(1);
            set_side(&mut world, fleet, &troops, side);
            let mut state = TroopTransportState::new();
            state
                .load(&mut world, &ProductRoom::default(), fleet, &troops[..1])
                .unwrap();
            world.systems[origin].control = ControlKind::Uncontrolled;

            let moved = move_to(
                &mut state,
                &mut world,
                troops[0],
                RegimentTarget::System(origin),
            );

            assert_eq!(
                moved,
                Err(TroopTransportError::OtherSideDestination),
                "{side:?}"
            );
            assert_eq!(state.cargo(fleet), &troops[..1], "{side:?}");
        }
    }

    // FUN_005073d0's lift is for regiments alone (local [0x10,0x14] test);
    // a squadron keeps 1/0x24 at an unpopulated neutral planet and boards
    // no ship (FUN_0053d430's non-regiment side test, 1/0x28).
    #[test]
    fn an_unpopulated_neutral_planet_refuses_a_squadron() {
        let (mut world, origin, _, fleet, _) = alliance_world(1);
        let class = world.fighter_classes.insert(Default::default());
        world.fleets[fleet]
            .fighters
            .push(crate::world::FighterEntry {
                class,
                count: 1,
                carrier: 0,
            });
        world.systems[origin].control = ControlKind::Uncontrolled;
        world.systems[origin].is_populated = false;
        let mut state = TroopTransportState::new();

        let moved = state.move_squadron(
            &mut world,
            &MovementState::new(),
            &HashSet::new(),
            &ProductRoom::default(),
            SquadronSource::Fleet { fleet, index: 0 },
            RegimentTarget::System(origin),
            10,
        );

        assert_eq!(moved, Err(TroopTransportError::OtherSideDestination));
        assert_eq!(world.fleets[fleet].fighters.len(), 1);

        // FUN_0053d430 refuses the group before FUN_00552300 builds a leg,
        // so a friendly carrier with room there takes nothing.
        let carrier = add_fleet(&mut world, origin, 0);
        let class = world.fleets[carrier].capital_ships[0].class;
        world.capital_ship_classes[class].fighter_capacity = 2;

        let moved = state.move_squadron(
            &mut world,
            &MovementState::new(),
            &HashSet::new(),
            &ProductRoom::default(),
            SquadronSource::Fleet { fleet, index: 0 },
            RegimentTarget::System(origin),
            10,
        );

        assert_eq!(moved, Err(TroopTransportError::OtherSideDestination));
        assert!(world.fleets[carrier].fighters.is_empty());
    }

    /// A side's fleet at `system` whose one ship holds `room` squadrons.
    fn carrier(world: &mut GameWorld, system: SystemKey, side: Faction, room: u32) -> FleetKey {
        let fleet = add_fleet(world, system, 0);
        set_side(world, fleet, &[], side);
        let class = world.fleets[fleet].capital_ships[0].class;
        world.capital_ship_classes[class].fighter_capacity = room;
        fleet
    }

    fn squadron_world(side: Faction) -> (GameWorld, SystemKey, FleetKey, FighterKey) {
        let (mut world, origin, _, fleet, _) = alliance_world(1);
        set_side(&mut world, fleet, &[], side);
        world.systems[origin].control = ControlKind::Controlled(side);
        let class = world.fighter_classes.insert(Default::default());
        (world, origin, fleet, class)
    }

    fn send_squadron(
        state: &mut TroopTransportState,
        world: &mut GameWorld,
        source: SquadronSource,
        target: RegimentTarget,
    ) -> Result<SquadronMove, TroopTransportError> {
        state.move_squadron(
            world,
            &MovementState::new(),
            &HashSet::new(),
            &ProductRoom::default(),
            source,
            target,
            10,
        )
    }

    // A squadron is one object, a child of its ship (FUN_005039d0); the
    // Move (FUN_00556390) takes that one object out of the fleet. port:
    // the fleet's squadrons of a class are one counted entry, so the move
    // takes one from the count and the last one takes the entry.
    #[test]
    fn a_squadron_leaves_its_fleet_one_at_a_time_and_the_last_takes_the_entry() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, origin, fleet, class) = squadron_world(side);
            world.fleets[fleet]
                .fighters
                .push(crate::world::FighterEntry {
                    class,
                    count: 2,
                    carrier: 0,
                });
            let mut state = TroopTransportState::new();
            let source = SquadronSource::Fleet { fleet, index: 0 };

            let first = send_squadron(&mut state, &mut world, source, RegimentTarget::System(origin));

            assert_eq!(first, Ok(SquadronMove::Placed(RegimentLeg::Surface(origin))), "{side:?}");
            assert_eq!(world.fleets[fleet].fighters.len(), 1, "{side:?}");
            assert_eq!(world.fleets[fleet].fighters[0].count, 1, "{side:?}");

            let second = send_squadron(&mut state, &mut world, source, RegimentTarget::System(origin));

            assert_eq!(second, Ok(SquadronMove::Placed(RegimentLeg::Surface(origin))), "{side:?}");
            assert!(world.fleets[fleet].fighters.is_empty(), "{side:?}");
            let is_alliance = side == Faction::Alliance;
            assert_eq!(
                world.systems[origin].fighters,
                vec![crate::world::SystemSquadrons { class, count: 2, is_alliance }],
                "{side:?}"
            );
        }
    }

    // port: an entry whose count is 0 holds no squadron object
    // (FUN_005039d0's children), so there is nothing to move.
    #[test]
    fn a_fleet_entry_with_no_squadrons_left_has_no_squadron_to_move() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, origin, fleet, class) = squadron_world(side);
            world.fleets[fleet]
                .fighters
                .push(crate::world::FighterEntry {
                    class,
                    count: 0,
                    carrier: 0,
                });
            let mut state = TroopTransportState::new();

            let moved = send_squadron(
                &mut state,
                &mut world,
                SquadronSource::Fleet { fleet, index: 0 },
                RegimentTarget::System(origin),
            );

            assert_eq!(moved, Err(TroopTransportError::MissingSquadron), "{side:?}");
            assert!(world.systems[origin].fighters.is_empty(), "{side:?}");
        }
    }

    // FUN_00552300 makes the squadron's own fleet the leg when it accepts
    // (FUN_00500ac0, FUN_00500b40); a fleet destination that is the
    // squadron's container changes nothing, another of the side takes it.
    #[test]
    fn a_squadron_sent_to_its_own_fleet_stays_and_to_another_moves() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, origin, _, class) = squadron_world(side);
            let fleet = carrier(&mut world, origin, side, 4);
            let other = carrier(&mut world, origin, side, 4);
            world.fleets[fleet]
                .fighters
                .push(crate::world::FighterEntry {
                    class,
                    count: 1,
                    carrier: 0,
                });
            let mut state = TroopTransportState::new();
            let source = SquadronSource::Fleet { fleet, index: 0 };

            let stay = send_squadron(&mut state, &mut world, source, RegimentTarget::Fleet(fleet));

            assert_eq!(stay, Ok(SquadronMove::Unchanged), "{side:?}");
            assert_eq!(world.fleets[fleet].fighters[0].count, 1, "{side:?}");

            let moved = send_squadron(&mut state, &mut world, source, RegimentTarget::Fleet(other));

            assert_eq!(moved, Ok(SquadronMove::Placed(RegimentLeg::Fleet(other))), "{side:?}");
            assert!(world.fleets[fleet].fighters.is_empty(), "{side:?}");
            assert_eq!(world.fleets[other].fighters.len(), 1, "{side:?}");
        }
    }

    // A system's squadron is a child of the system of its own side
    // (+0x24 bits 6-7; the Defenses window's Squadrons page lists them by
    // class and side): the Move takes one of that class and side, and no
    // other side's.
    #[test]
    fn a_system_squadron_moves_only_for_its_class_and_side() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, origin, _, class) = squadron_world(side);
            let other_class = world.fighter_classes.insert(Default::default());
            let fleet = carrier(&mut world, origin, side, 4);
            let is_alliance = side == Faction::Alliance;
            world.systems[origin].add_squadrons(class, 1, !is_alliance);
            let mut state = TroopTransportState::new();
            let system_source = |class, is_alliance| SquadronSource::System {
                system: origin,
                class,
                is_alliance,
            };

            let foreign = send_squadron(
                &mut state,
                &mut world,
                system_source(class, is_alliance),
                RegimentTarget::Fleet(fleet),
            );

            assert_eq!(foreign, Err(TroopTransportError::MissingSquadron), "{side:?}");

            world.systems[origin].add_squadrons(class, 1, is_alliance);
            let absent = send_squadron(
                &mut state,
                &mut world,
                system_source(other_class, is_alliance),
                RegimentTarget::Fleet(fleet),
            );

            assert_eq!(absent, Err(TroopTransportError::MissingSquadron), "{side:?}");

            let moved = send_squadron(
                &mut state,
                &mut world,
                system_source(class, is_alliance),
                RegimentTarget::Fleet(fleet),
            );

            assert_eq!(moved, Ok(SquadronMove::Placed(RegimentLeg::Fleet(fleet))), "{side:?}");
            assert_eq!(
                world.systems[origin].fighters,
                vec![crate::world::SystemSquadrons { class, count: 1, is_alliance: !is_alliance }],
                "{side:?}"
            );
            assert_eq!(world.fleets[fleet].fighters.len(), 1, "{side:?}");
        }
    }

    fn set_side(world: &mut GameWorld, fleet: FleetKey, troops: &[TroopKey], side: Faction) {
        let is_alliance = side == Faction::Alliance;
        world.fleets[fleet].is_alliance = is_alliance;
        for ship in &world.fleets[fleet].capital_ships {
            world.capital_ship_classes[ship.class].is_alliance = is_alliance;
        }
        for &troop in troops {
            world.troops[troop].is_alliance = is_alliance;
        }
    }

    // FUN_0053d200 lists each object's days from FUN_00555d30: a regiment's
    // trip at the default speed (FUN_004f63f0), a squadron's at its class's
    // (FUN_00502f80), none within one system. FUN_00487740 runs the move's
    // checks before FUN_00487cc0 asks, so a refusal is the move's and
    // nothing moves.
    #[test]
    fn a_units_move_days_are_its_trip_checked_without_moving_it_for_either_side() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, origin, destination, fleet, troops) = alliance_world(1);
            set_side(&mut world, fleet, &troops, side);
            for system in [origin, destination] {
                world.systems[system].control = ControlKind::Controlled(side);
            }
            let class = world.fighter_classes.insert(Default::default());
            world.fleets[fleet].fighters.push(crate::world::FighterEntry {
                class,
                count: 1,
                carrier: 0,
            });
            let state = TroopTransportState::new();
            let (movement, none, room) = (MovementState::new(), HashSet::new(), ProductRoom::default());
            let squadron = SquadronSource::Fleet { fleet, index: 0 };
            let regiment_days = |world: &GameWorld, target| {
                state.regiment_move_days(world, &movement, &none, &room, troops[0], target)
            };
            let squadron_days = |world: &GameWorld, target| {
                state.squadron_move_days(world, &movement, &none, &room, squadron, target)
            };
            let speed = fighter_speed(&world, &world.fighter_classes[class]);
            let trip = leg_ticks(&world, origin, destination, speed);

            assert_eq!(regiment_days(&world, RegimentTarget::System(destination)), Ok(20), "{side:?}");
            assert_eq!(regiment_days(&world, RegimentTarget::System(origin)), Ok(0), "{side:?}");
            assert_eq!(regiment_days(&world, RegimentTarget::Fleet(fleet)), Ok(0), "{side:?}");
            assert!(trip > 0);
            assert_eq!(squadron_days(&world, RegimentTarget::System(destination)), Ok(trip), "{side:?}");
            assert_eq!(squadron_days(&world, RegimentTarget::System(origin)), Ok(0), "{side:?}");

            let other = if side == Faction::Alliance {
                Faction::Empire
            } else {
                Faction::Alliance
            };
            world.systems[destination].control = ControlKind::Controlled(other);
            assert_eq!(
                regiment_days(&world, RegimentTarget::System(destination)),
                Err(TroopTransportError::OtherSideDestination),
                "{side:?}"
            );
            assert_eq!(
                squadron_days(&world, RegimentTarget::System(destination)),
                Err(TroopTransportError::OtherSideDestination),
                "{side:?}"
            );
            assert_eq!(regiment_system(&world, troops[0]), Some(origin), "{side:?}: nothing moved");
            assert_eq!(world.fleets[fleet].fighters[0].count, 1, "{side:?}");
            assert!(state.transits().is_empty() && world.squadron_transits.is_empty(), "{side:?}");
        }
    }

    // FUN_005073d0: 0x90/4 for a blockaded planet, which a regiment keeps
    // when no fleet of its side there has room.
    #[test]
    fn a_blockaded_planet_refuses_a_regiment_with_no_room_aboard() {
        let (mut world, origin, _, fleet, troops) = alliance_world(1);
        let mut state = TroopTransportState::new();
        state
            .load(&mut world, &ProductRoom::default(), fleet, &troops[..1])
            .unwrap();
        let blockaded = HashSet::from([origin]);

        let moved = state.move_regiment(
            &mut world,
            &MovementState::new(),
            &blockaded,
            &ProductRoom::default(),
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
            carrier: 0,
            arrival_tick: 30,
            via: None,
        };
        assert_eq!(moved, Ok(RegimentMove::Departed(transit.clone())));
        assert_eq!(state.transits(), std::slice::from_ref(&transit));
        assert_eq!(regiment_system(&world, troops[0]), None);
        assert!(state.advance_transit(&mut world, 29).arrived.is_empty());
        assert_eq!(state.advance_transit(&mut world, 30).arrived, vec![transit]);
        assert!(world.systems[destination].ground_units.contains(&troops[0]));
        assert!(state.transits().is_empty());
    }

    // FUN_00556430 moves the regiment into its new container at once, so
    // the overdraft's notice (0x7081) names the system it travels to: the
    // planet, or the system of the fleet it boards.
    #[test]
    fn a_regiment_scrapped_while_travelling_stands_at_its_destination() {
        let (mut world, _, destination, _, troops) = alliance_world(2);
        let fleet = add_fleet(&mut world, destination, 1);
        let mut state = TroopTransportState::new();
        for (troop, target) in [
            (troops[0], RegimentTarget::System(destination)),
            (troops[1], RegimentTarget::Fleet(fleet)),
        ] {
            assert!(matches!(
                move_to(&mut state, &mut world, troop, target),
                Ok(RegimentMove::Departed(_))
            ));

            assert_eq!(
                crate::scrap::target_system(
                    &world,
                    &state,
                    crate::scrap::ScrapTarget::Troop(troop)
                ),
                Some(destination)
            );
        }
    }

    // FUN_00530270 destroys the scrapped object, which sits in its new
    // container while en route (FUN_00556430): it never lands, and the
    // fleet it was boarding gets its room back. A regiment travelling with
    // it still lands.
    #[test]
    fn a_regiment_scrapped_while_travelling_never_arrives() {
        let (mut world, _, destination, _, troops) = alliance_world(3);
        let fleet = add_fleet(&mut world, destination, 1);
        let mut state = TroopTransportState::new();
        let mut stockpiles = crate::stockpiles::StockpileState::new();
        move_to(
            &mut state,
            &mut world,
            troops[0],
            RegimentTarget::System(destination),
        )
        .unwrap();
        move_to(
            &mut state,
            &mut world,
            troops[1],
            RegimentTarget::Fleet(fleet),
        )
        .unwrap();
        let Ok(RegimentMove::Departed(spared)) = move_to(
            &mut state,
            &mut world,
            troops[2],
            RegimentTarget::System(destination),
        ) else {
            panic!("the third regiment departs");
        };

        for troop in &troops[..2] {
            crate::scrap::scrap(
                &mut world,
                &mut stockpiles,
                &mut state,
                crate::scrap::ScrapTarget::Troop(*troop),
            );
        }
        let arrivals = state.advance_transit(&mut world, 30);

        assert_eq!(arrivals.arrived, vec![spared]);
        assert!(state.transits().is_empty());
        assert_eq!(state.incoming_count(fleet), 0);
        assert!(state.cargo(fleet).is_empty());
        for troop in &troops[..2] {
            assert!(!world.troops.contains_key(*troop));
            assert!(!world.systems[destination].ground_units.contains(troop));
        }
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
        state
            .load(&mut world, &ProductRoom::default(), fleet, &troops[..1])
            .unwrap();
        let mut movement = MovementState::new();
        movement.order(fleet, origin, destination, 5);

        let moved = state.move_regiment(
            &mut world,
            &movement,
            &HashSet::new(),
            &ProductRoom::default(),
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
            &ProductRoom::default(),
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

        // A merge (`crate::fleet_join`) moves the ships, then the cargo.
        let ships = std::mem::take(&mut world.fleets[first].capital_ships);
        world.fleets[second].capital_ships.extend(ships);
        world.fleets.remove(first);
        state.transfer_fleet(&mut world, first, second);
        state.advance_transit(&mut world, 30);

        assert_eq!(state.cargo(second), &troops[..1]);
    }

    // FUN_004f6fd0 leaves an object already in the chosen container alone.
    #[test]
    fn moving_a_regiment_onto_the_fleet_it_is_aboard_changes_nothing() {
        let (mut world, _, _, fleet, troops) = alliance_world(2);
        let mut state = TroopTransportState::new();
        state
            .load(&mut world, &ProductRoom::default(), fleet, &troops[..1])
            .unwrap();

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
    fn a_regiment_boarding_from_its_planet_goes_aboard() {
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
        assert_eq!(state.cargo(fleet), &troops[..1]);
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

    /// Two systems of `side`, 20 default-speed days apart, its regiments on
    /// the first and a squadron class.
    fn side_world(side: Faction) -> (GameWorld, SystemKey, SystemKey, Vec<TroopKey>, FighterKey) {
        let (mut world, origin, destination, fleet, troops) = alliance_world(1);
        set_side(&mut world, fleet, &troops, side);
        for system in [origin, destination] {
            world.systems[system].control = ControlKind::Controlled(side);
        }
        let class = world.fighter_classes.insert(Default::default());
        (world, origin, destination, troops, class)
    }

    /// A fleet of `side` at `system` whose ships hold `rooms` squadrons and
    /// as many regiments each.
    fn ships_fleet(world: &mut GameWorld, system: SystemKey, side: Faction, rooms: &[u32]) -> FleetKey {
        let is_alliance = side == Faction::Alliance;
        let capital_ships = rooms
            .iter()
            .map(|&room| {
                let class = world.capital_ship_classes.insert(CapitalShipClass {
                    is_alliance,
                    fighter_capacity: room,
                    troop_capacity: room,
                    hull: 100,
                    ..CapitalShipClass::default()
                });
                ShipInstance::new(class, 100, is_alliance)
            })
            .collect();
        let fleet = world.fleets.insert(Fleet {
            location: system,
            capital_ships,
            is_alliance,
            ..Fleet::default()
        });
        world.systems[system].fleets.push(fleet);
        fleet
    }

    /// Move `fleet`'s ship at `index` into a new fleet at the same system.
    fn split_ship(world: &mut GameWorld, fleet: FleetKey, index: usize) -> FleetKey {
        let ship = world.fleets[fleet].capital_ships.remove(index);
        let value = &world.fleets[fleet];
        let split = world.fleets.insert(Fleet {
            location: value.location,
            capital_ships: vec![ship],
            is_alliance: value.is_alliance,
            ..Fleet::default()
        });
        let location = world.fleets[split].location;
        world.systems[location].fleets.push(split);
        split
    }

    fn tags(world: &GameWorld, fleet: FleetKey) -> Vec<u32> {
        world.fleets[fleet].capital_ships.iter().map(|ship| ship.tag).collect()
    }

    /// `fleet`'s squadrons as (class, count, carrier tag).
    fn aboard(world: &GameWorld, fleet: FleetKey) -> Vec<(FighterKey, u32, u32)> {
        world.fleets[fleet]
            .fighters
            .iter()
            .map(|entry| (entry.class, entry.count, entry.carrier))
            .collect()
    }

    // FUN_00552300 picks the ship when the Move is ordered and FUN_00556390
    // reparents the squadron into it then; FUN_00556620 makes no new choice
    // on arrival, so the squadron arrives aboard that ship wherever it is.
    #[test]
    fn a_squadron_arrives_aboard_the_ship_it_was_ordered_onto() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, origin, destination, _, class) = side_world(side);
            let is_alliance = side == Faction::Alliance;
            let target = ships_fleet(&mut world, destination, side, &[2, 1]);
            world.systems[origin].add_squadrons(class, 1, is_alliance);
            let mut state = TroopTransportState::new();

            let moved = send_squadron(
                &mut state,
                &mut world,
                SquadronSource::System { system: origin, class, is_alliance },
                RegimentTarget::Fleet(target),
            );
            assert!(matches!(moved, Ok(SquadronMove::Departed(_))), "{side:?}");
            let split = split_ship(&mut world, target, 0);
            state.advance_transit(&mut world, 1000);

            assert!(world.fleets[target].fighters.is_empty(), "{side:?}");
            assert_eq!(aboard(&world, split), [(class, 1, tags(&world, split)[0])], "{side:?}");
        }
    }

    // FUN_00500b40 in mode 3 counts every created child of the ship, and a
    // squadron on its way is one (FUN_00556390), so it holds its ship's room
    // against the next order; FUN_00552b10 keeps the most room, the earlier
    // ship on a tie.
    #[test]
    fn a_squadron_on_its_way_to_a_ship_holds_its_room_against_the_next_order() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, origin, destination, _, class) = side_world(side);
            let is_alliance = side == Faction::Alliance;
            let target = ships_fleet(&mut world, origin, side, &[2, 3]);
            world.systems[destination].add_squadrons(class, 1, is_alliance);
            world.systems[origin].add_squadrons(class, 1, is_alliance);
            let mut state = TroopTransportState::new();

            let far = send_squadron(
                &mut state,
                &mut world,
                SquadronSource::System { system: destination, class, is_alliance },
                RegimentTarget::Fleet(target),
            );
            let near = send_squadron(
                &mut state,
                &mut world,
                SquadronSource::System { system: origin, class, is_alliance },
                RegimentTarget::Fleet(target),
            );

            assert!(matches!(far, Ok(SquadronMove::Departed(_))), "{side:?}");
            assert_eq!(near, Ok(SquadronMove::Placed(RegimentLeg::Fleet(target))), "{side:?}");
            assert_eq!(aboard(&world, target), [(class, 1, tags(&world, target)[0])], "{side:?}");
        }
    }

    // hyp: a squadron whose ship is lost before it arrives is lost with it,
    // as a squadron aboard a lost ship is (`crate::carriage::drop_lost_squadrons`);
    // the original's fate for it is untraced (`ghidra/notes/fleet-join-split.md`).
    #[test]
    fn a_squadron_whose_ship_is_lost_on_the_way_is_lost_on_arrival() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, origin, destination, _, class) = side_world(side);
            let is_alliance = side == Faction::Alliance;
            let target = ships_fleet(&mut world, destination, side, &[1, 1]);
            world.systems[origin].add_squadrons(class, 1, is_alliance);
            let mut state = TroopTransportState::new();
            send_squadron(
                &mut state,
                &mut world,
                SquadronSource::System { system: origin, class, is_alliance },
                RegimentTarget::Fleet(target),
            )
            .unwrap();
            world.fleets[target].capital_ships[0].alive = false;

            let ended = state.advance_transit(&mut world, 1000);

            assert_eq!(ended.squadrons_lost.len(), 1, "{side:?}");
            assert!(world.fleets[target].fighters.is_empty(), "{side:?}");
        }
    }

    // As a squadron: the regiment is reparented into the ship FUN_00552300
    // picks at the order (FUN_00556390) and arrives aboard it.
    #[test]
    fn a_regiment_arrives_aboard_the_ship_it_was_ordered_onto() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, _, destination, troops, _) = side_world(side);
            let target = ships_fleet(&mut world, destination, side, &[2, 1]);
            let mut state = TroopTransportState::new();

            let moved = move_to(&mut state, &mut world, troops[0], RegimentTarget::Fleet(target));
            assert!(matches!(moved, Ok(RegimentMove::Departed(_))), "{side:?}");
            let split = split_ship(&mut world, target, 0);
            state.advance_transit(&mut world, 30);

            assert!(state.cargo(target).is_empty(), "{side:?}");
            assert_eq!(state.cargo(split), &troops[..1], "{side:?}");
            assert_eq!(state.carrier(troops[0]), tags(&world, split)[0], "{side:?}");
        }
    }

    // FUN_00500b40 (mode 3): a regiment on its way to a ship holds that
    // ship's room against the next order; FUN_00552b10 breaks the tie to the
    // earlier ship.
    #[test]
    fn a_regiment_on_its_way_to_a_ship_holds_its_room_against_the_next_order() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, origin, destination, troops, _) = side_world(side);
            let target = ships_fleet(&mut world, origin, side, &[2, 3]);
            world.systems[origin].ground_units.retain(|&key| key != troops[0]);
            world.systems[destination].ground_units.push(troops[0]);
            let mut state = TroopTransportState::new();

            let far = move_to(&mut state, &mut world, troops[0], RegimentTarget::Fleet(target));
            let near = move_to(&mut state, &mut world, troops[1], RegimentTarget::Fleet(target));

            assert!(matches!(far, Ok(RegimentMove::Departed(_))), "{side:?}");
            assert_eq!(
                near,
                Ok(RegimentMove::Placed(RegimentLeg::Fleet(target))),
                "{side:?}"
            );
            assert_eq!(
                state.carrier(troops[1]),
                tags(&world, target)[0],
                "{side:?}"
            );
        }
    }

    // A load is the Move within one system (FUN_00552300), so a regiment on
    // its way to the fleet's only ship leaves no room for it (1/0x27).
    #[test]
    fn a_regiment_on_its_way_to_a_ship_holds_its_room_against_a_load() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, origin, destination, troops, _) = side_world(side);
            let target = ships_fleet(&mut world, origin, side, &[1]);
            world.systems[origin].ground_units.retain(|&key| key != troops[0]);
            world.systems[destination].ground_units.push(troops[0]);
            let mut state = TroopTransportState::new();
            move_to(&mut state, &mut world, troops[0], RegimentTarget::Fleet(target)).unwrap();

            let loaded = state.load(&mut world, &ProductRoom::default(), target, &troops[1..2]);

            assert_eq!(
                loaded,
                Err(TroopTransportError::CapacityExceeded { capacity: 1, requested: 2 }),
                "{side:?}"
            );
            assert!(state.cargo(target).is_empty(), "{side:?}");
            assert!(world.systems[origin].ground_units.contains(&troops[1]), "{side:?}");
        }
    }

    // FUN_00552300.c:108,147 seats a load on the fleet's ships in mode 1
    // (FUN_00502e30(fleet, 1)), so a ship on its own leg from its yard (bit
    // 5, FUN_00556430) gives no room (0x004f7b80), and the refusal's figures
    // count neither its capacity nor the products built for it.
    #[test]
    fn a_refused_load_counts_no_ship_on_its_own_leg_for_either_side() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, origin, _, troops, _) = side_world(side);
            let target = ships_fleet(&mut world, origin, side, &[1, 3]);
            let travelling = &mut world.fleets[target].capital_ships[1];
            travelling.tag = 5;
            travelling.transit = Some(crate::world::ShipTransit {
                from: origin,
                to: origin,
                arrival_tick: 9,
            });
            let mut products = ProductRoom::default();
            products.add(
                crate::manufacturing::BuildableKind::Troop(world.troops[troops[0]].class_dat_id),
                5,
            );
            let mut state = TroopTransportState::new();

            let loaded = state.load(&mut world, &products, target, &troops[..2]);

            assert_eq!(
                loaded,
                Err(TroopTransportError::CapacityExceeded { capacity: 1, requested: 2 }),
                "{side:?}"
            );
        }
    }

    // hyp: as a squadron, a regiment whose ship is lost before it arrives is
    // lost with it; the original's fate for it is untraced
    // (`ghidra/notes/fleet-join-split.md`).
    #[test]
    fn a_regiment_whose_ship_is_lost_on_the_way_is_lost_on_arrival() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, _, destination, troops, _) = side_world(side);
            let target = ships_fleet(&mut world, destination, side, &[1, 1]);
            let mut state = TroopTransportState::new();
            move_to(&mut state, &mut world, troops[0], RegimentTarget::Fleet(target)).unwrap();
            world.fleets[target].capital_ships[0].alive = false;

            let ended = state.advance_transit(&mut world, 30);

            assert_eq!(ended.lost.len(), 1, "{side:?}");
            assert!(state.cargo(target).is_empty(), "{side:?}");
            assert!(!world.troops.contains_key(troops[0]), "{side:?}");
        }
    }

    // FUN_005073d0 refuses a blockaded planet, and FUN_00552300 then offers
    // the side's fleets there in order: a full one gives 1/0x27
    // (FUN_00500b40), so the next with room takes the squadron.
    #[test]
    fn a_squadron_refused_by_its_planet_boards_the_first_fleet_with_room() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, origin, _, _, class) = side_world(side);
            let is_alliance = side == Faction::Alliance;
            let full = world.systems[origin].fleets[0];
            let roomy = ships_fleet(&mut world, origin, side, &[1]);
            world.systems[origin].add_squadrons(class, 1, is_alliance);
            let mut state = TroopTransportState::new();

            let moved = state.move_squadron(
                &mut world,
                &MovementState::new(),
                &HashSet::from([origin]),
                &ProductRoom::default(),
                SquadronSource::System { system: origin, class, is_alliance },
                RegimentTarget::System(origin),
                10,
            );

            assert_eq!(moved, Ok(SquadronMove::Placed(RegimentLeg::Fleet(roomy))), "{side:?}");
            assert!(world.fleets[full].fighters.is_empty(), "{side:?}");
            assert_eq!(aboard(&world, roomy), [(class, 1, tags(&world, roomy)[0])], "{side:?}");
        }
    }

    // FUN_00556430: the arrival day is the order's day plus the trip at the
    // squadron's speed (FUN_00502f80); FUN_004fc080 loses one whose planet
    // is destroyed on arrival.
    #[test]
    fn a_squadron_lands_on_its_day_and_is_lost_at_a_destroyed_planet() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, origin, destination, _, class) = side_world(side);
            let is_alliance = side == Faction::Alliance;
            world.systems[origin].add_squadrons(class, 2, is_alliance);
            let speed = fighter_speed(&world, &world.fighter_classes[class]);
            let day = 10 + u64::from(transit_ticks_between(&world, (0, 0), (100, 0), speed));
            let source = SquadronSource::System { system: origin, class, is_alliance };
            let mut state = TroopTransportState::new();

            let moved = send_squadron(&mut state, &mut world, source, RegimentTarget::System(destination));

            let Ok(SquadronMove::Departed(transit)) = moved else {
                panic!("{side:?}: the squadron departs");
            };
            assert_eq!(transit.arrival_tick, day, "{side:?}");
            assert!(state.advance_transit(&mut world, day - 1).squadrons_arrived.is_empty());
            assert_eq!(state.advance_transit(&mut world, day).squadrons_arrived.len(), 1);
            assert_eq!(world.systems[destination].fighters.len(), 1, "{side:?}");

            send_squadron(&mut state, &mut world, source, RegimentTarget::System(destination))
                .unwrap();
            world.systems[destination].is_destroyed = true;
            let ended = state.advance_transit(&mut world, day + 10);

            assert_eq!(ended.squadrons_lost.len(), 1, "{side:?}");
            assert_eq!(world.systems[destination].fighters[0].count, 1, "{side:?}");
        }
    }

    // embark refuses a selection naming one regiment twice, and then
    // nothing moves.
    #[test]
    fn a_load_naming_one_regiment_twice_is_refused() {
        let (mut world, origin, _, fleet, troops) = fixture(3);
        let mut state = TroopTransportState::new();

        let loaded = state.load(
            &mut world,
            &ProductRoom::default(),
            fleet,
            &[troops[0], troops[0]],
        );

        assert_eq!(loaded, Err(TroopTransportError::AlreadyEmbarked));
        assert!(state.cargo(fleet).is_empty());
        assert!(world.systems[origin].ground_units.contains(&troops[0]));
    }

    /// The products queued at `system` for `fleet`: one of `kind` aboard
    /// the ship tagged `carrier`.
    fn queued_product(
        system: SystemKey,
        fleet: FleetKey,
        kind: crate::manufacturing::BuildableKind,
        carrier: u32,
    ) -> ProductRoom {
        let mut manufacturing = crate::manufacturing::ManufacturingState::new();
        let mut item = crate::manufacturing::QueueItem::new(kind, 5)
            .delivered_to(crate::manufacturing::Destination::Fleet(fleet));
        item.carrier = carrier;
        manufacturing
            .queue_mut(system, crate::manufacturing::ProductionArea::of(kind))
            .enqueue(item);
        ProductRoom::new(&manufacturing, &crate::delivery::DeliveryState::new())
    }

    // A Build bound for a fleet creates its unit aboard the ship its leg
    // chose (FUN_005385f0 -> FUN_00551a20 -> FUN_00552300; FUN_0052a140 ->
    // FUN_004f7d50), and FUN_00500b40's mode 3 counts every created child:
    // a squadron or regiment queued for the fleet's only ship leaves no
    // room, so a Move or a load onto it is refused with 1/0x27.
    #[test]
    fn a_move_onto_the_ship_a_queued_product_fills_is_refused() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, origin, _, troops, class) = side_world(side);
            let is_alliance = side == Faction::Alliance;
            let target = ships_fleet(&mut world, origin, side, &[1]);
            let tag = crate::carriage::tag_ship(&mut world, target, 0).unwrap();
            world.systems[origin].add_squadrons(class, 1, is_alliance);
            let squadrons = queued_product(
                origin,
                target,
                crate::manufacturing::BuildableKind::Fighter(class),
                tag,
            );
            let regiment =
                crate::manufacturing::BuildableKind::Troop(crate::ids::DatId::new(0x1000_0001));
            let regiments = queued_product(origin, target, regiment, tag);
            let mut state = TroopTransportState::new();

            let squadron = state.move_squadron(
                &mut world,
                &MovementState::new(),
                &HashSet::new(),
                &squadrons,
                SquadronSource::System { system: origin, class, is_alliance },
                RegimentTarget::Fleet(target),
                10,
            );
            let moved = state.move_regiment(
                &mut world,
                &MovementState::new(),
                &HashSet::new(),
                &regiments,
                troops[1],
                RegimentTarget::Fleet(target),
                10,
            );
            let loaded = state.load(&mut world, &regiments, target, &troops[1..2]);

            let full = TroopTransportError::CapacityExceeded { capacity: 1, requested: 2 };
            assert_eq!(squadron, Err(TroopTransportError::NoSquadronRoom), "{side:?}");
            assert_eq!((moved, loaded), (Err(full), Err(full)), "{side:?}");
            assert!(world.fleets[target].fighters.is_empty(), "{side:?}");
            assert!(state.cargo(target).is_empty(), "{side:?}");
            assert!(world.systems[origin].ground_units.contains(&troops[1]), "{side:?}");
            // The same orders with nothing queued board the ship.
            assert!(
                state
                    .load(&mut world, &ProductRoom::default(), target, &troops[1..2])
                    .is_ok(),
                "{side:?}"
            );
        }
    }
}
