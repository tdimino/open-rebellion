//! Feature-gated content inspector for the validated encyclopedia pipeline.
//!
//! This module is compiled only for `interface-test-fixtures`. Its admission
//! snapshot deliberately exposes every validated catalog binding for transport
//! inspection; it is not a gameplay-availability adapter and is never used by
//! the production route.

#[cfg(not(target_arch = "wasm32"))]
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsStr,
    path::PathBuf,
    sync::Arc,
};

#[cfg(not(target_arch = "wasm32"))]
use rebellion_data::encyclopedia::{resolve_admitted_topics, resolve_localized_label};
use rebellion_data::encyclopedia::{
    AdmissionFact, AdmissionSnapshot, AdmittedBinding, EncyclopediaCatalog, EncyclopediaError,
    ViewerFaction,
};
#[cfg(test)]
use rebellion_render::EncyclopediaTextureBackend;
use rebellion_render::{
    apply_encyclopedia_action, reconcile_encyclopedia_state, BodyScrollIntent, EncyclopediaAction,
    EncyclopediaMode, EncyclopediaNavigationState, EncyclopediaTextureCache,
    EncyclopediaTextureEvent, EncyclopediaView, NavigationOutcome, TopicImageRenderProfile,
};
#[cfg(not(target_arch = "wasm32"))]
use rebellion_render::{
    ActiveTopicView, AssetRenderProfile, CategoryViewItem, EncyclopediaDiagnosticScope,
    EncyclopediaViewDiagnostic, NavigationState, TopicImageView, TopicViewItem,
};

#[cfg(any(test, target_arch = "wasm32"))]
use crate::interface_test_fixture::FixtureRequest;

use crate::encyclopedia_presenter::{build_encyclopedia_view, EncyclopediaPresenter};
use crate::encyclopedia_session::{EncyclopediaAvailability, EncyclopediaSession};
#[cfg(not(target_arch = "wasm32"))]
use crate::{
    encyclopedia_lifecycle::EncyclopediaLifecycle,
    encyclopedia_mods::{EffectiveEncyclopediaSnapshot, EncyclopediaImageOwner},
};

#[cfg(not(target_arch = "wasm32"))]
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
    initial_topic_selected: bool,
    removed_topic_for_fixture: Option<String>,
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
            initial_topic_selected: false,
            removed_topic_for_fixture: None,
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
    pub(crate) fn from_browser_request(
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

    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn from_packed_request(
        availability: EncyclopediaAvailability,
        language: impl Into<String>,
        request: FixtureRequest,
    ) -> Self {
        Self::from_browser_request(availability, language, request)
    }

    pub(crate) fn viewer(&self) -> Option<ViewerFaction> {
        self.admission.as_ref().map(|admission| admission.viewer)
    }

    fn catalog(&self) -> Option<&EncyclopediaCatalog> {
        match &self.content {
            InspectorContent::Ready(session) => Some(session.effective_catalog()),
            InspectorContent::Unavailable(_) => None,
        }
    }

    pub(crate) fn catalog_generation(&self) -> Option<u64> {
        match &self.content {
            InspectorContent::Ready(session) => Some(session.generation()),
            InspectorContent::Unavailable(_) => None,
        }
    }

    pub(crate) fn requested_language(&self) -> &str {
        &self.language
    }

    pub(crate) fn set_requested_language_for_fixture(&mut self, language: impl Into<String>) {
        self.language = language.into();
    }

    pub(crate) fn effective_language(&self, view: &EncyclopediaView) -> Option<&str> {
        let topic_id = view.active_topic.as_ref()?.topic_id.as_str();
        let catalog = self.catalog()?;
        let localized = &catalog.topics.get(topic_id)?.localized;
        if localized.contains_key(&self.language) {
            Some(&self.language)
        } else {
            localized
                .contains_key(&catalog.default_language)
                .then_some(catalog.default_language.as_str())
        }
    }

    /// Installs a replacement immutable admitted subset without consulting or
    /// mutating `GameWorld`. The feature harness removes only the currently
    /// selected, already-admitted binding and advances the supplied epoch.
    pub(crate) fn replace_admission_for_fixture(
        &mut self,
    ) -> Result<Option<String>, EncyclopediaError> {
        let Some(selected) = self.navigation.selection.topic_id.clone() else {
            return Ok(None);
        };
        let Some(catalog) = self.catalog() else {
            return Ok(None);
        };
        let Some(admission) = self.admission.as_ref() else {
            return Ok(None);
        };
        let position = admission.admitted.iter().position(|admitted| {
            rebellion_data::encyclopedia::resolve_topic(catalog, &admitted.key)
                .is_some_and(|topic_id| topic_id.0 == selected)
        });
        let Some(position) = position else {
            return Ok(None);
        };
        let Some(admission) = self.admission.as_mut() else {
            return Ok(None);
        };
        admission.admitted.remove(position);
        admission.world_epoch = admission.world_epoch.saturating_add(1);
        if let Some(view) = self.build_view()? {
            let _ = reconcile_encyclopedia_state(&mut self.navigation, &view);
        }
        self.removed_topic_for_fixture = Some(selected.clone());
        Ok(Some(selected))
    }

    /// Publishes a fresh immutable admitted snapshot for the same viewer. This
    /// feature-only control models a replacement epoch reusing the catalog's
    /// numeric IDs; it never derives admission from `GameWorld`.
    pub(crate) fn restore_admission_for_fixture(&mut self) -> bool {
        let Some(previous) = self.admission.as_ref() else {
            return false;
        };
        let viewer = previous.viewer;
        let world_epoch = previous.world_epoch.saturating_add(1);
        let Some(catalog) = self.catalog() else {
            return false;
        };
        let mut replacement = inspection_admission(catalog, viewer);
        replacement.world_epoch = world_epoch;
        self.admission = Some(replacement);
        true
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

    fn navigation(&self) -> &EncyclopediaNavigationState {
        &self.navigation
    }

    fn ensure_initial_topic(&mut self) -> Result<(), EncyclopediaError> {
        if self.initial_topic_selected {
            return Ok(());
        }
        if self.navigation.selection.topic_id.is_some() {
            self.initial_topic_selected = true;
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
        self.initial_topic_selected = true;
        Ok(())
    }
}

/// Feature-only projection of the exact E24 publication used by the native
/// live acceptance surface. The tracked snapshot clone keeps old-live bytes in
/// E24's global accounting; this model retains only one selected presentation
/// image copy and clears it whenever the published generation changes.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct LiveInspectorModel {
    snapshot: EffectiveEncyclopediaSnapshot,
    admission: AdmissionSnapshot,
    source_profile: String,
    language: String,
    navigation: EncyclopediaNavigationState,
    title_cache: BTreeMap<String, Arc<str>>,
    active_body: Option<(String, Arc<str>)>,
    active_image: Option<(String, String, TopicImageView)>,
    asset_profile: AssetRenderProfile,
    hd_root: Option<PathBuf>,
    prepared_generation: Option<u64>,
    prepared_images: crate::encyclopedia_hd::PreparedEncyclopediaImages,
    presentation_error: Option<String>,
    initial_topic_selected: bool,
    removed_topic_for_fixture: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
impl LiveInspectorModel {
    pub(crate) fn from_lifecycle(
        lifecycle: &EncyclopediaLifecycle,
        source_profile: impl Into<String>,
        language: impl Into<String>,
        viewer: ViewerFaction,
    ) -> Option<Self> {
        let snapshot = lifecycle.effective_snapshot_for_fixture()?;
        let admission = inspection_admission(snapshot.catalog(), viewer);
        Some(Self {
            snapshot,
            admission,
            source_profile: source_profile.into(),
            language: language.into(),
            navigation: EncyclopediaNavigationState::default(),
            title_cache: BTreeMap::new(),
            active_body: None,
            active_image: None,
            asset_profile: AssetRenderProfile::OriginalParity,
            hd_root: None,
            prepared_generation: None,
            prepared_images: crate::encyclopedia_hd::PreparedEncyclopediaImages::default(),
            presentation_error: None,
            initial_topic_selected: false,
            removed_topic_for_fixture: None,
        })
    }

    pub(crate) fn configure_images(
        &mut self,
        profile: AssetRenderProfile,
        hd_root: Option<PathBuf>,
    ) {
        self.asset_profile = profile;
        self.hd_root = hd_root;
        self.prepared_generation = None;
        self.prepared_images = crate::encyclopedia_hd::PreparedEncyclopediaImages::default();
    }

    /// Rebinds to the exact accepted generation while retaining stable-ID
    /// navigation. Returns true only when a new effective publication exists.
    pub(crate) fn rebind_from_lifecycle(&mut self, lifecycle: &EncyclopediaLifecycle) -> bool {
        let Some(snapshot) = lifecycle.effective_snapshot_for_fixture() else {
            return false;
        };
        if snapshot.generation() == self.snapshot.generation() {
            return false;
        }
        self.admission = inspection_admission(snapshot.catalog(), self.admission.viewer);
        self.snapshot = snapshot;
        self.title_cache.clear();
        self.active_body = None;
        self.active_image = None;
        self.prepared_generation = None;
        self.prepared_images = crate::encyclopedia_hd::PreparedEncyclopediaImages::default();
        self.presentation_error = None;
        true
    }

    #[must_use]
    pub(crate) fn generation(&self) -> u64 {
        self.snapshot.generation()
    }

    fn viewer(&self) -> ViewerFaction {
        self.admission.viewer
    }

    fn binding_for_topic(
        &self,
        topic_id: &str,
    ) -> Option<&rebellion_data::encyclopedia::BindingKey> {
        let mut matches = self.admission.admitted.iter().filter(|admitted| {
            rebellion_data::encyclopedia::resolve_topic(self.snapshot.catalog(), &admitted.key)
                .is_some_and(|resolved| resolved.0 == topic_id)
        });
        let binding = &matches.next()?.key;
        matches.next().is_none().then_some(binding)
    }

    fn requested_language(&self) -> &str {
        &self.language
    }

    fn set_requested_language_for_fixture(&mut self, language: impl Into<String>) {
        self.language = language.into();
    }

    fn effective_language(&self, view: &EncyclopediaView) -> Option<&str> {
        let topic_id = view.active_topic.as_ref()?.topic_id.as_str();
        let catalog = self.snapshot.catalog();
        let localized = &catalog.topics.get(topic_id)?.localized;
        if localized.contains_key(&self.language) {
            Some(&self.language)
        } else {
            localized
                .contains_key(&catalog.default_language)
                .then_some(catalog.default_language.as_str())
        }
    }

    fn replace_admission_for_fixture(&mut self) -> Result<Option<String>, EncyclopediaError> {
        let Some(selected) = self.navigation.selection.topic_id.clone() else {
            return Ok(None);
        };
        let position = self.admission.admitted.iter().position(|admitted| {
            rebellion_data::encyclopedia::resolve_topic(self.snapshot.catalog(), &admitted.key)
                .is_some_and(|topic_id| topic_id.0 == selected)
        });
        let Some(position) = position else {
            return Ok(None);
        };
        self.admission.admitted.remove(position);
        self.admission.world_epoch = self.admission.world_epoch.saturating_add(1);
        let view = self.build_view()?;
        let _ = reconcile_encyclopedia_state(&mut self.navigation, &view);
        self.removed_topic_for_fixture = Some(selected.clone());
        Ok(Some(selected))
    }

    fn restore_admission_for_fixture(&mut self) -> bool {
        let viewer = self.admission.viewer;
        let world_epoch = self.admission.world_epoch.saturating_add(1);
        self.admission = inspection_admission(self.snapshot.catalog(), viewer);
        self.admission.world_epoch = world_epoch;
        true
    }

    pub(crate) fn build_view(&mut self) -> Result<EncyclopediaView, EncyclopediaError> {
        if let Some(error) = &self.presentation_error {
            return Err(EncyclopediaError::for_session(
                "fixture_presentation_error",
                "$",
                error.clone(),
            ));
        }
        let mut view = build_effective_snapshot_view(
            &self.snapshot,
            &self.admission,
            &self.language,
            &self.navigation.selection,
            &mut self.title_cache,
            &mut self.active_body,
            &mut self.active_image,
        )?;
        self.apply_prepared_image(&mut view);
        Ok(view)
    }

    pub(crate) fn select_topic(
        &mut self,
        topic_id: String,
    ) -> Result<NavigationOutcome, EncyclopediaError> {
        let view = self.build_view()?;
        let outcome = apply_encyclopedia_action(
            &mut self.navigation,
            &view,
            EncyclopediaAction::SelectTopic(topic_id),
        );
        if outcome == NavigationOutcome::Applied {
            let _ = apply_encyclopedia_action(
                &mut self.navigation,
                &view,
                EncyclopediaAction::SetMode(EncyclopediaMode::Topic),
            );
        }
        Ok(outcome)
    }

    fn ensure_initial_topic(&mut self) -> Result<(), EncyclopediaError> {
        if self.initial_topic_selected {
            return Ok(());
        }
        if self.navigation.selection.topic_id.is_some() {
            self.initial_topic_selected = true;
            return Ok(());
        }
        let view = self.build_view()?;
        let Some(first_topic) = view.topics.first().map(|topic| topic.topic_id.clone()) else {
            return Ok(());
        };
        let _ = apply_encyclopedia_action(
            &mut self.navigation,
            &view,
            EncyclopediaAction::SelectTopic(first_topic),
        );
        let _ = apply_encyclopedia_action(
            &mut self.navigation,
            &view,
            EncyclopediaAction::SetMode(EncyclopediaMode::Topic),
        );
        self.initial_topic_selected = true;
        Ok(())
    }

    fn apply_action_batch(
        &mut self,
        actions: impl IntoIterator<Item = EncyclopediaAction>,
    ) -> Result<Vec<NavigationOutcome>, EncyclopediaError> {
        let mut outcomes = Vec::new();
        for action in actions {
            let view = self.build_view()?;
            outcomes.push(apply_encyclopedia_action(
                &mut self.navigation,
                &view,
                action,
            ));
        }
        Ok(outcomes)
    }

    fn navigation_mut(&mut self) -> &mut EncyclopediaNavigationState {
        &mut self.navigation
    }

    fn navigation(&self) -> &EncyclopediaNavigationState {
        &self.navigation
    }

    fn unavailable_diagnostic(&self) -> Option<&str> {
        self.presentation_error.as_deref()
    }

    fn source_profile(&self) -> Option<&str> {
        Some(&self.source_profile)
    }

    fn fail_presentation(&mut self, error: impl ToString) {
        self.presentation_error = Some(error.to_string());
    }

    fn selected_owner(&self, image_id: &str) -> Option<&EncyclopediaImageOwner> {
        self.snapshot.image_owners().get(image_id)
    }

    fn apply_prepared_image(&mut self, view: &mut EncyclopediaView) {
        let Some(active) = view.active_topic.as_mut() else {
            return;
        };
        if self.prepared_generation != Some(self.snapshot.generation()) {
            let prepared = match active.image.as_ref() {
                None => crate::encyclopedia_hd::prepare_native_encyclopedia_hd(
                    self.asset_profile,
                    self.hd_root.as_deref(),
                    &[crate::encyclopedia_hd::EncyclopediaImageCandidate::Null {
                        topic_id: &active.topic_id,
                    }],
                ),
                Some(image) => match self.selected_owner(&image.asset_id) {
                    Some(EncyclopediaImageOwner::Mod { .. }) => {
                        crate::encyclopedia_hd::prepare_native_encyclopedia_hd(
                            self.asset_profile,
                            self.hd_root.as_deref(),
                            &[crate::encyclopedia_hd::EncyclopediaImageCandidate::Mod {
                                topic_id: &active.topic_id,
                                image,
                            }],
                        )
                    }
                    Some(EncyclopediaImageOwner::Base { path }) => {
                        let file_name = std::path::Path::new(path)
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or(path);
                        let normalized = file_name.replace('.', "_");
                        let approval_key = format!("edata/{normalized}");
                        let output_path = PathBuf::from(format!("EData/{normalized}.png"));
                        crate::encyclopedia_hd::prepare_native_encyclopedia_hd(
                            self.asset_profile,
                            self.hd_root.as_deref(),
                            &[crate::encyclopedia_hd::EncyclopediaImageCandidate::Base {
                                topic_id: &active.topic_id,
                                image,
                                approval_key: &approval_key,
                                output_relative_path: &output_path,
                            }],
                        )
                    }
                    None => crate::encyclopedia_hd::PreparedEncyclopediaImages::original_only(),
                },
            };
            for diagnostic in prepared.diagnostics() {
                macroquad::logging::warn!(
                    "[encyclopedia_native_acceptance] hd_diagnostic={:?}",
                    diagnostic
                );
            }
            self.prepared_images = prepared;
            self.prepared_generation = Some(self.snapshot.generation());
        }
        active.image = self
            .prepared_images
            .select(&active.topic_id, active.image.as_ref());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn build_effective_snapshot_view(
    snapshot: &EffectiveEncyclopediaSnapshot,
    admission: &AdmissionSnapshot,
    language: &str,
    selection: &rebellion_render::EncyclopediaSelection,
    title_cache: &mut BTreeMap<String, Arc<str>>,
    active_body: &mut Option<(String, Arc<str>)>,
    active_image: &mut Option<(String, String, TopicImageView)>,
) -> Result<EncyclopediaView, EncyclopediaError> {
    let catalog = snapshot.catalog();
    let mut diagnostics = Vec::new();
    let index_label =
        match resolve_localized_label(&catalog.index.labels, &catalog.default_language, language) {
            Ok(label) => Some(Arc::from(label)),
            Err(error) => {
                diagnostics.push(EncyclopediaViewDiagnostic {
                    code: error.code(),
                    scope: EncyclopediaDiagnosticScope::Index,
                });
                None
            }
        };
    let index_enabled = index_label.is_some();
    let categories = catalog
        .categories
        .iter()
        .map(|category| {
            let label = match resolve_localized_label(
                &category.labels,
                &catalog.default_language,
                language,
            ) {
                Ok(label) => Some(Arc::from(label)),
                Err(error) => {
                    diagnostics.push(EncyclopediaViewDiagnostic {
                        code: error.code(),
                        scope: EncyclopediaDiagnosticScope::Category {
                            category_id: category.id.clone(),
                        },
                    });
                    None
                }
            };
            CategoryViewItem {
                category_id: category.id.clone(),
                command: category.command.clone(),
                enabled: label.is_some(),
                label,
            }
        })
        .collect::<Vec<_>>();
    let (selected_category_id, registry_topic_ids) = match selection.category_id.as_deref() {
        None if index_enabled => (None, catalog.index.topic_ids.as_slice()),
        None => (None, &[][..]),
        Some(category_id) => match catalog
            .categories
            .iter()
            .enumerate()
            .find(|(_, category)| category.id == category_id)
        {
            Some((index, category)) if categories[index].enabled => {
                (Some(category.id.clone()), category.topic_ids.as_slice())
            }
            Some((_, category)) => (Some(category.id.clone()), &[][..]),
            None if index_enabled => {
                diagnostics.push(EncyclopediaViewDiagnostic {
                    code: "unknown_category_selection",
                    scope: EncyclopediaDiagnosticScope::Category {
                        category_id: category_id.to_owned(),
                    },
                });
                (None, catalog.index.topic_ids.as_slice())
            }
            None => (None, &[][..]),
        },
    };
    let resolved = resolve_admitted_topics(catalog, registry_topic_ids, Some(admission), language)?;
    diagnostics.extend(
        resolved
            .diagnostics
            .iter()
            .map(|diagnostic| EncyclopediaViewDiagnostic {
                code: diagnostic.error.code(),
                scope: EncyclopediaDiagnosticScope::Topic {
                    topic_id: diagnostic.topic_id.0.clone(),
                },
            }),
    );
    let selected_position = selection.topic_id.as_deref().and_then(|selected| {
        resolved
            .rows
            .iter()
            .position(|topic| topic.topic_id.0 == selected)
    });
    let topics = resolved
        .rows
        .iter()
        .map(|topic| TopicViewItem {
            topic_id: topic.topic_id.0.clone(),
            title: title_cache
                .entry(topic.topic_id.0.clone())
                .or_insert_with(|| Arc::from(topic.localized.title.as_str()))
                .clone(),
        })
        .collect::<Vec<_>>();
    let active_topic = selected_position
        .map(|index| {
            let topic = &resolved.rows[index];
            let body = match active_body {
                Some((topic_id, body)) if topic_id == &topic.topic_id.0 => body.clone(),
                _ => {
                    let body: Arc<str> = Arc::from(topic.localized.body.as_str());
                    *active_body = Some((topic.topic_id.0.clone(), body.clone()));
                    body
                }
            };
            let image = effective_topic_image(
                snapshot,
                topic.image_id.map(|id| id.0.as_str()),
                active_image,
            )?;
            Ok(ActiveTopicView {
                topic_id: topic.topic_id.0.clone(),
                title: topics[index].title.clone(),
                body,
                image,
                stats: Vec::new(),
            })
        })
        .transpose()?;
    let selected_topic_id = selected_position.map(|index| topics[index].topic_id.clone());
    let previous_topic_id = selected_position
        .and_then(|index| index.checked_sub(1))
        .map(|index| topics[index].topic_id.clone());
    let next_topic_id = selected_position
        .and_then(|index| index.checked_add(1))
        .and_then(|index| topics.get(index))
        .map(|topic| topic.topic_id.clone());
    Ok(EncyclopediaView {
        index_label,
        index_enabled,
        categories,
        topics,
        active_topic,
        navigation: NavigationState {
            selected_category_id,
            selected_topic_id,
            previous_topic_id,
            next_topic_id,
            world_epoch: admission.world_epoch,
        },
        diagnostics,
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn effective_topic_image(
    snapshot: &EffectiveEncyclopediaSnapshot,
    image_id: Option<&str>,
    cache: &mut Option<(String, String, TopicImageView)>,
) -> Result<Option<TopicImageView>, EncyclopediaError> {
    let Some(image_id) = image_id else {
        *cache = None;
        return Ok(None);
    };
    let descriptor = snapshot.catalog().images.get(image_id).ok_or_else(|| {
        EncyclopediaError::for_session(
            "missing_effective_image",
            format!("$.images.{image_id}"),
            "selected image identity is absent from the effective snapshot",
        )
    })?;
    if let Some((cached_id, cached_digest, image)) = cache {
        if cached_id == image_id && cached_digest == &descriptor.sha256 {
            return Ok(Some(image.clone()));
        }
    }
    let bytes = snapshot.image(image_id).ok_or_else(|| {
        EncyclopediaError::for_session(
            "missing_runtime_file",
            &descriptor.path,
            "selected image has no retained effective provider bytes",
        )
    })?;
    if u64::try_from(bytes.len()).ok() != Some(descriptor.byte_length) {
        return Err(EncyclopediaError::for_session(
            "image_byte_length_mismatch",
            &descriptor.path,
            "effective provider length differs from its validated descriptor",
        ));
    }
    let image = TopicImageView {
        asset_id: image_id.to_owned(),
        digest: descriptor.sha256.clone(),
        format: descriptor.format.clone(),
        width: descriptor.width,
        height: descriptor.height,
        bytes: Arc::from(bytes),
        render_profile: TopicImageRenderProfile::OriginalNearest,
    };
    *cache = Some((
        image_id.to_owned(),
        descriptor.sha256.clone(),
        image.clone(),
    ));
    Ok(Some(image))
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

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BrowserEncyclopediaTransport {
    Packed,
    Loose,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl BrowserEncyclopediaTransport {
    const fn surface_label(self) -> &'static str {
        match self {
            Self::Packed => "packed-encyclopedia-fixture",
            Self::Loose => "loose-encyclopedia-fixture",
        }
    }
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

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
struct SurfaceFocusedControl {
    kind: &'static str,
    target_id: u64,
    focused_id: Option<u64>,
    owns_focus: bool,
}

/// One typed snapshot of the feature surface. Values are deliberately limited
/// to immutable presenter/controller state plus the exact selected image and
/// cache event; the record contains no catalog body or world object.
#[derive(Debug, Clone, serde::Serialize, PartialEq)]
struct SurfaceObservationValues {
    target: &'static str,
    viewer_faction: &'static str,
    mode: &'static str,
    focused_control: Option<SurfaceFocusedControl>,
    selected_category_command: Option<String>,
    visible_topic_ids: Vec<String>,
    visible_topic_ids_sha256: String,
    selected_topic_id: Option<String>,
    selected_topic_index: Option<usize>,
    visible_topic_count: usize,
    previous_enabled: bool,
    next_enabled: bool,
    body_scroll_offset: f32,
    world_epoch: u64,
    world_evidence_kind: &'static str,
    catalog_generation: u64,
    requested_language: String,
    effective_language: Option<String>,
    title_sha256: Option<String>,
    body_sha256: Option<String>,
    asset_id: Option<String>,
    asset_digest: Option<String>,
    render_profile: Option<&'static str>,
    selected_source_kind: &'static str,
    live_enabled_mods: Vec<String>,
    texture_cache_event: &'static str,
}

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
struct SurfaceControllerState {
    mode: &'static str,
    selected_category_command: Option<String>,
    selected_topic_id: Option<String>,
    selected_topic_index: Option<usize>,
    visible_topic_count: usize,
    visible_topic_ids: Vec<String>,
    visible_topic_ids_sha256: String,
    previous_enabled: bool,
    next_enabled: bool,
    world_epoch: u64,
}

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
struct SurfaceControllerStep {
    input_record_id: Option<String>,
    action: String,
    outcome: String,
    before: SurfaceControllerState,
    after: SurfaceControllerState,
}

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
struct SurfaceTextureEvent {
    kind: &'static str,
    asset_id: String,
    digest: Option<String>,
    render_profile: Option<&'static str>,
    cache_hit: Option<bool>,
    diagnostic: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
struct SurfaceInputAttempt {
    control: &'static str,
    enabled: bool,
}

#[derive(Debug, serde::Serialize)]
struct SurfaceTransition<'a> {
    from_record_id: Option<&'a str>,
    controller_steps: &'a [SurfaceControllerStep],
    fixture_controls: &'a [String],
    input_attempts: &'a [SurfaceInputAttempt],
    texture_events: &'a [SurfaceTextureEvent],
}

#[derive(Debug, serde::Serialize)]
struct SurfaceObservation<'a> {
    schema_version: u32,
    status: &'static str,
    record_id: String,
    sequence: u64,
    values: &'a SurfaceObservationValues,
    transition: SurfaceTransition<'a>,
}

#[derive(Default)]
struct SurfaceObservationRecorder {
    sequence: u64,
    last_fingerprint: Option<String>,
    last_record_id: Option<String>,
    pending_controller_steps: Vec<SurfaceControllerStep>,
    pending_fixture_controls: Vec<String>,
    pending_input_attempts: Vec<SurfaceInputAttempt>,
    pending_texture_events: Vec<SurfaceTextureEvent>,
}

impl SurfaceObservationRecorder {
    fn last_record_id(&self) -> Option<&str> {
        self.last_record_id.as_deref()
    }

    fn push_controller_steps(&mut self, steps: impl IntoIterator<Item = SurfaceControllerStep>) {
        self.pending_controller_steps.extend(steps);
    }

    fn push_fixture_control(&mut self, control: String) {
        self.pending_fixture_controls.push(control);
    }

    fn push_input_attempts(&mut self, attempts: impl IntoIterator<Item = SurfaceInputAttempt>) {
        self.pending_input_attempts.extend(attempts);
    }

    fn push_texture_events(&mut self, events: impl IntoIterator<Item = SurfaceTextureEvent>) {
        self.pending_texture_events.extend(events);
    }

    fn emit_if_changed(&mut self, values: &SurfaceObservationValues) -> Option<String> {
        let fingerprint = serde_json::to_vec(values)
            .expect("surface observations contain only serializable DTO fields");
        let fingerprint = rebellion_render::inspect_encyclopedia_bytes(&fingerprint, None)
            .expect("surface observation fits the shared inspection budget")
            .sha256;
        if self.last_fingerprint.as_deref() == Some(fingerprint.as_str())
            && self.pending_controller_steps.is_empty()
            && self.pending_fixture_controls.is_empty()
            && self.pending_input_attempts.is_empty()
            && self.pending_texture_events.is_empty()
        {
            return None;
        }

        self.sequence = self.sequence.saturating_add(1);
        let record_id = format!("surface-{:08}", self.sequence);
        let serialized = serde_json::to_string(&SurfaceObservation {
            schema_version: 5,
            status: "surface_observation",
            record_id: record_id.clone(),
            sequence: self.sequence,
            values,
            transition: SurfaceTransition {
                from_record_id: self.last_record_id.as_deref(),
                controller_steps: &self.pending_controller_steps,
                fixture_controls: &self.pending_fixture_controls,
                input_attempts: &self.pending_input_attempts,
                texture_events: &self.pending_texture_events,
            },
        })
        .expect("surface observation is serializable");
        self.last_record_id = Some(record_id);
        self.last_fingerprint = Some(fingerprint);
        self.pending_controller_steps.clear();
        self.pending_fixture_controls.clear();
        self.pending_input_attempts.clear();
        self.pending_texture_events.clear();
        Some(serialized)
    }
}

fn content_sha256(value: &str) -> String {
    rebellion_render::inspect_encyclopedia_bytes(value.as_bytes(), None)
        .expect("validated localized content fits the shared byte-inspection budget")
        .sha256
}

fn visible_topic_ids_sha256(view: &EncyclopediaView) -> String {
    let visible_ids = view
        .topics
        .iter()
        .map(|topic| topic.topic_id.as_str())
        .collect::<Vec<_>>();
    rebellion_render::inspect_encyclopedia_bytes(
        &serde_json::to_vec(&visible_ids).expect("stable topic IDs are serializable"),
        None,
    )
    .expect("stable topic IDs fit the shared inspection budget")
    .sha256
}

fn visible_topic_ids(view: &EncyclopediaView) -> Vec<String> {
    view.topics
        .iter()
        .map(|topic| topic.topic_id.clone())
        .collect()
}

fn selected_category_command(
    navigation: &EncyclopediaNavigationState,
    view: &EncyclopediaView,
) -> Option<String> {
    navigation
        .selected_category_id()
        .and_then(|category_id| {
            view.categories
                .iter()
                .find(|category| category.category_id == category_id)
                .map(|category| category.command.clone())
        })
        .or_else(|| Some("0x6f".to_owned()))
}

fn surface_controller_state(
    navigation: &EncyclopediaNavigationState,
    view: &EncyclopediaView,
) -> SurfaceControllerState {
    SurfaceControllerState {
        mode: match navigation.mode() {
            EncyclopediaMode::Index => "index",
            EncyclopediaMode::Topic => "topic",
        },
        selected_category_command: selected_category_command(navigation, view),
        selected_topic_id: view.navigation.selected_topic_id.clone(),
        selected_topic_index: view
            .navigation
            .selected_topic_id
            .as_deref()
            .and_then(|selected| {
                view.topics
                    .iter()
                    .position(|topic| topic.topic_id == selected)
            }),
        visible_topic_count: view.topics.len(),
        visible_topic_ids: visible_topic_ids(view),
        visible_topic_ids_sha256: visible_topic_ids_sha256(view),
        previous_enabled: view.navigation.previous_topic_id.is_some(),
        next_enabled: view.navigation.next_topic_id.is_some(),
        world_epoch: view.navigation.world_epoch,
    }
}

fn selection_evidence(
    model: &SurfaceModel,
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
    model: &SurfaceModel,
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

fn texture_cache_event_name(cache_status: &str) -> &'static str {
    match cache_status {
        "not selected" => "not_selected",
        "uploaded" => "uploaded",
        "cache hit" => "cache_hit",
        "released" => "released",
        "no art" => "no_art",
        "failed" => "failed",
        _ => "failed",
    }
}

fn surface_texture_event(event: &EncyclopediaTextureEvent) -> SurfaceTextureEvent {
    fn profile_name(profile: TopicImageRenderProfile) -> &'static str {
        match profile {
            TopicImageRenderProfile::OriginalNearest => "original_nearest",
            TopicImageRenderProfile::FaithfulHdLinear => "faithful_hd_linear",
        }
    }

    match event {
        EncyclopediaTextureEvent::Selected {
            asset_id,
            digest,
            profile,
            cache_hit,
        } => SurfaceTextureEvent {
            kind: "selected",
            asset_id: asset_id.clone(),
            digest: Some(digest.clone()),
            render_profile: Some(profile_name(*profile)),
            cache_hit: Some(*cache_hit),
            diagnostic: None,
        },
        EncyclopediaTextureEvent::Released {
            asset_id,
            digest,
            profile,
        } => SurfaceTextureEvent {
            kind: "released",
            asset_id: asset_id.clone(),
            digest: Some(digest.clone()),
            render_profile: Some(profile_name(*profile)),
            cache_hit: None,
            diagnostic: None,
        },
        EncyclopediaTextureEvent::Failed {
            asset_id,
            diagnostic,
        } => SurfaceTextureEvent {
            kind: "failed",
            asset_id: asset_id.clone(),
            digest: None,
            render_profile: None,
            cache_hit: None,
            diagnostic: Some(diagnostic.clone()),
        },
    }
}

fn surface_input_attempt(
    attempt: &rebellion_render::encyclopedia::EncyclopediaSurfaceInputAttempt,
) -> SurfaceInputAttempt {
    SurfaceInputAttempt {
        control: match attempt.control {
            rebellion_render::encyclopedia::EncyclopediaSurfaceInputControl::PreviousTopic => {
                "previous_topic"
            }
            rebellion_render::encyclopedia::EncyclopediaSurfaceInputControl::NextTopic => {
                "next_topic"
            }
        },
        enabled: attempt.enabled,
    }
}

fn surface_observation_values(
    model: &SurfaceModel,
    view: &EncyclopediaView,
    surface: &rebellion_render::EncyclopediaSurfaceState,
    focus: Option<rebellion_render::encyclopedia::EncyclopediaSurfaceFocus>,
    cache_status: &str,
    selected_source_kind: &'static str,
    live_enabled_mods: Vec<String>,
) -> Option<SurfaceObservationValues> {
    let viewer_faction = match model.viewer()? {
        ViewerFaction::Alliance => "alliance",
        ViewerFaction::Empire => "empire",
    };
    let navigation = model.navigation();
    let mode = match navigation.mode() {
        EncyclopediaMode::Index => "index",
        EncyclopediaMode::Topic => "topic",
    };
    let selected_category_command = selected_category_command(navigation, view);
    let visible_topic_ids = visible_topic_ids(view);
    let visible_topic_ids_sha256 = visible_topic_ids_sha256(view);
    let active = view.active_topic.as_ref();
    let image = active.and_then(|topic| topic.image.as_ref());
    let render_profile = image.map(|image| match image.render_profile {
        TopicImageRenderProfile::OriginalNearest => "original_nearest",
        TopicImageRenderProfile::FaithfulHdLinear => "faithful_hd_linear",
    });
    Some(SurfaceObservationValues {
        #[cfg(not(target_arch = "wasm32"))]
        target: "native",
        #[cfg(target_arch = "wasm32")]
        target: "browser",
        viewer_faction,
        mode,
        focused_control: focus.map(|focus| SurfaceFocusedControl {
            kind: match focus.target_kind {
                rebellion_render::encyclopedia::EncyclopediaSurfaceFocusKind::IndexList => {
                    "index_list"
                }
                rebellion_render::encyclopedia::EncyclopediaSurfaceFocusKind::TopicBody => {
                    "topic_body"
                }
            },
            target_id: focus.target_id,
            focused_id: focus.focused_id,
            owns_focus: focus.owns_focus,
        }),
        selected_category_command,
        visible_topic_ids,
        visible_topic_ids_sha256,
        selected_topic_id: view.navigation.selected_topic_id.clone(),
        selected_topic_index: view
            .navigation
            .selected_topic_id
            .as_deref()
            .and_then(|selected| {
                view.topics
                    .iter()
                    .position(|topic| topic.topic_id == selected)
            }),
        visible_topic_count: view.topics.len(),
        previous_enabled: view.navigation.previous_topic_id.is_some(),
        next_enabled: view.navigation.next_topic_id.is_some(),
        body_scroll_offset: surface.body_scroll_offset(),
        world_epoch: view.navigation.world_epoch,
        world_evidence_kind: "catalog_scoped_synthetic_admission",
        catalog_generation: model.catalog_generation()?,
        requested_language: model.requested_language().to_owned(),
        effective_language: model.effective_language(view).map(str::to_owned),
        title_sha256: active.map(|topic| content_sha256(&topic.title)),
        body_sha256: active.map(|topic| content_sha256(&topic.body)),
        asset_id: image.map(|image| image.asset_id.clone()),
        asset_digest: image.map(|image| image.digest.clone()),
        render_profile,
        selected_source_kind,
        live_enabled_mods,
        texture_cache_event: texture_cache_event_name(cache_status),
    })
}

fn navigation_outcome_name(outcome: NavigationOutcome) -> &'static str {
    match outcome {
        NavigationOutcome::Applied => "applied",
        NavigationOutcome::NoChange => "no_change",
        NavigationOutcome::Rejected(_) => "rejected",
        NavigationOutcome::ScrollRequested(_) => "scroll_requested",
        NavigationOutcome::CloseRequested => "close_requested",
        NavigationOutcome::ReturnForwarded => "return_forwarded",
    }
}

fn reduce_surface_actions(
    model: &mut SurfaceModel,
    recorder: &mut SurfaceObservationRecorder,
    actions: impl IntoIterator<Item = EncyclopediaAction>,
) -> Result<bool, EncyclopediaError> {
    let mut close_requested = false;
    let mut controller_steps = Vec::new();
    for action in actions {
        let Some(before_view) = model.build_view()? else {
            break;
        };
        let action_name = format!("{action:?}");
        let before = surface_controller_state(model.navigation(), &before_view);
        let outcome = model.apply_action_to_view(&before_view, action);
        let Some(after_view) = model.build_view()? else {
            break;
        };
        let after = surface_controller_state(model.navigation(), &after_view);
        controller_steps.push(SurfaceControllerStep {
            input_record_id: recorder.last_record_id().map(str::to_owned),
            action: action_name,
            outcome: navigation_outcome_name(outcome).to_owned(),
            before,
            after,
        });
        macroquad::logging::info!("[encyclopedia_inspector] navigation_outcome={:?}", outcome);
        close_requested |= outcome == NavigationOutcome::CloseRequested;
    }
    recorder.push_controller_steps(controller_steps);
    Ok(close_requested)
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
    fixture_report(
        BrowserEncyclopediaTransport::Packed,
        request,
        model,
        view,
        asset_id,
        digest,
        cache_status,
        diagnostic,
    )
}

#[cfg(any(test, target_arch = "wasm32"))]
#[expect(
    clippy::too_many_arguments,
    reason = "The report records independent presentation evidence without retaining renderer state."
)]
pub(crate) fn fixture_report(
    transport: BrowserEncyclopediaTransport,
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
        surface: transport.surface_label(),
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

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn fingerprint_serializable(value: &impl serde::Serialize) -> String {
    let bytes = serde_json::to_vec(value)
        .expect("feature-only native acceptance state must remain serializable");
    let hash = bytes.iter().fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
    });
    format!("fnv1a64:{hash:016x}")
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
#[derive(Debug, Clone)]
pub(crate) struct NativeSimulationFingerprints {
    pub(crate) world: String,
    pub(crate) save: String,
    pub(crate) rng: String,
}

enum SurfaceModel {
    Static(InspectorModel),
    #[cfg(not(target_arch = "wasm32"))]
    Live(LiveInspectorModel),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SurfaceFixtureControl {
    RequestMissingLanguage,
    RequestDefaultLanguage,
    RemoveSelectedAdmission,
    AttemptRemovedTopic,
    RestoreAdmissionSnapshot,
    OriginalProfile,
    FaithfulHdProfile,
}

#[derive(Debug, PartialEq, Eq)]
struct SurfaceFixtureEffect {
    label: String,
    action: Option<EncyclopediaAction>,
}

impl SurfaceFixtureEffect {
    fn label(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            action: None,
        }
    }
}

fn apply_surface_fixture_control(
    model: &mut SurfaceModel,
    control: SurfaceFixtureControl,
) -> Result<SurfaceFixtureEffect, EncyclopediaError> {
    match control {
        SurfaceFixtureControl::RequestMissingLanguage => {
            model.set_requested_language_for_fixture("1041");
            Ok(SurfaceFixtureEffect::label("request_language_1041"))
        }
        SurfaceFixtureControl::RequestDefaultLanguage => {
            model.set_requested_language_for_fixture("1033");
            Ok(SurfaceFixtureEffect::label("request_language_1033"))
        }
        SurfaceFixtureControl::RemoveSelectedAdmission => Ok(SurfaceFixtureEffect::label(
            model.replace_admission_for_fixture()?.map_or_else(
                || "remove_selected_admission:no_change".to_owned(),
                |topic| format!("remove_selected_admission:{topic}"),
            ),
        )),
        SurfaceFixtureControl::AttemptRemovedTopic => {
            let Some(topic_id) = model.take_removed_topic_for_fixture() else {
                return Ok(SurfaceFixtureEffect::label(
                    "attempt_removed_topic:no_target",
                ));
            };
            Ok(SurfaceFixtureEffect {
                label: format!("attempt_removed_topic:{topic_id}"),
                action: Some(EncyclopediaAction::SelectTopic(topic_id)),
            })
        }
        SurfaceFixtureControl::RestoreAdmissionSnapshot => {
            Ok(SurfaceFixtureEffect::label(format!(
                "restore_admission_snapshot:{}",
                if model.restore_admission_for_fixture() {
                    "applied"
                } else {
                    "no_change"
                }
            )))
        }
        SurfaceFixtureControl::OriginalProfile => {
            #[cfg(not(target_arch = "wasm32"))]
            let applied = model.set_asset_profile_for_fixture(AssetRenderProfile::OriginalParity);
            #[cfg(target_arch = "wasm32")]
            let applied = false;
            Ok(SurfaceFixtureEffect::label(format!(
                "profile_original:{}",
                if applied { "applied" } else { "unsupported" }
            )))
        }
        SurfaceFixtureControl::FaithfulHdProfile => {
            #[cfg(not(target_arch = "wasm32"))]
            let applied = model.set_asset_profile_for_fixture(AssetRenderProfile::FaithfulHd);
            #[cfg(target_arch = "wasm32")]
            let applied = false;
            Ok(SurfaceFixtureEffect::label(format!(
                "profile_faithful_hd:{}",
                if applied { "applied" } else { "unsupported" }
            )))
        }
    }
}

impl SurfaceModel {
    fn take_removed_topic_for_fixture(&mut self) -> Option<String> {
        match self {
            Self::Static(model) => model.removed_topic_for_fixture.take(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Live(model) => model.removed_topic_for_fixture.take(),
        }
    }

    fn viewer(&self) -> Option<ViewerFaction> {
        match self {
            Self::Static(model) => model.viewer(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Live(model) => Some(model.viewer()),
        }
    }

    fn selected_source_kind(&self, view: &EncyclopediaView) -> &'static str {
        let Some(active) = view.active_topic.as_ref() else {
            return "unavailable";
        };
        let Some(image) = active.image.as_ref() else {
            return "null";
        };
        if image.render_profile == TopicImageRenderProfile::FaithfulHdLinear {
            return "approved_hd";
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Self::Live(model) = self {
            return match model.selected_owner(&image.asset_id) {
                Some(EncyclopediaImageOwner::Mod { .. }) => "mod",
                Some(EncyclopediaImageOwner::Base { .. }) | None => "base",
            };
        }
        "base"
    }

    fn binding_for_topic(
        &self,
        topic_id: &str,
    ) -> Option<&rebellion_data::encyclopedia::BindingKey> {
        match self {
            Self::Static(model) => model.binding_for_topic(topic_id),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Live(model) => model.binding_for_topic(topic_id),
        }
    }

    fn catalog_generation(&self) -> Option<u64> {
        match self {
            Self::Static(model) => model.catalog_generation(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Live(model) => Some(model.generation()),
        }
    }

    fn requested_language(&self) -> &str {
        match self {
            Self::Static(model) => model.requested_language(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Live(model) => model.requested_language(),
        }
    }

    fn effective_language<'a>(&'a self, view: &EncyclopediaView) -> Option<&'a str> {
        match self {
            Self::Static(model) => model.effective_language(view),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Live(model) => model.effective_language(view),
        }
    }

    fn set_requested_language_for_fixture(&mut self, language: impl Into<String>) {
        let language = language.into();
        match self {
            Self::Static(model) => model.set_requested_language_for_fixture(language),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Live(model) => model.set_requested_language_for_fixture(language),
        }
    }

    fn replace_admission_for_fixture(&mut self) -> Result<Option<String>, EncyclopediaError> {
        match self {
            Self::Static(model) => model.replace_admission_for_fixture(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Live(model) => model.replace_admission_for_fixture(),
        }
    }

    fn restore_admission_for_fixture(&mut self) -> bool {
        match self {
            Self::Static(model) => model.restore_admission_for_fixture(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Live(model) => model.restore_admission_for_fixture(),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn set_asset_profile_for_fixture(&mut self, profile: AssetRenderProfile) -> bool {
        let Some(model) = self.live_model_mut() else {
            return false;
        };
        let hd_root = model.hd_root.clone();
        model.configure_images(profile, hd_root);
        true
    }

    fn ensure_initial_topic(&mut self) -> Result<(), EncyclopediaError> {
        match self {
            Self::Static(model) => model.ensure_initial_topic(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Live(model) => model.ensure_initial_topic(),
        }
    }

    fn build_view(&mut self) -> Result<Option<EncyclopediaView>, EncyclopediaError> {
        match self {
            Self::Static(model) => model.build_view(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Live(model) => model.build_view().map(Some),
        }
    }

    fn source_profile(&self) -> Option<&str> {
        match self {
            Self::Static(model) => model.source_profile(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Live(model) => model.source_profile(),
        }
    }

    fn unavailable_diagnostic(&self) -> Option<&str> {
        match self {
            Self::Static(model) => model.unavailable_diagnostic(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Live(model) => model.unavailable_diagnostic(),
        }
    }

    fn navigation_mut(&mut self) -> &mut EncyclopediaNavigationState {
        match self {
            Self::Static(model) => model.navigation_mut(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Live(model) => model.navigation_mut(),
        }
    }

    fn navigation(&self) -> &EncyclopediaNavigationState {
        match self {
            Self::Static(model) => model.navigation(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Live(model) => model.navigation(),
        }
    }

    fn apply_action_batch(
        &mut self,
        actions: impl IntoIterator<Item = EncyclopediaAction>,
    ) -> Result<Vec<NavigationOutcome>, EncyclopediaError> {
        match self {
            Self::Static(model) => model.apply_action_batch(actions),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Live(model) => model.apply_action_batch(actions),
        }
    }

    fn apply_action_to_view(
        &mut self,
        view: &EncyclopediaView,
        action: EncyclopediaAction,
    ) -> NavigationOutcome {
        match self {
            Self::Static(model) => model.apply_action(view, action),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Live(model) => apply_encyclopedia_action(&mut model.navigation, view, action),
        }
    }

    fn fail_presentation(&mut self, error: impl ToString) {
        match self {
            Self::Static(model) => {
                *model =
                    InspectorModel::new(InspectorContent::Unavailable(error.to_string()), "1033");
            }
            #[cfg(not(target_arch = "wasm32"))]
            Self::Live(model) => model.fail_presentation(error),
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn static_model(&self) -> &InspectorModel {
        let Self::Static(model) = self;
        model
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn live_model_mut(&mut self) -> Option<&mut LiveInspectorModel> {
        match self {
            Self::Static(_) => None,
            Self::Live(model) => Some(model),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn live_model(&self) -> Option<&LiveInspectorModel> {
        match self {
            Self::Static(_) => None,
            Self::Live(model) => Some(model),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct LiveRefreshContext<'a> {
    runtime: &'a mut rebellion_data::mods::ModRuntime,
    lifecycle: &'a mut EncyclopediaLifecycle,
    watcher: &'a mut crate::encyclopedia_watcher::EncyclopediaWatcher,
    fingerprint_source: &'a mut dyn FnMut() -> NativeSimulationFingerprints,
    fingerprints: NativeSimulationFingerprints,
    emit_report: bool,
    last_published_changed: bool,
    diagnostics: Vec<String>,
}

/// Synchronizes the feature fixture's file-authored desired set through the
/// same in-memory toggle API used by the real Mod Manager. Discovery must run
/// first because a harness step may install a manifest and enable it in one
/// atomic operation. This adapter deliberately has no world handle: its only
/// publication boundary is E50's content-only lifecycle entry.
#[cfg(not(target_arch = "wasm32"))]
fn sync_live_enabled_state_from_disk(
    runtime: &mut rebellion_data::mods::ModRuntime,
    lifecycle: &mut EncyclopediaLifecycle,
) -> Option<crate::encyclopedia_lifecycle::EncyclopediaContentRefresh> {
    let desired = rebellion_data::mods::ModConfig::load(&runtime.mods_dir);
    let current_names = runtime
        .config
        .enabled
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let desired_names = desired.enabled.iter().cloned().collect::<BTreeSet<_>>();
    if current_names == desired_names {
        return None;
    }

    runtime.refresh();
    let changed_names = current_names
        .symmetric_difference(&desired_names)
        .cloned()
        .collect::<Vec<_>>();
    for name in changed_names {
        runtime.toggle_mod(&name);
    }
    debug_assert_eq!(
        runtime
            .config
            .enabled
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>(),
        desired_names,
        "feature control must apply the exact desired enabled-name set"
    );

    let ordered = runtime.enabled_sorted();
    Some(lifecycle.refresh_content_only(&ordered))
}

#[cfg(not(target_arch = "wasm32"))]
fn record_live_refresh(
    model: &mut SurfaceModel,
    live: &mut LiveRefreshContext<'_>,
    refresh: crate::encyclopedia_lifecycle::EncyclopediaContentRefresh,
) {
    live.last_published_changed = refresh.published_changed;
    live.diagnostics = refresh
        .diagnostics
        .iter()
        .map(|diagnostic| {
            format!(
                "{}:{}:{}:{}",
                diagnostic.mod_name, diagnostic.code, diagnostic.path, diagnostic.message
            )
        })
        .collect();
    if let Some(live_model) = model.live_model_mut() {
        let _ = live_model.rebind_from_lifecycle(live.lifecycle);
    }
    live.emit_report = true;
}

#[cfg(not(target_arch = "wasm32"))]
#[expect(
    clippy::too_many_arguments,
    reason = "The feature-only route passes the already-owned runtime components without wrapping or duplicating them."
)]
pub(crate) async fn run_live<F>(
    gdata: &std::path::Path,
    source_profile: &str,
    runtime: &mut rebellion_data::mods::ModRuntime,
    lifecycle: &mut EncyclopediaLifecycle,
    watcher: &mut crate::encyclopedia_watcher::EncyclopediaWatcher,
    asset_profile: AssetRenderProfile,
    hd_root: Option<PathBuf>,
    mut fingerprint_source: F,
) where
    F: FnMut() -> NativeSimulationFingerprints,
{
    let (viewer, faction) =
        match native_viewer_faction(std::env::var_os(INSPECTOR_FACTION).as_deref()) {
            Ok(selection) => selection,
            Err(error) => {
                macroquad::logging::error!("[encyclopedia_inspector] {error}");
                return;
            }
        };
    let model = LiveInspectorModel::from_lifecycle(lifecycle, source_profile, "1033", viewer)
        .map(|mut model| {
            model.configure_images(asset_profile, hd_root);
            let _ = model.select_topic("original:5696".to_owned());
            model
        })
        .map_or_else(
            || {
                SurfaceModel::Static(InspectorModel::new(
                    InspectorContent::Unavailable(
                        "validated native encyclopedia session is unavailable".to_owned(),
                    ),
                    "1033",
                ))
            },
            SurfaceModel::Live,
        );
    let mut chrome = rebellion_render::BmpCache::new();
    chrome.set_base_path(gdata.join("ui"));
    let fingerprints = fingerprint_source();
    run_surface(
        model,
        chrome,
        faction,
        LiveRefreshContext {
            runtime,
            lifecycle,
            watcher,
            fingerprint_source: &mut fingerprint_source,
            fingerprints,
            emit_report: true,
            last_published_changed: false,
            diagnostics: Vec::new(),
        },
    )
    .await;
}

#[cfg(target_arch = "wasm32")]
pub(crate) async fn run_packed(availability: EncyclopediaAvailability, request: FixtureRequest) {
    run_browser(availability, request, BrowserEncyclopediaTransport::Packed).await;
}

#[cfg(target_arch = "wasm32")]
pub(crate) async fn run_loose(availability: EncyclopediaAvailability, request: FixtureRequest) {
    run_browser(availability, request, BrowserEncyclopediaTransport::Loose).await;
}

#[cfg(target_arch = "wasm32")]
async fn run_browser(
    availability: EncyclopediaAvailability,
    request: FixtureRequest,
    transport: BrowserEncyclopediaTransport,
) {
    let model = InspectorModel::from_browser_request(availability, "1033", request);
    run_surface(
        SurfaceModel::Static(model),
        rebellion_render::BmpCache::new(),
        request.faction,
        request,
        transport,
    )
    .await;
}

async fn run_surface(
    mut model: SurfaceModel,
    mut chrome: rebellion_render::BmpCache,
    faction: rebellion_render::CockpitFaction,
    #[cfg(not(target_arch = "wasm32"))] mut live: LiveRefreshContext<'_>,
    #[cfg(target_arch = "wasm32")] browser_request: FixtureRequest,
    #[cfg(target_arch = "wasm32")] browser_transport: BrowserEncyclopediaTransport,
) {
    use macroquad::prelude::{
        clear_background, is_key_pressed, is_quit_requested, next_frame, Color, KeyCode,
    };
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
    let mut observation_recorder = SurfaceObservationRecorder::default();
    #[cfg(target_arch = "wasm32")]
    let mut stable_frames = 0_u32;
    #[cfg(target_arch = "wasm32")]
    let mut report_emitted = false;

    while !close_requested {
        if is_quit_requested() {
            break;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            live.fingerprints = (live.fingerprint_source)();
            if let Some(refresh) = sync_live_enabled_state_from_disk(live.runtime, live.lifecycle) {
                record_live_refresh(&mut model, &mut live, refresh);
            }
            let outcome = live.watcher.poll_and_refresh(live.runtime, live.lifecycle);
            if let Some(refresh) = outcome.refresh {
                record_live_refresh(&mut model, &mut live, refresh);
            }
            if !outcome.diagnostics.is_empty() {
                live.diagnostics.extend(outcome.diagnostics);
                live.emit_report = true;
            }
        }
        clear_background(Color::from_rgba(9, 13, 22, 255));

        let mut fixture_actions = Vec::new();
        for (pressed, control) in [
            (
                is_key_pressed(KeyCode::F2),
                SurfaceFixtureControl::RequestMissingLanguage,
            ),
            (
                is_key_pressed(KeyCode::F3),
                SurfaceFixtureControl::RequestDefaultLanguage,
            ),
            (
                is_key_pressed(KeyCode::F4),
                SurfaceFixtureControl::RemoveSelectedAdmission,
            ),
            (
                is_key_pressed(KeyCode::F8),
                SurfaceFixtureControl::AttemptRemovedTopic,
            ),
            (
                is_key_pressed(KeyCode::F5),
                SurfaceFixtureControl::RestoreAdmissionSnapshot,
            ),
            (
                is_key_pressed(KeyCode::F6),
                SurfaceFixtureControl::OriginalProfile,
            ),
            (
                is_key_pressed(KeyCode::F7),
                SurfaceFixtureControl::FaithfulHdProfile,
            ),
        ] {
            if pressed {
                match apply_surface_fixture_control(&mut model, control) {
                    Ok(effect) => {
                        observation_recorder.push_fixture_control(effect.label);
                        fixture_actions.extend(effect.action);
                    }
                    Err(error) => {
                        observation_recorder
                            .push_fixture_control(format!("fixture_control_error:{error}"));
                        model.fail_presentation(error);
                    }
                }
            }
        }
        if !fixture_actions.is_empty() {
            match reduce_surface_actions(&mut model, &mut observation_recorder, fixture_actions) {
                Ok(requested) => close_requested |= requested,
                Err(error) => model.fail_presentation(error),
            }
        }

        if let Err(error) = model.ensure_initial_topic() {
            model.fail_presentation(error);
        }
        let source_profile = model.source_profile().map(str::to_owned);
        let unavailable = model.unavailable_diagnostic().map(str::to_owned);
        let view = match model.build_view() {
            Ok(view) => view,
            Err(error) => {
                model.fail_presentation(error);
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
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(live_model) = model.live_model() {
                    ui.label(format!("effective generation: {}", live_model.generation()));
                    ui.label(format!(
                        "retained bytes: {}",
                        live.lifecycle.retained_bytes_for_fixture()
                    ));
                    ui.label(format!(
                        "accepted mods: {:?}",
                        live.lifecycle.accepted_mod_names_for_fixture()
                    ));
                    ui.label(format!(
                        "world/save/rng: {} / {} / {}",
                        live.fingerprints.world, live.fingerprints.save, live.fingerprints.rng
                    ));
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
            let selected_source_kind = model.selected_source_kind(view.as_ref().unwrap());
            #[cfg(not(target_arch = "wasm32"))]
            let live_enabled_mods = live
                .runtime
                .enabled_mod_list()
                .into_iter()
                .map(|(name, version)| format!("{name}@{version}"))
                .collect();
            #[cfg(target_arch = "wasm32")]
            let live_enabled_mods = Vec::new();
            let observation_values = surface_observation_values(
                &model,
                view.as_ref().unwrap(),
                &surface,
                frame.keyboard_focus,
                &cache_status,
                selected_source_kind,
                live_enabled_mods,
            );
            observation_recorder
                .push_texture_events(frame.texture_events.iter().map(surface_texture_event));
            observation_recorder
                .push_input_attempts(frame.input_attempts.iter().map(surface_input_attempt));
            if let Some(values) = observation_values {
                if let Some(observation) = observation_recorder.emit_if_changed(&values) {
                    macroquad::logging::info!("[encyclopedia_surface_observation] {}", observation);
                }
            }

            let actions = std::mem::take(&mut frame.actions);
            match reduce_surface_actions(&mut model, &mut observation_recorder, actions) {
                Ok(requested) => close_requested |= requested,
                Err(error) => model.fail_presentation(error),
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if live.emit_report {
            emit_native_acceptance_report(
                &model,
                view.as_ref(),
                surface_frame.as_ref(),
                &cache_status,
                &live,
            );
            live.emit_report = false;
            live.last_published_changed = false;
            live.diagnostics.clear();
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
                let report = fixture_report(
                    browser_transport,
                    browser_request,
                    model.static_model(),
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

#[cfg(not(target_arch = "wasm32"))]
fn emit_native_acceptance_report(
    model: &SurfaceModel,
    view: Option<&EncyclopediaView>,
    frame: Option<&rebellion_render::EncyclopediaSurfaceFrame>,
    cache_status: &str,
    live: &LiveRefreshContext<'_>,
) {
    let live_model = model.live_model();
    let active = view.and_then(|view| view.active_topic.as_ref());
    let body_sha256 = active.and_then(|topic| {
        rebellion_render::inspect_encyclopedia_bytes(topic.body.as_bytes(), None)
            .ok()
            .map(|inspected| inspected.sha256)
    });
    let asset_id = frame.and_then(|frame| frame.active_asset_id.as_deref());
    let image_source = active
        .and_then(|topic| topic.image.as_ref())
        .filter(|image| image.render_profile == TopicImageRenderProfile::FaithfulHdLinear)
        .map(|_| "approved_hd")
        .or_else(|| {
            asset_id
                .and_then(|asset_id| live_model.and_then(|model| model.selected_owner(asset_id)))
                .map(|owner| match owner {
                    EncyclopediaImageOwner::Base { .. } => "base",
                    EncyclopediaImageOwner::Mod { .. } => "mod",
                })
        })
        .unwrap_or_else(|| {
            if active.is_some_and(|topic| topic.image.is_none()) {
                "null"
            } else {
                "unavailable"
            }
        });
    let texture_events = frame.map_or_else(Vec::new, |frame| {
        frame
            .texture_events
            .iter()
            .map(|event| format!("{event:?}"))
            .collect()
    });
    let accepted_mods = live.lifecycle.accepted_mod_names_for_fixture();
    let enabled_mods = live.runtime.enabled_mod_list();
    let dependency_diagnostics = live
        .runtime
        .errors
        .iter()
        .map(|error| {
            let code = match error {
                rebellion_data::mods::ModError::MissingDependency { .. } => "missing_dependency",
                rebellion_data::mods::ModError::VersionMismatch { .. } => "version_mismatch",
                rebellion_data::mods::ModError::ParseError { .. } => "mod_parse_error",
                rebellion_data::mods::ModError::LoadOrder { .. } => "load_order_error",
            };
            format!("{}:{code}:{error}", error.mod_name())
        })
        .collect::<Vec<_>>();
    let report = serde_json::json!({
        "schema_version": 1,
        "synthetic_test_only": true,
        "status": if active.is_some() { "ready" } else { "unavailable" },
        "source_profile": model.source_profile(),
        "generation": live_model.map(LiveInspectorModel::generation),
        "topic_id": active.map(|topic| topic.topic_id.as_str()),
        "title": active.map(|topic| topic.title.as_ref()),
        "body_sha256": body_sha256,
        "image_source": image_source,
        "asset_id": asset_id,
        "digest": frame.and_then(|frame| frame.active_digest.as_deref()),
        "cache_status": cache_status,
        "texture_events": texture_events,
        "live_texture_entries": usize::from(
            asset_id.is_some() && frame.is_some_and(|frame| frame.texture_diagnostic.is_none())
        ),
        "retained_bytes": live.lifecycle.retained_bytes_for_fixture(),
        "accepted_mods": accepted_mods,
        "enabled_mods": enabled_mods,
        "published_changed": live.last_published_changed,
        "diagnostics": live.diagnostics,
        "dependency_diagnostics": dependency_diagnostics,
        "world_fingerprint": live.fingerprints.world,
        "save_fingerprint": live.fingerprints.save,
        "rng_fingerprint": live.fingerprints.rng,
    });
    macroquad::logging::info!(
        "[encyclopedia_native_acceptance] {}",
        serde_json::to_string(&report).expect("native acceptance report is serializable")
    );
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::{BTreeMap, HashMap};
    use std::ffi::OsStr;
    use std::fs;
    use std::path::PathBuf;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    use rebellion_data::encyclopedia::{AdmissionFact, ViewerFaction};
    use rebellion_data::mods::{ModConfig, ModManifest, ModRuntime, ModWatchPoll};
    use rebellion_render::{
        draw_encyclopedia_surface, inspect_encyclopedia_bytes, BmpCache, BodyScrollIntent,
        CockpitFaction, EguiEncyclopediaTextureBackend, EncyclopediaAction, EncyclopediaMode,
        EncyclopediaSurfaceState, EncyclopediaTextureBackend, EncyclopediaTextureCache,
        EncyclopediaTextureEvent, EncyclopediaTextureUpload, NavigationOutcome,
        NavigationRejection, SelectionForce, SourceKeyIntent, TopicImageRenderProfile,
    };
    use serde_json::Value;

    use super::{
        apply_surface_fixture_control, content_sha256, fingerprint_serializable, fitted_art_size,
        fixture_report, inspection_admission, native_viewer_faction, navigation_outcome_name,
        packed_fixture_report, record_live_refresh, reduce_surface_actions, request_value_enabled,
        requested, selection_evidence, surface_controller_state, surface_observation_values,
        surface_texture_event, sync_live_enabled_state_from_disk, texture_cache_event_name,
        viewport_evidence, BrowserEncyclopediaTransport, InspectorContent, InspectorModel,
        InspectorTextureState, LiveInspectorModel, LiveRefreshContext,
        NativeSimulationFingerprints, SurfaceControllerStep, SurfaceFixtureControl,
        SurfaceFixtureEffect, SurfaceModel, SurfaceObservation, SurfaceObservationRecorder,
        SurfaceTextureEvent, SurfaceTransition, INSPECTOR_REQUEST,
    };
    use crate::encyclopedia_lifecycle::EncyclopediaLifecycle;
    use crate::encyclopedia_session::{
        prepare_encyclopedia_session, EncyclopediaAvailability, EncyclopediaBytes,
        EncyclopediaSession,
    };
    use crate::encyclopedia_watcher::EncyclopediaWatcher;
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

    static NEXT_LIVE_TEMP: AtomicU64 = AtomicU64::new(0);

    struct LiveModRoot(PathBuf);

    impl LiveModRoot {
        fn new() -> Self {
            let serial = NEXT_LIVE_TEMP.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "open-rebellion-e26-live-{}-{serial}",
                std::process::id()
            ));
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }

        fn write_overlay(&self, bytes: &[u8]) {
            fs::write(self.0.join("encyclopedia.json"), bytes).unwrap();
        }

        fn write_asset(&self, name: &str, bytes: &[u8]) {
            let assets = self.0.join("encyclopedia/assets");
            fs::create_dir_all(&assets).unwrap();
            fs::write(assets.join(name), bytes).unwrap();
        }

        fn manifest(&self) -> ModManifest {
            ModManifest {
                name: "e26-live".to_owned(),
                version: "1.0.0".to_owned(),
                author: "Synthetic E26 test".to_owned(),
                description: "Feature-only live inspector regression".to_owned(),
                dependencies: HashMap::new(),
                path: self.0.clone(),
                enabled: true,
            }
        }
    }

    impl Drop for LiveModRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn write_live_discovered_mod(
        mods_root: &std::path::Path,
        directory: &str,
        name: &str,
        dependencies: &[(&str, &str)],
        overlay: &[u8],
    ) {
        let root = mods_root.join(directory);
        fs::create_dir_all(&root).unwrap();
        let mut manifest = format!("name = \"{name}\"\nversion = \"1.0.0\"\n");
        if !dependencies.is_empty() {
            manifest.push_str("[dependencies]\n");
            for (dependency, requirement) in dependencies {
                manifest.push_str(&format!("\"{dependency}\" = \"{requirement}\"\n"));
            }
        }
        fs::write(root.join("mod.toml"), manifest).unwrap();
        fs::write(root.join("encyclopedia.json"), overlay).unwrap();
    }

    fn write_live_enabled_config(mods_root: &std::path::Path, names: &[&str]) {
        ModConfig {
            enabled: names.iter().map(|name| (*name).to_owned()).collect(),
        }
        .save(mods_root)
        .unwrap();
    }

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
    fn loose_requests_preserve_viewer_art_and_use_the_loose_transport_label() {
        for (code, viewer, expected_asset, expected_digest, expected_faction) in [
            (
                0x0131,
                ViewerFaction::Alliance,
                "edata:2",
                "a93a4e651a970119d8da0386846785163291b7cfd67edaac7f6fc37b719fe592",
                "alliance",
            ),
            (
                0x0231,
                ViewerFaction::Empire,
                "edata:3",
                "8746c347d4cf14daa2e0cc9d41d1f9abebbbb1aae997f31a172610e9ecdbe2dd",
                "empire",
            ),
        ] {
            let request = crate::interface_test_fixture::decode_request(code).unwrap();
            let mut model = InspectorModel::from_browser_request(
                EncyclopediaAvailability::Ready(session()),
                "1033",
                request,
            );
            assert_eq!(model.viewer(), Some(viewer));
            model.select_topic("original:60004".to_owned()).unwrap();
            let view = model.build_view().unwrap().unwrap();
            let image = view.active_topic.as_ref().unwrap().image.as_ref().unwrap();

            let report = fixture_report(
                BrowserEncyclopediaTransport::Loose,
                request,
                &model,
                Some(&view),
                Some(image.asset_id.as_str()),
                Some(image.digest.as_str()),
                "cache hit",
                None,
            );

            assert_eq!(report.faction(), expected_faction);
            assert_eq!(report.surface, "loose-encyclopedia-fixture");
            assert_eq!(report.asset_id.as_deref(), Some(expected_asset));
            assert_eq!(report.digest.as_deref(), Some(expected_digest));
        }
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
        let model = SurfaceModel::Static(model);
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
        let model = SurfaceModel::Static(model);
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
    fn feature_controls_publish_typed_admission_and_whole_language_without_catalog_reload() {
        let mut model = InspectorModel::new(InspectorContent::Ready(Box::new(session())), "1033");
        model.select_topic("original:60001".to_owned()).unwrap();
        let before = model.build_view().unwrap().unwrap();
        let generation = model.catalog_generation().unwrap();
        let selected = before.navigation.selected_topic_id.clone().unwrap();
        let before_ids = before
            .topics
            .iter()
            .map(|topic| topic.topic_id.clone())
            .collect::<Vec<_>>();

        model.set_requested_language_for_fixture("1041");
        let fallback = model.build_view().unwrap().unwrap();
        assert_eq!(model.requested_language(), "1041");
        assert_eq!(model.effective_language(&fallback), Some("1033"));
        assert_eq!(
            fallback.navigation.selected_topic_id.as_deref(),
            Some(selected.as_str())
        );
        assert_eq!(model.catalog_generation(), Some(generation));

        assert_eq!(
            model.replace_admission_for_fixture(),
            Ok(Some(selected.clone()))
        );
        let replaced = model.build_view().unwrap().unwrap();
        assert_eq!(
            replaced.navigation.world_epoch,
            before.navigation.world_epoch + 1
        );
        assert_eq!(model.catalog_generation(), Some(generation));
        assert_eq!(replaced.topics.len() + 1, before_ids.len());
        assert!(!replaced
            .topics
            .iter()
            .any(|topic| topic.topic_id == selected));

        assert!(model.restore_admission_for_fixture());
        let restored = model.build_view().unwrap().unwrap();
        assert_eq!(
            restored.navigation.world_epoch,
            replaced.navigation.world_epoch + 1
        );
        assert_eq!(model.catalog_generation(), Some(generation));
        assert_eq!(restored.topics.len(), before_ids.len());
        assert!(restored
            .topics
            .iter()
            .any(|topic| topic.topic_id == selected));
    }

    #[test]
    fn structured_surface_observation_binds_actual_controller_and_selected_image_state() {
        let mut model = InspectorModel::new_for_viewer(
            InspectorContent::Ready(Box::new(session())),
            "1041",
            ViewerFaction::Empire,
        );
        model.select_topic("original:60004".to_owned()).unwrap();
        let view = model.build_view().unwrap().unwrap();
        let mut model = SurfaceModel::Static(model);
        let values = surface_observation_values(
            &model,
            &view,
            &EncyclopediaSurfaceState::default(),
            Some(rebellion_render::encyclopedia::EncyclopediaSurfaceFocus {
                target_kind:
                    rebellion_render::encyclopedia::EncyclopediaSurfaceFocusKind::IndexList,
                target_id: 42,
                focused_id: Some(42),
                owns_focus: true,
            }),
            "cache hit",
            "base",
            Vec::new(),
        )
        .unwrap();

        assert_eq!(values.viewer_faction, "empire");
        assert_eq!(values.mode, "index");
        assert_eq!(values.focused_control.as_ref().unwrap().kind, "index_list");
        assert_eq!(values.focused_control.as_ref().unwrap().target_id, 42);
        assert_eq!(
            values.focused_control.as_ref().unwrap().focused_id,
            Some(42)
        );
        assert!(values.focused_control.as_ref().unwrap().owns_focus);
        assert_eq!(
            values.world_evidence_kind,
            "catalog_scoped_synthetic_admission"
        );
        assert_eq!(values.selected_category_command.as_deref(), Some("0x6f"));
        assert_eq!(values.selected_topic_id.as_deref(), Some("original:60004"));
        assert_eq!(values.requested_language, "1041");
        assert_eq!(values.effective_language.as_deref(), Some("1033"));
        let expected_title_sha256 = content_sha256(&view.active_topic.as_ref().unwrap().title);
        let expected_body_sha256 = content_sha256(&view.active_topic.as_ref().unwrap().body);
        assert_eq!(
            values.title_sha256.as_deref(),
            Some(expected_title_sha256.as_str())
        );
        assert_eq!(
            values.body_sha256.as_deref(),
            Some(expected_body_sha256.as_str())
        );
        assert_eq!(values.asset_id.as_deref(), Some("edata:3"));
        assert_eq!(
            values.asset_digest.as_deref(),
            Some("8746c347d4cf14daa2e0cc9d41d1f9abebbbb1aae997f31a172610e9ecdbe2dd")
        );
        assert_eq!(values.render_profile, Some("original_nearest"));
        assert_eq!(values.selected_source_kind, "base");
        assert_eq!(values.texture_cache_event, "cache_hit");

        let before = surface_controller_state(model.navigation(), &view);
        let outcome = model
            .apply_action_to_view(&view, EncyclopediaAction::SourceKey(SourceKeyIntent::Enter));
        let after_view = model.build_view().unwrap().unwrap();
        let after = surface_controller_state(model.navigation(), &after_view);
        let after_values = surface_observation_values(
            &model,
            &after_view,
            &EncyclopediaSurfaceState::default(),
            None,
            "cache hit",
            "base",
            Vec::new(),
        )
        .unwrap();
        let steps = [SurfaceControllerStep {
            input_record_id: Some("surface-00000001".to_owned()),
            action: "SourceKey(Enter)".to_owned(),
            outcome: navigation_outcome_name(outcome).to_owned(),
            before,
            after,
        }];
        let controls = ["request_language_1041".to_owned()];
        let texture_events = [SurfaceTextureEvent {
            kind: "released",
            asset_id: "edata:2".to_owned(),
            digest: Some("b".repeat(64)),
            render_profile: Some("original_nearest"),
            cache_hit: None,
            diagnostic: None,
        }];
        let input_attempts = [super::surface_input_attempt(
            &rebellion_render::encyclopedia::EncyclopediaSurfaceInputAttempt {
                control:
                    rebellion_render::encyclopedia::EncyclopediaSurfaceInputControl::PreviousTopic,
                enabled: false,
            },
        )];
        let observation = SurfaceObservation {
            schema_version: 5,
            status: "surface_observation",
            record_id: "surface-00000002".to_owned(),
            sequence: 2,
            values: &after_values,
            transition: SurfaceTransition {
                from_record_id: Some("surface-00000001"),
                controller_steps: &steps,
                fixture_controls: &controls,
                input_attempts: &input_attempts,
                texture_events: &texture_events,
            },
        };
        let serialized = serde_json::to_value(observation).unwrap();
        assert_eq!(serialized["schema_version"], 5);
        assert_eq!(
            serialized["values"]["visible_topic_ids"],
            serde_json::json!([
                "original:60001",
                "original:60002",
                "original:60003",
                "original:60004",
                "original:60005",
                "original:60006",
                "original:60007"
            ])
        );
        assert_eq!(
            serialized["transition"]["controller_steps"][0]["before"]["visible_topic_ids"],
            serde_json::json!([
                "original:60001",
                "original:60002",
                "original:60003",
                "original:60004",
                "original:60005",
                "original:60006",
                "original:60007"
            ])
        );
        assert_eq!(
            serialized["transition"]["from_record_id"],
            "surface-00000001"
        );
        assert_eq!(
            serialized["transition"]["controller_steps"][0]["action"],
            "SourceKey(Enter)"
        );
        assert_eq!(
            serialized["transition"]["controller_steps"][0]["input_record_id"],
            "surface-00000001"
        );
        assert_eq!(
            serialized["transition"]["controller_steps"][0]["before"]["mode"],
            "index"
        );
        assert_eq!(
            serialized["transition"]["controller_steps"][0]["after"]["mode"],
            "topic"
        );
        assert_eq!(
            serialized["transition"]["texture_events"][0]["kind"],
            "released"
        );
        assert_eq!(
            serialized["transition"]["input_attempts"],
            serde_json::json!([{
                "control": "previous_topic",
                "enabled": false,
            }])
        );
    }

    #[test]
    fn surface_observation_maps_exact_category_cache_and_controller_values() {
        assert_eq!(texture_cache_event_name("not selected"), "not_selected");
        assert_eq!(texture_cache_event_name("uploaded"), "uploaded");
        assert_eq!(texture_cache_event_name("cache hit"), "cache_hit");
        assert_eq!(texture_cache_event_name("released"), "released");
        assert_eq!(texture_cache_event_name("no art"), "no_art");
        assert_eq!(texture_cache_event_name("failed"), "failed");
        assert_eq!(texture_cache_event_name("unexpected"), "failed");
        assert_eq!(
            surface_texture_event(&EncyclopediaTextureEvent::Released {
                asset_id: "edata:34".to_owned(),
                digest: "a".repeat(64),
                profile: TopicImageRenderProfile::OriginalNearest,
            }),
            SurfaceTextureEvent {
                kind: "released",
                asset_id: "edata:34".to_owned(),
                digest: Some("a".repeat(64)),
                render_profile: Some("original_nearest"),
                cache_hit: None,
                diagnostic: None,
            }
        );
        assert_eq!(
            navigation_outcome_name(NavigationOutcome::Applied),
            "applied"
        );
        assert_eq!(
            navigation_outcome_name(NavigationOutcome::NoChange),
            "no_change"
        );
        assert_eq!(
            navigation_outcome_name(NavigationOutcome::Rejected(
                NavigationRejection::UnavailableTopic,
            )),
            "rejected"
        );
        assert_eq!(
            navigation_outcome_name(NavigationOutcome::ScrollRequested(
                BodyScrollIntent::PageDown,
            )),
            "scroll_requested"
        );
        assert_eq!(
            navigation_outcome_name(NavigationOutcome::CloseRequested),
            "close_requested"
        );
        assert_eq!(
            navigation_outcome_name(NavigationOutcome::ReturnForwarded),
            "return_forwarded"
        );

        let mut model = InspectorModel::new(InspectorContent::Ready(Box::new(session())), "1033");
        model
            .select_category(Some("command:0x70".to_owned()))
            .unwrap();
        model.select_topic("original:60001".to_owned()).unwrap();
        let view = model.build_view().unwrap().unwrap();
        let model = SurfaceModel::Static(model);
        let values = surface_observation_values(
            &model,
            &view,
            &EncyclopediaSurfaceState::default(),
            Some(rebellion_render::encyclopedia::EncyclopediaSurfaceFocus {
                target_kind:
                    rebellion_render::encyclopedia::EncyclopediaSurfaceFocusKind::IndexList,
                target_id: 9,
                focused_id: Some(77),
                owns_focus: false,
            }),
            "uploaded",
            "base",
            vec!["mod@1.0.0".to_owned()],
        )
        .unwrap();
        assert_eq!(values.selected_category_command.as_deref(), Some("0x70"));
        assert_eq!(values.texture_cache_event, "uploaded");
        assert_eq!(values.live_enabled_mods, ["mod@1.0.0"]);
        assert_eq!(values.focused_control.as_ref().unwrap().target_id, 9);
        assert_eq!(
            values.focused_control.as_ref().unwrap().focused_id,
            Some(77)
        );
        assert!(!values.focused_control.as_ref().unwrap().owns_focus);
    }

    #[test]
    fn feature_control_labels_describe_the_exact_applied_static_transition() {
        let mut model = InspectorModel::new(InspectorContent::Ready(Box::new(session())), "1033");
        model.select_topic("original:60001".to_owned()).unwrap();
        let mut model = SurfaceModel::Static(model);

        assert_eq!(
            super::apply_surface_fixture_control(
                &mut model,
                SurfaceFixtureControl::RequestMissingLanguage,
            ),
            Ok(SurfaceFixtureEffect::label("request_language_1041"))
        );
        assert_eq!(model.requested_language(), "1041");
        assert_eq!(
            super::apply_surface_fixture_control(
                &mut model,
                SurfaceFixtureControl::RequestDefaultLanguage,
            ),
            Ok(SurfaceFixtureEffect::label("request_language_1033"))
        );
        assert_eq!(model.requested_language(), "1033");
        assert_eq!(
            super::apply_surface_fixture_control(
                &mut model,
                SurfaceFixtureControl::RemoveSelectedAdmission,
            ),
            Ok(SurfaceFixtureEffect::label(
                "remove_selected_admission:original:60001"
            ))
        );
        assert_eq!(
            super::apply_surface_fixture_control(
                &mut model,
                SurfaceFixtureControl::AttemptRemovedTopic,
            ),
            Ok(SurfaceFixtureEffect {
                label: "attempt_removed_topic:original:60001".to_owned(),
                action: Some(EncyclopediaAction::SelectTopic("original:60001".to_owned())),
            })
        );
        assert_eq!(
            super::apply_surface_fixture_control(
                &mut model,
                SurfaceFixtureControl::RestoreAdmissionSnapshot,
            ),
            Ok(SurfaceFixtureEffect::label(
                "restore_admission_snapshot:applied"
            ))
        );
        assert_eq!(
            super::apply_surface_fixture_control(
                &mut model,
                SurfaceFixtureControl::OriginalProfile,
            ),
            Ok(SurfaceFixtureEffect::label("profile_original:unsupported"))
        );
        assert_eq!(
            super::apply_surface_fixture_control(
                &mut model,
                SurfaceFixtureControl::FaithfulHdProfile,
            ),
            Ok(SurfaceFixtureEffect::label(
                "profile_faithful_hd:unsupported"
            ))
        );
    }

    #[test]
    fn removed_selected_admission_still_emits_a_total_surface_observation() {
        let mut model = InspectorModel::new(InspectorContent::Ready(Box::new(session())), "1033");
        model.select_topic("original:60001".to_owned()).unwrap();
        let initial_epoch = model.build_view().unwrap().unwrap().navigation.world_epoch;
        assert_eq!(
            model.replace_admission_for_fixture(),
            Ok(Some("original:60001".to_owned()))
        );
        let removed = model.build_view().unwrap().unwrap();
        let model = SurfaceModel::Static(model);

        let values = surface_observation_values(
            &model,
            &removed,
            &EncyclopediaSurfaceState::default(),
            None,
            "no art",
            "unavailable",
            Vec::new(),
        )
        .expect("the removed state must remain observable");
        let serialized = serde_json::to_value(values).unwrap();

        assert!(serialized["selected_topic_id"].is_null());
        assert!(serialized["effective_language"].is_null());
        assert!(serialized["title_sha256"].is_null());
        assert!(serialized["body_sha256"].is_null());
        assert!(serialized["asset_id"].is_null());
        assert!(serialized["asset_digest"].is_null());
        assert_eq!(serialized["selected_source_kind"], "unavailable");
        assert_eq!(serialized["world_epoch"], initial_epoch + 1);
    }

    #[test]
    fn surface_recorder_emits_remove_reject_restore_then_explicit_reselect_in_order() {
        fn emit(
            model: &mut SurfaceModel,
            recorder: &mut SurfaceObservationRecorder,
        ) -> serde_json::Value {
            let view = model.build_view().unwrap().unwrap();
            let values = surface_observation_values(
                model,
                &view,
                &EncyclopediaSurfaceState::default(),
                None,
                "not selected",
                model.selected_source_kind(&view),
                Vec::new(),
            )
            .unwrap();
            serde_json::from_str(&recorder.emit_if_changed(&values).unwrap()).unwrap()
        }

        let mut model = SurfaceModel::Static(InspectorModel::new(
            InspectorContent::Ready(Box::new(session())),
            "1033",
        ));
        model.ensure_initial_topic().unwrap();
        let selected = model
            .build_view()
            .unwrap()
            .unwrap()
            .navigation
            .selected_topic_id
            .clone()
            .unwrap();
        let mut recorder = SurfaceObservationRecorder::default();
        let initial = emit(&mut model, &mut recorder);

        let removed_effect = apply_surface_fixture_control(
            &mut model,
            SurfaceFixtureControl::RemoveSelectedAdmission,
        )
        .unwrap();
        assert!(removed_effect.action.is_none());
        recorder.push_fixture_control(removed_effect.label.clone());
        model.ensure_initial_topic().unwrap();
        let removed = emit(&mut model, &mut recorder);

        let stale_effect =
            apply_surface_fixture_control(&mut model, SurfaceFixtureControl::AttemptRemovedTopic)
                .unwrap();
        recorder.push_fixture_control(stale_effect.label.clone());
        assert!(!reduce_surface_actions(&mut model, &mut recorder, stale_effect.action,).unwrap());
        let rejected = emit(&mut model, &mut recorder);

        let restored_effect = apply_surface_fixture_control(
            &mut model,
            SurfaceFixtureControl::RestoreAdmissionSnapshot,
        )
        .unwrap();
        assert!(restored_effect.action.is_none());
        recorder.push_fixture_control(restored_effect.label.clone());
        model.ensure_initial_topic().unwrap();
        let restored = emit(&mut model, &mut recorder);

        assert!(!reduce_surface_actions(
            &mut model,
            &mut recorder,
            [EncyclopediaAction::SelectTopic(selected.clone())],
        )
        .unwrap());
        let reselected = emit(&mut model, &mut recorder);

        assert_eq!(initial["sequence"], 1);
        assert_eq!(removed["sequence"], 2);
        assert_eq!(removed["values"]["mode"], "index");
        assert!(removed["values"]["selected_topic_id"].is_null());
        assert_eq!(
            removed["transition"]["fixture_controls"],
            serde_json::json!([removed_effect.label])
        );
        assert_eq!(rejected["sequence"], 3);
        assert_eq!(
            rejected["transition"]["controller_steps"][0]["outcome"],
            "rejected"
        );
        assert_eq!(
            rejected["transition"]["fixture_controls"],
            serde_json::json!([stale_effect.label])
        );
        assert_eq!(
            rejected["transition"]["controller_steps"][0]["before"],
            rejected["transition"]["controller_steps"][0]["after"]
        );
        assert_eq!(restored["sequence"], 4);
        assert!(restored["values"]["selected_topic_id"].is_null());
        assert_eq!(
            restored["transition"]["fixture_controls"],
            serde_json::json!([restored_effect.label])
        );
        assert_eq!(reselected["sequence"], 5);
        assert_eq!(reselected["values"]["selected_topic_id"], selected.as_str());
        assert_eq!(
            reselected["transition"]["controller_steps"][0]["outcome"],
            "applied"
        );
        let final_view = model.build_view().unwrap().unwrap();
        let final_values = surface_observation_values(
            &model,
            &final_view,
            &EncyclopediaSurfaceState::default(),
            None,
            "not selected",
            model.selected_source_kind(&final_view),
            Vec::new(),
        )
        .unwrap();
        assert!(recorder.emit_if_changed(&final_values).is_none());
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
    fn native_acceptance_fingerprint_is_deterministic_and_content_sensitive() {
        let initial = fingerprint_serializable(&vec![1_u32, 2, 3]);

        assert_eq!(initial, fingerprint_serializable(&vec![1_u32, 2, 3]));
        assert_ne!(initial, fingerprint_serializable(&vec![1_u32, 2, 4]));
        assert!(initial.starts_with("fnv1a64:"));
        assert_eq!(initial.len(), "fnv1a64:".len() + 16);
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
    fn surface_reducer_records_the_rebuilt_category_projection_and_selection_clear() {
        fn emit(
            model: &mut SurfaceModel,
            recorder: &mut SurfaceObservationRecorder,
        ) -> serde_json::Value {
            let view = model.build_view().unwrap().unwrap();
            let values = surface_observation_values(
                model,
                &view,
                &EncyclopediaSurfaceState::default(),
                None,
                "not selected",
                model.selected_source_kind(&view),
                Vec::new(),
            )
            .unwrap();
            serde_json::from_str(&recorder.emit_if_changed(&values).unwrap()).unwrap()
        }

        let mut static_model =
            InspectorModel::new(InspectorContent::Ready(Box::new(session())), "1033");
        static_model
            .select_topic("original:60002".to_owned())
            .unwrap();
        let mut model = SurfaceModel::Static(static_model);
        let mut recorder = SurfaceObservationRecorder::default();
        let initial = emit(&mut model, &mut recorder);

        assert!(!reduce_surface_actions(
            &mut model,
            &mut recorder,
            [EncyclopediaAction::SelectCategory {
                category_id: Some("command:0x70".to_owned()),
                force: SelectionForce::Normal,
            }],
        )
        .unwrap());
        let category = emit(&mut model, &mut recorder);

        assert_eq!(initial["values"]["selected_topic_id"], "original:60002");
        assert_eq!(category["values"]["selected_category_command"], "0x70");
        assert!(category["values"]["selected_topic_id"].is_null());
        assert_eq!(
            category["values"]["visible_topic_ids"],
            serde_json::json!(["original:60001"])
        );
        assert_eq!(
            category["transition"]["controller_steps"][0]["after"]["visible_topic_ids_sha256"],
            category["values"]["visible_topic_ids_sha256"]
        );
        assert_eq!(
            category["transition"]["controller_steps"][0]["outcome"],
            "applied"
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

    #[test]
    fn live_inspector_rebinds_the_real_effective_snapshot_without_losing_selection() {
        let root = LiveModRoot::new();
        let manifest = root.manifest();
        let mut lifecycle =
            EncyclopediaLifecycle::from_availability(EncyclopediaAvailability::Ready(session()));
        let mut model = LiveInspectorModel::from_lifecycle(
            &lifecycle,
            "e37-synthetic-v1",
            "1033",
            ViewerFaction::Alliance,
        )
        .expect("base effective snapshot");
        model.select_topic("original:60001".to_owned()).unwrap();
        let initial_generation = model.generation();

        root.write_asset("live.bmp", VALID_IMAGE_2);
        root.write_overlay(
            br#"[{"id":"original:60001","localized":{"1033":{"title":"Live title","image":{"path":"encyclopedia/assets/live.bmp"}}}}]"#,
        );
        let refresh = lifecycle.refresh_content_only(&[&manifest]);
        assert!(refresh.published_changed, "refresh: {refresh:?}");
        assert!(model.rebind_from_lifecycle(&lifecycle));

        let view = model.build_view().unwrap();
        assert_eq!(model.generation(), initial_generation + 1);
        assert_eq!(
            view.navigation.selected_topic_id.as_deref(),
            Some("original:60001")
        );
        assert_eq!(
            view.active_topic.as_ref().map(|topic| topic.title.as_ref()),
            Some("Live title")
        );
        let first_image = view
            .active_topic
            .as_ref()
            .and_then(|topic| topic.image.as_ref())
            .expect("accepted mod image");
        let first_asset_id = first_image.asset_id.clone();
        let first_digest = first_image.digest.clone();

        root.write_asset("live.bmp", VALID_IMAGE_3);
        let image_only = lifecycle.refresh_content_only(&[&manifest]);
        assert!(image_only.published_changed, "refresh: {image_only:?}");
        assert!(model.rebind_from_lifecycle(&lifecycle));
        let image_only_view = model.build_view().unwrap();
        let replacement = image_only_view
            .active_topic
            .as_ref()
            .and_then(|topic| topic.image.as_ref())
            .expect("accepted replacement image");
        assert_eq!(replacement.asset_id, first_asset_id);
        assert_ne!(replacement.digest, first_digest);
        assert_eq!(
            image_only_view.navigation.selected_topic_id.as_deref(),
            Some("original:60001")
        );

        root.write_overlay(br#"[{"#);
        let malformed = lifecycle.refresh_content_only(&[&manifest]);
        assert!(!malformed.published_changed, "refresh: {malformed:?}");
        assert!(!model.rebind_from_lifecycle(&lifecycle));
        assert_eq!(
            model
                .build_view()
                .unwrap()
                .active_topic
                .as_ref()
                .map(|topic| topic.title.as_ref()),
            Some("Live title"),
            "an eligible malformed edit must retain the last accepted publication"
        );

        let disabled = lifecycle.refresh_content_only(&[]);
        assert!(disabled.published_changed, "refresh: {disabled:?}");
        assert!(model.rebind_from_lifecycle(&lifecycle));
        let restored = model.build_view().unwrap();
        assert_eq!(
            restored.navigation.selected_topic_id.as_deref(),
            Some("original:60001")
        );
        assert_eq!(
            restored
                .active_topic
                .as_ref()
                .map(|topic| topic.title.as_ref()),
            Some("Amber system"),
            "disabling the mod must rebuild from immutable base without losing selection"
        );
    }

    #[test]
    fn live_surface_restores_a_removed_typed_admission_in_a_new_epoch() {
        let lifecycle =
            EncyclopediaLifecycle::from_availability(EncyclopediaAvailability::Ready(session()));
        let model = LiveInspectorModel::from_lifecycle(
            &lifecycle,
            "e30-synthetic-v1",
            "1033",
            ViewerFaction::Alliance,
        )
        .expect("base effective snapshot");
        let mut model = SurfaceModel::Live(model);
        model.ensure_initial_topic().unwrap();
        let before = model.build_view().unwrap().unwrap();
        let selected = before.navigation.selected_topic_id.clone().unwrap();
        let before_len = before.topics.len();

        assert_eq!(
            model.replace_admission_for_fixture(),
            Ok(Some(selected.clone()))
        );
        let removed = model.build_view().unwrap().unwrap();
        assert_eq!(removed.topics.len() + 1, before_len);
        assert!(!removed
            .topics
            .iter()
            .any(|topic| topic.topic_id == selected));

        assert_eq!(
            super::apply_surface_fixture_control(
                &mut model,
                SurfaceFixtureControl::RestoreAdmissionSnapshot,
            ),
            Ok(SurfaceFixtureEffect::label(
                "restore_admission_snapshot:applied"
            ))
        );
        let restored = model.build_view().unwrap().unwrap();
        assert_eq!(restored.topics.len(), before_len);
        assert_eq!(
            restored.navigation.world_epoch,
            removed.navigation.world_epoch + 1
        );
        assert!(restored
            .topics
            .iter()
            .any(|topic| topic.topic_id == selected));

        // Keep the lifecycle live for the whole test so the model's accepted
        // snapshot remains tied to an authoritative owner.
        assert!(lifecycle.effective_snapshot_for_fixture().is_some());
    }

    #[test]
    fn dependency_failure_uses_the_real_empty_resolved_order_and_restores_base() {
        let root = LiveModRoot::new();
        root.write_overlay(
            br#"[{"id":"original:60001","localized":{"1033":{"title":"Eligible title"}}}]"#,
        );
        let eligible = root.manifest();
        let mut lifecycle =
            EncyclopediaLifecycle::from_availability(EncyclopediaAvailability::Ready(session()));
        let accepted = lifecycle.refresh_content_only(&[&eligible]);
        assert!(accepted.published_changed, "refresh: {accepted:?}");
        let mut model = LiveInspectorModel::from_lifecycle(
            &lifecycle,
            "e37-synthetic-v1",
            "1033",
            ViewerFaction::Alliance,
        )
        .expect("accepted effective snapshot");
        model.select_topic("original:60001".to_owned()).unwrap();

        let mut blocked = eligible.clone();
        blocked.name = "e26-needs-missing".to_owned();
        blocked.dependencies =
            HashMap::from([("e26-not-installed".to_owned(), ">=1.0.0".to_owned())]);
        let runtime = ModRuntime {
            discovered: vec![eligible, blocked],
            config: ModConfig::default(),
            errors: Vec::new(),
            mods_dir: root.0.clone(),
        };

        let ordered = runtime.enabled_sorted();
        assert!(
            ordered.is_empty(),
            "invalid enabled batch must not be partial"
        );
        let restored = lifecycle.refresh_content_only(&ordered);
        assert!(restored.published_changed, "refresh: {restored:?}");
        assert!(model.rebind_from_lifecycle(&lifecycle));
        let view = model.build_view().unwrap();
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
    fn live_fixture_syncs_file_enabled_state_through_runtime_toggles_in_one_process() {
        let mods = LiveModRoot::new();
        write_live_enabled_config(&mods.0, &[]);
        let mut runtime = ModRuntime::discover(&mods.0);
        let mut lifecycle =
            EncyclopediaLifecycle::from_availability(EncyclopediaAvailability::Ready(session()));
        let base_generation = lifecycle
            .effective_snapshot_for_fixture()
            .unwrap()
            .generation();
        let mut surface_model = SurfaceModel::Live(
            LiveInspectorModel::from_lifecycle(
                &lifecycle,
                "e37-synthetic-v1",
                "1033",
                ViewerFaction::Alliance,
            )
            .expect("base effective snapshot"),
        );

        write_live_discovered_mod(
            &mods.0,
            "alpha",
            "e26-alpha",
            &[],
            br#"[{"id":"original:60001","localized":{"1033":{"title":"Alpha live"}}}]"#,
        );
        write_live_discovered_mod(
            &mods.0,
            "beta",
            "e26-beta",
            &[("e26-alpha", ">=1.0.0")],
            br#"[{"id":"original:60001","localized":{"1033":{"title":"Beta live"}}}]"#,
        );
        write_live_enabled_config(&mods.0, &["e26-alpha", "e26-beta"]);

        let enabled = sync_live_enabled_state_from_disk(&mut runtime, &mut lifecycle)
            .expect("the changed desired set must produce one content-only refresh");
        assert!(enabled.published_changed, "refresh: {enabled:?}");
        {
            let mut report_watcher = EncyclopediaWatcher::from_test_polls(&mods.0, []);
            let mut fingerprint_source = || NativeSimulationFingerprints {
                world: "world-before".to_owned(),
                save: "save-before".to_owned(),
                rng: "rng-before".to_owned(),
            };
            let mut live = LiveRefreshContext {
                runtime: &mut runtime,
                lifecycle: &mut lifecycle,
                watcher: &mut report_watcher,
                fingerprint_source: &mut fingerprint_source,
                fingerprints: NativeSimulationFingerprints {
                    world: "world-before".to_owned(),
                    save: "save-before".to_owned(),
                    rng: "rng-before".to_owned(),
                },
                emit_report: false,
                last_published_changed: false,
                diagnostics: Vec::new(),
            };
            record_live_refresh(&mut surface_model, &mut live, enabled);
            assert!(live.emit_report);
            assert!(live.last_published_changed);
            assert!(live.diagnostics.is_empty());
        }
        assert_eq!(
            surface_model
                .live_model()
                .expect("feature model remains live")
                .generation(),
            base_generation + 1
        );
        surface_model
            .ensure_initial_topic()
            .expect("synchronized live selection");
        let synchronized_view = surface_model
            .build_view()
            .expect("synchronized live view")
            .expect("live content remains available");
        assert_eq!(
            synchronized_view
                .active_topic
                .as_ref()
                .map(|topic| topic.title.as_ref()),
            Some("Beta live")
        );
        assert_eq!(
            runtime.enabled_mod_list(),
            vec![
                ("e26-alpha".to_owned(), "1.0.0".to_owned()),
                ("e26-beta".to_owned(), "1.0.0".to_owned()),
            ]
        );
        let enabled_snapshot = lifecycle.effective_snapshot_for_fixture().unwrap();
        assert_eq!(enabled_snapshot.generation(), base_generation + 1);
        assert_eq!(
            enabled_snapshot.catalog().topics["original:60001"].localized["1033"].title,
            "Beta live"
        );

        fs::write(mods.0.join("beta/encyclopedia.json"), br#"[{"#).unwrap();
        let mut watcher = EncyclopediaWatcher::from_test_polls(
            &mods.0,
            [
                ModWatchPoll {
                    changed: true,
                    diagnostics: Vec::new(),
                },
                ModWatchPoll::default(),
                ModWatchPoll {
                    changed: true,
                    diagnostics: Vec::new(),
                },
                ModWatchPoll::default(),
            ],
        );
        assert!(watcher
            .poll_and_refresh_at_for_test(&mut runtime, &mut lifecycle, Duration::ZERO)
            .refresh
            .is_none());
        let malformed = watcher
            .poll_and_refresh_at_for_test(&mut runtime, &mut lifecycle, Duration::from_millis(101))
            .refresh
            .expect("the accepted quiet boundary must perform one content-only refresh");
        assert!(!malformed.published_changed, "refresh: {malformed:?}");
        assert_eq!(
            lifecycle
                .effective_snapshot_for_fixture()
                .unwrap()
                .generation(),
            base_generation + 1,
            "a malformed edit must retain the eligible last-good publication"
        );

        fs::write(
            mods.0.join("beta/encyclopedia.json"),
            br#"[{"id":"original:60001","localized":{"1033":{"title":"Beta recovered"}}}]"#,
        )
        .unwrap();
        assert!(watcher
            .poll_and_refresh_at_for_test(&mut runtime, &mut lifecycle, Duration::from_millis(200),)
            .refresh
            .is_none());
        let recovered = watcher
            .poll_and_refresh_at_for_test(&mut runtime, &mut lifecycle, Duration::from_millis(301))
            .refresh
            .expect("a valid edit after the quiet boundary must recover in the same runtime");
        assert!(recovered.published_changed, "refresh: {recovered:?}");
        assert_eq!(
            lifecycle
                .effective_snapshot_for_fixture()
                .unwrap()
                .catalog()
                .topics["original:60001"]
                .localized["1033"]
                .title,
            "Beta recovered"
        );

        write_live_discovered_mod(
            &mods.0,
            "blocked",
            "e26-needs-missing",
            &[("e26-not-installed", ">=1.0.0")],
            br#"[{"id":"original:60001","localized":{"1033":{"title":"MUST NOT APPEAR"}}}]"#,
        );
        write_live_enabled_config(&mods.0, &["e26-alpha", "e26-beta", "e26-needs-missing"]);
        let blocked = sync_live_enabled_state_from_disk(&mut runtime, &mut lifecycle)
            .expect("enabling a newly discovered blocked mod must refresh content");
        assert!(blocked.published_changed, "refresh: {blocked:?}");
        assert!(runtime.enabled_sorted().is_empty());
        assert!(runtime
            .errors
            .iter()
            .any(|error| matches!(error, rebellion_data::mods::ModError::MissingDependency { mod_name, dep_name }
                if mod_name == "e26-needs-missing" && dep_name == "e26-not-installed")));
        assert_eq!(
            lifecycle
                .effective_snapshot_for_fixture()
                .unwrap()
                .catalog()
                .topics["original:60001"]
                .localized["1033"]
                .title,
            "Amber system"
        );

        write_live_enabled_config(&mods.0, &[]);
        let disabled = sync_live_enabled_state_from_disk(&mut runtime, &mut lifecycle)
            .expect("disabling every mod must refresh back to base");
        assert!(
            !disabled.published_changed,
            "base is already published: {disabled:?}"
        );
        assert!(runtime.enabled_mod_list().is_empty());
        assert!(sync_live_enabled_state_from_disk(&mut runtime, &mut lifecycle).is_none());
    }
}
