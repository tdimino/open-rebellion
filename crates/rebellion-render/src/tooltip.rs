//! The original's help balloons over command-center controls.
//!
//! `FUN_006008e0` creates one stock Win32 `tooltips_class32` window with
//! `TTS_ALWAYSTIP`, and controls register their TEXTSTRA strings with
//! `TTM_ADDTOOL` (`FUN_00600970`, `FUN_00600a40`, `FUN_00600b30`). The game
//! sets no colors, font or custom drawing, so the balloon is the system's:
//! `COLOR_INFOBK` cream, `COLOR_INFOTEXT` black, a one-pixel black border and
//! the 8-point system font. It shows after the hover delay, below the cursor,
//! and a click or leaving the control hides it.

use egui_macroquad::egui;

use crate::cockpit::{
    strategic_message_index_controls, strategic_primary_controls, CockpitButton, CockpitFaction,
    CockpitLayout, CockpitViewport,
};
use crate::game_speed::day_readout_rect;
use crate::message_log::MessageRail;

/// `COLOR_INFOBK` on Windows 95 and 98.
const FILL: egui::Color32 = egui::Color32::from_rgb(255, 255, 225);
const TEXT: egui::Color32 = egui::Color32::BLACK;
const BORDER: egui::Color32 = egui::Color32::BLACK;
/// The 8-point system font: an 11-pixel em at 96 dpi.
const FONT_SIZE: f32 = 11.0;
/// hyp: the stock control's margin around the text.
const PADDING: egui::Vec2 = egui::vec2(3.0, 2.0);
/// hyp: the system's initial delay, the double-click time's default.
const DELAY_SECONDS: f64 = 0.5;
/// hyp: the balloon's top below the cursor's hot spot, measured in a capture
/// of the original's "Raw Materials Monitor" balloon.
const BELOW_CURSOR: f32 = 12.0;

/// A control's balloon: its rectangle in the 640x480 canvas and its text.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TooltipTarget {
    pub rect: CockpitViewport,
    pub text: &'static str,
}

const fn rect(left: f32, top: f32, right: f32, bottom: f32) -> CockpitViewport {
    CockpitViewport {
        x: left,
        y: top,
        width: right - left,
        height: bottom - top,
    }
}

/// The resource counters' rectangles (`FUN_00422ce0`), left to right, with
/// TEXTSTRA 5392..5394.
const fn counter_targets(faction: CockpitFaction) -> [TooltipTarget; 3] {
    let rects = match faction {
        CockpitFaction::Alliance => [
            rect(240.0, 16.0, 330.0, 33.0),
            rect(335.0, 16.0, 425.0, 33.0),
            rect(430.0, 16.0, 520.0, 33.0),
        ],
        CockpitFaction::Empire => [
            rect(149.0, 18.0, 241.0, 37.0),
            rect(248.0, 18.0, 340.0, 37.0),
            rect(345.0, 18.0, 437.0, 37.0),
        ],
    };
    [
        TooltipTarget {
            rect: rects[0],
            text: "Raw Materials Monitor",
        },
        TooltipTarget {
            rect: rects[1],
            text: "Refined Materials Monitor",
        },
        TooltipTarget {
            rect: rects[2],
            text: "Maintenance Monitor",
        },
    ]
}

/// A console button's TEXTSTRA string (`FUN_004286b0`): 0x1500..0x1506.
const fn button_text(button: CockpitButton) -> &'static str {
    match button {
        CockpitButton::GameOptions => "Game Controls",
        CockpitButton::SystemFinder => "System Finder",
        CockpitButton::FleetFinder => "Fleet Finder",
        CockpitButton::PersonnelFinder => "Personnel Finder",
        CockpitButton::TroopFinder => "Troop Finder",
        CockpitButton::Encyclopedia => "Encyclopedia",
        CockpitButton::GalacticInformationDisplay => "Galactic Information Display",
    }
}

/// A rail control's TEXTSTRA string (`FUN_00427270`): 0x8011..0x8018 and
/// 0x8032.
const fn rail_text(rail: MessageRail) -> &'static str {
    match rail {
        MessageRail::PopularSupport => "Popular Support Messages",
        MessageRail::Fleet => "Fleet Messages",
        MessageRail::Mission => "Mission Messages",
        MessageRail::Resource => "Resource Messages",
        MessageRail::Manufacturing => "Manufacturing Messages",
        MessageRail::Defense => "Defense Messages",
        MessageRail::Conflict => "Conflict Messages",
        MessageRail::Advice => "Advice Messages",
        MessageRail::Chat => "Chat Messages",
    }
}

/// Every command-center control that registers a balloon: the three
/// counters, the day readout (TEXTSTRA 5395), the console buttons and the
/// Message Index rail.
#[must_use]
pub fn cockpit_tooltip_targets(faction: CockpitFaction) -> Vec<TooltipTarget> {
    let mut targets = counter_targets(faction).to_vec();
    targets.push(TooltipTarget {
        rect: day_readout_rect(faction),
        text: "Game Speed Control",
    });
    targets.extend(
        strategic_primary_controls(faction)
            .iter()
            .chain(std::iter::once(crate::cockpit::strategic_side_control(
                faction,
            )))
            .map(|control| TooltipTarget {
                rect: control.rect,
                text: button_text(control.button),
            }),
    );
    targets.extend(
        strategic_message_index_controls(faction)
            .iter()
            .zip(MessageRail::RAIL_ORDER)
            .map(|(control, rail)| TooltipTarget {
                rect: control.rect,
                text: rail_text(rail),
            }),
    );
    targets
}

/// The target under a canvas point; right and bottom edges are exclusive,
/// as `PtInRect` treats them.
#[must_use]
pub fn tooltip_target_at(targets: &[TooltipTarget], point: (f32, f32)) -> Option<usize> {
    targets.iter().position(|target| {
        point.0 >= target.rect.x
            && point.0 < target.rect.x + target.rect.width
            && point.1 >= target.rect.y
            && point.1 < target.rect.y + target.rect.height
    })
}

/// Which control the pointer rests on, and since when.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TooltipState {
    hovered: Option<(usize, f64)>,
    /// A click hides the balloon until the pointer leaves the control.
    dismissed: bool,
}

impl TooltipState {
    /// Track the target under the pointer at `time`, and return the one
    /// whose balloon shows. A press hides it until the pointer moves to
    /// another control.
    pub fn update(&mut self, target: Option<usize>, time: f64, pressed: bool) -> Option<usize> {
        match (self.hovered, target) {
            (Some((current, _)), Some(next)) if current == next => {}
            (_, Some(next)) => {
                self.hovered = Some((next, time));
                self.dismissed = false;
            }
            (_, None) => {
                self.hovered = None;
                self.dismissed = false;
            }
        }
        if pressed {
            self.dismissed = true;
        }
        let (index, since) = self.hovered?;
        (!self.dismissed && time - since >= DELAY_SECONDS).then_some(index)
    }
}

/// Show the balloon of the command-center control under the pointer.
///
/// No balloon shows while the pointer is over a window.
pub fn draw_cockpit_tooltips(
    ctx: &egui::Context,
    state: &mut TooltipState,
    layout: CockpitLayout,
    faction: CockpitFaction,
) {
    if layout.scale <= 0.0 {
        return;
    }
    let (pointer, time, pressed) = ctx.input(|input| {
        (
            input.pointer.hover_pos(),
            input.time,
            input.pointer.any_pressed(),
        )
    });
    let targets = cockpit_tooltip_targets(faction);
    let target = pointer
        .filter(|_| !ctx.is_pointer_over_area())
        .and_then(|pointer| {
            tooltip_target_at(
                &targets,
                (
                    (pointer.x - layout.canvas.x) / layout.scale,
                    (pointer.y - layout.canvas.y) / layout.scale,
                ),
            )
        });
    let shown = state.update(target, time, pressed);
    if target.is_some() && shown.is_none() {
        // Wake for the delay's end without new input.
        ctx.request_repaint_after(std::time::Duration::from_millis(50));
    }
    let (Some(index), Some(pointer)) = (shown, pointer) else {
        return;
    };
    let scale = layout.scale;
    let painter = ctx.layer_painter(egui::LayerId::new(
        egui::Order::Tooltip,
        egui::Id::new("original_tooltip"),
    ));
    let galley = painter.layout_no_wrap(
        targets[index].text.to_owned(),
        egui::FontId::proportional(FONT_SIZE * scale),
        TEXT,
    );
    let size = galley.size() + PADDING * 2.0 * scale;
    let screen = ctx.screen_rect();
    let min = egui::pos2(
        pointer.x.min(screen.max.x - size.x).max(screen.min.x),
        (pointer.y + BELOW_CURSOR * scale).min(screen.max.y - size.y),
    );
    let balloon = egui::Rect::from_min_size(min, size);
    painter.rect_filled(balloon, 0.0, FILL);
    painter.rect_stroke(
        balloon,
        0.0,
        egui::Stroke::new(scale.max(1.0), BORDER),
        egui::StrokeKind::Inside,
    );
    painter.galley(min + PADDING * scale, galley, TEXT);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_top_strip_registers_its_four_balloons() {
        // FUN_00422ce0.c:235-259: TEXTSTRA 0x1510..0x1513 on the counters'
        // and the day readout's rectangles.
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let targets = cockpit_tooltip_targets(faction);
            let texts: Vec<_> = targets[..4].iter().map(|target| target.text).collect();
            assert_eq!(
                texts,
                [
                    "Raw Materials Monitor",
                    "Refined Materials Monitor",
                    "Maintenance Monitor",
                    "Game Speed Control"
                ]
            );
        }
        let empire = cockpit_tooltip_targets(CockpitFaction::Empire);
        assert_eq!(tooltip_target_at(&empire, (200.0, 25.0)), Some(0));
        assert_eq!(tooltip_target_at(&empire, (300.0, 25.0)), Some(1));
        assert_eq!(tooltip_target_at(&empire, (400.0, 25.0)), Some(2));
        assert_eq!(tooltip_target_at(&empire, (530.0, 25.0)), Some(3));
        assert_eq!(tooltip_target_at(&empire, (241.0, 25.0)), None);
    }

    #[test]
    fn console_buttons_and_the_rail_carry_their_textstra_names() {
        // FUN_004286b0: 0x1500..0x1506; FUN_00427270: 0x8011..0x8018, 0x8032.
        let targets = cockpit_tooltip_targets(CockpitFaction::Alliance);
        let texts: Vec<_> = targets.iter().map(|target| target.text).collect();
        for text in [
            "Game Controls",
            "System Finder",
            "Fleet Finder",
            "Personnel Finder",
            "Troop Finder",
            "Encyclopedia",
            "Galactic Information Display",
            "Popular Support Messages",
            "Resource Messages",
            "Advice Messages",
            "Chat Messages",
        ] {
            assert!(texts.contains(&text), "{text}");
        }
        assert_eq!(targets.len(), 4 + 7 + 9);
    }

    #[test]
    fn a_balloon_waits_for_the_delay_and_a_click_hides_it_until_the_pointer_moves_on() {
        let mut state = TooltipState::default();
        assert_eq!(state.update(Some(0), 10.0, false), None);
        assert_eq!(state.update(Some(0), 10.4, false), None);
        assert_eq!(state.update(Some(0), 10.5, false), Some(0));
        assert_eq!(state.update(Some(0), 10.6, true), None);
        assert_eq!(state.update(Some(0), 12.0, false), None);
        assert_eq!(state.update(Some(1), 12.1, false), None);
        assert_eq!(state.update(Some(1), 12.6, false), Some(1));
        assert_eq!(state.update(None, 12.7, false), None);
        assert_eq!(state.update(Some(1), 12.8, false), None);
    }
}
