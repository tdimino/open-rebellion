//! The original `CoolSelectionBoxClass` and its drop-down popup.
//!
//! `FUN_0060bed0` constructs the box: its rect, the popup's row count
//! (`+0xa8`) and row height (`+0xa4`, the box's height), the items it shares
//! with every popup (`+0xd0`), the current item (`+0xe0`) and the popup
//! scroll bar's art (`+0xbc`, `+0xbe`). Its `WndProc` `FUN_0060c210` opens the
//! popup on command `0x3ea` or closes it when it is already open, and closes
//! it on command `0x3e9`. `FUN_0060cac0` places the popup under the box, the
//! box's width and `rows * row height + 4` high, above the box when that
//! would pass the screen's bottom, and clamped to the screen's top
//! (`+0xc4`, `+0xcc`). `FUN_0060cc50` builds the popup window, and its
//! `WndProc` `FUN_0060d020` takes the capture, hosts a `CoolDragList` two
//! pixels in with flags `0x70000` (single selection, no drag, hover tracking),
//! reveals the current item (`FUN_00609ed0`), dismisses on any press outside
//! its client area (`0x3e9`), forwarding nothing, and on the list's `0x29b`
//! commits the first selected item through `FUN_0060c970`, which notifies
//! the owner only for a new item, and closes (`FUN_0060cbf0`).
//!
//! The popup's list is the shared [`DragList`]. Item content and every game
//! rule stay with the feature: it passes its typed rows and the current item,
//! paints the rows this control lays out, and applies the typed outcome.

#![allow(
    clippy::module_name_repetitions,
    reason = "The source class name is the public renderer object name."
)]

use std::hash::Hash;

use egui_macroquad::egui;

use super::drag_list::{
    DragList, DragListInput, DragListLayout, DragListOutput, DragListResponse, DragListSpec,
    DragListState, DragPolicy, SelectionPolicy, WheelPolicy,
};
use super::ScrollBarArt;
use crate::bmp_cache::BmpCache;
use crate::cockpit::CockpitLayout;
use crate::native_geometry::logical_rect;
use crate::windowing_input::PointerButtonId;

/// The popup's border on each side, around its list: `FUN_0060cac0` adds 4
/// to the rows' height and `FUN_0060d020` builds the list at (2, 2), 4
/// narrower and lower than the popup.
const POPUP_INSET: f32 = 2.0;

/// A box's recovered configuration, in its window's logical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SelectionBoxSpec {
    /// The box's x, y, width and height in its window.
    pub(crate) rect: (f32, f32, f32, f32),
    /// The popup's rows (`+0xa8`).
    pub(crate) rows: usize,
    /// A popup row's height (`+0xa4`).
    pub(crate) row_height: f32,
    /// The popup bar's art (`+0xbc`, `+0xbe`), when the feature ports it.
    pub(crate) bar: Option<ScrollBarArt>,
}

/// The screen edges `FUN_0060cac0` places the popup against (`+0xc4`, the
/// top, and `+0xcc`, `SM_CYFULLSCREEN`), in the box window's logical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ScreenBounds {
    pub(crate) top: f32,
    pub(crate) bottom: f32,
}

impl ScreenBounds {
    /// The 640 by 480 cockpit canvas the original fills, from `window`'s
    /// origin. port: `SM_CYFULLSCREEN` is the screen less the system
    /// caption's height, which the port does not model; the canvas's bottom
    /// stands for it.
    pub(crate) fn canvas(layout: CockpitLayout, window: egui::Rect) -> Self {
        let scale = layout.scale.max(f32::EPSILON);
        Self {
            top: (layout.canvas.y - window.min.y) / scale,
            bottom: (layout.canvas.y + layout.canvas.height - window.min.y) / scale,
        }
    }
}

impl SelectionBoxSpec {
    /// `FUN_0060cac0`: under the box, its width and `rows * row height + 4`
    /// high; above it when that passes the screen's bottom, and from the
    /// screen's top down to the box when above passes the top.
    #[allow(
        clippy::cast_precision_loss,
        reason = "A recovered row count is a small whole number."
    )]
    pub(crate) fn popup(&self, bounds: ScreenBounds) -> (f32, f32, f32, f32) {
        let (x, y, width, height) = self.rect;
        let mut popup_height = self.rows as f32 * self.row_height + 2.0 * POPUP_INSET;
        let mut top = y + height;
        if top + popup_height > bounds.bottom {
            top = y - popup_height;
            if top < bounds.top {
                popup_height = y - bounds.top;
                top = bounds.top;
            }
        }
        (x, top, width, popup_height)
    }

    /// The popup's list (`FUN_0060d020`, `FUN_00607ea0` with flags
    /// `0x70000`): two pixels in, rows the box's width less the border.
    fn list(&self, bounds: ScreenBounds) -> DragListSpec {
        let (x, y, width, height) = self.popup(bounds);
        let inner_width = width - 2.0 * POPUP_INSET;
        DragListSpec {
            rect: (
                x + POPUP_INSET,
                y + POPUP_INSET,
                inner_width,
                height - 2.0 * POPUP_INSET,
            ),
            item: (inner_width, self.row_height),
            layout: DragListLayout::Stacked,
            selection: SelectionPolicy::Single,
            drag: DragPolicy::Disabled,
            wheel: WheelPolicy::Rows,
            bar: self.bar,
        }
    }
}

/// One typed outcome of a box frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SelectionBoxEvent<K> {
    /// Command `0x3ea` on a closed box opened the popup.
    Opened,
    /// Command `0x3ea` on an open box, or a commit of the current item,
    /// closed the popup.
    Closed,
    /// The popup committed a new item (`FUN_0060c970` posts `0x29b` to the
    /// owner) and closed.
    Selected(K),
    /// A press outside the popup closed it (`0x3e9`); the press went no
    /// further.
    Dismissed,
}

/// The open popup's own state; a new popup starts afresh.
#[derive(Debug, Clone, PartialEq)]
struct Popup<K> {
    /// The list's first row, press and capture.
    list: DragListState<K>,
    /// The item this list last marked hot (list `+0x100`).
    last_hot: Option<K>,
    /// A left press landed on a selected item (list flag `0x10`): the
    /// release posts `0x29b` instead.
    deferred: bool,
}

/// One box: the open popup and the item bits its popups share.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SelectionBoxState<K> {
    popup: Option<Popup<K>>,
    /// Each item's `+0x3c` bit 0, the pick: the items outlive each popup
    /// (`FUN_00608280` hands the list the box's own), so the pick does too.
    picked: Option<K>,
    /// Each item's `+0x3c` bit 1, set while a list's pointer is on it.
    hot: Vec<K>,
}

impl<K> Default for SelectionBoxState<K> {
    fn default() -> Self {
        Self {
            popup: None,
            picked: None,
            hot: Vec::new(),
        }
    }
}

/// Current-frame input facts in the box window's logical coordinates, for
/// driving the state without egui.
#[cfg(test)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct SelectionBoxInput {
    /// The popup's list input, its presses already admitted to the list.
    pub(crate) list: DragListInput,
    /// A press outside the popup's client area, which its capture receives.
    pub(crate) outside_press: bool,
}

impl<K: Copy + PartialEq> SelectionBoxState<K> {
    pub(crate) const fn is_open(&self) -> bool {
        self.popup.is_some()
    }

    /// Whether `item`'s pick bit is set.
    pub(crate) fn is_picked(&self, item: K) -> bool {
        self.picked == Some(item)
    }

    /// Whether `item`'s hover bit is set.
    pub(crate) fn is_hot(&self, item: K) -> bool {
        self.hot.contains(&item)
    }

    /// Command `0x3ea` (`FUN_0060c210`): close an open popup, else open a
    /// new one that reveals `current` (`FUN_0060d020`, `FUN_00609ed0`) and
    /// keeps the items' pick.
    pub(crate) fn toggle(
        &mut self,
        spec: SelectionBoxSpec,
        bounds: ScreenBounds,
        rows: &[K],
        current: Option<K>,
    ) -> SelectionBoxEvent<K> {
        if self.popup.take().is_some() {
            return SelectionBoxEvent::Closed;
        }
        let mut list = DragListState::default();
        list.select_and_reveal(spec.list(bounds), rows, current);
        list.select_only(self.picked);
        self.popup = Some(Popup {
            list,
            last_hot: None,
            deferred: false,
        });
        SelectionBoxEvent::Opened
    }

    /// `FUN_0060cbf0`: close the popup without a commit, as the owner does
    /// when it hides the box.
    pub(crate) fn close(&mut self) {
        self.popup = None;
    }

    /// Apply one input frame to an open popup; a closed box takes none. The
    /// egui adapter admits the list's presses and runs the same steps.
    #[cfg(test)]
    pub(crate) fn update(
        &mut self,
        spec: SelectionBoxSpec,
        bounds: ScreenBounds,
        rows: &[K],
        current: Option<K>,
        input: SelectionBoxInput,
    ) -> Option<SelectionBoxEvent<K>> {
        self.popup.as_ref()?;
        if input.outside_press {
            self.close();
            return Some(SelectionBoxEvent::Dismissed);
        }
        let list_spec = spec.list(bounds);
        let popup = self.popup.as_mut()?;
        let before = popup.list.clone();
        let output = popup.list.update(list_spec, rows, input.list);
        self.after_list(list_spec, rows, current, &before, &output, input.list)
    }

    /// The popup's share of a list frame: hover bits (flag `0x40000`), the
    /// pick, and the commit on the list's `0x29b`.
    fn after_list(
        &mut self,
        list_spec: DragListSpec,
        rows: &[K],
        current: Option<K>,
        before: &DragListState<K>,
        output: &DragListOutput<K>,
        input: DragListInput,
    ) -> Option<SelectionBoxEvent<K>> {
        let popup = self.popup.as_mut()?;
        // FUN_006083c0 case 0x200 with flag 0x40000: the item under the
        // pointer gains bit 1, the one this list marked before loses it.
        if let Some(at) = input.pointer.filter(|&(x, y)| {
            let (left, top, width, height) = list_spec.rect;
            x >= left && x < left + width && y >= top && y < top + height
        }) {
            let geometry = popup.list.geometry(list_spec, rows.len());
            let item = geometry
                .cells()
                .find(|&(_, (x, y, width, height))| {
                    at.0 >= x && at.0 < x + width && at.1 >= y && at.1 < y + height
                })
                .map(|(index, _)| rows[index]);
            if item != popup.last_hot {
                if let Some(old) = popup.last_hot {
                    self.hot.retain(|hot| *hot != old);
                }
                popup.last_hot = item;
                if let Some(item) = item.filter(|item| !self.hot.contains(item)) {
                    self.hot.push(item);
                }
            }
        }
        // FUN_006083c0 posts 0x29b on every admitted press that is not
        // deferred (cases 0x201 and 0x204, :557-585) and on the release
        // that ends a deferred left press (case 0x202, :326-356), whether
        // or not the selection changed. A deferred right press posts none.
        let mut posted = false;
        if let Some((button, item)) = output.pressed() {
            let deferred = item.is_some_and(|item| before.is_selected(item));
            if deferred {
                popup.deferred = button == PointerButtonId::Primary;
            } else {
                posted = true;
            }
        } else if popup.deferred && input.primary_released {
            popup.deferred = false;
            posted = true;
        } else if output.contains(super::drag_list::DragListEvent::Cancelled) {
            popup.deferred = false;
        }
        self.picked = popup.list.first_selected(rows);
        if !posted {
            return None;
        }
        // FUN_0060d020 case 0x111: the first selected item, if any, goes to
        // FUN_0060c970, which notifies only for a new item, then closes.
        let item = self.picked?;
        self.close();
        Some(if Some(item) == current {
            SelectionBoxEvent::Closed
        } else {
            SelectionBoxEvent::Selected(item)
        })
    }
}

/// One egui frame of a box: its outcome and, while it stays open, the popup
/// and the rows to paint.
pub(crate) struct SelectionBoxResponse<K> {
    pub(crate) event: Option<SelectionBoxEvent<K>>,
    /// The popup's frame in screen space while it is open.
    pub(crate) popup: Option<egui::Rect>,
    list: Option<DragListResponse<K>>,
}

impl<K: Copy + PartialEq> SelectionBoxResponse<K> {
    /// The open popup's rows to paint in screen space: item and cell.
    pub(crate) fn rows<'a>(
        &'a self,
        rows: &'a [K],
        window: egui::Rect,
        scale: f32,
    ) -> impl Iterator<Item = (K, egui::Rect)> + 'a {
        self.list
            .iter()
            .flat_map(move |list| list.screen_cells(window, scale))
            .map(|(index, cell)| (rows[index], cell))
    }

    /// Paint the popup's composed bar after its rows.
    pub(crate) fn paint_bar(
        &self,
        painter: &egui::Painter,
        ctx: &egui::Context,
        cache: &mut BmpCache,
        window: egui::Rect,
        scale: f32,
    ) {
        if let Some(list) = &self.list {
            list.paint_bar(painter, ctx, cache, window, scale);
        }
    }
}

/// The egui adapter for a [`SelectionBoxState`].
pub(crate) struct SelectionBox;

impl SelectionBox {
    /// Run an open popup for one frame and paint its frame. The popup holds
    /// the capture (`FUN_0060d020`): a surface the size of the screen, under
    /// the popup and over everything the box's window drew before it, takes
    /// every press off the popup, so a dismissing press reaches no other
    /// control. A press on the popup's border reaches nothing.
    #[allow(
        clippy::too_many_arguments,
        reason = "The control boundary keeps identity, geometry, rows, state and current item explicit."
    )]
    pub(crate) fn show<K: Copy + PartialEq>(
        ui: &mut egui::Ui,
        painter: &egui::Painter,
        id: impl Hash,
        spec: SelectionBoxSpec,
        bounds: ScreenBounds,
        rows: &[K],
        state: &mut SelectionBoxState<K>,
        current: Option<K>,
        window: egui::Rect,
        scale: f32,
    ) -> SelectionBoxResponse<K> {
        let mut response = SelectionBoxResponse {
            event: None,
            popup: None,
            list: None,
        };
        let Some(popup) = state.popup.as_ref() else {
            return response;
        };
        let id = ui.id().with(id);
        let screen = ui.ctx().screen_rect();
        let clip = ui.clip_rect();
        ui.set_clip_rect(screen);
        let capture = ui.interact(screen, id.with("capture"), egui::Sense::click());
        let (x, y, width, height) = spec.popup(bounds);
        let frame = logical_rect(window, scale, x, y, width, height);
        ui.interact(frame, id.with("frame"), egui::Sense::click());
        ui.set_clip_rect(clip);
        let captured =
            capture.is_pointer_button_down_on() || capture.clicked() || capture.secondary_clicked();
        let outside_press = captured
            && ui.ctx().input(|input| {
                input.pointer.button_pressed(egui::PointerButton::Primary)
                    || input.pointer.button_pressed(egui::PointerButton::Secondary)
            });
        if outside_press {
            state.close();
            response.event = Some(SelectionBoxEvent::Dismissed);
            return response;
        }
        let list_spec = spec.list(bounds);
        let shown = DragList::show(
            ui,
            id.with("list"),
            list_spec,
            rows,
            &popup.list,
            window,
            scale,
            None,
        );
        // The facts the popup reads beside the list's own outcome.
        let list_input = ui.ctx().input(|input| DragListInput {
            pointer: input.pointer.interact_pos().map(|point| {
                (
                    (point.x - window.min.x) / scale,
                    (point.y - window.min.y) / scale,
                )
            }),
            primary_released: input.pointer.button_released(egui::PointerButton::Primary),
            ..DragListInput::default()
        });
        let before = popup.list.clone();
        if let Some(popup) = state.popup.as_mut() {
            popup.list = shown.next.clone();
        }
        response.event =
            state.after_list(list_spec, rows, current, &before, &shown.output, list_input);
        if state.is_open() {
            painter.rect_filled(frame, 0.0, egui::Color32::BLACK);
            response.popup = Some(frame);
            response.list = Some(shown);
        }
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build Selection's class box (`FUN_00437f80`): 195 by 63 at (6, 22),
    /// two 70-pixel rows.
    const BOX: SelectionBoxSpec = SelectionBoxSpec {
        rect: (6.0, 22.0, 195.0, 63.0),
        rows: 2,
        row_height: 70.0,
        bar: None,
    };
    const SCREEN: ScreenBounds = ScreenBounds {
        top: -100.0,
        bottom: 380.0,
    };

    fn rows(count: u8) -> Vec<u8> {
        (0..count).collect()
    }

    fn opened(current: Option<u8>, count: u8) -> SelectionBoxState<u8> {
        let mut state = SelectionBoxState::default();
        assert_eq!(
            state.toggle(BOX, SCREEN, &rows(count), current),
            SelectionBoxEvent::Opened
        );
        state
    }

    /// A list point on row `slot` of the open popup at (8, 87).
    fn row_point(slot: f32) -> (f32, f32) {
        (100.0, 87.0 + 70.0 * slot + 35.0)
    }

    fn press_at(at: (f32, f32)) -> SelectionBoxInput {
        SelectionBoxInput {
            list: DragListInput {
                pointer: Some(at),
                primary_pressed: true,
                primary_down: true,
                ..DragListInput::default()
            },
            outside_press: false,
        }
    }

    fn release_at(at: (f32, f32)) -> SelectionBoxInput {
        SelectionBoxInput {
            list: DragListInput {
                pointer: Some(at),
                primary_released: true,
                ..DragListInput::default()
            },
            outside_press: false,
        }
    }

    fn hover_at(at: (f32, f32)) -> SelectionBoxInput {
        SelectionBoxInput {
            list: DragListInput {
                pointer: Some(at),
                ..DragListInput::default()
            },
            outside_press: false,
        }
    }

    fn step(
        state: &mut SelectionBoxState<u8>,
        current: Option<u8>,
        count: u8,
        input: SelectionBoxInput,
    ) -> Option<SelectionBoxEvent<u8>> {
        state.update(BOX, SCREEN, &rows(count), current, input)
    }

    // FUN_0060cac0: y = the box's bottom, height = rows * +0xa4 + 4, width
    // the box's (+0x38).
    #[test]
    fn the_popup_opens_under_the_box_its_width_and_its_rows_plus_four_high() {
        assert_eq!(BOX.popup(SCREEN), (6.0, 85.0, 195.0, 144.0));
        let list = BOX.list(SCREEN);
        // FUN_0060d020 builds the list at (2, 2), 4 narrower and lower.
        assert_eq!(list.rect, (8.0, 87.0, 191.0, 140.0));
        assert_eq!(list.item, (191.0, 70.0));
    }

    // FUN_0060cac0: when y + height passes +0xcc the popup ends at the box's
    // top; when that starts above +0xc4 it runs from +0xc4 to the box.
    #[test]
    fn a_popup_that_would_pass_the_screens_bottom_opens_above_and_stops_at_its_top() {
        // The popup's bottom, 85 + 144, exactly at the screen's stays below.
        let at_bottom = ScreenBounds {
            top: -200.0,
            bottom: 229.0,
        };
        assert_eq!(BOX.popup(at_bottom), (6.0, 85.0, 195.0, 144.0));
        let past = ScreenBounds {
            top: -200.0,
            bottom: 228.5,
        };
        assert_eq!(BOX.popup(past), (6.0, 22.0 - 144.0, 195.0, 144.0));
        let tight = ScreenBounds {
            top: -50.0,
            bottom: 200.0,
        };
        assert_eq!(BOX.popup(tight), (6.0, -50.0, 195.0, 72.0));
        let exact = ScreenBounds {
            top: 22.0 - 144.0,
            bottom: 200.0,
        };
        assert_eq!(BOX.popup(exact), (6.0, 22.0 - 144.0, 195.0, 144.0));
    }

    // Our own helper: the canvas edges from a window's origin, in the
    // window's logical pixels.
    #[test]
    fn the_screen_bounds_are_the_canvas_edges_from_the_windows_origin() {
        let layout = crate::test_support::offset_strategic_layout(2.0);
        let window = egui::Rect::from_min_size(egui::pos2(50.0, 120.0), egui::vec2(10.0, 10.0));
        assert_eq!(
            ScreenBounds::canvas(layout, window),
            ScreenBounds {
                top: (20.0 - 120.0) / 2.0,
                bottom: (20.0 + 960.0 - 120.0) / 2.0,
            }
        );
    }

    // FUN_0060c210 command 0x3ea: open when closed, close when open.
    #[test]
    fn the_drop_down_command_opens_a_closed_box_and_closes_an_open_one() {
        let mut state = opened(Some(0), 3);
        assert!(state.is_open());
        assert_eq!(
            state.toggle(BOX, SCREEN, &rows(3), Some(0)),
            SelectionBoxEvent::Closed
        );
        assert!(!state.is_open());
        assert_eq!(step(&mut state, Some(0), 3, press_at(row_point(0.0))), None);
    }

    // FUN_0060d020 WM_CREATE: FUN_00609ed0 reveals the box's +0xe0, from row
    // 0 just far enough to show it.
    #[test]
    fn a_new_popup_scrolls_just_far_enough_to_show_the_current_item() {
        for (current, first) in [(0, 0), (1, 0), (2, 1), (4, 3)] {
            let state = opened(Some(current), 5);
            let popup = state.popup.as_ref().unwrap();
            assert_eq!(popup.list.first_row(), first, "current {current}");
        }
    }

    // FUN_006083c0 case 0x201 selects and posts 0x29b; FUN_0060d020 hands
    // the first selected item to FUN_0060c970, which posts 0x29b to the
    // owner for a new item and closes (FUN_0060cbf0).
    #[test]
    fn a_press_on_a_row_commits_its_item_and_closes_the_popup() {
        let mut state = opened(Some(0), 3);
        assert_eq!(
            step(&mut state, Some(0), 3, press_at(row_point(1.0))),
            Some(SelectionBoxEvent::Selected(1))
        );
        assert!(!state.is_open());
        assert!(state.is_picked(1));
    }

    // FUN_0060c970 skips the owner's 0x29b when the item is already current
    // but still closes.
    #[test]
    fn a_press_on_the_current_item_closes_the_popup_without_a_selection() {
        let mut state = opened(Some(1), 3);
        assert_eq!(
            step(&mut state, Some(1), 3, press_at(row_point(1.0))),
            Some(SelectionBoxEvent::Closed)
        );
        assert!(!state.is_open());
    }

    // FUN_006083c0 case 0x201 defers a press on a selected item (flag 0x10);
    // case 0x202 then selects under the pointer and posts 0x29b.
    #[test]
    fn a_press_on_the_picked_row_commits_on_its_release() {
        let mut state = opened(Some(0), 3);
        step(&mut state, Some(0), 3, press_at(row_point(1.0)));
        state.toggle(BOX, SCREEN, &rows(3), Some(1));
        assert_eq!(step(&mut state, Some(1), 3, press_at(row_point(1.0))), None);
        assert!(state.is_open());
        assert_eq!(
            step(&mut state, Some(1), 3, release_at(row_point(0.0))),
            Some(SelectionBoxEvent::Selected(0))
        );
        assert!(!state.is_open());
    }

    // Case 0x201 off every row clears the pick (FUN_006094b0) and posts
    // 0x29b; FUN_0060d020 finds no selected item and keeps the popup.
    #[test]
    fn a_press_below_the_last_row_clears_the_pick_and_keeps_the_popup() {
        let mut state = opened(Some(0), 3);
        step(&mut state, Some(0), 3, press_at(row_point(0.0)));
        assert!(state.is_picked(0));
        state.toggle(BOX, SCREEN, &rows(1), Some(0));
        assert_eq!(step(&mut state, Some(0), 1, press_at(row_point(1.0))), None);
        assert!(state.is_open());
        assert!(!state.is_picked(0));
    }

    // FUN_0060d020 cases 0x201 and 0x204: a press outside the client area
    // sends 0x3e9 to the box, which closes the popup (FUN_0060c210 ->
    // FUN_0060cbf0) with nothing committed.
    #[test]
    fn a_press_outside_dismisses_the_popup_and_commits_nothing() {
        let mut state = opened(Some(0), 3);
        let outside = SelectionBoxInput {
            outside_press: true,
            ..press_at(row_point(1.0))
        };
        assert_eq!(
            step(&mut state, Some(0), 3, outside),
            Some(SelectionBoxEvent::Dismissed)
        );
        assert!(!state.is_open());
        assert!(!state.is_picked(1));
    }

    // The popup's border is its own client area: a press there reaches no
    // list row and keeps the popup (FUN_0060d020 forwards only to children
    // under the point).
    #[test]
    fn a_press_on_the_popups_border_keeps_it_open() {
        let mut state = opened(Some(0), 3);
        for border in [(7.0, 150.0), (100.0, 86.0), (100.0, 228.0)] {
            assert_eq!(step(&mut state, Some(0), 3, press_at(border)), None);
            assert!(state.is_open(), "{border:?}");
        }
    }

    // The list's wheel scrolls its rows, not the current item.
    #[test]
    fn the_wheel_scrolls_the_popups_rows_within_the_list() {
        let mut state = opened(Some(2), 3);
        let wheel = |delta| SelectionBoxInput {
            list: DragListInput {
                pointer: Some(row_point(0.5)),
                wheel: delta,
                ..DragListInput::default()
            },
            outside_press: false,
        };
        let first = |state: &SelectionBoxState<u8>| state.popup.as_ref().unwrap().list.first_row();
        assert_eq!(first(&state), 1);
        step(&mut state, Some(2), 3, wheel(1.0));
        assert_eq!(first(&state), 0);
        step(&mut state, Some(2), 3, wheel(1.0));
        assert_eq!(first(&state), 0);
        step(&mut state, Some(2), 3, wheel(-1.0));
        assert_eq!(first(&state), 1);
        step(&mut state, Some(2), 3, wheel(-1.0));
        assert_eq!(first(&state), 1);
        assert!(state.is_open());
    }

    // FUN_006083c0 case 0x200, flag 0x40000: the hovered item gains bit 1
    // and the item this list marked before loses it. The bits live in the
    // shared items, so they outlive the popup; a new list marks none.
    #[test]
    fn hover_marks_the_row_under_the_pointer_and_the_mark_outlives_the_popup() {
        let mut state = opened(Some(0), 3);
        step(&mut state, Some(0), 3, hover_at(row_point(0.0)));
        assert!(state.is_hot(0));
        step(&mut state, Some(0), 3, hover_at(row_point(1.0)));
        assert!(!state.is_hot(0));
        assert!(state.is_hot(1));
        // Off the list the list sees no move.
        step(&mut state, Some(0), 3, hover_at((100.0, 228.0)));
        assert!(state.is_hot(1));
        state.toggle(BOX, SCREEN, &rows(3), Some(0));
        state.toggle(BOX, SCREEN, &rows(3), Some(0));
        step(&mut state, Some(0), 3, hover_at(row_point(0.0)));
        assert!(state.is_hot(0));
        assert!(state.is_hot(1));
        // Below the last row: the list's mark clears.
        state.toggle(BOX, SCREEN, &rows(1), Some(0));
        state.toggle(BOX, SCREEN, &rows(1), Some(0));
        step(&mut state, Some(0), 1, hover_at(row_point(0.0)));
        step(&mut state, Some(0), 1, hover_at(row_point(1.0)));
        assert!(!state.is_hot(0));
    }

    // Our own layout: the list spans x 8..199 and y 87..227, and row 1
    // starts at y 157; each edge belongs to what follows it.
    #[test]
    fn hover_reads_each_list_and_row_edge_as_the_next_one() {
        let mut state = opened(Some(0), 3);
        step(&mut state, Some(0), 3, hover_at((100.0, 157.0)));
        assert!(state.is_hot(1));
        assert!(!state.is_hot(0));
        step(&mut state, Some(0), 3, hover_at((100.0, 156.9)));
        assert!(state.is_hot(0));
        // The list's right and bottom edges are off it: the mark stays.
        step(&mut state, Some(0), 3, hover_at((199.0, 100.0)));
        step(&mut state, Some(0), 3, hover_at((100.0, 227.0)));
        assert!(state.is_hot(0));
        step(&mut state, Some(0), 3, hover_at((198.9, 226.9)));
        assert!(state.is_hot(1));
    }

    // A reopened popup keeps the items' pick (FUN_00608280 shares them),
    // while it reveals the current item.
    #[test]
    fn a_reopened_popup_keeps_the_pick_and_reveals_the_current_item() {
        let mut state = opened(Some(0), 3);
        step(&mut state, Some(0), 3, press_at(row_point(1.0)));
        state.toggle(BOX, SCREEN, &rows(3), Some(2));
        assert!(state.is_picked(1));
        assert_eq!(state.popup.as_ref().unwrap().list.first_row(), 1);
    }

    // A right press selects and posts 0x29b as case 0x201 does (case 0x204
    // shares it), so it commits too.
    #[test]
    fn a_right_press_on_a_row_commits_it_as_a_left_press_does() {
        let mut state = opened(Some(0), 3);
        let right = SelectionBoxInput {
            list: DragListInput {
                pointer: Some(row_point(1.0)),
                secondary_pressed: true,
                ..DragListInput::default()
            },
            outside_press: false,
        };
        assert_eq!(
            step(&mut state, Some(0), 3, right),
            Some(SelectionBoxEvent::Selected(1))
        );
    }

    // Case 0x204 on a selected item sets flag 0x10, but no 0x205 case
    // posts: the popup stays until a later post.
    #[test]
    fn a_right_press_on_the_picked_row_commits_nothing() {
        let mut state = opened(Some(0), 3);
        step(&mut state, Some(0), 3, press_at(row_point(1.0)));
        state.toggle(BOX, SCREEN, &rows(3), Some(1));
        let right = |pressed| SelectionBoxInput {
            list: DragListInput {
                pointer: Some(row_point(1.0)),
                secondary_pressed: pressed,
                secondary_released: !pressed,
                ..DragListInput::default()
            },
            outside_press: false,
        };
        assert_eq!(step(&mut state, Some(1), 3, right(true)), None);
        assert_eq!(step(&mut state, Some(1), 3, right(false)), None);
        assert!(state.is_open());
    }

    // A capture lost before the release ends the deferred press without a
    // post (FUN_006083c0 case 0x215 clears the flags).
    #[test]
    fn a_deferred_press_whose_capture_is_lost_commits_nothing() {
        let mut state = opened(Some(0), 3);
        step(&mut state, Some(0), 3, press_at(row_point(1.0)));
        state.toggle(BOX, SCREEN, &rows(3), Some(1));
        step(&mut state, Some(1), 3, press_at(row_point(1.0)));
        assert_eq!(step(&mut state, Some(1), 3, hover_at(row_point(1.0))), None);
        assert_eq!(
            step(&mut state, Some(1), 3, release_at(row_point(1.0))),
            None
        );
        assert!(state.is_open());
    }
}
