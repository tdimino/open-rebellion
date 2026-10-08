//! The galaxy display's detailed legend (`ghidra/notes/gid-legend.md`):
//! the window a double click on the compact legend opens, drawn from the
//! shown display's STRATEGY `RT_RCDATA` script.
//!
//! `FUN_004522f0` picks each display's script; `FUN_00452630` replays it:
//! after the window's width and height and its bitmap (7, STRATEGY) and
//! string (2, TEXTSTRA) modules, command 1 or 2 blits a bitmap keyed at
//! (x, y) and command 3 draws a string in a font and color, centred across
//! the window when its y is 1. `FUN_00452240` darkens the galaxy under the
//! window through a palette remap (`FUN_005fe050`) and keys the content and
//! the frame (STRATEGY 10100..10107, `FUN_00607740`) over it.

use egui_macroquad::egui;

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{
    galaxy_aperture, paint_gid_frame_border, CockpitFaction, CockpitLayout, GidMode,
};
use crate::theme::game_font;

/// One command of a legend script.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegendItem {
    /// Commands 1 and 2: a STRATEGY bitmap blitted keyed at (x, y).
    Bitmap { x: u16, y: u16, id: u32 },
    /// Command 3: TEXTSTRA string `id` in `font` and `color` at (x, y).
    Text {
        x: u16,
        y: u16,
        id: u32,
        text: &'static str,
        font: u8,
        color: (u8, u8, u8),
    },
}

use LegendItem::{Bitmap, Text};

/// A legend script: the window's size and its commands in order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Legend {
    pub width: u16,
    pub height: u16,
    pub items: &'static [LegendItem],
}

/// The window's initial position in the galaxy view (`FUN_00421c70`:
/// `+0x11c` 0x96, `+0x120` 0x78).
pub const INITIAL_POSITION: (f32, f32) = (150.0, 120.0);

/// The size `FUN_00426d00` clamps the position with (0xb4 by 0xf0), the
/// window's creation size before its script resizes it.
const CLAMP_SIZE: (f32, f32) = (180.0, 240.0);

/// The close button (`FUN_004522f0`: STRATEGY 10108, pressed 10109, at the
/// window's width less its own less 3, and 3 down).
const CLOSE_NORMAL: u32 = 10_108;
const CLOSE_PRESSED: u32 = 10_109;

/// hyp: the darkening table built from `LAB_004ac550` is untraced; this
/// tint matches captures of the original (2026-10-08 parity QA).
const DARKEN: egui::Color32 = egui::Color32::from_rgba_premultiplied(20, 20, 20, 205);

/// The legend `FUN_004522f0` builds for `mode`, by the player's side for
/// Popular Support. Display Off has none (`FUN_00426d00` with 0x80 shows
/// the compact legend instead).
#[must_use]
pub const fn legend(mode: GidMode, faction: CockpitFaction) -> Option<&'static Legend> {
    Some(match mode {
        GidMode::PopularSupport => match faction {
            CockpitFaction::Alliance => &POPULAR_SUPPORT_ALLIANCE,
            CockpitFaction::Empire => &POPULAR_SUPPORT_EMPIRE,
        },
        GidMode::Uprisings => &UPRISINGS,
        GidMode::IdleFleets => &IDLE_FLEETS,
        GidMode::FleetsEnRoute => &FLEETS_EN_ROUTE,
        GidMode::IdlePersonnel => &IDLE_PERSONNEL,
        GidMode::ActivePersonnel => &ACTIVE_PERSONNEL,
        GidMode::AvailableEnergy => &AVAILABLE_ENERGY,
        GidMode::AvailableRawMaterial => &AVAILABLE_RAW_MATERIAL,
        GidMode::Mines => &MINES,
        GidMode::Refineries => &REFINERIES,
        GidMode::Shipyards => &SHIPYARDS,
        GidMode::TrainingFacilities => &TRAINING_FACILITIES,
        GidMode::ConstructionYards => &CONSTRUCTION_YARDS,
        GidMode::IdleShipyards => &IDLE_SHIPYARDS,
        GidMode::IdleTrainingFacilities => &IDLE_TRAINING_FACILITIES,
        GidMode::IdleConstructionYards => &IDLE_CONSTRUCTION_YARDS,
        GidMode::Troopers => &TROOPERS,
        GidMode::FighterSquadrons => &FIGHTER_SQUADRONS,
        GidMode::DeathStarShields => &DEATH_STAR_SHIELDS,
        GidMode::PlanetaryShieldGenerators => &PLANETARY_SHIELD_GENERATORS,
        GidMode::PlanetaryDefenseBatteries => &PLANETARY_DEFENSE_BATTERIES,
        GidMode::DisplayOff => return None,
    })
}

/// The screen rect of the legend `window` shows for `mode`.
#[must_use]
pub fn window_rect(
    layout: CockpitLayout,
    faction: CockpitFaction,
    mode: GidMode,
    window: &LegendWindow,
) -> Option<egui::Rect> {
    let legend = legend(mode, faction)?;
    let (view_x, view_y, _, _) = galaxy_aperture(faction);
    Some(egui::Rect::from_min_size(
        egui::pos2(
            layout.canvas.x + (view_x + window.position.0) * layout.scale,
            layout.canvas.y + (view_y + window.position.1) * layout.scale,
        ),
        egui::vec2(f32::from(legend.width), f32::from(legend.height)) * layout.scale,
    ))
}

/// The open legend's place in the galaxy view, and its drag.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LegendWindow {
    /// The window's top-left in galaxy-view pixels (`+0x11c`, `+0x120`).
    pub position: (f32, f32),
    close_pressed: bool,
}

impl Default for LegendWindow {
    fn default() -> Self {
        Self {
            position: INITIAL_POSITION,
            close_pressed: false,
        }
    }
}

/// Keep a window of `size` inside the view. port: hyp: the near edges
/// clamp at zero too.
fn clamp_position(position: (f32, f32), size: (f32, f32), view: (f32, f32)) -> (f32, f32) {
    (
        position.0.min(view.0 - size.0).max(0.0),
        position.1.min(view.1 - size.1).max(0.0),
    )
}

/// `FUN_00426d00`: opening the legend keeps its creation size (0xb4 by
/// 0xf0) inside the view.
pub fn clamp_on_open(window: &mut LegendWindow, faction: CockpitFaction) {
    let (_, _, width, height) = galaxy_aperture(faction);
    window.position = clamp_position(window.position, CLAMP_SIZE, (width, height));
}

/// Draw the open legend; returns whether its close button was clicked.
/// port: hyp: the window drags from anywhere but its close button
/// (`FUN_006071a0`, which the constructor calls, is not decompiled; the
/// original drags in captures, 2026-10-08 parity QA).
pub fn draw_legend_window(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    layout: CockpitLayout,
    faction: CockpitFaction,
    mode: GidMode,
    window: &mut LegendWindow,
    input_enabled: bool,
) -> bool {
    let Some(legend) = legend(mode, faction) else {
        return false;
    };
    let scale = layout.scale;
    let (view_x, view_y, view_width, view_height) = galaxy_aperture(faction);
    let legend_size = (f32::from(legend.width), f32::from(legend.height));
    let origin = egui::pos2(
        layout.canvas.x + (view_x + window.position.0) * scale,
        layout.canvas.y + (view_y + window.position.1) * scale,
    );
    let size = egui::vec2(f32::from(legend.width), f32::from(legend.height)) * scale;
    let mut closed = false;
    egui::Area::new(egui::Id::new("original_gid_legend"))
        .order(egui::Order::Middle)
        .fixed_pos(origin)
        .show(ctx, |ui| {
            let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
            let close_size = cache
                .original_resource_size(DllSource::Strategy, CLOSE_NORMAL)
                .map_or(egui::vec2(14.0, 14.0), |size| {
                    egui::vec2(size[0] as f32, size[1] as f32)
                })
                * scale;
            let close_rect = egui::Rect::from_min_size(
                egui::pos2(rect.max.x - close_size.x - 3.0 * scale, rect.min.y + 3.0 * scale),
                close_size,
            );
            let sense = |sense| {
                if input_enabled {
                    sense
                } else {
                    egui::Sense::hover()
                }
            };
            let drag = ui.interact(
                rect,
                ui.id().with("drag"),
                sense(egui::Sense::click_and_drag()),
            );
            let close = ui.interact(
                close_rect,
                ui.id().with("close"),
                sense(egui::Sense::click()),
            );
            if drag.dragged() && !close.is_pointer_button_down_on() {
                let delta = drag.drag_delta() / scale;
                // port: hyp: a drag keeps the whole window in the view
                // (FUN_006071a0 is not decompiled).
                window.position = clamp_position(
                    (window.position.0 + delta.x, window.position.1 + delta.y),
                    legend_size,
                    (view_width, view_height),
                );
            }
            window.close_pressed = close.is_pointer_button_down_on();
            closed = close.clicked();

            let painter = ui.painter_at(rect);
            painter.rect_filled(rect, 0.0, DARKEN);
            let at = |x: u16, y: u16| {
                rect.min + egui::vec2(f32::from(x), f32::from(y)) * scale
            };
            for item in legend.items {
                match *item {
                    Bitmap { x, y, id } => {
                        let Some(source) = cache.original_resource_size(DllSource::Strategy, id)
                        else {
                            continue;
                        };
                        if let Some(texture) = cache.get(ctx, DllSource::Strategy, id) {
                            painter.image(
                                texture.id(),
                                egui::Rect::from_min_size(
                                    at(x, y),
                                    egui::vec2(source[0] as f32, source[1] as f32) * scale,
                                ),
                                egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                                egui::Color32::WHITE,
                            );
                        }
                    }
                    Text {
                        x,
                        y,
                        text,
                        font,
                        color,
                        ..
                    } => {
                        let color = egui::Color32::from_rgb(color.0, color.1, color.2);
                        if y == 1 {
                            painter.text(
                                egui::pos2(rect.center().x, at(0, y).y),
                                egui::Align2::CENTER_TOP,
                                text,
                                game_font(font, scale),
                                color,
                            );
                        } else {
                            painter.text(
                                at(x, y),
                                egui::Align2::LEFT_TOP,
                                text,
                                game_font(font, scale),
                                color,
                            );
                        }
                    }
                }
            }
            paint_gid_frame_border(ui, cache, rect, scale);
            let close_id = if window.close_pressed {
                CLOSE_PRESSED
            } else {
                CLOSE_NORMAL
            };
            if let Some(texture) = cache.get(ctx, DllSource::Strategy, close_id) {
                painter.image(
                    texture.id(),
                    close_rect,
                    egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            }
        });
    closed
}

/// STRATEGY `RT_RCDATA` 10183.
const POPULAR_SUPPORT_ALLIANCE: Legend = Legend {
    width: 180,
    height: 135,
    items: &[
        Text { x: 21, y: 1, id: 5717, text: "Loyalty to the Alliance", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5719, text: "Loyal", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10180 },
        Text { x: 25, y: 40, id: 5720, text: "Obedient", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 60, id: 10170 },
        Text { x: 25, y: 60, id: 5721, text: "Disloyal", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10169 },
        Text { x: 25, y: 80, id: 5728, text: "Hostile", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 100, id: 10182 },
        Bitmap { x: 7, y: 105, id: 10243 },
        Text { x: 25, y: 105, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 120, id: 10241 },
        Text { x: 25, y: 120, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 105, id: 10245 },
        Text { x: 105, y: 105, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 120, id: 10158 },
        Text { x: 105, y: 120, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 10184.
const POPULAR_SUPPORT_EMPIRE: Legend = Legend {
    width: 180,
    height: 135,
    items: &[
        Text { x: 21, y: 1, id: 5718, text: "Loyalty to the Empire", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5719, text: "Loyal", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10180 },
        Text { x: 25, y: 40, id: 5720, text: "Obedient", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 60, id: 10170 },
        Text { x: 25, y: 60, id: 5721, text: "Disloyal", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10169 },
        Text { x: 25, y: 80, id: 5728, text: "Hostile", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 100, id: 10182 },
        Bitmap { x: 7, y: 105, id: 10243 },
        Text { x: 25, y: 105, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 120, id: 10241 },
        Text { x: 25, y: 120, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 105, id: 10245 },
        Text { x: 105, y: 105, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 120, id: 10158 },
        Text { x: 105, y: 120, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 11600.
const UPRISINGS: Legend = Legend {
    width: 180,
    height: 95,
    items: &[
        Text { x: 31, y: 1, id: 5722, text: "Worlds in Uprising", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5723, text: "Currently in Uprising", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10169 },
        Text { x: 25, y: 40, id: 5724, text: "Not in Uprising", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 60, id: 10182 },
        Bitmap { x: 7, y: 65, id: 10243 },
        Text { x: 25, y: 65, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10241 },
        Text { x: 25, y: 80, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 65, id: 10245 },
        Text { x: 105, y: 65, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 80, id: 10158 },
        Text { x: 105, y: 80, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 11601.
const IDLE_FLEETS: Legend = Legend {
    width: 180,
    height: 155,
    items: &[
        Text { x: 58, y: 1, id: 5738, text: "Idle Fleets", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5730, text: "3+ fleets", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10180 },
        Text { x: 25, y: 40, id: 5731, text: "2  fleets", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 60, id: 10170 },
        Text { x: 25, y: 60, id: 5732, text: "1  fleet", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10169 },
        Text { x: 25, y: 80, id: 5733, text: "No fleets", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 100, id: 10160 },
        Text { x: 25, y: 100, id: 12582, text: "Conflict", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 120, id: 10182 },
        Bitmap { x: 7, y: 125, id: 10243 },
        Text { x: 25, y: 125, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 140, id: 10241 },
        Text { x: 25, y: 140, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 125, id: 10245 },
        Text { x: 105, y: 125, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 140, id: 10158 },
        Text { x: 105, y: 140, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 11602.
const FLEETS_EN_ROUTE: Legend = Legend {
    width: 180,
    height: 155,
    items: &[
        Text { x: 45, y: 1, id: 5739, text: "Fleets Enroute", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5730, text: "3+ fleets", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10180 },
        Text { x: 25, y: 40, id: 5731, text: "2  fleets", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 60, id: 10170 },
        Text { x: 25, y: 60, id: 5732, text: "1  fleet", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10169 },
        Text { x: 25, y: 80, id: 5733, text: "No fleets", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 100, id: 10160 },
        Text { x: 25, y: 100, id: 12582, text: "Conflict", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 120, id: 10182 },
        Bitmap { x: 7, y: 125, id: 10243 },
        Text { x: 25, y: 125, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 140, id: 10241 },
        Text { x: 25, y: 140, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 125, id: 10245 },
        Text { x: 105, y: 125, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 140, id: 10158 },
        Text { x: 105, y: 140, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 11603.
const IDLE_PERSONNEL: Legend = Legend {
    width: 180,
    height: 95,
    items: &[
        Text { x: 47, y: 1, id: 5754, text: "Idle Personnel", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5755, text: "Idle", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10169 },
        Text { x: 25, y: 40, id: 5756, text: "None", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 60, id: 10182 },
        Bitmap { x: 7, y: 65, id: 10243 },
        Text { x: 25, y: 65, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10241 },
        Text { x: 25, y: 80, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 65, id: 10245 },
        Text { x: 105, y: 65, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 80, id: 10158 },
        Text { x: 105, y: 80, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 11604.
const ACTIVE_PERSONNEL: Legend = Legend {
    width: 180,
    height: 95,
    items: &[
        Text { x: 40, y: 1, id: 5757, text: "Active Personnel", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5758, text: "Active", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10169 },
        Text { x: 25, y: 40, id: 5759, text: "None", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 60, id: 10182 },
        Bitmap { x: 7, y: 65, id: 10243 },
        Text { x: 25, y: 65, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10241 },
        Text { x: 25, y: 80, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 65, id: 10245 },
        Text { x: 105, y: 65, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 80, id: 10158 },
        Text { x: 105, y: 80, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 10188.
const AVAILABLE_ENERGY: Legend = Legend {
    width: 180,
    height: 135,
    items: &[
        Text { x: 38, y: 1, id: 5749, text: "Available Energy", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5750, text: "6+  Points Available", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10180 },
        Text { x: 25, y: 40, id: 5751, text: "3-5 Points Available", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 60, id: 10170 },
        Text { x: 25, y: 60, id: 5752, text: "1-2 Points Available", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10169 },
        Text { x: 25, y: 80, id: 5753, text: "0   Points Available", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 100, id: 10182 },
        Bitmap { x: 7, y: 105, id: 10243 },
        Text { x: 25, y: 105, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 120, id: 10241 },
        Text { x: 25, y: 120, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 105, id: 10245 },
        Text { x: 105, y: 105, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 120, id: 10158 },
        Text { x: 105, y: 120, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 10189.
const AVAILABLE_RAW_MATERIAL: Legend = Legend {
    width: 180,
    height: 135,
    items: &[
        Text { x: 18, y: 1, id: 5760, text: "Available Raw Materials", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5761, text: "3+ Points Available", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10180 },
        Text { x: 25, y: 40, id: 5762, text: "2  Points Available", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 60, id: 10170 },
        Text { x: 25, y: 60, id: 5763, text: "1  Points Available", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10169 },
        Text { x: 25, y: 80, id: 5764, text: "0  Points Available", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 100, id: 10182 },
        Bitmap { x: 7, y: 105, id: 10243 },
        Text { x: 25, y: 105, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 120, id: 10241 },
        Text { x: 25, y: 120, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 105, id: 10245 },
        Text { x: 105, y: 105, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 120, id: 10158 },
        Text { x: 105, y: 120, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 10190.
const MINES: Legend = Legend {
    width: 180,
    height: 135,
    items: &[
        Text { x: 69, y: 1, id: 5765, text: "Mines", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5766, text: "6+  Mines", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10180 },
        Text { x: 25, y: 40, id: 5767, text: "3-5 Mines", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 60, id: 10170 },
        Text { x: 25, y: 60, id: 5768, text: "1-2 Mines", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10169 },
        Text { x: 25, y: 80, id: 5769, text: "No  Mines", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 100, id: 10182 },
        Bitmap { x: 7, y: 105, id: 10243 },
        Text { x: 25, y: 105, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 120, id: 10241 },
        Text { x: 25, y: 120, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 105, id: 10245 },
        Text { x: 105, y: 105, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 120, id: 10158 },
        Text { x: 105, y: 120, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 10191.
const REFINERIES: Legend = Legend {
    width: 180,
    height: 135,
    items: &[
        Text { x: 58, y: 1, id: 5776, text: "Refineries", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5777, text: "6+  Refineries", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10180 },
        Text { x: 25, y: 40, id: 5778, text: "3-5 Refineries", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 60, id: 10170 },
        Text { x: 25, y: 60, id: 5779, text: "1-2 Refineries", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10169 },
        Text { x: 25, y: 80, id: 5780, text: "No  Refineries", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 100, id: 10182 },
        Bitmap { x: 7, y: 105, id: 10243 },
        Text { x: 25, y: 105, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 120, id: 10241 },
        Text { x: 25, y: 120, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 105, id: 10245 },
        Text { x: 105, y: 105, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 120, id: 10158 },
        Text { x: 105, y: 120, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 10193.
const SHIPYARDS: Legend = Legend {
    width: 180,
    height: 135,
    items: &[
        Text { x: 58, y: 1, id: 5784, text: "Shipyards", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5785, text: "5+  Shipyards", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10180 },
        Text { x: 25, y: 40, id: 5889, text: "2-4 Shipyards", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 60, id: 10170 },
        Text { x: 25, y: 60, id: 5890, text: "1   Shipyards", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10169 },
        Text { x: 25, y: 80, id: 5891, text: "No  Shipyards", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 100, id: 10182 },
        Bitmap { x: 7, y: 105, id: 10243 },
        Text { x: 25, y: 105, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 120, id: 10241 },
        Text { x: 25, y: 120, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 105, id: 10245 },
        Text { x: 105, y: 105, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 120, id: 10158 },
        Text { x: 105, y: 120, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 10194.
const TRAINING_FACILITIES: Legend = Legend {
    width: 180,
    height: 135,
    items: &[
        Text { x: 35, y: 1, id: 5892, text: "Training Facilities", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5893, text: "5+  Training Facilities", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10180 },
        Text { x: 25, y: 40, id: 5894, text: "2-4 Training Facilities", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 60, id: 10170 },
        Text { x: 25, y: 60, id: 5895, text: "1   Training Facilities", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10169 },
        Text { x: 25, y: 80, id: 5896, text: "No  Training Facilities", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 100, id: 10182 },
        Bitmap { x: 7, y: 105, id: 10243 },
        Text { x: 25, y: 105, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 120, id: 10241 },
        Text { x: 25, y: 120, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 105, id: 10245 },
        Text { x: 105, y: 105, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 120, id: 10158 },
        Text { x: 105, y: 120, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 10195.
const CONSTRUCTION_YARDS: Legend = Legend {
    width: 180,
    height: 135,
    items: &[
        Text { x: 29, y: 1, id: 5897, text: "Construction Yards", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5904, text: "5+  Construction Yards", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10180 },
        Text { x: 25, y: 40, id: 5905, text: "2-4 Construction Yards", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 60, id: 10170 },
        Text { x: 25, y: 60, id: 5906, text: "1   Construction Yards", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10169 },
        Text { x: 25, y: 80, id: 5907, text: "No  Construction Yards", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 100, id: 10182 },
        Bitmap { x: 7, y: 105, id: 10243 },
        Text { x: 25, y: 105, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 120, id: 10241 },
        Text { x: 25, y: 120, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 105, id: 10245 },
        Text { x: 105, y: 105, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 120, id: 10158 },
        Text { x: 105, y: 120, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 11605.
const IDLE_SHIPYARDS: Legend = Legend {
    width: 180,
    height: 95,
    items: &[
        Text { x: 47, y: 1, id: 5786, text: "Idle Shipyards", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5755, text: "Idle", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10169 },
        Text { x: 25, y: 40, id: 5758, text: "Active", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 60, id: 10182 },
        Bitmap { x: 7, y: 65, id: 10243 },
        Text { x: 25, y: 65, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10241 },
        Text { x: 25, y: 80, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 65, id: 10245 },
        Text { x: 105, y: 65, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 80, id: 10158 },
        Text { x: 105, y: 80, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 11606.
const IDLE_TRAINING_FACILITIES: Legend = Legend {
    width: 180,
    height: 95,
    items: &[
        Text { x: 23, y: 1, id: 5899, text: "Idle Training Facilities", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5755, text: "Idle", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10169 },
        Text { x: 25, y: 40, id: 5758, text: "Active", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 60, id: 10182 },
        Bitmap { x: 7, y: 65, id: 10243 },
        Text { x: 25, y: 65, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10241 },
        Text { x: 25, y: 80, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 65, id: 10245 },
        Text { x: 105, y: 65, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 80, id: 10158 },
        Text { x: 105, y: 80, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 11607.
const IDLE_CONSTRUCTION_YARDS: Legend = Legend {
    width: 180,
    height: 95,
    items: &[
        Text { x: 19, y: 1, id: 5898, text: "Idle Construction Yards", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5755, text: "Idle", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10169 },
        Text { x: 25, y: 40, id: 5758, text: "Active", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 60, id: 10182 },
        Bitmap { x: 7, y: 65, id: 10243 },
        Text { x: 25, y: 65, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10241 },
        Text { x: 25, y: 80, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 65, id: 10245 },
        Text { x: 105, y: 65, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 80, id: 10158 },
        Text { x: 105, y: 80, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 10196.
const TROOPERS: Legend = Legend {
    width: 180,
    height: 135,
    items: &[
        Text { x: 31, y: 1, id: 5908, text: "Trooper Regiments", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5909, text: "6+  Trooper Regiments", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10180 },
        Text { x: 25, y: 40, id: 5910, text: "3-5 Trooper Regiments", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 60, id: 10170 },
        Text { x: 25, y: 60, id: 5911, text: "1-2 Trooper Regiments", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10169 },
        Text { x: 25, y: 80, id: 5912, text: "No  Trooper Regiments", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 100, id: 10182 },
        Bitmap { x: 7, y: 105, id: 10243 },
        Text { x: 25, y: 105, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 120, id: 10241 },
        Text { x: 25, y: 120, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 105, id: 10245 },
        Text { x: 105, y: 105, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 120, id: 10158 },
        Text { x: 105, y: 120, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 10197.
const FIGHTER_SQUADRONS: Legend = Legend {
    width: 180,
    height: 135,
    items: &[
        Text { x: 34, y: 1, id: 5913, text: "Fighter Squadrons", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5920, text: "6+  Fighter Squadrons", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10180 },
        Text { x: 25, y: 40, id: 5921, text: "3-5 Fighter Squadrons", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 60, id: 10170 },
        Text { x: 25, y: 60, id: 5922, text: "1-2 Fighter Squadrons", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10169 },
        Text { x: 25, y: 80, id: 5923, text: "No  Fighter Squadrons", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 100, id: 10182 },
        Bitmap { x: 7, y: 105, id: 10243 },
        Text { x: 25, y: 105, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 120, id: 10241 },
        Text { x: 25, y: 120, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 105, id: 10245 },
        Text { x: 105, y: 105, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 120, id: 10158 },
        Text { x: 105, y: 120, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 10198.
const DEATH_STAR_SHIELDS: Legend = Legend {
    width: 180,
    height: 95,
    items: &[
        Text { x: 30, y: 1, id: 5924, text: "Death Star Shields", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5925, text: "Death Star Shield", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10169 },
        Text { x: 25, y: 40, id: 5926, text: "No Death Star Shield", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 60, id: 10182 },
        Bitmap { x: 7, y: 65, id: 10243 },
        Text { x: 25, y: 65, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10241 },
        Text { x: 25, y: 80, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 65, id: 10245 },
        Text { x: 105, y: 65, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 80, id: 10158 },
        Text { x: 105, y: 80, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 10199.
const PLANETARY_SHIELD_GENERATORS: Legend = Legend {
    width: 180,
    height: 135,
    items: &[
        Text { x: 35, y: 1, id: 5927, text: "Shield Generators", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5928, text: "6+  Generators", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10180 },
        Text { x: 25, y: 40, id: 5929, text: "3-5 Generators", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 60, id: 10170 },
        Text { x: 25, y: 60, id: 5936, text: "1-2 Generators", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10169 },
        Text { x: 25, y: 80, id: 5937, text: "No  Generators", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 100, id: 10182 },
        Bitmap { x: 7, y: 105, id: 10243 },
        Text { x: 25, y: 105, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 120, id: 10241 },
        Text { x: 25, y: 120, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 105, id: 10245 },
        Text { x: 105, y: 105, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 120, id: 10158 },
        Text { x: 105, y: 120, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};

/// STRATEGY `RT_RCDATA` 10200.
const PLANETARY_DEFENSE_BATTERIES: Legend = Legend {
    width: 180,
    height: 135,
    items: &[
        Text { x: 35, y: 1, id: 5938, text: "Defense Batteries", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 20, id: 10181 },
        Text { x: 25, y: 20, id: 5939, text: "6+  Batteries", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 40, id: 10180 },
        Text { x: 25, y: 40, id: 5940, text: "3-5 Batteries", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 60, id: 10170 },
        Text { x: 25, y: 60, id: 5941, text: "1-2 Batteries", font: 4, color: (255, 255, 255) },
        Bitmap { x: 7, y: 80, id: 10169 },
        Text { x: 25, y: 80, id: 5942, text: "No  Batteries", font: 4, color: (255, 255, 255) },
        Bitmap { x: 0, y: 100, id: 10182 },
        Bitmap { x: 7, y: 105, id: 10243 },
        Text { x: 25, y: 105, id: 5713, text: "Alliance", font: 10, color: (255, 255, 255) },
        Bitmap { x: 7, y: 120, id: 10241 },
        Text { x: 25, y: 120, id: 5712, text: "Empire", font: 10, color: (255, 255, 255) },
        Bitmap { x: 85, y: 105, id: 10245 },
        Text { x: 105, y: 105, id: 5714, text: "Neutral", font: 10, color: (255, 255, 255) },
        Bitmap { x: 82, y: 120, id: 10158 },
        Text { x: 105, y: 120, id: 5715, text: "Unexplored", font: 10, color: (255, 255, 255) },
    ],
};
