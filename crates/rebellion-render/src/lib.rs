//! Galaxy map rendering and egui UI panels.

pub mod advisor;
pub mod audio;
pub mod bmp_cache;
pub mod cockpit;
pub mod combat_view;
pub mod encyclopedia;
pub mod fleet_movement;
pub mod ground_combat;
pub mod fog;
pub mod main_menu;
pub mod message_log;
pub mod panels;
pub mod tactical_view;
pub mod theme;
pub mod victory_screen;

use egui_macroquad::egui;
use macroquad::prelude::*;
use rebellion_core::blockade::BlockadeState;
use rebellion_core::ids::{FleetKey, SystemKey};
use rebellion_core::missions::MissionFaction;
use rebellion_core::movement::MovementState;
use rebellion_core::tick::{GameClock, GameSpeed};
use rebellion_core::world::GameWorld;

pub use advisor::{
    draw_advisor, advisor_greet, advisor_mission_result, advisor_combat_result,
    advisor_uprising, advisor_death_star, advisor_manufacturing_complete,
    AdvisorFaction, AdvisorMessage, AdvisorPriority, AdvisorState,
};
pub use audio::{draw_audio_controls, AudioVolumeState, MusicContext, MusicTrack, SfxKind, VoiceLine};
pub use bmp_cache::{BmpCache, DllSource};
pub use cockpit::{
    draw_cockpit_chrome, draw_cockpit_egui_layer,
    CockpitButton, CockpitFaction, CockpitState, CockpitViewport,
};
pub use combat_view::{draw_combat_summary, BattleOutcome, CombatResult, CombatSummaryState};
pub use ground_combat::{draw_ground_combat, GroundAction, GroundCombatState, GroundWinner};
pub use tactical_view::{draw_tactical_view, BattlePhase, BattleSession, CombatWinner, TacticalAction, TacticalState};
pub use victory_screen::{draw_victory_screen, GameStats, VictoryScreenState};
pub use encyclopedia::{draw_encyclopedia, EncyclopediaState, EncyclopediaTab};
pub use fleet_movement::{draw_fleet_overlays, hovered_fleet};
pub use fog::draw_fog_overlay;
pub use main_menu::{draw_main_menu, MainMenuAction};
pub use message_log::{draw_message_log, GameMessage, MessageCategory, MessageLog, MessageLogState};
pub use panels::{
    draw_fleets, draw_manufacturing, draw_missions, draw_mod_manager,
    draw_officers, draw_save_load,
    FleetsState, ManufacturingPanelState, MissionsPanelState,
    ModInfo, ModManagerAction, ModManagerState,
    OfficersState, SaveLoadPanelState, SaveSlotInfo,
    PanelAction,
};
pub use panels::game_setup::{draw_game_setup, Difficulty, GameSetupAction, GameSetupState};

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
    /// System context menu: system key + screen position of right-click.
    pub context_menu_system: Option<(SystemKey, f32, f32)>,
    /// Fleet context menu: fleet key + screen position of right-click.
    pub context_menu_fleet: Option<(FleetKey, f32, f32)>,
    /// Tracks whether right-mouse dragged (to distinguish click from pan).
    pub right_click_start: Option<(f32, f32)>,
    /// Cockpit viewport bounds for mouse input clamping.
    /// If set, mouse input outside this rect is ignored.
    pub viewport: Option<(f32, f32, f32, f32)>,
    /// Frame counter incremented while right-mouse is held.
    /// Used as a WASM fallback: browsers swallow the mouseup on right-click
    /// (context menu intercepts it), so `is_mouse_button_released` never fires.
    /// When the button goes from held → not-held without a released event, we
    /// detect it on the next frame via this counter (> 0 but button not down).
    pub right_click_held_frames: u32,
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
            context_menu_system: None,
            context_menu_fleet: None,
            right_click_start: None,
            viewport: None,
            right_click_held_frames: 0,
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

    // Check if mouse is within the cockpit viewport (if set).
    let in_viewport = if let Some((vx, vy, vw, vh)) = state.viewport {
        mx >= vx && mx <= vx + vw && my >= vy && my <= vy + vh
    } else {
        true
    };

    // ── Input: only when the cursor is in the map area ───────────────────────
    if mx < map_width && in_viewport {
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
        // Left-click anywhere dismisses context menus.
        state.context_menu_system = None;
        state.context_menu_fleet = None;
    }

    // ── Right-click context menu ─────────────────────────────────────────────
    // Open context menu only on a short right-click (no drag).
    //
    // WASM note: browsers intercept the contextmenu event and swallow the
    // mouseup, so `is_mouse_button_released(Right)` never fires in WebAssembly.
    // We detect the fallback condition: right_click_start is set (button was
    // pressed at least 1 frame) but button is no longer held this frame and no
    // released event arrived. Both paths share the same drag-distance check.
    //
    // NOTE: Do NOT clear right_click_start here — main.rs needs to read it
    // for fleet hover detection before it gets cleared. main.rs clears it.
    let right_released = is_mouse_button_released(MouseButton::Right);
    // WASM fallback: held for ≥1 frame, button no longer down, no released event.
    let right_released_wasm = !right_released
        && state.right_click_held_frames >= 1
        && !is_mouse_button_down(MouseButton::Right)
        && state.right_click_start.is_some();
    if (right_released || right_released_wasm) && mx < map_width {
        // Reset held counter on either release path.
        state.right_click_held_frames = 0;
        let was_drag = state.right_click_start.map_or(true, |(sx, sy)| {
            let dist = ((mx - sx).powi(2) + (my - sy).powi(2)).sqrt();
            dist > 5.0
        });
        if !was_drag {
            if let Some(sys_key) = state.hovered_system {
                state.context_menu_system = Some((sys_key, mx, my));
                state.context_menu_fleet = None;
            } else {
                // Clicked empty space — dismiss.
                state.context_menu_system = None;
                state.context_menu_fleet = None;
            }
        }
    }

    CameraView {
        cam_x,
        cam_y,
        zoom,
        map_width,
        screen_height: sh,
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
    let sx = (dat_x - cam.cam_x) * cam.zoom + cam.map_width / 2.0;
    let sy = (dat_y - cam.cam_y) * cam.zoom + cam.screen_height / 2.0;
    (sx, sy)
}

/// True if the screen-space point is inside the visible map area.
#[inline]
fn in_map_viewport(sx: f32, sy: f32, cam: &CameraView) -> bool {
    sx > -30.0 && sx < cam.map_width + 30.0 && sy > -30.0 && sy < cam.screen_height + 30.0
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
pub fn draw_facility_icons(world: &GameWorld, cam: &CameraView) {
    if cam.zoom < 0.5 {
        return;
    }

    let icon_size = (4.0 * cam.zoom).clamp(2.5, 8.0);
    let gap = icon_size + 1.5;

    for (_key, system) in &world.systems {
        let has_prod = !system.production_facilities.is_empty();
        let has_mfg = !system.manufacturing_facilities.is_empty();
        let has_def = !system.defense_facilities.is_empty();

        if !has_prod && !has_mfg && !has_def {
            continue;
        }

        let (sx, sy) = map_to_screen(system.x as f32, system.y as f32, cam);
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
            draw_rectangle(ix, base_y, icon_size, icon_size, Color::new(0.9, 0.8, 0.1, 0.85));
            col += 1;
        }
        if has_mfg {
            // Cyan: manufacturing (shipyards)
            let ix = base_x + col as f32 * gap;
            draw_rectangle(ix, base_y, icon_size, icon_size, Color::new(0.2, 0.8, 0.9, 0.85));
            col += 1;
        }
        if has_def {
            // Orange: defense installations
            let ix = base_x + col as f32 * gap;
            draw_rectangle(ix, base_y, icon_size, icon_size, Color::new(0.9, 0.5, 0.1, 0.85));
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
    pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then(a.1.partial_cmp(&b.1).unwrap()));
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
        while hull.len() >= lower_len && cross(hull[hull.len() - 2], hull[hull.len() - 1], p) <= 0.0 {
            hull.pop();
        }
        hull.push(p);
    }

    hull.pop(); // Remove the last point (same as first).
    hull
}

/// Dim sector-specific color from the SectorGroup.
fn sector_boundary_color(group: rebellion_core::dat::SectorGroup) -> Color {
    match group {
        rebellion_core::dat::SectorGroup::Core =>
            Color::new(0.7, 0.6, 0.2, 0.25), // warm gold — galactic core
        rebellion_core::dat::SectorGroup::RimInner =>
            Color::new(0.3, 0.5, 0.7, 0.22), // cool blue — inner rim
        rebellion_core::dat::SectorGroup::RimOuter =>
            Color::new(0.4, 0.3, 0.6, 0.20), // purple-grey — outer rim
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
pub fn draw_sector_boundaries(world: &GameWorld, cam: &CameraView, show: bool) {
    if !show {
        return;
    }

    let padding = 12.0 * cam.zoom.clamp(0.5, 2.0);

    for (_sec_key, sector) in &world.sectors {
        if sector.systems.len() < 2 {
            continue;
        }

        // Collect screen-space positions of all systems in this sector.
        let pts: Vec<(f32, f32)> = sector
            .systems
            .iter()
            .filter_map(|&sk| world.systems.get(sk))
            .map(|sys| map_to_screen(sys.x as f32, sys.y as f32, cam))
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
            draw_line(x1, y1, x2, y2, 1.0, color);
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

    let ring_radius = (7.0 * cam.zoom).clamp(5.0, 18.0);
    let ring_color = Color::new(0.9, 0.15, 0.15, 0.7);

    for &sys_key in blockaded {
        let system = match world.systems.get(sys_key) {
            Some(s) => s,
            None => continue,
        };

        let (sx, sy) = map_to_screen(system.x as f32, system.y as f32, cam);
        if !in_map_viewport(sx, sy, cam) {
            continue;
        }

        // Outer ring — solid dim red.
        draw_circle_lines(sx, sy, ring_radius, 1.5, ring_color);
        // Inner fill — very faint red tint over the system dot.
        draw_circle(sx, sy, ring_radius - 1.5, Color::new(0.9, 0.1, 0.1, 0.08));
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
                .max_width(320.0)
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        // ── Header ───────────────────────────────────────────
                        let name_color = match system.control {
                            rebellion_core::world::ControlKind::Controlled(rebellion_core::dat::Faction::Alliance) => theme::ALLIANCE_BLUE,
                            rebellion_core::world::ControlKind::Controlled(rebellion_core::dat::Faction::Empire) => theme::EMPIRE_RED,
                            _ => theme::TEXT_PRIMARY,
                        };
                        ui.heading(egui::RichText::new(&system.name).color(name_color));

                        if system.is_headquarters {
                            ui.label(egui::RichText::new("HEADQUARTERS").color(theme::GOLD).size(10.0).strong());
                        }
                        if system.is_destroyed {
                            ui.label(egui::RichText::new("DESTROYED").color(theme::DANGER_RED).size(10.0).strong());
                        }

                        // Sector + region
                        if let Some(sector) = world.sectors.get(system.sector) {
                            let region = match sector.group {
                                rebellion_core::dat::SectorGroup::Core => "Core",
                                rebellion_core::dat::SectorGroup::RimInner => "Inner Rim",
                                rebellion_core::dat::SectorGroup::RimOuter => "Outer Rim",
                            };
                            ui.label(egui::RichText::new(format!("{} — {}", sector.name, region)).color(theme::TEXT_SECONDARY).size(11.0));
                        }

                        ui.separator();

                        // ── Popularity bars ──────────────────────────────────
                        ui.label(egui::RichText::new("SUPPORT").color(theme::GOLD_DIM).size(10.0).strong());

                        let alliance_pct = system.popularity_alliance;
                        let empire_pct = system.popularity_empire;

                        // Alliance bar
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Alliance").color(theme::ALLIANCE_BLUE).size(11.0));
                            let bar = egui::ProgressBar::new(alliance_pct)
                                .text(format!("{:.0}%", alliance_pct * 100.0))
                                .fill(theme::ALLIANCE_BLUE);
                            ui.add(bar);
                        });

                        // Empire bar
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Empire  ").color(theme::EMPIRE_RED).size(11.0));
                            let bar = egui::ProgressBar::new(empire_pct)
                                .text(format!("{:.0}%", empire_pct * 100.0))
                                .fill(theme::EMPIRE_RED);
                            ui.add(bar);
                        });

                        // Control status
                        let control_str = match system.control {
                            rebellion_core::world::ControlKind::Uncontrolled => "Neutral",
                            rebellion_core::world::ControlKind::Controlled(rebellion_core::dat::Faction::Alliance) => "Alliance Controlled",
                            rebellion_core::world::ControlKind::Controlled(rebellion_core::dat::Faction::Empire) => "Empire Controlled",
                            rebellion_core::world::ControlKind::Controlled(rebellion_core::dat::Faction::Neutral) => "Neutral",
                            rebellion_core::world::ControlKind::Contested => "Contested",
                            rebellion_core::world::ControlKind::Uprising(_) => "Uprising",
                        };
                        ui.label(egui::RichText::new(control_str).color(theme::TEXT_SECONDARY).size(10.0));

                        ui.separator();

                        // ── Fleets ───────────────────────────────────────────
                        if !system.fleets.is_empty() {
                            ui.label(egui::RichText::new("FLEETS").color(theme::GOLD_DIM).size(10.0).strong());
                            for &fleet_key in &system.fleets {
                                if let Some(fleet) = world.fleets.get(fleet_key) {
                                    let faction_color = if fleet.is_alliance { theme::ALLIANCE_BLUE } else { theme::EMPIRE_RED };
                                    let faction_tag = if fleet.is_alliance { "A" } else { "E" };

                                    let ship_count: u32 = fleet.capital_ships.iter().map(|e| e.count).sum();
                                    let fighter_count: u32 = fleet.fighters.iter().map(|e| e.count).sum();

                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new(format!("[{}]", faction_tag)).color(faction_color).size(11.0).strong());

                                        let mut parts = Vec::new();
                                        if ship_count > 0 { parts.push(format!("{} ships", ship_count)); }
                                        if fighter_count > 0 { parts.push(format!("{} sqns", fighter_count)); }
                                        if fleet.has_death_star { parts.push("Death Star".to_string()); }
                                        ui.label(egui::RichText::new(parts.join(", ")).color(theme::TEXT_PRIMARY).size(11.0));
                                    });

                                    // Ship class breakdown
                                    for entry in &fleet.capital_ships {
                                        if let Some(class) = world.capital_ship_classes.get(entry.class) {
                                            ui.label(egui::RichText::new(format!("  {} ×{}", class.name, entry.count)).color(theme::TEXT_SECONDARY).size(10.0));
                                        }
                                    }

                                    // Commander
                                    for &char_key in &fleet.characters {
                                        if let Some(c) = world.characters.get(char_key) {
                                            ui.label(egui::RichText::new(format!("  Cmd: {}", c.name)).color(theme::GOLD_DIM).size(10.0));
                                        }
                                    }
                                }
                            }
                            ui.add_space(4.0);
                        }

                        // ── Ground Units ─────────────────────────────────────
                        if !system.ground_units.is_empty() {
                            let alliance_troops = system.ground_units.iter()
                                .filter(|k| world.troops.get(**k).map(|t| t.is_alliance).unwrap_or(false))
                                .count();
                            let empire_troops = system.ground_units.len() - alliance_troops;

                            ui.label(egui::RichText::new(format!("GROUND FORCES ({})", system.ground_units.len())).color(theme::GOLD_DIM).size(10.0).strong());
                            if alliance_troops > 0 {
                                ui.label(egui::RichText::new(format!("  Alliance: {} regiments", alliance_troops)).color(theme::ALLIANCE_BLUE).size(10.0));
                            }
                            if empire_troops > 0 {
                                ui.label(egui::RichText::new(format!("  Empire: {} regiments", empire_troops)).color(theme::EMPIRE_RED).size(10.0));
                            }
                            ui.add_space(4.0);
                        }

                        // ── Facilities ────────────────────────────────────────
                        let total_fac = system.defense_facilities.len()
                            + system.manufacturing_facilities.len()
                            + system.production_facilities.len();
                        if total_fac > 0 {
                            ui.label(egui::RichText::new(format!("FACILITIES ({})", total_fac)).color(theme::GOLD_DIM).size(10.0).strong());

                            if !system.defense_facilities.is_empty() {
                                ui.label(egui::RichText::new(format!("  Defense: {}", system.defense_facilities.len())).color(theme::TEXT_SECONDARY).size(10.0));
                            }
                            if !system.manufacturing_facilities.is_empty() {
                                ui.label(egui::RichText::new(format!("  Shipyards: {}", system.manufacturing_facilities.len())).color(theme::TEXT_SECONDARY).size(10.0));
                            }
                            if !system.production_facilities.is_empty() {
                                ui.label(egui::RichText::new(format!("  Production: {}", system.production_facilities.len())).color(theme::TEXT_SECONDARY).size(10.0));
                            }
                            ui.add_space(4.0);
                        }

                        // ── Coordinates (small, at bottom) ───────────────────
                        ui.separator();
                        ui.label(egui::RichText::new(format!("({}, {})", system.x, system.y)).color(theme::TEXT_DISABLED).size(9.0));
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
                rebellion_core::world::ControlKind::Controlled(rebellion_core::dat::Faction::Alliance) => theme::ALLIANCE_BLUE,
                rebellion_core::world::ControlKind::Controlled(rebellion_core::dat::Faction::Empire) => theme::EMPIRE_RED,
                _ => theme::TEXT_PRIMARY,
            };
            ui.label(egui::RichText::new(&system.name).color(name_color).strong().size(14.0));

            // Control status
            let control_str = match system.control {
                rebellion_core::world::ControlKind::Uncontrolled => "Neutral",
                rebellion_core::world::ControlKind::Controlled(rebellion_core::dat::Faction::Alliance) => "Alliance",
                rebellion_core::world::ControlKind::Controlled(rebellion_core::dat::Faction::Empire) => "Empire",
                rebellion_core::world::ControlKind::Controlled(rebellion_core::dat::Faction::Neutral) => "Neutral",
                rebellion_core::world::ControlKind::Contested => "Contested",
                rebellion_core::world::ControlKind::Uprising(_) => "Uprising",
            };
            ui.label(egui::RichText::new(control_str).color(theme::TEXT_SECONDARY).size(10.0));

            ui.separator();

            // ── Popularity snapshot ──────────────────────────────────────
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("A: {:.0}%", system.popularity_alliance * 100.0)).color(theme::ALLIANCE_BLUE).size(10.0));
                ui.label(egui::RichText::new(format!("E: {:.0}%", system.popularity_empire * 100.0)).color(theme::EMPIRE_RED).size(10.0));
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
                        ui.label(egui::RichText::new(format!("{} fleets", fleet_count)).color(theme::TEXT_SECONDARY).size(10.0));
                    }
                    if troop_count > 0 {
                        ui.label(egui::RichText::new(format!("{} troops", troop_count)).color(theme::TEXT_SECONDARY).size(10.0));
                    }
                    if fac_count > 0 {
                        ui.label(egui::RichText::new(format!("{} facilities", fac_count)).color(theme::TEXT_SECONDARY).size(10.0));
                    }
                });
            }

            ui.separator();

            // ── Quick actions ───────────────────────────────────────────
            if ui.button(egui::RichText::new("View Details").color(theme::GOLD).size(11.0)).clicked() {
                action = Some(PanelAction::FocusFleetSystem(sys_key));
                keep_open = false;
            }
            if ui.button(egui::RichText::new("Send Diplomat").color(theme::TEXT_PRIMARY).size(11.0)).clicked() {
                action = Some(PanelAction::OpenMissionTo {
                    target: sys_key,
                    kind: rebellion_core::missions::MissionKind::Diplomacy,
                    faction: player_faction,
                });
                keep_open = false;
            }
            if ui.button(egui::RichText::new("Send Spy").color(theme::TEXT_PRIMARY).size(11.0)).clicked() {
                action = Some(PanelAction::OpenMissionTo {
                    target: sys_key,
                    kind: rebellion_core::missions::MissionKind::Espionage,
                    faction: player_faction,
                });
                keep_open = false;
            }
            if ui.button(egui::RichText::new("Move Fleet Here").color(theme::TEXT_PRIMARY).size(11.0)).clicked() {
                action = Some(PanelAction::InitiateFleetMove { destination: sys_key });
                keep_open = false;
            }

            ui.add_space(2.0);
            if ui.small_button(egui::RichText::new("Close").color(theme::TEXT_DISABLED).size(10.0)).clicked() {
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
            let faction_color = if fleet.is_alliance { theme::ALLIANCE_BLUE } else { theme::EMPIRE_RED };
            let faction_tag = if fleet.is_alliance { "Alliance" } else { "Empire" };
            ui.label(egui::RichText::new(format!("{} Fleet", faction_tag)).color(faction_color).strong().size(13.0));

            // Location
            if let Some(sys) = world.systems.get(fleet.location) {
                ui.label(egui::RichText::new(format!("at {}", sys.name)).color(theme::TEXT_SECONDARY).size(10.0));
            }

            // Transit status
            if let Some(order) = movement_state.get(fleet_key) {
                if let Some(dest) = world.systems.get(order.destination) {
                    ui.label(egui::RichText::new(format!("→ {} ({}d)", dest.name, order.ticks_remaining()))
                        .color(theme::WARNING_AMBER).size(10.0));
                }
            }

            ui.separator();

            // ── Composition ─────────────────────────────────────────────
            let ship_count: u32 = fleet.capital_ships.iter().map(|e| e.count).sum();
            let fighter_count: u32 = fleet.fighters.iter().map(|e| e.count).sum();

            if ship_count > 0 {
                ui.label(egui::RichText::new(format!("{} capital ships", ship_count)).color(theme::TEXT_PRIMARY).size(11.0));
            }
            if fighter_count > 0 {
                ui.label(egui::RichText::new(format!("{} fighter sqns", fighter_count)).color(theme::TEXT_PRIMARY).size(11.0));
            }
            if fleet.has_death_star {
                ui.label(egui::RichText::new("DEATH STAR").color(theme::DANGER_RED).size(11.0).strong());
            }

            // Commander
            for &char_key in &fleet.characters {
                if let Some(c) = world.characters.get(char_key) {
                    ui.label(egui::RichText::new(format!("Cmd: {}", c.name)).color(theme::GOLD_DIM).size(10.0));
                }
            }

            ui.separator();

            // ── Quick actions ───────────────────────────────────────────
            if ui.button(egui::RichText::new("View in Fleet Panel").color(theme::GOLD).size(11.0)).clicked() {
                action = Some(PanelAction::FocusFleetSystem(fleet.location));
                keep_open = false;
            }

            // Transit status
            if movement_state.get(fleet_key).is_some() {
                ui.label(egui::RichText::new("In transit").color(theme::WARNING_AMBER).size(10.0));
            }

            ui.add_space(2.0);
            if ui.small_button(egui::RichText::new("Close").color(theme::TEXT_DISABLED).size(10.0)).clicked() {
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
