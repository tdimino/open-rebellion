//! Native app lifecycle adapter for the immutable encyclopedia mod engine.
//!
//! This module does not resolve mod order. Its callers supply one already-resolved
//! E49 order to both the world and content consumers through
//! [`apply_resolved_mod_update`]. Filesystem reads are handed to E24 as owned raw
//! targets; no overlay bytes are retained here.

use std::collections::BTreeSet;

use rebellion_data::encyclopedia::EncyclopediaError;
use rebellion_data::mods::{
    read_encyclopedia_target_with_admission, ModContentTarget, ModManifest,
    ENCYCLOPEDIA_MOD_FILENAME, ENCYCLOPEDIA_READ_ERROR_MESSAGE_BYTES_LIMIT,
};

use crate::encyclopedia_mods::{
    EncyclopediaModDiagnostic, EncyclopediaModEngine, ResolvedEncyclopediaMod,
};
use crate::encyclopedia_session::EncyclopediaAvailability;

/// Existing app event that requested one synchronized mod-content update.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModLifecycleTrigger {
    Startup,
    NewCampaign,
    ManualReload,
    Toggle,
    SavedWorldLoad,
    /// Entry used by E25 and explicit fixed-list content refreshes.
    ContentOnly,
}

impl ModLifecycleTrigger {
    const fn applies_world(self) -> bool {
        matches!(self, Self::Startup | Self::NewCampaign | Self::ManualReload)
    }
}

/// Results from the two consumers of one already-resolved mod order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedModUpdate<World, Content> {
    /// Absent for events whose world state is already authoritative.
    pub world: Option<World>,
    pub content: Content,
}

/// Passes the exact same resolved slice to the appropriate world consumer and
/// the content consumer. Content is always refreshed; world patches are never
/// replayed for toggles, saved-world loads, or content-only notifications.
pub fn apply_resolved_mod_update<World, Content>(
    trigger: ModLifecycleTrigger,
    ordered: &[&ModManifest],
    apply_world: impl FnOnce(&[&ModManifest]) -> World,
    refresh_content: impl FnOnce(&[&ModManifest]) -> Content,
) -> ResolvedModUpdate<World, Content> {
    let world = trigger.applies_world().then(|| apply_world(ordered));
    let content = refresh_content(ordered);
    ResolvedModUpdate { world, content }
}

/// App-facing result of one E24 content transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaContentRefresh {
    pub diagnostics: Vec<EncyclopediaModDiagnostic>,
    pub changed_image_ids: BTreeSet<String>,
    pub removed_image_ids: BTreeSet<String>,
    pub published_changed: bool,
    pub generation: Option<u64>,
}

impl EncyclopediaContentRefresh {
    fn unavailable(message: String) -> Self {
        Self {
            diagnostics: vec![EncyclopediaModDiagnostic {
                mod_name: String::new(),
                code: "encyclopedia_unavailable",
                path: "$".to_owned(),
                message,
            }],
            changed_image_ids: BTreeSet::new(),
            removed_image_ids: BTreeSet::new(),
            published_changed: false,
            generation: None,
        }
    }
}

enum LifecycleState {
    Unavailable(String),
    Ready(Box<EncyclopediaModEngine>),
}

/// Native state kept beside the app state, never inside `GameWorld` or saves.
pub struct EncyclopediaLifecycle {
    state: LifecycleState,
    diagnostics: Vec<EncyclopediaModDiagnostic>,
    #[cfg(test)]
    admitted_raw_reads: u64,
    #[cfg(test)]
    attempted_target_reads: u64,
}

impl EncyclopediaLifecycle {
    #[must_use]
    pub fn from_availability(availability: EncyclopediaAvailability) -> Self {
        let state = match availability {
            EncyclopediaAvailability::Unavailable(message) => LifecycleState::Unavailable(message),
            EncyclopediaAvailability::Ready(session) => {
                match EncyclopediaModEngine::new(&session) {
                    Ok(engine) => LifecycleState::Ready(Box::new(engine)),
                    Err(error) => LifecycleState::Unavailable(error.to_string()),
                }
            }
        };
        Self {
            state,
            diagnostics: Vec::new(),
            #[cfg(test)]
            admitted_raw_reads: 0,
            #[cfg(test)]
            attempted_target_reads: 0,
        }
    }

    /// Refreshes only encyclopedia content using the caller's resolved order.
    /// This is the E25-safe entry: it has no world or `PanelAction` access.
    pub fn refresh_resolved(&mut self, ordered: &[&ModManifest]) -> EncyclopediaContentRefresh {
        #[cfg(test)]
        let admitted_raw_reads = &mut self.admitted_raw_reads;
        #[cfg(test)]
        let attempted_target_reads = &mut self.attempted_target_reads;
        let (state, diagnostics) = (&mut self.state, &mut self.diagnostics);
        let LifecycleState::Ready(engine) = state else {
            let LifecycleState::Unavailable(message) = state else {
                unreachable!("lifecycle state has exactly two variants")
            };
            let report = EncyclopediaContentRefresh::unavailable(message.clone());
            diagnostics.clone_from(&report.diagnostics);
            return report;
        };

        let identity_bytes = match incoming_identity_bytes(ordered) {
            Ok(bytes) => bytes,
            Err(error) => return publish_error_report(diagnostics, engine, error),
        };
        let diagnostic_bytes = match incoming_diagnostic_bytes(ordered) {
            Ok(bytes) => bytes,
            Err(error) => return publish_error_report(diagnostics, engine, error),
        };
        if let Err(error) =
            engine.admit_incoming_raw_targets(ordered.len(), identity_bytes, 0, diagnostic_bytes)
        {
            return publish_error_report(diagnostics, engine, error);
        }
        let mut raw_capacity = 0_u64;
        let mut inputs = Vec::with_capacity(ordered.len());
        for manifest in ordered {
            #[cfg(test)]
            {
                *attempted_target_reads = attempted_target_reads
                    .checked_add(1)
                    .expect("a finite test cannot overflow the target-read attempt counter");
            }
            let content = match read_encyclopedia_target_with_admission(&manifest.path, |length| {
                let next = raw_capacity.checked_add(length).ok_or_else(|| {
                    EncyclopediaError::for_session(
                        "resource_limit:retained_bytes",
                        "$",
                        "incoming raw-target byte total overflowed",
                    )
                })?;
                engine.admit_incoming_raw_targets(
                    ordered.len(),
                    identity_bytes,
                    next,
                    diagnostic_bytes,
                )?;
                #[cfg(test)]
                {
                    *admitted_raw_reads = admitted_raw_reads.checked_add(1).expect(
                        "the test-only admitted-read counter cannot overflow in a finite test",
                    );
                }
                Ok::<(), EncyclopediaError>(())
            }) {
                Ok(content) => content,
                Err(error) => return publish_error_report(diagnostics, engine, error),
            };
            if let ModContentTarget::Bytes(bytes) = &content {
                let capacity = match u64::try_from(bytes.capacity()) {
                    Ok(capacity) => capacity,
                    Err(_) => {
                        return publish_error_report(
                            diagnostics,
                            engine,
                            EncyclopediaError::for_session(
                                "resource_limit:retained_bytes",
                                "$",
                                "incoming raw-target capacity does not fit u64",
                            ),
                        );
                    }
                };
                raw_capacity = match raw_capacity.checked_add(capacity) {
                    Some(total) => total,
                    None => {
                        return publish_error_report(
                            diagnostics,
                            engine,
                            EncyclopediaError::for_session(
                                "resource_limit:retained_bytes",
                                "$",
                                "incoming raw-target capacity total overflowed",
                            ),
                        );
                    }
                };
                if let Err(error) = engine.admit_incoming_raw_targets(
                    ordered.len(),
                    identity_bytes,
                    raw_capacity,
                    diagnostic_bytes,
                ) {
                    return publish_error_report(diagnostics, engine, error);
                }
            }
            inputs.push(ResolvedEncyclopediaMod {
                name: manifest.name.clone(),
                root: manifest.path.clone(),
                content,
            });
        }

        let report = match engine.refresh(inputs) {
            Ok(report) => EncyclopediaContentRefresh {
                diagnostics: report
                    .diagnostics
                    .into_iter()
                    .map(|diagnostic| contextualize_diagnostic(ordered, diagnostic))
                    .collect(),
                changed_image_ids: report.changed_image_ids,
                removed_image_ids: report.removed_image_ids,
                published_changed: report.published_changed,
                generation: Some(engine.snapshot().generation()),
            },
            Err(error) => error_report(engine, error),
        };
        diagnostics.clone_from(&report.diagnostics);
        report
    }

    /// Explicit E25 entry for a fixed already-resolved list. It cannot invoke
    /// `PanelAction::ReloadMods` or mutate a world because neither is accepted.
    #[allow(
        dead_code,
        reason = "E50 exposes this narrow entry for the subsequent E25 watcher integration"
    )]
    pub fn refresh_content_only(&mut self, ordered: &[&ModManifest]) -> EncyclopediaContentRefresh {
        apply_resolved_mod_update(
            ModLifecycleTrigger::ContentOnly,
            ordered,
            |_| unreachable!("content-only refresh cannot apply world patches"),
            |ordered| self.refresh_resolved(ordered),
        )
        .content
    }

    #[must_use]
    pub fn diagnostic_for_mod(&self, mod_name: &str) -> Option<String> {
        let matching: Vec<String> = self
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.mod_name.is_empty() || diagnostic.mod_name == mod_name)
            .map(|diagnostic| {
                format!(
                    "{} at {}: {}",
                    diagnostic.code, diagnostic.path, diagnostic.message
                )
            })
            .collect();
        (!matching.is_empty()).then(|| matching.join("; "))
    }

    #[cfg(test)]
    pub fn retained_bytes_for_test(&self) -> u64 {
        match &self.state {
            LifecycleState::Ready(engine) => engine.retained_bytes(),
            LifecycleState::Unavailable(_) => 0,
        }
    }

    #[cfg(test)]
    pub fn set_retained_limit_for_test(&mut self, limit: u64) {
        if let LifecycleState::Ready(engine) = &mut self.state {
            engine.set_retained_limit_for_test(limit);
        }
    }

    #[cfg(test)]
    pub fn admitted_raw_reads_for_test(&self) -> u64 {
        self.admitted_raw_reads
    }

    #[cfg(test)]
    pub fn attempted_target_reads_for_test(&self) -> u64 {
        self.attempted_target_reads
    }
}

fn incoming_identity_bytes(ordered: &[&ModManifest]) -> Result<u64, EncyclopediaError> {
    ordered.iter().try_fold(0_u64, |total, manifest| {
        let identity = manifest
            .name
            .len()
            .checked_add(manifest.path.as_os_str().as_encoded_bytes().len())
            .ok_or_else(|| {
                EncyclopediaError::for_session(
                    "resource_limit:retained_bytes",
                    &manifest.name,
                    "incoming mod identity accounting overflowed",
                )
            })?;
        total
            .checked_add(u64::try_from(identity).map_err(|_| {
                EncyclopediaError::for_session(
                    "resource_limit:retained_bytes",
                    &manifest.name,
                    "incoming mod identity length does not fit u64",
                )
            })?)
            .ok_or_else(|| {
                EncyclopediaError::for_session(
                    "resource_limit:retained_bytes",
                    "$",
                    "incoming mod identity total overflowed",
                )
            })
    })
}

fn incoming_diagnostic_bytes(ordered: &[&ModManifest]) -> Result<u64, EncyclopediaError> {
    ordered.iter().try_fold(0_u64, |total, manifest| {
        let path_bytes = manifest
            .path
            .as_os_str()
            .as_encoded_bytes()
            .len()
            .checked_add(1)
            .and_then(|length| length.checked_add(ENCYCLOPEDIA_MOD_FILENAME.len()))
            .ok_or_else(|| {
                EncyclopediaError::for_session(
                    "resource_limit:retained_bytes",
                    &manifest.name,
                    "incoming read diagnostic path accounting overflowed",
                )
            })?;
        let envelope = path_bytes
            .checked_add(ENCYCLOPEDIA_READ_ERROR_MESSAGE_BYTES_LIMIT)
            .ok_or_else(|| {
                EncyclopediaError::for_session(
                    "resource_limit:retained_bytes",
                    &manifest.name,
                    "incoming read diagnostic message accounting overflowed",
                )
            })?;
        total
            .checked_add(u64::try_from(envelope).map_err(|_| {
                EncyclopediaError::for_session(
                    "resource_limit:retained_bytes",
                    &manifest.name,
                    "incoming read diagnostic envelope does not fit u64",
                )
            })?)
            .ok_or_else(|| {
                EncyclopediaError::for_session(
                    "resource_limit:retained_bytes",
                    "$",
                    "incoming read diagnostic envelope total overflowed",
                )
            })
    })
}

fn error_report(
    engine: &EncyclopediaModEngine,
    error: EncyclopediaError,
) -> EncyclopediaContentRefresh {
    EncyclopediaContentRefresh {
        diagnostics: vec![EncyclopediaModDiagnostic {
            mod_name: String::new(),
            code: error.code(),
            path: error.path().to_owned(),
            message: error.to_string(),
        }],
        changed_image_ids: BTreeSet::new(),
        removed_image_ids: BTreeSet::new(),
        published_changed: false,
        generation: Some(engine.snapshot().generation()),
    }
}

fn publish_error_report(
    diagnostics: &mut Vec<EncyclopediaModDiagnostic>,
    engine: &EncyclopediaModEngine,
    error: EncyclopediaError,
) -> EncyclopediaContentRefresh {
    let report = error_report(engine, error);
    diagnostics.clone_from(&report.diagnostics);
    report
}

fn contextualize_diagnostic(
    ordered: &[&ModManifest],
    mut diagnostic: EncyclopediaModDiagnostic,
) -> EncyclopediaModDiagnostic {
    if let Some(manifest) = ordered
        .iter()
        .find(|manifest| manifest.name == diagnostic.mod_name)
    {
        diagnostic.path = format!(
            "{}#{}",
            manifest.path.join(ENCYCLOPEDIA_MOD_FILENAME).display(),
            diagnostic.path
        );
    }
    diagnostic
}
