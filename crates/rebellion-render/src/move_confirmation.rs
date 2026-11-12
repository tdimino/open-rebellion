//! The original move confirmation window (`FUN_0044f060` builds it,
//! `FUN_0044f180` lays it out, `FUN_0044f5e0` handles its controls;
//! `ghidra/notes/move-order.md`, "The confirmation window").
//!
//! A 424 by 331 window that shows the order's transit time in days. The
//! checkmark submits the order without asking again; the X destroys it.
//! Enter and Escape press them (`FUN_0044f640`).

use std::fmt::Write as _;

use egui_macroquad::egui;
use rebellion_core::fleet_join::FleetMover;
use rebellion_core::ids::{FleetKey, SystemKey};
use rebellion_core::manufacturing::ProductionArea;
use rebellion_core::missions::{MissionFaction, MissionMember};
use rebellion_core::scrap::{ScrapOrder, ScrapTarget};
use rebellion_core::troop_transport::SquadronSource;
use rebellion_windowing::{
    ContentReport, ContentStatus, RoutingSnapshot, WindowId, WindowKind, WindowTransition,
};

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::CockpitLayout;
use crate::mission_dialog::{button, galaxy_centered_rect, paint};
use crate::native_geometry::{logical_rect, rect_contains};
use crate::singleton_window_common::{PreparedContextualWindow, PreparedWindowConfirm};

pub const MOVE_CONFIRMATION_WIDTH: f32 = 424.0;
pub const MOVE_CONFIRMATION_HEIGHT: f32 = 331.0;

// STRATEGY bitmaps (FUN_0044f180, FUN_0049a350).
const BACKGROUND: [u32; 2] = [11125, 11126];
const PICTURE: [u32; 2] = [1018, 1019];
const BLOCKADE_PICTURE: [u32; 2] = [1031, 1030];
const CONFIRM: (u32, u32) = (10926, 10927);
const CANCEL: (u32, u32) = (10929, 10930);

// Logical rectangles (x, y, width, height) in the window (FUN_0044f180).
const TEXT_BOX: (f32, f32, f32, f32) = (24.0, 242.0, 322.0, 70.0);
const CONFIRM_RECT: (f32, f32, f32, f32) = (355.0, 244.0, 51.0, 35.0);
const CANCEL_RECT: (f32, f32, f32, f32) = (355.0, 281.0, 51.0, 35.0);
const PICTURE_AT: (f32, f32) = (12.0, 30.0);

fn commit_replacing<T>(
    state: (&mut Option<T>, &mut Option<WindowId>),
    prepared: (T, Option<WindowId>),
    transition: &WindowTransition,
) -> Option<WindowId> {
    let (window, window_id) = state;
    let (confirmation, replacing) = prepared;
    let id = match *transition {
        WindowTransition::Opened { id }
            if window.is_none() && window_id.is_none() && replacing.is_none() =>
        {
            id
        }
        WindowTransition::Replaced { old, new }
            if *window_id == Some(old) && replacing == Some(old) =>
        {
            new
        }
        _ => return None,
    };
    *window = Some(confirmation);
    *window_id = Some(id);
    Some(id)
}

fn managed_layer_id(
    window_id: Option<WindowId>,
    id: WindowId,
    name: &'static str,
) -> Option<egui::LayerId> {
    (window_id == Some(id)).then(|| egui::LayerId::new(egui::Order::Tooltip, egui::Id::new(name)))
}

fn managed_hit(
    window_id: Option<WindowId>,
    kind: WindowKind,
    frame: &RoutingSnapshot,
    layout: CockpitLayout,
    point: (f32, f32),
) -> Option<WindowId> {
    let id = window_id?;
    (frame.key(id).is_some_and(|key| key.kind == kind)
        && frame.allows_input(id)
        && rect_contains(window_rect(layout), egui::pos2(point.0, point.1)))
    .then_some(id)
}

fn managed_keyboard_owner(
    window_id: Option<WindowId>,
    frame: &RoutingSnapshot,
) -> Option<WindowId> {
    let id = window_id?;
    matches!(
        frame.keyboard_route().target,
        rebellion_windowing::RouteTarget::Window(target) if target.window == id
    )
    .then_some(id)
}

fn managed_content_reports(
    window_id: Option<WindowId>,
    kind: WindowKind,
    frame: &RoutingSnapshot,
) -> Vec<ContentReport> {
    frame
        .paint_order()
        .iter()
        .copied()
        .filter(|id| frame.key(*id).is_some_and(|key| key.kind == kind))
        .map(|id| ContentReport {
            window: id,
            status: if window_id == Some(id) {
                ContentStatus::Ready
            } else {
                ContentStatus::Unpaintable
            },
        })
        .collect()
}

fn remove_managed_content<T>(
    window: &mut Option<T>,
    window_id: &mut Option<WindowId>,
    removed: &[WindowId],
) {
    if window_id.is_some_and(|id| removed.contains(&id)) {
        *window = None;
        *window_id = None;
    }
}

fn commit_managed_confirm<T, A: PartialEq>(
    window: &mut Option<T>,
    window_id: &mut Option<WindowId>,
    prepared_window: WindowId,
    prepared_action: A,
    removed: &[WindowId],
    current_action: impl FnOnce(&T) -> A,
) -> Option<A> {
    if !removed.contains(&prepared_window)
        || *window_id != Some(prepared_window)
        || current_action(window.as_ref()?) != prepared_action
    {
        return None;
    }
    *window = None;
    *window_id = None;
    Some(prepared_action)
}

fn contains_confirmation<T>(window: Option<&T>, layout: CockpitLayout, point: (f32, f32)) -> bool {
    window.is_some() && rect_contains(window_rect(layout), egui::pos2(point.0, point.1))
}

macro_rules! managed_hit_window {
    ($kind:expr) => {
        /// Manager-owned hit for event-local pointer routing.
        #[must_use]
        pub fn hit_window(
            &self,
            frame: &RoutingSnapshot,
            layout: CockpitLayout,
            point: (f32, f32),
        ) -> Option<WindowId> {
            managed_hit(self.window_id, $kind, frame, layout, point)
        }

        #[must_use]
        pub fn keyboard_owner(&self, frame: &RoutingSnapshot) -> Option<WindowId> {
            managed_keyboard_owner(self.window_id, frame)
        }
    };
}

macro_rules! impl_managed_confirmation_state {
    (
        $state:ty,
        $confirmation:ty,
        $prepared:ty,
        $prepared_confirm:ty,
        $action:ty,
        $kind:path,
        $layer_name:literal
    ) => {
        impl $state {
            /// Publishes unmanaged content for renderer fixtures and legacy calls.
            pub fn open(&mut self, confirmation: $confirmation) {
                self.window = Some(confirmation);
                self.window_id = None;
            }

            /// Publishes prepared content only for the matching manager receipt.
            pub fn commit_open(
                &mut self,
                prepared: $prepared,
                transition: &WindowTransition,
            ) -> Option<WindowId> {
                let (confirmation, replacing) = prepared.into_parts();
                commit_replacing(
                    (&mut self.window, &mut self.window_id),
                    (confirmation, replacing),
                    transition,
                )
            }

            #[must_use]
            pub fn is_open(&self) -> bool {
                self.window.is_some()
            }

            /// The open window's order, if one waits for an answer.
            #[must_use]
            pub fn confirmation(&self) -> Option<&$confirmation> {
                self.window.as_ref()
            }

            /// Exact manager identity owning the published content.
            #[must_use]
            pub const fn window_id(&self) -> Option<WindowId> {
                self.window_id
            }

            /// Egui layer attached to the exact manager-issued confirmation.
            #[must_use]
            pub fn managed_layer_id(&self, id: WindowId) -> Option<egui::LayerId> {
                managed_layer_id(self.window_id, id, $layer_name)
            }

            managed_hit_window!($kind);

            /// Explicit content state for fail-closed manager reconciliation.
            #[must_use]
            pub fn content_reports(&self, frame: &RoutingSnapshot) -> Vec<ContentReport> {
                managed_content_reports(self.window_id, $kind, frame)
            }

            #[must_use]
            pub const fn request_close(&self) -> Option<WindowId> {
                self.window_id
            }

            /// Releases manager-owned content only from its exact teardown receipt.
            pub fn remove_content(&mut self, removed: &[WindowId]) {
                remove_managed_content(&mut self.window, &mut self.window_id, removed);
            }

            /// Snapshots the exact current order without performing gameplay.
            #[must_use]
            pub fn prepare_confirm(&self) -> Option<$prepared_confirm> {
                Some(<$prepared_confirm>::new(
                    self.window_id?,
                    self.window.as_ref()?.confirm_action(),
                ))
            }

            /// Consumes one confirmation only after the exact manager close.
            pub fn commit_confirm(
                &mut self,
                prepared: $prepared_confirm,
                removed: &[WindowId],
            ) -> Option<$action> {
                let (window, action) = prepared.into_parts();
                commit_managed_confirm(
                    &mut self.window,
                    &mut self.window_id,
                    window,
                    action,
                    removed,
                    <$confirmation>::confirm_action,
                )
            }

            /// Whether `point` falls on the open window.
            #[must_use]
            pub fn contains_screen_point(&self, layout: CockpitLayout, point: (f32, f32)) -> bool {
                contains_confirmation(self.window.as_ref(), layout, point)
            }
        }
    };
}

/// TEXTSTRA `RT_RCDATA` 0x7057.
const TRANSIT_TIME: &str = "Transit time in days";
/// TEXTSTRA `RT_RCDATA` 0x7056.
const BLOCKADE_EVACUATION: &str = "Units evacuating from worlds under blockade risk being destroyed by blockading vessels.  Are you sure you want to proceed with the evacuation?\nTransit time in days";

/// A regiment or squadron of an order's team, moved by its own `0x204`
/// (`FUN_0053ca50`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitMover {
    Regiment(rebellion_core::ids::TroopKey),
    Squadron(SquadronSource),
}

/// A move waiting for the player's answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveConfirmation {
    pub faction: MissionFaction,
    /// The order's team: one fleet, or a fleet icon's fleets.
    pub fleets: Vec<FleetKey>,
    /// The order's characters and special forces (its `0x241` teams).
    pub members: Vec<MissionMember>,
    /// The order's capital ships, a [`FleetMover::Ships`] (their `0x204`
    /// moves).
    pub ships: Option<FleetMover>,
    /// The order's regiments and squadrons.
    pub units: Vec<UnitMover>,
    pub destination: SystemKey,
    /// The fleet the move joins, when it was released on a Fleet window
    /// (`ghidra/notes/fleet-join-split.md`); characters board it.
    pub join: Option<FleetKey>,
    /// The first member is not a fleet and stands in a blockaded system.
    pub blockaded: bool,
    /// One `(name, days)` line per moving object (`FUN_0053c2e0`).
    pub lines: Vec<(String, u32)>,
}

impl MoveConfirmation {
    /// The text box's contents, the confirmation's `+0x50`: the template,
    /// then `FUN_0049a8b0` appends a newline, the name, `":  "` and the days
    /// for each object.
    ///
    /// `FUN_0049a350` takes the blockade warning 0x7056 and pictures
    /// 1030/1031 only when the first member is not a fleet and its container
    /// is a blockaded system, so a fleet's move always shows 0x7057.
    #[must_use]
    pub fn text(&self) -> String {
        let mut text = if self.blockaded {
            BLOCKADE_EVACUATION
        } else {
            TRANSIT_TIME
        }
        .to_string();
        for (name, days) in &self.lines {
            let _ = write!(text, "\n{name}:  {days}");
        }
        text
    }

    /// The picture `+0x2e`: 1018 for side 1, 1019 otherwise; under the
    /// blockade warning 1031 for side 1, 1030 otherwise.
    #[must_use]
    pub fn picture(&self) -> u32 {
        if self.blockaded {
            BLOCKADE_PICTURE[side(self.faction)]
        } else {
            PICTURE[side(self.faction)]
        }
    }

    fn confirm_action(&self) -> MoveConfirmationAction {
        MoveConfirmationAction::Confirm {
            fleets: self.fleets.clone(),
            members: self.members.clone(),
            ships: self.ships.clone(),
            units: self.units.clone(),
            destination: self.destination,
            join: self.join,
        }
    }
}

/// What the player answered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveConfirmationAction {
    /// The checkmark (control `0x14`) or Enter: `FUN_0041ce20(order, 1)`
    /// validates the order again and submits it without asking.
    Confirm {
        fleets: Vec<FleetKey>,
        members: Vec<MissionMember>,
        ships: Option<FleetMover>,
        units: Vec<UnitMover>,
        destination: SystemKey,
        join: Option<FleetKey>,
    },
    /// The X (control `0x15`) or Escape: the order is destroyed.
    Cancel,
}

/// Whether a confirmation window is open.
#[derive(Debug, Clone, Default)]
pub struct MoveConfirmationState {
    window: Option<MoveConfirmation>,
    window_id: Option<WindowId>,
}

/// Validated Move content kept inert until the manager accepts its fresh
/// command-context identity.
pub type PreparedMoveConfirmation = PreparedContextualWindow<MoveConfirmation>;

/// Exact Move candidate paired with one manager close. It remains inert until
/// the matching teardown receipt releases it.
pub type PreparedMoveConfirmationConfirm = PreparedWindowConfirm<MoveConfirmationAction>;

impl MoveConfirmationState {
    /// Prepares one valid logical Move context without disturbing any
    /// published confirmation. Exactly one mover family must be present and
    /// every mover must retain its transit line.
    #[must_use]
    pub fn prepare_open(&self, confirmation: MoveConfirmation) -> Option<PreparedMoveConfirmation> {
        let family_count = usize::from(!confirmation.fleets.is_empty())
            + usize::from(!confirmation.members.is_empty())
            + usize::from(confirmation.ships.is_some())
            + usize::from(!confirmation.units.is_empty());
        let line_count = confirmation.fleets.len()
            + confirmation.members.len()
            + confirmation.ships.as_ref().map_or(0, |mover| match mover {
                FleetMover::Fleet(_) => 1,
                FleetMover::Ships { ships, .. } => ships.len(),
            })
            + confirmation.units.len();
        if family_count != 1 || confirmation.lines.len() != line_count {
            return None;
        }
        Some(PreparedMoveConfirmation::new(
            confirmation,
            self.window_id,
            WindowKind::MoveConfirmation,
        ))
    }

    /// Either control closes the window (`+0x30`) after acting.
    fn answer(&mut self, confirm: bool) -> Option<MoveConfirmationAction> {
        if self.window_id.is_some() {
            return None;
        }
        let window = self.window.take()?;
        Some(if confirm {
            window.confirm_action()
        } else {
            MoveConfirmationAction::Cancel
        })
    }
}

impl_managed_confirmation_state!(
    MoveConfirmationState,
    MoveConfirmation,
    PreparedMoveConfirmation,
    PreparedMoveConfirmationConfirm,
    MoveConfirmationAction,
    WindowKind::MoveConfirmation,
    "original-move-confirmation"
);

const fn side(faction: MissionFaction) -> usize {
    match faction {
        MissionFaction::Alliance => 0,
        MissionFaction::Empire => 1,
    }
}

/// The window's screen rectangle.
fn window_rect(layout: CockpitLayout) -> egui::Rect {
    galaxy_centered_rect(layout, MOVE_CONFIRMATION_WIDTH, MOVE_CONFIRMATION_HEIGHT)
}

/// Draw the open window, if any, and report the player's answer.
pub fn draw_move_confirmation(
    ctx: &egui::Context,
    state: &mut MoveConfirmationState,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Option<MoveConfirmationAction> {
    let managed = state.window_id().is_some();
    let window = state.window.as_ref()?;
    let answer = draw_window(
        ctx,
        "original-move-confirmation",
        side(window.faction),
        window.picture(),
        window.text(),
        layout,
        cache,
    );
    match answer {
        Some(true) if managed => state.window.as_ref().map(MoveConfirmation::confirm_action),
        Some(false) if managed => Some(MoveConfirmationAction::Cancel),
        Some(confirm) => state.answer(confirm),
        None => None,
    }
}

/// The confirmation window every confirmed order shares (`FUN_0044f060`,
/// `FUN_0044f180`): the side's background, the order's picture at (12, 30),
/// its text, the checkmark and the X. `Some(true)` for the checkmark or
/// Enter, `Some(false)` for the X or Escape (`FUN_0044f640`).
/// The field's font is font 4 (`FUN_00420550`): `FUN_0060eed0`'s default
/// case, Arial with a 16-pixel cell at weight 400. Its lines are a cell
/// apart, each baseline the cell's ascent (13) below the line's top.
/// hyp (measured): the original's glyphs are Arial's at a 13-pixel em, not
/// the cell's 14.3: the Stop question's first line is 285 pixels wide,
/// Arial's hinted advances at that size. Those run about 3% wider than the
/// outline advances egui lays out, so a 13.4 em gives the same widths.
const TEXT_EM: f32 = 13.4;
const TEXT_LINE_HEIGHT: f32 = 16.0;
const TEXT_ASCENT: f32 = 13.0;

/// The checkmark or X: a `CoolStrobeButton` (`FUN_00602150`) that
/// `FUN_0044f180` builds 51 by 35, its bitmaps' own size, without a last
/// argument (`FUN_0044f180.c:129-130`). That sets `+0xc4` bit 0x10
/// (`FUN_00602150.c:115-117`), so a press and its release count only on an
/// opaque pixel of the bitmap, never on its outer edge
/// (`FUN_006035f0.c:28-31` → `FUN_005fca00`). port: with the bitmap not
/// loaded the whole rect counts.
fn bitmap_button(
    ui: &egui::Ui,
    cache: &mut BmpCache,
    rect: egui::Rect,
    id: &str,
    bitmaps: (u32, u32),
    scale: f32,
) -> bool {
    let (press, release) = ui
        .ctx()
        .input(|input| (input.pointer.press_origin(), input.pointer.interact_pos()));
    let clicked = button(ui, cache, rect, id, bitmaps, false, scale);
    let mut on_bitmap = |point: egui::Pos2| {
        if cache
            .original_resource_size(DllSource::Strategy, bitmaps.0)
            .is_none()
        {
            return true;
        }
        let local = (point - rect.min) / scale;
        local.x >= 0.0
            && local.y >= 0.0
            && cache.is_resource_hit(
                DllSource::Strategy,
                bitmaps.0,
                local.x.floor() as usize,
                local.y.floor() as usize,
            )
    };
    clicked && release.is_some_and(&mut on_bitmap) && press.is_none_or(&mut on_bitmap)
}

/// The field's text, broken between words to the field's width
/// (`FUN_0041fd00`: `DrawTextA` format 0x2410, `DT_WORDBREAK`), in white
/// (`+0xb4`, 0x2ffffff).
fn text_job(text: String, scale: f32) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::simple(
        text,
        egui::FontId::proportional(TEXT_EM * scale),
        egui::Color32::WHITE,
        TEXT_BOX.2 * scale,
    );
    for section in &mut job.sections {
        section.format.line_height = Some(TEXT_LINE_HEIGHT * scale);
    }
    job
}

/// Where the field's galley is painted for its first baseline to sit
/// `TEXT_ASCENT` below the field's top, as `DrawTextA` places it. An empty
/// field has no baseline and stays at the field's corner.
fn text_origin(galley: &egui::Galley, field: egui::Pos2, scale: f32) -> egui::Pos2 {
    let shift = galley
        .rows
        .first()
        .and_then(|row| row.glyphs.first())
        .map_or(0.0, |glyph| TEXT_ASCENT * scale - glyph.pos.y);
    field + egui::vec2(0.0, shift)
}

fn draw_window(
    ctx: &egui::Context,
    id: &str,
    side: usize,
    picture: u32,
    text: String,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Option<bool> {
    let scale = layout.scale;
    let rect = window_rect(layout);
    let mut answer = None;

    // Above the modeless windows, as the mission dialog is.
    egui::Area::new(egui::Id::new(id))
        .fixed_pos(rect.min)
        .order(egui::Order::Tooltip)
        .show(ctx, |ui| {
            let (frame, _) = ui.allocate_exact_size(rect.size(), egui::Sense::hover());
            let painter = ui.painter().with_clip_rect(frame);
            let at = |(x, y, w, h): (f32, f32, f32, f32)| logical_rect(frame, scale, x, y, w, h);

            paint(&painter, ctx, cache, BACKGROUND[side], frame, scale);
            // FUN_005fcc30 blits the picture at its own size from (12, 30).
            let (x, y) = PICTURE_AT;
            let corner = at((x, y, 0.0, 0.0)).min;
            let picture_rect = egui::Rect::from_min_max(corner, frame.max);
            paint(&painter, ctx, cache, picture, picture_rect, scale);

            // port: the field does not scroll.
            let text_box = at(TEXT_BOX);
            let galley = painter.layout_job(text_job(text, scale));
            painter.with_clip_rect(text_box).galley(
                text_origin(&galley, text_box.min, scale),
                galley,
                egui::Color32::WHITE,
            );

            if bitmap_button(ui, cache, at(CONFIRM_RECT), "confirm", CONFIRM, scale) {
                answer = Some(true);
            }
            if bitmap_button(ui, cache, at(CANCEL_RECT), "cancel", CANCEL, scale) {
                answer = Some(false);
            }
        });

    // FUN_0044f640: Enter is control 0x14 and Escape control 0x15.
    let (enter, escape) = ctx.input(|input| {
        (
            input.key_pressed(egui::Key::Enter),
            input.key_pressed(egui::Key::Escape),
        )
    });
    if enter {
        answer = Some(true);
    } else if escape {
        answer = Some(false);
    }
    answer
}

// ---------------------------------------------------------------------------
// Scrap (0x200)
// ---------------------------------------------------------------------------

/// TEXTSTRA `RT_RCDATA` 0x7050.
const SCRAP_QUESTION: &str = "Are you sure you want to scrap the following units?";
/// STRATEGY pictures for Scrap (`FUN_0049a350` case 0x200: `+0x2e` is
/// 0x408 for side 1, else 0x409).
const SCRAP_PICTURE: [u32; 2] = [1032, 1033];

/// A Scrap order waiting for the player's answer: `FUN_00487cc0` always
/// confirms kind 0x200.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrapConfirmation {
    pub faction: MissionFaction,
    /// Original semantic order, retained so confirmation can rederive the
    /// current target set before gameplay mutates anything.
    pub order: ScrapOrder,
    /// The order's objects.
    pub targets: Vec<ScrapTarget>,
    /// Each object's name, in order.
    pub names: Vec<String>,
}

impl ScrapConfirmation {
    /// The text box: 0x7050, then 0x7054 (`"\n"` and the name, the
    /// object's `+0x30`) for each object (`FUN_0049a880`).
    #[must_use]
    pub fn text(&self) -> String {
        let mut text = SCRAP_QUESTION.to_string();
        for name in &self.names {
            text.push('\n');
            text.push_str(name);
        }
        text
    }

    /// The picture `+0x2e`: 1032 for side 1, 1033 otherwise.
    #[must_use]
    pub fn picture(&self) -> u32 {
        SCRAP_PICTURE[side(self.faction)]
    }
}

/// What the player answered a Scrap confirmation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScrapConfirmationAction {
    /// The checkmark or Enter: scrap every object.
    Confirm {
        faction: MissionFaction,
        order: ScrapOrder,
        targets: Vec<ScrapTarget>,
    },
    /// The X or Escape: the order is destroyed.
    Cancel,
}

/// Whether a Scrap confirmation is open.
#[derive(Debug, Clone, Default)]
pub struct ScrapConfirmationState {
    window: Option<ScrapConfirmation>,
    window_id: Option<WindowId>,
}

/// Validated Scrap content held inert until its explicit port-modal identity
/// commits in the shared manager.
pub type PreparedScrapConfirmation = PreparedContextualWindow<ScrapConfirmation>;

/// Exact Scrap candidate paired with one manager close.
pub type PreparedScrapConfirmationConfirm = PreparedWindowConfirm<ScrapConfirmationAction>;

impl ScrapConfirmation {
    fn confirm_action(&self) -> ScrapConfirmationAction {
        ScrapConfirmationAction::Confirm {
            faction: self.faction,
            order: self.order,
            targets: self.targets.clone(),
        }
    }
}

impl ScrapConfirmationState {
    #[must_use]
    pub fn prepare_open(
        &self,
        confirmation: ScrapConfirmation,
    ) -> Option<PreparedScrapConfirmation> {
        if confirmation.targets.is_empty() || confirmation.names.is_empty() {
            return None;
        }
        Some(PreparedScrapConfirmation::new(
            confirmation,
            self.window_id,
            WindowKind::ScrapConfirmation,
        ))
    }
}

impl_managed_confirmation_state!(
    ScrapConfirmationState,
    ScrapConfirmation,
    PreparedScrapConfirmation,
    PreparedScrapConfirmationConfirm,
    ScrapConfirmationAction,
    WindowKind::ScrapConfirmation,
    "original-scrap-confirmation"
);

/// Draw the open Scrap confirmation, if any, and report the answer.
pub fn draw_scrap_confirmation(
    ctx: &egui::Context,
    state: &mut ScrapConfirmationState,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Option<ScrapConfirmationAction> {
    let managed = state.window_id().is_some();
    let window = state.window.as_ref()?;
    let confirm = draw_window(
        ctx,
        "original-scrap-confirmation",
        side(window.faction),
        window.picture(),
        window.text(),
        layout,
        cache,
    )?;
    if managed {
        return Some(if confirm {
            state.window.as_ref()?.confirm_action()
        } else {
            ScrapConfirmationAction::Cancel
        });
    }
    let window = state.window.take()?;
    Some(if confirm {
        window.confirm_action()
    } else {
        ScrapConfirmationAction::Cancel
    })
}

// ---------------------------------------------------------------------------
// Stop (0x213)
// ---------------------------------------------------------------------------

/// TEXTSTRA `RT_RCDATA` 0x7053.
const STOP_QUESTION: &str = "Are you sure you want to stop construction of the following?";
/// STRATEGY pictures for Stop (`FUN_0049a350` case 0x213: `+0x2e` is 0x40e
/// for side 1, else 0x40f).
const STOP_PICTURE: [u32; 2] = [1038, 1039];

/// A Stop order on a manufacturing band waiting for the player's answer:
/// `FUN_00487cc0` confirms kind 0x213 as it does Scrap, for ship, troop and
/// facility bands alike (each band's Stop is the same order).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StopConfirmation {
    pub faction: MissionFaction,
    pub system: SystemKey,
    pub area: ProductionArea,
    /// One `(units, class name)` per band the order stops.
    pub lines: Vec<(usize, String)>,
}

impl StopConfirmation {
    /// The Stop of `system`'s `area`, listing what the band builds: the
    /// units left and the product's name, as the band shows them
    /// ([`crate::manufacturing_window::band`]). `None` for an idle band,
    /// which has nothing to stop.
    /// port: the original's line comes from the band's manager; the area's
    /// queue stands in for it, as it does for the band.
    #[must_use]
    pub fn for_band(
        world: &rebellion_core::world::GameWorld,
        manufacturing: &rebellion_core::manufacturing::ManufacturingState,
        faction: MissionFaction,
        system: SystemKey,
        area: ProductionArea,
    ) -> Option<Self> {
        let queue = manufacturing.queue(system, area)?;
        let active = queue.active()?;
        let (name, _) = crate::manufacturing_window::product(world, active.kind);
        Some(Self {
            faction,
            system,
            area,
            lines: vec![(queue.len(), name)],
        })
    }

    /// The text box: 0x7053, then 0x7055 (`"\n"`, the units, two spaces
    /// and the name, items 6 and 5) for each band (`FUN_0049a350`).
    #[must_use]
    pub fn text(&self) -> String {
        let mut text = STOP_QUESTION.to_string();
        for (units, name) in &self.lines {
            let _ = write!(text, "\n{units}  {name}");
        }
        text
    }

    /// The picture `+0x2e`: 1038 for side 1, 1039 otherwise.
    #[must_use]
    pub fn picture(&self) -> u32 {
        STOP_PICTURE[side(self.faction)]
    }
}

/// What the player answered a Stop confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopConfirmationAction {
    /// The checkmark or Enter: the band stops (`FUN_0041ce20(order, 1)`).
    Confirm {
        system: SystemKey,
        area: ProductionArea,
    },
    /// The X or Escape: the order is destroyed and the band builds on.
    Cancel,
}

/// Whether a Stop confirmation is open.
#[derive(Debug, Clone, Default)]
pub struct StopConfirmationState {
    window: Option<StopConfirmation>,
    window_id: Option<WindowId>,
}

impl StopConfirmationState {
    /// Compatibility open for renderer-only tests. Production publishes the
    /// content only after the manager accepts its identity.
    pub fn open(&mut self, confirmation: StopConfirmation) {
        self.window = Some(confirmation);
        self.window_id = None;
    }

    /// Publishes inert Stop content for the exact accepted manager identity.
    pub fn commit_open(
        &mut self,
        confirmation: StopConfirmation,
        transition: &WindowTransition,
    ) -> Option<WindowId> {
        let WindowTransition::Opened { id } = *transition else {
            return None;
        };
        if self.window.is_some() || self.window_id.is_some() {
            return None;
        }
        self.window = Some(confirmation);
        self.window_id = Some(id);
        Some(id)
    }

    #[must_use]
    pub fn is_open(&self) -> bool {
        self.window.is_some()
    }

    #[must_use]
    pub fn confirmation(&self) -> Option<&StopConfirmation> {
        self.window.as_ref()
    }

    #[must_use]
    pub const fn window_id(&self) -> Option<WindowId> {
        self.window_id
    }

    /// Egui layer attached to the exact manager-issued Stop confirmation.
    #[must_use]
    pub fn managed_layer_id(&self, id: WindowId) -> Option<egui::LayerId> {
        managed_layer_id(self.window_id, id, "original-stop-confirmation")
    }

    managed_hit_window!(WindowKind::StopConfirmation);

    #[must_use]
    pub fn content_reports(&self, frame: &RoutingSnapshot) -> Vec<ContentReport> {
        self.window_id
            .filter(|id| frame.key(*id).is_some())
            .map(|window| ContentReport {
                window,
                status: ContentStatus::Ready,
            })
            .into_iter()
            .collect()
    }

    pub fn remove_content(&mut self, removed: &[WindowId]) {
        remove_managed_content(&mut self.window, &mut self.window_id, removed);
    }

    /// Whether `point` falls on the open window.
    #[must_use]
    pub fn contains_screen_point(&self, layout: CockpitLayout, point: (f32, f32)) -> bool {
        contains_confirmation(self.window.as_ref(), layout, point)
    }
}

/// Draw the open Stop confirmation, if any, and report the answer.
pub fn draw_stop_confirmation(
    ctx: &egui::Context,
    state: &mut StopConfirmationState,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Option<StopConfirmationAction> {
    let window = state.window.as_ref()?;
    let confirm = draw_window(
        ctx,
        "original-stop-confirmation",
        side(window.faction),
        window.picture(),
        window.text(),
        layout,
        cache,
    )?;
    let action = if confirm {
        StopConfirmationAction::Confirm {
            system: window.system,
            area: window.area,
        }
    } else {
        StopConfirmationAction::Cancel
    };
    if state.window_id.is_none() {
        state.window = None;
    }
    Some(action)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cockpit::CockpitViewport;
    use crate::test_support::{key_press as key, offset_strategic_layout, pointer_button as press};
    use rebellion_core::scrap::ScrapOrder;
    use rebellion_windowing::{
        CaptureCancelReason, InputPolicy, SubjectId, WindowCommand, WindowKind, WindowManager,
        WindowOwner, WindowTransition,
    };

    fn confirmation(faction: MissionFaction) -> MoveConfirmation {
        MoveConfirmation {
            faction,
            fleets: vec![FleetKey::default()],
            members: Vec::new(),
            ships: None,
            units: Vec::new(),
            destination: SystemKey::default(),
            join: None,
            blockaded: false,
            lines: vec![("Red Fleet".into(), 12)],
        }
    }

    fn open(faction: MissionFaction) -> MoveConfirmationState {
        let mut state = MoveConfirmationState::default();
        state.open(confirmation(faction));
        state
    }

    fn layout() -> CockpitLayout {
        offset_strategic_layout(1.0)
    }

    fn run_confirmation_frames<S, A>(
        layout: CockpitLayout,
        mut cache: BmpCache,
        state: &mut S,
        frames: impl IntoIterator<Item = Vec<egui::Event>>,
        mut draw: impl FnMut(&egui::Context, &mut S, CockpitLayout, &mut BmpCache) -> Option<A>,
    ) -> Option<A> {
        let ctx = egui::Context::default();
        let mut emitted = None;
        for events in frames {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(700.0, 520.0) * layout.scale,
                )),
                events,
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                if let Some(action) = draw(ctx, state, layout, &mut cache) {
                    emitted = Some(action);
                }
            });
        }
        emitted
    }

    fn drive_confirmation<S, A>(
        layout: CockpitLayout,
        cache: BmpCache,
        state: &mut S,
        (x, y): (f32, f32),
        frames: Vec<Vec<egui::Event>>,
        draw: impl FnMut(&egui::Context, &mut S, CockpitLayout, &mut BmpCache) -> Option<A>,
    ) -> Option<A> {
        let pos = window_rect(layout).min + egui::vec2(x, y) * layout.scale;
        let frames = [vec![], vec![]].into_iter().chain(frames).map(|extra| {
            let mut events = vec![egui::Event::PointerMoved(pos)];
            events.extend(extra.into_iter().map(|event| match event {
                egui::Event::PointerButton { pressed, .. } => press(pos, pressed),
                other => other,
            }));
            events
        });
        run_confirmation_frames(layout, cache, state, frames, draw)
    }

    /// Run two idle frames, then one frame per entry of `frames`, with the
    /// pointer at window pixel `(x, y)`.
    fn drive(
        state: &mut MoveConfirmationState,
        point: (f32, f32),
        frames: Vec<Vec<egui::Event>>,
    ) -> Option<MoveConfirmationAction> {
        drive_with(BmpCache::new(), state, point, frames)
    }

    fn drive_with(
        cache: BmpCache,
        state: &mut MoveConfirmationState,
        point: (f32, f32),
        frames: Vec<Vec<egui::Event>>,
    ) -> Option<MoveConfirmationAction> {
        drive_in(layout(), cache, state, point, frames)
    }

    /// As [`drive_with`], on `layout`: `(x, y)` is a window pixel, which
    /// the layout's scale enlarges.
    fn drive_in(
        layout: CockpitLayout,
        cache: BmpCache,
        state: &mut MoveConfirmationState,
        point: (f32, f32),
        frames: Vec<Vec<egui::Event>>,
    ) -> Option<MoveConfirmationAction> {
        drive_confirmation(layout, cache, state, point, frames, draw_move_confirmation)
    }

    fn click(
        state: &mut MoveConfirmationState,
        point: (f32, f32),
    ) -> Option<MoveConfirmationAction> {
        let at = egui::Pos2::ZERO;
        drive(
            state,
            point,
            vec![vec![press(at, true)], vec![press(at, false)]],
        )
    }

    /// A cache holding 51 by 35 checkmark and X bitmaps whose palette key
    /// (the first stored pixel) fills the bottom-left corner pixel, as the
    /// STRATEGY bitmaps 10926 and 10929 leave a few corner pixels keyed.
    /// port: test fixture values, not rules.
    fn keyed_buttons() -> BmpCache {
        static NEXT_FIXTURE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        let fixture = NEXT_FIXTURE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("confirm-hit-mask-{}-{fixture}", std::process::id()));
        let dir = root.join("strategy-dll/BMP");
        std::fs::create_dir_all(&dir).unwrap();
        let (width, height, stride, offset) = (51usize, 35usize, 52usize, 1078usize);
        let mut bmp = crate::test_support::bitmap_fixture(width, height, stride, offset, 8, 3);
        // The bottom row, stored first: its first pixels are the key.
        bmp[offset..offset + 4].fill(252);
        for id in [10_926, 10_929] {
            std::fs::write(dir.join(format!("{id}.bmp")), &bmp).unwrap();
        }
        let mut cache = BmpCache::new();
        cache.set_base_path(&root);
        cache
    }

    #[test]
    fn the_checkmark_and_x_take_a_click_only_on_their_bitmaps() {
        // FUN_0044f180 makes both buttons CoolStrobeButtons (FUN_00602150)
        // without a last argument (FUN_0044f180.c:129-130), which sets
        // +0xc4 bit 0x10 (FUN_00602150.c:115-117): a press or release hits
        // only an opaque pixel of the bitmap (FUN_006035f0.c:28-31 →
        // FUN_005fca00), never its outer edge. The 51 by 35 control is the
        // bitmap's own size, so a click on the bitmap's frame still counts.
        for faction in [MissionFaction::Alliance, MissionFaction::Empire] {
            for (rect, answer) in [
                (CONFIRM_RECT, MoveConfirmationAction::Cancel),
                (CANCEL_RECT, MoveConfirmationAction::Cancel),
            ] {
                let (x, y, _, height) = rect;
                let mut state = open(faction);
                let keyed = drive_with(
                    keyed_buttons(),
                    &mut state,
                    (x + 1.5, y + height - 0.5),
                    vec![
                        vec![press(egui::Pos2::ZERO, true)],
                        vec![press(egui::Pos2::ZERO, false)],
                    ],
                );
                assert_eq!(keyed, None, "{faction:?} {rect:?}");
                assert!(state.is_open());
                let edge = drive_with(
                    keyed_buttons(),
                    &mut state,
                    (x + 20.0, y + 0.5),
                    vec![
                        vec![press(egui::Pos2::ZERO, true)],
                        vec![press(egui::Pos2::ZERO, false)],
                    ],
                );
                assert_eq!(edge, None, "{faction:?} {rect:?}");
                let frame = drive_with(
                    keyed_buttons(),
                    &mut state,
                    (x + 20.0, y + height - 1.5),
                    vec![
                        vec![press(egui::Pos2::ZERO, true)],
                        vec![press(egui::Pos2::ZERO, false)],
                    ],
                );
                assert!(frame.is_some(), "{faction:?} {rect:?}");
                if rect == CANCEL_RECT {
                    assert_eq!(frame, Some(answer));
                }
                // A press on a keyed pixel takes nothing, even released on
                // the bitmap.
                let mut state = open(faction);
                let pressed_off = press_then_release(
                    &mut state,
                    (x + 1.5, y + height - 0.5),
                    (x + 20.0, y + height - 1.5),
                );
                assert_eq!(pressed_off, None, "{faction:?} {rect:?}");
            }
        }
    }

    #[test]
    fn a_doubled_layout_hits_the_same_bitmap_pixels() {
        // The bitmap's pixel under the pointer is the window point less the
        // button's corner, over the layout's scale (FUN_006035f0 tests the
        // original's own 640 by 480 pixels).
        let doubled = CockpitLayout {
            canvas: CockpitViewport {
                x: 10.0,
                y: 20.0,
                width: 1280.0,
                height: 960.0,
            },
            galaxy: CockpitViewport {
                x: 120.0,
                y: 100.0,
                width: 970.0,
                height: 700.0,
            },
            scale: 2.0,
        };
        for faction in [MissionFaction::Alliance, MissionFaction::Empire] {
            let (x, y, _, height) = CANCEL_RECT;
            let clicked = |point| {
                drive_in(
                    doubled,
                    keyed_buttons(),
                    &mut open(faction),
                    point,
                    vec![
                        vec![press(egui::Pos2::ZERO, true)],
                        vec![press(egui::Pos2::ZERO, false)],
                    ],
                )
            };
            assert_eq!(
                clicked((x + 20.0, y + height - 1.5)),
                Some(MoveConfirmationAction::Cancel),
                "{faction:?}"
            );
            assert_eq!(clicked((x + 1.5, y + height - 0.5)), None, "{faction:?}");
        }
    }

    /// Press at `from`, move, and release at `to`, window points.
    fn press_then_release(
        state: &mut MoveConfirmationState,
        from: (f32, f32),
        to: (f32, f32),
    ) -> Option<MoveConfirmationAction> {
        let layout = layout();
        let point = |(x, y): (f32, f32)| window_rect(layout).min + egui::vec2(x, y);
        let (from, to) = (point(from), point(to));
        let frames = [
            vec![egui::Event::PointerMoved(from)],
            vec![egui::Event::PointerMoved(from)],
            vec![egui::Event::PointerMoved(from), press(from, true)],
            vec![egui::Event::PointerMoved(to)],
            vec![egui::Event::PointerMoved(to), press(to, false)],
        ];
        run_confirmation_frames(
            layout,
            keyed_buttons(),
            state,
            frames,
            draw_move_confirmation,
        )
    }

    fn add_test_fleet(world: &mut rebellion_core::world::GameWorld) -> FleetKey {
        world.fleets.insert(rebellion_core::world::Fleet {
            location: SystemKey::default(),
            capital_ships: Vec::new(),
            fighters: Vec::new(),
            characters: Vec::new(),
            is_alliance: true,
            bombarding: false,
        })
    }

    #[test]
    fn a_fleets_move_lists_its_transit_days_under_the_template() {
        // FUN_0049a350 loads 0x7057; FUN_0049a8b0 appends "\n", the name,
        // ":  " (DAT_006a8798) and the days for each object.
        let mut window = confirmation(MissionFaction::Alliance);
        assert_eq!(window.text(), "Transit time in days\nRed Fleet:  12");
        window.lines.push(("Blue Fleet".into(), 3));
        assert_eq!(
            window.text(),
            "Transit time in days\nRed Fleet:  12\nBlue Fleet:  3"
        );
    }

    #[test]
    fn each_side_has_its_own_picture() {
        // FUN_0049a350: +0x2e is 1018 for side 1 and 1019 otherwise.
        assert_eq!(confirmation(MissionFaction::Alliance).picture(), 1018);
        assert_eq!(confirmation(MissionFaction::Empire).picture(), 1019);
    }

    #[test]
    fn the_checkmark_submits_the_order_and_closes() {
        // FUN_0044f180: control 0x14 at (355, 244), 51 by 35;
        // FUN_0044f5e0 calls FUN_0041ce20(order, 1) and closes.
        let mut state = open(MissionFaction::Empire);

        let action = click(&mut state, (355.0 + 25.0, 244.0 + 17.0));

        assert_eq!(
            action,
            Some(MoveConfirmationAction::Confirm {
                fleets: vec![FleetKey::default()],
                members: Vec::new(),
                ships: None,
                units: Vec::new(),
                destination: SystemKey::default(),
                join: None,
            })
        );
        assert!(!state.is_open());
    }

    #[test]
    fn the_checkmark_carries_the_fleet_a_confirmed_move_joins() {
        // FUN_0044f5e0 resubmits the same order, so a move onto a fleet
        // still joins it (ghidra/notes/fleet-join-split.md).
        let mut world = rebellion_core::world::GameWorld::default();
        let target = add_test_fleet(&mut world);
        let mut state = MoveConfirmationState::default();
        state.open(MoveConfirmation {
            join: Some(target),
            ..confirmation(MissionFaction::Alliance)
        });

        let action = click(&mut state, (355.0 + 25.0, 244.0 + 17.0));

        assert!(matches!(
            action,
            Some(MoveConfirmationAction::Confirm { join: Some(fleet), .. }) if fleet == target
        ));
    }

    #[test]
    fn the_checkmark_carries_every_fleet_of_a_fleet_icons_team() {
        // FUN_0044f5e0 resubmits the same order, whose team is the icon's
        // fleets (FUN_00512700, kind 0x10); FUN_0053c2e0 lists one line each.
        let mut world = rebellion_core::world::GameWorld::default();
        let team = vec![add_test_fleet(&mut world), add_test_fleet(&mut world)];
        let mut state = MoveConfirmationState::default();
        state.open(MoveConfirmation {
            fleets: team.clone(),
            lines: vec![("Fleet 1".into(), 3), ("Fleet 2".into(), 4)],
            ..confirmation(MissionFaction::Alliance)
        });
        assert!(state
            .confirmation()
            .is_some_and(|window| window.text().ends_with("Fleet 1:  3\nFleet 2:  4")));

        let action = click(&mut state, (355.0 + 25.0, 244.0 + 17.0));

        assert!(matches!(
            action,
            Some(MoveConfirmationAction::Confirm { fleets, .. }) if fleets == team
        ));
    }

    #[test]
    fn the_checkmark_carries_the_ships_of_a_ships_confirmed_move() {
        // FUN_0044f5e0 resubmits the same order, whose team is the ships
        // (FUN_0053ca50: one 0x204 each); FUN_0053c2e0 lists one line each.
        let ships = FleetMover::Ships {
            fleet: FleetKey::default(),
            ships: vec![0, 2],
            roster: 7,
        };
        let mut state = MoveConfirmationState::default();
        state.open(MoveConfirmation {
            fleets: Vec::new(),
            ships: Some(ships.clone()),
            lines: vec![("Galleon 2".into(), 5), ("Death Star 1".into(), 0)],
            ..confirmation(MissionFaction::Empire)
        });
        assert!(state.confirmation().is_some_and(
            |window| window.text() == "Transit time in days\nGalleon 2:  5\nDeath Star 1:  0"
        ));

        let action = click(&mut state, (355.0 + 25.0, 244.0 + 17.0));

        assert!(matches!(
            action,
            Some(MoveConfirmationAction::Confirm { fleets, ships: Some(team), .. })
                if fleets.is_empty() && team == ships
        ));
    }

    #[test]
    fn the_checkmark_carries_the_units_of_a_units_confirmed_move_under_the_evacuation_warning() {
        // FUN_0049a350: a team whose first member is not a fleet and stands
        // in a blockaded system gets 0x7056 and picture 1031 (side 1) or
        // 1030; FUN_0044f5e0 resubmits the order with its units.
        let unit = UnitMover::Regiment(rebellion_core::ids::TroopKey::default());
        for (faction, picture) in [
            (MissionFaction::Alliance, 1031),
            (MissionFaction::Empire, 1030),
        ] {
            let mut state = MoveConfirmationState::default();
            state.open(MoveConfirmation {
                fleets: Vec::new(),
                units: vec![unit],
                blockaded: true,
                lines: vec![("Alliance Army Regiment".into(), 4)],
                ..confirmation(faction)
            });
            let window = state.confirmation().unwrap();
            assert_eq!(window.picture(), picture, "{faction:?}");
            assert!(window.text().starts_with("Units evacuating"), "{faction:?}");
            assert!(
                window.text().ends_with("\nAlliance Army Regiment:  4"),
                "{faction:?}"
            );

            let action = click(&mut state, (355.0 + 25.0, 244.0 + 17.0));

            assert!(
                matches!(
                    &action,
                    Some(MoveConfirmationAction::Confirm { fleets, units, .. })
                        if fleets.is_empty() && *units == [unit]
                ),
                "{faction:?}: {action:?}"
            );
        }
    }

    #[test]
    fn the_x_destroys_the_order_and_closes() {
        // Control 0x15 at (355, 281), 51 by 35.
        let mut state = open(MissionFaction::Alliance);

        let action = click(&mut state, (355.0 + 25.0, 281.0 + 17.0));

        assert_eq!(action, Some(MoveConfirmationAction::Cancel));
        assert!(!state.is_open());
    }

    #[test]
    fn a_click_beside_the_controls_answers_nothing() {
        // Just left of the checkmark, and in the 2-pixel gap below it.
        for point in [(354.0, 260.0), (380.0, 279.5)] {
            let mut state = open(MissionFaction::Alliance);

            assert_eq!(click(&mut state, point), None, "{point:?}");
            assert!(state.is_open());
        }
    }

    #[test]
    fn enter_confirms_and_escape_cancels() {
        // FUN_0044f640 maps 0xd to 0x14 and 0x1b to 0x15.
        let mut state = open(MissionFaction::Alliance);
        let action = drive(&mut state, (0.0, 0.0), vec![vec![key(egui::Key::Enter)]]);
        assert!(matches!(
            action,
            Some(MoveConfirmationAction::Confirm { .. })
        ));
        assert!(!state.is_open());

        let mut state = open(MissionFaction::Alliance);
        let action = drive(&mut state, (0.0, 0.0), vec![vec![key(egui::Key::Escape)]]);
        assert_eq!(action, Some(MoveConfirmationAction::Cancel));
        assert!(!state.is_open());
    }

    #[test]
    fn the_window_takes_the_pointer_only_over_itself() {
        let layout = layout();
        let rect = window_rect(layout);
        assert_eq!(
            rect.size(),
            egui::vec2(MOVE_CONFIRMATION_WIDTH, MOVE_CONFIRMATION_HEIGHT)
        );
        let state = open(MissionFaction::Alliance);
        assert!(state.contains_screen_point(layout, (rect.min.x, rect.min.y)));
        assert!(!state.contains_screen_point(layout, (rect.min.x - 1.0, rect.min.y)));
        assert!(!MoveConfirmationState::default()
            .contains_screen_point(layout, (rect.min.x, rect.min.y)));
    }

    fn scrap(faction: MissionFaction) -> ScrapConfirmationState {
        let mut state = ScrapConfirmationState::default();
        state.open(ScrapConfirmation {
            faction,
            order: ScrapOrder::Troop(rebellion_core::ids::TroopKey::default()),
            targets: vec![ScrapTarget::Troop(rebellion_core::ids::TroopKey::default())],
            names: vec!["Army Regiment".into(), "Mine".into()],
        });
        state
    }

    fn managed_open<S, P>(
        state: &mut S,
        prepared: P,
        spec: impl FnOnce(&P) -> rebellion_windowing::WindowSpec,
        commit: impl FnOnce(&mut S, P, &WindowTransition) -> Option<WindowId>,
    ) -> (WindowManager, WindowId) {
        let mut manager = WindowManager::new();
        let committed = manager
            .apply(WindowCommand::Open(spec(&prepared)))
            .expect("manager accepts confirmation");
        let WindowTransition::Opened { id } = committed.transition else {
            panic!("expected new confirmation identity");
        };
        assert_eq!(commit(state, prepared, &committed.transition), Some(id));
        (manager, id)
    }

    fn command_subject() -> SubjectId {
        SubjectId::new(crate::windowing_policy::SUBJECT_COMMAND_CONTEXT, 1)
    }

    fn managed_move_open(
        state: &mut MoveConfirmationState,
        confirmation: MoveConfirmation,
    ) -> (WindowManager, rebellion_windowing::WindowId) {
        let prepared = state
            .prepare_open(confirmation)
            .expect("valid Move confirmation context");
        managed_open(
            state,
            prepared,
            |prepared| prepared.spec(command_subject()),
            MoveConfirmationState::commit_open,
        )
    }

    fn scrap_confirmation(faction: MissionFaction) -> ScrapConfirmation {
        ScrapConfirmation {
            faction,
            order: ScrapOrder::Troop(rebellion_core::ids::TroopKey::default()),
            targets: vec![ScrapTarget::Troop(rebellion_core::ids::TroopKey::default())],
            names: vec!["Army Regiment".into()],
        }
    }

    fn managed_scrap_open(
        state: &mut ScrapConfirmationState,
        confirmation: ScrapConfirmation,
    ) -> (WindowManager, WindowId) {
        let prepared = state
            .prepare_open(confirmation)
            .expect("valid Scrap confirmation context");
        managed_open(
            state,
            prepared,
            |prepared| prepared.spec(command_subject()),
            ScrapConfirmationState::commit_open,
        )
    }

    fn unrelated_transition() -> (WindowTransition, WindowId) {
        let mut manager = WindowManager::new();
        let committed = manager
            .apply(WindowCommand::Open(
                crate::windowing_policy::singleton_spec(
                    WindowKind::Encyclopedia,
                    WindowOwner::StrategicRoot,
                ),
            ))
            .unwrap();
        let WindowTransition::Opened { id } = committed.transition else {
            panic!("expected unrelated identity");
        };
        (committed.transition, id)
    }

    fn assert_legacy_preserved<S, P>(
        state: &mut S,
        prepared: P,
        commit: fn(&mut S, P, &WindowTransition) -> Option<WindowId>,
        faction: impl FnOnce(&S) -> Option<MissionFaction>,
    ) -> WindowId {
        let (transition, unrelated_id) = unrelated_transition();
        assert_eq!(commit(state, prepared, &transition), None);
        assert_eq!(faction(state), Some(MissionFaction::Alliance));
        unrelated_id
    }

    fn assert_managed_routes<S>(
        state: &S,
        manager: &mut WindowManager,
        id: WindowId,
        hit: fn(&S, &RoutingSnapshot, CockpitLayout, (f32, f32)) -> Option<WindowId>,
        keyboard: fn(&S, &RoutingSnapshot) -> Option<WindowId>,
        reports: fn(&S, &RoutingSnapshot) -> Vec<ContentReport>,
        request_close: fn(&S) -> Option<WindowId>,
    ) -> WindowId {
        let rect = window_rect(layout());
        let center = (rect.center().x, rect.center().y);
        let frame = manager.begin_frame();
        assert_eq!(hit(state, &frame, layout(), center), Some(id));
        assert_eq!(
            hit(state, &frame, layout(), (rect.min.x - 1.0, rect.min.y)),
            None
        );
        assert_eq!(keyboard(state, &frame), Some(id));
        assert_eq!(
            reports(state, &frame),
            [ContentReport {
                window: id,
                status: ContentStatus::Ready,
            }]
        );
        assert_eq!(request_close(state), Some(id));

        let blocker = manager
            .apply(WindowCommand::Open(
                crate::windowing_policy::singleton_spec(
                    WindowKind::Encyclopedia,
                    WindowOwner::StrategicRoot,
                ),
            ))
            .unwrap();
        let WindowTransition::Opened { id: blocker } = blocker.transition else {
            panic!("expected blocker identity");
        };
        let blocked = manager.begin_frame();
        assert_eq!(hit(state, &blocked, layout(), center), None);
        assert_eq!(keyboard(state, &blocked), None);
        blocker
    }

    fn assert_replacement_rejections<S, P: Clone>(
        state: &mut S,
        replacement: P,
        id: WindowId,
        blocker: WindowId,
        unrelated: WindowId,
        commit: fn(&mut S, P, &WindowTransition) -> Option<WindowId>,
        set_id: fn(&mut S, WindowId),
    ) {
        assert_eq!(
            commit(
                state,
                replacement.clone(),
                &WindowTransition::Opened { id: blocker }
            ),
            None
        );
        assert_eq!(
            commit(
                state,
                replacement.clone(),
                &WindowTransition::Replaced {
                    old: blocker,
                    new: unrelated,
                },
            ),
            None
        );
        set_id(state, blocker);
        assert_eq!(
            commit(
                state,
                replacement,
                &WindowTransition::Replaced {
                    old: id,
                    new: unrelated,
                },
            ),
            None
        );
        set_id(state, id);
    }

    fn assert_confirm_guards<S, P: Clone, A: std::fmt::Debug + PartialEq, R>(
        state: &mut S,
        id: WindowId,
        wrong: WindowId,
        prepare: fn(&S) -> Option<P>,
        commit: fn(&mut S, P, &[WindowId]) -> Option<A>,
        mutate: impl FnOnce(&mut S) -> R,
        restore: impl FnOnce(&mut S, R),
        accepted: impl FnOnce(Option<A>) -> bool,
    ) {
        let prepared = prepare(state).unwrap();
        assert_eq!(commit(state, prepared.clone(), &[wrong]), None);
        let original = mutate(state);
        assert_eq!(commit(state, prepared.clone(), &[id]), None);
        restore(state, original);
        assert!(accepted(commit(state, prepared.clone(), &[id])));
        assert_eq!(commit(state, prepared, &[id]), None);
    }

    fn assert_removal<S>(
        state: &mut S,
        blocker: WindowId,
        id: WindowId,
        remove: fn(&mut S, &[WindowId]),
        window_id: fn(&S) -> Option<WindowId>,
        is_open: fn(&S) -> bool,
    ) {
        remove(state, &[blocker]);
        assert_eq!(window_id(state), Some(id));
        remove(state, &[id]);
        assert!(!is_open(state));
    }

    #[test]
    fn move_confirmation_managed_adapter_enforces_exact_content_and_routes() {
        let mut invalid_both = confirmation(MissionFaction::Alliance);
        invalid_both.members = vec![MissionMember::Character(
            rebellion_core::ids::CharacterKey::default(),
        )];
        invalid_both.lines.push(("Leia".into(), 2));
        assert!(MoveConfirmationState::default()
            .prepare_open(invalid_both)
            .is_none());

        let valid_member = MoveConfirmation {
            fleets: Vec::new(),
            members: vec![MissionMember::Character(
                rebellion_core::ids::CharacterKey::default(),
            )],
            lines: vec![("Leia".into(), 2)],
            ..confirmation(MissionFaction::Alliance)
        };
        assert!(MoveConfirmationState::default()
            .prepare_open(valid_member)
            .is_some());

        // Source: FUN_00487740 -> FUN_00487cc0 -> FUN_0049a350. Confirmed
        // ship teams and blockaded regiment/squadron moves use the same
        // confirmation family and keep one display line per mover.
        for faction in [MissionFaction::Alliance, MissionFaction::Empire] {
            let valid_ships = MoveConfirmation {
                faction,
                fleets: Vec::new(),
                members: Vec::new(),
                ships: Some(FleetMover::Ships {
                    fleet: FleetKey::default(),
                    ships: vec![0, 1],
                    roster: 1,
                }),
                units: Vec::new(),
                destination: SystemKey::default(),
                join: None,
                blockaded: false,
                lines: vec![("Ship 1".into(), 2), ("Ship 2".into(), 3)],
            };
            assert!(MoveConfirmationState::default()
                .prepare_open(valid_ships)
                .is_some());
            let valid_unit = MoveConfirmation {
                faction,
                fleets: Vec::new(),
                members: Vec::new(),
                ships: None,
                units: vec![UnitMover::Regiment(rebellion_core::ids::TroopKey::default())],
                destination: SystemKey::default(),
                join: None,
                blockaded: true,
                lines: vec![("Regiment".into(), 1)],
            };
            assert!(MoveConfirmationState::default()
                .prepare_open(valid_unit)
                .is_some());
        }

        let mut legacy = open(MissionFaction::Alliance);
        let inert = legacy
            .prepare_open(confirmation(MissionFaction::Empire))
            .unwrap();
        let unrelated_id = assert_legacy_preserved(
            &mut legacy,
            inert,
            MoveConfirmationState::commit_open,
            |state| state.confirmation().map(|value| value.faction),
        );

        let mut state = MoveConfirmationState::default();
        let (mut manager, id) =
            managed_move_open(&mut state, confirmation(MissionFaction::Alliance));
        let blocker = assert_managed_routes(
            &state,
            &mut manager,
            id,
            MoveConfirmationState::hit_window,
            MoveConfirmationState::keyboard_owner,
            MoveConfirmationState::content_reports,
            MoveConfirmationState::request_close,
        );

        let replacement = state
            .prepare_open(confirmation(MissionFaction::Empire))
            .unwrap();
        assert_eq!(replacement.replacing(), Some(id));
        assert_replacement_rejections(
            &mut state,
            replacement,
            id,
            blocker,
            unrelated_id,
            MoveConfirmationState::commit_open,
            |state, id| state.window_id = Some(id),
        );
        assert_confirm_guards(
            &mut state,
            id,
            blocker,
            MoveConfirmationState::prepare_confirm,
            MoveConfirmationState::commit_confirm,
            |state| {
                let original = state.window.clone();
                state.window.as_mut().unwrap().join = Some(FleetKey::default());
                original
            },
            |state, original| state.window = original,
            |result| matches!(result, Some(MoveConfirmationAction::Confirm { .. })),
        );

        let mut removable = MoveConfirmationState::default();
        let (_, removable_id) =
            managed_move_open(&mut removable, confirmation(MissionFaction::Alliance));
        assert_removal(
            &mut removable,
            blocker,
            removable_id,
            MoveConfirmationState::remove_content,
            MoveConfirmationState::window_id,
            MoveConfirmationState::is_open,
        );
    }

    #[test]
    fn managed_move_draw_reports_without_consuming_content() {
        let mut confirmed = MoveConfirmationState::default();
        let (_, id) = managed_move_open(&mut confirmed, confirmation(MissionFaction::Alliance));
        assert!(matches!(
            click(&mut confirmed, (355.0 + 25.0, 244.0 + 17.0)),
            Some(MoveConfirmationAction::Confirm { .. })
        ));
        assert_eq!(confirmed.window_id(), Some(id));
        assert!(confirmed.is_open());

        let mut cancelled = MoveConfirmationState::default();
        let (_, id) = managed_move_open(&mut cancelled, confirmation(MissionFaction::Alliance));
        assert_eq!(
            click(&mut cancelled, (355.0 + 25.0, 281.0 + 17.0)),
            Some(MoveConfirmationAction::Cancel)
        );
        assert_eq!(cancelled.window_id(), Some(id));
        assert!(cancelled.is_open());
    }

    #[test]
    fn scrap_confirmation_managed_adapter_enforces_exact_content_and_routes() {
        let mut no_targets = scrap_confirmation(MissionFaction::Alliance);
        no_targets.targets.clear();
        assert!(ScrapConfirmationState::default()
            .prepare_open(no_targets)
            .is_none());
        let mut no_names = scrap_confirmation(MissionFaction::Alliance);
        no_names.names.clear();
        assert!(ScrapConfirmationState::default()
            .prepare_open(no_names)
            .is_none());

        let mut legacy = scrap(MissionFaction::Alliance);
        let inert = legacy
            .prepare_open(scrap_confirmation(MissionFaction::Empire))
            .unwrap();
        let unrelated_id = assert_legacy_preserved(
            &mut legacy,
            inert,
            ScrapConfirmationState::commit_open,
            |state| state.confirmation().map(|value| value.faction),
        );

        let mut state = ScrapConfirmationState::default();
        let (mut manager, id) =
            managed_scrap_open(&mut state, scrap_confirmation(MissionFaction::Alliance));
        assert_eq!(state.window_id(), Some(id));
        let blocker = assert_managed_routes(
            &state,
            &mut manager,
            id,
            ScrapConfirmationState::hit_window,
            ScrapConfirmationState::keyboard_owner,
            ScrapConfirmationState::content_reports,
            ScrapConfirmationState::request_close,
        );

        let replacement = state
            .prepare_open(scrap_confirmation(MissionFaction::Empire))
            .unwrap();
        assert_eq!(replacement.replacing(), Some(id));
        assert_replacement_rejections(
            &mut state,
            replacement.clone(),
            id,
            blocker,
            unrelated_id,
            ScrapConfirmationState::commit_open,
            |state, id| state.window_id = Some(id),
        );
        let previous_id = id;
        assert_eq!(
            state.commit_open(
                replacement,
                &WindowTransition::Replaced {
                    old: id,
                    new: blocker,
                }
            ),
            Some(blocker)
        );
        assert_eq!(state.window_id(), Some(blocker));
        assert_eq!(
            state.confirmation().map(|value| value.faction),
            Some(MissionFaction::Empire)
        );
        let id = blocker;
        assert_confirm_guards(
            &mut state,
            id,
            previous_id,
            ScrapConfirmationState::prepare_confirm,
            ScrapConfirmationState::commit_confirm,
            |state| {
                let original = state.window.clone();
                state.window.as_mut().unwrap().faction = MissionFaction::Alliance;
                original
            },
            |state, original| state.window = original,
            |result| matches!(result, Some(ScrapConfirmationAction::Confirm { .. })),
        );

        let mut removable = ScrapConfirmationState::default();
        let (_, removable_id) =
            managed_scrap_open(&mut removable, scrap_confirmation(MissionFaction::Alliance));
        assert_removal(
            &mut removable,
            blocker,
            removable_id,
            ScrapConfirmationState::remove_content,
            ScrapConfirmationState::window_id,
            ScrapConfirmationState::is_open,
        );
    }

    #[test]
    fn move_confirmation_windowing_failed_context_preserves_old() {
        let mut state = MoveConfirmationState::default();
        let original = confirmation(MissionFaction::Alliance);
        let (mut manager, id) = managed_move_open(&mut state, original.clone());

        let invalid = MoveConfirmation {
            fleets: Vec::new(),
            members: Vec::new(),
            lines: Vec::new(),
            ..confirmation(MissionFaction::Empire)
        };
        assert!(state.prepare_open(invalid).is_none());
        assert_eq!(state.confirmation(), Some(&original));
        assert_eq!(state.window_id(), Some(id));

        let replacement = state
            .prepare_open(confirmation(MissionFaction::Empire))
            .unwrap();
        assert_eq!(replacement.replacing(), Some(id));
        let unrelated = manager
            .apply(WindowCommand::Open(
                crate::windowing_policy::singleton_spec(
                    WindowKind::Encyclopedia,
                    WindowOwner::StrategicRoot,
                ),
            ))
            .unwrap();
        let WindowTransition::Opened { id: unrelated } = unrelated.transition else {
            panic!("expected unrelated identity");
        };
        manager.apply(WindowCommand::Close(unrelated)).unwrap();
        assert!(manager
            .apply(WindowCommand::Replace {
                old: unrelated,
                new: replacement.spec(SubjectId::new(
                    crate::windowing_policy::SUBJECT_COMMAND_CONTEXT,
                    2,
                )),
            })
            .is_err());
        assert_eq!(state.confirmation(), Some(&original));
        assert_eq!(state.window_id(), Some(id));

        let new = crate::test_support::commit_prepared_replacement!(
            &mut manager,
            &mut state,
            id,
            replacement,
            "Move"
        );
        assert_eq!(state.window_id(), Some(new));
        assert_eq!(
            state.confirmation().map(|value| value.faction),
            Some(MissionFaction::Empire)
        );
    }

    #[test]
    fn move_confirmation_windowing_confirm_is_once() {
        let mut state = MoveConfirmationState::default();
        let (mut manager, id) =
            managed_move_open(&mut state, confirmation(MissionFaction::Alliance));
        let prepared = state.prepare_confirm().unwrap();

        assert_eq!(state.commit_confirm(prepared.clone(), &[]), None);
        assert!(state.is_open());

        let committed = manager
            .apply(WindowCommand::Close(id))
            .expect("exact Move close commits");
        let WindowTransition::Closed { closed, .. } = committed.transition else {
            panic!("expected Move close receipt");
        };
        assert!(matches!(
            state.commit_confirm(prepared.clone(), &closed),
            Some(MoveConfirmationAction::Confirm { .. })
        ));
        assert_eq!(state.commit_confirm(prepared, &closed), None);
        assert!(!state.is_open());
    }

    #[test]
    fn scrap_confirmation_uses_explicit_port_modal_policy() {
        let mut manager = WindowManager::new();
        let owner = match manager
            .apply(WindowCommand::Open(
                crate::windowing_policy::contextual_spec(
                    WindowKind::Fleet,
                    SubjectId::new(1, 1),
                    WindowOwner::StrategicRoot,
                ),
            ))
            .unwrap()
            .transition
        {
            WindowTransition::Opened { id } => id,
            transition => panic!("expected Fleet owner, got {transition:?}"),
        };
        manager.apply(WindowCommand::CapturePointer(owner)).unwrap();

        let state = ScrapConfirmationState::default();
        let prepared = state
            .prepare_open(crate::test_support::scrap_confirmation())
            .unwrap();
        let spec = prepared.spec(SubjectId::new(
            crate::windowing_policy::SUBJECT_COMMAND_CONTEXT,
            2,
        ));
        assert_eq!(spec.key.kind, WindowKind::ScrapConfirmation);
        assert_eq!(spec.policy.input, InputPolicy::StrategicModal);
        let opened = manager.apply(WindowCommand::Open(spec)).unwrap();
        assert!(opened
            .events
            .contains(&rebellion_windowing::WindowEvent::CaptureCancelled {
                window: owner,
                reason: CaptureCancelReason::ModalOpened,
            }));
    }

    #[test]
    fn a_scrap_lists_each_unit_under_the_question() {
        // FUN_0049a350 loads 0x7050; FUN_0049a880 appends 0x7054, "\n" and
        // the name, for each object.
        let state = scrap(MissionFaction::Alliance);
        assert_eq!(
            state.confirmation().unwrap().text(),
            "Are you sure you want to scrap the following units?\nArmy Regiment\nMine"
        );
    }

    #[test]
    fn each_side_has_its_own_scrap_picture() {
        // FUN_0049a350 case 0x200: +0x2e is 0x408 for side 1, else 0x409.
        assert_eq!(
            scrap(MissionFaction::Alliance)
                .confirmation()
                .unwrap()
                .picture(),
            1032
        );
        assert_eq!(
            scrap(MissionFaction::Empire)
                .confirmation()
                .unwrap()
                .picture(),
            1033
        );
    }

    fn drive_scrap(
        state: &mut ScrapConfirmationState,
        point: (f32, f32),
        frames: Vec<Vec<egui::Event>>,
    ) -> Option<ScrapConfirmationAction> {
        drive_confirmation(
            layout(),
            BmpCache::new(),
            state,
            point,
            frames,
            draw_scrap_confirmation,
        )
    }

    #[test]
    fn the_scrap_checkmark_scraps_the_units_and_the_x_does_not() {
        // FUN_0044f5e0: control 0x14 submits the order, 0x15 destroys it.
        let at = egui::Pos2::ZERO;
        let mut state = scrap(MissionFaction::Empire);
        let action = drive_scrap(
            &mut state,
            (355.0 + 25.0, 244.0 + 17.0),
            vec![vec![press(at, true)], vec![press(at, false)]],
        );
        assert_eq!(
            action,
            Some(ScrapConfirmationAction::Confirm {
                faction: MissionFaction::Empire,
                order: ScrapOrder::Troop(rebellion_core::ids::TroopKey::default()),
                targets: vec![ScrapTarget::Troop(rebellion_core::ids::TroopKey::default())],
            })
        );
        assert!(!state.is_open());

        let mut state = scrap(MissionFaction::Empire);
        let action = drive_scrap(
            &mut state,
            (355.0 + 25.0, 281.0 + 17.0),
            vec![vec![press(at, true)], vec![press(at, false)]],
        );
        assert_eq!(action, Some(ScrapConfirmationAction::Cancel));
        assert!(!state.is_open());

        let mut state = scrap(MissionFaction::Empire);
        let action = drive_scrap(&mut state, (0.0, 0.0), vec![vec![key(egui::Key::Escape)]]);
        assert_eq!(action, Some(ScrapConfirmationAction::Cancel));
    }

    /// A shipyard at a system building `count` units of a class named
    /// "Death Star" for `faction`.
    /// port: test fixture values, not rules.
    fn building(
        faction: MissionFaction,
        count: u32,
    ) -> (
        rebellion_core::world::GameWorld,
        rebellion_core::manufacturing::ManufacturingState,
        SystemKey,
    ) {
        use rebellion_core::manufacturing::ManufacturingState;
        use rebellion_core::world::{GameWorld, System};
        let mut world = GameWorld::default();
        let system = world.systems.insert(System::default());
        let is_alliance = faction == MissionFaction::Alliance;
        let mut manufacturing = ManufacturingState::new();
        rebellion_core::test_support::queue_capital_ship_build(
            &mut world,
            &mut manufacturing,
            system,
            "Death Star",
            is_alliance,
            !is_alliance,
            count,
        );
        (world, manufacturing, system)
    }

    fn stop_state(
        faction: MissionFaction,
        count: u32,
    ) -> (
        StopConfirmationState,
        rebellion_core::manufacturing::ManufacturingState,
        SystemKey,
    ) {
        let (world, manufacturing, system) = building(faction, count);
        let mut state = StopConfirmationState::default();
        state.open(
            StopConfirmation::for_band(
                &world,
                &manufacturing,
                faction,
                system,
                ProductionArea::Shipyard,
            )
            .expect("the band builds"),
        );
        (state, manufacturing, system)
    }

    fn drive_stop(
        state: &mut StopConfirmationState,
        point: (f32, f32),
        frames: Vec<Vec<egui::Event>>,
    ) -> Option<StopConfirmationAction> {
        drive_confirmation(
            layout(),
            BmpCache::new(),
            state,
            point,
            frames,
            draw_stop_confirmation,
        )
    }

    /// The app's answer to the window: the checkmark stops the band
    /// (the app's `commit_stop_confirmation`), the X leaves it.
    fn answer(
        action: Option<StopConfirmationAction>,
        manufacturing: &mut rebellion_core::manufacturing::ManufacturingState,
    ) {
        if let Some(StopConfirmationAction::Confirm { system, area }) = action {
            manufacturing.stop(system, area);
        }
    }

    #[test]
    fn a_stop_lists_the_bands_units_and_class_under_the_question() {
        // FUN_0049a350 case 0x213 loads 0x7053, then 0x7055 ("\n", item 6,
        // two spaces, item 5) for each band: the units left and the class.
        for faction in [MissionFaction::Alliance, MissionFaction::Empire] {
            for count in [1, 3] {
                let (state, _, _) = stop_state(faction, count);
                assert_eq!(
                    state.confirmation().unwrap().text(),
                    format!(
                        "Are you sure you want to stop construction of the following?\n{count}  Death Star"
                    ),
                    "{faction:?}"
                );
            }
        }
    }

    /// Each line of the field's text at `scale`: its characters, its ink
    /// width (first drawn pixel to last), and its baseline below the
    /// field's top, in window pixels.
    fn field_lines(text: &str, scale: f32) -> Vec<(String, f32, f32)> {
        let ctx = egui::Context::default();
        crate::theme::load_fonts(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |_| {});
        let galley = ctx.fonts(|fonts| fonts.layout_job(text_job(text.to_owned(), scale)));
        let origin = text_origin(&galley, egui::Pos2::ZERO, scale);
        galley
            .rows
            .iter()
            .filter(|row| !row.glyphs.is_empty())
            .map(|row| {
                let chars: String = row.glyphs.iter().map(|glyph| glyph.chr).collect();
                let (left, right) = row
                    .glyphs
                    .iter()
                    .filter(|glyph| !glyph.chr.is_whitespace())
                    .map(|glyph| {
                        let left = glyph.pos.x + glyph.uv_rect.offset.x;
                        (left, left + glyph.uv_rect.size.x)
                    })
                    .fold((f32::MAX, f32::MIN), |(min, max), (left, right)| {
                        (min.min(left), max.max(right))
                    });
                let width = right - left;
                (
                    chars.trim_end().to_owned(),
                    width / scale,
                    (origin.y + row.glyphs[0].pos.y) / scale,
                )
            })
            .collect()
    }

    /// Each line's ink width lies within 5 pixels of the original's. egui
    /// lays out Liberation Sans's outline advances, which differ from
    /// Arial's hinted ones by a few pixels a line, more or less by string.
    fn assert_widths(lines: &[(String, f32, f32)], originals: &[f32], scale: f32) {
        assert_eq!(lines.len(), originals.len(), "scale {scale}");
        for (line, original) in lines.iter().zip(originals) {
            assert!(
                (line.1 - original).abs() <= 5.0,
                "{:?} is {} wide, the original {original} (scale {scale})",
                line.0,
                line.1
            );
        }
    }

    #[test]
    fn the_field_wraps_and_spaces_the_stop_question_as_the_original_does() {
        // FUN_00420550(field, 4): font 4 is FUN_0060eed0's default case,
        // Arial with a 16-pixel cell (ascent 13) at weight 400. FUN_0041fd00
        // draws with DrawTextA format 0x2410 (DT_WORDBREAK) across the
        // field's 322 pixels. hyp (measured): the original's Stop window
        // (Empire, day 136, one Death Star) breaks the question after "the";
        // its lines' ink is 285, 55 and 76 pixels wide (Arial's hinted
        // advances at a 13-pixel em) and 16 apart, the first baseline 13
        // below the field's top.
        let text = format!("{STOP_QUESTION}\n1  Death Star");
        for scale in [1.0, 5.0 / 3.0, 2.0] {
            let lines = field_lines(&text, scale);
            let words: Vec<&str> = lines.iter().map(|line| line.0.as_str()).collect();
            assert_eq!(
                words,
                [
                    "Are you sure you want to stop construction of the",
                    "following?",
                    "1  Death Star"
                ],
                "scale {scale}"
            );
            assert_widths(&lines, &[285.0, 55.0, 76.0], scale);
            // Within half a pixel: egui snaps each row to whole screen pixels.
            for (index, line) in lines.iter().enumerate() {
                let baseline = 13.0 + 16.0 * index as f32;
                assert!((line.2 - baseline).abs() <= 0.5, "{line:?} (scale {scale})");
            }
        }
    }

    #[test]
    fn the_field_keeps_the_scrap_question_and_a_moves_lines_whole() {
        // hyp (measured): from empire-day2-start.sav at day 3, the original
        // keeps "Are you sure you want to scrap the following units?" on one
        // line, its ink 295 pixels wide in the 322-pixel field; Fleet 1's
        // Confirmed Move to Yaga Minor reads 114 and 57 pixels.
        for scale in [1.0, 5.0 / 3.0, 2.0] {
            for (text, originals) in [
                (
                    format!("{SCRAP_QUESTION}\nCarrack Light Cruiser 2"),
                    vec![295.0, 131.0],
                ),
                (format!("{TRANSIT_TIME}\nFleet 1:  6"), vec![114.0, 57.0]),
            ] {
                let lines = field_lines(&text, scale);
                let words: Vec<&str> = lines.iter().map(|line| line.0.as_str()).collect();
                assert_eq!(words, text.split('\n').collect::<Vec<_>>(), "scale {scale}");
                assert_widths(&lines, &originals, scale);
            }
        }
    }

    #[test]
    fn each_side_has_its_own_stop_picture() {
        // FUN_0049a350 case 0x213: +0x2e is 0x40e for side 1, else 0x40f.
        assert_eq!(
            stop_state(MissionFaction::Alliance, 1)
                .0
                .confirmation()
                .unwrap()
                .picture(),
            1038
        );
        assert_eq!(
            stop_state(MissionFaction::Empire, 1)
                .0
                .confirmation()
                .unwrap()
                .picture(),
            1039
        );
    }

    #[test]
    fn the_stop_window_is_modal_and_takes_the_pointer_only_over_itself() {
        let layout = layout();
        let rect = window_rect(layout);
        let closed = StopConfirmationState::default();
        assert!(!closed.is_open());
        assert!(!closed.contains_screen_point(layout, (rect.min.x, rect.min.y)));
        let (state, _, _) = stop_state(MissionFaction::Empire, 1);
        assert!(state.is_open());
        assert!(state.contains_screen_point(layout, (rect.min.x, rect.min.y)));
        assert!(!state.contains_screen_point(layout, (rect.min.x - 1.0, rect.min.y)));
    }

    #[test]
    fn an_idle_band_opens_no_stop() {
        let (world, _, system) = building(MissionFaction::Empire, 1);
        let idle = rebellion_core::manufacturing::ManufacturingState::new();
        assert_eq!(
            StopConfirmation::for_band(
                &world,
                &idle,
                MissionFaction::Empire,
                system,
                ProductionArea::Shipyard
            ),
            None
        );
    }

    #[test]
    fn the_stop_checkmark_stops_the_band_and_the_x_leaves_it() {
        // FUN_0044f5e0: control 0x14 (355, 244) submits the order, which
        // stops the band; control 0x15 (355, 281) or Escape destroys it.
        let at = egui::Pos2::ZERO;
        let clicks = || vec![vec![press(at, true)], vec![press(at, false)]];
        for faction in [MissionFaction::Alliance, MissionFaction::Empire] {
            let (mut state, mut manufacturing, system) = stop_state(faction, 2);
            let action = drive_stop(&mut state, (355.0 + 25.0, 244.0 + 17.0), clicks());
            assert_eq!(
                action,
                Some(StopConfirmationAction::Confirm {
                    system,
                    area: ProductionArea::Shipyard
                }),
                "{faction:?}"
            );
            answer(action, &mut manufacturing);
            assert!(manufacturing
                .queue(system, ProductionArea::Shipyard)
                .is_none());
            assert!(!state.is_open());

            for cancel in [
                drive_stop(
                    &mut stop_state(faction, 2).0,
                    (355.0 + 25.0, 281.0 + 17.0),
                    clicks(),
                ),
                drive_stop(
                    &mut stop_state(faction, 2).0,
                    (0.0, 0.0),
                    vec![vec![key(egui::Key::Escape)]],
                ),
            ] {
                let (_, mut manufacturing, system) = stop_state(faction, 2);
                assert_eq!(cancel, Some(StopConfirmationAction::Cancel), "{faction:?}");
                answer(cancel, &mut manufacturing);
                assert_eq!(
                    manufacturing
                        .queue(system, ProductionArea::Shipyard)
                        .map(rebellion_core::manufacturing::ProductionQueue::len),
                    Some(2),
                    "{faction:?}"
                );
            }
        }
    }
}
