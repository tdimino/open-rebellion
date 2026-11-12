//! Shared, feature-gated fixtures for simulation and downstream crate tests.
//!
//! Keep defaults here intentionally small and unsurprising. Tests should
//! override only the fields whose behavior they observe.

use crate::dat::{ExplorationStatus, SectorGroup};
use crate::ids::{CapitalShipKey, DatId, FighterKey, FleetKey, SectorKey, SystemKey, TroopKey};
use crate::manufacturing::{BuildableKind, ManufacturingState, QueueItem};
use crate::movement::MovementState;
use crate::world::{
    CapitalShipClass, ControlKind, FighterClass, Fleet, GameWorld, GnprtbEntry, MissionMemberRules,
    MissionRecord, MissionTargetRules, SdprtbEntry, Sector, ShipInstance, System, TroopUnit,
};

/// One GNPRTB value repeated across every difficulty and game mode.
pub fn gnprtb_entry(id: impl TryInto<u32>, value: i32) -> GnprtbEntry {
    let Ok(parameter_id) = id.try_into() else {
        panic!("fixture parameter ID must fit u32");
    };
    GnprtbEntry {
        parameter_id,
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

/// One SDPRTB value repeated for both sides in every game mode.
pub fn sdprtb_entry(id: impl TryInto<u32>, value: i32) -> SdprtbEntry {
    sdprtb_sides(id, value, value)
}

/// One SDPRTB pair repeated across every game mode.
pub fn sdprtb_sides(id: impl TryInto<u32>, alliance: i32, empire: i32) -> SdprtbEntry {
    let Ok(parameter_id) = id.try_into() else {
        panic!("fixture parameter ID must fit u32");
    };
    SdprtbEntry {
        parameter_id,
        dev_alliance: alliance,
        dev_empire: empire,
        alliance_sp_easy_alliance: alliance,
        alliance_sp_easy_empire: empire,
        alliance_sp_medium_alliance: alliance,
        alliance_sp_medium_empire: empire,
        alliance_sp_hard_alliance: alliance,
        alliance_sp_hard_empire: empire,
        empire_sp_easy_alliance: alliance,
        empire_sp_easy_empire: empire,
        empire_sp_medium_alliance: alliance,
        empire_sp_medium_empire: empire,
        empire_sp_hard_alliance: alliance,
        empire_sp_hard_empire: empire,
        multiplayer_alliance: alliance,
        multiplayer_empire: empire,
    }
}

/// The shipped Diplomacy mission record used by lifecycle and interface tests.
///
/// port: test fixture values, not simulation rules.
#[must_use]
pub fn diplomacy_mission_record() -> MissionRecord {
    MissionRecord {
        dat_id: DatId::new(0x5100_0010),
        timer_min_days: 5,
        timer_spread_days: 10,
        repeats: true,
        hidden: false,
        detection_phases: true,
        can_resign: true,
        return_policy: 1,
        rules: MissionTargetRules {
            container_loss_ends: true,
            needs_populated_container: true,
            target_loss_ends: true,
            own_side_target: true,
            other_side_target: true,
            opponent_target: false,
            revolting_target: false,
            calm_target: true,
            prisoner_target: false,
            free_target: false,
        },
        members: MissionMemberRules {
            alliance: true,
            empire: true,
            special_force_mask: 0,
            character_mask: 0x1_0000,
        },
    }
}

/// A populated, explored system with neutral 50/50 popularity and no assets.
///
/// port: test fixture values, not simulation rules.
pub fn system(name: impl Into<String>, control: ControlKind) -> System {
    System {
        name: name.into(),
        exploration_status: ExplorationStatus::Explored,
        popularity_alliance: 0.5,
        popularity_empire: 0.5,
        is_populated: true,
        control,
        ..System::default()
    }
}

/// A deterministic Core-sector record with no systems yet attached.
///
/// port: test fixture values, not simulation rules.
pub fn sector(name: impl Into<String>) -> Sector {
    Sector {
        dat_id: DatId::new(0x8000_0001),
        name: name.into(),
        group: SectorGroup::Core,
        ..Sector::default()
    }
}

/// [`system`] placed in `sector` at the supplied galaxy coordinates.
pub fn system_at(
    name: impl Into<String>,
    sector: SectorKey,
    x: u16,
    y: u16,
    control: ControlKind,
) -> System {
    System {
        sector,
        x,
        y,
        ..system(name, control)
    }
}

/// Insert one system and attach it to the supplied sector fixture.
pub fn insert_system_in_sector(
    world: &mut GameWorld,
    sector: Sector,
    mut system: System,
) -> (SectorKey, SystemKey) {
    let sector = world.sectors.insert(sector);
    system.sector = sector;
    let system = world.systems.insert(system);
    world.sectors[sector].systems.push(system);
    (sector, system)
}

/// Insert one system and the deterministic Core sector that owns it.
pub fn insert_system(world: &mut GameWorld, system: System) -> SystemKey {
    insert_system_in_sector(world, sector("Core"), system).1
}

/// Put one system into a deterministic Core-sector world.
///
/// This is the smallest useful fixture for tests that exercise one system but
/// still need its sector relation to be valid.
#[must_use]
pub fn world_with_system(system: System) -> (GameWorld, SystemKey) {
    let mut world = GameWorld::default();
    let system = insert_system(&mut world, system);
    (world, system)
}

/// Put systems at the requested coordinates in one deterministic test sector.
///
/// port: test fixture values, not simulation rules.
#[must_use]
pub fn world_with_systems(positions: &[(u16, u16)]) -> (GameWorld, Vec<SystemKey>) {
    let mut world = GameWorld::default();
    let sector = world.sectors.insert(Sector {
        dat_id: DatId::new(0x9200_0000),
        name: "Test".into(),
        group: SectorGroup::Core,
        ..Sector::default()
    });
    let systems = positions
        .iter()
        .map(|&(x, y)| {
            let mut system = system_at(
                format!("Sys@{x},{y}"),
                sector,
                x,
                y,
                ControlKind::Uncontrolled,
            );
            system.dat_id = DatId::new(0x9000_0000);
            let system = world.systems.insert(system);
            world.sectors[sector].systems.push(system);
            system
        })
        .collect();
    (world, systems)
}

/// Insert a one-ship fleet using `class` and attach it to `system`.
///
/// port: test fixture values, not simulation rules.
pub fn add_fleet_with_class(
    world: &mut GameWorld,
    system: SystemKey,
    class: CapitalShipKey,
    is_alliance: bool,
) -> FleetKey {
    add_fleet_with_ship(
        world,
        system,
        ShipInstance::new(class, 100, is_alliance),
        is_alliance,
    )
}

/// Insert a one-ship fleet and attach it to `system`.
pub fn add_fleet_with_ship(
    world: &mut GameWorld,
    system: SystemKey,
    ship: ShipInstance,
    is_alliance: bool,
) -> FleetKey {
    add_fleet_with_ships(world, system, vec![ship], is_alliance)
}

/// Insert a fleet containing `ships` and attach it to `system`.
pub fn add_fleet_with_ships(
    world: &mut GameWorld,
    system: SystemKey,
    ships: Vec<ShipInstance>,
    is_alliance: bool,
) -> FleetKey {
    let fleet = world.insert_fleet(Fleet {
        location: system,
        capital_ships: ships,
        is_alliance,
        ..Fleet::default()
    });
    world.systems[system].fleets.push(fleet);
    fleet
}

/// Insert a usable one-ship fleet for the requested side.
///
/// port: test fixture values, not simulation rules.
pub fn add_usable_fleet(world: &mut GameWorld, system: SystemKey, is_alliance: bool) -> FleetKey {
    let class = world.capital_ship_classes.insert(CapitalShipClass {
        is_alliance,
        is_empire: !is_alliance,
        hull: 100,
        ..CapitalShipClass::default()
    });
    add_fleet_with_class(world, system, class, is_alliance)
}

/// Insert a completed Death Star ship into an existing fleet.
///
/// `class` carries scenario-specific values such as hull and bombardment;
/// this fixture supplies the canonical Death Star identity and family.
///
/// port: test fixture values, not simulation rules.
pub fn add_death_star_ship(
    world: &mut GameWorld,
    fleet: FleetKey,
    mut class: CapitalShipClass,
) -> CapitalShipKey {
    class.dat_id = DatId::new(crate::world::DEATH_STAR_CLASS_ID);
    class.family = 0x18;
    let hull = i32::try_from(class.hull).expect("fixture Death Star hull must fit i32");
    let is_alliance = world.fleets[fleet].is_alliance;
    let class = world.capital_ship_classes.insert(class);
    world.fleets[fleet]
        .capital_ships
        .push(ShipInstance::new(class, hull, is_alliance));
    class
}

/// Insert a completed Death Star fleet for the requested side.
///
/// port: test fixture values, not simulation rules.
pub fn add_death_star_fleet(
    world: &mut GameWorld,
    system: SystemKey,
    is_alliance: bool,
) -> FleetKey {
    let fleet = add_fleet_with_ships(world, system, Vec::new(), is_alliance);
    add_death_star_ship(
        world,
        fleet,
        CapitalShipClass {
            is_alliance,
            is_empire: !is_alliance,
            hull: 100,
            ..CapitalShipClass::default()
        },
    );
    fleet
}

/// Insert the canonical Cruiser and Wing classes used by battle-report tests.
///
/// port: test fixture values, not simulation rules.
pub fn add_cruiser_and_wing(
    world: &mut GameWorld,
    troop_capacity: u32,
    squadron_size: u32,
) -> (CapitalShipKey, FighterKey) {
    let cruiser = world.capital_ship_classes.insert(CapitalShipClass {
        dat_id: DatId::new(0x1400_0040),
        name: "Cruiser".into(),
        troop_capacity,
        ..CapitalShipClass::default()
    });
    let wing = world.fighter_classes.insert(FighterClass {
        dat_id: DatId::new(0x1c00_0001),
        name: "Wing".into(),
        squadron_size,
        ..FighterClass::default()
    });
    (cruiser, wing)
}

/// Insert a named capital-ship class and queue `count` units at `system`.
///
/// port: test fixture values, not simulation rules.
pub fn queue_capital_ship_build(
    world: &mut GameWorld,
    manufacturing: &mut ManufacturingState,
    system: SystemKey,
    name: &str,
    is_alliance: bool,
    is_empire: bool,
    count: u32,
) -> CapitalShipKey {
    let class = world.capital_ship_classes.insert(CapitalShipClass {
        dat_id: DatId::new(0x1400_0040),
        name: name.into(),
        is_alliance,
        is_empire,
        ..CapitalShipClass::default()
    });
    manufacturing.build(
        system,
        &QueueItem::new(BuildableKind::CapitalShip(class), 10),
        count,
    );
    class
}

/// Insert one default regiment for `is_alliance` and attach it to `system`.
pub fn add_ground_troop(world: &mut GameWorld, system: SystemKey, is_alliance: bool) -> TroopKey {
    let troop = world.troops.insert(TroopUnit {
        is_alliance,
        ..TroopUnit::default()
    });
    world.systems[system].ground_units.push(troop);
    troop
}

/// Assert that a completed movement replaced the fleet's old orbit with its destination.
pub fn assert_fleet_arrived(
    world: &GameWorld,
    movement: &MovementState,
    fleet: FleetKey,
    origin: SystemKey,
    destination: SystemKey,
) {
    assert!(!movement.is_in_transit(fleet));
    assert_eq!(world.fleets[fleet].location, destination);
    assert!(world.systems[destination].fleets.contains(&fleet));
    assert!(!world.systems[origin].fleets.contains(&fleet));
}
