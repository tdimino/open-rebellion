//! Galaxy map rendering and egui UI panels.

pub mod audio;
pub mod combat_view;
pub mod encyclopedia;
pub mod fleet_movement;
pub mod fog;
pub mod message_log;
pub mod panels;
pub mod victory_screen;

use egui_macroquad::egui;
use macroquad::prelude::*;
use rebellion_core::ids::SystemKey;
use rebellion_core::tick::{GameClock, GameSpeed};
use rebellion_core::world::GameWorld;

pub use audio::{draw_audio_controls, AudioVolumeState, MusicTrack, SfxKind};
pub use combat_view::{draw_combat_summary, BattleOutcome, CombatResult, CombatSummaryState};
pub use victory_screen::{draw_victory_screen, GameStats, VictoryScreenState};
pub use encyclopedia::{draw_encyclopedia, EncyclopediaState, EncyclopediaTab};
pub use fleet_movement::draw_fleet_overlays;
pub use fog::draw_fog_overlay;
pub use message_log::{draw_message_log, GameMessage, MessageCategory, MessageLog, MessageLogState};
pub use panels::{
    draw_faction_select, draw_fleets, draw_manufacturing, draw_missions, draw_mod_manager,
    draw_officers, draw_save_load,
    FactionSelectState, FleetsState, ManufacturingPanelState, MissionsPanelState,
    ModInfo, ModManagerAction, ModManagerState,
    OfficersState, SaveLoadPanelState, SaveSlotInfo,
    PanelAction,
};

#[cfg(debug_assertions)]
pub use panels::command_palette::{CommandPaletteState, draw_command_palette};

/// Computed camera parameters for overlay rendering.
///
/// Returned by `draw_galaxy_map` so fog/fleet overlays can use matching
/// coordinates without recomputing screen dimensions.
pub struct CameraView {
    pub cam_x: f32,
    pub cam_y: f32,
    pub zoom: f32,
    pub map_width: f32,
    pub screen_height: f32,
}

/// All mutable UI state for the galaxy map view.
pub struct GalaxyMapState {
    pub camera_x: f32,
    pub camera_y: f32,
    pub zoom: f32,
    pub selected_system: Option<SystemKey>,
    pub hovered_system: Option<SystemKey>,
    pub show_sector_labels: bool,
    pub show_grid: bool,
    /// Previous mouse position used for right-drag panning.
    /// macroquad 0.4 has no mouse_delta_position(); we track it manually.
    pub drag_start: Option<(f32, f32)>,
}

impl Default for GalaxyMapState {
    fn default() -> Self {
        Self {
            camera_x: 450.0,
            camera_y: 470.0,
            zoom: 1.0,
            selected_system: None,
            hovered_system: None,
            show_sector_labels: true,
            show_grid: false,
            drag_start: None,
        }
    }
}

/// Render the galaxy star map for one frame.
///
/// Handles all input (pan, zoom, click-to-select) and draws every system as a
/// colored dot sized by selection state. Returns a `CameraView` so fog and fleet
/// overlays can use matching coordinates.
pub fn draw_galaxy_map(world: &GameWorld, state: &mut GalaxyMapState) -> CameraView {
    clear_background(Color::new(0.02, 0.02, 0.08, 1.0));

    let sw = screen_width();
    let sh = screen_height();
    // Reserve space on the right when a system is selected.
    let panel_width: f32 = if state.selected_system.is_some() {
        300.0
    } else {
        0.0
    };
    let map_width = sw - panel_width;

    let (mx, my) = mouse_position();

    // ── Input: only when the cursor is in the map area ───────────────────────
    if mx < map_width {
        // Zoom with scroll wheel (vertical component).
        let wheel_y = mouse_wheel().1;
        if wheel_y != 0.0 {
            let factor = if wheel_y > 0.0 { 1.1_f32 } else { 1.0 / 1.1 };
            state.zoom = (state.zoom * factor).clamp(0.3, 5.0);
        }

        // Pan with right-mouse drag.
        // We store the position from last frame and compute the delta ourselves.
        if is_mouse_button_down(MouseButton::Right) {
            if let Some((px, py)) = state.drag_start {
                let dx = mx - px;
                let dy = my - py;
                state.camera_x -= dx / state.zoom;
                state.camera_y -= dy / state.zoom;
            }
            state.drag_start = Some((mx, my));
        } else {
            state.drag_start = None;
        }
    } else {
        // Cursor moved into the egui panel — release drag.
        state.drag_start = None;
    }

    // ── Coordinate transform helpers ─────────────────────────────────────────
    let cam_x = state.camera_x;
    let cam_y = state.camera_y;
    let zoom = state.zoom;

    let to_screen = |dat_x: f32, dat_y: f32| -> (f32, f32) {
        let sx = (dat_x - cam_x) * zoom + map_width / 2.0;
        let sy = (dat_y - cam_y) * zoom + sh / 2.0;
        (sx, sy)
    };

    // ── Sector labels ─────────────────────────────────────────────────────────
    if state.show_sector_labels {
        for (_, sector) in &world.sectors {
            let (sx, sy) = to_screen(sector.x as f32, sector.y as f32);
            if sx > -100.0 && sx < map_width + 100.0 && sy > -100.0 && sy < sh + 100.0 {
                let font_size = (16.0 * zoom).clamp(10.0, 32.0);
                draw_text(
                    &sector.name,
                    sx - 30.0,
                    sy,
                    font_size,
                    Color::new(0.3, 0.3, 0.5, 0.6),
                );
            }
        }
    }

    // ── Find hovered system (reset each frame) ────────────────────────────────
    state.hovered_system = None;
    let hover_radius = 8.0 * zoom;

    // ── Draw systems ──────────────────────────────────────────────────────────
    // Two-pass: first collect which system is hovered so the selected glow
    // renders on top without requiring Z-sort.

    // Pass 1: hover detection — pick the nearest system within hover_radius.
    if mx < map_width {
        let mut best_dist = hover_radius;
        for (key, system) in &world.systems {
            let (sx, sy) = to_screen(system.x as f32, system.y as f32);
            if sx < -20.0 || sx > map_width + 20.0 || sy < -20.0 || sy > sh + 20.0 {
                continue;
            }
            let dist = ((mx - sx).powi(2) + (my - sy).powi(2)).sqrt();
            if dist < best_dist {
                best_dist = dist;
                state.hovered_system = Some(key);
            }
        }
    }

    // Pass 2: draw all visible systems.
    for (key, system) in &world.systems {
        let (sx, sy) = to_screen(system.x as f32, system.y as f32);
        if sx < -20.0 || sx > map_width + 20.0 || sy < -20.0 || sy > sh + 20.0 {
            continue;
        }

        let is_selected = state.selected_system == Some(key);
        let is_hovered = state.hovered_system == Some(key);

        let color = if system.popularity_alliance > system.popularity_empire {
            Color::new(0.2, 0.5, 1.0, 1.0)
        } else if system.popularity_empire > system.popularity_alliance {
            Color::new(0.8, 0.2, 0.2, 1.0)
        } else {
            Color::new(0.6, 0.6, 0.6, 1.0)
        };

        let radius = if is_selected {
            5.0 * zoom
        } else if is_hovered {
            4.0 * zoom
        } else {
            3.0 * zoom
        };

        // Selection glow ring.
        if is_selected {
            draw_circle(sx, sy, radius + 3.0, Color::new(1.0, 1.0, 0.3, 0.3));
        }

        draw_circle(sx, sy, radius, color);

        // Name label on hover.
        if is_hovered {
            draw_text(&system.name, sx + 10.0, sy - 5.0, 18.0, WHITE);
        }
    }

    // ── Click to select ───────────────────────────────────────────────────────
    if is_mouse_button_pressed(MouseButton::Left) && mx < map_width {
        state.selected_system = state.hovered_system;
    }

    CameraView {
        cam_x,
        cam_y,
        zoom,
        map_width,
        screen_height: sh,
    }
}

/// Draw the system info side panel (right side).
///
/// Shows details for the currently selected system. Call inside
/// `egui_macroquad::ui(|ctx| { ... })`.
pub fn draw_system_info_panel(ctx: &egui::Context, world: &GameWorld, state: &GalaxyMapState) {
    if let Some(sys_key) = state.selected_system {
        if let Some(system) = world.systems.get(sys_key) {
            egui::SidePanel::right("system_info")
                .min_width(280.0)
                .max_width(300.0)
                .show(ctx, |ui| {
                    ui.heading(&system.name);
                    ui.separator();

                    if let Some(sector) = world.sectors.get(system.sector) {
                        ui.label(format!("Sector: {}", sector.name));
                        let region_label = match sector.group {
                            rebellion_core::dat::SectorGroup::Core => "Core",
                            rebellion_core::dat::SectorGroup::RimInner => "Inner Rim",
                            rebellion_core::dat::SectorGroup::RimOuter => "Outer Rim",
                        };
                        ui.label(format!("Region: {}", region_label));
                    }

                    ui.separator();
                    ui.label(format!("Position: ({}, {})", system.x, system.y));
                    ui.label(format!(
                        "Alliance Support: {:.0}%",
                        system.popularity_alliance * 100.0
                    ));
                    ui.label(format!(
                        "Empire Support: {:.0}%",
                        system.popularity_empire * 100.0
                    ));

                    ui.separator();
                    ui.label(format!("Fleets: {}", system.fleets.len()));
                    ui.label(format!("Ground Units: {}", system.ground_units.len()));
                    ui.label(format!("Defenses: {}", system.defense_facilities.len()));
                    ui.label(format!(
                        "Manufacturing: {}",
                        system.manufacturing_facilities.len()
                    ));
                    ui.label(format!("Production: {}", system.production_facilities.len()));
                });
        }
    }
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
