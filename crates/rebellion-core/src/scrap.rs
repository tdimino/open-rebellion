//! Scrapping: a unit or facility removed by its side, returning half its
//! refined cost (`FUN_00530270`) and its maintenance
//! (`ghidra/notes/top-bar-resource-counters.md`).

use serde::{Deserialize, Serialize};

use crate::dat::Faction;
use crate::ids::{
    DefenseFacilityKey, FleetKey, ManufacturingFacilityKey, ProductionFacilityKey, SpecialForceKey,
    TroopKey,
};
use crate::stockpiles::StockpileState;
use crate::troop_transport::TroopTransportState;
use crate::world::GameWorld;

/// One object to scrap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrapTarget {
    /// A regiment (TROOPSD, family `0x10`).
    Troop(TroopKey),
    /// A capital ship: its fleet and its index in the fleet's hulls.
    CapitalShip { fleet: FleetKey, index: usize },
    /// One fighter squadron: its fleet and its fighter entry.
    Fighter { fleet: FleetKey, entry: usize },
    /// One squadron a system holds, of a class and side (`FUN_00503a50`).
    SystemFighter {
        system: crate::ids::SystemKey,
        class: crate::ids::FighterKey,
        is_alliance: bool,
    },
    /// A planetary defense (DEFFACSD).
    Defense(DefenseFacilityKey),
    /// A shipyard, training facility or construction yard (MANFACSD).
    Manufacturing(ManufacturingFacilityKey),
    /// A mine or refinery (PROFACSD).
    Production(ProductionFacilityKey),
    /// A special force (SPECFCSD, family `0x3c`).
    SpecialForce(SpecialForceKey),
    /// A squadron travelling on its own (`GameWorld::squadron_transits`): the
    /// original holds it in its new container while en route
    /// (`FUN_00556430`), so it is the side's like any other squadron.
    TravellingSquadron {
        class: crate::ids::FighterKey,
        is_alliance: bool,
        leg: crate::troop_transport::RegimentLeg,
        arrival_tick: u64,
    },
}

/// The index in `world.squadron_transits` of the travelling squadron a
/// [`ScrapTarget::TravellingSquadron`] names.
fn travelling_squadron(
    world: &GameWorld,
    class: crate::ids::FighterKey,
    is_alliance: bool,
    leg: crate::troop_transport::RegimentLeg,
    arrival_tick: u64,
) -> Option<usize> {
    world.squadron_transits.iter().position(|transit| {
        transit.class == class
            && transit.is_alliance == is_alliance
            && transit.leg == leg
            && transit.arrival_tick == arrival_tick
    })
}

/// The side's objects the overdraft timer may scrap (`FUN_0052e530`): those
/// that exist, are the side's, and cost maintenance. Mines and refineries
/// cost none, so they are never picked. Listed in the walk's family order
/// (`FUN_0052e8b0`: families `0x10..0x3f`): regiments, capital ships,
/// fighter squadrons (in fleets, at systems, then travelling), defenses,
/// yards, special forces. port: within a family, arena order stands in for
/// the original's object order.
#[must_use]
pub fn overdraft_candidates(world: &GameWorld, side: Faction) -> Vec<ScrapTarget> {
    let is_alliance = side == Faction::Alliance;
    let costs = |id| {
        world
            .buildable_classes
            .get(id)
            .is_some_and(|class| class.maintenance_cost > 0)
    };
    let mut candidates = Vec::new();
    candidates.extend(
        world
            .troops
            .iter()
            .filter(|(_, troop)| troop.is_alliance == is_alliance && costs(&troop.class_dat_id))
            .map(|(key, _)| ScrapTarget::Troop(key)),
    );
    let fleets = || {
        world
            .fleets
            .iter()
            .filter(move |(_, fleet)| fleet.is_alliance == is_alliance)
    };
    for (key, fleet) in fleets() {
        candidates.extend(
            fleet
                .capital_ships
                .iter()
                .enumerate()
                .filter(|(_, ship)| {
                    ship.alive
                        && world
                            .capital_ship_classes
                            .get(ship.class)
                            .is_some_and(|class| class.maintenance_cost > 0)
                })
                .map(|(index, _)| ScrapTarget::CapitalShip { fleet: key, index }),
        );
    }
    for (key, fleet) in fleets() {
        for (entry, fighters) in fleet.fighters.iter().enumerate() {
            let costs = world
                .fighter_classes
                .get(fighters.class)
                .is_some_and(|class| class.maintenance_cost > 0);
            if costs {
                candidates.extend(
                    (0..fighters.count).map(|_| ScrapTarget::Fighter { fleet: key, entry }),
                );
            }
        }
    }
    for (system, value) in &world.systems {
        for fighters in value
            .fighters
            .iter()
            .filter(|entry| entry.is_alliance == is_alliance)
        {
            let costs = world
                .fighter_classes
                .get(fighters.class)
                .is_some_and(|class| class.maintenance_cost > 0);
            if costs {
                candidates.extend((0..fighters.count).map(|_| ScrapTarget::SystemFighter {
                    system,
                    class: fighters.class,
                    is_alliance,
                }));
            }
        }
    }
    candidates.extend(
        world
            .squadron_transits
            .iter()
            .filter(|transit| {
                transit.is_alliance == is_alliance
                    && world
                        .fighter_classes
                        .get(transit.class)
                        .is_some_and(|class| class.maintenance_cost > 0)
            })
            .map(|transit| ScrapTarget::TravellingSquadron {
                class: transit.class,
                is_alliance,
                leg: transit.leg,
                arrival_tick: transit.arrival_tick,
            }),
    );
    candidates.extend(
        world
            .defense_facilities
            .iter()
            .filter(|(_, facility)| facility.side == side && costs(&facility.class_dat_id))
            .map(|(key, _)| ScrapTarget::Defense(key)),
    );
    candidates.extend(
        world
            .manufacturing_facilities
            .iter()
            .filter(|(_, facility)| facility.side == side && costs(&facility.class_dat_id))
            .map(|(key, _)| ScrapTarget::Manufacturing(key)),
    );
    candidates.extend(
        world
            .special_forces
            .iter()
            .filter(|(_, unit)| unit.is_alliance == is_alliance && costs(&unit.class_dat_id))
            .map(|(key, _)| ScrapTarget::SpecialForce(key)),
    );
    candidates
}

/// What a scrap removed: its side and the refined material returned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Scrapped {
    pub side: Faction,
    pub refund: i64,
}

/// The side and refined cost of `target`, if it still exists.
fn owner_and_cost(world: &GameWorld, target: ScrapTarget) -> Option<(Faction, u32)> {
    let class_cost = |id| {
        world
            .buildable_classes
            .get(id)
            .map_or(0, |class| class.refined_material_cost)
    };
    match target {
        ScrapTarget::Troop(key) => world.troops.get(key).map(|troop| {
            (
                Faction::of_alliance(troop.is_alliance),
                class_cost(&troop.class_dat_id),
            )
        }),
        ScrapTarget::SpecialForce(key) => world.special_forces.get(key).map(|unit| {
            (
                Faction::of_alliance(unit.is_alliance),
                class_cost(&unit.class_dat_id),
            )
        }),
        ScrapTarget::CapitalShip { fleet, index } => {
            let value = world.fleets.get(fleet)?;
            let ship = value.capital_ships.get(index).filter(|ship| ship.alive)?;
            let cost = world
                .capital_ship_classes
                .get(ship.class)
                .map_or(0, |class| class.refined_material_cost);
            Some((Faction::of_alliance(value.is_alliance), cost))
        }
        ScrapTarget::Fighter { fleet, entry } => {
            let value = world.fleets.get(fleet)?;
            let fighters = value.fighters.get(entry).filter(|entry| entry.count > 0)?;
            let cost = world
                .fighter_classes
                .get(fighters.class)
                .map_or(0, |class| class.refined_material_cost);
            Some((Faction::of_alliance(value.is_alliance), cost))
        }
        ScrapTarget::SystemFighter {
            system,
            class,
            is_alliance,
        } => {
            let held = world.systems.get(system)?.fighters.iter().any(|entry| {
                entry.class == class && entry.is_alliance == is_alliance && entry.count > 0
            });
            if !held {
                return None;
            }
            let cost = world
                .fighter_classes
                .get(class)
                .map_or(0, |value| value.refined_material_cost);
            Some((Faction::of_alliance(is_alliance), cost))
        }
        ScrapTarget::TravellingSquadron {
            class,
            is_alliance,
            leg,
            arrival_tick,
        } => {
            travelling_squadron(world, class, is_alliance, leg, arrival_tick)?;
            let cost = world
                .fighter_classes
                .get(class)
                .map_or(0, |value| value.refined_material_cost);
            Some((Faction::of_alliance(is_alliance), cost))
        }
        ScrapTarget::Defense(key) => world
            .defense_facilities
            .get(key)
            .map(|facility| (facility.side, class_cost(&facility.class_dat_id))),
        ScrapTarget::Manufacturing(key) => world
            .manufacturing_facilities
            .get(key)
            .map(|facility| (facility.side, class_cost(&facility.class_dat_id))),
        ScrapTarget::Production(key) => world
            .production_facilities
            .get(key)
            .map(|facility| (facility.side, class_cost(&facility.class_dat_id))),
    }
}

/// Scrap `target`: remove it and return half its class's refined cost to
/// its side's stockpile (`FUN_00530270`: an object destroyed with a scrap
/// reason, `+0x40` byte `0x14..0x16`, adds `cost / 2` to side `+0x7c`). Its
/// maintenance returns with it, as the load is the sum over what exists
/// (`FUN_0052fff0`). A scrapped transport's regiments are lost with it.
/// `None` when the target no longer exists.
pub fn scrap(
    world: &mut GameWorld,
    stockpiles: &mut StockpileState,
    transport: &mut TroopTransportState,
    target: ScrapTarget,
) -> Option<Scrapped> {
    let (side, cost) = owner_and_cost(world, target)?;
    match target {
        ScrapTarget::Troop(key) => {
            if !transport.destroy_embarked(world, key) {
                transport.forget_transit(key);
                for (_, system) in &mut world.systems {
                    system.ground_units.retain(|&k| k != key);
                }
                world.troops.remove(key);
            }
        }
        ScrapTarget::SpecialForce(key) => crate::missions::destroy_special_force(world, key),
        ScrapTarget::CapitalShip { fleet, index } => {
            if let Some(ship) = world
                .fleets
                .get_mut(fleet)
                .and_then(|value| value.capital_ships.get_mut(index))
            {
                ship.alive = false;
            }
            // hyp: the squadrons aboard go with their ship (carriage).
            if let Some(value) = world.fleets.get_mut(fleet) {
                crate::carriage::drop_lost_squadrons(value);
            }
            remove_if_empty(world, fleet);
            transport.destroy_untransportable_cargo(world);
        }
        ScrapTarget::Fighter { fleet, entry } => {
            if let Some(fighters) = world
                .fleets
                .get_mut(fleet)
                .and_then(|value| value.fighters.get_mut(entry))
            {
                fighters.count -= 1;
            }
            remove_if_empty(world, fleet);
        }
        ScrapTarget::SystemFighter {
            system,
            class,
            is_alliance,
        } => {
            if let Some(value) = world.systems.get_mut(system) {
                value.take_squadron(class, is_alliance);
            }
        }
        ScrapTarget::TravellingSquadron {
            class,
            is_alliance,
            leg,
            arrival_tick,
        } => {
            if let Some(index) = travelling_squadron(world, class, is_alliance, leg, arrival_tick) {
                world.squadron_transits.remove(index);
            }
        }
        ScrapTarget::Defense(key) => {
            crate::missions::destroy_target(
                world,
                crate::missions::MissionTarget::DefenseFacility(key),
            );
        }
        ScrapTarget::Manufacturing(key) => {
            crate::missions::destroy_target(
                world,
                crate::missions::MissionTarget::ManufacturingFacility(key),
            );
        }
        ScrapTarget::Production(key) => {
            crate::missions::destroy_target(
                world,
                crate::missions::MissionTarget::ProductionFacility(key),
            );
        }
    }
    let refund = i64::from(cost / 2);
    stockpiles.side_mut(side).refined += refund;
    Some(Scrapped { side, refund })
}

/// Scrap every one of `targets`, one order's worth, and return the refund.
/// Ship indices hold while the order runs, so a scrapped hull stays in its
/// fleet, dead, until the end; then it goes, as every other lost ship does
/// (`retain(|ship| ship.alive)` in the integrator, missions and tactical
/// results).
pub fn scrap_all(
    world: &mut GameWorld,
    stockpiles: &mut StockpileState,
    transport: &mut TroopTransportState,
    targets: impl IntoIterator<Item = ScrapTarget>,
) -> i64 {
    let mut refund = 0;
    let mut fleets = Vec::new();
    for target in targets {
        if let ScrapTarget::CapitalShip { fleet, .. } = target {
            fleets.push(fleet);
        }
        if let Some(scrapped) = scrap(world, stockpiles, transport, target) {
            refund += scrapped.refund;
        }
    }
    for fleet in fleets {
        if let Some(value) = world.fleets.get_mut(fleet) {
            value.capital_ships.retain(|ship| ship.alive);
        }
    }
    refund
}

/// Removes `fleet` once its last unit is gone: `FUN_00530270` destroys the
/// fleet object with its members. hyp: a character aboard stays in the
/// system.
fn remove_if_empty(world: &mut GameWorld, fleet: FleetKey) {
    let Some(value) = world.fleets.get(fleet).filter(|value| value.is_empty()) else {
        return;
    };
    let (location, characters) = (value.location, value.characters.clone());
    for key in characters {
        if let Some(character) = world.characters.get_mut(key) {
            character.current_fleet = None;
            character.current_system = Some(location);
        }
    }
    if let Some(system) = world.systems.get_mut(location) {
        system.fleets.retain(|&key| key != fleet);
    }
    world.fleets.remove(fleet);
}

/// What a player's Scrap order (`0x200`) names: one object from its own
/// menu, or a sector window icon's team (`FUN_00512700`; facilities
/// `FUN_0053b6e0`, kind 4; regiments and defenses, kind 8; fleets,
/// `FUN_004ffe70`, kind `0x10`; `ghidra/notes/sector-icon-menus.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrapOrder {
    Fleet(FleetKey),
    Ship {
        fleet: FleetKey,
        index: usize,
    },
    Troop(TroopKey),
    /// One squadron of a fleet, from its own menu.
    Fighter {
        fleet: FleetKey,
        entry: usize,
    },
    /// One squadron a system holds, from its own menu.
    SystemFighter {
        system: crate::ids::SystemKey,
        class: crate::ids::FighterKey,
        is_alliance: bool,
    },
    /// One planetary defense, from its own menu.
    Defense(crate::ids::DefenseFacilityKey),
    /// A system's yards, mines and refineries of the side.
    Facilities(crate::ids::SystemKey),
    /// A system's regiments, squadrons and defenses of the side: the
    /// Defenses icon's team (`FUN_00512700` kind 8).
    Defenses(crate::ids::SystemKey),
    /// A system's fleets of the side.
    Fleets(crate::ids::SystemKey),
}

/// The side's fleets orbiting `system`.
fn side_fleets(world: &GameWorld, system: crate::ids::SystemKey, side: Faction) -> Vec<FleetKey> {
    world.systems.get(system).map_or_else(Vec::new, |value| {
        value
            .fleets
            .iter()
            .copied()
            .filter(|&fleet| {
                world
                    .fleets
                    .get(fleet)
                    .is_some_and(|value| Faction::of_alliance(value.is_alliance) == side)
            })
            .collect()
    })
}

/// A fleet's members: each squadron, then its ships, so a ship's scrap
/// drops no squadron a later target names. `order_targets` keeps only
/// those that exist.
/// hyp: scrapping a fleet (`FUN_004f84e0` on the fleet object) scraps what
/// it carries; the fleet's own destroy handler is untraced.
fn fleet_members(world: &GameWorld, fleet: FleetKey) -> Vec<ScrapTarget> {
    let Some(value) = world.fleets.get(fleet) else {
        return Vec::new();
    };
    let ships = value
        .capital_ships
        .iter()
        .enumerate()
        .map(|(index, _)| ScrapTarget::CapitalShip { fleet, index });
    let squadrons = value
        .fighters
        .iter()
        .enumerate()
        .flat_map(|(entry, fighters)| {
            (0..fighters.count).map(move |_| ScrapTarget::Fighter { fleet, entry })
        });
    squadrons.chain(ships).collect()
}

/// Every object `order` scraps for `side`.
#[must_use]
pub fn order_targets(world: &GameWorld, order: ScrapOrder, side: Faction) -> Vec<ScrapTarget> {
    let held = |target: &ScrapTarget| {
        owner_and_cost(world, *target).is_some_and(|(owner, _)| owner == side)
    };
    let mut targets = match order {
        ScrapOrder::Fleet(fleet) => fleet_members(world, fleet),
        ScrapOrder::Ship { fleet, index } => vec![ScrapTarget::CapitalShip { fleet, index }],
        ScrapOrder::Troop(troop) => vec![ScrapTarget::Troop(troop)],
        ScrapOrder::Fighter { fleet, entry } => vec![ScrapTarget::Fighter { fleet, entry }],
        ScrapOrder::Defense(key) => vec![ScrapTarget::Defense(key)],
        ScrapOrder::SystemFighter {
            system,
            class,
            is_alliance,
        } => vec![ScrapTarget::SystemFighter {
            system,
            class,
            is_alliance,
        }],
        ScrapOrder::Facilities(system) => {
            world.systems.get(system).map_or_else(Vec::new, |value| {
                value
                    .manufacturing_facilities
                    .iter()
                    .map(|&key| ScrapTarget::Manufacturing(key))
                    .chain(
                        value
                            .production_facilities
                            .iter()
                            .map(|&key| ScrapTarget::Production(key)),
                    )
                    .collect()
            })
        }
        ScrapOrder::Defenses(system) => world.systems.get(system).map_or_else(Vec::new, |value| {
            value
                .ground_units
                .iter()
                .map(|&key| ScrapTarget::Troop(key))
                .chain(value.fighters.iter().flat_map(|fighters| {
                    (0..fighters.count).map(move |_| ScrapTarget::SystemFighter {
                        system,
                        class: fighters.class,
                        is_alliance: fighters.is_alliance,
                    })
                }))
                .chain(
                    value
                        .defense_facilities
                        .iter()
                        .map(|&key| ScrapTarget::Defense(key)),
                )
                .collect()
        }),
        ScrapOrder::Fleets(system) => side_fleets(world, system, side)
            .into_iter()
            .flat_map(|fleet| fleet_members(world, fleet))
            .collect(),
    };
    targets.retain(held);
    targets
}

/// The names the confirmation lists, one per object of the order's team
/// (`FUN_0049a880`: the object's name `+0x30`). A fleet's team is the fleet.
#[must_use]
pub fn order_names(world: &GameWorld, order: ScrapOrder, side: Faction) -> Vec<String> {
    let fleet_name = |fleet| world.fleet_name(fleet).unwrap_or_default().to_string();
    match order {
        ScrapOrder::Fleet(fleet) => vec![fleet_name(fleet)],
        ScrapOrder::Fleets(system) => side_fleets(world, system, side)
            .into_iter()
            .map(fleet_name)
            .collect(),
        _ => order_targets(world, order, side)
            .into_iter()
            .map(|target| target_name(world, target))
            .collect(),
    }
}

/// An object's name (`+0x30`): a ship's own name, else its class's.
#[must_use]
pub fn target_name(world: &GameWorld, target: ScrapTarget) -> String {
    let class_name = |id: &crate::ids::DatId| {
        world
            .buildable_classes
            .get(id)
            .map_or_else(String::new, |class| class.name.clone())
    };
    match target {
        ScrapTarget::Troop(key) => world
            .troops
            .get(key)
            .map_or_else(String::new, |troop| class_name(&troop.class_dat_id)),
        ScrapTarget::SpecialForce(key) => world
            .special_forces
            .get(key)
            .map_or_else(String::new, |unit| class_name(&unit.class_dat_id)),
        ScrapTarget::CapitalShip { fleet, index } => world
            .ship_name(fleet, index)
            .unwrap_or_default()
            .to_string(),
        ScrapTarget::Fighter { fleet, entry } => world
            .fleets
            .get(fleet)
            .and_then(|value| value.fighters.get(entry))
            .and_then(|fighters| world.fighter_classes.get(fighters.class))
            .map_or_else(String::new, |class| class.name.clone()),
        ScrapTarget::SystemFighter { class, .. }
        | ScrapTarget::TravellingSquadron { class, .. } => world
            .fighter_classes
            .get(class)
            .map_or_else(String::new, |value| value.name.clone()),
        ScrapTarget::Defense(key) => world
            .defense_facilities
            .get(key)
            .map_or_else(String::new, |facility| class_name(&facility.class_dat_id)),
        ScrapTarget::Manufacturing(key) => world
            .manufacturing_facilities
            .get(key)
            .map_or_else(String::new, |facility| class_name(&facility.class_dat_id)),
        ScrapTarget::Production(key) => world
            .production_facilities
            .get(key)
            .map_or_else(String::new, |facility| class_name(&facility.class_dat_id)),
    }
}

/// The system `target` stands at: the system that lists it, or for a
/// fleet's unit, or a regiment aboard one, the fleet's system; a regiment
/// travelling on its own stands at its destination.
#[must_use]
pub fn target_system(
    world: &GameWorld,
    transport: &TroopTransportState,
    target: ScrapTarget,
) -> Option<crate::ids::SystemKey> {
    let listed = |holds: &dyn Fn(&crate::world::System) -> bool| {
        world
            .systems
            .iter()
            .find(|(_, system)| holds(system))
            .map(|(key, _)| key)
    };
    let fleet_system = |fleet| world.fleets.get(fleet).map(|value| value.location);
    match target {
        ScrapTarget::Troop(key) => listed(&|system| system.ground_units.contains(&key))
            .or_else(|| {
                world
                    .fleets
                    .iter()
                    .find(|(fleet, _)| transport.cargo(*fleet).contains(&key))
                    .map(|(_, value)| value.location)
            })
            .or_else(|| transport.transit_system(world, key)),
        ScrapTarget::SpecialForce(key) => listed(&|system| system.special_forces.contains(&key)),
        ScrapTarget::CapitalShip { fleet, .. } | ScrapTarget::Fighter { fleet, .. } => {
            fleet_system(fleet)
        }
        ScrapTarget::SystemFighter { system, .. } => Some(system),
        ScrapTarget::TravellingSquadron { leg, .. } => match leg {
            crate::troop_transport::RegimentLeg::Surface(system) => Some(system),
            crate::troop_transport::RegimentLeg::Fleet(fleet) => fleet_system(fleet),
        },
        ScrapTarget::Defense(key) => listed(&|system| system.defense_facilities.contains(&key)),
        ScrapTarget::Manufacturing(key) => {
            listed(&|system| system.manufacturing_facilities.contains(&key))
        }
        ScrapTarget::Production(key) => {
            listed(&|system| system.production_facilities.contains(&key))
        }
    }
}

/// Whether Scrap is enabled for `order` (its `+0x18`, the object's `+0x5c`:
/// `FUN_004f9860`, and for a fleet `FUN_004ffa70`): something to scrap,
/// every object the side's and not en route. A fleet in hyperspace, and a
/// regiment travelling on its own, are en route. port: the port's objects
/// exist only once completed, and regiments and ships have no mission.
#[must_use]
pub fn order_enabled(
    world: &GameWorld,
    movement: &crate::movement::MovementState,
    transport: &TroopTransportState,
    order: ScrapOrder,
    side: Faction,
) -> bool {
    let targets = order_targets(world, order, side);
    let in_orbit = |fleet| !movement.is_in_transit(fleet);
    let travelling = |troop| {
        transport
            .transits()
            .iter()
            .any(|transit| transit.troop == troop)
    };
    !targets.is_empty()
        && targets.iter().all(|target| match *target {
            ScrapTarget::CapitalShip { fleet, .. } | ScrapTarget::Fighter { fleet, .. } => {
                in_orbit(fleet)
            }
            ScrapTarget::Troop(troop) => !travelling(troop),
            _ => true,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::DatId;
    use crate::world::{BuildableClass, ProductionFacilityInstance, System, TroopUnit};

    fn world() -> (GameWorld, crate::ids::SystemKey) {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(crate::world::Sector {
            dat_id: DatId::new(0x8000_0001),
            name: "Core".into(),
            group: crate::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let system = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0001),
            name: "Bortras".into(),
            sector,
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 1.0,
            popularity_empire: 0.0,
            is_populated: true,
            total_energy: 10,
            raw_materials: 10,
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
            control: crate::world::ControlKind::Controlled(Faction::Alliance),
        });
        world.buildable_classes.insert(
            DatId::new(0x1000_0001),
            BuildableClass {
                refined_material_cost: 9,
                maintenance_cost: 3,
                ..BuildableClass::default()
            },
        );
        world.buildable_classes.insert(
            DatId::new(0x2c00_0001),
            BuildableClass {
                refined_material_cost: 20,
                ..BuildableClass::default()
            },
        );
        (world, system)
    }

    // FUN_00530270: a scrapped object returns half its class's refined cost
    // (class +0x48, FUN_004f2980) to its side, rounded down.
    #[test]
    fn scrapping_returns_half_the_refined_cost_to_its_side() {
        let (mut world, system) = world();
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: true,
            regiment_strength: 100,
        });
        world.systems[system].ground_units.push(troop);
        let mut stockpiles = StockpileState::new();
        let mut transport = TroopTransportState::default();

        let scrapped = scrap(
            &mut world,
            &mut stockpiles,
            &mut transport,
            ScrapTarget::Troop(troop),
        );

        assert_eq!(
            scrapped,
            Some(Scrapped {
                side: Faction::Alliance,
                refund: 4
            })
        );
        assert_eq!(stockpiles.side(Faction::Alliance).refined, 4);
        assert_eq!(stockpiles.side(Faction::Empire).refined, 0);
        assert!(world.troops.get(troop).is_none());
        assert!(world.systems[system].ground_units.is_empty());
    }

    // FUN_0052fff0: the load is the sum over what exists, so the scrapped
    // object's maintenance returns with it.
    #[test]
    fn scrapping_returns_the_objects_maintenance() {
        let (mut world, system) = world();
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: true,
            regiment_strength: 100,
        });
        world.systems[system].ground_units.push(troop);
        let manufacturing = crate::manufacturing::ManufacturingState::new();
        let before = crate::resources::maintenance_used(&world, &manufacturing, Faction::Alliance);

        scrap(
            &mut world,
            &mut StockpileState::new(),
            &mut TroopTransportState::default(),
            ScrapTarget::Troop(troop),
        );

        let after = crate::resources::maintenance_used(&world, &manufacturing, Faction::Alliance);
        assert_eq!(before - after, 3);
    }

    // FUN_0052e530: a unit that costs no maintenance is never picked, so a
    // mine is not a candidate.
    #[test]
    fn the_overdraft_never_picks_what_costs_no_maintenance() {
        let (mut world, system) = world();
        let mine = world
            .production_facilities
            .insert(ProductionFacilityInstance {
                class_dat_id: DatId::new(0x2c00_0001),
                side: Faction::Alliance,
                is_mine: true,
            });
        world.systems[system].production_facilities.push(mine);
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: true,
            regiment_strength: 100,
        });
        // A regiment class with no upkeep is never picked either.
        world.buildable_classes.insert(
            DatId::new(0x1000_0002),
            BuildableClass {
                refined_material_cost: 4,
                ..BuildableClass::default()
            },
        );
        world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0002),
            is_alliance: true,
            regiment_strength: 100,
        });
        let enemy = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: false,
            regiment_strength: 100,
        });
        let candidates = overdraft_candidates(&world, Faction::Alliance);
        assert_eq!(candidates, [ScrapTarget::Troop(troop)]);
        assert!(!candidates.contains(&ScrapTarget::Troop(enemy)));
    }

    // FUN_00556430 puts a travelling squadron in its new container at once,
    // so the overdraft walk (FUN_0052e530) meets it, and its scrap names the
    // destination; the transit goes with it.
    #[test]
    fn a_travelling_squadron_is_an_overdraft_candidate_at_its_destination() {
        for side in [Faction::Alliance, Faction::Empire] {
            let (mut world, system) = world();
            let class = world.fighter_classes.insert(crate::world::FighterClass {
                name: "X-wing".into(),
                maintenance_cost: 2,
                refined_material_cost: 8,
                ..crate::world::FighterClass::default()
            });
            world
                .squadron_transits
                .push(crate::troop_transport::SquadronTransit {
                    class,
                    is_alliance: side == Faction::Alliance,
                    origin: system,
                    leg: crate::troop_transport::RegimentLeg::Surface(system),
                    arrival_tick: 30,
                });
            let mut transport = TroopTransportState::new();
            let mut stockpiles = StockpileState::new();

            let candidates = overdraft_candidates(&world, side);
            assert_eq!(candidates.len(), 1, "{side:?}");
            assert_eq!(
                target_system(&world, &transport, candidates[0]),
                Some(system)
            );
            assert!(scrap(&mut world, &mut stockpiles, &mut transport, candidates[0]).is_some());
            assert!(world.squadron_transits.is_empty(), "{side:?}");
        }
    }

    // FUN_0052e530 walks only the side's objects that cost maintenance, in
    // family order (FUN_0052e8b0); a lost hull is not walked.
    #[test]
    fn the_overdraft_lists_the_sides_costing_objects_in_family_order() {
        let (mut world, system) = world();
        let (costly, free) = (DatId::new(0x1000_0001), DatId::new(0x2c00_0001));
        let ship_class = |world: &mut GameWorld, maintenance_cost| {
            world
                .capital_ship_classes
                .insert(crate::world::CapitalShipClass {
                    maintenance_cost,
                    ..crate::world::CapitalShipClass::default()
                })
        };
        let (costly_ship, free_ship) = (ship_class(&mut world, 5), ship_class(&mut world, 0));
        let fighter_class = |world: &mut GameWorld, maintenance_cost| {
            world.fighter_classes.insert(crate::world::FighterClass {
                maintenance_cost,
                ..crate::world::FighterClass::default()
            })
        };
        let (costly_fighter, free_fighter) =
            (fighter_class(&mut world, 2), fighter_class(&mut world, 0));
        let ship = |class, alive| crate::world::ShipInstance {
            class,
            hull_current: 10,
            damage_counters: 0,
            alive,
            name: None,
            tag: 0,
            repair_timer: None,
        };
        let entry = |class, count| crate::world::FighterEntry {
            class,
            count,
            carrier: 0,
        };
        let mut expected = std::collections::HashMap::new();
        for is_alliance in [true, false] {
            let side = Faction::of_alliance(is_alliance);
            let troop_of = |class_dat_id| TroopUnit {
                class_dat_id,
                is_alliance,
                regiment_strength: 100,
            };
            let troop = world.troops.insert(troop_of(costly));
            world.troops.insert(troop_of(free));
            let fleet = world.fleets.insert(crate::world::Fleet {
                location: system,
                capital_ships: vec![
                    ship(costly_ship, true),
                    ship(costly_ship, false),
                    ship(free_ship, true),
                ],
                fighters: vec![entry(costly_fighter, 2), entry(free_fighter, 1)],
                characters: vec![],
                is_alliance,
                has_death_star: false,
            });
            // The sides hold different counts, so each walk shows whose it took.
            let held = if is_alliance { 1 } else { 2 };
            world.systems[system].add_squadrons(costly_fighter, held, is_alliance);
            world.systems[system].add_squadrons(free_fighter, 1, is_alliance);
            for class in [costly_fighter, free_fighter] {
                world
                    .squadron_transits
                    .push(crate::troop_transport::SquadronTransit {
                        class,
                        is_alliance,
                        origin: system,
                        leg: crate::troop_transport::RegimentLeg::Surface(system),
                        arrival_tick: 30,
                    });
            }
            let defense_of =
                |class_dat_id| crate::world::DefenseFacilityInstance { class_dat_id, side };
            let defense = world.defense_facilities.insert(defense_of(costly));
            world.defense_facilities.insert(defense_of(free));
            let yard_of = |class_dat_id| crate::world::ManufacturingFacilityInstance {
                class_dat_id,
                side,
                ..Default::default()
            };
            let yard = world.manufacturing_facilities.insert(yard_of(costly));
            world.manufacturing_facilities.insert(yard_of(free));
            let unit_of = |class_dat_id| crate::world::SpecialForceUnit {
                class_dat_id,
                is_alliance,
                ..Default::default()
            };
            let unit = world.special_forces.insert(unit_of(costly));
            world.special_forces.insert(unit_of(free));
            let mut targets = vec![
                ScrapTarget::Troop(troop),
                ScrapTarget::CapitalShip { fleet, index: 0 },
                ScrapTarget::Fighter { fleet, entry: 0 },
                ScrapTarget::Fighter { fleet, entry: 0 },
            ];
            for _ in 0..held {
                targets.push(ScrapTarget::SystemFighter {
                    system,
                    class: costly_fighter,
                    is_alliance,
                });
            }
            targets.extend([
                ScrapTarget::TravellingSquadron {
                    class: costly_fighter,
                    is_alliance,
                    leg: crate::troop_transport::RegimentLeg::Surface(system),
                    arrival_tick: 30,
                },
                ScrapTarget::Defense(defense),
                ScrapTarget::Manufacturing(yard),
                ScrapTarget::SpecialForce(unit),
            ]);
            expected.insert(is_alliance, targets);
        }
        for is_alliance in [true, false] {
            assert_eq!(
                overdraft_candidates(&world, Faction::of_alliance(is_alliance)),
                expected[&is_alliance],
                "{is_alliance}"
            );
        }
    }

    // FUN_00530270 destroys the one object named: a travelling squadron
    // that differs in class, side, leg or arrival day is another squadron.
    #[test]
    fn a_travelling_squadrons_scrap_takes_only_the_one_named() {
        let (mut world, system) = world();
        let other_system = world.systems.insert(System::default());
        let class = |world: &mut GameWorld| {
            world.fighter_classes.insert(crate::world::FighterClass {
                refined_material_cost: 8,
                ..crate::world::FighterClass::default()
            })
        };
        let (named, other) = (class(&mut world), class(&mut world));
        let transit =
            |class, is_alliance, leg, arrival_tick| crate::troop_transport::SquadronTransit {
                class,
                is_alliance,
                origin: system,
                leg: crate::troop_transport::RegimentLeg::Surface(leg),
                arrival_tick,
            };
        let near_misses = vec![
            transit(other, true, system, 30),
            transit(named, false, system, 30),
            transit(named, true, other_system, 30),
            transit(named, true, system, 31),
        ];
        world.squadron_transits = near_misses.clone();
        world
            .squadron_transits
            .push(transit(named, true, system, 30));
        let target = ScrapTarget::TravellingSquadron {
            class: named,
            is_alliance: true,
            leg: crate::troop_transport::RegimentLeg::Surface(system),
            arrival_tick: 30,
        };
        let mut stockpiles = StockpileState::new();
        let mut transport = TroopTransportState::new();

        assert!(scrap(&mut world, &mut stockpiles, &mut transport, target).is_some());
        assert_eq!(world.squadron_transits, near_misses);
        assert_eq!(
            scrap(&mut world, &mut stockpiles, &mut transport, target),
            None
        );
        assert_eq!(world.squadron_transits, near_misses);
    }

    // FUN_00530270 scraps only an object that exists: a squadron entry left
    // empty, or one of another class or side, is nothing to scrap.
    #[test]
    fn an_emptied_squadron_scraps_nothing() {
        let (mut world, system) = world();
        let class = |world: &mut GameWorld| {
            world.fighter_classes.insert(crate::world::FighterClass {
                refined_material_cost: 8,
                ..crate::world::FighterClass::default()
            })
        };
        let (named, other) = (class(&mut world), class(&mut world));
        let mut stockpiles = StockpileState::new();
        let mut transport = TroopTransportState::new();
        let squadrons = |class, count, is_alliance| crate::world::SystemSquadrons {
            class,
            count,
            is_alliance,
        };
        world.systems[system].fighters = vec![
            squadrons(other, 1, true),
            squadrons(named, 1, false),
            squadrons(named, 0, true),
        ];
        let target = ScrapTarget::SystemFighter {
            system,
            class: named,
            is_alliance: true,
        };
        assert_eq!(
            scrap(&mut world, &mut stockpiles, &mut transport, target),
            None
        );
        world.systems[system].fighters[2].count = 1;
        assert!(scrap(&mut world, &mut stockpiles, &mut transport, target).is_some());

        let fleet = fleet(&mut world, system, true);
        world.fleets[fleet].fighters[0].count = 0;
        let target = ScrapTarget::Fighter { fleet, entry: 0 };
        assert_eq!(
            scrap(&mut world, &mut stockpiles, &mut transport, target),
            None
        );
    }

    #[test]
    fn a_target_that_is_gone_scraps_nothing() {
        let (mut world, _) = world();
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: true,
            regiment_strength: 100,
        });
        world.troops.remove(troop);
        let mut stockpiles = StockpileState::new();
        assert_eq!(
            scrap(
                &mut world,
                &mut stockpiles,
                &mut TroopTransportState::default(),
                ScrapTarget::Troop(troop)
            ),
            None
        );
        assert_eq!(stockpiles.side(Faction::Alliance).refined, 0);
    }

    /// A fleet of the side at `system` with two named ships, one lost, and
    /// two squadrons of one class.
    fn fleet(world: &mut GameWorld, system: crate::ids::SystemKey, is_alliance: bool) -> FleetKey {
        let class = world
            .capital_ship_classes
            .insert(crate::world::CapitalShipClass {
                name: "Corellian Corvette".into(),
                refined_material_cost: 30,
                ..crate::world::CapitalShipClass::default()
            });
        let fighters = world.fighter_classes.insert(crate::world::FighterClass {
            name: "X-wing".into(),
            ..crate::world::FighterClass::default()
        });
        let ship = |alive| crate::world::ShipInstance {
            class,
            hull_current: 10,
            damage_counters: 0,
            alive,
            name: None,
            tag: 0,
            repair_timer: None,
        };
        let key = world.fleets.insert(crate::world::Fleet {
            location: system,
            capital_ships: vec![ship(true), ship(false), ship(true)],
            fighters: vec![crate::world::FighterEntry {
                class: fighters,
                count: 2,
                carrier: 0,
            }],
            characters: vec![],
            is_alliance,
            has_death_star: false,
        });
        world.systems[system].fleets.push(key);
        key
    }

    // FUN_004f84e0 on the fleet object. hyp: its living ships and each
    // squadron go with it.
    #[test]
    fn a_fleets_scrap_takes_its_living_ships_and_squadrons() {
        let (mut world, system) = world();
        let key = fleet(&mut world, system, true);
        assert_eq!(
            order_targets(&world, ScrapOrder::Fleet(key), Faction::Alliance),
            [
                ScrapTarget::Fighter {
                    fleet: key,
                    entry: 0
                },
                ScrapTarget::Fighter {
                    fleet: key,
                    entry: 0
                },
                ScrapTarget::CapitalShip {
                    fleet: key,
                    index: 0
                },
                ScrapTarget::CapitalShip {
                    fleet: key,
                    index: 2
                },
            ]
        );
    }

    // FUN_004f9860: only the order's side's objects; FUN_00512700 filters
    // an icon's team by side (FUN_00553350).
    #[test]
    fn an_icons_scrap_names_only_the_sides_objects() {
        let (mut world, system) = world();
        let own = fleet(&mut world, system, true);
        fleet(&mut world, system, false);
        let targets = order_targets(&world, ScrapOrder::Fleets(system), Faction::Alliance);
        assert_eq!(targets.len(), 4);
        assert!(targets.iter().all(|target| matches!(
            target,
            ScrapTarget::CapitalShip { fleet, .. } | ScrapTarget::Fighter { fleet, .. } if *fleet == own
        )));
        assert!(order_targets(&world, ScrapOrder::Fleet(own), Faction::Empire).is_empty());
    }

    // FUN_004ffa70: a fleet en route cannot be scrapped; nor can an order
    // with nothing to scrap.
    #[test]
    fn a_fleet_in_hyperspace_cannot_be_scrapped() {
        let (mut world, system) = world();
        let key = fleet(&mut world, system, true);
        let transport = TroopTransportState::default();
        let mut movement = crate::movement::MovementState::new();
        let order = ScrapOrder::Fleet(key);
        assert!(order_enabled(
            &world,
            &movement,
            &transport,
            order,
            Faction::Alliance
        ));
        assert!(!order_enabled(
            &world,
            &movement,
            &transport,
            order,
            Faction::Empire
        ));
        movement.order(key, system, system, 5);
        assert!(!order_enabled(
            &world,
            &movement,
            &transport,
            order,
            Faction::Alliance
        ));
    }

    // FUN_0049a880 adds one line per object of the order's team: a fleet's
    // team is the fleet, so its name stands for its members.
    #[test]
    fn the_confirmation_names_the_teams_objects() {
        let (mut world, system) = world();
        let key = fleet(&mut world, system, true);
        let names = order_names(&world, ScrapOrder::Fleet(key), Faction::Alliance);
        assert_eq!(names, [world.fleet_name(key).unwrap().to_string()]);
        let names = order_names(
            &world,
            ScrapOrder::Ship {
                fleet: key,
                index: 2,
            },
            Faction::Alliance,
        );
        assert_eq!(names, ["Corellian Corvette"]);
    }

    // FUN_00530270: a scrapped ship refunds half its class's cost too, and
    // a squadron leaves its fleet one at a time.
    #[test]
    fn a_ship_and_a_squadron_scrap_one_at_a_time() {
        let (mut world, system) = world();
        let key = fleet(&mut world, system, true);
        let mut stockpiles = StockpileState::new();
        let mut transport = TroopTransportState::default();
        let ship = scrap(
            &mut world,
            &mut stockpiles,
            &mut transport,
            ScrapTarget::CapitalShip {
                fleet: key,
                index: 0,
            },
        );
        assert_eq!(ship.map(|scrapped| scrapped.refund), Some(15));
        assert!(!world.fleets[key].capital_ships[0].alive);
        scrap(
            &mut world,
            &mut stockpiles,
            &mut transport,
            ScrapTarget::Fighter {
                fleet: key,
                entry: 0,
            },
        );
        assert_eq!(world.fleets[key].fighters[0].count, 1);
    }

    // FUN_00530270 destroys the fleet object with its members. hyp: a
    // character aboard stays in the system.
    #[test]
    fn scrapping_a_fleets_last_unit_removes_the_fleet() {
        let (mut world, system) = world();
        let key = fleet(&mut world, system, true);
        let character = world.characters.insert(crate::world::Character {
            current_fleet: Some(key),
            ..crate::world::Character::default()
        });
        world.fleets[key].characters.push(character);
        let mut stockpiles = StockpileState::new();
        let mut transport = TroopTransportState::default();
        let targets = order_targets(&world, ScrapOrder::Fleet(key), Faction::Alliance);
        let (last, rest) = targets.split_last().unwrap();
        for &target in rest {
            scrap(&mut world, &mut stockpiles, &mut transport, target);
        }
        assert!(world.fleets.contains_key(key));
        scrap(&mut world, &mut stockpiles, &mut transport, *last);
        assert!(!world.fleets.contains_key(key));
        assert!(!world.systems[system].fleets.contains(&key));
        assert_eq!(world.characters[character].current_fleet, None);
        assert_eq!(world.characters[character].current_system, Some(system));
    }
}
