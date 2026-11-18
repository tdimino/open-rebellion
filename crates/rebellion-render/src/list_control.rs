//! Shared behavior of the original `CoolDragList` control
//! (`FUN_00607ea0`).
//!
//! This module owns list-control policy rather than any window family's
//! policy. The original Finder, Encyclopedia, Message Index, report, detail,
//! manufacturing, and battle-alert windows all instantiate the same control.

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
    use super::{longest_prefix_match, reveal_row, visible_row_capacity, PrefixMatch};

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
    fn viewport_capacity_uses_only_complete_rows() {
        assert_eq!(visible_row_capacity(194.0, 21.0), 9);
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
