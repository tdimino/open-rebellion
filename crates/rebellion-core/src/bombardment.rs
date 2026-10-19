//! Orbital bombardment: the four Planetary Bombardment orders `0x220`-`0x223`
//! (`ghidra/notes/bombardment.md`).
//!
//! An order flags each ordered fleet (`FUN_004fec00`, fleet `+0x58` bit 6)
//! and runs message `0x224` for each system holding them (`FUN_005637c0` ->
//! `FUN_0058d800`) in the step that issued it. The run is the ship phase
//! (`FUN_0058d990`), the defence phase (`FUN_0058dc10`), the effect phase
//! (`FUN_0058df60`) and the finalize (`FUN_0058e320`). Destroy System's
//! cascade and the character roll after it live with the integrator, which
//! holds the movement, production and blockade states the cascade reaches.

use serde::{Deserialize, Serialize};

use crate::dat::Faction;
use crate::ids::{CharacterKey, FleetKey, SectorKey, SystemKey, TroopKey};
use crate::mission_detection::MissionRng;
use crate::missions::MissionTarget;
use crate::officer_rank::CommandKind;
use crate::operation_report::ReportUnit;
use crate::uprising::Draws;
use crate::world::{FacilityRef, GameWorld, ShipInstance};

/// GNPRTB `0x600` (40, `FUN_0055e930`): the defending General's leadership
/// divisor in the defence phase.
const GNPRTB_DEFENCE_LEADERSHIP: u16 = 0x600;
/// GNPRTB `0x601` (40, `FUN_0055e950`): the Admiral's leadership divisor in
/// the hit count.
const GNPRTB_HIT_LEADERSHIP: u16 = 0x601;
/// GNPRTB `0x602` (1, `FUN_0055e970`): the low bound of a hit's draw.
const GNPRTB_HIT_DRAW_LOW: u16 = 0x602;
/// GNPRTB `0x603` (10, `FUN_0055e970`): its high bound.
const GNPRTB_HIT_DRAW_HIGH: u16 = 0x603;
/// GNPRTB `0x604` (9, `DAT_006bb760`): the energy target's value.
const GNPRTB_ENERGY_VALUE: u16 = 0x604;
/// GNPRTB `0x605` (9, `DAT_006bb770`): the raw-material target's value.
const GNPRTB_RAW_VALUE: u16 = 0x605;
/// GNPRTB `0x606` (50, `FUN_0055e9e0`): the chance a minor character at a
/// destroyed system dies.
const GNPRTB_CHARACTER_CHANCE: u16 = 0x606;

/// The KDY-150's family (`0x22`, `FUN_00527550`), the first to fire.
const KDY_FAMILY: u8 = 0x22;
/// The LNR Series I and II's family (`0x23`, `FUN_005272e0`).
const LNR_FAMILY: u8 = 0x23;
/// The shield generators whose defence subtracts from the hits (`0x24`,
/// `FUN_00527940`); the Death Star shield `0x25` does not count.
const SHIELD_FAMILY: u8 = 0x24;

/// The four orders (STRATEGY RCDATA, `ai-order-emission.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BombardKind {
    /// `0x220` Target Military Facilities.
    Military,
    /// `0x221` Target Civilian Facilities.
    Civilian,
    /// `0x222` General Bombardment.
    General,
    /// `0x223` Destroy System.
    DestroySystem,
}

impl BombardKind {
    /// The order's id (STRATEGY RCDATA; `FUN_0058d800` reads it from the
    /// command's order data).
    #[must_use]
    pub const fn order(self) -> u16 {
        match self {
            Self::Military => 0x220,
            Self::Civilian => 0x221,
            Self::General => 0x222,
            Self::DestroySystem => 0x223,
        }
    }

    /// `FUN_0058d800`'s flags: `+0x34` military targets.
    const fn military(self) -> bool {
        !matches!(self, Self::Civilian)
    }

    /// `+0x38` civilian targets.
    const fn civilian(self) -> bool {
        !matches!(self, Self::Military)
    }

    /// `+0x3c` energy and raw material.
    const fn resources(self) -> bool {
        matches!(self, Self::General | Self::DestroySystem)
    }

    /// `+0x40` collateral: Target Military only.
    const fn collateral(self) -> bool {
        matches!(self, Self::Military)
    }
}

/// Why an order cannot be issued (the enable slot `FUN_00581e80`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BombardRefusal {
    /// `8`/`6`: no fleet of the team has bombard strength (`FUN_00582440`).
    NoStrength,
    /// `8`/`8`: Destroy System and the last fleet examined holds no ship of
    /// family `0x18`-`0x1b`.
    NoDeathStar,
    /// `1`/`0x14`: the team's systems have different holders
    /// (`FUN_00553770`).
    MixedHolders,
    /// `1`/`0x22`: the system is destroyed (`FUN_00507f10`).
    Destroyed,
    /// `1`/`0x16`: the order has no fleet to command (`FUN_00553770`).
    NoTeam,
    /// `1`/`1`: a fleet command's `+0x18` refuses another side's fleet or
    /// one en route (`FUN_004ffa70` runs `FUN_004f9860` first).
    FleetRefused,
    /// `8`/`1`: a fleet command's `+0x18` finds no ship in its fleet
    /// (`FUN_004ffa70.c:15-60`, [`ships_refusal`]).
    NoShip,
    /// `1`/`0x10`: no ship of the fleet is completed and not en route, and
    /// the last one walked is not completed (`FUN_004ffa70.c:15-60`).
    LastShipUnfinished,
    /// `1`/`0x11`: no ship of the fleet is completed and not en route, and
    /// the last one walked is en route (`FUN_004ffa70.c:15-60`).
    LastShipEnRoute,
}

impl BombardRefusal {
    /// The two-word status the order carries: `FUN_00582440` (`8`/`6`,
    /// `8`/`8`), `FUN_00553770` (`1`/`0x14`, `1`/`0x16`), `FUN_00507f10`
    /// (`1`/`0x22`), `FUN_004f9860` through `FUN_004ffa70` (`1`/`1`) and
    /// `FUN_004ffa70.c:15-60` (`8`/`1`, `1`/`0x10`, `1`/`0x11`).
    #[must_use]
    pub const fn status(self) -> (u32, u32) {
        match self {
            Self::NoStrength => (8, 6),
            Self::NoDeathStar => (8, 8),
            Self::MixedHolders => (1, 0x14),
            Self::Destroyed => (1, 0x22),
            Self::NoTeam => (1, 0x16),
            Self::FleetRefused => (1, 1),
            Self::NoShip => (8, 1),
            Self::LastShipUnfinished => (1, 0x10),
            Self::LastShipEnRoute => (1, 0x11),
        }
    }
}

/// Why `FUN_004ffa70`'s walk of a fleet's capital ships refuses it: no
/// ship at all, or the state of the last ship it examined.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShipsRefusal {
    /// The fleet holds no ship (the status the walk starts with).
    NoShip,
    /// The last ship walked is not completed (`+0x50` bit 2).
    Unfinished,
    /// The last ship walked is completed but en route (bit 4).
    EnRoute,
}

impl From<ShipsRefusal> for BombardRefusal {
    fn from(refusal: ShipsRefusal) -> Self {
        match refusal {
            ShipsRefusal::NoShip => Self::NoShip,
            ShipsRefusal::Unfinished => Self::LastShipUnfinished,
            ShipsRefusal::EnRoute => Self::LastShipEnRoute,
        }
    }
}

/// `FUN_004ffa70.c:15-60`: walk the fleet's existing capital ships (mode 3,
/// `FUN_00502db0`, in container order) until one is completed and not en
/// route, which passes the fleet; otherwise the last ship's status stands:
/// uncompleted `1`/`0x10`, en route `1`/`0x11`, and none `8`/`1`. A ship's
/// en route is its own leg here; a fleet in hyperspace is refused before
/// the walk (`FUN_004f9860`).
#[must_use]
pub fn ships_refusal(fleet: &crate::world::Fleet) -> Option<ShipsRefusal> {
    let mut last = ShipsRefusal::NoShip;
    for ship in fleet.capital_ships.iter().filter(|ship| ship.alive) {
        if !ship.completed {
            last = ShipsRefusal::Unfinished;
        } else if ship.transit.is_some() {
            last = ShipsRefusal::EnRoute;
        } else {
            return None;
        }
    }
    Some(last)
}

/// A system's holder (system `+0x24` bits 6-7): its controlling faction,
/// else neutral.
#[must_use]
pub fn holder(world: &GameWorld, system: SystemKey) -> Faction {
    world
        .systems
        .get(system)
        .and_then(|sys| sys.control.faction())
        .unwrap_or(Faction::Neutral)
}

const fn side_of(is_alliance: bool) -> Faction {
    if is_alliance {
        Faction::Alliance
    } else {
        Faction::Empire
    }
}

/// Slot `+0x1dc` of a capital ship (`FUN_00501470`): its class's CAPSHPSD
/// `bombardment_modifier` less the share its hull damage costs
/// (`FUN_00500f00`, `FUN_0053e170`), or 0 once KDY fire has overflowed its
/// shields in this run (`kdy_overflow`, the first accumulator). Only a
/// usable ship adds it: `FUN_004fd6b0` walks the fleet's ships in mode 1
/// (`FUN_004fd6b0.c:23`, `FUN_00502db0(.., 1)`), so an unfinished ship or
/// one on its own leg adds nothing.
fn ship_strength(world: &GameWorld, ship: &ShipInstance, kdy_overflow: i32) -> i32 {
    let Some(class) = world.capital_ship_classes.get(ship.class) else {
        return 0;
    };
    if !ship.usable() || kdy_overflow != 0 {
        return 0;
    }
    let modifier = class.bombardment_modifier.cast_signed();
    let max_hull = class.hull.cast_signed();
    if max_hull <= 0 {
        return modifier;
    }
    let damage = (max_hull - ship.hull_current).clamp(0, max_hull);
    modifier - modifier * damage / max_hull
}

/// `FUN_004fd6b0`: a fleet's bombard strength, slot `+0x1dc` summed over its
/// live ships and squadrons. A squadron's is FIGHTSD `bombardment_defense`
/// (`FUN_005032a0`); port: the strategic fleet holds no squadron losses, so
/// each counts its full value.
#[must_use]
pub fn fleet_strength(world: &GameWorld, fleet: FleetKey) -> i32 {
    fleet_strength_with(world, fleet, &|_| 0)
}

fn fleet_strength_with(world: &GameWorld, fleet: FleetKey, overflow: &dyn Fn(usize) -> i32) -> i32 {
    let Some(value) = world.fleets.get(fleet) else {
        return 0;
    };
    let ships: i32 = value
        .capital_ships
        .iter()
        .enumerate()
        .map(|(index, ship)| ship_strength(world, ship, overflow(index)))
        .sum();
    // FUN_004fd6b0 walks only the fleet's active ships (FUN_00502db0) and
    // each one's squadrons (FUN_005039d0): a carrier the batteries destroyed
    // adds nothing for its squadrons.
    let squadrons: i32 = value
        .fighters
        .iter()
        .filter(|entry| crate::carriage::carried_by_living_ship(value, entry.carrier))
        .map(|entry| {
            world.fighter_classes.get(entry.class).map_or(0, |class| {
                class.bombardment_defense.cast_signed() * entry.count.cast_signed()
            })
        })
        .sum();
    ships + squadrons
}

/// The order's enable test for a team of fleets (`FUN_00581e80`): one holder
/// across the team's systems (`FUN_00553770`), then the gate
/// `FUN_00582440`. One fleet passing is enough; a fleet passes with a
/// nonzero strength and, for Destroy System, a Death Star ship. Nothing tests
/// the side, a shield, ion damage or the target's owner.
///
/// # Errors
/// The refusal the original reports.
pub fn gate(
    world: &GameWorld,
    fleets: &[FleetKey],
    kind: BombardKind,
) -> Result<(), BombardRefusal> {
    let mut holders = fleets
        .iter()
        .filter_map(|&fleet| world.fleets.get(fleet))
        .map(|value| holder(world, value.location));
    if let Some(first) = holders.next() {
        if holders.any(|other| other != first) {
            return Err(BombardRefusal::MixedHolders);
        }
    }
    // The 0x224 command refuses a destroyed system (1/0x22, FUN_00507f10).
    if fleets
        .iter()
        .filter_map(|&fleet| world.fleets.get(fleet))
        .any(|value| {
            world
                .systems
                .get(value.location)
                .is_some_and(|sys| sys.is_destroyed)
        })
    {
        return Err(BombardRefusal::Destroyed);
    }
    let mut last_held_death_star = false;
    for &fleet in fleets {
        if !world.fleets.contains_key(fleet) {
            continue;
        }
        let strength = fleet_strength(world, fleet);
        // A usable Death Star (`FUN_005003d0(fleet, 1)`, `FUN_00582440.c:70`).
        last_held_death_star = world.fleet_has_usable_death_star(fleet);
        if strength != 0 && (kind != BombardKind::DestroySystem || last_held_death_star) {
            return Ok(());
        }
    }
    if kind == BombardKind::DestroySystem && !last_held_death_star {
        return Err(BombardRefusal::NoDeathStar);
    }
    Err(BombardRefusal::NoStrength)
}

/// The enable slot the object menu asks of each order (`FUN_00581e80`): the
/// order base's own enable (`FUN_0051fe20`) builds one fleet command per team
/// fleet and runs each one's `+0x18` (`FUN_00553770` -> `FUN_0057fe90` ->
/// fleet slot `+0x1c4`, `FUN_004fdf70` -> `FUN_004ffa70`), then [`gate`].
/// A fleet command refuses a fleet of another side or one en route
/// (`FUN_004f9860`), then one with no completed ship that is not en route
/// ([`ships_refusal`]).
///
/// # Errors
/// The first refusal, in the original's order.
pub fn order_enabled(
    world: &GameWorld,
    movement: &crate::movement::MovementState,
    fleets: &[FleetKey],
    kind: BombardKind,
    side: Faction,
) -> Result<(), BombardRefusal> {
    if fleets.is_empty() {
        return Err(BombardRefusal::NoTeam);
    }
    for &fleet in fleets {
        let Some(value) = world
            .fleets
            .get(fleet)
            .filter(|value| side_of(value.is_alliance) == side && !movement.is_in_transit(fleet))
        else {
            return Err(BombardRefusal::FleetRefused);
        };
        if let Some(refusal) = ships_refusal(value) {
            return Err(refusal.into());
        }
    }
    gate(world, fleets, kind)
}

/// The order's fleet command (`FUN_00580030` -> fleet slot `+0x1d4`,
/// `FUN_004fec00` -> `+0x1c8` `FUN_004fe020` -> `+0x1c4` `FUN_004fdf70` ->
/// `FUN_004ffa70`): a fleet en route is refused (`FUN_004f9860`, `1`/`1`),
/// and so is one with no completed ship that is not en route
/// ([`ships_refusal`]); else it is flagged to bombard. Returns whether it
/// was.
pub fn flag_fleet(world: &mut GameWorld, fleet: FleetKey, in_transit: bool) -> bool {
    if in_transit {
        return false;
    }
    let Some(value) = world.fleets.get_mut(fleet) else {
        return false;
    };
    if ships_refusal(value).is_some() {
        return false;
    }
    value.bombarding = true;
    true
}

/// One target of the effect phase (`FUN_0058cba0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BombardTarget {
    Regiment(TroopKey),
    Facility(FacilityRef),
    /// The Alliance HQ (family `0x20`).
    Headquarters,
    Energy,
    RawMaterial,
}

/// What killed what in a run, for the finalize, the messages and telemetry.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StrikeReport {
    pub system: Option<SystemKey>,
    pub kind: Option<BombardKind>,
    /// The bombarding side.
    pub bombarder: Option<Faction>,
    /// The system's holder when the run began.
    pub holder: Option<Faction>,
    /// The hits left after the shield generators.
    pub hits: i32,
    /// Regiments destroyed, with their side (the blockade loss counters,
    /// `FUN_00508c60`).
    pub regiments: Vec<(TroopKey, Faction)>,
    pub facilities: Vec<FacilityRef>,
    pub headquarters: bool,
    pub energy: u32,
    pub raw_material: u32,
    /// Defence fire: damage per bombarding ship hit, and the ships destroyed.
    pub defence_damage: i32,
    pub ships_destroyed: u32,
    /// The Death Star destroyed the system.
    pub planet_destroyed: bool,
    /// Minor characters the ship phase's roll killed.
    pub characters: Vec<CharacterKey>,
    /// The finalize's flags.
    pub run: BombardmentRun,
    /// The destroyed regiments and ships with their reason byte, for the
    /// blockade loss counters (`FUN_00508c60`).
    pub losses: Vec<crate::blockade::UnitLoss>,
    /// What each destroyed object's event `0x302` adds to the Operation
    /// Report (`FUN_00497380`): an object whose container is the system to
    /// `+0x80`, one aboard a ship or fleet to `+0x88`.
    pub system_destroyed: Vec<ReportUnit>,
    pub fleet_destroyed: Vec<ReportUnit>,
}

/// How a `0x224` run opens (`FUN_0058d800` and the ship phase's test).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opening {
    /// No flagged fleet at the system, or two sides flagged: nothing runs.
    Idle,
    /// Destroy System with a Death Star ship among the bombarding fleets.
    Kill { bombarder: Faction },
    /// The defence and effect phases run.
    Run {
        bombarder: Faction,
        fleets: Vec<FleetKey>,
    },
}

/// `FUN_0058d800` and the ship phase's Death Star test (`FUN_0058d440`,
/// `FUN_0058ca70`): the bombarder is the side shared by every usable fleet
/// at the system flagged to bombard (`FUN_0058d800.c:55`, mode 1), and a
/// Kill needs a usable Death Star among them (`FUN_0058ca70.c:25-33` walks
/// the usable fleets' usable ships), as the gate does (`FUN_00582440.c:70`,
/// `FUN_005003d0(.., 1)`).
#[must_use]
pub fn open(world: &GameWorld, system: SystemKey, kind: BombardKind) -> Opening {
    let Some(sys) = world.systems.get(system) else {
        return Opening::Idle;
    };
    if sys.is_destroyed {
        return Opening::Idle;
    }
    let flagged: Vec<FleetKey> = sys
        .fleets
        .iter()
        .copied()
        .filter(|&fleet| {
            world
                .fleets
                .get(fleet)
                .is_some_and(|value| value.bombarding && value.usable())
        })
        .collect();
    let mut sides = flagged
        .iter()
        .map(|&fleet| side_of(world.fleets[fleet].is_alliance));
    let Some(bombarder) = sides.next() else {
        return Opening::Idle;
    };
    if sides.any(|side| side != bombarder) {
        return Opening::Idle;
    }
    if kind == BombardKind::DestroySystem
        && flagged
            .iter()
            .any(|&fleet| world.fleet_has_usable_death_star(fleet))
    {
        return Opening::Kill { bombarder };
    }
    Opening::Run {
        bombarder,
        fleets: flagged,
    }
}

/// A ship's remaining shields W = `+0x1e0` - `+0x204` (`FUN_00501160`),
/// derived on every read: the class shield strength S, scaled by the shield
/// counter c (`+0x64` bits 0-3) over GNPRTB `0x800` G, then by the hull left
/// over the maximum hull H, in integer divisions (`FUN_0053e170`); S when G
/// or H is 0.
fn remaining_shields(world: &GameWorld, ship: &ShipInstance) -> i32 {
    let Some(class) = world.capital_ship_classes.get(ship.class) else {
        return 0;
    };
    let strength = i64::from(class.shield_strength);
    let counter_max = crate::movement::gnprtb_or_shipped(
        world,
        crate::world::GNPRTB_DAMAGE_COUNTER_MAX,
        crate::world::SHIPPED_DAMAGE_COUNTER_MAX,
    );
    let hull_max = i64::from(class.hull);
    if counter_max == 0 || hull_max == 0 {
        return i32::try_from(strength).unwrap_or(i32::MAX);
    }
    let by_counter = strength * (counter_max - i64::from(ship.shield_nibble())) / counter_max;
    let hull_damage = hull_max - i64::from(ship.hull_current);
    let left = by_counter * (hull_max - hull_damage) / hull_max;
    i32::try_from(left).unwrap_or(i32::MAX)
}

/// `FUN_0058cba0`: the holder's targets, in the iterators' order: regiments
/// (`0x10`-`0x13`), defence facilities (`0x22`-`0x27`) and the HQ for a
/// military order; manufacturing and production facilities (`0x28`-`0x2f`)
/// for a civilian one; then energy and raw material while nonzero. Fleets
/// keep their regiments: only the system's own are walked. Each iterator
/// walks the system's child tree, ordered by family, then id
/// ([`container_order`]).
fn targets(world: &GameWorld, system: SystemKey, kind: BombardKind) -> Vec<BombardTarget> {
    let Some(sys) = world.systems.get(system) else {
        return Vec::new();
    };
    let holder = holder(world, system);
    let mut list = Vec::new();
    if kind.military() {
        let mut regiments: Vec<TroopKey> = sys
            .ground_units
            .iter()
            .copied()
            .filter(|&key| {
                world
                    .troops
                    .get(key)
                    .is_some_and(|unit| side_of(unit.is_alliance) == holder)
            })
            .collect();
        container_order(&mut regiments, |&key| world.troops[key].class_dat_id);
        list.extend(regiments.into_iter().map(BombardTarget::Regiment));
        let mut defence: Vec<FacilityRef> = sys
            .defense_facilities
            .iter()
            .filter(|&&key| {
                world
                    .defense_facilities
                    .get(key)
                    .is_some_and(|f| f.side == holder)
            })
            .map(|&key| FacilityRef::Defense(key))
            .collect();
        container_order(&mut defence, |&facility| {
            facility_class(world, facility).unwrap_or_default()
        });
        list.extend(defence.into_iter().map(BombardTarget::Facility));
        // The HQ is the Alliance's ALLFACSD object; Coruscant's flag is the
        // Empire's seat (`support_shift::alliance_headquarters`).
        if holder == Faction::Alliance && crate::support_shift::alliance_headquarters(world, system)
        {
            list.push(BombardTarget::Headquarters);
        }
    }
    if kind.civilian() {
        list.extend(
            civilian_facilities(world, system, holder)
                .into_iter()
                .map(BombardTarget::Facility),
        );
    }
    if kind.resources() {
        if sys.total_energy > 0 {
            list.push(BombardTarget::Energy);
        }
        if sys.raw_materials > 0 {
            list.push(BombardTarget::RawMaterial);
        }
    }
    list
}

/// `FUN_0053b760`: the holder's facilities of families `0x28`-`0x2f`, in
/// container order ([`container_order`]).
fn civilian_facilities(world: &GameWorld, system: SystemKey, holder: Faction) -> Vec<FacilityRef> {
    let Some(sys) = world.systems.get(system) else {
        return Vec::new();
    };
    let manufacturing = sys
        .manufacturing_facilities
        .iter()
        .filter(|&&key| {
            world
                .manufacturing_facilities
                .get(key)
                .is_some_and(|f| f.side == holder)
        })
        .map(|&key| FacilityRef::Manufacturing(key));
    let production = sys
        .production_facilities
        .iter()
        .filter(|&&key| {
            world
                .production_facilities
                .get(key)
                .is_some_and(|f| f.side == holder)
        })
        .map(|&key| FacilityRef::Production(key));
    let mut list: Vec<FacilityRef> = manufacturing.chain(production).collect();
    container_order(&mut list, |&facility| {
        facility_class(world, facility).unwrap_or_default()
    });
    list
}

/// Sort a system's children of one walk into its child tree's order: by
/// family (slot `+4`), then by id `+0x18` (`FUN_00584320`, slot 2 of the
/// tree class `FUN_00584150`, vtable `0x0066a168`), which every walk
/// follows from the leftmost child (`FUN_00539f70` -> `FUN_005f5060`,
/// `FUN_005130d0`). hyp: within a family the port's per-kind list is the id
/// order, ids rising as objects are made; the sort is stable.
fn container_order<T>(list: &mut [T], class: impl Fn(&T) -> crate::ids::DatId) {
    list.sort_by_key(|item| class(item).family());
}

/// The class record a facility names.
fn facility_class(world: &GameWorld, facility: FacilityRef) -> Option<crate::ids::DatId> {
    match facility {
        FacilityRef::Defense(key) => world.defense_facilities.get(key).map(|f| f.class_dat_id),
        FacilityRef::Manufacturing(key) => world
            .manufacturing_facilities
            .get(key)
            .map(|f| f.class_dat_id),
        FacilityRef::Production(key) => {
            world.production_facilities.get(key).map(|f| f.class_dat_id)
        }
    }
}

/// The class record's DAT statistics (`BuildableClass::stats`).
fn class_stats(world: &GameWorld, class: crate::ids::DatId) -> crate::world::ClassStats {
    world
        .buildable_classes
        .get(&class)
        .map(|value| value.stats)
        .unwrap_or_default()
}

/// The ALLFACSD record of the Alliance HQ (family `0x20`, record 1), whose
/// `extra7` slot `+0x1c0` reads on vtable `0x0065f280` (`FUN_00520b60`).
pub(crate) const HEADQUARTERS_CLASS: u32 = 0x2000_0001;

/// `FUN_0058cf80`: a target's value v; a hit destroys it when v is below the
/// 1-10 draw. Facilities and the HQ read their class `bombardment_defense`
/// (slot `+0x1c0`, `FUN_00520b60`; the HQ's is ALLFACSD `extra7`),
/// regiments their TROOPSD `bombardment_defense` (slot `+0x1d8`), energy and
/// raw material GNPRTB `0x604` and `0x605`.
fn target_value(world: &GameWorld, target: BombardTarget) -> i32 {
    let difficulty = world.difficulty_index;
    let stats = |class| class_stats(world, class).bombardment.cast_signed();
    match target {
        BombardTarget::Regiment(key) => world
            .troops
            .get(key)
            .map_or(0, |unit| stats(unit.class_dat_id)),
        BombardTarget::Facility(facility) => facility_class(world, facility).map_or(0, stats),
        BombardTarget::Headquarters => stats(crate::ids::DatId::new(HEADQUARTERS_CLASS)),
        BombardTarget::Energy => world.gnprtb.value(GNPRTB_ENERGY_VALUE, difficulty),
        BombardTarget::RawMaterial => world.gnprtb.value(GNPRTB_RAW_VALUE, difficulty),
    }
}

/// `FUN_0058d020`: a successful hit destroys the target (`FUN_0058ce70`) or
/// lowers energy or raw material by one (`FUN_0058cf20`, `FUN_0058cf50`).
fn apply_hit(
    world: &mut GameWorld,
    system: SystemKey,
    target: BombardTarget,
    report: &mut StrikeReport,
) {
    match target {
        BombardTarget::Regiment(key) => {
            report
                .system_destroyed
                .extend(crate::operation_report::regiment(world, key));
            if let Some(unit) = world.troops.get(key) {
                let side = side_of(unit.is_alliance);
                report.regiments.push((key, side));
                // FUN_0058ce70: cause 0x10 at the bombarder's own system.
                let reason = if report.holder == report.bombarder {
                    crate::blockade::REASON_OWN_BOMBARDMENT
                } else {
                    crate::blockade::REASON_BOMBARDMENT
                };
                report.losses.push(crate::blockade::UnitLoss {
                    side,
                    value: crate::support_shift::regiment_value(world, unit.class_dat_id),
                    reason,
                });
            }
            crate::missions::destroy_target(world, MissionTarget::Troop(key));
        }
        BombardTarget::Facility(facility) => {
            if !matches!(facility, FacilityRef::Defense(_)) {
                report.run.support_loss = true;
            }
            report.facilities.push(facility);
            report
                .system_destroyed
                .extend(crate::operation_report::facility(world, facility));
            crate::missions::destroy_target(
                world,
                match facility {
                    FacilityRef::Defense(key) => MissionTarget::DefenseFacility(key),
                    FacilityRef::Manufacturing(key) => MissionTarget::ManufacturingFacility(key),
                    FacilityRef::Production(key) => MissionTarget::ProductionFacility(key),
                },
            );
        }
        BombardTarget::Headquarters => {
            if crate::support_shift::remove_headquarters(world, system) {
                report.headquarters = true;
                report
                    .system_destroyed
                    .push(crate::operation_report::headquarters(world));
            }
        }
        BombardTarget::Energy => {
            if let Some(sys) = world.systems.get_mut(system) {
                sys.total_energy = sys.total_energy.saturating_sub(1);
            }
            report.energy += 1;
            report.run.support_loss = true;
        }
        BombardTarget::RawMaterial => {
            if let Some(sys) = world.systems.get_mut(system) {
                sys.raw_materials = sys.raw_materials.saturating_sub(1);
            }
            report.raw_material += 1;
            report.run.support_loss = true;
        }
    }
}

/// A character's leadership as missions read it (`member_skill`).
pub(crate) fn leadership(world: &GameWorld, character: CharacterKey) -> i32 {
    crate::missions::member_skill(
        world,
        crate::missions::MissionMember::Character(character),
        crate::world::Skill::Leadership,
    )
    .map_or(0, |value| i32::try_from(value).unwrap_or(0))
}

/// `FUN_00509330`: the first General (`+0x96` == 3) at the system itself,
/// not aboard a fleet, on the holder's side.
pub(crate) fn defending_general(
    world: &GameWorld,
    system: SystemKey,
    holder: Faction,
) -> Option<CharacterKey> {
    world
        .characters
        .iter()
        .find(|(_, character)| {
            !character.is_killed
                && character.current_system == Some(system)
                && character.current_fleet.is_none()
                && character.command_kind == CommandKind::General
                && side_of(character.is_alliance) == holder
        })
        .map(|(key, _)| key)
}

/// `FUN_004fd790(fleet, 2)`: the first Admiral (`+0x96` == 2) aboard the
/// fleet on its side.
fn admiral(world: &GameWorld, fleet: FleetKey) -> Option<CharacterKey> {
    fleet_officer(world, fleet, CommandKind::Admiral)
}

/// `FUN_004fd790(fleet, kind)`: the first character aboard the fleet on its
/// side whose `+0x96` is `kind`.
pub(crate) fn fleet_officer(
    world: &GameWorld,
    fleet: FleetKey,
    kind: CommandKind,
) -> Option<CharacterKey> {
    let value = world.fleets.get(fleet)?;
    value.characters.iter().copied().find(|&key| {
        world.characters.get(key).is_some_and(|character| {
            !character.is_killed
                && character.command_kind == kind
                && character.is_alliance == value.is_alliance
        })
    })
}

/// `FUN_0055e9a0`: damage against a pool; returns (leftover damage, pool).
const fn absorb(damage: i32, pool: i32) -> (i32, i32) {
    if pool < damage {
        (damage - pool, 0)
    } else {
        (0, pool - damage)
    }
}

/// The defence and effect phases (`FUN_0058dc10`, `FUN_0058df60`) of a run
/// `open` let through, seeded from one caller roll. port: the original
/// draws from its one generator; a seeded stream keeps the caller's budget
/// at one roll.
pub fn bombard(
    world: &mut GameWorld,
    system: SystemKey,
    kind: BombardKind,
    bombarder: Faction,
    fleets: &[FleetKey],
    roll: f64,
) -> StrikeReport {
    let mut draws = MissionRng::seeded(roll, system_seed(world, system));
    let holder = holder(world, system);
    let difficulty = world.difficulty_index;
    let mut report = StrikeReport {
        system: Some(system),
        kind: Some(kind),
        bombarder: Some(bombarder),
        holder: Some(holder),
        ..StrikeReport::default()
    };

    // Ship phase: the bombarding ships (+0x1c) and the targets (+0x20), the
    // usable ships of the usable flagged fleets (FUN_0058ca70.c:25-33, mode
    // 1 for both walks).
    let ships: Vec<(FleetKey, usize)> = fleets
        .iter()
        .flat_map(|&fleet| {
            world.fleets.get(fleet).into_iter().flat_map(move |value| {
                value
                    .capital_ships
                    .iter()
                    .enumerate()
                    .filter(|(_, ship)| ship.usable())
                    .map(move |(index, _)| (fleet, index))
            })
        })
        .collect();
    // Defence phase: each KDY, then each LNR, fires once at a random ship;
    // their iterators take no side (FUN_00527550, FUN_005272e0).
    let general = defending_general(world, system, holder).map_or(0, |key| leadership(world, key));
    // port: `.max(1)` only guards a GNPRTB 0x600 absent from the data (it
    // ships 40); the original divides by it as read.
    let multiplier = general
        / world
            .gnprtb
            .value(GNPRTB_DEFENCE_LEADERSHIP, difficulty)
            .max(1)
        + 1;
    let mut kdy = vec![0_i32; ships.len()];
    let mut lnr = vec![0_i32; ships.len()];
    let batteries: Vec<(u8, i32)> = world
        .systems
        .get(system)
        .map(|sys| {
            sys.defense_facilities
                .iter()
                .filter_map(|&key| world.defense_facilities.get(key))
                .map(|facility| {
                    (
                        facility.class_dat_id.family(),
                        class_stats(world, facility.class_dat_id)
                            .attack_strength
                            .cast_signed(),
                    )
                })
                .collect()
        })
        .unwrap_or_default();
    if !ships.is_empty() {
        let last = i32::try_from(ships.len() - 1).unwrap_or(i32::MAX);
        for (family, accumulator) in [(KDY_FAMILY, &mut kdy), (LNR_FAMILY, &mut lnr)] {
            for &(_, attack) in batteries.iter().filter(|(each, _)| *each == family) {
                let index = usize::try_from(draws.draw(last)).unwrap_or(0);
                accumulator[index] += attack * multiplier;
            }
        }
    }
    // FUN_0058d540 on every bombarding ship: KDY damage, then LNR damage,
    // against the shields; only LNR damage left over reaches the hull.
    let mut overflow = vec![0_i32; ships.len()];
    let mut death_star_lost = false;
    for (slot, &(fleet, index)) in ships.iter().enumerate() {
        let Some(ship) = world
            .fleets
            .get(fleet)
            .and_then(|value| value.capital_ships.get(index))
        else {
            continue;
        };
        let shields = remaining_shields(world, ship);
        let (kdy_left, shields) = absorb(kdy[slot], shields);
        let (lnr_left, _) = absorb(lnr[slot], shields);
        let (_, hull) = absorb(lnr_left, ship.hull_current);
        let value = crate::support_shift::ship_value(world, ship.class);
        overflow[slot] = kdy_left;
        report.defence_damage += kdy[slot] + lnr[slot];
        if lnr_left > 0 {
            if let Some(ship) = world
                .fleets
                .get_mut(fleet)
                .and_then(|value| value.capital_ships.get_mut(index))
            {
                ship.hull_current = hull;
                if hull <= 0 {
                    ship.alive = false;
                    death_star_lost |= world
                        .capital_ship_classes
                        .get(ship.class)
                        .is_some_and(crate::world::CapitalShipClass::is_death_star);
                    report.ships_destroyed += 1;
                    report
                        .fleet_destroyed
                        .extend(crate::operation_report::destroyed_ship(world, fleet, index));
                    // FUN_00501ae0(hull loss, 0xf, ...).
                    report.losses.push(crate::blockade::UnitLoss {
                        side: bombarder,
                        value,
                        reason: crate::blockade::REASON_BOMBARDMENT,
                    });
                }
            }
        }
    }
    // +0x30: no bombarding ship remains (the same mode-1 walk).
    let ships_left = fleets.iter().any(|&fleet| {
        world
            .fleets
            .get(fleet)
            .is_some_and(|value| value.capital_ships.iter().any(ShipInstance::usable))
    });
    if ships_left {
        effect_phase(
            world,
            system,
            kind,
            fleets,
            &ships,
            &overflow,
            &mut draws,
            &mut report,
        );
    }
    // Ships the batteries destroyed leave their fleets after the run, so the
    // run's ship indices hold throughout. The squadrons aboard die with
    // their ship's reason: its hook +0x11c (FUN_004fb520) calls each
    // child's slot +0xb4, FUN_004f8710 for a squadron, which destroys a
    // completed child with the container's reason byte (FUN_004f7ed0), 0xf
    // here, so FUN_00508c60 counts each in B as well.
    if report.ships_destroyed > 0 {
        for &fleet in fleets {
            let classes: Vec<crate::ids::FighterKey> = world
                .fleets
                .get(fleet)
                .into_iter()
                .flat_map(|value| {
                    value
                        .fighters
                        .iter()
                        .filter(|entry| crate::carriage::lost_with_ship(value, entry.carrier))
                        .flat_map(|entry| (0..entry.count).map(move |_| entry.class))
                })
                .collect();
            report
                .losses
                .extend(classes.iter().map(|&class| crate::blockade::UnitLoss {
                    side: bombarder,
                    value: crate::support_shift::squadron_value(world, class),
                    reason: crate::blockade::REASON_BOMBARDMENT,
                }));
            let lost: Vec<ReportUnit> = classes
                .into_iter()
                .filter_map(|class| crate::operation_report::squadron(world, class))
                .collect();
            report.fleet_destroyed.extend(lost);
            if let Some(value) = world.fleets.get_mut(fleet) {
                value.capital_ships.retain(|ship| ship.alive);
                crate::carriage::drop_lost_squadrons(value);
            }
        }
    }
    // A Death Star's kill clears its usable bit (FUN_004f7b80 ->
    // FUN_004f7410), whose event 0x301 reruns bit 11 (FUN_00514510).
    if death_star_lost {
        crate::death_star::refresh_sectors(world);
    }
    report
}

/// The effect phase (`FUN_0058df60`): the hits, collateral and each hit's
/// roll.
#[expect(
    clippy::too_many_arguments,
    reason = "The phase reads the run's ships, their overflow, its draws and its report."
)]
fn effect_phase(
    world: &mut GameWorld,
    system: SystemKey,
    kind: BombardKind,
    fleets: &[FleetKey],
    ships: &[(FleetKey, usize)],
    overflow: &[i32],
    draws: &mut MissionRng,
    report: &mut StrikeReport,
) {
    let holder = holder(world, system);
    let difficulty = world.difficulty_index;
    let target_count = targets(world, system, kind).len();
    // port: `.max(1)` only guards a GNPRTB 0x601 absent from the data (it
    // ships 40).
    let divisor = world.gnprtb.value(GNPRTB_HIT_LEADERSHIP, difficulty).max(1);
    let mut hits = 0;
    for &fleet in fleets {
        let overflow_of = |index: usize| {
            ships
                .iter()
                .position(|&(each, at)| each == fleet && at == index)
                .and_then(|slot| overflow.get(slot).copied())
                .unwrap_or(0)
        };
        let strength = fleet_strength_with(world, fleet, &overflow_of);
        let lead = admiral(world, fleet).map_or(0, |key| leadership(world, key));
        hits += strength * (lead / divisor + 1);
    }
    let shield_defence: i32 = world
        .systems
        .get(system)
        .map(|sys| {
            sys.defense_facilities
                .iter()
                .filter_map(|&key| world.defense_facilities.get(key))
                .filter(|facility| {
                    facility.side == holder && facility.class_dat_id.family() == SHIELD_FAMILY
                })
                .map(|facility| {
                    class_stats(world, facility.class_dat_id)
                        .shield_strength
                        .cast_signed()
                })
                .sum()
        })
        .unwrap_or(0);
    hits = (hits - shield_defence).max(0);
    report.hits = hits;
    if hits == 0 {
        return;
    }

    let low = world.gnprtb.value(GNPRTB_HIT_DRAW_LOW, difficulty);
    let high = world.gnprtb.value(GNPRTB_HIT_DRAW_HIGH, difficulty);
    let succeeds = |draws: &mut MissionRng, value: i32| value < low + draws.draw(high - low);
    let count = i32::try_from(target_count).unwrap_or(i32::MAX);

    // Collateral (0x220): one civilian facility, with chance 1 in count + 1.
    if kind.collateral() && draws.draw(count) < 1 {
        let civilian = civilian_facilities(world, system, holder);
        if let Some(&first) = civilian.first() {
            // FUN_0058d080: the index is drawn, but the walk never advances
            // (0x0058d0f8-0x0058d116), so the first facility is taken.
            let last = i32::try_from(civilian.len() - 1).unwrap_or(i32::MAX);
            draws.draw(last);
            let target = BombardTarget::Facility(first);
            if succeeds(draws, target_value(world, target)) {
                apply_hit(world, system, target, report);
            }
        }
    }
    // With no target the hit loop's FUN_0058d370 finds nothing.
    if target_count == 0 {
        return;
    }

    // Each hit picks an index over the ship phase's count; an index past the
    // shrunken list misses. The loop ends once no target remains.
    for _ in 0..hits {
        let index = usize::try_from(draws.draw(count - 1)).unwrap_or(usize::MAX);
        let current = targets(world, system, kind);
        if current.is_empty() {
            break;
        }
        let Some(&target) = current.get(index) else {
            continue;
        };
        if succeeds(draws, target_value(world, target)) {
            apply_hit(world, system, target, report);
        }
    }
}

/// The seed a run mixes with its roll: the system's DatId.
pub(crate) fn system_seed(world: &GameWorld, system: SystemKey) -> u64 {
    world
        .systems
        .get(system)
        .map_or(0, |sys| u64::from(sys.dat_id.raw()))
}

/// `FUN_0058d990` after a kill: each minor character (`0x38`-`0x3b`) still
/// directly at the system and not in transit, whatever its side, dies with
/// chance GNPRTB `0x606`; a hit runs the injury and the destroy slot.
pub fn roll_characters(world: &mut GameWorld, system: SystemKey, roll: f64) -> Vec<CharacterKey> {
    let mut draws = MissionRng::seeded(roll, system_seed(world, system));
    let chance = world
        .gnprtb
        .value(GNPRTB_CHARACTER_CHANCE, world.difficulty_index);
    let present: Vec<CharacterKey> = world
        .characters
        .iter()
        .filter(|(_, character)| {
            !character.is_killed
                && !character.is_major
                && !character.en_route
                && character.current_system == Some(system)
                && character.current_fleet.is_none()
        })
        .map(|(key, _)| key)
        .collect();
    let mut killed = Vec::new();
    for key in present {
        if draws.chance(chance) {
            if let Some(character) = world.characters.get_mut(key) {
                character.mark_killed();
            }
            killed.push(key);
        }
    }
    killed
}

/// The finalize's flag clearing (`FUN_004ffef0` mode 4, `FUN_004fe460(0)`):
/// every fleet of the bombarder's side at the system stops bombarding.
pub fn clear_flags(world: &mut GameWorld, system: SystemKey, bombarder: Faction) {
    let fleets = world
        .systems
        .get(system)
        .map(|sys| sys.fleets.clone())
        .unwrap_or_default();
    for fleet in fleets {
        if let Some(value) = world.fleets.get_mut(fleet) {
            if side_of(value.is_alliance) == bombarder {
                value.bombarding = false;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Finalize (`FUN_0058e320`)
// ---------------------------------------------------------------------------

/// GNPRTB 7705 (`0x1e19`, `DAT_006bb47c`, `FUN_00558bb0.c:178`): the change
/// to the bombarding side's support at the system (`FUN_0055a0e0`).
const GNPRTB_SYSTEM_SHIFT: u16 = 0x1e19;
/// GNPRTB `0x1401` (`DAT_006bb630`, `FUN_0055cb60.c:6`): the rim amount of
/// the galaxy-wide shift after a Death Star kill (`FUN_0055da90`).
const GNPRTB_RIM_AMOUNT: u16 = 0x1401;
/// GNPRTB `0x1402` (`DAT_006bb5fc`, `FUN_0055cb60.c:7`): its rim gate limit.
const GNPRTB_RIM_LIMIT: u16 = 0x1402;
/// GNPRTB `0x1403` (`DAT_006bb694`, `FUN_0055cb60.c:8`): its core amount.
const GNPRTB_CORE_AMOUNT: u16 = 0x1403;
/// GNPRTB `0x1404` (`DAT_006bb628`, `FUN_0055cb60.c:9`): its core gate limit.
const GNPRTB_CORE_LIMIT: u16 = 0x1404;
/// SDPRTB `0x1408` by side (`DAT_006bb614`/`DAT_006bb5f4`,
/// `FUN_0055cb60.c:36-37`): the rim amount of the sector-wide shift after
/// any other bombardment (`FUN_0055d900`).
const SDPRTB_RIM_AMOUNT: u16 = 0x1408;
/// SDPRTB `0x1409` (`DAT_006bb6d0`/`DAT_006bb6c4`, `FUN_0055cb60.c:38-39`):
/// its rim gate limit.
const SDPRTB_RIM_LIMIT: u16 = 0x1409;
/// SDPRTB `0x140a` (`DAT_006bb66c`/`DAT_006bb6a8`, `FUN_0055cb60.c:40-41`):
/// its core amount.
const SDPRTB_CORE_AMOUNT: u16 = 0x140a;
/// SDPRTB `0x140b` (`DAT_006bb6c0`/`DAT_006bb624`, `FUN_0055cb60.c:42-43`):
/// its core gate limit.
const SDPRTB_CORE_LIMIT: u16 = 0x140b;
/// The core system families a shift record names (`FUN_0055d900`,
/// `FUN_0055da90`: `0x90`..`0x92`).
const CORE_FAMILIES: std::ops::Range<u8> = 0x90..0x92;
/// The rim system families (`FUN_0055d900`, `FUN_0055da90`: `0x92`..`0x94`).
const RIM_FAMILIES: std::ops::Range<u8> = 0x92..0x94;

/// What a bombardment run leaves its finalize (the state's `+0x28` and
/// `+0x2c`, `FUN_0058d720`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BombardmentRun {
    /// `+0x28`: a civilian facility destroyed, or energy or raw material
    /// lost (`FUN_0058ce70`, `FUN_0058cf20`, `FUN_0058cf50`), or the planet
    /// destroyed (`FUN_0058d990`).
    pub support_loss: bool,
    /// `+0x2c`: the Death Star destroyed the planet (`FUN_0058d990` ->
    /// `FUN_0058ce70`).
    pub planet_destroyed: bool,
}

/// `FUN_0058e320`: the support shifts after `side` bombarded `system`. With
/// `+0x28`, the side's support at the system changes by GNPRTB 7705
/// (`FUN_0050c9f0`). Then, with `+0x2c`, every sector shifts against the
/// side (`FUN_0055da90`, `FUN_00518a70`); with `+0x28` alone, the system's
/// sector does (`FUN_0055d900`, `FUN_00518ac0`). Core systems shift before
/// rim ones (`FUN_0058e320` passes 1, then 0).
pub fn finalize(world: &mut GameWorld, system: SystemKey, side: Faction, run: BombardmentRun) {
    let difficulty = world.difficulty_index;
    let Some(sys) = world.systems.get(system) else {
        return;
    };
    let sector = sys.sector;
    if run.support_loss {
        let delta = world.gnprtb.value(GNPRTB_SYSTEM_SHIFT, difficulty);
        crate::uprising::apply_support_change(world, system, side, delta);
    }
    if run.planet_destroyed {
        let core = (
            world.gnprtb.value(GNPRTB_CORE_AMOUNT, difficulty),
            world.gnprtb.value(GNPRTB_CORE_LIMIT, difficulty),
        );
        let rim = (
            world.gnprtb.value(GNPRTB_RIM_AMOUNT, difficulty),
            world.gnprtb.value(GNPRTB_RIM_LIMIT, difficulty),
        );
        let sectors: Vec<SectorKey> = world.sectors.keys().collect();
        for (families, (amount, limit)) in [(CORE_FAMILIES, core), (RIM_FAMILIES, rim)] {
            for &each in &sectors {
                shift_sector(world, each, families.clone(), side, limit, amount);
            }
        }
    } else if run.support_loss {
        let core = (
            world.sdprtb.value(SDPRTB_CORE_AMOUNT, difficulty, side),
            world.sdprtb.value(SDPRTB_CORE_LIMIT, difficulty, side),
        );
        let rim = (
            world.sdprtb.value(SDPRTB_RIM_AMOUNT, difficulty, side),
            world.sdprtb.value(SDPRTB_RIM_LIMIT, difficulty, side),
        );
        for (families, (amount, limit)) in [(CORE_FAMILIES, core), (RIM_FAMILIES, rim)] {
            shift_sector(world, sector, families, side, limit, amount);
        }
    }
}

/// `FUN_0055d900`, `FUN_0055da90`: a record with `side` as gate and
/// direction, posted to `sector` (`support_shift::post`); it fires with
/// the strike's other shifts (`support_shift::fire`).
fn shift_sector(
    world: &mut GameWorld,
    sector: SectorKey,
    families: std::ops::Range<u8>,
    side: Faction,
    limit: i32,
    amount: i32,
) {
    let record = crate::support_shift::ShiftRecord {
        families,
        gate_side: side,
        gate_limit: limit,
        direction: side,
        amount,
    };
    crate::support_shift::post(
        world,
        sector,
        &record,
        crate::support_shift::ShiftCause::Bombardment,
    );
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{CapitalShipKey, DatId};
    use crate::world::{
        BuildableClass, CapitalShipClass, Character, ClassStats, ControlKind,
        DefenseFacilityInstance, Fleet, GnprtbEntry, GnprtbParams, ManufacturingFacilityInstance,
        Sector, System, TroopUnit,
    };

    fn entry(id: u16, value: i32) -> GnprtbEntry {
        GnprtbEntry {
            parameter_id: u32::from(id),
            development: value,
            alliance_sp_easy: value,
            alliance_sp_medium: value,
            alliance_sp_hard: value,
            empire_sp_easy: value,
            empire_sp_medium: value,
            empire_sp_hard: value,
            multiplayer: value,
        }
    }

    /// A world with one system held by `holder` and the shipped GNPRTB
    /// `0x600`-`0x606` (40, 40, 1, 10, 9, 9, 50) and 7705 (-20).
    fn world(holder: Option<Faction>) -> (GameWorld, SystemKey) {
        let mut world = GameWorld {
            gnprtb: GnprtbParams::new(vec![
                entry(0x600, 40),
                entry(0x601, 40),
                entry(0x602, 1),
                entry(0x603, 10),
                entry(0x604, 9),
                entry(0x605, 9),
                entry(0x606, 50),
                entry(0x1e19, -20),
            ]),
            ..GameWorld::default()
        };
        let sector = world.sectors.insert(Sector::default());
        let system = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0091),
            sector,
            is_populated: true,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            control: holder.map_or(ControlKind::Uncontrolled, ControlKind::Controlled),
            ..System::default()
        });
        world.sectors[sector].systems.push(system);
        (world, system)
    }

    fn ship_class(
        world: &mut GameWorld,
        bombard: u32,
        hull: u32,
        shields: u32,
        family: u8,
    ) -> CapitalShipKey {
        world.capital_ship_classes.insert(CapitalShipClass {
            family,
            bombardment_modifier: bombard,
            hull,
            shield_strength: shields,
            ..CapitalShipClass::default()
        })
    }

    fn fleet(
        world: &mut GameWorld,
        system: SystemKey,
        side: Faction,
        classes: &[CapitalShipKey],
    ) -> FleetKey {
        let is_alliance = side == Faction::Alliance;
        let ships = classes
            .iter()
            .map(|&class| {
                let hull = world.capital_ship_classes[class].hull.cast_signed();
                ShipInstance::new(class, hull, is_alliance)
            })
            .collect();
        let key = world.fleets.insert(Fleet {
            location: system,
            capital_ships: ships,
            is_alliance,
            ..Fleet::default()
        });
        world.systems[system].fleets.push(key);
        key
    }

    fn class(world: &mut GameWorld, raw: u32, stats: ClassStats) -> DatId {
        let id = DatId::new(raw);
        world.buildable_classes.insert(
            id,
            BuildableClass {
                stats,
                ..BuildableClass::default()
            },
        );
        id
    }

    fn regiment(world: &mut GameWorld, system: SystemKey, side: Faction, value: u32) -> TroopKey {
        let class_dat_id = class(
            world,
            0x1000_0000 | value,
            ClassStats {
                bombardment: value,
                ..ClassStats::default()
            },
        );
        let key = world.troops.insert(TroopUnit {
            class_dat_id,
            is_alliance: side == Faction::Alliance,
        });
        world.systems[system].ground_units.push(key);
        key
    }

    fn defence(
        world: &mut GameWorld,
        system: SystemKey,
        side: Faction,
        family: u8,
        stats: ClassStats,
    ) {
        let class_dat_id = class(
            world,
            (u32::from(family) << 24) | stats.attack_strength | stats.shield_strength << 12,
            stats,
        );
        let key = world
            .defense_facilities
            .insert(DefenseFacilityInstance { class_dat_id, side });
        world.systems[system].defense_facilities.push(key);
    }

    fn yard(world: &mut GameWorld, system: SystemKey, side: Faction, value: u32) {
        let class_dat_id = class(
            world,
            0x2800_0000 | value,
            ClassStats {
                bombardment: value,
                ..ClassStats::default()
            },
        );
        let key = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id,
                side,
                ..ManufacturingFacilityInstance::default()
            });
        world.systems[system].manufacturing_facilities.push(key);
    }

    /// A civilian facility of `family` (`0x28`-`0x2a` MANFACSD, `0x2c`-`0x2d`
    /// PROFACSD) valued 0, added to the end of its per-kind list.
    fn civilian(
        world: &mut GameWorld,
        system: SystemKey,
        side: Faction,
        family: u8,
    ) -> FacilityRef {
        let class_dat_id = DatId::new(u32::from(family) << 24);
        if family >= 0x2c {
            let key =
                world
                    .production_facilities
                    .insert(crate::world::ProductionFacilityInstance {
                        class_dat_id,
                        side,
                        ..crate::world::ProductionFacilityInstance::default()
                    });
            world.systems[system].production_facilities.push(key);
            FacilityRef::Production(key)
        } else {
            let key = world
                .manufacturing_facilities
                .insert(ManufacturingFacilityInstance {
                    class_dat_id,
                    side,
                    ..ManufacturingFacilityInstance::default()
                });
            world.systems[system].manufacturing_facilities.push(key);
            FacilityRef::Manufacturing(key)
        }
    }

    fn character(
        world: &mut GameWorld,
        system: SystemKey,
        set: impl FnOnce(&mut Character),
    ) -> CharacterKey {
        let mut value = Character {
            current_system: Some(system),
            ..Character::default()
        };
        set(&mut value);
        world.characters.insert(value)
    }

    fn run(
        world: &mut GameWorld,
        system: SystemKey,
        kind: BombardKind,
        fleets: &[FleetKey],
    ) -> StrikeReport {
        for &each in fleets {
            assert!(flag_fleet(world, each, false));
        }
        match open(world, system, kind) {
            Opening::Run { bombarder, fleets } => {
                bombard(world, system, kind, bombarder, &fleets, 0.5)
            }
            other => panic!("expected a run, got {other:?}"),
        }
    }

    #[test]
    fn the_four_orders_carry_their_strategy_ids() {
        // STRATEGY RCDATA 0x220-0x223, read by FUN_0058d800.
        assert_eq!(BombardKind::Military.order(), 0x220);
        assert_eq!(BombardKind::Civilian.order(), 0x221);
        assert_eq!(BombardKind::General.order(), 0x222);
        assert_eq!(BombardKind::DestroySystem.order(), 0x223);
    }

    #[test]
    fn each_order_targets_its_own_kinds() {
        // FUN_0058d800's +0x34 military, +0x38 civilian and +0x3c resource
        // flags select FUN_0058cba0's lists.
        for (holder, _) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            world.systems[system].is_headquarters = true;
            world.systems[system].total_energy = 3;
            world.systems[system].raw_materials = 2;
            regiment(&mut world, system, holder, 5);
            defence(
                &mut world,
                system,
                holder,
                KDY_FAMILY,
                ClassStats::default(),
            );
            yard(&mut world, system, holder, 5);
            let kinds = |kind| {
                let list = targets(&world, system, kind);
                [
                    list.iter().any(|t| matches!(t, BombardTarget::Regiment(_))),
                    list.iter()
                        .any(|t| matches!(t, BombardTarget::Facility(FacilityRef::Defense(_)))),
                    list.contains(&BombardTarget::Headquarters),
                    list.iter().any(|t| {
                        matches!(t, BombardTarget::Facility(FacilityRef::Manufacturing(_)))
                    }),
                    list.contains(&BombardTarget::Energy),
                    list.contains(&BombardTarget::RawMaterial),
                ]
            };
            // The HQ is the Alliance's object (FUN_0058cba0).
            let hq = holder == Faction::Alliance;
            assert_eq!(
                kinds(BombardKind::Military),
                [true, true, hq, false, false, false],
                "{holder:?}"
            );
            assert_eq!(
                kinds(BombardKind::Civilian),
                [false, false, false, true, false, false],
                "{holder:?}"
            );
            assert_eq!(
                kinds(BombardKind::General),
                [true, true, hq, true, true, true],
                "{holder:?}"
            );
            assert_eq!(
                kinds(BombardKind::DestroySystem),
                [true, true, hq, true, true, true],
                "{holder:?}"
            );
        }
    }

    /// Kills of civilian facilities over 40 seeds: one run per roll, each
    /// on a fresh world from `stage`.
    fn civilian_kills(
        holder: Faction,
        bombarder: Faction,
        kind: BombardKind,
        stage: impl Fn(&mut GameWorld, SystemKey),
    ) -> Vec<usize> {
        (0..40)
            .map(|step| {
                let (mut world, system) = world(Some(holder));
                let armed = ship_class(&mut world, 1, 100, 0, 0x14);
                let attacker = fleet(&mut world, system, bombarder, &[armed]);
                stage(&mut world, system);
                assert!(flag_fleet(&mut world, attacker, false));
                let report = bombard(
                    &mut world,
                    system,
                    kind,
                    bombarder,
                    &[attacker],
                    f64::from(step) / 40.0,
                );
                assert_eq!(report.hits, 1);
                report.facilities.len()
            })
            .collect()
    }

    #[test]
    fn target_military_kills_a_civilian_facility_only_as_collateral() {
        // FUN_0058df60 +0x40: with chance 1 in count + 1, one civilian
        // facility is hit before the loop. The one military target, valued
        // 10, never dies (FUN_0055e970), so any kill is the collateral.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let kills =
                civilian_kills(holder, bombarder, BombardKind::Military, |world, system| {
                    regiment(world, system, holder, 10);
                    yard(world, system, holder, 0);
                });
            assert!(kills.contains(&1), "{holder:?} {kills:?}");
            assert!(kills.contains(&0), "{holder:?} {kills:?}");
            assert!(
                kills.iter().all(|&count| count <= 1),
                "{holder:?} {kills:?}"
            );
        }
    }

    #[test]
    fn target_military_always_rolls_collateral_when_the_holder_has_no_military_target() {
        // FUN_0058df60: the roll is FUN_0053e290(+0x20 - 1 + +0x24) <
        // +0x24 with +0x24 = 1 (FUN_0058d990), so a count of 0 always takes
        // the collateral hit.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            for step in 0..40 {
                let (mut world, system) = world(Some(holder));
                let armed = ship_class(&mut world, 1, 100, 0, 0x14);
                let attacker = fleet(&mut world, system, bombarder, &[armed]);
                yard(&mut world, system, holder, 0);
                assert!(flag_fleet(&mut world, attacker, false));
                let report = bombard(
                    &mut world,
                    system,
                    BombardKind::Military,
                    bombarder,
                    &[attacker],
                    f64::from(step) / 40.0,
                );
                assert_eq!(report.facilities.len(), 1, "{holder:?} step {step}");
            }
        }
    }

    #[test]
    fn the_collateral_hit_always_takes_the_first_civilian_facility() {
        // FUN_0058d080 0x0058d0f8-0x0058d116: the walk counts up to the
        // drawn index without advancing (no FUN_005130d0), so the pick is
        // the iterator's first element.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            for step in 0..40 {
                let (mut world, system) = world(Some(holder));
                let armed = ship_class(&mut world, 1, 100, 0, 0x14);
                let attacker = fleet(&mut world, system, bombarder, &[armed]);
                yard(&mut world, system, holder, 0);
                yard(&mut world, system, holder, 0);
                let first = world.systems[system].manufacturing_facilities[0];
                assert!(flag_fleet(&mut world, attacker, false));
                let report = bombard(
                    &mut world,
                    system,
                    BombardKind::Military,
                    bombarder,
                    &[attacker],
                    f64::from(step) / 40.0,
                );
                assert_eq!(
                    report.facilities,
                    vec![FacilityRef::Manufacturing(first)],
                    "{holder:?} step {step}"
                );
            }
        }
    }

    #[test]
    fn the_collateral_hit_takes_the_lowest_family_in_container_order() {
        // FUN_0058d080 takes the first object of FUN_0053b760's walk, which
        // starts at the system's leftmost child (FUN_00539f70 ->
        // FUN_005f5060) and follows the in-order links; the child tree
        // orders by family, then id (comparator FUN_00584320, vtable
        // 0x0066a168 slot 2). A construction yard (0x2a) built before a
        // shipyard (0x28) still comes after it.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            for step in 0..40 {
                let (mut world, system) = world(Some(holder));
                let armed = ship_class(&mut world, 1, 100, 0, 0x14);
                let attacker = fleet(&mut world, system, bombarder, &[armed]);
                civilian(&mut world, system, holder, 0x2a);
                let shipyard = civilian(&mut world, system, holder, 0x28);
                assert!(flag_fleet(&mut world, attacker, false));
                let report = bombard(
                    &mut world,
                    system,
                    BombardKind::Military,
                    bombarder,
                    &[attacker],
                    f64::from(step) / 40.0,
                );
                assert_eq!(report.facilities, vec![shipyard], "{holder:?} step {step}");
            }
        }
    }

    #[test]
    fn targets_follow_the_systems_container_order() {
        // FUN_0058cba0 walks each family range over the system's child
        // tree, ordered by family then id (FUN_00584320): defence 0x22-0x27
        // (FUN_00527050) and civilian 0x28-0x2f (FUN_0053b760) come out by
        // family, each family in the order its objects were made.
        for holder in [Faction::Empire, Faction::Alliance] {
            let (mut world, system) = world(Some(holder));
            defence(
                &mut world,
                system,
                holder,
                SHIELD_FAMILY,
                ClassStats::default(),
            );
            defence(
                &mut world,
                system,
                holder,
                KDY_FAMILY,
                ClassStats::default(),
            );
            let [shield, kdy] = [0, 1].map(|index| {
                BombardTarget::Facility(FacilityRef::Defense(
                    world.systems[system].defense_facilities[index],
                ))
            });
            let refinery = civilian(&mut world, system, holder, 0x2d);
            let mine = civilian(&mut world, system, holder, 0x2c);
            let yard_b = civilian(&mut world, system, holder, 0x2a);
            let training = civilian(&mut world, system, holder, 0x29);
            let yard_a = civilian(&mut world, system, holder, 0x2a);
            let shipyard = civilian(&mut world, system, holder, 0x28);
            assert_eq!(
                targets(&world, system, BombardKind::Military),
                vec![kdy, shield],
                "{holder:?}"
            );
            assert_eq!(
                targets(&world, system, BombardKind::Civilian),
                [shipyard, training, yard_b, yard_a, mine, refinery]
                    .map(BombardTarget::Facility)
                    .to_vec(),
                "{holder:?}"
            );
        }
    }

    #[test]
    fn a_stalemated_system_is_bombarded_as_its_earlier_holders() {
        // FUN_0050b610.c:45-70 keeps the troop bits and the holder while both
        // sides' regiments stand; FUN_0058cba0 takes the holder from system
        // +0x24 bits 6-7.
        for (earlier, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(earlier));
            let armed = ship_class(&mut world, 5, 100, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            let defender = regiment(&mut world, system, earlier, 0);
            crate::support_shift::refresh_troop_bits(&mut world, system);
            regiment(&mut world, system, bombarder, 10);
            crate::support_shift::rebuild_control(&mut world, system);
            assert_eq!(holder(&world, system), earlier);
            let report = run(&mut world, system, BombardKind::Military, &[attacker]);
            assert_eq!(report.regiments, vec![(defender, earlier)], "{earlier:?}");
        }
    }

    #[test]
    fn a_ships_shields_shrink_with_its_shield_counter_and_its_hull_damage() {
        // FUN_0058d540 reads the remaining shields once per ship from slot
        // +0x204 (FUN_00501160): W = S*(G-c)/G, then W*(H-d)/H, integer
        // divisions (FUN_0053e170), S = CAPSHPSD shield_strength, G = GNPRTB
        // 0x800 (4), c = ship +0x64 bits 0-3, H = maximum hull, d = hull
        // damage. LNR damage past W reaches the hull. One LNR of 60, S = 100,
        // H = 1000: undamaged W = 100 takes it all; c = 2 gives W = 50 and
        // 10 to the hull; d = 500 gives W = 50 and 10 to the hull.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            for (counter, hull, expected) in [(0, 1000, 1000), (2, 1000, 990), (0, 500, 490)] {
                let (mut world, system) = world(Some(holder));
                let class = ship_class(&mut world, 1, 1000, 100, 0x14);
                let attacker = fleet(&mut world, system, bombarder, &[class]);
                let ship = &mut world.fleets[attacker].capital_ships[0];
                ship.set_shield_nibble(counter);
                ship.hull_current = hull;
                let lnr = ClassStats {
                    attack_strength: 60,
                    ..ClassStats::default()
                };
                defence(&mut world, system, holder, LNR_FAMILY, lnr);
                assert!(flag_fleet(&mut world, attacker, false));
                bombard(
                    &mut world,
                    system,
                    BombardKind::Military,
                    bombarder,
                    &[attacker],
                    0.5,
                );
                assert_eq!(
                    world.fleets[attacker].capital_ships[0].hull_current, expected,
                    "{holder:?} counter {counter} hull {hull}"
                );
            }
        }
    }

    #[test]
    fn a_battery_fires_whichever_side_it_belongs_to() {
        // FUN_00527550 (KDY) and FUN_005272e0 (LNR) build their iterators
        // with FUN_00513050, which takes no side; only the shield iterator
        // FUN_00527940 passes one.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 5, 10_000, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            let lnr = ClassStats {
                attack_strength: 60,
                ..ClassStats::default()
            };
            defence(&mut world, system, bombarder, LNR_FAMILY, lnr);
            assert!(flag_fleet(&mut world, attacker, false));
            let report = bombard(
                &mut world,
                system,
                BombardKind::Military,
                bombarder,
                &[attacker],
                0.5,
            );
            assert_eq!(report.defence_damage, 60, "{holder:?}");
        }
    }

    #[test]
    fn only_target_military_adds_the_collateral_hit() {
        // One hit on two civilian facilities valued 0 kills exactly one:
        // Target Civilian runs no collateral (+0x40 clear).
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let kills =
                civilian_kills(holder, bombarder, BombardKind::Civilian, |world, system| {
                    yard(world, system, holder, 0);
                    yard(world, system, holder, 0);
                });
            assert!(
                kills.iter().all(|&count| count == 1),
                "{holder:?} {kills:?}"
            );
        }
    }

    #[test]
    fn a_fleets_strength_sums_its_ships_and_each_squadrons_value() {
        // FUN_005032a0: +0x1dc sums the ships' bombard values and each
        // squadron's FIGHTSD bombardment_defense.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 5, 100, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            let class = world.fighter_classes.insert(crate::world::FighterClass {
                bombardment_defense: 2,
                ..crate::world::FighterClass::default()
            });
            // The squadrons ride the fleet's ship, as every squadron of a fleet
            // does (FUN_005039d0).
            let carrier = crate::carriage::tag_ship(&mut world, attacker, 0).unwrap();
            world.fleets[attacker]
                .fighters
                .push(crate::world::FighterEntry {
                    class,
                    count: 3,
                    carrier,
                });
            assert_eq!(fleet_strength(&world, attacker), 5 + 2 * 3, "{holder:?}");
        }
    }

    #[test]
    fn a_death_star_fleet_given_another_order_bombards_instead_of_destroying() {
        // FUN_0058d990 runs the destruction only for 0x223.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let death_star = ship_class(&mut world, 15, 25_000, 2000, 0x18);
            let attacker = fleet(&mut world, system, bombarder, &[death_star]);
            assert!(flag_fleet(&mut world, attacker, false), "{holder:?}");
            assert!(
                matches!(
                    open(&world, system, BombardKind::General),
                    Opening::Run { .. }
                ),
                "{holder:?}"
            );
            assert_eq!(
                open(&world, system, BombardKind::DestroySystem),
                Opening::Kill { bombarder },
                "{holder:?}"
            );
        }
    }

    #[test]
    fn the_hq_counts_only_for_its_alliance_holder_and_empty_stocks_are_no_targets() {
        // FUN_0058cba0: the HQ is the Alliance's object; energy and raw
        // material are targets only while nonzero.
        let (mut world, system) = world(Some(Faction::Empire));
        world.systems[system].is_headquarters = true;
        let list = targets(&world, system, BombardKind::General);
        assert!(!list.contains(&BombardTarget::Headquarters));
        assert!(!list.contains(&BombardTarget::Energy));
        assert!(!list.contains(&BombardTarget::RawMaterial));

        let (alliance, held) = self::world(Some(Faction::Alliance));
        assert!(
            !targets(&alliance, held, BombardKind::Military).contains(&BombardTarget::Headquarters)
        );
    }

    #[test]
    fn an_alliance_held_coruscant_offers_no_headquarters_target() {
        // Coruscant's flag is the Empire's seat, not the ALLFACSD HQ object
        // FUN_0058cba0 lists (support_shift::alliance_headquarters).
        let (mut world, system) = world(Some(Faction::Alliance));
        world.systems[system].dat_id = DatId::new(0x9000_0109);
        world.systems[system].is_headquarters = true;
        assert!(
            !targets(&world, system, BombardKind::Military).contains(&BombardTarget::Headquarters)
        );
        world.systems[system].dat_id = DatId::new(0x9000_0091);
        assert!(
            targets(&world, system, BombardKind::Military).contains(&BombardTarget::Headquarters)
        );
    }

    #[test]
    fn a_hit_on_the_headquarters_removes_it_and_reports_it() {
        // FUN_0058df60's HQ arm runs FUN_00517cd0 (support_shift::remove_headquarters).
        let (mut world, system) = world(Some(Faction::Alliance));
        world.systems[system].is_headquarters = true;
        let mut report = StrikeReport::default();
        apply_hit(&mut world, system, BombardTarget::Headquarters, &mut report);
        assert!(report.headquarters);
        assert!(!world.systems[system].is_headquarters);
    }

    #[test]
    fn only_the_holders_production_facilities_are_civilian_targets() {
        // FUN_0058cba0's production iterator takes the holder's only.
        let (mut world, system) = world(Some(Faction::Alliance));
        let keys = [Faction::Alliance, Faction::Empire].map(|side| {
            let key =
                world
                    .production_facilities
                    .insert(crate::world::ProductionFacilityInstance {
                        side,
                        ..crate::world::ProductionFacilityInstance::default()
                    });
            world.systems[system].production_facilities.push(key);
            key
        });
        assert_eq!(
            targets(&world, system, BombardKind::Civilian),
            vec![BombardTarget::Facility(FacilityRef::Production(keys[0]))]
        );
    }

    #[test]
    fn a_facility_valued_ten_survives_every_hit() {
        // FUN_0058cf80: a facility reads its class bombardment_defense.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 40, 100, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            yard(&mut world, system, holder, 10);
            let report = run(&mut world, system, BombardKind::Civilian, &[attacker]);
            assert_eq!(report.hits, 40, "{holder:?}");
            assert!(report.facilities.is_empty(), "{holder:?}");
            assert_eq!(
                world.systems[system].manufacturing_facilities.len(),
                1,
                "{holder:?}"
            );
        }
    }

    #[test]
    fn the_report_names_the_system_order_and_both_sides() {
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 1, 100, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            let report = run(&mut world, system, BombardKind::Civilian, &[attacker]);
            assert_eq!(report.system, Some(system), "{holder:?}");
            assert_eq!(report.kind, Some(BombardKind::Civilian), "{holder:?}");
            assert_eq!(report.bombarder, Some(bombarder), "{holder:?}");
            assert_eq!(report.holder, Some(holder), "{holder:?}");
        }
    }

    #[test]
    fn only_a_general_of_the_holder_standing_at_the_system_leads_its_defence() {
        // FUN_00509330: the first living General at the system, not aboard
        // a fleet, of the holder's side.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 5, 100_000, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            let decoys: [fn(&mut Character); 4] = [
                |c| c.command_kind = CommandKind::Admiral,
                |c| c.is_killed = true,
                |c| c.is_alliance = !c.is_alliance,
                |c| c.current_system = None,
            ];
            for set in decoys {
                character(&mut world, system, |c| {
                    c.command_kind = CommandKind::General;
                    c.leadership.base = 80;
                    c.is_alliance = holder == Faction::Alliance;
                    set(c);
                });
            }
            let lnr = ClassStats {
                attack_strength: 60,
                ..ClassStats::default()
            };
            defence(&mut world, system, holder, LNR_FAMILY, lnr);
            let report = run(&mut world, system, BombardKind::Military, &[attacker]);
            assert_eq!(report.defence_damage, 60, "{holder:?}");
        }
    }

    #[test]
    fn only_an_admiral_of_the_fleets_side_aboard_multiplies_its_hits() {
        // FUN_004fd790(fleet, 2): an Admiral aboard, alive, of the fleet's
        // side; a General aboard does not count.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 3, 100, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            let general = character(&mut world, system, |c| {
                c.command_kind = CommandKind::General;
                c.is_alliance = bombarder == Faction::Alliance;
                c.leadership.base = 80;
                c.current_fleet = Some(attacker);
            });
            let dead = character(&mut world, system, |c| {
                c.command_kind = CommandKind::Admiral;
                c.is_alliance = bombarder == Faction::Alliance;
                c.leadership.base = 80;
                c.is_killed = true;
                c.current_fleet = Some(attacker);
            });
            world.fleets[attacker].characters.extend([general, dead]);
            regiment(&mut world, system, holder, 10);
            let report = run(&mut world, system, BombardKind::Military, &[attacker]);
            assert_eq!(report.hits, 3, "{holder:?}");
        }
    }

    #[test]
    fn every_battery_fires_at_one_of_the_bombarding_ships() {
        // FUN_0058dc10: each battery draws a ship over 0..count - 1.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            for step in 0..40 {
                let (mut world, system) = world(Some(holder));
                let armed = ship_class(&mut world, 5, 10_000, 0, 0x14);
                let attacker = fleet(&mut world, system, bombarder, &[armed, armed]);
                let lnr = ClassStats {
                    attack_strength: 60,
                    ..ClassStats::default()
                };
                defence(&mut world, system, holder, LNR_FAMILY, lnr);
                assert!(flag_fleet(&mut world, attacker, false), "{holder:?}");
                let report = bombard(
                    &mut world,
                    system,
                    BombardKind::Military,
                    bombarder,
                    &[attacker],
                    f64::from(step) / 40.0,
                );
                let lost: i32 = world.fleets[attacker]
                    .capital_ships
                    .iter()
                    .map(|ship| 10_000 - ship.hull_current)
                    .sum();
                assert_eq!((report.defence_damage, lost), (60, 60), "{holder:?}");
            }
        }
    }

    #[test]
    fn lnr_fire_that_empties_a_hull_destroys_the_ship_after_the_run() {
        // FUN_0058d540: a hull at 0 runs the destroy slot; the ship leaves
        // its fleet once the run ends.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let frail = ship_class(&mut world, 5, 50, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[frail]);
            let lnr = ClassStats {
                attack_strength: 60,
                ..ClassStats::default()
            };
            defence(&mut world, system, holder, LNR_FAMILY, lnr);
            let report = run(&mut world, system, BombardKind::Military, &[attacker]);
            assert_eq!(report.ships_destroyed, 1, "{holder:?}");
            assert!(
                world.fleets[attacker].capital_ships.is_empty(),
                "{holder:?}"
            );
            assert_eq!(report.hits, 0, "{holder:?}");
        }
    }

    #[test]
    fn a_carrier_the_batteries_destroy_adds_nothing_for_its_squadrons() {
        // FUN_004fd6b0 sums the active ships (FUN_00502db0) and each one's
        // squadrons (FUN_005039d0); the effect phase (FUN_0058df60) runs
        // after the defence phase has destroyed the carrier.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let mut outcomes = std::collections::BTreeSet::new();
            for step in 0..40 {
                let (mut world, system) = world(Some(holder));
                let frail = ship_class(&mut world, 0, 50, 0, 0x14);
                let sturdy = ship_class(&mut world, 5, 10_000, 0, 0x14);
                let attacker = fleet(&mut world, system, bombarder, &[frail, sturdy]);
                let squadron = world.fighter_classes.insert(crate::world::FighterClass {
                    bombardment_defense: 2,
                    is_alliance: bombarder == Faction::Alliance,
                    ..crate::world::FighterClass::default()
                });
                let carried = &mut world.fleets[attacker];
                carried.capital_ships[0].tag = 1;
                carried.capital_ships[1].tag = 2;
                carried.fighters.push(crate::world::FighterEntry {
                    class: squadron,
                    count: 3,
                    carrier: 1,
                });
                let lnr = ClassStats {
                    attack_strength: 60,
                    ..ClassStats::default()
                };
                defence(&mut world, system, holder, LNR_FAMILY, lnr);
                regiment(&mut world, system, holder, 10);
                assert!(flag_fleet(&mut world, attacker, false));
                let report = bombard(
                    &mut world,
                    system,
                    BombardKind::Military,
                    bombarder,
                    &[attacker],
                    f64::from(step) / 40.0,
                );
                let expected = if report.ships_destroyed == 1 {
                    5
                } else {
                    5 + 6
                };
                assert_eq!(report.hits, expected, "{bombarder:?} step {step}");
                outcomes.insert(report.ships_destroyed);
            }
            assert_eq!(
                outcomes.into_iter().collect::<Vec<_>>(),
                [0, 1],
                "{bombarder:?}"
            );
        }
    }

    #[test]
    fn kdy_overflow_zeroes_only_the_ship_it_hit_across_two_fleets() {
        // FUN_0058d540's +0x1dc write is per ship; the other fleet's ship
        // keeps its strength, so the hits are one ship's worth.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            for step in 0..40 {
                let (mut world, system) = world(Some(holder));
                let armed = ship_class(&mut world, 5, 100, 0, 0x14);
                let first = fleet(&mut world, system, bombarder, &[armed]);
                let second = fleet(&mut world, system, bombarder, &[armed]);
                let kdy = ClassStats {
                    attack_strength: 2000,
                    ..ClassStats::default()
                };
                defence(&mut world, system, holder, KDY_FAMILY, kdy);
                regiment(&mut world, system, holder, 10);
                for each in [first, second] {
                    assert!(flag_fleet(&mut world, each, false), "{holder:?}");
                }
                let report = bombard(
                    &mut world,
                    system,
                    BombardKind::Military,
                    bombarder,
                    &[first, second],
                    f64::from(step) / 40.0,
                );
                assert_eq!(report.hits, 5, "{holder:?} roll {step}");
            }
        }
    }

    #[test]
    fn a_run_without_hits_takes_no_collateral() {
        // FUN_0058df60 returns before the collateral when the hits are 0.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let kills: Vec<usize> = (0..40)
                .map(|step| {
                    let (mut world, system) = world(Some(holder));
                    let unarmed = ship_class(&mut world, 0, 100, 0, 0x14);
                    let attacker = fleet(&mut world, system, bombarder, &[unarmed]);
                    regiment(&mut world, system, holder, 10);
                    yard(&mut world, system, holder, 0);
                    assert!(flag_fleet(&mut world, attacker, false), "{holder:?}");
                    bombard(
                        &mut world,
                        system,
                        BombardKind::Military,
                        bombarder,
                        &[attacker],
                        f64::from(step) / 40.0,
                    )
                    .facilities
                    .len()
                })
                .collect();
            assert!(
                kills.iter().all(|&count| count == 0),
                "{holder:?} {kills:?}"
            );
        }
    }

    #[test]
    fn one_fleet_with_strength_passes_the_gate() {
        // FUN_00582440 ends at the first passing fleet; zero strength is 8/6.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let idle = ship_class(&mut world, 0, 100, 0, 0x14);
            let armed = ship_class(&mut world, 3, 100, 0, 0x14);
            let weak = fleet(&mut world, system, bombarder, &[idle]);
            let strong = fleet(&mut world, system, bombarder, &[armed]);
            assert_eq!(
                gate(&world, &[weak, strong], BombardKind::General),
                Ok(()),
                "{holder:?}"
            );
            assert_eq!(
                gate(&world, &[weak], BombardKind::General),
                Err(BombardRefusal::NoStrength),
                "{holder:?}"
            );
        }
    }

    #[test]
    fn destroy_system_needs_a_death_star_ship() {
        // FUN_00582440: 0x223 also needs a family 0x18-0x1b ship (8/8).
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 3, 100, 0, 0x14);
            let star = ship_class(&mut world, 15, 100, 0, 0x18);
            let plain = fleet(&mut world, system, bombarder, &[armed]);
            assert_eq!(
                gate(&world, &[plain], BombardKind::DestroySystem),
                Err(BombardRefusal::NoDeathStar),
                "{holder:?}"
            );
            let death_star = fleet(&mut world, system, bombarder, &[star]);
            assert_eq!(
                gate(&world, &[plain, death_star], BombardKind::DestroySystem),
                Ok(()),
                "{holder:?}"
            );
            match open(&world, system, BombardKind::DestroySystem) {
                Opening::Idle => {}
                other => panic!("no fleet is flagged yet: {other:?}"),
            }
            assert!(flag_fleet(&mut world, death_star, false), "{holder:?}");
            assert_eq!(
                open(&world, system, BombardKind::DestroySystem),
                Opening::Kill { bombarder },
                "{holder:?}"
            );
        }
    }

    #[test]
    fn a_team_across_two_holders_is_refused() {
        // FUN_00553770: the order's systems must share one holder (1/0x14).
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, held) = world(Some(holder));
            let neutral = world.systems.insert(System::default());
            let armed = ship_class(&mut world, 3, 100, 0, 0x14);
            let first = fleet(&mut world, held, bombarder, &[armed]);
            let second = fleet(&mut world, neutral, bombarder, &[armed]);
            assert_eq!(
                gate(&world, &[first, second], BombardKind::General),
                Err(BombardRefusal::MixedHolders),
                "{holder:?}"
            );
        }
    }

    #[test]
    fn a_fleet_in_transit_is_not_flagged() {
        // FUN_004ffa70: an en-route ship refuses the fleet command (1/0x11).
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 3, 100, 0, 0x14);
            let mover = fleet(&mut world, system, bombarder, &[armed]);
            assert!(!flag_fleet(&mut world, mover, true), "{holder:?}");
            assert!(!world.fleets[mover].bombarding, "{holder:?}");
        }
    }

    const KINDS: [BombardKind; 4] = [
        BombardKind::Military,
        BombardKind::Civilian,
        BombardKind::General,
        BombardKind::DestroySystem,
    ];

    #[test]
    fn the_menu_enables_an_armed_fleets_orders_and_destroy_system_only_with_a_death_star() {
        // FUN_00581e80 -> FUN_00582440 for either side (VM Selonia: three
        // rows enabled and Destroy System disabled; VM Yaga Minor: all four
        // with the Death Star in orbit).
        let movement = crate::movement::MovementState::new();
        for (side, holder) in [
            (Faction::Empire, Faction::Alliance),
            (Faction::Alliance, Faction::Empire),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 3, 100, 0, 0x14);
            let star = ship_class(&mut world, 15, 100, 0, 0x18);
            let plain = fleet(&mut world, system, side, &[armed]);
            for kind in &KINDS[..3] {
                assert_eq!(
                    order_enabled(&world, &movement, &[plain], *kind, side),
                    Ok(()),
                    "{side:?} {kind:?}"
                );
            }
            assert_eq!(
                order_enabled(
                    &world,
                    &movement,
                    &[plain],
                    BombardKind::DestroySystem,
                    side
                ),
                Err(BombardRefusal::NoDeathStar),
                "{side:?}"
            );
            let death_star = fleet(&mut world, system, side, &[star]);
            for kind in KINDS {
                assert_eq!(
                    order_enabled(&world, &movement, &[death_star], kind, side),
                    Ok(()),
                    "{side:?} {kind:?}"
                );
            }
        }
    }

    #[test]
    fn the_menu_refuses_every_order_for_a_fleet_without_bombard_strength() {
        // FUN_00582440's 8/6 (VM Ord Pardon: an Alliance fleet of strength 0
        // shows Planetary Bombardment disabled).
        let movement = crate::movement::MovementState::new();
        for (side, holder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let idle = ship_class(&mut world, 0, 100, 0, 0x14);
            let unarmed = fleet(&mut world, system, side, &[idle]);
            for kind in &KINDS[..3] {
                assert_eq!(
                    order_enabled(&world, &movement, &[unarmed], *kind, side),
                    Err(BombardRefusal::NoStrength),
                    "{side:?} {kind:?}"
                );
            }
        }
    }

    #[test]
    fn the_menu_refuses_every_order_at_a_destroyed_system() {
        // FUN_00553770 runs each 0x224 command's +0x18: FUN_00563620 ->
        // FUN_00507f10 refuses a destroyed system (1/0x22), whoever holds it.
        let movement = crate::movement::MovementState::new();
        for side in [Faction::Empire, Faction::Alliance] {
            let (mut world, system) = world(None);
            let star = ship_class(&mut world, 15, 100, 0, 0x18);
            let death_star = fleet(&mut world, system, side, &[star]);
            world.systems[system].is_destroyed = true;
            for kind in KINDS {
                assert_eq!(
                    order_enabled(&world, &movement, &[death_star], kind, side),
                    Err(BombardRefusal::Destroyed),
                    "{side:?} {kind:?}"
                );
            }
        }
    }

    #[test]
    fn the_menu_refuses_an_order_whose_fleet_command_refuses_its_fleet() {
        // FUN_0051fe20 -> FUN_00553770 over the fleet commands: an empty team
        // is 1/0x16; FUN_004ffa70 refuses another side's fleet and a fleet en
        // route (FUN_004f9860, 1/1) and a fleet with no ship (8/1,
        // FUN_004ffa70.c:15-60).
        for side in [Faction::Empire, Faction::Alliance] {
            let other = if side == Faction::Empire {
                Faction::Alliance
            } else {
                Faction::Empire
            };
            let (mut world, system) = world(Some(other));
            let armed = ship_class(&mut world, 3, 100, 0, 0x14);
            let own = fleet(&mut world, system, side, &[armed]);
            let theirs = fleet(&mut world, system, other, &[armed]);
            let empty = fleet(&mut world, system, side, &[]);
            let mut movement = crate::movement::MovementState::new();
            let general = BombardKind::General;
            assert_eq!(
                order_enabled(&world, &movement, &[], general, side),
                Err(BombardRefusal::NoTeam)
            );
            assert_eq!(
                order_enabled(&world, &movement, &[theirs], general, side),
                Err(BombardRefusal::FleetRefused)
            );
            assert_eq!(
                order_enabled(&world, &movement, &[empty], general, side),
                Err(BombardRefusal::NoShip)
            );
            assert_eq!(
                order_enabled(&world, &movement, &[own], general, side),
                Ok(())
            );
            movement.order(own, system, system, 5);
            assert_eq!(
                order_enabled(&world, &movement, &[own], general, side),
                Err(BombardRefusal::FleetRefused)
            );
        }
    }

    #[test]
    fn bombard_refuses_a_fleet_whose_last_ship_is_unfinished_with_1_0x10_and_an_en_route_one_with_1_0x11_for_either_side(
    ) {
        // FUN_004ffa70.c:15-60 walks the fleet's ships in mode 3 until one is
        // completed and not en route; otherwise the last ship's status
        // stands: uncompleted 1/0x10, en route 1/0x11. An ordered ship is
        // uncompleted (FUN_004f7d50(.., 1, 0); the route-1 day-115 save:
        // +0x50 0x42), a ship on its own leg en route (FUN_00556430).
        let movement = crate::movement::MovementState::new();
        for side in [Faction::Empire, Faction::Alliance] {
            let other = if side == Faction::Empire {
                Faction::Alliance
            } else {
                Faction::Empire
            };
            let (mut world, system) = world(Some(other));
            let armed = ship_class(&mut world, 3, 100, 0, 0x14);
            let own = fleet(&mut world, system, side, &[armed, armed]);
            let general = BombardKind::General;
            let leg = Some(crate::world::ShipTransit {
                from: system,
                to: system,
                arrival_tick: 9,
            });
            world.fleets[own].capital_ships[0].transit = leg;
            world.fleets[own].capital_ships[1].completed = false;
            assert_eq!(
                order_enabled(&world, &movement, &[own], general, side),
                Err(BombardRefusal::LastShipUnfinished),
                "{side:?}"
            );
            world.fleets[own].capital_ships.swap(0, 1);
            assert_eq!(
                order_enabled(&world, &movement, &[own], general, side),
                Err(BombardRefusal::LastShipEnRoute),
                "{side:?}"
            );
            // One completed ship not en route passes the fleet.
            world.fleets[own].capital_ships[1].transit = None;
            assert_eq!(
                order_enabled(&world, &movement, &[own], general, side),
                Ok(()),
                "{side:?}"
            );
        }
    }

    #[test]
    fn the_fleet_command_flags_no_fleet_whose_ships_are_all_unfinished_for_either_side() {
        // FUN_004fec00 runs +0x1c8 (FUN_004fe020 -> FUN_004fdf70 ->
        // FUN_004ffa70.c:15-60) and sets +0x58 bit 6 only when the status
        // is still ok: a fleet of an unfinished ship (FUN_004f7d50(.., 1, 0);
        // the route-1 day-115 save: +0x50 0x42) stays unflagged.
        for side in [Faction::Empire, Faction::Alliance] {
            let (mut world, system) = world(None);
            let armed = ship_class(&mut world, 3, 100, 0, 0x14);
            let own = fleet(&mut world, system, side, &[armed]);
            world.fleets[own].capital_ships[0].completed = false;
            assert!(!flag_fleet(&mut world, own, false), "{side:?}");
            assert!(!world.fleets[own].bombarding, "{side:?}");
            world.fleets[own].capital_ships[0].completed = true;
            assert!(flag_fleet(&mut world, own, false), "{side:?}");
            assert!(world.fleets[own].bombarding, "{side:?}");
        }
    }

    #[test]
    fn each_bombard_refusal_carries_the_status_its_function_writes() {
        // The pairs the advisor reacts to, either side alike: FUN_00582440
        // (8/6, 8/8), FUN_00553770 (1/0x14, 1/0x16), FUN_00507f10 (1/0x22),
        // FUN_004f9860 through FUN_004ffa70 (1/1) and FUN_004ffa70.c:15-60
        // (8/1, 1/0x10, 1/0x11).
        for (refusal, status) in [
            (BombardRefusal::NoStrength, (8, 6)),
            (BombardRefusal::NoDeathStar, (8, 8)),
            (BombardRefusal::MixedHolders, (1, 0x14)),
            (BombardRefusal::Destroyed, (1, 0x22)),
            (BombardRefusal::NoTeam, (1, 0x16)),
            (BombardRefusal::FleetRefused, (1, 1)),
            (BombardRefusal::NoShip, (8, 1)),
            (BombardRefusal::LastShipUnfinished, (1, 0x10)),
            (BombardRefusal::LastShipEnRoute, (1, 0x11)),
        ] {
            assert_eq!(refusal.status(), status, "{refusal:?}");
        }
    }

    #[test]
    fn the_menu_refuses_a_team_across_two_holders() {
        // FUN_00553770 over the 0x224 commands: one holder (1/0x14).
        let movement = crate::movement::MovementState::new();
        for side in [Faction::Empire, Faction::Alliance] {
            let (mut world, held) = world(Some(Faction::Alliance));
            let neutral = world.systems.insert(System::default());
            let armed = ship_class(&mut world, 3, 100, 0, 0x14);
            let first = fleet(&mut world, held, side, &[armed]);
            let second = fleet(&mut world, neutral, side, &[armed]);
            assert_eq!(
                order_enabled(
                    &world,
                    &movement,
                    &[first, second],
                    BombardKind::Military,
                    side
                ),
                Err(BombardRefusal::MixedHolders)
            );
        }
    }

    #[test]
    fn two_flagged_sides_at_one_system_run_nothing() {
        // FUN_0058d800 returns 0 when two sides have flagged fleets.
        let (mut world, system) = world(None);
        let armed = ship_class(&mut world, 3, 100, 0, 0x14);
        let empire = fleet(&mut world, system, Faction::Empire, &[armed]);
        let alliance = fleet(&mut world, system, Faction::Alliance, &[armed]);
        flag_fleet(&mut world, empire, false);
        flag_fleet(&mut world, alliance, false);
        assert_eq!(open(&world, system, BombardKind::General), Opening::Idle);
    }

    #[test]
    fn hits_are_strength_times_the_admirals_leadership_less_the_shield_generators() {
        // FUN_0058df60: strength x (leadership / GNPRTB 0x601 + 1), less the
        // family-0x24 generators' DEFFACSD shield (+0x60); the 0x25 shield
        // does not count.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 3, 100, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            let admiral = character(&mut world, system, |c| {
                c.command_kind = CommandKind::Admiral;
                c.is_alliance = bombarder == Faction::Alliance;
                c.leadership.base = 80;
                c.current_fleet = Some(attacker);
            });
            world.fleets[attacker].characters.push(admiral);
            let shield = ClassStats {
                shield_strength: 4,
                ..ClassStats::default()
            };
            defence(&mut world, system, holder, 0x24, shield);
            let death_star_shield = ClassStats {
                shield_strength: 7,
                ..ClassStats::default()
            };
            defence(&mut world, system, holder, 0x25, death_star_shield);
            regiment(&mut world, system, holder, 10);
            let report = run(&mut world, system, BombardKind::General, &[attacker]);
            assert_eq!(report.hits, 3 * (80 / 40 + 1) - 4, "{holder:?}");
        }
    }

    #[test]
    fn a_target_valued_ten_survives_every_hit() {
        // FUN_0055e970: a hit kills when v < a draw over 1..=10.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 40, 100, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            let troop = regiment(&mut world, system, holder, 10);
            let report = run(&mut world, system, BombardKind::Military, &[attacker]);
            assert_eq!(report.hits, 40, "{holder:?}");
            assert!(world.troops.contains_key(troop), "{holder:?}");
            assert!(report.regiments.is_empty(), "{holder:?}");
        }
    }

    #[test]
    fn a_civilian_kill_sets_the_support_loss_and_a_military_one_does_not() {
        // +0x28 is set by a 0x28-0x2f facility, energy or raw material.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 5, 100, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            let troop = regiment(&mut world, system, holder, 0);
            let report = run(&mut world, system, BombardKind::Military, &[attacker]);
            assert_eq!(report.regiments, vec![(troop, holder)], "{holder:?}");
            assert!(!report.run.support_loss, "{holder:?}");

            yard(&mut world, system, holder, 0);
            clear_flags(&mut world, system, bombarder);
            let report = run(&mut world, system, BombardKind::Civilian, &[attacker]);
            assert_eq!(report.facilities.len(), 1, "{holder:?}");
            assert!(report.run.support_loss, "{holder:?}");
            assert!(
                world.systems[system].manufacturing_facilities.is_empty(),
                "{holder:?}"
            );
        }
    }

    #[test]
    fn only_the_holders_objects_are_targets() {
        // FUN_0058cba0's iterators take the holder's objects only.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 5, 100, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            let own = regiment(&mut world, system, bombarder, 0);
            let report = run(&mut world, system, BombardKind::Military, &[attacker]);
            assert!(report.regiments.is_empty(), "{holder:?}");
            assert!(world.troops.contains_key(own), "{holder:?}");
        }
    }

    #[test]
    fn a_side_may_bombard_its_own_system() {
        // No check compares holder and bombarder (FUN_00582440, FUN_00507f10).
        for side in [Faction::Empire, Faction::Alliance] {
            let (mut world, system) = world(Some(side));
            let armed = ship_class(&mut world, 5, 100, 0, 0x14);
            let attacker = fleet(&mut world, system, side, &[armed]);
            let own = regiment(&mut world, system, side, 0);
            assert_eq!(
                gate(&world, &[attacker], BombardKind::Military),
                Ok(()),
                "{side:?}"
            );
            let report = run(&mut world, system, BombardKind::Military, &[attacker]);
            assert_eq!(report.regiments, vec![(own, side)], "{side:?}");
        }
    }

    #[test]
    fn each_successful_hit_lowers_energy_or_raw_material_by_one() {
        // FUN_0058cf20, FUN_0058cf50; values GNPRTB 0x604 and 0x605, here 0.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            world.gnprtb =
                GnprtbParams::new(vec![entry(0x601, 40), entry(0x602, 1), entry(0x603, 10)]);
            world.systems[system].total_energy = 10;
            world.systems[system].raw_materials = 10;
            let armed = ship_class(&mut world, 3, 100, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            let report = run(&mut world, system, BombardKind::General, &[attacker]);
            assert_eq!(report.hits, 3, "{holder:?}");
            assert_eq!(report.energy + report.raw_material, 3, "{holder:?}");
            let sys = &world.systems[system];
            assert_eq!(sys.total_energy + sys.raw_materials, 17, "{holder:?}");
            assert!(report.run.support_loss, "{holder:?}");
        }
    }

    #[test]
    fn kdy_fire_past_the_shields_zeroes_a_ships_strength_and_spares_its_hull() {
        // FUN_0058d540: KDY overflow stays in the first accumulator, which
        // zeroes slot +0x1dc (FUN_00500f00); it never reaches the hull.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 5, 200, 50, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            let kdy = ClassStats {
                attack_strength: 100,
                ..ClassStats::default()
            };
            defence(&mut world, system, holder, 0x22, kdy);
            let report = run(&mut world, system, BombardKind::Military, &[attacker]);
            assert_eq!(report.hits, 0, "{holder:?}");
            assert_eq!(
                world.fleets[attacker].capital_ships[0].hull_current, 200,
                "{holder:?}"
            );
        }
    }

    #[test]
    fn lnr_fire_past_the_shields_lowers_the_hull_and_the_general_multiplies_it() {
        // FUN_0058d540: LNR damage left after the shields lowers the hull;
        // FUN_0055e930: attack x (General's leadership / GNPRTB 0x600 + 1).
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 5, 200, 50, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            character(&mut world, system, |c| {
                c.command_kind = CommandKind::General;
                c.leadership.base = 40;
                c.is_alliance = holder == Faction::Alliance;
            });
            let lnr = ClassStats {
                attack_strength: 60,
                ..ClassStats::default()
            };
            defence(&mut world, system, holder, 0x23, lnr);
            let report = run(&mut world, system, BombardKind::Military, &[attacker]);
            assert_eq!(report.defence_damage, 120, "{holder:?}");
            assert_eq!(
                world.fleets[attacker].capital_ships[0].hull_current,
                200 - (120 - 50),
                "{holder:?}"
            );
        }
    }

    #[test]
    fn hull_damage_lowers_a_ships_strength_in_proportion() {
        // FUN_00500f00: modifier - modifier x hull damage / max hull.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 10, 200, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            world.fleets[attacker].capital_ships[0].hull_current = 100;
            assert_eq!(fleet_strength(&world, attacker), 5, "{holder:?}");
        }
    }

    #[test]
    fn only_minor_characters_directly_at_the_system_are_rolled() {
        // FUN_0058d990: minor characters (0x38-0x3b), not aboard a fleet, not
        // in transit, any side; GNPRTB 0x606 here 100 percent.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            world.gnprtb = GnprtbParams::new(vec![entry(0x606, 100)]);
            let armed = ship_class(&mut world, 3, 100, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            let minor = character(&mut world, system, |_| {});
            let imperial = character(&mut world, system, |c| c.is_empire = true);
            let major = character(&mut world, system, |c| c.is_major = true);
            let aboard = character(&mut world, system, |c| c.current_fleet = Some(attacker));
            let travelling = character(&mut world, system, |c| c.en_route = true);
            let killed = roll_characters(&mut world, system, 0.5);
            assert_eq!(killed, vec![minor, imperial], "{holder:?}");
            for survivor in [major, aboard, travelling] {
                assert!(!world.characters[survivor].is_killed, "{holder:?}");
            }
        }
    }

    #[test]
    fn a_regiments_loss_carries_its_value_and_whose_system_it_died_at() {
        // FUN_0058ce70: cause 0xf at an enemy system, 0x10 at the
        // bombarder's own; FUN_00508c60 reads the regiment's slot +0x1c0
        // (TROOPSD uprising_defense).
        use crate::blockade::{UnitLoss, REASON_BOMBARDMENT, REASON_OWN_BOMBARDMENT};
        for (holder, bombarder, reason) in [
            (Faction::Alliance, Faction::Empire, REASON_BOMBARDMENT),
            (Faction::Empire, Faction::Alliance, REASON_BOMBARDMENT),
            (Faction::Empire, Faction::Empire, REASON_OWN_BOMBARDMENT),
            (Faction::Alliance, Faction::Alliance, REASON_OWN_BOMBARDMENT),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 5, 100, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            let key = regiment(&mut world, system, holder, 0);
            let class = world.troops[key].class_dat_id;
            if let Some(value) = world.buildable_classes.get_mut(&class) {
                value.stats.uprising_defense = 7;
            }
            let report = run(&mut world, system, BombardKind::Military, &[attacker]);
            assert_eq!(
                report.losses,
                vec![UnitLoss {
                    side: holder,
                    value: 7,
                    reason,
                }],
                "{bombarder:?} at {holder:?}"
            );
        }
    }

    #[test]
    fn a_ship_destroyed_by_defence_fire_is_the_bombarders_loss() {
        // FUN_0058d540 -> FUN_00501ae0(hull loss, 0xf); its slot +0x1c0 is
        // CAPSHPSD uprising_defense.
        use crate::blockade::{UnitLoss, REASON_BOMBARDMENT};
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 5, 50, 0, 0x14);
            world.capital_ship_classes[armed].uprising_defense = 4;
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            let lnr = ClassStats {
                attack_strength: 60,
                ..ClassStats::default()
            };
            defence(&mut world, system, holder, 0x23, lnr);
            let report = run(&mut world, system, BombardKind::Military, &[attacker]);
            assert_eq!(report.ships_destroyed, 1);
            assert_eq!(
                report.losses,
                vec![UnitLoss {
                    side: bombarder,
                    value: 4,
                    reason: REASON_BOMBARDMENT,
                }],
                "{bombarder:?}"
            );
        }
    }

    // FUN_0058ca70.c:25-33 walks the usable flagged fleets' usable ships
    // (mode 1) for the bombarding list (+0x1c) that the batteries fire at,
    // and FUN_004fd6b0.c:23 sums slot +0x1dc over the same (mode 1): an
    // unfinished ship or one on its own leg neither bombards nor draws fire.
    #[test]
    fn an_unfinished_or_travelling_ship_neither_bombards_nor_draws_fire_for_either_side() {
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 5, 10_000, 0, 0x14);
            let idle = ship_class(&mut world, 7, 10_000, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed, idle, idle]);
            world.fleets[attacker].capital_ships[1].completed = false;
            world.fleets[attacker].capital_ships[2].transit = Some(crate::world::ShipTransit {
                from: system,
                to: system,
                arrival_tick: 9,
            });
            assert_eq!(fleet_strength(&world, attacker), 5, "{bombarder:?}");
            let lnr = ClassStats {
                attack_strength: 60,
                ..ClassStats::default()
            };
            for _ in 0..4 {
                defence(&mut world, system, holder, LNR_FAMILY, lnr);
            }
            regiment(&mut world, system, holder, 10);
            let report = run(&mut world, system, BombardKind::Military, &[attacker]);
            assert_eq!(report.hits, 5, "{bombarder:?}");
            let hulls: Vec<i32> = world.fleets[attacker]
                .capital_ships
                .iter()
                .map(|ship| ship.hull_current)
                .collect();
            assert_eq!(hulls, [10_000 - 4 * 60, 10_000, 10_000], "{bombarder:?}");
        }
    }

    // FUN_0058d440's Death Star test runs over FUN_0058ca70's mode-1 walk,
    // as the gate's FUN_005003d0(.., 1) does (FUN_00582440.c:70): an
    // unfinished Death Star kills nothing.
    #[test]
    fn an_unfinished_death_star_opens_no_kill_for_either_side() {
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 5, 100, 0, 0x14);
            let death_star = ship_class(&mut world, 15, 25_000, 2000, 0x18);
            let attacker = fleet(&mut world, system, bombarder, &[armed, death_star]);
            world.fleets[attacker].capital_ships[1].completed = false;
            assert!(flag_fleet(&mut world, attacker, false));
            assert!(
                matches!(
                    open(&world, system, BombardKind::DestroySystem),
                    Opening::Run { .. }
                ),
                "{bombarder:?}"
            );
            world.fleets[attacker].capital_ships[1].completed = true;
            assert_eq!(
                open(&world, system, BombardKind::DestroySystem),
                Opening::Kill { bombarder },
                "{bombarder:?}"
            );
        }
    }

    // A Death Star's kill clears its usable bit (FUN_004f7b80 ->
    // FUN_004f7410), and event 0x301 reruns bit 11 at once (FUN_00514510).
    #[test]
    fn a_death_star_the_batteries_destroy_clears_its_sectors_bit_for_either_side() {
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let frail = ship_class(&mut world, 5, 50, 0, 0x18);
            let attacker = fleet(&mut world, system, bombarder, &[frail]);
            crate::death_star::refresh_sectors(&mut world);
            assert_eq!(world.death_star_sectors.len(), 1, "{bombarder:?}");
            let lnr = ClassStats {
                attack_strength: 60,
                ..ClassStats::default()
            };
            defence(&mut world, system, holder, LNR_FAMILY, lnr);
            regiment(&mut world, system, holder, 10);
            let report = run(&mut world, system, BombardKind::Military, &[attacker]);
            assert_eq!(report.ships_destroyed, 1, "{bombarder:?}");
            assert!(world.death_star_sectors.is_empty(), "{bombarder:?}");
        }
    }

    // The squadrons aboard a ship the batteries destroy die with its reason
    // byte: its hook +0x11c (FUN_004fb520) calls each child's slot +0xb4,
    // FUN_004f8710 for a squadron (vtable 0x0065def0), which passes a
    // completed child to FUN_004f7ed0 with the container's reason, 0xf
    // here; FUN_00508c60 then adds each one's slot +0x1c0 (FIGHTSD
    // uprising_defense) to B.
    #[test]
    fn the_squadrons_aboard_a_ship_the_batteries_destroy_are_the_bombarders_losses_for_either_side()
    {
        use crate::blockade::{UnitLoss, REASON_BOMBARDMENT};
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let frail = ship_class(&mut world, 5, 50, 0, 0x14);
            world.capital_ship_classes[frail].uprising_defense = 4;
            let attacker = fleet(&mut world, system, bombarder, &[frail]);
            let squadron = world.fighter_classes.insert(crate::world::FighterClass {
                uprising_defense: 3,
                is_alliance: bombarder == Faction::Alliance,
                ..crate::world::FighterClass::default()
            });
            let carrier = crate::carriage::tag_ship(&mut world, attacker, 0).unwrap();
            world.fleets[attacker]
                .fighters
                .push(crate::world::FighterEntry {
                    class: squadron,
                    count: 2,
                    carrier,
                });
            let lnr = ClassStats {
                attack_strength: 60,
                ..ClassStats::default()
            };
            defence(&mut world, system, holder, LNR_FAMILY, lnr);
            regiment(&mut world, system, holder, 10);
            let report = run(&mut world, system, BombardKind::Military, &[attacker]);
            let loss = |value| UnitLoss {
                side: bombarder,
                value,
                reason: REASON_BOMBARDMENT,
            };
            assert_eq!(report.ships_destroyed, 1, "{bombarder:?}");
            assert_eq!(report.losses, [loss(4), loss(3), loss(3)], "{bombarder:?}");
        }
    }

    #[test]
    fn each_object_a_run_destroys_at_the_system_is_reported_on_the_system_side() {
        // FUN_00497380: event 0x302 for an object whose container is the
        // system goes to the Operation Report's +0x80, the regiments and
        // facilities a hit destroys (FUN_0058ce70) and the HQ
        // (FUN_00517cd0).
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 5, 100, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            let troop = regiment(&mut world, system, holder, 0);
            let troop_class = world.troops[troop].class_dat_id;
            let report = run(&mut world, system, BombardKind::Military, &[attacker]);
            assert_eq!(
                report
                    .system_destroyed
                    .iter()
                    .map(|unit| unit.dat_id)
                    .collect::<Vec<_>>(),
                [troop_class],
                "{bombarder:?}"
            );
            assert!(report.fleet_destroyed.is_empty());

            yard(&mut world, system, holder, 0);
            clear_flags(&mut world, system, bombarder);
            let report = run(&mut world, system, BombardKind::Civilian, &[attacker]);
            assert_eq!(
                report
                    .system_destroyed
                    .iter()
                    .map(|unit| unit.dat_id)
                    .collect::<Vec<_>>(),
                [DatId::new(0x2800_0000)],
                "{bombarder:?}"
            );
        }
        let (mut world, system) = world(Some(Faction::Alliance));
        world.systems[system].is_headquarters = true;
        let mut report = StrikeReport::default();
        apply_hit(&mut world, system, BombardTarget::Headquarters, &mut report);
        assert_eq!(
            report
                .system_destroyed
                .iter()
                .map(|unit| unit.dat_id)
                .collect::<Vec<_>>(),
            [DatId::new(HEADQUARTERS_CLASS)]
        );
    }

    #[test]
    fn a_ship_lost_to_defence_fire_and_its_squadrons_are_reported_on_the_fleet_side() {
        // FUN_00497380: an object aboard a ship or fleet goes to +0x88; the
        // squadrons the lost ship carried go with it
        // (carriage::drop_lost_squadrons).
        use crate::world::{FighterClass, FighterEntry};
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 5, 50, 0, 0x14);
            world.capital_ship_classes[armed].dat_id = DatId::new(0x1400_0040);
            world.capital_ship_classes[armed].name = "Cruiser".into();
            let wing = world.fighter_classes.insert(FighterClass {
                dat_id: DatId::new(0x1c00_0001),
                name: "Wing".into(),
                ..FighterClass::default()
            });
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            world.fleets[attacker].capital_ships[0].tag = 3;
            world.fleets[attacker].fighters.push(FighterEntry {
                class: wing,
                count: 2,
                carrier: 3,
            });
            let lnr = ClassStats {
                attack_strength: 60,
                ..ClassStats::default()
            };
            defence(&mut world, system, holder, 0x23, lnr);
            let report = run(&mut world, system, BombardKind::Military, &[attacker]);
            let lost: Vec<_> = report
                .fleet_destroyed
                .iter()
                .map(|unit| unit.name.as_str())
                .collect();
            assert_eq!(lost, ["Cruiser", "Wing", "Wing"], "{bombarder:?}");
            assert!(report.system_destroyed.is_empty());
            assert!(world.fleets[attacker].fighters.is_empty());
        }
    }

    #[test]
    fn a_ship_the_batteries_destroy_is_listed_with_the_damaged_bit_it_died_with() {
        // FUN_0058d540 -> FUN_00501ae0: the hull loss is written first
        // (FUN_00501490, whose slot +0x29c -> +0x298 FUN_00501c00 sets
        // +0x50 bit 9 through FUN_004f7800 on both views), then the destroy
        // slot +0xac (FUN_004f71d0) writes only the cause +0x40. The report
        // window draws the dead ship from that view (FUN_0044e500,
        // FUN_0042c3b0), so its entry keeps the bit.
        use crate::operation_report::ReportObject;
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            let armed = ship_class(&mut world, 5, 50, 0, 0x14);
            let attacker = fleet(&mut world, system, bombarder, &[armed]);
            let lnr = ClassStats {
                attack_strength: 60,
                ..ClassStats::default()
            };
            defence(&mut world, system, holder, LNR_FAMILY, lnr);
            let report = run(&mut world, system, BombardKind::Military, &[attacker]);
            assert_eq!(report.ships_destroyed, 1, "{bombarder:?}");
            assert_eq!(
                report
                    .fleet_destroyed
                    .iter()
                    .map(|unit| unit.object)
                    .collect::<Vec<_>>(),
                [Some(ReportObject::Destroyed { damaged: true })],
                "{bombarder:?}"
            );
        }
    }

    #[test]
    fn the_sector_shift_reads_each_systems_systemsd_family_and_skips_bit_3() {
        // FUN_0055d900: SDPRTB 0x140a/0x140b for core (0x90-0x91) and
        // 0x1408/0x1409 for rim (0x92-0x93), the family the SYSTEMSD
        // record's (FUN_00518be0 slot +4), a system with +0x88 bit 3 left
        // out; GNPRTB 7705 (-20) at the system first. Values here: core
        // -4, rim -2, both limits 101.
        let sd = |id: u16, value: i32| crate::world::SdprtbEntry {
            parameter_id: u32::from(id),
            dev_alliance: value,
            dev_empire: value,
            alliance_sp_easy_alliance: value,
            alliance_sp_easy_empire: value,
            alliance_sp_medium_alliance: value,
            alliance_sp_medium_empire: value,
            alliance_sp_hard_alliance: value,
            alliance_sp_hard_empire: value,
            empire_sp_easy_alliance: value,
            empire_sp_easy_empire: value,
            empire_sp_medium_alliance: value,
            empire_sp_medium_empire: value,
            empire_sp_hard_alliance: value,
            empire_sp_hard_empire: value,
            multiplayer_alliance: value,
            multiplayer_empire: value,
        };
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            world.sdprtb = crate::world::SdprtbParams::new(vec![
                sd(SDPRTB_RIM_AMOUNT, -2),
                sd(SDPRTB_RIM_LIMIT, 101),
                sd(SDPRTB_CORE_AMOUNT, -4),
                sd(SDPRTB_CORE_LIMIT, 101),
            ]);
            let sector = world.systems[system].sector;
            let mut rim = || {
                let key = world.systems.insert(System {
                    dat_id: DatId::new(0x102),
                    sector,
                    is_populated: true,
                    popularity_alliance: 0.5,
                    popularity_empire: 0.5,
                    ..System::default()
                });
                world.sectors[sector].systems.push(key);
                world.system_families.insert(key, 0x92);
                key
            };
            let (open_rim, kept_out) = (rim(), rim());
            world.shift_excluded.insert(kept_out);
            finalize(
                &mut world,
                system,
                bombarder,
                BombardmentRun {
                    support_loss: true,
                    planet_destroyed: false,
                },
            );
            crate::support_shift::fire(&mut world, 1);
            let points = |key| crate::uprising::support_points(&world.systems[key], bombarder);
            assert_eq!(points(system), 26, "{bombarder:?}");
            assert_eq!(points(open_rim), 48, "{bombarder:?}");
            assert_eq!(points(kept_out), 50, "{bombarder:?}");
        }
    }

    #[test]
    fn clearing_the_flags_leaves_the_other_sides_fleets_flagged() {
        // FUN_004ffef0 mode 4 walks the bombarder's fleets only.
        for (cleared, kept) in [
            (Faction::Empire, Faction::Alliance),
            (Faction::Alliance, Faction::Empire),
        ] {
            let (mut world, system) = world(None);
            let armed = ship_class(&mut world, 3, 100, 0, 0x14);
            let cleared_fleet = fleet(&mut world, system, cleared, &[armed]);
            let kept_fleet = fleet(&mut world, system, kept, &[armed]);
            flag_fleet(&mut world, cleared_fleet, false);
            flag_fleet(&mut world, kept_fleet, false);
            clear_flags(&mut world, system, cleared);
            assert!(!world.fleets[cleared_fleet].bombarding, "{cleared:?}");
            assert!(world.fleets[kept_fleet].bombarding, "{cleared:?}");
        }
    }

    #[test]
    fn a_support_loss_moves_the_bombarders_support_at_the_system_by_gnprtb_7705() {
        // FUN_0058e320 -> FUN_0055a0e0: GNPRTB 7705 (-20) for the bombarder.
        for (holder, bombarder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system) = world(Some(holder));
            finalize(
                &mut world,
                system,
                bombarder,
                BombardmentRun {
                    support_loss: true,
                    planet_destroyed: false,
                },
            );
            assert_eq!(
                crate::uprising::support_points(&world.systems[system], bombarder),
                30,
                "{holder:?}"
            );
            let (mut world, system) = self::world(Some(holder));
            finalize(&mut world, system, bombarder, BombardmentRun::default());
            assert_eq!(
                crate::uprising::support_points(&world.systems[system], bombarder),
                50,
                "{holder:?}"
            );
        }
    }
}
