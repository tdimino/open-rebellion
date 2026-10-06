//! Application adapter between validated Encyclopedia data and render DTOs.

use rebellion_data::encyclopedia_presenter::{
    EncyclopediaPresentation, EncyclopediaPresentationMode, EncyclopediaTopicAvailability,
    EncyclopediaTopicContent,
};
use rebellion_data::encyclopedia_session::EncyclopediaSession;
use rebellion_data::encyclopedia_topics::EncyclopediaAudience;
use rebellion_render::{
    EncyclopediaArtworkView, EncyclopediaSurface, EncyclopediaSurfaceAudience,
    EncyclopediaSurfaceAvailability, EncyclopediaSurfaceCategory, EncyclopediaSurfaceMode,
    EncyclopediaSurfaceNavigation, EncyclopediaSurfaceTopic, EncyclopediaSurfaceTopicItem,
};

/// Mechanically adapt a validated presentation without copying prose or art.
///
/// The renderer intentionally has no dependency on `rebellion-data`; this is
/// the sole W4 seam between its lightweight borrowed DTO and the immutable
/// session/presenter pair.
pub fn adapt_encyclopedia_surface<'a>(
    session: &'a EncyclopediaSession,
    presentation: &EncyclopediaPresentation<'a>,
) -> EncyclopediaSurface<'a> {
    let category =
        |category: &rebellion_data::encyclopedia_presenter::EncyclopediaCategoryView<'a>| {
            EncyclopediaSurfaceCategory {
                command_id: category.command_id,
                label_resource_id: category.label_resource_id,
                label: category.label,
                topic_count: category.topic_count,
            }
        };
    EncyclopediaSurface {
        texture_generation: session.texture_generation(),
        title: presentation.title,
        topic_label: presentation.topic_label,
        audience: match presentation.audience {
            EncyclopediaAudience::Alliance => EncyclopediaSurfaceAudience::Alliance,
            EncyclopediaAudience::Empire => EncyclopediaSurfaceAudience::Empire,
        },
        mode: match presentation.mode {
            EncyclopediaPresentationMode::Index => EncyclopediaSurfaceMode::Index,
            EncyclopediaPresentationMode::Topic => EncyclopediaSurfaceMode::Topic,
        },
        category: category(&presentation.category),
        categories: presentation.categories.iter().map(category).collect(),
        topics: presentation
            .topics
            .iter()
            .map(|topic| EncyclopediaSurfaceTopicItem {
                object_id: topic.object_id,
                title: topic.title,
                availability: match topic.availability {
                    EncyclopediaTopicAvailability::Resolved => {
                        EncyclopediaSurfaceAvailability::Resolved
                    }
                    EncyclopediaTopicAvailability::SourceUnavailable => {
                        EncyclopediaSurfaceAvailability::SourceUnavailable
                    }
                },
            })
            .collect(),
        active_topic: presentation.active_topic.map(|topic| {
            let (description, artwork) = match topic.content {
                EncyclopediaTopicContent::Resolved {
                    description,
                    artwork_resource_id,
                    artwork_filename,
                    artwork_metadata,
                } => (
                    Some(description),
                    Some(EncyclopediaArtworkView {
                        resource_id: artwork_resource_id,
                        filename: artwork_filename,
                        digest: artwork_metadata.sha256(),
                        width: artwork_metadata.width(),
                        height: artwork_metadata.height(),
                        bytes: session
                            .artwork_bytes(artwork_filename)
                            .expect("validated presentation retains exact artwork bytes"),
                    }),
                ),
                EncyclopediaTopicContent::SourceUnavailable => (None, None),
            };
            EncyclopediaSurfaceTopic {
                object_id: topic.object_id,
                title: topic.title,
                topic_text_resource_id: topic.topic_text_resource_id,
                description,
                artwork,
            }
        }),
        navigation: EncyclopediaSurfaceNavigation {
            selected_object_id: presentation.navigation.selected_object_id,
            previous_object_id: presentation.navigation.previous_object_id,
            next_object_id: presentation.navigation.next_object_id,
        },
    }
}

#[cfg(any(test, all(target_arch = "wasm32", feature = "interface-test-fixtures")))]
#[cfg_attr(
    all(test, not(target_arch = "wasm32")),
    allow(
        dead_code,
        reason = "native tests validate fixture inputs; drawing is exercised by the WASM browser gate"
    )
)]
mod fixture {
    use std::collections::{BTreeMap, HashMap};
    use std::sync::Arc;

    use egui_macroquad::egui;
    use macroquad::prelude::{screen_height, screen_width};
    use rebellion_data::encyclopedia_catalog::{
        EncyclopediaCatalog, EncyclopediaCatalogEntry, EncyclopediaCategory,
    };
    use rebellion_data::encyclopedia_presenter::{
        EncyclopediaEntryIntent, EncyclopediaPresentationMode, EncyclopediaPresenter,
    };
    use rebellion_data::encyclopedia_session::{
        EncyclopediaResourceBytes, EncyclopediaSession, EncyclopediaSessionInput,
        EncyclopediaSessionStore,
    };
    use rebellion_data::encyclopedia_topics::EncyclopediaAudience;
    use rebellion_render::{
        draw_encyclopedia_surface, BmpCache, EncyclopediaSurfaceAction, EncyclopediaSurfaceState,
    };

    use super::adapt_encyclopedia_surface;

    const SOURCE: &[u8] = include_bytes!("../../../tests/fixtures/encyclopedia/w4/source.json");
    const MANIFEST: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/w4/source.json.manifest.json");
    const SYSTEM_ID: u32 = 0x9200_0064;
    const MISSION_ID: u32 = 0x5100_0010;
    const SHIP_ID: u32 = 0x1400_0040;
    const SOURCE_UNAVAILABLE_ID: u32 = 0x4100_0001;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum FixtureStart {
        Index,
        FirstTopic,
        MiddleTopic,
        SourceUnavailableTopic,
    }

    pub struct EncyclopediaSurfaceFixture {
        session: Arc<EncyclopediaSession>,
        intent: EncyclopediaEntryIntent,
        surface_state: EncyclopediaSurfaceState,
        open: bool,
    }

    impl EncyclopediaSurfaceFixture {
        pub fn new(audience: EncyclopediaAudience, start: FixtureStart) -> Result<Self, String> {
            let session = fixture_session()?;
            let intent = match start {
                FixtureStart::Index => EncyclopediaEntryIntent::Cockpit { audience },
                FixtureStart::FirstTopic => EncyclopediaEntryIntent::Object {
                    audience,
                    category_command: 0x6f,
                    object_id: SYSTEM_ID,
                },
                FixtureStart::MiddleTopic => EncyclopediaEntryIntent::Object {
                    audience,
                    category_command: 0x6f,
                    object_id: MISSION_ID,
                },
                FixtureStart::SourceUnavailableTopic => EncyclopediaEntryIntent::Object {
                    audience,
                    category_command: 0x6f,
                    object_id: SOURCE_UNAVAILABLE_ID,
                },
            };
            Ok(Self {
                session,
                intent,
                surface_state: EncyclopediaSurfaceState::new(),
                open: true,
            })
        }

        pub fn draw(&mut self, ctx: &egui::Context, cache: &mut BmpCache) {
            if !self.open {
                return;
            }
            let presentation = EncyclopediaPresenter::present(&self.session, self.intent)
                .expect("validated W4 fixture intent remains presentable");
            let surface = adapt_encyclopedia_surface(&self.session, &presentation);
            let scale = (screen_width() / 640.0)
                .min(screen_height() / 480.0)
                .max(f32::EPSILON);
            let origin = egui::pos2(
                (screen_width() - 640.0 * scale) / 2.0 + 85.0 * scale,
                (screen_height() - 480.0 * scale) / 2.0 + 55.0 * scale,
            );
            let action = draw_encyclopedia_surface(
                ctx,
                cache,
                origin,
                scale,
                &mut self.surface_state,
                &surface,
            );
            self.intent = match action {
                Some(EncyclopediaSurfaceAction::SelectCategory(command)) => {
                    presentation.follow_up(EncyclopediaPresentationMode::Index, command, None)
                }
                Some(EncyclopediaSurfaceAction::SelectTopic(object_id)) => presentation.follow_up(
                    EncyclopediaPresentationMode::Index,
                    presentation.category.command_id,
                    Some(object_id),
                ),
                Some(EncyclopediaSurfaceAction::OpenTopic(object_id)) => presentation.follow_up(
                    EncyclopediaPresentationMode::Topic,
                    presentation.category.command_id,
                    Some(object_id),
                ),
                Some(EncyclopediaSurfaceAction::ShowIndex) => presentation.follow_up(
                    EncyclopediaPresentationMode::Index,
                    presentation.category.command_id,
                    presentation.navigation.selected_object_id,
                ),
                Some(EncyclopediaSurfaceAction::Close) => {
                    self.open = false;
                    self.intent
                }
                None => self.intent,
            };
        }
    }

    fn fixture_session() -> Result<Arc<EncyclopediaSession>, String> {
        let category = |command_id: u16,
                        label_resource_id: u16,
                        label: &str,
                        family_range: Option<std::ops::Range<u8>>| {
            EncyclopediaCategory::new(command_id, label_resource_id, label.into(), family_range)
        };
        let catalog = EncyclopediaCatalog {
            title: "Galactic Encyclopedia".into(),
            topic_label: "Topic".into(),
            categories: [
                category(0x6f, 0x1850, "All", None),
                category(0x70, 0x1855, "Systems", Some(0x90..0x98)),
                category(0x71, 0x1854, "Ships", Some(0x14..0x20)),
                category(0x72, 0x1852, "Facilities", Some(0x20..0x30)),
                category(0x73, 0x1851, "Missions", Some(0x40..0x80)),
                category(0x74, 0x1856, "Troops", Some(0x10..0x14)),
                category(0x75, 0x1853, "Personnel", Some(0x30..0x40)),
            ],
            entries: vec![
                EncyclopediaCatalogEntry {
                    object_id: SYSTEM_ID,
                    text_resource_id: 0x2e00,
                    name: "Abregado-rae".into(),
                },
                EncyclopediaCatalogEntry {
                    object_id: MISSION_ID,
                    text_resource_id: 0x2c50,
                    name: "Diplomacy".into(),
                },
                EncyclopediaCatalogEntry {
                    object_id: SHIP_ID,
                    text_resource_id: 0x2740,
                    name: "Mon Calamari Cruiser".into(),
                },
                EncyclopediaCatalogEntry {
                    object_id: SOURCE_UNAVAILABLE_ID,
                    text_resource_id: 0x2f00,
                    name: "Source Unavailable".into(),
                },
            ],
        };
        let artwork: EncyclopediaResourceBytes = BTreeMap::from([
            ("EDATA.001".into(), indexed_bmp([26, 48, 82])),
            ("EDATA.014".into(), indexed_bmp([68, 98, 128])),
            ("EDATA.015".into(), indexed_bmp([84, 52, 102])),
            ("EDATA.115".into(), indexed_bmp([96, 32, 28])),
        ]);
        let mut store = EncyclopediaSessionStore::default();
        store
            .replace(EncyclopediaSessionInput {
                catalog,
                source_catalog_bytes: Arc::from(SOURCE),
                source_manifest_bytes: Arc::from(MANIFEST),
                system_pictures: HashMap::from([(SYSTEM_ID, 1)]),
                artwork,
            })
            .map_err(|error| error.to_string())?;
        store
            .current()
            .ok_or_else(|| "W4 fixture session was not published".into())
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

    #[cfg(test)]
    mod tests {
        use rebellion_data::encyclopedia_presenter::{
            EncyclopediaPresentationMode, EncyclopediaPresenter, EncyclopediaTopicContent,
        };

        use super::*;

        #[test]
        fn fixture_starts_cover_index_endpoints_long_prose_and_source_unavailable() {
            for audience in [EncyclopediaAudience::Alliance, EncyclopediaAudience::Empire] {
                let index = EncyclopediaSurfaceFixture::new(audience, FixtureStart::Index).unwrap();
                let view = EncyclopediaPresenter::present(&index.session, index.intent).unwrap();
                assert_eq!(view.mode, EncyclopediaPresentationMode::Index);
                assert_eq!(view.topics.len(), 4);

                let first =
                    EncyclopediaSurfaceFixture::new(audience, FixtureStart::FirstTopic).unwrap();
                let view = EncyclopediaPresenter::present(&first.session, first.intent).unwrap();
                assert_eq!(view.navigation.previous_object_id, None);
                assert!(view.navigation.next_object_id.is_some());

                let middle =
                    EncyclopediaSurfaceFixture::new(audience, FixtureStart::MiddleTopic).unwrap();
                let view = EncyclopediaPresenter::present(&middle.session, middle.intent).unwrap();
                assert!(view.navigation.previous_object_id.is_some());
                assert!(view.navigation.next_object_id.is_some());
                assert!(matches!(
                    view.active_topic.unwrap().content,
                    EncyclopediaTopicContent::Resolved { description, .. }
                        if description.len() > 200
                ));

                let unavailable =
                    EncyclopediaSurfaceFixture::new(audience, FixtureStart::SourceUnavailableTopic)
                        .unwrap();
                let view = EncyclopediaPresenter::present(&unavailable.session, unavailable.intent)
                    .unwrap();
                assert_eq!(view.navigation.next_object_id, None);
                assert_eq!(
                    view.active_topic.unwrap().content,
                    EncyclopediaTopicContent::SourceUnavailable
                );
            }
        }
    }
}

#[cfg(all(
    test,
    not(all(target_arch = "wasm32", feature = "interface-test-fixtures"))
))]
pub use fixture::FixtureStart;
#[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
pub use fixture::{EncyclopediaSurfaceFixture, FixtureStart};

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};
    use std::sync::Arc;

    use rebellion_data::encyclopedia_catalog::{
        EncyclopediaCatalog, EncyclopediaCatalogEntry, EncyclopediaCategory,
    };
    use rebellion_data::encyclopedia_presenter::{EncyclopediaEntryIntent, EncyclopediaPresenter};
    use rebellion_data::encyclopedia_session::{
        EncyclopediaResourceBytes, EncyclopediaSessionInput, EncyclopediaSessionStore,
    };
    use rebellion_data::encyclopedia_topics::EncyclopediaAudience;
    use rebellion_render::{
        EncyclopediaSurfaceAudience, EncyclopediaSurfaceAvailability, EncyclopediaSurfaceMode,
    };

    use super::adapt_encyclopedia_surface;

    const SOURCE: &[u8] = include_bytes!("../../../tests/fixtures/encyclopedia/p66a/source.json");
    const MANIFEST: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/p66a/source.json.manifest.json");

    fn category(
        command_id: u16,
        label_resource_id: u16,
        family_range: Option<std::ops::Range<u8>>,
    ) -> EncyclopediaCategory {
        EncyclopediaCategory::new(
            command_id,
            label_resource_id,
            format!("Category {command_id}"),
            family_range,
        )
    }

    fn catalog() -> EncyclopediaCatalog {
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
                    object_id: 0x5100_0010,
                    text_resource_id: 0x2c50,
                    name: "Diplomacy".into(),
                },
                EncyclopediaCatalogEntry {
                    object_id: 0x4100_0001,
                    text_resource_id: 0x2f00,
                    name: "Source Empty".into(),
                },
            ],
        }
    }

    fn indexed_bmp() -> Arc<[u8]> {
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
        bytes[58..62].copy_from_slice(&[17, 16, 15, 0]);
        bytes.resize(FILE_BYTES as usize, 1);
        bytes.into()
    }

    fn session() -> Arc<rebellion_data::encyclopedia_session::EncyclopediaSession> {
        let artwork: EncyclopediaResourceBytes = BTreeMap::from([
            ("EDATA.001".into(), indexed_bmp()),
            ("EDATA.014".into(), indexed_bmp()),
            ("EDATA.015".into(), indexed_bmp()),
            ("EDATA.115".into(), indexed_bmp()),
        ]);
        let mut store = EncyclopediaSessionStore::default();
        store
            .replace(EncyclopediaSessionInput {
                catalog: catalog(),
                source_catalog_bytes: Arc::from(SOURCE),
                source_manifest_bytes: Arc::from(MANIFEST),
                system_pictures: HashMap::new(),
                artwork,
            })
            .unwrap();
        store.current().unwrap()
    }

    #[test]
    fn adapter_maps_exact_session_identity_and_resolved_topic_without_copying_bytes() {
        let session = session();
        let presentation = EncyclopediaPresenter::present(
            &session,
            EncyclopediaEntryIntent::Object {
                audience: EncyclopediaAudience::Empire,
                category_command: 0x6f,
                object_id: 0x5100_0010,
            },
        )
        .unwrap();

        let surface = adapt_encyclopedia_surface(&session, &presentation);

        assert_eq!(surface.texture_generation, session.texture_generation());
        assert_eq!(surface.audience, EncyclopediaSurfaceAudience::Empire);
        assert_eq!(surface.mode, EncyclopediaSurfaceMode::Topic);
        assert_eq!(surface.category.command_id, 0x6f);
        assert_eq!(surface.categories.len(), 7);
        assert_eq!(surface.topics.len(), 2);
        assert_eq!(
            surface.topics[1].availability,
            EncyclopediaSurfaceAvailability::SourceUnavailable
        );
        let topic = surface.active_topic.unwrap();
        assert_eq!(topic.title, "Diplomacy");
        assert_eq!(topic.description, Some("Synthetic mission body."));
        let artwork = topic.artwork.unwrap();
        assert_eq!(artwork.resource_id, 0x2c50);
        assert_eq!(artwork.filename, "EDATA.115");
        assert_eq!((artwork.width, artwork.height), (400, 200));
        assert!(std::ptr::eq(
            artwork.bytes.as_ptr(),
            session.artwork_bytes("EDATA.115").unwrap().as_ptr()
        ));
        assert_eq!(artwork.digest.len(), 64);
    }

    #[test]
    fn adapter_keeps_source_unavailable_topic_visibly_empty() {
        let session = session();
        let presentation = EncyclopediaPresenter::present(
            &session,
            EncyclopediaEntryIntent::Object {
                audience: EncyclopediaAudience::Alliance,
                category_command: 0x6f,
                object_id: 0x4100_0001,
            },
        )
        .unwrap();

        let topic = adapt_encyclopedia_surface(&session, &presentation)
            .active_topic
            .unwrap();

        assert_eq!(topic.title, "Source Empty");
        assert_eq!(topic.description, None);
        assert_eq!(topic.artwork, None);
    }
}
