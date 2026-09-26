//! Original Game Speed control: the day readout and its right-click menu.
//!
//! The command center has no speed buttons. `FUN_00422ce0` builds a numeric
//! day readout in the top strip, and a right-button release inside its
//! rectangle calls `FUN_0042d190`. That routine opens a five-item Game Menu
//! Window (`FUN_00442590` / `FUN_00442860`) whose records are STRATEGY
//! `RT_RCDATA` 0x20..0x24 (Alliance) or 0x25..0x29 (Empire). A selection
//! reaches `FUN_00486fb0`, which pauses or calls `FUN_00487eb0` with the speed.
//!
//! Pause does not change the speed. `FUN_0041d2f0` sets a stop at the next
//! day and `FUN_00415e60` opens the REBDLOG "Resume Game Play?" alert; only
//! that alert's checkmark or Enter (`FUN_004011a0`) lifts the stop, and play
//! continues at whatever speed is selected.
//!
//! Recovery notes: `docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-24-game-speed-recovery.md`.

use egui_macroquad::egui;
use rebellion_core::tick::{GameClock, GameSpeed};

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{
    gid_popup_frame, logical_rect_to_screen, paint_gid_frame_border, CockpitFaction, CockpitLayout,
    CockpitViewport, STRATEGIC_LOGICAL_HEIGHT, STRATEGIC_LOGICAL_WIDTH,
};

/// Game-font entry 10 (`FUN_0060eed0`): 14-pixel Arial, normal weight.
///
/// Arial is not redistributable in the browser build, so text uses egui's
/// proportional face at the recovered height.
const DAY_FONT_HEIGHT: f32 = 14.0;

/// Menu text height. The Game Menu Window uses the application default font
/// (`FUN_00603850`), whose table entry is not yet recovered.
const MENU_FONT_HEIGHT: f32 = 14.0;

/// One record of the original speed menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameSpeedMenuItem {
    /// STRATEGY `RT_RCDATA` record ID and `FUN_00486fb0` command.
    pub item_id: u16,
    pub speed: GameSpeed,
    /// TEXTSTRA string named by record word 5.
    pub label_string_id: u16,
    pub label: &'static str,
    /// 16x10 STRATEGY icon named by record words 7 and 8.
    pub icon_resource: u32,
}

const fn item(
    item_id: u16,
    speed: GameSpeed,
    label_string_id: u16,
    label: &'static str,
    icon_resource: u32,
) -> GameSpeedMenuItem {
    GameSpeedMenuItem {
        item_id,
        speed,
        label_string_id,
        label,
        icon_resource,
    }
}

const ALLIANCE_ITEMS: [GameSpeedMenuItem; 5] = [
    item(0x20, GameSpeed::Paused, 34304, "Pause", 11580),
    item(0x21, GameSpeed::VerySlow, 34305, "Very Slow", 11581),
    item(0x22, GameSpeed::Slow, 34306, "Slow", 11582),
    item(0x23, GameSpeed::Medium, 34307, "Medium", 11583),
    item(0x24, GameSpeed::Fast, 34308, "Fast", 11588),
];

const EMPIRE_ITEMS: [GameSpeedMenuItem; 5] = [
    item(0x25, GameSpeed::Paused, 34304, "Pause", 11584),
    item(0x26, GameSpeed::VerySlow, 34305, "Very Slow", 11585),
    item(0x27, GameSpeed::Slow, 34306, "Slow", 11586),
    item(0x28, GameSpeed::Medium, 34307, "Medium", 11587),
    item(0x29, GameSpeed::Fast, 34308, "Fast", 11589),
];

/// Speed menu records in original order for a faction.
#[must_use]
pub fn game_speed_menu_items(faction: CockpitFaction) -> &'static [GameSpeedMenuItem; 5] {
    match faction {
        CockpitFaction::Alliance => &ALLIANCE_ITEMS,
        CockpitFaction::Empire => &EMPIRE_ITEMS,
    }
}

/// Right-click rectangle of the day readout in the 640x480 canvas.
///
/// `FUN_00422ce0` builds `(103,21)-(185,33)` for the Alliance and
/// `(499,19)-(580,31)` for the Empire; right and bottom are exclusive.
#[must_use]
pub const fn day_readout_rect(faction: CockpitFaction) -> CockpitViewport {
    match faction {
        CockpitFaction::Alliance => CockpitViewport {
            x: 103.0,
            y: 21.0,
            width: 82.0,
            height: 12.0,
        },
        CockpitFaction::Empire => CockpitViewport {
            x: 499.0,
            y: 19.0,
            width: 81.0,
            height: 12.0,
        },
    }
}

/// Text anchor `FUN_00601b30` assigns to the day readout.
#[must_use]
pub const fn day_text_origin(faction: CockpitFaction) -> (f32, f32) {
    match faction {
        CockpitFaction::Alliance => (104.0, 18.0),
        CockpitFaction::Empire => (500.0, 18.0),
    }
}

/// Faction text color: Alliance `0x20000ff` (red), Empire `0x200ff00` (green).
#[must_use]
pub const fn faction_text_color(faction: CockpitFaction) -> egui::Color32 {
    match faction {
        CockpitFaction::Alliance => egui::Color32::from_rgb(255, 0, 0),
        CockpitFaction::Empire => egui::Color32::from_rgb(0, 255, 0),
    }
}

/// Highlight color `0x2ffffff` that `FUN_0042d190` passes to every item.
const MENU_HIGHLIGHT_COLOR: egui::Color32 = egui::Color32::WHITE;

/// Row geometry of a Game Menu Window, in logical pixels.
#[derive(Debug, Clone, PartialEq)]
pub struct GameMenuGeometry {
    /// Widest item icon (`FUN_004abf60`), shared by every row.
    pub icon_column: f32,
    pub width: f32,
    pub height: f32,
    /// Top edge and height of each row.
    pub rows: Vec<(f32, f32)>,
}

/// Lay out rows as `FUN_00442a80` and `FUN_004abb80` / `FUN_004abbf0` /
/// `FUN_004abc70` do.
///
/// Each row is `icon_column + 12 + text` wide and `max(icon, text) + 4` tall.
/// Rows start two pixels down, and the window adds two pixels below the sum.
#[must_use]
pub fn game_menu_geometry(
    icon_sizes: &[(f32, f32)],
    text_sizes: &[(f32, f32)],
) -> GameMenuGeometry {
    let icon_column = icon_sizes.iter().map(|size| size.0).fold(0.0, f32::max);
    let mut width = 0.0_f32;
    let mut rows = Vec::with_capacity(text_sizes.len());
    let mut top = 2.0;
    for (index, text) in text_sizes.iter().enumerate() {
        let icon_height = icon_sizes.get(index).map_or(0.0, |size| size.1);
        width = width.max(icon_column + 12.0 + text.0);
        let height = (icon_height + 4.0).max(text.1 + 4.0);
        rows.push((top, height));
        top += height;
    }
    let height = rows.iter().map(|row| row.1).sum::<f32>() + 2.0;
    GameMenuGeometry {
        icon_column,
        width,
        height,
        rows,
    }
}

/// Place a menu at the cursor, flipping as `FUN_00442860` does when it would
/// cross the owner window's right or bottom edge.
#[must_use]
pub fn game_menu_origin(cursor: (f32, f32), size: (f32, f32), owner: (f32, f32)) -> (f32, f32) {
    let (mut x, mut y) = cursor;
    if y + size.1 > owner.1 {
        y -= size.1;
    }
    if x < 0.0 {
        x += size.0;
    } else if x + size.0 > owner.0 {
        x -= size.0;
    }
    (x, y)
}

/// Speed menu state owned by the command center.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct GameSpeedUiState {
    /// Logical cursor position of the right-click that opened the menu.
    pub menu_anchor: Option<(f32, f32)>,
}

/// Apply a Game Speed choice from the menu, keys, or palette.
///
/// Pause sets the clock's stop day, which opens the alert. Pausing again
/// while paused does nothing, as the Alt+P guard in `FUN_00422ce0` does;
/// otherwise the stop would move to the day after the held one. A running
/// speed only changes the rate, even while paused, because `FUN_00487eb0`
/// does not lift the stop.
pub fn choose_game_speed(clock: &mut GameClock, requested: GameSpeed) {
    match requested {
        GameSpeed::Paused if clock.pause_requested() => {}
        GameSpeed::Paused => {
            clock.pause();
        }
        running => clock.set_speed(running),
    }
}

/// The speed an Alt+NumPad step selects, or `None` while paused, when
/// `FUN_00422ce0` ignores the keys.
#[must_use]
pub fn stepped_game_speed(clock: &GameClock, faster: bool) -> Option<GameSpeed> {
    if clock.pause_requested() || clock.speed == GameSpeed::Paused {
        return None;
    }
    Some(if faster {
        clock.speed.faster()
    } else {
        clock.speed.slower()
    })
}

/// REBDLOG one-button alert background, 412x176 (`FUN_00417020`).
const ALERT_BACKGROUND: u32 = 0x297e;
/// Checkmark button: normal and pressed (`FUN_00417150`).
const ALERT_RESUME_NORMAL: u32 = 0x2981;
const ALERT_RESUME_PRESSED: u32 = 0x2980;
const ALERT_WIDTH: f32 = 412.0;
const ALERT_HEIGHT: f32 = 176.0;
/// Message rectangle `(43,25)-(368,112)` read from `0x00658920`.
const ALERT_TEXT_RECT: CockpitViewport = CockpitViewport {
    x: 43.0,
    y: 25.0,
    width: 325.0,
    height: 87.0,
};
/// Checkmark button at `(176,134)`, sized by its 57x28 bitmap.
const ALERT_BUTTON_RECT: CockpitViewport = CockpitViewport {
    x: 176.0,
    y: 134.0,
    width: 57.0,
    height: 28.0,
};
/// Game-font entry 5: 16 pixels, bold. Drawn with egui's proportional face.
const ALERT_FONT_HEIGHT: f32 = 16.0;
/// Text color `0x2f0fbff`.
const ALERT_TEXT_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 251, 240);
/// REBDLOG string 4611.
pub const RESUME_GAME_PLAY: &str = "Resume Game Play?";

/// Alert origin in the 640x480 canvas. `FUN_005ffeb0` centers the window in
/// its owner with integer division.
#[must_use]
pub const fn pause_alert_origin() -> (f32, f32) {
    (
        (STRATEGIC_LOGICAL_WIDTH as i32 / 2 - ALERT_WIDTH as i32 / 2) as f32,
        (STRATEGIC_LOGICAL_HEIGHT as i32 / 2 - ALERT_HEIGHT as i32 / 2) as f32,
    )
}

/// Whether a screen point lies on the open pause alert. The alert is a child
/// window, so the galaxy map beneath it must not receive the click.
#[must_use]
pub fn pause_alert_contains_screen_point(
    clock: &GameClock,
    layout: CockpitLayout,
    point: (f32, f32),
) -> bool {
    if !clock.pause_requested() {
        return false;
    }
    let (x, y) = pause_alert_origin();
    let left = layout.canvas.x + x * layout.scale;
    let top = layout.canvas.y + y * layout.scale;
    point.0 >= left
        && point.0 < left + ALERT_WIDTH * layout.scale
        && point.1 >= top
        && point.1 < top + ALERT_HEIGHT * layout.scale
}

/// Draw the "Resume Game Play?" alert while a pause is pending or holding.
///
/// Returns true when the checkmark is clicked or Enter is pressed; the caller
/// then resumes the clock. The original is a child window, not a modal
/// dialog, so the rest of the command center stays usable.
pub fn draw_pause_alert(
    ctx: &egui::Context,
    clock: &GameClock,
    cache: &mut BmpCache,
    layout: CockpitLayout,
    input_enabled: bool,
) -> bool {
    if !clock.pause_requested() || layout.scale <= 0.0 {
        return false;
    }
    let scale = layout.scale;
    let (x, y) = pause_alert_origin();
    let origin = egui::pos2(layout.canvas.x + x * scale, layout.canvas.y + y * scale);
    let local = |viewport: CockpitViewport| {
        egui::Rect::from_min_size(
            origin + egui::vec2(viewport.x * scale, viewport.y * scale),
            egui::vec2(viewport.width * scale, viewport.height * scale),
        )
    };
    let full_uv = egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0));

    let clicked = egui::Area::new(egui::Id::new("original_pause_alert"))
        .order(egui::Order::Foreground)
        .fade_in(false)
        .fixed_pos(origin)
        .show(ctx, |ui| {
            if !input_enabled {
                ui.disable();
            }
            let (rect, _) = ui.allocate_exact_size(
                egui::vec2(ALERT_WIDTH * scale, ALERT_HEIGHT * scale),
                egui::Sense::hover(),
            );
            match cache.get(ctx, DllSource::Rebdlog, ALERT_BACKGROUND) {
                Some(texture) => {
                    ui.painter()
                        .image(texture.id(), rect, full_uv, egui::Color32::WHITE);
                }
                None => {
                    ui.painter().rect_filled(rect, 0.0, gid_popup_frame().fill);
                }
            }
            ui.painter().text(
                local(ALERT_TEXT_RECT).center(),
                egui::Align2::CENTER_CENTER,
                RESUME_GAME_PLAY,
                egui::FontId::proportional(ALERT_FONT_HEIGHT * scale),
                ALERT_TEXT_COLOR,
            );
            let button = local(ALERT_BUTTON_RECT);
            let response = ui.interact(
                button,
                ui.id().with("pause_alert_resume"),
                egui::Sense::click(),
            );
            let resource = if response.is_pointer_button_down_on() {
                ALERT_RESUME_PRESSED
            } else {
                ALERT_RESUME_NORMAL
            };
            if let Some(texture) = cache.get(ctx, DllSource::Rebdlog, resource) {
                ui.painter()
                    .image(texture.id(), button, full_uv, egui::Color32::WHITE);
            }
            response.clicked()
        })
        .inner;

    // A focused text field, such as the command palette's, owns Enter.
    let enter = input_enabled
        && !ctx.wants_keyboard_input()
        && ctx.input(|input| input.key_pressed(egui::Key::Enter));
    clicked || enter
}

/// Paint the day readout as `FUN_00601ce0` draws it: centered, transparent,
/// in the faction text color.
pub fn draw_day_readout(
    ctx: &egui::Context,
    layout: CockpitLayout,
    faction: CockpitFaction,
    day: u64,
) {
    let (x, y) = day_text_origin(faction);
    let hit = day_readout_rect(faction);
    let center_x = x + (hit.x + hit.width - x) / 2.0;
    ctx.layer_painter(egui::LayerId::background()).text(
        egui::pos2(
            layout.canvas.x + center_x * layout.scale,
            layout.canvas.y + y * layout.scale,
        ),
        egui::Align2::CENTER_TOP,
        day.to_string(),
        egui::FontId::proportional(DAY_FONT_HEIGHT * layout.scale),
        faction_text_color(faction),
    );
}

/// Open the menu on a right-button release inside the day readout.
///
/// Returns true when the menu opened this frame.
pub fn open_game_speed_menu_on_right_click(
    ctx: &egui::Context,
    ui_state: &mut GameSpeedUiState,
    layout: CockpitLayout,
    faction: CockpitFaction,
) -> bool {
    if layout.scale <= 0.0 || ctx.is_pointer_over_area() {
        return false;
    }
    let (released, pointer) = ctx.input(|input| {
        (
            input
                .pointer
                .button_released(egui::PointerButton::Secondary),
            input.pointer.interact_pos(),
        )
    });
    let Some(pointer) = pointer.filter(|_| released) else {
        return false;
    };
    let hit = logical_rect_to_screen(layout, day_readout_rect(faction));
    if !hit.contains(pointer) {
        return false;
    }
    ui_state.menu_anchor = Some((
        (pointer.x - layout.canvas.x) / layout.scale,
        (pointer.y - layout.canvas.y) / layout.scale,
    ));
    true
}

/// Draw the open speed menu and return the chosen speed.
///
/// A choice, an outside press, or Escape closes the menu.
pub fn draw_game_speed_menu(
    ctx: &egui::Context,
    ui_state: &mut GameSpeedUiState,
    cache: &mut BmpCache,
    layout: CockpitLayout,
    faction: CockpitFaction,
    input_enabled: bool,
) -> Option<GameSpeed> {
    let anchor = ui_state.menu_anchor?;
    let scale = layout.scale.max(0.5);
    let items = game_speed_menu_items(faction);
    let font = egui::FontId::proportional(MENU_FONT_HEIGHT * scale);
    let text_sizes: Vec<(f32, f32)> = items
        .iter()
        .map(|item| {
            let size = ctx
                .fonts(|fonts| {
                    fonts.layout_no_wrap(item.label.to_owned(), font.clone(), egui::Color32::WHITE)
                })
                .size();
            (size.x / scale, size.y / scale)
        })
        .collect();
    let icon_sizes: Vec<(f32, f32)> = items
        .iter()
        .map(|item| {
            cache
                .get(ctx, DllSource::Strategy, item.icon_resource)
                .map_or((16.0, 10.0), |texture| {
                    let size = texture.size_vec2();
                    (size.x, size.y)
                })
        })
        .collect();
    let geometry = game_menu_geometry(&icon_sizes, &text_sizes);
    let (x, y) = game_menu_origin(
        anchor,
        (geometry.width, geometry.height),
        (STRATEGIC_LOGICAL_WIDTH, STRATEGIC_LOGICAL_HEIGHT),
    );
    let origin = egui::pos2(
        layout.canvas.x + x * layout.scale,
        layout.canvas.y + y * layout.scale,
    );
    let menu_rect = egui::Rect::from_min_size(
        origin,
        egui::vec2(geometry.width * scale, geometry.height * scale),
    );

    let chosen = egui::Area::new(egui::Id::new("original_game_speed_menu"))
        .order(egui::Order::Foreground)
        .fade_in(false)
        .fixed_pos(origin)
        .show(ctx, |ui| {
            if !input_enabled {
                ui.disable();
            }
            let (rect, _) = ui.allocate_exact_size(menu_rect.size(), egui::Sense::hover());
            ui.painter().rect_filled(rect, 0.0, gid_popup_frame().fill);
            let mut chosen = None;
            for ((item, (top, height)), text) in items
                .iter()
                .zip(geometry.rows.iter().copied())
                .zip(text_sizes.iter().copied())
            {
                let row = egui::Rect::from_min_size(
                    rect.min + egui::vec2(0.0, top * scale),
                    egui::vec2(rect.width(), height * scale),
                );
                let response = ui.interact(
                    row,
                    ui.id().with(("game_speed_item", item.item_id)),
                    egui::Sense::click(),
                );
                if let Some(texture) = cache.get(ctx, DllSource::Strategy, item.icon_resource) {
                    let size = texture.size_vec2() * scale;
                    ui.painter().image(
                        texture.id(),
                        egui::Rect::from_min_size(row.min + egui::vec2(6.0 * scale, 0.0), size),
                        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                }
                let color = if response.hovered() {
                    MENU_HIGHLIGHT_COLOR
                } else {
                    faction_text_color(faction)
                };
                ui.painter().text(
                    egui::pos2(
                        row.min.x + (geometry.icon_column + 6.0) * scale,
                        row.min.y + ((height - text.1) / 2.0) * scale,
                    ),
                    egui::Align2::LEFT_TOP,
                    item.label,
                    font.clone(),
                    color,
                );
                if response.clicked() {
                    chosen = Some(item.speed);
                }
            }
            paint_gid_frame_border(ui, cache, rect, scale);
            chosen
        });

    let dismiss = input_enabled
        && chosen.inner.is_none()
        && ctx.input(|input| {
            input.key_pressed(egui::Key::Escape)
                || (input.pointer.any_pressed()
                    && input
                        .pointer
                        .interact_pos()
                        .is_some_and(|pointer| !chosen.response.rect.contains(pointer)))
        });
    if chosen.inner.is_some() || dismiss {
        ui_state.menu_anchor = None;
    }
    chosen.inner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_records_match_strategy_rcdata() {
        let alliance = game_speed_menu_items(CockpitFaction::Alliance);
        let empire = game_speed_menu_items(CockpitFaction::Empire);
        assert_eq!(
            alliance.map(|item| item.item_id),
            [0x20, 0x21, 0x22, 0x23, 0x24]
        );
        assert_eq!(
            empire.map(|item| item.item_id),
            [0x25, 0x26, 0x27, 0x28, 0x29]
        );
        assert_eq!(
            alliance.map(|item| item.icon_resource),
            [11580, 11581, 11582, 11583, 11588]
        );
        assert_eq!(
            empire.map(|item| item.icon_resource),
            [11584, 11585, 11586, 11587, 11589]
        );
        for items in [alliance, empire] {
            assert_eq!(items.map(|item| item.speed), GameSpeed::ALL);
            assert_eq!(
                items.map(|item| item.label_string_id),
                [0x8600, 0x8601, 0x8602, 0x8603, 0x8604]
            );
            assert_eq!(
                items.map(|item| item.label),
                ["Pause", "Very Slow", "Slow", "Medium", "Fast"]
            );
        }
    }

    #[test]
    fn day_readout_uses_recovered_rectangles_and_anchors() {
        let alliance = day_readout_rect(CockpitFaction::Alliance);
        assert_eq!(
            (alliance.x, alliance.y, alliance.right(), alliance.bottom()),
            (103.0, 21.0, 185.0, 33.0)
        );
        let empire = day_readout_rect(CockpitFaction::Empire);
        assert_eq!(
            (empire.x, empire.y, empire.right(), empire.bottom()),
            (499.0, 19.0, 580.0, 31.0)
        );
        assert_eq!(day_text_origin(CockpitFaction::Alliance), (104.0, 18.0));
        assert_eq!(day_text_origin(CockpitFaction::Empire), (500.0, 18.0));
        assert_eq!(
            faction_text_color(CockpitFaction::Alliance),
            egui::Color32::from_rgb(255, 0, 0)
        );
        assert_eq!(
            faction_text_color(CockpitFaction::Empire),
            egui::Color32::from_rgb(0, 255, 0)
        );
    }

    #[test]
    fn menu_rows_follow_native_width_and_height_rules() {
        let icons = [(16.0, 10.0); 5];
        let texts = [
            (30.0, 16.0),
            (55.0, 16.0),
            (24.0, 16.0),
            (44.0, 16.0),
            (26.0, 16.0),
        ];
        let geometry = game_menu_geometry(&icons, &texts);
        assert_eq!(geometry.icon_column, 16.0);
        // Widest row: 16 + 12 + 55.
        assert_eq!(geometry.width, 83.0);
        assert_eq!(
            geometry.rows,
            vec![
                (2.0, 20.0),
                (22.0, 20.0),
                (42.0, 20.0),
                (62.0, 20.0),
                (82.0, 20.0)
            ]
        );
        assert_eq!(geometry.height, 102.0);
    }

    #[test]
    fn short_text_rows_keep_icon_height_plus_padding() {
        let geometry = game_menu_geometry(&[(16.0, 10.0)], &[(10.0, 8.0)]);
        assert_eq!(geometry.rows, vec![(2.0, 14.0)]);
    }

    fn running(speed: GameSpeed) -> GameClock {
        let mut clock = GameClock::new();
        clock.set_speed(speed);
        clock
    }

    #[test]
    fn pause_keeps_the_running_speed_for_resume() {
        let mut clock = running(GameSpeed::Slow);
        choose_game_speed(&mut clock, GameSpeed::Paused);
        assert!(clock.pause_requested());
        assert_eq!(clock.speed, GameSpeed::Slow);
        clock.resume();
        assert!(!clock.pause_requested());
    }

    #[test]
    fn pausing_again_while_held_runs_no_further_day() {
        let mut clock = running(GameSpeed::Medium);
        choose_game_speed(&mut clock, GameSpeed::Paused);
        clock.advance(5.0);
        assert_eq!(clock.tick, 1);
        choose_game_speed(&mut clock, GameSpeed::Paused);
        assert!(clock.advance(5.0).is_empty());
        assert_eq!(clock.tick, 1);
    }

    #[test]
    fn speed_choice_while_paused_waits_for_the_alert() {
        let mut clock = running(GameSpeed::Medium);
        choose_game_speed(&mut clock, GameSpeed::Paused);
        choose_game_speed(&mut clock, GameSpeed::Fast);
        choose_game_speed(&mut clock, GameSpeed::Paused);
        assert!(clock.pause_requested());
        assert_eq!(clock.speed, GameSpeed::Fast);
    }

    #[test]
    fn key_steps_do_nothing_while_paused() {
        let mut clock = running(GameSpeed::Medium);
        assert_eq!(stepped_game_speed(&clock, true), Some(GameSpeed::Fast));
        assert_eq!(stepped_game_speed(&clock, false), Some(GameSpeed::Slow));
        clock.pause();
        assert_eq!(stepped_game_speed(&clock, true), None);
        assert_eq!(stepped_game_speed(&clock, false), None);
        assert_eq!(stepped_game_speed(&GameClock::new(), true), None);
    }

    #[test]
    fn a_clock_paused_without_the_alert_runs_at_the_chosen_speed() {
        // A new game starts at Paused speed without an original pause.
        let mut clock = GameClock::new();
        choose_game_speed(&mut clock, GameSpeed::Paused);
        assert!(!clock.pause_requested());
        choose_game_speed(&mut clock, GameSpeed::Slow);
        assert_eq!(clock.speed, GameSpeed::Slow);
    }

    #[test]
    fn pause_alert_owns_its_pixels_only_while_open() {
        let layout = CockpitLayout {
            canvas: CockpitViewport {
                x: 100.0,
                y: 0.0,
                width: 1280.0,
                height: 960.0,
            },
            galaxy: CockpitViewport {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            scale: 2.0,
        };
        let mut clock = running(GameSpeed::Medium);
        let checkmark = (100.0 + 318.0 * 2.0, 300.0 * 2.0);
        assert!(!pause_alert_contains_screen_point(
            &clock, layout, checkmark
        ));
        clock.pause();
        assert!(pause_alert_contains_screen_point(&clock, layout, checkmark));
        assert!(!pause_alert_contains_screen_point(
            &clock,
            layout,
            (100.0 + 113.0 * 2.0, 300.0 * 2.0)
        ));
        assert!(!pause_alert_contains_screen_point(
            &clock,
            layout,
            (100.0 + 526.0 * 2.0, 300.0 * 2.0)
        ));
    }

    #[test]
    fn pause_alert_is_centered_in_the_owner() {
        assert_eq!(pause_alert_origin(), (114.0, 152.0));
        assert_eq!(
            (ALERT_TEXT_RECT.right(), ALERT_TEXT_RECT.bottom()),
            (368.0, 112.0)
        );
    }

    #[test]
    fn menu_touching_an_owner_edge_does_not_flip() {
        // FUN_00442860 flips only when the menu strictly crosses the edge.
        let owner = (640.0, 480.0);
        assert_eq!(
            game_menu_origin((557.0, 378.0), (83.0, 102.0), owner),
            (557.0, 378.0)
        );
        assert_eq!(
            game_menu_origin((558.0, 379.0), (83.0, 102.0), owner),
            (475.0, 277.0)
        );
    }

    #[test]
    fn menu_left_of_the_owner_moves_right_by_its_width() {
        // FUN_00442860: a negative x gains the menu width; zero stays put.
        let owner = (640.0, 480.0);
        assert_eq!(
            game_menu_origin((-10.0, 26.0), (83.0, 102.0), owner),
            (73.0, 26.0)
        );
        assert_eq!(
            game_menu_origin((0.0, 26.0), (83.0, 102.0), owner),
            (0.0, 26.0)
        );
    }

    #[test]
    fn pause_alert_hit_area_matches_its_scaled_rectangle() {
        let layout = CockpitLayout {
            canvas: CockpitViewport {
                x: 100.0,
                y: 40.0,
                width: 1280.0,
                height: 960.0,
            },
            galaxy: CockpitViewport {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            scale: 2.0,
        };
        let mut clock = running(GameSpeed::Slow);
        clock.pause();
        let (left, top) = (100.0 + 114.0 * 2.0, 40.0 + 152.0 * 2.0);
        let (right, bottom) = (left + 412.0 * 2.0, top + 176.0 * 2.0);
        let hit = |x, y| pause_alert_contains_screen_point(&clock, layout, (x, y));
        assert!(hit(left, top));
        assert!(!hit(left - 0.5, top));
        assert!(!hit(left, top - 0.5));
        assert!(hit(right - 0.5, bottom - 0.5));
        assert!(!hit(right, top));
        assert!(!hit(left, bottom));
    }

    #[test]
    fn menu_flips_away_from_right_and_bottom_owner_edges() {
        let owner = (640.0, 480.0);
        assert_eq!(
            game_menu_origin((120.0, 26.0), (83.0, 102.0), owner),
            (120.0, 26.0)
        );
        assert_eq!(
            game_menu_origin((600.0, 26.0), (83.0, 102.0), owner),
            (517.0, 26.0)
        );
        assert_eq!(
            game_menu_origin((120.0, 420.0), (83.0, 102.0), owner),
            (120.0, 318.0)
        );
    }
}
