//! The original mission-creation dialog (`FUN_0046a750` builds it,
//! `FUN_0046a9c0` its controls, `FUN_0046c3c0` handles them;
//! `ghidra/notes/mission-dialog.md`).
//!
//! A 259 by 355 window with two tabbed pages. The first chooses the mission
//! kind and shows the target; the second splits the chosen characters and
//! special forces between agents and decoys. "Begin Mission" submits the
//! order, "Cancel" and the close box discard it, and "Encylopedia" opens the
//! Encyclopedia.

use egui_macroquad::egui;
use rebellion_core::ids::SystemKey;
use rebellion_core::missions::{MissionFaction, MissionKind, MissionMember};
use rebellion_core::world::GameWorld;

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::CockpitLayout;
use crate::sector_window::planet_resource_id;
use crate::system_window::{
    character_mini_resource_id, exact_clicked, logical_rect, rect_contains, special_force_mini,
};

pub const MISSION_DIALOG_WIDTH: f32 = 259.0;
pub const MISSION_DIALOG_HEIGHT: f32 = 355.0;

// STRATEGY bitmaps (FUN_0046a9c0).
const TITLE_ALLIANCE: u32 = 10801;
const TITLE_EMPIRE: u32 = 10802;
const PANEL_MISSION: u32 = 11100;
const PANEL_AGENTS: u32 = 11101;
const AGENTS_HEADER: [u32; 2] = [11121, 11123];
const DECOYS_HEADER: [u32; 2] = [11122, 11124];
const CLOSE: (u32, u32) = (10108, 10109);
const MISSIONS: (u32, u32) = (10606, 10607);
const CANCEL: (u32, u32) = (10596, 10597);
const BEGIN: (u32, u32) = (10594, 10595);
const ENCYCLOPEDIA: (u32, u32) = (10592, 10593);
const TO_DECOYS: (u32, u32) = (11117, 11118);
const TO_AGENTS: (u32, u32) = (11119, 11120);
const MISSION_TAB: [(u32, u32); 2] = [(11103, 11104), (11105, 11106)];
const AGENTS_TAB: [(u32, u32); 2] = [(11107, 11108), (11109, 11110)];

// TEXTSTRA strings.
/// TEXTSTRA 34055, the title of both pages.
const CREATE_MISSION: &str = "Create Mission";
/// TEXTSTRA 34052, the target label on the first page.
const TARGET: &str = "Target";

// Geometry (FUN_0046a9c0), in dialog pixels.
const KIND_BOX: (f32, f32, f32, f32) = (35.0, 62.0, 200.0, 113.0);
const AGENTS_LIST: (f32, f32) = (8.0, 93.0);
const DECOYS_LIST: (f32, f32) = (136.0, 93.0);
const LIST_SIZE: (f32, f32) = (108.0, 213.0);
const ITEM_SIZE: (f32, f32) = (107.0, 59.0);
/// The target art is centered in a 165 by 79 box at (51, 211).
const TARGET_BOX: (f32, f32, f32, f32) = (51.0, 211.0, 165.0, 79.0);
/// The target's name, centered in a 185-wide box at (37, 294).
const TARGET_NAME: (f32, f32, f32) = (37.0, 294.0, 185.0);

/// A list item's name color, unselected and selected (list `+0xd8`/`+0xdc`,
/// `FUN_00607ea0`).
const ITEM_TEXT: egui::Color32 = egui::Color32::from_rgb(0x78, 0x78, 0x78);
const ITEM_TEXT_SELECTED: egui::Color32 = egui::Color32::WHITE;
/// A mission kind's name color (item `+0x60`, `FUN_0046a9c0`).
const KIND_TEXT: egui::Color32 = egui::Color32::from_rgb(0x80, 0x80, 0x80);

/// The two tabs of control `0x96` (`FUN_0046c8a0`, page at `+0x118`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MissionDialogPage {
    /// Tab `0x97`: the mission kind and the target.
    #[default]
    Mission,
    /// Tab `0x98`: agents and decoys.
    Agents,
}

/// One open dialog: the order it edits and its controls' state.
#[derive(Debug, Clone, PartialEq)]
pub struct MissionDialog {
    faction: MissionFaction,
    target: SystemKey,
    /// The kinds `FUN_004f5380` listed when the dialog was built.
    kinds: Vec<MissionKind>,
    kind: usize,
    page: MissionDialogPage,
    /// The mission-kind drop-down is open (`CoolSelectionBoxClass +0xe8`).
    list_open: bool,
    agents: Vec<MissionMember>,
    decoys: Vec<MissionMember>,
    selected: Vec<MissionMember>,
    agents_row: usize,
    decoys_row: usize,
}

impl MissionDialog {
    #[must_use]
    pub fn kind(&self) -> MissionKind {
        self.kinds[self.kind]
    }

    #[must_use]
    pub fn agents(&self) -> &[MissionMember] {
        &self.agents
    }

    #[must_use]
    pub fn decoys(&self) -> &[MissionMember] {
        &self.decoys
    }

    #[must_use]
    pub fn page(&self) -> MissionDialogPage {
        self.page
    }

    /// Whether the mission-kind drop-down is open.
    #[must_use]
    pub fn list_open(&self) -> bool {
        self.list_open
    }

    /// Show a page (`FUN_0046c8a0`); the drop-down belongs to the first page
    /// and closes.
    pub fn show_page(&mut self, page: MissionDialogPage) {
        self.page = page;
        self.list_open = false;
    }

    /// Choose a listed kind (`FUN_0060c970`); an index past the list is
    /// ignored.
    pub fn select_kind(&mut self, index: usize) {
        if index < self.kinds.len() {
            self.kind = index;
        }
    }

    /// Flip a member's selection in whichever list holds it.
    pub fn toggle(&mut self, member: MissionMember) {
        if let Some(index) = self.selected.iter().position(|&m| m == member) {
            self.selected.remove(index);
        } else if self.agents.contains(&member) || self.decoys.contains(&member) {
            self.selected.push(member);
        }
    }

    /// Cases `0xca` (to the decoys) and `0xcb` (to the agents) of
    /// `FUN_0046c3c0`: the selected members of the source list join the end
    /// of the other list in their order, and every selection clears.
    pub fn move_selected(&mut self, to_decoys: bool) {
        let (from, to) = if to_decoys {
            (&mut self.agents, &mut self.decoys)
        } else {
            (&mut self.decoys, &mut self.agents)
        };
        let moved: Vec<MissionMember> = from
            .iter()
            .copied()
            .filter(|member| self.selected.contains(member))
            .collect();
        from.retain(|member| !moved.contains(member));
        to.extend(moved);
        self.selected.clear();
        self.agents_row = self.agents_row.min(self.agents.len().saturating_sub(1));
        self.decoys_row = self.decoys_row.min(self.decoys.len().saturating_sub(1));
    }
}

/// Whether a mission dialog is open.
#[derive(Debug, Clone, Default)]
pub struct MissionDialogState {
    dialog: Option<MissionDialog>,
}

impl MissionDialogState {
    /// Open the dialog for an order's team and target with the kinds the
    /// team may undertake. With no kind the order is dropped and nothing
    /// opens (`FUN_0042a320` -> `FUN_0041ce20(order, 0)`). The team starts as
    /// the agents and the decoys empty, as a fresh order `0x240` leaves them.
    pub fn open(
        &mut self,
        faction: MissionFaction,
        target: SystemKey,
        team: Vec<MissionMember>,
        kinds: Vec<MissionKind>,
    ) -> bool {
        if kinds.is_empty() || team.is_empty() {
            return false;
        }
        self.dialog = Some(MissionDialog {
            faction,
            target,
            kinds,
            kind: 0,
            page: MissionDialogPage::Mission,
            list_open: false,
            agents: team,
            decoys: Vec::new(),
            selected: Vec::new(),
            agents_row: 0,
            decoys_row: 0,
        });
        true
    }

    #[must_use]
    pub fn is_open(&self) -> bool {
        self.dialog.is_some()
    }

    #[must_use]
    pub fn dialog(&self) -> Option<&MissionDialog> {
        self.dialog.as_ref()
    }

    pub fn dialog_mut(&mut self) -> Option<&mut MissionDialog> {
        self.dialog.as_mut()
    }

    pub fn close(&mut self) {
        self.dialog = None;
    }

    /// Whether `point` falls on the open dialog, so the galaxy map under it
    /// takes no input. The dialog is a child window (style `0x46000000`),
    /// not modal.
    #[must_use]
    pub fn contains_screen_point(&self, layout: CockpitLayout, point: (f32, f32)) -> bool {
        self.dialog.is_some() && rect_contains(dialog_rect(layout), egui::pos2(point.0, point.1))
    }

    /// "Begin Mission" (`0x66`): the order's mission becomes the selected
    /// kind and the order is submitted; the dialog closes whether or not the
    /// validator accepts it.
    pub fn begin(&mut self) -> Option<MissionDialogAction> {
        let dialog = self.dialog.take()?;
        Some(MissionDialogAction::Begin {
            kind: dialog.kind(),
            faction: dialog.faction,
            team: dialog.agents,
            decoys: dialog.decoys,
            target: dialog.target,
        })
    }
}

/// What the dialog asks of the game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MissionDialogAction {
    Begin {
        kind: MissionKind,
        faction: MissionFaction,
        team: Vec<MissionMember>,
        decoys: Vec<MissionMember>,
        target: SystemKey,
    },
    /// "Encylopedia" (`0x67` -> `FUN_00429f30`, window `0x19`). port: it opens
    /// the Encyclopedia; the entry for the selected kind is not bound yet
    /// (`encyclopedia-window.md`).
    Encyclopedia,
}

/// A kind's MISSNSD TEXTSTRA id: its name, and the source of its GOKRES icon.
const fn kind_text_id(kind: MissionKind) -> u16 {
    match kind {
        MissionKind::Diplomacy => 11280,
        MissionKind::Rescue => 11281,
        MissionKind::Sabotage => 11282,
        MissionKind::Espionage => 11283,
        MissionKind::Recruitment => 11286,
        MissionKind::Abduction => 11287,
        MissionKind::InciteUprising => 11328,
        MissionKind::DeathStarSabotage => 11329,
        MissionKind::SubdueUprising => 11392,
        MissionKind::Assassination => 11393,
        MissionKind::Autoscrap => 0,
    }
}

/// A kind's TEXTSTRA name.
#[must_use]
pub const fn kind_name(kind: MissionKind) -> &'static str {
    match kind {
        MissionKind::Diplomacy => "Diplomacy",
        MissionKind::Rescue => "Rescue",
        MissionKind::Sabotage => "Sabotage",
        MissionKind::Espionage => "Espionage",
        MissionKind::Recruitment => "Recruitment",
        MissionKind::Abduction => "Abduction",
        MissionKind::InciteUprising => "Incite Uprising",
        MissionKind::DeathStarSabotage => "Death Star Sabotage",
        MissionKind::SubdueUprising => "Subdue Uprising",
        MissionKind::Assassination => "Assassination",
        MissionKind::Autoscrap => "",
    }
}

/// A kind's GOKRES icon: its TEXTSTRA id's low 12 bits, plus `0x1000` for the
/// Empire (`FUN_0046a9c0`, record `+0x30`).
#[must_use]
pub fn kind_icon(kind: MissionKind, faction: MissionFaction) -> u32 {
    let base = u32::from(kind_text_id(kind) & 0xfff);
    match faction {
        MissionFaction::Alliance => base,
        MissionFaction::Empire => base + 0x1000,
    }
}

/// A member's list image and name: the GOKRES mini `FUN_0042c3b0(.., 0, 1)`
/// draws first. port: its status overlays (STRATEGY 11500..11502, 11570,
/// 11572, GOKRES `+0x7000`) are not drawn.
fn member_item(world: &GameWorld, member: MissionMember) -> Option<(Option<u32>, String)> {
    match member {
        MissionMember::Character(key) => {
            let character = world.characters.get(key)?;
            Some((
                character_mini_resource_id(character.dat_id, character.is_major),
                character.name.clone(),
            ))
        }
        MissionMember::SpecialForce(key) => {
            let unit = world.special_forces.get(key)?;
            let (resource, name) = special_force_mini(unit.class_dat_id)?;
            Some((Some(resource), name.to_string()))
        }
    }
}

/// Where a bitmap of `size` pixels lands when centered in `bounds` at
/// `scale`: the original's `(box - size) / 2` offsets (`FUN_0046a9c0`).
fn centered(bounds: egui::Rect, size: egui::Vec2, scale: f32) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(
            bounds.min.x + ((bounds.width() - size.x * scale) / 2.0).floor(),
            bounds.min.y + ((bounds.height() - size.y * scale) / 2.0).floor(),
        ),
        size * scale,
    )
}

/// One wheel step through `len` items from `row`: down (a negative delta)
/// goes one on, up goes one back, and neither leaves the list.
pub(crate) fn scrolled(row: usize, len: usize, wheel: f32) -> usize {
    if wheel < 0.0 && row + 1 < len {
        row + 1
    } else if wheel > 0.0 {
        row.saturating_sub(1)
    } else {
        row
    }
}

/// Paint a bitmap at its own size scaled with the dialog, centered in `bounds`.
pub(crate) fn paint_centered(
    painter: &egui::Painter,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    source: DllSource,
    resource_id: u32,
    bounds: egui::Rect,
    scale: f32,
) -> Option<egui::Rect> {
    let (texture_id, size) = cache
        .get(ctx, source, resource_id)
        .map(|texture| (texture.id(), texture.size_vec2()))?;
    let rect = centered(bounds, size, scale);
    painter.image(
        texture_id,
        rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
        egui::Color32::WHITE,
    );
    Some(rect)
}

/// `FUN_005fc140` blits a bitmap at its own size (a zero width or height
/// means the bitmap's) and the control's window clips the rest, so a bitmap
/// wider than its control shows its left part.
fn native(rect: egui::Rect, size: egui::Vec2, scale: f32) -> egui::Rect {
    egui::Rect::from_min_size(rect.min, size * scale)
}

pub(crate) fn paint(
    painter: &egui::Painter,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    resource_id: u32,
    rect: egui::Rect,
    scale: f32,
) {
    #[cfg(test)]
    crate::fleet_window::tests::PAINTED.with(|painted| {
        painted.borrow_mut().push((resource_id, rect.min));
    });
    if let Some(texture) = cache.get(ctx, DllSource::Strategy, resource_id) {
        painter
            .with_clip_rect(painter.clip_rect().intersect(rect))
            .image(
                texture.id(),
                native(rect, texture.size_vec2(), scale),
                egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
    }
}

/// A two-state bitmap button (`FUN_00602150`): the second bitmap shows while
/// it is held or, for a tab, while it is the current page.
pub(crate) fn button(
    ui: &egui::Ui,
    cache: &mut BmpCache,
    rect: egui::Rect,
    id: impl std::hash::Hash,
    (normal, pressed): (u32, u32),
    down: bool,
    scale: f32,
) -> bool {
    let response = ui.interact(rect, ui.id().with(id), egui::Sense::click());
    let (pointer, primary_down) = ui.ctx().input(|input| {
        (
            input.pointer.interact_pos(),
            input.pointer.button_down(egui::PointerButton::Primary),
        )
    });
    let held = primary_down && pointer.is_some_and(|point| rect_contains(rect, point));
    paint(
        ui.painter(),
        ui.ctx(),
        cache,
        if down || held { pressed } else { normal },
        rect,
        scale,
    );
    exact_clicked(&response, rect)
}

/// The dialog's screen rectangle.
fn dialog_rect(layout: CockpitLayout) -> egui::Rect {
    galaxy_centered_rect(layout, MISSION_DIALOG_WIDTH, MISSION_DIALOG_HEIGHT)
}

/// A `width` by `height` window of the galaxy view. hyp: `FUN_00606980`
/// places it in the galaxy view's rectangle (parent `+0xcc..+0xd8`); the port
/// centers it, on whole pixels so its bitmaps are not resampled.
pub(crate) fn galaxy_centered_rect(layout: CockpitLayout, width: f32, height: f32) -> egui::Rect {
    let size = egui::vec2(width * layout.scale, height * layout.scale);
    let center = egui::pos2(
        layout.galaxy.x + layout.galaxy.width / 2.0,
        layout.galaxy.y + layout.galaxy.height / 2.0,
    );
    egui::Rect::from_min_size((center - size / 2.0).round(), size)
}

/// Draw the open dialog, if any, and report what the player asked for.
#[expect(
    clippy::too_many_lines,
    reason = "The dialog paints in the original's control order; splitting it hides that order."
)]
pub fn draw_mission_dialog(
    ctx: &egui::Context,
    world: &GameWorld,
    state: &mut MissionDialogState,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Option<MissionDialogAction> {
    let dialog = state.dialog.as_mut()?;
    let scale = layout.scale;
    let side = usize::from(dialog.faction == MissionFaction::Empire);
    let window = dialog_rect(layout);
    let mut action = None;
    let mut close = false;
    let mut begin = false;

    // Above the modeless windows, which share Foreground and raise the
    // focused one each frame (`system_window.rs`).
    egui::Area::new(egui::Id::new("original-mission-dialog"))
        .fixed_pos(window.min)
        .order(egui::Order::Tooltip)
        .show(ctx, |ui| {
            let (frame, _) = ui.allocate_exact_size(window.size(), egui::Sense::hover());
            let painter = ui.painter().with_clip_rect(frame);
            let at = |x: f32, y: f32, w: f32, h: f32| logical_rect(frame, scale, x, y, w, h);

            // The page panel, then the title strip: the 240-pixel bar laid
            // twice, 30 pixels apart, at (2, 2), and the title over it.
            let panel = match dialog.page {
                MissionDialogPage::Mission => PANEL_MISSION,
                MissionDialogPage::Agents => PANEL_AGENTS,
            };
            paint(&painter, ctx, cache, panel, frame, scale);
            let title = [TITLE_ALLIANCE, TITLE_EMPIRE][side];
            paint(
                &painter,
                ctx,
                cache,
                title,
                at(2.0, 2.0, 240.0, 17.0),
                scale,
            );
            paint(
                &painter,
                ctx,
                cache,
                title,
                at(32.0, 2.0, 240.0, 17.0),
                scale,
            );
            // FUN_0046a9c0 draws the first page's title black and leaves the
            // white of "Target" set for the second page's.
            let title_color = match dialog.page {
                MissionDialogPage::Mission => egui::Color32::BLACK,
                MissionDialogPage::Agents => egui::Color32::WHITE,
            };
            let font = egui::FontId::proportional((11.0 * scale).max(7.0));
            let small = egui::FontId::proportional((8.0 * scale).max(5.0));
            painter.text(
                at(5.0, 2.0, 0.0, 0.0).min,
                egui::Align2::LEFT_TOP,
                CREATE_MISSION,
                font.clone(),
                title_color,
            );

            if button(
                ui,
                cache,
                at(242.0, 3.0, 14.0, 14.0),
                "close",
                CLOSE,
                false,
                scale,
            ) {
                close = true;
            }
            for (page, x, art) in [
                (MissionDialogPage::Mission, 0.0, MISSION_TAB[side]),
                (MissionDialogPage::Agents, 130.0, AGENTS_TAB[side]),
            ] {
                let rect = at(7.0 + x, 20.0, 116.0, 33.0);
                if button(
                    ui,
                    cache,
                    rect,
                    ("tab", x as i32),
                    art,
                    dialog.page == page,
                    scale,
                ) {
                    dialog.show_page(page);
                }
            }

            match dialog.page {
                MissionDialogPage::Mission => {
                    painter.text(
                        at(37.0, 195.0, 0.0, 0.0).min,
                        egui::Align2::LEFT_TOP,
                        TARGET,
                        font.clone(),
                        egui::Color32::WHITE,
                    );
                    let kind = dialog.kind();
                    let (x, y, w, h) = KIND_BOX;
                    paint_kind(
                        &painter,
                        ctx,
                        cache,
                        kind,
                        dialog.faction,
                        at(x, y, w, h),
                        scale,
                        &small,
                    );
                    if let Some(system) = world.systems.get(dialog.target) {
                        let (x, y, w, h) = TARGET_BOX;
                        paint_centered(
                            &painter,
                            ctx,
                            cache,
                            DllSource::Strategy,
                            planet_resource_id(system.dat_id),
                            at(x, y, w, h),
                            scale,
                        );
                        let (x, y, w) = TARGET_NAME;
                        painter.text(
                            at(x + w / 2.0, y, 0.0, 0.0).min,
                            egui::Align2::CENTER_TOP,
                            &system.name,
                            font.clone(),
                            egui::Color32::WHITE,
                        );
                    }
                    if button(
                        ui,
                        cache,
                        at(101.0, 174.0, 65.0, 18.0),
                        "missions",
                        MISSIONS,
                        false,
                        scale,
                    ) {
                        dialog.list_open = !dialog.list_open;
                    }
                }
                MissionDialogPage::Agents => {
                    paint(
                        &painter,
                        ctx,
                        cache,
                        AGENTS_HEADER[side],
                        at(8.0, 65.0, 108.0, 27.0),
                        scale,
                    );
                    paint(
                        &painter,
                        ctx,
                        cache,
                        DECOYS_HEADER[side],
                        at(136.0, 65.0, 108.0, 27.0),
                        scale,
                    );
                    for to_decoys in [false, true] {
                        let (x, y) = if to_decoys { DECOYS_LIST } else { AGENTS_LIST };
                        let list = at(x, y, LIST_SIZE.0, LIST_SIZE.1);
                        let (members, row) = if to_decoys {
                            (dialog.decoys.clone(), &mut dialog.decoys_row)
                        } else {
                            (dialog.agents.clone(), &mut dialog.agents_row)
                        };
                        let result = draw_list(
                            ui,
                            &painter,
                            cache,
                            world,
                            &members,
                            &dialog.selected,
                            row,
                            list,
                            scale,
                            to_decoys,
                            &small,
                        );
                        if let Some(member) = result.clicked {
                            dialog.toggle(member);
                        }
                        // A double click (notification 0x309) acts as the
                        // move out of its list (FUN_0046c3c0.c:80-86).
                        if let Some(member) = result.double_clicked {
                            dialog.selected.push(member);
                            dialog.move_selected(!to_decoys);
                        }
                    }
                    if button(
                        ui,
                        cache,
                        at(120.0, 136.0, 16.0, 16.0),
                        "to-decoys",
                        TO_DECOYS,
                        false,
                        scale,
                    ) {
                        dialog.move_selected(true);
                    }
                    if button(
                        ui,
                        cache,
                        at(120.0, 221.0, 16.0, 16.0),
                        "to-agents",
                        TO_AGENTS,
                        false,
                        scale,
                    ) {
                        dialog.move_selected(false);
                    }
                }
            }

            if button(
                ui,
                cache,
                at(170.0, 320.0, 64.0, 33.0),
                "cancel",
                CANCEL,
                false,
                scale,
            ) {
                close = true;
            }
            if button(
                ui,
                cache,
                at(102.0, 320.0, 64.0, 33.0),
                "begin",
                BEGIN,
                false,
                scale,
            ) {
                begin = true;
            }
            if button(
                ui,
                cache,
                at(33.0, 320.0, 64.0, 33.0),
                "encyclopedia",
                ENCYCLOPEDIA,
                false,
                scale,
            ) {
                action = Some(MissionDialogAction::Encyclopedia);
            }

            // The drop-down (FUN_0060cac0): a 200 by 117 window under the box
            // holding a one-item list at (2, 2). port: its scroll bar
            // (FUN_0060f640, base 0x299a) is not drawn; the wheel scrolls it.
            if dialog.list_open && dialog.page == MissionDialogPage::Mission {
                let (x, y, w, h) = KIND_BOX;
                let popup = at(x, y + h, w, h + 4.0);
                let item = at(x + 2.0, y + h + 2.0, w - 4.0, h);
                painter.rect_filled(popup, 0.0, egui::Color32::BLACK);
                let response = ui.interact(popup, ui.id().with("kind-list"), egui::Sense::click());
                if response.hovered() {
                    let wheel = ctx.input(|input| input.raw_scroll_delta.y);
                    dialog.select_kind(scrolled(dialog.kind, dialog.kinds.len(), wheel));
                }
                paint_kind(
                    &painter,
                    ctx,
                    cache,
                    dialog.kind(),
                    dialog.faction,
                    item,
                    scale,
                    &small,
                );
                // hyp: choosing an item closes the list, as a click outside
                // does (command 0x3e9 -> FUN_0060cbf0).
                if exact_clicked(&response, popup) {
                    dialog.list_open = false;
                }
            }
        });

    if begin {
        return state.begin();
    }
    if close {
        state.close();
    }
    action
}

/// One mission kind item: the 130 by 65 GOKRES icon centered in the box, and
/// its name. hyp: the name sits under the icon, centered.
#[expect(
    clippy::too_many_arguments,
    reason = "Keep explicit painter, cache and geometry inputs at this drawing boundary."
)]
fn paint_kind(
    painter: &egui::Painter,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    kind: MissionKind,
    faction: MissionFaction,
    bounds: egui::Rect,
    scale: f32,
    font: &egui::FontId,
) {
    let icon = paint_centered(
        painter,
        ctx,
        cache,
        DllSource::Gokres,
        kind_icon(kind, faction),
        bounds,
        scale,
    );
    let top = icon.map_or(bounds.center().y, |rect| rect.max.y + 2.0 * scale);
    painter.text(
        egui::pos2(bounds.center().x, top),
        egui::Align2::CENTER_TOP,
        kind_name(kind),
        font.clone(),
        KIND_TEXT,
    );
}

#[derive(Default)]
struct ListResult {
    clicked: Option<MissionMember>,
    double_clicked: Option<MissionMember>,
}

/// An agents or decoys list (`CoolDragList`, `FUN_00607ea0`): 107 by 59
/// items, each a centered mini with its name under it. port: its scroll bar
/// (base 0x29fc) is not drawn; the wheel scrolls it by one item.
#[expect(
    clippy::too_many_arguments,
    reason = "Keep explicit painter, cache and geometry inputs at this drawing boundary."
)]
fn draw_list(
    ui: &egui::Ui,
    painter: &egui::Painter,
    cache: &mut BmpCache,
    world: &GameWorld,
    members: &[MissionMember],
    selected: &[MissionMember],
    row: &mut usize,
    list: egui::Rect,
    scale: f32,
    decoys: bool,
    font: &egui::FontId,
) -> ListResult {
    let mut result = ListResult::default();
    let list_painter = painter.with_clip_rect(list);
    // The items sit over the list, so the pointer's position, not the list's
    // hover, decides where a wheel notch goes.
    let over = ui
        .ctx()
        .pointer_hover_pos()
        .is_some_and(|point| rect_contains(list, point));
    if over {
        let wheel = ui.ctx().input(|input| input.raw_scroll_delta.y);
        *row = scrolled(*row, members.len(), wheel);
    }
    for (index, &member) in members.iter().enumerate().skip(*row) {
        let offset = (index - *row) as f32 * ITEM_SIZE.1;
        if offset >= LIST_SIZE.1 {
            break;
        }
        let item = logical_rect(list, scale, 0.0, offset, ITEM_SIZE.0, ITEM_SIZE.1);
        let Some((mini, name)) = member_item(world, member) else {
            continue;
        };
        let image = mini.and_then(|id| {
            paint_centered(
                &list_painter,
                ui.ctx(),
                cache,
                DllSource::Gokres,
                id,
                item,
                scale,
            )
        });
        let color = if selected.contains(&member) {
            ITEM_TEXT_SELECTED
        } else {
            ITEM_TEXT
        };
        list_painter.text(
            egui::pos2(
                item.center().x,
                image.map_or(item.center().y, |rect| rect.max.y),
            ),
            egui::Align2::CENTER_TOP,
            name,
            font.clone(),
            color,
        );
        let clip = item.intersect(list);
        let response = ui.interact(
            clip,
            ui.id().with(("item", decoys, index)),
            egui::Sense::click(),
        );
        if response.double_clicked() {
            result.double_clicked = Some(member);
        } else if exact_clicked(&response, clip) {
            result.clicked = Some(member);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::world::Character;

    fn members(n: usize) -> Vec<MissionMember> {
        let mut world = GameWorld::default();
        (0..n)
            .map(|_| MissionMember::Character(world.characters.insert(Character::default())))
            .collect()
    }

    fn open_with(team: Vec<MissionMember>) -> MissionDialogState {
        let mut state = MissionDialogState::default();
        assert!(state.open(
            MissionFaction::Alliance,
            SystemKey::default(),
            team,
            vec![MissionKind::Diplomacy, MissionKind::Espionage],
        ));
        state
    }

    #[test]
    fn a_team_with_no_legal_mission_opens_no_dialog() {
        // FUN_0042a320: an empty preflight list drops the order instead.
        let mut state = MissionDialogState::default();

        let opened = state.open(
            MissionFaction::Alliance,
            SystemKey::default(),
            members(1),
            vec![],
        );

        assert!(!opened);
        assert!(!state.is_open());
    }

    #[test]
    fn the_dialog_opens_on_the_mission_page_with_the_team_as_agents_and_no_decoys() {
        // FUN_00402720 builds order 0x240 with +0x58 empty; tab 0x97 starts
        // selected (FUN_0060d700(.., 1, 0)).
        let team = members(2);
        let state = open_with(team.clone());
        let dialog = state.dialog().unwrap();

        assert_eq!(dialog.agents(), team.as_slice());
        assert!(dialog.decoys().is_empty());
        assert_eq!(dialog.page(), MissionDialogPage::Mission);
        assert_eq!(dialog.kind(), MissionKind::Diplomacy);
    }

    #[test]
    fn selected_agents_move_to_the_end_of_the_decoys_in_order() {
        // FUN_0046c3c0 case 0xca.
        let team = members(4);
        let mut state = open_with(team.clone());
        let dialog = state.dialog_mut().unwrap();
        dialog.toggle(team[1]);
        dialog.move_selected(true);
        dialog.toggle(team[3]);
        dialog.toggle(team[0]);

        dialog.move_selected(true);

        assert_eq!(dialog.agents(), &[team[2]]);
        assert_eq!(dialog.decoys(), &[team[1], team[0], team[3]]);
    }

    #[test]
    fn selected_decoys_move_back_to_the_agents_and_the_selection_clears() {
        // FUN_0046c3c0 case 0xcb clears each moved item's +0x3c bit 0.
        let team = members(2);
        let mut state = open_with(team.clone());
        let dialog = state.dialog_mut().unwrap();
        dialog.toggle(team[0]);
        dialog.move_selected(true);
        dialog.toggle(team[0]);

        dialog.move_selected(false);
        dialog.move_selected(true);

        assert_eq!(dialog.agents(), &[team[1], team[0]]);
        assert!(dialog.decoys().is_empty());
    }

    #[test]
    fn a_move_takes_only_selected_members_of_its_source_list() {
        let team = members(2);
        let mut state = open_with(team.clone());
        let dialog = state.dialog_mut().unwrap();
        dialog.toggle(team[0]);

        dialog.move_selected(false);

        assert_eq!(dialog.agents(), team.as_slice());
        assert!(dialog.decoys().is_empty());
    }

    #[test]
    fn begin_mission_submits_the_chosen_kind_with_both_lists_and_closes() {
        // Case 0x66 sets the order's +0x4c to the selected kind, submits it
        // (FUN_0041ce20), and falls through to the close.
        let team = members(2);
        let mut state = open_with(team.clone());
        let target = state.dialog().unwrap().target;
        let dialog = state.dialog_mut().unwrap();
        dialog.select_kind(1);
        dialog.toggle(team[1]);
        dialog.move_selected(true);

        let action = state.begin();

        assert_eq!(
            action,
            Some(MissionDialogAction::Begin {
                kind: MissionKind::Espionage,
                faction: MissionFaction::Alliance,
                team: vec![team[0]],
                decoys: vec![team[1]],
                target,
            })
        );
        assert!(!state.is_open());
    }

    /// The recovered Alliance cockpit at 640 by 480, scaled and offset.
    #[test]
    fn a_bitmap_wider_than_its_button_is_cropped_not_stretched() {
        // FUN_0046a9c0 makes "Cancel" 0x40 by 0x21 (64 by 33); STRATEGY 10596
        // is 66 by 33, and FUN_005fc140 blits it at its own size.
        let control = egui::Rect::from_min_size(egui::pos2(170.0, 320.0), egui::vec2(64.0, 33.0));

        let drawn = native(control, egui::vec2(66.0, 33.0), 1.0);

        assert_eq!(
            drawn,
            egui::Rect::from_min_size(control.min, egui::vec2(66.0, 33.0))
        );
        let doubled = native(control, egui::vec2(66.0, 33.0), 2.0);
        assert_eq!(doubled.size(), egui::vec2(132.0, 66.0));
    }

    #[test]
    fn the_dialog_sits_on_whole_pixels_in_both_cockpits() {
        // port: the original blits at integer coordinates; a half-pixel
        // origin would resample every bitmap. Both galaxy views center the
        // 259 by 355 window on a half pixel at 640 by 480.
        for faction in [
            crate::cockpit::CockpitFaction::Alliance,
            crate::cockpit::CockpitFaction::Empire,
        ] {
            let layout = crate::cockpit::CockpitState::new(faction).layout_for(640.0, 480.0);

            let rect = dialog_rect(layout);

            assert_eq!(rect.min, rect.min.round(), "{faction:?}");
            assert_eq!(
                rect.size(),
                egui::vec2(MISSION_DIALOG_WIDTH, MISSION_DIALOG_HEIGHT)
            );
        }
    }

    fn layout(scale: f32) -> CockpitLayout {
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

    fn press(pos: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        }
    }

    /// Run one frame per entry of `frames` with the pointer at dialog pixel
    /// `(x, y)`, after two idle frames that let the new area lay out.
    fn drive(
        world: &GameWorld,
        state: &mut MissionDialogState,
        (x, y): (f32, f32),
        frames: Vec<Vec<egui::Event>>,
    ) -> Option<MissionDialogAction> {
        let layout = layout(1.0);
        let pos = dialog_rect(layout).min + egui::vec2(x, y);
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let mut emitted = None;
        for extra in [vec![], vec![]].into_iter().chain(frames) {
            let mut events = vec![egui::Event::PointerMoved(pos)];
            events.extend(extra.into_iter().map(|event| match event {
                egui::Event::PointerButton { pressed, .. } => press(pos, pressed),
                other => other,
            }));
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(700.0, 520.0),
                )),
                events,
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                if let Some(action) = draw_mission_dialog(ctx, world, state, layout, &mut cache) {
                    emitted = Some(action);
                }
            });
        }
        emitted
    }

    /// Press and release the primary button at dialog pixel `(x, y)`.
    fn click(
        world: &GameWorld,
        state: &mut MissionDialogState,
        point: (f32, f32),
    ) -> Option<MissionDialogAction> {
        let at = egui::Pos2::ZERO;
        drive(
            world,
            state,
            point,
            vec![vec![press(at, true)], vec![press(at, false)]],
        )
    }

    /// Two clicks in a row at `(x, y)`, a double click.
    fn double_click(world: &GameWorld, state: &mut MissionDialogState, point: (f32, f32)) {
        let at = egui::Pos2::ZERO;
        let click = [vec![press(at, true)], vec![press(at, false)]];
        drive(
            world,
            state,
            point,
            click.iter().chain(&click).cloned().collect(),
        );
    }

    /// One wheel notch down over `(x, y)`.
    fn wheel_down(world: &GameWorld, state: &mut MissionDialogState, point: (f32, f32)) {
        let notch = egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Line,
            delta: egui::vec2(0.0, -1.0),
            modifiers: egui::Modifiers::default(),
        };
        drive(world, state, point, vec![vec![notch], vec![]]);
    }

    /// A world holding `n` characters, and a dialog open with them as agents.
    fn dialog_in_world(n: usize) -> (GameWorld, Vec<MissionMember>, MissionDialogState) {
        let mut world = GameWorld::default();
        let team: Vec<MissionMember> = (0..n)
            .map(|index| {
                MissionMember::Character(world.characters.insert(Character {
                    name: format!("Agent {index}"),
                    ..Character::default()
                }))
            })
            .collect();
        let state = open_with(team.clone());
        (world, team, state)
    }

    #[test]
    fn the_middle_bottom_button_begins_the_mission() {
        // FUN_0046a9c0: control 0x66 at (102, 320), 64 by 33, "Begin Mission".
        let (world, team, mut state) = dialog_in_world(1);

        let action = click(&world, &mut state, (134.0, 336.0));

        assert!(matches!(
            action,
            Some(MissionDialogAction::Begin { kind: MissionKind::Diplomacy, team: ref agents, .. })
                if *agents == team
        ));
        assert!(!state.is_open());
    }

    #[test]
    fn the_right_bottom_button_and_the_close_box_discard_the_order() {
        // Controls 0x65 "Cancel" at (170, 320) and 0x64 at (242, 3) destroy the
        // order and close.
        for point in [(202.0, 336.0), (249.0, 10.0)] {
            let (world, _, mut state) = dialog_in_world(1);

            let action = click(&world, &mut state, point);

            assert_eq!(action, None);
            assert!(!state.is_open());
        }
    }

    #[test]
    fn the_left_bottom_button_opens_the_encyclopedia_and_keeps_the_dialog() {
        // Control 0x67 at (33, 320) -> FUN_00429f30.
        let (world, _, mut state) = dialog_in_world(1);

        let action = click(&world, &mut state, (65.0, 336.0));

        assert_eq!(action, Some(MissionDialogAction::Encyclopedia));
        assert!(state.is_open());
    }

    #[test]
    fn the_second_tab_shows_the_agents_page_where_a_selected_agent_becomes_a_decoy() {
        // Tab 0x98 at (7 + 130, 20); the agents list at (8, 93) with 59-pixel
        // items; arrow 0xca at (120, 136).
        let (world, team, mut state) = dialog_in_world(2);

        click(&world, &mut state, (195.0, 36.0));
        assert_eq!(state.dialog().unwrap().page(), MissionDialogPage::Agents);
        click(&world, &mut state, (61.0, 93.0 + 59.0 + 29.0));
        click(&world, &mut state, (128.0, 144.0));

        let dialog = state.dialog().unwrap();
        assert_eq!(dialog.agents(), &[team[0]]);
        assert_eq!(dialog.decoys(), &[team[1]]);
    }

    #[test]
    fn the_dialog_stays_above_a_system_window_that_raises_itself_every_frame() {
        // FUN_0042a320 opens the dialog over the galaxy view's windows; a
        // focused system window raises itself each frame (system_window.rs).
        let (world, _, mut state) = dialog_in_world(1);
        let layout = layout(1.0);
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let window = egui::Id::new("raising_window");
        for _ in 0..3 {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(700.0, 520.0),
                )),
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                ctx.move_to_top(egui::LayerId::new(egui::Order::Foreground, window));
                egui::Area::new(window)
                    .order(egui::Order::Foreground)
                    .fixed_pos(egui::Pos2::ZERO)
                    .show(ctx, |ui| {
                        ui.allocate_response(egui::vec2(700.0, 520.0), egui::Sense::click());
                    });
                let _ = draw_mission_dialog(ctx, &world, &mut state, layout, &mut cache);
            });
        }

        assert_eq!(
            ctx.layer_id_at(dialog_rect(layout).center())
                .map(|layer| layer.id),
            Some(egui::Id::new("original-mission-dialog"))
        );
    }

    #[test]
    fn the_dialog_takes_the_pointer_only_over_itself_centered_in_the_galaxy_view() {
        // hyp: FUN_00606980 places the window in the galaxy view's rectangle.
        let layout = layout(2.0);
        let (_, _, mut state) = dialog_in_world(1);
        let center = (
            10.0 + (55.0 + 485.0 / 2.0) * 2.0,
            20.0 + (40.0 + 350.0 / 2.0) * 2.0,
        );
        let (half_w, half_h) = (MISSION_DIALOG_WIDTH, MISSION_DIALOG_HEIGHT);

        assert!(state.contains_screen_point(layout, center));
        assert!(
            state.contains_screen_point(layout, (center.0 - half_w + 1.0, center.1 - half_h + 1.0))
        );
        assert!(!state.contains_screen_point(layout, (center.0 + half_w + 1.0, center.1)));
        assert!(!state.contains_screen_point(layout, (center.0, center.1 - half_h - 1.0)));
        state.close();
        assert!(!state.is_open());
        assert!(!state.contains_screen_point(layout, center));
    }

    #[test]
    fn the_missions_button_opens_the_kind_list_and_a_click_on_it_closes_it() {
        // Control 0x68 at (101, 174) posts command 1000 to the box, which
        // opens its drop-down under the box (FUN_0060c210, FUN_0060cac0).
        let (world, _, mut state) = dialog_in_world(1);

        click(&world, &mut state, (133.0, 183.0));
        assert!(state.dialog().unwrap().list_open());
        // The drop-down is 117 pixels tall (113 * 1 + 4) from y = 175.
        click(&world, &mut state, (135.0, 290.0));

        assert!(!state.dialog().unwrap().list_open());
    }

    #[test]
    fn the_open_kind_list_leaves_the_bottom_buttons_free() {
        let (world, _, mut state) = dialog_in_world(1);
        click(&world, &mut state, (133.0, 183.0));

        let action = click(&world, &mut state, (134.0, 336.0));

        assert!(matches!(action, Some(MissionDialogAction::Begin { .. })));
    }

    #[test]
    fn a_wheel_notch_scrolls_the_agents_list_by_one_item() {
        // port: the lists' scroll bar (base 0x29fc) is not drawn.
        let (world, team, mut state) = dialog_in_world(3);
        click(&world, &mut state, (195.0, 36.0));
        wheel_down(&world, &mut state, (61.0, 150.0));

        click(&world, &mut state, (61.0, 93.0 + 29.0));
        click(&world, &mut state, (128.0, 144.0));

        assert_eq!(state.dialog().unwrap().decoys(), &[team[1]]);
    }

    #[test]
    fn a_double_click_on_an_agent_makes_it_a_decoy() {
        // FUN_0046c3c0: notification 0x309 on list 0xc8 becomes case 0xca.
        let (world, team, mut state) = dialog_in_world(2);
        click(&world, &mut state, (195.0, 36.0));

        double_click(&world, &mut state, (61.0, 93.0 + 29.0));

        let dialog = state.dialog().unwrap();
        assert_eq!(dialog.agents(), &[team[1]]);
        assert_eq!(dialog.decoys(), &[team[0]]);
    }

    #[test]
    fn a_wheel_step_moves_one_item_and_stays_in_the_list() {
        assert_eq!(scrolled(0, 3, -1.0), 1);
        assert_eq!(scrolled(2, 3, -1.0), 2);
        assert_eq!(scrolled(2, 3, 1.0), 1);
        assert_eq!(scrolled(0, 3, 1.0), 0);
        assert_eq!(scrolled(1, 3, 0.0), 1);
    }

    #[test]
    fn a_bitmap_is_centered_by_the_original_s_half_differences() {
        // FUN_0046a9c0: (0x6b - w) / 2, (0x3b - h) / 2 for a 107 by 59 item.
        let bounds = egui::Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(214.0, 118.0));

        let rect = centered(bounds, egui::vec2(61.0, 25.0), 2.0);

        assert_eq!(rect.min, egui::pos2(56.0, 54.0));
        assert_eq!(rect.size(), egui::vec2(122.0, 50.0));
    }

    #[test]
    fn a_member_shows_its_system_window_mini_and_name() {
        // FUN_0042c3b0(.., 0, 1) draws GOKRES (class +0x30 & 0xfff) + 0x4000,
        // the same mini the system window lists.
        let mut world = GameWorld::default();
        let luke = world.characters.insert(Character {
            name: "Luke Skywalker".into(),
            dat_id: rebellion_core::ids::DatId::new(576),
            is_major: true,
            ..Character::default()
        });
        let spies = world
            .special_forces
            .insert(rebellion_core::world::SpecialForceUnit {
                class_dat_id: rebellion_core::ids::DatId::new(0x3c00_0004),
                is_alliance: true,
                skills: [0; 8],
                on_mission: false,
            });

        assert_eq!(
            member_item(&world, MissionMember::Character(luke)),
            Some((Some(18_496), "Luke Skywalker".to_string()))
        );
        assert_eq!(
            member_item(&world, MissionMember::SpecialForce(spies)),
            Some((Some(17_731), "Bothan Spies".to_string()))
        );
    }

    #[test]
    fn each_kind_carries_its_textstra_name() {
        // TEXTSTRA 11280..11393, the MISSNSD records' names.
        assert_eq!(kind_name(MissionKind::Diplomacy), "Diplomacy");
        assert_eq!(kind_name(MissionKind::InciteUprising), "Incite Uprising");
        assert_eq!(
            kind_name(MissionKind::DeathStarSabotage),
            "Death Star Sabotage"
        );
        assert_eq!(kind_name(MissionKind::Assassination), "Assassination");
    }

    #[test]
    fn a_kind_past_the_list_is_not_chosen() {
        let mut state = open_with(members(1));
        let dialog = state.dialog_mut().unwrap();

        dialog.select_kind(2);

        assert_eq!(dialog.kind(), MissionKind::Diplomacy);
    }

    #[test]
    fn each_kind_s_icon_is_its_textstra_id_s_low_bits_plus_0x1000_for_the_empire() {
        // FUN_0046a9c0: GOKRES (record +0x30 & 0xfff), + 0x1000 off side 1.
        assert_eq!(
            kind_icon(MissionKind::Diplomacy, MissionFaction::Alliance),
            3088
        );
        assert_eq!(
            kind_icon(MissionKind::Diplomacy, MissionFaction::Empire),
            7184
        );
        assert_eq!(
            kind_icon(MissionKind::SubdueUprising, MissionFaction::Alliance),
            3200
        );
        assert_eq!(
            kind_icon(MissionKind::InciteUprising, MissionFaction::Empire),
            7232
        );
    }
}
