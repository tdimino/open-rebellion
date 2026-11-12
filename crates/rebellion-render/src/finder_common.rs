//! Shared mechanics for the original strategic Finder windows.
//!
//! Fleet, Personnel and Troop Finder have different rows, tabs, resources and
//! destinations, but the original gives them the same 470 by 330 strategic
//! window frame, name-prefix selection, manager routing and list navigation.

use egui_macroquad::egui;
use rebellion_core::dat::Faction;
use rebellion_windowing::{
    ContentReport, ControlId, ExistingEffect, FocusTarget, RoutingSnapshot, WindowId, WindowKind,
    WindowTransition,
};

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{CockpitFaction, CockpitLayout};
use crate::fleet_window::paint_native;
use crate::mission_dialog::{button, galaxy_centered_rect};
use crate::native_geometry::{logical_rect, rect_contains};
use crate::singleton_window_common;

/// The rendering and manager vocabulary every original Finder family needs.
/// Family modules import this prelude and keep only their domain-specific
/// world types beside it, so adding another Finder does not recreate the same
/// scaffolding imports.
pub(crate) mod prelude {
    pub(crate) use super::{BitmapControl as Control, FinderRowName, FinderUiState};
    pub(crate) use crate::bmp_cache::BmpCache;
    pub(crate) use crate::cockpit::{CockpitFaction, CockpitLayout};
    pub(crate) use egui_macroquad::egui;
    pub(crate) use rebellion_core::world::GameWorld;
    pub(crate) use rebellion_windowing::{ControlId, FocusTarget, WindowId, WindowKind};
}

pub(crate) const WIDTH: f32 = 470.0;
pub(crate) const HEIGHT: f32 = 330.0;
pub(crate) const BACKGROUND: [u32; 2] = [10335, 10336];
pub(crate) const PANEL_AT: (f32, f32) = (12.0, 13.0);
pub(crate) const RAIL_AT: (f32, f32) = (412.0, 0.0);
pub(crate) const TITLE_AT: (f32, f32) = (36.0, 14.0);
pub(crate) const NAME_LABEL_AT: (f32, f32) = (36.0, 48.0);
pub(crate) const NAME_BOX: LogicalRect = (143.0, 45.0, 250.0, 18.0);
pub(crate) const SIDE_TABS: [BitmapControl; 2] = [
    BitmapControl::new((36.0, 72.0, 49.0, 41.0), (10502, 10503)),
    BitmapControl::new((88.0, 72.0, 49.0, 41.0), (10505, 10506)),
];

/// Publish one Finder family's stable public dimensions and child-control
/// identities without repeating the same alias block in every family module.
macro_rules! define_finder_interface {
    ($width:ident, $height:ident, $name_control:ident, $list_control:ident) => {
        pub const $width: f32 = $crate::finder_common::WIDTH;
        pub const $height: f32 = $crate::finder_common::HEIGHT;
        /// Stable child identity for the name edit (`0xcb`).
        pub const $name_control: rebellion_windowing::ControlId =
            rebellion_windowing::ControlId(0xcb);
        /// Stable child identity for the result list (`0xca`).
        pub const $list_control: rebellion_windowing::ControlId =
            rebellion_windowing::ControlId(0xca);
    };
}

pub(crate) use define_finder_interface;

/// Define the two-side tab used by Finder families whose lists are split
/// strictly between Alliance and Imperial content.
///
/// Fleet Finder deliberately does not use this macro: its third `All` tab has
/// different filtering semantics.
macro_rules! define_side_tab {
    ($(#[$meta:meta])* $visibility:vis enum $name:ident {
        alliance: $alliance_label:literal,
        imperial: $imperial_label:literal $(,)?
    }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        $visibility enum $name {
            #[default]
            Alliance,
            Imperial,
        }

        impl $name {
            const ALL: [Self; 2] = [Self::Alliance, Self::Imperial];

            fn lists(self, is_alliance: bool) -> bool {
                $crate::finder_common::lists_side(self.index(), is_alliance)
            }

            const fn label(self) -> &'static str {
                match self {
                    Self::Alliance => $alliance_label,
                    Self::Imperial => $imperial_label,
                }
            }

            const fn index(self) -> usize {
                match self {
                    Self::Alliance => 0,
                    Self::Imperial => 1,
                }
            }
        }
    };
}

pub(crate) use define_side_tab;

pub(crate) type LogicalRect = (f32, f32, f32, f32);

/// The bitmap-backed canvas shared by every original Finder surface.
pub(crate) struct FinderCanvas<'a> {
    ctx: &'a egui::Context,
    pub(crate) frame: egui::Rect,
    pub(crate) painter: egui::Painter,
    pub(crate) scale: f32,
}

impl FinderCanvas<'_> {
    pub(crate) fn paint(&self, cache: &mut BmpCache, id: u32, at: (f32, f32)) {
        paint_native(
            &self.painter,
            self.ctx,
            cache,
            DllSource::Strategy,
            id,
            self.frame,
            self.scale,
            at.0,
            at.1,
        );
    }

    pub(crate) fn visible_rows<'a, R>(
        &self,
        rows: &'a [R],
        first_row: usize,
        visible_rows: usize,
        list: LogicalRect,
        row_height: f32,
    ) -> impl Iterator<Item = (usize, &'a R, egui::Rect)> {
        let frame = self.frame;
        let scale = self.scale;
        let end = rows.len().min(first_row.saturating_add(visible_rows));
        (first_row..end).map(move |index| {
            let line = (index - first_row) as f32;
            let rect = at(
                frame,
                scale,
                (list.0, list.1 + row_height * line, list.2, row_height),
            );
            (index, &rows[index], rect)
        })
    }

    /// Paint the three white text labels shared by every Finder and return
    /// the two original-font substitutes used by the rest of the surface.
    pub(crate) fn paint_header(
        &self,
        title: &str,
        name_label: &str,
        section: (&str, (f32, f32)),
    ) -> FinderFonts {
        let fonts = FinderFonts {
            title: egui::FontId::proportional((12.0 * self.scale).max(8.0)),
            body: egui::FontId::proportional((10.0 * self.scale).max(7.0)),
        };
        for (text, position, font) in [
            (title, TITLE_AT, &fonts.title),
            (name_label, NAME_LABEL_AT, &fonts.body),
            (section.0, section.1, &fonts.title),
        ] {
            self.painter.text(
                at(self.frame, self.scale, (position.0, position.1, 0.0, 0.0)).min,
                egui::Align2::LEFT_TOP,
                text,
                font.clone(),
                egui::Color32::WHITE,
            );
        }
        fonts
    }

    pub(crate) fn button(
        &self,
        ui: &egui::Ui,
        cache: &mut BmpCache,
        control: BitmapControl,
        id: impl std::hash::Hash,
        down: bool,
    ) -> bool {
        button(
            ui,
            cache,
            at(self.frame, self.scale, control.rect),
            id,
            control.art,
            down,
            self.scale,
        )
    }
}

pub(crate) struct FinderFonts {
    pub(crate) title: egui::FontId,
    pub(crate) body: egui::FontId,
}

/// Allocate and paint the common Finder canvas before family-specific content.
pub(crate) fn show_canvas<R>(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    area_id: egui::Id,
    layout: CockpitLayout,
    background: u32,
    draw: impl FnOnce(&mut egui::Ui, &FinderCanvas<'_>, &mut BmpCache) -> R,
) -> R {
    let rect = window_rect(layout);
    crate::native_geometry::show_area(
        ctx,
        area_id,
        rect,
        egui::Order::Tooltip,
        egui::Sense::click(),
        |ui, frame, painter| {
            let canvas = FinderCanvas {
                ctx,
                frame,
                painter,
                scale: layout.scale,
            };
            canvas.painter.rect_filled(frame, 0.0, egui::Color32::BLACK);
            canvas.paint(cache, background, (0.0, 0.0));
            draw(ui, &canvas, cache)
        },
    )
    .inner
}

pub(crate) struct IndexedFinderConfig<'a, Tab> {
    pub(crate) area_id: &'static str,
    pub(crate) name_id: &'static str,
    pub(crate) list_id: &'static str,
    pub(crate) row_id: &'static str,
    pub(crate) title: &'a str,
    pub(crate) name_label: &'a str,
    pub(crate) section_label: &'a str,
    pub(crate) section_at: (f32, f32),
    pub(crate) panel: u32,
    pub(crate) rail: u32,
    pub(crate) tabs: &'a [Tab],
    pub(crate) tab_controls: &'a [BitmapControl],
    pub(crate) controls: (BitmapControl, BitmapControl),
    pub(crate) name_box: LogicalRect,
    pub(crate) list: LogicalRect,
    pub(crate) row_height: f32,
    pub(crate) visible_rows: usize,
    pub(crate) semantic_controls: (ControlId, ControlId),
    pub(crate) accepts_batched_double_click: bool,
}

/// Static contract for the two-side indexed Finders. Dynamic art and labels
/// stay in [`SideFinderPresentation`] because they depend on the active tab or
/// sub-view.
pub(crate) struct SideFinderConfig<Tab: 'static> {
    pub(crate) area_id: &'static str,
    pub(crate) name_id: &'static str,
    pub(crate) list_id: &'static str,
    pub(crate) row_id: &'static str,
    pub(crate) title: &'static str,
    pub(crate) name_label: &'static str,
    pub(crate) tabs: &'static [Tab; 2],
    pub(crate) list: LogicalRect,
    pub(crate) row_height: f32,
    pub(crate) visible_rows: usize,
    pub(crate) semantic_controls: (ControlId, ControlId),
}

#[derive(Clone, Copy)]
pub(crate) struct SideFinderPresentation<'a> {
    pub(crate) section_label: &'a str,
    pub(crate) section_at: (f32, f32),
    pub(crate) panel: u32,
    pub(crate) rail: u32,
    pub(crate) controls: (BitmapControl, BitmapControl),
}

#[derive(Clone, Copy)]
pub(crate) struct SideFinderLook {
    panel: u32,
    rail: u32,
    controls: (BitmapControl, BitmapControl),
}

impl SideFinderLook {
    pub(crate) fn new(
        panel: u32,
        side_index: usize,
        rail: [u32; 2],
        controls: (BitmapControl, BitmapControl),
    ) -> Self {
        Self {
            panel,
            rail: rail[side_index],
            controls,
        }
    }
}

macro_rules! side_finder_presentation {
    ($label:expr, $at:expr, $tab:expr; $panel:expr, $side:expr, $rail:expr, $controls:expr) => {
        $crate::finder_common::SideFinderPresentation::with_colours(
            $label,
            $at,
            $tab,
            $crate::finder_common::SideFinderLook::new($panel, $side, $rail, $controls),
        )
    };
}

pub(crate) use side_finder_presentation;

impl<'a> SideFinderPresentation<'a> {
    pub(crate) fn with_colours(
        section_label: &'a str,
        section_at: (f32, f32),
        tab_index: usize,
        look: SideFinderLook,
    ) -> (Self, (egui::Color32, egui::Color32)) {
        (
            Self {
                section_label,
                section_at,
                panel: look.panel,
                rail: look.rail,
                controls: look.controls,
            },
            count_cell_colours(tab_index),
        )
    }
}

pub(crate) struct SideFinderFrame<'a, Row, Tab> {
    layout: CockpitLayout,
    focus: Option<FocusTarget>,
    snapshot: &'a FinderUiState,
    tab: Tab,
    chosen: Option<usize>,
    rows: &'a [Row],
}

impl<'a, Row, Tab> SideFinderFrame<'a, Row, Tab> {
    pub(crate) fn new(
        layout: CockpitLayout,
        focus: Option<FocusTarget>,
        snapshot: &'a FinderUiState,
        tab: Tab,
        chosen: Option<usize>,
        rows: &'a [Row],
    ) -> Self {
        Self {
            layout,
            focus,
            snapshot,
            tab,
            chosen,
            rows,
        }
    }
}

/// Bind the structural fields and count-cell colours shared by Personnel and
/// Troop Finder without exposing their private record types.
macro_rules! draw_coloured_side_finder_window {
    (
        $ctx:expr, $cache:expr, $layout:expr, $focus:expr,
        $window:expr, $rows:expr, $config:expr, $presentation:expr;
        extra_controls: $extra_controls:expr, paint_details: $paint_details:expr;
        row_action: $row_action:expr $(,)?
    ) => {{
        let finder_window = &$window;
        let finder_rows = &$rows;
        let (presentation, colours) = $presentation;
        $crate::finder_common::draw_side_finder(
            $ctx,
            $cache,
            &$crate::finder_common::SideFinderFrame::new(
                $layout,
                $focus,
                &finder_window.ui,
                finder_window.tab,
                finder_window.chosen,
                finder_rows,
            ),
            $config,
            presentation,
            $extra_controls,
            |canvas, painter, row, line, font| {
                ($paint_details)(canvas, painter, row, line, font, colours)
            },
            $row_action,
        )
    }};
}

pub(crate) use draw_coloured_side_finder_window;

/// Clone one family's private window snapshot and bind its side-specific
/// controls inside the owning module.
macro_rules! side_finder_snapshot {
    ($state:expr, $controls:expr) => {{
        let window = $state.window.clone()?;
        let side_index = $crate::finder_common::side(window.ui.faction);
        let controls = &$controls[side_index];
        (window, side_index, controls)
    }};
}

pub(crate) use side_finder_snapshot;

macro_rules! side_finder_config {
    (
        $tab:ty, $tabs:expr;
        area: $area_id:literal, name: $name_id:literal, list_id: $list_id:literal,
        row: $row_id:literal, title: $title:literal, name_label: $name_label:literal;
        list: $list:expr, row_height: $row_height:expr, visible_rows: $visible_rows:expr;
        controls: ($name_control:expr, $list_control:expr) $(,)?
    ) => {
        $crate::finder_common::SideFinderConfig::<$tab> {
            area_id: $area_id,
            name_id: $name_id,
            list_id: $list_id,
            row_id: $row_id,
            title: $title,
            name_label: $name_label,
            tabs: $tabs,
            list: $list,
            row_height: $row_height,
            visible_rows: $visible_rows,
            semantic_controls: ($name_control, $list_control),
        }
    };
}

pub(crate) use side_finder_config;

pub(crate) struct IndexedFinderOutput<Tab, Action> {
    pub(crate) ui: FinderUiState,
    pub(crate) chosen: Option<usize>,
    pub(crate) selection_touched: bool,
    pub(crate) tab: Option<Tab>,
    pub(crate) open: Option<Action>,
    pub(crate) close: bool,
}

/// The family-independent result after an indexed Finder commits its common
/// UI, selection and side-tab state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FinderDrawOutcome<Action> {
    Open(Action),
    Close,
}

#[allow(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "This is the shared ordered control pass for indexed Finder families."
)]
pub(crate) fn draw_indexed_finder<Row, Tab, Action>(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    layout: CockpitLayout,
    focus: Option<FocusTarget>,
    snapshot: &FinderUiState,
    current_tab: Tab,
    current_chosen: Option<usize>,
    current_open: Option<Action>,
    rows: &[Row],
    config: &IndexedFinderConfig<'_, Tab>,
    mut extra_controls: impl FnMut(&mut egui::Ui, &FinderCanvas<'_>, &mut BmpCache),
    mut paint_details: impl FnMut(&FinderCanvas<'_>, &egui::Painter, &Row, f32, &egui::FontId),
    row_action: impl Fn(&Row) -> Action,
) -> IndexedFinderOutput<Tab, Action>
where
    Row: FinderRowName,
    Tab: Copy + PartialEq,
    Action: Copy,
{
    let focused_control = focused_control(snapshot, focus);
    let side_index = side(snapshot.faction);
    let scale = layout.scale;
    let name_id = egui::Id::new(config.name_id);
    let list_id = egui::Id::new(config.list_id);
    let name_should_focus =
        snapshot.focus_name || focused_control == Some(config.semantic_controls.0);
    let name_had_focus = ctx.memory(|memory| memory.has_focus(name_id));
    let deferred_name_input = if name_should_focus && !name_had_focus {
        ctx.input(|input| {
            input
                .events
                .iter()
                .filter_map(|event| match event {
                    egui::Event::Text(text) if text != "\n" && text != "\r" => Some(text.as_str()),
                    egui::Event::Paste(text) => Some(text.as_str()),
                    _ => None,
                })
                .collect::<String>()
        })
    } else {
        String::new()
    };
    if focused_control == Some(config.semantic_controls.1) {
        ctx.memory_mut(|memory| memory.request_focus(list_id));
    } else if name_should_focus {
        ctx.memory_mut(|memory| memory.request_focus(name_id));
    }

    let mut close = false;
    let mut open = None;
    let mut next_tab = None;
    let mut interaction = FinderInteraction::new(snapshot);
    show_canvas(
        ctx,
        cache,
        egui::Id::new(config.area_id),
        layout,
        BACKGROUND[side_index],
        |ui, canvas, cache| {
            canvas.paint(cache, config.panel, PANEL_AT);
            canvas.paint(cache, config.rail, RAIL_AT);
            let font = canvas
                .paint_header(
                    config.title,
                    config.name_label,
                    (config.section_label, config.section_at),
                )
                .body;

            for (index, candidate) in config.tabs.iter().copied().enumerate() {
                if canvas.button(
                    ui,
                    cache,
                    config.tab_controls[index],
                    (config.row_id, "side-tab", index),
                    candidate == current_tab,
                ) {
                    next_tab = Some(candidate);
                }
            }
            extra_controls(ui, canvas, cache);
            if canvas.button(
                ui,
                cache,
                config.controls.0,
                (config.row_id, "close"),
                false,
            ) {
                close = true;
            }
            if canvas.button(
                ui,
                cache,
                config.controls.1,
                (config.row_id, "display"),
                false,
            ) {
                open = current_open;
            }

            interaction.edit_name(
                ui,
                at(canvas.frame, scale, config.name_box),
                (name_id, list_id),
                &font,
                snapshot,
                focused_control,
                config.semantic_controls,
            );
            if !interaction.name_changed && !deferred_name_input.is_empty() {
                interaction.name.push_str(&deferred_name_input);
                interaction.name_changed = true;
            }

            let list_rect = at(canvas.frame, scale, config.list);
            let list_painter = canvas.painter.with_clip_rect(list_rect);
            let cell_font = egui::FontId::proportional((10.0 * scale).max(7.0));
            for (offset, row, row_rect) in canvas.visible_rows(
                rows,
                snapshot.first_row,
                config.visible_rows,
                config.list,
                config.row_height,
            ) {
                let line = (offset - snapshot.first_row) as f32;
                paint_row_name(
                    &list_painter,
                    row_rect,
                    scale,
                    row.finder_name(),
                    &font,
                    current_chosen == Some(offset),
                );
                paint_details(canvas, &list_painter, row, line, &cell_font);
                if interaction.interact_row(
                    ui,
                    ctx,
                    row_rect,
                    (config.row_id, "row", offset),
                    snapshot,
                    offset,
                    config.accepts_batched_double_click,
                    config.semantic_controls.1,
                ) {
                    open = Some(row_action(row));
                }
            }
            interaction.wheel = list_wheel(ui, list_rect);
        },
    );

    let keys = FinderKeys::read(ctx);
    let selection_touched = interaction.name_changed
        || interaction.clicked_row.is_some()
        || (!rows.is_empty()
            && ((focused_control == Some(config.semantic_controls.0) && keys.arrow_down)
                || (focused_control == Some(config.semantic_controls.1)
                    && (keys.arrow_down || (keys.arrow_up && current_chosen.unwrap_or(0) > 0)))));
    let mut ui_state = snapshot.clone();
    let mut chosen = current_chosen;
    let navigation = interaction.finish_indexed_navigation(
        &mut ui_state,
        &mut chosen,
        rows,
        focused_control,
        config.semantic_controls,
        keys,
        config.visible_rows,
        config
            .tabs
            .iter()
            .position(|candidate| *candidate == current_tab)
            .unwrap_or(0),
        config.tabs.len(),
    );
    if navigation.list_enter {
        open = open.or_else(|| selected_row(rows, chosen).map(&row_action));
    }
    if let Some(index) = navigation.horizontal {
        next_tab = config.tabs.get(index).copied();
    }
    if navigation.edit_enter {
        open = open
            .or_else(|| selected_row(rows, chosen).map(row_action))
            .or(current_open);
    }
    IndexedFinderOutput {
        ui: ui_state,
        chosen,
        selection_touched,
        tab: next_tab,
        open,
        close: close || keys.escape,
    }
}

/// Draw a standard two-side Finder, supplying the common tabs, name box,
/// click semantics and selected-row action in one place.
#[allow(
    clippy::too_many_arguments,
    reason = "Render context, frozen focus and family callbacks are separate Finder contracts."
)]
pub(crate) fn draw_side_finder<Row, Tab, Action>(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    frame: &SideFinderFrame<'_, Row, Tab>,
    config: &SideFinderConfig<Tab>,
    presentation: SideFinderPresentation<'_>,
    extra_controls: impl FnMut(&mut egui::Ui, &FinderCanvas<'_>, &mut BmpCache),
    paint_details: impl FnMut(&FinderCanvas<'_>, &egui::Painter, &Row, f32, &egui::FontId),
    row_action: impl Fn(&Row) -> Action + Copy,
) -> IndexedFinderOutput<Tab, Action>
where
    Row: FinderRowName,
    Tab: Copy + PartialEq,
    Action: Copy,
{
    draw_indexed_finder(
        ctx,
        cache,
        frame.layout,
        frame.focus,
        frame.snapshot,
        frame.tab,
        frame.chosen,
        selected_row(frame.rows, frame.chosen).map(row_action),
        frame.rows,
        &IndexedFinderConfig {
            area_id: config.area_id,
            name_id: config.name_id,
            list_id: config.list_id,
            row_id: config.row_id,
            title: config.title,
            name_label: config.name_label,
            section_label: presentation.section_label,
            section_at: presentation.section_at,
            panel: presentation.panel,
            rail: presentation.rail,
            tabs: config.tabs,
            tab_controls: &SIDE_TABS,
            controls: presentation.controls,
            name_box: NAME_BOX,
            list: config.list,
            row_height: config.row_height,
            visible_rows: config.visible_rows,
            semantic_controls: config.semantic_controls,
            accepts_batched_double_click: false,
        },
        extra_controls,
        paint_details,
        row_action,
    )
}

/// One original bitmap control: its logical rectangle and normal/pressed art.
#[derive(Debug, Clone, Copy)]
pub(crate) struct BitmapControl {
    pub(crate) rect: LogicalRect,
    pub(crate) art: (u32, u32),
}

impl BitmapControl {
    pub(crate) const fn new(rect: LogicalRect, art: (u32, u32)) -> Self {
        Self { rect, art }
    }
}

/// The row text used by the original name-prefix selection routine.
pub(crate) trait FinderRowName {
    fn finder_name(&self) -> &str;
}

/// One original Finder count cell. The port currently has no per-unit usable
/// bit, so populated rows initialize `all_usable` to true.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FinderCount {
    pub count: u32,
    pub all_usable: bool,
}

/// Count side-filtered, classed objects into the columns declared by a Finder
/// family. Missing objects and classes outside the displayed columns are
/// ignored, matching the original list builders.
pub(crate) fn count_classes<K: Copy, const N: usize>(
    keys: &[K],
    classes: &[u32; N],
    side_index: usize,
    mut classed_object: impl FnMut(K) -> Option<(bool, u32)>,
) -> [FinderCount; N] {
    let mut counts = [FinderCount {
        count: 0,
        all_usable: true,
    }; N];
    for &key in keys {
        let Some((is_alliance, class)) = classed_object(key) else {
            continue;
        };
        if !lists_side(side_index, is_alliance) {
            continue;
        }
        if let Some(index) = classes.iter().position(|candidate| *candidate == class) {
            counts[index].count += 1;
        }
    }
    counts
}

/// Generate a thin world-collection adapter around [`count_classes`]. A macro
/// is appropriate here because Rust functions cannot abstract over a struct
/// field name without introducing a trait solely for this local operation.
macro_rules! define_world_class_counter {
    ($function:ident, $key:ty, $tab:ty, $count:ty, $columns:literal, $classes:path, $field:ident) => {
        fn $function(
            world: &rebellion_core::world::GameWorld,
            keys: &[$key],
            tab: $tab,
        ) -> [$count; $columns] {
            $crate::finder_common::count_classes(keys, $classes(tab), tab.index(), |key| {
                world
                    .$field
                    .get(key)
                    .map(|object| (object.is_alliance, object.class_dat_id.raw()))
            })
        }
    };
}

pub(crate) use define_world_class_counter;

/// Keep only keys whose live objects belong to the displayed Finder side.
pub(crate) fn side_keys<K: Copy>(
    keys: &[K],
    side_index: usize,
    mut object_side: impl FnMut(K) -> Option<bool>,
) -> Vec<K> {
    keys.iter()
        .copied()
        .filter(|&key| {
            object_side(key).is_some_and(|is_alliance| lists_side(side_index, is_alliance))
        })
        .collect()
}

/// Map a system/fleet row to its family-specific destination.
pub(crate) fn location_action<Action>(
    system: rebellion_core::ids::SystemKey,
    fleet: Option<rebellion_core::ids::FleetKey>,
    open_fleet: impl FnOnce(rebellion_core::ids::SystemKey, rebellion_core::ids::FleetKey) -> Action,
    open_system: impl FnOnce(rebellion_core::ids::SystemKey) -> Action,
) -> Action {
    match fleet {
        Some(fleet) => open_fleet(system, fleet),
        None => open_system(system),
    }
}

/// State shared by all three original Finder window families.
#[derive(Debug, Clone)]
pub(crate) struct FinderUiState {
    /// Strategic-window identity accepted at the app commit boundary.
    pub(crate) id: Option<WindowId>,
    pub(crate) faction: CockpitFaction,
    pub(crate) name: String,
    pub(crate) first_row: usize,
    pub(crate) focus_name: bool,
    pub(crate) pending_focus: Option<ControlId>,
    pub(crate) last_clicked: Option<(usize, f64)>,
}

impl FinderUiState {
    pub(crate) fn new(id: Option<WindowId>, faction: CockpitFaction) -> Self {
        Self {
            id,
            faction,
            name: String::new(),
            first_row: 0,
            focus_name: true,
            pending_focus: None,
            last_clicked: None,
        }
    }

    pub(crate) const fn window_id(&self) -> Option<WindowId> {
        self.id
    }

    pub(crate) fn managed_layer_id(
        &self,
        id: WindowId,
        layer_name: &'static str,
    ) -> Option<egui::LayerId> {
        singleton_window_common::managed_layer_id(self.id, id, layer_name)
    }

    pub(crate) fn hit_window(
        &self,
        frame: &RoutingSnapshot,
        kind: WindowKind,
        layout: CockpitLayout,
        point: (f32, f32),
    ) -> Option<WindowId> {
        singleton_window_common::hit_window(self.id, frame, kind, window_rect(layout), point)
    }

    pub(crate) fn keyboard_owner(&self, frame: &RoutingSnapshot) -> Option<WindowId> {
        singleton_window_common::keyboard_owner(self.id, frame)
    }

    pub(crate) fn take_focus_request(&mut self) -> Option<FocusTarget> {
        Some(FocusTarget {
            window: self.id?,
            control: Some(self.pending_focus.take()?),
        })
    }

    pub(crate) fn contains_screen_point(layout: CockpitLayout, point: (f32, f32)) -> bool {
        rect_contains(window_rect(layout), egui::pos2(point.0, point.1))
    }

    pub(crate) fn reset_list_navigation(&mut self) {
        self.first_row = 0;
        self.last_clicked = None;
    }

    pub(crate) fn commit_focus_and_wheel(
        &mut self,
        focus_request: Option<ControlId>,
        wheel: f32,
        row_count: usize,
        visible_rows: usize,
    ) {
        if let Some(control) = focus_request {
            self.pending_focus = Some(control);
        }
        if wheel != 0.0 {
            let last = row_count.saturating_sub(visible_rows);
            self.first_row = if wheel > 0.0 {
                self.first_row.saturating_sub(1)
            } else {
                (self.first_row + 1).min(last)
            };
        }
    }
}

/// Apply a Finder tab or mode change and reset the selection/list state that
/// belongs to the previous category. Returns whether anything changed.
pub(crate) fn change_category<Category: PartialEq, Selection>(
    current: &mut Category,
    chosen: &mut Option<Selection>,
    ui: &mut FinderUiState,
    next: Category,
) -> bool {
    if *current == next {
        return false;
    }
    *current = next;
    *chosen = None;
    ui.reset_list_navigation();
    true
}

/// Commit the common output of an indexed Finder to its family model.
/// `selection` translates a visible row index into the identity retained by
/// that family (an index for Personnel/Troop, an object for Fleet).
pub(crate) fn commit_indexed_output<Row, Tab, Action, Selection>(
    ui: &mut FinderUiState,
    tab: &mut Tab,
    chosen: &mut Option<Selection>,
    rows: &[Row],
    output: IndexedFinderOutput<Tab, Action>,
    selection: impl Fn(usize, &Row) -> Selection,
) -> Option<FinderDrawOutcome<Action>>
where
    Tab: PartialEq,
{
    *ui = output.ui;
    if output.selection_touched {
        *chosen = output
            .chosen
            .and_then(|index| rows.get(index).map(|row| selection(index, row)));
    }
    if let Some(next_tab) = output.tab {
        change_category(tab, chosen, ui, next_tab);
    }
    if let Some(action) = output.open {
        Some(FinderDrawOutcome::Open(action))
    } else if output.close {
        Some(FinderDrawOutcome::Close)
    } else {
        None
    }
}

/// Family-specific Finder content that embeds the common manager/UI state.
pub(crate) trait FinderContent {
    fn finder_ui(&self) -> &FinderUiState;
    fn finder_ui_mut(&mut self) -> &mut FinderUiState;
}

macro_rules! impl_finder_content {
    ($content:ty) => {
        impl $crate::finder_common::FinderContent for $content {
            fn finder_ui(&self) -> &$crate::finder_common::FinderUiState {
                &self.ui
            }

            fn finder_ui_mut(&mut self) -> &mut $crate::finder_common::FinderUiState {
                &mut self.ui
            }
        }
    };
}

pub(crate) use impl_finder_content;

/// Publish the common manager-facing API for one Finder family.
macro_rules! impl_finder_state_shell {
    ($state:ty, $kind:path, $layer_name:literal) => {
        impl $state {
            /// Exact manager identity currently owning the published content.
            #[must_use]
            pub fn window_id(&self) -> Option<rebellion_windowing::WindowId> {
                $crate::finder_common::window_id(self.window.as_ref())
            }

            /// Egui layer attached to this exact manager-issued Finder.
            #[must_use]
            pub fn managed_layer_id(
                &self,
                id: rebellion_windowing::WindowId,
            ) -> Option<egui_macroquad::egui::LayerId> {
                $crate::finder_common::managed_layer_id(self.window.as_ref(), id, $layer_name)
            }

            /// Exact focused manager identity that receives this frame's keys.
            #[must_use]
            pub fn keyboard_owner(
                &self,
                frame: &rebellion_windowing::RoutingSnapshot,
            ) -> Option<rebellion_windowing::WindowId> {
                $crate::finder_common::keyboard_owner(self.window.as_ref(), frame)
            }

            /// Manager-owned Finder hit for event-local pointer routing.
            #[must_use]
            pub fn hit_window(
                &self,
                frame: &rebellion_windowing::RoutingSnapshot,
                layout: $crate::cockpit::CockpitLayout,
                point: (f32, f32),
            ) -> Option<rebellion_windowing::WindowId> {
                $crate::finder_common::hit_window(self.window.as_ref(), frame, $kind, layout, point)
            }

            /// Explicit content state for this Finder in the immutable frame.
            #[must_use]
            pub fn content_reports(
                &self,
                frame: &rebellion_windowing::RoutingSnapshot,
            ) -> Vec<rebellion_windowing::ContentReport> {
                $crate::finder_common::content_reports_for(self.window.as_ref(), frame, $kind)
            }

            /// Release only content named by manager reconciliation receipts.
            pub fn remove_content(&mut self, removed: &[rebellion_windowing::WindowId]) {
                $crate::finder_common::remove_content(&mut self.window, removed);
            }

            /// Take one child-focus intent scoped to the exact owning identity.
            pub fn take_focus_request(&mut self) -> Option<rebellion_windowing::FocusTarget> {
                $crate::finder_common::take_focus_request(self.window.as_mut())
            }

            /// The window's screen rectangle while it is open.
            #[must_use]
            pub fn screen_rect(
                &self,
                layout: $crate::cockpit::CockpitLayout,
            ) -> Option<egui_macroquad::egui::Rect> {
                $crate::finder_common::screen_rect(self.window.as_ref(), layout)
            }

            /// Whether `point` falls on the open Finder surface.
            #[must_use]
            pub fn contains_screen_point(
                &self,
                layout: $crate::cockpit::CockpitLayout,
                point: (f32, f32),
            ) -> bool {
                $crate::finder_common::contains_screen_point(self.window.as_ref(), layout, point)
            }

            #[cfg(any(test, feature = "test-support"))]
            pub(crate) fn fixture_ui(&self) -> Option<&$crate::finder_common::FinderUiState> {
                self.window
                    .as_ref()
                    .map(|window| $crate::finder_common::FinderContent::finder_ui(window))
            }

            #[cfg(any(test, feature = "test-support"))]
            pub(crate) fn fixture_ui_mut(
                &mut self,
            ) -> Option<&mut $crate::finder_common::FinderUiState> {
                self.window
                    .as_mut()
                    .map(|window| $crate::finder_common::FinderContent::finder_ui_mut(window))
            }
        }
    };
}

pub(crate) use impl_finder_state_shell;

/// Publish the shared singleton lifecycle for one Finder family.
macro_rules! impl_finder_lifecycle {
    ($state:ident, $prepared:ident, $kind:path, |$faction:ident, $window_id:ident| $content:block) => {
        impl $prepared {
            #[must_use]
            pub const fn spec(self) -> rebellion_windowing::WindowSpec {
                $crate::windowing_policy::singleton_spec(
                    $kind,
                    rebellion_windowing::WindowOwner::StrategicRoot,
                )
            }
        }

        impl $state {
            /// Prepare inert content for a manager-owned singleton open.
            #[must_use]
            pub const fn prepare_open(faction: $crate::cockpit::CockpitFaction) -> $prepared {
                $prepared { faction }
            }

            /// Publish prepared content for a new identity, preserving a duplicate.
            pub fn commit_open(
                &mut self,
                prepared: $prepared,
                transition: &rebellion_windowing::WindowTransition,
            ) -> Option<rebellion_windowing::WindowId> {
                match $crate::finder_common::accept_singleton_open(transition, self.window_id())? {
                    $crate::finder_common::OpenCommit::Publish(id) => {
                        let $faction = prepared.faction;
                        let $window_id = Some(id);
                        self.window = Some($content);
                        Some(id)
                    }
                    $crate::finder_common::OpenCommit::Existing(id) => Some(id),
                }
            }

            /// Open direct fixture content unless the Finder is already open.
            pub fn open(&mut self, faction: $crate::cockpit::CockpitFaction) -> bool {
                if self.window.is_some() {
                    return false;
                }
                let $faction = faction;
                let $window_id = None;
                self.window = Some($content);
                true
            }

            pub fn close(&mut self) {
                self.window = None;
            }

            #[must_use]
            pub fn is_open(&self) -> bool {
                self.window.is_some()
            }
        }
    };
}

pub(crate) use impl_finder_lifecycle;

pub(crate) fn window_id<T: FinderContent>(window: Option<&T>) -> Option<WindowId> {
    window.and_then(|window| window.finder_ui().window_id())
}

pub(crate) fn managed_layer_id<T: FinderContent>(
    window: Option<&T>,
    id: WindowId,
    layer_name: &'static str,
) -> Option<egui::LayerId> {
    window?.finder_ui().managed_layer_id(id, layer_name)
}

pub(crate) fn keyboard_owner<T: FinderContent>(
    window: Option<&T>,
    frame: &RoutingSnapshot,
) -> Option<WindowId> {
    window?.finder_ui().keyboard_owner(frame)
}

pub(crate) fn hit_window<T: FinderContent>(
    window: Option<&T>,
    frame: &RoutingSnapshot,
    kind: WindowKind,
    layout: CockpitLayout,
    point: (f32, f32),
) -> Option<WindowId> {
    window?.finder_ui().hit_window(frame, kind, layout, point)
}

pub(crate) fn content_reports_for<T: FinderContent>(
    window: Option<&T>,
    frame: &RoutingSnapshot,
    kind: WindowKind,
) -> Vec<ContentReport> {
    content_reports(frame, kind, window_id(window))
}

pub(crate) fn remove_content<T: FinderContent>(window: &mut Option<T>, removed: &[WindowId]) {
    if should_remove(window_id(window.as_ref()), removed) {
        *window = None;
    }
}

pub(crate) fn take_focus_request<T: FinderContent>(window: Option<&mut T>) -> Option<FocusTarget> {
    window?.finder_ui_mut().take_focus_request()
}

pub(crate) fn screen_rect<T>(window: Option<&T>, layout: CockpitLayout) -> Option<egui::Rect> {
    window.map(|_| window_rect(layout))
}

pub(crate) fn contains_screen_point<T>(
    window: Option<&T>,
    layout: CockpitLayout,
    point: (f32, f32),
) -> bool {
    window.is_some() && FinderUiState::contains_screen_point(layout, point)
}

pub(crate) fn content_reports(
    frame: &RoutingSnapshot,
    kind: WindowKind,
    current: Option<WindowId>,
) -> Vec<ContentReport> {
    singleton_window_common::content_reports(frame, kind, |id| current == Some(id))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OpenCommit {
    Publish(WindowId),
    Existing(WindowId),
}

pub(crate) fn accept_singleton_open(
    transition: &WindowTransition,
    current: Option<WindowId>,
) -> Option<OpenCommit> {
    match *transition {
        WindowTransition::Opened { id } if current.is_none() => Some(OpenCommit::Publish(id)),
        WindowTransition::Existing {
            id,
            effect: ExistingEffect::NoOp,
        } if current == Some(id) => Some(OpenCommit::Existing(id)),
        _ => None,
    }
}

pub(crate) fn prefix_choice<R: FinderRowName>(rows: &[R], text: &str) -> Option<usize> {
    let typed: Vec<char> = text.chars().map(|c| c.to_ascii_lowercase()).collect();
    let mut best: Option<(usize, usize)> = None;
    for (index, row) in rows.iter().enumerate() {
        let common = row
            .finder_name()
            .chars()
            .map(|c| c.to_ascii_lowercase())
            .zip(&typed)
            .take_while(|(a, b)| a == *b)
            .count();
        if common > 0 && best.is_none_or(|(_, length)| common > length) {
            best = Some((index, common));
        }
    }
    best.map(|(index, _)| index)
}

/// One frame's Finder navigation keys.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct FinderKeys {
    pub(crate) escape: bool,
    pub(crate) arrow_up: bool,
    pub(crate) arrow_down: bool,
    pub(crate) arrow_left: bool,
    pub(crate) arrow_right: bool,
    pub(crate) enter: bool,
}

impl FinderKeys {
    pub(crate) fn read(ctx: &egui::Context) -> Self {
        let [escape, arrow_up, arrow_down, arrow_left, arrow_right, enter] = ctx.input(|input| {
            [
                egui::Key::Escape,
                egui::Key::ArrowUp,
                egui::Key::ArrowDown,
                egui::Key::ArrowLeft,
                egui::Key::ArrowRight,
                egui::Key::Enter,
            ]
            .map(|key| input.key_pressed(key))
        });
        Self {
            escape,
            arrow_up,
            arrow_down,
            arrow_left,
            arrow_right,
            enter,
        }
    }
}

pub(crate) fn focused_control(
    state: &FinderUiState,
    focus: Option<FocusTarget>,
) -> Option<ControlId> {
    state
        .id
        .and_then(|id| focus.filter(|target| target.window == id))
        .and_then(|target| target.control)
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct NameEditOutcome {
    pub(crate) changed: bool,
    pub(crate) enter: bool,
    pub(crate) focus_request: Option<ControlId>,
}

pub(crate) struct FinderInteraction {
    pub(crate) name: String,
    pub(crate) name_changed: bool,
    pub(crate) enter: bool,
    pub(crate) wheel: f32,
    pub(crate) clicked_row: Option<(usize, f64)>,
    pub(crate) focus_request: Option<ControlId>,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct FinderNavigation {
    pub(crate) list_enter: bool,
    pub(crate) edit_enter: bool,
    pub(crate) horizontal: Option<usize>,
}

impl FinderInteraction {
    pub(crate) fn new(state: &FinderUiState) -> Self {
        Self {
            name: state.name.clone(),
            name_changed: false,
            enter: false,
            wheel: 0.0,
            clicked_row: None,
            focus_request: None,
        }
    }

    pub(crate) fn record_name_edit(&mut self, edit: NameEditOutcome) {
        self.name_changed = edit.changed;
        self.enter = edit.enter;
        self.focus_request = edit.focus_request.or(self.focus_request);
    }

    pub(crate) fn edit_name(
        &mut self,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        widget_ids: (egui::Id, egui::Id),
        font: &egui::FontId,
        state: &FinderUiState,
        focused: Option<ControlId>,
        controls: (ControlId, ControlId),
    ) {
        let (name_id, list_id) = widget_ids;
        let (name_control, list_control) = controls;
        if focused == Some(list_control) {
            ui.memory_mut(|memory| memory.request_focus(list_id));
        }
        let response = ui.put(
            rect,
            egui::TextEdit::singleline(&mut self.name)
                .id(name_id)
                .frame(false)
                .font(font.clone())
                .text_color(egui::Color32::WHITE)
                .desired_width(rect.width()),
        );
        if state.focus_name || focused == Some(name_control) {
            response.request_focus();
        }
        let edit = NameEditOutcome {
            changed: response.changed(),
            enter: response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter)),
            focus_request: (state.id.is_some() && response.gained_focus()).then_some(name_control),
        };
        self.record_name_edit(edit);
    }

    /// Record the common selection/focus half of a row click. True asks the
    /// family to perform its row-specific open action.
    pub(crate) fn record_row_click(
        &mut self,
        click: RowClick,
        row: usize,
        managed: bool,
        list_control: ControlId,
    ) -> bool {
        match click {
            RowClick::Open => true,
            RowClick::Select(at) => {
                self.clicked_row = Some((row, at));
                if managed {
                    self.focus_request = Some(list_control);
                }
                false
            }
            RowClick::None => false,
        }
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "Row geometry, identity and semantic focus are distinct Finder contracts."
    )]
    pub(crate) fn interact_row(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        row_rect: egui::Rect,
        row_id: impl std::hash::Hash,
        state: &FinderUiState,
        row: usize,
        accepts_batched_double_click: bool,
        list_control: ControlId,
    ) -> bool {
        let click = row_click(
            ui,
            ctx,
            row_rect,
            ui.id().with(row_id),
            state.last_clicked,
            row,
            accepts_batched_double_click,
        );
        self.record_row_click(click, row, state.id.is_some(), list_control)
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "Selection, focus controls, key state and tab count are separate Finder contracts."
    )]
    pub(crate) fn finish_indexed_navigation<R: FinderRowName>(
        &mut self,
        ui: &mut FinderUiState,
        chosen: &mut Option<usize>,
        rows: &[R],
        focused_control: Option<ControlId>,
        controls: (ControlId, ControlId),
        keys: FinderKeys,
        visible_rows: usize,
        tab_index: usize,
        tab_count: usize,
    ) -> FinderNavigation {
        let (name_control, list_control) = controls;
        ui.focus_name = false;
        if self.name_changed {
            ui.name.clone_from(&self.name);
            *chosen = prefix_choice(rows, &self.name);
            if let Some(index) = *chosen {
                ui.first_row = crate::list_control::reveal_row(ui.first_row, index, visible_rows);
            }
        }
        if let Some((row, at)) = self.clicked_row {
            ui.last_clicked = Some((row, at));
            *chosen = Some(row);
            set_name(ui, rows[row].finder_name());
        }
        if focused_control == Some(name_control) && keys.arrow_down && !rows.is_empty() {
            let row = (*chosen).unwrap_or(0).min(rows.len() - 1);
            *chosen = Some(row);
            set_name(ui, rows[row].finder_name());
            ui.first_row = crate::list_control::reveal_row(ui.first_row, row, visible_rows);
            self.focus_request = Some(list_control);
        }
        if focused_control == Some(list_control) {
            if rows.is_empty() {
                if keys.arrow_up {
                    self.focus_request = Some(name_control);
                }
            } else {
                let current = (*chosen).unwrap_or(0).min(rows.len() - 1);
                let next = if keys.arrow_down {
                    (current + 1).min(rows.len() - 1)
                } else if keys.arrow_up && current > 0 {
                    current - 1
                } else {
                    current
                };
                if keys.arrow_up && current == 0 {
                    self.focus_request = Some(name_control);
                } else if keys.arrow_up || keys.arrow_down {
                    *chosen = Some(next);
                    set_name(ui, rows[next].finder_name());
                    ui.first_row =
                        crate::list_control::reveal_row(ui.first_row, next, visible_rows);
                }
            }
        }
        let list_focused = focused_control == Some(controls.1);
        let navigation = FinderNavigation {
            list_enter: list_focused && !rows.is_empty() && keys.enter,
            edit_enter: self.enter || (focused_control == Some(controls.0) && keys.enter),
            horizontal: list_focused
                .then(|| horizontal_choice(keys, tab_index, tab_count))
                .flatten(),
        };
        ui.commit_focus_and_wheel(self.focus_request, self.wheel, rows.len(), visible_rows);
        navigation
    }
}

pub(crate) fn selected_row<R>(rows: &[R], chosen: Option<usize>) -> Option<&R> {
    rows.get(chosen?)
}

pub(crate) fn horizontal_choice(
    keys: FinderKeys,
    current: usize,
    choice_count: usize,
) -> Option<usize> {
    if keys.arrow_left {
        Some(current.saturating_sub(1))
    } else if keys.arrow_right {
        Some((current + 1).min(choice_count - 1))
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum RowClick {
    None,
    Select(f64),
    Open,
}

/// Interpret one Finder row's click and double-click semantics.
pub(crate) fn row_click(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    row_rect: egui::Rect,
    row_id: egui::Id,
    last_clicked: Option<(usize, f64)>,
    row: usize,
    accepts_batched_double_click: bool,
) -> RowClick {
    let response = ui.interact(row_rect, row_id, egui::Sense::click());
    let batched_double_click = accepts_batched_double_click
        && response.double_clicked()
        && ui.input(|input| {
            input
                .events
                .iter()
                .filter(|event| {
                    matches!(
                        event,
                        egui::Event::PointerButton {
                            pos,
                            button: egui::PointerButton::Primary,
                            pressed: false,
                            ..
                        } if row_rect.contains(*pos)
                    )
                })
                .count()
                >= 2
        });
    if batched_double_click {
        return RowClick::Open;
    }
    if !response.clicked() {
        return RowClick::None;
    }
    let now = ui.input(|input| input.time);
    let delay = ctx.options(|options| options.input_options.max_double_click_delay);
    if last_clicked.is_some_and(|(last, at)| last == row && now - at <= delay) {
        RowClick::Open
    } else {
        RowClick::Select(now)
    }
}

pub(crate) fn list_wheel(ui: &egui::Ui, list_rect: egui::Rect) -> f32 {
    ui.input(|input| {
        if input
            .pointer
            .hover_pos()
            .is_some_and(|point| rect_contains(list_rect, point))
        {
            input.raw_scroll_delta.y
        } else {
            0.0
        }
    })
}

pub(crate) fn paint_row_name(
    painter: &egui::Painter,
    row_rect: egui::Rect,
    scale: f32,
    name: &str,
    font: &egui::FontId,
    selected: bool,
) {
    painter.text(
        row_rect.left_center() + egui::vec2(4.0 * scale, 0.0),
        egui::Align2::LEFT_CENTER,
        name,
        font.clone(),
        if selected {
            egui::Color32::YELLOW
        } else {
            egui::Color32::WHITE
        },
    );
}

/// Paint one original Finder count cell: a dash for zero or a boxed number.
pub(crate) fn paint_count_cell(
    painter: &egui::Painter,
    frame: egui::Rect,
    scale: f32,
    at: (f32, f32),
    count: u32,
    font: &egui::FontId,
    colours: (egui::Color32, egui::Color32),
) {
    if count == 0 {
        let cell_pos = self::at(frame, scale, (at.0, at.1, 28.0, 0.0)).min;
        painter.text(
            cell_pos + egui::vec2(10.0 * scale, 0.0),
            egui::Align2::CENTER_TOP,
            "-",
            font.clone(),
            egui::Color32::WHITE,
        );
    } else {
        let box_rect = self::at(frame, scale, (at.0, at.1, 21.0, 16.0));
        painter.rect_filled(box_rect, 0.0, colours.0);
        painter.text(
            box_rect.center(),
            egui::Align2::CENTER_CENTER,
            count.to_string(),
            font.clone(),
            colours.1,
        );
    }
}

#[allow(
    clippy::too_many_arguments,
    reason = "Finder count-grid geometry is explicit data."
)]
pub(crate) fn paint_count_cells(
    painter: &egui::Painter,
    frame: egui::Rect,
    scale: f32,
    list_origin: (f32, f32),
    row_height: f32,
    line: f32,
    first_column: f32,
    counts: impl IntoIterator<Item = u32>,
    font: &egui::FontId,
    colours: (egui::Color32, egui::Color32),
) {
    for (index, count) in counts.into_iter().enumerate() {
        paint_count_cell(
            painter,
            frame,
            scale,
            (
                list_origin.0 + first_column + 28.0 * index as f32,
                list_origin.1 + row_height * line + 2.0,
            ),
            count,
            font,
            colours,
        );
    }
}

/// Count-cell colours shared by the Personnel and Troop side tabs.
pub(crate) fn count_cell_colours(side_index: usize) -> (egui::Color32, egui::Color32) {
    if side_index == 0 {
        (egui::Color32::from_rgb(255, 0, 0), egui::Color32::WHITE)
    } else {
        (egui::Color32::from_rgb(0, 255, 0), egui::Color32::BLACK)
    }
}

fn set_name(ui: &mut FinderUiState, name: &str) {
    ui.name.clear();
    ui.name.push_str(name);
}

pub(crate) const fn side(faction: CockpitFaction) -> usize {
    match faction {
        CockpitFaction::Alliance => 0,
        CockpitFaction::Empire => 1,
    }
}

pub(crate) fn side_value<T: Copy>(faction: CockpitFaction, values: [T; 2]) -> T {
    values[side(faction)]
}

pub(crate) const fn lists_side(side_index: usize, is_alliance: bool) -> bool {
    side_index == if is_alliance { 0 } else { 1 }
}

pub(crate) const fn player_side(faction: CockpitFaction) -> Faction {
    match faction {
        CockpitFaction::Alliance => Faction::Alliance,
        CockpitFaction::Empire => Faction::Empire,
    }
}

/// `FUN_00606980` places each Finder in the galaxy view; the port centers it.
pub(crate) fn window_rect(layout: CockpitLayout) -> egui::Rect {
    galaxy_centered_rect(layout, WIDTH, HEIGHT)
}

pub(crate) fn at(frame: egui::Rect, scale: f32, (x, y, w, h): LogicalRect) -> egui::Rect {
    logical_rect(frame, scale, x, y, w, h)
}

pub(crate) fn should_remove(id: Option<WindowId>, removed: &[WindowId]) -> bool {
    id.is_some_and(|id| removed.contains(&id))
}
