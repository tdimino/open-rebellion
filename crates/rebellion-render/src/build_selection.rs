//! The original Build Selection window (`FUN_00437df0` builds it,
//! `FUN_00437f80` and `FUN_00438620` its fields and controls, `FUN_00438800`
//! handles them; `ghidra/notes/manufacturing-build-selection.md`).
//!
//! A 210 by 261 window over background 10800 that a Manufacturing window
//! band's Build opens. It lists the classes the band's yards build, shows
//! the order's two costs and its best completion and deployment times, and
//! takes the number to build. Confirm replaces what the band was building.

use egui_macroquad::egui;
use rebellion_core::build_selection::{estimate, order_refusal, BuildEstimate};
use rebellion_core::delivery::DeliveryState;
use rebellion_core::ids::SystemKey;
use rebellion_core::manufacturing::{BuildableKind, ManufacturingState, ProductionArea};
use rebellion_core::movement::MovementState;
use rebellion_core::troop_transport::TroopTransportState;
use rebellion_core::world::GameWorld;

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::CockpitLayout;
use crate::manufacturing_window::product;
use crate::mission_dialog::{button, galaxy_centered_rect, paint, paint_button, paint_centered, scrolled};
use crate::system_window::{exact_clicked, logical_rect, rect_contains};

pub const BUILD_SELECTION_WIDTH: f32 = 210.0;
pub const BUILD_SELECTION_HEIGHT: f32 = 261.0;

// STRATEGY bitmaps (FUN_00437df0, FUN_00438500, FUN_00438620).
const BACKGROUND: u32 = 10800;
const TITLE_ALLIANCE: u32 = 10801;
const TITLE_EMPIRE: u32 = 10802;
const CLOSE: (u32, u32) = (10108, 10109);
const LIST: (u32, u32) = (10606, 10607);
const CONFIRM: (u32, u32) = (10594, 10595);
const CANCEL: (u32, u32) = (10596, 10597);
const ENCYCLOPEDIA: (u32, u32) = (10592, 10593);
const MORE: (u32, u32) = (10610, 10611);
const FEWER: (u32, u32) = (10612, 10613);

// TEXTSTRA strings (FUN_00438500, FUN_00438dd0, FUN_00438f30).
/// 14352, the title.
const TITLE: &str = "Build Selection";
/// 14353.
const NUMBER_TO_BUILD: &str = "Number to build:";
/// 14354.
const BEST_COMPLETION: &str = "Best Time To Completion:";
/// 14355.
const BEST_DEPLOYMENT: &str = "Best Time To Deployment:";
/// 14357, after a time's number.
const DAYS: &str = " Days";
/// 14358, a value of an order that cannot be built.
const NOT_AVAILABLE: &str = "n/a";

// Geometry (FUN_00437f80, FUN_00438500, FUN_00438620), in window pixels.
/// The selected class, 195 by 63 at (6, 22) (list `FUN_0060bed0`).
const ITEM_BOX: (f32, f32, f32, f32) = (6.0, 22.0, 195.0, 63.0);
/// Refined material (`+0x134`) and maintenance (`+0x138`), 64 by 23.
const REFINED_FIELD: (f32, f32) = (36.0, 110.0);
const MAINTENANCE_FIELD: (f32, f32) = (138.0, 110.0);
/// Best completion (`+0x12c`) and deployment (`+0x130`), 60 by 15.
const COMPLETION_FIELD: (f32, f32) = (140.0, 145.0);
const DEPLOYMENT_FIELD: (f32, f32) = (140.0, 165.0);
/// The number to build, an edit of 45 by 17 (`+0x128`).
const QUANTITY_FIELD: (f32, f32, f32, f32) = (141.0, 196.0, 45.0, 17.0);

/// The drop-down's rows (box `+0xa8`, `FUN_00437f80`'s 2) and their height
/// (`+0xa4`, 0x46); `FUN_0060cac0` makes it the rows' height plus 4.
const LIST_ROWS: usize = 2;
const LIST_ROW_HEIGHT: f32 = 70.0;

/// The labels' color, `0x2f0fbff` (`FUN_00438500`).
const LABEL: egui::Color32 = egui::Color32::from_rgb(0xff, 0xfb, 0xf0);
/// A list row's name unless picked or under the pointer: item `+0x60`,
/// 0x808080 (`FUN_00437880`); the other two states keep white.
const ROW_GREY: egui::Color32 = egui::Color32::from_rgb(0x80, 0x80, 0x80);

/// The most units one order builds (`FUN_00438c30`, `FUN_00438c60`).
pub const MAX_QUANTITY: u32 = 255;

/// One open window: the band it orders for and its controls' state.
#[derive(Debug, Clone, PartialEq)]
pub struct BuildSelection {
    system: SystemKey,
    area: ProductionArea,
    is_alliance: bool,
    /// The classes `FUN_0052e580` listed when the window was built.
    kinds: Vec<BuildableKind>,
    selected: usize,
    quantity: u32,
    /// The class list is open (`CoolSelectionBoxClass +0xe8`).
    list_open: bool,
    /// The list's first shown row (its scroll, list `+0xc4`).
    top: usize,
    /// The row last clicked in the list: its item's `+0x3c` bit 0, which
    /// `FUN_006094b0` clears on every other item.
    picked: Option<usize>,
    /// Each row's item `+0x3c` bit 1, set while the pointer is on it. The
    /// items outlive the list (`FUN_00608280` shares the box's), so a bit
    /// stays until a list moves the pointer off its row.
    hot: Vec<bool>,
    /// The row this list marked hot (list `+0x100`); a new list has none.
    last_hot: Option<usize>,
}

impl BuildSelection {
    #[must_use]
    pub fn area(&self) -> ProductionArea {
        self.area
    }

    #[must_use]
    pub fn kind(&self) -> BuildableKind {
        self.kinds[self.selected]
    }

    #[must_use]
    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    #[must_use]
    pub fn list_open(&self) -> bool {
        self.list_open
    }

    /// Choose a listed class; an index past the list is ignored.
    pub fn select(&mut self, index: usize) {
        if index < self.kinds.len() {
            self.selected = index;
        }
    }

    /// Quantity up (`0x69`, keypad `+`) or down (`0x6a`, keypad `-`), held
    /// to 1 through 255.
    pub fn step_quantity(&mut self, up: bool) {
        self.quantity = if up {
            (self.quantity + 1).min(MAX_QUANTITY)
        } else {
            self.quantity.saturating_sub(1).max(1)
        };
    }

    /// A digit typed into the number edit; the value stays in 1..=255.
    pub fn type_digit(&mut self, digit: u32) {
        self.quantity = (self.quantity * 10 + digit).clamp(1, MAX_QUANTITY);
    }

    /// Backspace in the number edit.
    pub fn erase_digit(&mut self) {
        self.quantity = (self.quantity / 10).max(1);
    }

    /// The drop-down (`0x6c`): open a new list scrolled to the class
    /// (`FUN_00609ed0` from 0 brings it up to the last row), or close it.
    fn toggle_list(&mut self) {
        self.list_open = !self.list_open;
        if self.list_open {
            self.top = self.selected.saturating_sub(LIST_ROWS - 1);
            self.last_hot = None;
        }
    }

    /// The pointer moved over the list (`FUN_006083c0`, `WM_MOUSEMOVE`):
    /// the row it left loses bit 1 and the row under it, if any, gains it.
    fn hover(&mut self, row: Option<usize>) {
        if row == self.last_hot {
            return;
        }
        if let Some(old) = self.last_hot {
            self.hot[old] = false;
        }
        self.last_hot = row;
        if let Some(row) = row {
            self.hot[row] = true;
        }
    }

    /// A click in the list (`FUN_006083c0`, buttons up): the row's class is
    /// picked and the list closes (notification `0x29b` -> `FUN_0060c970`);
    /// off the rows only the old pick clears.
    fn click_list(&mut self, row: Option<usize>) {
        self.picked = row;
        if let Some(row) = row {
            self.select(row);
            self.list_open = false;
        }
    }

    /// A row's name color (`FUN_006083c0`, `WM_PAINT`).
    fn row_color(&self, row: usize) -> egui::Color32 {
        if self.picked == Some(row) || self.hot[row] {
            egui::Color32::WHITE
        } else {
            ROW_GREY
        }
    }
}

/// Whether a Build Selection window is open, and the class each production
/// area last confirmed.
#[derive(Debug, Clone, Default)]
pub struct BuildSelectionState {
    dialog: Option<BuildSelection>,
    /// `FUN_00437f80` reopens on the class last built by the same kind of
    /// manager (`DAT_006b289c` facilities, `DAT_006b28a0` troops,
    /// `DAT_006b28a4` ships; set by `FUN_00438980`).
    remembered: [Option<BuildableKind>; 3],
}

const fn area_index(area: ProductionArea) -> usize {
    match area {
        ProductionArea::Shipyard => 0,
        ProductionArea::TrainingFacility => 1,
        ProductionArea::ConstructionYard => 2,
    }
}

impl BuildSelectionState {
    /// Open the window for a band with the classes its manager lists. With
    /// none, nothing opens. The quantity starts at 1 (`FUN_00437f80`).
    pub fn open(
        &mut self,
        system: SystemKey,
        area: ProductionArea,
        is_alliance: bool,
        kinds: Vec<BuildableKind>,
    ) -> bool {
        if kinds.is_empty() {
            return false;
        }
        let selected = self.remembered[area_index(area)]
            .and_then(|kind| kinds.iter().position(|&listed| listed == kind))
            .unwrap_or(0);
        let hot = vec![false; kinds.len()];
        self.dialog = Some(BuildSelection {
            system,
            area,
            is_alliance,
            kinds,
            selected,
            quantity: 1,
            list_open: false,
            top: 0,
            picked: None,
            hot,
            last_hot: None,
        });
        true
    }

    #[must_use]
    pub fn is_open(&self) -> bool {
        self.dialog.is_some()
    }

    #[must_use]
    pub fn dialog(&self) -> Option<&BuildSelection> {
        self.dialog.as_ref()
    }

    pub fn dialog_mut(&mut self) -> Option<&mut BuildSelection> {
        self.dialog.as_mut()
    }

    /// Close and Cancel discard the order (`FUN_00438800`, 100 and 102).
    pub fn close(&mut self) {
        self.dialog = None;
    }

    /// Whether `point` falls on the open window, so the galaxy map under it
    /// takes no input.
    #[must_use]
    pub fn contains_screen_point(&self, layout: CockpitLayout, point: (f32, f32)) -> bool {
        self.dialog.is_some() && rect_contains(window_rect(layout), egui::pos2(point.0, point.1))
    }

    /// Confirm (`0x65`, `FUN_00438980`): remember the class for its kind of
    /// manager and submit the order; the window closes.
    pub fn confirm(&mut self) -> Option<BuildSelectionAction> {
        let dialog = self.dialog.take()?;
        let kind = dialog.kind();
        self.remembered[area_index(dialog.area)] = Some(kind);
        Some(BuildSelectionAction::Confirm {
            system: dialog.system,
            area: dialog.area,
            kind,
            count: dialog.quantity,
        })
    }
}

/// What the window asks of the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildSelectionAction {
    /// Build `count` units of `kind` at the band (`FUN_0041ce20`).
    Confirm {
        system: SystemKey,
        area: ProductionArea,
        kind: BuildableKind,
        count: u32,
    },
    /// Encyclopedia (`0x67`, `FUN_0041d6b0`). port: it opens the
    /// Encyclopedia; the selected class's entry is not bound yet.
    Encyclopedia,
}

/// The window's screen rectangle. hyp: placed in the galaxy view like the
/// mission dialog, centered.
#[must_use]
pub fn window_rect(layout: CockpitLayout) -> egui::Rect {
    galaxy_centered_rect(layout, BUILD_SELECTION_WIDTH, BUILD_SELECTION_HEIGHT)
}

/// A cost as `FUN_00438dd0` writes it: capped at 9999, or "n/a".
#[must_use]
pub fn cost_text(value: u32, valid: bool) -> String {
    if valid {
        value.min(9999).to_string()
    } else {
        NOT_AVAILABLE.to_owned()
    }
}

/// A time as `FUN_00438f30` writes it: capped at 9999 with " Days", or
/// "n/a".
#[must_use]
pub fn days_text(value: u32, valid: bool) -> String {
    if valid {
        format!("{}{DAYS}", value.min(9999))
    } else {
        NOT_AVAILABLE.to_owned()
    }
}

/// Draw the open window, if any, and report what the player asked for.
#[expect(
    clippy::too_many_lines,
    reason = "The window paints in the original's control order; splitting it hides that order."
)]
#[expect(
    clippy::too_many_arguments,
    reason = "The validator reads the queues, the cargo aboard and the products on their way."
)]
pub fn draw_build_selection(
    ctx: &egui::Context,
    world: &GameWorld,
    manufacturing: &ManufacturingState,
    transport: &TroopTransportState,
    deliveries: &DeliveryState,
    movement: &MovementState,
    state: &mut BuildSelectionState,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Option<BuildSelectionAction> {
    let dialog = state.dialog.as_mut()?;
    let scale = layout.scale;
    let window = window_rect(layout);
    let estimate: BuildEstimate = estimate(
        world,
        manufacturing,
        dialog.system,
        dialog.area,
        dialog.kind(),
        dialog.quantity,
        dialog.is_alliance,
    );
    // FUN_00439160: FUN_00538220 runs the order's validator (slot +0x1c,
    // FUN_0051ff30 -> FUN_005385f0), whose refusals (`order_refusal`) make
    // the order invalid as a missing yard does.
    let valid = estimate.completion.is_some()
        && order_refusal(
            world,
            manufacturing,
            transport,
            deliveries,
            dialog.system,
            dialog.area,
            dialog.kind(),
            dialog.quantity,
            dialog.is_alliance,
            movement,
        )
        .is_none();
    let mut action = None;
    let mut close = false;
    let mut confirm = false;

    // FUN_00438b60: Enter confirms, even an invalid order, which its
    // validator then refuses; Escape closes, keypad + and - step the
    // number; the number edit has the focus for digits.
    ctx.input(|input| {
        for event in &input.events {
            if let egui::Event::Key {
                key, pressed: true, ..
            } = event
            {
                match key {
                    egui::Key::Enter => confirm = true,
                    egui::Key::Escape => close = true,
                    egui::Key::Plus | egui::Key::Equals => dialog.step_quantity(true),
                    egui::Key::Minus => dialog.step_quantity(false),
                    egui::Key::Backspace => dialog.erase_digit(),
                    _ => {}
                }
            }
            if let egui::Event::Text(text) = event {
                for digit in text.chars().filter_map(|c| c.to_digit(10)) {
                    dialog.type_digit(digit);
                }
            }
        }
    });

    egui::Area::new(egui::Id::new("original-build-selection"))
        .fixed_pos(window.min)
        .order(egui::Order::Tooltip)
        .show(ctx, |ui| {
            let (frame, _) = ui.allocate_exact_size(window.size(), egui::Sense::hover());
            let painter = ui.painter().with_clip_rect(frame);
            let at = |x: f32, y: f32, w: f32, h: f32| logical_rect(frame, scale, x, y, w, h);
            let title_font = egui::FontId::proportional((11.0 * scale).max(7.0));
            let value_font = egui::FontId::proportional((10.0 * scale).max(7.0));
            let small = egui::FontId::proportional((9.0 * scale).max(6.0));

            paint(&painter, ctx, cache, BACKGROUND, frame, scale);
            // FUN_00438500: the side's 240-pixel strip at (2, 2), which the
            // window clips, then the labels.
            let strip = if dialog.is_alliance {
                TITLE_ALLIANCE
            } else {
                TITLE_EMPIRE
            };
            paint(
                &painter,
                ctx,
                cache,
                strip,
                at(2.0, 2.0, 208.0, 17.0),
                scale,
            );
            painter.text(
                at(2.0, 2.0, 202.0, 14.0).center(),
                egui::Align2::CENTER_CENTER,
                TITLE,
                title_font,
                egui::Color32::BLACK,
            );
            painter.text(
                at(20.0, 196.0, 118.0, 15.0).right_top(),
                egui::Align2::RIGHT_TOP,
                NUMBER_TO_BUILD,
                value_font.clone(),
                LABEL,
            );
            for (label, y) in [(BEST_COMPLETION, 145.0), (BEST_DEPLOYMENT, 165.0)] {
                painter.text(
                    at(10.0, y, 125.0, 15.0).left_center(),
                    egui::Align2::LEFT_CENTER,
                    label,
                    small.clone(),
                    LABEL,
                );
            }

            // The selected class: its portrait centered in the box, its name
            // from the portrait's height down (FUN_00437880's 195 by 63 item).
            let (x, y, w, h) = ITEM_BOX;
            paint_item(
                &painter,
                ctx,
                cache,
                world,
                dialog.kind(),
                at(x, y, w, h),
                scale,
                &small,
                egui::Color32::WHITE,
            );

            // FUN_00438dd0 / FUN_00438f30: the values, white, each in its
            // FUN_00437f80 field. port: clipped to the field, where the
            // original's aliased GDI text ends; the anti-aliased edge of a
            // right-aligned value would otherwise cross it.
            let in_field = |field: egui::Rect| painter.with_clip_rect(field.intersect(painter.clip_rect()));
            for ((x, y), text) in [
                (REFINED_FIELD, cost_text(estimate.refined_material, valid)),
                (MAINTENANCE_FIELD, cost_text(estimate.maintenance, valid)),
            ] {
                let field = at(x, y, 64.0, 23.0);
                in_field(field).text(
                    field.center(),
                    egui::Align2::CENTER_CENTER,
                    text,
                    value_font.clone(),
                    egui::Color32::WHITE,
                );
            }
            for ((x, y), text) in [
                (
                    COMPLETION_FIELD,
                    days_text(estimate.completion.unwrap_or(0), valid),
                ),
                (DEPLOYMENT_FIELD, days_text(estimate.deployment, valid)),
            ] {
                let field = at(x, y, 60.0, 15.0);
                in_field(field).text(
                    field.right_center(),
                    egui::Align2::RIGHT_CENTER,
                    text,
                    small.clone(),
                    egui::Color32::WHITE,
                );
            }
            let (x, y, w, h) = QUANTITY_FIELD;
            in_field(at(x, y, w, h)).text(
                at(x, y, w, h).left_center(),
                egui::Align2::LEFT_CENTER,
                dialog.quantity.to_string(),
                value_font.clone(),
                egui::Color32::WHITE,
            );

            if button(
                ui,
                cache,
                at(193.0, 3.0, 14.0, 14.0),
                "close",
                CLOSE,
                false,
                scale,
            ) {
                close = true;
            }
            if button(
                ui,
                cache,
                at(79.0, 90.0, 65.0, 18.0),
                "list",
                LIST,
                false,
                scale,
            ) {
                dialog.toggle_list();
            }
            if button(
                ui,
                cache,
                at(189.0, 196.0, 13.0, 8.0),
                "more",
                MORE,
                false,
                scale,
            ) {
                dialog.step_quantity(true);
            }
            if button(
                ui,
                cache,
                at(189.0, 205.0, 13.0, 8.0),
                "fewer",
                FEWER,
                false,
                scale,
            ) {
                dialog.step_quantity(false);
            }
            if button(
                ui,
                cache,
                at(5.0, 224.0, 66.0, 33.0),
                "encyclopedia",
                ENCYCLOPEDIA,
                false,
                scale,
            ) {
                action = Some(BuildSelectionAction::Encyclopedia);
            }
            // FUN_00439160 disables Confirm for an order that cannot be
            // built. port: a disabled button keeps its rest art.
            let confirm_rect = at(73.0, 224.0, 66.0, 33.0);
            if valid {
                if button(ui, cache, confirm_rect, "confirm", CONFIRM, false, scale) {
                    confirm = true;
                }
            } else {
                paint_button(&painter, ctx, cache, CONFIRM.0, confirm_rect, scale);
            }
            if button(
                ui,
                cache,
                at(141.0, 224.0, 66.0, 33.0),
                "cancel",
                CANCEL,
                false,
                scale,
            ) {
                close = true;
            }

            // The drop-down (FUN_0060cac0): a window under the box, the box's
            // width and two rows high, its list (FUN_00607ea0) 2 pixels in.
            // port: its scroll bar (base 0x299a) is not drawn; the wheel
            // scrolls the rows, as the mission dialog's.
            if dialog.list_open {
                // FUN_0060d020 takes the capture: a press off the popup
                // closes it and reaches no other control. port: on the
                // release, within the window.
                let capture =
                    ui.interact(frame, ui.id().with("class-list-capture"), egui::Sense::click());
                let (x, y, w, h) = ITEM_BOX;
                let rows_height = LIST_ROWS as f32 * LIST_ROW_HEIGHT;
                let popup = at(x, y + h, w, rows_height + 4.0);
                let list = at(x + 2.0, y + h + 2.0, w - 4.0, rows_height);
                painter.rect_filled(popup, 0.0, egui::Color32::BLACK);
                let response = ui.interact(popup, ui.id().with("class-list"), egui::Sense::click());
                let len = dialog.kinds.len();
                let top = dialog.top;
                let row_at = |point: egui::Pos2| {
                    let row = top + ((point.y - list.min.y) / (LIST_ROW_HEIGHT * scale)) as usize;
                    (row < len).then_some(row)
                };
                // The list sees the pointer only over itself.
                if let Some(point) = ctx
                    .input(|input| input.pointer.hover_pos())
                    .filter(|&point| rect_contains(list, point))
                {
                    dialog.hover(row_at(point));
                }
                if response.hovered() {
                    let wheel = ctx.input(|input| input.raw_scroll_delta.y);
                    dialog.top = scrolled(dialog.top, len.saturating_sub(LIST_ROWS) + 1, wheel);
                }
                for (slot, row) in (dialog.top..len).take(LIST_ROWS).enumerate() {
                    paint_item(
                        &painter,
                        ctx,
                        cache,
                        world,
                        dialog.kinds[row],
                        at(
                            x + 2.0,
                            y + h + 2.0 + slot as f32 * LIST_ROW_HEIGHT,
                            w - 4.0,
                            h,
                        ),
                        scale,
                        &small,
                        dialog.row_color(row),
                    );
                }
                if exact_clicked(&response, list) {
                    let row = response.interact_pointer_pos().and_then(row_at);
                    dialog.click_list(row);
                } else if capture.clicked() {
                    dialog.toggle_list();
                }
            }
        });

    if confirm {
        return state.confirm();
    }
    if close {
        state.close();
    }
    action
}

/// The GOKRES picture a class's list item holds: the class's portrait,
/// `+0x30 & 0xfff` (`FUN_00437880.c:94-102`), the band's picture
/// (`FUN_00457c90.c:48-55`), not its `+0x4000` mini.
fn item_picture(world: &GameWorld, kind: BuildableKind) -> Option<u32> {
    product(world, kind)
        .1
        .map(crate::fleet_window::ship_portrait)
}

/// Where an item's name starts: its picture's height below the item's top
/// (`FUN_00437880.c:138-139` stores the height as the item's text offset,
/// `+0x34`, which the box and the list add to the text rect's top,
/// `FUN_0060c210.c:60-74`, `FUN_006083c0.c:124-127`). port: with no
/// picture loaded, the item's middle.
fn name_top(bounds: egui::Rect, picture: Option<egui::Rect>) -> f32 {
    picture.map_or(bounds.center().y, |rect| bounds.min.y + rect.height())
}

/// One class: its GOKRES portrait centered in the box (`FUN_00437880.c:
/// 128-133`) and its name from [`name_top`], centered across the item
/// (`DT_CENTER | DT_NOCLIP`, `0x101`, `FUN_00437f80.c:55`), in `color`.
#[expect(
    clippy::too_many_arguments,
    reason = "Keep explicit painter, cache and geometry inputs at this drawing boundary."
)]
fn paint_item(
    painter: &egui::Painter,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    world: &GameWorld,
    kind: BuildableKind,
    bounds: egui::Rect,
    scale: f32,
    font: &egui::FontId,
    color: egui::Color32,
) {
    let name = product(world, kind).0;
    let image = item_picture(world, kind)
        .and_then(|id| paint_centered(painter, ctx, cache, DllSource::Gokres, id, bounds, scale));
    painter.text(
        egui::pos2(bounds.center().x, name_top(bounds, image)),
        egui::Align2::CENTER_TOP,
        name,
        font.clone(),
        color,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fleet_window::tests::PAINTED;
    use rebellion_core::dat::{ExplorationStatus, Faction};
    use rebellion_core::ids::DatId;
    use rebellion_core::world::{
        BuildableClass, ControlKind, ManufacturingFacilityInstance, System,
    };

    fn layout() -> CockpitLayout {
        scaled(1.0)
    }

    fn scaled(scale: f32) -> CockpitLayout {
        use crate::cockpit::CockpitViewport;
        CockpitLayout {
            canvas: CockpitViewport {
                x: 10.0,
                y: 20.0,
                width: 640.0 * scale,
                height: 480.0 * scale,
            },
            galaxy: CockpitViewport {
                x: 10.0 + 55.0 * scale,
                y: 20.0 + 40.0 * scale,
                width: 485.0 * scale,
                height: 350.0 * scale,
            },
            scale,
        }
    }

    /// A drawn text: its string, position and font size.
    /// A painted text, the top centre of its galley, its size and color.
    type Text = (String, egui::Pos2, f32, egui::Color32);

    /// A system of the Alliance's with a training facility (period 4) of
    /// each side when `yard`, the regiment class TROOPSD 1 (cost 8,
    /// maintenance 6), and a mine and refinery pair of each side (50
    /// maintenance capacity).
    fn galaxy(yard: bool) -> (GameWorld, SystemKey) {
        let mut world = GameWorld::default();
        let system = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0001),
            name: "Bortras".into(),
            sector: rebellion_core::ids::SectorKey::default(),
            x: 0,
            y: 0,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Alliance),
        });
        let class = |refined, maintenance, rate| BuildableClass {
            is_alliance: true,
            is_empire: true,
            refined_material_cost: refined,
            maintenance_cost: maintenance,
            processing_rate: rate,
            ..BuildableClass::default()
        };
        world
            .buildable_classes
            .insert(DatId::new(0x2900_0002), class(10, 10, 4));
        world
            .buildable_classes
            .insert(DatId::new(0x1000_0001), class(8, 6, 0));
        for side in [Faction::Alliance, Faction::Empire] {
            if yard {
                let key = world
                    .manufacturing_facilities
                    .insert(ManufacturingFacilityInstance {
                        class_dat_id: DatId::new(0x2900_0002),
                        side,
                        is_shipyard: false,
                    });
                world.systems[system].manufacturing_facilities.push(key);
            }
            for is_mine in [true, false] {
                world
                    .production_facilities
                    .insert(rebellion_core::world::ProductionFacilityInstance {
                        class_dat_id: DatId::new(0x2c00_0001),
                        side,
                        is_mine,
                    });
            }
        }
        (world, system)
    }

    fn opened(system: SystemKey, is_alliance: bool) -> BuildSelectionState {
        let mut state = BuildSelectionState::default();
        assert!(state.open(
            system,
            ProductionArea::TrainingFacility,
            is_alliance,
            vec![BuildableKind::Troop(DatId::new(0x1000_0001))],
        ));
        state
    }

    fn press(pos: egui::Pos2, pressed: bool) -> egui::Event {
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

    /// Two idle frames, then one per entry of `frames`, the pointer at
    /// window pixel `(x, y)`; every button event lands there.
    fn drive(
        world: &GameWorld,
        state: &mut BuildSelectionState,
        point: (f32, f32),
        frames: Vec<Vec<egui::Event>>,
    ) -> Option<BuildSelectionAction> {
        drive_at(world, state, 1.0, point, frames).0
    }

    /// `drive` at a layout scale, also returning the last frame's texts.
    fn drive_at(
        world: &GameWorld,
        state: &mut BuildSelectionState,
        scale: f32,
        (x, y): (f32, f32),
        frames: Vec<Vec<egui::Event>>,
    ) -> (Option<BuildSelectionAction>, Vec<Text>) {
        let layout = scaled(scale);
        let pos = window_rect(layout).min + egui::vec2(x, y) * scale;
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let manufacturing = ManufacturingState::new();
        let mut emitted = None;
        let mut texts = Vec::new();
        for extra in [vec![], vec![]].into_iter().chain(frames) {
            let mut events = vec![egui::Event::PointerMoved(pos)];
            events.extend(extra.into_iter().map(|event| match event {
                egui::Event::PointerButton { pressed, .. } => press(pos, pressed),
                other => other,
            }));
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1400.0, 1040.0),
                )),
                events,
                ..Default::default()
            };
            let output = ctx.run(input, |ctx| {
                if let Some(action) = draw_build_selection(
                    ctx,
                    world,
                    &manufacturing,
                    &TroopTransportState::default(),
                    &DeliveryState::default(),
                    &MovementState::new(),
                    state,
                    layout,
                    &mut cache,
                ) {
                    emitted = Some(action);
                }
            });
            texts = output
                .shapes
                .into_iter()
                .filter_map(|clipped| match clipped.shape {
                    egui::Shape::Text(text) => Some((
                        text.galley.text().to_owned(),
                        text.pos + egui::vec2(text.galley.size().x / 2.0, 0.0),
                        text.galley.job.sections[0].format.font_id.size,
                        text.galley.job.sections[0].format.color,
                    )),
                    _ => None,
                })
                .collect();
        }
        (emitted, texts)
    }

    fn click(
        world: &GameWorld,
        state: &mut BuildSelectionState,
        point: (f32, f32),
    ) -> Option<BuildSelectionAction> {
        let at = egui::Pos2::ZERO;
        drive(
            world,
            state,
            point,
            vec![vec![press(at, true)], vec![press(at, false)]],
        )
    }

    // FUN_00437880.c:94-102 loads each item's picture from GOKRES at the
    // class's +0x30 & 0xfff (FUN_005fbd20(.., id & 0xfff, 2)), the portrait,
    // not its +0x4000 mini, and centres it in a 0xc3 by 0x3f (195 by 63)
    // buffer (:111-131); the box shows the picked item (FUN_00608280).
    #[test]
    fn each_class_in_the_list_and_box_shows_its_portrait() {
        use crate::panels::fleets::capital_ship_mini_id;
        let ship = DatId::new(0x1400_0040);
        let mut world = GameWorld::default();
        let class = world
            .capital_ship_classes
            .insert(rebellion_core::world::CapitalShipClass {
                dat_id: ship,
                ..Default::default()
            });
        for (kind, portrait) in [
            (
                BuildableKind::CapitalShip(class),
                capital_ship_mini_id(ship).map(|mini| mini - 0x4000),
            ),
            (BuildableKind::Troop(DatId::new(0x1000_0001)), Some(0x440)),
            (
                BuildableKind::ProductionFacility(rebellion_core::manufacturing::FacilityBuild {
                    class: DatId::new(0x2c00_0001),
                    is_alliance: true,
                }),
                Some(1),
            ),
        ] {
            assert!(portrait.is_some(), "{kind:?}");
            assert_eq!(item_picture(&world, kind), portrait, "{kind:?}");
        }
    }

    // FUN_00437880.c:138-139 sets each item's text offset (+0x34) to its
    // picture's height; the box (FUN_0060c210.c:60-74) and the list
    // (FUN_006083c0.c:124-127) add it to the top of the item's text rect,
    // drawn with DT_CENTER | DT_NOCLIP (0x101, FUN_00437f80.c:55) and no
    // vertical alignment. So the name starts the picture's height below the
    // item's top, inside the centred picture, not under it.
    #[test]
    fn an_items_name_starts_its_pictures_height_below_the_items_top() {
        let item = egui::Rect::from_min_size(egui::pos2(6.0, 22.0), egui::vec2(195.0, 63.0));
        // A 122 by 50 portrait centred in the item, (63 - 50) / 2 = 6 down.
        let picture = egui::Rect::from_min_size(egui::pos2(42.0, 28.0), egui::vec2(122.0, 50.0));
        assert_eq!(name_top(item, Some(picture)), 22.0 + 50.0);
    }

    // FUN_00437df0: background 10800 at (0, 0); FUN_00438500: the side's
    // strip at (2, 2), 10801 for side 1 and 10802 for the other;
    // FUN_00438620: Confirm 10594 at (73, 224).
    #[test]
    fn the_window_paints_its_background_side_strip_and_controls() {
        let (world, system) = galaxy(true);
        for (is_alliance, strip) in [(true, 10_801), (false, 10_802)] {
            PAINTED.with(|painted| painted.borrow_mut().clear());
            let mut state = opened(system, is_alliance);
            drive(&world, &mut state, (0.0, 0.0), vec![]);
            let origin = window_rect(layout()).min;
            let painted = PAINTED.with(|painted| painted.borrow().clone());
            for (id, x, y) in [
                (10_800, 0.0, 0.0),
                (strip, 2.0, 2.0),
                (10_606, 79.0, 90.0),
                (10_592, 5.0, 224.0),
                (10_594, 73.0, 224.0),
                (10_596, 141.0, 224.0),
                (10_610, 189.0, 196.0),
                (10_612, 189.0, 205.0),
            ] {
                assert!(
                    painted.contains(&(id, origin + egui::vec2(x, y))),
                    "{id} at ({x}, {y})"
                );
            }
        }
    }

    // FUN_00438800 case 0x65 confirms; FUN_00439160 disables Confirm when
    // the manager has no yard to build with.
    #[test]
    fn confirm_orders_the_build_only_when_a_yard_can_build_it() {
        let (world, system) = galaxy(true);
        let mut state = opened(system, true);
        assert_eq!(
            click(&world, &mut state, (100.0, 240.0)),
            Some(BuildSelectionAction::Confirm {
                system,
                area: ProductionArea::TrainingFacility,
                kind: BuildableKind::Troop(DatId::new(0x1000_0001)),
                count: 1,
            })
        );
        assert!(!state.is_open());

        let (world, system) = galaxy(false);
        let mut state = opened(system, true);
        assert_eq!(click(&world, &mut state, (100.0, 240.0)), None);
        assert!(state.is_open());
        // FUN_00438b60 case 0xd: Enter calls FUN_00438980 whatever the
        // button's state, then closes; FUN_0041ce20 issues the order and its
        // validator refuses it (mission-dialog.md, "Refusal").
        assert!(matches!(
            drive(
                &world,
                &mut state,
                (0.0, 0.0),
                vec![vec![key(egui::Key::Enter)]]
            ),
            Some(BuildSelectionAction::Confirm { .. })
        ));
        assert!(!state.is_open());
    }

    // FUN_00439160 -> FUN_00538220 -> the order's slot +0x1c (FUN_0051ff30)
    // -> slot +0x50 (FUN_005385f0) -> FUN_0052e970: an order whose
    // maintenance exceeds the side's spare is invalid, so all four fields
    // read 14358 "n/a" and Confirm is disabled; equality passes.
    #[test]
    fn an_order_over_the_spare_maintenance_reads_na_and_cannot_be_confirmed() {
        let (world, system) = galaxy(true);
        for side in [Faction::Alliance, Faction::Empire] {
            let spare = rebellion_core::resources::maintenance_surplus(
                &world,
                &ManufacturingState::new(),
                side,
            );
            // Six maintenance a regiment.
            let fits = u32::try_from(spare / 6).unwrap();
            assert!(fits > 1, "the fixture leaves room for a regiment");
            for (count, valid) in [(fits, true), (fits + 1, false)] {
                let mut state = opened(system, side == Faction::Alliance);
                state.dialog_mut().unwrap().quantity = count;
                let (_, texts) = drive_at(&world, &mut state, 1.0, (0.0, 0.0), vec![]);
                let na = texts.iter().filter(|(text, ..)| text == "n/a").count();
                assert_eq!(na, if valid { 0 } else { 4 }, "{side:?} x{count}");
                let confirmed = click(&world, &mut state, (100.0, 240.0));
                assert_eq!(confirmed.is_some(), valid, "{side:?} x{count}");
                assert_eq!(state.is_open(), !valid);
            }
        }
    }

    // FUN_00438b60: Enter confirms, Escape closes, keypad + steps the
    // number; FUN_00438800: Cancel (0x66) discards.
    #[test]
    fn keys_and_cancel_drive_the_window() {
        let (world, system) = galaxy(true);
        let mut state = opened(system, true);
        drive(
            &world,
            &mut state,
            (0.0, 0.0),
            vec![vec![key(egui::Key::Plus)]],
        );
        assert_eq!(state.dialog().unwrap().quantity(), 2);
        let digit = egui::Event::Text("4".into());
        drive(&world, &mut state, (0.0, 0.0), vec![vec![digit]]);
        assert_eq!(state.dialog().unwrap().quantity(), 24);
        drive(
            &world,
            &mut state,
            (0.0, 0.0),
            vec![vec![key(egui::Key::Backspace)]],
        );
        assert_eq!(state.dialog().unwrap().quantity(), 2);
        drive(
            &world,
            &mut state,
            (0.0, 0.0),
            vec![vec![key(egui::Key::Minus)]],
        );
        assert_eq!(state.dialog().unwrap().quantity(), 1);
        drive(
            &world,
            &mut state,
            (0.0, 0.0),
            vec![vec![key(egui::Key::Plus)]],
        );
        let confirmed = drive(
            &world,
            &mut state,
            (0.0, 0.0),
            vec![vec![key(egui::Key::Enter)]],
        );
        assert!(matches!(
            confirmed,
            Some(BuildSelectionAction::Confirm { count: 2, .. })
        ));

        let mut state = opened(system, true);
        drive(
            &world,
            &mut state,
            (0.0, 0.0),
            vec![vec![key(egui::Key::Escape)]],
        );
        assert!(!state.is_open());

        let mut state = opened(system, true);
        assert_eq!(click(&world, &mut state, (170.0, 240.0)), None);
        assert!(!state.is_open());
    }

    fn kinds() -> Vec<BuildableKind> {
        vec![
            BuildableKind::Troop(DatId::new(0x1000_0001)),
            BuildableKind::Troop(DatId::new(0x1000_0002)),
        ]
    }

    fn open() -> BuildSelectionState {
        let mut state = BuildSelectionState::default();
        assert!(state.open(
            SystemKey::default(),
            ProductionArea::TrainingFacility,
            true,
            kinds()
        ));
        state
    }

    #[test]
    fn a_manager_with_nothing_to_build_opens_no_window() {
        let mut state = BuildSelectionState::default();
        assert!(!state.open(
            SystemKey::default(),
            ProductionArea::TrainingFacility,
            true,
            Vec::new()
        ));
        assert!(!state.is_open());
    }

    // FUN_00438c30 / FUN_00438c60: the number to build stays in 1..=255.
    #[test]
    fn the_number_to_build_starts_at_one_and_stays_within_1_to_255() {
        let mut state = open();
        let dialog = state.dialog_mut().unwrap();
        assert_eq!(dialog.quantity(), 1);
        dialog.step_quantity(false);
        assert_eq!(dialog.quantity(), 1);
        dialog.step_quantity(true);
        assert_eq!(dialog.quantity(), 2);
        dialog.type_digit(5);
        assert_eq!(dialog.quantity(), 25);
        dialog.type_digit(9);
        assert_eq!(dialog.quantity(), 255);
        dialog.step_quantity(true);
        assert_eq!(dialog.quantity(), 255);
        dialog.erase_digit();
        assert_eq!(dialog.quantity(), 25);
        dialog.erase_digit();
        dialog.erase_digit();
        assert_eq!(dialog.quantity(), 1);
    }

    // FUN_00438980 remembers the confirmed class for its kind of manager;
    // FUN_00437f80 reopens on it.
    #[test]
    fn confirm_submits_the_order_and_the_next_window_reopens_on_its_class() {
        let mut state = open();
        let dialog = state.dialog_mut().unwrap();
        dialog.select(1);
        dialog.select(2);
        dialog.select(9);
        dialog.step_quantity(true);
        let action = state.confirm();
        assert_eq!(
            action,
            Some(BuildSelectionAction::Confirm {
                system: SystemKey::default(),
                area: ProductionArea::TrainingFacility,
                kind: kinds()[1],
                count: 2,
            })
        );
        assert!(!state.is_open());
        assert!(state.open(
            SystemKey::default(),
            ProductionArea::TrainingFacility,
            true,
            kinds()
        ));
        assert_eq!(state.dialog().unwrap().kind(), kinds()[1]);
        // Another kind of manager keeps its own.
        assert!(state.open(
            SystemKey::default(),
            ProductionArea::Shipyard,
            true,
            kinds()
        ));
        assert_eq!(state.dialog().unwrap().kind(), kinds()[0]);
    }

    /// The drop-down's rows, top to bottom, with their colors: the class
    /// names drawn below the class box (window y 85 on).
    fn list_rows(texts: &[Text], names: &[String]) -> Vec<(String, egui::Color32)> {
        let below = window_rect(layout()).min.y + 85.0;
        let mut rows: Vec<_> = texts
            .iter()
            .filter(|(text, top, ..)| top.y > below && names.contains(text))
            .map(|(text, top, _, color)| (top.y, text.clone(), *color))
            .collect();
        rows.sort_by(|a, b| a.0.total_cmp(&b.0));
        rows.into_iter().map(|(_, text, color)| (text, color)).collect()
    }

    const GREY: egui::Color32 = egui::Color32::from_rgb(0x80, 0x80, 0x80);

    // The drop-down (command 0x6c, FUN_0060c1f0 -> FUN_0060cac0) hangs
    // under the class box, 195 wide and two 70-pixel rows plus 4 high
    // (box +0xa8 = 2 from FUN_00437f80, +0xa4 = 0x46); its list
    // (FUN_00607ea0) sits 2 in. A click on a row picks its class and closes
    // it (notification 0x29b, FUN_0060d020 -> FUN_0060c970).
    #[test]
    fn the_class_list_shows_two_rows_and_a_click_picks_one() {
        let (world, system) = galaxy(true);
        let mut state = BuildSelectionState::default();
        let regiments = kinds();
        assert!(state.open(
            system,
            ProductionArea::TrainingFacility,
            true,
            regiments.clone()
        ));
        let at = egui::Pos2::ZERO;
        drive(
            &world,
            &mut state,
            (111.0, 99.0),
            vec![vec![press(at, true)], vec![press(at, false)]],
        );
        assert!(state.dialog().unwrap().list_open());
        let (_, texts) = drive_at(&world, &mut state, 1.0, (111.0, 99.0), vec![]);
        let origin = window_rect(layout()).min;
        for (row, kind) in regiments.iter().enumerate() {
            let item = egui::Rect::from_min_size(
                origin + egui::vec2(8.0, 87.0 + 70.0 * row as f32),
                egui::vec2(191.0, 63.0),
            );
            let name = product(&world, *kind).0;
            // No portrait is painted here, so the name hangs from the item's
            // middle.
            let top = texts
                .iter()
                .filter(|(text, ..)| *text == name)
                .map(|(_, top, ..)| *top)
                .max_by(|a, b| a.y.total_cmp(&b.y))
                .unwrap();
            assert!((top - item.center()).length() < 0.5, "{top:?} vs {item:?}");
        }
        // The popup holds the capture (FUN_0060d020): a click beside it (x
        // 6..201) or on Cancel closes it, picks nothing and cancels nothing.
        let click_at = |state: &mut BuildSelectionState, point| {
            drive(
                &world,
                state,
                point,
                vec![vec![press(at, true)], vec![press(at, false)]],
            )
        };
        for outside in [(203.0, 150.0), (170.0, 240.0)] {
            if !state.dialog().unwrap().list_open() {
                click_at(&mut state, (111.0, 99.0));
            }
            assert_eq!(click_at(&mut state, outside), None);
            assert!(state.is_open());
            assert!(!state.dialog().unwrap().list_open(), "{outside:?}");
            assert_eq!(state.dialog().unwrap().kind(), regiments[0]);
        }
        click_at(&mut state, (111.0, 99.0));
        click_at(&mut state, (100.0, 190.0));
        assert_eq!(state.dialog().unwrap().kind(), regiments[1]);
        assert!(!state.dialog().unwrap().list_open());
    }

    // FUN_006083c0 (WM_PAINT): a row's name takes item +0x5c when picked
    // (+0x3c bit 0), else +0x64 when under the pointer (bit 1), else +0x60;
    // FUN_00437880 sets +0x60 to 0x808080 and leaves the others white. The
    // list shares the box's items (FUN_00608280), so the bits outlive it;
    // only WM_MOUSEMOVE clears bit 1, on the row the open list marked last.
    // The popup covers the drop-down button, so the pointer that opened it
    // rests on the first row.
    #[test]
    fn the_class_list_greys_every_row_but_the_picked_and_hovered_ones() {
        let (world, system) = galaxy(true);
        let regiments = kinds();
        let names: Vec<String> = regiments.iter().map(|&k| product(&world, k).0).collect();
        let mut state = BuildSelectionState::default();
        assert!(state.open(
            system,
            ProductionArea::TrainingFacility,
            true,
            regiments.clone()
        ));
        let at = egui::Pos2::ZERO;
        let toggle = |state: &mut BuildSelectionState| {
            drive(
                &world,
                state,
                (111.0, 99.0),
                vec![vec![press(at, true)], vec![press(at, false)]],
            );
        };
        let rows = |state: &mut BuildSelectionState, point| {
            list_rows(&drive_at(&world, state, 1.0, point, vec![]).1, &names)
        };
        let white = egui::Color32::WHITE;
        toggle(&mut state);
        // Nothing picked yet: only the row under the pointer is white.
        assert_eq!(
            rows(&mut state, (111.0, 99.0)),
            vec![(names[0].clone(), white), (names[1].clone(), GREY)]
        );
        assert_eq!(
            rows(&mut state, (100.0, 190.0)),
            vec![(names[0].clone(), GREY), (names[1].clone(), white)]
        );
        // On the popup's frame, off its list (y 87..227), or off the popup,
        // the list sees no move and nothing changes.
        for off in [(100.0, 228.0), (203.0, 150.0)] {
            assert_eq!(
                rows(&mut state, off),
                vec![(names[0].clone(), GREY), (names[1].clone(), white)]
            );
        }
        assert_eq!(
            rows(&mut state, (100.0, 120.0)),
            vec![(names[0].clone(), white), (names[1].clone(), GREY)]
        );
        drive(
            &world,
            &mut state,
            (100.0, 190.0),
            vec![vec![press(at, true)], vec![press(at, false)]],
        );
        assert_eq!(state.dialog().unwrap().kind(), regiments[1]);
        // Reopened over the first row, the picked second row stays white.
        toggle(&mut state);
        assert_eq!(
            rows(&mut state, (111.0, 99.0)),
            vec![(names[0].clone(), white), (names[1].clone(), white)]
        );
        // A move to the picked row leaves the first grey.
        assert_eq!(
            rows(&mut state, (100.0, 190.0)),
            vec![(names[0].clone(), GREY), (names[1].clone(), white)]
        );
    }

    // FUN_00609ed0 scrolls the new list to the box's class; the wheel
    // scrolls the rows, not the class. port: the wheel stands in for the
    // list's scroll bar.
    #[test]
    fn the_class_list_opens_on_the_class_and_scrolls_with_the_wheel() {
        let (world, system) = galaxy(true);
        let mut regiments = kinds();
        regiments.push(BuildableKind::Troop(DatId::new(0x1000_0003)));
        let names: Vec<String> = regiments.iter().map(|&k| product(&world, k).0).collect();
        let mut state = BuildSelectionState::default();
        assert!(state.open(
            system,
            ProductionArea::TrainingFacility,
            true,
            regiments.clone()
        ));
        state.dialog_mut().unwrap().select(2);
        let at = egui::Pos2::ZERO;
        drive(
            &world,
            &mut state,
            (111.0, 99.0),
            vec![vec![press(at, true)], vec![press(at, false)]],
        );
        let shown = |state: &mut BuildSelectionState, events| {
            list_rows(&drive_at(&world, state, 1.0, (100.0, 150.0), events).1, &names)
                .into_iter()
                .map(|(name, _)| name)
                .collect::<Vec<_>>()
        };
        assert_eq!(shown(&mut state, vec![]), names[1..].to_vec());
        let wheel = |delta| {
            vec![vec![egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Line,
                delta: egui::vec2(0.0, delta),
                modifiers: egui::Modifiers::default(),
            }]]
        };
        assert_eq!(shown(&mut state, wheel(1.0)), names[..2].to_vec());
        assert_eq!(shown(&mut state, wheel(1.0)), names[..2].to_vec());
        assert_eq!(shown(&mut state, wheel(-1.0)), names[1..].to_vec());
        assert_eq!(shown(&mut state, wheel(-1.0)), names[1..].to_vec());
        assert_eq!(state.dialog().unwrap().kind(), regiments[2]);
    }

    // FUN_0060d020 forwards a press on the popup only to the list under it
    // (2 in, 191 by 140); FUN_006083c0 picks only on a row, and below the
    // last row it clears the pick and leaves the list open.
    #[test]
    fn the_class_list_picks_only_on_its_rows() {
        let (world, system) = galaxy(true);
        let at = egui::Pos2::ZERO;
        let click_at = |state: &mut BuildSelectionState, point| {
            drive(
                &world,
                state,
                point,
                vec![vec![press(at, true)], vec![press(at, false)]],
            );
        };
        let mut state = open();
        click_at(&mut state, (111.0, 99.0));
        // The popup's frame: left of, above and below the list.
        for frame in [(7.0, 190.0), (100.0, 86.0), (100.0, 228.0)] {
            click_at(&mut state, frame);
            assert!(state.dialog().unwrap().list_open(), "{frame:?}");
            assert_eq!(state.dialog().unwrap().kind(), kinds()[0], "{frame:?}");
        }
        // The list's left edge, x 8.
        click_at(&mut state, (9.0, 190.0));
        assert_eq!(state.dialog().unwrap().kind(), kinds()[1]);
        assert!(!state.dialog().unwrap().list_open());

        // One class: the second row is empty.
        let mut state = opened(system, true);
        let name = product(&world, state.dialog().unwrap().kind()).0;
        click_at(&mut state, (111.0, 99.0));
        let (_, texts) = drive_at(&world, &mut state, 1.0, (100.0, 190.0), vec![]);
        assert_eq!(list_rows(&texts, &[name.clone()]), vec![(name, GREY)]);
        click_at(&mut state, (100.0, 190.0));
        assert!(state.dialog().unwrap().list_open());
    }

    // The rows and the pointer's row scale with the window.
    #[test]
    fn the_class_list_rows_scale_with_the_window() {
        let (world, system) = galaxy(true);
        let regiments = kinds();
        let names: Vec<String> = regiments.iter().map(|&k| product(&world, k).0).collect();
        let mut state = BuildSelectionState::default();
        assert!(state.open(system, ProductionArea::TrainingFacility, true, regiments));
        let at = egui::Pos2::ZERO;
        drive_at(
            &world,
            &mut state,
            2.0,
            (111.0, 99.0),
            vec![vec![press(at, true)], vec![press(at, false)]],
        );
        assert!(state.dialog().unwrap().list_open());
        let (_, texts) = drive_at(&world, &mut state, 2.0, (100.0, 190.0), vec![]);
        let mut named: Vec<_> = texts
            .iter()
            .filter(|(text, ..)| names.contains(text))
            .map(|(text, top, _, color)| (top.y, text.clone(), *color))
            .collect();
        named.sort_by(|a, b| a.0.total_cmp(&b.0));
        // The class box's name comes first, then the two rows.
        let rows: Vec<_> = named[1..].iter().map(|(_, text, color)| (text.clone(), *color)).collect();
        assert_eq!(
            rows,
            vec![(names[0].clone(), GREY), (names[1].clone(), egui::Color32::WHITE)]
        );
    }

    #[test]
    fn the_window_blocks_the_map_only_while_open() {
        let (_, system) = galaxy(true);
        let mut state = BuildSelectionState::default();
        let inside = window_rect(layout()).center();
        let point = (inside.x, inside.y);
        assert!(!state.contains_screen_point(layout(), point));
        state = opened(system, true);
        assert!(state.contains_screen_point(layout(), point));
        assert!(!state.contains_screen_point(layout(), (0.0, 0.0)));
    }

    // FUN_00438500 fonts 5, 4 and 10 and FUN_00437f80's value fonts, at
    // twice the size on a doubled layout; the class's name under its
    // portrait.
    #[test]
    fn texts_scale_with_the_window_and_name_the_class() {
        let (world, system) = galaxy(true);
        let mut state = opened(system, true);
        let (_, texts) = drive_at(&world, &mut state, 2.0, (0.0, 0.0), vec![]);
        let size = |wanted: &str| {
            texts
                .iter()
                .find(|(text, ..)| text == wanted)
                .map(|(_, _, size, _)| *size)
        };
        assert_eq!(size(TITLE), Some(22.0));
        assert_eq!(size(NUMBER_TO_BUILD), Some(20.0));
        assert_eq!(size(BEST_COMPLETION), Some(18.0));
        assert_eq!(size("32 Days"), Some(18.0));
        assert_eq!(size("8"), Some(20.0));
        assert_eq!(size("6"), Some(20.0));
        assert_eq!(size("Alliance Fleet Regiment"), Some(18.0));
    }

    // FUN_00437f80's fields: each value stays inside its own rectangle, as
    // the original's aliased text does (the original ends "40 Days" inside
    // its 60-pixel field), so no edge pixel lands on the background.
    #[test]
    fn each_value_is_clipped_to_its_field() {
        let (world, system) = galaxy(true);
        let mut state = opened(system, true);
        let layout = layout();
        let window = window_rect(layout);
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let manufacturing = ManufacturingState::new();
        // egui paints an Area from its second pass; read the third.
        let mut output = egui::FullOutput::default();
        for _ in 0..3 {
            output = ctx.run(egui::RawInput::default(), |ctx| {
                let _ = draw_build_selection(
                    ctx,
                    &world,
                    &manufacturing,
                    &TroopTransportState::default(),
                    &DeliveryState::default(),
                    &MovementState::new(),
                    &mut state,
                    layout,
                    &mut cache,
                );
            });
        }
        let clip = |wanted: &str| {
            output.shapes.iter().find_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) if text.galley.text() == wanted => Some(clipped.clip_rect),
                _ => None,
            })
        };
        let field = |x: f32, y: f32, w: f32, h: f32| {
            egui::Rect::from_min_size(window.min + egui::vec2(x, y), egui::vec2(w, h))
        };
        assert_eq!(clip("32 Days"), Some(field(140.0, 145.0, 60.0, 15.0)));
        assert_eq!(clip("0 Days"), Some(field(140.0, 165.0, 60.0, 15.0)));
        assert_eq!(clip("8"), Some(field(36.0, 110.0, 64.0, 23.0)));
        assert_eq!(clip("6"), Some(field(138.0, 110.0, 64.0, 23.0)));
        assert_eq!(clip("1"), Some(field(141.0, 196.0, 45.0, 17.0)));
    }

    // FUN_00438dd0 / FUN_00438f30: 9999 at most, " Days" (14357), and "n/a"
    // (14358) for an order that cannot be built.
    #[test]
    fn values_cap_at_9999_and_show_na_when_the_order_cannot_be_built() {
        assert_eq!(cost_text(24, true), "24");
        assert_eq!(cost_text(12_000, true), "9999");
        assert_eq!(cost_text(24, false), "n/a");
        assert_eq!(days_text(32, true), "32 Days");
        assert_eq!(days_text(10_000, true), "9999 Days");
        assert_eq!(days_text(32, false), "n/a");
    }
}
