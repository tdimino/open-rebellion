//! Test-only deterministic interface fixture bridge.
//!
//! This module is compiled for the dedicated WASM acceptance artifact and
//! native unit tests. Production builds contain no fixture bridge.

use rebellion_core::blockade::{BlockadeState, BlockadeSystem};
use rebellion_core::carriage::ProductRoom;
use rebellion_core::dat::{ExplorationStatus, Faction};
use rebellion_core::economy::EconomyState;
use rebellion_core::fog::FogState;
use rebellion_core::ids::{FleetKey, SystemKey, TroopKey};
use rebellion_core::manufacturing::ManufacturingState;
use rebellion_core::missions::{
    available_kinds, MissionFaction, MissionKind, MissionMember, MissionState,
};
use rebellion_core::movement::{
    begin_fleet_transit, fleet_speed, reconcile_fleet_orbits, MovementState,
};
use rebellion_core::tick::TickEvent;
use rebellion_core::troop_transport::{regiment_system, TroopTransportState};
use rebellion_core::uprising::UprisingState;
use rebellion_core::world::{ControlKind, GameWorld, ShipInstance, TroopUnit};
use rebellion_render::defenses_window::DEFENSES_WINDOW_WIDTH;
use rebellion_render::fleet_finder::{FinderControl, FinderMode, FinderTab, FleetFinderState};
use rebellion_render::fleet_window::{
    FleetWindowEntry, FleetWindowState, FleetWindowTab, FLEET_WINDOW_WIDTH,
};
use rebellion_render::game_speed::day_readout_rect;
use rebellion_render::mission_dialog::{MissionDialogPage, MissionDialogState};
use rebellion_render::object_menu::{ObjectMenuCommand, ObjectMenuState};
use rebellion_render::quadrant_icons::Quadrant;
use rebellion_render::{
    strategic_primary_controls, CockpitButton, CockpitFaction, CockpitState, GalaxyMapState,
    GameMessage, GidMode, SectorWindowState, SystemWindowState,
};
use rebellion_render::{DefensesPage, DefensesWindowState, MissionsTab, MissionsWindowState};
use serde::Serialize;

use crate::encyclopedia_surface::{CanonicalFixtureStart, FixtureStart};
use crate::GameMode;

const FIXTURE_ABSENT: u32 = 0;
/// How far right of the galaxy view's centre the targeting scenario puts its
/// target system, clear of the system window it opens on the left.
#[cfg(test)]
const SCENARIO_COUNT: u8 = 75;

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
    FleetLoad = 47,
    FleetLoadFull = 48,
    Quadrants = 49,
    RegimentUnloadRefused = 50,
    FleetJoin = 51,
    FleetFinder = 52,
    ProductionDestination = 53,
    BuildSelection = 54,
    EncyclopediaSurfaceMiddle = 55,
    EncyclopediaSurfaceFirst = 56,
    EncyclopediaSurfaceUnavailable = 57,
    EncyclopediaSurfaceIndex = 58,
    EncyclopediaCanonicalIndex = 59,
    EncyclopediaCanonicalFirst = 60,
    EncyclopediaCanonicalLast = 61,
    EncyclopediaCanonicalLongest = 62,
    EncyclopediaCanonicalUnavailable = 63,
    EncyclopediaCanonicalContextual = 64,
    WindowingContract = 65,
    OperationReport = 66,
    StrikeDeathStar = 67,
    UnfinishedShip = 68,
    ShipBesideUnfinished = 69,
    StrikeShielded = 70,
    AssaultCaptures = 71,
    AssaultRepelled = 72,
    StrikeDamagedSurvivor = 73,
    StrikeDamagedLoss = 74,
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
            47 => Self::FleetLoad,
            48 => Self::FleetLoadFull,
            49 => Self::Quadrants,
            50 => Self::RegimentUnloadRefused,
            51 => Self::FleetJoin,
            52 => Self::FleetFinder,
            53 => Self::ProductionDestination,
            54 => Self::BuildSelection,
            55 => Self::EncyclopediaSurfaceMiddle,
            56 => Self::EncyclopediaSurfaceFirst,
            57 => Self::EncyclopediaSurfaceUnavailable,
            58 => Self::EncyclopediaSurfaceIndex,
            59 => Self::EncyclopediaCanonicalIndex,
            60 => Self::EncyclopediaCanonicalFirst,
            61 => Self::EncyclopediaCanonicalLast,
            62 => Self::EncyclopediaCanonicalLongest,
            63 => Self::EncyclopediaCanonicalUnavailable,
            64 => Self::EncyclopediaCanonicalContextual,
            65 => Self::WindowingContract,
            66 => Self::OperationReport,
            67 => Self::StrikeDeathStar,
            68 => Self::UnfinishedShip,
            69 => Self::ShipBesideUnfinished,
            70 => Self::StrikeShielded,
            71 => Self::AssaultCaptures,
            72 => Self::AssaultRepelled,
            73 => Self::StrikeDamagedSurvivor,
            74 => Self::StrikeDamagedLoss,
            _ => return None,
        })
    }

    pub fn encyclopedia_fixture_start(self) -> Option<FixtureStart> {
        match self {
            Self::EncyclopediaSurfaceMiddle => Some(FixtureStart::MiddleTopic),
            Self::EncyclopediaSurfaceFirst => Some(FixtureStart::FirstTopic),
            Self::EncyclopediaSurfaceIndex => Some(FixtureStart::Index),
            _ => None,
        }
    }

    pub fn canonical_encyclopedia_fixture_start(self) -> Option<CanonicalFixtureStart> {
        match self {
            Self::EncyclopediaCanonicalIndex => Some(CanonicalFixtureStart::Index),
            Self::EncyclopediaCanonicalFirst => Some(CanonicalFixtureStart::FirstTopic),
            Self::EncyclopediaCanonicalLast => Some(CanonicalFixtureStart::LastTopic),
            Self::EncyclopediaCanonicalLongest => Some(CanonicalFixtureStart::LongestResolvedTopic),
            Self::EncyclopediaCanonicalContextual => {
                Some(CanonicalFixtureStart::ContextualFirstTopic)
            }
            _ => None,
        }
    }

    pub fn is_encyclopedia_surface(self) -> bool {
        self.encyclopedia_fixture_start().is_some()
            || self.canonical_encyclopedia_fixture_start().is_some()
    }

    fn moves_a_fleet(self) -> bool {
        matches!(
            self,
            Self::FleetMove | Self::FleetMoveBlockade | Self::StrikeDeathStar
        ) || self.stages_a_strike()
    }

    /// The Operation Report scenarios (EVT-03): the blockade variant with
    /// the fleet, the holder's garrison and its defences staged for one
    /// report ([`stage_strike`]).
    fn stages_a_strike(self) -> bool {
        matches!(
            self,
            Self::StrikeShielded
                | Self::AssaultCaptures
                | Self::AssaultRepelled
                | Self::StrikeDamagedSurvivor
                | Self::StrikeDamagedLoss
        )
    }

    /// The scenarios whose gate works the primary system's Fleet window.
    fn uses_the_fleet_window(self) -> bool {
        matches!(
            self,
            Self::FleetLoad
                | Self::FleetLoadFull
                | Self::RegimentUnloadRefused
                | Self::FleetJoin
                | Self::ProductionDestination
                | Self::UnfinishedShip
                | Self::ShipBesideUnfinished
        )
    }

    /// The scenarios whose gate opens the object pop-up menu.
    fn reports_object_menu(self) -> bool {
        self == Self::MissionTargeting
            || self == Self::BuildSelection
            || self.moves_a_fleet()
            || self.uses_the_fleet_window()
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

fn decode_request(code: u32) -> Option<FixtureRequest> {
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

pub fn requested() -> Option<FixtureRequest> {
    decode_request(unsafe { open_rebellion_interface_fixture_code() })
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
    fleets: &mut FleetWindowState,
    defenses: &mut DefensesWindowState,
    troop_transport: &mut TroopTransportState,
) {
    *game_mode = GameMode::Galaxy;
    *player_faction = match request.faction {
        CockpitFaction::Alliance => MissionFaction::Alliance,
        CockpitFaction::Empire => MissionFaction::Empire,
    };
    cockpit.faction = request.faction;
    cockpit.gid_mode = request.scenario.mode();
    cockpit.gid_ui.close_menu();

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
    // A fleet has the Death Star when it holds the family-0x18 ship
    // (`FUN_005003d0`), so the Death Star scenario gives the first fleet one.
    let death_star = (request.scenario == Scenario::DeathStarIntel).then(|| {
        world
            .capital_ship_classes
            .iter()
            .find(|(_, class)| class.is_death_star())
            .map(|(key, class)| (key, class.hull.cast_signed()))
            .expect("the Death Star scenario needs a family-0x18 capital ship class")
    });
    if let Some((fleet_key, fleet)) = world.fleets.iter_mut().next() {
        fleet.location = primary;
        fleet.is_alliance = player_is_alliance;
        if let Some((class, hull)) = death_star {
            fleet
                .capital_ships
                .push(ShipInstance::new(class, hull, player_is_alliance));
        }
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
    // The Build Selection gate: a shipyard, a training facility and a
    // construction yard of the player's at the primary system, whose
    // Manufacturing window opens on its overview.
    if request.scenario == Scenario::BuildSelection {
        for class in [0x2800_0001, 0x2900_0002, 0x2a00_0003] {
            let yard = world.manufacturing_facilities.insert(
                rebellion_core::world::ManufacturingFacilityInstance {
                    class_dat_id: rebellion_core::ids::DatId::new(class),
                    side: rebellion_core::dat::Faction::of_alliance(player_is_alliance),
                    is_shipyard: class == 0x2800_0001,
                },
            );
            world.systems[primary].manufacturing_facilities.push(yard);
        }
    }
    if matches!(
        request.scenario,
        Scenario::System | Scenario::BuildSelection
    ) {
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
            fleets,
            troop_transport,
            (
                primary,
                secondary,
                *system_keys.last().unwrap_or(&secondary),
            ),
        );
    }
    if request.scenario.uses_the_fleet_window() {
        place_loading_fleet(
            request,
            world,
            cockpit,
            movement,
            sectors,
            defenses,
            troop_transport,
            primary,
        );
    }
    if request.scenario == Scenario::Quadrants {
        place_quadrant_contents(request, world, movement, missions, sectors, primary);
    }
    if request.scenario == Scenario::FleetFinder {
        place_finder_fleets(request, world, movement, &system_keys);
        map.selected_system = Some(primary);
        sectors.open_for_system(world, primary, request.faction);
    }
    if request.scenario == Scenario::MissionTargeting {
        // The primary system's Defenses window at the galaxy view's right
        // edge holds the agent first on its personnel page; the second
        // system's planet in its sector window, in the first column on the
        // left, is the target (FUN_0045c830).
        let layout = cockpit.layout_for(640.0, 480.0);
        let galaxy = layout.galaxy;
        defenses.open(
            world,
            primary,
            (
                (galaxy.x + galaxy.width - DEFENSES_WINDOW_WIDTH) as i16 - 5,
                galaxy.y as i16 + 5,
            ),
            request.faction,
            layout,
        );
        sectors.open_for_system(world, secondary, request.faction);
    }

    let _ = manufacturing;
}

/// The Fleet Finder scenario: the player's first fleet at the primary
/// system, a copy of it at the third system (the player's), and a copy for
/// the other side at the primary system, which the player holds and so sees
/// into (`opposing_contents_visible`).
fn place_finder_fleets(
    request: FixtureRequest,
    world: &mut GameWorld,
    movement: &MovementState,
    system_keys: &[SystemKey],
) {
    let player_is_alliance = request.faction == CockpitFaction::Alliance;
    let (Some(&primary), Some(&third), Some(first)) = (
        system_keys.first(),
        system_keys.get(2),
        world.fleets.keys().next(),
    ) else {
        return;
    };
    let mut second = world.fleets[first].clone();
    second.location = third;
    second.is_alliance = player_is_alliance;
    let mut enemy = world.fleets[first].clone();
    enemy.location = primary;
    enemy.is_alliance = !player_is_alliance;
    world.fleets.insert(second);
    world.fleets.insert(enemy);
    reconcile_fleet_orbits(movement, world);
}

/// The fleet-move scenarios: the player's first fleet, alone in the primary
/// system and able to enter hyperspace, in that system's window at the galaxy
/// view's right edge, with the second system's sector window open as the
/// target. In the blockade variant the enemy holds the primary system, so the
/// player's fleet is alone there and blockades it
/// (`BlockadeSystem::system_is_blockaded`). The strike variant is the
/// blockade one with the Death Star aboard, for the Planetary Bombardment
/// rows.
#[allow(clippy::too_many_arguments)]
fn place_moving_fleet(
    request: FixtureRequest,
    world: &mut GameWorld,
    cockpit: &CockpitState,
    movement: &mut MovementState,
    blockade: &mut BlockadeState,
    sectors: &mut SectorWindowState,
    fleets: &mut FleetWindowState,
    troop_transport: &mut TroopTransportState,
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
    // The strike scenario's fleet holds the Death Star, the family-0x18
    // ship (`FUN_005003d0`), so Destroy System is listed enabled.
    if request.scenario == Scenario::StrikeDeathStar {
        let (class, hull) = world
            .capital_ship_classes
            .iter()
            .find(|(_, class)| class.is_death_star())
            .map(|(key, class)| (key, class.hull.cast_signed()))
            .expect("the strike scenario needs a family-0x18 capital ship class");
        world.fleets[player]
            .capital_ships
            .push(ShipInstance::new(class, hull, player_is_alliance));
    }
    world.fleets[player].location = primary;
    world.fleets[player].is_alliance = player_is_alliance;
    if request.scenario.stages_a_strike() {
        stage_strike(request, world, troop_transport, player, (primary, elsewhere));
    }
    reconcile_fleet_orbits(movement, world);
    if matches!(
        request.scenario,
        Scenario::FleetMoveBlockade | Scenario::StrikeDeathStar
    ) || request.scenario.stages_a_strike()
    {
        world.systems[primary].control = ControlKind::Controlled(if player_is_alliance {
            Faction::Empire
        } else {
            Faction::Alliance
        });
        let _ = BlockadeSystem::advance(blockade, world, &[TickEvent { tick: 1 }]);
    }
    let layout = cockpit.layout_for(640.0, 480.0);
    let galaxy = layout.galaxy;
    fleets.open(
        world,
        primary,
        (
            (galaxy.x + galaxy.width - FLEET_WINDOW_WIDTH) as i16 - 5,
            galaxy.y as i16 + 5,
        ),
        request.faction,
        layout,
    );
    sectors.open_for_system(world, secondary, request.faction);
}

/// The Operation Report scenarios (EVT-03), staged on the blockade variant.
/// The holder's regiments and defences at the system move elsewhere and the
/// fleet is emptied, so each report lists only what its scenario puts there:
///
/// - shielded: a bombarding ship under shield generators (family `0x24`)
///   whose shields reach the fleet's strength, so no hit lands
///   (`FUN_0058df60`; `ghidra/notes/bombardment.md`, "Effect phase");
/// - assault captures: a carrier with the player's regiments aboard against
///   an empty garrison (`FUN_00563390`);
/// - assault repelled: one regiment aboard against six of the holder's;
/// - damaged survivor: a bombarding ship at half its hull and no battery, so
///   it stays damaged (`+0x50` bit 9, `FUN_00501c00`);
/// - damaged loss: one ship at 1 hull carrying a bombing squadron (the
///   fleet's strength, `FUN_004fd6b0`) under one LNR battery, whose fire
///   has no other target and destroys it (`FUN_0058d540`).
///
/// port: test fixture values, not rules.
fn stage_strike(
    request: FixtureRequest,
    world: &mut GameWorld,
    troop_transport: &mut TroopTransportState,
    fleet: FleetKey,
    (primary, elsewhere): (SystemKey, SystemKey),
) {
    use rebellion_core::world::{CapitalShipClass, DefenseFacilityInstance, FighterEntry};
    let player_is_alliance = request.faction == CockpitFaction::Alliance;
    let holder = Faction::of_alliance(!player_is_alliance);
    let displaced = std::mem::take(&mut world.systems[primary].ground_units);
    world.systems[elsewhere].ground_units.extend(displaced);
    let defences = std::mem::take(&mut world.systems[primary].defense_facilities);
    world.systems[elsewhere].defense_facilities.extend(defences);
    let value = &mut world.fleets[fleet];
    value.characters.clear();
    value.fighters.clear();
    value.capital_ships.clear();

    let bomber = fixture_ship_class(
        world,
        player_is_alliance,
        |class| class.bombardment_modifier > 0,
        CapitalShipClass {
            name: "Bomber".into(),
            hull: 1000,
            hyperdrive: 80,
            bombardment_modifier: 2,
            ..CapitalShipClass::default()
        },
    );
    let carrier = fixture_ship_class(
        world,
        player_is_alliance,
        |class| class.troop_capacity >= 2,
        CapitalShipClass {
            name: "Transport".into(),
            hull: 100,
            hyperdrive: 80,
            troop_capacity: 2,
            ..CapitalShipClass::default()
        },
    );
    let board = |world: &mut GameWorld, class, hull| {
        world.fleets[fleet]
            .capital_ships
            .push(ShipInstance::new(class, hull, player_is_alliance));
        let index = world.fleets[fleet].capital_ships.len() - 1;
        rebellion_core::carriage::tag_ship(world, fleet, index).unwrap_or(0)
    };
    let hull = |world: &GameWorld, class| world.capital_ship_classes[class].hull.cast_signed();
    let regiments = |world: &mut GameWorld, alliance: bool, count: usize| {
        let class_dat_id = fixture_regiment_class(world, alliance);
        (0..count)
            .map(|_| {
                let key = world.troops.insert(TroopUnit {
                    class_dat_id,
                    is_alliance: alliance,
                });
                world.systems[primary].ground_units.push(key);
                key
            })
            .collect::<Vec<_>>()
    };
    let defence = |world: &mut GameWorld, class_dat_id| {
        let key = world.defense_facilities.insert(DefenseFacilityInstance {
            class_dat_id,
            side: holder,
        });
        world.systems[primary].defense_facilities.push(key);
    };
    match request.scenario {
        Scenario::StrikeShielded => {
            let full = hull(world, bomber);
            board(world, bomber, full);
            regiments(world, !player_is_alliance, 1);
            let strength = rebellion_core::bombardment::fleet_strength(world, fleet);
            let (shield, value) = fixture_defence_class(world, 0x24, |stats| stats.shield_strength);
            for _ in 0..strength.unsigned_abs().div_ceil(value).max(1) {
                defence(world, shield);
            }
        }
        Scenario::AssaultCaptures | Scenario::AssaultRepelled => {
            let full = hull(world, carrier);
            board(world, carrier, full);
            let aboard = if request.scenario == Scenario::AssaultCaptures {
                world.capital_ship_classes[carrier].troop_capacity as usize
            } else {
                regiments(world, !player_is_alliance, 6);
                1
            };
            let troops = regiments(world, player_is_alliance, aboard);
            let _ = troop_transport.load(world, &ProductRoom::default(), fleet, &troops);
        }
        Scenario::StrikeDamagedSurvivor => {
            let half = hull(world, bomber) / 2;
            board(world, bomber, half);
        }
        Scenario::StrikeDamagedLoss => {
            let tag = board(world, bomber, 1);
            let class = fixture_bombing_squadron(world, player_is_alliance);
            world.fleets[fleet].fighters.push(FighterEntry {
                class,
                count: 1,
                carrier: tag,
            });
            let (battery, _) = fixture_defence_class(world, 0x23, |stats| stats.attack_strength);
            defence(world, battery);
        }
        _ => {}
    }
}

/// The player's capital ship class `wanted` picks, or `template` added for
/// the player's side when the data has none.
fn fixture_ship_class(
    world: &mut GameWorld,
    player_is_alliance: bool,
    wanted: impl Fn(&rebellion_core::world::CapitalShipClass) -> bool,
    template: rebellion_core::world::CapitalShipClass,
) -> rebellion_core::ids::CapitalShipKey {
    world
        .capital_ship_classes
        .iter()
        .find(|(_, class)| {
            class.is_alliance == player_is_alliance
                && class.hyperdrive > 0
                && !class.is_death_star()
                && wanted(class)
        })
        .map(|(key, _)| key)
        .unwrap_or_else(|| {
            world
                .capital_ship_classes
                .insert(rebellion_core::world::CapitalShipClass {
                    is_alliance: player_is_alliance,
                    ..template
                })
        })
}

/// A regiment class of the side: the class of one of its regiments, else
/// the Army (Alliance) or Stormtrooper (Empire) regiment.
fn fixture_regiment_class(world: &GameWorld, alliance: bool) -> rebellion_core::ids::DatId {
    world
        .troops
        .values()
        .find(|troop| troop.is_alliance == alliance)
        .map_or_else(
            || rebellion_core::ids::DatId::new(if alliance { 0x1000_0001 } else { 0x1000_0006 }),
            |troop| troop.class_dat_id,
        )
}

/// The defence class of `family` with the largest `figure` (DEFFACSD), and
/// that figure; a class of that family with the figure 40 when the data has
/// none.
fn fixture_defence_class(
    world: &mut GameWorld,
    family: u8,
    figure: impl Fn(&rebellion_core::world::ClassStats) -> u32,
) -> (rebellion_core::ids::DatId, u32) {
    let found = world
        .buildable_classes
        .iter()
        .filter(|(id, class)| id.family() == family && figure(&class.stats) > 0)
        .max_by_key(|(id, class)| (figure(&class.stats), std::cmp::Reverse(id.raw())))
        .map(|(id, class)| (*id, figure(&class.stats)));
    found.unwrap_or_else(|| {
        let id = rebellion_core::ids::DatId::new(u32::from(family) << 24 | 1);
        let stats = rebellion_core::world::ClassStats {
            attack_strength: 40,
            shield_strength: 40,
            ..rebellion_core::world::ClassStats::default()
        };
        world.buildable_classes.insert(
            id,
            rebellion_core::world::BuildableClass {
                stats,
                ..rebellion_core::world::BuildableClass::default()
            },
        );
        (id, figure(&stats))
    })
}

/// A squadron class of the player's side with a bombard value (FIGHTSD
/// `bombardment_defense`), or one added when the data has none.
fn fixture_bombing_squadron(
    world: &mut GameWorld,
    player_is_alliance: bool,
) -> rebellion_core::ids::FighterKey {
    world
        .fighter_classes
        .iter()
        .find(|(_, class)| class.is_alliance == player_is_alliance && class.bombardment_defense > 0)
        .map(|(key, _)| key)
        .unwrap_or_else(|| {
            world
                .fighter_classes
                .insert(rebellion_core::world::FighterClass {
                    name: "Bomber Squadron".into(),
                    is_alliance: player_is_alliance,
                    is_empire: !player_is_alliance,
                    bombardment_defense: 2,
                    ..rebellion_core::world::FighterClass::default()
                })
        })
}

/// The regiment-loading target: another system in the primary system's
/// sector, so the sector window that shows the fleet icon shows it too.
fn loading_target(world: &GameWorld, primary: SystemKey) -> Option<SystemKey> {
    let sector = world.systems.get(primary)?.sector;
    world
        .sectors
        .get(sector)?
        .systems
        .iter()
        .copied()
        .find(|&key| key != primary)
}

/// The system the loading scenarios' Defenses window shows: the target in
/// the refused variant (the drop it refuses), the primary system otherwise.
fn defenses_window_subject(
    request: FixtureRequest,
    world: &GameWorld,
    primary: SystemKey,
) -> Option<SystemKey> {
    if request.scenario == Scenario::RegimentUnloadRefused {
        loading_target(world, primary)
    } else {
        Some(primary)
    }
}

/// The joining scenario's second fleet: the world's second, or a copy of
/// the first when the world has one.
fn joining_fleet(
    request: FixtureRequest,
    world: &mut GameWorld,
    first: FleetKey,
) -> Option<FleetKey> {
    if request.scenario != Scenario::FleetJoin {
        return None;
    }
    if let Some(second) = world.fleets.keys().nth(1) {
        return Some(second);
    }
    let copy = world.fleets[first].clone();
    Some(world.fleets.insert(copy))
}

/// The fixture's regiment: the first on the primary system's surface.
fn loading_regiment(world: &GameWorld, primary: SystemKey) -> Option<TroopKey> {
    world.systems.get(primary)?.ground_units.first().copied()
}

/// The regiment-loading scenarios: the player's first fleet, alone at the
/// player's primary system with one capital ship that carries regiments, and
/// one of the player's regiments first on the surface. The Defenses window
/// on the galaxy view's left shows its regiment page; the primary system's
/// sector window takes the right column (a window for another sector opens
/// first), so the Fleet window its icon opens lands clear of it. The
/// target system, in the same sector, is the player's and empty, so the
/// regiment lands there unopposed. In the full variant the fleet's room is
/// taken by regiments loaded beforehand (`FUN_00500b40`); in the refused
/// variant the target is the other side's and populated, the regiment
/// starts aboard, and the Defenses window shows the target, so a
/// drop on it is the refused move (the Fleet window covers the target's
/// planet in the sector window).
#[allow(clippy::too_many_arguments)]
fn place_loading_fleet(
    request: FixtureRequest,
    world: &mut GameWorld,
    cockpit: &CockpitState,
    movement: &mut MovementState,
    sectors: &mut SectorWindowState,
    defenses: &mut DefensesWindowState,
    troop_transport: &mut TroopTransportState,
    primary: SystemKey,
) {
    let player_is_alliance = request.faction == CockpitFaction::Alliance;
    let player = if player_is_alliance {
        Faction::Alliance
    } else {
        Faction::Empire
    };
    let (Some(target), Some(fleet)) = (loading_target(world, primary), world.fleets.keys().next())
    else {
        return;
    };
    let second = joining_fleet(request, world, fleet);
    let Some(elsewhere) = world
        .systems
        .keys()
        .find(|&key| world.systems[key].sector != world.systems[primary].sector)
    else {
        return;
    };
    for system in [primary, target] {
        world.systems[system].control = ControlKind::Controlled(player);
        world.systems[system].exploration_status = ExplorationStatus::Explored;
    }
    for (key, value) in &mut world.fleets {
        if key != fleet
            && Some(key) != second
            && (value.location == primary || value.location == target)
        {
            value.location = elsewhere;
        }
    }
    // One carrier of the player's side that can enter hyperspace.
    let carrier = world
        .capital_ship_classes
        .iter()
        .find(|(_, class)| {
            class.is_alliance == player_is_alliance
                && class.troop_capacity > 0
                && class.hyperdrive > 0
        })
        .map(|(key, _)| key)
        .unwrap_or_else(|| {
            world
                .capital_ship_classes
                .insert(rebellion_core::world::CapitalShipClass {
                    name: "Transport".into(),
                    is_alliance: player_is_alliance,
                    hull: 100,
                    hyperdrive: 80,
                    troop_capacity: 2,
                    ..Default::default()
                })
        });
    let value = &mut world.fleets[fleet];
    value.location = primary;
    value.is_alliance = player_is_alliance;
    value.characters.clear();
    value.fighters.clear();
    value.capital_ships = vec![ShipInstance::new(carrier, 100, player_is_alliance)];
    // The joining variant: the first fleet holds two ships and a second
    // fleet of the player's, with one, orbits beside it.
    if let Some(second) = second {
        value
            .capital_ships
            .push(ShipInstance::new(carrier, 100, player_is_alliance));
        let other = &mut world.fleets[second];
        other.location = primary;
        other.is_alliance = player_is_alliance;
        other.characters.clear();
        other.fighters.clear();
        other.capital_ships = vec![ShipInstance::new(carrier, 100, player_is_alliance)];
    }
    reconcile_fleet_orbits(movement, world);

    let class_dat_id = world
        .troops
        .values()
        .find(|troop| troop.is_alliance == player_is_alliance)
        .map_or_else(
            || {
                rebellion_core::ids::DatId::new(if player_is_alliance {
                    0x1000_0001
                } else {
                    0x1000_0006
                })
            },
            |troop| troop.class_dat_id,
        );
    let regiment = || TroopUnit {
        class_dat_id,
        is_alliance: player_is_alliance,
    };
    for system in [primary, target] {
        let displaced = std::mem::take(&mut world.systems[system].ground_units);
        world.systems[elsewhere].ground_units.extend(displaced);
        world.systems[system].special_forces.clear();
    }
    let troop = world.troops.insert(regiment());
    world.systems[primary].ground_units.push(troop);
    // The refused variant: the target is the other side's and populated,
    // which FUN_0053d430 refuses a regiment (1/0x28).
    if request.scenario == Scenario::RegimentUnloadRefused {
        let enemy = if player_is_alliance {
            Faction::Empire
        } else {
            Faction::Alliance
        };
        world.systems[target].control = ControlKind::Controlled(enemy);
        world.systems[target].is_populated = true;
        let _ = troop_transport.load(world, &ProductRoom::default(), fleet, &[troop]);
    }
    // The unfinished variant: the fleet's ships are not yet completed
    // (+0x50 bit 2 clear), as a Build order leaves its ship
    // (FUN_004f7d50(.., 1, 0)), for the under-construction mark
    // (FUN_0042c3b0.c:412-431) and the ship's menu (FUN_004f9860.c:45-50).
    if request.scenario == Scenario::UnfinishedShip {
        for ship in &mut world.fleets[fleet].capital_ships {
            ship.completed = false;
        }
    }
    // The mixed variant: a completed ship, then an unfinished one beside
    // it, as the route-1 Empire save's Fleet 1 held a Galleon and the
    // unfinished Death Star; the fleet's Scrap passes at the completed ship
    // (FUN_004ffa70.c:27-62).
    if request.scenario == Scenario::ShipBesideUnfinished {
        let mut unfinished = ShipInstance::new(carrier, 100, player_is_alliance);
        unfinished.completed = false;
        world.fleets[fleet].capital_ships.push(unfinished);
    }
    // The destination variant: a shipyard of the player's, so the facility
    // icon's menu offers Destination (0x214, FUN_00512700 kind 4).
    if request.scenario == Scenario::ProductionDestination {
        let shipyard = world.manufacturing_facilities.insert(
            rebellion_core::world::ManufacturingFacilityInstance {
                class_dat_id: rebellion_core::ids::DatId::new(0x2800_0001),
                side: rebellion_core::dat::Faction::of_alliance(player_is_alliance),
                is_shipyard: true,
            },
        );
        world.systems[primary]
            .manufacturing_facilities
            .push(shipyard);
    }
    if request.scenario == Scenario::FleetLoadFull {
        let room = world.capital_ship_classes[carrier].troop_capacity;
        let aboard: Vec<_> = (0..room)
            .map(|_| {
                let key = world.troops.insert(regiment());
                world.systems[primary].ground_units.push(key);
                key
            })
            .collect();
        let _ = troop_transport.load(world, &ProductRoom::default(), fleet, &aboard);
    }

    let layout = cockpit.layout_for(640.0, 480.0);
    let galaxy = layout.galaxy;
    let subject = defenses_window_subject(request, world, primary).unwrap_or(primary);
    defenses.open(
        world,
        subject,
        (galaxy.x as i16 + 5, galaxy.y as i16 + 5),
        request.faction,
        layout,
    );
    defenses.show_page(subject, DefensesPage::Regiments);
    sectors.open_for_system(world, elsewhere, request.faction);
    sectors.open_for_system(world, primary, request.faction);
}

/// The quadrant-icon scenario. The primary system (the player's) and a
/// second system in its sector (the other side's) are emptied, then stocked
/// so each icon the gate checks has one cause:
///
/// - primary: a player mine (kind 4), a player regiment and KDY-150
///   (kind 8, and rows on the Defenses window's regiment and battery
///   pages), the player's first fleet when it has one (kind `0x10`), and
///   one agent of each side on a Diplomacy mission there (kind `0x40`, the
///   other side's art, `FUN_004a1f60`);
/// - second: a player regiment (kind 8, the system's side art) and a player
///   agent on a mission there (kind `0x40`, the player's art).
///
/// Only the primary system's sector window is open.
fn place_quadrant_contents(
    request: FixtureRequest,
    world: &mut GameWorld,
    movement: &mut MovementState,
    missions: &mut MissionState,
    sectors: &mut SectorWindowState,
    primary: SystemKey,
) {
    let player_is_alliance = request.faction == CockpitFaction::Alliance;
    let (player, enemy) = if player_is_alliance {
        (Faction::Alliance, Faction::Empire)
    } else {
        (Faction::Empire, Faction::Alliance)
    };
    let Some(second) = loading_target(world, primary) else {
        return;
    };
    let Some(elsewhere) = world
        .systems
        .keys()
        .find(|&key| world.systems[key].sector != world.systems[primary].sector)
    else {
        return;
    };
    let fleet = world
        .fleets
        .iter()
        .find(|(_, value)| value.is_alliance == player_is_alliance)
        .map(|(key, _)| key);
    world.systems[primary].control = ControlKind::Controlled(player);
    world.systems[second].control = ControlKind::Controlled(enemy);
    for system in [primary, second] {
        let value = &mut world.systems[system];
        value.exploration_status = ExplorationStatus::Explored;
        value.defense_facilities.clear();
        value.manufacturing_facilities.clear();
        value.production_facilities.clear();
        let troops = std::mem::take(&mut value.ground_units);
        let forces = std::mem::take(&mut value.special_forces);
        world.systems[elsewhere].ground_units.extend(troops);
        world.systems[elsewhere].special_forces.extend(forces);
    }
    for (key, value) in &mut world.fleets {
        if Some(key) != fleet && (value.location == primary || value.location == second) {
            value.location = elsewhere;
        }
    }
    if let Some(fleet) = fleet {
        world.fleets[fleet].location = primary;
    }
    reconcile_fleet_orbits(movement, world);
    for character in world.characters.values_mut() {
        if character.current_system == Some(primary) || character.current_system == Some(second) {
            character.current_system = Some(elsewhere);
        }
    }

    let mine =
        world
            .production_facilities
            .insert(rebellion_core::world::ProductionFacilityInstance {
                class_dat_id: rebellion_core::ids::DatId::new(0x2c00_0001),
                side: rebellion_core::dat::Faction::of_alliance(player_is_alliance),
                is_mine: true,
            });
    world.systems[primary].production_facilities.push(mine);
    let battery = world
        .defense_facilities
        .insert(rebellion_core::world::DefenseFacilityInstance {
            class_dat_id: rebellion_core::ids::DatId::new(0x2200_0001),
            side: rebellion_core::dat::Faction::of_alliance(player_is_alliance),
        });
    world.systems[primary].defense_facilities.push(battery);
    let class_dat_id = rebellion_core::ids::DatId::new(if player_is_alliance {
        0x1000_0001
    } else {
        0x1000_0006
    });
    for system in [primary, second] {
        let troop = world.troops.insert(TroopUnit {
            class_dat_id,
            is_alliance: player_is_alliance,
        });
        world.systems[system].ground_units.push(troop);
    }

    let aboard: std::collections::HashSet<_> = world
        .fleets
        .values()
        .flat_map(|value| value.characters.iter().copied())
        .collect();
    let agents = |alliance: bool| {
        world
            .characters
            .iter()
            .filter(|(key, character)| {
                character.is_alliance == alliance
                    && character.is_empire != alliance
                    && !character.is_killed
                    && !character.is_captive
                    && !aboard.contains(key)
            })
            .map(|(key, _)| key)
            .collect::<Vec<_>>()
            .into_iter()
    };
    let mut own = agents(player_is_alliance);
    let mut theirs = agents(!player_is_alliance);
    let placements = [
        (own.next(), primary, player_is_alliance),
        (theirs.next(), primary, !player_is_alliance),
        (own.next(), second, player_is_alliance),
    ];
    for (agent, system, alliance) in placements {
        let Some(agent) = agent else {
            continue;
        };
        world.characters[agent].current_system = Some(system);
        missions.dispatch(rebellion_core::missions::MissionRequest::single(
            MissionKind::Diplomacy,
            if alliance {
                MissionFaction::Alliance
            } else {
                MissionFaction::Empire
            },
            agent,
            system,
            None,
            0,
        ));
        rebellion_core::missions::set_on_mission(world, MissionMember::Character(agent), true);
    }
    sectors.open_for_system(world, primary, request.faction);
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
}

pub fn emit_ready(request: FixtureRequest, world: &GameWorld, map: &GalaxyMapState) {
    let faction = match request.faction {
        CockpitFaction::Alliance => "alliance",
        CockpitFaction::Empire => "empire",
    };
    let fingerprint_input = serde_json::to_vec(&(request.code, world, map.selected_system))
        .expect("serialize deterministic interface fixture state");
    let aperture = CockpitState::new(request.faction)
        .layout_for(640.0, 480.0)
        .galaxy;
    let probe = world
        .systems
        .iter()
        .map(|(_, system)| {
            let (x, y) = screen_point(system, request.faction);
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
    };
    let bytes = serde_json::to_vec(&report).expect("serialize interface fixture report");
    unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
}

/// Where `system` is drawn in the 640 by 480 fixture canvas.
fn screen_point(system: &rebellion_core::world::System, faction: CockpitFaction) -> (f32, f32) {
    let aperture = CockpitState::new(faction).layout_for(640.0, 480.0).galaxy;
    rebellion_render::galaxy_camera(
        (aperture.x, aperture.y, aperture.width, aperture.height),
        1.0,
        faction,
    )
    .to_screen(f32::from(system.x), f32::from(system.y))
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
    create_fleet_row: Option<usize>,
    rename_row: Option<usize>,
    destination_row: Option<usize>,
    status_window_row: Option<usize>,
    build_row: Option<usize>,
    stop_row: Option<usize>,
    bombardment_row: Option<usize>,
    assault_row: Option<usize>,
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
    let key = if request.scenario.uses_the_fleet_window() {
        loading_target(world, world.systems.keys().next()?)?
    } else {
        world.systems.keys().nth(1)?
    };
    let target = world.systems.get(key)?;
    let layout = CockpitState::new(request.faction).layout_for(640.0, 480.0);
    // The Build Selection gate targets nothing; its menu reports anyway.
    let planet = match sectors.planet_screen_rect(world, layout, key) {
        Some(rect) => rect.center(),
        None if request.scenario == Scenario::BuildSelection => egui_macroquad::egui::Pos2::ZERO,
        None => return None,
    };
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
        create_fleet_row: menu.row_of(ObjectMenuCommand::CreateFleet),
        rename_row: menu.row_of(ObjectMenuCommand::Rename),
        destination_row: menu.row_of(ObjectMenuCommand::Destination),
        status_window_row: menu.row_of(ObjectMenuCommand::Status),
        build_row: menu.row_of(ObjectMenuCommand::Build),
        stop_row: menu.row_of(ObjectMenuCommand::Stop),
        bombardment_row: menu.row_of(ObjectMenuCommand::Bombardment),
        assault_row: menu.row_of(ObjectMenuCommand::Assault),
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

/// One Manufacturing window band as the Build Selection gate sees it.
#[derive(Debug, Serialize, PartialEq)]
struct FixtureBand {
    area: &'static str,
    left: f32,
    top: f32,
    width: f32,
    height: f32,
    queued: usize,
    product: String,
}

/// The primary system's bands and the Build Selection window, each frame.
#[derive(Debug, Serialize, PartialEq)]
struct FixtureProduction {
    status: &'static str,
    code: u32,
    bands: Vec<FixtureBand>,
    build_selection_open: bool,
    build_selection_left: f32,
    build_selection_top: f32,
}

fn production_report(
    request: FixtureRequest,
    world: &GameWorld,
    manufacturing: &ManufacturingState,
    systems: &SystemWindowState,
    build_selection: &rebellion_render::build_selection::BuildSelectionState,
) -> Option<FixtureProduction> {
    let primary = world.systems.keys().next()?;
    let layout = CockpitState::new(request.faction).layout_for(640.0, 480.0);
    let bands = rebellion_core::manufacturing::ProductionArea::ALL
        .iter()
        .filter_map(|&area| {
            let rect = systems.band_screen_rect(layout, primary, area)?;
            let queue = manufacturing.queue(primary, area);
            Some(FixtureBand {
                area: match area {
                    rebellion_core::manufacturing::ProductionArea::Shipyard => "ships",
                    rebellion_core::manufacturing::ProductionArea::TrainingFacility => "troops",
                    rebellion_core::manufacturing::ProductionArea::ConstructionYard => "facilities",
                },
                left: rect.min.x,
                top: rect.min.y,
                width: rect.width(),
                height: rect.height(),
                queued: queue.map_or(0, rebellion_core::manufacturing::ProductionQueue::len),
                product: queue
                    .and_then(rebellion_core::manufacturing::ProductionQueue::active)
                    .map(|item| rebellion_render::manufacturing_window::product(world, item.kind).0)
                    .unwrap_or_default(),
            })
        })
        .collect();
    let window = rebellion_render::build_selection::window_rect(layout);
    Some(FixtureProduction {
        status: "production",
        code: request.code,
        bands,
        build_selection_open: build_selection.is_open(),
        build_selection_left: window.min.x,
        build_selection_top: window.min.y,
    })
}

pub fn emit_production(
    request: FixtureRequest,
    world: &GameWorld,
    manufacturing: &ManufacturingState,
    systems: &SystemWindowState,
    build_selection: &rebellion_render::build_selection::BuildSelectionState,
) {
    if request.scenario != Scenario::BuildSelection {
        return;
    }
    let Some(report) = production_report(request, world, manufacturing, systems, build_selection)
    else {
        return;
    };
    let bytes = serde_json::to_vec(&report).expect("serialize the production report");
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
    sectors: &SectorWindowState,
    fleets: &FleetWindowState,
) -> Option<FixtureFleetMoveSetup> {
    if !request.scenario.moves_a_fleet() {
        return None;
    }
    let layout = CockpitState::new(request.faction).layout_for(640.0, 480.0);
    let mut keys = world.systems.keys();
    let (primary, target) = (keys.next()?, keys.next()?);
    // The fleet is the only one at its system: the Fleet window's first entry.
    let item = fleets.entry_screen_rect(layout, primary, 0)?.center();
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
                let (x, y) = screen_point(system, request.faction);
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
    sectors: &SectorWindowState,
    fleets: &FleetWindowState,
) {
    let Some(report) = fleet_move_setup(request, world, sectors, fleets) else {
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

fn screen_center(rect: egui_macroquad::egui::Rect) -> (f32, f32) {
    (rect.center().x, rect.center().y)
}

/// One quadrant overlay's screen rect, as `[left, top, width, height]`.
#[derive(Debug, Serialize, PartialEq)]
struct FixtureQuadrant {
    system_dat_id: u32,
    quadrant: &'static str,
    rect: [f32; 4],
}

/// Where the quadrant gate looks: every quadrant overlay rect of the primary
/// system and the second system, and the layout's scale.
#[derive(Debug, Serialize, PartialEq)]
struct FixtureQuadrantSetup {
    status: &'static str,
    code: u32,
    primary_dat_id: u32,
    second_dat_id: u32,
    scale: f32,
    quadrants: Vec<FixtureQuadrant>,
}

fn quadrant_setup(
    request: FixtureRequest,
    world: &GameWorld,
    sectors: &SectorWindowState,
) -> Option<FixtureQuadrantSetup> {
    if request.scenario != Scenario::Quadrants {
        return None;
    }
    let layout = CockpitState::new(request.faction).layout_for(640.0, 480.0);
    let primary = world.systems.keys().next()?;
    let second = loading_target(world, primary)?;
    let mut quadrants = Vec::new();
    for system in [primary, second] {
        for (quadrant, name) in [
            (Quadrant::System, "system"),
            (Quadrant::Defenses, "defenses"),
            (Quadrant::Fleets, "fleets"),
            (Quadrant::Missions, "missions"),
        ] {
            let rect = sectors.quadrant_screen_rect(world, layout, system, quadrant)?;
            quadrants.push(FixtureQuadrant {
                system_dat_id: world.systems[system].dat_id.raw(),
                quadrant: name,
                rect: [rect.min.x, rect.min.y, rect.width(), rect.height()],
            });
        }
    }
    Some(FixtureQuadrantSetup {
        status: "quadrant-setup",
        code: request.code,
        primary_dat_id: world.systems[primary].dat_id.raw(),
        second_dat_id: world.systems[second].dat_id.raw(),
        scale: layout.scale,
        quadrants,
    })
}

pub fn emit_quadrant_setup(
    request: FixtureRequest,
    world: &GameWorld,
    sectors: &SectorWindowState,
) {
    let Some(report) = quadrant_setup(request, world, sectors) else {
        return;
    };
    let bytes = serde_json::to_vec(&report).expect("serialize the quadrant setup");
    unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
}

/// One open Defenses window, for the quadrant gate: what it shows and where
/// its tab buttons and listed cells are, as `[left, top, width, height]`.
#[derive(Debug, Clone, Serialize, PartialEq)]
struct FixtureDefensesWindow {
    system_dat_id: u32,
    origin: (i16, i16),
    side: u8,
    page: &'static str,
    counts: [usize; 5],
    rows: Vec<String>,
    selected: Option<usize>,
    garrison: Option<String>,
    tabs: Vec<(&'static str, [f32; 4])>,
    cells: Vec<[f32; 4]>,
}

/// One open Missions window, for the quadrant gate: its rows (name, side,
/// GOKRES mini), selection, tab, members and target, and the rects of its
/// mission rows, tabs and member rows, as `[left, top, width, height]`.
#[derive(Debug, Clone, Serialize, PartialEq)]
struct FixtureMissionsWindow {
    system_dat_id: u32,
    origin: (i16, i16),
    side: u8,
    rows: Vec<(String, u8, u32)>,
    selected: Option<usize>,
    tab: &'static str,
    tab_side: Option<u8>,
    members: Vec<String>,
    target: Option<String>,
    row_rects: Vec<[f32; 4]>,
    tabs: Vec<(&'static str, [f32; 4])>,
    member_rects: Vec<[f32; 4]>,
}

/// The open System, Defenses and Missions windows and the rail, for the
/// quadrant gate.
#[derive(Debug, Clone, Serialize, PartialEq)]
struct QuadrantObservation {
    status: &'static str,
    code: u32,
    system_windows: Vec<(u32, (i16, i16))>,
    defenses_windows: Vec<FixtureDefensesWindow>,
    missions_windows: Vec<FixtureMissionsWindow>,
    rail: Vec<(u32, &'static str, [f32; 4])>,
}

/// The windows and pages the quadrant observation reads.
pub struct QuadrantWindows<'a> {
    pub systems: &'a SystemWindowState,
    pub defenses: &'a DefensesWindowState,
    pub missions_windows: &'a MissionsWindowState,
    pub fog: &'a FogState,
    pub missions: &'a MissionState,
    pub economy: &'a EconomyState,
}

fn defenses_page_name(page: DefensesPage) -> &'static str {
    match page {
        DefensesPage::Personnel => "personnel",
        DefensesPage::Regiments => "regiments",
        DefensesPage::Squadrons => "squadrons",
        DefensesPage::Shields => "shields",
        DefensesPage::Batteries => "batteries",
    }
}

fn missions_tab_name(tab: MissionsTab) -> &'static str {
    match tab {
        MissionsTab::Agents => "agents",
        MissionsTab::Decoys => "decoys",
    }
}

fn rect_array(rect: egui_macroquad::egui::Rect) -> [f32; 4] {
    [rect.min.x, rect.min.y, rect.width(), rect.height()]
}

/// Emits the open windows and the rail each time they change.
#[derive(Debug, Default)]
pub struct QuadrantWatch {
    last: Option<QuadrantObservation>,
}

impl QuadrantWatch {
    fn next(
        &mut self,
        request: FixtureRequest,
        world: &GameWorld,
        windows: &QuadrantWindows<'_>,
    ) -> Option<QuadrantObservation> {
        if request.scenario != Scenario::Quadrants {
            return None;
        }
        let layout = CockpitState::new(request.faction).layout_for(640.0, 480.0);
        let defenses_windows = world
            .systems
            .iter()
            .filter_map(|(system, value)| {
                let report =
                    windows
                        .defenses
                        .report(world, windows.fog, windows.missions, system)?;
                Some(FixtureDefensesWindow {
                    system_dat_id: value.dat_id.raw(),
                    origin: report.origin,
                    side: report.side,
                    page: defenses_page_name(report.page),
                    counts: report.counts,
                    cells: (0..report.rows.len().min(9))
                        .filter_map(|index| {
                            windows.defenses.cell_screen_rect(layout, system, index)
                        })
                        .map(rect_array)
                        .collect(),
                    rows: report.rows,
                    selected: report.selected,
                    garrison: report.garrison,
                    tabs: DefensesPage::ALL
                        .into_iter()
                        .filter_map(|page| {
                            let rect = windows.defenses.tab_screen_rect(layout, system, page)?;
                            Some((defenses_page_name(page), rect_array(rect)))
                        })
                        .collect(),
                })
            })
            .collect();
        let missions_windows = world
            .systems
            .iter()
            .filter_map(|(system, value)| {
                let shown = windows.missions_windows;
                let report = shown.report(world, windows.fog, windows.missions, system)?;
                Some(FixtureMissionsWindow {
                    system_dat_id: value.dat_id.raw(),
                    origin: report.origin,
                    side: report.side,
                    row_rects: (0..report.rows.len())
                        .filter_map(|index| shown.mission_row_screen_rect(layout, system, index))
                        .map(rect_array)
                        .collect(),
                    member_rects: (0..report.members.len())
                        .filter_map(|index| shown.member_row_screen_rect(layout, system, index))
                        .map(rect_array)
                        .collect(),
                    tabs: MissionsTab::ALL
                        .into_iter()
                        .filter_map(|tab| {
                            let rect = shown.tab_screen_rect(layout, system, tab)?;
                            Some((missions_tab_name(tab), rect_array(rect)))
                        })
                        .collect(),
                    rows: report.rows,
                    selected: report.selected,
                    tab: missions_tab_name(report.tab),
                    tab_side: report.tab_side,
                    members: report.members,
                    target: report.target,
                })
            })
            .collect();
        let now = QuadrantObservation {
            status: "quadrant-observation",
            code: request.code,
            system_windows: windows
                .systems
                .open_windows()
                .filter_map(|(system, at)| Some((world.systems.get(system)?.dat_id.raw(), at)))
                .collect(),
            defenses_windows,
            missions_windows,
            rail: windows
                .systems
                .rail_entries()
                .enumerate()
                .filter_map(|(index, (system, kind))| {
                    let rect = windows.systems.rail_slot_screen_rect(layout, index)?;
                    Some((
                        world.systems.get(system)?.dat_id.raw(),
                        kind,
                        rect_array(rect),
                    ))
                })
                .collect(),
        };
        if self.last.as_ref() == Some(&now) {
            return None;
        }
        self.last = Some(now.clone());
        Some(now)
    }

    pub fn observe(
        &mut self,
        request: FixtureRequest,
        world: &GameWorld,
        windows: &QuadrantWindows<'_>,
    ) {
        let Some(report) = self.next(request, world, windows) else {
            return;
        };
        let bytes = serde_json::to_vec(&report).expect("serialize the quadrant observation");
        unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
    }
}

/// Where the fleet-load gate presses: the fleet icon in the primary system's
/// sector window (and the target's name, for the arrival message), the
/// regiment's cell (none when it starts aboard) and the
/// Fleets tab in the system window, the primary and target planets, and the
/// day readout (the Game Speed control).
#[derive(Debug, Serialize, PartialEq)]
struct FixtureFleetLoadSetup {
    status: &'static str,
    code: u32,
    primary_dat_id: u32,
    target_dat_id: u32,
    target_name: String,
    icon: (f32, f32),
    /// The primary system's facility (System quadrant) icon, when shown.
    system_icon: Option<(f32, f32)>,
    /// The regiment's cell on the Defenses window's regiment page.
    troop_item: Option<(f32, f32)>,
    /// The Defenses window's centre: a move released anywhere on it targets
    /// its system (`+0x70`, `FUN_004aa470`).
    defenses_window: (f32, f32),
    primary_planet: (f32, f32),
    target_planet: (f32, f32),
    day_readout: (f32, f32),
    capacity: Option<u32>,
}

fn fleet_load_setup(
    request: FixtureRequest,
    world: &GameWorld,
    sectors: &SectorWindowState,
    defenses: &DefensesWindowState,
) -> Option<FixtureFleetLoadSetup> {
    if !request.scenario.uses_the_fleet_window() {
        return None;
    }
    let layout = CockpitState::new(request.faction).layout_for(640.0, 480.0);
    let primary = world.systems.keys().next()?;
    let target = loading_target(world, primary)?;
    let fleet = world.fleets.keys().next()?;
    let subject = defenses_window_subject(request, world, primary)?;
    let day = day_readout_rect(request.faction);
    Some(FixtureFleetLoadSetup {
        status: "fleet-load-setup",
        code: request.code,
        primary_dat_id: world.systems[primary].dat_id.raw(),
        target_dat_id: world.systems[target].dat_id.raw(),
        target_name: world.systems[target].name.clone(),
        icon: screen_center(sectors.fleet_icon_screen_rect(world, layout, primary)?),
        system_icon: sectors
            .quadrant_screen_rect(world, layout, primary, Quadrant::System)
            .map(screen_center),
        troop_item: defenses
            .cell_screen_rect(layout, primary, 0)
            .map(screen_center),
        defenses_window: screen_center(defenses.screen_rect(layout, subject)?),
        primary_planet: screen_center(sectors.planet_screen_rect(world, layout, primary)?),
        target_planet: screen_center(sectors.planet_screen_rect(world, layout, target)?),
        day_readout: (
            layout.canvas.x + (day.x + day.width / 2.0) * layout.scale,
            layout.canvas.y + (day.y + day.height / 2.0) * layout.scale,
        ),
        capacity: TroopTransportState::fleet_capacity(world, fleet),
    })
}

pub fn emit_fleet_load_setup(
    request: FixtureRequest,
    world: &GameWorld,
    sectors: &SectorWindowState,
    defenses: &DefensesWindowState,
) {
    let Some(report) = fleet_load_setup(request, world, sectors, defenses) else {
        return;
    };
    let bytes = serde_json::to_vec(&report).expect("serialize the fleet load setup");
    unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
}

/// One fleet the primary system's Fleet window lists: its living capital
/// ships and where its entry lies.
#[derive(Debug, Clone, Serialize, PartialEq)]
struct FleetEntryObservation {
    ships: usize,
    entry: (f32, f32),
}

/// What the fleet-load gate checks after each step: the primary system's
/// Fleet window (its list, tabs and contents, and where its first entry,
/// each listed fleet, the Troops tab and the first right-list item lie),
/// where the regiment is and whether it travels on its own, the fleet's
/// cargo and hold, its transit, whether the move confirmation is open, the
/// last message (a refusal's text), and the last regiment line.
#[derive(Debug, Clone, Serialize, PartialEq)]
struct FleetLoadObservation {
    status: &'static str,
    code: u32,
    window_open: bool,
    origin: Option<(i16, i16)>,
    entries: usize,
    selected: Option<&'static str>,
    tab: Option<String>,
    enabled: [bool; 4],
    items: Vec<String>,
    counts: Option<(u32, u32)>,
    fleet_entry: Option<(f32, f32)>,
    fleets: Vec<FleetEntryObservation>,
    troops_tab: Option<(f32, f32)>,
    first_item: Option<(f32, f32)>,
    aboard: bool,
    cargo: usize,
    troop_system_dat_id: Option<u32>,
    in_transit: bool,
    regiment_travelling: bool,
    confirmation_open: bool,
    last_message: Option<String>,
    last_regiment_message: Option<String>,
}

#[allow(clippy::too_many_arguments)]
fn fleet_load_observation(
    request: FixtureRequest,
    regiment: TroopKey,
    world: &GameWorld,
    fog: &FogState,
    movement: &MovementState,
    transport: &TroopTransportState,
    fleets: &FleetWindowState,
    confirmation_open: bool,
    messages: &[GameMessage],
) -> Option<FleetLoadObservation> {
    let layout = CockpitState::new(request.faction).layout_for(640.0, 480.0);
    let primary = world.systems.keys().next()?;
    let fleet = world.fleets.keys().next()?;
    let report = fleets.report(world, movement, fog, transport, primary);
    Some(FleetLoadObservation {
        status: "fleet-load",
        code: request.code,
        window_open: report.is_some(),
        origin: report.as_ref().map(|report| report.origin),
        entries: report.as_ref().map_or(0, |report| report.entries),
        selected: report
            .as_ref()
            .and_then(|report| report.selected)
            .map(|entry| match entry {
                FleetWindowEntry::Fleet(_) => "fleet",
                FleetWindowEntry::Ship { .. } => "ship",
            }),
        tab: report.as_ref().map(|report| format!("{:?}", report.tab)),
        enabled: report.as_ref().map_or([false; 4], |report| report.enabled),
        items: report
            .as_ref()
            .map(|report| report.items.clone())
            .unwrap_or_default(),
        counts: report.as_ref().and_then(|report| report.counts),
        fleet_entry: fleets
            .entry_screen_rect(layout, primary, 0)
            .map(screen_center),
        fleets: report
            .as_ref()
            .map(|report| {
                report
                    .fleet_rows
                    .iter()
                    .filter_map(|&(listed, row)| {
                        Some(FleetEntryObservation {
                            ships: world
                                .fleets
                                .get(listed)?
                                .capital_ships
                                .iter()
                                .filter(|ship| ship.alive)
                                .count(),
                            entry: screen_center(fleets.entry_screen_rect(layout, primary, row)?),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default(),
        troops_tab: fleets
            .tab_screen_rect(layout, primary, FleetWindowTab::Troops)
            .map(screen_center),
        first_item: fleets
            .item_screen_rect(layout, primary, 0)
            .map(screen_center),
        aboard: transport.cargo(fleet).contains(&regiment),
        cargo: transport.carried_count(fleet),
        troop_system_dat_id: regiment_system(world, regiment)
            .and_then(|system| world.systems.get(system))
            .map(|system| system.dat_id.raw()),
        in_transit: movement.is_in_transit(fleet),
        regiment_travelling: transport
            .transits()
            .iter()
            .any(|transit| transit.troop == regiment),
        confirmation_open,
        last_message: messages.last().map(|message| message.text.clone()),
        // One frame can run several days, so a regiment's line may not be
        // the last.
        last_regiment_message: messages
            .iter()
            .rev()
            .find(|message| message.text.starts_with("Regiment "))
            .map(|message| message.text.clone()),
    })
}

/// Emits a [`FleetLoadObservation`] whenever it changes. The regiment it
/// follows is the one first on the primary system's surface when the watch
/// starts, or else the first aboard the player's fleet (the refused
/// variant).
#[derive(Debug, Default)]
pub struct FleetLoadWatch {
    regiment: Option<TroopKey>,
    last: Option<FleetLoadObservation>,
}

impl FleetLoadWatch {
    /// The observation when it differs from the last one.
    #[allow(clippy::too_many_arguments)]
    fn next(
        &mut self,
        request: FixtureRequest,
        world: &GameWorld,
        fog: &FogState,
        movement: &MovementState,
        transport: &TroopTransportState,
        fleets: &FleetWindowState,
        confirmation_open: bool,
        messages: &[GameMessage],
    ) -> Option<FleetLoadObservation> {
        if !request.scenario.uses_the_fleet_window() {
            return None;
        }
        if self.regiment.is_none() {
            self.regiment = world
                .systems
                .keys()
                .next()
                .and_then(|primary| loading_regiment(world, primary))
                .or_else(|| {
                    let fleet = world.fleets.keys().next()?;
                    transport.cargo(fleet).first().copied()
                });
        }
        let now = fleet_load_observation(
            request,
            self.regiment?,
            world,
            fog,
            movement,
            transport,
            fleets,
            confirmation_open,
            messages,
        );
        if now.is_none() || now == self.last {
            return None;
        }
        self.last.clone_from(&now);
        now
    }

    #[allow(clippy::too_many_arguments)]
    pub fn observe(
        &mut self,
        request: FixtureRequest,
        world: &GameWorld,
        fog: &FogState,
        movement: &MovementState,
        transport: &TroopTransportState,
        fleets: &FleetWindowState,
        confirmation_open: bool,
        messages: &[GameMessage],
    ) {
        let Some(report) = self.next(
            request,
            world,
            fog,
            movement,
            transport,
            fleets,
            confirmation_open,
            messages,
        ) else {
            return;
        };
        let bytes = serde_json::to_vec(&report).expect("serialize the fleet load observation");
        unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
    }
}

/// One Fleet Finder row: its name, what it stands for (a fleet by its place
/// in the world's fleets, and a ship by its index there), the side, and the
/// system the Finder would open.
#[derive(Debug, Clone, Serialize, PartialEq)]
struct FinderRowObservation {
    name: String,
    kind: &'static str,
    fleet: Option<usize>,
    ship: Option<usize>,
    is_alliance: bool,
    system_dat_id: Option<u32>,
}

/// Where the Finder's controls lie on screen, at their centers.
#[derive(Debug, Clone, Serialize, PartialEq)]
struct FinderControlsObservation {
    origin: (f32, f32),
    tabs: Vec<(f32, f32)>,
    close: (f32, f32),
    display: (f32, f32),
    ship_finder: (f32, f32),
    fleet_finder: (f32, f32),
    name_box: (f32, f32),
    /// Each row's center while the list shows it.
    rows: Vec<Option<(f32, f32)>>,
}

/// An open Fleet window and its selected entry.
#[derive(Debug, Clone, Serialize, PartialEq)]
struct FinderFleetWindowObservation {
    system_dat_id: u32,
    selected: Option<&'static str>,
    fleet: Option<usize>,
    ship: Option<usize>,
    /// The window's screen rect, `(x, y, width, height)`, so the gate can
    /// click a Finder control the window covers.
    rect: Option<(f32, f32, f32, f32)>,
}

/// A planet currently exposed by an open sector window. The browser gate uses
/// these exact screen coordinates to prove that a Finder click cannot fall
/// through to the covered original window.
#[derive(Debug, Clone, Serialize, PartialEq)]
struct FinderSectorPlanetObservation {
    system_dat_id: u32,
    center: (f32, f32),
}

/// What the fleet-finder gate checks after each step: the Finder (its mode,
/// tab, name box, rows, choice and controls), the cockpit's Fleet Finder
/// control, the open Fleet windows and the systems whose sector window is
/// open, and the galaxy view's selected system.
#[derive(Debug, Clone, Serialize, PartialEq)]
struct FleetFinderObservation {
    status: &'static str,
    code: u32,
    open: bool,
    player_is_alliance: bool,
    mode: Option<&'static str>,
    tab: Option<&'static str>,
    name: Option<String>,
    rows: Vec<FinderRowObservation>,
    chosen: Option<usize>,
    first_row: usize,
    controls: Option<FinderControlsObservation>,
    cockpit_button: (f32, f32),
    fleet_windows: Vec<FinderFleetWindowObservation>,
    sector_open: Vec<u32>,
    sector_planets: Vec<FinderSectorPlanetObservation>,
    selected_system_dat_id: Option<u32>,
    left_panel_open: bool,
}

fn fleet_ordinal(world: &GameWorld, fleet: FleetKey) -> Option<usize> {
    world.fleets.keys().position(|key| key == fleet)
}

fn finder_entry(
    world: &GameWorld,
    entry: FleetWindowEntry,
) -> (&'static str, Option<usize>, Option<usize>) {
    match entry {
        FleetWindowEntry::Fleet(fleet) => ("fleet", fleet_ordinal(world, fleet), None),
        FleetWindowEntry::Ship { fleet, index } => {
            ("ship", fleet_ordinal(world, fleet), Some(index))
        }
    }
}

pub struct FinderWindows<'a> {
    pub fog: &'a FogState,
    pub movement: &'a MovementState,
    pub finder: &'a FleetFinderState,
    pub fleets: &'a FleetWindowState,
    pub sectors: &'a SectorWindowState,
    pub map: &'a GalaxyMapState,
    /// Whether a left panel the galaxy view's letter keys toggle is open.
    pub left_panel_open: bool,
}

fn fleet_finder_observation(
    request: FixtureRequest,
    world: &GameWorld,
    windows: &FinderWindows<'_>,
) -> Option<FleetFinderObservation> {
    if request.scenario != Scenario::FleetFinder {
        return None;
    }
    let layout = CockpitState::new(request.faction).layout_for(640.0, 480.0);
    let player = if request.faction == CockpitFaction::Alliance {
        Faction::Alliance
    } else {
        Faction::Empire
    };
    let report = windows
        .finder
        .report(world, windows.movement, windows.fog, player);
    let rows = report.as_ref().map_or_else(Vec::new, |report| {
        report
            .rows
            .iter()
            .map(|row| {
                let (kind, fleet, ship) = finder_entry(world, row.object);
                let value = world.fleets.get(row.object.fleet());
                FinderRowObservation {
                    name: row.name.clone(),
                    kind,
                    fleet,
                    ship,
                    is_alliance: value.is_some_and(|value| value.is_alliance),
                    system_dat_id: value
                        .and_then(|value| world.systems.get(value.location))
                        .map(|system| system.dat_id.raw()),
                }
            })
            .collect()
    });
    let center = |control| {
        windows
            .finder
            .control_screen_rect(layout, control)
            .map(screen_center)
    };
    let controls = windows.finder.screen_rect(layout).and_then(|rect| {
        Some(FinderControlsObservation {
            origin: (rect.min.x, rect.min.y),
            tabs: [FinderTab::All, FinderTab::Alliance, FinderTab::Imperial]
                .into_iter()
                .map(|tab| center(FinderControl::Tab(tab)))
                .collect::<Option<_>>()?,
            close: center(FinderControl::Close)?,
            display: center(FinderControl::Display)?,
            ship_finder: center(FinderControl::ShipFinder)?,
            fleet_finder: center(FinderControl::FleetFinder)?,
            name_box: center(FinderControl::NameBox)?,
            rows: (0..rows.len())
                .map(|row| center(FinderControl::Row(row)))
                .collect(),
        })
    });
    let button = strategic_primary_controls(request.faction)
        .iter()
        .find(|control| control.button == CockpitButton::FleetFinder)?
        .rect;
    let fleet_windows = world
        .systems
        .iter()
        .filter_map(|(key, system)| {
            let (selected, _) = windows.fleets.selection(key)?;
            let (kind, fleet, ship) = selected.map_or((None, None, None), |entry| {
                let (kind, fleet, ship) = finder_entry(world, entry);
                (Some(kind), fleet, ship)
            });
            Some(FinderFleetWindowObservation {
                system_dat_id: system.dat_id.raw(),
                selected: kind,
                fleet,
                ship,
                rect: windows
                    .fleets
                    .screen_rect(layout, key)
                    .map(|rect| (rect.min.x, rect.min.y, rect.width(), rect.height())),
            })
        })
        .collect();
    let sector_planets: Vec<_> = world
        .systems
        .iter()
        .filter_map(|(key, system)| {
            let rect = windows.sectors.planet_screen_rect(world, layout, key)?;
            let center = rect.center();
            Some(FinderSectorPlanetObservation {
                system_dat_id: system.dat_id.raw(),
                center: (center.x, center.y),
            })
        })
        .collect();
    let sector_open = sector_planets
        .iter()
        .map(|planet| planet.system_dat_id)
        .collect();
    Some(FleetFinderObservation {
        status: "fleet-finder",
        code: request.code,
        open: report.is_some(),
        player_is_alliance: player == Faction::Alliance,
        mode: report.as_ref().map(|report| match report.mode {
            FinderMode::Fleets => "fleets",
            FinderMode::Ships => "ships",
        }),
        tab: report.as_ref().map(|report| match report.tab {
            FinderTab::All => "all",
            FinderTab::Alliance => "alliance",
            FinderTab::Imperial => "imperial",
        }),
        name: report.as_ref().map(|report| report.name.clone()),
        rows,
        chosen: report.as_ref().and_then(|report| report.chosen),
        first_row: report.as_ref().map_or(0, |report| report.first_row),
        controls,
        cockpit_button: (
            layout.canvas.x + (button.x + button.width / 2.0) * layout.scale,
            layout.canvas.y + (button.y + button.height / 2.0) * layout.scale,
        ),
        fleet_windows,
        sector_open,
        sector_planets,
        selected_system_dat_id: windows
            .map
            .selected_system
            .and_then(|system| world.systems.get(system))
            .map(|system| system.dat_id.raw()),
        left_panel_open: windows.left_panel_open,
    })
}

#[derive(Debug, Default)]
pub struct FleetFinderWatch {
    last: Option<FleetFinderObservation>,
}

impl FleetFinderWatch {
    /// The observation when it differs from the last one.
    fn next(
        &mut self,
        request: FixtureRequest,
        world: &GameWorld,
        windows: &FinderWindows<'_>,
    ) -> Option<FleetFinderObservation> {
        let now = fleet_finder_observation(request, world, windows);
        if now.is_none() || now == self.last {
            return None;
        }
        self.last.clone_from(&now);
        now
    }

    pub fn observe(
        &mut self,
        request: FixtureRequest,
        world: &GameWorld,
        windows: &FinderWindows<'_>,
    ) {
        let Some(report) = self.next(request, world, windows) else {
            return;
        };
        let bytes = serde_json::to_vec(&report).expect("serialize the fleet finder observation");
        unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
    }
}

/// The Operation Report scenario's report: the player's fleet bombarded
/// the galaxy's first system, held by the other side, losing one ship, and
/// killed a regiment and a shipyard there, so every list and both pictures'
/// families show. Its message is the player's own copy, as a live alert
/// opens it ([`crate::strike_orders::report_messages`]).
///
/// port: test fixture values, not rules.
#[must_use]
pub fn operation_report(
    world: &GameWorld,
    templates: &rebellion_data::text_templates::TextTemplates,
    faction: CockpitFaction,
) -> rebellion_render::OperationReportState {
    use rebellion_core::ids::DatId;
    use rebellion_core::operation_report::{OperationReport, ReportUnit};
    let unit = |raw: u32, name: &str, major: bool| ReportUnit {
        dat_id: DatId::new(raw),
        name: name.to_owned(),
        major,
        object: None,
    };
    let alliance = faction == CockpitFaction::Alliance;
    let (owner, holder) = if alliance {
        (Faction::Alliance, Faction::Empire)
    } else {
        (Faction::Empire, Faction::Alliance)
    };
    let mut state = rebellion_render::OperationReportState::default();
    let Some(system) = world.systems.keys().next() else {
        return state;
    };
    let mut report = OperationReport::new(system, false, holder, owner);
    if alliance {
        report.fleet_operational = vec![
            unit(0x1400_0040, "Mon Calamari Cruiser", false),
            unit(0x1c00_0001, "X-wing", false),
            unit(0x3000_0240, "Luke Skywalker", true),
        ];
        report.fleet_destroyed = vec![unit(0x1400_0045, "Corellian Corvette", false)];
        report.system_operational = vec![
            unit(0x1000_0006, "Stormtrooper Regiment", false),
            unit(0x2200_0001, "KDY-150", false),
        ];
        report.system_destroyed = vec![
            unit(0x1000_0008, "Imperial Army Regiment", false),
            unit(0x2800_0001, "Orbital Shipyard", false),
        ];
    } else {
        report.fleet_operational = vec![
            unit(0x1400_0080, "Strike Cruiser", false),
            unit(0x1c00_0005, "TIE Fighter", false),
            unit(0x3000_0280, "Darth Vader", true),
        ];
        report.fleet_destroyed = vec![unit(0x1400_0081, "Lancer Frigate", false)];
        report.system_operational = vec![
            unit(0x1000_0005, "Mon Calamari Regiment", false),
            unit(0x2200_0001, "KDY-150", false),
        ];
        report.system_destroyed = vec![
            unit(0x1000_0002, "Alliance Army Regiment", false),
            unit(0x2800_0001, "Orbital Shipyard", false),
        ];
    }
    let report = std::sync::Arc::new(report);
    let messages = crate::strike_orders::report_messages(world, templates, 0, &report);
    if let Some(message) = messages.iter().find(|message| message.alerts()) {
        state.open(world, message, faction, None);
    }
    state
}

#[derive(Debug, Serialize, PartialEq)]
struct FixtureOperationReport {
    status: &'static str,
    code: u32,
    open: bool,
    view: u16,
    tab: u16,
}

/// A cell of the live report: its name, DAT family and the bitmaps
/// [`rebellion_render::operation_report::cell_layers`] stacks for it, bottom
/// first, on the tab that lists its family.
#[derive(Debug, Serialize, PartialEq)]
struct FixtureReportCell {
    name: String,
    family: u8,
    layers: Vec<(String, u32)>,
}

/// The shown live report: what it says, its lists and, for its Display
/// buttons, the last one taken and the windows open on its system.
#[derive(Debug, Serialize, PartialEq)]
struct FixtureLiveReport {
    status: &'static str,
    code: u32,
    live: bool,
    open: bool,
    view: u16,
    tab: u16,
    title: String,
    body: String,
    picture: u16,
    assault: bool,
    fleet_operational: Vec<FixtureReportCell>,
    fleet_destroyed: Vec<FixtureReportCell>,
    system_operational: Vec<FixtureReportCell>,
    system_destroyed: Vec<FixtureReportCell>,
    destination: Option<String>,
    sector_open: bool,
    fleet_window_open: bool,
}

/// The windows a live report's Display buttons open, and the button taken
/// this frame: its system and whether it was the Fleet button.
pub struct ReportDestinations<'a> {
    pub sectors: &'a SectorWindowState,
    pub fleets: &'a FleetWindowState,
    pub chosen: Option<(SystemKey, bool)>,
}

/// Emits the report window's view and tab each time they change, so the
/// browser probe can follow its clicks.
#[derive(Debug, Default)]
pub struct OperationReportWatch {
    last: Option<(bool, u16, u16)>,
    live_last: Option<Vec<u8>>,
    live_system: Option<SystemKey>,
    destination: Option<String>,
}

impl OperationReportWatch {
    fn next(
        &mut self,
        request: FixtureRequest,
        state: &rebellion_render::OperationReportState,
    ) -> Option<FixtureOperationReport> {
        let (view, tab) = state
            .view()
            .map_or((0, 0), |(view, tab)| (view.command(), tab.command()));
        let now = (state.is_open(), view, tab);
        if request.scenario != Scenario::OperationReport || self.last == Some(now) {
            return None;
        }
        self.last = Some(now);
        Some(FixtureOperationReport {
            status: "operation-report",
            code: request.code,
            open: now.0,
            view,
            tab,
        })
    }

    pub fn observe(
        &mut self,
        request: FixtureRequest,
        state: &rebellion_render::OperationReportState,
    ) {
        let Some(report) = self.next(request, state) else {
            return;
        };
        let bytes = serde_json::to_vec(&report).expect("serialize the operation report state");
        unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
    }

    fn next_live(
        &mut self,
        request: FixtureRequest,
        state: &rebellion_render::OperationReportState,
        world: &GameWorld,
        movement: &MovementState,
        windows: &ReportDestinations<'_>,
    ) -> Option<Vec<u8>> {
        if !request.scenario.moves_a_fleet() {
            return None;
        }
        let shown = state.shown();
        if let Some((report, _, _)) = shown {
            self.live_system = Some(report.system);
        }
        let (view, tab) = state
            .view()
            .map_or((0, 0), |(view, tab)| (view.command(), tab.command()));
        let cells = |units: &[rebellion_core::operation_report::ReportUnit]| {
            units
                .iter()
                .map(|unit| report_cell(world, movement, unit))
                .collect::<Vec<_>>()
        };
        if let Some((system, fleet)) = windows.chosen {
            let name = world.systems.get(system).map_or("", |value| value.name.as_str());
            self.destination = Some(format!("{}:{name}", if fleet { "fleet" } else { "sector" }));
        }
        let system = self.live_system;
        let observation = FixtureLiveReport {
            status: "operation-report",
            code: request.code,
            live: true,
            open: state.is_open(),
            view,
            tab,
            title: shown.map_or_else(String::new, |(_, title, _)| title.to_owned()),
            body: shown.map_or_else(String::new, |(_, _, body)| body.to_owned()),
            picture: shown.map_or(0, |(report, _, _)| report.picture()),
            assault: shown.is_some_and(|(report, _, _)| report.assault),
            fleet_operational: shown.map_or_else(Vec::new, |(report, _, _)| {
                cells(&report.fleet_operational)
            }),
            fleet_destroyed: shown.map_or_else(Vec::new, |(report, _, _)| {
                cells(&report.fleet_destroyed)
            }),
            system_operational: shown.map_or_else(Vec::new, |(report, _, _)| {
                cells(&report.system_operational)
            }),
            system_destroyed: shown.map_or_else(Vec::new, |(report, _, _)| {
                cells(&report.system_destroyed)
            }),
            destination: self.destination.clone(),
            sector_open: system
                .and_then(|system| world.systems.get(system))
                .is_some_and(|value| {
                    windows
                        .sectors
                        .children()
                        .any(|(_, child)| child == rebellion_render::GalaxyChild::Sector(value.sector))
                }),
            fleet_window_open: system.is_some_and(|system| windows.fleets.is_open(system)),
        };
        let bytes = serde_json::to_vec(&observation).expect("serialize the live report");
        if self.live_last.as_ref() == Some(&bytes) {
            return None;
        }
        self.live_last = Some(bytes.clone());
        Some(bytes)
    }

    /// Emits the live report window (the strike and assault scenarios) each
    /// time it or its Display destinations change.
    pub fn observe_live(
        &mut self,
        request: FixtureRequest,
        state: &rebellion_render::OperationReportState,
        world: &GameWorld,
        movement: &MovementState,
        windows: &ReportDestinations<'_>,
    ) {
        let Some(bytes) = self.next_live(request, state, world, movement, windows) else {
            return;
        };
        unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
    }
}

/// `unit`'s cell on the tab that lists its family.
fn report_cell(
    world: &GameWorld,
    movement: &MovementState,
    unit: &rebellion_core::operation_report::ReportUnit,
) -> FixtureReportCell {
    use rebellion_render::operation_report::{cell_layers, cell_status, ReportTab};
    let family = unit.family();
    let layers = ReportTab::ALL
        .into_iter()
        .find(|tab| tab.families().contains(&family))
        .map_or_else(Vec::new, |tab| {
            cell_layers(unit, tab, cell_status(world, movement, unit))
                .into_iter()
                .map(|(source, id)| (format!("{source:?}"), id))
                .collect()
        });
    FixtureReportCell {
        name: unit.name.clone(),
        family,
        layers,
    }
}

/// Where the open Game Speed menu lies, so the fleet-load gate can unpause
/// the clock: the browser has no keypad `+` (`gl.js`).
#[derive(Debug, Serialize, PartialEq)]
struct FixtureSpeedMenu {
    status: &'static str,
    code: u32,
    left: f32,
    top: f32,
    width: f32,
    height: f32,
    rows: usize,
}

/// Emits the speed menu's rect each time it opens or moves.
#[derive(Debug, Default)]
pub struct SpeedMenuWatch {
    last: Option<egui_macroquad::egui::Rect>,
}

impl SpeedMenuWatch {
    /// The report when the menu has opened or moved since the last one.
    fn next(
        &mut self,
        request: FixtureRequest,
        rect: Option<egui_macroquad::egui::Rect>,
    ) -> Option<FixtureSpeedMenu> {
        if !request.scenario.uses_the_fleet_window() || rect == self.last {
            return None;
        }
        self.last = rect;
        let rect = rect?;
        Some(FixtureSpeedMenu {
            status: "speed-menu",
            code: request.code,
            left: rect.min.x,
            top: rect.min.y,
            width: rect.width(),
            height: rect.height(),
            rows: 5,
        })
    }

    pub fn observe(&mut self, request: FixtureRequest, rect: Option<egui_macroquad::egui::Rect>) {
        let Some(report) = self.next(request, rect) else {
            return;
        };
        let bytes = serde_json::to_vec(&report).expect("serialize the speed menu report");
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
    let kinds = available_kinds(world, uprisings, faction, &team, &[], target, None);
    if !dialog.open(faction, target, None, team, kinds) {
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

    #[test]
    fn windowing_fixture_codes_round_trip_without_changing_existing_requests() {
        assert_eq!(Scenario::WindowingContract as u8, 65);
        assert_eq!(Scenario::OperationReport as u8, 66);
        assert_eq!(Scenario::StrikeDeathStar as u8, 67);
        assert_eq!(Scenario::UnfinishedShip as u8, 68);
        assert_eq!(Scenario::ShipBesideUnfinished as u8, 69);
        assert_eq!(Scenario::StrikeShielded as u8, 70);
        assert_eq!(Scenario::StrikeDamagedLoss as u8, 74);
        assert_eq!(SCENARIO_COUNT, 75);

        for scenario_value in 0..SCENARIO_COUNT {
            let scenario = Scenario::decode(scenario_value).expect("known fixture scenario");
            for (faction_byte, faction) in
                [(1, CockpitFaction::Alliance), (2, CockpitFaction::Empire)]
            {
                let external_code = u32::from(scenario_value) + 1 | (faction_byte << 8);
                let decoded = decode_request(external_code).expect("valid fixture request");
                assert_eq!(decoded.scenario, scenario);
                assert_eq!(decoded.faction, faction);
                assert_eq!(decoded.code, external_code);
            }
        }

        assert_eq!(
            decode_request(66 | (1 << 8)).map(|request| request.scenario),
            Some(Scenario::WindowingContract)
        );
        assert!(decode_request(66 | (3 << 8)).is_none());
        assert!(decode_request(66 | (1 << 8) | (1 << 16)).is_none());
    }

    #[test]
    fn the_operation_report_scenario_opens_the_players_own_report_on_its_summary() {
        // The fixture's report is the fleet owner's copy, a class-7 alert
        // (FUN_0048a060), opened on Summary and tab 0x97 (FUN_0044c630).
        use rebellion_render::operation_report::{ReportTab, ReportView};
        let mut world = GameWorld::default();
        world
            .systems
            .insert(rebellion_core::world::System::default());
        // The report's text is TEXTSTRA 0x7098/0x7099 (FUN_00496770);
        // these are the shipped entries.
        let entries: std::collections::HashMap<u16, &str> = [
            (0x7098, "Orbital bombardment of |\u{1}\u{4}\0\0\0\u{1}"),
            (
                0x7099,
                "|\u{1}\u{4}\0\0\0 ships have conducted an orbital strike on the \
                 |\u{2}\u{4}\0\0\0 system of |\u{3}\u{1}\0\0\0\u{1}",
            ),
        ]
        .into_iter()
        .collect();
        let templates = rebellion_data::text_templates::TextTemplates::from_json(
            &serde_json::to_vec(&entries).unwrap(),
        )
        .unwrap();
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let state = operation_report(&world, &templates, faction);
            assert_eq!(
                state.view(),
                Some((ReportView::Summary, ReportTab::CapitalShips)),
                "{faction:?}"
            );
        }
        let empty = operation_report(&GameWorld::default(), &templates, CockpitFaction::Empire);
        assert!(!empty.is_open());
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
            fighters: Vec::new(),
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
        let mut defenses = DefensesWindowState::default();

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
            &mut SystemWindowState::default(),
            &mut FleetWindowState::default(),
            &mut defenses,
            &mut TroopTransportState::default(),
        );

        // MissionState::mission_order_enabled enables the menu's Mission.
        assert!(missions.mission_order_enabled(
            &world,
            MissionFaction::Alliance,
            &[MissionMember::Character(agent)],
        ));
        let layout = cockpit.layout_for(640.0, 480.0);
        let galaxy = layout.galaxy;
        // The Defenses window's top-left corner, 5 pixels in from the
        // view's top-right.
        let corner = (
            galaxy.x + galaxy.width - DEFENSES_WINDOW_WIDTH - 5.0,
            galaxy.y + 5.0,
        );
        assert!(defenses.contains_screen_point(layout, corner));
        assert!(!defenses.contains_screen_point(layout, (corner.0 - 1.0, corner.1)));
        assert!(!defenses.contains_screen_point(layout, (corner.0, corner.1 - 1.0)));
        assert_eq!(sectors.window_count(), 1);

        let menu = ObjectMenuState::new(
            Some(MenuObject::Character(agent)),
            OrderGates {
                mission: true,
                ..OrderGates::default()
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
        // planet at (77, 70) in it is 37 by 37.
        let target = (report.target_screen_x, report.target_screen_y);
        assert_eq!(target, (60.0 + 77.0 + 18.5, 35.0 + 70.0 + 18.5));
        assert!(sectors.contains_screen_point(layout, target));
        assert!(!defenses.contains_screen_point(layout, target));
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
                bombarding: false,
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
        fleets: FleetWindowState,
        defenses: DefensesWindowState,
        transport: TroopTransportState,
        missions: MissionState,
    }

    fn apply_to_fleet_world(request: FixtureRequest) -> Applied {
        apply_to_fleet_world_with(request, |_| {})
    }

    fn apply_to_fleet_world_with(
        request: FixtureRequest,
        prepare: impl FnOnce(&mut GameWorld),
    ) -> Applied {
        let mut world = fleet_world();
        prepare(&mut world);
        let mut applied = Applied {
            world,
            movement: MovementState::default(),
            blockade: BlockadeState::default(),
            map: GalaxyMapState::default(),
            sectors: SectorWindowState::default(),
            systems: SystemWindowState::default(),
            fleets: FleetWindowState::default(),
            defenses: DefensesWindowState::default(),
            transport: TroopTransportState::default(),
            missions: MissionState::default(),
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
            &mut applied.missions,
            &mut applied.blockade,
            &mut applied.sectors,
            &mut applied.systems,
            &mut applied.fleets,
            &mut applied.defenses,
            &mut applied.transport,
        );
        applied
    }

    /// [`fleet_world`] with its third system moved into a sector of its own,
    /// where the loading scenarios send what they clear away.
    fn apply_to_loading_world(request: FixtureRequest) -> Applied {
        apply_to_loading_world_with(request, |_| {})
    }

    fn apply_to_loading_world_with(
        request: FixtureRequest,
        prepare: impl FnOnce(&mut GameWorld),
    ) -> Applied {
        let mut world = fleet_world();
        let mut keys = world.systems.keys();
        let (here, elsewhere) = (keys.next().unwrap(), keys.nth(1).unwrap());
        let home = world.systems[here].sector;
        let mut far = world.sectors[home].clone();
        far.dat_id = rebellion_core::ids::DatId::new(0x8000_0002);
        far.systems = vec![elsewhere];
        let far = world.sectors.insert(far);
        world.sectors[home].systems.retain(|&key| key != elsewhere);
        world.systems[elsewhere].sector = far;
        prepare(&mut world);
        let mut applied = Applied {
            world,
            movement: MovementState::default(),
            blockade: BlockadeState::default(),
            map: GalaxyMapState::default(),
            sectors: SectorWindowState::default(),
            systems: SystemWindowState::default(),
            fleets: FleetWindowState::default(),
            defenses: DefensesWindowState::default(),
            transport: TroopTransportState::default(),
            missions: MissionState::default(),
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
            &mut applied.missions,
            &mut applied.blockade,
            &mut applied.sectors,
            &mut applied.systems,
            &mut applied.fleets,
            &mut applied.defenses,
            &mut applied.transport,
        );
        applied
    }

    #[test]
    fn a_fixture_closes_an_open_gid_menu_and_shows_the_windows_it_hid() {
        for (faction, side) in [(CockpitFaction::Alliance, 1), (CockpitFaction::Empire, 2)] {
            let request = decode_request(1 | (side << 8)).expect("the first scenario");
            let mut cockpit = CockpitState::new(faction);
            cockpit.gid_ui.open_menu();
            cockpit.gid_ui.category = Some(rebellion_render::cockpit::GidCategory::Fleets);
            apply(
                request,
                &mut fleet_world(),
                &mut GameMode::MainMenu,
                &mut MissionFaction::Alliance,
                &mut cockpit,
                &mut GalaxyMapState::default(),
                &mut MovementState::default(),
                &mut ManufacturingState::default(),
                &mut EconomyState::default(),
                &mut MissionState::default(),
                &mut BlockadeState::default(),
                &mut SectorWindowState::default(),
                &mut SystemWindowState::default(),
                &mut FleetWindowState::default(),
                &mut DefensesWindowState::default(),
                &mut TroopTransportState::default(),
            );
            assert!(!cockpit.gid_ui.menu_open, "{faction:?}");
            assert!(!cockpit.gid_ui.windows_hidden, "{faction:?}");
            assert_eq!(cockpit.gid_ui.category, None);
        }
    }

    // port: the fixture's own layout; the load itself follows FUN_00500b40.
    #[test]
    fn the_fleet_load_scenario_leaves_one_carrier_and_one_movable_regiment_at_the_players_system() {
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let request = request(Scenario::FleetLoad, faction);
            let mut applied = apply_to_loading_world(request);
            let world = &applied.world;
            let mut keys = world.systems.keys();
            let (here, target) = (keys.next().unwrap(), keys.next().unwrap());
            let fleet = world.fleets.keys().next().unwrap();
            let is_alliance = faction == CockpitFaction::Alliance;
            let player = if is_alliance {
                Faction::Alliance
            } else {
                Faction::Empire
            };

            assert_eq!(world.systems[here].fleets, [fleet], "{faction:?}");
            assert!(world.systems[target].fleets.is_empty(), "{faction:?}");
            assert_eq!(world.fleets[fleet].is_alliance, is_alliance);
            let [ship] = world.fleets[fleet].capital_ships.as_slice() else {
                panic!("{faction:?}: the fleet holds one capital ship");
            };
            let class = &world.capital_ship_classes[ship.class];
            assert!(
                class.troop_capacity > 0 && class.hyperdrive > 0,
                "{faction:?}"
            );
            assert_eq!(
                world.systems[target].control,
                ControlKind::Controlled(player)
            );
            assert!(world.systems[target].ground_units.is_empty());
            let regiment = loading_regiment(world, here).unwrap();
            assert_eq!(world.systems[here].ground_units, [regiment]);
            assert_eq!(world.troops[regiment].is_alliance, is_alliance);
            assert!(applied.transport.is_empty());
            assert!(applied
                .transport
                .regiment_move_enabled(world, regiment, is_alliance));

            let layout = CockpitState::new(faction).layout_for(640.0, 480.0);
            let setup =
                fleet_load_setup(request, world, &applied.sectors, &applied.defenses).unwrap();
            assert_eq!(setup.target_dat_id, world.systems[target].dat_id.raw());
            assert_eq!(setup.target_name, world.systems[target].name);
            assert_eq!(setup.capacity, Some(class.troop_capacity));
            // The Defenses window lies on the galaxy view's left, the
            // primary system's sector window in the right column.
            assert!(applied
                .defenses
                .contains_screen_point(layout, setup.troop_item.unwrap()));
            assert!(!applied.defenses.contains_screen_point(layout, setup.icon));
            assert!(applied.sectors.contains_screen_point(layout, setup.icon));
            assert!(applied
                .sectors
                .contains_screen_point(layout, setup.target_planet));

            applied
                .transport
                .load(
                    &mut applied.world,
                    &ProductRoom::default(),
                    fleet,
                    &[regiment],
                )
                .unwrap();
        }
    }

    #[test]
    fn the_destination_scenario_adds_a_player_shipyard_and_shows_its_facility_icon() {
        // port: the fixture's own layout for the Destination gate
        // (ghidra/notes/production-destination.md).
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let player_shipyards = |scenario| {
                let applied = apply_to_loading_world(request(scenario, faction));
                let world = &applied.world;
                let primary = world.systems.keys().next().unwrap();
                let setup = fleet_load_setup(
                    request(scenario, faction),
                    world,
                    &applied.sectors,
                    &applied.defenses,
                )
                .unwrap();
                let layout = CockpitState::new(faction).layout_for(640.0, 480.0);
                if let Some(icon) = setup.system_icon {
                    assert!(
                        applied.sectors.contains_screen_point(layout, icon),
                        "{faction:?}"
                    );
                }
                let shipyards = world.systems[primary]
                    .manufacturing_facilities
                    .iter()
                    .filter(|key| {
                        let facility = &world.manufacturing_facilities[**key];
                        facility.is_shipyard
                            && (facility.side == rebellion_core::dat::Faction::Alliance)
                                == (faction == CockpitFaction::Alliance)
                    })
                    .count();
                (shipyards, setup.system_icon.is_some())
            };
            let (before, _) = player_shipyards(Scenario::FleetLoad);
            assert_eq!(
                player_shipyards(Scenario::ProductionDestination),
                (before + 1, true),
                "{faction:?}"
            );
        }
    }

    #[test]
    fn the_joining_scenario_puts_a_player_fleet_of_one_ship_beside_one_of_two() {
        // port: the fixture's own layout for the join gate
        // (ghidra/notes/fleet-join-split.md).
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let applied = apply_to_loading_world(request(Scenario::FleetJoin, faction));
            let world = &applied.world;
            let here = world.systems.keys().next().unwrap();
            let first = world.fleets.keys().next().unwrap();
            let is_alliance = faction == CockpitFaction::Alliance;
            let [a, b] = world.systems[here].fleets.as_slice() else {
                panic!("{faction:?}: two fleets orbit the primary system");
            };
            assert_eq!(*a, first, "{faction:?}");
            for (fleet, ships) in [(*a, 2), (*b, 1)] {
                let value = &world.fleets[fleet];
                assert_eq!(value.capital_ships.len(), ships, "{faction:?}");
                assert!(value.capital_ships.iter().all(|ship| ship.alive));
                assert_eq!(value.is_alliance, is_alliance);
                assert!(
                    !value.has_death_star(&world.capital_ship_classes)
                        && value.characters.is_empty()
                );
                assert!(!applied.movement.is_in_transit(fleet));
            }
        }
    }

    #[test]
    fn the_fleet_load_observation_lists_each_fleet_with_its_ships_and_entry() {
        // port: the fixture's own report.
        let request = request(Scenario::FleetJoin, CockpitFaction::Alliance);
        let applied = apply_to_loading_world(request);
        let here = applied.world.systems.keys().next().unwrap();
        let regiment = loading_regiment(&applied.world, here).unwrap();
        let mut fog = FogState::new(Faction::Alliance);
        fog.reveal(here);
        let layout = CockpitState::new(CockpitFaction::Alliance).layout_for(640.0, 480.0);
        let mut fleets = FleetWindowState::default();
        fleets.open(
            &applied.world,
            here,
            (300, 80),
            CockpitFaction::Alliance,
            layout,
        );
        let observe = |confirmation_open| {
            fleet_load_observation(
                request,
                regiment,
                &applied.world,
                &fog,
                &applied.movement,
                &applied.transport,
                &fleets,
                confirmation_open,
                &[],
            )
            .unwrap()
        };
        let row = |row| screen_center(fleets.entry_screen_rect(layout, here, row).unwrap());

        let observed = observe(false);
        assert_eq!(observed.entries, 2);
        assert_eq!(
            observed.fleets,
            [
                FleetEntryObservation {
                    ships: 2,
                    entry: row(0)
                },
                FleetEntryObservation {
                    ships: 1,
                    entry: row(1)
                },
            ]
        );
        assert_ne!(row(0), row(1));
        assert!(!observed.confirmation_open);
        assert!(observe(true).confirmation_open);
    }

    #[test]
    fn the_fleet_load_scenario_picks_a_carrier_of_the_players_side_that_can_jump() {
        // port: the fixture's own choice of a regiment carrier.
        use rebellion_core::world::CapitalShipClass;
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let is_alliance = faction == CockpitFaction::Alliance;
            let class = |is_alliance, troop_capacity, hyperdrive| CapitalShipClass {
                is_alliance,
                troop_capacity,
                hyperdrive,
                hull: 100,
                ..CapitalShipClass::default()
            };
            let mut carrier = None;
            let applied =
                apply_to_loading_world_with(request(Scenario::FleetLoad, faction), |world| {
                    world.capital_ship_classes.insert(class(is_alliance, 0, 80));
                    world.capital_ship_classes.insert(class(is_alliance, 2, 0));
                    world
                        .capital_ship_classes
                        .insert(class(!is_alliance, 2, 80));
                    carrier = Some(world.capital_ship_classes.insert(class(is_alliance, 3, 80)));
                });
            let fleet = applied.world.fleets.keys().next().unwrap();
            assert_eq!(
                applied.world.fleets[fleet].capital_ships[0].class,
                carrier.unwrap(),
                "{faction:?}"
            );
        }
    }

    #[test]
    fn without_a_carrier_the_fleet_load_scenario_builds_one_for_the_players_side() {
        // port: the fixture's fallback class.
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let applied = apply_to_loading_world(request(Scenario::FleetLoad, faction));
            let fleet = applied.world.fleets.keys().next().unwrap();
            let class = &applied.world.capital_ship_classes
                [applied.world.fleets[fleet].capital_ships[0].class];
            assert_eq!(class.name, "Transport");
            assert_eq!(class.is_alliance, faction == CockpitFaction::Alliance);
            assert_eq!(
                (class.hull, class.hyperdrive, class.troop_capacity),
                (100, 80, 2)
            );
        }
    }

    #[test]
    fn the_refused_unload_scenario_starts_with_the_regiment_aboard_and_the_target_in_the_system_window(
    ) {
        // port: the fixture's own layout; FUN_0053d430 refuses the drop
        // (other side, populated).
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let request = request(Scenario::RegimentUnloadRefused, faction);
            let applied = apply_to_loading_world(request);
            let world = &applied.world;
            let mut keys = world.systems.keys();
            let (here, target) = (keys.next().unwrap(), keys.next().unwrap());
            let fleet = world.fleets.keys().next().unwrap();
            let is_alliance = faction == CockpitFaction::Alliance;
            let enemy = if is_alliance {
                Faction::Empire
            } else {
                Faction::Alliance
            };

            assert_eq!(
                world.systems[target].control,
                ControlKind::Controlled(enemy),
                "{faction:?}"
            );
            assert!(world.systems[target].is_populated);
            assert!(world.systems[here].ground_units.is_empty());
            let [regiment] = applied.transport.cargo(fleet) else {
                panic!("{faction:?}: one regiment starts aboard");
            };
            assert_eq!(world.troops[*regiment].is_alliance, is_alliance);
            assert_eq!(applied.defenses.window_count(), 1);
            assert!(applied.defenses.is_open(target));

            let layout = CockpitState::new(faction).layout_for(640.0, 480.0);
            let setup =
                fleet_load_setup(request, world, &applied.sectors, &applied.defenses).unwrap();
            assert_eq!(setup.troop_item, None);
            assert!(applied
                .defenses
                .contains_screen_point(layout, setup.defenses_window));

            let mut watch = FleetLoadWatch::default();
            let start = watch
                .next(
                    request,
                    world,
                    &FogState::new(enemy),
                    &applied.movement,
                    &applied.transport,
                    &FleetWindowState::default(),
                    false,
                    &[],
                )
                .unwrap();
            assert!(start.aboard, "{faction:?}");
            assert_eq!((start.cargo, start.troop_system_dat_id), (1, None));
        }
    }

    #[test]
    fn the_fleet_load_scenario_leaves_fleets_elsewhere_where_they_are() {
        // port: only fleets at the loading and target systems are moved.
        let mut kept = None;
        let applied = apply_to_loading_world_with(
            request(Scenario::FleetLoad, CockpitFaction::Alliance),
            |world| {
                let copy = world.systems.values().nth(2).unwrap().clone();
                let fourth = world.systems.insert(copy);
                let fleet = world
                    .fleets
                    .insert(world.fleets.values().next().unwrap().clone());
                world.fleets[fleet].location = fourth;
                kept = Some((fleet, fourth));
            },
        );
        let (fleet, fourth) = kept.unwrap();
        assert_eq!(applied.world.fleets[fleet].location, fourth);
    }

    #[test]
    fn the_fleet_load_scenarios_regiment_is_of_the_players_troop_class() {
        // port: the fixture copies the class of a regiment of the player's side.
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let is_alliance = faction == CockpitFaction::Alliance;
            let applied =
                apply_to_loading_world_with(request(Scenario::FleetLoad, faction), |world| {
                    for (side, raw) in [(!is_alliance, 0x1000_0010), (is_alliance, 0x1000_0020)] {
                        world.troops.insert(TroopUnit {
                            class_dat_id: rebellion_core::ids::DatId::new(raw),
                            is_alliance: side,
                        });
                    }
                });
            let here = applied.world.systems.keys().next().unwrap();
            let regiment = loading_regiment(&applied.world, here).unwrap();
            assert_eq!(
                applied.world.troops[regiment].class_dat_id.raw(),
                0x1000_0020,
                "{faction:?}"
            );
        }
    }

    #[test]
    fn the_fleet_load_scenarios_defenses_window_sits_five_pixels_into_the_galaxy_view() {
        // port: the fixture's own placement.
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let applied = apply_to_loading_world(request(Scenario::FleetLoad, faction));
            let galaxy = CockpitState::new(faction).layout_for(640.0, 480.0).galaxy;
            let layout = CockpitState::new(faction).layout_for(640.0, 480.0);
            let inside = |x: f32, y: f32| applied.defenses.contains_screen_point(layout, (x, y));
            let (x, y) = (galaxy.x + 5.0, galaxy.y + 5.0);
            assert!(inside(x, y), "{faction:?}");
            assert!(!inside(x - 0.5, y) && !inside(x, y - 0.5), "{faction:?}");
        }
    }

    #[test]
    fn the_fleet_load_watch_reports_only_its_own_regiment_as_travelling() {
        // port: the fixture's own report of TroopTransportState::transits.
        use rebellion_core::troop_transport::RegimentTarget;
        let request = request(Scenario::FleetLoad, CockpitFaction::Alliance);
        let mut applied = apply_to_loading_world(request);
        let mut keys = applied.world.systems.keys();
        let (here, target) = (keys.next().unwrap(), keys.next().unwrap());
        let regiment = loading_regiment(&applied.world, here).unwrap();
        let other = applied
            .world
            .troops
            .insert(applied.world.troops[regiment].clone());
        applied.world.systems[here].ground_units.push(other);
        let fog = FogState::new(Faction::Alliance);
        let fleets = FleetWindowState::default();
        let observe = |applied: &Applied| {
            fleet_load_observation(
                request,
                regiment,
                &applied.world,
                &fog,
                &applied.movement,
                &applied.transport,
                &fleets,
                false,
                &[],
            )
            .unwrap()
        };

        let blockaded = std::collections::HashSet::new();
        applied
            .transport
            .move_regiment(
                &mut applied.world,
                &applied.movement,
                &blockaded,
                &ProductRoom::default(),
                other,
                RegimentTarget::System(target),
                0,
            )
            .unwrap();
        assert!(!observe(&applied).regiment_travelling);

        applied
            .transport
            .move_regiment(
                &mut applied.world,
                &applied.movement,
                &blockaded,
                &ProductRoom::default(),
                regiment,
                RegimentTarget::System(target),
                0,
            )
            .unwrap();
        let travelling = observe(&applied);
        assert!(travelling.regiment_travelling);
        assert_eq!(travelling.troop_system_dat_id, None);

        use rebellion_render::MessageCategory;
        let line = |text: &str| GameMessage::new(0, text.to_string(), MessageCategory::Event);
        let messages = [
            line("Regiment move rejected: the destination is blockaded"),
            line("Regiment arrived at Cathar"),
            line("Luke Skywalker has reached Jedi Knight tier"),
        ];
        let later = fleet_load_observation(
            request,
            regiment,
            &applied.world,
            &fog,
            &applied.movement,
            &applied.transport,
            &fleets,
            false,
            &messages,
        )
        .unwrap();
        assert_eq!(
            later.last_regiment_message.as_deref(),
            Some("Regiment arrived at Cathar")
        );
    }

    #[test]
    fn the_fleet_load_watch_reports_each_change_once() {
        // port: the fixture's own report.
        let request = request(Scenario::FleetLoad, CockpitFaction::Alliance);
        let mut applied = apply_to_loading_world(request);
        let here = applied.world.systems.keys().next().unwrap();
        let fleet = applied.world.fleets.keys().next().unwrap();
        let regiment = loading_regiment(&applied.world, here).unwrap();
        let mut fog = FogState::new(Faction::Alliance);
        fog.reveal(here);
        let mut fleets = FleetWindowState::default();
        let mut watch = FleetLoadWatch::default();
        let layout = CockpitState::new(CockpitFaction::Alliance).layout_for(640.0, 480.0);
        let mut next = |applied: &Applied, fleets: &FleetWindowState| {
            watch.next(
                request,
                &applied.world,
                &fog,
                &applied.movement,
                &applied.transport,
                fleets,
                false,
                &[],
            )
        };

        let start = next(&applied, &fleets).unwrap();
        assert_eq!(start.status, "fleet-load");
        assert!(!start.window_open && !start.aboard);
        assert_eq!(
            start.troop_system_dat_id,
            Some(applied.world.systems[here].dat_id.raw())
        );
        assert_eq!(next(&applied, &fleets), None);

        fleets.open(
            &applied.world,
            here,
            (300, 80),
            CockpitFaction::Alliance,
            layout,
        );
        let opened = next(&applied, &fleets).unwrap();
        assert!(opened.window_open);
        assert_eq!((opened.origin, opened.entries), (Some((300, 80)), 1));
        assert_eq!(
            opened.fleet_entry,
            Some((300.0 + 4.0 + 45.5, 80.0 + 29.0 + 25.0))
        );
        assert_eq!(
            opened.troops_tab,
            Some((300.0 + 99.0 + 65.0 + 15.5, 80.0 + 96.0 + 14.0))
        );

        applied
            .transport
            .load(
                &mut applied.world,
                &ProductRoom::default(),
                fleet,
                &[regiment],
            )
            .unwrap();
        let loaded = next(&applied, &fleets).unwrap();
        assert!(loaded.aboard);
        assert_eq!((loaded.cargo, loaded.troop_system_dat_id), (1, None));

        let moving = FixtureRequest {
            scenario: Scenario::FleetMove,
            ..request
        };
        assert_eq!(
            FleetLoadWatch::default().next(
                moving,
                &applied.world,
                &fog,
                &applied.movement,
                &applied.transport,
                &fleets,
                false,
                &[],
            ),
            None
        );
    }

    #[test]
    fn the_speed_menu_watch_reports_the_menu_when_it_opens_or_moves() {
        // port: the fixture's own report; the menu lists five speeds
        // (FUN_0042d190).
        use egui_macroquad::egui;
        let request = request(Scenario::FleetLoad, CockpitFaction::Alliance);
        let rect = egui::Rect::from_min_size(egui::pos2(100.0, 30.0), egui::vec2(90.0, 72.0));
        let mut watch = SpeedMenuWatch::default();

        assert_eq!(watch.next(request, None), None);
        assert_eq!(
            watch.next(request, Some(rect)),
            Some(FixtureSpeedMenu {
                status: "speed-menu",
                code: request.code,
                left: 100.0,
                top: 30.0,
                width: 90.0,
                height: 72.0,
                rows: 5,
            })
        );
        assert_eq!(watch.next(request, Some(rect)), None);
        assert_eq!(watch.next(request, None), None);
        assert!(watch.next(request, Some(rect)).is_some());
        let moving = FixtureRequest {
            scenario: Scenario::FleetMove,
            ..request
        };
        assert_eq!(SpeedMenuWatch::default().next(moving, Some(rect)), None);
    }

    #[test]
    fn the_fleet_load_setup_reports_the_day_readouts_centre() {
        // GameSpeed: the readout is the Game Speed control (day_readout_rect).
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let loading = request(Scenario::FleetLoad, faction);
            let applied = apply_to_loading_world(loading);
            let setup =
                fleet_load_setup(loading, &applied.world, &applied.sectors, &applied.defenses)
                    .unwrap();
            let day = day_readout_rect(faction);
            assert_eq!(
                setup.day_readout,
                (day.x + day.width / 2.0, day.y + day.height / 2.0),
                "{faction:?}"
            );
            assert_eq!(setup.status, "fleet-load-setup");
            let moving = request(Scenario::FleetMove, faction);
            assert_eq!(
                fleet_load_setup(moving, &applied.world, &applied.sectors, &applied.defenses),
                None
            );
        }
    }

    #[test]
    fn the_full_fleet_load_scenario_leaves_no_room_for_the_regiment() {
        // FUN_00500b40: the room is the capacity less the regiments aboard.
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let mut applied = apply_to_loading_world(request(Scenario::FleetLoadFull, faction));
            let here = applied.world.systems.keys().next().unwrap();
            let fleet = applied.world.fleets.keys().next().unwrap();
            let capacity = TroopTransportState::fleet_capacity(&applied.world, fleet).unwrap();
            assert_eq!(applied.transport.carried_count(fleet), capacity as usize);
            let regiment = loading_regiment(&applied.world, here).unwrap();
            assert_eq!(applied.world.systems[here].ground_units, [regiment]);
            assert_eq!(
                applied.transport.load(
                    &mut applied.world,
                    &ProductRoom::default(),
                    fleet,
                    &[regiment]
                ),
                Err(
                    rebellion_core::troop_transport::TroopTransportError::CapacityExceeded {
                        capacity,
                        requested: capacity + 1,
                    }
                )
            );
        }
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
            // The Fleet window's top-left corner, 5 pixels in from the
            // galaxy view's top-right.
            let galaxy = layout.galaxy;
            let corner = (
                galaxy.x + galaxy.width - FLEET_WINDOW_WIDTH - 5.0,
                galaxy.y + 5.0,
            );
            assert!(applied.fleets.contains_screen_point(layout, corner));
            assert!(!applied
                .fleets
                .contains_screen_point(layout, (corner.0 - 1.0, corner.1)));
            assert!(!applied
                .fleets
                .contains_screen_point(layout, (corner.0, corner.1 - 1.0)));
            let setup = fleet_move_setup(request, world, &applied.sectors, &applied.fleets)
                .expect("the scenario reports its points");
            let item = (setup.fleet_item_x, setup.fleet_item_y);
            let planet = (setup.target_screen_x, setup.target_screen_y);
            assert!(applied.fleets.contains_screen_point(layout, item));
            assert!(applied.sectors.contains_screen_point(layout, planet));
            assert!(!applied.fleets.contains_screen_point(layout, planet));
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
            fleet_move_setup(request, &applied.world, &applied.sectors, &applied.fleets),
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

    /// The quadrant scenario on the loading world (three systems, the third
    /// in its own sector) plus a fourth in that sector, with one Alliance
    /// fleet that carries the world's first agent, a bystander fleet and
    /// agent at the fourth system, an agent waiting at the second system,
    /// and three more agents of each side nowhere.
    fn apply_quadrants(faction: CockpitFaction) -> Applied {
        apply_to_loading_world_with(request(Scenario::Quadrants, faction), |world| {
            use rebellion_core::world::{Character, Fleet};
            let carried = world.characters.keys().next().unwrap();
            let mut keys = world.systems.keys();
            let (second, third) = (keys.nth(1).unwrap(), keys.next().unwrap());
            let mut fourth = world.systems[third].clone();
            fourth.dat_id = rebellion_core::ids::DatId::new(0x9000_0004);
            let fourth = world.systems.insert(fourth);
            let far = world.systems[third].sector;
            world.sectors[far].systems.push(fourth);
            world.characters[carried].current_system = Some(third);
            for (location, is_alliance) in [(fourth, false), (fourth, true)] {
                world.fleets.insert(Fleet {
                    location,
                    capital_ships: Vec::new(),
                    fighters: Vec::new(),
                    characters: Vec::new(),
                    is_alliance,
                    bombarding: false,
                });
            }
            for (name, location) in [("Bystander", fourth), ("Waiting", second)] {
                world.characters.insert(Character {
                    name: name.into(),
                    is_alliance: false,
                    is_empire: false,
                    current_system: Some(location),
                    ..Default::default()
                });
            }
            world.fleets.insert(Fleet {
                location: third,
                capital_ships: Vec::new(),
                fighters: Vec::new(),
                characters: vec![carried],
                is_alliance: true,
                bombarding: false,
            });
            for (index, is_alliance) in [true, true, true, false, false, false]
                .into_iter()
                .enumerate()
            {
                world.characters.insert(Character {
                    name: format!("Agent {index}"),
                    is_alliance,
                    is_empire: !is_alliance,
                    recruited: true,
                    ..Default::default()
                });
            }
        })
    }

    #[test]
    fn the_quadrant_scenario_gives_each_checked_icon_its_side() {
        // port: the fixture's own layout; the sides follow FUN_0045cdc0,
        // FUN_0045ce80, FUN_0045ccc0 and FUN_004a1f60.
        use rebellion_render::quadrant_icons::quadrant_side;
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let applied = apply_quadrants(faction);
            let world = &applied.world;
            let mut keys = world.systems.keys();
            let (primary, second) = (keys.next().unwrap(), keys.next().unwrap());
            let (player, own, other) = match faction {
                CockpitFaction::Alliance => (Faction::Alliance, 1, 2),
                CockpitFaction::Empire => (Faction::Empire, 2, 1),
            };
            let fog = FogState::new(player);
            let side = |system, quadrant| {
                quadrant_side(
                    world,
                    &fog,
                    &applied.missions,
                    &applied.movement,
                    player,
                    system,
                    quadrant,
                )
            };
            assert_eq!(side(primary, Quadrant::System), Some(own), "{faction:?}");
            assert_eq!(side(primary, Quadrant::Defenses), Some(own), "{faction:?}");
            assert_eq!(side(primary, Quadrant::Fleets), Some(own), "{faction:?}");
            assert_eq!(
                side(primary, Quadrant::Missions),
                Some(other),
                "{faction:?}"
            );
            assert_eq!(side(second, Quadrant::System), None, "{faction:?}");
            assert_eq!(side(second, Quadrant::Defenses), Some(other), "{faction:?}");
            assert_eq!(side(second, Quadrant::Fleets), None, "{faction:?}");
            assert_eq!(side(second, Quadrant::Missions), Some(own), "{faction:?}");
            assert_eq!(applied.missions.missions().len(), 3, "{faction:?}");
            assert_eq!(applied.sectors.window_count(), 1, "{faction:?}");

            // Only what the scenario placed lies at the two systems.
            let is_alliance = faction == CockpitFaction::Alliance;
            let own_fleets: Vec<_> = world
                .fleets
                .iter()
                .filter(|(_, fleet)| fleet.location == primary)
                .map(|(_, fleet)| fleet.is_alliance)
                .collect();
            assert_eq!(own_fleets, [is_alliance], "{faction:?}");
            assert!(world.fleets.values().all(|fleet| fleet.location != second));
            let at = |system| {
                world
                    .characters
                    .values()
                    .filter(|character| character.current_system == Some(system))
                    .count()
            };
            assert_eq!((at(primary), at(second)), (2, 1), "{faction:?}");
            // What lies at the fourth system stays there.
            let fourth = world.systems.keys().nth(3).unwrap();
            assert_eq!(at(fourth), 1, "{faction:?}");
            assert_eq!(
                world
                    .fleets
                    .values()
                    .filter(|fleet| fleet.location == fourth)
                    .count(),
                2,
                "{faction:?}"
            );
            // Each mission is sent by its agent's side.
            for mission in applied.missions.missions() {
                let MissionMember::Character(agent) = mission.team[0] else {
                    panic!("the scenario sends characters");
                };
                let sent_by_alliance = mission.faction == MissionFaction::Alliance;
                assert_eq!(
                    sent_by_alliance, world.characters[agent].is_alliance,
                    "{faction:?}"
                );
            }
        }
    }

    #[test]
    fn the_quadrant_scenario_leaves_an_agent_aboard_a_fleet_where_it_is() {
        let applied = apply_quadrants(CockpitFaction::Alliance);
        let world = &applied.world;
        let carried = world.characters.keys().next().unwrap();
        let third = world.systems.keys().nth(2).unwrap();
        assert_eq!(world.characters[carried].current_system, Some(third));
        assert!(!world.characters[carried].on_mission);
        assert!(applied
            .missions
            .missions()
            .iter()
            .all(|mission| !mission.team.contains(&MissionMember::Character(carried))));
    }

    #[test]
    fn the_quadrant_setup_reports_both_systems_four_rects_in_kind_order() {
        let applied = apply_quadrants(CockpitFaction::Empire);
        let request = request(Scenario::Quadrants, CockpitFaction::Empire);
        let report = quadrant_setup(request, &applied.world, &applied.sectors).unwrap();
        let world = &applied.world;
        let mut keys = world.systems.keys();
        let (primary, second) = (keys.next().unwrap(), keys.next().unwrap());
        let layout = CockpitState::new(CockpitFaction::Empire).layout_for(640.0, 480.0);
        assert_eq!(report.status, "quadrant-setup");
        assert_eq!(report.primary_dat_id, world.systems[primary].dat_id.raw());
        assert_eq!(report.second_dat_id, world.systems[second].dat_id.raw());
        assert_eq!(report.scale, layout.scale);
        let names = ["system", "defenses", "fleets", "missions"];
        let kinds = [
            Quadrant::System,
            Quadrant::Defenses,
            Quadrant::Fleets,
            Quadrant::Missions,
        ];
        let mut expected = Vec::new();
        for system in [primary, second] {
            for (quadrant, name) in kinds.into_iter().zip(names) {
                let rect = applied
                    .sectors
                    .quadrant_screen_rect(world, layout, system, quadrant)
                    .unwrap();
                expected.push(FixtureQuadrant {
                    system_dat_id: world.systems[system].dat_id.raw(),
                    quadrant: name,
                    rect: [rect.min.x, rect.min.y, rect.width(), rect.height()],
                });
            }
        }
        assert_eq!(report.quadrants, expected);
        assert!(quadrant_setup(
            super::tests::request(Scenario::FleetLoad, CockpitFaction::Empire),
            world,
            &applied.sectors
        )
        .is_none());
    }

    #[test]
    fn the_quadrant_watch_reports_the_open_windows_and_the_rail_when_they_change() {
        let mut applied = apply_quadrants(CockpitFaction::Alliance);
        let request = request(Scenario::Quadrants, CockpitFaction::Alliance);
        let primary = applied.world.systems.keys().next().unwrap();
        let mut fog = FogState::new(Faction::Alliance);
        fog.reveal(primary);
        let economy = EconomyState::default();
        let mut defenses = DefensesWindowState::default();
        let mut missions_windows = MissionsWindowState::default();
        let mut watch = QuadrantWatch::default();
        macro_rules! next {
            ($watch:expr) => {
                $watch.next(
                    request,
                    &applied.world,
                    &QuadrantWindows {
                        systems: &applied.systems,
                        defenses: &defenses,
                        missions_windows: &missions_windows,
                        fog: &fog,
                        missions: &applied.missions,
                        economy: &economy,
                    },
                )
            };
        }
        let first = next!(watch).unwrap();
        assert_eq!(first.status, "quadrant-observation");
        assert!(first.system_windows.is_empty() && first.defenses_windows.is_empty());
        assert!(first.missions_windows.is_empty());
        assert!(first.rail.is_empty());
        assert!(next!(watch).is_none());

        let layout = CockpitState::new(CockpitFaction::Alliance).layout_for(640.0, 480.0);
        let dat = applied.world.systems[primary].dat_id.raw();
        assert!(applied.systems.open(
            &applied.world,
            primary,
            (100, 50),
            CockpitFaction::Alliance,
            layout
        ));
        let opened = next!(watch).unwrap();
        assert_eq!(opened.system_windows, [(dat, (100, 50))]);

        // The primary system's Defenses window: the player's regiment and
        // KDY-150, the garrison line on the regiment page, and each tab's
        // and listed cell's rect.
        assert!(defenses.open(
            &applied.world,
            primary,
            (200, 60),
            CockpitFaction::Alliance,
            layout
        ));
        let shown = next!(watch).unwrap();
        let [window] = &shown.defenses_windows[..] else {
            panic!("{:?}", shown.defenses_windows);
        };
        assert_eq!(window.system_dat_id, dat);
        assert_eq!(window.origin, (200, 60));
        assert_eq!((window.side, window.page), (1, "personnel"));
        assert_eq!(window.counts, [0, 1, 0, 0, 1]);
        assert!(window.rows.is_empty() && window.cells.is_empty());
        assert_eq!((window.selected, window.garrison.as_deref()), (None, None));
        let tab = |name: &str| {
            window
                .tabs
                .iter()
                .find(|(tab, _)| *tab == name)
                .map(|(_, rect)| *rect)
        };
        let origin = |x: f32, y: f32| {
            [
                layout.canvas.x + (200.0 + x) * layout.scale,
                layout.canvas.y + (60.0 + y) * layout.scale,
            ]
        };
        let [left, top] = origin(64.0, 20.0);
        assert_eq!(
            tab("regiments"),
            Some([left, top, 36.0 * layout.scale, 33.0 * layout.scale])
        );
        assert_eq!(window.tabs.len(), 5);

        applied.systems.minimize_defenses_window(primary, (200, 60));
        let railed = next!(watch).unwrap();
        let slot = applied.systems.rail_slot_screen_rect(layout, 0).unwrap();
        assert_eq!(railed.rail, [(dat, "defenses", super::rect_array(slot))]);

        // The primary system's Missions window: the player's Diplomacy
        // mission first (selected, its agent listed), then the other
        // side's; both target the system.
        assert!(missions_windows.open(
            &applied.world,
            &fog,
            &applied.missions,
            primary,
            (150, 40),
            CockpitFaction::Alliance,
            layout
        ));
        let shown = next!(watch).unwrap();
        let [window] = &shown.missions_windows[..] else {
            panic!("{:?}", shown.missions_windows);
        };
        assert_eq!((window.system_dat_id, window.origin), (dat, (150, 40)));
        assert_eq!(
            window.rows,
            [
                ("Diplomacy".to_string(), 1, 0x4c10),
                ("Diplomacy".to_string(), 2, 0x5c10)
            ]
        );
        assert_eq!(window.side, 2);
        assert_eq!(
            (window.selected, window.tab, window.tab_side),
            (Some(0), "agents", Some(1))
        );
        assert_eq!(window.members.len(), 1);
        assert_eq!(window.member_rects.len(), 1);
        assert_eq!(
            window.target.as_deref(),
            Some(applied.world.systems[primary].name.as_str())
        );
        let [left, top] = [
            layout.canvas.x + (150.0 + 5.0) * layout.scale,
            layout.canvas.y + (40.0 + 74.0) * layout.scale,
        ];
        assert_eq!(
            window.row_rects[1],
            [left, top, 90.0 * layout.scale, 50.0 * layout.scale]
        );
        assert_eq!(window.tabs.len(), 2);

        applied.systems.minimize_missions_window(primary, (150, 40));
        let railed = next!(watch).unwrap();
        assert_eq!(railed.rail.last().map(|entry| entry.1), Some("missions"));

        let other = super::tests::request(Scenario::FleetLoad, CockpitFaction::Alliance);
        assert!(QuadrantWatch::default()
            .next(
                other,
                &applied.world,
                &QuadrantWindows {
                    systems: &applied.systems,
                    defenses: &defenses,
                    missions_windows: &missions_windows,
                    fog: &fog,
                    missions: &applied.missions,
                    economy: &economy,
                },
            )
            .is_none());
    }

    #[test]
    fn the_targeting_fleet_move_and_fleet_window_scenarios_report_the_object_menu() {
        let reporting: Vec<_> = (0..SCENARIO_COUNT)
            .filter_map(Scenario::decode)
            .filter(|scenario| scenario.reports_object_menu())
            .collect();

        assert_eq!(
            reporting,
            [
                Scenario::MissionTargeting,
                Scenario::FleetMove,
                Scenario::FleetMoveBlockade,
                Scenario::FleetLoad,
                Scenario::FleetLoadFull,
                Scenario::RegimentUnloadRefused,
                Scenario::FleetJoin,
                Scenario::ProductionDestination,
                Scenario::BuildSelection,
                Scenario::StrikeDeathStar,
                Scenario::UnfinishedShip,
                Scenario::ShipBesideUnfinished,
                Scenario::StrikeShielded,
                Scenario::AssaultCaptures,
                Scenario::AssaultRepelled,
                Scenario::StrikeDamagedSurvivor,
                Scenario::StrikeDamagedLoss
            ]
        );
    }

    /// The player's fleet's strike in `applied`, through the integrator the
    /// menu row reaches (`strike_orders::issue`).
    fn strike(
        applied: &mut Applied,
        kind: rebellion_core::bombardment::BombardKind,
    ) -> rebellion_data::integrator::StrikeEffects {
        let fleet = applied.world.fleets.keys().next().unwrap();
        let mut manufacturing = ManufacturingState::default();
        let mut deliveries = rebellion_core::delivery::DeliveryState::new();
        let uprisings = UprisingState::new();
        rebellion_data::integrator::order_strike(
            &mut applied.world,
            &[fleet],
            kind,
            &mut rebellion_data::integrator::StrikeStates {
                movement: &mut applied.movement,
                manufacturing: &mut manufacturing,
                transport: &mut applied.transport,
                deliveries: &mut deliveries,
                missions: &mut applied.missions,
                uprisings: &uprisings,
                blockade: &mut applied.blockade,
                tick: 2,
                step: 0,
            },
            &mut || 0.5,
        )
        .expect("the fixture's strike runs")
        .into_iter()
        .next()
        .expect("one run")
    }

    /// The player's fleet's assault in `applied` (`assault_orders::issue`).
    fn assault(
        applied: &mut Applied,
        faction: CockpitFaction,
    ) -> rebellion_data::integrator::AssaultEffects {
        let fleet = applied.world.fleets.keys().next().unwrap();
        let mut manufacturing = ManufacturingState::default();
        let mut deliveries = rebellion_core::delivery::DeliveryState::new();
        let uprisings = UprisingState::new();
        rebellion_data::integrator::order_assault(
            &mut applied.world,
            &[fleet],
            Faction::of_alliance(faction == CockpitFaction::Alliance),
            &mut rebellion_data::integrator::StrikeStates {
                movement: &mut applied.movement,
                manufacturing: &mut manufacturing,
                transport: &mut applied.transport,
                deliveries: &mut deliveries,
                missions: &mut applied.missions,
                uprisings: &uprisings,
                blockade: &mut applied.blockade,
                tick: 2,
                step: 0,
            },
            &mut || 0.5,
        )
        .expect("the fixture's assault runs")
        .into_iter()
        .next()
        .expect("one run")
    }

    #[test]
    fn the_live_report_observation_reports_the_shown_report_and_where_it_sent_the_player() {
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let request = request(Scenario::StrikeDamagedSurvivor, faction);
            let mut applied = apply_to_fleet_world(request);
            let run = strike(&mut applied, rebellion_core::bombardment::BombardKind::General);
            let operation = std::sync::Arc::new(run.operation.expect("the report"));
            let primary = operation.system;
            let message = rebellion_render::GameMessage::at_system(
                2,
                "Orbital bombardment",
                rebellion_render::MessageCategory::Combat,
                primary,
            )
            .with_report(std::sync::Arc::clone(&operation));
            let mut state = rebellion_render::OperationReportState::default();
            let mut watch = OperationReportWatch::default();
            let observe = |watch: &mut OperationReportWatch,
                           state: &rebellion_render::OperationReportState,
                           applied: &Applied,
                           chosen| {
                watch
                    .next_live(
                        request,
                        state,
                        &applied.world,
                        &applied.movement,
                        &ReportDestinations {
                            sectors: &applied.sectors,
                            fleets: &applied.fleets,
                            chosen,
                        },
                    )
                    .map(|bytes| serde_json::from_slice::<serde_json::Value>(&bytes).unwrap())
            };

            let closed = observe(&mut watch, &state, &applied, None).expect("the first frame");
            assert_eq!(closed["open"], false, "{faction:?}");
            assert!(observe(&mut watch, &state, &applied, None).is_none(), "{faction:?}");

            assert!(state.open(&applied.world, &message, faction, None));
            let shown = observe(&mut watch, &state, &applied, None).expect("the report opens");
            assert_eq!(shown["live"], true, "{faction:?}");
            assert_eq!(shown["open"], true, "{faction:?}");
            assert_eq!(shown["title"], "Orbital bombardment", "{faction:?}");
            assert_eq!(shown["picture"], operation.picture(), "{faction:?}");
            assert_eq!(shown["assault"], false, "{faction:?}");
            assert_eq!(
                shown["fleet_operational"].as_array().map(Vec::len),
                Some(operation.fleet_operational.len()),
                "{faction:?}"
            );
            assert_eq!(shown["destination"], serde_json::Value::Null, "{faction:?}");

            state.close();
            let sent = observe(&mut watch, &state, &applied, Some((primary, true)))
                .expect("the Fleet button closed it");
            let name = &applied.world.systems[primary].name;
            assert_eq!(sent["open"], false, "{faction:?}");
            assert_eq!(sent["destination"], format!("fleet:{name}"), "{faction:?}");
            assert_eq!(sent["fleet_window_open"], true, "{faction:?}");
            assert_eq!(sent["sector_open"], true, "{faction:?}");
            let sent = observe(&mut watch, &state, &applied, Some((primary, false)))
                .expect("the System button");
            assert_eq!(sent["destination"], format!("sector:{name}"), "{faction:?}");
        }
    }

    // FUN_0042c3b0.c:350-410: a damaged object's cell stacks its GOKRES
    // damage art, portrait + 0x2000, under the portrait.
    #[test]
    fn a_report_cell_names_its_damage_art_under_its_portrait() {
        use rebellion_core::operation_report::{ReportObject, ReportUnit};
        let applied = apply_to_fleet_world(request(Scenario::StrikeDamagedLoss, CockpitFaction::Alliance));
        let corvette = |damaged| ReportUnit {
            dat_id: rebellion_core::ids::DatId::new(0x1400_0045),
            name: "Corellian Corvette 1".into(),
            major: false,
            object: Some(ReportObject::Destroyed { damaged }),
        };
        let cell = report_cell(&applied.world, &applied.movement, &corvette(true));
        assert_eq!(cell.name, "Corellian Corvette 1");
        assert_eq!(cell.family, 0x14);
        assert_eq!(
            cell.layers,
            [("Gokres".to_owned(), 1861 + 0x2000), ("Gokres".to_owned(), 1861)]
        );
        let cell = report_cell(&applied.world, &applied.movement, &corvette(false));
        assert_eq!(cell.layers, [("Gokres".to_owned(), 1861)]);
    }

    #[test]
    fn the_fixture_picks_a_ship_class_of_the_players_side_that_suits_or_adds_one() {
        use rebellion_core::world::CapitalShipClass;
        let mut world = GameWorld::default();
        let class = |is_alliance, family, hyperdrive, bombardment_modifier| CapitalShipClass {
            is_alliance,
            family,
            hyperdrive,
            bombardment_modifier,
            ..CapitalShipClass::default()
        };
        // A Death Star, a ship that cannot bomb and one with no hyperdrive
        // come first and are passed over.
        world.capital_ship_classes.insert(class(false, 0x18, 80, 15));
        let empire = world.capital_ship_classes.insert(class(false, 0x14, 80, 2));
        world.capital_ship_classes.insert(class(true, 0x14, 80, 0));
        world.capital_ship_classes.insert(class(true, 0x14, 0, 2));
        let alliance = world.capital_ship_classes.insert(class(true, 0x14, 80, 2));
        let bombs = |class: &CapitalShipClass| class.bombardment_modifier > 0;
        let template = || class(false, 0x14, 80, 9);
        assert_eq!(fixture_ship_class(&mut world, false, bombs, template()), empire);
        assert_eq!(fixture_ship_class(&mut world, true, bombs, template()), alliance);

        let added = fixture_ship_class(&mut world, true, |class| class.troop_capacity > 0, template());
        assert_eq!(world.capital_ship_classes.len(), 6);
        assert!(world.capital_ship_classes[added].is_alliance);
        assert_eq!(world.capital_ship_classes[added].bombardment_modifier, 9);
    }

    #[test]
    fn the_fixture_regiment_class_is_one_of_the_sides_or_its_first_regiment() {
        let mut world = GameWorld::default();
        let army = rebellion_core::ids::DatId::new(0x1000_0001);
        let stormtroopers = rebellion_core::ids::DatId::new(0x1000_0006);
        assert_eq!(fixture_regiment_class(&world, true), army);
        assert_eq!(fixture_regiment_class(&world, false), stormtroopers);
        let fleet_regiment = rebellion_core::ids::DatId::new(0x1000_0003);
        world.troops.insert(TroopUnit {
            class_dat_id: fleet_regiment,
            is_alliance: true,
        });
        assert_eq!(fixture_regiment_class(&world, true), fleet_regiment);
        assert_eq!(fixture_regiment_class(&world, false), stormtroopers);
    }

    #[test]
    fn the_fixture_defence_class_is_the_familys_strongest_or_one_added() {
        use rebellion_core::ids::DatId;
        use rebellion_core::world::{BuildableClass, ClassStats};
        let mut world = GameWorld::default();
        for (raw, shield, attack) in [
            (0x2400_0003, 40, 0),
            (0x2400_0006, 80, 0),
            (0x2400_0007, 0, 900),
            (0x2300_0002, 0, 800),
            (0x2200_0001, 90, 2000),
            (0x2600_0004, 0, 0),
        ] {
            world.buildable_classes.insert(
                DatId::new(raw),
                BuildableClass {
                    stats: ClassStats {
                        shield_strength: shield,
                        attack_strength: attack,
                        ..ClassStats::default()
                    },
                    ..BuildableClass::default()
                },
            );
        }
        assert_eq!(
            fixture_defence_class(&mut world, 0x24, |stats| stats.shield_strength),
            (DatId::new(0x2400_0006), 80)
        );
        assert_eq!(
            fixture_defence_class(&mut world, 0x23, |stats| stats.attack_strength),
            (DatId::new(0x2300_0002), 800)
        );
        assert_eq!(
            fixture_defence_class(&mut world, 0x25, |stats| stats.shield_strength),
            (DatId::new(0x2500_0001), 40)
        );
        assert_eq!(
            fixture_defence_class(&mut world, 0x26, |stats| stats.shield_strength),
            (DatId::new(0x2600_0001), 40)
        );
        assert_eq!(world.buildable_classes.len(), 8);
    }

    #[test]
    fn the_fixture_bombing_squadron_is_one_of_the_sides_that_bombs_or_one_added() {
        use rebellion_core::world::FighterClass;
        let mut world = GameWorld::default();
        let class = |is_alliance, bombardment_defense| FighterClass {
            is_alliance,
            bombardment_defense,
            ..FighterClass::default()
        };
        let tie_bomber = world.fighter_classes.insert(class(false, 2));
        world.fighter_classes.insert(class(true, 0));
        let y_wing = world.fighter_classes.insert(class(true, 2));
        assert_eq!(fixture_bombing_squadron(&mut world, false), tie_bomber);
        assert_eq!(fixture_bombing_squadron(&mut world, true), y_wing);

        for alliance in [true, false] {
            let mut world = GameWorld::default();
            world.fighter_classes.insert(class(alliance, 0));
            let added = fixture_bombing_squadron(&mut world, alliance);
            assert_eq!(world.fighter_classes.len(), 2);
            let added = &world.fighter_classes[added];
            assert_eq!(added.name, "Bomber Squadron");
            assert_eq!((added.is_alliance, added.is_empire), (alliance, !alliance));
            assert_eq!(added.bombardment_defense, 2);
        }
    }

    // FUN_0058df60: the holder's family-0x24 generators' shields come off the
    // hits, so the fixture's generators leave none (bombardment.md, "Effect
    // phase").
    #[test]
    fn the_shielded_strike_fixture_lands_no_hit_for_either_side() {
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let alliance = faction == CockpitFaction::Alliance;
            let mut bomber = None;
            let mut applied =
                apply_to_fleet_world_with(request(Scenario::StrikeShielded, faction), |world| {
                    bomber = Some(world.capital_ship_classes.insert(
                        rebellion_core::world::CapitalShipClass {
                            is_alliance: alliance,
                            hull: 600,
                            hyperdrive: 80,
                            bombardment_modifier: 3,
                            ..rebellion_core::world::CapitalShipClass::default()
                        },
                    ));
                });
            let primary = applied.world.systems.keys().next().unwrap();
            let fleet = applied.world.fleets.keys().next().unwrap();
            let ships = &applied.world.fleets[fleet].capital_ships;
            assert_eq!(ships.len(), 1, "{faction:?}");
            assert_eq!(Some(ships[0].class), bomber, "{faction:?}");
            let garrison = &applied.world.systems[primary].ground_units;
            assert_eq!(garrison.len(), 1, "{faction:?}");
            assert_eq!(applied.world.troops[garrison[0]].is_alliance, !alliance, "{faction:?}");
            let generators = &applied.world.systems[primary].defense_facilities;
            assert!(!generators.is_empty(), "{faction:?}");
            assert!(
                generators
                    .iter()
                    .all(|&key| applied.world.defense_facilities[key].class_dat_id.family() == 0x24),
                "{faction:?}"
            );
            let run = strike(&mut applied, rebellion_core::bombardment::BombardKind::Military);
            let report = run.report.expect("a run");
            assert!(
                rebellion_core::bombardment::fleet_strength(
                    &applied.world,
                    applied.world.fleets.keys().next().unwrap()
                ) > 0,
                "{faction:?}"
            );
            assert_eq!(report.hits, 0, "{faction:?}");
            assert!(report.system_destroyed.is_empty(), "{faction:?}");
        }
    }

    /// The shipped GNPRTB the assault reads: `0x200`-`0x205` (5, 30, 10, 4,
    /// 6, 8), the shield limit `0x1e03` (2) and the landing's garrison
    /// figures 7761 (60) and 7762 (-10).
    fn assault_gnprtb(world: &mut GameWorld) {
        let entry = |id: u32, value: i32| rebellion_core::world::GnprtbEntry {
            parameter_id: id,
            development: value,
            alliance_sp_easy: value,
            alliance_sp_medium: value,
            alliance_sp_hard: value,
            empire_sp_easy: value,
            empire_sp_medium: value,
            empire_sp_hard: value,
            multiplayer: value,
        };
        world.gnprtb = rebellion_core::world::GnprtbParams::new(
            [
                (0x200, 5),
                (0x201, 30),
                (0x202, 10),
                (0x203, 4),
                (0x204, 6),
                (0x205, 8),
                (0x1e03, 2),
                (7761, 60),
                (7762, -10),
            ]
                .into_iter()
                .map(|(id, value)| entry(id, value))
                .collect(),
        );
    }

    // FUN_00563390: with no defender left standing the assault takes the
    // system; against six defenders one regiment cannot (ground-assault.md).
    #[test]
    fn the_assault_fixtures_capture_the_empty_garrison_and_lose_to_the_full_one() {
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let player = Faction::of_alliance(faction == CockpitFaction::Alliance);
            let holder = Faction::of_alliance(faction != CockpitFaction::Alliance);
            let mut applied =
                apply_to_fleet_world_with(request(Scenario::AssaultCaptures, faction), assault_gnprtb);
            let fleet = applied.world.fleets.keys().next().unwrap();
            let class = &applied.world.capital_ship_classes
                [applied.world.fleets[fleet].capital_ships[0].class];
            assert_eq!(
                (class.name.as_str(), class.hull, class.hyperdrive, class.troop_capacity),
                ("Transport", 100, 80, 2),
                "{faction:?}"
            );
            let report = assault(&mut applied, faction).report.expect("a run");
            assert!(report.attackers >= 2, "{faction:?}");
            assert_eq!(report.defenders, 0, "{faction:?}");
            assert_eq!(report.landed.len(), 2, "{faction:?}");
            assert_eq!(report.holder_after, Some(player), "{faction:?}");

            let mut applied =
                apply_to_fleet_world_with(request(Scenario::AssaultRepelled, faction), assault_gnprtb);
            let report = assault(&mut applied, faction).report.expect("a run");
            assert_eq!((report.attackers, report.defenders), (1, 6), "{faction:?}");
            assert_eq!(report.holder_after, Some(holder), "{faction:?}");
        }
    }

    // FUN_00501c00 sets +0x50 bit 9 on a hull below its maximum, and
    // FUN_0042c3b0.c:350-410 draws the damage art for it in either column;
    // the batteries' only target dies with the bit (FUN_0058d540).
    #[test]
    fn the_damaged_strike_fixtures_list_a_damaged_survivor_and_a_damaged_loss() {
        use rebellion_core::operation_report::ReportObject;
        use rebellion_render::operation_report::cell_status;
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let mut applied =
                apply_to_fleet_world(request(Scenario::StrikeDamagedSurvivor, faction));
            let fleet = applied.world.fleets.keys().next().unwrap();
            let ship = &applied.world.fleets[fleet].capital_ships[0];
            let class = &applied.world.capital_ship_classes[ship.class];
            assert_eq!(
                (class.name.as_str(), class.hull, class.hyperdrive, ship.hull_current),
                ("Bomber", 1000, 80, 500),
                "{faction:?}"
            );
            let run = strike(&mut applied, rebellion_core::bombardment::BombardKind::General);
            let operation = run.operation.expect("the report");
            assert_eq!(run.report.expect("a run").ships_destroyed, 0, "{faction:?}");
            let ship = operation
                .fleet_operational
                .iter()
                .find(|unit| matches!(unit.object, Some(ReportObject::Ship { tag, .. }) if tag != 0))
                .expect("the ship is listed operational");
            assert!(
                cell_status(&applied.world, &applied.movement, ship).damaged,
                "{faction:?}"
            );

            let mut applied = apply_to_fleet_world(request(Scenario::StrikeDamagedLoss, faction));
            let run = strike(&mut applied, rebellion_core::bombardment::BombardKind::Military);
            assert_eq!(run.report.expect("a run").ships_destroyed, 1, "{faction:?}");
            let operation = run.operation.expect("the report");
            // The ship's squadron dies with it and is listed beside it with
            // no object state.
            assert_eq!(
                operation
                    .fleet_destroyed
                    .iter()
                    .filter(|unit| matches!(unit.object, Some(ReportObject::Destroyed { .. })))
                    .map(|unit| unit.object.clone())
                    .collect::<Vec<_>>(),
                [Some(ReportObject::Destroyed { damaged: true })],
                "{faction:?}"
            );
        }
    }

    fn finder_observation(
        applied: &Applied,
        finder: &FleetFinderState,
        faction: CockpitFaction,
    ) -> FleetFinderObservation {
        let player = if faction == CockpitFaction::Alliance {
            Faction::Alliance
        } else {
            Faction::Empire
        };
        fleet_finder_observation(
            request(Scenario::FleetFinder, faction),
            &applied.world,
            &FinderWindows {
                fog: &FogState::new(player),
                movement: &applied.movement,
                finder,
                fleets: &FleetWindowState::default(),
                sectors: &applied.sectors,
                map: &applied.map,
                left_panel_open: false,
            },
        )
        .unwrap()
    }

    // port: the fixture's own layout; the rows follow FUN_00462be0.
    #[test]
    fn the_fleet_finder_scenario_lists_a_second_own_fleet_and_the_other_sides_fleet_at_home() {
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let applied = apply_to_fleet_world(request(Scenario::FleetFinder, faction));
            let world = &applied.world;
            let is_alliance = faction == CockpitFaction::Alliance;
            let keys: Vec<_> = world.systems.keys().take(10).collect();
            let (home, third) = (
                world.systems[keys[0]].dat_id.raw(),
                world.systems[keys[2]].dat_id.raw(),
            );
            assert_eq!(world.fleets.len(), 5);
            let mut finder = FleetFinderState::default();
            assert!(finder_observation(&applied, &finder, faction)
                .rows
                .is_empty());
            finder.open(faction);

            let observed = finder_observation(&applied, &finder, faction);

            assert!(observed.open);
            assert_eq!(observed.mode, Some("fleets"));
            assert_eq!(observed.tab, Some("all"));
            let row = |side: bool, system: u32| {
                observed
                    .rows
                    .iter()
                    .any(|row| row.is_alliance == side && row.system_dat_id == Some(system))
            };
            assert!(row(!is_alliance, home), "{faction:?}: {:?}", observed.rows);
            assert!(row(is_alliance, third), "{faction:?}: {:?}", observed.rows);
            let controls = observed.controls.unwrap();
            assert_eq!(controls.rows.len(), observed.rows.len());
            assert!(controls.rows.iter().all(Option::is_some));
            assert_eq!(observed.player_is_alliance, is_alliance);
            // FUN_00427270: the control's rectangle, (157, 407) 27 by 15 for
            // the Alliance and (199, 434) 37 by 24 for the Empire.
            let button = if is_alliance {
                (170.5, 414.5)
            } else {
                (217.5, 446.0)
            };
            assert_eq!(observed.cockpit_button, button);
            // The first fleet stays first; the other side's copy is the last.
            let fleet = |ordinal| observed.rows.iter().find(|row| row.fleet == Some(ordinal));
            let first = fleet(0).unwrap();
            assert_eq!(
                (first.kind, first.ship, first.is_alliance),
                ("fleet", None, is_alliance)
            );
            assert_eq!(fleet(4).map(|row| row.is_alliance), Some(!is_alliance));
        }
    }

    #[test]
    fn the_finder_observation_names_ships_and_the_fleet_windows_selection() {
        let faction = CockpitFaction::Alliance;
        let mut applied = apply_to_fleet_world(request(Scenario::FleetFinder, faction));
        let class = applied.world.capital_ship_classes.keys().next().unwrap();
        let first = applied.world.fleets.keys().next().unwrap();
        let home = applied.world.fleets[first].location;
        applied.world.fleets[first].capital_ships = vec![
            ShipInstance::new(class, 100, true),
            ShipInstance::new(class, 100, true),
        ];
        let mut finder = FleetFinderState::default();
        finder.open(faction);
        let layout = CockpitState::new(faction).layout_for(640.0, 480.0);
        let mut fleets = FleetWindowState::default();
        assert!(fleets.open(&applied.world, home, (100, 100), faction, layout));
        assert!(fleets.select(
            home,
            FleetWindowEntry::Ship {
                fleet: first,
                index: 1
            }
        ));
        let observe = |finder: &FleetFinderState| {
            fleet_finder_observation(
                request(Scenario::FleetFinder, faction),
                &applied.world,
                &FinderWindows {
                    fog: &FogState::new(Faction::Alliance),
                    movement: &applied.movement,
                    finder,
                    fleets: &fleets,
                    sectors: &applied.sectors,
                    map: &applied.map,
                    left_panel_open: true,
                },
            )
            .unwrap()
        };
        let observed = observe(&finder);

        let window = observed.fleet_windows.first().unwrap();
        assert_eq!(observed.fleet_windows.len(), 1);
        assert_eq!(
            window.system_dat_id,
            applied.world.systems[home].dat_id.raw()
        );
        assert_eq!(
            (window.selected, window.fleet, window.ship),
            (Some("ship"), Some(0), Some(1))
        );
        // FUN_004a2630: the Fleet window is 235 by 304, where it opened.
        let rect = fleets.screen_rect(layout, home).unwrap();
        assert_eq!(window.rect, Some((rect.min.x, rect.min.y, 235.0, 304.0)));
        assert!(observed.left_panel_open);

        finder.set_mode(FinderMode::Ships);
        let observed = observe(&finder);
        let row = observed
            .rows
            .iter()
            .find(|row| row.fleet == Some(0) && row.ship == Some(1));
        assert_eq!(row.map(|row| row.kind), Some("ship"));
    }

    #[test]
    fn the_finder_watch_reports_only_a_changed_finder() {
        let faction = CockpitFaction::Empire;
        let applied = apply_to_fleet_world(request(Scenario::FleetFinder, faction));
        let fog = FogState::new(Faction::Empire);
        let fleets = FleetWindowState::default();
        let mut finder = FleetFinderState::default();
        let mut watch = FleetFinderWatch::default();
        let next = |watch: &mut FleetFinderWatch, scenario, finder: &FleetFinderState| {
            watch.next(
                request(scenario, faction),
                &applied.world,
                &FinderWindows {
                    fog: &fog,
                    movement: &applied.movement,
                    finder,
                    fleets: &fleets,
                    sectors: &applied.sectors,
                    map: &applied.map,
                    left_panel_open: false,
                },
            )
        };

        let finder_scenario = Scenario::FleetFinder;
        assert!(next(&mut watch, finder_scenario, &finder).is_some_and(|observed| !observed.open));
        assert!(next(&mut watch, finder_scenario, &finder).is_none());
        finder.open(faction);
        assert!(next(&mut watch, finder_scenario, &finder).is_some_and(|observed| observed.open));
        assert!(next(&mut watch, finder_scenario, &finder).is_none());
        assert!(next(&mut watch, Scenario::FleetJoin, &finder).is_none());
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
    fn a_system_point_is_its_place_on_the_starfield() {
        // FUN_00425d00: (int)(x * 607 / 1024) + 0x54, (int)(y * 437 / 1024)
        // + 0x1b for the Empire, with no zoom or pan.
        let world = diplomacy_world(Faction::Alliance);
        let mut system = world.systems.values().next().unwrap().clone();
        system.x = 1024;
        system.y = 1024;
        assert_eq!(
            screen_point(&system, CockpitFaction::Empire),
            (84.0 + 607.0, 27.0 + 437.0)
        );
    }

    #[test]
    fn fixture_fingerprint_is_stable() {
        assert_eq!(
            fnv1a64(b"gid/alliance/popular-support"),
            0x432f_ad5e_fe03_453d
        );
    }
}
