//! What a production manager can build and what an order costs: the model
//! behind the Build Selection window (`ghidra/notes/manufacturing-build-selection.md`).
//!
//! `FUN_00537ff0` maps a manager to the classes its yards build and
//! `FUN_0052e580` lists those its side has researched; `FUN_00538220`
//! prices an order as each class cost times the quantity and asks the
//! manager for the best completion and deployment times.

use crate::carriage::{Cargo, ProductRoom};
use crate::delivery::{transit_days, DeliveryState};
use crate::ids::{DatId, SystemKey};
use crate::manufacturing::{
    completion_day, BuildableKind, Destination, FacilityBuild, ManufacturingState, ProductionArea,
    QueueItem,
};
use crate::research::{ResearchState, TechType};
use crate::troop_transport::TroopTransportState;
use crate::world::GameWorld;

/// The class production family an area's yards build, and the yards' own
/// facility family (MANFACSD `production_family`, `FUN_00537ff0`: ships
/// `0x28`, regiments and special forces `0x29`, facilities `0x2a`).
#[must_use]
pub const fn area_family(area: ProductionArea) -> u8 {
    match area {
        ProductionArea::Shipyard => 0x28,
        ProductionArea::TrainingFacility => 0x29,
        ProductionArea::ConstructionYard => 0x2a,
    }
}

/// The research tree whose level gates an area's classes (side `+0x9c`,
/// `+0xa0`, `+0xa4` in `FUN_0052e580`).
const fn area_tech(area: ProductionArea) -> TechType {
    match area {
        ProductionArea::Shipyard => TechType::Ship,
        ProductionArea::TrainingFacility => TechType::Troop,
        ProductionArea::ConstructionYard => TechType::Facility,
    }
}

/// A catalog class as the kind an order names, for `is_alliance`'s side,
/// by its DAT family byte: TROOPSD `0x10`, SPECFCSD `0x3c`, DEFFACSD
/// `0x22..0x25`, MANFACSD `0x28..0x2a`, PROFACSD `0x2c..0x2d`
/// (`FUN_00458fe0`, `FUN_00537ff0`).
fn catalog_kind(class: DatId, is_alliance: bool) -> Option<BuildableKind> {
    let build = FacilityBuild { class, is_alliance };
    match class.family() {
        0x10 => Some(BuildableKind::Troop(class)),
        0x3c => Some(BuildableKind::SpecialForce(class)),
        0x22..=0x25 => Some(BuildableKind::DefenseFacility(build)),
        0x28..=0x2a => Some(BuildableKind::ManufacturingFacility(build)),
        0x2c..=0x2d => Some(BuildableKind::ProductionFacility(build)),
        _ => None,
    }
}

/// The classes `area`'s manager lists for a side: its family's classes that
/// side builds and has researched, in `DatId` order. hyp: `FUN_0051cb20`
/// walks the class tables in id order.
#[must_use]
pub fn listed_classes(
    world: &GameWorld,
    research: &ResearchState,
    area: ProductionArea,
    is_alliance: bool,
) -> Vec<BuildableKind> {
    let tech = area_tech(area);
    let unlocked = |order| research.is_unlocked(is_alliance, order, tech);
    let serves = |alliance: bool, empire: bool| if is_alliance { alliance } else { empire };
    let mut listed: Vec<(u32, BuildableKind)> = Vec::new();
    if area == ProductionArea::Shipyard {
        listed.extend(
            world
                .capital_ship_classes
                .iter()
                .filter(|(_, c)| serves(c.is_alliance, c.is_empire) && unlocked(c.research_order))
                .map(|(key, c)| (c.dat_id.raw(), BuildableKind::CapitalShip(key))),
        );
        listed.extend(
            world
                .fighter_classes
                .iter()
                .filter(|(_, c)| serves(c.is_alliance, c.is_empire) && unlocked(c.research_order))
                .map(|(key, c)| (c.dat_id.raw(), BuildableKind::Fighter(key))),
        );
    } else {
        listed.extend(
            world
                .buildable_classes
                .iter()
                .filter_map(|(&class, value)| {
                    let kind = catalog_kind(class, is_alliance)?;
                    (ProductionArea::of(kind) == area
                        && value.serves(is_alliance)
                        && unlocked(value.research_order))
                    .then_some((class.raw(), kind))
                }),
        );
    }
    listed.sort_unstable_by_key(|(id, _)| *id);
    listed.into_iter().map(|(_, kind)| kind).collect()
}

/// One unit's refined material and maintenance costs (class `+0x48`,
/// `+0x4c`; `FUN_0053b860`, `FUN_0053b870`).
#[must_use]
pub fn unit_costs(world: &GameWorld, kind: BuildableKind) -> Option<(u32, u32)> {
    match kind {
        BuildableKind::CapitalShip(key) => world
            .capital_ship_classes
            .get(key)
            .map(|c| (c.refined_material_cost, c.maintenance_cost)),
        BuildableKind::Fighter(key) => world
            .fighter_classes
            .get(key)
            .map(|c| (c.refined_material_cost, c.maintenance_cost)),
        _ => world
            .buildable_classes
            .get(&kind.class_dat_id()?)
            .map(|c| (c.refined_material_cost, c.maintenance_cost)),
    }
}

/// The days per unit of progress of each of `area`'s yards at `system` on
/// a side (`FUN_00520b70`: the yard class's `+0x5c`, MANFACSD
/// `processing_rate`). port: every yard counts; the original skips one
/// whose `+0x60` bit 0 is set (`FUN_00528b30`), which is untraced.
#[must_use]
pub fn yard_periods(
    world: &GameWorld,
    system: SystemKey,
    area: ProductionArea,
    is_alliance: bool,
) -> Vec<u32> {
    let Some(value) = world.systems.get(system) else {
        return Vec::new();
    };
    value
        .manufacturing_facilities
        .iter()
        .filter_map(|&key| world.manufacturing_facilities.get(key))
        .filter(|yard| {
            yard.side == crate::dat::Faction::of_alliance(is_alliance)
                && yard.class_dat_id.family() == area_family(area)
        })
        .filter_map(|yard| world.buildable_classes.get(&yard.class_dat_id))
        .map(|class| class.processing_rate)
        .collect()
}

/// What Build Selection shows for an order (`FUN_00538220`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildEstimate {
    /// Refined material, the class's times the quantity.
    pub refined_material: u32,
    /// Maintenance, the class's times the quantity.
    pub maintenance: u32,
    /// Best time to completion, in days; `None` when the manager has no
    /// yard to build with, which disables Confirm (`FUN_00439160`).
    pub completion: Option<u32>,
    /// Best time to deployment: the days from the manager's system to its
    /// destination.
    pub deployment: u32,
}

/// Price `count` units of `kind` at `system`'s `area` for a side.
#[must_use]
pub fn estimate(
    world: &GameWorld,
    manufacturing: &ManufacturingState,
    system: SystemKey,
    area: ProductionArea,
    kind: BuildableKind,
    count: u32,
    is_alliance: bool,
) -> BuildEstimate {
    let (refined, maintenance) = unit_costs(world, kind).unwrap_or_default();
    let destination = manufacturing
        .destination(system, area)
        .and_then(|d| d.system(world))
        .unwrap_or(system);
    BuildEstimate {
        refined_material: refined.saturating_mul(count),
        maintenance: maintenance.saturating_mul(count),
        completion: completion_day(
            &yard_periods(world, system, area, is_alliance),
            refined.saturating_mul(count),
        ),
        deployment: transit_days(world, kind, system, destination),
    }
}

/// The queue items a confirmed order adds: `count` units, each needing its
/// class's refined cost in units of work. `None` when no yard can build
/// (`FUN_00439160` disables Confirm).
#[must_use]
pub fn order_items(
    world: &GameWorld,
    system: SystemKey,
    area: ProductionArea,
    kind: BuildableKind,
    count: u32,
    is_alliance: bool,
) -> Option<Vec<QueueItem>> {
    let (refined, _) = unit_costs(world, kind)?;
    completion_day(&yard_periods(world, system, area, is_alliance), refined)?;
    Some((0..count).map(|_| QueueItem::new(kind, refined)).collect())
}

/// Why a Build (`FUN_005385f0`, [`order_refusal`]) or a Destination order
/// (`FUN_00537180`, [`destination_refusal`]) is refused: the order's
/// destination, the maintenance (`FUN_0052e970`), room aboard a destination
/// fleet's ships, and the room its destination system has for a Death Star
/// (`FUN_00507ab0` through the route, after `FUN_00552210`). None has text;
/// the side's advisor reacts to the status (`death-star-product.md`, 2 and
/// 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderRefusal {
    /// `{0xf3, 1}`: the units' maintenance exceeds the side's spare.
    Maintenance,
    /// `{0x90, 3}`: the destination already holds a Death Star.
    DeathStarRoom,
    /// `{1, 0x27}`: no ship of the destination fleet has room for a
    /// squadron or regiment (`FUN_00552300`).
    NoRoom,
    /// `{1, 0x22}`: the destination no longer resolves (`FUN_00504e60`, in
    /// `FUN_005385f0` and `FUN_00537180`).
    DestinationGone,
    /// `{1, 0x21}`: the Destination order names a fleet en route
    /// (`FUN_00537180.c:214-256`).
    DestinationEnRoute,
    /// `{1, 0x25}`: the Destination order names something the manager
    /// cannot send to: anything but a system for the facility manager
    /// (`FUN_00537180.c:194-212`).
    NotASystem,
}

impl OrderRefusal {
    /// The status word the advisor reacts to (`FUN_0052e970` writes
    /// `{0xf3, 1}`, `FUN_00507ab0` stores `{0x90, 3}`).
    #[must_use]
    pub const fn status(self) -> (u32, u32) {
        match self {
            Self::Maintenance => (0xf3, 1),
            Self::DeathStarRoom => (0x90, 3),
            Self::NoRoom => (1, 0x27),
            Self::DestinationGone => (1, 0x22),
            Self::DestinationEnRoute => (1, 0x21),
            Self::NotASystem => (1, 0x25),
        }
    }
}

/// Whether the original refuses `count` units of `kind` ordered at
/// `system`'s `area`.
///
/// Maintenance (`FUN_0052e970`, `FUN_0057f540`): the class's `+0x4c` times
/// the count, when non-zero, against side `+0x58` − `+0x5c`, strictly, so
/// equality passes. The room (`FUN_00507ab0`): one family `0x18`-`0x1b` ship
/// per system, counting every existing one there, either side's, unfinished
/// ones included: an ordered ship is a member of its destination's fleet
/// from its order (iterator mode 3; `death-star-product.md` section 2), and
/// a fleet bound for the destination counts there, one ordered away does
/// not, as for Move ([`crate::movement::death_stars_counted_at`]). The
/// area's own products are not counted: the Build replaces them, and the
/// check first gives their room back (`FUN_00552210` -> `FUN_00552000` ->
/// `FUN_00552dd0(.., 0, ..)`). A destination fleet that is gone refuses
/// first (`FUN_00504e60`, `{1, 0x22}`). The leg (`FUN_00551a20` ->
/// `FUN_00552300`, one unit after another): squadrons or regiments bound
/// for a fleet each need a ship with room, counting those aboard, on their
/// way and built for it ([`crate::carriage::ships_for`]), else `{1, 0x27}`;
/// the area's own queued units give theirs back first, as its Death Stars
/// do.
#[expect(
    clippy::too_many_arguments,
    reason = "The validator reads the world, the queues, the cargo and the order's fields."
)]
#[must_use]
pub fn order_refusal(
    world: &GameWorld,
    manufacturing: &crate::manufacturing::ManufacturingState,
    transport: &TroopTransportState,
    deliveries: &DeliveryState,
    system: SystemKey,
    area: ProductionArea,
    kind: BuildableKind,
    count: u32,
    is_alliance: bool,
    movement: &crate::movement::MovementState,
) -> Option<OrderRefusal> {
    let (_, maintenance) = unit_costs(world, kind)?;
    if manufacturing
        .destination(system, area)
        .is_some_and(|destination| destination.system(world).is_none())
    {
        return Some(OrderRefusal::DestinationGone);
    }
    let need = i64::from(maintenance) * i64::from(count);
    let side = crate::dat::Faction::of_alliance(is_alliance);
    if need != 0 && crate::resources::maintenance_surplus(world, manufacturing, side) < need {
        return Some(OrderRefusal::Maintenance);
    }
    if let (Some(Destination::Fleet(fleet)), Some(cargo)) =
        (manufacturing.destination(system, area), Cargo::of(kind))
    {
        // The area's own units give their room back first (`FUN_00552210`).
        let products = ProductRoom::excluding(manufacturing, deliveries, system, area);
        let count = usize::try_from(count).unwrap_or(usize::MAX);
        if crate::carriage::ships_for(world, transport, &products, fleet, cargo, count).is_none() {
            return Some(OrderRefusal::NoRoom);
        }
    }
    let is_death_star = |kind: BuildableKind| match kind {
        BuildableKind::CapitalShip(class) => world
            .capital_ship_classes
            .get(class)
            .is_some_and(crate::world::CapitalShipClass::is_death_star),
        _ => false,
    };
    if !is_death_star(kind) {
        return None;
    }
    let destination = manufacturing
        .destination(system, area)
        .and_then(|destination| destination.system(world))
        .unwrap_or(system);
    // The Build replaces the area's own products, so the room first takes
    // them back (`FUN_00552210` -> `FUN_00552000` -> `FUN_00552dd0(.., 0,
    // ..)`, room + request in `FUN_0054fca0`).
    let own: Vec<u32> = manufacturing
        .queue(system, area)
        .map(|queue| queue.items().iter().map(|item| item.ship).collect())
        .unwrap_or_default();
    let present = crate::movement::death_stars_counted_at(movement, world, destination, &own);
    (present + count as usize > 1).then_some(OrderRefusal::DeathStarRoom)
}

/// The Destination order (`0x214`) on `system`'s `area`: its checks
/// ([`destination_refusal`]), then the order itself
/// ([`ManufacturingState::set_destination`], `FUN_005298d0`), which moves
/// the area's unfinished ships into their new container at once
/// ([`crate::manufacturing::move_ordered_ships`], `FUN_0052a380`); each
/// squadron and regiment it sends to a fleet then takes a ship there
/// ([`crate::carriage::seat_products`]).
///
/// # Errors
/// The first refusal of [`destination_refusal`]; nothing changes.
pub fn order_destination(
    world: &mut GameWorld,
    manufacturing: &mut ManufacturingState,
    transport: &TroopTransportState,
    deliveries: &DeliveryState,
    system: SystemKey,
    area: ProductionArea,
    destination: Destination,
    movement: &crate::movement::MovementState,
) -> Result<(), OrderRefusal> {
    if let Some(refusal) = destination_refusal(
        world,
        manufacturing,
        transport,
        deliveries,
        system,
        area,
        destination,
        movement,
    ) {
        return Err(refusal);
    }
    manufacturing.set_destination(system, area, destination);
    crate::manufacturing::move_ordered_ships(world, manufacturing, system, area);
    crate::carriage::seat_products(world, transport, deliveries, manufacturing);
    Ok(())
}

/// The facility icon's Destination: one order whose team is every manager
/// of the system (`FUN_00512700`, kind 4), checked in container order, the
/// facility manager (`0xa0`) first, then ships (`0xa2`) and troops
/// (`0xa4`). The first refusal refuses the order and nothing changes.
///
/// # Errors
/// The first area's refusal ([`destination_refusal`]).
pub fn order_destination_all(
    world: &mut GameWorld,
    manufacturing: &mut ManufacturingState,
    transport: &TroopTransportState,
    deliveries: &DeliveryState,
    system: SystemKey,
    destination: Destination,
    movement: &crate::movement::MovementState,
) -> Result<(), OrderRefusal> {
    const CONTAINER_ORDER: [ProductionArea; 3] = [
        ProductionArea::ConstructionYard,
        ProductionArea::Shipyard,
        ProductionArea::TrainingFacility,
    ];
    if let Some(refusal) = CONTAINER_ORDER.iter().find_map(|&area| {
        destination_refusal(
            world,
            manufacturing,
            transport,
            deliveries,
            system,
            area,
            destination,
            movement,
        )
    }) {
        return Err(refusal);
    }
    for area in CONTAINER_ORDER {
        order_destination(
            world,
            manufacturing,
            transport,
            deliveries,
            system,
            area,
            destination,
            movement,
        )?;
    }
    Ok(())
}

/// Why `FUN_00537180` refuses a Destination on `system`'s `area`:
/// - a destination that does not resolve, `{1, 0x22}` (`FUN_00504e60`);
/// - for the facility manager (`0xa0`), anything but a system, `{1, 0x25}`
///   (`FUN_00537180.c:194-212`);
/// - a fleet en route, `{1, 0x21}` (`:214-256`; a port fleet en route is
///   absent from its system's list);
/// - with units queued, each product's new leg (`FUN_00551900` ->
///   `FUN_00552300`, `:321-345`): the Death Star room of its system,
///   `{0x90, 3}` (`FUN_00507ab0`, the count of
///   [`crate::movement::death_stars_counted_at`] less the area's own
///   products, which `FUN_00552210` gives back at `:321`), and room aboard
///   the fleet's ships for
///   squadrons and regiments, `{1, 0x27}`.
#[must_use]
pub fn destination_refusal(
    world: &GameWorld,
    manufacturing: &ManufacturingState,
    transport: &TroopTransportState,
    deliveries: &DeliveryState,
    system: SystemKey,
    area: ProductionArea,
    destination: Destination,
    movement: &crate::movement::MovementState,
) -> Option<OrderRefusal> {
    let Some(target) = destination.system(world) else {
        return Some(OrderRefusal::DestinationGone);
    };
    let target = match destination {
        _ if world.systems.get(target).is_none() => return Some(OrderRefusal::DestinationGone),
        Destination::System(_) => target,
        Destination::Fleet(_) if area == ProductionArea::ConstructionYard => {
            return Some(OrderRefusal::NotASystem)
        }
        Destination::Fleet(fleet) if !world.systems[target].fleets.contains(&fleet) => {
            return Some(OrderRefusal::DestinationEnRoute)
        }
        Destination::Fleet(_) => target,
    };
    let queued = manufacturing.queue(system, area);
    let death_star = |class| {
        world
            .capital_ship_classes
            .get(class)
            .is_some_and(crate::world::CapitalShipClass::is_death_star)
    };
    let moving: Vec<u32> = queued.map_or_else(Vec::new, |queue| {
        queue
            .items()
            .iter()
            .map(|item| item.ship)
            .filter(|&tag| tag != 0)
            .collect()
    });
    let request = queued.map_or(0, |queue| {
        queue
            .items()
            .iter()
            .filter(
                |item| matches!(item.kind, BuildableKind::CapitalShip(class) if death_star(class)),
            )
            .count()
    });
    if request > 0 {
        let present = crate::movement::death_stars_counted_at(movement, world, target, &moving);
        if present + request > 1 {
            return Some(OrderRefusal::DeathStarRoom);
        }
    }
    if let Destination::Fleet(fleet) = destination {
        let products = ProductRoom::excluding(manufacturing, deliveries, system, area);
        for cargo in [Cargo::Squadron, Cargo::Regiment] {
            let count = queued.map_or(0, |queue| {
                queue
                    .items()
                    .iter()
                    .filter(|item| Cargo::of(item.kind) == Some(cargo))
                    .count()
            });
            if crate::carriage::ships_for(world, transport, &products, fleet, cargo, count)
                .is_none()
            {
                return Some(OrderRefusal::NoRoom);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dat::{ExplorationStatus, Faction};
    use crate::world::{BuildableClass, ControlKind, ManufacturingFacilityInstance, System};

    fn system(world: &mut GameWorld, x: u16) -> SystemKey {
        world.systems.insert(System {
            dat_id: DatId::new(0x9000_0001),
            name: "Bortras".into(),
            sector: crate::ids::SectorKey::default(),
            x,
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
            control: ControlKind::Controlled(Faction::Alliance),
        })
    }

    fn class(
        world: &mut GameWorld,
        id: u32,
        sides: (bool, bool),
        costs: (u32, u32),
        order: u32,
        rate: u32,
    ) {
        world.buildable_classes.insert(
            DatId::new(id),
            BuildableClass {
                name: String::new(),
                is_alliance: sides.0,
                is_empire: sides.1,
                refined_material_cost: costs.0,
                maintenance_cost: costs.1,
                research_order: order,
                research_difficulty: 0,
                processing_rate: rate,
                ..BuildableClass::default()
            },
        );
    }

    fn yard(world: &mut GameWorld, at: SystemKey, id: u32, is_alliance: bool) {
        let key = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: DatId::new(id),
                side: crate::dat::Faction::of_alliance(is_alliance),
                is_shipyard: false,
            });
        world.systems[at].manufacturing_facilities.push(key);
    }

    /// MANFACSD: construction yard 4 days, advanced construction yard 2
    /// (research order 2); TROOPSD regiment 1 Alliance, 6 Empire; SPECFCSD
    /// unit 1 Alliance; DEFFACSD 4 (Death Star shield) Empire only.
    fn catalog() -> GameWorld {
        let mut world = GameWorld::default();
        class(&mut world, 0x2a00_0003, (true, true), (10, 10), 0, 4);
        class(&mut world, 0x2a00_0006, (true, true), (10, 10), 2, 2);
        class(&mut world, 0x2900_0002, (true, true), (10, 10), 0, 4);
        class(&mut world, 0x1000_0001, (true, false), (8, 6), 0, 0);
        class(&mut world, 0x1000_0006, (false, true), (9, 6), 0, 0);
        class(&mut world, 0x3c00_0001, (true, false), (1, 1), 0, 0);
        class(&mut world, 0x2500_0004, (false, true), (25, 20), 3, 0);
        class(&mut world, 0x2c00_0001, (true, true), (20, 0), 0, 5);
        world
    }

    /// `pairs` mine and refinery pairs of `side`: 50 maintenance each.
    fn capacity(world: &mut GameWorld, side: Faction, pairs: usize) {
        for _ in 0..pairs {
            for is_mine in [true, false] {
                world
                    .production_facilities
                    .insert(crate::world::ProductionFacilityInstance {
                        class_dat_id: DatId::new(0x2c00_0001),
                        side,
                        is_mine,
                    });
            }
        }
    }

    /// A fleet of a side at `at` with one ship holding `room` squadrons and
    /// as many regiments, and the side's squadron class.
    fn carrier_fleet(
        world: &mut GameWorld,
        at: SystemKey,
        is_alliance: bool,
        room: u32,
    ) -> (crate::ids::FleetKey, BuildableKind) {
        let class = world
            .capital_ship_classes
            .insert(crate::world::CapitalShipClass {
                is_alliance,
                fighter_capacity: room,
                troop_capacity: room,
                ..crate::world::CapitalShipClass::default()
            });
        let fleet = world.fleets.insert(crate::world::Fleet {
            location: at,
            is_alliance,
            capital_ships: vec![crate::world::ShipInstance::new(class, 10, is_alliance)],
            ..crate::world::Fleet::default()
        });
        world.systems[at].fleets.push(fleet);
        let squadron = world.fighter_classes.insert(crate::world::FighterClass {
            is_alliance,
            refined_material_cost: 5,
            maintenance_cost: 1,
            ..crate::world::FighterClass::default()
        });
        (fleet, BuildableKind::Fighter(squadron))
    }

    // FUN_005385f0 builds each unit's leg (FUN_00551a20 -> FUN_00552300),
    // and the route takes each unit's request from the ship's room
    // (FUN_00552dd0 -> FUN_00550eb0 writes it back): a Build bound for a
    // fleet whose ships lack room for every unit is refused with
    // {1, 0x27}. The units of one that fits, once queued, hold that room
    // against another yard's Build; a Build of their own area replaces them,
    // so its check gives their room back first (FUN_00552210 ->
    // FUN_00552000 -> FUN_00552dd0(.., 0, ..), room + request in
    // FUN_0054fca0).
    #[test]
    fn a_build_bound_for_a_fleet_needs_a_ship_with_room_for_each_unit() {
        for side in [Faction::Alliance, Faction::Empire] {
            let is_alliance = side == Faction::Alliance;
            let mut world = catalog();
            let at = system(&mut world, 0);
            let there = system(&mut world, 50);
            capacity(&mut world, side, 10);
            let (fleet, squadron) = carrier_fleet(&mut world, at, is_alliance, 2);
            let regiment = BuildableKind::Troop(DatId::new(if is_alliance {
                0x1000_0001
            } else {
                0x1000_0006
            }));
            let transport = TroopTransportState::new();
            let deliveries = DeliveryState::new();
            let mut mfg = ManufacturingState::new();
            for kind in [squadron, regiment] {
                let area = ProductionArea::of(kind);
                order_destination(
                    &mut world,
                    &mut mfg,
                    &transport,
                    &deliveries,
                    at,
                    area,
                    Destination::Fleet(fleet),
                    &crate::movement::MovementState::new(),
                )
                .unwrap();
                let refusal = |world: &GameWorld, mfg: &ManufacturingState, count| {
                    order_refusal(
                        world,
                        mfg,
                        &transport,
                        &deliveries,
                        at,
                        area,
                        kind,
                        count,
                        is_alliance,
                        &crate::movement::MovementState::new(),
                    )
                };
                assert_eq!(
                    refusal(&world, &mfg, 3),
                    Some(OrderRefusal::NoRoom),
                    "{side:?} {kind:?}"
                );
                assert_eq!(refusal(&world, &mfg, 2), None, "{side:?} {kind:?}");
                mfg.enqueue(at, QueueItem::new(kind, 5));
                crate::carriage::seat_products(&mut world, &transport, &deliveries, &mut mfg);
                assert_eq!(refusal(&world, &mfg, 2), None, "{side:?} {kind:?}");
                assert_eq!(
                    refusal(&world, &mfg, 3),
                    Some(OrderRefusal::NoRoom),
                    "{side:?} {kind:?}"
                );

                order_destination(
                    &mut world,
                    &mut mfg,
                    &transport,
                    &deliveries,
                    there,
                    area,
                    Destination::Fleet(fleet),
                    &crate::movement::MovementState::new(),
                )
                .unwrap();
                mfg.enqueue(there, QueueItem::new(kind, 5));
                crate::carriage::seat_products(&mut world, &transport, &deliveries, &mut mfg);
                assert_eq!(
                    refusal(&world, &mfg, 2),
                    Some(OrderRefusal::NoRoom),
                    "{side:?} {kind:?}"
                );
                assert_eq!(refusal(&world, &mfg, 1), None, "{side:?} {kind:?}");
            }
            let tag = world.fleets[fleet].capital_ships[0].tag;
            let seated: Vec<u32> = mfg.items().map(|(_, _, item)| item.carrier).collect();
            assert_eq!(seated, [tag; 4], "{side:?}");
            assert_eq!(OrderRefusal::NoRoom.status(), (1, 0x27));
        }
    }

    // hyp (order_destination): the Destination order's handler is
    // untraced; a queued unit is created aboard its ship (FUN_004f7d50), so
    // sending it to another fleet is a leg there (FUN_00552300), refused
    // with {1, 0x27} when that fleet's ships lack room, and then nothing
    // changes. A system destination seats nothing.
    #[test]
    fn a_destination_order_moves_the_queued_units_to_the_new_fleets_ships_or_is_refused() {
        for side in [Faction::Alliance, Faction::Empire] {
            let is_alliance = side == Faction::Alliance;
            let mut world = catalog();
            let at = system(&mut world, 0);
            let away = system(&mut world, 40);
            let (first, squadron) = carrier_fleet(&mut world, at, is_alliance, 2);
            let (small, _) = carrier_fleet(&mut world, at, is_alliance, 1);
            // Room for two regiments but one squadron: only the squadrons'
            // room refuses.
            let class = world.fleets[small].capital_ships[0].class;
            world.capital_ship_classes[class].troop_capacity = 2;
            let (large, _) = carrier_fleet(&mut world, at, is_alliance, 2);
            // A squadron on its way to this one's ship leaves it one place.
            let (shared, _) = carrier_fleet(&mut world, at, is_alliance, 2);
            let aboard = crate::carriage::tag_ship(&mut world, shared, 0).unwrap();
            let mut deliveries = DeliveryState::new();
            deliveries.depart(
                &world,
                &[crate::manufacturing::Departure {
                    origin: away,
                    destination: Destination::Fleet(shared),
                    tick: 0,
                    kind: squadron,
                    carrier: aboard,
                }],
            );
            let transport = TroopTransportState::new();
            let mut mfg = ManufacturingState::new();
            let area = ProductionArea::Shipyard;
            let order = |world: &mut GameWorld, mfg: &mut ManufacturingState, to| {
                order_destination(
                    world,
                    mfg,
                    &transport,
                    &deliveries,
                    at,
                    area,
                    to,
                    &crate::movement::MovementState::new(),
                )
            };
            order(&mut world, &mut mfg, Destination::Fleet(first)).unwrap();
            for _ in 0..2 {
                mfg.enqueue(at, QueueItem::new(squadron, 5));
            }
            crate::carriage::seat_products(&mut world, &transport, &deliveries, &mut mfg);
            let carriers = |mfg: &ManufacturingState| {
                mfg.items()
                    .map(|(_, _, item)| (item.destination, item.carrier))
                    .collect::<Vec<_>>()
            };
            let held = carriers(&mfg);
            let tag = |world: &GameWorld, fleet| world.fleets[fleet].capital_ships[0].tag;
            assert_eq!(
                held,
                [(Some(Destination::Fleet(first)), tag(&world, first)); 2],
                "{side:?}"
            );

            for full in [small, shared] {
                assert_eq!(
                    order(&mut world, &mut mfg, Destination::Fleet(full)),
                    Err(OrderRefusal::NoRoom),
                    "{side:?}"
                );
            }
            assert_eq!(carriers(&mfg), held, "{side:?}");
            assert_eq!(
                mfg.destination(at, area),
                Some(Destination::Fleet(first)),
                "{side:?}"
            );

            order(&mut world, &mut mfg, Destination::Fleet(large)).unwrap();
            assert_eq!(
                carriers(&mfg),
                [(Some(Destination::Fleet(large)), tag(&world, large)); 2],
                "{side:?}"
            );
            order(&mut world, &mut mfg, Destination::System(away)).unwrap();
            assert_eq!(
                carriers(&mfg),
                [(Some(Destination::System(away)), 0); 2],
                "{side:?}"
            );
        }
    }

    /// A side's capital ship ordered at `at`'s shipyard and seated, with
    /// the ships band's destination already set (as `order_items` and
    /// `seat_ordered_ships` do). Returns its tag.
    fn ordered_ship(
        world: &mut GameWorld,
        mfg: &mut ManufacturingState,
        at: SystemKey,
        is_alliance: bool,
    ) -> u32 {
        let class = world
            .capital_ship_classes
            .insert(crate::world::CapitalShipClass {
                is_alliance,
                is_empire: !is_alliance,
                hull: 100,
                ..crate::world::CapitalShipClass::default()
            });
        mfg.build_units(
            at,
            vec![QueueItem::new(BuildableKind::CapitalShip(class), 50)],
        );
        crate::manufacturing::seat_ordered_ships(world, mfg);
        mfg.queue(at, ProductionArea::Shipyard).unwrap().items()[0].ship
    }

    /// The fleet holding the ship tagged `tag`.
    fn holder(world: &GameWorld, tag: u32) -> Option<crate::ids::FleetKey> {
        crate::carriage::fleet_of_ship(world, tag)
    }

    // FUN_00537180.c:321-345: with units queued, each product's new leg
    // (FUN_00551900 -> FUN_00552300) picks the new system's spare, and
    // FUN_0052a380 reparents it at once; the old fleet, left with no ship,
    // disbands (FUN_004fe630). Progress lives with the order. The route-1
    // Empire game played on: a Destination to Averam moves the queued Death
    // Star into one fleet there, disbands Fleet 6 and keeps its progress.
    #[test]
    fn a_destination_change_moves_the_areas_unfinished_ships_into_one_new_fleet_and_disbands_the_old_for_either_side(
    ) {
        for is_alliance in [true, false] {
            let mut world = catalog();
            let (at, away) = (system(&mut world, 0), system(&mut world, 40));
            let mut mfg = ManufacturingState::new();
            let tag = ordered_ship(&mut world, &mut mfg, at, is_alliance);
            mfg.add_progress(at, ProductionArea::Shipyard);
            let old = holder(&world, tag).unwrap();

            order_destination(
                &mut world,
                &mut mfg,
                &TroopTransportState::new(),
                &DeliveryState::new(),
                at,
                ProductionArea::Shipyard,
                Destination::System(away),
                &crate::movement::MovementState::new(),
            )
            .unwrap();

            assert!(!world.fleets.contains_key(old), "{is_alliance}");
            assert!(world.systems[at].fleets.is_empty(), "{is_alliance}");
            let fleet = holder(&world, tag).unwrap();
            assert_eq!(world.systems[away].fleets, [fleet], "{is_alliance}");
            assert!(!world.fleets[fleet].capital_ships[0].completed);
            let item = &mfg.queue(at, ProductionArea::Shipyard).unwrap().items()[0];
            assert_eq!((item.ship, item.work_done), (tag, 1), "{is_alliance}");
        }
    }

    // FUN_00537180.c:262-306: the ship manager may name a fleet, which then
    // takes the ordered ship itself (FUN_004c27b0 -> FUN_00552300.c:100-118,
    // FUN_004fe160). The route-1 Empire game played on: the ships band's
    // Destination on Fleet 1 moves the queued Death Star (still 0x42) into
    // Fleet 1, and a ship ordered after it goes there too.
    #[test]
    fn a_ship_band_destination_on_a_fleet_seats_the_ordered_ship_in_that_fleet_unfinished_for_either_side(
    ) {
        for is_alliance in [true, false] {
            let mut world = catalog();
            let at = system(&mut world, 0);
            let (fleet, _) = carrier_fleet(&mut world, at, is_alliance, 0);
            let mut mfg = ManufacturingState::new();
            let queued = ordered_ship(&mut world, &mut mfg, at, is_alliance);
            let order = |world: &mut GameWorld, mfg: &mut ManufacturingState| {
                order_destination(
                    world,
                    mfg,
                    &TroopTransportState::new(),
                    &DeliveryState::new(),
                    at,
                    ProductionArea::Shipyard,
                    Destination::Fleet(fleet),
                    &crate::movement::MovementState::new(),
                )
            };

            order(&mut world, &mut mfg).unwrap();
            assert_eq!(holder(&world, queued), Some(fleet), "{is_alliance}");
            assert_eq!(world.systems[at].fleets, [fleet], "{is_alliance}");

            let later = ordered_ship(&mut world, &mut mfg, at, is_alliance);
            assert_eq!(holder(&world, later), Some(fleet), "{is_alliance}");
            let unfinished: Vec<bool> = world.fleets[fleet]
                .capital_ships
                .iter()
                .map(|ship| ship.completed)
                .collect();
            assert_eq!(unfinished, [true, false], "{is_alliance}");
        }
    }

    // FUN_00537180.c:194-212: the facility manager (0xa0, first in the
    // icon's team, FUN_00512700) takes only a system, else 1/0x25, which
    // refuses the whole order; :214-256: a fleet en route gives 1/0x21. The
    // route-1 Empire game played on: the facility icon's Destination
    // released on Fleet 1 changes nothing.
    #[test]
    fn an_all_areas_destination_on_a_fleet_is_refused_with_1_0x25_and_changes_nothing_for_either_side(
    ) {
        for is_alliance in [true, false] {
            let mut world = catalog();
            let at = system(&mut world, 0);
            let (fleet, _) = carrier_fleet(&mut world, at, is_alliance, 0);
            let mut mfg = ManufacturingState::new();
            let tag = ordered_ship(&mut world, &mut mfg, at, is_alliance);
            let seated = holder(&world, tag);

            let refused = order_destination_all(
                &mut world,
                &mut mfg,
                &TroopTransportState::new(),
                &DeliveryState::new(),
                at,
                Destination::Fleet(fleet),
                &crate::movement::MovementState::new(),
            );

            assert_eq!(refused, Err(OrderRefusal::NotASystem), "{is_alliance}");
            assert_eq!(OrderRefusal::NotASystem.status(), (1, 0x25));
            assert_eq!(holder(&world, tag), seated, "{is_alliance}");
            for area in ProductionArea::ALL {
                assert_eq!(mfg.destination(at, area), None, "{is_alliance} {area:?}");
            }

            world.systems[at].fleets.retain(|&key| key != fleet);
            let moving = order_destination(
                &mut world,
                &mut mfg,
                &TroopTransportState::new(),
                &DeliveryState::new(),
                at,
                ProductionArea::Shipyard,
                Destination::Fleet(fleet),
                &crate::movement::MovementState::new(),
            );
            assert_eq!(
                moving,
                Err(OrderRefusal::DestinationEnRoute),
                "{is_alliance}"
            );
            assert_eq!(OrderRefusal::DestinationEnRoute.status(), (1, 0x21));
            assert_eq!(holder(&world, tag), seated, "{is_alliance}");
        }
    }

    // FUN_0052a380 reparents only the products the manager lists, unfinished
    // (FUN_00528890: created, not completed), in list order, and the
    // side's spare takes them all (FUN_005097d0); a fleet keeps what is
    // left (FUN_004fe630.c:21-35).
    #[test]
    fn a_destination_change_moves_only_the_areas_unfinished_ships_in_queue_order_into_one_fleet_for_either_side(
    ) {
        for is_alliance in [true, false] {
            let mut world = catalog();
            let (at, away) = (system(&mut world, 0), system(&mut world, 40));
            let class = world
                .capital_ship_classes
                .insert(crate::world::CapitalShipClass {
                    is_alliance,
                    is_empire: !is_alliance,
                    hull: 100,
                    ..crate::world::CapitalShipClass::default()
                });
            let mut mfg = ManufacturingState::new();
            mfg.build_units(
                at,
                vec![QueueItem::new(BuildableKind::CapitalShip(class), 50); 2],
            );
            crate::manufacturing::seat_ordered_ships(&mut world, &mut mfg);
            let old = world.systems[at].fleets[0];
            let tags: Vec<u32> = world.fleets[old]
                .capital_ships
                .iter()
                .map(|s| s.tag)
                .collect();
            let mut finished = crate::world::ShipInstance::new(class, 100, is_alliance);
            finished.tag = 90;
            let mut other = crate::world::ShipInstance::ordered(class, 100, is_alliance);
            other.tag = 91;
            world.fleets[old].capital_ships.extend([finished, other]);

            order_destination(
                &mut world,
                &mut mfg,
                &TroopTransportState::new(),
                &DeliveryState::new(),
                at,
                ProductionArea::Shipyard,
                Destination::System(away),
                &crate::movement::MovementState::new(),
            )
            .unwrap();

            let kept: Vec<u32> = world.fleets[old]
                .capital_ships
                .iter()
                .map(|s| s.tag)
                .collect();
            assert_eq!(kept, [90, 91], "{is_alliance}");
            assert_eq!(world.systems[away].fleets.len(), 1, "{is_alliance}");
            let fleet = world.systems[away].fleets[0];
            let moved: Vec<u32> = world.fleets[fleet]
                .capital_ships
                .iter()
                .map(|s| s.tag)
                .collect();
            assert_eq!(moved, tags, "{is_alliance}");
        }
    }

    // FUN_00551900 runs each product's new leg through FUN_00552dd0, so a
    // Death Star's room binds a Destination as it binds a Build ({0x90, 3},
    // FUN_00507ab0); the ship that moves is not counted against itself. A
    // destination that does not resolve refuses first (FUN_00504e60,
    // {1, 0x22}).
    #[test]
    fn a_destination_change_is_refused_by_the_death_star_room_and_a_missing_system_for_either_side()
    {
        for is_alliance in [true, false] {
            let mut world = catalog();
            let (at, away) = (system(&mut world, 0), system(&mut world, 40));
            let (carrier, _) = carrier_fleet(&mut world, at, is_alliance, 0);
            let death_star = world
                .capital_ship_classes
                .insert(crate::world::CapitalShipClass {
                    family: 0x18,
                    is_alliance,
                    is_empire: !is_alliance,
                    hull: 100,
                    ..crate::world::CapitalShipClass::default()
                });
            let mut mfg = ManufacturingState::new();
            mfg.build_units(
                at,
                vec![QueueItem::new(BuildableKind::CapitalShip(death_star), 50)],
            );
            crate::manufacturing::seat_ordered_ships(&mut world, &mut mfg);
            let order = |world: &mut GameWorld, mfg: &mut ManufacturingState, to| {
                order_destination(
                    world,
                    mfg,
                    &TroopTransportState::new(),
                    &DeliveryState::new(),
                    at,
                    ProductionArea::Shipyard,
                    to,
                    &crate::movement::MovementState::new(),
                )
            };
            let blocker = world.fleets.insert(crate::world::Fleet {
                location: away,
                is_alliance: !is_alliance,
                capital_ships: vec![crate::world::ShipInstance::new(
                    death_star,
                    100,
                    !is_alliance,
                )],
                ..crate::world::Fleet::default()
            });
            world.systems[away].fleets.push(blocker);
            assert_eq!(
                order(&mut world, &mut mfg, Destination::System(away)),
                Err(OrderRefusal::DeathStarRoom),
                "{is_alliance}"
            );
            assert_eq!(
                order(&mut world, &mut mfg, Destination::Fleet(carrier)),
                Ok(()),
                "{is_alliance}"
            );
            let gone = system(&mut world, 80);
            world.systems.remove(gone);
            assert_eq!(
                order(&mut world, &mut mfg, Destination::System(gone)),
                Err(OrderRefusal::DestinationGone),
                "{is_alliance}"
            );
            assert_eq!(OrderRefusal::DestinationGone.status(), (1, 0x22));
        }
    }

    #[test]
    fn an_order_whose_maintenance_exceeds_the_spare_is_refused() {
        // FUN_0052e970: class +0x4c times the quantity against side +0x58 −
        // +0x5c, strictly; a class with no maintenance never refuses.
        for side in [Faction::Alliance, Faction::Empire] {
            let mut world = catalog();
            let at = system(&mut world, 0);
            capacity(&mut world, side, 1);
            let mfg = crate::manufacturing::ManufacturingState::new();
            let yard = BuildableKind::ManufacturingFacility(FacilityBuild {
                class: DatId::new(0x2a00_0003),
                is_alliance: side == Faction::Alliance,
            });
            let refusal = |count| {
                order_refusal(
                    &world,
                    &mfg,
                    &crate::troop_transport::TroopTransportState::default(),
                    &crate::delivery::DeliveryState::default(),
                    at,
                    ProductionArea::ConstructionYard,
                    yard,
                    count,
                    side == Faction::Alliance,
                    &crate::movement::MovementState::new(),
                )
            };
            assert_eq!(refusal(5), None, "50 spare covers 5 x 10");
            assert_eq!(refusal(6), Some(OrderRefusal::Maintenance));
            assert_eq!(OrderRefusal::Maintenance.status(), (0xf3, 1));
            let mine = BuildableKind::ProductionFacility(FacilityBuild {
                class: DatId::new(0x2c00_0001),
                is_alliance: side == Faction::Alliance,
            });
            assert_eq!(
                order_refusal(
                    &world,
                    &mfg,
                    &crate::troop_transport::TroopTransportState::default(),
                    &crate::delivery::DeliveryState::default(),
                    at,
                    ProductionArea::ConstructionYard,
                    mine,
                    100,
                    side == Faction::Alliance,
                    &crate::movement::MovementState::new(),
                ),
                None
            );
        }
    }

    #[test]
    fn a_death_star_order_is_refused_where_one_already_is_or_is_queued() {
        // FUN_00507ab0: room 1 − every family-0x18 ship of either side at the
        // destination, uncompleted products included; {0x90, 3}.
        use crate::world::{CapitalShipClass, Fleet, ShipInstance};
        for (side, other) in [
            (Faction::Empire, Faction::Alliance),
            (Faction::Alliance, Faction::Empire),
        ] {
            let mut world = GameWorld::default();
            let here = system(&mut world, 0);
            let there = system(&mut world, 50);
            capacity(&mut world, side, 100);
            let death_star = world.capital_ship_classes.insert(CapitalShipClass {
                family: 0x18,
                maintenance_cost: 600,
                refined_material_cost: 584,
                ..CapitalShipClass::default()
            });
            let order = BuildableKind::CapitalShip(death_star);
            let mut mfg = crate::manufacturing::ManufacturingState::new();
            let refusal =
                |world: &GameWorld, mfg: &crate::manufacturing::ManufacturingState, count| {
                    order_refusal(
                        world,
                        mfg,
                        &crate::troop_transport::TroopTransportState::default(),
                        &crate::delivery::DeliveryState::default(),
                        here,
                        ProductionArea::Shipyard,
                        order,
                        count,
                        side == Faction::Alliance,
                        &crate::movement::MovementState::new(),
                    )
                };
            assert_eq!(refusal(&world, &mfg, 1), None);
            assert_eq!(refusal(&world, &mfg, 2), Some(OrderRefusal::DeathStarRoom));

            // The other side's Death Star at the system blocks too.
            let fleet = world.fleets.insert(Fleet {
                location: here,
                is_alliance: other == Faction::Alliance,
                capital_ships: vec![ShipInstance::new(death_star, 1, false)],
                ..Fleet::default()
            });
            world.systems[here].fleets.push(fleet);
            assert_eq!(refusal(&world, &mfg, 1), Some(OrderRefusal::DeathStarRoom));
            world.fleets[fleet].capital_ships[0].alive = false;
            assert_eq!(refusal(&world, &mfg, 1), None);

            // A Death Star queued elsewhere but bound here blocks: the order
            // created it in this system's spare fleet.
            mfg.enqueue(
                there,
                crate::manufacturing::QueueItem::new(order, 584)
                    .delivered_to(crate::manufacturing::Destination::System(here)),
            );
            crate::manufacturing::seat_ordered_ships(&mut world, &mut mfg);
            assert_eq!(refusal(&world, &mfg, 1), Some(OrderRefusal::DeathStarRoom));
            assert_eq!(OrderRefusal::DeathStarRoom.status(), (0x90, 3));
        }
    }

    // FUN_005385f0 runs FUN_00552210 for each producer before its units'
    // FUN_00551a20: FUN_00552150 -> FUN_00552000 -> FUN_00552dd0(.., 0, ..)
    // gives each of the manager's own products' request back to its
    // container and that container's system (FUN_0054fca0: room + request
    // when the last argument is 0), so the Death Star the Build replaces
    // (FUN_00529ab0: FUN_0052a2c0, then FUN_0052a140) leaves the room.
    // Wine, the ds-b save (route-1 Empire game, day 151, maintenance raised
    // by 4000): Coruscant's ship band read "Death Star 1, Building: 1,
    // Destination: Fleet 6"; a Build of a Death Star there was accepted,
    // `0x18000df5` left every graph and the new `0x18000e4f` (`+0x50` 0x42)
    // sat in the same Fleet 6 `0x08000aea`, its only ship. The side's
    // counter for the class never goes down (FUN_005302c0), so the new ship
    // is "Death Star 2" where the replaced one was "Death Star 1" (the same
    // game, day 185).
    #[test]
    fn a_death_star_build_at_the_yard_building_one_replaces_it_in_the_same_destination_fleet_for_either_side(
    ) {
        use crate::manufacturing::{
            seat_ordered_ships, Destination, ManufacturingState, QueueItem,
        };
        use crate::world::{CapitalShipClass, Fleet, ShipInstance};
        for (side, other) in [
            (Faction::Empire, Faction::Alliance),
            (Faction::Alliance, Faction::Empire),
        ] {
            let is_alliance = side == Faction::Alliance;
            let mut world = GameWorld::default();
            let here = system(&mut world, 0);
            capacity(&mut world, side, 100);
            let death_star = world.capital_ship_classes.insert(CapitalShipClass {
                name: "Death Star".into(),
                family: 0x18,
                is_alliance,
                is_empire: !is_alliance,
                hull: 100,
                maintenance_cost: 600,
                refined_material_cost: 584,
                ..CapitalShipClass::default()
            });
            let order = BuildableKind::CapitalShip(death_star);
            let destination = world.fleets.insert(Fleet {
                location: here,
                is_alliance,
                ..Fleet::default()
            });
            world.systems[here].fleets.push(destination);
            let mut mfg = ManufacturingState::new();
            mfg.set_destination(
                here,
                ProductionArea::Shipyard,
                Destination::Fleet(destination),
            );
            let unit = QueueItem::new(order, 584);
            mfg.build(here, &unit, 1);
            seat_ordered_ships(&mut world, &mut mfg);
            let replaced = world.fleets[destination].capital_ships[0].tag;
            assert_eq!(
                world.fleets[destination].capital_ships[0].name.as_deref(),
                Some("Death Star 1"),
                "{side:?}"
            );
            let refusal = |world: &GameWorld, mfg: &ManufacturingState, count| {
                order_refusal(
                    world,
                    mfg,
                    &crate::troop_transport::TroopTransportState::default(),
                    &crate::delivery::DeliveryState::default(),
                    here,
                    ProductionArea::Shipyard,
                    order,
                    count,
                    is_alliance,
                    &crate::movement::MovementState::new(),
                )
            };

            assert_eq!(refusal(&world, &mfg, 1), None, "{side:?}");
            assert_eq!(
                refusal(&world, &mfg, 2),
                Some(OrderRefusal::DeathStarRoom),
                "{side:?}"
            );
            mfg.build(here, &unit, 1);
            seat_ordered_ships(&mut world, &mut mfg);
            let ships: Vec<_> = world.fleets[destination]
                .capital_ships
                .iter()
                .map(|ship| (ship.class, ship.completed))
                .collect();
            assert_eq!(ships, [(death_star, false)], "{side:?}");
            assert_ne!(
                world.fleets[destination].capital_ships[0].tag, replaced,
                "{side:?}"
            );
            assert_eq!(
                world.fleets[destination].capital_ships[0].name.as_deref(),
                Some("Death Star 2"),
                "{side:?}"
            );

            // Any other Death Star at the destination still takes the room.
            let theirs = world.fleets.insert(Fleet {
                location: here,
                is_alliance: other == Faction::Alliance,
                capital_ships: vec![ShipInstance::new(death_star, 100, !is_alliance)],
                ..Fleet::default()
            });
            world.systems[here].fleets.push(theirs);
            assert_eq!(
                refusal(&world, &mfg, 1),
                Some(OrderRefusal::DeathStarRoom),
                "{side:?}"
            );
        }
    }

    // FUN_00551a20 binds a Build's Death Star, and FUN_00537180's new legs
    // (FUN_00551900, :336) bind a Destination's, through FUN_00552dd0 ->
    // FUN_00550fb0 on the destination system's room, the array
    // FUN_00507ab0 fills by walking the system's fleets in mode 3
    // (FUN_004ffe70), which Move binds too (FUN_00551630). A moving fleet's
    // container changes at its order (death-star-product.md, section 2;
    // movement::death_star_room): one bound for the system counts there,
    // one ordered away does not.
    #[test]
    fn a_death_star_order_counts_the_death_stars_bound_for_its_destination_not_those_leaving_for_either_side(
    ) {
        use crate::manufacturing::{Destination, ManufacturingState, QueueItem};
        use crate::movement::MovementState;
        use crate::world::{CapitalShipClass, Fleet, ShipInstance};
        for side in [Faction::Empire, Faction::Alliance] {
            let is_alliance = side == Faction::Alliance;
            let world_with = |coming: bool| {
                let mut world = GameWorld::default();
                let here = system(&mut world, 0);
                let there = system(&mut world, 50);
                capacity(&mut world, side, 100);
                let death_star = world.capital_ship_classes.insert(CapitalShipClass {
                    family: 0x18,
                    is_alliance,
                    is_empire: !is_alliance,
                    hull: 100,
                    hyperdrive: 100,
                    maintenance_cost: 600,
                    refined_material_cost: 584,
                    ..CapitalShipClass::default()
                });
                let at = if coming { there } else { here };
                let fleet = world.fleets.insert(Fleet {
                    location: at,
                    is_alliance: !is_alliance,
                    capital_ships: vec![ShipInstance::new(death_star, 100, !is_alliance)],
                    ..Fleet::default()
                });
                world.systems[at].fleets.push(fleet);
                let mut movement = MovementState::new();
                if coming {
                    assert!(crate::movement::begin_fleet_transit(
                        &mut movement,
                        &mut world,
                        fleet,
                        here,
                        10
                    ));
                } else {
                    // Ordered away, still listed at its system.
                    assert!(movement.order(fleet, here, there, 10));
                }
                (world, movement, here, there, death_star)
            };
            for (coming, refused) in [(true, Some(OrderRefusal::DeathStarRoom)), (false, None)] {
                let (world, movement, here, there, death_star) = world_with(coming);
                let order = BuildableKind::CapitalShip(death_star);
                let mfg = ManufacturingState::new();
                assert_eq!(
                    order_refusal(
                        &world,
                        &mfg,
                        &crate::troop_transport::TroopTransportState::default(),
                        &crate::delivery::DeliveryState::default(),
                        here,
                        ProductionArea::Shipyard,
                        order,
                        1,
                        is_alliance,
                        &movement,
                    ),
                    refused,
                    "Build {side:?} coming {coming}"
                );

                // The same Death Star queued at another yard, sent here.
                let mut mfg = ManufacturingState::new();
                mfg.enqueue(there, QueueItem::new(order, 584));
                assert_eq!(
                    destination_refusal(
                        &world,
                        &mfg,
                        &crate::troop_transport::TroopTransportState::default(),
                        &crate::delivery::DeliveryState::default(),
                        there,
                        ProductionArea::Shipyard,
                        Destination::System(here),
                        &movement,
                    ),
                    refused,
                    "Destination {side:?} coming {coming}"
                );
            }
        }
    }

    // FUN_0054fd90 checks a room category only when the request's word for
    // it is not 0, so a Destination whose queue holds no Death Star never
    // reaches the Death Star room, however full it is.
    #[test]
    fn a_destination_with_no_death_star_queued_ignores_the_death_star_room_for_either_side() {
        use crate::manufacturing::{Destination, ManufacturingState, QueueItem};
        use crate::world::{CapitalShipClass, Fleet, ShipInstance};
        for side in [Faction::Empire, Faction::Alliance] {
            let is_alliance = side == Faction::Alliance;
            let mut world = GameWorld::default();
            let here = system(&mut world, 0);
            let there = system(&mut world, 50);
            let class = |world: &mut GameWorld, family| {
                world.capital_ship_classes.insert(CapitalShipClass {
                    family,
                    is_alliance,
                    is_empire: !is_alliance,
                    hull: 100,
                    refined_material_cost: 5,
                    ..CapitalShipClass::default()
                })
            };
            let death_star = class(&mut world, 0x18);
            let cruiser = class(&mut world, 0x14);
            for owner in [is_alliance, !is_alliance] {
                let fleet = world.fleets.insert(Fleet {
                    location: here,
                    is_alliance: owner,
                    capital_ships: vec![ShipInstance::new(death_star, 100, owner)],
                    ..Fleet::default()
                });
                world.systems[here].fleets.push(fleet);
            }
            let mut mfg = ManufacturingState::new();
            mfg.enqueue(
                there,
                QueueItem::new(BuildableKind::CapitalShip(cruiser), 5),
            );

            assert_eq!(
                destination_refusal(
                    &world,
                    &mfg,
                    &crate::troop_transport::TroopTransportState::default(),
                    &crate::delivery::DeliveryState::default(),
                    there,
                    ProductionArea::Shipyard,
                    Destination::System(here),
                    &crate::movement::MovementState::new(),
                ),
                None,
                "{side:?}"
            );
        }
    }

    // FUN_0052e580: the manager's family (0x29 regiments and special forces,
    // 0x2a facilities) for its side, up to its research level.
    #[test]
    fn a_manager_lists_its_familys_classes_its_side_has_researched() {
        let world = catalog();
        let mut research = ResearchState::new();
        let yard = |id| {
            BuildableKind::ManufacturingFacility(FacilityBuild {
                class: DatId::new(id),
                is_alliance: true,
            })
        };
        let mine = BuildableKind::ProductionFacility(FacilityBuild {
            class: DatId::new(0x2c00_0001),
            is_alliance: true,
        });
        assert_eq!(
            listed_classes(&world, &research, ProductionArea::ConstructionYard, true),
            [yard(0x2900_0002), yard(0x2a00_0003), mine]
        );
        research.alliance.facility = 2;
        assert_eq!(
            listed_classes(&world, &research, ProductionArea::ConstructionYard, true),
            [
                yard(0x2900_0002),
                yard(0x2a00_0003),
                yard(0x2a00_0006),
                mine
            ]
        );
        assert_eq!(
            listed_classes(&world, &research, ProductionArea::TrainingFacility, true),
            [
                BuildableKind::Troop(DatId::new(0x1000_0001)),
                BuildableKind::SpecialForce(DatId::new(0x3c00_0001))
            ]
        );
        assert_eq!(
            listed_classes(&world, &research, ProductionArea::TrainingFacility, false),
            [BuildableKind::Troop(DatId::new(0x1000_0006))]
        );
        // The Empire's facility list waits on its own level for the shield.
        research.empire.facility = 3;
        assert!(
            listed_classes(&world, &research, ProductionArea::ConstructionYard, false).contains(
                &BuildableKind::DefenseFacility(FacilityBuild {
                    class: DatId::new(0x2500_0004),
                    is_alliance: false,
                })
            )
        );
    }

    // FUN_0052e580 with family 0x28: the side's researched ship and fighter
    // classes, in id order; FUN_0053b860/FUN_0053b870 their costs.
    #[test]
    fn a_shipyard_lists_its_sides_researched_ships_and_fighters() {
        use crate::world::{CapitalShipClass, FighterClass};
        let mut world = GameWorld::default();
        let ship = |id, alliance, empire, order| CapitalShipClass {
            dat_id: DatId::new(id),
            is_alliance: alliance,
            is_empire: empire,
            refined_material_cost: 30,
            maintenance_cost: 25,
            research_order: order,
            ..CapitalShipClass::default()
        };
        let corvette = world
            .capital_ship_classes
            .insert(ship(0x1400_0002, true, false, 0));
        world
            .capital_ship_classes
            .insert(ship(0x1400_0001, false, true, 0));
        world
            .capital_ship_classes
            .insert(ship(0x1400_0003, true, false, 4));
        let wing = |id, alliance, empire, order| FighterClass {
            dat_id: DatId::new(id),
            is_alliance: alliance,
            is_empire: empire,
            refined_material_cost: 4,
            maintenance_cost: 3,
            research_order: order,
            ..FighterClass::default()
        };
        let x_wing = world
            .fighter_classes
            .insert(wing(0x1a00_0001, true, false, 0));
        world
            .fighter_classes
            .insert(wing(0x1a00_0002, false, true, 0));
        world
            .fighter_classes
            .insert(wing(0x1a00_0003, true, false, 2));
        let research = ResearchState::new();

        assert_eq!(
            listed_classes(&world, &research, ProductionArea::Shipyard, true),
            [
                BuildableKind::CapitalShip(corvette),
                BuildableKind::Fighter(x_wing)
            ]
        );
        assert_eq!(
            unit_costs(&world, BuildableKind::CapitalShip(corvette)),
            Some((30, 25))
        );
        assert_eq!(
            unit_costs(&world, BuildableKind::Fighter(x_wing)),
            Some((4, 3))
        );
    }

    // FUN_00538220: both costs times the quantity; FUN_00528d30 the best
    // completion from the manager's yards; FUN_00555f90 the trip to its
    // destination.
    #[test]
    fn an_estimate_prices_the_quantity_and_times_the_managers_yards() {
        let mut world = catalog();
        let home = system(&mut world, 0);
        let away = system(&mut world, 200);
        yard(&mut world, home, 0x2900_0002, true);
        yard(&mut world, home, 0x2900_0002, false);
        let regiment = BuildableKind::Troop(DatId::new(0x1000_0001));
        let mut manufacturing = ManufacturingState::new();

        let at_home = estimate(
            &world,
            &manufacturing,
            home,
            ProductionArea::TrainingFacility,
            regiment,
            3,
            true,
        );
        assert_eq!(
            at_home,
            BuildEstimate {
                refined_material: 24,
                maintenance: 18,
                completion: Some(96),
                deployment: 0,
            }
        );

        manufacturing.set_destination(
            home,
            ProductionArea::TrainingFacility,
            crate::manufacturing::Destination::System(away),
        );
        let sent = estimate(
            &world,
            &manufacturing,
            home,
            ProductionArea::TrainingFacility,
            regiment,
            1,
            true,
        );
        assert_eq!(sent.completion, Some(32));
        assert!(sent.deployment > 0);

        // No training facility of the side: nothing can build it.
        let empire = estimate(
            &world,
            &manufacturing,
            away,
            ProductionArea::TrainingFacility,
            regiment,
            1,
            true,
        );
        assert_eq!(empire.completion, None);
    }

    // FUN_00538220 prices each unit at its class's refined cost; the
    // yards' cycles draw it one unit at a time (FUN_00530950).
    #[test]
    fn an_order_queues_each_unit_with_its_classs_work() {
        let mut world = catalog();
        let home = system(&mut world, 0);
        yard(&mut world, home, 0x2a00_0003, true);
        yard(&mut world, home, 0x2a00_0006, true);
        let kind = BuildableKind::ManufacturingFacility(FacilityBuild {
            class: DatId::new(0x2a00_0003),
            is_alliance: true,
        });
        let items = order_items(
            &world,
            home,
            ProductionArea::ConstructionYard,
            kind,
            2,
            true,
        )
        .expect("the yards can build");
        assert_eq!(items.len(), 2);
        assert!(items
            .iter()
            .all(|item| item.total_cost == 10 && item.work_done == 0 && item.kind == kind));
        assert!(order_items(
            &world,
            home,
            ProductionArea::TrainingFacility,
            kind,
            1,
            true
        )
        .is_none());
    }
}
