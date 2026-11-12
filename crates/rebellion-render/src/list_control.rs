//! Shared behavior of the original `CoolDragList` control
//! (`FUN_00607ea0`).
//!
//! This module owns list-control policy rather than any window family's
//! policy. The original Finder, Encyclopedia, Message Index, report, detail,
//! manufacturing, and battle-alert windows all instantiate the same control.

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
    use super::reveal_row;

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
