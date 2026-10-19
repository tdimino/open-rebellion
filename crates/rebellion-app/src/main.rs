mod audio;
#[cfg(not(target_arch = "wasm32"))]
mod dev_actions;
#[cfg(not(target_arch = "wasm32"))]
mod dev_channel;
#[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
mod dev_commands;
#[cfg(not(target_arch = "wasm32"))]
mod dev_input;
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
#[cfg(any(target_arch = "wasm32", test))]
mod runtime_pack;
mod strategic_windowing;
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
use rebellion_core::ai::{AIAction, AIState, AISystem, AiFaction, FleetMoveReason};
use rebellion_core::betrayal::{BetrayalState, BetrayalSystem};
use rebellion_core::blockade::{BlockadeState, BlockadeSystem};
use rebellion_core::bombardment::BombardmentSystem;
use rebellion_core::combat::{side_present, CombatForce, CombatSide};
use rebellion_core::dat::Faction;
use rebellion_core::death_star::{DeathStarState, DeathStarSystem};
use rebellion_core::delivery::DeliveryState;
use rebellion_core::economy::{EconomyEvent, EconomyState, EconomySystem};
use rebellion_core::events::{EventAction, EventState, EventSystem};
use rebellion_core::fleet_join::FleetMover;
use rebellion_core::fog::{FogState, FogSystem};
use rebellion_core::ids::{CharacterKey, FighterKey, FleetKey, SystemKey};
use rebellion_core::jedi::{JediState, JediSystem};
use rebellion_core::manufacturing::{ManufacturingState, ManufacturingSystem, QueueItem};
use rebellion_core::missions::{
    MissionEffect, MissionFaction, MissionKind, MissionMember, MissionState, MissionSystem,
};
use rebellion_core::movement::{
    apply_fleet_arrival, begin_faction_fleet_transit, begin_fleet_transit, fleet_move_confirms,
    fleet_move_enabled, fleet_transit_ticks, fleets_move_enabled, reconcile_fleet_orbits,
    system_side_fleets, validate_fleet_dispatch, validate_fleets_dispatch, MovementState,
    MovementSystem,
};
use rebellion_core::repair::{RepairEvent, RepairState, RepairSystem};
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
    draw_build_selection, BuildSelectionAction, BuildSelectionState, PreparedBuildSelectionConfirm,
};
use rebellion_render::defenses_window::draw_managed_defenses_windows;
use rebellion_render::fleet_finder::{
    draw_fleet_finder_focused, FleetFinderAction, FleetFinderState,
};
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
use rebellion_render::missions_window::{draw_managed_missions_windows, PreparedMissionsWindow};
use rebellion_render::move_confirmation::{
    draw_move_confirmation, draw_scrap_confirmation, MoveConfirmation, MoveConfirmationAction,
    MoveConfirmationState, ScrapConfirmation, ScrapConfirmationAction, ScrapConfirmationState,
};
use rebellion_render::object_menu::{
    draw_object_menu, CommandGates, MenuObject, ObjectMenuCommand, ObjectMenuState, OrderGates,
};
use rebellion_render::panels::bombardment::{draw_bombardment, BombardmentPanelState};
use rebellion_render::panels::death_star::draw_death_star;
use rebellion_render::personnel_finder::{
    draw_personnel_finder, PersonnelFinderAction, PersonnelFinderState,
};
use rebellion_render::quadrant_icons::Quadrant;
use rebellion_render::sector_window::{draw_managed_sector_windows, SectorWindowInputEvent};
use rebellion_render::status_rows::{StatusObject, StatusSources};
use rebellion_render::status_window::{
    draw_managed_status_window, StatusWindowAction, StatusWindowState,
};
use rebellion_render::system_window::{draw_managed_system_windows, fleet_label};
use rebellion_render::tactical_view::{
    TacticalWindowEvent, TacticalWindowInput, TacticalWindowPhase,
};
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
#[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
use rebellion_render::InterfaceCommand;
use rebellion_render::MdataTrack;
use rebellion_render::{
    cockpit_accelerator, draw_advisor, draw_audio_controls, draw_cockpit_background,
    draw_cockpit_chrome, draw_cockpit_egui_layer, draw_credits, draw_event_screen, draw_fleets,
    draw_galaxy_backdrop, draw_galaxy_map, draw_game_options, draw_game_setup, draw_ground_combat,
    draw_main_menu, draw_missions, draw_multiplayer_setup, draw_save_load, draw_tactical_view,
    handle_cockpit_egui_input, set_cockpit_viewport_clip, show_event_screen, update_event_screen,
    update_galaxy_map_input, AdvisorFaction, AdvisorState, AssetRenderProfile, AudioVolumeState,
    BmpCache, CockpitButton, CockpitFaction, CockpitState, CreditsState, EventScreenState,
    FleetsState, GalaxyMapState, GameMessage, GameOptionsAction, GameOptionsOrigin,
    GameOptionsState, GameSetupAction, GameSetupState, GroundAction, GroundCombatState,
    MainMenuAction, MainMenuState, MenuDestinationAction, MessageCategory, MessageDisplay,
    MessageLog, MessageLogState, MessageRail, MessageTarget, MultiplayerSetupAction,
    MultiplayerSetupState, OriginalEncyclopediaCatalog, OriginalEncyclopediaEntry, PanelAction,
    RailAudience, SectorWindowAction, SectorWindowState, SystemWindowAction, SystemWindowState,
    TacticalAction, TacticalState, TacticalTrenchRunOutcome, VideoError, VideoPlayer,
};
use rebellion_render::{DefensesWindowAction, DefensesWindowState};
use rebellion_render::{FleetWindowAction, FleetWindowState};
use rebellion_render::{MissionsWindowAction, MissionsWindowState};
#[cfg(not(target_arch = "wasm32"))]
use rebellion_render::{MusicSelector, SfxKind};
use strategic_windowing::{
    ApplicationInputLocks, CommittedTacticalPhase, LegacyInputBlockers,
    LegacyInputRestrictionLatch, StrategicWindowing, TacticalWindowIdentity, WindowScene,
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
    /// Save-slot picker entered from the main menu.
    LoadGame,
    /// Scrolling original-game and Open Rebellion credits.
    Credits,
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
    /// Ground combat phase after space combat.
    GroundCombat,
    /// Victory/defeat modal overlay on the frozen galaxy map.
    VictoryModal { alliance_won: bool },
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
        GameMode::MainMenu
        | GameMode::LoadGame
        | GameMode::Credits
        | GameMode::MultiplayerSetup
        | GameMode::GameSetup => Some(MusicScene::Menu),
        GameMode::Galaxy
        | GameMode::TacticalCombat
        | GameMode::GroundCombat
        | GameMode::VictoryModal { .. } => Some(MusicScene::Play),
        GameMode::Cutscene { .. } => Some(MusicScene::Movie),
        GameMode::GameOptions { .. } => None,
    }
}

/// Which cutscene is playing — determines post-cutscene transition.
#[derive(Debug, Clone, PartialEq)]
enum CutsceneKind {
    /// Game intro (000.webm) → `MainMenu`.
    Intro,
    /// Death Star trench-run result (201/202.webm) → resume tactical view.
    TrenchRun(TacticalTrenchRunOutcome),
    /// In-game story cutscene (101–108.webm) → resume Galaxy.
    Story(u32),
}

const INTRO_CUTSCENE: &str = "assets/references/ref-videos/000.webm";
const TRENCH_RUN_SUCCESS_CUTSCENE: &str = "assets/references/ref-videos/201.webm";
const TRENCH_RUN_FAILURE_CUTSCENE: &str = "assets/references/ref-videos/202.webm";
/// How many frames a developer command waits for the galaxy view before it
/// is refused (about 10 s at 60 fps). Frames rather than wall time, so a
/// covered window that macOS throttles does not use up the wait between two
/// frames.
#[cfg(not(target_arch = "wasm32"))]
const DEV_GALAXY_WAIT_FRAMES: u32 = 600;

/// Map a story event ID to its cutscene file number (101–108), if any.
/// Returns `None` for events that don't trigger a cutscene.
fn story_event_to_cutscene(event_id: u32) -> Option<u32> {
    match event_id {
        0x221 => Some(101), // Luke departs for Dagobah
        0x210 => Some(102), // Luke completes Dagobah training
        0x212 => Some(103), // Bounty hunters capture Han
        0x383 => Some(104), // Palace rescue success
        0x220 => Some(105), // Luke vs Vader final confrontation
        0x393 => Some(106), // Vader dispatched
        0x396 => Some(107), // Father and son confrontation
        0x397 => Some(108), // Empire dispatches bounty hunters
        _ => None,
    }
}

/// Build the cutscene asset path for a story cutscene number.
fn story_cutscene_path(number: u32) -> String {
    format!("assets/references/ref-videos/{number}.webm")
}

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

fn read_save_slots(saves_dir: &Path) -> Vec<rebellion_render::SaveSlotInfo> {
    let mut slots: Vec<_> = rebellion_data::save::list_saves(saves_dir)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .map(|meta| rebellion_render::SaveSlotInfo {
            slot: meta.slot,
            name: meta.name,
            timestamp: if meta.timestamp_secs == 0 {
                "Browser save".to_string()
            } else {
                let hours = (meta.timestamp_secs / 3600) % 24;
                let minutes = (meta.timestamp_secs / 60) % 60;
                format!("{hours:02}:{minutes:02}")
            },
            game_tick: meta.game_tick,
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
                timestamp: "Load to inspect error".into(),
                game_tick: 0,
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
    death_star: &'a mut DeathStarState,
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
            death_star: self.death_star.clone(),
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
        *self.death_star = state.death_star;
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
    }
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
}

#[derive(Debug, Default)]
struct StrategicRawInputCollector {
    next_ordinal: u32,
    keys: Vec<StrategicIndexedKeyEvent>,
    pointers: Vec<StrategicRawPointerEvent>,
    pressed: BTreeSet<PointerButtonId>,
    pointer: (f32, f32),
}

impl StrategicRawInputCollector {
    fn take(&mut self) -> StrategicRawInputBatch {
        self.next_ordinal = 0;
        StrategicRawInputBatch {
            keys: std::mem::take(&mut self.keys),
            pointers: std::mem::take(&mut self.pointers),
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
        fleet_finder: Option<rebellion_windowing::WindowId>,
        encyclopedia: Option<rebellion_windowing::WindowId>,
        message_index: Option<rebellion_windowing::WindowId>,
        status: Option<rebellion_windowing::WindowId>,
        build_selection: Option<rebellion_windowing::WindowId>,
    ) {
        for event in &self.events {
            let recipient = router.keyboard_recipient(event.id);
            let disposition = legacy_child_key_disposition(
                *event,
                recipient,
                handles_keyboard,
                fleet,
                fleet_finder,
                encyclopedia,
                message_index,
                status,
                build_selection,
            );
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

#[expect(
    clippy::too_many_arguments,
    reason = "One frozen event is matched against each migrated legacy child owner."
)]
fn legacy_child_key_disposition(
    event: StrategicKeyEvent,
    recipient: InputRecipient,
    handles_keyboard: bool,
    fleet: Option<(rebellion_windowing::WindowId, bool)>,
    fleet_finder: Option<rebellion_windowing::WindowId>,
    encyclopedia: Option<rebellion_windowing::WindowId>,
    message_index: Option<rebellion_windowing::WindowId>,
    status: Option<rebellion_windowing::WindowId>,
    build_selection: Option<rebellion_windowing::WindowId>,
) -> Option<InputDisposition> {
    match recipient {
        InputRecipient::Root => Some(if handles_keyboard {
            InputDisposition::Handled
        } else {
            InputDisposition::Unhandled
        }),
        InputRecipient::Window(window) if fleet_finder == Some(window) => {
            Some(if event.key == KeyCode::Escape {
                InputDisposition::Handled
            } else {
                InputDisposition::Unhandled
            })
        }
        InputRecipient::Window(window) if encyclopedia == Some(window) => {
            Some(if handles_keyboard || event.key == KeyCode::Escape {
                InputDisposition::Handled
            } else {
                InputDisposition::Unhandled
            })
        }
        InputRecipient::Window(window) if message_index == Some(window) => {
            Some(if event.key == KeyCode::Escape {
                InputDisposition::Handled
            } else {
                InputDisposition::Unhandled
            })
        }
        InputRecipient::Window(window) if status == Some(window) => {
            Some(if event.key == KeyCode::Escape {
                InputDisposition::Handled
            } else {
                InputDisposition::Unhandled
            })
        }
        InputRecipient::Window(window) if build_selection == Some(window) => Some(
            if matches!(
                event.key,
                KeyCode::Escape
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
            ) {
                InputDisposition::Handled
            } else {
                InputDisposition::Unhandled
            },
        ),
        InputRecipient::Window(window) => fleet
            .filter(|(fleet_window, _)| *fleet_window == window)
            .map(|(_, renaming)| {
                if renaming && rename_edit_owns_key(event) {
                    InputDisposition::Handled
                } else {
                    InputDisposition::Unhandled
                }
            }),
        InputRecipient::Blocked => None,
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

    fn sector_events(&self) -> Vec<SectorWindowInputEvent> {
        self.events
            .iter()
            .map(|event| SectorWindowInputEvent {
                id: event.id,
                point: event.point,
                phase: event.phase,
            })
            .collect()
    }

    fn has_blocked_pointer_input(&self, router: &FrameInputRouter) -> bool {
        self.events
            .iter()
            .any(|event| router.pointer_recipient(event.id) == InputRecipient::Blocked)
    }

    fn blocks_pointer_phase(&self, router: &FrameInputRouter, phase: PointerPhase) -> bool {
        self.events.iter().any(|event| {
            event.phase == phase && router.pointer_recipient(event.id) == InputRecipient::Blocked
        })
    }

    fn blocks_map_pointer_phase(&self, router: &FrameInputRouter, phase: PointerPhase) -> bool {
        self.events.iter().any(|event| {
            event.phase == phase && router.pointer_recipient(event.id) != InputRecipient::Root
        })
    }
}

fn latch_map_pointer_from_strategic_batch(
    map: &mut GalaxyMapState,
    batch: &StrategicPointerBatch,
    router: &FrameInputRouter,
) {
    map.pointer_blocked |=
        batch.blocks_map_pointer_phase(router, PointerPhase::Press(PointerButtonId::Primary));
}

fn route_tactical_window_input(
    batch: &StrategicPointerBatch,
    router: &mut FrameInputRouter,
    window: rebellion_windowing::WindowId,
    pointer_screen: (f32, f32),
    primary_down: bool,
    wheel_y: f32,
) -> TacticalWindowInput {
    let owner = InputRecipient::Window(window);
    let mut input = TacticalWindowInput {
        pointer_screen,
        primary_down,
        ..TacticalWindowInput::default()
    };
    for event in &batch.events {
        let recipient = router.pointer_recipient(event.id);
        if recipient == InputRecipient::Blocked
            && event.phase == PointerPhase::Release(PointerButtonId::Primary)
        {
            input.events.push(TacticalWindowEvent::CancelGesture);
        }
        if recipient != owner || !router.claim(event.id, owner) {
            continue;
        }
        match event.phase {
            PointerPhase::Press(PointerButtonId::Primary) => {
                input
                    .events
                    .push(TacticalWindowEvent::PrimaryPress(event.point));
            }
            PointerPhase::Release(PointerButtonId::Primary) => {
                input
                    .events
                    .push(TacticalWindowEvent::PrimaryRelease(event.point));
            }
            PointerPhase::Auxiliary => input.events.push(TacticalWindowEvent::Wheel {
                screen: event.point,
                delta_y: wheel_y,
            }),
            PointerPhase::Press(_)
            | PointerPhase::Motion(_)
            | PointerPhase::Release(_)
            | PointerPhase::Hover => {}
        }
    }
    input
}

/// A tactical modal command captured under one frozen manager identity and
/// held until the next frame commits that identity's exact close receipt.
#[derive(Debug, Clone, PartialEq)]
struct PreparedTacticalWindowAction {
    action: TacticalAction,
    battle_session: u64,
    window: TacticalWindowIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TacticalResultDestination {
    System(SystemKey),
    Fleet(FleetKey),
}

fn tactical_result_destination(action: &TacticalAction) -> Option<TacticalResultDestination> {
    match action {
        TacticalAction::OpenBattleSystem(system) => {
            Some(TacticalResultDestination::System(*system))
        }
        TacticalAction::OpenBattleFleet(fleet) => Some(TacticalResultDestination::Fleet(*fleet)),
        _ => None,
    }
}

fn take_tactical_destination_for_galaxy(
    pending: &mut Option<TacticalResultDestination>,
    galaxy_active: bool,
) -> Option<TacticalResultDestination> {
    galaxy_active.then(|| pending.take()).flatten()
}

fn reset_pending_tactical_handoff(
    action: &mut Option<PreparedTacticalWindowAction>,
    destination: &mut Option<TacticalResultDestination>,
) {
    *action = None;
    *destination = None;
}

impl PreparedTacticalWindowAction {
    fn phases(&self) -> Option<(TacticalWindowPhase, TacticalWindowPhase)> {
        match (self.window.kind, &self.action) {
            (
                rebellion_windowing::WindowKind::BattleAlert,
                TacticalAction::BeginCombat | TacticalAction::RetreatBeforeBattle,
            ) => Some((TacticalWindowPhase::Alert, TacticalWindowPhase::Combat)),
            (rebellion_windowing::WindowKind::BattleAlert, TacticalAction::AutoResolve) => {
                Some((TacticalWindowPhase::Alert, TacticalWindowPhase::Results))
            }
            (
                rebellion_windowing::WindowKind::BattleResults,
                TacticalAction::ReturnToGalaxy
                | TacticalAction::OpenBattleSystem(_)
                | TacticalAction::OpenBattleFleet(_),
            ) => Some((TacticalWindowPhase::Results, TacticalWindowPhase::Inactive)),
            _ => None,
        }
    }
}

fn prepare_tactical_window_action(
    action: TacticalAction,
    battle_session: Option<u64>,
    window: Option<TacticalWindowIdentity>,
) -> Option<PreparedTacticalWindowAction> {
    let battle_session = battle_session?;
    let prepared = PreparedTacticalWindowAction {
        action,
        battle_session,
        window: window?,
    };
    prepared.phases().is_some().then_some(prepared)
}

/// Releases a deferred tactical effect only after the manager proves it closed
/// the same live identity at the requested phase boundary.
fn approve_tactical_window_action(
    prepared: PreparedTacticalWindowAction,
    live_battle_session: Option<u64>,
    live_phase: TacticalWindowPhase,
    committed: &CommittedTacticalPhase,
) -> Option<TacticalAction> {
    let (expected_live_phase, next_phase) = prepared.phases()?;
    let expected_reason = if next_phase == TacticalWindowPhase::Results {
        rebellion_windowing::CloseReason::Replaced
    } else {
        rebellion_windowing::CloseReason::Requested
    };
    let expected_key =
        rebellion_windowing::WindowKey::contextual(prepared.window.kind, prepared.window.subject);
    let exact_close_count = committed
        .events
        .iter()
        .filter(|event| {
            matches!(
                event,
                rebellion_windowing::WindowEvent::Closed {
                    window,
                    key,
                    reason,
                } if *window == prepared.window.id
                    && *key == expected_key
                    && *reason == expected_reason
            )
        })
        .count();
    let next_window_matches = match next_phase {
        TacticalWindowPhase::Results => committed.window.is_some_and(|window| {
            window.id != prepared.window.id
                && window.subject == prepared.window.subject
                && window.kind == rebellion_windowing::WindowKind::BattleResults
        }),
        TacticalWindowPhase::Combat | TacticalWindowPhase::Inactive => committed.window.is_none(),
        TacticalWindowPhase::Alert => false,
    };
    (live_battle_session == Some(prepared.battle_session)
        && live_phase == expected_live_phase
        && next_window_matches
        && committed.removed_content.as_slice() == [prepared.window.id]
        && exact_close_count == 1)
        .then_some(prepared.action)
}

fn take_approved_tactical_window_action(
    pending: &mut Option<PreparedTacticalWindowAction>,
    live_battle_session: Option<u64>,
    live_phase: TacticalWindowPhase,
    committed: &CommittedTacticalPhase,
) -> Option<TacticalAction> {
    let action = approve_tactical_window_action(
        pending.as_ref()?.clone(),
        live_battle_session,
        live_phase,
        committed,
    )?;
    pending.take();
    Some(action)
}

#[derive(Debug)]
struct CommittedTacticalWindowAction {
    phase: CommittedTacticalPhase,
    approved_action: Option<TacticalAction>,
}

/// Production tactical handoff seam: atomically commits the phase transition
/// and releases the deferred effect only from that same commit's receipts.
fn commit_pending_tactical_window_action(
    coordinator: &mut StrategicWindowing,
    pending: &mut Option<PreparedTacticalWindowAction>,
    live_battle_session: Option<u64>,
    live_phase: TacticalWindowPhase,
) -> Result<Option<CommittedTacticalWindowAction>, rebellion_windowing::WindowError> {
    let Some(prepared) = pending.as_ref().cloned() else {
        return Ok(None);
    };
    let Some((_, next_phase)) = prepared.phases() else {
        return Ok(None);
    };
    let Some(phase) = coordinator.commit_tactical_transition_if_exact(
        prepared.battle_session,
        prepared.window,
        next_phase,
    )?
    else {
        return Ok(None);
    };
    let approved_action =
        take_approved_tactical_window_action(pending, live_battle_session, live_phase, &phase);
    Ok(Some(CommittedTacticalWindowAction {
        phase,
        approved_action,
    }))
}

/// Collects the app's normalized strategic pointer batch and folds the
/// manager-owned Message Index surface into the same hit set as the retained
/// strategic windows. Keeping this seam shared by production and tests makes
/// the modal's outside-click block observable before galaxy-map dispatch.
fn collect_strategic_pointer_batch(
    frame_number: u64,
    router: &mut FrameInputRouter,
    events: &[StrategicRawPointerEvent],
    frame: &rebellion_windowing::RoutingSnapshot,
    layout: rebellion_render::CockpitLayout,
    message_index: &MessageIndexState,
    status: &StatusWindowState,
    build_selection: &BuildSelectionState,
    retained_hits: impl Fn((f32, f32)) -> BTreeSet<rebellion_windowing::WindowId>,
) -> StrategicPointerBatch {
    StrategicPointerBatch::collect_from_raw(frame_number, router, events, |point| {
        let mut hits = retained_hits(point);
        if let Some(window) = message_index.hit_window(frame, layout, point) {
            hits.insert(window);
        }
        if let Some(window) = status.hit_window(frame, layout, point) {
            hits.insert(window);
        }
        if let Some(window) = build_selection.hit_window(frame, layout, point) {
            hits.insert(window);
        }
        hits
    })
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
            | KeyCode::R
            | KeyCode::P
            | KeyCode::G
            | KeyCode::U
            | KeyCode::A
            | KeyCode::O
            | KeyCode::F
            | KeyCode::N
            | KeyCode::T
            | KeyCode::J
            | KeyCode::B
            | KeyCode::D
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

#[derive(Default)]
struct RetainedLegacyInputFilter {
    context: egui_macroquad::egui::Context,
}

impl RetainedLegacyInputFilter {
    fn reset(&mut self) {
        self.context = egui_macroquad::egui::Context::default();
    }

    fn install(
        &mut self,
        target: &egui_macroquad::egui::Context,
        batch: &StrategicPointerBatch,
        router: &FrameInputRouter,
        suppress_all_input: bool,
    ) -> Option<egui_macroquad::egui::InputState> {
        let saved = target.input(Clone::clone);
        let has_blocked = batch.has_blocked_pointer_input(router);
        let mut raw = saved.raw.clone();
        if suppress_all_input {
            self.reset();
            raw.events.clear();
            raw.modifiers = egui_macroquad::egui::Modifiers::NONE;
            raw.focused = false;
        } else if has_blocked {
            let mut next_route = 0;
            raw.events.retain(|event| {
                if !is_routed_egui_pointer_event(event) {
                    return true;
                }
                let Some(routed) = batch.events.get(next_route) else {
                    return false;
                };
                next_route += 1;
                egui_event_matches_phase(event, routed.phase)
                    && router.pointer_recipient(routed.id) != InputRecipient::Blocked
            });
        }

        let options = target.options(Clone::clone);
        self.context
            .options_mut(|filtered_options| *filtered_options = options);
        let mut filtered = None;
        let _ = self.context.run(raw, |ctx| {
            filtered = Some(ctx.input(Clone::clone));
        });
        let filtered = filtered.expect("the retained input filter runs one egui pass");
        target.input_mut(|input| *input = filtered);
        Some(saved)
    }
}

const fn is_routed_egui_pointer_event(event: &egui_macroquad::egui::Event) -> bool {
    matches!(
        event,
        egui_macroquad::egui::Event::PointerMoved(_)
            | egui_macroquad::egui::Event::PointerButton { .. }
            | egui_macroquad::egui::Event::MouseWheel { .. }
    )
}

fn egui_event_matches_phase(event: &egui_macroquad::egui::Event, phase: PointerPhase) -> bool {
    use egui_macroquad::egui::{Event, PointerButton};

    let button = |button| match button {
        PointerButton::Primary => PointerButtonId::Primary,
        PointerButton::Secondary => PointerButtonId::Secondary,
        PointerButton::Middle => PointerButtonId::Middle,
        PointerButton::Extra1 => PointerButtonId::Extra(1),
        PointerButton::Extra2 => PointerButtonId::Extra(2),
    };
    match (event, phase) {
        (Event::PointerMoved(_), PointerPhase::Motion(_) | PointerPhase::Hover)
        | (Event::MouseWheel { .. }, PointerPhase::Auxiliary) => true,
        (
            Event::PointerButton {
                button: event_button,
                pressed,
                ..
            },
            PointerPhase::Press(phase_button),
        ) => *pressed && button(*event_button) == phase_button,
        (
            Event::PointerButton {
                button: event_button,
                pressed,
                ..
            },
            PointerPhase::Release(phase_button),
        ) => !*pressed && button(*event_button) == phase_button,
        _ => false,
    }
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

/// Manager-owned strategic surfaces in the exact immutable paint sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ManagedStrategicSurface {
    Encyclopedia(rebellion_windowing::WindowId),
    FleetFinder(rebellion_windowing::WindowId),
    MessageIndex(rebellion_windowing::WindowId),
    Status(rebellion_windowing::WindowId),
    BuildSelection(rebellion_windowing::WindowId),
}

fn managed_strategic_surface_plan(
    frame: &rebellion_windowing::RoutingSnapshot,
    encyclopedia: Option<rebellion_windowing::WindowId>,
    fleet_finder: Option<rebellion_windowing::WindowId>,
    message_index: Option<rebellion_windowing::WindowId>,
    status: Option<rebellion_windowing::WindowId>,
    build_selection: Option<rebellion_windowing::WindowId>,
) -> Vec<(ManagedStrategicSurface, bool)> {
    frame
        .paint_order()
        .iter()
        .filter_map(|id| {
            let surface = if Some(*id) == encyclopedia {
                ManagedStrategicSurface::Encyclopedia(*id)
            } else if Some(*id) == fleet_finder {
                ManagedStrategicSurface::FleetFinder(*id)
            } else if Some(*id) == message_index {
                ManagedStrategicSurface::MessageIndex(*id)
            } else if Some(*id) == status {
                ManagedStrategicSurface::Status(*id)
            } else if Some(*id) == build_selection {
                ManagedStrategicSurface::BuildSelection(*id)
            } else {
                return None;
            };
            Some((surface, frame.allows_input(*id)))
        })
        .collect()
}

/// Stable evidence captured when an indexed fleet object opens its context
/// menu. Status preflight rechecks it when the player eventually chooses the
/// command, so vector compaction cannot make the old row name a new object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IndexedStatusMenuWitness {
    Ship {
        fleet: FleetKey,
        index: usize,
        roster: u64,
        class: rebellion_core::ids::CapitalShipKey,
        tag: u32,
    },
    Fighter {
        fleet: FleetKey,
        index: usize,
        class: FighterKey,
        carrier: u32,
    },
}

fn capture_indexed_status_menu_witness(
    world: &GameWorld,
    selection: Option<MenuObject>,
) -> Option<IndexedStatusMenuWitness> {
    match selection? {
        MenuObject::Ship {
            fleet,
            index,
            roster,
        } => {
            (rebellion_core::fleet_join::roster(world, fleet) == Some(roster)).then_some(())?;
            let ship = world.fleets.get(fleet)?.capital_ships.get(index)?;
            ship.alive.then_some(IndexedStatusMenuWitness::Ship {
                fleet,
                index,
                roster,
                class: ship.class,
                tag: ship.tag,
            })
        }
        MenuObject::Fighter { fleet, index } => {
            let fighter = world.fleets.get(fleet)?.fighters.get(index)?;
            (fighter.count > 0).then_some(IndexedStatusMenuWitness::Fighter {
                fleet,
                index,
                class: fighter.class,
                carrier: fighter.carrier,
            })
        }
        _ => None,
    }
}

fn status_object_if_menu_current(
    world: &GameWorld,
    selection: MenuObject,
    witness: Option<IndexedStatusMenuWitness>,
) -> Option<StatusObject> {
    match (selection, witness) {
        (
            MenuObject::Ship {
                fleet,
                index,
                roster,
            },
            Some(IndexedStatusMenuWitness::Ship {
                fleet: witnessed_fleet,
                index: witnessed_index,
                roster: witnessed_roster,
                class,
                tag,
            }),
        ) if fleet == witnessed_fleet
            && index == witnessed_index
            && roster == witnessed_roster
            && rebellion_core::fleet_join::roster(world, fleet) == Some(roster) =>
        {
            let ship = world.fleets.get(fleet)?.capital_ships.get(index)?;
            (ship.alive && ship.class == class && ship.tag == tag)
                .then_some(StatusObject::Ship { fleet, index })
        }
        (
            MenuObject::Fighter { fleet, index },
            Some(IndexedStatusMenuWitness::Fighter {
                fleet: witnessed_fleet,
                index: witnessed_index,
                class,
                carrier,
            }),
        ) if fleet == witnessed_fleet && index == witnessed_index => {
            let fighter = world.fleets.get(fleet)?.fighters.get(index)?;
            (fighter.count > 0 && fighter.class == class && fighter.carrier == carrier)
                .then_some(StatusObject::Fighter { fleet, index })
        }
        (MenuObject::Ship { .. } | MenuObject::Fighter { .. }, _) => None,
        (selection, _) => selection.status_object(),
    }
}

fn apply_capture_cancellation_events(
    events: &[rebellion_windowing::WindowEvent],
    router: &mut FrameInputRouter,
    sectors: &mut SectorWindowState,
    fleets: &mut FleetWindowState,
    defenses: &mut DefensesWindowState,
) {
    for event in events {
        if let rebellion_windowing::WindowEvent::CaptureCancelled { window, .. } = event {
            macroquad::logging::info!("[windowing] {:?}", event);
            router.cancel_window(*window);
            sectors.cancel_pointer_capture(*window);
            fleets.cancel_pointer_capture(*window);
            defenses.cancel_pointer_capture(*window);
        }
    }
}

fn enqueue_application_hold_capture_cancellation(
    coordinator: &mut StrategicWindowing,
    application_held: bool,
    sector_drag_owner: Option<rebellion_windowing::WindowId>,
    fleet_drag_owner: Option<rebellion_windowing::WindowId>,
    defenses_drag_owner: Option<rebellion_windowing::WindowId>,
) {
    if application_held
        && (sector_drag_owner.is_some()
            || fleet_drag_owner.is_some()
            || defenses_drag_owner.is_some())
    {
        coordinator.enqueue(rebellion_windowing::WindowCommand::CancelCapture(
            rebellion_windowing::CaptureCancelReason::HoldStarted,
        ));
    }
}

/// Publishes prepared Sector content only after the exact manager receipt and
/// releases its staged dependent intent only after the renderer and app both
/// accept that identity.
fn commit_sector_window_publication(
    coordinator: &mut StrategicWindowing,
    state: &mut SectorWindowState,
    pending: &mut Vec<PendingSectorPublication>,
    prepared: Option<rebellion_render::sector_window::PreparedSectorWindow>,
    expected_id: Option<rebellion_windowing::WindowId>,
    result: &Result<rebellion_windowing::WindowCommit, rebellion_windowing::WindowError>,
) -> (
    Option<SectorDependentIntent>,
    Vec<rebellion_windowing::WindowId>,
) {
    let pending = prepared.and_then(|prepared| {
        pending
            .iter()
            .position(|candidate| candidate.prepared == Some(prepared))
            .map(|index| pending.remove(index))
    });
    let Some(prepared) = prepared else {
        return (None, Vec::new());
    };
    let Ok(commit) = result else {
        return (None, Vec::new());
    };
    let Some(pending) = pending else {
        let removed = match commit.transition {
            rebellion_windowing::WindowTransition::Opened { id }
            | rebellion_windowing::WindowTransition::Replaced { new: id, .. } => {
                coordinator.rollback_sector_publication(id)
            }
            _ => Vec::new(),
        };
        return (None, removed);
    };
    let receipt_id = match commit.transition {
        rebellion_windowing::WindowTransition::Opened { id }
        | rebellion_windowing::WindowTransition::Existing { id, .. }
        | rebellion_windowing::WindowTransition::Replaced { new: id, .. } => Some(id),
        _ => None,
    };
    let Some(id) = expected_id.filter(|id| receipt_id == Some(*id)) else {
        macroquad::logging::error!(
            "[windowing] Sector content receipt mismatch expected={:?} actual={:?}",
            expected_id,
            receipt_id
        );
        let removed = match commit.transition {
            rebellion_windowing::WindowTransition::Opened { id }
            | rebellion_windowing::WindowTransition::Replaced { new: id, .. } => {
                coordinator.rollback_sector_publication(id)
            }
            _ => Vec::new(),
        };
        return (None, removed);
    };
    let snapshot = coordinator.begin_frame();
    if state.commit_open(prepared, &commit.transition, &snapshot) != Some(id) {
        macroquad::logging::error!(
            "[windowing] Sector content commit did not match identity={:?}",
            id
        );
        let removed = if matches!(
            commit.transition,
            rebellion_windowing::WindowTransition::Opened { .. }
                | rebellion_windowing::WindowTransition::Replaced { .. }
        ) {
            coordinator.rollback_sector_publication(id)
        } else {
            Vec::new()
        };
        return (None, removed);
    }
    let committed = coordinator.has_content(id) || coordinator.acknowledge_sector_publication(id);
    if !committed {
        state.remove_content(&[id]);
        return (None, coordinator.rollback_sector_publication(id));
    }
    (pending.dependent, Vec::new())
}

/// Routes manager-owned Sector actions through the coordinator and leaves
/// gameplay/detail actions for the caller.
fn enqueue_sector_manager_action(
    coordinator: &mut StrategicWindowing,
    action: SectorWindowAction,
) -> bool {
    let command = match action {
        SectorWindowAction::Activate(window) => {
            rebellion_windowing::WindowCommand::Activate(window)
        }
        SectorWindowAction::CapturePointer(window) => {
            rebellion_windowing::WindowCommand::CapturePointer(window)
        }
        SectorWindowAction::ReleasePointer(window) => {
            rebellion_windowing::WindowCommand::ReleasePointer(window)
        }
        SectorWindowAction::Close(window) => rebellion_windowing::WindowCommand::Close(window),
        _ => return false,
    };
    coordinator.enqueue(command);
    true
}

/// Publishes one Build Selection order only after the exact manager teardown
/// receipt and a fresh source validation consume its prepared confirmation.
fn commit_build_selection_gameplay(
    prepared: Option<PreparedBuildSelectionConfirm>,
    removed_content: &[rebellion_windowing::WindowId],
    world: &GameWorld,
    state: &mut BuildSelectionState,
    manufacturing: &mut ManufacturingState,
) -> Option<BuildSelectionAction> {
    let committed = state.commit_confirm(prepared?, removed_content, world)?;
    let (action, items) = committed.into_order();
    let BuildSelectionAction::Confirm { system, .. } = action else {
        return None;
    };
    manufacturing.build_units(system, items);
    Some(action)
}

#[cfg(test)]
mod frame_input_tests {
    #[cfg(feature = "interface-test-fixtures")]
    use super::{apply_capture_cancellation_events, enqueue_application_hold_capture_cancellation};
    use super::{
        approve_tactical_window_action, collect_strategic_pointer_batch,
        commit_pending_tactical_window_action, commit_sector_window_publication,
        enqueue_sector_manager_action, enqueue_sector_window, pump_pending_sector_publication,
        refresh_pending_sector_publication, reset_pending_tactical_handoff, restore_egui_input,
        route_tactical_window_input, tactical_result_destination,
        take_approved_tactical_window_action, take_tactical_destination_for_galaxy,
        with_retained_input, CommittedTacticalWindowAction, PendingSectorPublication,
        PreparedTacticalWindowAction, RetainedLegacyInputFilter, SectorDependentIntent,
        StrategicPointerBatch, StrategicRawPointerEvent, TacticalResultDestination,
    };
    use egui_macroquad::egui;
    use rebellion_render::build_selection::BuildSelectionState;
    #[cfg(feature = "interface-test-fixtures")]
    use rebellion_render::defenses_window::DefensesWindowState;
    use rebellion_render::fleet_finder::FleetFinderState;
    #[cfg(feature = "interface-test-fixtures")]
    use rebellion_render::fleet_window::FleetWindowState;
    use rebellion_render::message_index::MessageIndexState;
    use rebellion_render::sector_window::SectorWindowState;
    use rebellion_render::status_window::StatusWindowState;
    use rebellion_render::tactical_view::TacticalWindowEvent;
    use rebellion_render::windowing_input::{
        FrameInputRouter, InputEventId, InputRecipient, PointerButtonId, PointerPhase,
    };
    use rebellion_render::{CockpitFaction, CockpitLayout, CockpitViewport};
    use rebellion_windowing::{WindowCommand, WindowManager};
    use std::collections::BTreeSet;

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

    fn sector_fixture_world() -> (
        rebellion_core::world::GameWorld,
        rebellion_core::ids::SystemKey,
    ) {
        use rebellion_core::dat::{ExplorationStatus, SectorGroup};
        use rebellion_core::ids::DatId;
        use rebellion_core::world::{ControlKind, GameWorld, Sector, System};

        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(0x5000_0001),
            name: "Corellian".into(),
            group: SectorGroup::Core,
            x: 200,
            y: 200,
            systems: Vec::new(),
        });
        let system = world.systems.insert(System {
            dat_id: DatId::new(0x5000_0002),
            name: "Corellia".into(),
            sector,
            x: 210,
            y: 210,
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
            fighters: Vec::new(),
            defense_facilities: Vec::new(),
            manufacturing_facilities: Vec::new(),
            production_facilities: Vec::new(),
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Uncontrolled,
        });
        world.sectors[sector].systems.push(system);
        (world, system)
    }

    fn add_sector_fixture(
        world: &mut rebellion_core::world::GameWorld,
        dat: u32,
        x: u16,
    ) -> rebellion_core::ids::SystemKey {
        use rebellion_core::dat::SectorGroup;
        use rebellion_core::ids::DatId;
        use rebellion_core::world::{Sector, System};

        let template = world.systems.values().next().unwrap().clone();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(dat),
            name: format!("Sector {dat}"),
            group: SectorGroup::Core,
            x,
            y: 200,
            systems: Vec::new(),
        });
        let system = world.systems.insert(System {
            dat_id: DatId::new(dat + 100),
            name: format!("System {dat}"),
            sector,
            x: x.saturating_add(10),
            y: 210,
            ..template
        });
        world.sectors[sector].systems.push(system);
        system
    }

    #[test]
    fn sector_dependent_intent_releases_only_after_exact_publication() {
        let (world, system) = sector_fixture_world();
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let mut sectors = SectorWindowState::default();
        let mut pending = Vec::<PendingSectorPublication>::new();
        assert!(enqueue_sector_window(
            &mut coordinator,
            &sectors,
            &world,
            system,
            CockpitFaction::Alliance,
            &mut pending,
            Some(SectorDependentIntent::Defenses { system }),
        ));
        assert_eq!(sectors.window_count(), 0);
        assert_eq!(pending.len(), 1);

        let mut applied = coordinator.apply_pending().remove(0);
        let (dependent, removed) = commit_sector_window_publication(
            &mut coordinator,
            &mut sectors,
            &mut pending,
            applied.prepared_sector.take(),
            applied.sector_content_id,
            &applied.result,
        );
        assert!(removed.is_empty());
        assert!(matches!(
            dependent,
            Some(SectorDependentIntent::Defenses {
                system: committed_system,
            }) if committed_system == system
        ));
        assert_eq!(sectors.window_count(), 1);
        assert!(pending.is_empty());
    }

    #[test]
    fn sector_dependent_intent_is_discarded_on_wrong_publication_receipt() {
        let (world, system) = sector_fixture_world();
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let mut sectors = SectorWindowState::default();
        let mut pending = Vec::<PendingSectorPublication>::new();
        assert!(enqueue_sector_window(
            &mut coordinator,
            &sectors,
            &world,
            system,
            CockpitFaction::Alliance,
            &mut pending,
            Some(SectorDependentIntent::Defenses { system }),
        ));
        let mut applied = coordinator.apply_pending().remove(0);
        let opened = applied.sector_content_id.unwrap();
        let (dependent, removed) = commit_sector_window_publication(
            &mut coordinator,
            &mut sectors,
            &mut pending,
            applied.prepared_sector.take(),
            None,
            &applied.result,
        );
        assert!(dependent.is_none());
        assert_eq!(removed, vec![opened]);
        assert!(pending.is_empty());
        assert_eq!(sectors.window_count(), 0);
        assert!(coordinator.begin_frame().key(opened).is_none());
    }

    #[test]
    fn sector_open_receipt_without_matching_pending_intent_is_rolled_back() {
        let (world, system) = sector_fixture_world();
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let mut sectors = SectorWindowState::default();
        let mut pending = Vec::<PendingSectorPublication>::new();
        assert!(enqueue_sector_window(
            &mut coordinator,
            &sectors,
            &world,
            system,
            CockpitFaction::Alliance,
            &mut pending,
            Some(SectorDependentIntent::Defenses { system }),
        ));
        let mut applied = coordinator.apply_pending().remove(0);
        let opened = applied.sector_content_id.unwrap();
        pending.clear();

        let (dependent, removed) = commit_sector_window_publication(
            &mut coordinator,
            &mut sectors,
            &mut pending,
            applied.prepared_sector.take(),
            Some(opened),
            &applied.result,
        );
        assert!(dependent.is_none());
        assert_eq!(removed, vec![opened]);
        assert!(coordinator.begin_frame().key(opened).is_none());
        assert_eq!(sectors.window_count(), 0);
    }

    #[test]
    fn same_frame_sector_requests_prepare_serially_and_never_publish_three() {
        let (mut world, first) = sector_fixture_world();
        let second = add_sector_fixture(&mut world, 0x5000_0010, 800);
        let third = add_sector_fixture(&mut world, 0x5000_0020, 120);
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let mut sectors = SectorWindowState::default();
        let mut pending = Vec::<PendingSectorPublication>::new();

        for system in [first, second, third] {
            assert!(enqueue_sector_window(
                &mut coordinator,
                &sectors,
                &world,
                system,
                CockpitFaction::Alliance,
                &mut pending,
                None,
            ));
        }
        assert_eq!(pending.len(), 3);
        assert_eq!(
            pending
                .iter()
                .filter(|candidate| candidate.prepared.is_some())
                .count(),
            1
        );

        for expected_remaining in [2, 1, 0] {
            let mut applied = coordinator.apply_pending();
            assert_eq!(applied.len(), 1, "only one Sector mutation is in flight");
            let mut applied = applied.remove(0);
            let (_, removed) = commit_sector_window_publication(
                &mut coordinator,
                &mut sectors,
                &mut pending,
                applied.prepared_sector.take(),
                applied.sector_content_id,
                &applied.result,
            );
            assert!(removed.is_empty());
            assert!(sectors.window_count() <= 2);
            assert!(coordinator.begin_frame().paint_order().len() <= 2);
            assert_eq!(pending.len(), expected_remaining);
            pump_pending_sector_publication(&mut coordinator, &sectors, &world, &mut pending);
        }
        assert_eq!(sectors.window_count(), 2);
    }

    #[cfg(feature = "interface-test-fixtures")]
    #[test]
    fn queued_sector_replacement_refreshes_after_interactive_side_switch() {
        let (mut world, first) = sector_fixture_world();
        let second = add_sector_fixture(&mut world, 0x5000_0010, 800);
        let third = add_sector_fixture(&mut world, 0x5000_0020, 120);
        let fourth = add_sector_fixture(&mut world, 0x5000_0030, 900);
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let mut sectors = SectorWindowState::default();
        let mut pending = Vec::<PendingSectorPublication>::new();

        for system in [first, second] {
            assert!(enqueue_sector_window(
                &mut coordinator,
                &sectors,
                &world,
                system,
                CockpitFaction::Alliance,
                &mut pending,
                None,
            ));
            let mut applied = coordinator.apply_pending().remove(0);
            let (_, removed) = commit_sector_window_publication(
                &mut coordinator,
                &mut sectors,
                &mut pending,
                applied.prepared_sector.take(),
                applied.sector_content_id,
                &applied.result,
            );
            assert!(removed.is_empty());
        }
        let first_id = sectors.window_id(world.systems[first].sector).unwrap();
        let second_id = sectors.window_id(world.systems[second].sector).unwrap();
        for system in [third, fourth] {
            assert!(enqueue_sector_window(
                &mut coordinator,
                &sectors,
                &world,
                system,
                CockpitFaction::Alliance,
                &mut pending,
                None,
            ));
        }
        let mut applied = coordinator.apply_pending().remove(0);
        let (_, removed) = commit_sector_window_publication(
            &mut coordinator,
            &mut sectors,
            &mut pending,
            applied.prepared_sector.take(),
            applied.sector_content_id,
            &applied.result,
        );
        assert!(removed.is_empty());
        pump_pending_sector_publication(&mut coordinator, &sectors, &world, &mut pending);
        assert_eq!(pending[0].prepared.unwrap().replacing(), Some(second_id));

        // The next interactive frame switches the surviving Sector's column
        // after fourth has been prepared but before its manager Replace.
        sectors.switch_managed_side_for_test(world.systems[third].sector);
        refresh_pending_sector_publication(&mut coordinator, &sectors, &world, &mut pending);
        let mut applied = coordinator.apply_pending().remove(0);
        let (_, removed) = commit_sector_window_publication(
            &mut coordinator,
            &mut sectors,
            &mut pending,
            applied.prepared_sector.take(),
            applied.sector_content_id,
            &applied.result,
        );
        sectors.remove_content(&removed);
        assert_eq!(sectors.window_count(), 2);
        assert_eq!(
            sectors.window_id(world.systems[second].sector),
            Some(second_id)
        );
        assert!(sectors.window_id(world.systems[fourth].sector).is_some());
        assert!(coordinator.begin_frame().key(first_id).is_none());
    }

    #[cfg(feature = "interface-test-fixtures")]
    #[test]
    fn queued_sector_duplicate_refreshes_column_before_releasing_dependent() {
        let (mut world, first) = sector_fixture_world();
        let second = add_sector_fixture(&mut world, 0x5000_0010, 800);
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let mut sectors = SectorWindowState::default();
        let mut pending = Vec::<PendingSectorPublication>::new();
        for system in [first, second] {
            assert!(enqueue_sector_window(
                &mut coordinator,
                &sectors,
                &world,
                system,
                CockpitFaction::Alliance,
                &mut pending,
                None,
            ));
            let mut applied = coordinator.apply_pending().remove(0);
            let (_, removed) = commit_sector_window_publication(
                &mut coordinator,
                &mut sectors,
                &mut pending,
                applied.prepared_sector.take(),
                applied.sector_content_id,
                &applied.result,
            );
            assert!(removed.is_empty());
        }
        let first_id = sectors.window_id(world.systems[first].sector).unwrap();
        assert!(enqueue_sector_window(
            &mut coordinator,
            &sectors,
            &world,
            first,
            CockpitFaction::Alliance,
            &mut pending,
            Some(SectorDependentIntent::Defenses { system: first }),
        ));
        sectors.switch_managed_side_for_test(world.systems[first].sector);
        refresh_pending_sector_publication(&mut coordinator, &sectors, &world, &mut pending);
        let mut applied = coordinator.apply_pending().remove(0);
        assert!(matches!(
            applied.result.as_ref().unwrap().transition,
            rebellion_windowing::WindowTransition::Existing { id, .. } if id == first_id
        ));
        let (dependent, removed) = commit_sector_window_publication(
            &mut coordinator,
            &mut sectors,
            &mut pending,
            applied.prepared_sector.take(),
            applied.sector_content_id,
            &applied.result,
        );
        assert!(removed.is_empty());
        assert!(
            matches!(dependent, Some(SectorDependentIntent::Defenses { system }) if system == first)
        );
        assert_eq!(sectors.window_count(), 2);
        assert_eq!(
            sectors.window_id(world.systems[first].sector),
            Some(first_id)
        );
    }

    #[test]
    fn vanished_sector_intent_is_cancelled_before_manager_mutation() {
        let (mut world, first) = sector_fixture_world();
        let second = add_sector_fixture(&mut world, 0x5000_0010, 800);
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let mut sectors = SectorWindowState::default();
        let mut pending = Vec::<PendingSectorPublication>::new();
        assert!(enqueue_sector_window(
            &mut coordinator,
            &sectors,
            &world,
            first,
            CockpitFaction::Alliance,
            &mut pending,
            None,
        ));
        let mut opened = coordinator.apply_pending().remove(0);
        let (_, removed) = commit_sector_window_publication(
            &mut coordinator,
            &mut sectors,
            &mut pending,
            opened.prepared_sector.take(),
            opened.sector_content_id,
            &opened.result,
        );
        assert!(removed.is_empty());
        let first_id = sectors.window_id(world.systems[first].sector).unwrap();
        assert!(enqueue_sector_window(
            &mut coordinator,
            &sectors,
            &world,
            second,
            CockpitFaction::Alliance,
            &mut pending,
            Some(SectorDependentIntent::Defenses { system: second }),
        ));
        world.sectors.remove(world.systems[second].sector);
        refresh_pending_sector_publication(&mut coordinator, &sectors, &world, &mut pending);
        assert!(coordinator.apply_pending().is_empty());
        assert!(pending.is_empty());
        assert_eq!(
            sectors.window_id(world.systems[first].sector),
            Some(first_id)
        );
        assert_eq!(sectors.window_count(), 1);
    }

    #[test]
    fn queued_sector_open_refreshes_against_prior_same_frame_close() {
        let (mut world, first) = sector_fixture_world();
        let second = add_sector_fixture(&mut world, 0x5000_0010, 800);
        let third = add_sector_fixture(&mut world, 0x5000_0020, 900);
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let mut sectors = SectorWindowState::default();
        let mut pending = Vec::<PendingSectorPublication>::new();
        for system in [first, second] {
            assert!(enqueue_sector_window(
                &mut coordinator,
                &sectors,
                &world,
                system,
                CockpitFaction::Alliance,
                &mut pending,
                None,
            ));
            let mut applied = coordinator.apply_pending().remove(0);
            let (_, removed) = commit_sector_window_publication(
                &mut coordinator,
                &mut sectors,
                &mut pending,
                applied.prepared_sector.take(),
                applied.sector_content_id,
                &applied.result,
            );
            assert!(removed.is_empty());
        }
        let first_id = sectors.window_id(world.systems[first].sector).unwrap();
        let second_id = sectors.window_id(world.systems[second].sector).unwrap();
        coordinator.enqueue(WindowCommand::Close(second_id));
        assert!(enqueue_sector_window(
            &mut coordinator,
            &sectors,
            &world,
            third,
            CockpitFaction::Alliance,
            &mut pending,
            Some(SectorDependentIntent::Defenses { system: third }),
        ));
        assert_eq!(pending[0].prepared.unwrap().replacing(), Some(second_id));
        let mut released = Vec::new();
        let pending_command_count = coordinator.begin_apply_batch();
        for _ in 0..pending_command_count {
            refresh_pending_sector_publication(&mut coordinator, &sectors, &world, &mut pending);
            let mut applied = coordinator.apply_next_pending().unwrap();
            let (dependent, rollback) = commit_sector_window_publication(
                &mut coordinator,
                &mut sectors,
                &mut pending,
                applied.prepared_sector.take(),
                applied.sector_content_id,
                &applied.result,
            );
            released.extend(dependent);
            let mut removed = applied.removed_content;
            removed.extend(rollback);
            sectors.remove_content(&removed);
        }
        assert_eq!(sectors.window_count(), 2);
        assert_eq!(
            sectors.window_id(world.systems[first].sector),
            Some(first_id)
        );
        assert!(sectors.window_id(world.systems[third].sector).is_some());
        assert!(matches!(
            released.as_slice(),
            [SectorDependentIntent::Defenses { system }] if *system == third
        ));
    }

    #[test]
    fn invalid_sector_candidate_does_not_pull_new_intent_into_current_batch() {
        let (world, first) = sector_fixture_world();
        let mut world = world;
        let invalid = add_sector_fixture(&mut world, 0x5000_0010, 800);
        let next = add_sector_fixture(&mut world, 0x5000_0020, 900);
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let mut sectors = SectorWindowState::default();
        let mut pending = Vec::<PendingSectorPublication>::new();
        assert!(enqueue_sector_window(
            &mut coordinator,
            &sectors,
            &world,
            first,
            CockpitFaction::Alliance,
            &mut pending,
            None,
        ));
        let mut applied = coordinator.apply_pending().remove(0);
        let (_, removed) = commit_sector_window_publication(
            &mut coordinator,
            &mut sectors,
            &mut pending,
            applied.prepared_sector.take(),
            applied.sector_content_id,
            &applied.result,
        );
        assert!(removed.is_empty());
        for system in [invalid, next] {
            assert!(enqueue_sector_window(
                &mut coordinator,
                &sectors,
                &world,
                system,
                CockpitFaction::Alliance,
                &mut pending,
                Some(SectorDependentIntent::Defenses { system }),
            ));
        }
        assert!(pending[0].prepared.is_some());
        assert!(pending[1].prepared.is_none());
        world.sectors.remove(world.systems[invalid].sector);
        let initial_count = coordinator.begin_apply_batch();
        assert_eq!(initial_count, 1);
        for _ in 0..initial_count {
            refresh_pending_sector_publication(&mut coordinator, &sectors, &world, &mut pending);
            if let Some(mut applied) = coordinator.apply_next_pending() {
                let (_, rollback) = commit_sector_window_publication(
                    &mut coordinator,
                    &mut sectors,
                    &mut pending,
                    applied.prepared_sector.take(),
                    applied.sector_content_id,
                    &applied.result,
                );
                sectors.remove_content(&rollback);
            }
        }
        assert!(sectors.window_id(world.systems[next].sector).is_none());
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].system, next);
        assert!(pending[0].prepared.is_some());
        assert_eq!(coordinator.begin_apply_batch(), 1);
        refresh_pending_sector_publication(&mut coordinator, &sectors, &world, &mut pending);
        let mut next_frame = coordinator.apply_next_pending().unwrap();
        let (dependent, removed) = commit_sector_window_publication(
            &mut coordinator,
            &mut sectors,
            &mut pending,
            next_frame.prepared_sector.take(),
            next_frame.sector_content_id,
            &next_frame.result,
        );
        assert!(removed.is_empty());
        assert!(
            matches!(dependent, Some(SectorDependentIntent::Defenses { system }) if system == next)
        );
        assert!(sectors.window_id(world.systems[next].sector).is_some());
        assert!(pending.is_empty());
    }

    #[test]
    fn queued_duplicate_sector_releases_each_dependent_after_its_own_receipt() {
        let (mut world, system) = sector_fixture_world();
        let member = rebellion_core::missions::MissionMember::Character(
            world.characters.insert(Default::default()),
        );
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let mut sectors = SectorWindowState::default();
        let mut pending = Vec::<PendingSectorPublication>::new();
        assert!(enqueue_sector_window(
            &mut coordinator,
            &sectors,
            &world,
            system,
            CockpitFaction::Alliance,
            &mut pending,
            Some(SectorDependentIntent::Defenses { system }),
        ));
        assert!(enqueue_sector_window(
            &mut coordinator,
            &sectors,
            &world,
            system,
            CockpitFaction::Alliance,
            &mut pending,
            Some(SectorDependentIntent::Missions { system, member }),
        ));

        let mut first = coordinator.apply_pending().remove(0);
        let (released, removed) = commit_sector_window_publication(
            &mut coordinator,
            &mut sectors,
            &mut pending,
            first.prepared_sector.take(),
            first.sector_content_id,
            &first.result,
        );
        assert!(removed.is_empty());
        assert!(matches!(
            released,
            Some(SectorDependentIntent::Defenses { .. })
        ));
        pump_pending_sector_publication(&mut coordinator, &sectors, &world, &mut pending);

        let mut duplicate = coordinator.apply_pending().remove(0);
        assert!(matches!(
            duplicate.result.as_ref().unwrap().transition,
            rebellion_windowing::WindowTransition::Existing {
                effect: rebellion_windowing::ExistingEffect::NoOp,
                ..
            }
        ));
        let (released, removed) = commit_sector_window_publication(
            &mut coordinator,
            &mut sectors,
            &mut pending,
            duplicate.prepared_sector.take(),
            duplicate.sector_content_id,
            &duplicate.result,
        );
        assert!(removed.is_empty());
        assert!(matches!(
            released,
            Some(SectorDependentIntent::Missions { .. })
        ));
        assert!(pending.is_empty());
        assert_eq!(sectors.window_count(), 1);
    }

    #[test]
    fn message_fleet_choice_releases_exact_fleet_only_after_sector_receipt() {
        use rebellion_core::world::Fleet;

        let (mut world, system) = sector_fixture_world();
        let fleet = world.insert_fleet(Fleet {
            location: system,
            capital_ships: Vec::new(),
            fighters: Vec::new(),
            characters: Vec::new(),
            is_alliance: true,
            has_death_star: false,
        });
        world.systems[system].fleets.push(fleet);
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let mut sectors = SectorWindowState::default();
        let mut pending = Vec::<PendingSectorPublication>::new();
        assert!(enqueue_sector_window(
            &mut coordinator,
            &sectors,
            &world,
            system,
            CockpitFaction::Alliance,
            &mut pending,
            super::message_choice_sector_dependent(system, super::FinderTarget::Fleet(fleet)),
        ));
        assert!(sectors.window_id(world.systems[system].sector).is_none());
        assert_eq!(pending.len(), 1);

        let mut applied = coordinator.apply_pending().remove(0);
        let (released, removed) = commit_sector_window_publication(
            &mut coordinator,
            &mut sectors,
            &mut pending,
            applied.prepared_sector.take(),
            applied.sector_content_id,
            &applied.result,
        );
        assert!(removed.is_empty());
        assert!(matches!(
            released,
            Some(SectorDependentIntent::Fleet {
                system: selected_system,
                selected: Some(rebellion_render::fleet_window::FleetWindowEntry::Fleet(selected)),
                rename: false,
            }) if selected_system == system && selected == fleet
        ));
        assert!(pending.is_empty());
        assert!(sectors.window_id(world.systems[system].sector).is_some());
        assert!(
            super::message_choice_sector_dependent(system, super::FinderTarget::Sector).is_none()
        );
    }

    #[test]
    fn sector_publication_uses_active_factions_known_facility_view() {
        use rebellion_core::dat::Faction;
        use rebellion_core::ids::DatId;
        use rebellion_core::world::ManufacturingFacilityInstance;

        let (mut world, system) = sector_fixture_world();
        let enemy_yard = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: DatId::new(0x2900_0001),
                side: Faction::Empire,
                is_shipyard: true,
            });
        world.systems[system]
            .manufacturing_facilities
            .push(enemy_yard);
        let mut alliance_fog = rebellion_core::fog::FogState::new(Faction::Alliance);
        alliance_fog.learn_facilities(&world, system);
        let empire_fog = rebellion_core::fog::FogState::new(Faction::Empire);
        let missions = rebellion_core::missions::MissionState::new();
        let movement = rebellion_core::movement::MovementState::default();
        let mut sectors = SectorWindowState::default();
        assert!(sectors.open_for_system(&world, system, CockpitFaction::Alliance));
        let layout = rebellion_render::CockpitLayout {
            canvas: rebellion_render::CockpitViewport {
                x: 0.0,
                y: 0.0,
                width: 640.0,
                height: 480.0,
            },
            galaxy: rebellion_render::CockpitViewport {
                x: 0.0,
                y: 0.0,
                width: 640.0,
                height: 480.0,
            },
            scale: 1.0,
        };
        let command = super::InterfaceCommand::OpenIconMenu {
            system,
            quadrant: rebellion_render::quadrant_icons::Quadrant::System,
        };
        let selected_fog = super::sector_publication_fog_for_faction(
            super::MissionFaction::Alliance,
            &alliance_fog,
            &empire_fog,
        );
        let actions = sectors
            .command_actions(
                &world,
                selected_fog,
                &missions,
                &movement,
                CockpitFaction::Alliance,
                layout,
                command,
            )
            .expect("active faction sees its learned opposing facility");
        assert!(actions.iter().any(|action| matches!(
            action,
            rebellion_render::sector_window::SectorWindowAction::OpenObjectMenu {
                selection: Some(rebellion_render::object_menu::MenuObject::SystemIcon {
                    system: selected,
                    quadrant: rebellion_render::quadrant_icons::Quadrant::System,
                }),
                ..
            } if *selected == system
        )));
        assert!(sectors
            .command_actions(
                &world,
                &empire_fog,
                &missions,
                &movement,
                CockpitFaction::Alliance,
                layout,
                command,
            )
            .is_err());
    }

    #[test]
    fn sector_routed_close_removes_only_sector_and_preserves_root_detail() {
        use rebellion_render::windowing_policy::{contextual_spec, system_subject};
        use rebellion_windowing::{WindowKind, WindowOwner, WindowTransition};

        let (world, system) = sector_fixture_world();
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let mut sectors = SectorWindowState::default();
        let mut pending = Vec::<PendingSectorPublication>::new();
        assert!(enqueue_sector_window(
            &mut coordinator,
            &sectors,
            &world,
            system,
            CockpitFaction::Alliance,
            &mut pending,
            None,
        ));
        let mut sector_open = coordinator.apply_pending().remove(0);
        let sector = sector_open.sector_content_id.unwrap();
        assert!(commit_sector_window_publication(
            &mut coordinator,
            &mut sectors,
            &mut pending,
            sector_open.prepared_sector.take(),
            Some(sector),
            &sector_open.result,
        )
        .1
        .is_empty());

        coordinator.enqueue(WindowCommand::Open(contextual_spec(
            WindowKind::System,
            system_subject(&world, system).unwrap(),
            WindowOwner::StrategicRoot,
        )));
        let detail = match coordinator
            .apply_pending()
            .remove(0)
            .result
            .unwrap()
            .transition
        {
            WindowTransition::Opened { id } => id,
            transition => panic!("expected System open, got {transition:?}"),
        };
        assert!(enqueue_sector_manager_action(
            &mut coordinator,
            rebellion_render::SectorWindowAction::Close(sector),
        ));
        let closed = coordinator.apply_pending().remove(0);
        assert_eq!(closed.removed_content, vec![sector]);
        sectors.remove_content(&closed.removed_content);
        let snapshot = coordinator.begin_frame();
        assert!(snapshot.key(sector).is_none());
        assert!(snapshot.key(detail).is_some());
        assert_eq!(sectors.window_count(), 0);
    }

    #[test]
    fn sector_manager_actions_enqueue_exact_activate_capture_and_release() {
        use rebellion_windowing::{FocusTarget, WindowTransition};

        let (world, system) = sector_fixture_world();
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let mut sectors = SectorWindowState::default();
        let mut pending = Vec::<PendingSectorPublication>::new();
        assert!(enqueue_sector_window(
            &mut coordinator,
            &sectors,
            &world,
            system,
            CockpitFaction::Alliance,
            &mut pending,
            None,
        ));
        let mut opened = coordinator.apply_pending().remove(0);
        let window = opened.sector_content_id.unwrap();
        assert!(commit_sector_window_publication(
            &mut coordinator,
            &mut sectors,
            &mut pending,
            opened.prepared_sector.take(),
            Some(window),
            &opened.result,
        )
        .1
        .is_empty());

        for action in [
            rebellion_render::SectorWindowAction::Activate(window),
            rebellion_render::SectorWindowAction::CapturePointer(window),
            rebellion_render::SectorWindowAction::ReleasePointer(window),
        ] {
            assert!(enqueue_sector_manager_action(&mut coordinator, action));
            let applied = coordinator.apply_pending();
            assert_eq!(applied.len(), 1);
            assert!(applied[0].result.is_ok());
            match action {
                rebellion_render::SectorWindowAction::Activate(_) => {
                    assert!(matches!(
                        applied[0].result.as_ref().unwrap().transition,
                        WindowTransition::Raised { id } if id == window
                    ));
                    assert_eq!(
                        coordinator.begin_frame().focused(),
                        Some(FocusTarget {
                            window,
                            control: None,
                        })
                    );
                }
                rebellion_render::SectorWindowAction::CapturePointer(_) => {
                    assert_eq!(coordinator.begin_frame().capture_owner(), Some(window));
                }
                rebellion_render::SectorWindowAction::ReleasePointer(_) => {
                    assert_eq!(coordinator.begin_frame().capture_owner(), None);
                }
                _ => unreachable!(),
            }
        }
    }

    #[cfg(feature = "interface-test-fixtures")]
    #[test]
    fn application_hold_cancels_defenses_drag_through_exact_production_receipt() {
        use rebellion_render::windowing_policy::contextual_spec;
        use rebellion_windowing::{
            CaptureCancelReason, SubjectId, WindowEvent, WindowKind, WindowOwner, WindowTransition,
        };

        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        coordinator.enqueue(WindowCommand::Open(contextual_spec(
            WindowKind::Defenses,
            SubjectId::new(2, 100),
            WindowOwner::StrategicRoot,
        )));
        let window = match coordinator.apply_pending()[0]
            .result
            .as_ref()
            .unwrap()
            .transition
        {
            WindowTransition::Opened { id } => id,
            ref transition => panic!("expected Defenses open, got {transition:?}"),
        };
        coordinator.enqueue(WindowCommand::CapturePointer(window));
        assert!(coordinator.apply_pending()[0].result.is_ok());

        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &coordinator.begin_frame());
        let press = InputEventId {
            frame: 1,
            ordinal: 0,
        };
        assert_eq!(
            router.collect_pointer(
                press,
                &BTreeSet::from([window]),
                PointerPhase::Press(PointerButtonId::Primary),
            ),
            InputRecipient::Window(window)
        );
        assert!(router.claim(press, InputRecipient::Window(window)));

        let mut sectors = SectorWindowState::default();
        let mut fleets = FleetWindowState::default();
        let mut defenses = DefensesWindowState::default();
        defenses.seed_managed_drag_for_test(window);
        assert!(defenses.is_dragging());
        assert_eq!(defenses.drag_owner(), Some(window));

        enqueue_application_hold_capture_cancellation(
            &mut coordinator,
            true,
            sectors.drag_owner(),
            fleets.drag_owner(),
            defenses.drag_owner(),
        );
        let applied = coordinator.apply_pending();
        let commit = applied[0].result.as_ref().unwrap();
        assert_eq!(
            commit.events,
            vec![WindowEvent::CaptureCancelled {
                window,
                reason: CaptureCancelReason::HoldStarted,
            }]
        );
        apply_capture_cancellation_events(
            &commit.events,
            &mut router,
            &mut sectors,
            &mut fleets,
            &mut defenses,
        );

        assert_eq!(coordinator.begin_frame().capture_owner(), None);
        assert!(!defenses.is_dragging());
        assert_eq!(defenses.drag_owner(), None);

        router.begin_frame(2, &coordinator.begin_frame());
        let release = InputEventId {
            frame: 2,
            ordinal: 0,
        };
        assert_eq!(
            router.collect_pointer(
                release,
                &BTreeSet::new(),
                PointerPhase::Release(PointerButtonId::Primary),
            ),
            InputRecipient::Blocked,
            "the exact cancellation receipt quarantines the orphan release"
        );
        assert!(!router.claim_root(release));
    }

    #[cfg(feature = "interface-test-fixtures")]
    #[test]
    fn application_hold_cancels_sector_drag_through_exact_production_receipt() {
        use rebellion_windowing::{CaptureCancelReason, WindowEvent};

        let (world, system) = sector_fixture_world();
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let mut sectors = SectorWindowState::default();
        let mut pending = Vec::<PendingSectorPublication>::new();
        assert!(enqueue_sector_window(
            &mut coordinator,
            &sectors,
            &world,
            system,
            CockpitFaction::Alliance,
            &mut pending,
            None,
        ));
        let mut opened = coordinator.apply_pending().remove(0);
        let window = opened.sector_content_id.unwrap();
        assert!(commit_sector_window_publication(
            &mut coordinator,
            &mut sectors,
            &mut pending,
            opened.prepared_sector.take(),
            Some(window),
            &opened.result,
        )
        .1
        .is_empty());
        coordinator.enqueue(WindowCommand::CapturePointer(window));
        assert!(coordinator.apply_pending()[0].result.is_ok());
        sectors.seed_managed_drag_for_test(window);
        assert_eq!(sectors.drag_owner(), Some(window));

        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &coordinator.begin_frame());
        let mut fleets = FleetWindowState::default();
        let mut defenses = DefensesWindowState::default();
        enqueue_application_hold_capture_cancellation(
            &mut coordinator,
            true,
            sectors.drag_owner(),
            fleets.drag_owner(),
            defenses.drag_owner(),
        );
        let cancelled = coordinator.apply_pending().remove(0);
        let commit = cancelled.result.unwrap();
        assert_eq!(
            commit.events,
            vec![WindowEvent::CaptureCancelled {
                window,
                reason: CaptureCancelReason::HoldStarted,
            }]
        );
        apply_capture_cancellation_events(
            &commit.events,
            &mut router,
            &mut sectors,
            &mut fleets,
            &mut defenses,
        );
        assert_eq!(coordinator.begin_frame().capture_owner(), None);
        assert_eq!(sectors.drag_owner(), None);
    }

    #[cfg(feature = "interface-test-fixtures")]
    #[test]
    fn application_hold_cancellation_requires_both_hold_and_drag_owner() {
        let (world, system) = sector_fixture_world();
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let mut sectors = SectorWindowState::default();
        let mut pending = Vec::<PendingSectorPublication>::new();
        assert!(enqueue_sector_window(
            &mut coordinator,
            &sectors,
            &world,
            system,
            CockpitFaction::Alliance,
            &mut pending,
            None,
        ));
        let mut opened = coordinator.apply_pending().remove(0);
        let window = opened.sector_content_id.unwrap();
        assert!(commit_sector_window_publication(
            &mut coordinator,
            &mut sectors,
            &mut pending,
            opened.prepared_sector.take(),
            Some(window),
            &opened.result,
        )
        .1
        .is_empty());
        coordinator.enqueue(WindowCommand::CapturePointer(window));
        assert!(coordinator.apply_pending()[0].result.is_ok());
        sectors.seed_managed_drag_for_test(window);

        enqueue_application_hold_capture_cancellation(
            &mut coordinator,
            false,
            sectors.drag_owner(),
            None,
            None,
        );
        assert!(coordinator.apply_pending().is_empty());
        assert_eq!(coordinator.begin_frame().capture_owner(), Some(window));
        enqueue_application_hold_capture_cancellation(&mut coordinator, true, None, None, None);
        assert!(coordinator.apply_pending().is_empty());
        assert_eq!(coordinator.begin_frame().capture_owner(), Some(window));
    }

    #[test]
    fn message_index_outside_press_is_blocked_before_app_map_dispatch() {
        let mut manager = WindowManager::new();
        let mut message_index = MessageIndexState::new();
        let prepared = MessageIndexState::prepare_open(0x79).unwrap();
        let committed = manager.apply(WindowCommand::Open(prepared.spec())).unwrap();
        message_index
            .commit_open(prepared, &committed.transition)
            .expect("manager receipt publishes Message Index content");
        let frame = manager.begin_frame();
        let layout = CockpitLayout {
            canvas: CockpitViewport {
                x: 0.0,
                y: 0.0,
                width: 640.0,
                height: 480.0,
            },
            galaxy: CockpitViewport {
                x: 65.0,
                y: 60.0,
                width: 485.0,
                height: 350.0,
            },
            scale: 1.0,
        };
        let mut router = FrameInputRouter::default();
        let status = StatusWindowState::default();
        let build_selection = BuildSelectionState::default();
        router.begin_frame(9, &frame);
        let batch = collect_strategic_pointer_batch(
            9,
            &mut router,
            &[StrategicRawPointerEvent {
                ordinal: 0,
                // Inside the galaxy aperture but outside the centered
                // Message Index rectangle.
                point: (66.0, 61.0),
                phase: PointerPhase::Press(PointerButtonId::Primary),
            }],
            &frame,
            layout,
            &message_index,
            &status,
            &build_selection,
            |_| BTreeSet::new(),
        );
        let event = batch.events[0].id;
        let map_pointer_blocked =
            batch.blocks_pointer_phase(&router, PointerPhase::Press(PointerButtonId::Primary));
        let mut selected_map_system = None;
        if !map_pointer_blocked && router.claim_root(event) {
            selected_map_system = Some(7_u16);
        }

        assert_eq!(router.pointer_recipient(event), InputRecipient::Blocked);
        assert!(map_pointer_blocked);
        assert_eq!(selected_map_system, None);
    }

    #[test]
    fn finder_close_quarantines_release_before_retained_legacy_input() {
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        coordinator
            .enqueue_fleet_finder_open(FleetFinderState::prepare_open(CockpitFaction::Alliance));
        let finder = coordinator.apply_pending()[0]
            .fleet_finder_content_id
            .expect("Fleet Finder open allocates an identity");
        assert!(coordinator.acknowledge_fleet_finder_publication(finder));

        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &coordinator.begin_frame());
        let modal_batch = StrategicPointerBatch::collect_from_raw(
            1,
            &mut router,
            &[
                StrategicRawPointerEvent {
                    ordinal: 0,
                    point: (20.0, 20.0),
                    phase: PointerPhase::Press(PointerButtonId::Secondary),
                },
                StrategicRawPointerEvent {
                    ordinal: 1,
                    point: (20.0, 20.0),
                    phase: PointerPhase::Auxiliary,
                },
            ],
            |_| BTreeSet::from([finder]),
        );
        assert_eq!(
            router.pointer_recipient(modal_batch.events[0].id),
            InputRecipient::Window(finder)
        );
        let ctx = egui::Context::default();
        let point = egui::pos2(20.0, 20.0);
        let mut legacy_filter = RetainedLegacyInputFilter::default();
        let modal_input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(200.0, 200.0),
            )),
            events: vec![
                egui::Event::PointerButton {
                    pos: point,
                    button: egui::PointerButton::Secondary,
                    pressed: true,
                    modifiers: egui::Modifiers::default(),
                },
                egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Line,
                    delta: egui::vec2(0.0, 1.0),
                    modifiers: egui::Modifiers::default(),
                },
            ],
            ..egui::RawInput::default()
        };
        let _ = ctx.run(modal_input, |ctx| {
            let saved = legacy_filter
                .install(ctx, &modal_batch, &router, true)
                .expect("a modal installs filtered legacy input");
            assert!(!ctx.input(|input| input.pointer.secondary_down()));
            assert_eq!(
                ctx.input(|input| input.smooth_scroll_delta),
                egui::Vec2::ZERO
            );
            restore_egui_input(ctx, saved);
        });

        coordinator.enqueue(rebellion_windowing::WindowCommand::Close(finder));
        assert!(coordinator.apply_pending()[0].result.is_ok());
        router.begin_frame(2, &coordinator.begin_frame());
        let mixed = StrategicPointerBatch::collect_from_raw(
            2,
            &mut router,
            &[
                StrategicRawPointerEvent {
                    ordinal: 0,
                    point: (20.0, 20.0),
                    phase: PointerPhase::Release(PointerButtonId::Secondary),
                },
                StrategicRawPointerEvent {
                    ordinal: 1,
                    point: (30.0, 30.0),
                    phase: PointerPhase::Press(PointerButtonId::Primary),
                },
            ],
            |_| BTreeSet::new(),
        );
        assert_eq!(
            router.pointer_recipient(mixed.events[0].id),
            InputRecipient::Blocked
        );
        assert_eq!(
            router.pointer_recipient(mixed.events[1].id),
            InputRecipient::Root
        );
        assert!(
            mixed.blocks_pointer_phase(&router, PointerPhase::Release(PointerButtonId::Secondary))
        );
        assert!(!mixed.blocks_pointer_phase(&router, PointerPhase::Press(PointerButtonId::Primary)));

        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(200.0, 200.0),
            )),
            events: vec![
                egui::Event::PointerButton {
                    pos: point,
                    button: egui::PointerButton::Secondary,
                    pressed: false,
                    modifiers: egui::Modifiers::default(),
                },
                egui::Event::PointerButton {
                    pos: egui::pos2(30.0, 30.0),
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::default(),
                },
            ],
            ..egui::RawInput::default()
        };
        let mut legacy_saw_release = false;
        let mut legacy_saw_press = false;
        let mut managed_seam_saw_both = false;
        let _ = ctx.run(input, |ctx| {
            let saved = legacy_filter
                .install(ctx, &mixed, &router, false)
                .expect("the blocked release installs event-filtered legacy input");
            legacy_saw_release = ctx.input(|input| input.pointer.secondary_released());
            legacy_saw_press = ctx.input(|input| input.pointer.primary_pressed());
            restore_egui_input(ctx, saved);
            managed_seam_saw_both = ctx.input(|input| {
                input.pointer.secondary_released() && input.pointer.primary_pressed()
            });
        });

        assert!(!legacy_saw_release);
        assert!(legacy_saw_press);
        assert!(managed_seam_saw_both);

        router.begin_frame(3, &coordinator.begin_frame());
        let empty = StrategicPointerBatch::default();
        let mut retained_state_is_clean = false;
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(200.0, 200.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| {
                let saved = legacy_filter
                    .install(ctx, &empty, &router, false)
                    .expect("clean frames retain the filtered input history");
                retained_state_is_clean = ctx.input(|input| {
                    input.pointer.primary_down()
                        && !input.pointer.secondary_down()
                        && !input.pointer.secondary_released()
                        && input.raw_scroll_delta == egui::Vec2::ZERO
                        && input.smooth_scroll_delta == egui::Vec2::ZERO
                });
                restore_egui_input(ctx, saved);
            },
        );
        assert!(retained_state_is_clean);
    }

    #[test]
    fn routed_sector_press_blocks_map_without_parallel_geometry_state() {
        use rebellion_render::windowing_policy::contextual_spec;
        use rebellion_windowing::{SubjectId, WindowKind, WindowOwner, WindowTransition};

        let mut manager = WindowManager::new();
        let window = match manager
            .apply(WindowCommand::Open(contextual_spec(
                WindowKind::Sector,
                SubjectId::new(1, 36),
                WindowOwner::StrategicRoot,
            )))
            .unwrap()
            .transition
        {
            WindowTransition::Opened { id } => id,
            other => panic!("expected Sector open, got {other:?}"),
        };
        let snapshot = manager.begin_frame();
        let mut router = FrameInputRouter::default();
        router.begin_frame(31, &snapshot);
        let batch = StrategicPointerBatch::collect_from_raw(
            31,
            &mut router,
            &[StrategicRawPointerEvent {
                ordinal: 0,
                point: (100.0, 100.0),
                phase: PointerPhase::Press(PointerButtonId::Primary),
            }],
            |_| BTreeSet::from([window]),
        );

        assert_eq!(
            router.pointer_recipient(batch.events[0].id),
            InputRecipient::Window(window)
        );
        assert!(!batch.blocks_pointer_phase(&router, PointerPhase::Press(PointerButtonId::Primary)));
        assert!(
            batch.blocks_map_pointer_phase(&router, PointerPhase::Press(PointerButtonId::Primary))
        );
        let mut map = rebellion_render::GalaxyMapState::default();
        super::latch_map_pointer_from_strategic_batch(&mut map, &batch, &router);
        assert!(
            map.pointer_blocked,
            "managed Sector press blocks map underlay"
        );

        let mut root_router = FrameInputRouter::default();
        root_router.begin_frame(32, &snapshot);
        let root_batch = StrategicPointerBatch::collect_from_raw(
            32,
            &mut root_router,
            &[StrategicRawPointerEvent {
                ordinal: 0,
                point: (500.0, 400.0),
                phase: PointerPhase::Press(PointerButtonId::Primary),
            }],
            |_| BTreeSet::new(),
        );
        assert_eq!(
            root_router.pointer_recipient(root_batch.events[0].id),
            InputRecipient::Root
        );
        super::latch_map_pointer_from_strategic_batch(&mut map, &root_batch, &root_router);
        assert!(
            map.pointer_blocked,
            "a later Root-only batch preserves the earlier block"
        );
    }

    #[test]
    fn sector_pointer_batch_preserves_event_order_points_and_phase_specific_map_ownership() {
        use rebellion_render::sector_window::SectorWindowInputEvent;
        use rebellion_render::windowing_policy::contextual_spec;
        use rebellion_windowing::{SubjectId, WindowKind, WindowOwner, WindowTransition};

        let mut manager = WindowManager::new();
        let window = match manager
            .apply(WindowCommand::Open(contextual_spec(
                WindowKind::Sector,
                SubjectId::new(1, 36),
                WindowOwner::StrategicRoot,
            )))
            .unwrap()
            .transition
        {
            WindowTransition::Opened { id } => id,
            other => panic!("expected Sector open, got {other:?}"),
        };
        let mut router = FrameInputRouter::default();
        router.begin_frame(32, &manager.begin_frame());
        let raw = [
            StrategicRawPointerEvent {
                ordinal: 0,
                point: (10.0, 20.0),
                phase: PointerPhase::Press(PointerButtonId::Secondary),
            },
            StrategicRawPointerEvent {
                ordinal: 1,
                point: (110.0, 120.0),
                phase: PointerPhase::Press(PointerButtonId::Primary),
            },
            StrategicRawPointerEvent {
                ordinal: 2,
                point: (30.0, 40.0),
                phase: PointerPhase::Release(PointerButtonId::Secondary),
            },
            StrategicRawPointerEvent {
                ordinal: 3,
                point: (130.0, 140.0),
                phase: PointerPhase::Release(PointerButtonId::Primary),
            },
        ];
        let batch = StrategicPointerBatch::collect_from_raw(32, &mut router, &raw, |point| {
            if point.0 >= 100.0 {
                BTreeSet::from([window])
            } else {
                BTreeSet::new()
            }
        });
        assert_eq!(
            batch.sector_events(),
            raw.map(|event| SectorWindowInputEvent {
                id: InputEventId {
                    frame: 32,
                    ordinal: event.ordinal,
                },
                point: event.point,
                phase: event.phase,
            })
        );
        assert_eq!(
            batch
                .events
                .iter()
                .map(|event| router.pointer_recipient(event.id))
                .collect::<Vec<_>>(),
            [
                InputRecipient::Root,
                InputRecipient::Window(window),
                InputRecipient::Root,
                InputRecipient::Window(window),
            ]
        );
        assert!(
            batch.blocks_map_pointer_phase(&router, PointerPhase::Press(PointerButtonId::Primary))
        );
        assert!(batch
            .blocks_map_pointer_phase(&router, PointerPhase::Release(PointerButtonId::Primary)));
        assert!(!batch
            .blocks_map_pointer_phase(&router, PointerPhase::Press(PointerButtonId::Secondary)));
        assert!(!batch
            .blocks_map_pointer_phase(&router, PointerPhase::Release(PointerButtonId::Secondary)));
        assert!(!batch.blocks_map_pointer_phase(&router, PointerPhase::Hover));
    }

    #[test]
    fn tactical_modal_route_quarantines_release_after_scene_switch() {
        use crate::strategic_windowing::WindowScene;
        use rebellion_render::tactical_view::TacticalWindowPhase;

        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let alert = coordinator
            .commit_scene_phase_for_battle(
                WindowScene::Tactical,
                Some(1),
                TacticalWindowPhase::Alert,
            )
            .unwrap()
            .window
            .unwrap()
            .id;
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &coordinator.begin_frame());
        let press = StrategicPointerBatch::collect_from_raw(
            1,
            &mut router,
            &[StrategicRawPointerEvent {
                ordinal: 0,
                point: (300.0, 350.0),
                phase: PointerPhase::Press(PointerButtonId::Primary),
            }],
            |_| BTreeSet::from([alert]),
        );
        let admitted =
            route_tactical_window_input(&press, &mut router, alert, (300.0, 350.0), true, 0.0);
        assert_eq!(
            admitted.events,
            vec![TacticalWindowEvent::PrimaryPress((300.0, 350.0))]
        );

        coordinator
            .commit_scene_phase_for_battle(WindowScene::Galaxy, None, TacticalWindowPhase::Inactive)
            .unwrap();
        router.begin_frame(2, &coordinator.begin_frame());
        let release = StrategicPointerBatch::collect_from_raw(
            2,
            &mut router,
            &[StrategicRawPointerEvent {
                ordinal: 0,
                point: (300.0, 350.0),
                phase: PointerPhase::Release(PointerButtonId::Primary),
            }],
            |_| BTreeSet::new(),
        );
        let quarantined =
            route_tactical_window_input(&release, &mut router, alert, (300.0, 350.0), false, 0.0);
        assert_eq!(quarantined.events, vec![TacticalWindowEvent::CancelGesture]);
        assert_eq!(
            router.pointer_recipient(release.events[0].id),
            InputRecipient::Blocked
        );
    }

    #[test]
    fn tactical_modal_route_preserves_same_frame_raw_event_order() {
        use crate::strategic_windowing::WindowScene;
        use rebellion_render::tactical_view::TacticalWindowPhase;

        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let alert = coordinator
            .commit_scene_phase_for_battle(
                WindowScene::Tactical,
                Some(4),
                TacticalWindowPhase::Alert,
            )
            .unwrap()
            .window
            .unwrap()
            .id;
        let raw = [
            StrategicRawPointerEvent {
                ordinal: 0,
                point: (101.0, 102.0),
                phase: PointerPhase::Press(PointerButtonId::Primary),
            },
            StrategicRawPointerEvent {
                ordinal: 1,
                point: (201.0, 202.0),
                phase: PointerPhase::Release(PointerButtonId::Primary),
            },
            StrategicRawPointerEvent {
                ordinal: 2,
                point: (301.0, 302.0),
                phase: PointerPhase::Press(PointerButtonId::Primary),
            },
            StrategicRawPointerEvent {
                ordinal: 3,
                point: (401.0, 402.0),
                phase: PointerPhase::Auxiliary,
            },
        ];
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &coordinator.begin_frame());
        let batch = StrategicPointerBatch::collect_from_raw(1, &mut router, &raw, |_| {
            BTreeSet::from([alert])
        });

        let routed =
            route_tactical_window_input(&batch, &mut router, alert, (9.0, 10.0), false, -2.5);
        assert_eq!(
            routed.events,
            vec![
                TacticalWindowEvent::PrimaryPress((101.0, 102.0)),
                TacticalWindowEvent::PrimaryRelease((201.0, 202.0)),
                TacticalWindowEvent::PrimaryPress((301.0, 302.0)),
                TacticalWindowEvent::Wheel {
                    screen: (401.0, 402.0),
                    delta_y: -2.5,
                },
            ]
        );
        assert!(batch
            .events
            .iter()
            .all(|event| { router.pointer_recipient(event.id) == InputRecipient::Window(alert) }));
        assert!(batch
            .events
            .iter()
            .all(|event| { !router.claim(event.id, InputRecipient::Window(alert)) }));
    }

    #[test]
    fn tactical_result_exit_waits_for_exact_close_receipt() {
        use crate::strategic_windowing::WindowScene;
        use rebellion_render::tactical_view::TacticalWindowPhase;
        use rebellion_render::TacticalAction;

        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let opened = coordinator
            .commit_scene_phase_for_battle(
                WindowScene::Tactical,
                Some(7),
                TacticalWindowPhase::Results,
            )
            .unwrap();
        let prepared = PreparedTacticalWindowAction {
            action: TacticalAction::ReturnToGalaxy,
            battle_session: 7,
            window: opened.window.unwrap(),
        };

        assert_eq!(
            approve_tactical_window_action(
                prepared.clone(),
                Some(7),
                TacticalWindowPhase::Results,
                &opened,
            ),
            None,
            "an open receipt cannot authorize Results effects"
        );
        assert!(coordinator
            .commit_tactical_transition_if_exact(8, prepared.window, TacticalWindowPhase::Inactive,)
            .unwrap()
            .is_none());
        assert_eq!(
            coordinator.begin_frame().active_modal(),
            Some(prepared.window.id),
            "a stale battle token cannot retire the live Results identity"
        );

        let closed = coordinator
            .commit_tactical_transition_if_exact(7, prepared.window, TacticalWindowPhase::Inactive)
            .unwrap()
            .expect("the exact live identity closes");
        assert_eq!(
            approve_tactical_window_action(
                prepared,
                Some(7),
                TacticalWindowPhase::Results,
                &closed,
            ),
            Some(TacticalAction::ReturnToGalaxy)
        );

        let alert = coordinator
            .commit_scene_phase_for_battle(
                WindowScene::Tactical,
                Some(9),
                TacticalWindowPhase::Alert,
            )
            .unwrap();
        let prepared = PreparedTacticalWindowAction {
            action: TacticalAction::BeginCombat,
            battle_session: 9,
            window: alert.window.unwrap(),
        };
        assert_eq!(
            approve_tactical_window_action(
                prepared.clone(),
                Some(9),
                TacticalWindowPhase::Alert,
                &alert,
            ),
            None,
            "Alert effects also wait for the exact close receipt"
        );
        let closed = coordinator
            .commit_tactical_transition_if_exact(9, prepared.window, TacticalWindowPhase::Combat)
            .unwrap()
            .expect("the exact Alert identity closes into Combat");
        assert_eq!(
            approve_tactical_window_action(prepared, Some(9), TacticalWindowPhase::Alert, &closed,),
            Some(TacticalAction::BeginCombat)
        );
    }

    #[test]
    fn tactical_auto_resolve_uses_atomic_alert_results_replace() {
        use crate::strategic_windowing::WindowScene;
        use rebellion_render::tactical_view::TacticalWindowPhase;
        use rebellion_render::TacticalAction;
        use rebellion_windowing::{CloseReason, WindowEvent, WindowKind};

        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let alert = coordinator
            .commit_scene_phase_for_battle(
                WindowScene::Tactical,
                Some(21),
                TacticalWindowPhase::Alert,
            )
            .unwrap()
            .window
            .unwrap();
        let mut pending = Some(PreparedTacticalWindowAction {
            action: TacticalAction::AutoResolve,
            battle_session: 21,
            window: alert,
        });

        let committed = commit_pending_tactical_window_action(
            &mut coordinator,
            &mut pending,
            Some(21),
            TacticalWindowPhase::Alert,
        )
        .unwrap()
        .expect("production AutoResolve replaces Alert with Results");
        let CommittedTacticalWindowAction {
            phase: replaced,
            approved_action,
        } = committed;
        let results = replaced.window.expect("Results opens in the same commit");
        assert_eq!(results.subject, alert.subject);
        assert_eq!(results.kind, WindowKind::BattleResults);
        assert_ne!(results.id, alert.id);
        assert_eq!(replaced.removed_content, [alert.id]);
        assert!(replaced.events.iter().any(|event| matches!(
            event,
            WindowEvent::Closed {
                window,
                reason: CloseReason::Replaced,
                ..
            } if *window == alert.id
        )));
        assert_eq!(approved_action, Some(TacticalAction::AutoResolve));
        assert!(pending.is_none());
    }

    #[test]
    fn tactical_action_approval_rejects_wrong_receipts_and_dispatches_once() {
        use crate::strategic_windowing::WindowScene;
        use rebellion_render::tactical_view::TacticalWindowPhase;
        use rebellion_render::TacticalAction;
        use rebellion_windowing::{CloseReason, SubjectId, WindowEvent, WindowKey, WindowKind};

        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let alert = coordinator
            .commit_scene_phase_for_battle(
                WindowScene::Tactical,
                Some(31),
                TacticalWindowPhase::Alert,
            )
            .unwrap()
            .window
            .unwrap();
        let results = coordinator
            .commit_scene_phase_for_battle(
                WindowScene::Tactical,
                Some(31),
                TacticalWindowPhase::Results,
            )
            .unwrap()
            .window
            .unwrap();
        let prepared = PreparedTacticalWindowAction {
            action: TacticalAction::ReturnToGalaxy,
            battle_session: 31,
            window: results,
        };
        let expected_key = WindowKey::contextual(WindowKind::BattleResults, results.subject);
        let exact_event = WindowEvent::Closed {
            window: results.id,
            key: expected_key,
            reason: CloseReason::Requested,
        };
        let candidate =
            |event, removed_content| crate::strategic_windowing::CommittedTacticalPhase {
                window: None,
                events: vec![event],
                removed_content,
            };

        for wrong in [
            candidate(
                WindowEvent::Closed {
                    window: alert.id,
                    key: expected_key,
                    reason: CloseReason::Requested,
                },
                vec![results.id],
            ),
            candidate(
                WindowEvent::Closed {
                    window: results.id,
                    key: WindowKey::contextual(WindowKind::BattleResults, SubjectId::new(99, 99)),
                    reason: CloseReason::Requested,
                },
                vec![results.id],
            ),
            candidate(
                WindowEvent::Closed {
                    window: results.id,
                    key: expected_key,
                    reason: CloseReason::Replaced,
                },
                vec![results.id],
            ),
            candidate(exact_event, vec![alert.id]),
        ] {
            assert_eq!(
                approve_tactical_window_action(
                    prepared.clone(),
                    Some(31),
                    TacticalWindowPhase::Results,
                    &wrong,
                ),
                None
            );
        }

        let exact = coordinator
            .commit_tactical_transition_if_exact(31, results, TacticalWindowPhase::Inactive)
            .unwrap()
            .unwrap();
        let mut pending = Some(prepared);
        assert_eq!(
            take_approved_tactical_window_action(
                &mut pending,
                Some(31),
                TacticalWindowPhase::Results,
                &exact,
            ),
            Some(TacticalAction::ReturnToGalaxy)
        );
        assert_eq!(
            take_approved_tactical_window_action(
                &mut pending,
                Some(31),
                TacticalWindowPhase::Results,
                &exact,
            ),
            None,
            "one exact receipt can dispatch the prepared effect at most once"
        );
    }

    #[test]
    fn tactical_results_destination_waits_for_receipt_and_eventual_galaxy() {
        use crate::strategic_windowing::WindowScene;
        use rebellion_core::ids::SystemKey;
        use rebellion_render::tactical_view::TacticalWindowPhase;
        use rebellion_render::TacticalAction;

        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let results = coordinator
            .commit_scene_phase_for_battle(
                WindowScene::Tactical,
                Some(51),
                TacticalWindowPhase::Results,
            )
            .unwrap()
            .window
            .unwrap();
        let routed = TacticalAction::OpenBattleSystem(SystemKey::default());
        let mut pending_action = Some(PreparedTacticalWindowAction {
            action: routed.clone(),
            battle_session: 51,
            window: results,
        });
        let mut pending_destination = None;

        assert_eq!(
            pending_destination, None,
            "routing alone cannot queue navigation"
        );
        let preclose = coordinator
            .commit_scene_phase_for_battle(
                WindowScene::Tactical,
                Some(51),
                TacticalWindowPhase::Results,
            )
            .unwrap();
        assert_eq!(
            take_approved_tactical_window_action(
                &mut pending_action,
                Some(51),
                TacticalWindowPhase::Results,
                &preclose,
            ),
            None,
            "the routed action remains inert before its exact close receipt"
        );
        assert_eq!(pending_destination, None);

        let closed = coordinator
            .commit_tactical_transition_if_exact(51, results, TacticalWindowPhase::Inactive)
            .unwrap()
            .unwrap();
        let approved = take_approved_tactical_window_action(
            &mut pending_action,
            Some(51),
            TacticalWindowPhase::Results,
            &closed,
        )
        .expect("the exact Results close releases navigation");
        pending_destination = tactical_result_destination(&approved);
        assert_eq!(
            pending_destination,
            Some(TacticalResultDestination::System(SystemKey::default()))
        );

        assert_eq!(
            take_tactical_destination_for_galaxy(&mut pending_destination, false),
            None,
            "GroundCombat retains the approved destination"
        );
        assert!(pending_destination.is_some());
        assert_eq!(
            take_tactical_destination_for_galaxy(&mut pending_destination, true),
            Some(TacticalResultDestination::System(SystemKey::default())),
            "the first eventual Galaxy frame dispatches it"
        );
        assert_eq!(
            take_tactical_destination_for_galaxy(&mut pending_destination, true),
            None,
            "an approved destination dispatches at most once"
        );
    }

    #[test]
    fn tactical_pending_handoff_reset_clears_action_and_destination() {
        use crate::strategic_windowing::WindowScene;
        use rebellion_core::ids::FleetKey;
        use rebellion_render::tactical_view::TacticalWindowPhase;
        use rebellion_render::TacticalAction;

        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let results = coordinator
            .commit_scene_phase_for_battle(
                WindowScene::Tactical,
                Some(61),
                TacticalWindowPhase::Results,
            )
            .unwrap()
            .window
            .unwrap();
        let mut action = Some(PreparedTacticalWindowAction {
            action: TacticalAction::OpenBattleFleet(FleetKey::default()),
            battle_session: 61,
            window: results,
        });
        let mut destination = Some(TacticalResultDestination::Fleet(FleetKey::default()));

        reset_pending_tactical_handoff(&mut action, &mut destination);

        assert!(action.is_none());
        assert!(destination.is_none());
    }
}

#[cfg(test)]
mod panel_toggle_tests {
    use super::{
        collect_strategic_pointer_batch, commit_build_selection_gameplay,
        legacy_child_key_disposition, managed_strategic_surface_plan, panel_letters_open_panels,
        toggle_exclusive_panel, BuildSelectionAction, ManagedStrategicSurface,
        StrategicPointerBatch, StrategicRawInputCollector, StrategicRawKeyCollector,
        StrategicRawKeyEvent, StrategicRawPointerEvent, StrategicRootKeyBatch,
    };
    use macroquad::miniquad::{EventHandler, KeyMods, MouseButton};
    use macroquad::prelude::KeyCode;
    use rebellion_render::build_selection::BuildSelectionState;
    use rebellion_render::message_index::MessageIndexState;
    use rebellion_render::status_rows::{StatusObject, StatusSources};
    use rebellion_render::status_window::StatusWindowState;
    use rebellion_render::windowing_input::{
        FrameInputRouter, InputDisposition, InputRecipient, PointerButtonId, PointerPhase,
    };
    use rebellion_render::{CockpitLayout, CockpitViewport};

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
        batch.report_legacy_children(&mut router, false, None, None, None, None, None, None);

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
        batch.report_legacy_children(&mut router, false, None, None, None, None, None, None);

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
        batch.report_legacy_children(&mut router, true, None, None, None, None, None, None);

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

    fn opened_encyclopedia(
        coordinator: &mut crate::strategic_windowing::StrategicWindowing,
    ) -> rebellion_windowing::WindowId {
        use rebellion_render::windowing_policy::singleton_spec;
        use rebellion_windowing::{WindowCommand, WindowKind, WindowOwner, WindowTransition};

        coordinator.enqueue(WindowCommand::Open(singleton_spec(
            WindowKind::Encyclopedia,
            WindowOwner::StrategicRoot,
        )));
        let applied = coordinator.apply_pending();
        match applied[0].result.as_ref().unwrap().transition {
            WindowTransition::Opened { id } => id,
            ref transition => panic!("expected Encyclopedia open, got {transition:?}"),
        }
    }

    #[test]
    fn managed_status_owns_production_pointer_keyboard_and_paint_seams() {
        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let encyclopedia = opened_encyclopedia(&mut coordinator);
        let mut status = StatusWindowState::default();
        let mut world = rebellion_core::world::GameWorld::default();
        let character = world.characters.insert(Default::default());
        let missions = rebellion_core::missions::MissionState::new();
        let movement = rebellion_core::movement::MovementState::new();
        let transport = rebellion_core::troop_transport::TroopTransportState::new();
        let manufacturing = rebellion_core::manufacturing::ManufacturingState::new();
        let sources = StatusSources {
            world: &world,
            missions: &missions,
            movement: &movement,
            transport: &transport,
            manufacturing: &manufacturing,
            today: 0,
        };
        let subject = coordinator.next_context_subject().unwrap();
        coordinator.enqueue_status_open(
            StatusWindowState::prepare_open(sources, StatusObject::Character(character), subject)
                .unwrap(),
            None,
        );
        let mut applied = coordinator.apply_pending().remove(0);
        let status_id = applied.status_content_id.unwrap();
        assert_eq!(
            status.commit_open(
                applied.prepared_status.take().unwrap(),
                &applied.result.as_ref().unwrap().transition,
            ),
            Some(status_id)
        );
        assert!(coordinator.acknowledge_status_publication(status_id));

        let frame = coordinator.begin_frame();
        assert_eq!(
            managed_strategic_surface_plan(
                &frame,
                Some(encyclopedia),
                None,
                None,
                Some(status_id),
                None,
            ),
            [
                (ManagedStrategicSurface::Encyclopedia(encyclopedia), false),
                (ManagedStrategicSurface::Status(status_id), true),
            ]
        );

        let layout = CockpitLayout {
            canvas: CockpitViewport {
                x: 0.0,
                y: 0.0,
                width: 640.0,
                height: 480.0,
            },
            galaxy: CockpitViewport {
                x: 65.0,
                y: 60.0,
                width: 485.0,
                height: 350.0,
            },
            scale: 1.0,
        };
        let mut pointer_router = FrameInputRouter::default();
        pointer_router.begin_frame(10, &frame);
        let pointer = collect_strategic_pointer_batch(
            10,
            &mut pointer_router,
            &[
                StrategicRawPointerEvent {
                    ordinal: 0,
                    // In the galaxy aperture but outside the centered Status.
                    point: (66.0, 61.0),
                    phase: PointerPhase::Press(PointerButtonId::Primary),
                },
                StrategicRawPointerEvent {
                    ordinal: 1,
                    point: (320.0, 240.0),
                    phase: PointerPhase::Press(PointerButtonId::Primary),
                },
            ],
            &frame,
            layout,
            &MessageIndexState::default(),
            &status,
            &BuildSelectionState::default(),
            |_| std::collections::BTreeSet::new(),
        );
        assert_eq!(
            pointer_router.pointer_recipient(pointer.events[0].id),
            InputRecipient::Blocked
        );
        assert_eq!(
            pointer_router.pointer_recipient(pointer.events[1].id),
            InputRecipient::Window(status_id)
        );
        assert!(pointer.blocks_pointer_phase(
            &pointer_router,
            PointerPhase::Press(PointerButtonId::Primary),
        ));

        let mut key_router = FrameInputRouter::default();
        key_router.begin_frame(11, &frame);
        let keys = StrategicRootKeyBatch::collect_from_raw(
            11,
            &mut key_router,
            &[key(KeyCode::F2, false)],
        );
        assert_eq!(status.keyboard_owner(&frame), Some(status_id));
        keys.report_legacy_children(
            &mut key_router,
            false,
            None,
            None,
            None,
            None,
            Some(status_id),
            None,
        );
        assert_eq!(
            legacy_child_key_disposition(
                keys.events[0],
                InputRecipient::Window(status_id),
                false,
                None,
                None,
                None,
                None,
                Some(status_id),
                None,
            ),
            Some(InputDisposition::Unhandled)
        );
        assert_eq!(
            key_router.keyboard_recipient(keys.events[0].id),
            InputRecipient::Window(status_id)
        );
        assert!(!key_router.claim_root(keys.events[0].id));
    }

    #[test]
    fn managed_modal_surfaces_follow_snapshot_order_and_input_eligibility() {
        use rebellion_render::fleet_finder::FleetFinderState;
        use rebellion_windowing::WindowCommand;

        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        coordinator.enqueue_fleet_finder_open(FleetFinderState::prepare_open(
            rebellion_render::CockpitFaction::Alliance,
        ));
        let finder = match coordinator.apply_pending()[0]
            .result
            .as_ref()
            .unwrap()
            .transition
        {
            rebellion_windowing::WindowTransition::Opened { id } => id,
            ref transition => panic!("expected Fleet Finder open, got {transition:?}"),
        };
        let encyclopedia = opened_encyclopedia(&mut coordinator);
        let frame = coordinator.begin_frame();

        assert_eq!(
            managed_strategic_surface_plan(
                &frame,
                Some(encyclopedia),
                Some(finder),
                None,
                None,
                None,
            ),
            [
                (ManagedStrategicSurface::FleetFinder(finder), false),
                (ManagedStrategicSurface::Encyclopedia(encyclopedia), true,),
            ]
        );
        assert_eq!(
            frame
                .pointer_route(&std::collections::BTreeSet::from([finder]))
                .target,
            rebellion_windowing::RouteTarget::BlockedBy(encyclopedia)
        );

        coordinator.enqueue(WindowCommand::Close(encyclopedia));
        coordinator.apply_pending();
        assert_eq!(
            managed_strategic_surface_plan(
                &coordinator.begin_frame(),
                None,
                Some(finder),
                None,
                None,
                None,
            ),
            [(ManagedStrategicSurface::FleetFinder(finder), true)]
        );

        let mut reverse = crate::strategic_windowing::StrategicWindowing::new();
        let encyclopedia = opened_encyclopedia(&mut reverse);
        reverse.enqueue_fleet_finder_open(FleetFinderState::prepare_open(
            rebellion_render::CockpitFaction::Alliance,
        ));
        let finder = match reverse.apply_pending()[0]
            .result
            .as_ref()
            .unwrap()
            .transition
        {
            rebellion_windowing::WindowTransition::Opened { id } => id,
            ref transition => panic!("expected Fleet Finder open, got {transition:?}"),
        };
        let frame = reverse.begin_frame();
        assert_eq!(
            managed_strategic_surface_plan(
                &frame,
                Some(encyclopedia),
                Some(finder),
                None,
                None,
                None,
            ),
            [
                (ManagedStrategicSurface::Encyclopedia(encyclopedia), false,),
                (ManagedStrategicSurface::FleetFinder(finder), true),
            ]
        );
        assert_eq!(
            frame
                .pointer_route(&std::collections::BTreeSet::from([encyclopedia]))
                .target,
            rebellion_windowing::RouteTarget::BlockedBy(finder)
        );
    }

    #[test]
    fn managed_build_selection_owns_production_routes_and_paint_seam() {
        use rebellion_core::ids::{DatId, SystemKey};
        use rebellion_core::manufacturing::{BuildableKind, ProductionArea};

        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let mut state = BuildSelectionState::default();
        coordinator.enqueue_build_selection_open(
            state
                .prepare_open(
                    SystemKey::default(),
                    ProductionArea::TrainingFacility,
                    true,
                    vec![BuildableKind::Troop(DatId::new(0x1000_0001))],
                )
                .unwrap(),
        );
        let mut applied = coordinator.apply_pending().remove(0);
        let id = applied.build_selection_content_id.unwrap();
        assert_eq!(
            state.commit_open(
                applied.prepared_build_selection.take().unwrap(),
                &applied.result.as_ref().unwrap().transition,
            ),
            Some(id)
        );
        assert!(coordinator.acknowledge_build_selection_publication(id));

        let frame = coordinator.begin_frame();
        assert_eq!(
            managed_strategic_surface_plan(&frame, None, None, None, None, Some(id)),
            [(ManagedStrategicSurface::BuildSelection(id), true)]
        );
        assert_eq!(state.keyboard_owner(&frame), Some(id));
        assert_eq!(
            frame
                .pointer_route(&std::collections::BTreeSet::new())
                .target,
            rebellion_windowing::RouteTarget::BlockedBy(id)
        );

        let layout = CockpitLayout {
            canvas: CockpitViewport {
                x: 0.0,
                y: 0.0,
                width: 640.0,
                height: 480.0,
            },
            galaxy: CockpitViewport {
                x: 65.0,
                y: 60.0,
                width: 485.0,
                height: 350.0,
            },
            scale: 1.0,
        };
        let mut pointer_router = FrameInputRouter::default();
        pointer_router.begin_frame(12, &frame);
        let pointer = collect_strategic_pointer_batch(
            12,
            &mut pointer_router,
            &[
                StrategicRawPointerEvent {
                    ordinal: 0,
                    point: (66.0, 61.0),
                    phase: PointerPhase::Press(PointerButtonId::Primary),
                },
                StrategicRawPointerEvent {
                    ordinal: 1,
                    point: (307.5, 235.0),
                    phase: PointerPhase::Press(PointerButtonId::Primary),
                },
            ],
            &frame,
            layout,
            &MessageIndexState::default(),
            &StatusWindowState::default(),
            &state,
            |_| std::collections::BTreeSet::new(),
        );
        assert_eq!(
            pointer_router.pointer_recipient(pointer.events[0].id),
            InputRecipient::Blocked
        );
        assert_eq!(
            pointer_router.pointer_recipient(pointer.events[1].id),
            InputRecipient::Window(id)
        );

        let mut key_router = FrameInputRouter::default();
        key_router.begin_frame(13, &frame);
        let keys = StrategicRootKeyBatch::collect_from_raw(
            13,
            &mut key_router,
            &[key(KeyCode::F2, false), key(KeyCode::Escape, false)],
        );
        keys.report_legacy_children(
            &mut key_router,
            false,
            None,
            None,
            None,
            None,
            None,
            Some(id),
        );
        assert_eq!(
            legacy_child_key_disposition(
                keys.events[0],
                InputRecipient::Window(id),
                false,
                None,
                None,
                None,
                None,
                None,
                Some(id),
            ),
            Some(InputDisposition::Unhandled)
        );
        assert_eq!(
            legacy_child_key_disposition(
                keys.events[1],
                InputRecipient::Window(id),
                false,
                None,
                None,
                None,
                None,
                None,
                Some(id),
            ),
            Some(InputDisposition::Handled)
        );
        assert!(!key_router.root_may_handle(keys.events[0].id));
        assert!(!key_router.root_may_handle(keys.events[1].id));
    }

    #[test]
    fn build_selection_gameplay_commits_once_after_exact_teardown_and_live_source() {
        use rebellion_core::dat::{ExplorationStatus, Faction};
        use rebellion_core::ids::{DatId, SectorKey};
        use rebellion_core::manufacturing::{BuildableKind, ManufacturingState, ProductionArea};
        use rebellion_core::world::{
            BuildableClass, ControlKind, GameWorld, ManufacturingFacilityInstance, System,
        };
        use rebellion_render::windowing_policy::singleton_spec;
        use rebellion_windowing::{
            WindowCommand, WindowKind, WindowManager, WindowOwner, WindowTransition,
        };

        let mut world = GameWorld::default();
        let system = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0001),
            name: "Bortras".into(),
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
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Alliance),
        });
        let yard_class = DatId::new(0x2900_0002);
        let product_class = DatId::new(0x1000_0001);
        world.buildable_classes.insert(
            yard_class,
            BuildableClass {
                refined_material_cost: 10,
                processing_rate: 4,
                is_alliance: true,
                is_empire: false,
                ..BuildableClass::default()
            },
        );
        world.buildable_classes.insert(
            product_class,
            BuildableClass {
                refined_material_cost: 8,
                is_alliance: true,
                is_empire: false,
                ..BuildableClass::default()
            },
        );
        let yard = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: yard_class,
                side: Faction::Alliance,
                is_shipyard: false,
            });
        world.systems[system].manufacturing_facilities.push(yard);

        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let mut state = BuildSelectionState::default();
        let mut manufacturing = ManufacturingState::new();
        coordinator.enqueue_build_selection_open(
            state
                .prepare_open(
                    system,
                    ProductionArea::TrainingFacility,
                    true,
                    vec![BuildableKind::Troop(product_class)],
                )
                .unwrap(),
        );
        let mut opened = coordinator.apply_pending().remove(0);
        let id = opened.build_selection_content_id.unwrap();
        assert_eq!(
            state.commit_open(
                opened.prepared_build_selection.take().unwrap(),
                &opened.result.as_ref().unwrap().transition,
            ),
            Some(id)
        );
        assert!(coordinator.acknowledge_build_selection_publication(id));

        let prepared = state.prepare_confirm(&world).unwrap();
        let mut unrelated = WindowManager::new();
        unrelated
            .apply(WindowCommand::Open(singleton_spec(
                WindowKind::Encyclopedia,
                WindowOwner::StrategicRoot,
            )))
            .unwrap();
        let wrong = unrelated
            .apply(WindowCommand::Open(singleton_spec(
                WindowKind::MessageIndex,
                WindowOwner::StrategicRoot,
            )))
            .unwrap();
        let WindowTransition::Opened { id: wrong_id } = wrong.transition else {
            panic!("expected manager-issued wrong identity");
        };
        assert_ne!(wrong_id, id);
        assert_eq!(
            commit_build_selection_gameplay(
                Some(prepared),
                &[wrong_id],
                &world,
                &mut state,
                &mut manufacturing,
            ),
            None
        );
        assert!(state.is_open());
        assert_eq!(manufacturing.queued_at(system), 0);
        assert_eq!(
            commit_build_selection_gameplay(
                Some(prepared),
                &[],
                &world,
                &mut state,
                &mut manufacturing,
            ),
            None
        );
        assert!(state.is_open());
        assert_eq!(manufacturing.queued_at(system), 0);

        coordinator.enqueue_build_selection_confirm(prepared);
        coordinator.enqueue_build_selection_confirm(prepared);
        let mut applied = coordinator.apply_pending();
        let accepted = applied.remove(0);
        let rejected = applied.remove(0);
        assert!(rejected.result.is_err());
        assert_eq!(rejected.confirmed_build_selection, None);
        assert_eq!(
            commit_build_selection_gameplay(
                rejected.confirmed_build_selection,
                &rejected.removed_content,
                &world,
                &mut state,
                &mut manufacturing,
            ),
            None
        );
        assert_eq!(manufacturing.queued_at(system), 0);

        assert!(matches!(
            commit_build_selection_gameplay(
                accepted.confirmed_build_selection,
                &accepted.removed_content,
                &world,
                &mut state,
                &mut manufacturing,
            ),
            Some(BuildSelectionAction::Confirm { count: 1, .. })
        ));
        assert_eq!(manufacturing.queued_at(system), 1);
        assert_eq!(
            commit_build_selection_gameplay(
                Some(prepared),
                &accepted.removed_content,
                &world,
                &mut state,
                &mut manufacturing,
            ),
            None
        );
        assert_eq!(manufacturing.queued_at(system), 1);

        coordinator.enqueue_build_selection_open(
            state
                .prepare_open(
                    system,
                    ProductionArea::TrainingFacility,
                    true,
                    vec![BuildableKind::Troop(product_class)],
                )
                .unwrap(),
        );
        let mut reopened = coordinator.apply_pending().remove(0);
        let reopened_id = reopened.build_selection_content_id.unwrap();
        assert_eq!(
            state.commit_open(
                reopened.prepared_build_selection.take().unwrap(),
                &reopened.result.as_ref().unwrap().transition,
            ),
            Some(reopened_id)
        );
        assert!(coordinator.acknowledge_build_selection_publication(reopened_id));
        let stale = state.prepare_confirm(&world).unwrap();
        coordinator.enqueue_build_selection_confirm(stale);
        let stale_close = coordinator.apply_pending().remove(0);
        world.systems[system].manufacturing_facilities.clear();
        assert_eq!(
            commit_build_selection_gameplay(
                stale_close.confirmed_build_selection,
                &stale_close.removed_content,
                &world,
                &mut state,
                &mut manufacturing,
            ),
            None
        );
        assert_eq!(manufacturing.queued_at(system), 1);
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

        batch.report_legacy_children(
            &mut router,
            false,
            Some((fleet, false)),
            None,
            None,
            None,
            None,
            None,
        );

        assert!(router.claim_root(batch.events[0].id));
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

        batch.report_legacy_children(
            &mut router,
            false,
            Some((fleet, true)),
            None,
            None,
            None,
            None,
            None,
        );

        let bubbled: Vec<bool> = batch
            .events
            .iter()
            .map(|event| router.claim_root(event.id))
            .collect();
        assert_eq!(bubbled, [false, true, false, true, false, true]);
    }

    #[test]
    fn encyclopedia_search_handling_precedes_cockpit_shortcuts() {
        use rebellion_render::{windowing_input::InputRecipient, ENCYCLOPEDIA_SEARCH_CONTROL};
        use rebellion_windowing::{FocusTarget, WindowCommand};

        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let encyclopedia = opened_encyclopedia(&mut coordinator);
        coordinator.enqueue(WindowCommand::Focus(FocusTarget {
            window: encyclopedia,
            control: Some(ENCYCLOPEDIA_SEARCH_CONTROL),
        }));
        assert!(coordinator.apply_pending()[0].result.is_ok());
        let frame = coordinator.begin_frame();
        assert_eq!(
            frame.focused(),
            Some(FocusTarget {
                window: encyclopedia,
                control: Some(ENCYCLOPEDIA_SEARCH_CONTROL),
            })
        );
        let mut router = FrameInputRouter::default();
        router.begin_frame(5, &frame);
        let batch =
            StrategicRootKeyBatch::collect_from_raw(5, &mut router, &[key(KeyCode::F, false)]);

        assert_eq!(
            router.keyboard_recipient(batch.events[0].id),
            InputRecipient::Window(encyclopedia)
        );
        batch.report_legacy_children(
            &mut router,
            true,
            None,
            None,
            Some(encyclopedia),
            None,
            None,
            None,
        );

        assert!(!router.claim_root(batch.events[0].id));
    }

    #[test]
    fn encyclopedia_escape_close_keeps_frame_keyboard_lease() {
        use rebellion_render::windowing_input::InputRecipient;
        use rebellion_windowing::WindowCommand;

        let mut coordinator = crate::strategic_windowing::StrategicWindowing::new();
        let encyclopedia = opened_encyclopedia(&mut coordinator);
        let frozen = coordinator.begin_frame();
        let mut router = FrameInputRouter::default();
        router.begin_frame(6, &frozen);
        let batch =
            StrategicRootKeyBatch::collect_from_raw(6, &mut router, &[key(KeyCode::Escape, false)]);

        coordinator.enqueue(WindowCommand::Close(encyclopedia));
        assert!(coordinator.apply_pending()[0].result.is_ok());
        assert!(coordinator.begin_frame().paint_order().is_empty());
        assert_eq!(
            router.keyboard_recipient(batch.events[0].id),
            InputRecipient::Window(encyclopedia)
        );
        batch.report_legacy_children(
            &mut router,
            false,
            None,
            None,
            Some(encyclopedia),
            None,
            None,
            None,
        );

        assert!(!router.claim_root(batch.events[0].id));
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
    let mut game_mode = GameMode::MainMenu;
    let mut main_menu_state = MainMenuState::default();
    let mut game_setup_state = GameSetupState::default();
    let mut credits_state = CreditsState::default();
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
    let mut death_star_state = DeathStarState::default();
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
    // Find HQ systems for victory detection
    let alliance_hq = world
        .systems
        .iter()
        .find(|(_, s)| s.is_headquarters && s.control.is_controlled_by(Faction::Alliance))
        .map(|(k, _)| k);
    let empire_hq = world
        .systems
        .iter()
        .find(|(_, s)| s.is_headquarters && s.control.is_controlled_by(Faction::Empire))
        .map(|(k, _)| k);
    let mut victory_state = if let (Some(a), Some(e)) = (alliance_hq, empire_hq) {
        VictoryState::new(a, e)
    } else {
        // Fallback: use first two systems if HQs not marked
        let mut keys = world.systems.keys();
        let a = keys
            .next()
            .expect("world must have at least 2 systems for victory");
        let e = keys
            .next()
            .expect("world must have at least 2 systems for victory");
        VictoryState::new(a, e)
    };

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
    let mut status_window_state = StatusWindowState::default();
    let mut fleet_finder_state = FleetFinderState::default();
    let mut encyclopedia_surface = encyclopedia_surface::EncyclopediaSurfaceController::new();
    let mut troop_finder_state = TroopFinderState::default();
    let mut personnel_finder_state = PersonnelFinderState::default();
    // The open Message Index (window 0x0d) and the speed its Advice
    // category saved (FUN_00487ff0 +0x58).
    let mut message_index = MessageIndexState::new();
    let mut advice_saved_speed: Option<GameSpeed> = None;
    // The sound of the message the Message Index last showed, played by the
    // next frame's audio step.
    let mut pending_message_sound: Option<u16> = None;
    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    let mut enc_state = EncyclopediaState::new();
    let mut bombardment_panel_state = BombardmentPanelState::default();
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
    // Frames the next line has waited for the galaxy view.
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_galaxy_wait_frames: u32 = 0;
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_capture: Option<String> = None;
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_input = dev_input::DevInput::default();
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_queue_drained = false;
    #[cfg(not(target_arch = "wasm32"))]
    let mut dev_commands_ready: Vec<dev_commands::DevCommand> = Vec::new();
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
    let mut show_bombardment = false;
    let mut show_death_star = false;
    let mut save_load_panel_state = rebellion_render::SaveLoadPanelState::default();
    let saves_dir = rebellion_data::save::default_saves_dir();
    let mut save_slots = read_save_slots(&saves_dir);

    // ── Event screen overlay ─────────────────────────────────────────────────
    let mut event_screen_state = EventScreenState::new();

    // ── Tactical combat state ────────────────────────────────────────────────
    let mut tactical_state = TacticalState::new();
    let mut pending_tactical_window_action: Option<PreparedTacticalWindowAction> = None;
    let mut pending_tactical_destination: Option<TacticalResultDestination> = None;
    let mut ground_combat_state: Option<GroundCombatState> = None;

    // ── Cockpit chrome ───────────────────────────────────────────────────────
    let mut cockpit_state = CockpitState::new(CockpitFaction::Alliance);
    let mut game_speed_ui = GameSpeedUiState::default();
    let mut cockpit_tooltips = rebellion_render::tooltip::TooltipState::default();
    let mut sector_window_state = SectorWindowState::default();
    let mut pending_sector_publications = Vec::<PendingSectorPublication>::new();
    let mut pending_committed_sector_actions = Vec::<SectorWindowAction>::new();
    let mut system_window_state = SystemWindowState::default();
    let mut fleet_window_state = FleetWindowState::default();
    let mut defenses_window_state = DefensesWindowState::default();
    let mut missions_window_state = MissionsWindowState::default();
    // The object pop-up menu a right-click in a system window opens.
    let mut object_menu: Option<ObjectMenuState> = None;
    let mut indexed_status_menu_witness: Option<IndexedStatusMenuWitness> = None;
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

    let mut cutscene_player = open_cutscene(
        Path::new(INTRO_CUTSCENE),
        audio_vol.cutscene_volume(),
        &mut msg_log,
        clock.tick,
        #[cfg(not(target_arch = "wasm32"))]
        &mut audio_engine,
    );
    if cutscene_player.is_some() {
        game_mode = GameMode::Cutscene {
            kind: CutsceneKind::Intro,
        };
    }

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
    // Temporary projection from manager commits into the detail families
    // that still use the legacy cross-family focus serial.
    let mut system_legacy_bridge_serial = 0_u64;
    let mut fleet_legacy_bridge_serial = 0_u64;
    let mut defenses_legacy_bridge_serial = 0_u64;
    let mut missions_legacy_bridge_serial = 0_u64;
    let mut strategic_input_router = rebellion_render::windowing_input::FrameInputRouter::default();
    let mut retained_legacy_input_filter = RetainedLegacyInputFilter::default();
    let mut strategic_frame_number = 0_u64;
    let strategic_input_subscriber = macroquad::input::utils::register_input_subscriber();
    let mut strategic_raw_input = StrategicRawInputCollector::default();
    let mut focus_loss_watch = strategic_windowing::FocusLossWatch::default();
    #[cfg(target_os = "macos")]
    let mut native_focus = strategic_windowing::FocusStateCounter::default();
    #[cfg(target_os = "macos")]
    let mut native_active_logged: Option<bool> = None;

    loop {
        let dt = get_frame_time();
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
                    .filter(|_| !advisor_state.holds_clock()),
            );
            for step in advisor_state.take_cockpit_steps() {
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
            play_message_sound(&mut audio_engine, pending_message_sound.take(), &audio_vol);
            #[cfg(target_arch = "wasm32")]
            if let Some(engine) = browser_menu_audio.as_mut() {
                play_message_sound(engine, pending_message_sound.take(), &audio_vol);
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

        // ── Event screen overlay timer ────────────────────────────────────
        update_event_screen(&mut event_screen_state, dt);

        // ── Global keyboard shortcuts ───────────────────────────────────────
        if matches!(game_mode, GameMode::Cutscene { .. }) {
            if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Space) {
                if let Some(player) = cutscene_player.as_mut() {
                    player.stop();
                }
            }
        } else if is_key_pressed(KeyCode::Escape)
            && !event_screen_state.is_active()
            && game_mode != GameMode::Galaxy
        {
            if let GameMode::GameOptions { origin } = game_mode.clone() {
                if game_options_state.escape() == GameOptionsAction::Return {
                    game_mode = match origin {
                        GameOptionsOrigin::ShuttleCockpit => GameMode::MainMenu,
                        GameOptionsOrigin::CommandCenter => GameMode::Galaxy,
                        GameOptionsOrigin::TacticalBattle => GameMode::TacticalCombat,
                    };
                }
            } else if game_mode == GameMode::LoadGame {
                save_load_panel_state.close();
                game_mode = GameMode::MainMenu;
            } else if matches!(game_mode, GameMode::Credits | GameMode::MultiplayerSetup) {
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
        // ── Tick the clock (Galaxy mode only) ────────────────────────────────
        // A new game's clock holds through the briefing until its step 11.
        let tick_events = if game_mode == GameMode::Galaxy
            && (interface_fixture_active || !advisor_state.holds_clock())
        {
            clock.advance(dt)
        } else {
            vec![]
        };

        if !tick_events.is_empty() {
            // Used by the Dabora 2 notification paths that need to timestamp
            // message-log entries. `current_tick` is re-bound further down for
            // the rest of the tick loop; this earlier binding is read-only.
            let economy_tick = tick_events.last().map_or(0, |e| e.tick);

            // Active movement orders are authoritative; repair stale orbit
            // indexes before economy and manufacturing inspect fleet presence.
            reconcile_fleet_orbits(&movement_state, &mut world);

            // ── Economy (runs BEFORE manufacturing — affects production) ──────
            let economy_events = EconomySystem::advance_with_uprisings(
                &mut economy_state,
                &world,
                &tick_events,
                world.difficulty_index,
                &uprising_state,
            );
            for ev in &economy_events {
                match ev {
                    EconomyEvent::SupportDrifted {
                        system,
                        alliance_delta,
                        empire_delta,
                    } => {
                        if let Some(sys) = world.systems.get_mut(*system) {
                            sys.popularity_alliance =
                                (sys.popularity_alliance + alliance_delta).clamp(0.0, 1.0);
                            sys.popularity_empire =
                                (sys.popularity_empire + empire_delta).clamp(0.0, 1.0);
                        }
                    }
                    EconomyEvent::ControlResolved {
                        system,
                        new_control,
                    } => {
                        let side = |faction: Option<Faction>| match faction {
                            Some(Faction::Alliance) => Some(true),
                            Some(Faction::Empire) => Some(false),
                            _ => None,
                        };
                        let old = world
                            .systems
                            .get(*system)
                            .and_then(|sys| side(sys.control.faction()));
                        if let Some(sys) = world.systems.get_mut(*system) {
                            sys.control = *new_control;
                        }
                        // FUN_00510d70 → FUN_004f6f40: the system's facilities
                        // follow its new holder.
                        world.hand_over_facilities(*system);
                        // Only a change of side is a loyalty change; a
                        // contested system has not declared neutrality.
                        let new = side(new_control.faction());
                        if new == old
                            || matches!(new_control, rebellion_core::world::ControlKind::Contested)
                        {
                            continue;
                        }
                        for message in loyalty_messages(
                            &world,
                            &text_templates,
                            *system,
                            economy_tick,
                            new,
                            old,
                        ) {
                            msg_log.push(message);
                        }
                    }
                    // Knesset Shamash-Bet Dabora 2 notification events —
                    // surface them in the interactive message log.
                    EconomyEvent::ResourceDiscovered { system, new_output } => {
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
                    EconomyEvent::MaintenanceShortfall {
                        faction_is_alliance,
                        deficit_system_count,
                    } => {
                        let faction_str = if *faction_is_alliance {
                            "Alliance"
                        } else {
                            "Empire"
                        };
                        // Notification 0xc, Maintenance Shortfall.
                        msg_log.push(filed(
GameMessage::new(
                            economy_tick,
                            format!(
                                "{faction_str} reports maintenance shortfall across {deficit_system_count} systems"
                            ),
                            MessageCategory::Event,
                        )
                        // FUN_00498970: a maintenance shortfall carries code 0xc.
                        .with_advice(0xc),
MessageRail::Manufacturing,
Some(RailAudience::side(*faction_is_alliance)),
));
                    }
                    _ => {} // Telemetry-only events (collection rate, garrison, incidents, support change tier)
                }
            }

            // ── Stockpiles: facility cycles, yard work, overdraft scraps ─────
            let stockpile_rolls: Vec<f64> = (0..tick_events.len())
                .map(|_| sim_rng.gen::<f64>())
                .collect();
            let stockpile_events = StockpileSystem::advance(
                &mut stockpile_state,
                &world,
                &mut mfg_state,
                &economy_state,
                &tick_events,
                blockade_state.blockaded_systems(),
                &stockpile_rolls,
            );
            rebellion_core::scrap::scrap_all(
                &mut world,
                &mut stockpile_state,
                &mut troop_transport_state,
                stockpile_events.iter().filter_map(|event| match *event {
                    StockpileEvent::OverdraftScrap { target, .. } => Some(target),
                    _ => None,
                }),
            );

            // ── Manufacturing (blockaded systems are skipped) ─────────────────
            // Use advance_tracked so we also pick up K6 EVT_MANUFACTURING_IDLE
            // transitions for the interactive message log.
            let mfg_advance = ManufacturingSystem::advance_tracked(
                &mut mfg_state,
                &tick_events,
                blockade_state.blockaded_systems(),
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
                let destination = system_name(&world, departure.destination);
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
            delivery_state.depart(&world, &mfg_advance.departures);
            let delivered = delivery_state.advance(&world, &tick_events);
            for completion in &delivered.arrivals {
                rebellion_data::integrator::apply_build_completion_inner(completion, &mut world);
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

            // Unopposed troop transports can land immediately. Contested
            // orbits retain cargo until space combat produces a winner.
            let mut ground_resolved_systems = HashSet::new();
            let current_tick = tick_events.last().map_or(0, |event| event.tick);
            let landing_targets: Vec<_> = world
                .systems
                .iter()
                .filter_map(|(system, value)| {
                    let has_alliance = value.fleets.iter().any(|fleet| {
                        world
                            .fleets
                            .get(*fleet)
                            .is_some_and(|value| value.is_alliance)
                    });
                    let has_empire = value.fleets.iter().any(|fleet| {
                        world
                            .fleets
                            .get(*fleet)
                            .is_some_and(|value| !value.is_alliance)
                    });
                    let faction = match (has_alliance, has_empire) {
                        (true, false) => Some(true),
                        (false, true) => Some(false),
                        _ => None,
                    }?;
                    value
                        .fleets
                        .iter()
                        .any(|fleet| troop_transport_state.landing_count(*fleet) > 0)
                        .then_some((system, faction))
                })
                .collect();
            for (system, is_alliance) in landing_targets {
                if !is_alliance && system == victory_state.alliance_hq {
                    let bombardment_fleet = world.systems.get(system).and_then(|value| {
                        value.fleets.iter().copied().find(|fleet| {
                            world
                                .fleets
                                .get(*fleet)
                                .is_some_and(|value| !value.is_alliance)
                                && troop_transport_state.landing_count(*fleet) > 0
                        })
                    });
                    if let Some(fleet) = bombardment_fleet {
                        tactical_flow::apply_automatic_bombardment(
                            &mut world,
                            &victory_state,
                            fleet,
                            system,
                            current_tick,
                            &mut msg_log,
                        );
                    }
                }
                let ground_rolls: Vec<f64> = (0..256).map(|_| sim_rng.gen::<f64>()).collect();
                ground_resolved_systems.insert(system);
                tactical_flow::resolve_ground_campaign(
                    &mut world,
                    &mut troop_transport_state,
                    system,
                    is_alliance,
                    &ground_rolls,
                    current_tick,
                    &mut msg_log,
                );
            }

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
                    let sys = &world.systems[sys_key];
                    let alliance_fleets: Vec<_> = sys
                        .fleets
                        .iter()
                        .copied()
                        .filter(|&k| world.fleets.get(k).is_some_and(|f| f.is_alliance))
                        .collect();
                    let empire_fleets: Vec<_> = sys
                        .fleets
                        .iter()
                        .copied()
                        .filter(|&k| world.fleets.get(k).is_some_and(|f| !f.is_alliance))
                        .collect();
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

            for (sys_key, atk_force, def_force) in combat_triggers {
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
                    if let Err(error) = tactical_flow::begin_player_battle(
                        &world,
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
                    break; // Handle one player battle at a time.
                }

                // Nobody watches this battle: it runs headless (FUN_0040a050
                // state 8), as the shared simulation's battles do.
                let space_result = rebellion_data::integrator::resolve_system_space_combat(
                    &mut world,
                    &mut troop_transport_state,
                    &mut movement_state,
                    sys_key,
                    sim_rng.gen::<u32>(),
                    current_tick,
                );
                for line in &space_result.log {
                    macroquad::logging::info!("{}", line);
                }
                tactical_flow::reconcile_death_star_result(
                    &world,
                    &mut death_star_state,
                    &mut victory_state,
                );

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

                // Surviving transports land only after a decisive space result.
                // FUN_00517270: a side left with only its system squadrons
                // is still present, so the system stays contested.
                let winner = match (
                    side_present(&world, sys_key, true),
                    side_present(&world, sys_key, false),
                ) {
                    (true, false) => atk_force.map(|fleet| (true, fleet)),
                    (false, true) => def_force.map(|fleet| (false, fleet)),
                    _ => None,
                }
                .filter(|&(_, fleet)| world.fleets.contains_key(fleet));
                if let Some((winner_is_alliance, winner_fleet)) = winner {
                    tactical_flow::apply_automatic_bombardment(
                        &mut world,
                        &victory_state,
                        winner_fleet,
                        sys_key,
                        current_tick,
                        &mut msg_log,
                    );

                    let ground_rolls: Vec<f64> = (0..256).map(|_| sim_rng.gen::<f64>()).collect();
                    ground_resolved_systems.insert(sys_key);
                    tactical_flow::resolve_ground_campaign(
                        &mut world,
                        &mut troop_transport_state,
                        sys_key,
                        winner_is_alliance,
                        &ground_rolls,
                        current_tick,
                        &mut msg_log,
                    );
                }
            }

            // Continue unresolved surface engagements even after every troop
            // has left its transport. This mirrors the shared simulation loop
            // and prevents a no-progress round from freezing an invasion until
            // another fleet happens to arrive.
            let continuing_ground_battles: Vec<_> = world
                .systems
                .keys()
                .filter_map(|system| {
                    if ground_resolved_systems.contains(&system) {
                        return None;
                    }
                    let value = world.systems.get(system)?;
                    let (alliance_troops, empire_troops) =
                        value
                            .ground_units
                            .iter()
                            .fold((false, false), |counts, troop| {
                                match world.troops.get(*troop) {
                                    Some(value)
                                        if value.regiment_strength > 0 && value.is_alliance =>
                                    {
                                        (true, counts.1)
                                    }
                                    Some(value) if value.regiment_strength > 0 => (counts.0, true),
                                    _ => counts,
                                }
                            });
                    if !alliance_troops || !empire_troops {
                        return None;
                    }

                    let (alliance_fleet, empire_fleet) =
                        value.fleets.iter().fold((false, false), |counts, fleet| {
                            match world.fleets.get(*fleet) {
                                Some(value) if value.is_alliance => (true, counts.1),
                                Some(_) => (counts.0, true),
                                None => counts,
                            }
                        });
                    let attacker_is_alliance =
                        !matches!((alliance_fleet, empire_fleet), (false, true));
                    Some((system, attacker_is_alliance))
                })
                .collect();
            for (system, attacker_is_alliance) in continuing_ground_battles {
                let ground_rolls: Vec<f64> = (0..256).map(|_| sim_rng.gen::<f64>()).collect();
                tactical_flow::resolve_ground_campaign(
                    &mut world,
                    &mut troop_transport_state,
                    system,
                    attacker_is_alliance,
                    &ground_rolls,
                    current_tick,
                    &mut msg_log,
                );
            }

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
            ai_state.free_mission_members(&mission_advance.effects);
            if let Some(second_ai) = secondary_ai_state.as_mut() {
                second_ai.free_mission_members(&mission_advance.effects);
            }

            for result in &mission_advance.results {
                apply_mission_result(
                    result,
                    &mut world,
                    &mut msg_log,
                    #[cfg(not(target_arch = "wasm32"))]
                    &mut audio_engine,
                    #[cfg(not(target_arch = "wasm32"))]
                    &audio_vol,
                );
                for ended in rebellion_data::integrator::apply_mission_state_effects(
                    &result.effects,
                    &world,
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

            // ── Character escapes ────────────────────────────────────────────
            let escape_rolls: Vec<f64> = (0..world.characters.len())
                .map(|_| sim_rng.gen::<f64>())
                .collect();
            let escape_effects = MissionSystem::check_escapes(&world, &escape_rolls);
            for effect in &escape_effects {
                if let MissionEffect::CharacterEscaped {
                    character,
                    escaped_to_alliance,
                } = effect
                {
                    if let Some(c) = world.characters.get_mut(*character) {
                        c.is_alliance = *escaped_to_alliance;
                        c.is_empire = !*escaped_to_alliance;
                        c.is_captive = false;
                        c.captured_by = None;
                        c.capture_tick = None;
                    }
                    for (_, fleet) in &mut world.fleets {
                        fleet.characters.retain(|&k| k != *character);
                    }
                    let name = world
                        .characters
                        .get(*character)
                        .map_or_else(|| "Unknown".into(), |c| c.display_name().to_owned());
                    // FUN_00490340: the escape carries the character's code,
                    // for the side it escapes to.
                    msg_log.push(
                        GameMessage::new(
                            current_tick,
                            format!("{name} has escaped captivity!"),
                            MessageCategory::Event,
                        )
                        .with_advice_to(
                            escape_advice(&world, *character),
                            RailAudience::side(*escaped_to_alliance),
                        ),
                    );
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

            // ── Story event screens ──────────────────────────────────────────
            // Show a full-screen BMP overlay for scripted story moments.
            // Only trigger if no overlay is already active (highest-priority event wins).
            if !event_screen_state.is_active() {
                // #R4: resolve Luke's heritage_known for render-layer BMP branching
                let heritage_known = world
                    .characters
                    .values()
                    .find(|c| c.name.contains("Luke"))
                    .is_some_and(|c| c.heritage_known);

                for fired in &fired_events {
                    use rebellion_core::events::{
                        EVT_BOUNTY_ATTACK, EVT_CHARACTER_FORCE, EVT_DAGOBAH_COMPLETED,
                        EVT_FINAL_BATTLE, EVT_FORCE_TRAINING, EVT_LUKE_DAGOBAH,
                    };
                    // Build a human-readable title + description for each story beat.
                    let screen = match fired.event_id {
                        EVT_CHARACTER_FORCE => Some((
                            "The Force Awakens",
                            "A disturbance in the Force... Luke Skywalker's potential has been noticed.",
                        )),
                        EVT_FORCE_TRAINING => Some((
                            "Jedi Training Begins",
                            "Luke Skywalker begins his path in the ways of the Force.",
                        )),
                        EVT_LUKE_DAGOBAH => Some((
                            "The Path to Dagobah",
                            "Luke has departed for the Dagobah system to seek out Yoda.",
                        )),
                        EVT_DAGOBAH_COMPLETED => Some((
                            "Training Complete",
                            "Luke Skywalker has completed his Jedi training on Dagobah.",
                        )),
                        EVT_FINAL_BATTLE => Some((
                            "The Final Battle",
                            "The Emperor has mobilized the full might of the Empire. The fate of the galaxy will be decided now.",
                        )),
                        EVT_BOUNTY_ATTACK => Some((
                            "A Trap is Sprung",
                            "Bounty hunters strike! Han Solo has been captured and frozen in carbonite.",
                        )),
                        0x380 => Some(("Jabba's Demand", "Jabba the Hutt demands the return of Solo. A debt must be paid.")),
                        0x381 => Some(("The Rescue Plan", "Princess Leia has devised a plan to rescue Han Solo from Jabba's palace.")),
                        0x382 => Some(("Into Jabba's Palace", "Alliance agents infiltrate Jabba's fortress. The rescue is underway.")),
                        0x383 => Some(("Jabba Defeated", "Jabba the Hutt is dead. Han Solo is free.")),
                        0x390 => Some(("The Empire Strikes", "Darth Vader has launched a devastating offensive.")),
                        0x391 => Some(("Vader's Ultimatum", "Darth Vader delivers an ultimatum to Alliance command.")),
                        0x393 => Some(("The Emperor Watches", "The Emperor himself turns his attention to the conflict.")),
                        0x394 => Some(("Imperial Intervention", "The Emperor has intervened directly in the war.")),
                        0x397 => Some(("Hunters Dispatched", "Bounty hunters have been unleashed across the galaxy.")),
                        0x398 => Some(("Closing In", "The bounty hunters are closing in on their quarry.")),
                        0x399 => Some(("Alliance Mobilizes", "Mon Mothma has ordered a full mobilization of Alliance forces.")),
                        0x39A => Some(("The Final Stand", "The Alliance makes its final stand against the Empire.")),
                        _ => None,
                    };
                    if let Some((title, description)) = screen {
                        show_event_screen(
                            &mut event_screen_state,
                            fired.event_id,
                            title,
                            description,
                            heritage_known,
                        );
                        break; // One overlay at a time
                    }
                }
            }

            // ── Story cutscene triggers (C1–C8) ─────────────────────────────
            // When a story event fires that maps to a cutscene file (101–108),
            // launch the cutscene. Only the first matching event triggers a
            // cutscene per tick. Story cutscenes return to Galaxy when done.
            for fired in &fired_events {
                if let Some(number) = story_event_to_cutscene(fired.event_id) {
                    let path_str = story_cutscene_path(number);
                    cutscene_player = open_cutscene(
                        Path::new(&path_str),
                        audio_vol.cutscene_volume(),
                        &mut msg_log,
                        current_tick,
                        #[cfg(not(target_arch = "wasm32"))]
                        &mut audio_engine,
                    );
                    if cutscene_player.is_some() {
                        game_mode = GameMode::Cutscene {
                            kind: CutsceneKind::Story(number),
                        };
                        break;
                    }
                }
            }

            // ── AI ──────────────────────────────────────────────────────────
            let ai_actions = AISystem::advance(
                &mut ai_state,
                &world,
                &mfg_state,
                &mission_state,
                &movement_state,
                &tick_events,
                &game_config,
                &research_state,
            );
            apply_ai_actions(
                &ai_actions,
                &mut ai_state,
                &mut mission_state,
                &mut mfg_state,
                &mut movement_state,
                &mut troop_transport_state,
                &mut research_state,
                &mut world,
                &mut msg_log,
                tick_events.last().map_or(0, |e| e.tick),
                #[cfg(not(target_arch = "wasm32"))]
                &mut audio_engine,
                #[cfg(not(target_arch = "wasm32"))]
                &audio_vol,
            );

            // ── The player's agent (FUN_00439a10): once a day, its running
            // modules issue Destination and build orders. Not in dual-AI
            // mode, where an AI plays the player's side.
            if secondary_ai_state.is_none() {
                for _ in &tick_events {
                    let orders = player_agent.advance(
                        &world,
                        &mfg_state,
                        &economy_state,
                        player_faction == MissionFaction::Alliance,
                    );
                    for order in &orders {
                        macroquad::logging::info!("[agent] order={:?}", order);
                    }
                    rebellion_core::agent_automation::apply_orders(&orders, &mut mfg_state);
                }
            }

            // ── Dual AI (second faction) ────────────────────────────────────
            if let Some(ref mut second_ai) = secondary_ai_state {
                let second_actions = AISystem::advance(
                    second_ai,
                    &world,
                    &mfg_state,
                    &mission_state,
                    &movement_state,
                    &tick_events,
                    &game_config,
                    &research_state,
                );
                apply_ai_actions(
                    &second_actions,
                    second_ai,
                    &mut mission_state,
                    &mut mfg_state,
                    &mut movement_state,
                    &mut troop_transport_state,
                    &mut research_state,
                    &mut world,
                    &mut msg_log,
                    tick_events.last().map_or(0, |e| e.tick),
                    #[cfg(not(target_arch = "wasm32"))]
                    &mut audio_engine,
                    #[cfg(not(target_arch = "wasm32"))]
                    &audio_vol,
                );
            }

            // ── Blockade ─────────────────────────────────────────────────────
            let mut blockade_events =
                BlockadeSystem::advance(&mut blockade_state, &world, &tick_events);
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
            for evt in &blockade_events {
                match evt {
                    rebellion_core::blockade::BlockadeEvent::BlockadeStarted { system, tick } => {
                        let name = world
                            .systems
                            .get(*system)
                            .map_or_else(|| "unknown".into(), |s| s.name.clone());
                        for message in blockade_messages(
                            &world,
                            &text_templates,
                            *system,
                            *tick,
                            format!("Blockade established at {name}"),
                        ) {
                            msg_log.push(message);
                        }
                    }
                    rebellion_core::blockade::BlockadeEvent::BlockadeEnded { system, tick } => {
                        let name = world
                            .systems
                            .get(*system)
                            .map_or_else(|| "unknown".into(), |s| s.name.clone());
                        // Notification 7, Blockade Message.
                        msg_log.push(filed(
                            GameMessage::at_system(
                                *tick,
                                format!("Blockade lifted at {name}"),
                                MessageCategory::Combat,
                                *system,
                            ),
                            MessageRail::Conflict,
                            Some(RailAudience::Both),
                        ));
                    }
                    rebellion_core::blockade::BlockadeEvent::TroopDestroyed {
                        system,
                        troop,
                        tick,
                    } => {
                        // The regiment was aboard a fleet leaving the system.
                        troop_transport_state.destroy_embarked(&mut world, *troop);
                        let name = world
                            .systems
                            .get(*system)
                            .map_or_else(|| "unknown".into(), |s| s.name.clone());
                        // Notification 7, Blockade Message.
                        msg_log.push(filed(
                            GameMessage::at_system(
                                *tick,
                                format!("Troops destroyed by blockade at {name}"),
                                MessageCategory::Combat,
                                *system,
                            ),
                            MessageRail::Conflict,
                            Some(RailAudience::Both),
                        ));
                    }
                }
            }

            // ── Repair ──────────────────────────────────────────────────────
            let repair_events = RepairSystem::advance(&mut repair_state, &world, &tick_events);
            for evt in &repair_events {
                if let RepairEvent::ShipRepaired {
                    fleet,
                    ship_index,
                    hull_after,
                    ..
                } = evt
                {
                    if let Some(f) = world.fleets.get_mut(*fleet) {
                        if let Some(ship) = f.capital_ships.get_mut(*ship_index) {
                            ship.hull_current = *hull_after;
                        }
                    }
                }
            }

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
                    | UprisingEvent::Disaster { system, tick, .. } => (*system, *tick),
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
                };
                // port: incidents and disasters have no traced class yet.
                msg_log.push(filed(
                    GameMessage::at_system(tick, text, category, system),
                    MessageRail::PopularSupport,
                    system_audience(&world, system),
                ));
                rebellion_core::uprising::apply_uprising_event(&mut world, evt);
            }

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

            // ── Death Star ───────────────────────────────────────────────────
            let ds_events = DeathStarSystem::advance(&mut death_star_state, &world, &tick_events);
            for evt in &ds_events {
                match evt {
                    rebellion_core::death_star::DeathStarEvent::ConstructionCompleted {
                        system,
                        tick,
                    } => {
                        let name = world
                            .systems
                            .get(*system)
                            .map_or_else(|| "unknown".into(), |s| s.name.clone());
                        // Notification 0x21, Construction Complete.
                        msg_log.push(filed(
                            GameMessage::at_system(
                                *tick,
                                format!("Death Star construction complete at {name}"),
                                MessageCategory::Event,
                                *system,
                            )
                            .with_advice(3),
                            MessageRail::Manufacturing,
                            Some(RailAudience::Empire),
                        ));
                    }
                    rebellion_core::death_star::DeathStarEvent::PlanetDestroyed { .. } => {
                        // Knesset Shamash-Bet Fix D (CRITICAL C3):
                        // `DeathStarSystem::advance()` NEVER emits
                        // `PlanetDestroyed` in the current codebase — only
                        // `DeathStarSystem::fire()` does, and that path goes
                        // through `PanelAction::FireDeathStar` which already
                        // calls `cleanup_destroyed_system` with the effects
                        // buffer for EVT_CHARACTER_KILLED telemetry.
                        //
                        // This branch was pre-existing dead code that marked
                        // `is_destroyed = true` but forgot to call
                        // `cleanup_destroyed_system`, leaking entities under
                        // R11. If a future refactor wires automatic DS
                        // firing into `advance()`, the `unreachable!` will
                        // fire loudly and force the implementer to handle
                        // cleanup + telemetry correctly instead of silently
                        // re-introducing the leak.
                        unreachable!(
                            "DeathStarSystem::advance does not emit PlanetDestroyed — \
                             use DeathStarSystem::fire via PanelAction::FireDeathStar \
                             so cleanup_destroyed_system is called with the effects \
                             out-param (Knesset Shamash-Bet Fix D / CRITICAL C3)"
                        );
                    }
                    rebellion_core::death_star::DeathStarEvent::NearbyWarning { system, tick } => {
                        let name = world
                            .systems
                            .get(*system)
                            .map_or_else(|| "unknown".into(), |s| s.name.clone());
                        msg_log.push(GameMessage::at_system(
                            *tick,
                            format!("Death Star detected near {name}!"),
                            MessageCategory::Event,
                            *system,
                        ));
                    }
                }
            }

            // ── Research ─────────────────────────────────────────────────────
            let research_results =
                ResearchSystem::advance(&mut research_state, &world, &tick_events);
            for result in &research_results {
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
                // Notification 5, Research Report.
                msg_log.push(filed(
                    GameMessage::new(
                        current_tick,
                        format!("{faction_name} {tech_name} tech advanced to level {new_level}"),
                        MessageCategory::Event,
                    )
                    // FUN_00499aa0: research carries code 4.
                    .with_advice(4),
                    MessageRail::Manufacturing,
                    Some(RailAudience::side(*faction_is_alliance)),
                ));
            }
            // Apply research level-ups (advance() is now pure — caller must apply)
            for result in &research_results {
                let rebellion_core::research::ResearchResult::TechUnlocked {
                    faction_is_alliance,
                    tech_type,
                    ..
                } = result;
                if *faction_is_alliance {
                    research_state.alliance.advance(*tech_type);
                } else {
                    research_state.empire.advance(*tech_type);
                }
            }

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

            // ── Victory check ────────────────────────────────────────────────
            if let Some(outcome) = VictorySystem::check(
                &victory_state,
                &world,
                &tick_events,
                campaign_config.victory_conditions,
            ) {
                victory_state.resolved = true;
                let msg = match &outcome {
                    rebellion_core::victory::VictoryOutcome::HqCaptured {
                        winner, loser, ..
                    } => {
                        format!(
                            "{winner:?} captured {loser:?} headquarters! {winner:?} wins!"
                        )
                    }
                    rebellion_core::victory::VictoryOutcome::HqDestroyed { .. } => {
                        "The Empire destroyed the Alliance headquarters and secured its system. Empire wins!"
                            .to_string()
                    }
                    rebellion_core::victory::VictoryOutcome::DeathStarVictory { .. } => {
                        "The Death Star destroyed the Alliance headquarters system. Empire wins!"
                            .to_string()
                    }
                };
                msg_log.push(GameMessage::new(current_tick, msg, MessageCategory::Event));

                game_mode = GameMode::VictoryModal {
                    alliance_won: victory_winner_is_alliance(&outcome),
                };
            }
        }

        // FUN_004eeed0, FUN_00560d50: a rank lapses as soon as its holder
        // is taken, killed, sent on a mission or moved off its post.
        rebellion_core::officer_rank::clear_lapsed_ranks(&mut world);

        // ── Developer commands ───────────────────────────────────────────────

        let mut panel_actions: Vec<PanelAction> = Vec::new();
        // A command script runs one line a frame from the main menu or the
        // galaxy, once the last line's commands have run.
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(script) = command_script.as_mut() {
            for (seq, line) in dev_channel.poll(get_time()) {
                script.push_live(seq, line);
            }
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
                };
                if met {
                    dev_channel.note(&format!("day={} after {frames} frames", clock.tick));
                    dev_wait_until = None;
                } else if *frames >= dev_commands::WAIT_UNTIL_FRAMES {
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
            // A command that runs anywhere does not wait for the galaxy; a
            // malformed one runs at once, to be refused.
            let ready = !held
                && match script.peek().map(dev_commands::parse_line) {
                    Some(Ok(Some(command))) => {
                        galaxy_ready || command.readiness() == dev_commands::Readiness::Anywhere
                    }
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
            match command {
                DevCommand::Capture(name) => {
                    dev_capture = Some(name.unwrap_or_else(|| {
                        format!("capture-{}", dev_channel.next_capture_number())
                    }));
                }
                DevCommand::WaitMs(ms) => dev_hold_until = get_time() + f64::from(ms) / 1000.0,
                DevCommand::WaitFrames(frames) => dev_hold_frames = frames,
                DevCommand::WaitUntil(condition) => dev_wait_until = Some((condition, 0)),
                // Two frames after the last step let the game read it.
                DevCommand::Click { x, y, right, ctrl } => {
                    dev_input.click(x, y, right, ctrl);
                    dev_hold_frames = 2;
                }
                DevCommand::Drag { from, to } => {
                    dev_input.drag(from, to);
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
                    let rows = rebellion_render::message_index::filtered_messages(
                        &msg_log,
                        category,
                        player_is_alliance,
                    );
                    let row = row.map(|row| match row {
                        dev_commands::Row::Number(n) => n,
                        dev_commands::Row::Last => rows.len(),
                    });
                    let Some(mut prepared) = MessageIndexState::prepare_open(category) else {
                        dev_channel
                            .refuse(&format!("unknown Message Index category 0x{category:x}"));
                        continue;
                    };
                    match row {
                        None => {
                            dev_channel.note(&format!("{} rows", rows.len()));
                        }
                        Some(row) => match rows.get(row.saturating_sub(1)) {
                            Some(message) if row > 0 => {
                                let title = message
                                    .display
                                    .as_ref()
                                    .map_or(message.text.as_str(), |d| d.title.as_str());
                                eprintln!("[message_index] viewing row {row}: {title}");
                                dev_channel.note(&format!("row {row} of {}: {title}", rows.len()));
                                prepared = prepared.with_initial_row(row);
                            }
                            None => {
                                dev_channel.refuse(&format!("row {row} of {} rows", rows.len()));
                            }
                            Some(_) => {
                                dev_channel.refuse(&format!("row {row} of {} rows", rows.len()));
                            }
                        },
                    }
                    strategic_windowing.enqueue_message_index_open(prepared);
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
                    match dev_actions::give_death_star(&mut world, &mut death_star_state, fleet) {
                        Ok(detail) => dev_channel.note(&detail),
                        Err(why) => dev_channel.refuse(&why),
                    }
                }
                DevCommand::Status => {
                    let mode = format!("{game_mode:?}");
                    let mode = mode
                        .split(|c: char| !c.is_alphanumeric())
                        .next()
                        .unwrap_or_default()
                        .to_owned();
                    let index = if message_index.is_open() {
                        let name = dev_commands::MESSAGE_CATEGORIES
                            .iter()
                            .find(|(_, command)| *command == message_index.category())
                            .map_or("?", |(name, _)| name);
                        format!("{name}/{:?}", message_index.mode())
                    } else {
                        "closed".to_owned()
                    };
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
                         alliance={} empire={}",
                        clock.tick,
                        clock.speed,
                        clock.pause_requested(),
                        tactical_state.battle_alert_open(),
                        advisor_state.holds_clock(),
                        player_faction,
                        strip(Faction::Alliance),
                        strip(Faction::Empire),
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
                DevCommand::SkipBriefing => {
                    if advisor_state.holds_clock() {
                        advisor_state.skip_tour();
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

        // Freeze the production route only after simulation and scripted
        // commands have had their chance to introduce a modal this frame.
        let desired_window_scene = match game_mode {
            GameMode::Galaxy | GameMode::VictoryModal { .. } => WindowScene::Galaxy,
            GameMode::TacticalCombat | GameMode::GroundCombat => WindowScene::Tactical,
            GameMode::Cutscene { .. }
            | GameMode::MainMenu
            | GameMode::LoadGame
            | GameMode::Credits
            | GameMode::MultiplayerSetup
            | GameMode::GameSetup
            | GameMode::GameOptions { .. } => WindowScene::Suspended,
        };
        let previous_window_scene = strategic_windowing.scene();
        let returning_to_galaxy = previous_window_scene != WindowScene::Galaxy
            && desired_window_scene == WindowScene::Galaxy;
        let tactical_window_phase = tactical_state.windowing_phase();
        let tactical_battle_session = tactical_state.windowing_battle_session();
        let tactical_action_handoff = pending_tactical_window_action.is_some();
        let mut approved_tactical_action = None;
        let mut tactical_window_identity = None;
        let mut tactical_windowing_failed = false;
        if let Some(prepared) = pending_tactical_window_action.clone() {
            let (expected_phase, _) = prepared
                .phases()
                .expect("only supported tactical actions enter the pending slot");
            if tactical_battle_session != Some(prepared.battle_session)
                || tactical_window_phase != expected_phase
            {
                macroquad::logging::warn!(
                    "[windowing] discarded stale tactical action battle={} window={:?}",
                    prepared.battle_session,
                    prepared.window.id
                );
                pending_tactical_window_action = None;
                tactical_windowing_failed = true;
            } else {
                match commit_pending_tactical_window_action(
                    &mut strategic_windowing,
                    &mut pending_tactical_window_action,
                    tactical_battle_session,
                    tactical_window_phase,
                ) {
                    Ok(Some(committed_action)) => {
                        tactical_window_identity = committed_action.phase.window;
                        apply_capture_cancellation_events(
                            &committed_action.phase.events,
                            &mut strategic_input_router,
                            &mut sector_window_state,
                            &mut fleet_window_state,
                            &mut defenses_window_state,
                        );
                        approved_tactical_action = committed_action.approved_action;
                        if approved_tactical_action.is_none() {
                            macroquad::logging::error!(
                                "[windowing] tactical close lacked its exact receipt"
                            );
                            tactical_windowing_failed = true;
                        }
                    }
                    Ok(None) => {
                        macroquad::logging::warn!(
                            "[windowing] discarded stale tactical manager identity battle={} window={:?}",
                            prepared.battle_session,
                            prepared.window.id
                        );
                        pending_tactical_window_action = None;
                        tactical_windowing_failed = true;
                    }
                    Err(error) => {
                        macroquad::logging::error!(
                            "[windowing] tactical close rejected battle={} window={:?} error={error:?}",
                            prepared.battle_session,
                            prepared.window.id
                        );
                        tactical_windowing_failed = true;
                    }
                }
            }
        } else {
            match strategic_windowing.commit_scene_phase_for_battle(
                desired_window_scene,
                tactical_battle_session,
                tactical_window_phase,
            ) {
                Ok(committed) => {
                    tactical_window_identity = committed.window;
                    apply_capture_cancellation_events(
                        &committed.events,
                        &mut strategic_input_router,
                        &mut sector_window_state,
                        &mut fleet_window_state,
                        &mut defenses_window_state,
                    );
                }
                Err(error) => {
                    macroquad::logging::error!(
                        "[windowing] tactical scene commit rejected scene={desired_window_scene:?} phase={:?} error={error:?}",
                        tactical_window_phase
                    );
                    tactical_windowing_failed = true;
                }
            }
        }
        if desired_window_scene != WindowScene::Tactical {
            tactical_state.cancel_window_gestures();
        }
        if returning_to_galaxy && !tactical_windowing_failed {
            // Reconcile the newly active Galaxy membership before publishing
            // its first route. Dormant records remain registered in Tactical,
            // but stale Galaxy modeless content never receives one permissive
            // frame on return.
            let return_candidate = strategic_windowing.begin_frame();
            let mut reports = sector_window_state.content_reports(&return_candidate, &world);
            reports.extend(system_window_state.content_reports(&return_candidate, &world));
            reports.extend(fleet_window_state.content_reports(&return_candidate, &world));
            reports.extend(defenses_window_state.content_reports(&return_candidate, &world));
            reports.extend(missions_window_state.content_reports(&return_candidate, &world));
            reports.extend(fleet_finder_state.content_reports(&return_candidate));
            reports.extend(encyclopedia_surface.content_reports(&return_candidate));
            reports.extend(message_index.content_reports(&return_candidate));
            reports.extend(build_selection_state.content_reports(&return_candidate));
            reports.extend(status_window_state.content_reports(
                &return_candidate,
                StatusSources {
                    world: &world,
                    missions: &mission_state,
                    movement: &movement_state,
                    transport: &troop_transport_state,
                    manufacturing: &mfg_state,
                    today: clock.tick,
                },
            ));
            let reconciliation = strategic_windowing.reconcile(&return_candidate, &reports);
            sector_window_state.remove_content(&reconciliation.removed);
            system_window_state.remove_content(&reconciliation.removed);
            fleet_window_state.remove_content(&reconciliation.removed);
            defenses_window_state.remove_content(&reconciliation.removed);
            missions_window_state.remove_content(&reconciliation.removed);
            fleet_finder_state.remove_content(&reconciliation.removed);
            encyclopedia_surface.remove_content(&reconciliation.removed);
            status_window_state.remove_content(&reconciliation.removed);
            build_selection_state.remove_content(&reconciliation.removed);
            let message_actions = message_index.remove_content(&reconciliation.removed);
            apply_message_index_actions(
                message_actions,
                &mut msg_log,
                &mut clock,
                &mut advice_saved_speed,
                &mut pending_message_sound,
            );
            apply_capture_cancellation_events(
                &reconciliation.events,
                &mut strategic_input_router,
                &mut sector_window_state,
                &mut fleet_window_state,
                &mut defenses_window_state,
            );
            if !reconciliation.invalid_reports.is_empty()
                || !reconciliation.blocking_unpainted.is_empty()
            {
                macroquad::logging::warn!(
                    "[windowing] galaxy return reconcile invalid={:?} blocking={:?}",
                    reconciliation.invalid_reports,
                    reconciliation.blocking_unpainted
                );
            }
        }
        #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
        let original_modal_fixture_open = interface_fixture_request.is_some_and(|request| {
            matches!(
                request.scenario,
                interface_test_fixture::Scenario::EncyclopediaArtwork
                    | interface_test_fixture::Scenario::MessageIndexShell
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
            event_screen: event_screen_state.is_active(),
            briefing: !interface_fixture_active && briefing_lock,
            fixture_setup: false,
            scene_transition: desired_window_scene == WindowScene::Suspended
                || tactical_windowing_failed
                || tactical_action_handoff,
        };
        let legacy_blockers = LegacyInputBlockers {
            strategic_modal: mission_dialog_state.is_open()
                || move_confirmation_state.is_open()
                || scrap_confirmation_state.is_open()
                || troop_finder_state.is_open()
                || personnel_finder_state.is_open()
                || original_modal_fixture_open,
            focused_control: command_palette_open,
            transient_menu: object_menu.is_some()
                || game_speed_ui.menu_anchor.is_some()
                || agent_menu_state.anchor.is_some()
                || cockpit_state.gid_ui.menu_open,
            root_gesture: targeting.is_some(),
        };
        let strategic_frame_boundary =
            strategic_windowing.begin_production_frame(strategic_locks, legacy_blockers);
        defenses_window_state.refresh_release_targets(strategic_frame_boundary.snapshot(), &world);
        missions_window_state.refresh_release_targets(strategic_frame_boundary.snapshot(), &world);
        let fleet_keyboard_owner =
            fleet_window_state.keyboard_owner(strategic_frame_boundary.snapshot());
        let fleet_finder_keyboard_owner =
            fleet_finder_state.keyboard_owner(strategic_frame_boundary.snapshot());
        let encyclopedia_keyboard_owner =
            encyclopedia_surface.keyboard_owner(strategic_frame_boundary.snapshot());
        let message_index_keyboard_owner =
            message_index.keyboard_owner(strategic_frame_boundary.snapshot());
        let status_keyboard_owner =
            status_window_state.keyboard_owner(strategic_frame_boundary.snapshot());
        let build_selection_keyboard_owner =
            build_selection_state.keyboard_owner(strategic_frame_boundary.snapshot());
        let mut legacy_restriction_latch = LegacyInputRestrictionLatch::new(legacy_blockers);
        let strategic_frame_sequence_valid =
            if let Some(next_frame) = strategic_frame_number.checked_add(1) {
                strategic_frame_number = next_frame;
                strategic_input_router
                    .begin_frame(strategic_frame_number, strategic_frame_boundary.snapshot());
                let migrated_input_held = strategic_frame_boundary.migrated_input_held();
                strategic_input_router.set_application_hold(migrated_input_held);
                enqueue_application_hold_capture_cancellation(
                    &mut strategic_windowing,
                    migrated_input_held,
                    sector_window_state.drag_owner(),
                    fleet_window_state.drag_owner(),
                    defenses_window_state.drag_owner(),
                );
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
                    tactical_state.cancel_window_gestures();
                }
                true
            } else {
                macroquad::logging::error!("[windowing] frame_sequence_exhausted");
                strategic_input_router.set_application_hold(true);
                false
            };
        let mut pending_encyclopedia_open: Option<EncyclopediaEntryIntent> = None;

        // ── Rendering (mode-specific) ────────────────────────────────────────

        // The Battle Alert and the results are a window over the live
        // command center (`FUN_0049c0d0`; capture 22-battle-alert-result
        // shows the counters, day and sector windows behind it): draw the
        // command center with its input held, then the window over it.
        let battle_window_underlay =
            game_mode == GameMode::TacticalCombat && tactical_state.over_command_center();
        let tactical_frame = game_mode == GameMode::TacticalCombat;
        let mut tactical_window_input = TacticalWindowInput {
            pointer_screen: mouse_position(),
            primary_down: is_mouse_button_down(MouseButton::Left),
            ..TacticalWindowInput::default()
        };
        let render_mode = if battle_window_underlay {
            GameMode::Galaxy
        } else {
            game_mode.clone()
        };
        match render_mode {
            GameMode::Cutscene { ref kind } => {
                clear_background(BLACK);

                let next_mode = match kind {
                    CutsceneKind::Intro => GameMode::MainMenu,
                    CutsceneKind::TrenchRun(_) => GameMode::TacticalCombat,
                    CutsceneKind::Story(_) => GameMode::Galaxy,
                };

                if let Some(player) = cutscene_player.as_mut() {
                    player.advance(dt);
                    if let Some(frame) = player.current_frame() {
                        draw_fullscreen_texture(frame);
                    }

                    let skip_label = "SPACE / ESC to skip";
                    let metrics = measure_text(skip_label, None, 24, 1.0);
                    draw_text(
                        skip_label,
                        screen_width() - metrics.width - 24.0,
                        screen_height() - 24.0,
                        24.0,
                        Color::new(1.0, 1.0, 1.0, 0.8),
                    );

                    if player.is_finished() {
                        cutscene_player = None;
                        game_mode = next_mode;
                    }
                } else {
                    game_mode = next_mode;
                }
            }

            GameMode::MainMenu => {
                sector_window_state.clear();
                pending_sector_publications.clear();
                pending_committed_sector_actions.clear();
                system_window_state.clear();
                fleet_window_state.clear();
                defenses_window_state.clear();
                missions_window_state.clear();
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
                            credits_state.reset();
                            game_mode = GameMode::Credits;
                            macroquad::logging::info!("[main_menu] destination=credits");
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

            GameMode::Credits => {
                clear_background(BLACK);
                let mut destination_action = None;
                egui_macroquad::ui(|ctx| {
                    destination_action = draw_credits(ctx, &mut credits_state);
                });
                egui_macroquad::draw();
                if destination_action == Some(MenuDestinationAction::Back) {
                    game_mode = GameMode::MainMenu;
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

            GameMode::LoadGame => {
                clear_background(Color::new(0.02, 0.02, 0.06, 1.0));
                egui_macroquad::ui(|ctx| {
                    egui_macroquad::egui::TopBottomPanel::bottom("main_menu_audio_options").show(
                        ctx,
                        |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Audio");
                                draw_audio_controls(ui, &mut audio_vol);
                            });
                        },
                    );
                    if let Some(action) =
                        draw_save_load(ctx, &save_slots, &mut save_load_panel_state)
                    {
                        panel_actions.push(action);
                    }
                });
                egui_macroquad::draw();
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
                    GameOptionsAction::Delete { slot } => {
                        panel_actions.push(PanelAction::DeleteSave { slot })
                    }
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
                            player_faction = faction;

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
                            campaign_config = CampaignConfig::from_seed_options(
                                seed_options,
                                pending_victory_conditions,
                            );
                            pending_victory_conditions = VictoryConditions::Standard;
                            let campaign_loaded = match rebellion_data::load_game_data_with_options(
                                &gdata_path,
                                &seed_options,
                            ) {
                                Ok(mut w) => {
                                    for error in mod_runtime.apply_enabled(&mut w) {
                                        macroquad::logging::error!(
                                            "[campaign] mod_reapply_error={:?}",
                                            error
                                        );
                                    }
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
                                    game_speed_ui = GameSpeedUiState::default();
                                    mfg_state = ManufacturingState::new();
                                    mission_state = MissionState::new();
                                    event_state = EventState::new();
                                    rebellion_core::story_events::define_story_events(
                                        &mut event_state,
                                        &world,
                                    );
                                    movement_state = MovementState::new();
                                    blockade_state = BlockadeState::new();
                                    uprising_state = UprisingState::new();
                                    death_star_state = DeathStarState::default();
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
                                    message_index.reset();
                                    advice_saved_speed = None;
                                    bombardment_panel_state = BombardmentPanelState::default();
                                    encyclopedia_surface.reset();
                                    show_fleets = false;
                                    show_missions = false;
                                    show_bombardment = false;
                                    show_death_star = false;
                                    save_load_panel_state =
                                        rebellion_render::SaveLoadPanelState::default();
                                    save_slots = read_save_slots(&saves_dir);
                                    event_screen_state = EventScreenState::new();
                                    tactical_state = TacticalState::new();
                                    reset_pending_tactical_handoff(
                                        &mut pending_tactical_window_action,
                                        &mut pending_tactical_destination,
                                    );
                                    ground_combat_state = None;

                                    let alliance_hq = world
                                        .systems
                                        .iter()
                                        .find(|(_, system)| {
                                            system.is_headquarters
                                                && system
                                                    .control
                                                    .is_controlled_by(Faction::Alliance)
                                        })
                                        .map(|(key, _)| key);
                                    let empire_hq = world
                                        .systems
                                        .iter()
                                        .find(|(_, system)| {
                                            system.is_headquarters
                                                && system.control.is_controlled_by(Faction::Empire)
                                        })
                                        .map(|(key, _)| key);
                                    victory_state = if let (Some(alliance), Some(empire)) =
                                        (alliance_hq, empire_hq)
                                    {
                                        VictoryState::new(alliance, empire)
                                    } else {
                                        let mut keys = world.systems.keys();
                                        let alliance = keys.next().expect(
                                            "world must have at least 2 systems for victory",
                                        );
                                        let empire = keys.next().expect(
                                            "world must have at least 2 systems for victory",
                                        );
                                        VictoryState::new(alliance, empire)
                                    };
                                    true
                                }
                                Err(e) => {
                                    macroquad::logging::error!(
                                        "Failed to reload game data with seed options: {}",
                                        e
                                    );
                                    false
                                }
                            };

                            if campaign_loaded {
                                drop(strategic_windowing.reset_session());
                                strategic_input_router.reset_session();
                                retained_legacy_input_filter.reset();
                                // Sync cockpit chrome to player faction
                                cockpit_state =
                                    CockpitState::new(if faction == MissionFaction::Alliance {
                                        CockpitFaction::Alliance
                                    } else {
                                        CockpitFaction::Empire
                                    });
                                sector_window_state.clear();
                                pending_sector_publications.clear();
                                pending_committed_sector_actions.clear();
                                system_window_state.clear();
                                fleet_window_state.clear();
                                defenses_window_state.clear();
                                missions_window_state.clear();
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

                                game_mode = GameMode::Galaxy;
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
                map_state.pointer_blocked = cockpit_state
                    .gid_legend_contains(cockpit_layout, pointer)
                    || system_window_state.contains_visible_screen_point(
                        strategic_frame_boundary.snapshot(),
                        cockpit_layout,
                        pointer,
                    )
                    || fleet_window_state.contains_visible_screen_point(
                        strategic_frame_boundary.snapshot(),
                        cockpit_layout,
                        pointer,
                    )
                    || defenses_window_state.contains_visible_screen_point(
                        strategic_frame_boundary.snapshot(),
                        cockpit_layout,
                        pointer,
                    )
                    || missions_window_state.contains_visible_screen_point(
                        strategic_frame_boundary.snapshot(),
                        cockpit_layout,
                        pointer,
                    )
                    || fleet_window_state.is_dragging()
                    || defenses_window_state.is_dragging()
                    || mission_dialog_state.contains_screen_point(cockpit_layout, pointer)
                    || move_confirmation_state.contains_screen_point(cockpit_layout, pointer)
                    || scrap_confirmation_state.contains_screen_point(cockpit_layout, pointer)
                    || fleet_finder_state.contains_screen_point(cockpit_layout, pointer)
                    || troop_finder_state.contains_screen_point(cockpit_layout, pointer)
                    || personnel_finder_state.contains_screen_point(cockpit_layout, pointer)
                    || cockpit_state.gid_ui.menu_open
                    || original_modal_fixture_open
                    || game_speed_ui.menu_anchor.is_some()
                    || object_menu.is_some()
                    || pause_alert_contains_screen_point(&clock, cockpit_layout, pointer)
                    || event_screen_state.is_active()
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
                let mut pending_fleet_finder_action = None;
                let mut message_choice = None;
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
                        batch = collect_strategic_pointer_batch(
                            strategic_frame_number,
                            &mut strategic_input_router,
                            &strategic_raw_input_events.pointers,
                            strategic_frame_boundary.snapshot(),
                            cockpit_layout,
                            &message_index,
                            &status_window_state,
                            &build_selection_state,
                            |point| {
                                let top_layer =
                                    ctx.layer_id_at(egui_macroquad::egui::pos2(point.0, point.1));
                                let mut hits = sector_window_state.hit_window_ids(
                                    strategic_frame_boundary.snapshot(),
                                    cockpit_layout,
                                    point,
                                    top_layer,
                                );
                                hits.extend(system_window_state.hit_window_ids(
                                    strategic_frame_boundary.snapshot(),
                                    cockpit_layout,
                                    point,
                                    top_layer,
                                ));
                                hits.extend(fleet_window_state.hit_window_ids(
                                    strategic_frame_boundary.snapshot(),
                                    cockpit_layout,
                                    point,
                                    top_layer,
                                ));
                                hits.extend(defenses_window_state.hit_window_ids(
                                    strategic_frame_boundary.snapshot(),
                                    cockpit_layout,
                                    point,
                                    top_layer,
                                ));
                                hits.extend(missions_window_state.hit_window_ids(
                                    strategic_frame_boundary.snapshot(),
                                    cockpit_layout,
                                    point,
                                    top_layer,
                                ));
                                if let Some(window) = fleet_finder_state.hit_window(
                                    strategic_frame_boundary.snapshot(),
                                    cockpit_layout,
                                    point,
                                ) {
                                    hits.insert(window);
                                }
                                if let Some(window) = encyclopedia_surface.hit_window(
                                    strategic_frame_boundary.snapshot(),
                                    screen_width(),
                                    screen_height(),
                                    point,
                                ) {
                                    hits.insert(window);
                                }
                                if let Some(window) = tactical_window_identity.filter(|_| {
                                    tactical_state.windowing_contains_screen_point(
                                        screen_width(),
                                        screen_height(),
                                        point,
                                    )
                                }) {
                                    hits.insert(window.id);
                                }
                                hits
                            },
                        );
                    });
                    if let Some(window) = tactical_window_identity {
                        tactical_window_input = route_tactical_window_input(
                            &batch,
                            &mut strategic_input_router,
                            window.id,
                            mouse_position(),
                            is_mouse_button_down(MouseButton::Left),
                            mouse_wheel().1,
                        );
                    }
                    batch
                } else {
                    StrategicPointerBatch::default()
                };
                let strategic_pointer_routes = strategic_pointer_events.router_events();
                let sector_pointer_routes = strategic_pointer_events.sector_events();
                latch_map_pointer_from_strategic_batch(
                    &mut map_state,
                    &strategic_pointer_events,
                    &strategic_input_router,
                );
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
                    let retained_legacy_input = retained_legacy_input_filter.install(
                        ctx,
                        &strategic_pointer_events,
                        &strategic_input_router,
                        strategic_frame_boundary.migrated_modal_active(),
                    );
                    let strategic_input_enabled = strategic_root_pointer_enabled;
                    let strategic_owner_input_enabled = strategic_owner_pointer_enabled;
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
                        if let Some(category) =
                            rebellion_render::message_index::opening_category(command)
                        {
                            strategic_windowing.enqueue_message_index_open(
                                MessageIndexState::prepare_open(category)
                                    .expect("opening_category returns a Message Index category"),
                            );
                            legacy_restriction_latch.note_start();
                            macroquad::logging::info!(
                                "[interface] command=0x{:x} destination=message_index category=0x{:x} status=queued_shared",
                                command,
                                category
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
                    if show_bombardment {
                        if let Some(action) = draw_bombardment(
                            ctx,
                            &world,
                            &mut bombardment_panel_state,
                            player_faction,
                        ) {
                            panel_actions.push(action);
                        }
                    }
                    if show_death_star {
                        if let Some(action) =
                            draw_death_star(ctx, &world, &death_star_state, player_faction)
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
                        let commanded = std::mem::take(&mut pending_committed_sector_actions);
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
                            let system = match command {
                                InterfaceCommand::StartGame(_) => {
                                    #[cfg(not(target_arch = "wasm32"))]
                                    dev_channel.refuse("the app starts a game");
                                    #[cfg(target_arch = "wasm32")]
                                    eprintln!("[dev-command] refused: the app starts a game");
                                    continue;
                                }
                                InterfaceCommand::OpenSector(system)
                                | InterfaceCommand::OpenQuadrantWindow { system, .. }
                                | InterfaceCommand::OpenIconMenu { system, .. } => system,
                            };
                            enqueue_sector_window(
                                &mut strategic_windowing,
                                &sector_window_state,
                                &world,
                                system,
                                cockpit_state.faction,
                                &mut pending_sector_publications,
                                Some(SectorDependentIntent::Interface(command)),
                            );
                        }
                        commanded
                    };

                    for action in with_retained_input(ctx, retained_window_input, || {
                        draw_managed_sector_windows(
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
                            strategic_frame_boundary.snapshot(),
                            &sector_pointer_routes,
                            &mut strategic_input_router,
                        )
                    })
                    .into_iter()
                    .chain(commanded_sector_actions)
                    {
                        if enqueue_sector_manager_action(&mut strategic_windowing, action) {
                            continue;
                        }
                        match action {
                            SectorWindowAction::Activate(_)
                            | SectorWindowAction::CapturePointer(_)
                            | SectorWindowAction::ReleasePointer(_)
                            | SectorWindowAction::Close(_) => unreachable!(
                                "manager-owned Sector actions are consumed before gameplay"
                            ),
                            SectorWindowAction::SelectSystem(system) => {
                                map_state.selected_system = Some(system);
                            }
                            SectorWindowAction::OpenSystemWindow {
                                system,
                                logical_position,
                            } => {
                                map_state.selected_system = Some(system);
                                enqueue_system_window(
                                    &mut strategic_windowing,
                                    &system_window_state,
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
                                enqueue_defenses_window(
                                    &mut strategic_windowing,
                                    &defenses_window_state,
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
                                enqueue_missions_window(
                                    &mut strategic_windowing,
                                    &missions_window_state,
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
                                    &mfg_state,
                                    &research_state,
                                    player_faction,
                                );
                                indexed_status_menu_witness =
                                    capture_indexed_status_menu_witness(&world, selection);
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
                                            tick: clock.tick,
                                        },
                                        &fleets,
                                        confirmed,
                                        target,
                                        &mut msg_log,
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
                        {
                            let serial = system_legacy_bridge_serial;
                            #[cfg(feature = "interface-test-fixtures")]
                            let serial = serial.max(system_window_state.focus_serial());
                            serial
                        },
                        {
                            let serial = fleet_legacy_bridge_serial;
                            #[cfg(feature = "interface-test-fixtures")]
                            let serial = serial.max(fleet_window_state.fixture_focus_serial());
                            serial
                        },
                        {
                            let serial = defenses_legacy_bridge_serial;
                            #[cfg(feature = "interface-test-fixtures")]
                            let serial = serial.max(defenses_window_state.focus_serial());
                            visible_legacy_bridge_serial(
                                serial,
                                defenses_window_state.has_visible_content(
                                    strategic_frame_boundary.snapshot(),
                                    &world,
                                ),
                            )
                        },
                        visible_legacy_bridge_serial(
                            missions_legacy_bridge_serial,
                            missions_window_state
                                .has_visible_content(strategic_frame_boundary.snapshot(), &world),
                        ),
                    );
                    for action in with_retained_input(ctx, retained_window_input, || {
                        draw_managed_system_windows(
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
                            strategic_frame_boundary.snapshot(),
                            &strategic_pointer_routes,
                            &mut strategic_input_router,
                            galaxy_front == GalaxyFront::System,
                        )
                    }) {
                        match action {
                            SystemWindowAction::FocusSector(system) => {
                                enqueue_sector_window(
                                    &mut strategic_windowing,
                                    &sector_window_state,
                                    &world,
                                    system,
                                    cockpit_state.faction,
                                    &mut pending_sector_publications,
                                    None,
                                );
                                map_state.selected_system = Some(system);
                            }
                            SystemWindowAction::SelectSystem(system) => {
                                map_state.selected_system = Some(system);
                            }
                            SystemWindowAction::Activate(window) => strategic_windowing
                                .enqueue(rebellion_windowing::WindowCommand::Activate(window)),
                            SystemWindowAction::Close(window) => strategic_windowing
                                .enqueue(rebellion_windowing::WindowCommand::Close(window)),
                            SystemWindowAction::MinimizeManaged {
                                window,
                                system,
                                logical_position,
                            } => strategic_windowing.enqueue_system_minimize(
                                window,
                                system,
                                logical_position,
                            ),
                            SystemWindowAction::EvictManagedWindow(window) => strategic_windowing
                                .enqueue(rebellion_windowing::WindowCommand::Close(window)),
                            SystemWindowAction::RestoreSystemWindow {
                                id,
                                system: _,
                                logical_position: _,
                            } => strategic_windowing.enqueue_system_restore(id),
                            SystemWindowAction::RestoreFleetWindow {
                                id,
                                system: _,
                                logical_position: _,
                            } => strategic_windowing.enqueue_fleet_restore(id),
                            SystemWindowAction::RestoreDefensesWindow {
                                system,
                                logical_position: _,
                            } => {
                                if let Some(window) = exact_minimized_defenses_restore(
                                    strategic_frame_boundary.snapshot(),
                                    defenses_window_state.window_id(system),
                                    rebellion_render::windowing_policy::system_subject(
                                        &world, system,
                                    ),
                                ) {
                                    strategic_windowing.enqueue_defenses_restore(window);
                                }
                            }
                            SystemWindowAction::RestoreMissionsWindow {
                                system,
                                logical_position: _,
                            } => {
                                if let Some(window) = exact_minimized_missions_restore(
                                    strategic_frame_boundary.snapshot(),
                                    missions_window_state.window_id(system),
                                    rebellion_render::windowing_policy::system_subject(
                                        &world, system,
                                    ),
                                ) {
                                    strategic_windowing.enqueue_missions_restore(window);
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
                                    &mfg_state,
                                    &research_state,
                                    player_faction,
                                );
                                indexed_status_menu_witness =
                                    capture_indexed_status_menu_witness(&world, selection);
                                object_menu = Some(ObjectMenuState::new(selection, gates, point));
                                legacy_restriction_latch.note_start();
                            }
                        }
                    }

                    for action in draw_managed_fleet_windows(
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
                    ) {
                        match action {
                            FleetWindowAction::OpenSector(system) => {
                                enqueue_sector_window(
                                    &mut strategic_windowing,
                                    &sector_window_state,
                                    &world,
                                    system,
                                    cockpit_state.faction,
                                    &mut pending_sector_publications,
                                    None,
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
                                    &mfg_state,
                                    &research_state,
                                    player_faction,
                                );
                                indexed_status_menu_witness =
                                    capture_indexed_status_menu_witness(&world, Some(selection));
                                object_menu =
                                    Some(ObjectMenuState::new(Some(selection), gates, point));
                                legacy_restriction_latch.note_start();
                            }
                            // FUN_00422ce0, window type 4: the fleet moves
                            // with 0x201 against what +0x70 gives under the
                            // point. port: Ctrl's 0x202 is not ported here.
                            FleetWindowAction::DragFleet { fleet, point } => {
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
                                            tick: clock.tick,
                                        },
                                        &[fleet],
                                        false,
                                        target,
                                        &mut msg_log,
                                        &mut move_confirmation_state,
                                        &mut panel_actions,
                                    ) {
                                        legacy_restriction_latch.note_start();
                                    }
                                }
                            }
                            // FUN_00422ce0: the ship moves with 0x201 against
                            // the window under the point
                            // (ghidra/notes/fleet-join-split.md).
                            FleetWindowAction::DragShip {
                                fleet,
                                index,
                                roster,
                                point,
                            } => {
                                let windows = ReleaseWindows {
                                    sector: &sector_window_state,
                                    system: &system_window_state,
                                    fleet: &fleet_window_state,
                                    defenses: &defenses_window_state,
                                    missions: &missions_window_state,
                                };
                                match release_destination(
                                    ctx,
                                    &world,
                                    &movement_state,
                                    fog_state,
                                    cockpit_layout,
                                    windows,
                                    point,
                                ) {
                                    Some(ReleaseTarget::Fleet { fleet: target, .. }) => {
                                        panel_actions.push(PanelAction::JoinFleet {
                                            mover: FleetMover::Ships {
                                                fleet,
                                                ships: vec![index],
                                                roster,
                                            },
                                            target,
                                        });
                                    }
                                    Some(ReleaseTarget::System(system)) => {
                                        panel_actions.push(PanelAction::MoveShips {
                                            fleet,
                                            ships: vec![index],
                                            roster,
                                            system,
                                        });
                                    }
                                    None => {}
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
                                        tick: clock.tick,
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
                            FleetWindowAction::DragRegiment { troop, point } => {
                                let windows = ReleaseWindows {
                                    sector: &sector_window_state,
                                    system: &system_window_state,
                                    fleet: &fleet_window_state,
                                    defenses: &defenses_window_state,
                                    missions: &missions_window_state,
                                };
                                let target = match release_destination(
                                    ctx,
                                    &world,
                                    &movement_state,
                                    fog_state,
                                    cockpit_layout,
                                    windows,
                                    point,
                                ) {
                                    Some(ReleaseTarget::System(system)) => {
                                        RegimentTarget::System(system)
                                    }
                                    Some(ReleaseTarget::Fleet { fleet, .. }) => {
                                        RegimentTarget::Fleet(fleet)
                                    }
                                    None => continue,
                                };
                                panel_actions.push(PanelAction::MoveRegiment { troop, target });
                            }
                            FleetWindowAction::DragSquadron {
                                fleet,
                                index,
                                point,
                            } => {
                                let windows = ReleaseWindows {
                                    sector: &sector_window_state,
                                    system: &system_window_state,
                                    fleet: &fleet_window_state,
                                    defenses: &defenses_window_state,
                                    missions: &missions_window_state,
                                };
                                let target = match release_destination(
                                    ctx,
                                    &world,
                                    &movement_state,
                                    fog_state,
                                    cockpit_layout,
                                    windows,
                                    point,
                                ) {
                                    Some(ReleaseTarget::System(system)) => {
                                        RegimentTarget::System(system)
                                    }
                                    Some(ReleaseTarget::Fleet { fleet, .. }) => {
                                        RegimentTarget::Fleet(fleet)
                                    }
                                    None => continue,
                                };
                                panel_actions.push(PanelAction::MoveSquadron {
                                    source: SquadronSource::Fleet { fleet, index },
                                    target,
                                });
                            }
                        }
                    }

                    for action in with_retained_input(ctx, retained_window_input, || {
                        draw_managed_defenses_windows(
                            ctx,
                            &world,
                            fog_state,
                            &mission_state,
                            &economy_state,
                            &mut defenses_window_state,
                            cockpit_state.faction,
                            cockpit_layout,
                            &mut bmp_cache,
                            strategic_frame_boundary.snapshot(),
                            &strategic_pointer_routes,
                            &mut strategic_input_router,
                            galaxy_front == GalaxyFront::Defenses,
                        )
                    }) {
                        match action {
                            DefensesWindowAction::OpenSector(system) => {
                                enqueue_sector_window(
                                    &mut strategic_windowing,
                                    &sector_window_state,
                                    &world,
                                    system,
                                    cockpit_state.faction,
                                    &mut pending_sector_publications,
                                    None,
                                );
                                map_state.selected_system = Some(system);
                            }
                            DefensesWindowAction::SelectSystem(system) => {
                                map_state.selected_system = Some(system);
                            }
                            DefensesWindowAction::Activate(window) => strategic_windowing
                                .enqueue(rebellion_windowing::WindowCommand::Activate(window)),
                            DefensesWindowAction::CapturePointer(window) => strategic_windowing
                                .enqueue(rebellion_windowing::WindowCommand::CapturePointer(
                                    window,
                                )),
                            DefensesWindowAction::ReleasePointer(window) => strategic_windowing
                                .enqueue(rebellion_windowing::WindowCommand::ReleasePointer(
                                    window,
                                )),
                            DefensesWindowAction::Close(window) => strategic_windowing
                                .enqueue(rebellion_windowing::WindowCommand::Close(window)),
                            DefensesWindowAction::Minimize {
                                system,
                                logical_position,
                            } => {
                                if let Some(window) = system_window_state
                                    .minimize_defenses_window(system, logical_position)
                                {
                                    strategic_windowing
                                        .enqueue(rebellion_windowing::WindowCommand::Close(window));
                                }
                            }
                            DefensesWindowAction::MinimizeManaged {
                                window,
                                system,
                                logical_position,
                            } => strategic_windowing.enqueue_defenses_minimize(
                                window,
                                system,
                                logical_position,
                            ),
                            DefensesWindowAction::OpenObjectMenu {
                                selection, point, ..
                            } => {
                                let gates = order_gates(
                                    selection,
                                    &world,
                                    &mission_state,
                                    &movement_state,
                                    &troop_transport_state,
                                    &mfg_state,
                                    &research_state,
                                    player_faction,
                                );
                                indexed_status_menu_witness =
                                    capture_indexed_status_menu_witness(&world, selection);
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
                                        tick: clock.tick,
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
                                point,
                            } => {
                                let windows = ReleaseWindows {
                                    sector: &sector_window_state,
                                    system: &system_window_state,
                                    fleet: &fleet_window_state,
                                    defenses: &defenses_window_state,
                                    missions: &missions_window_state,
                                };
                                let target = match release_destination(
                                    ctx,
                                    &world,
                                    &movement_state,
                                    fog_state,
                                    cockpit_layout,
                                    windows,
                                    point,
                                ) {
                                    Some(ReleaseTarget::System(system)) => {
                                        RegimentTarget::System(system)
                                    }
                                    Some(ReleaseTarget::Fleet { fleet, .. }) => {
                                        RegimentTarget::Fleet(fleet)
                                    }
                                    None => continue,
                                };
                                panel_actions.push(PanelAction::MoveSquadron {
                                    source: SquadronSource::System {
                                        system,
                                        class,
                                        is_alliance,
                                    },
                                    target,
                                });
                            }
                        }
                    }

                    for action in with_retained_input(ctx, retained_window_input, || {
                        draw_managed_missions_windows(
                            ctx,
                            &world,
                            fog_state,
                            &mission_state,
                            &mut missions_window_state,
                            cockpit_state.faction,
                            cockpit_layout,
                            &mut bmp_cache,
                            strategic_frame_boundary.snapshot(),
                            &strategic_pointer_routes,
                            &mut strategic_input_router,
                            galaxy_front == GalaxyFront::Missions,
                        )
                    }) {
                        match action {
                            MissionsWindowAction::OpenSector(system) => {
                                enqueue_sector_window(
                                    &mut strategic_windowing,
                                    &sector_window_state,
                                    &world,
                                    system,
                                    cockpit_state.faction,
                                    &mut pending_sector_publications,
                                    None,
                                );
                                map_state.selected_system = Some(system);
                            }
                            MissionsWindowAction::SelectSystem(system) => {
                                map_state.selected_system = Some(system);
                            }
                            MissionsWindowAction::Activate(window) => strategic_windowing
                                .enqueue(rebellion_windowing::WindowCommand::Activate(window)),
                            MissionsWindowAction::Close(window) => strategic_windowing
                                .enqueue(rebellion_windowing::WindowCommand::Close(window)),
                            MissionsWindowAction::MinimizeManaged {
                                window,
                                system,
                                logical_position,
                            } => strategic_windowing.enqueue_missions_minimize(
                                window,
                                system,
                                logical_position,
                            ),
                            MissionsWindowAction::Minimize {
                                system,
                                logical_position,
                            } => {
                                if let Some(window) = system_window_state
                                    .minimize_missions_window(system, logical_position)
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
                        strategic_owner_input_enabled,
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
                                pending_encyclopedia_open.get_or_insert(intent);
                                legacy_restriction_latch.note_start();
                                macroquad::logging::info!(
                                    "[encyclopedia] production_route status=queued origin=contextual caller=FUN_00486fb0_event_0x100 requested={:#010x}",
                                    requested_object_id
                                );
                            } else {
                                macroquad::logging::warn!(
                                    "[encyclopedia] production_route unavailable origin=contextual reason=no_session"
                                );
                            }
                        }
                        // FUN_00486fb0 0x103 → FUN_0042a440: the Status
                        // window (type 0x1a) for the selection.
                        Some((ObjectMenuCommand::Status, Some(selection))) => {
                            let prepared = status_object_if_menu_current(
                                &world,
                                selection,
                                indexed_status_menu_witness,
                            )
                            .and_then(|object| {
                                strategic_windowing.next_context_subject().ok().and_then(
                                    |subject| {
                                        StatusWindowState::prepare_open(
                                            StatusSources {
                                                world: &world,
                                                missions: &mission_state,
                                                movement: &movement_state,
                                                transport: &troop_transport_state,
                                                manufacturing: &mfg_state,
                                                today: clock.tick,
                                            },
                                            object,
                                            subject,
                                        )
                                        .map(|prepared| (object, prepared))
                                    },
                                )
                            });
                            if let Some((object, prepared)) = prepared {
                                strategic_windowing
                                    .enqueue_status_open(prepared, status_window_state.window_id());
                                legacy_restriction_latch.note_start();
                                let rect =
                                    rebellion_render::status_window::window_rect(cockpit_layout);
                                macroquad::logging::info!(
                                    "[interface] command=0x103 destination=status_window status=queued object={:?} rect={},{},{},{}",
                                    object,
                                    rect.min.x,
                                    rect.min.y,
                                    rect.width(),
                                    rect.height()
                                );
                            } else {
                                macroquad::logging::warn!(
                                    "[interface] command=0x103 destination=status_window status=rejected selection={:?}",
                                    selection
                                );
                            }
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
                        Some((ObjectMenuCommand::Move, Some(MenuObject::Troop(troop)))) => {
                            targeting = Some(Targeting::new(TargetOrder::TroopMove { troop }));
                            legacy_restriction_latch.note_start();
                        }
                        Some((
                            ObjectMenuCommand::Move,
                            Some(MenuObject::Fighter { fleet, index }),
                        )) => {
                            targeting = Some(Targeting::new(TargetOrder::SquadronMove {
                                source: SquadronSource::Fleet { fleet, index },
                            }));
                            legacy_restriction_latch.note_start();
                        }
                        Some((
                            ObjectMenuCommand::Move,
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
                            ObjectMenuCommand::Move,
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
                            if let Some(prepared) =
                                build_selection_state.prepare_open(system, area, is_alliance, kinds)
                            {
                                strategic_windowing.enqueue_build_selection_open(prepared);
                                legacy_restriction_latch.note_start();
                                macroquad::logging::info!(
                                    "[interface] command=0x21x destination=build_selection status=queued area={area:?} system={}",
                                    world.systems.get(system).map_or("", |value| value.name.as_str())
                                );
                            }
                        }
                        // Stop (0x213) runs at once: the band's area drops
                        // what it was building (manual p. 84).
                        Some((
                            ObjectMenuCommand::Stop,
                            Some(MenuObject::Producer { system, area }),
                        )) => panel_actions.push(PanelAction::StopProduction { system, area }),
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
                                    if let Some(point) = visible
                                        .then(|| fleet_window_state.window_logical_position(system))
                                        .flatten()
                                    {
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
                                    } else {
                                        enqueue_sector_window(
                                            &mut strategic_windowing,
                                            &sector_window_state,
                                            &world,
                                            system,
                                            cockpit_state.faction,
                                            &mut pending_sector_publications,
                                            Some(SectorDependentIntent::Fleet {
                                                system,
                                                selected: Some(entry),
                                                rename: true,
                                            }),
                                        );
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
                        }) => panel_actions.push(PanelAction::DispatchMission {
                            kind,
                            faction,
                            team,
                            decoys,
                            target,
                            target_character: None,
                            tick: clock.tick,
                        }),
                        Some(MissionDialogAction::Encyclopedia) => {
                            if encyclopedia_session_store.current().is_some() {
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
                                pending_encyclopedia_open.get_or_insert(intent);
                                legacy_restriction_latch.note_start();
                                macroquad::logging::info!(
                                    "[encyclopedia] production_route status=queued origin=contextual caller=FUN_0046c3c0_command_0x67 requested=unresolved"
                                );
                            } else {
                                macroquad::logging::warn!(
                                    "[encyclopedia] production_route unavailable origin=mission_dialog reason=no_session"
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
                    if let Some((system, target)) = troop_choice.or(personnel_choice) {
                        map_state.selected_system = Some(system);
                        enqueue_sector_window(
                            &mut strategic_windowing,
                            &sector_window_state,
                            &world,
                            system,
                            cockpit_state.faction,
                            &mut pending_sector_publications,
                            match target {
                                FinderTarget::Sector => None,
                                FinderTarget::Defenses => {
                                    Some(SectorDependentIntent::Defenses { system })
                                }
                                FinderTarget::Missions(member) => {
                                    Some(SectorDependentIntent::Missions { system, member })
                                }
                                FinderTarget::Fleet(fleet) => Some(SectorDependentIntent::Fleet {
                                    system,
                                    selected: Some(FleetWindowEntry::Fleet(fleet)),
                                    rename: false,
                                }),
                            },
                        );
                    }

                    // FUN_0044f5e0: the checkmark resubmits with force 1,
                    // which validates again and departs without asking.
                    if let Some(MoveConfirmationAction::Confirm {
                        fleets,
                        members,
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
                        panel_actions.extend(fleets.into_iter().map(|fleet| match join {
                            Some(target) => PanelAction::JoinFleet {
                                mover: FleetMover::Fleet(fleet),
                                target,
                            },
                            None => PanelAction::DispatchFleet {
                                fleet,
                                destination,
                                troops: Vec::new(),
                            },
                        }));
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
                        let refund = rebellion_core::scrap::scrap_all(
                            &mut world,
                            &mut stockpile_state,
                            &mut troop_transport_state,
                            targets.iter().copied(),
                        );
                        macroquad::logging::info!(
                            "[interface] command=0x200 status=scrapped objects={} refund={}",
                            targets.len(),
                            refund
                        );
                    }

                    if targeting.is_some()
                        && strategic_frame_boundary.legacy_surface_input_allowed()
                        && strategic_root_keys.contains(KeyCode::Escape)
                    {
                        legacy_keyboard_handled = true;
                        targeting = None;
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
                        if strategic_owner_input_enabled
                            && targeting_held
                            && !strategic_pointer_events.blocks_pointer_phase(
                                &strategic_input_router,
                                PointerPhase::Release(PointerButtonId::Primary),
                            )
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
                                    );
                                    macroquad::logging::info!(
                                        "[order] mission dialog {:?} at {:?}: {:?}",
                                        team,
                                        system,
                                        kinds
                                    );
                                    mission_dialog_state.open(player_faction, system, team, kinds);
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
                                            tick: clock.tick,
                                        },
                                        &fleets,
                                        confirmed,
                                        target,
                                        &mut msg_log,
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
                                        tick: clock.tick,
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
                                        },
                                    target: ReleaseTarget::Fleet { fleet: target, .. },
                                }) => panel_actions.push(PanelAction::JoinFleet {
                                    mover: FleetMover::Ships {
                                        fleet,
                                        ships,
                                        roster,
                                    },
                                    target,
                                }),
                                Some(TargetingEnd::Target {
                                    order:
                                        TargetOrder::ShipMove {
                                            fleet,
                                            ships,
                                            roster,
                                        },
                                    target: ReleaseTarget::System(system),
                                }) => panel_actions.push(PanelAction::MoveShips {
                                    fleet,
                                    ships,
                                    roster,
                                    system,
                                }),
                                // A regiment's 0x201 against the release
                                // window's +0x70 (FUN_004a3130 for a Fleet
                                // window), boarding, landing or travelling.
                                Some(TargetingEnd::Target {
                                    order: TargetOrder::TroopMove { troop },
                                    target: ReleaseTarget::Fleet { fleet, .. },
                                }) => panel_actions.push(PanelAction::MoveRegiment {
                                    troop,
                                    target: RegimentTarget::Fleet(fleet),
                                }),
                                Some(TargetingEnd::Target {
                                    order: TargetOrder::TroopMove { troop },
                                    target: ReleaseTarget::System(system),
                                }) => panel_actions.push(PanelAction::MoveRegiment {
                                    troop,
                                    target: RegimentTarget::System(system),
                                }),
                                Some(TargetingEnd::Target {
                                    order: TargetOrder::SquadronMove { source },
                                    target: ReleaseTarget::Fleet { fleet, .. },
                                }) => panel_actions.push(PanelAction::MoveSquadron {
                                    source,
                                    target: RegimentTarget::Fleet(fleet),
                                }),
                                Some(TargetingEnd::Target {
                                    order: TargetOrder::SquadronMove { source },
                                    target: ReleaseTarget::System(system),
                                }) => panel_actions.push(PanelAction::MoveSquadron {
                                    source,
                                    target: RegimentTarget::System(system),
                                }),
                                // FUN_00512700 kind 4: the order's team is
                                // every production area of the system, or
                                // a band's own area.
                                Some(TargetingEnd::Target {
                                    order: TargetOrder::Destination { system, area },
                                    target:
                                        ReleaseTarget::System(destination)
                                        | ReleaseTarget::Fleet {
                                            system: destination,
                                            ..
                                        },
                                }) => panel_actions.push(PanelAction::SetDestination {
                                    system,
                                    area,
                                    destination,
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

                    // Story event screen overlay (top-most, including the advisor). Preserve
                    // the pre-draw state so the click that dismisses an event cannot also
                    // activate a cockpit control underneath it in the same frame.
                    let event_screen_was_active = event_screen_state.is_active();
                    draw_event_screen(ctx, &mut event_screen_state, &mut bmp_cache);

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
                    pending_cockpit_button = (strategic_input_enabled
                        && !event_screen_was_active
                        && !event_screen_state.is_active())
                    .then(|| handle_cockpit_egui_input(ctx, &mut cockpit_state, &mut bmp_cache))
                    .flatten();
                    // Future manager-owned content paints after this restore;
                    // every retained legacy surface above was input-muted.
                    if let Some(saved) = retained_legacy_input {
                        restore_egui_input(ctx, saved);
                    }
                    // Manager-owned surfaces paint in the manager's immutable
                    // bottom-to-top order. Lower modals still paint, but their
                    // input is muted; only the active modal receives this
                    // frame's egui events and supplies the outside barrier.
                    let mut encyclopedia_close_queued = false;
                    let mut message_index_close_queued = false;
                    let mut status_close_queued = false;
                    let managed_surfaces = managed_strategic_surface_plan(
                        strategic_frame_boundary.snapshot(),
                        encyclopedia_surface.window_id(),
                        fleet_finder_state.window_id(),
                        message_index.window_id(),
                        status_window_state.window_id(),
                        build_selection_state.window_id(),
                    );
                    for (surface, allows_input) in managed_surfaces {
                        match surface {
                            ManagedStrategicSurface::Encyclopedia(window) => {
                                let focused_control = strategic_frame_boundary
                                    .snapshot()
                                    .focused()
                                    .filter(|focus| focus.window == window)
                                    .and_then(|focus| focus.control);
                                with_retained_input(ctx, allows_input, || {
                                    if let Some(session) = encyclopedia_session_store.current() {
                                        match encyclopedia_surface.draw(
                                            ctx,
                                            &mut bmp_cache,
                                            &session,
                                            &encyclopedia_hd,
                                            focused_control,
                                        ) {
                                            Ok(Some(route)) => {
                                                if let Some((window, _)) =
                                                    encyclopedia_surface.request_close()
                                                {
                                                    strategic_windowing.enqueue(
                                                        rebellion_windowing::WindowCommand::Close(
                                                            window,
                                                        ),
                                                    );
                                                    encyclopedia_close_queued = true;
                                                    log_encyclopedia_return(route);
                                                }
                                                encyclopedia_surface.mark_paintable();
                                            }
                                            Ok(None) => encyclopedia_surface.mark_paintable(),
                                            Err(error) => {
                                                encyclopedia_surface.mark_unpaintable();
                                                macroquad::logging::error!(
                                                    "[encyclopedia] production_route draw_failed error={}",
                                                    error
                                                );
                                            }
                                        }
                                    } else {
                                        encyclopedia_surface.mark_unpaintable();
                                        macroquad::logging::warn!(
                                            "[encyclopedia] production_route unavailable reason=no_session"
                                        );
                                    }
                                });
                                if allows_input && !encyclopedia_close_queued {
                                    if let Some(focus) = encyclopedia_surface.take_focus_request() {
                                        strategic_windowing.enqueue(
                                            rebellion_windowing::WindowCommand::Focus(focus),
                                        );
                                    }
                                }
                            }
                            ManagedStrategicSurface::FleetFinder(_) => {
                                pending_fleet_finder_action =
                                    with_retained_input(ctx, allows_input, || {
                                        draw_fleet_finder_focused(
                                            ctx,
                                            &world,
                                            &movement_state,
                                            fog_state,
                                            &mut fleet_finder_state,
                                            strategic_frame_boundary.snapshot().focused(),
                                            cockpit_layout,
                                            &mut bmp_cache,
                                        )
                                    });
                                if allows_input {
                                    if let Some(focus) = fleet_finder_state.take_focus_request() {
                                        strategic_windowing.enqueue(
                                            rebellion_windowing::WindowCommand::Focus(focus),
                                        );
                                    }
                                }
                            }
                            ManagedStrategicSurface::MessageIndex(_) => {
                                let actions = with_retained_input(ctx, allows_input, || {
                                    if cockpit_layout.scale <= 0.0 {
                                        message_index.mark_unpaintable();
                                        return Vec::new();
                                    }
                                    let actions =
                                        rebellion_render::message_index::draw_message_index(
                                            ctx,
                                            &mut bmp_cache,
                                            cockpit_state.faction,
                                            rebellion_render::message_index::window_rect(
                                                cockpit_layout,
                                            )
                                            .min,
                                            cockpit_layout.scale,
                                            &msg_log,
                                            &mut message_index,
                                            player_faction == MissionFaction::Alliance,
                                        );
                                    message_index.mark_paintable();
                                    actions
                                });
                                message_choice = actions.iter().find_map(|action| match action {
                                    MessageIndexAction::GoTo(MessageTarget::System(system)) => {
                                        Some((*system, FinderTarget::Sector))
                                    }
                                    MessageIndexAction::GoTo(MessageTarget::Fleet(fleet)) => {
                                        rebellion_core::movement::listed_location(
                                            &movement_state,
                                            &world,
                                            *fleet,
                                        )
                                        .map(|system| (system, FinderTarget::Fleet(*fleet)))
                                    }
                                    _ => None,
                                });
                                if actions.contains(&MessageIndexAction::Close) {
                                    if let Some(window) = message_index.request_close() {
                                        strategic_windowing.enqueue(
                                            rebellion_windowing::WindowCommand::Close(window),
                                        );
                                        message_index_close_queued = true;
                                    }
                                }
                                apply_message_index_actions(
                                    actions,
                                    &mut msg_log,
                                    &mut clock,
                                    &mut advice_saved_speed,
                                    &mut pending_message_sound,
                                );
                            }
                            ManagedStrategicSurface::Status(_) => {
                                if let Some((window, action)) = draw_managed_status_window(
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
                                    strategic_frame_boundary.snapshot(),
                                ) {
                                    strategic_windowing
                                        .enqueue(rebellion_windowing::WindowCommand::Close(window));
                                    status_close_queued = true;
                                    macroquad::logging::info!(
                                        "[interface] destination=status_window status=close_queued action={}",
                                        match action {
                                            StatusWindowAction::Encyclopedia => "encyclopedia",
                                            StatusWindowAction::Close => "close",
                                        }
                                    );
                                    if action == StatusWindowAction::Encyclopedia {
                                        if encyclopedia_session_store.current().is_some() {
                                            let intent = EncyclopediaEntryIntent::Contextual {
                                                audience: encyclopedia_audience(player_faction),
                                                // The Status window's selected-object admission
                                                // join is not part of the merged upstream contract.
                                                object_id: 0,
                                                caller: EncyclopediaContextCaller::Handler004443a0Command66,
                                            };
                                            pending_encyclopedia_open.get_or_insert(intent);
                                            legacy_restriction_latch.note_start();
                                            macroquad::logging::info!(
                                                "[encyclopedia] production_route status=queued origin=contextual caller=FUN_004443a0_command_0x66 requested=unresolved"
                                            );
                                        } else {
                                            macroquad::logging::warn!(
                                                "[encyclopedia] production_route unavailable origin=status_window reason=no_session"
                                            );
                                        }
                                    }
                                }
                            }
                            ManagedStrategicSurface::BuildSelection(_) => {
                                let action = with_retained_input(ctx, allows_input, || {
                                    draw_build_selection(
                                        ctx,
                                        &world,
                                        &mfg_state,
                                        &mut build_selection_state,
                                        cockpit_layout,
                                        &mut bmp_cache,
                                    )
                                });
                                match action {
                                    Some(BuildSelectionAction::Confirm { .. }) => {
                                        if let Some(prepared) =
                                            build_selection_state.prepare_confirm(&world)
                                        {
                                            strategic_windowing
                                                .enqueue_build_selection_confirm(prepared);
                                        }
                                    }
                                    Some(BuildSelectionAction::Close) => {
                                        if let Some(window) = build_selection_state.request_close()
                                        {
                                            strategic_windowing.enqueue(
                                                rebellion_windowing::WindowCommand::Close(window),
                                            );
                                        }
                                    }
                                    Some(BuildSelectionAction::Encyclopedia) => {
                                        if encyclopedia_session_store.current().is_some() {
                                            let intent = EncyclopediaEntryIntent::Contextual {
                                                audience: encyclopedia_audience(player_faction),
                                                // Upstream recovered this caller while the
                                                // class-to-topic join remains explicitly
                                                // unbound. Preserve the caller and use the
                                                // presenter's unresolved index fallback.
                                                object_id: 0,
                                                caller: EncyclopediaContextCaller::Handler00438800Command67,
                                            };
                                            pending_encyclopedia_open.get_or_insert(intent);
                                            legacy_restriction_latch.note_start();
                                            macroquad::logging::info!(
                                                "[encyclopedia] production_route status=queued origin=contextual caller=FUN_00438800_command_0x67 requested=unresolved"
                                            );
                                        } else {
                                            macroquad::logging::warn!(
                                                "[encyclopedia] production_route unavailable origin=build_selection reason=no_session"
                                            );
                                        }
                                    }
                                    None => {}
                                }
                            }
                        }
                    }
                    if !encyclopedia_close_queued
                        && encyclopedia_keyboard_owner.is_some()
                        && strategic_root_keys.contains(KeyCode::Escape)
                    {
                        if let Some((window, route)) = encyclopedia_surface.request_close() {
                            strategic_windowing
                                .enqueue(rebellion_windowing::WindowCommand::Close(window));
                            log_encyclopedia_return(route);
                            legacy_keyboard_handled = true;
                        }
                    }
                    if !message_index_close_queued
                        && message_index_keyboard_owner.is_some()
                        && strategic_root_keys.contains(KeyCode::Escape)
                    {
                        if let Some(window) = message_index.request_close() {
                            strategic_windowing
                                .enqueue(rebellion_windowing::WindowCommand::Close(window));
                            legacy_keyboard_handled = true;
                        }
                    }
                    if !status_close_queued
                        && status_keyboard_owner.is_some()
                        && strategic_root_keys.contains(KeyCode::Escape)
                    {
                        if let Some(window) = status_window_state.request_close() {
                            strategic_windowing
                                .enqueue(rebellion_windowing::WindowCommand::Close(window));
                            legacy_keyboard_handled = true;
                        }
                    }
                    if let Some((system, target)) = message_choice {
                        map_state.selected_system = Some(system);
                        enqueue_sector_window(
                            &mut strategic_windowing,
                            &sector_window_state,
                            &world,
                            system,
                            cockpit_state.faction,
                            &mut pending_sector_publications,
                            message_choice_sector_dependent(system, target),
                        );
                    }
                    egui_claims_pointer = ctx.is_pointer_over_area() || ctx.wants_pointer_input();
                    egui_claims_keyboard = ctx.wants_keyboard_input();
                });

                strategic_root_keys.report_legacy_children(
                    &mut strategic_input_router,
                    egui_claims_keyboard || legacy_keyboard_handled,
                    fleet_keyboard_owner,
                    fleet_finder_keyboard_owner,
                    encyclopedia_keyboard_owner,
                    message_index_keyboard_owner,
                    status_keyboard_owner,
                    build_selection_keyboard_owner,
                );

                // Child surfaces have now handled their input. Keep every
                // restriction introduced during dispatch sticky until the next
                // immutable frame boundary, even if that surface also closes.
                #[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
                let command_palette_now_open = command_palette_state.open;
                #[cfg(not(any(debug_assertions, not(target_arch = "wasm32"))))]
                let command_palette_now_open = false;
                legacy_restriction_latch.observe(LegacyInputBlockers {
                    strategic_modal: mission_dialog_state.is_open()
                        || move_confirmation_state.is_open()
                        || scrap_confirmation_state.is_open()
                        || troop_finder_state.is_open()
                        || personnel_finder_state.is_open()
                        || original_modal_fixture_open,
                    focused_control: command_palette_now_open,
                    transient_menu: object_menu.is_some()
                        || game_speed_ui.menu_anchor.is_some()
                        || agent_menu_state.anchor.is_some()
                        || cockpit_state.gid_ui.menu_open,
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
                                strategic_windowing.enqueue_fleet_finder_open(
                                    FleetFinderState::prepare_open(cockpit_state.faction),
                                );
                                strategic_dispatch_restricted = true;
                                strategic_input_router.set_application_hold(true);
                                macroquad::logging::info!(
                                    "[interface] command=0x12e destination=fleet_finder status=queued"
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
                                cockpit_state.gid_ui.menu_open = !cockpit_state.gid_ui.menu_open;
                                cockpit_state.gid_ui.category = None;
                                if cockpit_state.gid_ui.menu_open {
                                    strategic_dispatch_restricted = true;
                                    strategic_input_router.set_application_hold(true);
                                }
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
                                    "[interface] command=0x{:x} destination=gid_menu status={}",
                                    rebellion_render::strategic_gid_control(cockpit_state.faction)
                                        .command_id,
                                    if cockpit_state.gid_ui.menu_open {
                                        "opened_original"
                                    } else {
                                        "closed"
                                    }
                                );
                                (
                                    rebellion_render::strategic_gid_control(cockpit_state.faction)
                                        .command_id,
                                    "gid_menu",
                                )
                            }
                        };
                        if btn == CockpitButton::Encyclopedia {
                            if encyclopedia_session_store.current().is_some() {
                                let intent = EncyclopediaEntryIntent::Cockpit {
                                    audience: encyclopedia_audience(player_faction),
                                };
                                pending_encyclopedia_open.get_or_insert(intent);
                                strategic_dispatch_restricted = true;
                                strategic_input_router.set_application_hold(true);
                                macroquad::logging::info!(
                                    "[interface] command=0x{:x} destination={} status=queued",
                                    command,
                                    destination
                                );
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

                if let Some(action) = pending_fleet_finder_action {
                    strategic_dispatch_restricted = true;
                    strategic_input_router.set_application_hold(true);
                    match action {
                        FleetFinderAction::Close(window) => {
                            strategic_windowing
                                .enqueue(rebellion_windowing::WindowCommand::Close(window));
                        }
                        FleetFinderAction::Open(entry) => {
                            if let Some(window) = fleet_finder_state.window_id() {
                                strategic_windowing
                                    .enqueue(rebellion_windowing::WindowCommand::Close(window));
                            }
                            // FUN_00429440: a Finder choice opens its system's
                            // sector window, then the Fleet window with the
                            // chosen fleet or ship selected. A fleet in
                            // hyperspace belongs to its destination here
                            // (FUN_00556390).
                            if let Some(system) = rebellion_core::movement::listed_location(
                                &movement_state,
                                &world,
                                entry.fleet(),
                            ) {
                                map_state.selected_system = Some(system);
                                enqueue_sector_window(
                                    &mut strategic_windowing,
                                    &sector_window_state,
                                    &world,
                                    system,
                                    cockpit_state.faction,
                                    &mut pending_sector_publications,
                                    Some(SectorDependentIntent::Fleet {
                                        system,
                                        selected: Some(entry),
                                        rename: false,
                                    }),
                                );
                            }
                        }
                    }
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
                            KeyCode::B => Some(2),
                            KeyCode::D => Some(3),
                            _ => None,
                        };
                        #[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
                        let palette_shortcut = palette_enabled && event.key == KeyCode::GraveAccent;
                        #[cfg(not(any(debug_assertions, not(target_arch = "wasm32"))))]
                        let palette_shortcut = false;
                        let root_action = event.key == KeyCode::Escape
                            || cockpit_accelerator(event.key).is_some()
                            || event.key == KeyCode::F6
                            || event.key == KeyCode::R
                            || event.alt
                                && matches!(
                                    event.key,
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
                            || panel_index.is_some_and(|_| {
                                panel_letters_open_panels(cfg!(debug_assertions), event.alt)
                            })
                            || event.key == KeyCode::S && !event.alt
                            || event.key == KeyCode::Tab
                            || palette_shortcut;
                        if !root_action || !strategic_input_router.claim_root(event.id) {
                            continue;
                        }

                        if event.key == KeyCode::Escape {
                            show_fleets = false;
                            show_missions = false;
                            show_bombardment = false;
                            show_death_star = false;
                            save_load_panel_state.close();
                            game_mode = GameMode::MainMenu;
                            strategic_dispatch_restricted = true;
                            strategic_input_router.set_application_hold(true);
                            macroquad::logging::info!(
                                "[main_menu] returned_from_campaign generation={} audio_context=main_menu",
                                campaign_generation
                            );
                        } else if let Some(btn) = cockpit_accelerator(event.key) {
                            dispatch_cockpit_button!(btn);
                        } else if event.key == KeyCode::F6 {
                            if let Some(category) =
                                rebellion_render::message_index::opening_category(0x75)
                            {
                                strategic_windowing.enqueue_message_index_open(
                                    MessageIndexState::prepare_open(category).expect(
                                        "opening_category returns a Message Index category",
                                    ),
                                );
                                strategic_dispatch_restricted = true;
                                strategic_input_router.set_application_hold(true);
                                macroquad::logging::info!(
                                    "[interface] command=0x75 destination=message_index category=0x{:x} status=queued_shared",
                                    category
                                );
                            }
                        } else if event.key == KeyCode::R {
                            map_state = GalaxyMapState::default();
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
                                &mut [
                                    &mut show_fleets,
                                    &mut show_missions,
                                    &mut show_bombardment,
                                    &mut show_death_star,
                                ],
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
                    enqueue_sector_window(
                        &mut strategic_windowing,
                        &sector_window_state,
                        &world,
                        system,
                        cockpit_state.faction,
                        &mut pending_sector_publications,
                        None,
                    );
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

            GameMode::GroundCombat => {
                if let Some(ref mut gc_state) = ground_combat_state {
                    let gc_action = draw_ground_combat(gc_state);
                    if gc_action == GroundAction::Done {
                        // Apply ground combat results to GameWorld.
                        if let Some(gc) = ground_combat_state.take() {
                            let sys_key = gc.system;
                            let attacker_is_alliance = gc.attacker_is_alliance;
                            let regiment_strengths: Vec<_> = gc
                                .regiments
                                .iter()
                                .map(|regiment| (regiment.troop, regiment.strength))
                                .collect();
                            tactical_flow::apply_tactical_ground_strengths(
                                &mut world,
                                sys_key,
                                &regiment_strengths,
                            );

                            let occupying_faction = match gc.winner {
                                Some(rebellion_render::GroundWinner::Attacker) => {
                                    Some(if attacker_is_alliance {
                                        Faction::Alliance
                                    } else {
                                        Faction::Empire
                                    })
                                }
                                Some(rebellion_render::GroundWinner::Defender) => {
                                    Some(if attacker_is_alliance {
                                        Faction::Empire
                                    } else {
                                        Faction::Alliance
                                    })
                                }
                                _ => None,
                            };
                            if let Some(winner) = occupying_faction {
                                tactical_flow::apply_system_occupation(
                                    &mut world,
                                    sys_key,
                                    winner,
                                    clock.tick,
                                    &mut msg_log,
                                );
                            }

                            let gc_winner_str = match gc.winner {
                                Some(rebellion_render::GroundWinner::Attacker) => {
                                    "Attacker ground victory"
                                }
                                Some(rebellion_render::GroundWinner::Defender) => {
                                    "Defender holds ground"
                                }
                                Some(rebellion_render::GroundWinner::Draw) | None => {
                                    "Ground combat draw"
                                }
                            };
                            msg_log.push(GameMessage::at_system(
                                clock.tick,
                                format!("Ground battle at {} — {}", gc.system_name, gc_winner_str),
                                MessageCategory::Combat,
                                gc.system,
                            ));
                        }
                        game_mode = GameMode::Galaxy;
                    }
                } else {
                    game_mode = GameMode::Galaxy;
                }
            }

            GameMode::VictoryModal { alliance_won } => {
                // Frozen galaxy backdrop with modal overlay.
                clear_background(Color::new(0.02, 0.02, 0.06, 1.0));
                egui_macroquad::ui(|ctx| {
                    use egui_macroquad::egui;
                    let title = if alliance_won {
                        "Alliance Victory!"
                    } else {
                        "Imperial Victory!"
                    };
                    let body = if alliance_won {
                        "The Rebel Alliance has triumphed. Freedom is restored to the galaxy."
                    } else {
                        "The Galactic Empire has crushed the Rebellion. Order reigns supreme."
                    };
                    egui::Area::new(egui::Id::new("victory_modal"))
                        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                        .order(egui::Order::Foreground)
                        .show(ctx, |ui| {
                            egui::Frame::new()
                                .fill(egui::Color32::from_rgba_unmultiplied(10, 15, 30, 230))
                                .inner_margin(egui::Margin::same(32))
                                .show(ui, |ui| {
                                    ui.set_width(400.0);
                                    ui.vertical_centered(|ui| {
                                        ui.label(
                                            egui::RichText::new(title)
                                                .size(24.0)
                                                .color(egui::Color32::from_rgb(230, 200, 100)),
                                        );
                                        ui.add_space(16.0);
                                        ui.label(
                                            egui::RichText::new(body)
                                                .size(14.0)
                                                .color(egui::Color32::from_rgb(200, 195, 185)),
                                        );
                                        ui.add_space(24.0);
                                        if ui.button("Continue").clicked() {
                                            game_mode = GameMode::MainMenu;
                                        }
                                    });
                                });
                        });
                });
                egui_macroquad::draw();
            }
        }

        if tactical_frame {
            // Under the Battle Alert or the results, the command center pass
            // above has drawn the droids; the window is all that is left.
            let rendered_tac_action = draw_tactical_view(
                &mut tactical_state,
                &mut bmp_cache,
                &world,
                &troop_transport_state,
                &tactical_window_input,
            );
            let manager_approved_action = approved_tactical_action.is_some();
            let tac_action = if let Some(action) = approved_tactical_action.take() {
                action
            } else if let Some(prepared) = prepare_tactical_window_action(
                rendered_tac_action.clone(),
                tactical_battle_session,
                tactical_window_identity,
            ) {
                pending_tactical_window_action = Some(prepared);
                TacticalAction::None
            } else {
                if matches!(
                    &rendered_tac_action,
                    TacticalAction::BeginCombat
                        | TacticalAction::AutoResolve
                        | TacticalAction::RetreatBeforeBattle
                        | TacticalAction::ReturnToGalaxy
                        | TacticalAction::OpenBattleSystem(_)
                        | TacticalAction::OpenBattleFleet(_)
                ) {
                    macroquad::logging::error!(
                        "[windowing] refused tactical action without its live identity"
                    );
                    TacticalAction::None
                } else {
                    rendered_tac_action
                }
            };
            if manager_approved_action
                && matches!(
                    &tac_action,
                    TacticalAction::BeginCombat
                        | TacticalAction::AutoResolve
                        | TacticalAction::RetreatBeforeBattle
                )
            {
                tactical_state.commit_battle_alert_exit();
            }
            if manager_approved_action {
                if let Some(destination) = tactical_result_destination(&tac_action) {
                    pending_tactical_destination = Some(destination);
                }
            }

            match tac_action {
                TacticalAction::BeginCombat => {
                    // Advance from placement to combat phase.
                    if let Some(ref mut session) = tactical_state.session {
                        session.phase = rebellion_render::BattlePhase::Combat;
                        session.queue_battle_ready_voice();
                    }
                }
                TacticalAction::AutoResolve => {
                    // Simulate runs the tactical engine to its end without its
                    // window (FUN_0040a050 state 8), then shows the same
                    // Battle Results window as a played engagement.
                    let mut result_presented = false;
                    if tactical_state.simulate_battle() {
                        let session = tactical_state
                            .session
                            .clone()
                            .expect("simulate_battle keeps the session");
                        let battle_return = tactical_flow::apply_results(
                            &session,
                            &mut world,
                            &mut troop_transport_state,
                            &mut movement_state,
                        );
                        if let (Some(destination), Some(shown)) =
                            (battle_return.withdrawal, tactical_state.session.as_mut())
                        {
                            shown.withdrawal_destination = world
                                .systems
                                .get(destination)
                                .map(|system| system.name.clone());
                        }
                        tactical_flow::reconcile_death_star_result(
                            &world,
                            &mut death_star_state,
                            &mut victory_state,
                        );

                        let ground_attacker = battle_return
                            .winner_fleet
                            .filter(|&fleet| world.fleets.contains_key(fleet));
                        if let Some(winner_fleet) = ground_attacker {
                            let ground_rolls: Vec<f64> =
                                (0..256).map(|_| sim_rng.gen::<f64>()).collect();
                            let outcome = tactical_flow::resolve_post_battle(
                                &mut world,
                                &victory_state,
                                &mut troop_transport_state,
                                winner_fleet,
                                session.system,
                                session.start_tick,
                                &mut msg_log,
                                tactical_flow::PostBattleGroundMode::Automatic {
                                    rolls: &ground_rolls,
                                },
                            );
                            macroquad::logging::info!(
                                "[post_battle] mode=automatic landed={} bombardment_damage={} headquarters_destroyed={}",
                                outcome.landed_regiments,
                                outcome.bombardment_damage,
                                outcome.headquarters_destroyed,
                            );
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
                            tactical_flow::reconcile_death_star_result(
                                &world,
                                &mut death_star_state,
                                &mut victory_state,
                            );
                        }
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

                        // Auto-resolve already completed bombardment and
                        // ground resolution before presenting its result.
                        if strategic_results_applied {
                            game_mode = GameMode::Galaxy;
                        } else if let Some(winner_fleet) = battle_return.winner_fleet {
                            let outcome = tactical_flow::resolve_post_battle(
                                &mut world,
                                &victory_state,
                                &mut troop_transport_state,
                                winner_fleet,
                                session.system,
                                session.start_tick,
                                &mut msg_log,
                                tactical_flow::PostBattleGroundMode::Interactive,
                            );
                            macroquad::logging::info!(
                                "[post_battle] mode=interactive landed={} bombardment_damage={} headquarters_destroyed={}",
                                outcome.landed_regiments,
                                outcome.bombardment_damage,
                                outcome.headquarters_destroyed,
                            );
                            match outcome.continuation {
                                tactical_flow::PostBattleContinuation::Galaxy => {
                                    game_mode = GameMode::Galaxy;
                                }
                                tactical_flow::PostBattleContinuation::GroundCombat(state) => {
                                    ground_combat_state = Some(state);
                                    game_mode = GameMode::GroundCombat;
                                }
                            }
                        } else {
                            game_mode = GameMode::Galaxy;
                        }
                    } else {
                        game_mode = GameMode::Galaxy;
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
                TacticalAction::RetreatSelected => {
                    if let Some(ref mut session) = tactical_state.session {
                        let player_side = session.player_is_attacker;
                        for ship in &mut session.ships {
                            if ship.selected && ship.is_attacker == player_side && ship.alive {
                                ship.retreating = true;
                                ship.selected = false;
                            }
                        }
                        session.selected_ship = None;
                    }
                }
                TacticalAction::WithdrawFromBattle => {
                    if let Some(ref mut session) = tactical_state.session {
                        let player_side = session.player_is_attacker;
                        for ship in &mut session.ships {
                            if ship.alive
                                && !ship.retreating
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
                    }
                }
                TacticalAction::RetreatBeforeBattle => {
                    if let Some(ref mut session) = tactical_state.session {
                        session.queue_withdrawal_started_voice();
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
                    cutscene_player = open_cutscene(
                        Path::new(trench_run_cutscene_path(outcome)),
                        audio_vol.cutscene_volume(),
                        &mut msg_log,
                        clock.tick,
                        #[cfg(not(target_arch = "wasm32"))]
                        &mut audio_engine,
                    );
                    if cutscene_player.is_some() {
                        game_mode = GameMode::Cutscene {
                            kind: CutsceneKind::TrenchRun(outcome),
                        };
                    }
                }
            }
        }

        if let Some(destination) = take_tactical_destination_for_galaxy(
            &mut pending_tactical_destination,
            game_mode == GameMode::Galaxy,
        ) {
            match destination {
                TacticalResultDestination::System(system) => {
                    map_state.selected_system = Some(system);
                    let layout = cockpit_state.layout();
                    let _ = enqueue_galaxy_system_window(
                        &mut strategic_windowing,
                        &system_window_state,
                        &world,
                        system,
                        (85, 55),
                        cockpit_state.faction,
                        layout,
                    );
                }
                TacticalResultDestination::Fleet(fleet) => {
                    // FUN_00429440: the Results fleet shortcut returns to the
                    // Fleet window with that exact fleet selected.
                    if let Some(value) = world.fleets.get(fleet) {
                        let system = value.location;
                        map_state.selected_system = Some(system);
                        let layout = cockpit_state.layout();
                        enqueue_galaxy_fleet_window(
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
                        death_star: &mut death_star_state,
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
                    let active_mods = mod_runtime.enabled_mod_list();
                    match rebellion_data::save::save_slot(
                        &saves_dir,
                        slot,
                        &name,
                        &state,
                        &active_mods,
                    ) {
                        Ok(fingerprint) => {
                            macroquad::logging::info!(
                                "save_state_fingerprint slot={} tick={} fingerprint={}",
                                slot,
                                state.clock.tick,
                                fingerprint
                            );
                            save_load_panel_state.error_message = None;
                            save_slots = read_save_slots(&saves_dir);
                            game_options_state.refresh_saves(&save_slots);
                            game_options_state.error = None;
                            msg_log.push(GameMessage::new(
                                clock.tick,
                                format!("Saved game to slot {}", slot + 1),
                                MessageCategory::Event,
                            ));
                        }
                        Err(error) => {
                            save_load_panel_state.error_message = Some(error.to_string());
                            game_options_state.error = Some(error.to_string());
                        }
                    }
                }
                PanelAction::LoadGame { slot } => {
                    match rebellion_data::save::load_slot(&saves_dir, slot) {
                        Ok((meta, state)) => {
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
                                death_star: &mut death_star_state,
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
                            retained_legacy_input_filter.reset();
                            let restored_fingerprint =
                                rebellion_data::save::compute_state_fingerprint(
                                    &campaign.snapshot(),
                                );
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
                            advisor_state
                                .resume_saved_game(AdvisorFaction::from(cockpit_state.faction));
                            advice_topics = rebellion_render::advice_topics::AdviceTopics::new(
                                AdvisorFaction::from(cockpit_state.faction),
                                |id| text_templates.get(id),
                            );
                            map_state = GalaxyMapState::default();
                            sector_window_state.clear();
                            pending_sector_publications.clear();
                            pending_committed_sector_actions.clear();
                            system_window_state.clear();
                            fleet_window_state.clear();
                            defenses_window_state.clear();
                            missions_window_state.clear();
                            object_menu = None;
                            targeting = None;
                            fleets_state = FleetsState::default();
                            build_selection_state = BuildSelectionState::default();
                            mission_dialog_state = MissionDialogState::default();
                            move_confirmation_state = MoveConfirmationState::default();
                            scrap_confirmation_state = ScrapConfirmationState::default();
                            status_window_state = StatusWindowState::default();
                            fleet_finder_state = FleetFinderState::default();
                            troop_finder_state = TroopFinderState::default();
                            personnel_finder_state = PersonnelFinderState::default();
                            message_index.reset();
                            advice_saved_speed = None;
                            bombardment_panel_state = BombardmentPanelState::default();
                            encyclopedia_surface.reset();
                            show_fleets = false;
                            show_missions = false;
                            show_bombardment = false;
                            show_death_star = false;
                            save_load_panel_state.close();
                            event_screen_state = EventScreenState::new();
                            tactical_state = TacticalState::new();
                            reset_pending_tactical_handoff(
                                &mut pending_tactical_window_action,
                                &mut pending_tactical_destination,
                            );
                            ground_combat_state = None;
                            dual_ai_mode = secondary_ai_state.is_some();
                            msg_log = MessageLog::default();
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
                        }
                        Err(error) => {
                            save_load_panel_state.error_message = Some(error.to_string());
                            game_options_state.error = Some(error.to_string());
                        }
                    }
                }
                PanelAction::DeleteSave { slot } => {
                    match rebellion_data::save::delete_slot(&saves_dir, slot) {
                        Ok(()) => {
                            save_load_panel_state.selected_slot = None;
                            save_load_panel_state.error_message = None;
                            save_slots = read_save_slots(&saves_dir);
                            game_options_state.refresh_saves(&save_slots);
                            game_options_state.error = None;
                            msg_log.push(GameMessage::new(
                                clock.tick,
                                format!("Deleted save in slot {}", slot + 1),
                                MessageCategory::Event,
                            ));
                        }
                        Err(error) => {
                            save_load_panel_state.error_message = Some(error.to_string());
                            game_options_state.error = Some(error.to_string());
                        }
                    }
                }
                PanelAction::SetGameSpeed(speed) => choose_game_speed(&mut clock, speed),
                PanelAction::CloseSaveLoadPanel => {
                    save_load_panel_state.close();
                    if game_mode == GameMode::LoadGame {
                        game_mode = GameMode::MainMenu;
                    }
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
                        &mut mission_state,
                        &mut movement_state,
                        &mut troop_transport_state,
                        active_fog_state,
                        &mut ai_state,
                        &mut death_star_state,
                        &mut msg_log,
                        &mut advisor_state,
                        &mut player_faction,
                        &mut clock,
                        &mut dual_ai_mode,
                        &mut secondary_ai_state,
                        &mut victory_state,
                        campaign_config,
                        &mut blockade_state,
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
                        left_panel_open: show_fleets
                            || show_missions
                            || show_bombardment
                            || show_death_star,
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

        let mut strategic_content_reports =
            sector_window_state.content_reports(strategic_frame_boundary.snapshot(), &world);
        strategic_content_reports.extend(
            system_window_state.content_reports(strategic_frame_boundary.snapshot(), &world),
        );
        strategic_content_reports.extend(
            fleet_window_state.content_reports(strategic_frame_boundary.snapshot(), &world),
        );
        strategic_content_reports.extend(
            defenses_window_state.content_reports(strategic_frame_boundary.snapshot(), &world),
        );
        strategic_content_reports.extend(
            missions_window_state.content_reports(strategic_frame_boundary.snapshot(), &world),
        );
        strategic_content_reports
            .extend(fleet_finder_state.content_reports(strategic_frame_boundary.snapshot()));
        strategic_content_reports
            .extend(encyclopedia_surface.content_reports(strategic_frame_boundary.snapshot()));
        strategic_content_reports
            .extend(message_index.content_reports(strategic_frame_boundary.snapshot()));
        strategic_content_reports
            .extend(build_selection_state.content_reports(strategic_frame_boundary.snapshot()));
        strategic_content_reports.extend(status_window_state.content_reports(
            strategic_frame_boundary.snapshot(),
            StatusSources {
                world: &world,
                missions: &mission_state,
                movement: &movement_state,
                transport: &troop_transport_state,
                manufacturing: &mfg_state,
                today: clock.tick,
            },
        ));
        strategic_content_reports.extend(
            strategic_windowing.tactical_content_reports(
                strategic_frame_boundary.snapshot(),
                tactical_window_phase,
            ),
        );

        // Window intents commit only after input and painting have completed.
        // Detail content is published only after the matching manager commit;
        // close and eviction receipts remove only their exact identities.
        let publication_layout = cockpit_state.layout();
        let publication_fog = sector_publication_fog_for_faction(
            player_faction,
            &fog_alliance_state,
            &fog_empire_state,
        );
        let pending_command_count = strategic_windowing.begin_apply_batch();
        for _ in 0..pending_command_count {
            refresh_pending_sector_publication(
                &mut strategic_windowing,
                &sector_window_state,
                &world,
                &mut pending_sector_publications,
            );
            let Some(applied) = strategic_windowing.apply_next_pending() else {
                continue;
            };
            let strategic_windowing::AppliedWindowCommand {
                command,
                result,
                mut removed_content,
                prepared_build_selection,
                build_selection_content_id,
                confirmed_build_selection,
                prepared_system,
                system_content_id,
                prepared_defenses,
                defenses_content_id,
                prepared_missions,
                missions_content_id,
                prepared_fleet,
                fleet_content_id,
                prepared_fleet_finder,
                fleet_finder_content_id,
                prepared_message_index,
                message_index_content_id,
                prepared_status,
                status_content_id,
                prepared_sector,
                sector_content_id,
                minimized_system,
                restored_system,
                minimized_defenses,
                restored_defenses,
                minimized_missions,
                restored_missions,
                minimized_fleet,
                restored_fleet,
            } = applied;
            let activated_system = matches!(
                command,
                rebellion_windowing::WindowCommand::Activate(id)
                    if system_window_state.contains_window(id)
            );
            let activated_fleet = matches!(
                command,
                rebellion_windowing::WindowCommand::Activate(id)
                    if fleet_window_state.contains_window(id)
            );
            let activated_defenses = matches!(
                command,
                rebellion_windowing::WindowCommand::Activate(id)
                    if defenses_window_state.contains_window(id)
            );
            let activated_missions = matches!(
                command,
                rebellion_windowing::WindowCommand::Activate(id)
                    if missions_window_state.contains_window(id)
            );
            let mut committed_system_open = false;
            let mut committed_defenses_open = false;
            let mut committed_fleet_open = false;
            if let (Some(prepared), Some(id), Ok(commit)) = (
                prepared_build_selection,
                build_selection_content_id,
                &result,
            ) {
                if build_selection_state.commit_open(prepared, &commit.transition) == Some(id) {
                    let committed = strategic_windowing.has_content(id)
                        || strategic_windowing.acknowledge_build_selection_publication(id);
                    if !committed {
                        build_selection_state.remove_content(&[id]);
                        removed_content
                            .extend(strategic_windowing.rollback_build_selection_publication(id));
                    }
                } else {
                    macroquad::logging::error!(
                        "[windowing] Build Selection content commit did not match identity={id:?}"
                    );
                    if matches!(
                        commit.transition,
                        rebellion_windowing::WindowTransition::Opened { .. }
                    ) {
                        removed_content
                            .extend(strategic_windowing.rollback_build_selection_publication(id));
                    }
                }
            }
            if let (Some(prepared), Some(id), Ok(commit)) =
                (prepared_system, system_content_id, &result)
            {
                if system_window_state.commit_open(prepared, &commit.transition) == Some(id) {
                    committed_system_open = strategic_windowing.has_content(id)
                        || strategic_windowing.acknowledge_system_publication(id);
                    if !committed_system_open {
                        system_window_state.remove_content(&[id]);
                        removed_content.extend(strategic_windowing.rollback_detail_publication(id));
                    }
                } else {
                    macroquad::logging::error!(
                        "[windowing] System content commit did not match identity={id:?}"
                    );
                    if matches!(
                        commit.transition,
                        rebellion_windowing::WindowTransition::Opened { .. }
                    ) {
                        removed_content.extend(strategic_windowing.rollback_detail_publication(id));
                    }
                }
            }
            let (accepted_sector_dependent, sector_rollback) = commit_sector_window_publication(
                &mut strategic_windowing,
                &mut sector_window_state,
                &mut pending_sector_publications,
                prepared_sector,
                sector_content_id,
                &result,
            );
            removed_content.extend(sector_rollback);
            pump_pending_sector_publication(
                &mut strategic_windowing,
                &sector_window_state,
                &world,
                &mut pending_sector_publications,
            );
            if let Some(dependent) = accepted_sector_dependent {
                match dependent {
                    SectorDependentIntent::Defenses { system } => {
                        if let Some(logical_position) = sector_window_state.quadrant_window_point(
                            &world,
                            publication_layout,
                            system,
                            Quadrant::Defenses,
                        ) {
                            enqueue_defenses_window(
                                &mut strategic_windowing,
                                &defenses_window_state,
                                &world,
                                system,
                                logical_position,
                                cockpit_state.faction,
                                publication_layout,
                            );
                        }
                    }
                    SectorDependentIntent::Fleet {
                        system,
                        selected,
                        rename,
                    } => {
                        let queued = sector_window_state
                            .quadrant_window_point(
                                &world,
                                publication_layout,
                                system,
                                Quadrant::Fleets,
                            )
                            .is_some_and(|logical_position| {
                                if rename {
                                    selected
                                        .and_then(|entry| {
                                            fleet_window_state
                                                .prepare_open(
                                                    &world,
                                                    system,
                                                    logical_position,
                                                    cockpit_state.faction,
                                                    publication_layout,
                                                    Some(entry),
                                                )
                                                .and_then(|prepared| {
                                                    prepared.with_rename(&world, entry)
                                                })
                                        })
                                        .is_some_and(|prepared| {
                                            strategic_windowing.enqueue_fleet_open(prepared);
                                            true
                                        })
                                } else {
                                    enqueue_fleet_window(
                                        &mut strategic_windowing,
                                        &fleet_window_state,
                                        &world,
                                        system,
                                        logical_position,
                                        cockpit_state.faction,
                                        publication_layout,
                                        selected,
                                    )
                                }
                            });
                        if queued && rename {
                            legacy_restriction_latch.note_start();
                        }
                    }
                    SectorDependentIntent::Missions { system, member } => {
                        if let Some(logical_position) = sector_window_state.quadrant_window_point(
                            &world,
                            publication_layout,
                            system,
                            Quadrant::Missions,
                        ) {
                            enqueue_missions_window_for_member(
                                &mut strategic_windowing,
                                &missions_window_state,
                                &world,
                                publication_fog,
                                &mission_state,
                                system,
                                logical_position,
                                cockpit_state.faction,
                                publication_layout,
                                member,
                            );
                        }
                    }
                    #[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
                    SectorDependentIntent::Interface(command) => {
                        match sector_window_state.command_actions(
                            &world,
                            publication_fog,
                            &mission_state,
                            &movement_state,
                            cockpit_state.faction,
                            publication_layout,
                            command,
                        ) {
                            Ok(actions) => pending_committed_sector_actions.extend(actions),
                            #[cfg(not(target_arch = "wasm32"))]
                            Err(why) => dev_channel.refuse(why),
                            #[cfg(target_arch = "wasm32")]
                            Err(why) => eprintln!("[dev-command] refused: {why}"),
                        }
                    }
                }
            }
            if let (Some(prepared), Some(id), Ok(commit)) =
                (prepared_defenses, defenses_content_id, &result)
            {
                if defenses_window_state.commit_open(prepared, &commit.transition) == Some(id) {
                    committed_defenses_open = strategic_windowing.has_content(id)
                        || strategic_windowing.acknowledge_defenses_publication(id);
                    if !committed_defenses_open {
                        defenses_window_state.remove_content(&[id]);
                        removed_content.extend(strategic_windowing.rollback_detail_publication(id));
                    }
                } else {
                    macroquad::logging::error!(
                        "[windowing] Defenses content commit did not match identity={id:?}"
                    );
                    if matches!(
                        commit.transition,
                        rebellion_windowing::WindowTransition::Opened { .. }
                    ) {
                        removed_content.extend(strategic_windowing.rollback_detail_publication(id));
                    }
                }
            }
            let (missions_committed, missions_rollback) = commit_missions_window_publication(
                &mut strategic_windowing,
                &mut missions_window_state,
                prepared_missions,
                missions_content_id,
                &result,
            );
            let committed_missions_open = missions_committed;
            removed_content.extend(missions_rollback);
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
                        "[windowing] Fleet content commit did not match identity={id:?}"
                    );
                    if matches!(
                        commit.transition,
                        rebellion_windowing::WindowTransition::Opened { .. }
                    ) {
                        removed_content.extend(strategic_windowing.rollback_fleet_publication(id));
                    }
                }
            }
            if let (Some(prepared), Some(id), Ok(commit)) =
                (prepared_fleet_finder, fleet_finder_content_id, &result)
            {
                if fleet_finder_state.commit_open(prepared, &commit.transition) == Some(id) {
                    let committed = strategic_windowing.has_content(id)
                        || strategic_windowing.acknowledge_fleet_finder_publication(id);
                    if !committed {
                        fleet_finder_state.remove_content(&[id]);
                        removed_content
                            .extend(strategic_windowing.rollback_fleet_finder_publication(id));
                    }
                } else {
                    macroquad::logging::error!(
                        "[windowing] Fleet Finder content commit did not match identity={id:?}"
                    );
                    if matches!(
                        commit.transition,
                        rebellion_windowing::WindowTransition::Opened { .. }
                    ) {
                        removed_content
                            .extend(strategic_windowing.rollback_fleet_finder_publication(id));
                    }
                }
            }
            if let (Some(prepared), Some(id), Ok(commit)) =
                (prepared_message_index, message_index_content_id, &result)
            {
                let initial_row = prepared.initial_row();
                let opened = matches!(
                    commit.transition,
                    rebellion_windowing::WindowTransition::Opened { .. }
                );
                if let Some((committed_id, mut actions)) =
                    message_index.commit_open(prepared, &commit.transition)
                {
                    if committed_id == id {
                        if opened {
                            if let Some(row) = initial_row {
                                let rows = rebellion_render::message_index::filtered_messages(
                                    &msg_log,
                                    message_index.category(),
                                    player_faction == MissionFaction::Alliance,
                                );
                                actions.extend(message_index.view_row(&rows, row));
                            }
                        }
                        let committed = strategic_windowing.has_content(id)
                            || strategic_windowing.acknowledge_message_index_publication(id);
                        if committed {
                            apply_message_index_actions(
                                actions,
                                &mut msg_log,
                                &mut clock,
                                &mut advice_saved_speed,
                                &mut pending_message_sound,
                            );
                        } else {
                            let close_actions = message_index.remove_content(&[id]);
                            apply_message_index_actions(
                                close_actions,
                                &mut msg_log,
                                &mut clock,
                                &mut advice_saved_speed,
                                &mut pending_message_sound,
                            );
                            removed_content
                                .extend(strategic_windowing.rollback_message_index_publication(id));
                        }
                    }
                } else {
                    macroquad::logging::error!(
                        "[windowing] Message Index content commit did not match identity={id:?}"
                    );
                    if opened {
                        removed_content
                            .extend(strategic_windowing.rollback_message_index_publication(id));
                    }
                }
            }
            if let (Some(prepared), Some(id), Ok(commit)) =
                (prepared_status, status_content_id, &result)
            {
                let created = matches!(
                    commit.transition,
                    rebellion_windowing::WindowTransition::Opened { .. }
                        | rebellion_windowing::WindowTransition::Replaced { .. }
                );
                if status_window_state.commit_open(prepared, &commit.transition) == Some(id) {
                    let committed = strategic_windowing.has_content(id)
                        || strategic_windowing.acknowledge_status_publication(id);
                    if !committed {
                        status_window_state.remove_content(&[id]);
                        removed_content.extend(strategic_windowing.rollback_status_publication(id));
                    }
                } else {
                    macroquad::logging::error!(
                        "[windowing] Status content commit did not match identity={id:?}"
                    );
                    if created {
                        removed_content.extend(strategic_windowing.rollback_status_publication(id));
                    }
                }
            }
            if let Some(BuildSelectionAction::Confirm {
                system,
                area,
                count,
                ..
            }) = commit_build_selection_gameplay(
                confirmed_build_selection,
                &removed_content,
                &world,
                &mut build_selection_state,
                &mut mfg_state,
            ) {
                macroquad::logging::info!(
                    "[interface] command=0x21x destination=production_build status=queued area={area:?} count={count} system={}",
                    world.systems.get(system).map_or("", |value| value.name.as_str())
                );
            }
            if (activated_fleet && result.is_ok()) || committed_fleet_open {
                fleet_legacy_bridge_serial = FleetWindowState::next_legacy_bridge_serial();
            }
            if (activated_system && result.is_ok()) || committed_system_open {
                system_legacy_bridge_serial = SystemWindowState::next_legacy_bridge_serial();
            }
            if (activated_defenses && result.is_ok()) || committed_defenses_open {
                defenses_legacy_bridge_serial = DefensesWindowState::next_legacy_bridge_serial();
            }
            if (activated_missions && result.is_ok()) || committed_missions_open {
                missions_legacy_bridge_serial = MissionsWindowState::next_legacy_bridge_serial();
            }
            if let Some(window) = restored_system {
                system_window_state.restore_system_window(window);
            }
            if let Some(window) = restored_fleet {
                system_window_state.restore_fleet_window(window);
            }
            if restored_defenses.is_some() {
                defenses_legacy_bridge_serial = DefensesWindowState::next_legacy_bridge_serial();
            }
            if restored_missions.is_some() {
                missions_legacy_bridge_serial = MissionsWindowState::next_legacy_bridge_serial();
            }
            if let (Some((window, system, logical_position)), Ok(commit)) =
                (minimized_system, &result)
            {
                if matches!(
                    &commit.transition,
                    rebellion_windowing::WindowTransition::Minimized { .. }
                ) {
                    if let Some(evicted) =
                        system_window_state.minimize_system_window(window, system, logical_position)
                    {
                        strategic_input_router.cancel_window(evicted);
                        fleet_window_state.cancel_pointer_capture(evicted);
                        removed_content
                            .extend(strategic_windowing.rollback_detail_publication(evicted));
                    }
                }
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
                            .extend(strategic_windowing.rollback_detail_publication(evicted));
                    }
                }
            }
            if let (Some((window, system, logical_position)), Ok(commit)) =
                (minimized_defenses, &result)
            {
                if matches!(
                    &commit.transition,
                    rebellion_windowing::WindowTransition::Minimized { .. }
                ) {
                    if let Some(evicted) =
                        system_window_state.minimize_defenses_window(system, logical_position)
                    {
                        strategic_input_router.cancel_window(evicted);
                        fleet_window_state.cancel_pointer_capture(evicted);
                        defenses_window_state.cancel_pointer_capture(evicted);
                        removed_content
                            .extend(strategic_windowing.rollback_detail_publication(evicted));
                    }
                    debug_assert!(defenses_window_state.contains_window(window));
                }
            }
            if let (Some((window, system, logical_position)), Ok(commit)) =
                (minimized_missions, &result)
            {
                if matches!(
                    &commit.transition,
                    rebellion_windowing::WindowTransition::Minimized { .. }
                ) {
                    if let Some(evicted) =
                        system_window_state.minimize_missions_window(system, logical_position)
                    {
                        strategic_input_router.cancel_window(evicted);
                        fleet_window_state.cancel_pointer_capture(evicted);
                        defenses_window_state.cancel_pointer_capture(evicted);
                        removed_content
                            .extend(strategic_windowing.rollback_detail_publication(evicted));
                    }
                    debug_assert!(missions_window_state.contains_window(window));
                }
            }
            sector_window_state.remove_content(&removed_content);
            system_window_state.remove_content(&removed_content);
            fleet_window_state.remove_content(&removed_content);
            defenses_window_state.remove_content(&removed_content);
            missions_window_state.remove_content(&removed_content);
            fleet_finder_state.remove_content(&removed_content);
            encyclopedia_surface.remove_content(&removed_content);
            status_window_state.remove_content(&removed_content);
            build_selection_state.remove_content(&removed_content);
            let message_close_actions = message_index.remove_content(&removed_content);
            apply_message_index_actions(
                message_close_actions,
                &mut msg_log,
                &mut clock,
                &mut advice_saved_speed,
                &mut pending_message_sound,
            );

            match &result {
                Ok(commit) => {
                    apply_capture_cancellation_events(
                        &commit.events,
                        &mut strategic_input_router,
                        &mut sector_window_state,
                        &mut fleet_window_state,
                        &mut defenses_window_state,
                    );
                    if !removed_content.is_empty() {
                        macroquad::logging::info!(
                            "[windowing] released_content={:?}",
                            removed_content
                        );
                    }
                }
                Err(error) => {
                    macroquad::logging::warn!(
                        "[windowing] rejected command={:?} error={error:?}",
                        command
                    );
                }
            }
        }

        // Reports reconcile after command commit against the immutable frame
        // that actually routed and painted this content.
        let reconciliation = strategic_windowing.reconcile(
            strategic_frame_boundary.snapshot(),
            &strategic_content_reports,
        );
        sector_window_state.remove_content(&reconciliation.removed);
        system_window_state.remove_content(&reconciliation.removed);
        fleet_window_state.remove_content(&reconciliation.removed);
        defenses_window_state.remove_content(&reconciliation.removed);
        missions_window_state.remove_content(&reconciliation.removed);
        fleet_finder_state.remove_content(&reconciliation.removed);
        encyclopedia_surface.remove_content(&reconciliation.removed);
        status_window_state.remove_content(&reconciliation.removed);
        build_selection_state.remove_content(&reconciliation.removed);
        let message_close_actions = message_index.remove_content(&reconciliation.removed);
        apply_message_index_actions(
            message_close_actions,
            &mut msg_log,
            &mut clock,
            &mut advice_saved_speed,
            &mut pending_message_sound,
        );
        apply_capture_cancellation_events(
            &reconciliation.events,
            &mut strategic_input_router,
            &mut sector_window_state,
            &mut fleet_window_state,
            &mut defenses_window_state,
        );
        if !reconciliation.invalid_reports.is_empty()
            || !reconciliation.blocking_unpainted.is_empty()
        {
            macroquad::logging::warn!(
                "[windowing] reconcile invalid={:?} blocking={:?}",
                reconciliation.invalid_reports,
                reconciliation.blocking_unpainted
            );
        }
        if let Some(intent) = pending_encyclopedia_open {
            if let Some(session) = encyclopedia_session_store.current() {
                match strategic_windowing.commit_encyclopedia_open(
                    &mut encyclopedia_surface,
                    &session,
                    intent,
                ) {
                    Ok(committed) => {
                        macroquad::logging::info!(
                            "[encyclopedia] production_route status=committed identity={:?}",
                            committed.window_id
                        );
                        apply_capture_cancellation_events(
                            &committed.events,
                            &mut strategic_input_router,
                            &mut sector_window_state,
                            &mut fleet_window_state,
                            &mut defenses_window_state,
                        );
                    }
                    Err(error) => {
                        macroquad::logging::error!(
                            "[encyclopedia] production_route status=rejected error={error}"
                        );
                    }
                }
            } else {
                macroquad::logging::warn!(
                    "[encyclopedia] production_route status=rejected reason=no_session"
                );
            }
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
    tick: u64,
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
    msg_log: &mut MessageLog,
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
        Some(target) if !fleets.is_empty() => fleets.iter().try_for_each(|fleet| {
            rebellion_core::fleet_join::validate_join(
                world,
                context.movement,
                &FleetMover::Fleet(*fleet),
                target,
                is_alliance,
            )
            .map(|_| ())
            .map_err(|error| error.to_string())
        }),
        _ => validate_fleets_dispatch(context.movement, world, fleets, destination, is_alliance)
            .map_err(|error| error.to_string()),
    };
    if let Err(error) = checked {
        // port: FUN_00487c90 plays the side's advisor reaction; this line
        // stands in.
        msg_log.push(GameMessage::new(
            context.tick,
            format!("Fleet move rejected: {error}"),
            MessageCategory::Event,
        ));
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
        panel_actions.extend(fleets.iter().map(|fleet| match join {
            Some(target) => PanelAction::JoinFleet {
                mover: FleetMover::Fleet(*fleet),
                target,
            },
            None => PanelAction::DispatchFleet {
                fleet: *fleet,
                destination,
                troops: Vec::new(),
            },
        }));
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
        destination,
        join,
        blockaded: false,
        lines,
    });
    true
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
            "[interface] command=0x241 destination={order:?} status=refused code={:?}",
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
            regiment_strength: 10,
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
            has_death_star: false,
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

/// Whether each order of `selection`'s menu passes its own rule (`+0x18`).
fn order_gates(
    selection: Option<MenuObject>,
    world: &GameWorld,
    mission_state: &MissionState,
    movement_state: &MovementState,
    troop_transport_state: &TroopTransportState,
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
            Some(MenuObject::Ship { fleet, index, .. }) => {
                fleet_move_enabled(movement_state, world, fleet, is_alliance)
                    && world
                        .fleets
                        .get(fleet)
                        .and_then(|value| value.capital_ships.get(index))
                        .is_some_and(|ship| ship.alive)
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
        // Stop clears a band that is building (manual p. 84).
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
        // refuses a destroyed object (FUN_0053a000). The order's own +0x18
        // rule is untraced.
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
                            .is_some_and(|ship| ship.alive)
                })
            }
            _ => false,
        },
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
#[derive(Debug, Clone, Copy)]
enum FinderTarget {
    /// A system alone: its sector window.
    Sector,
    /// The System Defenses window (kind 10).
    Defenses,
    /// The Fleet window (kind 4) with the fleet selected.
    Fleet(rebellion_core::ids::FleetKey),
    /// The Missions window (kind 11) showing the member's mission.
    Missions(rebellion_core::missions::MissionMember),
}

fn message_choice_sector_dependent(
    system: SystemKey,
    target: FinderTarget,
) -> Option<SectorDependentIntent> {
    match target {
        FinderTarget::Fleet(fleet) => Some(SectorDependentIntent::Fleet {
            system,
            selected: Some(FleetWindowEntry::Fleet(fleet)),
            rename: false,
        }),
        _ => None,
    }
}

fn sector_publication_fog_for_faction<'a>(
    player_faction: MissionFaction,
    alliance: &'a FogState,
    empire: &'a FogState,
) -> &'a FogState {
    if player_faction == MissionFaction::Alliance {
        alliance
    } else {
        empire
    }
}

/// A detail intent held until the exact Sector publication receipt is
/// accepted. This prevents a rejected replacement from opening content for a
/// Sector the user never saw.
#[derive(Debug, Clone, Copy)]
enum SectorDependentIntent {
    Defenses {
        system: SystemKey,
    },
    Fleet {
        system: SystemKey,
        selected: Option<FleetWindowEntry>,
        rename: bool,
    },
    Missions {
        system: SystemKey,
        member: rebellion_core::missions::MissionMember,
    },
    #[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
    Interface(InterfaceCommand),
}

#[derive(Debug, Clone, Copy)]
struct PendingSectorPublication {
    system: SystemKey,
    faction: CockpitFaction,
    prepared: Option<rebellion_render::sector_window::PreparedSectorWindow>,
    dependent: Option<SectorDependentIntent>,
}

/// Carry out what the Message Index asks: the Advice speed hold
/// (`FUN_00487ff0`), marking read and queuing the shown message's sound
/// (`FUN_0048a530`, `FUN_00469de0`), deleting (`FUN_005f54a0`) and the
/// posting flags (`FUN_0041d230`, `FUN_0041d290`). The caller routes
/// [`MessageIndexAction::GoTo`].
fn apply_message_index_actions(
    actions: Vec<MessageIndexAction>,
    log: &mut MessageLog,
    clock: &mut GameClock,
    saved_speed: &mut Option<GameSpeed>,
    sound: &mut Option<u16>,
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
                *sound = log
                    .messages()
                    .iter()
                    .find(|msg| msg.id == id)
                    .and_then(|msg| msg.display.as_ref())
                    .map(|display| display.sound)
                    .filter(|&sound| sound != 0);
            }
            MessageIndexAction::SetPostFlags { mask, flags } => {
                log.set_post_flags(mask, flags);
                macroquad::logging::info!(
                    "[message_index] post_flags mask=0x{mask:x} silent={}",
                    flags & rebellion_render::message_log::POST_SILENTLY != 0
                );
            }
            MessageIndexAction::Close | MessageIndexAction::GoTo(_) => {}
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
        "[agent] command={name} status={} source={source}",
        if on { "on" } else { "off" }
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
    mission_state: &mut MissionState,
    movement_state: &mut MovementState,
    troop_transport_state: &mut TroopTransportState,
    fog_state: &mut FogState,
    ai_state: &mut AIState,
    death_star_state: &mut DeathStarState,
    msg_log: &mut MessageLog,
    advisor_state: &mut AdvisorState,
    player_faction: &mut MissionFaction,
    clock: &mut GameClock,
    dual_ai_mode: &mut bool,
    secondary_ai_state: &mut Option<AIState>,
    victory_state: &mut VictoryState,
    campaign_config: CampaignConfig,
    blockade_state: &mut BlockadeState,
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
                    troop,
                    target,
                    clock.tick,
                )
            } else {
                Err(rebellion_core::troop_transport::TroopTransportError::WrongFaction)
            };
            // port: FUN_00487c90's advisor reaction; this line stands in.
            if let Err(error) = result {
                msg_log.push(GameMessage::new(
                    clock.tick,
                    format!("Regiment move rejected: {error}"),
                    MessageCategory::Event,
                ));
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
                    source,
                    target,
                    clock.tick,
                )
            } else {
                Err(rebellion_core::troop_transport::TroopTransportError::WrongFaction)
            };
            eprintln!("[order] squadron move {source:?} to {target:?}: {result:?}");
            // port: FUN_00487c90's advisor reaction; this line stands in.
            if let Err(error) = result {
                msg_log.push(GameMessage::new(
                    clock.tick,
                    format!("Squadron move rejected: {error}"),
                    MessageCategory::Event,
                ));
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
                        "[interface] command=0x241 destination={destination:?} status=refused code={:?}",
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
            );
            if let Err(error) = result {
                msg_log.push(GameMessage::new(
                    clock.tick,
                    format!("Fleet move rejected: {error}"),
                    MessageCategory::Event,
                ));
            }
        }
        PanelAction::SetDestination {
            system,
            area,
            destination,
        } => {
            let areas = area.map_or(
                rebellion_core::manufacturing::ProductionArea::ALL.to_vec(),
                |area| vec![area],
            );
            for area in &areas {
                mfg_state.set_destination(system, *area, destination);
            }
            macroquad::logging::info!(
                "[interface] command=0x214 destination=production_destination status=set areas={} to={}",
                areas.len(),
                world
                    .systems
                    .get(destination)
                    .map_or("", |value| value.name.as_str())
            );
        }
        PanelAction::Rename { fleet, ship, name } => {
            let renamed = match ship {
                Some(index) => world.rename_ship(fleet, index, &name),
                None => world.rename_fleet(fleet, &name),
            };
            macroquad::logging::info!(
                "[interface] command=0x203 destination=rename status={} name={name}",
                if renamed { "applied" } else { "refused" }
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
                msg_log.push(GameMessage::new(
                    clock.tick,
                    format!("Create Fleet rejected: {error}"),
                    MessageCategory::Event,
                ));
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
            );
            if let Err(error) = result {
                msg_log.push(GameMessage::new(
                    clock.tick,
                    format!("Fleet move rejected: {error}"),
                    MessageCategory::Event,
                ));
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
                msg_log.push(GameMessage::new(
                    clock.tick,
                    format!("Fleet move rejected: {error}"),
                    MessageCategory::Event,
                ));
                return;
            }
            if let Err(error) = troop_transport_state.embark(world, fleet, &troops) {
                if !troops.is_empty() {
                    msg_log.push(GameMessage::new(
                        clock.tick,
                        format!("Troop embarkation rejected: {error}"),
                        MessageCategory::Event,
                    ));
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
                Ok(departure) => {
                    let origin_name = world
                        .systems
                        .get(departure.origin)
                        .map_or("Unknown", |system| system.name.as_str());
                    let destination_name = world
                        .systems
                        .get(departure.destination)
                        .map_or("Unknown", |system| system.name.as_str());
                    msg_log.push(GameMessage::at_system(
                        clock.tick,
                        format!(
                            "Fleet departed {} for {} ({} days)",
                            origin_name, destination_name, departure.transit_ticks,
                        ),
                        MessageCategory::Event,
                        departure.destination,
                    ));
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
                        let _ =
                            troop_transport_state.disembark_selected(world, fleet, origin, &troops);
                    }
                    msg_log.push(GameMessage::new(
                        clock.tick,
                        format!("Fleet move rejected: {error}"),
                        MessageCategory::Event,
                    ));
                }
            }
        }
        PanelAction::StopProduction { system, area } => {
            mfg_state.stop(system, area);
            macroquad::logging::info!(
                "[interface] command=0x213 destination=production_stop status=stopped area={area:?} system={}",
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
            if let Some(items) = rebellion_core::build_selection::order_items(
                world,
                system,
                area,
                kind,
                count,
                is_alliance,
            ) {
                mfg_state.build_units(system, items);
                macroquad::logging::info!(
                    "[interface] command=0x21x destination=production_build status=queued area={area:?} count={count} system={}",
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
                target_object: None,
                tick,
            };
            if let Err(refusal) = mission_state.dispatch_guarded(request, world) {
                // FUN_00487c90 shows no text: the side's advisor reacts
                // (`mission-dialog.md`, "Refusal").
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
        | PanelAction::OpenSaveLoad
        | PanelAction::SaveGame { .. }
        | PanelAction::LoadGame { .. }
        | PanelAction::DeleteSave { .. }
        | PanelAction::CloseSaveLoadPanel
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
        PanelAction::OrderBombardment { fleet, system } => {
            // Guard: both fleet and system must still exist (prevents panic in resolve).
            if world.fleets.contains_key(fleet) && world.systems.contains_key(system) {
                let attacker_is_alliance = world.fleets[fleet].is_alliance;
                let result = BombardmentSystem::resolve_bombardment(
                    world,
                    fleet,
                    system,
                    world.difficulty_index,
                    clock.tick,
                );
                let attacker = if attacker_is_alliance {
                    Faction::Alliance
                } else {
                    Faction::Empire
                };
                let headquarters_destroyed = VictorySystem::apply_headquarters_bombardment(
                    victory_state,
                    world,
                    &result,
                    attacker,
                );
                if let Some(sys) = world.systems.get_mut(system) {
                    let pop_reduction = (result.damage as f32 / 100.0).min(0.25);
                    if attacker_is_alliance {
                        sys.popularity_empire =
                            (sys.popularity_empire - pop_reduction).clamp(0.0, 1.0);
                    } else {
                        sys.popularity_alliance =
                            (sys.popularity_alliance - pop_reduction).clamp(0.0, 1.0);
                    }
                }
                msg_log.push(GameMessage::new(
                    clock.tick,
                    format!("Orbital bombardment — {} damage", result.damage),
                    MessageCategory::Combat,
                ));
                if headquarters_destroyed {
                    let system_name = world
                        .systems
                        .get(system)
                        .map_or("Unknown", |system| system.name.as_str());
                    // Notification 0x1f, Rebel HQ Destroyed.
                    msg_log.push(filed(
                        GameMessage::new(
                            clock.tick,
                            format!("Alliance headquarters destroyed at {system_name}"),
                            MessageCategory::Combat,
                        ),
                        MessageRail::Resource,
                        Some(RailAudience::Both),
                    ));
                }
            }
        }
        PanelAction::FireDeathStar { system } => {
            // Use DeathStarSystem::fire() for precondition validation (guards from Ghidra RE).
            if let Some(rebellion_core::death_star::DeathStarEvent::PlanetDestroyed { .. }) =
                DeathStarSystem::fire(death_star_state, world, system, clock.tick)
            {
                let name = world
                    .systems
                    .get(system)
                    .map_or_else(|| "Unknown".to_string(), |s| s.name.clone());
                if let Some(sys) = world.systems.get_mut(system) {
                    sys.is_destroyed = true;
                }
                victory_state.death_star_location = Some(system);
                // Drain the telemetry out-param into the message log so
                // interactive play surfaces the killed characters
                // immediately; the run_simulation_tick flow does the same
                // via the `PerceptionIntegrator`.
                let mut cleanup_effects: Vec<rebellion_core::effects::GameEffect> = Vec::new();
                rebellion_core::death_star::cleanup_destroyed_system(
                    world,
                    system,
                    movement_state,
                    death_star_state,
                    mfg_state,
                    blockade_state,
                    &mut cleanup_effects,
                );
                for effect in cleanup_effects.drain(..) {
                    if let rebellion_core::effects::GameEffect::CharacterKilled { character } =
                        effect
                    {
                        if let Some(c) = world.characters.get(character) {
                            // Notification 0x1a, Character Health.
                            msg_log.push(filed(
                                GameMessage::new(
                                    clock.tick,
                                    format!("{} has been killed.", c.display_name()),
                                    MessageCategory::Event,
                                ),
                                MessageRail::Mission,
                                character_audience(&world, character),
                            ));
                        }
                    }
                }
                // Notification 0x15, Planet Destroyed.
                msg_log.push(filed(
                    GameMessage::new(
                        clock.tick,
                        format!("{name} DESTROYED by Death Star superlaser!"),
                        MessageCategory::Combat,
                    ),
                    MessageRail::Mission,
                    Some(RailAudience::Both),
                ));
            }
        }
        PanelAction::MoveDeathStar { system } => {
            // Issue a movement order for the Death Star fleet to the target system.
            if let Some(fleet_key) = death_star_state.death_star_fleet {
                // Don't issue if already in transit.
                if movement_state.get(fleet_key).is_some() {
                    msg_log.push(GameMessage::new(
                        clock.tick,
                        "Death Star fleet is already in transit".to_string(),
                        MessageCategory::Event,
                    ));
                } else if let Some(fleet) = world.fleets.get(fleet_key) {
                    let origin = fleet.location;
                    let ticks =
                        rebellion_core::movement::fleet_transit_ticks(fleet, world, origin, system);
                    if origin != system && ticks.is_none() {
                        msg_log.push(GameMessage::new(
                            clock.tick,
                            "Death Star fleet cannot enter hyperspace".to_string(),
                            MessageCategory::Event,
                        ));
                    } else if let (true, Some(ticks)) = (origin != system, ticks) {
                        let dest_name = world
                            .systems
                            .get(system)
                            .map_or_else(|| "Unknown".to_string(), |s| s.name.clone());
                        if begin_fleet_transit(movement_state, world, fleet_key, system, ticks) {
                            msg_log.push(GameMessage::new(
                                clock.tick,
                                format!("Death Star fleet moving to {dest_name} ({ticks} days)"),
                                MessageCategory::Event,
                            ));
                        }
                    }
                }
            }
        }
        PanelAction::AdvanceTicks(n) => {
            // Force-advance N ticks synchronously.
            // NOTE: This only advances the clock counter. Full simulation tick
            // execution requires run_simulation_tick() which needs &mut access to
            // all states — not available inside apply_panel_action(). The actual
            // simulation will catch up on the next frame when clock.advance(dt)
            // emits the pending TickEvents. For instant effect, set speed to Faster.
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
            MissionEffect::TargetSabotaged { target, .. } => {
                rebellion_core::missions::destroy_target(world, *target);
                if matches!(
                    target,
                    rebellion_core::missions::MissionTarget::DeathStar(_)
                ) {
                    // Notification 0x23, Death Star Sabotaged.
                    log.push(filed(
                        GameMessage::new(
                            result.tick,
                            "The Death Star has been sabotaged.".to_string(),
                            MessageCategory::Mission,
                        ),
                        MessageRail::Mission,
                        Some(RailAudience::side(
                            result.faction == MissionFaction::Alliance,
                        )),
                    ));
                }
            }
            MissionEffect::CharacterKilled { character, .. } => {
                // Remove character from any fleet they're assigned to
                for (_, fleet) in &mut world.fleets {
                    fleet.characters.retain(|&k| k != *character);
                }
                world.characters.remove(*character);
            }
            MissionEffect::CharacterCaptured {
                character,
                captured_by,
                ..
            } => {
                // Set captivity state — do NOT flip is_alliance/is_empire.
                // Those fields encode faction *identity*, not current holder.
                // Flipping them corrupts escape direction (check_escapes uses
                // is_alliance to determine where the character escapes TO).
                if let Some(c) = world.characters.get_mut(*character) {
                    c.is_captive = true;
                    c.captured_by = Some(match captured_by {
                        MissionFaction::Alliance => Faction::Alliance,
                        MissionFaction::Empire => Faction::Empire,
                    });
                    c.capture_tick = Some(result.tick);
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
            MissionEffect::CharacterEscaped {
                character,
                escaped_to_alliance,
            } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.is_alliance = *escaped_to_alliance;
                    c.is_empire = !*escaped_to_alliance;
                    c.is_captive = false;
                    c.captured_by = None;
                    c.capture_tick = None;
                }
                let name = world
                    .characters
                    .get(*character)
                    .map_or_else(|| "Unknown".into(), |c| c.display_name().to_owned());
                // FUN_00490340: the escape carries the character's code.
                log.push(
                    GameMessage::new(
                        result.tick,
                        format!("{name} has escaped captivity!"),
                        MessageCategory::Event,
                    )
                    .with_advice_to(
                        escape_advice(world, *character),
                        RailAudience::side(*escaped_to_alliance),
                    ),
                );
            }
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

#[expect(
    clippy::too_many_arguments,
    reason = "Keep explicit state and UI inputs at this existing integration boundary."
)]
#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
fn apply_ai_actions(
    actions: &[AIAction],
    ai_state: &mut AIState,
    mission_state: &mut MissionState,
    mfg_state: &mut ManufacturingState,
    movement_state: &mut MovementState,
    troop_transport_state: &mut TroopTransportState,
    research_state: &mut ResearchState,
    world: &mut GameWorld,
    log: &mut MessageLog,
    tick: u64,
    #[cfg(not(target_arch = "wasm32"))] audio_engine: &mut audio::AudioEngine,
    #[cfg(not(target_arch = "wasm32"))] audio_vol: &AudioVolumeState,
) {
    for action in actions {
        match action {
            AIAction::DispatchMission {
                kind,
                team,
                decoys,
                target_system,
                target_character,
                target_object,
            } => {
                let ai_faction = ai_state.faction.unwrap_or(AiFaction::Empire);
                let request = rebellion_core::missions::MissionRequest {
                    kind: *kind,
                    faction: ai_faction.as_mission_faction(),
                    team: team.clone(),
                    decoys: decoys.clone(),
                    target_system: *target_system,
                    target_character: *target_character,
                    target_object: *target_object,
                    tick,
                };
                if mission_state.dispatch_guarded(request, world).is_err() {
                    continue;
                }
                for character in team.iter().chain(decoys).filter_map(|m| m.character()) {
                    ai_state.mark_busy(character);
                }
                let faction_name = match ai_faction {
                    AiFaction::Alliance => "Alliance",
                    AiFaction::Empire => "Empire",
                };
                log.push(GameMessage::at_system(
                    tick,
                    format!("{faction_name} dispatched {kind:?} mission"),
                    MessageCategory::Ai,
                    *target_system,
                ));
            }
            AIAction::EnqueueProduction { system, kind, work } => {
                mfg_state.enqueue(*system, QueueItem::new(*kind, *work));
            }
            AIAction::MoveFleet {
                fleet,
                to_system,
                reason,
                troops,
            } => {
                let transit = world.fleets.get(*fleet).and_then(|fleet| {
                    rebellion_core::movement::fleet_transit_ticks(
                        fleet,
                        world,
                        fleet.location,
                        *to_system,
                    )
                    .map(|ticks| (ticks, fleet.is_alliance))
                });
                if let Some((transit, is_alliance)) = transit {
                    let embarked = troops.is_empty()
                        || troop_transport_state.embark(world, *fleet, troops).is_ok();
                    if embarked
                        && begin_fleet_transit(movement_state, world, *fleet, *to_system, transit)
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            audio_engine.play_sfx(SfxKind::FleetDeparture, audio_vol);
                        }
                        let reason_str = match reason {
                            FleetMoveReason::Attack => "attack",
                            FleetMoveReason::Reinforce => "reinforce",
                        };
                        log.push(GameMessage::at_system(
                            tick,
                            format!(
                                "{} fleet moving to system ({}){}",
                                if is_alliance { "Alliance" } else { "Empire" },
                                reason_str,
                                if troops.is_empty() {
                                    String::new()
                                } else {
                                    format!(" with {} regiment(s)", troops.len())
                                },
                            ),
                            MessageCategory::Ai,
                            *to_system,
                        ));
                    } else if embarked && !troops.is_empty() {
                        let origin = world.fleets.get(*fleet).map(|value| value.location);
                        if let Some(origin) = origin {
                            let _ = troop_transport_state.disembark_all(world, *fleet, origin);
                        }
                    }
                }
            }
            AIAction::DispatchResearch {
                character,
                tech_type,
                ticks,
            } => {
                let is_alliance = ai_state
                    .faction
                    .is_some_and(|f| matches!(f, AiFaction::Alliance));
                research_state.dispatch(rebellion_core::research::ResearchProject {
                    tech_type: *tech_type,
                    character: *character,
                    faction_is_alliance: is_alliance,
                    ticks_remaining: *ticks,
                    total_ticks: *ticks,
                });
                ai_state.mark_busy(*character);
                let char_name = world
                    .characters
                    .get(*character)
                    .map_or("unknown", |c| c.display_name());
                log.push(GameMessage::new(
                    tick,
                    format!("{char_name} assigned to {tech_type:?} research ({ticks} ticks)"),
                    MessageCategory::Ai,
                ));
            }
        }
    }
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

fn enqueue_sector_window(
    strategic_windowing: &mut strategic_windowing::StrategicWindowing,
    state: &SectorWindowState,
    world: &GameWorld,
    system: SystemKey,
    faction: CockpitFaction,
    pending: &mut Vec<PendingSectorPublication>,
    dependent: Option<SectorDependentIntent>,
) -> bool {
    let Some(sector) = world.systems.get(system).map(|value| value.sector) else {
        return false;
    };
    if rebellion_render::windowing_policy::sector_subject(world, sector).is_none() {
        return false;
    }
    pending.push(PendingSectorPublication {
        system,
        faction,
        prepared: None,
        dependent,
    });
    pump_pending_sector_publication(strategic_windowing, state, world, pending);
    true
}

/// Queues at most one prepared Sector publication. Later requests remain as
/// intents until the exact earlier receipt has committed, so every prepare
/// observes the renderer state produced by its predecessor.
fn pump_pending_sector_publication(
    strategic_windowing: &mut strategic_windowing::StrategicWindowing,
    state: &SectorWindowState,
    world: &GameWorld,
    pending: &mut Vec<PendingSectorPublication>,
) -> bool {
    if pending.iter().any(|candidate| candidate.prepared.is_some()) {
        return false;
    }
    while let Some(candidate) = pending.first_mut() {
        if let Some(prepared) = state.prepare_open(world, candidate.system, candidate.faction) {
            candidate.prepared = Some(prepared);
            strategic_windowing.enqueue_sector_open(prepared);
            return true;
        }
        pending.remove(0);
    }
    false
}

/// Refresh the exact in-flight Sector declaration against the renderer state
/// after all routed input, before any manager Open/Replace can remove content.
fn refresh_pending_sector_publication(
    strategic_windowing: &mut strategic_windowing::StrategicWindowing,
    state: &SectorWindowState,
    world: &GameWorld,
    pending: &mut Vec<PendingSectorPublication>,
) {
    let Some(index) = pending
        .iter()
        .position(|candidate| candidate.prepared.is_some())
    else {
        return;
    };
    let candidate = pending[index];
    let previous = candidate.prepared.expect("prepared candidate");
    let refreshed = state.prepare_open(world, candidate.system, candidate.faction);
    if refreshed == Some(previous) {
        return;
    }
    if strategic_windowing.refresh_queued_sector_open(previous, refreshed) {
        if let Some(refreshed) = refreshed {
            pending[index].prepared = Some(refreshed);
        } else {
            pending.remove(index);
            pump_pending_sector_publication(strategic_windowing, state, world, pending);
        }
    } else {
        // The manager no longer has this exact candidate. Never publish its
        // dependent intent using a receipt from a different queued command.
        pending.remove(index);
    }
}

fn enqueue_system_window(
    strategic_windowing: &mut strategic_windowing::StrategicWindowing,
    state: &SystemWindowState,
    world: &GameWorld,
    system: SystemKey,
    logical_position: (i16, i16),
    faction: CockpitFaction,
    layout: rebellion_render::CockpitLayout,
) -> bool {
    let Some(prepared) = state.prepare_open(world, system, logical_position, faction, layout)
    else {
        return false;
    };
    strategic_windowing.enqueue_system_open(prepared);
    true
}

fn enqueue_galaxy_system_window(
    strategic_windowing: &mut strategic_windowing::StrategicWindowing,
    state: &SystemWindowState,
    world: &GameWorld,
    system: SystemKey,
    logical_position: (i16, i16),
    faction: CockpitFaction,
    layout: rebellion_render::CockpitLayout,
) -> bool {
    let Some(prepared) = state.prepare_open(world, system, logical_position, faction, layout)
    else {
        return false;
    };
    strategic_windowing.enqueue_galaxy_system_open(prepared);
    true
}

fn enqueue_defenses_window(
    strategic_windowing: &mut strategic_windowing::StrategicWindowing,
    state: &DefensesWindowState,
    world: &GameWorld,
    system: SystemKey,
    logical_position: (i16, i16),
    faction: CockpitFaction,
    layout: rebellion_render::CockpitLayout,
) -> bool {
    let Some(prepared) = state.prepare_open(world, system, logical_position, faction, layout)
    else {
        return false;
    };
    strategic_windowing.enqueue_defenses_open(prepared);
    true
}

/// Publishes prepared Missions content only after the application observes an
/// exact accepted Open/Existing receipt. A failed new publication compensates
/// the manager identity allocated by that receipt and no preserved identity.
fn commit_missions_window_publication(
    strategic_windowing: &mut strategic_windowing::StrategicWindowing,
    state: &mut MissionsWindowState,
    prepared: Option<PreparedMissionsWindow>,
    expected_id: Option<rebellion_windowing::WindowId>,
    result: &Result<rebellion_windowing::WindowCommit, rebellion_windowing::WindowError>,
) -> (bool, Vec<rebellion_windowing::WindowId>) {
    let Some(prepared) = prepared else {
        return (false, Vec::new());
    };
    let Ok(commit) = result else {
        return (false, Vec::new());
    };
    let receipt_id = match &commit.transition {
        rebellion_windowing::WindowTransition::Opened { id }
        | rebellion_windowing::WindowTransition::Existing { id, .. } => Some(*id),
        _ => None,
    };
    let Some(id) = expected_id.filter(|id| receipt_id == Some(*id)) else {
        macroquad::logging::error!(
            "[windowing] Missions content receipt mismatch expected={expected_id:?} actual={receipt_id:?}"
        );
        let removed = match &commit.transition {
            rebellion_windowing::WindowTransition::Opened { id } => {
                strategic_windowing.rollback_detail_publication(*id)
            }
            _ => Vec::new(),
        };
        return (false, removed);
    };
    if state.commit_open(prepared, &commit.transition) != Some(id) {
        macroquad::logging::error!(
            "[windowing] Missions content commit did not match identity={id:?}"
        );
        let removed = match &commit.transition {
            rebellion_windowing::WindowTransition::Opened { id } => {
                strategic_windowing.rollback_detail_publication(*id)
            }
            _ => Vec::new(),
        };
        return (false, removed);
    }

    let committed = strategic_windowing.has_content(id)
        || strategic_windowing.acknowledge_missions_publication(id);
    if committed {
        return (true, Vec::new());
    }
    state.remove_content(&[id]);
    (false, strategic_windowing.rollback_detail_publication(id))
}

#[expect(
    clippy::too_many_arguments,
    reason = "The lifecycle adapter carries the existing Missions opener's complete content context."
)]
fn enqueue_missions_window(
    strategic_windowing: &mut strategic_windowing::StrategicWindowing,
    state: &MissionsWindowState,
    world: &GameWorld,
    fog: &rebellion_core::fog::FogState,
    missions: &rebellion_core::missions::MissionState,
    system: SystemKey,
    logical_position: (i16, i16),
    faction: CockpitFaction,
    layout: rebellion_render::CockpitLayout,
) -> bool {
    let Some(prepared) = state.prepare_open(
        world,
        fog,
        missions,
        system,
        logical_position,
        faction,
        layout,
    ) else {
        return false;
    };
    strategic_windowing.enqueue_missions_open(prepared);
    true
}

#[expect(
    clippy::too_many_arguments,
    reason = "Finder publication carries member selection in the same prepared Missions content."
)]
fn enqueue_missions_window_for_member(
    strategic_windowing: &mut strategic_windowing::StrategicWindowing,
    state: &MissionsWindowState,
    world: &GameWorld,
    fog: &rebellion_core::fog::FogState,
    missions: &rebellion_core::missions::MissionState,
    system: SystemKey,
    logical_position: (i16, i16),
    faction: CockpitFaction,
    layout: rebellion_render::CockpitLayout,
    member: rebellion_core::missions::MissionMember,
) -> bool {
    let Some(prepared) = state.prepare_open_for_member(
        world,
        fog,
        missions,
        system,
        logical_position,
        faction,
        layout,
        member,
    ) else {
        return false;
    };
    strategic_windowing.enqueue_missions_open(prepared);
    true
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

#[expect(
    clippy::too_many_arguments,
    reason = "The Results adapter carries the existing Fleet opener's complete content context."
)]
fn enqueue_galaxy_fleet_window(
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
    strategic_windowing.enqueue_galaxy_fleet_open(prepared);
    true
}

fn galaxy_front(
    system_serial: u64,
    fleet_serial: u64,
    defenses_serial: u64,
    missions_serial: u64,
) -> GalaxyFront {
    [
        (system_serial, GalaxyFront::System),
        (fleet_serial, GalaxyFront::Fleet),
        (defenses_serial, GalaxyFront::Defenses),
        (missions_serial, GalaxyFront::Missions),
    ]
    .into_iter()
    .max_by_key(|&(serial, _)| serial)
    .map_or(GalaxyFront::System, |(_, front)| front)
}

/// Resolves the legacy system-keyed Defenses rail action only when the exact
/// candidate remains a minimized Defenses identity for the same stable DAT
/// subject in this frame's frozen manager snapshot.
fn exact_minimized_defenses_restore(
    snapshot: &rebellion_windowing::RoutingSnapshot,
    candidate: Option<rebellion_windowing::WindowId>,
    expected_subject: Option<rebellion_windowing::SubjectId>,
) -> Option<rebellion_windowing::WindowId> {
    let candidate = candidate?;
    let expected_subject = expected_subject?;
    (snapshot.presentation_history().contains(&candidate)
        && snapshot.key(candidate).is_some_and(|key| {
            key.kind == rebellion_windowing::WindowKind::Defenses
                && key.subject == Some(expected_subject)
        }))
    .then_some(candidate)
}

/// Resolves the legacy system-keyed Missions rail action only when the exact
/// candidate remains a minimized Missions identity for the same stable DAT
/// subject in this frame's frozen manager snapshot.
fn exact_minimized_missions_restore(
    snapshot: &rebellion_windowing::RoutingSnapshot,
    candidate: Option<rebellion_windowing::WindowId>,
    expected_subject: Option<rebellion_windowing::SubjectId>,
) -> Option<rebellion_windowing::WindowId> {
    let candidate = candidate?;
    let expected_subject = expected_subject?;
    (snapshot.presentation_history().contains(&candidate)
        && snapshot.key(candidate).is_some_and(|key| {
            key.kind == rebellion_windowing::WindowKind::Missions
                && key.subject == Some(expected_subject)
        }))
    .then_some(candidate)
}

const fn visible_legacy_bridge_serial(serial: u64, exact_content_visible: bool) -> u64 {
    if exact_content_visible {
        serial
    } else {
        0
    }
}

#[cfg(test)]
mod detail_restore_guard_tests {
    use super::{
        exact_minimized_defenses_restore, exact_minimized_missions_restore, galaxy_front,
        visible_legacy_bridge_serial, GalaxyFront,
    };
    use rebellion_render::windowing_policy::contextual_spec;
    use rebellion_windowing::{
        SubjectId, WindowCommand, WindowKind, WindowManager, WindowOwner, WindowTransition,
    };

    fn open(
        manager: &mut WindowManager,
        kind: WindowKind,
        subject: SubjectId,
    ) -> rebellion_windowing::WindowId {
        let spec = contextual_spec(kind, subject, WindowOwner::StrategicRoot);
        match manager.apply(WindowCommand::Open(spec)).unwrap().transition {
            WindowTransition::Opened { id } => id,
            transition => panic!("expected open, got {transition:?}"),
        }
    }

    #[test]
    fn stale_defenses_rail_does_not_retarget_a_visible_replacement() {
        let expected = SubjectId::new(2, 100);
        let mut manager = WindowManager::new();
        let stale = open(&mut manager, WindowKind::Defenses, expected);
        manager.apply(WindowCommand::Minimize(stale)).unwrap();
        assert_eq!(
            exact_minimized_defenses_restore(&manager.begin_frame(), Some(stale), Some(expected)),
            Some(stale)
        );

        manager.apply(WindowCommand::Close(stale)).unwrap();
        let replacement = open(&mut manager, WindowKind::Defenses, expected);
        assert_ne!(replacement, stale);
        assert_eq!(
            exact_minimized_defenses_restore(
                &manager.begin_frame(),
                Some(replacement),
                Some(expected),
            ),
            None,
            "a system-keyed stale rail action cannot target visible replacement content"
        );

        let mut wrong_kind = WindowManager::new();
        let system = open(&mut wrong_kind, WindowKind::System, expected);
        wrong_kind.apply(WindowCommand::Minimize(system)).unwrap();
        assert_eq!(
            exact_minimized_defenses_restore(
                &wrong_kind.begin_frame(),
                Some(system),
                Some(expected),
            ),
            None
        );

        let mut wrong_subject = WindowManager::new();
        let other = SubjectId::new(2, 101);
        let defenses = open(&mut wrong_subject, WindowKind::Defenses, other);
        wrong_subject
            .apply(WindowCommand::Minimize(defenses))
            .unwrap();
        assert_eq!(
            exact_minimized_defenses_restore(
                &wrong_subject.begin_frame(),
                Some(defenses),
                Some(expected),
            ),
            None
        );
    }

    #[test]
    fn defenses_bridge_serial_contributes_only_with_exact_visible_content() {
        assert_eq!(
            galaxy_front(7, 6, visible_legacy_bridge_serial(99, false), 0),
            GalaxyFront::System
        );
        assert_eq!(
            galaxy_front(7, 6, visible_legacy_bridge_serial(99, true), 0),
            GalaxyFront::Defenses
        );
    }

    #[test]
    fn stale_missions_rail_does_not_retarget_a_visible_replacement() {
        let expected = SubjectId::new(2, 100);
        let mut manager = WindowManager::new();
        let stale = open(&mut manager, WindowKind::Missions, expected);
        manager.apply(WindowCommand::Minimize(stale)).unwrap();
        assert_eq!(
            exact_minimized_missions_restore(&manager.begin_frame(), Some(stale), Some(expected)),
            Some(stale)
        );

        manager.apply(WindowCommand::Close(stale)).unwrap();
        let replacement = open(&mut manager, WindowKind::Missions, expected);
        assert_ne!(replacement, stale);
        assert_eq!(
            exact_minimized_missions_restore(
                &manager.begin_frame(),
                Some(replacement),
                Some(expected),
            ),
            None,
            "a system-keyed stale rail action cannot target visible replacement content"
        );

        let mut wrong_kind = WindowManager::new();
        let defenses = open(&mut wrong_kind, WindowKind::Defenses, expected);
        wrong_kind.apply(WindowCommand::Minimize(defenses)).unwrap();
        assert_eq!(
            exact_minimized_missions_restore(
                &wrong_kind.begin_frame(),
                Some(defenses),
                Some(expected),
            ),
            None
        );

        let mut wrong_subject = WindowManager::new();
        let other = SubjectId::new(2, 101);
        let missions = open(&mut wrong_subject, WindowKind::Missions, other);
        wrong_subject
            .apply(WindowCommand::Minimize(missions))
            .unwrap();
        assert_eq!(
            exact_minimized_missions_restore(
                &wrong_subject.begin_frame(),
                Some(missions),
                Some(expected),
            ),
            None
        );
    }

    #[test]
    fn missions_bridge_serial_contributes_only_with_exact_visible_content() {
        assert_eq!(
            galaxy_front(7, 6, 5, visible_legacy_bridge_serial(99, false)),
            GalaxyFront::System
        );
        assert_eq!(
            galaxy_front(7, 6, 5, visible_legacy_bridge_serial(99, true)),
            GalaxyFront::Missions
        );
    }
}

#[cfg(test)]
mod missions_publication_tests {
    use super::{
        commit_missions_window_publication, enqueue_missions_window,
        enqueue_missions_window_for_member,
    };
    use rebellion_core::dat::{ExplorationStatus, Faction};
    use rebellion_core::fog::FogState;
    use rebellion_core::ids::{CharacterKey, DatId, SystemKey};
    use rebellion_core::missions::{
        MissionFaction, MissionKind, MissionMember, MissionRequest, MissionState,
    };
    use rebellion_core::world::{Character, ControlKind, GameWorld, System};
    use rebellion_render::cockpit::{CockpitFaction, CockpitLayout, CockpitViewport};
    use rebellion_render::missions_window::MissionsWindowState;
    use rebellion_render::windowing_policy::contextual_spec;
    use rebellion_windowing::{
        SubjectId, WindowCommand, WindowError, WindowKind, WindowManager, WindowOwner,
        WindowTransition,
    };

    fn fixture() -> (GameWorld, SystemKey, MissionState, [CharacterKey; 2]) {
        let mut world = GameWorld::default();
        let system = world.systems.insert(System {
            dat_id: DatId::new(100),
            name: "Sluis Van".into(),
            sector: Default::default(),
            x: 320,
            y: 250,
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
            fighters: Vec::new(),
            defense_facilities: Vec::new(),
            manufacturing_facilities: Vec::new(),
            production_facilities: Vec::new(),
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Alliance),
        });
        let mut character = |name: &str| {
            world.characters.insert(Character {
                dat_id: DatId::new(832),
                name: name.into(),
                is_alliance: true,
                current_system: Some(system),
                recruited: true,
                ..Default::default()
            })
        };
        let first = character("Leia");
        let second = character("Mon");
        let mut missions = MissionState::new();
        for (kind, member) in [
            (MissionKind::Diplomacy, first),
            (MissionKind::Espionage, second),
        ] {
            missions.dispatch(MissionRequest {
                kind,
                faction: MissionFaction::Alliance,
                team: vec![MissionMember::Character(member)],
                decoys: Vec::new(),
                target_system: system,
                target_character: None,
                target_object: None,
                tick: 0,
            });
        }
        (world, system, missions, [first, second])
    }

    fn layout() -> CockpitLayout {
        let viewport = CockpitViewport {
            x: 0.0,
            y: 0.0,
            width: 640.0,
            height: 480.0,
        };
        CockpitLayout {
            canvas: viewport,
            galaxy: viewport,
            scale: 1.0,
        }
    }

    fn unrelated_second_id() -> rebellion_windowing::WindowId {
        let mut manager = WindowManager::new();
        let open = |manager: &mut WindowManager, kind, subject| match manager
            .apply(WindowCommand::Open(contextual_spec(
                kind,
                subject,
                WindowOwner::StrategicRoot,
            )))
            .unwrap()
            .transition
        {
            WindowTransition::Opened { id } => id,
            transition => panic!("expected open, got {transition:?}"),
        };
        let _ = open(&mut manager, WindowKind::System, SubjectId::new(2, 900));
        open(&mut manager, WindowKind::Defenses, SubjectId::new(2, 901))
    }

    #[test]
    fn finder_publication_requires_exact_receipt_and_rolls_back_only_new_identity() {
        let (world, system, missions, [first, second]) = fixture();
        let fog = FogState::new(Faction::Alliance);
        let mut coordinator = super::strategic_windowing::StrategicWindowing::new();
        let mut state = MissionsWindowState::default();
        assert!(enqueue_missions_window(
            &mut coordinator,
            &state,
            &world,
            &fog,
            &missions,
            system,
            (20, 30),
            CockpitFaction::Alliance,
            layout(),
        ));
        assert!(
            state.report(&world, &fog, &missions, system).is_none(),
            "queued open intent is inert before the coordinator commit"
        );
        let mut opened = coordinator.apply_pending().pop().unwrap();
        let id = opened.missions_content_id.unwrap();
        let (committed, removed) = commit_missions_window_publication(
            &mut coordinator,
            &mut state,
            opened.prepared_missions.take(),
            opened.missions_content_id,
            &opened.result,
        );
        assert!(committed);
        assert!(removed.is_empty());
        assert_eq!(
            state
                .report(&world, &fog, &missions, system)
                .unwrap()
                .selected,
            Some(0)
        );

        assert!(enqueue_missions_window_for_member(
            &mut coordinator,
            &state,
            &world,
            &fog,
            &missions,
            system,
            (300, 200),
            CockpitFaction::Alliance,
            layout(),
            MissionMember::Character(second),
        ));
        assert_eq!(
            state
                .report(&world, &fog, &missions, system)
                .unwrap()
                .selected,
            Some(0),
            "Finder intent cannot mutate published selection before commit"
        );
        let mut existing = coordinator.apply_pending().pop().unwrap();
        assert!(matches!(
            existing.result.as_ref().unwrap().transition,
            WindowTransition::Existing { id: existing, .. } if existing == id
        ));
        let prepared = existing.prepared_missions.take();
        let wrong = unrelated_second_id();
        assert_ne!(wrong, id);
        let (committed, removed) = commit_missions_window_publication(
            &mut coordinator,
            &mut state,
            prepared,
            Some(wrong),
            &existing.result,
        );
        assert!(!committed);
        assert!(removed.is_empty());
        assert_eq!(
            state
                .report(&world, &fog, &missions, system)
                .unwrap()
                .selected,
            Some(0),
            "wrong Existing identity must be rejected before Finder selection mutates"
        );
        let (committed, removed) = commit_missions_window_publication(
            &mut coordinator,
            &mut state,
            prepared,
            existing.missions_content_id,
            &existing.result,
        );
        assert!(committed);
        assert!(removed.is_empty());
        assert_eq!(
            state
                .report(&world, &fog, &missions, system)
                .unwrap()
                .selected,
            Some(1),
            "the exact Existing receipt releases Finder selection"
        );

        let rejected = state
            .prepare_open_for_member(
                &world,
                &fog,
                &missions,
                system,
                (400, 250),
                CockpitFaction::Alliance,
                layout(),
                MissionMember::Character(first),
            )
            .unwrap();
        let rejection = Err(WindowError::UnknownWindow(id));
        let (committed, removed) = commit_missions_window_publication(
            &mut coordinator,
            &mut state,
            Some(rejected),
            Some(id),
            &rejection,
        );
        assert!(!committed);
        assert!(removed.is_empty());
        assert_eq!(
            state
                .report(&world, &fog, &missions, system)
                .unwrap()
                .selected,
            Some(1),
            "a rejected receipt leaves published content unchanged"
        );

        let mut rollback_coordinator = super::strategic_windowing::StrategicWindowing::new();
        let mut occupied = MissionsWindowState::default();
        assert!(occupied.open(
            &world,
            &fog,
            &missions,
            system,
            (20, 30),
            CockpitFaction::Alliance,
            layout(),
        ));
        assert!(enqueue_missions_window(
            &mut rollback_coordinator,
            &occupied,
            &world,
            &fog,
            &missions,
            system,
            (20, 30),
            CockpitFaction::Alliance,
            layout(),
        ));
        let mut opened = rollback_coordinator.apply_pending().pop().unwrap();
        let opened_id = opened.missions_content_id.unwrap();
        let (committed, removed) = commit_missions_window_publication(
            &mut rollback_coordinator,
            &mut occupied,
            opened.prepared_missions.take(),
            opened.missions_content_id,
            &opened.result,
        );
        assert!(!committed);
        assert_eq!(removed, [opened_id]);
        assert!(rollback_coordinator.begin_frame().key(opened_id).is_none());
        assert_eq!(occupied.window_count(), 1);
        assert!(!occupied.contains_window(opened_id));
    }
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
/// no blockading fleet (`FUN_0052bed0`), files nothing; the port logs
/// `fallback` off the rail. The original also files nothing when system
/// flag `+0x22 & 0x4000` is set; port: that flag is untraced and not
/// checked.
fn blockade_messages(
    world: &GameWorld,
    templates: &rebellion_data::text_templates::TextTemplates,
    system: SystemKey,
    tick: u64,
    fallback: String,
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
        return vec![GameMessage::at_system(
            tick,
            fallback,
            MessageCategory::Combat,
            system,
        )];
    };
    let place = TemplateArg::named(sys.name.clone());
    let enemy = TemplateArg::side(!holder_is_alliance);
    let fleet = TemplateArg::named(blockader);
    let background = if holder_is_alliance { 0x404 } else { 0x403 };
    [true, false]
        .into_iter()
        .map(|side_is_alliance| {
            let holds = side_is_alliance == holder_is_alliance;
            let (title, body, advice) = if holds {
                (0x70a2, 0x70a3, 0xe)
            } else {
                (0x70a0, 0x70a1, 0xd)
            };
            let display = templates
                .format(title, &[Some(&place)])
                .zip(templates.format(body, &[Some(&enemy), Some(&place), Some(&fleet)]))
                .map(|(title, body)| MessageDisplay {
                    title,
                    body,
                    background,
                    overlay: 0,
                    sound: 0x45e,
                });
            let text = display
                .as_ref()
                .map_or_else(|| fallback.clone(), |display| display.title.clone());
            let message = GameMessage::at_system(tick, text, MessageCategory::Combat, system)
                .with_advice(advice)
                .with_target(MessageTarget::System(system))
                .on_rail(MessageRail::Conflict, RailAudience::side(side_is_alliance));
            match display {
                Some(display) => message.with_display(display),
                None => message,
            }
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
    })
}

/// Notification 4, the Loyalty Change Message (`FUN_00499760`, event
/// `0x100`), built for each side over the crowd picture `0x3ed`. The side
/// the world joins reads "| Joins" (`0x7038` / `0x703b`, code 1), the other
/// "| Joins Enemy" (`0x7039` / `0x703c`, code 2), both playing `0x450`; a
/// world turning neutral "| Declares Neutrality" (`0x703a` / `0x703d`,
/// naming the side it left), playing `0x451` with no code. hyp: the port
/// files it where troop and support resolution changes the holder
/// (`FUN_0050b610`); the original's gate (system flag `0x2000`) is
/// untraced.
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

/// The advice code of `character`'s escape (`FUN_00490340`).
fn escape_advice(world: &GameWorld, character: CharacterKey) -> u8 {
    major_character_slot(world, character)
        .map_or(0x24, |slot| [0x26, 0x18, 0x25, 0x19, 0x27, 0x28][slot])
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

/// Play the sound of the message the Message Index just showed
/// (`FUN_00469de0` → `FUN_00610c30`).
fn play_message_sound(engine: &mut audio::AudioEngine, sound: Option<u16>, vol: &AudioVolumeState) {
    let Some(sound) = sound else {
        return;
    };
    if engine.play_message_sound(u32::from(sound), vol) {
        macroquad::logging::info!(
            "[message] sound 0x{sound:x} started volume={:.2}",
            vol.effective_sfx_volume()
        );
    } else {
        macroquad::logging::warn!("[message] sound 0x{:x} is not staged", sound);
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

fn victory_winner_is_alliance(outcome: &rebellion_core::victory::VictoryOutcome) -> bool {
    match outcome {
        rebellion_core::victory::VictoryOutcome::HqCaptured { winner, .. }
        | rebellion_core::victory::VictoryOutcome::HqDestroyed { winner, .. } => {
            *winner == Faction::Alliance
        }
        rebellion_core::victory::VictoryOutcome::DeathStarVictory { .. } => false,
    }
}

const fn trench_run_cutscene_path(outcome: TacticalTrenchRunOutcome) -> &'static str {
    match outcome {
        TacticalTrenchRunOutcome::Success => TRENCH_RUN_SUCCESS_CUTSCENE,
        TacticalTrenchRunOutcome::Failure => TRENCH_RUN_FAILURE_CUTSCENE,
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

fn open_cutscene(
    path: &Path,
    volume: f32,
    msg_log: &mut MessageLog,
    tick: u64,
    #[cfg(not(target_arch = "wasm32"))] audio_engine: &mut audio::AudioEngine,
) -> Option<VideoPlayer> {
    match VideoPlayer::open(path, volume) {
        Ok(player) => {
            // A skipped movie leaves the music playing; nothing would restart it.
            #[cfg(not(target_arch = "wasm32"))]
            audio_engine.stop_music();
            macroquad::logging::info!("[cutscene] opened path={}", path.display());
            Some(player)
        }
        Err(VideoError::NotDecoded { .. }) => {
            let message =
                "cutscene skipped — run scripts/decode-cutscenes.sh to enable".to_string();
            eprintln!("[cutscene] {message}");
            msg_log.push(GameMessage::new(tick, message, MessageCategory::Event));
            None
        }
        Err(error) => {
            let message = format!("cutscene skipped — {error}");
            eprintln!("[cutscene] {message}");
            msg_log.push(GameMessage::new(tick, message, MessageCategory::Event));
            None
        }
    }
}

#[cfg(test)]
mod tactical_ground_tests {
    use super::*;
    use rebellion_core::dat::{ExplorationStatus, SectorGroup};
    use rebellion_core::ids::{CapitalShipKey, DatId, FighterKey};
    use rebellion_core::world::{
        ControlKind, FighterEntry, Fleet, Sector, ShipInstance, System, TroopUnit,
    };

    #[test]
    fn indexed_status_menu_witness_rejects_compacted_ship_and_fighter_rows() {
        let mut world = GameWorld::default();
        let ship_class = CapitalShipKey::default();
        let fighter_a = FighterKey::default();
        let fighter_b = world.fighter_classes.insert(Default::default());
        let fighter_c = world.fighter_classes.insert(Default::default());
        let mut first_ship = ShipInstance::new(ship_class, 100, true);
        first_ship.tag = 101;
        let mut second_ship = ShipInstance::new(ship_class, 100, true);
        second_ship.tag = 102;
        let mut third_ship = ShipInstance::new(ship_class, 100, true);
        third_ship.tag = 103;
        let fleet = world.fleets.insert(Fleet {
            location: SystemKey::default(),
            capital_ships: vec![first_ship, second_ship, third_ship],
            fighters: vec![
                FighterEntry {
                    class: fighter_a,
                    count: 1,
                    carrier: 101,
                },
                FighterEntry {
                    class: fighter_b,
                    count: 1,
                    carrier: 102,
                },
                FighterEntry {
                    class: fighter_c,
                    count: 1,
                    carrier: 103,
                },
            ],
            characters: Vec::new(),
            is_alliance: true,
            has_death_star: false,
        });

        let ship_selection = MenuObject::Ship {
            fleet,
            index: 1,
            roster: rebellion_core::fleet_join::roster(&world, fleet).unwrap(),
        };
        let ship_witness = capture_indexed_status_menu_witness(&world, Some(ship_selection));
        assert_eq!(
            status_object_if_menu_current(&world, ship_selection, ship_witness),
            Some(StatusObject::Ship { fleet, index: 1 })
        );
        world.fleets[fleet].capital_ships.remove(0);
        assert_eq!(
            world.fleets[fleet].capital_ships[1].tag, 103,
            "the raw row now names C instead of the witnessed B"
        );
        assert!(ship_selection.status_object().is_some());
        assert_eq!(
            status_object_if_menu_current(&world, ship_selection, ship_witness),
            None
        );

        let fighter_selection = MenuObject::Fighter { fleet, index: 1 };
        let fighter_witness = capture_indexed_status_menu_witness(&world, Some(fighter_selection));
        assert_eq!(
            status_object_if_menu_current(&world, fighter_selection, fighter_witness),
            Some(StatusObject::Fighter { fleet, index: 1 })
        );
        world.fleets[fleet].fighters.remove(0);
        assert_eq!(
            world.fleets[fleet].fighters[1].class, fighter_c,
            "the raw row now names C instead of the witnessed B"
        );
        assert_eq!(world.fleets[fleet].fighters[1].carrier, 103);
        assert!(fighter_selection.status_object().is_some());
        assert_eq!(
            status_object_if_menu_current(&world, fighter_selection, fighter_witness),
            None
        );
    }

    #[test]
    fn trench_run_outcomes_route_to_their_source_movies() {
        assert_eq!(
            trench_run_cutscene_path(TacticalTrenchRunOutcome::Success),
            "assets/references/ref-videos/201.webm"
        );
        assert_eq!(
            trench_run_cutscene_path(TacticalTrenchRunOutcome::Failure),
            "assets/references/ref-videos/202.webm"
        );
    }

    #[test]
    fn each_victory_outcome_names_its_winning_faction() {
        let system = rebellion_core::ids::SystemKey::default();
        assert!(victory_winner_is_alliance(
            &rebellion_core::victory::VictoryOutcome::HqCaptured {
                winner: Faction::Alliance,
                loser: Faction::Empire,
                hq_system: system,
            }
        ));
        assert!(!victory_winner_is_alliance(
            &rebellion_core::victory::VictoryOutcome::HqCaptured {
                winner: Faction::Empire,
                loser: Faction::Alliance,
                hq_system: system,
            }
        ));
        assert!(!victory_winner_is_alliance(
            &rebellion_core::victory::VictoryOutcome::HqDestroyed {
                winner: Faction::Empire,
                loser: Faction::Alliance,
                hq_system: system,
            }
        ));
        assert!(!victory_winner_is_alliance(
            &rebellion_core::victory::VictoryOutcome::DeathStarVictory {
                target_system: system,
            }
        ));
    }

    #[test]
    fn tactical_ground_results_persist_survivor_damage_and_remove_losses() {
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
            control: ControlKind::Uncontrolled,
        });
        let survivor = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(3),
            is_alliance: true,
            regiment_strength: 100,
        });
        let destroyed = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(4),
            is_alliance: false,
            regiment_strength: 100,
        });
        world.systems[system].ground_units = vec![survivor, destroyed];

        tactical_flow::apply_tactical_ground_strengths(
            &mut world,
            system,
            &[(survivor, 37), (destroyed, 0)],
        );

        assert_eq!(world.troops[survivor].regiment_strength, 37);
        assert!(!world.troops.contains_key(destroyed));
        assert_eq!(world.systems[system].ground_units, vec![survivor]);
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "The tactical result regression keeps its opposing fleets and exact roster assertions together."
    )]
    fn tactical_space_results_preserve_exact_capital_and_fighter_roster_slots() {
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
            // The defender holds the system, so the post-battle pass keeps its
            // fleet there (FUN_00516ef0 → FUN_00515480, chain stage 1).
            control: ControlKind::Controlled(rebellion_core::dat::Faction::Empire),
        });
        let ship_class = CapitalShipKey::default();
        let fighter_class = FighterKey::default();
        let mut preexisting_dead_ship = ShipInstance::new(ship_class, 100, true);
        preexisting_dead_ship.hull_current = 0;
        preexisting_dead_ship.alive = false;
        let attacker = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![
                preexisting_dead_ship,
                ShipInstance::new(ship_class, 100, true),
                ShipInstance::new(ship_class, 100, true),
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
            has_death_star: true,
        });
        let defender = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ShipInstance::new(ship_class, 100, false)],
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            has_death_star: false,
        });
        world.systems[system].fleets = vec![attacker, defender];
        let session = rebellion_render::BattleSession {
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
                test_tactical_ship(ship_class, defender, false, 0, 91, true),
            ],
            fighters: vec![
                rebellion_render::tactical_view::TacticalFighter {
                    held_by_system: false,
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
                hull: 0.0,
                laser_charge: 0.0,
                destroyed: true,
                action_committed: false,
            }),
            death_star_beam: None,
            trench_run_outcome: None,
            trench_run_remaining_ms: None,
            trench_run_group: None,
            trench_run_success_rating: 1,
            trench_run_pending_outcome: None,
            trench_run_participants: vec![],
            trench_run_chatter: [0x13d; 9],
            trench_run_chatter_cursor: 0,
            trench_run_cinematic_pending: false,
            navigation_sets: std::array::from_fn(|_| Vec::new()),
            source_layout: rebellion_render::tactical_view::OriginalTacticalLayout::default(),
            selected_ship: None,
            selected_fighter_group: None,
            placement_confirmed: true,
            start_tick: 1,
            combat_tick: 1,
            tactical_rng: rebellion_render::OriginalTacticalRng::new(0x5eed_ba77),
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
            engagement: rebellion_render::tactical_view::TacticalEngagementState::default(),
            holder_is_alliance: None,
            blockaded_at_start: false,
            withdrawal_destination: None,
        };

        let battle_return = tactical_flow::apply_results(
            &session,
            &mut world,
            &mut TroopTransportState::default(),
            &mut MovementState::new(),
        );

        assert_eq!(world.fleets[attacker].capital_ships.len(), 1);
        assert_eq!(world.fleets[attacker].capital_ships[0].hull_current, 43);
        assert_eq!(world.fleets[attacker].fighters[0].count, 1);
        assert_eq!(world.fleets[attacker].fighters[1].count, 0);
        assert!(!world.fleets[attacker].has_death_star);
        assert_eq!(world.fleets[defender].capital_ships[0].hull_current, 91);
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
            death_star_resource: rebellion_render::death_star_tactical_resource(DatId::new(1)),
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
        let path = rebellion_data::save::slot_path(&root, 9);
        std::fs::write(&path, b"corrupt save fixture").unwrap();
        let slots = super::read_save_slots(&root);
        assert_eq!(slots.len(), 1);
        assert_eq!(slots[0].slot, 9);
        assert_eq!(slots[0].name, "Unreadable save");
        assert_eq!(std::fs::read(&path).unwrap(), b"corrupt save fixture");
        std::fs::remove_dir_all(root).unwrap();
    }
}

#[cfg(test)]
mod fleet_move_tests {
    use super::*;
    use rebellion_core::dat::{ExplorationStatus, SectorGroup};
    use rebellion_core::ids::DatId;
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
                    has_death_star: false,
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
        log: MessageLog,
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
        let movement = MovementState::new();
        let blockaded = HashSet::new();
        let mut issued = Issued {
            actions: Vec::new(),
            confirmation: MoveConfirmationState::default(),
            log: MessageLog::new(8),
        };
        issue_fleet_move(
            &FleetMoveContext {
                world,
                movement: &movement,
                blockaded: &blockaded,
                faction: MissionFaction::Alliance,
                tick: 0,
            },
            fleets,
            confirmed,
            target,
            &mut issued.log,
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
        assert!(issued.log.messages().is_empty());
    }

    #[test]
    fn an_empty_team_moving_onto_a_fleet_is_refused() {
        // FUN_0053c4b0 refuses a move with no members (1/0x16) before any
        // sub-order runs.
        let (world, fleets, _) = world_with_two_fleets();
        let origin = world.fleets[fleets[0]].location;
        let issued = issue_onto(
            &world,
            &[],
            false,
            ReleaseTarget::Fleet {
                fleet: fleets[0],
                system: origin,
            },
        );
        assert!(issued.actions.is_empty());
        assert_eq!(issued.log.messages().len(), 1);
    }

    #[test]
    fn a_bands_stop_and_destination_belong_to_its_systems_holder() {
        // Manual p. 83-84: a construction yard's band takes Destination and
        // Stop; Stop clears what the band is building. Either side holds
        // its own bands.
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
                    manufacturing,
                    &rebellion_core::research::ResearchState::new(),
                    player,
                )
            };
            let idle = gates(&world, &manufacturing, holder);
            assert!(idle.destination && idle.build && !idle.stop);
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
        let (mut world, fleets, destination) = world_with_two_fleets();
        world.fleets[fleets[1]].capital_ships.clear();
        let issued = issue(&world, &fleets, false, destination);
        assert!(issued.actions.is_empty());
        assert!(!issued.confirmation.is_open());
        assert_eq!(issued.log.messages().len(), 1);
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
