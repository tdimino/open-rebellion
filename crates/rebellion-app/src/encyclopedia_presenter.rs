use std::collections::BTreeMap;
use std::sync::Arc;

use rebellion_data::encyclopedia::{
    resolve_admitted_topics, resolve_localized_label, AdmissionSnapshot, EncyclopediaError,
    ResolvedTopic, TopicId,
};
use rebellion_render::{
    ActiveTopicView, CategoryViewItem, EncyclopediaDiagnosticScope, EncyclopediaSelection,
    EncyclopediaView, EncyclopediaViewDiagnostic, NavigationState, TopicImageRenderProfile,
    TopicImageView, TopicViewItem,
};

use crate::encyclopedia_session::EncyclopediaSession;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct TextCacheKey {
    generation: u64,
    topic_id: String,
    language: String,
}

/// Owns only immutable presentation text. Admission membership, faction art,
/// navigation neighbors, and the world epoch are recomputed on every build.
#[derive(Debug, Default)]
pub struct EncyclopediaPresenter {
    cached_generation: Option<u64>,
    titles: BTreeMap<TextCacheKey, Arc<str>>,
    active_body: Option<(TextCacheKey, Arc<str>)>,
}

impl EncyclopediaPresenter {
    fn begin_generation(&mut self, generation: u64) {
        if self.cached_generation != Some(generation) {
            self.cached_generation = Some(generation);
            self.titles.clear();
            self.active_body = None;
        }
    }

    fn text_key(
        &self,
        session: &EncyclopediaSession,
        topic_id: &TopicId,
        language: &str,
    ) -> TextCacheKey {
        TextCacheKey {
            generation: session.generation(),
            topic_id: topic_id.0.clone(),
            language: language.to_owned(),
        }
    }

    fn title(
        &mut self,
        session: &EncyclopediaSession,
        topic_id: &TopicId,
        language: &str,
        title: &str,
    ) -> Arc<str> {
        let key = self.text_key(session, topic_id, language);
        self.titles
            .entry(key)
            .or_insert_with(|| Arc::from(title))
            .clone()
    }

    fn active_body(
        &mut self,
        session: &EncyclopediaSession,
        topic_id: &TopicId,
        language: &str,
        body: &str,
    ) -> Arc<str> {
        let key = self.text_key(session, topic_id, language);
        if let Some((cached_key, cached_body)) = &self.active_body {
            if cached_key == &key {
                return cached_body.clone();
            }
        }
        let body: Arc<str> = Arc::from(body);
        self.active_body = Some((key, body.clone()));
        body
    }

    pub fn build_encyclopedia_view(
        &mut self,
        session: &EncyclopediaSession,
        admission: Option<&AdmissionSnapshot>,
        language: &str,
        selection: &EncyclopediaSelection,
    ) -> Result<EncyclopediaView, EncyclopediaError> {
        self.begin_generation(session.generation());
        let catalog = session.effective_catalog();
        let mut diagnostics = Vec::new();

        let index_label = match resolve_localized_label(
            &catalog.index.labels,
            &catalog.default_language,
            language,
        ) {
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

        let categories: Vec<_> = catalog
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
            .collect();

        let (selected_category_id, registry_topic_ids): (Option<String>, &[TopicId]) =
            match selection.category_id.as_deref() {
                None if index_enabled => (None, &catalog.index.topic_ids),
                None => (None, &[]),
                Some(category_id) => match catalog
                    .categories
                    .iter()
                    .enumerate()
                    .find(|(_, category)| category.id == category_id)
                {
                    Some((index, category)) if categories[index].enabled => {
                        (Some(category.id.clone()), &category.topic_ids)
                    }
                    Some((_, category)) => (Some(category.id.clone()), &[]),
                    None if index_enabled => {
                        diagnostics.push(EncyclopediaViewDiagnostic {
                            code: "unknown_category_selection",
                            scope: EncyclopediaDiagnosticScope::Category {
                                category_id: category_id.to_owned(),
                            },
                        });
                        (None, &catalog.index.topic_ids)
                    }
                    None => (None, &[]),
                },
            };

        let resolved = resolve_admitted_topics(catalog, registry_topic_ids, admission, language)?;
        let Some(admission) = admission else {
            unreachable!("the resolver rejects missing admission before returning a view")
        };
        diagnostics.extend(resolved.diagnostics.iter().map(|diagnostic| {
            EncyclopediaViewDiagnostic {
                code: diagnostic.error.code(),
                scope: EncyclopediaDiagnosticScope::Topic {
                    topic_id: diagnostic.topic_id.0.clone(),
                },
            }
        }));

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
                title: self.title(session, topic.topic_id, language, &topic.localized.title),
            })
            .collect::<Vec<_>>();

        let active_topic = selected_position
            .map(|index| {
                self.active_topic(
                    session,
                    &resolved.rows[index],
                    language,
                    topics[index].title.clone(),
                )
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

    fn active_topic(
        &mut self,
        session: &EncyclopediaSession,
        resolved: &ResolvedTopic<'_>,
        language: &str,
        title: Arc<str>,
    ) -> Result<ActiveTopicView, EncyclopediaError> {
        Ok(ActiveTopicView {
            topic_id: resolved.topic_id.0.clone(),
            title,
            body: self.active_body(
                session,
                resolved.topic_id,
                language,
                &resolved.localized.body,
            ),
            image: selected_image(session, resolved)?,
            // The reviewed UI contract identifies authored body text and no
            // connected live-stat controls. E19 therefore exposes no rows.
            stats: Vec::new(),
        })
    }
}

pub fn build_encyclopedia_view(
    presenter: &mut EncyclopediaPresenter,
    session: &EncyclopediaSession,
    admission: Option<&AdmissionSnapshot>,
    language: &str,
    selection: &EncyclopediaSelection,
) -> Result<EncyclopediaView, EncyclopediaError> {
    presenter.build_encyclopedia_view(session, admission, language, selection)
}

fn selected_image(
    session: &EncyclopediaSession,
    topic: &ResolvedTopic<'_>,
) -> Result<Option<TopicImageView>, EncyclopediaError> {
    let Some(image_id) = topic.image_id else {
        return Ok(None);
    };
    let descriptor = session
        .effective_catalog()
        .images
        .get(image_id)
        .ok_or_else(|| {
            EncyclopediaError::for_session(
                "missing_effective_image",
                format!("$.images.{}", image_id.0),
                "selected image identity is absent from the validated effective catalog",
            )
        })?;
    let bytes = session
        .effective_bytes()
        .get(&descriptor.path)
        .cloned()
        .ok_or_else(|| {
            EncyclopediaError::for_session(
                "missing_runtime_file",
                &descriptor.path,
                "selected image bytes are absent from the validated session",
            )
        })?;
    let observed = session
        .observed_facts()
        .get(&descriptor.path)
        .ok_or_else(|| {
            EncyclopediaError::for_session(
                "missing_image_facts",
                &descriptor.path,
                "selected image has no retained byte inspection facts",
            )
        })?;
    let image = observed.image.as_ref().ok_or_else(|| {
        EncyclopediaError::for_session(
            "missing_image_facts",
            &descriptor.path,
            "selected retained bytes were not validated as an image",
        )
    })?;

    Ok(Some(TopicImageView {
        asset_id: image_id.0.clone(),
        digest: observed.sha256.clone(),
        format: image.format.clone(),
        width: image.width,
        height: image.height,
        bytes,
        render_profile: TopicImageRenderProfile::OriginalNearest,
    }))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::sync::Arc;

    use rand::{RngCore, SeedableRng};
    use rand_xoshiro::Xoshiro256PlusPlus;
    use rebellion_data::encyclopedia::{
        AdmissionFact, AdmissionSnapshot, AdmittedBinding, BindingKey, SystemSourceAncestry,
        ViewerFaction,
    };
    use rebellion_render::{
        inspect_encyclopedia_bytes, EncyclopediaDiagnosticScope, EncyclopediaSelection,
        TopicImageRenderProfile,
    };
    use serde_json::Value;

    use super::{build_encyclopedia_view, EncyclopediaPresenter};
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

    fn definition(family: &str, dat_id: u32, variant: &str) -> AdmittedBinding {
        AdmittedBinding {
            key: BindingKey {
                family: family.to_owned(),
                dat_id,
                variant: variant.to_owned(),
            },
            fact: AdmissionFact::DefinitionPresent,
        }
    }

    fn system(dat_id: u32, viewer: ViewerFaction) -> AdmittedBinding {
        AdmittedBinding {
            key: BindingKey {
                family: "system_locations".to_owned(),
                dat_id,
                variant: "default".to_owned(),
            },
            fact: AdmissionFact::InstantiatedSystem {
                selected_view: viewer,
                ancestry: SystemSourceAncestry::NoTypeF2,
            },
        }
    }

    fn snapshot(
        world_epoch: u64,
        viewer: ViewerFaction,
        admitted: Vec<AdmittedBinding>,
    ) -> AdmissionSnapshot {
        AdmissionSnapshot {
            world_epoch,
            viewer,
            admitted,
        }
    }

    fn move_english_record_to_french(catalog: &mut Value, topic_id: &str) {
        let localized = catalog["topics"][topic_id]["localized"]
            .as_object_mut()
            .unwrap();
        let english = localized.remove("1033").unwrap();
        localized.insert("1036".to_owned(), english);
    }

    fn topic_ids(view: &rebellion_render::EncyclopediaView) -> Vec<&str> {
        view.topics
            .iter()
            .map(|topic| topic.topic_id.as_str())
            .collect()
    }

    fn topic_titles(view: &rebellion_render::EncyclopediaView) -> Vec<&str> {
        view.topics
            .iter()
            .map(|topic| topic.title.as_ref())
            .collect()
    }

    #[test]
    fn same_numeric_id_in_different_families_keeps_original_topic_identity() {
        let session = session();
        let admission = snapshot(
            4,
            ViewerFaction::Alliance,
            vec![
                definition("capital_ship_classes", 7, "default"),
                system(7, ViewerFaction::Alliance),
            ],
        );
        let selection = EncyclopediaSelection {
            category_id: None,
            topic_id: Some("original:60002".to_owned()),
        };

        let view = build_encyclopedia_view(
            &mut EncyclopediaPresenter::default(),
            &session,
            Some(&admission),
            "1033",
            &selection,
        )
        .unwrap();

        assert_eq!(topic_ids(&view), ["original:60001", "original:60002"]);
        assert_eq!(
            view.active_topic
                .as_ref()
                .map(|topic| topic.topic_id.as_str()),
            Some("original:60002")
        );
        assert_eq!(view.navigation.world_epoch, 4);
    }

    #[test]
    fn requested_and_default_whole_records_drive_labels_and_sorted_titles() {
        let session = session();
        let admission = snapshot(
            1,
            ViewerFaction::Alliance,
            vec![
                system(7, ViewerFaction::Alliance),
                definition("capital_ship_classes", 7, "default"),
            ],
        );
        let mut presenter = EncyclopediaPresenter::default();

        let view = build_encyclopedia_view(
            &mut presenter,
            &session,
            Some(&admission),
            "1036",
            &EncyclopediaSelection::default(),
        )
        .unwrap();

        assert_eq!(topic_titles(&view), ["Blue vessel", "Système ambre"]);
        assert_eq!(
            view.categories[0].label.as_deref(),
            Some("Systèmes synthétiques")
        );
        assert_eq!(view.categories[1].label.as_deref(), Some("Synthetic craft"));
    }

    #[test]
    fn faction_art_uses_actual_selected_bytes_and_source_proven_topics_have_no_stats() {
        let session = session();
        let alliance = snapshot(
            1,
            ViewerFaction::Alliance,
            vec![definition("missions", 21, "viewer_faction")],
        );
        let empire = snapshot(
            2,
            ViewerFaction::Empire,
            vec![definition("missions", 21, "viewer_faction")],
        );
        let selection = EncyclopediaSelection {
            category_id: None,
            topic_id: Some("original:60004".to_owned()),
        };
        let mut presenter = EncyclopediaPresenter::default();

        let alliance_view = build_encyclopedia_view(
            &mut presenter,
            &session,
            Some(&alliance),
            "1033",
            &selection,
        )
        .unwrap();
        let alliance_topic = alliance_view.active_topic.unwrap();
        let alliance_image = alliance_topic.image.unwrap();
        assert_eq!(alliance_image.asset_id, "edata:2");
        assert_eq!(
            alliance_image.digest,
            session.observed_facts()["assets/EDATA.002"].sha256
        );
        assert_eq!(
            alliance_image.render_profile,
            TopicImageRenderProfile::OriginalNearest
        );
        assert!(Arc::ptr_eq(
            &alliance_image.bytes,
            &session.effective_bytes()["assets/EDATA.002"]
        ));
        assert!(alliance_topic.stats.is_empty());

        let empire_view =
            build_encyclopedia_view(&mut presenter, &session, Some(&empire), "1033", &selection)
                .unwrap();
        assert_eq!(
            empire_view.active_topic.unwrap().image.unwrap().asset_id,
            "edata:3"
        );
    }

    #[test]
    fn no_art_and_aggregate_only_topics_preserve_their_catalog_membership() {
        let session = session();
        let admission = snapshot(
            1,
            ViewerFaction::Alliance,
            vec![
                definition("troop_classes", 5, "default"),
                definition("special_forces", 8, "default"),
                definition("fixture_fleet", 2, "default"),
            ],
        );
        let mut presenter = EncyclopediaPresenter::default();

        let index = build_encyclopedia_view(
            &mut presenter,
            &session,
            Some(&admission),
            "1033",
            &EncyclopediaSelection {
                category_id: None,
                topic_id: Some("original:60005".to_owned()),
            },
        )
        .unwrap();
        assert!(topic_ids(&index).contains(&"original:60007"));
        assert!(index.active_topic.unwrap().image.is_none());

        let craft_category = build_encyclopedia_view(
            &mut presenter,
            &session,
            Some(&admission),
            "1033",
            &EncyclopediaSelection {
                category_id: Some("command:0x71".to_owned()),
                topic_id: None,
            },
        )
        .unwrap();
        assert!(craft_category.topics.is_empty());
    }

    #[test]
    fn world_epoch_changes_refresh_membership_without_invalidating_cached_text() {
        let session = session();
        let first = snapshot(
            10,
            ViewerFaction::Alliance,
            vec![
                system(7, ViewerFaction::Alliance),
                definition("capital_ship_classes", 7, "default"),
            ],
        );
        let replacement = snapshot(
            11,
            ViewerFaction::Alliance,
            vec![definition("capital_ship_classes", 7, "default")],
        );
        let selection = EncyclopediaSelection {
            category_id: None,
            topic_id: Some("original:60001".to_owned()),
        };
        let original_selection = selection.clone();
        let mut presenter = EncyclopediaPresenter::default();

        let before =
            build_encyclopedia_view(&mut presenter, &session, Some(&first), "1033", &selection)
                .unwrap();
        let cached_title = before
            .topics
            .iter()
            .find(|topic| topic.topic_id == "original:60002")
            .unwrap()
            .title
            .clone();
        let after = build_encyclopedia_view(
            &mut presenter,
            &session,
            Some(&replacement),
            "1033",
            &selection,
        )
        .unwrap();

        assert_eq!(topic_ids(&after), ["original:60002"]);
        assert_eq!(after.navigation.world_epoch, 11);
        assert_eq!(after.navigation.selected_topic_id, None);
        assert_eq!(after.active_topic, None);
        assert!(Arc::ptr_eq(&cached_title, &after.topics[0].title));
        assert_eq!(selection, original_selection);
    }

    #[test]
    fn immutable_text_cache_reuses_arcs_within_a_generation_and_discards_prior_generations() {
        let session = session();
        let admission = snapshot(
            1,
            ViewerFaction::Alliance,
            vec![system(7, ViewerFaction::Alliance)],
        );
        let selection = EncyclopediaSelection {
            category_id: None,
            topic_id: Some("original:60001".to_owned()),
        };
        let mut presenter = EncyclopediaPresenter::default();

        let first = build_encyclopedia_view(
            &mut presenter,
            &session,
            Some(&admission),
            "1033",
            &selection,
        )
        .unwrap();
        let repeated = build_encyclopedia_view(
            &mut presenter,
            &session,
            Some(&admission),
            "1033",
            &selection,
        )
        .unwrap();
        let first_topic = first.active_topic.unwrap();
        let repeated_topic = repeated.active_topic.unwrap();
        assert!(Arc::ptr_eq(&first_topic.title, &repeated_topic.title));
        assert!(Arc::ptr_eq(&first_topic.body, &repeated_topic.body));

        let replacement_generation = session.generation() + 1;
        presenter.begin_generation(replacement_generation);
        assert_eq!(presenter.cached_generation, Some(replacement_generation));
        assert!(presenter.titles.is_empty());
        assert!(presenter.active_body.is_none());
    }

    #[test]
    fn category_and_index_availability_control_membership_without_mutating_selection() {
        let admission = snapshot(
            1,
            ViewerFaction::Alliance,
            vec![
                system(7, ViewerFaction::Alliance),
                definition("capital_ship_classes", 7, "default"),
            ],
        );
        let enabled_session = session();
        let selected_category = EncyclopediaSelection {
            category_id: Some("command:0x71".to_owned()),
            topic_id: None,
        };
        let enabled = build_encyclopedia_view(
            &mut EncyclopediaPresenter::default(),
            &enabled_session,
            Some(&admission),
            "1033",
            &selected_category,
        )
        .unwrap();
        assert_eq!(topic_ids(&enabled), ["original:60002"]);
        assert_eq!(
            enabled.navigation.selected_category_id.as_deref(),
            Some("command:0x71")
        );

        let disabled_category_session = customized_session(|catalog| {
            catalog["categories"][1]["labels"] = serde_json::json!({"1036": "Vaisseaux"});
        });
        let disabled_category = build_encyclopedia_view(
            &mut EncyclopediaPresenter::default(),
            &disabled_category_session,
            Some(&admission),
            "1041",
            &selected_category,
        )
        .unwrap();
        assert!(disabled_category.topics.is_empty());
        assert_eq!(
            disabled_category.navigation.selected_category_id.as_deref(),
            Some("command:0x71")
        );

        let unknown_category = EncyclopediaSelection {
            category_id: Some("command:unknown".to_owned()),
            topic_id: None,
        };
        let enabled_index = build_encyclopedia_view(
            &mut EncyclopediaPresenter::default(),
            &enabled_session,
            Some(&admission),
            "1033",
            &unknown_category,
        )
        .unwrap();
        assert_eq!(
            topic_ids(&enabled_index),
            ["original:60001", "original:60002"]
        );
        assert_eq!(enabled_index.navigation.selected_category_id, None);
        assert!(enabled_index.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "unknown_category_selection"
                && diagnostic.scope
                    == EncyclopediaDiagnosticScope::Category {
                        category_id: "command:unknown".to_owned(),
                    }
        }));

        let disabled_index_session = customized_session(|catalog| {
            catalog["index"]["labels"] = serde_json::json!({"1036": "Index"});
        });
        for selection in [&EncyclopediaSelection::default(), &unknown_category] {
            let disabled_index = build_encyclopedia_view(
                &mut EncyclopediaPresenter::default(),
                &disabled_index_session,
                Some(&admission),
                "1041",
                selection,
            )
            .unwrap();
            assert!(!disabled_index.index_enabled);
            assert!(disabled_index.topics.is_empty());
            assert_eq!(disabled_index.navigation.selected_category_id, None);
        }
    }

    #[test]
    fn missing_snapshot_is_whole_view_unavailability_but_missing_language_is_per_topic() {
        let session = customized_session(|catalog| {
            move_english_record_to_french(catalog, "original:60002");
        });
        let admission = snapshot(
            1,
            ViewerFaction::Alliance,
            vec![
                system(7, ViewerFaction::Alliance),
                definition("capital_ship_classes", 7, "default"),
            ],
        );
        let mut presenter = EncyclopediaPresenter::default();

        let unavailable = build_encyclopedia_view(
            &mut presenter,
            &session,
            None,
            "1041",
            &EncyclopediaSelection::default(),
        )
        .unwrap_err();
        assert_eq!(unavailable.code(), "missing_admission_facts");

        let available = build_encyclopedia_view(
            &mut presenter,
            &session,
            Some(&admission),
            "1041",
            &EncyclopediaSelection::default(),
        )
        .unwrap();
        assert_eq!(topic_ids(&available), ["original:60001"]);
        assert!(available.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "missing_localized_record"
                && diagnostic.scope
                    == EncyclopediaDiagnosticScope::Topic {
                        topic_id: "original:60002".to_owned(),
                    }
        }));
    }

    #[test]
    fn all_unlocalized_admitted_topics_leave_an_available_empty_list_with_diagnostics() {
        let session = customized_session(|catalog| {
            for id in ["original:60001", "original:60002"] {
                move_english_record_to_french(catalog, id);
            }
        });
        let admission = snapshot(
            1,
            ViewerFaction::Alliance,
            vec![
                system(7, ViewerFaction::Alliance),
                definition("capital_ship_classes", 7, "default"),
            ],
        );

        let view = build_encyclopedia_view(
            &mut EncyclopediaPresenter::default(),
            &session,
            Some(&admission),
            "1041",
            &EncyclopediaSelection::default(),
        )
        .unwrap();

        assert!(view.topics.is_empty());
        assert_eq!(
            view.diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.code == "missing_localized_record")
                .count(),
            2
        );
    }

    #[test]
    fn effective_title_order_changes_without_changing_selected_original_identity() {
        let session = customized_session(|catalog| {
            catalog["topics"]["original:60001"]["localized"]["1033"]["title"] =
                Value::from("Zulu system");
            catalog["topics"]["original:60002"]["localized"]["1033"]["title"] =
                Value::from("Aardvark vessel");
        });
        let admission = snapshot(
            1,
            ViewerFaction::Alliance,
            vec![
                system(7, ViewerFaction::Alliance),
                definition("capital_ship_classes", 7, "default"),
            ],
        );
        let selection = EncyclopediaSelection {
            category_id: None,
            topic_id: Some("original:60001".to_owned()),
        };

        let view = build_encyclopedia_view(
            &mut EncyclopediaPresenter::default(),
            &session,
            Some(&admission),
            "1033",
            &selection,
        )
        .unwrap();

        assert_eq!(topic_ids(&view), ["original:60002", "original:60001"]);
        assert_eq!(
            view.active_topic.unwrap().topic_id,
            "original:60001",
            "selection follows stable topic identity rather than row position"
        );
        assert_eq!(
            view.navigation.previous_topic_id.as_deref(),
            Some("original:60002")
        );
        assert_eq!(view.navigation.next_topic_id, None);
    }

    #[test]
    fn equal_effective_titles_keep_registry_order() {
        let session = customized_session(|catalog| {
            for id in ["original:60001", "original:60002"] {
                catalog["topics"][id]["localized"]["1033"]["title"] = Value::from("Same");
            }
        });
        let admission = snapshot(
            1,
            ViewerFaction::Alliance,
            vec![
                definition("capital_ship_classes", 7, "default"),
                system(7, ViewerFaction::Alliance),
            ],
        );

        let view = build_encyclopedia_view(
            &mut EncyclopediaPresenter::default(),
            &session,
            Some(&admission),
            "1033",
            &EncyclopediaSelection::default(),
        )
        .unwrap();

        assert_eq!(topic_ids(&view), ["original:60001", "original:60002"]);
    }

    #[test]
    fn missing_category_label_disables_it_without_exposing_the_internal_key() {
        let session = customized_session(|catalog| {
            catalog["categories"][1]["labels"] = serde_json::json!({"1036": "Vaisseaux"});
        });
        let admission = snapshot(
            1,
            ViewerFaction::Alliance,
            vec![definition("capital_ship_classes", 7, "default")],
        );

        let view = build_encyclopedia_view(
            &mut EncyclopediaPresenter::default(),
            &session,
            Some(&admission),
            "1041",
            &EncyclopediaSelection::default(),
        )
        .unwrap();

        assert_eq!(view.categories[1].label, None);
        assert!(!view.categories[1].enabled);
        assert!(view.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "missing_localized_label"
                && diagnostic.scope
                    == EncyclopediaDiagnosticScope::Category {
                        category_id: "command:0x71".to_owned(),
                    }
        }));
    }

    #[test]
    fn building_a_view_leaves_admission_bytes_selection_and_rng_unchanged() {
        let session = session();
        let admission = snapshot(
            77,
            ViewerFaction::Alliance,
            vec![system(7, ViewerFaction::Alliance)],
        );
        let admission_before = admission.clone();
        let selection = EncyclopediaSelection::default();
        let selection_before = selection.clone();
        let catalog_bytes = session.effective_bytes()["catalog.json"].clone();
        let mut actual_rng = Xoshiro256PlusPlus::seed_from_u64(0x1919);
        let mut expected_rng = actual_rng.clone();

        let _ = build_encyclopedia_view(
            &mut EncyclopediaPresenter::default(),
            &session,
            Some(&admission),
            "1033",
            &selection,
        )
        .unwrap();

        assert_eq!(admission, admission_before);
        assert_eq!(selection, selection_before);
        assert!(Arc::ptr_eq(
            &catalog_bytes,
            &session.effective_bytes()["catalog.json"]
        ));
        assert_eq!(actual_rng.next_u64(), expected_rng.next_u64());
    }
}
