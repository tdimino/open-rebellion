//! Feature-gated content inspector for the validated encyclopedia pipeline.
//!
//! This module is compiled only for `interface-test-fixtures`. Its admission
//! snapshot deliberately exposes every validated catalog binding for transport
//! inspection; it is not a gameplay-availability adapter and is never used by
//! the production route.

use std::ffi::OsStr;

use rebellion_data::encyclopedia::{
    AdmissionFact, AdmissionSnapshot, AdmittedBinding, EncyclopediaCatalog, EncyclopediaError,
    ViewerFaction,
};
use rebellion_render::{
    apply_encyclopedia_action, BodyScrollIntent, EncyclopediaAction, EncyclopediaMode,
    EncyclopediaNavigationState, EncyclopediaTextureCache, EncyclopediaTextureEvent,
    EncyclopediaView, NavigationOutcome,
};
#[cfg(test)]
use rebellion_render::{EncyclopediaTextureBackend, TopicImageView};

#[cfg(any(test, target_arch = "wasm32"))]
use crate::interface_test_fixture::FixtureRequest;

use crate::encyclopedia_presenter::{build_encyclopedia_view, EncyclopediaPresenter};
use crate::encyclopedia_session::{EncyclopediaAvailability, EncyclopediaSession};

const INSPECTOR_REQUEST: &str = "REBELLION_ENCYCLOPEDIA_INSPECTOR";
const INSPECTOR_FACTION: &str = "REBELLION_ENCYCLOPEDIA_VIEWER_FACTION";

pub(crate) enum InspectorContent {
    Ready(Box<EncyclopediaSession>),
    Unavailable(String),
}

pub(crate) struct InspectorModel {
    content: InspectorContent,
    presenter: EncyclopediaPresenter,
    admission: Option<AdmissionSnapshot>,
    language: String,
    navigation: EncyclopediaNavigationState,
}

impl InspectorModel {
    pub(crate) fn new(content: InspectorContent, language: impl Into<String>) -> Self {
        Self::new_for_viewer(content, language, ViewerFaction::Alliance)
    }

    fn new_for_viewer(
        content: InspectorContent,
        language: impl Into<String>,
        viewer: ViewerFaction,
    ) -> Self {
        let admission = match &content {
            InspectorContent::Ready(session) => {
                Some(inspection_admission(session.effective_catalog(), viewer))
            }
            InspectorContent::Unavailable(_) => None,
        };
        Self {
            content,
            presenter: EncyclopediaPresenter::default(),
            admission,
            language: language.into(),
            navigation: EncyclopediaNavigationState::default(),
        }
    }

    pub(crate) fn from_availability(
        availability: EncyclopediaAvailability,
        language: impl Into<String>,
    ) -> Self {
        Self::from_availability_for_viewer(availability, language, ViewerFaction::Alliance)
    }

    fn from_availability_for_viewer(
        availability: EncyclopediaAvailability,
        language: impl Into<String>,
        viewer: ViewerFaction,
    ) -> Self {
        let content = match availability {
            EncyclopediaAvailability::Ready(session) => InspectorContent::Ready(Box::new(session)),
            EncyclopediaAvailability::Unavailable(diagnostic) => {
                InspectorContent::Unavailable(diagnostic)
            }
        };
        Self::new_for_viewer(content, language, viewer)
    }

    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn from_packed_request(
        availability: EncyclopediaAvailability,
        language: impl Into<String>,
        request: FixtureRequest,
    ) -> Self {
        let viewer = match request.faction {
            rebellion_render::CockpitFaction::Alliance => ViewerFaction::Alliance,
            rebellion_render::CockpitFaction::Empire => ViewerFaction::Empire,
        };
        Self::from_availability_for_viewer(availability, language, viewer)
    }

    pub(crate) fn viewer(&self) -> Option<ViewerFaction> {
        self.admission.as_ref().map(|admission| admission.viewer)
    }

    pub(crate) fn build_view(&mut self) -> Result<Option<EncyclopediaView>, EncyclopediaError> {
        let InspectorContent::Ready(session) = &self.content else {
            return Ok(None);
        };
        build_encyclopedia_view(
            &mut self.presenter,
            session,
            self.admission.as_ref(),
            &self.language,
            &self.navigation.selection,
        )
        .map(Some)
    }

    pub(crate) fn unavailable_diagnostic(&self) -> Option<&str> {
        match &self.content {
            InspectorContent::Ready(_) => None,
            InspectorContent::Unavailable(diagnostic) => Some(diagnostic),
        }
    }

    pub(crate) fn source_profile(&self) -> Option<&str> {
        match &self.content {
            InspectorContent::Ready(session) => Some(&session.base_manifest().source_profile),
            InspectorContent::Unavailable(_) => None,
        }
    }

    fn binding_for_topic(
        &self,
        topic_id: &str,
    ) -> Option<&rebellion_data::encyclopedia::BindingKey> {
        let InspectorContent::Ready(session) = &self.content else {
            return None;
        };
        let admission = self.admission.as_ref()?;
        let mut matches = admission.admitted.iter().filter(|admitted| {
            rebellion_data::encyclopedia::resolve_topic(session.effective_catalog(), &admitted.key)
                .is_some_and(|resolved| resolved.0 == topic_id)
        });
        let binding = &matches.next()?.key;
        matches.next().is_none().then_some(binding)
    }

    pub(crate) fn apply_action(
        &mut self,
        view: &EncyclopediaView,
        action: EncyclopediaAction,
    ) -> NavigationOutcome {
        apply_encyclopedia_action(&mut self.navigation, view, action)
    }

    /// Apply a finite physical-input batch while rebuilding the presenter DTO
    /// before each dependent command. This prevents a category or mode change
    /// from leaving later commands bound to the prior projection.
    pub(crate) fn apply_action_batch(
        &mut self,
        actions: impl IntoIterator<Item = EncyclopediaAction>,
    ) -> Result<Vec<NavigationOutcome>, EncyclopediaError> {
        let mut outcomes = Vec::new();
        for action in actions {
            let Some(view) = self.build_view()? else {
                break;
            };
            outcomes.push(self.apply_action(&view, action));
        }
        Ok(outcomes)
    }

    #[cfg(test)]
    pub(crate) fn select_category(
        &mut self,
        category_id: Option<String>,
    ) -> Result<Option<NavigationOutcome>, EncyclopediaError> {
        let Some(view) = self.build_view()? else {
            return Ok(None);
        };
        Ok(Some(self.apply_action(
            &view,
            EncyclopediaAction::SelectCategory {
                category_id,
                force: rebellion_render::SelectionForce::Normal,
            },
        )))
    }

    #[cfg(test)]
    pub(crate) fn select_topic(
        &mut self,
        topic_id: String,
    ) -> Result<Option<NavigationOutcome>, EncyclopediaError> {
        let Some(view) = self.build_view()? else {
            return Ok(None);
        };
        Ok(Some(self.apply_action(
            &view,
            EncyclopediaAction::SelectTopic(topic_id),
        )))
    }

    pub(crate) fn navigation_mut(&mut self) -> &mut EncyclopediaNavigationState {
        &mut self.navigation
    }

    fn ensure_initial_topic(&mut self) -> Result<(), EncyclopediaError> {
        if self.navigation.selection.topic_id.is_some() {
            return Ok(());
        }
        let Some(view) = self.build_view()? else {
            return Ok(());
        };
        let Some(first_topic) = view.topics.first().map(|topic| topic.topic_id.clone()) else {
            return Ok(());
        };
        let _ = self.apply_action(&view, EncyclopediaAction::SelectTopic(first_topic));
        let _ = self.apply_action(&view, EncyclopediaAction::SetMode(EncyclopediaMode::Topic));
        Ok(())
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug, serde::Serialize, PartialEq, Eq)]
pub(crate) struct PackedFixtureReport {
    schema_version: u32,
    pub(crate) status: &'static str,
    code: u32,
    scenario: u8,
    faction: &'static str,
    pub(crate) surface: &'static str,
    source_profile: Option<String>,
    pub(crate) topic_id: Option<String>,
    pub(crate) title: Option<String>,
    pub(crate) title_sha256: Option<String>,
    pub(crate) body_sha256: Option<String>,
    pub(crate) body_chars: Option<usize>,
    pub(crate) binding_family: Option<String>,
    pub(crate) binding_dat_id: Option<u32>,
    pub(crate) binding_variant: Option<String>,
    pub(crate) asset_id: Option<String>,
    pub(crate) digest: Option<String>,
    pub(crate) cache_status: String,
    pub(crate) navigation_requests: Option<u32>,
    diagnostic: Option<String>,
    stable_frames: u32,
}

#[derive(Debug, serde::Serialize, PartialEq, Eq)]
struct SelectionEvidence {
    source_profile: Option<String>,
    pub(crate) viewer: &'static str,
    pub(crate) topic_id: String,
    pub(crate) binding_family: String,
    pub(crate) binding_dat_id: u32,
    pub(crate) binding_variant: String,
    pub(crate) title_sha256: String,
    pub(crate) body_sha256: String,
    body_chars: usize,
    pub(crate) asset_id: Option<String>,
    digest: Option<String>,
    pub(crate) cache_status: String,
}

#[derive(Debug, serde::Serialize, PartialEq)]
struct ViewportEvidence {
    source_profile: Option<String>,
    viewer: &'static str,
    pub(crate) topic_id: String,
    pub(crate) body_sha256: String,
    pub(crate) scroll_offset: f32,
    pub(crate) consumed_scroll_intents: Vec<&'static str>,
}

fn content_sha256(value: &str) -> String {
    rebellion_render::inspect_encyclopedia_bytes(value.as_bytes(), None)
        .expect("validated localized content fits the shared byte-inspection budget")
        .sha256
}

fn selection_evidence(
    model: &InspectorModel,
    view: &EncyclopediaView,
    cache_status: &str,
) -> Option<SelectionEvidence> {
    let active = view.active_topic.as_ref()?;
    let binding = model.binding_for_topic(&active.topic_id)?;
    Some(SelectionEvidence {
        source_profile: model.source_profile().map(str::to_owned),
        viewer: match model.viewer()? {
            ViewerFaction::Alliance => "alliance",
            ViewerFaction::Empire => "empire",
        },
        topic_id: active.topic_id.clone(),
        binding_family: binding.family.clone(),
        binding_dat_id: binding.dat_id,
        binding_variant: binding.variant.clone(),
        title_sha256: content_sha256(&active.title),
        body_sha256: content_sha256(&active.body),
        body_chars: active.body.chars().count(),
        asset_id: active.image.as_ref().map(|image| image.asset_id.clone()),
        digest: active.image.as_ref().map(|image| image.digest.clone()),
        cache_status: cache_status.to_owned(),
    })
}

fn viewport_evidence(
    model: &InspectorModel,
    view: &EncyclopediaView,
    scroll_offset: f32,
    consumed_scroll_intents: &[BodyScrollIntent],
) -> Option<ViewportEvidence> {
    let active = view.active_topic.as_ref()?;
    Some(ViewportEvidence {
        source_profile: model.source_profile().map(str::to_owned),
        viewer: match model.viewer()? {
            ViewerFaction::Alliance => "alliance",
            ViewerFaction::Empire => "empire",
        },
        topic_id: active.topic_id.clone(),
        body_sha256: content_sha256(&active.body),
        scroll_offset,
        consumed_scroll_intents: consumed_scroll_intents
            .iter()
            .map(|intent| match intent {
                BodyScrollIntent::LineUp => "line_up",
                BodyScrollIntent::LineDown => "line_down",
                BodyScrollIntent::PageUp => "page_up",
                BodyScrollIntent::PageDown => "page_down",
                BodyScrollIntent::ResetToTop => "reset_to_top",
            })
            .collect(),
    })
}

#[cfg(test)]
impl PackedFixtureReport {
    pub(crate) fn faction(&self) -> &str {
        self.faction
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
#[expect(
    clippy::too_many_arguments,
    reason = "The report records independent presentation evidence without retaining renderer state."
)]
pub(crate) fn packed_fixture_report(
    request: FixtureRequest,
    model: &InspectorModel,
    view: Option<&EncyclopediaView>,
    asset_id: Option<&str>,
    digest: Option<&str>,
    cache_status: &str,
    diagnostic: Option<&str>,
) -> PackedFixtureReport {
    let active = view.and_then(|view| view.active_topic.as_ref());
    let binding = active.and_then(|topic| model.binding_for_topic(&topic.topic_id));
    let unavailable = diagnostic.or_else(|| model.unavailable_diagnostic());
    PackedFixtureReport {
        schema_version: 1,
        status: if view.is_some() && unavailable.is_none() {
            "ready"
        } else {
            "unavailable"
        },
        code: request.code,
        scenario: request.scenario as u8,
        faction: match request.faction {
            rebellion_render::CockpitFaction::Alliance => "alliance",
            rebellion_render::CockpitFaction::Empire => "empire",
        },
        surface: "packed-encyclopedia-fixture",
        source_profile: model.source_profile().map(str::to_owned),
        topic_id: active.map(|topic| topic.topic_id.clone()),
        title: active.map(|topic| topic.title.to_string()),
        title_sha256: active.map(|topic| content_sha256(&topic.title)),
        body_sha256: active.map(|topic| content_sha256(&topic.body)),
        body_chars: active.map(|topic| topic.body.chars().count()),
        binding_family: binding.map(|binding| binding.family.clone()),
        binding_dat_id: binding.map(|binding| binding.dat_id),
        binding_variant: binding.map(|binding| binding.variant.clone()),
        asset_id: asset_id.map(str::to_owned),
        digest: digest.map(str::to_owned),
        cache_status: cache_status.to_owned(),
        // Network request counts are browser-observer evidence, not an
        // in-process fact. The coordinator's browser gate supplies them.
        navigation_requests: None,
        diagnostic: unavailable.map(str::to_owned),
        stable_frames: 3,
    }
}

pub(crate) fn inspection_admission(
    catalog: &EncyclopediaCatalog,
    viewer: ViewerFaction,
) -> AdmissionSnapshot {
    AdmissionSnapshot {
        world_epoch: 0,
        viewer,
        admitted: catalog
            .bindings
            .iter()
            .map(|binding| AdmittedBinding {
                key: binding.key(),
                fact: AdmissionFact::DefinitionPresent,
            })
            .collect(),
    }
}

#[cfg(test)]
pub(crate) struct InspectorTextureFrame<'a, Texture> {
    pub(crate) texture: Option<&'a Texture>,
    pub(crate) asset_id: Option<String>,
    pub(crate) digest: Option<String>,
    pub(crate) cache_status: String,
    pub(crate) diagnostic: Option<String>,
    pub(crate) events: Vec<EncyclopediaTextureEvent>,
}

#[cfg(test)]
pub(crate) struct InspectorTextureState<Backend: EncyclopediaTextureBackend> {
    cache: EncyclopediaTextureCache<Backend>,
    last_status: String,
}

#[cfg(test)]
impl<Backend: EncyclopediaTextureBackend> InspectorTextureState<Backend> {
    pub(crate) fn new(backend: Backend) -> Self {
        Self {
            cache: EncyclopediaTextureCache::new(backend),
            last_status: "not selected".to_owned(),
        }
    }

    pub(crate) fn resolve<'a>(
        &'a mut self,
        image: Option<&TopicImageView>,
    ) -> InspectorTextureFrame<'a, Backend::Texture> {
        let asset_id = image.map(|image| image.asset_id.clone());
        let digest = image.map(|image| image.digest.clone());
        let (cache, last_status) = (&mut self.cache, &mut self.last_status);
        let resolution = cache.resolve(image);
        if image.is_none() {
            *last_status = "no art".to_owned();
        }
        for event in &resolution.events {
            *last_status = match event {
                EncyclopediaTextureEvent::Selected {
                    cache_hit: true, ..
                } => "cache hit".to_owned(),
                EncyclopediaTextureEvent::Selected {
                    cache_hit: false, ..
                } => "uploaded".to_owned(),
                EncyclopediaTextureEvent::Released { .. } if image.is_none() => "no art".to_owned(),
                EncyclopediaTextureEvent::Released { .. } => "released".to_owned(),
                EncyclopediaTextureEvent::Failed { .. } => "failed".to_owned(),
            };
        }
        InspectorTextureFrame {
            texture: resolution.texture,
            asset_id,
            digest,
            cache_status: last_status.clone(),
            diagnostic: resolution.diagnostic.map(str::to_owned),
            events: resolution.events,
        }
    }
}

pub(crate) fn request_value_enabled(value: Option<&OsStr>) -> bool {
    value == Some(OsStr::new("1"))
}

#[cfg(not(target_arch = "wasm32"))]
fn native_viewer_faction(
    value: Option<&OsStr>,
) -> Result<(ViewerFaction, rebellion_render::CockpitFaction), String> {
    match value.map(OsStr::to_str) {
        None => Ok((
            ViewerFaction::Alliance,
            rebellion_render::CockpitFaction::Alliance,
        )),
        Some(Some("alliance")) => Ok((
            ViewerFaction::Alliance,
            rebellion_render::CockpitFaction::Alliance,
        )),
        Some(Some("empire")) => Ok((
            ViewerFaction::Empire,
            rebellion_render::CockpitFaction::Empire,
        )),
        Some(_) => Err(format!(
            "invalid {INSPECTOR_FACTION} value; expected alliance or empire"
        )),
    }
}

#[cfg(test)]
fn fitted_art_size(width: u32, height: u32) -> [f32; 2] {
    let longest_side = width.max(height) as f32;
    debug_assert!(longest_side > 0.0, "validated art has nonzero dimensions");
    let displayed_longest_side = longest_side.clamp(160.0, 360.0);
    let scale = displayed_longest_side / longest_side;
    [width as f32 * scale, height as f32 * scale]
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn requested() -> bool {
    request_value_enabled(std::env::var_os(INSPECTOR_REQUEST).as_deref())
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn run(gdata: &std::path::Path) {
    let (viewer, faction) =
        match native_viewer_faction(std::env::var_os(INSPECTOR_FACTION).as_deref()) {
            Ok(selection) => selection,
            Err(error) => {
                macroquad::logging::error!("[encyclopedia_inspector] {error}");
                return;
            }
        };
    let override_root =
        std::env::var_os("REBELLION_ENCYCLOPEDIA_DIR").map(std::path::PathBuf::from);
    let availability =
        crate::encyclopedia_runtime::load_native_encyclopedia(gdata, override_root.as_deref());
    let model = InspectorModel::from_availability_for_viewer(availability, "1033", viewer);
    let mut chrome = rebellion_render::BmpCache::new();
    chrome.set_base_path(gdata.join("ui"));
    run_surface(model, chrome, faction).await;
}

#[cfg(target_arch = "wasm32")]
pub(crate) async fn run_packed(availability: EncyclopediaAvailability, request: FixtureRequest) {
    let model = InspectorModel::from_packed_request(availability, "1033", request);
    run_surface(
        model,
        rebellion_render::BmpCache::new(),
        request.faction,
        request,
    )
    .await;
}

async fn run_surface(
    mut model: InspectorModel,
    mut chrome: rebellion_render::BmpCache,
    faction: rebellion_render::CockpitFaction,
    #[cfg(target_arch = "wasm32")] browser_request: FixtureRequest,
) {
    use macroquad::prelude::{clear_background, is_quit_requested, next_frame, Color};
    use rebellion_render::{
        draw_encyclopedia_surface, EguiEncyclopediaTextureBackend, EncyclopediaSurfaceLabels,
        EncyclopediaSurfaceState,
    };

    let missing_chrome = chrome.missing_encyclopedia_chrome_resources();
    let mut textures: Option<EncyclopediaTextureCache<EguiEncyclopediaTextureBackend>> = None;
    let mut surface = EncyclopediaSurfaceState::default();
    surface.set_source_labels(EncyclopediaSurfaceLabels {
        index_header: Some(std::sync::Arc::from("Synthetic encyclopedia index")),
        index_static: Some(std::sync::Arc::from("Synthetic category selection")),
    });
    let mut theme_applied = false;
    let mut cache_status = "not selected".to_owned();
    let mut last_evidence_topic = None;
    let mut close_requested = false;
    #[cfg(target_arch = "wasm32")]
    let mut stable_frames = 0_u32;
    #[cfg(target_arch = "wasm32")]
    let mut report_emitted = false;

    while !close_requested {
        if is_quit_requested() {
            break;
        }
        clear_background(Color::from_rgba(9, 13, 22, 255));

        if let Err(error) = model.ensure_initial_topic() {
            model = InspectorModel::new(InspectorContent::Unavailable(error.to_string()), "1033");
        }
        let source_profile = model.source_profile().map(str::to_owned);
        let unavailable = model.unavailable_diagnostic().map(str::to_owned);
        let view = match model.build_view() {
            Ok(view) => view,
            Err(error) => {
                model =
                    InspectorModel::new(InspectorContent::Unavailable(error.to_string()), "1033");
                None
            }
        };

        let mut surface_frame = None;
        egui_macroquad::ui(|ctx| {
            if !theme_applied {
                rebellion_render::theme::load_fonts(ctx);
                rebellion_render::theme::apply_theme(ctx);
                theme_applied = true;
            }
            let textures = textures.get_or_insert_with(|| {
                EncyclopediaTextureCache::new(EguiEncyclopediaTextureBackend::new(ctx))
            });
            if let Some(view) = view.as_ref() {
                surface_frame = Some(draw_encyclopedia_surface(
                    ctx,
                    view,
                    model.navigation_mut(),
                    &mut surface,
                    &mut chrome,
                    textures,
                    faction,
                    egui_macroquad::egui::pos2(85.0, 55.0),
                    1.0,
                ));
            } else {
                let _ = textures.resolve(None);
                egui_macroquad::egui::CentralPanel::default().show(ctx, |ui| {
                    ui.colored_label(
                        egui_macroquad::egui::Color32::LIGHT_RED,
                        format!(
                            "encyclopedia content unavailable: {}",
                            unavailable.as_deref().unwrap_or("unknown diagnostic")
                        ),
                    );
                });
            }

            egui_macroquad::egui::Area::new(egui_macroquad::egui::Id::new(
                "encyclopedia-fixture-diagnostics",
            ))
            .fixed_pos(egui_macroquad::egui::pos2(8.0, 392.0))
            .movable(false)
            .show(ctx, |ui| {
                ui.label("fixture diagnostics — transport/render evidence only");
                if let Some(profile) = source_profile.as_deref() {
                    ui.label(format!("source profile: {profile}"));
                }
                if missing_chrome.is_empty() {
                    ui.label("chrome source ids: ready");
                } else {
                    let ids = missing_chrome
                        .iter()
                        .take(4)
                        .map(|resource| format!("0x{:x}", resource.resource_id))
                        .collect::<Vec<_>>()
                        .join(",");
                    ui.colored_label(
                        egui_macroquad::egui::Color32::LIGHT_RED,
                        format!("chrome missing: {} ({ids})", missing_chrome.len()),
                    );
                }
                if let Some(frame) = surface_frame.as_ref() {
                    ui.label(format!(
                        "asset: {}",
                        frame.active_asset_id.as_deref().unwrap_or("none")
                    ));
                    ui.label(format!(
                        "digest: {}",
                        frame.active_digest.as_deref().unwrap_or("none")
                    ));
                    ui.label(format!("cache: {cache_status}"));
                    if let Some(diagnostic) = frame.texture_diagnostic.as_deref() {
                        ui.colored_label(egui_macroquad::egui::Color32::LIGHT_RED, diagnostic);
                    }
                    for diagnostic in &frame.surface_diagnostics {
                        ui.colored_label(
                            egui_macroquad::egui::Color32::LIGHT_RED,
                            format!("surface diagnostic: {diagnostic:?}"),
                        );
                    }
                }
            });
        });

        if let (Some(_), Some(frame)) = (view.as_ref(), surface_frame.as_mut()) {
            for event in &frame.texture_events {
                cache_status = match event {
                    EncyclopediaTextureEvent::Selected {
                        cache_hit: true, ..
                    } => "cache hit".to_owned(),
                    EncyclopediaTextureEvent::Selected {
                        cache_hit: false, ..
                    } => "uploaded".to_owned(),
                    EncyclopediaTextureEvent::Released { .. } => "released".to_owned(),
                    EncyclopediaTextureEvent::Failed { .. } => "failed".to_owned(),
                };
                macroquad::logging::info!("[encyclopedia_inspector] texture_event={:?}", event);
            }
            if frame.active_asset_id.is_none() {
                cache_status = "no art".to_owned();
            }
            if !frame.consumed_scroll_intents.is_empty() {
                if let Some(evidence) = viewport_evidence(
                    &model,
                    view.as_ref().unwrap(),
                    surface.body_scroll_offset(),
                    &frame.consumed_scroll_intents,
                ) {
                    macroquad::logging::info!(
                        "[encyclopedia_inspector] viewport_evidence={}",
                        serde_json::to_string(&evidence)
                            .expect("viewport evidence contains only serializable DTO fields")
                    );
                }
            }
            if let Some(active) = view
                .as_ref()
                .and_then(|current| current.active_topic.as_ref())
            {
                if last_evidence_topic.as_deref() != Some(active.topic_id.as_str()) {
                    if let Some(evidence) =
                        selection_evidence(&model, view.as_ref().unwrap(), &cache_status)
                    {
                        macroquad::logging::info!(
                            "[encyclopedia_inspector] selection_evidence={}",
                            serde_json::to_string(&evidence)
                                .expect("selection evidence contains only serializable DTO fields")
                        );
                        last_evidence_topic = Some(active.topic_id.clone());
                    }
                }
            }
            match model.apply_action_batch(std::mem::take(&mut frame.actions)) {
                Ok(outcomes) => {
                    for outcome in outcomes {
                        macroquad::logging::info!(
                            "[encyclopedia_inspector] navigation_outcome={:?}",
                            outcome
                        );
                        if outcome == NavigationOutcome::CloseRequested {
                            close_requested = true;
                        }
                    }
                }
                Err(error) => {
                    model = InspectorModel::new(
                        InspectorContent::Unavailable(error.to_string()),
                        "1033",
                    );
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        if !report_emitted {
            stable_frames += 1;
            if stable_frames >= 3 {
                let (asset_id, digest, surface_diagnostic) =
                    surface_frame.as_ref().map_or((None, None, None), |frame| {
                        (
                            frame.active_asset_id.as_deref(),
                            frame.active_digest.as_deref(),
                            frame.texture_diagnostic.as_deref(),
                        )
                    });
                let report = packed_fixture_report(
                    browser_request,
                    &model,
                    view.as_ref(),
                    asset_id,
                    digest,
                    &cache_status,
                    surface_diagnostic,
                );
                crate::interface_test_fixture::emit_report(&report);
                report_emitted = true;
            }
        }
        egui_macroquad::draw();
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::ffi::OsStr;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

    use rebellion_data::encyclopedia::{AdmissionFact, ViewerFaction};
    use rebellion_render::{
        draw_encyclopedia_surface, inspect_encyclopedia_bytes, BmpCache, BodyScrollIntent,
        CockpitFaction, EguiEncyclopediaTextureBackend, EncyclopediaAction, EncyclopediaMode,
        EncyclopediaSurfaceState, EncyclopediaTextureBackend, EncyclopediaTextureCache,
        EncyclopediaTextureUpload, NavigationOutcome, NavigationRejection, SelectionForce,
        SourceKeyIntent,
    };
    use serde_json::Value;

    use super::{
        fitted_art_size, inspection_admission, native_viewer_faction, packed_fixture_report,
        request_value_enabled, requested, selection_evidence, viewport_evidence, InspectorContent,
        InspectorModel, InspectorTextureState, INSPECTOR_REQUEST,
    };
    use crate::encyclopedia_session::{
        prepare_encyclopedia_session, EncyclopediaAvailability, EncyclopediaBytes,
        EncyclopediaSession,
    };
    use crate::interface_test_fixture::{FixtureRequest, Scenario};

    const VALID_CATALOG: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/catalog.json");
    const VALID_MANIFEST: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/manifest.json");
    const VALID_IMAGE_1: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.001"
    );
    const VALID_IMAGE_2: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.002"
    );
    const VALID_IMAGE_3: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.003"
    );

    struct InspectorRequestEnvRestore(Option<std::ffi::OsString>);

    impl Drop for InspectorRequestEnvRestore {
        fn drop(&mut self) {
            if let Some(previous) = self.0.take() {
                std::env::set_var(INSPECTOR_REQUEST, previous);
            } else {
                std::env::remove_var(INSPECTOR_REQUEST);
            }
        }
    }

    fn session() -> EncyclopediaSession {
        customized_session(|_| {})
    }

    fn customized_session(mut mutate: impl FnMut(&mut Value)) -> EncyclopediaSession {
        let mut catalog: Value = serde_json::from_slice(VALID_CATALOG).unwrap();
        mutate(&mut catalog);
        let catalog_bytes = serde_json::to_vec(&catalog).unwrap();
        let catalog_digest = inspect_encyclopedia_bytes(&catalog_bytes, None)
            .unwrap()
            .sha256;
        let mut manifest: Value = serde_json::from_slice(VALID_MANIFEST).unwrap();
        manifest["catalog_sha256"] = Value::from(catalog_digest.clone());
        manifest["files"]["catalog.json"] = Value::from(catalog_digest);

        let bytes: EncyclopediaBytes = BTreeMap::from([
            ("catalog.json".to_owned(), Arc::from(catalog_bytes)),
            (
                "manifest.json".to_owned(),
                Arc::from(serde_json::to_vec(&manifest).unwrap()),
            ),
            ("assets/EDATA.001".to_owned(), Arc::from(VALID_IMAGE_1)),
            ("assets/EDATA.002".to_owned(), Arc::from(VALID_IMAGE_2)),
            ("assets/EDATA.003".to_owned(), Arc::from(VALID_IMAGE_3)),
        ]);
        let dats = BTreeMap::from([(
            "SYNTHETIC.DAT".to_owned(),
            "39fb2c329d34fbdd94bb2a2a596b694597eb00b8b402905ec4920f560210d322".to_owned(),
        )]);
        prepare_encyclopedia_session(bytes, &dats).unwrap()
    }

    #[derive(Debug, Default)]
    struct BackendCounts {
        created: usize,
        released: usize,
    }

    #[derive(Clone)]
    struct CountingBackend {
        counts: Rc<RefCell<BackendCounts>>,
    }

    impl EncyclopediaTextureBackend for CountingBackend {
        type Texture = usize;

        fn upload(
            &mut self,
            upload: EncyclopediaTextureUpload<'_>,
        ) -> Result<Self::Texture, String> {
            assert_eq!(
                upload.rgba.len(),
                upload.width as usize * upload.height as usize * 4
            );
            let mut counts = self.counts.borrow_mut();
            counts.created += 1;
            Ok(counts.created)
        }

        fn release(&mut self, _texture: Self::Texture) {
            self.counts.borrow_mut().released += 1;
        }
    }

    fn texture_state() -> (
        InspectorTextureState<CountingBackend>,
        Rc<RefCell<BackendCounts>>,
    ) {
        let counts = Rc::new(RefCell::new(BackendCounts::default()));
        let backend = CountingBackend {
            counts: Rc::clone(&counts),
        };
        (InspectorTextureState::new(backend), counts)
    }

    #[test]
    fn fixture_view_exposes_selector_text_art_identity_and_quiet_cache_diagnostics() {
        let session = session();
        let expected_digest = session.observed_facts()["assets/EDATA.001"].sha256.clone();
        let mut model = InspectorModel::new(InspectorContent::Ready(Box::new(session)), "1033");
        assert_eq!(model.source_profile(), Some("e37-synthetic-v1"));
        model.select_topic("original:60001".to_owned()).unwrap();

        let view = model.build_view().unwrap().unwrap();
        assert_eq!(view.index_label.as_deref(), Some("Synthetic aggregate"));
        assert_eq!(
            view.categories[0].label.as_deref(),
            Some("Synthetic systems")
        );
        assert_eq!(view.topics[0].topic_id, "original:60001");
        let active = view.active_topic.as_ref().unwrap();
        assert_eq!(active.title.as_ref(), "Amber system");
        assert_eq!(
            active.body.as_ref(),
            "Contributor-written system description."
        );
        let image = active.image.as_ref().unwrap();
        assert_eq!(image.asset_id, "edata:1");
        assert_eq!(image.digest, expected_digest);

        let (mut textures, counts) = texture_state();
        let first = textures.resolve(Some(image));
        assert!(first.texture.is_some());
        assert_eq!(first.asset_id.as_deref(), Some("edata:1"));
        assert_eq!(first.digest.as_deref(), Some(expected_digest.as_str()));
        assert_eq!(first.cache_status, "uploaded");
        assert_eq!(first.diagnostic, None);
        assert_eq!(first.events.len(), 1);

        let second = textures.resolve(Some(image));
        assert!(second.texture.is_some());
        assert_eq!(second.cache_status, "cache hit");
        assert_eq!(second.events.len(), 1);

        let third = textures.resolve(Some(image));
        assert!(third.texture.is_some());
        assert_eq!(third.cache_status, "cache hit");
        assert!(
            third.events.is_empty(),
            "stable frames must not emit cache spam"
        );
        assert_eq!(counts.borrow().created, 1);
        assert_eq!(counts.borrow().released, 0);
    }

    #[test]
    fn packed_availability_feeds_the_existing_inspector_without_reparsing() {
        let prepared = EncyclopediaAvailability::Ready(session());
        let mut model = InspectorModel::from_availability(prepared, "1033");

        model.select_topic("original:60001".to_owned()).unwrap();
        let view = model.build_view().unwrap().unwrap();

        assert_eq!(model.source_profile(), Some("e37-synthetic-v1"));
        assert_eq!(
            view.active_topic.as_ref().unwrap().title.as_ref(),
            "Amber system"
        );
    }

    #[test]
    fn native_inspector_default_remains_alliance_for_viewer_sensitive_art() {
        let mut model =
            InspectorModel::from_availability(EncyclopediaAvailability::Ready(session()), "1033");

        assert_eq!(model.viewer(), Some(ViewerFaction::Alliance));
        model.select_topic("original:60004".to_owned()).unwrap();
        let view = model.build_view().unwrap().unwrap();
        let image = view.active_topic.as_ref().unwrap().image.as_ref().unwrap();

        assert_eq!(image.asset_id, "edata:2");
        assert_eq!(
            image.digest,
            "a93a4e651a970119d8da0386846785163291b7cfd67edaac7f6fc37b719fe592"
        );
    }

    #[test]
    fn native_faction_selector_defaults_to_alliance_and_accepts_both_named_viewers() {
        assert_eq!(
            native_viewer_faction(None).unwrap(),
            (ViewerFaction::Alliance, CockpitFaction::Alliance)
        );
        assert_eq!(
            native_viewer_faction(Some(OsStr::new("alliance"))).unwrap(),
            (ViewerFaction::Alliance, CockpitFaction::Alliance)
        );
        assert_eq!(
            native_viewer_faction(Some(OsStr::new("empire"))).unwrap(),
            (ViewerFaction::Empire, CockpitFaction::Empire)
        );
    }

    #[test]
    fn native_faction_selector_rejects_invalid_or_non_unicode_values() {
        assert_eq!(
            native_viewer_faction(Some(OsStr::new("Alliance"))).unwrap_err(),
            "invalid REBELLION_ENCYCLOPEDIA_VIEWER_FACTION value; expected alliance or empire"
        );
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            assert_eq!(
                native_viewer_faction(Some(OsStr::from_bytes(b"empire\xff"))).unwrap_err(),
                "invalid REBELLION_ENCYCLOPEDIA_VIEWER_FACTION value; expected alliance or empire"
            );
        }
    }

    #[test]
    fn native_faction_selection_drives_the_existing_viewer_sensitive_selector() {
        for (name, expected_viewer, expected_faction, expected_asset, expected_digest) in [
            (
                "alliance",
                ViewerFaction::Alliance,
                CockpitFaction::Alliance,
                "edata:2",
                "a93a4e651a970119d8da0386846785163291b7cfd67edaac7f6fc37b719fe592",
            ),
            (
                "empire",
                ViewerFaction::Empire,
                CockpitFaction::Empire,
                "edata:3",
                "8746c347d4cf14daa2e0cc9d41d1f9abebbbb1aae997f31a172610e9ecdbe2dd",
            ),
        ] {
            let (viewer, faction) = native_viewer_faction(Some(OsStr::new(name))).unwrap();
            assert_eq!(viewer, expected_viewer);
            assert_eq!(faction, expected_faction);
            let mut model = InspectorModel::new_for_viewer(
                InspectorContent::Ready(Box::new(session())),
                "1033",
                viewer,
            );
            model.select_topic("original:60004".to_owned()).unwrap();
            let view = model.build_view().unwrap().unwrap();
            let image = view.active_topic.as_ref().unwrap().image.as_ref().unwrap();
            assert_eq!(image.asset_id, expected_asset);
            assert_eq!(image.digest, expected_digest);
        }
    }

    #[test]
    fn packed_report_does_not_fabricate_unmeasured_navigation_requests() {
        let mut model =
            InspectorModel::from_availability(EncyclopediaAvailability::Ready(session()), "1033");
        model.select_topic("original:60001".to_owned()).unwrap();
        let view = model.build_view().unwrap().unwrap();
        let image = view.active_topic.as_ref().unwrap().image.as_ref().unwrap();
        let request = FixtureRequest {
            scenario: Scenario::PackedEncyclopedia,
            faction: CockpitFaction::Alliance,
            code: 0x0130,
        };

        let report = packed_fixture_report(
            request,
            &model,
            Some(&view),
            Some(image.asset_id.as_str()),
            Some(image.digest.as_str()),
            "cache hit",
            None,
        );

        assert_eq!(report.status, "ready");
        assert_eq!(report.surface, "packed-encyclopedia-fixture");
        assert_eq!(report.topic_id.as_deref(), Some("original:60001"));
        assert_eq!(
            report.title_sha256.as_deref(),
            Some("ba528fb133a6a24d3746cf33432d0365ffd9b0cda0c166079b2861ce9e5a0faf")
        );
        assert_eq!(
            report.body_sha256.as_deref(),
            Some("51332a039e28b4d1a1496dce0248964ff6303a57d4fd8c07aa8a038f19d674ca")
        );
        assert_eq!(report.binding_family.as_deref(), Some("system_locations"));
        assert_eq!(report.binding_dat_id, Some(7));
        assert_eq!(report.binding_variant.as_deref(), Some("default"));
        assert_eq!(report.asset_id.as_deref(), Some("edata:1"));
        assert_eq!(report.digest.as_deref(), Some(image.digest.as_str()));
        assert_eq!(report.cache_status, "cache hit");
        assert_eq!(report.navigation_requests, None);

        let failed_upload = packed_fixture_report(
            request,
            &model,
            Some(&view),
            Some(image.asset_id.as_str()),
            Some(image.digest.as_str()),
            "failed",
            Some("asset edata:1 upload failed"),
        );
        assert_eq!(failed_upload.status, "unavailable");
        assert_eq!(
            failed_upload.diagnostic.as_deref(),
            Some("asset edata:1 upload failed")
        );
    }

    #[test]
    fn selection_evidence_uses_admitted_binding_and_hashes_without_copying_text() {
        let mut model = InspectorModel::new_for_viewer(
            InspectorContent::Ready(Box::new(session())),
            "1033",
            ViewerFaction::Empire,
        );
        model.select_topic("original:60004".to_owned()).unwrap();
        let view = model.build_view().unwrap().unwrap();
        let evidence = selection_evidence(&model, &view, "uploaded").unwrap();

        assert_eq!(evidence.viewer, "empire");
        assert_eq!(evidence.topic_id, "original:60004");
        assert_eq!(evidence.binding_family, "missions");
        assert_eq!(evidence.binding_dat_id, 21);
        assert_eq!(evidence.binding_variant, "viewer_faction");
        assert_eq!(
            evidence.title_sha256,
            "869fd5c18ec158a9268aeaba1bfc7d3cd40ffda34cc69678c30be1962020b0be"
        );
        assert_eq!(
            evidence.body_sha256,
            "c7f8d63cf357218979234ef834cab8f7e0c64c0777f8b80a1a08d33fc0056baa"
        );
        assert_eq!(evidence.asset_id.as_deref(), Some("edata:3"));
        assert_eq!(evidence.cache_status, "uploaded");
        let serialized = serde_json::to_string(&evidence).unwrap();
        assert!(!serialized.contains("Dual beacon"));
        assert!(!serialized.contains("Synthetic faction-sensitive art text."));
    }

    #[test]
    fn viewport_evidence_reports_ordered_scroll_without_copying_owned_text() {
        let mut model = InspectorModel::new_for_viewer(
            InspectorContent::Ready(Box::new(session())),
            "1033",
            ViewerFaction::Alliance,
        );
        model.select_topic("original:60001".to_owned()).unwrap();
        let view = model.build_view().unwrap().unwrap();
        let evidence = viewport_evidence(
            &model,
            &view,
            160.0,
            &[BodyScrollIntent::PageDown, BodyScrollIntent::LineDown],
        )
        .unwrap();

        assert_eq!(evidence.topic_id, "original:60001");
        assert_eq!(
            evidence.body_sha256,
            "51332a039e28b4d1a1496dce0248964ff6303a57d4fd8c07aa8a038f19d674ca"
        );
        assert_eq!(evidence.scroll_offset, 160.0);
        assert_eq!(evidence.consumed_scroll_intents, ["page_down", "line_down"]);
        let serialized = serde_json::to_string(&evidence).unwrap();
        assert!(!serialized.contains("Synthetic system body."));
    }

    #[test]
    fn packed_report_preserves_long_unicode_text_and_explicit_no_art() {
        let long_body = format!("Συνθετικό σώμα 🚀 {}", "λ".repeat(8_192));
        let session = customized_session(|catalog| {
            catalog["topics"]["original:60005"]["localized"]["1033"]["title"] =
                Value::from("Χωρίς εικόνα");
            catalog["topics"]["original:60005"]["localized"]["1033"]["body"] =
                Value::from(long_body.clone());
        });
        let mut model =
            InspectorModel::from_availability(EncyclopediaAvailability::Ready(session), "1033");
        model.select_topic("original:60005".to_owned()).unwrap();
        let view = model.build_view().unwrap().unwrap();
        let request = FixtureRequest {
            scenario: Scenario::PackedEncyclopedia,
            faction: CockpitFaction::Empire,
            code: 0x0230,
        };

        let report =
            packed_fixture_report(request, &model, Some(&view), None, None, "no art", None);

        assert_eq!(report.title.as_deref(), Some("Χωρίς εικόνα"));
        assert_eq!(report.body_chars, Some(long_body.chars().count()));
        assert_eq!(report.asset_id, None);
        assert_eq!(report.digest, None);
        assert_eq!(report.cache_status, "no art");
    }

    #[test]
    fn packed_report_keeps_missing_content_unavailable_without_an_approximate_view() {
        let model = InspectorModel::from_availability(
            EncyclopediaAvailability::Unavailable(
                "invalid_encyclopedia_bundle:catalog_hash_mismatch".to_owned(),
            ),
            "1033",
        );
        let request = FixtureRequest {
            scenario: Scenario::PackedEncyclopedia,
            faction: CockpitFaction::Alliance,
            code: 0x0130,
        };

        let report = packed_fixture_report(request, &model, None, None, None, "not selected", None);

        assert_eq!(report.status, "unavailable");
        assert_eq!(report.topic_id, None);
        assert_eq!(report.asset_id, None);
        assert_eq!(report.navigation_requests, None);
        assert_eq!(
            report.diagnostic.as_deref(),
            Some("invalid_encyclopedia_bundle:catalog_hash_mismatch")
        );
    }

    #[test]
    fn absent_art_allocates_nothing_and_releases_the_previous_selection() {
        let mut model = InspectorModel::new(InspectorContent::Ready(Box::new(session())), "1033");
        let (mut textures, counts) = texture_state();

        model.select_topic("original:60001".to_owned()).unwrap();
        let with_art = model.build_view().unwrap().unwrap();
        let image = with_art.active_topic.as_ref().unwrap().image.as_ref();
        assert!(textures.resolve(image).texture.is_some());

        model.select_topic("original:60005".to_owned()).unwrap();
        let without_art = model.build_view().unwrap().unwrap();
        assert!(without_art.active_topic.as_ref().unwrap().image.is_none());
        let absent = textures.resolve(None);
        assert!(absent.texture.is_none());
        assert_eq!(absent.cache_status, "no art");
        assert_eq!(counts.borrow().created, 1);
        assert_eq!(counts.borrow().released, 1);

        let repeated = textures.resolve(None);
        assert!(repeated.events.is_empty());
        assert_eq!(counts.borrow().created, 1);
        assert_eq!(counts.borrow().released, 1);
    }

    #[test]
    fn missing_content_reports_unavailable_without_constructing_a_view() {
        let mut model = InspectorModel::new(
            InspectorContent::Unavailable("bundle_absent at synthetic root".to_owned()),
            "1033",
        );

        assert_eq!(model.build_view().unwrap(), None);
        assert_eq!(
            model.unavailable_diagnostic(),
            Some("bundle_absent at synthetic root")
        );
    }

    #[test]
    fn long_body_survives_the_fixture_view_without_truncation() {
        let long_body = "synthetic long body ".repeat(2_048);
        let session = customized_session(|catalog| {
            catalog["topics"]["original:60001"]["localized"]["1033"]["body"] =
                Value::from(long_body.clone());
        });
        let mut model = InspectorModel::new(InspectorContent::Ready(Box::new(session)), "1033");
        model.select_topic("original:60001".to_owned()).unwrap();

        let view = model.build_view().unwrap().unwrap();
        assert_eq!(view.active_topic.as_ref().unwrap().body.as_ref(), long_body);
    }

    #[test]
    fn repeated_image_changes_keep_only_one_backend_resource_owned() {
        let mut model = InspectorModel::new(InspectorContent::Ready(Box::new(session())), "1033");
        let (mut textures, counts) = texture_state();

        for index in 0..24 {
            let topic = if index % 2 == 0 {
                "original:60001"
            } else {
                "original:60002"
            };
            model.select_topic(topic.to_owned()).unwrap();
            let view = model.build_view().unwrap().unwrap();
            let image = view.active_topic.as_ref().unwrap().image.as_ref();
            assert!(textures.resolve(image).texture.is_some());
            let counts = counts.borrow();
            assert_eq!(counts.created - counts.released, 1);
        }

        drop(textures);
        let counts = counts.borrow();
        assert_eq!(counts.created, 24);
        assert_eq!(counts.released, 24);
    }

    #[test]
    fn inspection_snapshot_is_explicitly_catalog_scoped_and_world_independent() {
        let session = session();
        let admission = inspection_admission(session.effective_catalog(), ViewerFaction::Alliance);

        assert_eq!(admission.viewer, ViewerFaction::Alliance);
        assert_eq!(admission.world_epoch, 0);
        assert_eq!(
            admission.admitted.len(),
            session.effective_catalog().bindings.len()
        );
        assert!(admission
            .admitted
            .iter()
            .all(|binding| binding.fact == AdmissionFact::DefinitionPresent));
        assert_eq!(
            admission
                .admitted
                .iter()
                .map(|binding| binding.key.clone())
                .collect::<Vec<_>>(),
            session
                .effective_catalog()
                .bindings
                .iter()
                .map(|binding| binding.key())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn native_request_requires_the_exact_enabled_value() {
        assert!(request_value_enabled(Some(OsStr::new("1"))));
        for value in [
            None,
            Some(OsStr::new("")),
            Some(OsStr::new("0")),
            Some(OsStr::new("true")),
        ] {
            assert!(!request_value_enabled(value));
        }
    }

    #[test]
    fn native_request_reads_the_feature_only_environment_boundary() {
        static ENV_LOCK: Mutex<()> = Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap();
        let _restore = InspectorRequestEnvRestore(std::env::var_os(INSPECTOR_REQUEST));

        std::env::remove_var(INSPECTOR_REQUEST);
        assert!(!requested());
        std::env::set_var(INSPECTOR_REQUEST, "1");
        assert!(requested());
    }

    #[test]
    fn category_selection_changes_membership_and_clears_the_previous_topic() {
        let mut model = InspectorModel::new(InspectorContent::Ready(Box::new(session())), "1033");
        model.select_topic("original:60002".to_owned()).unwrap();
        model
            .select_category(Some("command:0x70".to_owned()))
            .unwrap();

        let view = model.build_view().unwrap().unwrap();
        assert_eq!(
            view.navigation.selected_category_id.as_deref(),
            Some("command:0x70")
        );
        assert_eq!(view.navigation.selected_topic_id, None);
        assert_eq!(
            view.topics
                .iter()
                .map(|topic| topic.topic_id.as_str())
                .collect::<Vec<_>>(),
            ["original:60001"]
        );
    }

    #[test]
    fn finite_batch_rebuilds_projection_before_rejecting_a_stale_topic_target() {
        let mut model = InspectorModel::new(InspectorContent::Ready(Box::new(session())), "1033");
        model.select_topic("original:60002".to_owned()).unwrap();

        let outcomes = model
            .apply_action_batch([
                EncyclopediaAction::SelectCategory {
                    category_id: Some("command:0x70".to_owned()),
                    force: SelectionForce::Normal,
                },
                EncyclopediaAction::SelectTopic("original:60002".to_owned()),
            ])
            .unwrap();

        assert_eq!(
            outcomes,
            vec![
                NavigationOutcome::Applied,
                NavigationOutcome::Rejected(NavigationRejection::UnavailableTopic),
            ]
        );
        let view = model.build_view().unwrap().unwrap();
        assert_eq!(
            view.navigation.selected_category_id.as_deref(),
            Some("command:0x70")
        );
        assert_eq!(view.navigation.selected_topic_id, None);
        assert_eq!(
            view.topics
                .iter()
                .map(|topic| topic.topic_id.as_str())
                .collect::<Vec<_>>(),
            ["original:60001"]
        );
    }

    #[test]
    fn finite_batch_rebuilds_between_repeated_next_commands() {
        let mut model = InspectorModel::new(InspectorContent::Ready(Box::new(session())), "1033");
        model.ensure_initial_topic().unwrap();
        let expected = model
            .build_view()
            .unwrap()
            .unwrap()
            .topics
            .iter()
            .take(3)
            .map(|topic| topic.topic_id.clone())
            .collect::<Vec<_>>();
        assert_eq!(expected.len(), 3);

        let outcomes = model
            .apply_action_batch([EncyclopediaAction::NextTopic, EncyclopediaAction::NextTopic])
            .unwrap();

        assert_eq!(
            outcomes,
            vec![NavigationOutcome::Applied, NavigationOutcome::Applied]
        );
        assert_eq!(
            model.navigation.selection.topic_id.as_ref(),
            expected.get(2)
        );
    }

    #[test]
    fn finite_batch_rebuilds_after_mode_change_and_preserves_ordered_scroll_intents() {
        let mut model = InspectorModel::new(InspectorContent::Ready(Box::new(session())), "1033");
        let topic_ids = model
            .build_view()
            .unwrap()
            .unwrap()
            .topics
            .iter()
            .take(2)
            .map(|topic| topic.topic_id.clone())
            .collect::<Vec<_>>();
        model.select_topic(topic_ids[0].clone()).unwrap();

        let outcomes = model
            .apply_action_batch([
                EncyclopediaAction::SetMode(EncyclopediaMode::Topic),
                EncyclopediaAction::NextTopic,
            ])
            .unwrap();

        assert_eq!(
            outcomes,
            vec![NavigationOutcome::Applied, NavigationOutcome::Applied]
        );
        assert_eq!(
            model.navigation.selection.topic_id.as_ref(),
            topic_ids.get(1)
        );
        assert_eq!(
            model.navigation.pending_body_scroll_intents(),
            [BodyScrollIntent::ResetToTop, BodyScrollIntent::ResetToTop]
        );
        assert_eq!(
            model.navigation.take_body_scroll_intents(),
            [BodyScrollIntent::ResetToTop, BodyScrollIntent::ResetToTop]
        );
        assert!(model.navigation.take_body_scroll_intents().is_empty());
    }

    fn apply_actual_key_batch(
        model: &mut InspectorModel,
        keys: impl IntoIterator<Item = egui_macroquad::egui::Key>,
    ) -> (Vec<EncyclopediaAction>, Vec<NavigationOutcome>) {
        let ctx = egui_macroquad::egui::Context::default();
        let mut surface = EncyclopediaSurfaceState::default();
        let mut chrome = BmpCache::new();
        let mut textures = EncyclopediaTextureCache::new(EguiEncyclopediaTextureBackend::new(&ctx));

        let view = model.build_view().unwrap().unwrap();
        let _ = ctx.run(egui_macroquad::egui::RawInput::default(), |ctx| {
            let _ = draw_encyclopedia_surface(
                ctx,
                &view,
                model.navigation_mut(),
                &mut surface,
                &mut chrome,
                &mut textures,
                CockpitFaction::Alliance,
                egui_macroquad::egui::Pos2::ZERO,
                1.0,
            );
        });

        let raw_input = egui_macroquad::egui::RawInput {
            events: keys
                .into_iter()
                .map(|key| egui_macroquad::egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui_macroquad::egui::Modifiers::default(),
                })
                .collect(),
            ..Default::default()
        };
        let mut frame = None;
        let _ = ctx.run(raw_input, |ctx| {
            frame = Some(draw_encyclopedia_surface(
                ctx,
                &view,
                model.navigation_mut(),
                &mut surface,
                &mut chrome,
                &mut textures,
                CockpitFaction::Alliance,
                egui_macroquad::egui::Pos2::ZERO,
                1.0,
            ));
        });
        let actions = frame.unwrap().actions;
        let outcomes = model.apply_action_batch(actions.clone()).unwrap();
        (actions, outcomes)
    }

    #[test]
    fn actual_enter_then_right_batch_uses_the_evolving_topic_mode() {
        use egui_macroquad::egui::Key;

        let mut model = InspectorModel::new(InspectorContent::Ready(Box::new(session())), "1033");
        let topics = model
            .build_view()
            .unwrap()
            .unwrap()
            .topics
            .iter()
            .take(2)
            .map(|topic| topic.topic_id.clone())
            .collect::<Vec<_>>();
        model.select_topic(topics[0].clone()).unwrap();

        let (actions, outcomes) = apply_actual_key_batch(&mut model, [Key::Enter, Key::ArrowRight]);

        assert_eq!(
            actions,
            [
                EncyclopediaAction::SourceKey(SourceKeyIntent::Enter),
                EncyclopediaAction::SourceKey(SourceKeyIntent::Right),
            ]
        );
        assert_eq!(
            outcomes,
            [NavigationOutcome::Applied, NavigationOutcome::Applied]
        );
        assert_eq!(model.navigation.mode(), EncyclopediaMode::Topic);
        assert_eq!(model.navigation.selection.topic_id.as_ref(), topics.get(1));
    }

    #[test]
    fn rejected_enter_keeps_following_right_key_in_index_mode() {
        use egui_macroquad::egui::Key;

        let mut model = InspectorModel::new(InspectorContent::Ready(Box::new(session())), "1033");
        let expected_category = model
            .build_view()
            .unwrap()
            .unwrap()
            .categories
            .iter()
            .find(|category| category.enabled)
            .unwrap()
            .category_id
            .clone();

        let (_, outcomes) = apply_actual_key_batch(&mut model, [Key::Enter, Key::ArrowRight]);

        assert_eq!(
            outcomes,
            [
                NavigationOutcome::Rejected(NavigationRejection::NoCurrentTopic),
                NavigationOutcome::Applied,
            ]
        );
        assert_eq!(model.navigation.mode(), EncyclopediaMode::Index);
        assert_eq!(
            model.navigation.selected_category_id(),
            Some(expected_category.as_str())
        );
    }

    #[test]
    fn evolving_key_batch_preserves_repeated_scroll_and_return_order() {
        use egui_macroquad::egui::Key;

        let mut model = InspectorModel::new(InspectorContent::Ready(Box::new(session())), "1033");
        let first_topic = model
            .build_view()
            .unwrap()
            .unwrap()
            .topics
            .first()
            .unwrap()
            .topic_id
            .clone();
        model.select_topic(first_topic).unwrap();

        let (_, outcomes) = apply_actual_key_batch(
            &mut model,
            [
                Key::Enter,
                Key::ArrowDown,
                Key::ArrowDown,
                Key::PageDown,
                Key::Enter,
            ],
        );

        assert_eq!(
            outcomes,
            [
                NavigationOutcome::Applied,
                NavigationOutcome::ScrollRequested(BodyScrollIntent::LineDown),
                NavigationOutcome::ScrollRequested(BodyScrollIntent::LineDown),
                NavigationOutcome::ScrollRequested(BodyScrollIntent::PageDown),
                NavigationOutcome::ReturnForwarded,
            ]
        );
        assert_eq!(
            model.navigation.take_body_scroll_intents(),
            [
                BodyScrollIntent::ResetToTop,
                BodyScrollIntent::LineDown,
                BodyScrollIntent::LineDown,
                BodyScrollIntent::PageDown,
            ]
        );
        assert!(model.navigation.take_body_scroll_intents().is_empty());
    }

    #[test]
    fn initial_fixture_frame_selects_the_first_resolved_stable_topic() {
        let mut model = InspectorModel::new(InspectorContent::Ready(Box::new(session())), "1033");

        model.ensure_initial_topic().unwrap();
        let view = model.build_view().unwrap().unwrap();
        assert_eq!(
            view.navigation.selected_topic_id.as_deref(),
            Some("original:60001")
        );
        assert_eq!(
            view.active_topic.as_ref().map(|topic| topic.title.as_ref()),
            Some("Amber system")
        );
    }

    #[test]
    fn fixture_art_is_large_enough_to_inspect_and_still_bounded() {
        assert_eq!(fitted_art_size(2, 2), [160.0, 160.0]);
        assert_eq!(fitted_art_size(1_000, 500), [360.0, 180.0]);
        assert_eq!(fitted_art_size(80, 320), [80.0, 320.0]);
    }
}
