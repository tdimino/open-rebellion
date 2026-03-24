mod audio;

use macroquad::prelude::*;
use ::rand::SeedableRng;
use ::rand::Rng;
use rand_xoshiro::Xoshiro256PlusPlus;
use std::path::{Path, PathBuf};

use rebellion_core::ai::{AiFaction, AIAction, AIState, AISystem, FleetMoveReason};
use rebellion_core::betrayal::{BetrayalState, BetrayalSystem};
use rebellion_core::blockade::{BlockadeState, BlockadeSystem};
use rebellion_core::bombardment::BombardmentSystem;
use rebellion_core::combat::{CombatSide, CombatSystem};
use rebellion_core::dat::Faction;
use rebellion_core::death_star::{DeathStarState, DeathStarSystem};
use rebellion_core::events::{EventAction, EventState, EventSystem, SkillField};
use rebellion_core::fog::{FogState, FogSystem};
use rebellion_core::jedi::{JediState, JediSystem};
use rebellion_core::manufacturing::{ManufacturingState, ManufacturingSystem, QueueItem};
use rebellion_core::missions::{
    MissionEffect, MissionFaction, MissionKind, MissionState, MissionSystem,
};
use rebellion_core::movement::{MovementState, MovementSystem};
use rebellion_core::research::{ResearchState, ResearchSystem};
use rebellion_core::tick::{GameClock, GameSpeed};
use rebellion_core::uprising::{UprisingState, UprisingSystem};
use rebellion_core::victory::{VictoryState, VictorySystem};
use rebellion_core::world::{ControlKind, GameWorld, MstbTable};

use rebellion_render::{
    draw_encyclopedia, draw_fleet_overlays, draw_fleets, draw_fog_overlay,
    draw_fleet_context_menu, draw_galaxy_map, draw_game_setup, draw_main_menu,
    draw_manufacturing, draw_message_log, draw_missions, draw_officers,
    draw_status_bar, draw_system_context_menu, draw_system_info_panel,
    hovered_fleet,
    AudioVolumeState, Difficulty, EncyclopediaState,
    FleetsState, GalaxyMapState, GameMessage, GameSetupAction, GameSetupState,
    MainMenuAction, ManufacturingPanelState,
    MessageCategory, MessageLog, MessageLogState, MissionsPanelState, OfficersState, PanelAction,
    SfxKind,
};
use rebellion_render::panels::research::{draw_research, ResearchPanelState};
use rebellion_render::panels::jedi::{draw_jedi, JediPanelState};

/// Top-level game mode state machine.
///
/// Controls which screen renders each frame. Transitions are handled
/// in the main loop by matching on actions returned from each screen.
#[derive(Debug, Clone, PartialEq)]
enum GameMode {
    /// Title screen: New Game / Load Game / Quit.
    MainMenu,
    /// Campaign configuration: galaxy size, difficulty, faction.
    GameSetup,
    /// The main strategy game: galaxy map + War Room panels.
    Galaxy,
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

#[macroquad::main(window_conf)]
async fn main() {
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

    // ── Load game data ─────────────────────────────────────────────────────
    // Native: filesystem read via load_game_data()
    // WASM: HTTP fetch via macroquad::file::load_file() into cache, then load_game_data()
    #[cfg(not(target_arch = "wasm32"))]
    let mut world = match rebellion_data::load_game_data(&gdata_path) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Failed to load game data from {}: {}", gdata_path.display(), e);
            std::process::exit(1);
        }
    };

    #[cfg(target_arch = "wasm32")]
    let mut world = {
        use std::collections::HashMap;

        // Files to load via HTTP fetch
        let required: &[&str] = &[
            "SECTORSD.DAT", "SYSTEMSD.DAT", "CAPSHPSD.DAT", "FIGHTSD.DAT",
            "MJCHARSD.DAT", "MNCHARSD.DAT",
        ];
        let optional: &[&str] = &[
            "GNPRTB.DAT", "DEFFACSD.DAT",
            "CMUNEFTB.DAT", "CMUNAFTB.DAT", "CMUNEMTB.DAT", "CMUNALTB.DAT",
            "CMUNCRTB.DAT", "CMUNHQTB.DAT", "CMUNYVTB.DAT",
            "FACLCRTB.DAT", "FACLHQTB.DAT",
            "DIPLMSTB.DAT", "ESPIMSTB.DAT", "ASSNMSTB.DAT", "INCTMSTB.DAT",
            "DSSBMSTB.DAT", "ABDCMSTB.DAT", "RCRTMSTB.DAT", "RESCMSTB.DAT",
            "SBTGMSTB.DAT", "SUBDMSTB.DAT", "ESCAPETB.DAT", "FDECOYTB.DAT",
            "FOILTB.DAT",   "INFORMTB.DAT", "CSCRHTTB.DAT", "UPRIS1TB.DAT",
            "UPRIS2TB.DAT", "RLEVADTB.DAT", "RESRCTB.DAT",  "TDECOYTB.DAT",
        ];

        let mut files: HashMap<String, Vec<u8>> = HashMap::new();
        let total = required.len() + optional.len();
        let mut loaded = 0;

        // Loading screen: show progress while fetching DAT files
        for &name in required {
            let path = format!("data/base/{}", name);
            match macroquad::file::load_file(&path).await {
                Ok(data) => {
                    files.insert(name.to_string(), data);
                    loaded += 1;
                }
                Err(e) => {
                    panic!("Required file {} failed to load: {:?}", name, e);
                }
            }
            // Draw loading progress
            clear_background(Color::new(0.02, 0.02, 0.06, 1.0));
            let text = format!("Loading... ({}/{})", loaded, total);
            let font_size = 24.0;
            let dims = measure_text(&text, None, font_size as u16, 1.0);
            draw_text(&text, (screen_width() - dims.width) / 2.0,
                screen_height() / 2.0, font_size, WHITE);
            // Progress bar
            let bar_w = 300.0;
            let bar_h = 8.0;
            let bar_x = (screen_width() - bar_w) / 2.0;
            let bar_y = screen_height() / 2.0 + 20.0;
            draw_rectangle(bar_x, bar_y, bar_w, bar_h, DARKGRAY);
            draw_rectangle(bar_x, bar_y, bar_w * (loaded as f32 / total as f32), bar_h, GREEN);
            next_frame().await;
        }

        // Optional files: silently skip on 404
        for &name in optional {
            let path = format!("data/base/{}", name);
            if let Ok(data) = macroquad::file::load_file(&path).await {
                files.insert(name.to_string(), data);
            }
            loaded += 1;
            // Update loading bar
            clear_background(Color::new(0.02, 0.02, 0.06, 1.0));
            let text = format!("Loading... ({}/{})", loaded, total);
            let font_size = 24.0;
            let dims = measure_text(&text, None, font_size as u16, 1.0);
            draw_text(&text, (screen_width() - dims.width) / 2.0,
                screen_height() / 2.0, font_size, WHITE);
            let bar_w = 300.0;
            let bar_h = 8.0;
            let bar_x = (screen_width() - bar_w) / 2.0;
            let bar_y = screen_height() / 2.0 + 20.0;
            draw_rectangle(bar_x, bar_y, bar_w, bar_h, DARKGRAY);
            draw_rectangle(bar_x, bar_y, bar_w * (loaded as f32 / total as f32), bar_h, GREEN);
            next_frame().await;
        }

        // Load string table from pre-extracted JSON (TEXTSTRA.DLL can't be parsed in WASM)
        let string_table: HashMap<u16, String> = match macroquad::file::load_file("data/base/textstra.json").await {
            Ok(data) => serde_json::from_slice(&data).unwrap_or_default(),
            Err(_) => HashMap::new(),
        };
        if !string_table.is_empty() {
            eprintln!("Loaded {} entity names from textstra.json", string_table.len());
        }

        // Set the file cache and string table so load_game_data() can find them
        rebellion_data::set_string_table(string_table);
        rebellion_data::set_file_cache(files);

        match rebellion_data::load_game_data(&gdata_path) {
            Ok(w) => w,
            Err(e) => panic!("Failed to parse game data: {}", e),
        }
    };

    eprintln!(
        "Loaded: {} systems, {} sectors, {} ship classes, {} fighter classes, {} characters",
        world.systems.len(),
        world.sectors.len(),
        world.capital_ship_classes.len(),
        world.fighter_classes.len(),
        world.characters.len(),
    );

    // ── Mod Runtime ──────────────────────────────────────────────────────────
    // mods/ lives alongside data/, not inside it: data/base → data → repo root → mods/
    let mods_dir = gdata_path.parent().and_then(|p| p.parent()).unwrap_or(std::path::Path::new(".")).join("mods");
    let mut mod_runtime = rebellion_data::mods::ModRuntime::discover(&mods_dir);
    if !mod_runtime.discovered.is_empty() {
        eprintln!("Discovered {} mods ({} enabled)", mod_runtime.discovered.len(),
            mod_runtime.discovered.iter().filter(|m| m.enabled).count());
        let mod_errors = mod_runtime.apply_enabled(&mut world);
        for err in &mod_errors {
            eprintln!("Mod error: {:?}", err);
        }
    }

    // ── Game mode ─────────────────────────────────────────────────────────
    let mut game_mode = GameMode::MainMenu;
    let mut game_setup_state = GameSetupState::default();
    let mut _difficulty = Difficulty::Medium; // stored after setup, used for AI tuning

    // ── Simulation state ────────────────────────────────────────────────────
    // Seedable RNG for deterministic simulation
    let rng_seed = {
        #[cfg(not(target_arch = "wasm32"))]
        { std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(42) }
        #[cfg(target_arch = "wasm32")]
        { (macroquad::time::get_time() * 1_000_000.0) as u64 }
    };
    let mut sim_rng = Xoshiro256PlusPlus::seed_from_u64(rng_seed);
    let mut clock = GameClock::new();
    let mut mfg_state = ManufacturingState::new();
    let mut mission_state = MissionState::new();
    let mut event_state = EventState::new();
    let mut ai_state = AIState::new(AiFaction::Empire);
    let game_config = rebellion_core::tuning::GameConfig::default();
    let mut dual_ai_mode = false;
    let mut ai2_state: Option<AIState> = None;
    let mut movement_state = MovementState::new();
    let mut fog_state = FogState::new(Faction::Alliance);
    FogSystem::seed(&mut fog_state, &world);
    let mut combat_cooldowns: std::collections::HashMap<rebellion_core::ids::SystemKey, u64> = std::collections::HashMap::new();
    let mut blockade_state = BlockadeState::new();
    let mut uprising_state = UprisingState::new();
    let mut death_star_state = DeathStarState::default();
    let mut research_state = ResearchState::new();
    let mut jedi_state = JediState::new();
    let mut betrayal_state = BetrayalState::new();
    // Find HQ systems for victory detection
    let alliance_hq = world.systems.iter()
        .find(|(_, s)| s.is_headquarters && s.control.is_controlled_by(Faction::Alliance))
        .map(|(k, _)| k);
    let empire_hq = world.systems.iter()
        .find(|(_, s)| s.is_headquarters && s.control.is_controlled_by(Faction::Empire))
        .map(|(k, _)| k);
    let mut victory_state = match (alliance_hq, empire_hq) {
        (Some(a), Some(e)) => VictoryState::new(a, e),
        _ => {
            // Fallback: use first two systems if HQs not marked
            let mut keys = world.systems.keys();
            let a = keys.next().expect("world must have at least 2 systems for victory");
            let e = keys.next().expect("world must have at least 2 systems for victory");
            VictoryState::new(a, e)
        }
    };

    // Register scripted story events
    rebellion_core::story_events::define_story_events(&mut event_state, &world);

    // ── UI state ────────────────────────────────────────────────────────────
    let mut map_state = GalaxyMapState::default();
    let mut msg_log = MessageLog::default();
    let mut log_state = MessageLogState::default();

    // ── War Room panel state ────────────────────────────────────────────────
    let mut player_faction = MissionFaction::Alliance;
    let mut officers_state = OfficersState::default();
    let mut fleets_state = FleetsState::default();
    let mut mfg_panel_state = ManufacturingPanelState::default();
    let mut missions_panel_state = MissionsPanelState::default();
    let mut enc_state = EncyclopediaState::new();
    let mut research_panel_state = ResearchPanelState::default();
    let mut jedi_panel_state = JediPanelState::default();
    let mut mod_manager_state = rebellion_render::ModManagerState::default();
    #[cfg(debug_assertions)]
    let mut command_palette_state = rebellion_render::CommandPaletteState::new();
    enc_state.set_edata_path(gdata_path.join("EData"));
    // HD upscaled PNGs live as a sibling of the base data directory.
    let hd_path = gdata_path
        .parent()
        .unwrap_or(Path::new("."))
        .join("hd")
        .join("EData");
    enc_state.set_hd_path(hd_path);

    // Panel visibility (mutually exclusive left panels)
    let mut show_officers = false;
    let mut show_fleets = false;
    let mut show_manufacturing = false;
    let mut show_missions = false;
    let mut show_research = false;
    let mut show_jedi = false;

    // ── Audio state ─────────────────────────────────────────────────────────
    let mut audio_vol = AudioVolumeState::default();
    let sounds_dir = PathBuf::from("data/sounds");

    #[cfg(not(target_arch = "wasm32"))]
    let mut audio_engine = {
        let mut engine = audio::AudioEngine::new();
        if sounds_dir.exists() {
            engine.load_all(&sounds_dir);
        }
        audio_vol.backend_available = engine.is_available();
        engine
    };

    // ── Apply Star Wars theme ────────────────────────────────────────────
    // Must happen inside the macroquad async context, after first frame init.
    let mut theme_applied = false;

    loop {
        let dt = get_frame_time();

        // Apply theme on first frame (egui context exists after first next_frame)
        if !theme_applied {
            egui_macroquad::ui(|ctx| {
                rebellion_render::theme::load_fonts(ctx);
                rebellion_render::theme::apply_theme(ctx);
            });
            egui_macroquad::draw();
            theme_applied = true;
        }

        // ── Global keyboard shortcuts ───────────────────────────────────────
        if is_key_pressed(KeyCode::Escape) {
            // In Galaxy mode, Escape could open a menu later.
            // For now, Escape quits from any mode.
            break;
        }
        // ── Galaxy-mode keyboard shortcuts ──────────────────────────────────
        if game_mode == GameMode::Galaxy {
            if is_key_pressed(KeyCode::R) {
                map_state = GalaxyMapState::default();
            }
            // Speed controls
            if is_key_pressed(KeyCode::Space) {
                if clock.speed == GameSpeed::Paused {
                    clock.set_speed(GameSpeed::Normal);
                } else {
                    clock.set_speed(GameSpeed::Paused);
                }
            }
            if is_key_pressed(KeyCode::Key1) {
                clock.set_speed(GameSpeed::Normal);
            }
            if is_key_pressed(KeyCode::Key2) {
                clock.set_speed(GameSpeed::Fast);
            }
            if is_key_pressed(KeyCode::Key3) {
                clock.set_speed(GameSpeed::Faster);
            }
            // Panel toggles (mutually exclusive left panels)
            if is_key_pressed(KeyCode::O) {
                show_officers = !show_officers;
                if show_officers {
                    show_fleets = false;
                    show_manufacturing = false;
                    show_missions = false;
                }
            }
            if is_key_pressed(KeyCode::F) {
                show_fleets = !show_fleets;
                if show_fleets {
                    show_officers = false;
                    show_manufacturing = false;
                    show_missions = false;
                }
            }
            if is_key_pressed(KeyCode::M) {
                show_manufacturing = !show_manufacturing;
                if show_manufacturing {
                    show_officers = false;
                    show_fleets = false;
                    show_missions = false;
                }
            }
            if is_key_pressed(KeyCode::N) {
                show_missions = !show_missions;
                if show_missions {
                    show_officers = false;
                    show_fleets = false;
                    show_manufacturing = false;
                }
            }
            if is_key_pressed(KeyCode::T) {
                show_research = !show_research;
                if show_research {
                    show_officers = false;
                    show_fleets = false;
                    show_manufacturing = false;
                    show_missions = false;
                    show_jedi = false;
                }
            }
            if is_key_pressed(KeyCode::J) {
                show_jedi = !show_jedi;
                if show_jedi {
                    show_officers = false;
                    show_fleets = false;
                    show_manufacturing = false;
                    show_missions = false;
                    show_research = false;
                }
            }
            if is_key_pressed(KeyCode::E) {
                enc_state.open = !enc_state.open;
            }
            if is_key_pressed(KeyCode::Tab) {
                mod_manager_state.open = !mod_manager_state.open;
            }
            #[cfg(debug_assertions)]
            if is_key_pressed(KeyCode::GraveAccent) {
                command_palette_state.open = !command_palette_state.open;
            }
        }

        // ── Tick the clock (Galaxy mode only) ────────────────────────────────
        let tick_events = if game_mode == GameMode::Galaxy {
            clock.advance(dt)
        } else {
            vec![]
        };

        if !tick_events.is_empty() {
            // ── Manufacturing (blockaded systems are skipped) ─────────────────
            let completions = ManufacturingSystem::advance_with_blockade(
                &mut mfg_state, &tick_events, blockade_state.blockaded_systems(),
            );
            for completion in &completions {
                let sys_name = world
                    .systems
                    .get(completion.system)
                    .map(|s| s.name.clone())
                    .unwrap_or_else(|| "unknown".into());
                msg_log.push(GameMessage::at_system(
                    completion.tick,
                    format!("Construction complete at {}", sys_name),
                    MessageCategory::Manufacturing,
                    completion.system,
                ));
                #[cfg(not(target_arch = "wasm32"))]
                audio_engine.play_sfx(SfxKind::BuildComplete, &audio_vol);
            }

            // ── Movement ────────────────────────────────────────────────────
            let arrivals = MovementSystem::advance(&mut movement_state, &tick_events);
            for arrival in &arrivals {
                if let Some(fleet) = world.fleets.get_mut(arrival.fleet) {
                    fleet.location = arrival.system;
                }
                // Update System.fleets: remove from origin, add to destination
                if let Some(origin_sys) = world.systems.get_mut(arrival.origin) {
                    origin_sys.fleets.retain(|&k| k != arrival.fleet);
                }
                if let Some(dest_sys) = world.systems.get_mut(arrival.system) {
                    if !dest_sys.fleets.contains(&arrival.fleet) {
                        dest_sys.fleets.push(arrival.fleet);
                    }
                }
                let sys_name = world
                    .systems
                    .get(arrival.system)
                    .map(|s| s.name.clone())
                    .unwrap_or_else(|| "unknown".into());
                msg_log.push(GameMessage::at_system(
                    arrival.tick,
                    format!("Fleet arrived at {}", sys_name),
                    MessageCategory::Mission,
                    arrival.system,
                ));
                #[cfg(not(target_arch = "wasm32"))]
                audio_engine.play_sfx(SfxKind::FleetArrival, &audio_vol);
            }

            // ── Combat ──────────────────────────────────────────────────────
            // After fleet arrivals, check every system for opposing fleets.
            // Collect combat triggers first (immutable world borrow).
            let current_tick = tick_events.last().map(|e| e.tick).unwrap_or(0);
            let combat_triggers: Vec<_> = world.systems.keys()
                .filter_map(|sys_key| {
                    // Combat cooldown: skip systems that had combat within last 5 ticks.
                    if let Some(&last_battle) = combat_cooldowns.get(&sys_key) {
                        if current_tick < last_battle + 5 { return None; }
                    }
                    let sys = &world.systems[sys_key];
                    let alliance_fleets: Vec<_> = sys.fleets.iter()
                        .copied()
                        .filter(|&k| world.fleets.get(k).map(|f| f.is_alliance).unwrap_or(false))
                        .collect();
                    let empire_fleets: Vec<_> = sys.fleets.iter()
                        .copied()
                        .filter(|&k| world.fleets.get(k).map(|f| !f.is_alliance).unwrap_or(false))
                        .collect();
                    if !alliance_fleets.is_empty() && !empire_fleets.is_empty() {
                        Some((sys_key, alliance_fleets[0], empire_fleets[0]))
                    } else {
                        None
                    }
                })
                .collect();

            for (sys_key, atk_fleet, def_fleet) in combat_triggers {
                let sys_name = world.systems.get(sys_key)
                    .map(|s| s.name.clone())
                    .unwrap_or_else(|| "Unknown".into());

                // Generate RNG rolls: 256 rolls covers fleets up to 16 ships each.
                let combat_rolls: Vec<f64> =
                    (0..256).map(|_| sim_rng.gen::<f64>()).collect();
                let space_result = CombatSystem::resolve_space(
                    &world, atk_fleet, def_fleet, sys_key,
                    2, // medium difficulty
                    &combat_rolls,
                    current_tick,
                );

                // Apply ship damage: reduce counts proportional to destroyed hulls.
                apply_space_combat_result(&space_result, &mut world);
                // Record combat cooldown to prevent infinite re-trigger on draws.
                combat_cooldowns.insert(sys_key, current_tick);

                let winner_str = match space_result.winner {
                    CombatSide::Attacker => "Alliance victory",
                    CombatSide::Defender => "Empire victory",
                    CombatSide::Draw     => "Draw",
                };
                msg_log.push(GameMessage::at_system(
                    current_tick,
                    format!("Space battle at {} — {}", sys_name, winner_str),
                    MessageCategory::Combat,
                    sys_key,
                ));
                #[cfg(not(target_arch = "wasm32"))]
                audio_engine.play_sfx(SfxKind::FleetArrival, &audio_vol);

                // Ground combat: if attacker wins space battle and system has enemy troops.
                if space_result.winner == CombatSide::Attacker {
                    let ground_rolls: Vec<f64> =
                        (0..256).map(|_| sim_rng.gen::<f64>()).collect();
                    let ground_result = CombatSystem::resolve_ground(
                        &world, sys_key, true, // alliance is attacker
                        &ground_rolls, current_tick,
                    );
                    apply_ground_combat_result(&ground_result, &mut world);

                    if !ground_result.troop_damage.is_empty() {
                        let ground_str = match ground_result.winner {
                            CombatSide::Attacker => "Alliance ground victory",
                            CombatSide::Defender => "Empire ground holds",
                            CombatSide::Draw     => "Ground combat draw",
                        };
                        msg_log.push(GameMessage::at_system(
                            current_tick,
                            format!("Ground battle at {} — {}", sys_name, ground_str),
                            MessageCategory::Combat,
                            sys_key,
                        ));
                    }

                    // Orbital bombardment: empire fleet bombards remaining defenders.
                    let brd_result = BombardmentSystem::resolve_bombardment(
                        &world, atk_fleet, sys_key, 2, current_tick,
                    );
                    if brd_result.damage > 0 {
                        msg_log.push(GameMessage::at_system(
                            current_tick,
                            format!(
                                "Orbital bombardment at {} — {} damage",
                                sys_name, brd_result.damage
                            ),
                            MessageCategory::Combat,
                            sys_key,
                        ));
                    }
                }
            }

            // ── Fog of war ──────────────────────────────────────────────────
            let reveals = FogSystem::advance(&mut fog_state, &world, &movement_state);
            for reveal in &reveals {
                let sys_name = world
                    .systems
                    .get(reveal.system)
                    .map(|s| s.name.clone())
                    .unwrap_or_else(|| "unknown".into());
                msg_log.push(GameMessage::at_system(
                    tick_events.last().unwrap().tick,
                    format!("System {} revealed", sys_name),
                    MessageCategory::Event,
                    reveal.system,
                ));
            }

            // ── Missions ────────────────────────────────────────────────────
            let mission_rolls: Vec<f64> = (0..mission_state.len())
                .map(|_| sim_rng.gen::<f64>())
                .collect();
            let mission_results =
                MissionSystem::advance(&mut mission_state, &world, &tick_events, &mission_rolls);

            for result in &mission_results {
                apply_mission_result(
                    result,
                    &mut world,
                    &mut msg_log,
                    #[cfg(not(target_arch = "wasm32"))]
                    &mut audio_engine,
                    #[cfg(not(target_arch = "wasm32"))]
                    &audio_vol,
                );
                ai_state.mark_available(result.character);
            }

            // ── Character escapes ────────────────────────────────────────────
            let escape_rolls: Vec<f64> = (0..world.characters.len())
                .map(|_| sim_rng.gen::<f64>()).collect();
            let escape_effects = MissionSystem::check_escapes(&world, &escape_rolls);
            for effect in &escape_effects {
                if let MissionEffect::CharacterEscaped { character, escaped_to_alliance } = effect {
                    if let Some(c) = world.characters.get_mut(*character) {
                        c.is_alliance = *escaped_to_alliance;
                        c.is_empire = !*escaped_to_alliance;
                        c.is_captive = false;
                        c.captured_by = None;
                        c.capture_tick = None;
                    }
                    for (_, fleet) in world.fleets.iter_mut() {
                        fleet.characters.retain(|&k| k != *character);
                    }
                    let name = world.characters.get(*character).map(|c| c.name.clone()).unwrap_or_else(|| "Unknown".into());
                    msg_log.push(GameMessage::new(current_tick, format!("{} has escaped captivity!", name), MessageCategory::Event));
                }
            }

            // ── Events ──────────────────────────────────────────────────────
            let event_rolls: Vec<f32> =
                (0..16).map(|_| sim_rng.gen::<f32>()).collect();
            let fired_events =
                EventSystem::advance(&mut event_state, &world, &tick_events, &event_rolls);

            for fired in &fired_events {
                apply_event_actions(&fired.actions, &mut world, &mut msg_log, fired.tick);
            }

            // Apply Jedi training from story events (outside apply_event_actions
            // because it needs jedi_state which is not passed to that function)
            for fired in &fired_events {
                for action in &fired.actions {
                    if let EventAction::StartJediTraining { character } = action {
                        if let Some(c) = world.characters.get(*character) {
                            jedi_state.start_training(*character, c.is_alliance, current_tick);
                        }
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
            let ai_rolls: Vec<f64> =
                (0..8).map(|_| sim_rng.gen::<f64>()).collect();
            apply_ai_actions(
                &ai_actions,
                &ai_rolls,
                &mut ai_state,
                &mut mission_state,
                &mut mfg_state,
                &mut movement_state,
                &mut research_state,
                &world,
                &mut msg_log,
                tick_events.last().map(|e| e.tick).unwrap_or(0),
                #[cfg(not(target_arch = "wasm32"))]
                &mut audio_engine,
                #[cfg(not(target_arch = "wasm32"))]
                &audio_vol,
            );

            // ── Dual AI (second faction) ────────────────────────────────────
            if let Some(ref mut second_ai) = ai2_state {
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
                let second_rolls: Vec<f64> =
                    (0..8).map(|_| sim_rng.gen::<f64>()).collect();
                apply_ai_actions(
                    &second_actions,
                    &second_rolls,
                    second_ai,
                    &mut mission_state,
                    &mut mfg_state,
                    &mut movement_state,
                    &mut research_state,
                    &world,
                    &mut msg_log,
                    tick_events.last().map(|e| e.tick).unwrap_or(0),
                    #[cfg(not(target_arch = "wasm32"))]
                    &mut audio_engine,
                    #[cfg(not(target_arch = "wasm32"))]
                    &audio_vol,
                );
            }

            // ── Blockade ─────────────────────────────────────────────────────
            let blockade_events = BlockadeSystem::advance(&mut blockade_state, &world, &tick_events);
            for evt in &blockade_events {
                match evt {
                    rebellion_core::blockade::BlockadeEvent::BlockadeStarted { system, tick } => {
                        let name = world.systems.get(*system).map(|s| s.name.clone()).unwrap_or_else(|| "unknown".into());
                        msg_log.push(GameMessage::at_system(*tick, format!("Blockade established at {}", name), MessageCategory::Combat, *system));
                    }
                    rebellion_core::blockade::BlockadeEvent::BlockadeEnded { system, tick } => {
                        let name = world.systems.get(*system).map(|s| s.name.clone()).unwrap_or_else(|| "unknown".into());
                        msg_log.push(GameMessage::at_system(*tick, format!("Blockade lifted at {}", name), MessageCategory::Combat, *system));
                    }
                    rebellion_core::blockade::BlockadeEvent::TroopDestroyed { system, troop, tick } => {
                        if let Some(sys) = world.systems.get_mut(*system) {
                            sys.ground_units.retain(|&k| k != *troop);
                        }
                        world.troops.remove(*troop);
                        let name = world.systems.get(*system).map(|s| s.name.clone()).unwrap_or_else(|| "unknown".into());
                        msg_log.push(GameMessage::at_system(*tick, format!("Troops destroyed by blockade at {}", name), MessageCategory::Combat, *system));
                    }
                }
            }

            // ── Uprising ─────────────────────────────────────────────────────
            let uprising_rolls: Vec<f64> = (0..world.systems.len())
                .map(|_| sim_rng.gen::<f64>()).collect();
            let empty_upris1tb = MstbTable::new(vec![]);
            let upris1tb = world.mission_tables.get("UPRIS1TB").unwrap_or(&empty_upris1tb);
            let uprising_events = UprisingSystem::advance(&mut uprising_state, &world, &tick_events, &uprising_rolls, upris1tb);
            for evt in &uprising_events {
                match evt {
                    rebellion_core::uprising::UprisingEvent::UprisingIncident { system, tick } => {
                        let name = world.systems.get(*system).map(|s| s.name.clone()).unwrap_or_else(|| "unknown".into());
                        msg_log.push(GameMessage::at_system(*tick, format!("Uprising incident at {}", name), MessageCategory::Diplomacy, *system));
                    }
                    rebellion_core::uprising::UprisingEvent::UprisingBegan { system, tick } => {
                        // Flip controlling faction
                        if let Some(sys) = world.systems.get_mut(*system) {
                            sys.control = match sys.control {
                                ControlKind::Controlled(Faction::Alliance) => ControlKind::Controlled(Faction::Empire),
                                ControlKind::Controlled(Faction::Empire) => ControlKind::Controlled(Faction::Alliance),
                                other => other,
                            };
                        }
                        let name = world.systems.get(*system).map(|s| s.name.clone()).unwrap_or_else(|| "unknown".into());
                        msg_log.push(GameMessage::at_system(*tick, format!("Uprising! {} has changed hands", name), MessageCategory::Diplomacy, *system));
                    }
                    rebellion_core::uprising::UprisingEvent::UprisingSubdued { system, tick } => {
                        let name = world.systems.get(*system).map(|s| s.name.clone()).unwrap_or_else(|| "unknown".into());
                        msg_log.push(GameMessage::at_system(*tick, format!("Uprising subdued at {}", name), MessageCategory::Diplomacy, *system));
                    }
                }
            }

            // ── Betrayal ─────────────────────────────────────────────────────
            let betrayal_rolls: Vec<f64> = (0..world.characters.len())
                .map(|_| sim_rng.gen::<f64>()).collect();
            let empty_loyalty_tb = MstbTable::new(vec![]);
            let loyalty_tb = world.mission_tables.get("UPRIS1TB").unwrap_or(&empty_loyalty_tb);
            let betrayal_events = BetrayalSystem::advance(&mut betrayal_state, &world, &tick_events, &betrayal_rolls, loyalty_tb);
            for evt in &betrayal_events {
                let rebellion_core::betrayal::BetrayalEvent::CharacterBetrayed { character, defected_to_alliance } = evt;
                if let Some(c) = world.characters.get_mut(*character) {
                    c.is_alliance = *defected_to_alliance;
                    c.is_empire = !*defected_to_alliance;
                }
                // Remove from current fleet
                for (_, fleet) in world.fleets.iter_mut() {
                    fleet.characters.retain(|&k| k != *character);
                }
                let name = world.characters.get(*character).map(|c| c.name.clone()).unwrap_or_else(|| "Unknown".into());
                let to_faction = if *defected_to_alliance { "Alliance" } else { "Empire" };
                msg_log.push(GameMessage::new(current_tick, format!("{} has betrayed and defected to the {}!", name, to_faction), MessageCategory::Event));
            }

            // ── Death Star ───────────────────────────────────────────────────
            let ds_events = DeathStarSystem::advance(&mut death_star_state, &world, &tick_events);
            for evt in &ds_events {
                match evt {
                    rebellion_core::death_star::DeathStarEvent::ConstructionCompleted { system, tick } => {
                        let name = world.systems.get(*system).map(|s| s.name.clone()).unwrap_or_else(|| "unknown".into());
                        msg_log.push(GameMessage::at_system(*tick, format!("Death Star construction complete at {}", name), MessageCategory::Event, *system));
                    }
                    rebellion_core::death_star::DeathStarEvent::PlanetDestroyed { system, tick } => {
                        if let Some(sys) = world.systems.get_mut(*system) {
                            sys.is_destroyed = true;
                        }
                        let name = world.systems.get(*system).map(|s| s.name.clone()).unwrap_or_else(|| "unknown".into());
                        msg_log.push(GameMessage::at_system(*tick, format!("{} destroyed by Death Star!", name), MessageCategory::Event, *system));
                    }
                    rebellion_core::death_star::DeathStarEvent::NearbyWarning { system, tick } => {
                        let name = world.systems.get(*system).map(|s| s.name.clone()).unwrap_or_else(|| "unknown".into());
                        msg_log.push(GameMessage::at_system(*tick, format!("Death Star detected near {}!", name), MessageCategory::Event, *system));
                    }
                }
            }

            // ── Research ─────────────────────────────────────────────────────
            let research_results = ResearchSystem::advance(&mut research_state, &world, &tick_events);
            for result in &research_results {
                let rebellion_core::research::ResearchResult::TechUnlocked { faction_is_alliance, tech_type, new_level } = result;
                let faction_name = if *faction_is_alliance { "Alliance" } else { "Empire" };
                let tech_name = match tech_type {
                    rebellion_core::research::TechType::Ship => "Ship",
                    rebellion_core::research::TechType::Troop => "Troop",
                    rebellion_core::research::TechType::Facility => "Facility",
                };
                msg_log.push(GameMessage::new(
                    current_tick,
                    format!("{} {} tech advanced to level {}", faction_name, tech_name, new_level),
                    MessageCategory::Event,
                ));
            }
            // Apply research level-ups (advance() is now pure — caller must apply)
            for result in &research_results {
                let rebellion_core::research::ResearchResult::TechUnlocked { faction_is_alliance, tech_type, .. } = result;
                if *faction_is_alliance {
                    research_state.alliance.advance(*tech_type);
                } else {
                    research_state.empire.advance(*tech_type);
                }
            }

            // ── Jedi training ────────────────────────────────────────────────
            let jedi_rolls: Vec<f64> = (0..jedi_state.training.len().max(1))
                .map(|_| sim_rng.gen::<f64>()).collect();
            let jedi_events = JediSystem::advance(&mut jedi_state, &world, &tick_events, &jedi_rolls);
            for evt in &jedi_events {
                match evt {
                    rebellion_core::jedi::JediEvent::TierAdvanced { character, new_tier } => {
                        if let Some(c) = world.characters.get_mut(*character) {
                            c.force_tier = *new_tier;
                            // Persist XP: set to threshold for the new tier
                            c.force_experience = match new_tier {
                                rebellion_core::world::ForceTier::None => 0,
                                rebellion_core::world::ForceTier::Aware => 1,
                                rebellion_core::world::ForceTier::Training => rebellion_core::jedi::XP_TO_TRAINING,
                                rebellion_core::world::ForceTier::Experienced => rebellion_core::jedi::XP_TO_EXPERIENCED,
                            };
                        }
                        let name = world.characters.get(*character).map(|c| c.name.clone()).unwrap_or_else(|| "Unknown".into());
                        let tier_str = match new_tier {
                            rebellion_core::world::ForceTier::None => "None",
                            rebellion_core::world::ForceTier::Aware => "Force Aware",
                            rebellion_core::world::ForceTier::Training => "Jedi Training",
                            rebellion_core::world::ForceTier::Experienced => "Jedi Knight",
                        };
                        msg_log.push(GameMessage::new(current_tick, format!("{} has reached {} tier", name, tier_str), MessageCategory::Event));
                    }
                    rebellion_core::jedi::JediEvent::TrainingComplete { character } => {
                        jedi_state.stop_training(*character);
                    }
                    rebellion_core::jedi::JediEvent::JediDiscovered { character, .. } => {
                        if let Some(c) = world.characters.get_mut(*character) {
                            c.is_discovered_jedi = true;
                        }
                        let name = world.characters.get(*character).map(|c| c.name.clone()).unwrap_or_else(|| "Unknown".into());
                        msg_log.push(GameMessage::new(current_tick, format!("{}'s Force sensitivity discovered!", name), MessageCategory::Event));
                    }
                }
            }

            // ── Victory check ────────────────────────────────────────────────
            if let Some(outcome) = VictorySystem::check(&victory_state, &world, &tick_events) {
                victory_state.resolved = true;
                let msg = match &outcome {
                    rebellion_core::victory::VictoryOutcome::HqCaptured { winner, loser, .. } => {
                        format!("{:?} captured {:?} headquarters! {:?} wins!", winner, loser, winner)
                    }
                    rebellion_core::victory::VictoryOutcome::DeathStarVictory { .. } => {
                        "The Death Star destroyed the Rebel base! Empire wins!".to_string()
                    }
                    rebellion_core::victory::VictoryOutcome::DeathStarDestroyed { .. } => {
                        "The Death Star has been destroyed! Alliance wins!".to_string()
                    }
                };
                msg_log.push(GameMessage::new(current_tick, msg, MessageCategory::Event));
            }
        }

        // ── Rendering (mode-specific) ────────────────────────────────────────

        let mut panel_actions: Vec<PanelAction> = Vec::new();

        match game_mode {
            GameMode::MainMenu => {
                clear_background(Color::new(0.02, 0.02, 0.06, 1.0));
                let mut menu_action = None;
                egui_macroquad::ui(|ctx| {
                    menu_action = draw_main_menu(ctx);
                });
                egui_macroquad::draw();

                if let Some(action) = menu_action {
                    match action {
                        MainMenuAction::NewGame => {
                            game_setup_state = GameSetupState::default();
                            game_mode = GameMode::GameSetup;
                        }
                        MainMenuAction::LoadGame => {
                            // TODO: transition to save/load screen
                            // For now, go straight to galaxy with defaults
                            game_mode = GameMode::Galaxy;
                        }
                        MainMenuAction::Quit => {
                            break;
                        }
                    }
                }
            }

            GameMode::GameSetup => {
                clear_background(Color::new(0.02, 0.02, 0.06, 1.0));
                let mut setup_action = None;
                egui_macroquad::ui(|ctx| {
                    setup_action = draw_game_setup(ctx, &mut game_setup_state);
                });
                egui_macroquad::draw();

                if let Some(action) = setup_action {
                    match action {
                        GameSetupAction::StartGame { difficulty, faction, .. } => {
                            _difficulty = difficulty;
                            player_faction = faction;

                            // Initialize game state for chosen faction
                            let dat_faction = match faction {
                                MissionFaction::Alliance => Faction::Alliance,
                                MissionFaction::Empire => Faction::Empire,
                            };
                            fog_state = FogState::new(dat_faction);
                            FogSystem::seed(&mut fog_state, &world);

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
                            msg_log.push(GameMessage::new(
                                clock.tick,
                                format!("You command the {}.", faction_name),
                                MessageCategory::Event,
                            ));

                            // Start main theme music
                            #[cfg(not(target_arch = "wasm32"))]
                            audio_engine.play_music(
                                rebellion_render::MusicTrack::MainTheme,
                                &sounds_dir,
                                &audio_vol,
                            );

                            game_mode = GameMode::Galaxy;
                        }
                        GameSetupAction::Back => {
                            game_mode = GameMode::MainMenu;
                        }
                    }
                }
            }

            GameMode::Galaxy => {
                // 1. Galaxy map (pure macroquad) — returns camera params
                let cam = draw_galaxy_map(&world, &mut map_state);

                // 2. Fog overlay (pure macroquad) — dim non-visible systems
                draw_fog_overlay(
                    &world,
                    &fog_state,
                    cam.cam_x,
                    cam.cam_y,
                    cam.zoom,
                    cam.map_width,
                    cam.screen_height,
                );

                // 3. Fleet overlays (pure macroquad) — on top of fog
                draw_fleet_overlays(
                    &world,
                    &movement_state,
                    cam.cam_x,
                    cam.cam_y,
                    cam.zoom,
                    cam.map_width,
                    cam.screen_height,
                );

                // 3b. Fleet hover detection — check if right-click landed on a fleet.
                // Fleet takes priority over system — overrides the system menu set
                // by draw_galaxy_map if both are under cursor.
                {
                    let (mx, my) = mouse_position();
                    if is_mouse_button_released(macroquad::input::MouseButton::Right) && mx < cam.map_width {
                        let was_drag = map_state.right_click_start.map_or(true, |(sx, sy)| {
                            ((mx - sx).powi(2) + (my - sy).powi(2)).sqrt() > 5.0
                        });
                        if !was_drag {
                            if let Some(fleet_key) = hovered_fleet(
                                &world,
                                &movement_state,
                                cam.cam_x,
                                cam.cam_y,
                                cam.zoom,
                                cam.map_width,
                                cam.screen_height,
                                mx,
                                my,
                            ) {
                                map_state.context_menu_fleet = Some((fleet_key, mx, my));
                                map_state.context_menu_system = None;
                            }
                        }
                        // Clear right_click_start after both draw_galaxy_map and this
                        // check have had a chance to read it.
                        map_state.right_click_start = None;
                    }
                }

                // 4. All egui panels in a single ui() + draw() pass
                egui_macroquad::ui(|ctx| {
                    // War Room panels (mutually exclusive left panels)
                    if show_officers {
                        if let Some(action) =
                            draw_officers(ctx, &world, &mut officers_state, player_faction)
                        {
                            panel_actions.push(action);
                        }
                    }
                    if show_fleets {
                        if let Some(action) =
                            draw_fleets(ctx, &world, &movement_state, &mut fleets_state, player_faction)
                        {
                            panel_actions.push(action);
                        }
                    }
                    if show_manufacturing {
                        if let Some(action) = draw_manufacturing(
                            ctx,
                            &world,
                            &mfg_state,
                            &mut mfg_panel_state,
                            player_faction,
                        ) {
                            panel_actions.push(action);
                        }
                    }
                    if show_missions {
                        let duration_roll = sim_rng.gen::<f64>();
                        if let Some(action) = draw_missions(
                            ctx,
                            &world,
                            &mission_state,
                            &mut missions_panel_state,
                            player_faction,
                            duration_roll,
                        ) {
                            panel_actions.push(action);
                        }
                    }
                    if show_research {
                        if let Some(action) = draw_research(
                            ctx,
                            &world,
                            &research_state,
                            &mut research_panel_state,
                            player_faction,
                        ) {
                            panel_actions.push(action);
                        }
                    }
                    if show_jedi {
                        if let Some(action) = draw_jedi(
                            ctx,
                            &world,
                            &jedi_state,
                            &mut jedi_panel_state,
                            player_faction,
                        ) {
                            panel_actions.push(action);
                        }
                    }

                    // Encyclopedia (floating window)
                    if let Some(sys_key) = draw_encyclopedia(ctx, &world, &mut enc_state) {
                        panel_actions.push(PanelAction::FocusFleetSystem(sys_key));
                    }

                    // Mod Manager (floating window)
                    let mod_infos: Vec<rebellion_render::ModInfo> = mod_runtime.discovered.iter().map(|m| {
                        let err = mod_runtime.errors.iter().find(|e| format!("{:?}", e).contains(&m.name));
                        rebellion_render::ModInfo {
                            name: m.name.clone(),
                            version: m.version.clone(),
                            author: m.author.clone(),
                            description: m.description.clone(),
                            enabled: m.enabled,
                            dependencies: m.dependencies.keys().cloned().collect(),
                            has_error: err.is_some(),
                            error_message: err.map(|e| format!("{:?}", e)),
                        }
                    }).collect();
                    let mod_actions = rebellion_render::draw_mod_manager(ctx, &mod_infos, &mut mod_manager_state);
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

                    // Command palette (debug only)
                    #[cfg(debug_assertions)]
                    {
                        let palette_actions = rebellion_render::draw_command_palette(ctx, &mut command_palette_state);
                        for action in palette_actions {
                            panel_actions.push(action);
                        }
                    }

                    // System info panel (right side)
                    draw_system_info_panel(ctx, &world, &map_state);

                    // Context menus (floating, triggered by right-click)
                    if let Some(action) = draw_system_context_menu(
                        ctx, &world, &mut map_state, player_faction,
                    ) {
                        panel_actions.push(action);
                    }
                    if let Some(action) = draw_fleet_context_menu(
                        ctx, &world, &movement_state, &mut map_state,
                    ) {
                        panel_actions.push(action);
                    }

                    // Message log (bottom panel, above status bar)
                    draw_message_log(ctx, &msg_log, &mut log_state);

                    // Status bar (bottom-most, with speed controls + audio)
                    draw_status_bar(ctx, &world, &mut clock, &mut audio_vol);
                });
                egui_macroquad::draw();
            }
        }

        // 5. Apply panel actions
        for action in panel_actions {
            apply_panel_action(
                action,
                &mut world,
                &mut map_state,
                &mut mfg_state,
                &mut mission_state,
                &mut movement_state,
                &mut fog_state,
                &mut ai_state,
                &mut research_state,
                &mut jedi_state,
                &mut msg_log,
                &mut player_faction,
                &mut clock,
                &mut dual_ai_mode,
                &mut ai2_state,
                &victory_state,
                &event_state,
                &mut mod_runtime,
                #[cfg(not(target_arch = "wasm32"))]
                &mut audio_engine,
                #[cfg(not(target_arch = "wasm32"))]
                &audio_vol,
                #[cfg(not(target_arch = "wasm32"))]
                &sounds_dir,
            );
        }

        // 6. Apply audio volume changes
        #[cfg(not(target_arch = "wasm32"))]
        if audio_vol.dirty {
            audio_engine.apply_volume(&audio_vol);
            audio_vol.dirty = false;
        }

        // 7. Handle focus requests from message log + encyclopedia
        if let Some(focus_key) = log_state.focus_system.take() {
            if let Some(system) = world.systems.get(focus_key) {
                map_state.camera_x = system.x as f32;
                map_state.camera_y = system.y as f32;
                map_state.selected_system = Some(focus_key);
            }
        }

        next_frame().await;
    }
}

// ---------------------------------------------------------------------------
// Panel action handler
// ---------------------------------------------------------------------------

fn apply_panel_action(
    action: PanelAction,
    world: &mut GameWorld,
    map_state: &mut GalaxyMapState,
    mfg_state: &mut ManufacturingState,
    mission_state: &mut MissionState,
    movement_state: &mut MovementState,
    fog_state: &mut FogState,
    ai_state: &mut AIState,
    research_state: &mut ResearchState,
    jedi_state: &mut JediState,
    msg_log: &mut MessageLog,
    player_faction: &mut MissionFaction,
    clock: &mut GameClock,
    dual_ai_mode: &mut bool,
    ai2_state: &mut Option<AIState>,
    victory_state: &VictoryState,
    event_state: &EventState,
    mod_runtime: &mut rebellion_data::mods::ModRuntime,
    #[cfg(not(target_arch = "wasm32"))] audio_engine: &mut audio::AudioEngine,
    #[cfg(not(target_arch = "wasm32"))] audio_vol: &AudioVolumeState,
    #[cfg(not(target_arch = "wasm32"))] sounds_dir: &Path,
) {
    match action {
        PanelAction::SelectFaction(_) => {
            // Faction selection now handled by GameSetup screen transition.
            // This variant is kept for backwards compatibility but is a no-op.
        }
        PanelAction::FocusCharacter(_) => {
            // Officers panel handles its own focus state internally
        }
        PanelAction::FocusFleetSystem(sys_key) => {
            if let Some(system) = world.systems.get(sys_key) {
                map_state.camera_x = system.x as f32;
                map_state.camera_y = system.y as f32;
                map_state.selected_system = Some(sys_key);
            }
        }
        PanelAction::AssignCharacterToFleet { character, fleet } => {
            if let Some(f) = world.fleets.get_mut(fleet) {
                if !f.characters.contains(&character) {
                    f.characters.push(character);
                }
            }
            if let Some(c) = world.characters.get_mut(character) {
                let name = c.name.clone();
                c.current_fleet = Some(fleet);
                msg_log.push(GameMessage::new(
                    clock.tick,
                    format!("{} assigned to fleet", name),
                    MessageCategory::Event,
                ));
            }
        }
        PanelAction::RemoveCharacterFromFleet { character, fleet } => {
            if let Some(f) = world.fleets.get_mut(fleet) {
                f.characters.retain(|&c| c != character);
            }
            if let Some(c) = world.characters.get_mut(character) {
                let name = c.name.clone();
                c.current_fleet = None;
                msg_log.push(GameMessage::new(
                    clock.tick,
                    format!("{} removed from fleet", name),
                    MessageCategory::Event,
                ));
            }
        }
        PanelAction::MergeFleets { fleet_a, fleet_b } => {
            // Guard: both fleets must still exist and neither should be in transit.
            let a_exists = world.fleets.contains_key(fleet_a);
            let b_exists = world.fleets.contains_key(fleet_b);
            let a_transit = movement_state.get(fleet_a).is_some();
            let b_transit = movement_state.get(fleet_b).is_some();

            if !a_exists || !b_exists || a_transit || b_transit {
                // Abort silently — stale action from a previous frame.
            } else {
                // Transfer all ships, fighters, and characters from fleet_b into fleet_a.
                let source = world.fleets.get(fleet_b).unwrap();
                let ships = source.capital_ships.clone();
                let fighters = source.fighters.clone();
                let chars = source.characters.clone();
                let had_ds = source.has_death_star;

                let dest = world.fleets.get_mut(fleet_a).unwrap();
                // Merge capital ships
                for entry in ships {
                    if let Some(existing) = dest.capital_ships.iter_mut().find(|e| e.class == entry.class) {
                        existing.count += entry.count;
                    } else {
                        dest.capital_ships.push(entry);
                    }
                }
                // Merge fighters
                for entry in fighters {
                    if let Some(existing) = dest.fighters.iter_mut().find(|e| e.class == entry.class) {
                        existing.count += entry.count;
                    } else {
                        dest.fighters.push(entry);
                    }
                }
                // Merge characters + update current_fleet
                for ck in &chars {
                    if let Some(c) = world.characters.get_mut(*ck) {
                        c.current_fleet = Some(fleet_a);
                    }
                }
                let dest = world.fleets.get_mut(fleet_a).unwrap();
                for ck in chars {
                    if !dest.characters.contains(&ck) {
                        dest.characters.push(ck);
                    }
                }
                if had_ds {
                    dest.has_death_star = true;
                }

                // Cancel any movement order for fleet_b (defensive).
                movement_state.cancel(fleet_b);

                // Remove fleet_b from its system's fleet list and from the world.
                if let Some(source) = world.fleets.get(fleet_b) {
                    let loc = source.location;
                    if let Some(sys) = world.systems.get_mut(loc) {
                        sys.fleets.retain(|&fk| fk != fleet_b);
                    }
                }
                world.fleets.remove(fleet_b);
            }

            msg_log.push(GameMessage::new(
                clock.tick,
                "Fleets merged".to_string(),
                MessageCategory::Event,
            ));
        }
        PanelAction::Enqueue {
            system,
            kind,
            ticks,
            ..
        } => {
            mfg_state.enqueue(system, QueueItem::new(kind, ticks, ticks));
        }
        PanelAction::CancelQueueItem { system, index } => {
            mfg_state.queue_mut(system).cancel(index);
        }
        PanelAction::PrioritizeQueueItem { system, index } => {
            mfg_state.queue_mut(system).prioritize(index);
        }
        PanelAction::DispatchMission {
            kind,
            faction,
            character,
            target,
            duration_roll,
        } => {
            mission_state.dispatch(kind, faction, character, target, duration_roll);
            let char_name = world
                .characters
                .get(character)
                .map(|c| c.name.clone())
                .unwrap_or_else(|| "Unknown".into());
            let sys_name = world
                .systems
                .get(target)
                .map(|s| s.name.clone())
                .unwrap_or_else(|| "unknown".into());
            let kind_name = match kind {
                MissionKind::Diplomacy      => "Diplomacy",
                MissionKind::Recruitment    => "Recruitment",
                MissionKind::Sabotage       => "Sabotage",
                MissionKind::Assassination  => "Assassination",
                MissionKind::Espionage      => "Espionage",
                MissionKind::Rescue         => "Rescue",
                MissionKind::Abduction      => "Abduction",
                MissionKind::InciteUprising    => "Incite Uprising",
                MissionKind::SubdueUprising    => "Subdue Uprising",
                MissionKind::DeathStarSabotage => "Death Star Sabotage",
                MissionKind::Autoscrap         => "Autoscrap",
            };
            msg_log.push(GameMessage::at_system(
                clock.tick,
                format!("{} dispatched on {} mission to {}", char_name, kind_name, sys_name),
                MessageCategory::Mission,
                target,
            ));
        }
        PanelAction::CancelMission(id) => {
            mission_state.cancel(id);
        }
        // Save/load actions are handled by the caller before dispatching here;
        // they require access to the full save state and are not routed through
        // this helper.
        PanelAction::SaveGame { .. }
        | PanelAction::LoadGame { .. }
        | PanelAction::DeleteSave { .. }
        | PanelAction::CloseSaveLoadPanel => {}
        PanelAction::OpenModManager => {
            // Handled by UI state toggle (not a world mutation)
        }
        PanelAction::ToggleMod { ref name } => {
            mod_runtime.toggle_mod(name);
            msg_log.push(GameMessage::new(
                clock.tick,
                format!("Toggled mod: {}", name),
                MessageCategory::Event,
            ));
        }
        PanelAction::ReloadMods => {
            mod_runtime.refresh();
            let mod_errors = mod_runtime.apply_enabled(world);
            for err in &mod_errors {
                eprintln!("Mod reload error: {:?}", err);
            }
            msg_log.push(GameMessage::new(
                clock.tick,
                format!("Reloaded {} mods", mod_runtime.discovered.len()),
                MessageCategory::Event,
            ));
        }
        PanelAction::OpenMissionTo { target, kind: _, faction: _ } => {
            // Select the target system and open missions panel.
            // The mission panel will let the player pick character + confirm.
            map_state.selected_system = Some(target);
            // TODO: pre-select mission kind in MissionsPanelState when
            // the panel supports target pre-selection.
        }
        PanelAction::InitiateFleetMove { destination } => {
            // Select the destination system so the player can see it,
            // and open the fleets panel for move order dispatch.
            map_state.selected_system = Some(destination);
            // TODO: implement fleet move selection flow in fleets panel.
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
        PanelAction::SetGameSpeed(speed) => {
            let game_speed = match speed {
                0 => GameSpeed::Paused,
                1 => GameSpeed::Normal,
                2 => GameSpeed::Fast,
                _ => GameSpeed::Faster,
            };
            clock.set_speed(game_speed);
        }
        PanelAction::ToggleDualAI => {
            *dual_ai_mode = !*dual_ai_mode;
            if *dual_ai_mode {
                // Create persistent second AI for the opposite faction
                let second_faction = match ai_state.faction {
                    Some(AiFaction::Empire) => AiFaction::Alliance,
                    _ => AiFaction::Empire,
                };
                *ai2_state = Some(AIState::new(second_faction));
            } else {
                *ai2_state = None;
            }
            let state_str = if *dual_ai_mode { "ENABLED" } else { "DISABLED" };
            msg_log.push(GameMessage::new(
                clock.tick,
                format!("Dual AI mode {}", state_str),
                MessageCategory::Event,
            ));
        }
        PanelAction::ForceVictoryCheck => {
            let tick_ev = rebellion_core::tick::TickEvent { tick: clock.tick };
            let result = rebellion_core::victory::VictorySystem::check(victory_state, world, &[tick_ev]);
            if let Some(outcome) = result {
                msg_log.push(GameMessage::new(
                    clock.tick,
                    format!("Victory check: {:?}", outcome),
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
            for (sys_key, _) in world.systems.iter() {
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
                    eprintln!("Failed to export game log: {}", e);
                }
            }
        }
        PanelAction::ShowGameStats => {
            let alliance_systems = world.systems.values().filter(|s| s.control.is_controlled_by(Faction::Alliance)).count();
            let empire_systems = world.systems.values().filter(|s| s.control.is_controlled_by(Faction::Empire)).count();
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
                msg_log.push(GameMessage::new(clock.tick, "No active missions".to_string(), MessageCategory::Mission));
            } else {
                msg_log.push(GameMessage::new(clock.tick, format!("{} active missions:", missions.len()), MessageCategory::Mission));
                for m in missions {
                    let char_name = world.characters.get(m.character)
                        .map(|c| c.name.clone())
                        .unwrap_or_else(|| "Unknown".into());
                    let sys_name = world.systems.get(m.target_system)
                        .map(|s| s.name.clone())
                        .unwrap_or_else(|| "unknown".into());
                    msg_log.push(GameMessage::new(
                        clock.tick,
                        format!("  {:?} — {} at {}", m.kind, char_name, sys_name),
                        MessageCategory::Mission,
                    ));
                }
            }
        }
        PanelAction::ListActiveFleets => {
            msg_log.push(GameMessage::new(clock.tick, format!("{} fleets:", world.fleets.len()), MessageCategory::Event));
            for (_, fleet) in world.fleets.iter() {
                let sys_name = world.systems.get(fleet.location)
                    .map(|s| s.name.clone())
                    .unwrap_or_else(|| "unknown".into());
                let faction = if fleet.is_alliance { "Alliance" } else { "Empire" };
                let ship_count = fleet.capital_ships.iter().map(|e| e.count as usize).sum::<usize>()
                    + fleet.fighters.iter().map(|e| e.count as usize).sum::<usize>();
                msg_log.push(GameMessage::new(
                    clock.tick,
                    format!("  {} fleet at {} — {} ships",
                        faction, sys_name, ship_count),
                    MessageCategory::Event,
                ));
            }
        }
        PanelAction::ShowEventCount => {
            let total = event_state.events().len();
            let fired = event_state.events().iter().filter(|e| event_state.has_fired(e.id)).count();
            msg_log.push(GameMessage::new(
                clock.tick,
                format!("Events: {} defined, {} fired", total, fired),
                MessageCategory::Event,
            ));
        }

        // ── Research ─────────────────────────────────────────────────────────
        PanelAction::DispatchResearch { character, tech_type, faction } => {
            let is_alliance = faction == MissionFaction::Alliance;
            let current_level = research_state.level(is_alliance, tech_type);
            // Calculate research duration from world data
            let ticks = rebellion_core::research::ResearchSystem::ticks_for_next_level(
                world, is_alliance, tech_type, current_level,
            );
            let project = rebellion_core::research::ResearchProject {
                tech_type,
                character,
                faction_is_alliance: is_alliance,
                ticks_remaining: ticks,
                total_ticks: ticks,
            };
            research_state.dispatch(project);
            let char_name = world.characters.get(character)
                .map(|c| c.name.clone())
                .unwrap_or_else(|| "Unknown".into());
            let tree_name = match tech_type {
                rebellion_core::research::TechType::Ship => "Ship",
                rebellion_core::research::TechType::Troop => "Troop",
                rebellion_core::research::TechType::Facility => "Facility",
            };
            msg_log.push(GameMessage::new(
                clock.tick,
                format!("{} assigned to {} research (level {} → {}, {} ticks)",
                    char_name, tree_name, current_level, current_level + 1, ticks),
                MessageCategory::Event,
            ));
        }
        PanelAction::CancelResearch { tech_type, faction } => {
            let is_alliance = faction == MissionFaction::Alliance;
            research_state.cancel(is_alliance, tech_type);
            let tree_name = match tech_type {
                rebellion_core::research::TechType::Ship => "Ship",
                rebellion_core::research::TechType::Troop => "Troop",
                rebellion_core::research::TechType::Facility => "Facility",
            };
            msg_log.push(GameMessage::new(
                clock.tick,
                format!("{} research cancelled", tree_name),
                MessageCategory::Event,
            ));
        }

        // ── Jedi Training ────────────────────────────────────────────────────
        PanelAction::StartJediTraining { character, faction } => {
            let is_alliance = faction == MissionFaction::Alliance;
            jedi_state.start_training(character, is_alliance, clock.tick);
            let char_name = world.characters.get(character)
                .map(|c| c.name.clone())
                .unwrap_or_else(|| "Unknown".into());
            msg_log.push(GameMessage::new(
                clock.tick,
                format!("{} begins Force training", char_name),
                MessageCategory::Event,
            ));
        }
        PanelAction::StopJediTraining { character } => {
            jedi_state.stop_training(character);
            let char_name = world.characters.get(character)
                .map(|c| c.name.clone())
                .unwrap_or_else(|| "Unknown".into());
            msg_log.push(GameMessage::new(
                clock.tick,
                format!("{} Force training stopped", char_name),
                MessageCategory::Event,
            ));
        }
    }
}

// ---------------------------------------------------------------------------
// Effect application helpers
// ---------------------------------------------------------------------------

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
        MissionKind::Diplomacy      => "Diplomacy",
        MissionKind::Recruitment    => "Recruitment",
        MissionKind::Sabotage       => "Sabotage",
        MissionKind::Assassination  => "Assassination",
        MissionKind::Espionage      => "Espionage",
        MissionKind::Rescue         => "Rescue",
        MissionKind::Abduction      => "Abduction",
        MissionKind::InciteUprising    => "Incite Uprising",
        MissionKind::SubdueUprising    => "Subdue Uprising",
        MissionKind::DeathStarSabotage => "Death Star Sabotage",
        MissionKind::Autoscrap         => "Autoscrap",
    };
    let sys_name = world
        .systems
        .get(result.target_system)
        .map(|s| s.name.clone())
        .unwrap_or_else(|| "unknown".into());
    let outcome_str = match result.outcome {
        rebellion_core::missions::MissionOutcome::Success => "succeeded",
        rebellion_core::missions::MissionOutcome::Failure => "failed",
        rebellion_core::missions::MissionOutcome::Foiled => "was foiled",
    };

    let category = match result.kind {
        MissionKind::Diplomacy
        | MissionKind::InciteUprising
        | MissionKind::SubdueUprising         => MessageCategory::Diplomacy,
        MissionKind::Recruitment
        | MissionKind::Sabotage
        | MissionKind::Assassination
        | MissionKind::Espionage
        | MissionKind::Rescue
        | MissionKind::Abduction
        | MissionKind::DeathStarSabotage
        | MissionKind::Autoscrap              => MessageCategory::Mission,
    };
    log.push(GameMessage::at_system(
        result.tick,
        format!(
            "{} {} mission at {} {}",
            faction_name, kind_name, sys_name, outcome_str
        ),
        category,
        result.target_system,
    ));

    // SFX for mission outcomes
    #[cfg(not(target_arch = "wasm32"))]
    match result.outcome {
        rebellion_core::missions::MissionOutcome::Success => {
            audio_engine.play_sfx(SfxKind::MissionSuccess, audio_vol);
        }
        _ => {
            audio_engine.play_sfx(SfxKind::MissionFail, audio_vol);
        }
    }

    for effect in &result.effects {
        match effect {
            MissionEffect::PopularityShifted { system, faction, delta } => {
                if let Some(sys) = world.systems.get_mut(*system) {
                    match faction {
                        MissionFaction::Alliance => {
                            sys.popularity_alliance =
                                (sys.popularity_alliance + delta).clamp(0.0, 1.0);
                        }
                        MissionFaction::Empire => {
                            sys.popularity_empire =
                                (sys.popularity_empire + delta).clamp(0.0, 1.0);
                        }
                    }
                }
            }
            MissionEffect::UprisingStarted { system, popularity_delta } => {
                // Shift popularity against the controlling faction.
                if let Some(sys) = world.systems.get_mut(*system) {
                    sys.popularity_alliance =
                        (sys.popularity_alliance + popularity_delta).clamp(0.0, 1.0);
                    sys.popularity_empire =
                        (sys.popularity_empire - popularity_delta).clamp(0.0, 1.0);
                }
            }
            MissionEffect::SystemIntelligenceGathered { system, .. } => {
                // Reveal fog: mark system as explored (full implementation in fog task).
                if let Some(sys) = world.systems.get_mut(*system) {
                    sys.exploration_status = rebellion_core::dat::ExplorationStatus::Explored;
                }
            }
            MissionEffect::CharacterRecruited { faction, .. } => {
                // Recruitment shifts the recruiter's faction allegiance
                // (the recruit joins the faction that sent the recruiter)
                let _ = faction; // effect is already applied via PopularityShifted
            }
            MissionEffect::FacilitySabotaged { system, facility_index, ticks_lost } => {
                // Remove the facility at facility_index from the system
                if let Some(sys) = world.systems.get_mut(*system) {
                    if *facility_index < sys.manufacturing_facilities.len() {
                        let fac_key = sys.manufacturing_facilities.remove(*facility_index);
                        world.manufacturing_facilities.remove(fac_key);
                    } else if *facility_index < sys.manufacturing_facilities.len() + sys.defense_facilities.len() {
                        let adj_idx = *facility_index - sys.manufacturing_facilities.len();
                        let fac_key = sys.defense_facilities.remove(adj_idx);
                        world.defense_facilities.remove(fac_key);
                    }
                }
                let _ = ticks_lost;
            }
            MissionEffect::CharacterKilled { character, .. } => {
                // Remove character from any fleet they're assigned to
                for (_, fleet) in world.fleets.iter_mut() {
                    fleet.characters.retain(|&k| k != *character);
                }
                world.characters.remove(*character);
            }
            MissionEffect::CharacterCaptured { character, captured_by, .. } => {
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
                for (_, fleet) in world.fleets.iter_mut() {
                    fleet.characters.retain(|&k| k != *character);
                }
            }
            MissionEffect::CharacterRescued { character, returned_to, .. } => {
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
            MissionEffect::CharacterBusy { character } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.on_mission = true;
                }
            }
            MissionEffect::CharacterAvailable { character } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.on_mission = false;
                    c.on_hidden_mission = false;
                }
            }
            MissionEffect::DecoyTriggered { system, decoy_character } => {
                let sys_name = world.systems.get(*system).map(|s| s.name.clone()).unwrap_or_else(|| "unknown".into());
                let char_name = world.characters.get(*decoy_character).map(|c| c.name.clone()).unwrap_or_else(|| "Unknown".into());
                log.push(GameMessage::at_system(result.tick, format!("Mission intercepted by decoy {} at {}", char_name, sys_name), MessageCategory::Mission, *system));
            }
            MissionEffect::CharacterEscaped { character, escaped_to_alliance } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.is_alliance = *escaped_to_alliance;
                    c.is_empire = !*escaped_to_alliance;
                    c.is_captive = false;
                    c.captured_by = None;
                    c.capture_tick = None;
                }
                let name = world.characters.get(*character).map(|c| c.name.clone()).unwrap_or_else(|| "Unknown".into());
                log.push(GameMessage::new(result.tick, format!("{} has escaped captivity!", name), MessageCategory::Event));
            }
            MissionEffect::UprisingSubdued { system } => {
                // Shift popularity toward controlling faction
                if let Some(sys) = world.systems.get_mut(*system) {
                    match sys.control {
                        ControlKind::Controlled(Faction::Alliance) => {
                            sys.popularity_alliance = (sys.popularity_alliance + 0.05).clamp(0.0, 1.0);
                            sys.popularity_empire = (sys.popularity_empire - 0.05).clamp(0.0, 1.0);
                        }
                        _ => {
                            sys.popularity_empire = (sys.popularity_empire + 0.05).clamp(0.0, 1.0);
                            sys.popularity_alliance = (sys.popularity_alliance - 0.05).clamp(0.0, 1.0);
                        }
                    }
                }
                // Clear uprising — uprising_state not accessible here, handled in simulation layer
            }
            MissionEffect::DeathStarSabotaged { ticks_delayed } => {
                // Death Star delay applied in simulation layer (death_star_state.add_sabotage_delay)
                log.push(GameMessage::new(result.tick,
                    format!("Death Star construction sabotaged! {} ticks delayed.", ticks_delayed),
                    MessageCategory::Mission));
            }
        }
    }
}

fn apply_event_actions(
    actions: &[EventAction],
    world: &mut GameWorld,
    log: &mut MessageLog,
    tick: u64,
) {
    for action in actions {
        match action {
            EventAction::DisplayMessage { text } => {
                log.push(GameMessage::new(
                    tick,
                    text.clone(),
                    MessageCategory::Event,
                ));
            }
            EventAction::ShiftPopularity {
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
            EventAction::ModifyCharacterSkill {
                character,
                skill,
                base_delta,
            } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    let d = *base_delta;
                    let apply =
                        |v: u32, delta: i32| (v as i64 + delta as i64).max(0) as u32;
                    match skill {
                        SkillField::Diplomacy => {
                            c.diplomacy.base = apply(c.diplomacy.base, d)
                        }
                        SkillField::Espionage => {
                            c.espionage.base = apply(c.espionage.base, d)
                        }
                        SkillField::ShipDesign => {
                            c.ship_design.base = apply(c.ship_design.base, d)
                        }
                        SkillField::TroopTraining => {
                            c.troop_training.base = apply(c.troop_training.base, d)
                        }
                        SkillField::FacilityDesign => {
                            c.facility_design.base = apply(c.facility_design.base, d)
                        }
                        SkillField::Combat => c.combat.base = apply(c.combat.base, d),
                        SkillField::Leadership => {
                            c.leadership.base = apply(c.leadership.base, d)
                        }
                        SkillField::Loyalty => c.loyalty.base = apply(c.loyalty.base, d),
                        SkillField::JediLevel => {
                            c.jedi_level.base = apply(c.jedi_level.base, d)
                        }
                    }
                }
            }
            EventAction::RelocateCharacter { .. } => {
                // Deferred: character location tracking lands with full character system.
            }
            // v0.5.0 story event actions — full implementation in Pidray's integration pass
            EventAction::SetMandatoryMission { character, mandatory } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.on_mandatory_mission = *mandatory;
                }
            }
            EventAction::ModifyForceTier { character, new_tier } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.force_tier = *new_tier;
                }
            }
            EventAction::RemoveCharacter { character } => {
                for (_, fleet) in world.fleets.iter_mut() {
                    fleet.characters.retain(|&k| k != *character);
                }
                world.characters.remove(*character);
            }
            EventAction::StartJediTraining { .. } => {
                // Handled by the StartJediTraining extraction loop above (needs jedi_state).
                // Do NOT add logic here — it would double-enroll characters.
            }
            EventAction::TransferCharacter { character, destination, new_faction } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.current_system = Some(*destination);
                    if let Some(faction) = new_faction {
                        match faction {
                            Faction::Alliance => { c.is_alliance = true; c.is_empire = false; }
                            Faction::Empire => { c.is_alliance = false; c.is_empire = true; }
                            _ => {}
                        }
                    }
                }
            }
            EventAction::TriggerEvent { .. } => {
                // Event chaining handled internally by EventSystem
            }
            EventAction::AccumulateForceExperience { character, amount } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.force_experience += amount;
                }
            }
            EventAction::CaptureCharacter { character, captor_faction } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.is_captive = true;
                    c.captured_by = Some(*captor_faction);
                    c.capture_tick = Some(tick);
                }
                // Remove from fleets
                for (_, fleet) in world.fleets.iter_mut() {
                    fleet.characters.retain(|&k| k != *character);
                }
            }
            EventAction::SetCarboniteState { character, frozen } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.on_mandatory_mission = *frozen;
                    if *frozen {
                        // Carbonite freeze = captive state + mandatory mission lock
                        c.is_captive = true;
                        c.capture_tick = Some(tick);
                    } else {
                        // Thaw = clear all captivity
                        c.is_captive = false;
                        c.captured_by = None;
                        c.capture_tick = None;
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Combat effect application helpers
// ---------------------------------------------------------------------------

/// Apply a `SpaceCombatResult` to `GameWorld`.
///
/// Maps destroyed hull snapshots (hull_after == 0) back to fleet `capital_ships`
/// counts. Since `ShipDamageEvent.ship_index` is the per-hull expansion index
/// (not a class index), we convert: the first `entry.count` indices map to
/// entry[0], the next `entry.count` to entry[1], and so on.
/// Ships reduced to hull=0 decrement their entry count.
/// Entries reaching count=0 are removed; fleets with no ships are removed from
/// their system's fleet list.
fn apply_space_combat_result(
    result: &rebellion_core::combat::SpaceCombatResult,
    world: &mut GameWorld,
) {
    for evt in &result.ship_damage {
        if evt.hull_after > 0 { continue; } // still alive — no removal needed

        let fleet_key = evt.fleet;
        // Determine which entry this ship_index falls into.
        let entry_idx = {
            let fleet = match world.fleets.get(fleet_key) {
                Some(f) => f,
                None => continue,
            };
            let mut offset = 0usize;
            let mut found = None;
            for (i, entry) in fleet.capital_ships.iter().enumerate() {
                if evt.ship_index < offset + entry.count as usize {
                    found = Some(i);
                    break;
                }
                offset += entry.count as usize;
            }
            found
        };
        if let Some(idx) = entry_idx {
            if let Some(fleet) = world.fleets.get_mut(fleet_key) {
                if fleet.capital_ships[idx].count > 0 {
                    fleet.capital_ships[idx].count -= 1;
                }
            }
        }
    }

    // Remove empty fleets from system fleet lists.
    for &fleet_key in &[result.attacker_fleet, result.defender_fleet] {
        let is_empty = world.fleets.get(fleet_key)
            .map(|f| f.capital_ships.iter().all(|e| e.count == 0)
                && f.fighters.iter().all(|e| e.count == 0))
            .unwrap_or(true);
        if is_empty {
            if let Some(fleet) = world.fleets.get(fleet_key) {
                let loc = fleet.location;
                if let Some(sys) = world.systems.get_mut(loc) {
                    sys.fleets.retain(|&k| k != fleet_key);
                }
            }
            world.fleets.remove(fleet_key);
        }
    }
}

/// Apply a `GroundCombatResult` to `GameWorld`.
///
/// Updates `TroopUnit.regiment_strength` for all damaged units.
/// Units reaching strength ≤ 0 are removed from the system's ground_units list.
fn apply_ground_combat_result(
    result: &rebellion_core::combat::GroundCombatResult,
    world: &mut GameWorld,
) {
    // Apply the last recorded strength for each troop.
    let mut final_strengths: std::collections::HashMap<rebellion_core::ids::TroopKey, i16> =
        std::collections::HashMap::new();
    for evt in &result.troop_damage {
        final_strengths.insert(evt.troop, evt.strength_after);
    }
    for (&key, &strength) in &final_strengths {
        if let Some(troop) = world.troops.get_mut(key) {
            troop.regiment_strength = strength;
        }
    }

    // Remove destroyed troops from system.
    let sys_key = result.system;
    if let Some(sys) = world.systems.get_mut(sys_key) {
        sys.ground_units.retain(|&k| {
            world.troops.get(k).map(|t| t.regiment_strength > 0).unwrap_or(false)
        });
    }
    // Remove destroyed troops from the world arena.
    let dead: Vec<_> = final_strengths.iter()
        .filter(|(_, &s)| s <= 0)
        .map(|(&k, _)| k)
        .collect();
    for key in dead {
        world.troops.remove(key);
    }
}

fn apply_ai_actions(
    actions: &[AIAction],
    rolls: &[f64],
    ai_state: &mut AIState,
    mission_state: &mut MissionState,
    mfg_state: &mut ManufacturingState,
    movement_state: &mut MovementState,
    research_state: &mut ResearchState,
    world: &GameWorld,
    log: &mut MessageLog,
    tick: u64,
    #[cfg(not(target_arch = "wasm32"))] audio_engine: &mut audio::AudioEngine,
    #[cfg(not(target_arch = "wasm32"))] audio_vol: &AudioVolumeState,
) {
    let mut roll_idx = 0;
    for action in actions {
        match action {
            AIAction::DispatchMission {
                kind,
                character,
                target_system,
                duration_roll,
            } => {
                let roll = rolls.get(roll_idx).copied().unwrap_or(*duration_roll);
                roll_idx += 1;
                mission_state.dispatch(
                    *kind,
                    MissionFaction::Empire,
                    *character,
                    *target_system,
                    roll,
                );
                ai_state.mark_busy(*character);
                log.push(GameMessage::at_system(
                    tick,
                    format!("Empire dispatched {:?} mission", kind),
                    MessageCategory::Ai,
                    *target_system,
                ));
            }
            AIAction::EnqueueProduction {
                system,
                kind,
                ticks,
            } => {
                mfg_state.enqueue(*system, QueueItem::new(*kind, *ticks, *ticks));
            }
            AIAction::MoveFleet {
                fleet,
                to_system,
                reason,
            } => {
                let reason_str = match reason {
                    FleetMoveReason::Attack => "attack",
                    FleetMoveReason::Reinforce => "reinforce",
                };
                // Only issue a new order if the fleet isn't already in transit
                // to the same destination. Re-issuing would reset ticks_elapsed to 0.
                let already_moving = movement_state
                    .get(*fleet)
                    .map(|o| o.destination == *to_system)
                    .unwrap_or(false);
                if !already_moving {
                    if let Some(f) = world.fleets.get(*fleet) {
                        let transit =
                            rebellion_core::movement::fleet_transit_ticks(f, world, f.location, *to_system);
                        movement_state.order(*fleet, f.location, *to_system, transit);
                        #[cfg(not(target_arch = "wasm32"))]
                        audio_engine.play_sfx(SfxKind::FleetDeparture, audio_vol);
                    }
                }
                log.push(GameMessage::at_system(
                    tick,
                    format!("Empire fleet moving to system ({})", reason_str),
                    MessageCategory::Ai,
                    *to_system,
                ));
            }
            AIAction::DispatchResearch {
                character, tech_type, ticks,
            } => {
                let is_alliance = ai_state.faction
                    .map(|f| matches!(f, AiFaction::Alliance))
                    .unwrap_or(false);
                research_state.dispatch(rebellion_core::research::ResearchProject {
                    tech_type: *tech_type,
                    character: *character,
                    faction_is_alliance: is_alliance,
                    ticks_remaining: *ticks,
                    total_ticks: *ticks,
                });
                ai_state.mark_busy(*character);
                let char_name = world.characters.get(*character)
                    .map(|c| c.name.as_str()).unwrap_or("unknown");
                log.push(GameMessage::new(
                    tick,
                    format!("{} assigned to {:?} research ({} ticks)", char_name, tech_type, ticks),
                    MessageCategory::Ai,
                ));
            }
        }
    }
}
