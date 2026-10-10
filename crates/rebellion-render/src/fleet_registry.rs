//! The Fleet Registry, a port extension: a cockpit readout, opened from the
//! main menu's chip beside the music toggle, that chooses how a new game
//! names its fleets (`FleetNaming`). The 1998 cockpit has no such control;
//! the original only numbers fleets (`ghidra/notes/fleet-names.md`).
//!
//! It draws in the green monospace of the cockpit's game-type readout. Each
//! side's row scrolls through the names its next fleets will take; hovering
//! a name pauses the row and shows a line on who the formation was.

use egui_macroquad::egui::{
    self, Color32, FontFamily, FontId, Pos2, Rect, Sense, Stroke, TextureId, Vec2, WidgetInfo,
    WidgetType,
};
use rebellion_core::fleet_name_bank::{self, BankName};
use rebellion_core::world::FleetNaming;

use crate::main_menu::{LogicalRect, LOGICAL_WIDTH};

/// The readout's place on the 640x480 cockpit.
pub const PANEL_RECT: LogicalRect = LogicalRect::new(100.0, 92.0, 440.0, 262.0);
const ORIGINAL_ROW: LogicalRect = LogicalRect::new(116.0, 128.0, 408.0, 22.0);
const CANONICAL_ROW: LogicalRect = LogicalRect::new(116.0, 154.0, 408.0, 22.0);
const EMPIRE_ROW: LogicalRect = LogicalRect::new(116.0, 196.0, 408.0, 30.0);
const ALLIANCE_ROW: LogicalRect = LogicalRect::new(116.0, 232.0, 408.0, 30.0);
const LORE_RECT: LogicalRect = LogicalRect::new(116.0, 270.0, 408.0, 40.0);
const RETURN_RECT: LogicalRect = LogicalRect::new(444.0, 320.0, 80.0, 22.0);
/// The emblem's square at a side row's left; the ticker runs to its right.
const EMBLEM_SIZE: f32 = 30.0;
/// Ticker speed, logical pixels a second.
const TICKER_SPEED: f32 = 24.0;
/// Names shown under Original: the first numbers a side gives.
const ORIGINAL_PREVIEW: u32 = 4;
const SEPARATOR: &str = "  ·  ";

const READOUT: Color32 = Color32::from_rgb(80, 255, 80);
const READOUT_DIM: Color32 = Color32::from_rgb(34, 110, 34);
const SIGNATURE: Color32 = Color32::from_rgb(255, 190, 60);
const APERTURE: Color32 = Color32::from_rgb(6, 10, 8);
const BEVEL_LIGHT: Color32 = Color32::from_rgb(214, 226, 222);
const BEVEL_FACE: Color32 = Color32::from_rgb(166, 180, 176);
const BEVEL_DARK: Color32 = Color32::from_rgb(92, 104, 104);

/// One name in a side's ticker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TickerEntry {
    pub label: String,
    /// The bank entry behind a canonical name; `None` for "Fleet N".
    pub bank: Option<BankName>,
}

/// The names `is_alliance`'s side would give its next fleets under `naming`.
#[must_use]
pub fn ticker_entries(naming: FleetNaming, is_alliance: bool) -> Vec<TickerEntry> {
    match naming {
        FleetNaming::Original => (1..=ORIGINAL_PREVIEW)
            .map(|number| TickerEntry {
                label: format!("FLEET {number}"),
                bank: None,
            })
            .collect(),
        FleetNaming::Canonical => fleet_name_bank::bank(is_alliance)
            .iter()
            .map(|entry| TickerEntry {
                label: entry.name.to_uppercase(),
                bank: Some(*entry),
            })
            .collect(),
    }
}

/// The line shown for a hovered name.
#[must_use]
pub fn lore_line(entry: &BankName) -> String {
    let source = if entry.canon { "CANON" } else { "LEGENDS" };
    let flagship = match entry.flagship {
        Some(fleet_name_bank::SUPER_STAR_DESTROYER) => {
            " Waits for a fleet holding a Super Star Destroyer."
        }
        Some(fleet_name_bank::MON_CALAMARI_CRUISER) => {
            " Waits for a fleet holding a Mon Calamari Cruiser."
        }
        _ => "",
    };
    format!("[{source}] {}{flagship}", entry.note)
}

/// What a frame of the open registry asks of the menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistryOutcome {
    Open,
    Close,
}

/// The open registry's ticker positions.
#[derive(Debug, Clone, PartialEq)]
pub struct FleetRegistryState {
    /// Per side (Empire, Alliance): how far the row has scrolled, logical px.
    offsets: [f32; 2],
    last_frame: Option<f64>,
}

impl FleetRegistryState {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            offsets: [0.0; 2],
            last_frame: None,
        }
    }

    /// Scroll each unpaused row by the time since the last frame.
    fn advance(&mut self, now: f64, paused: [bool; 2]) {
        #[expect(
            clippy::cast_possible_truncation,
            reason = "A frame's elapsed seconds fit an f32 offset."
        )]
        let elapsed = self
            .last_frame
            .map_or(0.0, |last| (now - last).max(0.0) as f32);
        self.last_frame = Some(now);
        for (offset, paused) in self.offsets.iter_mut().zip(paused) {
            if !paused {
                *offset += elapsed * TICKER_SPEED;
            }
        }
    }
}

impl Default for FleetRegistryState {
    fn default() -> Self {
        Self::new()
    }
}

fn to_screen(canvas: Rect, logical: LogicalRect) -> Rect {
    crate::main_menu::control_rect(canvas, logical)
}

fn bevel(painter: &egui::Painter, rect: Rect, scale: f32) {
    let stroke = scale.max(1.0);
    painter.rect_filled(rect, scale, BEVEL_FACE);
    painter.line_segment(
        [rect.left_bottom(), rect.left_top()],
        Stroke::new(stroke, BEVEL_LIGHT),
    );
    painter.line_segment(
        [rect.left_top(), rect.right_top()],
        Stroke::new(stroke, BEVEL_LIGHT),
    );
    painter.line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        Stroke::new(stroke, BEVEL_DARK),
    );
    painter.line_segment(
        [rect.right_bottom(), rect.right_top()],
        Stroke::new(stroke, BEVEL_DARK),
    );
    painter.rect_filled(rect.shrink(4.0 * scale), 0.5 * scale, APERTURE);
}

fn font(scale: f32, size: f32) -> FontId {
    FontId::new((size * scale).max(8.0), FontFamily::Monospace)
}

/// A mode row: a lamp, lit when `lit`, and its label.
#[expect(
    clippy::too_many_arguments,
    reason = "A row's paint needs its geometry, state and both texts."
)]
fn mode_row(
    ui: &mut egui::Ui,
    rect: Rect,
    scale: f32,
    lit: bool,
    name: &str,
    detail: &str,
    id: egui::Id,
    accessible: &str,
) -> bool {
    let response = ui.interact(rect, id, Sense::click());
    response.widget_info(|| WidgetInfo::selected(WidgetType::RadioButton, true, lit, accessible));
    let painter = ui.painter();
    let lamp = Rect::from_min_size(
        rect.left_center() - Vec2::new(0.0, 4.0 * scale),
        Vec2::new(12.0 * scale, 8.0 * scale),
    );
    painter.rect_filled(lamp.expand(scale), 0.0, Color32::from_rgb(24, 30, 32));
    painter.rect_filled(
        lamp,
        0.0,
        if lit {
            READOUT
        } else {
            Color32::from_rgb(20, 44, 20)
        },
    );
    let color = if lit { READOUT } else { READOUT_DIM };
    painter.text(
        rect.left_center() + Vec2::new(20.0 * scale, 0.0),
        egui::Align2::LEFT_CENTER,
        name,
        font(scale, 12.0),
        color,
    );
    painter.text(
        rect.left_center() + Vec2::new(130.0 * scale, 0.0),
        egui::Align2::LEFT_CENTER,
        detail,
        font(scale, 10.0),
        color,
    );
    if response.has_focus() || response.hovered() {
        painter.rect_stroke(
            rect.expand(scale),
            scale,
            Stroke::new(scale.max(1.0), READOUT_DIM),
            egui::StrokeKind::Outside,
        );
    }
    let keyboard = response.has_focus()
        && ui.input(|input| {
            input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Space)
        });
    response.clicked() || keyboard
}

/// Draw one side's ticker inside `rect` from `offset`; returns the hovered
/// entry, whose row then holds still.
fn ticker(
    ui: &egui::Ui,
    rect: Rect,
    scale: f32,
    offset: f32,
    entries: &[TickerEntry],
    emblem: Option<TextureId>,
) -> Option<TickerEntry> {
    let painter = ui.painter().with_clip_rect(rect);
    // The cockpit's emblem monitors are 62x55 (COMMON.DLL 10009, 10007).
    let emblem_rect = Rect::from_min_size(
        rect.min,
        Vec2::new(EMBLEM_SIZE * 62.0 / 55.0, EMBLEM_SIZE) * scale,
    );
    if let Some(texture) = emblem {
        painter.image(
            texture,
            emblem_rect,
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            Color32::WHITE,
        );
    }
    let lane = Rect::from_min_max(
        Pos2::new(emblem_rect.right() + 8.0 * scale, rect.top()),
        rect.max,
    );
    let painter = painter.with_clip_rect(lane);
    let font = font(scale, 12.0);
    let separator = painter.layout_no_wrap(SEPARATOR.to_owned(), font.clone(), READOUT_DIM);
    let galleys: Vec<_> = entries
        .iter()
        .map(|entry| {
            let color = if entry.bank.is_some_and(|bank| bank.flagship.is_some()) {
                SIGNATURE
            } else {
                READOUT
            };
            painter.layout_no_wrap(entry.label.clone(), font.clone(), color)
        })
        .collect();
    let cycle: f32 = galleys
        .iter()
        .map(|galley| galley.size().x + separator.size().x)
        .sum();
    if cycle <= 0.0 {
        return None;
    }
    let pointer = ui
        .ctx()
        .pointer_hover_pos()
        .filter(|point| lane.contains(*point));
    let mut hovered = None;
    let mut x = lane.left() - (offset * scale).rem_euclid(cycle);
    let y = lane.center().y;
    // Draw enough cycles to fill the lane from the scrolled start.
    while x < lane.right() {
        for (entry, galley) in entries.iter().zip(&galleys) {
            let at = Pos2::new(x, y - galley.size().y / 2.0);
            let bounds = Rect::from_min_size(at, galley.size());
            if pointer.is_some_and(|point| bounds.contains(point)) {
                hovered = Some(entry.clone());
            }
            painter.galley(at, galley.clone(), READOUT);
            x += galley.size().x;
            painter.galley(
                Pos2::new(x, y - separator.size().y / 2.0),
                separator.clone(),
                READOUT_DIM,
            );
            x += separator.size().x;
        }
    }
    hovered
}

/// Draw the open registry over `canvas`, blocking the cockpit beneath it.
/// `emblems` are the cockpit's Empire and Alliance monitors.
pub fn draw_fleet_registry(
    ctx: &egui::Context,
    canvas: Rect,
    state: &mut FleetRegistryState,
    naming: &mut FleetNaming,
    emblems: [Option<TextureId>; 2],
) -> RegistryOutcome {
    let scale = canvas.width() / LOGICAL_WIDTH;
    let now = ctx.input(|input| input.time);
    let mut outcome = RegistryOutcome::Open;
    egui::Area::new(egui::Id::new("fleet_registry"))
        .order(egui::Order::Foreground)
        .fixed_pos(canvas.min)
        .show(ctx, |ui| {
            ui.set_min_size(canvas.size());
            // Swallow every press on the cockpit beneath.
            ui.interact(canvas, ui.id().with("veil"), Sense::click_and_drag());
            ui.painter()
                .rect_filled(canvas, 0.0, Color32::from_black_alpha(150));
            let panel = to_screen(canvas, PANEL_RECT);
            bevel(ui.painter(), panel, scale);
            ui.painter().text(
                panel.center_top() + Vec2::new(0.0, 16.0 * scale),
                egui::Align2::CENTER_CENTER,
                "OPEN REBELLION · FLEET REGISTRY",
                font(scale, 13.0),
                READOUT,
            );

            if mode_row(
                ui,
                to_screen(canvas, ORIGINAL_ROW),
                scale,
                *naming == FleetNaming::Original,
                "ORIGINAL",
                "FLEET 1 · FLEET 2 · FLEET 3, AS IN 1998",
                ui.id().with("original"),
                "Original fleet names: Fleet 1, Fleet 2",
            ) {
                *naming = FleetNaming::Original;
            }
            if mode_row(
                ui,
                to_screen(canvas, CANONICAL_ROW),
                scale,
                *naming == FleetNaming::Canonical,
                "CANONICAL",
                "NAMES FROM THE GALACTIC CIVIL WAR",
                ui.id().with("canonical"),
                "Canonical fleet names from the Galactic Civil War",
            ) {
                *naming = FleetNaming::Canonical;
            }

            let rows = [
                (to_screen(canvas, EMPIRE_ROW), false, emblems[0]),
                (to_screen(canvas, ALLIANCE_ROW), true, emblems[1]),
            ];
            let mut paused = [false; 2];
            let mut hovered = None;
            for (index, (rect, is_alliance, emblem)) in rows.into_iter().enumerate() {
                let entries = ticker_entries(*naming, is_alliance);
                if let Some(entry) = ticker(ui, rect, scale, state.offsets[index], &entries, emblem)
                {
                    paused[index] = true;
                    hovered = Some(entry);
                }
            }
            state.advance(now, paused);

            let lore = to_screen(canvas, LORE_RECT);
            let line = hovered.and_then(|entry| entry.bank).map_or_else(
                || "APPLIES TO YOUR NEXT NEW GAME. A SAVED GAME KEEPS ITS NAMES.".to_owned(),
                |bank| lore_line(&bank),
            );
            let galley = ui
                .painter()
                .layout(line, font(scale, 10.0), READOUT, lore.width());
            ui.painter().galley(lore.min, galley, READOUT);

            let back = to_screen(canvas, RETURN_RECT);
            let response = ui.interact(back, ui.id().with("return"), Sense::click());
            response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, true, "Return"));
            ui.painter().rect_stroke(
                back,
                scale,
                Stroke::new(
                    scale.max(1.0),
                    if response.hovered() {
                        READOUT
                    } else {
                        READOUT_DIM
                    },
                ),
                egui::StrokeKind::Inside,
            );
            ui.painter().text(
                back.center(),
                egui::Align2::CENTER_CENTER,
                "RETURN",
                font(scale, 12.0),
                READOUT,
            );
            let keyboard_return = response.has_focus()
                && ui.input(|input| {
                    input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Space)
                });
            if response.clicked()
                || keyboard_return
                || ui.input(|input| input.key_pressed(egui::Key::Escape))
            {
                outcome = RegistryOutcome::Close;
            }
        });
    ctx.request_repaint();
    outcome
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_naming_previews_the_sides_first_numbers() {
        let entries = ticker_entries(FleetNaming::Original, false);
        let labels: Vec<&str> = entries.iter().map(|entry| entry.label.as_str()).collect();
        assert_eq!(labels, ["FLEET 1", "FLEET 2", "FLEET 3", "FLEET 4"]);
        assert!(entries.iter().all(|entry| entry.bank.is_none()));
    }

    #[test]
    fn canonical_naming_previews_each_sides_bank_in_order() {
        let empire = ticker_entries(FleetNaming::Canonical, false);
        assert_eq!(empire.len(), fleet_name_bank::EMPIRE.len());
        assert_eq!(empire[0].label, "DEATH SQUADRON");
        assert_eq!(empire[1].label, "SEVENTH FLEET");
        let alliance = ticker_entries(FleetNaming::Canonical, true);
        assert_eq!(alliance[1].label, "ALPHA GROUP");
        assert_eq!(alliance[0].bank, Some(fleet_name_bank::ALLIANCE[0]));
    }

    #[test]
    fn a_names_lore_gives_its_source_and_any_flagship_it_waits_for() {
        let squadron = lore_line(&fleet_name_bank::EMPIRE[0]);
        assert!(squadron.starts_with("[CANON] "));
        assert!(squadron.ends_with("Waits for a fleet holding a Super Star Destroyer."));
        let command = lore_line(&fleet_name_bank::ALLIANCE[0]);
        assert!(command.starts_with("[LEGENDS] "));
        assert!(command.ends_with("Waits for a fleet holding a Mon Calamari Cruiser."));
        let plain = lore_line(&fleet_name_bank::ALLIANCE[1]);
        assert_eq!(
            plain,
            format!("[CANON] {}", fleet_name_bank::ALLIANCE[1].note)
        );
    }

    #[test]
    fn a_hovered_row_holds_still_while_the_other_scrolls() {
        let mut state = FleetRegistryState::new();
        state.advance(10.0, [false; 2]);
        assert_eq!(state.offsets, [0.0; 2]);
        state.advance(11.0, [true, false]);
        assert_eq!(state.offsets, [0.0, TICKER_SPEED]);
        state.advance(10.5, [false; 2]);
        assert_eq!(state.offsets, [0.0, TICKER_SPEED]);
    }

    /// Run the open registry over a 640x480 cockpit, a frame per event list;
    /// returns the last frame's outcome.
    fn run(naming: &mut FleetNaming, frames: Vec<Vec<egui::Event>>) -> RegistryOutcome {
        let ctx = egui::Context::default();
        let canvas = Rect::from_min_size(Pos2::ZERO, Vec2::new(640.0, 480.0));
        let mut state = FleetRegistryState::new();
        let mut outcome = RegistryOutcome::Open;
        for events in [vec![], vec![]].into_iter().chain(frames) {
            let input = egui::RawInput {
                screen_rect: Some(canvas),
                events,
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                outcome = draw_fleet_registry(ctx, canvas, &mut state, naming, [None, None]);
            });
        }
        outcome
    }

    fn click(rect: LogicalRect) -> Vec<Vec<egui::Event>> {
        let pos = Pos2::new(rect.x + rect.width / 2.0, rect.y + rect.height / 2.0);
        let press = |pressed| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        vec![
            vec![egui::Event::PointerMoved(pos)],
            vec![press(true)],
            vec![press(false)],
        ]
    }

    fn key(key: egui::Key) -> Vec<Vec<egui::Event>> {
        vec![vec![egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        }]]
    }

    #[test]
    fn hovering_a_name_holds_its_row_while_the_other_scrolls() {
        let ctx = egui::Context::default();
        let canvas = Rect::from_min_size(Pos2::ZERO, Vec2::new(640.0, 480.0));
        let mut state = FleetRegistryState::new();
        let mut naming = FleetNaming::Canonical;
        // The Empire lane's first letters, past its emblem and gap.
        let lane = EMPIRE_ROW.x + EMBLEM_SIZE * 62.0 / 55.0 + 8.0;
        let pointer = Pos2::new(lane + 4.0, EMPIRE_ROW.y + EMPIRE_ROW.height / 2.0);
        for (time, events) in [
            (0.0, vec![egui::Event::PointerMoved(pointer)]),
            (1.0, vec![]),
            (2.0, vec![]),
        ] {
            let input = egui::RawInput {
                screen_rect: Some(canvas),
                time: Some(time),
                events,
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                draw_fleet_registry(ctx, canvas, &mut state, &mut naming, [None, None]);
            });
        }
        assert_eq!(state.offsets, [0.0, 2.0 * TICKER_SPEED]);
    }

    #[test]
    fn clicking_a_mode_row_lights_it_and_the_registry_stays_open() {
        let mut naming = FleetNaming::Original;
        assert_eq!(
            run(&mut naming, click(CANONICAL_ROW)),
            RegistryOutcome::Open
        );
        assert_eq!(naming, FleetNaming::Canonical);
        assert_eq!(run(&mut naming, click(ORIGINAL_ROW)), RegistryOutcome::Open);
        assert_eq!(naming, FleetNaming::Original);
    }

    #[test]
    fn return_or_escape_closes_the_registry_keeping_the_choice() {
        let mut naming = FleetNaming::Canonical;
        assert_eq!(run(&mut naming, click(RETURN_RECT)), RegistryOutcome::Close);
        assert_eq!(
            run(&mut naming, key(egui::Key::Escape)),
            RegistryOutcome::Close
        );
        assert_eq!(naming, FleetNaming::Canonical);
        let elsewhere = LogicalRect::new(10.0, 10.0, 4.0, 4.0);
        assert_eq!(run(&mut naming, click(elsewhere)), RegistryOutcome::Open);
        // Enter answers only the focused control; nothing has focus here.
        assert_eq!(
            run(&mut naming, key(egui::Key::Enter)),
            RegistryOutcome::Open
        );
    }

    #[test]
    fn the_readout_sits_inside_the_cockpit_clear_of_the_extension_chips() {
        let right = PANEL_RECT.x + PANEL_RECT.width;
        let bottom = PANEL_RECT.y + PANEL_RECT.height;
        assert!(PANEL_RECT.x >= 0.0 && right <= LOGICAL_WIDTH);
        assert!(bottom <= crate::main_menu::LOGICAL_HEIGHT);
        const { assert!(PANEL_RECT.y > crate::main_menu::MUSIC_TOGGLE_RECT.y + 22.0) };
        for row in [
            ORIGINAL_ROW,
            CANONICAL_ROW,
            EMPIRE_ROW,
            ALLIANCE_ROW,
            LORE_RECT,
            RETURN_RECT,
        ] {
            assert!(row.x >= PANEL_RECT.x && row.x + row.width <= right);
            assert!(row.y >= PANEL_RECT.y && row.y + row.height <= bottom);
        }
    }
}
