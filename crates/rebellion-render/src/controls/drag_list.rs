//! The original `CoolDragList`.
//!
//! `FUN_00607ea0` constructs the control and `FUN_006083c0` is its `WndProc`:
//! press selection with modifier rules, mouse capture, thresholded drags and
//! drops (`0x29a`), selection notifications (`0x29b`), activation (`0x309`),
//! bounded keyboard movement and reveal (`FUN_00609500`, `FUN_00609ed0`).
//! `FUN_006094b0` clears the selection and `FUN_00609410` collects it. The
//! layout is `FUN_00609ae0`'s: stacked rows, or a grid that wraps before the
//! next cell would pass the list's width less a shown scroll bar. The bar is
//! the canonical [`ScrollBar`] (`FUN_0060aa70`), composed rather than copied.
//!
//! Row data, row art and text, and every game action stay with the feature
//! that owns the list. This control reports typed events instead of posting
//! owner notifications, and it never mutates feature state itself: the owner
//! applies [`DragListResponse::next`] only after the frozen
//! `FrameInputRouter` admits the output's pointer phases.

#![allow(
    clippy::module_name_repetitions,
    reason = "The source class name is the public renderer object name."
)]

use std::hash::Hash;

use egui_macroquad::egui;

use super::{
    paint_scroll_bar, ScrollBar, ScrollBarArt, ScrollBlit, ScrollDrag, ScrollPointer, ScrollRange,
};
use crate::bmp_cache::BmpCache;
use crate::native_geometry::logical_rect;
use crate::windowing_input::{PointerButtonId, PointerPhase};

/// The squared release distance a drag must exceed before it drops
/// (`FUN_006083c0` case `0x202`: `dx * dx + dy * dy > 0x18`).
pub(crate) const DROP_DISTANCE_SQUARED: f32 = 24.0;

/// How `FUN_00609ae0` places items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DragListLayout {
    /// One item per row, stacked downwards (`+0xf0` 1 without flag
    /// `0x80000`).
    Stacked,
    /// Row-major cells that wrap before the next one would pass the list's
    /// width, less a shown scroll bar (flag `0x80000`).
    Grid,
}

/// Flag `0x10000`: whether Ctrl and Shift extend the selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SelectionPolicy {
    /// Modifiers are ignored and every selection replaces the last.
    Single,
    /// Ctrl toggles an item and Shift selects a range from the anchor.
    Multiple,
}

/// Flags `0x20000` and `0x100000`: whether a captured press may drag, and
/// where its release drops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DragPolicy {
    /// Flag `0x20000`: captured motion never starts a drag.
    Disabled,
    /// Flag `0x100000` clear: any release past the distance drops.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "Recovered list policy the remaining CoolDragList consumers select (or-ep-uniform-ui-controls-gru5.5)."
        )
    )]
    DropAnywhere,
    /// Flag `0x100000` set: only a release outside the list drops.
    DropOutside,
}

/// Wheel handling. `FUN_006083c0` has no `WM_MOUSEWHEEL` case, so only a
/// feature with its own recovered wheel rule opts in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WheelPolicy {
    Ignored,
    /// One row per wheel step, as the list popups and report lists use.
    Rows,
}

/// One keyboard command `FUN_006083c0` handles in case `0x100`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ListKey {
    Up,
    Down,
    PageUp,
    PageDown,
    Home,
    End,
    Enter,
}

/// A list's recovered configuration, in its window's logical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct DragListSpec {
    /// The list's x, y, width and height in its window.
    pub(crate) rect: (f32, f32, f32, f32),
    /// The item cell's width and height (`+0xcc`, `+0xd0`).
    pub(crate) item: (f32, f32),
    pub(crate) layout: DragListLayout,
    pub(crate) selection: SelectionPolicy,
    pub(crate) drag: DragPolicy,
    pub(crate) wheel: WheelPolicy,
    /// The RCDATA of the bar `FUN_0060a490` attaches, if the feature ports it.
    pub(crate) bar: Option<ScrollBarArt>,
}

impl DragListSpec {
    fn contains(&self, (x, y): (f32, f32)) -> bool {
        let (left, top, width, height) = self.rect;
        x >= left && x < left + width && y >= top && y < top + height
    }
}

/// One frame's laid-out list: columns, rows and the bar while it shows.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct DragListGeometry {
    spec: DragListSpec,
    columns: usize,
    item_count: usize,
    total_rows: usize,
    first_row: usize,
    bar: Option<ScrollBar>,
}

impl DragListGeometry {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss,
        reason = "Validated non-negative list geometry is truncated to whole cells."
    )]
    fn new(spec: DragListSpec, item_count: usize, first_row: usize) -> Self {
        let (x, y, width, height) = spec.rect;
        let columns_for = |width: f32| match spec.layout {
            DragListLayout::Stacked => 1,
            DragListLayout::Grid => ((width / spec.item.0).max(0.0) as usize).max(1),
        };
        let rows_for = |columns: usize| item_count.div_ceil(columns);
        let viewport = height / spec.item.1;
        let mut columns = columns_for(width);
        let mut total_rows = rows_for(columns);
        let mut bar = None;
        if let Some(art) = spec.bar {
            let range = ScrollRange {
                first: 0,
                visible: viewport,
                total: total_rows,
            };
            if range.overflows() {
                let shown = ScrollBar::new(art, x, y, width, height);
                columns = columns_for(width - shown.width());
                total_rows = rows_for(columns);
                bar = Some(shown);
            }
        }
        let max_first = total_rows.saturating_sub(viewport.floor().max(1.0) as usize);
        Self {
            spec,
            columns,
            item_count,
            total_rows,
            first_row: first_row.min(max_first),
            bar,
        }
    }

    /// The items laid out per row.
    #[cfg(test)]
    pub(crate) const fn columns(&self) -> usize {
        self.columns
    }

    /// Rows that fit whole, which paging and reveal use.
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "Validated non-negative list geometry is truncated to whole rows."
    )]
    pub(crate) fn whole_rows(&self) -> usize {
        ((self.spec.rect.3 / self.spec.item.1) as usize).max(1)
    }

    /// Rows painted, including a final one the list clips.
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "Validated non-negative list geometry is rounded up to painted rows."
    )]
    fn painted_rows(&self) -> usize {
        ((self.spec.rect.3 / self.spec.item.1).ceil() as usize).max(1)
    }

    fn range(&self) -> ScrollRange {
        ScrollRange {
            first: self.first_row,
            visible: self.spec.rect.3 / self.spec.item.1,
            total: self.total_rows,
        }
    }

    fn on_bar(&self, (x, y): (f32, f32)) -> bool {
        let (left, top, width, height) = self.spec.rect;
        self.bar.is_some_and(|bar| {
            x >= left + width - bar.width() && x < left + width && y >= top && y < top + height
        })
    }

    /// The item index whose cell holds `point`, if any (`FUN_00609d80`,
    /// `FUN_00609d10`).
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "The point is inside the list, so its cell offsets are non-negative."
    )]
    pub(crate) fn item_at(&self, point: (f32, f32)) -> Option<usize> {
        if !self.spec.contains(point) || self.on_bar(point) {
            return None;
        }
        let column = ((point.0 - self.spec.rect.0) / self.spec.item.0) as usize;
        let row = ((point.1 - self.spec.rect.1) / self.spec.item.1) as usize;
        if column >= self.columns {
            return None;
        }
        let index = (self.first_row + row) * self.columns + column;
        (index < self.item_count).then_some(index)
    }

    /// The painted cells: each item's index and logical cell rectangle.
    #[allow(
        clippy::cast_precision_loss,
        reason = "Visible cell counts are small window-layout indices."
    )]
    pub(crate) fn cells(&self) -> impl Iterator<Item = (usize, (f32, f32, f32, f32))> + '_ {
        let start = self.first_row * self.columns;
        let end = self
            .item_count
            .min(start + self.painted_rows() * self.columns);
        (start..end).map(move |index| {
            let shown = index - start;
            let column = shown % self.columns;
            let row = shown / self.columns;
            (
                index,
                (
                    self.spec.rect.0 + column as f32 * self.spec.item.0,
                    self.spec.rect.1 + row as f32 * self.spec.item.1,
                    self.spec.item.0,
                    self.spec.item.1,
                ),
            )
        })
    }
}

/// An active left-button capture (`FUN_00601080`): where it began, whether
/// it pressed an already selected item (flag `0x10`), and whether captured
/// motion began a drag (flag `1`).
#[derive(Debug, Clone, Copy, PartialEq)]
struct Capture {
    origin: (f32, f32),
    deferred: bool,
    dragging: bool,
}

/// Persistent control-local state. Item identity is the feature's key.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DragListState<K> {
    selected: Vec<K>,
    anchor: Option<K>,
    /// The item released by this list's preceding click. Egui's click count
    /// is pointer-global, so the control still verifies that both clicks
    /// belonged to the same list item before posting `0x309`.
    last_click: Option<K>,
    first_row: usize,
    scroll: ScrollDrag,
    capture: Option<Capture>,
}

impl<K> Default for DragListState<K> {
    fn default() -> Self {
        Self {
            selected: Vec::new(),
            anchor: None,
            last_click: None,
            first_row: 0,
            scroll: ScrollDrag::default(),
            capture: None,
        }
    }
}

impl<K: Copy + PartialEq> DragListState<K> {
    /// A list showing its first row with `item` selected, as a feature's
    /// create path does with `FUN_00609500(.., 0)`.
    #[cfg(test)]
    pub(crate) fn selecting(item: Option<K>) -> Self {
        Self {
            selected: item.into_iter().collect(),
            ..Self::default()
        }
    }

    pub(crate) fn is_selected(&self, item: K) -> bool {
        self.selected.contains(&item)
    }

    /// The selected items in row order (`FUN_00609410`).
    pub(crate) fn selection(&self, rows: &[K]) -> Vec<K> {
        rows.iter()
            .copied()
            .filter(|row| self.is_selected(*row))
            .collect()
    }

    /// The first selected item in row order.
    pub(crate) fn first_selected(&self, rows: &[K]) -> Option<K> {
        rows.iter().copied().find(|row| self.is_selected(*row))
    }

    pub(crate) const fn first_row(&self) -> usize {
        self.first_row
    }

    /// The selected items in selection order.
    pub(crate) fn selected(&self) -> &[K] {
        &self.selected
    }

    #[cfg(test)]
    pub(crate) const fn anchor(&self) -> Option<K> {
        self.anchor
    }

    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn set_first_row(&mut self, first_row: usize) {
        self.first_row = first_row;
    }

    pub(crate) const fn is_capturing(&self) -> bool {
        self.capture.is_some()
    }

    /// Select only `item` without notifying, as `FUN_00609500(.., 0)` does
    /// for a list a feature fills.
    pub(crate) fn select_only(&mut self, item: Option<K>) {
        self.selected = item.into_iter().collect();
    }

    /// Replace the selection without notifying, for an owner's Select All,
    /// refill or restored interface state.
    pub(crate) fn set_selection(&mut self, items: impl IntoIterator<Item = K>) {
        self.selected = items.into_iter().collect();
        self.anchor = None;
    }

    /// Select one owner-chosen item and reveal it with the control's own
    /// layout, as a parent calling `FUN_00609500` followed by
    /// `FUN_00609ed0` does after prefix search or a category refill.
    pub(crate) fn select_and_reveal(&mut self, spec: DragListSpec, rows: &[K], item: Option<K>) {
        self.select_only(item);
        self.anchor = item;
        let Some(index) = item.and_then(|item| rows.iter().position(|row| *row == item)) else {
            return;
        };
        let geometry = self.geometry(spec, rows.len());
        self.first_row = crate::list_control::reveal_row(
            geometry.first_row,
            index / geometry.columns,
            geometry.whole_rows(),
        );
    }

    /// Capture loss (`0x215`, `0x40d`): end the press and any drag.
    pub(crate) fn cancel_capture(&mut self) -> bool {
        let cancelled = self.capture.take().is_some() || self.scroll != ScrollDrag::default();
        self.scroll = ScrollDrag::default();
        cancelled
    }

    /// The frame's layout for `item_count` items.
    pub(crate) fn geometry(&self, spec: DragListSpec, item_count: usize) -> DragListGeometry {
        DragListGeometry::new(spec, item_count, self.first_row)
    }

    /// Logical cells for a native-painter owner such as the tactical modal.
    pub(crate) fn logical_cells(
        &self,
        spec: DragListSpec,
        item_count: usize,
    ) -> Vec<(usize, (f32, f32, f32, f32))> {
        self.geometry(spec, item_count).cells().collect()
    }

    /// Canonical bar pieces for a native-painter owner after this frame's
    /// input has updated the control state.
    pub(crate) fn bar_blits(
        &self,
        spec: DragListSpec,
        item_count: usize,
        pointer: ScrollPointer,
    ) -> Vec<ScrollBlit> {
        let geometry = self.geometry(spec, item_count);
        geometry.bar.map_or_else(Vec::new, |bar| {
            bar.blits(geometry.range(), self.scroll, pointer)
        })
    }

    fn reconcile(&mut self, rows: &[K]) {
        self.selected.retain(|item| rows.contains(item));
        if self.anchor.is_some_and(|anchor| !rows.contains(&anchor)) {
            self.anchor = None;
        }
        if self
            .last_click
            .is_some_and(|last_click| !rows.contains(&last_click))
        {
            self.last_click = None;
        }
    }

    fn toggle(&mut self, item: K) {
        if let Some(index) = self.selected.iter().position(|selected| *selected == item) {
            self.selected.remove(index);
        } else {
            self.selected.push(item);
        }
    }

    /// The press and deferred-release selection shared by `0x201`, `0x204`
    /// and `0x202`. Returns whether the source posts `0x29b`.
    fn select_by_pointer(
        &mut self,
        rows: &[K],
        item: Option<K>,
        modifiers: (bool, bool),
        anchor: Option<K>,
    ) -> bool {
        match modifiers {
            (true, _) => item.is_some_and(|item| {
                self.toggle(item);
                true
            }),
            (false, true) => {
                // hyp: FUN_00609590 replaces the selection with the inclusive
                // row-order range from the anchor; its body is untraced.
                let position =
                    |key: Option<K>| key.and_then(|key| rows.iter().position(|row| *row == key));
                if let (Some(from), Some(to)) = (position(anchor.or(item)), position(item)) {
                    let (low, high) = (from.min(to), from.max(to));
                    self.selected = rows[low..=high].to_vec();
                }
                true
            }
            (false, false) => {
                self.selected = item.into_iter().collect();
                true
            }
        }
    }
}

/// Current-frame input facts in the list's window logical coordinates.
/// The egui adapter admits presses only when egui gives them to the list.
#[allow(
    clippy::struct_excessive_bools,
    reason = "Each flag is an independent fact of one routed input frame."
)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct DragListInput {
    pub(crate) pointer: Option<(f32, f32)>,
    pub(crate) primary_pressed: bool,
    /// A primary press this adapter observed but egui routed elsewhere.
    pub(crate) primary_pressed_elsewhere: bool,
    pub(crate) primary_down: bool,
    pub(crate) primary_released: bool,
    pub(crate) secondary_pressed: bool,
    pub(crate) secondary_released: bool,
    pub(crate) double_clicked: bool,
    /// Two complete primary clicks arrived in one egui input batch.
    pub(crate) batched_double_clicked: bool,
    pub(crate) ctrl: bool,
    pub(crate) shift: bool,
    pub(crate) wheel: f32,
    pub(crate) key: Option<ListKey>,
}

/// One typed outcome of a list frame, in the order the source posts them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum DragListEvent<K> {
    /// `0x40e` to the owner on every admitted press, with the item hit.
    Pressed {
        button: PointerButtonId,
        item: Option<K>,
    },
    /// `0x29b`: the selection may have changed.
    SelectionChanged,
    /// The first row changed through the bar, wheel or a reveal.
    Scrolled,
    /// A captured left press on a selected item that this list may drag.
    DragArmed { item: K, origin: (f32, f32) },
    /// Captured motion began the drag (flag `1`).
    DragStarted,
    /// `0x29a`: the drag dropped at `point` with the selection.
    Dropped { point: (f32, f32) },
    /// `0x309`: a double click or Enter activated the selection.
    Activated,
    /// The right release the list leaves to its owner, with the item hit.
    ContextReleased { item: Option<K>, point: (f32, f32) },
    /// Capture ended without a release.
    Cancelled,
}

/// The ordered outcomes of one frame and the pointer phases that caused them.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DragListOutput<K> {
    pub(crate) events: Vec<DragListEvent<K>>,
    pub(crate) phases: Vec<PointerPhase>,
}

impl<K> Default for DragListOutput<K> {
    fn default() -> Self {
        Self {
            events: Vec::new(),
            phases: Vec::new(),
        }
    }
}

impl<K: Copy + PartialEq> DragListOutput<K> {
    fn push(&mut self, event: DragListEvent<K>, phase: Option<PointerPhase>) {
        self.events.push(event);
        if let Some(phase) = phase.filter(|phase| !self.phases.contains(phase)) {
            self.phases.push(phase);
        }
    }

    /// Whether a pointer gesture caused any outcome, so the owner must have
    /// the router admit [`Self::phases`] before applying the next state.
    pub(crate) fn pointer_mutation(&self) -> bool {
        !self.phases.is_empty()
    }

    pub(crate) fn contains(&self, event: DragListEvent<K>) -> bool {
        self.events.contains(&event)
    }

    pub(crate) fn pressed(&self) -> Option<(PointerButtonId, Option<K>)> {
        self.events.iter().find_map(|event| match *event {
            DragListEvent::Pressed { button, item } => Some((button, item)),
            _ => None,
        })
    }

    pub(crate) fn drag_armed(&self) -> Option<(K, (f32, f32))> {
        self.events.iter().find_map(|event| match *event {
            DragListEvent::DragArmed { item, origin } => Some((item, origin)),
            _ => None,
        })
    }

    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "Recovered list policy the remaining CoolDragList consumers select (or-ep-uniform-ui-controls-gru5.5)."
        )
    )]
    pub(crate) fn dropped(&self) -> Option<(f32, f32)> {
        self.events.iter().find_map(|event| match *event {
            DragListEvent::Dropped { point } => Some(point),
            _ => None,
        })
    }

    pub(crate) fn context_released(&self) -> Option<(Option<K>, (f32, f32))> {
        self.events.iter().find_map(|event| match *event {
            DragListEvent::ContextReleased { item, point } => Some((item, point)),
            _ => None,
        })
    }
}

impl<K: Copy + PartialEq> DragListState<K> {
    /// Apply one input frame to `rows` and return the ordered outcomes.
    #[allow(
        clippy::too_many_lines,
        reason = "One ordered pass mirrors FUN_006083c0's message cases."
    )]
    pub(crate) fn update(
        &mut self,
        spec: DragListSpec,
        rows: &[K],
        input: DragListInput,
    ) -> DragListOutput<K> {
        let mut output = DragListOutput::default();
        self.reconcile(rows);
        if input.primary_pressed_elsewhere {
            self.last_click = None;
        }
        let mut geometry = self.geometry(spec, rows.len());
        self.first_row = geometry.first_row;

        // The bar is a child window: input over it never reaches the list.
        let on_bar = input.pointer.is_some_and(|at| geometry.on_bar(at));
        if let Some(bar) = geometry.bar {
            let before = (self.first_row, self.scroll);
            let first = bar.update(
                &mut self.scroll,
                geometry.range(),
                geometry.whole_rows(),
                ScrollPointer {
                    at: input.pointer,
                    pressed: input.primary_pressed,
                    down: input.primary_down,
                    released: input.primary_released,
                },
            );
            self.first_row = first;
            if (self.first_row, self.scroll) != before {
                let phase = if input.primary_pressed {
                    PointerPhase::Press(PointerButtonId::Primary)
                } else if input.primary_released {
                    PointerPhase::Release(PointerButtonId::Primary)
                } else {
                    PointerPhase::Motion(PointerButtonId::Primary)
                };
                output.push(DragListEvent::Scrolled, Some(phase));
            }
        } else {
            self.scroll = ScrollDrag::default();
        }

        let inside = input.pointer.filter(|&at| spec.contains(at));
        if spec.wheel == WheelPolicy::Rows && input.wheel != 0.0 && inside.is_some() {
            let positions = geometry.range().max_first() + 1;
            let first = crate::list_control::wheel_step(self.first_row, positions, input.wheel);
            if first != self.first_row {
                self.first_row = first;
                output.push(DragListEvent::Scrolled, Some(PointerPhase::Auxiliary));
            }
        }
        geometry.first_row = self.first_row;
        let item_at = |at: (f32, f32)| geometry.item_at(at).map(|index| rows[index]);
        let modifiers = match spec.selection {
            SelectionPolicy::Single => (false, false),
            SelectionPolicy::Multiple => (input.ctrl, input.shift),
        };

        // Lost capture: the button came up without a release this list saw.
        if self.capture.is_some()
            && !input.primary_down
            && !input.primary_released
            && !input.primary_pressed
        {
            self.capture = None;
            output.push(DragListEvent::Cancelled, None);
        }

        // Cases 0x201 and 0x204. A right press during a drag is ignored.
        let press = if input.primary_pressed {
            Some(PointerButtonId::Primary)
        } else if input.secondary_pressed && !self.capture.is_some_and(|capture| capture.dragging) {
            Some(PointerButtonId::Secondary)
        } else {
            None
        };
        if let (Some(button), Some(at), false) = (press, inside, on_bar) {
            let item = item_at(at);
            let phase = Some(PointerPhase::Press(button));
            output.push(DragListEvent::Pressed { button, item }, phase);
            let deferred = item.is_some_and(|item| self.is_selected(item));
            let preserve_anchor =
                spec.selection == SelectionPolicy::Multiple && input.shift && self.anchor.is_some();
            if !deferred {
                let anchor = self.anchor;
                if self.select_by_pointer(rows, item, modifiers, anchor) {
                    output.push(DragListEvent::SelectionChanged, phase);
                }
            }
            if !preserve_anchor {
                self.anchor = item;
            }
            if button == PointerButtonId::Primary {
                self.capture = Some(Capture {
                    origin: at,
                    deferred,
                    dragging: false,
                });
                if let (Some(item), false) = (item, spec.drag == DragPolicy::Disabled) {
                    if self.is_selected(item) {
                        output.push(DragListEvent::DragArmed { item, origin: at }, phase);
                    }
                }
            }
        }

        // Case 0x200: captured motion with the left button down drags.
        if let (Some(capture), Some(at)) = (self.capture.as_mut(), input.pointer) {
            if input.primary_down
                && !input.primary_pressed
                && !capture.dragging
                && spec.drag != DragPolicy::Disabled
                && at != capture.origin
            {
                capture.dragging = true;
                output.push(
                    DragListEvent::DragStarted,
                    Some(PointerPhase::Motion(PointerButtonId::Primary)),
                );
            }
        }

        // Case 0x202: drop past the distance, else apply a deferred press.
        if input.primary_released {
            if let Some(capture) = self.capture.take() {
                let phase = Some(PointerPhase::Release(PointerButtonId::Primary));
                let dropped = input.pointer.filter(|&(x, y)| {
                    let (dx, dy) = (x - capture.origin.0, y - capture.origin.1);
                    capture.dragging
                        && dx * dx + dy * dy > DROP_DISTANCE_SQUARED
                        && match spec.drag {
                            DragPolicy::Disabled => false,
                            DragPolicy::DropAnywhere => true,
                            DragPolicy::DropOutside => !spec.contains((x, y)),
                        }
                });
                if let Some(point) = dropped {
                    output.push(DragListEvent::Dropped { point }, phase);
                } else {
                    let item = input.pointer.and_then(item_at);
                    if capture.deferred {
                        let anchor = self.anchor;
                        if self.select_by_pointer(rows, item, modifiers, anchor) {
                            output.push(DragListEvent::SelectionChanged, phase);
                        }
                    }
                    if input.double_clicked
                        && item.is_some()
                        && (input.batched_double_clicked || self.last_click == item)
                    {
                        output.push(DragListEvent::Activated, phase);
                    }
                    self.last_click = item;
                }
            }
        }

        // The right release is not a list case; the owner receives it.
        if let (true, Some(at), false) = (input.secondary_released, inside, on_bar) {
            output.push(
                DragListEvent::ContextReleased {
                    item: item_at(at),
                    point: at,
                },
                Some(PointerPhase::Release(PointerButtonId::Secondary)),
            );
        }

        if let Some(key) = input.key {
            self.key(spec, rows, &geometry, key, &mut output);
        }
        output
    }

    /// Case `0x100`: move from the first selected item, else the first row.
    fn key(
        &mut self,
        spec: DragListSpec,
        rows: &[K],
        geometry: &DragListGeometry,
        key: ListKey,
        output: &mut DragListOutput<K>,
    ) {
        let Some(last) = rows.len().checked_sub(1) else {
            return;
        };
        let current = rows.iter().position(|row| self.is_selected(*row));
        if key == ListKey::Enter {
            let current = current.unwrap_or(0);
            if !self.is_selected(rows[current]) {
                self.selected.push(rows[current]);
            }
            output.push(DragListEvent::Activated, None);
            return;
        }
        let page = geometry.whole_rows();
        let target = match key {
            ListKey::Up => current.map_or(Some(0), |current| current.checked_sub(1)),
            ListKey::Down => {
                current.map_or(Some(0), |current| (current < last).then_some(current + 1))
            }
            ListKey::PageUp => Some(current.map_or(0, |current| current.saturating_sub(page))),
            ListKey::PageDown => {
                Some(current.map_or(page.min(last), |current| (current + page).min(last)))
            }
            ListKey::Home => Some(0),
            ListKey::End => Some(last),
            ListKey::Enter => None,
        };
        let Some(target) = target else {
            return;
        };
        // FUN_006083c0 clears the current item; FUN_00609500 clears every
        // other one only for a single-selection list, then selects and posts.
        if let Some(current) = current {
            self.selected.retain(|item| *item != rows[current]);
        }
        if spec.selection == SelectionPolicy::Single {
            self.selected.clear();
        }
        self.selected.push(rows[target]);
        output.push(DragListEvent::SelectionChanged, None);
        // FUN_00609ed0: scroll only far enough to show the item.
        let first = crate::list_control::reveal_row(
            self.first_row,
            target / geometry.columns,
            geometry.whole_rows(),
        );
        if first != self.first_row {
            self.first_row = first;
            output.push(DragListEvent::Scrolled, None);
        }
    }
}

/// The egui-composed list: the next state for the owner to apply, the
/// outcomes, and the frame's geometry for painting rows and the bar.
#[derive(Debug)]
pub(crate) struct DragListResponse<K> {
    pub(crate) next: DragListState<K>,
    pub(crate) output: DragListOutput<K>,
    pub(crate) geometry: DragListGeometry,
    pointer: ScrollPointer,
}

impl<K: Copy + PartialEq> DragListResponse<K> {
    /// The cells to paint in screen space: item index and cell rectangle.
    pub(crate) fn screen_cells(
        &self,
        window: egui::Rect,
        scale: f32,
    ) -> impl Iterator<Item = (usize, egui::Rect)> + '_ {
        self.geometry
            .cells()
            .map(move |(index, (x, y, width, height))| {
                (index, logical_rect(window, scale, x, y, width, height))
            })
    }

    /// Paint the composed bar after the rows, as the child window draws over
    /// its owner.
    pub(crate) fn paint_bar(
        &self,
        painter: &egui::Painter,
        ctx: &egui::Context,
        cache: &mut BmpCache,
        window: egui::Rect,
        scale: f32,
    ) {
        if let Some(bar) = self.geometry.bar {
            paint_scroll_bar(
                painter,
                ctx,
                cache,
                window,
                scale,
                &bar.blits(self.geometry.range(), self.next.scroll, self.pointer),
            );
        }
    }
}

/// The egui adapter for a [`DragListState`].
pub(crate) struct DragList;

impl DragList {
    /// Admit this frame's egui input and run the list. Egui settles which
    /// overlapping surface owns a press before the source-shaped state machine
    /// sees it; releases always reach a list holding capture.
    #[allow(
        clippy::too_many_arguments,
        reason = "The control boundary keeps identity, geometry, rows, state and keys explicit."
    )]
    pub(crate) fn show<K: Copy + PartialEq>(
        ui: &mut egui::Ui,
        id: impl Hash,
        spec: DragListSpec,
        rows: &[K],
        state: &DragListState<K>,
        window: egui::Rect,
        scale: f32,
        key: Option<ListKey>,
    ) -> DragListResponse<K> {
        let (x, y, width, height) = spec.rect;
        let list = logical_rect(window, scale, x, y, width, height);
        let response = ui.interact(list, ui.id().with(id), egui::Sense::click());
        // Read every response fact first: several take the context lock that
        // `input` holds.
        // A press and its release can arrive in one frame; egui then reports
        // the click rather than a held button.
        let secondary_released = response.secondary_clicked();
        let owned_press = response.is_pointer_button_down_on()
            || response.clicked_by(egui::PointerButton::Primary)
            || secondary_released;
        // Egui's click count is pointer-global. A first list click after a
        // different control can be "double", and the second list click then
        // "triple"; the state machine below gates either signal by this
        // list's own prior item.
        let double_clicked = response.double_clicked() || response.triple_clicked();
        let hovered = response.hovered();
        let input = ui.ctx().input(|input| {
            let logical = |point: egui::Pos2| {
                (
                    (point.x - window.min.x) / scale,
                    (point.y - window.min.y) / scale,
                )
            };
            let pointer = input.pointer.interact_pos().map(logical);
            let batched_double_clicked = double_clicked
                && input
                    .events
                    .iter()
                    .filter(|event| {
                        matches!(
                            event,
                            egui::Event::PointerButton {
                                button: egui::PointerButton::Primary,
                                pressed: false,
                                ..
                            }
                        )
                    })
                    .count()
                    >= 2;
            DragListInput {
                pointer,
                primary_pressed: owned_press
                    && input.pointer.button_pressed(egui::PointerButton::Primary),
                primary_pressed_elsewhere: !owned_press
                    && input.pointer.button_pressed(egui::PointerButton::Primary),
                primary_down: input.pointer.button_down(egui::PointerButton::Primary),
                primary_released: input.pointer.button_released(egui::PointerButton::Primary),
                secondary_pressed: owned_press
                    && input.pointer.button_pressed(egui::PointerButton::Secondary),
                secondary_released,
                double_clicked,
                batched_double_clicked,
                ctrl: input.modifiers.command,
                shift: input.modifiers.shift,
                wheel: if hovered {
                    input.raw_scroll_delta.y
                } else {
                    0.0
                },
                key,
            }
        });
        let mut next = state.clone();
        let output = next.update(spec, rows, input);
        let geometry = next.geometry(spec, rows.len());
        DragListResponse {
            next,
            output,
            geometry,
            pointer: ScrollPointer {
                at: input.pointer,
                pressed: input.primary_pressed,
                down: input.primary_down,
                released: input.primary_released,
            },
        }
    }
}

impl DragList {
    /// Run a list whose owner keeps the selection itself, as a window
    /// mirrors `0x29b` into its own field (`+0x1c4` in the Missions
    /// window). The list's press, capture and first row stay in egui's typed
    /// temporary store under the stable id, as a button's armed state does.
    #[allow(
        clippy::too_many_arguments,
        reason = "The control boundary keeps identity, geometry, rows and selection explicit."
    )]
    pub(crate) fn show_mirrored<K: Copy + PartialEq + Send + Sync + 'static>(
        ui: &mut egui::Ui,
        id: impl Hash,
        spec: DragListSpec,
        rows: &[K],
        selected: Option<K>,
        window: egui::Rect,
        scale: f32,
    ) -> DragListResponse<K> {
        let id = ui.id().with(id);
        let state_id = id.with("drag-list-state");
        let mut state = ui
            .data(|data| data.get_temp::<DragListState<K>>(state_id))
            .unwrap_or_default();
        if state.first_selected(rows) == selected {
            state.select_only(selected);
        } else {
            state.select_and_reveal(spec, rows, selected);
        }
        let response = Self::show(ui, id, spec, rows, &state, window, scale, None);
        ui.data_mut(|data| data.insert_temp(state_id, response.next.clone()));
        response
    }
}

/// An item's word-wrapped label, as the list's paint case draws it with
/// `DrawTextA` from the cell moved by (`+0xe8`, `+0xec`).
#[allow(
    clippy::too_many_arguments,
    reason = "The source label carries rect, scale, text, font, colour, wrap width and offset."
)]
pub(crate) fn paint_label(
    painter: &egui::Painter,
    cell: egui::Rect,
    scale: f32,
    text: &str,
    font: &egui::FontId,
    color: egui::Color32,
    logical_width: f32,
    text_offset: (f32, f32),
) {
    let galley = painter.layout(text.to_owned(), font.clone(), color, logical_width * scale);
    painter.galley(
        logical_rect(cell, scale, text_offset.0, text_offset.1, 0.0, 0.0).min,
        galley,
        color,
    );
}

#[allow(
    clippy::float_cmp,
    reason = "Logical list coordinates are exact small integers."
)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::controls::LIST_BAR_29CC;

    // FUN_004a8790: the System Defenses list at (7, 81), 222 by 210, with
    // 70 by 70 grid cells (flag 0x80000) and RCDATA 0x29cc's bar.
    const GRID: DragListSpec = DragListSpec {
        rect: (7.0, 81.0, 222.0, 210.0),
        item: (70.0, 70.0),
        layout: DragListLayout::Grid,
        selection: SelectionPolicy::Multiple,
        drag: DragPolicy::DropOutside,
        wheel: WheelPolicy::Ignored,
        bar: Some(LIST_BAR_29CC),
    };

    // FUN_0049f540: the Missions list at (5, 24), 94 by 275, 90 by 50 rows.
    const STACKED: DragListSpec = DragListSpec {
        rect: (5.0, 24.0, 94.0, 275.0),
        item: (90.0, 50.0),
        layout: DragListLayout::Stacked,
        selection: SelectionPolicy::Single,
        drag: DragPolicy::Disabled,
        wheel: WheelPolicy::Ignored,
        bar: None,
    };

    fn rows(count: u32) -> Vec<u32> {
        (0..count).collect()
    }

    /// The centre of grid cell `column`, `row` from the list's top.
    fn grid_cell(column: u8, row: u8) -> (f32, f32) {
        (
            7.0 + 70.0 * f32::from(column) + 35.0,
            81.0 + 70.0 * f32::from(row) + 35.0,
        )
    }

    fn press(at: (f32, f32)) -> DragListInput {
        DragListInput {
            pointer: Some(at),
            primary_pressed: true,
            primary_down: true,
            ..DragListInput::default()
        }
    }

    fn hold(at: (f32, f32)) -> DragListInput {
        DragListInput {
            pointer: Some(at),
            primary_down: true,
            ..DragListInput::default()
        }
    }

    fn release(at: (f32, f32)) -> DragListInput {
        DragListInput {
            pointer: Some(at),
            primary_released: true,
            ..DragListInput::default()
        }
    }

    fn key(key: ListKey) -> DragListInput {
        DragListInput {
            key: Some(key),
            ..DragListInput::default()
        }
    }

    fn click(state: &mut DragListState<u32>, spec: DragListSpec, rows: &[u32], at: (f32, f32)) {
        let _ = state.update(spec, rows, press(at));
        let _ = state.update(spec, rows, release(at));
    }

    #[test]
    fn a_grid_wraps_three_cells_and_paints_only_its_visible_rows() {
        let rows = rows(11);
        let state = DragListState::<u32>::default();
        let geometry = state.geometry(GRID, rows.len());

        assert_eq!(geometry.columns(), 3);
        assert_eq!(geometry.whole_rows(), 3);
        let cells: Vec<_> = geometry.cells().collect();
        assert_eq!(cells.len(), 9);
        assert_eq!(cells[0], (0, (7.0, 81.0, 70.0, 70.0)));
        assert_eq!(cells[4], (4, (77.0, 151.0, 70.0, 70.0)));
    }

    #[test]
    fn a_stacked_list_paints_the_row_its_foot_clips_but_pages_by_whole_rows() {
        let geometry = DragListState::<u32>::default().geometry(STACKED, 8);

        assert_eq!(geometry.columns(), 1);
        assert_eq!(geometry.whole_rows(), 5);
        assert_eq!(geometry.cells().count(), 6);
        assert_eq!(geometry.cells().last(), Some((5, (5.0, 274.0, 90.0, 50.0))));
    }

    #[test]
    fn a_stored_first_row_is_bounded_when_the_rows_shrink() {
        let mut state = DragListState::<u32> {
            first_row: 9,
            ..DragListState::default()
        };

        let _ = state.update(GRID, &rows(11), DragListInput::default());

        assert_eq!(state.first_row(), 1);
    }

    #[test]
    fn a_press_selects_its_item_and_reports_the_press_then_the_selection() {
        let rows = rows(5);
        let mut state = DragListState::default();

        let output = state.update(STACKED, &rows, press((10.0, 80.0)));

        assert_eq!(
            output.events,
            vec![
                DragListEvent::Pressed {
                    button: PointerButtonId::Primary,
                    item: Some(1),
                },
                DragListEvent::SelectionChanged,
            ]
        );
        assert_eq!(
            output.phases,
            vec![PointerPhase::Press(PointerButtonId::Primary)]
        );
        assert!(state.is_selected(1));
    }

    #[test]
    fn a_press_on_empty_list_space_clears_the_selection() {
        // FUN_006094b0 on a press that hits no item.
        let rows = rows(2);
        let mut state = DragListState::selecting(Some(1));

        let output = state.update(STACKED, &rows, press((10.0, 200.0)));

        assert!(output.contains(DragListEvent::SelectionChanged));
        assert_eq!(state.selection(&rows), Vec::<u32>::new());
    }

    #[test]
    fn a_single_selection_list_ignores_ctrl_and_shift() {
        let rows = rows(5);
        let mut state = DragListState::selecting(Some(0));
        let ctrl = DragListInput {
            ctrl: true,
            shift: true,
            ..press((10.0, 130.0))
        };

        let _ = state.update(STACKED, &rows, ctrl);

        assert_eq!(state.selection(&rows), vec![2]);
    }

    #[test]
    fn ctrl_toggles_and_shift_selects_the_range_in_a_multiple_selection_list() {
        let rows = rows(9);
        let mut state = DragListState::default();
        click(&mut state, GRID, &rows, grid_cell(1, 0));
        let ctrl = |at| DragListInput {
            ctrl: true,
            ..press(at)
        };
        let _ = state.update(GRID, &rows, ctrl(grid_cell(0, 1)));
        let _ = state.update(GRID, &rows, release(grid_cell(0, 1)));
        assert_eq!(state.selection(&rows), vec![1, 3]);

        let _ = state.update(GRID, &rows, ctrl(grid_cell(1, 0)));
        assert_eq!(state.selection(&rows), vec![1, 3]);
        // A press on a selected item defers to its release (flag 0x10),
        // which reads the modifiers then held.
        let ctrl_release = DragListInput {
            ctrl: true,
            ..release(grid_cell(1, 0))
        };
        let _ = state.update(GRID, &rows, ctrl_release);
        assert_eq!(state.selection(&rows), vec![3]);

        let shift = DragListInput {
            shift: true,
            ..press(grid_cell(2, 1))
        };
        let _ = state.update(GRID, &rows, shift);
        assert_eq!(state.selection(&rows), vec![1, 2, 3, 4, 5]);
        assert_eq!(state.anchor(), Some(1));
    }

    #[test]
    fn a_press_on_a_selected_item_keeps_the_selection_until_its_release() {
        let rows = rows(9);
        let mut state = DragListState::default();
        let ctrl = |at| DragListInput {
            ctrl: true,
            ..press(at)
        };
        click(&mut state, GRID, &rows, grid_cell(0, 0));
        let _ = state.update(GRID, &rows, ctrl(grid_cell(1, 0)));
        let _ = state.update(GRID, &rows, release(grid_cell(1, 0)));

        let pressed = state.update(GRID, &rows, press(grid_cell(0, 0)));
        assert!(!pressed.contains(DragListEvent::SelectionChanged));
        assert_eq!(state.selection(&rows), vec![0, 1]);

        let released = state.update(GRID, &rows, release(grid_cell(0, 0)));
        assert!(released.contains(DragListEvent::SelectionChanged));
        assert_eq!(state.selection(&rows), vec![0]);
    }

    #[test]
    fn a_right_press_selects_like_a_left_press_without_capturing() {
        let rows = rows(5);
        let mut state = DragListState::default();
        let right = DragListInput {
            pointer: Some((10.0, 80.0)),
            secondary_pressed: true,
            ..DragListInput::default()
        };

        let output = state.update(STACKED, &rows, right);

        assert_eq!(
            output.pressed(),
            Some((PointerButtonId::Secondary, Some(1)))
        );
        assert!(state.is_selected(1));
        assert!(!state.is_capturing());
    }

    #[test]
    fn a_right_release_reports_the_item_under_it_to_the_owner() {
        let rows = rows(5);
        let mut state = DragListState::default();
        let release = |at| DragListInput {
            pointer: Some(at),
            secondary_released: true,
            ..DragListInput::default()
        };

        let on_row = state.update(STACKED, &rows, release((10.0, 80.0)));
        let on_space = state.update(STACKED, &rows, release((10.0, 290.0)));

        assert_eq!(on_row.context_released(), Some((Some(1), (10.0, 80.0))));
        assert_eq!(on_space.context_released(), Some((None, (10.0, 290.0))));
        assert_eq!(
            on_row.phases,
            vec![PointerPhase::Release(PointerButtonId::Secondary)]
        );
    }

    #[test]
    fn a_right_press_during_a_drag_is_ignored() {
        let rows = rows(9);
        let mut state = DragListState::default();
        let _ = state.update(GRID, &rows, press(grid_cell(0, 0)));
        let _ = state.update(GRID, &rows, hold(grid_cell(1, 0)));
        let right = DragListInput {
            secondary_pressed: true,
            ..hold(grid_cell(2, 0))
        };

        let output = state.update(GRID, &rows, right);

        assert_eq!(output.pressed(), None);
        assert_eq!(state.selection(&rows), vec![0]);
    }

    #[test]
    fn a_double_click_activates_the_list() {
        let rows = rows(5);
        let mut state = DragListState::default();
        let _ = state.update(STACKED, &rows, press((10.0, 30.0)));
        let first = state.update(STACKED, &rows, release((10.0, 30.0)));
        assert!(!first.contains(DragListEvent::Activated));
        let _ = state.update(STACKED, &rows, press((10.0, 30.0)));
        let double = DragListInput {
            double_clicked: true,
            ..release((10.0, 30.0))
        };

        let output = state.update(STACKED, &rows, double);

        assert!(output.contains(DragListEvent::Activated));
    }

    #[test]
    fn a_same_frame_double_click_batch_activates_the_list() {
        let rows = rows(5);
        let mut state = DragListState::default();

        let output = state.update(
            STACKED,
            &rows,
            DragListInput {
                pointer: Some((10.0, 30.0)),
                primary_pressed: true,
                primary_released: true,
                double_clicked: true,
                batched_double_clicked: true,
                ..DragListInput::default()
            },
        );

        assert!(output.contains(DragListEvent::Activated));
        assert_eq!(state.selection(&rows), vec![0]);
    }

    #[test]
    fn a_multi_click_signal_after_a_different_control_does_not_activate_the_list() {
        let rows = rows(5);
        let mut state = DragListState::default();
        let _ = state.update(STACKED, &rows, press((10.0, 30.0)));
        let _ = state.update(STACKED, &rows, release((10.0, 30.0)));
        let _ = state.update(
            STACKED,
            &rows,
            DragListInput {
                primary_pressed_elsewhere: true,
                ..DragListInput::default()
            },
        );
        let _ = state.update(STACKED, &rows, press((10.0, 30.0)));

        let output = state.update(
            STACKED,
            &rows,
            DragListInput {
                double_clicked: true,
                ..release((10.0, 30.0))
            },
        );

        assert!(!output.contains(DragListEvent::Activated));
        assert_eq!(state.selection(&rows), vec![0]);
    }

    #[test]
    fn a_double_click_on_blank_list_space_does_not_activate_a_selection() {
        let rows = rows(1);
        let blank = (10.0, 100.0);
        let mut state = DragListState::default();
        let _ = state.update(STACKED, &rows, press(blank));
        let _ = state.update(STACKED, &rows, release(blank));
        let _ = state.update(STACKED, &rows, press(blank));

        let output = state.update(
            STACKED,
            &rows,
            DragListInput {
                double_clicked: true,
                ..release(blank)
            },
        );

        assert!(!output.contains(DragListEvent::Activated));
        assert_eq!(state.first_selected(&rows), None);
    }

    #[test]
    fn a_drag_drops_only_outside_the_list_and_past_the_distance() {
        let rows = rows(9);
        let mut state = DragListState::default();
        let origin = grid_cell(0, 0);
        let armed = state.update(GRID, &rows, press(origin));
        assert_eq!(armed.drag_armed(), Some((0, origin)));
        let started = state.update(GRID, &rows, hold((origin.0 + 1.0, origin.1)));
        assert!(started.contains(DragListEvent::DragStarted));

        // Inside the list the release is a selection, not a drop.
        let inside = state.update(GRID, &rows, release(grid_cell(1, 0)));
        assert_eq!(inside.dropped(), None);

        // Out of the list's top edge but within the distance: no drop.
        let edge = (origin.0, 83.0);
        let _ = state.update(GRID, &rows, press(edge));
        let _ = state.update(GRID, &rows, hold((edge.0, 79.0)));
        let near = state.update(GRID, &rows, release((edge.0, 80.0)));
        assert_eq!(near.dropped(), None, "within the distance it is a click");

        let _ = state.update(GRID, &rows, press(origin));
        let _ = state.update(GRID, &rows, hold((origin.0, 70.0)));
        let outside = state.update(GRID, &rows, release((origin.0, 40.0)));
        assert_eq!(outside.dropped(), Some((origin.0, 40.0)));
        assert!(!state.is_capturing());
    }

    #[test]
    fn a_drop_anywhere_list_drops_inside_once_past_the_distance() {
        let rows = rows(9);
        let mut state = DragListState::default();
        let spec = DragListSpec {
            drag: DragPolicy::DropAnywhere,
            ..GRID
        };
        click(&mut state, spec, &rows, grid_cell(0, 0));
        // The press on the selected item defers its selection to the release.
        let _ = state.update(spec, &rows, press(grid_cell(0, 0)));
        let _ = state.update(spec, &rows, hold(grid_cell(1, 0)));

        let output = state.update(spec, &rows, release(grid_cell(1, 0)));

        assert_eq!(output.dropped(), Some(grid_cell(1, 0)));
        assert_eq!(
            state.selection(&rows),
            vec![0],
            "a drop skips the deferred selection"
        );
    }

    #[test]
    fn a_list_that_may_not_drag_never_arms_or_drops() {
        let rows = rows(5);
        let mut state = DragListState::default();

        let armed = state.update(STACKED, &rows, press((10.0, 30.0)));
        let moved = state.update(STACKED, &rows, hold((200.0, 30.0)));
        let released = state.update(STACKED, &rows, release((200.0, 30.0)));

        assert_eq!(armed.drag_armed(), None);
        assert!(!moved.contains(DragListEvent::DragStarted));
        assert_eq!(released.dropped(), None);
    }

    #[test]
    fn capture_lost_without_a_release_cancels_the_drag_without_a_pointer_phase() {
        let rows = rows(9);
        let mut state = DragListState::default();
        let _ = state.update(GRID, &rows, press(grid_cell(0, 0)));
        let _ = state.update(GRID, &rows, hold(grid_cell(1, 0)));

        let output = state.update(
            GRID,
            &rows,
            DragListInput {
                pointer: Some((0.0, 0.0)),
                ..DragListInput::default()
            },
        );

        assert_eq!(output.events, vec![DragListEvent::Cancelled]);
        assert!(!output.pointer_mutation());
        assert!(!state.is_capturing());
    }

    #[test]
    fn the_bar_arrows_track_and_thumb_scroll_without_touching_the_selection() {
        let rows = rows(15);
        let mut state = DragListState::selecting(Some(0));
        let bar_x = 7.0 + 222.0 - 5.0;

        click(&mut state, GRID, &rows, (bar_x, 286.0));
        assert_eq!(state.first_row(), 1, "the down arrow steps on release");

        let page = state.update(GRID, &rows, press((bar_x, 280.0)));
        let _ = state.update(GRID, &rows, release((bar_x, 280.0)));
        assert_eq!(state.first_row(), 2, "the track pages to the last row");
        assert_eq!(page.pressed(), None, "the bar owns its press");

        click(&mut state, GRID, &rows, (bar_x, 84.0));
        assert_eq!(state.first_row(), 1, "the up arrow steps back");

        let _ = state.update(GRID, &rows, press((bar_x, 160.0)));
        let _ = state.update(GRID, &rows, hold((bar_x, 290.0)));
        assert_eq!(state.first_row(), 2, "the thumb follows its drag");
        let _ = state.update(GRID, &rows, release((bar_x, 290.0)));

        assert_eq!(state.selection(&rows), vec![0]);
    }

    #[test]
    fn the_bar_paints_at_the_first_row_its_own_frame_committed() {
        let rows = rows(24);
        let mut state = DragListState::<u32> {
            first_row: 1,
            ..DragListState::default()
        };
        let thumb = (224.0, 120.0);

        let _ = state.update(GRID, &rows, press(thumb));
        let _ = state.update(GRID, &rows, hold((224.0, 250.0)));

        // Eight rows of three show three, so the bottom of the track is row 5.
        assert_eq!(state.first_row(), 5);
        assert_eq!(state.geometry(GRID, rows.len()).range().first, 5);
    }

    /// A stacked list whose items span the bar's column once it shows.
    const BARRED: DragListSpec = DragListSpec {
        rect: (0.0, 0.0, 100.0, 100.0),
        item: (100.0, 20.0),
        layout: DragListLayout::Stacked,
        selection: SelectionPolicy::Single,
        drag: DragPolicy::Disabled,
        wheel: WheelPolicy::Ignored,
        bar: Some(LIST_BAR_29CC),
    };

    #[test]
    fn input_over_the_bar_column_never_reaches_the_item_under_it() {
        let rows = rows(10);
        let mut state = DragListState::default();
        let right = |at, pressed| DragListInput {
            pointer: Some(at),
            secondary_pressed: pressed,
            secondary_released: !pressed,
            ..DragListInput::default()
        };

        click(&mut state, BARRED, &rows, (89.5, 5.0));
        assert_eq!(
            state.selection(&rows),
            vec![0],
            "left of the bar is the item"
        );

        for at in [(90.0, 25.0), (99.5, 25.0)] {
            let pressed = state.update(BARRED, &rows, right(at, true));
            let released = state.update(BARRED, &rows, right(at, false));
            assert_eq!(pressed.pressed(), None, "{at:?}");
            assert_eq!(released.context_released(), None, "{at:?}");
        }
        assert_eq!(state.selection(&rows), vec![0]);
    }

    #[test]
    fn the_lists_right_and_bottom_edges_lie_outside_it() {
        // Win32 client rectangles are half-open.
        let rows = rows(8);
        let mut state = DragListState::default();
        let right_release = |at| DragListInput {
            pointer: Some(at),
            secondary_released: true,
            ..DragListInput::default()
        };

        assert_eq!(
            state
                .update(STACKED, &rows, right_release((99.0, 30.0)))
                .context_released(),
            None
        );
        let foot = state.update(STACKED, &rows, press((10.0, 299.0)));
        assert_eq!(foot.events, []);
        let _ = state.update(STACKED, &rows, release((10.0, 299.0)));
        let above = state.update(STACKED, &rows, press((10.0, 298.5)));
        assert_eq!(above.pressed(), Some((PointerButtonId::Primary, Some(5))));
    }

    #[test]
    fn a_deferred_release_below_the_list_clears_the_selection() {
        let rows = rows(15);
        let mut state = DragListState::selecting(Some(0));
        let _ = state.update(GRID, &rows, press(grid_cell(0, 0)));
        assert!(state.is_capturing());

        let _ = state.update(GRID, &rows, release((42.0, 300.0)));

        assert_eq!(state.selection(&rows), Vec::<u32>::new());
    }

    #[test]
    fn a_cells_last_pixel_column_still_belongs_to_it() {
        let rows = rows(9);
        let mut state = DragListState::default();

        let output = state.update(GRID, &rows, press((76.5, 116.0)));

        assert_eq!(output.pressed(), Some((PointerButtonId::Primary, Some(0))));
    }

    #[test]
    fn a_scrolled_grid_paints_from_its_first_rows_first_item() {
        let state = DragListState::<u32> {
            first_row: 1,
            ..DragListState::default()
        };
        let geometry = state.geometry(GRID, 15);
        let cells: Vec<_> = geometry.cells().collect();

        assert_eq!(cells.len(), 9);
        assert_eq!(cells[0], (3, (7.0, 81.0, 70.0, 70.0)));
        assert_eq!(cells[4], (7, (77.0, 151.0, 70.0, 70.0)));
    }

    #[test]
    fn a_cancelled_capture_drops_its_deferred_selection() {
        let rows = rows(9);
        let mut state = DragListState::selecting(Some(0));
        let _ = state.update(GRID, &rows, press(grid_cell(0, 0)));

        assert!(state.cancel_capture());
        assert!(!state.cancel_capture());
        let _ = state.update(GRID, &rows, release(grid_cell(1, 0)));

        assert_eq!(state.selection(&rows), vec![0]);
    }

    #[test]
    fn the_wheel_reaches_but_never_passes_the_last_first_row() {
        let rows = rows(15);
        let spec = DragListSpec {
            wheel: WheelPolicy::Rows,
            ..GRID
        };
        let mut state = DragListState::<u32> {
            first_row: 1,
            ..DragListState::default()
        };
        let wheel = DragListInput {
            pointer: Some(grid_cell(0, 0)),
            wheel: -1.0,
            ..DragListInput::default()
        };

        let _ = state.update(spec, &rows, wheel);
        assert_eq!(state.first_row(), 2);
        let _ = state.update(spec, &rows, wheel);
        assert_eq!(state.first_row(), 2);
    }

    #[test]
    fn a_drop_needs_a_squared_distance_above_twenty_four() {
        let rows = rows(9);
        let spec = DragListSpec {
            rect: (0.0, 0.0, 222.0, 210.0),
            drag: DragPolicy::DropAnywhere,
            bar: None,
            ..GRID
        };
        let origin = (0.0, 0.0);
        let drag = |to: (f32, f32)| {
            let mut state = DragListState::default();
            let _ = state.update(spec, &rows, press(origin));
            let _ = state.update(spec, &rows, hold(to));
            state.update(spec, &rows, release(to)).dropped()
        };

        let five = (origin.0 + 5.0, origin.1);
        assert_eq!(drag(five), Some(five), "25 drops");
        assert_eq!(
            drag((origin.0 + 2.0, origin.1 + 20.0_f32.sqrt())),
            None,
            "exactly 24 does not drop"
        );
        assert_eq!(drag((origin.0 + 4.0, origin.1 + 2.0)), None, "20 does not");
    }

    #[test]
    fn enter_with_nothing_selected_selects_the_first_row_and_down_stops_at_the_last() {
        let rows = rows(3);
        let mut state = DragListState::default();

        let enter = state.update(STACKED, &rows, key(ListKey::Enter));
        assert_eq!(enter.events, vec![DragListEvent::Activated]);
        assert_eq!(state.selection(&rows), vec![0]);

        let _ = state.update(STACKED, &rows, key(ListKey::End));
        let down = state.update(STACKED, &rows, key(ListKey::Down));
        assert_eq!(down.events, []);
        let again = state.update(STACKED, &rows, key(ListKey::Enter));
        assert_eq!(again.events, vec![DragListEvent::Activated]);
        assert_eq!(state.selected(), [2]);
    }

    #[test]
    fn movement_from_an_unselected_list_starts_at_its_first_row() {
        let rows = rows(3);
        let mut state = DragListState::default();

        let moved = state.update(STACKED, &rows, key(ListKey::Down));

        assert!(moved.contains(DragListEvent::SelectionChanged));
        assert_eq!(state.selection(&rows), vec![0]);
    }

    #[test]
    fn a_shown_bar_takes_its_width_from_the_grid_before_it_wraps() {
        // 3 cells of 70 fit 215 pixels, but not once a 10-pixel bar shows.
        let spec = DragListSpec {
            rect: (0.0, 0.0, 215.0, 140.0),
            ..GRID
        };
        let state = DragListState::<u32>::default();

        assert_eq!(state.geometry(spec, 6).columns(), 3);
        assert_eq!(state.geometry(spec, 7).columns(), 2);
    }

    #[test]
    fn the_wheel_scrolls_only_lists_that_opt_in() {
        let rows = rows(15);
        let wheel = DragListInput {
            pointer: Some(grid_cell(0, 0)),
            wheel: -1.0,
            ..DragListInput::default()
        };
        let mut ignored = DragListState::default();
        let mut rows_wheel = DragListState::default();
        let spec = DragListSpec {
            wheel: WheelPolicy::Rows,
            ..GRID
        };

        let _ = ignored.update(GRID, &rows, wheel);
        let output = rows_wheel.update(spec, &rows, wheel);

        assert_eq!(ignored.first_row(), 0);
        assert_eq!(rows_wheel.first_row(), 1);
        assert_eq!(output.phases, vec![PointerPhase::Auxiliary]);
    }

    #[test]
    fn keys_move_from_the_first_selected_item_and_reveal_it() {
        let rows = rows(15);
        let mut state = DragListState::selecting(Some(0));
        let spec = DragListSpec {
            selection: SelectionPolicy::Single,
            ..GRID
        };

        let up = state.update(spec, &rows, key(ListKey::Up));
        assert!(up.events.is_empty(), "the first item has no previous one");

        let _ = state.update(spec, &rows, key(ListKey::End));
        assert_eq!(state.selection(&rows), vec![14]);
        assert_eq!(state.first_row(), 2);

        let _ = state.update(spec, &rows, key(ListKey::PageUp));
        assert_eq!(state.selection(&rows), vec![11]);
        let _ = state.update(spec, &rows, key(ListKey::Home));
        assert_eq!(state.selection(&rows), vec![0]);
        assert_eq!(state.first_row(), 0);
        let _ = state.update(spec, &rows, key(ListKey::PageDown));
        let _ = state.update(spec, &rows, key(ListKey::Down));
        assert_eq!(state.selection(&rows), vec![4]);

        let enter = state.update(spec, &rows, key(ListKey::Enter));
        assert_eq!(enter.events, vec![DragListEvent::Activated]);
    }

    #[test]
    fn a_key_in_a_multiple_selection_list_moves_only_the_current_item() {
        // FUN_00609500 clears the other items only under flag 0x10000.
        let rows = rows(9);
        let mut state = DragListState {
            selected: vec![2, 5],
            ..DragListState::default()
        };

        let _ = state.update(GRID, &rows, key(ListKey::Down));

        assert_eq!(state.selection(&rows), vec![3, 5]);
    }

    #[test]
    fn a_refilled_list_drops_selected_items_it_no_longer_holds() {
        let mut state = DragListState::selecting(Some(7));

        let _ = state.update(GRID, &rows(3), DragListInput::default());

        assert_eq!(state.first_selected(&rows(9)), None);
    }

    #[test]
    fn an_owner_selected_item_is_revealed_without_reimplementing_list_geometry() {
        let rows = rows(15);
        let mut state = DragListState::selecting(Some(0));

        state.select_and_reveal(GRID, &rows, Some(14));

        assert_eq!(state.selection(&rows), vec![14]);
        assert_eq!(state.first_row(), 2);
    }

    #[test]
    fn a_press_outside_the_list_or_past_its_last_cell_hits_nothing() {
        let rows = rows(4);
        let mut state = DragListState::selecting(Some(0));

        let outside = state.update(GRID, &rows, press((2.0, 2.0)));
        let _ = state.update(GRID, &rows, release((2.0, 2.0)));
        assert_eq!(outside.events, []);

        let past = state.update(GRID, &rows, press(grid_cell(2, 1)));
        assert_eq!(past.pressed(), Some((PointerButtonId::Primary, None)));
    }
}
