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

use egui_macroquad::egui;
use macroquad::prelude::*;

use crate::bmp_cache::{resources, BmpCache, DllSource};

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

/// Original strategic command-control identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CockpitButton {
    /// Find a known star system (F2).
    SystemFinder,
    /// Find a fleet or ship (F3).
    FleetFinder,
    /// Find a troop unit (F4).
    TroopFinder,
    /// Find a character or special force (F5).
    PersonnelFinder,
    /// Open the original game-options destination (F7).
    GameOptions,
    /// Open the Encyclopedia.
    Encyclopedia,
}

/// Recovered native control record for one strategic command button.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrategicControlSpec {
    pub button: CockpitButton,
    pub command_id: u16,
    pub rect: CockpitViewport,
    pub normal_resource: u32,
    pub pressed_resource: u32,
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

const ALLIANCE_PRIMARY_CONTROLS: [StrategicControlSpec; 6] = [
    StrategicControlSpec {
        button: CockpitButton::SystemFinder,
        command_id: 0x12d,
        rect: CockpitViewport {
            x: 106.0,
            y: 408.0,
            width: 27.0,
            height: 16.0,
        },
        normal_resource: resources::strategy::ALLIANCE_SYSTEM_FINDER_NORMAL,
        pressed_resource: resources::strategy::ALLIANCE_SYSTEM_FINDER_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::FleetFinder,
        command_id: 0x12e,
        rect: CockpitViewport {
            x: 157.0,
            y: 407.0,
            width: 27.0,
            height: 15.0,
        },
        normal_resource: resources::strategy::ALLIANCE_FLEET_FINDER_NORMAL,
        pressed_resource: resources::strategy::ALLIANCE_FLEET_FINDER_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::PersonnelFinder,
        command_id: 0x12f,
        rect: CockpitViewport {
            x: 258.0,
            y: 405.0,
            width: 27.0,
            height: 16.0,
        },
        normal_resource: resources::strategy::ALLIANCE_PERSONNEL_FINDER_NORMAL,
        pressed_resource: resources::strategy::ALLIANCE_PERSONNEL_FINDER_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::TroopFinder,
        command_id: 0x130,
        rect: CockpitViewport {
            x: 209.0,
            y: 405.0,
            width: 27.0,
            height: 16.0,
        },
        normal_resource: resources::strategy::ALLIANCE_TROOP_FINDER_NORMAL,
        pressed_resource: resources::strategy::ALLIANCE_TROOP_FINDER_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::GameOptions,
        command_id: 0x131,
        rect: CockpitViewport {
            x: 394.0,
            y: 405.0,
            width: 27.0,
            height: 16.0,
        },
        normal_resource: resources::strategy::ALLIANCE_GAME_OPTIONS_NORMAL,
        pressed_resource: resources::strategy::ALLIANCE_GAME_OPTIONS_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::Encyclopedia,
        command_id: 0x132,
        rect: CockpitViewport {
            x: 446.0,
            y: 406.0,
            width: 27.0,
            height: 16.0,
        },
        normal_resource: resources::strategy::ALLIANCE_ENCYCLOPEDIA_NORMAL,
        pressed_resource: resources::strategy::ALLIANCE_ENCYCLOPEDIA_PRESSED,
    },
];

const EMPIRE_PRIMARY_CONTROLS: [StrategicControlSpec; 6] = [
    StrategicControlSpec {
        button: CockpitButton::SystemFinder,
        command_id: 0x12d,
        rect: CockpitViewport {
            x: 143.0,
            y: 434.0,
            width: 37.0,
            height: 24.0,
        },
        normal_resource: resources::strategy::EMPIRE_SYSTEM_FINDER_NORMAL,
        pressed_resource: resources::strategy::EMPIRE_SYSTEM_FINDER_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::FleetFinder,
        command_id: 0x12e,
        rect: CockpitViewport {
            x: 199.0,
            y: 434.0,
            width: 37.0,
            height: 24.0,
        },
        normal_resource: resources::strategy::EMPIRE_FLEET_FINDER_NORMAL,
        pressed_resource: resources::strategy::EMPIRE_FLEET_FINDER_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::PersonnelFinder,
        command_id: 0x12f,
        rect: CockpitViewport {
            x: 412.0,
            y: 433.0,
            width: 34.0,
            height: 22.0,
        },
        normal_resource: resources::strategy::EMPIRE_PERSONNEL_FINDER_NORMAL,
        pressed_resource: resources::strategy::EMPIRE_PERSONNEL_FINDER_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::TroopFinder,
        command_id: 0x130,
        rect: CockpitViewport {
            x: 253.0,
            y: 433.0,
            width: 34.0,
            height: 22.0,
        },
        normal_resource: resources::strategy::EMPIRE_TROOP_FINDER_NORMAL,
        pressed_resource: resources::strategy::EMPIRE_TROOP_FINDER_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::GameOptions,
        command_id: 0x131,
        rect: CockpitViewport {
            x: 465.0,
            y: 434.0,
            width: 35.0,
            height: 24.0,
        },
        normal_resource: resources::strategy::EMPIRE_GAME_OPTIONS_NORMAL,
        pressed_resource: resources::strategy::EMPIRE_GAME_OPTIONS_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::Encyclopedia,
        command_id: 0x132,
        rect: CockpitViewport {
            x: 519.0,
            y: 434.0,
            width: 37.0,
            height: 25.0,
        },
        normal_resource: resources::strategy::EMPIRE_ENCYCLOPEDIA_NORMAL,
        pressed_resource: resources::strategy::EMPIRE_ENCYCLOPEDIA_PRESSED,
    },
];

/// Exact primary-control table created by `FUN_00427270` for a faction.
pub fn strategic_primary_controls(faction: CockpitFaction) -> &'static [StrategicControlSpec; 6] {
    match faction {
        CockpitFaction::Alliance => &ALLIANCE_PRIMARY_CONTROLS,
        CockpitFaction::Empire => &EMPIRE_PRIMARY_CONTROLS,
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
    /// Primary control currently holding native-style pointer capture.
    pressed_control: Option<CockpitButton>,
}

impl Default for CockpitState {
    fn default() -> Self {
        CockpitState {
            faction: CockpitFaction::Alliance,
            top_bar_h: 32.0,
            bottom_bar_h: 40.0,
            side_gutter_w: 0.0, // no side gutters for now — full width
            pressed_control: None,
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
        resources::strategy::ALLIANCE_COMMAND_CENTER_SHELL
    } else {
        resources::strategy::EMPIRE_COMMAND_CENTER_SHELL
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

/// Paint the six native primary strategic controls over their shell apertures.
///
/// `FUN_00602d30` paints the first resource in each pair at rest and the
/// second only while a valid primary press is captured. The original control
/// has no separate hover or persistent-selected bitmap state.
pub fn draw_cockpit_egui_layer(ctx: &egui::Context, state: &CockpitState, cache: &mut BmpCache) {
    let layout = state.layout();
    let controls = strategic_primary_controls(state.faction);
    let primary_down = ctx.input(|input| input.pointer.button_down(egui::PointerButton::Primary));
    let painter = ctx.layer_painter(egui::LayerId::background());

    for control in controls {
        let pressed = primary_down && state.pressed_control == Some(control.button);
        let resource_id = control_resource(control, pressed);
        let Some(original_size) =
            cache.original_resource_size(DllSource::Strategy, control.normal_resource)
        else {
            continue;
        };
        let Some(texture_id) = cache
            .get(ctx, DllSource::Strategy, resource_id)
            .map(|texture| texture.id())
        else {
            continue;
        };
        let screen_rect = logical_rect_to_screen(layout, control.rect);
        let image_rect = egui::Rect::from_min_size(
            screen_rect.min,
            egui::vec2(
                original_size[0] as f32 * layout.scale,
                original_size[1] as f32 * layout.scale,
            ),
        );
        painter.with_clip_rect(screen_rect).image(
            texture_id,
            image_rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }
}

/// Resolve control input after floating panels have registered their areas.
///
/// This prevents a control hidden by a modeless window from receiving the
/// window's click. Paint remains on egui's canonical background layer.
pub fn handle_cockpit_egui_input(
    ctx: &egui::Context,
    state: &mut CockpitState,
    cache: &mut BmpCache,
) -> Option<CockpitButton> {
    let layout = state.layout();
    let controls = strategic_primary_controls(state.faction);
    let (pointer_pos, primary_pressed, primary_down, primary_released) = ctx.input(|input| {
        (
            input.pointer.interact_pos(),
            input.pointer.button_pressed(egui::PointerButton::Primary),
            input.pointer.button_down(egui::PointerButton::Primary),
            input.pointer.button_released(egui::PointerButton::Primary),
        )
    });
    let pointer_hit = if ctx.is_pointer_over_area() {
        None
    } else {
        pointer_pos.and_then(|pointer| control_at_pointer(cache, controls, layout, pointer))
    };

    let clicked = update_control_capture(
        &mut state.pressed_control,
        primary_pressed,
        primary_down,
        primary_released,
        pointer_hit,
    );

    clicked.or_else(|| keyboard_control(ctx))
}

fn update_control_capture(
    captured: &mut Option<CockpitButton>,
    primary_pressed: bool,
    primary_down: bool,
    primary_released: bool,
    pointer_hit: Option<CockpitButton>,
) -> Option<CockpitButton> {
    if primary_pressed {
        *captured = pointer_hit;
    } else if !primary_down && !primary_released {
        // Browser focus loss can omit the release event. Do not leave a
        // native pressed frame latched when capture has ended.
        *captured = None;
    }

    if primary_released {
        let pressed = captured.take();
        pressed.filter(|button| Some(*button) == pointer_hit)
    } else {
        None
    }
}

fn control_resource(control: &StrategicControlSpec, pressed: bool) -> u32 {
    if pressed {
        control.pressed_resource
    } else {
        control.normal_resource
    }
}

fn logical_rect_to_screen(layout: CockpitLayout, rect: CockpitViewport) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(
            layout.canvas.x + rect.x * layout.scale,
            layout.canvas.y + rect.y * layout.scale,
        ),
        egui::vec2(rect.width * layout.scale, rect.height * layout.scale),
    )
}

fn resource_pixel_at_pointer(
    screen_rect: egui::Rect,
    scale: f32,
    pointer: egui::Pos2,
) -> Option<(usize, usize)> {
    if scale <= 0.0
        || pointer.x <= screen_rect.min.x
        || pointer.y <= screen_rect.min.y
        || pointer.x >= screen_rect.max.x
        || pointer.y >= screen_rect.max.y
    {
        return None;
    }
    Some((
        ((pointer.x - screen_rect.min.x) / scale).floor() as usize,
        ((pointer.y - screen_rect.min.y) / scale).floor() as usize,
    ))
}

fn control_at_pointer(
    cache: &mut BmpCache,
    controls: &[StrategicControlSpec],
    layout: CockpitLayout,
    pointer: egui::Pos2,
) -> Option<CockpitButton> {
    controls.iter().find_map(|control| {
        let screen_rect = logical_rect_to_screen(layout, control.rect);
        let (x, y) = resource_pixel_at_pointer(screen_rect, layout.scale, pointer)?;
        cache
            .is_resource_hit(DllSource::Strategy, control.normal_resource, x, y)
            .then_some(control.button)
    })
}

fn keyboard_control(ctx: &egui::Context) -> Option<CockpitButton> {
    let egui_key = ctx.input(|input| {
        [
            (egui::Key::F2, CockpitButton::SystemFinder),
            (egui::Key::F3, CockpitButton::FleetFinder),
            (egui::Key::F4, CockpitButton::TroopFinder),
            (egui::Key::F5, CockpitButton::PersonnelFinder),
            (egui::Key::F7, CockpitButton::GameOptions),
        ]
        .into_iter()
        .find_map(|(key, button)| input.key_pressed(key).then_some(button))
    });
    egui_key.or_else(|| {
        [
            KeyCode::F2,
            KeyCode::F3,
            KeyCode::F4,
            KeyCode::F5,
            KeyCode::F7,
        ]
        .into_iter()
        .find(|key| is_key_pressed(*key))
        .and_then(macroquad_accelerator)
    })
}

fn macroquad_accelerator(key: KeyCode) -> Option<CockpitButton> {
    match key {
        KeyCode::F2 => Some(CockpitButton::SystemFinder),
        KeyCode::F3 => Some(CockpitButton::FleetFinder),
        KeyCode::F4 => Some(CockpitButton::TroopFinder),
        KeyCode::F5 => Some(CockpitButton::PersonnelFinder),
        KeyCode::F7 => Some(CockpitButton::GameOptions),
        _ => None,
    }
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

    #[test]
    fn alliance_primary_controls_match_recovered_constructor_records() {
        let controls = strategic_primary_controls(CockpitFaction::Alliance);
        let records: Vec<_> = controls
            .iter()
            .map(|control| {
                (
                    control.button,
                    control.command_id,
                    control.rect,
                    control.normal_resource,
                    control.pressed_resource,
                )
            })
            .collect();

        assert_eq!(
            records,
            vec![
                (
                    CockpitButton::SystemFinder,
                    0x12d,
                    CockpitViewport {
                        x: 106.0,
                        y: 408.0,
                        width: 27.0,
                        height: 16.0,
                    },
                    10002,
                    10001,
                ),
                (
                    CockpitButton::FleetFinder,
                    0x12e,
                    CockpitViewport {
                        x: 157.0,
                        y: 407.0,
                        width: 27.0,
                        height: 15.0,
                    },
                    10004,
                    10003,
                ),
                (
                    CockpitButton::PersonnelFinder,
                    0x12f,
                    CockpitViewport {
                        x: 258.0,
                        y: 405.0,
                        width: 27.0,
                        height: 16.0,
                    },
                    10006,
                    10005,
                ),
                (
                    CockpitButton::TroopFinder,
                    0x130,
                    CockpitViewport {
                        x: 209.0,
                        y: 405.0,
                        width: 27.0,
                        height: 16.0,
                    },
                    10008,
                    10007,
                ),
                (
                    CockpitButton::GameOptions,
                    0x131,
                    CockpitViewport {
                        x: 394.0,
                        y: 405.0,
                        width: 27.0,
                        height: 16.0,
                    },
                    10010,
                    10009,
                ),
                (
                    CockpitButton::Encyclopedia,
                    0x132,
                    CockpitViewport {
                        x: 446.0,
                        y: 406.0,
                        width: 27.0,
                        height: 16.0,
                    },
                    10012,
                    10011,
                ),
            ]
        );
    }

    #[test]
    fn empire_primary_controls_match_recovered_constructor_records() {
        let controls = strategic_primary_controls(CockpitFaction::Empire);
        assert_eq!(
            controls
                .iter()
                .map(|control| (
                    control.command_id,
                    control.rect.x,
                    control.rect.y,
                    control.rect.width,
                    control.rect.height,
                    control.normal_resource,
                    control.pressed_resource,
                ))
                .collect::<Vec<_>>(),
            vec![
                (0x12d, 143.0, 434.0, 37.0, 24.0, 10016, 10015),
                (0x12e, 199.0, 434.0, 37.0, 24.0, 10018, 10017),
                (0x12f, 412.0, 433.0, 34.0, 22.0, 10020, 10019),
                (0x130, 253.0, 433.0, 34.0, 22.0, 10022, 10021),
                (0x131, 465.0, 434.0, 35.0, 24.0, 10024, 10023),
                (0x132, 519.0, 434.0, 37.0, 25.0, 10026, 10025),
            ]
        );
    }

    #[test]
    fn primary_control_rects_follow_uniform_canvas_scaling() {
        let layout = CockpitState::new(CockpitFaction::Empire).layout_for(1600.0, 960.0);
        let screen_rect = logical_rect_to_screen(
            layout,
            strategic_primary_controls(CockpitFaction::Empire)[0].rect,
        );

        assert_close(layout.scale, 2.0);
        assert_close(screen_rect.min.x, 160.0 + 143.0 * 2.0);
        assert_close(screen_rect.min.y, 434.0 * 2.0);
        assert_close(screen_rect.width(), 74.0);
        assert_close(screen_rect.height(), 48.0);
    }

    #[test]
    fn native_pointer_conversion_excludes_exact_outer_edges() {
        let rect = egui::Rect::from_min_max(egui::pos2(10.0, 20.0), egui::pos2(64.0, 52.0));

        assert_eq!(resource_pixel_at_pointer(rect, 2.0, rect.min), None);
        assert_eq!(
            resource_pixel_at_pointer(rect, 2.0, egui::pos2(10.1, 20.1)),
            Some((0, 0))
        );
        assert_eq!(
            resource_pixel_at_pointer(rect, 2.0, egui::pos2(12.0, 22.0)),
            Some((1, 1))
        );
        assert_eq!(
            resource_pixel_at_pointer(rect, 2.0, egui::pos2(63.999, 51.999)),
            Some((26, 15))
        );
        assert_eq!(resource_pixel_at_pointer(rect, 2.0, rect.max), None);
        assert_eq!(
            resource_pixel_at_pointer(rect, 2.0, egui::pos2(rect.max.x, 30.0)),
            None
        );
        assert_eq!(
            resource_pixel_at_pointer(rect, 2.0, egui::pos2(30.0, rect.max.y)),
            None
        );
    }

    #[test]
    fn native_capture_dispatches_only_after_release_over_the_same_control() {
        let mut captured = None;
        assert_eq!(
            update_control_capture(
                &mut captured,
                true,
                true,
                false,
                Some(CockpitButton::FleetFinder),
            ),
            None
        );
        assert_eq!(captured, Some(CockpitButton::FleetFinder));

        assert_eq!(
            update_control_capture(
                &mut captured,
                false,
                false,
                true,
                Some(CockpitButton::FleetFinder),
            ),
            Some(CockpitButton::FleetFinder)
        );
        assert_eq!(captured, None);
    }

    #[test]
    fn native_capture_cancels_on_outside_release_or_lost_capture() {
        let mut captured = Some(CockpitButton::PersonnelFinder);
        assert_eq!(
            update_control_capture(&mut captured, false, false, true, None),
            None
        );
        assert_eq!(captured, None);

        captured = Some(CockpitButton::TroopFinder);
        assert_eq!(
            update_control_capture(&mut captured, false, false, false, None),
            None
        );
        assert_eq!(captured, None);
    }

    #[test]
    fn control_art_and_macroquad_accelerators_match_native_states() {
        let control = &strategic_primary_controls(CockpitFaction::Alliance)[0];
        assert_eq!(control_resource(control, false), control.normal_resource);
        assert_eq!(control_resource(control, true), control.pressed_resource);
        assert_eq!(
            macroquad_accelerator(KeyCode::F2),
            Some(CockpitButton::SystemFinder)
        );
        assert_eq!(
            macroquad_accelerator(KeyCode::F3),
            Some(CockpitButton::FleetFinder)
        );
        assert_eq!(
            macroquad_accelerator(KeyCode::F4),
            Some(CockpitButton::TroopFinder)
        );
        assert_eq!(
            macroquad_accelerator(KeyCode::F5),
            Some(CockpitButton::PersonnelFinder)
        );
        assert_eq!(
            macroquad_accelerator(KeyCode::F7),
            Some(CockpitButton::GameOptions)
        );
        assert_eq!(macroquad_accelerator(KeyCode::F6), None);
    }
}
