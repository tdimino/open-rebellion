//! Fog of war and intel system.
//!
//! Each faction starts with knowledge of only the systems they physically
//! occupy (fleet or character present). As fleets move and characters are
//! dispatched, new systems are revealed and added to that faction's
//! `FogState`.
//!
//! # Visibility rules (simplified Phase 0)
//!
//! A system is **visible** to a faction if:
//! 1. The faction has at least one fleet at that system (`Fleet.location`), OR
//! 2. The faction has at least one character assigned to that system via a fleet.
//!
//! Once revealed a system stays revealed (no fog-of-war regression in this
//! phase). Full sensor/probe-droid range is deferred to a later milestone.
//!
//! # Usage
//!
//! ```ignore
//! use rebellion_core::fog::{FogState, FogSystem};
//! use rebellion_core::dat::Faction;
//!
//! let mut fog = FogState::new(Faction::Alliance);
//! // Seed with all systems containing Alliance fleets at game start:
//! FogSystem::seed(&mut fog, &world);
//!
//! // Each tick after fleet movement resolves:
//! let reveals = FogSystem::advance(&mut fog, &world, &movement_state);
//! ```

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::dat::Faction;
use crate::ids::SystemKey;
use crate::movement::MovementState;
use crate::world::GameWorld;

// ---------------------------------------------------------------------------
// FogState
// ---------------------------------------------------------------------------

/// Per-faction visibility state: the set of systems currently known to
/// this faction.
///
/// Systems are added monotonically — once revealed they stay revealed.
/// A fleet arriving at a new system via `MovementState` will reveal it
/// on the next `FogSystem::advance` call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FogState {
    /// Which faction owns this fog view.
    pub faction: Faction,
    /// All systems currently visible to this faction.
    pub visible: HashSet<SystemKey>,
}

impl FogState {
    pub fn new(faction: Faction) -> Self {
        FogState {
            faction,
            visible: HashSet::new(),
        }
    }

    /// Returns true if `system` is currently visible to this faction.
    #[inline]
    pub fn is_visible(&self, system: SystemKey) -> bool {
        self.visible.contains(&system)
    }

    /// Mark a system as revealed (idempotent).
    pub fn reveal(&mut self, system: SystemKey) {
        self.visible.insert(system);
    }

    pub fn len(&self) -> usize {
        self.visible.len()
    }

    pub fn is_empty(&self) -> bool {
        self.visible.is_empty()
    }
}

// ---------------------------------------------------------------------------
// RevealEvent
// ---------------------------------------------------------------------------

/// Emitted when a system becomes newly visible to a faction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevealEvent {
    /// The faction that gained visibility.
    pub faction: Faction,
    /// The system that was revealed.
    pub system: SystemKey,
}

// ---------------------------------------------------------------------------
// FogSystem
// ---------------------------------------------------------------------------

/// Stateless system that updates fog-of-war visibility.
pub struct FogSystem;

impl FogSystem {
    /// Seed initial visibility from the current world state.
    ///
    /// Call once at game start (or after loading a save) to populate
    /// `FogState` with all systems the faction already occupies.
    /// Idempotent — safe to call multiple times.
    pub fn seed(fog: &mut FogState, world: &GameWorld) {
        let is_alliance = fog.faction == Faction::Alliance;
        for (_fleet_key, fleet) in world.fleets.iter() {
            if fleet.is_alliance == is_alliance {
                fog.reveal(fleet.location);
            }
        }
    }

    /// Advance fog-of-war after fleet movement has resolved for this tick.
    ///
    /// Scans all fleets (stationary and in-transit destination) belonging to
    /// this faction and reveals any system not yet visible.
    ///
    /// Returns one `RevealEvent` per newly-revealed system.
    pub fn advance(
        fog: &mut FogState,
        world: &GameWorld,
        movement_state: &MovementState,
    ) -> Vec<RevealEvent> {
        let is_alliance = fog.faction == Faction::Alliance;
        let mut events = Vec::new();

        // Stationary fleets reveal their current location.
        for (_fleet_key, fleet) in world.fleets.iter() {
            if fleet.is_alliance != is_alliance {
                continue;
            }
            if !fog.is_visible(fleet.location) {
                fog.reveal(fleet.location);
                events.push(RevealEvent {
                    faction: fog.faction,
                    system: fleet.location,
                });
            }
        }

        // In-transit fleets reveal their *destination* on the tick they
        // arrive. We check the destination of every active order whose
        // fleet belongs to this faction.
        for (_fleet_key, order) in movement_state.orders().iter() {
            // Look up the fleet to determine faction.
            let fleet = match world.fleets.get(order.fleet) {
                Some(f) => f,
                None => continue,
            };
            if fleet.is_alliance != is_alliance {
                continue;
            }
            // Reveal destination once the fleet is close (progress >= 0.5)
            // so the player gets advance intel as the fleet approaches.
            if order.progress() >= 0.5 && !fog.is_visible(order.destination) {
                fog.reveal(order.destination);
                events.push(RevealEvent {
                    faction: fog.faction,
                    system: order.destination,
                });
            }
        }

        // Sensor-radius reveals: fleets with detection capability reveal nearby systems.
        const SENSOR_MULTIPLIER: f32 = 15.0; // coordinate units per detection point
        for (_, fleet) in world.fleets.iter() {
            if fleet.is_alliance != is_alliance {
                continue; // skip enemy fleets
            }
            // Find max detection from fleet's ship classes.
            let max_detection = fleet.capital_ships.iter()
                .filter_map(|entry| world.capital_ship_classes.get(entry.class))
                .map(|c| c.detection)
                .max()
                .unwrap_or(0);
            if max_detection == 0 {
                continue;
            }
            // Use f64 to avoid i64 overflow at extreme detection values.
            let radius = max_detection as f64 * SENSOR_MULTIPLIER as f64;
            let radius_sq = radius * radius;

            if let Some(fleet_sys) = world.systems.get(fleet.location) {
                let fx = fleet_sys.x as f64;
                let fy = fleet_sys.y as f64;
                for (sys_key, sys) in world.systems.iter() {
                    if fog.is_visible(sys_key) {
                        continue; // already visible
                    }
                    let dx = sys.x as f64 - fx;
                    let dy = sys.y as f64 - fy;
                    if dx * dx + dy * dy <= radius_sq {
                        fog.reveal(sys_key);
                        events.push(RevealEvent {
                            faction: fog.faction,
                            system: sys_key,
                        });
                    }
                }
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
    use crate::world::ControlKind;
    use crate::ids::{FleetKey, SystemKey};
    use crate::movement::MovementState;
    use crate::world::{Fleet, GameWorld};

    // Build a minimal GameWorld with N systems and M alliance fleets.
    fn make_world_with_fleets(
        n_systems: usize,
        fleet_locations: &[usize], // indices into systems vec
        is_alliance: bool,
    ) -> (GameWorld, Vec<SystemKey>, Vec<FleetKey>) {
        let mut world = GameWorld::default();

        // Create a dummy sector key for systems.
        let sector = world.sectors.insert(crate::world::Sector {
            dat_id: crate::ids::DatId(0),
            name: "Test Sector".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });

        let sys_keys: Vec<SystemKey> = (0..n_systems)
            .map(|i| {
                world.systems.insert(crate::world::System {
                    dat_id: crate::ids::DatId(i as u32),
                    name: format!("System {}", i),
                    sector,
                    x: i as u16 * 10,
                    y: 0,
                    exploration_status: crate::dat::ExplorationStatus::Explored,
                    popularity_alliance: 0.5,
                    popularity_empire: 0.5,
                    fleets: vec![],
                    ground_units: vec![],
                    special_forces: vec![],
                    defense_facilities: vec![],
                    manufacturing_facilities: vec![],
                    production_facilities: vec![],
                    is_headquarters: false,
                    is_destroyed: false,
                    control: ControlKind::Uncontrolled,
                })
            })
            .collect();

        let fleet_keys: Vec<FleetKey> = fleet_locations
            .iter()
            .map(|&idx| {
                world.fleets.insert(Fleet {
                    location: sys_keys[idx],
                    capital_ships: vec![],
                    fighters: vec![],
                    characters: vec![],
                    is_alliance,
                    has_death_star: false,
                })
            })
            .collect();

        (world, sys_keys, fleet_keys)
    }

    #[test]
    fn new_fog_is_empty() {
        let fog = FogState::new(Faction::Alliance);
        assert!(fog.is_empty());
    }

    #[test]
    fn reveal_makes_system_visible() {
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let key = sys_sm.insert(());
        let mut fog = FogState::new(Faction::Alliance);
        fog.reveal(key);
        assert!(fog.is_visible(key));
        assert_eq!(fog.len(), 1);
    }

    #[test]
    fn reveal_is_idempotent() {
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let key = sys_sm.insert(());
        let mut fog = FogState::new(Faction::Alliance);
        fog.reveal(key);
        fog.reveal(key);
        assert_eq!(fog.len(), 1);
    }

    #[test]
    fn seed_reveals_faction_fleet_locations() {
        let (world, sys_keys, _) = make_world_with_fleets(3, &[0, 2], true);
        let mut fog = FogState::new(Faction::Alliance);
        FogSystem::seed(&mut fog, &world);
        assert!(fog.is_visible(sys_keys[0]));
        assert!(!fog.is_visible(sys_keys[1]));
        assert!(fog.is_visible(sys_keys[2]));
    }

    #[test]
    fn seed_ignores_enemy_fleets() {
        // Empire fleets should not reveal for Alliance fog.
        let (world, sys_keys, _) = make_world_with_fleets(2, &[0], false); // empire fleet at sys 0
        let mut fog = FogState::new(Faction::Alliance);
        FogSystem::seed(&mut fog, &world);
        assert!(!fog.is_visible(sys_keys[0]));
    }

    #[test]
    fn advance_reveals_new_fleet_location() {
        let (world, sys_keys, _) = make_world_with_fleets(2, &[0, 1], true);
        let mut fog = FogState::new(Faction::Alliance);
        // Pre-reveal sys 0 only.
        fog.reveal(sys_keys[0]);
        let ms = MovementState::new();
        let events = FogSystem::advance(&mut fog, &world, &ms);
        // sys 1 should now be revealed.
        assert!(fog.is_visible(sys_keys[1]));
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].system, sys_keys[1]);
        assert_eq!(events[0].faction, Faction::Alliance);
    }

    #[test]
    fn advance_no_new_systems_no_events() {
        let (world, sys_keys, _) = make_world_with_fleets(1, &[0], true);
        let mut fog = FogState::new(Faction::Alliance);
        fog.reveal(sys_keys[0]);
        let ms = MovementState::new();
        let events = FogSystem::advance(&mut fog, &world, &ms);
        assert!(events.is_empty());
    }

    #[test]
    fn advance_in_transit_reveals_destination_at_half_progress() {
        // Build world with 2 systems and 1 Alliance fleet at sys 0.
        let (world, sys_keys, fleet_keys) = make_world_with_fleets(2, &[0], true);
        let mut fog = FogState::new(Faction::Alliance);
        fog.reveal(sys_keys[0]);

        let mut ms = MovementState::new();
        // Issue order: fleet at sys_keys[0] → sys_keys[1], 10 ticks.
        ms.order(fleet_keys[0], sys_keys[0], sys_keys[1], 10);

        // At 4 ticks elapsed (40%) — destination not yet revealed.
        ms.orders_mut().get_mut(&fleet_keys[0]).unwrap().ticks_elapsed = 4;
        let events = FogSystem::advance(&mut fog, &world, &ms);
        assert!(!fog.is_visible(sys_keys[1]));
        // The stationary fleet at sys 0 is already known, so no stationary
        // reveal event either.
        assert!(events.is_empty());

        // At 5 ticks elapsed (50%) — destination revealed.
        ms.orders_mut().get_mut(&fleet_keys[0]).unwrap().ticks_elapsed = 5;
        let events = FogSystem::advance(&mut fog, &world, &ms);
        assert!(fog.is_visible(sys_keys[1]));
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].system, sys_keys[1]);
    }

    #[test]
    fn advance_ignores_enemy_transit_orders() {
        let (world, sys_keys, fleet_keys) = make_world_with_fleets(2, &[0], false); // empire fleet
        let mut fog = FogState::new(Faction::Alliance);

        let mut ms = MovementState::new();
        ms.order(fleet_keys[0], sys_keys[0], sys_keys[1], 10);
        ms.orders_mut().get_mut(&fleet_keys[0]).unwrap().ticks_elapsed = 8;

        let events = FogSystem::advance(&mut fog, &world, &ms);
        assert!(events.is_empty());
        assert!(!fog.is_visible(sys_keys[1]));
    }

    #[test]
    fn empire_fog_tracks_empire_fleets() {
        let (world, sys_keys, _) = make_world_with_fleets(2, &[1], false); // empire at sys 1
        let mut fog = FogState::new(Faction::Empire);
        FogSystem::seed(&mut fog, &world);
        assert!(!fog.is_visible(sys_keys[0]));
        assert!(fog.is_visible(sys_keys[1]));
    }

    #[test]
    fn sensor_radius_reveals_nearby_systems() {
        use crate::ids::DatId;
        use crate::world::{CapitalShipClass, ShipEntry};

        let mut world = GameWorld::default();
        let sector = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0),
            name: "Test".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0, y: 0,
            systems: vec![],
        });

        // Fleet at (0, 0), nearby system at (100, 0), far system at (1000, 0).
        // detection=10 → radius = 10 * 15.0 = 150 units → (100,0) within range, (1000,0) not.
        let sys0 = world.systems.insert(crate::world::System {
            dat_id: DatId(0), name: "Base".into(), sector,
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![], is_headquarters: false,
            is_destroyed: false, control: ControlKind::Uncontrolled,
        });
        let sys_near = world.systems.insert(crate::world::System {
            dat_id: DatId(1), name: "Near".into(), sector,
            x: 100, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![], is_headquarters: false,
            is_destroyed: false, control: ControlKind::Uncontrolled,
        });
        let sys_far = world.systems.insert(crate::world::System {
            dat_id: DatId(2), name: "Far".into(), sector,
            x: 1000, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![], is_headquarters: false,
            is_destroyed: false, control: ControlKind::Uncontrolled,
        });

        let ship_class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId(0), name: "Sensor Ship".into(),
            is_alliance: true, is_empire: false,
            refined_material_cost: 100, maintenance_cost: 10,
            research_order: 0, research_difficulty: 0,
            hull: 100, shield_strength: 50,
            sub_light_engine: 5, maneuverability: 5, hyperdrive: 2,
            fighter_capacity: 0, troop_capacity: 0,
            detection: 10,
            turbolaser_fore: 0, turbolaser_aft: 0, turbolaser_port: 0, turbolaser_starboard: 0,
            ion_cannon_fore: 0, ion_cannon_aft: 0, ion_cannon_port: 0, ion_cannon_starboard: 0,
            laser_cannon_fore: 0, laser_cannon_aft: 0, laser_cannon_port: 0, laser_cannon_starboard: 0,
            shield_recharge_rate: 0, damage_control: 0, bombardment_modifier: 0,
        });

        world.fleets.insert(Fleet {
            location: sys0,
            capital_ships: vec![ShipEntry { class: ship_class, count: 1 }],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });

        let mut fog = FogState::new(Faction::Alliance);
        fog.reveal(sys0); // fleet's own system already visible
        let ms = MovementState::new();
        let events = FogSystem::advance(&mut fog, &world, &ms);

        assert!(fog.is_visible(sys_near), "nearby system should be revealed by sensor radius");
        assert!(!fog.is_visible(sys_far), "far system should NOT be revealed");
        assert!(events.iter().any(|e| e.system == sys_near));
        assert!(!events.iter().any(|e| e.system == sys_far));
    }

    #[test]
    fn sensor_detection_zero_reveals_only_own_system() {
        use crate::ids::DatId;
        use crate::world::{CapitalShipClass, ShipEntry};

        let mut world = GameWorld::default();
        let sector = world.sectors.insert(crate::world::Sector {
            dat_id: DatId(0),
            name: "Test".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0, y: 0,
            systems: vec![],
        });

        let sys0 = world.systems.insert(crate::world::System {
            dat_id: DatId(0), name: "Base".into(), sector,
            x: 0, y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![], is_headquarters: false,
            is_destroyed: false, control: ControlKind::Uncontrolled,
        });
        let sys_near = world.systems.insert(crate::world::System {
            dat_id: DatId(1), name: "Near".into(), sector,
            x: 10, y: 0, // very close
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![], is_headquarters: false,
            is_destroyed: false, control: ControlKind::Uncontrolled,
        });

        // Ship with detection=0
        let ship_class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId(0), name: "Blind Ship".into(),
            is_alliance: true, is_empire: false,
            refined_material_cost: 100, maintenance_cost: 10,
            research_order: 0, research_difficulty: 0,
            hull: 100, shield_strength: 50,
            sub_light_engine: 5, maneuverability: 5, hyperdrive: 2,
            fighter_capacity: 0, troop_capacity: 0,
            detection: 0, // no detection
            turbolaser_fore: 0, turbolaser_aft: 0, turbolaser_port: 0, turbolaser_starboard: 0,
            ion_cannon_fore: 0, ion_cannon_aft: 0, ion_cannon_port: 0, ion_cannon_starboard: 0,
            laser_cannon_fore: 0, laser_cannon_aft: 0, laser_cannon_port: 0, laser_cannon_starboard: 0,
            shield_recharge_rate: 0, damage_control: 0, bombardment_modifier: 0,
        });

        world.fleets.insert(Fleet {
            location: sys0,
            capital_ships: vec![ShipEntry { class: ship_class, count: 1 }],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });

        let mut fog = FogState::new(Faction::Alliance);
        fog.reveal(sys0);
        let ms = MovementState::new();
        let events = FogSystem::advance(&mut fog, &world, &ms);

        assert!(fog.is_visible(sys0), "own system should be visible");
        assert!(!fog.is_visible(sys_near), "detection=0 should not reveal nearby systems");
        assert!(events.is_empty());
    }
}
