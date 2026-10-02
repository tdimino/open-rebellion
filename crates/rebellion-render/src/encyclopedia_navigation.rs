//! Pure stable-ID state transitions for encyclopedia index and topic views.
//!
//! The presenter has already applied world admission, whole-record language
//! fallback, and effective-title sorting. This module consumes that immutable
//! view and never reconstructs membership or gameplay predicates.

use crate::encyclopedia_view::{EncyclopediaSelection, EncyclopediaView};

/// Source viewer mode. Index rows may retain a current stable topic without
/// displaying its body; topic mode displays that current topic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaMode {
    Index,
    Topic,
}

/// Whether a category command came through the source's forced transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionForce {
    Normal,
    Forced,
}

/// A renderer-facing body-scroll request. Pixel and font metrics remain owned
/// by the drawing surface; the controller retains only source-defined intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyScrollIntent {
    LineUp,
    LineDown,
    PageUp,
    PageDown,
    ResetToTop,
}

/// Source-defined category traversal while the index list owns focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexCategoryDirection {
    Previous,
    Next,
}

/// Source-defined row traversal while the index list owns focus. Page sizes
/// are supplied by the drawing adapter from the actual list/row geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexRowNavigation {
    LineUp,
    LineDown,
    PageUp { visible_rows: usize },
    PageDown { visible_rows: usize },
    First,
    Last,
}

/// One source keyboard command retained until the controller applies it.
///
/// Unlike a resolved navigation action, this intent is interpreted from the
/// controller's current mode at application time. That preserves the source's
/// ordered behavior when an earlier key in the same finite input batch changes
/// mode, and also when that earlier transition is rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKeyIntent {
    Escape,
    Enter,
    Left,
    Right,
    Up,
    Down,
    PageUp { visible_rows: usize },
    PageDown { visible_rows: usize },
    Home,
    End,
}

/// Stable-ID commands emitted by either the native or browser input adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncyclopediaAction {
    SelectCategory {
        category_id: Option<String>,
        force: SelectionForce,
    },
    SelectTopic(String),
    SetMode(EncyclopediaMode),
    NavigateIndexCategory(IndexCategoryDirection),
    NavigateIndexRow(IndexRowNavigation),
    PreviousTopic,
    NextTopic,
    Scroll(BodyScrollIntent),
    Return,
    Close,
    SourceKey(SourceKeyIntent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationRejection {
    UnknownCategory,
    DisabledCategory,
    UnavailableIndex,
    UnavailableTopic,
    NoCurrentTopic,
    WrongMode,
    InvalidPageSize,
}

/// Result for one action. Close and Return are intentions only; caller focus
/// and campaign lifecycle remain application-owned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationOutcome {
    Applied,
    NoChange,
    Rejected(NavigationRejection),
    ScrollRequested(BodyScrollIntent),
    CloseRequested,
    ReturnForwarded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReconcileOutcome {
    Unchanged,
    SelectionAdjusted,
}

/// Mutable graphics-free navigation state.
///
/// `selection.category_id` is the category whose topic projection the
/// presenter should build. `selected_category_id` is deliberately separate:
/// E27 proves that a forced family command selects that control while binding
/// the full master collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaState {
    pub selection: EncyclopediaSelection,
    selected_category_id: Option<String>,
    mode: EncyclopediaMode,
    pending_body_scroll: Vec<BodyScrollIntent>,
}

impl EncyclopediaState {
    #[must_use]
    pub fn new(selection: EncyclopediaSelection, mode: EncyclopediaMode) -> Self {
        let mode = if mode == EncyclopediaMode::Topic && selection.topic_id.is_none() {
            EncyclopediaMode::Index
        } else {
            mode
        };
        Self {
            selected_category_id: selection.category_id.clone(),
            selection,
            mode,
            pending_body_scroll: Vec::new(),
        }
    }

    #[must_use]
    pub fn selected_category_id(&self) -> Option<&str> {
        self.selected_category_id.as_deref()
    }

    #[must_use]
    pub fn mode(&self) -> EncyclopediaMode {
        self.mode
    }

    /// Ordered body-scroll commands waiting for the drawing adapter.
    ///
    /// An input adapter may apply a finite batch of actions before drawing. It
    /// must then call [`Self::take_body_scroll_intents`] once for that batch and
    /// apply the returned commands in order. This makes reset-plus-scroll and
    /// repeated-scroll batches lossless without retaining commands across
    /// rendered frames.
    #[must_use]
    pub fn pending_body_scroll_intents(&self) -> &[BodyScrollIntent] {
        &self.pending_body_scroll
    }

    /// Transfer ownership of the pending finite action batch to the caller.
    /// A second call returns an empty vector until another action queues work.
    #[must_use = "body-scroll commands must be applied in order"]
    pub fn take_body_scroll_intents(&mut self) -> Vec<BodyScrollIntent> {
        std::mem::take(&mut self.pending_body_scroll)
    }

    fn request_body_scroll(&mut self, intent: BodyScrollIntent) {
        self.pending_body_scroll.push(intent);
    }
}

impl Default for EncyclopediaState {
    fn default() -> Self {
        Self::new(EncyclopediaSelection::default(), EncyclopediaMode::Index)
    }
}

/// Reconcile retained stable IDs with a newly published immutable view.
///
/// A surviving canonical ID is retained even when the row moves. An absent ID
/// is cleared rather than replaced with a guessed row; this also prevents a
/// reused numeric DAT component in another family from preserving selection.
pub fn reconcile_encyclopedia_state(
    state: &mut EncyclopediaState,
    view: &EncyclopediaView,
) -> ReconcileOutcome {
    let before = state.clone();

    if state
        .selection
        .category_id
        .as_deref()
        .is_some_and(|category_id| !enabled_category_exists(view, category_id))
    {
        state.selection.category_id = None;
    }

    if state
        .selected_category_id
        .as_deref()
        .is_some_and(|category_id| !enabled_category_exists(view, category_id))
    {
        state.selected_category_id = state.selection.category_id.clone();
    } else if state.selected_category_id.is_none() && state.selection.category_id.is_some() {
        state.selected_category_id = state.selection.category_id.clone();
    }

    if state
        .selection
        .topic_id
        .as_deref()
        .is_some_and(|topic_id| !topic_exists(view, topic_id))
    {
        state.selection.topic_id = None;
        state.mode = EncyclopediaMode::Index;
        state.request_body_scroll(BodyScrollIntent::ResetToTop);
    } else if state.mode == EncyclopediaMode::Topic && state.selection.topic_id.is_none() {
        state.mode = EncyclopediaMode::Index;
    }

    if *state == before {
        ReconcileOutcome::Unchanged
    } else {
        ReconcileOutcome::SelectionAdjusted
    }
}

/// Apply one action against the current view. Targets and retained IDs are
/// checked at application time so an event queued before a replacement cannot
/// select a hidden or deleted row.
pub fn apply_encyclopedia_action(
    state: &mut EncyclopediaState,
    view: &EncyclopediaView,
    action: EncyclopediaAction,
) -> NavigationOutcome {
    let _ = reconcile_encyclopedia_state(state, view);

    let action = match action {
        EncyclopediaAction::SourceKey(intent) => {
            let Some(action) = action_for_source_key(state.mode, intent) else {
                return NavigationOutcome::NoChange;
            };
            action
        }
        action => action,
    };

    match action {
        EncyclopediaAction::SelectCategory { category_id, force } => {
            apply_category_selection(state, view, category_id, force)
        }
        EncyclopediaAction::SelectTopic(topic_id) => {
            if !topic_exists(view, &topic_id) {
                return NavigationOutcome::Rejected(NavigationRejection::UnavailableTopic);
            }
            if state.selection.topic_id.as_deref() == Some(topic_id.as_str()) {
                return NavigationOutcome::NoChange;
            }
            state.selection.topic_id = Some(topic_id);
            if state.mode == EncyclopediaMode::Topic {
                state.request_body_scroll(BodyScrollIntent::ResetToTop);
            }
            NavigationOutcome::Applied
        }
        EncyclopediaAction::SetMode(mode) => apply_mode(state, view, mode),
        EncyclopediaAction::NavigateIndexCategory(direction) => {
            navigate_index_category(state, view, direction)
        }
        EncyclopediaAction::NavigateIndexRow(movement) => navigate_index_row(state, view, movement),
        EncyclopediaAction::PreviousTopic => move_topic(state, view, Direction::Previous),
        EncyclopediaAction::NextTopic => move_topic(state, view, Direction::Next),
        EncyclopediaAction::Scroll(intent) => {
            if state.selection.topic_id.is_none() {
                return NavigationOutcome::Rejected(NavigationRejection::NoCurrentTopic);
            }
            if state.mode != EncyclopediaMode::Topic {
                return NavigationOutcome::Rejected(NavigationRejection::WrongMode);
            }
            state.request_body_scroll(intent);
            NavigationOutcome::ScrollRequested(intent)
        }
        EncyclopediaAction::Return => {
            if state.mode == EncyclopediaMode::Topic {
                NavigationOutcome::ReturnForwarded
            } else {
                apply_mode(state, view, EncyclopediaMode::Topic)
            }
        }
        EncyclopediaAction::Close => NavigationOutcome::CloseRequested,
        EncyclopediaAction::SourceKey(_) => unreachable!("source keys are resolved above"),
    }
}

fn action_for_source_key(
    mode: EncyclopediaMode,
    intent: SourceKeyIntent,
) -> Option<EncyclopediaAction> {
    match (mode, intent) {
        (_, SourceKeyIntent::Escape) => Some(EncyclopediaAction::Close),
        (EncyclopediaMode::Index, SourceKeyIntent::Enter) => {
            Some(EncyclopediaAction::SetMode(EncyclopediaMode::Topic))
        }
        (EncyclopediaMode::Index, SourceKeyIntent::Left) => Some(
            EncyclopediaAction::NavigateIndexCategory(IndexCategoryDirection::Previous),
        ),
        (EncyclopediaMode::Index, SourceKeyIntent::Right) => Some(
            EncyclopediaAction::NavigateIndexCategory(IndexCategoryDirection::Next),
        ),
        (EncyclopediaMode::Index, SourceKeyIntent::Up) => Some(
            EncyclopediaAction::NavigateIndexRow(IndexRowNavigation::LineUp),
        ),
        (EncyclopediaMode::Index, SourceKeyIntent::Down) => Some(
            EncyclopediaAction::NavigateIndexRow(IndexRowNavigation::LineDown),
        ),
        (EncyclopediaMode::Index, SourceKeyIntent::PageUp { visible_rows }) => Some(
            EncyclopediaAction::NavigateIndexRow(IndexRowNavigation::PageUp { visible_rows }),
        ),
        (EncyclopediaMode::Index, SourceKeyIntent::PageDown { visible_rows }) => Some(
            EncyclopediaAction::NavigateIndexRow(IndexRowNavigation::PageDown { visible_rows }),
        ),
        (EncyclopediaMode::Index, SourceKeyIntent::Home) => Some(
            EncyclopediaAction::NavigateIndexRow(IndexRowNavigation::First),
        ),
        (EncyclopediaMode::Index, SourceKeyIntent::End) => Some(
            EncyclopediaAction::NavigateIndexRow(IndexRowNavigation::Last),
        ),
        (EncyclopediaMode::Topic, SourceKeyIntent::Left) => Some(EncyclopediaAction::PreviousTopic),
        (EncyclopediaMode::Topic, SourceKeyIntent::Right) => Some(EncyclopediaAction::NextTopic),
        (EncyclopediaMode::Topic, SourceKeyIntent::Up) => {
            Some(EncyclopediaAction::Scroll(BodyScrollIntent::LineUp))
        }
        (EncyclopediaMode::Topic, SourceKeyIntent::Down) => {
            Some(EncyclopediaAction::Scroll(BodyScrollIntent::LineDown))
        }
        (EncyclopediaMode::Topic, SourceKeyIntent::PageUp { .. }) => {
            Some(EncyclopediaAction::Scroll(BodyScrollIntent::PageUp))
        }
        (EncyclopediaMode::Topic, SourceKeyIntent::PageDown { .. }) => {
            Some(EncyclopediaAction::Scroll(BodyScrollIntent::PageDown))
        }
        (EncyclopediaMode::Topic, SourceKeyIntent::Enter) => Some(EncyclopediaAction::Return),
        (EncyclopediaMode::Topic, SourceKeyIntent::Home | SourceKeyIntent::End) => None,
    }
}

fn navigate_index_category(
    state: &mut EncyclopediaState,
    view: &EncyclopediaView,
    direction: IndexCategoryDirection,
) -> NavigationOutcome {
    if state.mode != EncyclopediaMode::Index {
        return NavigationOutcome::Rejected(NavigationRejection::WrongMode);
    }

    // Slot zero is the aggregate index command; remaining slots retain the
    // presenter's source registry order, including disabled/hidden entries.
    let current = match state.selected_category_id.as_deref() {
        None => 0,
        Some(selected) => view
            .categories
            .iter()
            .position(|category| category.category_id == selected)
            .map_or(0, |index| index + 1),
    };
    let first_lookup = match direction {
        IndexCategoryDirection::Previous => current.checked_sub(1),
        IndexCategoryDirection::Next => current
            .checked_add(1)
            .filter(|index| *index <= view.categories.len()),
    };

    // FUN_0045fe60 falls back to the tree's leftmost child only when the
    // initial lookup is null. That fallback deliberately precedes visibility
    // filtering, so apply_category_selection remains the fail-closed gate.
    let Some(mut candidate) = first_lookup else {
        return apply_category_selection(state, view, None, SelectionForce::Normal);
    };

    loop {
        let visible = if candidate == 0 {
            view.index_enabled
        } else {
            view.categories[candidate - 1].enabled
        };
        if visible {
            let category_id =
                (candidate != 0).then(|| view.categories[candidate - 1].category_id.clone());
            return apply_category_selection(state, view, category_id, SelectionForce::Normal);
        }

        candidate = match direction {
            IndexCategoryDirection::Previous => match candidate.checked_sub(1) {
                Some(index) => index,
                None => return NavigationOutcome::NoChange,
            },
            IndexCategoryDirection::Next => match candidate
                .checked_add(1)
                .filter(|index| *index <= view.categories.len())
            {
                Some(index) => index,
                None => return NavigationOutcome::NoChange,
            },
        };
    }
}

fn navigate_index_row(
    state: &mut EncyclopediaState,
    view: &EncyclopediaView,
    movement: IndexRowNavigation,
) -> NavigationOutcome {
    if state.mode != EncyclopediaMode::Index {
        return NavigationOutcome::Rejected(NavigationRejection::WrongMode);
    }
    let Some(last_index) = view.topics.len().checked_sub(1) else {
        return NavigationOutcome::Rejected(NavigationRejection::UnavailableTopic);
    };
    if matches!(
        movement,
        IndexRowNavigation::PageUp { visible_rows: 0 }
            | IndexRowNavigation::PageDown { visible_rows: 0 }
    ) {
        return NavigationOutcome::Rejected(NavigationRejection::InvalidPageSize);
    }

    let target_index = match movement {
        IndexRowNavigation::First => 0,
        IndexRowNavigation::Last => last_index,
        IndexRowNavigation::LineUp
        | IndexRowNavigation::LineDown
        | IndexRowNavigation::PageUp { .. }
        | IndexRowNavigation::PageDown { .. } => {
            let Some(current) = state.selection.topic_id.as_deref() else {
                return NavigationOutcome::Rejected(NavigationRejection::NoCurrentTopic);
            };
            let Some(index) = view
                .topics
                .iter()
                .position(|topic| topic.topic_id == current)
            else {
                return NavigationOutcome::Rejected(NavigationRejection::UnavailableTopic);
            };
            match movement {
                IndexRowNavigation::LineUp => index.saturating_sub(1),
                IndexRowNavigation::LineDown => index.saturating_add(1).min(last_index),
                IndexRowNavigation::PageUp { visible_rows } => index.saturating_sub(visible_rows),
                IndexRowNavigation::PageDown { visible_rows } => {
                    index.saturating_add(visible_rows).min(last_index)
                }
                IndexRowNavigation::First | IndexRowNavigation::Last => unreachable!(),
            }
        }
    };
    let target = &view.topics[target_index].topic_id;
    if state.selection.topic_id.as_deref() == Some(target.as_str()) {
        return NavigationOutcome::NoChange;
    }
    state.selection.topic_id = Some(target.clone());
    NavigationOutcome::Applied
}

fn apply_category_selection(
    state: &mut EncyclopediaState,
    view: &EncyclopediaView,
    category_id: Option<String>,
    force: SelectionForce,
) -> NavigationOutcome {
    let target_command = match category_id.as_deref() {
        None if view.index_enabled => None,
        None => return NavigationOutcome::Rejected(NavigationRejection::UnavailableIndex),
        Some(category_id) => {
            let Some(category) = view
                .categories
                .iter()
                .find(|category| category.category_id == category_id)
            else {
                return NavigationOutcome::Rejected(NavigationRejection::UnknownCategory);
            };
            if !category.enabled {
                return NavigationOutcome::Rejected(NavigationRejection::DisabledCategory);
            }
            Some(category.command.as_str())
        }
    };

    let current_command = state
        .selected_category_id
        .as_deref()
        .and_then(|selected| {
            view.categories
                .iter()
                .find(|category| category.category_id == selected && category.enabled)
        })
        .map(|category| category.command.as_str());
    let same_command = match (
        state.selected_category_id.as_deref(),
        category_id.as_deref(),
    ) {
        (None, None) => true,
        (Some(_), Some(_)) => current_command == target_command,
        _ => false,
    };

    // FUN_0045f100 checks equality before either force or mode.
    if same_command {
        return NavigationOutcome::NoChange;
    }
    if state.mode == EncyclopediaMode::Topic && force == SelectionForce::Normal {
        return NavigationOutcome::NoChange;
    }

    state.selected_category_id = category_id.clone();
    if category_id.is_none() || force == SelectionForce::Forced {
        state.selection.category_id = None;
    } else {
        state.selection.category_id = category_id;
    }
    if force == SelectionForce::Forced && state.mode != EncyclopediaMode::Topic {
        state.selection.topic_id = None;
    }
    NavigationOutcome::Applied
}

fn apply_mode(
    state: &mut EncyclopediaState,
    view: &EncyclopediaView,
    mode: EncyclopediaMode,
) -> NavigationOutcome {
    if state.mode == mode {
        return NavigationOutcome::NoChange;
    }
    if mode == EncyclopediaMode::Topic {
        let Some(topic_id) = state.selection.topic_id.as_deref() else {
            return NavigationOutcome::Rejected(NavigationRejection::NoCurrentTopic);
        };
        if !topic_exists(view, topic_id) {
            return NavigationOutcome::Rejected(NavigationRejection::UnavailableTopic);
        }
        state.request_body_scroll(BodyScrollIntent::ResetToTop);
    }
    state.mode = mode;
    NavigationOutcome::Applied
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Previous,
    Next,
}

fn move_topic(
    state: &mut EncyclopediaState,
    view: &EncyclopediaView,
    direction: Direction,
) -> NavigationOutcome {
    let Some(current) = state.selection.topic_id.as_deref() else {
        return NavigationOutcome::Rejected(NavigationRejection::NoCurrentTopic);
    };
    if state.mode != EncyclopediaMode::Topic {
        return NavigationOutcome::Rejected(NavigationRejection::WrongMode);
    }
    let Some(index) = view
        .topics
        .iter()
        .position(|topic| topic.topic_id == current)
    else {
        return NavigationOutcome::Rejected(NavigationRejection::UnavailableTopic);
    };
    let neighbor = match direction {
        Direction::Previous => index
            .checked_sub(1)
            .and_then(|index| view.topics.get(index)),
        Direction::Next => index
            .checked_add(1)
            .and_then(|index| view.topics.get(index)),
    };
    let Some(neighbor) = neighbor else {
        return NavigationOutcome::NoChange;
    };

    state.selection.topic_id = Some(neighbor.topic_id.clone());
    state.request_body_scroll(BodyScrollIntent::ResetToTop);
    NavigationOutcome::Applied
}

fn enabled_category_exists(view: &EncyclopediaView, category_id: &str) -> bool {
    view.categories
        .iter()
        .any(|category| category.category_id == category_id && category.enabled)
}

fn topic_exists(view: &EncyclopediaView, topic_id: &str) -> bool {
    view.topics.iter().any(|topic| topic.topic_id == topic_id)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::encyclopedia_view::{
        ActiveTopicView, CategoryViewItem, EncyclopediaSelection, EncyclopediaView,
        NavigationState, TopicImageRenderProfile, TopicImageView, TopicViewItem,
    };

    use super::{
        apply_encyclopedia_action, reconcile_encyclopedia_state, BodyScrollIntent,
        EncyclopediaAction, EncyclopediaMode, EncyclopediaState, IndexCategoryDirection,
        IndexRowNavigation, NavigationOutcome, NavigationRejection, ReconcileOutcome,
        SelectionForce, SourceKeyIntent,
    };

    fn category(id: &str, command: &str, enabled: bool) -> CategoryViewItem {
        CategoryViewItem {
            category_id: id.to_owned(),
            command: command.to_owned(),
            label: enabled.then(|| Arc::from(id)),
            enabled,
        }
    }

    fn view(
        topic_ids: &[&str],
        selected_category_id: Option<&str>,
        selected_topic_id: Option<&str>,
        world_epoch: u64,
    ) -> EncyclopediaView {
        let selected_position = selected_topic_id
            .and_then(|selected| topic_ids.iter().position(|topic| *topic == selected));
        EncyclopediaView {
            index_label: Some(Arc::from("All")),
            index_enabled: true,
            categories: vec![
                category("ships", "0x70", true),
                category("forces", "0x71", true),
                category("hidden", "0x72", false),
            ],
            topics: topic_ids
                .iter()
                .map(|topic_id| TopicViewItem {
                    topic_id: (*topic_id).to_owned(),
                    title: Arc::from(*topic_id),
                })
                .collect(),
            active_topic: selected_position.map(|index| ActiveTopicView {
                topic_id: topic_ids[index].to_owned(),
                title: Arc::from(topic_ids[index]),
                body: Arc::from("body"),
                image: None,
                stats: Vec::new(),
            }),
            navigation: NavigationState {
                selected_category_id: selected_category_id.map(str::to_owned),
                selected_topic_id: selected_position.map(|index| topic_ids[index].to_owned()),
                previous_topic_id: selected_position
                    .and_then(|index| index.checked_sub(1))
                    .map(|index| topic_ids[index].to_owned()),
                next_topic_id: selected_position
                    .and_then(|index| index.checked_add(1))
                    .and_then(|index| topic_ids.get(index))
                    .map(|topic| (*topic).to_owned()),
                world_epoch,
            },
            diagnostics: Vec::new(),
        }
    }

    fn state(
        category_id: Option<&str>,
        topic_id: Option<&str>,
        mode: EncyclopediaMode,
    ) -> EncyclopediaState {
        EncyclopediaState::new(
            EncyclopediaSelection {
                category_id: category_id.map(str::to_owned),
                topic_id: topic_id.map(str::to_owned),
            },
            mode,
        )
    }

    // Source: encyclopedia-ui-contract.md lines 216-235. The first null
    // predecessor/successor falls back to the leftmost category, while a null
    // reached only after skipping hidden children retains the current category.
    #[test]
    fn index_category_navigation_preserves_source_asymmetric_edges_and_hidden_skip() {
        let mut current = view(&["alpha"], Some("ships"), None, 1);
        current.categories = vec![
            category("ships", "0x70", true),
            category("hidden", "0x71", false),
            category("forces", "0x72", true),
        ];
        let mut category_state = state(Some("ships"), None, EncyclopediaMode::Index);

        assert_eq!(
            apply_encyclopedia_action(
                &mut category_state,
                &current,
                EncyclopediaAction::NavigateIndexCategory(IndexCategoryDirection::Previous),
            ),
            NavigationOutcome::Applied
        );
        assert_eq!(category_state.selected_category_id(), None);
        assert_eq!(
            apply_encyclopedia_action(
                &mut category_state,
                &current,
                EncyclopediaAction::NavigateIndexCategory(IndexCategoryDirection::Previous),
            ),
            NavigationOutcome::NoChange
        );
        assert_eq!(
            apply_encyclopedia_action(
                &mut category_state,
                &current,
                EncyclopediaAction::NavigateIndexCategory(IndexCategoryDirection::Next),
            ),
            NavigationOutcome::Applied
        );
        assert_eq!(category_state.selected_category_id(), Some("ships"));

        assert_eq!(
            apply_encyclopedia_action(
                &mut category_state,
                &current,
                EncyclopediaAction::NavigateIndexCategory(IndexCategoryDirection::Next),
            ),
            NavigationOutcome::Applied
        );
        assert_eq!(category_state.selected_category_id(), Some("forces"));
        assert_eq!(
            apply_encyclopedia_action(
                &mut category_state,
                &current,
                EncyclopediaAction::NavigateIndexCategory(IndexCategoryDirection::Next),
            ),
            NavigationOutcome::Applied
        );
        assert_eq!(category_state.selected_category_id(), None);

        current.categories = vec![
            category("ships", "0x70", true),
            category("forces", "0x71", true),
            category("hidden", "0x72", false),
        ];
        let mut state = state(Some("forces"), None, EncyclopediaMode::Index);
        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &current,
                EncyclopediaAction::NavigateIndexCategory(IndexCategoryDirection::Next),
            ),
            NavigationOutcome::NoChange
        );
        assert_eq!(state.selected_category_id(), Some("forces"));
    }

    #[test]
    fn source_key_intents_follow_each_successful_or_rejected_mode_transition() {
        let current = view(&["alpha", "beta"], None, Some("alpha"), 1);
        let mut topic_transition = state(None, Some("alpha"), EncyclopediaMode::Index);

        assert_eq!(
            apply_encyclopedia_action(
                &mut topic_transition,
                &current,
                EncyclopediaAction::SourceKey(SourceKeyIntent::Enter),
            ),
            NavigationOutcome::Applied
        );
        assert_eq!(topic_transition.mode(), EncyclopediaMode::Topic);
        assert_eq!(
            apply_encyclopedia_action(
                &mut topic_transition,
                &current,
                EncyclopediaAction::SourceKey(SourceKeyIntent::Right),
            ),
            NavigationOutcome::Applied
        );
        assert_eq!(topic_transition.selection.topic_id.as_deref(), Some("beta"));

        let no_selection = view(&["alpha", "beta"], None, None, 1);
        let mut rejected_transition = state(None, None, EncyclopediaMode::Index);
        assert_eq!(
            apply_encyclopedia_action(
                &mut rejected_transition,
                &no_selection,
                EncyclopediaAction::SourceKey(SourceKeyIntent::Enter),
            ),
            NavigationOutcome::Rejected(NavigationRejection::NoCurrentTopic)
        );
        assert_eq!(rejected_transition.mode(), EncyclopediaMode::Index);
        assert_eq!(
            apply_encyclopedia_action(
                &mut rejected_transition,
                &no_selection,
                EncyclopediaAction::SourceKey(SourceKeyIntent::Right),
            ),
            NavigationOutcome::Applied
        );
        assert_eq!(rejected_transition.selected_category_id(), Some("ships"));
    }

    // Source: encyclopedia-ui-contract.md lines 237-239. Index rows move by
    // one or the caller-supplied visible-row count, clamp at endpoints, and do
    // not wrap.
    #[test]
    fn index_row_navigation_moves_lines_pages_and_endpoints_without_wrapping() {
        let current = view(
            &["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"],
            None,
            Some("c"),
            1,
        );
        let mut state = state(None, Some("c"), EncyclopediaMode::Index);

        for (movement, expected) in [
            (IndexRowNavigation::LineUp, "b"),
            (IndexRowNavigation::LineDown, "c"),
            (IndexRowNavigation::PageDown { visible_rows: 8 }, "j"),
            (IndexRowNavigation::PageUp { visible_rows: 8 }, "b"),
            (IndexRowNavigation::First, "a"),
            (IndexRowNavigation::Last, "j"),
        ] {
            assert_eq!(
                apply_encyclopedia_action(
                    &mut state,
                    &current,
                    EncyclopediaAction::NavigateIndexRow(movement),
                ),
                NavigationOutcome::Applied
            );
            assert_eq!(state.selection.topic_id.as_deref(), Some(expected));
        }

        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &current,
                EncyclopediaAction::NavigateIndexRow(IndexRowNavigation::LineDown),
            ),
            NavigationOutcome::NoChange
        );
        assert_eq!(state.selection.topic_id.as_deref(), Some("j"));
        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &current,
                EncyclopediaAction::NavigateIndexRow(IndexRowNavigation::PageDown {
                    visible_rows: 0,
                }),
            ),
            NavigationOutcome::Rejected(NavigationRejection::InvalidPageSize)
        );
        assert_eq!(state.selection.topic_id.as_deref(), Some("j"));
    }

    // Source: encyclopedia-source-contract.md, decision scenario "First / middle / last topic";
    // FUN_004ad730/FUN_004ad750 skip disabled links and null endpoints do not wrap.
    #[test]
    fn previous_and_next_follow_the_current_sorted_rows_without_wrapping() {
        let view = view(&["alpha", "beta", "gamma"], None, Some("beta"), 1);
        let mut state = state(None, Some("beta"), EncyclopediaMode::Topic);

        assert_eq!(
            apply_encyclopedia_action(&mut state, &view, EncyclopediaAction::PreviousTopic),
            NavigationOutcome::Applied
        );
        assert_eq!(state.selection.topic_id.as_deref(), Some("alpha"));
        assert_eq!(
            state.take_body_scroll_intents(),
            vec![BodyScrollIntent::ResetToTop]
        );
        assert_eq!(
            apply_encyclopedia_action(&mut state, &view, EncyclopediaAction::PreviousTopic),
            NavigationOutcome::NoChange
        );
        assert_eq!(state.selection.topic_id.as_deref(), Some("alpha"));

        assert_eq!(
            apply_encyclopedia_action(&mut state, &view, EncyclopediaAction::NextTopic),
            NavigationOutcome::Applied
        );
        assert_eq!(state.selection.topic_id.as_deref(), Some("beta"));
        assert_eq!(
            apply_encyclopedia_action(&mut state, &view, EncyclopediaAction::NextTopic),
            NavigationOutcome::Applied
        );
        assert_eq!(state.selection.topic_id.as_deref(), Some("gamma"));
        let _ = state.take_body_scroll_intents();
        assert_eq!(
            apply_encyclopedia_action(&mut state, &view, EncyclopediaAction::NextTopic),
            NavigationOutcome::NoChange
        );
        assert!(state.take_body_scroll_intents().is_empty());
    }

    #[test]
    fn empty_and_unavailable_topic_lists_never_fabricate_a_selection() {
        let view = view(&[], None, None, 1);
        let mut state = EncyclopediaState::default();

        assert_eq!(
            apply_encyclopedia_action(&mut state, &view, EncyclopediaAction::NextTopic),
            NavigationOutcome::Rejected(NavigationRejection::NoCurrentTopic)
        );
        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &view,
                EncyclopediaAction::SelectTopic("missing".to_owned())
            ),
            NavigationOutcome::Rejected(NavigationRejection::UnavailableTopic)
        );
        assert_eq!(state.selection.topic_id, None);
    }

    #[test]
    fn a_topic_target_hidden_between_input_and_apply_is_rejected() {
        let replacement = view(&["still-visible"], None, Some("still-visible"), 2);
        let mut state = state(None, Some("still-visible"), EncyclopediaMode::Index);
        let before = state.clone();

        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &replacement,
                EncyclopediaAction::SelectTopic("now-hidden".to_owned())
            ),
            NavigationOutcome::Rejected(NavigationRejection::UnavailableTopic)
        );
        assert_eq!(state, before);
    }

    #[test]
    fn disabled_unknown_and_unavailable_index_categories_are_rejected() {
        let mut current = view(&["alpha"], None, None, 1);
        let mut state = EncyclopediaState::default();

        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &current,
                EncyclopediaAction::SelectCategory {
                    category_id: Some("hidden".to_owned()),
                    force: SelectionForce::Normal,
                }
            ),
            NavigationOutcome::Rejected(NavigationRejection::DisabledCategory)
        );
        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &current,
                EncyclopediaAction::SelectCategory {
                    category_id: Some("unknown".to_owned()),
                    force: SelectionForce::Normal,
                }
            ),
            NavigationOutcome::Rejected(NavigationRejection::UnknownCategory)
        );

        current.index_enabled = false;
        current.index_label = None;
        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &current,
                EncyclopediaAction::SelectCategory {
                    category_id: None,
                    force: SelectionForce::Normal,
                }
            ),
            NavigationOutcome::Rejected(NavigationRejection::UnavailableIndex)
        );
    }

    // Source: encyclopedia-source-contract.md, "Selection, routing, and typed application
    // boundary", steps 1-2. Same command returns before force; a changed normal command returns
    // in topic mode.
    #[test]
    fn same_category_returns_before_force_and_normal_topic_changes_are_ignored() {
        let view = view(&["alpha"], Some("ships"), Some("alpha"), 1);
        let mut index = state(Some("ships"), Some("alpha"), EncyclopediaMode::Index);

        assert_eq!(
            apply_encyclopedia_action(
                &mut index,
                &view,
                EncyclopediaAction::SelectCategory {
                    category_id: Some("ships".to_owned()),
                    force: SelectionForce::Forced,
                }
            ),
            NavigationOutcome::NoChange
        );
        assert_eq!(index.selection.category_id.as_deref(), Some("ships"));
        assert_eq!(index.selection.topic_id.as_deref(), Some("alpha"));

        let mut topic = state(Some("ships"), Some("alpha"), EncyclopediaMode::Topic);
        let before = topic.clone();
        assert_eq!(
            apply_encyclopedia_action(
                &mut topic,
                &view,
                EncyclopediaAction::SelectCategory {
                    category_id: Some("forces".to_owned()),
                    force: SelectionForce::Normal,
                }
            ),
            NavigationOutcome::NoChange
        );
        assert_eq!(topic, before);
    }

    // Source: encyclopedia-source-contract.md, selection steps 3-4. A forced change binds the
    // master collection; only forced non-topic mode clears the current row.
    #[test]
    fn forced_category_change_binds_master_and_clears_only_an_index_current_topic() {
        let view = view(&["alpha"], Some("ships"), Some("alpha"), 1);
        let action = EncyclopediaAction::SelectCategory {
            category_id: Some("forces".to_owned()),
            force: SelectionForce::Forced,
        };
        let mut index = state(Some("ships"), Some("alpha"), EncyclopediaMode::Index);
        let mut topic = state(Some("ships"), Some("alpha"), EncyclopediaMode::Topic);

        assert_eq!(
            apply_encyclopedia_action(&mut index, &view, action.clone()),
            NavigationOutcome::Applied
        );
        assert_eq!(index.selected_category_id(), Some("forces"));
        assert_eq!(index.selection.category_id, None);
        assert_eq!(index.selection.topic_id, None);

        assert_eq!(
            apply_encyclopedia_action(&mut topic, &view, action),
            NavigationOutcome::Applied
        );
        assert_eq!(topic.selected_category_id(), Some("forces"));
        assert_eq!(topic.selection.category_id, None);
        assert_eq!(topic.selection.topic_id.as_deref(), Some("alpha"));
    }

    #[test]
    fn aggregate_and_normal_family_changes_bind_the_expected_collection() {
        let view = view(&["alpha"], Some("ships"), Some("alpha"), 1);
        let mut state = state(Some("ships"), Some("alpha"), EncyclopediaMode::Index);

        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &view,
                EncyclopediaAction::SelectCategory {
                    category_id: Some("forces".to_owned()),
                    force: SelectionForce::Normal,
                }
            ),
            NavigationOutcome::Applied
        );
        assert_eq!(state.selected_category_id(), Some("forces"));
        assert_eq!(state.selection.category_id.as_deref(), Some("forces"));
        assert_eq!(state.selection.topic_id.as_deref(), Some("alpha"));

        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &view,
                EncyclopediaAction::SelectCategory {
                    category_id: None,
                    force: SelectionForce::Normal,
                }
            ),
            NavigationOutcome::Applied
        );
        assert_eq!(state.selected_category_id(), None);
        assert_eq!(state.selection.category_id, None);
        assert_eq!(state.selection.topic_id.as_deref(), Some("alpha"));
    }

    // Source: encyclopedia-ui-contract.md, "Mode, focus, keyboard, and scrolling". The body
    // consumes line/page scrolls; topic Return is forwarded and Escape/0xfb requests close.
    // FUN_0045fa60 -> FUN_0041fc30 resets the body scroll fields before recalculation.
    #[test]
    fn body_scroll_reset_and_close_return_are_typed_intents_without_local_side_effects() {
        let view = view(&["alpha", "beta"], None, Some("alpha"), 1);
        let mut state = state(None, Some("alpha"), EncyclopediaMode::Topic);

        for intent in [
            BodyScrollIntent::LineUp,
            BodyScrollIntent::LineDown,
            BodyScrollIntent::PageUp,
            BodyScrollIntent::PageDown,
        ] {
            assert_eq!(
                apply_encyclopedia_action(&mut state, &view, EncyclopediaAction::Scroll(intent)),
                NavigationOutcome::ScrollRequested(intent)
            );
            assert_eq!(state.pending_body_scroll_intents(), &[intent]);
            assert_eq!(state.take_body_scroll_intents(), vec![intent]);
        }

        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &view,
                EncyclopediaAction::SelectTopic("beta".to_owned())
            ),
            NavigationOutcome::Applied
        );
        assert_eq!(
            state.take_body_scroll_intents(),
            vec![BodyScrollIntent::ResetToTop]
        );

        let before = state.clone();
        assert_eq!(
            apply_encyclopedia_action(&mut state, &view, EncyclopediaAction::Return),
            NavigationOutcome::ReturnForwarded
        );
        assert_eq!(state, before);
        assert_eq!(
            apply_encyclopedia_action(&mut state, &view, EncyclopediaAction::Close),
            NavigationOutcome::CloseRequested
        );
        assert_eq!(state, before);
    }

    #[test]
    fn index_return_opens_only_a_current_enabled_topic_and_resets_its_body() {
        let populated = view(&["alpha"], None, Some("alpha"), 1);
        let mut state = state(None, Some("alpha"), EncyclopediaMode::Index);

        assert_eq!(
            apply_encyclopedia_action(&mut state, &populated, EncyclopediaAction::Return),
            NavigationOutcome::Applied
        );
        assert_eq!(state.mode(), EncyclopediaMode::Topic);
        assert_eq!(
            state.take_body_scroll_intents(),
            vec![BodyScrollIntent::ResetToTop]
        );

        let empty = view(&[], None, None, 1);
        let mut empty_state = EncyclopediaState::default();
        assert_eq!(
            apply_encyclopedia_action(&mut empty_state, &empty, EncyclopediaAction::Return),
            NavigationOutcome::Rejected(NavigationRejection::NoCurrentTopic)
        );
    }

    // Source: encyclopedia-ui-contract.md, "Index mode". A new single-click selects a row but
    // does not open it; Return dispatches the topic-mode command for that current row.
    #[test]
    fn index_topic_selection_stays_in_index_until_the_mode_command_opens_it() {
        let populated = view(&["alpha", "beta"], None, Some("alpha"), 1);
        let mut state = state(None, Some("alpha"), EncyclopediaMode::Index);

        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &populated,
                EncyclopediaAction::SelectTopic("beta".to_owned())
            ),
            NavigationOutcome::Applied
        );
        assert_eq!(state.mode(), EncyclopediaMode::Index);
        assert!(state.take_body_scroll_intents().is_empty());
        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &populated,
                EncyclopediaAction::SetMode(EncyclopediaMode::Topic)
            ),
            NavigationOutcome::Applied
        );
        assert_eq!(state.mode(), EncyclopediaMode::Topic);
        assert_eq!(
            state.take_body_scroll_intents(),
            vec![BodyScrollIntent::ResetToTop]
        );
    }

    #[test]
    fn reconcile_preserves_a_canonical_topic_across_reorder_and_clears_reused_numeric_ids() {
        let renamed = view(&["ship:8", "ship:7", "ship:9"], None, Some("ship:7"), 2);
        let mut state = state(None, Some("ship:7"), EncyclopediaMode::Topic);

        assert_eq!(
            reconcile_encyclopedia_state(&mut state, &renamed),
            ReconcileOutcome::Unchanged
        );
        assert_eq!(state.selection.topic_id.as_deref(), Some("ship:7"));
        assert_eq!(
            apply_encyclopedia_action(&mut state, &renamed, EncyclopediaAction::PreviousTopic),
            NavigationOutcome::Applied
        );
        assert_eq!(state.selection.topic_id.as_deref(), Some("ship:8"));

        let replacement = view(&["fighter:7"], None, None, 3);
        assert_eq!(
            reconcile_encyclopedia_state(&mut state, &replacement),
            ReconcileOutcome::SelectionAdjusted
        );
        assert_eq!(state.selection.topic_id, None);
        assert_eq!(state.mode(), EncyclopediaMode::Index);

        state.selection.topic_id = Some("fighter:7".to_owned());
        let next_replacement = view(&["fighter:8", "fighter:7"], None, Some("fighter:7"), 4);
        assert_eq!(
            reconcile_encyclopedia_state(&mut state, &next_replacement),
            ReconcileOutcome::Unchanged
        );
        assert_eq!(state.selection.topic_id.as_deref(), Some("fighter:7"));
    }

    #[test]
    fn reconcile_repairs_disabled_category_and_public_selection_drift() {
        let current = view(&["alpha"], None, None, 1);
        let mut disabled = state(Some("hidden"), None, EncyclopediaMode::Index);

        assert_eq!(
            reconcile_encyclopedia_state(&mut disabled, &current),
            ReconcileOutcome::SelectionAdjusted
        );
        assert_eq!(disabled.selection.category_id, None);
        assert_eq!(disabled.selected_category_id(), None);

        let mut missing_selected_control = EncyclopediaState::default();
        missing_selected_control.selection.category_id = Some("ships".to_owned());
        assert_eq!(
            reconcile_encyclopedia_state(&mut missing_selected_control, &current),
            ReconcileOutcome::SelectionAdjusted
        );
        assert_eq!(
            missing_selected_control.selected_category_id(),
            Some("ships")
        );

        let mut distinct_selected_control = state(Some("forces"), None, EncyclopediaMode::Index);
        distinct_selected_control.selection.category_id = Some("ships".to_owned());
        assert_eq!(
            reconcile_encyclopedia_state(&mut distinct_selected_control, &current),
            ReconcileOutcome::Unchanged
        );
        assert_eq!(
            distinct_selected_control.selected_category_id(),
            Some("forces")
        );
        assert_eq!(
            distinct_selected_control.selection.category_id.as_deref(),
            Some("ships")
        );

        let mut topic_without_current = EncyclopediaState::default();
        topic_without_current.mode = EncyclopediaMode::Topic;
        assert_eq!(
            reconcile_encyclopedia_state(&mut topic_without_current, &current),
            ReconcileOutcome::SelectionAdjusted
        );
        assert_eq!(topic_without_current.mode(), EncyclopediaMode::Index);
    }

    // Source: encyclopedia-source-contract.md, FUN_0045f100 step 1. The equality check is on
    // the selected command, not on the first enabled category and not on force.
    #[test]
    fn category_command_equality_uses_the_selected_control_and_aggregate_repeat_is_a_no_op() {
        let current = view(&["alpha"], Some("forces"), Some("alpha"), 1);
        let mut family = state(Some("forces"), Some("alpha"), EncyclopediaMode::Index);

        assert_eq!(
            apply_encyclopedia_action(
                &mut family,
                &current,
                EncyclopediaAction::SelectCategory {
                    category_id: Some("ships".to_owned()),
                    force: SelectionForce::Normal,
                }
            ),
            NavigationOutcome::Applied
        );
        assert_eq!(family.selected_category_id(), Some("ships"));
        assert_eq!(family.selection.category_id.as_deref(), Some("ships"));

        let mut aggregate = state(None, Some("alpha"), EncyclopediaMode::Index);
        let before = aggregate.clone();
        assert_eq!(
            apply_encyclopedia_action(
                &mut aggregate,
                &current,
                EncyclopediaAction::SelectCategory {
                    category_id: None,
                    force: SelectionForce::Forced,
                }
            ),
            NavigationOutcome::NoChange
        );
        assert_eq!(aggregate, before);
    }

    #[test]
    fn rename_language_fallback_and_tie_order_change_neighbors_without_changing_identity() {
        let requested_language = view(&["c", "a", "b"], None, Some("a"), 4);
        let fallback_language = view(&["b", "a", "c"], None, Some("a"), 4);
        let tied_registry_order = view(&["a", "c", "b"], None, Some("a"), 4);
        let mut state = state(None, Some("a"), EncyclopediaMode::Topic);

        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &requested_language,
                EncyclopediaAction::PreviousTopic
            ),
            NavigationOutcome::Applied
        );
        assert_eq!(state.selection.topic_id.as_deref(), Some("c"));

        state.selection.topic_id = Some("a".to_owned());
        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &fallback_language,
                EncyclopediaAction::PreviousTopic
            ),
            NavigationOutcome::Applied
        );
        assert_eq!(state.selection.topic_id.as_deref(), Some("b"));

        state.selection.topic_id = Some("a".to_owned());
        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &tied_registry_order,
                EncyclopediaAction::NextTopic
            ),
            NavigationOutcome::Applied
        );
        assert_eq!(state.selection.topic_id.as_deref(), Some("c"));
    }

    #[test]
    fn faction_and_art_variant_replacements_preserve_the_original_topic_id() {
        let mut alliance = view(&["character:42"], None, Some("character:42"), 8);
        alliance.active_topic.as_mut().unwrap().image = Some(TopicImageView {
            asset_id: "edata:alliance".to_owned(),
            digest: "alliance-digest".to_owned(),
            format: "bmp".to_owned(),
            width: 1,
            height: 1,
            bytes: Arc::from([1_u8, 2, 3, 4]),
            render_profile: TopicImageRenderProfile::OriginalNearest,
        });
        let mut empire = alliance.clone();
        empire.navigation.world_epoch = 9;
        empire.active_topic.as_mut().unwrap().image = Some(TopicImageView {
            asset_id: "edata:empire".to_owned(),
            digest: "empire-digest".to_owned(),
            format: "bmp".to_owned(),
            width: 1,
            height: 1,
            bytes: Arc::from([5_u8, 6, 7, 8]),
            render_profile: TopicImageRenderProfile::OriginalNearest,
        });
        let mut state = state(None, Some("character:42"), EncyclopediaMode::Topic);

        assert_eq!(
            reconcile_encyclopedia_state(&mut state, &alliance),
            ReconcileOutcome::Unchanged
        );
        assert_eq!(
            reconcile_encyclopedia_state(&mut state, &empire),
            ReconcileOutcome::Unchanged
        );
        assert_eq!(state.selection.topic_id.as_deref(), Some("character:42"));
    }

    #[test]
    fn a_rejected_content_replacement_leaves_navigation_on_the_prior_published_order() {
        let retained = view(&["alpha", "beta", "gamma"], None, Some("beta"), 10);
        let mut state = state(None, Some("beta"), EncyclopediaMode::Topic);

        assert_eq!(
            apply_encyclopedia_action(&mut state, &retained, EncyclopediaAction::NextTopic),
            NavigationOutcome::Applied
        );
        assert_eq!(state.selection.topic_id.as_deref(), Some("gamma"));
        assert_eq!(
            apply_encyclopedia_action(&mut state, &retained, EncyclopediaAction::PreviousTopic),
            NavigationOutcome::Applied
        );
        assert_eq!(state.selection.topic_id.as_deref(), Some("beta"));
    }

    #[test]
    fn batched_topic_change_and_scroll_preserve_reset_before_the_scroll_command() {
        let view = view(&["alpha", "beta", "gamma"], None, Some("beta"), 11);
        let mut state = state(None, Some("beta"), EncyclopediaMode::Topic);

        assert_eq!(
            apply_encyclopedia_action(&mut state, &view, EncyclopediaAction::NextTopic),
            NavigationOutcome::Applied
        );
        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &view,
                EncyclopediaAction::Scroll(BodyScrollIntent::LineDown)
            ),
            NavigationOutcome::ScrollRequested(BodyScrollIntent::LineDown)
        );

        assert_eq!(
            state.pending_body_scroll_intents(),
            &[BodyScrollIntent::ResetToTop, BodyScrollIntent::LineDown]
        );
        assert_eq!(
            state.take_body_scroll_intents(),
            vec![BodyScrollIntent::ResetToTop, BodyScrollIntent::LineDown]
        );
        assert!(state.take_body_scroll_intents().is_empty());
    }

    #[test]
    fn repeated_batched_scroll_commands_are_not_collapsed() {
        let view = view(&["alpha"], None, Some("alpha"), 11);
        let mut state = state(None, Some("alpha"), EncyclopediaMode::Topic);

        for _ in 0..2 {
            assert_eq!(
                apply_encyclopedia_action(
                    &mut state,
                    &view,
                    EncyclopediaAction::Scroll(BodyScrollIntent::LineDown)
                ),
                NavigationOutcome::ScrollRequested(BodyScrollIntent::LineDown)
            );
        }

        assert_eq!(
            state.take_body_scroll_intents(),
            vec![BodyScrollIntent::LineDown, BodyScrollIntent::LineDown]
        );
        assert!(state.take_body_scroll_intents().is_empty());
    }

    #[test]
    fn reset_page_and_reconciliation_commands_compose_in_action_order() {
        let current = view(&["alpha", "beta", "gamma"], None, Some("beta"), 11);
        let mut state = state(None, Some("beta"), EncyclopediaMode::Topic);

        assert_eq!(
            apply_encyclopedia_action(&mut state, &current, EncyclopediaAction::NextTopic),
            NavigationOutcome::Applied
        );
        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &current,
                EncyclopediaAction::Scroll(BodyScrollIntent::PageDown)
            ),
            NavigationOutcome::ScrollRequested(BodyScrollIntent::PageDown)
        );
        assert_eq!(
            state.take_body_scroll_intents(),
            vec![BodyScrollIntent::ResetToTop, BodyScrollIntent::PageDown]
        );

        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &current,
                EncyclopediaAction::Scroll(BodyScrollIntent::LineUp)
            ),
            NavigationOutcome::ScrollRequested(BodyScrollIntent::LineUp)
        );
        let replacement = view(&["alpha", "beta"], None, None, 12);
        assert_eq!(
            reconcile_encyclopedia_state(&mut state, &replacement),
            ReconcileOutcome::SelectionAdjusted
        );
        assert_eq!(
            state.take_body_scroll_intents(),
            vec![BodyScrollIntent::LineUp, BodyScrollIntent::ResetToTop]
        );
    }

    #[test]
    fn rejected_and_no_change_actions_preserve_an_existing_scroll_batch() {
        let view = view(&["alpha"], None, Some("alpha"), 11);
        let mut state = state(None, Some("alpha"), EncyclopediaMode::Topic);

        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &view,
                EncyclopediaAction::Scroll(BodyScrollIntent::PageUp)
            ),
            NavigationOutcome::ScrollRequested(BodyScrollIntent::PageUp)
        );
        assert_eq!(
            apply_encyclopedia_action(
                &mut state,
                &view,
                EncyclopediaAction::SelectTopic("hidden".to_owned())
            ),
            NavigationOutcome::Rejected(NavigationRejection::UnavailableTopic)
        );
        assert_eq!(
            apply_encyclopedia_action(&mut state, &view, EncyclopediaAction::NextTopic),
            NavigationOutcome::NoChange
        );
        assert_eq!(
            state.take_body_scroll_intents(),
            vec![BodyScrollIntent::PageUp]
        );
        assert!(state.take_body_scroll_intents().is_empty());
    }

    #[test]
    fn equivalent_native_and_browser_action_streams_are_identical_and_inputs_stay_immutable() {
        let native_view = view(&["alpha", "beta", "gamma"], None, Some("beta"), 11);
        let browser_view = native_view.clone();
        let view_before = native_view.clone();
        let world_snapshot = vec![1_u64, 2, 3];
        let rng_snapshot = [7_u8, 8, 9];
        let world_before = world_snapshot.clone();
        let rng_before = rng_snapshot;
        let mut native = state(None, Some("beta"), EncyclopediaMode::Topic);
        let mut browser = native.clone();
        let actions = [
            EncyclopediaAction::PreviousTopic,
            EncyclopediaAction::NextTopic,
            EncyclopediaAction::Scroll(BodyScrollIntent::LineDown),
            EncyclopediaAction::Return,
            EncyclopediaAction::Close,
        ];

        for action in actions {
            let native_outcome =
                apply_encyclopedia_action(&mut native, &native_view, action.clone());
            let browser_outcome = apply_encyclopedia_action(&mut browser, &browser_view, action);
            assert_eq!(native_outcome, browser_outcome);
        }

        assert_eq!(native, browser);
        assert_eq!(
            native.take_body_scroll_intents(),
            vec![
                BodyScrollIntent::ResetToTop,
                BodyScrollIntent::ResetToTop,
                BodyScrollIntent::LineDown,
            ]
        );
        assert_eq!(
            browser.take_body_scroll_intents(),
            vec![
                BodyScrollIntent::ResetToTop,
                BodyScrollIntent::ResetToTop,
                BodyScrollIntent::LineDown,
            ]
        );
        assert!(native.take_body_scroll_intents().is_empty());
        assert!(browser.take_body_scroll_intents().is_empty());
        assert_eq!(native_view, view_before);
        assert_eq!(world_snapshot, world_before);
        assert_eq!(rng_snapshot, rng_before);
    }
}
