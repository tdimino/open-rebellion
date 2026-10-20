//! The original move confirmation window (`FUN_0044f060` builds it,
//! `FUN_0044f180` lays it out, `FUN_0044f5e0` handles its controls;
//! `ghidra/notes/move-order.md`, "The confirmation window").
//!
//! A 424 by 331 window that shows the order's transit time in days. The
//! checkmark submits the order without asking again; the X destroys it.
//! Enter and Escape press them (`FUN_0044f640`).

use egui_macroquad::egui;
use rebellion_core::ids::{FleetKey, SystemKey};
use rebellion_core::missions::{MissionFaction, MissionMember};
use rebellion_core::scrap::{ScrapOrder, ScrapTarget};
use rebellion_windowing::{
    ContentReport, ContentStatus, RoutingSnapshot, SubjectId, WindowId, WindowKind, WindowOwner,
    WindowSpec, WindowTransition,
};

use crate::bmp_cache::BmpCache;
use crate::cockpit::CockpitLayout;
use crate::mission_dialog::{button, galaxy_centered_rect, paint};
use crate::system_window::{logical_rect, rect_contains};

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

/// TEXTSTRA `RT_RCDATA` 0x7057.
const TRANSIT_TIME: &str = "Transit time in days";
/// TEXTSTRA `RT_RCDATA` 0x7056.
const BLOCKADE_EVACUATION: &str = "Units evacuating from worlds under blockade risk being destroyed by blockading vessels.  Are you sure you want to proceed with the evacuation?\nTransit time in days";

/// A move waiting for the player's answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveConfirmation {
    pub faction: MissionFaction,
    /// The order's team: one fleet, or a fleet icon's fleets.
    pub fleets: Vec<FleetKey>,
    /// The order's characters and special forces (its `0x241` teams).
    pub members: Vec<MissionMember>,
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
            text.push_str(&format!("\n{name}:  {days}"));
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
#[derive(Debug, Clone)]
pub struct PreparedMoveConfirmation {
    confirmation: MoveConfirmation,
    replacing: Option<WindowId>,
}

impl PreparedMoveConfirmation {
    #[must_use]
    pub fn spec(&self, subject: SubjectId) -> WindowSpec {
        crate::windowing_policy::contextual_spec(
            WindowKind::MoveConfirmation,
            subject,
            WindowOwner::StrategicRoot,
        )
    }

    #[must_use]
    pub const fn replacing(&self) -> Option<WindowId> {
        self.replacing
    }
}

/// Exact Move candidate paired with one manager close. It remains inert until
/// the matching teardown receipt releases it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedMoveConfirmationConfirm {
    window: WindowId,
    action: MoveConfirmationAction,
}

impl PreparedMoveConfirmationConfirm {
    #[must_use]
    pub const fn window_id(&self) -> WindowId {
        self.window
    }
}

impl MoveConfirmationState {
    pub fn open(&mut self, confirmation: MoveConfirmation) {
        self.window = Some(confirmation);
        self.window_id = None;
    }

    /// Prepares one valid logical Move context without disturbing any
    /// published confirmation. Exactly one mover family must be present and
    /// every mover must retain its transit line.
    #[must_use]
    pub fn prepare_open(&self, confirmation: MoveConfirmation) -> Option<PreparedMoveConfirmation> {
        let movers = confirmation.fleets.len() + confirmation.members.len();
        if movers == 0
            || (!confirmation.fleets.is_empty() && !confirmation.members.is_empty())
            || confirmation.lines.len() != movers
        {
            return None;
        }
        Some(PreparedMoveConfirmation {
            confirmation,
            replacing: self.window_id,
        })
    }

    /// Publishes prepared content only for the exact manager open or
    /// replacement receipt. A duplicate receipt preserves current content.
    pub fn commit_open(
        &mut self,
        prepared: PreparedMoveConfirmation,
        transition: &WindowTransition,
    ) -> Option<WindowId> {
        match *transition {
            WindowTransition::Opened { id }
                if self.window.is_none()
                    && self.window_id.is_none()
                    && prepared.replacing.is_none() =>
            {
                self.window = Some(prepared.confirmation);
                self.window_id = Some(id);
                Some(id)
            }
            WindowTransition::Replaced { old, new }
                if self.window_id == Some(old) && prepared.replacing == Some(old) =>
            {
                self.window = Some(prepared.confirmation);
                self.window_id = Some(new);
                Some(new)
            }
            _ => None,
        }
    }

    #[must_use]
    pub fn is_open(&self) -> bool {
        self.window.is_some()
    }

    /// The open window's order, if one waits for an answer.
    #[must_use]
    pub fn confirmation(&self) -> Option<&MoveConfirmation> {
        self.window.as_ref()
    }

    /// Exact manager identity owning the published Move content.
    #[must_use]
    pub const fn window_id(&self) -> Option<WindowId> {
        self.window_id
    }

    /// Manager-owned hit for event-local pointer routing.
    #[must_use]
    pub fn hit_window(
        &self,
        frame: &RoutingSnapshot,
        layout: CockpitLayout,
        point: (f32, f32),
    ) -> Option<WindowId> {
        let id = self.window_id?;
        (frame
            .key(id)
            .is_some_and(|key| key.kind == WindowKind::MoveConfirmation)
            && frame.allows_input(id)
            && rect_contains(window_rect(layout), egui::pos2(point.0, point.1)))
        .then_some(id)
    }

    /// Exact manager identity holding this frame's keyboard lease.
    #[must_use]
    pub fn keyboard_owner(&self, frame: &RoutingSnapshot) -> Option<WindowId> {
        let id = self.window_id?;
        matches!(
            frame.keyboard_route().target,
            rebellion_windowing::RouteTarget::Window(target) if target.window == id
        )
        .then_some(id)
    }

    /// Explicit content state for fail-closed manager reconciliation.
    #[must_use]
    pub fn content_reports(&self, frame: &RoutingSnapshot) -> Vec<ContentReport> {
        frame
            .paint_order()
            .iter()
            .copied()
            .filter(|id| {
                frame
                    .key(*id)
                    .is_some_and(|key| key.kind == WindowKind::MoveConfirmation)
            })
            .map(|id| ContentReport {
                window: id,
                status: if self.window_id == Some(id) {
                    ContentStatus::Ready
                } else {
                    ContentStatus::Unpaintable
                },
            })
            .collect()
    }

    #[must_use]
    pub const fn request_close(&self) -> Option<WindowId> {
        self.window_id
    }

    /// Releases manager-owned content only from its exact teardown receipt.
    pub fn remove_content(&mut self, removed: &[WindowId]) {
        if self.window_id.is_some_and(|id| removed.contains(&id)) {
            self.window = None;
            self.window_id = None;
        }
    }

    /// Snapshots the exact current Move order without performing gameplay.
    #[must_use]
    pub fn prepare_confirm(&self) -> Option<PreparedMoveConfirmationConfirm> {
        Some(PreparedMoveConfirmationConfirm {
            window: self.window_id?,
            action: self.window.as_ref()?.confirm_action(),
        })
    }

    /// Consumes one Move confirmation only after the exact manager close and
    /// while its published payload still matches the prepared candidate.
    pub fn commit_confirm(
        &mut self,
        prepared: PreparedMoveConfirmationConfirm,
        removed: &[WindowId],
    ) -> Option<MoveConfirmationAction> {
        if !removed.contains(&prepared.window)
            || self.window_id != Some(prepared.window)
            || self.window.as_ref()?.confirm_action() != prepared.action
        {
            return None;
        }
        self.window = None;
        self.window_id = None;
        Some(prepared.action)
    }

    /// Whether `point` falls on the open window, so the galaxy map under it
    /// takes no input.
    #[must_use]
    pub fn contains_screen_point(&self, layout: CockpitLayout, point: (f32, f32)) -> bool {
        self.window.is_some() && rect_contains(window_rect(layout), egui::pos2(point.0, point.1))
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

            // hyp: the field's font 4 (FUN_00420550) is not mapped; the
            // mission dialog's text size stands in. Its colour +0xb4 is
            // white (0x2ffffff). port: the field does not scroll.
            let text_box = at(TEXT_BOX);
            let galley = painter.layout(
                text,
                egui::FontId::proportional((11.0 * scale).max(7.0)),
                egui::Color32::WHITE,
                text_box.width(),
            );
            painter
                .with_clip_rect(text_box)
                .galley(text_box.min, galley, egui::Color32::WHITE);

            if button(
                ui,
                cache,
                at(CONFIRM_RECT),
                "confirm",
                CONFIRM,
                false,
                scale,
            ) {
                answer = Some(true);
            }
            if button(ui, cache, at(CANCEL_RECT), "cancel", CANCEL, false, scale) {
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
#[derive(Debug, Clone)]
pub struct PreparedScrapConfirmation {
    confirmation: ScrapConfirmation,
    replacing: Option<WindowId>,
}

impl PreparedScrapConfirmation {
    #[must_use]
    pub fn spec(&self, subject: SubjectId) -> WindowSpec {
        crate::windowing_policy::contextual_spec(
            WindowKind::ScrapConfirmation,
            subject,
            WindowOwner::StrategicRoot,
        )
    }

    #[must_use]
    pub const fn replacing(&self) -> Option<WindowId> {
        self.replacing
    }
}

/// Exact Scrap candidate paired with one manager close.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedScrapConfirmationConfirm {
    window: WindowId,
    action: ScrapConfirmationAction,
}

impl PreparedScrapConfirmationConfirm {
    #[must_use]
    pub const fn window_id(&self) -> WindowId {
        self.window
    }
}

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
    pub fn open(&mut self, confirmation: ScrapConfirmation) {
        self.window = Some(confirmation);
        self.window_id = None;
    }

    #[must_use]
    pub fn prepare_open(
        &self,
        confirmation: ScrapConfirmation,
    ) -> Option<PreparedScrapConfirmation> {
        if confirmation.targets.is_empty() || confirmation.names.is_empty() {
            return None;
        }
        Some(PreparedScrapConfirmation {
            confirmation,
            replacing: self.window_id,
        })
    }

    pub fn commit_open(
        &mut self,
        prepared: PreparedScrapConfirmation,
        transition: &WindowTransition,
    ) -> Option<WindowId> {
        match *transition {
            WindowTransition::Opened { id }
                if self.window.is_none()
                    && self.window_id.is_none()
                    && prepared.replacing.is_none() =>
            {
                self.window = Some(prepared.confirmation);
                self.window_id = Some(id);
                Some(id)
            }
            WindowTransition::Replaced { old, new }
                if self.window_id == Some(old) && prepared.replacing == Some(old) =>
            {
                self.window = Some(prepared.confirmation);
                self.window_id = Some(new);
                Some(new)
            }
            _ => None,
        }
    }

    #[must_use]
    pub fn is_open(&self) -> bool {
        self.window.is_some()
    }

    #[must_use]
    pub fn confirmation(&self) -> Option<&ScrapConfirmation> {
        self.window.as_ref()
    }

    #[must_use]
    pub const fn window_id(&self) -> Option<WindowId> {
        self.window_id
    }

    #[must_use]
    pub fn hit_window(
        &self,
        frame: &RoutingSnapshot,
        layout: CockpitLayout,
        point: (f32, f32),
    ) -> Option<WindowId> {
        let id = self.window_id?;
        (frame
            .key(id)
            .is_some_and(|key| key.kind == WindowKind::ScrapConfirmation)
            && frame.allows_input(id)
            && rect_contains(window_rect(layout), egui::pos2(point.0, point.1)))
        .then_some(id)
    }

    #[must_use]
    pub fn keyboard_owner(&self, frame: &RoutingSnapshot) -> Option<WindowId> {
        let id = self.window_id?;
        matches!(
            frame.keyboard_route().target,
            rebellion_windowing::RouteTarget::Window(target) if target.window == id
        )
        .then_some(id)
    }

    #[must_use]
    pub fn content_reports(&self, frame: &RoutingSnapshot) -> Vec<ContentReport> {
        frame
            .paint_order()
            .iter()
            .copied()
            .filter(|id| {
                frame
                    .key(*id)
                    .is_some_and(|key| key.kind == WindowKind::ScrapConfirmation)
            })
            .map(|id| ContentReport {
                window: id,
                status: if self.window_id == Some(id) {
                    ContentStatus::Ready
                } else {
                    ContentStatus::Unpaintable
                },
            })
            .collect()
    }

    #[must_use]
    pub const fn request_close(&self) -> Option<WindowId> {
        self.window_id
    }

    pub fn remove_content(&mut self, removed: &[WindowId]) {
        if self.window_id.is_some_and(|id| removed.contains(&id)) {
            self.window = None;
            self.window_id = None;
        }
    }

    #[must_use]
    pub fn prepare_confirm(&self) -> Option<PreparedScrapConfirmationConfirm> {
        Some(PreparedScrapConfirmationConfirm {
            window: self.window_id?,
            action: self.window.as_ref()?.confirm_action(),
        })
    }

    pub fn commit_confirm(
        &mut self,
        prepared: PreparedScrapConfirmationConfirm,
        removed: &[WindowId],
    ) -> Option<ScrapConfirmationAction> {
        if !removed.contains(&prepared.window)
            || self.window_id != Some(prepared.window)
            || self.window.as_ref()?.confirm_action() != prepared.action
        {
            return None;
        }
        self.window = None;
        self.window_id = None;
        Some(prepared.action)
    }

    /// Whether `point` falls on the open window.
    #[must_use]
    pub fn contains_screen_point(&self, layout: CockpitLayout, point: (f32, f32)) -> bool {
        self.window.is_some() && rect_contains(window_rect(layout), egui::pos2(point.0, point.1))
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cockpit::CockpitViewport;
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
        CockpitLayout {
            canvas: CockpitViewport {
                x: 10.0,
                y: 20.0,
                width: 640.0,
                height: 480.0,
            },
            galaxy: CockpitViewport {
                x: 65.0,
                y: 60.0,
                width: 485.0,
                height: 350.0,
            },
            scale: 1.0,
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

    fn key(key: egui::Key) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        }
    }

    /// Run two idle frames, then one frame per entry of `frames`, with the
    /// pointer at window pixel `(x, y)`.
    fn drive(
        state: &mut MoveConfirmationState,
        (x, y): (f32, f32),
        frames: Vec<Vec<egui::Event>>,
    ) -> Option<MoveConfirmationAction> {
        let layout = layout();
        let pos = window_rect(layout).min + egui::vec2(x, y);
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
                if let Some(action) = draw_move_confirmation(ctx, state, layout, &mut cache) {
                    emitted = Some(action);
                }
            });
        }
        emitted
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
        let target = world.fleets.insert(rebellion_core::world::Fleet {
            location: SystemKey::default(),
            capital_ships: Vec::new(),
            fighters: Vec::new(),
            characters: Vec::new(),
            is_alliance: true,
            has_death_star: false,
        });
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
        let mut fleet = || {
            world.fleets.insert(rebellion_core::world::Fleet {
                location: SystemKey::default(),
                capital_ships: Vec::new(),
                fighters: Vec::new(),
                characters: Vec::new(),
                is_alliance: true,
                has_death_star: false,
            })
        };
        let team = vec![fleet(), fleet()];
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

    fn managed_move_open(
        state: &mut MoveConfirmationState,
        confirmation: MoveConfirmation,
    ) -> (WindowManager, rebellion_windowing::WindowId) {
        let prepared = state
            .prepare_open(confirmation)
            .expect("valid Move confirmation context");
        let mut manager = WindowManager::new();
        let committed = manager
            .apply(WindowCommand::Open(prepared.spec(SubjectId::new(
                crate::windowing_policy::SUBJECT_COMMAND_CONTEXT,
                1,
            ))))
            .expect("manager accepts Move confirmation");
        let WindowTransition::Opened { id } = committed.transition else {
            panic!("expected new Move confirmation identity");
        };
        assert_eq!(state.commit_open(prepared, &committed.transition), Some(id));
        (manager, id)
    }

    fn scrap_confirmation(faction: MissionFaction) -> ScrapConfirmation {
        ScrapConfirmation {
            faction,
            order: ScrapOrder::Troop(Default::default()),
            targets: vec![ScrapTarget::Troop(Default::default())],
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
        let mut manager = WindowManager::new();
        let committed = manager
            .apply(WindowCommand::Open(prepared.spec(SubjectId::new(
                crate::windowing_policy::SUBJECT_COMMAND_CONTEXT,
                1,
            ))))
            .expect("manager accepts Scrap confirmation");
        let WindowTransition::Opened { id } = committed.transition else {
            panic!("expected new Scrap confirmation identity");
        };
        assert_eq!(state.commit_open(prepared, &committed.transition), Some(id));
        (manager, id)
    }

    #[test]
    fn move_confirmation_managed_adapter_enforces_exact_content_and_routes() {
        let mut invalid_both = confirmation(MissionFaction::Alliance);
        invalid_both.members = vec![MissionMember::Character(Default::default())];
        invalid_both.lines.push(("Leia".into(), 2));
        assert!(MoveConfirmationState::default()
            .prepare_open(invalid_both)
            .is_none());

        let valid_member = MoveConfirmation {
            fleets: Vec::new(),
            members: vec![MissionMember::Character(Default::default())],
            lines: vec![("Leia".into(), 2)],
            ..confirmation(MissionFaction::Alliance)
        };
        assert!(MoveConfirmationState::default()
            .prepare_open(valid_member)
            .is_some());

        let mut legacy = open(MissionFaction::Alliance);
        let inert = legacy
            .prepare_open(confirmation(MissionFaction::Empire))
            .unwrap();
        let mut unrelated = WindowManager::new();
        let unrelated = unrelated
            .apply(WindowCommand::Open(
                crate::windowing_policy::singleton_spec(
                    WindowKind::Encyclopedia,
                    WindowOwner::StrategicRoot,
                ),
            ))
            .unwrap();
        let WindowTransition::Opened { id: unrelated_id } = unrelated.transition else {
            panic!("expected unrelated identity");
        };
        assert_eq!(legacy.commit_open(inert, &unrelated.transition), None);
        assert_eq!(
            legacy.confirmation().map(|value| value.faction),
            Some(MissionFaction::Alliance)
        );

        let mut state = MoveConfirmationState::default();
        let (mut manager, id) =
            managed_move_open(&mut state, confirmation(MissionFaction::Alliance));
        let frame = manager.begin_frame();
        let rect = window_rect(layout());
        assert_eq!(
            state.hit_window(&frame, layout(), (rect.center().x, rect.center().y)),
            Some(id)
        );
        assert_eq!(
            state.hit_window(&frame, layout(), (rect.min.x - 1.0, rect.min.y)),
            None
        );
        assert_eq!(state.keyboard_owner(&frame), Some(id));
        assert_eq!(
            state.content_reports(&frame),
            [ContentReport {
                window: id,
                status: ContentStatus::Ready,
            }]
        );
        assert_eq!(state.request_close(), Some(id));

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
        assert_eq!(
            state.hit_window(&blocked, layout(), (rect.center().x, rect.center().y)),
            None
        );
        assert_eq!(state.keyboard_owner(&blocked), None);

        let replacement = state
            .prepare_open(confirmation(MissionFaction::Empire))
            .unwrap();
        assert_eq!(replacement.replacing(), Some(id));
        assert_eq!(
            state.commit_open(
                replacement.clone(),
                &WindowTransition::Opened { id: blocker }
            ),
            None
        );
        assert_eq!(
            state.commit_open(
                replacement.clone(),
                &WindowTransition::Replaced {
                    old: blocker,
                    new: unrelated_id,
                }
            ),
            None
        );
        state.window_id = Some(blocker);
        assert_eq!(
            state.commit_open(
                replacement.clone(),
                &WindowTransition::Replaced {
                    old: id,
                    new: unrelated_id,
                }
            ),
            None
        );
        state.window_id = Some(id);

        let prepared = state.prepare_confirm().unwrap();
        assert_eq!(state.commit_confirm(prepared.clone(), &[blocker]), None);
        let original = state.window.clone();
        state.window.as_mut().unwrap().join = Some(FleetKey::default());
        assert_eq!(state.commit_confirm(prepared.clone(), &[id]), None);
        state.window = original;
        assert!(matches!(
            state.commit_confirm(prepared.clone(), &[id]),
            Some(MoveConfirmationAction::Confirm { .. })
        ));
        assert_eq!(state.commit_confirm(prepared, &[id]), None);

        let mut removable = MoveConfirmationState::default();
        let (_, removable_id) =
            managed_move_open(&mut removable, confirmation(MissionFaction::Alliance));
        removable.remove_content(&[blocker]);
        assert_eq!(removable.window_id(), Some(removable_id));
        removable.remove_content(&[removable_id]);
        assert!(!removable.is_open());
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
        let mut unrelated = WindowManager::new();
        let unrelated = unrelated
            .apply(WindowCommand::Open(
                crate::windowing_policy::singleton_spec(
                    WindowKind::Encyclopedia,
                    WindowOwner::StrategicRoot,
                ),
            ))
            .unwrap();
        let WindowTransition::Opened { id: unrelated_id } = unrelated.transition else {
            panic!("expected unrelated identity");
        };
        assert_eq!(legacy.commit_open(inert, &unrelated.transition), None);
        assert_eq!(
            legacy.confirmation().map(|value| value.faction),
            Some(MissionFaction::Alliance)
        );

        let mut state = ScrapConfirmationState::default();
        let (mut manager, id) =
            managed_scrap_open(&mut state, scrap_confirmation(MissionFaction::Alliance));
        assert_eq!(state.window_id(), Some(id));
        let frame = manager.begin_frame();
        let rect = window_rect(layout());
        assert_eq!(
            state.hit_window(&frame, layout(), (rect.center().x, rect.center().y)),
            Some(id)
        );
        assert_eq!(
            state.hit_window(&frame, layout(), (rect.min.x - 1.0, rect.min.y)),
            None
        );
        assert_eq!(state.keyboard_owner(&frame), Some(id));
        assert_eq!(
            state.content_reports(&frame),
            [ContentReport {
                window: id,
                status: ContentStatus::Ready,
            }]
        );
        assert_eq!(state.request_close(), Some(id));

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
        assert_eq!(
            state.hit_window(&blocked, layout(), (rect.center().x, rect.center().y)),
            None
        );
        assert_eq!(state.keyboard_owner(&blocked), None);

        let replacement = state
            .prepare_open(scrap_confirmation(MissionFaction::Empire))
            .unwrap();
        assert_eq!(replacement.replacing(), Some(id));
        assert_eq!(
            state.commit_open(
                replacement.clone(),
                &WindowTransition::Opened { id: blocker }
            ),
            None
        );
        assert_eq!(
            state.commit_open(
                replacement.clone(),
                &WindowTransition::Replaced {
                    old: blocker,
                    new: unrelated_id,
                }
            ),
            None
        );
        state.window_id = Some(blocker);
        assert_eq!(
            state.commit_open(
                replacement.clone(),
                &WindowTransition::Replaced {
                    old: id,
                    new: unrelated_id,
                }
            ),
            None
        );
        state.window_id = Some(id);
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

        let prepared = state.prepare_confirm().unwrap();
        assert_eq!(state.commit_confirm(prepared.clone(), &[previous_id]), None);
        let original = state.window.clone();
        state.window.as_mut().unwrap().faction = MissionFaction::Alliance;
        assert_eq!(state.commit_confirm(prepared.clone(), &[id]), None);
        state.window = original;
        assert!(matches!(
            state.commit_confirm(prepared.clone(), &[id]),
            Some(ScrapConfirmationAction::Confirm { .. })
        ));
        assert_eq!(state.commit_confirm(prepared, &[id]), None);

        let mut removable = ScrapConfirmationState::default();
        let (_, removable_id) =
            managed_scrap_open(&mut removable, scrap_confirmation(MissionFaction::Alliance));
        removable.remove_content(&[blocker]);
        assert_eq!(removable.window_id(), Some(removable_id));
        removable.remove_content(&[removable_id]);
        assert!(!removable.is_open());
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

        let committed = manager
            .apply(WindowCommand::Replace {
                old: id,
                new: replacement.spec(SubjectId::new(
                    crate::windowing_policy::SUBJECT_COMMAND_CONTEXT,
                    3,
                )),
            })
            .expect("fresh Move context replaces atomically");
        let WindowTransition::Replaced { new, .. } = committed.transition else {
            panic!("expected Move replacement");
        };
        assert_eq!(
            state.commit_open(replacement, &committed.transition),
            Some(new)
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
            .prepare_open(ScrapConfirmation {
                faction: MissionFaction::Alliance,
                order: ScrapOrder::Troop(Default::default()),
                targets: vec![ScrapTarget::Troop(Default::default())],
                names: vec!["Army Regiment".into()],
            })
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
        (x, y): (f32, f32),
        frames: Vec<Vec<egui::Event>>,
    ) -> Option<ScrapConfirmationAction> {
        let layout = layout();
        let pos = window_rect(layout).min + egui::vec2(x, y);
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
                if let Some(action) = draw_scrap_confirmation(ctx, state, layout, &mut cache) {
                    emitted = Some(action);
                }
            });
        }
        emitted
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
}
