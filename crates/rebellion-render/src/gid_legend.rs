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

const fn bitmap(x: u16, y: u16, id: u32) -> LegendItem {
    Bitmap { x, y, id }
}

const fn white_text(
    x: u16,
    y: u16,
    id: u32,
    text: &'static str,
    font: u8,
) -> LegendItem {
    Text {
        x,
        y,
        id,
        text,
        font,
        color: (255, 255, 255),
    }
}

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
        // FUN_004522f0 has no script for the tour's displays.
        GidMode::DisplayOff
        | GidMode::LoyalToPlayer
        | GidMode::PlayerMilitaryControl
        | GidMode::LoyalToEnemy
        | GidMode::EnemyMilitaryControl
        | GidMode::UnexploredSystems
        | GidMode::AllDefenses
        | GidMode::HighlightEmpire
        | GidMode::HighlightAlliance => return None,
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
                egui::pos2(
                    rect.max.x - close_size.x - 3.0 * scale,
                    rect.min.y + 3.0 * scale,
                ),
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
            let at = |x: u16, y: u16| rect.min + egui::vec2(f32::from(x), f32::from(y)) * scale;
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

macro_rules! legend {
    ($name:ident, $height:expr, $footer_y:expr, $title:expr; $(($y:expr, $bitmap:expr, $id:expr, $text:expr)),+ $(,)?) => {
        const $name: Legend = Legend {
            width: 180,
            height: $height,
            items: &[
                $title,
                $(bitmap(7, $y, $bitmap), white_text(25, $y, $id, $text, 4),)+
                bitmap(0, $footer_y - 5, 10182),
                bitmap(7, $footer_y, 10243),
                white_text(25, $footer_y, 5713, "Alliance", 10),
                bitmap(7, $footer_y + 15, 10241),
                white_text(25, $footer_y + 15, 5712, "Empire", 10),
                bitmap(85, $footer_y, 10245),
                white_text(105, $footer_y, 5714, "Neutral", 10),
                bitmap(82, $footer_y + 15, 10158),
                white_text(105, $footer_y + 15, 5715, "Unexplored", 10),
            ],
        };
    };
}

/// STRATEGY `RT_RCDATA` 10183.
legend!(POPULAR_SUPPORT_ALLIANCE, 135, 105, white_text(21, 1, 5717, "Loyalty to the Alliance", 4); (20, 10181, 5719, "Loyal"), (40, 10180, 5720, "Obedient"), (60, 10170, 5721, "Disloyal"), (80, 10169, 5728, "Hostile"));
/// STRATEGY `RT_RCDATA` 10184.
legend!(POPULAR_SUPPORT_EMPIRE, 135, 105, white_text(21, 1, 5718, "Loyalty to the Empire", 4); (20, 10181, 5719, "Loyal"), (40, 10180, 5720, "Obedient"), (60, 10170, 5721, "Disloyal"), (80, 10169, 5728, "Hostile"));
/// STRATEGY `RT_RCDATA` 11600.
legend!(UPRISINGS, 95, 65, white_text(31, 1, 5722, "Worlds in Uprising", 4); (20, 10181, 5723, "Currently in Uprising"), (40, 10169, 5724, "Not in Uprising"));
/// STRATEGY `RT_RCDATA` 11601.
legend!(IDLE_FLEETS, 155, 125, white_text(58, 1, 5738, "Idle Fleets", 4); (20, 10181, 5730, "3+ fleets"), (40, 10180, 5731, "2  fleets"), (60, 10170, 5732, "1  fleet"), (80, 10169, 5733, "No fleets"), (100, 10160, 12582, "Conflict"));
/// STRATEGY `RT_RCDATA` 11602.
legend!(FLEETS_EN_ROUTE, 155, 125, white_text(45, 1, 5739, "Fleets Enroute", 4); (20, 10181, 5730, "3+ fleets"), (40, 10180, 5731, "2  fleets"), (60, 10170, 5732, "1  fleet"), (80, 10169, 5733, "No fleets"), (100, 10160, 12582, "Conflict"));
/// STRATEGY `RT_RCDATA` 11603.
legend!(IDLE_PERSONNEL, 95, 65, white_text(47, 1, 5754, "Idle Personnel", 4); (20, 10181, 5755, "Idle"), (40, 10169, 5756, "None"));
/// STRATEGY `RT_RCDATA` 11604.
legend!(ACTIVE_PERSONNEL, 95, 65, white_text(40, 1, 5757, "Active Personnel", 4); (20, 10181, 5758, "Active"), (40, 10169, 5759, "None"));
/// STRATEGY `RT_RCDATA` 10188.
legend!(AVAILABLE_ENERGY, 135, 105, white_text(38, 1, 5749, "Available Energy", 4); (20, 10181, 5750, "6+  Points Available"), (40, 10180, 5751, "3-5 Points Available"), (60, 10170, 5752, "1-2 Points Available"), (80, 10169, 5753, "0   Points Available"));
/// STRATEGY `RT_RCDATA` 10189.
legend!(AVAILABLE_RAW_MATERIAL, 135, 105, white_text(18, 1, 5760, "Available Raw Materials", 4); (20, 10181, 5761, "3+ Points Available"), (40, 10180, 5762, "2  Points Available"), (60, 10170, 5763, "1  Points Available"), (80, 10169, 5764, "0  Points Available"));
/// STRATEGY `RT_RCDATA` 10190.
legend!(MINES, 135, 105, white_text(69, 1, 5765, "Mines", 4); (20, 10181, 5766, "6+  Mines"), (40, 10180, 5767, "3-5 Mines"), (60, 10170, 5768, "1-2 Mines"), (80, 10169, 5769, "No  Mines"));
/// STRATEGY `RT_RCDATA` 10191.
legend!(REFINERIES, 135, 105, white_text(58, 1, 5776, "Refineries", 4); (20, 10181, 5777, "6+  Refineries"), (40, 10180, 5778, "3-5 Refineries"), (60, 10170, 5779, "1-2 Refineries"), (80, 10169, 5780, "No  Refineries"));
/// STRATEGY `RT_RCDATA` 10193.
legend!(SHIPYARDS, 135, 105, white_text(58, 1, 5784, "Shipyards", 4); (20, 10181, 5785, "5+  Shipyards"), (40, 10180, 5889, "2-4 Shipyards"), (60, 10170, 5890, "1   Shipyards"), (80, 10169, 5891, "No  Shipyards"));
/// STRATEGY `RT_RCDATA` 10194.
legend!(TRAINING_FACILITIES, 135, 105, white_text(35, 1, 5892, "Training Facilities", 4); (20, 10181, 5893, "5+  Training Facilities"), (40, 10180, 5894, "2-4 Training Facilities"), (60, 10170, 5895, "1   Training Facilities"), (80, 10169, 5896, "No  Training Facilities"));
/// STRATEGY `RT_RCDATA` 10195.
legend!(CONSTRUCTION_YARDS, 135, 105, white_text(29, 1, 5897, "Construction Yards", 4); (20, 10181, 5904, "5+  Construction Yards"), (40, 10180, 5905, "2-4 Construction Yards"), (60, 10170, 5906, "1   Construction Yards"), (80, 10169, 5907, "No  Construction Yards"));
/// STRATEGY `RT_RCDATA` 11605.
legend!(IDLE_SHIPYARDS, 95, 65, white_text(47, 1, 5786, "Idle Shipyards", 4); (20, 10181, 5755, "Idle"), (40, 10169, 5758, "Active"));
/// STRATEGY `RT_RCDATA` 11606.
legend!(IDLE_TRAINING_FACILITIES, 95, 65, white_text(23, 1, 5899, "Idle Training Facilities", 4); (20, 10181, 5755, "Idle"), (40, 10169, 5758, "Active"));
/// STRATEGY `RT_RCDATA` 11607.
legend!(IDLE_CONSTRUCTION_YARDS, 95, 65, white_text(19, 1, 5898, "Idle Construction Yards", 4); (20, 10181, 5755, "Idle"), (40, 10169, 5758, "Active"));
/// STRATEGY `RT_RCDATA` 10196.
legend!(TROOPERS, 135, 105, white_text(31, 1, 5908, "Trooper Regiments", 4); (20, 10181, 5909, "6+  Trooper Regiments"), (40, 10180, 5910, "3-5 Trooper Regiments"), (60, 10170, 5911, "1-2 Trooper Regiments"), (80, 10169, 5912, "No  Trooper Regiments"));
/// STRATEGY `RT_RCDATA` 10197.
legend!(FIGHTER_SQUADRONS, 135, 105, white_text(34, 1, 5913, "Fighter Squadrons", 4); (20, 10181, 5920, "6+  Fighter Squadrons"), (40, 10180, 5921, "3-5 Fighter Squadrons"), (60, 10170, 5922, "1-2 Fighter Squadrons"), (80, 10169, 5923, "No  Fighter Squadrons"));
/// STRATEGY `RT_RCDATA` 10198.
legend!(DEATH_STAR_SHIELDS, 95, 65, white_text(30, 1, 5924, "Death Star Shields", 4); (20, 10181, 5925, "Death Star Shield"), (40, 10169, 5926, "No Death Star Shield"));
/// STRATEGY `RT_RCDATA` 10199.
legend!(PLANETARY_SHIELD_GENERATORS, 135, 105, white_text(35, 1, 5927, "Shield Generators", 4); (20, 10181, 5928, "6+  Generators"), (40, 10180, 5929, "3-5 Generators"), (60, 10170, 5936, "1-2 Generators"), (80, 10169, 5937, "No  Generators"));
/// STRATEGY `RT_RCDATA` 10200.
legend!(PLANETARY_DEFENSE_BATTERIES, 135, 105, white_text(35, 1, 5938, "Defense Batteries", 4); (20, 10181, 5939, "6+  Batteries"), (40, 10180, 5940, "3-5 Batteries"), (60, 10170, 5941, "1-2 Batteries"), (80, 10169, 5942, "No  Batteries"));
