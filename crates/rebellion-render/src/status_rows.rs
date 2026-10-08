//! The Status window's contents for each object family: the title, the
//! picture, the name line and the rows its filler writes (`FUN_00443130`
//! picks the filler by the id's family). Recovery notes:
//! `ghidra/notes/status-window.md`.
//!
//! Every "current:maximum" value joins the two with the executable's ":"
//! (`DAT_006a872c`). Where the port keeps no damage or losses, the current
//! value is the maximum.

use rebellion_core::dat::Faction;
use rebellion_core::ids::{
    CharacterKey, DatId, DefenseFacilityKey, FleetKey, ManufacturingFacilityKey,
    ProductionFacilityKey, SpecialForceKey, SystemKey, TroopKey,
};
use rebellion_core::manufacturing::{ManufacturingState, ProductionArea};
use rebellion_core::missions::{MissionFaction, MissionMember, MissionState, MissionTarget};
use rebellion_core::movement::MovementState;
use rebellion_core::troop_transport::{RegimentLeg, TroopTransportState};
use rebellion_core::world::{CapitalShipClass, GameWorld, ShipInstance};

use crate::bmp_cache::DllSource;
use crate::mission_dialog::{kind_icon, kind_name};
use crate::status_window::{character_rows, StatusRow};
use crate::panels::fleets::{capital_ship_mini_id, fighter_mini_id};
use crate::system_window::{
    character_mini_resource_id, defense_facility_mini, fleet_label, manufacturing_facility_mini,
    production_facility_mini, special_force_mini, troop_mini,
};

/// The state the fillers read.
#[derive(Clone, Copy)]
pub struct StatusSources<'a> {
    pub world: &'a GameWorld,
    pub missions: &'a MissionState,
    pub movement: &'a MovementState,
    pub transport: &'a TroopTransportState,
    pub manufacturing: &'a ManufacturingState,
    /// Today's day number (`FUN_004fd340`).
    pub today: u64,
}

/// The object a Status window shows, by the id families `FUN_00443130`
/// dispatches on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusObject {
    /// `0x30..0x3b`, `FUN_004486f0`.
    Character(CharacterKey),
    /// `0x3c..0x3f`, `FUN_00445780`.
    SpecialForce(SpecialForceKey),
    /// `0x08..0x0f`, `FUN_00449200`.
    Fleet(FleetKey),
    /// `0x14..0x1b`, `FUN_00446fd0`: a fleet's ship by its index.
    Ship { fleet: FleetKey, index: usize },
    /// `0x1c..0x1f`, `FUN_004444c0`: a fleet's squadron entry by its index.
    /// port: squadrons live on their fleet, not as objects.
    Fighter { fleet: FleetKey, index: usize },
    /// `0x10..0x13`, `FUN_00445280`.
    Troop(TroopKey),
    /// `0x20..0x27`, `FUN_00444e20`.
    DefenseFacility(DefenseFacilityKey),
    /// `0x28..0x2a`, `FUN_00446c50`.
    ManufacturingFacility(ManufacturingFacilityKey),
    /// `0x2c..0x2d`, `FUN_00446c50`.
    ProductionFacility(ProductionFacilityKey),
    /// `0x50..0x7f`, `FUN_00445c60`: an active mission by its id.
    Mission(u64),
    /// `0xa0..0xaf`, `FUN_0044acd0`: a system's manager for one area.
    Producer {
        system: SystemKey,
        area: ProductionArea,
    },
}

/// What the window draws for one object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusView {
    pub title: &'static str,
    /// The production manager's title is re-styled to font `0x11`
    /// (`FUN_00403e90(title, 0x11)`).
    pub manager_title: bool,
    /// The background row (`FUN_00443130`): 0 side 1, 1 side 2, 2 other.
    pub side: usize,
    pub picture: Option<(DllSource, u32)>,
    /// `FUN_0044a210`: the object's name.
    pub name: String,
    pub rows: Vec<StatusRow>,
}

fn row(label: &str, value: impl Into<String>) -> StatusRow {
    StatusRow {
        label: label.to_owned(),
        value: value.into(),
    }
}

/// A heading row: the label over an empty value (`0x006b120c`).
fn heading(label: &str) -> StatusRow {
    row(label, "")
}

/// "current:maximum", joined by the executable's ":" (`0x006a872c`).
fn ratio(current: impl std::fmt::Display, maximum: impl std::fmt::Display) -> String {
    format!("{current}:{maximum}")
}

/// TEXTSTRA 34646 "Yes" or 34645 "No".
const fn yes_no(value: bool) -> &'static str {
    if value {
        "Yes"
    } else {
        "No"
    }
}

/// 34658 "Not Assigned". port: the port keeps no character command rank
/// (`+0x96`), so a fleet's or system's Admiral, General and Commander rows
/// always read it.
const NOT_ASSIGNED: &str = "Not Assigned";

/// The shared ETA row (`FUN_004fd2b0`): 34662 "ETA Destination:", 14356
/// "Day " and the arrival day.
fn eta(rows: &mut Vec<StatusRow>, arrival: Option<u64>) {
    if let Some(day) = arrival {
        rows.push(row("ETA Destination:", format!("Day {day}")));
    }
}

/// A side's background row.
const fn side_index(is_alliance: bool) -> usize {
    if is_alliance {
        0
    } else {
        1
    }
}

/// A side's background row; a neutral one is "other".
const fn faction_index(side: Faction) -> usize {
    match side {
        Faction::Alliance => 0,
        Faction::Empire => 1,
        Faction::Neutral => 2,
    }
}

/// The system's holder as a background row; neutral and contested are
/// "other".
fn holder_index(world: &GameWorld, system: SystemKey) -> usize {
    world
        .systems
        .get(system)
        .and_then(|value| value.control.faction())
        .map_or(2, faction_index)
}

fn system_name(world: &GameWorld, system: SystemKey) -> String {
    world
        .systems
        .get(system)
        .map(|value| value.name.clone())
        .unwrap_or_default()
}

/// A class record's catalog name (TEXTSTRA).
fn class_name(world: &GameWorld, class: DatId) -> String {
    world
        .buildable_classes
        .get(&class)
        .map(|value| value.name.clone())
        .unwrap_or_default()
}

/// `FUN_0044a620`: class `+0x4c` (`FUN_004f2990`).
fn maintenance(world: &GameWorld, class: DatId) -> String {
    world
        .buildable_classes
        .get(&class)
        .map_or(0, |value| value.maintenance_cost)
        .to_string()
}

/// A GOKRES picture (`FUN_0042c3b0(.., 1, 1)`): the class record's `+0x30`
/// resource `& 0xfff`, the list mini less `0x4000`. port: the regiment
/// mark (STRATEGY 11514) and the status marks are not drawn.
fn picture(mini: Option<u32>) -> Option<(DllSource, u32)> {
    mini.map(|mini| (DllSource::Gokres, mini - 0x4000))
}

/// A fleet's arrival day: today plus the transit left (`move-order.md`).
fn fleet_arrival(sources: StatusSources<'_>, fleet: FleetKey) -> Option<u64> {
    sources.movement.get(fleet).map(|order| {
        sources.today + u64::from(order.transit_ticks.saturating_sub(order.ticks_elapsed))
    })
}

/// The fleet's own Status word: 34628 "Enroute" or 34632 "Awaiting
/// Orders". port: a fleet or ship exists only once complete, so "Under
/// Construction" never shows.
const fn moving_status(en_route: bool) -> &'static str {
    if en_route {
        "Enroute"
    } else {
        "Awaiting Orders"
    }
}

fn alive_ships(
    world: &GameWorld,
    fleet: FleetKey,
) -> impl Iterator<Item = (&ShipInstance, &CapitalShipClass)> + '_ {
    world
        .fleets
        .get(fleet)
        .into_iter()
        .flat_map(|value| value.capital_ships.iter())
        .filter(|ship| ship.alive)
        .filter_map(|ship| Some((ship, world.capital_ship_classes.get(ship.class)?)))
}

/// hyp: the port's nearest field to the damage bit (`+0x50` bit `0x200`):
/// a hull below its class's.
fn damaged(ship: &ShipInstance, class: &CapitalShipClass) -> bool {
    u32::try_from(ship.hull_current).unwrap_or(0) < class.hull
}

/// The Fleet Status rows (`FUN_00449200`).
///
/// port: the fleet's squadrons, regiments and characters are kept on the
/// fleet, so Embarked and Personnel sum them there. Personnel counts the
/// fleet's characters; the port carries no special force aboard a fleet.
/// hyp: Hyperdrive Rating reads Yes when an alive ship's class has a
/// hyperdrive (`FUN_004fd900` is traced only to its walk).
#[must_use]
pub fn fleet_rows(sources: StatusSources<'_>, key: FleetKey) -> Option<Vec<StatusRow>> {
    let world = sources.world;
    let fleet = world.fleets.get(key)?;
    let arrival = fleet_arrival(sources, key);
    let mut rows = vec![row("Status:", moving_status(arrival.is_some()))];
    eta(&mut rows, arrival);
    let ships: Vec<_> = alive_ships(world, key).collect();
    let fighter_capacity: u32 = ships.iter().map(|(_, class)| class.fighter_capacity).sum();
    let troop_capacity: u32 = ships.iter().map(|(_, class)| class.troop_capacity).sum();
    let squadrons: u32 = fleet.fighters.iter().map(|entry| entry.count).sum();
    rows.extend([
        row("Admiral:", NOT_ASSIGNED),
        row("General:", NOT_ASSIGNED),
        row("Commander:", NOT_ASSIGNED),
        row("Number Of Ships", fleet.ship_count().to_string()),
        heading("Capacity:"),
        row("Fighter Squadrons:", fighter_capacity.to_string()),
        row("Trooper Regiments:", troop_capacity.to_string()),
        heading("Embarked:"),
        row("Fighter Squadrons:", squadrons.to_string()),
        row(
            "Trooper Regiments:",
            sources.transport.carried_count(key).to_string(),
        ),
        row("Personnel:", fleet.characters.len().to_string()),
        row(
            "Damaged Ships:",
            ships
                .iter()
                .filter(|(ship, class)| damaged(ship, class))
                .count()
                .to_string(),
        ),
        row(
            "Hyperdrive Rating:",
            yes_no(ships.iter().any(|(_, class)| class.hyperdrive != 0)),
        ),
    ]);
    Some(rows)
}

/// What one ship carries of its fleet's `total`: the ships fill in order up
/// to each one's capacity. port: the port keeps squadrons and regiments on
/// the fleet, not on a ship (`+0x23c`, `+0x240`).
fn aboard(capacities: &[u32], index: usize, total: u32) -> u32 {
    let before: u32 = capacities[..index].iter().sum();
    total
        .saturating_sub(before)
        .min(capacities.get(index).copied().unwrap_or(0))
}

/// The Capital Ship Status rows (`FUN_00446fd0`), for the fleet's ship at
/// `index`.
///
/// port: the ship's damage nibbles (`+0x64` bits 8..23) and
/// `DAT_006bb388` are not kept, so the tractor beam, sub-light,
/// recharge and hyperdrive rows read the class value over itself. The
/// hull is `ShipInstance::hull_current`. hyp: Ship Damaged reads Yes while
/// the hull is below its class's (`+0x50` bit `0x200`). Personnel puts the
/// fleet's characters aboard its first alive ship.
#[must_use]
pub fn ship_rows(
    sources: StatusSources<'_>,
    fleet_key: FleetKey,
    index: usize,
) -> Option<Vec<StatusRow>> {
    let world = sources.world;
    let fleet = world.fleets.get(fleet_key)?;
    let ship = fleet.capital_ships.get(index).filter(|ship| ship.alive)?;
    let class = world.capital_ship_classes.get(ship.class)?;
    let alive: Vec<usize> = (0..fleet.capital_ships.len())
        .filter(|&i| fleet.capital_ships[i].alive)
        .collect();
    let position = alive.iter().position(|&i| i == index)?;
    let capacities = |field: fn(&CapitalShipClass) -> u32| -> Vec<u32> {
        alive
            .iter()
            .map(|&i| {
                world
                    .capital_ship_classes
                    .get(fleet.capital_ships[i].class)
                    .map_or(0, field)
            })
            .collect()
    };
    let squadrons: u32 = fleet.fighters.iter().map(|entry| entry.count).sum();
    let regiments = u32::try_from(sources.transport.carried_count(fleet_key)).unwrap_or(u32::MAX);
    let personnel = if position == 0 {
        fleet.characters.len()
    } else {
        0
    };
    let arrival = fleet_arrival(sources, fleet_key);
    let mut rows = vec![
        row("Class:", class.name.clone()),
        row("Fleet:", fleet_label(world, fleet_key).unwrap_or_default()),
        row("Status:", moving_status(arrival.is_some())),
    ];
    eta(&mut rows, arrival);
    let hyperdrive = if class.hyperdrive == 0 {
        class.hyperdrive_if_damaged
    } else {
        class.hyperdrive
    };
    rows.extend([
        row("Maintenance Cost:", class.maintenance_cost.to_string()),
        heading("Capacity:"),
        row("Fighter Squadrons:", class.fighter_capacity.to_string()),
        row("Trooper Regiments:", class.troop_capacity.to_string()),
        heading("Embarked:"),
        row(
            "Fighter Squadrons:",
            aboard(&capacities(|c| c.fighter_capacity), position, squadrons).to_string(),
        ),
        row(
            "Trooper Regiments:",
            aboard(&capacities(|c| c.troop_capacity), position, regiments).to_string(),
        ),
        row("Personnel:", personnel.to_string()),
        row("Ship Damaged:", yes_no(damaged(ship, class))),
        row("Hyperdrive Rating:", ratio(hyperdrive, class.hyperdrive)),
        row("Hull Value:", ratio(ship.hull_current, class.hull)),
        row("Damage Control Rating:", class.damage_control.to_string()),
        row(
            "Shield Recharge Rate:",
            ratio(class.shield_recharge_rate, class.shield_recharge_rate),
        ),
        row("Maximum Shield Strength:", class.shield_strength.to_string()),
        row(
            "Tractor Beam Power:",
            ratio(class.tractor_beam_power, class.tractor_beam_power),
        ),
        row(
            "Sub-Light Engine Rating:",
            ratio(class.sub_light_engine, class.sub_light_engine),
        ),
        row("Maneuverability:", class.maneuverability.to_string()),
        row("Detection Rating:", class.detection.to_string()),
        row(
            "Weapon Recharge Rate:",
            ratio(class.weapon_recharge_rate, class.weapon_recharge_rate),
        ),
        row("Bombardment Modifier:", class.bombardment_modifier.to_string()),
    ]);
    // Fore 0, aft 1, then starboard (arc 3) before port (arc 2), read from
    // the pushes at 0x447f2c..0x448650.
    for (heading_label, [turbo, ion, laser]) in [
        (
            "Forward Weapons Arc Rating:",
            [class.turbolaser_fore, class.ion_cannon_fore, class.laser_cannon_fore],
        ),
        (
            "Aft Weapons Arc Rating:",
            [class.turbolaser_aft, class.ion_cannon_aft, class.laser_cannon_aft],
        ),
        (
            "Starboard Weapons Arc Rating:",
            [
                class.turbolaser_starboard,
                class.ion_cannon_starboard,
                class.laser_cannon_starboard,
            ],
        ),
        (
            "Port Weapons Arc Rating:",
            [class.turbolaser_port, class.ion_cannon_port, class.laser_cannon_port],
        ),
    ] {
        rows.extend([
            heading(heading_label),
            row("Turbo Laser:", turbo.to_string()),
            row(" Ion Cannon:", ion.to_string()),
            row("Laser Cannon:", laser.to_string()),
        ]);
    }
    Some(rows)
}

/// The Fighter Squadron Status rows (`FUN_004444c0`), for the fleet's
/// squadron entry at `index`. It has no "Status:" row.
///
/// port: the port counts whole squadrons and keeps no losses (`+0x60`), so
/// each "current" is its maximum.
#[must_use]
pub fn fighter_rows(
    sources: StatusSources<'_>,
    fleet_key: FleetKey,
    index: usize,
) -> Option<Vec<StatusRow>> {
    let world = sources.world;
    let fleet = world.fleets.get(fleet_key)?;
    let entry = fleet.fighters.get(index)?;
    let class = world.fighter_classes.get(entry.class)?;
    let size = class.squadron_size;
    let full = |per_craft: u32| ratio(per_craft * size, per_craft * size);
    let mut rows = vec![row(
        "Attached: ",
        fleet_label(world, fleet_key).unwrap_or_default(),
    )];
    eta(&mut rows, fleet_arrival(sources, fleet_key));
    rows.extend([
        row("Maintenance Cost:", class.maintenance_cost.to_string()),
        row("Squadron Size:", ratio(size, size)),
        row("Hyperdrive Rating:", class.hyperdrive.to_string()),
        row("Maximum Shield Strength:", full(class.shield_strength)),
        row("Sub-Light Engine Rating:", class.sub_light_engine.to_string()),
        row("Maneuverability:", class.maneuverability.to_string()),
        row("Detection Rating:", class.detection.to_string()),
        row(
            "Bombardment Value:",
            ratio(class.bombardment_defense, class.bombardment_defense),
        ),
        heading("Weapons Rating:"),
        row(" Laser Rating:", full(class.laser_cannon_attack_strength)),
        row(" Ion Cannon:", full(class.ion_cannon_attack_strength)),
        row(" Torpedoes:", full(class.torpedoes)),
    ]);
    Some(rows)
}

/// Where a regiment is, and its arrival day while it travels.
fn regiment_place(sources: StatusSources<'_>, key: TroopKey) -> (String, Option<u64>) {
    let world = sources.world;
    if let Some(transit) = sources
        .transport
        .transits()
        .iter()
        .find(|transit| transit.troop == key)
    {
        // An object in hyperspace is attached to its destination
        // (FUN_00556390).
        let place = match transit.leg {
            RegimentLeg::Surface(system) => system_name(world, system),
            RegimentLeg::Fleet(fleet) => fleet_label(world, fleet).unwrap_or_default(),
        };
        return (place, Some(transit.arrival_tick));
    }
    if let Some(fleet) = sources
        .transport
        .fleet_keys()
        .into_iter()
        .find(|&fleet| sources.transport.cargo(fleet).contains(&key))
    {
        return (fleet_label(world, fleet).unwrap_or_default(), None);
    }
    let system = world
        .systems
        .iter()
        .find(|(_, value)| value.ground_units.contains(&key))
        .map(|(_, value)| value.name.clone())
        .unwrap_or_default();
    (system, None)
}

/// The Trooper Regiment Status rows (`FUN_00445280`).
///
/// port: a regiment exists only once trained, so "Training" never shows.
#[must_use]
pub fn troop_rows(sources: StatusSources<'_>, key: TroopKey) -> Option<Vec<StatusRow>> {
    let world = sources.world;
    let troop = world.troops.get(key)?;
    let class = world.troop_classes.get(&troop.class_dat_id);
    let stats = world
        .buildable_classes
        .get(&troop.class_dat_id)
        .map(|value| value.stats)
        .unwrap_or_default();
    let (attached, arrival) = regiment_place(sources, key);
    let mut rows = vec![
        row("Attached: ", attached),
        row("Status:", moving_status(arrival.is_some())),
    ];
    eta(&mut rows, arrival);
    rows.extend([
        row("Maintenance Cost:", maintenance(world, troop.class_dat_id)),
        row(
            "Attack Strength:",
            class.map_or(0, |value| value.attack_strength).to_string(),
        ),
        row(
            "Defense Strength:",
            class.map_or(0, |value| value.defense_strength).to_string(),
        ),
        row("Bombardment Value:", stats.bombardment.to_string()),
        row(
            "Detection Value:",
            class.map_or(0, |value| value.detection).to_string(),
        ),
    ]);
    Some(rows)
}

/// The Spec Forces Status rows (`FUN_00445780`). The ratings are slots
/// `+0x1dc`, `+0x1e0`, `+0x1f0` and `+0x1f4`: skills 0, 1, 5 and 6.
///
/// port: a special force exists only once built, so "Under Construction"
/// never shows; it is en route while its mission transit lasts, as a
/// character is.
#[must_use]
pub fn special_force_rows(
    sources: StatusSources<'_>,
    key: SpecialForceKey,
) -> Option<Vec<StatusRow>> {
    let world = sources.world;
    let unit = world.special_forces.get(key)?;
    let transit = sources
        .missions
        .en_route()
        .iter()
        .find(|transit| transit.member == MissionMember::SpecialForce(key));
    let attached = transit.map_or_else(
        || {
            world
                .systems
                .iter()
                .find(|(_, value)| value.special_forces.contains(&key))
                .map(|(_, value)| value.name.clone())
                .unwrap_or_default()
        },
        |transit| system_name(world, transit.to),
    );
    let status = if transit.is_some() {
        "Enroute"
    } else if unit.on_mission {
        "On Mission"
    } else {
        "Awaiting Orders"
    };
    let mut rows = vec![row("Attached: ", attached), row("Status:", status)];
    eta(&mut rows, transit.map(|transit| transit.arrival));
    rows.extend([
        row("Maintenance Cost:", maintenance(world, unit.class_dat_id)),
        row("Diplomacy Rating:", unit.skills[0].to_string()),
        row("Espionage Rating:", unit.skills[1].to_string()),
        row("Combat Rating:", unit.skills[5].to_string()),
        row("Leadership Rating:", unit.skills[6].to_string()),
    ]);
    Some(rows)
}

/// The system holding a facility key in one of its lists.
fn facility_system<K: PartialEq>(
    world: &GameWorld,
    key: K,
    list: impl Fn(&rebellion_core::world::System) -> &[K],
) -> Option<SystemKey> {
    world
        .systems
        .iter()
        .find(|(_, value)| list(value).contains(&key))
        .map(|(system, _)| system)
}

/// 34659 "Active". port: a facility exists as a world object only once
/// complete, and sits in a queue or a delivery before that, so "Enroute",
/// "Under Construction" and the ETA row never show (`FUN_0044a470`).
const ACTIVE: &str = "Active";

/// The Defense Facility Status rows (`FUN_00444e20`). Weapons Rating and
/// Shield Strength show only for families `0x22..0x27`.
#[must_use]
pub fn defense_facility_rows(
    sources: StatusSources<'_>,
    key: DefenseFacilityKey,
) -> Option<Vec<StatusRow>> {
    let world = sources.world;
    let facility = world.defense_facilities.get(key)?;
    let system = facility_system(world, key, |value| &value.defense_facilities)?;
    let class = facility.class_dat_id;
    let stats = world
        .buildable_classes
        .get(&class)
        .map(|value| value.stats)
        .unwrap_or_default();
    let mut rows = vec![
        row("Location:", system_name(world, system)),
        row("Status:", ACTIVE),
        row("Maintenance Cost:", maintenance(world, class)),
    ];
    if (0x22..=0x27).contains(&class.family()) {
        rows.extend([
            row("Weapons Rating:", stats.attack_strength.to_string()),
            row("Shield Strength", stats.shield_strength.to_string()),
        ]);
    }
    rows.push(row(
        "Bombardment Defense Strength:",
        stats.bombardment.to_string(),
    ));
    Some(rows)
}

/// The Manufacturing Status rows (`FUN_00446c50`), for a yard, mine or
/// refinery.
fn manufacturing_rows(world: &GameWorld, system: SystemKey, class: DatId) -> Vec<StatusRow> {
    let value = world.buildable_classes.get(&class);
    vec![
        row("Location:", system_name(world, system)),
        row("Status:", ACTIVE),
        row("Maintenance Cost:", maintenance(world, class)),
        row(
            "Standard Processing Rate:",
            value.map_or(0, |value| value.processing_rate).to_string(),
        ),
        row(
            "Bombardment Value:",
            value.map_or(0, |value| value.stats.bombardment).to_string(),
        ),
    ]
}

/// A mission's target by name (`+0x70`), or 34087 "Target Unknown".
fn target_name(world: &GameWorld, mission: &rebellion_core::missions::ActiveMission) -> String {
    let named = |name: Option<String>| name.filter(|name| !name.is_empty());
    let object = mission.target_object.and_then(|target| {
        named(match target {
            MissionTarget::DefenseFacility(key) => world
                .defense_facilities
                .get(key)
                .map(|value| class_name(world, value.class_dat_id)),
            MissionTarget::ManufacturingFacility(key) => world
                .manufacturing_facilities
                .get(key)
                .map(|value| class_name(world, value.class_dat_id)),
            MissionTarget::ProductionFacility(key) => world
                .production_facilities
                .get(key)
                .map(|value| class_name(world, value.class_dat_id)),
            MissionTarget::Troop(key) => world
                .troops
                .get(key)
                .map(|value| class_name(world, value.class_dat_id)),
            MissionTarget::SpecialForce(key) => world
                .special_forces
                .get(key)
                .map(|value| class_name(world, value.class_dat_id)),
            MissionTarget::DeathStar(fleet) => fleet_label(world, fleet),
        })
    });
    object
        .or_else(|| {
            named(
                mission
                    .target_character
                    .and_then(|key| world.characters.get(key))
                    .map(|value| value.name.clone()),
            )
        })
        .or_else(|| named(Some(system_name(world, mission.target_system))))
        .unwrap_or_else(|| "Target Unknown".to_owned())
}

/// The Mission Status rows (`FUN_00445c60`): the team counts its decoys.
///
/// port: the team is the mission's `team` and `decoys`; whether a captured
/// member still counts is untraced, so the captured are left out.
#[must_use]
pub fn mission_rows(sources: StatusSources<'_>, id: u64) -> Option<Vec<StatusRow>> {
    let mission = sources
        .missions
        .missions()
        .iter()
        .find(|mission| mission.id == id)?;
    let first = mission.team.first().or(mission.decoys.first()).copied();
    let arrival = first.and_then(|member| {
        sources
            .missions
            .en_route()
            .iter()
            .find(|transit| transit.member == member && transit.mission_id == id)
            .map(|transit| transit.arrival)
    });
    let mut rows = vec![row("Target:", target_name(sources.world, mission))];
    eta(&mut rows, arrival);
    rows.extend([
        row(
            "Team Size:",
            (mission.team.len() + mission.decoys.len()).to_string(),
        ),
        row("Decoys:", mission.decoys.len().to_string()),
    ]);
    Some(rows)
}

/// An area's manager title (6195, 6185, 6193), the yard family it counts
/// and its busy word (34672 "Building", 34673 "Training").
const fn manager(area: ProductionArea) -> (&'static str, u8, &'static str) {
    match area {
        ProductionArea::ConstructionYard => ("Facilities Under Construction", 0x2a, "Building"),
        ProductionArea::Shipyard => ("Ship Construction", 0x28, "Building"),
        ProductionArea::TrainingFacility => ("Troops in Training", 0x29, "Training"),
    }
}

/// The Production Manager Status rows (`FUN_0044acd0`).
///
/// port: the area's queue stands in for the manager, and a system's yards
/// serve its holder. hyp: the completion day is the queue's last unit's.
#[must_use]
pub fn producer_rows(
    sources: StatusSources<'_>,
    system: SystemKey,
    area: ProductionArea,
) -> Option<Vec<StatusRow>> {
    let world = sources.world;
    let value = world.systems.get(system)?;
    let (_, family, busy) = manager(area);
    let yards = value
        .manufacturing_facilities
        .iter()
        .filter_map(|key| world.manufacturing_facilities.get(*key))
        .filter(|facility| facility.class_dat_id.family() == family)
        .count();
    let queue = sources
        .manufacturing
        .queue(system, area)
        .filter(|queue| !queue.is_empty());
    let status = match (yards, queue) {
        (0, _) => "No Facilities",
        (_, None) => "Idle",
        (_, Some(_)) => busy,
    };
    let mut rows = vec![
        row("Location:", value.name.clone()),
        row("Status:", status),
    ];
    if let (true, Some(queue)) = (yards > 0, queue) {
        rows.push(row("Items to Build:", queue.len().to_string()));
        if let Some(day) = queue.completion_days(sources.today).last() {
            rows.push(row("Estimated Day of Completion:", day.to_string()));
        }
    }
    Some(rows)
}

/// The window's contents for `object`, or `None` once it is gone.
#[must_use]
pub fn status_view(sources: StatusSources<'_>, object: StatusObject) -> Option<StatusView> {
    let world = sources.world;
    let view = |title, side, picture, name, rows| StatusView {
        title,
        manager_title: false,
        side,
        picture,
        name,
        rows,
    };
    Some(match object {
        StatusObject::Character(key) => {
            let character = world.characters.get(key)?;
            let side = if character.is_alliance {
                0
            } else if character.is_empire {
                1
            } else {
                2
            };
            view(
                "Character Status",
                side,
                character_mini_resource_id(character.dat_id, character.is_major)
                    .map(|mini| (DllSource::Gokres, mini - 0x4000)),
                character.name.clone(),
                character_rows(world, sources.missions.en_route(), key)?,
            )
        }
        StatusObject::SpecialForce(key) => {
            let unit = world.special_forces.get(key)?;
            view(
                "Spec Forces Status",
                side_index(unit.is_alliance),
                picture(special_force_mini(unit.class_dat_id).map(|(mini, _)| mini)),
                class_name(world, unit.class_dat_id),
                special_force_rows(sources, key)?,
            )
        }
        StatusObject::Fleet(key) => {
            let fleet = world.fleets.get(key)?;
            // FUN_0042c3b0: STRATEGY 0x28b9 for side 1, 0x28eb otherwise.
            let picture = if fleet.is_alliance { 10_425 } else { 10_475 };
            view(
                "Fleet Status",
                side_index(fleet.is_alliance),
                Some((DllSource::Strategy, picture)),
                fleet_label(world, key).unwrap_or_default(),
                fleet_rows(sources, key)?,
            )
        }
        StatusObject::Ship { fleet, index } => {
            let value = world.fleets.get(fleet)?;
            let ship = value.capital_ships.get(index)?;
            let class = world.capital_ship_classes.get(ship.class)?;
            view(
                "Capital Ship Status",
                side_index(value.is_alliance),
                picture(capital_ship_mini_id(class.dat_id)),
                ship.name.clone().unwrap_or_else(|| class.name.clone()),
                ship_rows(sources, fleet, index)?,
            )
        }
        StatusObject::Fighter { fleet, index } => {
            let value = world.fleets.get(fleet)?;
            let class = world.fighter_classes.get(value.fighters.get(index)?.class)?;
            view(
                "Fighter Squadron Status",
                side_index(value.is_alliance),
                picture(fighter_mini_id(class.dat_id)),
                class.name.clone(),
                fighter_rows(sources, fleet, index)?,
            )
        }
        StatusObject::Troop(key) => {
            let troop = world.troops.get(key)?;
            view(
                "Trooper Regiment Status",
                side_index(troop.is_alliance),
                picture(troop_mini(troop.class_dat_id).map(|(mini, _)| mini)),
                class_name(world, troop.class_dat_id),
                troop_rows(sources, key)?,
            )
        }
        StatusObject::DefenseFacility(key) => {
            let facility = world.defense_facilities.get(key)?;
            view(
                "Defense Facility Status",
                faction_index(facility.side),
                picture(defense_facility_mini(facility.class_dat_id).map(|(mini, _)| mini)),
                class_name(world, facility.class_dat_id),
                defense_facility_rows(sources, key)?,
            )
        }
        StatusObject::ManufacturingFacility(key) => {
            let facility = world.manufacturing_facilities.get(key)?;
            let system = facility_system(world, key, |value| &value.manufacturing_facilities)?;
            view(
                "Manufacturing Status",
                faction_index(facility.side),
                picture(manufacturing_facility_mini(facility.class_dat_id).map(|(mini, _)| mini)),
                class_name(world, facility.class_dat_id),
                manufacturing_rows(world, system, facility.class_dat_id),
            )
        }
        StatusObject::ProductionFacility(key) => {
            let facility = world.production_facilities.get(key)?;
            let system = facility_system(world, key, |value| &value.production_facilities)?;
            view(
                "Manufacturing Status",
                faction_index(facility.side),
                picture(production_facility_mini(facility.class_dat_id).map(|(mini, _)| mini)),
                class_name(world, facility.class_dat_id),
                manufacturing_rows(world, system, facility.class_dat_id),
            )
        }
        StatusObject::Mission(id) => {
            let mission = sources
                .missions
                .missions()
                .iter()
                .find(|mission| mission.id == id)?;
            let side = match mission.faction {
                MissionFaction::Alliance => 0,
                MissionFaction::Empire => 1,
            };
            view(
                "Mission Status",
                side,
                Some((DllSource::Gokres, kind_icon(mission.kind, mission.faction))),
                kind_name(mission.kind).to_owned(),
                mission_rows(sources, id)?,
            )
        }
        StatusObject::Producer { system, area } => {
            let (title, _, _) = manager(area);
            // FUN_00443130: GOKRES 263 (0xa0..0xa1), 262 (0xa2..0xa3), 264.
            let picture = match area {
                ProductionArea::ConstructionYard => 263,
                ProductionArea::Shipyard => 262,
                ProductionArea::TrainingFacility => 264,
            };
            StatusView {
                manager_title: true,
                ..view(
                    title,
                    holder_index(world, system),
                    Some((DllSource::Gokres, picture)),
                    // hyp: the manager's name (FUN_004f62d0) is its
                    // system's.
                    system_name(world, system),
                    producer_rows(sources, system, area)?,
                )
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::dat::ExplorationStatus;
    use rebellion_core::ids::SectorKey;
    use rebellion_core::manufacturing::{BuildableKind, QueueItem};
    use rebellion_core::missions::{MissionKind, MissionRequest};
    use rebellion_core::world::{
        BuildableClass, ClassStats, ControlKind, DefenseFacilityInstance, FighterClass,
        FighterEntry, Fleet, ManufacturingFacilityInstance, SpecialForceUnit, System, TroopUnit,
    };

    /// The states a Status window reads, all idle.
    struct States {
        missions: MissionState,
        movement: MovementState,
        transport: TroopTransportState,
        manufacturing: ManufacturingState,
    }

    impl States {
        fn new() -> Self {
            Self {
                missions: MissionState::new(),
                movement: MovementState::new(),
                transport: TroopTransportState::new(),
                manufacturing: ManufacturingState::new(),
            }
        }

        fn sources<'a>(&'a self, world: &'a GameWorld) -> StatusSources<'a> {
            StatusSources {
                world,
                missions: &self.missions,
                movement: &self.movement,
                transport: &self.transport,
                manufacturing: &self.manufacturing,
                today: 7,
            }
        }
    }

    /// `state` with `entry` appended to its private list `field`, through
    /// its serde form.
    fn with_entry<T: serde::Serialize + serde::de::DeserializeOwned>(
        state: &T,
        field: &str,
        entry: impl serde::Serialize,
    ) -> T {
        let mut value = serde_json::to_value(state).unwrap();
        value[field]
            .as_array_mut()
            .unwrap()
            .push(serde_json::to_value(entry).unwrap());
        serde_json::from_value(value).unwrap()
    }

    fn system(world: &mut GameWorld, name: &str, control: ControlKind) -> SystemKey {
        world.systems.insert(System {
            dat_id: DatId::new(0x9000_0001),
            name: name.into(),
            sector: SectorKey::default(),
            x: 0,
            y: 0,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: Vec::new(),
            ground_units: Vec::new(),
            special_forces: Vec::new(),
            defense_facilities: Vec::new(),
            manufacturing_facilities: Vec::new(),
            production_facilities: Vec::new(),
            is_headquarters: false,
            is_destroyed: false,
            control,
        })
    }

    fn catalog(world: &mut GameWorld, id: u32, name: &str, class: BuildableClass) -> DatId {
        let dat_id = DatId::new(id);
        world.buildable_classes.insert(
            dat_id,
            BuildableClass {
                name: name.into(),
                ..class
            },
        );
        dat_id
    }

    fn rows(rows: &[StatusRow]) -> Vec<(&str, &str)> {
        rows.iter()
            .map(|row| (row.label.as_str(), row.value.as_str()))
            .collect()
    }

    /// An Alliance fleet at `at` of two carriers (2 and 3 squadrons, 1
    /// regiment each), the second damaged, with 4 squadrons and one
    /// character aboard.
    fn carrier_fleet(world: &mut GameWorld, at: SystemKey) -> FleetKey {
        let carrier = |name: &str, squadrons, hyperdrive| CapitalShipClass {
            dat_id: DatId::new(0x1400_0040),
            name: name.into(),
            is_alliance: true,
            fighter_capacity: squadrons,
            troop_capacity: 1,
            hull: 100,
            hyperdrive,
            maintenance_cost: 6,
            turbolaser_starboard: 3,
            turbolaser_port: 4,
            ..CapitalShipClass::default()
        };
        let small = world.capital_ship_classes.insert(CapitalShipClass {
            hyperdrive_if_damaged: 9,
            ..carrier("Small", 2, 0)
        });
        let large = world.capital_ship_classes.insert(carrier("Large", 3, 2));
        let fighter = world.fighter_classes.insert(FighterClass {
            dat_id: DatId::new(0x1c00_0001),
            name: "X-wing".into(),
            squadron_size: 12,
            shield_strength: 5,
            maintenance_cost: 2,
            ..FighterClass::default()
        });
        let mut damaged = ShipInstance::new(large, 100, true);
        damaged.hull_current = 60;
        let character = world.characters.insert(rebellion_core::world::Character::default());
        let fleet = world.insert_fleet(Fleet {
            location: at,
            capital_ships: vec![ShipInstance::new(small, 100, true), damaged],
            fighters: vec![FighterEntry {
                class: fighter,
                count: 4,
            }],
            characters: vec![character],
            is_alliance: true,
            has_death_star: false,
        });
        world.systems[at].fleets.push(fleet);
        fleet
    }

    #[test]
    fn fleet_status_sums_its_ships_and_reads_no_commander() {
        // FUN_00449200: Status, the three command ranks, ship count, the
        // capacity and embarked blocks, personnel, damage and hyperdrive.
        let mut world = GameWorld::default();
        let here = system(&mut world, "Yavin", ControlKind::Controlled(Faction::Alliance));
        let fleet = carrier_fleet(&mut world, here);
        let states = States::new();

        let rows = fleet_rows(states.sources(&world), fleet).unwrap();

        assert_eq!(
            super::tests::rows(&rows),
            vec![
                ("Status:", "Awaiting Orders"),
                ("Admiral:", "Not Assigned"),
                ("General:", "Not Assigned"),
                ("Commander:", "Not Assigned"),
                ("Number Of Ships", "2"),
                ("Capacity:", ""),
                ("Fighter Squadrons:", "5"),
                ("Trooper Regiments:", "2"),
                ("Embarked:", ""),
                ("Fighter Squadrons:", "4"),
                ("Trooper Regiments:", "0"),
                ("Personnel:", "1"),
                ("Damaged Ships:", "1"),
                ("Hyperdrive Rating:", "Yes"),
            ]
        );
    }

    #[test]
    fn a_moving_fleet_is_enroute_with_its_arrival_day() {
        // FUN_00449200 34628 "Enroute"; FUN_004fd2b0 "ETA Destination:".
        let mut world = GameWorld::default();
        let here = system(&mut world, "Yavin", ControlKind::Controlled(Faction::Alliance));
        let there = system(&mut world, "Hoth", ControlKind::Uncontrolled);
        let fleet = carrier_fleet(&mut world, here);
        let mut states = States::new();
        assert!(states.movement.order(fleet, here, there, 5));

        let rows = fleet_rows(states.sources(&world), fleet).unwrap();

        assert_eq!(
            super::tests::rows(&rows[..2]),
            vec![("Status:", "Enroute"), ("ETA Destination:", "Day 12")]
        );
    }

    #[test]
    fn each_ship_carries_the_fleets_squadrons_in_order_up_to_its_capacity() {
        // FUN_00446fd0 reads each ship's own embarked counts (+0x23c);
        // port: the fleet's 4 squadrons fill the first ship's 2 bays first.
        let mut world = GameWorld::default();
        let here = system(&mut world, "Yavin", ControlKind::Controlled(Faction::Alliance));
        let fleet = carrier_fleet(&mut world, here);
        let states = States::new();
        let sources = states.sources(&world);
        let value = |index: usize, label: &str, nth: usize| -> String {
            ship_rows(sources, fleet, index)
                .unwrap()
                .into_iter()
                .filter(|row| row.label == label)
                .nth(nth)
                .unwrap()
                .value
        };

        assert_eq!(value(0, "Fighter Squadrons:", 1), "2");
        assert_eq!(value(1, "Fighter Squadrons:", 1), "2");
        assert_eq!(value(0, "Personnel:", 0), "1");
        assert_eq!(value(1, "Personnel:", 0), "0");
        assert_eq!(value(0, "Ship Damaged:", 0), "No");
        assert_eq!(value(1, "Ship Damaged:", 0), "Yes");
        assert_eq!(value(1, "Hull Value:", 0), "60:100");
        // A class with no hyperdrive shows its damaged rating (FUN_00446fd0).
        assert_eq!(value(0, "Hyperdrive Rating:", 0), "9:0");
        assert_eq!(value(1, "Hyperdrive Rating:", 0), "2:2");
    }

    #[test]
    fn a_fleet_without_a_hyperdrive_ship_reads_no() {
        // FUN_00449200 34645 "No" when no ship can jump.
        let mut world = GameWorld::default();
        let here = system(&mut world, "Yavin", ControlKind::Controlled(Faction::Alliance));
        let fleet = carrier_fleet(&mut world, here);
        world.fleets[fleet].capital_ships[1].alive = false;
        let states = States::new();

        let rows = fleet_rows(states.sources(&world), fleet).unwrap();

        assert_eq!(
            super::tests::rows(&rows).last(),
            Some(&("Hyperdrive Rating:", "No"))
        );
    }

    #[test]
    fn a_ships_weapon_arcs_read_fore_aft_starboard_then_port() {
        // FUN_00446fd0 pushes arc 3 before arc 2 (0x447f2c..0x448650).
        let mut world = GameWorld::default();
        let here = system(&mut world, "Yavin", ControlKind::Controlled(Faction::Alliance));
        let fleet = carrier_fleet(&mut world, here);
        let states = States::new();

        let rows = ship_rows(states.sources(&world), fleet, 0).unwrap();
        let arcs: Vec<(&str, &str)> = super::tests::rows(&rows)
            .into_iter()
            .filter(|(label, _)| label.ends_with("Arc Rating:") || *label == "Turbo Laser:")
            .collect();

        assert_eq!(
            arcs,
            vec![
                ("Forward Weapons Arc Rating:", ""),
                ("Turbo Laser:", "0"),
                ("Aft Weapons Arc Rating:", ""),
                ("Turbo Laser:", "0"),
                ("Starboard Weapons Arc Rating:", ""),
                ("Turbo Laser:", "3"),
                ("Port Weapons Arc Rating:", ""),
                ("Turbo Laser:", "4"),
            ]
        );
    }

    #[test]
    fn a_squadron_has_no_status_row_and_scales_shields_by_its_craft() {
        // FUN_004444c0: Attached, then the ratings; shield strength is per
        // craft times the squadron size.
        let mut world = GameWorld::default();
        let here = system(&mut world, "Yavin", ControlKind::Controlled(Faction::Alliance));
        let fleet = carrier_fleet(&mut world, here);
        let states = States::new();

        let rows = fighter_rows(states.sources(&world), fleet, 0).unwrap();

        assert_eq!(
            super::tests::rows(&rows[..5]),
            vec![
                ("Attached: ", "Fleet 1"),
                ("Maintenance Cost:", "2"),
                ("Squadron Size:", "12:12"),
                ("Hyperdrive Rating:", "0"),
                ("Maximum Shield Strength:", "60:60"),
            ]
        );
    }

    #[test]
    fn a_regiment_reads_its_system_and_class_strengths() {
        // FUN_00445280: Attached, Status, maintenance, attack, defense,
        // bombardment, detection.
        let mut world = GameWorld::default();
        let here = system(&mut world, "Hoth", ControlKind::Controlled(Faction::Alliance));
        let class = catalog(
            &mut world,
            0x1000_0002,
            "Alliance Army Regiment",
            BuildableClass {
                maintenance_cost: 1,
                stats: ClassStats {
                    bombardment: 4,
                    ..ClassStats::default()
                },
                ..BuildableClass::default()
            },
        );
        world.troop_classes.insert(
            class,
            rebellion_core::world::TroopClassDef {
                attack_strength: 7,
                defense_strength: 9,
                detection: 3,
            },
        );
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: class,
            is_alliance: true,
            regiment_strength: 100,
        });
        world.systems[here].ground_units.push(troop);
        let states = States::new();

        let rows = troop_rows(states.sources(&world), troop).unwrap();

        assert_eq!(
            super::tests::rows(&rows),
            vec![
                ("Attached: ", "Hoth"),
                ("Status:", "Awaiting Orders"),
                ("Maintenance Cost:", "1"),
                ("Attack Strength:", "7"),
                ("Defense Strength:", "9"),
                ("Bombardment Value:", "4"),
                ("Detection Value:", "3"),
            ]
        );
    }

    #[test]
    fn a_special_force_reads_skills_zero_one_five_and_six() {
        // FUN_00445780: slots +0x1dc, +0x1e0, +0x1f0, +0x1f4.
        let mut world = GameWorld::default();
        let here = system(&mut world, "Hoth", ControlKind::Controlled(Faction::Alliance));
        let unit = world.special_forces.insert(SpecialForceUnit {
            class_dat_id: DatId::new(0x3c00_0001),
            is_alliance: true,
            skills: [10, 11, 12, 13, 14, 15, 16, 17],
            on_mission: true,
        });
        world.systems[here].special_forces.push(unit);
        let states = States::new();

        let rows = special_force_rows(states.sources(&world), unit).unwrap();

        assert_eq!(
            super::tests::rows(&rows),
            vec![
                ("Attached: ", "Hoth"),
                ("Status:", "On Mission"),
                ("Maintenance Cost:", "0"),
                ("Diplomacy Rating:", "10"),
                ("Espionage Rating:", "11"),
                ("Combat Rating:", "15"),
                ("Leadership Rating:", "16"),
            ]
        );
    }

    #[test]
    fn only_armed_defense_families_show_weapons_and_shield() {
        // FUN_00444e20 writes Weapons Rating and Shield Strength for
        // families 0x22..0x27 only.
        let mut world = GameWorld::default();
        let here = system(&mut world, "Hoth", ControlKind::Controlled(Faction::Alliance));
        let armed_stats = ClassStats {
            bombardment: 2,
            attack_strength: 30,
            shield_strength: 40,
        };
        let mut defense = |id| {
            let class = catalog(
                &mut world,
                id,
                "Defense",
                BuildableClass {
                    stats: armed_stats,
                    ..BuildableClass::default()
                },
            );
            let key = world.defense_facilities.insert(DefenseFacilityInstance {
                class_dat_id: class,
                side: Faction::Alliance,
            });
            world.systems[here].defense_facilities.push(key);
            key
        };
        let armed = defense(0x2200_0001);
        let unarmed = defense(0x2100_0001);
        let states = States::new();
        let sources = states.sources(&world);

        let labels = |key| -> Vec<String> {
            defense_facility_rows(sources, key)
                .unwrap()
                .into_iter()
                .map(|row| row.label)
                .collect()
        };

        assert_eq!(
            labels(armed),
            vec![
                "Location:",
                "Status:",
                "Maintenance Cost:",
                "Weapons Rating:",
                "Shield Strength",
                "Bombardment Defense Strength:",
            ]
        );
        assert_eq!(
            labels(unarmed),
            vec![
                "Location:",
                "Status:",
                "Maintenance Cost:",
                "Bombardment Defense Strength:",
            ]
        );
    }

    #[test]
    fn a_producer_reads_no_facilities_idle_then_its_queue() {
        // FUN_0044acd0: 34671 "No Facilities", 34670 "Idle", 34673
        // "Training" with the items left and the completion day.
        let mut world = GameWorld::default();
        let here = system(&mut world, "Hoth", ControlKind::Controlled(Faction::Alliance));
        let mut states = States::new();
        let status = |world: &GameWorld, states: &States| -> Vec<(String, String)> {
            producer_rows(states.sources(world), here, ProductionArea::TrainingFacility)
                .unwrap()
                .into_iter()
                .map(|row| (row.label, row.value))
                .collect()
        };
        let pair = |label: &str, value: &str| (label.to_owned(), value.to_owned());

        assert_eq!(status(&world, &states)[1], pair("Status:", "No Facilities"));

        let yard = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: DatId::new(0x2900_0001),
                side: Faction::Alliance,
                is_shipyard: false,
            });
        world.systems[here].manufacturing_facilities.push(yard);
        assert_eq!(status(&world, &states)[1], pair("Status:", "Idle"));

        states.manufacturing.enqueue(
            here,
            QueueItem::new(BuildableKind::Troop(DatId::new(0x1000_0002)), 4, 9),
        );
        let rows = status(&world, &states);
        assert_eq!(rows[1], pair("Status:", "Training"));
        assert_eq!(rows[2], pair("Items to Build:", "1"));
        assert_eq!(rows.len(), 4);
    }

    #[test]
    fn a_mission_counts_its_decoys_in_the_team() {
        // FUN_00445c60: Target, Team Size (members and decoys), Decoys.
        let mut world = GameWorld::default();
        let here = system(&mut world, "Bespin", ControlKind::Uncontrolled);
        let mut states = States::new();
        let a = world.characters.insert(rebellion_core::world::Character::default());
        let b = world.characters.insert(rebellion_core::world::Character::default());
        let id = states.missions.dispatch(MissionRequest {
            decoys: vec![MissionMember::Character(b)],
            ..MissionRequest::single(MissionKind::Diplomacy, MissionFaction::Empire, a, here, None, 0)
        });

        let rows = mission_rows(states.sources(&world), id).unwrap();

        assert_eq!(
            super::tests::rows(&rows),
            vec![("Target:", "Bespin"), ("Team Size:", "2"), ("Decoys:", "1")]
        );
    }

    #[test]
    fn each_family_has_its_title_and_picture() {
        // FUN_00443130 titles; FUN_0042c3b0(.., 1, 1): STRATEGY 10425 for a
        // side-1 fleet, a class's +0x30 resource & 0xfff (its list mini less
        // 0x4000), the producer's GOKRES 264 with the manager font.
        let mut world = GameWorld::default();
        let here = system(&mut world, "Yavin", ControlKind::Controlled(Faction::Alliance));
        let fleet = carrier_fleet(&mut world, here);
        let states = States::new();
        let sources = states.sources(&world);
        let view = |object| status_view(sources, object).unwrap();

        let fleet_view = view(StatusObject::Fleet(fleet));
        assert_eq!(fleet_view.title, "Fleet Status");
        assert_eq!(fleet_view.picture, Some((DllSource::Strategy, 10_425)));
        assert_eq!(fleet_view.side, 0);

        let ship = view(StatusObject::Ship { fleet, index: 0 });
        assert_eq!(ship.title, "Capital Ship Status");
        let mini = capital_ship_mini_id(DatId::new(0x1400_0040)).unwrap();
        assert_eq!(ship.picture, Some((DllSource::Gokres, mini - 0x4000)));

        let fighter = view(StatusObject::Fighter { fleet, index: 0 });
        assert_eq!(fighter.title, "Fighter Squadron Status");
        let mini = fighter_mini_id(DatId::new(0x1c00_0001)).unwrap();
        assert_eq!(fighter.picture, Some((DllSource::Gokres, mini - 0x4000)));

        let producer = view(StatusObject::Producer {
            system: here,
            area: ProductionArea::TrainingFacility,
        });
        assert_eq!(producer.title, "Troops in Training");
        assert!(producer.manager_title);
        assert_eq!(producer.picture, Some((DllSource::Gokres, 264)));
        assert!(!fleet_view.manager_title);
    }

    #[test]
    fn an_empire_fleet_shows_the_other_fleet_picture() {
        // FUN_0042c3b0: STRATEGY 10475 for any side but 1.
        let mut world = GameWorld::default();
        let here = system(&mut world, "Coruscant", ControlKind::Controlled(Faction::Empire));
        let fleet = carrier_fleet(&mut world, here);
        world.fleets[fleet].is_alliance = false;
        let states = States::new();

        let view = status_view(states.sources(&world), StatusObject::Fleet(fleet)).unwrap();

        assert_eq!(view.picture, Some((DllSource::Strategy, 10_475)));
        assert_eq!(view.side, 1);
    }

    #[test]
    fn a_facility_takes_its_own_side_for_the_background() {
        // FUN_00443130 picks the background by the object's side bits; a
        // contested system's facility keeps its last holder
        // (ghidra/notes/facility-ownership.md).
        let mut world = GameWorld::default();
        let here = system(&mut world, "Hoth", ControlKind::Contested);
        let yard = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: DatId::new(0x2900_0001),
                side: Faction::Empire,
                is_shipyard: false,
            });
        world.systems[here].manufacturing_facilities.push(yard);
        let states = States::new();

        let view = status_view(
            states.sources(&world),
            StatusObject::ManufacturingFacility(yard),
        )
        .unwrap();

        assert_eq!(view.title, "Manufacturing Status");
        assert_eq!(view.side, 1);
    }

    #[test]
    fn a_travelling_regiment_is_attached_to_its_destination() {
        // FUN_00445280 with FUN_00556390: an object in hyperspace is
        // attached to its destination; 34628 "Enroute" and the ETA.
        let mut world = GameWorld::default();
        let here = system(&mut world, "Hoth", ControlKind::Controlled(Faction::Alliance));
        let there = system(&mut world, "Bespin", ControlKind::Uncontrolled);
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0002),
            is_alliance: true,
            regiment_strength: 100,
        });
        let mut states = States::new();
        states.transport = with_entry(
            &states.transport,
            "transit",
            rebellion_core::troop_transport::RegimentTransit {
                troop,
                origin: here,
                leg: RegimentLeg::Surface(there),
                arrival_tick: 20,
            },
        );

        let rows = troop_rows(states.sources(&world), troop).unwrap();

        assert_eq!(
            super::tests::rows(&rows[..3]),
            vec![
                ("Attached: ", "Bespin"),
                ("Status:", "Enroute"),
                ("ETA Destination:", "Day 20"),
            ]
        );
    }

    #[test]
    fn a_special_force_on_its_way_is_attached_to_the_target() {
        // FUN_00445780: Attached names the transit's destination, then
        // "Enroute" and the ETA.
        let mut world = GameWorld::default();
        let here = system(&mut world, "Hoth", ControlKind::Controlled(Faction::Alliance));
        let there = system(&mut world, "Bespin", ControlKind::Uncontrolled);
        let unit = world.special_forces.insert(SpecialForceUnit {
            class_dat_id: DatId::new(0x3c00_0001),
            is_alliance: true,
            skills: [0; 8],
            on_mission: true,
        });
        world.systems[here].special_forces.push(unit);
        let mut states = States::new();
        states.missions = with_entry(
            &states.missions,
            "en_route",
            rebellion_core::missions::MemberTransit {
                member: MissionMember::SpecialForce(unit),
                mission_id: 1,
                to: there,
                arrival: 15,
            },
        );

        let rows = special_force_rows(states.sources(&world), unit).unwrap();

        assert_eq!(
            super::tests::rows(&rows[..3]),
            vec![
                ("Attached: ", "Bespin"),
                ("Status:", "Enroute"),
                ("ETA Destination:", "Day 15"),
            ]
        );
    }

    #[test]
    fn a_missions_eta_is_its_first_members_transit_for_that_mission() {
        // FUN_00445c60 reads the ETA of the mission's first member.
        let mut world = GameWorld::default();
        let here = system(&mut world, "Bespin", ControlKind::Uncontrolled);
        let mut states = States::new();
        let a = world.characters.insert(rebellion_core::world::Character::default());
        let id = states.missions.dispatch(MissionRequest::single(
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            a,
            here,
            None,
            0,
        ));
        let transit = |mission_id, arrival| rebellion_core::missions::MemberTransit {
            member: MissionMember::Character(a),
            mission_id,
            to: here,
            arrival,
        };
        states.missions = with_entry(&states.missions, "en_route", transit(id + 1, 50));
        states.missions = with_entry(&states.missions, "en_route", transit(id, 30));

        let rows = mission_rows(states.sources(&world), id).unwrap();

        assert_eq!(rows[1], row("ETA Destination:", "Day 30"));
    }

    #[test]
    fn a_producer_without_yards_lists_no_queue() {
        // FUN_0044acd0 writes the items and day only for a manager with
        // facilities.
        let mut world = GameWorld::default();
        let here = system(&mut world, "Hoth", ControlKind::Controlled(Faction::Alliance));
        let mut states = States::new();
        states.manufacturing.enqueue(
            here,
            QueueItem::new(BuildableKind::Troop(DatId::new(0x1000_0002)), 4, 9),
        );

        let rows =
            producer_rows(states.sources(&world), here, ProductionArea::TrainingFacility).unwrap();

        assert_eq!(
            super::tests::rows(&rows),
            vec![("Location:", "Hoth"), ("Status:", "No Facilities")]
        );
    }

    #[test]
    fn a_production_facility_reads_its_rate_and_bombardment() {
        // FUN_00446c50: Location, Status, maintenance, processing rate,
        // bombardment; a neutral facility takes the "other" background.
        let mut world = GameWorld::default();
        let here = system(&mut world, "Kessel", ControlKind::Uncontrolled);
        let class = catalog(
            &mut world,
            0x2c00_0001,
            "Mine",
            BuildableClass {
                maintenance_cost: 1,
                processing_rate: 3,
                stats: ClassStats {
                    bombardment: 8,
                    ..ClassStats::default()
                },
                ..BuildableClass::default()
            },
        );
        let mine = world
            .production_facilities
            .insert(rebellion_core::world::ProductionFacilityInstance {
                class_dat_id: class,
                side: Faction::Neutral,
                is_mine: true,
            });
        world.systems[here].production_facilities.push(mine);
        let states = States::new();

        let view = status_view(
            states.sources(&world),
            StatusObject::ProductionFacility(mine),
        )
        .unwrap();

        assert_eq!(view.name, "Mine");
        assert_eq!(view.side, 2);
        assert_eq!(
            super::tests::rows(&view.rows),
            vec![
                ("Location:", "Kessel"),
                ("Status:", "Active"),
                ("Maintenance Cost:", "1"),
                ("Standard Processing Rate:", "3"),
                ("Bombardment Value:", "8"),
            ]
        );
    }

    #[test]
    fn an_alliance_defense_takes_the_side_one_background_and_its_class_name() {
        // FUN_00443130 background by side; FUN_0044a210 the class's name.
        let mut world = GameWorld::default();
        let here = system(&mut world, "Hoth", ControlKind::Controlled(Faction::Alliance));
        let class = catalog(&mut world, 0x2200_0001, "Planetary Shield", BuildableClass::default());
        let shield = world.defense_facilities.insert(DefenseFacilityInstance {
            class_dat_id: class,
            side: Faction::Alliance,
        });
        world.systems[here].defense_facilities.push(shield);
        let states = States::new();

        let view =
            status_view(states.sources(&world), StatusObject::DefenseFacility(shield)).unwrap();

        assert_eq!(view.title, "Defense Facility Status");
        assert_eq!(view.name, "Planetary Shield");
        assert_eq!(view.side, 0);
        assert_eq!(view.rows[0], row("Location:", "Hoth"));
    }

    #[test]
    fn a_producer_takes_its_systems_holder_for_the_background() {
        // FUN_00443130: a manager's side bits are its system's.
        let mut world = GameWorld::default();
        let empire = system(&mut world, "Kuat", ControlKind::Controlled(Faction::Empire));
        let neutral = system(&mut world, "Kessel", ControlKind::Uncontrolled);
        let states = States::new();
        let side = |system| {
            status_view(
                states.sources(&world),
                StatusObject::Producer {
                    system,
                    area: ProductionArea::Shipyard,
                },
            )
            .unwrap()
            .side
        };

        assert_eq!(side(empire), 1);
        assert_eq!(side(neutral), 2);
    }

    #[test]
    fn an_empire_missions_picture_is_offset_by_0x1000() {
        // FUN_00443130: GOKRES class +0x30 & 0xfff, plus 0x1000 unless the
        // side bits are 1.
        let mut world = GameWorld::default();
        let here = system(&mut world, "Bespin", ControlKind::Uncontrolled);
        let mut states = States::new();
        let a = world.characters.insert(rebellion_core::world::Character::default());
        let request = |faction| {
            MissionRequest::single(MissionKind::Diplomacy, faction, a, here, None, 0)
        };
        let alliance = states.missions.dispatch(request(MissionFaction::Alliance));
        let empire = states.missions.dispatch(request(MissionFaction::Empire));
        let sources = states.sources(&world);
        let view = |id| status_view(sources, StatusObject::Mission(id)).unwrap();

        let (theirs, ours) = (view(empire), view(alliance));
        assert_eq!(theirs.title, "Mission Status");
        assert_eq!((ours.side, theirs.side), (0, 1));
        let Some((DllSource::Gokres, base)) = ours.picture else {
            panic!("a mission has a GOKRES picture");
        };
        assert_eq!(theirs.picture, Some((DllSource::Gokres, base + 0x1000)));
    }
}
