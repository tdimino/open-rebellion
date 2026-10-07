//! The original Build Selection window (`FUN_00437df0` builds it,
//! `FUN_00437f80` and `FUN_00438620` its fields and controls, `FUN_00438800`
//! handles them; `ghidra/notes/manufacturing-build-selection.md`).
//!
//! A 210 by 261 window over background 10800 that a Manufacturing window
//! band's Build opens. It lists the classes the band's yards build, shows
//! the order's two costs and its best completion and deployment times, and
//! takes the number to build. Confirm replaces what the band was building.

use egui_macroquad::egui;
use rebellion_core::build_selection::{estimate, BuildEstimate};
use rebellion_core::ids::SystemKey;
use rebellion_core::manufacturing::{BuildableKind, ManufacturingState, ProductionArea};
use rebellion_core::world::GameWorld;

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::CockpitLayout;
use crate::manufacturing_window::product;
use crate::mission_dialog::{button, galaxy_centered_rect, paint, paint_centered, scrolled};
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

/// The labels' color, `0x2f0fbff` (`FUN_00438500`).
const LABEL: egui::Color32 = egui::Color32::from_rgb(0xff, 0xfb, 0xf0);

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
        self.dialog = Some(BuildSelection {
            system,
            area,
            is_alliance,
            kinds,
            selected,
            quantity: 1,
            list_open: false,
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
pub fn draw_build_selection(
    ctx: &egui::Context,
    world: &GameWorld,
    manufacturing: &ManufacturingState,
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
    let valid = estimate.completion.is_some();
    let mut action = None;
    let mut close = false;
    let mut confirm = false;

    // FUN_00438b60: Enter confirms, Escape closes, keypad + and - step the
    // number; the number edit has the focus for digits.
    ctx.input(|input| {
        for event in &input.events {
            if let egui::Event::Key {
                key, pressed: true, ..
            } = event
            {
                match key {
                    egui::Key::Enter if valid => confirm = true,
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

            // The selected class: its mini centered in the box, its name
            // under it (FUN_00437880's 195 by 63 item).
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
            );

            // FUN_00438dd0 / FUN_00438f30: the values, white.
            for ((x, y), text) in [
                (REFINED_FIELD, cost_text(estimate.refined_material, valid)),
                (MAINTENANCE_FIELD, cost_text(estimate.maintenance, valid)),
            ] {
                painter.text(
                    at(x, y, 64.0, 23.0).center(),
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
                painter.text(
                    at(x, y, 60.0, 15.0).right_center(),
                    egui::Align2::RIGHT_CENTER,
                    text,
                    small.clone(),
                    egui::Color32::WHITE,
                );
            }
            let (x, y, w, h) = QUANTITY_FIELD;
            painter.text(
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
                dialog.list_open = !dialog.list_open;
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
                paint(&painter, ctx, cache, CONFIRM.0, confirm_rect, scale);
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

            // The drop-down: a window under the box holding one item at a
            // time. port: its scroll bar (base 0x299a) is not drawn; the
            // wheel scrolls it, as the mission dialog's.
            if dialog.list_open {
                let (x, y, w, h) = ITEM_BOX;
                let popup = at(x, y + h, w, h + 4.0);
                let item = at(x + 2.0, y + h + 2.0, w - 4.0, h);
                painter.rect_filled(popup, 0.0, egui::Color32::BLACK);
                let response = ui.interact(popup, ui.id().with("class-list"), egui::Sense::click());
                if response.hovered() {
                    let wheel = ctx.input(|input| input.raw_scroll_delta.y);
                    let row = scrolled(dialog.selected, dialog.kinds.len(), wheel);
                    dialog.select(row);
                }
                paint_item(
                    &painter,
                    ctx,
                    cache,
                    world,
                    dialog.kind(),
                    item,
                    scale,
                    &small,
                );
                if exact_clicked(&response, popup) {
                    dialog.list_open = false;
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

/// One class: its GOKRES mini centered in the box and its name under it.
/// hyp: the name sits under the mini, centered, as the mission kinds'.
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
) {
    let (name, mini) = product(world, kind);
    let image = mini
        .and_then(|id| paint_centered(painter, ctx, cache, DllSource::Gokres, id, bounds, scale));
    painter.text(
        egui::pos2(
            bounds.center().x,
            image.map_or(bounds.center().y, |rect| rect.max.y),
        ),
        egui::Align2::CENTER_TOP,
        name,
        font.clone(),
        egui::Color32::WHITE,
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
    /// A painted text, the top centre of its galley, and its size.
    type Text = (String, egui::Pos2, f32);

    /// A system of the Alliance's with a training facility (period 4) when
    /// `yard`, and the regiment class TROOPSD 1 (cost 8, maintenance 6).
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
        if yard {
            let key = world
                .manufacturing_facilities
                .insert(ManufacturingFacilityInstance {
                    class_dat_id: DatId::new(0x2900_0002),
                    is_alliance: true,
                    is_shipyard: false,
                });
            world.systems[system].manufacturing_facilities.push(key);
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
                if let Some(action) =
                    draw_build_selection(ctx, world, &manufacturing, state, layout, &mut cache)
                {
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
        assert_eq!(
            drive(
                &world,
                &mut state,
                (0.0, 0.0),
                vec![vec![key(egui::Key::Enter)]]
            ),
            None
        );
        assert!(state.is_open());
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

    // The drop-down (command 0x6c) opens under the class box; the wheel
    // walks the list and a click closes it.
    #[test]
    fn the_class_list_opens_scrolls_and_closes() {
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
        // Down one notch over the list's last rows, below the box it hangs
        // from (y 85..152).
        let wheel = egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Line,
            delta: egui::vec2(0.0, -1.0),
            modifiers: egui::Modifiers::default(),
        };
        drive(&world, &mut state, (100.0, 147.0), vec![vec![wheel]]);
        assert_eq!(state.dialog().unwrap().kind(), regiments[1]);
        // The list's item names the class in its own box, under the main one.
        let (_, texts) = drive_at(&world, &mut state, 1.0, (100.0, 147.0), vec![]);
        let origin = window_rect(layout()).min;
        let item =
            egui::Rect::from_min_size(origin + egui::vec2(8.0, 87.0), egui::vec2(191.0, 63.0));
        // The class box above names it too; the list's copy is the lower.
        let named = texts
            .iter()
            .filter(|(text, _, _)| text == "Alliance Army Regiment")
            .map(|(_, top, _)| *top)
            .max_by(|a, b| a.y.total_cmp(&b.y))
            .unwrap();
        assert!(
            (named.x - item.center().x).abs() < 0.5,
            "{named:?} vs {item:?}"
        );
        // No mini is painted here, so the name hangs from the item's middle.
        assert!(
            (named.y - item.center().y).abs() < 0.5,
            "{named:?} vs {item:?}"
        );
        // The list is the box's height and four more (y 85..152): the wheel
        // below it, up a notch, leaves the class alone.
        let up = egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Line,
            delta: egui::vec2(0.0, 1.0),
            modifiers: egui::Modifiers::default(),
        };
        drive(&world, &mut state, (100.0, 155.0), vec![vec![up]]);
        assert_eq!(state.dialog().unwrap().kind(), regiments[1]);
        drive(
            &world,
            &mut state,
            (100.0, 147.0),
            vec![vec![press(at, true)], vec![press(at, false)]],
        );
        assert!(!state.dialog().unwrap().list_open());
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
    // twice the size on a doubled layout; the class's name under its mini.
    #[test]
    fn texts_scale_with_the_window_and_name_the_class() {
        let (world, system) = galaxy(true);
        let mut state = opened(system, true);
        let (_, texts) = drive_at(&world, &mut state, 2.0, (0.0, 0.0), vec![]);
        let size = |wanted: &str| {
            texts
                .iter()
                .find(|(text, _, _)| text == wanted)
                .map(|(_, _, size)| *size)
        };
        assert_eq!(size(TITLE), Some(22.0));
        assert_eq!(size(NUMBER_TO_BUILD), Some(20.0));
        assert_eq!(size(BEST_COMPLETION), Some(18.0));
        assert_eq!(size("32 Days"), Some(18.0));
        assert_eq!(size("8"), Some(20.0));
        assert_eq!(size("6"), Some(20.0));
        assert_eq!(size("Alliance Fleet Regiment"), Some(18.0));
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
