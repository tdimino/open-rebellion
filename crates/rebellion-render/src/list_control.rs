//! Shared behavior of the original `CoolDragList` control
//! (`FUN_00607ea0`).
//!
//! This module owns list-control policy rather than any window family's
//! policy. The original Finder, Encyclopedia, Message Index, report, detail,
//! manufacturing, and battle-alert windows all instantiate the same control.

use std::ops::Range;

use egui_macroquad::egui;

use crate::native_geometry::rect_contains;

/// The result of the original list's ordered, case-insensitive prefix scan.
///
/// Callers keep the score because the original Finder wrappers clear their
/// selection when it is zero, while the Encyclopedia retains the first row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PrefixMatch {
    pub(crate) index: usize,
    pub(crate) shared_chars: usize,
}

/// Find the first row with the longest case-insensitive prefix shared with
/// `query`, matching `FUN_00609650`'s ordered scan.
///
/// Empty queries and empty lists have no match. A non-empty query can return a
/// zero-length match so each recovered parent wrapper can preserve its own
/// selection policy.
pub(crate) fn longest_prefix_match<'a>(
    labels: impl IntoIterator<Item = &'a str>,
    query: &str,
) -> Option<PrefixMatch> {
    if query.is_empty() {
        return None;
    }

    labels
        .into_iter()
        .enumerate()
        .map(|(index, label)| PrefixMatch {
            index,
            shared_chars: label
                .chars()
                .zip(query.chars())
                .take_while(|(label, query)| label.eq_ignore_ascii_case(query))
                .count(),
        })
        .reduce(|best, candidate| {
            if candidate.shared_chars > best.shared_chars {
                candidate
            } else {
                best
            }
        })
}

/// One native bounded-selection movement handled by `CoolDragList`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ListMove {
    Previous,
    Next,
    PagePrevious,
    PageNext,
    First,
    Last,
}

/// Resolve a bounded row index for one native list movement.
///
/// With no current selection, previous/next select the first row, while a
/// page-next starts one page after it. This is the behavior recovered from
/// `FUN_006083c0` and used by the Encyclopedia index.
pub(crate) fn move_selection(
    current: Option<usize>,
    row_count: usize,
    page_rows: usize,
    movement: ListMove,
) -> Option<usize> {
    let last = row_count.checked_sub(1)?;
    let current = current.map(|current| current.min(last));
    Some(match movement {
        ListMove::Previous => current.unwrap_or(0).saturating_sub(1),
        ListMove::Next => current.map_or(0, |current| current.saturating_add(1).min(last)),
        ListMove::PagePrevious => current.unwrap_or(0).saturating_sub(page_rows),
        ListMove::PageNext => current.unwrap_or(0).saturating_add(page_rows).min(last),
        ListMove::First => 0,
        ListMove::Last => last,
    })
}

/// The furthest first row that still fills a viewport when possible.
pub(crate) const fn max_first_row(row_count: usize, visible_rows: usize) -> usize {
    row_count.saturating_sub(visible_rows)
}

/// Number of complete rows that fit in one list-control viewport.
///
/// Window families supply their recovered pixel geometry; the shared
/// `CoolDragList` control owns the truncation to whole rows.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "Validated non-negative list geometry is truncated to complete rows."
)]
pub(crate) const fn visible_row_capacity(viewport_height: f32, row_height: f32) -> usize {
    assert!(
        viewport_height >= 0.0,
        "list viewport height must be non-negative"
    );
    assert!(row_height > 0.0, "list row height must be positive");
    (viewport_height / row_height) as usize
}

/// Bound a stored first row after the list or viewport size changes.
pub(crate) fn clamp_first_row(first_row: usize, row_count: usize, visible_rows: usize) -> usize {
    first_row.min(max_first_row(row_count, visible_rows))
}

/// Return the bounded row indices visible in the current viewport.
pub(crate) fn visible_range(
    first_row: usize,
    row_count: usize,
    visible_rows: usize,
) -> Range<usize> {
    let start = first_row.min(row_count);
    start..row_count.min(start.saturating_add(visible_rows))
}

/// Return the selected row when the optional index is valid.
pub(crate) fn selected_row<R>(rows: &[R], selected: Option<usize>) -> Option<&R> {
    rows.get(selected?)
}

/// Apply one wheel step through `positions` bounded list positions.
///
/// Negative wheel input advances and positive input moves back, matching the
/// list popups and list viewports in the original strategic UI.
pub(crate) const fn wheel_step(current: usize, positions: usize, wheel: f32) -> usize {
    if wheel < 0.0 && current.saturating_add(1) < positions {
        current + 1
    } else if wheel > 0.0 {
        current.saturating_sub(1)
    } else {
        current
    }
}

/// Read wheel input only while the pointer is inside a half-open list aperture.
pub(crate) fn wheel_delta(ui: &egui::Ui, list_rect: egui::Rect) -> f32 {
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

/// A list row's single- or double-click interpretation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum RowClick {
    None,
    Select(f64),
    Open,
}

/// Interpret one `CoolDragList` row's click and double-click semantics.
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

/// Move the first visible row only far enough to reveal `selected`.
///
/// `FUN_00609ed0` applies this after a `CoolDragList` item is selected,
/// including selections made by the shared title-prefix search
/// (`FUN_00609650`).
pub(crate) const fn reveal_row(first_row: usize, selected: usize, visible_rows: usize) -> usize {
    if selected < first_row {
        selected
    } else if selected >= first_row.saturating_add(visible_rows) {
        selected.saturating_add(1).saturating_sub(visible_rows)
    } else {
        first_row
    }
}

#[cfg(test)]
mod tests {
    use super::{
        clamp_first_row, longest_prefix_match, reveal_row, visible_range, visible_row_capacity,
        PrefixMatch,
    };

    #[test]
    fn prefix_search_keeps_the_first_strictly_longest_case_insensitive_match() {
        let labels = ["Corulag", "Coruscant", "Yavin"];

        assert_eq!(
            ["CORUS", "cor", "z", ""].map(|query| longest_prefix_match(labels, query)),
            [
                Some(PrefixMatch {
                    index: 1,
                    shared_chars: 5,
                }),
                Some(PrefixMatch {
                    index: 0,
                    shared_chars: 3,
                }),
                Some(PrefixMatch {
                    index: 0,
                    shared_chars: 0,
                }),
                None,
            ]
        );
    }

    #[test]
    fn viewport_helpers_bound_offsets_and_visible_indices() {
        assert_eq!(visible_row_capacity(194.0, 21.0), 9);
        assert_eq!(
            [
                clamp_first_row(5, 3, 8),
                clamp_first_row(2, 11, 8),
                clamp_first_row(5, 11, 8),
            ],
            [0, 2, 3]
        );
        assert_eq!(
            [
                visible_range(2, 11, 8),
                visible_range(10, 11, 8),
                visible_range(4, 0, 8),
            ],
            [2..10, 10..11, 0..0]
        );
    }

    #[test]
    fn selection_reveal_scrolls_only_far_enough_to_show_the_row() {
        let visible_rows = 8;
        let first_row = 5;
        let last_visible = visible_rows - 1;
        let selected_at_end = first_row + last_visible;

        assert_eq!(reveal_row(0, last_visible, visible_rows), 0);
        assert_eq!(reveal_row(0, visible_rows, visible_rows), 1);
        assert_eq!(reveal_row(first_row, first_row - 2, visible_rows), 3);
        assert_eq!(reveal_row(first_row, selected_at_end, visible_rows), 5);
        assert_eq!(reveal_row(first_row - 1, selected_at_end, visible_rows), 5);
    }
}
