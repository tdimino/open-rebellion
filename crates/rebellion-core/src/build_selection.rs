//! What a production manager can build and what an order costs: the model
//! behind the Build Selection window (`ghidra/notes/manufacturing-build-selection.md`).
//!
//! `FUN_00537ff0` maps a manager to the classes its yards build and
//! `FUN_0052e580` lists those its side has researched; `FUN_00538220`
//! prices an order as each class cost times the quantity and asks the
//! manager for the best completion and deployment times.

use crate::delivery::transit_days;
use crate::ids::{DatId, SystemKey};
use crate::manufacturing::{
    completion_day, unit_build_days, BuildableKind, FacilityBuild, ManufacturingState,
    ProductionArea, QueueItem,
};
use crate::research::{ResearchState, TechType};
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
            yard.is_alliance == is_alliance && yard.class_dat_id.family() == area_family(area)
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
    let destination = manufacturing.destination(system, area).unwrap_or(system);
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

/// The queue items a confirmed order adds: `count` units, each with its
/// own build days (`unit_build_days`). `None` when no yard can build.
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
    let days = unit_build_days(
        &yard_periods(world, system, area, is_alliance),
        refined,
        count,
    )?;
    Some(
        days.into_iter()
            .map(|days| QueueItem::new(kind, days, refined))
            .collect(),
    )
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
            },
        );
    }

    fn yard(world: &mut GameWorld, at: SystemKey, id: u32, is_alliance: bool) {
        let key = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: DatId::new(id),
                is_alliance,
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

        manufacturing.set_destination(home, ProductionArea::TrainingFacility, away);
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

    #[test]
    fn an_order_queues_each_unit_with_its_own_build_days() {
        let mut world = catalog();
        let home = system(&mut world, 0);
        yard(&mut world, home, 0x2a00_0003, true);
        yard(&mut world, home, 0x2a00_0006, true);
        let kind = BuildableKind::ManufacturingFacility(FacilityBuild {
            class: DatId::new(0x2a00_0003),
            is_alliance: true,
        });
        // Periods 4 and 2: 10 units of work end on day 14 (3 + 7), 20 on
        // day 28 (7 + 14).
        let items = order_items(
            &world,
            home,
            ProductionArea::ConstructionYard,
            kind,
            2,
            true,
        )
        .expect("the yards can build");
        let days: Vec<u32> = items.iter().map(|item| item.ticks_remaining).collect();
        assert_eq!(days, [14, 14]);
        assert!(items
            .iter()
            .all(|item| item.total_cost == 10 && item.kind == kind));
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
