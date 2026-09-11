//! Cockpit frame rendering — the chrome border around the galaxy map.
//!
//! Renders a faction-specific decorative border that frames the galaxy map
//! area, replicating the "cockpit" aesthetic of the original game's strategy
//! view.  When BMP assets are staged (`data/base/ui/`), the background texture
//! is loaded via `BmpCache`; otherwise a styled fallback is drawn using theme
//! colors and macroquad primitives.
//!
//! # Screen layout
//!
//! ```text
//! ┌──────────────────────────────────────────────────┐
//! │  [top bar: faction logo + status]                │
//! │                                                  │
//! │  ┌────────────────────────────────────────────┐  │
//! │  │                                            │  │
//! │  │         GALAXY MAP VIEWPORT                │  │
//! │  │                                            │  │
//! │  └────────────────────────────────────────────┘  │
//! │                                                  │
//! │  [bottom bar: cockpit control buttons]           │
//! └──────────────────────────────────────────────────┘
//! ```
//!
//! The original command center is a fixed 640×480 composition. Wider or taller
//! browser windows therefore letterbox one uniformly scaled canvas rather than
//! stretching independent layers. The returned `CockpitViewport` is the exact
//! faction aperture recovered from `FUN_00421c70`.
//!
//! # BMP resource IDs
//!
//! | DLL | ID | Content |
//! |-----|----|---------|
//! | STRATEGY | 900 | Alliance command-center shell (640×481 source, 640×480 display) |
//! | STRATEGY | 901 | Imperial command-center shell (640×481 source, 640×480 display) |
//! | COMMON | 20001 | Main-menu background (640×480) |
//! | COMMON | 11001-11275 | Animated cockpit display sequences, not a sequential logical-button map |
//!
//! The bitmap shells and aperture geometry are faction-specific. No synthetic
//! top or bottom chrome is drawn underneath them.

use egui_macroquad::egui::{self, Ui};
use macroquad::prelude::*;

use crate::bmp_cache::{resources, BmpCache, DllSource};
use crate::theme;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Which player faction owns this cockpit chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CockpitFaction {
    Alliance,
    Empire,
}

/// Logical width of the original strategic command-center surface.
pub const STRATEGIC_LOGICAL_WIDTH: f32 = 640.0;

/// Logical height displayed by the original strategic command center.
///
/// The recovered STRATEGY resources contain one extra source row. It is not
/// part of the displayed 640×480 composition.
pub const STRATEGIC_LOGICAL_HEIGHT: f32 = 480.0;

/// Cockpit button identifiers.
///
/// These correspond to the nine main strategy-view control buttons in the
/// original game.  Keyboard shortcuts are listed as fallbacks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CockpitButton {
    /// Officers panel (O)
    Officers,
    /// Fleets panel (F)
    Fleets,
    /// Manufacturing panel (M)
    Manufacturing,
    /// Missions panel (N)
    Missions,
    /// Research panel (T)
    Research,
    /// Encyclopedia (E)
    Encyclopedia,
    /// Save / Load
    SaveLoad,
    /// Speed: decrease
    SpeedDown,
    /// Speed: increase
    SpeedUp,
}

/// Pixel viewport the galaxy map should render into.
///
/// All coordinates are in macroquad screen pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CockpitViewport {
    /// Left edge of the usable map area (pixels from left).
    pub x: f32,
    /// Top edge of the usable map area (pixels from top).
    pub y: f32,
    /// Width of the usable map area in pixels.
    pub width: f32,
    /// Height of the usable map area in pixels.
    pub height: f32,
}

impl CockpitViewport {
    /// Viewport that fills the entire screen (no cockpit chrome).
    pub fn fullscreen() -> Self {
        CockpitViewport {
            x: 0.0,
            y: 0.0,
            width: screen_width(),
            height: screen_height(),
        }
    }

    /// Right edge in screen pixels.
    pub fn right(self) -> f32 {
        self.x + self.width
    }

    /// Bottom edge in screen pixels.
    pub fn bottom(self) -> f32 {
        self.y + self.height
    }

    /// Whether a screen-space point lies inside this viewport.
    pub fn contains(self, x: f32, y: f32) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }
}

/// Uniformly scaled strategic canvas and its transparent galaxy aperture.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CockpitLayout {
    pub canvas: CockpitViewport,
    pub galaxy: CockpitViewport,
    pub scale: f32,
}

/// All mutable state owned by the cockpit module.
pub struct CockpitState {
    /// Which faction's chrome to render.
    pub faction: CockpitFaction,
    /// Height of the top decorative bar in pixels.
    pub top_bar_h: f32,
    /// Height of the bottom button bar in pixels.
    pub bottom_bar_h: f32,
    /// Side gutters width in pixels (equal left/right).
    pub side_gutter_w: f32,
}

impl Default for CockpitState {
    fn default() -> Self {
        CockpitState {
            faction: CockpitFaction::Alliance,
            top_bar_h: 32.0,
            bottom_bar_h: 40.0,
            side_gutter_w: 0.0, // no side gutters for now — full width
        }
    }
}

impl CockpitState {
    pub fn new(faction: CockpitFaction) -> Self {
        CockpitState {
            faction,
            ..Default::default()
        }
    }

    /// Compute the recovered command-center layout for the current screen.
    pub fn layout(&self) -> CockpitLayout {
        self.layout_for(screen_width(), screen_height())
    }

    /// Compute the recovered command-center layout for an arbitrary screen.
    ///
    /// This pure variant keeps the 640×480 composition testable without a
    /// graphics context.
    pub fn layout_for(&self, screen_width: f32, screen_height: f32) -> CockpitLayout {
        let scale = (screen_width / STRATEGIC_LOGICAL_WIDTH)
            .min(screen_height / STRATEGIC_LOGICAL_HEIGHT)
            .max(0.0);
        let canvas = CockpitViewport {
            x: (screen_width - STRATEGIC_LOGICAL_WIDTH * scale) / 2.0,
            y: (screen_height - STRATEGIC_LOGICAL_HEIGHT * scale) / 2.0,
            width: STRATEGIC_LOGICAL_WIDTH * scale,
            height: STRATEGIC_LOGICAL_HEIGHT * scale,
        };

        // FUN_00421c70 constructs these exact client rectangles. The right and
        // bottom values are exclusive in the original Win32 RECT contract.
        let (x, y, width, height) = match self.faction {
            CockpitFaction::Alliance => (55.0, 40.0, 485.0, 350.0),
            CockpitFaction::Empire => (120.0, 40.0, 480.0, 355.0),
        };
        let galaxy = CockpitViewport {
            x: canvas.x + x * scale,
            y: canvas.y + y * scale,
            width: width * scale,
            height: height * scale,
        };

        CockpitLayout {
            canvas,
            galaxy,
            scale,
        }
    }

    /// Compute the galaxy map viewport for the current screen.
    pub fn galaxy_viewport(&self) -> CockpitViewport {
        self.layout().galaxy
    }
}

// ---------------------------------------------------------------------------
// Draw functions
// ---------------------------------------------------------------------------

/// Prepare the strategic canvas and return its exact layout.
///
/// Call before the macroquad galaxy layers. The authentic shell is painted in
/// egui later in the same frame, above the clipped map and below other windows.
pub fn draw_cockpit_chrome(state: &CockpitState) -> CockpitLayout {
    clear_background(BLACK);
    state.layout()
}

/// Apply or clear macroquad's top-left-origin scissor rectangle.
///
/// All strategic map layers use this one clip, preventing synthetic map pixels
/// from leaking into advisor and command-control apertures in the shell.
pub fn set_cockpit_viewport_clip(viewport: Option<CockpitViewport>) {
    let clip = viewport.map(|viewport| {
        (
            viewport.x.round() as i32,
            viewport.y.round() as i32,
            viewport.width.round() as i32,
            viewport.height.round() as i32,
        )
    });
    // SAFETY: macroquad exposes its immediate drawing state through this API.
    // The clip is reset before egui begins its pass in the same frame.
    unsafe {
        get_internal_gl().quad_gl.scissor(clip);
    }
}

/// Draw the faction's authentic STRATEGY.DLL cockpit frame as the first egui
/// layer of the frame. Panels rendered afterward remain readable above it.
pub fn draw_cockpit_background(ctx: &egui::Context, state: &CockpitState, cache: &mut BmpCache) {
    let background_id = if state.faction == CockpitFaction::Alliance {
        resources::strategy::GALAXY_BACKGROUND
    } else {
        resources::strategy::GALAXY_BACKGROUND_EMPIRE
    };
    let Some(texture) = cache.get(ctx, DllSource::Strategy, background_id) else {
        return;
    };

    // `SidePanel` paints on egui's canonical background layer. Painting the
    // cockpit into a separate `Order::Background` layer can still place that
    // layer above side panels, depending on egui's area ordering. Use the same
    // canonical layer instead: this shape is appended first, then panels append
    // their frames, text, and bitmaps over it later in the frame.
    let painter = ctx.layer_painter(egui::LayerId::background());
    let canvas = state.layout().canvas;
    let uv_max_y = cockpit_source_uv_max_y(texture.size());
    painter.image(
        texture.id(),
        egui::Rect::from_min_size(
            egui::pos2(canvas.x, canvas.y),
            egui::vec2(canvas.width, canvas.height),
        ),
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, uv_max_y)),
        egui::Color32::WHITE,
    );
}

fn cockpit_source_uv_max_y(texture_size: [usize; 2]) -> f32 {
    let visible_source_height = (texture_size[0] as f32 * STRATEGIC_LOGICAL_HEIGHT
        / STRATEGIC_LOGICAL_WIDTH)
        .min(texture_size[1] as f32);
    visible_source_height / texture_size[1] as f32
}

/// Draw egui-layer cockpit elements: control button bar.
///
/// Call inside `egui_macroquad::ui(|ctx| { ... })`.
///
/// Returns the `CockpitButton` that was clicked this frame, if any.
pub fn draw_cockpit_egui_layer(
    ctx: &egui::Context,
    state: &CockpitState,
    _cache: &mut BmpCache,
    // Panel visibility flags so buttons show active state
    show_officers: bool,
    show_fleets: bool,
    show_manufacturing: bool,
    show_missions: bool,
    show_research: bool,
    enc_open: bool,
) -> Option<CockpitButton> {
    let sw = screen_width();
    let sh = screen_height();
    let bottom_y = sh - state.bottom_bar_h;

    let mut clicked: Option<CockpitButton> = None;

    // ── Bottom button bar ────────────────────────────────────────────────────
    // Place an egui panel anchored to the bottom of the screen, matching the
    // macroquad-drawn chrome bar.
    egui::Area::new(egui::Id::new("cockpit_buttons"))
        .fixed_pos(egui::pos2(0.0, bottom_y + 2.0))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            ui.set_width(sw);
            ui.set_height(state.bottom_bar_h - 2.0);

            ui.horizontal_centered(|ui| {
                ui.add_space(8.0);

                let faction_active = if state.faction == CockpitFaction::Alliance {
                    theme::ALLIANCE_BLUE
                } else {
                    theme::EMPIRE_RED
                };

                // The extracted 11001–11275 resources are animated cockpit
                // sequences, not one logical button per numeric triplet. Until
                // the original command-to-sequence table is resolved, use a
                // clear functional label instead of displaying unrelated art.
                let control_btn =
                    |ui: &mut Ui, label: &str, key: &str, active: bool| -> bool {
                        let text = format!("{}\n[{}]", label, key);
                        let rt = egui::RichText::new(text).size(9.0).color(if active {
                            faction_active
                        } else {
                            theme::TEXT_SECONDARY
                        });
                        ui.add(egui::Button::new(rt).min_size(egui::vec2(52.0, 32.0)).fill(
                            if active {
                                egui::Color32::from_rgba_unmultiplied(30, 60, 120, 200)
                            } else {
                                egui::Color32::from_rgba_unmultiplied(10, 15, 30, 200)
                            },
                        ))
                        .on_hover_text(format!("{} [{}]", label, key))
                        .clicked()
                    };

                // Main panel buttons (Officers → Encyclopedia)
                let buttons: &[(CockpitButton, &str, &str, bool)] = &[
                    (CockpitButton::Officers, "Officers", "O", show_officers),
                    (CockpitButton::Fleets, "Fleets", "F", show_fleets),
                    (CockpitButton::Manufacturing, "Mfg", "M", show_manufacturing),
                    (CockpitButton::Missions, "Missions", "N", show_missions),
                    (CockpitButton::Research, "Research", "T", show_research),
                    (CockpitButton::Encyclopedia, "Encyclopedia", "E", enc_open),
                ];
                for &(btn_id, label, key, active) in buttons {
                    if control_btn(ui, label, key, active) {
                        clicked = Some(btn_id);
                    }
                }

                ui.add_space(16.0);

                if control_btn(ui, "Save/Load", "S", false) {
                    clicked = Some(CockpitButton::SaveLoad);
                }

                ui.add_space(16.0);

                if control_btn(ui, "Slower", "<", false) {
                    clicked = Some(CockpitButton::SpeedDown);
                }
                if control_btn(ui, "Faster", ">", false) {
                    clicked = Some(CockpitButton::SpeedUp);
                }
            });
        });

    clicked
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.001,
            "expected {expected}, got {actual}"
        );
    }

    fn assert_viewport(actual: CockpitViewport, x: f32, y: f32, width: f32, height: f32) {
        assert_close(actual.x, x);
        assert_close(actual.y, y);
        assert_close(actual.width, width);
        assert_close(actual.height, height);
    }

    #[test]
    fn alliance_uses_recovered_640_by_480_aperture() {
        let layout = CockpitState::new(CockpitFaction::Alliance).layout_for(640.0, 480.0);

        assert_close(layout.scale, 1.0);
        assert_viewport(layout.canvas, 0.0, 0.0, 640.0, 480.0);
        assert_viewport(layout.galaxy, 55.0, 40.0, 485.0, 350.0);
    }

    #[test]
    fn empire_uses_recovered_640_by_480_aperture() {
        let layout = CockpitState::new(CockpitFaction::Empire).layout_for(640.0, 480.0);

        assert_close(layout.scale, 1.0);
        assert_viewport(layout.canvas, 0.0, 0.0, 640.0, 480.0);
        assert_viewport(layout.galaxy, 120.0, 40.0, 480.0, 355.0);
    }

    #[test]
    fn widescreen_is_uniformly_scaled_and_pillarboxed() {
        let layout = CockpitState::new(CockpitFaction::Alliance).layout_for(1280.0, 800.0);
        let scale = 800.0 / 480.0;

        assert_close(layout.scale, scale);
        assert_viewport(
            layout.canvas,
            (1280.0 - 640.0 * scale) / 2.0,
            0.0,
            640.0 * scale,
            800.0,
        );
        assert_close(layout.galaxy.x, layout.canvas.x + 55.0 * scale);
        assert_close(layout.galaxy.y, 40.0 * scale);
        assert_close(layout.galaxy.width, 485.0 * scale);
        assert_close(layout.galaxy.height, 350.0 * scale);
    }

    #[test]
    fn tall_screen_is_uniformly_scaled_and_letterboxed() {
        let layout = CockpitState::new(CockpitFaction::Empire).layout_for(640.0, 600.0);

        assert_close(layout.scale, 1.0);
        assert_viewport(layout.canvas, 0.0, 60.0, 640.0, 480.0);
        assert_viewport(layout.galaxy, 120.0, 100.0, 480.0, 355.0);
    }

    #[test]
    fn extra_strategy_source_row_is_not_displayed() {
        assert_close(cockpit_source_uv_max_y([640, 481]), 480.0 / 481.0);
        assert_close(cockpit_source_uv_max_y([1280, 962]), 960.0 / 962.0);
        assert_close(cockpit_source_uv_max_y([640, 480]), 1.0);
    }

    #[test]
    fn recovered_aperture_uses_exclusive_right_and_bottom_edges() {
        let viewport = CockpitState::new(CockpitFaction::Empire)
            .layout_for(640.0, 480.0)
            .galaxy;

        assert!(viewport.contains(120.0, 40.0));
        assert!(viewport.contains(599.999, 394.999));
        assert!(!viewport.contains(600.0, 394.0));
        assert!(!viewport.contains(599.0, 395.0));
    }
}
