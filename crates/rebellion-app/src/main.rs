mod audio;

use macroquad::prelude::*;
use std::path::{Path, PathBuf};

use rebellion_core::ai::{AiFaction, AIAction, AIState, AISystem, FleetMoveReason};
use rebellion_core::dat::Faction;
use rebellion_core::events::{EventAction, EventState, EventSystem, SkillField};
use rebellion_core::fog::{FogState, FogSystem};
use rebellion_core::manufacturing::{ManufacturingState, ManufacturingSystem, QueueItem};
use rebellion_core::missions::{
    MissionEffect, MissionFaction, MissionKind, MissionState, MissionSystem,
};
use rebellion_core::movement::{MovementState, MovementSystem};
use rebellion_core::tick::{GameClock, GameSpeed};
use rebellion_core::world::GameWorld;

use rebellion_render::{
    draw_encyclopedia, draw_faction_select, draw_fleet_overlays, draw_fleets, draw_fog_overlay,
    draw_galaxy_map, draw_manufacturing, draw_message_log, draw_missions, draw_officers,
    draw_status_bar, draw_system_info_panel, AudioVolumeState, EncyclopediaState,
    FactionSelectState, FleetsState, GalaxyMapState, GameMessage, ManufacturingPanelState,
    MessageCategory, MessageLog, MessageLogState, MissionsPanelState, OfficersState, PanelAction,
    SfxKind,
};

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

    let mut world = match rebellion_data::load_game_data(&gdata_path) {
        Ok(w) => w,
        Err(e) => {
            eprintln!(
                "Failed to load game data from {}: {}",
                gdata_path.display(),
                e
            );
            #[cfg(not(target_arch = "wasm32"))]
            std::process::exit(1);
            #[cfg(target_arch = "wasm32")]
            panic!("Failed to load game data from {}: {}", gdata_path.display(), e);
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

    // ── Simulation state ────────────────────────────────────────────────────
    let mut clock = GameClock::new();
    let mut mfg_state = ManufacturingState::new();
    let mut mission_state = MissionState::new();
    let mut event_state = EventState::new();
    let mut ai_state = AIState::new(AiFaction::Empire);
    let mut movement_state = MovementState::new();
    let mut fog_state = FogState::new(Faction::Alliance);
    FogSystem::seed(&mut fog_state, &world);

    // ── UI state ────────────────────────────────────────────────────────────
    let mut map_state = GalaxyMapState::default();
    let mut msg_log = MessageLog::default();
    let mut log_state = MessageLogState::default();

    // ── War Room panel state ────────────────────────────────────────────────
    let mut faction_select_state = FactionSelectState::default();
    let mut player_faction = MissionFaction::Alliance;
    let mut officers_state = OfficersState::default();
    let mut fleets_state = FleetsState::default();
    let mut mfg_panel_state = ManufacturingPanelState::default();
    let mut missions_panel_state = MissionsPanelState::default();
    let mut enc_state = EncyclopediaState::new();
    enc_state.set_edata_path(gdata_path.join("EData"));

    // Panel visibility (mutually exclusive left panels)
    let mut show_officers = false;
    let mut show_fleets = false;
    let mut show_manufacturing = false;
    let mut show_missions = false;

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

    loop {
        let dt = get_frame_time();

        // ── Keyboard shortcuts ──────────────────────────────────────────────
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
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
        if is_key_pressed(KeyCode::E) {
            enc_state.open = !enc_state.open;
        }

        // ── Tick the clock ──────────────────────────────────────────────────
        let tick_events = clock.advance(dt);

        if !tick_events.is_empty() {
            // ── Manufacturing ───────────────────────────────────────────────
            let completions = ManufacturingSystem::advance(&mut mfg_state, &tick_events);
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
                .map(|_| rand::gen_range(0.0f64, 1.0f64))
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

            // ── Events ──────────────────────────────────────────────────────
            let event_rolls: Vec<f32> =
                (0..16).map(|_| rand::gen_range(0.0f32, 1.0f32)).collect();
            let fired_events =
                EventSystem::advance(&mut event_state, &world, &tick_events, &event_rolls);

            for fired in &fired_events {
                apply_event_actions(&fired.actions, &mut world, &mut msg_log, fired.tick);
            }

            // ── AI ──────────────────────────────────────────────────────────
            let ai_actions = AISystem::advance(
                &mut ai_state,
                &world,
                &mfg_state,
                &mission_state,
                &tick_events,
            );
            let ai_rolls: Vec<f64> =
                (0..8).map(|_| rand::gen_range(0.0f64, 1.0f64)).collect();
            apply_ai_actions(
                &ai_actions,
                &ai_rolls,
                &mut ai_state,
                &mut mission_state,
                &mut mfg_state,
                &mut movement_state,
                &world,
                &mut msg_log,
                tick_events.last().map(|e| e.tick).unwrap_or(0),
                #[cfg(not(target_arch = "wasm32"))]
                &mut audio_engine,
                #[cfg(not(target_arch = "wasm32"))]
                &audio_vol,
            );
        }

        // ── Rendering ───────────────────────────────────────────────────────

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

        // 4. All egui panels in a single ui() + draw() pass
        let mut panel_actions: Vec<PanelAction> = Vec::new();

        egui_macroquad::ui(|ctx| {
            // Faction select modal (gates everything else)
            if faction_select_state.chosen.is_none() {
                if let Some(action) = draw_faction_select(ctx, &mut faction_select_state) {
                    panel_actions.push(action);
                }
            } else {
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
                        draw_fleets(ctx, &world, &mut fleets_state, player_faction)
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
                    let duration_roll = rand::gen_range(0.0f64, 1.0f64);
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

                // Encyclopedia (floating window, always available when faction chosen)
                if let Some(sys_key) = draw_encyclopedia(ctx, &world, &mut enc_state) {
                    panel_actions.push(PanelAction::FocusFleetSystem(sys_key));
                }
            }

            // System info panel (right side)
            draw_system_info_panel(ctx, &world, &map_state);

            // Message log (bottom panel, above status bar)
            draw_message_log(ctx, &msg_log, &mut log_state);

            // Status bar (bottom-most, with speed controls + audio)
            draw_status_bar(ctx, &world, &mut clock, &mut audio_vol);
        });
        egui_macroquad::draw();

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
                &mut msg_log,
                &mut player_faction,
                &clock,
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
    _movement_state: &mut MovementState,
    fog_state: &mut FogState,
    ai_state: &mut AIState,
    msg_log: &mut MessageLog,
    player_faction: &mut MissionFaction,
    clock: &GameClock,
    #[cfg(not(target_arch = "wasm32"))] audio_engine: &mut audio::AudioEngine,
    #[cfg(not(target_arch = "wasm32"))] audio_vol: &AudioVolumeState,
    #[cfg(not(target_arch = "wasm32"))] sounds_dir: &Path,
) {
    match action {
        PanelAction::SelectFaction(faction) => {
            *player_faction = faction;
            // Re-seed fog for the correct faction
            let dat_faction = match faction {
                MissionFaction::Alliance => Faction::Alliance,
                MissionFaction::Empire => Faction::Empire,
            };
            *fog_state = FogState::new(dat_faction);
            FogSystem::seed(fog_state, world);
            // Flip AI to control the opposite faction
            if faction == MissionFaction::Empire {
                *ai_state = AIState::new(AiFaction::Alliance);
            } else {
                *ai_state = AIState::new(AiFaction::Empire);
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
                sounds_dir,
                audio_vol,
            );
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
                MissionKind::Diplomacy => "Diplomacy",
                MissionKind::Recruitment => "Recruitment",
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
        MissionKind::Diplomacy => "Diplomacy",
        MissionKind::Recruitment => "Recruitment",
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
        MissionKind::Diplomacy => MessageCategory::Diplomacy,
        MissionKind::Recruitment => MessageCategory::Mission,
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
            MissionEffect::PopularityShifted {
                system,
                faction,
                delta,
            } => {
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
            MissionEffect::CharacterRecruited { .. } => {
                // Deferred: character location tracking lands with full character system.
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
        }
    }
}

fn apply_ai_actions(
    actions: &[AIAction],
    rolls: &[f64],
    ai_state: &mut AIState,
    mission_state: &mut MissionState,
    mfg_state: &mut ManufacturingState,
    movement_state: &mut MovementState,
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
                // Issue actual movement order if fleet has a valid location
                if let Some(f) = world.fleets.get(*fleet) {
                    let ticks_per_hop =
                        rebellion_core::movement::fleet_ticks_per_hop(f, world);
                    movement_state.order(*fleet, f.location, *to_system, ticks_per_hop);
                    #[cfg(not(target_arch = "wasm32"))]
                    audio_engine.play_sfx(SfxKind::FleetDeparture, audio_vol);
                }
                log.push(GameMessage::at_system(
                    tick,
                    format!("Empire fleet moving to system ({})", reason_str),
                    MessageCategory::Ai,
                    *to_system,
                ));
            }
        }
    }
}
