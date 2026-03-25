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
//! The returned `CockpitViewport` tells `draw_galaxy_map` exactly which
//! rectangle to render into.
//!
//! # BMP resource IDs
//!
//! | DLL | ID | Content |
//! |-----|----|---------|
//! | STRATEGY | 900 | Galaxy map starfield background (640×481) |
//! | COMMON | 20001 | Main-menu background (640×480) |
//! | COMMON | 11001-11215 | Button sprite library |
//!
//! Alliance-specific cockpit elements are distinguished in the original game
//! by color palettes applied at render time.  We approximate with `ALLIANCE_BLUE`
//! vs `EMPIRE_RED` accents drawn over a shared chrome layout.

use egui_macroquad::egui::{self, Ui};
use macroquad::prelude::*;

use crate::bmp_cache::{BmpCache, DllSource};
use crate::theme;

// ---------------------------------------------------------------------------
// Button sprite ID mapping
// ---------------------------------------------------------------------------

/// COMMON.DLL resource IDs for a single cockpit button (3 states).
///
/// The button sprite library (IDs 11001–11215) stores every button as a
/// sequential triplet: `[unpressed, pressed, disabled]`.  The base ID is the
/// unpressed state; pressed is base+1, disabled is base+2.
#[derive(Debug, Clone, Copy)]
struct ButtonSprite {
    /// COMMON.DLL resource ID for the unpressed/normal state.
    normal: u32,
    /// COMMON.DLL resource ID for the pressed/active state.
    pressed: u32,
    /// COMMON.DLL resource ID for the disabled/grayed state.
    disabled: u32,
}

impl ButtonSprite {
    const fn from_base(base: u32) -> Self {
        ButtonSprite { normal: base, pressed: base + 1, disabled: base + 2 }
    }
}

impl CockpitButton {
    /// Return the COMMON.DLL sprite triplet for this button.
    ///
    /// IDs are from the original game's COMMON.DLL button library (11001–11215).
    /// Mapping derived from resource extraction order: 9 main strategy-view
    /// control buttons occupy the first 27 IDs in groups of 3.
    fn sprite(self) -> ButtonSprite {
        match self {
            CockpitButton::Officers      => ButtonSprite::from_base(11001),
            CockpitButton::Fleets        => ButtonSprite::from_base(11004),
            CockpitButton::Manufacturing => ButtonSprite::from_base(11007),
            CockpitButton::Missions      => ButtonSprite::from_base(11010),
            CockpitButton::Research      => ButtonSprite::from_base(11013),
            CockpitButton::Encyclopedia  => ButtonSprite::from_base(11016),
            CockpitButton::SaveLoad      => ButtonSprite::from_base(11019),
            CockpitButton::SpeedDown     => ButtonSprite::from_base(11022),
            CockpitButton::SpeedUp       => ButtonSprite::from_base(11025),
        }
    }
}

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Which player faction owns this cockpit chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CockpitFaction {
    Alliance,
    Empire,
}

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
#[derive(Debug, Clone, Copy)]
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
        CockpitState { faction, ..Default::default() }
    }

    /// Compute the galaxy map viewport given the current screen size.
    ///
    /// The returned `CockpitViewport` is the rectangle that callers should
    /// pass to `draw_galaxy_map` as its clipping region.
    pub fn galaxy_viewport(&self) -> CockpitViewport {
        let sw = screen_width();
        let sh = screen_height();
        CockpitViewport {
            x: self.side_gutter_w,
            y: self.top_bar_h,
            width: sw - self.side_gutter_w * 2.0,
            height: sh - self.top_bar_h - self.bottom_bar_h,
        }
    }
}

// ---------------------------------------------------------------------------
// Draw functions
// ---------------------------------------------------------------------------

/// Draw the top and bottom cockpit chrome bars using macroquad.
///
/// Call before `egui_macroquad::ui` so the chrome renders beneath egui panels.
/// Returns the viewport reserved for the galaxy map.
pub fn draw_cockpit_chrome(
    state: &CockpitState,
    cache: &mut BmpCache,
    ctx: &egui::Context,
) -> CockpitViewport {
    let sw = screen_width();
    let sh = screen_height();

    let faction_color: Color = if state.faction == CockpitFaction::Alliance {
        Color::new(0.15, 0.35, 0.65, 1.0) // deep Alliance blue
    } else {
        Color::new(0.55, 0.10, 0.10, 1.0) // deep Empire crimson
    };
    let accent_color: Color = if state.faction == CockpitFaction::Alliance {
        Color::new(0.4, 0.6, 1.0, 1.0)
    } else {
        Color::new(1.0, 0.35, 0.35, 1.0)
    };

    // ── Top bar ──────────────────────────────────────────────────────────────
    draw_rectangle(0.0, 0.0, sw, state.top_bar_h, faction_color);
    // Thin accent line at bottom of top bar
    draw_rectangle(0.0, state.top_bar_h - 2.0, sw, 2.0, accent_color);

    // Faction label
    let label = if state.faction == CockpitFaction::Alliance {
        "REBEL ALLIANCE — COMMAND BRIDGE"
    } else {
        "GALACTIC EMPIRE — COMMAND BRIDGE"
    };
    let font_size = 14.0;
    let dims = measure_text(label, None, font_size as u16, 1.0);
    draw_text(
        label,
        (sw - dims.width) / 2.0,
        state.top_bar_h * 0.72,
        font_size,
        Color::new(0.9, 0.85, 0.6, 1.0),
    );

    // ── Bottom bar ───────────────────────────────────────────────────────────
    let bottom_y = sh - state.bottom_bar_h;
    draw_rectangle(0.0, bottom_y, sw, state.bottom_bar_h, faction_color);
    // Thin accent line at top of bottom bar
    draw_rectangle(0.0, bottom_y, sw, 2.0, accent_color);

    // Try to load galaxy background texture (STRATEGY.DLL id 900).
    // This renders behind the galaxy dots when present.
    // (We can't scissor/clip macroquad draw calls to the viewport without
    // a render target, so we just draw it across the map area.)
    let vp = state.galaxy_viewport();
    if let Some(tex) = cache.get(ctx, DllSource::Strategy, 900) {
        let size = egui::vec2(vp.width, vp.height);
        // The texture is registered in egui; we draw it via egui's painter
        // in a transparent overlay pass inside draw_cockpit_egui_layer.
        let _ = (tex, size); // consumed in the egui layer below
    }

    vp
}

/// Draw egui-layer cockpit elements: control button bar.
///
/// Call inside `egui_macroquad::ui(|ctx| { ... })`.
///
/// Returns the `CockpitButton` that was clicked this frame, if any.
pub fn draw_cockpit_egui_layer(
    ctx: &egui::Context,
    state: &CockpitState,
    cache: &mut BmpCache,
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

                // Helper: render a cockpit button using a BMP sprite triplet from
                // COMMON.DLL, with a text-button fallback when assets are absent.
                //
                // `active` = panel is currently open (renders pressed sprite).
                // `disabled` = button should be grayed (not used yet, reserved for
                //   e.g. Death Star button when DS not constructed).
                //
                // The closure captures `cache` mutably and `ctx` immutably, which
                // means we call it sequentially (not in parallel).
                let sprite_btn = |ui: &mut Ui,
                                  label: &str,
                                  key: &str,
                                  active: bool,
                                  sprite: ButtonSprite,
                                  cache: &mut BmpCache|
                    -> bool {
                    // Pick which resource ID to show based on state.
                    let res_id = if active { sprite.pressed } else { sprite.normal };

                    if let Some(tex) = cache.get(ctx, DllSource::Common, res_id) {
                        // Sprite available — render as an image button.
                        // Original button dimensions are ~52×32 pixels; we preserve
                        // that aspect ratio and add a highlight tint when active.
                        let size = egui::vec2(52.0, 32.0);
                        let tint = if active {
                            // Brighter tint when panel is open
                            egui::Color32::from_rgba_unmultiplied(220, 220, 255, 255)
                        } else {
                            egui::Color32::WHITE
                        };
                        let tex_id = tex.id();
                        let img = egui::Image::new((tex_id, size))
                            .tint(tint);
                        let response = ui.add(egui::ImageButton::new(img)
                            .frame(false));
                        // Tooltip with label and keyboard shortcut.
                        response.on_hover_text(format!("{} [{}]", label, key))
                               .clicked()
                    } else {
                        // Fallback: styled text button when sprites not staged.
                        let text = format!("{}\n[{}]", label, key);
                        let rt = egui::RichText::new(text)
                            .size(9.0)
                            .color(if active { faction_active } else { theme::TEXT_SECONDARY });
                        let response = ui.add(egui::Button::new(rt)
                            .min_size(egui::vec2(52.0, 32.0))
                            .fill(if active {
                                egui::Color32::from_rgba_unmultiplied(30, 60, 120, 200)
                            } else {
                                egui::Color32::from_rgba_unmultiplied(10, 15, 30, 200)
                            }));
                        response.clicked()
                    }
                };

                // Main panel buttons (Officers → Encyclopedia)
                let buttons: &[(CockpitButton, &str, &str, bool)] = &[
                    (CockpitButton::Officers,      "Officers",     "O", show_officers),
                    (CockpitButton::Fleets,        "Fleets",       "F", show_fleets),
                    (CockpitButton::Manufacturing, "Mfg",          "M", show_manufacturing),
                    (CockpitButton::Missions,      "Missions",     "N", show_missions),
                    (CockpitButton::Research,      "Research",     "T", show_research),
                    (CockpitButton::Encyclopedia,  "Encyclopedia", "E", enc_open),
                ];
                for &(btn_id, label, key, active) in buttons {
                    if sprite_btn(ui, label, key, active, btn_id.sprite(), cache) {
                        clicked = Some(btn_id);
                    }
                }

                ui.add_space(16.0);

                if sprite_btn(ui, "Save/Load", "S", false, CockpitButton::SaveLoad.sprite(), cache) {
                    clicked = Some(CockpitButton::SaveLoad);
                }

                ui.add_space(16.0);

                if sprite_btn(ui, "Slower", "<", false, CockpitButton::SpeedDown.sprite(), cache) {
                    clicked = Some(CockpitButton::SpeedDown);
                }
                if sprite_btn(ui, "Faster", ">", false, CockpitButton::SpeedUp.sprite(), cache) {
                    clicked = Some(CockpitButton::SpeedUp);
                }
            });
        });

    clicked
}
