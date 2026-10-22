mod assault_orders;
mod audio;
#[cfg(not(target_arch = "wasm32"))]
mod dev_actions;
#[cfg(not(target_arch = "wasm32"))]
mod dev_channel;
#[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
mod dev_commands;
#[cfg(not(target_arch = "wasm32"))]
mod dev_input;
#[cfg(not(target_arch = "wasm32"))]
mod dev_policy;
mod encyclopedia_content;
mod encyclopedia_hd;
mod encyclopedia_mods;
mod encyclopedia_surface;
#[cfg(any(test, all(target_arch = "wasm32", feature = "interface-test-fixtures")))]
#[cfg_attr(
    all(test, not(target_arch = "wasm32")),
    expect(
        dead_code,
        reason = "Native tests cover fixture helpers; the browser bridge is called only by WASM."
    )
)]
mod interface_test_fixture;
mod movies;
#[cfg(any(target_arch = "wasm32", test))]
mod runtime_pack;
mod strategic_windowing;
mod strike_orders;
mod tactical_flow;
#[cfg(any(test, all(target_arch = "wasm32", feature = "interface-test-fixtures")))]
#[cfg_attr(
    all(test, not(target_arch = "wasm32")),
    expect(
        dead_code,
        reason = "Native tests cover fixture code decoding; the bridge is called only by WASM."
    )
)]
mod tactical_test_fixture;
#[cfg(target_arch = "wasm32")]
mod web_accessibility;
#[cfg(target_arch = "wasm32")]
mod web_replay;

use ::rand::Rng;
use ::rand::SeedableRng;
use macroquad::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use std::collections::{BTreeSet, HashSet};
use std::path::{Path, PathBuf};

use rebellion_core::agent_automation::{AutomationModule, PlayerAgent};
use rebellion_core::ai::{AIAction, AIState, AiFaction, FleetMoveReason};
use rebellion_core::betrayal::{BetrayalState, BetrayalSystem};
use rebellion_core::blockade::{BlockadeState, BlockadeSystem};
use rebellion_core::carriage::ProductRoom;
use rebellion_core::combat::{side_present, CombatForce, CombatSide};
use rebellion_core::dat::Faction;
use rebellion_core::delivery::DeliveryState;
use rebellion_core::economy::{EconomyEvent, EconomyState, EconomySystem};
use rebellion_core::events::{EventAction, EventState, EventSystem};
use rebellion_core::fleet_join::FleetMover;
use rebellion_core::fog::{FogState, FogSystem};
use rebellion_core::ids::{CharacterKey, FleetKey, SystemKey, TroopKey};
use rebellion_core::jedi::{JediState, JediSystem};
use rebellion_core::manufacturing::{ManufacturingState, ManufacturingSystem};
use rebellion_core::missions::{
    MissionEffect, MissionFaction, MissionKind, MissionMember, MissionState, MissionSystem,
};
use rebellion_core::movement::{
    apply_fleet_arrival, begin_faction_fleet_transit, fleet_move_confirms,
    fleet_move_enabled, fleet_transit_ticks, fleets_move_enabled, reconcile_fleet_orbits,
    system_side_fleets, validate_fleet_dispatch, validate_fleets_dispatch, MovementState,
    MovementSystem,
};
use rebellion_core::repair::{RepairState, RepairSystem};
use rebellion_core::research::{ResearchState, ResearchSystem};
use rebellion_core::stockpiles::{StockpileEvent, StockpileState, StockpileSystem};
use rebellion_core::tick::{GameClock, GameSpeed};
use rebellion_core::troop_transport::{
    RegimentLeg, RegimentTarget, SquadronSource, TroopTransportState,
};
use rebellion_core::uprising::{UprisingState, UprisingSystem};
use rebellion_core::victory::{VictoryState, VictorySystem};
use rebellion_core::world::{
    CampaignConfig, GameWorld, MstbTable, SeedDifficulty, SeedOptions, VictoryConditions,
};
use rebellion_data::encyclopedia_catalog::EncyclopediaCatalog;
use rebellion_data::encyclopedia_presenter::{
    EncyclopediaContextCaller, EncyclopediaEntryIntent, EncyclopediaReturnRoute,
};
use rebellion_data::encyclopedia_topics::EncyclopediaAudience;

use rebellion_render::build_selection::{
    draw_build_selection, BuildSelectionAction, BuildSelectionState,
};
use rebellion_render::fleet_finder::{draw_fleet_finder, FleetFinderAction, FleetFinderState};
use rebellion_render::fleet_window::{draw_managed_fleet_windows, FleetWindowEntry};
use rebellion_render::game_speed::{
    choose_game_speed, draw_day_readout, draw_game_speed_menu, draw_pause_alert,
    open_game_speed_menu_on_right_click, pause_alert_contains_screen_point, stepped_game_speed,
    GameSpeedUiState,
};
use rebellion_render::message_index::{MessageIndexAction, MessageIndexState};
use rebellion_render::mission_dialog::{
    draw_mission_dialog, MissionDialogAction, MissionDialogState,
};
use rebellion_render::move_confirmation::{
    draw_move_confirmation, draw_scrap_confirmation, draw_stop_confirmation, MoveConfirmation,
    MoveConfirmationAction, MoveConfirmationState, ScrapConfirmation, ScrapConfirmationAction,
    ScrapConfirmationState, StopConfirmation, StopConfirmationAction, StopConfirmationState,
    UnitMover,
};
use rebellion_render::object_menu::{
    draw_object_menu, CommandGates, MenuObject, ObjectMenuCommand, ObjectMenuState, OrderGates,
};
use rebellion_render::operation_report::{
    draw_operation_report, OperationReportAction, OperationReportState,
};
use rebellion_render::personnel_finder::{
    draw_personnel_finder, PersonnelFinderAction, PersonnelFinderState,
};
use rebellion_render::quadrant_icons::Quadrant;
use rebellion_render::status_rows::StatusSources;
use rebellion_render::status_window::{draw_status_window, StatusWindowAction, StatusWindowState};
use rebellion_render::system_window::fleet_label;
use rebellion_render::targeting::{
    capture_pointer, draw_targeting_cursor, release_destination, ReleaseTarget, ReleaseWindows,
    TargetOrder, Targeting, TargetingEnd,
};
use rebellion_render::troop_finder::{draw_troop_finder, TroopFinderAction, TroopFinderState};
use rebellion_render::windowing_input::{
    FrameInputRouter, InputDisposition, InputEventId, InputRecipient, PointerButtonId, PointerPhase,
};
#[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
use rebellion_render::EncyclopediaState;
#[cfg(test)]
use rebellion_render::GalaxyFocus;
use rebellion_render::MdataTrack;
use rebellion_render::{apply_focus_step, galaxy_children, GalaxyChild, GalaxyFocusState};
use rebellion_render::{
    cockpit_accelerator, draw_advisor, draw_cockpit_background,
    draw_cockpit_chrome, draw_cockpit_egui_layer, draw_fleets, draw_galaxy_backdrop,
    draw_galaxy_map, draw_game_options, draw_game_setup, draw_main_menu, draw_missions,
    draw_multiplayer_setup, draw_sector_windows, draw_system_windows,
    draw_tactical_view, handle_cockpit_egui_input, set_cockpit_viewport_clip,
    update_galaxy_map_input, AdvisorFaction, AdvisorState, AssetRenderProfile, AudioVolumeState,
    BmpCache, CockpitButton, CockpitFaction, CockpitState, FleetsState, GalaxyMapState,
    GameMessage, GameOptionsAction, GameOptionsOrigin, GameOptionsState, GameSetupAction,
    GameSetupState, MainMenuAction, MainMenuState, MessageCategory, MessageDisplay, MessageLog,
    MessageLogState, MessageRail, MessageTarget, MultiplayerSetupAction, MultiplayerSetupState,
    OriginalEncyclopediaCatalog, OriginalEncyclopediaEntry, PanelAction, RailAudience,
    SectorWindowAction, SectorWindowState, SystemWindowAction, SystemWindowState, TacticalAction,
    TacticalState, TacticalTrenchRunOutcome, VideoError, VideoPlayer,
};
use rebellion_render::{draw_defenses_windows, DefensesWindowAction, DefensesWindowState};
use rebellion_render::{draw_missions_windows, MissionsWindowAction, MissionsWindowState};
use rebellion_render::{FleetWindowAction, FleetWindowState};
#[cfg(not(target_arch = "wasm32"))]
use rebellion_render::{MusicSelector, SfxKind};
use strategic_windowing::{
    ApplicationInputLocks, LegacyInputBlockers, LegacyInputRestrictionLatch, WindowScene,
};

/// Top-level game mode state machine.
///
/// Controls which screen renders each frame. Transitions are handled
/// in the main loop by matching on actions returned from each screen.
#[derive(Debug, Clone, PartialEq)]
enum GameMode {
    /// Full-screen prerendered cutscene playback.
    Cutscene { kind: CutsceneKind },
    /// Title screen: New Game / Load Game / Quit.
    MainMenu,
    /// Historical head-to-head setup destination.
    MultiplayerSetup,
    /// Campaign configuration: galaxy size, difficulty, faction.
    GameSetup,
    /// The main strategy game: galaxy map + War Room panels.
    Galaxy,
    /// Original unified save, audio, and tactical-display options screen.
    GameOptions { origin: GameOptionsOrigin },
    /// 2D tactical combat view for player-involved battles.
    TacticalCombat,
}

/// The screens music follows (`ghidra/notes/audio-dispatch.md`, "Music"):
/// the menu window's creation picks without a session (`FUN_00405050`), the
/// cockpit's creation and the end of a movie pick in one (`FUN_00422cb0`).
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MusicScene {
    Menu,
    Play,
    Movie,
}

/// The scene a mode belongs to; the options screen keeps the scene it
/// opened over.
#[cfg(not(target_arch = "wasm32"))]
const fn music_scene(mode: &GameMode) -> Option<MusicScene> {
    match mode {
        GameMode::MainMenu | GameMode::MultiplayerSetup | GameMode::GameSetup => {
            Some(MusicScene::Menu)
        }
        GameMode::Galaxy | GameMode::TacticalCombat => Some(MusicScene::Play),
        GameMode::Cutscene { .. } => Some(MusicScene::Movie),
        GameMode::GameOptions { .. } => None,
    }
}

/// Which cutscene is playing — determines post-cutscene transition.
#[derive(Debug, Clone, PartialEq)]
enum CutsceneKind {
    /// A movie the application window plays outside a game (`FUN_004146e0`):
    /// the intro `0x65` and `0x66`, the side's start movie `0x68`/`0x69`, an
    /// ending `0x6a`-`0x6d` (`FUN_00414830`), or the Credits movie `0x6e`
    /// (`FUN_00406000`).
    Application(u8),
    /// Death Star trench-run result (201/202.webm) → resume tactical view.
    TrenchRun(TacticalTrenchRunOutcome),
    /// A kind-6 game message's movie and caption → resume Galaxy
    /// (`FUN_00422ce0`, `0x402`).
    Message(movies::MessageMovie),
}

impl CutsceneKind {
    /// The original's movie id (`ghidra/notes/movies.md`).
    const fn movie(&self) -> u8 {
        match self {
            Self::Application(movie) => *movie,
            Self::TrenchRun(TacticalTrenchRunOutcome::Success) => 0x71,
            Self::TrenchRun(TacticalTrenchRunOutcome::Failure) => 0x70,
            Self::Message(message) => message.movie,
        }
    }

    /// The mode once this movie ends (`movies::after_movie`): the next
    /// state's movie when `open` opens it, the main menu, the command center,
    /// or the window the movie played over (`0x402` to the child window).
    fn ended(&self, side: Faction, open: impl FnMut(u8) -> bool) -> GameMode {
        match movies::after_movie(self.movie(), side, open) {
            movies::Scene::Movie(next) => GameMode::Cutscene {
                kind: Self::Application(next),
            },
            movies::Scene::MainMenu => GameMode::MainMenu,
            movies::Scene::Game => GameMode::Galaxy,
            movies::Scene::Child => match self {
                Self::TrenchRun(_) => GameMode::TacticalCombat,
                Self::Message(_) => GameMode::Galaxy,
                // The Credits movie's callback repaints the menu
                // (FUN_00406520).
                Self::Application(_) => GameMode::MainMenu,
            },
        }
    }
}

#[cfg(test)]
mod movie_end_tests {
    use super::*;

    const SIDES: [Faction; 2] = [Faction::Alliance, Faction::Empire];

    fn cutscene(movie: u8) -> GameMode {
        GameMode::Cutscene {
            kind: CutsceneKind::Application(movie),
        }
    }

    #[test]
    fn the_intro_chains_into_0x66_then_the_main_menu() {
        // FUN_004138a0 0x405: 0x65 enters state 2 (FUN_00414830 plays 0x66);
        // 0x66 enters state 3, the menu; a 0x66 that cannot open ends at once.
        for side in SIDES {
            let intro = CutsceneKind::Application(0x65);
            assert_eq!(intro.ended(side, |_| true), cutscene(0x66));
            assert_eq!(intro.ended(side, |_| false), GameMode::MainMenu);
            assert_eq!(
                CutsceneKind::Application(0x66).ended(side, |_| true),
                GameMode::MainMenu
            );
        }
    }

    #[test]
    fn the_sides_start_movie_ends_in_the_command_center() {
        // FUN_004138a0 0x405: 0x68 and 0x69 enter state 8 (FUN_00414eb0).
        for (side, movie) in [(Faction::Alliance, 0x68), (Faction::Empire, 0x69)] {
            assert_eq!(movies::side_movie(side), movie);
            assert_eq!(
                CutsceneKind::Application(movie).ended(side, |_| true),
                GameMode::Galaxy
            );
        }
    }

    #[test]
    fn the_credits_movie_returns_to_the_main_menu_and_the_others_to_their_window() {
        // FUN_004138a0 0x405 posts 0x402 to the child for 0x6e-0x74: the
        // Credits callback repaints the menu (FUN_00406520), the strategic
        // window resumes (FUN_00422ce0), the tactical view resumes.
        for side in SIDES {
            assert_eq!(
                CutsceneKind::Application(movies::CREDITS).ended(side, |_| true),
                GameMode::MainMenu
            );
            let message = CutsceneKind::Message(movies::MessageMovie {
                movie: movies::CORUSCANT_CAPTURED,
                caption: String::new(),
            });
            assert_eq!(message.ended(side, |_| true), GameMode::Galaxy);
            for outcome in [
                TacticalTrenchRunOutcome::Success,
                TacticalTrenchRunOutcome::Failure,
            ] {
                assert_eq!(
                    CutsceneKind::TrenchRun(outcome).ended(side, |_| true),
                    GameMode::TacticalCombat
                );
            }
            for ending in 0x6a..=0x6d {
                assert_eq!(
                    CutsceneKind::Application(ending).ended(side, |_| true),
                    GameMode::MainMenu
                );
            }
        }
    }
}

/// How many frames a developer command waits for the galaxy view before it
/// is refused (about 10 s at 60 fps). Frames rather than wall time, so a
/// covered window that macOS throttles does not use up the wait between two
/// frames.
#[cfg(not(target_arch = "wasm32"))]
const DEV_GALAXY_WAIT_FRAMES: u32 = 600;

fn window_conf() -> Conf {
    Conf {
        window_title: "Open Rebellion — Star Wars Rebellion".to_string(),
        window_width: 1280,
        window_height: 800,
        window_resizable: true,
        ..Default::default()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn original_game_dir() -> PathBuf {
    std::env::var_os("REBELLION_GAME_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("REBELLION_MDATA_DIR")
                .map(PathBuf::from)
                .and_then(|directory| directory.parent().map(Path::to_path_buf))
        })
        .unwrap_or_else(|| PathBuf::from("../star-wars-rebellion"))
}

#[cfg(not(target_arch = "wasm32"))]
fn common_edata_sibling(gdata_path: &Path) -> Option<PathBuf> {
    gdata_path
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| name.eq_ignore_ascii_case("GData"))
        .and_then(|_| gdata_path.parent())
        .map(|root| root.join("EData"))
}

#[cfg(any(not(target_arch = "wasm32"), feature = "interface-test-fixtures"))]
fn configured_edata_path(gdata_path: &Path) -> PathBuf {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(path) = std::env::var_os("REBELLION_EDATA_DIR").map(PathBuf::from) {
            return path;
        }
        let nested = gdata_path.join("EData");
        if nested.is_dir() {
            return nested;
        }
        if let Some(sibling) = common_edata_sibling(gdata_path) {
            if sibling.is_dir() {
                return sibling;
            }
        }
        original_game_dir().join("EData")
    }

    #[cfg(target_arch = "wasm32")]
    {
        gdata_path.join("EData")
    }
}

#[cfg(test)]
mod edata_path_tests {
    use super::*;

    #[test]
    fn common_original_install_uses_sibling_edata_directory() {
        assert_eq!(
            common_edata_sibling(Path::new("/games/Rebellion/GData")),
            Some(PathBuf::from("/games/Rebellion/EData"))
        );
        assert_eq!(common_edata_sibling(Path::new("data/base")), None);
    }
}

/// Resolve the explicit native asset profile. Browser builds remain on the
/// original-parity profile until manifest-approved HD entries join the runtime
/// pack, so a missing enhancement can never alter browser parity evidence.
fn configured_asset_render_profile() -> AssetRenderProfile {
    #[cfg(not(target_arch = "wasm32"))]
    {
        match std::env::var("OPEN_REBELLION_ASSET_PROFILE") {
            Ok(value) => AssetRenderProfile::parse(&value).unwrap_or_else(|| {
                eprintln!(
                    "[assets] unknown OPEN_REBELLION_ASSET_PROFILE={value:?}; using original-parity"
                );
                AssetRenderProfile::OriginalParity
            }),
            Err(_) => AssetRenderProfile::OriginalParity,
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        AssetRenderProfile::OriginalParity
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn prepare_encyclopedia_hd_snapshot(
    profile: AssetRenderProfile,
    gdata_path: &Path,
    store: &rebellion_data::encyclopedia_session::EncyclopediaSessionStore,
) -> encyclopedia_hd::PreparedEncyclopediaHd {
    let hd_root = gdata_path.parent().unwrap_or(Path::new(".")).join("hd");
    let prepared = store.current().map_or_else(
        encyclopedia_hd::PreparedEncyclopediaHd::original_only,
        |session| {
            encyclopedia_hd::prepare_native_encyclopedia_hd(profile, Some(&hd_root), &session)
        },
    );
    for diagnostic in prepared.diagnostics() {
        macroquad::logging::warn!(
            "[encyclopedia] faithful_hd fallback code={} asset={} detail={}",
            diagnostic.code,
            diagnostic.asset_id.as_deref().unwrap_or("all"),
            diagnostic.detail
        );
    }
    macroquad::logging::info!(
        "[encyclopedia] faithful_hd prepared profile={} selected={} diagnostics={}",
        profile.as_str(),
        prepared.selected_count(),
        prepared.diagnostics().len()
    );
    prepared
}

// The save layer's slots are the Game Options window's rows.
const _: () =
    assert!(rebellion_data::save::MAX_SAVE_SLOTS == rebellion_render::game_options::SAVE_ROWS);

fn read_save_slots(saves_dir: &Path) -> Vec<rebellion_render::SaveSlotInfo> {
    let mut slots: Vec<_> = rebellion_data::save::list_saves(saves_dir)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .map(|meta| rebellion_render::SaveSlotInfo {
            slot: meta.slot,
            name: meta.name,
            player_is_alliance: meta.player_is_alliance,
        })
        .collect();
    for slot in 0..rebellion_data::save::MAX_SAVE_SLOTS {
        if !slots.iter().any(|save| save.slot == slot)
            && rebellion_data::save::slot_occupied(saves_dir, slot)
        {
            slots.push(rebellion_render::SaveSlotInfo {
                slot,
                name: "Unreadable save".into(),
                player_is_alliance: None,
            });
        }
    }
    slots.sort_by_key(|save| save.slot);
    slots
}

struct LiveCampaign<'a> {
    world: &'a mut GameWorld,
    clock: &'a mut GameClock,
    manufacturing: &'a mut ManufacturingState,
    missions: &'a mut MissionState,
    events: &'a mut EventState,
    ai: &'a mut AIState,
    movement: &'a mut MovementState,
    fog_alliance: &'a mut FogState,
    fog_empire: &'a mut FogState,
    player_faction: &'a mut MissionFaction,
    blockade: &'a mut BlockadeState,
    uprising: &'a mut UprisingState,
    research: &'a mut ResearchState,
    jedi: &'a mut JediState,
    victory: &'a mut VictoryState,
    betrayal: &'a mut BetrayalState,
    economy: &'a mut EconomyState,
    sim_rng: &'a mut Xoshiro256PlusPlus,
    ai2: &'a mut Option<AIState>,
    repair: &'a mut RepairState,
    troop_transport: &'a mut TroopTransportState,
    deliveries: &'a mut DeliveryState,
    player_agent: &'a mut PlayerAgent,
    stockpiles: &'a mut StockpileState,
    game_config: &'a mut rebellion_core::tuning::GameConfig,
    campaign_config: &'a mut CampaignConfig,
}

impl LiveCampaign<'_> {
    fn snapshot(&self) -> rebellion_data::save::SaveState {
        rebellion_data::save::SaveState {
            world: self.world.clone(),
            clock: self.clock.clone(),
            manufacturing: self.manufacturing.clone(),
            missions: self.missions.clone(),
            events: self.events.clone(),
            ai: self.ai.clone(),
            movement: self.movement.clone(),
            fog_alliance: self.fog_alliance.clone(),
            fog_empire: self.fog_empire.clone(),
            player_is_alliance: *self.player_faction == MissionFaction::Alliance,
            blockade: self.blockade.clone(),
            uprising: self.uprising.clone(),
            research: self.research.clone(),
            jedi: self.jedi.clone(),
            victory: self.victory.clone(),
            betrayal: self.betrayal.clone(),
            economy: self.economy.clone(),
            sim_rng: self.sim_rng.clone(),
            ai2: self.ai2.clone(),
            repair: self.repair.clone(),
            game_config: self.game_config.clone(),
            campaign_config: *self.campaign_config,
            troop_transport: self.troop_transport.clone(),
            deliveries: self.deliveries.clone(),
            player_agent: self.player_agent.clone(),
            stockpiles: self.stockpiles.clone(),
        }
    }

    fn restore(&mut self, state: rebellion_data::save::SaveState) {
        *self.world = state.world;
        *self.clock = state.clock;
        *self.manufacturing = state.manufacturing;
        *self.missions = state.missions;
        *self.events = state.events;
        *self.ai = state.ai;
        *self.movement = state.movement;
        *self.fog_alliance = state.fog_alliance;
        *self.fog_empire = state.fog_empire;
        *self.player_faction = if state.player_is_alliance {
            MissionFaction::Alliance
        } else {
            MissionFaction::Empire
        };
        *self.blockade = state.blockade;
        *self.uprising = state.uprising;
        *self.research = state.research;
        *self.jedi = state.jedi;
        *self.victory = state.victory;
        *self.betrayal = state.betrayal;
        *self.economy = state.economy;
        *self.sim_rng = state.sim_rng;
        *self.ai2 = state.ai2;
        *self.repair = state.repair;
        *self.game_config = state.game_config;
        *self.campaign_config = state.campaign_config;
        *self.troop_transport = state.troop_transport;
        *self.deliveries = state.deliveries;
        *self.player_agent = state.player_agent;
        *self.stockpiles = state.stockpiles;
        // Bit 11 is derived, not saved: rebuild it before an in-day AI round
        // or a panel action reads it.
        rebellion_core::death_star::refresh_sectors(self.world);
    }
}

/// The advisor of a loaded game, whose saved win `word` the load restores.
/// The original's save holds the advisor's slot table (`FUN_004397a0`,
/// `FUN_00439550`: `+0x164` count, `+0x168` stamps), so a game saved after GO
/// comes back with its ending slot stamped; the port saves the word and
/// stamps the slot again (`FUN_004c3010`, `FUN_004c0f10`). The strategic
/// window is built anew (`FUN_00421c70` zeroes `+0xac`), so the clock runs
/// until the ending.
fn resume_saved_advisor(advisor: &mut AdvisorState, faction: AdvisorFaction, word: u32) {
    advisor.resume_saved_game(faction);
    advisor.post_word(word);
}

/// The status a refused regiment or squadron Move (`0x201`) hands the
/// side's advisor, which shows no text but reacts (`FUN_00487c90`,
/// `mission-dialog.md`, "Refusal"): en route `1`/`1` (`FUN_004f9860`),
/// another side's destination `1`/`0x28` (`FUN_0053d430`), a destroyed
/// planet `1`/`0x22` and a blockaded one `0x90`/4 (`FUN_005073d0`), a fleet
/// in hyperspace `1`/`0x21` (`FUN_00553410`) and no room `1`/`0x27`
/// (`FUN_00552300`; `regiment-unload.md`). port: `None` for the port's own
/// checks of a selection gone stale, which the original's menu never offers.
fn transport_refusal_status(
    error: rebellion_core::troop_transport::TroopTransportError,
) -> Option<rebellion_render::advisor_script::RefusalStatus> {
    use rebellion_core::troop_transport::TroopTransportError as E;
    match error {
        E::RegimentEnRoute | E::SquadronEnRoute => Some((1, 1)),
        E::OtherSideDestination | E::WrongFaction => Some((1, 0x28)),
        E::DestinationDestroyed => Some((1, 0x22)),
        E::DestinationBlockaded => Some((0x90, 4)),
        E::DestinationInTransit => Some((1, 0x21)),
        E::CapacityExceeded { .. } | E::NoSquadronRoom => Some((1, 0x27)),
        E::MissingFleet
        | E::MissingSystem
        | E::NoTroops
        | E::MissingTroop
        | E::TroopNotAtFleetSystem
        | E::AlreadyEmbarked
        | E::FleetNotAtDestination
        | E::MissingSquadron => None,
    }
}

/// The status a refused fleet Move (`0x201`) hands the side's advisor
/// (`FUN_00487c90`; `move-order.md`): an object or destination that does
/// not resolve `1`/`0x12`, `1`/`0x22` (`FUN_00578d00`); another side's or an
/// en-route fleet `1`/`1` (`FUN_004fdc70` → `FUN_004f9860`); no hyperdrive
/// `1`/`0x18` (`FUN_00555920` → `FUN_004fd900`); a blockaded system drag
/// `1`/`1` (`FUN_00552210` → `FUN_005287f0`); a second Death Star `0x90`/3
/// (`FUN_00507ab0`). port: `None` for the port's own origin and same-system
/// checks.
fn fleet_dispatch_refusal_status(
    error: rebellion_core::movement::FleetDispatchError,
) -> Option<rebellion_render::advisor_script::RefusalStatus> {
    use rebellion_core::movement::FleetDispatchError as E;
    match error {
        E::MissingFleet => Some((1, 0x12)),
        E::MissingDestination => Some((1, 0x22)),
        E::WrongFaction | E::AlreadyInTransit | E::Blockaded => Some((1, 1)),
        E::NoHyperdrive => Some((1, 0x18)),
        E::DeathStarRoom => Some((0x90, 3)),
        E::MissingOrigin | E::AlreadyAtDestination => None,
    }
}

/// The status a refused join, Create Fleet or ship Move hands the side's
/// advisor (`FUN_00487c90`; `fleet-join-split.md`): an object that does not
/// resolve `1`/`0x12` (`FUN_00578c00`); another side's or an en-route mover
/// `1`/`1` (`FUN_004f9860`), and an unfinished ship of a Create Fleet `1`/`2`
/// (`FUN_004f9860.c:45-50`); a move into itself `1`/`0x26` (`FUN_00553aa0`);
/// another side's fleet `1`/`0x24`, its first test, and one en route
/// `1`/`0x21` (`FUN_00553410`); a departure's refusal as a fleet Move's.
/// port: `None` for the port's roster check (`fleet_join::roster`).
fn fleet_order_refusal_status(
    error: rebellion_core::fleet_join::FleetOrderError,
) -> Option<rebellion_render::advisor_script::RefusalStatus> {
    use rebellion_core::fleet_join::FleetOrderError as E;
    match error {
        E::MissingFleet | E::MissingShip => Some((1, 0x12)),
        E::WrongFaction | E::InTransit => Some((1, 1)),
        E::Unfinished => Some((1, 2)),
        E::IntoItself => Some((1, 0x26)),
        E::OtherSideDestination => Some((1, 0x24)),
        E::DestinationInTransit => Some((1, 0x21)),
        E::Dispatch(error) => fleet_dispatch_refusal_status(error),
        E::ShipsChanged => None,
    }
}

/// Each side's headquarters, which the seeding marks, or the null key for a
/// side whose data marks none, a mod's. The win word reads a missing
/// Alliance HQ as AD (`FUN_005180e0` on `FUN_00506110`'s null) and finds
/// Coruscant by its id, reading no Empire HQ, so either may be missing.
fn headquarters(world: &GameWorld) -> (SystemKey, SystemKey) {
    let held = |side: Faction| {
        world
            .systems
            .iter()
            .find(|(_, s)| s.is_headquarters && s.control.is_controlled_by(side))
            .map_or_else(SystemKey::default, |(key, _)| key)
    };
    (held(Faction::Alliance), held(Faction::Empire))
}

/// The victory state of the galaxy loaded at startup, before any campaign.
fn startup_victory_state(world: &GameWorld) -> VictoryState {
    let (alliance_hq, empire_hq) = headquarters(world);
    VictoryState::new(alliance_hq, empire_hq)
}

#[cfg(any(target_arch = "wasm32", test))]
const REQUIRED_WASM_DATA: &[&str] = &[
    "SECTORSD.DAT",
    "SYSTEMSD.DAT",
    "CAPSHPSD.DAT",
    "FIGHTSD.DAT",
    "TROOPSD.DAT",
    "MJCHARSD.DAT",
    "MNCHARSD.DAT",
];

#[cfg(any(target_arch = "wasm32", test))]
const OPTIONAL_WASM_DATA: &[&str] = &[
    "GNPRTB.DAT",
    "SDPRTB.DAT",
    "DEFFACSD.DAT",
    "MANFACSD.DAT",
    "PROFACSD.DAT",
    "SYFCCRTB.DAT",
    "SYFCRMTB.DAT",
    "CMUNEFTB.DAT",
    "CMUNAFTB.DAT",
    "CMUNEMTB.DAT",
    "CMUNALTB.DAT",
    "CMUNCRTB.DAT",
    "CMUNHQTB.DAT",
    "CMUNYVTB.DAT",
    "FACLCRTB.DAT",
    "FACLHQTB.DAT",
    "DIPLMSTB.DAT",
    "ESPIMSTB.DAT",
    "ASSNMSTB.DAT",
    "INCTMSTB.DAT",
    "DSSBMSTB.DAT",
    "ABDCMSTB.DAT",
    "RCRTMSTB.DAT",
    "RESCMSTB.DAT",
    "SBTGMSTB.DAT",
    "SUBDMSTB.DAT",
    "ESCAPETB.DAT",
    "FDECOYTB.DAT",
    "FOILTB.DAT",
    "INFORMTB.DAT",
    "CSCRHTTB.DAT",
    "UPRIS1TB.DAT",
    "UPRIS2TB.DAT",
    "RLEVADTB.DAT",
    "RESRCTB.DAT",
    "TDECOYTB.DAT",
    "MISSNSD.DAT",
    "SPECFCSD.DAT",
];

#[cfg(test)]
mod wasm_data_manifest_tests {
    use super::{OPTIONAL_WASM_DATA, REQUIRED_WASM_DATA};

    #[test]
    fn legacy_fallback_contains_every_encyclopedia_catalog_table() {
        for required in [
            "SYSTEMSD.DAT",
            "CAPSHPSD.DAT",
            "FIGHTSD.DAT",
            "TROOPSD.DAT",
            "MJCHARSD.DAT",
            "MNCHARSD.DAT",
            "DEFFACSD.DAT",
            "MANFACSD.DAT",
            "PROFACSD.DAT",
            "MISSNSD.DAT",
            "SPECFCSD.DAT",
        ] {
            assert!(
                REQUIRED_WASM_DATA.contains(&required) || OPTIONAL_WASM_DATA.contains(&required),
                "legacy browser fallback omits Encyclopedia source table {required}",
            );
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn draw_loading_progress(label: &str, loaded: usize, total: usize) {
    clear_background(Color::new(0.02, 0.02, 0.06, 1.0));
    let text = if total == 0 {
        label.to_string()
    } else {
        format!("{label} ({loaded}/{total})")
    };
    let font_size = 24.0;
    let dims = measure_text(&text, None, font_size as u16, 1.0);
    draw_text(
        &text,
        (screen_width() - dims.width) / 2.0,
        screen_height() / 2.0,
        font_size,
        WHITE,
    );
    let bar_w = 300.0;
    let bar_h = 8.0;
    let bar_x = (screen_width() - bar_w) / 2.0;
    let bar_y = screen_height() / 2.0 + 20.0;
    draw_rectangle(bar_x, bar_y, bar_w, bar_h, DARKGRAY);
    let ratio = if total == 0 {
        0.0
    } else {
        loaded as f32 / total as f32
    };
    draw_rectangle(bar_x, bar_y, bar_w * ratio, bar_h, GREEN);
}

#[cfg(target_arch = "wasm32")]
struct WasmRuntimeAssets {
    audio_files: std::collections::HashMap<String, Vec<u8>>,
    encyclopedia: Option<encyclopedia_content::EncyclopediaContentPayload>,
    text_templates: rebellion_data::text_templates::TextTemplates,
}

#[cfg(target_arch = "wasm32")]
fn install_runtime_pack(bytes: &[u8]) -> Result<WasmRuntimeAssets, String> {
    let mut pack = runtime_pack::parse_runtime_pack(bytes).map_err(|error| error.to_string())?;
    for required in REQUIRED_WASM_DATA {
        if !pack.game_files.contains_key(*required) {
            return Err(format!("required entry is missing: {required}"));
        }
    }

    let encyclopedia = encyclopedia_content::EncyclopediaContentPayload::take_from_runtime_pack(
        &mut pack.game_files,
    )
    .map_err(|error| error.to_string())?;
    let encyclopedia_asset_count = encyclopedia.as_ref().map_or(
        0,
        encyclopedia_content::EncyclopediaContentPayload::artwork_count,
    );
    let game_file_count = pack.game_files.len();
    let string_table: std::collections::HashMap<u16, String> = pack
        .game_files
        .remove("textstra.json")
        .and_then(|data| serde_json::from_slice(&data).ok())
        .unwrap_or_default();
    let bitmap_count = pack.bitmaps.len();
    let audio_file_count = pack.audio_files.len();
    let advisor_frame_count = pack.advisor_frames.len();
    let tactical_mesh_count = pack.tactical_meshes.len();
    let tactical_texture_count = pack.tactical_textures.len();
    if bitmap_count == 0 {
        return Err("runtime pack contains no UI bitmaps".to_string());
    }

    let advisor_bitmaps = pack
        .bitmaps
        .iter()
        .filter(|(key, _)| {
            [
                "alsprite-dll/",
                "emsprite-dll/",
                "albrief-dll/",
                "embrief-dll/",
            ]
            .iter()
            .any(|dll| key.starts_with(dll))
        })
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();

    rebellion_data::set_string_table(string_table);
    #[cfg(feature = "interface-test-fixtures")]
    rebellion_render::set_encyclopedia_asset_cache(
        encyclopedia
            .as_ref()
            .map_or_else(std::collections::HashMap::new, |content| {
                content.legacy_renderer_artwork()
            }),
    );
    #[cfg(not(feature = "interface-test-fixtures"))]
    rebellion_render::set_encyclopedia_asset_cache(std::collections::HashMap::new());
    // The templates leave the pack before its game files become the cache.
    let text_templates = pack
        .game_files
        .remove("textstra-rcdata.json")
        .map(|data| rebellion_data::text_templates::TextTemplates::from_json(&data))
        .transpose()
        .map_err(|error| error.to_string())?
        .unwrap_or_default();
    rebellion_data::set_file_cache(pack.game_files);
    rebellion_render::set_advisor_asset_cache(pack.advisor_frames, advisor_bitmaps);
    rebellion_render::set_bmp_cache(pack.bitmaps);
    rebellion_render::set_tactical_asset_cache(pack.tactical_meshes, pack.tactical_textures);
    macroquad::logging::info!(
        "runtime_asset_pack loaded game_files={} encyclopedia_assets={} ui_bitmaps={} advisor_frames={} audio_files={} tactical_meshes={} tactical_textures={} bytes={}",
        game_file_count,
        encyclopedia_asset_count,
        bitmap_count,
        advisor_frame_count,
        audio_file_count,
        tactical_mesh_count,
        tactical_texture_count,
        bytes.len()
    );
    Ok(WasmRuntimeAssets {
        audio_files: pack.audio_files,
        encyclopedia,
        text_templates,
    })
}

#[cfg(target_arch = "wasm32")]
async fn load_loose_wasm_encyclopedia(
) -> Result<Option<encyclopedia_content::EncyclopediaContentPayload>, String> {
    let pointer_bytes = match macroquad::file::load_file("data/encyclopedia/current.json").await {
        Ok(bytes) => bytes,
        Err(_) => return Ok(None),
    };
    let Some(generation) = encyclopedia_content::parse_loose_pointer(&pointer_bytes)
        .map_err(|error| error.to_string())?
    else {
        return Ok(None);
    };
    let base = format!("data/encyclopedia/generations/{generation}");
    let catalog_path = format!("{base}/catalog.json");
    let manifest_path = format!("{base}/manifest.json");
    let catalog = macroquad::file::load_file(&catalog_path)
        .await
        .map_err(|error| format!("{catalog_path} failed to load: {error:?}"))?;
    let manifest = macroquad::file::load_file(&manifest_path)
        .await
        .map_err(|error| format!("{manifest_path} failed to load: {error:?}"))?;
    let (source, _) = rebellion_data::encyclopedia_topics::parse_encyclopedia_source_with_manifest(
        &catalog, &manifest,
    )
    .map_err(|error| error.to_string())?;
    let mut entries = std::collections::HashMap::from([
        ("encyclopedia/catalog.json".to_string(), catalog),
        ("encyclopedia/manifest.json".to_string(), manifest),
    ]);
    let filenames: std::collections::BTreeSet<String> = source.artwork.values().cloned().collect();
    let mut retained_artwork_bytes = 0;
    for filename in filenames {
        let path = format!("{base}/assets/{filename}");
        let bytes = macroquad::file::load_file(&path)
            .await
            .map_err(|error| format!("{path} failed to load: {error:?}"))?;
        retained_artwork_bytes = encyclopedia_content::checked_artwork_transfer_total(
            retained_artwork_bytes,
            bytes.len(),
        )
        .map_err(|error| error.to_string())?;
        entries.insert(format!("encyclopedia/assets/{filename}"), bytes);
    }
    encyclopedia_content::EncyclopediaContentPayload::from_loose_entries(entries)
        .map(Some)
        .map_err(|error| error.to_string())
}

#[cfg(target_arch = "wasm32")]
async fn load_legacy_wasm_assets(
) -> Result<Option<encyclopedia_content::EncyclopediaContentPayload>, String> {
    use std::collections::HashMap;

    let total = REQUIRED_WASM_DATA.len() + OPTIONAL_WASM_DATA.len();
    let mut files: HashMap<String, Vec<u8>> = HashMap::new();
    let mut loaded = 0;

    for &name in REQUIRED_WASM_DATA {
        let path = format!("data/base/{name}");
        match macroquad::file::load_file(&path).await {
            Ok(data) => {
                files.insert(name.to_string(), data);
                loaded += 1;
            }
            Err(error) => panic!("Required file {name} failed to load: {error:?}"),
        }
        draw_loading_progress("Loading game data…", loaded, total);
        next_frame().await;
    }

    for &name in OPTIONAL_WASM_DATA {
        let path = format!("data/base/{name}");
        if let Ok(data) = macroquad::file::load_file(&path).await {
            files.insert(name.to_string(), data);
        }
        loaded += 1;
        draw_loading_progress("Loading game data…", loaded, total);
        next_frame().await;
    }

    let string_table: HashMap<u16, String> =
        match macroquad::file::load_file("data/base/textstra.json").await {
            Ok(data) => serde_json::from_slice(&data).unwrap_or_default(),
            Err(_) => HashMap::new(),
        };
    rebellion_data::set_string_table(string_table);
    rebellion_data::set_file_cache(files);

    #[derive(serde::Deserialize)]
    struct BmpEntry {
        dll: String,
        id: u32,
    }

    let entries: Vec<BmpEntry> = match macroquad::file::load_file("data/ui/bmp-manifest.json").await
    {
        Ok(manifest_bytes) => match serde_json::from_slice(&manifest_bytes) {
            Ok(entries) => entries,
            Err(error) => {
                eprintln!(
                    "ERROR: bmp-manifest.json is malformed: {error} — UI textures will be missing"
                );
                Vec::new()
            }
        },
        Err(error) => {
            eprintln!(
                "WARNING: bmp-manifest.json failed to load ({error:?}) — UI textures will be missing"
            );
            Vec::new()
        }
    };
    let bmp_total = entries.len();
    let mut bmp_cache = HashMap::with_capacity(bmp_total);
    let mut fetch_failures = 0;

    for (index, entry) in entries.iter().enumerate() {
        let path = format!("data/ui/{}/BMP/{}.bmp", entry.dll, entry.id);
        match macroquad::file::load_file(&path).await {
            Ok(data) => {
                bmp_cache.insert(format!("{}/{}", entry.dll, entry.id), data);
            }
            Err(error) => {
                fetch_failures += 1;
                if fetch_failures <= 5 {
                    eprintln!("WARNING: failed to fetch {path}: {error:?}");
                }
            }
        }
        let bmp_loaded = index + 1;
        if bmp_total > 0 && (bmp_loaded % 50 == 0 || bmp_loaded == bmp_total) {
            draw_loading_progress("Loading UI assets…", bmp_loaded, bmp_total);
            next_frame().await;
        }
    }

    if fetch_failures > 0 {
        eprintln!(
            "WARNING: {fetch_failures}/{bmp_total} BMP fetches failed — some UI textures will be missing"
        );
    }
    eprintln!(
        "Loaded {} of {} UI BMPs through legacy per-file fallback",
        bmp_cache.len(),
        bmp_total
    );
    rebellion_render::set_bmp_cache(bmp_cache);
    let encyclopedia = load_loose_wasm_encyclopedia().await?;
    #[cfg(feature = "interface-test-fixtures")]
    rebellion_render::set_encyclopedia_asset_cache(
        encyclopedia
            .as_ref()
            .map_or_else(std::collections::HashMap::new, |content| {
                content.legacy_renderer_artwork()
            }),
    );
    #[cfg(not(feature = "interface-test-fixtures"))]
    rebellion_render::set_encyclopedia_asset_cache(std::collections::HashMap::new());
    Ok(encyclopedia)
}

#[cfg(target_arch = "wasm32")]
async fn load_wasm_assets() -> WasmRuntimeAssets {
    draw_loading_progress("Loading optimized runtime assets…", 0, 0);
    next_frame().await;

    match macroquad::file::load_file("data/runtime.orpk").await {
        Ok(bytes) => install_runtime_pack(&bytes)
            .unwrap_or_else(|error| panic!("Invalid data/runtime.orpk: {error}")),
        Err(error) => {
            eprintln!(
                "WARNING: data/runtime.orpk unavailable ({error:?}); using legacy per-file loading"
            );
            let encyclopedia = load_legacy_wasm_assets()
                .await
                .unwrap_or_else(|error| panic!("Invalid loose Encyclopedia publication: {error}"));
            WasmRuntimeAssets {
                audio_files: std::collections::HashMap::new(),
                encyclopedia,
                text_templates: rebellion_data::text_templates::TextTemplates::default(),
            }
        }
    }
}

/// Cache every glyph the Macroquad layers can draw in the current Galaxy view
/// before the first draw call for a new font size.
///
/// Macroquad 0.4.x may resize its shared font atlas while a render batch still
/// references the old texture. Pre-measuring the complete character set makes
/// any resize happen at the safe start of the frame instead. Egui uses its own
/// atlas and is unaffected.
fn prewarm_galaxy_font_sizes(
    world: &GameWorld,
    map_state: &GalaxyMapState,
    warmed_sizes: &mut HashSet<u16>,
) {
    // The map never zooms (rebellion_render::galaxy_camera), so its label
    // sizes are those of scale 1.
    let mut sizes = vec![14, 18];
    if map_state.show_sector_labels {
        sizes.push(16);
    }

    if sizes.iter().all(|size| warmed_sizes.contains(size)) {
        return;
    }

    let mut characters = std::collections::BTreeSet::new();
    for text in [
        "REBEL ALLIANCE — COMMAND CENTER",
        "GALACTIC EMPIRE — COMMAND BRIDGE",
        "0123456789d",
    ] {
        characters.extend(text.chars());
    }
    for (_, sector) in &world.sectors {
        characters.extend(sector.name.chars());
    }
    for (_, system) in &world.systems {
        characters.extend(system.name.chars());
    }
    let sample: String = characters.into_iter().collect();

    for size in sizes {
        if warmed_sizes.insert(size) {
            measure_text(&sample, None, size, 1.0);
        }
    }
}

fn toggle_exclusive_panel(panels: &mut [&mut bool], selected: usize) {
    let was_open = *panels[selected];
    if !was_open {
        for panel in panels.iter_mut() {
            **panel = false;
        }
    }
    *panels[selected] = !was_open;
}

/// Whether the single-letter keys open the egui side panels (Officers,
/// Fleets, Missions, Research, Jedi, Bombardment, Death Star, Loyalty).
/// port: those panels are not original interface and hide the cockpit's
/// left side, so only debug builds open them. Release builds, including
/// native acceptance runs with `OPEN_REBELLION_DEV`, never do. Alt chords
/// belong to the original accelerators.
fn panel_letters_open_panels(debug_build: bool, alt_down: bool) -> bool {
    debug_build && !alt_down
}

/// One normalized set of strategic root keys for the immutable frame lease.
/// Children render first; root claims are attempted only afterwards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StrategicKeyEvent {
    key: KeyCode,
    id: InputEventId,
    alt: bool,
    repeat: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StrategicRawKeyEvent {
    key: KeyCode,
    alt: bool,
    repeat: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct StrategicIndexedKeyEvent {
    ordinal: u32,
    event: StrategicRawKeyEvent,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct StrategicRawPointerEvent {
    ordinal: u32,
    point: (f32, f32),
    phase: PointerPhase,
}

#[derive(Debug, Default)]
struct StrategicRawInputBatch {
    keys: Vec<StrategicIndexedKeyEvent>,
    pointers: Vec<StrategicRawPointerEvent>,
    /// Every key-down in order, autorepeats included, for the tactical
    /// window ([`TacticalState::set_frame_key_downs`]).
    key_downs: Vec<KeyCode>,
}

#[derive(Debug, Default)]
struct StrategicRawInputCollector {
    next_ordinal: u32,
    keys: Vec<StrategicIndexedKeyEvent>,
    pointers: Vec<StrategicRawPointerEvent>,
    key_downs: Vec<KeyCode>,
    pressed: BTreeSet<PointerButtonId>,
    pointer: (f32, f32),
}

impl StrategicRawInputCollector {
    fn take(&mut self) -> StrategicRawInputBatch {
        self.next_ordinal = 0;
        StrategicRawInputBatch {
            keys: std::mem::take(&mut self.keys),
            pointers: std::mem::take(&mut self.pointers),
            key_downs: std::mem::take(&mut self.key_downs),
        }
    }

    fn next_ordinal(&mut self) -> Option<u32> {
        let ordinal = self.next_ordinal;
        self.next_ordinal = self.next_ordinal.checked_add(1)?;
        Some(ordinal)
    }

    fn push_pointer(&mut self, point: (f32, f32), phase: PointerPhase) {
        if let Some(ordinal) = self.next_ordinal() {
            self.pointers.push(StrategicRawPointerEvent {
                ordinal,
                point,
                phase,
            });
        }
    }
}

const fn normalized_pointer_button(button: macroquad::miniquad::MouseButton) -> PointerButtonId {
    match button {
        macroquad::miniquad::MouseButton::Left => PointerButtonId::Primary,
        macroquad::miniquad::MouseButton::Right => PointerButtonId::Secondary,
        macroquad::miniquad::MouseButton::Middle => PointerButtonId::Middle,
        macroquad::miniquad::MouseButton::Unknown => PointerButtonId::Extra(u8::MAX),
    }
}

impl macroquad::miniquad::EventHandler for StrategicRawInputCollector {
    fn update(&mut self) {}

    fn draw(&mut self) {}

    fn key_down_event(
        &mut self,
        key: KeyCode,
        modifiers: macroquad::miniquad::KeyMods,
        repeat: bool,
    ) {
        self.key_downs.push(key);
        if strategic_root_key(key) {
            let event = StrategicRawKeyEvent {
                key,
                alt: modifiers.alt,
                repeat,
            };
            if let Some(ordinal) = self.next_ordinal() {
                self.keys.push(StrategicIndexedKeyEvent { ordinal, event });
            }
        }
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.pointer = (x, y);
        let phase = self
            .pressed
            .iter()
            .next()
            .copied()
            .map_or(PointerPhase::Hover, PointerPhase::Motion);
        self.push_pointer(self.pointer, phase);
    }

    fn mouse_wheel_event(&mut self, _x: f32, _y: f32) {
        self.push_pointer(self.pointer, PointerPhase::Auxiliary);
    }

    fn mouse_button_down_event(
        &mut self,
        button: macroquad::miniquad::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.pointer = (x, y);
        let button = normalized_pointer_button(button);
        self.pressed.insert(button);
        self.push_pointer(self.pointer, PointerPhase::Press(button));
    }

    fn mouse_button_up_event(&mut self, button: macroquad::miniquad::MouseButton, x: f32, y: f32) {
        self.pointer = (x, y);
        let button = normalized_pointer_button(button);
        self.pressed.remove(&button);
        self.push_pointer(self.pointer, PointerPhase::Release(button));
    }
}

/// Key-only backend capture; production collects keys and pointers together
/// in [`StrategicRawInputCollector`].
#[cfg(test)]
#[derive(Debug, Default)]
struct StrategicRawKeyCollector {
    events: Vec<StrategicRawKeyEvent>,
}

#[cfg(test)]
impl StrategicRawKeyCollector {
    fn take(&mut self) -> Vec<StrategicRawKeyEvent> {
        std::mem::take(&mut self.events)
    }
}

#[cfg(test)]
impl macroquad::miniquad::EventHandler for StrategicRawKeyCollector {
    fn update(&mut self) {}

    fn draw(&mut self) {}

    fn key_down_event(
        &mut self,
        key: KeyCode,
        modifiers: macroquad::miniquad::KeyMods,
        repeat: bool,
    ) {
        if strategic_root_key(key) {
            self.events.push(StrategicRawKeyEvent {
                key,
                alt: modifiers.alt,
                repeat,
            });
        }
    }
}

#[derive(Debug, Default)]
struct StrategicRootKeyBatch {
    events: Vec<StrategicKeyEvent>,
}

impl StrategicRootKeyBatch {
    #[cfg(test)]
    fn collect_from_raw(
        frame: u64,
        router: &mut FrameInputRouter,
        events: &[StrategicRawKeyEvent],
    ) -> Self {
        let events = events
            .iter()
            .enumerate()
            .filter_map(|(ordinal, event)| {
                let id = InputEventId {
                    frame,
                    ordinal: u32::try_from(ordinal).ok()?,
                };
                router.collect_keyboard(id);
                Some(StrategicKeyEvent {
                    key: event.key,
                    id,
                    alt: event.alt,
                    repeat: event.repeat,
                })
            })
            .collect();
        Self { events }
    }

    fn collect_from_indexed_raw(
        frame: u64,
        router: &mut FrameInputRouter,
        events: &[StrategicIndexedKeyEvent],
    ) -> Self {
        let events = events
            .iter()
            .map(|indexed| {
                let id = InputEventId {
                    frame,
                    ordinal: indexed.ordinal,
                };
                router.collect_keyboard(id);
                StrategicKeyEvent {
                    key: indexed.event.key,
                    id,
                    alt: indexed.event.alt,
                    repeat: indexed.event.repeat,
                }
            })
            .collect();
        Self { events }
    }

    /// Reports each key's disposition for the child that owned it at the
    /// frame boundary. `fleet` names the focused Fleet window and whether
    /// its in-place rename was open; each key it receives is decided on its
    /// own by [`rename_edit_owns_key`].
    fn report_legacy_children(
        &self,
        router: &mut FrameInputRouter,
        handles_keyboard: bool,
        fleet: Option<(rebellion_windowing::WindowId, bool)>,
    ) {
        for event in &self.events {
            let recipient = router.keyboard_recipient(event.id);
            let disposition = match recipient {
                InputRecipient::Root => Some(if handles_keyboard {
                    InputDisposition::Handled
                } else {
                    InputDisposition::Unhandled
                }),
                InputRecipient::Window(window) => fleet
                    .filter(|(fleet_window, _)| *fleet_window == window)
                    .map(|(_, renaming)| {
                        // Escape closes the window when no rename holds it
                        // (slot +0x30, FUN_004a7390).
                        if (renaming && rename_edit_owns_key(*event))
                            || (!renaming && event.key == KeyCode::Escape)
                        {
                            InputDisposition::Handled
                        } else {
                            InputDisposition::Unhandled
                        }
                    }),
                InputRecipient::Blocked => None,
            };
            if let Some(disposition) = disposition {
                router.report_key(event.id, recipient, disposition);
            }
        }
    }

    /// Initial presses in backend order. port: root shortcuts act on a key's
    /// first press only, as the `is_key_pressed` dispatch before the frame
    /// router did, so a held key never re-toggles a panel, the palette or a
    /// window; whether the original's accelerators auto-repeat is untraced.
    fn presses(&self) -> impl Iterator<Item = StrategicKeyEvent> + '_ {
        self.events.iter().copied().filter(|event| !event.repeat)
    }

    fn contains(&self, key: KeyCode) -> bool {
        self.presses().any(|event| event.key == key)
    }
}

/// Whether an open in-place rename consumes `event`. The application loop
/// runs `TranslateAcceleratorA` before dispatch (`FUN_00413560`), so the
/// accelerators, F1 to F7 and the Alt chords, never reach the edit and
/// bubble to the strategic view; every other strategic key is the edit's
/// text, Enter or Escape (`FUN_004ac950`, `FUN_004aca40`;
/// `ghidra/notes/strategic-windowing-toolkit.md`, "Keyboard ownership").
const fn rename_edit_owns_key(event: StrategicKeyEvent) -> bool {
    !event.alt
        && !matches!(
            event.key,
            KeyCode::F1
                | KeyCode::F2
                | KeyCode::F3
                | KeyCode::F4
                | KeyCode::F5
                | KeyCode::F6
                | KeyCode::F7
        )
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct StrategicPointerEvent {
    id: InputEventId,
    point: (f32, f32),
    phase: PointerPhase,
}

#[derive(Debug, Default)]
struct StrategicPointerBatch {
    events: Vec<StrategicPointerEvent>,
}

impl StrategicPointerBatch {
    fn collect_from_raw(
        frame: u64,
        router: &mut FrameInputRouter,
        events: &[StrategicRawPointerEvent],
        hit_windows: impl Fn((f32, f32)) -> BTreeSet<rebellion_windowing::WindowId>,
    ) -> Self {
        let events = events
            .iter()
            .map(|event| {
                let id = InputEventId {
                    frame,
                    ordinal: event.ordinal,
                };
                router.collect_pointer(id, &hit_windows(event.point), event.phase);
                StrategicPointerEvent {
                    id,
                    point: event.point,
                    phase: event.phase,
                }
            })
            .collect();
        Self { events }
    }

    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    fn latest_primary_release(&self) -> Option<InputEventId> {
        self.events.iter().rev().find_map(|event| {
            matches!(event.phase, PointerPhase::Release(PointerButtonId::Primary))
                .then_some(event.id)
        })
    }

    fn router_events(&self) -> Vec<(InputEventId, PointerPhase)> {
        self.events
            .iter()
            .map(|event| (event.id, event.phase))
            .collect()
    }
}

/// Whether the galaxy view acts on a root key: its accelerators (TEXTCOMM 11,
/// `FUN_00422ce0` case `0x483`) and F1-F7 (`WM_KEYDOWN`), and the port's
/// panel letters, S, Tab (the Mod Manager) and palette key. Escape is not
/// one: the view's `WM_KEYDOWN` has no `VK_ESCAPE` case, passes other keys to
/// `FUN_005ff530`, which has none, and the table has no Escape entry
/// (`ghidra/notes/keyboard-shortcuts.md`, section 1), so on the bare galaxy
/// it does nothing; the windows that close on it read it first.
fn galaxy_root_acts(key: KeyCode, alt: bool, debug: bool, palette: bool) -> bool {
    let panel_letter = matches!(key, KeyCode::F | KeyCode::N);
    cockpit_accelerator(key).is_some()
        || key == KeyCode::F6
        || alt
            && matches!(
                key,
                KeyCode::P
                    | KeyCode::KpAdd
                    | KeyCode::KpSubtract
                    | KeyCode::G
                    | KeyCode::U
                    | KeyCode::A
                    | KeyCode::Key1
                    | KeyCode::Key2
                    | KeyCode::Key3
                    | KeyCode::Key4
                    | KeyCode::Key5
                    | KeyCode::Key6
                    | KeyCode::Key7
                    | KeyCode::Key8
                    | KeyCode::Key9
            )
        || panel_letter && panel_letters_open_panels(debug, alt)
        || key == KeyCode::S && !alt
        || key == KeyCode::Tab
        || palette
}

#[cfg(test)]
mod galaxy_root_key_tests {
    use super::*;

    #[test]
    fn escape_on_the_bare_galaxy_does_nothing_while_its_shortcuts_still_act() {
        // FUN_00422ce0's WM_KEYDOWN handles Tab and F1-F7 but no VK_ESCAPE,
        // and TEXTCOMM 11 has no Escape accelerator (move-order.md; VM,
        // alliance-day927-orto-orbit.sav).
        for (alt, debug) in [(false, false), (true, true), (false, true)] {
            assert!(!galaxy_root_acts(KeyCode::Escape, alt, debug, false));
        }
        assert!(galaxy_root_acts(KeyCode::F1, false, false, false));
        assert!(galaxy_root_acts(KeyCode::F6, false, false, false));
        // Nor is R: no WM_KEYDOWN case and no TEXTCOMM 11 entry has it
        // (keyboard-shortcuts.md, sections 1 and 2).
        assert!(!galaxy_root_acts(KeyCode::R, false, false, false));
        assert!(!strategic_root_key(KeyCode::R));
        assert!(galaxy_root_acts(KeyCode::Tab, false, false, false));
        assert!(galaxy_root_acts(KeyCode::P, true, false, false));
        assert!(!galaxy_root_acts(KeyCode::P, false, false, false));
        assert!(galaxy_root_acts(KeyCode::S, false, false, false));
        assert!(!galaxy_root_acts(KeyCode::S, true, false, false));
        assert!(galaxy_root_acts(KeyCode::N, false, true, false));
        assert!(!galaxy_root_acts(KeyCode::N, false, false, false));
        assert!(!galaxy_root_acts(KeyCode::Q, false, true, false));
        assert!(galaxy_root_acts(KeyCode::GraveAccent, false, true, true));
    }
}

const fn strategic_root_key(key: KeyCode) -> bool {
    matches!(
        key,
        KeyCode::Escape
            | KeyCode::F1
            | KeyCode::F2
            | KeyCode::F3
            | KeyCode::F4
            | KeyCode::F5
            | KeyCode::F6
            | KeyCode::F7
            | KeyCode::P
            | KeyCode::G
            | KeyCode::U
            | KeyCode::A
            | KeyCode::O
            | KeyCode::F
            | KeyCode::N
            | KeyCode::T
            | KeyCode::J
            | KeyCode::L
            | KeyCode::S
            | KeyCode::Tab
            | KeyCode::GraveAccent
            | KeyCode::KpAdd
            | KeyCode::KpSubtract
            | KeyCode::Key1
            | KeyCode::Key2
            | KeyCode::Key3
            | KeyCode::Key4
            | KeyCode::Key5
            | KeyCode::Key6
            | KeyCode::Key7
            | KeyCode::Key8
            | KeyCode::Key9
    )
}

#[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
fn emit_windowing_route_receipt(
    router: &FrameInputRouter,
    event: InputEventId,
    kind: &str,
    application: &str,
    underlay_action_count: usize,
) {
    let recipient = match if kind == "key" {
        router.keyboard_recipient(event)
    } else {
        router.pointer_recipient(event)
    } {
        InputRecipient::Root => "root".to_owned(),
        InputRecipient::Window(window) => format!("window:{}", window.get()),
        InputRecipient::Blocked => "blocked".to_owned(),
    };
    macroquad::logging::info!(
        "[windowing-route] physical={}:{} kind={} router={} application={} underlay={}",
        event.frame,
        event.ordinal,
        kind,
        recipient,
        application,
        underlay_action_count,
    );
}

/// Temporarily remove input from the retained strategic egui pass while a
/// manager-owned modal has the lease. Layout and paint still run so the legacy
/// underlay remains visually stable. The saved state is restored before the
/// migrated-content seam and root disposition checks.
fn suspend_legacy_egui_input(
    ctx: &egui_macroquad::egui::Context,
) -> egui_macroquad::egui::InputState {
    let saved = ctx.input(Clone::clone);
    ctx.input_mut(|input| {
        input.raw.events.clear();
        input.events.clear();
        input.pointer = egui_macroquad::egui::PointerState::default();
        input.raw_scroll_delta = egui_macroquad::egui::Vec2::ZERO;
        input.smooth_scroll_delta = egui_macroquad::egui::Vec2::ZERO;
        input.modifiers = egui_macroquad::egui::Modifiers::NONE;
        input.keys_down.clear();
        input.focused = false;
    });
    saved
}

fn restore_egui_input(
    ctx: &egui_macroquad::egui::Context,
    saved: egui_macroquad::egui::InputState,
) {
    ctx.input_mut(|input| *input = saved);
}

/// Focus losses the browser shell has counted (`web/index.html`).
#[cfg(target_arch = "wasm32")]
fn browser_focus_losses() -> u32 {
    extern "C" {
        fn open_rebellion_focus_losses() -> u32;
    }
    // SAFETY: the shell's `open_rebellion_focus` plugin defines this
    // argumentless import before the module starts.
    unsafe { open_rebellion_focus_losses() }
}

/// Whether AppKit reports the application active
/// (`-[NSApplication isActive]`).
#[cfg(target_os = "macos")]
fn native_app_active() -> bool {
    use macroquad::miniquad::native::apple::frameworks::{
        class, msg_send, sel, sel_impl, ObjcId, BOOL, NO,
    };
    // SAFETY: called on the main thread inside the game loop, which AppKit
    // runs; `sharedApplication` exists for the whole process.
    unsafe {
        let app: ObjcId = msg_send![class!(NSApplication), sharedApplication];
        let active: BOOL = msg_send![app, isActive];
        active != NO
    }
}

/// Run one retained legacy window family's pass with its input muted unless
/// the frame boundary permits it. Layout and paint are unchanged.
fn with_retained_input<T>(
    ctx: &egui_macroquad::egui::Context,
    allowed: bool,
    draw: impl FnOnce() -> T,
) -> T {
    let saved = (!allowed).then(|| suspend_legacy_egui_input(ctx));
    let drawn = draw();
    if let Some(saved) = saved {
        restore_egui_input(ctx, saved);
    }
    drawn
}

#[cfg(test)]
mod frame_input_tests {
    use super::with_retained_input;
    use egui_macroquad::egui;

    /// Clicks a retained window's area and reports whether the window saw
    /// the click and whether the input reached owners drawn after it.
    fn click_retained_window(allowed: bool) -> (bool, bool) {
        let ctx = egui::Context::default();
        let point = egui::pos2(20.0, 20.0);
        let button = |pressed| egui::Event::PointerButton {
            pos: point,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        let mut clicked = false;
        let mut released_after = false;
        for events in [
            vec![egui::Event::PointerMoved(point)],
            vec![egui::Event::PointerMoved(point)],
            vec![button(true)],
            vec![button(false)],
            vec![],
        ] {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(200.0, 200.0),
                )),
                events,
                ..egui::RawInput::default()
            };
            let _ = ctx.run(input, |ctx| {
                with_retained_input(ctx, allowed, || {
                    egui::Area::new(egui::Id::new("retained"))
                        .fixed_pos(egui::Pos2::ZERO)
                        .show(ctx, |ui| {
                            let response = ui
                                .allocate_response(egui::vec2(100.0, 100.0), egui::Sense::click());
                            clicked |= response.clicked();
                        });
                });
                released_after |= ctx.input(|input| input.pointer.primary_released());
            });
        }
        (clicked, released_after)
    }

    #[test]
    fn a_held_retained_window_ignores_clicks_and_later_owners_still_receive_them() {
        assert_eq!(click_retained_window(true), (true, true));
        assert_eq!(click_retained_window(false), (false, true));
    }
}

#[cfg(test)]
mod panel_toggle_tests {
    use super::{
        panel_letters_open_panels, toggle_exclusive_panel, StrategicPointerBatch,
        StrategicRawInputCollector, StrategicRawKeyCollector, StrategicRawKeyEvent,
        StrategicRootKeyBatch,
    };
    use macroquad::miniquad::{EventHandler, KeyMods, MouseButton};
    use macroquad::prelude::KeyCode;
    use rebellion_render::windowing_input::FrameInputRouter;

    #[test]
    fn production_root_shortcut_claims_one_normalized_event_once() {
        let coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let frame = coordinator.begin_frame();
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &frame);
        let events = [StrategicRawKeyEvent {
            key: KeyCode::Escape,
            alt: false,
            repeat: false,
        }];
        let batch = StrategicRootKeyBatch::collect_from_raw(1, &mut router, &events);
        batch.report_legacy_children(&mut router, false, None);

        assert!(router.claim_root(batch.events[0].id));
        assert!(!router.claim_root(batch.events[0].id));
    }

    #[test]
    fn production_root_shortcuts_preserve_repeat_order_and_event_modifiers() {
        let coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let frame = coordinator.begin_frame();
        let mut router = FrameInputRouter::default();
        router.begin_frame(9, &frame);
        let events = [
            StrategicRawKeyEvent {
                key: KeyCode::P,
                alt: true,
                repeat: false,
            },
            StrategicRawKeyEvent {
                key: KeyCode::P,
                alt: false,
                repeat: true,
            },
        ];
        let batch = StrategicRootKeyBatch::collect_from_raw(9, &mut router, &events);
        batch.report_legacy_children(&mut router, false, None);

        assert_eq!(batch.events.len(), 2);
        assert!(batch.events[0].alt);
        assert!(!batch.events[1].alt);
        assert!(!batch.events[0].repeat);
        assert!(batch.events[1].repeat);
        assert!(router.claim_root(batch.events[0].id));
        assert!(router.claim_root(batch.events[1].id));
    }

    #[test]
    fn production_root_shortcut_stops_after_child_handled_disposition() {
        let coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let frame = coordinator.begin_frame();
        let mut router = FrameInputRouter::default();
        router.begin_frame(2, &frame);
        let events = [StrategicRawKeyEvent {
            key: KeyCode::Escape,
            alt: false,
            repeat: false,
        }];
        let batch = StrategicRootKeyBatch::collect_from_raw(2, &mut router, &events);
        batch.report_legacy_children(&mut router, true, None);

        assert!(!router.claim_root(batch.events[0].id));
    }

    fn opened_fleet(
        coordinator: &mut crate::strategic_windowing::StrategicWindowing,
    ) -> rebellion_windowing::WindowId {
        use rebellion_render::windowing_policy::contextual_spec;
        use rebellion_windowing::{
            SubjectId, WindowCommand, WindowKind, WindowOwner, WindowTransition,
        };

        coordinator.enqueue(WindowCommand::Open(contextual_spec(
            WindowKind::Fleet,
            SubjectId::new(2, 100),
            WindowOwner::StrategicRoot,
        )));
        let applied = coordinator.apply_pending();
        match applied[0].result.as_ref().unwrap().transition {
            WindowTransition::Opened { id } => id,
            ref transition => panic!("expected Fleet open, got {transition:?}"),
        }
    }

    fn key(key: KeyCode, alt: bool) -> StrategicRawKeyEvent {
        StrategicRawKeyEvent {
            key,
            alt,
            repeat: false,
        }
    }

    #[test]
    fn a_focused_fleet_without_a_rename_bubbles_every_key_to_root() {
        use rebellion_render::windowing_input::InputRecipient;

        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let fleet = opened_fleet(&mut coordinator);
        let frame = coordinator.begin_frame();
        let events = [key(KeyCode::P, true), key(KeyCode::R, false)];
        let mut router = FrameInputRouter::default();
        router.begin_frame(3, &frame);
        let batch = StrategicRootKeyBatch::collect_from_raw(3, &mut router, &events);
        assert_eq!(
            router.keyboard_recipient(batch.events[0].id),
            InputRecipient::Window(fleet)
        );

        batch.report_legacy_children(&mut router, false, Some((fleet, false)));

        assert!(router.claim_root(batch.events[0].id));
        assert!(router.claim_root(batch.events[1].id));
    }

    #[test]
    fn a_focused_fleet_keeps_its_escape_which_closes_it() {
        // fleet-window.md: Escape runs the window's slot +0x30 close
        // (FUN_004a7390 -> FUN_004ac3a0), so it never reaches the view.
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let fleet = opened_fleet(&mut coordinator);
        let frame = coordinator.begin_frame();
        let events = [key(KeyCode::Escape, false), key(KeyCode::R, false)];
        let mut router = FrameInputRouter::default();
        router.begin_frame(6, &frame);
        let batch = StrategicRootKeyBatch::collect_from_raw(6, &mut router, &events);

        batch.report_legacy_children(&mut router, false, Some((fleet, false)));

        assert!(!router.claim_root(batch.events[0].id));
        assert!(router.claim_root(batch.events[1].id));
    }

    #[test]
    fn an_open_fleet_rename_owns_its_text_keys_while_accelerators_bubble_to_root() {
        // FUN_00413560 translates accelerators before dispatch; the edit
        // owns its text, Enter and Escape (FUN_004ac950, FUN_004aca40).
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let fleet = opened_fleet(&mut coordinator);
        let frame = coordinator.begin_frame();
        let events = [
            key(KeyCode::R, false),
            key(KeyCode::F2, false),
            key(KeyCode::Escape, false),
            key(KeyCode::P, true),
            key(KeyCode::Key2, false),
            key(KeyCode::Key2, true),
        ];
        let mut router = FrameInputRouter::default();
        router.begin_frame(4, &frame);
        let batch = StrategicRootKeyBatch::collect_from_raw(4, &mut router, &events);

        batch.report_legacy_children(&mut router, false, Some((fleet, true)));

        let bubbled: Vec<bool> = batch
            .events
            .iter()
            .map(|event| router.claim_root(event.id))
            .collect();
        assert_eq!(bubbled, [false, true, false, true, false, true]);
    }

    #[test]
    fn held_keys_dispatch_root_shortcuts_on_their_first_press_only() {
        let coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let frame = coordinator.begin_frame();
        let mut router = FrameInputRouter::default();
        router.begin_frame(5, &frame);
        let repeat = |key| StrategicRawKeyEvent {
            key,
            alt: false,
            repeat: true,
        };
        let events = [
            key(KeyCode::Tab, false),
            repeat(KeyCode::Tab),
            repeat(KeyCode::Tab),
            repeat(KeyCode::Escape),
            key(KeyCode::GraveAccent, false),
        ];
        let batch = StrategicRootKeyBatch::collect_from_raw(5, &mut router, &events);

        let pressed: Vec<KeyCode> = batch.presses().map(|event| event.key).collect();
        assert_eq!(pressed, [KeyCode::Tab, KeyCode::GraveAccent]);
        assert!(batch.contains(KeyCode::Tab));
        assert!(!batch.contains(KeyCode::Escape));
    }

    #[test]
    fn raw_backend_capture_keeps_function_and_keypad_accelerators_in_order() {
        let mut collector = StrategicRawKeyCollector::default();
        let function_keys = [
            KeyCode::F1,
            KeyCode::F2,
            KeyCode::F3,
            KeyCode::F4,
            KeyCode::F5,
            KeyCode::F6,
            KeyCode::F7,
        ];
        for key in function_keys {
            collector.key_down_event(key, KeyMods::default(), false);
            collector.key_up_event(key, KeyMods::default());
        }
        collector.key_down_event(
            KeyCode::KpAdd,
            KeyMods {
                alt: true,
                ..KeyMods::default()
            },
            true,
        );
        collector.key_down_event(KeyCode::KpSubtract, KeyMods::default(), false);

        let captured = collector.take();
        let keys: Vec<_> = captured.iter().map(|event| event.key).collect();
        assert_eq!(
            keys,
            [
                KeyCode::F1,
                KeyCode::F2,
                KeyCode::F3,
                KeyCode::F4,
                KeyCode::F5,
                KeyCode::F6,
                KeyCode::F7,
                KeyCode::KpAdd,
                KeyCode::KpSubtract,
            ]
        );
        assert_eq!(
            captured[7],
            StrategicRawKeyEvent {
                key: KeyCode::KpAdd,
                alt: true,
                repeat: true,
            }
        );
        assert!(!captured[8].repeat);
    }

    #[test]
    fn raw_backend_pointer_and_key_events_share_physical_order_and_router_recipients() {
        let coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let frame = coordinator.begin_frame();
        let mut router = FrameInputRouter::default();
        router.begin_frame(12, &frame);
        let mut collector = StrategicRawInputCollector::default();
        collector.mouse_motion_event(30.0, 40.0);
        collector.mouse_button_down_event(MouseButton::Left, 30.0, 40.0);
        collector.key_down_event(
            KeyCode::Key2,
            KeyMods {
                alt: true,
                ..KeyMods::default()
            },
            false,
        );
        collector.mouse_button_up_event(MouseButton::Left, 35.0, 45.0);
        let raw = collector.take();
        let pointers =
            StrategicPointerBatch::collect_from_raw(12, &mut router, &raw.pointers, |_| {
                Default::default()
            });
        let keys = StrategicRootKeyBatch::collect_from_indexed_raw(12, &mut router, &raw.keys);

        assert_eq!(
            pointers
                .events
                .iter()
                .map(|event| event.id.ordinal)
                .collect::<Vec<_>>(),
            [0, 1, 3]
        );
        assert_eq!(keys.events[0].id.ordinal, 2);
        assert!(pointers.events.iter().all(|event| {
            router.pointer_recipient(event.id)
                == rebellion_render::windowing_input::InputRecipient::Root
        }));
        assert_eq!(
            router.keyboard_recipient(keys.events[0].id),
            rebellion_render::windowing_input::InputRecipient::Root,
        );
    }

    // Source: the tactical window acts on every WM_KEYDOWN of a held number
    // key; only Tab tests the autorepeat bit (FUN_0059b730.c:8-11).
    #[test]
    fn raw_backend_capture_keeps_every_key_down_for_the_tactical_window() {
        let mut collector = StrategicRawInputCollector::default();
        collector.key_down_event(KeyCode::Key1, KeyMods::default(), false);
        collector.key_down_event(KeyCode::Key1, KeyMods::default(), true);
        collector.key_down_event(KeyCode::F9, KeyMods::default(), false);
        assert_eq!(
            collector.take().key_downs,
            [KeyCode::Key1, KeyCode::Key1, KeyCode::F9]
        );
        assert!(collector.take().key_downs.is_empty());
    }

    #[test]
    fn panel_letters_open_panels_only_in_debug_builds_without_alt() {
        assert!(panel_letters_open_panels(true, false));
        assert!(!panel_letters_open_panels(false, false));
        assert!(!panel_letters_open_panels(true, true));
        assert!(!panel_letters_open_panels(false, true));
    }

    #[test]
    fn opens_closes_and_switches_exclusive_panels() {
        let mut panels = [false; 9];
        let press = |panels: &mut [bool; 9], selected| {
            let mut refs: Vec<&mut bool> = panels.iter_mut().collect();
            toggle_exclusive_panel(&mut refs, selected);
        };

        press(&mut panels, 0);
        assert_eq!(
            panels,
            [true, false, false, false, false, false, false, false, false]
        );
        press(&mut panels, 0);
        assert_eq!(panels, [false; 9]);
        press(&mut panels, 2);
        press(&mut panels, 8);
        assert_eq!(
            panels,
            [false, false, false, false, false, false, false, false, true]
        );
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    #![expect(
        clippy::too_many_lines,
        reason = "Keep the existing main loop together; extracting phases is a separate refactor."
    )]
    // Accept an optional GData path as the first CLI argument.
    // On WASM there is no CLI, so always use the hardcoded default.
    #[cfg(not(target_arch = "wasm32"))]
    let gdata_path = {
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 1 {
            PathBuf::from(&args[1])
        } else {
            let candidate = PathBuf::from("data/base");
            if candidate.join("SYSTEMSD.DAT").exists() {
                candidate
            } else {
                eprintln!("Usage: open-rebellion <path-to-GData>");
                eprintln!("  GData directory must contain .DAT files (SYSTEMSD.DAT, etc.)");
                std::process::exit(1);
            }
        }
    };
    #[cfg(target_arch = "wasm32")]
    let gdata_path = PathBuf::from("data/base");

    let asset_render_profile = configured_asset_render_profile();
    macroquad::logging::info!("[assets] render_profile={}", asset_render_profile.as_str());

    #[cfg(not(target_arch = "wasm32"))]
    let encyclopedia_content = match encyclopedia_content::read_native_encyclopedia(
        &gdata_path,
        &configured_edata_path(&gdata_path),
    ) {
        Ok(content) => content,
        Err(error) => {
            macroquad::logging::error!(
                "[encyclopedia] native_content rejected before installation error={}",
                error
            );
            None
        }
    };

    #[cfg(target_arch = "wasm32")]
    if web_replay::requested() {
        web_replay::run(&gdata_path).await;
    }

    // ── Load game data ─────────────────────────────────────────────────────
    // Native: filesystem read via load_game_data()
    // WASM: HTTP fetch via macroquad::file::load_file() into cache, then load_game_data()
    #[cfg(not(target_arch = "wasm32"))]
    let mut world = match rebellion_data::load_game_data(&gdata_path) {
        Ok(w) => w,
        Err(e) => {
            eprintln!(
                "Failed to load game data from {}: {}",
                gdata_path.display(),
                e
            );
            std::process::exit(1);
        }
    };

    #[cfg(target_arch = "wasm32")]
    let (mut world, mut browser_audio_files, encyclopedia_content, text_templates) = {
        let assets = load_wasm_assets().await;
        let world = rebellion_data::load_game_data(&gdata_path)
            .unwrap_or_else(|error| panic!("Failed to parse game data: {error}"));
        (
            world,
            assets.audio_files,
            assets.encyclopedia,
            assets.text_templates,
        )
    };
    // TEXTSTRA's message templates and advice topics (RCDATA).
    #[cfg(not(target_arch = "wasm32"))]
    let text_templates = rebellion_data::text_templates::TextTemplates::load(&gdata_path)
        .unwrap_or_else(|error| {
            eprintln!("WARNING: {error:#}; messages show without their original text");
            rebellion_data::text_templates::TextTemplates::default()
        });

    eprintln!(
        "Loaded: {} systems, {} sectors, {} ship classes, {} fighter classes, {} characters",
        world.systems.len(),
        world.sectors.len(),
        world.capital_ship_classes.len(),
        world.fighter_classes.len(),
        world.characters.len(),
    );

    // mods/ lives alongside data/, not inside it. One resolved runtime order
    // feeds both world and presentation-content consumers.
    let mods_dir = gdata_path
        .parent()
        .and_then(|p| p.parent())
        .unwrap_or(std::path::Path::new("."))
        .join("mods");
    let mut mod_runtime = rebellion_data::mods::ModRuntime::discover(&mods_dir);
    for error in &mod_runtime.errors {
        eprintln!("[mod-runtime] {error}");
    }
    let resolved_mod_order = mod_runtime.enabled_sorted();

    // The Galactic Encyclopedia is immutable reference data, not campaign
    // state. Rebuild it from the installed DAT/TEXTSTRA source so opening the
    // index after a save/load never changes the serialized world format.
    let encyclopedia_catalog = match rebellion_data::encyclopedia_catalog::load_encyclopedia_catalog(
        &gdata_path,
    ) {
        Ok(source) => {
            let category_counts: [usize; 7] = std::array::from_fn(|index| {
                source
                    .entries_for(source.categories[index].command_id)
                    .len()
            });
            macroquad::logging::info!(
                    "[encyclopedia] source_catalog loaded entries={} categories=all:{},systems:{},ships:{},facilities:{},missions:{},troops:{},personnel:{}",
                    source.entries.len(),
                    category_counts[0],
                    category_counts[1],
                    category_counts[2],
                    category_counts[3],
                    category_counts[4],
                    category_counts[5],
                    category_counts[6],
                );
            Some(source)
        }
        Err(error) => {
            macroquad::logging::error!("[encyclopedia] source_catalog unavailable error={}", error);
            None
        }
    };
    let original_encyclopedia_catalog = encyclopedia_catalog.as_ref().map(|source| {
        let entries = source
            .entries
            .iter()
            .map(|entry| OriginalEncyclopediaEntry {
                object_id: entry.object_id,
                name: entry.name.clone(),
            })
            .collect();
        OriginalEncyclopediaCatalog::new(
            source.title.clone(),
            source.topic_label.clone(),
            std::array::from_fn(|index| source.categories[index].label.clone()),
            entries,
        )
    });
    let mut encyclopedia_session_store =
        rebellion_data::encyclopedia_session::EncyclopediaSessionStore::default();
    let mut encyclopedia_base_disposition = None;
    #[cfg(not(target_arch = "wasm32"))]
    let mut encyclopedia_base_input = None;
    if let (Some(content), Some(catalog)) = (encyclopedia_content, encyclopedia_catalog) {
        match rebellion_data::encyclopedia_topics::load_encyclopedia_system_pictures(&gdata_path) {
            Ok(system_pictures) => {
                let input = content.into_session_input(catalog, system_pictures);
                match encyclopedia_session_store.replace(input.clone()) {
                    Ok(disposition) => {
                        encyclopedia_base_disposition = Some(disposition);
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let report = encyclopedia_mods::install_native_encyclopedia_mods(
                                &input,
                                &mut encyclopedia_session_store,
                                &mod_runtime,
                                &resolved_mod_order,
                            );
                            encyclopedia_mods::log_native_encyclopedia_mod_report(&report);
                            encyclopedia_base_input = Some(input.clone());
                        }
                    }
                    Err(error) => {
                        macroquad::logging::error!(
                            "[encyclopedia] content_session rejected before publication error={}",
                            error
                        );
                    }
                }
            }
            Err(error) => {
                macroquad::logging::error!(
                    "[encyclopedia] system_picture_bindings unavailable error={}",
                    error
                );
            }
        }
        if let (Some(session), Some(disposition)) = (
            encyclopedia_session_store.current(),
            encyclopedia_base_disposition,
        ) {
            macroquad::logging::info!(
                "[encyclopedia] content_session installed disposition={:?} fingerprint={} alliance_topics={} empire_topics={}",
                disposition,
                session.logical_fingerprint(),
                session
                    .topics(rebellion_data::encyclopedia_topics::EncyclopediaAudience::Alliance)
                    .entries
                    .len(),
                session
                    .topics(rebellion_data::encyclopedia_topics::EncyclopediaAudience::Empire)
                    .entries
                    .len(),
            );
        }
    } else {
        macroquad::logging::warn!(
            "[encyclopedia] content_session unavailable; production route remains disabled"
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    let mut encyclopedia_hd = prepare_encyclopedia_hd_snapshot(
        asset_render_profile,
        &gdata_path,
        &encyclopedia_session_store,
    );
    #[cfg(target_arch = "wasm32")]
    let encyclopedia_hd = encyclopedia_hd::PreparedEncyclopediaHd::original_only();
    #[cfg(not(target_arch = "wasm32"))]
    let mut encyclopedia_hd_generation = encyclopedia_session_store.texture_generation();
    #[cfg(not(all(target_arch = "wasm32", feature = "interface-test-fixtures")))]
    let _ = original_encyclopedia_catalog.as_ref();

    // ── Mod Runtime ──────────────────────────────────────────────────────────
    if !mod_runtime.discovered.is_empty() {
        eprintln!(
            "Discovered {} mods ({} enabled)",
            mod_runtime.discovered.len(),
            mod_runtime.discovered.iter().filter(|m| m.enabled).count()
        );
        let mod_errors = mod_runtime.apply_ordered(&mut world, &resolved_mod_order);
        for err in &mod_errors {
            eprintln!("Mod error: {err:?}");
        }
    }
    drop(resolved_mod_order);

    // ── Game mode ─────────────────────────────────────────────────────────
    // State 1 at the start of the program (FUN_00414830): the intro, whose
    // end plays 0x66 and then shows the menu (CutsceneKind::ended). One
    // that cannot open ends at once.
    let mut game_mode = GameMode::Cutscene {
        kind: CutsceneKind::Application(movies::INTRO),
    };
    let mut main_menu_state = MainMenuState::default();
    let mut game_setup_state = GameSetupState::default();
    let mut multiplayer_setup_state = MultiplayerSetupState::default();
    let mut pending_cockpit_start: Option<GameSetupAction> = None;
    let mut pending_victory_conditions = VictoryConditions::Standard;
    let mut campaign_generation = 0_u32;

    // ── Simulation state ────────────────────────────────────────────────────
    // Seedable RNG for deterministic simulation
    let rng_seed = {
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(42, |d| d.as_secs())
        }
        #[cfg(target_arch = "wasm32")]
        {
            (macroquad::time::get_time() * 1_000_000.0) as u64
        }
    };
    let mut sim_rng = Xoshiro256PlusPlus::seed_from_u64(rng_seed);
    let mut clock = GameClock::new();
    let mut mfg_state = ManufacturingState::new();
    let mut mission_state = MissionState::new();
    let mut event_state = EventState::new();
    let mut ai_state = AIState::new(AiFaction::Empire);
    let mut game_config = rebellion_core::tuning::GameConfig::default();
    let mut campaign_config = CampaignConfig::default();
    let mut dual_ai_mode = false;
    let mut secondary_ai_state: Option<AIState> = None;
    let mut movement_state = MovementState::new();
    let mut fog_alliance_state = FogState::new(Faction::Alliance);
    let mut fog_empire_state = FogState::new(Faction::Empire);
    FogSystem::seed(&mut fog_alliance_state, &world);
    FogSystem::seed(&mut fog_empire_state, &world);
    let mut blockade_state = BlockadeState::new();
    let mut uprising_state = UprisingState::new();
    let mut research_state = ResearchState::new();
    let mut jedi_state = JediState::new();
    let mut betrayal_state = BetrayalState::new();
    let mut repair_state = RepairState::default();
    let mut troop_transport_state = TroopTransportState::default();
    let mut delivery_state = DeliveryState::new();
    let mut stockpile_state = StockpileState::new();
    let mut economy_state = EconomyState::default();
    // The player's agent: Manage Garrisons and Manage Production, both off
    // at a new game (FUN_00439950).
    let mut player_agent = PlayerAgent::default();
    let mut agent_menu_state = rebellion_render::agent_menu::AgentMenuState::default();
    // The local player's message movies and the ending (movies.rs), and
    // whether the galaxy had no modal window last frame (FUN_00601340).
    let mut movie_queue = movies::MovieQueue::default();
    let mut movie_window_free = false;
    // Find HQ systems for victory detection
    let mut victory_state = startup_victory_state(&world);

    // Register scripted story events
    rebellion_core::story_events::define_story_events(&mut event_state, &world);

    // ── UI state ────────────────────────────────────────────────────────────
    let mut map_state = GalaxyMapState::default();
    let mut warmed_galaxy_font_sizes = HashSet::new();
    let mut msg_log = MessageLog::default();
    let mut log_state = MessageLogState::default();

    // ── War Room panel state ────────────────────────────────────────────────
    let mut player_faction = MissionFaction::Alliance;
    let mut fleets_state = FleetsState::default();
    let mut build_selection_state = BuildSelectionState::default();
    let mut mission_dialog_state = MissionDialogState::default();
    let mut move_confirmation_state = MoveConfirmationState::default();
    let mut scrap_confirmation_state = ScrapConfirmationState::default();
    let mut stop_confirmation_state = StopConfirmationState::default();
    let mut status_window_state = StatusWindowState::default();
    let mut fleet_finder_state = FleetFinderState::default();
    let mut encyclopedia_surface = encyclopedia_surface::EncyclopediaSurfaceController::new();
    let mut troop_finder_state = TroopFinderState::default();
    let mut personnel_finder_state = PersonnelFinderState::default();
    // The Operation Report window (type 0x1c, FUN_0044c410).
    let mut operation_report_state = OperationReportState::default();
    // The open Message Index (window 0x0d) and the speed its Advice
    // category saved (FUN_00487ff0 +0x58).
    let mut message_index: Option<MessageIndexState> = None;
    let mut advice_saved_speed: Option<GameSpeed> = None;
    // The Encyclopedia holds the speed through the same slot (FUN_0045ddc0,
    // FUN_00460070); this is whether the last frame saw it open.
    let mut encyclopedia_speed_held = false;
    // The sound of the message the Message Index last showed, played by the
    // next frame's audio step.
    let mut pending_message_sound: Option<MessageSounds> = None;
    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    let mut enc_state = EncyclopediaState::new();
    let mut mod_manager_state = rebellion_render::ModManagerState::default();
    #[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
    let mut command_palette_state = rebellion_render::CommandPaletteState::new();
    #[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
    let palette_enabled = dev_commands::palette_enabled();
    // Palette and script commands waiting for the frame that can run them.
    #[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
    let mut pending_interface: Vec<rebellion_render::InterfaceCommand> = Vec::new();
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_channel = dev_channel::DevChannel::from_env();
    #[cfg(not(target_arch = "wasm32"))]
    if dev_commands::palette_enabled() {
        rebellion_data::telemetry::enable_native_from_env();
    }
    // A live inbox keeps the queue open after the script's last line.
    #[cfg(not(target_arch = "wasm32"))]
    let mut command_script = dev_commands::CommandScript::from_env().or_else(|| {
        dev_channel
            .is_live()
            .then(dev_commands::CommandScript::default)
    });
    // A `Wait` holds the queue until this time or for this many frames; a
    // `Capture` saves the frame being drawn.
    #[cfg(not(target_arch = "wasm32"))]
    let (mut dev_hold_until, mut dev_hold_frames) = (0.0_f64, 0_u32);
    // A `Wait until` holds the queue while its condition is false, with the
    // frames it has waited.
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_wait_until: Option<(dev_commands::WaitCondition, u32)> = None;
    // The frames the pending wait may run, when its line gave `within <n> frames`.
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_wait_budget: Option<u32> = None;
    // What `Wait until message` and `Wait until mission resolved` count from.
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_wait_baseline: u64 = 0;
    // Ids of the player's side's missions as they resolved, oldest first.
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_missions_resolved: Vec<u64> = Vec::new();
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_policy = dev_policy::DevPolicy::default();
    // Until when, and for how many frames, a `Wait <n> ms` or `Wait <n> frames`
    // holds the queue: the policies act then, not in a click's hold frames.
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_policy_hold = (0.0_f64, 0_u32);
    // The mission count before a `Mission:` dispatches, to see it land.
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_mission_check: Option<usize> = None;
    // Frames the next line has waited for the galaxy view.
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_galaxy_wait_frames: u32 = 0;
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_capture: Option<String> = None;
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_input = dev_input::DevInput::default();
    #[cfg(not(target_arch = "wasm32"))]
    rebellion_render::control_registry::set_enabled(dev_commands::palette_enabled());
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_queue_drained = false;
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_commands_ready: Vec<dev_commands::DevCommand> = Vec::new();
    // The file a `Save:` or `Load:` names, for the SaveGame or LoadGame it
    // queues in place of a slot.
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_save_file: Option<PathBuf> = None;
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_load_file: Option<PathBuf> = None;
    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    {
        enc_state.set_edata_path(configured_edata_path(&gdata_path));
        enc_state.set_asset_profile(asset_render_profile);
        // HD upscaled PNGs live as a sibling of the base data directory.
        let hd_path = gdata_path
            .parent()
            .unwrap_or(Path::new("."))
            .join("hd")
            .join("EData");
        enc_state.set_hd_path(hd_path);
    }
    // Panel visibility (mutually exclusive left panels)
    let mut show_fleets = false;
    let mut show_missions = false;
    let saves_dir = rebellion_data::save::default_saves_dir();
    let mut save_slots = read_save_slots(&saves_dir);

    // ── Tactical combat state ────────────────────────────────────────────────
    let mut tactical_state = TacticalState::new();

    // ── Cockpit chrome ───────────────────────────────────────────────────────
    let mut cockpit_state = CockpitState::new(CockpitFaction::Alliance);
    let mut game_speed_ui = GameSpeedUiState::default();
    let mut cockpit_tooltips = rebellion_render::tooltip::TooltipState::default();
    let mut sector_window_state = SectorWindowState::default();
    let mut system_window_state = SystemWindowState::default();
    let mut fleet_window_state = FleetWindowState::default();
    let mut defenses_window_state = DefensesWindowState::default();
    let mut missions_window_state = MissionsWindowState::default();
    // The object pop-up menu a right-click in a system window opens.
    let mut object_menu: Option<ObjectMenuState> = None;
    // A Mission order waiting for its target (the galaxy view's mode 2).
    let mut targeting: Option<Targeting> = None;
    // Whether the system cursor is hidden behind the targeting cursor.
    let mut system_cursor_hidden = false;
    let mut bmp_cache = BmpCache::new();
    {
        // gdata_path is data/base; staged UI BMPs live at data/base/ui/
        let ui_path = gdata_path.join("ui");
        bmp_cache.set_base_path(&ui_path);
        #[cfg(not(target_arch = "wasm32"))]
        {
            let tactical_runtime = ui_path
                .join("tactical-dll")
                .join("TACTICAL3D")
                .join("runtime");
            if tactical_runtime.is_dir() {
                match rebellion_render::install_native_tactical_assets(&tactical_runtime) {
                    Ok(()) => {
                        macroquad::logging::info!(
                            "[tactical_3d] installed complete native tactical corpus"
                        );
                    }
                    Err(error) => {
                        macroquad::logging::warn!(
                            "[tactical_3d] native tactical assets rejected: {}",
                            error
                        );
                    }
                }
            }
        }
        // HD PNG overrides at data/hd/{dll-name}/{resource_id}.png.
        let hd_ui_path = gdata_path
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join("hd");
        bmp_cache.set_hd_path(hd_ui_path);
        bmp_cache.set_render_profile(asset_render_profile);
    }

    // ── Droid advisor ──────────────────────────────────────────────────────
    let mut advisor_state = AdvisorState::new(AdvisorFaction::Alliance);
    // The agent's advice topics (FUN_0048b460), rebuilt with each campaign.
    let mut advice_topics = rebellion_render::advice_topics::AdviceTopics::default();
    // The side whose droid sounds the audio engine holds.
    let mut advisor_voice_side: Option<AdvisorFaction> = None;
    // A cockpit command the briefing's last step issues: the Message Index
    // on Advice (FUN_0041d770(1, 0x82)).
    let mut pending_index_command: Option<u16> = None;
    {
        let sprite_dir = gdata_path.join("ui");
        advisor_state.set_sprite_dir(&sprite_dir);
    }

    // ── Audio state ─────────────────────────────────────────────────────────
    let mut audio_vol = AudioVolumeState::default();
    // OPEN_REBELLION_MUTE silences music, effects and cutscenes from launch,
    // for native acceptance runs (scripts/launch-native.sh).
    #[cfg(not(target_arch = "wasm32"))]
    if std::env::var("OPEN_REBELLION_MUTE").is_ok_and(|value| env_flag_on(&value)) {
        audio_vol.muted = true;
        audio_vol.music_muted = true;
        audio_vol.dirty = true;
    }
    let mut game_options_state = GameOptionsState::default();
    let mut quit_requested = false;
    #[cfg(not(target_arch = "wasm32"))]
    let sounds_dir = PathBuf::from("data/sounds");

    #[cfg(not(target_arch = "wasm32"))]
    let mut audio_engine = {
        let mut engine = audio::AudioEngine::new();
        if sounds_dir.exists() {
            engine.load_all(&sounds_dir);
        }
        let common_dll = original_game_dir().join("COMMON.DLL");
        if common_dll.exists() {
            engine.load_original_menu_sfx(&common_dll);
        }
        let tactical_dll = original_game_dir().join("TACTICAL.DLL");
        if tactical_dll.exists() {
            engine.load_original_tactical_sfx(&tactical_dll);
        }
        engine.load_original_tactical_voice(
            &original_game_dir().join("VOICEFXA.DLL"),
            &original_game_dir().join("VOICEFXE.DLL"),
        );
        if load_message_sounds(&mut engine, &gdata_path.join("ui")) == 0 {
            engine.load_original_message_sounds(&original_game_dir().join("STRATEGY.DLL"));
        }
        audio_vol.backend_available = engine.is_available();
        engine
    };

    #[cfg(target_arch = "wasm32")]
    let browser_main_theme = browser_audio_files.remove("music/main_theme.wav");
    #[cfg(target_arch = "wasm32")]
    let browser_menu_sfx: Vec<_> = audio::MENU_SFX_ASSETS
        .iter()
        .filter_map(|&(kind, path, _)| browser_audio_files.remove(path).map(|bytes| (kind, bytes)))
        .collect();
    #[cfg(target_arch = "wasm32")]
    let browser_tactical_sfx: Vec<_> = audio::TACTICAL_SFX_ASSETS
        .iter()
        .filter_map(|&(_, _, path, resource_id)| {
            browser_audio_files
                .remove(path)
                .map(|bytes| (resource_id, bytes))
        })
        .collect();
    #[cfg(target_arch = "wasm32")]
    let browser_tactical_voice: Vec<_> = audio::tactical_voice_assets()
        .into_iter()
        .filter_map(|(faction, path, resource_id)| {
            browser_audio_files
                .remove(&path)
                .map(|bytes| (faction, resource_id, bytes))
        })
        .collect();
    #[cfg(target_arch = "wasm32")]
    let mut browser_menu_audio = if browser_main_theme.is_some()
        || !browser_menu_sfx.is_empty()
        || !browser_tactical_sfx.is_empty()
        || !browser_tactical_voice.is_empty()
    {
        // Initialise WebAudio and begin decoding before the first interaction.
        // Its resume handlers are then ready for the first user gesture.
        let mut engine = audio::AudioEngine::new();
        if let Some(bytes) = browser_main_theme.as_deref() {
            engine.load_music_bytes(MdataTrack::MENU, bytes);
        }
        for (kind, bytes) in &browser_menu_sfx {
            engine.load_sfx_bytes(*kind, bytes);
        }
        for (resource_id, bytes) in &browser_tactical_sfx {
            engine.load_tactical_sfx_bytes(*resource_id, bytes);
        }
        for (faction, resource_id, bytes) in &browser_tactical_voice {
            engine.load_tactical_voice_bytes(*faction, *resource_id, bytes);
        }
        load_message_sounds(&mut engine, &gdata_path.join("ui"));
        Some(engine)
    } else {
        None
    };
    #[cfg(target_arch = "wasm32")]
    let mut browser_menu_audio_requested = false;
    #[cfg(target_arch = "wasm32")]
    {
        audio_vol.backend_available = browser_menu_audio.is_some();
    }
    #[cfg(not(target_arch = "wasm32"))]
    let mdata_dir = original_game_dir().join("MDATA");
    #[cfg(not(target_arch = "wasm32"))]
    let mut music_selector = MusicSelector::default();
    #[cfg(not(target_arch = "wasm32"))]
    let mut music_scene_now: Option<MusicScene> = None;
    #[cfg(not(target_arch = "wasm32"))]
    let mut music_alert_open = false;
    #[cfg(not(target_arch = "wasm32"))]
    let mut music_result: Option<MdataTrack> = None;

    let mut cutscene_player = open_movie(
        movies::INTRO,
        &audio_vol,
        #[cfg(not(target_arch = "wasm32"))]
        &mut audio_engine,
    );
    // A message movie that could not open still shows its caption until
    // dismissed: the caption is that message's only notice (kind 6 files
    // no Message Index row). port: the original's movies always open.
    let mut caption_without_movie = false;

    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    let interface_fixture_request = interface_test_fixture::requested();
    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    if let Some(request) = interface_fixture_request {
        cutscene_player = None;
        audio_vol.muted = true;
        audio_vol.music_muted = true;
        audio_vol.dirty = true;
        interface_test_fixture::apply(
            request,
            &mut world,
            &mut game_mode,
            &mut player_faction,
            &mut cockpit_state,
            &mut map_state,
            &mut movement_state,
            &mut mfg_state,
            &mut economy_state,
            &mut mission_state,
            &mut blockade_state,
            &mut sector_window_state,
            &mut system_window_state,
            &mut fleet_window_state,
            &mut defenses_window_state,
            &mut troop_transport_state,
        );
        // The AI plays the other side, as a new game sets it.
        ai_state = AIState::new(if player_faction == MissionFaction::Empire {
            AiFaction::Alliance
        } else {
            AiFaction::Empire
        });
        interface_test_fixture::open_mission_dialog(
            request,
            &world,
            &uprising_state,
            &mut mission_dialog_state,
        );
    }
    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    let mut encyclopedia_surface_fixture = interface_fixture_request
        .filter(|request| request.scenario.is_encyclopedia_surface())
        .map(|request| {
            let audience = match request.faction {
                CockpitFaction::Alliance => {
                    rebellion_data::encyclopedia_topics::EncyclopediaAudience::Alliance
                }
                CockpitFaction::Empire => {
                    rebellion_data::encyclopedia_topics::EncyclopediaAudience::Empire
                }
            };
            if let Some(start) = request.scenario.encyclopedia_fixture_start() {
                encyclopedia_surface::EncyclopediaSurfaceFixture::new(audience, start)
                    .unwrap_or_else(|error| panic!("W4 Encyclopedia fixture is invalid: {error}"))
            } else {
                let session = encyclopedia_session_store.current().unwrap_or_else(|| {
                    panic!(
                        "E30 canonical Encyclopedia fixture requires an installed content session"
                    )
                });
                encyclopedia_surface::EncyclopediaSurfaceFixture::from_session(
                    session,
                    audience,
                    request
                        .scenario
                        .canonical_encyclopedia_fixture_start()
                        .expect("canonical Encyclopedia scenario has a fixture start"),
                )
                .unwrap_or_else(|error| panic!("E30 Encyclopedia fixture is invalid: {error}"))
            }
        });
    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    let tactical_fixture_request = tactical_test_fixture::requested();
    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    let tactical_fixture_failed = if let Some(request) = tactical_fixture_request {
        cutscene_player = None;
        audio_vol.muted = true;
        audio_vol.music_muted = true;
        audio_vol.dirty = true;
        match tactical_test_fixture::apply(
            request,
            &mut world,
            &mut tactical_state,
            &mut msg_log,
            &mut player_faction,
            &mut cockpit_state,
            &mut game_mode,
            &mut troop_transport_state,
        ) {
            Ok(()) => false,
            Err(error) => {
                tactical_test_fixture::emit_failed(request, &error);
                true
            }
        }
    } else {
        false
    };
    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    let mut interface_fixture_frames = 0_u32;
    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    let mut interface_fixture_emitted = tactical_fixture_failed;
    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    let mut fleet_move_watch = interface_test_fixture::FleetMoveWatch::default();
    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    let mut fleet_load_watch = interface_test_fixture::FleetLoadWatch::default();
    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    let mut speed_menu_watch = interface_test_fixture::SpeedMenuWatch::default();
    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    let mut operation_report_watch = interface_test_fixture::OperationReportWatch::default();
    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    let mut fixture_operation_report: Option<OperationReportState> = None;
    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    let mut quadrant_watch = interface_test_fixture::QuadrantWatch::default();
    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    let mut fleet_finder_watch = interface_test_fixture::FleetFinderWatch::default();
    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    let interface_fixture_active =
        interface_fixture_request.is_some() || tactical_fixture_request.is_some();
    #[cfg(not(all(target_arch = "wasm32", feature = "interface-test-fixtures")))]
    let interface_fixture_active = false;

    // ── Apply Star Wars theme ────────────────────────────────────────────
    // Must happen inside the macroquad async context, after first frame init.
    let mut theme_applied = false;
    let mut strategic_windowing = strategic_windowing::StrategicWindowing::new();
    // Temporary projection from manager commits into the three detail
    // families that still use the legacy cross-family focus serial.
    let mut fleet_legacy_bridge_serial = 0_u64;
    let mut galaxy_focus = GalaxyFocusState::default();
    // A load's saved windows, waiting for its Fleet windows' commit.
    let mut pending_window_restore: Option<Vec<rebellion_data::save::SavedWindow>> = None;
    let mut strategic_input_router = rebellion_render::windowing_input::FrameInputRouter::default();
    let mut strategic_frame_number = 0_u64;
    let strategic_input_subscriber = macroquad::input::utils::register_input_subscriber();
    let mut strategic_raw_input = StrategicRawInputCollector::default();
    let mut focus_loss_watch = strategic_windowing::FocusLossWatch::default();
    #[cfg(target_os = "macos")]
    let mut native_focus = strategic_windowing::FocusStateCounter::default();
    #[cfg(target_os = "macos")]
    let mut native_active_logged: Option<bool> = None;
    // The systems whose battle the combat loop left for a later pass.
    let mut held_battles: Vec<SystemKey> = Vec::new();
    // One site's AI in-day steps (`FUN_0051dea0` `+0x20`) of a day, through
    // the shared driver; each order's record is written and shown. The
    // player's agent takes the player's slot, except in dual-AI mode, where
    // an AI plays the player's side.
    macro_rules! run_ai_steps {
        ($steps:expr, $day:expr) => {{
            let steps = $steps;
            // In dual-AI mode the agent's last order is dropped, so turning
            // the mode off later applies no stale order.
            let mut agent = if secondary_ai_state.is_none() {
                Some(rebellion_data::simulation::AgentSlot {
                    agent: &mut player_agent,
                    alliance: player_faction == MissionFaction::Alliance,
                    economy: &economy_state,
                    applied: None,
                })
            } else {
                player_agent.take_pending();
                None
            };
            let mut sides: Vec<(&mut AIState, bool)> = vec![(&mut ai_state, false)];
            if let Some(second) = secondary_ai_state.as_mut() {
                sides.push((second, true));
            }
            for step in steps {
                let mut ctx = rebellion_data::simulation::AiCtx {
                    world: &mut world,
                    missions: &mut mission_state,
                    manufacturing: &mut mfg_state,
                    movement: &mut movement_state,
                    transport: &mut troop_transport_state,
                    deliveries: &mut delivery_state,
                    stockpiles: &mut stockpile_state,
                    victory: &mut victory_state,
                    blockade: &mut blockade_state,
                    research: &research_state,
                    uprising: &uprising_state,
                    config: &game_config,
                };
                let (records, runs) = rebellion_data::simulation::ai_step(
                    &mut ctx,
                    &mut sides,
                    agent.as_mut(),
                    step,
                    $day,
                    &|world, transport, action| held_at(world, transport, &held_battles, action),
                    &mut |n| (0..n).map(|_| sim_rng.gen::<f64>()).collect(),
                );
                if let Some(order) = agent.as_ref().and_then(|slot| slot.applied.as_ref()) {
                    macroquad::logging::info!("[agent] order={:?}", order);
                }
                show_ai_records(
                    &records,
                    $day,
                    &world,
                    &text_templates,
                    &mut msg_log,
                    #[cfg(not(target_arch = "wasm32"))]
                    &mut audio_engine,
                    #[cfg(not(target_arch = "wasm32"))]
                    &audio_vol,
                );
                movie_queue.file(strike_orders::strike_movies(
                    &world,
                    &text_templates,
                    records
                        .iter()
                        .filter(|record| record.outcome.is_applied())
                        .map(|record| &record.strike),
                    Faction::of_alliance(player_faction == MissionFaction::Alliance),
                ));
                show_blockade_events(
                    $day,
                    &runs,
                    &mut world,
                    &mut mfg_state,
                    &mut troop_transport_state,
                    &movement_state,
                    &mut delivery_state,
                    &blockade_state,
                    &text_templates,
                    &mut msg_log,
                );
            }
        }};
    }

    loop {
        let dt = get_frame_time();
        rebellion_render::control_registry::begin_frame();
        // Scripted input posts one event a frame; AppKit delivers it before
        // the next frame, as it delivers hardware input.
        #[cfg(not(target_arch = "wasm32"))]
        if let Err(why) = dev_input.post_next() {
            eprintln!("[dev-input] {why}");
            dev_channel.refuse(&format!("input not posted: {why}"));
        }
        macroquad::input::utils::repeat_all_miniquad_input(
            &mut strategic_raw_input,
            strategic_input_subscriber,
        );
        let strategic_raw_input_events = strategic_raw_input.take();
        let mut targeting_cursor_drawn = false;
        #[cfg(target_arch = "wasm32")]
        if !browser_menu_audio_requested
            && (is_mouse_button_pressed(macroquad::input::MouseButton::Left)
                || is_key_pressed(KeyCode::Enter)
                || is_key_pressed(KeyCode::Space)
                || is_key_pressed(KeyCode::Tab))
        {
            browser_menu_audio_requested = true;
        }

        // Apply theme on first frame (egui context exists after first next_frame)
        if !theme_applied {
            egui_macroquad::ui(|ctx| {
                rebellion_render::theme::load_fonts(ctx);
                rebellion_render::theme::apply_theme(ctx);
            });
            egui_macroquad::draw();
            theme_applied = true;
        }

        // ── Cockpit droids ──────────────────────────────────────────────────
        // The agent's step count runs with the game clock; the droids'
        // players tick on wall time (ghidra/notes/droid-advisor-triggers.md).
        if !interface_fixture_active {
            let player_is_alliance = cockpit_state.faction == CockpitFaction::Alliance;
            for (code, audience) in msg_log.take_advice() {
                if audience.includes(player_is_alliance) {
                    advisor_state.post_code(code);
                }
            }
        }
        cockpit_state.briefing = !interface_fixture_active && advisor_state.holds_clock();
        // The droids act in the command center only, so the briefing waits
        // for the opening movie. The Battle Alert opens over the command
        // center, droids and all (original capture 22-battle-alert-result).
        let droids_shown = game_mode == GameMode::Galaxy
            || (game_mode == GameMode::TacticalCombat && tactical_state.over_command_center());
        if !interface_fixture_active && droids_shown {
            advisor_state.update(
                dt,
                clock
                    .is_running()
                    .then(|| clock.speed.steps_per_second())
                    .flatten()
                    .filter(|_| !advisor_state.holds_clock() && !movie_queue.days_stopped()),
            );
            for step in advisor_state.take_cockpit_steps() {
                // Step 3 closes the ending script: the side's word becomes
                // the strategic window's ending code (FUN_004c3060 →
                // FUN_0041d830).
                if step.kind == 3 {
                    movie_queue.end(
                        victory_state.word,
                        Faction::of_alliance(cockpit_state.faction == CockpitFaction::Alliance),
                    );
                }
                if step.kind == 2 {
                    apply_briefing_step(
                        step.step,
                        &world,
                        &mut cockpit_state,
                        &mut pending_index_command,
                        player_agent.advice(),
                    );
                    // Step 13 files the opening advice topics (FUN_00439f20).
                    if step.step == 13 && player_agent.advice() {
                        let topics = advice_topics.release_opening(|id| text_templates.get(id));
                        file_advice_topics(
                            &mut msg_log,
                            topics,
                            clock.tick,
                            cockpit_state.faction == CockpitFaction::Alliance,
                        );
                    }
                }
            }
            // The side's update releases a held topic every 300 steps
            // (FUN_004866b0 → FUN_00439bc0).
            if player_agent.advice() && !advisor_state.holds_clock() {
                use rebellion_render::advice_topics::TopicWindow;
                let now = advisor_state.now();
                let mut topics = advice_topics.tick(now, |id| text_templates.get(id));
                // The first window of each kind releases one topic
                // (FUN_00429ce0, FUN_0045aac0 → FUN_0043a0b0).
                for (window, open) in [
                    (TopicWindow::Sector, sector_window_state.any_open()),
                    (TopicWindow::System, system_window_state.any_open()),
                    (TopicWindow::Fleet, fleet_window_state.any_open()),
                    (
                        TopicWindow::SystemDefenses,
                        defenses_window_state.any_open(),
                    ),
                    (TopicWindow::Missions, missions_window_state.any_open()),
                ] {
                    if open {
                        topics.extend(
                            advice_topics.window_opened(window, now, |id| text_templates.get(id)),
                        );
                    }
                }
                file_advice_topics(
                    &mut msg_log,
                    topics,
                    clock.tick,
                    cockpit_state.faction == CockpitFaction::Alliance,
                );
            }
            #[cfg(not(target_arch = "wasm32"))]
            play_advisor_voices(
                &mut audio_engine,
                &mut advisor_state,
                &mut advisor_voice_side,
                &audio_vol,
            );
            #[cfg(not(target_arch = "wasm32"))]
            play_message_sound(
                &mut audio_engine,
                pending_message_sound.take(),
                &advisor_state,
                &audio_vol,
            );
            #[cfg(target_arch = "wasm32")]
            if let Some(engine) = browser_menu_audio.as_mut() {
                play_message_sound(
                    engine,
                    pending_message_sound.take(),
                    &advisor_state,
                    &audio_vol,
                );
            }
            #[cfg(target_arch = "wasm32")]
            match browser_menu_audio.as_mut() {
                Some(engine) => play_advisor_voices(
                    engine,
                    &mut advisor_state,
                    &mut advisor_voice_side,
                    &audio_vol,
                ),
                None => drop(advisor_state.take_voices()),
            }
        }

        // ── Global keyboard shortcuts ───────────────────────────────────────
        if let GameMode::Cutscene { kind } = &game_mode {
            // Esc or the left button ends any movie (FUN_005f7240, 0x404);
            // any other key ends all but the trench-run pair (FUN_00421790).
            let other_key = get_last_key_pressed().is_some_and(|key| key != KeyCode::Escape);
            if is_key_pressed(KeyCode::Escape)
                || is_mouse_button_pressed(MouseButton::Left)
                || (other_key && movies::keys_dismiss(kind.movie()))
            {
                if let Some(player) = cutscene_player.as_mut() {
                    player.stop();
                }
                caption_without_movie = false;
            }
        } else if is_key_pressed(KeyCode::Escape)
            && game_mode != GameMode::Galaxy
        {
            if matches!(game_mode, GameMode::GameOptions { .. }) {
                // The window handles no key (FUN_00406b80); Escape leaves a
                // focused name edit or answers a confirmation No
                // (GameOptionsState::key, FUN_00416f30), and nothing else.
            } else if game_mode == GameMode::MultiplayerSetup {
                game_mode = GameMode::MainMenu;
            } else if game_mode == GameMode::MainMenu && main_menu_state.registry_open() {
                // The Fleet Registry answers Escape by closing itself.
            } else {
                quit_requested = true;
            }
        }
        // The briefing tour holds the cockpit's input (FUN_0041d9d0's hooks).
        // Escape on release, or a left or right press, skips it once
        // (lpfn_0041d8f0, lpfn_0041d950 → FUN_0043a200).
        let briefing_lock = game_mode == GameMode::Galaxy && advisor_state.tour_active();
        if briefing_lock
            && (is_key_released(KeyCode::Escape)
                || is_mouse_button_pressed(MouseButton::Left)
                || is_mouse_button_pressed(MouseButton::Right))
        {
            advisor_state.skip_tour();
        }
        rebellion_render::game_speed::hold_speed_while_open(
            &mut clock,
            &mut advice_saved_speed,
            &mut encyclopedia_speed_held,
            encyclopedia_surface.is_open(),
        );
        // ── Tick the clock (Galaxy mode only) ────────────────────────────────
        // A new game's clock holds through the briefing until its step 11,
        // and stops for good once the advisor stamps its ending slot.
        let tick_events = if game_mode == GameMode::Galaxy
            && !movie_queue.days_stopped()
            && (interface_fixture_active || !advisor_state.holds_clock())
        {
            // Fast forward (dev): exactly one day a frame, as the systems
            // read `tick_events.last()`.
            #[cfg(not(target_arch = "wasm32"))]
            let clock_dt = match clock.speed.seconds_per_day() {
                // Only while a wait that needs days is pending: between
                // commands the clock runs at the chosen speed.
                Some(seconds)
                    if dev_policy.fast_forward
                        && dev_wait_until.is_some_and(|(condition, _)| condition.needs_clock()) =>
                {
                    seconds
                }
                _ => dt,
            };
            #[cfg(target_arch = "wasm32")]
            let clock_dt = dt;
            clock.advance(clock_dt)
        } else {
            vec![]
        };

        #[cfg(not(target_arch = "wasm32"))]
        rebellion_data::telemetry::set_native_seq(
            command_script.as_ref().and_then(|script| script.last_seq()),
        );
        if !tick_events.is_empty() {
            // Every prisoner and where it is held, for the release messages
            // (notification 0x1e0) once the day is done.
            let held_at_dawn: Vec<(CharacterKey, String)> = world
                .characters
                .iter()
                .filter(|(_, c)| c.is_captive && !c.is_killed)
                .filter_map(|(key, _)| Some((key, prison_name(&world, key)?)))
                .collect();
            // Used by the Dabora 2 notification paths that need to timestamp
            // message-log entries. `current_tick` is re-bound further down for
            // the rest of the tick loop; this earlier binding is read-only.
            let economy_tick = tick_events.last().map_or(0, |e| e.tick);

            // Active movement orders are authoritative; repair stale orbit
            // indexes before economy and manufacturing inspect fleet presence.
            reconcile_fleet_orbits(&movement_state, &mut world);

            // The day before closes: the AI rounds its frames left run (a
            // fast-forward day, a frame that crossed several steps, or a first
            // round a battle held), then every side's order count clears
            // (FUN_00485de0 via phase 13). Its last round's order stays
            // pending. port: a frame that crosses several days closes only the
            // day before the first; the days between run no rounds and write
            // no ai_day, as a headless call carrying several days does.
            if ai_state.scheduler.day_open() {
                let day = tick_events[0].tick - 1;
                let rate = rebellion_data::simulation::day_rate(&clock, &game_config);
                run_ai_steps!(ai_state.scheduler.next_step()..=rate, day);
                let mut sides: Vec<(&mut AIState, bool)> = vec![(&mut ai_state, false)];
                if let Some(second) = secondary_ai_state.as_mut() {
                    sides.push((second, true));
                }
                for closed in
                    rebellion_data::simulation::close_ai_days(&mut sides, &clock, &game_config, day)
                {
                    rebellion_data::telemetry::native_ai_day(clock.tick, &closed);
                }
                player_agent.end_day();
            }

            // ── Economy (runs BEFORE manufacturing — affects production) ──────
            let economy_events =
                EconomySystem::advance(&mut economy_state, &mut world, &tick_events);
            for ev in &economy_events {
                // Knesset Shamash-Bet Dabora 2 notification events — surface
                // them in the interactive message log.
                if let EconomyEvent::ResourceDiscovered { system, new_output } = ev {
                    let name = world
                        .systems
                        .get(*system)
                        .map_or_else(|| "unknown".into(), |s| s.name.clone());
                    // Notification 0x20, System Resources Messages.
                    msg_log.push(filed(
                        GameMessage::at_system(
                            economy_tick,
                            format!("New resources discovered at {name} ({new_output} units)"),
                            MessageCategory::Event,
                            *system,
                        ),
                        MessageRail::Resource,
                        system_audience(&world, *system),
                    ));
                }
                rebellion_data::telemetry::native_economy_record(economy_tick, &world, ev);
            }

            // ── Stockpiles: facility cycles, yard work, overdraft scraps ─────
            let stockpile_rolls: Vec<f64> = (0..tick_events.len())
                .map(|_| sim_rng.gen::<f64>())
                .collect();
            let stockpile_events = StockpileSystem::advance(
                &mut stockpile_state,
                &world,
                &mut mfg_state,
                &tick_events,
                blockade_state.blockaded_systems(),
                &stockpile_rolls,
            );
            // Notification 0xc: each overdraft scrap's notice, named before
            // the scrap removes the unit.
            rebellion_data::telemetry::native_maintenance_shortfall(
                economy_tick,
                &world,
                &troop_transport_state,
                &stockpile_events,
            );
            for event in &stockpile_events {
                if let StockpileEvent::OverdraftScrap { side, target } = *event {
                    if let Some(message) = maintenance_shortfall_message(
                        &world,
                        &troop_transport_state,
                        &text_templates,
                        side,
                        target,
                        economy_tick,
                    ) {
                        msg_log.push(message);
                    }
                }
            }
            let overdraft = rebellion_core::scrap::scrap_all_reporting(
                &mut world,
                &mut stockpile_state,
                &mut troop_transport_state,
                stockpile_events.iter().filter_map(|event| match *event {
                    StockpileEvent::OverdraftScrap { target, .. } => Some(target),
                    _ => None,
                }),
            );
            for loss in &overdraft.losses {
                let system = world.systems.get(loss.system).map_or("", |value| value.name.as_str());
                macroquad::logging::info!("{}", loss.log_line(system));
            }
            // A scrapped regiment's holder rerun (FUN_00508660).
            fire_support_events(
                &mut world,
                &mut mfg_state,
                &mut troop_transport_state,
                &movement_state,
                &mut delivery_state,
                &text_templates,
                &mut msg_log,
                economy_tick,
                0,
            );

            // ── Manufacturing (blockaded systems are skipped) ─────────────────
            // Use advance_tracked so we also pick up K6 EVT_MANUFACTURING_IDLE
            // transitions for the interactive message log.
            let mfg_advance = ManufacturingSystem::advance_tracked(
                &mut mfg_state,
                &tick_events,
                blockade_state.blockaded_systems(),
            );
            // An ordered capital ship completes in place in its fleet
            // (FUN_0052bee0).
            rebellion_core::manufacturing::finish_ships(
                &mut world,
                &movement_state,
                &mfg_advance.finished_ships,
            );
            for completion in &mfg_advance.completions {
                // Apply the built item to the game world (ships, facilities, troops).
                rebellion_data::integrator::apply_build_completion_inner(completion, &mut world);
                let sys_name = world
                    .systems
                    .get(completion.system)
                    .map_or_else(|| "unknown".into(), |s| s.name.clone());
                // Notification 0x21, Construction Complete.
                msg_log.push(filed(
                    GameMessage::at_system(
                        completion.tick,
                        format!("Construction complete at {sys_name}"),
                        MessageCategory::Manufacturing,
                        completion.system,
                    )
                    // FUN_00497940: a deployed unit carries code 3.
                    .with_advice(3),
                    MessageRail::Manufacturing,
                    system_audience(&world, completion.system),
                ));
                #[cfg(not(target_arch = "wasm32"))]
                audio_engine.play_sfx(SfxKind::BuildComplete, &audio_vol);
            }
            // Remote products complete at their facility, then travel to
            // their destination and complete there on arrival (F-030).
            let system_name = |world: &GameWorld, key| {
                world
                    .systems
                    .get(key)
                    .map_or_else(|| "unknown".to_string(), |s| s.name.clone())
            };
            for departure in &mfg_advance.departures {
                let origin = system_name(&world, departure.origin);
                let destination =
                    rebellion_data::integrator::destination_name(&world, departure.destination);
                // Notification 0x21, Construction Complete.
                msg_log.push(filed(
                    GameMessage::at_system(
                        departure.tick,
                        format!("Construction complete at {origin}, en route to {destination}"),
                        MessageCategory::Manufacturing,
                        departure.origin,
                    )
                    .with_advice(3),
                    MessageRail::Manufacturing,
                    system_audience(&world, departure.origin),
                ));
                #[cfg(not(target_arch = "wasm32"))]
                audio_engine.play_sfx(SfxKind::BuildComplete, &audio_vol);
            }
            // A capital ship travels inside its destination fleet.
            let travelling =
                rebellion_data::integrator::travelling_products(&mfg_advance.departures);
            delivery_state.depart(&world, &travelling);
            let delivered = delivery_state.advance(&world, &tick_events);
            let ships = rebellion_core::delivery::advance_ship_legs(
                &mut world,
                &movement_state,
                &tick_events,
            );
            for completion in &delivered.arrivals {
                rebellion_data::integrator::apply_build_completion_inner(completion, &mut world);
                if matches!(
                    completion.kind,
                    rebellion_core::manufacturing::BuildableKind::SpecialForce(_)
                ) {
                    rebellion_core::death_star::deploy_special_forces(
                        &mut world,
                        &troop_transport_state,
                        &movement_state,
                        completion.system,
                    );
                }
            }
            for boarded in &delivered.boarded {
                rebellion_data::integrator::apply_boarding_inner(
                    boarded,
                    &mut world,
                    &mut troop_transport_state,
                );
            }
            rebellion_data::integrator::apply_ship_arrivals_inner(&mut world, &ships);
            // Ordered ships whose queue entries are gone.
            rebellion_core::manufacturing::seat_ordered_ships(&mut world, &mut mfg_state);
            // A finished regiment's holder rerun (FUN_00508660).
            fire_support_events(
                &mut world,
                &mut mfg_state,
                &mut troop_transport_state,
                &movement_state,
                &mut delivery_state,
                &text_templates,
                &mut msg_log,
                economy_tick,
                0,
            );
            let ships = ships.iter().map(|ship| rebellion_core::manufacturing::CompletionEvent {
                system: ship.system,
                tick: ship.tick,
                kind: rebellion_core::manufacturing::BuildableKind::CapitalShip(ship.class),
            });
            let arrived = delivered
                .arrivals
                .iter()
                .cloned()
                .chain(delivered.boarded.iter().map(|boarded| boarded.completion.clone()))
                .chain(ships);
            for completion in arrived {
                // Notification 0xd, Unit Arrival.
                msg_log.push(filed(
                    GameMessage::at_system(
                        completion.tick,
                        format!("Unit arrived at {}", system_name(&world, completion.system)),
                        MessageCategory::Manufacturing,
                        completion.system,
                    )
                    // FUN_004981c0: arriving units carry code 6.
                    .with_advice(6),
                    MessageRail::Manufacturing,
                    system_audience(&world, completion.system),
                ));
            }
            for lost in &delivered.lost {
                // Event 0x303, GameObjDestroyedOnArrivalNotif.
                msg_log.push(GameMessage::at_system(
                    lost.arrival_tick,
                    format!(
                        "Unit lost on arrival: {} no longer exists",
                        system_name(&world, lost.destination)
                    ),
                    MessageCategory::Manufacturing,
                    lost.origin,
                ));
            }
            // K6 EVT_MANUFACTURING_IDLE (0x160) — surface idle transitions
            // in the player-facing message log.
            for &system in &mfg_advance.newly_idle {
                let sys_name_str = world
                    .systems
                    .get(system)
                    .map_or_else(|| "unknown".into(), |s| s.name.clone());
                // FUN_0048be60: an idle facility carries code 3, for the
                // side that holds it.
                let message = GameMessage::at_system(
                    economy_tick,
                    format!("Manufacturing queue idle at {sys_name_str}"),
                    MessageCategory::Manufacturing,
                    system,
                );
                msg_log.push(match system_audience(&world, system) {
                    Some(audience) => message.with_advice_to(3, audience),
                    None => message,
                });
            }

            // ── Movement ────────────────────────────────────────────────────
            let arrivals = MovementSystem::advance(&mut movement_state, &tick_events);
            for arrival in &arrivals {
                let Some(applied) = apply_fleet_arrival(
                    &mut world,
                    &movement_state,
                    &mut troop_transport_state,
                    arrival,
                ) else {
                    continue;
                };
                let sys_name = world
                    .systems
                    .get(arrival.system)
                    .map_or_else(|| "unknown".into(), |s| s.name.clone());
                // A fleet that joined another names it.
                let fleet_name = arrival
                    .join
                    .and_then(|join| world.fleet_name(join))
                    .or_else(|| world.fleet_name(arrival.fleet))
                    .unwrap_or_default()
                    .to_owned();
                // The message plays its sound when shown (FUN_00469de0);
                // filing it plays none.
                msg_log.push(fleet_arrival_message(
                    &text_templates,
                    arrival.join.unwrap_or(arrival.fleet),
                    &fleet_name,
                    arrival.system,
                    &sys_name,
                    arrival.tick,
                    applied.is_alliance,
                ));
            }
            // Regiments travelling on their own (FUN_00556430, event 0x387).
            let now = tick_events.last().map_or(clock.tick, |event| event.tick);
            let regiments = troop_transport_state.advance_transit(&mut world, now);
            // An arrival's holder rerun (FUN_00508660).
            fire_support_events(
                &mut world,
                &mut mfg_state,
                &mut troop_transport_state,
                &movement_state,
                &mut delivery_state,
                &text_templates,
                &mut msg_log,
                now,
                0,
            );
            sweep_flagged(
                &mut world,
                &mut mission_state,
                &troop_transport_state,
                &movement_state,
                &blockade_state,
                now,
            );
            for transit in &regiments.arrived {
                let system = match transit.leg {
                    RegimentLeg::Surface(system) => Some(system),
                    RegimentLeg::Fleet(fleet) => world.fleets.get(fleet).map(|f| f.location),
                };
                let Some(system) = system else { continue };
                let name = world
                    .systems
                    .get(system)
                    .map_or_else(|| "unknown".into(), |s| s.name.clone());
                let audience = world
                    .troops
                    .get(transit.troop)
                    .map(|troop| RailAudience::side(troop.is_alliance));
                // Notification 0xd, Unit Arrival.
                msg_log.push(filed(
                    GameMessage::at_system(
                        transit.arrival_tick,
                        format!("Regiment arrived at {name}"),
                        MessageCategory::Mission,
                        system,
                    )
                    .with_advice(6),
                    MessageRail::Fleet,
                    audience,
                ));
            }
            for transit in &regiments.lost {
                // Event 0x303, GameObjDestroyedOnArrivalNotif.
                msg_log.push(GameMessage::at_system(
                    transit.arrival_tick,
                    "Regiment lost on arrival: its destination no longer exists".to_string(),
                    MessageCategory::Mission,
                    transit.origin,
                ));
            }
            for transit in &regiments.squadrons_arrived {
                let system = match transit.leg {
                    RegimentLeg::Surface(system) => Some(system),
                    RegimentLeg::Fleet(fleet) => world.fleets.get(fleet).map(|f| f.location),
                };
                let Some(system) = system else { continue };
                let name = world
                    .systems
                    .get(system)
                    .map_or_else(|| "unknown".into(), |s| s.name.clone());
                // Notification 0xd, Unit Arrival.
                msg_log.push(filed(
                    GameMessage::at_system(
                        transit.arrival_tick,
                        format!("Squadron arrived at {name}"),
                        MessageCategory::Mission,
                        system,
                    )
                    .with_advice(6),
                    MessageRail::Fleet,
                    Some(RailAudience::side(transit.is_alliance)),
                ));
            }
            for transit in &regiments.squadrons_lost {
                // Event 0x303, GameObjDestroyedOnArrivalNotif.
                msg_log.push(GameMessage::at_system(
                    transit.arrival_tick,
                    "Squadron lost on arrival: its destination no longer exists".to_string(),
                    MessageCategory::Mission,
                    transit.origin,
                ));
            }

            // No regiment lands without an order: the assault (0x234) or a
            // Move (0x201) puts one down (ghidra/notes/ground-assault.md,
            // "Landing without the order").
            let current_tick = tick_events.last().map_or(0, |event| event.tick);

            // ── Combat ──────────────────────────────────────────────────────
            // After fleet arrivals, check every system for opposing fleets.
            // Collect combat triggers first (immutable world borrow).
            let combat_triggers: Vec<_> = world
                .systems
                .keys()
                .filter_map(|sys_key| {
                    // FUN_0051ddf0 -> FUN_00517270: a system
                    // fights on every pass that finds both sides present;
                    // when both remain the battle fires again next pass.
                    // There is no cooldown.
                    // Each side's lead is its first fleet that fights.
                    let alliance_fleets =
                        rebellion_core::combat::fighting_fleets(&world, sys_key, true);
                    let empire_fleets =
                        rebellion_core::combat::fighting_fleets(&world, sys_key, false);
                    // FUN_00509710 / FUN_0050b8e0: both sides present, each
                    // with a fleet or squadrons the system holds for it.
                    (side_present(&world, sys_key, true) && side_present(&world, sys_key, false))
                        .then(|| {
                            (
                                sys_key,
                                alliance_fleets.first().copied(),
                                empire_fleets.first().copied(),
                            )
                        })
                })
                .collect();

            // The systems whose battle waits this frame: the one the Battle
            // Alert opens and every trigger the loop leaves for a later pass.
            let mut engaged_systems: Vec<SystemKey> = Vec::new();
            let mut triggers = combat_triggers.into_iter();
            while let Some((sys_key, atk_force, def_force)) = triggers.next() {
                let sys_name = world
                    .systems
                    .get(sys_key)
                    .map_or_else(|| "Unknown".into(), |s| s.name.clone());

                // Check if the player is involved in this battle.
                let player_is_alliance = player_faction == MissionFaction::Alliance;
                let atk_is_alliance = true;
                let def_is_alliance = false;
                // Player is involved if either side is the player's faction.
                let player_involved = (player_is_alliance == atk_is_alliance)
                    || (player_is_alliance == def_is_alliance);
                if player_involved && game_mode == GameMode::Galaxy {
                    // Transition to tactical combat view for player-involved battles.
                    let player_is_attacker = if player_is_alliance {
                        atk_is_alliance
                    } else {
                        !atk_is_alliance
                    };
                    // The battle's AI threshold reads the ships' tags.
                    rebellion_core::carriage::tag_all(&mut world);
                    if let Err(error) = tactical_flow::begin_player_battle(
                        &world,
                        Some(&mfg_state),
                        tactical_flow::BattleEntry {
                            system: sys_key,
                            attacker: CombatForce {
                                fleet: atk_force,
                                is_alliance: true,
                            },
                            defender: CombatForce {
                                fleet: def_force,
                                is_alliance: false,
                            },
                            player_is_attacker,
                            tick: current_tick,
                            rng_seed: sim_rng.gen::<u32>(),
                        },
                        &mut tactical_state,
                        &mut msg_log,
                        &mut game_mode,
                    ) {
                        eprintln!("Skipping invalid tactical battle at {sys_name}: {error:?}");
                        continue;
                    }
                    tactical_state.set_display_options(game_options_state.tactical_flags());
                    if let Some((title, summary)) = battle_alert_text(
                        &world,
                        &text_templates,
                        blockade_state.is_blockaded(sys_key),
                        sys_key,
                        player_is_alliance,
                    ) {
                        tactical_state.set_battle_alert_text(title, summary);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    audio_engine.play_sfx(SfxKind::CombatStart, &audio_vol);
                    engaged_systems.push(sys_key);
                    engaged_systems.extend(triggers.by_ref().map(|(system, _, _)| system));
                    break; // Handle one player battle at a time.
                }

                // Nobody watches this battle: it runs headless (FUN_0040a050
                // state 8), as the shared simulation's battles do.
                let space_result = rebellion_data::integrator::resolve_system_space_combat_for(
                    &mut world,
                    &mut troop_transport_state,
                    &mut movement_state,
                    sys_key,
                    sim_rng.gen::<u32>(),
                    current_tick,
                    Some(player_is_alliance),
                    Some(&mfg_state),
                    &ProductRoom::new(&mfg_state, &delivery_state),
                );
                for line in &space_result.log {
                    macroquad::logging::info!("{}", line);
                }
                rebellion_data::telemetry::native_combat_space(current_tick, &world, &space_result);

                let winner_str = match space_result.winner {
                    CombatSide::Attacker => "Alliance victory",
                    CombatSide::Defender => "Empire victory",
                    CombatSide::Draw => "Draw",
                };
                msg_log.push(GameMessage::at_system(
                    current_tick,
                    format!("Space battle at {sys_name} — {winner_str}"),
                    MessageCategory::Combat,
                    sys_key,
                ));
                #[cfg(not(target_arch = "wasm32"))]
                audio_engine.play_sfx(SfxKind::CombatStart, &audio_vol);
            }

            // The battles left for a later pass hold the AI's orders there
            // until the next day's combat (`held_at`). The opened battle
            // closes before any round runs, as rounds run in the galaxy only.
            held_battles = engaged_systems.get(1..).unwrap_or_default().to_vec();

            // Observe embarked regiments after arrivals and landings, so a
            // fleet that arrives and is ordered away again this tick still
            // copies the arrival system's withdraw percent.
            let mut running_regiments = if tick_events.is_empty() {
                Vec::new()
            } else {
                BlockadeSystem::running_regiments(
                    &mut blockade_state,
                    &world,
                    &movement_state,
                    &troop_transport_state,
                )
            };

            // The blockade rule reruns as an arrival or a battle settles,
            // before the AI's orders take effect (`FUN_0050b8e0` through
            // event 0x311, `ghidra/notes/blockade-bit.md`, "When it runs").
            let mut blockade_events =
                BlockadeSystem::advance(&mut blockade_state, &world, &tick_events);
            // The same rule writes the battle bit; each change sweeps the
            // system's adrift characters (event 0x304, FUN_00548e90).
            if let Some(last) = tick_events.last() {
                for (system, started) in rebellion_core::death_star::sweep_battle_changes(
                    &mut world,
                    &mut mission_state,
                    &mut blockade_state,
                    &troop_transport_state,
                    &movement_state,
                    last.tick,
                ) {
                    rebellion_data::telemetry::native_adrift_trips(
                        last.tick,
                        &world,
                        &mission_state,
                        system,
                        &started,
                    );
                }
                sweep_flagged(
                    &mut world,
                    &mut mission_state,
                    &troop_transport_state,
                    &movement_state,
                    &blockade_state,
                    last.tick,
                );
            }

            // ── Fog of war ──────────────────────────────────────────────────
            let alliance_reveals =
                FogSystem::advance(&mut fog_alliance_state, &world, &movement_state);
            let empire_reveals = FogSystem::advance(&mut fog_empire_state, &world, &movement_state);
            let reveals = if player_faction == MissionFaction::Alliance {
                alliance_reveals
            } else {
                empire_reveals
            };
            for reveal in &reveals {
                let sys_name = world
                    .systems
                    .get(reveal.system)
                    .map_or_else(|| "unknown".into(), |s| s.name.clone());
                msg_log.push(GameMessage::at_system(
                    tick_events.last().unwrap().tick,
                    format!("System {sys_name} revealed"),
                    MessageCategory::Event,
                    reveal.system,
                ));
            }

            // ── Missions ────────────────────────────────────────────────────
            let mission_rolls: Vec<f64> = (0..mission_state.len()
                * tick_events.len()
                * rebellion_core::missions::ROLLS_PER_MISSION)
                .map(|_| sim_rng.gen::<f64>())
                .collect();
            let mission_advance = MissionSystem::advance(
                &mut mission_state,
                &world,
                &uprising_state,
                &tick_events,
                &mission_rolls,
            );
            // Member moves and releases come before the results.
            rebellion_core::missions::apply_advance_effects(&mut world, &mission_advance.effects);
            rebellion_data::telemetry::native_mission_ended(&mission_advance.ended);
            ai_state.free_mission_members(&mission_advance.effects);
            if let Some(second_ai) = secondary_ai_state.as_mut() {
                second_ai.free_mission_members(&mission_advance.effects);
            }

            for result in &mission_advance.results {
                #[cfg(not(target_arch = "wasm32"))]
                if result.faction == player_faction {
                    dev_missions_resolved.push(result.mission_id);
                }
                apply_mission_result(
                    result,
                    &mut world,
                    &mut msg_log,
                    #[cfg(not(target_arch = "wasm32"))]
                    &mut audio_engine,
                    #[cfg(not(target_arch = "wasm32"))]
                    &audio_vol,
                );
                movie_queue.file(strike_orders::sabotage_movies(
                    &world,
                    &text_templates,
                    &result.effects,
                ));
                let research_results = ResearchSystem::apply_mission_points(
                    &mut world,
                    &mut research_state,
                    &result.effects,
                );
                rebellion_data::telemetry::native_research_results(result.tick, &research_results);
                push_research_reports(&mut msg_log, result.tick, &research_results);
                for ended in rebellion_data::integrator::apply_mission_state_effects(
                    &result.effects,
                    &mut world,
                    result.tick,
                    &mut uprising_state,
                ) {
                    if let rebellion_core::uprising::UprisingEvent::UprisingEnded { system, tick } =
                        ended
                    {
                        let name = world
                            .systems
                            .get(system)
                            .map_or_else(|| "unknown".into(), |s| s.name.clone());
                        for message in uprising_messages(
                            &world,
                            &text_templates,
                            system,
                            tick,
                            false,
                            format!("Uprising subdued at {name}"),
                        ) {
                            msg_log.push(message);
                        }
                    }
                }
            }

            // A support gain's control change (FUN_005109f0, event 0x31b).
            fire_support_events(
                &mut world,
                &mut mfg_state,
                &mut troop_transport_state,
                &movement_state,
                &mut delivery_state,
                &text_templates,
                &mut msg_log,
                tick_events.last().map_or(0, |e| e.tick),
                0,
            );
            sweep_flagged(
                &mut world,
                &mut mission_state,
                &troop_transport_state,
                &movement_state,
                &blockade_state,
                tick_events.last().map_or(0, |e| e.tick),
            );

            // ── Members going home from ended missions (FUN_00548120) ──────
            for ended in &mission_advance.ended {
                let started = rebellion_core::death_star::return_members(
                    &mut world,
                    &mut mission_state,
                    &troop_transport_state,
                    &movement_state,
                    &blockade_state,
                    ended,
                    current_tick,
                );
                // Notification 0x1f1 on the end code (FUN_00491500).
                let returning_to =
                    mission_returning_place(&world, &mission_state, ended, &started);
                let luke_trained =
                    event_state.has_fired(rebellion_core::events::EVT_DAGOBAH_COMPLETED);
                let voice = mission_end_voice_list(&world, ended, luke_trained)
                    .map_or(0, |list| advisor_state.voice_line(list));
                for message in mission_end_messages(
                    &world,
                    &text_templates,
                    ended,
                    returning_to.as_deref(),
                    voice,
                ) {
                    msg_log.push(message);
                }
                if let Some(system) = ended.origin {
                    rebellion_data::telemetry::native_adrift_trips(
                        current_tick,
                        &world,
                        &mission_state,
                        system,
                        &started,
                    );
                }
            }

            // ── Character escapes ────────────────────────────────────────────
            let escape_effects =
                MissionSystem::escape_step(&mut world, current_tick, sim_rng.gen::<f64>());
            for effect in &escape_effects {
                if let MissionEffect::CharacterEscaped { character, .. } = effect {
                    // The release's autoscrap request sweeps the escapee's
                    // location at once (event 0x307, FUN_00548da0).
                    let started = rebellion_core::death_star::free_escapee(
                        &mut world,
                        &mut mission_state,
                        &troop_transport_state,
                        &movement_state,
                        &blockade_state,
                        *character,
                        current_tick,
                    );
                    if let Some(system) =
                        world.characters.get(*character).and_then(|c| c.current_system)
                    {
                        rebellion_data::telemetry::native_adrift_trips(
                            current_tick,
                            &world,
                            &mission_state,
                            system,
                            &started,
                        );
                    }
                }
            }

            // ── Events ──────────────────────────────────────────────────────
            let event_rolls: Vec<f32> = (0..16).map(|_| sim_rng.gen::<f32>()).collect();
            let fired_events =
                EventSystem::advance(&mut event_state, &world, &tick_events, &event_rolls);

            // #F7 + #A3: call the pub'd integrator helper. DisplayMessage
            // routes through `GameEffect::StoryMessageDisplayed` and
            // `SpawnSpecialForce` resolves via `current_system` + fallback
            // to `MovementState::orders()`. The effect buffer drains into
            // `msg_log` immediately below.
            let mut story_effects_out: Vec<rebellion_core::effects::GameEffect> = Vec::new();
            for fired in &fired_events {
                rebellion_data::integrator::apply_event_action_to_world(
                    &fired.actions,
                    &mut world,
                    &mut story_effects_out,
                    fired.tick,
                    &movement_state,
                );
            }
            // Drain StoryMessageDisplayed + SpecialForceSpawned effects into
            // the interactive message log. SpecialForceUnit arena wiring is
            // handled in apply_event_action_to_world; here we just log.
            for eff in story_effects_out {
                use rebellion_core::effects::GameEffect;
                match eff {
                    GameEffect::StoryMessageDisplayed { text, .. } => {
                        msg_log.push(GameMessage::new(current_tick, text, MessageCategory::Event));
                    }
                    GameEffect::SpecialForceSpawned {
                        at_system,
                        is_alliance,
                    } => {
                        let name = world
                            .systems
                            .get(at_system)
                            .map_or_else(|| "unknown".into(), |s| s.name.clone());
                        let side = if is_alliance { "Alliance" } else { "Imperial" };
                        msg_log.push(GameMessage::at_system(
                            current_tick,
                            format!("{side} special force lands at {name}"),
                            MessageCategory::Event,
                            at_system,
                        ));
                    }
                    _ => {}
                }
            }

            // Apply Jedi training from story events (outside the event-action
            // helper because it needs jedi_state which is not in scope there).
            for fired in &fired_events {
                for action in &fired.actions {
                    if let EventAction::StartJediTraining { character } = action {
                        if let Some(c) = world.characters.get(*character) {
                            jedi_state.start_training(*character, c.is_alliance, current_tick);
                        }
                    }
                }
            }

            // ── Blockade ─────────────────────────────────────────────────────
            if let Some(last) = tick_events.last() {
                running_regiments.extend(BlockadeSystem::running_regiments(
                    &mut blockade_state,
                    &world,
                    &movement_state,
                    &troop_transport_state,
                ));
                let running_rolls: Vec<f64> = (0..running_regiments.len())
                    .map(|_| sim_rng.gen::<f64>())
                    .collect();
                blockade_events.extend(BlockadeSystem::resolve_running(
                    &running_regiments,
                    &running_rolls,
                    last.tick,
                ));
            }
            show_blockade_events(
                current_tick,
                &blockade_events,
                &mut world,
                &mut mfg_state,
                &mut troop_transport_state,
                &movement_state,
                &mut delivery_state,
                &blockade_state,
                &text_templates,
                &mut msg_log,
            );

            // ── Uprising and disaster (FUN_0050b800, events 0x38d and 0x38f) ──
            let uprising_tick = tick_events.last().map_or(0, |event| event.tick);
            let uprising_budget =
                UprisingSystem::roll_budget(&uprising_state, &world, uprising_tick);
            let uprising_rolls: Vec<f64> =
                (0..uprising_budget).map(|_| sim_rng.gen::<f64>()).collect();
            let uprising_events = UprisingSystem::advance(
                &mut uprising_state,
                &world,
                &economy_state,
                &mission_state,
                &tick_events,
                &uprising_rolls,
            );
            for evt in &uprising_events {
                use rebellion_core::uprising::UprisingEvent;
                let (system, tick) = match evt {
                    UprisingEvent::UprisingBegan { system, tick }
                    | UprisingEvent::UprisingEnded { system, tick }
                    | UprisingEvent::UprisingIncident { system, tick, .. }
                    | UprisingEvent::Disaster { system, tick, .. }
                    | UprisingEvent::RevoltSupportLoss { system, tick, .. } => (*system, *tick),
                };
                let name = world
                    .systems
                    .get(system)
                    .map_or_else(|| "unknown".into(), |s| s.name.clone());
                let (text, category) = match evt {
                    UprisingEvent::UprisingBegan { .. } | UprisingEvent::UprisingEnded { .. } => {
                        let began = matches!(evt, UprisingEvent::UprisingBegan { .. });
                        let fallback = if began {
                            format!("Uprising at {name}!")
                        } else {
                            format!("The uprising at {name} has ended")
                        };
                        for message in uprising_messages(
                            &world,
                            &text_templates,
                            system,
                            tick,
                            began,
                            fallback,
                        ) {
                            msg_log.push(message);
                        }
                        rebellion_core::uprising::apply_uprising_event(&mut world, evt);
                        continue;
                    }
                    UprisingEvent::UprisingIncident { .. } => (
                        format!("Uprising incident at {name}"),
                        MessageCategory::Diplomacy,
                    ),
                    UprisingEvent::Disaster { .. } => (
                        format!("Natural disaster strikes {name}"),
                        MessageCategory::Event,
                    ),
                    // Timer 0x384 raises no notice (FUN_0050caa0).
                    UprisingEvent::RevoltSupportLoss { .. } => {
                        rebellion_core::uprising::apply_uprising_event(&mut world, evt);
                        continue;
                    }
                };
                // port: incidents and disasters have no traced class yet.
                msg_log.push(filed(
                    GameMessage::at_system(tick, text, category, system),
                    MessageRail::PopularSupport,
                    system_audience(&world, system),
                ));
                rebellion_core::uprising::apply_uprising_event(&mut world, evt);
            }
            fire_support_events(
                &mut world,
                &mut mfg_state,
                &mut troop_transport_state,
                &movement_state,
                &mut delivery_state,
                &text_templates,
                &mut msg_log,
                tick_events.last().map_or(0, |e| e.tick),
                0,
            );
            sweep_flagged(
                &mut world,
                &mut mission_state,
                &troop_transport_state,
                &movement_state,
                &blockade_state,
                tick_events.last().map_or(0, |e| e.tick),
            );

            // ── Betrayal ─────────────────────────────────────────────────────
            let betrayal_rolls: Vec<f64> = (0..world.characters.len())
                .map(|_| sim_rng.gen::<f64>())
                .collect();
            let empty_loyalty_tb = MstbTable::new(vec![]);
            let loyalty_tb = world
                .mission_tables
                .get("UPRIS1TB")
                .unwrap_or(&empty_loyalty_tb);
            let betrayal_events = BetrayalSystem::advance(
                &mut betrayal_state,
                &world,
                &tick_events,
                &betrayal_rolls,
                loyalty_tb,
            );
            for evt in &betrayal_events {
                let rebellion_core::betrayal::BetrayalEvent::CharacterBetrayed {
                    character,
                    defected_to_alliance,
                } = evt;
                if let Some(c) = world.characters.get_mut(*character) {
                    c.is_alliance = *defected_to_alliance;
                    c.is_empire = !*defected_to_alliance;
                }
                // Remove from current fleet
                for (_, fleet) in &mut world.fleets {
                    fleet.characters.retain(|&k| k != *character);
                }
                let name = world
                    .characters
                    .get(*character)
                    .map_or_else(|| "Unknown".into(), |c| c.display_name().to_owned());
                let to_faction = if *defected_to_alliance {
                    "Alliance"
                } else {
                    "Empire"
                };
                msg_log.push(GameMessage::new(
                    current_tick,
                    format!("{name} has betrayed and defected to the {to_faction}!"),
                    MessageCategory::Event,
                ));
            }

            // ── Research ─────────────────────────────────────────────────────
            let research_rolls: Vec<f64> = (0..tick_events.len())
                .map(|_| sim_rng.gen::<f64>())
                .collect();
            let research_results = ResearchSystem::advance(
                &mut research_state,
                &mut world,
                &uprising_state,
                &tick_events,
                &research_rolls,
            );
            rebellion_data::telemetry::native_research_results(current_tick, &research_results);
            push_research_reports(&mut msg_log, current_tick, &research_results);

            // ── Repair ──────────────────────────────────────────────────────
            let repair_rolls: Vec<f64> = (0..RepairSystem::roll_budget(&world, tick_events.len()))
                .map(|_| sim_rng.gen::<f64>())
                .collect();
            let repair_events = RepairSystem::advance(
                &mut repair_state,
                &world,
                &uprising_state,
                &tick_events,
                &repair_rolls,
            );
            for evt in &repair_events {
                evt.apply(&mut world);
            }
            rebellion_data::telemetry::native_repair_events(current_tick, &world, &repair_events);

            // ── Jedi training ────────────────────────────────────────────────
            let jedi_rolls: Vec<f64> = (0..jedi_state.training.len().max(1))
                .map(|_| sim_rng.gen::<f64>())
                .collect();
            let jedi_events =
                JediSystem::advance(&mut jedi_state, &world, &tick_events, &jedi_rolls);
            for evt in &jedi_events {
                match evt {
                    rebellion_core::jedi::JediEvent::TierAdvanced {
                        character,
                        new_tier,
                    } => {
                        if let Some(c) = world.characters.get_mut(*character) {
                            c.force_tier = *new_tier;
                            // Persist XP: set to threshold for the new tier
                            c.force_experience = match new_tier {
                                rebellion_core::world::ForceTier::None => 0,
                                rebellion_core::world::ForceTier::Aware => 1,
                                rebellion_core::world::ForceTier::Training => {
                                    rebellion_core::jedi::XP_TO_TRAINING
                                }
                                rebellion_core::world::ForceTier::Experienced => {
                                    rebellion_core::jedi::XP_TO_EXPERIENCED
                                }
                            };
                        }
                        let name = world
                            .characters
                            .get(*character)
                            .map_or_else(|| "Unknown".into(), |c| c.display_name().to_owned());
                        let tier_str = match new_tier {
                            rebellion_core::world::ForceTier::None => "None",
                            rebellion_core::world::ForceTier::Aware => "Force Aware",
                            rebellion_core::world::ForceTier::Training => "Jedi Training",
                            rebellion_core::world::ForceTier::Experienced => "Jedi Knight",
                        };
                        // Notification 0x1d, Force Skill Improvement.
                        msg_log.push(filed(
                            GameMessage::new(
                                current_tick,
                                format!("{name} has reached {tier_str} tier"),
                                MessageCategory::Event,
                            )
                            // FUN_0048ed80: Force growth carries the character's code.
                            .with_advice(personnel_advice(&world, *character)),
                            MessageRail::Mission,
                            character_audience(&world, *character),
                        ));
                    }
                    rebellion_core::jedi::JediEvent::TrainingComplete { character } => {
                        jedi_state.stop_training(*character);
                    }
                    rebellion_core::jedi::JediEvent::JediDiscovered { character, .. } => {
                        if let Some(c) = world.characters.get_mut(*character) {
                            c.is_discovered_jedi = true;
                        }
                        let name = world
                            .characters
                            .get(*character)
                            .map_or_else(|| "Unknown".into(), |c| c.display_name().to_owned());
                        msg_log.push(GameMessage::new(
                            current_tick,
                            format!("{name}'s Force sensitivity discovered!"),
                            MessageCategory::Event,
                        ));
                    }
                }
            }

            // ── AI ───────────────────────────────────────────────────────────
            // Phase 8 (FUN_004361a0) follows every due timer of phase 5, so
            // the day's first round runs after the daily stages; the later
            // rounds run over the day's frames (below). A player battle or a
            // cutscene opened this frame holds the round until it closes:
            // hyp: the Battle Alert is modal before the step's phase 8; how
            // FUN_0040a050's alert states gate FUN_0051dea0's steps is
            // untraced. The first galaxy frame after it runs the round.
            let headquarters_only = campaign_config.victory_conditions
                == rebellion_core::world::VictoryConditions::HeadquartersOnly;
            ai_state.set_headquarters_only(headquarters_only);
            ai_state.scheduler.day_started = true;
            if let Some(second) = secondary_ai_state.as_mut() {
                second.set_headquarters_only(headquarters_only);
                second.scheduler.day_started = true;
            }
            if game_mode == GameMode::Galaxy {
                rebellion_core::carriage::tag_all(&mut world);
                run_ai_steps!(0..=0, current_tick);
            }

            // ── Releases (notification 0x1e0, FUN_00490340) ─────────────────
            for (character, location) in &held_at_dawn {
                if world
                    .characters
                    .get(*character)
                    .is_some_and(|c| !c.is_captive && !c.is_killed)
                {
                    let luke_trained =
                        event_state.has_fired(rebellion_core::events::EVT_DAGOBAH_COMPLETED);
                    let voice = major_voice_list(&world, *character, luke_trained, RELEASE_VOICES)
                        .map_or(0, |list| advisor_state.voice_line(list));
                    for message in release_messages(
                        &world,
                        &text_templates,
                        current_tick,
                        *character,
                        location,
                        voice,
                    ) {
                        msg_log.push(message);
                    }
                }
            }

            // ── Victory check ────────────────────────────────────────────────
            // The win word (FUN_005180e0); a change is delivered as
            // notification 0x128, which files its movie messages and, with
            // GO set, hands the word to the local player's advisor
            // (FUN_00486890), whose ending slot stops the clock.
            let word = (!tick_events.is_empty() && !victory_state.resolved)
                .then(|| {
                    VictorySystem::word(&victory_state, &world, campaign_config.victory_conditions)
                })
                .flatten();
            if let Some(word) = word.filter(|&word| word != victory_state.word) {
                movie_queue.file(strike_orders::word_movies(
                    &text_templates,
                    victory_state.word,
                    word,
                ));
                victory_state.word = word;
                if word & rebellion_core::victory::GO != 0 {
                    victory_state.resolved = true;
                    if advisor_state.post_word(word) {
                        movie_queue.stop_days();
                    }
                }
            }
        }

        // ── AI in-day rounds (FUN_0051dea0 `+0x20`) ──────────────────────
        // The day's later steps, spread over its frames, and a first round a
        // closed battle held. Rounds run while the clock runs in the galaxy.
        if game_mode == GameMode::Galaxy
            && (interface_fixture_active || !advisor_state.holds_clock())
            && clock.tick >= 1
        {
            let rounds = game_config.ai.rounds_per_day.at(clock.speed);
            let next = ai_state.scheduler.next_step();
            if let Some(upto) = clock.in_day_step(rounds).filter(|&upto| next <= upto) {
                rebellion_core::carriage::tag_all(&mut world);
                run_ai_steps!(next..=upto, clock.tick);
            }
        }

        // FUN_004eeed0, FUN_00560d50: a rank lapses as soon as its holder
        // is taken, killed, sent on a mission or moved off its post.
        rebellion_core::officer_rank::clear_lapsed_ranks(&mut world);

        rebellion_data::telemetry::flush_native_traces(clock.tick);

        // ── Developer commands ───────────────────────────────────────────────

        let mut panel_actions: Vec<PanelAction> = Vec::new();
        // A command script runs one line a frame from the main menu or the
        // galaxy, once the last line's commands have run.
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(script) = command_script.as_mut() {
            for (seq, line) in dev_channel.poll(get_time()) {
                script.push_live(seq, line);
            }
            // What the waits and policies read of the game this frame.
            let dev_observed = dev_policy::Observed {
                day: clock.tick,
                mode: format!("{game_mode:?}")
                    .split(|c: char| !c.is_alphanumeric())
                    .next()
                    .unwrap_or_default()
                    .to_owned(),
                galaxy: game_mode == GameMode::Galaxy,
                main_menu: game_mode == GameMode::MainMenu,
                battle_alert: tactical_state.battle_alert_open().then(|| {
                    tactical_state
                        .session
                        .as_ref()
                        .map_or_else(|| "?".to_owned(), |session| session.system_name.clone())
                }),
                results: game_mode == GameMode::TacticalCombat
                    && tactical_state.over_command_center()
                    && !tactical_state.battle_alert_open(),
                pause_alert: clock.pause_requested(),
                paused: clock.speed.seconds_per_day().is_none(),
                briefing: advisor_state.holds_clock(),
                ending: movie_queue.days_stopped(),
                advice_index: message_index
                    .as_ref()
                    .is_some_and(|index| index.category() == dev_commands::MESSAGE_CATEGORIES[9].1),
            };
            // A wait for the alert or the battle itself must find it standing.
            let dev_waits_for_battle = matches!(
                dev_wait_until,
                Some((
                    dev_commands::WaitCondition::BattleAlert | dev_commands::WaitCondition::Battle,
                    _
                ))
            );
            if !dev_waits_for_battle
                && (dev_wait_until.is_some()
                    || dev_policy_hold.1 > 0
                    || get_time() < dev_policy_hold.0)
            {
                dev_policy.answer_modals(&dev_observed, &mut dev_input);
            }
            dev_policy_hold.1 = dev_policy_hold.1.saturating_sub(1);
            if let Some((condition, frames)) = dev_wait_until.as_mut() {
                use dev_commands::WaitCondition;
                let met = match *condition {
                    WaitCondition::Day(day) => clock.tick >= day,
                    WaitCondition::BattleAlert => tactical_state.battle_alert_open(),
                    // The battlefield itself, not the Battle Alert or the
                    // Results, which show in the same mode.
                    WaitCondition::Battle => {
                        game_mode == GameMode::TacticalCombat
                            && !tactical_state.over_command_center()
                    }
                    WaitCondition::Galaxy => game_mode == GameMode::Galaxy,
                    WaitCondition::MissionResolved(None) => {
                        dev_missions_resolved.len() as u64 > dev_wait_baseline
                    }
                    WaitCondition::MissionResolved(Some(id)) => dev_missions_resolved.contains(&id),
                    WaitCondition::Message(category) => msg_log.messages().iter().any(|message| {
                        message.id > dev_wait_baseline
                            && message
                                .audience
                                .includes(player_faction == MissionFaction::Alliance)
                            && message.rail.is_some_and(|rail| {
                                rebellion_render::message_index::category_rail(category)
                                    .map_or(true, |wanted| rail == wanted)
                            })
                    }),
                    WaitCondition::PauseAlert => clock.pause_requested(),
                    WaitCondition::BriefingSkipped => dev_policy::skip_briefing(&mut advisor_state),
                    WaitCondition::Tech {
                        alliance,
                        tech,
                        level,
                    } => research_state.level(alliance, tech) >= level,
                };
                let blocker = if met {
                    None
                } else {
                    dev_policy.blocker(&dev_observed, *condition)
                };
                if let Some(blocker) = blocker {
                    dev_channel.refuse(&blocker);
                    dev_wait_until = None;
                } else if met {
                    dev_channel.note(&format!("day={} after {frames} frames", clock.tick));
                    dev_wait_until = None;
                } else if *frames >= dev_wait_budget.unwrap_or(dev_commands::WAIT_UNTIL_FRAMES) {
                    dev_channel.refuse(&format!(
                        "waited {frames} frames for {condition:?}; day={} mode={game_mode:?}",
                        clock.tick
                    ));
                    dev_wait_until = None;
                } else {
                    *frames += 1;
                }
            }
            let held = dev_hold_frames > 0
                || get_time() < dev_hold_until
                || dev_wait_until.is_some()
                || dev_input.busy();
            dev_hold_frames = dev_hold_frames.saturating_sub(1);
            let galaxy_ready = matches!(game_mode, GameMode::MainMenu | GameMode::Galaxy)
                && pending_interface.is_empty();
            // A command that runs anywhere does not wait for the galaxy, and a
            // galaxy command never runs on the main menu's background world;
            // a palette line (Start game) runs from the menu too. A malformed
            // line runs at once, to be refused.
            let in_galaxy = matches!(game_mode, GameMode::Galaxy) && pending_interface.is_empty();
            let ready = !held
                && match script.peek().map(dev_commands::parse_line) {
                    Some(Ok(Some(command))) => command.readiness().admits(in_galaxy),
                    Some(Err(_)) => true,
                    _ => galaxy_ready,
                };
            // A line that has waited too long for the galaxy view is refused
            // rather than holding back every line behind it (a Battle Alert
            // keeps the game out of the galaxy until something closes it).
            let waiting = !held && !ready && script.peek().is_some();
            dev_galaxy_wait_frames = if waiting {
                dev_galaxy_wait_frames.saturating_add(1)
            } else {
                0
            };
            if let GameMode::Cutscene { kind } = &game_mode {
                if dev_policy::skips_movie(waiting, dev_wait_until.map(|(c, _)| c)) {
                    eprintln!("[dev-command] movie {:#x} ended as Escape", kind.movie());
                    if let Some(player) = cutscene_player.as_mut() {
                        player.stop();
                    }
                    caption_without_movie = false;
                }
            }
            if dev_galaxy_wait_frames >= DEV_GALAXY_WAIT_FRAMES {
                dev_galaxy_wait_frames = 0;
                if let dev_commands::ScriptStep::Run(line) = script.step(true) {
                    eprintln!(
                        "[dev-command] run #{}: {line}",
                        script
                            .last_seq()
                            .map_or_else(|| "-".to_owned(), |seq| seq.to_string())
                    );
                    dev_channel.begin(script.last_seq(), &line);
                    dev_channel.refuse(&format!(
                        "waited {DEV_GALAXY_WAIT_FRAMES} frames for the galaxy view; the game is in {game_mode:?}"
                    ));
                }
            } else {
                match script.step(ready) {
                    dev_commands::ScriptStep::Wait => {}
                    dev_commands::ScriptStep::Run(line) => {
                        dev_queue_drained = false;
                        // The marker scripts/dev-report.py splits game.log at.
                        eprintln!(
                            "[dev-command] run #{}: {line}",
                            script
                                .last_seq()
                                .map_or_else(|| "-".to_owned(), |seq| seq.to_string())
                        );
                        dev_channel.begin(script.last_seq(), &line);
                        match dev_commands::parse_line(&line) {
                            Err(why) => dev_channel.refuse(&why),
                            Ok(Some(command)) => {
                                eprintln!("[dev-command] sent {line:?}");
                                dev_commands_ready.push(command);
                            }
                            Ok(None) => {
                                command_palette_state.refresh_script(&world);
                                match command_palette_state
                                    .command_named(&line)
                                    .map(|command| command.action.clone())
                                {
                                    Some(rebellion_render::PaletteAction::Panel(action)) => {
                                        eprintln!("[dev-command] sent {line:?}");
                                        panel_actions.push(action);
                                    }
                                    Some(rebellion_render::PaletteAction::Interface(command)) => {
                                        eprintln!("[dev-command] sent {line:?}");
                                        pending_interface.push(command);
                                    }
                                    // Script entries parse as DevCommands before here.
                                    Some(rebellion_render::PaletteAction::Script(_)) => {}
                                    None => {
                                        eprintln!("[dev-command] unknown {line:?}");
                                        dev_channel.refuse("no command has this name");
                                    }
                                }
                            }
                        }
                    }
                    dev_commands::ScriptStep::Done => {
                        if !dev_queue_drained {
                            eprintln!("[dev-command] done");
                            dev_queue_drained = true;
                        }
                        if !dev_channel.is_live() {
                            command_script = None;
                        }
                    }
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        for command in std::mem::take(&mut dev_commands_ready) {
            use dev_commands::DevCommand;
            // What `Save:` and `Load:` must not run behind.
            let dev_modal = if tactical_state.battle_alert_open() {
                Some("the Battle Alert")
            } else if clock.pause_requested() {
                Some("the pause alert")
            } else if mission_dialog_state.is_open() {
                Some("the mission dialog")
            } else {
                None
            };
            if matches!(
                command,
                DevCommand::WaitUntil(_) | DevCommand::WaitUntilWithin(..)
            ) {
                dev_wait_budget = None;
            }
            let command = match command {
                DevCommand::WaitUntilWithin(condition, frames) => {
                    dev_wait_budget = Some(frames);
                    DevCommand::WaitUntil(condition)
                }
                command => command,
            };
            match command {
                DevCommand::Capture(name) => {
                    dev_capture = Some(name.unwrap_or_else(|| {
                        format!("capture-{}", dev_channel.next_capture_number())
                    }));
                }
                DevCommand::WaitMs(ms) => {
                    dev_hold_until = get_time() + f64::from(ms) / 1000.0;
                    dev_policy_hold.0 = dev_hold_until;
                }
                DevCommand::WaitFrames(frames) => {
                    dev_hold_frames = frames;
                    dev_policy_hold.1 = frames;
                }
                // Rewritten to `WaitUntil` above.
                DevCommand::WaitUntilWithin(..) => {}
                DevCommand::WaitUntil(condition) => {
                    // A mission the player's side is not running resolves
                    // never (or, an old id, at once): refuse it up front.
                    if let dev_commands::WaitCondition::MissionResolved(Some(id)) = condition {
                        if !mission_state
                            .missions()
                            .iter()
                            .any(|mission| mission.id == id && mission.faction == player_faction)
                        {
                            dev_channel.refuse(&format!(
                                "mission {id} is not an active mission of the player's side"
                            ));
                            continue;
                        }
                    }
                    dev_wait_baseline = match condition {
                        dev_commands::WaitCondition::Message(_) => {
                            msg_log.messages().last().map_or(0, |message| message.id)
                        }
                        _ => dev_missions_resolved.len() as u64,
                    };
                    dev_wait_until = Some((condition, 0));
                }
                DevCommand::FastForward(on) => {
                    dev_policy.fast_forward = on;
                    dev_channel.note(&format!("fast_forward={}", if on { "on" } else { "off" }));
                }
                DevCommand::BattlePolicy(policy) => {
                    dev_policy.battle = policy;
                    dev_channel.note(&format!("battle policy {policy:?}"));
                }
                DevCommand::AdvicePolicy(policy) => {
                    dev_policy.advice = policy;
                    dev_channel.note(&format!("advice policy {policy:?}"));
                }
                // Two frames after the last step let the game read it.
                DevCommand::Click { x, y, right, ctrl } => {
                    dev_input.click(x, y, right, ctrl);
                    dev_hold_frames = 2;
                }
                DevCommand::ClickControl { name, right } => match dev_commands::control_rect(&name)
                {
                    Ok(rect) => {
                        let (x, y) = rect.center();
                        dev_input.click(x, y, right, false);
                        dev_hold_frames = 2;
                        dev_channel.note(&format!("{name} at {x:.0},{y:.0}"));
                    }
                    Err(why) => dev_channel.refuse(&why),
                },
                DevCommand::LocateControl(name) => match dev_commands::control_rect(&name) {
                    Ok(rect) => {
                        let (x, y) = rect.center();
                        dev_channel.note(&format!(
                            "{name} rect {:.0},{:.0} {:.0}x{:.0} center {x:.0},{y:.0}",
                            rect.x, rect.y, rect.width, rect.height
                        ));
                    }
                    Err(why) => dev_channel.refuse(&why),
                },
                DevCommand::ListControls(window) => {
                    let names = rebellion_render::control_registry::names(window.as_deref());
                    if names.is_empty() {
                        dev_channel.refuse(&format!(
                            "no controls registered{}",
                            window.map_or_else(String::new, |window| format!(" under {window:?}"))
                        ));
                    } else {
                        dev_channel.note(&format!("[{}]", names.join(", ")));
                    }
                }
                DevCommand::Drag { from, to, ctrl } => {
                    dev_input.drag(from, to, ctrl);
                    dev_hold_frames = 2;
                }
                DevCommand::HoldDrag { from, to } => {
                    dev_input.hold_drag(from, to);
                    dev_hold_frames = 2;
                }
                DevCommand::Release { x, y } => {
                    dev_input.release(x, y);
                    dev_hold_frames = 2;
                }
                DevCommand::Scroll { x, y, notches } => {
                    dev_input.scroll(x, y, notches);
                    dev_hold_frames = 2;
                }
                DevCommand::Press(key) => match dev_input.press(&key) {
                    Ok(()) => dev_hold_frames = 2,
                    Err(why) => dev_channel.refuse(&why),
                },
                DevCommand::OpenMessageIndex { category, row } => {
                    let player_is_alliance = player_faction == MissionFaction::Alliance;
                    let index = message_index.get_or_insert_with(MessageIndexState::new);
                    let mut actions = index.set_category(category);
                    let rows = rebellion_render::message_index::filtered_messages(
                        &msg_log,
                        category,
                        player_is_alliance,
                    );
                    let row = row.map(|row| match row {
                        dev_commands::Row::Number(n) => n,
                        dev_commands::Row::Last => rows.len(),
                    });
                    match row {
                        None => {
                            index.return_to_list();
                            dev_channel.note(&format!("{} rows", rows.len()));
                        }
                        Some(row) => match index.view_row(&rows, row) {
                            Some(action) => {
                                let title = rows[row - 1]
                                    .display
                                    .as_ref()
                                    .map_or(rows[row - 1].text.as_str(), |d| d.title.as_str());
                                eprintln!("[message_index] viewing row {row}: {title}");
                                dev_channel.note(&format!("row {row} of {}: {title}", rows.len()));
                                actions.push(action);
                                // FUN_00469de0: a report row opens the
                                // report window and closes the Index.
                                actions.extend(index.route_report(&rows));
                            }
                            None => {
                                dev_channel.refuse(&format!("row {row} of {} rows", rows.len()));
                            }
                        },
                    }
                    let report = actions.iter().find_map(|action| match action {
                        MessageIndexAction::OpenReport(id) => Some(*id),
                        _ => None,
                    });
                    if let Some(id) = report {
                        if let Some(message) = msg_log.get(id) {
                            operation_report_state.open(
                                &world,
                                message,
                                cockpit_state.faction,
                                Some(category),
                            );
                        }
                        actions.extend(
                            message_index
                                .take()
                                .map(|index| index.close())
                                .into_iter()
                                .flatten(),
                        );
                    }
                    apply_message_index_actions(
                        actions,
                        &mut msg_log,
                        &mut clock,
                        &mut advice_saved_speed,
                        &mut pending_message_sound,
                    );
                }
                DevCommand::PostMessage { class, system } => {
                    let result = dev_actions::find_system(&world, &system).and_then(|key| {
                        dev_actions::post_message(
                            &world,
                            &text_templates,
                            &mut msg_log,
                            class,
                            key,
                            clock.tick,
                            player_faction == MissionFaction::Alliance,
                        )
                    });
                    match result {
                        Ok(detail) => dev_channel.note(&detail),
                        Err(why) => dev_channel.refuse(&why),
                    }
                }
                DevCommand::BattleAt(ref system) | DevCommand::Blockade(ref system) => {
                    let battle = matches!(command, DevCommand::BattleAt(_));
                    let result = dev_actions::find_system(&world, system).and_then(|key| {
                        if battle {
                            dev_actions::battle_at(
                                &mut world,
                                &movement_state,
                                &mut troop_transport_state,
                                key,
                                clock.tick,
                            )
                        } else {
                            dev_actions::blockade(
                                &mut world,
                                &movement_state,
                                &mut troop_transport_state,
                                key,
                                clock.tick,
                            )
                        }
                    });
                    match result {
                        Ok(detail) => dev_channel.note(&detail),
                        Err(why) => dev_channel.refuse(&why),
                    }
                }
                DevCommand::KnockOutHyperdrive(n) => {
                    let result = tactical_state
                        .session
                        .as_mut()
                        .ok_or_else(|| "no battle is running".to_owned())
                        .and_then(|session| session.knock_out_player_hyperdrive(n));
                    match result {
                        Ok(detail) => dev_channel.note(&detail),
                        Err(why) => dev_channel.refuse(&why),
                    }
                }
                DevCommand::GiveDeathStar(ref fleet) => {
                    match dev_actions::give_death_star(&mut world, fleet) {
                        Ok(detail) => dev_channel.note(&detail),
                        Err(why) => dev_channel.refuse(&why),
                    }
                }
                DevCommand::Bombard {
                    kind,
                    side,
                    ref fleet,
                } => {
                    let player = Faction::of_alliance(player_faction == MissionFaction::Alliance);
                    let side = side.unwrap_or(player);
                    let mut states = rebellion_data::integrator::StrikeStates {
                        movement: &mut movement_state,
                        manufacturing: &mut mfg_state,
                        transport: &mut troop_transport_state,
                        deliveries: &mut delivery_state,
                        missions: &mut mission_state,
                        uprisings: &uprising_state,
                        blockade: &mut blockade_state,
                        tick: clock.tick,
                        // hyp: the order's command runs in the next step's phase 5,
                        // before that step's AI round (FUN_0051df30, FUN_004361a0).
                        step: ai_state.scheduler.next_step(),
                    };
                    let result = dev_actions::find_fleet(&world, side, fleet).and_then(|key| {
                        if side == player {
                            // The menu row's enable test first (FUN_00581e80).
                            rebellion_core::bombardment::order_enabled(
                                &world,
                                states.movement,
                                &[key],
                                kind,
                                side,
                            )
                            .and_then(|()| {
                                strike_orders::issue(
                                    &mut world,
                                    &[key],
                                    kind,
                                    &mut states,
                                    &mut || sim_rng.gen::<f64>(),
                                    &text_templates,
                                    &mut msg_log,
                                )
                            })
                            .map_err(|refusal| format!("{refusal:?}"))
                        } else {
                            // The other side's fleet strikes as the AI's does,
                            // and its messages file as an AI record's
                            // (show_ai_records, native_ai_records).
                            dev_actions::ai_strike(&mut world, key, kind, &mut states, &mut || {
                                sim_rng.gen::<f64>()
                            })
                            .map(|strike| {
                                rebellion_data::telemetry::native_strike(
                                    clock.tick, &world, &strike,
                                );
                                for message in strike_orders::strike_messages(
                                    &world,
                                    &text_templates,
                                    clock.tick,
                                    &strike,
                                ) {
                                    msg_log.push(message);
                                }
                                vec![strike]
                            })
                        }
                    });
                    match result {
                        Ok(runs) => {
                            movie_queue.file(strike_orders::strike_movies(
                                &world,
                                &text_templates,
                                &runs,
                                player,
                            ));
                            match dev_actions::strike_detail(kind, &runs) {
                                Ok(detail) => dev_channel.note(&detail),
                                Err(detail) => dev_channel.refuse(&detail),
                            }
                        }
                        Err(why) => dev_channel.refuse(&why),
                    }
                }
                DevCommand::Assault(ref fleet) => {
                    let alliance = player_faction == MissionFaction::Alliance;
                    let found = world
                        .fleets
                        .iter()
                        .filter(|(_, value)| value.is_alliance == alliance)
                        .map(|(key, _)| key)
                        .find(|&key| {
                            world
                                .fleet_name(key)
                                .is_some_and(|name| name.eq_ignore_ascii_case(fleet.trim()))
                        });
                    match found {
                        None => dev_channel.refuse(&format!("no fleet of yours named {fleet:?}")),
                        Some(key) => match assault_orders::issue(
                            &mut world,
                            &[key],
                            Faction::of_alliance(alliance),
                            &mut rebellion_data::integrator::StrikeStates {
                                movement: &mut movement_state,
                                manufacturing: &mut mfg_state,
                                transport: &mut troop_transport_state,
                                deliveries: &mut delivery_state,
                                missions: &mut mission_state,
                                uprisings: &uprising_state,
                                blockade: &mut blockade_state,
                                tick: clock.tick,
                                // hyp: the order's command runs in the next step's phase 5,
                                // before that step's AI round (FUN_0051df30, FUN_004361a0).
                                step: ai_state.scheduler.next_step(),
                            },
                            &mut || sim_rng.gen::<f64>(),
                            &text_templates,
                            &mut msg_log,
                        ) {
                            Ok(runs) => dev_channel.note(&format!(
                                "0x234: {}",
                                runs.iter()
                                    .filter_map(|run| run.report.as_ref())
                                    .map(|report| format!(
                                        "attackers {} defenders {} shots {} engagements {} attackers lost {} defenders lost {} facilities {} energy {} raw {} landed {} holder {:?}",
                                        report.attackers,
                                        report.defenders,
                                        report.shots.iter().filter(|shot| shot.fired).count(),
                                        report.engagements.len(),
                                        report.attackers_lost.len(),
                                        report.defenders_lost.len(),
                                        report.facilities_lost.len(),
                                        report.energy,
                                        report.raw_material,
                                        report.landed.len(),
                                        report.holder_after,
                                    ))
                                    .collect::<Vec<_>>()
                                    .join("; ")
                            )),
                            Err(refusal) => dev_channel.refuse(&format!("{refusal:?}")),
                        },
                    }
                }
                DevCommand::DamageShip(ref fleet) => {
                    let alliance = player_faction == MissionFaction::Alliance;
                    match dev_actions::damage_ship(&mut world, alliance, fleet) {
                        Ok(detail) => dev_channel.note(&detail),
                        Err(why) => dev_channel.refuse(&why),
                    }
                }
                DevCommand::QueueBuild {
                    alliance,
                    ref class,
                    ref system,
                } => {
                    let result = dev_actions::find_system(&world, system).and_then(|key| {
                        dev_actions::queue_build(
                            &mut world,
                            &research_state,
                            &mut mfg_state,
                            &troop_transport_state,
                            &delivery_state,
                            &movement_state,
                            alliance,
                            class,
                            key,
                        )
                    });
                    match result {
                        Ok(detail) => dev_channel.note(&detail),
                        Err(why) => dev_channel.refuse(&why),
                    }
                }
                DevCommand::StripDefenders(ref system) => {
                    let result = dev_actions::find_system(&world, system).and_then(|key| {
                        dev_actions::strip_defenders(
                            &mut world,
                            &mut troop_transport_state,
                            key,
                            clock.tick,
                        )
                    });
                    // The removed regiments' holder rerun (FUN_00508660),
                    // served before a save can follow.
                    fire_support_events(
                        &mut world,
                        &mut mfg_state,
                        &mut troop_transport_state,
                        &movement_state,
                        &mut delivery_state,
                        &text_templates,
                        &mut msg_log,
                        clock.tick,
                        0,
                    );
                    match result {
                        Ok(detail) => dev_channel.note(&detail),
                        Err(why) => dev_channel.refuse(&why),
                    }
                }
                DevCommand::Embark { ref fleet, count } => {
                    let result = dev_actions::embark(
                        &mut world,
                        &mut troop_transport_state,
                        &movement_state,
                        blockade_state.blockaded_systems(),
                        &ProductRoom::new(&mfg_state, &delivery_state),
                        player_faction == MissionFaction::Alliance,
                        fleet,
                        count,
                        clock.tick,
                    );
                    // Each boarding's holder rerun (FUN_00508660), served
                    // before a save can follow.
                    fire_support_events(
                        &mut world,
                        &mut mfg_state,
                        &mut troop_transport_state,
                        &movement_state,
                        &mut delivery_state,
                        &text_templates,
                        &mut msg_log,
                        clock.tick,
                        0,
                    );
                    match result {
                        Ok(detail) => dev_channel.note(&detail),
                        Err(why) => dev_channel.refuse(&why),
                    }
                }
                DevCommand::MaintenanceCapacity { alliance, amount } => {
                    match dev_actions::raise_maintenance_capacity(&mut world, alliance, amount) {
                        Ok(detail) => dev_channel.note(&detail),
                        Err(why) => dev_channel.refuse(&why),
                    }
                }
                DevCommand::Quit => {
                    dev_channel.note("quitting");
                    quit_requested = true;
                }
                DevCommand::Status => {
                    let mode = format!("{game_mode:?}");
                    let mode = mode
                        .split(|c: char| !c.is_alphanumeric())
                        .next()
                        .unwrap_or_default()
                        .to_owned();
                    let index = message_index.as_ref().map_or_else(
                        || "closed".to_owned(),
                        |index| {
                            let name = dev_commands::MESSAGE_CATEGORIES
                                .iter()
                                .find(|(_, command)| *command == index.category())
                                .map_or("?", |(name, _)| name);
                            format!("{name}/{:?}", index.mode())
                        },
                    );
                    // Each side's top strip: raw, refined, and maintenance
                    // as capacity − use (`top-bar-resource-counters.md`).
                    let strip = |side: Faction| {
                        use rebellion_core::resources::{maintenance_capacity, maintenance_used};
                        let stock = stockpile_state.side(side);
                        format!(
                            "{}/{}/{}-{}",
                            stock.raw,
                            stock.refined,
                            maintenance_capacity(&world, side),
                            maintenance_used(&world, &mfg_state, side),
                        )
                    };
                    let status = format!(
                        "mode={mode} day={} speed={:?} pause_alert={} battle_alert={} \
                         message_index={index} briefing={} side={:?} \
                         alliance={} empire={} fast_forward={}",
                        clock.tick,
                        clock.speed,
                        clock.pause_requested(),
                        tactical_state.battle_alert_open(),
                        advisor_state.holds_clock(),
                        player_faction,
                        strip(Faction::Alliance),
                        strip(Faction::Empire),
                        if dev_policy.fast_forward { "on" } else { "off" },
                    );
                    eprintln!("[dev-command] status: {status}");
                    dev_channel.note(&status);
                }
                DevCommand::ListSystems => {
                    let listing = dev_actions::list_systems(&world);
                    eprintln!("[dev-command] systems: {listing}");
                    dev_channel.note(&listing);
                }
                DevCommand::ListAdviceTopics => {
                    let held = advice_topics.held_topics();
                    dev_channel.note(&format!("held: {held:?}"));
                }
                DevCommand::ListFleets(side) => dev_channel.note(
                    &rebellion_data::readouts::list_fleets(&world, &movement_state, side),
                ),
                DevCommand::ListCharacters(side) => {
                    dev_channel.note(&rebellion_data::readouts::list_characters(
                        &world,
                        &mission_state,
                        &movement_state,
                        side,
                    ))
                }
                DevCommand::ListMissions(side) => {
                    dev_channel.note(&rebellion_data::readouts::list_missions(
                        &world,
                        &mission_state,
                        clock.tick,
                        side,
                    ))
                }
                DevCommand::ShowResearch => dev_channel.note(
                    &rebellion_data::readouts::show_research(&world, &research_state),
                ),
                DevCommand::ListProduction(side) => {
                    dev_channel.note(&rebellion_data::readouts::list_production(
                        &world, &mfg_state, clock.tick, side,
                    ))
                }
                DevCommand::ShowSide(side) => {
                    dev_channel.note(&rebellion_data::readouts::show_side(
                        &world,
                        &stockpile_state,
                        &mfg_state,
                        &mission_state,
                        side,
                    ))
                }
                DevCommand::ShowSystem(ref name) => {
                    match rebellion_data::readouts::show_system(&world, name) {
                        Ok(text) => dev_channel.note(&text),
                        Err(why) => dev_channel.refuse(&why),
                    }
                }
                DevCommand::Save(name) => {
                    if game_mode != GameMode::Galaxy {
                        dev_channel.refuse(&format!(
                            "the galaxy must show to save; the game is in {game_mode:?}"
                        ));
                    } else if let Some(modal) = dev_modal {
                        dev_channel.refuse(&format!("{modal} is open; close it first"));
                    } else {
                        dev_save_file = Some(dev_actions::save_path(dev_channel.evidence(), &name));
                        panel_actions.push(PanelAction::SaveGame { slot: 0, name });
                    }
                }
                DevCommand::Load(name) => {
                    let path = dev_actions::save_path(dev_channel.evidence(), &name);
                    if !matches!(game_mode, GameMode::MainMenu | GameMode::Galaxy) {
                        dev_channel.refuse(&format!(
                            "a save loads from the main menu or the galaxy; the game is in {game_mode:?}"
                        ));
                    } else if let Some(modal) = dev_modal {
                        dev_channel.refuse(&format!("{modal} is open; close it first"));
                    } else if !path.is_file() {
                        dev_channel.refuse(&format!("no save at {}", path.display()));
                    } else {
                        dev_load_file = Some(path);
                        panel_actions.push(PanelAction::LoadGame { slot: 0 });
                    }
                }
                DevCommand::Mission {
                    team,
                    kind,
                    system,
                    ship,
                    decoys,
                } => {
                    match dev_actions::mission_order(
                        &world,
                        &uprising_state,
                        &mission_state,
                        player_faction,
                        &team,
                        &decoys,
                        kind,
                        &system,
                        ship,
                        clock.tick,
                    ) {
                        Ok(order) => {
                            // The result is written once the dispatch has run
                            // (below, after the panel actions).
                            if let Ok(mut last) = dev_actions::LAST_DISPATCH_REFUSAL.lock() {
                                *last = None;
                            }
                            dev_mission_check = Some(mission_state.len());
                            panel_actions.push(PanelAction::DispatchMission {
                                kind: order.kind,
                                faction: player_faction,
                                team: order.team,
                                decoys: order.decoys,
                                target: order.target,
                                target_character: None,
                                target_object: order.object,
                                tick: clock.tick,
                            });
                        }
                        Err(why) => dev_channel.refuse(&why),
                    }
                }
                DevCommand::Recruit {
                    character,
                    side,
                    system,
                } => match dev_actions::recruit(&mut world, &character, side, system.as_deref()) {
                    Ok(detail) => dev_channel.note(&detail),
                    Err(why) => dev_channel.refuse(&why),
                },
                DevCommand::Jail(character) => {
                    match dev_actions::jail(&mut world, &character, clock.tick) {
                        Ok(detail) => dev_channel.note(&detail),
                        Err(why) => dev_channel.refuse(&why),
                    }
                }
                DevCommand::SkipBriefing => {
                    // The skip goes in, and the line answers, in its wait
                    // (dev_policy::skip_briefing).
                    if advisor_state.holds_clock() {
                        dev_wait_budget = None;
                        dev_wait_until = Some((dev_commands::WaitCondition::BriefingSkipped, 0));
                    } else {
                        dev_channel.refuse("no briefing is running");
                    }
                }
                DevCommand::AdvisorPostCode(code) => match u8::try_from(code) {
                    Ok(code) => advisor_state.post_code(code),
                    Err(_) => dev_channel.refuse("advice codes run 0..=0xff"),
                },
                DevCommand::ReleaseAdviceTopic(topic) => {
                    match advice_topics
                        .release_topic(topic, advisor_state.now(), |id| text_templates.get(id))
                    {
                        Ok(message) => {
                            dev_channel.note(&format!("filed {}", message.display.title));
                            file_advice_topics(
                                &mut msg_log,
                                vec![message],
                                clock.tick,
                                cockpit_state.faction == CockpitFaction::Alliance,
                            );
                        }
                        Err(why) => dev_channel.refuse(&why),
                    }
                }
            }
        }
        // A start runs from the main menu as its faction control does;
        // everything else waits for the galaxy.
        #[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
        let mut scripted_menu_action = None;
        #[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
        if !matches!(game_mode, GameMode::Galaxy) {
            for command in std::mem::take(&mut pending_interface) {
                match command {
                    rebellion_render::InterfaceCommand::StartGame(faction)
                        if matches!(game_mode, GameMode::MainMenu) =>
                    {
                        scripted_menu_action = main_menu_state.activate_control(
                            if faction == MissionFaction::Alliance {
                                rebellion_render::MainMenuControl::Alliance
                            } else {
                                rebellion_render::MainMenuControl::Empire
                            },
                        );
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    _ => dev_channel.refuse("the galaxy is not showing"),
                    #[cfg(target_arch = "wasm32")]
                    _ => eprintln!("[dev-command] refused: the galaxy is not showing"),
                }
            }
        }

        // ── Movies (FUN_00422ce0, message 0x482) ─────────────────────────────
        // A message movie, then the ending, once the galaxy has no modal
        // window (FUN_00601340): a waiting movie first closes the windows
        // that close on 0x40d, and waits for the player to close any other.
        if game_mode == GameMode::Galaxy {
            let closing = movies::clear_for_movie(
                &movie_queue,
                &mut status_window_state,
                &mut encyclopedia_surface,
                &mut message_index,
            );
            if !closing.is_empty() {
                apply_message_index_actions(
                    closing,
                    &mut msg_log,
                    &mut clock,
                    &mut advice_saved_speed,
                    &mut pending_message_sound,
                );
            }
        }
        if let Some(showing) =
            movie_queue.next(game_mode == GameMode::Galaxy && movie_window_free)
        {
            let (movie, kind) = match showing {
                movies::Showing::Message(message) => {
                    (message.movie, CutsceneKind::Message(message))
                }
                movies::Showing::Ending(movie) => {
                    // FUN_00414330(0x10-0x13) tears the game down before
                    // the movie; there is no way back to it.
                    show_fleets = false;
                    show_missions = false;
                    macroquad::logging::info!(
                        "[main_menu] campaign_ended generation={} movie={:#x}",
                        campaign_generation,
                        movie
                    );
                    game_mode = GameMode::MainMenu;
                    (movie, CutsceneKind::Application(movie))
                }
            };
            let bounds = movies::video_bounds(
                matches!(kind, CutsceneKind::Message(_)),
                egui_macroquad::egui::Rect::from_min_size(
                    egui_macroquad::egui::Pos2::ZERO,
                    egui_macroquad::egui::vec2(screen_width(), screen_height()),
                ),
            );
            cutscene_player = movies::path(movie).and_then(|path| {
                open_cutscene(
                    Path::new(&path),
                    audio_vol.cutscene_volume(),
                    bounds,
                    #[cfg(not(target_arch = "wasm32"))]
                    &mut audio_engine,
                )
            });
            caption_without_movie =
                cutscene_player.is_none() && matches!(kind, CutsceneKind::Message(_));
            if cutscene_player.is_some() || caption_without_movie {
                game_mode = GameMode::Cutscene { kind };
            }
        }

        // Freeze the production route only after simulation and scripted
        // commands have had their chance to introduce a modal this frame.
        let desired_window_scene = match game_mode {
            GameMode::Galaxy => WindowScene::Galaxy,
            GameMode::TacticalCombat => WindowScene::Tactical,
            GameMode::Cutscene { .. }
            | GameMode::MainMenu
            | GameMode::MultiplayerSetup
            | GameMode::GameSetup
            | GameMode::GameOptions { .. } => WindowScene::Suspended,
        };
        if strategic_windowing.scene() != desired_window_scene {
            for event in strategic_windowing.enter_scene(desired_window_scene) {
                if let rebellion_windowing::WindowEvent::CaptureCancelled { window, .. } = event {
                    strategic_input_router.cancel_window(window);
                    fleet_window_state.cancel_pointer_capture(window);
                }
            }
        }
        #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
        let original_modal_fixture_open = interface_fixture_request.is_some_and(|request| {
            matches!(
                request.scenario,
                interface_test_fixture::Scenario::EncyclopediaArtwork
                    | interface_test_fixture::Scenario::MessageIndexShell
                    | interface_test_fixture::Scenario::OperationReport
                    | interface_test_fixture::Scenario::EncyclopediaIndexShell
                    | interface_test_fixture::Scenario::EncyclopediaIndexCatalog
            )
        }) || interface_fixture_request
            .is_some_and(|request| request.scenario.is_encyclopedia_surface());
        #[cfg(not(all(target_arch = "wasm32", feature = "interface-test-fixtures")))]
        let original_modal_fixture_open = false;
        #[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
        let command_palette_open = command_palette_state.open;
        #[cfg(not(any(debug_assertions, not(target_arch = "wasm32"))))]
        let command_palette_open = false;
        let strategic_locks = ApplicationInputLocks {
            briefing: !interface_fixture_active && briefing_lock,
            fixture_setup: false,
            scene_transition: desired_window_scene != WindowScene::Galaxy,
        };
        let legacy_blockers = LegacyInputBlockers {
            strategic_modal: build_selection_state.is_open()
                || mission_dialog_state.is_open()
                || move_confirmation_state.is_open()
                || scrap_confirmation_state.is_open()
                || stop_confirmation_state.is_open()
                || status_window_state.is_open()
                || fleet_finder_state.is_open()
                || troop_finder_state.is_open()
                || personnel_finder_state.is_open()
                || operation_report_state.is_open()
                || message_index.is_some()
                || encyclopedia_surface.is_open()
                || original_modal_fixture_open,
            // A band's name edit holds the keyboard (FUN_00458640's
            // SetFocus); its Escape closes its window itself.
            focused_control: command_palette_open || system_window_state.band_rename_open(),
            // The GID menu is none: it takes no capture and its procedure
            // (FUN_004aab50) passes all but Enter and Escape to the view.
            transient_menu: object_menu.is_some()
                || game_speed_ui.menu_anchor.is_some()
                || agent_menu_state.anchor.is_some(),
            root_gesture: targeting.is_some(),
        };
        let strategic_frame_boundary =
            strategic_windowing.begin_production_frame(strategic_locks, legacy_blockers);
        if game_mode == GameMode::Galaxy {
            let children = galaxy_children(
                &sector_window_state,
                &system_window_state,
                &defenses_window_state,
                &missions_window_state,
                &fleet_window_state,
                strategic_frame_boundary.snapshot(),
            );
            for step in galaxy_focus.sync(children) {
                apply_focus_step(
                    step,
                    &mut cockpit_state.gid_ui,
                    &mut sector_window_state,
                    &mut system_window_state,
                    &mut defenses_window_state,
                    &mut missions_window_state,
                    &mut |command| strategic_windowing.enqueue(command),
                );
            }
        }
        let fleet_keyboard_owner =
            fleet_window_state.keyboard_owner(strategic_frame_boundary.snapshot());
        let mut legacy_restriction_latch = LegacyInputRestrictionLatch::new(legacy_blockers);
        movie_window_free =
            !legacy_blockers.strategic_modal && !strategic_frame_boundary.migrated_modal_active();
        let strategic_frame_sequence_valid =
            if let Some(next_frame) = strategic_frame_number.checked_add(1) {
                strategic_frame_number = next_frame;
                strategic_input_router
                    .begin_frame(strategic_frame_number, strategic_frame_boundary.snapshot());
                strategic_input_router
                    .set_application_hold(strategic_frame_boundary.migrated_input_held());
                if strategic_frame_boundary.migrated_input_held()
                    && fleet_window_state.drag_owner().is_some()
                {
                    strategic_windowing.enqueue(rebellion_windowing::WindowCommand::CancelCapture(
                        rebellion_windowing::CaptureCancelReason::HoldStarted,
                    ));
                }
                // miniquad 0.4.8 passes focus loss to no input subscriber:
                // the browser shell counts it, and on macOS each frame
                // samples AppKit's active state. port: other native hosts
                // report none.
                #[cfg(target_arch = "wasm32")]
                let focus_losses = Some(browser_focus_losses());
                #[cfg(target_os = "macos")]
                let focus_losses = {
                    let active = native_app_active();
                    if native_active_logged != Some(active) {
                        macroquad::logging::info!("[windowing] app_active={}", active);
                        native_active_logged = Some(active);
                    }
                    Some(native_focus.sample(active))
                };
                #[cfg(not(any(target_arch = "wasm32", target_os = "macos")))]
                let focus_losses: Option<u32> = None;
                if focus_losses.is_some_and(|count| focus_loss_watch.lost(count)) {
                    macroquad::logging::info!(
                        "[windowing] focus_lost fleet_drag_owner={:?}",
                        fleet_window_state.drag_owner()
                    );
                    strategic_windowing.enqueue(rebellion_windowing::WindowCommand::CancelCapture(
                        rebellion_windowing::CaptureCancelReason::FocusLost,
                    ));
                }
                true
            } else {
                macroquad::logging::error!("[windowing] frame_sequence_exhausted");
                strategic_input_router.set_application_hold(true);
                false
            };

        // ── Rendering (mode-specific) ────────────────────────────────────────

        // The Battle Alert and the results are a window over the live
        // command center (`FUN_0049c0d0`; capture 22-battle-alert-result
        // shows the counters, day and sector windows behind it): draw the
        // command center with its input held, then the window over it.
        let battle_window_underlay =
            game_mode == GameMode::TacticalCombat && tactical_state.over_command_center();
        let tactical_frame = game_mode == GameMode::TacticalCombat;
        let render_mode = if battle_window_underlay {
            GameMode::Galaxy
        } else {
            game_mode.clone()
        };
        match render_mode {
            GameMode::Cutscene { ref kind } => {
                clear_background(BLACK);

                let mut ended = true;
                if let Some(player) = cutscene_player.as_mut() {
                    player.advance(dt);
                    // A resize moves the picture's place on the screen.
                    player.place(movies::video_bounds(
                        matches!(kind, CutsceneKind::Message(_)),
                        egui_macroquad::egui::Rect::from_min_size(
                            egui_macroquad::egui::Pos2::ZERO,
                            egui_macroquad::egui::vec2(screen_width(), screen_height()),
                        ),
                    ));
                    if let CutsceneKind::Message(message) = kind {
                        draw_message_movie(player.current_frame(), &message.caption);
                    } else if let Some(frame) = player.current_frame() {
                        draw_fullscreen_texture(frame);
                    }
                    ended = player.is_finished();
                } else if let (true, CutsceneKind::Message(message)) = (caption_without_movie, kind)
                {
                    draw_message_movie(None, &message.caption);
                    ended = false;
                }
                if ended {
                    // FUN_004138a0 0x405: the next state's movie, or the
                    // screen after it.
                    cutscene_player = None;
                    let side = if player_faction == MissionFaction::Alliance {
                        Faction::Alliance
                    } else {
                        Faction::Empire
                    };
                    game_mode = kind.ended(side, |next| {
                        cutscene_player = open_movie(
                            next,
                            &audio_vol,
                            #[cfg(not(target_arch = "wasm32"))]
                            &mut audio_engine,
                        );
                        cutscene_player.is_some()
                    });
                    macroquad::logging::info!(
                        "[movie] ended id={:#x} next={:?}",
                        kind.movie(),
                        game_mode
                    );
                }
            }

            GameMode::MainMenu => {
                sector_window_state.clear();
                system_window_state.clear();
                fleet_window_state.clear();
                defenses_window_state.clear();
                missions_window_state.clear();
                galaxy_focus = GalaxyFocusState::default();
                object_menu = None;
                targeting = None;

                #[cfg(target_arch = "wasm32")]
                if let Some(focus) = web_accessibility::take_focus_update() {
                    main_menu_state.set_semantic_focus(focus);
                }
                #[cfg(target_arch = "wasm32")]
                let semantic_action = web_accessibility::take_activation()
                    .and_then(|control| main_menu_state.activate_control(control));
                #[cfg(target_arch = "wasm32")]
                let semantic_interaction = web_accessibility::take_user_interaction();

                clear_background(Color::new(0.02, 0.02, 0.06, 1.0));
                #[cfg(target_arch = "wasm32")]
                let mut menu_action = semantic_action;
                #[cfg(not(target_arch = "wasm32"))]
                let mut menu_action = None;
                egui_macroquad::ui(|ctx| {
                    let canvas_action = draw_main_menu(
                        ctx,
                        &mut bmp_cache,
                        &mut main_menu_state,
                        audio_vol.music_enabled(),
                    );
                    if menu_action.is_none() {
                        menu_action = canvas_action;
                    }
                });
                egui_macroquad::draw();
                #[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
                if menu_action.is_none() {
                    menu_action = scripted_menu_action.take();
                }

                if let Some(sfx) = main_menu_state.take_sfx() {
                    #[cfg(not(target_arch = "wasm32"))]
                    audio_engine.play_sfx(sfx, &audio_vol);
                    #[cfg(target_arch = "wasm32")]
                    if let Some(engine) = browser_menu_audio.as_mut() {
                        engine.play_sfx(sfx, &audio_vol);
                    }
                    let resource_id = audio::MENU_SFX_ASSETS
                        .iter()
                        .find_map(|&(kind, _, resource_id)| (kind == sfx).then_some(resource_id))
                        .unwrap_or_default();
                    macroquad::logging::info!(
                        "[audio] menu_sfx={:?} resource={}",
                        sfx,
                        resource_id
                    );
                }

                #[cfg(target_arch = "wasm32")]
                if !browser_menu_audio_requested
                    && (is_mouse_button_pressed(macroquad::input::MouseButton::Left)
                        || is_key_pressed(KeyCode::Enter)
                        || is_key_pressed(KeyCode::Space)
                        || is_key_pressed(KeyCode::Tab)
                        || semantic_interaction)
                {
                    browser_menu_audio_requested = true;
                    if browser_menu_audio.is_none() {
                        eprintln!(
                            "[audio] menu music unavailable: runtime pack has no MDATA.300 cue"
                        );
                    }
                }
                #[cfg(target_arch = "wasm32")]
                if browser_menu_audio_requested {
                    if let Some(engine) = browser_menu_audio.as_mut() {
                        engine.try_play_loaded_music(&audio_vol);
                    }
                }

                if let Some(action) = menu_action {
                    match action {
                        MainMenuAction::StartGame {
                            difficulty,
                            faction,
                            galaxy_size,
                            headquarters_only,
                        } => {
                            // The original faction controls start immediately.
                            // Reuse the established campaign initialization path
                            // without displaying the replacement setup page.
                            game_setup_state.difficulty = difficulty;
                            game_setup_state.faction = Some(faction);
                            game_setup_state.galaxy_size = galaxy_size;
                            pending_victory_conditions = if headquarters_only {
                                VictoryConditions::HeadquartersOnly
                            } else {
                                VictoryConditions::Standard
                            };
                            pending_cockpit_start = Some(GameSetupAction::StartGame {
                                difficulty,
                                faction,
                                galaxy_size,
                            });
                            game_mode = GameMode::GameSetup;
                        }
                        MainMenuAction::LoadGame => {
                            save_slots = read_save_slots(&saves_dir);
                            game_options_state.set_origin(GameOptionsOrigin::ShuttleCockpit);
                            game_options_state.refresh_saves(&save_slots);
                            game_mode = GameMode::GameOptions {
                                origin: GameOptionsOrigin::ShuttleCockpit,
                            };
                            macroquad::logging::info!(
                                "[interface] command=load_options destination=game_options status=opened_original"
                            );
                        }
                        MainMenuAction::Credits => {
                            // The LucasArts logo plays the Credits movie
                            // over the menu (FUN_00406000 0x73 →
                            // FUN_004019a0(0x6e, 0, FUN_00406520)).
                            cutscene_player = open_movie(
                                movies::CREDITS,
                                &audio_vol,
                                #[cfg(not(target_arch = "wasm32"))]
                                &mut audio_engine,
                            );
                            if cutscene_player.is_some() {
                                game_mode = GameMode::Cutscene {
                                    kind: CutsceneKind::Application(movies::CREDITS),
                                };
                            }
                            macroquad::logging::info!(
                                "[main_menu] destination=credits movie={:#x} opened={}",
                                movies::CREDITS,
                                cutscene_player.is_some()
                            );
                        }
                        MainMenuAction::Multiplayer => {
                            multiplayer_setup_state.status_message = None;
                            game_mode = GameMode::MultiplayerSetup;
                            macroquad::logging::info!("[main_menu] destination=multiplayer_setup");
                        }
                        MainMenuAction::ToggleMusic => {
                            audio_vol.toggle_music();
                            macroquad::logging::info!(
                                "[audio] menu_music_enabled={}",
                                audio_vol.music_enabled()
                            );
                        }
                        MainMenuAction::Quit => quit_requested = true,
                    }
                }
            }

            GameMode::MultiplayerSetup => {
                clear_background(Color::new(0.02, 0.02, 0.06, 1.0));
                let mut multiplayer_action = None;
                egui_macroquad::ui(|ctx| {
                    multiplayer_action = draw_multiplayer_setup(ctx, &mut multiplayer_setup_state);
                });
                egui_macroquad::draw();
                match multiplayer_action {
                    Some(MultiplayerSetupAction::Back) => {
                        game_mode = GameMode::MainMenu;
                    }
                    Some(MultiplayerSetupAction::StartRequested) => {
                        let message = multiplayer_setup_state.unavailable_message();
                        multiplayer_setup_state.status_message = Some(message.clone());
                        macroquad::logging::info!(
                            "[multiplayer] status=unavailable transport={:?} player={}",
                            multiplayer_setup_state.transport,
                            multiplayer_setup_state.player_name
                        );
                    }
                    None => {}
                }
            }

            GameMode::GameOptions { origin } => {
                let mut action = draw_game_options(
                    &mut game_options_state,
                    &mut bmp_cache,
                    &save_slots,
                    &mut audio_vol,
                );
                egui_macroquad::ui(|ctx| {
                    let overlay = rebellion_render::game_options::draw_game_options_overlay(
                        ctx,
                        &mut bmp_cache,
                        &mut game_options_state,
                    );
                    if overlay != GameOptionsAction::None {
                        action = overlay;
                    }
                });
                egui_macroquad::draw();
                match action {
                    GameOptionsAction::None => {}
                    GameOptionsAction::Return => {
                        game_mode = match origin {
                            GameOptionsOrigin::ShuttleCockpit => GameMode::MainMenu,
                            GameOptionsOrigin::CommandCenter => GameMode::Galaxy,
                            GameOptionsOrigin::TacticalBattle => GameMode::TacticalCombat,
                        };
                        macroquad::logging::info!(
                            "[game_options] command=return origin={:?}",
                            origin
                        );
                    }
                    GameOptionsAction::Restart => {
                        game_mode = GameMode::MainMenu;
                        macroquad::logging::info!("[game_options] command=restart");
                    }
                    GameOptionsAction::Exit => quit_requested = true,
                    GameOptionsAction::Save { slot, name } => {
                        panel_actions.push(PanelAction::SaveGame { slot, name });
                    }
                    GameOptionsAction::Load { slot } => {
                        panel_actions.push(PanelAction::LoadGame { slot });
                    }
                }
            }

            GameMode::GameSetup => {
                let mut setup_action = pending_cockpit_start.take();
                if setup_action.is_none() {
                    clear_background(Color::new(0.02, 0.02, 0.06, 1.0));
                    egui_macroquad::ui(|ctx| {
                        setup_action = draw_game_setup(ctx, &mut game_setup_state);
                    });
                    egui_macroquad::draw();
                }

                if let Some(action) = setup_action {
                    match action {
                        GameSetupAction::StartGame {
                            difficulty,
                            faction,
                            galaxy_size,
                        } => {
                            // Convert setup choices to SeedOptions and reload world.
                            let dat_faction_for_seed = match faction {
                                MissionFaction::Alliance => Faction::Alliance,
                                MissionFaction::Empire => Faction::Empire,
                            };
                            let seed_difficulty = match difficulty {
                                rebellion_render::Difficulty::Easy => SeedDifficulty::Easy,
                                rebellion_render::Difficulty::Medium => SeedDifficulty::Medium,
                                rebellion_render::Difficulty::Hard => SeedDifficulty::Hard,
                            };
                            let seed_options = SeedOptions {
                                galaxy_size,
                                difficulty: seed_difficulty,
                                player_faction: dat_faction_for_seed,
                                // Fresh random seed each game, unless
                                // OPEN_REBELLION_SEED fixes it.
                                #[cfg(not(target_arch = "wasm32"))]
                                rng_seed: dev_commands::campaign_seed(),
                                #[cfg(target_arch = "wasm32")]
                                rng_seed: None,
                            };
                            let seeded = rebellion_data::load_game_data_with_options(
                                &gdata_path,
                                &seed_options,
                            )
                            .map(|mut w| {
                                for error in mod_runtime.apply_enabled(&mut w) {
                                    macroquad::logging::error!(
                                        "[campaign] mod_reapply_error={:?}",
                                        error
                                    );
                                }
                                let hq = headquarters(&w);
                                (w, hq)
                            });
                            let campaign_loaded = match seeded {
                                Ok((mut w, (alliance_hq, empire_hq))) => {
                                    player_faction = faction;
                                    campaign_config = CampaignConfig::from_seed_options(
                                        seed_options,
                                        pending_victory_conditions,
                                    );
                                    pending_victory_conditions = VictoryConditions::Standard;
                                    // The Fleet Registry's choice (a port extension).
                                    w.start_fleet_naming(main_menu_state.fleet_naming);
                                    macroquad::logging::info!(
                                        "[campaign] fleet_naming={:?}",
                                        main_menu_state.fleet_naming
                                    );
                                    world = w;
                                    campaign_generation += 1;
                                    sim_rng = Xoshiro256PlusPlus::seed_from_u64(
                                        rng_seed.wrapping_add(u64::from(campaign_generation)),
                                    );
                                    clock = GameClock::new();
                                    galaxy_focus = GalaxyFocusState::default();
                                    game_speed_ui = GameSpeedUiState::default();
                                    mfg_state = ManufacturingState::new();
                                    mission_state = MissionState::new();
                                    event_state = EventState::new();
                                    rebellion_core::story_events::define_story_events(
                                        &mut event_state,
                                        &world,
                                    );
                                    movement_state = MovementState::new();
                                    rebellion_core::death_star::refresh_sectors(&mut world);
                                    blockade_state = BlockadeState::new();
                                    uprising_state = UprisingState::new();
                                    research_state = ResearchState::new();
                                    jedi_state = JediState::new();
                                    betrayal_state = BetrayalState::new();
                                    repair_state = RepairState::default();
                                    troop_transport_state = TroopTransportState::default();
                                    delivery_state = DeliveryState::new();
                                    stockpile_state = StockpileState::new();
                                    economy_state = EconomyState::default();
                                    player_agent =
                                        PlayerAgent::for_new_game(campaign_config.difficulty);
                                    game_config = rebellion_core::tuning::GameConfig::default();
                                    dual_ai_mode = false;
                                    secondary_ai_state = None;
                                    held_battles.clear();

                                    map_state = GalaxyMapState::default();
                                    warmed_galaxy_font_sizes.clear();
                                    msg_log = MessageLog::default();
                                    log_state = MessageLogState::default();
                                    fleets_state = FleetsState::default();
                                    build_selection_state = BuildSelectionState::default();
                                    mission_dialog_state = MissionDialogState::default();
                                    move_confirmation_state = MoveConfirmationState::default();
                                    status_window_state = StatusWindowState::default();
                                    fleet_finder_state = FleetFinderState::default();
                                    troop_finder_state = TroopFinderState::default();
                                    personnel_finder_state = PersonnelFinderState::default();
                                    operation_report_state = OperationReportState::default();
                                    message_index = None;
                                    advice_saved_speed = None;
                                    encyclopedia_surface.close();
                                    show_fleets = false;
                                    show_missions = false;
                                    save_slots = read_save_slots(&saves_dir);
                                    tactical_state = TacticalState::new();

                                    victory_state = VictoryState::new(alliance_hq, empire_hq);
                                    true
                                }
                                // port: the original always has both headquarters and
                                // shows nothing here; its menu has no message for a
                                // refused start, so the refusal goes to the log (the
                                // terminal natively, the console in the browser) and
                                // the menu stays.
                                Err(e) => {
                                    macroquad::logging::error!(
                                        "[campaign] new game refused, the menu stays: {:#}",
                                        e
                                    );
                                    false
                                }
                            };

                            if campaign_loaded {
                                drop(strategic_windowing.reset_session());
                                strategic_input_router.reset_session();
                                // Sync cockpit chrome to player faction
                                cockpit_state =
                                    CockpitState::new(if faction == MissionFaction::Alliance {
                                        CockpitFaction::Alliance
                                    } else {
                                        CockpitFaction::Empire
                                    });
                                sector_window_state.clear();
                                system_window_state.clear();
                                fleet_window_state.clear();
                                defenses_window_state.clear();
                                missions_window_state.clear();
                                galaxy_focus = GalaxyFocusState::default();
                                object_menu = None;
                                targeting = None;

                                // Initialize game state for chosen faction
                                fog_alliance_state = FogState::new(Faction::Alliance);
                                fog_empire_state = FogState::new(Faction::Empire);
                                FogSystem::seed(&mut fog_alliance_state, &world);
                                FogSystem::seed(&mut fog_empire_state, &world);
                                economy_state = EconomyState::default();
                                player_agent =
                                    PlayerAgent::for_new_game(campaign_config.difficulty);
                                held_battles.clear();

                                // AI controls the opposite faction
                                if faction == MissionFaction::Empire {
                                    ai_state = AIState::new(AiFaction::Alliance);
                                } else {
                                    ai_state = AIState::new(AiFaction::Empire);
                                }

                                let faction_name = if faction == MissionFaction::Alliance {
                                    "Rebel Alliance"
                                } else {
                                    "Galactic Empire"
                                };
                                let campaign_summary = campaign_config.summary();
                                macroquad::logging::info!(
                                    "[campaign] faction={} configuration={}",
                                    faction_name,
                                    campaign_summary
                                );
                                macroquad::logging::info!(
                                "[campaign] reset generation={} tick={} missions={} movements={} dual_ai={} second_ai={} event_definitions={}",
                                campaign_generation,
                                clock.tick,
                                mission_state.len(),
                                movement_state.len(),
                                dual_ai_mode,
                                secondary_ai_state.is_some(),
                                event_state.events().len()
                            );
                                msg_log.push(GameMessage::new(
                                    clock.tick,
                                    format!("You command the {faction_name} — {campaign_summary}."),
                                    MessageCategory::Event,
                                ));

                                // Sync advisor faction and send greeting
                                advisor_state =
                                    AdvisorState::new(AdvisorFaction::from(cockpit_state.faction));
                                advice_topics = rebellion_render::advice_topics::AdviceTopics::new(
                                    AdvisorFaction::from(cockpit_state.faction),
                                    |id| text_templates.get(id),
                                );
                                let sprite_dir = gdata_path.join("ui");
                                advisor_state.set_sprite_dir(&sprite_dir);
                                movie_queue = movies::MovieQueue::default();
                                // FUN_00439320: a game's advice agent starts
                                // the music counter at 0; the Play scene
                                // follows the side's movie, so it keeps it.
                                #[cfg(not(target_arch = "wasm32"))]
                                music_selector.reset();

                                // State 7 (FUN_00414830): with the session
                                // built (FUN_00401b90), the side's movie
                                // plays; its end enters state 8, the command
                                // center (FUN_00414eb0).
                                let movie = movies::side_movie(dat_faction_for_seed);
                                cutscene_player = open_movie(
                                    movie,
                                    &audio_vol,
                                    #[cfg(not(target_arch = "wasm32"))]
                                    &mut audio_engine,
                                );
                                game_mode = if cutscene_player.is_some() {
                                    GameMode::Cutscene {
                                        kind: CutsceneKind::Application(movie),
                                    }
                                } else {
                                    GameMode::Galaxy
                                };
                            } else {
                                game_mode = GameMode::MainMenu;
                            }
                        }
                        GameSetupAction::Back => {
                            game_mode = GameMode::MainMenu;
                        }
                    }
                }
            }

            GameMode::Galaxy => {
                #[cfg(not(target_arch = "wasm32"))]
                if encyclopedia_hd_generation != encyclopedia_session_store.texture_generation() {
                    encyclopedia_hd = prepare_encyclopedia_hd_snapshot(
                        asset_render_profile,
                        &gdata_path,
                        &encyclopedia_session_store,
                    );
                    encyclopedia_hd_generation = encyclopedia_session_store.texture_generation();
                }
                prewarm_galaxy_font_sizes(&world, &map_state, &mut warmed_galaxy_font_sizes);
                let fog_state = if player_faction == MissionFaction::Alliance {
                    &fog_alliance_state
                } else {
                    &fog_empire_state
                };
                // 1. Prepare the original 640×480 command-center canvas and
                // recover the faction-specific galaxy aperture.
                // The authentic bitmap frame is drawn in the single egui pass
                // below so input is consumed exactly once per game frame.
                let cockpit_layout = cockpit_state.layout();
                let cockpit_vp = cockpit_layout.galaxy;

                // Pass cockpit viewport to galaxy map for mouse input clamping.
                map_state.viewport = Some((
                    cockpit_vp.x,
                    cockpit_vp.y,
                    cockpit_vp.width,
                    cockpit_vp.height,
                ));
                map_state.display_scale = cockpit_layout.scale;
                let pointer = mouse_position();
                let strategic_root_pointer_enabled = strategic_frame_sequence_valid
                    && strategic_frame_boundary.legacy_pointer_allowed();
                let strategic_owner_pointer_enabled = strategic_frame_sequence_valid
                    && strategic_frame_boundary.legacy_owned_pointer_allowed()
                    && strategic_frame_boundary.legacy_surface_input_allowed();
                // Windows the GID menu hides take no press: it reaches the
                // galaxy view (FUN_00422ce0 0x202).
                let shown_child = |child: GalaxyChild| !cockpit_state.gid_ui.hides(child);
                let galaxy_child_under_pointer = [
                    sector_window_state.contains_shown_screen_point(
                        cockpit_layout,
                        pointer,
                        &shown_child,
                    ),
                    system_window_state.contains_shown_screen_point(
                        cockpit_layout,
                        pointer,
                        &shown_child,
                    ),
                    defenses_window_state.contains_shown_screen_point(
                        cockpit_layout,
                        pointer,
                        &shown_child,
                    ),
                    missions_window_state.contains_shown_screen_point(
                        cockpit_layout,
                        pointer,
                        &shown_child,
                    ),
                    !cockpit_state.gid_ui.windows_hidden
                        && fleet_window_state.contains_visible_screen_point(
                            strategic_frame_boundary.snapshot(),
                            cockpit_layout,
                            pointer,
                        ),
                ]
                .contains(&true);
                map_state.pointer_blocked = galaxy_child_under_pointer
                    || cockpit_state.gid_legend_contains(cockpit_layout, pointer)
                    || fleet_window_state.is_dragging()
                    || defenses_window_state.is_dragging()
                    || mission_dialog_state.contains_screen_point(cockpit_layout, pointer)
                    || build_selection_state.contains_screen_point(cockpit_layout, pointer)
                    || move_confirmation_state.contains_screen_point(cockpit_layout, pointer)
                    || scrap_confirmation_state.contains_screen_point(cockpit_layout, pointer)
                    || stop_confirmation_state.contains_screen_point(cockpit_layout, pointer)
                    || status_window_state.contains_screen_point(cockpit_layout, pointer)
                    || fleet_finder_state.contains_screen_point(cockpit_layout, pointer)
                    || troop_finder_state.contains_screen_point(cockpit_layout, pointer)
                    || personnel_finder_state.contains_screen_point(cockpit_layout, pointer)
                    || operation_report_state.contains_screen_point(cockpit_layout, pointer)
                    || message_index.is_some()
                        && rebellion_render::message_index::window_rect(cockpit_layout)
                            .contains(egui_macroquad::egui::pos2(pointer.0, pointer.1))
                    || encyclopedia_surface.is_open()
                    || original_modal_fixture_open
                    || game_speed_ui.menu_anchor.is_some()
                    || object_menu.is_some()
                    || pause_alert_contains_screen_point(&clock, cockpit_layout, pointer)
                    || briefing_lock
                    || battle_window_underlay
                    || !strategic_root_pointer_enabled;
                map_state.targeting = targeting.is_some();

                // The Message Index rail lights each category with unread
                // messages for the player's side (FUN_0048a2a0).
                cockpit_state.message_unread_mask =
                    msg_log.unread_mask(player_faction == MissionFaction::Alliance);

                // 4. All egui panels in a single ui() + draw() pass
                let mut pending_cockpit_button = None;
                #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
                let windowing_map_selection_before = map_state.selected_system;
                #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
                let mut pending_windowing_route_receipts = Vec::new();
                let strategic_pointer_events = if strategic_frame_sequence_valid {
                    // Hit testing follows the layers egui itself routes this
                    // frame's pointer through, so a retained window painted
                    // over a Fleet window keeps the event from it.
                    let mut batch = StrategicPointerBatch::default();
                    egui_macroquad::cfg(|ctx| {
                        batch = StrategicPointerBatch::collect_from_raw(
                            strategic_frame_number,
                            &mut strategic_input_router,
                            &strategic_raw_input_events.pointers,
                            |point| {
                                fleet_window_state.hit_window_ids(
                                    strategic_frame_boundary.snapshot(),
                                    cockpit_layout,
                                    point,
                                    ctx.layer_id_at(egui_macroquad::egui::pos2(point.0, point.1)),
                                )
                            },
                        );
                    });
                    batch
                } else {
                    StrategicPointerBatch::default()
                };
                let strategic_pointer_routes = strategic_pointer_events.router_events();
                let strategic_root_keys = if strategic_frame_sequence_valid {
                    StrategicRootKeyBatch::collect_from_indexed_raw(
                        strategic_frame_number,
                        &mut strategic_input_router,
                        &strategic_raw_input_events.keys,
                    )
                } else {
                    StrategicRootKeyBatch::default()
                };
                let mut egui_claims_pointer = false;
                let mut egui_claims_keyboard = false;
                let mut legacy_keyboard_handled = false;
                egui_macroquad::ui(|ctx| {
                    let retained_legacy_input = strategic_frame_boundary
                        .migrated_modal_active()
                        .then(|| suspend_legacy_egui_input(ctx));
                    let strategic_input_enabled = strategic_root_pointer_enabled;
                    // The open GID menu hides the galaxy's child windows
                    // (FUN_00422ce0 0x132); its close shows them again.
                    let gid_ui = cockpit_state.gid_ui.clone();
                    let shown = |child: GalaxyChild| !gid_ui.hides(child);
                    let strategic_owner_input_enabled = strategic_owner_pointer_enabled;
                    // The popup menu and the target release keep the pointer
                    // under a focused window.
                    let pointer_owner_input_enabled = strategic_frame_sequence_valid
                        && strategic_frame_boundary.legacy_owned_pointer_allowed();
                    let retained_window_input =
                        strategic_frame_boundary.retained_window_input_allowed();
                    // Register the cockpit background before panels so the
                    // opaque chrome never covers their content or artwork.
                    draw_cockpit_background(ctx, &cockpit_state, &mut bmp_cache);
                    // Paint the native primary controls before floating
                    // windows. Input resolves after those windows register.
                    if let Some(mode) = draw_cockpit_egui_layer(
                        ctx,
                        &mut cockpit_state,
                        &mut bmp_cache,
                        strategic_input_enabled,
                        strategic_owner_input_enabled,
                    ) {
                        #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
                        if let Some(event) = strategic_pointer_events.latest_primary_release() {
                            pending_windowing_route_receipts.push((event, "pointer", "gid"));
                        }
                        macroquad::logging::info!(
                            "[interface] command=0x{:x} destination=gid status=selected label={}",
                            mode.command_id(),
                            mode.label()
                        );
                        #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
                        if let Some(request) = interface_fixture_request {
                            interface_test_fixture::emit_selected(request, cockpit_state.gid_mode);
                        }
                    }

                    rebellion_render::draw_gid_caption(
                        ctx,
                        cockpit_layout,
                        cockpit_state.faction,
                        cockpit_state.gid_mode,
                        &cockpit_state.gid_highlight,
                    );
                    rebellion_render::sector_hover::draw_sector_hover_label(
                        ctx,
                        &world,
                        cockpit_layout,
                        cockpit_state.faction,
                    );
                    // The day readout is the Game Speed control; a right
                    // click on it opens the original speed menu. The
                    // briefing leaves it blank until its step 11; the
                    // resource counters show throughout (A0 Wine capture).
                    if !cockpit_state.briefing {
                        draw_day_readout(
                            ctx,
                            &mut bmp_cache,
                            cockpit_layout,
                            cockpit_state.faction,
                            clock.tick,
                            clock.speed,
                        );
                    }
                    // Raw, refined and maintenance: side `+0x78`, `+0x7c`
                    // and `+0x58 − +0x74` (`FUN_00422620`).
                    let side = match cockpit_state.faction {
                        rebellion_render::cockpit::CockpitFaction::Alliance => Faction::Alliance,
                        rebellion_render::cockpit::CockpitFaction::Empire => Faction::Empire,
                    };
                    rebellion_render::game_speed::draw_resource_counters(
                        ctx,
                        cockpit_layout,
                        cockpit_state.faction,
                        [
                            Some(stockpile_state.side(side).raw),
                            Some(stockpile_state.side(side).refined),
                            Some(rebellion_core::resources::maintenance_surplus(
                                &world, &mfg_state, side,
                            )),
                        ],
                    );
                    let speed_menu_was_open = game_speed_ui.menu_anchor.is_some();
                    if strategic_input_enabled {
                        open_game_speed_menu_on_right_click(
                            ctx,
                            &mut game_speed_ui,
                            cockpit_layout,
                            cockpit_state.faction,
                        );
                    }
                    if !speed_menu_was_open && game_speed_ui.menu_anchor.is_some() {
                        legacy_restriction_latch.note_start();
                    }
                    rebellion_render::tooltip::draw_cockpit_tooltips(
                        ctx,
                        &mut cockpit_tooltips,
                        cockpit_layout,
                        cockpit_state.faction,
                    );
                    if let Some(speed) = draw_game_speed_menu(
                        ctx,
                        &mut game_speed_ui,
                        &mut bmp_cache,
                        cockpit_layout,
                        cockpit_state.faction,
                        strategic_owner_input_enabled,
                    ) {
                        choose_game_speed(&mut clock, speed);
                    }
                    // A right click on the agent's droid opens the Agent
                    // menu (list 0xdead, FUN_00487900).
                    let agent_menu_was_open = agent_menu_state.anchor.is_some();
                    if strategic_input_enabled {
                        rebellion_render::agent_menu::open_agent_menu_on_right_click(
                            ctx,
                            &mut agent_menu_state,
                            cockpit_layout,
                            cockpit_state.faction,
                        );
                    }
                    if !agent_menu_was_open && agent_menu_state.anchor.is_some() {
                        legacy_restriction_latch.note_start();
                    }
                    let agent_view = rebellion_render::agent_menu::AgentMenuView {
                        garrisons: player_agent.is_running(AutomationModule::Garrisons),
                        production: player_agent.is_running(AutomationModule::Production),
                        advice: player_agent.advice(),
                    };
                    if let Some(command) = rebellion_render::agent_menu::draw_agent_menu(
                        ctx,
                        &mut agent_menu_state,
                        &mut bmp_cache,
                        cockpit_layout,
                        cockpit_state.faction,
                        agent_view,
                        strategic_owner_input_enabled,
                    ) {
                        apply_agent_command(&mut player_agent, command, "menu");
                    }
                    // A rail light's click (0x136..0x13e) or F6 (0x75) opens
                    // the Message Index on its category; FUN_0042a240 does
                    // nothing while one is open.
                    let index_command = pending_index_command.take().or_else(|| {
                        strategic_input_enabled
                            .then(|| {
                                let (released, pointer) = ctx.input(|input| {
                                    (
                                        input.pointer.primary_released(),
                                        input.pointer.interact_pos(),
                                    )
                                });
                                let pointer =
                                    pointer.filter(|_| released && !ctx.is_pointer_over_area())?;
                                rebellion_render::cockpit::message_index_command_at(
                                    cockpit_state.faction,
                                    (
                                        (pointer.x - cockpit_layout.canvas.x)
                                            / cockpit_layout.scale,
                                        (pointer.y - cockpit_layout.canvas.y)
                                            / cockpit_layout.scale,
                                    ),
                                )
                            })
                            .flatten()
                    });
                    if let Some(command) = index_command {
                        if message_index.is_none() {
                            if let Some(category) =
                                rebellion_render::message_index::opening_category(command)
                            {
                                let mut index = MessageIndexState::new();
                                let actions = index.set_category(category);
                                message_index = Some(index);
                                legacy_restriction_latch.note_start();
                                apply_message_index_actions(
                                    actions,
                                    &mut msg_log,
                                    &mut clock,
                                    &mut advice_saved_speed,
                                    &mut pending_message_sound,
                                );
                                macroquad::logging::info!(
                                    "[interface] command=0x{:x} destination=message_index category=0x{:x} status=opened_original",
                                    command,
                                    category
                                );
                            }
                        }
                    }
                    // Display (0x67) goes to the message's object through
                    // FUN_00429440, as the Finders' choices below do.
                    if message_index.is_some()
                        && strategic_frame_boundary.legacy_surface_input_allowed()
                        && strategic_root_keys.contains(KeyCode::Escape)
                    {
                        legacy_keyboard_handled = true;
                        let closing = message_index.take().map(|index| index.close());
                        apply_message_index_actions(
                            closing.into_iter().flatten().collect(),
                            &mut msg_log,
                            &mut clock,
                            &mut advice_saved_speed,
                            &mut pending_message_sound,
                        );
                    }
                    let mut message_choice = None;
                    let mut report_from_index = None;
                    if let Some(index) = message_index.as_mut() {
                        let actions = rebellion_render::message_index::draw_message_index(
                            ctx,
                            &mut bmp_cache,
                            cockpit_state.faction,
                            rebellion_render::message_index::window_rect(cockpit_layout).min,
                            cockpit_layout.scale,
                            &msg_log,
                            index,
                            player_faction == MissionFaction::Alliance,
                            &|id| encyclopedia_picture(encyclopedia_session_store.current().as_deref(), id),
                        );
                        message_choice = actions.iter().find_map(|action| match action {
                            MessageIndexAction::GoTo(MessageTarget::System(system)) => {
                                Some((*system, FinderTarget::Sector))
                            }
                            // A fleet in hyperspace opens at its destination
                            // (FUN_00556390), as the Fleet Finder's choice.
                            MessageIndexAction::GoTo(MessageTarget::Fleet(fleet)) => {
                                rebellion_core::movement::listed_location(
                                    &movement_state,
                                    &world,
                                    *fleet,
                                )
                                .map(|system| (system, FinderTarget::Fleet(*fleet)))
                            }
                            MessageIndexAction::GoTo(MessageTarget::Personnel(member)) => {
                                personnel_window(&world, &movement_state, *member)
                            }
                            _ => None,
                        });
                        // FUN_00469de0: a report shown from the list opens
                        // its window, which reopens the Index on its
                        // category when it closes (FUN_00469d60).
                        report_from_index = actions.iter().find_map(|action| match action {
                            MessageIndexAction::OpenReport(id) => Some((*id, index.category())),
                            _ => None,
                        });
                        if actions.contains(&MessageIndexAction::Close) {
                            let closing = message_index.take().map(|index| index.close());
                            apply_message_index_actions(
                                actions
                                    .into_iter()
                                    .chain(closing.into_iter().flatten())
                                    .collect(),
                                &mut msg_log,
                                &mut clock,
                                &mut advice_saved_speed,
                                &mut pending_message_sound,
                            );
                        } else {
                            apply_message_index_actions(
                                actions,
                                &mut msg_log,
                                &mut clock,
                                &mut advice_saved_speed,
                                &mut pending_message_sound,
                            );
                        }
                    }
                    if let Some((id, category)) = report_from_index {
                        if let Some(message) = msg_log.get(id) {
                            operation_report_state.open(
                                &world,
                                message,
                                cockpit_state.faction,
                                Some(category),
                            );
                        }
                    }
                    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
                    if let Some(request) = interface_fixture_request {
                        speed_menu_watch.observe(
                            request,
                            game_speed_ui.menu_anchor.and_then(|_| {
                                rebellion_render::game_speed::game_speed_menu_rect(ctx)
                            }),
                        );
                    }
                    if draw_pause_alert(
                        ctx,
                        &clock,
                        &mut bmp_cache,
                        cockpit_layout,
                        strategic_input_enabled,
                    ) {
                        clock.resume();
                    }

                    // War Room panels (mutually exclusive left panels)
                    if show_fleets {
                        if let Some(action) = draw_fleets(
                            ctx,
                            &world,
                            &movement_state,
                            &troop_transport_state,
                            &mut fleets_state,
                            player_faction,
                            &mut bmp_cache,
                        ) {
                            panel_actions.push(action);
                        }
                    }
                    if show_missions {
                        if let Some(action) =
                            draw_missions(ctx, &world, &mission_state, player_faction, clock.tick)
                        {
                            panel_actions.push(action);
                        }
                    }

                    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
                    if interface_fixture_request.is_some_and(|request| {
                        request.scenario == interface_test_fixture::Scenario::EncyclopediaArtwork
                    }) {
                        rebellion_render::draw_encyclopedia_artwork_fixture(
                            ctx,
                            42,
                            &mut enc_state,
                        );
                    }
                    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
                    if let Some(request) = interface_fixture_request.filter(|request| {
                        request.scenario == interface_test_fixture::Scenario::MessageIndexShell
                    }) {
                        rebellion_render::draw_message_index_fixture(
                            ctx,
                            &mut bmp_cache,
                            request.faction,
                        );
                    }
                    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
                    if let Some(request) = interface_fixture_request.filter(|request| {
                        request.scenario == interface_test_fixture::Scenario::OperationReport
                    }) {
                        let state = fixture_operation_report.get_or_insert_with(|| {
                            interface_test_fixture::operation_report(
                                &world,
                                &text_templates,
                                request.faction,
                            )
                        });
                        let _ = rebellion_render::draw_operation_report_fixture(
                            ctx,
                            state,
                            &world,
                            &movement_state,
                            &mut bmp_cache,
                            &text_templates
                                .format(rebellion_render::operation_report::DISPLAY_TEXT_ID, &[])
                                .unwrap_or_default(),
                        );
                        operation_report_watch.observe(request, state);
                    }
                    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
                    if let Some(request) = interface_fixture_request.filter(|request| {
                        request.scenario == interface_test_fixture::Scenario::EncyclopediaIndexShell
                    }) {
                        if let Some(command) = rebellion_render::draw_encyclopedia_index_fixture(
                            ctx,
                            &mut bmp_cache,
                            request.faction,
                            enc_state.original_category_command,
                        ) {
                            if (0x6f..=0x75).contains(&command) {
                                enc_state.original_category_command = command;
                            }
                        }
                    }
                    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
                    if let (Some(request), Some(catalog)) = (
                        interface_fixture_request.filter(|request| {
                            request.scenario
                                == interface_test_fixture::Scenario::EncyclopediaIndexCatalog
                        }),
                        original_encyclopedia_catalog.as_ref(),
                    ) {
                        rebellion_render::draw_encyclopedia_index_catalog_fixture(
                            ctx,
                            &mut bmp_cache,
                            request.faction,
                            &mut enc_state,
                            catalog,
                        );
                    }
                    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
                    if let Some(fixture) = encyclopedia_surface_fixture.as_mut() {
                        fixture.draw(
                            ctx,
                            &mut bmp_cache,
                            interface_fixture_request
                                .expect("Encyclopedia fixture has an interface request")
                                .code,
                        );
                    }

                    // Mod Manager (floating window)
                    let mod_infos: Vec<rebellion_render::ModInfo> = mod_runtime
                        .discovered
                        .iter()
                        .map(|m| {
                            let err = mod_runtime.errors.iter().find(|e| e.mod_name() == m.name);
                            rebellion_render::ModInfo {
                                name: m.name.clone(),
                                version: m.version.clone(),
                                author: m.author.clone(),
                                description: m.description.clone(),
                                enabled: m.enabled,
                                dependencies: m.dependencies.keys().cloned().collect(),
                                has_error: err.is_some(),
                                error_message: err.map(ToString::to_string),
                            }
                        })
                        .collect();
                    let mod_global_errors: Vec<String> = mod_runtime
                        .errors
                        .iter()
                        .filter(|error| error.mod_name().is_empty())
                        .map(ToString::to_string)
                        .collect();
                    let mod_actions = rebellion_render::draw_mod_manager(
                        ctx,
                        &mod_infos,
                        &mod_global_errors,
                        &mut mod_manager_state,
                    );
                    for action in mod_actions {
                        match action {
                            rebellion_render::ModManagerAction::ToggleMod(name) => {
                                panel_actions.push(PanelAction::ToggleMod { name });
                            }
                            rebellion_render::ModManagerAction::ReloadMods => {
                                panel_actions.push(PanelAction::ReloadMods);
                            }
                        }
                    }

                    // Developer command palette: its commands take the
                    // paths the clicks they replace would.
                    #[cfg(not(any(debug_assertions, not(target_arch = "wasm32"))))]
                    let commanded_sector_actions = Vec::new();
                    #[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
                    let commanded_sector_actions = {
                        let mut commanded = Vec::new();
                        if palette_enabled {
                            for action in rebellion_render::draw_command_palette(
                                ctx,
                                &mut command_palette_state,
                            ) {
                                match action {
                                    rebellion_render::PaletteAction::Panel(action) => {
                                        panel_actions.push(action);
                                    }
                                    rebellion_render::PaletteAction::Interface(command) => {
                                        pending_interface.push(command);
                                    }
                                    // A developer line joins the script's
                                    // queue, so it runs as a script line does.
                                    #[cfg(not(target_arch = "wasm32"))]
                                    rebellion_render::PaletteAction::Script(line) => {
                                        command_script
                                            .get_or_insert_with(Default::default)
                                            .push_line(line);
                                    }
                                }
                            }
                        }
                        for command in std::mem::take(&mut pending_interface) {
                            match sector_window_state.command_actions(
                                &world,
                                fog_state,
                                &mission_state,
                                &movement_state,
                                cockpit_state.faction,
                                cockpit_layout,
                                command,
                            ) {
                                Ok(actions) => commanded.extend(actions),
                                #[cfg(not(target_arch = "wasm32"))]
                                Err(why) => dev_channel.refuse(why),
                                #[cfg(target_arch = "wasm32")]
                                Err(why) => eprintln!("[dev-command] refused: {why}"),
                            }
                        }
                        commanded
                    };

                    for action in with_retained_input(ctx, retained_window_input, || {
                        draw_sector_windows(
                            ctx,
                            &world,
                            &movement_state,
                            fog_state,
                            &mut sector_window_state,
                            cockpit_state.faction,
                            cockpit_layout,
                            &mut bmp_cache,
                            &mission_state,
                            &uprising_state,
                            &shown,
                        )
                    })
                    .into_iter()
                    .chain(commanded_sector_actions)
                    {
                        match action {
                            SectorWindowAction::SelectSystem(system) => {
                                map_state.selected_system = Some(system);
                            }
                            SectorWindowAction::OpenSystemWindow {
                                system,
                                logical_position,
                            } => {
                                map_state.selected_system = Some(system);
                                system_window_state.open(
                                    &world,
                                    system,
                                    logical_position,
                                    cockpit_state.faction,
                                    cockpit_layout,
                                );
                            }
                            SectorWindowAction::OpenFleetWindow {
                                system,
                                logical_position,
                            } => {
                                map_state.selected_system = Some(system);
                                enqueue_fleet_window(
                                    &mut strategic_windowing,
                                    &fleet_window_state,
                                    &world,
                                    system,
                                    logical_position,
                                    cockpit_state.faction,
                                    cockpit_layout,
                                    None,
                                );
                            }
                            SectorWindowAction::OpenDefensesWindow {
                                system,
                                logical_position,
                            } => {
                                map_state.selected_system = Some(system);
                                defenses_window_state.open(
                                    &world,
                                    system,
                                    logical_position,
                                    cockpit_state.faction,
                                    cockpit_layout,
                                );
                            }
                            SectorWindowAction::OpenMissionsWindow {
                                system,
                                logical_position,
                            } => {
                                map_state.selected_system = Some(system);
                                missions_window_state.open(
                                    &world,
                                    fog_state,
                                    &mission_state,
                                    system,
                                    logical_position,
                                    cockpit_state.faction,
                                    cockpit_layout,
                                );
                            }
                            SectorWindowAction::OpenObjectMenu { selection, point } => {
                                let gates = order_gates(
                                    selection,
                                    &world,
                                    &mission_state,
                                    &movement_state,
                                    &troop_transport_state,
                                    &blockade_state,
                                    &mfg_state,
                                    &research_state,
                                    player_faction,
                                );
                                object_menu = Some(ObjectMenuState::new(selection, gates, point));
                                legacy_restriction_latch.note_start();
                            }
                            // FUN_00422ce0, window type 1: the selection moves
                            // against what +0x70 gives under the point, kind
                            // 4 entries apart (0x214 against +0x68). port: only
                            // the fleet icon's drop moves; its fleets are the
                            // system's of the player's side (FUN_00512700).
                            SectorWindowAction::DropSelection {
                                selection:
                                    MenuObject::SystemIcon {
                                        system,
                                        quadrant: Quadrant::Fleets,
                                    },
                                point,
                                confirmed,
                            } => {
                                let windows = ReleaseWindows {
                                    sector: &sector_window_state,
                                    system: &system_window_state,
                                    fleet: &fleet_window_state,
                                    defenses: &defenses_window_state,
                                    missions: &missions_window_state,
                                };
                                if let Some(target) = release_destination(
                                    ctx,
                                    &world,
                                    &movement_state,
                                    fog_state,
                                    cockpit_layout,
                                    windows,
                                    point,
                                ) {
                                    let fleets = system_side_fleets(
                                        &world,
                                        system,
                                        player_faction == MissionFaction::Alliance,
                                    );
                                    if issue_fleet_move(
                                        &FleetMoveContext {
                                            world: &world,
                                            movement: &movement_state,
                                            blockaded: blockade_state.blockaded_systems(),
                                            faction: player_faction,
                                        },
                                        &fleets,
                                        confirmed,
                                        target,
                                        &mut move_confirmation_state,
                                        &mut panel_actions,
                                    ) {
                                        legacy_restriction_latch.note_start();
                                    }
                                }
                            }
                            SectorWindowAction::DropSelection { .. } => {}
                        }
                    }

                    // One focus across the galaxy's child windows (window_focus): the
                    // kind that took focus last paints in front.
                    let galaxy_front = galaxy_front(
                        &system_window_state,
                        {
                            let serial = fleet_legacy_bridge_serial;
                            #[cfg(feature = "interface-test-fixtures")]
                            let serial = serial.max(fleet_window_state.fixture_focus_serial());
                            serial
                        },
                        &defenses_window_state,
                        &missions_window_state,
                    );
                    for action in with_retained_input(ctx, retained_window_input, || {
                        draw_system_windows(
                            ctx,
                            &world,
                            &movement_state,
                            fog_state,
                            &mission_state,
                            &mfg_state,
                            &delivery_state,
                            &mut system_window_state,
                            cockpit_state.faction,
                            cockpit_layout,
                            &mut bmp_cache,
                            galaxy_front == GalaxyFront::System,
                            &shown,
                        )
                    }) {
                        match action {
                            SystemWindowAction::FocusSector(system) => {
                                sector_window_state.open_for_system(
                                    &world,
                                    system,
                                    cockpit_state.faction,
                                );
                                map_state.selected_system = Some(system);
                            }
                            SystemWindowAction::SelectSystem(system) => {
                                map_state.selected_system = Some(system);
                            }
                            SystemWindowAction::EvictFleetWindow(window) => strategic_windowing
                                .enqueue(rebellion_windowing::WindowCommand::Close(window)),
                            SystemWindowAction::RestoreFleetWindow {
                                id,
                                system: _,
                                logical_position: _,
                            } => strategic_windowing.enqueue_fleet_restore(id),
                            SystemWindowAction::RestoreDefensesWindow {
                                system,
                                logical_position,
                            } => {
                                defenses_window_state.open(
                                    &world,
                                    system,
                                    logical_position,
                                    cockpit_state.faction,
                                    cockpit_layout,
                                );
                            }
                            SystemWindowAction::RestoreMissionsWindow {
                                system,
                                logical_position,
                            } => {
                                missions_window_state.open(
                                    &world,
                                    fog_state,
                                    &mission_state,
                                    system,
                                    logical_position,
                                    cockpit_state.faction,
                                    cockpit_layout,
                                );
                            }
                            // FUN_00458780 submits the Rename (0x215), which
                            // names the band's current product.
                            SystemWindowAction::RenameProduct { system, area, name } => {
                                let target = band_rename_target(
                                    &world,
                                    &mfg_state,
                                    system,
                                    area,
                                    player_faction == MissionFaction::Alliance,
                                );
                                macroquad::logging::info!(
                                    "[interface] command=0x215 destination=band_rename status={} area={:?} name={}",
                                    if target.is_some() { "submitted" } else { "refused" },
                                    area,
                                    name
                                );
                                if let Some((fleet, index)) = target {
                                    panel_actions.push(PanelAction::Rename {
                                        fleet,
                                        ship: Some(index),
                                        name,
                                    });
                                }
                            }
                            SystemWindowAction::OpenObjectMenu {
                                selection, point, ..
                            } => {
                                let gates = order_gates(
                                    selection,
                                    &world,
                                    &mission_state,
                                    &movement_state,
                                    &troop_transport_state,
                                    &blockade_state,
                                    &mfg_state,
                                    &research_state,
                                    player_faction,
                                );
                                object_menu = Some(ObjectMenuState::new(selection, gates, point));
                                legacy_restriction_latch.note_start();
                            }
                        }
                    }

                    let fleet_actions = draw_managed_fleet_windows(
                        ctx,
                        &world,
                        &movement_state,
                        fog_state,
                        &troop_transport_state,
                        &mut fleet_window_state,
                        cockpit_state.faction,
                        cockpit_layout,
                        &mut bmp_cache,
                        strategic_frame_boundary.snapshot(),
                        &strategic_pointer_routes,
                        &mut strategic_input_router,
                        galaxy_front == GalaxyFront::Fleet,
                        &shown,
                    );
                    for action in fleet_actions {
                        match action {
                            FleetWindowAction::OpenSector(system) => {
                                sector_window_state.open_for_system(
                                    &world,
                                    system,
                                    cockpit_state.faction,
                                );
                                map_state.selected_system = Some(system);
                            }
                            FleetWindowAction::SelectSystem(system) => {
                                map_state.selected_system = Some(system);
                            }
                            FleetWindowAction::Activate(window) => strategic_windowing
                                .enqueue(rebellion_windowing::WindowCommand::Activate(window)),
                            FleetWindowAction::CapturePointer(window) => strategic_windowing
                                .enqueue(rebellion_windowing::WindowCommand::CapturePointer(
                                    window,
                                )),
                            FleetWindowAction::ReleasePointer(window) => strategic_windowing
                                .enqueue(rebellion_windowing::WindowCommand::ReleasePointer(
                                    window,
                                )),
                            FleetWindowAction::Close(window) => strategic_windowing
                                .enqueue(rebellion_windowing::WindowCommand::Close(window)),
                            FleetWindowAction::Minimize {
                                system,
                                logical_position,
                            } => {
                                if let Some(window) = fleet_window_state.window_id(system) {
                                    strategic_windowing.enqueue_fleet_minimize(
                                        window,
                                        system,
                                        logical_position,
                                    );
                                }
                            }
                            FleetWindowAction::MinimizeManaged {
                                window,
                                system,
                                logical_position,
                            } => strategic_windowing.enqueue_fleet_minimize(
                                window,
                                system,
                                logical_position,
                            ),
                            // FUN_00422ce0: a drop from the Fleet window
                            // (type 4) moves the selection against the
                            // window under the point with 0x201.
                            FleetWindowAction::Rename { entry, name } => {
                                panel_actions.push(PanelAction::Rename {
                                    fleet: entry.fleet(),
                                    ship: match entry {
                                        rebellion_render::fleet_window::FleetWindowEntry::Ship { index, .. } => Some(index),
                                        rebellion_render::fleet_window::FleetWindowEntry::Fleet(_) => None,
                                    },
                                    name,
                                });
                            }
                            FleetWindowAction::OpenObjectMenu { selection, point } => {
                                let gates = order_gates(
                                    Some(selection),
                                    &world,
                                    &mission_state,
                                    &movement_state,
                                    &troop_transport_state,
                                    &blockade_state,
                                    &mfg_state,
                                    &research_state,
                                    player_faction,
                                );
                                object_menu =
                                    Some(ObjectMenuState::new(Some(selection), gates, point));
                                legacy_restriction_latch.note_start();
                            }
                            // FUN_00422ce0, window type 4: the fleet moves
                            // with 0x201, or 0x202 with Ctrl, against what
                            // +0x70 gives under the point.
                            FleetWindowAction::DragFleet {
                                fleet,
                                confirmed,
                                point,
                            } => {
                                let windows = ReleaseWindows {
                                    sector: &sector_window_state,
                                    system: &system_window_state,
                                    fleet: &fleet_window_state,
                                    defenses: &defenses_window_state,
                                    missions: &missions_window_state,
                                };
                                if let Some(target) = release_destination(
                                    ctx,
                                    &world,
                                    &movement_state,
                                    fog_state,
                                    cockpit_layout,
                                    windows,
                                    point,
                                ) {
                                    if issue_fleet_move(
                                        &FleetMoveContext {
                                            world: &world,
                                            movement: &movement_state,
                                            blockaded: blockade_state.blockaded_systems(),
                                            faction: player_faction,
                                        },
                                        &[fleet],
                                        confirmed,
                                        target,
                                        &mut move_confirmation_state,
                                        &mut panel_actions,
                                    ) {
                                        legacy_restriction_latch.note_start();
                                    }
                                }
                            }
                            // FUN_00422ce0: the ship moves with 0x201, or
                            // 0x202 with Ctrl, against the window under the
                            // point (ghidra/notes/fleet-join-split.md).
                            FleetWindowAction::DragShip {
                                fleet,
                                index,
                                roster,
                                confirmed,
                                point,
                            } => {
                                let windows = ReleaseWindows {
                                    sector: &sector_window_state,
                                    system: &system_window_state,
                                    fleet: &fleet_window_state,
                                    defenses: &defenses_window_state,
                                    missions: &missions_window_state,
                                };
                                if let Some(target) = release_destination(
                                    ctx,
                                    &world,
                                    &movement_state,
                                    fog_state,
                                    cockpit_layout,
                                    windows,
                                    point,
                                ) {
                                    if issue_ship_move(
                                        &FleetMoveContext {
                                            world: &world,
                                            movement: &movement_state,
                                            blockaded: blockade_state.blockaded_systems(),
                                            faction: player_faction,
                                        },
                                        fleet,
                                        vec![index],
                                        roster,
                                        confirmed,
                                        target,
                                        &mut move_confirmation_state,
                                        &mut panel_actions,
                                    ) {
                                        legacy_restriction_latch.note_start();
                                    }
                                }
                            }
                            // FUN_00422ce0: the drop issues 0x201 (0x202 with
                            // Ctrl) against the window under the point, and
                            // FUN_0053c4b0 makes it a 0x241 Move mission.
                            FleetWindowAction::DragCharacter {
                                character,
                                confirmed,
                                point,
                            } => {
                                let windows = ReleaseWindows {
                                    sector: &sector_window_state,
                                    system: &system_window_state,
                                    fleet: &fleet_window_state,
                                    defenses: &defenses_window_state,
                                    missions: &missions_window_state,
                                };
                                let Some(target) = release_destination(
                                    ctx,
                                    &world,
                                    &movement_state,
                                    fog_state,
                                    cockpit_layout,
                                    windows,
                                    point,
                                ) else {
                                    continue;
                                };
                                issue_member_move(
                                    &FleetMoveContext {
                                        world: &world,
                                        movement: &movement_state,
                                        blockaded: blockade_state.blockaded_systems(),
                                        faction: player_faction,
                                    },
                                    &mission_state,
                                    vec![MissionMember::Character(character)],
                                    confirmed,
                                    target,
                                    &mut advisor_state,
                                    &mut move_confirmation_state,
                                    &mut panel_actions,
                                );
                            }
                            FleetWindowAction::DragRegiment {
                                troop,
                                confirmed,
                                point,
                            } => {
                                let windows = ReleaseWindows {
                                    sector: &sector_window_state,
                                    system: &system_window_state,
                                    fleet: &fleet_window_state,
                                    defenses: &defenses_window_state,
                                    missions: &missions_window_state,
                                };
                                let Some(target) = release_destination(
                                    ctx,
                                    &world,
                                    &movement_state,
                                    fog_state,
                                    cockpit_layout,
                                    windows,
                                    point,
                                ) else {
                                    continue;
                                };
                                if issue_unit_move(
                                    &FleetMoveContext {
                                        world: &world,
                                        movement: &movement_state,
                                        blockaded: blockade_state.blockaded_systems(),
                                        faction: player_faction,
                                    },
                                    &troop_transport_state,
                                    &ProductRoom::new(&mfg_state, &delivery_state),
                                    UnitMover::Regiment(troop),
                                    confirmed,
                                    target,
                                    &mut move_confirmation_state,
                                    &mut panel_actions,
                                ) {
                                    legacy_restriction_latch.note_start();
                                }
                            }
                            FleetWindowAction::DragSquadron {
                                fleet,
                                index,
                                confirmed,
                                point,
                            } => {
                                let windows = ReleaseWindows {
                                    sector: &sector_window_state,
                                    system: &system_window_state,
                                    fleet: &fleet_window_state,
                                    defenses: &defenses_window_state,
                                    missions: &missions_window_state,
                                };
                                let Some(target) = release_destination(
                                    ctx,
                                    &world,
                                    &movement_state,
                                    fog_state,
                                    cockpit_layout,
                                    windows,
                                    point,
                                ) else {
                                    continue;
                                };
                                if issue_unit_move(
                                    &FleetMoveContext {
                                        world: &world,
                                        movement: &movement_state,
                                        blockaded: blockade_state.blockaded_systems(),
                                        faction: player_faction,
                                    },
                                    &troop_transport_state,
                                    &ProductRoom::new(&mfg_state, &delivery_state),
                                    UnitMover::Squadron(SquadronSource::Fleet { fleet, index }),
                                    confirmed,
                                    target,
                                    &mut move_confirmation_state,
                                    &mut panel_actions,
                                ) {
                                    legacy_restriction_latch.note_start();
                                }
                            }
                        }
                    }

                    for action in with_retained_input(ctx, retained_window_input, || {
                        draw_defenses_windows(
                            ctx,
                            &world,
                            fog_state,
                            &mission_state,
                            &mut defenses_window_state,
                            cockpit_state.faction,
                            cockpit_layout,
                            &mut bmp_cache,
                            galaxy_front == GalaxyFront::Defenses,
                            &shown,
                        )
                    }) {
                        match action {
                            DefensesWindowAction::OpenSector(system) => {
                                sector_window_state.open_for_system(
                                    &world,
                                    system,
                                    cockpit_state.faction,
                                );
                                map_state.selected_system = Some(system);
                            }
                            DefensesWindowAction::SelectSystem(system) => {
                                map_state.selected_system = Some(system);
                            }
                            DefensesWindowAction::Minimize {
                                system,
                                logical_position,
                                serial,
                            } => {
                                if let Some(window) = system_window_state
                                    .minimize_defenses_window_at(system, logical_position, serial)
                                {
                                    strategic_windowing
                                        .enqueue(rebellion_windowing::WindowCommand::Close(window));
                                }
                            }
                            DefensesWindowAction::OpenObjectMenu {
                                selection, point, ..
                            } => {
                                let gates = order_gates(
                                    selection,
                                    &world,
                                    &mission_state,
                                    &movement_state,
                                    &troop_transport_state,
                                    &blockade_state,
                                    &mfg_state,
                                    &research_state,
                                    player_faction,
                                );
                                object_menu = Some(ObjectMenuState::new(selection, gates, point));
                                legacy_restriction_latch.note_start();
                            }
                            // FUN_00422ce0: the drop issues 0x201 (0x202 with
                            // Ctrl) against the window under the point, and
                            // FUN_0053c4b0 makes it a 0x241 Move mission.
                            DefensesWindowAction::DragMember {
                                member,
                                confirmed,
                                point,
                            } => {
                                let windows = ReleaseWindows {
                                    sector: &sector_window_state,
                                    system: &system_window_state,
                                    fleet: &fleet_window_state,
                                    defenses: &defenses_window_state,
                                    missions: &missions_window_state,
                                };
                                let Some(target) = release_destination(
                                    ctx,
                                    &world,
                                    &movement_state,
                                    fog_state,
                                    cockpit_layout,
                                    windows,
                                    point,
                                ) else {
                                    continue;
                                };
                                issue_member_move(
                                    &FleetMoveContext {
                                        world: &world,
                                        movement: &movement_state,
                                        blockaded: blockade_state.blockaded_systems(),
                                        faction: player_faction,
                                    },
                                    &mission_state,
                                    vec![member],
                                    confirmed,
                                    target,
                                    &mut advisor_state,
                                    &mut move_confirmation_state,
                                    &mut panel_actions,
                                );
                            }
                            DefensesWindowAction::DragSquadron {
                                system,
                                class,
                                is_alliance,
                                confirmed,
                                point,
                            } => {
                                let windows = ReleaseWindows {
                                    sector: &sector_window_state,
                                    system: &system_window_state,
                                    fleet: &fleet_window_state,
                                    defenses: &defenses_window_state,
                                    missions: &missions_window_state,
                                };
                                let Some(target) = release_destination(
                                    ctx,
                                    &world,
                                    &movement_state,
                                    fog_state,
                                    cockpit_layout,
                                    windows,
                                    point,
                                ) else {
                                    continue;
                                };
                                if issue_unit_move(
                                    &FleetMoveContext {
                                        world: &world,
                                        movement: &movement_state,
                                        blockaded: blockade_state.blockaded_systems(),
                                        faction: player_faction,
                                    },
                                    &troop_transport_state,
                                    &ProductRoom::new(&mfg_state, &delivery_state),
                                    UnitMover::Squadron(SquadronSource::System {
                                        system,
                                        class,
                                        is_alliance,
                                    }),
                                    confirmed,
                                    target,
                                    &mut move_confirmation_state,
                                    &mut panel_actions,
                                ) {
                                    legacy_restriction_latch.note_start();
                                }
                            }
                        }
                    }

                    for action in with_retained_input(ctx, retained_window_input, || {
                        draw_missions_windows(
                            ctx,
                            &world,
                            fog_state,
                            &mission_state,
                            &mut missions_window_state,
                            cockpit_state.faction,
                            cockpit_layout,
                            &mut bmp_cache,
                            galaxy_front == GalaxyFront::Missions,
                            &shown,
                        )
                    }) {
                        match action {
                            MissionsWindowAction::OpenSector(system) => {
                                sector_window_state.open_for_system(
                                    &world,
                                    system,
                                    cockpit_state.faction,
                                );
                                map_state.selected_system = Some(system);
                            }
                            MissionsWindowAction::SelectSystem(system) => {
                                map_state.selected_system = Some(system);
                            }
                            MissionsWindowAction::Minimize {
                                system,
                                logical_position,
                                serial,
                            } => {
                                if let Some(window) = system_window_state
                                    .minimize_missions_window_at(system, logical_position, serial)
                                {
                                    strategic_windowing
                                        .enqueue(rebellion_windowing::WindowCommand::Close(window));
                                }
                            }
                        }
                    }

                    // The menu commits on the release that starts targeting;
                    // only a later release may end the order.
                    let targeting_held = targeting.is_some();
                    match draw_object_menu(
                        ctx,
                        &mut object_menu,
                        &mut bmp_cache,
                        cockpit_layout,
                        cockpit_state.faction,
                        pointer_owner_input_enabled,
                    ) {
                        Some((ObjectMenuCommand::Encyclopedia, selection)) => {
                            if let Some(session) = encyclopedia_session_store.current() {
                                let requested_object_id = selection
                                    .and_then(|object| {
                                        menu_object_encyclopedia_id(
                                            &world,
                                            session.catalog(),
                                            object,
                                        )
                                    })
                                    .unwrap_or(0);
                                let intent = EncyclopediaEntryIntent::Contextual {
                                    audience: encyclopedia_audience(player_faction),
                                    object_id: requested_object_id,
                                    caller: EncyclopediaContextCaller::Handler00486fb0Event100,
                                };
                                match encyclopedia_surface.open(&session, intent) {
                                    Ok(()) => {
                                        legacy_restriction_latch.note_start();
                                        macroquad::logging::info!(
                                            "[encyclopedia] production_route status=opened origin=contextual caller=FUN_00486fb0_event_0x100 requested={:#010x}",
                                            requested_object_id
                                        );
                                    }
                                    Err(error) => {
                                        macroquad::logging::error!(
                                            "[encyclopedia] production_route rejected origin=contextual error={}",
                                            error
                                        );
                                    }
                                }
                            } else {
                                macroquad::logging::warn!(
                                    "[encyclopedia] production_route unavailable origin=contextual reason=no_session"
                                );
                            }
                        }
                        // FUN_00486fb0 0x103 → FUN_0042a440: the Status
                        // window (type 0x1a) for the selection.
                        Some((ObjectMenuCommand::Status, Some(selection)))
                            if selection.status_object().is_some() =>
                        {
                            let object = selection
                                .status_object()
                                .expect("the guard saw a Status object");
                            status_window_state.open(object);
                            legacy_restriction_latch.note_start();
                            let rect = rebellion_render::status_window::window_rect(cockpit_layout);
                            macroquad::logging::info!(
                                "[interface] command=0x103 destination=status_window status=opened object={:?} rect={},{},{},{}",
                                object,
                                rect.min.x,
                                rect.min.y,
                                rect.width(),
                                rect.height()
                            );
                        }
                        // FUN_00487c50 builds the order with the selection
                        // as its team; FUN_00429320 starts targeting.
                        Some((ObjectMenuCommand::Mission, Some(object))) => {
                            if let Some(member) = object.mission_member() {
                                targeting =
                                    Some(Targeting::new(TargetOrder::Mission(vec![member])));
                                legacy_restriction_latch.note_start();
                            }
                        }
                        // FUN_004ac730 → FUN_00486fb0: a move with no
                        // target goes to targeting too (FUN_00429320).
                        Some((
                            command @ (ObjectMenuCommand::Move | ObjectMenuCommand::ConfirmedMove),
                            Some(MenuObject::Fleet(fleet)),
                        )) => {
                            targeting = Some(Targeting::new(TargetOrder::FleetMove {
                                fleets: vec![fleet],
                                confirmed: command == ObjectMenuCommand::ConfirmedMove,
                            }));
                            legacy_restriction_latch.note_start();
                        }
                        // FUN_00512700, kind 0x10: the order's team is the
                        // system's fleets of the player's side.
                        Some((
                            command @ (ObjectMenuCommand::Move | ObjectMenuCommand::ConfirmedMove),
                            Some(MenuObject::SystemIcon {
                                system,
                                quadrant: Quadrant::Fleets,
                            }),
                        )) => {
                            targeting = Some(Targeting::new(TargetOrder::FleetMove {
                                fleets: system_side_fleets(
                                    &world,
                                    system,
                                    player_faction == MissionFaction::Alliance,
                                ),
                                confirmed: command == ObjectMenuCommand::ConfirmedMove,
                            }));
                            legacy_restriction_latch.note_start();
                        }
                        // FUN_0053c4b0 expands a character's or special
                        // force's move into 0x241 Move missions.
                        Some((
                            command @ (ObjectMenuCommand::Move | ObjectMenuCommand::ConfirmedMove),
                            Some(object @ (MenuObject::Character(_) | MenuObject::SpecialForce(_))),
                        )) => {
                            if let Some(member) = object.mission_member() {
                                targeting = Some(Targeting::new(TargetOrder::MemberMove {
                                    members: vec![member],
                                    confirmed: command == ObjectMenuCommand::ConfirmedMove,
                                }));
                            }
                        }
                        Some((
                            command @ (ObjectMenuCommand::Move | ObjectMenuCommand::ConfirmedMove),
                            Some(MenuObject::Troop(troop)),
                        )) => {
                            targeting = Some(Targeting::new(TargetOrder::TroopMove {
                                troop,
                                confirmed: command == ObjectMenuCommand::ConfirmedMove,
                            }));
                            legacy_restriction_latch.note_start();
                        }
                        Some((
                            command @ (ObjectMenuCommand::Move | ObjectMenuCommand::ConfirmedMove),
                            Some(MenuObject::Fighter { fleet, index }),
                        )) => {
                            targeting = Some(Targeting::new(TargetOrder::SquadronMove {
                                source: SquadronSource::Fleet { fleet, index },
                                confirmed: command == ObjectMenuCommand::ConfirmedMove,
                            }));
                            legacy_restriction_latch.note_start();
                        }
                        Some((
                            command @ (ObjectMenuCommand::Move | ObjectMenuCommand::ConfirmedMove),
                            Some(MenuObject::SystemFighter {
                                system,
                                class,
                                is_alliance,
                            }),
                        )) => {
                            targeting = Some(Targeting::new(TargetOrder::SquadronMove {
                                source: SquadronSource::System {
                                    system,
                                    class,
                                    is_alliance,
                                },
                                confirmed: command == ObjectMenuCommand::ConfirmedMove,
                            }));
                            legacy_restriction_latch.note_start();
                        }
                        // FUN_00561bb0: a Command submenu order gives the
                        // character the rank, or takes it when it holds it.
                        Some((
                            ObjectMenuCommand::Rank(kind),
                            Some(MenuObject::Character(character)),
                        )) => {
                            let is_alliance = player_faction == MissionFaction::Alliance;
                            match rebellion_core::officer_rank::command_refusal(
                                &world,
                                character,
                                is_alliance,
                                kind,
                            ) {
                                None => {
                                    rebellion_core::officer_rank::give_command(
                                        &mut world, character, kind,
                                    );
                                    let held = world
                                        .characters
                                        .get(character)
                                        .map(|value| (value.command_kind, value.command_post));
                                    macroquad::logging::info!(
                                        "[interface] command={:#x} destination=officer_rank status=applied character={} held={:?}",
                                        kind.order(),
                                        world
                                            .characters
                                            .get(character)
                                            .map_or("", |value| value.display_name()),
                                        held
                                    );
                                }
                                Some(refusal) => macroquad::logging::info!(
                                    "[interface] command={:#x} destination=officer_rank status=refused reason={:?}",
                                    kind.order(),
                                    refusal
                                ),
                            }
                        }
                        // FUN_004424c0 -> FUN_004ac730 -> FUN_0041cdf0: a
                        // bombardment row executes its order at once on the
                        // selection's team (FUN_005820a0).
                        Some((ObjectMenuCommand::Bombard(kind), selection)) => {
                            let fleets = strike_orders::menu_team(
                                &world,
                                selection,
                                player_faction == MissionFaction::Alliance,
                            );
                            panel_actions.push(PanelAction::Bombard { fleets, kind });
                        }
                        // The assault row executes its order at once on the
                        // selection's team, as a bombardment row does.
                        Some((ObjectMenuCommand::Assault, selection)) => {
                            let fleets = strike_orders::menu_team(
                                &world,
                                selection,
                                player_faction == MissionFaction::Alliance,
                            );
                            panel_actions.push(PanelAction::Assault { fleets });
                        }
                        // FUN_00487cc0 always confirms kind 0x200; the
                        // window lists the order's team (FUN_0049a350).
                        Some((ObjectMenuCommand::Scrap, selection)) => {
                            if let Some(order) = scrap_order(selection) {
                                let side = if player_faction == MissionFaction::Alliance {
                                    Faction::Alliance
                                } else {
                                    Faction::Empire
                                };
                                scrap_confirmation_state.open(ScrapConfirmation {
                                    faction: player_faction,
                                    targets: rebellion_core::scrap::order_targets(
                                        &world, order, side,
                                    ),
                                    names: rebellion_core::scrap::order_names(&world, order, side),
                                });
                                legacy_restriction_latch.note_start();
                                macroquad::logging::info!(
                                    "[interface] command=0x200 destination=scrap_confirmation status=opened order={:?}",
                                    order
                                );
                            }
                        }
                        Some((
                            command @ (ObjectMenuCommand::Move | ObjectMenuCommand::ConfirmedMove),
                            Some(MenuObject::Ship {
                                fleet,
                                index,
                                roster,
                            }),
                        )) => {
                            targeting = Some(Targeting::new(TargetOrder::ShipMove {
                                fleet,
                                ships: vec![index],
                                roster,
                                confirmed: command == ObjectMenuCommand::ConfirmedMove,
                            }));
                            legacy_restriction_latch.note_start();
                        }
                        // FUN_00580b00: Create Fleet runs at once, against the
                        // ships' own system.
                        Some((
                            ObjectMenuCommand::CreateFleet,
                            Some(MenuObject::Ship {
                                fleet,
                                index,
                                roster,
                            }),
                        )) => panel_actions.push(PanelAction::CreateFleet {
                            fleet,
                            ships: vec![index],
                            roster,
                        }),
                        Some((
                            ObjectMenuCommand::Destination,
                            Some(MenuObject::SystemIcon {
                                system,
                                quadrant: Quadrant::System,
                            }),
                        )) => {
                            targeting = Some(Targeting::new(TargetOrder::Destination {
                                system,
                                area: None,
                            }));
                            legacy_restriction_latch.note_start();
                        }
                        // A band's manager takes the order for its own area
                        // (manual p. 83, "select Destination from this menu
                        // and then click the targeting cross hairs").
                        Some((
                            ObjectMenuCommand::Destination,
                            Some(MenuObject::Producer { system, area }),
                        )) => {
                            targeting = Some(Targeting::new(TargetOrder::Destination {
                                system,
                                area: Some(area),
                            }));
                            legacy_restriction_latch.note_start();
                        }
                        // Build (0x210..0x212): FUN_0041d640 opens Build
                        // Selection on what the band's manager lists.
                        Some((
                            ObjectMenuCommand::Build,
                            Some(MenuObject::Producer { system, area }),
                        )) => {
                            let is_alliance = player_faction == MissionFaction::Alliance;
                            let kinds = rebellion_core::build_selection::listed_classes(
                                &world,
                                &research_state,
                                area,
                                is_alliance,
                            );
                            if build_selection_state.open(system, area, is_alliance, kinds) {
                                legacy_restriction_latch.note_start();
                                macroquad::logging::info!(
                                    "[interface] command=0x21x destination=build_selection status=opened area={:?} system={}",
                                    area,
                                    world.systems.get(system).map_or("", |value| value.name.as_str())
                                );
                            }
                        }
                        // FUN_00487cc0 confirms Stop (0x213) as it does
                        // Scrap; the window lists the band (FUN_0049a350).
                        Some((
                            ObjectMenuCommand::Stop,
                            Some(MenuObject::Producer { system, area }),
                        )) => {
                            if let Some(confirmation) = StopConfirmation::for_band(
                                &world,
                                &mfg_state,
                                player_faction,
                                system,
                                area,
                            ) {
                                stop_confirmation_state.open(confirmation);
                                legacy_restriction_latch.note_start();
                                macroquad::logging::info!(
                                    "[interface] command=0x213 destination=stop_confirmation status=opened area={:?}",
                                    area
                                );
                            }
                        }
                        // FUN_00486fb0 0x215 → FUN_0041d600 → FUN_00429350 →
                        // FUN_00458b50: the edit opens over the band's
                        // product name in its System window.
                        Some((
                            ObjectMenuCommand::Rename,
                            Some(MenuObject::Producer { system, area }),
                        )) => {
                            let name = band_rename_target(
                                &world,
                                &mfg_state,
                                system,
                                area,
                                player_faction == MissionFaction::Alliance,
                            )
                            .and_then(|(fleet, index)| world.ship_name(fleet, index))
                            .map(str::to_owned);
                            if let Some(name) = name {
                                if system_window_state.begin_band_rename(system, area, &name) {
                                    // FUN_00458640's SetFocus lands in the
                                    // System window, as its 0x467 does.
                                    galaxy_focus.activate(GalaxyChild::System(system));
                                    macroquad::logging::info!(
                                        "[interface] command=0x215 destination=band_rename status=opened area={:?}",
                                        area
                                    );
                                }
                            }
                        }
                        // FUN_00486fb0 0x203 → FUN_0041d600 → FUN_00429350:
                        // the edit opens in the window that lists the object.
                        // port: with no Fleet window open there, one opens.
                        Some((
                            ObjectMenuCommand::Rename,
                            Some(object @ (MenuObject::Fleet(_) | MenuObject::Ship { .. })),
                        )) => {
                            let entry = match object {
                                MenuObject::Ship { fleet, index, .. } => {
                                    Some(rebellion_render::fleet_window::FleetWindowEntry::Ship {
                                        fleet,
                                        index,
                                    })
                                }
                                MenuObject::Fleet(fleet) => Some(
                                    rebellion_render::fleet_window::FleetWindowEntry::Fleet(fleet),
                                ),
                                _ => None,
                            };
                            if let Some((entry, system)) = entry.and_then(|entry| {
                                rebellion_core::movement::listed_location(
                                    &movement_state,
                                    &world,
                                    entry.fleet(),
                                )
                                .map(|system| (entry, system))
                            }) {
                                let visible = fleet_window_state
                                    .window_is_visible(strategic_frame_boundary.snapshot(), system);
                                if fleet_window_state.window_id(system).is_some() || !visible {
                                    let point = if visible {
                                        fleet_window_state.window_logical_position(system)
                                    } else {
                                        sector_window_state.open_for_system(
                                            &world,
                                            system,
                                            cockpit_state.faction,
                                        );
                                        sector_window_state.fleet_window_point(
                                            &world,
                                            cockpit_layout,
                                            system,
                                        )
                                    };
                                    if let Some(point) = point {
                                        if let Some(prepared) = fleet_window_state
                                            .prepare_open(
                                                &world,
                                                system,
                                                point,
                                                cockpit_state.faction,
                                                cockpit_layout,
                                                Some(entry),
                                            )
                                            .and_then(|prepared| {
                                                prepared.with_rename(&world, entry)
                                            })
                                        {
                                            strategic_windowing.enqueue_fleet_open(prepared);
                                            legacy_restriction_latch.note_start();
                                        }
                                    }
                                } else if fleet_window_state.begin_rename(&world, system, entry) {
                                    legacy_restriction_latch.note_start();
                                }
                            }
                        }
                        // port: the other items are drawn disabled.
                        Some(_) | None => {}
                    }
                    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
                    if let (Some(request), Some(menu), Some(rect)) = (
                        interface_fixture_request,
                        object_menu.as_ref(),
                        rebellion_render::object_menu::object_menu_rect(ctx),
                    ) {
                        interface_test_fixture::emit_object_menu(
                            request,
                            rect,
                            menu,
                            &world,
                            &sector_window_state,
                        );
                    }

                    match draw_mission_dialog(
                        ctx,
                        &world,
                        &mut mission_dialog_state,
                        cockpit_layout,
                        &mut bmp_cache,
                    ) {
                        Some(MissionDialogAction::Begin {
                            kind,
                            faction,
                            team,
                            decoys,
                            target,
                            object,
                        }) => panel_actions.push(PanelAction::DispatchMission {
                            kind,
                            faction,
                            team,
                            decoys,
                            target,
                            target_character: None,
                            target_object: object,
                            tick: clock.tick,
                        }),
                        Some(MissionDialogAction::Encyclopedia) => {
                            if let Some(session) = encyclopedia_session_store.current() {
                                let intent = EncyclopediaEntryIntent::Contextual {
                                    audience: encyclopedia_audience(player_faction),
                                    // The selected mission-kind-to-object join
                                    // is not source-proven. Preserve the exact
                                    // caller and use the presenter's explicit
                                    // unresolved-context index fallback.
                                    object_id: 0,
                                    caller:
                                        EncyclopediaContextCaller::MissionDialog0046c3c0Command67,
                                };
                                match encyclopedia_surface.open(&session, intent) {
                                    Ok(()) => {
                                        macroquad::logging::info!(
                                            "[encyclopedia] production_route status=opened origin=contextual caller=FUN_0046c3c0_command_0x67 requested=unresolved"
                                        );
                                    }
                                    Err(error) => {
                                        macroquad::logging::error!(
                                            "[encyclopedia] production_route rejected origin=mission_dialog error={}",
                                            error
                                        );
                                    }
                                }
                            } else {
                                macroquad::logging::warn!(
                                    "[encyclopedia] production_route unavailable origin=mission_dialog reason=no_session"
                                );
                            }
                        }
                        None => {}
                    }

                    match draw_build_selection(
                        ctx,
                        &world,
                        &mfg_state,
                        &troop_transport_state,
                        &delivery_state,
                        &movement_state,
                        &mut build_selection_state,
                        cockpit_layout,
                        &mut bmp_cache,
                    ) {
                        Some(BuildSelectionAction::Confirm {
                            system,
                            area,
                            kind,
                            count,
                        }) => panel_actions.push(PanelAction::BuildProduction {
                            system,
                            area,
                            kind,
                            count,
                        }),
                        Some(BuildSelectionAction::Encyclopedia) => {
                            if let Some(session) = encyclopedia_session_store.current() {
                                let intent = EncyclopediaEntryIntent::Contextual {
                                    audience: encyclopedia_audience(player_faction),
                                    // Upstream recovered this caller while the
                                    // class-to-topic join remains explicitly
                                    // unbound. Preserve the caller and use the
                                    // presenter's unresolved index fallback.
                                    object_id: 0,
                                    caller: EncyclopediaContextCaller::Handler00438800Command67,
                                };
                                match encyclopedia_surface.open(&session, intent) {
                                    Ok(()) => {
                                        macroquad::logging::info!(
                                            "[encyclopedia] production_route status=opened origin=contextual caller=FUN_00438800_command_0x67 requested=unresolved"
                                        );
                                    }
                                    Err(error) => {
                                        macroquad::logging::error!(
                                            "[encyclopedia] production_route rejected origin=build_selection error={}",
                                            error
                                        );
                                    }
                                }
                            } else {
                                macroquad::logging::warn!(
                                    "[encyclopedia] production_route unavailable origin=build_selection reason=no_session"
                                );
                            }
                        }
                        None => {}
                    }
                    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
                    if let Some(request) = interface_fixture_request {
                        interface_test_fixture::emit_production(
                            request,
                            &world,
                            &mfg_state,
                            &system_window_state,
                            &build_selection_state,
                        );
                    }

                    // FUN_00429440: the Finder's choice opens its system's
                    // sector window, then the Fleet window from the fleet
                    // icon with the fleet or ship selected.
                    if let Some(FleetFinderAction::Open(entry)) = draw_fleet_finder(
                        ctx,
                        &world,
                        &movement_state,
                        fog_state,
                        &mut fleet_finder_state,
                        cockpit_layout,
                        &mut bmp_cache,
                    ) {
                        // A fleet in hyperspace is its destination's child
                        // (FUN_00556390), so its windows open there.
                        if let Some(system) = rebellion_core::movement::listed_location(
                            &movement_state,
                            &world,
                            entry.fleet(),
                        ) {
                            sector_window_state.open_for_system(
                                &world,
                                system,
                                cockpit_state.faction,
                            );
                            if let Some(point) = sector_window_state.fleet_window_point(
                                &world,
                                cockpit_layout,
                                system,
                            ) {
                                map_state.selected_system = Some(system);
                                enqueue_fleet_window(
                                    &mut strategic_windowing,
                                    &fleet_window_state,
                                    &world,
                                    system,
                                    point,
                                    cockpit_state.faction,
                                    cockpit_layout,
                                    Some(entry),
                                );
                            }
                        }
                    }

                    // FUN_0046df90 (Troop Finder) / FUN_00464c20 (Personnel
                    // Finder) command slots: a Finder's choice goes through
                    // FUN_00429440, which opens the target's sector window
                    // and then the window for its kind.
                    let troop_choice = draw_troop_finder(
                        ctx,
                        &world,
                        &mut troop_finder_state,
                        cockpit_layout,
                        &mut bmp_cache,
                        &troop_transport_state,
                    )
                    .map(|action| match action {
                        TroopFinderAction::OpenDefenses { system } => {
                            (system, FinderTarget::Defenses)
                        }
                        TroopFinderAction::OpenFleet { system, fleet } => {
                            (system, FinderTarget::Fleet(fleet))
                        }
                    });
                    let personnel_choice = draw_personnel_finder(
                        ctx,
                        &world,
                        fog_state,
                        &mut personnel_finder_state,
                        cockpit_layout,
                        &mut bmp_cache,
                    )
                    .and_then(|action| match action {
                        // FUN_00429440 family 0x30 (character_target).
                        PersonnelFinderAction::OpenCharacter { character, system } => {
                            use rebellion_render::personnel_finder::{
                                character_target, CharacterTarget,
                            };
                            match world.characters.get(character).map(character_target) {
                                Some(CharacterTarget::Fleet(fleet)) => {
                                    rebellion_core::movement::listed_location(
                                        &movement_state,
                                        &world,
                                        fleet,
                                    )
                                    .map(|system| (system, FinderTarget::Fleet(fleet)))
                                }
                                Some(CharacterTarget::Missions) => system.map(|system| {
                                    (
                                        system,
                                        FinderTarget::Missions(
                                            rebellion_core::missions::MissionMember::Character(
                                                character,
                                            ),
                                        ),
                                    )
                                }),
                                Some(CharacterTarget::Defenses) | None => {
                                    system.map(|system| (system, FinderTarget::Defenses))
                                }
                            }
                        }
                        PersonnelFinderAction::OpenSystem { system } => {
                            Some((system, FinderTarget::Sector))
                        }
                        PersonnelFinderAction::OpenFleet { system, fleet } => {
                            Some((system, FinderTarget::Fleet(fleet)))
                        }
                    });
                    // FUN_0048a060 class 7: a report filed for the side
                    // that owns the fleet opens its window (window message
                    // 0x468, wParam 0x1c); only the player's side shows one.
                    let player_is_alliance = player_faction == MissionFaction::Alliance;
                    for id in msg_log.take_alerts() {
                        if let Some(message) = msg_log
                            .get(id)
                            .filter(|message| message.audience.includes(player_is_alliance))
                        {
                            operation_report_state.open(
                                &world,
                                message,
                                cockpit_state.faction,
                                None,
                            );
                        }
                    }
                    let report_choice = match draw_operation_report(
                        ctx,
                        &mut operation_report_state,
                        cockpit_layout,
                        &world,
                        &movement_state,
                        &mut bmp_cache,
                        &text_templates
                            .format(rebellion_render::operation_report::DISPLAY_TEXT_ID, &[])
                            .unwrap_or_default(),
                    ) {
                        // FUN_00469d60: a report the Index opened reopens it.
                        Some(OperationReportAction::Closed { reopen_index }) => {
                            if let Some(category) = reopen_index.filter(|_| message_index.is_none())
                            {
                                let mut index = MessageIndexState::new();
                                let actions = index.set_category(category);
                                message_index = Some(index);
                                legacy_restriction_latch.note_start();
                                apply_message_index_actions(
                                    actions,
                                    &mut msg_log,
                                    &mut clock,
                                    &mut advice_saved_speed,
                                    &mut pending_message_sound,
                                );
                            }
                            None
                        }
                        // FUN_0044df90 0x0b / 0x0c: the system's Sector
                        // window, then for 0x0c its fleets (hyp: the
                        // report's fleet still at the system).
                        Some(
                            ref action @ (OperationReportAction::GotoSystem { .. }
                            | OperationReportAction::GotoFleet { .. }),
                        ) => report_destination(action),
                        None => None,
                    };
                    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
                    if let Some(request) = interface_fixture_request {
                        operation_report_watch.observe_live(
                            request,
                            &operation_report_state,
                            &world,
                            &movement_state,
                            &interface_test_fixture::ReportDestinations {
                                sectors: &sector_window_state,
                                fleets: &fleet_window_state,
                                chosen: report_choice.map(|(system, target)| {
                                    (system, target == FinderTarget::FleetWindow)
                                }),
                            },
                        );
                    }
                    if let Some((system, target)) = troop_choice
                        .or(personnel_choice)
                        .or(message_choice)
                        .or(report_choice)
                    {
                        sector_window_state.open_for_system(&world, system, cockpit_state.faction);
                        map_state.selected_system = Some(system);
                        match target {
                            FinderTarget::Sector => {}
                            FinderTarget::Defenses => {
                                if let Some(point) = sector_window_state.quadrant_window_point(
                                    &world,
                                    cockpit_layout,
                                    system,
                                    rebellion_render::quadrant_icons::Quadrant::Defenses,
                                ) {
                                    defenses_window_state.open(
                                        &world,
                                        system,
                                        point,
                                        cockpit_state.faction,
                                        cockpit_layout,
                                    );
                                }
                            }
                            // FUN_00429440 kind 11, then slot 27
                            // (FUN_004a1e10) shows the member.
                            FinderTarget::Missions(member) => {
                                if let Some(point) = sector_window_state.quadrant_window_point(
                                    &world,
                                    cockpit_layout,
                                    system,
                                    rebellion_render::quadrant_icons::Quadrant::Missions,
                                ) {
                                    missions_window_state.open(
                                        &world,
                                        fog_state,
                                        &mission_state,
                                        system,
                                        point,
                                        cockpit_state.faction,
                                        cockpit_layout,
                                    );
                                    missions_window_state.show_member(
                                        &mission_state,
                                        system,
                                        member,
                                    );
                                }
                            }
                            FinderTarget::Fleet(_) | FinderTarget::FleetWindow => {
                                if let Some(point) = sector_window_state.fleet_window_point(
                                    &world,
                                    cockpit_layout,
                                    system,
                                ) {
                                    let selected = match target {
                                        FinderTarget::Fleet(fleet) => {
                                            Some(FleetWindowEntry::Fleet(fleet))
                                        }
                                        _ => None,
                                    };
                                    enqueue_fleet_window(
                                        &mut strategic_windowing,
                                        &fleet_window_state,
                                        &world,
                                        system,
                                        point,
                                        cockpit_state.faction,
                                        cockpit_layout,
                                        selected,
                                    );
                                }
                            }
                        }
                    }

                    // FUN_004443a0: 0x66 closes the Status window and opens
                    // the Encyclopedia; 0x65 closes it.
                    if let Some(action) = draw_status_window(
                        ctx,
                        StatusSources {
                            world: &world,
                            missions: &mission_state,
                            movement: &movement_state,
                            transport: &troop_transport_state,
                            manufacturing: &mfg_state,
                            today: clock.tick,
                        },
                        &mut status_window_state,
                        cockpit_state.faction,
                        cockpit_layout,
                        &mut bmp_cache,
                    ) {
                        macroquad::logging::info!(
                            "[interface] destination=status_window status=closed action={}",
                            match action {
                                StatusWindowAction::Encyclopedia => "encyclopedia",
                                StatusWindowAction::Close => "close",
                            }
                        );
                        if action == StatusWindowAction::Encyclopedia {
                            if let Some(session) = encyclopedia_session_store.current() {
                                let intent = EncyclopediaEntryIntent::Contextual {
                                    audience: encyclopedia_audience(player_faction),
                                    // The Status window's selected-object
                                    // admission join is not part of the
                                    // merged upstream contract yet.
                                    object_id: 0,
                                    caller: EncyclopediaContextCaller::Handler004443a0Command66,
                                };
                                match encyclopedia_surface.open(&session, intent) {
                                    Ok(()) => {
                                        macroquad::logging::info!(
                                            "[encyclopedia] production_route status=opened origin=contextual caller=FUN_004443a0_command_0x66 requested=unresolved"
                                        );
                                    }
                                    Err(error) => {
                                        macroquad::logging::error!(
                                            "[encyclopedia] production_route rejected origin=status_window error={}",
                                            error
                                        );
                                    }
                                }
                            } else {
                                macroquad::logging::warn!(
                                    "[encyclopedia] production_route unavailable origin=status_window reason=no_session"
                                );
                            }
                        }
                    }

                    // FUN_0044f5e0: the checkmark resubmits with force 1,
                    // which validates again and departs without asking.
                    if let Some(MoveConfirmationAction::Confirm {
                        fleets,
                        members,
                        ships,
                        units,
                        destination,
                        join,
                    }) = draw_move_confirmation(
                        ctx,
                        &mut move_confirmation_state,
                        cockpit_layout,
                        &mut bmp_cache,
                    ) {
                        if !members.is_empty() {
                            panel_actions.push(PanelAction::MoveMembers {
                                members,
                                destination: join.map_or(
                                    rebellion_core::missions::MoveDestination::System(destination),
                                    rebellion_core::missions::MoveDestination::Fleet,
                                ),
                            });
                        }
                        panel_actions.extend(
                            fleets
                                .into_iter()
                                .map(FleetMover::Fleet)
                                .chain(ships)
                                .map(|mover| mover_move_action(mover, destination, join)),
                        );
                        let target = join.map_or(RegimentTarget::System(destination), RegimentTarget::Fleet);
                        panel_actions.extend(units.into_iter().map(|unit| unit_move_action(unit, target)));
                    }

                    // FUN_0044f5e0: the checkmark submits the Stop order
                    // (the band drops what it was building, manual p. 84);
                    // the X destroys it.
                    if let Some(StopConfirmationAction::Confirm { system, area }) =
                        draw_stop_confirmation(
                            ctx,
                            &mut stop_confirmation_state,
                            cockpit_layout,
                            &mut bmp_cache,
                        )
                    {
                        panel_actions.push(PanelAction::StopProduction { system, area });
                    }

                    // FUN_0044f5e0: the checkmark submits the Scrap order,
                    // which scraps each object at once (FUN_004f84e0 →
                    // FUN_00534c00, reason 0x14).
                    if let Some(ScrapConfirmationAction::Confirm(targets)) = draw_scrap_confirmation(
                        ctx,
                        &mut scrap_confirmation_state,
                        cockpit_layout,
                        &mut bmp_cache,
                    ) {
                        let scrapped = rebellion_core::scrap::scrap_all_reporting(
                            &mut world,
                            &mut stockpile_state,
                            &mut troop_transport_state,
                            targets.iter().copied(),
                        );
                        let refund = scrapped.refund;
                        for loss in &scrapped.losses {
                            let system = world
                                .systems
                                .get(loss.system)
                                .map_or("", |value| value.name.as_str());
                            macroquad::logging::info!("{}", loss.log_line(system));
                        }
                        macroquad::logging::info!(
                            "[interface] command=0x200 status=scrapped objects={} refund={}",
                            targets.len(),
                            refund
                        );
                        // A scrapped regiment's holder rerun (FUN_00508660),
                        // in the order's step (FUN_0051df30).
                        let step = ai_state.scheduler.next_step();
                        fire_support_events(
                            &mut world,
                            &mut mfg_state,
                            &mut troop_transport_state,
                            &movement_state,
                            &mut delivery_state,
                            &text_templates,
                            &mut msg_log,
                            clock.tick,
                            step,
                        );
                        sweep_flagged(
                            &mut world,
                            &mut mission_state,
                            &troop_transport_state,
                            &movement_state,
                            &blockade_state,
                            clock.tick,
                        );
                    }

                    // FUN_00422ce0's WM_KEYDOWN has no VK_ESCAPE case: in the
                    // original Escape neither cancels targeting nor reaches
                    // the main menu (VM, alliance-day927-orto-orbit.sav); a
                    // left release off any target ends it below.
                    if targeting.is_some()
                        && strategic_frame_boundary.legacy_surface_input_allowed()
                        && strategic_root_keys.contains(KeyCode::Escape)
                    {
                        legacy_keyboard_handled = true;
                    }
                    if targeting.is_some() {
                        capture_pointer(ctx);
                        let (mouse_x, mouse_y) = mouse_position();
                        let pointer = ctx
                            .input(|input| input.pointer.latest_pos())
                            .unwrap_or(egui_macroquad::egui::pos2(mouse_x, mouse_y));
                        targeting_cursor_drawn =
                            draw_targeting_cursor(ctx, &mut bmp_cache, cockpit_layout, pointer);
                        // FUN_00422ce0's WM_LBUTTONUP in mode 2 targets what
                        // the window under the point answers, then
                        // FUN_0042a320 opens the dialog with the kinds the
                        // team may undertake; with none, nothing opens.
                        if pointer_owner_input_enabled
                            && targeting_held
                            && is_mouse_button_released(MouseButton::Left)
                        {
                            let windows = ReleaseWindows {
                                sector: &sector_window_state,
                                system: &system_window_state,
                                fleet: &fleet_window_state,
                                defenses: &defenses_window_state,
                                missions: &missions_window_state,
                            };
                            let destination = release_destination(
                                ctx,
                                &world,
                                &movement_state,
                                fog_state,
                                cockpit_layout,
                                windows,
                                pointer,
                            );
                            // A Mission order asks the window's +0x68 object.
                            let object = rebellion_render::targeting::release_object(
                                ctx,
                                &world,
                                &movement_state,
                                fog_state,
                                &troop_transport_state,
                                cockpit_layout,
                                windows,
                                pointer,
                            );
                            match targeting.take().map(|order| order.release(destination)) {
                                Some(TargetingEnd::Target {
                                    order: TargetOrder::Mission(team),
                                    target,
                                }) => {
                                    let system = target.system();
                                    let kinds = rebellion_core::missions::available_kinds(
                                        &world,
                                        &uprising_state,
                                        player_faction,
                                        &team,
                                        &[],
                                        system,
                                        object,
                                    );
                                    macroquad::logging::info!(
                                        "[order] mission dialog {:?} at {:?} on {:?}: {:?}",
                                        team,
                                        system,
                                        object,
                                        kinds
                                    );
                                    mission_dialog_state.open(
                                        player_faction,
                                        system,
                                        object,
                                        team,
                                        kinds,
                                    );
                                }
                                Some(TargetingEnd::Target {
                                    order: TargetOrder::FleetMove { fleets, confirmed },
                                    target,
                                }) => {
                                    if issue_fleet_move(
                                        &FleetMoveContext {
                                            world: &world,
                                            movement: &movement_state,
                                            blockaded: blockade_state.blockaded_systems(),
                                            faction: player_faction,
                                        },
                                        &fleets,
                                        confirmed,
                                        target,
                                        &mut move_confirmation_state,
                                        &mut panel_actions,
                                    ) {
                                        legacy_restriction_latch.note_start();
                                    }
                                }
                                Some(TargetingEnd::Target {
                                    order: TargetOrder::MemberMove { members, confirmed },
                                    target,
                                }) => issue_member_move(
                                    &FleetMoveContext {
                                        world: &world,
                                        movement: &movement_state,
                                        blockaded: blockade_state.blockaded_systems(),
                                        faction: player_faction,
                                    },
                                    &mission_state,
                                    members,
                                    confirmed,
                                    target,
                                    &mut advisor_state,
                                    &mut move_confirmation_state,
                                    &mut panel_actions,
                                ),
                                Some(TargetingEnd::Target {
                                    order:
                                        TargetOrder::ShipMove {
                                            fleet,
                                            ships,
                                            roster,
                                            confirmed,
                                        },
                                    target,
                                }) => {
                                    if issue_ship_move(
                                        &FleetMoveContext {
                                            world: &world,
                                            movement: &movement_state,
                                            blockaded: blockade_state.blockaded_systems(),
                                            faction: player_faction,
                                        },
                                        fleet,
                                        ships,
                                        roster,
                                        confirmed,
                                        target,
                                        &mut move_confirmation_state,
                                        &mut panel_actions,
                                    ) {
                                        legacy_restriction_latch.note_start();
                                    }
                                }
                                // A regiment's or squadron's 0x201 or 0x202
                                // against the release window's +0x70
                                // (FUN_004a3130 for a Fleet window), boarding,
                                // landing or travelling.
                                Some(TargetingEnd::Target {
                                    order:
                                        order @ (TargetOrder::TroopMove { .. }
                                        | TargetOrder::SquadronMove { .. }),
                                    target,
                                }) => {
                                    let (unit, confirmed) = match order {
                                        TargetOrder::TroopMove { troop, confirmed } => {
                                            (UnitMover::Regiment(troop), confirmed)
                                        }
                                        TargetOrder::SquadronMove { source, confirmed } => {
                                            (UnitMover::Squadron(source), confirmed)
                                        }
                                        _ => unreachable!("matched a unit order"),
                                    };
                                    if issue_unit_move(
                                        &FleetMoveContext {
                                            world: &world,
                                            movement: &movement_state,
                                            blockaded: blockade_state.blockaded_systems(),
                                            faction: player_faction,
                                        },
                                        &troop_transport_state,
                                        &ProductRoom::new(&mfg_state, &delivery_state),
                                        unit,
                                        confirmed,
                                        target,
                                        &mut move_confirmation_state,
                                        &mut panel_actions,
                                    ) {
                                        legacy_restriction_latch.note_start();
                                    }
                                }
                                // FUN_00512700 kind 4: the order's team is
                                // every production area of the system, or
                                // a band's own area.
                                Some(TargetingEnd::Target {
                                    order: TargetOrder::Destination { system, area },
                                    target,
                                }) => panel_actions.push(PanelAction::SetDestination {
                                    system,
                                    area,
                                    destination: match target {
                                        ReleaseTarget::System(key) => {
                                            rebellion_core::manufacturing::Destination::System(key)
                                        }
                                        ReleaseTarget::Fleet { fleet, .. } => {
                                            rebellion_core::manufacturing::Destination::Fleet(fleet)
                                        }
                                    },
                                }),
                                Some(TargetingEnd::Dropped) | None => {}
                            }
                        }
                    }

                    // The replacement message and status bars covered the
                    // original command controls. Keep those reconstructed
                    // surfaces out of parity mode until their bitmap-driven
                    // versions are restored.

                    // Droid advisor (floating window, bottom-right)
                    draw_advisor(ctx, &mut advisor_state);

                    // Source-backed production Encyclopedia is modal over
                    // every retained command-center window. The legacy
                    // replacement panel remains fixture-only above.
                    if encyclopedia_surface.is_open()
                        && strategic_frame_boundary.legacy_surface_input_allowed()
                        && strategic_root_keys.contains(KeyCode::Escape)
                    {
                        legacy_keyboard_handled = true;
                        if let Some(session) = encyclopedia_session_store.current() {
                            match encyclopedia_surface.apply_action(
                                &session,
                                rebellion_render::EncyclopediaSurfaceAction::Close,
                            ) {
                                Ok(Some(route)) => log_encyclopedia_return(route),
                                Ok(None) => {}
                                Err(error) => {
                                    encyclopedia_surface.close();
                                    macroquad::logging::error!(
                                        "[encyclopedia] production_route close_failed error={}",
                                        error
                                    );
                                }
                            }
                        } else {
                            encyclopedia_surface.close();
                        }
                    }
                    // Escape goes where the keyboard focus is (GalaxyFocus):
                    // the GID menu shows the top window, a detail window
                    // closes, and a sector window or a hidden one does
                    // nothing; the galaxy view never sees the key.
                    if !legacy_keyboard_handled
                        && !legacy_blockers.strategic_modal
                        && !legacy_blockers.transient_menu
                        && !legacy_blockers.focused_control
                        && strategic_frame_boundary.legacy_surface_input_allowed()
                        && strategic_root_keys.contains(KeyCode::Escape)
                    {
                        if let Some(step) = galaxy_focus.escape() {
                            apply_focus_step(
                                step,
                                &mut cockpit_state.gid_ui,
                                &mut sector_window_state,
                                &mut system_window_state,
                                &mut defenses_window_state,
                                &mut missions_window_state,
                                &mut |command| strategic_windowing.enqueue(command),
                            );
                            legacy_keyboard_handled = true;
                        }
                    }
                    if encyclopedia_surface.is_open() {
                        if let Some(session) = encyclopedia_session_store.current() {
                            match encyclopedia_surface.draw(
                                ctx,
                                &mut bmp_cache,
                                &session,
                                &encyclopedia_hd,
                            ) {
                                Ok(Some(route)) => log_encyclopedia_return(route),
                                Ok(None) => {}
                                Err(error) => {
                                    encyclopedia_surface.close();
                                    macroquad::logging::error!(
                                        "[encyclopedia] production_route draw_failed error={}",
                                        error
                                    );
                                }
                            }
                        } else {
                            encyclopedia_surface.close();
                            macroquad::logging::warn!(
                                "[encyclopedia] production_route unavailable reason=no_session"
                            );
                        }
                    }

                    // Under the battle window the command center takes no
                    // pointer input: this foreground layer absorbs it, so the
                    // windows beneath stay as they are.
                    if battle_window_underlay {
                        egui_macroquad::egui::Area::new("battle_window_input_hold".into())
                            .order(egui_macroquad::egui::Order::Foreground)
                            .fixed_pos(egui_macroquad::egui::Pos2::ZERO)
                            .show(ctx, |ui| {
                                ui.allocate_response(
                                    ctx.screen_rect().size(),
                                    egui_macroquad::egui::Sense::click_and_drag(),
                                );
                            });
                    }

                    // The galaxy view holds the capture while targeting, and
                    // the briefing tour's hooks hold it while they are in
                    // (FUN_0041d9d0: a press skips the tour instead).
                    pending_cockpit_button = strategic_input_enabled
                        .then(|| handle_cockpit_egui_input(ctx, &mut cockpit_state, &mut bmp_cache))
                        .flatten();
                    // Future manager-owned content paints after this restore;
                    // every retained legacy surface above was input-muted.
                    if let Some(saved) = retained_legacy_input {
                        restore_egui_input(ctx, saved);
                    }
                    egui_claims_pointer = ctx.is_pointer_over_area() || ctx.wants_pointer_input();
                    egui_claims_keyboard = ctx.wants_keyboard_input();
                });

                strategic_root_keys.report_legacy_children(
                    &mut strategic_input_router,
                    egui_claims_keyboard || legacy_keyboard_handled,
                    fleet_keyboard_owner,
                );

                // Child surfaces have now handled their input. Keep every
                // restriction introduced during dispatch sticky until the next
                // immutable frame boundary, even if that surface also closes.
                #[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
                let command_palette_now_open = command_palette_state.open;
                #[cfg(not(any(debug_assertions, not(target_arch = "wasm32"))))]
                let command_palette_now_open = false;
                legacy_restriction_latch.observe(LegacyInputBlockers {
                    strategic_modal: build_selection_state.is_open()
                        || mission_dialog_state.is_open()
                        || move_confirmation_state.is_open()
                        || scrap_confirmation_state.is_open()
                        || stop_confirmation_state.is_open()
                        || status_window_state.is_open()
                        || fleet_finder_state.is_open()
                        || troop_finder_state.is_open()
                        || personnel_finder_state.is_open()
                        || operation_report_state.is_open()
                        || message_index.is_some()
                        || encyclopedia_surface.is_open()
                        || original_modal_fixture_open,
                    focused_control: command_palette_now_open
                        || system_window_state.band_rename_open(),
                    transient_menu: object_menu.is_some()
                        || game_speed_ui.menu_anchor.is_some()
                        || agent_menu_state.anchor.is_some(),
                    root_gesture: targeting.is_some(),
                });
                let mut strategic_dispatch_restricted = legacy_restriction_latch.started();
                if strategic_dispatch_restricted {
                    strategic_input_router.set_application_hold(true);
                }

                let pending_cockpit_button = (!strategic_dispatch_restricted)
                    .then_some(pending_cockpit_button)
                    .flatten();
                macro_rules! dispatch_cockpit_button {
                    ($btn:expr) => {{
                        let btn = $btn;
                        let (command, destination) = match btn {
                            CockpitButton::SystemFinder => (0x12d, "system_finder"),
                            CockpitButton::FleetFinder => {
                                fleet_finder_state.open(cockpit_state.faction);
                                strategic_dispatch_restricted = true;
                                strategic_input_router.set_application_hold(true);
                                macroquad::logging::info!(
                                    "[interface] command=0x12e destination=fleet_finder status=opened_original"
                                );
                                (0x12e, "fleet_finder")
                            }
                            CockpitButton::PersonnelFinder => {
                                personnel_finder_state.open(cockpit_state.faction);
                                strategic_dispatch_restricted = true;
                                strategic_input_router.set_application_hold(true);
                                macroquad::logging::info!(
                                    "[interface] command=0x12f destination=personnel_finder status=opened_original"
                                );
                                (0x12f, "personnel_finder")
                            }
                            CockpitButton::TroopFinder => {
                                troop_finder_state.open(cockpit_state.faction);
                                strategic_dispatch_restricted = true;
                                strategic_input_router.set_application_hold(true);
                                macroquad::logging::info!(
                                    "[interface] command=0x130 destination=troop_finder status=opened_original"
                                );
                                (0x130, "troop_finder")
                            }
                            CockpitButton::GameOptions => (0x133, "game_options"),
                            CockpitButton::Encyclopedia => (0x131, "encyclopedia"),
                            CockpitButton::GalacticInformationDisplay => {
                                let built_now = cockpit_state.gid_ui.open_menu();
                                galaxy_focus.gid_opened(built_now);
                                strategic_dispatch_restricted = true;
                                strategic_input_router.set_application_hold(true);
                                #[cfg(all(
                                    target_arch = "wasm32",
                                    feature = "interface-test-fixtures"
                                ))]
                                if let Some(event) = strategic_pointer_events.latest_primary_release() {
                                    pending_windowing_route_receipts.push((
                                        event,
                                        "pointer",
                                        "gid_menu",
                                    ));
                                }
                                macroquad::logging::info!(
                                    "[interface] command=0x{:x} destination=gid_menu status=opened_original",
                                    rebellion_render::strategic_gid_control(cockpit_state.faction)
                                        .command_id,
                                );
                                (
                                    rebellion_render::strategic_gid_control(cockpit_state.faction)
                                        .command_id,
                                    "gid_menu",
                                )
                            }
                        };
                        if btn == CockpitButton::Encyclopedia {
                            if let Some(session) = encyclopedia_session_store.current() {
                                let intent = EncyclopediaEntryIntent::Cockpit {
                                    audience: encyclopedia_audience(player_faction),
                                };
                                match encyclopedia_surface.open(&session, intent) {
                                    Ok(()) => {
                                        strategic_dispatch_restricted = true;
                                        strategic_input_router.set_application_hold(true);
                                        macroquad::logging::info!(
                                            "[interface] command=0x{:x} destination={} status=opened_original",
                                            command,
                                            destination
                                        );
                                    }
                                    Err(error) => {
                                        macroquad::logging::error!(
                                            "[interface] command=0x{:x} destination={} status=rejected error={}",
                                            command,
                                            destination,
                                            error
                                        );
                                    }
                                }
                            } else {
                                macroquad::logging::warn!(
                                    "[interface] command=0x{:x} destination={} status=unavailable",
                                    command,
                                    destination
                                );
                            }
                        } else if btn == CockpitButton::GameOptions {
                            save_slots = read_save_slots(&saves_dir);
                            game_options_state.set_origin(GameOptionsOrigin::CommandCenter);
                            game_options_state.refresh_saves(&save_slots);
                            game_mode = GameMode::GameOptions {
                                origin: GameOptionsOrigin::CommandCenter,
                            };
                            strategic_dispatch_restricted = true;
                            strategic_input_router.set_application_hold(true);
                            macroquad::logging::info!(
                                "[interface] command=0x{:x} destination={} status=opened_original",
                                command,
                                destination
                            );
                        } else if matches!(btn, CockpitButton::SystemFinder) {
                            macroquad::logging::info!(
                                "[interface] command=0x{:x} destination={} status=pending_original_window",
                                command,
                                destination
                            );
                        }
                    }};
                }

                if let Some(btn) = pending_cockpit_button {
                    dispatch_cockpit_button!(btn);
                }

                // Root consumes the normalized key presses in backend order,
                // each with its own event id and modifier state; a newly
                // introduced restriction stops later events.
                if strategic_frame_boundary.root_shortcuts_allowed() {
                    for event in strategic_root_keys.presses() {
                        if strategic_dispatch_restricted {
                            break;
                        }

                        let panel_index = match event.key {
                            KeyCode::F => Some(0),
                            KeyCode::N => Some(1),
                            _ => None,
                        };
                        #[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
                        let palette_shortcut = palette_enabled && event.key == KeyCode::GraveAccent;
                        #[cfg(not(any(debug_assertions, not(target_arch = "wasm32"))))]
                        let palette_shortcut = false;
                        let root_action = galaxy_root_acts(
                            event.key,
                            event.alt,
                            cfg!(debug_assertions),
                            palette_shortcut,
                        );
                        if !root_action || !strategic_input_router.claim_root(event.id) {
                            continue;
                        }

                        if let Some(btn) = cockpit_accelerator(event.key) {
                            dispatch_cockpit_button!(btn);
                        } else if event.key == KeyCode::F6 && message_index.is_none() {
                            if let Some(category) =
                                rebellion_render::message_index::opening_category(0x75)
                            {
                                let mut index = MessageIndexState::new();
                                let actions = index.set_category(category);
                                message_index = Some(index);
                                apply_message_index_actions(
                                    actions,
                                    &mut msg_log,
                                    &mut clock,
                                    &mut advice_saved_speed,
                                    &mut pending_message_sound,
                                );
                                strategic_dispatch_restricted = true;
                                strategic_input_router.set_application_hold(true);
                                macroquad::logging::info!(
                                    "[interface] command=0x75 destination=message_index category=0x{:x} status=opened_original",
                                    category
                                );
                            }
                        } else if event.alt
                            && matches!(
                                event.key,
                                KeyCode::P | KeyCode::KpAdd | KeyCode::KpSubtract
                            )
                        {
                            let requested = match event.key {
                                KeyCode::P => Some(GameSpeed::Paused),
                                KeyCode::KpAdd => stepped_game_speed(&clock, true),
                                KeyCode::KpSubtract => stepped_game_speed(&clock, false),
                                _ => None,
                            };
                            if let Some(requested) = requested {
                                choose_game_speed(&mut clock, requested);
                            }
                        } else if event.alt
                            && matches!(event.key, KeyCode::G | KeyCode::U | KeyCode::A)
                        {
                            let command = match event.key {
                                KeyCode::G => {
                                    rebellion_render::agent_menu::AgentCommand::ManageGarrisons
                                }
                                KeyCode::U => {
                                    rebellion_render::agent_menu::AgentCommand::ManageProduction
                                }
                                KeyCode::A => {
                                    rebellion_render::agent_menu::AgentCommand::AgentAdvice
                                }
                                _ => unreachable!(),
                            };
                            apply_agent_command(&mut player_agent, command, "accelerator");
                        } else if event.alt {
                            let digit = match event.key {
                                KeyCode::Key1 => Some(1),
                                KeyCode::Key2 => Some(2),
                                KeyCode::Key3 => Some(3),
                                KeyCode::Key4 => Some(4),
                                KeyCode::Key5 => Some(5),
                                KeyCode::Key6 => Some(6),
                                KeyCode::Key7 => Some(7),
                                KeyCode::Key8 => Some(8),
                                KeyCode::Key9 => Some(9),
                                _ => None,
                            };
                            if let Some(mode) = digit
                                .and_then(rebellion_render::GidMode::from_accelerator_digit)
                                .filter(|mode| cockpit_state.select_gid_from_keyboard(*mode))
                            {
                                #[cfg(all(
                                    target_arch = "wasm32",
                                    feature = "interface-test-fixtures"
                                ))]
                                pending_windowing_route_receipts.push((event.id, "key", "gid"));
                                macroquad::logging::info!(
                                    "[interface] command=0x{:x} destination=gid status=selected label={} source=accelerator",
                                    mode.command_id(),
                                    mode.label()
                                );
                                #[cfg(all(
                                    target_arch = "wasm32",
                                    feature = "interface-test-fixtures"
                                ))]
                                if let Some(request) = interface_fixture_request {
                                    interface_test_fixture::emit_selected(
                                        request,
                                        cockpit_state.gid_mode,
                                    );
                                }
                            }
                        } else if let Some(index) = panel_index {
                            toggle_exclusive_panel(
                                &mut [&mut show_fleets, &mut show_missions],
                                index,
                            );
                        } else if event.key == KeyCode::S {
                            save_slots = read_save_slots(&saves_dir);
                            game_options_state.set_origin(GameOptionsOrigin::CommandCenter);
                            game_options_state.refresh_saves(&save_slots);
                            game_mode = GameMode::GameOptions {
                                origin: GameOptionsOrigin::CommandCenter,
                            };
                            strategic_dispatch_restricted = true;
                            strategic_input_router.set_application_hold(true);
                        } else if event.key == KeyCode::Tab {
                            mod_manager_state.open = !mod_manager_state.open;
                        } else if event.key == KeyCode::GraveAccent {
                            #[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
                            {
                                command_palette_state.toggle();
                                if command_palette_state.open {
                                    command_palette_state.refresh_interface(&world);
                                }
                            }
                        }
                    }
                }

                // Pointer root dispatch follows the completed egui child pass.
                // The map update is input-only; all strategic painting starts
                // after keyboard and pointer ownership have been resolved.
                map_state.viewport = Some((
                    cockpit_vp.x,
                    cockpit_vp.y,
                    cockpit_vp.width,
                    cockpit_vp.height,
                ));
                map_state.display_scale = cockpit_layout.scale;
                map_state.pointer_blocked |= egui_claims_pointer || strategic_dispatch_restricted;
                update_galaxy_map_input(&world, &mut map_state, cockpit_state.faction);
                if let Some(system) = map_state.activated_system {
                    sector_window_state.open_for_system(&world, system, cockpit_state.faction);
                }
                #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
                {
                    let underlay_action_count = usize::from(
                        map_state.selected_system != windowing_map_selection_before
                            || map_state.activated_system.is_some(),
                    );
                    if underlay_action_count > 0 {
                        macroquad::logging::info!(
                            "[windowing-underlay] surface=galaxy-map action=selection"
                        );
                    }
                    for (event, kind, application) in pending_windowing_route_receipts.drain(..) {
                        emit_windowing_route_receipt(
                            &strategic_input_router,
                            event,
                            kind,
                            application,
                            underlay_action_count,
                        );
                    }
                }
                #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
                if let Some(request) = interface_fixture_request {
                    if request.scenario == interface_test_fixture::Scenario::Hover {
                        interface_test_fixture::emit_hover(request, &world, &map_state);
                    }
                }

                let _ = draw_cockpit_chrome(&cockpit_state);
                // The whole 607x437 starfield lies behind the shell at its
                // FUN_00427010 offset. The map is clipped to its aperture;
                // egui then paints the shell and child windows above it.
                set_cockpit_viewport_clip(Some(cockpit_layout.canvas));
                draw_galaxy_backdrop(
                    cockpit_layout,
                    cockpit_state.faction,
                    &mut bmp_cache,
                    cockpit_state.gid_mode,
                );
                set_cockpit_viewport_clip(Some(cockpit_vp));
                draw_galaxy_map(
                    &world,
                    &map_state,
                    &mut bmp_cache,
                    cockpit_state.gid_mode,
                    cockpit_state.faction,
                    &rebellion_render::GidOverlayContext {
                        movement: &movement_state,
                        manufacturing: &mfg_state,
                        economy: &economy_state,
                        missions: &mission_state,
                        uprisings: &uprising_state,
                        highlight: &cockpit_state.gid_highlight.systems,
                    },
                );
                set_cockpit_viewport_clip(None);
                egui_macroquad::draw();
            }

            // Drawn after the match, over the command center when the
            // Battle Alert is up.
            GameMode::TacticalCombat => {}
        }

        if tactical_frame {
            // Under the Battle Alert or the results, the command center pass
            // above has drawn the droids; the window is all that is left.
            tactical_state.set_frame_key_downs(strategic_raw_input_events.key_downs.iter().copied());
            let tac_action = draw_tactical_view(
                &mut tactical_state,
                &mut bmp_cache,
                &world,
                &troop_transport_state,
            );
            let requested_result_system = match &tac_action {
                TacticalAction::OpenBattleSystem(system) => Some(*system),
                _ => None,
            };
            let requested_result_fleet = match &tac_action {
                TacticalAction::OpenBattleFleet(fleet) => Some(*fleet),
                _ => None,
            };

            match tac_action {
                TacticalAction::BeginCombat => {
                    // Advance from placement to combat phase.
                    if let Some(ref mut session) = tactical_state.session {
                        session.phase = rebellion_render::BattlePhase::Combat;
                        session.arm_docked_hangars();
                        session.queue_battle_ready_voice();
                    }
                }
                TacticalAction::AutoResolve => {
                    // Simulate runs the tactical engine to its end without its
                    // window (FUN_0040a050 state 8), then shows the same
                    // Battle Results window as a played engagement.
                    let mut result_presented = false;
                    if tactical_state.simulate_battle(&world) {
                        let session = tactical_state
                            .session
                            .clone()
                            .expect("simulate_battle keeps the session");
                        let battle_return = tactical_flow::apply_results(
                            &session,
                            &mut world,
                            &mut troop_transport_state,
                            &mut movement_state,
                            &ProductRoom::new(&mfg_state, &delivery_state),
                        );
                        if let (Some(destination), Some(shown)) =
                            (battle_return.withdrawal, tactical_state.session.as_mut())
                        {
                            shown.withdrawal_destination = world
                                .systems
                                .get(destination)
                                .map(|system| system.name.clone());
                        }

                        let alliance_won = match battle_return.winner {
                            Some(rebellion_render::CombatWinner::Attacker) => {
                                Some(session.attacker_is_alliance)
                            }
                            Some(rebellion_render::CombatWinner::Defender) => {
                                Some(!session.attacker_is_alliance)
                            }
                            Some(rebellion_render::CombatWinner::Draw) | None => None,
                        };
                        let winner_str = match alliance_won {
                            Some(true) => "Alliance victory",
                            Some(false) => "Empire victory",
                            None => "Draw",
                        };
                        msg_log.push(GameMessage::at_system(
                            session.start_tick,
                            format!(
                                "Space battle at {} — {} (simulated)",
                                session.system_name, winner_str
                            ),
                            MessageCategory::Combat,
                            session.system,
                        ));

                        // FUN_00517270: while both sides remain there is no
                        // result; the battle fires again on the next pass.
                        let both_remain = alliance_won.is_none()
                            && side_present(&world, session.system, true)
                            && side_present(&world, session.system, false);
                        if both_remain {
                            tactical_state.end_battle();
                            macroquad::logging::info!(
                                "[battle] both sides remain at {}; no result, fights again next pass",
                                session.system_name
                            );
                        } else {
                            result_presented = true;
                        }
                    }
                    if !result_presented {
                        game_mode = GameMode::Galaxy;
                    }
                }
                TacticalAction::ReturnToGalaxy
                | TacticalAction::OpenBattleSystem(_)
                | TacticalAction::OpenBattleFleet(_) => {
                    let strategic_results_applied = tactical_state.strategic_results_applied();
                    // Apply combat results from tactical session to GameWorld.
                    if let Some(session) = tactical_state.end_battle() {
                        let battle_return = if strategic_results_applied {
                            tactical_flow::summarize_results(&session)
                        } else {
                            let before = tactical_flow::persistence_snapshot(&session, &world);
                            let result = tactical_flow::apply_results(
                                &session,
                                &mut world,
                                &mut troop_transport_state,
                                &mut movement_state,
                                &ProductRoom::new(&mfg_state, &delivery_state),
                            );
                            let after = tactical_flow::persistence_snapshot(&session, &world);
                            macroquad::logging::info!(
                                "[tactical_results] strategic_persistence applied=true attacker_present={}->{} defender_present={}->{} attacker_capitals={}->{} defender_capitals={}->{} attacker_fighters={}->{} defender_fighters={}->{} attacker_death_star={}->{} defender_death_star={}->{}",
                                before.attacker_present,
                                after.attacker_present,
                                before.defender_present,
                                after.defender_present,
                                before.attacker_capital_ships,
                                after.attacker_capital_ships,
                                before.defender_capital_ships,
                                after.defender_capital_ships,
                                before.attacker_fighter_squadrons,
                                after.attacker_fighter_squadrons,
                                before.defender_fighter_squadrons,
                                after.defender_fighter_squadrons,
                                before.attacker_has_death_star,
                                after.attacker_has_death_star,
                                before.defender_has_death_star,
                                after.defender_has_death_star,
                            );
                            result
                        };
                        if !strategic_results_applied {
                            let winner_str = match battle_return.winner {
                                Some(rebellion_render::CombatWinner::Attacker) => {
                                    "Attacker victory"
                                }
                                Some(rebellion_render::CombatWinner::Defender) => {
                                    "Defender victory"
                                }
                                Some(rebellion_render::CombatWinner::Draw) | None => "Draw",
                            };
                            msg_log.push(GameMessage::at_system(
                                session.start_tick,
                                format!(
                                    "Space battle at {} — {} (tactical)",
                                    session.system_name, winner_str
                                ),
                                MessageCategory::Combat,
                                session.system,
                            ));
                        }

                        // No landing follows a battle: regiments go down only
                        // by the assault order (ghidra/notes/ground-assault.md).
                        game_mode = GameMode::Galaxy;
                    } else {
                        game_mode = GameMode::Galaxy;
                    }

                    if game_mode == GameMode::Galaxy {
                        if let Some(system) = requested_result_system {
                            map_state.selected_system = Some(system);
                            let layout = cockpit_state.layout();
                            let _ = system_window_state.open(
                                &world,
                                system,
                                (85, 55),
                                cockpit_state.faction,
                                layout,
                            );
                        }
                        // port: the fleet's Fleet window with the fleet
                        // selected, as the Fleet Finder opens it
                        // (FUN_00429440).
                        if let Some(fleet) = requested_result_fleet {
                            if let Some(value) = world.fleets.get(fleet) {
                                let system = value.location;
                                map_state.selected_system = Some(system);
                                let layout = cockpit_state.layout();
                                enqueue_fleet_window(
                                    &mut strategic_windowing,
                                    &fleet_window_state,
                                    &world,
                                    system,
                                    (85, 55),
                                    cockpit_state.faction,
                                    layout,
                                    Some(FleetWindowEntry::Fleet(fleet)),
                                );
                            }
                        }
                    }
                }
                TacticalAction::TogglePause => {
                    if let Some(ref mut session) = tactical_state.session {
                        session.paused = !session.paused;
                    }
                }
                TacticalAction::SetSpeed(speed) => {
                    if let Some(ref mut session) = tactical_state.session {
                        session.combat_speed = speed.clamp(1, 4);
                    }
                }
                // FUN_0059fd40 -> FUN_005ad530: the player's side withdraws
                // through its controller's countdown and the Withdraw order.
                TacticalAction::WithdrawFromBattle => tactical_state.withdraw_player_side(),
                TacticalAction::RetreatBeforeBattle => {
                    if let Some(ref mut session) = tactical_state.session {
                        let player_side = session.player_is_attacker;
                        for ship in &mut session.ships {
                            if ship.alive
                                && ship.is_attacker == player_side
                                && ship.subsystem_capacity.hyperdrive > 0
                                && ship.subsystem_condition.hyperdrive > 0
                            {
                                ship.retreating = true;
                                ship.selected = false;
                            }
                        }
                        session.selected_ship = None;
                        session.selected_fighter_group = None;
                        session.selected_task_force = None;
                        session.paused = false;
                    }
                }
                TacticalAction::OpenGameOptions => {
                    save_slots = read_save_slots(&saves_dir);
                    game_options_state.set_origin(GameOptionsOrigin::TacticalBattle);
                    game_options_state.refresh_saves(&save_slots);
                    game_mode = GameMode::GameOptions {
                        origin: GameOptionsOrigin::TacticalBattle,
                    };
                    macroquad::logging::info!(
                        "[interface] command=0x133 destination=game_options status=opened_original"
                    );
                }
                TacticalAction::None => {}
            }

            if game_mode == GameMode::TacticalCombat {
                if let Some(outcome) = tactical_state.take_pending_trench_run_cinematic() {
                    let kind = CutsceneKind::TrenchRun(outcome);
                    cutscene_player = open_movie(
                        kind.movie(),
                        &audio_vol,
                        #[cfg(not(target_arch = "wasm32"))]
                        &mut audio_engine,
                    );
                    if cutscene_player.is_some() {
                        game_mode = GameMode::Cutscene { kind };
                    }
                }
            }
        }

        // 5. Apply panel actions
        for action in panel_actions {
            match action {
                PanelAction::SaveGame { slot, name } => {
                    let state = LiveCampaign {
                        world: &mut world,
                        clock: &mut clock,
                        manufacturing: &mut mfg_state,
                        missions: &mut mission_state,
                        events: &mut event_state,
                        ai: &mut ai_state,
                        movement: &mut movement_state,
                        fog_alliance: &mut fog_alliance_state,
                        fog_empire: &mut fog_empire_state,
                        player_faction: &mut player_faction,
                        blockade: &mut blockade_state,
                        uprising: &mut uprising_state,
                        research: &mut research_state,
                        jedi: &mut jedi_state,
                        victory: &mut victory_state,
                        betrayal: &mut betrayal_state,
                        economy: &mut economy_state,
                        sim_rng: &mut sim_rng,
                        ai2: &mut secondary_ai_state,
                        repair: &mut repair_state,
                        troop_transport: &mut troop_transport_state,
                        deliveries: &mut delivery_state,
                        player_agent: &mut player_agent,
                        stockpiles: &mut stockpile_state,
                        game_config: &mut game_config,
                        campaign_config: &mut campaign_config,
                    }
                    .snapshot();
                    let interface = saved_interface(
                        &world,
                        &cockpit_state,
                        &sector_window_state,
                        &system_window_state,
                        &fleet_window_state,
                        &defenses_window_state,
                        &missions_window_state,
                    );
                    let active_mods = mod_runtime.enabled_mod_list();
                    // A `Save:` names its file; it opens no slot and logs nothing.
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Some(path) = dev_save_file.take() {
                        match rebellion_data::save::save_file_with_interface(
                            &path,
                            &name,
                            &state,
                            &interface,
                            &active_mods,
                        ) {
                            Ok(fingerprint) => {
                                dev_channel.set_path(&path);
                                dev_channel.note(&format!(
                                    "tick {} fingerprint {fingerprint}",
                                    state.clock.tick
                                ));
                            }
                            Err(error) => dev_channel.refuse(&format!("{error:#}")),
                        }
                        continue;
                    }
                    match rebellion_data::save::save_slot_with_interface(
                        &saves_dir,
                        slot,
                        &name,
                        &state,
                        &interface,
                        &active_mods,
                    ) {
                        Ok(fingerprint) => {
                            macroquad::logging::info!(
                                "save_state_fingerprint slot={} tick={} fingerprint={}",
                                slot,
                                state.clock.tick,
                                fingerprint
                            );
                            save_slots = read_save_slots(&saves_dir);
                            game_options_state.refresh_saves(&save_slots);
                            game_options_state.save_result(slot, true);
                            game_options_state.error = None;
                            msg_log.push(GameMessage::new(
                                clock.tick,
                                format!("Saved game to slot {}", slot + 1),
                                MessageCategory::Event,
                            ));
                        }
                        Err(error) => {
                            game_options_state.save_result(slot, false);
                            game_options_state.error = Some(error.to_string());
                        }
                    }
                }
                PanelAction::LoadGame { slot } => {
                    // A `Load:` names its file in place of a slot.
                    #[cfg(not(target_arch = "wasm32"))]
                    let dev_loaded = dev_load_file.take();
                    #[cfg(not(target_arch = "wasm32"))]
                    let loaded = match &dev_loaded {
                        Some(path) => rebellion_data::save::load_file_with_interface(path, slot),
                        None => rebellion_data::save::load_slot_with_interface(&saves_dir, slot),
                    };
                    #[cfg(target_arch = "wasm32")]
                    let loaded = rebellion_data::save::load_slot_with_interface(&saves_dir, slot);
                    match loaded {
                        Ok((meta, state, interface)) => {
                            macroquad::logging::info!(
                                "load_state_fingerprint slot={} tick={} fingerprint={}",
                                slot,
                                meta.game_tick,
                                meta.state_fingerprint
                            );
                            let mut campaign = LiveCampaign {
                                world: &mut world,
                                clock: &mut clock,
                                manufacturing: &mut mfg_state,
                                missions: &mut mission_state,
                                events: &mut event_state,
                                ai: &mut ai_state,
                                movement: &mut movement_state,
                                fog_alliance: &mut fog_alliance_state,
                                fog_empire: &mut fog_empire_state,
                                player_faction: &mut player_faction,
                                blockade: &mut blockade_state,
                                uprising: &mut uprising_state,
                                research: &mut research_state,
                                jedi: &mut jedi_state,
                                victory: &mut victory_state,
                                betrayal: &mut betrayal_state,
                                economy: &mut economy_state,
                                sim_rng: &mut sim_rng,
                                ai2: &mut secondary_ai_state,
                                repair: &mut repair_state,
                                troop_transport: &mut troop_transport_state,
                                deliveries: &mut delivery_state,
                                player_agent: &mut player_agent,
                                stockpiles: &mut stockpile_state,
                                game_config: &mut game_config,
                                campaign_config: &mut campaign_config,
                            };
                            campaign.restore(state);
                            drop(strategic_windowing.reset_session());
                            strategic_input_router.reset_session();
                            let restored_fingerprint =
                                rebellion_data::save::compute_state_fingerprint(
                                    &campaign.snapshot(),
                                );
                            // The battles left for a later pass are the
                            // contested systems the save holds (the combat
                            // triggers, FUN_00509710 / FUN_0050b8e0).
                            held_battles = contested_systems(&world);
                            game_speed_ui = GameSpeedUiState::default();
                            macroquad::logging::info!(
                                "[campaign] loaded configuration={}",
                                campaign_config.summary()
                            );
                            warmed_galaxy_font_sizes.clear();

                            cockpit_state.faction = if player_faction == MissionFaction::Alliance {
                                CockpitFaction::Alliance
                            } else {
                                CockpitFaction::Empire
                            };
                            cockpit_state.gid_mode = saved_gid_mode(&interface);
                            cockpit_state.gid_highlight = rebellion_render::GidHighlight::default();
                            resume_saved_advisor(
                                &mut advisor_state,
                                AdvisorFaction::from(cockpit_state.faction),
                                victory_state.word,
                            );
                            movie_queue = movies::MovieQueue::default();
                            advice_topics = rebellion_render::advice_topics::AdviceTopics::new(
                                AdvisorFaction::from(cockpit_state.faction),
                                |id| text_templates.get(id),
                            );
                            map_state = GalaxyMapState::default();
                            sector_window_state.clear();
                            system_window_state.clear();
                            fleet_window_state.clear();
                            defenses_window_state.clear();
                            missions_window_state.clear();
                            galaxy_focus = GalaxyFocusState::default();
                            // The loaded game's view builds its own GID menu.
                            cockpit_state.gid_ui.close_menu();
                            cockpit_state.gid_ui.built = false;
                            restore_interface(
                                &interface.windows,
                                &world,
                                if player_faction == MissionFaction::Alliance {
                                    &fog_alliance_state
                                } else {
                                    &fog_empire_state
                                },
                                &mission_state,
                                cockpit_state.faction,
                                cockpit_state.layout(),
                                RestoredWindows {
                                    sectors: &mut sector_window_state,
                                    systems: &mut system_window_state,
                                    fleets: &fleet_window_state,
                                    defenses: &mut defenses_window_state,
                                    missions: &mut missions_window_state,
                                },
                                &mut strategic_windowing,
                            );
                            pending_window_restore = Some(interface.windows);
                            object_menu = None;
                            targeting = None;
                            fleets_state = FleetsState::default();
                            build_selection_state = BuildSelectionState::default();
                            mission_dialog_state = MissionDialogState::default();
                            move_confirmation_state = MoveConfirmationState::default();
                            scrap_confirmation_state = ScrapConfirmationState::default();
                            stop_confirmation_state = StopConfirmationState::default();
                            status_window_state = StatusWindowState::default();
                            fleet_finder_state = FleetFinderState::default();
                            troop_finder_state = TroopFinderState::default();
                            personnel_finder_state = PersonnelFinderState::default();
                            operation_report_state = OperationReportState::default();
                            message_index = None;
                            advice_saved_speed = None;
                            encyclopedia_surface.close();
                            show_fleets = false;
                            show_missions = false;
                            tactical_state = TacticalState::new();
                            dual_ai_mode = secondary_ai_state.is_some();
                            msg_log = MessageLog::default();
                            // Mission ids and message ids start over with the save.
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                dev_missions_resolved.clear();
                                dev_wait_baseline = 0;
                            }
                            msg_log.push(GameMessage::new(
                                clock.tick,
                                format!("Loaded save ‘{}’ from slot {}", meta.name, slot + 1),
                                MessageCategory::Event,
                            ));
                            game_mode = GameMode::Galaxy;
                            if let Ok(fingerprint) = restored_fingerprint {
                                macroquad::logging::info!(
                                    "[campaign] restored slot={} mode=Galaxy tick={} fingerprint={} faction={:?}",
                                    slot, clock.tick, fingerprint, player_faction
                                );
                            }
                            #[cfg(not(target_arch = "wasm32"))]
                            if let Some(path) = &dev_loaded {
                                dev_channel.set_path(path);
                                dev_channel.note(&format!(
                                    "tick {} fingerprint {}",
                                    clock.tick, meta.state_fingerprint
                                ));
                            }
                        }
                        Err(error) => {
                            #[cfg(not(target_arch = "wasm32"))]
                            if dev_loaded.is_some() {
                                dev_channel.refuse(&format!("{error:#}"));
                            }
                            game_options_state.error = Some(error.to_string());
                        }
                    }
                }
                PanelAction::SetGameSpeed(speed) => choose_game_speed(&mut clock, speed),
                PanelAction::Bombard { fleets, kind } => {
                    let result = strike_orders::issue(
                        &mut world,
                        &fleets,
                        kind,
                        &mut rebellion_data::integrator::StrikeStates {
                            movement: &mut movement_state,
                            manufacturing: &mut mfg_state,
                            transport: &mut troop_transport_state,
                            deliveries: &mut delivery_state,
                            missions: &mut mission_state,
                            uprisings: &uprising_state,
                            blockade: &mut blockade_state,
                            tick: clock.tick,
                            // hyp: the order's command runs in the next step's phase 5,
                            // before that step's AI round (FUN_0051df30, FUN_004361a0).
                            step: ai_state.scheduler.next_step(),
                        },
                        &mut || sim_rng.gen::<f64>(),
                        &text_templates,
                        &mut msg_log,
                    );
                    if let Ok(runs) = &result {
                        movie_queue.file(strike_orders::strike_movies(
                            &world,
                            &text_templates,
                            runs,
                            Faction::of_alliance(player_faction == MissionFaction::Alliance),
                        ));
                    }
                    macroquad::logging::info!(
                        "[interface] command={:#x} destination=bombardment status={}",
                        kind.order(),
                        // No run: every fleet was in transit and kept no
                        // flag; an idle run: both sides flagged there.
                        match &result {
                            Ok(runs) if runs.is_empty() => "refused reason=InTransit".to_owned(),
                            Ok(runs) if runs.iter().all(|strike| strike.report.is_none()) => {
                                format!("idle runs={}", runs.len())
                            }
                            Ok(runs) => format!(
                                "issued runs={} hits={}",
                                runs.len(),
                                runs.iter()
                                    .filter_map(|strike| strike.report.as_ref())
                                    .map(|report| report.hits.to_string())
                                    .collect::<Vec<_>>()
                                    .join(",")
                            ),
                            Err(refusal) => format!("refused reason={refusal:?}"),
                        }
                    );
                }
                PanelAction::Assault { fleets } => {
                    let result = assault_orders::issue(
                        &mut world,
                        &fleets,
                        Faction::of_alliance(player_faction == MissionFaction::Alliance),
                        &mut rebellion_data::integrator::StrikeStates {
                            movement: &mut movement_state,
                            manufacturing: &mut mfg_state,
                            transport: &mut troop_transport_state,
                            deliveries: &mut delivery_state,
                            missions: &mut mission_state,
                            uprisings: &uprising_state,
                            blockade: &mut blockade_state,
                            tick: clock.tick,
                            // hyp: the order's command runs in the next step's phase 5,
                            // before that step's AI round (FUN_0051df30, FUN_004361a0).
                            step: ai_state.scheduler.next_step(),
                        },
                        &mut || sim_rng.gen::<f64>(),
                        &text_templates,
                        &mut msg_log,
                    );
                    macroquad::logging::info!(
                        "[interface] command=0x234 destination=assault status={}",
                        match &result {
                            Ok(runs) => format!("issued runs={}", runs.len()),
                            Err(refusal) => format!("refused reason={refusal:?}"),
                        }
                    );
                }
                action => {
                    let active_fog_state = if player_faction == MissionFaction::Alliance {
                        &mut fog_alliance_state
                    } else {
                        &mut fog_empire_state
                    };
                    apply_panel_action(
                        action,
                        &mut world,
                        &mut map_state,
                        &mut mfg_state,
                        &delivery_state,
                        &mut mission_state,
                        &mut movement_state,
                        &mut troop_transport_state,
                        active_fog_state,
                        &mut ai_state,
                        &mut msg_log,
                        &mut advisor_state,
                        &mut player_faction,
                        &mut clock,
                        &mut dual_ai_mode,
                        &mut secondary_ai_state,
                        &mut victory_state,
                        campaign_config,
                        &mut blockade_state,
                        &uprising_state,
                        &event_state,
                        &mut mod_runtime,
                        #[cfg(not(target_arch = "wasm32"))]
                        encyclopedia_base_input.as_ref(),
                        #[cfg(not(target_arch = "wasm32"))]
                        &mut encyclopedia_session_store,
                        #[cfg(not(target_arch = "wasm32"))]
                        &mut audio_engine,
                        #[cfg(not(target_arch = "wasm32"))]
                        &audio_vol,
                        #[cfg(not(target_arch = "wasm32"))]
                        &sounds_dir,
                    );
                    // A player's order runs in the next step's phase 5,
                    // which serves its regiments' holder reruns
                    // (FUN_00508660, FUN_0051df30).
                    let step = ai_state.scheduler.next_step();
                    fire_support_events(
                        &mut world,
                        &mut mfg_state,
                        &mut troop_transport_state,
                        &movement_state,
                        &mut delivery_state,
                        &text_templates,
                        &mut msg_log,
                        clock.tick,
                        step,
                    );
                    sweep_flagged(
                        &mut world,
                        &mut mission_state,
                        &troop_transport_state,
                        &movement_state,
                        &blockade_state,
                        clock.tick,
                    );
                }
            }
        }

        if quit_requested {
            #[cfg(not(target_arch = "wasm32"))]
            audio_engine.stop_music();
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(engine) = browser_menu_audio.as_mut() {
                    engine.stop_music();
                }
                web_accessibility::sync_menu(false, &main_menu_state, false);
                web_accessibility::show_quit();
            }
            macroquad::logging::info!("[quit] audio_stopped=true cleanup=complete");
            break;
        }

        // 6. Music (ghidra/notes/audio-dispatch.md, "Music"), then the
        // tactical event cues. The Battle Alert forces its side's track
        // (FUN_0044f860); each of its three commands stops the music
        // (FUN_00450540), so combat plays none; the result window forces its
        // track on first paint (FUN_00471200) and its close stops it and
        // picks again (FUN_004727a0).
        let alert_open =
            game_mode == GameMode::TacticalCombat && tactical_state.battle_alert_open();
        #[cfg(not(target_arch = "wasm32"))]
        {
            let player_alliance = player_faction == MissionFaction::Alliance;
            let result_track = if game_mode == GameMode::TacticalCombat {
                tactical_state.result_music()
            } else {
                None
            };
            let now = get_time();
            let mut roll = |n: u32| macroquad::rand::gen_range(0, n.max(1));
            if alert_open && !music_alert_open {
                audio_engine.force_track(
                    MdataTrack::battle_alert(player_alliance),
                    &mdata_dir,
                    &audio_vol,
                    now,
                );
            } else if !alert_open && music_alert_open {
                audio_engine.stop_music();
            }
            if let (Some(track), None) = (result_track, music_result) {
                audio_engine.force_track(track, &mdata_dir, &audio_vol, now);
            } else if result_track.is_none() && music_result.is_some() {
                audio_engine.stop_music();
                if music_scene(&game_mode) == Some(MusicScene::Play) {
                    let track = music_selector.default_pick(true, &mut roll);
                    audio_engine.start_track(track, &mdata_dir, &audio_vol, now);
                }
            }
            let scene = music_scene(&game_mode).or(music_scene_now);
            if scene != music_scene_now {
                match scene {
                    Some(MusicScene::Menu) => {
                        let track = music_selector.default_pick(false, &mut roll);
                        audio_engine.start_track(track, &mdata_dir, &audio_vol, now);
                    }
                    Some(MusicScene::Play) => {
                        // FUN_00439320: a game's advice agent starts the
                        // counter at 0.
                        if music_scene_now != Some(MusicScene::Movie) {
                            music_selector.reset();
                        }
                        let track = music_selector.default_pick(true, &mut roll);
                        audio_engine.start_track(track, &mdata_dir, &audio_vol, now);
                    }
                    Some(MusicScene::Movie) | None => {}
                }
                music_scene_now = scene;
            }
            if audio_engine.poll_track_end(&mdata_dir, &audio_vol, now) {
                let in_session = music_scene_now == Some(MusicScene::Play);
                let track = music_selector.default_pick(in_session, &mut roll);
                audio_engine.start_track(track, &mdata_dir, &audio_vol, now);
            }
            music_alert_open = alert_open;
            music_result = result_track;
        }
        let tactical_audio_context = game_mode == GameMode::TacticalCombat && !alert_open;

        if tactical_audio_context {
            for cue in tactical_state.take_pending_audio_cues() {
                #[cfg(not(target_arch = "wasm32"))]
                let loaded = audio_engine.play_tactical_sfx(cue.resource_id, &audio_vol);
                #[cfg(target_arch = "wasm32")]
                let loaded = browser_menu_audio
                    .as_mut()
                    .is_some_and(|engine| engine.play_tactical_sfx(cue.resource_id, &audio_vol));
                macroquad::logging::info!(
                    "[audio] context=combat event=0x{:02x} wave={} routed=true loaded={} muted={}",
                    cue.event.event_id(),
                    cue.resource_id,
                    loaded,
                    audio_vol.muted
                );
            }
            for cue in tactical_state.take_pending_voice_cues() {
                #[cfg(not(target_arch = "wasm32"))]
                let loaded =
                    audio_engine.play_tactical_voice(cue.faction, cue.resource_id, &audio_vol);
                #[cfg(target_arch = "wasm32")]
                let loaded = browser_menu_audio.as_mut().is_some_and(|engine| {
                    engine.play_tactical_voice(cue.faction, cue.resource_id, &audio_vol)
                });
                macroquad::logging::info!(
                    "[audio] context=combat voice_event={:?} source_event=0x{:x} wave={} faction={:?} routed=true loaded={} muted={}",
                    cue.event,
                    cue.source_event,
                    cue.resource_id,
                    cue.faction,
                    loaded,
                    audio_vol.muted
                );
            }
        }

        #[cfg(target_arch = "wasm32")]
        if browser_menu_audio_requested {
            if let Some(engine) = browser_menu_audio.as_mut() {
                engine.try_play_loaded_music(&audio_vol);
            }
        }

        // Apply audio volume changes on both native and WebAudio backends.
        if audio_vol.dirty {
            #[cfg(not(target_arch = "wasm32"))]
            audio_engine.apply_volume(&audio_vol);
            #[cfg(target_arch = "wasm32")]
            if let Some(engine) = browser_menu_audio.as_mut() {
                engine.apply_volume(&audio_vol);
            }
            macroquad::logging::info!(
                "[audio] music_volume={:.2} muted={} music_muted={}",
                audio_vol.effective_music_volume(),
                audio_vol.muted,
                audio_vol.music_muted
            );
            audio_vol.dirty = false;
        }

        #[cfg(target_arch = "wasm32")]
        {
            if game_mode != GameMode::MainMenu {
                main_menu_state.set_semantic_focus(None);
            }
            web_accessibility::sync_menu(
                game_mode == GameMode::MainMenu,
                &main_menu_state,
                audio_vol.music_enabled(),
            );
        }

        // 7. Handle focus requests from message log + encyclopedia
        if let Some(focus_key) = log_state.focus_system.take() {
            // The map does not pan (galaxy_camera): focusing only selects.
            if world.systems.contains_key(focus_key) {
                map_state.selected_system = Some(focus_key);
            }
        }

        #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
        if let Some(request) = interface_fixture_request {
            if game_mode == GameMode::Galaxy && !interface_fixture_emitted {
                interface_fixture_frames += 1;
                if interface_fixture_frames >= 3 {
                    interface_test_fixture::emit_ready(request, &world, &map_state);
                    interface_test_fixture::emit_fleet_move_setup(
                        request,
                        &world,
                        &sector_window_state,
                        &fleet_window_state,
                    );
                    interface_test_fixture::emit_fleet_load_setup(
                        request,
                        &world,
                        &sector_window_state,
                        &defenses_window_state,
                    );
                    interface_test_fixture::emit_quadrant_setup(
                        request,
                        &world,
                        &sector_window_state,
                    );
                    interface_fixture_emitted = true;
                }
            } else if game_mode == GameMode::Galaxy {
                fleet_move_watch.observe(
                    request,
                    &world,
                    &movement_state,
                    move_confirmation_state.is_open(),
                    &map_state,
                    msg_log.messages(),
                );
                fleet_load_watch.observe(
                    request,
                    &world,
                    if player_faction == MissionFaction::Alliance {
                        &fog_alliance_state
                    } else {
                        &fog_empire_state
                    },
                    &movement_state,
                    &troop_transport_state,
                    &fleet_window_state,
                    move_confirmation_state.is_open(),
                    msg_log.messages(),
                );
                fleet_finder_watch.observe(
                    request,
                    &world,
                    &interface_test_fixture::FinderWindows {
                        movement: &movement_state,
                        fog: if player_faction == MissionFaction::Alliance {
                            &fog_alliance_state
                        } else {
                            &fog_empire_state
                        },
                        finder: &fleet_finder_state,
                        fleets: &fleet_window_state,
                        sectors: &sector_window_state,
                        map: &map_state,
                        left_panel_open: show_fleets || show_missions,
                    },
                );
                quadrant_watch.observe(
                    request,
                    &world,
                    &interface_test_fixture::QuadrantWindows {
                        systems: &system_window_state,
                        defenses: &defenses_window_state,
                        missions_windows: &missions_window_state,
                        fog: if player_faction == MissionFaction::Alliance {
                            &fog_alliance_state
                        } else {
                            &fog_empire_state
                        },
                        missions: &mission_state,
                        economy: &economy_state,
                    },
                );
            }
        }
        #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
        if let Some(request) = tactical_fixture_request {
            if game_mode == GameMode::TacticalCombat && !interface_fixture_emitted {
                interface_fixture_frames += 1;
                if interface_fixture_frames >= 3 {
                    tactical_test_fixture::emit_ready(request, &tactical_state);
                    interface_fixture_emitted = true;
                }
            }
        }

        if targeting_cursor_drawn != system_cursor_hidden {
            show_mouse(!targeting_cursor_drawn);
            system_cursor_hidden = targeting_cursor_drawn;
        }

        // A capture saves this frame; a command's result is written once
        // its frame (or its wait) is over.
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(before) = dev_mission_check.take() {
                if mission_state.len() > before {
                    dev_channel.note("dispatched");
                } else {
                    let why = dev_actions::LAST_DISPATCH_REFUSAL
                        .lock()
                        .ok()
                        .and_then(|mut last| last.take())
                        .unwrap_or_else(|| "no reason recorded".to_owned());
                    dev_channel.refuse(&format!("the dispatch was refused: {why}"));
                }
            }
            if let Some(name) = dev_capture.take() {
                match dev_channel::capture_frame(dev_channel.evidence(), &name) {
                    Ok((path, width, height)) => {
                        eprintln!("[capture] {} {width}x{height}", path.display());
                        dev_channel.set_path(&path);
                        dev_channel.note(&format!("{width}x{height}"));
                    }
                    Err(why) => dev_channel.refuse(&why),
                }
            }
            if dev_hold_frames == 0
                && get_time() >= dev_hold_until
                && dev_wait_until.is_none()
                && !dev_input.busy()
            {
                dev_channel.finish();
            }
        }

        let fleet_content_reports =
            fleet_window_state.content_reports(strategic_frame_boundary.snapshot(), &world);

        // Window intents commit only after input and painting have completed.
        // Fleet content is published only after the matching manager commit;
        // close and eviction receipts remove only their exact identities.
        for applied in strategic_windowing.apply_pending() {
            let strategic_windowing::AppliedWindowCommand {
                command,
                result,
                mut removed_content,
                prepared_fleet,
                fleet_content_id,
                minimized_fleet,
                restored_fleet,
            } = applied;
            let activated_fleet = matches!(
                command,
                rebellion_windowing::WindowCommand::Activate(id)
                    if fleet_window_state.contains_window(id)
            );
            let mut committed_fleet_open = false;
            if let (Some(prepared), Some(id), Ok(commit)) =
                (prepared_fleet, fleet_content_id, &result)
            {
                if fleet_window_state.commit_open(prepared, &commit.transition) == Some(id) {
                    committed_fleet_open = strategic_windowing.has_content(id)
                        || strategic_windowing.acknowledge_fleet_publication(id);
                    if !committed_fleet_open {
                        fleet_window_state.remove_content(&[id]);
                        removed_content.extend(strategic_windowing.rollback_fleet_publication(id));
                    }
                } else {
                    macroquad::logging::error!(
                        "[windowing] Fleet content commit did not match identity={:?}",
                        id
                    );
                    if matches!(
                        commit.transition,
                        rebellion_windowing::WindowTransition::Opened { .. }
                    ) {
                        removed_content.extend(strategic_windowing.rollback_fleet_publication(id));
                    }
                }
            }
            if (activated_fleet && result.is_ok()) || committed_fleet_open {
                fleet_legacy_bridge_serial = FleetWindowState::next_legacy_bridge_serial();
                let activated = match command {
                    rebellion_windowing::WindowCommand::Activate(id) => Some(id),
                    _ => fleet_content_id,
                };
                if let Some(id) = activated {
                    fleet_window_state.note_activation(id);
                }
            }
            if let Some(window) = restored_fleet {
                system_window_state.restore_fleet_window(window);
            }
            if let (Some((window, system, logical_position)), Ok(commit)) =
                (minimized_fleet, &result)
            {
                if matches!(
                    &commit.transition,
                    rebellion_windowing::WindowTransition::Minimized { .. }
                ) {
                    if let Some(evicted) =
                        system_window_state.minimize_fleet_window(window, system, logical_position)
                    {
                        strategic_input_router.cancel_window(evicted);
                        fleet_window_state.cancel_pointer_capture(evicted);
                        removed_content
                            .extend(strategic_windowing.rollback_fleet_publication(evicted));
                    }
                }
            }
            fleet_window_state.remove_content(&removed_content);

            match &result {
                Ok(commit) => {
                    for event in &commit.events {
                        if let rebellion_windowing::WindowEvent::CaptureCancelled {
                            window, ..
                        } = event
                        {
                            macroquad::logging::info!("[windowing] {:?}", event);
                            strategic_input_router.cancel_window(*window);
                            fleet_window_state.cancel_pointer_capture(*window);
                        }
                    }
                    if !removed_content.is_empty() {
                        macroquad::logging::info!(
                            "[windowing] released_content={:?}",
                            removed_content
                        );
                    }
                }
                Err(error) => {
                    macroquad::logging::warn!(
                        "[windowing] rejected command={:?} error={:?}",
                        command,
                        error
                    );
                }
            }
        }

        // Reports reconcile after command commit against the immutable frame
        // that actually routed and painted this content.
        let reconciliation = strategic_windowing
            .reconcile(strategic_frame_boundary.snapshot(), &fleet_content_reports);
        fleet_window_state.remove_content(&reconciliation.removed);
        for event in &reconciliation.events {
            if let rebellion_windowing::WindowEvent::CaptureCancelled { window, .. } = event {
                strategic_input_router.cancel_window(*window);
                fleet_window_state.cancel_pointer_capture(*window);
            }
        }
        if !reconciliation.invalid_reports.is_empty()
            || !reconciliation.blocking_unpainted.is_empty()
        {
            macroquad::logging::warn!(
                "[windowing] reconcile invalid={:?} blocking={:?}",
                reconciliation.invalid_reports,
                reconciliation.blocking_unpainted
            );
        }
        // The load's Fleet windows have committed: the saved order and the
        // focus come back (FUN_00440f80).
        if let Some(order) = pending_window_restore.take() {
            let hidden = finish_interface_restore(
                &order,
                &world,
                &mut system_window_state,
                &mut defenses_window_state,
                &mut missions_window_state,
                &mut fleet_window_state,
                &mut strategic_windowing,
                &mut fleet_legacy_bridge_serial,
            );
            let children = galaxy_children(
                &sector_window_state,
                &system_window_state,
                &defenses_window_state,
                &missions_window_state,
                &fleet_window_state,
                &strategic_windowing.begin_frame(),
            )
            .into_iter()
            .filter(|(_, child)| !hidden.contains(child))
            .collect();
            galaxy_focus = focus_after_load(children, &mut strategic_windowing);
        }
        next_frame().await;
    }
}

/// What a fleet move needs to read when it lands.
struct FleetMoveContext<'a> {
    world: &'a GameWorld,
    movement: &'a MovementState,
    blockaded: &'a HashSet<SystemKey>,
    faction: MissionFaction,
}

/// A team of fleets' Move (`0x201`) or Confirmed Move (`0x202`) with its
/// target set: `FUN_00486fb0` validates it (`FUN_00487740`; the move order's
/// validator `FUN_0053c1a0` runs every member's, so one refusal refuses all),
/// then asks first when `FUN_00487cc0` says so, listing one line per member
/// (`FUN_0053c2e0`), or departs. A move onto a fleet joins it
/// (`FUN_004ffc90`, `FUN_004feca0`).
fn issue_fleet_move(
    context: &FleetMoveContext<'_>,
    fleets: &[FleetKey],
    confirmed: bool,
    target: ReleaseTarget,
    move_confirmation_state: &mut MoveConfirmationState,
    panel_actions: &mut Vec<PanelAction>,
) -> bool {
    let world = context.world;
    let is_alliance = context.faction == MissionFaction::Alliance;
    let (destination, join) = match target {
        ReleaseTarget::System(system) => (system, None),
        ReleaseTarget::Fleet { fleet, system } => (system, Some(fleet)),
    };
    // FUN_0053c4b0: a team with no object is refused 1/0x16 before any
    // sub-order runs; then every sub-order's validator (FUN_0053c1a0).
    let refusal = if fleets.is_empty() {
        Some(Some((1, 0x16)))
    } else if let Some(target) = join {
        fleets.iter().find_map(|fleet| {
            rebellion_core::fleet_join::validate_join(
                world,
                context.movement,
                &FleetMover::Fleet(*fleet),
                target,
                is_alliance,
            )
            .err()
            .map(|error| {
                macroquad::logging::info!("[order] fleet join refused: {:?}", error);
                fleet_order_refusal_status(error)
            })
        })
    } else {
        validate_fleets_dispatch(context.movement, world, fleets, destination, is_alliance)
            .err()
            .map(|error| {
                macroquad::logging::info!("[order] fleet move refused: {:?}", error);
                fleet_dispatch_refusal_status(error)
            })
    };
    if let Some(status) = refusal {
        // FUN_00487c90: no text; the side's advisor reacts to the status.
        if let Some(status) = status {
            panel_actions.push(PanelAction::AdvisorRefusal(status));
        }
        return false;
    }
    let asks = fleets.iter().any(|fleet| match join {
        Some(target) => rebellion_core::fleet_join::join_confirms(
            world,
            context.blockaded,
            *fleet,
            target,
            confirmed,
        ),
        None => fleet_move_confirms(world, context.blockaded, *fleet, confirmed),
    });
    if !asks {
        panel_actions.extend(
            fleets
                .iter()
                .map(|&fleet| mover_move_action(FleetMover::Fleet(fleet), destination, join)),
        );
        return false;
    }
    // FUN_0048a340 → FUN_0049a350: a line per member, its name and days.
    let lines = fleets
        .iter()
        .filter_map(|fleet| {
            let value = world.fleets.get(*fleet)?;
            let days = if value.location == destination {
                0
            } else {
                fleet_transit_ticks(value, world, value.location, destination)?
            };
            Some((fleet_label(world, *fleet)?, days))
        })
        .collect();
    move_confirmation_state.open(MoveConfirmation {
        faction: context.faction,
        fleets: fleets.to_vec(),
        members: Vec::new(),
        ships: None,
        units: Vec::new(),
        destination,
        join,
        blockaded: false,
        lines,
    });
    true
}

/// A capital ship team's Move (`0x201`) or Confirmed Move (`0x202`) with
/// its target set, each ship a per-object move (`0x204`, `FUN_0053ca50`):
/// `FUN_00487740` validates it (a refusal reaches the side's advisor,
/// `FUN_00487c90`), then `FUN_00487cc0` asks for `0x202`, and for `0x201`
/// only when the first member's container is a blockaded system; a ship's
/// container is its fleet, so its Move never asks. The window lists each
/// ship's name (`FUN_004f6270`) and days (`fleet_join::ship_move_days`).
/// Whether the window opened.
#[expect(
    clippy::too_many_arguments,
    reason = "The release hands over the order and every window it may open."
)]
fn issue_ship_move(
    context: &FleetMoveContext<'_>,
    fleet: FleetKey,
    ships: Vec<usize>,
    roster: u64,
    confirmed: bool,
    target: ReleaseTarget,
    move_confirmation_state: &mut MoveConfirmationState,
    panel_actions: &mut Vec<PanelAction>,
) -> bool {
    let world = context.world;
    let is_alliance = context.faction == MissionFaction::Alliance;
    let (destination, join) = match target {
        ReleaseTarget::System(system) => (system, None),
        ReleaseTarget::Fleet { fleet, system } => (system, Some(fleet)),
    };
    let checked = match join {
        Some(target) => rebellion_core::fleet_join::validate_join(
            world,
            context.movement,
            &FleetMover::Ships {
                fleet,
                ships: ships.clone(),
                roster,
            },
            target,
            is_alliance,
        )
        .map(drop),
        None => rebellion_core::fleet_join::validate_ships_to_system(
            world,
            context.movement,
            fleet,
            &ships,
            roster,
            destination,
            is_alliance,
        )
        .map(drop),
    };
    if let Err(error) = checked {
        macroquad::logging::info!("[order] ship move refused: {:?}", error);
        if let Some(status) = fleet_order_refusal_status(error) {
            panel_actions.push(PanelAction::AdvisorRefusal(status));
        }
        return false;
    }
    if !confirmed {
        panel_actions.push(mover_move_action(
            FleetMover::Ships {
                fleet,
                ships,
                roster,
            },
            destination,
            join,
        ));
        return false;
    }
    let lines = ships
        .iter()
        .filter_map(|&index| {
            Some((
                world.ship_name(fleet, index)?.to_owned(),
                rebellion_core::fleet_join::ship_move_days(world, fleet, index, destination)?,
            ))
        })
        .collect();
    move_confirmation_state.open(MoveConfirmation {
        faction: context.faction,
        fleets: Vec::new(),
        members: Vec::new(),
        ships: Some(FleetMover::Ships {
            fleet,
            ships,
            roster,
        }),
        units: Vec::new(),
        destination,
        join,
        blockaded: false,
        lines,
    });
    true
}

/// The panel action that carries out a mover's validated move: onto the
/// fleet it joins (`FUN_004ffc90`), else to the system.
fn mover_move_action(mover: FleetMover, destination: SystemKey, join: Option<FleetKey>) -> PanelAction {
    match (join, mover) {
        (Some(target), mover) => PanelAction::JoinFleet { mover, target },
        (None, FleetMover::Ships { fleet, ships, roster }) => PanelAction::MoveShips {
            fleet,
            ships,
            roster,
            system: destination,
        },
        (None, FleetMover::Fleet(fleet)) => PanelAction::DispatchFleet {
            fleet,
            destination,
            troops: Vec::new(),
        },
    }
}

/// A regiment's or squadron's Move (`0x201`) or Confirmed Move (`0x202`)
/// with its target set, the unit's own per-object move (`0x204`,
/// `FUN_0053ca50`): `FUN_00487740` validates it (a refusal reaches the
/// side's advisor, `FUN_00487c90`), then `FUN_00487cc0` asks for `0x202`,
/// and for `0x201` when the unit's container is a blockaded system of its
/// side; that window warns of the evacuation (0x7056, `FUN_0049a350`). A
/// unit aboard a fleet has the ship for its container. The window lists the
/// unit's name (`FUN_004f6270`) and days (`FUN_0053d200`). Whether the
/// window opened.
#[expect(
    clippy::too_many_arguments,
    reason = "The release hands over the order, the transport state and every window it may open."
)]
fn issue_unit_move(
    context: &FleetMoveContext<'_>,
    transport: &TroopTransportState,
    products: &ProductRoom,
    unit: UnitMover,
    confirmed: bool,
    target: ReleaseTarget,
    move_confirmation_state: &mut MoveConfirmationState,
    panel_actions: &mut Vec<PanelAction>,
) -> bool {
    use rebellion_core::troop_transport::TroopTransportError;
    let world = context.world;
    let (destination, join) = match target {
        ReleaseTarget::System(system) => (system, None),
        ReleaseTarget::Fleet { fleet, system } => (system, Some(fleet)),
    };
    let order = join.map_or(RegimentTarget::System(destination), RegimentTarget::Fleet);
    let (is_alliance, standing, name) = match unit {
        UnitMover::Regiment(troop) => {
            let value = world.troops.get(troop);
            (
                value.map(|value| value.is_alliance),
                world
                    .systems
                    .iter()
                    .find(|(_, system)| system.ground_units.contains(&troop))
                    .map(|(key, _)| key),
                value
                    .and_then(|value| world.buildable_classes.get(&value.class_dat_id))
                    .map(|class| class.name.clone()),
            )
        }
        UnitMover::Squadron(SquadronSource::Fleet { fleet, index }) => {
            let value = world.fleets.get(fleet);
            let class = value
                .and_then(|value| value.fighters.get(index))
                .and_then(|entry| world.fighter_classes.get(entry.class));
            (
                value.map(|value| value.is_alliance),
                None,
                class.map(|class| class.name.clone()),
            )
        }
        UnitMover::Squadron(SquadronSource::System {
            system,
            class,
            is_alliance,
        }) => (
            Some(is_alliance),
            Some(system),
            world.fighter_classes.get(class).map(|class| class.name.clone()),
        ),
    };
    let player_is_alliance = context.faction == MissionFaction::Alliance;
    let days = if is_alliance == Some(player_is_alliance) {
        match unit {
            UnitMover::Regiment(troop) => transport.regiment_move_days(
                world,
                context.movement,
                context.blockaded,
                products,
                troop,
                order,
            ),
            UnitMover::Squadron(source) => transport.squadron_move_days(
                world,
                context.movement,
                context.blockaded,
                products,
                source,
                order,
            ),
        }
    } else {
        Err(TroopTransportError::WrongFaction)
    };
    let days = match days {
        Ok(days) => days,
        Err(error) => {
            macroquad::logging::info!("[order] unit move refused: {:?}", error);
            if let Some(status) = transport_refusal_status(error) {
                panel_actions.push(PanelAction::AdvisorRefusal(status));
            }
            return false;
        }
    };
    let side = if player_is_alliance {
        Faction::Alliance
    } else {
        Faction::Empire
    };
    let blockaded = standing.is_some_and(|system| {
        context.blockaded.contains(&system)
            && world
                .systems
                .get(system)
                .is_some_and(|value| value.control.is_controlled_by(side))
    });
    if !confirmed && !blockaded {
        panel_actions.push(unit_move_action(unit, order));
        return false;
    }
    move_confirmation_state.open(MoveConfirmation {
        faction: context.faction,
        fleets: Vec::new(),
        members: Vec::new(),
        ships: None,
        units: vec![unit],
        destination,
        join,
        blockaded,
        lines: vec![(name.unwrap_or_default(), days)],
    });
    true
}

/// The panel action that carries out a unit's validated move
/// (`FUN_00578f30` → `FUN_00556390`).
fn unit_move_action(unit: UnitMover, target: RegimentTarget) -> PanelAction {
    match unit {
        UnitMover::Regiment(troop) => PanelAction::MoveRegiment { troop, target },
        UnitMover::Squadron(source) => PanelAction::MoveSquadron { source, target },
    }
}

/// Issue a character's or special force's Move or Confirmed Move with its
/// target set: `FUN_00486fb0` validates the `0x201` (each `0x241`'s
/// validator, `MissionState::move_order_enabled` and
/// `MissionState::move_destination_refusal`; a refusal reaches the side's
/// advisor, `FUN_00487c90`), then asks first when `FUN_00487cc0` says so:
/// for Confirmed Move, or when the first member stands in a blockaded
/// system, whose window warns of the evacuation (0x7056, `FUN_0049a350`).
/// The window lists each member's name and days (`FUN_004f4850`).
#[expect(
    clippy::too_many_arguments,
    reason = "The release hands over the order and every window it may open."
)]
fn issue_member_move(
    context: &FleetMoveContext<'_>,
    mission_state: &MissionState,
    members: Vec<MissionMember>,
    confirmed: bool,
    target: ReleaseTarget,
    advisor_state: &mut AdvisorState,
    move_confirmation_state: &mut MoveConfirmationState,
    panel_actions: &mut Vec<PanelAction>,
) {
    use rebellion_core::missions::MoveDestination;
    let world = context.world;
    let (destination, join) = match target {
        ReleaseTarget::System(system) => (system, None),
        ReleaseTarget::Fleet { fleet, system } => (system, Some(fleet)),
    };
    let order = join.map_or(MoveDestination::System(destination), MoveDestination::Fleet);
    let refusal =
        mission_state.move_refusal(world, context.movement, context.faction, &members, order);
    if let Some(refusal) = refusal {
        macroquad::logging::info!(
            "[interface] command=0x241 destination={:?} status=refused code={:?}",
            order,
            refusal.status()
        );
        advisor_state.refuse(refusal.status());
        return;
    }
    // FUN_00487cc0: a 0x201 asks only when the first member's container is
    // a blockaded system of the member's side; a character aboard a fleet
    // has the fleet as its container.
    let first_system = members.first().and_then(|&member| match member {
        MissionMember::Character(key) => world
            .characters
            .get(key)
            .filter(|c| c.current_fleet.is_none())
            .and_then(|c| c.current_system),
        MissionMember::SpecialForce(key) => world
            .systems
            .iter()
            .find(|(_, system)| system.special_forces.contains(&key))
            .map(|(system, _)| system),
    });
    let side = if context.faction == MissionFaction::Alliance {
        Faction::Alliance
    } else {
        Faction::Empire
    };
    let blockaded = first_system.is_some_and(|system| {
        context.blockaded.contains(&system)
            && world
                .systems
                .get(system)
                .is_some_and(|value| value.control.is_controlled_by(side))
    });
    if !confirmed && !blockaded {
        panel_actions.push(PanelAction::MoveMembers {
            members,
            destination: order,
        });
        return;
    }
    let lines = MissionState::move_days(world, context.faction, &members, order)
        .into_iter()
        .filter_map(|(member, days)| {
            let name = match member {
                MissionMember::Character(key) => {
                    world.characters.get(key)?.display_name().to_owned()
                }
                MissionMember::SpecialForce(key) => {
                    let unit = world.special_forces.get(key)?;
                    world
                        .buildable_classes
                        .get(&unit.class_dat_id)
                        .map_or_else(String::new, |class| class.name.clone())
                }
            };
            Some((name, days))
        })
        .collect();
    move_confirmation_state.open(MoveConfirmation {
        faction: context.faction,
        fleets: Vec::new(),
        members,
        ships: None,
        units: Vec::new(),
        destination,
        join,
        blockaded,
        lines,
    });
}

const fn encyclopedia_audience(faction: MissionFaction) -> EncyclopediaAudience {
    match faction {
        MissionFaction::Alliance => EncyclopediaAudience::Alliance,
        MissionFaction::Empire => EncyclopediaAudience::Empire,
    }
}

fn catalog_object_id(
    catalog: &EncyclopediaCatalog,
    dat_id: u32,
    family_range: std::ops::Range<u8>,
    source_name: Option<&str>,
) -> Option<u32> {
    if catalog
        .entries
        .iter()
        .any(|entry| entry.object_id == dat_id)
    {
        return Some(dat_id);
    }
    if dat_id >> 24 != 0 {
        return None;
    }
    catalog
        .entries
        .iter()
        .find(|entry| {
            entry.object_id & 0x00ff_ffff == dat_id
                && family_range.contains(&entry.family())
                && source_name.is_none_or(|name| entry.name == name)
        })
        .map(|entry| entry.object_id)
}

fn menu_object_encyclopedia_id(
    world: &GameWorld,
    catalog: &EncyclopediaCatalog,
    object: MenuObject,
) -> Option<u32> {
    match object {
        MenuObject::Character(key) => {
            let value = world.characters.get(key)?;
            catalog_object_id(catalog, value.dat_id.raw(), 0x30..0x40, Some(&value.name))
        }
        MenuObject::SpecialForce(key) => {
            let value = world.special_forces.get(key)?;
            catalog_object_id(catalog, value.class_dat_id.raw(), 0x30..0x40, None)
        }
        MenuObject::Troop(key) => {
            let value = world.troops.get(key)?;
            catalog_object_id(catalog, value.class_dat_id.raw(), 0x10..0x14, None)
        }
        MenuObject::Ship {
            fleet,
            index,
            roster,
        } => {
            if rebellion_core::fleet_join::roster(world, fleet) != Some(roster) {
                return None;
            }
            let class = world.fleets.get(fleet)?.capital_ships.get(index)?.class;
            let value = world.capital_ship_classes.get(class)?;
            catalog_object_id(catalog, value.dat_id.raw(), 0x14..0x20, Some(&value.name))
        }
        MenuObject::Fighter { fleet, index } => {
            let class = world.fleets.get(fleet)?.fighters.get(index)?.class;
            let value = world.fighter_classes.get(class)?;
            catalog_object_id(catalog, value.dat_id.raw(), 0x1c..0x20, Some(&value.name))
        }
        MenuObject::SystemFighter { class, .. } => {
            let value = world.fighter_classes.get(class)?;
            catalog_object_id(catalog, value.dat_id.raw(), 0x1c..0x20, Some(&value.name))
        }
        MenuObject::DefenseFacility(key) => {
            let value = world.defense_facilities.get(key)?;
            catalog_object_id(catalog, value.class_dat_id.raw(), 0x20..0x28, None)
        }
        MenuObject::SystemIcon {
            system,
            quadrant: Quadrant::System,
        } => {
            let value = world.systems.get(system)?;
            catalog_object_id(catalog, value.dat_id.raw(), 0x90..0x98, Some(&value.name))
        }
        MenuObject::Fleet(_)
        | MenuObject::Producer { .. }
        | MenuObject::SystemIcon {
            quadrant: Quadrant::Defenses | Quadrant::Fleets | Quadrant::Missions,
            ..
        } => None,
    }
}

fn log_encyclopedia_return(route: EncyclopediaReturnRoute) {
    macroquad::logging::info!(
        "[encyclopedia] production_route status=closed return={:?}",
        route
    );
}

#[cfg(test)]
mod encyclopedia_route_tests {
    use super::*;

    fn route_catalog() -> EncyclopediaCatalog {
        use rebellion_data::encyclopedia_catalog::{
            EncyclopediaCatalogEntry, EncyclopediaCategory,
        };

        EncyclopediaCatalog {
            title: "Galactic Encyclopedia".into(),
            topic_label: "Topic".into(),
            categories: [
                EncyclopediaCategory::new(0x6f, 0x1850, "All".into(), None),
                EncyclopediaCategory::new(0x70, 0x1855, "Systems".into(), Some(0x90..0x98)),
                EncyclopediaCategory::new(0x71, 0x1854, "Ships".into(), Some(0x14..0x20)),
                EncyclopediaCategory::new(0x72, 0x1852, "Facilities".into(), Some(0x20..0x30)),
                EncyclopediaCategory::new(0x73, 0x1851, "Missions".into(), Some(0x40..0x80)),
                EncyclopediaCategory::new(0x74, 0x1856, "Troops".into(), Some(0x10..0x14)),
                EncyclopediaCategory::new(0x75, 0x1853, "Personnel".into(), Some(0x30..0x40)),
            ],
            entries: vec![
                EncyclopediaCatalogEntry {
                    object_id: 0x1000_0002,
                    text_resource_id: 1,
                    name: "Troop".into(),
                },
                EncyclopediaCatalogEntry {
                    object_id: 0x1400_0045,
                    text_resource_id: 2,
                    name: "Corellian Corvette".into(),
                },
                EncyclopediaCatalogEntry {
                    object_id: 0x1c00_0003,
                    text_resource_id: 5,
                    name: "A-wing".into(),
                },
                EncyclopediaCatalogEntry {
                    object_id: 0x2000_0004,
                    text_resource_id: 6,
                    name: "Planetary Shield".into(),
                },
                EncyclopediaCatalogEntry {
                    object_id: 0x3100_0001,
                    text_resource_id: 3,
                    name: "Leia".into(),
                },
                EncyclopediaCatalogEntry {
                    object_id: 0x9200_002a,
                    text_resource_id: 4,
                    name: "Yavin".into(),
                },
            ],
        }
    }

    #[test]
    fn contextual_menu_objects_use_stable_source_class_identities() {
        let catalog = route_catalog();
        let mut world = GameWorld::default();
        let character = world.characters.insert(rebellion_core::world::Character {
            dat_id: rebellion_core::ids::DatId::new(0x3100_0001),
            name: "Leia".into(),
            ..rebellion_core::world::Character::default()
        });
        let troop = world.troops.insert(rebellion_core::world::TroopUnit {
            class_dat_id: rebellion_core::ids::DatId::new(0x1000_0002),
            is_alliance: true,
        });
        let fighter_class = world
            .fighter_classes
            .insert(rebellion_core::world::FighterClass {
                dat_id: rebellion_core::ids::DatId::new(0x1c00_0003),
                name: "A-wing".into(),
                ..rebellion_core::world::FighterClass::default()
            });
        let fleet = world.fleets.insert(rebellion_core::world::Fleet {
            location: rebellion_core::ids::SystemKey::default(),
            capital_ships: Vec::new(),
            fighters: vec![rebellion_core::world::FighterEntry {
                class: fighter_class,
                count: 1,
                carrier: 0,
            }],
            characters: Vec::new(),
            is_alliance: true,
            bombarding: false,
        });
        let defense =
            world
                .defense_facilities
                .insert(rebellion_core::world::DefenseFacilityInstance {
                    class_dat_id: rebellion_core::ids::DatId::new(0x2000_0004),
                    side: Faction::Alliance,
                });

        assert_eq!(
            menu_object_encyclopedia_id(&world, &catalog, MenuObject::Character(character)),
            Some(0x3100_0001)
        );
        assert_eq!(
            menu_object_encyclopedia_id(&world, &catalog, MenuObject::Troop(troop)),
            Some(0x1000_0002)
        );
        assert_eq!(
            menu_object_encyclopedia_id(&world, &catalog, MenuObject::Fighter { fleet, index: 0 },),
            Some(0x1c00_0003)
        );
        assert_eq!(
            menu_object_encyclopedia_id(&world, &catalog, MenuObject::DefenseFacility(defense),),
            Some(0x2000_0004)
        );
    }

    #[test]
    fn contextual_raw_dat_ids_rejoin_their_source_family_and_name() {
        let catalog = route_catalog();

        assert_eq!(
            catalog_object_id(&catalog, 0x45, 0x14..0x20, Some("Corellian Corvette")),
            Some(0x1400_0045)
        );
        assert_eq!(
            catalog_object_id(&catalog, 0x2a, 0x90..0x98, Some("Yavin")),
            Some(0x9200_002a)
        );
        assert_eq!(
            catalog_object_id(&catalog, 0x45, 0x14..0x20, Some("Wrong ship")),
            None
        );
    }

    #[test]
    fn player_faction_selects_the_matching_encyclopedia_audience() {
        assert_eq!(
            encyclopedia_audience(MissionFaction::Alliance),
            EncyclopediaAudience::Alliance
        );
        assert_eq!(
            encyclopedia_audience(MissionFaction::Empire),
            EncyclopediaAudience::Empire
        );
    }
}

/// The ship a band's Rename (0x215) names: the band manager's current
/// product (`FUN_0052be30`), when it takes a rename. `FUN_00527f40` asks the
/// product's `+0x64` for kind 0x203: a capital ship runs the object check
/// `FUN_004f9860`, which passes an unfinished ship of the side that is not
/// en route; a squadron refuses (`FUN_004f66a0`, 1/0x17), and an idle band
/// has no product (0xa0/1). Under Wine Rename was enabled on the route-1
/// Empire save's ships band building Death Star 1 (day 123 played on).
fn band_rename_target(
    world: &GameWorld,
    manufacturing: &ManufacturingState,
    system: SystemKey,
    area: rebellion_core::manufacturing::ProductionArea,
    is_alliance: bool,
) -> Option<(FleetKey, usize)> {
    let item = manufacturing.queue(system, area)?.active()?;
    let (fleet, index) = rebellion_core::carriage::ship_of_tag(world, item.ship)?;
    let value = world.fleets.get(fleet)?;
    let ship = value.capital_ships.get(index)?;
    (value.is_alliance == is_alliance && ship.alive && ship.transit.is_none())
        .then_some((fleet, index))
}

/// The system whose DAT id is `id`: how a save names a window's subject.
fn system_by_dat_id(world: &GameWorld, id: rebellion_core::ids::DatId) -> Option<SystemKey> {
    world
        .systems
        .iter()
        .find(|(_, system)| system.dat_id == id)
        .map(|(key, _)| key)
}

/// The galaxy child a saved detail window is, while it is open.
fn saved_child(
    world: &GameWorld,
    fleets: &FleetWindowState,
    window: &rebellion_data::save::SavedWindow,
) -> Option<GalaxyChild> {
    use rebellion_data::save::SavedWindowKind;
    match window.kind {
        SavedWindowKind::Sector { .. } => None,
        SavedWindowKind::System { system } => {
            system_by_dat_id(world, system).map(GalaxyChild::System)
        }
        SavedWindowKind::Defenses { system } => {
            system_by_dat_id(world, system).map(GalaxyChild::Defenses)
        }
        SavedWindowKind::Missions { system } => {
            system_by_dat_id(world, system).map(GalaxyChild::Missions)
        }
        SavedWindowKind::Fleet { system, .. } => system_by_dat_id(world, system)
            .and_then(|system| fleets.window_id(system))
            .map(GalaxyChild::Fleet),
    }
}

/// The interface a save keeps (`FUN_00440ea0`): the GID mode and the galaxy
/// view's windows in one list from the bottom of the z-order up, shown and
/// hidden alike, each hidden window at its own place (`FUN_00440ea0.c:19-24`).
/// `FUN_00440f80` rebuilds them in that order and appends each hidden one to
/// the rail as it meets it (`FUN_00428b40`), so a load's rail is in the
/// hidden windows' z-order. A
/// window the open GID menu hides saves hidden (its slot `+0x08` clears
/// `+0x110`) and so comes back on the rail: under Wine Coruscant's System
/// window, hidden by the menu at the save, came back minimized (Empire day-2
/// start, played on to day 8).
#[expect(
    clippy::too_many_arguments,
    reason = "The interface reads every galaxy window family."
)]
fn saved_interface(
    world: &GameWorld,
    cockpit: &rebellion_render::CockpitState,
    sectors: &SectorWindowState,
    systems: &SystemWindowState,
    fleets: &FleetWindowState,
    defenses: &DefensesWindowState,
    missions: &MissionsWindowState,
) -> rebellion_data::save::SavedInterface {
    let mut shown = sectors.saved_windows(world);
    for (serial, window) in systems
        .saved_windows(world)
        .into_iter()
        .chain(defenses.saved_windows(world))
        .chain(missions.saved_windows(world))
        .chain(fleets.saved_windows(world, |id| systems.rail_holds_fleet(id)))
    {
        let hidden =
            saved_child(world, fleets, &window).is_some_and(|child| cockpit.gid_ui.hides(child));
        shown.push((
            serial,
            rebellion_data::save::SavedWindow {
                shown: !hidden,
                ..window
            },
        ));
    }
    shown.extend(systems.saved_rail(world, |id| {
        fleets.saved_tab(id).zip(fleets.saved_serial(id))
    }));
    shown.sort_by_key(|(serial, _)| *serial);
    rebellion_data::save::SavedInterface {
        gid_mode: Some(cockpit.gid_mode.command_id()),
        windows: shown.into_iter().map(|(_, window)| window).collect(),
    }
}

/// The galaxy window families a load rebuilds into.
struct RestoredWindows<'a> {
    sectors: &'a mut SectorWindowState,
    systems: &'a mut SystemWindowState,
    fleets: &'a FleetWindowState,
    defenses: &'a mut DefensesWindowState,
    missions: &'a mut MissionsWindowState,
}

/// Rebuild a load's windows in saved order (`FUN_00440f80` →
/// `FUN_00441190`): each family's own restore, a hidden detail window back
/// on the rail. Fleet windows open through the window manager, which
/// commits them at the end of the frame (`finish_interface_restore`).
#[expect(
    clippy::too_many_arguments,
    reason = "The restore reads the world, the player's fog and missions, and the layout."
)]
fn restore_interface(
    order: &[rebellion_data::save::SavedWindow],
    world: &GameWorld,
    fog: &FogState,
    mission_state: &MissionState,
    faction: CockpitFaction,
    layout: rebellion_render::CockpitLayout,
    windows: RestoredWindows<'_>,
    strategic_windowing: &mut strategic_windowing::StrategicWindowing,
) {
    use rebellion_data::save::SavedWindowKind;
    for window in order {
        match window.kind {
            SavedWindowKind::Sector { .. } => {
                windows.sectors.restore_saved(world, faction, window);
            }
            SavedWindowKind::System { .. } => {
                windows.systems.restore_saved(world, faction, window);
            }
            SavedWindowKind::Defenses { .. } if window.shown => {
                windows.defenses.restore_saved(world, faction, window);
            }
            SavedWindowKind::Missions { .. } if window.shown => {
                windows
                    .missions
                    .restore_saved(world, fog, mission_state, faction, window);
            }
            SavedWindowKind::Defenses { .. } | SavedWindowKind::Missions { .. } => {
                windows.systems.restore_saved(world, faction, window);
            }
            SavedWindowKind::Fleet { system, tab } => {
                let prepared = system_by_dat_id(world, system).and_then(|system| {
                    windows.fleets.prepare_open(
                        world,
                        system,
                        window.position,
                        faction,
                        layout,
                        None,
                    )
                });
                let tab = rebellion_render::fleet_window::FleetWindowTab::from_control_id(tab);
                if let (Some(prepared), Some(tab)) = (prepared, tab) {
                    strategic_windowing.enqueue_fleet_open(prepared.with_tab(tab));
                }
            }
        }
    }
}

/// After a load's Fleet windows commit: a hidden one takes its slot on the
/// rail, where `FUN_00441190` appends each hidden window in save order
/// (slot `+0x08`, `FUN_00428b40`), and the shown windows take the saved
/// z-order again, each above the last, hidden and shown alike, since the
/// Fleet windows committed last. Sector windows stay at the bottom
/// (`FUN_00440f80.c:31-35`). A shown Fleet window brings the Fleet windows'
/// layer forward (`fleet_front`, the serial `galaxy_front` reads) at its
/// place in that order. Returns the Fleet windows going to the rail, which
/// take no focus.
fn finish_interface_restore(
    order: &[rebellion_data::save::SavedWindow],
    world: &GameWorld,
    systems: &mut SystemWindowState,
    defenses: &mut DefensesWindowState,
    missions: &mut MissionsWindowState,
    fleets: &mut FleetWindowState,
    strategic_windowing: &mut strategic_windowing::StrategicWindowing,
    fleet_front: &mut u64,
) -> Vec<GalaxyChild> {
    let mut hidden = Vec::new();
    let mut slot = 0;
    for window in order {
        let child = saved_child(world, fleets, window);
        match child {
            Some(GalaxyChild::Fleet(id)) if !window.shown => {
                if let rebellion_data::save::SavedWindowKind::Fleet { system, .. } = window.kind {
                    if let Some(system) = system_by_dat_id(world, system) {
                        systems.insert_fleet_rail(slot, id, system, window.position);
                        strategic_windowing.enqueue_fleet_minimize(id, system, window.position);
                    }
                }
                fleets.note_activation(id);
                hidden.push(GalaxyChild::Fleet(id));
            }
            Some(GalaxyChild::Fleet(id)) => {
                *fleet_front = FleetWindowState::next_legacy_bridge_serial();
                fleets.note_activation(id);
            }
            Some(child) if !window.shown => {
                systems.lift_hidden(child);
            }
            Some(GalaxyChild::System(system)) => {
                systems.raise(system);
            }
            Some(GalaxyChild::Defenses(system)) => {
                defenses.raise(system);
            }
            Some(GalaxyChild::Missions(system)) => {
                missions.raise(system);
            }
            Some(GalaxyChild::Sector(_)) | None => {}
        }
        if !window.shown
            && !matches!(
                window.kind,
                rebellion_data::save::SavedWindowKind::Sector { .. }
            )
        {
            slot += 1;
        }
    }
    hidden
}

/// The focus a load leaves: the top restored window that is shown
/// (`GalaxyFocusState::after_load`). A Fleet window there is the window
/// manager's, so the manager activates it too, which gives it the keyboard;
/// any other focus leaves the keyboard with the galaxy view's root, which
/// passes it to its own windows, a Fleet window opened by the load holding
/// it no longer.
fn focus_after_load(
    children: Vec<(u64, GalaxyChild)>,
    strategic_windowing: &mut strategic_windowing::StrategicWindowing,
) -> GalaxyFocusState {
    let focus = GalaxyFocusState::after_load(children);
    strategic_windowing.enqueue(match focus.focus {
        rebellion_render::GalaxyFocus::Child(GalaxyChild::Fleet(id)) => {
            rebellion_windowing::WindowCommand::Activate(id)
        }
        _ => rebellion_windowing::WindowCommand::FocusRoot,
    });
    focus
}

/// The GID mode a loaded game shows: the save's (view `+0xc8`), which
/// `FUN_00440f80` replays as the row choice (`WM_COMMAND` 0x140), or the
/// new-game default when the save has none. A highlight's systems (`+0x2cc`)
/// are not saved.
fn saved_gid_mode(interface: &rebellion_data::save::SavedInterface) -> rebellion_render::GidMode {
    interface
        .gid_mode
        .and_then(rebellion_render::GidMode::from_command_id)
        .unwrap_or(rebellion_render::GidMode::PopularSupport)
}

/// Whether each order of `selection`'s menu passes its own rule (`+0x18`).
#[expect(
    clippy::too_many_arguments,
    reason = "The gates read every state an order's rule reaches."
)]
fn order_gates(
    selection: Option<MenuObject>,
    world: &GameWorld,
    mission_state: &MissionState,
    movement_state: &MovementState,
    troop_transport_state: &TroopTransportState,
    blockade_state: &BlockadeState,
    manufacturing: &ManufacturingState,
    research: &rebellion_core::research::ResearchState,
    player_faction: MissionFaction,
) -> OrderGates {
    let is_alliance = player_faction == MissionFaction::Alliance;
    let side = if is_alliance {
        Faction::Alliance
    } else {
        Faction::Empire
    };
    // port: a band's manager is its system's holder's; the orders' own
    // +0x18 rules (FUN_0052ae30's class) are untraced.
    let holds = |system| {
        world
            .systems
            .get(system)
            .is_some_and(|value| value.control.is_controlled_by(side))
    };
    OrderGates {
        mission: selection
            .and_then(MenuObject::mission_member)
            .is_some_and(|member| {
                mission_state.mission_order_enabled(world, player_faction, &[member])
            }),
        fleet_move: match selection {
            Some(MenuObject::Fleet(fleet)) => {
                fleet_move_enabled(movement_state, world, fleet, is_alliance)
            }
            Some(MenuObject::SystemIcon {
                system,
                quadrant: Quadrant::Fleets,
            }) => fleets_move_enabled(
                movement_state,
                world,
                &system_side_fleets(world, system, is_alliance),
                is_alliance,
            ),
            _ => false,
        },
        troop_move: match selection {
            Some(MenuObject::Troop(troop)) => {
                troop_transport_state.regiment_move_enabled(world, troop, is_alliance)
            }
            _ => false,
        },
        // A squadron's own (FUN_00503930): the player's, held where it is.
        squadron_move: match selection {
            Some(MenuObject::SystemFighter {
                system,
                class,
                is_alliance: side,
            }) => {
                side == is_alliance
                    && world.systems.get(system).is_some_and(|value| {
                        value.fighters.iter().any(|entry| {
                            entry.class == class && entry.is_alliance == side && entry.count > 0
                        })
                    })
            }
            Some(MenuObject::Fighter { fleet, index }) => {
                fleet_move_enabled(movement_state, world, fleet, is_alliance)
                    && world
                        .fleets
                        .get(fleet)
                        .and_then(|value| value.fighters.get(index))
                        .is_some_and(|entry| entry.count > 0)
            }
            _ => false,
        },
        // FUN_004f9860: the ship's fleet is the player's and in orbit.
        member_move: selection
            .and_then(MenuObject::mission_member)
            .is_some_and(|member| {
                mission_state.move_order_enabled(world, player_faction, &[member])
            }),
        ship_move: match selection {
            Some(MenuObject::Ship { fleet, index, .. }) => rebellion_core::fleet_join::ship_move_enabled(
                world,
                movement_state,
                fleet,
                index,
                is_alliance,
            ),
            _ => false,
        },
        create_fleet: match selection {
            Some(MenuObject::Ship { fleet, index, .. }) => {
                rebellion_core::fleet_join::create_fleet_enabled(
                    world,
                    movement_state,
                    fleet,
                    index,
                    is_alliance,
                )
            }
            _ => false,
        },
        // port: a system's production areas are its manufacturing
        // facilities' (the 0xa0 managers are not modelled), the player's.
        destination: match selection {
            Some(MenuObject::SystemIcon {
                system,
                quadrant: Quadrant::System,
            }) => world.systems.get(system).is_some_and(|value| {
                value.manufacturing_facilities.iter().any(|key| {
                    world
                        .manufacturing_facilities
                        .get(*key)
                        .is_some_and(|facility| {
                            facility.side == rebellion_core::dat::Faction::of_alliance(is_alliance)
                        })
                })
            }),
            Some(MenuObject::Producer { system, .. }) => holds(system),
            _ => false,
        },
        // Build opens Build Selection on what the band's manager lists
        // (FUN_0052e580).
        build: match selection {
            Some(MenuObject::Producer { system, area }) => {
                holds(system)
                    && !rebellion_core::build_selection::listed_classes(
                        world,
                        research,
                        area,
                        is_alliance,
                    )
                    .is_empty()
            }
            _ => false,
        },
        // Stop clears a band that is building (manual p. 84). Its command's
        // +0x18 (FUN_0057eaa0) asks the manager's +0x1cc (FUN_00528270),
        // which refuses 0xa0/1 while no product resolves from ProductKey
        // +0x74 (FUN_0052be30): an idle band's Stop is greyed.
        stop: match selection {
            Some(MenuObject::Producer { system, area }) => {
                holds(system)
                    && manufacturing
                        .queue(system, area)
                        .is_some_and(|queue| !queue.is_empty())
            }
            _ => false,
        },
        // port: the player names its own fleets and ships; FUN_004f6e60
        // refuses a destroyed object (FUN_0053a000), and FUN_004f9860.c:61
        // one en route (its own leg); 0x203 is exempt from its completed
        // test, so an unfinished ship keeps Rename (the route-1 Empire save,
        // its day-123 Death Star played on). The order's own +0x18 rule is
        // untraced. A band's Rename (FUN_0057e6a0 → manager +0x1c4,
        // FUN_00527f40) refuses 0xa0/1 with no product, as Stop does, and
        // otherwise asks the product's +0x64 for kind 0x203
        // (band_rename_target).
        rename: match selection {
            Some(MenuObject::Fleet(fleet)) => world
                .fleets
                .get(fleet)
                .is_some_and(|value| value.is_alliance == is_alliance),
            Some(MenuObject::Ship { fleet, index, .. }) => {
                world.fleets.get(fleet).is_some_and(|value| {
                    value.is_alliance == is_alliance
                        && value
                            .capital_ships
                            .get(index)
                            .is_some_and(|ship| ship.alive && ship.transit.is_none())
                })
            }
            Some(MenuObject::Producer { system, area }) => {
                holds(system)
                    && band_rename_target(world, manufacturing, system, area, is_alliance).is_some()
            }
            _ => false,
        },
        bombard: strike_orders::bombard_gates(world, movement_state, selection, is_alliance),
        assault: assault_orders::assault_gate(
            world,
            movement_state,
            troop_transport_state,
            blockade_state,
            selection,
            is_alliance,
        ),
        scrap: scrap_order(selection).is_some_and(|order| {
            rebellion_core::scrap::order_enabled(
                world,
                movement_state,
                troop_transport_state,
                order,
                side,
            )
        }),
        command: match selection {
            Some(MenuObject::Character(character)) => {
                use rebellion_core::officer_rank::{command_refusal, CommandKind};
                let allowed = [
                    CommandKind::None,
                    CommandKind::Commander,
                    CommandKind::Admiral,
                    CommandKind::General,
                ]
                .map(|kind| command_refusal(world, character, is_alliance, kind).is_none());
                CommandGates {
                    allowed,
                    held: world
                        .characters
                        .get(character)
                        .map_or(CommandKind::None, |value| value.command_kind),
                }
            }
            _ => CommandGates::default(),
        },
    }
}

/// The Scrap order (`0x200`) a menu's object makes: its own, or a sector
/// window icon's team (`FUN_00512700`).
fn scrap_order(selection: Option<MenuObject>) -> Option<rebellion_core::scrap::ScrapOrder> {
    use rebellion_core::scrap::ScrapOrder;
    match selection? {
        MenuObject::Fleet(fleet) => Some(ScrapOrder::Fleet(fleet)),
        MenuObject::Ship { fleet, index, .. } => Some(ScrapOrder::Ship { fleet, index }),
        MenuObject::Troop(troop) => Some(ScrapOrder::Troop(troop)),
        MenuObject::DefenseFacility(key) => Some(ScrapOrder::Defense(key)),
        MenuObject::Fighter { fleet, index } => Some(ScrapOrder::Fighter {
            fleet,
            entry: index,
        }),
        MenuObject::SystemFighter {
            system,
            class,
            is_alliance,
        } => Some(ScrapOrder::SystemFighter {
            system,
            class,
            is_alliance,
        }),
        MenuObject::SystemIcon { system, quadrant } => match quadrant {
            Quadrant::System => Some(ScrapOrder::Facilities(system)),
            Quadrant::Defenses => Some(ScrapOrder::Defenses(system)),
            Quadrant::Fleets => Some(ScrapOrder::Fleets(system)),
            Quadrant::Missions => None,
        },
        _ => None,
    }
}

/// Carry out an Agent menu command or its accelerator: Manage Garrisons and
/// Manage Production toggle their module (`FUN_00439d60`), Agent Advice its
/// bit (`FUN_00439e80`).
/// Where a Finder's choice opens through `FUN_00429440`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FinderTarget {
    /// A system alone: its sector window.
    Sector,
    /// The System Defenses window (kind 10).
    Defenses,
    /// The Fleet window (kind 4) with the fleet selected.
    Fleet(rebellion_core::ids::FleetKey),
    /// The Fleet window (kind 4) with nothing selected.
    FleetWindow,
    /// The Missions window (kind 11) showing the member's mission.
    Missions(rebellion_core::missions::MissionMember),
}

/// The two waves a shown message plays (`FUN_00469de0`): `+0x30` from
/// STRATEGY (module 7) and its voice line `+0x32` from the player's sprite
/// DLL (module 9); 0 plays nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct MessageSounds {
    sound: u16,
    voice: u16,
}

/// Carry out what the Message Index asks: the Advice speed hold
/// (`FUN_00487ff0`), marking read and queuing the shown message's sound
/// (`FUN_0048a530`, `FUN_00469de0`), deleting (`FUN_005f54a0`) and the
/// posting flags (`FUN_0041d230`, `FUN_0041d290`). The caller routes
/// [`MessageIndexAction::GoTo`] and [`MessageIndexAction::OpenReport`].
fn apply_message_index_actions(
    actions: Vec<MessageIndexAction>,
    log: &mut MessageLog,
    clock: &mut GameClock,
    saved_speed: &mut Option<GameSpeed>,
    sound: &mut Option<MessageSounds>,
) {
    for action in actions {
        match action {
            MessageIndexAction::AdviceShown => {
                rebellion_render::game_speed::hold_speed_for_advice(clock, saved_speed, true);
            }
            MessageIndexAction::AdviceHidden => {
                rebellion_render::game_speed::hold_speed_for_advice(clock, saved_speed, false);
            }
            MessageIndexAction::MarkCategoryRead(Some(rail)) => log.mark_read(rail),
            MessageIndexAction::MarkCategoryRead(None) => log.mark_all_read(),
            MessageIndexAction::DeleteSelected(ids) => log.delete_by_ids(&ids),
            MessageIndexAction::MessageDisplayed(id) => {
                log.mark_message_read(id);
                // FUN_00469de0: stop the last message's waves
                // (FUN_0046a6e0), then play this one's +0x30 and +0x32.
                *sound = Some(
                    log.messages()
                        .iter()
                        .find(|msg| msg.id == id)
                        .and_then(|msg| msg.display.as_ref())
                        .map_or_else(MessageSounds::default, |display| MessageSounds {
                            sound: display.sound,
                            voice: display.voice,
                        }),
                );
            }
            MessageIndexAction::SetPostFlags { mask, flags } => {
                log.set_post_flags(mask, flags);
                macroquad::logging::info!(
                    "[message_index] post_flags mask=0x{:x} silent={}",
                    mask,
                    flags & rebellion_render::message_log::POST_SILENTLY != 0
                );
            }
            MessageIndexAction::Close
            | MessageIndexAction::GoTo(_)
            | MessageIndexAction::OpenReport(_) => {}
        }
    }
}

fn apply_agent_command(
    agent: &mut PlayerAgent,
    command: rebellion_render::agent_menu::AgentCommand,
    source: &str,
) {
    use rebellion_render::agent_menu::AgentCommand;
    let (name, on) = match command {
        AgentCommand::ManageGarrisons => (
            "manage_garrisons",
            agent.toggle(AutomationModule::Garrisons),
        ),
        AgentCommand::ManageProduction => (
            "manage_production",
            agent.toggle(AutomationModule::Production),
        ),
        AgentCommand::AgentAdvice => ("agent_advice", agent.toggle_advice()),
    };
    macroquad::logging::info!(
        "[agent] command={} status={} source={}",
        name,
        if on { "on" } else { "off" },
        source
    );
}

// ---------------------------------------------------------------------------
// Panel action handler
// ---------------------------------------------------------------------------

#[expect(
    clippy::too_many_arguments,
    reason = "Keep explicit state and UI inputs at this existing integration boundary."
)]
#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
#[expect(
    clippy::cast_precision_loss,
    reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
)]
fn apply_panel_action(
    action: PanelAction,
    world: &mut GameWorld,
    map_state: &mut GalaxyMapState,
    mfg_state: &mut ManufacturingState,
    delivery_state: &DeliveryState,
    mission_state: &mut MissionState,
    movement_state: &mut MovementState,
    troop_transport_state: &mut TroopTransportState,
    fog_state: &mut FogState,
    ai_state: &mut AIState,
    msg_log: &mut MessageLog,
    advisor_state: &mut AdvisorState,
    player_faction: &mut MissionFaction,
    clock: &mut GameClock,
    dual_ai_mode: &mut bool,
    secondary_ai_state: &mut Option<AIState>,
    victory_state: &mut VictoryState,
    campaign_config: CampaignConfig,
    blockade_state: &mut BlockadeState,
    uprising_state: &UprisingState,
    event_state: &EventState,
    mod_runtime: &mut rebellion_data::mods::ModRuntime,
    #[cfg(not(target_arch = "wasm32"))] encyclopedia_base_input: Option<
        &rebellion_data::encyclopedia_session::EncyclopediaSessionInput,
    >,
    #[cfg(not(target_arch = "wasm32"))] encyclopedia_session_store: &mut rebellion_data::encyclopedia_session::EncyclopediaSessionStore,
    #[cfg(not(target_arch = "wasm32"))] audio_engine: &mut audio::AudioEngine,
    #[cfg(not(target_arch = "wasm32"))] audio_vol: &AudioVolumeState,
    #[cfg(not(target_arch = "wasm32"))] _sounds_dir: &Path,
) {
    match action {
        // FUN_00578f30 → FUN_00556390 (ghidra/notes/regiment-unload.md).
        PanelAction::MoveRegiment { troop, target } => {
            let expected_is_alliance = *player_faction == MissionFaction::Alliance;
            let result = if world
                .troops
                .get(troop)
                .is_some_and(|value| value.is_alliance == expected_is_alliance)
            {
                troop_transport_state.move_regiment(
                    world,
                    movement_state,
                    blockade_state.blockaded_systems(),
                    &ProductRoom::new(mfg_state, delivery_state),
                    troop,
                    target,
                    clock.tick,
                )
            } else {
                Err(rebellion_core::troop_transport::TroopTransportError::WrongFaction)
            };
            eprintln!("[order] regiment move {troop:?} to {target:?}: {result:?}");
            if let Some(status) = result.err().and_then(transport_refusal_status) {
                advisor_state.refuse(status);
            }
        }
        // FUN_00556390 for a squadron (TroopTransportState::move_squadron).
        PanelAction::MoveSquadron { source, target } => {
            let expected_is_alliance = *player_faction == MissionFaction::Alliance;
            let is_alliance = match source {
                SquadronSource::Fleet { fleet, .. } => {
                    world.fleets.get(fleet).map(|value| value.is_alliance)
                }
                SquadronSource::System { is_alliance, .. } => Some(is_alliance),
            };
            let result = if is_alliance == Some(expected_is_alliance) {
                troop_transport_state.move_squadron(
                    world,
                    movement_state,
                    blockade_state.blockaded_systems(),
                    &ProductRoom::new(mfg_state, delivery_state),
                    source,
                    target,
                    clock.tick,
                )
            } else {
                Err(rebellion_core::troop_transport::TroopTransportError::WrongFaction)
            };
            eprintln!("[order] squadron move {source:?} to {target:?}: {result:?}");
            if let Some(status) = result.err().and_then(transport_refusal_status) {
                advisor_state.refuse(status);
            }
        }
        // FUN_0053c4b0: one 0x241 Move mission per location (command 0x250).
        PanelAction::MoveMembers {
            members,
            destination,
        } => {
            let result = mission_state.dispatch_move(
                world,
                movement_state,
                uprising_state,
                *player_faction,
                &members,
                destination,
                clock.tick,
            );
            match &result {
                Ok(ids) => macroquad::logging::info!(
                    "[interface] command=0x241 destination={:?} status=applied missions={:?}",
                    destination,
                    ids
                ),
                Err(refusal) => {
                    macroquad::logging::info!(
                        "[interface] command=0x241 destination={:?} status=refused code={:?}",
                        destination,
                        refusal.status()
                    );
                    advisor_state.refuse(refusal.status());
                }
            }
        }
        PanelAction::FocusFleetSystem(sys_key) => {
            if world.systems.contains_key(sys_key) {
                map_state.selected_system = Some(sys_key);
            }
        }
        // ghidra/notes/fleet-join-split.md.
        PanelAction::JoinFleet { mover, target } => {
            let result = rebellion_core::fleet_join::join_fleet(
                world,
                movement_state,
                troop_transport_state,
                &mover,
                target,
                *player_faction == MissionFaction::Alliance,
                clock.tick,
            );
            if let Err(error) = result {
                macroquad::logging::info!("[order] fleet join refused: {:?}", error);
                if let Some(status) = fleet_order_refusal_status(error) {
                    advisor_state.refuse(status);
                }
            }
        }
        PanelAction::SetDestination {
            system,
            area,
            destination,
        } => {
            // A band orders its own manager; the facility icon orders all
            // three, refused as one (FUN_00537180).
            let result = match area {
                Some(area) => rebellion_core::build_selection::order_destination(
                    world,
                    mfg_state,
                    troop_transport_state,
                    delivery_state,
                    system,
                    area,
                    destination,
                    movement_state,
                ),
                None => rebellion_core::build_selection::order_destination_all(
                    world,
                    mfg_state,
                    troop_transport_state,
                    delivery_state,
                    system,
                    destination,
                    movement_state,
                ),
            };
            let refusal = result.err();
            if let Some(refusal) = refusal {
                advisor_state.refuse(refusal.status());
            }
            macroquad::logging::info!(
                "[interface] command=0x214 destination=production_destination status={} areas={} to={}",
                if refusal.is_some() { "refused" } else { "set" },
                area.map_or(3, |_| 1),
                rebellion_data::integrator::destination_name(&world, destination)
            );
        }
        PanelAction::Rename { fleet, ship, name } => {
            let renamed = match ship {
                Some(index) => world.rename_ship(fleet, index, &name),
                None => world.rename_fleet(fleet, &name),
            };
            macroquad::logging::info!(
                "[interface] command=0x203 destination=rename status={} name={}",
                if renamed { "applied" } else { "refused" },
                name
            );
        }
        PanelAction::CreateFleet {
            fleet,
            ships,
            roster,
        } => {
            let result = rebellion_core::fleet_join::create_fleet(
                world,
                movement_state,
                troop_transport_state,
                fleet,
                &ships,
                roster,
                *player_faction == MissionFaction::Alliance,
            );
            if let Err(error) = result {
                macroquad::logging::info!("[order] create fleet refused: {:?}", error);
                if let Some(status) = fleet_order_refusal_status(error) {
                    advisor_state.refuse(status);
                }
            }
        }
        PanelAction::MoveShips {
            fleet,
            ships,
            roster,
            system,
        } => {
            let result = rebellion_core::fleet_join::move_ships_to_system(
                world,
                movement_state,
                troop_transport_state,
                fleet,
                &ships,
                roster,
                system,
                *player_faction == MissionFaction::Alliance,
                clock.tick,
            );
            if let Err(error) = result {
                macroquad::logging::info!("[order] move ships refused: {:?}", error);
                if let Some(status) = fleet_order_refusal_status(error) {
                    advisor_state.refuse(status);
                }
            }
        }
        PanelAction::DispatchFleet {
            fleet,
            destination,
            troops,
        } => {
            let expected_is_alliance = *player_faction == MissionFaction::Alliance;
            if let Err(error) = validate_fleet_dispatch(
                movement_state,
                world,
                fleet,
                destination,
                expected_is_alliance,
            ) {
                macroquad::logging::info!("[order] fleet dispatch refused: {:?}", error);
                if let Some(status) = fleet_dispatch_refusal_status(error) {
                    advisor_state.refuse(status);
                }
                return;
            }
            let products = ProductRoom::new(mfg_state, delivery_state);
            if let Err(error) = troop_transport_state.embark(world, &products, fleet, &troops) {
                if !troops.is_empty() {
                    macroquad::logging::info!("[order] fleet dispatch embark refused: {:?}", error);
                    if let Some(status) = transport_refusal_status(error) {
                        advisor_state.refuse(status);
                    }
                    return;
                }
            }
            match begin_faction_fleet_transit(
                movement_state,
                world,
                fleet,
                destination,
                expected_is_alliance,
            ) {
                // No message class files a departure (no TEXTSTRA template
                // names one); the fleet's transit shows on the map.
                Ok(_) => {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        audio_engine.play_sfx(SfxKind::FleetDeparture, audio_vol);
                    }
                }
                Err(error) => {
                    if !troops.is_empty() {
                        let origin = world
                            .fleets
                            .get(fleet)
                            .map(|value| value.location)
                            .unwrap_or_default();
                        if let Err(rollback) =
                            troop_transport_state.disembark_selected(world, fleet, origin, &troops)
                        {
                            macroquad::logging::error!(
                                "[order] fleet dispatch rollback failed: {:?}",
                                rollback
                            );
                        }
                    }
                    macroquad::logging::info!("[order] fleet dispatch refused: {:?}", error);
                    if let Some(status) = fleet_dispatch_refusal_status(error) {
                        advisor_state.refuse(status);
                    }
                }
            }
        }
        PanelAction::StopProduction { system, area } => {
            mfg_state.stop(system, area);
            // FUN_0052a2c0: the stopped products go with their entries.
            rebellion_core::manufacturing::seat_ordered_ships(world, mfg_state);
            macroquad::logging::info!(
                "[interface] command=0x213 destination=production_stop status=stopped area={:?} system={}",
                area,
                world.systems.get(system).map_or("", |value| value.name.as_str())
            );
        }
        // FUN_00438980 -> FUN_0041ce20: the units replace what the band
        // was building, each with its own build days.
        PanelAction::BuildProduction {
            system,
            area,
            kind,
            count,
        } => {
            let is_alliance = *player_faction == MissionFaction::Alliance;
            if let Some(refusal) = rebellion_core::build_selection::order_refusal(
                world,
                mfg_state,
                troop_transport_state,
                delivery_state,
                system,
                area,
                kind,
                count,
                is_alliance,
                movement_state,
            ) {
                // No text: the side's advisor reacts (FUN_004c1980,
                // FUN_004c1550; death-star-product.md).
                advisor_state.refuse(refusal.status());
                return;
            }
            if let Some(items) = rebellion_core::build_selection::order_items(
                world,
                system,
                area,
                kind,
                count,
                is_alliance,
            ) {
                mfg_state.build_units(system, items);
                // FUN_0052a140 creates the ordered products at once, a
                // fleet's squadrons and regiments aboard the ships the leg
                // chose.
                rebellion_core::carriage::seat_products(
                    world,
                    troop_transport_state,
                    delivery_state,
                    mfg_state,
                );
                rebellion_core::manufacturing::seat_ordered_ships(world, mfg_state);
                macroquad::logging::info!(
                    "[interface] command=0x21x destination=production_build status=queued area={:?} count={} system={}",
                    area,
                    count,
                    world.systems.get(system).map_or("", |value| value.name.as_str())
                );
            }
        }
        PanelAction::DispatchMission {
            kind,
            faction,
            team,
            decoys,
            target,
            target_character,
            target_object,
            tick,
        } => {
            // The first agent names the order in its messages.
            let char_name = team
                .iter()
                .find_map(|member| member.character())
                .and_then(|key| world.characters.get(key))
                .map_or_else(|| "Your team".into(), |c| c.display_name().to_owned());
            let request = rebellion_core::missions::MissionRequest {
                kind,
                faction,
                team,
                decoys,
                target_system: target,
                target_character,
                target_object,
                tick,
            };
            if let Err(refusal) = mission_state.dispatch_validated(request, world, uprising_state) {
                // FUN_00487c90 shows no text: the side's advisor reacts
                // (`mission-dialog.md`, "Refusal").
                #[cfg(not(target_arch = "wasm32"))]
                if let Ok(mut last) = dev_actions::LAST_DISPATCH_REFUSAL.lock() {
                    let (a, b) = refusal.status();
                    *last = Some(format!("{refusal:?} (0x{a:x}/0x{b:x})"));
                }
                advisor_state.refuse(refusal.status());
                return;
            }
            let sys_name = world
                .systems
                .get(target)
                .map_or_else(|| "unknown".into(), |s| s.name.clone());
            let kind_name = match kind {
                MissionKind::Diplomacy => "Diplomacy",
                MissionKind::Recruitment => "Recruitment",
                MissionKind::Sabotage => "Sabotage",
                MissionKind::Assassination => "Assassination",
                MissionKind::Espionage => "Espionage",
                MissionKind::Rescue => "Rescue",
                MissionKind::Abduction => "Abduction",
                MissionKind::InciteUprising => "Incite Uprising",
                MissionKind::SubdueUprising => "Subdue Uprising",
                MissionKind::DeathStarSabotage => "Death Star Sabotage",
                MissionKind::Research(_) => rebellion_render::mission_dialog::kind_name(kind),
                MissionKind::Autoscrap => "Autoscrap",
                MissionKind::Move => "Move",
            };
            msg_log.push(GameMessage::at_system(
                clock.tick,
                format!("{char_name} dispatched on {kind_name} mission to {sys_name}"),
                MessageCategory::Mission,
                target,
            ));
        }
        PanelAction::CancelMission(id) => {
            mission_state.release(id, world);
        }
        // Save/load actions are handled by the caller before dispatching here;
        // they require access to the full save state and are not routed through
        // this helper.
        PanelAction::SelectFaction(_)
        | PanelAction::Bombard { .. }
        | PanelAction::Assault { .. }
        | PanelAction::SaveGame { .. }
        | PanelAction::LoadGame { .. }
        | PanelAction::OpenModManager => {
            // Handled by UI state toggle (not a world mutation)
        }
        PanelAction::ToggleMod { ref name } => match mod_runtime.toggle_mod(name) {
            Ok(()) => {
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(base) = encyclopedia_base_input {
                    let resolved_mod_order = mod_runtime.enabled_sorted();
                    let report = encyclopedia_mods::install_native_encyclopedia_mods(
                        base,
                        encyclopedia_session_store,
                        mod_runtime,
                        &resolved_mod_order,
                    );
                    encyclopedia_mods::log_native_encyclopedia_mod_report(&report);
                }
                msg_log.push(GameMessage::new(
                    clock.tick,
                    format!("Toggled mod: {name}"),
                    MessageCategory::Event,
                ));
            }
            Err(error) => {
                eprintln!("[mod-runtime] failed to toggle {name}: {error}");
                msg_log.push(GameMessage::new(
                    clock.tick,
                    format!("Failed to toggle mod {name}: {error}"),
                    MessageCategory::Event,
                ));
            }
        },
        PanelAction::ReloadMods => {
            mod_runtime.refresh();
            let resolved_mod_order = mod_runtime.enabled_sorted();
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(base) = encyclopedia_base_input {
                let report = encyclopedia_mods::install_native_encyclopedia_mods(
                    base,
                    encyclopedia_session_store,
                    mod_runtime,
                    &resolved_mod_order,
                );
                encyclopedia_mods::log_native_encyclopedia_mod_report(&report);
            }
            let mod_errors = mod_runtime.apply_ordered(world, &resolved_mod_order);
            for err in &mod_errors {
                eprintln!("Mod reload error: {err:?}");
            }
            msg_log.push(GameMessage::new(
                clock.tick,
                format!("Reloaded {} mods", mod_runtime.discovered.len()),
                MessageCategory::Event,
            ));
        }
        PanelAction::AdvanceTicks(n) => {
            // Force-advance N ticks synchronously.
            // NOTE: This only advances the clock counter. Full simulation tick
            // execution requires run_simulation_tick() which needs &mut access to
            // all states — not available inside apply_panel_action(). The actual
            // simulation will catch up on the next frame when clock.advance(dt)
            // emits the pending TickEvents. For instant effect, set speed to Faster.
            // port: the AI's in-day rounds then go on under the new day number
            // with the old day's step count, until the next tick closes it.
            clock.tick += n;
        }
        // The caller routes speed through the clock's pause stop.
        PanelAction::SetGameSpeed(_) => {}
        PanelAction::ToggleDualAI => {
            *dual_ai_mode = !*dual_ai_mode;
            if *dual_ai_mode {
                // Create persistent second AI for the opposite faction
                let second_faction = match ai_state.faction {
                    Some(AiFaction::Empire) => AiFaction::Alliance,
                    _ => AiFaction::Empire,
                };
                *secondary_ai_state = Some(AIState::new(second_faction));
            } else {
                *secondary_ai_state = None;
            }
            let state_str = if *dual_ai_mode { "ENABLED" } else { "DISABLED" };
            msg_log.push(GameMessage::new(
                clock.tick,
                format!("Dual AI mode {state_str}"),
                MessageCategory::Event,
            ));
        }
        PanelAction::AdvisorRefusal(status) => advisor_state.refuse(status),
        PanelAction::ForceVictoryCheck => {
            let tick_ev = rebellion_core::tick::TickEvent { tick: clock.tick };
            let result = rebellion_core::victory::VictorySystem::check(
                victory_state,
                world,
                &[tick_ev],
                campaign_config.victory_conditions,
            );
            if let Some(outcome) = result {
                msg_log.push(GameMessage::new(
                    clock.tick,
                    format!("Victory check: {outcome:?}"),
                    MessageCategory::Event,
                ));
            } else {
                msg_log.push(GameMessage::new(
                    clock.tick,
                    "Victory check: no winner yet".to_string(),
                    MessageCategory::Event,
                ));
            }
        }
        PanelAction::RevealAllFog => {
            // Reveal all systems in fog state
            for (sys_key, _) in &world.systems {
                fog_state.reveal(sys_key);
            }
        }
        PanelAction::ExportGameLog => {
            // Resolve system keys to names before export
            msg_log.resolve_system_names(|key| world.systems.get(key).map(|s| s.name.clone()));
            let path = std::path::PathBuf::from("game_log.jsonl");
            match msg_log.export_jsonl(&path) {
                Ok(()) => {
                    msg_log.push(GameMessage::new(
                        clock.tick,
                        format!("Exported game log to {}", path.display()),
                        MessageCategory::Event,
                    ));
                }
                Err(e) => {
                    eprintln!("Failed to export game log: {e}");
                }
            }
        }
        PanelAction::ShowGameStats => {
            let alliance_systems = world
                .systems
                .values()
                .filter(|s| s.control.is_controlled_by(Faction::Alliance))
                .count();
            let empire_systems = world
                .systems
                .values()
                .filter(|s| s.control.is_controlled_by(Faction::Empire))
                .count();
            msg_log.push(GameMessage::new(
                clock.tick,
                format!("Stats: tick {}, Alliance {} systems, Empire {} systems, {} fleets, {} characters",
                    clock.tick, alliance_systems, empire_systems, world.fleets.len(), world.characters.len()),
                MessageCategory::Event,
            ));
        }
        PanelAction::ListActiveMissions => {
            let missions = mission_state.missions();
            if missions.is_empty() {
                msg_log.push(GameMessage::new(
                    clock.tick,
                    "No active missions".to_string(),
                    MessageCategory::Mission,
                ));
            } else {
                msg_log.push(GameMessage::new(
                    clock.tick,
                    format!("{} active missions:", missions.len()),
                    MessageCategory::Mission,
                ));
                for m in missions {
                    let char_name = m
                        .lead_character(world)
                        .and_then(|key| world.characters.get(key))
                        .map_or_else(|| "Unknown".into(), |c| c.display_name().to_owned());
                    let sys_name = world
                        .systems
                        .get(m.target_system)
                        .map_or_else(|| "unknown".into(), |s| s.name.clone());
                    msg_log.push(GameMessage::new(
                        clock.tick,
                        format!("  {:?} — {} at {}", m.kind, char_name, sys_name),
                        MessageCategory::Mission,
                    ));
                }
            }
        }
        PanelAction::ListActiveFleets => {
            msg_log.push(GameMessage::new(
                clock.tick,
                format!("{} fleets:", world.fleets.len()),
                MessageCategory::Event,
            ));
            for (_, fleet) in &world.fleets {
                let sys_name = world
                    .systems
                    .get(fleet.location)
                    .map_or_else(|| "unknown".into(), |s| s.name.clone());
                let faction = if fleet.is_alliance {
                    "Alliance"
                } else {
                    "Empire"
                };
                let ship_count = fleet.ship_count() as usize
                    + fleet
                        .fighters
                        .iter()
                        .map(|e| e.count as usize)
                        .sum::<usize>();
                msg_log.push(GameMessage::new(
                    clock.tick,
                    format!("  {faction} fleet at {sys_name} — {ship_count} ships"),
                    MessageCategory::Event,
                ));
            }
        }
        PanelAction::ShowEventCount => {
            let total = event_state.events().len();
            let fired = event_state
                .events()
                .iter()
                .filter(|e| event_state.has_fired(e.id))
                .count();
            msg_log.push(GameMessage::new(
                clock.tick,
                format!("Events: {total} defined, {fired} fired"),
                MessageCategory::Event,
            ));
        }
    }
}

// ---------------------------------------------------------------------------
// Effect application helpers
// ---------------------------------------------------------------------------

/// Notification 5, Research Report, for each level a side bought.
fn push_research_reports(
    msg_log: &mut MessageLog,
    tick: u64,
    results: &[rebellion_core::research::ResearchResult],
) {
    for result in results {
        let rebellion_core::research::ResearchResult::TechUnlocked {
            faction_is_alliance,
            tech_type,
            new_level,
        } = result;
        let faction_name = if *faction_is_alliance {
            "Alliance"
        } else {
            "Empire"
        };
        let tech_name = match tech_type {
            rebellion_core::research::TechType::Ship => "Ship",
            rebellion_core::research::TechType::Troop => "Troop",
            rebellion_core::research::TechType::Facility => "Facility",
        };
        msg_log.push(filed(
            GameMessage::new(
                tick,
                format!("{faction_name} {tech_name} tech advanced to level {new_level}"),
                MessageCategory::Event,
            )
            // FUN_00499aa0: research carries code 4.
            .with_advice(4),
            MessageRail::Manufacturing,
            Some(RailAudience::side(*faction_is_alliance)),
        ));
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
fn apply_mission_result(
    result: &rebellion_core::missions::MissionResult,
    world: &mut GameWorld,
    log: &mut MessageLog,
    #[cfg(not(target_arch = "wasm32"))] audio_engine: &mut audio::AudioEngine,
    #[cfg(not(target_arch = "wasm32"))] audio_vol: &AudioVolumeState,
) {
    rebellion_data::telemetry::native_mission_resolved(world, result);
    let faction_name = match result.faction {
        MissionFaction::Alliance => "Alliance",
        MissionFaction::Empire => "Empire",
    };
    let kind_name = match result.kind {
        MissionKind::Diplomacy => "Diplomacy",
        MissionKind::Recruitment => "Recruitment",
        MissionKind::Sabotage => "Sabotage",
        MissionKind::Assassination => "Assassination",
        MissionKind::Espionage => "Espionage",
        MissionKind::Rescue => "Rescue",
        MissionKind::Abduction => "Abduction",
        MissionKind::InciteUprising => "Incite Uprising",
        MissionKind::SubdueUprising => "Subdue Uprising",
        MissionKind::DeathStarSabotage => "Death Star Sabotage",
        MissionKind::Autoscrap => "Autoscrap",
        MissionKind::Move => "Move",
        MissionKind::Research(rebellion_core::research::TechType::Ship) => "Ship Design Research",
        MissionKind::Research(rebellion_core::research::TechType::Troop) => {
            "Troop Training Research"
        }
        MissionKind::Research(rebellion_core::research::TechType::Facility) => {
            "Facility Design Research"
        }
    };
    let sys_name = world
        .systems
        .get(result.target_system)
        .map_or_else(|| "unknown".into(), |s| s.name.clone());
    let outcome_str = match result.outcome {
        rebellion_core::missions::MissionOutcome::Success => "succeeded",
        rebellion_core::missions::MissionOutcome::Failure => "failed",
    };

    let category = match result.kind {
        MissionKind::Diplomacy | MissionKind::InciteUprising | MissionKind::SubdueUprising => {
            MessageCategory::Diplomacy
        }
        MissionKind::Recruitment
        | MissionKind::Sabotage
        | MissionKind::Assassination
        | MissionKind::Espionage
        | MissionKind::Rescue
        | MissionKind::Abduction
        | MissionKind::DeathStarSabotage
        | MissionKind::Research(_)
        | MissionKind::Autoscrap
        | MissionKind::Move => MessageCategory::Mission,
    };
    // Notifications 0x16 and 0x17, Mission Report and Mission Failed.
    log.push(filed(
        GameMessage::at_system(
            result.tick,
            format!("{faction_name} {kind_name} mission at {sys_name} {outcome_str}"),
            category,
            result.target_system,
        )
        // FUN_004927c0, FUN_00491500: a mission report carries its agent's
        // code.
        .with_advice(
            result
                .character
                .map_or(0x15, |agent| personnel_advice(world, agent)),
        ),
        MessageRail::Mission,
        Some(RailAudience::side(
            result.faction == MissionFaction::Alliance,
        )),
    ));

    // SFX + voice lines for mission outcomes
    #[cfg(not(target_arch = "wasm32"))]
    if result.outcome == rebellion_core::missions::MissionOutcome::Success {
        audio_engine.play_sfx(SfxKind::MissionSuccess, audio_vol);
    } else {
        audio_engine.play_sfx(SfxKind::MissionFail, audio_vol);
    }

    for effect in &result.effects {
        match effect {
            MissionEffect::SupportGained {
                system,
                side,
                points,
            } => rebellion_core::uprising::apply_support_change(world, *system, *side, *points),
            MissionEffect::SkillRaised {
                character,
                skill,
                amount,
            } => rebellion_core::missions::raise_skill(world, *character, *skill, *amount),
            // Credited by ResearchSystem::apply_mission_points.
            MissionEffect::ResearchPoints { .. } => {}
            MissionEffect::UprisingIncident(event) => {
                rebellion_core::uprising::apply_uprising_event(world, event);
            }
            MissionEffect::SystemIntelligenceGathered { system, .. } => {
                // Reveal fog: mark system as explored (full implementation in fog task).
                if let Some(sys) = world.systems.get_mut(*system) {
                    sys.exploration_status = rebellion_core::dat::ExplorationStatus::Explored;
                }
            }
            MissionEffect::CharacterRecruited {
                character,
                system,
                faction,
                pool_emptied,
            } => rebellion_core::missions::recruit_character(
                world,
                *character,
                *system,
                *faction,
                *pool_emptied,
            ),
            // A sabotaged Death Star's message is a movie
            // (strike_orders::sabotage_movies).
            MissionEffect::TargetSabotaged { target, .. } => {
                rebellion_core::missions::destroy_target(world, *target);
            }
            // As the integrator does: the destroyed character leaves every
            // fleet and stays in the arena, marked killed.
            MissionEffect::CharacterKilled { character, .. } => {
                for (_, fleet) in &mut world.fleets {
                    fleet.characters.retain(|&k| k != *character);
                }
                if let Some(c) = world.characters.get_mut(*character) {
                    c.mark_killed();
                }
            }
            MissionEffect::CharacterCaptured {
                character,
                captured_by,
                ..
            } => {
                // is_alliance is the native side; a prisoner's side bits are
                // its opposite while held (FUN_004f1a80 -> FUN_004eec60).
                if let Some(c) = world.characters.get_mut(*character) {
                    c.is_captive = true;
                    c.captured_by = Some(match captured_by {
                        MissionFaction::Alliance => Faction::Alliance,
                        MissionFaction::Empire => Faction::Empire,
                    });
                    c.capture_tick = Some(result.tick);
                    c.leave_abode();
                }
                // Remove from current fleet assignments
                for (_, fleet) in &mut world.fleets {
                    fleet.characters.retain(|&k| k != *character);
                }
            }
            MissionEffect::SpecialForceDestroyed { unit } => {
                rebellion_core::missions::destroy_special_force(world, *unit);
            }
            // port: the port stores no injury (F-026).
            MissionEffect::CharacterInjured { .. } => {}
            MissionEffect::CharacterRescued {
                character,
                returned_to,
                ..
            } => {
                // Restore character to the specified faction
                if let Some(c) = world.characters.get_mut(*character) {
                    match returned_to {
                        MissionFaction::Alliance => {
                            c.is_alliance = true;
                            c.is_empire = false;
                        }
                        MissionFaction::Empire => {
                            c.is_alliance = false;
                            c.is_empire = true;
                        }
                    }
                    c.is_captive = false;
                    c.captured_by = None;
                    c.capture_tick = None;
                    c.leave_abode();
                }
            }
            MissionEffect::MemberAvailable { member } => {
                rebellion_core::missions::set_on_mission(world, *member, false);
            }
            MissionEffect::MemberMoved { member, to } => {
                rebellion_core::missions::move_member(world, *member, *to);
            }
            MissionEffect::MemberBoarded { member, fleet } => {
                rebellion_core::missions::board_member(world, *member, *fleet);
            }
            MissionEffect::EnRoute { member, en_route } => {
                rebellion_core::missions::set_en_route(world, *member, *en_route);
            }
            // The escape step's own release (death_star::free_escapee); no
            // mission result carries one.
            MissionEffect::CharacterEscaped { .. } => {}
            MissionEffect::UprisingSubdued {
                system,
                side,
                support_gain,
            } => {
                // FUN_00569c20 -> FUN_0050c9f0. The caller then runs the
                // FUN_0050c910 end check via apply_mission_state_effects.
                rebellion_core::uprising::apply_support_change(
                    world,
                    *system,
                    *side,
                    *support_gain,
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Knesset Shamash-Bet Dabora 2 (#F7): the old `apply_event_actions` duplicate
// was deleted here. The canonical implementation lives in
// `rebellion_data::integrator::apply_event_action_to_world` (pub #[inline]).
// `DisplayMessage` now routes through `GameEffect::StoryMessageDisplayed`
// and is drained into `msg_log` at the interactive tick call site.
// SetHeritageKnown (Dabora 3 #R4) is handled in the canonical function.
// ---------------------------------------------------------------------------

/// The system whose battle waits that `action` would move or change a
/// fleet, squadron, regiment or planet at, if any. The original resolves a
/// battle before its computer player orders again: the Battle Alert and the
/// battle are `FUN_0040a050` states 5-0x17, and the day stands still under
/// the alert and its result (the original from save `alliance-day54`, the
/// Battle at Praesitlyn on days 508 and 532: the day held while the Retreat
/// result was up). hyp: the round scheduler stands still with the day.
fn held_at(
    world: &GameWorld,
    transport: &TroopTransportState,
    held: &[SystemKey],
    action: &AIAction,
) -> Option<SystemKey> {
    if held.is_empty() {
        return None;
    }
    let at = |fleet: &FleetKey| {
        world
            .fleets
            .get(*fleet)
            .map(|value| value.location)
            .filter(|location| held.contains(location))
    };
    let system = |system: &SystemKey| held.contains(system).then_some(*system);
    let aboard = |troop: &TroopKey| {
        world
            .fleets
            .iter()
            .find(|(key, value)| {
                held.contains(&value.location) && transport.cargo(*key).contains(troop)
            })
            .map(|(_, value)| value.location)
    };
    match action {
        AIAction::MoveFleet { fleet, .. }
        | AIAction::CreateFleet { fleet, .. }
        | AIAction::Assault { fleet, .. }
        | AIAction::MoveCharacterToFleet { fleet, .. } => at(fleet),
        AIAction::JoinFleet { fleet, target, .. } => at(fleet).or_else(|| at(target)),
        AIAction::Bombard {
            fleet,
            system: at_system,
            ..
        }
        | AIAction::DestroySystem {
            fleet,
            system: at_system,
        } => at(fleet).or_else(|| system(at_system)),
        AIAction::MoveSquadrons { squadrons, fleet } => at(fleet).or_else(|| {
            squadrons.iter().find_map(|source| match source {
                SquadronSource::Fleet { fleet, .. } => at(fleet),
                SquadronSource::System { system: from, .. } => system(from),
            })
        }),
        AIAction::MoveRegiments { troops, .. } => troops.iter().find_map(aboard),
        AIAction::Scrap { targets } => targets.iter().find_map(|target| match target {
            rebellion_core::scrap::ScrapTarget::CapitalShip { fleet, .. }
            | rebellion_core::scrap::ScrapTarget::Fighter { fleet, .. } => at(fleet),
            rebellion_core::scrap::ScrapTarget::SystemFighter { system: from, .. } => system(from),
            rebellion_core::scrap::ScrapTarget::Troop(troop) => aboard(troop),
            _ => None,
        }),
        _ => None,
    }
}

/// The systems where both sides are present, each a battle the next combat
/// pass fights (FUN_00509710 / FUN_0050b8e0).
fn contested_systems(world: &GameWorld) -> Vec<SystemKey> {
    world
        .systems
        .keys()
        .filter(|&system| side_present(world, system, true) && side_present(world, system, false))
        .collect()
}

/// Write each AI order's records, stamped with the day its step ran, and
/// show what an applied one did: the dispatch message, the fleet's
/// departure sound and message, and the strike's messages.
fn show_ai_records(
    records: &[rebellion_data::simulation::AiOrderRecord],
    tick: u64,
    world: &GameWorld,
    templates: &rebellion_data::text_templates::TextTemplates,
    log: &mut MessageLog,
    #[cfg(not(target_arch = "wasm32"))] audio_engine: &mut audio::AudioEngine,
    #[cfg(not(target_arch = "wasm32"))] audio_vol: &AudioVolumeState,
) {
    rebellion_data::telemetry::native_ai_records(tick, world, records);
    for record in records {
        let origin = record.origin();
        if !record.outcome.is_applied() {
            continue;
        }
        match &record.issued.action {
            AIAction::DispatchMission {
                kind,
                target_system,
                ..
            } => {
                log.push(GameMessage::at_system(
                    tick,
                    format!("{} dispatched {kind:?} mission", origin.faction),
                    MessageCategory::Ai,
                    *target_system,
                ));
            }
            AIAction::MoveFleet {
                to_system,
                reason,
                troops,
                ..
            } => {
                #[cfg(not(target_arch = "wasm32"))]
                audio_engine.play_sfx(SfxKind::FleetDeparture, audio_vol);
                let reason = match reason {
                    FleetMoveReason::Attack => "attack",
                    FleetMoveReason::Reinforce => "reinforce",
                };
                log.push(GameMessage::at_system(
                    tick,
                    format!(
                        "{} fleet moving to system ({reason}){}",
                        origin.faction,
                        if troops.is_empty() {
                            String::new()
                        } else {
                            format!(" with {} regiment(s)", troops.len())
                        },
                    ),
                    MessageCategory::Ai,
                    *to_system,
                ));
            }
            AIAction::Bombard { .. } | AIAction::DestroySystem { .. } => {
                for message in
                    strike_orders::strike_messages(world, templates, tick, &record.strike)
                {
                    log.push(message);
                }
            }
            AIAction::Assault { .. } => {
                for message in
                    assault_orders::assault_messages(world, templates, tick, &record.assault)
                {
                    log.push(message);
                }
            }
            _ => {}
        }
        // The holder changes the order's regiments made (FUN_00508660 ->
        // FUN_0050b610).
        for change in &record.fired.controls {
            for message in control_change_messages(world, templates, tick, change) {
                log.push(message);
            }
        }
    }
}

/// The messages of a blockade starting: event `0x14e`'s Blockade Message
/// (`blockade_messages`). A lift files nothing: `FUN_004960f0` files only
/// for a set bit and a blockading fleet (`FUN_0052bed0`). Other events have
/// none.
fn blockade_transition_messages(
    world: &GameWorld,
    templates: &rebellion_data::text_templates::TextTemplates,
    evt: &rebellion_core::blockade::BlockadeEvent,
) -> Vec<GameMessage> {
    match *evt {
        rebellion_core::blockade::BlockadeEvent::BlockadeStarted { system, tick } => {
            blockade_messages(world, templates, system, tick)
        }
        _ => Vec::new(),
    }
}

/// Event `0x340`'s notice (`FUN_00495e90`; `FUN_00504a00` raises the event
/// for a regiment lost running a blockade): "Evacuation Losses" (`0x70a8`),
/// "The following units were lost running an enemy blockade of |."
/// (`0x70a9`, the system as argument 2) and a line per regiment (`0x70aa`,
/// argument 1), with code `0x10`; picture `0x407` and sound `0x460` for side
/// 1, `0x406` and `0x45f` for side 2. The class adds each later regiment to
/// its message (`+0x68`); the port gathers a system's regiments of one
/// tick. hyp: both sides file it, as `FUN_00504850` raises the event in
/// both side views; its rail is untraced, and the port uses the Blockade
/// Message's. Without the TEXTSTRA entries nothing is filed.
fn evacuation_losses_messages(
    world: &GameWorld,
    templates: &rebellion_data::text_templates::TextTemplates,
    system: SystemKey,
    tick: u64,
    units: &[String],
) -> Vec<GameMessage> {
    use rebellion_data::text_templates::TemplateArg;
    let Some(sys) = world.systems.get(system) else {
        return Vec::new();
    };
    if units.is_empty() {
        return Vec::new();
    }
    let place = TemplateArg::named(sys.name.clone());
    [true, false]
        .into_iter()
        .filter_map(|alliance| {
            let title = templates.format(0x70a8, &[])?;
            let mut body = templates.format(0x70a9, &[None, Some(&place)])?;
            for unit in units {
                body += &templates.format(0x70aa, &[Some(&TemplateArg::named(unit.clone()))])?;
            }
            let (background, sound) = if alliance {
                (0x407, 0x460)
            } else {
                (0x406, 0x45f)
            };
            Some(
                GameMessage::at_system(tick, title.clone(), MessageCategory::Combat, system)
                    .with_advice(0x10)
                    .with_target(MessageTarget::System(system))
                    .on_rail(MessageRail::Conflict, RailAudience::side(alliance))
                    .with_display(MessageDisplay {
                        title,
                        body,
                        background,
                        overlay: 0,
                        sound,
                        voice: 0,
                    }),
            )
        })
        .collect()
}

/// Write the blockade events and show their messages: a blockade started
/// or lifted, and the regiments a run destroyed aboard a departing fleet;
/// then fire the support shifts posted on `tick`.
#[expect(
    clippy::too_many_arguments,
    reason = "The events, the world, the states a holder change moves, the blockade and the message templates and log."
)]
fn show_blockade_events(
    tick: u64,
    events: &[rebellion_core::blockade::BlockadeEvent],
    world: &mut GameWorld,
    manufacturing: &mut ManufacturingState,
    transport: &mut TroopTransportState,
    movement: &MovementState,
    deliveries: &mut DeliveryState,
    blockade: &BlockadeState,
    templates: &rebellion_data::text_templates::TextTemplates,
    log: &mut MessageLog,
) {
    // The regiments lost at each system on each tick, for one notice each.
    let mut lost: Vec<(SystemKey, u64, Vec<String>)> = Vec::new();
    for evt in events {
        rebellion_data::telemetry::native_blockade_event(world, blockade, evt);
        match evt {
            rebellion_core::blockade::BlockadeEvent::BlockadeStarted { .. } => {
                for message in blockade_transition_messages(world, templates, evt) {
                    log.push(message);
                }
            }
            rebellion_core::blockade::BlockadeEvent::BlockadeEnded { system, .. } => {
                rebellion_core::support_shift::blockade_ended(world, blockade, *system);
            }
            rebellion_core::blockade::BlockadeEvent::TroopDestroyed {
                system,
                troop,
                tick,
            } => {
                // The regiment was aboard a fleet leaving the system.
                let name = rebellion_core::scrap::target_name(
                    world,
                    rebellion_core::scrap::ScrapTarget::Troop(*troop),
                );
                if transport.destroy_embarked(world, *troop) {
                    rebellion_data::telemetry::native_troop_destroyed(*tick, world, *system);
                    match lost
                        .iter_mut()
                        .find(|(at, on, _)| at == system && on == tick)
                    {
                        Some((_, _, units)) => units.push(name),
                        None => lost.push((*system, *tick, vec![name])),
                    }
                }
            }
            // Timer 0x386 raises no notice (FUN_0050cb20).
            rebellion_core::blockade::BlockadeEvent::SupportShift {
                system,
                side,
                delta,
                tick,
            } => {
                rebellion_core::uprising::apply_support_change(world, *system, *side, *delta);
                rebellion_data::telemetry::native_support_shift(
                    *tick, world, *system, *side, *delta,
                );
            }
        }
    }
    for (system, on, units) in &lost {
        for message in evacuation_losses_messages(world, templates, *system, *on, units) {
            log.push(message);
        }
    }
    fire_support_events(
        world,
        manufacturing,
        transport,
        movement,
        deliveries,
        templates,
        log,
        tick,
        0,
    );
}

/// A message movie in the 640 by 480 movie window, with its caption field
/// (`FUN_00421790`): the body, wrapped, white, font 4. hyp: font 4 at entry
/// 10's size, as the Message Index body measures (`message_index.rs`).
fn draw_message_movie(frame: Option<&Texture2D>, caption: &str) {
    use egui_macroquad::egui;
    let screen = egui::Rect::from_min_size(
        egui::Pos2::ZERO,
        egui::vec2(screen_width(), screen_height()),
    );
    if let Some(frame) = frame {
        let (picture, _, _) =
            movies::message_layout(screen, egui::vec2(frame.width(), frame.height()));
        draw_texture_ex(
            frame,
            picture.min.x,
            picture.min.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(picture.width(), picture.height())),
                ..Default::default()
            },
        );
    }
    if caption.is_empty() {
        return;
    }
    egui_macroquad::ui(|ctx| {
        let (_, field, scale) = movies::message_layout(ctx.screen_rect(), egui::Vec2::ZERO);
        let painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            egui::Id::new("movie_caption"),
        ));
        let mut job = egui::text::LayoutJob::simple(
            caption.to_owned(),
            rebellion_render::theme::game_font_on(ctx, 10, scale),
            egui::Color32::WHITE,
            field.width(),
        );
        for section in &mut job.sections {
            section.format.line_height = Some(16.0 * scale);
        }
        let galley = painter.layout_job(job);
        painter
            .with_clip_rect(field)
            .galley(field.min, galley, egui::Color32::WHITE);
    });
    egui_macroquad::draw();
}

fn draw_fullscreen_texture(texture: &Texture2D) {
    let texture_width = texture.width();
    let texture_height = texture.height();
    if texture_width <= 0.0 || texture_height <= 0.0 {
        return;
    }

    let scale = (screen_width() / texture_width).min(screen_height() / texture_height);
    let dest_width = texture_width * scale;
    let dest_height = texture_height * scale;
    let x = (screen_width() - dest_width) * 0.5;
    let y = (screen_height() - dest_height) * 0.5;

    draw_texture_ex(
        texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(dest_width, dest_height)),
            ..Default::default()
        },
    );
}

/// File a report on the Message Index rail when some side receives it.
/// Which kind of galaxy child window is in front: the one whose window took
/// focus last (`rebellion_render::window_focus`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GalaxyFront {
    System,
    Fleet,
    Defenses,
    Missions,
}

#[expect(
    clippy::too_many_arguments,
    reason = "The lifecycle adapter carries the existing Fleet opener's complete content context."
)]
fn enqueue_fleet_window(
    strategic_windowing: &mut strategic_windowing::StrategicWindowing,
    state: &FleetWindowState,
    world: &GameWorld,
    system: SystemKey,
    logical_position: (i16, i16),
    faction: CockpitFaction,
    layout: rebellion_render::CockpitLayout,
    selected: Option<FleetWindowEntry>,
) -> bool {
    let Some(prepared) =
        state.prepare_open(world, system, logical_position, faction, layout, selected)
    else {
        return false;
    };
    strategic_windowing.enqueue_fleet_open(prepared);
    true
}

fn galaxy_front(
    system: &SystemWindowState,
    fleet_serial: u64,
    defenses: &DefensesWindowState,
    missions: &MissionsWindowState,
) -> GalaxyFront {
    [
        (system.focus_serial(), GalaxyFront::System),
        (fleet_serial, GalaxyFront::Fleet),
        (defenses.focus_serial(), GalaxyFront::Defenses),
        (missions.focus_serial(), GalaxyFront::Missions),
    ]
    .into_iter()
    .max_by_key(|&(serial, _)| serial)
    .map_or(GalaxyFront::System, |(_, front)| front)
}

fn filed(message: GameMessage, rail: MessageRail, audience: Option<RailAudience>) -> GameMessage {
    match audience {
        Some(audience) => message.on_rail(rail, audience),
        None => message,
    }
}

/// File advice topics in the player's Advice category (`FUN_0048a590`); each
/// carries advice code `0x2a` (`FUN_0048b2e0`).
fn file_advice_topics(
    msg_log: &mut MessageLog,
    topics: Vec<rebellion_render::advice_topics::TopicMessage>,
    tick: u64,
    player_is_alliance: bool,
) {
    for topic in topics {
        macroquad::logging::info!("[advice] topic filed: {}", topic.display.title);
        msg_log.push(
            GameMessage::new(tick, topic.display.title.clone(), MessageCategory::Event)
                .with_display(topic.display)
                .with_advice(0x2a)
                .on_rail(MessageRail::Advice, RailAudience::side(player_is_alliance)),
        );
    }
}

/// The side that controls `system`, whose Message Index receives its reports.
/// Notification 3, the Uprising Message (`FUN_00499460`), built for each
/// side: the original gives no side filter, only the advice code differs.
/// A system nobody holds files nothing (`+0x40 = 0`); the port still logs
/// `fallback` there, off the rail. The original also shows it only when
/// system flag `+0x22 & 4` agrees with `began`; hyp: that bit is the
/// uprising flag the event just changed, so the port's transition events
/// always pass it.
fn uprising_messages(
    world: &GameWorld,
    templates: &rebellion_data::text_templates::TextTemplates,
    system: SystemKey,
    tick: u64,
    began: bool,
    fallback: String,
) -> Vec<GameMessage> {
    use rebellion_data::text_templates::TemplateArg;
    let Some(sys) = world.systems.get(system) else {
        return Vec::new();
    };
    let holder_is_alliance = match sys.control.faction() {
        Some(Faction::Alliance) => true,
        Some(Faction::Empire) => false,
        _ => {
            return vec![GameMessage::at_system(
                tick,
                fallback,
                MessageCategory::Diplomacy,
                system,
            )];
        }
    };
    let place = TemplateArg::named(sys.name.clone());
    let holder = TemplateArg::side(holder_is_alliance);
    // Began: title 0x7030, body 0x7033 ("the |side controlled system |sys"),
    // picture 0x3f2, sound 0x454. Ended: 0x7031 / 0x7032, the holder's
    // picture 0x3f3 / 0x3f4, sound 0x455.
    let (title, body, background, sound) = if began {
        (0x7030, 0x7033, 0x3f2, 0x454)
    } else {
        let background = if holder_is_alliance { 0x3f3 } else { 0x3f4 };
        (0x7031, 0x7032, background, 0x455)
    };
    let display = templates
        .format(title, &[Some(&place)])
        .zip(templates.format(body, &[Some(&holder), Some(&place)]))
        .map(|(title, body)| MessageDisplay {
            title,
            body,
            background,
            overlay: 0,
            sound,
            voice: 0,
        });
    [true, false]
        .into_iter()
        .map(|side_is_alliance| {
            // +0x28: 1 when "this side holds the system" differs from
            // "the uprising began" (+0x64), else 2.
            let advice = if (side_is_alliance == holder_is_alliance) == began {
                2
            } else {
                1
            };
            let text = display
                .as_ref()
                .map_or_else(|| fallback.clone(), |display| display.title.clone());
            let message = GameMessage::at_system(tick, text, MessageCategory::Diplomacy, system)
                .with_advice(advice)
                .with_target(MessageTarget::System(system))
                .on_rail(
                    MessageRail::PopularSupport,
                    RailAudience::side(side_is_alliance),
                );
            match &display {
                Some(display) => message.with_display(display.clone()),
                None => message,
            }
        })
        .collect()
}

/// The Battle Alert's title (`0x7020`) and summary, chosen as `FUN_0049c0d0`
/// does from the system's holder and its blockade (`sys[0x1e]` naming the
/// holder): the player's own system threatened (`0x7021`) or blockaded
/// (`0x7022`), a neutral or contested one (`0x7025`), the enemy's blockaded
/// (`0x7024`), or the enemy's entered (`0x7023`). hyp: the port's blockade
/// state stands for `sys[0x1e]` naming the holder; that field is untraced.
fn battle_alert_text(
    world: &GameWorld,
    templates: &rebellion_data::text_templates::TextTemplates,
    blockaded: bool,
    system: SystemKey,
    player_is_alliance: bool,
) -> Option<(String, String)> {
    use rebellion_data::text_templates::TemplateArg;
    let sys = world.systems.get(system)?;
    let player = TemplateArg::side(player_is_alliance);
    let enemy = TemplateArg::side(!player_is_alliance);
    let place = TemplateArg::place(sys.name.clone());
    let holder = match sys.control.faction() {
        Some(Faction::Alliance) => Some(true),
        Some(Faction::Empire) => Some(false),
        _ => None,
    };
    let (id, args) = match holder {
        Some(holder) if holder == player_is_alliance && blockaded => {
            (0x7022, [&player, &place, &enemy])
        }
        Some(holder) if holder == player_is_alliance => (0x7021, [&enemy, &place, &player]),
        None => (0x7025, [&player, &enemy, &place]),
        Some(_) if blockaded => (0x7024, [&enemy, &player, &place]),
        Some(_) => (0x7023, [&player, &place, &enemy]),
    };
    let title = templates.format(0x7020, &[Some(&place)])?;
    let summary = templates.format(id, &args.map(Some))?;
    Some((title, summary))
}

/// Notification 7, the Blockade Message (`FUN_004960f0`, event `0x14e`),
/// built for each side. The holder reads "| Under Blockade" (`0x70a2` /
/// `0x70a3`, code `0xe`), the other side "Fleet Initiates Blockade of |"
/// (`0x70a0` / `0x70a1`, code `0xd`); both show the holder's picture (`0x404`
/// Alliance, `0x403` Empire) and play `0x45e`. A neutral system, or one with
/// no blockading fleet (`FUN_0052bed0`), files nothing, as does a side
/// without the TEXTSTRA entries. The original also files nothing when
/// system flag `+0x22 & 0x4000` is set; port: that flag is untraced and not
/// checked.
fn blockade_messages(
    world: &GameWorld,
    templates: &rebellion_data::text_templates::TextTemplates,
    system: SystemKey,
    tick: u64,
) -> Vec<GameMessage> {
    use rebellion_data::text_templates::TemplateArg;
    let Some(sys) = world.systems.get(system) else {
        return Vec::new();
    };
    let holder_is_alliance = match sys.control.faction() {
        Some(Faction::Alliance) => Some(true),
        Some(Faction::Empire) => Some(false),
        _ => None,
    };
    // port: the first enemy fleet at the system stands for FUN_0052bed0's.
    let blockader = holder_is_alliance.and_then(|holder| {
        world
            .fleets
            .iter()
            .find(|(_, fleet)| fleet.location == system && fleet.is_alliance != holder)
            .map(|(key, _)| world.fleet_name(key).unwrap_or_default().to_owned())
    });
    let (Some(holder_is_alliance), Some(blockader)) = (holder_is_alliance, blockader) else {
        return Vec::new();
    };
    let place = TemplateArg::named(sys.name.clone());
    let enemy = TemplateArg::side(!holder_is_alliance);
    let fleet = TemplateArg::named(blockader);
    let background = if holder_is_alliance { 0x404 } else { 0x403 };
    [true, false]
        .into_iter()
        .filter_map(|side_is_alliance| {
            let holds = side_is_alliance == holder_is_alliance;
            let (title, body, advice) = if holds {
                (0x70a2, 0x70a3, 0xe)
            } else {
                (0x70a0, 0x70a1, 0xd)
            };
            let title = templates.format(title, &[Some(&place)])?;
            let body = templates.format(body, &[Some(&enemy), Some(&place), Some(&fleet)])?;
            Some(
                GameMessage::at_system(tick, title.clone(), MessageCategory::Combat, system)
                    .with_advice(advice)
                    .with_target(MessageTarget::System(system))
                    .on_rail(MessageRail::Conflict, RailAudience::side(side_is_alliance))
                    .with_display(MessageDisplay {
                        title,
                        body,
                        background,
                        overlay: 0,
                        sound: 0x45e,
                        voice: 0,
                    }),
            )
        })
        .collect()
}

/// Notification 0xd, Unit Arrival (`FUN_004981c0`, case 1): a fleet's
/// arrival, filed on the Fleet rail for the arriving side with code 5. The
/// original shows it only when the arriving unit (`piVar4`, the player's
/// view of `param_3`) is the player's own (`piVar4[9] & 0xc0`).
fn fleet_arrival_message(
    templates: &rebellion_data::text_templates::TextTemplates,
    fleet: FleetKey,
    fleet_name: &str,
    system: SystemKey,
    sys_name: &str,
    tick: u64,
    alliance: bool,
) -> GameMessage {
    let display = fleet_arrival_display(templates, fleet_name, sys_name, alliance);
    let message = GameMessage::at_system(
        tick,
        display.as_ref().map_or_else(
            || format!("Fleet arrived at {sys_name}"),
            |d| d.title.clone(),
        ),
        MessageCategory::Mission,
        system,
    )
    .with_advice(5)
    .with_target(MessageTarget::Fleet(fleet));
    filed(
        match display {
            Some(display) => message.with_display(display),
            None => message,
        },
        MessageRail::Fleet,
        Some(RailAudience::side(alliance)),
    )
}

/// `FUN_004981c0`, case 1 (an arriving fleet, type `0x08..0x0f`): "Fleet
/// Arrives at |" (`0x7078`) and "| has arrived at |." (`0x7079`) over the
/// receiving side's picture (`0x3fa` Alliance, `0x3fb` Empire), playing
/// `0x45a`. Only the arriving side files it.
fn fleet_arrival_display(
    templates: &rebellion_data::text_templates::TextTemplates,
    fleet: &str,
    system: &str,
    alliance: bool,
) -> Option<MessageDisplay> {
    use rebellion_data::text_templates::TemplateArg;
    let fleet = TemplateArg::named(fleet);
    let place = TemplateArg::named(system);
    let args = [Some(&fleet), Some(&place)];
    Some(MessageDisplay {
        title: templates.format(0x7078, &args)?,
        body: templates.format(0x7079, &args)?,
        background: if alliance { 0x3fa } else { 0x3fb },
        overlay: 0,
        sound: 0x45a,
        voice: 0,
    })
}

/// Notification `0xc`, the Maintenance Shortfall notice (`FUN_00498970`,
/// event `0x304`, which `FUN_004fc080` raises for each object scrapped with
/// reason `0x15`): "Maintenance Shortfall" (`0x7080`), "The following units
/// have been destroyed at | due to a maintenance shortfall:" (`0x7081`, the
/// system as argument 3) and a line per unit (`0x7082`), over the side's
/// picture (`0x3f5` Alliance, `0x3f6` Empire), playing `0x456`
/// (`FUN_0048bbf0.c:17-24`). One notice per scrapped unit, filed for its
/// side. hyp: whether the original merges a system's units into one notice
/// is untraced; the overdraft timer scraps one unit a fire.
fn maintenance_shortfall_message(
    world: &GameWorld,
    transport: &TroopTransportState,
    templates: &rebellion_data::text_templates::TextTemplates,
    side: rebellion_core::dat::Faction,
    target: rebellion_core::scrap::ScrapTarget,
    tick: u64,
) -> Option<GameMessage> {
    use rebellion_data::text_templates::TemplateArg;
    let system = rebellion_core::scrap::target_system(world, transport, target)?;
    let sys_name = world.systems.get(system)?.name.clone();
    let unit = rebellion_core::scrap::target_name(world, target);
    let alliance = side == rebellion_core::dat::Faction::Alliance;
    let place = TemplateArg::named(sys_name.clone());
    let named = TemplateArg::named(unit.clone());
    let display = templates.format(0x7080, &[]).and_then(|title| {
        Some(MessageDisplay {
            title,
            body: templates.format(0x7081, &[None, None, Some(&place)])?
                + &templates.format(0x7082, &[Some(&named)])?,
            background: if alliance { 0x3f5 } else { 0x3f6 },
            overlay: 0,
            sound: 0x456,
            voice: 0,
        })
    });
    let message = GameMessage::at_system(
        tick,
        format!("{unit} destroyed at {sys_name} due to a maintenance shortfall"),
        MessageCategory::Event,
        system,
    )
    .with_advice(0xc);
    Some(filed(
        match display {
            Some(display) => message.with_display(display),
            None => message,
        },
        MessageRail::Manufacturing,
        Some(RailAudience::side(alliance)),
    ))
}

/// The Loyalty Change Messages of a holder change ([`loyalty_messages`]):
/// only a change of side is a loyalty change, and a contested system has
/// not declared neutrality. A change with regiment bits in either control
/// word files none: `FUN_00510d70.c:12-18` clears the loyalty bit
/// (`+0x88` `0x2000`) before the owner write posts notification `0x100`,
/// and `FUN_00499760.c:23-29` files only with the bit set.
fn control_change_messages(
    world: &GameWorld,
    templates: &rebellion_data::text_templates::TextTemplates,
    tick: u64,
    change: &rebellion_core::support_shift::ControlRerun,
) -> Vec<GameMessage> {
    let side = |faction: Option<Faction>| match faction {
        Some(Faction::Alliance) => Some(true),
        Some(Faction::Empire) => Some(false),
        _ => None,
    };
    let (old, new) = (
        side(change.previous.faction()),
        side(change.control.faction()),
    );
    let (old_troops, new_troops) = change.troops;
    if new == old || old_troops || new_troops {
        return Vec::new();
    }
    loyalty_messages(world, templates, change.system, tick, new, old)
}

/// Phase 5 of in-day step `step` of day `tick` (`FUN_0051df30`): serve the
/// support shifts, regiment reruns and holder changes the operation raised
/// (`support_shift::fire`), record them, and file the Loyalty Change
/// Messages of each holder change without regiments
/// ([`control_change_messages`]). The day's own work runs
/// in its first step (0); a player's order in the scheduler's next step.
/// Each holder change's autoscrap requests for what is bound for the
/// system follow (event `0x307`,
/// `rebellion_core::death_star::serve_holder_requests`).
#[expect(
    clippy::too_many_arguments,
    reason = "Phase 5 reads the world, the states a holder change moves, the message templates and log, and the step."
)]
fn fire_support_events(
    world: &mut GameWorld,
    manufacturing: &mut ManufacturingState,
    transport: &mut TroopTransportState,
    movement: &MovementState,
    deliveries: &mut DeliveryState,
    templates: &rebellion_data::text_templates::TextTemplates,
    log: &mut MessageLog,
    tick: u64,
    step: u16,
) {
    let fired = rebellion_data::integrator::fire_holder_changes(
        world,
        manufacturing,
        rebellion_core::support_shift::step(tick, step),
    );
    rebellion_core::death_star::serve_holder_requests(
        world,
        movement,
        transport,
        manufacturing,
        deliveries,
    );
    rebellion_data::telemetry::native_support_shifts(tick, world, &fired);
    for change in &fired.controls {
        for message in control_change_messages(world, templates, tick, change) {
            log.push(message);
        }
    }
}

/// The characters the operation's holder changes flagged leave (event
/// `0x307`, `integrator::sweep_requests`), with their trips' records. The
/// original files no message for them (`FUN_00536ac0` only logs).
fn sweep_flagged(
    world: &mut GameWorld,
    missions: &mut MissionState,
    transport: &TroopTransportState,
    movement: &MovementState,
    blockade: &BlockadeState,
    tick: u64,
) {
    let swept = rebellion_data::integrator::sweep_requests(
        world, missions, transport, movement, blockade, tick,
    );
    rebellion_data::telemetry::native_swept(tick, world, &swept);
}

/// Notification 4, the Loyalty Change Message (`FUN_00499760`, event
/// `0x100`), built for each side over the crowd picture `0x3ed`. The side
/// the world joins reads "| Joins" (`0x7038` / `0x703b`, code 1), the other
/// "| Joins Enemy" (`0x7039` / `0x703c`, code 2), both playing `0x450`; a
/// world turning neutral "| Declares Neutrality" (`0x703a` / `0x703d`,
/// naming the side it left), playing `0x451` with no code. It files for
/// each holder change whose loyalty bit (`+0x88` `0x2000`) is set
/// ([`control_change_messages`]).
fn loyalty_messages(
    world: &GameWorld,
    templates: &rebellion_data::text_templates::TextTemplates,
    system: SystemKey,
    tick: u64,
    new_side_is_alliance: Option<bool>,
    old_side_is_alliance: Option<bool>,
) -> Vec<GameMessage> {
    use rebellion_data::text_templates::TemplateArg;
    let Some(sys) = world.systems.get(system) else {
        return Vec::new();
    };
    let place = TemplateArg::named(sys.name.clone());
    [true, false]
        .into_iter()
        .filter_map(|side_is_alliance| {
            let (title, body, side, sound, advice) = match new_side_is_alliance {
                Some(new) if new == side_is_alliance => (0x7038, 0x703b, new, 0x450, Some(1)),
                Some(new) => (0x7039, 0x703c, new, 0x450, Some(2)),
                None => (0x703a, 0x703d, old_side_is_alliance?, 0x451, None),
            };
            let side = TemplateArg::side(side);
            let args = [Some(&place), Some(&side)];
            let display = templates
                .format(title, &args)
                .zip(templates.format(body, &args))
                .map(|(title, body)| MessageDisplay {
                    title,
                    body,
                    background: 0x3ed,
                    overlay: 0,
                    sound,
                    voice: 0,
                });
            let text = display.as_ref().map_or_else(
                || format!("Loyalty change at {}", sys.name),
                |display| display.title.clone(),
            );
            let message = GameMessage::at_system(tick, text, MessageCategory::Diplomacy, system)
                .with_target(MessageTarget::System(system))
                .on_rail(
                    MessageRail::PopularSupport,
                    RailAudience::side(side_is_alliance),
                );
            let message = match display {
                Some(display) => message.with_display(display),
                None => message,
            };
            Some(match advice {
                Some(code) => message.with_advice(code),
                None => message,
            })
        })
        .collect()
}

fn system_audience(world: &GameWorld, system: SystemKey) -> Option<RailAudience> {
    match world.systems.get(system)?.control.faction()? {
        Faction::Alliance => Some(RailAudience::Alliance),
        Faction::Empire => Some(RailAudience::Empire),
        Faction::Neutral => None,
    }
}

/// The six major characters by MJCHARSD id (`& 0xffffff`): Mon Mothma,
/// Leia, Luke, Han, the Emperor, Vader.
const MAJOR_CHARACTER_IDS: [u32; 6] = [0x240, 0x241, 0x242, 0x243, 0x280, 0x281];

fn major_character_slot(world: &GameWorld, character: CharacterKey) -> Option<usize> {
    let id = world.characters.get(character)?.dat_id.raw() & 0xff_ffff;
    MAJOR_CHARACTER_IDS.iter().position(|&major| major == id)
}

/// The advice code of a message about `character`'s doings: its own for a
/// major character, else `0x15` (`FUN_00491500`, `FUN_004955b0`).
fn personnel_advice(world: &GameWorld, character: CharacterKey) -> u8 {
    major_character_slot(world, character)
        .map_or(0x15, |slot| [0x17, 0x18, 0x16, 0x19, 0x1a, 0x1b][slot])
}

/// The advice code of `character`'s escape for the side `own` or not
/// (`FUN_00490340.c:207-244`, `:297-319`): a major's own code; another
/// major's none; a minor's `0x15` for its own side, `0x24` for the other.
fn escape_advice(world: &GameWorld, character: CharacterKey, own: bool) -> Option<u8> {
    if let Some(slot) = major_character_slot(world, character) {
        return Some([0x26, 0x18, 0x25, 0x19, 0x27, 0x28][slot]);
    }
    let major = world.characters.get(character)?.is_major;
    (!major).then_some(if own { 0x15 } else { 0x24 })
}

/// Where a prisoner is held, for its release message: its jail, its fleet's
/// system, or its system.
fn prison_name(world: &GameWorld, character: CharacterKey) -> Option<String> {
    let value = world.characters.get(character)?;
    if let Some(abode) = value.held_abode() {
        return Some(abode.name().to_owned());
    }
    let system = value
        .current_fleet
        .and_then(|fleet| world.fleets.get(fleet))
        .map(|fleet| fleet.location)
        .or(value.current_system)?;
    world.systems.get(system).map(|sys| sys.name.clone())
}

/// Notification `0x1e0` on a prisoner's status write (`FUN_004f1fd0`),
/// filed by `FUN_00490340` for an escape (status 1) and a release (status
/// 2) alike: to its own side "escaped from the {captor} at {location}"
/// (`0x715c`/`0x715d`), to the other "has escaped from imprisonment at
/// {location}" (`0x715e`/`0x715f`), picture `0x405`, sound `0x469`.
/// hyp: both sides hold a view of every character (the view gate,
/// `FUN_004fd3d0`, is untraced); the rail of class `0x19` is untraced, so
/// it files on the Mission rail with the port's other personnel reports.
fn release_messages(
    world: &GameWorld,
    templates: &rebellion_data::text_templates::TextTemplates,
    tick: u64,
    character: CharacterKey,
    location: &str,
    voice: u16,
) -> Vec<GameMessage> {
    use rebellion_data::text_templates::TemplateArg;
    let Some(value) = world.characters.get(character) else {
        return Vec::new();
    };
    let who = TemplateArg::named(value.display_name().to_owned());
    let place = TemplateArg::place(location.to_owned());
    let captor = TemplateArg::side(!value.is_alliance);
    [true, false]
        .into_iter()
        .filter_map(|own| {
            let (title, body) = if own {
                (0x715c, 0x715d)
            } else {
                (0x715e, 0x715f)
            };
            let args = [Some(&who), Some(&place), Some(&captor)];
            // The message has only TEXTSTRA's text; without it, none.
            let title = templates.format(title, &args)?;
            let body = templates.format(body, &args)?;
            let audience = RailAudience::side(value.is_alliance == own);
            let message = GameMessage::new(tick, body.clone(), MessageCategory::Event)
                .on_rail(MessageRail::Mission, audience);
            let message = match escape_advice(world, character, own) {
                Some(advice) => message.with_advice_to(advice, audience),
                None => message,
            };
            // FUN_00490340.c:207-244: the escapee's own side hears its
            // voice line; the other side none.
            Some(message.with_display(MessageDisplay {
                title,
                body,
                background: 0x405,
                overlay: 0,
                sound: 0x469,
                voice: if own { voice } else { 0 },
            }))
        })
        .collect()
}

/// A character's message portrait (`FUN_004c5000`): STRATEGY
/// `(class & 0xfff) + 0x1000`, the same class bits as its GOKRES mini at
/// `+ 0x4000` (STRATEGY 6208-6211, 6224, 6272-6273 are the majors, 6720
/// on the minors).
fn character_message_portrait(world: &GameWorld, character: CharacterKey) -> Option<u16> {
    let value = world.characters.get(character)?;
    let mini = rebellion_core::tactical::character_mini_resource_id(value.dat_id, value.is_major)?;
    u16::try_from(mini.checked_sub(0x3000)?).ok()
}

/// A message picture STRATEGY lacks, from its ENCYBMAP EDATA file
/// (`FUN_0046a280` -> `FUN_0045f970`): the staged ENCYBMAP string table's
/// file name for `id`, read from the Encyclopedia session.
fn encyclopedia_picture(
    session: Option<&rebellion_data::encyclopedia_session::EncyclopediaSession>,
    id: u16,
) -> Option<Vec<u8>> {
    let session = session?;
    let file = session.source_catalog().artwork.get(&id)?;
    session.artwork_bytes(file).map(<[u8]>::to_vec)
}

/// A special force's message portrait (`FUN_004c5000` over slot `+0x30`,
/// `FUN_004f63d0`): STRATEGY `(TEXTSTRA id & 0xfff) + 0x1000`, its GOKRES
/// mini less `0x3000` (5440-5443, 5504-5508). STRATEGY holds none of them;
/// the picture loader `FUN_0046a280` falls back to ENCYBMAP's EDATA files.
fn special_force_message_portrait(
    world: &GameWorld,
    unit: rebellion_core::ids::SpecialForceKey,
) -> Option<u16> {
    let class = world.special_forces.get(unit)?.class_dat_id;
    let (mini, _) = rebellion_render::system_window::special_force_mini(class)?;
    u16::try_from(mini.checked_sub(0x3000)?).ok()
}

/// Where an ended mission's personnel go (`FUN_00491500.c:141-195`): the
/// first team member still alive and free, at the system its Return heads
/// for, else its fleet, else its system; `None` when none is left.
/// port: the Return is queued, so its destination stands for the
/// container the original reparents the member into at once.
fn mission_returning_place(
    world: &GameWorld,
    missions: &MissionState,
    ended: &rebellion_core::missions::MissionEnded,
    started: &[u64],
) -> Option<String> {
    ended.team.iter().find_map(|&member| {
        let fleet = match member {
            MissionMember::Character(key) => {
                let value = world.characters.get(key).filter(|c| !c.is_killed && !c.is_captive)?;
                value.current_fleet
            }
            MissionMember::SpecialForce(key) => {
                world.special_forces.get(key)?;
                None
            }
        };
        let returning = missions
            .missions()
            .iter()
            .find(|mission| started.contains(&mission.id) && mission.team.contains(&member))
            .map(|mission| mission.target_system);
        let name = match (returning, fleet) {
            (Some(system), _) => world.systems.get(system).map(|sys| sys.name.clone()),
            (None, Some(fleet)) => world.fleet_name(fleet).map(str::to_owned),
            (None, None) => rebellion_core::missions::member_location(world, member)
                .and_then(|system| world.systems.get(system))
                .map(|sys| sys.name.clone()),
        };
        Some(name)
    })?
}

/// The window a character's or special force's Display opens
/// (`FUN_00429440`, families `0x30`-`0x3f`): aboard, its fleet's Fleet
/// window; at a system, the Missions window while on a mission that is not
/// hidden (`FUN_00520b70`), else the Defenses window; in an abode, none.
/// Where the Operation Report's Display buttons go (`FUN_0044df90` cases
/// `0xb`/`0xc`): `FUN_0045c8e0(sector window, system, kind)`. The system
/// button opens the sector window; the fleet button, kind 4, its top-right
/// Fleet icon (`FUN_0045aac0`), which opens the Fleet window with nothing
/// selected and does not look for the report's fleets.
fn report_destination(action: &OperationReportAction) -> Option<(SystemKey, FinderTarget)> {
    match *action {
        OperationReportAction::GotoSystem { system } => Some((system, FinderTarget::Sector)),
        OperationReportAction::GotoFleet { system } => Some((system, FinderTarget::FleetWindow)),
        OperationReportAction::Closed { .. } => None,
    }
}

#[cfg(test)]
mod report_destination_tests {
    use super::*;

    #[test]
    fn the_reports_fleet_button_opens_the_systems_fleet_window_with_nothing_selected() {
        // FUN_0044df90 case 0xc -> FUN_0045c8e0(.., 4) -> FUN_0045aac0: the
        // system's Fleet window, no fleet selected; case 0xb the sector.
        let mut world = GameWorld::default();
        let system = world.systems.insert(rebellion_core::world::System::default());
        assert_eq!(
            report_destination(&OperationReportAction::GotoSystem { system }),
            Some((system, FinderTarget::Sector))
        );
        assert_eq!(
            report_destination(&OperationReportAction::GotoFleet { system }),
            Some((system, FinderTarget::FleetWindow))
        );
        assert_eq!(
            report_destination(&OperationReportAction::Closed { reopen_index: None }),
            None
        );
    }
}

fn personnel_window(
    world: &GameWorld,
    movement: &MovementState,
    member: MissionMember,
) -> Option<(SystemKey, FinderTarget)> {
    use rebellion_render::personnel_finder::{character_target, CharacterTarget};
    match member {
        MissionMember::Character(key) => {
            let value = world.characters.get(key)?;
            if value.held_abode().is_some() {
                return None;
            }
            match character_target(value) {
                CharacterTarget::Fleet(fleet) => {
                    rebellion_core::movement::listed_location(movement, world, fleet)
                        .map(|system| (system, FinderTarget::Fleet(fleet)))
                }
                CharacterTarget::Missions => value
                    .current_system
                    .map(|system| (system, FinderTarget::Missions(member))),
                CharacterTarget::Defenses => value
                    .current_system
                    .map(|system| (system, FinderTarget::Defenses)),
            }
        }
        MissionMember::SpecialForce(key) => {
            let unit = world.special_forces.get(key)?;
            let system = rebellion_core::missions::member_location(world, member)?;
            let target = if unit.on_mission && !unit.on_hidden_mission {
                FinderTarget::Missions(member)
            } else {
                FinderTarget::Defenses
            };
            Some((system, target))
        }
    }
}

/// A major character's voice lists for one message class, the module 9
/// RCDATA that `FUN_004c4990` draws `+0x32` from, by record: Mon Mothma,
/// Leia, Luke, Luke with `+0xb0` bit 3 (his Dagobah training done), Han,
/// the Emperor, Vader.
type MajorVoices = [u16; 7];

/// The end-code message's lists (`FUN_00491500.c:214-256`).
const MISSION_END_VOICES: MajorVoices = [0xf3, 0xdf, 0xcb, 0xd5, 0xe9, 0x107, 0xfd];

/// The escape message's lists, own side (`FUN_00490340.c:207-244`).
const RELEASE_VOICES: MajorVoices = [0xf6, 0xe2, 0xce, 0xd8, 0xec, 0x10a, 0x100];

/// `character`'s voice list in `lists`; none for a minor character.
/// port: Luke's `+0xb0` bit 3 is the story event `0x210`
/// (`EVT_DAGOBAH_COMPLETED`) having fired.
fn major_voice_list(
    world: &GameWorld,
    character: CharacterKey,
    luke_trained: bool,
    lists: MajorVoices,
) -> Option<u16> {
    let [mothma, leia, luke, trained, han, emperor, vader] = lists;
    let luke = if luke_trained { trained } else { luke };
    major_character_slot(world, character).map(|slot| [mothma, leia, luke, han, emperor, vader][slot])
}

/// The voice list of a mission end's lead (`FUN_00491500.c:214-256`); none
/// for a special force or a minor character.
fn mission_end_voice_list(
    world: &GameWorld,
    ended: &rebellion_core::missions::MissionEnded,
    luke_trained: bool,
) -> Option<u16> {
    let MissionMember::Character(lead) = ended.lead? else {
        return None;
    };
    major_voice_list(world, lead, luke_trained, MISSION_END_VOICES)
}

/// Notification `0x1f1` on a mission's end code (`FUN_00521900` ->
/// `FUN_00523ef0` -> `FUN_00525070`), filed by `FUN_00491500` on the
/// Mission rail (`mission-lifecycle.md`, "End-code messages"). The
/// mission's side hears from its lead: codes 3-5 "Mission Foiled"
/// (`0x7148`-`0x714b`, picture `0x416`/`0x417`, sound `0x480`), 6, 7, 9
/// and 10 "Mission Failed" (`0x7138`-`0x713b`, none for Research), 8
/// "Mission Aborted" (`0x7140`-`0x7145`) and `0xc` Research's "no research
/// facilities" (`0x713c`/`0x713d`), first person from a major character;
/// each body ends with `0x70e6` "Personnel are returning to …" or `0x70e7`.
/// The defender hears "Enemy Mission Foiled" (`0x713e`/`0x713f`, picture
/// `0x431`/`0x432`) only when a detection ended it, codes 3 and 4
/// (`FUN_00547f60.c:71-95`).
/// port: the subject key (`+0x5c`) is the target system, as the port's
/// messages open systems and fleets only; a special force lead's portrait
/// is not resolved, so it keeps the base picture.
fn mission_end_messages(
    world: &GameWorld,
    templates: &rebellion_data::text_templates::TextTemplates,
    ended: &rebellion_core::missions::MissionEnded,
    returning_to: Option<&str>,
    voice: u16,
) -> Vec<GameMessage> {
    use rebellion_data::text_templates::TemplateArg;
    // FUN_00488de0.c:317-330: the class (0x17) only for a mission outside
    // families 0x40-0x4f, whose record is not hidden (FUN_00520b70) and
    // whose result +0x60 is 0.
    let listed = ended.kind.record_id().is_some_and(|id| {
        !(0x40..0x50).contains(&id.family())
            && !world.mission_record(id).is_some_and(|record| record.hidden)
    });
    if !listed || ended.result != 0 {
        return Vec::new();
    }
    let Some(sys) = world.systems.get(ended.target_system) else {
        return Vec::new();
    };
    let is_alliance = ended.faction == MissionFaction::Alliance;
    let side_offset = |alliance: bool| u16::from(!alliance);
    let mission = TemplateArg::named(ended.kind.name());
    let place = TemplateArg::named(sys.name.clone());
    let mut out = Vec::new();
    let file = |title: String, body: String, audience, display: (u16, u16, u16, u16), subject| {
        let (background, overlay, sound, voice) = display;
        GameMessage::at_system(ended.tick, body.clone(), MessageCategory::Mission, ended.target_system)
            .with_target(subject)
            .on_rail(MessageRail::Mission, audience)
            .with_display(MessageDisplay {
                title,
                body,
                background,
                overlay,
                sound,
                voice,
            })
    };

    // The mission's side (`FUN_00491500.c:109-650`), from its lead.
    let lead = ended.lead.and_then(|member| match member {
        MissionMember::Character(key) => world
            .characters
            .get(key)
            .filter(|c| !c.is_killed)
            .map(|c| (c.display_name().to_owned(), Some(key), c.is_major)),
        MissionMember::SpecialForce(key) => world
            .special_forces
            .get(key)
            .and_then(|unit| world.buildable_classes.get(&unit.class_dat_id))
            .map(|class| (class.name.clone(), None, false)),
    });
    if let Some((name, character, major)) = lead {
        let research = matches!(ended.kind, MissionKind::Research(_));
        let pick = |first: (u16, u16), third: (u16, u16)| if major { first } else { third };
        let holder = rebellion_core::bombardment::holder(world, ended.target_system);
        let (ids, foiled) = match ended.end_code {
            3..=5 => (Some(pick((0x7148, 0x7149), (0x714a, 0x714b))), true),
            6 | 7 | 9 | 10 if !research => (Some(pick((0x7138, 0x7139), (0x713a, 0x713b))), false),
            8 if holder == Faction::Neutral => (Some(pick((0x7140, 0x7144), (0x7142, 0x7145))), false),
            8 => (Some(pick((0x7140, 0x7141), (0x7142, 0x7143))), false),
            0xc if research => (Some(pick((0x7138, 0x713c), (0x713a, 0x713d))), false),
            _ => (None, false),
        };
        let leader = TemplateArg::named(name);
        let joined = TemplateArg::side(holder == Faction::Alliance);
        let args = [Some(&leader), Some(&mission), Some(&place), Some(&joined)];
        let suffix = match returning_to {
            Some(at) => templates.format(0x70e6, &[Some(&TemplateArg::named(at.to_owned()))]),
            None => templates.format(0x70e7, &[]),
        };
        if let Some((title, body, suffix)) = ids.and_then(|(title, body)| {
            Some((templates.format(title, &args)?, templates.format(body, &args)?, suffix?))
        }) {
            // :273-313: the base picture; a character lead's portrait is the
            // overlay, any other lead's the background; codes 3-5 take
            // 0x416/0x417 for the background (no Reconnaissance kind).
            let mut background = 0x412 + side_offset(is_alliance);
            let mut overlay = 0;
            match ended.lead {
                Some(MissionMember::Character(key)) => {
                    overlay = character_message_portrait(world, key).unwrap_or(0);
                }
                Some(MissionMember::SpecialForce(unit)) => {
                    background = special_force_message_portrait(world, unit).unwrap_or(background);
                }
                None => {}
            }
            if foiled {
                background = 0x416 + side_offset(is_alliance);
            }
            let base = background;
            let sound = if foiled { 0x480 } else { 0x47c + side_offset(is_alliance) };
            let audience = RailAudience::side(is_alliance);
            // The subject key +0x5c: the lead unless it is a prisoner
            // (+0xac bit 0), else the target system (:128-137).
            let free = character
                .and_then(|key| world.characters.get(key))
                .is_none_or(|c| !c.is_captive);
            let subject = match ended.lead {
                Some(lead) if free => MessageTarget::Personnel(lead),
                _ => MessageTarget::System(ended.target_system),
            };
            let message = file(
                title,
                body + &suffix,
                audience,
                (base, overlay, sound, voice),
                subject,
            );
            out.push(match character {
                Some(key) => message.with_advice_to(personnel_advice(world, key), audience),
                None => message,
            });
        }
    }

    // The defending side (`FUN_00491500.c:87-105`), only after a detection.
    if matches!(ended.end_code, 3 | 4) {
        let args = [Some(&mission), Some(&place)];
        if let (Some(title), Some(body)) =
            (templates.format(0x713e, &args), templates.format(0x713f, &args))
        {
            let defender = !is_alliance;
            out.push(file(
                title,
                body,
                RailAudience::side(defender),
                (0x431 + side_offset(defender), 0, 0x47c + side_offset(defender), 0),
                MessageTarget::System(ended.target_system),
            ));
        }
    }
    out
}

/// The side a character serves, whose Message Index receives its reports.
/// Load the Message Index's sounds from the staged STRATEGY WAVEs; the count
/// loaded.
fn load_message_sounds(engine: &mut audio::AudioEngine, ui_root: &Path) -> usize {
    audio::MESSAGE_SOUNDS
        .iter()
        .filter_map(|&id| {
            let bytes =
                rebellion_render::staged_wave(ui_root, "strategy-dll", u16::try_from(id).ok()?)?;
            engine.load_message_sound_bytes(id, &bytes);
            Some(id)
        })
        .count()
}

/// The waves a shown message starts after stopping the last one's
/// (`FUN_00469de0`, `FUN_0046a6e0`): `+0x30`, a STRATEGY WAVE, and `+0x32`,
/// the key of a WAVE of the player's sprite DLL; each only when non-zero.
fn message_waves(sounds: MessageSounds, advisor: &AdvisorState) -> (Option<u32>, Option<String>) {
    (
        (sounds.sound != 0).then_some(u32::from(sounds.sound)),
        (sounds.voice != 0).then(|| advisor.sprite_voice(sounds.voice).key),
    )
}

/// Play the waves of the message the Message Index just showed
/// (`FUN_00469de0` → `FUN_00610c30`), the last message's stopped first
/// (`FUN_0046a6e0`). The sprite DLL's sounds are the droids'
/// (`play_advisor_voices` loads them).
fn play_message_sound(
    engine: &mut audio::AudioEngine,
    sounds: Option<MessageSounds>,
    advisor: &AdvisorState,
    vol: &AudioVolumeState,
) {
    let Some(sounds) = sounds else {
        return;
    };
    engine.stop_message_sounds();
    let (sound, voice) = message_waves(sounds, advisor);
    if let Some(sound) = sound {
        if engine.play_message_sound(sound, vol) {
            macroquad::logging::info!(
                "[message] sound 0x{:x} started volume={:.2}",
                sound,
                vol.effective_sfx_volume()
            );
        } else {
            macroquad::logging::warn!("[message] sound 0x{:x} is not staged", sound);
        }
    }
    if let Some(voice) = voice {
        if engine.play_message_voice(&voice, vol) {
            macroquad::logging::info!("[message] voice {} started", voice);
        } else {
            macroquad::logging::warn!("[message] voice {} is not staged", voice);
        }
    }
}

/// Load the side's droid sounds when the side changes, then play the ones the
/// droids started (command 4, `FUN_00403f70`). Loading ahead lets the
/// browser finish its asynchronous decode before the first play.
fn play_advisor_voices(
    engine: &mut audio::AudioEngine,
    advisor: &mut AdvisorState,
    loaded_side: &mut Option<AdvisorFaction>,
    vol: &AudioVolumeState,
) {
    if *loaded_side != Some(advisor.faction) {
        *loaded_side = Some(advisor.faction);
        for clip in advisor.voice_clips() {
            if !engine.has_advisor_voice(&clip.key) {
                if let Some(bytes) = advisor.voice_bytes(&clip) {
                    engine.load_advisor_voice_bytes(&clip.key, &bytes);
                }
            }
        }
    }
    for voice in advisor.take_voices() {
        if engine.play_advisor_voice(&voice.key, vol) {
            macroquad::logging::info!("[advisor] sound {} played", voice.key);
        } else {
            macroquad::logging::warn!("[advisor] sound {} is not staged", voice.key);
        }
    }
}

/// Carry out briefing step `step` on the cockpit (`FUN_004c30c0`,
/// `FUN_004c0fc0`).
fn apply_briefing_step(
    step: u32,
    world: &GameWorld,
    cockpit: &mut CockpitState,
    pending_index_command: &mut Option<u16>,
    advice: bool,
) {
    use rebellion_render::briefing_tour::{resolve_highlight, tour_step, TourStep};
    let Some(action) = tour_step(cockpit.faction, step) else {
        return;
    };
    match action {
        TourStep::Display(mode) => {
            cockpit.gid_mode = mode;
            cockpit.gid_highlight = rebellion_render::GidHighlight::default();
        }
        TourStep::Highlight {
            mode,
            target,
            caption,
        } => {
            cockpit.gid_mode = mode;
            cockpit.gid_highlight = resolve_highlight(world, cockpit.faction, target, caption);
        }
        TourStep::ReleaseClock => {
            cockpit.gid_mode = rebellion_render::GidMode::PopularSupport;
            cockpit.gid_highlight = rebellion_render::GidHighlight::default();
        }
        TourStep::LockInput => {}
        // With Agent Advice on (DAT_006b28b0 bit 0x8000 clear),
        // FUN_0041d770(1, 0x82) opens the Message Index on Advice, as the
        // rail's command 0x13d does.
        TourStep::Finish => {
            if advice {
                *pending_index_command = Some(0x13d);
            }
        }
    }
}

fn character_audience(world: &GameWorld, character: CharacterKey) -> Option<RailAudience> {
    let character = world.characters.get(character)?;
    if character.is_alliance {
        Some(RailAudience::Alliance)
    } else if character.is_empire {
        Some(RailAudience::Empire)
    } else {
        None
    }
}

/// Whether an `OPEN_REBELLION_MUTE` or `OPEN_REBELLION_DEV` value turns its
/// switch on: 1, true, yes or on, in any case.
#[cfg(not(target_arch = "wasm32"))]
fn env_flag_on(value: &str) -> bool {
    ["1", "true", "yes", "on"]
        .iter()
        .any(|on| value.trim().eq_ignore_ascii_case(on))
}

/// Open movie id `movie` over the whole screen.
fn open_movie(
    movie: u8,
    audio_vol: &AudioVolumeState,
    #[cfg(not(target_arch = "wasm32"))] audio_engine: &mut audio::AudioEngine,
) -> Option<VideoPlayer> {
    movies::path(movie).and_then(|path| {
        open_cutscene(
            Path::new(&path),
            audio_vol.cutscene_volume(),
            movies::FULL_SCREEN,
            #[cfg(not(target_arch = "wasm32"))]
            audio_engine,
        )
    })
}

/// Open the movie at `path`; the browser shows it in `bounds`
/// (`movies::video_bounds`).
fn open_cutscene(
    path: &Path,
    volume: f32,
    bounds: [f32; 4],
    #[cfg(not(target_arch = "wasm32"))] audio_engine: &mut audio::AudioEngine,
) -> Option<VideoPlayer> {
    match VideoPlayer::open(path, volume, bounds) {
        Ok(player) => {
            // A skipped movie leaves the music playing; nothing would restart it.
            #[cfg(not(target_arch = "wasm32"))]
            audio_engine.stop_music();
            macroquad::logging::info!("[cutscene] opened path={}", path.display());
            Some(player)
        }
        Err(VideoError::NotDecoded { .. }) => {
            macroquad::logging::warn!(
                "[cutscene] skipped path={}: run scripts/decode-cutscenes.sh to enable",
                path.display()
            );
            None
        }
        Err(error) => {
            macroquad::logging::warn!("[cutscene] skipped path={}: {}", path.display(), error);
            None
        }
    }
}

#[cfg(test)]
mod tactical_ground_tests {
    use super::*;
    use rebellion_core::dat::{ExplorationStatus, SectorGroup};
    use rebellion_core::ids::{CapitalShipKey, DatId, FighterKey};
    use rebellion_core::world::{ControlKind, FighterEntry, Fleet, Sector, ShipInstance, System};

    #[test]
    fn trench_run_outcomes_route_to_their_source_movies() {
        // FUN_0059cc60 plays 0x71 on success, FUN_0059cc80 0x70 on failure;
        // FUN_00421ba0 names MDATA.201 and MDATA.202.
        let path = |outcome| movies::path(CutsceneKind::TrenchRun(outcome).movie());
        assert_eq!(
            path(TacticalTrenchRunOutcome::Success).as_deref(),
            Some("assets/references/ref-videos/201.webm")
        );
        assert_eq!(
            path(TacticalTrenchRunOutcome::Failure).as_deref(),
            Some("assets/references/ref-videos/202.webm")
        );
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "The tactical result regression keeps its opposing fleets and exact roster assertions together."
    )]
    fn tactical_space_results_preserve_capital_slots_and_redock_surviving_squadrons() {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(1),
            name: "Test Sector".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let system = world.systems.insert(System {
            dat_id: DatId::new(2),
            name: "Test System".into(),
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
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            // The defender holds the system and loses the battle; the
            // post-battle pass moves its fleet to another system it holds
            // (VM, `empire-day660-deathstar-pre-alert.sav`: a loser at its own
            // held Carida and Ando ended at Halowan).
            control: ControlKind::Controlled(rebellion_core::dat::Faction::Empire),
        });
        let refuge = world.systems.insert(System {
            dat_id: DatId::new(4),
            name: "Refuge".into(),
            sector,
            x: 30,
            y: 0,
            exploration_status: ExplorationStatus::Explored,
            is_populated: true,
            control: ControlKind::Controlled(rebellion_core::dat::Faction::Empire),
            ..System::default()
        });
        // The defender's ship can jump; a fleet that cannot travel is
        // destroyed (FUN_00515480, reason 5).
        let defender_class = world
            .capital_ship_classes
            .insert(rebellion_core::world::CapitalShipClass {
                family: 0x14,
                is_empire: true,
                hull: 100,
                hyperdrive: 1,
                ..Default::default()
            });
        let ship_class = CapitalShipKey::default();
        let fighter_class = FighterKey::default();
        // FUN_005003d0: the fleet's Death Star is its family-0x18 ship; the
        // fleet loop exports it as the station alone (FUN_0040b0e0), whose
        // hull 0 after trench-run state 6 kills it (FUN_0040d170).
        let death_star_class =
            world
                .capital_ship_classes
                .insert(rebellion_core::world::CapitalShipClass {
                    family: 0x18,
                    is_alliance: true,
                    hull: 100,
                    ..Default::default()
                });
        let mut preexisting_dead_ship = ShipInstance::new(ship_class, 100, true);
        preexisting_dead_ship.hull_current = 0;
        preexisting_dead_ship.alive = false;
        let attacker = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![
                preexisting_dead_ship,
                ShipInstance::new(ship_class, 100, true),
                ShipInstance::new(ship_class, 100, true),
                ShipInstance::new(death_star_class, 100, true),
            ],
            fighters: vec![
                FighterEntry {
                    class: fighter_class,
                    count: 12,
                    carrier: 0,
                },
                FighterEntry {
                    class: fighter_class,
                    count: 12,
                    carrier: 0,
                },
            ],
            characters: vec![],
            is_alliance: true,
            bombarding: false,
        });
        let defender = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ShipInstance::new(defender_class, 100, false)],
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            bombarding: false,
        });
        world.systems[system].fleets = vec![attacker, defender];
        let mut session = rebellion_render::BattleSession {
            system,
            system_name: "Test System".into(),
            system_picture_id: 1,
            attacker_fleet: Some(attacker),
            defender_fleet: Some(defender),
            attacker_is_alliance: true,
            player_is_attacker: true,
            phase: rebellion_render::BattlePhase::Results,
            ships: vec![
                test_tactical_ship(ship_class, attacker, true, 1, 43, true),
                test_tactical_ship(ship_class, attacker, true, 2, 0, false),
                test_tactical_ship(defender_class, defender, false, 0, 91, true),
            ],
            fighters: vec![
                rebellion_render::tactical_view::TacticalFighter {
                    held_by_system: false,
                    off_field: false,
                    carrier: None,
                    docked: None,
                    class_key: fighter_class,
                    identity: rebellion_render::tactical_view::TacticalObjectIdentity {
                        class_dat_id: DatId::new(3),
                        fleet_roster_index: 0,
                        is_alliance: true,
                    },
                    tactical_resource: rebellion_render::fighter_tactical_resource(DatId::new(3)),
                    source_position: rebellion_render::tactical_view::TacticalWorldPosition {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    sublight_engine_power: 0.0,
                    fleet: Some(attacker),
                    fleet_fighter_index: 0,
                    fleet_squadron_index: 0,
                    name: "Fighter".into(),
                    x: 0.0,
                    y: 0.0,
                    squad_count: 12,
                    hull_current: 96.0,
                    hull_max: 96.0,
                    shield: 0.0,
                    shield_max: 0.0,
                    weapon_arc: rebellion_render::tactical_view::TacticalWeaponArc::default(),
                    weapon_ranges:
                        rebellion_render::tactical_view::TacticalWeaponRanges::default(),
                    torpedo_strength: 0,
                    torpedo_range: 0.0,
                    maneuverability: 0,
                    is_attacker: true,
                    alive: true,
                    fighter_group: 0,
                    selected: false,
                    order: rebellion_render::tactical_view::TacticalOrder::None,
                    tactic: rebellion_render::tactical_view::TacticalTactic::StandOff,
                    attack_target: None,
                    escort_target: None,
                    recovery_state: rebellion_render::tactical_view::TacticalFighterRecoveryState::AwaitingCarrier,
                    recovery_target: None,
                },
                rebellion_render::tactical_view::TacticalFighter {
                    held_by_system: false,
                    off_field: false,
                    carrier: None,
                    docked: None,
                    class_key: fighter_class,
                    identity: rebellion_render::tactical_view::TacticalObjectIdentity {
                        class_dat_id: DatId::new(3),
                        fleet_roster_index: 1,
                        is_alliance: true,
                    },
                    tactical_resource: rebellion_render::fighter_tactical_resource(DatId::new(3)),
                    source_position: rebellion_render::tactical_view::TacticalWorldPosition {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    sublight_engine_power: 0.0,
                    fleet: Some(attacker),
                    fleet_fighter_index: 1,
                    fleet_squadron_index: 0,
                    name: "Fighter".into(),
                    x: 0.0,
                    y: 0.0,
                    squad_count: 0,
                    hull_current: 0.0,
                    hull_max: 96.0,
                    shield: 0.0,
                    shield_max: 0.0,
                    weapon_arc: rebellion_render::tactical_view::TacticalWeaponArc::default(),
                    weapon_ranges:
                        rebellion_render::tactical_view::TacticalWeaponRanges::default(),
                    torpedo_strength: 0,
                    torpedo_range: 0.0,
                    maneuverability: 0,
                    is_attacker: true,
                    alive: true,
                    fighter_group: 1,
                    selected: false,
                    order: rebellion_render::tactical_view::TacticalOrder::None,
                    tactic: rebellion_render::tactical_view::TacticalTactic::StandOff,
                    attack_target: None,
                    escort_target: None,
                    recovery_state: rebellion_render::tactical_view::TacticalFighterRecoveryState::AwaitingCarrier,
                    recovery_target: None,
                },
            ],
            death_star: Some(rebellion_render::tactical_view::TacticalDeathStar {
                resource: rebellion_render::DEATH_STAR_TACTICAL_RESOURCE,
                is_attacker: true,
                is_alliance: true,
                source_position: rebellion_render::tactical_view::TacticalWorldPosition {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                hull_max: 100.0,
                hull: 0.0,
                laser_charge: 0.0,
                shielded: false,
                not_completed: false,
                action_committed: false,
                hangar_capacity: 0,
                ship: Some((attacker, 3)),
            }),
            death_star_beam: None,
            trench_run_outcome: None,
            trench_run_remaining_ms: None,
            trench_run_group: None,
            trench_run_commander: None,
            trench_run_pending_outcome: None,
            trench_run_participants: vec![],
            trench_run_snapshot: vec![],
            trench_run_chatter: [0x13d; 9],
            trench_run_chatter_cursor: 0,
            trench_run_cinematic_pending: false,
            navigation_sets: std::array::from_fn(|_| Vec::new()),
            source_layout: rebellion_render::tactical_view::OriginalTacticalLayout::default(),
            selected_ship: None,
            selected_fighter_group: None,
            selected_task_force: None,
            player_command: true,
            placement_confirmed: true,
            start_tick: 1,
            combat_tick: 1,
            tactical_rng: rebellion_render::OriginalTacticalRng::new(0x5eed_ba77),
            battle_rng: rebellion_core::tactical::OriginalBattleRng::default(),
            settled_crew: None,
            weapon_effects: vec![],
            impact_effects: vec![],
            pending_audio_cues: vec![],
            pending_voice_cues: vec![],
            pending_log_lines: vec![],
            field_effects: vec![],
            subsystem_repairs: vec![],
            paused: false,
            combat_speed: 1,
            step_accumulator: 0.0,
            winner: Some(rebellion_render::CombatWinner::Attacker),
            stalemate: rebellion_render::tactical_view::TacticalStalemateWatch::default(),
            death_star_ai: rebellion_render::tactical_view::TacticalDeathStarAi::default(),
            hangars: rebellion_render::tactical_view::TacticalHangars::default(),
            morale: rebellion_render::tactical_view::TacticalMorale::default(),
            engagement: rebellion_render::tactical_view::TacticalEngagementState::default(),
            holder_is_alliance: None,
            arrival_holder_is_alliance: None,
            blockaded_at_start: false,
            withdrawal_destination: None,
        };

        // FUN_005bef90.c:72-96: the surviving squadron docks in the first
        // ship of its side with room; FUN_0040d320 makes it that ship's child.
        session.ships[0].fighter_capacity = 1;

        let mut movement = MovementState::new();
        let battle_return = tactical_flow::apply_results(
            &session,
            &mut world,
            &mut TroopTransportState::default(),
            &mut movement,
            &ProductRoom::default(),
        );

        assert_eq!(world.fleets[attacker].capital_ships.len(), 1);
        assert_eq!(world.fleets[attacker].capital_ships[0].hull_current, 43);
        let carrier = world.fleets[attacker].capital_ships[0].tag;
        assert_ne!(carrier, 0);
        assert_eq!(
            world.fleets[attacker]
                .fighters
                .iter()
                .map(|entry| (entry.count, entry.carrier))
                .collect::<Vec<_>>(),
            vec![(1, carrier)]
        );
        assert!(!world.fleet_has_death_star(attacker));
        assert_eq!(world.fleets[defender].capital_ships[0].hull_current, 91);
        assert_eq!(
            movement.get(defender).map(|order| order.destination),
            Some(refuge)
        );
        assert_eq!(
            battle_return.winner,
            Some(rebellion_render::CombatWinner::Attacker)
        );
        assert_eq!(battle_return.winner_fleet, Some(attacker));
        assert!(battle_return.player_won);
    }

    fn test_tactical_ship(
        class_key: CapitalShipKey,
        fleet: FleetKey,
        is_attacker: bool,
        fleet_ship_index: usize,
        hull_current: i32,
        alive: bool,
    ) -> rebellion_render::tactical_view::TacticalShip {
        rebellion_render::tactical_view::TacticalShip {
            class_key,
            identity: rebellion_render::tactical_view::TacticalObjectIdentity {
                class_dat_id: DatId::new(1),
                fleet_roster_index: fleet_ship_index,
                is_alliance: is_attacker,
            },
            tactical_resource: rebellion_render::capital_ship_tactical_resource(DatId::new(1)),
            source_position: rebellion_render::tactical_view::TacticalWorldPosition {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            source_forward: rebellion_render::tactical_view::TacticalWorldVector {
                x: 0.0,
                y: 0.0,
                z: if is_attacker { 1.0 } else { -1.0 },
            },
            source_desired_forward: rebellion_render::tactical_view::TacticalWorldVector {
                x: 0.0,
                y: 0.0,
                z: if is_attacker { 1.0 } else { -1.0 },
            },
            source_velocity: rebellion_render::tactical_view::TacticalWorldVector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            source_waypoint: None,
            navigation_route: Vec::new(),
            source_collision_envelope: None,
            name: "Ship".into(),
            x: 0.0,
            y: 0.0,
            hull_current,
            hull_max: 100,
            shield: 0,
            shield_max: 0,
            subsystem_condition: rebellion_render::tactical_view::TacticalSubsystemCondition {
                shields: 0,
                weapons: 0,
                tractor: 0,
                engines: 0,
                hyperdrive: 0,
            },
            subsystem_capacity: rebellion_render::tactical_view::TacticalSubsystemCapacity {
                shields: false,
                weapons: false,
                tractor: false,
                engines: false,
                hyperdrive: 0,
            },
            subsystem_damage: rebellion_render::tactical_view::TacticalSubsystemDamage::default(),
            sublight_engine_power: 0.0,
            maneuverability: 0,
            tractor_beam_power: 0.0,
            fighter_capacity: 0,
            maneuver_state_value: None,
            damage_control: 0,
            is_attacker,
            alive,
            selected: false,
            order: rebellion_render::tactical_view::TacticalOrder::None,
            tactic: rebellion_render::tactical_view::TacticalTactic::StandOff,
            task_force: u8::try_from(fleet_ship_index.min(7)).unwrap_or(7),
            task_force_joined: 0,
            fleet: Some(fleet),
            fleet_ship_index,
            sprite_id: None,
            weapon_arcs: [rebellion_render::tactical_view::TacticalWeaponArc::default(); 4],
            weapon_ranges: rebellion_render::tactical_view::TacticalWeaponRanges::default(),
            weapon_recharge_rate: 0.0,
            shield_recharge_rate: 0.0,
            shield_recharge_carry: 0.0,
            weapon_recharge_queue: Vec::new(),
            attack_target: None,
            manual_targets: Vec::new(),
            escort_target: None,
            contents: Vec::new(),
            retreating: false,
            retreat_progress: 0.0,
            retreated: false,
            outstanding_hits: 0,
            repair_timer_steps: 0,
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod options_save_tests {
    #[test]
    fn unreadable_native_saves_remain_occupied_and_unchanged() {
        let root =
            std::env::temp_dir().join(format!("options-corrupt-save-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let path = rebellion_data::save::slot_path(&root, 5);
        std::fs::write(&path, b"corrupt save fixture").unwrap();
        let slots = super::read_save_slots(&root);
        assert_eq!(slots.len(), 1);
        assert_eq!(slots[0].slot, 5);
        assert_eq!(slots[0].name, "Unreadable save");
        assert_eq!(std::fs::read(&path).unwrap(), b"corrupt save fixture");
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn the_options_window_reads_only_its_six_rows() {
        // FUN_00407180 builds rows 1 to 6, row n reading save file n; a file
        // past them is no row's.
        let root = std::env::temp_dir().join(format!("options-six-rows-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        for slot in [6, 9] {
            std::fs::write(
                rebellion_data::save::slot_path(&root, slot),
                b"past the rows",
            )
            .unwrap();
        }
        assert!(super::read_save_slots(&root).is_empty());
        std::fs::remove_dir_all(root).unwrap();
    }
}

#[cfg(test)]
mod fleet_move_tests {
    use super::*;
    use rebellion_core::dat::{ExplorationStatus, SectorGroup};
    use rebellion_core::ids::DatId;
    use rebellion_core::manufacturing::QueueItem;
    use rebellion_core::world::{
        CapitalShipClass, ControlKind, Fleet, Sector, ShipInstance, System,
    };

    fn system(sector: rebellion_core::ids::SectorKey, x: u16) -> System {
        System {
            dat_id: DatId::new(0x9000_0000),
            name: format!("Sys{x}"),
            sector,
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
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Uncontrolled,
        }
    }

    /// Two Alliance fleets at one system, and a second system 40 away.
    fn world_with_two_fleets() -> (GameWorld, Vec<FleetKey>, SystemKey) {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(0x9200_0000),
            name: "Test".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let origin = world.systems.insert(system(sector, 0));
        let destination = world.systems.insert(system(sector, 40));
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            name: "TestShip".into(),
            is_alliance: true,
            hull: 100,
            hyperdrive: 80,
            ..CapitalShipClass::default()
        });
        let fleets = (0..2)
            .map(|_| {
                let fleet = world.fleets.insert(Fleet {
                    location: origin,
                    capital_ships: vec![ShipInstance::new(class, 100, true)],
                    fighters: vec![],
                    characters: vec![],
                    is_alliance: true,
                    bombarding: false,
                });
                world.systems[origin].fleets.push(fleet);
                fleet
            })
            .collect();
        (world, fleets, destination)
    }

    struct Issued {
        actions: Vec<PanelAction>,
        confirmation: MoveConfirmationState,
    }

    fn issue(world: &GameWorld, fleets: &[FleetKey], confirmed: bool, to: SystemKey) -> Issued {
        issue_onto(world, fleets, confirmed, ReleaseTarget::System(to))
    }

    fn issue_onto(
        world: &GameWorld,
        fleets: &[FleetKey],
        confirmed: bool,
        target: ReleaseTarget,
    ) -> Issued {
        issue_as(world, fleets, confirmed, target, MissionFaction::Alliance)
    }

    fn issue_as(
        world: &GameWorld,
        fleets: &[FleetKey],
        confirmed: bool,
        target: ReleaseTarget,
        faction: MissionFaction,
    ) -> Issued {
        let movement = MovementState::new();
        let blockaded = HashSet::new();
        let mut issued = Issued {
            actions: Vec::new(),
            confirmation: MoveConfirmationState::default(),
        };
        issue_fleet_move(
            &FleetMoveContext {
                world,
                movement: &movement,
                blockaded: &blockaded,
                faction,
            },
            fleets,
            confirmed,
            target,
            &mut issued.confirmation,
            &mut issued.actions,
        );
        issued
    }

    fn dispatched(actions: &[PanelAction]) -> Vec<FleetKey> {
        actions
            .iter()
            .filter_map(|action| match action {
                PanelAction::DispatchFleet { fleet, .. } => Some(*fleet),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_fleet_icons_move_departs_every_fleet_of_the_team() {
        // FUN_00486fb0 -> FUN_00487740: a valid Move that need not ask
        // submits the order, and every sub-order departs.
        let (world, fleets, destination) = world_with_two_fleets();
        let issued = issue(&world, &fleets, false, destination);
        assert_eq!(dispatched(&issued.actions), fleets);
        assert!(!issued.confirmation.is_open());
    }

    #[test]
    fn a_fleet_icons_confirmed_move_asks_once_for_the_whole_team() {
        // FUN_00487cc0 asks for 0x202; the window lists a line per member
        // (FUN_0053c2e0) and departs nothing until the checkmark.
        let (world, fleets, destination) = world_with_two_fleets();
        let issued = issue(&world, &fleets, true, destination);
        assert!(issued.actions.is_empty());
        let window = issued
            .confirmation
            .confirmation()
            .expect("the window opens");
        assert_eq!(window.fleets, fleets);
        assert_eq!(window.lines.len(), 2);
        // FUN_0049a350: each line counts the member's days to go.
        assert!(window.lines.iter().all(|(_, days)| *days > 0));
    }

    #[test]
    fn a_fleet_icons_move_onto_a_fleet_in_its_own_system_joins_each_member() {
        // FUN_00486fb0: a target fleet makes each sub-order a join
        // (FUN_004ffc90), which needs no departure where the target lies.
        let (mut world, fleets, _) = world_with_two_fleets();
        let origin = world.fleets[fleets[0]].location;
        let target = world.fleets.insert(world.fleets[fleets[0]].clone());
        world.systems[origin].fleets.push(target);
        let issued = issue_onto(
            &world,
            &fleets,
            false,
            ReleaseTarget::Fleet {
                fleet: target,
                system: origin,
            },
        );
        let joined: Vec<FleetKey> = issued
            .actions
            .iter()
            .filter_map(|action| match action {
                PanelAction::JoinFleet {
                    mover: FleetMover::Fleet(fleet),
                    target: onto,
                } if *onto == target => Some(*fleet),
                _ => None,
            })
            .collect();
        assert_eq!(joined, fleets);
    }

    /// The fleets of `world_with_two_fleets` given to `faction`.
    fn sided(world: &mut GameWorld, fleets: &[FleetKey], faction: MissionFaction) {
        let alliance = faction == MissionFaction::Alliance;
        for fleet in fleets {
            world.fleets[*fleet].is_alliance = alliance;
        }
    }

    /// `ships` of `fleet`, Moved or Confirmed-Moved onto `target` by
    /// `faction`.
    fn issue_ships(
        world: &GameWorld,
        fleet: FleetKey,
        ships: Vec<usize>,
        confirmed: bool,
        target: ReleaseTarget,
        faction: MissionFaction,
    ) -> Issued {
        let movement = MovementState::new();
        let blockaded = HashSet::new();
        let mut issued = Issued {
            actions: Vec::new(),
            confirmation: MoveConfirmationState::default(),
        };
        issue_ship_move(
            &FleetMoveContext {
                world,
                movement: &movement,
                blockaded: &blockaded,
                faction,
            },
            fleet,
            ships,
            rebellion_core::fleet_join::roster(world, fleet).unwrap(),
            confirmed,
            target,
            &mut issued.confirmation,
            &mut issued.actions,
        );
        issued
    }

    // FUN_00487cc0 asks for 0x202 always, and for 0x201 only when the first
    // member's container is a blockaded system: a ship's is its fleet. The
    // window lists each ship's days (FUN_0053d200): none for an unfinished
    // ship, which is not mobile (FUN_00555d30 -> FUN_004f6410). The route-1
    // Empire save, its day-123 Death Star played on: the unfinished Death
    // Star's menu and a completed ship's both offered Confirmed Move.
    #[test]
    fn a_ships_confirmed_move_asks_with_each_ships_days_and_its_move_departs_for_either_side() {
        for faction in [MissionFaction::Alliance, MissionFaction::Empire] {
            let (mut world, fleets, destination) = world_with_two_fleets();
            sided(&mut world, &fleets, faction);
            let fleet = fleets[0];
            let mut unfinished = world.fleets[fleet].capital_ships[0].clone();
            unfinished.completed = false;
            world.fleets[fleet].capital_ships.push(unfinished);
            let to = ReleaseTarget::System(destination);

            let moved = issue_ships(&world, fleet, vec![0], false, to, faction);
            assert!(!moved.confirmation.is_open(), "{faction:?}");
            assert!(
                matches!(
                    moved.actions.as_slice(),
                    [PanelAction::MoveShips { fleet: from, ships, system, .. }]
                        if *from == fleet && *ships == [0] && *system == destination
                ),
                "{faction:?}: {:?}",
                moved.actions
            );

            let asked = issue_ships(&world, fleet, vec![0, 1], true, to, faction);
            assert!(asked.actions.is_empty(), "{faction:?}");
            let window = asked.confirmation.confirmation().expect("the window opens");
            assert_eq!(
                window.ships,
                Some(FleetMover::Ships {
                    fleet,
                    ships: vec![0, 1],
                    roster: rebellion_core::fleet_join::roster(&world, fleet).unwrap(),
                }),
                "{faction:?}"
            );
            assert!(window.fleets.is_empty() && !window.blockaded, "{faction:?}");
            assert_eq!(window.lines.len(), 2, "{faction:?}");
            assert!(window.lines[0].1 > 0, "{faction:?}: the completed ship's leg");
            assert_eq!(window.lines[1].1, 0, "{faction:?}: the unfinished ship's");
        }
    }

    // FUN_00487740 runs the validator before FUN_00487cc0 asks: a ship on
    // its own leg is en route (FUN_004f9860, 1/1), so nothing asks.
    #[test]
    fn a_ships_confirmed_move_the_order_refuses_never_asks_for_either_side() {
        for faction in [MissionFaction::Alliance, MissionFaction::Empire] {
            let (mut world, fleets, destination) = world_with_two_fleets();
            sided(&mut world, &fleets, faction);
            let origin = world.fleets[fleets[0]].location;
            world.fleets[fleets[0]].capital_ships[0].transit =
                Some(rebellion_core::world::ShipTransit {
                    from: origin,
                    to: destination,
                    arrival_tick: 9,
                });

            let to = ReleaseTarget::System(destination);
            let issued = issue_ships(&world, fleets[0], vec![0], true, to, faction);
            assert!(!issued.confirmation.is_open(), "{faction:?}");
            assert!(
                matches!(
                    issued.actions.as_slice(),
                    [PanelAction::AdvisorRefusal((1, 1))]
                ),
                "{faction:?}: {:?}",
                issued.actions
            );
        }
    }

    /// `world_with_two_fleets` for `faction`: both systems its, a regiment
    /// of a named class on the origin's surface and a squadron aboard the
    /// first fleet.
    fn unit_world(faction: MissionFaction) -> (GameWorld, Vec<FleetKey>, SystemKey, TroopKey) {
        let (mut world, fleets, destination) = world_with_two_fleets();
        sided(&mut world, &fleets, faction);
        let is_alliance = faction == MissionFaction::Alliance;
        let side = if is_alliance {
            Faction::Alliance
        } else {
            Faction::Empire
        };
        let origin = world.fleets[fleets[0]].location;
        for system in [origin, destination] {
            world.systems[system].control = ControlKind::Controlled(side);
        }
        let class_dat_id = DatId::new(0x1000_0002);
        world.buildable_classes.insert(
            class_dat_id,
            rebellion_core::world::BuildableClass {
                name: "Army Regiment".into(),
                ..Default::default()
            },
        );
        let troop = world.troops.insert(rebellion_core::world::TroopUnit {
            class_dat_id,
            is_alliance,
        });
        world.systems[origin].ground_units.push(troop);
        let fighters = world.fighter_classes.insert(rebellion_core::world::FighterClass {
            name: "Fighter".into(),
            ..Default::default()
        });
        world.fleets[fleets[0]]
            .fighters
            .push(rebellion_core::world::FighterEntry {
                class: fighters,
                count: 1,
                carrier: 0,
            });
        (world, fleets, destination, troop)
    }

    /// `unit` Moved or Confirmed-Moved onto `target` by `faction`, with
    /// `blockaded` systems under blockade.
    fn issue_unit(
        world: &GameWorld,
        unit: UnitMover,
        confirmed: bool,
        target: ReleaseTarget,
        (faction, blockaded): (MissionFaction, &HashSet<SystemKey>),
    ) -> Issued {
        let movement = MovementState::new();
        let mut issued = Issued {
            actions: Vec::new(),
            confirmation: MoveConfirmationState::default(),
        };
        issue_unit_move(
            &FleetMoveContext {
                world,
                movement: &movement,
                blockaded,
                faction,
            },
            &TroopTransportState::default(),
            &ProductRoom::default(),
            unit,
            confirmed,
            target,
            &mut issued.confirmation,
            &mut issued.actions,
        );
        issued
    }

    // FUN_00487cc0 asks for a unit's 0x202 always; FUN_0049a350 lists its
    // name and days (FUN_0053d200: a regiment's trip at the default speed,
    // a squadron's at its class's). Its 0x201 departs at once.
    #[test]
    fn a_units_confirmed_move_asks_with_its_days_and_its_move_departs_for_either_side() {
        for faction in [MissionFaction::Alliance, MissionFaction::Empire] {
            let (world, fleets, destination, troop) = unit_world(faction);
            let to = ReleaseTarget::System(destination);
            let open = (faction, &HashSet::new());
            let squadron = UnitMover::Squadron(SquadronSource::Fleet {
                fleet: fleets[0],
                index: 0,
            });
            for (unit, name) in [
                (UnitMover::Regiment(troop), "Army Regiment"),
                (squadron, "Fighter"),
            ] {
                let moved = issue_unit(&world, unit, false, to, open);
                assert!(!moved.confirmation.is_open(), "{faction:?} {unit:?}");
                assert_eq!(moved.actions.len(), 1, "{faction:?} {unit:?}");

                let asked = issue_unit(&world, unit, true, to, open);
                assert!(asked.actions.is_empty(), "{faction:?} {unit:?}");
                let window = asked.confirmation.confirmation().expect("the window opens");
                assert_eq!(window.units, [unit], "{faction:?}");
                assert!(!window.blockaded, "{faction:?} {unit:?}");
                assert_eq!(window.lines.len(), 1, "{faction:?} {unit:?}");
                assert_eq!(window.lines[0].0, name, "{faction:?}");
                assert!(window.lines[0].1 > 0, "{faction:?} {unit:?}: its trip");
            }
            let origin = world.fleets[fleets[0]].location;
            let asked = issue_unit(&world, UnitMover::Regiment(troop), true, ReleaseTarget::System(origin), open);
            assert_eq!(
                asked.confirmation.confirmation().map(|window| window.lines[0].1),
                Some(0),
                "{faction:?}: within its system"
            );
        }
    }

    // FUN_00487cc0 asks for a unit's 0x201 when its container is a
    // blockaded system of its side, under the evacuation warning (0x7056,
    // FUN_0049a350); a squadron aboard a fleet has the ship for its
    // container, so it departs at once.
    #[test]
    fn a_units_move_off_its_sides_blockaded_planet_asks_under_the_evacuation_warning_for_either_side() {
        for faction in [MissionFaction::Alliance, MissionFaction::Empire] {
            let (world, fleets, destination, troop) = unit_world(faction);
            let origin = world.fleets[fleets[0]].location;
            let blockaded = HashSet::from([origin]);
            let to = ReleaseTarget::System(destination);

            let asked = issue_unit(&world, UnitMover::Regiment(troop), false, to, (faction, &blockaded));
            assert!(asked.actions.is_empty(), "{faction:?}");
            assert!(
                asked.confirmation.confirmation().is_some_and(|window| window.blockaded),
                "{faction:?}"
            );

            let aboard = UnitMover::Squadron(SquadronSource::Fleet {
                fleet: fleets[0],
                index: 0,
            });
            let moved = issue_unit(&world, aboard, false, to, (faction, &blockaded));
            assert!(!moved.confirmation.is_open(), "{faction:?}");
            assert_eq!(moved.actions.len(), 1, "{faction:?}");
        }
    }

    // FUN_00487740 runs the validator before FUN_00487cc0 asks: another
    // side's populated system refuses a regiment (FUN_0053d430, 1/0x28),
    // and so does the other side's unit (FUN_004f9860).
    #[test]
    fn a_units_confirmed_move_the_order_refuses_never_asks_for_either_side() {
        for faction in [MissionFaction::Alliance, MissionFaction::Empire] {
            let (mut world, _, destination, troop) = unit_world(faction);
            let other = if faction == MissionFaction::Alliance {
                MissionFaction::Empire
            } else {
                MissionFaction::Alliance
            };
            let unit = UnitMover::Regiment(troop);
            let to = ReleaseTarget::System(destination);
            let refused = issue_unit(&world, unit, true, to, (other, &HashSet::new()));
            assert!(!refused.confirmation.is_open(), "{faction:?}");
            assert!(
                matches!(refused.actions.as_slice(), [PanelAction::AdvisorRefusal((1, 0x28))]),
                "{faction:?}: {:?}",
                refused.actions
            );
            world.systems[destination].control = ControlKind::Controlled(if faction == MissionFaction::Alliance {
                Faction::Empire
            } else {
                Faction::Alliance
            });
            let refused = issue_unit(&world, unit, true, to, (faction, &HashSet::new()));
            assert!(!refused.confirmation.is_open(), "{faction:?}");
            assert!(
                matches!(refused.actions.as_slice(), [PanelAction::AdvisorRefusal((1, 0x28))]),
                "{faction:?}: {:?}",
                refused.actions
            );
        }
    }

    // FUN_0044f5e0 resubmits the confirmed order; each unit's 0x204 moves it
    // onto the fleet or the system (FUN_00578f30 -> FUN_00556390).
    #[test]
    fn a_confirmed_units_move_carries_out_each_units_move() {
        let (world, fleets, destination, troop) = unit_world(MissionFaction::Alliance);
        let source = SquadronSource::Fleet {
            fleet: fleets[0],
            index: 0,
        };
        assert!(matches!(
            unit_move_action(UnitMover::Regiment(troop), RegimentTarget::System(destination)),
            PanelAction::MoveRegiment { troop: moved, target: RegimentTarget::System(to) }
                if moved == troop && to == destination
        ));
        assert!(matches!(
            unit_move_action(UnitMover::Squadron(source), RegimentTarget::Fleet(fleets[1])),
            PanelAction::MoveSquadron { source: moved, target: RegimentTarget::Fleet(onto) }
                if moved == source && onto == fleets[1]
        ));
        assert!(world.troops.contains_key(troop));
    }

    // FUN_0044f5e0 resubmits the confirmed order: a team released on a Fleet
    // window joins it (FUN_004ffc90), else its ships go to the system and
    // its fleets depart.
    #[test]
    fn a_confirmed_move_carries_out_each_movers_move() {
        let (world, fleets, destination) = world_with_two_fleets();
        let ships = FleetMover::Ships {
            fleet: fleets[0],
            ships: vec![0],
            roster: 3,
        };
        assert!(matches!(
            mover_move_action(ships.clone(), destination, Some(fleets[1])),
            PanelAction::JoinFleet { mover, target } if mover == ships && target == fleets[1]
        ));
        assert!(matches!(
            mover_move_action(ships, destination, None),
            PanelAction::MoveShips { fleet, ships, roster: 3, system }
                if fleet == fleets[0] && ships == [0] && system == destination
        ));
        assert!(matches!(
            mover_move_action(FleetMover::Fleet(fleets[1]), destination, None),
            PanelAction::DispatchFleet { fleet, destination: to, troops }
                if fleet == fleets[1] && to == destination && troops.is_empty()
        ));
        assert!(world.fleets.contains_key(fleets[1]));
    }

    #[test]
    fn an_empty_team_moving_onto_a_fleet_is_refused() {
        // FUN_0053c4b0 refuses a move with no members (1/0x16) before any
        // sub-order runs; FUN_00487c90 shows no text, the advisor reacts.
        for faction in [MissionFaction::Alliance, MissionFaction::Empire] {
            let (mut world, fleets, _) = world_with_two_fleets();
            sided(&mut world, &fleets, faction);
            let origin = world.fleets[fleets[0]].location;
            let issued = issue_as(
                &world,
                &[],
                false,
                ReleaseTarget::Fleet {
                    fleet: fleets[0],
                    system: origin,
                },
                faction,
            );
            assert!(
                matches!(issued.actions[..], [PanelAction::AdvisorRefusal((1, 0x16))]),
                "{faction:?}"
            );
        }
    }

    #[test]
    fn a_bands_stop_and_destination_belong_to_its_systems_holder() {
        // Manual p. 83-84: a construction yard's band takes Destination and
        // Stop; Stop clears what the band is building. Either side holds
        // its own bands. An idle band greys Stop (FUN_00528270.c:22-31) and
        // Rename (FUN_00527f40.c:23-33): no product resolves from +0x74.
        for (holder, other) in [
            (MissionFaction::Alliance, MissionFaction::Empire),
            (MissionFaction::Empire, MissionFaction::Alliance),
        ] {
            let mut world = GameWorld::default();
            let control = rebellion_core::world::ControlKind::Controlled(match holder {
                MissionFaction::Alliance => Faction::Alliance,
                MissionFaction::Empire => Faction::Empire,
            });
            let system = world.systems.insert(rebellion_core::world::System {
                dat_id: rebellion_core::ids::DatId::new(0x9000_0001),
                name: "Bortras".into(),
                sector: rebellion_core::ids::SectorKey::default(),
                x: 0,
                y: 0,
                exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
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
                control,
            });
            let class =
                world
                    .capital_ship_classes
                    .insert(rebellion_core::world::CapitalShipClass {
                        is_alliance: true,
                        is_empire: true,
                        ..rebellion_core::world::CapitalShipClass::default()
                    });
            let mut manufacturing = ManufacturingState::new();
            let band = Some(MenuObject::Producer {
                system,
                area: rebellion_core::manufacturing::ProductionArea::Shipyard,
            });
            let gates = |world: &GameWorld, manufacturing: &ManufacturingState, player| {
                order_gates(
                    band,
                    world,
                    &MissionState::new(),
                    &MovementState::new(),
                    &TroopTransportState::new(),
                    &BlockadeState::new(),
                    manufacturing,
                    &rebellion_core::research::ResearchState::new(),
                    player,
                )
            };
            let idle = gates(&world, &manufacturing, holder);
            assert!(idle.destination && idle.build && !idle.stop && !idle.rename);
            manufacturing.build(
                system,
                &QueueItem::new(
                    rebellion_core::manufacturing::BuildableKind::CapitalShip(class),
                    10,
                ),
                1,
            );
            let building = gates(&world, &manufacturing, holder);
            assert!(building.destination && building.stop);
            let foreign = gates(&world, &manufacturing, other);
            assert!(!foreign.destination && !foreign.build && !foreign.stop);
            // With nothing its side may build, Build stays off.
            world.capital_ship_classes[class].research_order = 1;
            assert!(!gates(&world, &manufacturing, holder).build);
        }
    }

    #[test]
    fn a_ships_band_renames_its_ordered_ship_for_its_holder_only() {
        // FUN_00527f40: a band's Rename (0x215) asks its current product's
        // +0x64 for 0x203. An unfinished capital ship of the side passes
        // FUN_004f9860; a squadron refuses (FUN_004f66a0, 1/0x17); an idle
        // band has no product (0xa0/1). Under Wine Rename was enabled on the
        // route-1 Empire save's ships band building Death Star 1 (day 123
        // played on); no fighter band was at hand there.
        for (holder, other) in [
            (MissionFaction::Alliance, MissionFaction::Empire),
            (MissionFaction::Empire, MissionFaction::Alliance),
        ] {
            let alliance = holder == MissionFaction::Alliance;
            let mut world = GameWorld::default();
            let control =
                rebellion_core::world::ControlKind::Controlled(Faction::of_alliance(alliance));
            let system = world.systems.insert(rebellion_core::world::System {
                dat_id: rebellion_core::ids::DatId::new(0x9000_0001),
                name: "Coruscant".into(),
                sector: rebellion_core::ids::SectorKey::default(),
                x: 0,
                y: 0,
                exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
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
                control,
            });
            let fighters = world.systems.insert(rebellion_core::world::System {
                dat_id: rebellion_core::ids::DatId::new(0x9000_0002),
                ..world.systems[system].clone()
            });
            let class =
                world
                    .capital_ship_classes
                    .insert(rebellion_core::world::CapitalShipClass {
                        name: "Death Star".into(),
                        hull: 10,
                        is_alliance: alliance,
                        is_empire: !alliance,
                        ..rebellion_core::world::CapitalShipClass::default()
                    });
            let squadron = world
                .fighter_classes
                .insert(rebellion_core::world::FighterClass::default());
            let mut manufacturing = ManufacturingState::new();
            let gate = |world: &GameWorld, manufacturing: &ManufacturingState, at, player| {
                order_gates(
                    Some(MenuObject::Producer {
                        system: at,
                        area: rebellion_core::manufacturing::ProductionArea::Shipyard,
                    }),
                    world,
                    &MissionState::new(),
                    &MovementState::new(),
                    &TroopTransportState::new(),
                    &BlockadeState::new(),
                    manufacturing,
                    &rebellion_core::research::ResearchState::new(),
                    player,
                )
                .rename
            };
            assert!(!gate(&world, &manufacturing, system, holder));

            manufacturing.build(
                system,
                &QueueItem::new(
                    rebellion_core::manufacturing::BuildableKind::CapitalShip(class),
                    10,
                ),
                1,
            );
            manufacturing.build(
                fighters,
                &QueueItem::new(
                    rebellion_core::manufacturing::BuildableKind::Fighter(squadron),
                    4,
                ),
                1,
            );
            rebellion_core::manufacturing::seat_ordered_ships(&mut world, &mut manufacturing);
            assert!(gate(&world, &manufacturing, system, holder), "{holder:?}");
            assert!(!gate(&world, &manufacturing, system, other), "{holder:?}");
            assert!(
                !gate(&world, &manufacturing, fighters, holder),
                "{holder:?}"
            );

            let (fleet, index) = band_rename_target(
                &world,
                &manufacturing,
                system,
                rebellion_core::manufacturing::ProductionArea::Shipyard,
                alliance,
            )
            .unwrap();
            assert_eq!(world.ship_name(fleet, index), Some("Death Star 1"));
            assert!(!world.fleets[fleet].capital_ships[index].completed);
            // The other side's player names nothing there.
            assert_eq!(
                band_rename_target(
                    &world,
                    &manufacturing,
                    system,
                    rebellion_core::manufacturing::ProductionArea::Shipyard,
                    !alliance,
                ),
                None
            );
        }
    }

    #[test]
    fn a_fleet_icons_move_is_enabled_only_where_its_side_has_fleets() {
        // FUN_0051d990 asks each order's +0x18; Move on the fleet icon
        // (kind 0x10) needs a team (FUN_0053c4b0) that may all move.
        let (world, fleets, destination) = world_with_two_fleets();
        let origin = world.fleets[fleets[0]].location;
        let gate = |system| {
            order_gates(
                Some(MenuObject::SystemIcon {
                    system,
                    quadrant: Quadrant::Fleets,
                }),
                &world,
                &MissionState::new(),
                &MovementState::new(),
                &TroopTransportState::new(),
                &BlockadeState::new(),
                &ManufacturingState::new(),
                &rebellion_core::research::ResearchState::new(),
                MissionFaction::Alliance,
            )
            .fleet_move
        };
        assert!(gate(origin));
        assert!(!gate(destination));
    }

    #[test]
    fn one_refused_fleet_keeps_the_whole_team_home() {
        // FUN_0053c1a0 runs every sub-order's validator; FUN_00487c90 reports
        // the refusal and nothing departs.
        // An empty fleet has no speed (FUN_00555920 → FUN_004fd900: 1/0x18),
        // and the refusal shows no text.
        for faction in [MissionFaction::Alliance, MissionFaction::Empire] {
            let (mut world, fleets, destination) = world_with_two_fleets();
            sided(&mut world, &fleets, faction);
            world.fleets[fleets[1]].capital_ships.clear();
            let issued = issue_as(
                &world,
                &fleets,
                false,
                ReleaseTarget::System(destination),
                faction,
            );
            assert!(
                matches!(issued.actions[..], [PanelAction::AdvisorRefusal((1, 0x18))]),
                "{faction:?}"
            );
            assert!(!issued.confirmation.is_open());
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod launch_mute_tests {
    use super::env_flag_on;

    #[test]
    fn the_mute_variable_takes_common_true_words_in_any_case() {
        for on in ["1", "true", "TRUE", "Yes", "on", " on\n"] {
            assert!(env_flag_on(on), "{on:?}");
        }
        for off in ["", "0", "false", "no", "off", "muted"] {
            assert!(!env_flag_on(off), "{off:?}");
        }
    }
}

#[cfg(test)]
mod release_message_tests {
    use super::*;
    use rebellion_core::ids::DatId;
    use rebellion_core::world::Character;
    use std::collections::HashMap;

    /// The shipped TEXTSTRA entries `0x715c`-`0x715f`.
    fn templates() -> rebellion_data::text_templates::TextTemplates {
        let entries: HashMap<u16, &str> = [
            (0x715c, "|\u{1}\u{1}\0\0\0 Escaped\u{1}"),
            (
                0x715d,
                "|\u{1}\u{1}\0\0\0 escaped from the |\u{3}\u{1}\0\0\0 at |\u{2}\u{1}\0\0\0.\u{1}",
            ),
            (0x715e, "|\u{1}\u{1}\0\0\0 Escaped\u{1}"),
            (
                0x715f,
                "|\u{1}\u{1}\0\0\0 has escaped from imprisonment at |\u{2}\u{1}\0\0\0.\u{1}",
            ),
        ]
        .into_iter()
        .collect();
        rebellion_data::text_templates::TextTemplates::from_json(
            &serde_json::to_vec(&entries).unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn an_escape_without_its_textstra_entries_files_nothing() {
        // FUN_00490340's text is TEXTSTRA 0x715c-0x715f alone.
        let empty = rebellion_data::text_templates::TextTemplates::default();
        for is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let minor = world.characters.insert(Character {
                name: "Agent".into(),
                is_alliance,
                is_empire: !is_alliance,
                ..Character::default()
            });
            assert!(release_messages(&world, &empty, 9, minor, "Kessel", 0).is_empty());
        }
    }

    #[test]
    fn an_escapee_s_own_side_hears_its_voice_line_and_the_other_side_none() {
        // FUN_00490340.c:207-244: case 1 (own side) draws +0x32 for a
        // major from 0xf6, 0xe2, 0xce or 0xd8 (Luke, +0xb0 bit 3), 0xec,
        // 0x10a, 0x100; case 2 (the other side) sets none.
        for (record, untrained, trained) in [
            (0x240, 0xf6, 0xf6),
            (0x241, 0xe2, 0xe2),
            (0x242, 0xce, 0xd8),
            (0x243, 0xec, 0xec),
            (0x280, 0x10a, 0x10a),
            (0x281, 0x100, 0x100),
        ] {
            let is_alliance = record < 0x280;
            let mut world = GameWorld::default();
            let major = world.characters.insert(Character {
                name: "Major".into(),
                dat_id: DatId::new(0x3000_0000 | record),
                is_major: true,
                is_alliance,
                is_empire: !is_alliance,
                ..Character::default()
            });
            assert_eq!(
                [false, true].map(|luke| major_voice_list(&world, major, luke, RELEASE_VOICES)),
                [Some(untrained), Some(trained)],
                "{record:#x}"
            );
            let voices: Vec<(u16, RailAudience)> =
                release_messages(&world, &templates(), 9, major, "Kessel", 1358)
                    .iter()
                    .map(|message| (message.display.as_ref().unwrap().voice, message.audience))
                    .collect();
            assert_eq!(
                voices,
                [
                    (1358, RailAudience::side(is_alliance)),
                    (0, RailAudience::side(!is_alliance))
                ],
                "{record:#x}"
            );
        }
        let mut world = GameWorld::default();
        let minor = world.characters.insert(Character::default());
        assert_eq!(major_voice_list(&world, minor, false, RELEASE_VOICES), None);
    }

    #[test]
    fn a_release_files_its_own_and_the_other_sides_escape_message() {
        // FUN_00490340 on notification 0x1e0: the native side reads
        // 0x715c/0x715d with the captor, the other 0x715e/0x715f; picture
        // 0x405, sound 0x469; a minor's code 0x15 own, 0x24 other.
        for is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let minor = world.characters.insert(Character {
                name: "Agent".into(),
                is_alliance,
                is_empire: !is_alliance,
                ..Character::default()
            });

            let messages = release_messages(&world, &templates(), 9, minor, "Kessel", 0);

            let captor = if is_alliance { "Empire" } else { "Alliance" };
            let shown: Vec<(String, String, Option<u8>, RailAudience)> = messages
                .iter()
                .map(|message| {
                    let display = message.display.as_ref().unwrap();
                    assert_eq!((display.background, display.sound), (0x405, 0x469));
                    (
                        display.title.clone(),
                        display.body.clone(),
                        message.advice,
                        message.audience,
                    )
                })
                .collect();
            assert_eq!(
                shown,
                [
                    (
                        "Agent Escaped".to_owned(),
                        format!("Agent escaped from the {captor} at Kessel."),
                        Some(0x15),
                        RailAudience::side(is_alliance),
                    ),
                    (
                        "Agent Escaped".to_owned(),
                        "Agent has escaped from imprisonment at Kessel.".to_owned(),
                        Some(0x24),
                        RailAudience::side(!is_alliance),
                    ),
                ]
            );
        }
    }

    #[test]
    fn a_majors_release_carries_its_own_code_and_another_majors_none() {
        // FUN_00490340.c:207-244: Han Solo (0x241) 0x18 for both sides; a
        // major outside the six is unset.
        let mut world = GameWorld::default();
        let han = world.characters.insert(Character {
            dat_id: DatId::new(0x3000_0241),
            is_alliance: true,
            is_major: true,
            ..Character::default()
        });
        let other = world.characters.insert(Character {
            dat_id: DatId::new(0x3000_0250),
            is_alliance: true,
            is_major: true,
            ..Character::default()
        });
        assert_eq!(
            [true, false].map(|own| escape_advice(&world, han, own)),
            [Some(0x18); 2]
        );
        assert_eq!(
            [true, false].map(|own| escape_advice(&world, other, own)),
            [None; 2]
        );
    }
}

#[cfg(test)]
mod mission_end_message_tests {
    use super::*;
    use rebellion_core::ids::{DatId, SystemKey};
    use rebellion_core::missions::MissionEnded;
    use rebellion_core::world::{Character, ControlKind, System};
    use std::collections::HashMap;

    /// The shipped TEXTSTRA entries `0x70e6`, `0x70e7`, `0x7138`-`0x7145`
    /// and `0x7148`-`0x714b`.
    fn templates() -> rebellion_data::text_templates::TextTemplates {
        let entries: HashMap<u16, &str> = [
            (0x70e6, "\nPersonnel are returning to |\u{1}\u{1}\0\0\0.\u{1}"),
            (0x70e7, "\nAll personnel assigned to the mission have failed to return.\u{1}"),
            (0x7138, "|\u{1}\u{1}\0\0\0 Mission Failed\u{1}"),
            (0x7139, "My |\u{2}\u{1}\0\0\0 Mission to |\u{3}\u{1}\0\0\0 has failed because the target was not found at that location.  \u{1}"),
            (0x713a, "|\u{2}\u{1}\0\0\0 Mission Failed\u{1}"),
            (0x713b, "The |\u{2}\u{1}\0\0\0 Mission to |\u{3}\u{1}\0\0\0 has failed because the target was not found at that location.  \u{1}"),
            (0x713c, "My |\u{2}\u{1}\0\0\0 Mission to |\u{3}\u{1}\0\0\0 has failed because we have no research facilities at that location.\u{1}"),
            (0x713d, "The |\u{2}\u{1}\0\0\0 Mission to |\u{3}\u{1}\0\0\0 has failed because we have no research facilities at that location.  \u{1}"),
            (0x713e, "Enemy Mission Foiled\u{1}"),
            (0x713f, "An enemy |\u{1}\u{1}\0\0\0 mission to |\u{2}\u{1}\0\0\0 has been foiled by our forces.\u{1}"),
            (0x7140, "|\u{1}\u{1}\0\0\0 Mission Aborted\u{1}"),
            (0x7141, "My |\u{2}\u{1}\0\0\0 mission to |\u{3}\u{1}\0\0\0 has been aborted because the system has joined the |\u{4}\u{1}\0\0\0.  \u{1}"),
            (0x7142, "|\u{2}\u{1}\0\0\0 Mission Aborted.\u{1}"),
            (0x7143, "The |\u{2}\u{1}\0\0\0 mission to |\u{3}\u{1}\0\0\0 has been aborted because the system has joined the |\u{4}\u{1}\0\0\0.  \u{1}"),
            (0x7144, "My |\u{2}\u{1}\0\0\0 mission to |\u{3}\u{1}\0\0\0 has been aborted because the system has declared neutrality.\u{1}"),
            (0x7145, "The |\u{2}\u{1}\0\0\0 mission to |\u{3}\u{1}\0\0\0 has been aborted because the system has declared neutrality.\u{1}"),
            (0x7148, "|\u{1}\u{1}\0\0\0 Mission Foiled\u{1}"),
            (0x7149, "My |\u{2}\u{1}\0\0\0 mission to |\u{3}\u{1}\0\0\0 has been foiled by opposing forces.  \u{1}"),
            (0x714a, "|\u{2}\u{1}\0\0\0 Mission Foiled\u{1}"),
            (0x714b, "The |\u{2}\u{1}\0\0\0 mission to |\u{3}\u{1}\0\0\0 has been foiled by opposing forces.  \u{1}"),
        ]
        .into_iter()
        .collect();
        rebellion_data::text_templates::TextTemplates::from_json(
            &serde_json::to_vec(&entries).unwrap(),
        )
        .unwrap()
    }

    /// A world with the target system Coruscant held by `holder`, and a
    /// `side` agent there: a minor (MNCHARSD 832) or Han Solo (MJCHARSD
    /// 579). port: test fixture values, not rules.
    fn world_with(
        is_alliance: bool,
        major: bool,
        holder: ControlKind,
    ) -> (GameWorld, SystemKey, CharacterKey) {
        let mut world = GameWorld::default();
        let target = world.systems.insert(System {
            name: "Coruscant".into(),
            control: holder,
            ..System::default()
        });
        let agent = world.characters.insert(Character {
            name: if major { "Han Solo" } else { "Agent" }.into(),
            dat_id: DatId::new(if major { 0x3000_0243 } else { 0x3800_0340 }),
            is_major: major,
            is_alliance,
            is_empire: !is_alliance,
            current_system: Some(target),
            ..Character::default()
        });
        (world, target, agent)
    }

    fn ended(
        kind: MissionKind,
        is_alliance: bool,
        end_code: u8,
        target: SystemKey,
        lead: MissionMember,
    ) -> MissionEnded {
        MissionEnded {
            mission_id: 42,
            kind,
            faction: if is_alliance {
                MissionFaction::Alliance
            } else {
                MissionFaction::Empire
            },
            end_code,
            tick: 467,
            ordered_tick: 420,
            rolls: 0,
            team: vec![lead],
            captured: Vec::new(),
            origin: None,
            set_out_from: None,
            target_system: target,
            lead: Some(lead),
            result: 0,
        }
    }

    /// Each filed message as (title, body, background, overlay, sound,
    /// advice, audience).
    fn shown(
        messages: &[GameMessage],
    ) -> Vec<(String, String, u16, u16, u16, Option<u8>, RailAudience)> {
        messages
            .iter()
            .map(|message| {
                let display = message.display.as_ref().unwrap();
                assert_eq!(message.rail, Some(MessageRail::Mission));
                (
                    display.title.clone(),
                    display.body.clone(),
                    display.background,
                    display.overlay,
                    display.sound,
                    message.advice,
                    message.audience,
                )
            })
            .collect()
    }

    #[test]
    fn a_detected_mission_is_foiled_for_its_side_and_for_the_defender() {
        // FUN_00491500 on 0x1f1, codes 3 and 4: the lead's side reads
        // 0x714a/0x714b (a minor speaks of "The" mission), picture
        // 0x416/0x417 with the lead's portrait over it (FUN_004c5000:
        // MNCHARSD 832 is STRATEGY 6720), sound 0x480, code 0x15, then
        // 0x70e6; the defender reads 0x713e/0x713f, picture 0x431/0x432,
        // sound 0x47c/0x47d (FUN_00491500.c:87-105, FUN_00547f60.c:71-95).
        for (is_alliance, code) in [(true, 4), (false, 3)] {
            let (world, target, agent) =
                world_with(is_alliance, false, ControlKind::Controlled(Faction::Empire));
            let ended = ended(
                MissionKind::Sabotage,
                is_alliance,
                code,
                target,
                MissionMember::Character(agent),
            );

            let messages = mission_end_messages(&world, &templates(), &ended, Some("Yavin"), 0);

            let (own, other) = (u16::from(!is_alliance), u16::from(is_alliance));
            assert_eq!(
                shown(&messages),
                [
                    (
                        "Sabotage Mission Foiled".to_owned(),
                        "The Sabotage mission to Coruscant has been foiled by opposing forces.  \nPersonnel are returning to Yavin.".to_owned(),
                        0x416 + own,
                        6720,
                        0x480,
                        Some(0x15),
                        RailAudience::side(is_alliance),
                    ),
                    (
                        "Enemy Mission Foiled".to_owned(),
                        "An enemy Sabotage mission to Coruscant has been foiled by our forces.".to_owned(),
                        0x431 + other,
                        0,
                        0x47c + other,
                        None,
                        RailAudience::side(!is_alliance),
                    ),
                ],
                "{is_alliance}"
            );
        }
    }

    #[test]
    fn a_major_lead_speaks_in_the_first_person_and_only_its_side_hears_a_resignation() {
        // Code 5 (every member resigned) is foiled for the mission's side
        // alone; Han Solo (0x30 family) titles it 0x7148 by name, writes
        // 0x7149, shows his portrait (STRATEGY 6211) and carries his code
        // 0x19; with no one left free the body ends with 0x70e7.
        for is_alliance in [true, false] {
            let (world, target, agent) =
                world_with(is_alliance, true, ControlKind::Controlled(Faction::Empire));
            let ended = ended(
                MissionKind::DeathStarSabotage,
                is_alliance,
                5,
                target,
                MissionMember::Character(agent),
            );

            let messages = mission_end_messages(&world, &templates(), &ended, None, 0);

            assert_eq!(
                shown(&messages),
                [(
                    "Han Solo Mission Foiled".to_owned(),
                    "My Death Star Sabotage mission to Coruscant has been foiled by opposing forces.  \nAll personnel assigned to the mission have failed to return.".to_owned(),
                    0x416 + u16::from(!is_alliance),
                    6211,
                    0x480,
                    Some(0x19),
                    RailAudience::side(is_alliance),
                )],
                "{is_alliance}"
            );
        }
    }

    #[test]
    fn a_lost_target_fails_the_mission_except_for_research_which_fails_on_its_facilities() {
        // Codes 6, 7, 9 and 10 read 0x713a/0x713b (none for family 0x53,
        // Research); code 0xc reads 0x713a/0x713d for Research alone; base
        // picture 0x412/0x413, sound 0x47c/0x47d; codes 1, 2 and 0xb file
        // nothing (FUN_00491500.c:301-306, :376-386, :566-640).
        let research = MissionKind::Research(rebellion_core::research::TechType::Ship);
        for is_alliance in [true, false] {
            let (world, target, agent) =
                world_with(is_alliance, false, ControlKind::Controlled(Faction::Empire));
            let lead = MissionMember::Character(agent);
            let side = u16::from(!is_alliance);
            let titles = |kind, code| {
                shown(&mission_end_messages(
                    &world,
                    &templates(),
                    &ended(kind, is_alliance, code, target, lead),
                    Some("Yavin"),
        0,
                ))
                .into_iter()
                .map(|(title, body, background, _, sound, _, _)| (title, body, background, sound))
                .collect::<Vec<_>>()
            };
            let failed = |name: &str, why: &str| {
                vec![(
                    format!("{name} Mission Failed"),
                    format!("The {name} Mission to Coruscant has failed because {why}\nPersonnel are returning to Yavin."),
                    0x412 + side,
                    0x47c + side,
                )]
            };
            for code in [6, 7, 9, 10] {
                assert_eq!(
                    titles(MissionKind::Espionage, code),
                    failed("Espionage", "the target was not found at that location.  "),
                    "{is_alliance} {code}"
                );
                assert!(titles(research, code).is_empty(), "{is_alliance} {code}");
            }
            assert_eq!(
                titles(research, 0xc),
                failed(
                    "Ship Design Research",
                    "we have no research facilities at that location.  "
                )
            );
            for code in [0, 1, 2, 0xb, 0xd, 0x10] {
                assert!(titles(MissionKind::Espionage, code).is_empty(), "{code}");
            }
            assert!(titles(MissionKind::Espionage, 0xc).is_empty());
        }
    }

    #[test]
    fn a_target_that_changes_sides_aborts_the_mission_naming_its_new_holder() {
        // Code 8: 0x7142 with 0x7143 "has joined the {side}" for the new
        // holder (FUN_004f3dd0), or 0x7145 when the system declared
        // neutrality (owner bits 3) (FUN_00491500.c:452-563).
        for is_alliance in [true, false] {
            for (holder, ending) in [
                (
                    ControlKind::Controlled(Faction::Alliance),
                    "has joined the Alliance.  ",
                ),
                (
                    ControlKind::Controlled(Faction::Empire),
                    "has joined the Empire.  ",
                ),
                (ControlKind::Uncontrolled, "has declared neutrality."),
            ] {
                let (world, target, agent) = world_with(is_alliance, false, holder);
                let messages = mission_end_messages(
                    &world,
                    &templates(),
                    &ended(
                        MissionKind::Diplomacy,
                        is_alliance,
                        8,
                        target,
                        MissionMember::Character(agent),
                    ),
                    Some("Yavin"),
        0,
                );
                let display = messages[0].display.as_ref().unwrap();
                assert_eq!(
                    (messages.len(), display.title.as_str(), display.body.clone()),
                    (
                        1,
                        "Diplomacy Mission Aborted.",
                        format!("The Diplomacy mission to Coruscant has been aborted because the system {ending}\nPersonnel are returning to Yavin.")
                    ),
                    "{is_alliance} {holder:?}"
                );
            }
        }
    }

    #[test]
    fn an_outcome_s_result_a_hidden_record_or_a_move_files_no_end_message() {
        // FUN_00488de0.c:317-330 gives notification 0x1f1 its class (0x17)
        // only for a mission outside families 0x40-0x4f whose record is not
        // hidden (FUN_00520b70) and whose result +0x60 is 0.
        for is_alliance in [true, false] {
            let (mut world, target, agent) =
                world_with(is_alliance, false, ControlKind::Controlled(Faction::Empire));
            let lead = MissionMember::Character(agent);
            let filed = |world: &GameWorld, ended: &MissionEnded| {
                mission_end_messages(world, &templates(), ended, Some("Yavin"), 0).len()
            };
            let foiled = ended(MissionKind::Sabotage, is_alliance, 4, target, lead);
            assert_eq!(filed(&world, &foiled), 2);
            let resolved = MissionEnded {
                result: 3,
                ..foiled.clone()
            };
            assert_eq!(filed(&world, &resolved), 0, "{is_alliance}");
            let moved = ended(MissionKind::Move, is_alliance, 6, target, lead);
            assert_eq!(filed(&world, &moved), 0, "{is_alliance}");
            world.mission_records = vec![rebellion_core::world::MissionRecord {
                dat_id: MissionKind::Sabotage.record_id().unwrap(),
                timer_min_days: 1,
                timer_spread_days: 2,
                repeats: false,
                hidden: true,
                detection_phases: true,
                can_resign: true,
                return_policy: 1,
                rules: Default::default(),
                members: Default::default(),
            }];
            assert_eq!(filed(&world, &foiled), 0, "{is_alliance}");
        }
    }

    #[test]
    fn the_end_message_opens_its_free_lead_and_the_defender_s_opens_the_system() {
        // FUN_00491500.c:128-137: the subject key +0x5c is the lead unless
        // it is a prisoner (+0xac bit 0), else the target location; the
        // defender's is the location (:84).
        for is_alliance in [true, false] {
            let (mut world, target, agent) =
                world_with(is_alliance, false, ControlKind::Controlled(Faction::Empire));
            let lead = MissionMember::Character(agent);
            let foiled = ended(MissionKind::Sabotage, is_alliance, 4, target, lead);
            let targets = |world: &GameWorld| {
                mission_end_messages(world, &templates(), &foiled, Some("Yavin"), 0)
                    .iter()
                    .map(|message| message.target)
                    .collect::<Vec<_>>()
            };

            assert_eq!(
                targets(&world),
                [
                    Some(MessageTarget::Personnel(lead)),
                    Some(MessageTarget::System(target))
                ],
                "{is_alliance}"
            );
            world.characters[agent].is_captive = true;
            assert_eq!(
                targets(&world),
                [
                    Some(MessageTarget::System(target)),
                    Some(MessageTarget::System(target))
                ],
                "{is_alliance}"
            );
        }
    }

    #[test]
    fn a_personnel_display_opens_its_fleet_its_mission_or_its_system_s_defenses() {
        // FUN_00429440, families 0x30-0x3f: a ship parent opens the Fleet
        // window (kind 4); a system parent opens kind 11 while the object's
        // mission is not hidden (FUN_00520b70), else kind 10; an abode
        // (0x98) opens nothing.
        use rebellion_core::world::{Abode, Fleet, SpecialForceUnit};
        for is_alliance in [true, false] {
            let (mut world, target, agent) =
                world_with(is_alliance, false, ControlKind::Controlled(Faction::Empire));
            let movement = MovementState::new();
            let member = MissionMember::Character(agent);
            let open = |world: &GameWorld, member| personnel_window(world, &movement, member);

            assert_eq!(open(&world, member), Some((target, FinderTarget::Defenses)));
            world.characters[agent].on_mission = true;
            assert_eq!(open(&world, member), Some((target, FinderTarget::Missions(member))));
            world.characters[agent].on_hidden_mission = true;
            assert_eq!(open(&world, member), Some((target, FinderTarget::Defenses)));
            let fleet = world.fleets.insert(Fleet {
                location: target,
                is_alliance,
                ..Fleet::default()
            });
            world.characters[agent].current_fleet = Some(fleet);
            assert_eq!(open(&world, member), Some((target, FinderTarget::Fleet(fleet))));
            let jailed = &mut world.characters[agent];
            jailed.current_fleet = None;
            jailed.current_system = None;
            jailed.is_captive = true;
            jailed.abode = Some(Abode::jail_of(!is_alliance));
            assert_eq!(open(&world, member), None);

            let unit = world.special_forces.insert(SpecialForceUnit {
                is_alliance,
                ..SpecialForceUnit::default()
            });
            world.systems[target].special_forces.push(unit);
            let unit = MissionMember::SpecialForce(unit);
            assert_eq!(open(&world, unit), Some((target, FinderTarget::Defenses)));
            if let MissionMember::SpecialForce(key) = unit {
                world.special_forces[key].on_mission = true;
            }
            assert_eq!(open(&world, unit), Some((target, FinderTarget::Missions(unit))));
        }
    }

    #[test]
    fn a_special_force_lead_shows_its_own_picture_unless_the_mission_is_foiled() {
        // FUN_00491500.c:285-313: a lead outside 0x30-0x3b puts
        // FUN_004c5000's id in the background +0x2e, (SPECFCSD TEXTSTRA id
        // & 0xfff) + 0x1000 (FUN_004f63d0): STRATEGY 0x1540 for record 1,
        // 0x1584 for record 9; codes 3-5 replace it with 0x416/0x417 (no
        // Reconnaissance kind). No overlay and no advice code.
        use rebellion_core::world::{BuildableClass, SpecialForceUnit};
        for (is_alliance, record, picture) in [(true, 1, 0x1540), (false, 9, 0x1584)] {
            let (mut world, target, _) =
                world_with(is_alliance, false, ControlKind::Controlled(Faction::Empire));
            let class = DatId::new(0x3c00_0000 | record);
            world.buildable_classes.insert(
                class,
                BuildableClass {
                    name: "Commandos".into(),
                    ..BuildableClass::default()
                },
            );
            let unit = world.special_forces.insert(SpecialForceUnit {
                class_dat_id: class,
                is_alliance,
                ..SpecialForceUnit::default()
            });
            world.systems[target].special_forces.push(unit);
            let lead = MissionMember::SpecialForce(unit);
            let shown = |code| {
                let messages = mission_end_messages(
                    &world,
                    &templates(),
                    &ended(MissionKind::Espionage, is_alliance, code, target, lead),
                    Some("Yavin"),
        0,
                );
                let display = messages[0].display.clone().unwrap();
                (display.title, display.background, display.overlay, messages[0].advice)
            };

            assert_eq!(
                shown(6),
                ("Espionage Mission Failed".to_owned(), picture, 0, None),
                "{is_alliance}"
            );
            assert_eq!(
                shown(4),
                (
                    "Espionage Mission Foiled".to_owned(),
                    0x416 + u16::from(!is_alliance),
                    0,
                    None
                ),
                "{is_alliance}"
            );
        }
    }

    #[test]
    fn a_picture_strategy_lacks_reads_its_encybmap_edata_file() {
        // FUN_0046a280 -> FUN_0045f970: ENCYBMAP's string for the id names
        // the EDATA file (the W4 fixture maps 5952 to EDATA.014).
        let session = crate::encyclopedia_surface::fixture::fixture_session().unwrap();
        let expected = session.artwork_bytes("EDATA.014").map(<[u8]>::to_vec);
        assert!(expected.is_some());
        assert_eq!(encyclopedia_picture(Some(&session), 5952), expected);
        assert_eq!(encyclopedia_picture(Some(&session), 5440), None);
        assert_eq!(encyclopedia_picture(None, 5952), None);
    }

    #[test]
    fn a_major_lead_s_voice_list_is_its_own_and_a_trained_luke_s_differs() {
        // FUN_00491500.c:214-256 draws +0x32 through FUN_004c4990 from
        // 0xf3 (0x240), 0xdf (0x241), 0xcb or 0xd5 by +0xb0 bit 3 (0x242),
        // 0xe9 (0x243), 0x107 (0x280), 0xfd (0x281); minors have none.
        for (record, untrained, trained) in [
            (0x240, 0xf3, 0xf3),
            (0x241, 0xdf, 0xdf),
            (0x242, 0xcb, 0xd5),
            (0x243, 0xe9, 0xe9),
            (0x280, 0x107, 0x107),
            (0x281, 0xfd, 0xfd),
        ] {
            let is_alliance = record < 0x280;
            let (mut world, target, agent) =
                world_with(is_alliance, true, ControlKind::Controlled(Faction::Empire));
            world.characters[agent].dat_id = DatId::new(0x3000_0000 | record);
            let ended = ended(
                MissionKind::Sabotage,
                is_alliance,
                4,
                target,
                MissionMember::Character(agent),
            );
            assert_eq!(
                [false, true].map(|luke| mission_end_voice_list(&world, &ended, luke)),
                [Some(untrained), Some(trained)],
                "{record:#x}"
            );
        }
        for is_alliance in [true, false] {
            let (world, target, agent) =
                world_with(is_alliance, false, ControlKind::Controlled(Faction::Empire));
            let ended = ended(
                MissionKind::Sabotage,
                is_alliance,
                4,
                target,
                MissionMember::Character(agent),
            );
            assert_eq!(mission_end_voice_list(&world, &ended, true), None);
        }
    }

    #[test]
    fn the_voice_line_goes_with_the_mission_s_own_message_only() {
        // FUN_00491500 sets +0x32 on the lead's side; the defender's
        // "Enemy Mission Foiled" (:87-105) has none.
        for is_alliance in [true, false] {
            let (world, target, agent) =
                world_with(is_alliance, true, ControlKind::Controlled(Faction::Empire));
            let ended = ended(
                MissionKind::Sabotage,
                is_alliance,
                4,
                target,
                MissionMember::Character(agent),
            );
            let voices: Vec<u16> =
                mission_end_messages(&world, &templates(), &ended, Some("Yavin"), 1358)
                    .iter()
                    .map(|message| message.display.as_ref().unwrap().voice)
                    .collect();
            assert_eq!(voices, [1358, 0], "{is_alliance}");
        }
    }

    #[test]
    fn a_message_starts_each_non_zero_wave_and_its_voice_from_the_player_s_sprite_dll() {
        // FUN_00469de0: +0x30 from module 7 and +0x32 from module 9 (the
        // player's ALSPRITE or EMSPRITE), each only when non-zero.
        for (faction, dll) in [
            (AdvisorFaction::Alliance, "alsprite-dll"),
            (AdvisorFaction::Empire, "emsprite-dll"),
        ] {
            let advisor = AdvisorState::new(faction);
            let waves = |sound, voice| message_waves(MessageSounds { sound, voice }, &advisor);
            assert_eq!(waves(0x480, 1358), (Some(0x480), Some(format!("{dll}/1358"))));
            assert_eq!(waves(0, 0), (None, None));
            assert_eq!(waves(0x47c, 0), (Some(0x47c), None));
            assert_eq!(waves(0, 1387), (None, Some(format!("{dll}/1387"))));
        }
    }

    #[test]
    fn showing_a_message_stops_the_last_waves_and_plays_its_sound_and_voice() {
        // FUN_00469de0: FUN_0046a6e0 stops both waves, then +0x30 plays
        // from module 7 and +0x32 from module 9 when non-zero.
        let mut log = MessageLog::default();
        let voiced = log.push(GameMessage::new(1, "Foiled", MessageCategory::Mission).with_display(
            MessageDisplay {
                sound: 0x480,
                voice: 1358,
                ..MessageDisplay::default()
            },
        ));
        let plain = log.push(GameMessage::new(1, "Plain", MessageCategory::Mission));
        let mut clock = GameClock::new();
        let shown = |log: &mut MessageLog, clock: &mut GameClock, id| {
            let mut sounds = None;
            apply_message_index_actions(
                vec![MessageIndexAction::MessageDisplayed(id)],
                log,
                clock,
                &mut None,
                &mut sounds,
            );
            sounds
        };

        assert_eq!(
            shown(&mut log, &mut clock, voiced),
            Some(MessageSounds {
                sound: 0x480,
                voice: 1358
            })
        );
        assert_eq!(shown(&mut log, &mut clock, plain), Some(MessageSounds::default()));
    }

    #[test]
    fn a_mission_with_no_lead_or_no_text_files_nothing_for_its_side() {
        // FUN_00491500.c:109-124 files the mission's side only with its
        // lead's copy; the text is TEXTSTRA's alone.
        for is_alliance in [true, false] {
            let (mut world, target, agent) =
                world_with(is_alliance, false, ControlKind::Controlled(Faction::Empire));
            let mut gone = ended(
                MissionKind::Sabotage,
                is_alliance,
                5,
                target,
                MissionMember::Character(agent),
            );
            let empty = rebellion_data::text_templates::TextTemplates::default();
            assert!(mission_end_messages(&world, &empty, &gone, None, 0).is_empty());
            world.characters[agent].mark_killed();
            assert!(mission_end_messages(&world, &templates(), &gone, None, 0).is_empty());
            gone.lead = None;
            assert!(mission_end_messages(&world, &templates(), &gone, None, 0).is_empty());
        }
    }

    #[test]
    fn personnel_return_to_their_return_s_system_else_where_they_stand() {
        // FUN_00491500.c:141-195: the first finished member still alive
        // and free names the system or fleet it is a child of; the port's
        // queued Return names its destination.
        for is_alliance in [true, false] {
            let (mut world, target, agent) =
                world_with(is_alliance, false, ControlKind::Controlled(Faction::Empire));
            let home = world.systems.insert(System {
                name: "Yavin".into(),
                ..System::default()
            });
            let prisoner = world.characters.insert(Character {
                name: "Prisoner".into(),
                is_alliance,
                is_empire: !is_alliance,
                is_captive: true,
                current_system: Some(target),
                ..Character::default()
            });
            let team = vec![
                MissionMember::Character(prisoner),
                MissionMember::Character(agent),
            ];
            let mut ended = ended(MissionKind::Sabotage, is_alliance, 4, target, team[1]);
            ended.team = team;
            let mut missions = MissionState::new();

            assert_eq!(
                mission_returning_place(&world, &missions, &ended, &[]),
                Some("Coruscant".to_owned())
            );

            let id = missions.dispatch(rebellion_core::missions::MissionRequest::single(
                MissionKind::Move,
                ended.faction,
                agent,
                home,
                None,
                467,
            ));
            assert_eq!(
                mission_returning_place(&world, &missions, &ended, &[id]),
                Some("Yavin".to_owned())
            );
            assert_eq!(mission_returning_place(&world, &missions, &ended, &[]), Some("Coruscant".to_owned()));

            world.characters[agent].is_captive = true;
            assert_eq!(mission_returning_place(&world, &missions, &ended, &[id]), None);
        }
    }
}

#[cfg(test)]
mod interface_restore_tests {
    use super::*;
    use rebellion_core::ids::DatId;
    use rebellion_data::save::{SavedInterface, SavedWindow, SavedWindowKind};

    /// A sector of three systems. port: test fixture values, not rules.
    fn world() -> (GameWorld, [SystemKey; 4]) {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(rebellion_core::world::Sector {
            dat_id: DatId::new(0x9200_0001),
            ..rebellion_core::world::Sector::default()
        });
        let systems = [1, 2, 3, 4].map(|index| {
            world.systems.insert(rebellion_core::world::System {
                dat_id: DatId::new(0x9000_0000 + index),
                sector,
                ..rebellion_core::world::System::default()
            })
        });
        world.sectors[sector].systems = systems.to_vec();
        (world, systems)
    }

    struct Windows {
        sectors: SectorWindowState,
        systems: SystemWindowState,
        fleets: FleetWindowState,
        defenses: DefensesWindowState,
        missions: MissionsWindowState,
    }

    impl Windows {
        fn new() -> Self {
            Self {
                sectors: SectorWindowState::default(),
                systems: SystemWindowState::default(),
                fleets: FleetWindowState::default(),
                defenses: DefensesWindowState::default(),
                missions: MissionsWindowState::default(),
            }
        }

        fn children(&self) -> Vec<(u64, GalaxyChild)> {
            self.children_in(&strategic_windowing::StrategicWindowing::new().begin_frame())
        }

        fn children_in(
            &self,
            snapshot: &rebellion_windowing::RoutingSnapshot,
        ) -> Vec<(u64, GalaxyChild)> {
            galaxy_children(
                &self.sectors,
                &self.systems,
                &self.defenses,
                &self.missions,
                &self.fleets,
                snapshot,
            )
        }

        fn saved(
            &self,
            world: &GameWorld,
            cockpit: &rebellion_render::CockpitState,
        ) -> SavedInterface {
            saved_interface(
                world,
                cockpit,
                &self.sectors,
                &self.systems,
                &self.fleets,
                &self.defenses,
                &self.missions,
            )
        }
    }

    fn fog(faction: CockpitFaction) -> FogState {
        FogState::new(Faction::of_alliance(faction == CockpitFaction::Alliance))
    }

    /// In kind, the galaxy of Empire day-2 start played on to day 6 under
    /// Wine: a sector window; from the bottom up a minimized System window,
    /// a Defenses window, a minimized Defenses window, a Missions window and
    /// a System window over them.
    fn opened(world: &GameWorld, systems: [SystemKey; 4], faction: CockpitFaction) -> Windows {
        let layout = rebellion_render::CockpitState::new(faction).layout_for(1280.0, 960.0);
        let mut windows = Windows::new();
        assert!(windows.sectors.open_for_system(world, systems[0], faction));
        assert!(windows.systems.restore_saved(
            world,
            faction,
            &SavedWindow {
                kind: SavedWindowKind::System {
                    system: DatId::new(0x9000_0002),
                },
                position: (90, 50),
                shown: false,
            },
        ));
        assert!(windows
            .defenses
            .open(world, systems[1], (100, 60), faction, layout));
        // A minimized window keeps its place in the z-order.
        assert!(windows
            .defenses
            .open(world, systems[2], (130, 75), faction, layout));
        let serial = windows.defenses.focus_serial();
        assert!(windows.defenses.close_window(systems[2]));
        windows
            .systems
            .minimize_defenses_window_at(systems[2], (130, 75), serial);
        assert!(windows.missions.open(
            world,
            &fog(faction),
            &MissionState::new(),
            systems[2],
            (110, 70),
            faction,
            layout,
        ));
        assert!(windows
            .systems
            .open(world, systems[0], (120, 80), faction, layout));
        windows
    }

    fn kinds(interface: &SavedInterface) -> Vec<(SavedWindowKind, bool)> {
        interface
            .windows
            .iter()
            .map(|window| (window.kind, window.shown))
            .collect()
    }

    #[test]
    fn a_save_keeps_every_window_from_the_bottom_of_the_z_order_up() {
        // FUN_00440ea0 writes every child of the view from the bottom of
        // the z-order up, a hidden one (+0x110 = 0) at its own place
        // (FUN_00440ea0.c:19-24).
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let (world, systems) = world();
            let windows = opened(&world, systems, faction);
            let cockpit = rebellion_render::CockpitState::new(faction);
            let interface = windows.saved(&world, &cockpit);
            assert_eq!(interface.gid_mode, Some(cockpit.gid_mode.command_id()));
            let system = |id| SavedWindowKind::System {
                system: DatId::new(id),
            };
            assert_eq!(
                kinds(&interface),
                [
                    (
                        SavedWindowKind::Sector {
                            sector: DatId::new(0x9200_0001)
                        },
                        true
                    ),
                    (system(0x9000_0002), false),
                    (
                        SavedWindowKind::Defenses {
                            system: DatId::new(0x9000_0002)
                        },
                        true
                    ),
                    (
                        SavedWindowKind::Defenses {
                            system: DatId::new(0x9000_0003)
                        },
                        false
                    ),
                    (
                        SavedWindowKind::Missions {
                            system: DatId::new(0x9000_0003)
                        },
                        true
                    ),
                    (system(0x9000_0001), true),
                ],
                "{faction:?}"
            );
        }
    }

    #[test]
    fn a_window_the_gid_menu_hides_saves_hidden_but_a_sector_stays_shown() {
        // GID's slot +0x08 clears +0x110 on each child it hides. Under Wine
        // a System window the menu hid came back on the rail, and the
        // sector window it hid came back shown (Empire day-2 start, played
        // on to day 8).
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let (world, systems) = world();
            let windows = opened(&world, systems, faction);
            let mut cockpit = rebellion_render::CockpitState::new(faction);
            cockpit.gid_ui.open_menu();
            let shown: Vec<bool> = kinds(&windows.saved(&world, &cockpit))
                .into_iter()
                .map(|(_, shown)| shown)
                .collect();
            assert_eq!(
                shown,
                [true, false, false, false, false, false],
                "{faction:?}"
            );
        }
    }

    #[test]
    fn a_load_rebuilds_the_saved_order_and_focuses_the_top_window() {
        // FUN_00440f80 → FUN_00441190 rebuilds the windows bottom first,
        // the rail's back on the rail and sectors at the bottom; the focus
        // is the top shown window that is not a sector (Wine runs r, t1,
        // t3, u and w1, empire-fleet-to-yaga-minor at day 38).
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let (world, systems) = world();
            let windows = opened(&world, systems, faction);
            let cockpit = rebellion_render::CockpitState::new(faction);
            let interface = windows.saved(&world, &cockpit);

            let mut restored = Windows::new();
            let mut strategic = strategic_windowing::StrategicWindowing::new();
            restore_interface(
                &interface.windows,
                &world,
                &fog(faction),
                &MissionState::new(),
                faction,
                cockpit.layout_for(1280.0, 960.0),
                RestoredWindows {
                    sectors: &mut restored.sectors,
                    systems: &mut restored.systems,
                    fleets: &restored.fleets,
                    defenses: &mut restored.defenses,
                    missions: &mut restored.missions,
                },
                &mut strategic,
            );
            let hidden = finish_interface_restore(
                &interface.windows,
                &world,
                &mut restored.systems,
                &mut restored.defenses,
                &mut restored.missions,
                &mut restored.fleets,
                &mut strategic,
                &mut 0,
            );
            assert!(hidden.is_empty());

            let order = |windows: &Windows| {
                let mut children = windows.children();
                children.sort_by_key(|(serial, _)| *serial);
                children
                    .into_iter()
                    .map(|(_, child)| child)
                    .collect::<Vec<_>>()
            };
            assert_eq!(order(&restored), order(&windows), "{faction:?}");
            assert_eq!(restored.systems.rail_count(), 2);
            assert_eq!(kinds(&restored.saved(&world, &cockpit)), kinds(&interface));
            let focus = GalaxyFocusState::after_load(restored.children());
            assert_eq!(
                focus.focus,
                GalaxyFocus::Child(GalaxyChild::System(systems[0]))
            );
        }
    }

    /// Commit the manager's pending opens as the frame's end does: publish
    /// each Fleet window and note its activation.
    fn commit_fleets(
        strategic: &mut strategic_windowing::StrategicWindowing,
        fleets: &mut FleetWindowState,
    ) {
        for mut applied in strategic.apply_pending() {
            let (Some(prepared), Some(id), Ok(commit)) = (
                applied.prepared_fleet.take(),
                applied.fleet_content_id,
                applied.result.as_ref(),
            ) else {
                continue;
            };
            assert_eq!(fleets.commit_open(prepared, &commit.transition), Some(id));
            assert!(strategic.acknowledge_fleet_publication(id));
            fleets.note_activation(id);
        }
    }

    #[test]
    fn a_fleet_window_saved_on_top_comes_back_in_front_with_the_keyboard() {
        // FUN_00441190 makes the saved windows bottom first, each above the
        // last, and the top shown one takes the focus (Wine runs r, t1, t3,
        // u and w1, empire-fleet-to-yaga-minor at day 38). A Fleet window
        // saved over a System window paints over it again, and holds the
        // keyboard; saved under it, the System window stays in front.
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            for fleet_on_top in [true, false] {
                let (world, systems) = world();
                let fleet = SavedWindow {
                    kind: SavedWindowKind::Fleet {
                        system: DatId::new(0x9000_0001),
                        tab: 0x66,
                    },
                    position: (130, 60),
                    shown: true,
                };
                let system = SavedWindow {
                    kind: SavedWindowKind::System {
                        system: DatId::new(0x9000_0002),
                    },
                    position: (120, 80),
                    shown: true,
                };
                let order = if fleet_on_top {
                    [system, fleet]
                } else {
                    [fleet, system]
                };
                let cockpit = rebellion_render::CockpitState::new(faction);
                let mut restored = Windows::new();
                let mut strategic = strategic_windowing::StrategicWindowing::new();
                restore_interface(
                    &order,
                    &world,
                    &fog(faction),
                    &MissionState::new(),
                    faction,
                    cockpit.layout_for(1280.0, 960.0),
                    RestoredWindows {
                        sectors: &mut restored.sectors,
                        systems: &mut restored.systems,
                        fleets: &restored.fleets,
                        defenses: &mut restored.defenses,
                        missions: &mut restored.missions,
                    },
                    &mut strategic,
                );
                commit_fleets(&mut strategic, &mut restored.fleets);
                // The commit brings the Fleet window forward, as the frame's
                // end does.
                let mut fleet_front = FleetWindowState::next_legacy_bridge_serial();
                let hidden = finish_interface_restore(
                    &order,
                    &world,
                    &mut restored.systems,
                    &mut restored.defenses,
                    &mut restored.missions,
                    &mut restored.fleets,
                    &mut strategic,
                    &mut fleet_front,
                );
                assert!(hidden.is_empty());
                let front = galaxy_front(
                    &restored.systems,
                    fleet_front,
                    &restored.defenses,
                    &restored.missions,
                );
                let id = restored.fleets.window_id(systems[0]).unwrap();
                let focus = focus_after_load(
                    restored.children_in(&strategic.begin_frame()),
                    &mut strategic,
                );
                let commands: Vec<_> = strategic
                    .apply_pending()
                    .into_iter()
                    .map(|applied| applied.command)
                    .collect();
                // The keyboard goes where the focus is: the Fleet window, or
                // the galaxy view's own windows through the root.
                let keyboard = strategic.begin_frame().keyboard_route().target;
                if fleet_on_top {
                    assert_eq!(front, GalaxyFront::Fleet, "{faction:?}");
                    assert_eq!(focus.focus, GalaxyFocus::Child(GalaxyChild::Fleet(id)));
                    assert_eq!(commands, [rebellion_windowing::WindowCommand::Activate(id)]);
                    assert!(
                        matches!(keyboard, rebellion_windowing::RouteTarget::Window(target) if target.window == id),
                        "{faction:?}"
                    );
                } else {
                    assert_eq!(front, GalaxyFront::System, "{faction:?}");
                    assert_eq!(
                        focus.focus,
                        GalaxyFocus::Child(GalaxyChild::System(systems[1]))
                    );
                    assert_eq!(commands, [rebellion_windowing::WindowCommand::FocusRoot]);
                    assert_eq!(
                        keyboard,
                        rebellion_windowing::RouteTarget::StrategicRoot,
                        "{faction:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn a_save_from_game_options_keeps_a_shown_fleet_window_shown_and_a_minimized_one_hidden() {
        // FUN_00440ea0 walks the galaxy view's child windows from the
        // bottom up and writes each one's shown flag (+0x110): the Game
        // Options window hides none of them, so a Fleet window open under it
        // saves shown and one on the rail saves hidden. Game Options
        // suspends the strategic scene, so its frame paints no Fleet window.
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let (world, systems) = world();
            let fleet = |system, shown| SavedWindow {
                kind: SavedWindowKind::Fleet {
                    system: DatId::new(system),
                    tab: 0x67,
                },
                position: (130, 60),
                shown,
            };
            let order = [fleet(0x9000_0001, true), fleet(0x9000_0002, false)];
            let cockpit = rebellion_render::CockpitState::new(faction);
            let mut windows = Windows::new();
            let mut strategic = strategic_windowing::StrategicWindowing::new();
            restore_interface(
                &order,
                &world,
                &fog(faction),
                &MissionState::new(),
                faction,
                cockpit.layout_for(1280.0, 960.0),
                RestoredWindows {
                    sectors: &mut windows.sectors,
                    systems: &mut windows.systems,
                    fleets: &windows.fleets,
                    defenses: &mut windows.defenses,
                    missions: &mut windows.missions,
                },
                &mut strategic,
            );
            commit_fleets(&mut strategic, &mut windows.fleets);
            finish_interface_restore(
                &order,
                &world,
                &mut windows.systems,
                &mut windows.defenses,
                &mut windows.missions,
                &mut windows.fleets,
                &mut strategic,
                &mut 0,
            );
            for applied in strategic.apply_pending() {
                if let Some((window, system, position)) = applied.minimized_fleet {
                    windows
                        .systems
                        .minimize_fleet_window(window, system, position);
                }
            }
            assert!(windows.fleets.window_id(systems[0]).is_some(), "{faction:?}");

            strategic.enter_scene(strategic_windowing::WindowScene::Suspended);
            let saved = saved_interface(
                &world,
                &cockpit,
                &windows.sectors,
                &windows.systems,
                &windows.fleets,
                &windows.defenses,
                &windows.missions,
            );
            assert_eq!(kinds(&saved), order.map(|window| (window.kind, window.shown)), "{faction:?}");
        }
    }

    #[test]
    fn a_load_keeps_fleet_windows_at_their_saved_places_and_rail_slots() {
        // FUN_00441190 creates each window above the last and appends each
        // hidden one to the rail as it meets it (slot +0x08, FUN_00428b40),
        // so a hidden Fleet window takes its rail slot among the hidden
        // windows and a shown one its place in the z-order. The port's
        // Fleet windows commit at the frame's end, after the others, so the
        // others are raised again in saved order and the hidden Fleet window
        // takes its slot before the manager's minimize.
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let (world, systems) = world();
            let fleet = |system, shown| SavedWindow {
                kind: SavedWindowKind::Fleet {
                    system: DatId::new(system),
                    tab: 0x67,
                },
                position: (130, 60),
                shown,
            };
            let window = |kind, shown| SavedWindow {
                kind,
                position: (120, 80),
                shown,
            };
            let order = [
                fleet(0x9000_0001, true),
                window(
                    SavedWindowKind::System {
                        system: DatId::new(0x9000_0002),
                    },
                    true,
                ),
                // A hidden window before the hidden Fleet window takes the
                // rail slot ahead of it.
                window(
                    SavedWindowKind::Defenses {
                        system: DatId::new(0x9000_0002),
                    },
                    false,
                ),
                fleet(0x9000_0004, false),
                fleet(0x9000_0003, true),
                window(
                    SavedWindowKind::Defenses {
                        system: DatId::new(0x9000_0001),
                    },
                    true,
                ),
                window(
                    SavedWindowKind::System {
                        system: DatId::new(0x9000_0003),
                    },
                    false,
                ),
                window(
                    SavedWindowKind::Missions {
                        system: DatId::new(0x9000_0001),
                    },
                    false,
                ),
            ];
            let cockpit = rebellion_render::CockpitState::new(faction);
            let mut restored = Windows::new();
            let mut strategic = strategic_windowing::StrategicWindowing::new();
            restore_interface(
                &order,
                &world,
                &fog(faction),
                &MissionState::new(),
                faction,
                cockpit.layout_for(1280.0, 960.0),
                RestoredWindows {
                    sectors: &mut restored.sectors,
                    systems: &mut restored.systems,
                    fleets: &restored.fleets,
                    defenses: &mut restored.defenses,
                    missions: &mut restored.missions,
                },
                &mut strategic,
            );
            commit_fleets(&mut strategic, &mut restored.fleets);
            let hidden = finish_interface_restore(
                &order,
                &world,
                &mut restored.systems,
                &mut restored.defenses,
                &mut restored.missions,
                &mut restored.fleets,
                &mut strategic,
                &mut 0,
            );
            let id = |system| restored.fleets.window_id(system).unwrap();
            let (first, third, hidden_fleet) = (id(systems[0]), id(systems[2]), id(systems[3]));
            assert_eq!(hidden, [GalaxyChild::Fleet(hidden_fleet)], "{faction:?}");
            assert_eq!(
                restored.fleets.selection(systems[0]).map(|(_, tab)| tab),
                Some(rebellion_render::fleet_window::FleetWindowTab::Fighters)
            );

            let mut children: Vec<_> = restored
                .children_in(&strategic.begin_frame())
                .into_iter()
                .filter(|(_, child)| !hidden.contains(child))
                .collect();
            children.sort_by_key(|(serial, _)| *serial);
            assert_eq!(
                children.iter().map(|(_, child)| *child).collect::<Vec<_>>(),
                [
                    GalaxyChild::Fleet(first),
                    GalaxyChild::System(systems[1]),
                    GalaxyChild::Fleet(third),
                    GalaxyChild::Defenses(systems[0]),
                ],
                "{faction:?}"
            );
            assert_eq!(
                GalaxyFocusState::after_load(children).focus,
                GalaxyFocus::Child(GalaxyChild::Defenses(systems[0]))
            );

            // The manager's minimize commits next frame; the rail keeps the
            // slot the load gave it.
            for applied in strategic.apply_pending() {
                if let Some((window, system, position)) = applied.minimized_fleet {
                    restored
                        .systems
                        .minimize_fleet_window(window, system, position);
                }
            }
            let rail: Vec<_> = restored
                .systems
                .saved_rail(&world, |_| Some((0x67, 0)))
                .into_iter()
                .map(|(_, window)| window.kind)
                .collect();
            assert_eq!(
                rail,
                [
                    SavedWindowKind::Defenses {
                        system: DatId::new(0x9000_0002),
                    },
                    SavedWindowKind::Fleet {
                        system: DatId::new(0x9000_0004),
                        tab: 0x67,
                    },
                    SavedWindowKind::System {
                        system: DatId::new(0x9000_0003),
                    },
                    SavedWindowKind::Missions {
                        system: DatId::new(0x9000_0001),
                    },
                ],
                "{faction:?}"
            );
            // Saved again, every window comes back in the same order.
            let again = saved_interface(
                &world,
                &cockpit,
                &restored.sectors,
                &restored.systems,
                &restored.fleets,
                &restored.defenses,
                &restored.missions,
            );
            assert_eq!(again.windows, order, "{faction:?}");
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod saved_ending_tests {
    use super::*;
    use rebellion_core::victory::{AD, AW, CA, EW, GO};
    use rebellion_render::advisor::CockpitStep;

    /// A campaign whose win word is `word`, saved to a file and loaded back
    /// through `LiveCampaign`, as the Load Game path does. port: fixture
    /// state, not rules.
    fn saved_and_loaded(word: u32, player_is_alliance: bool) -> VictoryState {
        let mut world = GameWorld::default();
        let mut clock = GameClock::new();
        let mut manufacturing = ManufacturingState::new();
        let mut missions = MissionState::new();
        let mut events = EventState::new();
        let mut ai = AIState::new(AiFaction::Empire);
        let mut movement = MovementState::new();
        let mut fog_alliance = FogState::new(Faction::Alliance);
        let mut fog_empire = FogState::new(Faction::Empire);
        let mut player_faction = if player_is_alliance {
            MissionFaction::Alliance
        } else {
            MissionFaction::Empire
        };
        let mut blockade = BlockadeState::new();
        let mut uprising = UprisingState::new();
        let mut research = ResearchState::new();
        let mut jedi = JediState::new();
        let mut victory = VictoryState::new(SystemKey::default(), SystemKey::default());
        victory.word = word;
        victory.resolved = true;
        let mut betrayal = BetrayalState::new();
        let mut economy = EconomyState::default();
        let mut sim_rng = Xoshiro256PlusPlus::seed_from_u64(7);
        let mut ai2 = None;
        let mut repair = RepairState::default();
        let mut troop_transport = TroopTransportState::default();
        let mut deliveries = DeliveryState::new();
        let mut player_agent = PlayerAgent::default();
        let mut stockpiles = StockpileState::new();
        let mut game_config = rebellion_core::tuning::GameConfig::default();
        let mut campaign_config = CampaignConfig::default();
        let mut campaign = LiveCampaign {
            world: &mut world,
            clock: &mut clock,
            manufacturing: &mut manufacturing,
            missions: &mut missions,
            events: &mut events,
            ai: &mut ai,
            movement: &mut movement,
            fog_alliance: &mut fog_alliance,
            fog_empire: &mut fog_empire,
            player_faction: &mut player_faction,
            blockade: &mut blockade,
            uprising: &mut uprising,
            research: &mut research,
            jedi: &mut jedi,
            victory: &mut victory,
            betrayal: &mut betrayal,
            economy: &mut economy,
            sim_rng: &mut sim_rng,
            ai2: &mut ai2,
            repair: &mut repair,
            troop_transport: &mut troop_transport,
            deliveries: &mut deliveries,
            player_agent: &mut player_agent,
            stockpiles: &mut stockpiles,
            game_config: &mut game_config,
            campaign_config: &mut campaign_config,
        };
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("ending.sav");
        rebellion_data::save::save_file(&path, "Ending", &campaign.snapshot(), &[]).unwrap();
        *campaign.victory = VictoryState::new(SystemKey::default(), SystemKey::default());
        let (_, state) = rebellion_data::save::load_file(&path, 0).unwrap();
        campaign.restore(state);
        victory
    }

    #[test]
    fn a_loaded_game_shows_the_gid_mode_it_was_saved_with() {
        // FUN_00440ea0 saves the view's GID mode (+0xc8) and FUN_00440f80
        // replays it as the row choice (WM_COMMAND 0x140); a save without
        // one shows the new game's Popular Support.
        for mode in [
            rebellion_render::GidMode::IdleFleets,
            rebellion_render::GidMode::DisplayOff,
            rebellion_render::GidMode::PopularSupport,
        ] {
            let interface = rebellion_data::save::SavedInterface {
                gid_mode: Some(mode.command_id()),
                windows: Vec::new(),
            };
            assert_eq!(saved_gid_mode(&interface), mode);
        }
        assert_eq!(
            saved_gid_mode(&rebellion_data::save::SavedInterface::default()),
            rebellion_render::GidMode::PopularSupport
        );
    }

    #[test]
    fn a_game_saved_after_the_win_plays_its_ending_once_loaded_on_either_side() {
        // FUN_004397a0 / FUN_00439550 save and load the advisor's slot table
        // (+0x164 count, +0x168 stamps), so the ending slot FUN_004c3010 /
        // FUN_004c0f10 stamped comes back with the game and the ending still
        // plays (FUN_004c3060 / FUN_004c0f60 → FUN_00425c90 → FUN_00414830).
        for (alliance, word, movie) in [
            (true, EW | GO | AD, 0x6a),
            (true, AW | GO | CA, 0x6b),
            (false, EW | GO | AD, 0x6d),
            (false, AW | GO | CA, 0x6c),
        ] {
            let loaded = saved_and_loaded(word, alliance);
            assert_eq!(loaded.word, word);
            let faction = if alliance {
                AdvisorFaction::Alliance
            } else {
                AdvisorFaction::Empire
            };
            let mut advisor = AdvisorState::new(faction);
            resume_saved_advisor(&mut advisor, faction, loaded.word);
            assert_eq!(
                advisor.take_cockpit_steps(),
                [CockpitStep { kind: 3, step: 1 }],
                "{alliance} {word:#x}"
            );
            let mut queue = movies::MovieQueue::default();
            queue.end(loaded.word, Faction::of_alliance(alliance));
            assert_eq!(queue.next(true), Some(movies::Showing::Ending(movie)));
        }
    }

    #[test]
    fn a_game_saved_before_any_win_loads_without_an_ending() {
        // FUN_00486890 hands the advisor only a word with GO.
        for alliance in [true, false] {
            let loaded = saved_and_loaded(AD, alliance);
            let faction = if alliance {
                AdvisorFaction::Alliance
            } else {
                AdvisorFaction::Empire
            };
            let mut advisor = AdvisorState::new(faction);
            resume_saved_advisor(&mut advisor, faction, loaded.word);
            assert!(advisor.take_cockpit_steps().is_empty());
        }
    }
}

#[cfg(test)]
mod headquarters_tests {
    use super::*;

    /// A galaxy whose headquarters are `hqs`: one held system per side,
    /// marked or not. port: fixture values.
    fn galaxy(alliance_hq: bool, empire_hq: bool) -> (GameWorld, SystemKey, SystemKey) {
        let mut world = GameWorld::default();
        let mut system = |side, hq| {
            world.systems.insert(rebellion_core::world::System {
                is_headquarters: hq,
                control: rebellion_core::world::ControlKind::Controlled(side),
                ..rebellion_core::world::System::default()
            })
        };
        let alliance = system(Faction::Alliance, alliance_hq);
        let empire = system(Faction::Empire, empire_hq);
        (world, alliance, empire)
    }

    #[test]
    fn data_without_a_headquarters_for_either_side_starts_without_panicking() {
        for (alliance_hq, empire_hq) in [(false, true), (true, false), (false, false)] {
            let (world, _, _) = galaxy(alliance_hq, empire_hq);
            let state = std::panic::catch_unwind(|| startup_victory_state(&world));
            assert!(state.is_ok(), "{alliance_hq} {empire_hq}");
        }
    }

    #[test]
    fn a_campaign_on_data_without_an_alliance_headquarters_starts_with_ad() {
        // FUN_005180e0 sets AD when FUN_00506110 finds no HQ object, and it
        // finds Coruscant by its id, reading no Empire HQ: data without
        // either headquarters still starts a campaign.
        use rebellion_core::victory::{
            VictorySystem, AD, CORUSCANT, LUKE, MON_MOTHMA, PALPATINE, VADER,
        };
        for (alliance_hq, empire_hq) in [(false, true), (true, false), (false, false)] {
            let (mut world, _, empire) = galaxy(alliance_hq, empire_hq);
            world.systems[empire].dat_id = rebellion_core::ids::DatId::new(CORUSCANT);
            for id in [LUKE, MON_MOTHMA, VADER, PALPATINE] {
                world.characters.insert(rebellion_core::world::Character {
                    dat_id: rebellion_core::ids::DatId::new(id),
                    ..rebellion_core::world::Character::default()
                });
            }

            let (alliance_key, empire_key) = headquarters(&world);
            let state = VictoryState::new(alliance_key, empire_key);

            let word = VictorySystem::word(&state, &world, VictoryConditions::Standard)
                .expect("every object the word reads exists");
            assert_eq!(word & AD != 0, !alliance_hq, "{alliance_hq} {empire_hq}");
        }
    }

    #[test]
    fn each_side_keeps_the_headquarters_the_seeding_marked() {
        let (world, alliance, empire) = galaxy(true, true);
        let state = startup_victory_state(&world);
        assert_eq!((state.alliance_hq, state.empire_hq), (alliance, empire));
    }
}

#[cfg(test)]
mod evacuation_losses_tests {
    use super::*;
    use std::collections::HashMap;

    /// The shipped TEXTSTRA entries `0x70a8`-`0x70aa`.
    fn templates() -> rebellion_data::text_templates::TextTemplates {
        let entries: HashMap<u16, &str> = [
            (0x70a8, "Evacuation Losses\u{1}"),
            (
                0x70a9,
                "The following units were lost running an enemy blockade of |\u{2}\u{1}\0\0\0.\n\u{1}",
            ),
            (0x70aa, "\n|\u{1}\u{1}\0\0\0\u{1}"),
        ]
        .into_iter()
        .collect();
        rebellion_data::text_templates::TextTemplates::from_json(
            &serde_json::to_vec(&entries).unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn regiments_lost_running_a_blockade_file_one_evacuation_losses_message_per_side() {
        // FUN_00495e90 on event 0x340 (FUN_00504a00): "Evacuation Losses"
        // (0x70a8), the system as 0x70a9's argument 2, a 0x70aa line per
        // regiment (argument 1), code 0x10; picture 0x407 and sound 0x460
        // for side 1, 0x406 and 0x45f for side 2.
        let mut world = GameWorld::default();
        let system = world.systems.insert(rebellion_core::world::System {
            name: "Selonia".into(),
            ..rebellion_core::world::System::default()
        });
        let units = [
            "Alliance Army Regiment".to_owned(),
            "Mon Calamari Regiment".to_owned(),
        ];
        let messages = evacuation_losses_messages(&world, &templates(), system, 9, &units);
        assert_eq!(messages.len(), 2);
        for (message, (alliance, picture, sound)) in messages
            .iter()
            .zip([(true, 0x407, 0x460), (false, 0x406, 0x45f)])
        {
            let display = message.display.as_ref().unwrap();
            assert_eq!(display.title, "Evacuation Losses");
            assert_eq!(message.text, "Evacuation Losses");
            assert_eq!(
                display.body,
                "The following units were lost running an enemy blockade of Selonia.\n\
                 \nAlliance Army Regiment\nMon Calamari Regiment"
            );
            assert_eq!((display.background, display.sound), (picture, sound));
            assert_eq!(message.advice, Some(0x10));
            assert_eq!(message.audience, RailAudience::side(alliance));
            assert_eq!(message.rail, Some(MessageRail::Conflict));
            assert_eq!(message.target, Some(MessageTarget::System(system)));
        }
    }

    #[test]
    fn a_systems_regiments_lost_on_one_tick_share_a_notice_for_either_side() {
        // FUN_00495e90 adds each later regiment to its message (+0x68);
        // another system or tick starts a notice of its own.
        use rebellion_core::blockade::BlockadeEvent;
        use rebellion_core::ids::DatId;
        use rebellion_core::world::{
            BuildableClass, CapitalShipClass, Fleet, ShipInstance, TroopUnit,
        };
        for alliance in [true, false] {
            let mut world = GameWorld::default();
            let mut transport = TroopTransportState::new();
            let mut system = |name: &str| {
                world.systems.insert(rebellion_core::world::System {
                    name: name.into(),
                    ..rebellion_core::world::System::default()
                })
            };
            let (selonia, drall) = (system("Selonia"), system("Drall"));
            let class = world.capital_ship_classes.insert(CapitalShipClass {
                troop_capacity: 4,
                ..CapitalShipClass::default()
            });
            let fleet = world.fleets.insert(Fleet {
                location: selonia,
                capital_ships: vec![ShipInstance::new(class, 100, alliance)],
                is_alliance: alliance,
                ..Fleet::default()
            });
            let mut regiment = |raw: u32, name: &str| {
                world.buildable_classes.insert(
                    DatId::new(raw),
                    BuildableClass {
                        name: name.into(),
                        ..BuildableClass::default()
                    },
                );
                let troop = world.troops.insert(TroopUnit {
                    class_dat_id: DatId::new(raw),
                    is_alliance: alliance,
                });
                world.systems[selonia].ground_units.push(troop);
                troop
            };
            let troops = [
                regiment(0x1000_0001, "Army Regiment"),
                regiment(0x1000_0002, "Fleet Regiment"),
                regiment(0x1000_0003, "Guard Regiment"),
                regiment(0x1000_0004, "Garrison Regiment"),
            ];
            transport
                .embark(&mut world, &ProductRoom::default(), fleet, &troops)
                .unwrap();
            let destroyed = |system, troop, tick| BlockadeEvent::TroopDestroyed {
                system,
                troop,
                tick,
            };
            let events = [
                destroyed(selonia, troops[0], 9),
                destroyed(drall, troops[1], 9),
                destroyed(selonia, troops[2], 9),
                destroyed(selonia, troops[3], 10),
            ];
            let mut log = MessageLog::default();
            show_blockade_events(
                10,
                &events,
                &mut world,
                &mut ManufacturingState::new(),
                &mut transport,
                &MovementState::new(),
                &mut DeliveryState::new(),
                &BlockadeState::new(),
                &templates(),
                &mut log,
            );
            let bodies: Vec<&str> = log
                .messages()
                .iter()
                .map(|message| message.display.as_ref().unwrap().body.as_str())
                .collect();
            let lost = |at: &str, units: &str| {
                format!("The following units were lost running an enemy blockade of {at}.\n{units}")
            };
            let (first, second, third) = (
                lost("Selonia", "\nArmy Regiment\nGuard Regiment"),
                lost("Drall", "\nFleet Regiment"),
                lost("Selonia", "\nGarrison Regiment"),
            );
            assert_eq!(
                bodies,
                [&first, &first, &second, &second, &third, &third].map(String::as_str),
                "{alliance}"
            );
            assert!(world.troops.is_empty());
        }
    }

    #[test]
    fn no_lost_regiment_or_no_original_text_files_nothing() {
        // FUN_00495e90 sets its text only for a regiment and a system.
        let mut world = GameWorld::default();
        let system = world
            .systems
            .insert(rebellion_core::world::System::default());
        assert!(evacuation_losses_messages(&world, &templates(), system, 9, &[]).is_empty());
        let empty = rebellion_data::text_templates::TextTemplates::default();
        assert!(
            evacuation_losses_messages(&world, &empty, system, 9, &["Regiment".to_owned()])
                .is_empty()
        );
    }
}

#[cfg(test)]
mod transport_refusal_tests {
    use super::*;
    use rebellion_core::troop_transport::TroopTransportError as E;

    #[test]
    fn a_refused_regiment_or_squadron_move_hands_the_advisor_the_originals_status() {
        // regiment-unload.md, "Refusals" and "Choosing the container":
        // FUN_004f9860 en route 1/1; FUN_0053d430 another side's
        // destination 1/0x28; FUN_005073d0 destroyed 1/0x22 and blockaded
        // 0x90/4; FUN_00553410 a fleet in hyperspace 1/0x21; a room node
        // with no room 1/0x27 (FUN_00552300, FUN_00500b40).
        for (error, status) in [
            (E::RegimentEnRoute, (1, 1)),
            (E::SquadronEnRoute, (1, 1)),
            (E::OtherSideDestination, (1, 0x28)),
            (E::WrongFaction, (1, 0x28)),
            (E::DestinationDestroyed, (1, 0x22)),
            (E::DestinationBlockaded, (0x90, 4)),
            (E::DestinationInTransit, (1, 0x21)),
            (
                E::CapacityExceeded {
                    capacity: 0,
                    requested: 1,
                },
                (1, 0x27),
            ),
            (E::NoSquadronRoom, (1, 0x27)),
        ] {
            assert_eq!(transport_refusal_status(error), Some(status), "{error:?}");
        }
    }

    #[test]
    fn a_refused_fleet_move_hands_the_advisor_the_originals_status() {
        // move-order.md: FUN_00578d00 an object or destination that does
        // not resolve 1/0x12, 1/0x22; FUN_004fdc70 → FUN_004f9860 another
        // side's or an en-route fleet 1/1; FUN_00555920 → FUN_004fd900 no
        // hyperdrive 1/0x18; FUN_00552210 → FUN_005287f0 a blockaded drag
        // 1/1; FUN_00507ab0 the Death Star room 0x90/3.
        use rebellion_core::movement::FleetDispatchError as D;
        for (error, status) in [
            (D::MissingFleet, (1, 0x12)),
            (D::MissingDestination, (1, 0x22)),
            (D::WrongFaction, (1, 1)),
            (D::AlreadyInTransit, (1, 1)),
            (D::NoHyperdrive, (1, 0x18)),
            (D::Blockaded, (1, 1)),
            (D::DeathStarRoom, (0x90, 3)),
        ] {
            assert_eq!(
                fleet_dispatch_refusal_status(error),
                Some(status),
                "{error:?}"
            );
        }
        // port: the origin and same-system checks are the port's own.
        for error in [D::MissingOrigin, D::AlreadyAtDestination] {
            assert_eq!(fleet_dispatch_refusal_status(error), None, "{error:?}");
        }
    }

    #[test]
    fn a_refused_join_split_or_ship_move_hands_the_advisor_the_originals_status() {
        // fleet-join-split.md: FUN_00578c00 an object that does not resolve
        // 1/0x12; FUN_004f9860 another side's or an en-route mover 1/1;
        // FUN_004f9860.c:45-50 an unfinished ship of a Create Fleet 1/2;
        // FUN_00553aa0 into itself 1/0x26; FUN_00553410 another side's
        // fleet 1/0x24 (its first test), one en route 1/0x21; a departure
        // as a fleet move's.
        use rebellion_core::fleet_join::FleetOrderError as O;
        use rebellion_core::movement::FleetDispatchError as D;
        for (error, status) in [
            (O::MissingFleet, (1, 0x12)),
            (O::MissingShip, (1, 0x12)),
            (O::WrongFaction, (1, 1)),
            (O::InTransit, (1, 1)),
            (O::Unfinished, (1, 2)),
            (O::IntoItself, (1, 0x26)),
            (O::OtherSideDestination, (1, 0x24)),
            (O::DestinationInTransit, (1, 0x21)),
            (O::Dispatch(D::NoHyperdrive), (1, 0x18)),
            (O::Dispatch(D::DeathStarRoom), (0x90, 3)),
        ] {
            assert_eq!(fleet_order_refusal_status(error), Some(status), "{error:?}");
        }
        // port: the roster check of fleet_join::roster is the port's own.
        assert_eq!(fleet_order_refusal_status(O::ShipsChanged), None);
        assert_eq!(
            fleet_order_refusal_status(O::Dispatch(D::MissingOrigin)),
            None
        );
    }

    #[test]
    fn a_stale_selection_the_menu_cannot_offer_gets_no_reaction() {
        // port: the port's own checks of keys gone since the menu opened.
        for error in [
            E::MissingFleet,
            E::MissingSystem,
            E::NoTroops,
            E::MissingTroop,
            E::TroopNotAtFleetSystem,
            E::AlreadyEmbarked,
            E::FleetNotAtDestination,
            E::MissingSquadron,
        ] {
            assert_eq!(transport_refusal_status(error), None, "{error:?}");
        }
    }
}

#[cfg(test)]
mod battle_alert_text_tests {
    use super::*;
    use std::collections::HashMap;

    /// Templates `0x7020`-`0x7025` that print their own id and their three
    /// slots by name, so a test reads which template and argument order the
    /// alert chose. port: fixture text, not the shipped entries.
    fn templates() -> rebellion_data::text_templates::TextTemplates {
        let slots = "|\u{1}\u{1}\0\0\0,|\u{2}\u{1}\0\0\0,|\u{3}\u{1}\0\0\0\u{1}";
        let mut entries: HashMap<u16, String> =
            [(0x7020, "Battle at |\u{1}\u{1}\0\0\0\u{1}".to_owned())].into_iter().collect();
        for id in 0x7021..=0x7025 {
            entries.insert(id, format!("{id:x}:{slots}"));
        }
        rebellion_data::text_templates::TextTemplates::from_json(
            &serde_json::to_vec(&entries).unwrap(),
        )
        .unwrap()
    }

    fn alert(
        holder: Option<Faction>,
        blockaded: bool,
        player_is_alliance: bool,
    ) -> Option<(String, String)> {
        let mut world = GameWorld::default();
        let system = world.systems.insert(rebellion_core::world::System {
            name: "Corellia".into(),
            control: holder.map_or(
                rebellion_core::world::ControlKind::Uncontrolled,
                rebellion_core::world::ControlKind::Controlled,
            ),
            ..rebellion_core::world::System::default()
        });
        battle_alert_text(&world, &templates(), blockaded, system, player_is_alliance)
    }

    #[test]
    fn the_battle_alert_picks_its_summary_from_the_holder_and_the_blockade_for_either_side() {
        // FUN_0049c0d0: title 0x7020; the player's system threatened 0x7021
        // (enemy, system, player) or blockaded 0x7022 (player, system,
        // enemy); a neutral one 0x7025 (player, enemy, system); the enemy's
        // blockaded 0x7024 (enemy, player, system) or entered 0x7023
        // (player, system, enemy).
        for player_is_alliance in [true, false] {
            let (player, enemy) = if player_is_alliance {
                (Faction::Alliance, Faction::Empire)
            } else {
                (Faction::Empire, Faction::Alliance)
            };
            let (p, e) = if player_is_alliance {
                ("Alliance", "Empire")
            } else {
                ("Empire", "Alliance")
            };
            for (holder, blockaded, summary) in [
                (Some(player), false, format!("7021:{e},Corellia,{p}")),
                (Some(player), true, format!("7022:{p},Corellia,{e}")),
                (None, false, format!("7025:{p},{e},Corellia")),
                (None, true, format!("7025:{p},{e},Corellia")),
                (Some(enemy), true, format!("7024:{e},{p},Corellia")),
                (Some(enemy), false, format!("7023:{p},Corellia,{e}")),
            ] {
                assert_eq!(
                    alert(holder, blockaded, player_is_alliance),
                    Some(("Battle at Corellia".to_owned(), summary)),
                    "{player_is_alliance} {holder:?} {blockaded}"
                );
            }
        }
    }

    #[test]
    fn an_unknown_system_or_missing_text_shows_no_alert() {
        let world = GameWorld::default();
        let missing = rebellion_core::ids::SystemKey::default();
        assert_eq!(battle_alert_text(&world, &templates(), false, missing, true), None);
        let mut world = GameWorld::default();
        let system = world.systems.insert(rebellion_core::world::System::default());
        let empty = rebellion_data::text_templates::TextTemplates::default();
        assert_eq!(battle_alert_text(&world, &empty, false, system, true), None);
    }
}
