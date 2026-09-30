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
    EncyclopediaSelection, EncyclopediaTextureBackend, EncyclopediaTextureCache,
    EncyclopediaTextureEvent, EncyclopediaView, TopicImageView,
};

use crate::encyclopedia_presenter::{build_encyclopedia_view, EncyclopediaPresenter};
#[cfg(not(target_arch = "wasm32"))]
use crate::encyclopedia_session::EncyclopediaAvailability;
use crate::encyclopedia_session::EncyclopediaSession;

const INSPECTOR_REQUEST: &str = "REBELLION_ENCYCLOPEDIA_INSPECTOR";
const INSPECTOR_LABEL: &str = "Encyclopedia content inspector — not parity";

pub(crate) enum InspectorContent {
    Ready(Box<EncyclopediaSession>),
    Unavailable(String),
}

pub(crate) struct InspectorModel {
    content: InspectorContent,
    presenter: EncyclopediaPresenter,
    admission: Option<AdmissionSnapshot>,
    language: String,
    selection: EncyclopediaSelection,
}

impl InspectorModel {
    pub(crate) fn new(content: InspectorContent, language: impl Into<String>) -> Self {
        let admission = match &content {
            InspectorContent::Ready(session) => {
                Some(inspection_admission(session.effective_catalog()))
            }
            InspectorContent::Unavailable(_) => None,
        };
        Self {
            content,
            presenter: EncyclopediaPresenter::default(),
            admission,
            language: language.into(),
            selection: EncyclopediaSelection::default(),
        }
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
            &self.selection,
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

    pub(crate) fn select_category(&mut self, category_id: Option<String>) {
        self.selection.category_id = category_id;
        self.selection.topic_id = None;
    }

    pub(crate) fn select_topic(&mut self, topic_id: Option<String>) {
        self.selection.topic_id = topic_id;
    }

    fn ensure_initial_topic(&mut self) -> Result<(), EncyclopediaError> {
        if self.selection.topic_id.is_some() {
            return Ok(());
        }
        let first_topic = self
            .build_view()?
            .and_then(|view| view.topics.first().map(|topic| topic.topic_id.clone()));
        self.selection.topic_id = first_topic;
        Ok(())
    }
}

pub(crate) fn inspection_admission(catalog: &EncyclopediaCatalog) -> AdmissionSnapshot {
    AdmissionSnapshot {
        world_epoch: 0,
        viewer: ViewerFaction::Alliance,
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

pub(crate) struct InspectorTextureFrame<'a, Texture> {
    pub(crate) texture: Option<&'a Texture>,
    pub(crate) asset_id: Option<String>,
    pub(crate) digest: Option<String>,
    pub(crate) cache_status: String,
    pub(crate) diagnostic: Option<String>,
    pub(crate) events: Vec<EncyclopediaTextureEvent>,
}

pub(crate) struct InspectorTextureState<Backend: EncyclopediaTextureBackend> {
    cache: EncyclopediaTextureCache<Backend>,
    last_status: String,
}

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
    use macroquad::prelude::{
        clear_background, is_key_pressed, is_quit_requested, next_frame, Color, KeyCode,
    };
    use rebellion_render::{EguiEncyclopediaTextureBackend, TopicImageRenderProfile};

    let override_root =
        std::env::var_os("REBELLION_ENCYCLOPEDIA_DIR").map(std::path::PathBuf::from);
    let content = match crate::encyclopedia_runtime::load_native_encyclopedia(
        gdata,
        override_root.as_deref(),
    ) {
        EncyclopediaAvailability::Ready(session) => InspectorContent::Ready(Box::new(session)),
        EncyclopediaAvailability::Unavailable(diagnostic) => {
            InspectorContent::Unavailable(diagnostic)
        }
    };
    let mut model = InspectorModel::new(content, "1033");
    let mut textures = None;
    let mut theme_applied = false;

    loop {
        if is_quit_requested() || is_key_pressed(KeyCode::Escape) {
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

        egui_macroquad::ui(|ctx| {
            if !theme_applied {
                rebellion_render::theme::load_fonts(ctx);
                rebellion_render::theme::apply_theme(ctx);
                theme_applied = true;
            }
            let textures = textures.get_or_insert_with(|| {
                InspectorTextureState::new(EguiEncyclopediaTextureBackend::new(ctx))
            });
            egui_macroquad::egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading(INSPECTOR_LABEL);
                ui.label(
                    "Temporary fixture surface for validated content transport and rendering.",
                );
                ui.label("Press Escape to close.");
                ui.separator();

                if let Some(diagnostic) = unavailable.as_deref() {
                    ui.colored_label(
                        egui_macroquad::egui::Color32::LIGHT_RED,
                        format!("content unavailable: {diagnostic}"),
                    );
                    let _ = textures.resolve(None);
                    return;
                }
                let Some(view) = view.as_ref() else {
                    ui.label("content unavailable");
                    let _ = textures.resolve(None);
                    return;
                };

                ui.horizontal_wrapped(|ui| {
                    let index_selected = view.navigation.selected_category_id.is_none();
                    if ui
                        .add_enabled(
                            view.index_enabled,
                            egui_macroquad::egui::Button::new(
                                view.index_label.as_deref().unwrap_or("Unavailable index"),
                            )
                            .selected(index_selected),
                        )
                        .clicked()
                    {
                        model.select_category(None);
                    }
                    for category in &view.categories {
                        let selected = view.navigation.selected_category_id.as_deref()
                            == Some(category.category_id.as_str());
                        if ui
                            .add_enabled(
                                category.enabled,
                                egui_macroquad::egui::Button::new(
                                    category.label.as_deref().unwrap_or("Unavailable category"),
                                )
                                .selected(selected),
                            )
                            .clicked()
                        {
                            model.select_category(Some(category.category_id.clone()));
                        }
                    }
                });
                ui.separator();

                ui.columns(2, |columns| {
                    egui_macroquad::egui::ScrollArea::vertical()
                        .id_salt("encyclopedia_fixture_topics")
                        .show(&mut columns[0], |ui| {
                            for topic in &view.topics {
                                let selected = view.navigation.selected_topic_id.as_deref()
                                    == Some(topic.topic_id.as_str());
                                if ui
                                    .selectable_label(selected, topic.title.as_ref())
                                    .clicked()
                                {
                                    model.select_topic(Some(topic.topic_id.clone()));
                                }
                            }
                        });

                    egui_macroquad::egui::ScrollArea::vertical()
                        .id_salt("encyclopedia_fixture_detail")
                        .show(&mut columns[1], |ui| {
                            let Some(active) = view.active_topic.as_ref() else {
                                ui.label("Select an available topic.");
                                let _ = textures.resolve(None);
                                return;
                            };

                            ui.heading(active.title.as_ref());
                            ui.label(format!("topic: {}", active.topic_id));
                            if let Some(profile) = source_profile.as_deref() {
                                ui.label(format!("source profile: {profile}"));
                            }

                            let texture_frame = textures.resolve(active.image.as_ref());
                            for event in &texture_frame.events {
                                macroquad::logging::info!(
                                    "[encyclopedia_inspector] texture_event={:?}",
                                    event
                                );
                            }
                            if let (Some(texture), Some(image)) =
                                (texture_frame.texture, active.image.as_ref())
                            {
                                let [width, height] = fitted_art_size(image.width, image.height);
                                let size = egui_macroquad::egui::vec2(width, height);
                                ui.add(egui_macroquad::egui::Image::new((texture.id(), size)));
                            } else if active.image.is_none() {
                                ui.label("art: none");
                            }
                            if let Some(asset_id) = texture_frame.asset_id.as_deref() {
                                ui.label(format!("asset: {asset_id}"));
                            }
                            if let Some(digest) = texture_frame.digest.as_deref() {
                                ui.label(format!("digest: {digest}"));
                            }
                            ui.label(format!("cache: {}", texture_frame.cache_status));
                            if let Some(diagnostic) = texture_frame.diagnostic.as_deref() {
                                ui.colored_label(
                                    egui_macroquad::egui::Color32::LIGHT_RED,
                                    diagnostic,
                                );
                            }
                            if let Some(image) = active.image.as_ref() {
                                let sampling = match image.render_profile {
                                    TopicImageRenderProfile::OriginalNearest => "nearest",
                                    TopicImageRenderProfile::FaithfulHdLinear => "linear",
                                };
                                ui.label(format!(
                                    "image: {}x{} {} ({sampling})",
                                    image.width, image.height, image.format
                                ));
                            }
                            ui.separator();
                            ui.label(active.body.as_ref());
                        });
                });

                for diagnostic in &view.diagnostics {
                    ui.label(format!(
                        "diagnostic: {} {:?}",
                        diagnostic.code, diagnostic.scope
                    ));
                }
            });
        });
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
        inspect_encyclopedia_bytes, EncyclopediaTextureBackend, EncyclopediaTextureUpload,
    };
    use serde_json::Value;

    use super::{
        fitted_art_size, inspection_admission, request_value_enabled, requested, InspectorContent,
        InspectorModel, InspectorTextureState, INSPECTOR_REQUEST,
    };
    use crate::encyclopedia_session::{
        prepare_encyclopedia_session, EncyclopediaBytes, EncyclopediaSession,
    };

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
        model.select_topic(Some("original:60001".to_owned()));

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
    fn absent_art_allocates_nothing_and_releases_the_previous_selection() {
        let mut model = InspectorModel::new(InspectorContent::Ready(Box::new(session())), "1033");
        let (mut textures, counts) = texture_state();

        model.select_topic(Some("original:60001".to_owned()));
        let with_art = model.build_view().unwrap().unwrap();
        let image = with_art.active_topic.as_ref().unwrap().image.as_ref();
        assert!(textures.resolve(image).texture.is_some());

        model.select_topic(Some("original:60005".to_owned()));
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
        model.select_topic(Some("original:60001".to_owned()));

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
            model.select_topic(Some(topic.to_owned()));
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
        let admission = inspection_admission(session.effective_catalog());

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
        model.select_topic(Some("original:60002".to_owned()));
        model.select_category(Some("command:0x70".to_owned()));

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
