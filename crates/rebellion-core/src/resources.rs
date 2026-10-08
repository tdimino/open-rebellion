//! A side's raw materials, refined materials and maintenance, the three
//! numbers of the command center's top strip
//! (`ghidra/notes/top-bar-resource-counters.md`).

use crate::build_selection::unit_costs;
use crate::dat::Faction;
use crate::manufacturing::ManufacturingState;
use crate::world::GameWorld;

/// The maintenance capacity each mine and each refinery brings: their
/// initialisers (`FUN_0052d540` mine, `FUN_0052d100` refinery; Ghidra had
/// not split them out) set the `+0x64` pair to `(0x32, 0)`.
pub const FACILITY_MAINTENANCE_CAPACITY: i64 = 50;

/// A side's maintenance capacity: side `+0x58`, the lesser of its mines'
/// and refineries' summed capacities (`FUN_0052fff0` sums them,
/// `FUN_005323c0` takes the minimum).
#[must_use]
pub fn maintenance_capacity(world: &GameWorld, side: Faction) -> i64 {
    let (mines, refineries) = world
        .production_facilities
        .values()
        .filter(|facility| facility.side == side)
        .fold((0_i64, 0_i64), |(mines, refineries), facility| {
            if facility.is_mine {
                (mines + 1, refineries)
            } else {
                (mines, refineries + 1)
            }
        });
    FACILITY_MAINTENANCE_CAPACITY * mines.min(refineries)
}

/// The maintenance a side's objects use: side `+0x74`, the sum over its
/// existing objects of their class `+0x4c` (`FUN_0052fff0`,
/// `FUN_004f2990` → `FUN_0053b870`). Characters' classes carry none.
///
/// hyp: units in a production queue count from their order (manual p. 84
/// deducts maintenance at the order; `build-delivery.md`: the queued units
/// are game objects from the order). A queue belongs to its system's
/// holder.
#[must_use]
pub fn maintenance_used(
    world: &GameWorld,
    manufacturing: &ManufacturingState,
    side: Faction,
) -> i64 {
    let is_alliance = side == Faction::Alliance;
    let class = |id| {
        world
            .buildable_classes
            .get(id)
            .map_or(0, |class| i64::from(class.maintenance_cost))
    };
    let mut used = 0_i64;
    for fleet in world.fleets.values().filter(|f| f.is_alliance == is_alliance) {
        for ship in fleet.capital_ships.iter().filter(|ship| ship.alive) {
            used += world
                .capital_ship_classes
                .get(ship.class)
                .map_or(0, |class| i64::from(class.maintenance_cost));
        }
        for entry in &fleet.fighters {
            used += world.fighter_classes.get(entry.class).map_or(0, |class| {
                i64::from(class.maintenance_cost) * i64::from(entry.count)
            });
        }
    }
    used += world
        .troops
        .values()
        .filter(|troop| troop.is_alliance == is_alliance)
        .map(|troop| class(&troop.class_dat_id))
        .sum::<i64>();
    used += world
        .special_forces
        .values()
        .filter(|unit| unit.is_alliance == is_alliance)
        .map(|unit| class(&unit.class_dat_id))
        .sum::<i64>();
    used += world
        .defense_facilities
        .values()
        .filter(|facility| facility.side == side)
        .map(|facility| class(&facility.class_dat_id))
        .sum::<i64>();
    used += world
        .manufacturing_facilities
        .values()
        .filter(|facility| facility.side == side)
        .map(|facility| class(&facility.class_dat_id))
        .sum::<i64>();
    used += world
        .production_facilities
        .values()
        .filter(|facility| facility.side == side)
        .map(|facility| class(&facility.class_dat_id))
        .sum::<i64>();
    for (&(system, _), queue) in manufacturing.queues() {
        if !world
            .systems
            .get(system)
            .is_some_and(|system| system.control.is_controlled_by(side))
        {
            continue;
        }
        used += queue
            .items()
            .iter()
            .filter_map(|item| unit_costs(world, item.kind))
            .map(|(_, maintenance)| i64::from(maintenance))
            .sum::<i64>();
    }
    used
}

/// The Maintenance Monitor's number: side `+0x58` − `+0x74`
/// (`FUN_00422620`, message `0x14e`).
#[must_use]
pub fn maintenance_surplus(
    world: &GameWorld,
    manufacturing: &ManufacturingState,
    side: Faction,
) -> i64 {
    maintenance_capacity(world, side) - maintenance_used(world, manufacturing, side)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::DatId;
    use crate::world::{BuildableClass, ProductionFacilityInstance, TroopUnit};

    fn facility(side: Faction, is_mine: bool) -> ProductionFacilityInstance {
        ProductionFacilityInstance {
            class_dat_id: DatId::new(if is_mine { 0x2c00_0001 } else { 0x2d00_0002 }),
            side,
            is_mine,
        }
    }

    #[test]
    fn capacity_is_fifty_for_each_mine_matched_by_a_refinery() {
        // 0x52d540 / 0x52d100: each mine and refinery holds (50, 0);
        // FUN_005323c0 keeps the lesser sum.
        let mut world = GameWorld::default();
        for _ in 0..3 {
            world
                .production_facilities
                .insert(facility(Faction::Empire, true));
        }
        world
            .production_facilities
            .insert(facility(Faction::Empire, false));
        world
            .production_facilities
            .insert(facility(Faction::Alliance, false));
        assert_eq!(maintenance_capacity(&world, Faction::Empire), 50);
        assert_eq!(maintenance_capacity(&world, Faction::Alliance), 0);
        world
            .production_facilities
            .insert(facility(Faction::Empire, false));
        assert_eq!(maintenance_capacity(&world, Faction::Empire), 100);
    }

    #[test]
    fn the_surplus_takes_each_of_the_sides_units_class_maintenance() {
        // FUN_0052fff0: Σ class +0x4c over the side's objects;
        // FUN_00422620 shows +0x58 − +0x74.
        let mut world = GameWorld::default();
        let regiment = DatId::new(0x1000_0001);
        world.buildable_classes.insert(
            regiment,
            BuildableClass {
                maintenance_cost: 3,
                ..BuildableClass::default()
            },
        );
        for is_alliance in [false, false, true] {
            world.troops.insert(TroopUnit {
                class_dat_id: regiment,
                is_alliance,
                regiment_strength: 1,
            });
        }
        world
            .production_facilities
            .insert(facility(Faction::Empire, true));
        world
            .production_facilities
            .insert(facility(Faction::Empire, false));
        let manufacturing = ManufacturingState::new();
        assert_eq!(
            maintenance_surplus(&world, &manufacturing, Faction::Empire),
            50 - 6
        );
        assert_eq!(
            maintenance_surplus(&world, &manufacturing, Faction::Alliance),
            -3
        );
    }
}
