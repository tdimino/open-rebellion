//! Renderer-owned Galactic Encyclopedia surface data and interactions.
//!
//! These borrowed DTOs deliberately contain no `rebellion-data` types. The
//! application adapts its validated presentation into this model, preserving
//! the renderer's one-way dependency boundary.

use crate::controls::TEXT_BAR_299D;
use crate::controls::{
    DragListInput, DragListLayout, DragListSpec, DragListState, DragPolicy, ListKey,
    SelectionPolicy, WheelPolicy,
};

/// `FUN_0045ddc0`: command `0x65`, 350 by 160, 350 by 20 items, flags
/// `0x30000`, and RCDATA `0x299d`'s canonical scroll bar.
pub(crate) const ENCYCLOPEDIA_INDEX_LIST: DragListSpec = DragListSpec {
    rect: (36.0, 137.0, 350.0, 160.0),
    item: (350.0, 20.0),
    layout: DragListLayout::Stacked,
    selection: SelectionPolicy::Multiple,
    drag: DragPolicy::Disabled,
    wheel: WheelPolicy::Rows,
    bar: Some(TEXT_BAR_299D),
};

/// Original audience-specific Encyclopedia chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaSurfaceAudience {
    Alliance,
    Empire,
}

/// Which of the original viewer's two surfaces is visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaSurfaceMode {
    Index,
    Topic,
}

/// Source-backed availability of one index row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaSurfaceAvailability {
    Resolved,
    SourceUnavailable,
}

/// One source category shown by the authentic index controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncyclopediaSurfaceCategory<'a> {
    pub command_id: u16,
    pub label_resource_id: u16,
    pub label: &'a str,
    pub topic_count: usize,
}

/// One source-ordered row in the authentic index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncyclopediaSurfaceTopicItem<'a> {
    pub object_id: u32,
    pub title: &'a str,
    pub availability: EncyclopediaSurfaceAvailability,
}

/// Exact, validated artwork bytes for the active topic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncyclopediaArtworkView<'a> {
    pub resource_id: u16,
    pub filename: &'a str,
    pub digest: &'a str,
    pub width: u32,
    pub height: u32,
    pub bytes: &'a [u8],
    pub sampling: crate::encyclopedia_textures::EncyclopediaTextureSampling,
}

/// The active topic. Source-unavailable topics carry neither prose nor art.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncyclopediaSurfaceTopic<'a> {
    pub object_id: u32,
    pub title: &'a str,
    pub topic_text_resource_id: u16,
    pub description: Option<&'a str>,
    pub artwork: Option<EncyclopediaArtworkView<'a>>,
}

/// Bounded topic navigation; endpoints are represented by `None`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncyclopediaSurfaceNavigation {
    pub selected_object_id: Option<u32>,
    pub previous_object_id: Option<u32>,
    pub next_object_id: Option<u32>,
}

/// Complete graphics-neutral input for one authentic Encyclopedia frame.
#[derive(Debug, PartialEq, Eq)]
pub struct EncyclopediaSurface<'a> {
    pub texture_generation: u64,
    pub title: &'a str,
    pub topic_label: &'a str,
    pub audience: EncyclopediaSurfaceAudience,
    pub mode: EncyclopediaSurfaceMode,
    pub category: EncyclopediaSurfaceCategory<'a>,
    pub categories: Vec<EncyclopediaSurfaceCategory<'a>>,
    pub topics: Vec<EncyclopediaSurfaceTopicItem<'a>>,
    pub active_topic: Option<EncyclopediaSurfaceTopic<'a>>,
    pub navigation: EncyclopediaSurfaceNavigation,
}

/// Logical keys consumed by the original viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaSurfaceKey {
    Left,
    Right,
    Up,
    Down,
    PageUp,
    PageDown,
    Home,
    End,
    Enter,
    Escape,
    Tab,
}

/// Intent emitted by the renderer for the application-owned presenter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaSurfaceAction {
    SelectCategory(u16),
    SelectTopic(u32),
    ClearTopicSelection,
    OpenTopic(u32),
    ShowIndex,
    Close,
}

/// Resolve the mode-specific action for keys shared by index and topic mode.
///
/// Index category traversal wraps right from the final category to the first;
/// left at the first category reselects it. Topic navigation never wraps.
#[must_use]
pub fn encyclopedia_keyboard_action(
    mode: EncyclopediaSurfaceMode,
    key: EncyclopediaSurfaceKey,
    category_commands: &[u16],
    selected_category: u16,
    selected_object_id: Option<u32>,
    previous_object_id: Option<u32>,
    next_object_id: Option<u32>,
) -> Option<EncyclopediaSurfaceAction> {
    match (mode, key) {
        (_, EncyclopediaSurfaceKey::Escape) => Some(EncyclopediaSurfaceAction::Close),
        (EncyclopediaSurfaceMode::Index, EncyclopediaSurfaceKey::Enter) => {
            selected_object_id.map(EncyclopediaSurfaceAction::OpenTopic)
        }
        (EncyclopediaSurfaceMode::Index, EncyclopediaSurfaceKey::Left) => {
            let position = category_commands
                .iter()
                .position(|command| *command == selected_category)?;
            let next = position
                .checked_sub(1)
                .and_then(|index| category_commands.get(index))
                .or_else(|| category_commands.first())?;
            Some(EncyclopediaSurfaceAction::SelectCategory(*next))
        }
        (EncyclopediaSurfaceMode::Index, EncyclopediaSurfaceKey::Right) => {
            let position = category_commands
                .iter()
                .position(|command| *command == selected_category)?;
            let next = category_commands
                .get(position + 1)
                .or_else(|| category_commands.first())?;
            Some(EncyclopediaSurfaceAction::SelectCategory(*next))
        }
        (EncyclopediaSurfaceMode::Topic, EncyclopediaSurfaceKey::Left) => {
            previous_object_id.map(EncyclopediaSurfaceAction::OpenTopic)
        }
        (EncyclopediaSurfaceMode::Topic, EncyclopediaSurfaceKey::Right) => {
            next_object_id.map(EncyclopediaSurfaceAction::OpenTopic)
        }
        (
            _,
            EncyclopediaSurfaceKey::Tab
            | EncyclopediaSurfaceKey::Up
            | EncyclopediaSurfaceKey::Down
            | EncyclopediaSurfaceKey::PageUp
            | EncyclopediaSurfaceKey::PageDown
            | EncyclopediaSurfaceKey::Home
            | EncyclopediaSurfaceKey::End,
        )
        | (EncyclopediaSurfaceMode::Topic, EncyclopediaSurfaceKey::Enter) => None,
    }
}

/// Resolve native bounded index-list movement through the shared
/// `CoolDragList` state. The recovered 160-by-20 aperture pages by eight
/// complete rows.
#[must_use]
pub fn encyclopedia_index_list_action(
    key: EncyclopediaSurfaceKey,
    topic_ids: &[u32],
    selected_object_id: Option<u32>,
) -> Option<EncyclopediaSurfaceAction> {
    let key = match key {
        EncyclopediaSurfaceKey::Up => ListKey::Up,
        EncyclopediaSurfaceKey::Down => ListKey::Down,
        EncyclopediaSurfaceKey::PageUp => ListKey::PageUp,
        EncyclopediaSurfaceKey::PageDown => ListKey::PageDown,
        EncyclopediaSurfaceKey::Home => ListKey::Home,
        EncyclopediaSurfaceKey::End => ListKey::End,
        _ => return None,
    };
    let mut list = DragListState::default();
    list.select_only(selected_object_id);
    let _ = list.update(
        ENCYCLOPEDIA_INDEX_LIST,
        topic_ids,
        DragListInput {
            key: Some(key),
            ..DragListInput::default()
        },
    );
    list.first_selected(topic_ids)
        .map(EncyclopediaSurfaceAction::SelectTopic)
}

/// Reproduce the original index edit control's ordered title lookup.
///
/// The viewer compares titles case-insensitively and retains the first entry
/// with the longest common prefix. It deliberately does not require the full
/// query to match, so a typo after a useful prefix (for example `tallon`)
/// still resolves `Talon Karrde`. A non-empty query with no shared prefix
/// retains the first ordered entry, matching `FUN_00609650`.
#[must_use]
pub fn encyclopedia_title_search_match(
    query: &str,
    topics: &[EncyclopediaSurfaceTopicItem<'_>],
) -> Option<u32> {
    if query.is_empty() {
        return None;
    }

    crate::list_control::longest_prefix_match(topics.iter().map(|topic| topic.title), query)
        .map(|matched| topics[matched.index].object_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn topic(object_id: u32, title: &'static str) -> EncyclopediaSurfaceTopicItem<'static> {
        EncyclopediaSurfaceTopicItem {
            object_id,
            title,
            availability: EncyclopediaSurfaceAvailability::Resolved,
        }
    }

    #[test]
    fn title_search_keeps_the_longest_case_insensitive_prefix_after_a_typo() {
        let topics = [
            topic(11, "TIE Defender"),
            topic(12, "Talon Karrde"),
            topic(13, "Tatooine"),
        ];

        assert_eq!(encyclopedia_title_search_match("tallon", &topics), Some(12));
        assert_eq!(encyclopedia_title_search_match("TALON", &topics), Some(12));
    }

    #[test]
    fn title_search_retains_the_first_ordered_match_on_equal_prefixes() {
        let topics = [topic(11, "TIE Bomber"), topic(12, "TIE Defender")];

        assert_eq!(
            encyclopedia_title_search_match("tie fighter", &topics),
            Some(11)
        );
        assert_eq!(encyclopedia_title_search_match("x", &topics), Some(11));
    }

    #[test]
    fn title_search_clears_selection_for_empty_input() {
        let topics = [topic(11, "A-wing")];

        assert_eq!(encyclopedia_title_search_match("", &topics), None);
    }

    #[test]
    fn down_from_an_unselected_index_selects_the_first_topic() {
        let topics = [11, 12, 13];

        assert_eq!(
            encyclopedia_index_list_action(EncyclopediaSurfaceKey::Down, &topics, None),
            Some(EncyclopediaSurfaceAction::SelectTopic(11))
        );
    }

    #[test]
    fn index_page_keys_use_the_recovered_eight_row_aperture() {
        let topics = [11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21];

        assert_eq!(
            encyclopedia_index_list_action(EncyclopediaSurfaceKey::PageDown, &topics, Some(11),),
            Some(EncyclopediaSurfaceAction::SelectTopic(19))
        );
        assert_eq!(
            encyclopedia_index_list_action(EncyclopediaSurfaceKey::PageUp, &topics, Some(21)),
            Some(EncyclopediaSurfaceAction::SelectTopic(13))
        );
    }
}
