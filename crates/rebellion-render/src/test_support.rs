//! Shared fixtures for renderer unit tests.
//!
//! These helpers keep strategic-window tests focused on the behavior they
//! exercise instead of repeating world, layout, and input-frame construction.

use std::collections::BTreeSet;

use egui_macroquad::egui;
use rebellion_core::dat::{Faction, SectorGroup};
use rebellion_core::ids::{CharacterKey, DatId, FleetKey, SpecialForceKey, SystemKey};
use rebellion_core::missions::{MissionFaction, MissionKind, MissionRequest, MissionState};
use rebellion_core::saved_interface::SavedWindow;
use rebellion_core::test_support::add_fleet_with_class;
use rebellion_core::world::{
    CapitalShipClass, Character, ControlKind, GameWorld, Sector, SpecialForceUnit,
};
use rebellion_windowing::{
    ContentReport, ContentStatus, ControlId, ExistingEffect, FocusOnOpen, FocusTarget, InputPolicy,
    RouteTarget, RoutingSnapshot, SubjectId, WindowCommand, WindowId, WindowKey, WindowKind,
    WindowManager, WindowOwner, WindowPolicy, WindowSpec, WindowTransition,
};

use crate::cockpit::{CockpitLayout, CockpitViewport};
use crate::strategic_window_common;
use crate::windowing_input::{
    FrameInputRouter, InputDisposition, InputEventId, InputRecipient, PointerButtonId, PointerPhase,
};

thread_local! {
    /// Bitmap resource requests recorded while a textureless renderer fixture runs.
    static PAINTED: std::cell::RefCell<Vec<(u32, egui::Pos2)>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

pub(crate) fn record_painted(resource_id: u32, position: egui::Pos2) {
    PAINTED.with(|painted| painted.borrow_mut().push((resource_id, position)));
}

pub(crate) fn clear_painted_trace() {
    PAINTED.with(|painted| painted.borrow_mut().clear());
}

pub(crate) fn take_painted_trace() -> Vec<(u32, egui::Pos2)> {
    PAINTED.with(std::cell::RefCell::take)
}

pub(crate) fn painted_trace() -> Vec<(u32, egui::Pos2)> {
    PAINTED.with(|painted| painted.borrow().clone())
}

/// Build a minimally valid uncompressed BMP fixture with a uniform pixel
/// payload. Tests may overwrite palette entries or individual pixels after
/// construction; row padding is expressed explicitly through `stride`.
pub(crate) fn bitmap_fixture(
    width: usize,
    height: usize,
    stride: usize,
    pixel_offset: usize,
    bits_per_pixel: u16,
    pixel: u8,
) -> Vec<u8> {
    let mut bytes = vec![0; pixel_offset + stride * height];
    let file_size = u32::try_from(bytes.len()).unwrap();
    bytes[0..2].copy_from_slice(b"BM");
    bytes[2..6].copy_from_slice(&file_size.to_le_bytes());
    bytes[10..14].copy_from_slice(&u32::try_from(pixel_offset).unwrap().to_le_bytes());
    bytes[14..18].copy_from_slice(&40_u32.to_le_bytes());
    bytes[18..22].copy_from_slice(&i32::try_from(width).unwrap().to_le_bytes());
    bytes[22..26].copy_from_slice(&i32::try_from(height).unwrap().to_le_bytes());
    bytes[26..28].copy_from_slice(&1_u16.to_le_bytes());
    bytes[28..30].copy_from_slice(&bits_per_pixel.to_le_bytes());
    bytes[34..38].copy_from_slice(&u32::try_from(stride * height).unwrap().to_le_bytes());
    bytes[pixel_offset..].fill(pixel);
    bytes
}

/// The original 640 by 480 canvas and strategic galaxy viewport.
pub(crate) fn finder_layout() -> CockpitLayout {
    CockpitLayout {
        canvas: CockpitViewport {
            x: 0.0,
            y: 0.0,
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

fn full_layout_for(viewport: CockpitViewport, scale: f32) -> CockpitLayout {
    CockpitLayout {
        canvas: viewport,
        galaxy: viewport,
        scale,
    }
}

/// A full-canvas strategic layout at native scale.
pub(crate) fn full_layout() -> CockpitLayout {
    full_layout_for(
        CockpitViewport {
            x: 0.0,
            y: 0.0,
            width: 640.0,
            height: 480.0,
        },
        1.0,
    )
}

/// A full-canvas layout offset by (10, 20) and rendered at double scale.
pub(crate) fn scaled_full_layout() -> CockpitLayout {
    full_layout_for(
        CockpitViewport {
            x: 10.0,
            y: 20.0,
            width: 1280.0,
            height: 960.0,
        },
        2.0,
    )
}

/// A strategic layout with the test canvas at (10, 20), the galaxy inset by
/// (55, 40), and every dimension scaled by `scale`.
pub(crate) fn offset_strategic_layout(scale: f32) -> CockpitLayout {
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

/// A test canvas at (10, 20) with no active galaxy viewport.
pub(crate) fn offset_canvas_layout(scale: f32) -> CockpitLayout {
    CockpitLayout {
        canvas: CockpitViewport {
            x: 10.0,
            y: 20.0,
            width: 640.0 * scale,
            height: 480.0 * scale,
        },
        galaxy: CockpitViewport {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
        scale,
    }
}

/// Read one fixture-only secondary-button release and translate it through
/// the caller's logical hit test.
pub(crate) fn secondary_release_anchor(
    ctx: &egui::Context,
    layout: CockpitLayout,
    hit_test: impl FnOnce((f32, f32)) -> Option<(f32, f32)>,
) -> Option<(f32, f32)> {
    if layout.scale <= 0.0 || ctx.is_pointer_over_area() {
        return None;
    }
    let (released, pointer) = ctx.input(|input| {
        (
            input
                .pointer
                .button_released(egui::PointerButton::Secondary),
            input.pointer.interact_pos(),
        )
    });
    let pointer = pointer.filter(|_| released)?;
    hit_test((pointer.x, pointer.y))
}

/// Open a fixture-owned context menu from one secondary-button release.
/// The screen supplies only its hit/anchor mapping; this helper owns the
/// shared direct payload update used by context-menu fixtures.
pub(crate) fn open_context_menu_on_secondary_release(
    ctx: &egui::Context,
    layout: CockpitLayout,
    anchor: &mut Option<(f32, f32)>,
    hit_test: impl FnOnce((f32, f32)) -> Option<(f32, f32)>,
) -> bool {
    let Some(open_at) = secondary_release_anchor(ctx, layout, hit_test) else {
        return false;
    };
    *anchor = Some(open_at);
    true
}

/// Insert a recruited character at `system` with the requested side.
pub(crate) fn add_character(
    world: &mut GameWorld,
    system: SystemKey,
    name: impl Into<String>,
    is_alliance: bool,
) -> CharacterKey {
    world.characters.insert(Character {
        dat_id: DatId::new(832),
        name: name.into(),
        is_alliance,
        is_empire: !is_alliance,
        current_system: Some(system),
        recruited: true,
        ..Character::default()
    })
}

/// Insert one idle special-force unit and attach it to `system`.
pub(crate) fn add_special_force(
    world: &mut GameWorld,
    system: SystemKey,
    class_raw: u32,
    is_alliance: bool,
) -> SpecialForceKey {
    let key = world.special_forces.insert(SpecialForceUnit {
        class_dat_id: DatId::new(class_raw),
        is_alliance,
        skills: [0; 8],
        on_mission: false,
        en_route: false,
        on_hidden_mission: false,
    });
    world.systems[system].special_forces.push(key);
    key
}

/// Assert case-insensitive longest-prefix selection, including first-row tie
/// breaking, for any Finder row type.
pub(crate) fn assert_finder_prefix_choices<Row>(
    make_row: impl FnMut(&str) -> Row,
    choose: impl Fn(&[Row], &str) -> Option<usize>,
) {
    let rows = ["Corulag", "Coruscant", "Yavin"].map(make_row);
    assert_eq!(choose(&rows, "corus"), Some(1));
    assert_eq!(choose(&rows, "cor"), Some(0));
    assert_eq!(choose(&rows, "ya"), Some(2));
    assert_eq!(choose(&rows, "z"), None);
    assert_eq!(choose(&rows, ""), None);
}

/// Assert one optional screen rectangle without repeating its construction.
pub(crate) fn assert_screen_rect(actual: Option<egui::Rect>, min: egui::Pos2, size: egui::Vec2) {
    assert_eq!(actual, Some(egui::Rect::from_min_size(min, size)));
}

/// Assert that a rectangle hit test includes its minimum and interior while
/// excluding the right and bottom edges.
pub(crate) fn assert_exclusive_rect_edges(
    rect: egui::Rect,
    contains: impl Fn(egui::Rect, egui::Pos2) -> bool,
) {
    let interior = egui::pos2(rect.max.x - 0.001, rect.max.y - 0.001);
    assert!(contains(rect, rect.min));
    assert!(contains(rect, interior));
    assert!(!contains(rect, egui::pos2(rect.max.x, rect.min.y)));
    assert!(!contains(rect, egui::pos2(rect.min.x, rect.max.y)));
}

/// Insert a one-ship transport fleet and attach it to its system.
pub(crate) fn add_transport_fleet(
    world: &mut GameWorld,
    system: SystemKey,
    is_alliance: bool,
    troop_capacity: u32,
) -> FleetKey {
    let class = world.capital_ship_classes.insert(CapitalShipClass {
        name: "Transport".into(),
        is_alliance,
        troop_capacity,
        hull: 100,
        hyperdrive: 1,
        ..CapitalShipClass::default()
    });
    add_fleet_with_class(world, system, class, is_alliance)
}

/// Dispatch the standard Alliance diplomacy mission used by window fixtures.
pub(crate) fn send_on_mission(
    missions: &mut MissionState,
    member: CharacterKey,
    system: SystemKey,
) {
    missions.dispatch(MissionRequest::single(
        MissionKind::Diplomacy,
        MissionFaction::Alliance,
        member,
        system,
        None,
        0,
    ));
}

/// Convert one window-local logical point into screen coordinates.
pub(crate) fn window_point(
    layout: CockpitLayout,
    origin: (i16, i16),
    local: (f32, f32),
) -> egui::Pos2 {
    egui::pos2(
        layout.canvas.x + (f32::from(origin.0) + local.0) * layout.scale,
        layout.canvas.y + (f32::from(origin.1) + local.1) * layout.scale,
    )
}

#[cfg(test)]
macro_rules! define_detail_window_point {
    ($name:ident, $origin:expr) => {
        fn $name(x: f32, y: f32) -> egui_macroquad::egui::Pos2 {
            $crate::test_support::window_point(
                $crate::test_support::scaled_full_layout(),
                $origin,
                (x, y),
            )
        }
    };
}

#[cfg(test)]
pub(crate) use define_detail_window_point;

pub(crate) fn foreground_layer(id: egui::Id) -> egui::LayerId {
    egui::LayerId::new(egui::Order::Foreground, id)
}

/// Egui input for one deterministic 20 Hz renderer-test frame.
pub(crate) fn timed_raw_input(
    index: usize,
    events: Vec<egui::Event>,
    modifiers: egui::Modifiers,
    screen_size: egui::Vec2,
) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, screen_size)),
        time: Some(index as f64 * 0.05),
        events,
        modifiers,
        ..Default::default()
    }
}

pub(crate) fn raw_input(events: Vec<egui::Event>, screen_size: egui::Vec2) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, screen_size)),
        events,
        ..Default::default()
    }
}

/// Assert that an unhandled key owned by a strategic window remains blocked
/// from the root for the lifetime of the frozen routing snapshot.
pub fn assert_unhandled_window_key_blocks_root(
    snapshot: &RoutingSnapshot,
    window: WindowId,
    event: InputEventId,
) {
    let mut router = FrameInputRouter::default();
    router.begin_frame(event.frame, snapshot);
    router.collect_keyboard(event);
    assert_eq!(
        router.keyboard_recipient(event),
        InputRecipient::Window(window)
    );
    router.report_key(
        event,
        InputRecipient::Window(window),
        InputDisposition::Unhandled,
    );
    assert!(!router.claim_root(event));
}

pub(crate) fn full_raw_input(events: Vec<egui::Event>) -> egui::RawInput {
    raw_input(events, egui::vec2(1400.0, 1000.0))
}

/// One Escape key press in the standard Finder test viewport.
pub(crate) fn finder_escape_input() -> egui::RawInput {
    raw_input(vec![key_press(egui::Key::Escape)], egui::vec2(700.0, 520.0))
}

/// Build one input frame at `point`, rewriting placeholder pointer-button
/// coordinates while preserving every non-pointer event.
pub(crate) fn pointer_raw_input(
    point: egui::Pos2,
    events: Vec<egui::Event>,
    screen_size: egui::Vec2,
) -> egui::RawInput {
    let mut positioned = vec![egui::Event::PointerMoved(point)];
    positioned.extend(events.into_iter().map(|event| match event {
        egui::Event::PointerButton {
            button,
            pressed,
            modifiers,
            ..
        } => egui::Event::PointerButton {
            pos: point,
            button,
            pressed,
            modifiers,
        },
        other => other,
    }));
    raw_input(positioned, screen_size)
}

/// A focused contextual System window for low-level routing tests.
///
/// Production System opens preserve focus; these tests need a window to own
/// keyboard input immediately so they can exercise handled/unhandled routing.
pub(crate) fn routing_window_spec(subject: u64, input: InputPolicy) -> WindowSpec {
    WindowSpec {
        key: WindowKey::contextual(WindowKind::System, SubjectId::new(2, subject)),
        owner: WindowOwner::StrategicRoot,
        policy: WindowPolicy::restorable_history(input, FocusOnOpen::Window),
    }
}

pub(crate) const fn control_focus(window: WindowId, control: ControlId) -> FocusTarget {
    FocusTarget {
        window,
        control: Some(control),
    }
}

pub(crate) fn assert_finder_selection(
    actual: Option<usize>,
    actual_name: &str,
    expected: usize,
    expected_name: &str,
) {
    assert_eq!(actual, Some(expected));
    assert_eq!(actual_name, expected_name);
}

pub(crate) fn assert_painted_at(
    painted: &[(u32, egui::Pos2)],
    expected: impl IntoIterator<Item = (u32, egui::Pos2)>,
) {
    for (id, point) in expected {
        assert!(
            painted.contains(&(id, point)),
            "{id} at {point:?} in {painted:?}"
        );
    }
}

pub(crate) fn assert_detail_chrome(
    painted: &[(u32, egui::Pos2)],
    background: u32,
    title: u32,
    at: impl Fn(f32, f32) -> egui::Pos2,
) {
    assert_painted_at(
        painted,
        [
            (background, at(0.0, 0.0)),
            (title, at(2.0, 2.0)),
            (strategic_window_common::SECTOR_ART.0, at(3.0, 3.0)),
            (strategic_window_common::MINIMIZE_ART.0, at(204.0, 3.0)),
            (strategic_window_common::CLOSE_ART.0, at(218.0, 3.0)),
        ],
    );
}

#[cfg(test)]
macro_rules! assert_indexed_finder_selection {
    ($state:expr, $expected:expr, $name:expr $(,)?) => {{
        let window = $state.window.as_ref().expect("open Finder fixture");
        $crate::test_support::assert_finder_selection(
            window.chosen,
            &window.ui.name,
            $expected,
            $name,
        );
    }};
}

#[cfg(test)]
pub(crate) use assert_indexed_finder_selection;

pub(crate) fn assert_existing_noop(transition: &WindowTransition, expected: WindowId) {
    assert!(matches!(
        transition,
        WindowTransition::Existing {
            id,
            effect: ExistingEffect::NoOp,
        } if *id == expected
    ));
}

pub(crate) fn commit_replacement(
    manager: &mut WindowManager,
    old: WindowId,
    new: WindowSpec,
    name: &str,
) -> (WindowTransition, WindowId) {
    let committed = manager
        .apply(WindowCommand::Replace { old, new })
        .unwrap_or_else(|error| panic!("fresh {name} context replaces atomically: {error:?}"));
    let new = match committed.transition {
        WindowTransition::Replaced { new, .. } => new,
        ref other => panic!("expected {name} replacement, got {other:?}"),
    };
    (committed.transition, new)
}

#[cfg(test)]
macro_rules! commit_prepared_replacement {
    ($manager:expr, $state:expr, $old:expr, $prepared:expr, $name:literal) => {{
        let prepared = $prepared;
        let (transition, new) = $crate::test_support::commit_replacement(
            $manager,
            $old,
            prepared.spec($crate::windowing_policy::command_context_subject(3)),
            $name,
        );
        assert_eq!($state.commit_open(prepared, &transition), Some(new));
        new
    }};
}

#[cfg(test)]
pub(crate) use commit_prepared_replacement;

/// Borrow status rows as label/value pairs for compact fixture assertions.
pub(crate) fn status_pairs(rows: &[crate::status_window::StatusRow]) -> Vec<(&str, &str)> {
    rows.iter()
        .map(|row| (row.label.as_str(), row.value.as_str()))
        .collect()
}

/// Assert the common closed/open/outside hit contract for a Finder state.
pub(crate) fn assert_finder_rectangle<S>(
    state: &mut S,
    layout: CockpitLayout,
    open: impl FnOnce(&mut S),
    contains: impl Fn(&S, (f32, f32)) -> bool,
) -> egui::Rect {
    let rect = crate::finder_common::window_rect(layout);
    let center = (rect.center().x, rect.center().y);
    assert!(!contains(state, center));
    open(state);
    assert!(contains(state, center));
    assert!(!contains(state, (rect.max.x + 1.0, rect.center().y)));
    rect
}

pub(crate) trait FinderFixtureState {
    fn open_fixture(&mut self);
    fn contains_fixture(&self, layout: CockpitLayout, point: (f32, f32)) -> bool;
    fn screen_rect_fixture(&self, layout: CockpitLayout) -> Option<egui::Rect>;
    fn hit_window_fixture(
        &self,
        frame: &RoutingSnapshot,
        layout: CockpitLayout,
        point: (f32, f32),
    ) -> Option<WindowId>;
    fn content_reports_fixture(&self, frame: &RoutingSnapshot) -> Vec<ContentReport>;
}

macro_rules! impl_finder_fixture_state {
    ($($state:path),+ $(,)?) => {
        $(
            impl FinderFixtureState for $state {
                fn open_fixture(&mut self) {
                    self.open(crate::cockpit::CockpitFaction::Alliance);
                }

                fn contains_fixture(
                    &self,
                    layout: CockpitLayout,
                    point: (f32, f32),
                ) -> bool {
                    self.contains_screen_point(layout, point)
                }

                fn screen_rect_fixture(&self, layout: CockpitLayout) -> Option<egui::Rect> {
                    self.screen_rect(layout)
                }

                fn hit_window_fixture(
                    &self,
                    frame: &RoutingSnapshot,
                    layout: CockpitLayout,
                    point: (f32, f32),
                ) -> Option<WindowId> {
                    self.hit_window(frame, layout, point)
                }

                fn content_reports_fixture(
                    &self,
                    frame: &RoutingSnapshot,
                ) -> Vec<ContentReport> {
                    self.content_reports(frame)
                }
            }
        )+
    };
}

impl_finder_fixture_state!(
    crate::fleet_finder::FleetFinderState,
    crate::personnel_finder::PersonnelFinderState,
    crate::troop_finder::TroopFinderState,
);

/// Assert the common closed/open/outside hit contract for any Finder family.
pub(crate) fn assert_finder_bounds<S: FinderFixtureState>(
    state: &mut S,
    layout: CockpitLayout,
) -> egui::Rect {
    assert_finder_rectangle(
        state,
        layout,
        FinderFixtureState::open_fixture,
        |state, point| state.contains_fixture(layout, point),
    )
}

/// Assert the shared modal routing contract at the center of a managed Finder.
pub(crate) fn assert_finder_modal_routing<S: FinderFixtureState>(
    manager: &WindowManager,
    state: &S,
    id: WindowId,
    layout: CockpitLayout,
) -> (RoutingSnapshot, egui::Pos2) {
    let frame = manager.begin_frame();
    let inside = state.screen_rect_fixture(layout).unwrap().center();
    let hit = state.hit_window_fixture(&frame, layout, (inside.x, inside.y));
    assert_modal_window_routing(&frame, id, hit, &state.content_reports_fixture(&frame));
    (frame, inside)
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RenderedText {
    pub(crate) text: String,
    pub(crate) pos: egui::Pos2,
    pub(crate) color: egui::Color32,
    pub(crate) font: f32,
    pub(crate) size: egui::Vec2,
    pub(crate) wrap: f32,
    pub(crate) rect: egui::Rect,
}

/// Extract the text shapes emitted by one egui frame.
pub(crate) fn rendered_texts(output: egui::FullOutput) -> Vec<RenderedText> {
    output
        .shapes
        .into_iter()
        .filter_map(|clipped| {
            let egui::Shape::Text(text) = clipped.shape else {
                return None;
            };
            let format = &text.galley.job.sections[0].format;
            Some(RenderedText {
                text: text.galley.text().to_owned(),
                pos: text.pos,
                color: format.color,
                font: format.font_id.size,
                size: text.galley.size(),
                wrap: text.galley.job.wrap.max_width,
                rect: text.galley.rect,
            })
        })
        .collect()
}

#[derive(Debug)]
pub(crate) struct RenderRun<Action> {
    pub(crate) actions: Vec<Action>,
    pub(crate) texts: Vec<RenderedText>,
    pub(crate) painted: Vec<(u32, egui::Pos2)>,
}

impl<Action> Default for RenderRun<Action> {
    fn default() -> Self {
        Self {
            actions: Vec::new(),
            texts: Vec::new(),
            painted: Vec::new(),
        }
    }
}

impl<Action> RenderRun<Action> {
    pub(crate) fn text(&self, text: &str) -> &RenderedText {
        self.texts
            .iter()
            .find(|value| value.text == text)
            .unwrap_or_else(|| panic!("no {text:?} in {:?}", self.texts))
    }

    fn run_frame(
        &mut self,
        ctx: &egui::Context,
        input: egui::RawInput,
        mut draw: impl FnMut(&egui::Context) -> Vec<Action>,
        take_painted: impl FnOnce() -> Vec<(u32, egui::Pos2)>,
    ) {
        let output = ctx.run(input, |ctx| self.actions.extend(draw(ctx)));
        self.painted = take_painted();
        self.texts = rendered_texts(output);
    }
}

fn new_render_run<Action>() -> (egui::Context, RenderRun<Action>) {
    (egui::Context::default(), RenderRun::default())
}

/// Run deterministic egui frames and retain their actions plus the final
/// frame's text and bitmap trace.
pub(crate) fn run_render_frames<Action>(
    frames: Vec<Vec<egui::Event>>,
    mut input: impl FnMut(usize, Vec<egui::Event>) -> egui::RawInput,
    mut draw: impl FnMut(&egui::Context) -> Vec<Action>,
    mut take_painted: impl FnMut() -> Vec<(u32, egui::Pos2)>,
) -> RenderRun<Action> {
    let (ctx, mut result) = new_render_run();
    for (index, events) in frames.into_iter().enumerate() {
        result.run_frame(&ctx, input(index, events), &mut draw, &mut take_painted);
    }
    result
}

/// [`run_render_frames`] with the renderer's shared bitmap trace reset and
/// collected for every frame.
pub(crate) fn run_painted_render_frames<Action>(
    frames: Vec<Vec<egui::Event>>,
    mut input: impl FnMut(usize, Vec<egui::Event>) -> egui::RawInput,
    draw: impl FnMut(&egui::Context) -> Vec<Action>,
) -> RenderRun<Action> {
    run_render_frames(
        frames,
        |index, events| {
            clear_painted_trace();
            input(index, events)
        },
        draw,
        take_painted_trace,
    )
}

/// Drive an existing egui context through input frames and return the last
/// action emitted. Finder keyboard tests use an existing context because its
/// focus memory is part of the behavior under test.
pub(crate) fn run_action_frames<Action>(
    ctx: &egui::Context,
    frames: Vec<Vec<egui::Event>>,
    mut draw: impl FnMut(&egui::Context) -> Option<Action>,
) -> Option<Action> {
    let mut emitted = None;
    for events in frames {
        let input = raw_input(events, egui::vec2(700.0, 520.0));
        let _ = ctx.run(input, |ctx| {
            if let Some(action) = draw(ctx) {
                emitted = Some(action);
            }
        });
    }
    emitted
}

/// Prepend the two idle layout frames required before interacting with a newly
/// opened egui area. `initial` carries the pointer position or other frame
/// metadata that must remain stable while the area settles.
pub(crate) fn warmed_frames<T: Clone>(
    initial: T,
    frames: impl IntoIterator<Item = T>,
) -> impl Iterator<Item = T> {
    [initial.clone(), initial].into_iter().chain(frames)
}

/// Open one manager-owned singleton and require its content publication to
/// accept the exact manager receipt.
pub(crate) fn open_managed_singleton(
    spec: WindowSpec,
    name: &str,
    mut commit: impl FnMut(&WindowTransition) -> Option<WindowId>,
) -> (WindowManager, WindowId) {
    let mut manager = WindowManager::new();
    let opened = manager.apply(WindowCommand::Open(spec)).unwrap();
    let id = opened_window_id(&opened.transition, name);
    assert_eq!(commit(&opened.transition), Some(id));
    (manager, id)
}

pub(crate) trait ManagedFinderFixture: Default {
    type Prepared: Copy;

    fn prepare_finder(faction: crate::cockpit::CockpitFaction) -> Self::Prepared;
    fn finder_spec(prepared: Self::Prepared) -> WindowSpec;
    fn commit_finder(
        &mut self,
        prepared: Self::Prepared,
        transition: &WindowTransition,
    ) -> Option<WindowId>;
    fn open_direct_finder(&mut self, faction: crate::cockpit::CockpitFaction) -> bool;
    fn close_direct_finder(&mut self);
    fn direct_finder_is_open(&self) -> bool;
    fn fixture_ui(&self) -> Option<&crate::finder_common::FinderUiState>;
    fn fixture_ui_mut(&mut self) -> Option<&mut crate::finder_common::FinderUiState>;
}

macro_rules! impl_managed_finder_fixture {
    ($state:path, $prepared:path) => {
        impl ManagedFinderFixture for $state {
            type Prepared = $prepared;

            fn prepare_finder(faction: crate::cockpit::CockpitFaction) -> Self::Prepared {
                Self::prepare_open(faction)
            }

            fn finder_spec(prepared: Self::Prepared) -> WindowSpec {
                prepared.spec()
            }

            fn commit_finder(
                &mut self,
                prepared: Self::Prepared,
                transition: &WindowTransition,
            ) -> Option<WindowId> {
                self.commit_open(prepared, transition)
            }

            fn open_direct_finder(&mut self, faction: crate::cockpit::CockpitFaction) -> bool {
                self.open(faction)
            }

            fn close_direct_finder(&mut self) {
                self.close();
            }

            fn direct_finder_is_open(&self) -> bool {
                self.is_open()
            }

            fn fixture_ui(&self) -> Option<&crate::finder_common::FinderUiState> {
                self.fixture_ui()
            }

            fn fixture_ui_mut(&mut self) -> Option<&mut crate::finder_common::FinderUiState> {
                self.fixture_ui_mut()
            }
        }
    };
}

impl_managed_finder_fixture!(
    crate::fleet_finder::FleetFinderState,
    crate::fleet_finder::PreparedFleetFinder
);
impl_managed_finder_fixture!(
    crate::personnel_finder::PersonnelFinderState,
    crate::personnel_finder::PreparedPersonnelFinder
);
impl_managed_finder_fixture!(
    crate::troop_finder::TroopFinderState,
    crate::troop_finder::PreparedTroopFinder
);

/// Open any Finder family's manager-owned fixture through the same singleton
/// publication contract.
pub(crate) fn managed_finder_fixture<State: ManagedFinderFixture>(
    faction: crate::cockpit::CockpitFaction,
    name: &str,
) -> (WindowManager, State, WindowId) {
    let mut state = State::default();
    let prepared = State::prepare_finder(faction);
    let (manager, id) = open_managed_singleton(State::finder_spec(prepared), name, |transition| {
        state.commit_finder(prepared, transition)
    });
    (manager, state, id)
}

/// Add the shared name/list focus targets and persistent egui context to one
/// manager-owned Finder fixture.
pub(crate) fn finder_navigation_fixture<State>(
    (manager, state, id): (WindowManager, State, WindowId),
    name_control: ControlId,
    list_control: ControlId,
) -> (
    WindowManager,
    State,
    WindowId,
    FocusTarget,
    FocusTarget,
    egui::Context,
) {
    (
        manager,
        state,
        id,
        control_focus(id, name_control),
        control_focus(id, list_control),
        egui::Context::default(),
    )
}

/// Exercise the direct-fixture singleton lifecycle while the owning test
/// supplies only its family-specific state mutation and assertion.
pub(crate) fn assert_finder_reopen_preserves<State: ManagedFinderFixture>(
    configure: impl FnOnce(&mut State),
    preserved: impl FnOnce(&State),
) {
    let mut state = State::default();
    assert!(state.open_direct_finder(crate::cockpit::CockpitFaction::Alliance));
    configure(&mut state);
    assert!(!state.open_direct_finder(crate::cockpit::CockpitFaction::Alliance));
    preserved(&state);
    assert!(state.direct_finder_is_open());
    state.close_direct_finder();
    assert!(!state.direct_finder_is_open());
}

/// Require a Finder category switch to clear its seeded row selection.
pub(crate) fn assert_finder_category_clears_choice<State: ManagedFinderFixture>(
    seed_choice: impl FnOnce(&mut State),
    switch_category: impl FnOnce(&mut State),
    assert_cleared: impl FnOnce(&State),
) {
    let mut state = State::default();
    assert!(state.open_direct_finder(crate::cockpit::CockpitFaction::Alliance));
    seed_choice(&mut state);
    switch_category(&mut state);
    assert_cleared(&state);
}

/// Exercise the common category-change contract while expanding private
/// selection-field wiring inside the owning Finder module.
#[cfg(test)]
macro_rules! assert_finder_category_change_clears {
    ($state:ty, $choice:expr, $switch:expr $(,)?) => {
        $crate::test_support::assert_finder_category_clears_choice::<$state>(
            |state| state.window.as_mut().unwrap().chosen = $choice,
            $switch,
            |state| assert_eq!(state.window.as_ref().unwrap().chosen, None),
        )
    };
}

#[cfg(test)]
pub(crate) use assert_finder_category_change_clears;

/// Reopen an existing managed Finder with the opposite fixture faction and
/// require the singleton transition to preserve its exact identity.
pub(crate) fn recommit_duplicate_finder<State: ManagedFinderFixture>(
    manager: &mut WindowManager,
    state: &mut State,
    existing: WindowId,
) {
    let duplicate = State::prepare_finder(crate::cockpit::CockpitFaction::Empire);
    let transition =
        duplicate_singleton_transition(manager, State::finder_spec(duplicate), existing);
    assert_eq!(state.commit_finder(duplicate, &transition), Some(existing));
}

/// Give a Finder child control manager focus, then exercise its duplicate-open
/// commit without repeating that ordered manager ceremony in each family.
pub(crate) fn focus_and_recommit_duplicate_finder<State: ManagedFinderFixture>(
    manager: &mut WindowManager,
    state: &mut State,
    existing: WindowId,
    focused: FocusTarget,
) {
    manager.apply(WindowCommand::Focus(focused)).unwrap();
    recommit_duplicate_finder(manager, state, existing);
}

/// Focus and recommit a duplicate Finder, then assert every common UI field
/// the singleton open must preserve.
pub(crate) fn assert_focused_duplicate_finder_ui<State: ManagedFinderFixture>(
    manager: &mut WindowManager,
    state: &mut State,
    existing: WindowId,
    focused: FocusTarget,
    name: &str,
    first_row: usize,
    focus_name: bool,
) {
    let ui = state
        .fixture_ui_mut()
        .expect("Finder content remains attached");
    ui.name = name.into();
    ui.first_row = first_row;
    ui.focus_name = focus_name;
    focus_and_recommit_duplicate_finder(manager, state, existing, focused);
    assert_preserved_finder_ui(
        manager,
        state.fixture_ui().expect("Finder content remains attached"),
        existing,
        focused,
        name,
        first_row,
        focus_name,
    );
}

/// Assert the common UI fields and manager focus that a duplicate Finder open
/// must preserve. Family-specific tab/view/selection assertions stay beside
/// each test.
pub(crate) fn assert_preserved_finder_ui(
    manager: &WindowManager,
    ui: &crate::finder_common::FinderUiState,
    expected_id: WindowId,
    focused: FocusTarget,
    name: &str,
    first_row: usize,
    focus_name: bool,
) {
    assert_eq!(ui.window_id(), Some(expected_id));
    assert_eq!(ui.faction, crate::cockpit::CockpitFaction::Alliance);
    assert_eq!(ui.name, name);
    assert_eq!(ui.first_row, first_row);
    assert_eq!(ui.focus_name, focus_name);
    assert_eq!(manager.begin_frame().focused(), Some(focused));
}

/// Exercise stale `Opened` and unrelated `Existing` receipts against any
/// Finder family while preserving its currently attached identity.
pub(crate) fn assert_stale_finder_publications_rejected<State: ManagedFinderFixture>(
    manager: &mut WindowManager,
    state: &mut State,
    current: WindowId,
    unrelated_kind: WindowKind,
) {
    let other = open_singleton_window(
        manager,
        unrelated_kind,
        WindowOwner::StrategicRoot,
        "unrelated Finder",
    );
    let replacement = State::prepare_finder(crate::cockpit::CockpitFaction::Empire);
    assert_stale_singleton_publications_rejected(
        replacement,
        current,
        other,
        |prepared, transition| state.commit_finder(prepared, transition),
    );
    assert_eq!(
        state
            .fixture_ui()
            .and_then(crate::finder_common::FinderUiState::window_id),
        Some(current)
    );
    assert_eq!(
        state.fixture_ui().map(|ui| ui.faction),
        Some(crate::cockpit::CockpitFaction::Alliance)
    );
}

pub(crate) fn assert_finder_count_colours(alliance_index: usize, imperial_index: usize) {
    assert_eq!(
        crate::finder_common::count_cell_colours(alliance_index),
        (egui::Color32::from_rgb(255, 0, 0), egui::Color32::WHITE)
    );
    assert_eq!(
        crate::finder_common::count_cell_colours(imperial_index),
        (egui::Color32::from_rgb(0, 255, 0), egui::Color32::BLACK)
    );
}

/// Require list-focused Finder text input to leave the shared name edit
/// untouched while transferring egui focus to the list widget.
pub(crate) fn assert_finder_list_focus_ignores_text<State, Action>(
    ctx: &egui::Context,
    state: &mut State,
    focused_widget: &'static str,
    drive: impl FnOnce(&mut State, Vec<Vec<egui::Event>>) -> Option<Action>,
) where
    State: ManagedFinderFixture,
{
    let name_before = state
        .fixture_ui()
        .expect("Finder content remains attached")
        .name
        .clone();
    assert!(
        drive(state, vec![vec![egui::Event::Text("ignored".into())]]).is_none(),
        "list-focused text must not emit a Finder action"
    );
    assert_eq!(
        state
            .fixture_ui()
            .expect("Finder content remains attached")
            .name,
        name_before
    );
    assert_eq!(
        ctx.memory(egui::Memory::focused),
        Some(egui::Id::new(focused_widget))
    );
}

/// Define a module-local Alliance Finder fixture constructor without
/// repeating the concrete manager publication ceremony in every Finder test
/// module.
#[cfg(test)]
macro_rules! define_managed_finder_fixture {
    ($function:ident, $state:ty, $name:literal) => {
        fn $function() -> (
            rebellion_windowing::WindowManager,
            $state,
            rebellion_windowing::WindowId,
        ) {
            $crate::test_support::managed_finder_fixture::<$state>(
                $crate::cockpit::CockpitFaction::Alliance,
                $name,
            )
        }
    };
}

#[cfg(test)]
pub(crate) use define_managed_finder_fixture;

/// Require an `Opened` receipt and return its manager identity.
pub(crate) fn opened_window_id(transition: &WindowTransition, name: &str) -> WindowId {
    match *transition {
        WindowTransition::Opened { id } => id,
        ref other => panic!("expected {name} open, got {other:?}"),
    }
}

/// Open one test window through the manager and return its identity.
pub(crate) fn open_window(manager: &mut WindowManager, spec: WindowSpec, name: &str) -> WindowId {
    let transition = manager.apply(WindowCommand::Open(spec)).unwrap().transition;
    opened_window_id(&transition, name)
}

/// Open one singleton fixture window for a manager-owned policy family.
pub(crate) fn open_singleton_window(
    manager: &mut WindowManager,
    kind: WindowKind,
    owner: rebellion_windowing::WindowOwner,
    name: &str,
) -> WindowId {
    open_window(
        manager,
        crate::windowing_policy::singleton_spec(kind, owner),
        name,
    )
}

/// The canonical one-regiment scrap payload used by renderer and app tests.
#[must_use]
pub fn scrap_confirmation() -> crate::move_confirmation::ScrapConfirmation {
    use rebellion_core::missions::MissionFaction;
    use rebellion_core::scrap::{ScrapOrder, ScrapTarget};

    let troop = rebellion_core::ids::TroopKey::default();
    crate::move_confirmation::ScrapConfirmation {
        faction: MissionFaction::Alliance,
        order: ScrapOrder::Troop(troop),
        targets: vec![ScrapTarget::Troop(troop)],
        names: vec!["Army Regiment".into()],
    }
}

/// Run manager-owned content with the shared bitmap trace reset and collected
/// for every frame. Each draw receives the frozen snapshot and claim router.
pub(crate) fn run_painted_routed_render_frames<Action>(
    manager: &WindowManager,
    window: WindowId,
    frames: Vec<(Vec<egui::Event>, Vec<PointerPhase>)>,
    mut input: impl FnMut(usize, Vec<egui::Event>) -> egui::RawInput,
    draw: impl FnMut(
        &egui::Context,
        &RoutingSnapshot,
        &[(InputEventId, PointerPhase)],
        &mut FrameInputRouter,
    ) -> Vec<Action>,
) -> RenderRun<Action> {
    let (ctx, mut result) = new_render_run();
    let mut draw = draw;
    for_each_routed_pointer_frame(
        manager,
        window,
        frames,
        |index, events, snapshot, routed, router| {
            clear_painted_trace();
            result.run_frame(
                &ctx,
                input(index, events),
                |ctx| draw(ctx, snapshot, routed, router),
                take_painted_trace,
            );
        },
    );
    result
}

/// Insert one explored fixture system in a minimal sector.
pub(crate) fn add_system(world: &mut GameWorld, name: &str, control: ControlKind) -> SystemKey {
    let mut system = rebellion_core::test_support::system(name, control);
    system.dat_id = DatId::new(100);
    system.x = 320;
    system.y = 250;
    rebellion_core::test_support::insert_system_in_sector(
        world,
        Sector {
            dat_id: DatId::new(36),
            name: "Sesswenna".into(),
            group: SectorGroup::Core,
            x: 317,
            y: 248,
            systems: Vec::new(),
        },
        system,
    )
    .1
}

/// Insert one named system without a sector relation for row-level fixtures
/// that intentionally exercise only system-local state.
pub(crate) fn add_standalone_system(
    world: &mut GameWorld,
    name: &str,
    control: ControlKind,
) -> SystemKey {
    world.systems.insert(rebellion_core::world::System {
        dat_id: DatId::new(0x9000_0001),
        ..rebellion_core::test_support::system(name, control)
    })
}

/// A minimal world containing the canonical Sluis Van fixture system.
#[must_use]
pub fn world(control: ControlKind) -> (GameWorld, SystemKey) {
    let mut world = GameWorld::default();
    let system = add_system(&mut world, "Sluis Van", control);
    (world, system)
}

/// Insert one Alliance-controlled fixture system.
pub(crate) fn alliance_system(world: &mut GameWorld, name: &str) -> SystemKey {
    add_system(world, name, ControlKind::Controlled(Faction::Alliance))
}

pub(crate) fn key_state(key: egui::Key, pressed: bool) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed,
        repeat: false,
        modifiers: egui::Modifiers::default(),
    }
}

pub(crate) fn key_press(key: egui::Key) -> egui::Event {
    key_state(key, true)
}

pub(crate) fn key_frame(key: egui::Key) -> Vec<Vec<egui::Event>> {
    vec![vec![key_press(key)]]
}

pub(crate) fn tap_key(key: egui::Key) -> Vec<Vec<egui::Event>> {
    vec![vec![key_state(key, true)], vec![key_state(key, false)]]
}

pub(crate) fn hover_frames(point: egui::Pos2) -> Vec<Vec<egui::Event>> {
    vec![
        vec![egui::Event::PointerMoved(point)],
        vec![egui::Event::PointerMoved(point)],
    ]
}

pub(crate) fn pointer_button(point: egui::Pos2, pressed: bool) -> egui::Event {
    pointer_button_with(point, egui::PointerButton::Primary, pressed)
}

pub(crate) fn pointer_button_with(
    point: egui::Pos2,
    button: egui::PointerButton,
    pressed: bool,
) -> egui::Event {
    egui::Event::PointerButton {
        pos: point,
        button,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}

pub(crate) fn click_frames(point: egui::Pos2) -> Vec<Vec<egui::Event>> {
    click_frames_with(point, egui::PointerButton::Primary)
}

/// Two complete primary clicks at one point, preceded by the usual hover
/// warm-up and followed by an idle frame.
pub(crate) fn double_click_frames(point: egui::Pos2) -> Vec<Vec<egui::Event>> {
    let mut frames = hover_frames(point);
    frames.extend([
        vec![pointer_button(point, true)],
        vec![pointer_button(point, false)],
        vec![pointer_button(point, true)],
        vec![pointer_button(point, false)],
        vec![],
    ]);
    frames
}

/// One pointer-move frame followed immediately by primary press and release.
pub(crate) fn immediate_click_frames(point: egui::Pos2) -> Vec<Vec<egui::Event>> {
    vec![
        vec![egui::Event::PointerMoved(point)],
        vec![pointer_button(point, true)],
        vec![pointer_button(point, false)],
    ]
}

pub(crate) fn right_click_frames(point: egui::Pos2) -> Vec<Vec<egui::Event>> {
    click_frames_with(point, egui::PointerButton::Secondary)
}

pub(crate) fn routed_primary_click_frames(
    point: egui::Pos2,
) -> Vec<(Vec<egui::Event>, Vec<PointerPhase>)> {
    routed_pointer_button_frames(
        point,
        egui::PointerButton::Primary,
        PointerButtonId::Primary,
        true,
    )
}

fn routed_pointer_button_frames(
    point: egui::Pos2,
    egui_button: egui::PointerButton,
    routed_button: PointerButtonId,
    release: bool,
) -> Vec<(Vec<egui::Event>, Vec<PointerPhase>)> {
    let mut frames = vec![
        (vec![egui::Event::PointerMoved(point)], vec![]),
        (vec![egui::Event::PointerMoved(point)], vec![]),
        (
            vec![pointer_button_with(point, egui_button, true)],
            vec![PointerPhase::Press(routed_button)],
        ),
    ];
    if release {
        frames.push((
            vec![pointer_button_with(point, egui_button, false)],
            vec![PointerPhase::Release(routed_button)],
        ));
    }
    frames
}

pub(crate) fn routed_secondary_press_frames(
    point: egui::Pos2,
) -> Vec<(Vec<egui::Event>, Vec<PointerPhase>)> {
    routed_pointer_button_frames(
        point,
        egui::PointerButton::Secondary,
        PointerButtonId::Secondary,
        false,
    )
}

pub(crate) fn routed_primary_drag_frames(
    from: egui::Pos2,
    to: egui::Pos2,
) -> Vec<(Vec<egui::Event>, Vec<PointerPhase>)> {
    vec![
        (vec![egui::Event::PointerMoved(from)], vec![]),
        (vec![egui::Event::PointerMoved(from)], vec![]),
        (
            vec![pointer_button(from, true)],
            vec![PointerPhase::Press(PointerButtonId::Primary)],
        ),
        (
            vec![egui::Event::PointerMoved(to)],
            vec![PointerPhase::Motion(PointerButtonId::Primary)],
        ),
    ]
}

pub(crate) fn click_frames_with(
    point: egui::Pos2,
    button: egui::PointerButton,
) -> Vec<Vec<egui::Event>> {
    let mut frames = hover_frames(point);
    frames.extend([
        vec![pointer_button_with(point, button, true)],
        vec![pointer_button_with(point, button, false)],
        vec![],
    ]);
    frames
}

/// Press at `from`, move to `to` with the button held, and release.
pub(crate) fn title_drag_frames(from: egui::Pos2, to: egui::Pos2) -> Vec<Vec<egui::Event>> {
    let mut frames = hover_frames(from);
    frames.extend([
        vec![pointer_button(from, true)],
        vec![egui::Event::PointerMoved(from.lerp(to, 0.5))],
        vec![egui::Event::PointerMoved(to)],
        vec![pointer_button(to, false)],
        vec![egui::Event::PointerMoved(to + egui::vec2(30.0, 30.0))],
    ]);
    frames
}

/// Assert the routing contract shared by modal manager-owned windows.
pub(crate) fn assert_modal_window_routing(
    frame: &RoutingSnapshot,
    window: WindowId,
    hit: Option<WindowId>,
    reports: &[ContentReport],
) {
    let hits = hit.into_iter().collect::<BTreeSet<_>>();
    assert_eq!(
        frame.pointer_route(&hits).target,
        RouteTarget::Window(FocusTarget {
            window,
            control: None,
        })
    );
    assert_eq!(
        frame.pointer_route(&BTreeSet::new()).target,
        RouteTarget::BlockedBy(window)
    );
    assert!(!frame.pointer_allows_root());
    assert_eq!(
        reports,
        [ContentReport {
            window,
            status: ContentStatus::Ready,
        }]
    );
}

pub(crate) fn assert_strategic_title_resources() {
    assert_eq!(
        [1, 2, 3].map(|side| {
            [
                strategic_window_common::title_resource(side, true),
                strategic_window_common::title_resource(side, false),
            ]
        }),
        [[10299, 10200], [10201, 10302], [10303, 10304]]
    );
}

pub(crate) fn assert_strategic_title_button_press<Action>(
    at: impl Fn(f32, f32) -> egui::Pos2,
    mut draw: impl FnMut(Vec<Vec<egui::Event>>) -> (RenderRun<Action>, usize),
) {
    let (hovered, _) = draw(hover_frames(at(210.0, 9.0)));
    assert!(hovered
        .painted
        .contains(&(strategic_window_common::MINIMIZE_ART.0, at(204.0, 3.0),)));

    let mut frames = hover_frames(at(224.0, 9.0));
    frames.push(vec![pointer_button(at(224.0, 9.0), true)]);
    let (held, window_count) = draw(frames);
    assert!(held
        .painted
        .contains(&(strategic_window_common::CLOSE_ART.1, at(218.0, 3.0),)));
    assert!(held
        .painted
        .contains(&(strategic_window_common::MINIMIZE_ART.0, at(204.0, 3.0),)));
    assert_eq!(window_count, 1);
}

pub(crate) fn assert_detail_title_button_actions<State, Action>(
    at: impl Fn(f32, f32) -> egui::Pos2,
    mut opened: impl FnMut() -> State,
    mut draw: impl FnMut(&mut State, Vec<Vec<egui::Event>>) -> RenderRun<Action>,
    expected_sector: &[Action; 2],
    expected_minimize: Action,
    window_count: impl Fn(&State) -> usize,
) where
    Action: std::fmt::Debug + PartialEq,
{
    let mut state = opened();
    assert_eq!(
        draw(&mut state, click_frames(at(9.0, 9.0)))
            .actions
            .as_slice(),
        expected_sector
    );
    assert_eq!(
        draw(&mut state, click_frames(at(210.0, 9.0))).actions,
        [expected_minimize]
    );
    assert_eq!(window_count(&state), 0);

    let mut state = opened();
    assert_eq!(
        draw(&mut state, click_frames(at(224.0, 9.0))).actions,
        Vec::<Action>::new()
    );
    assert_eq!(window_count(&state), 0);
}

#[cfg(test)]
macro_rules! assert_detail_title_buttons {
    ($at:expr, $opened:expr, $draw:expr, $action:ident, $state:ident, $system:expr, $origin:expr $(,)?) => {
        $crate::test_support::assert_detail_title_button_actions(
            $at,
            $opened,
            $draw,
            &[$action::OpenSector($system), $action::SelectSystem($system)],
            $action::Minimize {
                system: $system,
                logical_position: $origin,
            },
            $state::window_count,
        )
    };
}

#[cfg(test)]
pub(crate) use assert_detail_title_buttons;

pub(crate) fn reassert_fixture_surface(
    ctx: &egui::Context,
    id: egui::Id,
    order: egui::Order,
    rect: egui::Rect,
) {
    ctx.move_to_top(egui::LayerId::new(order, id));
    egui::Area::new(id)
        .order(order)
        .fixed_pos(rect.min)
        .show(ctx, |ui| {
            ui.allocate_response(rect.size(), egui::Sense::click());
        });
}

pub(crate) fn assert_manager_activation_preserves_content_order(
    manager: &mut WindowManager,
    ids: &[WindowId],
    content_order: impl Fn() -> Vec<WindowId>,
) {
    assert_eq!(ids.len(), 2, "fixture expects exactly two managed windows");
    assert_eq!(content_order(), ids);
    manager.apply(WindowCommand::Activate(ids[0])).unwrap();
    assert_eq!(
        content_order(),
        ids,
        "local content order must not emulate a manager raise"
    );
    assert_eq!(manager.begin_frame().paint_order(), &[ids[1], ids[0]]);
}

/// Prove a managed surface is restored above a lower layer that reasserts
/// itself before every draw frame.
pub(crate) fn assert_managed_surface_reasserted(
    ctx: &egui::Context,
    rect: egui::Rect,
    expected_layer: egui::Id,
    mut render: impl FnMut(&egui::Context),
) {
    let lower = egui::Id::new("raising-lower-managed-surface");
    for _ in 0..3 {
        let input = raw_input(Vec::new(), egui::vec2(700.0, 520.0));
        let _ = ctx.run(input, |ctx| {
            reassert_fixture_surface(ctx, lower, egui::Order::Tooltip, rect);
            render(ctx);
        });
    }
    assert_eq!(
        ctx.layer_id_at(rect.center()).map(|layer| layer.id),
        Some(expected_layer)
    );
}

pub(crate) fn assert_visibility_transitions(
    manager: &mut WindowManager,
    id: WindowId,
    visible: impl Fn(&RoutingSnapshot) -> bool,
) {
    assert!(visible(&manager.begin_frame()));
    manager.apply(WindowCommand::Minimize(id)).unwrap();
    assert!(!visible(&manager.begin_frame()));
    manager.apply(WindowCommand::Restore(id)).unwrap();
    assert!(visible(&manager.begin_frame()));
}

fn for_each_strategic_fixture(
    mut test: impl FnMut(&mut GameWorld, SystemKey, SystemKey, crate::cockpit::CockpitFaction),
) {
    let (mut world, first) = world(ControlKind::Uncontrolled);
    let second = world.systems.insert(world.systems[first].clone());
    for faction in [
        crate::cockpit::CockpitFaction::Alliance,
        crate::cockpit::CockpitFaction::Empire,
    ] {
        test(&mut world, first, second, faction);
    }
}

pub(crate) fn assert_strategic_keyed_close<State>(
    mut make_state: impl FnMut() -> State,
    mut open: impl FnMut(&mut State, &GameWorld, SystemKey, crate::cockpit::CockpitFaction) -> bool,
    mut close: impl FnMut(&mut State, SystemKey) -> bool,
    is_open: impl Fn(&State, SystemKey) -> bool,
    window_count: impl Fn(&State) -> usize,
) {
    for_each_strategic_fixture(|world, first, second, faction| {
        let mut state = make_state();
        for system in [first, second] {
            assert!(open(&mut state, world, system, faction));
        }
        assert!(close(&mut state, second));
        assert!(
            is_open(&state, first) && !is_open(&state, second),
            "{faction:?}"
        );
        assert!(close(&mut state, first));
        assert_eq!(window_count(&state), 0);
        assert!(!close(&mut state, first));
    });
}

pub(crate) fn assert_strategic_raise<State>(
    mut make_state: impl FnMut() -> State,
    mut open: impl FnMut(&mut State, &GameWorld, SystemKey, crate::cockpit::CockpitFaction) -> bool,
    mut raise: impl FnMut(&mut State, SystemKey) -> bool,
    child_count: impl Fn(&State) -> usize,
    window_count: impl Fn(&State) -> usize,
) {
    for_each_strategic_fixture(|world, first, second, faction| {
        let missing = world.systems.insert(world.systems[first].clone());
        let mut state = make_state();
        assert!(open(&mut state, world, first, faction));
        assert!(open(&mut state, world, second, faction));
        assert_eq!(child_count(&state), 2, "{faction:?}");
        assert!(raise(&mut state, first), "{faction:?}");
        assert_eq!(window_count(&state), 2, "{faction:?}");
        assert!(!raise(&mut state, missing), "{faction:?}");
    });
}

pub(crate) fn assert_detail_window_hits(
    id: WindowId,
    rect: egui::Rect,
    own: egui::LayerId,
    mut hits: impl FnMut((f32, f32), Option<egui::LayerId>) -> BTreeSet<WindowId>,
) {
    let inside = (rect.center().x, rect.center().y);
    let outside = (rect.max.x + 1.0, rect.max.y + 1.0);
    let other = egui::LayerId::new(egui::Order::Foreground, egui::Id::new("other-window"));
    assert_eq!(hits(inside, Some(own)), BTreeSet::from([id]));
    assert!(hits(inside, Some(other)).is_empty());
    assert!(hits(inside, None).is_empty());
    assert!(hits(outside, Some(own)).is_empty());
}

/// Assert the common right-edge inclusion/exclusion contract of original
/// strategic detail canvases while leaving family-specific geometry explicit.
pub(crate) fn assert_detail_canvas_bounds(
    inside: egui::Pos2,
    outside: egui::Pos2,
    mut contains: impl FnMut((f32, f32)) -> bool,
) {
    assert!(contains((inside.x, inside.y)));
    assert!(!contains((outside.x, outside.y)));
}

/// Assert that a direct strategic detail layer names its system while an
/// unrelated retained layer cannot be accepted as a release target.
pub(crate) fn assert_system_release_target(
    system: SystemKey,
    own: egui::LayerId,
    mut release_target: impl FnMut(egui::LayerId) -> Option<SystemKey>,
) {
    assert_eq!(release_target(own), Some(system));
    let elsewhere = egui::LayerId::new(egui::Order::Foreground, egui::Id::new("elsewhere"));
    assert_eq!(release_target(elsewhere), None);
}

/// Assert the shared canvas edge and system release-target contracts of one
/// strategic detail window.
pub(crate) fn assert_detail_canvas_routing(
    system: SystemKey,
    own: egui::LayerId,
    inside: egui::Pos2,
    outside: egui::Pos2,
    contains: impl FnMut((f32, f32)) -> bool,
    release_target: impl FnMut(egui::LayerId) -> Option<SystemKey>,
) {
    assert_detail_canvas_bounds(inside, outside, contains);
    assert_system_release_target(system, own, release_target);
}

/// Paint a retained foreground area that requests the top layer every frame,
/// used to verify that strategic modals reassert their stronger ordering.
pub(crate) fn show_self_raising_area(ctx: &egui::Context, window: egui::Id, size: egui::Vec2) {
    ctx.move_to_top(egui::LayerId::new(egui::Order::Foreground, window));
    show_foreground_area(ctx, window, size);
}

/// Paint a retained clickable foreground area without changing its order.
pub(crate) fn show_foreground_area(ctx: &egui::Context, window: egui::Id, size: egui::Vec2) {
    egui::Area::new(window)
        .order(egui::Order::Foreground)
        .fixed_pos(egui::Pos2::ZERO)
        .show(ctx, |ui| {
            ui.allocate_response(size, egui::Sense::click());
        });
}

/// Prove that manager-owned detail content follows its stable DAT subject,
/// not merely a reused runtime key, and disappears after manager teardown.
pub(crate) fn assert_system_subject_visibility(
    manager: &mut WindowManager,
    world: &mut GameWorld,
    system: SystemKey,
    id: WindowId,
    replacement: DatId,
    family: &str,
    visible: impl Fn(&GameWorld, &RoutingSnapshot) -> bool,
) {
    assert_visibility_transitions(manager, id, |snapshot| visible(world, snapshot));

    let original = world.systems[system].dat_id;
    world.systems[system].dat_id = replacement;
    assert!(
        !visible(world, &manager.begin_frame()),
        "same-key replacement content is not the committed {family} subject"
    );

    world.systems[system].dat_id = original;
    assert!(visible(world, &manager.begin_frame()));
    manager.apply(WindowCommand::Close(id)).unwrap();
    assert!(
        !visible(world, &manager.begin_frame()),
        "attached content stops contributing when its manager identity closes"
    );
}

pub fn route_window_pointer_events(
    router: &mut FrameInputRouter,
    frame: u64,
    snapshot: &RoutingSnapshot,
    window: WindowId,
    phases: impl IntoIterator<Item = PointerPhase>,
) -> Vec<(InputEventId, PointerPhase)> {
    router.begin_frame(frame, snapshot);
    phases
        .into_iter()
        .enumerate()
        .map(|(ordinal, phase)| {
            let event = InputEventId {
                frame,
                ordinal: u32::try_from(ordinal).unwrap(),
            };
            assert_eq!(
                router.collect_pointer(event, &BTreeSet::from([window]), phase),
                InputRecipient::Window(window)
            );
            (event, phase)
        })
        .collect()
}

/// Iterate deterministic manager-owned pointer frames, rebuilding the frozen
/// routing snapshot and normalized event identities for every input batch.
pub(crate) fn for_each_routed_pointer_frame(
    manager: &WindowManager,
    window: WindowId,
    frames: Vec<(Vec<egui::Event>, Vec<PointerPhase>)>,
    mut run: impl FnMut(
        usize,
        Vec<egui::Event>,
        &RoutingSnapshot,
        &[(InputEventId, PointerPhase)],
        &mut FrameInputRouter,
    ),
) {
    let mut router = FrameInputRouter::default();
    for (index, (events, phases)) in frames.into_iter().enumerate() {
        let frame = u64::try_from(index + 1).unwrap();
        let snapshot = manager.begin_frame();
        let routed = route_window_pointer_events(&mut router, frame, &snapshot, window, phases);
        run(index, events, &snapshot, &routed, &mut router);
    }
}

/// Open a duplicate singleton and assert the manager preserves `existing`.
pub(crate) fn duplicate_singleton_transition(
    manager: &mut WindowManager,
    spec: WindowSpec,
    existing: WindowId,
) -> WindowTransition {
    let transition = manager.apply(WindowCommand::Open(spec)).unwrap().transition;
    assert_eq!(
        transition,
        WindowTransition::Existing {
            id: existing,
            effect: ExistingEffect::NoOp,
        }
    );
    transition
}

/// Assert that singleton content refuses publication receipts for a different
/// manager identity while keeping its current identity attached.
pub(crate) fn assert_stale_singleton_publications_rejected<Prepared: Copy>(
    prepared: Prepared,
    current: WindowId,
    other: WindowId,
    mut commit: impl FnMut(Prepared, &WindowTransition) -> Option<WindowId>,
) {
    assert_eq!(
        commit(prepared, &WindowTransition::Opened { id: other }),
        None
    );
    assert_eq!(
        commit(
            prepared,
            &WindowTransition::Existing {
                id: other,
                effect: ExistingEffect::NoOp,
            },
        ),
        None
    );
    assert_ne!(current, other, "fixture identities must be distinct");
}

/// Close manager-owned fixture content and prove that it remains attached
/// until the exact manager teardown receipt is applied.
pub(crate) fn assert_managed_close_receipt<State>(
    manager: &mut WindowManager,
    state: &mut State,
    id: WindowId,
    name: &str,
    is_open: impl Fn(&State) -> bool,
    remove_content: impl Fn(&mut State, &[WindowId]),
) {
    assert!(is_open(state), "content waits for the close receipt");
    let closed = manager.apply(WindowCommand::Close(id)).unwrap();
    let removed = match closed.transition {
        WindowTransition::Closed { closed, .. } => closed,
        ref other => panic!("expected {name} close, got {other:?}"),
    };
    remove_content(state, &removed);
    assert!(!is_open(state));
}

/// Drive Escape through a focused Finder, assert its managed close action,
/// then apply the exact manager teardown receipt.
pub(crate) fn assert_managed_finder_escape_close<State, Action>(
    manager: &mut WindowManager,
    state: &mut State,
    id: WindowId,
    name: &str,
    expected: Action,
    mut draw: impl FnMut(
        &egui::Context,
        &mut State,
        Option<FocusTarget>,
        &mut crate::bmp_cache::BmpCache,
    ) -> Option<Action>,
    is_open: impl Fn(&State) -> bool,
    remove_content: impl Fn(&mut State, &[WindowId]),
) where
    Action: std::fmt::Debug + PartialEq,
{
    let ctx = egui::Context::default();
    let mut cache = crate::bmp_cache::BmpCache::new();
    let mut action = None;
    let _ = ctx.run(finder_escape_input(), |ctx| {
        action = draw(ctx, state, manager.begin_frame().focused(), &mut cache);
    });
    assert_eq!(action, Some(expected));
    assert_managed_close_receipt(manager, state, id, name, is_open, remove_content);
}

/// Minimize manager-owned content, reopen it through the family adapter, and
/// prove the exact identity was restored rather than republished.
pub(crate) fn assert_minimized_reopen<State>(
    manager: &mut WindowManager,
    state: &mut State,
    id: WindowId,
    is_visible: impl Fn(&State, &RoutingSnapshot) -> bool,
    on_minimized: impl FnOnce(&State, &RoutingSnapshot),
    reopen: impl FnOnce(&mut WindowManager, &mut State) -> (WindowId, WindowTransition),
) {
    manager.apply(WindowCommand::Minimize(id)).unwrap();
    let minimized = manager.begin_frame();
    assert!(!is_visible(state, &minimized));
    on_minimized(state, &minimized);

    let (restored_id, transition) = reopen(manager, state);
    assert_eq!(restored_id, id);
    assert!(matches!(
        transition,
        WindowTransition::Existing { id: existing, .. } if existing == id
    ));
    assert!(is_visible(state, &manager.begin_frame()));
}

pub(crate) fn assert_existing_detail_reopen(
    expected: WindowId,
    reopen: impl FnOnce() -> (WindowId, WindowTransition),
) {
    let (actual, transition) = reopen();
    assert_eq!(actual, expected);
    assert!(matches!(
        transition,
        WindowTransition::Existing { id, .. } if id == expected
    ));
}

/// [`assert_minimized_reopen`] for detail families with no additional
/// minimized-state assertion.
pub(crate) fn assert_simple_minimized_reopen<State>(
    manager: &mut WindowManager,
    state: &mut State,
    id: WindowId,
    is_visible: impl Fn(&State, &RoutingSnapshot) -> bool,
    reopen: impl FnOnce(&mut WindowManager, &mut State) -> (WindowId, WindowTransition),
) {
    assert_minimized_reopen(manager, state, id, is_visible, |_, _| {}, reopen);
}

#[cfg(test)]
macro_rules! assert_detail_minimized_reopen {
    ($manager:expr, $state:expr, $id:expr, $system:expr, $prepare:expr, $commit:path $(,)?) => {
        $crate::test_support::assert_simple_minimized_reopen(
            $manager,
            $state,
            $id,
            |state, frame| state.window_is_visible(frame, $system),
            |manager, state| {
                let restored = ($prepare)(state);
                $commit(manager, state, restored)
            },
        )
    };
}

#[cfg(test)]
pub(crate) use assert_detail_minimized_reopen;

/// Assert the common faction replacement and clear contract for direct
/// strategic detail fixtures. The open expression stays at the call site so
/// each family retains its explicit dependencies.
#[cfg(test)]
macro_rules! assert_detail_fixture_reset {
    ($state:ident, $system:expr, $open:expr $(,)?) => {{
        assert!($open);
        assert_eq!($state.faction, $crate::cockpit::CockpitFaction::Empire);
        $state.clear();
        assert_eq!($state.window_count(), 0);
        assert!(!$state.is_open($system));
    }};
}

#[cfg(test)]
pub(crate) use assert_detail_fixture_reset;

/// Assert that reopening a direct detail fixture at a new origin reuses the
/// family window for the same system.
#[cfg(test)]
macro_rules! assert_detail_fixture_reopen {
    ($state:ident, $system:expr, $open:expr $(,)?) => {{
        assert!($open);
        assert_eq!($state.window_count(), 1);
        assert!($state.is_open($system));
    }};
}

#[cfg(test)]
pub(crate) use assert_detail_fixture_reopen;

/// Assert the shared direct-detail canvas routing contract, then replace the
/// fixture with its opposite-faction instance and clear it. The open
/// expression remains at the call site because each family has different
/// production dependencies and private state.
#[cfg(test)]
macro_rules! assert_detail_canvas_and_reset {
    (
        $state:ident,
        $system:expr,
        $layer:expr,
        $inside:expr,
        $outside:expr,
        $layout:expr,
        $open:expr $(,)?
    ) => {{
        $crate::test_support::assert_detail_canvas_routing(
            $system,
            $layer,
            $inside,
            $outside,
            |point| $state.contains_screen_point($layout, point),
            |layer| $state.release_target(layer),
        );
        $crate::test_support::assert_detail_fixture_reset!($state, $system, $open);
    }};
}

#[cfg(test)]
pub(crate) use assert_detail_canvas_and_reset;

/// Restore every saved window through one family adapter and identify the
/// faction in any failed fixture assertion.
pub(crate) fn assert_saved_windows_restore(
    saved: &[SavedWindow],
    faction: crate::cockpit::CockpitFaction,
    mut restore: impl FnMut(&SavedWindow) -> bool,
) {
    for window in saved {
        assert!(restore(window), "{faction:?}");
    }
}

pub(crate) fn restored_saved_windows<State: Default>(
    saved: &[SavedWindow],
    faction: crate::cockpit::CockpitFaction,
    mut restore: impl FnMut(&mut State, &SavedWindow) -> bool,
) -> State {
    let mut state = State::default();
    assert_saved_windows_restore(saved, faction, |window| restore(&mut state, window));
    state
}
