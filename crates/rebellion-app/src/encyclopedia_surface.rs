//! Application adapter between validated Encyclopedia data and render DTOs.

use egui_macroquad::egui;
use macroquad::prelude::{screen_height, screen_width};
use rebellion_data::encyclopedia_presenter::{
    EncyclopediaEntryIntent, EncyclopediaPresentation, EncyclopediaPresentationMode,
    EncyclopediaPresenter, EncyclopediaPresenterError, EncyclopediaReturnRoute,
    EncyclopediaTopicAvailability, EncyclopediaTopicContent,
};
use rebellion_data::encyclopedia_session::EncyclopediaSession;
use rebellion_data::encyclopedia_topics::EncyclopediaAudience;
use rebellion_render::{
    draw_encyclopedia_surface, BmpCache, EncyclopediaArtworkView, EncyclopediaSurface,
    EncyclopediaSurfaceAction, EncyclopediaSurfaceAudience, EncyclopediaSurfaceAvailability,
    EncyclopediaSurfaceCategory, EncyclopediaSurfaceMode, EncyclopediaSurfaceNavigation,
    EncyclopediaSurfaceState, EncyclopediaSurfaceTopic, EncyclopediaSurfaceTopicItem,
    EncyclopediaTextureSampling,
};

use crate::encyclopedia_hd::PreparedEncyclopediaHd;

fn encyclopedia_surface_origin(
    width: f32,
    height: f32,
    scale: f32,
    audience: EncyclopediaSurfaceAudience,
) -> egui::Pos2 {
    let (native_x, native_y) = match audience {
        EncyclopediaSurfaceAudience::Alliance => (62.0, 50.0),
        EncyclopediaSurfaceAudience::Empire => (125.0, 52.0),
    };
    egui::pos2(
        (width - 640.0 * scale) / 2.0 + native_x * scale,
        (height - 480.0 * scale) / 2.0 + native_y * scale,
    )
}

/// Mechanically adapt a validated presentation without copying prose or art.
///
/// The renderer intentionally has no dependency on `rebellion-data`; this is
/// the sole W4 seam between its lightweight borrowed DTO and the immutable
/// session/presenter pair.
#[cfg_attr(
    not(any(test, all(target_arch = "wasm32", feature = "interface-test-fixtures"))),
    allow(
        dead_code,
        reason = "W5 installs production sessions while E32 retains the fail-closed route gate."
    )
)]
pub fn adapt_encyclopedia_surface<'a>(
    session: &'a EncyclopediaSession,
    presentation: &EncyclopediaPresentation<'a>,
) -> EncyclopediaSurface<'a> {
    adapt_encyclopedia_surface_inner(session, presentation, None)
}

/// Adapt through one native profile snapshot prepared outside the frame loop.
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "E32 owns the production route that consumes W6 selection."
    )
)]
pub(crate) fn adapt_encyclopedia_surface_with_hd<'a>(
    session: &'a EncyclopediaSession,
    presentation: &EncyclopediaPresentation<'a>,
    hd: &'a PreparedEncyclopediaHd,
) -> EncyclopediaSurface<'a> {
    adapt_encyclopedia_surface_inner(session, presentation, Some(hd))
}

fn adapt_encyclopedia_surface_inner<'a>(
    session: &'a EncyclopediaSession,
    presentation: &EncyclopediaPresentation<'a>,
    hd: Option<&'a PreparedEncyclopediaHd>,
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
                } => {
                    let original = EncyclopediaArtworkView {
                        resource_id: artwork_resource_id,
                        filename: artwork_filename,
                        digest: artwork_metadata.sha256(),
                        width: artwork_metadata.width(),
                        height: artwork_metadata.height(),
                        bytes: session
                            .artwork_bytes(artwork_filename)
                            .expect("validated presentation retains exact artwork bytes"),
                        sampling: EncyclopediaTextureSampling::Nearest,
                    };
                    (
                        Some(description),
                        Some(hd.map_or(original, |prepared| prepared.select(original))),
                    )
                }
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

/// Production owner for one source-backed Encyclopedia journey.
///
/// The immutable origin survives every internal index/topic transition so a
/// close returns to the exact caller. If a native overlay replacement removes
/// the selected topic, the controller rebinds from that origin against the new
/// session instead of retaining a stale selection.
pub(crate) struct EncyclopediaSurfaceController {
    origin: Option<EncyclopediaEntryIntent>,
    current: Option<EncyclopediaEntryIntent>,
    session_generation: Option<u64>,
    surface_state: EncyclopediaSurfaceState,
}

impl Default for EncyclopediaSurfaceController {
    fn default() -> Self {
        Self::new()
    }
}

impl EncyclopediaSurfaceController {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {
            origin: None,
            current: None,
            session_generation: None,
            surface_state: EncyclopediaSurfaceState::new(),
        }
    }

    #[must_use]
    pub(crate) const fn is_open(&self) -> bool {
        self.current.is_some()
    }

    /// Validate and open one production entry intent.
    pub(crate) fn open(
        &mut self,
        session: &EncyclopediaSession,
        intent: EncyclopediaEntryIntent,
    ) -> Result<(), EncyclopediaPresenterError> {
        EncyclopediaPresenter::present(session, intent)?;
        self.origin = Some(intent);
        self.current = Some(intent);
        self.session_generation = Some(session.texture_generation());
        self.surface_state = EncyclopediaSurfaceState::new();
        Ok(())
    }

    pub(crate) fn close(&mut self) {
        self.origin = None;
        self.current = None;
        self.session_generation = None;
        self.surface_state = EncyclopediaSurfaceState::new();
    }

    pub(crate) fn presentation<'a>(
        &mut self,
        session: &'a EncyclopediaSession,
    ) -> Result<EncyclopediaPresentation<'a>, EncyclopediaPresenterError> {
        let current = self
            .current
            .expect("an open Encyclopedia controller has a current intent");
        if self.session_generation != Some(session.texture_generation()) {
            if EncyclopediaPresenter::present(session, current).is_err() {
                self.current = self.origin;
            }
            self.session_generation = Some(session.texture_generation());
        }
        EncyclopediaPresenter::present(
            session,
            self.current
                .expect("an open Encyclopedia controller retains its origin"),
        )
    }

    /// Apply one renderer action. A returned route means the viewer closed.
    pub(crate) fn apply_action(
        &mut self,
        session: &EncyclopediaSession,
        action: EncyclopediaSurfaceAction,
    ) -> Result<Option<EncyclopediaReturnRoute>, EncyclopediaPresenterError> {
        let presentation = self.presentation(session)?;
        let return_route = presentation.return_route;
        let next = match action {
            EncyclopediaSurfaceAction::SelectCategory(command) => {
                Some(presentation.follow_up(EncyclopediaPresentationMode::Index, command, None))
            }
            EncyclopediaSurfaceAction::SelectTopic(object_id) => Some(presentation.follow_up(
                EncyclopediaPresentationMode::Index,
                presentation.category.command_id,
                Some(object_id),
            )),
            EncyclopediaSurfaceAction::ClearTopicSelection => Some(presentation.follow_up(
                EncyclopediaPresentationMode::Index,
                presentation.category.command_id,
                None,
            )),
            EncyclopediaSurfaceAction::OpenTopic(object_id) => Some(presentation.follow_up(
                EncyclopediaPresentationMode::Topic,
                presentation.category.command_id,
                Some(object_id),
            )),
            EncyclopediaSurfaceAction::ShowIndex => Some(presentation.follow_up(
                EncyclopediaPresentationMode::Index,
                presentation.category.command_id,
                presentation.navigation.selected_object_id,
            )),
            EncyclopediaSurfaceAction::Close => None,
        };
        if let Some(next) = next {
            EncyclopediaPresenter::present(session, next)?;
            self.current = Some(next);
            Ok(None)
        } else {
            self.close();
            Ok(Some(return_route))
        }
    }

    /// Draw one production frame through the same presenter and renderer used
    /// by the accepted canonical fixture route.
    pub(crate) fn draw(
        &mut self,
        ctx: &egui::Context,
        cache: &mut BmpCache,
        session: &EncyclopediaSession,
        hd: &PreparedEncyclopediaHd,
    ) -> Result<Option<EncyclopediaReturnRoute>, EncyclopediaPresenterError> {
        if !self.is_open() {
            return Ok(None);
        }
        let presentation = self.presentation(session)?;
        let surface = adapt_encyclopedia_surface_with_hd(session, &presentation, hd);
        let scale = (screen_width() / 640.0)
            .min(screen_height() / 480.0)
            .max(f32::EPSILON);
        let origin =
            encyclopedia_surface_origin(screen_width(), screen_height(), scale, surface.audience);
        let action =
            draw_encyclopedia_surface(ctx, cache, origin, scale, &mut self.surface_state, &surface);
        match action {
            Some(action) => self.apply_action(session, action),
            None => Ok(None),
        }
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
        EncyclopediaContextCaller, EncyclopediaEntryIntent, EncyclopediaPresentationMode,
        EncyclopediaPresenter, EncyclopediaReturnRoute, EncyclopediaTopicAvailability,
        EncyclopediaTopicContent,
    };
    use rebellion_data::encyclopedia_session::{
        EncyclopediaResourceBytes, EncyclopediaSession, EncyclopediaSessionInput,
        EncyclopediaSessionStore,
    };
    use rebellion_data::encyclopedia_topics::EncyclopediaAudience;
    use rebellion_render::{
        draw_encyclopedia_surface, BmpCache, EncyclopediaSurfaceAction, EncyclopediaSurfaceState,
    };
    use serde::Serialize;

    use super::{adapt_encyclopedia_surface, encyclopedia_surface_origin};

    const SOURCE: &[u8] = include_bytes!("../../../tests/fixtures/encyclopedia/w4/source.json");
    const MANIFEST: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/w4/source.json.manifest.json");
    const SYSTEM_ID: u32 = 0x9200_0064;
    const MISSION_ID: u32 = 0x5100_0010;
    const SHIP_ID: u32 = 0x1400_0040;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum FixtureStart {
        Index,
        FirstTopic,
        MiddleTopic,
    }

    /// Canonical fixture entry points selected from the installed session.
    ///
    /// Unlike [`FixtureStart`], these never name an object from the synthetic
    /// W4 corpus. This keeps E30's native and packaged-browser journeys tied to
    /// the exact catalog that crossed the W2 publication boundary.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CanonicalFixtureStart {
        Index,
        FirstTopic,
        LastTopic,
        LongestResolvedTopic,
        ContextualFirstTopic,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct EncyclopediaCategoryObservation {
        pub command_id: u16,
        pub label_resource_id: u16,
        pub topic_count: usize,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct EncyclopediaArtworkObservation {
        pub resource_id: u16,
        pub filename: String,
        pub sha256: String,
        pub width: u32,
        pub height: u32,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct EncyclopediaReturnObservation {
        pub kind: &'static str,
        pub audience: &'static str,
        pub category_command: Option<u16>,
        pub selected_object_id: Option<u32>,
        pub requested_object_id: Option<u32>,
        pub caller: Option<&'static str>,
    }

    /// Fixture-only, prose-free proof of one canonical rendered state.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct EncyclopediaSurfaceObservation {
        pub schema_version: u32,
        pub status: &'static str,
        pub code: u32,
        pub open: bool,
        pub audience: &'static str,
        pub mode: &'static str,
        pub category_command: u16,
        pub category_label_resource_id: u16,
        pub categories: Vec<EncyclopediaCategoryObservation>,
        pub topic_count: usize,
        pub resolved_count: usize,
        pub source_unavailable_count: usize,
        pub selected_object_id: Option<u32>,
        pub previous_object_id: Option<u32>,
        pub next_object_id: Option<u32>,
        pub active_object_id: Option<u32>,
        pub topic_text_resource_id: Option<u16>,
        pub availability: Option<&'static str>,
        pub body_utf8_bytes: Option<usize>,
        pub artwork: Option<EncyclopediaArtworkObservation>,
        pub return_route: EncyclopediaReturnObservation,
        pub logical_fingerprint: String,
        pub texture_generation: u64,
        pub index_scroll_row: usize,
    }

    pub struct EncyclopediaSurfaceFixture {
        session: Arc<EncyclopediaSession>,
        intent: EncyclopediaEntryIntent,
        surface_state: EncyclopediaSurfaceState,
        open: bool,
        #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
        last_observation: Option<EncyclopediaSurfaceObservation>,
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
            };
            Ok(Self::from_intent(session, intent))
        }

        pub fn from_session(
            session: Arc<EncyclopediaSession>,
            audience: EncyclopediaAudience,
            start: CanonicalFixtureStart,
        ) -> Result<Self, String> {
            let all = 0x6f;
            let intent = match start {
                CanonicalFixtureStart::Index => EncyclopediaEntryIntent::Cockpit { audience },
                CanonicalFixtureStart::FirstTopic => EncyclopediaEntryIntent::Object {
                    audience,
                    category_command: all,
                    object_id: session
                        .catalog()
                        .entries
                        .first()
                        .ok_or_else(|| "canonical Encyclopedia catalog is empty".to_owned())?
                        .object_id,
                },
                CanonicalFixtureStart::LastTopic => EncyclopediaEntryIntent::Object {
                    audience,
                    category_command: all,
                    object_id: session
                        .catalog()
                        .entries
                        .last()
                        .ok_or_else(|| "canonical Encyclopedia catalog is empty".to_owned())?
                        .object_id,
                },
                CanonicalFixtureStart::LongestResolvedTopic => EncyclopediaEntryIntent::Object {
                    audience,
                    category_command: all,
                    object_id: longest_resolved_object_id(&session, audience).ok_or_else(|| {
                        "canonical Encyclopedia catalog has no resolved topic".to_owned()
                    })?,
                },
                CanonicalFixtureStart::ContextualFirstTopic => {
                    EncyclopediaEntryIntent::Contextual {
                        audience,
                        object_id: session
                            .catalog()
                            .entries
                            .first()
                            .ok_or_else(|| "canonical Encyclopedia catalog is empty".to_owned())?
                            .object_id,
                        caller: EncyclopediaContextCaller::Handler00438800Command67,
                    }
                }
            };
            EncyclopediaPresenter::present(&session, intent)
                .map_err(|error| format!("canonical Encyclopedia start is invalid: {error}"))?;
            Ok(Self::from_intent(session, intent))
        }

        fn from_intent(session: Arc<EncyclopediaSession>, intent: EncyclopediaEntryIntent) -> Self {
            Self {
                session,
                intent,
                surface_state: EncyclopediaSurfaceState::new(),
                open: true,
                #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
                last_observation: None,
            }
        }

        pub fn observation(&self, code: u32) -> Result<EncyclopediaSurfaceObservation, String> {
            let presentation =
                EncyclopediaPresenter::present(&self.session, self.intent).map_err(|error| {
                    format!("canonical Encyclopedia observation is invalid: {error}")
                })?;
            let (active_object_id, topic_text_resource_id, availability, body_utf8_bytes, artwork) =
                presentation.active_topic.map_or(
                    (None, None, None, None, None),
                    |topic| match topic.content {
                        EncyclopediaTopicContent::Resolved {
                            description,
                            artwork_resource_id,
                            artwork_filename,
                            artwork_metadata,
                        } => (
                            Some(topic.object_id),
                            Some(topic.topic_text_resource_id),
                            Some("resolved"),
                            Some(description.len()),
                            Some(EncyclopediaArtworkObservation {
                                resource_id: artwork_resource_id,
                                filename: artwork_filename.to_owned(),
                                sha256: artwork_metadata.sha256().to_owned(),
                                width: artwork_metadata.width(),
                                height: artwork_metadata.height(),
                            }),
                        ),
                        EncyclopediaTopicContent::SourceUnavailable => (
                            Some(topic.object_id),
                            Some(topic.topic_text_resource_id),
                            Some("source-unavailable"),
                            None,
                            None,
                        ),
                    },
                );
            let resolved_count = presentation
                .topics
                .iter()
                .filter(|topic| topic.availability == EncyclopediaTopicAvailability::Resolved)
                .count();
            let source_unavailable_count = presentation
                .topics
                .iter()
                .filter(|topic| {
                    topic.availability == EncyclopediaTopicAvailability::SourceUnavailable
                })
                .count();
            Ok(EncyclopediaSurfaceObservation {
                schema_version: 1,
                status: "encyclopedia-surface",
                code,
                open: self.open,
                audience: match presentation.audience {
                    EncyclopediaAudience::Alliance => "alliance",
                    EncyclopediaAudience::Empire => "empire",
                },
                mode: match presentation.mode {
                    EncyclopediaPresentationMode::Index => "index",
                    EncyclopediaPresentationMode::Topic => "topic",
                },
                category_command: presentation.category.command_id,
                category_label_resource_id: presentation.category.label_resource_id,
                categories: presentation
                    .categories
                    .iter()
                    .map(|category| EncyclopediaCategoryObservation {
                        command_id: category.command_id,
                        label_resource_id: category.label_resource_id,
                        topic_count: category.topic_count,
                    })
                    .collect(),
                topic_count: presentation.topics.len(),
                resolved_count,
                source_unavailable_count,
                selected_object_id: presentation.navigation.selected_object_id,
                previous_object_id: presentation.navigation.previous_object_id,
                next_object_id: presentation.navigation.next_object_id,
                active_object_id,
                topic_text_resource_id,
                availability,
                body_utf8_bytes,
                artwork,
                return_route: observe_return_route(presentation.return_route),
                logical_fingerprint: self.session.logical_fingerprint().to_owned(),
                texture_generation: self.session.texture_generation(),
                index_scroll_row: self.surface_state.scroll_row,
            })
        }

        pub fn draw(&mut self, ctx: &egui::Context, cache: &mut BmpCache, _code: u32) {
            if !self.open {
                return;
            }
            let presentation = EncyclopediaPresenter::present(&self.session, self.intent)
                .expect("validated W4 fixture intent remains presentable");
            let surface = adapt_encyclopedia_surface(&self.session, &presentation);
            let scale = (screen_width() / 640.0)
                .min(screen_height() / 480.0)
                .max(f32::EPSILON);
            let origin = encyclopedia_surface_origin(
                screen_width(),
                screen_height(),
                scale,
                surface.audience,
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
                Some(EncyclopediaSurfaceAction::ClearTopicSelection) => presentation.follow_up(
                    EncyclopediaPresentationMode::Index,
                    presentation.category.command_id,
                    None,
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
            #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
            self.emit_changed_observation(_code);
        }

        #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
        fn emit_changed_observation(&mut self, code: u32) {
            let observation = self
                .observation(code)
                .expect("validated Encyclopedia fixture remains observable");
            if self.last_observation.as_ref() == Some(&observation) {
                return;
            }
            let bytes = serde_json::to_vec(&observation)
                .expect("serialize canonical Encyclopedia surface observation");
            unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
            self.last_observation = Some(observation);
        }
    }

    #[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
    extern "C" {
        fn open_rebellion_interface_fixture_emit(ptr: *const u8, len: usize);
    }

    fn longest_resolved_object_id(
        session: &EncyclopediaSession,
        audience: EncyclopediaAudience,
    ) -> Option<u32> {
        let mut longest = None;
        for entry in &session.topics(audience).entries {
            let Some(body) = entry.body.as_ref().filter(|_| entry.is_complete()) else {
                continue;
            };
            longest = retain_longest(longest, (entry.object_id, body.len()));
        }
        longest.map(|(object_id, _)| object_id)
    }

    fn retain_longest(
        current: Option<(u32, usize)>,
        candidate: (u32, usize),
    ) -> Option<(u32, usize)> {
        if current.is_none_or(|(_, bytes)| candidate.1 > bytes) {
            Some(candidate)
        } else {
            current
        }
    }

    fn observe_return_route(route: EncyclopediaReturnRoute) -> EncyclopediaReturnObservation {
        let audience_name = |audience| match audience {
            EncyclopediaAudience::Alliance => "alliance",
            EncyclopediaAudience::Empire => "empire",
        };
        match route {
            EncyclopediaReturnRoute::Cockpit { audience } => EncyclopediaReturnObservation {
                kind: "cockpit",
                audience: audience_name(audience),
                category_command: None,
                selected_object_id: None,
                requested_object_id: None,
                caller: None,
            },
            EncyclopediaReturnRoute::Index {
                audience,
                category_command,
                selected_object_id,
            } => EncyclopediaReturnObservation {
                kind: "index",
                audience: audience_name(audience),
                category_command: Some(category_command),
                selected_object_id,
                requested_object_id: None,
                caller: None,
            },
            EncyclopediaReturnRoute::Contextual {
                audience,
                requested_object_id,
                caller,
            } => EncyclopediaReturnObservation {
                kind: "contextual",
                audience: audience_name(audience),
                category_command: None,
                selected_object_id: None,
                requested_object_id: Some(requested_object_id),
                caller: Some(match caller {
                    EncyclopediaContextCaller::Handler00438800Command67 => {
                        "handler-00438800-command-67"
                    }
                    EncyclopediaContextCaller::Handler004443a0Command66 => {
                        "handler-004443a0-command-66"
                    }
                    EncyclopediaContextCaller::Handler00467f10Command67Or97 => {
                        "handler-00467f10-command-67-or-97"
                    }
                    EncyclopediaContextCaller::MissionDialog0046c3c0Command67 => {
                        "mission-dialog-0046c3c0-command-67"
                    }
                    EncyclopediaContextCaller::Handler00486fb0Event100 => {
                        "handler-00486fb0-event-100"
                    }
                }),
            },
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
        fn fixture_starts_cover_index_endpoints_and_long_prose() {
            for audience in [EncyclopediaAudience::Alliance, EncyclopediaAudience::Empire] {
                let index = EncyclopediaSurfaceFixture::new(audience, FixtureStart::Index).unwrap();
                let view = EncyclopediaPresenter::present(&index.session, index.intent).unwrap();
                assert_eq!(view.mode, EncyclopediaPresentationMode::Index);
                assert_eq!(view.topics.len(), 3);

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
            }
        }

        #[test]
        fn canonical_fixture_starts_are_derived_from_the_installed_session() {
            let session = fixture_session().unwrap();
            for audience in [EncyclopediaAudience::Alliance, EncyclopediaAudience::Empire] {
                let first = EncyclopediaSurfaceFixture::from_session(
                    Arc::clone(&session),
                    audience,
                    CanonicalFixtureStart::FirstTopic,
                )
                .unwrap();
                let first_view =
                    EncyclopediaPresenter::present(&first.session, first.intent).unwrap();
                assert_eq!(
                    first_view.navigation.selected_object_id,
                    session
                        .catalog()
                        .entries
                        .first()
                        .map(|entry| entry.object_id)
                );
                assert_eq!(first_view.navigation.previous_object_id, None);

                let last = EncyclopediaSurfaceFixture::from_session(
                    Arc::clone(&session),
                    audience,
                    CanonicalFixtureStart::LastTopic,
                )
                .unwrap();
                let last_view = EncyclopediaPresenter::present(&last.session, last.intent).unwrap();
                assert_eq!(
                    last_view.navigation.selected_object_id,
                    session
                        .catalog()
                        .entries
                        .last()
                        .map(|entry| entry.object_id)
                );
                assert_eq!(last_view.navigation.next_object_id, None);

                let longest = EncyclopediaSurfaceFixture::from_session(
                    Arc::clone(&session),
                    audience,
                    CanonicalFixtureStart::LongestResolvedTopic,
                )
                .unwrap();
                let longest_view =
                    EncyclopediaPresenter::present(&longest.session, longest.intent).unwrap();
                let longest_body_bytes = session
                    .topics(audience)
                    .entries
                    .iter()
                    .filter_map(|entry| entry.body.as_ref())
                    .map(String::len)
                    .max()
                    .unwrap();
                assert!(matches!(
                    longest_view.active_topic.unwrap().content,
                    EncyclopediaTopicContent::Resolved { description, .. }
                        if description.len() == longest_body_bytes
                ));

                let contextual = EncyclopediaSurfaceFixture::from_session(
                    Arc::clone(&session),
                    audience,
                    CanonicalFixtureStart::ContextualFirstTopic,
                )
                .unwrap();
                let contextual_view =
                    EncyclopediaPresenter::present(&contextual.session, contextual.intent).unwrap();
                assert!(matches!(
                    contextual_view.return_route,
                    rebellion_data::encyclopedia_presenter::EncyclopediaReturnRoute::Contextual {
                        audience: returned_audience,
                        requested_object_id,
                        caller: rebellion_data::encyclopedia_presenter::EncyclopediaContextCaller::Handler00438800Command67,
                    } if returned_audience == audience
                        && Some(requested_object_id)
                            == session.catalog().entries.first().map(|entry| entry.object_id)
                ));
            }
        }

        #[test]
        fn canonical_observation_reports_identity_without_exporting_owned_prose() {
            let session = fixture_session().unwrap();
            let fixture = EncyclopediaSurfaceFixture::from_session(
                Arc::clone(&session),
                EncyclopediaAudience::Empire,
                CanonicalFixtureStart::LongestResolvedTopic,
            )
            .unwrap();

            let observation = fixture.observation(0x23_003d).unwrap();
            assert_eq!(observation.status, "encyclopedia-surface");
            assert_eq!(observation.code, 0x23_003d);
            assert!(observation.open);
            assert_eq!(observation.audience, "empire");
            assert_eq!(observation.mode, "topic");
            assert_eq!(observation.category_command, 0x6f);
            assert_eq!(observation.categories.len(), 7);
            assert_eq!(observation.topic_count, session.catalog().entries.len());
            assert_eq!(observation.resolved_count, 3);
            assert_eq!(observation.source_unavailable_count, 0);
            assert_eq!(
                observation.logical_fingerprint,
                session.logical_fingerprint()
            );
            assert!(observation.body_utf8_bytes.is_some_and(|bytes| bytes > 200));
            let artwork = observation.artwork.as_ref().unwrap();
            assert_eq!((artwork.width, artwork.height), (400, 200));
            assert_eq!(artwork.sha256.len(), 64);

            let encoded = serde_json::to_string(&observation).unwrap();
            assert!(!encoded.contains("Diplomacy"));
            assert!(!encoded.contains("Synthetic mission body"));

            let contextual = EncyclopediaSurfaceFixture::from_session(
                fixture_session().unwrap(),
                EncyclopediaAudience::Empire,
                CanonicalFixtureStart::ContextualFirstTopic,
            )
            .unwrap();
            let observation = contextual.observation(0x23_003f).unwrap();
            assert_eq!(observation.return_route.kind, "contextual");
            assert_eq!(
                observation.return_route.requested_object_id,
                observation.active_object_id
            );
            assert_eq!(
                observation.return_route.caller,
                Some("handler-00438800-command-67")
            );
        }

        #[test]
        fn canonical_longest_selection_keeps_the_first_topic_on_a_tie() {
            assert_eq!(retain_longest(None, (1, 9)), Some((1, 9)));
            assert_eq!(retain_longest(Some((1, 9)), (2, 10)), Some((2, 10)));
            assert_eq!(retain_longest(Some((1, 10)), (2, 10)), Some((1, 10)));
            assert_eq!(retain_longest(Some((1, 10)), (2, 9)), Some((1, 10)));
        }

        #[test]
        #[ignore = "requires the owned ignored P66A source and EData directory"]
        fn owned_native_session_covers_the_canonical_e30_state_matrix() {
            let source = std::path::PathBuf::from(
                std::env::var_os("REBELLION_ENCYCLOPEDIA_TEST_SOURCE")
                    .expect("set REBELLION_ENCYCLOPEDIA_TEST_SOURCE"),
            );
            let gdata = source
                .parent()
                .and_then(std::path::Path::parent)
                .expect("source path is GData/encyclopedia/source.json");
            let edata = std::path::PathBuf::from(
                std::env::var_os("REBELLION_EDATA_DIR").expect("set REBELLION_EDATA_DIR"),
            );
            let payload = crate::encyclopedia_content::read_native_encyclopedia(gdata, &edata)
                .unwrap()
                .expect("owned native Encyclopedia is published");
            let catalog =
                rebellion_data::encyclopedia_catalog::load_encyclopedia_catalog(gdata).unwrap();
            let system_pictures =
                rebellion_data::encyclopedia_topics::load_encyclopedia_system_pictures(gdata)
                    .unwrap();
            let mut store = EncyclopediaSessionStore::default();
            store
                .replace(payload.into_session_input(catalog, system_pictures))
                .unwrap();
            let session = store.current().unwrap();
            assert_eq!(
                session.logical_fingerprint(),
                "20c342868cee50e80ef3b94b9f81a898c48f4b593ddd0d83ab69b67d755ae9ea"
            );

            for audience in [EncyclopediaAudience::Alliance, EncyclopediaAudience::Empire] {
                let index = EncyclopediaSurfaceFixture::from_session(
                    Arc::clone(&session),
                    audience,
                    CanonicalFixtureStart::Index,
                )
                .unwrap();
                let observation = index.observation(1).unwrap();
                assert_eq!(observation.categories.len(), 7);
                assert_eq!(observation.topic_count, 346);
                assert_eq!(observation.resolved_count, 346);
                assert_eq!(observation.source_unavailable_count, 0);
                for category in &observation.categories {
                    let category_view = EncyclopediaPresenter::present(
                        &session,
                        EncyclopediaEntryIntent::Index {
                            audience,
                            category_command: category.command_id,
                            selected_object_id: None,
                        },
                    )
                    .unwrap();
                    assert_eq!(category_view.category.command_id, category.command_id);
                    assert_eq!(category_view.category.topic_count, category.topic_count);
                }

                for start in [
                    CanonicalFixtureStart::FirstTopic,
                    CanonicalFixtureStart::LastTopic,
                    CanonicalFixtureStart::LongestResolvedTopic,
                    CanonicalFixtureStart::ContextualFirstTopic,
                ] {
                    let fixture = EncyclopediaSurfaceFixture::from_session(
                        Arc::clone(&session),
                        audience,
                        start,
                    )
                    .unwrap();
                    let observation = fixture.observation(1).unwrap();
                    assert_eq!(observation.mode, "topic");
                    assert_eq!(observation.selected_object_id, observation.active_object_id);
                    assert_eq!(observation.availability, Some("resolved"));
                    let artwork = observation.artwork.unwrap();
                    assert_eq!((artwork.width, artwork.height), (400, 200));
                    assert_eq!(artwork.sha256.len(), 64);
                    if start == CanonicalFixtureStart::ContextualFirstTopic {
                        assert_eq!(observation.return_route.kind, "contextual");
                        assert_eq!(
                            observation.return_route.requested_object_id,
                            observation.active_object_id
                        );
                    }
                }
            }
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
pub use fixture::{CanonicalFixtureStart, EncyclopediaSurfaceFixture, FixtureStart};
#[cfg(all(
    test,
    not(all(target_arch = "wasm32", feature = "interface-test-fixtures"))
))]
pub use fixture::{CanonicalFixtureStart, FixtureStart};

#[cfg(test)]
mod tests {
    use egui_macroquad::egui;
    use std::collections::{BTreeMap, HashMap};
    use std::sync::Arc;

    use rebellion_data::encyclopedia_catalog::{
        EncyclopediaCatalog, EncyclopediaCatalogEntry, EncyclopediaCategory,
    };
    use rebellion_data::encyclopedia_presenter::{
        EncyclopediaContextCaller, EncyclopediaEntryIntent, EncyclopediaPresentationMode,
        EncyclopediaPresenter, EncyclopediaReturnRoute,
    };
    use rebellion_data::encyclopedia_session::{
        EncyclopediaResourceBytes, EncyclopediaSessionInput, EncyclopediaSessionStore,
    };
    use rebellion_data::encyclopedia_topics::EncyclopediaAudience;
    use rebellion_render::{EncyclopediaSurfaceAudience, EncyclopediaSurfaceMode};

    use super::{
        adapt_encyclopedia_surface, encyclopedia_surface_origin, EncyclopediaSurfaceController,
    };

    const SOURCE: &[u8] = include_bytes!("../../../tests/fixtures/encyclopedia/p66a/source.json");
    const MANIFEST: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/p66a/source.json.manifest.json");

    #[test]
    fn production_surface_uses_the_original_native_window_origin() {
        assert_eq!(
            encyclopedia_surface_origin(640.0, 480.0, 1.0, EncyclopediaSurfaceAudience::Alliance,),
            egui::pos2(62.0, 50.0)
        );
        assert_eq!(
            encyclopedia_surface_origin(800.0, 600.0, 1.25, EncyclopediaSurfaceAudience::Alliance,),
            egui::pos2(77.5, 62.5)
        );
        assert_eq!(
            encyclopedia_surface_origin(640.0, 480.0, 1.0, EncyclopediaSurfaceAudience::Empire,),
            egui::pos2(125.0, 52.0)
        );
        assert_eq!(
            encyclopedia_surface_origin(800.0, 600.0, 1.25, EncyclopediaSurfaceAudience::Empire,),
            egui::pos2(156.25, 65.0)
        );
        assert_eq!(
            encyclopedia_surface_origin(1024.0, 600.0, 1.25, EncyclopediaSurfaceAudience::Alliance,),
            egui::pos2(189.5, 62.5)
        );
        assert_eq!(
            encyclopedia_surface_origin(800.0, 800.0, 1.25, EncyclopediaSurfaceAudience::Alliance,),
            egui::pos2(77.5, 162.5)
        );
    }

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
            entries: vec![EncyclopediaCatalogEntry {
                object_id: 0x5100_0010,
                text_resource_id: 0x2c50,
                name: "Diplomacy".into(),
            }],
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

    fn session_input() -> EncyclopediaSessionInput {
        let artwork: EncyclopediaResourceBytes = BTreeMap::from([
            ("EDATA.001".into(), indexed_bmp()),
            ("EDATA.014".into(), indexed_bmp()),
            ("EDATA.015".into(), indexed_bmp()),
            ("EDATA.115".into(), indexed_bmp()),
        ]);
        EncyclopediaSessionInput {
            catalog: catalog(),
            source_catalog_bytes: Arc::from(SOURCE),
            source_manifest_bytes: Arc::from(MANIFEST),
            system_pictures: HashMap::new(),
            artwork,
        }
    }

    fn session() -> Arc<rebellion_data::encyclopedia_session::EncyclopediaSession> {
        let mut store = EncyclopediaSessionStore::default();
        store.replace(session_input()).unwrap();
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
        assert_eq!(surface.topics.len(), 1);
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
    fn production_controller_preserves_cockpit_route_through_open_and_close() {
        let session = session();
        let mut controller = EncyclopediaSurfaceController::new();

        controller
            .open(
                &session,
                EncyclopediaEntryIntent::Cockpit {
                    audience: EncyclopediaAudience::Empire,
                },
            )
            .unwrap();

        assert!(controller.is_open());
        let presentation = controller.presentation(&session).unwrap();
        assert_eq!(presentation.mode, EncyclopediaPresentationMode::Index);
        assert_eq!(
            presentation.return_route,
            EncyclopediaReturnRoute::Cockpit {
                audience: EncyclopediaAudience::Empire
            }
        );
        let route = controller
            .apply_action(&session, rebellion_render::EncyclopediaSurfaceAction::Close)
            .unwrap()
            .unwrap();
        assert_eq!(route, presentation.return_route);
        assert!(!controller.is_open());
    }

    #[test]
    fn production_controller_keeps_contextual_origin_across_index_transition() {
        let session = session();
        let caller = EncyclopediaContextCaller::Handler00486fb0Event100;
        let mut controller = EncyclopediaSurfaceController::new();
        controller
            .open(
                &session,
                EncyclopediaEntryIntent::Contextual {
                    audience: EncyclopediaAudience::Alliance,
                    object_id: 0x5100_0010,
                    caller,
                },
            )
            .unwrap();

        controller
            .apply_action(
                &session,
                rebellion_render::EncyclopediaSurfaceAction::ShowIndex,
            )
            .unwrap();
        let presentation = controller.presentation(&session).unwrap();
        assert_eq!(presentation.mode, EncyclopediaPresentationMode::Index);
        assert_eq!(
            presentation.return_route,
            EncyclopediaReturnRoute::Contextual {
                audience: EncyclopediaAudience::Alliance,
                requested_object_id: 0x5100_0010,
                caller,
            }
        );
    }

    #[test]
    fn production_controller_rebinds_to_a_replacement_session_generation() {
        let mut store = EncyclopediaSessionStore::default();
        store.replace(session_input()).unwrap();
        let first = store.current().unwrap();
        let mut controller = EncyclopediaSurfaceController::new();
        controller
            .open(
                &first,
                EncyclopediaEntryIntent::Cockpit {
                    audience: EncyclopediaAudience::Alliance,
                },
            )
            .unwrap();

        let mut replacement = session_input();
        replacement.catalog.title = "Replacement Encyclopedia".into();
        store.replace(replacement).unwrap();
        let second = store.current().unwrap();
        assert_ne!(first.texture_generation(), second.texture_generation());

        let presentation = controller.presentation(&second).unwrap();

        assert_eq!(presentation.title, "Replacement Encyclopedia");
        assert_eq!(
            controller.session_generation,
            Some(second.texture_generation())
        );
    }
}
