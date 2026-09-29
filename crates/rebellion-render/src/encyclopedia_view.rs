use std::sync::Arc;

/// Stable-ID selection borrowed by the pure presenter. Navigation code owns
/// changes to these IDs; view construction never mutates them.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EncyclopediaSelection {
    pub category_id: Option<String>,
    pub topic_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CategoryViewItem {
    pub category_id: String,
    pub command: String,
    pub label: Option<Arc<str>>,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopicViewItem {
    pub topic_id: String,
    pub title: Arc<str>,
}

/// Original first-profile EData is presented without interpolation. Later
/// byte selectors can add reviewed profile intents without changing asset IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TopicImageRenderProfile {
    OriginalNearest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopicImageView {
    pub asset_id: String,
    pub digest: String,
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub bytes: Arc<[u8]>,
    pub render_profile: TopicImageRenderProfile,
}

impl TopicImageView {
    /// Immutable identity used by E46's texture cache. The digest describes
    /// the exact selected bytes, not merely the catalog declaration.
    #[must_use]
    pub fn cache_key(&self) -> (&str, &str, TopicImageRenderProfile) {
        (&self.asset_id, &self.digest, self.render_profile)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatRowView {
    pub label: Arc<str>,
    pub value: Arc<str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveTopicView {
    pub topic_id: String,
    pub title: Arc<str>,
    pub body: Arc<str>,
    pub image: Option<TopicImageView>,
    pub stats: Vec<StatRowView>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationState {
    pub selected_category_id: Option<String>,
    pub selected_topic_id: Option<String>,
    pub previous_topic_id: Option<String>,
    pub next_topic_id: Option<String>,
    pub world_epoch: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncyclopediaDiagnosticScope {
    Index,
    Category { category_id: String },
    Topic { topic_id: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaViewDiagnostic {
    pub code: &'static str,
    pub scope: EncyclopediaDiagnosticScope,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaView {
    pub index_label: Option<Arc<str>>,
    pub index_enabled: bool,
    pub categories: Vec<CategoryViewItem>,
    pub topics: Vec<TopicViewItem>,
    pub active_topic: Option<ActiveTopicView>,
    pub navigation: NavigationState,
    pub diagnostics: Vec<EncyclopediaViewDiagnostic>,
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::{EncyclopediaSelection, TopicImageRenderProfile, TopicImageView};

    #[test]
    fn selection_defaults_to_the_source_index_without_fabricating_ids() {
        let selection = EncyclopediaSelection::default();

        assert_eq!(selection.category_id, None);
        assert_eq!(selection.topic_id, None);
    }

    #[test]
    fn image_cache_identity_uses_actual_asset_digest_and_profile() {
        let bytes: Arc<[u8]> = Arc::from([1_u8, 2, 3]);
        let image = TopicImageView {
            asset_id: "edata:7".to_owned(),
            digest: "actual-digest".to_owned(),
            format: "bmp".to_owned(),
            width: 2,
            height: 1,
            bytes: bytes.clone(),
            render_profile: TopicImageRenderProfile::OriginalNearest,
        };

        assert_eq!(
            image.cache_key(),
            (
                "edata:7",
                "actual-digest",
                TopicImageRenderProfile::OriginalNearest
            )
        );
        assert!(Arc::ptr_eq(&image.bytes, &bytes));
    }
}
