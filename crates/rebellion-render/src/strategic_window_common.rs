//! Shared collection mechanics for the strategic detail-window families.
//!
//! The four detail families own different content but share manager identity,
//! z-order, visibility, teardown and pointer-routing rules.

use std::collections::BTreeSet;
use std::hash::Hash;

use egui_macroquad::egui;
use rebellion_core::ids::{DatId, SystemKey};
use rebellion_core::saved_interface::{SavedWindow, SavedWindowKind};
use rebellion_windowing::{
    ContentReport, ContentStatus, RoutingSnapshot, SubjectId, WindowId, WindowKind, WindowOwner,
    WindowSpec, WindowTransition,
};

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{CockpitFaction, CockpitLayout};
use crate::fleet_window::paint_native;
use crate::native_geometry::{exact_clicked, logical_rect, rect_contains};
use crate::system_window::{title_drag_step, TitleDrag};
use crate::windowing_input::{
    FrameInputRouter, InputEventId, InputRecipient, PointerButtonId, PointerPhase,
};
use crate::windowing_policy::{contextual_spec, system_subject};

/// Common renderer/window-manager vocabulary used by strategic window
/// families. Family-specific domain types remain imported locally.
#[allow(unused_imports)]
pub(crate) mod window_prelude {
    pub(crate) use rebellion_windowing::{
        ContentReport, ContentStatus, ExistingEffect, RoutingSnapshot, SubjectId, WindowId,
        WindowKind, WindowOwner, WindowSpec, WindowTransition,
    };

    pub(crate) use crate::bmp_cache::{BmpCache, DllSource};
    pub(crate) use crate::cockpit::{CockpitFaction, CockpitLayout};
    pub(crate) use crate::fleet_window::paint_native;
}

pub(crate) type WindowIdentity = (Option<WindowId>, Option<SubjectId>, SystemKey);
pub(crate) const SECTOR_ART: (u32, u32) = (10209, 10208);
pub(crate) const MINIMIZE_ART: (u32, u32) = (10253, 10254);
pub(crate) const CLOSE_ART: (u32, u32) = (10108, 10109);
pub(crate) const TITLE_BUTTONS: [(u32, u32, f32, &str); 3] = title_buttons([3.0, 204.0, 218.0]);

pub(crate) struct PreparedDetailWindow {
    pub(crate) spec: WindowSpec,
    pub(crate) logical_position: (i16, i16),
}

/// Define the inert candidate and `prepare_open` implementation for a detail
/// family whose preparation carries only the shared system identity,
/// position, and faction. Richer Fleet and Missions candidates stay explicit.
macro_rules! define_basic_prepared_detail {
    (
        $(#[$meta:meta])*
        $prepared:ident for $state:ty,
        kind: $kind:path,
        size: ($width:expr, $height:expr) $(,)?
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $prepared {
            spec: rebellion_windowing::WindowSpec,
            faction: $crate::cockpit::CockpitFaction,
            system: rebellion_core::ids::SystemKey,
            logical_position: (i16, i16),
        }

        impl $prepared {
            /// Complete manager declaration for this content candidate.
            #[must_use]
            pub const fn spec(&self) -> rebellion_windowing::WindowSpec {
                self.spec
            }

            /// Runtime system identity retained beside the stable DAT subject.
            #[must_use]
            pub const fn system(&self) -> rebellion_core::ids::SystemKey {
                self.system
            }
        }

        impl $state {
            /// Validate and prepare system-backed content without mutating the
            /// live store.
            #[must_use]
            pub fn prepare_open(
                &self,
                world: &rebellion_core::world::GameWorld,
                system: rebellion_core::ids::SystemKey,
                logical_position: (i16, i16),
                faction: $crate::cockpit::CockpitFaction,
                layout: $crate::cockpit::CockpitLayout,
            ) -> Option<$prepared> {
                let prepared = $crate::strategic_window_common::prepare_detail_window(
                    world,
                    system,
                    logical_position,
                    layout,
                    $kind,
                    ($width, $height),
                )?;
                Some($prepared {
                    spec: prepared.spec,
                    faction,
                    system,
                    logical_position: prepared.logical_position,
                })
            }
        }
    };
}

pub(crate) use define_basic_prepared_detail;

/// Validate a system-backed detail identity and clamp its initial rectangle
/// into the strategic galaxy aperture.
pub(crate) fn prepare_detail_window(
    world: &rebellion_core::world::GameWorld,
    system: SystemKey,
    logical_position: (i16, i16),
    layout: CockpitLayout,
    kind: WindowKind,
    size: (f32, f32),
) -> Option<PreparedDetailWindow> {
    let subject = system_subject(world, system)?;
    Some(PreparedDetailWindow {
        spec: contextual_spec(kind, subject, WindowOwner::StrategicRoot),
        logical_position: crate::system_window::clamp_window_to_galaxy(
            logical_position,
            layout,
            size.0,
            size.1,
        ),
    })
}

pub(crate) const fn title_buttons(xs: [f32; 3]) -> [(u32, u32, f32, &'static str); 3] {
    [
        (SECTOR_ART.0, SECTOR_ART.1, xs[0], "sector"),
        (MINIMIZE_ART.0, MINIMIZE_ART.1, xs[1], "minimize"),
        (CLOSE_ART.0, CLOSE_ART.1, xs[2], "close"),
    ]
}

#[allow(
    clippy::cast_precision_loss,
    reason = "Strategic grid indexes are small."
)]
pub(crate) fn grid_cell_rect(
    window: egui::Rect,
    scale: f32,
    index: usize,
    columns: usize,
    origin: (f32, f32),
    cell: (f32, f32),
) -> egui::Rect {
    let column = (index % columns) as f32;
    let row = (index / columns) as f32;
    logical_rect(
        window,
        scale,
        origin.0 + column * cell.0,
        origin.1 + row * cell.1,
        cell.0,
        cell.1,
    )
}

/// Lay out and paint one clipped strategic grid-cell label using logical
/// coordinates. Detail families retain their own offset, width, font, and
/// colour policy.
pub(crate) fn paint_grid_cell_label(
    painter: &egui::Painter,
    cell: egui::Rect,
    scale: f32,
    text: &str,
    font: &egui::FontId,
    color: egui::Color32,
    logical_width: f32,
    offset: (f32, f32),
) {
    let galley = painter.layout(text.to_owned(), font.clone(), color, logical_width * scale);
    painter.galley(
        logical_rect(cell, scale, offset.0, offset.1, 0.0, 0.0).min,
        galley,
        color,
    );
}

pub(crate) fn tab_strip_rect(
    window: egui::Rect,
    scale: f32,
    strip: (f32, f32),
    size: (f32, f32),
    x_offset: f32,
) -> egui::Rect {
    logical_rect(window, scale, strip.0 + x_offset, strip.1, size.0, size.1)
}

#[allow(
    clippy::cast_precision_loss,
    reason = "Strategic list indexes are small."
)]
pub(crate) fn stacked_row_rect(
    window: egui::Rect,
    scale: f32,
    list: (f32, f32),
    row: (f32, f32),
    index: usize,
) -> egui::Rect {
    logical_rect(
        window,
        scale,
        list.0,
        list.1 + index as f32 * row.1,
        row.0,
        row.1,
    )
}

pub(crate) fn cell_screen_rect<T: StrategicWindowRecord>(
    windows: &[T],
    layout: CockpitLayout,
    system: SystemKey,
    index: usize,
    screen_rect: impl Fn(&T, CockpitLayout) -> egui::Rect,
    cell_rect: impl Fn(egui::Rect, f32, usize) -> egui::Rect,
) -> Option<egui::Rect> {
    let window = windows
        .iter()
        .find(|window| window.identity().2 == system)?;
    Some(cell_rect(screen_rect(window, layout), layout.scale, index))
}

macro_rules! define_record_screen_rect {
    ($function:ident, $record:ty, $width:expr, $height:expr) => {
        fn $function(
            window: &$record,
            layout: $crate::cockpit::CockpitLayout,
        ) -> egui_macroquad::egui::Rect {
            $crate::strategic_window_common::record_screen_rect(window, layout, ($width, $height))
        }
    };
}

pub(crate) use define_record_screen_rect;

/// Resolve a retained egui layer to its strategic-window record and translate
/// a screen point into that window's native logical coordinates.
pub(crate) fn local_point_for_layer<T>(
    windows: &[T],
    layout: CockpitLayout,
    layer: egui::LayerId,
    point: egui::Pos2,
    area_id: impl Fn(&T) -> egui::Id,
    screen_rect: impl Fn(&T, CockpitLayout) -> egui::Rect,
) -> Option<(&T, (f32, f32))> {
    let window = windows.iter().find(|window| area_id(window) == layer.id)?;
    let rect = screen_rect(window, layout);
    Some((
        window,
        (
            (point.x - rect.min.x) / layout.scale,
            (point.y - rect.min.y) / layout.scale,
        ),
    ))
}

/// Resolve a strategic window's system, marking its draw outcome for closure
/// when the attached world entity disappeared.
pub(crate) fn system_or_close<'a>(
    world: &'a rebellion_core::world::GameWorld,
    system: SystemKey,
    close: &mut bool,
) -> Option<&'a rebellion_core::world::System> {
    let found = world.systems.get(system);
    if found.is_none() {
        *close = true;
    }
    found
}

pub(crate) struct WindowDrawFrame<'a> {
    pub(crate) system: &'a rebellion_core::world::System,
    pub(crate) scale: f32,
    pub(crate) screen_rect: egui::Rect,
    pub(crate) area_id: egui::Id,
}

/// Resolve the shared world entity and geometry needed to paint one strategic
/// detail-window record. A missing system marks the family result for closure.
pub(crate) fn begin_window_draw<'a, T: StrategicWindowRecord>(
    world: &'a rebellion_core::world::GameWorld,
    window: &T,
    layout: CockpitLayout,
    close: &mut bool,
) -> Option<WindowDrawFrame<'a>> {
    Some(WindowDrawFrame {
        system: system_or_close(world, StrategicWindowRecord::identity(window).2, close)?,
        scale: layout.scale,
        screen_rect: window.screen_rect(layout),
        area_id: window.area_id(),
    })
}

/// Bind the standard strategic-window draw frame or return the caller's empty
/// draw result after marking vanished content for closure.
macro_rules! draw_frame {
    ($result:ident, $system:ident, $scale:ident, $screen_rect:ident, $id:ident; $world:expr, $window:expr, $layout:expr) => {
        let Some($crate::strategic_window_common::WindowDrawFrame {
            system: $system,
            scale: $scale,
            screen_rect: $screen_rect,
            area_id: $id,
        }) = $crate::strategic_window_common::begin_window_draw(
            $world,
            $window,
            $layout,
            &mut $result.close,
        )
        else {
            return $result;
        };
    };
}

pub(crate) use draw_frame;

/// A strategic canvas requests focus from either its surface or any title
/// control, matching the original window families' shared activation rule.
pub(crate) fn interaction_requests_focus(surface: &egui::Response, title_buttons: &[bool]) -> bool {
    surface.clicked() || title_buttons.iter().any(|&clicked| clicked)
}

/// Select an entire single-line egui text edit after a strategic Rename
/// command creates or replaces its editor.
pub(crate) fn select_all_text(ctx: &egui::Context, id: egui::Id, char_count: usize) {
    let mut state = egui::TextEdit::load_state(ctx, id).unwrap_or_default();
    state
        .cursor
        .set_char_range(Some(egui::text::CCursorRange::two(
            egui::text::CCursor::new(0),
            egui::text::CCursor::new(char_count),
        )));
    state.store(ctx, id);
}

pub(crate) trait StrategicWindowRecord {
    fn identity(&self) -> WindowIdentity;
    fn logical_position(&self) -> (i16, i16);
    fn set_logical_position(&mut self, position: (i16, i16));
    fn area_id(&self) -> egui::Id;
    fn screen_rect(&self, layout: CockpitLayout) -> egui::Rect;
}

macro_rules! impl_strategic_window_record {
    ($record:ty, $area_id:path, $screen_rect:path) => {
        impl $crate::strategic_window_common::StrategicWindowRecord for $record {
            fn identity(&self) -> $crate::strategic_window_common::WindowIdentity {
                (self.id, self.subject, self.system)
            }

            fn logical_position(&self) -> (i16, i16) {
                self.logical_position
            }

            fn set_logical_position(&mut self, position: (i16, i16)) {
                self.logical_position = position;
            }

            fn area_id(&self) -> egui_macroquad::egui::Id {
                $area_id(self)
            }

            fn screen_rect(
                &self,
                layout: $crate::cockpit::CockpitLayout,
            ) -> egui_macroquad::egui::Rect {
                $screen_rect(self, layout)
            }
        }
    };
}

pub(crate) use impl_strategic_window_record;

/// Define the stable release-object API while leaving private record lookup
/// and mapping in the owning window module.
macro_rules! define_release_object_method {
    ($(#[$attr:meta])* |$this:ident, $layout:ident, $layer:ident, $point:ident| $body:block) => {
        $(#[$attr])*
        #[must_use]
        pub fn release_object(
            &self,
            $layout: $crate::cockpit::CockpitLayout,
            $layer: egui_macroquad::egui::LayerId,
            $point: egui_macroquad::egui::Pos2,
        ) -> Option<rebellion_core::missions::MissionTarget> {
            let $this = self;
            $body
        }
    };
}

pub(crate) use define_release_object_method;

/// Define the stable manager-aware hit-test API for a private window family.
macro_rules! define_hit_window_ids_method {
    ($(#[$attr:meta])* |$this:ident, $snapshot:ident, $layout:ident, $point:ident, $top_layer:ident| $body:block) => {
        $(#[$attr])*
        #[must_use]
        pub fn hit_window_ids(
            &self,
            $snapshot: &rebellion_windowing::RoutingSnapshot,
            $layout: $crate::cockpit::CockpitLayout,
            $point: (f32, f32),
            $top_layer: Option<egui_macroquad::egui::LayerId>,
        ) -> std::collections::BTreeSet<rebellion_windowing::WindowId> {
            let $this = self;
            $body
        }
    };
}

pub(crate) use define_hit_window_ids_method;

/// Define the compatibility open API shared by direct system fixtures.
macro_rules! define_direct_system_open_method {
    ($(#[$attr:meta])* |$this:ident, $world:ident, $system:ident, $position:ident, $faction:ident, $layout:ident| $body:block) => {
        $(#[$attr])*
        pub fn open(
            &mut self,
            $world: &rebellion_core::world::GameWorld,
            $system: rebellion_core::ids::SystemKey,
            $position: (i16, i16),
            $faction: $crate::cockpit::CockpitFaction,
            $layout: $crate::cockpit::CockpitLayout,
        ) -> bool {
            let $this = self;
            $body
        }
    };
}

pub(crate) use define_direct_system_open_method;

/// Publish the collection and manager-identity API shared by every strategic
/// detail-window state. Family-specific teardown remains in the owning module
/// because Fleet, Defenses and System also clear different transient state.
macro_rules! impl_strategic_window_state_core {
    (
        $state:ty,
        $record:ty,
        $area_id:path
        $(, $extra_field:ident = $extra_default:expr)*
        $(,)?
    ) => {
        impl Default for $state {
            fn default() -> Self {
                Self {
                    faction: $crate::cockpit::CockpitFaction::Alliance,
                    windows: Vec::new(),
                    title_drag: None,
                    $($extra_field: $extra_default,)*
                }
            }
        }

        impl $state {
            /// Whether any window of this kind is open.
            #[must_use]
            pub fn any_open(&self) -> bool {
                !$crate::strategic_window_common::is_empty(&self.windows)
            }

            /// Egui layer attached to one manager-issued content identity.
            #[must_use]
            pub fn managed_layer_id(
                &self,
                id: rebellion_windowing::WindowId,
            ) -> Option<egui_macroquad::egui::LayerId> {
                $crate::strategic_window_common::managed_layer_id(&self.windows, id, $area_id)
            }

            /// Exact manager identity attached to `system`'s committed content.
            #[must_use]
            pub fn window_id(
                &self,
                system: rebellion_core::ids::SystemKey,
            ) -> Option<rebellion_windowing::WindowId> {
                $crate::strategic_window_common::window_id(&self.windows, system)
            }

            /// Whether `system`'s exact content identity participates in this
            /// frozen frame. Direct fixture content has no manager identity.
            #[must_use]
            pub fn window_is_visible(
                &self,
                snapshot: &rebellion_windowing::RoutingSnapshot,
                system: rebellion_core::ids::SystemKey,
            ) -> bool {
                $crate::strategic_window_common::window_is_visible(&self.windows, snapshot, system)
            }

            /// Whether exact manager-owned content remains attached.
            #[must_use]
            pub fn contains_window(&self, id: rebellion_windowing::WindowId) -> bool {
                $crate::strategic_window_common::contains_window(&self.windows, id)
            }

            /// Carry a window drag one frame while the primary button remains
            /// down, releasing it when the shared title-drag state ends.
            fn update_title_drag(
                &mut self,
                ctx: &egui_macroquad::egui::Context,
                layout: $crate::cockpit::CockpitLayout,
            ) {
                $crate::strategic_window_common::update_title_drag(
                    &mut self.windows,
                    &mut self.title_drag,
                    ctx,
                    layout,
                    $area_id,
                );
            }

            fn window_mut(
                &mut self,
                system: rebellion_core::ids::SystemKey,
            ) -> Option<&mut $record> {
                $crate::strategic_window_common::window_mut(&mut self.windows, system)
            }
        }
    };
}

pub(crate) use impl_strategic_window_state_core;

/// Publish the count, keyed-open and rectangle accessors shared by detail
/// states whose public API exposes all three operations.
macro_rules! impl_strategic_window_collection_accessors {
    ($state:ty, $screen_rect:path) => {
        impl $state {
            #[must_use]
            pub fn window_count(&self) -> usize {
                self.windows.len()
            }

            #[must_use]
            pub fn is_open(&self, system: rebellion_core::ids::SystemKey) -> bool {
                $crate::strategic_window_common::is_open(&self.windows, system)
            }

            /// The screen rectangle of `system`'s window.
            #[must_use]
            pub fn screen_rect(
                &self,
                layout: $crate::cockpit::CockpitLayout,
                system: rebellion_core::ids::SystemKey,
            ) -> Option<egui_macroquad::egui::Rect> {
                $crate::strategic_window_common::screen_rect(&self.windows, system, |window| {
                    $screen_rect(window, layout)
                })
            }
        }
    };
}

pub(crate) use impl_strategic_window_collection_accessors;

macro_rules! impl_strategic_window_cell_rect {
    ($state:ty, $screen_rect:expr, $cell_rect:path) => {
        impl $state {
            /// The screen rectangle of the indexed grid cell in `system`.
            #[must_use]
            pub fn cell_screen_rect(
                &self,
                layout: $crate::cockpit::CockpitLayout,
                system: rebellion_core::ids::SystemKey,
                index: usize,
            ) -> Option<egui_macroquad::egui::Rect> {
                $crate::strategic_window_common::cell_screen_rect(
                    &self.windows,
                    layout,
                    system,
                    index,
                    $screen_rect,
                    $cell_rect,
                )
            }
        }
    };
}

pub(crate) use impl_strategic_window_cell_rect;

/// Publish the point-routing API shared by windows keyed directly by a
/// strategic system. Fleet is deliberately excluded: its `GalaxyChild` key is
/// the manager identity rather than the system identity.
macro_rules! impl_system_keyed_window_routing {
    ($state:ty, $child:path, $area_id:path, $screen_rect:path) => {
        impl $state {
            #[must_use]
            pub fn contains_screen_point(
                &self,
                layout: $crate::cockpit::CockpitLayout,
                point: (f32, f32),
            ) -> bool {
                self.contains_shown_screen_point(layout, point, &|_| true)
            }

            /// [`Self::contains_screen_point`] over the windows accepted by
            /// the caller's strategic-scene projection.
            #[must_use]
            pub fn contains_shown_screen_point(
                &self,
                layout: $crate::cockpit::CockpitLayout,
                point: (f32, f32),
                shown: &dyn Fn($crate::windowing::GalaxyChild) -> bool,
            ) -> bool {
                $crate::strategic_window_common::contains_shown_screen_point(
                    &self.windows,
                    point,
                    |system| shown($child(system)),
                    |window| $screen_rect(window, layout),
                )
            }

            /// Whether visible content covers `point`. Minimized manager
            /// content stays attached but cannot block the strategic root.
            #[must_use]
            pub fn contains_visible_screen_point(
                &self,
                snapshot: &rebellion_windowing::RoutingSnapshot,
                layout: $crate::cockpit::CockpitLayout,
                point: (f32, f32),
            ) -> bool {
                $crate::strategic_window_common::contains_visible_screen_point_where(
                    &self.windows,
                    snapshot,
                    point,
                    |_| true,
                    |window| $screen_rect(window, layout),
                )
            }

            /// Exact visible manager identities whose rectangle covers
            /// `point` when egui reports that content layer as topmost.
            #[must_use]
            pub fn hit_window_ids(
                &self,
                snapshot: &rebellion_windowing::RoutingSnapshot,
                layout: $crate::cockpit::CockpitLayout,
                point: (f32, f32),
                top_layer: Option<egui_macroquad::egui::LayerId>,
            ) -> std::collections::BTreeSet<rebellion_windowing::WindowId> {
                $crate::strategic_window_common::hit_window_ids_where(
                    &self.windows,
                    snapshot,
                    point,
                    top_layer,
                    |_| true,
                    $area_id,
                    |window| $screen_rect(window, layout),
                )
            }
        }
    };
}

pub(crate) use impl_system_keyed_window_routing;

/// Publish the direct-fixture z-order operations shared by the Defenses and
/// Missions states. Manager-owned z-order remains exclusively in the manager.
macro_rules! impl_fixture_window_order {
    ($state:ty, $record:ty) => {
        impl $state {
            /// Bring `system`'s direct fixture window to the top of its local
            /// z-order. False when it is not open.
            pub fn raise(&mut self, system: rebellion_core::ids::SystemKey) -> bool {
                self.focus(system)
            }

            /// Close `system`'s direct fixture window. False when absent.
            pub fn close_window(&mut self, system: rebellion_core::ids::SystemKey) -> bool {
                self.close(system).is_some()
            }

            fn focus(&mut self, system: rebellion_core::ids::SystemKey) -> bool {
                $crate::strategic_window_common::focus(&mut self.windows, system)
            }

            fn close(&mut self, system: rebellion_core::ids::SystemKey) -> Option<$record> {
                $crate::strategic_window_common::close(&mut self.windows, system)
            }

            fn open_direct(
                &mut self,
                system_exists: bool,
                system: rebellion_core::ids::SystemKey,
                faction: $crate::cockpit::CockpitFaction,
                create: impl FnOnce() -> $record,
            ) -> bool {
                self.prepare_faction(faction);
                $crate::strategic_window_common::open_direct(
                    &mut self.windows,
                    system_exists,
                    |window: &$record| {
                        $crate::strategic_window_common::StrategicWindowRecord::identity(window).2
                            == system
                    },
                    create,
                )
            }
        }
    };
}

pub(crate) use impl_fixture_window_order;

macro_rules! impl_admitted_release_target {
    ($state:ty, $area_id:path) => {
        impl $state {
            /// The system under an admitted gameplay-release layer, or `None`
            /// when the layer does not name this window family.
            #[must_use]
            pub fn release_target(
                &self,
                layer: egui_macroquad::egui::LayerId,
            ) -> Option<rebellion_core::ids::SystemKey> {
                $crate::strategic_window_common::release_target(
                    &self.windows,
                    layer,
                    |window| window.release_target_visible,
                    $area_id,
                )
            }
        }
    };
}

pub(crate) use impl_admitted_release_target;

pub(crate) const fn title_resource(side: u8, focused: bool) -> u32 {
    match (side, focused) {
        (1, true) => 10299,
        (1, false) => 10200,
        (2, true) => 10201,
        (2, false) => 10302,
        (_, true) => 10303,
        (_, false) => 10304,
    }
}

pub(crate) struct WindowPaintSet<T> {
    pub(crate) windows: Vec<T>,
    pub(crate) stale_manager_windows: Vec<WindowId>,
    pub(crate) focused_manager: Option<WindowId>,
    pub(crate) focused_fixture_system: Option<SystemKey>,
}

pub(crate) struct PaintProjection<T> {
    pub(crate) windows: Vec<T>,
    pub(crate) stale_manager_windows: Vec<WindowId>,
}

/// Project manager paint order onto one renderer family, validating each
/// manager identity and then appending direct test fixtures.
pub(crate) fn paint_projection<T: Clone>(
    windows: &[T],
    snapshot: Option<&RoutingSnapshot>,
    kind: WindowKind,
    identity: impl Fn(&T) -> (Option<WindowId>, Option<SubjectId>),
    shown: impl Fn(&T) -> bool,
    admitted: impl Fn(&T) -> bool,
) -> PaintProjection<T> {
    let mut paint = Vec::new();
    let mut stale_manager_windows = Vec::new();
    if let Some(snapshot) = snapshot {
        for &id in snapshot.paint_order() {
            let Some(key) = snapshot.key(id).filter(|key| key.kind == kind) else {
                continue;
            };
            match windows
                .iter()
                .find(|window| identity(window).0 == Some(id))
                .cloned()
            {
                Some(window) if !shown(&window) => {}
                Some(window) if admitted(&window) && key.subject == identity(&window).1 => {
                    paint.push(window);
                }
                Some(_) | None => stale_manager_windows.push(id),
            }
        }
    }
    paint.extend(
        windows
            .iter()
            .filter(|window| identity(window).0.is_none())
            .cloned(),
    );
    PaintProjection {
        windows: paint,
        stale_manager_windows,
    }
}

/// Rebuild one detail family's paint list from the manager snapshot, then add
/// direct fixture content behind the same focus rules.
pub(crate) fn window_paint_set<T: StrategicWindowRecord + Clone>(
    windows: &[T],
    snapshot: Option<&RoutingSnapshot>,
    kind: WindowKind,
    shown: impl Fn(SystemKey) -> bool,
    admitted: impl Fn(&T) -> bool,
) -> WindowPaintSet<T> {
    let PaintProjection {
        windows: paint,
        stale_manager_windows,
    } = paint_projection(
        windows,
        snapshot,
        kind,
        |window| (window.identity().0, window.identity().1),
        |window| shown(window.identity().2),
        admitted,
    );
    let focused_manager = snapshot.and_then(|frame| frame.focused().map(|focus| focus.window));
    let focused_fixture_system = paint
        .last()
        .filter(|window| window.identity().0.is_none())
        .map(|window| window.identity().2);
    WindowPaintSet {
        windows: paint,
        stale_manager_windows,
        focused_manager,
        focused_fixture_system,
    }
}

/// Refresh one detail family's per-frame release admission and build its paint
/// projection in one pass. This keeps the manager/direct-fixture split
/// identical across window families.
#[allow(
    clippy::too_many_arguments,
    reason = "Visibility, projection and mutation are separate window-family contracts."
)]
pub(crate) fn refreshed_window_paint_set<T: StrategicWindowRecord + Clone>(
    windows: &mut [T],
    snapshot: Option<&RoutingSnapshot>,
    kind: WindowKind,
    shown: impl Fn(SystemKey) -> bool,
    managed_visible: impl Fn(&T, &RoutingSnapshot) -> bool,
    direct_visible: impl Fn(&T) -> bool,
    mut set_visible: impl FnMut(&mut T, bool),
    admitted: impl Fn(&T) -> bool,
) -> WindowPaintSet<T> {
    refresh_release_targets(
        windows,
        snapshot,
        managed_visible,
        direct_visible,
        &mut set_visible,
    );
    window_paint_set(windows, snapshot, kind, shown, admitted)
}

/// Build the release-admitted paint set shared by system-backed detail
/// families whose private records carry `release_target_visible`.
macro_rules! refreshed_release_window_paint_set {
    ($windows:expr, $snapshot:expr, $kind:path, $shown:expr, $world:expr, $visible:path $(,)?) => {
        $crate::strategic_window_common::refreshed_window_paint_set(
            $windows,
            $snapshot,
            $kind,
            $shown,
            |window, snapshot| $visible(window, snapshot, $world),
            |window| window.id.is_none() && $world.systems.contains_key(window.system),
            |window, visible| window.release_target_visible = visible,
            |window| window.release_target_visible,
        )
    };
}

pub(crate) use refreshed_release_window_paint_set;

/// Rebuild a detail family's paint order from manager identities, append
/// direct fixture records, and apply the family's visibility projection.
/// Families that must diagnose stale manager content use [`window_paint_set`]
/// instead; this helper preserves the legacy silent-skip contract.
pub(crate) fn presentation_windows<T: StrategicWindowRecord + Clone>(
    windows: &[T],
    snapshot: Option<&RoutingSnapshot>,
    kind: WindowKind,
    shown: impl Fn(&T) -> bool,
) -> Vec<T> {
    let mut paint = Vec::new();
    if let Some(snapshot) = snapshot {
        paint.extend(snapshot.paint_order().iter().filter_map(|id| {
            snapshot
                .key(*id)
                .filter(|key| key.kind == kind)
                .and_then(|_| {
                    windows
                        .iter()
                        .find(|window| window.identity().0 == Some(*id))
                })
                .cloned()
        }));
    }
    paint.extend(
        windows
            .iter()
            .filter(|window| window.identity().0.is_none())
            .cloned(),
    );
    paint.retain(shown);
    paint
}

pub(crate) fn presentation_focus<T: StrategicWindowRecord>(
    windows: &[T],
    snapshot: Option<&RoutingSnapshot>,
    fallback_to_last_managed: bool,
) -> (Option<WindowId>, Option<SystemKey>) {
    let last = windows.last();
    let focused_manager = snapshot
        .and_then(|frame| frame.focused().map(|focus| focus.window))
        .or_else(|| {
            fallback_to_last_managed
                .then(|| last?.identity().0)
                .flatten()
        });
    let focused_fixture_system = last
        .filter(|window| window.identity().0.is_none())
        .map(|window| window.identity().2);
    (focused_manager, focused_fixture_system)
}

/// Refresh the per-frame admission bit shared by strategic windows that can
/// receive gameplay drops. Manager-owned records use the frozen snapshot;
/// direct fixture records use the caller's current-world predicate.
pub(crate) fn refresh_release_targets<T>(
    windows: &mut [T],
    snapshot: Option<&RoutingSnapshot>,
    visible: impl Fn(&T, &RoutingSnapshot) -> bool,
    direct_visible: impl Fn(&T) -> bool,
    mut set_visible: impl FnMut(&mut T, bool),
) {
    for window in windows {
        let admitted = snapshot.map_or_else(
            || direct_visible(window),
            |snapshot| visible(window, snapshot),
        );
        set_visible(window, admitted);
    }
}

/// Resolve the common focus rule for manager-owned and direct fixture detail
/// windows without making local vector order impersonate manager z-order.
pub(crate) fn is_focused<T: StrategicWindowRecord>(
    window: &T,
    focused_manager: Option<WindowId>,
    focused_fixture_system: Option<SystemKey>,
) -> bool {
    match window.identity().0 {
        Some(id) => focused_manager == Some(id),
        None => focused_fixture_system == Some(window.identity().2),
    }
}

/// Common close/minimize inputs for one strategic detail window after its
/// family-specific draw pass and pointer admission have completed.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DetailWindowExit {
    pub(crate) id: Option<WindowId>,
    pub(crate) system: SystemKey,
    pub(crate) logical_position: (i16, i16),
    pub(crate) close_requested: bool,
    pub(crate) minimize_requested: bool,
    pub(crate) pointer_permitted: bool,
    pub(crate) system_exists: bool,
}

/// Apply the close/minimize contract shared by strategic detail families.
/// Returns true when the caller must stop processing this window.
pub(crate) fn apply_detail_window_exit<Action>(
    request: DetailWindowExit,
    actions: &mut Vec<Action>,
    mut close_fixture: impl FnMut(SystemKey) -> Option<(i16, i16)>,
    close_managed: impl FnOnce(WindowId) -> Action,
    minimize_managed: impl FnOnce(WindowId, SystemKey, (i16, i16)) -> Action,
    minimize_fixture: impl FnOnce(SystemKey, (i16, i16)) -> Option<Action>,
) -> bool {
    if request.close_requested || !request.system_exists {
        if let Some(id) = request.id {
            actions.push(close_managed(id));
        } else {
            close_fixture(request.system);
        }
        return true;
    }
    if !request.minimize_requested || !request.pointer_permitted {
        return false;
    }
    if let Some(id) = request.id {
        actions.push(minimize_managed(
            id,
            request.system,
            request.logical_position,
        ));
    } else if let Some(position) = close_fixture(request.system) {
        if let Some(action) = minimize_fixture(request.system, position) {
            actions.push(action);
        }
    }
    true
}

/// Bind the standard managed/direct minimize variants used by Fleet,
/// Defenses and Missions while keeping their private `close` method and action
/// enum in the owning module.
macro_rules! apply_minimizable_detail_exit {
    (
        ($state:expr, $actions:expr, $window:expr, $system:expr),
        ($close:expr, $minimize:expr, $pointer:expr, $exists:expr),
        $action:ident $(,)?
    ) => {
        $crate::strategic_window_common::apply_detail_window_exit(
            $crate::strategic_window_common::DetailWindowExit {
                id: $window.id,
                system: $system,
                logical_position: $window.logical_position,
                close_requested: $close,
                minimize_requested: $minimize,
                pointer_permitted: $pointer,
                system_exists: $exists,
            },
            &mut $actions,
            |system| ($state).close(system).map(|closed| closed.logical_position),
            $action::Close,
            |window, system, logical_position| $action::MinimizeManaged {
                window,
                system,
                logical_position,
            },
            |system, logical_position| {
                Some($action::Minimize {
                    system,
                    logical_position,
                })
            },
        )
    };
}

pub(crate) use apply_minimizable_detail_exit;

/// Apply the activation and system-selection tail shared by detail families.
pub(crate) fn apply_detail_window_focus<Action>(
    id: Option<WindowId>,
    system: SystemKey,
    requested: bool,
    pointer_permitted: bool,
    actions: &mut Vec<Action>,
    activate: impl FnOnce(WindowId) -> Action,
    mut focus_fixture: impl FnMut(SystemKey),
    select_system: impl FnOnce(SystemKey) -> Action,
) {
    if !requested || !pointer_permitted {
        return;
    }
    if let Some(id) = id {
        actions.push(activate(id));
    } else {
        focus_fixture(system);
    }
    actions.push(select_system(system));
}

/// Bind the standard Activate/SelectSystem focus tail while expanding in the
/// owning module so its private direct-fixture `focus` method remains private.
macro_rules! apply_detail_focus {
    (
        ($state:expr, $actions:expr, $window:expr, $system:expr),
        ($requested:expr, $pointer:expr),
        $action:ident $(,)?
    ) => {
        $crate::strategic_window_common::apply_detail_window_focus(
            $window.id,
            $system,
            $requested,
            $pointer,
            &mut $actions,
            $action::Activate,
            |system| {
                ($state).focus(system);
            },
            $action::SelectSystem,
        )
    };
}

pub(crate) use apply_detail_focus;

pub(crate) fn pointer_mutation_permitted(
    window: Option<WindowId>,
    mutation: bool,
    causal_phases: &[PointerPhase],
    router: Option<&mut FrameInputRouter>,
    pointer_events: &[(InputEventId, PointerPhase)],
    already_claimed: impl Fn(PointerPhase) -> bool,
) -> bool {
    match (window, mutation, router) {
        (Some(id), true, Some(router)) => {
            !causal_phases.is_empty()
                && causal_phases.iter().all(|&phase| {
                    already_claimed(phase)
                        || claim_pointer_event(router, pointer_events.iter().copied(), id, phase)
                            .is_some()
                })
        }
        (Some(_), true, None) => false,
        _ => true,
    }
}

/// Choose the first semantic pointer phase whose mutation occurred, falling
/// back to the ordinary primary release used by strategic controls.
pub(crate) fn pointer_mutation_phase(
    candidates: impl IntoIterator<Item = (bool, PointerPhase)>,
) -> PointerPhase {
    candidates
        .into_iter()
        .find_map(|(occurred, phase)| occurred.then_some(phase))
        .unwrap_or(PointerPhase::Release(PointerButtonId::Primary))
}

/// Admit a single-phase strategic-window mutation through its exact manager
/// event. Primary releases may reuse an already-claimed drag release or fall
/// back to the gesture's primary press, matching the legacy family adapters.
pub(crate) fn single_phase_pointer_mutation_permitted(
    window: Option<WindowId>,
    mutation: bool,
    router: Option<&mut FrameInputRouter>,
    phase: PointerPhase,
    primary_release_already_claimed: bool,
    mut claim: impl FnMut(&mut FrameInputRouter, WindowId, PointerPhase) -> bool,
) -> bool {
    match (window, mutation, router) {
        (Some(_), true, Some(_)) if primary_release_already_claimed => true,
        (Some(id), true, Some(router)) => {
            claim(router, id, phase)
                || (phase == PointerPhase::Release(PointerButtonId::Primary)
                    && claim(router, id, PointerPhase::Press(PointerButtonId::Primary)))
        }
        (Some(_), true, None) => false,
        _ => true,
    }
}

pub(crate) fn start_title_drag(
    active: &mut Option<TitleDrag>,
    area: egui::Id,
    grab: Option<egui::Vec2>,
) {
    if let Some(grab) = grab {
        *active = Some(TitleDrag { area, grab });
    }
}

pub(crate) fn record_title_drag<T>(
    active: &mut Option<TitleDrag>,
    area: egui::Id,
    result: T,
    grab: impl FnOnce(&T) -> Option<egui::Vec2>,
) -> T {
    start_title_drag(active, area, grab(&result));
    result
}

/// The bitmap-backed foreground canvas shared by strategic detail windows.
pub(crate) struct StrategicWindowCanvas<'a> {
    ctx: &'a egui::Context,
    pub(crate) local: egui::Rect,
    pub(crate) response: egui::Response,
    pub(crate) painter: egui::Painter,
    pub(crate) scale: f32,
    pub(crate) pointer: Option<egui::Pos2>,
    pub(crate) primary_down: bool,
}

pub(crate) struct DetailWindowDraw<'a, T> {
    ctx: &'a egui::Context,
    world: &'a rebellion_core::world::GameWorld,
    window: &'a T,
    focused: bool,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &'a mut BmpCache,
}

/// Input and chrome results shared by every strategic detail-window frame.
/// Screen modules add only their domain-specific selection and list outcomes.
#[derive(Default)]
pub(crate) struct DetailWindowInteraction {
    /// Where a press on the window's own surface took hold.
    pub(crate) drag_grab: Option<egui::Vec2>,
    pub(crate) focus: bool,
    pub(crate) minimize: bool,
    pub(crate) open_sector: bool,
    /// Physical phases that causally produced mutations, in backend order.
    pub(crate) causal_pointer_phases: Vec<PointerPhase>,
}

impl DetailWindowInteraction {
    pub(crate) fn note_pointer_phase(&mut self, phase: PointerPhase) {
        if !self.causal_pointer_phases.contains(&phase) {
            self.causal_pointer_phases.push(phase);
        }
    }
}

pub(crate) struct DetailWindowsDraw<'a> {
    pub(crate) ctx: &'a egui::Context,
    pub(crate) world: &'a rebellion_core::world::GameWorld,
    pub(crate) fog: &'a rebellion_core::fog::FogState,
    pub(crate) missions: &'a rebellion_core::missions::MissionState,
    pub(crate) faction: CockpitFaction,
    pub(crate) layout: CockpitLayout,
    pub(crate) cache: &'a mut BmpCache,
    pub(crate) snapshot: Option<&'a RoutingSnapshot>,
    pub(crate) pointer_events: &'a [(InputEventId, PointerPhase)],
    pub(crate) router: Option<&'a mut FrameInputRouter>,
    pub(crate) shown: &'a dyn Fn(crate::windowing::GalaxyChild) -> bool,
}

/// The common application boundary for manager-owned strategic detail
/// windows. Families provide only their concrete state; rendering, routing,
/// and command-center visibility travel together through this framework type.
pub struct ManagedDetailWindowsDraw<'a, State> {
    pub ctx: &'a egui::Context,
    pub world: &'a rebellion_core::world::GameWorld,
    pub fog: &'a rebellion_core::fog::FogState,
    pub missions: &'a rebellion_core::missions::MissionState,
    pub state: &'a mut State,
    pub faction: CockpitFaction,
    pub layout: CockpitLayout,
    pub cache: &'a mut BmpCache,
    pub snapshot: &'a RoutingSnapshot,
    pub pointer_events: &'a [(InputEventId, PointerPhase)],
    pub router: &'a mut FrameInputRouter,
}

impl<'a, State> ManagedDetailWindowsDraw<'a, State> {
    pub(crate) fn with_visibility<'draw>(
        self,
        shown: &'draw dyn Fn(crate::windowing::GalaxyChild) -> bool,
    ) -> (DetailWindowsDraw<'draw>, &'draw mut State)
    where
        'a: 'draw,
    {
        let Self {
            ctx,
            world,
            fog,
            missions,
            state,
            faction,
            layout,
            cache,
            snapshot,
            pointer_events,
            router,
        } = self;
        (
            DetailWindowsDraw::new(
                ctx,
                world,
                fog,
                missions,
                faction,
                layout,
                cache,
                Some(snapshot),
                pointer_events,
                Some(router),
                shown,
            ),
            state,
        )
    }
}

static SHOW_ALL_GALAXY_CHILDREN: fn(crate::windowing::GalaxyChild) -> bool = |_| true;

impl<'a> DetailWindowsDraw<'a> {
    #[allow(
        clippy::too_many_arguments,
        reason = "This is the shared boundary replacing per-family argument plumbing."
    )]
    pub(crate) fn new(
        ctx: &'a egui::Context,
        world: &'a rebellion_core::world::GameWorld,
        fog: &'a rebellion_core::fog::FogState,
        missions: &'a rebellion_core::missions::MissionState,
        faction: CockpitFaction,
        layout: CockpitLayout,
        cache: &'a mut BmpCache,
        snapshot: Option<&'a RoutingSnapshot>,
        pointer_events: &'a [(InputEventId, PointerPhase)],
        router: Option<&'a mut FrameInputRouter>,
        shown: &'a dyn Fn(crate::windowing::GalaxyChild) -> bool,
    ) -> Self {
        Self {
            ctx,
            world,
            fog,
            missions,
            faction,
            layout,
            cache,
            snapshot,
            pointer_events,
            router,
            shown,
        }
    }

    pub(crate) fn unmanaged(
        ctx: &'a egui::Context,
        world: &'a rebellion_core::world::GameWorld,
        fog: &'a rebellion_core::fog::FogState,
        missions: &'a rebellion_core::missions::MissionState,
        faction: CockpitFaction,
        layout: CockpitLayout,
        cache: &'a mut BmpCache,
    ) -> Self {
        Self::new(
            ctx,
            world,
            fog,
            missions,
            faction,
            layout,
            cache,
            None,
            &[],
            None,
            &SHOW_ALL_GALAXY_CHILDREN,
        )
    }
}

/// Define the public unmanaged and manager-owned entrypoints for a strategic
/// detail family that uses [`DetailWindowsDraw`]. Screen modules retain their
/// concrete state, action, and paint implementation; this framework macro owns
/// the identical input plumbing between those boundaries.
macro_rules! define_detail_window_draw_entrypoints {
    (
        unmanaged: $unmanaged:ident,
        managed: $managed:ident,
        managed_shown: $managed_shown:ident,
        managed_draw: $managed_draw:ident,
        state: $state:ty,
        action: $action:ty,
        draw: $draw:path $(,)?
    ) => {
        /// Draw every open window in this detail family without manager routing.
        #[allow(
            clippy::too_many_arguments,
            reason = "Keep the public fixture boundary aligned with manager-owned detail drawing."
        )]
        pub fn $unmanaged(
            ctx: &egui::Context,
            world: &GameWorld,
            fog: &FogState,
            missions: &MissionState,
            state: &mut $state,
            faction: CockpitFaction,
            layout: CockpitLayout,
            cache: &mut BmpCache,
        ) -> Vec<$action> {
            $draw(
                $crate::strategic_window_common::DetailWindowsDraw::unmanaged(
                    ctx, world, fog, missions, faction, layout, cache,
                ),
                state,
            )
        }

        /// Draw manager-owned content in frozen bottom-to-top paint order.
        #[allow(
            clippy::too_many_arguments,
            reason = "Keep the public manager boundary aligned with unmanaged detail drawing."
        )]
        pub fn $managed(
            ctx: &egui::Context,
            world: &GameWorld,
            fog: &FogState,
            missions: &MissionState,
            state: &mut $state,
            faction: CockpitFaction,
            layout: CockpitLayout,
            cache: &mut BmpCache,
            snapshot: &RoutingSnapshot,
            pointer_events: &[(InputEventId, PointerPhase)],
            router: &mut FrameInputRouter,
        ) -> Vec<$action> {
            $managed_shown(
                $managed_draw {
                    ctx,
                    world,
                    fog,
                    missions,
                    state,
                    faction,
                    layout,
                    cache,
                    snapshot,
                    pointer_events,
                    router,
                },
                &|_| true,
            )
        }

        /// Shared manager-owned inputs specialized to this detail family.
        pub type $managed_draw<'a> =
            $crate::strategic_window_common::ManagedDetailWindowsDraw<'a, $state>;

        /// Manager-owned drawing with command-center child visibility.
        pub fn $managed_shown(
            input: $managed_draw<'_>,
            shown: &dyn Fn(GalaxyChild) -> bool,
        ) -> Vec<$action> {
            let (draw, state) = input.with_visibility(shown);
            $draw(draw, state)
        }
    };
}

pub(crate) use define_detail_window_draw_entrypoints;

macro_rules! unpack_detail_windows_draw {
    (
        $draw:expr => $ctx:ident $world:ident $fog:ident $missions:ident $faction:ident
        $layout:ident $cache:ident $snapshot:ident $pointer_events:ident $router:ident $shown:ident
    ) => {
        let $crate::strategic_window_common::DetailWindowsDraw {
            ctx: $ctx,
            world: $world,
            fog: $fog,
            missions: $missions,
            faction: $faction,
            layout: $layout,
            cache: $cache,
            snapshot: $snapshot,
            pointer_events: $pointer_events,
            router: mut $router,
            shown: $shown,
        } = $draw;
    };
}

pub(crate) use unpack_detail_windows_draw;

impl<'a, T> DetailWindowDraw<'a, T> {
    pub(crate) fn new(
        ctx: &'a egui::Context,
        world: &'a rebellion_core::world::GameWorld,
        window: &'a T,
        focused: bool,
        faction: CockpitFaction,
        layout: CockpitLayout,
        cache: &'a mut BmpCache,
    ) -> Self {
        Self {
            ctx,
            world,
            window,
            focused,
            faction,
            layout,
            cache,
        }
    }

    pub(crate) fn parts(
        self,
    ) -> (
        &'a egui::Context,
        &'a rebellion_core::world::GameWorld,
        &'a T,
        bool,
        CockpitFaction,
        CockpitLayout,
        &'a mut BmpCache,
    ) {
        (
            self.ctx,
            self.world,
            self.window,
            self.focused,
            self.faction,
            self.layout,
            self.cache,
        )
    }
}

/// Unpack a detail draw and initialize its shared frame/result vocabulary.
macro_rules! begin_detail_window_draw {
    (
        $draw:expr =>
        $ctx:ident $world:ident $window:ident $focused:ident $faction:ident
        $layout:ident $cache:ident $result:ident $system:ident $scale:ident
        $screen_rect:ident $id:ident $player:ident : $result_type:ty
    ) => {
        let ($ctx, $world, $window, $focused, $faction, $layout, $cache) = $draw.parts();
        let mut $result = <$result_type>::default();
        $crate::strategic_window_common::draw_frame!(
            $result, $system, $scale, $screen_rect, $id;
            $world, $window, $layout
        );
        let $player = $faction.player();
    };
}

pub(crate) use begin_detail_window_draw;

impl StrategicWindowCanvas<'_> {
    pub(crate) fn input_parts(&self) -> (egui::Rect, egui::Response, egui::Painter) {
        (self.local, self.response.clone(), self.painter.clone())
    }

    pub(crate) fn paint(&self, cache: &mut BmpCache, source: DllSource, id: u32, x: f32, y: f32) {
        paint_native(
            &self.painter,
            self.ctx,
            cache,
            source,
            id,
            self.local,
            self.scale,
            x,
            y,
        );
    }

    /// Paint the opaque black shell and its family-specific background art.
    pub(crate) fn paint_background(&self, cache: &mut BmpCache, resource: u32) {
        self.painter
            .rect_filled(self.local, 0.0, egui::Color32::BLACK);
        self.paint(cache, DllSource::Strategy, resource, 0.0, 0.0);
    }

    /// Paint the shared faction title strip, its label, and the three title
    /// controls. Callers retain the family-specific meaning of each click.
    pub(crate) fn paint_title_bar<K: Copy + Hash>(
        &self,
        ui: &mut egui::Ui,
        cache: &mut BmpCache,
        side: u8,
        focused: bool,
        label: TitleLabel<'_>,
        buttons: [(u32, u32, f32, &'static str); 3],
        id_seed: K,
        register: impl FnMut(usize, egui::Rect),
    ) -> [bool; 3] {
        self.paint(
            cache,
            DllSource::Strategy,
            title_resource(side, focused),
            2.0,
            2.0,
        );
        self.painter.text(
            logical_rect(self.local, self.scale, label.at.0, label.at.1, 0.0, 0.0).min,
            label.align,
            label.text,
            egui::FontId::proportional((label.font_size * self.scale).max(label.minimum_size)),
            egui::Color32::BLACK,
        );
        self.title_buttons(ui, cache, DllSource::Strategy, buttons, id_seed, register)
    }

    /// Paint and interact with the shared Sector/Minimize/Close title controls.
    pub(crate) fn title_buttons<K: Copy + Hash>(
        &self,
        ui: &mut egui::Ui,
        cache: &mut BmpCache,
        source: DllSource,
        buttons: [(u32, u32, f32, &'static str); 3],
        id_seed: K,
        mut register: impl FnMut(usize, egui::Rect),
    ) -> [bool; 3] {
        let mut clicked = [false; 3];
        for (index, (normal, pressed, x, name)) in buttons.into_iter().enumerate() {
            let rect = logical_rect(self.local, self.scale, x, 3.0, 14.0, 14.0);
            register(index, rect);
            let response = ui.interact(rect, ui.id().with((id_seed, name)), egui::Sense::click());
            let down =
                self.primary_down && self.pointer.is_some_and(|point| rect_contains(rect, point));
            self.paint(cache, source, if down { pressed } else { normal }, x, 3.0);
            clicked[index] = exact_clicked(&response, rect);
        }
        clicked
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TitleLabel<'a> {
    pub(crate) text: &'a str,
    pub(crate) at: (f32, f32),
    pub(crate) align: egui::Align2,
    pub(crate) font_size: f32,
    pub(crate) minimum_size: f32,
}

impl<'a> TitleLabel<'a> {
    pub(crate) const fn new(
        text: &'a str,
        at: (f32, f32),
        align: egui::Align2,
        font_size: f32,
        minimum_size: f32,
    ) -> Self {
        Self {
            text,
            at,
            align,
            font_size,
            minimum_size,
        }
    }

    /// The common font-5, format-0x24 strategic title label.
    pub(crate) const fn left_centered(text: &'a str, x: f32, y: f32, height: f32) -> Self {
        Self::new(
            text,
            (x, y + height / 2.0),
            egui::Align2::LEFT_CENTER,
            11.0,
            7.0,
        )
    }
}

/// Allocate the common strategic detail-window shell, then draw its content.
pub(crate) fn show_canvas<R>(
    ctx: &egui::Context,
    area_id: egui::Id,
    screen_rect: egui::Rect,
    scale: f32,
    draw: impl FnOnce(&mut egui::Ui, &StrategicWindowCanvas<'_>) -> R,
) -> egui::InnerResponse<R> {
    egui::Area::new(area_id)
        .fixed_pos(screen_rect.min)
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            let (pointer, primary_down) = ctx.input(|input| {
                (
                    input.pointer.interact_pos(),
                    input.pointer.button_down(egui::PointerButton::Primary),
                )
            });
            let (local, response) =
                ui.allocate_exact_size(screen_rect.size(), egui::Sense::click());
            let canvas = StrategicWindowCanvas {
                ctx,
                local,
                response,
                painter: ui.painter().clone(),
                scale,
                pointer,
                primary_down,
            };
            draw(ui, &canvas)
        })
}

/// Claim the first matching pointer event in the caller's semantic order.
pub(crate) fn claim_pointer_event(
    router: &mut FrameInputRouter,
    events: impl IntoIterator<Item = (InputEventId, PointerPhase)>,
    window: WindowId,
    phase: PointerPhase,
) -> Option<InputEventId> {
    events.into_iter().find_map(|(event, candidate)| {
        (candidate == phase
            && router.pointer_recipient(event) == InputRecipient::Window(window)
            && router.claim(event, InputRecipient::Window(window)))
        .then_some(event)
    })
}

pub(crate) enum RoutedPrimaryRelease {
    Waiting {
        button_down: bool,
    },
    Unclaimed,
    Released {
        point: Option<egui::Pos2>,
        ctrl: bool,
        claimed_by: Option<WindowId>,
    },
}

/// Freeze primary-release input and claim the matching manager event before a
/// window mutates drag payload. The caller chooses event iteration order.
pub(crate) fn routed_primary_release(
    ctx: &egui::Context,
    owner: Option<WindowId>,
    router: Option<&mut FrameInputRouter>,
    events: impl IntoIterator<Item = (InputEventId, PointerPhase)>,
) -> RoutedPrimaryRelease {
    let (released, button_down, point, ctrl) = ctx.input(|input| {
        (
            input.pointer.primary_released(),
            input.pointer.primary_down(),
            input.pointer.latest_pos(),
            input.modifiers.ctrl,
        )
    });
    if !released {
        return RoutedPrimaryRelease::Waiting { button_down };
    }
    let claimed_by = if let (Some(owner), Some(router)) = (owner, router) {
        if claim_pointer_event(
            router,
            events,
            owner,
            PointerPhase::Release(PointerButtonId::Primary),
        )
        .is_none()
        {
            return RoutedPrimaryRelease::Unclaimed;
        }
        Some(owner)
    } else {
        None
    };
    RoutedPrimaryRelease::Released {
        point,
        ctrl,
        claimed_by,
    }
}

pub(crate) struct RoutedDragRelease<T> {
    pub(crate) payload: Option<T>,
    pub(crate) point: Option<egui::Pos2>,
    pub(crate) ctrl: bool,
    pub(crate) owner: Option<WindowId>,
    pub(crate) claimed_by: Option<WindowId>,
}

pub(crate) struct RoutedDrop<T> {
    pub(crate) payload: T,
    pub(crate) point: egui::Pos2,
    pub(crate) ctrl: bool,
}

pub(crate) struct RoutedDragResult<A, T> {
    pub(crate) actions: Vec<A>,
    pub(crate) claimed_primary_release_by: Option<WindowId>,
    pub(crate) drop: Option<RoutedDrop<T>>,
}

pub(crate) trait RoutedDragPayload {
    fn press(&self) -> egui::Pos2;
    fn list(&self) -> egui::Rect;
}

impl<A, T> Default for RoutedDragResult<A, T> {
    fn default() -> Self {
        Self {
            actions: Vec::new(),
            claimed_primary_release_by: None,
            drop: None,
        }
    }
}

impl<A, T> RoutedDragResult<A, T> {
    pub(crate) fn take_drop(&mut self) -> Option<(T, egui::Pos2, bool)> {
        self.drop
            .take()
            .map(|drop| (drop.payload, drop.point, drop.ctrl))
    }
}

/// Clear a capture payload only when the manager cancellation names its
/// current owner.
pub(crate) fn cancel_owned_capture<T>(
    payload: &mut Option<T>,
    owner: &mut Option<WindowId>,
    cancelled: WindowId,
) {
    if *owner == Some(cancelled) {
        *payload = None;
        *owner = None;
    }
}

/// The mutable payload, manager owner and input router participating in one
/// routed drag. Grouping them prevents each window family from rebuilding the
/// same lifecycle call shape.
pub(crate) struct RoutedDrag<'a, T> {
    drag: &'a mut Option<T>,
    owner: &'a mut Option<WindowId>,
    ctx: &'a egui::Context,
    router: Option<&'a mut FrameInputRouter>,
}

pub(crate) struct RoutedDragFrame<'a> {
    pub(crate) ctx: &'a egui::Context,
    pub(crate) scale: f32,
    pub(crate) router: Option<&'a mut FrameInputRouter>,
    pub(crate) pointer_events: &'a [(InputEventId, PointerPhase)],
}

impl<'a> RoutedDragFrame<'a> {
    pub(crate) fn new(
        ctx: &'a egui::Context,
        scale: f32,
        router: Option<&'a mut FrameInputRouter>,
        pointer_events: &'a [(InputEventId, PointerPhase)],
    ) -> Self {
        Self {
            ctx,
            scale,
            router,
            pointer_events,
        }
    }
}

impl<'a, T> RoutedDrag<'a, T> {
    pub(crate) fn new(
        drag: &'a mut Option<T>,
        owner: &'a mut Option<WindowId>,
        ctx: &'a egui::Context,
        router: Option<&'a mut FrameInputRouter>,
    ) -> Self {
        Self {
            drag,
            owner,
            ctx,
            router,
        }
    }

    /// Complete the common capture lifecycle. A missing payload preserves its
    /// owner, matching the states' fail-closed handling of invalid receipts.
    fn take(
        &mut self,
        events: impl IntoIterator<Item = (InputEventId, PointerPhase)>,
    ) -> Option<RoutedDragRelease<T>> {
        let (point, ctrl, claimed_by) =
            match routed_primary_release(self.ctx, *self.owner, self.router.as_deref_mut(), events)
            {
                RoutedPrimaryRelease::Waiting { button_down } => {
                    if !button_down && self.owner.is_none() {
                        *self.drag = None;
                    }
                    return None;
                }
                RoutedPrimaryRelease::Unclaimed => return None,
                RoutedPrimaryRelease::Released {
                    point,
                    ctrl,
                    claimed_by,
                } => (point, ctrl, claimed_by),
            };
        let payload = self.drag.take();
        let released_owner = payload.as_ref().and_then(|_| self.owner.take());
        Some(RoutedDragRelease {
            payload,
            point,
            ctrl,
            owner: released_owner,
            claimed_by,
        })
    }
}

impl<T: RoutedDragPayload> RoutedDrag<'_, T> {
    /// Emit the common capture-release action and retain a drop only when the
    /// family-specific distance test accepts its release point.
    pub(crate) fn complete<A>(
        mut self,
        events: impl IntoIterator<Item = (InputEventId, PointerPhase)>,
        release_action: impl FnOnce(WindowId) -> A,
        scale: f32,
        minimum_distance_squared: f32,
    ) -> Option<RoutedDragResult<A, T>> {
        let release = self.take(events)?;
        let drop = match (release.payload, release.point) {
            (Some(payload), Some(point)) => {
                let moved = (point - payload.press()) / scale;
                (moved.length_sq() > minimum_distance_squared
                    && !rect_contains(payload.list(), point))
                .then_some(RoutedDrop {
                    payload,
                    point,
                    ctrl: release.ctrl,
                })
            }
            _ => None,
        };
        Some(RoutedDragResult {
            actions: release.owner.map(release_action).into_iter().collect(),
            claimed_primary_release_by: release.claimed_by,
            drop,
        })
    }
}

macro_rules! complete_window_drag_events {
    ($state:expr, $frame:expr, $events:expr, $release:path) => {
        $crate::strategic_window_common::RoutedDrag::new(
            &mut $state.drag,
            &mut $state.drag_owner,
            $frame.ctx,
            $frame.router.as_deref_mut(),
        )
        .complete(
            $events,
            $release,
            $frame.scale,
            $crate::fleet_window::DRAG_DISTANCE_SQUARED,
        )
        .unwrap_or_default()
    };
}

pub(crate) use complete_window_drag_events;

/// Complete a private window state's standard routed drag capture.
macro_rules! complete_window_drag {
    ($state:expr, $frame:expr, forward, $release:path) => {
        $crate::strategic_window_common::complete_window_drag_events!(
            $state,
            $frame,
            $frame.pointer_events.iter().copied(),
            $release
        )
    };
    ($state:expr, $frame:expr, reverse, $release:path) => {
        $crate::strategic_window_common::complete_window_drag_events!(
            $state,
            $frame,
            $frame.pointer_events.iter().rev().copied(),
            $release
        )
    };
}

pub(crate) use complete_window_drag;

pub(crate) fn managed_layer_id<T: StrategicWindowRecord>(
    windows: &[T],
    id: WindowId,
    area_id: impl Fn(&T) -> egui::Id,
) -> Option<egui::LayerId> {
    windows
        .iter()
        .find(|window| window.identity().0 == Some(id))
        .map(|window| egui::LayerId::new(egui::Order::Foreground, area_id(window)))
}

pub(crate) fn update_title_drag<T: StrategicWindowRecord>(
    windows: &mut [T],
    active: &mut Option<TitleDrag>,
    ctx: &egui::Context,
    layout: CockpitLayout,
    area_id: impl Fn(&T) -> egui::Id,
) {
    let Some(drag) = *active else {
        return;
    };
    let window = windows
        .iter_mut()
        .find(|window| area_id(window) == drag.area);
    match (window, title_drag_step(ctx, layout, drag)) {
        (Some(window), Ok(Some(position))) => window.set_logical_position(position),
        (Some(_), Ok(None)) => {}
        _ => *active = None,
    }
}

pub(crate) fn commit_open<T: StrategicWindowRecord>(
    windows: &mut Vec<T>,
    faction: &mut CockpitFaction,
    requested_faction: CockpitFaction,
    system: SystemKey,
    subject: SubjectId,
    transition: &WindowTransition,
    make_window: impl FnOnce(WindowId, SubjectId) -> T,
    update_existing: impl FnOnce(&mut T),
) -> Option<WindowId> {
    match *transition {
        WindowTransition::Opened { id } => {
            if windows.iter().any(|window| {
                let identity = window.identity();
                identity.0 == Some(id) || identity.1 == Some(subject) || identity.2 == system
            }) {
                return None;
            }
            if windows.is_empty() {
                *faction = requested_faction;
            } else if *faction != requested_faction {
                return None;
            }
            windows.push(make_window(id, subject));
            Some(id)
        }
        WindowTransition::Existing { id, .. } => {
            if *faction != requested_faction {
                return None;
            }
            let window = windows.iter_mut().find(|window| {
                let identity = window.identity();
                identity.0 == Some(id) && identity.1 == Some(subject) && identity.2 == system
            })?;
            update_existing(window);
            Some(id)
        }
        _ => None,
    }
}

/// Bind the structural fields shared by ordinary prepared detail candidates
/// while leaving record construction and existing-content updates explicit in
/// the owning family.
macro_rules! commit_prepared_detail {
    ($state:expr, $prepared:ident, $transition:expr, $make_window:expr, $update_existing:expr $(,)?) => {{
        let subject = $prepared.spec.key.subject?;
        $crate::strategic_window_common::commit_open(
            &mut ($state).windows,
            &mut ($state).faction,
            $prepared.faction,
            $prepared.system,
            subject,
            $transition,
            $make_window,
            $update_existing,
        )
    }};
}

pub(crate) use commit_prepared_detail;

pub(crate) fn window_id<T: StrategicWindowRecord>(
    windows: &[T],
    system: SystemKey,
) -> Option<WindowId> {
    windows
        .iter()
        .find(|window| window.identity().2 == system)
        .and_then(|window| window.identity().0)
}

pub(crate) fn is_empty<T>(windows: &[T]) -> bool {
    windows.is_empty()
}

pub(crate) fn contains_window<T: StrategicWindowRecord>(windows: &[T], id: WindowId) -> bool {
    windows.iter().any(|window| window.identity().0 == Some(id))
}

pub(crate) fn is_open<T: StrategicWindowRecord>(windows: &[T], system: SystemKey) -> bool {
    windows.iter().any(|window| window.identity().2 == system)
}

pub(crate) fn screen_rect<T: StrategicWindowRecord>(
    windows: &[T],
    system: SystemKey,
    rect: impl Fn(&T) -> egui::Rect,
) -> Option<egui::Rect> {
    windows
        .iter()
        .find(|window| window.identity().2 == system)
        .map(rect)
}

pub(crate) fn record_screen_rect<T: StrategicWindowRecord>(
    window: &T,
    layout: CockpitLayout,
    logical_size: (f32, f32),
) -> egui::Rect {
    let position = window.logical_position();
    egui::Rect::from_min_size(
        egui::pos2(
            layout.canvas.x + f32::from(position.0) * layout.scale,
            layout.canvas.y + f32::from(position.1) * layout.scale,
        ),
        egui::vec2(logical_size.0 * layout.scale, logical_size.1 * layout.scale),
    )
}

pub(crate) fn release_target<T: StrategicWindowRecord>(
    windows: &[T],
    layer: egui::LayerId,
    admitted: impl Fn(&T) -> bool,
    area_id: impl Fn(&T) -> egui::Id,
) -> Option<SystemKey> {
    windows
        .iter()
        .find(|window| admitted(window) && area_id(window) == layer.id)
        .map(|window| window.identity().2)
}

pub(crate) fn window_is_visible<T: StrategicWindowRecord>(
    windows: &[T],
    snapshot: &RoutingSnapshot,
    system: SystemKey,
) -> bool {
    windows.iter().any(|window| {
        let identity = window.identity();
        identity.2 == system && identity.0.is_none_or(|id| snapshot.is_visible(id))
    })
}

pub(crate) fn remove_content<T: StrategicWindowRecord>(windows: &mut Vec<T>, removed: &[WindowId]) {
    windows.retain(|window| window.identity().0.is_none_or(|id| !removed.contains(&id)));
}

pub(crate) fn content_reports<T: StrategicWindowRecord>(
    windows: &[T],
    snapshot: &RoutingSnapshot,
    kind: WindowKind,
    ready: impl Fn(&T, WindowId) -> bool,
) -> Vec<ContentReport> {
    snapshot
        .paint_order()
        .iter()
        .chain(snapshot.presentation_history())
        .copied()
        .filter(|id| snapshot.key(*id).is_some_and(|key| key.kind == kind))
        .map(|id| ContentReport {
            window: id,
            status: if windows
                .iter()
                .any(|window| window.identity().0 == Some(id) && ready(window, id))
            {
                ContentStatus::Ready
            } else {
                ContentStatus::Absent
            },
        })
        .collect()
}

/// Manager reports for system-backed content whose stable DAT subject must
/// still agree with both the live world and the manager key.
pub(crate) fn system_content_reports<T: StrategicWindowRecord>(
    windows: &[T],
    snapshot: &RoutingSnapshot,
    world: &rebellion_core::world::GameWorld,
    kind: WindowKind,
) -> Vec<ContentReport> {
    content_reports(windows, snapshot, kind, |window, id| {
        let (_, subject, system) = window.identity();
        world.systems.contains_key(system)
            && system_subject(world, system) == subject
            && snapshot.key(id).is_some_and(|key| key.subject == subject)
    })
}

/// Publish the identical manager-report API of system-backed detail states.
macro_rules! impl_system_content_reports {
    ($state:ty, $kind:path) => {
        impl $state {
            #[must_use]
            pub fn content_reports(
                &self,
                snapshot: &rebellion_windowing::RoutingSnapshot,
                world: &rebellion_core::world::GameWorld,
            ) -> Vec<rebellion_windowing::ContentReport> {
                $crate::strategic_window_common::system_content_reports(
                    &self.windows,
                    snapshot,
                    world,
                    $kind,
                )
            }
        }
    };
}

pub(crate) use impl_system_content_reports;

/// Project system-backed detail records into shown save records while
/// preserving their local order.
pub(crate) fn saved_system_windows<T: StrategicWindowRecord>(
    windows: &[T],
    world: &rebellion_core::world::GameWorld,
    kind: impl Fn(DatId) -> SavedWindowKind,
) -> Vec<SavedWindow> {
    windows
        .iter()
        .filter_map(|window| {
            let (_, _, system) = window.identity();
            Some(SavedWindow {
                kind: kind(world.systems.get(system)?.dat_id),
                position: window.logical_position(),
                shown: true,
            })
        })
        .collect()
}

/// Resolve a shown saved detail record of the expected family back to its
/// live runtime system. Restore adapters can then own only their distinct
/// default content.
pub(crate) fn shown_saved_system(
    world: &rebellion_core::world::GameWorld,
    saved: &SavedWindow,
    expected: fn(SavedWindowKind) -> Option<DatId>,
) -> Option<SystemKey> {
    let dat_id = saved.shown.then(|| expected(saved.kind)).flatten()?;
    world
        .systems
        .iter()
        .find_map(|(key, system)| (system.dat_id == dat_id).then_some(key))
}

/// Bind the private state operations shared by shown saved detail restores;
/// the family retains ownership of the content record it creates afterward.
macro_rules! restore_shown_detail_system {
    ($state:expr, $world:expr, $faction:expr, $saved:expr, $kind:ident) => {{
        let Some(system) =
            $crate::strategic_window_common::shown_saved_system($world, $saved, |saved_kind| {
                match saved_kind {
                    rebellion_core::saved_interface::SavedWindowKind::$kind { system } => {
                        Some(system)
                    }
                    _ => None,
                }
            })
        else {
            return false;
        };
        $state.prepare_faction($faction);
        $state.close(system);
        system
    }};
}

pub(crate) use restore_shown_detail_system;

/// Publish the identical shown-save API of system-backed detail states.
macro_rules! impl_saved_system_windows {
    ($state:ty, $kind:expr) => {
        impl $state {
            #[must_use]
            pub fn saved_windows(
                &self,
                world: &rebellion_core::world::GameWorld,
            ) -> Vec<rebellion_core::saved_interface::SavedWindow> {
                $crate::strategic_window_common::saved_system_windows(&self.windows, world, $kind)
            }
        }
    };
}

pub(crate) use impl_saved_system_windows;

pub(crate) fn contains_shown_screen_point<T: StrategicWindowRecord>(
    windows: &[T],
    point: (f32, f32),
    shown: impl Fn(SystemKey) -> bool,
    screen_rect: impl Fn(&T) -> egui::Rect,
) -> bool {
    let point = egui::pos2(point.0, point.1);
    windows
        .iter()
        .filter(|window| shown(window.identity().2))
        .any(|window| rect_contains(screen_rect(window), point))
}

pub(crate) fn contains_visible_screen_point_where<T: StrategicWindowRecord>(
    windows: &[T],
    snapshot: &RoutingSnapshot,
    point: (f32, f32),
    shown: impl Fn(&T) -> bool,
    screen_rect: impl Fn(&T) -> egui::Rect,
) -> bool {
    let point = egui::pos2(point.0, point.1);
    windows.iter().any(|window| {
        window.identity().0.is_none_or(|id| snapshot.is_visible(id))
            && shown(window)
            && rect_contains(screen_rect(window), point)
    })
}

pub(crate) fn hit_window_ids_where<T: StrategicWindowRecord>(
    windows: &[T],
    snapshot: &RoutingSnapshot,
    point: (f32, f32),
    top_layer: Option<egui::LayerId>,
    shown: impl Fn(WindowId) -> bool,
    area_id: impl Fn(&T) -> egui::Id,
    screen_rect: impl Fn(&T) -> egui::Rect,
) -> BTreeSet<WindowId> {
    let point = egui::pos2(point.0, point.1);
    windows
        .iter()
        .filter(|window| top_layer.is_some_and(|layer| layer.id == area_id(window)))
        .filter_map(|window| {
            window.identity().0.filter(|id| {
                snapshot.is_visible(*id) && shown(*id) && rect_contains(screen_rect(window), point)
            })
        })
        .collect()
}

pub(crate) fn focus<T: StrategicWindowRecord>(windows: &mut Vec<T>, system: SystemKey) -> bool {
    focus_where(windows, |window| window.identity().2 == system)
}

pub(crate) fn focus_direct_system<T: StrategicWindowRecord>(
    windows: &mut Vec<T>,
    system: SystemKey,
) -> bool {
    focus_where(windows, |window| {
        window.identity().0.is_none() && window.identity().2 == system
    })
}

pub(crate) fn focus_where<T>(windows: &mut Vec<T>, matches: impl Fn(&T) -> bool) -> bool {
    let Some(index) = windows.iter().position(matches) else {
        return false;
    };
    let window = windows.remove(index);
    windows.push(window);
    true
}

/// Validate a direct fixture open and raise matching content when it already
/// exists. `None` means the caller must construct the new family-specific
/// record; `Some` is the final open result.
pub(crate) fn begin_direct_open<T>(
    windows: &mut Vec<T>,
    subject_exists: bool,
    matches: impl Fn(&T) -> bool,
) -> Option<bool> {
    if !subject_exists {
        return Some(false);
    }
    focus_where(windows, matches).then_some(true)
}

/// Open one direct fixture window, constructing family content only when the
/// subject exists and no matching record is already open.
pub(crate) fn open_direct<T>(
    windows: &mut Vec<T>,
    subject_exists: bool,
    matches: impl Fn(&T) -> bool,
    create: impl FnOnce() -> T,
) -> bool {
    if let Some(opened) = begin_direct_open(windows, subject_exists, matches) {
        return opened;
    }
    windows.push(create());
    true
}

/// Open one system-backed direct fixture record after validating its live
/// world key. Family-specific matching decides whether manager-owned content
/// may satisfy the compatibility open.
pub(crate) fn open_direct_system<T>(
    windows: &mut Vec<T>,
    world: &rebellion_core::world::GameWorld,
    system: SystemKey,
    matches: impl Fn(&T) -> bool,
    create: impl FnOnce() -> T,
) -> bool {
    open_direct(windows, world.systems.contains_key(system), matches, create)
}

/// Bind the shared state/faction setup for a direct system fixture while the
/// owning module supplies its private record construction.
macro_rules! open_direct_system_window {
    ($state:expr, $world:expr, $system:expr, $faction:expr, $matches:expr, $create:expr $(,)?) => {{
        $state.prepare_faction($faction);
        $crate::strategic_window_common::open_direct_system(
            &mut $state.windows,
            $world,
            $system,
            $matches,
            $create,
        )
    }};
}

pub(crate) use open_direct_system_window;

pub(crate) fn close<T: StrategicWindowRecord>(
    windows: &mut Vec<T>,
    system: SystemKey,
) -> Option<T> {
    let index = windows
        .iter()
        .position(|window| window.identity().2 == system)?;
    Some(windows.remove(index))
}

pub(crate) fn window_mut<T: StrategicWindowRecord>(
    windows: &mut [T],
    system: SystemKey,
) -> Option<&mut T> {
    windows
        .iter_mut()
        .find(|window| window.identity().2 == system)
}
