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
    gid_popup_frame, logical_rect_to_screen, CockpitFaction, CockpitLayout, CockpitViewport,
    STRATEGIC_LOGICAL_HEIGHT, STRATEGIC_LOGICAL_WIDTH,
};
use crate::game_menu::{
    draw_game_menu, faction_text_color, GameMenuEntry, GameMenuPlacement, GameMenuResponse,
};

/// Game-font entry 10 (`FUN_0060eed0`): 14-pixel Arial, normal weight.
///
/// Arial is not redistributable in the browser build, so text uses egui's
/// proportional face at the recovered height.
const DAY_FONT_HEIGHT: f32 = 14.0;

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

/// Hold or release the speed for the Message Index's Advice category, as
/// `FUN_00487ff0` does: showing it saves the speed once and drops to Very
/// Slow, and hiding it restores a saved speed. A second show while one is
/// saved, or a hide with none saved, changes nothing.
pub fn hold_speed_for_advice(clock: &mut GameClock, saved: &mut Option<GameSpeed>, shown: bool) {
    match (*saved, shown) {
        (Some(speed), false) => {
            choose_game_speed(clock, speed);
            *saved = None;
        }
        (None, true) => {
            *saved = Some(clock.speed);
            choose_game_speed(clock, GameSpeed::VerySlow);
        }
        _ => {}
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
const SPEED_MENU_ID: &str = "original_game_speed_menu";
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
    // The rectangle's right and bottom edges are exclusive, as Win32's
    // `PtInRect` treats them; egui's `Rect::contains` includes them.
    let hit = logical_rect_to_screen(layout, day_readout_rect(faction));
    if !(hit.x_range().min..hit.x_range().max).contains(&pointer.x)
        || !(hit.y_range().min..hit.y_range().max).contains(&pointer.y)
    {
        return false;
    }
    ui_state.menu_anchor = Some((
        (pointer.x - layout.canvas.x) / layout.scale,
        (pointer.y - layout.canvas.y) / layout.scale,
    ));
    true
}

/// Where the open speed menu lies on screen.
#[must_use]
pub fn game_speed_menu_rect(ctx: &egui::Context) -> Option<egui::Rect> {
    ctx.memory(|memory| memory.area_rect(egui::Id::new(SPEED_MENU_ID)))
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
    let items = game_speed_menu_items(faction);
    let entries = items.map(|item| GameMenuEntry {
        label: item.label,
        icon: Some(item.icon_resource),
        enabled: true,
        submenu: false,
    });
    match draw_game_menu(
        ctx,
        egui::Id::new(SPEED_MENU_ID),
        cache,
        layout,
        faction,
        GameMenuPlacement::in_frame(anchor),
        &entries,
        input_enabled,
    ) {
        GameMenuResponse::Open => None,
        GameMenuResponse::Dismissed => {
            ui_state.menu_anchor = None;
            None
        }
        GameMenuResponse::Chosen(index) => {
            ui_state.menu_anchor = None;
            Some(items[index].speed)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_records_match_strategy_rcdata() {
        // Source: STRATEGY.DLL RCDATA records via FUN_0042d190 (ghidra/notes/FUN_0042d190.c).
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
        // Source: FUN_00422ce0 day readout rects and FUN_00601b30 text anchor (ghidra/notes/).
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

    fn running(speed: GameSpeed) -> GameClock {
        let mut clock = GameClock::new();
        clock.set_speed(speed);
        clock
    }

    #[test]
    fn advice_drops_to_very_slow_once_and_hiding_it_restores_the_speed() {
        // FUN_00487ff0: +0x58 saves +0x54 only when empty, then speed 1;
        // a hide restores +0x58 and clears it.
        let mut clock = running(GameSpeed::Fast);
        let mut saved = None;
        hold_speed_for_advice(&mut clock, &mut saved, true);
        assert_eq!(clock.speed, GameSpeed::VerySlow);
        clock.set_speed(GameSpeed::Medium);
        hold_speed_for_advice(&mut clock, &mut saved, true);
        assert_eq!(
            clock.speed,
            GameSpeed::Medium,
            "a second show saves nothing"
        );
        hold_speed_for_advice(&mut clock, &mut saved, false);
        assert_eq!(clock.speed, GameSpeed::Fast);
        assert_eq!(saved, None);
        hold_speed_for_advice(&mut clock, &mut saved, false);
        assert_eq!(
            clock.speed,
            GameSpeed::Fast,
            "a hide with nothing saved holds"
        );
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
        // Source: FUN_005ffeb0 centering, FUN_00417020 alert 412x176, rect at 0x00658920.
        assert_eq!(pause_alert_origin(), (114.0, 152.0));
        assert_eq!(
            (ALERT_TEXT_RECT.right(), ALERT_TEXT_RECT.bottom()),
            (368.0, 112.0)
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

    fn primary(pos: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        }
    }

    fn key(key: egui::Key) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        }
    }

    /// Run one egui frame with the pointer at `pointer` and `events` after it,
    /// returning the shapes it painted.
    fn frame(
        ctx: &egui::Context,
        pointer: egui::Pos2,
        events: Vec<egui::Event>,
        draw: impl FnMut(&egui::Context),
    ) -> Vec<egui::epaint::ClippedShape> {
        let mut all = vec![egui::Event::PointerMoved(pointer)];
        all.extend(events);
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1400.0, 1040.0),
            )),
            events: all,
            ..Default::default()
        };
        ctx.run(input, draw).shapes
    }

    fn strategic_layout(scale: f32) -> CockpitLayout {
        crate::cockpit::CockpitState::new(CockpitFaction::Alliance)
            .layout_for(640.0 * scale, 480.0 * scale)
    }

    fn screen(layout: CockpitLayout, (x, y): (f32, f32)) -> egui::Pos2 {
        egui::pos2(
            layout.canvas.x + x * layout.scale,
            layout.canvas.y + y * layout.scale,
        )
    }

    #[test]
    fn a_right_release_on_the_day_readout_opens_the_menu_at_the_pointer() {
        // FUN_00422ce0: WM_RBUTTONUP inside (103,21)-(185,33) opens the menu
        // at the cursor; PtInRect excludes the right and bottom edges.
        // An offset canvas, so the anchor must subtract it.
        let mut layout = strategic_layout(2.0);
        layout.canvas.x += 10.0;
        layout.canvas.y += 20.0;
        let open_at = |point: (f32, f32), button: egui::PointerButton, covered: bool| {
            let ctx = egui::Context::default();
            let mut state = GameSpeedUiState::default();
            let pos = screen(layout, point);
            let button_event = |pressed| egui::Event::PointerButton {
                pos,
                button,
                pressed,
                modifiers: egui::Modifiers::default(),
            };
            let mut opened = false;
            for events in [vec![], vec![button_event(true)], vec![button_event(false)]] {
                frame(&ctx, pos, events, |ctx| {
                    if covered {
                        // Another window, such as an open menu, over the readout.
                        egui::Area::new(egui::Id::new("cover"))
                            .fixed_pos(egui::Pos2::ZERO)
                            .show(ctx, |ui| {
                                ui.allocate_exact_size(
                                    egui::vec2(1400.0, 1040.0),
                                    egui::Sense::hover(),
                                );
                            });
                    }
                    opened |= open_game_speed_menu_on_right_click(
                        ctx,
                        &mut state,
                        layout,
                        CockpitFaction::Alliance,
                    );
                });
            }
            (opened, state.menu_anchor)
        };
        let secondary = egui::PointerButton::Secondary;
        assert_eq!(
            open_at((120.0, 27.0), secondary, false),
            (true, Some((120.0, 27.0)))
        );
        assert_eq!(
            open_at((103.0, 21.0), secondary, false),
            (true, Some((103.0, 21.0)))
        );
        assert_eq!(open_at((185.0, 27.0), secondary, false), (false, None));
        assert_eq!(open_at((120.0, 33.0), secondary, false), (false, None));
        assert_eq!(
            open_at((120.0, 27.0), egui::PointerButton::Primary, false),
            (false, None)
        );
        assert_eq!(open_at((120.0, 27.0), secondary, true), (false, None));
    }

    #[test]
    fn choosing_a_speed_menu_row_returns_its_speed_and_closes_the_menu() {
        // FUN_004424c0 reports the chosen item; FUN_0042d190's records map
        // the five rows to the five speeds in order.
        for index in 0..GameSpeed::ALL.len() {
            let layout = strategic_layout(1.0);
            let ctx = egui::Context::default();
            let mut cache = BmpCache::new();
            let mut state = GameSpeedUiState {
                menu_anchor: Some((100.0, 100.0)),
            };
            let mut chosen = None;
            let mut draw = |ctx: &egui::Context| {
                chosen = chosen.or(draw_game_speed_menu(
                    ctx,
                    &mut state,
                    &mut cache,
                    layout,
                    CockpitFaction::Alliance,
                    true,
                ));
            };
            frame(&ctx, egui::Pos2::ZERO, vec![], &mut draw);
            frame(&ctx, egui::Pos2::ZERO, vec![], &mut draw);
            let rect = game_speed_menu_rect(&ctx).expect("the speed menu is laid out");
            let row = (rect.height() - 2.0) / GameSpeed::ALL.len() as f32;
            let at = egui::pos2(
                rect.center().x,
                rect.min.y + 2.0 + row * (index as f32 + 0.5),
            );
            frame(&ctx, at, vec![primary(at, true)], &mut draw);
            frame(&ctx, at, vec![primary(at, false)], &mut draw);
            assert_eq!(chosen, Some(GameSpeed::ALL[index]), "row {index}");
            assert_eq!(state.menu_anchor, None);
        }
    }

    #[test]
    fn a_press_outside_the_speed_menu_closes_it_without_a_choice() {
        let layout = strategic_layout(1.0);
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let mut state = GameSpeedUiState {
            menu_anchor: Some((100.0, 100.0)),
        };
        let mut chosen = None;
        let outside = screen(layout, (600.0, 450.0));
        for events in [vec![], vec![], vec![primary(outside, true)]] {
            frame(&ctx, outside, events, |ctx| {
                chosen = chosen.or(draw_game_speed_menu(
                    ctx,
                    &mut state,
                    &mut cache,
                    layout,
                    CockpitFaction::Alliance,
                    true,
                ));
            });
        }
        assert_eq!(chosen, None);
        assert_eq!(state.menu_anchor, None);
    }

    /// Draw the alert with the pointer at logical `point`: two layout frames,
    /// then a frame with the presses in `last` and one with the rest. Returns
    /// whether any frame asked to resume.
    fn alert_resumes(clock: &GameClock, point: (f32, f32), last: Vec<egui::Event>) -> bool {
        let layout = strategic_layout(2.0);
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let pos = screen(layout, point);
        let mut resumed = false;
        let last = last
            .into_iter()
            .map(|event| match event {
                egui::Event::PointerButton { pressed, .. } => primary(pos, pressed),
                other => other,
            })
            .collect::<Vec<_>>();
        let (press, release): (Vec<_>, Vec<_>) = last
            .into_iter()
            .partition(|event| matches!(event, egui::Event::PointerButton { pressed: true, .. }));
        for events in [vec![], vec![], press, release] {
            frame(&ctx, pos, events, |ctx| {
                resumed |= draw_pause_alert(ctx, clock, &mut cache, layout, true);
            });
        }
        resumed
    }

    #[test]
    fn the_pause_alert_checkmark_or_enter_resumes_and_nothing_else_does() {
        // FUN_00417150: the checkmark button at (176,134), 57 by 28, inside the
        // alert centred by FUN_005ffeb0; Enter is its default button.
        let mut paused = running(GameSpeed::Slow);
        paused.pause();
        let (x, y) = pause_alert_origin();
        let checkmark = (x + 176.0 + 28.0, y + 134.0 + 14.0);
        let click = vec![
            primary(egui::Pos2::ZERO, true),
            primary(egui::Pos2::ZERO, false),
        ];
        assert!(alert_resumes(&paused, checkmark, click.clone()));
        assert!(alert_resumes(
            &paused,
            (x + 20.0, y + 20.0),
            vec![key(egui::Key::Enter)]
        ));
        assert!(!alert_resumes(&paused, (x + 20.0, y + 20.0), click.clone()));
        assert!(!alert_resumes(&paused, checkmark, vec![]));
        let running = running(GameSpeed::Slow);
        assert!(!alert_resumes(&running, checkmark, click));
        assert!(!alert_resumes(
            &running,
            checkmark,
            vec![key(egui::Key::Enter)]
        ));
    }

    #[test]
    fn the_pause_alert_message_is_centred_in_its_rectangle() {
        // The message rectangle (43,25)-(368,112) read from 0x00658920.
        let layout = strategic_layout(2.0);
        let mut paused = running(GameSpeed::Slow);
        paused.pause();
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let mut shapes = Vec::new();
        for _ in 0..2 {
            shapes = frame(&ctx, egui::Pos2::ZERO, vec![], |ctx| {
                draw_pause_alert(ctx, &paused, &mut cache, layout, true);
            });
        }
        let text = shapes
            .into_iter()
            .find_map(|clipped| match clipped.shape {
                egui::Shape::Text(text) if text.galley.text() == RESUME_GAME_PLAY => Some(text),
                _ => None,
            })
            .expect("the message is painted");
        let (x, y) = pause_alert_origin();
        let centre = screen(
            layout,
            (
                x + ALERT_TEXT_RECT.x + ALERT_TEXT_RECT.width / 2.0,
                y + ALERT_TEXT_RECT.y + ALERT_TEXT_RECT.height / 2.0,
            ),
        );
        let painted = text.pos + text.galley.size() / 2.0;
        assert!(
            (painted - centre).length() < 0.5,
            "{painted:?} vs {centre:?}"
        );
        assert_eq!(text.fallback_color, ALERT_TEXT_COLOR);
        let expected = ctx.fonts(|fonts| {
            fonts
                .layout_no_wrap(
                    RESUME_GAME_PLAY.to_owned(),
                    egui::FontId::proportional(ALERT_FONT_HEIGHT * 2.0),
                    ALERT_TEXT_COLOR,
                )
                .size()
        });
        assert_eq!(text.galley.size(), expected);
        // FUN_00417020: the 412 by 176 alert, at twice the size.
        let rect = ctx
            .memory(|memory| memory.area_rect(egui::Id::new("original_pause_alert")))
            .expect("the alert is laid out");
        assert_eq!(rect.size(), egui::vec2(ALERT_WIDTH, ALERT_HEIGHT) * 2.0);
    }

    #[test]
    fn the_day_readout_is_centred_between_its_anchor_and_the_hit_rectangle() {
        // FUN_00601ce0 centres the text from FUN_00601b30's anchor to the
        // right edge of the readout.
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let layout = crate::cockpit::CockpitState::new(faction).layout_for(1280.0, 960.0);
            let ctx = egui::Context::default();
            let shapes = frame(&ctx, egui::Pos2::ZERO, vec![], |ctx| {
                draw_day_readout(ctx, layout, faction, 42);
            });
            let text = shapes
                .into_iter()
                .find_map(|clipped| match clipped.shape {
                    egui::Shape::Text(text) => Some(text),
                    _ => None,
                })
                .expect("the day is painted");
            assert_eq!(text.galley.text(), "42");
            let (x, y) = day_text_origin(faction);
            let hit = day_readout_rect(faction);
            let centre = screen(layout, ((x + hit.x + hit.width) / 2.0, y));
            assert!(
                (text.pos.x + text.galley.size().x / 2.0 - centre.x).abs() < 0.01,
                "{faction:?}"
            );
            assert_eq!(text.pos.y, centre.y, "{faction:?}");
            assert_eq!(text.fallback_color, faction_text_color(faction));
            let expected = ctx.fonts(|fonts| {
                fonts
                    .layout_no_wrap(
                        "42".to_owned(),
                        egui::FontId::proportional(DAY_FONT_HEIGHT * layout.scale),
                        faction_text_color(faction),
                    )
                    .size()
            });
            assert_eq!(text.galley.size(), expected, "{faction:?}");
        }
    }
}
