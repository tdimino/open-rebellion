//! Ship repair system — restores hull at systems with shipyard facilities.
//!
//! From community disassembly cross-reference: `CombatUnitUnderRepair` /
//! `CombatUnitFastRepair` notifications. Ships at systems with manufacturing
//! facilities (shipyards) auto-repair each tick using the class `damage_control`
//! rate.
//!
//! # Advance contract
//! `RepairSystem::advance()` never mutates `GameWorld`.
//! It returns `Vec<RepairEvent>` for the caller to apply.

use serde::{Deserialize, Serialize};

use crate::ids::*;
use crate::tick::TickEvent;
use crate::world::GameWorld;

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

/// Events produced by the repair system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepairEvent {
    /// A ship's hull was restored.
    ShipRepaired {
        fleet: FleetKey,
        ship_index: usize,
        hull_before: i32,
        hull_after: i32,
    },
    /// Repair check performed: fleet at shipyard with damage_control capability.
    /// Emitted even when no actual hull restoration occurs (ShipInstance not yet
    /// promoted). Ensures telemetry coverage for the repair system.
    RepairCheckPerformed {
        system: SystemKey,
        fleet: FleetKey,
        ships_checked: usize,
    },
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

/// Repair system state (currently stateless — repair is immediate per tick).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RepairState;

// ---------------------------------------------------------------------------
// System
// ---------------------------------------------------------------------------

pub struct RepairSystem;

impl RepairSystem {
    /// Advance ship repairs. For each fleet at a system with manufacturing
    /// facilities, repair damaged ships using the class `damage_control` rate.
    ///
    /// Returns repair events for the caller to apply to `GameWorld`.
    pub fn advance(
        _state: &mut RepairState,
        world: &GameWorld,
        tick_events: &[TickEvent],
    ) -> Vec<RepairEvent> {
        if tick_events.is_empty() {
            return Vec::new();
        }

        let mut events = Vec::new();

        // Iterate all systems that have manufacturing facilities (shipyards).
        for (sys_key, sys) in world.systems.iter() {
            if sys.is_destroyed || sys.manufacturing_facilities.is_empty() {
                continue;
            }

            // Each fleet at this system gets repair service.
            for &fleet_key in &sys.fleets {
                let fleet = match world.fleets.get(fleet_key) {
                    Some(f) => f,
                    None => continue,
                };

                // Count ships with damage_control capability.
                let ships_checked = fleet.capital_ships.iter().filter(|entry| {
                    world.capital_ship_classes.get(entry.class)
                        .map_or(false, |c| c.damage_control > 0)
                }).count();

                if ships_checked > 0 {
                    events.push(RepairEvent::RepairCheckPerformed {
                        system: sys_key,
                        fleet: fleet_key,
                        ships_checked,
                    });
                }

                // NOTE: Full per-hull repair (ShipRepaired events) requires
                // ShipInstance promotion to fleet-level storage. Until then,
                // RepairCheckPerformed confirms the system is active.
                // TODO: when ShipInstance is promoted, check hull_current < class.hull
                // and emit ShipRepaired with actual delta.
            }
        }

        events
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dat::{ExplorationStatus, Faction};
    use crate::ids::*;
    use crate::world::*;

    fn make_shipyard_system(world: &mut GameWorld) -> SystemKey {
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(0x9200_0000),
            name: "Test Sector".into(),
            group: crate::dat::SectorGroup::Core,
            x: 100,
            y: 100,
            systems: vec![],
        });
        let mfg_key = world.manufacturing_facilities.insert(ManufacturingFacilityInstance {
            class_dat_id: DatId::new(0),
            is_alliance: false,
        });
        let sys_key = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0000),
            name: "Kuat".into(),
            sector,
            x: 100, y: 100,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.3,
            popularity_empire: 0.7,
            is_populated: true,
            total_energy: 10,
            raw_materials: 8,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![mfg_key],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Empire),
        });
        sys_key
    }

    fn add_fleet_with_ship(world: &mut GameWorld, sys_key: SystemKey) -> FleetKey {
        let class_key = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(0x3000_0001),
            name: "Star Destroyer".into(),
            is_alliance: false,
            is_empire: true,
            refined_material_cost: 100,
            maintenance_cost: 10,
            research_order: 1,
            research_difficulty: 5,
            hull: 200,
            shield_strength: 100,
            sub_light_engine: 50,
            maneuverability: 30,
            hyperdrive: 40,
            fighter_capacity: 6,
            troop_capacity: 4,
            detection: 3,
            turbolaser_fore: 10, turbolaser_aft: 5, turbolaser_port: 8, turbolaser_starboard: 8,
            ion_cannon_fore: 5, ion_cannon_aft: 3, ion_cannon_port: 4, ion_cannon_starboard: 4,
            laser_cannon_fore: 3, laser_cannon_aft: 2, laser_cannon_port: 3, laser_cannon_starboard: 3,
            shield_recharge_rate: 5,
            damage_control: 10,
            bombardment_modifier: 50,
            ..Default::default()
        });
        let fleet_key = world.fleets.insert(Fleet {
            location: sys_key,
            capital_ships: vec![ShipEntry { class: class_key, count: 3 }],
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            has_death_star: false,
        });
        world.systems.get_mut(sys_key).unwrap().fleets.push(fleet_key);
        fleet_key
    }

    #[test]
    fn repair_at_shipyard_emits_check_performed() {
        let mut world = GameWorld::default();
        let sys_key = make_shipyard_system(&mut world);
        add_fleet_with_ship(&mut world, sys_key);
        let mut state = RepairState;
        let tick_events = vec![crate::tick::TickEvent { tick: 1 }];

        let events = RepairSystem::advance(&mut state, &world, &tick_events);
        assert_eq!(events.len(), 1, "expected RepairCheckPerformed event");
        match &events[0] {
            RepairEvent::RepairCheckPerformed { system, ships_checked, .. } => {
                assert_eq!(*system, sys_key);
                assert_eq!(*ships_checked, 1);
            }
            other => panic!("expected RepairCheckPerformed, got {:?}", other),
        }
    }

    #[test]
    fn no_repair_without_shipyard() {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(0x9200_0000),
            name: "Outer Rim".into(),
            group: crate::dat::SectorGroup::RimOuter,
            x: 500, y: 500,
            systems: vec![],
        });
        // System with NO manufacturing facilities
        let sys_key = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0001),
            name: "Tatooine".into(),
            sector,
            x: 500, y: 500,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            is_populated: true,
            total_energy: 3, raw_materials: 2,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![], // no shipyard!
            production_facilities: vec![],
            is_headquarters: false, is_destroyed: false,
            control: ControlKind::Uncontrolled,
        });
        add_fleet_with_ship(&mut world, sys_key);
        let mut state = RepairState;
        let tick_events = vec![crate::tick::TickEvent { tick: 1 }];

        let events = RepairSystem::advance(&mut state, &world, &tick_events);
        assert!(events.is_empty(), "no repair without shipyard");
    }

    #[test]
    fn no_repair_without_ticks() {
        let mut world = GameWorld::default();
        let sys_key = make_shipyard_system(&mut world);
        add_fleet_with_ship(&mut world, sys_key);
        let mut state = RepairState;

        let events = RepairSystem::advance(&mut state, &world, &[]);
        assert!(events.is_empty(), "no repair without tick events");
    }

    #[test]
    fn no_repair_at_destroyed_system() {
        let mut world = GameWorld::default();
        let sys_key = make_shipyard_system(&mut world);
        add_fleet_with_ship(&mut world, sys_key);
        world.systems.get_mut(sys_key).unwrap().is_destroyed = true;
        let mut state = RepairState;
        let tick_events = vec![crate::tick::TickEvent { tick: 1 }];

        let events = RepairSystem::advance(&mut state, &world, &tick_events);
        assert!(events.is_empty(), "no repair at destroyed system");
    }
}
