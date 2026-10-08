//! Pure, source-ordered Galactic Encyclopedia presentation.
//!
//! This module projects one validated immutable session into renderer-facing
//! text and resource identities. It performs no I/O, decoding, texture work,
//! campaign lookup, or mutation. Entry and return values retain closed,
//! source-backed caller identities so a fixture-only route cannot leak into a
//! production presentation.

use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::encyclopedia_catalog::{EncyclopediaCatalogEntry, EncyclopediaCategory};
use crate::encyclopedia_session::{EncyclopediaResourceMetadata, EncyclopediaSession};
use crate::encyclopedia_topics::{EncyclopediaAudience, EncyclopediaTopicBinding};

/// The five direct contextual call sites recovered for the original viewer.
///
/// Their visible owning surfaces are not all source-named, so the stable call
/// address and command/event remain the authority instead of guessed labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaContextCaller {
    Handler00438800Command67,
    Handler004443a0Command66,
    Handler00467f10Command67Or97,
    MissionDialog0046c3c0Command67,
    Handler00486fb0Event100,
}

/// One entry through the shared presenter API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaEntryIntent {
    /// Context-free command `0x131` or F7 entry into the full index.
    Cockpit { audience: EncyclopediaAudience },
    /// An already-open index state, including its stable row selection.
    Index {
        audience: EncyclopediaAudience,
        category_command: u16,
        selected_object_id: Option<u32>,
    },
    /// A topic opened from one row in an index projection.
    Object {
        audience: EncyclopediaAudience,
        category_command: u16,
        object_id: u32,
    },
    /// A typed source caller requesting one object directly.
    Contextual {
        audience: EncyclopediaAudience,
        object_id: u32,
        caller: EncyclopediaContextCaller,
    },
    /// A transition inside an open viewer that retains its immutable origin.
    FollowUp {
        category_command: u16,
        selected_object_id: Option<u32>,
        mode: EncyclopediaPresentationMode,
        return_route: EncyclopediaReturnRoute,
    },
}

/// Stable destination retained for close/return handling by the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaReturnRoute {
    Cockpit {
        audience: EncyclopediaAudience,
    },
    Index {
        audience: EncyclopediaAudience,
        category_command: u16,
        selected_object_id: Option<u32>,
    },
    Contextual {
        audience: EncyclopediaAudience,
        requested_object_id: u32,
        caller: EncyclopediaContextCaller,
    },
}

impl EncyclopediaReturnRoute {
    const fn audience(self) -> EncyclopediaAudience {
        match self {
            Self::Cockpit { audience }
            | Self::Index { audience, .. }
            | Self::Contextual { audience, .. } => audience,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaPresentationMode {
    Index,
    Topic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaTopicAvailability {
    Resolved,
    SourceUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncyclopediaCategoryView<'a> {
    pub command_id: u16,
    pub label_resource_id: u16,
    pub label: &'a str,
    pub topic_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncyclopediaTopicListItem<'a> {
    pub object_id: u32,
    pub title: &'a str,
    pub availability: EncyclopediaTopicAvailability,
}

/// Topic prose and its validated resource identity, without decoded pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaTopicContent<'a> {
    Resolved {
        description: &'a str,
        artwork_resource_id: u16,
        artwork_filename: &'a str,
        artwork_metadata: &'a EncyclopediaResourceMetadata,
    },
    /// The source-proven object exists in the index but has neither prose nor
    /// an artwork mapping. No fallback content is supplied.
    SourceUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncyclopediaTopicView<'a> {
    pub object_id: u32,
    pub title: &'a str,
    pub topic_text_resource_id: u16,
    pub category_command: u16,
    pub audience: EncyclopediaAudience,
    pub content: EncyclopediaTopicContent<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncyclopediaNavigation {
    pub selected_object_id: Option<u32>,
    pub previous_object_id: Option<u32>,
    pub next_object_id: Option<u32>,
}

/// Complete graphics-free view for one index or topic frame.
#[derive(Debug, PartialEq, Eq)]
pub struct EncyclopediaPresentation<'a> {
    pub title: &'a str,
    pub topic_label: &'a str,
    pub audience: EncyclopediaAudience,
    pub mode: EncyclopediaPresentationMode,
    pub category: EncyclopediaCategoryView<'a>,
    pub categories: Vec<EncyclopediaCategoryView<'a>>,
    pub topics: Vec<EncyclopediaTopicListItem<'a>>,
    pub active_topic: Option<EncyclopediaTopicView<'a>>,
    pub navigation: EncyclopediaNavigation,
    pub return_route: EncyclopediaReturnRoute,
}

impl EncyclopediaPresentation<'_> {
    /// Build an in-view transition without reconstructing or changing the
    /// caller to which close/return must eventually navigate.
    pub const fn follow_up(
        &self,
        mode: EncyclopediaPresentationMode,
        category_command: u16,
        selected_object_id: Option<u32>,
    ) -> EncyclopediaEntryIntent {
        EncyclopediaEntryIntent::FollowUp {
            category_command,
            selected_object_id,
            mode,
            return_route: self.return_route,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaPresenterError {
    MissingTopicSelection,
    UnknownCategory(u16),
    UnknownObject(u32),
    ObjectOutsideCategory {
        object_id: u32,
        category_command: u16,
    },
}

impl Display for EncyclopediaPresenterError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingTopicSelection => {
                formatter.write_str("Encyclopedia topic mode requires a selected object")
            }
            Self::UnknownCategory(command) => {
                write!(formatter, "unknown Encyclopedia category {command:#04x}")
            }
            Self::UnknownObject(object_id) => {
                write!(formatter, "unknown Encyclopedia object {object_id:#010x}")
            }
            Self::ObjectOutsideCategory {
                object_id,
                category_command,
            } => write!(
                formatter,
                "Encyclopedia object {object_id:#010x} is outside category {category_command:#04x}"
            ),
        }
    }
}

impl Error for EncyclopediaPresenterError {}

/// Stateless presenter over one immutable session.
#[derive(Debug, Default)]
pub struct EncyclopediaPresenter;

impl EncyclopediaPresenter {
    /// Derive one source-ordered index or topic view.
    ///
    /// Contextual misses follow the original unresolved-context behavior and
    /// open the full index while retaining the exact caller token. Invalid
    /// internal index/object selections fail closed instead of guessing.
    pub fn present<'a>(
        session: &'a EncyclopediaSession,
        intent: EncyclopediaEntryIntent,
    ) -> Result<EncyclopediaPresentation<'a>, EncyclopediaPresenterError> {
        let normalized = NormalizedEntry::from_intent(session, intent)?;
        let catalog = session.catalog();
        let category = catalog.category(normalized.category_command).ok_or(
            EncyclopediaPresenterError::UnknownCategory(normalized.category_command),
        )?;
        let entries = catalog
            .entries
            .iter()
            .filter(|entry| category.contains(entry))
            .collect::<Vec<_>>();

        let selected_position = normalized
            .selected_object_id
            .and_then(|selected| entries.iter().position(|entry| entry.object_id == selected));
        if let (Some(object_id), None) = (normalized.selected_object_id, selected_position) {
            if catalog
                .entries
                .iter()
                .any(|entry| entry.object_id == object_id)
            {
                return Err(EncyclopediaPresenterError::ObjectOutsideCategory {
                    object_id,
                    category_command: normalized.category_command,
                });
            }
            return Err(EncyclopediaPresenterError::UnknownObject(object_id));
        }

        let topic_catalog = session.topics(normalized.audience);
        let topics = entries
            .iter()
            .map(|entry| {
                let binding = topic_catalog
                    .topic(entry.object_id)
                    .expect("validated session binds every catalog object");
                EncyclopediaTopicListItem {
                    object_id: entry.object_id,
                    title: &entry.name,
                    availability: topic_availability(binding),
                }
            })
            .collect();

        let active_topic = if normalized.mode == EncyclopediaPresentationMode::Topic {
            selected_position.map(|position| {
                topic_view(
                    session,
                    entries[position],
                    topic_catalog
                        .topic(entries[position].object_id)
                        .expect("validated session binds every catalog object"),
                    normalized.category_command,
                    normalized.audience,
                )
            })
        } else {
            None
        };
        let (previous_object_id, next_object_id) =
            if normalized.mode == EncyclopediaPresentationMode::Topic {
                (
                    selected_position
                        .and_then(|position| position.checked_sub(1))
                        .map(|position| entries[position].object_id),
                    selected_position
                        .and_then(|position| position.checked_add(1))
                        .and_then(|position| entries.get(position))
                        .map(|entry| entry.object_id),
                )
            } else {
                (None, None)
            };

        Ok(EncyclopediaPresentation {
            title: &catalog.title,
            topic_label: &catalog.topic_label,
            audience: normalized.audience,
            mode: normalized.mode,
            category: category_view(category, entries.len()),
            categories: catalog
                .categories
                .iter()
                .map(|candidate| {
                    category_view(
                        candidate,
                        catalog
                            .entries
                            .iter()
                            .filter(|entry| candidate.contains(entry))
                            .count(),
                    )
                })
                .collect(),
            topics,
            active_topic,
            navigation: EncyclopediaNavigation {
                selected_object_id: normalized.selected_object_id,
                previous_object_id,
                next_object_id,
            },
            return_route: normalized.return_route,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct NormalizedEntry {
    audience: EncyclopediaAudience,
    category_command: u16,
    selected_object_id: Option<u32>,
    mode: EncyclopediaPresentationMode,
    return_route: EncyclopediaReturnRoute,
}

impl NormalizedEntry {
    fn from_intent(
        session: &EncyclopediaSession,
        intent: EncyclopediaEntryIntent,
    ) -> Result<Self, EncyclopediaPresenterError> {
        let entry = match intent {
            EncyclopediaEntryIntent::Cockpit { audience } => Self {
                audience,
                category_command: 0x6f,
                selected_object_id: None,
                mode: EncyclopediaPresentationMode::Index,
                return_route: EncyclopediaReturnRoute::Cockpit { audience },
            },
            EncyclopediaEntryIntent::Index {
                audience,
                category_command,
                selected_object_id,
            } => Self {
                audience,
                category_command,
                selected_object_id,
                mode: EncyclopediaPresentationMode::Index,
                return_route: EncyclopediaReturnRoute::Index {
                    audience,
                    category_command,
                    selected_object_id,
                },
            },
            EncyclopediaEntryIntent::Object {
                audience,
                category_command,
                object_id,
            } => Self {
                audience,
                category_command,
                selected_object_id: Some(object_id),
                mode: EncyclopediaPresentationMode::Topic,
                return_route: EncyclopediaReturnRoute::Index {
                    audience,
                    category_command,
                    selected_object_id: Some(object_id),
                },
            },
            EncyclopediaEntryIntent::Contextual {
                audience,
                object_id,
                caller,
            } => {
                let selected_object_id = session
                    .catalog()
                    .entries
                    .iter()
                    .any(|entry| entry.object_id == object_id)
                    .then_some(object_id);
                Self {
                    audience,
                    category_command: 0x6f,
                    selected_object_id,
                    mode: if selected_object_id.is_some() {
                        EncyclopediaPresentationMode::Topic
                    } else {
                        EncyclopediaPresentationMode::Index
                    },
                    return_route: EncyclopediaReturnRoute::Contextual {
                        audience,
                        requested_object_id: object_id,
                        caller,
                    },
                }
            }
            EncyclopediaEntryIntent::FollowUp {
                category_command,
                selected_object_id,
                mode,
                return_route,
            } => Self {
                audience: return_route.audience(),
                category_command,
                selected_object_id,
                mode,
                return_route,
            },
        };
        if session.catalog().category(entry.category_command).is_none() {
            return Err(EncyclopediaPresenterError::UnknownCategory(
                entry.category_command,
            ));
        }
        if entry.mode == EncyclopediaPresentationMode::Topic && entry.selected_object_id.is_none() {
            return Err(EncyclopediaPresenterError::MissingTopicSelection);
        }
        Ok(entry)
    }
}

fn category_view(
    category: &EncyclopediaCategory,
    topic_count: usize,
) -> EncyclopediaCategoryView<'_> {
    EncyclopediaCategoryView {
        command_id: category.command_id,
        label_resource_id: category.label_resource_id,
        label: &category.label,
        topic_count,
    }
}

fn topic_availability(binding: &EncyclopediaTopicBinding) -> EncyclopediaTopicAvailability {
    if binding.is_complete() {
        EncyclopediaTopicAvailability::Resolved
    } else {
        EncyclopediaTopicAvailability::SourceUnavailable
    }
}

fn topic_view<'a>(
    session: &'a EncyclopediaSession,
    entry: &'a EncyclopediaCatalogEntry,
    binding: &'a EncyclopediaTopicBinding,
    category_command: u16,
    audience: EncyclopediaAudience,
) -> EncyclopediaTopicView<'a> {
    let content = if binding.is_complete() {
        let description = binding
            .body
            .as_deref()
            .expect("validated complete topic has prose");
        let artwork_resource_id = binding
            .artwork_resource_id
            .expect("validated complete topic has an artwork identity");
        let artwork_filename = binding
            .artwork_filename
            .as_deref()
            .expect("validated complete topic has an artwork filename");
        let artwork_metadata = session
            .resource_metadata()
            .get(artwork_filename)
            .expect("validated complete topic has retained artwork metadata");
        EncyclopediaTopicContent::Resolved {
            description,
            artwork_resource_id,
            artwork_filename,
            artwork_metadata,
        }
    } else {
        EncyclopediaTopicContent::SourceUnavailable
    };
    EncyclopediaTopicView {
        object_id: entry.object_id,
        title: &entry.name,
        topic_text_resource_id: binding.topic_text_resource_id,
        category_command,
        audience,
        content,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};
    use std::sync::Arc;

    use sha2::{Digest, Sha256};

    use super::{
        EncyclopediaContextCaller, EncyclopediaEntryIntent, EncyclopediaPresentationMode,
        EncyclopediaPresenter, EncyclopediaPresenterError, EncyclopediaReturnRoute,
        EncyclopediaTopicAvailability, EncyclopediaTopicContent,
    };
    use crate::encyclopedia_catalog::{
        EncyclopediaCatalog, EncyclopediaCatalogEntry, EncyclopediaCategory,
    };
    use crate::encyclopedia_session::{
        EncyclopediaResourceBytes, EncyclopediaSession, EncyclopediaSessionInput,
        EncyclopediaSessionStore,
    };
    use crate::encyclopedia_topics::EncyclopediaAudience;

    const SHARED_SOURCE: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/p66a/source.json");
    const SHARED_MANIFEST: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/p66a/source.json.manifest.json");

    fn category(
        command_id: u16,
        label_resource_id: u16,
        family_range: Option<std::ops::Range<u8>>,
    ) -> EncyclopediaCategory {
        EncyclopediaCategory {
            command_id,
            label_resource_id,
            label: format!("Category {command_id}"),
            family_range,
        }
    }

    fn fixture_catalog() -> EncyclopediaCatalog {
        EncyclopediaCatalog {
            title: "Galactic Encyclopedia".into(),
            topic_label: "Topic".into(),
            categories: [
                category(0x6f, 0x1850, None),
                category(0x70, 0x1855, Some(0x90..0x98)),
                category(0x71, 0x1854, Some(0x14..0x20)),
                category(0x72, 0x1852, Some(0x20..0x30)),
                category(0x73, 0x1851, Some(0x40..0x80)),
                category(0x74, 0x1856, Some(0x10..0x14)),
                category(0x75, 0x1853, Some(0x30..0x40)),
            ],
            entries: vec![
                EncyclopediaCatalogEntry {
                    object_id: 0x9200_0064,
                    text_resource_id: 0x2e00,
                    name: "Abregado-rae".into(),
                },
                EncyclopediaCatalogEntry {
                    object_id: 0x5100_0010,
                    text_resource_id: 0x2c50,
                    name: "Diplomacy".into(),
                },
                EncyclopediaCatalogEntry {
                    object_id: 0x1400_0040,
                    text_resource_id: 0x2740,
                    name: "Mon Calamari Cruiser".into(),
                },
            ],
        }
    }

    fn indexed_bmp(pixel: [u8; 3]) -> Arc<[u8]> {
        const WIDTH: u32 = 400;
        const HEIGHT: u32 = 200;
        const PIXEL_OFFSET: u32 = 14 + 40 + 1024;
        const FILE_BYTES: u32 = PIXEL_OFFSET + WIDTH * HEIGHT;
        let mut bytes = Vec::with_capacity(FILE_BYTES as usize);
        bytes.extend_from_slice(b"BM");
        bytes.extend_from_slice(&FILE_BYTES.to_le_bytes());
        bytes.extend_from_slice(&[0; 4]);
        bytes.extend_from_slice(&PIXEL_OFFSET.to_le_bytes());
        bytes.extend_from_slice(&40_u32.to_le_bytes());
        bytes.extend_from_slice(&(WIDTH as i32).to_le_bytes());
        bytes.extend_from_slice(&(HEIGHT as i32).to_le_bytes());
        bytes.extend_from_slice(&1_u16.to_le_bytes());
        bytes.extend_from_slice(&8_u16.to_le_bytes());
        bytes.extend_from_slice(&0_u32.to_le_bytes());
        bytes.extend_from_slice(&(WIDTH * HEIGHT).to_le_bytes());
        bytes.extend_from_slice(&[0; 8]);
        bytes.extend_from_slice(&256_u32.to_le_bytes());
        bytes.extend_from_slice(&0_u32.to_le_bytes());
        bytes.resize(PIXEL_OFFSET as usize, 0);
        bytes[58..62].copy_from_slice(&[pixel[2], pixel[1], pixel[0], 0]);
        bytes.resize(FILE_BYTES as usize, 1);
        bytes.into()
    }

    fn fixture_session_with_catalog(catalog: EncyclopediaCatalog) -> Arc<EncyclopediaSession> {
        let artwork: EncyclopediaResourceBytes = BTreeMap::from([
            ("EDATA.001".into(), indexed_bmp([1, 2, 3])),
            ("EDATA.014".into(), indexed_bmp([14, 15, 16])),
            ("EDATA.015".into(), indexed_bmp([15, 16, 17])),
            ("EDATA.115".into(), indexed_bmp([115, 116, 117])),
        ]);
        let mut store = EncyclopediaSessionStore::default();
        store
            .replace(EncyclopediaSessionInput {
                catalog,
                source_catalog_bytes: Arc::from(SHARED_SOURCE),
                source_manifest_bytes: Arc::from(SHARED_MANIFEST),
                system_pictures: HashMap::from([(0x9200_0064, 1)]),
                artwork,
            })
            .unwrap();
        store.current().unwrap()
    }

    fn fixture_session() -> Arc<EncyclopediaSession> {
        fixture_session_with_catalog(fixture_catalog())
    }

    fn full_catalog() -> EncyclopediaCatalog {
        let mut entries = Vec::with_capacity(346);
        let mut ordinal = 1_u16;
        let mut push = |object_id: u32| {
            entries.push(EncyclopediaCatalogEntry {
                object_id,
                text_resource_id: 0x2000 | ordinal,
                name: format!("Topic {ordinal:03}"),
            });
            ordinal += 1;
        };
        for record_id in 1..=200 {
            push(0x9000_0000 | record_id);
        }
        for record_id in 1..=38 {
            push(0x1400_0000 | record_id);
        }
        for record_id in 1..=14 {
            push(0x2000_0000 | record_id);
        }
        for record_id in 1..=15 {
            push(0x5000_0000 | record_id);
        }
        for record_id in 1..=10 {
            push(0x1000_0000 | record_id);
        }
        for record_id in 1..=69 {
            push(0x3000_0000 | record_id);
        }
        assert_eq!(entries.len(), 346);

        EncyclopediaCatalog {
            title: "Galactic Encyclopedia".into(),
            topic_label: "Topic".into(),
            categories: [
                category(0x6f, 0x1850, None),
                category(0x70, 0x1855, Some(0x90..0x98)),
                category(0x71, 0x1854, Some(0x14..0x20)),
                category(0x72, 0x1852, Some(0x20..0x30)),
                category(0x73, 0x1851, Some(0x40..0x80)),
                category(0x74, 0x1856, Some(0x10..0x14)),
                category(0x75, 0x1853, Some(0x30..0x40)),
            ],
            entries,
        }
    }

    fn full_session() -> Arc<EncyclopediaSession> {
        let catalog = full_catalog();
        let mut texts = serde_json::Map::new();
        let mut artwork = serde_json::Map::new();
        let mut system_pictures = HashMap::new();
        for entry in &catalog.entries {
            let key = (entry.text_resource_id & 0x0fff) + 0x1000;
            let body = format!("Synthetic body for {}.", entry.name);
            texts.insert(
                key.to_string(),
                serde_json::json!({
                    "body": body,
                    "body_sha256": format!("{:x}", Sha256::digest(body.as_bytes())),
                }),
            );
            match entry.family() {
                0x90..=0x97 => {
                    system_pictures.insert(entry.object_id, 1);
                    artwork.insert("11100".into(), serde_json::json!("EDATA.001"));
                }
                0x40..=0x7f => {
                    artwork.insert(key.to_string(), serde_json::json!("EDATA.001"));
                    artwork.insert((key + 0x1000).to_string(), serde_json::json!("EDATA.001"));
                }
                _ => {
                    artwork.insert(key.to_string(), serde_json::json!("EDATA.001"));
                }
            }
        }
        assert_eq!(texts.len(), 346);
        let text_count = texts.len();
        let artwork_count = artwork.len();
        let source_bytes = serde_json::to_vec(&serde_json::json!({
            "schema_version": 1,
            "language_id": 1033,
            "encoding": "windows-1252",
            "source_code_page": 0,
            "texts": texts,
            "artwork": artwork,
        }))
        .unwrap();
        let manifest_bytes = serde_json::to_vec(&serde_json::json!({
            "schema_version": 1,
            "catalog_sha256": format!("{:x}", Sha256::digest(&source_bytes)),
            "source_files": {
                "encytext_sha256": "1".repeat(64),
                "encybmap_sha256": "2".repeat(64),
            },
            "counts": {
                "texts": text_count,
                "artwork_mappings": artwork_count,
            },
        }))
        .unwrap();
        let mut store = EncyclopediaSessionStore::default();
        store
            .replace(EncyclopediaSessionInput {
                catalog,
                source_catalog_bytes: source_bytes.into(),
                source_manifest_bytes: manifest_bytes.into(),
                system_pictures,
                artwork: BTreeMap::from([("EDATA.001".into(), indexed_bmp([1, 2, 3]))]),
            })
            .unwrap();
        store.current().unwrap()
    }

    #[test]
    fn object_entry_derives_a_resolved_topic_and_bounded_source_order() {
        let session = fixture_session();

        let view = EncyclopediaPresenter::present(
            &session,
            EncyclopediaEntryIntent::Object {
                audience: EncyclopediaAudience::Alliance,
                category_command: 0x6f,
                object_id: 0x5100_0010,
            },
        )
        .unwrap();

        assert_eq!(view.mode, EncyclopediaPresentationMode::Topic);
        assert_eq!(view.audience, EncyclopediaAudience::Alliance);
        assert_eq!(view.category.command_id, 0x6f);
        assert_eq!(
            view.topics
                .iter()
                .map(|topic| topic.object_id)
                .collect::<Vec<_>>(),
            [0x9200_0064, 0x5100_0010, 0x1400_0040]
        );
        assert_eq!(view.navigation.previous_object_id, Some(0x9200_0064));
        assert_eq!(view.navigation.next_object_id, Some(0x1400_0040));
        assert_eq!(
            view.return_route,
            EncyclopediaReturnRoute::Index {
                audience: EncyclopediaAudience::Alliance,
                category_command: 0x6f,
                selected_object_id: Some(0x5100_0010),
            }
        );
        let topic = view.active_topic.unwrap();
        assert_eq!(topic.title, "Diplomacy");
        assert_eq!(topic.category_command, 0x6f);
        assert_eq!(topic.audience, EncyclopediaAudience::Alliance);
        assert!(matches!(
            topic.content,
            EncyclopediaTopicContent::Resolved {
                description: "Synthetic mission body.",
                artwork_filename: "EDATA.015",
                ..
            }
        ));
    }

    #[test]
    fn every_visible_entry_for_both_factions_is_resolved() {
        let session = full_session();

        for audience in [EncyclopediaAudience::Alliance, EncyclopediaAudience::Empire] {
            let index = EncyclopediaPresenter::present(
                &session,
                EncyclopediaEntryIntent::Cockpit { audience },
            )
            .unwrap();
            assert_eq!(index.mode, EncyclopediaPresentationMode::Index);
            assert_eq!(index.topics.len(), 346);
            assert_eq!(
                index
                    .categories
                    .iter()
                    .map(|category| category.topic_count)
                    .collect::<Vec<_>>(),
                [346, 200, 38, 14, 15, 10, 69]
            );
            assert_eq!(
                index
                    .topics
                    .iter()
                    .filter(|topic| topic.availability == EncyclopediaTopicAvailability::Resolved)
                    .count(),
                346
            );
            assert!(index
                .topics
                .iter()
                .all(|topic| topic.availability == EncyclopediaTopicAvailability::Resolved));

            for expected in &session.catalog().entries {
                let topic = EncyclopediaPresenter::present(
                    &session,
                    EncyclopediaEntryIntent::Object {
                        audience,
                        category_command: 0x6f,
                        object_id: expected.object_id,
                    },
                )
                .unwrap()
                .active_topic
                .unwrap();
                assert_eq!(topic.object_id, expected.object_id);
                assert_eq!(topic.title, expected.name);
                assert!(matches!(
                    topic.content,
                    EncyclopediaTopicContent::Resolved { .. }
                ));
            }
        }
    }

    #[test]
    fn category_projection_preserves_order_and_topic_endpoints_do_not_wrap() {
        let session = full_session();
        let ship_ids = session
            .catalog()
            .entries
            .iter()
            .filter(|entry| (0x14..0x20).contains(&entry.family()))
            .map(|entry| entry.object_id)
            .collect::<Vec<_>>();

        let first = EncyclopediaPresenter::present(
            &session,
            EncyclopediaEntryIntent::Object {
                audience: EncyclopediaAudience::Alliance,
                category_command: 0x71,
                object_id: ship_ids[0],
            },
        )
        .unwrap();
        assert_eq!(
            first
                .topics
                .iter()
                .map(|topic| topic.object_id)
                .collect::<Vec<_>>(),
            ship_ids
        );
        assert_eq!(first.navigation.previous_object_id, None);
        assert_eq!(first.navigation.next_object_id, Some(ship_ids[1]));

        let last = EncyclopediaPresenter::present(
            &session,
            EncyclopediaEntryIntent::Object {
                audience: EncyclopediaAudience::Alliance,
                category_command: 0x71,
                object_id: *ship_ids.last().unwrap(),
            },
        )
        .unwrap();
        assert_eq!(
            last.navigation.previous_object_id,
            Some(ship_ids[ship_ids.len() - 2])
        );
        assert_eq!(last.navigation.next_object_id, None);
    }

    #[test]
    fn presenter_preserves_literal_mixed_case_and_equal_fold_catalog_order() {
        let mut catalog = fixture_catalog();
        catalog.entries = vec![
            EncyclopediaCatalogEntry {
                object_id: 0x1400_0040,
                text_resource_id: 0x2740,
                name: "alpha".into(),
            },
            EncyclopediaCatalogEntry {
                object_id: 0x5100_0010,
                text_resource_id: 0x2c50,
                name: "ALPHA".into(),
            },
            EncyclopediaCatalogEntry {
                object_id: 0x9200_0064,
                text_resource_id: 0x2e00,
                name: "Beta".into(),
            },
        ];
        let session = fixture_session_with_catalog(catalog);

        let full_index = EncyclopediaPresenter::present(
            &session,
            EncyclopediaEntryIntent::Cockpit {
                audience: EncyclopediaAudience::Alliance,
            },
        )
        .unwrap();
        assert_eq!(
            full_index
                .topics
                .iter()
                .map(|topic| (topic.object_id, topic.title))
                .collect::<Vec<_>>(),
            [
                (0x1400_0040, "alpha"),
                (0x5100_0010, "ALPHA"),
                (0x9200_0064, "Beta"),
            ]
        );

        let mission_index = EncyclopediaPresenter::present(
            &session,
            EncyclopediaEntryIntent::Index {
                audience: EncyclopediaAudience::Alliance,
                category_command: 0x73,
                selected_object_id: None,
            },
        )
        .unwrap();
        assert_eq!(
            mission_index
                .topics
                .iter()
                .map(|topic| topic.object_id)
                .collect::<Vec<_>>(),
            [0x5100_0010]
        );
    }

    #[test]
    fn entry_intents_preserve_the_exact_non_fixture_return_route() {
        let session = fixture_session();
        let cockpit = EncyclopediaPresenter::present(
            &session,
            EncyclopediaEntryIntent::Cockpit {
                audience: EncyclopediaAudience::Empire,
            },
        )
        .unwrap();
        assert_eq!(
            cockpit.return_route,
            EncyclopediaReturnRoute::Cockpit {
                audience: EncyclopediaAudience::Empire
            }
        );

        let index = EncyclopediaPresenter::present(
            &session,
            EncyclopediaEntryIntent::Index {
                audience: EncyclopediaAudience::Alliance,
                category_command: 0x71,
                selected_object_id: Some(0x1400_0040),
            },
        )
        .unwrap();
        assert_eq!(index.mode, EncyclopediaPresentationMode::Index);
        assert_eq!(index.active_topic, None);
        assert_eq!(index.navigation.previous_object_id, None);
        assert_eq!(index.navigation.next_object_id, None);
        assert_eq!(
            index.return_route,
            EncyclopediaReturnRoute::Index {
                audience: EncyclopediaAudience::Alliance,
                category_command: 0x71,
                selected_object_id: Some(0x1400_0040),
            }
        );

        for caller in [
            EncyclopediaContextCaller::Handler00438800Command67,
            EncyclopediaContextCaller::Handler004443a0Command66,
            EncyclopediaContextCaller::Handler00467f10Command67Or97,
            EncyclopediaContextCaller::MissionDialog0046c3c0Command67,
            EncyclopediaContextCaller::Handler00486fb0Event100,
        ] {
            let contextual = EncyclopediaPresenter::present(
                &session,
                EncyclopediaEntryIntent::Contextual {
                    audience: EncyclopediaAudience::Empire,
                    object_id: 0x5100_0010,
                    caller,
                },
            )
            .unwrap();
            assert_eq!(contextual.mode, EncyclopediaPresentationMode::Topic);
            assert_eq!(
                contextual.return_route,
                EncyclopediaReturnRoute::Contextual {
                    audience: EncyclopediaAudience::Empire,
                    requested_object_id: 0x5100_0010,
                    caller,
                }
            );
            assert!(matches!(
                contextual.active_topic.unwrap().content,
                EncyclopediaTopicContent::Resolved {
                    artwork_resource_id: 0x2c50,
                    artwork_filename: "EDATA.115",
                    ..
                }
            ));
        }
    }

    #[test]
    fn unresolved_context_falls_back_to_index_without_losing_its_caller() {
        let session = fixture_session();
        let caller = EncyclopediaContextCaller::Handler00486fb0Event100;

        let view = EncyclopediaPresenter::present(
            &session,
            EncyclopediaEntryIntent::Contextual {
                audience: EncyclopediaAudience::Alliance,
                object_id: 0x1400_dead,
                caller,
            },
        )
        .unwrap();

        assert_eq!(view.mode, EncyclopediaPresentationMode::Index);
        assert_eq!(view.category.command_id, 0x6f);
        assert_eq!(view.navigation.selected_object_id, None);
        assert_eq!(view.active_topic, None);
        assert_eq!(
            view.return_route,
            EncyclopediaReturnRoute::Contextual {
                audience: EncyclopediaAudience::Alliance,
                requested_object_id: 0x1400_dead,
                caller,
            }
        );
    }

    #[test]
    fn follow_up_navigation_keeps_the_origin_route_across_complete_journeys() {
        let session = fixture_session();
        let caller = EncyclopediaContextCaller::MissionDialog0046c3c0Command67;
        let contextual_route = EncyclopediaReturnRoute::Contextual {
            audience: EncyclopediaAudience::Alliance,
            requested_object_id: 0x1400_dead,
            caller,
        };

        let contextual_miss = EncyclopediaPresenter::present(
            &session,
            EncyclopediaEntryIntent::Contextual {
                audience: EncyclopediaAudience::Alliance,
                object_id: 0x1400_dead,
                caller,
            },
        )
        .unwrap();
        let selected_from_fallback = EncyclopediaPresenter::present(
            &session,
            contextual_miss.follow_up(EncyclopediaPresentationMode::Topic, 0x6f, Some(0x5100_0010)),
        )
        .unwrap();
        let selected_next = EncyclopediaPresenter::present(
            &session,
            selected_from_fallback.follow_up(
                EncyclopediaPresentationMode::Topic,
                0x6f,
                selected_from_fallback.navigation.next_object_id,
            ),
        )
        .unwrap();
        assert_eq!(selected_from_fallback.return_route, contextual_route);
        assert_eq!(selected_next.return_route, contextual_route);

        let contextual_topic = EncyclopediaPresenter::present(
            &session,
            EncyclopediaEntryIntent::Contextual {
                audience: EncyclopediaAudience::Empire,
                object_id: 0x5100_0010,
                caller,
            },
        )
        .unwrap();
        let next = EncyclopediaPresenter::present(
            &session,
            contextual_topic.follow_up(
                EncyclopediaPresentationMode::Topic,
                0x6f,
                contextual_topic.navigation.next_object_id,
            ),
        )
        .unwrap();
        let previous = EncyclopediaPresenter::present(
            &session,
            next.follow_up(
                EncyclopediaPresentationMode::Topic,
                0x6f,
                next.navigation.previous_object_id,
            ),
        )
        .unwrap();
        assert_eq!(next.return_route, contextual_topic.return_route);
        assert_eq!(previous.return_route, contextual_topic.return_route);

        let cockpit = EncyclopediaPresenter::present(
            &session,
            EncyclopediaEntryIntent::Cockpit {
                audience: EncyclopediaAudience::Alliance,
            },
        )
        .unwrap();
        let index = EncyclopediaPresenter::present(
            &session,
            cockpit.follow_up(EncyclopediaPresentationMode::Index, 0x71, Some(0x1400_0040)),
        )
        .unwrap();
        let topic = EncyclopediaPresenter::present(
            &session,
            index.follow_up(EncyclopediaPresentationMode::Topic, 0x71, Some(0x1400_0040)),
        )
        .unwrap();
        assert_eq!(index.active_topic, None);
        assert_eq!(index.return_route, cockpit.return_route);
        assert_eq!(topic.return_route, cockpit.return_route);
    }

    #[test]
    fn invalid_internal_selections_fail_closed_instead_of_guessing() {
        let session = fixture_session();

        let cockpit = EncyclopediaPresenter::present(
            &session,
            EncyclopediaEntryIntent::Cockpit {
                audience: EncyclopediaAudience::Alliance,
            },
        )
        .unwrap();
        let missing_topic = EncyclopediaPresenter::present(
            &session,
            cockpit.follow_up(EncyclopediaPresentationMode::Topic, 0x6f, None),
        )
        .unwrap_err();
        assert_eq!(
            missing_topic,
            EncyclopediaPresenterError::MissingTopicSelection
        );
        assert_eq!(
            missing_topic.to_string(),
            "Encyclopedia topic mode requires a selected object"
        );

        let unknown_category = EncyclopediaPresenter::present(
            &session,
            EncyclopediaEntryIntent::Object {
                audience: EncyclopediaAudience::Alliance,
                category_command: 0xffff,
                object_id: 0x1400_0040,
            },
        )
        .unwrap_err();
        assert_eq!(
            unknown_category,
            EncyclopediaPresenterError::UnknownCategory(0xffff)
        );
        assert_eq!(
            unknown_category.to_string(),
            "unknown Encyclopedia category 0xffff"
        );

        let outside_category = EncyclopediaPresenter::present(
            &session,
            EncyclopediaEntryIntent::Object {
                audience: EncyclopediaAudience::Alliance,
                category_command: 0x70,
                object_id: 0x1400_0040,
            },
        )
        .unwrap_err();
        assert_eq!(
            outside_category,
            EncyclopediaPresenterError::ObjectOutsideCategory {
                object_id: 0x1400_0040,
                category_command: 0x70,
            }
        );
        assert_eq!(
            outside_category.to_string(),
            "Encyclopedia object 0x14000040 is outside category 0x70"
        );

        let unknown_object = EncyclopediaPresenter::present(
            &session,
            EncyclopediaEntryIntent::Object {
                audience: EncyclopediaAudience::Alliance,
                category_command: 0x6f,
                object_id: 0x1400_dead,
            },
        )
        .unwrap_err();
        assert_eq!(
            unknown_object,
            EncyclopediaPresenterError::UnknownObject(0x1400_dead)
        );
        assert_eq!(
            unknown_object.to_string(),
            "unknown Encyclopedia object 0x1400dead"
        );
    }
}
