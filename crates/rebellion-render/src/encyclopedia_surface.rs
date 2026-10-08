//! Renderer-owned Galactic Encyclopedia surface data and interactions.
//!
//! These borrowed DTOs deliberately contain no `rebellion-data` types. The
//! application adapts its validated presentation into this model, preserving
//! the renderer's one-way dependency boundary.

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
        (_, EncyclopediaSurfaceKey::Tab) => None,
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
        (EncyclopediaSurfaceMode::Topic, EncyclopediaSurfaceKey::Enter) => None,
        (
            _,
            EncyclopediaSurfaceKey::Up
            | EncyclopediaSurfaceKey::Down
            | EncyclopediaSurfaceKey::PageUp
            | EncyclopediaSurfaceKey::PageDown
            | EncyclopediaSurfaceKey::Home
            | EncyclopediaSurfaceKey::End,
        ) => None,
    }
}

/// Resolve native bounded index-list movement. Page keys move by the nine
/// rows visible in the recovered list aperture.
#[must_use]
pub fn encyclopedia_index_list_action(
    key: EncyclopediaSurfaceKey,
    topic_ids: &[u32],
    selected_object_id: Option<u32>,
) -> Option<EncyclopediaSurfaceAction> {
    let last = topic_ids.len().checked_sub(1)?;
    let current =
        selected_object_id.and_then(|selected| topic_ids.iter().position(|id| *id == selected));
    let current_or_first = current.unwrap_or(0);
    let next = match key {
        EncyclopediaSurfaceKey::Up => current_or_first.saturating_sub(1),
        EncyclopediaSurfaceKey::Down => {
            current.map_or(0, |index| index.saturating_add(1).min(last))
        }
        EncyclopediaSurfaceKey::PageUp => current_or_first.saturating_sub(9),
        EncyclopediaSurfaceKey::PageDown => current_or_first.saturating_add(9).min(last),
        EncyclopediaSurfaceKey::Home => 0,
        EncyclopediaSurfaceKey::End => last,
        _ => return None,
    };
    Some(EncyclopediaSurfaceAction::SelectTopic(topic_ids[next]))
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

    topics
        .iter()
        .map(|topic| {
            let prefix = topic
                .title
                .chars()
                .zip(query.chars())
                .take_while(|(title, query)| title.eq_ignore_ascii_case(query))
                .count();
            (topic.object_id, prefix)
        })
        .reduce(|best, candidate| {
            if candidate.1 > best.1 {
                candidate
            } else {
                best
            }
        })
        .map(|(object_id, _)| object_id)
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
}
