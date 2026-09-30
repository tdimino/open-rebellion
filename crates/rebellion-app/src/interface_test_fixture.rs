//! Test-only deterministic interface fixture bridge.
//!
//! This module is compiled for the dedicated WASM acceptance artifact and
//! native unit tests. Production builds contain no fixture bridge.

use rebellion_core::blockade::{BlockadeState, BlockadeSystem};
use rebellion_core::dat::{ExplorationStatus, Faction};
use rebellion_core::economy::EconomyState;
use rebellion_core::manufacturing::ManufacturingState;
use rebellion_core::missions::{
    available_kinds, MissionFaction, MissionKind, MissionMember, MissionState,
};
use rebellion_core::movement::{
    begin_fleet_transit, fleet_speed, reconcile_fleet_orbits, MovementState,
};
use rebellion_core::tick::TickEvent;
use rebellion_core::uprising::UprisingState;
use rebellion_core::world::{ControlKind, GameWorld, ShipInstance};
use rebellion_render::mission_dialog::{MissionDialogPage, MissionDialogState};
use rebellion_render::object_menu::{ObjectMenuCommand, ObjectMenuState};
use rebellion_render::system_window::SYSTEM_WINDOW_WIDTH;
use rebellion_render::{
    CockpitFaction, CockpitState, GalaxyMapState, GameMessage, GidMode, SectorWindowState,
    SystemWindowState,
};
use serde::Serialize;

use crate::GameMode;

const FIXTURE_ABSENT: u32 = 0;
/// How far right of the galaxy view's centre the targeting scenario puts its
/// target system, clear of the system window it opens on the left.
#[cfg(test)]
const SCENARIO_COUNT: u8 = 48;

extern "C" {
    fn open_rebellion_interface_fixture_code() -> u32;
    fn open_rebellion_interface_fixture_emit(ptr: *const u8, len: usize);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixtureRequest {
    pub scenario: Scenario,
    pub faction: CockpitFaction,
    pub code: u32,
}

impl FixtureRequest {
    pub fn is_packed_encyclopedia(self) -> bool {
        self.scenario == Scenario::PackedEncyclopedia
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Scenario {
    Galaxy = 0,
    Sector = 1,
    System = 2,
    DisplayOff = 3,
    PopularSupport = 4,
    Uprising = 5,
    Fleets = 6,
    Personnel = 7,
    Energy = 8,
    RawMaterial = 9,
    Mines = 10,
    Refineries = 11,
    Shipyards = 12,
    Training = 13,
    Construction = 14,
    Defenses = 15,
    MatchingLegend = 16,
    Known = 17,
    Unknown = 18,
    Uninhabited = 19,
    Headquarters = 20,
    Blockade = 21,
    Mission = 22,
    Fleet = 23,
    DeathStarIntel = 24,
    Hover = 25,
    Selection = 26,
    Pan = 27,
    Zoom = 28,
    FleetsEnroute = 29,
    ActivePersonnel = 30,
    IdleShipyards = 31,
    IdleTraining = 32,
    IdleConstruction = 33,
    Troopers = 34,
    FighterSquadrons = 35,
    DeathStarShields = 36,
    PlanetaryShields = 37,
    EncyclopediaArtwork = 38,
    MessageIndexShell = 39,
    EncyclopediaIndexShell = 40,
    EncyclopediaIndexCatalog = 41,
    MissionDialogMission = 42,
    MissionDialogAgents = 43,
    MissionTargeting = 44,
    FleetMove = 45,
    FleetMoveBlockade = 46,
    PackedEncyclopedia = 47,
}

impl Scenario {
    fn decode(value: u8) -> Option<Self> {
        Some(match value {
            0 => Self::Galaxy,
            1 => Self::Sector,
            2 => Self::System,
            3 => Self::DisplayOff,
            4 => Self::PopularSupport,
            5 => Self::Uprising,
            6 => Self::Fleets,
            7 => Self::Personnel,
            8 => Self::Energy,
            9 => Self::RawMaterial,
            10 => Self::Mines,
            11 => Self::Refineries,
            12 => Self::Shipyards,
            13 => Self::Training,
            14 => Self::Construction,
            15 => Self::Defenses,
            16 => Self::MatchingLegend,
            17 => Self::Known,
            18 => Self::Unknown,
            19 => Self::Uninhabited,
            20 => Self::Headquarters,
            21 => Self::Blockade,
            22 => Self::Mission,
            23 => Self::Fleet,
            24 => Self::DeathStarIntel,
            25 => Self::Hover,
            26 => Self::Selection,
            27 => Self::Pan,
            28 => Self::Zoom,
            29 => Self::FleetsEnroute,
            30 => Self::ActivePersonnel,
            31 => Self::IdleShipyards,
            32 => Self::IdleTraining,
            33 => Self::IdleConstruction,
            34 => Self::Troopers,
            35 => Self::FighterSquadrons,
            36 => Self::DeathStarShields,
            37 => Self::PlanetaryShields,
            38 => Self::EncyclopediaArtwork,
            39 => Self::MessageIndexShell,
            40 => Self::EncyclopediaIndexShell,
            41 => Self::EncyclopediaIndexCatalog,
            42 => Self::MissionDialogMission,
            43 => Self::MissionDialogAgents,
            44 => Self::MissionTargeting,
            45 => Self::FleetMove,
            46 => Self::FleetMoveBlockade,
            47 => Self::PackedEncyclopedia,
            _ => return None,
        })
    }

    fn moves_a_fleet(self) -> bool {
        matches!(self, Self::FleetMove | Self::FleetMoveBlockade)
    }

    /// The scenarios whose gate opens the object pop-up menu.
    fn reports_object_menu(self) -> bool {
        self == Self::MissionTargeting || self.moves_a_fleet()
    }

    pub fn mode(self) -> GidMode {
        match self {
            Self::DisplayOff => GidMode::DisplayOff,
            Self::Uprising => GidMode::Uprisings,
            Self::Fleets => GidMode::IdleFleets,
            Self::Personnel => GidMode::IdlePersonnel,
            Self::Energy => GidMode::AvailableEnergy,
            Self::RawMaterial => GidMode::AvailableRawMaterial,
            Self::Mines => GidMode::Mines,
            Self::Refineries => GidMode::Refineries,
            Self::Shipyards => GidMode::Shipyards,
            Self::Training => GidMode::TrainingFacilities,
            Self::Construction => GidMode::ConstructionYards,
            Self::Defenses => GidMode::PlanetaryDefenseBatteries,
            Self::FleetsEnroute => GidMode::FleetsEnRoute,
            Self::ActivePersonnel => GidMode::ActivePersonnel,
            Self::IdleShipyards => GidMode::IdleShipyards,
            Self::IdleTraining => GidMode::IdleTrainingFacilities,
            Self::IdleConstruction => GidMode::IdleConstructionYards,
            Self::Troopers => GidMode::Troopers,
            Self::FighterSquadrons => GidMode::FighterSquadrons,
            Self::DeathStarShields => GidMode::DeathStarShields,
            Self::PlanetaryShields => GidMode::PlanetaryShieldGenerators,
            _ => GidMode::PopularSupport,
        }
    }
}

pub fn requested() -> Option<FixtureRequest> {
    decode_request(unsafe { open_rebellion_interface_fixture_code() })
}

pub(crate) fn decode_request(code: u32) -> Option<FixtureRequest> {
    if code == FIXTURE_ABSENT || code >> 16 != 0 {
        return None;
    }
    let scenario = Scenario::decode(((code & 0xff) as u8).checked_sub(1)?)?;
    let faction = match (code >> 8) & 0xff {
        1 => CockpitFaction::Alliance,
        2 => CockpitFaction::Empire,
        _ => return None,
    };
    Some(FixtureRequest {
        scenario,
        faction,
        code,
    })
}

pub fn emit_report(report: &impl Serialize) {
    let bytes = serde_json::to_vec(report).expect("serialize interface fixture report");
    unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
}

#[allow(clippy::too_many_arguments)]
#[expect(
    clippy::too_many_lines,
    reason = "Keep the deterministic browser fixture setup in its existing order."
)]
pub fn apply(
    request: FixtureRequest,
    world: &mut GameWorld,
    game_mode: &mut GameMode,
    player_faction: &mut MissionFaction,
    cockpit: &mut CockpitState,
    map: &mut GalaxyMapState,
    movement: &mut MovementState,
    manufacturing: &mut ManufacturingState,
    economy: &mut EconomyState,
    missions: &mut MissionState,
    blockade: &mut BlockadeState,
    sectors: &mut SectorWindowState,
    systems: &mut SystemWindowState,
) {
    *game_mode = GameMode::Galaxy;
    *player_faction = match request.faction {
        CockpitFaction::Alliance => MissionFaction::Alliance,
        CockpitFaction::Empire => MissionFaction::Empire,
    };
    cockpit.faction = request.faction;
    cockpit.gid_mode = request.scenario.mode();
    cockpit.gid_ui.menu_open = false;
    cockpit.gid_ui.category = None;

    map.camera_x = 450.0;
    map.camera_y = 470.0;
    map.zoom = if request.scenario == Scenario::Zoom {
        1.65
    } else {
        1.0
    };
    if request.scenario == Scenario::Pan {
        map.camera_x = 565.0;
        map.camera_y = 390.0;
    }

    let system_keys: Vec<_> = world.systems.keys().take(10).collect();
    let Some(&primary) = system_keys.first() else {
        return;
    };
    let secondary = system_keys.get(1).copied().unwrap_or(primary);
    let player_is_alliance = request.faction == CockpitFaction::Alliance;
    let player_control = if player_is_alliance {
        Faction::Alliance
    } else {
        Faction::Empire
    };
    let enemy_control = if player_is_alliance {
        Faction::Empire
    } else {
        Faction::Alliance
    };

    for (index, key) in system_keys.iter().copied().enumerate() {
        if let Some(system) = world.systems.get_mut(key) {
            system.exploration_status = ExplorationStatus::Explored;
            system.is_populated = true;
            system.is_destroyed = false;
            system.is_headquarters = false;
            system.popularity_alliance = [0.92, 0.72, 0.55, 0.28][index % 4];
            system.popularity_empire = 1.0 - system.popularity_alliance;
            system.control = ControlKind::Controlled(if index % 2 == 0 {
                player_control
            } else {
                enemy_control
            });
            system.total_energy = 12;
            system.raw_materials = 11;
        }
        economy.per_system.entry(key).or_default().energy_allocated = 3;
        economy
            .per_system
            .entry(key)
            .or_default()
            .raw_material_allocated = 2;
    }

    match request.scenario {
        Scenario::Unknown => {
            world.systems[primary].exploration_status = ExplorationStatus::Unexplored;
        }
        Scenario::Uninhabited => {
            world.systems[primary].is_populated = false;
        }
        Scenario::Headquarters => {
            world.systems[primary].is_headquarters = true;
        }
        Scenario::Uprising => {
            world.systems[primary].control = ControlKind::Uprising(player_control);
        }
        _ => {}
    }

    if let Some((character_key, character)) = world.characters.iter_mut().next() {
        character.current_system = Some(primary);
        character.current_fleet = None;
        character.is_alliance = player_is_alliance;
        character.is_empire = !player_is_alliance;
        character.on_mission = matches!(
            request.scenario,
            Scenario::Mission | Scenario::ActivePersonnel
        );
        if request.scenario == Scenario::MissionTargeting {
            // MissionState::mission_order_enabled: a free recruited member.
            character.recruited = true;
            character.is_captive = false;
            character.on_mandatory_mission = false;
        }
        if matches!(
            request.scenario,
            Scenario::Mission | Scenario::ActivePersonnel
        ) {
            let faction = if player_is_alliance {
                MissionFaction::Alliance
            } else {
                MissionFaction::Empire
            };
            missions.dispatch(rebellion_core::missions::MissionRequest::single(
                MissionKind::Espionage,
                faction,
                character_key,
                primary,
                None,
                0,
            ));
        }
    }

    let mut transit_fleet = None;
    if let Some((fleet_key, fleet)) = world.fleets.iter_mut().next() {
        fleet.location = primary;
        fleet.is_alliance = player_is_alliance;
        fleet.has_death_star = request.scenario == Scenario::DeathStarIntel;
        transit_fleet = Some(fleet_key);
    }
    if matches!(request.scenario, Scenario::FleetsEnroute) {
        if let Some(fleet_key) = transit_fleet {
            let _ = begin_fleet_transit(movement, world, fleet_key, secondary, 12);
        }
    }
    reconcile_fleet_orbits(movement, world);

    if request.scenario == Scenario::Blockade {
        world.systems[primary].control = ControlKind::Controlled(player_control);
        if let Some((_, fleet)) = world.fleets.iter_mut().next() {
            fleet.location = primary;
            fleet.is_alliance = !player_is_alliance;
        }
        reconcile_fleet_orbits(movement, world);
        let _ = BlockadeSystem::advance(blockade, world, &[TickEvent { tick: 1 }]);
    }

    if request.scenario == Scenario::Selection {
        map.selected_system = Some(primary);
    }
    if request.scenario == Scenario::Sector {
        sectors.open_for_system(world, primary, request.faction);
    }
    if request.scenario == Scenario::System {
        systems.open(
            world,
            primary,
            (225, 76),
            request.faction,
            cockpit.layout_for(640.0, 480.0),
        );
    }
    if request.scenario.moves_a_fleet() {
        place_moving_fleet(
            request,
            world,
            cockpit,
            movement,
            blockade,
            sectors,
            systems,
            (
                primary,
                secondary,
                *system_keys.last().unwrap_or(&secondary),
            ),
        );
    }
    if request.scenario == Scenario::MissionTargeting {
        // The primary system's window at the galaxy view's right edge holds
        // the agent; the second system's planet in its sector window, in the
        // first column on the left, is the target (FUN_0045c830).
        let layout = cockpit.layout_for(640.0, 480.0);
        let galaxy = layout.galaxy;
        systems.open(
            world,
            primary,
            (
                (galaxy.x + galaxy.width - SYSTEM_WINDOW_WIDTH) as i16 - 5,
                galaxy.y as i16 + 5,
            ),
            request.faction,
            layout,
        );
        sectors.open_for_system(world, secondary, request.faction);
    }

    let _ = manufacturing;
}

/// The fleet-move scenarios: the player's first fleet, alone in the primary
/// system and able to enter hyperspace, in that system's window at the galaxy
/// view's right edge, with the second system's sector window open as the
/// target. In the blockade variant the enemy holds the primary system, so the
/// player's fleet is alone there and blockades it
/// (`BlockadeSystem::system_is_blockaded`).
#[allow(clippy::too_many_arguments)]
fn place_moving_fleet(
    request: FixtureRequest,
    world: &mut GameWorld,
    cockpit: &CockpitState,
    movement: &mut MovementState,
    blockade: &mut BlockadeState,
    sectors: &mut SectorWindowState,
    systems: &mut SystemWindowState,
    (primary, secondary, elsewhere): (
        rebellion_core::ids::SystemKey,
        rebellion_core::ids::SystemKey,
        rebellion_core::ids::SystemKey,
    ),
) {
    let player_is_alliance = request.faction == CockpitFaction::Alliance;
    let Some(player) = world.fleets.keys().next() else {
        return;
    };
    for (key, fleet) in &mut world.fleets {
        if key != player && fleet.location == primary {
            fleet.location = elsewhere;
        }
    }
    if let Some(class) = world.capital_ship_classes.keys().next() {
        if fleet_speed(&world.fleets[player], world).is_none() {
            world.fleets[player].capital_ships.push(ShipInstance::new(
                class,
                100,
                player_is_alliance,
            ));
        }
    }
    world.fleets[player].location = primary;
    world.fleets[player].is_alliance = player_is_alliance;
    reconcile_fleet_orbits(movement, world);
    if request.scenario == Scenario::FleetMoveBlockade {
        world.systems[primary].control = ControlKind::Controlled(if player_is_alliance {
            Faction::Empire
        } else {
            Faction::Alliance
        });
        let _ = BlockadeSystem::advance(blockade, world, &[TickEvent { tick: 1 }]);
    }
    let layout = cockpit.layout_for(640.0, 480.0);
    let galaxy = layout.galaxy;
    systems.open_fleet(
        world,
        player,
        (
            (galaxy.x + galaxy.width - SYSTEM_WINDOW_WIDTH) as i16 - 5,
            galaxy.y as i16 + 5,
        ),
        request.faction,
        layout,
    );
    sectors.open_for_system(world, secondary, request.faction);
}

#[derive(Serialize)]
struct FixtureReady<'a> {
    schema_version: u32,
    status: &'static str,
    code: u32,
    scenario: u8,
    faction: &'static str,
    mode: &'a str,
    state_fingerprint: String,
    stable_frames: u32,
    primary_system_x: u16,
    primary_system_y: u16,
    probe_system_dat_id: u32,
    probe_system_name: &'a str,
    probe_screen_x: f32,
    probe_screen_y: f32,
    camera_x: f32,
    camera_y: f32,
    zoom: f32,
}

pub fn emit_ready(request: FixtureRequest, world: &GameWorld, map: &GalaxyMapState) {
    let faction = match request.faction {
        CockpitFaction::Alliance => "alliance",
        CockpitFaction::Empire => "empire",
    };
    let fingerprint_input = serde_json::to_vec(&(
        request.code,
        world,
        map.camera_x,
        map.camera_y,
        map.zoom,
        map.selected_system,
    ))
    .expect("serialize deterministic interface fixture state");
    let aperture = CockpitState::new(request.faction)
        .layout_for(640.0, 480.0)
        .galaxy;
    let probe = world
        .systems
        .iter()
        .map(|(_, system)| {
            let (x, y) = screen_point(system, map, request.faction);
            (system, x, y)
        })
        .filter(|(_, x, y)| {
            *x > aperture.x + 20.0
                && *x < aperture.x + aperture.width - 20.0
                && *y > aperture.y + 20.0
                && *y < aperture.y + aperture.height - 20.0
        })
        .min_by(|a, b| {
            let center_x = aperture.x + aperture.width / 2.0;
            let center_y = aperture.y + aperture.height / 2.0;
            let distance = |x: f32, y: f32| (x - center_x).powi(2) + (y - center_y).powi(2);
            distance(a.1, a.2).total_cmp(&distance(b.1, b.2))
        })
        .expect("fixture has a visible system for hover probe");
    let report = FixtureReady {
        schema_version: 1,
        status: "ready",
        code: request.code,
        scenario: request.scenario as u8,
        faction,
        mode: request.scenario.mode().label(),
        state_fingerprint: format!("fnv1a64:{:016x}", fnv1a64(&fingerprint_input)),
        stable_frames: 2,
        primary_system_x: world.systems.iter().next().map_or(0, |(_, value)| value.x),
        primary_system_y: world.systems.iter().next().map_or(0, |(_, value)| value.y),
        probe_system_dat_id: probe.0.dat_id.raw(),
        probe_system_name: &probe.0.name,
        probe_screen_x: probe.1,
        probe_screen_y: probe.2,
        camera_x: map.camera_x,
        camera_y: map.camera_y,
        zoom: map.zoom,
    };
    let bytes = serde_json::to_vec(&report).expect("serialize interface fixture report");
    unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
}

/// Where `system` is drawn in the 640 by 480 fixture canvas.
fn screen_point(
    system: &rebellion_core::world::System,
    map: &GalaxyMapState,
    faction: CockpitFaction,
) -> (f32, f32) {
    let aperture = CockpitState::new(faction).layout_for(640.0, 480.0).galaxy;
    (
        (f32::from(system.x) - map.camera_x) * map.zoom + aperture.x + aperture.width / 2.0,
        (f32::from(system.y) - map.camera_y) * map.zoom + aperture.y + aperture.height / 2.0,
    )
}

/// The open object pop-up menu and the targeting scenario's target planet,
/// so the browser gate can choose Mission and release over it.
#[derive(Debug, Serialize, PartialEq)]
struct FixtureObjectMenu<'a> {
    status: &'static str,
    code: u32,
    left: f32,
    top: f32,
    width: f32,
    height: f32,
    rows: usize,
    mission_row: Option<usize>,
    move_row: Option<usize>,
    confirmed_move_row: Option<usize>,
    target_dat_id: u32,
    target_name: &'a str,
    target_screen_x: f32,
    target_screen_y: f32,
}

fn object_menu_report<'a>(
    request: FixtureRequest,
    rect: egui_macroquad::egui::Rect,
    menu: &ObjectMenuState,
    world: &'a GameWorld,
    sectors: &SectorWindowState,
) -> Option<FixtureObjectMenu<'a>> {
    let (key, target) = world.systems.iter().nth(1)?;
    let layout = CockpitState::new(request.faction).layout_for(640.0, 480.0);
    let planet = sectors.planet_screen_rect(world, layout, key)?.center();
    let (target_screen_x, target_screen_y) = (planet.x, planet.y);
    Some(FixtureObjectMenu {
        status: "object-menu",
        code: request.code,
        left: rect.min.x,
        top: rect.min.y,
        width: rect.width(),
        height: rect.height(),
        rows: menu.row_count(),
        mission_row: menu.row_of(ObjectMenuCommand::Mission),
        move_row: menu.row_of(ObjectMenuCommand::Move),
        confirmed_move_row: menu.row_of(ObjectMenuCommand::ConfirmedMove),
        target_dat_id: target.dat_id.raw(),
        target_name: &target.name,
        target_screen_x,
        target_screen_y,
    })
}

pub fn emit_object_menu(
    request: FixtureRequest,
    rect: egui_macroquad::egui::Rect,
    menu: &ObjectMenuState,
    world: &GameWorld,
    sectors: &SectorWindowState,
) {
    if !request.scenario.reports_object_menu() {
        return;
    }
    let Some(report) = object_menu_report(request, rect, menu, world, sectors) else {
        return;
    };
    let bytes = serde_json::to_vec(&report).expect("serialize the object menu report");
    unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
}

#[derive(Debug, Serialize, PartialEq)]
struct FixtureSystemPoint {
    dat_id: u32,
    x: f32,
    y: f32,
}

/// Where the fleet-move gate presses and releases: the moving fleet's cell,
/// the target planet in the sector window, and every system on the map.
#[derive(Debug, Serialize, PartialEq)]
struct FixtureFleetMoveSetup {
    status: &'static str,
    code: u32,
    primary_dat_id: u32,
    fleet_item_x: f32,
    fleet_item_y: f32,
    target_dat_id: u32,
    target_screen_x: f32,
    target_screen_y: f32,
    systems: Vec<FixtureSystemPoint>,
}

fn fleet_move_setup(
    request: FixtureRequest,
    world: &GameWorld,
    map: &GalaxyMapState,
    sectors: &SectorWindowState,
    systems: &SystemWindowState,
) -> Option<FixtureFleetMoveSetup> {
    if !request.scenario.moves_a_fleet() {
        return None;
    }
    let layout = CockpitState::new(request.faction).layout_for(640.0, 480.0);
    let mut keys = world.systems.keys();
    let (primary, target) = (keys.next()?, keys.next()?);
    let item = systems.first_item_screen_rect(layout, primary)?.center();
    let planet = sectors.planet_screen_rect(world, layout, target)?.center();
    Some(FixtureFleetMoveSetup {
        status: "fleet-move-setup",
        code: request.code,
        primary_dat_id: world.systems[primary].dat_id.raw(),
        fleet_item_x: item.x,
        fleet_item_y: item.y,
        target_dat_id: world.systems[target].dat_id.raw(),
        target_screen_x: planet.x,
        target_screen_y: planet.y,
        systems: world
            .systems
            .values()
            .map(|system| {
                let (x, y) = screen_point(system, map, request.faction);
                FixtureSystemPoint {
                    dat_id: system.dat_id.raw(),
                    x,
                    y,
                }
            })
            .collect(),
    })
}

pub fn emit_fleet_move_setup(
    request: FixtureRequest,
    world: &GameWorld,
    map: &GalaxyMapState,
    sectors: &SectorWindowState,
    systems: &SystemWindowState,
) {
    let Some(report) = fleet_move_setup(request, world, map, sectors, systems) else {
        return;
    };
    let bytes = serde_json::to_vec(&report).expect("serialize the fleet move setup");
    unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
}

/// What the fleet-move gate checks after each step: the fleet's transit
/// order, whether the confirmation window is open, the map's selection, and
/// the last message (a refusal's text).
#[derive(Debug, Clone, Serialize, PartialEq)]
struct FleetMoveObservation {
    status: &'static str,
    code: u32,
    in_transit: bool,
    destination_dat_id: Option<u32>,
    confirmation_open: bool,
    selected_system_dat_id: Option<u32>,
    last_message: Option<String>,
}

fn fleet_move_observation(
    request: FixtureRequest,
    world: &GameWorld,
    movement: &MovementState,
    confirmation_open: bool,
    map: &GalaxyMapState,
    messages: &[GameMessage],
) -> Option<FleetMoveObservation> {
    if !request.scenario.moves_a_fleet() {
        return None;
    }
    let fleet = world.fleets.keys().next()?;
    let dat_id = |key| world.systems.get(key).map(|system| system.dat_id.raw());
    Some(FleetMoveObservation {
        status: "fleet-move",
        code: request.code,
        in_transit: movement.is_in_transit(fleet),
        destination_dat_id: movement
            .get(fleet)
            .and_then(|order| dat_id(order.destination)),
        confirmation_open,
        selected_system_dat_id: map.selected_system.and_then(dat_id),
        last_message: messages.last().map(|message| message.text.clone()),
    })
}

/// Emits a [`FleetMoveObservation`] whenever it changes.
#[derive(Debug, Default)]
pub struct FleetMoveWatch {
    last: Option<FleetMoveObservation>,
}

impl FleetMoveWatch {
    fn changed(&mut self, now: Option<FleetMoveObservation>) -> Option<&FleetMoveObservation> {
        if now.is_none() || now == self.last {
            return None;
        }
        self.last = now;
        self.last.as_ref()
    }

    pub fn observe(
        &mut self,
        request: FixtureRequest,
        world: &GameWorld,
        movement: &MovementState,
        confirmation_open: bool,
        map: &GalaxyMapState,
        messages: &[GameMessage],
    ) {
        let now =
            fleet_move_observation(request, world, movement, confirmation_open, map, messages);
        let Some(report) = self.changed(now) else {
            return;
        };
        let bytes = serde_json::to_vec(report).expect("serialize the fleet move observation");
        unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
    }
}

#[derive(Serialize)]
struct FixtureSelected<'a> {
    status: &'static str,
    code: u32,
    mode: &'a str,
    command_id: u8,
}

pub fn emit_selected(request: FixtureRequest, mode: GidMode) {
    let report = FixtureSelected {
        status: "selected",
        code: request.code,
        mode: mode.label(),
        command_id: mode.command_id(),
    };
    let bytes = serde_json::to_vec(&report).expect("serialize selected GID mode");
    unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
}

#[derive(Serialize)]
struct FixtureHover<'a> {
    status: &'static str,
    code: u32,
    system_dat_id: u32,
    system_name: &'a str,
}

pub fn emit_hover(request: FixtureRequest, world: &GameWorld, map: &GalaxyMapState) {
    let Some(system) = map.hovered_system.and_then(|key| world.systems.get(key)) else {
        return;
    };
    let report = FixtureHover {
        status: "hovered",
        code: request.code,
        system_dat_id: system.dat_id.raw(),
        system_name: &system.name,
    };
    let bytes = serde_json::to_vec(&report).expect("serialize hovered GID system");
    unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
}

/// Opens the mission dialog for the mission-dialog scenarios, as a drop of the
/// fixture's first character onto its primary system would (`FUN_0042a320`).
/// Returns false for every other scenario, or when no kind is available.
pub fn open_mission_dialog(
    request: FixtureRequest,
    world: &GameWorld,
    uprisings: &UprisingState,
    dialog: &mut MissionDialogState,
) -> bool {
    let page = match request.scenario {
        Scenario::MissionDialogMission => MissionDialogPage::Mission,
        Scenario::MissionDialogAgents => MissionDialogPage::Agents,
        _ => return false,
    };
    let (Some(character), Some(target)) =
        (world.characters.keys().next(), world.systems.keys().next())
    else {
        return false;
    };
    let faction = match request.faction {
        CockpitFaction::Alliance => MissionFaction::Alliance,
        CockpitFaction::Empire => MissionFaction::Empire,
    };
    let team = vec![MissionMember::Character(character)];
    let kinds = available_kinds(world, uprisings, faction, &team, &[], target);
    if !dialog.open(faction, target, team, kinds) {
        return false;
    }
    if let Some(open) = dialog.dialog_mut() {
        open.show_page(page);
    }
    true
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_render::object_menu::{MenuObject, OrderGates};

    #[test]
    fn scenario_table_is_stable_and_complete() {
        for value in 0..SCENARIO_COUNT {
            assert_eq!(Scenario::decode(value).unwrap() as u8, value);
        }
        assert!(Scenario::decode(SCENARIO_COUNT).is_none());
    }

    /// One system held by `side` with one recruited character of that side
    /// on it, and the shipped MISSNSD Diplomacy record `0x51000010`.
    fn diplomacy_world(side: Faction) -> GameWorld {
        use rebellion_core::world::{
            Character, MissionMemberRules, MissionRecord, MissionTargetRules, SkillPair, System,
        };
        let mut world = GameWorld::default();
        let here = world.systems.insert(System {
            dat_id: rebellion_core::ids::DatId::new(0x9000_0001),
            name: "System".into(),
            sector: rebellion_core::ids::SectorKey::default(),
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
            control: ControlKind::Controlled(side),
        });
        world.characters.insert(Character {
            name: "Agent".into(),
            is_alliance: side == Faction::Alliance,
            is_empire: side == Faction::Empire,
            loyalty: SkillPair {
                base: 100,
                variance: 0,
            },
            current_system: Some(here),
            recruited: true,
            ..Default::default()
        });
        world.mission_records = vec![MissionRecord {
            dat_id: rebellion_core::ids::DatId::new(0x5100_0010),
            timer_min_days: 5,
            timer_spread_days: 10,
            repeats: true,
            hidden: false,
            detection_phases: true,
            can_resign: true,
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
        }];
        world
    }

    fn request(scenario: Scenario, faction: CockpitFaction) -> FixtureRequest {
        FixtureRequest {
            scenario,
            faction,
            code: 0,
        }
    }

    #[test]
    fn the_agents_scenario_opens_the_dialog_on_its_second_page() {
        let world = diplomacy_world(Faction::Alliance);
        let mut dialog = MissionDialogState::default();

        let opened = open_mission_dialog(
            request(Scenario::MissionDialogAgents, CockpitFaction::Alliance),
            &world,
            &UprisingState::default(),
            &mut dialog,
        );

        assert!(opened);
        let open = dialog.dialog().expect("the dialog is open");
        assert_eq!(open.page(), MissionDialogPage::Agents);
        assert_eq!(open.kind(), MissionKind::Diplomacy);
        assert_eq!(open.agents().len(), 1);
    }

    #[test]
    fn the_mission_scenario_opens_the_empire_dialog_on_its_first_page() {
        let world = diplomacy_world(Faction::Empire);
        let mut dialog = MissionDialogState::default();

        let opened = open_mission_dialog(
            request(Scenario::MissionDialogMission, CockpitFaction::Empire),
            &world,
            &UprisingState::default(),
            &mut dialog,
        );

        assert!(opened);
        let open = dialog.dialog().expect("the dialog is open");
        assert_eq!(open.page(), MissionDialogPage::Mission);
        let Some(rebellion_render::mission_dialog::MissionDialogAction::Begin { faction, .. }) =
            dialog.begin()
        else {
            panic!("an open dialog begins its mission");
        };
        assert_eq!(faction, MissionFaction::Empire);
    }

    #[test]
    fn other_scenarios_and_refused_teams_open_no_dialog() {
        // FUN_0042a320: with no legal kind nothing opens. An Alliance agent
        // cannot run an Empire mission (FUN_00583320).
        let world = diplomacy_world(Faction::Alliance);
        for request in [
            request(Scenario::Galaxy, CockpitFaction::Alliance),
            request(Scenario::MissionDialogAgents, CockpitFaction::Empire),
        ] {
            let mut dialog = MissionDialogState::default();

            let opened =
                open_mission_dialog(request, &world, &UprisingState::default(), &mut dialog);

            assert!(!opened, "{:?}", request.scenario);
            assert!(!dialog.is_open());
        }
        let mut dialog = MissionDialogState::default();
        assert!(!open_mission_dialog(
            request(Scenario::MissionDialogAgents, CockpitFaction::Alliance),
            &GameWorld::default(),
            &UprisingState::default(),
            &mut dialog,
        ));
    }

    #[test]
    fn the_targeting_scenario_frees_the_agent_and_shows_the_target_planet_clear_of_its_window() {
        let mut world = diplomacy_world(Faction::Alliance);
        let sector = world.sectors.insert(rebellion_core::world::Sector {
            dat_id: rebellion_core::ids::DatId::new(0x8000_0001),
            name: "Sector".into(),
            group: rebellion_core::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: Vec::new(),
        });
        let here = world.systems.keys().next().unwrap();
        world.systems[here].sector = sector;
        let mut second = world.systems[here].clone();
        second.dat_id = rebellion_core::ids::DatId::new(0x9000_0002);
        second.name = "Target".into();
        second.x = 26;
        second.y = 20;
        let target = world.systems.insert(second);
        world.sectors[sector].systems = vec![here, target];
        let agent = world.characters.keys().next().unwrap();
        world.characters[agent].recruited = false;
        world.characters[agent].is_captive = true;
        let request = request(Scenario::MissionTargeting, CockpitFaction::Alliance);
        let mut cockpit = CockpitState::new(CockpitFaction::Alliance);
        let mut missions = MissionState::default();
        let mut sectors = SectorWindowState::default();
        let mut systems = SystemWindowState::default();

        apply(
            request,
            &mut world,
            &mut GameMode::MainMenu,
            &mut MissionFaction::Alliance,
            &mut cockpit,
            &mut GalaxyMapState::default(),
            &mut MovementState::default(),
            &mut ManufacturingState::default(),
            &mut EconomyState::default(),
            &mut missions,
            &mut BlockadeState::default(),
            &mut sectors,
            &mut systems,
        );

        // MissionState::mission_order_enabled enables the menu's Mission.
        assert!(missions.mission_order_enabled(
            &world,
            MissionFaction::Alliance,
            &[MissionMember::Character(agent)],
        ));
        let layout = cockpit.layout_for(640.0, 480.0);
        let galaxy = layout.galaxy;
        // The window's top-left corner, 5 pixels in from the view's top-right.
        let corner = (
            galaxy.x + galaxy.width - SYSTEM_WINDOW_WIDTH - 5.0,
            galaxy.y + 5.0,
        );
        assert!(systems.contains_screen_point(layout, corner));
        assert!(!systems.contains_screen_point(layout, (corner.0 - 1.0, corner.1)));
        assert!(!systems.contains_screen_point(layout, (corner.0, corner.1 - 1.0)));
        assert_eq!(sectors.window_count(), 1);

        let menu = ObjectMenuState::new(
            Some(MenuObject::Character(agent)),
            OrderGates {
                mission: true,
                fleet_move: false,
            },
            (0, 0),
        );
        let rect = egui_macroquad::egui::Rect::from_min_size(
            egui_macroquad::egui::pos2(100.0, 150.0),
            egui_macroquad::egui::vec2(120.0, 142.0),
        );
        let report = object_menu_report(request, rect, &menu, &world, &sectors).unwrap();
        assert_eq!(
            (report.left, report.top, report.width, report.height),
            (100.0, 150.0, 120.0, 142.0)
        );
        assert_eq!((report.rows, report.mission_row), (7, Some(2)));
        assert_eq!(
            (report.target_dat_id, report.target_name),
            (0x9000_0002, "Target")
        );
        // The Alliance's first sector window sits at (60, 35); the target's
        // planet at (74, 74) in it is 37 by 37.
        let target = (report.target_screen_x, report.target_screen_y);
        assert_eq!(target, (60.0 + 74.0 + 18.5, 35.0 + 74.0 + 18.5));
        assert!(sectors.contains_screen_point(layout, target));
        assert!(!systems.contains_screen_point(layout, target));
    }

    /// Three systems in one sector, the second the target in the targeting
    /// layout; three fleets of the other side with no capital ship, two at the
    /// primary system and one at the target; and one capital ship class.
    fn fleet_world() -> GameWorld {
        use rebellion_core::world::{CapitalShipClass, Fleet};
        let mut world = diplomacy_world(Faction::Alliance);
        let sector = world.sectors.insert(rebellion_core::world::Sector {
            dat_id: rebellion_core::ids::DatId::new(0x8000_0001),
            name: "Sector".into(),
            group: rebellion_core::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: Vec::new(),
        });
        let here = world.systems.keys().next().unwrap();
        world.systems[here].sector = sector;
        let mut others = Vec::new();
        for (raw, x) in [(0x9000_0002, 26), (0x9000_0003, 300)] {
            let mut system = world.systems[here].clone();
            system.dat_id = rebellion_core::ids::DatId::new(raw);
            system.x = x;
            system.y = 20;
            others.push(world.systems.insert(system));
        }
        world.sectors[sector].systems = vec![here, others[0], others[1]];
        world.capital_ship_classes.insert(CapitalShipClass {
            hyperdrive: 80,
            ..CapitalShipClass::default()
        });
        for location in [here, here, others[0]] {
            world.fleets.insert(Fleet {
                location,
                capital_ships: Vec::new(),
                fighters: Vec::new(),
                characters: Vec::new(),
                is_alliance: false,
                has_death_star: false,
            });
        }
        world
    }

    struct Applied {
        world: GameWorld,
        movement: MovementState,
        blockade: BlockadeState,
        map: GalaxyMapState,
        sectors: SectorWindowState,
        systems: SystemWindowState,
    }

    fn apply_to_fleet_world(request: FixtureRequest) -> Applied {
        let mut applied = Applied {
            world: fleet_world(),
            movement: MovementState::default(),
            blockade: BlockadeState::default(),
            map: GalaxyMapState::default(),
            sectors: SectorWindowState::default(),
            systems: SystemWindowState::default(),
        };
        apply(
            request,
            &mut applied.world,
            &mut GameMode::MainMenu,
            &mut MissionFaction::Alliance,
            &mut CockpitState::new(request.faction),
            &mut applied.map,
            &mut applied.movement,
            &mut ManufacturingState::default(),
            &mut EconomyState::default(),
            &mut MissionState::default(),
            &mut applied.blockade,
            &mut applied.sectors,
            &mut applied.systems,
        );
        applied
    }

    #[test]
    fn the_fleet_move_scenario_leaves_the_players_fleet_alone_and_movable_beside_its_target() {
        use rebellion_core::movement::{fleet_move_confirms, validate_fleet_dispatch};
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let request = request(Scenario::FleetMove, faction);
            let applied = apply_to_fleet_world(request);
            let world = &applied.world;
            let mut systems = world.systems.keys();
            let (here, target, elsewhere) = (
                systems.next().unwrap(),
                systems.next().unwrap(),
                systems.next().unwrap(),
            );
            let fleets: Vec<_> = world.fleets.keys().collect();
            let (fleet, other, away) = (fleets[0], fleets[1], fleets[2]);
            let is_alliance = faction == CockpitFaction::Alliance;

            assert_eq!(world.systems[here].fleets, [fleet], "{faction:?}");
            assert_eq!(world.fleets[other].location, elsewhere);
            assert_eq!(world.fleets[away].location, target);
            assert_eq!(world.fleets[fleet].is_alliance, is_alliance);
            assert_eq!(
                validate_fleet_dispatch(&applied.movement, world, fleet, target, is_alliance),
                Ok(())
            );
            assert!(!fleet_move_confirms(
                world,
                applied.blockade.blockaded_systems(),
                fleet,
                false
            ));

            let layout = CockpitState::new(faction).layout_for(640.0, 480.0);
            // The window's top-left corner, 5 pixels in from the galaxy
            // view's top-right, as in the targeting scenario.
            let galaxy = layout.galaxy;
            let corner = (
                galaxy.x + galaxy.width - SYSTEM_WINDOW_WIDTH - 5.0,
                galaxy.y + 5.0,
            );
            assert!(applied.systems.contains_screen_point(layout, corner));
            assert!(!applied
                .systems
                .contains_screen_point(layout, (corner.0 - 1.0, corner.1)));
            assert!(!applied
                .systems
                .contains_screen_point(layout, (corner.0, corner.1 - 1.0)));
            let setup = fleet_move_setup(
                request,
                world,
                &applied.map,
                &applied.sectors,
                &applied.systems,
            )
            .expect("the scenario reports its points");
            let item = (setup.fleet_item_x, setup.fleet_item_y);
            let planet = (setup.target_screen_x, setup.target_screen_y);
            assert!(applied.systems.contains_screen_point(layout, item));
            assert!(applied.sectors.contains_screen_point(layout, planet));
            assert!(!applied.systems.contains_screen_point(layout, planet));
            assert_eq!(
                (setup.primary_dat_id, setup.target_dat_id),
                (0x9000_0001, 0x9000_0002)
            );
            assert_eq!(setup.systems.len(), 3);

            let observed =
                fleet_move_observation(request, world, &applied.movement, false, &applied.map, &[])
                    .unwrap();
            assert_eq!(
                (
                    observed.in_transit,
                    observed.destination_dat_id,
                    observed.selected_system_dat_id,
                    observed.last_message,
                ),
                (false, None, None, None)
            );
        }
    }

    #[test]
    fn the_blockade_variant_leaves_the_fleet_blockading_an_enemy_system() {
        use rebellion_core::movement::{
            fleet_move_confirms, validate_fleet_destination, FleetDispatchError,
        };
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let applied = apply_to_fleet_world(request(Scenario::FleetMoveBlockade, faction));
            let world = &applied.world;
            let mut systems = world.systems.keys();
            let (here, target) = (systems.next().unwrap(), systems.next().unwrap());
            let fleet = world.fleets.keys().next().unwrap();
            let blockaded = applied.blockade.blockaded_systems();
            let is_alliance = faction == CockpitFaction::Alliance;

            assert!(blockaded.contains(&here), "{faction:?}");
            assert_eq!(
                validate_fleet_destination(
                    &applied.movement,
                    world,
                    blockaded,
                    fleet,
                    target,
                    is_alliance
                ),
                Err(FleetDispatchError::Blockaded)
            );
            // The enemy holds the system, so a menu Move does not ask.
            assert!(!fleet_move_confirms(world, blockaded, fleet, false));
        }
    }

    #[test]
    fn a_dispatched_fleet_reports_its_transit_and_the_last_message() {
        let request = request(Scenario::FleetMove, CockpitFaction::Alliance);
        let mut applied = apply_to_fleet_world(request);
        let fleet = applied.world.fleets.keys().next().unwrap();
        let target = applied.world.systems.keys().nth(1).unwrap();
        assert!(begin_fleet_transit(
            &mut applied.movement,
            &mut applied.world,
            fleet,
            target,
            5
        ));
        applied.map.selected_system = Some(target);
        let message = GameMessage::new(
            0,
            "Fleet move rejected",
            rebellion_render::MessageCategory::Event,
        );

        let observed = fleet_move_observation(
            request,
            &applied.world,
            &applied.movement,
            true,
            &applied.map,
            &[message],
        )
        .unwrap();

        assert_eq!(
            (
                observed.in_transit,
                observed.destination_dat_id,
                observed.confirmation_open,
                observed.selected_system_dat_id,
                observed.last_message.as_deref(),
            ),
            (
                true,
                Some(0x9000_0002),
                true,
                Some(0x9000_0002),
                Some("Fleet move rejected")
            )
        );
    }

    #[test]
    fn only_the_fleet_move_scenarios_report_fleet_moves() {
        let request = request(Scenario::MissionTargeting, CockpitFaction::Alliance);
        let applied = apply_to_fleet_world(request);

        assert_eq!(
            fleet_move_setup(
                request,
                &applied.world,
                &applied.map,
                &applied.sectors,
                &applied.systems
            ),
            None
        );
        assert_eq!(
            fleet_move_observation(
                request,
                &applied.world,
                &applied.movement,
                false,
                &applied.map,
                &[]
            ),
            None
        );
    }

    #[test]
    fn the_targeting_and_fleet_move_scenarios_report_the_object_menu() {
        let reporting: Vec<_> = (0..SCENARIO_COUNT)
            .filter_map(Scenario::decode)
            .filter(|scenario| scenario.reports_object_menu())
            .collect();

        assert_eq!(
            reporting,
            [
                Scenario::MissionTargeting,
                Scenario::FleetMove,
                Scenario::FleetMoveBlockade
            ]
        );
    }

    #[test]
    fn the_watch_passes_each_new_observation_once() {
        let mut watch = FleetMoveWatch::default();
        let first = FleetMoveObservation {
            status: "fleet-move",
            code: 0,
            in_transit: false,
            destination_dat_id: None,
            confirmation_open: false,
            selected_system_dat_id: None,
            last_message: None,
        };
        let second = FleetMoveObservation {
            in_transit: true,
            ..first.clone()
        };

        assert_eq!(watch.changed(Some(first.clone())), Some(&first));
        assert_eq!(watch.changed(Some(first.clone())), None);
        assert_eq!(watch.changed(None), None);
        assert_eq!(watch.changed(Some(second.clone())), Some(&second));
    }

    #[test]
    fn a_system_point_scales_its_offset_from_the_camera_by_the_zoom() {
        let world = diplomacy_world(Faction::Alliance);
        let mut system = world.systems.values().next().unwrap().clone();
        system.x = 110;
        system.y = 90;
        let map = GalaxyMapState {
            camera_x: 100.0,
            camera_y: 100.0,
            zoom: 2.0,
            ..GalaxyMapState::default()
        };
        let galaxy = CockpitState::new(CockpitFaction::Empire)
            .layout_for(640.0, 480.0)
            .galaxy;

        assert_eq!(
            screen_point(&system, &map, CockpitFaction::Empire),
            (
                galaxy.x + galaxy.width / 2.0 + 20.0,
                galaxy.y + galaxy.height / 2.0 - 20.0
            )
        );
    }

    #[test]
    fn packed_encyclopedia_request_is_a_distinct_feature_fixture() {
        let request = decode_request(0x0130).expect("packed encyclopedia scenario");

        assert_eq!(request.scenario, Scenario::PackedEncyclopedia);
        assert_eq!(request.faction, CockpitFaction::Alliance);
        assert!(request.is_packed_encyclopedia());
        assert_eq!(
            decode_request(0x0230).unwrap().faction,
            CockpitFaction::Empire
        );
        assert!(!decode_request(0x0127).unwrap().is_packed_encyclopedia());
        assert_eq!(decode_request(0x0000), None);
        assert_eq!(decode_request(0x0330), None);
        assert_eq!(decode_request(0x01_0130), None);
    }

    #[test]
    fn encyclopedia_fixture_scenarios_coexist_for_both_factions() {
        for (code, scenario, faction) in [
            (
                0x0129,
                Scenario::EncyclopediaIndexShell,
                CockpitFaction::Alliance,
            ),
            (
                0x0229,
                Scenario::EncyclopediaIndexShell,
                CockpitFaction::Empire,
            ),
            (
                0x012a,
                Scenario::EncyclopediaIndexCatalog,
                CockpitFaction::Alliance,
            ),
            (
                0x022a,
                Scenario::EncyclopediaIndexCatalog,
                CockpitFaction::Empire,
            ),
            (
                0x012b,
                Scenario::MissionDialogMission,
                CockpitFaction::Alliance,
            ),
            (
                0x022b,
                Scenario::MissionDialogMission,
                CockpitFaction::Empire,
            ),
            (
                0x012c,
                Scenario::MissionDialogAgents,
                CockpitFaction::Alliance,
            ),
            (
                0x022c,
                Scenario::MissionDialogAgents,
                CockpitFaction::Empire,
            ),
            (
                0x012d,
                Scenario::MissionTargeting,
                CockpitFaction::Alliance,
            ),
            (
                0x022d,
                Scenario::MissionTargeting,
                CockpitFaction::Empire,
            ),
            (
                0x012e,
                Scenario::PackedEncyclopedia,
                CockpitFaction::Alliance,
            ),
            (0x022e, Scenario::PackedEncyclopedia, CockpitFaction::Empire),
        ] {
            let request = decode_request(code).expect("encyclopedia fixture scenario");
            assert_eq!(request.scenario, scenario);
            assert_eq!(request.faction, faction);
            assert_eq!(
                request.is_packed_encyclopedia(),
                scenario == Scenario::PackedEncyclopedia
            );
        }
    }

    #[test]
    fn fixture_fingerprint_is_stable() {
        assert_eq!(
            fnv1a64(b"gid/alliance/popular-support"),
            0x432f_ad5e_fe03_453d
        );
    }
}
