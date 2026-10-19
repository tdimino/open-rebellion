//! The original Status window (window type `0x1a`, `FUN_00442d70`; create
//! `FUN_00443130`; controls `FUN_00443020`, `FUN_004443a0`). The object
//! menu's Status (`0x103`) opens it (`FUN_0042a440`). Recovery notes:
//! `ghidra/notes/status-window.md`.
//!
//! A 379 by 272 window: a two-column list of labels and values on the left,
//! the object's portrait and name on the right, and the Encyclopedia and
//! Close buttons under them. Each object family has its own filler
//! (`status_rows.rs`); characters are `FUN_004486f0` (manual p. 101, Fig.
//! 3.46).

use egui_macroquad::egui;
use rebellion_core::ids::{CharacterKey, FighterKey};
use rebellion_core::missions::{MemberTransit, MissionMember};
use rebellion_core::officer_rank::CommandPost;
use rebellion_core::world::GameWorld;
use rebellion_windowing::{
    ContentReport, ContentStatus, ExistingEffect, RoutingSnapshot, SubjectId, WindowId, WindowKind,
    WindowOwner, WindowSpec, WindowTransition,
};

use crate::bmp_cache::BmpCache;
use crate::cockpit::{CockpitFaction, CockpitLayout};
use crate::fleet_window::paint_native;
use crate::mission_dialog::{button, galaxy_centered_rect, paint};
use crate::status_rows::{status_view, StatusObject, StatusSources};
use crate::system_window::{fleet_label, logical_rect, rect_contains};
use crate::theme::game_font;

pub const STATUS_WINDOW_WIDTH: f32 = 379.0;
pub const STATUS_WINDOW_HEIGHT: f32 = 272.0;

/// The background (STRATEGY `0x2d22..`), by the object's side bits, for a
/// side-1 player; a player of another side takes the bitmap three on.
const BACKGROUND: [u32; 3] = [11_554, 11_555, 11_556];
/// `0x66`, Encyclopedia, and `0x65`, Close (`FUN_00443020`).
const ENCYCLOPEDIA: (u32, u32) = (11_552, 11_553);
const CLOSE: (u32, u32) = (10_370, 10_371);
const ENCYCLOPEDIA_RECT: (f32, f32, f32, f32) = (258.0, 218.0, 32.0, 31.0);
const CLOSE_RECT: (f32, f32, f32, f32) = (324.0, 218.0, 32.0, 31.0);

/// `+0x130`: (15, 18), 211 by 18, font 5, format `0x21`.
const TITLE: (f32, f32, f32, f32) = (15.0, 18.0, 211.0, 18.0);
/// `+0x198`: (242, 137), 130 by 44, font 5, format `0x11`.
const NAME: (f32, f32, f32, f32) = (242.0, 137.0, 130.0, 44.0);
/// `+0x118`: the list, (18, 47), 208 by 204.
const LIST: (f32, f32, f32, f32) = (18.0, 47.0, 208.0, 204.0);
/// `FUN_00449e00`: labels at x 0 and values at x 103, each 103 wide.
const COLUMN: f32 = 103.0;
/// The picture is centred on (307, 64) (`FUN_00443130`).
const PICTURE_CENTRE: (i32, i32) = (307, 64);

/// One list row: `FUN_0044b1d0` appends the label to one list and the value
/// to the other.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusRow {
    pub label: String,
    pub value: String,
}

fn row(label: &str, value: impl Into<String>) -> StatusRow {
    StatusRow {
        label: label.to_owned(),
        value: value.into(),
    }
}

/// TEXTSTRA 34646 "Yes" or 34645 "No".
const fn yes_no(value: bool) -> &'static str {
    if value {
        "Yes"
    } else {
        "No"
    }
}

/// Force Ranking by the Force value `+0x8c` (`FUN_004486f0`).
#[must_use]
pub const fn force_ranking(value: u32) -> &'static str {
    match value {
        0..=9 => "None",
        10..=19 => "Novice",
        20..=79 => "Trainee",
        80..=99 => "Jedi Student",
        100..=119 => "Jedi Knight",
        _ => "Jedi Master",
    }
}

/// The Character Status rows (`FUN_004486f0`). `en_route` is the mission
/// transits (`MissionState::en_route`), which give an en route character's
/// arrival day.
///
/// Commanding names the fleet or system the character's rank commands
/// (`+0xa8`). port: "Injured" (`+0x94`) never shows: the port keeps no
/// injury. An en route character stands in its destination container from
/// the order on (`FUN_00556390`, `move-order.md`), so Attached names it.
#[must_use]
pub fn character_rows(
    world: &GameWorld,
    en_route: &[MemberTransit],
    key: CharacterKey,
) -> Option<Vec<StatusRow>> {
    let character = world.characters.get(key)?;
    let transit = en_route
        .iter()
        .find(|transit| transit.member == MissionMember::Character(key))
        .filter(|_| character.en_route);
    let attached = character
        .current_fleet
        .and_then(|fleet| fleet_label(world, fleet))
        .or_else(|| {
            character
                .current_system
                .and_then(|system| world.systems.get(system))
                .map(|system| system.name.clone())
        })
        .unwrap_or_default();
    // The first that holds: en route (+0x50 bit 4), captured (+0xac bit 0),
    // injured (+0x94), a mission that is not hidden (FUN_00520b70).
    let status = if character.en_route {
        "Enroute"
    } else if character.is_captive {
        "Captured"
    } else if character.on_mission && !character.on_hidden_mission {
        "On Mission"
    } else {
        "Awaiting Orders"
    };
    let commanding = match character.command_post {
        Some(CommandPost::Fleet(fleet)) => fleet_label(world, fleet),
        Some(CommandPost::System(system)) => {
            world.systems.get(system).map(|system| system.name.clone())
        }
        None => None,
    }
    .unwrap_or_else(|| "None".to_owned());
    let mut rows = vec![
        row("Commanding:", commanding),
        // FUN_0044a2e0: the literal, for the shipped TEXTSTRA 1.00.00
        // (FUN_00406850).
        row("Attached: ", attached),
        row("Status:", status),
    ];
    // FUN_004fd2b0: "ETA Destination:" and TEXTSTRA 14356 "Day " plus the
    // arrival day, only while en route.
    if let Some(transit) = transit {
        rows.push(row("ETA Destination:", format!("Day {}", transit.arrival)));
    }
    rows.extend([
        row("Force Ranking:", force_ranking(character.jedi_level.base)),
        row("Diplomacy Rating:", character.diplomacy.base.to_string()),
        row("Espionage Rating:", character.espionage.base.to_string()),
        row("Combat Rating:", character.combat.base.to_string()),
        row("Leadership Rating:", character.leadership.base.to_string()),
        // TEXTSTRA 34608 "R&&D Capabilities", DrawText's escape for "&".
        row("R&D Capabilities", ""),
        row(" Ship Design", yes_no(character.ship_design.base != 0)),
        row(
            " Troop Training",
            yes_no(character.troop_training.base != 0),
        ),
        row(
            " Facility Design",
            yes_no(character.facility_design.base != 0),
        ),
        row("Possible Command Ranks", ""),
        row("Admiral:", yes_no(character.can_be_admiral)),
        row("General:", yes_no(character.can_be_general)),
        row("Commander:", yes_no(character.can_be_commander)),
    ]);
    Some(rows)
}

/// The background for an object of side index `side`, seen by `viewer`
/// (`FUN_00443130`); a side past 2 takes the third ("other") bitmap.
#[must_use]
pub fn background(side: usize, viewer: CockpitFaction) -> u32 {
    let base = BACKGROUND.get(side).copied().unwrap_or(BACKGROUND[2]);
    match viewer {
        CockpitFaction::Alliance => base,
        CockpitFaction::Empire => base + 3,
    }
}

/// What the player chose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusWindowAction {
    /// `0x66`: close, then open the Encyclopedia (`FUN_0041d6b0`). port: on
    /// its index, as the object menu's Encyclopedia opens it.
    Encyclopedia,
    /// `0x65`, or Escape: close.
    Close,
}

#[derive(Debug, Clone, PartialEq)]
struct OpenStatusWindow {
    /// Exact manager identity, absent only for renderer fixture calls.
    id: Option<WindowId>,
    /// Per-logical-open context allocated by the app coordinator.
    subject: Option<SubjectId>,
    object: StatusObject,
    /// Stable facts captured when an index-backed object was prepared. Direct
    /// fixture opens have no witness and retain their legacy behavior.
    witness: Option<StatusWitness>,
    /// The list's scroll offset, in window pixels.
    scroll: f32,
}

/// Whether a Status window is open. There is one at a time: the galaxy view
/// builds it from its `0x468` handler, as the move confirmation.
#[derive(Debug, Clone, Default)]
pub struct StatusWindowState {
    window: Option<OpenStatusWindow>,
}

/// Validated Status content. It remains inert until the manager accepts its
/// contextual identity and returns an open or replacement receipt.
#[derive(Debug, Clone, Copy)]
pub struct PreparedStatusWindow {
    subject: SubjectId,
    object: StatusObject,
    witness: StatusWitness,
}

/// Identity evidence for a committed Status object. Most object families use
/// stable arena keys directly; the two fleet-vector families need an explicit
/// witness so index compaction cannot silently retarget an open modal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StatusWitness {
    Direct,
    Ship { roster: u64, tag: u32 },
    Fighter { class: FighterKey, carrier: u32 },
}

fn status_witness(sources: StatusSources<'_>, object: StatusObject) -> Option<StatusWitness> {
    match object {
        StatusObject::Ship { fleet, index } => {
            let roster = rebellion_core::fleet_join::roster(sources.world, fleet)?;
            let ship = sources.world.fleets.get(fleet)?.capital_ships.get(index)?;
            ship.alive.then_some(StatusWitness::Ship {
                roster,
                tag: ship.tag,
            })
        }
        StatusObject::Fighter { fleet, index } => {
            let fighter = sources.world.fleets.get(fleet)?.fighters.get(index)?;
            (fighter.count > 0).then_some(StatusWitness::Fighter {
                class: fighter.class,
                carrier: fighter.carrier,
            })
        }
        _ => Some(StatusWitness::Direct),
    }
}

fn status_view_if_current(
    sources: StatusSources<'_>,
    object: StatusObject,
    witness: Option<StatusWitness>,
) -> Option<crate::status_rows::StatusView> {
    match (object, witness) {
        (_, None | Some(StatusWitness::Direct)) => status_view(sources, object),
        (StatusObject::Ship { fleet, index }, Some(StatusWitness::Ship { roster, tag })) => {
            (rebellion_core::fleet_join::roster(sources.world, fleet) == Some(roster))
                .then_some(())?;
            let ship = sources.world.fleets.get(fleet)?.capital_ships.get(index)?;
            (ship.alive && (tag == 0 || ship.tag == tag)).then_some(())?;
            status_view(sources, object)
        }
        (
            StatusObject::Fighter { fleet, index },
            Some(StatusWitness::Fighter { class, carrier }),
        ) => {
            let fighter = sources.world.fleets.get(fleet)?.fighters.get(index)?;
            (fighter.count > 0 && fighter.class == class && fighter.carrier == carrier)
                .then_some(())?;
            status_view(sources, object)
        }
        _ => None,
    }
}

impl PreparedStatusWindow {
    #[must_use]
    pub const fn spec(self) -> WindowSpec {
        crate::windowing_policy::contextual_spec(
            WindowKind::Status,
            self.subject,
            WindowOwner::StrategicRoot,
        )
    }
}

impl StatusWindowState {
    /// Validates a runtime object before any manager or published-content
    /// mutation. A stale key therefore cannot replace or retarget an existing
    /// Status context.
    #[must_use]
    pub fn prepare_open(
        sources: StatusSources<'_>,
        object: StatusObject,
        subject: SubjectId,
    ) -> Option<PreparedStatusWindow> {
        status_view(sources, object)?;
        let witness = status_witness(sources, object)?;
        Some(PreparedStatusWindow {
            subject,
            object,
            witness,
        })
    }

    /// Publishes validated content only for the manager receipt that owns it.
    /// Duplicate `NoOp` opens retain the prior object and scroll position;
    /// replacement swaps content only after the manager transaction succeeds.
    pub fn commit_open(
        &mut self,
        prepared: PreparedStatusWindow,
        transition: &WindowTransition,
    ) -> Option<WindowId> {
        match *transition {
            WindowTransition::Opened { id } if self.window.is_none() => {
                self.window = Some(OpenStatusWindow {
                    id: Some(id),
                    subject: Some(prepared.subject),
                    object: prepared.object,
                    witness: Some(prepared.witness),
                    scroll: 0.0,
                });
                Some(id)
            }
            WindowTransition::Replaced { old, new } if self.window_id() == Some(old) => {
                self.window = Some(OpenStatusWindow {
                    id: Some(new),
                    subject: Some(prepared.subject),
                    object: prepared.object,
                    witness: Some(prepared.witness),
                    scroll: 0.0,
                });
                Some(new)
            }
            WindowTransition::Existing {
                id,
                effect: ExistingEffect::NoOp,
            } if self.window_id() == Some(id)
                && self.window.as_ref().and_then(|window| window.subject)
                    == Some(prepared.subject) =>
            {
                Some(id)
            }
            _ => None,
        }
    }

    /// Exact manager identity currently owning the published content.
    #[must_use]
    pub fn window_id(&self) -> Option<WindowId> {
        self.window.as_ref().and_then(|window| window.id)
    }

    /// Manager-owned Status hit for event-local pointer routing.
    #[must_use]
    pub fn hit_window(
        &self,
        frame: &RoutingSnapshot,
        layout: CockpitLayout,
        point: (f32, f32),
    ) -> Option<WindowId> {
        let id = self.window_id()?;
        (frame
            .key(id)
            .is_some_and(|key| key.kind == WindowKind::Status)
            && frame.allows_input(id)
            && rect_contains(window_rect(layout), egui::pos2(point.0, point.1)))
        .then_some(id)
    }

    /// Exact Status identity that owns this frame's keyboard lease.
    #[must_use]
    pub fn keyboard_owner(&self, frame: &RoutingSnapshot) -> Option<WindowId> {
        let id = self.window_id()?;
        matches!(
            frame.keyboard_route().target,
            rebellion_windowing::RouteTarget::Window(target) if target.window == id
        )
        .then_some(id)
    }

    /// Exact committed identity used by button, Escape, and recovery closes.
    #[must_use]
    pub fn request_close(&self) -> Option<WindowId> {
        self.window_id()
    }

    /// Reports whether each manager-owned Status identity has paintable
    /// content. Missing or stale content stays explicitly unpaintable so its
    /// modal boundary remains fail-closed and recoverable.
    #[must_use]
    pub fn content_reports(
        &self,
        frame: &RoutingSnapshot,
        sources: StatusSources<'_>,
    ) -> Vec<ContentReport> {
        frame
            .paint_order()
            .iter()
            .copied()
            .filter(|id| {
                frame
                    .key(*id)
                    .is_some_and(|key| key.kind == WindowKind::Status)
            })
            .map(|id| ContentReport {
                window: id,
                status: if self.window_id() == Some(id)
                    && self.window.as_ref().is_some_and(|window| {
                        status_view_if_current(sources, window.object, window.witness).is_some()
                    }) {
                    ContentStatus::Ready
                } else {
                    ContentStatus::Unpaintable
                },
            })
            .collect()
    }

    /// Releases only content named by manager close/replacement receipts.
    pub fn remove_content(&mut self, removed: &[WindowId]) {
        if self.window_id().is_some_and(|id| removed.contains(&id)) {
            self.window = None;
        }
    }

    /// `FUN_0042a440` → the `0x468` handler: open on an object the player
    /// knows (`FUN_004f2d10`).
    pub fn open(&mut self, object: StatusObject) {
        self.window = Some(OpenStatusWindow {
            id: None,
            subject: None,
            object,
            witness: None,
            scroll: 0.0,
        });
    }

    /// Open on a character.
    pub fn open_character(&mut self, character: CharacterKey) {
        self.open(StatusObject::Character(character));
    }

    #[must_use]
    pub fn is_open(&self) -> bool {
        self.window.is_some()
    }

    /// Whether `point` falls on the open window, so the galaxy map under it
    /// takes no input.
    #[must_use]
    pub fn contains_screen_point(&self, layout: CockpitLayout, point: (f32, f32)) -> bool {
        self.window.is_some() && rect_contains(window_rect(layout), egui::pos2(point.0, point.1))
    }
}

/// The window's screen rectangle (`FUN_00606980` places it from the galaxy
/// view's rectangle). hyp: centred there, as the move confirmation is.
#[must_use]
pub fn window_rect(layout: CockpitLayout) -> egui::Rect {
    galaxy_centered_rect(layout, STATUS_WINDOW_WIDTH, STATUS_WINDOW_HEIGHT)
}

/// `FUN_00449e00`'s row placement from each row's (label, value) height: a
/// row is as tall as its taller cell, and a shorter value moves down to the
/// label's bottom. Returns each row's (label, value) top and the total.
fn row_tops(heights: &[(f32, f32)]) -> (Vec<(f32, f32)>, f32) {
    let mut tops = Vec::with_capacity(heights.len());
    let mut y = 0.0;
    for &(label, value) in heights {
        tops.push((y, y + (label - value).max(0.0)));
        y += label.max(value);
    }
    (tops, y)
}

/// The picture's top-left in the window: centred on (307, 64) with integer
/// halves (`FUN_00443130`).
#[expect(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "Bitmap sizes are small integers."
)]
fn picture_origin(width: usize, height: usize) -> (f32, f32) {
    let (cx, cy) = PICTURE_CENTRE;
    (
        (cx - width as i32 / 2) as f32,
        (cy - height as i32 / 2) as f32,
    )
}

/// The list's scroll after a wheel `delta` (screen pixels), kept within the
/// rows' `height` (screen pixels) less the list's own.
fn scrolled(scroll: f32, delta: f32, scale: f32, height: f32) -> f32 {
    (scroll - delta / scale).clamp(0.0, (height / scale - LIST.3).max(0.0))
}

/// One laid-out row: its label and value galleys and their tops.
type LaidRow = (
    std::sync::Arc<egui::Galley>,
    std::sync::Arc<egui::Galley>,
    f32,
    f32,
);

/// Lay out the rows' galleys and place them with [`row_tops`].
fn layout_rows(
    painter: &egui::Painter,
    rows: &[StatusRow],
    font: &egui::FontId,
    column: f32,
) -> (Vec<LaidRow>, f32) {
    let galleys: Vec<_> = rows
        .iter()
        .map(|row| {
            let layout = |text: &str| {
                painter.layout(text.to_owned(), font.clone(), egui::Color32::WHITE, column)
            };
            (layout(&row.label), layout(&row.value))
        })
        .collect();
    let heights: Vec<_> = galleys
        .iter()
        .map(|(label, value)| (label.size().y, value.size().y))
        .collect();
    let (tops, height) = row_tops(&heights);
    let laid = galleys
        .into_iter()
        .zip(tops)
        .map(|((label, value), (label_y, value_y))| (label, value, label_y, value_y))
        .collect();
    (laid, height)
}

/// Draw the open window, if any, and report what the player chose.
pub fn draw_status_window(
    ctx: &egui::Context,
    sources: StatusSources<'_>,
    state: &mut StatusWindowState,
    viewer: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Option<StatusWindowAction> {
    let object = state.window.as_ref()?.object;
    // Direct renderer fixtures retain the legacy disappearance behavior.
    // Production uses `draw_managed_status_window`, where stale committed
    // content stays modal and exposes an explicit recovery close.
    if status_view(sources, object).is_none() {
        state.window = None;
        return None;
    }
    let action = draw_status_window_surface(ctx, sources, state, viewer, layout, cache, true, true);
    if action.is_some() {
        state.window = None;
    }
    action
}

/// Draw manager-owned Status content against its exact immutable-frame
/// identity. A control action names that identity and leaves content in place
/// until the app supplies the manager's removal receipt.
pub fn draw_managed_status_window(
    ctx: &egui::Context,
    sources: StatusSources<'_>,
    state: &mut StatusWindowState,
    viewer: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
    frame: &RoutingSnapshot,
) -> Option<(WindowId, StatusWindowAction)> {
    let id = state.window_id()?;
    if !frame
        .key(id)
        .is_some_and(|key| key.kind == WindowKind::Status)
    {
        return None;
    }
    draw_status_window_surface(
        ctx,
        sources,
        state,
        viewer,
        layout,
        cache,
        frame.allows_input(id),
        false,
    )
    .map(|action| (id, action))
}

#[expect(
    clippy::too_many_arguments,
    reason = "Shared implementation for production snapshots and fixture-only direct content."
)]
fn draw_status_window_surface(
    ctx: &egui::Context,
    sources: StatusSources<'_>,
    state: &mut StatusWindowState,
    viewer: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
    input_enabled: bool,
    raw_escape_enabled: bool,
) -> Option<StatusWindowAction> {
    let open = state.window.as_mut()?;
    let view = status_view_if_current(sources, open.object, open.witness);
    let scale = layout.scale;
    let rect = window_rect(layout);
    let mut action = None;

    // Above the modeless windows, as the move confirmation is.
    let area_id = egui::Id::new("original-status-window");
    // Lower managed surfaces also reassert their persistent egui layers each
    // frame. Reassert Status after they paint so the manager's immutable
    // paint order is the actual visual and input order too.
    ctx.move_to_top(egui::LayerId::new(egui::Order::Tooltip, area_id));
    egui::Area::new(area_id)
        .fixed_pos(rect.min)
        .order(egui::Order::Tooltip)
        .show(ctx, |ui| {
            let (frame, _) = ui.allocate_exact_size(rect.size(), egui::Sense::hover());
            let painter = ui.painter().with_clip_rect(frame);
            let at = |(x, y, w, h): (f32, f32, f32, f32)| logical_rect(frame, scale, x, y, w, h);

            if let Some(view) = view {
                paint(
                    &painter,
                    ctx,
                    cache,
                    background(view.side, viewer),
                    frame,
                    scale,
                );

                // The title and name take font 5 and the list font 10
                // (FUN_0060eed0). The production manager's font 0x11 is the
                // table's default, the same 16 pixels as font 5.
                let title_font = game_font(5, scale);
                let list_font = game_font(10, scale);

                let title = at(TITLE);
                painter.text(
                    title.center(),
                    egui::Align2::CENTER_CENTER,
                    view.title,
                    title_font.clone(),
                    egui::Color32::WHITE,
                );

                // The picture, keyed (FUN_005fd0f0), centred with integer
                // offsets on (307, 64).
                if let Some((source, picture)) = view.picture {
                    if let Some([width, height]) = cache.original_resource_size(source, picture) {
                        let (x, y) = picture_origin(width, height);
                        paint_native(&painter, ctx, cache, source, picture, frame, scale, x, y);
                    }
                }

                let name = at(NAME);
                let galley =
                    painter.layout(view.name, title_font, egui::Color32::WHITE, name.width());
                let name_at = egui::pos2(
                    name.center().x - galley.size().x / 2.0,
                    name.center().y - galley.size().y / 2.0,
                );
                painter
                    .with_clip_rect(name)
                    .galley(name_at, galley, egui::Color32::WHITE);

                // The list. port: the wheel scrolls it; no scroll bar is drawn.
                let list = at(LIST);
                let (laid, height) = layout_rows(&painter, &view.rows, &list_font, COLUMN * scale);
                let list_response =
                    ui.interact(list, ui.id().with("status-list"), egui::Sense::hover());
                let wheel = if input_enabled && list_response.hovered() {
                    ui.ctx().input(|input| input.smooth_scroll_delta.y)
                } else {
                    0.0
                };
                open.scroll = scrolled(open.scroll, wheel, scale, height);
                let list_painter = painter.with_clip_rect(list);
                let top = list.min.y - open.scroll * scale;
                for (label, value, label_y, value_y) in laid {
                    list_painter.galley(
                        egui::pos2(list.min.x, top + label_y),
                        label,
                        egui::Color32::WHITE,
                    );
                    list_painter.galley(
                        egui::pos2(list.min.x + COLUMN * scale, top + value_y),
                        value,
                        egui::Color32::WHITE,
                    );
                }

                if input_enabled
                    && button(
                        ui,
                        cache,
                        at(ENCYCLOPEDIA_RECT),
                        "status-encyclopedia",
                        ENCYCLOPEDIA,
                        false,
                        scale,
                    )
                {
                    action = Some(StatusWindowAction::Encyclopedia);
                }
            } else {
                // Fail closed after a committed object's backing runtime row
                // disappears. The dark recovery surface keeps the modal
                // visible and gives the player an explicit close route.
                painter.rect_filled(frame, 0.0, egui::Color32::from_rgb(24, 24, 24));
                painter.text(
                    frame.center(),
                    egui::Align2::CENTER_CENTER,
                    "Status unavailable",
                    game_font(5, scale),
                    egui::Color32::WHITE,
                );
            }

            if input_enabled
                && button(
                    ui,
                    cache,
                    at(CLOSE_RECT),
                    "status-close",
                    CLOSE,
                    false,
                    scale,
                )
            {
                action = Some(StatusWindowAction::Close);
            }
        });

    // Direct fixture path only. Production routes Escape through the frozen
    // keyboard lease and asks the manager to close the exact identity.
    if raw_escape_enabled && ctx.input(|input| input.key_pressed(egui::Key::Escape)) {
        action = Some(StatusWindowAction::Close);
    }
    action
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::bmp_cache::DllSource;
    use crate::cockpit::CockpitViewport;
    use crate::windowing_input::{
        FrameInputRouter, InputDisposition, InputEventId, InputRecipient, PointerButtonId,
        PointerPhase,
    };
    use rebellion_core::dat::{ExplorationStatus, Faction, SectorGroup};
    use rebellion_core::ids::{DatId, FleetKey, SystemKey};
    use rebellion_core::world::{
        CapitalShipClass, Character, ControlKind, FighterClass, FighterEntry, Fleet, ForceTier,
        Sector, ShipInstance, SkillPair, System,
    };
    use rebellion_windowing::{
        ContentReport, ContentStatus, ExistingEffect, RouteTarget, SubjectId, WindowCommand,
        WindowManager, WindowTransition,
    };

    fn skill(base: u32) -> SkillPair {
        SkillPair { base, variance: 0 }
    }

    fn luke() -> Character {
        Character {
            // MJCHARSD index 578, TEXTSTRA 10306 (0x2842) "Luke Skywalker".
            dat_id: DatId::new(0x3000_0242),
            name: "Luke Skywalker".into(),
            is_alliance: true,
            is_empire: false,
            is_major: true,
            diplomacy: skill(75),
            espionage: skill(75),
            ship_design: skill(0),
            troop_training: skill(0),
            facility_design: skill(0),
            combat: skill(135),
            leadership: skill(70),
            loyalty: skill(100),
            jedi_probability: 0,
            jedi_level: skill(40),
            can_be_admiral: true,
            can_be_commander: true,
            can_be_general: false,
            force_tier: ForceTier::None,
            force_experience: 0,
            is_discovered_jedi: false,
            is_unable_to_betray: false,
            is_jedi_trainer: false,
            is_known_jedi: false,
            hyperdrive_modifier: 0,
            enhanced_loyalty: 0,
            on_mission: false,
            on_hidden_mission: false,
            on_mandatory_mission: false,
            captured_by: None,
            capture_tick: None,
            is_captive: false,
            current_system: None,
            current_fleet: None,
            en_route: false,
            heritage_known: false,
            is_killed: false,
            recruited: true,
            command_kind: rebellion_core::officer_rank::CommandKind::None,
            command_post: None,
            ranked_names: Default::default(),
        }
    }

    fn add_system(world: &mut GameWorld, name: &str) -> SystemKey {
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(36),
            name: "Sesswenna".into(),
            group: SectorGroup::Core,
            x: 317,
            y: 248,
            systems: Vec::new(),
        });
        world.systems.insert(System {
            dat_id: DatId::new(100),
            name: name.into(),
            sector,
            x: 320,
            y: 250,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: Vec::new(),
            ground_units: Vec::new(),
            special_forces: Vec::new(),
            fighters: Vec::new(),
            defense_facilities: Vec::new(),
            manufacturing_facilities: Vec::new(),
            production_facilities: Vec::new(),
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Alliance),
        })
    }

    fn world_with_luke() -> (GameWorld, CharacterKey, SystemKey, SystemKey) {
        let mut world = GameWorld::default();
        let yavin = add_system(&mut world, "Yavin");
        let hoth = add_system(&mut world, "Hoth");
        let mut character = luke();
        character.current_system = Some(yavin);
        let key = world.characters.insert(character);
        (world, key, yavin, hoth)
    }

    fn add_character(world: &mut GameWorld, name: &str) -> CharacterKey {
        let mut character = luke();
        character.name = name.into();
        world.characters.insert(character)
    }

    fn world_with_indexed_fleet() -> (GameWorld, FleetKey) {
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Yavin");
        let ships = [
            (0x1400_0010, "Ship A", 101),
            (0x1400_0011, "Ship B", 102),
            (0x1400_0012, "Ship C", 103),
        ]
        .map(|(dat_id, name, tag)| {
            let class = world.capital_ship_classes.insert(CapitalShipClass {
                dat_id: DatId::new(dat_id),
                name: name.into(),
                hull: 100,
                ..CapitalShipClass::default()
            });
            ShipInstance {
                tag,
                ..ShipInstance::new(class, 100, true)
            }
        })
        .to_vec();
        let fighters = [
            (0x1c00_0010, "Fighter A", 101),
            (0x1c00_0011, "Fighter B", 102),
            (0x1c00_0012, "Fighter C", 103),
        ]
        .map(|(dat_id, name, carrier)| FighterEntry {
            class: world.fighter_classes.insert(FighterClass {
                dat_id: DatId::new(dat_id),
                name: name.into(),
                squadron_size: 12,
                ..FighterClass::default()
            }),
            count: 1,
            carrier,
        })
        .to_vec();
        let fleet = world.insert_fleet(Fleet {
            location: system,
            capital_ships: ships,
            fighters,
            characters: Vec::new(),
            is_alliance: true,
            has_death_star: false,
        });
        world.systems[system].fleets.push(fleet);
        (world, fleet)
    }

    fn context(value: u64) -> SubjectId {
        SubjectId::new(crate::windowing_policy::SUBJECT_COMMAND_CONTEXT, value)
    }

    fn opened_status(
        manager: &mut WindowManager,
        state: &mut StatusWindowState,
        sources: StatusSources<'_>,
        object: StatusObject,
        subject: SubjectId,
    ) -> rebellion_windowing::WindowId {
        let prepared = StatusWindowState::prepare_open(sources, object, subject)
            .expect("valid Status object prepares content");
        let commit = manager
            .apply(WindowCommand::Open(prepared.spec()))
            .expect("Status opens");
        let id = match commit.transition {
            WindowTransition::Opened { id } => id,
            ref other => panic!("expected Status open, got {other:?}"),
        };
        assert_eq!(state.commit_open(prepared, &commit.transition), Some(id));
        id
    }

    fn managed_status_content(
        manager: &WindowManager,
        state: &StatusWindowState,
        sources: StatusSources<'_>,
    ) -> ContentStatus {
        state.content_reports(&manager.begin_frame(), sources)[0].status
    }

    #[test]
    fn status_window_windowing_preserves_context() {
        let (mut world, luke, ..) = world_with_luke();
        let han = add_character(&mut world, "Han Solo");
        let empty = Empty::new();
        let mut manager = WindowManager::new();
        let mut state = StatusWindowState::default();
        let subject = context(41);
        let id = opened_status(
            &mut manager,
            &mut state,
            empty.sources(&world),
            StatusObject::Character(luke),
            subject,
        );
        state.window.as_mut().unwrap().scroll = 17.0;

        let duplicate = StatusWindowState::prepare_open(
            empty.sources(&world),
            StatusObject::Character(han),
            subject,
        )
        .unwrap();
        let duplicate_commit = manager
            .apply(WindowCommand::Open(duplicate.spec()))
            .unwrap();
        assert!(matches!(
            duplicate_commit.transition,
            WindowTransition::Existing {
                id: existing,
                effect: ExistingEffect::NoOp,
            } if existing == id
        ));
        assert_eq!(
            state.commit_open(duplicate, &duplicate_commit.transition),
            Some(id)
        );

        let content = state.window.as_ref().unwrap();
        assert_eq!(content.object, StatusObject::Character(luke));
        assert_eq!(content.subject, Some(subject));
        assert_eq!(content.scroll, 17.0);
    }

    #[test]
    fn status_window_windowing_blocks_underlay() {
        let (world, luke, ..) = world_with_luke();
        let empty = Empty::new();
        let mut manager = WindowManager::new();
        let mut state = StatusWindowState::default();
        let id = opened_status(
            &mut manager,
            &mut state,
            empty.sources(&world),
            StatusObject::Character(luke),
            context(42),
        );
        let frame = manager.begin_frame();
        let inside = window_rect(layout()).center();
        let hits = state
            .hit_window(&frame, layout(), (inside.x, inside.y))
            .into_iter()
            .collect::<BTreeSet<_>>();

        assert_eq!(state.keyboard_owner(&frame), Some(id));
        assert_eq!(state.request_close(), Some(id));
        assert_eq!(
            state.hit_window(&frame, layout(), (0.0, 0.0)),
            None,
            "geometry excludes points outside the Status rectangle"
        );

        assert_eq!(
            frame.pointer_route(&hits).target,
            RouteTarget::Window(rebellion_windowing::FocusTarget {
                window: id,
                control: None,
            })
        );
        assert_eq!(
            frame.pointer_route(&BTreeSet::new()).target,
            RouteTarget::BlockedBy(id)
        );
        assert!(!frame.pointer_allows_root());
        assert_eq!(
            state.content_reports(&frame, empty.sources(&world)),
            [ContentReport {
                window: id,
                status: ContentStatus::Ready,
            }]
        );

        manager
            .apply(WindowCommand::Open(
                crate::windowing_policy::singleton_spec(
                    WindowKind::FleetFinder,
                    WindowOwner::StrategicRoot,
                ),
            ))
            .unwrap();
        let covered = manager.begin_frame();
        assert_eq!(
            state.hit_window(&covered, layout(), (inside.x, inside.y)),
            None,
            "a higher modal owns the point even inside Status geometry"
        );
        assert_eq!(state.keyboard_owner(&covered), None);
        assert_eq!(
            state.request_close(),
            Some(id),
            "the explicit recovery close remains exact while covered"
        );
    }

    #[test]
    fn status_window_windowing_close_keeps_lease() {
        let (world, luke, ..) = world_with_luke();
        let empty = Empty::new();
        let mut manager = WindowManager::new();
        let mut state = StatusWindowState::default();
        let id = opened_status(
            &mut manager,
            &mut state,
            empty.sources(&world),
            StatusObject::Character(luke),
            context(43),
        );
        let frame = manager.begin_frame();
        let mut router = FrameInputRouter::default();
        router.begin_frame(7, &frame);
        let escape = InputEventId {
            frame: 7,
            ordinal: 0,
        };
        assert_eq!(router.collect_keyboard(escape), InputRecipient::Window(id));

        let close = manager.apply(WindowCommand::Close(id)).unwrap();
        assert!(
            state.is_open(),
            "content waits for an exact removal receipt"
        );
        assert!(router.claim(escape, InputRecipient::Window(id)));
        router.report_key(
            escape,
            InputRecipient::Window(id),
            InputDisposition::Handled,
        );
        assert!(!router.root_may_handle(escape));

        let removed = match close.transition {
            WindowTransition::Closed { closed, .. } => closed,
            ref other => panic!("expected Status close, got {other:?}"),
        };
        state.remove_content(&removed);
        assert!(!state.is_open());
    }

    #[test]
    fn status_window_context_replacement_is_atomic() {
        let (mut world, luke, ..) = world_with_luke();
        let han = add_character(&mut world, "Han Solo");
        let empty = Empty::new();
        let mut manager = WindowManager::new();
        let mut state = StatusWindowState::default();
        let old = opened_status(
            &mut manager,
            &mut state,
            empty.sources(&world),
            StatusObject::Character(luke),
            context(44),
        );
        state.window.as_mut().unwrap().scroll = 9.0;
        let replacement = StatusWindowState::prepare_open(
            empty.sources(&world),
            StatusObject::Character(han),
            context(45),
        )
        .unwrap();

        assert_eq!(
            state.window.as_ref().unwrap().object,
            StatusObject::Character(luke)
        );
        let commit = manager
            .apply(WindowCommand::Replace {
                old,
                new: replacement.spec(),
            })
            .unwrap();
        assert_eq!(
            state.window.as_ref().unwrap().object,
            StatusObject::Character(luke)
        );
        let new = match commit.transition {
            WindowTransition::Replaced { old: removed, new } => {
                assert_eq!(removed, old);
                new
            }
            ref other => panic!("expected Status replacement, got {other:?}"),
        };
        assert_eq!(
            state.commit_open(replacement, &commit.transition),
            Some(new)
        );
        state.remove_content(&[old]);

        let content = state.window.as_ref().unwrap();
        assert_eq!(content.id, Some(new));
        assert_eq!(content.subject, Some(context(45)));
        assert_eq!(content.object, StatusObject::Character(han));
        assert_eq!(content.scroll, 0.0);
    }

    #[test]
    fn status_window_rejects_mismatched_publication_receipts() {
        let (mut world, luke, ..) = world_with_luke();
        let han = add_character(&mut world, "Han Solo");
        let empty = Empty::new();
        let mut manager = WindowManager::new();
        let mut state = StatusWindowState::default();
        let original_subject = context(56);
        let original = opened_status(
            &mut manager,
            &mut state,
            empty.sources(&world),
            StatusObject::Character(luke),
            original_subject,
        );
        state.window.as_mut().unwrap().scroll = 13.0;

        let unrelated_open = StatusWindowState::prepare_open(
            empty.sources(&world),
            StatusObject::Character(han),
            context(57),
        )
        .unwrap();
        let opened = manager
            .apply(WindowCommand::Open(unrelated_open.spec()))
            .unwrap();
        let unrelated = match opened.transition {
            WindowTransition::Opened { id } => id,
            ref other => panic!("expected unrelated Status open, got {other:?}"),
        };
        assert_eq!(state.commit_open(unrelated_open, &opened.transition), None);

        let unrelated_replacement = StatusWindowState::prepare_open(
            empty.sources(&world),
            StatusObject::Character(han),
            context(58),
        )
        .unwrap();
        let replaced = manager
            .apply(WindowCommand::Replace {
                old: unrelated,
                new: unrelated_replacement.spec(),
            })
            .unwrap();
        let replacement = match replaced.transition {
            WindowTransition::Replaced { new, .. } => new,
            ref other => panic!("expected unrelated Status replacement, got {other:?}"),
        };
        assert_eq!(
            state.commit_open(unrelated_replacement, &replaced.transition),
            None
        );

        let wrong_subject = StatusWindowState::prepare_open(
            empty.sources(&world),
            StatusObject::Character(han),
            context(59),
        )
        .unwrap();
        assert_eq!(
            state.commit_open(
                wrong_subject,
                &WindowTransition::Existing {
                    id: original,
                    effect: ExistingEffect::NoOp,
                },
            ),
            None
        );
        let right_subject = StatusWindowState::prepare_open(
            empty.sources(&world),
            StatusObject::Character(han),
            original_subject,
        )
        .unwrap();
        assert_eq!(
            state.commit_open(
                right_subject,
                &WindowTransition::Existing {
                    id: replacement,
                    effect: ExistingEffect::NoOp,
                },
            ),
            None
        );

        let content = state.window.as_ref().unwrap();
        assert_eq!(content.id, Some(original));
        assert_eq!(content.subject, Some(original_subject));
        assert_eq!(content.object, StatusObject::Character(luke));
        assert_eq!(content.scroll, 13.0);
    }

    #[test]
    fn status_window_outside_press_and_f_keys_do_not_reach_cockpit() {
        let (world, luke, ..) = world_with_luke();
        let empty = Empty::new();
        let mut manager = WindowManager::new();
        let mut state = StatusWindowState::default();
        let id = opened_status(
            &mut manager,
            &mut state,
            empty.sources(&world),
            StatusObject::Character(luke),
            context(46),
        );
        let frame = manager.begin_frame();
        let mut router = FrameInputRouter::default();
        router.begin_frame(8, &frame);
        let outside = InputEventId {
            frame: 8,
            ordinal: 0,
        };
        let f_key = InputEventId {
            frame: 8,
            ordinal: 1,
        };

        assert_eq!(
            router.collect_pointer(
                outside,
                &BTreeSet::new(),
                PointerPhase::Press(PointerButtonId::Primary),
            ),
            InputRecipient::Blocked
        );
        assert!(!router.claim_root(outside));
        assert_eq!(router.collect_keyboard(f_key), InputRecipient::Window(id));
        assert!(router.claim(f_key, InputRecipient::Window(id)));
        router.report_key(
            f_key,
            InputRecipient::Window(id),
            InputDisposition::Unhandled,
        );
        assert!(!router.root_may_handle(f_key));
    }

    #[test]
    fn status_window_stale_subject_rejects_confirm_without_retargeting() {
        let (mut world, luke, ..) = world_with_luke();
        let stale = add_character(&mut world, "Stale Character");
        let empty = Empty::new();
        let mut manager = WindowManager::new();
        let mut state = StatusWindowState::default();
        let id = opened_status(
            &mut manager,
            &mut state,
            empty.sources(&world),
            StatusObject::Character(luke),
            context(47),
        );
        world.characters.remove(stale);

        assert!(StatusWindowState::prepare_open(
            empty.sources(&world),
            StatusObject::Character(stale),
            context(48),
        )
        .is_none());
        assert_eq!(state.window_id(), Some(id));
        assert_eq!(
            state.window.as_ref().unwrap().object,
            StatusObject::Character(luke)
        );
    }

    #[test]
    fn committed_ship_index_compaction_is_unpaintable_without_retargeting() {
        let (mut world, fleet) = world_with_indexed_fleet();
        for ship in &mut world.fleets[fleet].capital_ships {
            ship.tag = 0;
        }
        let empty = Empty::new();
        let mut manager = WindowManager::new();
        let mut state = StatusWindowState::default();
        let id = opened_status(
            &mut manager,
            &mut state,
            empty.sources(&world),
            StatusObject::Ship { fleet, index: 1 },
            context(50),
        );
        assert_eq!(
            status_view(
                empty.sources(&world),
                StatusObject::Ship { fleet, index: 1 }
            )
            .unwrap()
            .name,
            "Ship B"
        );
        assert_eq!(
            managed_status_content(&manager, &state, empty.sources(&world)),
            ContentStatus::Ready
        );

        world.fleets[fleet].capital_ships.remove(0);
        assert_eq!(
            status_view(
                empty.sources(&world),
                StatusObject::Ship { fleet, index: 1 }
            )
            .unwrap()
            .name,
            "Ship C",
            "the raw index demonstrates the stale-index retarget hazard"
        );
        let frame = manager.begin_frame();

        assert_eq!(
            state.content_reports(&frame, empty.sources(&world)),
            [ContentReport {
                window: id,
                status: ContentStatus::Unpaintable,
            }]
        );
        assert_eq!(
            manager
                .reconcile(
                    &frame,
                    &state.content_reports(&frame, empty.sources(&world))
                )
                .blocking_unpainted,
            [id]
        );
        assert_eq!(state.window_id(), Some(id));
    }

    #[test]
    fn committed_fighter_index_compaction_is_unpaintable_without_retargeting() {
        let (mut world, fleet) = world_with_indexed_fleet();
        let empty = Empty::new();
        let mut manager = WindowManager::new();
        let mut state = StatusWindowState::default();
        let id = opened_status(
            &mut manager,
            &mut state,
            empty.sources(&world),
            StatusObject::Fighter { fleet, index: 1 },
            context(51),
        );
        assert_eq!(
            status_view(
                empty.sources(&world),
                StatusObject::Fighter { fleet, index: 1 }
            )
            .unwrap()
            .name,
            "Fighter B"
        );
        assert_eq!(
            managed_status_content(&manager, &state, empty.sources(&world)),
            ContentStatus::Ready
        );

        world.fleets[fleet].fighters.remove(0);
        assert_eq!(
            status_view(
                empty.sources(&world),
                StatusObject::Fighter { fleet, index: 1 }
            )
            .unwrap()
            .name,
            "Fighter C",
            "the raw index demonstrates the stale-index retarget hazard"
        );
        let frame = manager.begin_frame();

        assert_eq!(
            state.content_reports(&frame, empty.sources(&world)),
            [ContentReport {
                window: id,
                status: ContentStatus::Unpaintable,
            }]
        );
        assert_eq!(
            manager
                .reconcile(
                    &frame,
                    &state.content_reports(&frame, empty.sources(&world))
                )
                .blocking_unpainted,
            [id]
        );
        assert_eq!(state.window_id(), Some(id));
    }

    #[test]
    fn committed_ship_requires_same_roster_living_hull_and_tag() {
        let (mut world, fleet) = world_with_indexed_fleet();
        let empty = Empty::new();
        let mut manager = WindowManager::new();
        let mut state = StatusWindowState::default();
        opened_status(
            &mut manager,
            &mut state,
            empty.sources(&world),
            StatusObject::Ship { fleet, index: 1 },
            context(52),
        );

        world.fleets[fleet].capital_ships[1].alive = false;
        assert_eq!(
            managed_status_content(&manager, &state, empty.sources(&world)),
            ContentStatus::Unpaintable
        );
        world.fleets[fleet].capital_ships[1].alive = true;

        let extra = world.fleets[fleet].capital_ships[2].clone();
        world.fleets[fleet].capital_ships.push(extra);
        assert_eq!(
            managed_status_content(&manager, &state, empty.sources(&world)),
            ContentStatus::Unpaintable,
            "a changed roster invalidates even when the selected index is unchanged"
        );
        world.fleets[fleet].capital_ships.pop();

        world.fleets[fleet].capital_ships[1].tag = 999;
        assert_eq!(
            managed_status_content(&manager, &state, empty.sources(&world)),
            ContentStatus::Unpaintable
        );
    }

    #[test]
    fn committed_fighter_requires_positive_count_same_class_and_carrier() {
        let (mut world, fleet) = world_with_indexed_fleet();
        let empty = Empty::new();
        let mut manager = WindowManager::new();
        let mut state = StatusWindowState::default();
        opened_status(
            &mut manager,
            &mut state,
            empty.sources(&world),
            StatusObject::Fighter { fleet, index: 1 },
            context(53),
        );

        world.fleets[fleet].fighters[1].count = 0;
        assert_eq!(
            managed_status_content(&manager, &state, empty.sources(&world)),
            ContentStatus::Unpaintable
        );
        world.fleets[fleet].fighters[1].count = 1;

        let other_class = world.fleets[fleet].fighters[2].class;
        let original_class = world.fleets[fleet].fighters[1].class;
        world.fleets[fleet].fighters[1].class = other_class;
        assert_eq!(
            managed_status_content(&manager, &state, empty.sources(&world)),
            ContentStatus::Unpaintable
        );
        world.fleets[fleet].fighters[1].class = original_class;

        world.fleets[fleet].fighters[1].carrier = 999;
        assert_eq!(
            managed_status_content(&manager, &state, empty.sources(&world)),
            ContentStatus::Unpaintable
        );
    }

    #[test]
    fn indexed_status_preflight_rejects_dead_or_empty_entries() {
        let (mut world, fleet) = world_with_indexed_fleet();
        let empty = Empty::new();
        world.fleets[fleet].capital_ships[1].alive = false;
        world.fleets[fleet].fighters[1].count = 0;

        assert!(StatusWindowState::prepare_open(
            empty.sources(&world),
            StatusObject::Ship { fleet, index: 1 },
            context(54),
        )
        .is_none());
        assert!(StatusWindowState::prepare_open(
            empty.sources(&world),
            StatusObject::Fighter { fleet, index: 1 },
            context(55),
        )
        .is_none());
    }

    #[test]
    fn committed_unpaintable_status_stays_modal_for_recovery_close() {
        let (mut world, luke, ..) = world_with_luke();
        let empty = Empty::new();
        let mut manager = WindowManager::new();
        let mut state = StatusWindowState::default();
        let id = opened_status(
            &mut manager,
            &mut state,
            empty.sources(&world),
            StatusObject::Character(luke),
            context(49),
        );
        world.characters.remove(luke);
        let frame = manager.begin_frame();

        assert_eq!(
            state.content_reports(&frame, empty.sources(&world)),
            [ContentReport {
                window: id,
                status: ContentStatus::Unpaintable,
            }]
        );
        let reconciliation = manager.reconcile(
            &frame,
            &state.content_reports(&frame, empty.sources(&world)),
        );
        assert_eq!(reconciliation.blocking_unpainted, [id]);
        assert!(state.is_open(), "recovery close remains available");
        assert_eq!(
            click_managed(
                &world,
                &mut state,
                &frame,
                (CLOSE_RECT.0 + 16.0, CLOSE_RECT.1 + 15.0),
            ),
            Some((id, StatusWindowAction::Close))
        );
        assert!(
            state.is_open(),
            "recovery close waits for the manager receipt"
        );
    }

    fn pairs(rows: &[StatusRow]) -> Vec<(&str, &str)> {
        rows.iter()
            .map(|row| (row.label.as_str(), row.value.as_str()))
            .collect()
    }

    #[test]
    fn a_character_at_a_system_lists_the_rows_of_figure_3_46() {
        // FUN_004486f0, in order; manual p. 101, Fig. 3.46 (Luke at Yavin,
        // Awaiting Orders, Trainee, 75/75/135/70, no R&D, Admiral Yes).
        let (world, key, ..) = world_with_luke();
        let rows = character_rows(&world, &[], key).expect("rows");
        assert_eq!(
            pairs(&rows),
            [
                ("Commanding:", "None"),
                ("Attached: ", "Yavin"),
                ("Status:", "Awaiting Orders"),
                ("Force Ranking:", "Trainee"),
                ("Diplomacy Rating:", "75"),
                ("Espionage Rating:", "75"),
                ("Combat Rating:", "135"),
                ("Leadership Rating:", "70"),
                ("R&D Capabilities", ""),
                (" Ship Design", "No"),
                (" Troop Training", "No"),
                (" Facility Design", "No"),
                ("Possible Command Ranks", ""),
                ("Admiral:", "Yes"),
                ("General:", "No"),
                ("Commander:", "Yes"),
            ]
        );
    }

    #[test]
    fn a_skill_above_zero_is_an_r_and_d_capability() {
        // FUN_004486f0: +0x80, +0x82 and +0x84 show Yes when non-zero.
        let (mut world, key, ..) = world_with_luke();
        world.characters[key].troop_training = skill(1);
        let rows = character_rows(&world, &[], key).expect("rows");
        let value = |label: &str| {
            rows.iter()
                .find(|row| row.label == label)
                .map(|row| row.value.clone())
        };
        assert_eq!(value(" Troop Training").as_deref(), Some("Yes"));
        assert_eq!(value(" Ship Design").as_deref(), Some("No"));
    }

    #[test]
    fn a_character_on_its_way_is_enroute_with_its_destination_and_arrival_day() {
        // FUN_004486f0: +0x50 bit 4 first, then FUN_004fd2b0's "ETA
        // Destination:" and "Day " + the arrival; Attached names the
        // container, where an en route object already stands
        // (move-order.md).
        let (mut world, key, _, hoth) = world_with_luke();
        world.characters[key].current_system = Some(hoth);
        world.characters[key].en_route = true;
        world.characters[key].on_mission = true;
        let transit = MemberTransit {
            member: MissionMember::Character(key),
            mission_id: 3,
            arrival: 42,
        };
        let rows = character_rows(&world, &[transit], key).expect("rows");
        assert_eq!(
            pairs(&rows[..4]),
            [
                ("Commanding:", "None"),
                ("Attached: ", "Hoth"),
                ("Status:", "Enroute"),
                ("ETA Destination:", "Day 42"),
            ]
        );
        assert_eq!(rows[4].label, "Force Ranking:");
    }

    #[test]
    fn the_status_word_takes_the_first_state_that_holds() {
        // FUN_004486f0: captured before a mission; a hidden mission
        // (FUN_00520b70) reads as Awaiting Orders.
        let (mut world, key, ..) = world_with_luke();
        let status = |world: &GameWorld| {
            character_rows(world, &[], key).expect("rows")[2]
                .value
                .clone()
        };
        world.characters[key].on_mission = true;
        assert_eq!(status(&world), "On Mission");
        world.characters[key].on_hidden_mission = true;
        assert_eq!(status(&world), "Awaiting Orders");
        world.characters[key].on_hidden_mission = false;
        world.characters[key].is_captive = true;
        assert_eq!(status(&world), "Captured");
    }

    #[test]
    fn force_ranking_follows_the_force_value_thresholds() {
        // FUN_004486f0: <10, <20, <80, <100, <120, else Jedi Master.
        for (value, word) in [
            (0, "None"),
            (9, "None"),
            (10, "Novice"),
            (19, "Novice"),
            (20, "Trainee"),
            (79, "Trainee"),
            (80, "Jedi Student"),
            (99, "Jedi Student"),
            (100, "Jedi Knight"),
            (119, "Jedi Knight"),
            (120, "Jedi Master"),
        ] {
            assert_eq!(force_ranking(value), word, "{value}");
        }
    }

    #[test]
    fn the_background_follows_the_objects_side_and_the_players() {
        // FUN_00443130: 0x2d22 + side, plus 3 for a player not of side 1.
        assert_eq!(background(0, CockpitFaction::Alliance), 11_554);
        assert_eq!(background(1, CockpitFaction::Alliance), 11_555);
        assert_eq!(background(2, CockpitFaction::Alliance), 11_556);
        assert_eq!(background(0, CockpitFaction::Empire), 11_557);
        assert_eq!(background(1, CockpitFaction::Empire), 11_558);
        assert_eq!(background(2, CockpitFaction::Empire), 11_559);
    }

    #[test]
    fn a_characters_picture_is_its_portrait_its_mini_less_0x4000() {
        // FUN_0042c3b0(.., 1, 1): class +0x30 & 0xfff. Luke's TEXTSTRA id
        // is 0x2842, so his mini is 18498 and his portrait 2114.
        let (world, key, _, _) = world_with_luke();
        let view = Empty::new()
            .view(&world, StatusObject::Character(key))
            .unwrap();
        assert_eq!(view.picture, Some((DllSource::Gokres, 2_114)));
        assert_eq!(view.title, "Character Status");
    }

    /// The other state the fillers read, empty.
    pub(crate) struct Empty {
        missions: rebellion_core::missions::MissionState,
        movement: rebellion_core::movement::MovementState,
        transport: rebellion_core::troop_transport::TroopTransportState,
        manufacturing: rebellion_core::manufacturing::ManufacturingState,
    }

    impl Empty {
        pub(crate) fn new() -> Self {
            Self {
                missions: rebellion_core::missions::MissionState::new(),
                movement: rebellion_core::movement::MovementState::new(),
                transport: rebellion_core::troop_transport::TroopTransportState::new(),
                manufacturing: rebellion_core::manufacturing::ManufacturingState::new(),
            }
        }

        pub(crate) fn sources<'a>(&'a self, world: &'a GameWorld) -> StatusSources<'a> {
            StatusSources {
                world,
                missions: &self.missions,
                movement: &self.movement,
                transport: &self.transport,
                manufacturing: &self.manufacturing,
                today: 0,
            }
        }

        fn view(
            &self,
            world: &GameWorld,
            object: StatusObject,
        ) -> Option<crate::status_rows::StatusView> {
            crate::status_rows::status_view(self.sources(world), object)
        }
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

    /// Run two idle frames, then one per entry of `frames`, with the pointer
    /// at window pixel `(x, y)`.
    fn drive(
        world: &GameWorld,
        state: &mut StatusWindowState,
        (x, y): (f32, f32),
        frames: Vec<Vec<egui::Event>>,
    ) -> Option<StatusWindowAction> {
        let layout = layout();
        let pos = window_rect(layout).min + egui::vec2(x, y);
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let empty = Empty::new();
        let mut emitted = None;
        for extra in [vec![], vec![]].into_iter().chain(frames) {
            let mut events = vec![egui::Event::PointerMoved(pos)];
            events.extend(extra);
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(700.0, 520.0),
                )),
                events,
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                if let Some(action) = draw_status_window(
                    ctx,
                    empty.sources(world),
                    state,
                    CockpitFaction::Alliance,
                    layout,
                    &mut cache,
                ) {
                    emitted = Some(action);
                }
            });
        }
        emitted
    }

    fn click(
        world: &GameWorld,
        state: &mut StatusWindowState,
        point: (f32, f32),
    ) -> Option<StatusWindowAction> {
        let pos = window_rect(layout()).min + egui::vec2(point.0, point.1);
        drive(
            world,
            state,
            point,
            vec![vec![press(pos, true)], vec![press(pos, false)]],
        )
    }

    fn click_managed(
        world: &GameWorld,
        state: &mut StatusWindowState,
        frame: &RoutingSnapshot,
        point: (f32, f32),
    ) -> Option<(WindowId, StatusWindowAction)> {
        let layout = layout();
        let pos = window_rect(layout).min + egui::vec2(point.0, point.1);
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let empty = Empty::new();
        let mut emitted = None;
        for extra in [
            vec![],
            vec![],
            vec![press(pos, true)],
            vec![press(pos, false)],
        ] {
            let mut events = vec![egui::Event::PointerMoved(pos)];
            events.extend(extra);
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(700.0, 520.0),
                )),
                events,
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                if let Some(action) = draw_managed_status_window(
                    ctx,
                    empty.sources(world),
                    state,
                    CockpitFaction::Alliance,
                    layout,
                    &mut cache,
                    frame,
                ) {
                    emitted = Some(action);
                }
            });
        }
        emitted
    }

    #[test]
    fn managed_status_reasserts_its_layer_after_lower_surfaces_raise() {
        let (world, key, ..) = world_with_luke();
        let empty = Empty::new();
        let mut manager = WindowManager::new();
        let mut state = StatusWindowState::default();
        opened_status(
            &mut manager,
            &mut state,
            empty.sources(&world),
            StatusObject::Character(key),
            context(40),
        );
        let frame = manager.begin_frame();
        let layout = layout();
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let lower = egui::Id::new("raising-lower-managed-surface");

        for _ in 0..3 {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(700.0, 520.0),
                )),
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                // A lower managed surface reasserts itself before Status in
                // the manager's bottom-to-top paint sequence.
                ctx.move_to_top(egui::LayerId::new(egui::Order::Tooltip, lower));
                egui::Area::new(lower)
                    .order(egui::Order::Tooltip)
                    .fixed_pos(window_rect(layout).min)
                    .show(ctx, |ui| {
                        ui.allocate_response(window_rect(layout).size(), egui::Sense::click());
                    });
                let _ = draw_managed_status_window(
                    ctx,
                    empty.sources(&world),
                    &mut state,
                    CockpitFaction::Alliance,
                    layout,
                    &mut cache,
                    &frame,
                );
            });
        }

        assert_eq!(
            ctx.layer_id_at(window_rect(layout).center())
                .map(|layer| layer.id),
            Some(egui::Id::new("original-status-window"))
        );
    }

    #[test]
    fn the_buttons_open_the_encyclopedia_or_close_and_both_close_the_window() {
        // FUN_00443020: 0x66 at (258, 218), 0x65 at (324, 218);
        // FUN_004443a0 closes (+0x30) for both.
        let (world, key, ..) = world_with_luke();
        for (point, expected) in [
            (
                (258.0 + 16.0, 218.0 + 15.0),
                StatusWindowAction::Encyclopedia,
            ),
            ((324.0 + 16.0, 218.0 + 15.0), StatusWindowAction::Close),
        ] {
            let mut state = StatusWindowState::default();
            state.open_character(key);
            assert_eq!(click(&world, &mut state, point), Some(expected));
            assert!(!state.is_open());
        }
    }

    #[test]
    fn a_click_beside_the_buttons_keeps_the_window_open() {
        let (world, key, ..) = world_with_luke();
        for point in [(257.0, 230.0), (300.0, 230.0), (340.0, 249.5)] {
            let mut state = StatusWindowState::default();
            state.open_character(key);
            assert_eq!(click(&world, &mut state, point), None, "{point:?}");
            assert!(state.is_open());
        }
    }

    #[test]
    fn escape_closes_the_window() {
        let (world, key, ..) = world_with_luke();
        let mut state = StatusWindowState::default();
        state.open_character(key);
        let escape = egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        };
        assert_eq!(
            drive(&world, &mut state, (0.0, 0.0), vec![vec![escape]]),
            Some(StatusWindowAction::Close)
        );
        assert!(!state.is_open());
    }

    #[test]
    fn the_window_paints_its_background_and_takes_the_pointer_only_over_itself() {
        // FUN_00442d70: 379 by 272; the side-1 character's background for a
        // side-1 player is 11554.
        let (world, key, ..) = world_with_luke();
        let mut state = StatusWindowState::default();
        state.open_character(key);
        crate::fleet_window::tests::PAINTED.with(|painted| painted.borrow_mut().clear());
        let _ = drive(&world, &mut state, (0.0, 0.0), vec![]);
        let rect = window_rect(layout());
        let painted = crate::fleet_window::tests::PAINTED.with(|painted| painted.borrow().clone());
        assert!(painted.contains(&(11_554, rect.min)), "{painted:?}");
        assert!(painted.iter().any(|(id, _)| *id == ENCYCLOPEDIA.0));
        assert!(painted.iter().any(|(id, _)| *id == CLOSE.0));

        assert_eq!(
            rect.size(),
            egui::vec2(STATUS_WINDOW_WIDTH, STATUS_WINDOW_HEIGHT)
        );
        assert!(state.contains_screen_point(layout(), (rect.min.x, rect.min.y)));
        assert!(!state.contains_screen_point(layout(), (rect.min.x - 1.0, rect.min.y)));
        assert!(
            !StatusWindowState::default().contains_screen_point(layout(), (rect.min.x, rect.min.y))
        );
    }

    #[test]
    fn a_shorter_cell_sits_at_the_bottom_of_its_row_and_rows_stack() {
        // FUN_00449e00: the row advances by its taller cell; a value shorter
        // than its label moves down by the difference, a taller one does not
        // move the label.
        let (tops, height) = row_tops(&[(20.0, 10.0), (10.0, 30.0), (5.0, 5.0)]);
        assert_eq!(tops, [(0.0, 10.0), (20.0, 20.0), (50.0, 50.0)]);
        assert_eq!(height, 55.0);
    }

    #[test]
    fn the_picture_centres_on_307_64_with_integer_halves() {
        // FUN_00443130: (0x133 - w / 2, 0x40 - h / 2).
        assert_eq!(picture_origin(80, 80), (267.0, 24.0));
        assert_eq!(picture_origin(81, 79), (267.0, 25.0));
    }

    #[test]
    fn the_wheel_scrolls_the_list_only_as_far_as_its_rows_reach() {
        // port: the list's 204 rows of height scroll; at scale 2, 500 screen
        // pixels of rows overflow it by 46 window pixels.
        assert_eq!(scrolled(0.0, -20.0, 2.0, 500.0), 10.0);
        assert_eq!(scrolled(10.0, 8.0, 2.0, 500.0), 6.0);
        assert_eq!(scrolled(40.0, -200.0, 2.0, 500.0), 46.0);
        assert_eq!(scrolled(5.0, 100.0, 2.0, 500.0), 0.0);
        assert_eq!(scrolled(5.0, -100.0, 2.0, 300.0), 0.0);
    }

    #[test]
    fn a_side_past_the_third_takes_the_other_background() {
        assert_eq!(background(3, CockpitFaction::Alliance), 11_556);
        assert_eq!(background(3, CockpitFaction::Empire), 11_559);
    }

    fn scaled() -> CockpitLayout {
        CockpitLayout {
            canvas: CockpitViewport {
                x: 20.0,
                y: 40.0,
                width: 1280.0,
                height: 960.0,
            },
            galaxy: CockpitViewport {
                x: 130.0,
                y: 120.0,
                width: 970.0,
                height: 700.0,
            },
            scale: 2.0,
        }
    }

    /// A drawn text: its string, top-left, size and font size.
    type Text = (String, egui::Pos2, egui::Vec2, f32);

    /// Two frames at scale 2; the last frame's text shapes as (text,
    /// top-left, size, font size), and the bitmaps painted.
    fn render(
        world: &GameWorld,
        state: &mut StatusWindowState,
    ) -> (Vec<Text>, Vec<(u32, egui::Pos2)>) {
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        cache.set_base_path(staged_portrait());
        let empty = Empty::new();
        let mut output = None;
        crate::fleet_window::tests::PAINTED.with(|painted| painted.borrow_mut().clear());
        for _ in 0..2 {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1400.0, 1100.0),
                )),
                ..Default::default()
            };
            output = Some(ctx.run(input, |ctx| {
                let _ = draw_status_window(
                    ctx,
                    empty.sources(world),
                    state,
                    CockpitFaction::Alliance,
                    scaled(),
                    &mut cache,
                );
            }));
        }
        fn collect(shape: &egui::Shape, out: &mut Vec<Text>) {
            match shape {
                egui::Shape::Text(text) => out.push((
                    text.galley.text().to_owned(),
                    text.pos,
                    text.galley.size(),
                    text.galley
                        .job
                        .sections
                        .first()
                        .map_or(0.0, |section| section.format.font_id.size),
                )),
                egui::Shape::Vec(shapes) => {
                    for shape in shapes {
                        collect(shape, out);
                    }
                }
                _ => {}
            }
        }
        let mut texts = Vec::new();
        for clipped in &output.expect("a frame").shapes {
            collect(&clipped.shape, &mut texts);
        }
        let painted = crate::fleet_window::tests::PAINTED.with(|painted| painted.borrow().clone());
        (texts, painted)
    }

    /// A staged 80 by 80 GOKRES 2114, Luke's portrait's size, in a fresh
    /// directory.
    fn staged_portrait() -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!("status-portrait-{}", std::process::id()));
        let dir = root.join("gokres-dll/BMP");
        std::fs::create_dir_all(&dir).unwrap();
        // An 8-bit indexed BMP, as the original resources are.
        let (side, offset) = (80usize, 54usize + 1024);
        let mut bmp = vec![1_u8; offset + side * side];
        bmp[..2].copy_from_slice(b"BM");
        bmp[10..14].copy_from_slice(&u32::try_from(offset).unwrap().to_le_bytes());
        bmp[14..18].copy_from_slice(&40u32.to_le_bytes());
        bmp[18..22].copy_from_slice(&i32::try_from(side).unwrap().to_le_bytes());
        bmp[22..26].copy_from_slice(&i32::try_from(side).unwrap().to_le_bytes());
        bmp[26..28].copy_from_slice(&1u16.to_le_bytes());
        bmp[28..30].copy_from_slice(&8u16.to_le_bytes());
        bmp[30..54].fill(0);
        std::fs::write(dir.join("2114.bmp"), bmp).unwrap();
        root
    }

    fn find<'a>(texts: &'a [Text], text: &str) -> &'a Text {
        texts
            .iter()
            .find(|(found, ..)| found == text)
            .unwrap_or_else(|| panic!("{text} not drawn in {texts:?}"))
    }

    fn near(a: egui::Pos2, b: egui::Pos2) -> bool {
        (a - b).length() < 0.01
    }

    #[test]
    fn the_title_name_and_rows_sit_where_the_window_lays_them_out() {
        // FUN_00443130 at scale 2: the title centred in (15, 18, 211, 18),
        // the name centred in (242, 137, 130, 44), labels at the list's
        // (18, 47) and values 103 to their right (FUN_00449e00). Fonts 5
        // and 10 are 16 and 14 pixels (FUN_0060eed0).
        let (world, key, ..) = world_with_luke();
        let mut state = StatusWindowState::default();
        state.open_character(key);
        let (texts, _) = render(&world, &mut state);
        let frame = window_rect(scaled()).min;
        let at = |x: f32, y: f32| frame + egui::vec2(x, y) * 2.0;

        let (_, pos, size, font) = find(&texts, "Character Status");
        assert!(
            near(*pos + *size / 2.0, at(15.0 + 105.5, 18.0 + 9.0)),
            "{pos:?} {size:?}"
        );
        assert_eq!(*font, crate::theme::game_font_size(5) * 2.0);
        let (_, pos, size, font) = find(&texts, "Luke Skywalker");
        assert!(
            near(*pos + *size / 2.0, at(242.0 + 65.0, 137.0 + 22.0)),
            "{pos:?} {size:?}"
        );
        assert_eq!(*font, crate::theme::game_font_size(5) * 2.0);

        let (_, commanding, row, font) = find(&texts, "Commanding:");
        assert!(near(*commanding, at(18.0, 47.0)), "{commanding:?}");
        assert_eq!(*font, crate::theme::game_font_size(10) * 2.0);
        let (_, none, ..) = find(&texts, "None");
        assert!(near(*none, at(18.0 + 103.0, 47.0)), "{none:?}");
        let (_, yavin, ..) = find(&texts, "Yavin");
        assert!(near(*yavin, *none + egui::vec2(0.0, row.y)), "{yavin:?}");
        let (_, attached, ..) = find(&texts, "Attached: ");
        assert!(
            near(*attached, *commanding + egui::vec2(0.0, row.y)),
            "{attached:?}"
        );
    }

    #[test]
    fn a_scrolled_list_draws_its_rows_higher() {
        // A name that wraps over many lines pushes the rows past the list.
        let (mut world, key, yavin, _) = world_with_luke();
        world.systems[yavin].name = "Yavin ".repeat(40);
        let mut state = StatusWindowState::default();
        state.open_character(key);
        if let Some(open) = state.window.as_mut() {
            open.scroll = 10.0;
        }
        let (texts, _) = render(&world, &mut state);
        let frame = window_rect(scaled()).min;
        let (_, commanding, ..) = find(&texts, "Commanding:");
        assert!(
            near(*commanding, frame + egui::vec2(18.0, 47.0 - 10.0) * 2.0),
            "{commanding:?}"
        );
    }

    #[test]
    fn the_portrait_and_the_sides_background_are_painted() {
        // FUN_00443130: Luke's 80 by 80 portrait at (267, 24); an Empire
        // character's background 11555 and a neutral one's 11556 for a
        // side-1 player. Reads the staged GOKRES and STRATEGY bitmaps.
        let (mut world, key, ..) = world_with_luke();
        let mut state = StatusWindowState::default();
        state.open_character(key);
        let frame = window_rect(scaled()).min;
        let (_, painted) = render(&world, &mut state);
        assert!(painted.contains(&(11_554, frame)), "{painted:?}");
        assert!(
            painted.contains(&(2_114, frame + egui::vec2(267.0, 24.0) * 2.0)),
            "{painted:?}"
        );

        world.characters[key].is_alliance = false;
        world.characters[key].is_empire = true;
        let (_, painted) = render(&world, &mut state);
        assert!(painted.contains(&(11_555, frame)), "{painted:?}");
        world.characters[key].is_empire = false;
        let (_, painted) = render(&world, &mut state);
        assert!(painted.contains(&(11_556, frame)), "{painted:?}");
    }

    #[test]
    fn a_window_for_a_character_that_is_gone_closes() {
        let (mut world, key, ..) = world_with_luke();
        let mut state = StatusWindowState::default();
        state.open_character(key);
        world.characters.remove(key);
        let _ = drive(&world, &mut state, (0.0, 0.0), vec![]);
        assert!(!state.is_open());
    }
}
