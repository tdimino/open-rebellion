//! Fleet cargo for troop regiments.
//!
//! The original game associates a regiment in transit with a capital-ship
//! transport and limits carried regiments by the ships' `troop_capacity`.
//! Surface garrisons remain in `System::ground_units`; embarked regiments live
//! only in this state until they are landed or their transport is destroyed.

use std::collections::{HashMap, HashSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::ids::{FleetKey, SystemKey, TroopKey};
use crate::world::GameWorld;

/// Persistent regiment cargo keyed by its carrying fleet.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TroopTransportState {
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    cargo: HashMap<FleetKey, Vec<TroopKey>>,
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
    CapacityExceeded { capacity: u32, requested: u32 },
    FleetNotAtDestination,
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
        }
    }
}

impl TroopTransportState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Troops currently carried by `fleet`, in stable key order.
    pub fn cargo(&self, fleet: FleetKey) -> &[TroopKey] {
        self.cargo.get(&fleet).map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn carried_count(&self, fleet: FleetKey) -> usize {
        self.cargo(fleet).len()
    }

    pub fn is_empty(&self) -> bool {
        self.cargo.is_empty()
    }

    pub fn is_embarked(&self, troop: TroopKey) -> bool {
        self.cargo.values().any(|cargo| cargo.contains(&troop))
    }

    /// Fleet identities currently carrying at least one regiment.
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
        let cargo = self.cargo.entry(fleet).or_default();
        cargo.extend(troops.iter().copied());
        cargo.sort_unstable();
        cargo.dedup();
        Ok(())
    }

    /// Land every carried regiment at the fleet's current system.
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
        Ok(landed)
    }

    /// Return a selected set of cargo to the fleet's current system.
    ///
    /// This is primarily the atomic rollback path for a player departure that
    /// becomes invalid after embarkation. Cargo not named in `troops` remains
    /// aboard.
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
        if self.cargo.get(&fleet).is_some_and(Vec::is_empty) {
            self.cargo.remove(&fleet);
        }
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
        let Some(mut moved) = self.cargo.remove(&from) else {
            return;
        };
        let cargo = self.cargo.entry(to).or_default();
        cargo.append(&mut moved);
        cargo.sort_unstable();
        cargo.dedup();
    }

    /// Destroy every regiment aboard a fleet that has been destroyed.
    pub fn destroy_fleet_cargo(&mut self, world: &mut GameWorld, fleet: FleetKey) -> Vec<TroopKey> {
        let cargo = self.cargo.remove(&fleet).unwrap_or_default();
        for troop in &cargo {
            world.troops.remove(*troop);
        }
        cargo
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
            let capacity = Self::fleet_capacity(world, fleet).map(|value| value as usize);
            let lost = match capacity {
                None => self.cargo.remove(&fleet).unwrap_or_default(),
                Some(capacity) => {
                    let cargo = self.cargo.entry(fleet).or_default();
                    if cargo.len() <= capacity {
                        continue;
                    }
                    cargo.split_off(capacity)
                }
            };
            for troop in &lost {
                world.troops.remove(*troop);
            }
            if self.cargo.get(&fleet).is_some_and(Vec::is_empty) {
                self.cargo.remove(&fleet);
            }
            destroyed.push((fleet, lost));
        }
        destroyed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn cargo_above_surviving_transport_capacity_is_destroyed() {
        let (mut world, _, _, fleet, troops) = fixture(3);
        let transport_class = world.fleets[fleet].capital_ships[0].class;
        let survivor_class = world.capital_ship_classes.insert(CapitalShipClass {
            name: "Surviving Transport".into(),
            is_alliance: true,
            troop_capacity: 1,
            hull: 100,
            ..CapitalShipClass::default()
        });
        world.fleets[fleet]
            .capital_ships
            .push(ShipInstance::new(survivor_class, 100, true));
        let mut state = TroopTransportState::new();
        state.embark(&mut world, fleet, &troops).unwrap();
        world.fleets[fleet]
            .capital_ships
            .iter_mut()
            .find(|ship| ship.class == transport_class)
            .unwrap()
            .alive = false;

        let destroyed = state.destroy_untransportable_cargo(&mut world);

        assert_eq!(state.cargo(fleet), &troops[..1]);
        assert_eq!(destroyed, vec![(fleet, troops[1..].to_vec())]);
        assert!(world.troops.contains_key(troops[0]));
        assert!(!world.troops.contains_key(troops[1]));
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
}
