//! Galaxy map rendering and egui UI panels.

pub mod advisor;
pub mod audio;
pub mod bmp_cache;
pub mod cockpit;
pub mod combat_view;
pub mod encyclopedia;
pub mod event_screen;
pub mod fleet_movement;
pub mod fog;
pub mod ground_combat;
pub mod main_menu;
pub mod main_menu_destinations;
pub mod message_log;
pub mod panels;
pub mod sector_window;
pub mod system_window;
#[cfg(feature = "interface-test-fixtures")]
mod tactical_assets;
pub mod tactical_view;
pub mod theme;
pub mod victory_screen;
pub mod video_player;

use egui_macroquad::egui;
use macroquad::prelude::*;
use rebellion_core::blockade::BlockadeState;
use rebellion_core::dat::ExplorationStatus;
use rebellion_core::economy::EconomyState;
use rebellion_core::ids::{FleetKey, SystemKey};
use rebellion_core::manufacturing::ManufacturingState;
use rebellion_core::missions::{MissionFaction, MissionState};
use rebellion_core::movement::MovementState;
use rebellion_core::tick::{GameClock, GameSpeed};
use rebellion_core::world::{ControlKind, GameWorld, System};

#[cfg(target_arch = "wasm32")]
pub use advisor::set_advisor_asset_cache;
pub use advisor::{
    advisor_combat_result, advisor_death_star, advisor_greet, advisor_manufacturing_complete,
    advisor_mission_result, advisor_uprising, draw_advisor, AdvisorFaction, AdvisorMessage,
    AdvisorPriority, AdvisorState,
};
pub use audio::{
    draw_audio_controls, AudioVolumeState, MusicContext, MusicTrack, SfxKind, VoiceLine,
};
#[cfg(target_arch = "wasm32")]
pub use bmp_cache::set_bmp_cache;
pub use bmp_cache::{AssetRenderProfile, BmpCache, DllSource};
pub use cockpit::{
    draw_cockpit_background, draw_cockpit_chrome, draw_cockpit_egui_layer,
    handle_cockpit_egui_input, set_cockpit_viewport_clip, strategic_gid_control,
    strategic_primary_controls, CockpitButton, CockpitFaction, CockpitLayout, CockpitState,
    CockpitViewport, GidMode, StrategicControlSpec, STRATEGIC_LOGICAL_HEIGHT,
    STRATEGIC_LOGICAL_WIDTH,
};
pub use combat_view::{draw_combat_summary, BattleOutcome, CombatResult, CombatSummaryState};
pub use encyclopedia::{draw_encyclopedia, EncyclopediaState, EncyclopediaTab};
pub use event_screen::{
    draw_event_screen, show_event_screen, show_event_screen_raw, update_event_screen,
    EventScreenState,
};
pub use fleet_movement::{draw_fleet_overlays, hovered_fleet};
pub use fog::draw_fog_overlay;
pub use ground_combat::{draw_ground_combat, GroundAction, GroundCombatState, GroundWinner};
pub use main_menu::{draw_main_menu, MainMenuAction, MainMenuControl, MainMenuState};
pub use main_menu_destinations::{
    draw_credits, draw_multiplayer_setup, CreditsState, MenuDestinationAction,
    MultiplayerSetupAction, MultiplayerSetupState, MultiplayerTransport,
};
pub use message_log::{
    draw_message_log, GameMessage, MessageCategory, MessageLog, MessageLogState,
};
pub use panels::game_setup::{draw_game_setup, Difficulty, GameSetupAction, GameSetupState};
pub use panels::{
    draw_fleets, draw_manufacturing, draw_missions, draw_mod_manager, draw_officers,
    draw_save_load, FleetsState, ManufacturingPanelState, MissionsPanelState, ModInfo,
    ModManagerAction, ModManagerState, OfficersState, PanelAction, SaveLoadPanelState,
    SaveSlotInfo,
};
pub use sector_window::{
    draw_sector_windows, SectorWindowAction, SectorWindowState, SECTOR_WINDOW_HEIGHT,
    SECTOR_WINDOW_WIDTH,
};
pub use system_window::{
    draw_system_windows, SystemWindowAction, SystemWindowState, SystemWindowTab,
    REFERENCE_RAIL_SLOTS, SYSTEM_WINDOW_CLIENT_WIDTH, SYSTEM_WINDOW_HEIGHT, SYSTEM_WINDOW_WIDTH,
};
#[cfg(all(feature = "interface-test-fixtures", not(target_arch = "wasm32")))]
pub use tactical_assets::install_native_tactical_proof;
#[cfg(feature = "interface-test-fixtures")]
pub use tactical_assets::set_tactical_asset_cache;
pub use tactical_view::{
    draw_tactical_view, BattlePhase, BattleSession, CombatWinner, TacticalAction, TacticalState,
};
pub use victory_screen::{draw_victory_screen, GameStats, VictoryScreenState};
pub use video_player::{VideoError, VideoPlayer};

#[cfg(debug_assertions)]
pub use panels::command_palette::{draw_command_palette, CommandPaletteState};

/// Computed camera parameters for overlay rendering.
///
/// Returned by `draw_galaxy_map` so fog/fleet overlays can use matching
/// coordinates without recomputing screen dimensions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraView {
    pub cam_x: f32,
    pub cam_y: f32,
    /// Effective screen-space zoom, including the strategic canvas scale.
    pub zoom: f32,
    /// Player-controlled gameplay zoom, independent of window size.
    pub logical_zoom: f32,
    /// Uniform scale from original 640×480 pixels to screen pixels.
    pub display_scale: f32,
    pub viewport_x: f32,
    pub viewport_y: f32,
    pub viewport_width: f32,
    pub viewport_height: f32,
}

impl CameraView {
    /// Convert original DAT coordinates into this aperture's screen space.
    #[must_use]
    pub fn to_screen(self, dat_x: f32, dat_y: f32) -> (f32, f32) {
        let sx = (dat_x - self.cam_x) * self.zoom + self.viewport_x + self.viewport_width / 2.0;
        let sy = (dat_y - self.cam_y) * self.zoom + self.viewport_y + self.viewport_height / 2.0;
        (sx, sy)
    }

    /// Whether a screen-space point is inside the recovered map aperture.
    #[must_use]
    pub fn contains(self, x: f32, y: f32) -> bool {
        x >= self.viewport_x
            && x < self.viewport_x + self.viewport_width
            && y >= self.viewport_y
            && y < self.viewport_y + self.viewport_height
    }

    /// Whether a point lies in or just outside the aperture for draw culling.
    #[must_use]
    pub fn contains_with_margin(self, x: f32, y: f32, margin: f32) -> bool {
        x >= self.viewport_x - margin
            && x <= self.viewport_x + self.viewport_width + margin
            && y >= self.viewport_y - margin
            && y <= self.viewport_y + self.viewport_height + margin
    }

    /// Convert a fixed original-interface pixel measurement to screen pixels.
    #[must_use]
    pub fn scale_pixels(self, logical_pixels: f32) -> f32 {
        logical_pixels * self.display_scale
    }

    /// Apply gameplay zoom and its original clamp before display scaling.
    #[must_use]
    pub fn zoomed_pixels(self, base: f32, min: f32, max: f32) -> f32 {
        (base * self.logical_zoom).clamp(min, max) * self.display_scale
    }
}

/// All mutable UI state for the galaxy map view.
pub struct GalaxyMapState {
    pub camera_x: f32,
    pub camera_y: f32,
    pub zoom: f32,
    pub selected_system: Option<SystemKey>,
    pub hovered_system: Option<SystemKey>,
    /// System activated by a primary press in the current frame.
    /// Consumers clear this by drawing the next map frame.
    pub activated_system: Option<SystemKey>,
    /// True while a modeless original-interface window owns the pointer.
    pub pointer_blocked: bool,
    pub show_sector_labels: bool,
    pub show_grid: bool,
    /// Previous mouse position used for right-drag panning.
    /// macroquad 0.4 has no `mouse_delta_position()`; we track it manually.
    pub drag_start: Option<(f32, f32)>,
    /// System context menu: system key + screen position of right-click.
    pub context_menu_system: Option<(SystemKey, f32, f32)>,
    /// Fleet context menu: fleet key + screen position of right-click.
    pub context_menu_fleet: Option<(FleetKey, f32, f32)>,
    /// Tracks whether right-mouse dragged (to distinguish click from pan).
    pub right_click_start: Option<(f32, f32)>,
    /// Cockpit viewport bounds for mouse input clamping.
    /// If set, mouse input outside this rect is ignored.
    pub viewport: Option<(f32, f32, f32, f32)>,
    /// Uniform scale of the original 640×480 strategic canvas.
    pub display_scale: f32,
    /// Frame counter incremented while right-mouse is held.
    /// Used as a WASM fallback: browsers swallow the mouseup on right-click
    /// (context menu intercepts it), so `is_mouse_button_released` never fires.
    /// When the button goes from held → not-held without a released event, we
    /// detect it on the next frame via this counter (> 0 but button not down).
    pub right_click_held_frames: u32,
}

/// Simulation views needed to reproduce the original GID filters.
///
/// The renderer reads these canonical systems directly. Deterministic browser
/// fixtures therefore exercise the same resource cache, metrics, marker
/// selection, and drawing path as a production campaign.
pub struct GidOverlayContext<'a> {
    pub movement: &'a MovementState,
    pub manufacturing: &'a ManufacturingState,
    pub economy: &'a EconomyState,
    pub missions: &'a MissionState,
}

impl Default for GalaxyMapState {
    fn default() -> Self {
        Self {
            camera_x: 450.0,
            camera_y: 470.0,
            zoom: 1.0,
            selected_system: None,
            hovered_system: None,
            activated_system: None,
            pointer_blocked: false,
            show_sector_labels: true,
            show_grid: false,
            drag_start: None,
            context_menu_system: None,
            context_menu_fleet: None,
            right_click_start: None,
            viewport: None,
            display_scale: 1.0,
            right_click_held_frames: 0,
        }
    }
}

/// Paint the original bright STRATEGY galaxy at the strategic canvas origin.
///
/// The 607x437 bitmap is not fitted to either faction aperture. The original
/// command-center shells reveal different source-aligned crops through their
/// transparent openings. A dark fill remains underneath as a fail-closed
/// fallback when the original resource is unavailable.
pub fn draw_galaxy_backdrop(
    layout: CockpitLayout,
    cache: &mut BmpCache,
    gid_mode: GidMode,
) -> bool {
    let viewport = layout.galaxy;
    draw_rectangle(
        viewport.x,
        viewport.y,
        viewport.width,
        viewport.height,
        Color::new(0.02, 0.02, 0.08, 1.0),
    );

    let Some(texture) =
        cache.get_macroquad_original(DllSource::Strategy, gid_backdrop_resource(gid_mode))
    else {
        return false;
    };
    let destination = galaxy_backdrop_destination(layout, texture.width(), texture.height());
    draw_texture_ex(
        texture,
        destination.x,
        destination.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(destination.width, destination.height)),
            ..Default::default()
        },
    );
    true
}

fn gid_backdrop_resource(gid_mode: GidMode) -> u32 {
    if gid_mode.is_active() {
        bmp_cache::resources::strategy::GALAXY_STARFIELD_DIM
    } else {
        bmp_cache::resources::strategy::GALAXY_STARFIELD_BRIGHT
    }
}

fn galaxy_backdrop_destination(
    layout: CockpitLayout,
    source_width: f32,
    source_height: f32,
) -> CockpitViewport {
    CockpitViewport {
        x: layout.canvas.x,
        y: layout.canvas.y,
        width: source_width * layout.scale,
        height: source_height * layout.scale,
    }
}

/// Render the galaxy star map for one frame.
///
/// Handles all input and draws the recovered Popular Support GID baseline.
/// Returns a `CameraView` so later authentic overlays can share its transform.
pub fn draw_galaxy_map(
    world: &GameWorld,
    state: &mut GalaxyMapState,
    cache: &mut BmpCache,
    gid_mode: GidMode,
    faction: CockpitFaction,
    gid: &GidOverlayContext<'_>,
) -> CameraView {
    discard_stale_context_menus(world, state);
    state.activated_system = None;

    let sw = screen_width();
    let sh = screen_height();
    let (viewport_x, viewport_y, viewport_width, viewport_height) =
        state.viewport.unwrap_or((0.0, 0.0, sw, sh));
    let (mx, my) = mouse_position();
    let in_viewport = !state.pointer_blocked
        && mx >= viewport_x
        && mx < viewport_x + viewport_width
        && my >= viewport_y
        && my < viewport_y + viewport_height;

    // ── Input: only when the cursor is in the map area ───────────────────────
    if in_viewport {
        // Zoom with scroll wheel (vertical component).
        let wheel_y = mouse_wheel().1;
        if wheel_y != 0.0 {
            let factor = if wheel_y > 0.0 { 1.1_f32 } else { 1.0 / 1.1 };
            state.zoom = (state.zoom * factor).clamp(0.3, 5.0);
        }

        // Pan with right-mouse drag.
        // We store the position from last frame and compute the delta ourselves.
        if is_mouse_button_pressed(MouseButton::Right) {
            state.right_click_start = Some((mx, my));
            state.right_click_held_frames = 0;
        }
        if is_mouse_button_down(MouseButton::Right) {
            state.right_click_held_frames += 1;
            if let Some((px, py)) = state.drag_start {
                let dx = mx - px;
                let dy = my - py;
                let effective_zoom = state.zoom * state.display_scale;
                state.camera_x -= dx / effective_zoom;
                state.camera_y -= dy / effective_zoom;
            }
            state.drag_start = Some((mx, my));
        } else {
            state.drag_start = None;
        }
    } else {
        // Cursor moved into the egui panel — release drag.
        state.drag_start = None;
    }

    // Build the shared transform after input so wheel zoom and drag apply in
    // the frame where they occur.
    let cam = CameraView {
        cam_x: state.camera_x,
        cam_y: state.camera_y,
        zoom: state.zoom * state.display_scale,
        logical_zoom: state.zoom,
        display_scale: state.display_scale,
        viewport_x,
        viewport_y,
        viewport_width,
        viewport_height,
    };

    if gid_mode.is_active() {
        draw_gid_caption(cam, faction, gid_mode);
    }

    // ── Find hovered system (reset each frame) ────────────────────────────────
    state.hovered_system = None;
    let hover_radius = cam.scale_pixels(8.5);

    // ── Draw systems ──────────────────────────────────────────────────────────
    // Detect the nearest system for input without adding replacement hover art.
    if in_viewport {
        let mut best_dist = hover_radius;
        for (key, system) in &world.systems {
            let (sx, sy) = cam.to_screen(f32::from(system.x), f32::from(system.y));
            if !cam.contains_with_margin(sx, sy, cam.scale_pixels(20.0)) {
                continue;
            }
            let dist = ((mx - sx).powi(2) + (my - sy).powi(2)).sqrt();
            if dist < best_dist {
                best_dist = dist;
                state.hovered_system = Some(key);
            }
        }
    }

    // Draw the native fixed-size marker bitmaps. Selection and hover
    // remain input states, but the replacement circles, glows, and labels are
    // deliberately absent from the parity surface.
    if gid_mode.is_active() {
        for (key, system) in &world.systems {
            let (sx, sy) = cam.to_screen(f32::from(system.x), f32::from(system.y));
            if !cam.contains_with_margin(sx, sy, cam.scale_pixels(20.0)) {
                continue;
            }
            draw_gid_marker(
                cache, world, key, system, faction, gid_mode, gid, cam, sx, sy,
            );
        }
    }

    // ── Click to select ───────────────────────────────────────────────────────
    if is_mouse_button_pressed(MouseButton::Left)
        && in_viewport
        && !context_menu_owns_pointer(state)
    {
        state.selected_system = state.hovered_system;
        state.activated_system = state.hovered_system;
    }

    // End right-button capture after panning. The parity path intentionally
    // does not create the replacement system or fleet context menus.
    let right_released = is_mouse_button_released(MouseButton::Right);
    // Browsers can swallow the release event after a context-menu gesture.
    let right_released_wasm = !right_released
        && state.right_click_held_frames >= 1
        && !is_mouse_button_down(MouseButton::Right)
        && state.right_click_start.is_some();
    if right_released || right_released_wasm {
        state.right_click_held_frames = 0;
        state.right_click_start = None;
    }

    cam
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "Preserve existing font-size rounding and narrowing for rendering."
)]
fn draw_gid_caption(cam: CameraView, faction: CockpitFaction, mode: GidMode) {
    let caption = mode.label();
    let font_size = cam.scale_pixels(11.0);
    let text_width = measure_text(caption, None, font_size.round() as u16, 1.0).width;
    let color = match faction {
        CockpitFaction::Alliance => Color::new(0.78, 0.16, 0.16, 1.0),
        CockpitFaction::Empire => Color::new(0.16, 0.65, 0.20, 1.0),
    };
    draw_text(
        caption,
        cam.viewport_x + (cam.viewport_width - text_width) / 2.0,
        cam.viewport_y + cam.scale_pixels(13.0),
        font_size,
        color,
    );
}

#[expect(
    clippy::too_many_arguments,
    reason = "Keep explicit state and rendering inputs at this existing UI boundary."
)]
fn draw_gid_marker(
    cache: &mut BmpCache,
    world: &GameWorld,
    system_key: SystemKey,
    system: &System,
    faction: CockpitFaction,
    mode: GidMode,
    gid: &GidOverlayContext<'_>,
    cam: CameraView,
    center_x: f32,
    center_y: f32,
) {
    let resource_id = gid_marker_for_system(world, system_key, system, faction, mode, gid);
    let Some(texture) = cache.get_macroquad_original(DllSource::Strategy, resource_id) else {
        return;
    };
    let width = texture.width() * cam.display_scale;
    let height = texture.height() * cam.display_scale;
    draw_texture_ex(
        texture,
        center_x - width / 2.0,
        center_y - height / 2.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(width, height)),
            ..Default::default()
        },
    );
}

fn popular_support_marker_resource(system: &System, faction: CockpitFaction) -> u32 {
    let explored = system.exploration_status == ExplorationStatus::Explored && system.is_populated;
    let popularity = match faction {
        CockpitFaction::Alliance => system.popularity_alliance,
        CockpitFaction::Empire => system.popularity_empire,
    };
    gid_marker_resource(system.control, explored, popularity)
}

fn gid_marker_for_system(
    world: &GameWorld,
    system_key: SystemKey,
    system: &System,
    faction: CockpitFaction,
    mode: GidMode,
    gid: &GidOverlayContext<'_>,
) -> u32 {
    if mode == GidMode::PopularSupport {
        return popular_support_marker_resource(system, faction);
    }
    if system.exploration_status != ExplorationStatus::Explored || !system.is_populated {
        return bmp_cache::resources::strategy::GID_UNEXPLORED;
    }

    let player_is_alliance = faction == CockpitFaction::Alliance;
    let value = gid_metric(world, system_key, system, player_is_alliance, mode, gid);
    let size = gid_metric_size(mode, value);
    gid_marker_resource_for_size(system.control, true, size)
}

#[expect(
    clippy::too_many_lines,
    clippy::cast_possible_truncation,
    reason = "Preserve upstream metric count conversion and keep the GID mode table together."
)]
fn gid_metric(
    world: &GameWorld,
    system_key: SystemKey,
    system: &System,
    player_is_alliance: bool,
    mode: GidMode,
    gid: &GidOverlayContext<'_>,
) -> u32 {
    let same_faction = |is_alliance: bool| is_alliance == player_is_alliance;
    let queue_idle = gid
        .manufacturing
        .queue(system_key)
        .is_none_or(rebellion_core::manufacturing::ProductionQueue::is_empty);
    match mode {
        GidMode::PopularSupport | GidMode::DisplayOff => 0,
        GidMode::Uprisings => u32::from(matches!(system.control, ControlKind::Uprising(_))),
        GidMode::IdleFleets => system
            .fleets
            .iter()
            .filter(|key| {
                world
                    .fleets
                    .get(**key)
                    .is_some_and(|fleet| same_faction(fleet.is_alliance))
            })
            .count() as u32,
        GidMode::FleetsEnRoute => gid
            .movement
            .orders()
            .iter()
            .filter(|(key, order)| {
                order.destination == system_key
                    && world
                        .fleets
                        .get(**key)
                        .is_some_and(|fleet| same_faction(fleet.is_alliance))
            })
            .count() as u32,
        GidMode::IdlePersonnel | GidMode::ActivePersonnel => world
            .characters
            .iter()
            .filter(|(character_key, character)| {
                let belongs = if player_is_alliance {
                    character.is_alliance
                } else {
                    character.is_empire
                };
                let at_system = character.current_system == Some(system_key)
                    || character.current_fleet.is_some_and(|fleet_key| {
                        world.fleets.get(fleet_key).is_some_and(|fleet| {
                            fleet.location == system_key && !gid.movement.is_in_transit(fleet_key)
                        })
                    });
                let active = character.on_mission
                    || gid
                        .missions
                        .missions()
                        .iter()
                        .any(|mission| mission.character == *character_key);
                belongs
                    && at_system
                    && if mode == GidMode::ActivePersonnel {
                        active
                    } else {
                        !active
                    }
            })
            .count() as u32,
        GidMode::AvailableEnergy => {
            let used = gid
                .economy
                .per_system
                .get(&system_key)
                .map_or(0, |economy| economy.energy_allocated);
            u32::from(system.total_energy).saturating_sub(used)
        }
        GidMode::AvailableRawMaterial => {
            let used = gid
                .economy
                .per_system
                .get(&system_key)
                .map_or(0, |economy| economy.raw_material_allocated);
            u32::from(system.raw_materials).saturating_sub(used)
        }
        GidMode::Mines | GidMode::Refineries => system
            .production_facilities
            .iter()
            .filter(|key| {
                world
                    .production_facilities
                    .get(**key)
                    .is_some_and(|facility| {
                        same_faction(facility.is_alliance)
                            && (facility.is_mine == (mode == GidMode::Mines))
                    })
            })
            .count() as u32,
        GidMode::Shipyards
        | GidMode::IdleShipyards
        | GidMode::TrainingFacilities
        | GidMode::IdleTrainingFacilities
        | GidMode::ConstructionYards
        | GidMode::IdleConstructionYards => system
            .manufacturing_facilities
            .iter()
            .filter(|key| {
                world
                    .manufacturing_facilities
                    .get(**key)
                    .is_some_and(|facility| {
                        let family = facility.class_dat_id.family();
                        let correct_type = match mode {
                            GidMode::Shipyards | GidMode::IdleShipyards => {
                                facility.is_shipyard || family == 0x28
                            }
                            GidMode::TrainingFacilities | GidMode::IdleTrainingFacilities => {
                                family == 0x29
                            }
                            GidMode::ConstructionYards | GidMode::IdleConstructionYards => {
                                matches!(family, 0x20 | 0x2a)
                            }
                            _ => false,
                        };
                        let idle_mode = matches!(
                            mode,
                            GidMode::IdleShipyards
                                | GidMode::IdleTrainingFacilities
                                | GidMode::IdleConstructionYards
                        );
                        same_faction(facility.is_alliance)
                            && correct_type
                            && (!idle_mode || queue_idle)
                    })
            })
            .count() as u32,
        GidMode::Troopers => system
            .ground_units
            .iter()
            .filter(|key| {
                world
                    .troops
                    .get(**key)
                    .is_some_and(|troop| same_faction(troop.is_alliance))
            })
            .count() as u32,
        GidMode::FighterSquadrons => system
            .fleets
            .iter()
            .filter_map(|key| world.fleets.get(*key))
            .filter(|fleet| same_faction(fleet.is_alliance))
            .flat_map(|fleet| fleet.fighters.iter())
            .map(|fighters| fighters.count)
            .sum(),
        GidMode::DeathStarShields => system
            .defense_facilities
            .iter()
            .filter(|key| {
                world.defense_facilities.get(**key).is_some_and(|facility| {
                    same_faction(facility.is_alliance) && facility.class_dat_id.index() == 4
                })
            })
            .count() as u32,
        GidMode::PlanetaryShieldGenerators => system
            .defense_facilities
            .iter()
            .filter(|key| {
                world.defense_facilities.get(**key).is_some_and(|facility| {
                    same_faction(facility.is_alliance)
                        && matches!(facility.class_dat_id.index(), 3 | 6)
                })
            })
            .count() as u32,
        GidMode::PlanetaryDefenseBatteries => system
            .defense_facilities
            .iter()
            .filter(|key| {
                world.defense_facilities.get(**key).is_some_and(|facility| {
                    same_faction(facility.is_alliance)
                        && matches!(facility.class_dat_id.index(), 1 | 2 | 5)
                })
            })
            .count() as u32,
    }
}

fn gid_metric_size(mode: GidMode, value: u32) -> usize {
    match mode {
        GidMode::Uprisings | GidMode::DeathStarShields => usize::from(value > 0) * 3,
        GidMode::IdleFleets | GidMode::FleetsEnRoute => match value {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => 3,
        },
        GidMode::Shipyards
        | GidMode::IdleShipyards
        | GidMode::TrainingFacilities
        | GidMode::IdleTrainingFacilities
        | GidMode::ConstructionYards
        | GidMode::IdleConstructionYards => match value {
            0 => 0,
            1 => 1,
            2..=4 => 2,
            _ => 3,
        },
        _ => match value {
            0 => 0,
            1..=2 => 1,
            3..=5 => 2,
            _ => 3,
        },
    }
}

fn gid_marker_resource_for_size(control: ControlKind, explored: bool, size: usize) -> u32 {
    use bmp_cache::resources::strategy;

    if !explored {
        return strategy::GID_UNEXPLORED;
    }
    let size = size.min(3);
    match control.faction() {
        Some(rebellion_core::dat::Faction::Alliance) => [
            strategy::GID_ALLIANCE_SMALLEST,
            strategy::GID_ALLIANCE_MEDIUM,
            strategy::GID_ALLIANCE_LARGE,
            strategy::GID_ALLIANCE_LARGEST,
        ][size],
        Some(rebellion_core::dat::Faction::Empire) => [
            strategy::GID_EMPIRE_SMALLEST,
            strategy::GID_EMPIRE_MEDIUM,
            strategy::GID_EMPIRE_LARGE,
            strategy::GID_EMPIRE_LARGEST,
        ][size],
        Some(rebellion_core::dat::Faction::Neutral) | None => [
            strategy::GID_NEUTRAL_SMALLEST,
            strategy::GID_NEUTRAL_MEDIUM,
            strategy::GID_NEUTRAL_LARGE,
            strategy::GID_NEUTRAL_LARGEST,
        ][size],
    }
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "Popularity is clamped to 0..=1 and rounded to a 0..=100 marker percentage."
)]
fn gid_marker_resource(control: ControlKind, explored: bool, popularity: f32) -> u32 {
    use bmp_cache::resources::strategy;

    if !explored {
        return strategy::GID_UNEXPLORED;
    }

    let support = (popularity.clamp(0.0, 1.0) * 100.0).round() as u8;
    let size = match support {
        81..=u8::MAX => 3,
        60..=80 => 2,
        50..=59 => 1,
        _ => 0,
    };
    gid_marker_resource_for_size(control, explored, size)
}

/// An open egui context menu receives the click before the map may select a
/// system. Closing happens through the menu action or its explicit Close button.
fn context_menu_owns_pointer(state: &GalaxyMapState) -> bool {
    state.context_menu_system.is_some() || state.context_menu_fleet.is_some()
}

/// Remove menu targets that disappeared after an arrival merge or world update.
/// A stale target has no visible window and must not continue consuming map clicks.
fn discard_stale_context_menus(world: &GameWorld, state: &mut GalaxyMapState) {
    if state
        .context_menu_system
        .is_some_and(|(system, _, _)| world.systems.get(system).is_none())
    {
        state.context_menu_system = None;
    }
    if state
        .context_menu_fleet
        .is_some_and(|(fleet, _, _)| world.fleets.get(fleet).is_none())
    {
        state.context_menu_fleet = None;
    }
}

// ---------------------------------------------------------------------------
// Galaxy map overlays (macroquad, called after draw_galaxy_map)
// ---------------------------------------------------------------------------

/// Coordinate transform: game-space → screen-space.
///
/// Mirrors the closure inside `draw_galaxy_map` so overlays align.
#[inline]
fn map_to_screen(dat_x: f32, dat_y: f32, cam: &CameraView) -> (f32, f32) {
    cam.to_screen(dat_x, dat_y)
}

/// True if the screen-space point is inside the visible map area.
#[inline]
fn in_map_viewport(sx: f32, sy: f32, cam: &CameraView) -> bool {
    cam.contains_with_margin(sx, sy, cam.scale_pixels(30.0))
}

/// Draw small facility indicator squares next to systems that have facilities.
///
/// Three icon types are shown in a horizontal row offset to the upper-right of
/// the system dot, each as a 5×5 pixel square scaled by zoom:
/// - Yellow  = production facilities (mines / refineries)
/// - Cyan    = manufacturing facilities (shipyards / training)
/// - Orange  = defense facilities
///
/// Icons are only drawn when `zoom >= 0.5` to avoid clutter at high zoom-out.
/// Scale with zoom up to a cap so they stay readable without dominating.
///
/// Pass the `CameraView` returned by `draw_galaxy_map`.
#[expect(
    clippy::cast_precision_loss,
    reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
)]
pub fn draw_facility_icons(world: &GameWorld, cam: &CameraView) {
    if cam.logical_zoom < 0.5 {
        return;
    }

    let icon_size = cam.zoomed_pixels(4.0, 2.5, 8.0);
    let gap = icon_size + cam.scale_pixels(1.5);

    for (_key, system) in &world.systems {
        let has_prod = !system.production_facilities.is_empty();
        let has_mfg = !system.manufacturing_facilities.is_empty();
        let has_def = !system.defense_facilities.is_empty();

        if !has_prod && !has_mfg && !has_def {
            continue;
        }

        let (sx, sy) = map_to_screen(f32::from(system.x), f32::from(system.y), cam);
        if !in_map_viewport(sx, sy, cam) {
            continue;
        }

        // Offset cluster to upper-right of the system dot so it doesn't
        // overlap the name label (which appears to the right) or the fleet
        // diamond (which appears directly above).
        let base_x = sx + 6.0 * cam.zoom;
        let base_y = sy - 8.0 * cam.zoom - icon_size;

        let mut col = 0;

        if has_prod {
            // Yellow: resource production
            let ix = base_x + col as f32 * gap;
            draw_rectangle(
                ix,
                base_y,
                icon_size,
                icon_size,
                Color::new(0.9, 0.8, 0.1, 0.85),
            );
            col += 1;
        }
        if has_mfg {
            // Cyan: manufacturing (shipyards)
            let ix = base_x + col as f32 * gap;
            draw_rectangle(
                ix,
                base_y,
                icon_size,
                icon_size,
                Color::new(0.2, 0.8, 0.9, 0.85),
            );
            col += 1;
        }
        if has_def {
            // Orange: defense installations
            let ix = base_x + col as f32 * gap;
            draw_rectangle(
                ix,
                base_y,
                icon_size,
                icon_size,
                Color::new(0.9, 0.5, 0.1, 0.85),
            );
        }
    }
}

/// Compute a convex hull (Graham scan) of a set of 2D screen-space points.
///
/// Returns the points in counter-clockwise order.  Returns an empty Vec if
/// fewer than 3 points are provided.
fn convex_hull(mut pts: Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    if pts.len() < 3 {
        return pts;
    }

    // Sort by x then y.
    pts.sort_by(|a, b| {
        a.0.partial_cmp(&b.0)
            .unwrap()
            .then(a.1.partial_cmp(&b.1).unwrap())
    });
    pts.dedup_by(|a, b| (a.0 - b.0).abs() < 0.5 && (a.1 - b.1).abs() < 0.5);

    if pts.len() < 3 {
        return pts;
    }

    let cross = |o: (f32, f32), a: (f32, f32), b: (f32, f32)| -> f32 {
        (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
    };

    let n = pts.len();
    let mut hull: Vec<(f32, f32)> = Vec::with_capacity(2 * n);

    // Build lower hull.
    for &p in &pts {
        while hull.len() >= 2 && cross(hull[hull.len() - 2], hull[hull.len() - 1], p) <= 0.0 {
            hull.pop();
        }
        hull.push(p);
    }

    // Build upper hull.
    let lower_len = hull.len() + 1;
    for &p in pts.iter().rev() {
        while hull.len() >= lower_len && cross(hull[hull.len() - 2], hull[hull.len() - 1], p) <= 0.0
        {
            hull.pop();
        }
        hull.push(p);
    }

    hull.pop(); // Remove the last point (same as first).
    hull
}

/// Dim sector-specific color from the `SectorGroup`.
fn sector_boundary_color(group: rebellion_core::dat::SectorGroup) -> Color {
    match group {
        rebellion_core::dat::SectorGroup::Core => Color::new(0.7, 0.6, 0.2, 0.25), // warm gold — galactic core
        rebellion_core::dat::SectorGroup::RimInner => Color::new(0.3, 0.5, 0.7, 0.22), // cool blue — inner rim
        rebellion_core::dat::SectorGroup::RimOuter => Color::new(0.4, 0.3, 0.6, 0.20), // purple-grey — outer rim
    }
}

/// Draw translucent polygon outlines for each sector region.
///
/// Computes the convex hull of member systems' screen positions, expands it
/// outward by `padding` pixels, then draws the hull edges with `draw_line()`.
///
/// Toggle controlled via `GalaxyMapState::show_sector_labels` — if sector
/// labels are hidden, boundaries are also hidden.
///
/// Pass the `CameraView` returned by `draw_galaxy_map`.
#[expect(
    clippy::cast_precision_loss,
    reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
)]
pub fn draw_sector_boundaries(world: &GameWorld, cam: &CameraView, show: bool) {
    if !show {
        return;
    }

    let padding = 12.0 * cam.logical_zoom.clamp(0.5, 2.0) * cam.display_scale;

    for (_sec_key, sector) in &world.sectors {
        if sector.systems.len() < 2 {
            continue;
        }

        // Collect screen-space positions of all systems in this sector.
        let pts: Vec<(f32, f32)> = sector
            .systems
            .iter()
            .filter_map(|&sk| world.systems.get(sk))
            .map(|sys| map_to_screen(f32::from(sys.x), f32::from(sys.y), cam))
            .filter(|&(sx, sy)| in_map_viewport(sx, sy, cam))
            .collect();

        if pts.len() < 2 {
            continue;
        }

        let hull = if pts.len() == 2 {
            // Two-point degenerate case: just draw the line.
            pts.clone()
        } else {
            convex_hull(pts.clone())
        };

        if hull.len() < 2 {
            continue;
        }

        let color = sector_boundary_color(sector.group);

        // Expand each hull vertex outward from the centroid by `padding`.
        let cx = hull.iter().map(|p| p.0).sum::<f32>() / hull.len() as f32;
        let cy = hull.iter().map(|p| p.1).sum::<f32>() / hull.len() as f32;

        let expanded: Vec<(f32, f32)> = hull
            .iter()
            .map(|&(px, py)| {
                let dx = px - cx;
                let dy = py - cy;
                let len = (dx * dx + dy * dy).sqrt().max(0.001);
                (px + dx / len * padding, py + dy / len * padding)
            })
            .collect();

        // Draw edges.
        let n = expanded.len();
        for i in 0..n {
            let (x1, y1) = expanded[i];
            let (x2, y2) = expanded[(i + 1) % n];
            draw_line(x1, y1, x2, y2, cam.scale_pixels(1.0), color);
        }
    }
}

/// Draw a blockade indicator (pulsing red ring) around each blockaded system.
///
/// Renders a solid outer ring in danger-red at low alpha around the system dot
/// to visually flag active blockades without obscuring the system color.
///
/// Pass the `CameraView` returned by `draw_galaxy_map` and the current
/// `BlockadeState` from the simulation.
pub fn draw_blockade_indicators(world: &GameWorld, blockade: &BlockadeState, cam: &CameraView) {
    let blockaded = blockade.blockaded_systems();
    if blockaded.is_empty() {
        return;
    }

    let ring_radius = cam.zoomed_pixels(7.0, 5.0, 18.0);
    let ring_width = cam.scale_pixels(1.5);
    let ring_color = Color::new(0.9, 0.15, 0.15, 0.7);

    for &sys_key in blockaded {
        let Some(system) = world.systems.get(sys_key) else {
            continue;
        };

        let (sx, sy) = map_to_screen(f32::from(system.x), f32::from(system.y), cam);
        if !in_map_viewport(sx, sy, cam) {
            continue;
        }

        // Outer ring — solid dim red.
        draw_circle_lines(sx, sy, ring_radius, ring_width, ring_color);
        // Inner fill — very faint red tint over the system dot.
        draw_circle(
            sx,
            sy,
            ring_radius - ring_width,
            Color::new(0.9, 0.1, 0.1, 0.08),
        );
    }
}

/// Draw the system info side panel (right side).
///
/// Shows details for the currently selected system. Call inside
/// `egui_macroquad::ui(|ctx| { ... })`.
#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
pub fn draw_system_info_panel(ctx: &egui::Context, world: &GameWorld, state: &GalaxyMapState) {
    if let Some(sys_key) = state.selected_system {
        if let Some(system) = world.systems.get(sys_key) {
            egui::SidePanel::right("system_info")
                .min_width(280.0)
                .max_width(320.0)
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        // ── Header ───────────────────────────────────────────
                        let name_color = match system.control {
                            rebellion_core::world::ControlKind::Controlled(
                                rebellion_core::dat::Faction::Alliance,
                            ) => theme::ALLIANCE_BLUE,
                            rebellion_core::world::ControlKind::Controlled(
                                rebellion_core::dat::Faction::Empire,
                            ) => theme::EMPIRE_RED,
                            _ => theme::TEXT_PRIMARY,
                        };
                        ui.heading(egui::RichText::new(&system.name).color(name_color));

                        if system.is_headquarters {
                            ui.label(
                                egui::RichText::new("HEADQUARTERS")
                                    .color(theme::GOLD)
                                    .size(10.0)
                                    .strong(),
                            );
                        }
                        if system.is_destroyed {
                            ui.label(
                                egui::RichText::new("DESTROYED")
                                    .color(theme::DANGER_RED)
                                    .size(10.0)
                                    .strong(),
                            );
                        }

                        // Sector + region
                        if let Some(sector) = world.sectors.get(system.sector) {
                            let region = match sector.group {
                                rebellion_core::dat::SectorGroup::Core => "Core",
                                rebellion_core::dat::SectorGroup::RimInner => "Inner Rim",
                                rebellion_core::dat::SectorGroup::RimOuter => "Outer Rim",
                            };
                            ui.label(
                                egui::RichText::new(format!("{} — {}", sector.name, region))
                                    .color(theme::TEXT_SECONDARY)
                                    .size(11.0),
                            );
                        }

                        ui.separator();

                        // ── Popularity bars ──────────────────────────────────
                        ui.label(
                            egui::RichText::new("SUPPORT")
                                .color(theme::GOLD_DIM)
                                .size(10.0)
                                .strong(),
                        );

                        let alliance_pct = system.popularity_alliance;
                        let empire_pct = system.popularity_empire;

                        // Alliance bar
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Alliance")
                                    .color(theme::ALLIANCE_BLUE)
                                    .size(11.0),
                            );
                            let bar = egui::ProgressBar::new(alliance_pct)
                                .text(format!("{:.0}%", alliance_pct * 100.0))
                                .fill(theme::ALLIANCE_BLUE);
                            ui.add(bar);
                        });

                        // Empire bar
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Empire  ")
                                    .color(theme::EMPIRE_RED)
                                    .size(11.0),
                            );
                            let bar = egui::ProgressBar::new(empire_pct)
                                .text(format!("{:.0}%", empire_pct * 100.0))
                                .fill(theme::EMPIRE_RED);
                            ui.add(bar);
                        });

                        // Control status
                        let control_str = match system.control {
                            rebellion_core::world::ControlKind::Controlled(
                                rebellion_core::dat::Faction::Alliance,
                            ) => "Alliance Controlled",
                            rebellion_core::world::ControlKind::Controlled(
                                rebellion_core::dat::Faction::Empire,
                            ) => "Empire Controlled",
                            rebellion_core::world::ControlKind::Uncontrolled
                            | rebellion_core::world::ControlKind::Controlled(
                                rebellion_core::dat::Faction::Neutral,
                            ) => "Neutral",
                            rebellion_core::world::ControlKind::Contested => "Contested",
                            rebellion_core::world::ControlKind::Uprising(_) => "Uprising",
                        };
                        ui.label(
                            egui::RichText::new(control_str)
                                .color(theme::TEXT_SECONDARY)
                                .size(10.0),
                        );

                        ui.separator();

                        // ── Fleets ───────────────────────────────────────────
                        if !system.fleets.is_empty() {
                            ui.label(
                                egui::RichText::new("FLEETS")
                                    .color(theme::GOLD_DIM)
                                    .size(10.0)
                                    .strong(),
                            );
                            for &fleet_key in &system.fleets {
                                if let Some(fleet) = world.fleets.get(fleet_key) {
                                    let faction_color = if fleet.is_alliance {
                                        theme::ALLIANCE_BLUE
                                    } else {
                                        theme::EMPIRE_RED
                                    };
                                    let faction_tag = if fleet.is_alliance { "A" } else { "E" };

                                    let ship_count: u32 = fleet.ship_count();
                                    let fighter_count: u32 =
                                        fleet.fighters.iter().map(|e| e.count).sum();

                                    ui.horizontal(|ui| {
                                        ui.label(
                                            egui::RichText::new(format!("[{faction_tag}]"))
                                                .color(faction_color)
                                                .size(11.0)
                                                .strong(),
                                        );

                                        let mut parts = Vec::new();
                                        if ship_count > 0 {
                                            parts.push(format!("{ship_count} ships"));
                                        }
                                        if fighter_count > 0 {
                                            parts.push(format!("{fighter_count} sqns"));
                                        }
                                        if fleet.has_death_star {
                                            parts.push("Death Star".to_string());
                                        }
                                        ui.label(
                                            egui::RichText::new(parts.join(", "))
                                                .color(theme::TEXT_PRIMARY)
                                                .size(11.0),
                                        );
                                    });

                                    // Ship class breakdown
                                    for (class_key, count) in fleet.ship_counts_by_class() {
                                        if let Some(class) =
                                            world.capital_ship_classes.get(class_key)
                                        {
                                            ui.label(
                                                egui::RichText::new(format!(
                                                    "  {} ×{}",
                                                    class.name, count
                                                ))
                                                .color(theme::TEXT_SECONDARY)
                                                .size(10.0),
                                            );
                                        }
                                    }

                                    // Commander
                                    for &char_key in &fleet.characters {
                                        if let Some(c) = world.characters.get(char_key) {
                                            ui.label(
                                                egui::RichText::new(format!("  Cmd: {}", c.name))
                                                    .color(theme::GOLD_DIM)
                                                    .size(10.0),
                                            );
                                        }
                                    }
                                }
                            }
                            ui.add_space(4.0);
                        }

                        // ── Ground Units ─────────────────────────────────────
                        if !system.ground_units.is_empty() {
                            let alliance_troops = system
                                .ground_units
                                .iter()
                                .filter(|k| world.troops.get(**k).is_some_and(|t| t.is_alliance))
                                .count();
                            let empire_troops = system.ground_units.len() - alliance_troops;

                            ui.label(
                                egui::RichText::new(format!(
                                    "GROUND FORCES ({})",
                                    system.ground_units.len()
                                ))
                                .color(theme::GOLD_DIM)
                                .size(10.0)
                                .strong(),
                            );
                            if alliance_troops > 0 {
                                ui.label(
                                    egui::RichText::new(format!(
                                        "  Alliance: {alliance_troops} regiments"
                                    ))
                                    .color(theme::ALLIANCE_BLUE)
                                    .size(10.0),
                                );
                            }
                            if empire_troops > 0 {
                                ui.label(
                                    egui::RichText::new(format!(
                                        "  Empire: {empire_troops} regiments"
                                    ))
                                    .color(theme::EMPIRE_RED)
                                    .size(10.0),
                                );
                            }
                            ui.add_space(4.0);
                        }

                        // ── Facilities ────────────────────────────────────────
                        let total_fac = system.defense_facilities.len()
                            + system.manufacturing_facilities.len()
                            + system.production_facilities.len();
                        if total_fac > 0 {
                            ui.label(
                                egui::RichText::new(format!("FACILITIES ({total_fac})"))
                                    .color(theme::GOLD_DIM)
                                    .size(10.0)
                                    .strong(),
                            );

                            if !system.defense_facilities.is_empty() {
                                ui.label(
                                    egui::RichText::new(format!(
                                        "  Defense: {}",
                                        system.defense_facilities.len()
                                    ))
                                    .color(theme::TEXT_SECONDARY)
                                    .size(10.0),
                                );
                            }
                            if !system.manufacturing_facilities.is_empty() {
                                ui.label(
                                    egui::RichText::new(format!(
                                        "  Shipyards: {}",
                                        system.manufacturing_facilities.len()
                                    ))
                                    .color(theme::TEXT_SECONDARY)
                                    .size(10.0),
                                );
                            }
                            if !system.production_facilities.is_empty() {
                                ui.label(
                                    egui::RichText::new(format!(
                                        "  Production: {}",
                                        system.production_facilities.len()
                                    ))
                                    .color(theme::TEXT_SECONDARY)
                                    .size(10.0),
                                );
                            }
                            ui.add_space(4.0);
                        }

                        // ── Coordinates (small, at bottom) ───────────────────
                        ui.separator();
                        ui.label(
                            egui::RichText::new(format!("({}, {})", system.x, system.y))
                                .color(theme::TEXT_DISABLED)
                                .size(9.0),
                        );
                    });
                });
        }
    }
}

// ---------------------------------------------------------------------------
// Context menus
// ---------------------------------------------------------------------------

/// Draw the system right-click context menu as a floating egui window.
///
/// Shows system summary (faction control, popularity, garrison) and quick
/// action buttons (Send Diplomat, Move Fleet Here, Build Facility).
/// Returns `Some(PanelAction)` when an action button is clicked.
#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
pub fn draw_system_context_menu(
    ctx: &egui::Context,
    world: &GameWorld,
    state: &mut GalaxyMapState,
    player_faction: MissionFaction,
) -> Option<PanelAction> {
    let (sys_key, screen_x, screen_y) = state.context_menu_system?;
    let system = world.systems.get(sys_key)?;

    let mut action = None;
    let mut keep_open = true;

    egui::Window::new("system_context")
        .title_bar(false)
        .resizable(false)
        .collapsible(false)
        .fixed_pos(egui::pos2(screen_x, screen_y))
        .min_width(200.0)
        .max_width(240.0)
        .show(ctx, |ui| {
            // ── Header ──────────────────────────────────────────────────
            let name_color = match system.control {
                rebellion_core::world::ControlKind::Controlled(
                    rebellion_core::dat::Faction::Alliance,
                ) => theme::ALLIANCE_BLUE,
                rebellion_core::world::ControlKind::Controlled(
                    rebellion_core::dat::Faction::Empire,
                ) => theme::EMPIRE_RED,
                _ => theme::TEXT_PRIMARY,
            };
            ui.label(
                egui::RichText::new(&system.name)
                    .color(name_color)
                    .strong()
                    .size(14.0),
            );

            // Control status
            let control_str = match system.control {
                rebellion_core::world::ControlKind::Controlled(
                    rebellion_core::dat::Faction::Alliance,
                ) => "Alliance",
                rebellion_core::world::ControlKind::Controlled(
                    rebellion_core::dat::Faction::Empire,
                ) => "Empire",
                rebellion_core::world::ControlKind::Uncontrolled
                | rebellion_core::world::ControlKind::Controlled(
                    rebellion_core::dat::Faction::Neutral,
                ) => "Neutral",
                rebellion_core::world::ControlKind::Contested => "Contested",
                rebellion_core::world::ControlKind::Uprising(_) => "Uprising",
            };
            ui.label(
                egui::RichText::new(control_str)
                    .color(theme::TEXT_SECONDARY)
                    .size(10.0),
            );

            ui.separator();

            // ── Popularity snapshot ──────────────────────────────────────
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("A: {:.0}%", system.popularity_alliance * 100.0))
                        .color(theme::ALLIANCE_BLUE)
                        .size(10.0),
                );
                ui.label(
                    egui::RichText::new(format!("E: {:.0}%", system.popularity_empire * 100.0))
                        .color(theme::EMPIRE_RED)
                        .size(10.0),
                );
            });

            // ── Garrison summary ────────────────────────────────────────
            let fleet_count = system.fleets.len();
            let troop_count = system.ground_units.len();
            let fac_count = system.defense_facilities.len()
                + system.manufacturing_facilities.len()
                + system.production_facilities.len();

            if fleet_count > 0 || troop_count > 0 || fac_count > 0 {
                ui.horizontal(|ui| {
                    if fleet_count > 0 {
                        ui.label(
                            egui::RichText::new(format!("{fleet_count} fleets"))
                                .color(theme::TEXT_SECONDARY)
                                .size(10.0),
                        );
                    }
                    if troop_count > 0 {
                        ui.label(
                            egui::RichText::new(format!("{troop_count} troops"))
                                .color(theme::TEXT_SECONDARY)
                                .size(10.0),
                        );
                    }
                    if fac_count > 0 {
                        ui.label(
                            egui::RichText::new(format!("{fac_count} facilities"))
                                .color(theme::TEXT_SECONDARY)
                                .size(10.0),
                        );
                    }
                });
            }

            ui.separator();

            // ── Quick actions ───────────────────────────────────────────
            if ui
                .button(
                    egui::RichText::new("View Details")
                        .color(theme::GOLD)
                        .size(11.0),
                )
                .clicked()
            {
                action = Some(PanelAction::FocusFleetSystem(sys_key));
                keep_open = false;
            }
            if ui
                .button(
                    egui::RichText::new("Send Diplomat")
                        .color(theme::TEXT_PRIMARY)
                        .size(11.0),
                )
                .clicked()
            {
                action = Some(PanelAction::OpenMissionTo {
                    target: sys_key,
                    kind: rebellion_core::missions::MissionKind::Diplomacy,
                    faction: player_faction,
                });
                keep_open = false;
            }
            if ui
                .button(
                    egui::RichText::new("Send Spy")
                        .color(theme::TEXT_PRIMARY)
                        .size(11.0),
                )
                .clicked()
            {
                action = Some(PanelAction::OpenMissionTo {
                    target: sys_key,
                    kind: rebellion_core::missions::MissionKind::Espionage,
                    faction: player_faction,
                });
                keep_open = false;
            }
            if ui
                .button(
                    egui::RichText::new("Move Fleet Here")
                        .color(theme::TEXT_PRIMARY)
                        .size(11.0),
                )
                .clicked()
            {
                action = Some(PanelAction::InitiateFleetMove {
                    destination: sys_key,
                });
                keep_open = false;
            }

            ui.add_space(2.0);
            if ui
                .small_button(
                    egui::RichText::new("Close")
                        .color(theme::TEXT_DISABLED)
                        .size(10.0),
                )
                .clicked()
            {
                keep_open = false;
            }
        });

    if !keep_open {
        state.context_menu_system = None;
    }

    action
}

/// Draw the fleet right-click context menu as a floating egui window.
///
/// Shows fleet composition, commander, faction, and quick actions
/// (Move, View in Fleet Panel). Returns `Some(PanelAction)` on action.
#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
pub fn draw_fleet_context_menu(
    ctx: &egui::Context,
    world: &GameWorld,
    movement_state: &MovementState,
    state: &mut GalaxyMapState,
) -> Option<PanelAction> {
    let (fleet_key, screen_x, screen_y) = state.context_menu_fleet?;
    let fleet = world.fleets.get(fleet_key)?;

    let mut action = None;
    let mut keep_open = true;

    egui::Window::new("fleet_context")
        .title_bar(false)
        .resizable(false)
        .collapsible(false)
        .fixed_pos(egui::pos2(screen_x, screen_y))
        .min_width(180.0)
        .max_width(220.0)
        .show(ctx, |ui| {
            // ── Header ──────────────────────────────────────────────────
            let faction_color = if fleet.is_alliance {
                theme::ALLIANCE_BLUE
            } else {
                theme::EMPIRE_RED
            };
            let faction_tag = if fleet.is_alliance {
                "Alliance"
            } else {
                "Empire"
            };
            ui.label(
                egui::RichText::new(format!("{faction_tag} Fleet"))
                    .color(faction_color)
                    .strong()
                    .size(13.0),
            );

            // Location
            if let Some(sys) = world.systems.get(fleet.location) {
                ui.label(
                    egui::RichText::new(format!("at {}", sys.name))
                        .color(theme::TEXT_SECONDARY)
                        .size(10.0),
                );
            }

            // Transit status
            if let Some(order) = movement_state.get(fleet_key) {
                if let Some(dest) = world.systems.get(order.destination) {
                    ui.label(
                        egui::RichText::new(format!(
                            "→ {} ({}d)",
                            dest.name,
                            order.ticks_remaining()
                        ))
                        .color(theme::WARNING_AMBER)
                        .size(10.0),
                    );
                }
            }

            ui.separator();

            // ── Composition ─────────────────────────────────────────────
            let ship_count: u32 = fleet.ship_count();
            let fighter_count: u32 = fleet.fighters.iter().map(|e| e.count).sum();

            if ship_count > 0 {
                ui.label(
                    egui::RichText::new(format!("{ship_count} capital ships"))
                        .color(theme::TEXT_PRIMARY)
                        .size(11.0),
                );
            }
            if fighter_count > 0 {
                ui.label(
                    egui::RichText::new(format!("{fighter_count} fighter sqns"))
                        .color(theme::TEXT_PRIMARY)
                        .size(11.0),
                );
            }
            if fleet.has_death_star {
                ui.label(
                    egui::RichText::new("DEATH STAR")
                        .color(theme::DANGER_RED)
                        .size(11.0)
                        .strong(),
                );
            }

            // Commander
            for &char_key in &fleet.characters {
                if let Some(c) = world.characters.get(char_key) {
                    ui.label(
                        egui::RichText::new(format!("Cmd: {}", c.name))
                            .color(theme::GOLD_DIM)
                            .size(10.0),
                    );
                }
            }

            ui.separator();

            // ── Quick actions ───────────────────────────────────────────
            if ui
                .button(
                    egui::RichText::new("View in Fleet Panel")
                        .color(theme::GOLD)
                        .size(11.0),
                )
                .clicked()
            {
                action = Some(PanelAction::FocusFleetSystem(fleet.location));
                keep_open = false;
            }

            // Transit status
            if movement_state.get(fleet_key).is_some() {
                ui.label(
                    egui::RichText::new("In transit")
                        .color(theme::WARNING_AMBER)
                        .size(10.0),
                );
            }

            ui.add_space(2.0);
            if ui
                .small_button(
                    egui::RichText::new("Close")
                        .color(theme::TEXT_DISABLED)
                        .size(10.0),
                )
                .clicked()
            {
                keep_open = false;
            }
        });

    if !keep_open {
        state.context_menu_fleet = None;
    }

    action
}

/// Draw the bottom status bar with speed controls, day counter, world stats,
/// and audio volume controls.
///
/// Call inside `egui_macroquad::ui(|ctx| { ... })`.
pub fn draw_status_bar(
    ctx: &egui::Context,
    world: &GameWorld,
    clock: &mut GameClock,
    audio_vol: &mut AudioVolumeState,
) {
    egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            // ── Speed controls ───────────────────────────────────────────
            if ui
                .selectable_label(clock.speed == GameSpeed::Paused, "⏸ Pause")
                .clicked()
            {
                clock.set_speed(GameSpeed::Paused);
            }
            if ui
                .selectable_label(clock.speed == GameSpeed::Normal, "▶ 1×")
                .clicked()
            {
                clock.set_speed(GameSpeed::Normal);
            }
            if ui
                .selectable_label(clock.speed == GameSpeed::Fast, "▶▶ 2×")
                .clicked()
            {
                clock.set_speed(GameSpeed::Fast);
            }
            if ui
                .selectable_label(clock.speed == GameSpeed::Faster, "▶▶▶ 4×")
                .clicked()
            {
                clock.set_speed(GameSpeed::Faster);
            }

            ui.separator();
            ui.label(format!("Day {}", clock.tick));
            ui.separator();

            ui.label(format!(
                "Systems: {} | Sectors: {} | Ships: {} | Fighters: {} | Characters: {}",
                world.systems.len(),
                world.sectors.len(),
                world.capital_ship_classes.len(),
                world.fighter_classes.len(),
                world.characters.len(),
            ));

            // ── Audio controls ───────────────────────────────────────────
            audio::draw_audio_controls(ui, audio_vol);
        });
    });
}

#[cfg(test)]
mod interaction_tests {
    use super::*;

    #[test]
    #[expect(
        clippy::float_cmp,
        reason = "These regression checks require exact copied values, endpoints, and pixel coordinates."
    )]
    fn galaxy_backdrop_uses_canvas_origin_and_native_resource_size() {
        let alliance = CockpitState::new(CockpitFaction::Alliance).layout_for(640.0, 480.0);
        let destination = galaxy_backdrop_destination(alliance, 607.0, 437.0);
        assert_eq!(destination.x, 0.0);
        assert_eq!(destination.y, 0.0);
        assert_eq!(destination.width, 607.0);
        assert_eq!(destination.height, 437.0);
        assert_eq!(alliance.galaxy.x, 55.0);
        assert_eq!(alliance.galaxy.y, 40.0);

        let empire = CockpitState::new(CockpitFaction::Empire).layout_for(1280.0, 960.0);
        let destination = galaxy_backdrop_destination(empire, 607.0, 437.0);
        assert_eq!(destination.x, 0.0);
        assert_eq!(destination.y, 0.0);
        assert_eq!(destination.width, 1214.0);
        assert_eq!(destination.height, 874.0);
        assert_eq!(empire.galaxy.x, 240.0);
        assert_eq!(empire.galaxy.y, 80.0);
    }

    #[test]
    fn active_gid_uses_dim_backdrop_and_display_off_uses_bright() {
        assert_eq!(
            gid_backdrop_resource(GidMode::PopularSupport),
            bmp_cache::resources::strategy::GALAXY_STARFIELD_DIM
        );
        assert_eq!(
            gid_backdrop_resource(GidMode::DisplayOff),
            bmp_cache::resources::strategy::GALAXY_STARFIELD_BRIGHT
        );
    }

    #[test]
    fn popular_support_thresholds_select_native_marker_sizes() {
        use bmp_cache::resources::strategy;
        use rebellion_core::dat::Faction;

        for (support, expected) in [
            (0.49, strategy::GID_ALLIANCE_SMALLEST),
            (0.50, strategy::GID_ALLIANCE_MEDIUM),
            (0.60, strategy::GID_ALLIANCE_LARGE),
            (0.81, strategy::GID_ALLIANCE_LARGEST),
        ] {
            assert_eq!(
                gid_marker_resource(ControlKind::Controlled(Faction::Alliance), true, support),
                expected
            );
        }
        assert_eq!(
            gid_marker_resource(ControlKind::Controlled(Faction::Empire), true, 0.81),
            strategy::GID_EMPIRE_LARGEST
        );
        assert_eq!(
            gid_marker_resource(ControlKind::Uncontrolled, true, 0.60),
            strategy::GID_NEUTRAL_LARGE
        );
        assert_eq!(
            gid_marker_resource(ControlKind::Controlled(Faction::Alliance), false, 1.0),
            strategy::GID_UNEXPLORED
        );
    }

    #[test]
    #[expect(
        clippy::float_cmp,
        reason = "These regression checks require exact copied values, endpoints, and pixel coordinates."
    )]
    fn camera_transform_includes_aperture_offset() {
        let camera = CameraView {
            cam_x: 450.0,
            cam_y: 470.0,
            zoom: 2.0,
            logical_zoom: 1.0,
            display_scale: 2.0,
            viewport_x: 100.0,
            viewport_y: 40.0,
            viewport_width: 480.0,
            viewport_height: 355.0,
        };

        assert_eq!(camera.to_screen(450.0, 470.0), (340.0, 217.5));
        assert!(camera.contains(100.0, 40.0));
        assert!(camera.contains(579.999, 394.999));
        assert!(!camera.contains(99.0, 40.0));
        assert!(!camera.contains(580.0, 394.0));
        assert!(!camera.contains(579.0, 395.0));
        assert!(camera.contains_with_margin(90.0, 30.0, 10.0));
        assert_eq!(camera.scale_pixels(8.0), 16.0);
        assert_eq!(camera.zoomed_pixels(4.0, 2.5, 8.0), 8.0);
    }

    #[test]
    #[expect(
        clippy::float_cmp,
        reason = "These regression checks require exact copied values, endpoints, and pixel coordinates."
    )]
    fn display_scale_does_not_change_logical_visibility_thresholds() {
        let baseline = CameraView {
            cam_x: 0.0,
            cam_y: 0.0,
            zoom: 0.7,
            logical_zoom: 0.7,
            display_scale: 1.0,
            viewport_x: 0.0,
            viewport_y: 0.0,
            viewport_width: 100.0,
            viewport_height: 100.0,
        };
        let widescreen = CameraView {
            zoom: 0.7 * 1.666_666_6,
            display_scale: 1.666_666_6,
            viewport_width: 166.666_66,
            viewport_height: 166.666_66,
            ..baseline
        };

        assert_eq!(baseline.logical_zoom, widescreen.logical_zoom);
        assert_eq!(baseline.logical_zoom > 0.8, widescreen.logical_zoom > 0.8);
        assert!(
            (widescreen.zoomed_pixels(4.0, 2.5, 8.0) / baseline.zoomed_pixels(4.0, 2.5, 8.0)
                - widescreen.display_scale)
                .abs()
                < 0.001
        );
    }

    #[test]
    fn open_context_menu_owns_map_left_click() {
        let mut state = GalaxyMapState::default();
        assert!(!context_menu_owns_pointer(&state));

        state.context_menu_system = Some((SystemKey::default(), 10.0, 20.0));
        assert!(context_menu_owns_pointer(&state));

        state.context_menu_system = None;
        state.context_menu_fleet = Some((FleetKey::default(), 30.0, 40.0));
        assert!(context_menu_owns_pointer(&state));
    }

    #[test]
    fn stale_context_menu_releases_map_left_click() {
        let world = GameWorld::default();
        let mut state = GalaxyMapState {
            context_menu_system: Some((SystemKey::default(), 10.0, 20.0)),
            context_menu_fleet: Some((FleetKey::default(), 30.0, 40.0)),
            ..GalaxyMapState::default()
        };

        discard_stale_context_menus(&world, &mut state);

        assert!(!context_menu_owns_pointer(&state));
    }

    #[test]
    fn gid_mode_marker_bands_cover_zero_threshold_and_high_values() {
        assert_eq!(gid_metric_size(GidMode::Uprisings, 0), 0);
        assert_eq!(gid_metric_size(GidMode::Uprisings, 1), 3);
        assert_eq!(gid_metric_size(GidMode::IdleFleets, 2), 2);
        assert_eq!(gid_metric_size(GidMode::IdleFleets, 3), 3);
        assert_eq!(gid_metric_size(GidMode::Shipyards, 4), 2);
        assert_eq!(gid_metric_size(GidMode::Shipyards, 5), 3);
        assert_eq!(gid_metric_size(GidMode::AvailableEnergy, 5), 2);
        assert_eq!(gid_metric_size(GidMode::AvailableEnergy, 6), 3);
    }
}
