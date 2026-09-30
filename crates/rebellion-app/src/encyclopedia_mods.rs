//! Native encyclopedia mod snapshots and atomic effective-session publication.
//!
//! This module deliberately owns no watcher or application lifecycle hook. The
//! caller supplies the already-resolved enabled order and the raw content target
//! separated by `rebellion_data::mods`. Files are read only while constructing a
//! candidate; the published snapshot serves the exact retained buffers thereafter.

use std::collections::{BTreeMap, BTreeSet};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[cfg(all(test, any(target_os = "linux", target_os = "android")))]
use std::fs;
#[cfg(any(target_os = "linux", target_os = "android"))]
use std::fs::{File, OpenOptions};
#[cfg(any(target_os = "linux", target_os = "android"))]
use std::io::Read;
#[cfg(any(target_os = "linux", target_os = "android"))]
use std::os::fd::AsRawFd;
#[cfg(any(target_os = "linux", target_os = "android"))]
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};

#[cfg(not(target_arch = "wasm32"))]
use rebellion_data::encyclopedia::ImageFacts;
use rebellion_data::encyclopedia::{
    apply_encyclopedia_overlay, parse_encyclopedia_overlay, validate_effective_catalog, AssetFacts,
    BaseImageIdField, EncyclopediaCatalog, EncyclopediaError, FactionImagePatch, ImagePatch,
    LocalizedContent, NullableBaseImageId, OverlayImageInputs, PatchField, TopicPatch,
};
use rebellion_data::mods::ModContentTarget;
#[cfg(not(target_arch = "wasm32"))]
use rebellion_render::encyclopedia_assets::MAX_ENCYCLOPEDIA_IMAGE_BYTES;
use rebellion_render::inspect_encyclopedia_bytes;

use crate::encyclopedia_session::{
    EncyclopediaBytes, EncyclopediaSession, MAX_ENCYCLOPEDIA_RETAINED_BYTES,
};

const MOD_IMAGE_PREFIX: &str = "mod:v1:";
const MAX_EFFECTIVE_IMAGE_BYTES: u64 = 128 * 1024 * 1024;
// E22's Serde raw tree and typed overlay coexist during parsing. Admit a
// conservative 64 bytes of transient tree/typed workspace for every retained
// input byte before entering that parser. This is charged to the one global
// cap, is checked for overflow, and is released before image inspection; it is
// not a per-mod quota or a total-process-memory claim.
const OVERLAY_PARSE_WORKING_BYTES_PER_INPUT_BYTE: u64 = 64;
// The accepted renderer supports PNG RGBA16, so the frozen 16,000,000-pixel
// limit permits a 128,000,000-byte decoded result. Conservatively admit two
// such allocations for the codec's bounded workspace plus returned image.
// Exact compressed Vec-to-Arc overlap is charged separately below.
const MAX_DECODED_IMAGE_BYTES: u64 = 16_000_000 * 8;
const IMAGE_DECODE_WORKING_BYTES: u64 = MAX_DECODED_IMAGE_BYTES * 2;

/// Deterministic test observation points after an OS handle, rather than a
/// pathname, owns the confinement decision.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfinementTestPoint {
    RootPinned,
    ParentPinned,
    ImageInspectionStarting,
    OverlayApplicationStarting,
}

/// One enabled mod in the dependency order already resolved by the shared mod loader.
#[derive(Debug)]
pub struct ResolvedEncyclopediaMod {
    pub name: String,
    pub root: PathBuf,
    pub content: ModContentTarget,
}

/// Stable, contextual failure for one mod contribution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaModDiagnostic {
    pub mod_name: String,
    pub code: &'static str,
    pub path: String,
    pub message: String,
}

/// Exact owner of bytes published under one canonical image identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncyclopediaImageOwner {
    Base { path: String },
    Mod { mod_name: String, path: String },
}

#[derive(Debug)]
struct PublishedGeneration {
    catalog: EncyclopediaCatalog,
    image_bytes: BTreeMap<String, Arc<[u8]>>,
    image_owners: BTreeMap<String, EncyclopediaImageOwner>,
    generation: u64,
    metadata_bytes: u64,
}

#[derive(Debug, Default)]
struct ExternalRetentionState {
    publications: BTreeMap<usize, (u64, usize)>,
    buffers: BTreeMap<(usize, usize), (u64, usize)>,
}

#[derive(Debug, Default)]
struct ExternalRetentionTracker {
    state: Mutex<ExternalRetentionState>,
}

/// Immutable effective content made visible as one generation. A cloned
/// snapshot is an explicit old-live lease and remains charged to the engine's
/// single retained-byte cap until the final clone drops.
#[derive(Debug)]
pub struct EffectiveEncyclopediaSnapshot {
    inner: Arc<PublishedGeneration>,
    tracker: Arc<ExternalRetentionTracker>,
    externally_tracked: bool,
}

impl EffectiveEncyclopediaSnapshot {
    #[must_use]
    pub fn catalog(&self) -> &EncyclopediaCatalog {
        &self.inner.catalog
    }

    /// Borrows exact retained bytes without transferring untracked ownership.
    #[must_use]
    pub fn image(&self, image_id: &str) -> Option<&[u8]> {
        self.inner.image_bytes.get(image_id).map(AsRef::as_ref)
    }

    pub fn image_ids(&self) -> impl Iterator<Item = &String> {
        self.inner.image_bytes.keys()
    }

    #[must_use]
    pub fn contains_image(&self, image_id: &str) -> bool {
        self.inner.image_bytes.contains_key(image_id)
    }

    /// Explicit long-lived image ownership. Unlike exposing `Arc`, this lease
    /// remains visible to retained-byte admission after its publication is old.
    #[must_use]
    pub fn lease_image(&self, image_id: &str) -> Option<EncyclopediaImageLease> {
        let bytes = Arc::clone(self.inner.image_bytes.get(image_id)?);
        self.tracker.register_buffer(&bytes);
        Some(EncyclopediaImageLease {
            bytes,
            tracker: Arc::clone(&self.tracker),
        })
    }

    #[must_use]
    pub fn image_owners(&self) -> &BTreeMap<String, EncyclopediaImageOwner> {
        &self.inner.image_owners
    }

    #[must_use]
    pub fn generation(&self) -> u64 {
        self.inner.generation
    }
}

impl Clone for EffectiveEncyclopediaSnapshot {
    fn clone(&self) -> Self {
        self.tracker.register_publication(&self.inner);
        Self {
            inner: Arc::clone(&self.inner),
            tracker: Arc::clone(&self.tracker),
            externally_tracked: true,
        }
    }
}

impl Drop for EffectiveEncyclopediaSnapshot {
    fn drop(&mut self) {
        if self.externally_tracked {
            self.tracker.unregister_publication(&self.inner);
        }
    }
}

/// Tracked ownership of one retained image buffer.
#[derive(Debug)]
pub struct EncyclopediaImageLease {
    bytes: Arc<[u8]>,
    tracker: Arc<ExternalRetentionTracker>,
}

impl Clone for EncyclopediaImageLease {
    fn clone(&self) -> Self {
        self.tracker.register_buffer(&self.bytes);
        Self {
            bytes: Arc::clone(&self.bytes),
            tracker: Arc::clone(&self.tracker),
        }
    }
}

impl Drop for EncyclopediaImageLease {
    fn drop(&mut self) {
        self.tracker.unregister_buffer(&self.bytes);
    }
}

impl Deref for EncyclopediaImageLease {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl ExternalRetentionTracker {
    fn register_publication(&self, publication: &Arc<PublishedGeneration>) {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        let identity = Arc::as_ptr(publication) as usize;
        let entry = state
            .publications
            .entry(identity)
            .or_insert((publication.metadata_bytes, 0));
        entry.1 += 1;
        for bytes in publication.image_bytes.values() {
            register_buffer_state(&mut state, bytes);
        }
    }

    fn unregister_publication(&self, publication: &Arc<PublishedGeneration>) {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        let identity = Arc::as_ptr(publication) as usize;
        unregister_count(&mut state.publications, &identity);
        for bytes in publication.image_bytes.values() {
            unregister_buffer_state(&mut state, bytes);
        }
    }

    fn register_buffer(&self, bytes: &Arc<[u8]>) {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        register_buffer_state(&mut state, bytes);
    }

    fn unregister_buffer(&self, bytes: &Arc<[u8]>) {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        unregister_buffer_state(&mut state, bytes);
    }
}

fn register_buffer_state(state: &mut ExternalRetentionState, bytes: &Arc<[u8]>) {
    let identity = (bytes.as_ptr() as usize, bytes.len());
    let entry = state
        .buffers
        .entry(identity)
        .or_insert((bytes.len() as u64, 0));
    entry.1 += 1;
}

fn unregister_buffer_state(state: &mut ExternalRetentionState, bytes: &Arc<[u8]>) {
    let identity = (bytes.as_ptr() as usize, bytes.len());
    unregister_count(&mut state.buffers, &identity);
}

fn unregister_count<K: Ord + Copy>(map: &mut BTreeMap<K, (u64, usize)>, key: &K) {
    let remove = if let Some((_, count)) = map.get_mut(key) {
        *count -= 1;
        *count == 0
    } else {
        false
    };
    if remove {
        map.remove(key);
    }
}

/// Result of one serialized refresh attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaRefreshReport {
    pub diagnostics: Vec<EncyclopediaModDiagnostic>,
    pub changed_image_ids: BTreeSet<String>,
    pub removed_image_ids: BTreeSet<String>,
    pub published_changed: bool,
    pub peak_retained_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RetainedImage {
    bytes: Arc<[u8]>,
    facts: AssetFacts,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AcceptedModSnapshot {
    overlay_bytes: Arc<Vec<u8>>,
    patches: Vec<TopicPatch>,
    images: BTreeMap<String, RetainedImage>,
}

/// Session-local transaction owner. It never mutates the validated base session.
pub struct EncyclopediaModEngine {
    base_catalog: EncyclopediaCatalog,
    base_bytes: EncyclopediaBytes,
    base_image_facts: BTreeMap<String, AssetFacts>,
    base_image_bytes: BTreeMap<String, Arc<[u8]>>,
    base_image_owners: BTreeMap<String, EncyclopediaImageOwner>,
    accepted: BTreeMap<String, Arc<AcceptedModSnapshot>>,
    published: EffectiveEncyclopediaSnapshot,
    external_retention: Arc<ExternalRetentionTracker>,
    retained_limit: u64,
    effective_image_limit: u64,
    retained_bytes: u64,
    #[cfg(test)]
    confinement_hook: Option<Arc<dyn Fn(ConfinementTestPoint) + Send + Sync>>,
}

impl EncyclopediaModEngine {
    /// Starts from a complete E45 session. Base bytes and metadata remain immutable.
    pub fn new(base: &EncyclopediaSession) -> Result<Self, EncyclopediaError> {
        Self::new_with_limit(base, MAX_ENCYCLOPEDIA_RETAINED_BYTES)
    }

    fn new_with_limit(
        base: &EncyclopediaSession,
        retained_limit: u64,
    ) -> Result<Self, EncyclopediaError> {
        let preflight = preflight_base_engine_bytes(base)?;
        ensure_retained_limit(preflight, retained_limit)?;
        let mut base_image_facts = BTreeMap::new();
        let mut base_image_bytes = BTreeMap::new();
        let mut base_image_owners = BTreeMap::new();
        for (image_id, descriptor) in &base.base_catalog().images {
            let facts = base.observed_facts().get(&descriptor.path).ok_or_else(|| {
                session_error(
                    "missing_image_provider",
                    &descriptor.path,
                    "validated base session omits observed image facts",
                )
            })?;
            let bytes = base.base_bytes().get(&descriptor.path).ok_or_else(|| {
                session_error(
                    "missing_runtime_file",
                    &descriptor.path,
                    "validated base session omits retained image bytes",
                )
            })?;
            base_image_facts.insert(image_id.0.clone(), facts.clone());
            base_image_bytes.insert(image_id.0.clone(), Arc::clone(bytes));
            base_image_owners.insert(
                image_id.0.clone(),
                EncyclopediaImageOwner::Base {
                    path: descriptor.path.clone(),
                },
            );
        }
        validate_effective_catalog(base.base_catalog(), &base_image_facts)?;

        let external_retention = Arc::new(ExternalRetentionTracker::default());
        let published = EffectiveEncyclopediaSnapshot {
            inner: make_published_generation(
                base.base_catalog().clone(),
                base_image_bytes.clone(),
                base_image_owners.clone(),
                base.generation(),
            )?,
            tracker: Arc::clone(&external_retention),
            externally_tracked: false,
        };
        let mut engine = Self {
            base_catalog: base.base_catalog().clone(),
            base_bytes: base.base_bytes().clone(),
            base_image_facts,
            base_image_bytes,
            base_image_owners,
            accepted: BTreeMap::new(),
            published,
            external_retention,
            retained_limit,
            effective_image_limit: MAX_EFFECTIVE_IMAGE_BYTES,
            retained_bytes: 0,
            #[cfg(test)]
            confinement_hook: None,
        };
        engine.retained_bytes = engine.account_state(&engine.accepted, &engine.published)?;
        if engine.retained_bytes != preflight {
            return Err(session_error(
                "retained_accounting_mismatch",
                "$",
                format!(
                    "base preflight measured {preflight} bytes but constructed state measured {}",
                    engine.retained_bytes
                ),
            ));
        }
        Ok(engine)
    }

    #[must_use]
    pub const fn snapshot(&self) -> &EffectiveEncyclopediaSnapshot {
        &self.published
    }

    #[must_use]
    pub fn retained_bytes(&self) -> u64 {
        self.account_state(&self.accepted, &self.published)
            .unwrap_or(u64::MAX)
    }

    #[must_use]
    pub fn accepted_mod_names(&self) -> Vec<&str> {
        self.accepted.keys().map(String::as_str).collect()
    }

    /// Test-only injection avoids allocating a 512 MiB boundary fixture.
    #[cfg(test)]
    pub fn set_retained_limit_for_test(&mut self, limit: u64) {
        self.retained_limit = limit;
    }

    #[cfg(test)]
    pub fn new_with_retained_limit_for_test(
        base: &EncyclopediaSession,
        limit: u64,
    ) -> Result<Self, EncyclopediaError> {
        Self::new_with_limit(base, limit)
    }

    #[cfg(test)]
    pub fn set_confinement_hook_for_test(
        &mut self,
        hook: Option<Arc<dyn Fn(ConfinementTestPoint) + Send + Sync>>,
    ) {
        self.confinement_hook = hook;
    }

    fn observe_confinement(&self, point: ConfinementTestPoint) {
        #[cfg(test)]
        if let Some(hook) = &self.confinement_hook {
            hook(point);
        }
        #[cfg(not(test))]
        let _ = point;
    }

    /// Test-only scaling of the independent 128 MiB effective-image gate.
    #[cfg(test)]
    pub fn set_effective_image_limit_for_test(&mut self, limit: u64) {
        self.effective_image_limit = limit;
    }

    /// Rebuilds from immutable base in exactly the supplied order, then publishes once.
    ///
    /// Per-mod parse/read/apply failures produce diagnostics and try that enabled
    /// mod's independent last-good snapshot. Session-wide resource failures reject
    /// the whole candidate and leave `self` byte-for-byte unchanged.
    pub fn refresh(
        &mut self,
        ordered: Vec<ResolvedEncyclopediaMod>,
    ) -> Result<EncyclopediaRefreshReport, EncyclopediaError> {
        if self.accepted.is_empty()
            && ordered
                .iter()
                .all(|item| matches!(item.content, ModContentTarget::Missing))
        {
            return Ok(EncyclopediaRefreshReport {
                diagnostics: Vec::new(),
                changed_image_ids: BTreeSet::new(),
                removed_image_ids: BTreeSet::new(),
                published_changed: false,
                peak_retained_bytes: self.retained_bytes(),
            });
        }

        let input_nonbuffer_reservation = input_nonbuffer_reservation(&ordered)?;
        let mut remaining_raw_capacity = input_raw_capacity(&ordered)?;
        let initial_input_reservation =
            checked_add_u64(input_nonbuffer_reservation, remaining_raw_capacity)?;
        let mut peak = self.retained_bytes();
        self.check_candidate_peak(
            &self.accepted,
            &self.base_catalog,
            &self.base_image_facts,
            &self.base_image_bytes,
            &self.base_image_owners,
            &[],
            initial_input_reservation,
            &mut peak,
        )?;
        reject_duplicate_names(&ordered)?;

        let enabled: BTreeSet<&str> = ordered.iter().map(|item| item.name.as_str()).collect();
        let mut working_accepted: BTreeMap<String, Arc<AcceptedModSnapshot>> = self
            .accepted
            .iter()
            .filter(|(name, _)| enabled.contains(name.as_str()))
            .map(|(name, snapshot)| (name.clone(), Arc::clone(snapshot)))
            .collect();
        let mut catalog = self.base_catalog.clone();
        let mut facts = self.base_image_facts.clone();
        let mut image_bytes = self.base_image_bytes.clone();
        let mut image_owners = self.base_image_owners.clone();
        let mut diagnostics = Vec::new();
        for item in ordered {
            let ResolvedEncyclopediaMod {
                name,
                root,
                content,
            } = item;
            if let ModContentTarget::Bytes(bytes) = &content {
                remaining_raw_capacity = remaining_raw_capacity
                    .checked_sub(u64::try_from(bytes.capacity()).map_err(|_| {
                        session_error(
                            "resource_limit:retained_bytes",
                            &name,
                            "incoming overlay capacity does not fit u64",
                        )
                    })?)
                    .ok_or_else(|| {
                        session_error(
                            "resource_limit:retained_bytes",
                            &name,
                            "incoming overlay accounting underflowed",
                        )
                    })?;
            }
            let transaction_reservation =
                checked_add_u64(input_nonbuffer_reservation, remaining_raw_capacity)?;
            if matches!(content, ModContentTarget::Missing) {
                working_accepted.remove(&name);
                continue;
            }

            let previous = self.accepted.get(&name).cloned();
            let candidate = match content {
                ModContentTarget::Bytes(bytes) => {
                    match self.build_snapshot(
                        &name,
                        &root,
                        bytes,
                        &working_accepted,
                        &catalog,
                        &facts,
                        &image_bytes,
                        &image_owners,
                        transaction_reservation,
                        &mut peak,
                    ) {
                        Ok(snapshot) => Some(snapshot),
                        Err(error) if is_session_resource_failure(&error) => return Err(error),
                        Err(error) => {
                            diagnostics.push(diagnostic(&name, &error));
                            None
                        }
                    }
                }
                ModContentTarget::ReadError { path, message, .. } => {
                    diagnostics.push(EncyclopediaModDiagnostic {
                        mod_name: name.clone(),
                        code: "mod_content_read_error",
                        path: path.display().to_string(),
                        message,
                    });
                    None
                }
                ModContentTarget::Missing => unreachable!("handled above"),
            };

            let mut applied = false;
            if let Some(candidate) = candidate {
                let candidate = previous
                    .as_ref()
                    .filter(|old| old.as_ref() == candidate.as_ref())
                    .cloned()
                    .unwrap_or(candidate);
                match self.apply_snapshot(
                    &name,
                    &candidate,
                    &catalog,
                    &facts,
                    &image_bytes,
                    &image_owners,
                    &working_accepted,
                    transaction_reservation,
                    &mut peak,
                ) {
                    Ok(next) => {
                        catalog = next.catalog;
                        facts = next.facts;
                        image_bytes = next.image_bytes;
                        image_owners = next.image_owners;
                        working_accepted.insert(name.clone(), candidate);
                        applied = true;
                    }
                    Err(error) if is_session_resource_failure(&error) => return Err(error),
                    Err(error) => diagnostics.push(diagnostic(&name, &error)),
                }
            }

            if !applied {
                if let Some(previous) = previous {
                    match self.apply_snapshot(
                        &name,
                        &previous,
                        &catalog,
                        &facts,
                        &image_bytes,
                        &image_owners,
                        &working_accepted,
                        transaction_reservation,
                        &mut peak,
                    ) {
                        Ok(next) => {
                            catalog = next.catalog;
                            facts = next.facts;
                            image_bytes = next.image_bytes;
                            image_owners = next.image_owners;
                            working_accepted.insert(name.clone(), previous);
                        }
                        Err(error) if is_session_resource_failure(&error) => return Err(error),
                        Err(error) => diagnostics.push(diagnostic(&name, &error)),
                    }
                } else {
                    working_accepted.remove(&name);
                }
            }
        }

        validate_effective_catalog(&catalog, &facts)?;
        self.ensure_effective_image_limit(&facts)?;
        verify_exact_provider_closure(&catalog, &facts, &image_bytes, &image_owners, |point| {
            self.observe_confinement(point)
        })?;

        let mut candidate_without_generation = EffectiveEncyclopediaSnapshot {
            inner: make_published_generation(
                catalog,
                image_bytes,
                image_owners,
                self.published.generation(),
            )?,
            tracker: Arc::clone(&self.external_retention),
            externally_tracked: false,
        };
        let changed_image_ids = changed_image_ids(&self.published, &candidate_without_generation);
        let removed_image_ids = removed_image_ids(&self.published, &candidate_without_generation);
        let published_changed =
            !same_effective_content(&self.published, &candidate_without_generation);
        let generation = if published_changed {
            self.published.generation().checked_add(1).ok_or_else(|| {
                session_error(
                    "generation_overflow",
                    "$",
                    "encyclopedia catalog generation overflowed",
                )
            })?
        } else {
            self.published.generation()
        };
        Arc::get_mut(&mut candidate_without_generation.inner)
            .expect("candidate publication has no external owners")
            .generation = generation;
        let final_usage = if published_changed {
            self.account_state(&working_accepted, &candidate_without_generation)?
        } else {
            self.account_state(&working_accepted, &self.published)?
        };
        peak = peak.max(final_usage);
        self.ensure_limit(peak)?;

        self.accepted = working_accepted;
        if published_changed {
            self.published = candidate_without_generation;
        }
        self.retained_bytes = final_usage;
        Ok(EncyclopediaRefreshReport {
            diagnostics,
            changed_image_ids,
            removed_image_ids,
            published_changed,
            peak_retained_bytes: peak,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn build_snapshot(
        &self,
        name: &str,
        root: &Path,
        bytes: Vec<u8>,
        working_accepted: &BTreeMap<String, Arc<AcceptedModSnapshot>>,
        catalog: &EncyclopediaCatalog,
        facts: &BTreeMap<String, AssetFacts>,
        image_bytes: &BTreeMap<String, Arc<[u8]>>,
        image_owners: &BTreeMap<String, EncyclopediaImageOwner>,
        transaction_reservation: u64,
        peak: &mut u64,
    ) -> Result<Arc<AcceptedModSnapshot>, EncyclopediaError> {
        let overlay_bytes = Arc::new(bytes);
        let parse_reservation = checked_add_u64(
            transaction_reservation,
            overlay_parse_working_reservation(overlay_bytes.capacity())?,
        )?;
        let provisional = AcceptedModSnapshot {
            overlay_bytes: Arc::clone(&overlay_bytes),
            patches: Vec::new(),
            images: BTreeMap::new(),
        };
        self.check_candidate_peak(
            working_accepted,
            catalog,
            facts,
            image_bytes,
            image_owners,
            &[(name, &provisional)],
            parse_reservation,
            peak,
        )?;

        let patches = parse_encyclopedia_overlay(overlay_bytes.as_slice())?;
        let mut snapshot = AcceptedModSnapshot {
            overlay_bytes,
            patches,
            images: BTreeMap::new(),
        };
        self.check_candidate_peak(
            working_accepted,
            catalog,
            facts,
            image_bytes,
            image_owners,
            &[(name, &snapshot)],
            transaction_reservation,
            peak,
        )?;

        for path in replacement_paths(&snapshot.patches) {
            let expected_format = path.rsplit_once('.').map_or("", |(_, value)| value);
            let retained = load_confined_image(
                root,
                path,
                expected_format,
                |length| {
                    self.check_candidate_peak(
                        working_accepted,
                        catalog,
                        facts,
                        image_bytes,
                        image_owners,
                        &[(name, &snapshot)],
                        checked_add_u64(transaction_reservation, length)?,
                        peak,
                    )
                },
                |point| self.observe_confinement(point),
            )?;
            snapshot.images.insert(path.to_owned(), retained);
            self.check_candidate_peak(
                working_accepted,
                catalog,
                facts,
                image_bytes,
                image_owners,
                &[(name, &snapshot)],
                transaction_reservation,
                peak,
            )?;
        }
        Ok(Arc::new(snapshot))
    }

    #[allow(clippy::too_many_arguments)]
    fn apply_snapshot(
        &self,
        name: &str,
        snapshot: &Arc<AcceptedModSnapshot>,
        catalog: &EncyclopediaCatalog,
        facts: &BTreeMap<String, AssetFacts>,
        image_bytes: &BTreeMap<String, Arc<[u8]>>,
        image_owners: &BTreeMap<String, EncyclopediaImageOwner>,
        working_accepted: &BTreeMap<String, Arc<AcceptedModSnapshot>>,
        transaction_reservation: u64,
        peak: &mut u64,
    ) -> Result<AppliedSnapshot, EncyclopediaError> {
        let identity_reservation = checked_identity_reservation(name, snapshot.images.keys())?;
        let clone_reservation =
            checked_clone_reservation(catalog, facts, image_bytes, image_owners, Some(snapshot))?;
        let application_reservation =
            checked_overlay_application_reservation(name, catalog, snapshot)?;
        let identity_reservation_u64 = u64::try_from(identity_reservation).map_err(|_| {
            session_error(
                "resource_limit:retained_bytes",
                name,
                "generated image identity reservation does not fit u64",
            )
        })?;
        self.check_candidate_peak(
            working_accepted,
            catalog,
            facts,
            image_bytes,
            image_owners,
            &[(name, snapshot)],
            checked_add_u64(
                transaction_reservation,
                checked_add_u64(
                    identity_reservation_u64,
                    checked_add_u64(clone_reservation, application_reservation)?,
                )?,
            )?,
            peak,
        )?;
        let replacements: BTreeMap<String, AssetFacts> = snapshot
            .images
            .iter()
            .map(|(path, retained)| (path.clone(), retained.facts.clone()))
            .collect();
        self.observe_confinement(ConfinementTestPoint::OverlayApplicationStarting);
        let next_catalog = apply_encyclopedia_overlay(
            catalog,
            name,
            &snapshot.patches,
            OverlayImageInputs {
                current: facts,
                replacements: &replacements,
                retained_identity_bytes: identity_reservation,
            },
        )?;

        let described: BTreeSet<String> =
            next_catalog.images.keys().map(|id| id.0.clone()).collect();
        let mut next_facts = facts.clone();
        let mut next_bytes = image_bytes.clone();
        let mut next_owners = image_owners.clone();
        next_facts.retain(|id, _| described.contains(id));
        next_bytes.retain(|id, _| described.contains(id));
        next_owners.retain(|id, _| described.contains(id));

        for (image_id, descriptor) in &next_catalog.images {
            if next_facts.contains_key(&image_id.0) {
                continue;
            }
            let matched = snapshot.images.iter().find(|(author_path, retained)| {
                author_path.strip_prefix("encyclopedia/") == Some(descriptor.path.as_str())
                    && retained.facts.sha256 == descriptor.sha256
                    && retained.facts.byte_len == descriptor.byte_length
            });
            let Some((author_path, retained)) = matched else {
                return Err(session_error(
                    "missing_image_provider",
                    format!("$.images.{}", image_id.0),
                    "effective descriptor has no exact owning snapshot bytes",
                ));
            };
            next_facts.insert(image_id.0.clone(), retained.facts.clone());
            next_bytes.insert(image_id.0.clone(), Arc::clone(&retained.bytes));
            next_owners.insert(
                image_id.0.clone(),
                EncyclopediaImageOwner::Mod {
                    mod_name: name.to_owned(),
                    path: author_path.clone(),
                },
            );
        }
        validate_effective_catalog(&next_catalog, &next_facts)?;
        self.ensure_effective_image_limit(&next_facts)?;
        verify_exact_provider_closure(
            &next_catalog,
            &next_facts,
            &next_bytes,
            &next_owners,
            |point| self.observe_confinement(point),
        )?;
        self.check_candidate_peak(
            working_accepted,
            &next_catalog,
            &next_facts,
            &next_bytes,
            &next_owners,
            &[(name, snapshot)],
            checked_add_u64(
                transaction_reservation,
                checked_clone_reservation(catalog, facts, image_bytes, image_owners, None)?,
            )?,
            peak,
        )?;
        Ok(AppliedSnapshot {
            catalog: next_catalog,
            facts: next_facts,
            image_bytes: next_bytes,
            image_owners: next_owners,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn check_candidate_peak(
        &self,
        working_accepted: &BTreeMap<String, Arc<AcceptedModSnapshot>>,
        catalog: &EncyclopediaCatalog,
        facts: &BTreeMap<String, AssetFacts>,
        image_bytes: &BTreeMap<String, Arc<[u8]>>,
        image_owners: &BTreeMap<String, EncyclopediaImageOwner>,
        transient: &[(&str, &AcceptedModSnapshot)],
        additional: u64,
        peak: &mut u64,
    ) -> Result<(), EncyclopediaError> {
        let usage = self.account_peak(
            working_accepted,
            catalog,
            facts,
            image_bytes,
            image_owners,
            transient,
            additional,
        )?;
        *peak = (*peak).max(usage);
        self.ensure_limit(usage)
    }

    fn ensure_limit(&self, usage: u64) -> Result<(), EncyclopediaError> {
        if usage > self.retained_limit {
            return Err(session_error(
                "resource_limit:retained_bytes",
                "$",
                format!(
                    "retained candidate requires {usage} bytes; limit is {}",
                    self.retained_limit
                ),
            ));
        }
        Ok(())
    }

    fn ensure_effective_image_limit(
        &self,
        facts: &BTreeMap<String, AssetFacts>,
    ) -> Result<(), EncyclopediaError> {
        let mut total = 0_u64;
        for fact in facts.values() {
            total = total.checked_add(fact.byte_len).ok_or_else(|| {
                session_error(
                    "resource_limit:effective_image_bytes",
                    "$.images",
                    "effective image byte total overflowed",
                )
            })?;
            if total > self.effective_image_limit {
                return Err(session_error(
                    "resource_limit:effective_image_bytes",
                    "$.images",
                    format!(
                        "effective image bytes require {total}; limit is {}",
                        self.effective_image_limit
                    ),
                ));
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn account_peak(
        &self,
        candidate_accepted: &BTreeMap<String, Arc<AcceptedModSnapshot>>,
        candidate_catalog: &EncyclopediaCatalog,
        candidate_facts: &BTreeMap<String, AssetFacts>,
        candidate_image_bytes: &BTreeMap<String, Arc<[u8]>>,
        candidate_image_owners: &BTreeMap<String, EncyclopediaImageOwner>,
        transient: &[(&str, &AcceptedModSnapshot)],
        additional: u64,
    ) -> Result<u64, EncyclopediaError> {
        let mut accounting = RetainedAccounting::default();
        accounting.add_catalog(&self.base_catalog)?;
        accounting.add_bytes(&self.base_bytes)?;
        accounting.add_facts(&self.base_image_facts)?;
        accounting.add_image_bytes(&self.base_image_bytes)?;
        accounting.add_owners(&self.base_image_owners)?;
        accounting.add_accepted(&self.accepted)?;
        accounting.add_published(&self.published)?;
        accounting.add_accepted(candidate_accepted)?;
        accounting.add_catalog(candidate_catalog)?;
        accounting.add_facts(candidate_facts)?;
        accounting.add_image_bytes(candidate_image_bytes)?;
        accounting.add_owners(candidate_image_owners)?;
        for (name, snapshot) in transient {
            accounting.add_string(name)?;
            accounting.add_snapshot_once(snapshot)?;
        }
        accounting.add_external(&self.external_retention)?;
        accounting.add_u64(additional)?;
        Ok(accounting.total)
    }

    fn account_state(
        &self,
        accepted: &BTreeMap<String, Arc<AcceptedModSnapshot>>,
        published: &EffectiveEncyclopediaSnapshot,
    ) -> Result<u64, EncyclopediaError> {
        let mut accounting = RetainedAccounting::default();
        accounting.add_catalog(&self.base_catalog)?;
        accounting.add_bytes(&self.base_bytes)?;
        accounting.add_facts(&self.base_image_facts)?;
        accounting.add_image_bytes(&self.base_image_bytes)?;
        accounting.add_owners(&self.base_image_owners)?;
        accounting.add_accepted(accepted)?;
        accounting.add_published(published)?;
        accounting.add_external(&self.external_retention)?;
        Ok(accounting.total)
    }
}

struct AppliedSnapshot {
    catalog: EncyclopediaCatalog,
    facts: BTreeMap<String, AssetFacts>,
    image_bytes: BTreeMap<String, Arc<[u8]>>,
    image_owners: BTreeMap<String, EncyclopediaImageOwner>,
}

fn reject_duplicate_names(ordered: &[ResolvedEncyclopediaMod]) -> Result<(), EncyclopediaError> {
    let mut names = BTreeSet::new();
    for item in ordered {
        if !names.insert(item.name.as_str()) {
            return Err(session_error(
                "duplicate_mod_name",
                &item.name,
                "resolved order contains an exact duplicate mod identity",
            ));
        }
    }
    Ok(())
}

fn replacement_paths(patches: &[TopicPatch]) -> BTreeSet<&str> {
    let mut paths = BTreeSet::new();
    for patch in patches {
        for localized in patch.localized.values() {
            let PatchField::Value(localized) = localized else {
                continue;
            };
            match &localized.image {
                PatchField::Value(ImagePatch::Static { path }) => {
                    paths.insert(path.as_str());
                }
                PatchField::Value(ImagePatch::ViewerFaction(pair)) => {
                    for side in [&pair.alliance, &pair.empire] {
                        if let FactionImagePatch::Value { path } = side {
                            paths.insert(path.as_str());
                        }
                    }
                }
                PatchField::Missing | PatchField::Null => {}
            }
        }
    }
    paths
}

fn checked_identity_reservation<'a>(
    mod_name: &str,
    paths: impl Iterator<Item = &'a String>,
) -> Result<usize, EncyclopediaError> {
    let mut total = 0_usize;
    for path in paths {
        let identity = mod_name
            .len()
            .checked_mul(2)
            .and_then(|length| length.checked_add(MOD_IMAGE_PREFIX.len()))
            .and_then(|length| length.checked_add(1))
            .and_then(|length| length.checked_add(path.len()))
            .ok_or_else(|| {
                session_error(
                    "identity_length_overflow",
                    path,
                    "generated image identity length overflowed",
                )
            })?;
        let reservation = identity
            .checked_add(mod_name.len())
            .and_then(|length| length.checked_add(path.len()))
            .ok_or_else(|| {
                session_error(
                    "identity_length_overflow",
                    path,
                    "generated image ownership length overflowed",
                )
            })?;
        total = total.checked_add(reservation).ok_or_else(|| {
            session_error(
                "identity_length_overflow",
                path,
                "generated image identity total overflowed",
            )
        })?;
    }
    Ok(total)
}

fn checked_clone_reservation(
    catalog: &EncyclopediaCatalog,
    facts: &BTreeMap<String, AssetFacts>,
    image_bytes: &BTreeMap<String, Arc<[u8]>>,
    owners: &BTreeMap<String, EncyclopediaImageOwner>,
    replacement: Option<&AcceptedModSnapshot>,
) -> Result<u64, EncyclopediaError> {
    let mut accounting = RetainedAccounting::default();
    accounting.add_catalog(catalog)?;
    accounting.add_facts(facts)?;
    for image_id in image_bytes.keys() {
        accounting.add_string(image_id)?;
    }
    accounting.add_owners(owners)?;
    if let Some(snapshot) = replacement {
        for (path, retained) in &snapshot.images {
            accounting.add_string(path)?;
            accounting.add_string(&retained.facts.sha256)?;
            if let Some(image) = &retained.facts.image {
                accounting.add_string(&image.format)?;
            }
        }
    }
    Ok(accounting.total)
}

/// Bytes copied while E22 builds a new effective catalog from the already
/// retained typed patch. This is an admission bound for owned string storage,
/// not an allocator-wide or total-process-memory estimate.
fn checked_overlay_application_reservation(
    mod_name: &str,
    catalog: &EncyclopediaCatalog,
    snapshot: &AcceptedModSnapshot,
) -> Result<u64, EncyclopediaError> {
    let mut accounting = RetainedAccounting::default();
    let mut prepared_paths = BTreeSet::new();

    for patch in &snapshot.patches {
        let topic = catalog.topics.get(&patch.id);
        for (language, localized_field) in &patch.localized {
            let PatchField::Value(localized) = localized_field else {
                continue;
            };
            accounting.add_string(language)?;
            let existing = topic.and_then(|topic| topic.localized.get(language));
            add_required_text_copy(
                &mut accounting,
                existing.map(|content| content.title.as_str()),
                &localized.title,
            )?;
            add_required_text_copy(
                &mut accounting,
                existing.map(|content| content.body.as_str()),
                &localized.body,
            )?;

            match &localized.image {
                PatchField::Missing => {
                    if let Some(content) = existing {
                        add_localized_image_copy(&mut accounting, content)?;
                    }
                }
                PatchField::Null => {}
                PatchField::Value(ImagePatch::Static { path }) => {
                    accounting.add_usize(checked_generated_identity_length(mod_name, path)?)?;
                    prepared_paths.insert(path.as_str());
                }
                PatchField::Value(ImagePatch::ViewerFaction(pair)) => {
                    accounting.add_string("viewer_faction")?;
                    for side in [&pair.alliance, &pair.empire] {
                        if let FactionImagePatch::Value { path } = side {
                            accounting
                                .add_usize(checked_generated_identity_length(mod_name, path)?)?;
                            prepared_paths.insert(path.as_str());
                        }
                    }
                }
            }
        }
    }

    for path in prepared_paths {
        let identity_len = checked_generated_identity_length(mod_name, path)?;
        // The identity reservation passed into E22 separately accounts for one
        // generated ID plus the retained owner name/path. These three copies
        // cover descriptor/facts/prepared keys; each localized reference was
        // counted above. This also bounds the later bytes/owner map keys once
        // the temporary prepared map is released.
        accounting.add_usize(identity_len.checked_mul(3).ok_or_else(|| {
            session_error(
                "resource_limit:retained_bytes",
                path,
                "generated image identity copy accounting overflowed",
            )
        })?)?;
        accounting.add_string(path)?;
        if let Some(runtime_path) = path.strip_prefix("encyclopedia/") {
            accounting.add_string(runtime_path)?;
        }
        if let Some(retained) = snapshot.images.get(path) {
            // Descriptor and effective-facts copies coexist.
            accounting.add_string(&retained.facts.sha256)?;
            accounting.add_string(&retained.facts.sha256)?;
            if let Some(image) = &retained.facts.image {
                accounting.add_string(&image.format)?;
                accounting.add_string(&image.format)?;
            }
        }
        accounting.add_string("runtime:mod-snapshot")?;
    }

    Ok(accounting.total)
}

fn add_required_text_copy(
    accounting: &mut RetainedAccounting,
    existing: Option<&str>,
    patch: &PatchField<String>,
) -> Result<(), EncyclopediaError> {
    match patch {
        PatchField::Missing => {
            if let Some(existing) = existing {
                accounting.add_string(existing)?;
            }
        }
        PatchField::Value(value) => accounting.add_string(value)?,
        PatchField::Null => {}
    }
    Ok(())
}

fn add_localized_image_copy(
    accounting: &mut RetainedAccounting,
    content: &LocalizedContent,
) -> Result<(), EncyclopediaError> {
    if let BaseImageIdField::Value(image_id) = &content.image_id {
        accounting.add_string(&image_id.0)?;
    }
    if let Some(selector) = &content.image_selector {
        accounting.add_string(&selector.kind)?;
        for image_id in [&selector.alliance_image_id, &selector.empire_image_id] {
            if let NullableBaseImageId::Value(image_id) = image_id {
                accounting.add_string(&image_id.0)?;
            }
        }
    }
    Ok(())
}

fn checked_generated_identity_length(
    mod_name: &str,
    path: &str,
) -> Result<usize, EncyclopediaError> {
    mod_name
        .len()
        .checked_mul(2)
        .and_then(|length| length.checked_add(MOD_IMAGE_PREFIX.len()))
        .and_then(|length| length.checked_add(1))
        .and_then(|length| length.checked_add(path.len()))
        .ok_or_else(|| {
            session_error(
                "identity_length_overflow",
                path,
                "generated image identity length overflowed",
            )
        })
}

fn checked_add_u64(left: u64, right: u64) -> Result<u64, EncyclopediaError> {
    left.checked_add(right).ok_or_else(|| {
        session_error(
            "resource_limit:retained_bytes",
            "$",
            "candidate reservation overflowed",
        )
    })
}

fn ensure_retained_limit(usage: u64, limit: u64) -> Result<(), EncyclopediaError> {
    if usage > limit {
        return Err(session_error(
            "resource_limit:retained_bytes",
            "$",
            format!("retained candidate requires {usage} bytes; limit is {limit}"),
        ));
    }
    Ok(())
}

fn make_published_generation(
    catalog: EncyclopediaCatalog,
    image_bytes: BTreeMap<String, Arc<[u8]>>,
    image_owners: BTreeMap<String, EncyclopediaImageOwner>,
    generation: u64,
) -> Result<Arc<PublishedGeneration>, EncyclopediaError> {
    let mut accounting = RetainedAccounting::default();
    accounting.add_catalog(&catalog)?;
    for image_id in image_bytes.keys() {
        accounting.add_string(image_id)?;
    }
    accounting.add_owners(&image_owners)?;
    Ok(Arc::new(PublishedGeneration {
        catalog,
        image_bytes,
        image_owners,
        generation,
        metadata_bytes: accounting.total,
    }))
}

fn preflight_base_engine_bytes(base: &EncyclopediaSession) -> Result<u64, EncyclopediaError> {
    let mut accounting = RetainedAccounting::default();
    accounting.add_catalog(base.base_catalog())?;
    accounting.add_bytes(base.base_bytes())?;
    for (image_id, descriptor) in &base.base_catalog().images {
        let facts = base.observed_facts().get(&descriptor.path).ok_or_else(|| {
            session_error(
                "missing_image_provider",
                &descriptor.path,
                "validated base session omits observed image facts",
            )
        })?;
        let bytes = base.base_bytes().get(&descriptor.path).ok_or_else(|| {
            session_error(
                "missing_runtime_file",
                &descriptor.path,
                "validated base session omits retained image bytes",
            )
        })?;
        accounting.add_string(&image_id.0)?;
        accounting.add_string(&facts.sha256)?;
        if let Some(image) = &facts.image {
            accounting.add_string(&image.format)?;
        }
        accounting.add_string(&image_id.0)?;
        accounting.add_arc(bytes)?;
        accounting.add_string(&image_id.0)?;
        accounting.add_string(&descriptor.path)?;
    }
    // The published generation owns an independent catalog and map/owner keys,
    // while its image buffers share the already-counted immutable base Arcs.
    accounting.add_catalog(base.base_catalog())?;
    for (image_id, descriptor) in &base.base_catalog().images {
        accounting.add_string(&image_id.0)?;
        accounting.add_string(&image_id.0)?;
        accounting.add_string(&descriptor.path)?;
    }
    Ok(accounting.total)
}

fn input_raw_capacity(ordered: &[ResolvedEncyclopediaMod]) -> Result<u64, EncyclopediaError> {
    ordered.iter().try_fold(0_u64, |total, item| {
        let ModContentTarget::Bytes(bytes) = &item.content else {
            return Ok(total);
        };
        let capacity = u64::try_from(bytes.capacity()).map_err(|_| {
            session_error(
                "resource_limit:retained_bytes",
                &item.name,
                "incoming overlay capacity does not fit u64",
            )
        })?;
        checked_add_u64(total, capacity)
    })
}

fn input_nonbuffer_reservation(
    ordered: &[ResolvedEncyclopediaMod],
) -> Result<u64, EncyclopediaError> {
    ordered.iter().try_fold(0_u64, |total, item| {
        if !matches!(item.content, ModContentTarget::Bytes(_)) {
            return Ok(total);
        }
        let identity = item
            .name
            .len()
            .checked_add(item.root.as_os_str().len())
            .ok_or_else(|| {
                session_error(
                    "resource_limit:retained_bytes",
                    &item.name,
                    "incoming mod identity accounting overflowed",
                )
            })?;
        checked_add_u64(
            total,
            u64::try_from(identity).map_err(|_| {
                session_error(
                    "resource_limit:retained_bytes",
                    &item.name,
                    "incoming mod identity length does not fit u64",
                )
            })?,
        )
    })
}

fn image_transition_reservation(length: u64) -> Result<u64, EncyclopediaError> {
    let compressed_overlap = length.checked_mul(2).ok_or_else(|| {
        session_error(
            "resource_limit:retained_bytes",
            "$",
            "image read-to-Arc overlap accounting overflowed",
        )
    })?;
    checked_add_u64(compressed_overlap, IMAGE_DECODE_WORKING_BYTES)
}

fn overlay_parse_working_reservation(capacity: usize) -> Result<u64, EncyclopediaError> {
    u64::try_from(capacity)
        .ok()
        .and_then(|bytes| bytes.checked_mul(OVERLAY_PARSE_WORKING_BYTES_PER_INPUT_BYTE))
        .ok_or_else(|| {
            session_error(
                "resource_limit:retained_bytes",
                "$",
                "overlay parser working-set accounting overflowed",
            )
        })
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn load_confined_image<F, O>(
    root: &Path,
    relative: &str,
    format: &str,
    mut reserve: F,
    mut observe: O,
) -> Result<RetainedImage, EncyclopediaError>
where
    F: FnMut(u64) -> Result<(), EncyclopediaError>,
    O: FnMut(ConfinementTestPoint),
{
    let mut directory = open_directory_nofollow(root).map_err(|error| {
        session_error(
            "mod_image_read_error",
            root.display().to_string(),
            format!("cannot pin no-follow mod root: {error}"),
        )
    })?;
    observe(ConfinementTestPoint::RootPinned);

    let relative_path = Path::new(relative);
    let mut components = relative_path.components();
    let final_component = components.next_back().ok_or_else(|| {
        session_error(
            "unsafe_asset_path",
            relative,
            "image path contains no file component",
        )
    })?;
    let std::path::Component::Normal(final_component) = final_component else {
        return Err(session_error(
            "unsafe_asset_path",
            relative,
            "image path is not confined to normal relative components",
        ));
    };
    for component in components {
        let std::path::Component::Normal(component) = component else {
            return Err(session_error(
                "unsafe_asset_path",
                relative,
                "image path is not confined to normal relative components",
            ));
        };
        let child = proc_fd_child(&directory, component);
        directory = open_directory_nofollow(&child).map_err(|error| {
            session_error(
                "unsafe_asset_path",
                relative,
                format!("cannot pin no-follow image directory: {error}"),
            )
        })?;
    }
    observe(ConfinementTestPoint::ParentPinned);

    let final_path = proc_fd_child(&directory, final_component);
    let pinned = open_path_nofollow(&final_path).map_err(|error| {
        session_error(
            "unsafe_asset_path",
            relative,
            format!("cannot pin no-follow image file: {error}"),
        )
    })?;
    let pinned_metadata = pinned
        .metadata()
        .map_err(|error| session_error("mod_image_read_error", relative, error.to_string()))?;
    if !pinned_metadata.is_file() {
        return Err(session_error(
            "mod_image_read_error",
            relative,
            "confined image target is not a regular file",
        ));
    }
    let length = pinned_metadata.len();
    if length == 0 || length > MAX_ENCYCLOPEDIA_IMAGE_BYTES as u64 {
        return Err(session_error(
            "resource_limit:image_bytes",
            relative,
            format!("image length {length} is outside the bounded range"),
        ));
    }
    reserve(image_transition_reservation(length)?)?;

    let pinned_path = proc_fd_path(&pinned);
    let mut file = OpenOptions::new()
        .read(true)
        .open(&pinned_path)
        .map_err(|error| {
            session_error(
                "mod_image_read_error",
                relative,
                format!("cannot open pinned confined image: {error}"),
            )
        })?;
    let opened = file
        .metadata()
        .map_err(|error| session_error("mod_image_read_error", relative, error.to_string()))?;
    if !opened.is_file() || !same_pinned_file_identity(&pinned_metadata, &opened) {
        return Err(session_error(
            "mod_image_source_changed",
            relative,
            "pinned image identity changed before the bounded read",
        ));
    }
    let capacity = usize::try_from(length).map_err(|_| {
        session_error(
            "resource_limit:image_bytes",
            relative,
            "image length does not fit address space",
        )
    })?;
    let mut bytes = vec![0_u8; capacity];
    file.read_exact(&mut bytes).map_err(|error| {
        session_error(
            "mod_image_source_changed",
            relative,
            format!("pinned image changed during the exact read: {error}"),
        )
    })?;
    let mut extra = [0_u8; 1];
    if file.read(&mut extra).map_err(|error| {
        session_error(
            "mod_image_read_error",
            relative,
            format!("cannot finish bounded image read: {error}"),
        )
    })? != 0
    {
        return Err(session_error(
            "mod_image_source_changed",
            relative,
            "pinned image grew during the bounded read",
        ));
    }
    observe(ConfinementTestPoint::ImageInspectionStarting);
    let inspected = inspect_encyclopedia_bytes(&bytes, Some(format))
        .map_err(|detail| session_error("invalid_image", relative, detail))?;
    let image = match (inspected.format, inspected.width, inspected.height) {
        (Some(format), Some(width), Some(height)) => ImageFacts {
            format,
            width,
            height,
        },
        _ => {
            return Err(session_error(
                "invalid_image",
                relative,
                "byte inspector returned incomplete image facts",
            ));
        }
    };
    Ok(RetainedImage {
        bytes: Arc::from(bytes),
        facts: AssetFacts {
            sha256: inspected.sha256,
            byte_len: inspected.byte_len,
            image: Some(image),
        },
    })
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn proc_fd_path(file: &File) -> PathBuf {
    PathBuf::from(format!("/proc/self/fd/{}", file.as_raw_fd()))
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn proc_fd_child(file: &File, component: &std::ffi::OsStr) -> PathBuf {
    let mut path = proc_fd_path(file);
    path.push(component);
    path
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn open_directory_nofollow(path: &Path) -> std::io::Result<File> {
    const O_NOFOLLOW: i32 = 0o400000;
    const O_DIRECTORY: i32 = 0o200000;
    const O_CLOEXEC: i32 = 0o2000000;
    OpenOptions::new()
        .read(true)
        .custom_flags(O_NOFOLLOW | O_DIRECTORY | O_CLOEXEC)
        .open(path)
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn open_path_nofollow(path: &Path) -> std::io::Result<File> {
    const O_NOFOLLOW: i32 = 0o400000;
    const O_CLOEXEC: i32 = 0o2000000;
    const O_PATH: i32 = 0o10000000;
    OpenOptions::new()
        .read(true)
        .custom_flags(O_NOFOLLOW | O_CLOEXEC | O_PATH)
        .open(path)
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn same_pinned_file_identity(before: &std::fs::Metadata, opened: &std::fs::Metadata) -> bool {
    before.dev() == opened.dev() && before.ino() == opened.ino() && before.len() == opened.len()
}

#[cfg(all(
    not(target_arch = "wasm32"),
    not(any(target_os = "linux", target_os = "android"))
))]
fn load_confined_image<F, O>(
    _root: &Path,
    relative: &str,
    _format: &str,
    _reserve: F,
    _observe: O,
) -> Result<RetainedImage, EncyclopediaError>
where
    F: FnMut(u64) -> Result<(), EncyclopediaError>,
    O: FnMut(ConfinementTestPoint),
{
    Err(session_error(
        "secure_confinement_unavailable",
        relative,
        "this platform lacks the pinned no-follow directory-handle implementation",
    ))
}

#[cfg(target_arch = "wasm32")]
fn load_confined_image<F, O>(
    _root: &Path,
    relative: &str,
    _format: &str,
    _reserve: F,
    _observe: O,
) -> Result<RetainedImage, EncyclopediaError>
where
    F: FnMut(u64) -> Result<(), EncyclopediaError>,
    O: FnMut(ConfinementTestPoint),
{
    Err(session_error(
        "native_only",
        relative,
        "native mod filesystem loading is unavailable on wasm32",
    ))
}

fn verify_exact_provider_closure<O>(
    catalog: &EncyclopediaCatalog,
    facts: &BTreeMap<String, AssetFacts>,
    bytes: &BTreeMap<String, Arc<[u8]>>,
    owners: &BTreeMap<String, EncyclopediaImageOwner>,
    _observe: O,
) -> Result<(), EncyclopediaError>
where
    O: FnMut(ConfinementTestPoint),
{
    let described: BTreeSet<String> = catalog.images.keys().map(|id| id.0.clone()).collect();
    if bytes.keys().cloned().collect::<BTreeSet<_>>() != described
        || owners.keys().cloned().collect::<BTreeSet<_>>() != described
        || facts.keys().cloned().collect::<BTreeSet<_>>() != described
    {
        return Err(session_error(
            "missing_image_provider",
            "$.images",
            "catalog identities, facts, retained bytes, and owners do not close exactly",
        ));
    }
    for image_id in described {
        let retained = &bytes[&image_id];
        let observed = &facts[&image_id];
        // Base/session construction and confined mod loading already performed
        // the bounded full decode that produced `observed.image`. At this
        // closure boundary only prove that the retained provider is still the
        // exact authenticated allocation; re-decoding would allocate codec
        // workspace again during otherwise text-only or last-good refreshes.
        let inspected = inspect_encyclopedia_bytes(retained, None)
            .map_err(|detail| session_error("invalid_image", &image_id, detail))?;
        if inspected.sha256 != observed.sha256 || inspected.byte_len != observed.byte_len {
            return Err(session_error(
                "image_facts_mismatch",
                &image_id,
                "retained provider bytes differ from published observations",
            ));
        }
    }
    Ok(())
}

fn diagnostic(mod_name: &str, error: &EncyclopediaError) -> EncyclopediaModDiagnostic {
    EncyclopediaModDiagnostic {
        mod_name: mod_name.to_owned(),
        code: error.code(),
        path: error.path().to_owned(),
        message: error.to_string(),
    }
}

fn is_session_resource_failure(error: &EncyclopediaError) -> bool {
    matches!(
        error.code(),
        "resource_limit:retained_bytes" | "resource_limit:effective_image_bytes"
    )
}

fn same_effective_content(
    left: &EffectiveEncyclopediaSnapshot,
    right: &EffectiveEncyclopediaSnapshot,
) -> bool {
    left.inner.catalog == right.inner.catalog
        && left.inner.image_bytes == right.inner.image_bytes
        && left.inner.image_owners == right.inner.image_owners
}

fn changed_image_ids(
    old: &EffectiveEncyclopediaSnapshot,
    new: &EffectiveEncyclopediaSnapshot,
) -> BTreeSet<String> {
    new.inner
        .image_bytes
        .iter()
        .filter(|(id, bytes)| {
            old.inner.image_bytes.get(*id).map(AsRef::as_ref) != Some(bytes.as_ref())
                || old.inner.image_owners.get(*id) != new.inner.image_owners.get(*id)
        })
        .map(|(id, _)| id.clone())
        .collect()
}

fn removed_image_ids(
    old: &EffectiveEncyclopediaSnapshot,
    new: &EffectiveEncyclopediaSnapshot,
) -> BTreeSet<String> {
    old.inner
        .image_bytes
        .keys()
        .filter(|id| !new.inner.image_bytes.contains_key(*id))
        .cloned()
        .collect()
}

fn session_error(
    code: &'static str,
    path: impl Into<String>,
    detail: impl Into<String>,
) -> EncyclopediaError {
    EncyclopediaError::for_session(code, path, detail)
}

#[derive(Default)]
struct RetainedAccounting {
    total: u64,
    storage: BTreeSet<(usize, usize)>,
    snapshots: BTreeSet<usize>,
    publications: BTreeSet<usize>,
}

impl RetainedAccounting {
    fn add_u64(&mut self, value: u64) -> Result<(), EncyclopediaError> {
        self.total = self.total.checked_add(value).ok_or_else(|| {
            session_error(
                "resource_limit:retained_bytes",
                "$",
                "retained-byte accounting overflowed",
            )
        })?;
        Ok(())
    }

    fn add_usize(&mut self, value: usize) -> Result<(), EncyclopediaError> {
        let value = u64::try_from(value).map_err(|_| {
            session_error(
                "resource_limit:retained_bytes",
                "$",
                "retained allocation length does not fit u64",
            )
        })?;
        self.add_u64(value)
    }

    fn add_string(&mut self, value: &str) -> Result<(), EncyclopediaError> {
        self.add_usize(value.len())
    }

    fn add_arc(&mut self, value: &Arc<[u8]>) -> Result<(), EncyclopediaError> {
        let identity = (value.as_ptr() as usize, value.len());
        if self.storage.insert(identity) {
            self.add_usize(value.len())?;
        }
        Ok(())
    }

    fn add_vec_arc(&mut self, value: &Arc<Vec<u8>>) -> Result<(), EncyclopediaError> {
        let identity = (value.as_ptr() as usize, value.capacity());
        if self.storage.insert(identity) {
            self.add_usize(value.capacity())?;
        }
        Ok(())
    }

    fn add_bytes(&mut self, bytes: &EncyclopediaBytes) -> Result<(), EncyclopediaError> {
        for (path, bytes) in bytes {
            self.add_string(path)?;
            self.add_arc(bytes)?;
        }
        Ok(())
    }

    fn add_image_bytes(
        &mut self,
        bytes: &BTreeMap<String, Arc<[u8]>>,
    ) -> Result<(), EncyclopediaError> {
        for (image_id, bytes) in bytes {
            self.add_string(image_id)?;
            self.add_arc(bytes)?;
        }
        Ok(())
    }

    fn add_facts(&mut self, facts: &BTreeMap<String, AssetFacts>) -> Result<(), EncyclopediaError> {
        for (image_id, facts) in facts {
            self.add_string(image_id)?;
            self.add_string(&facts.sha256)?;
            if let Some(image) = &facts.image {
                self.add_string(&image.format)?;
            }
        }
        Ok(())
    }

    fn add_owners(
        &mut self,
        owners: &BTreeMap<String, EncyclopediaImageOwner>,
    ) -> Result<(), EncyclopediaError> {
        for (image_id, owner) in owners {
            self.add_string(image_id)?;
            match owner {
                EncyclopediaImageOwner::Base { path } => self.add_string(path)?,
                EncyclopediaImageOwner::Mod { mod_name, path } => {
                    self.add_string(mod_name)?;
                    self.add_string(path)?;
                }
            }
        }
        Ok(())
    }

    fn add_accepted(
        &mut self,
        accepted: &BTreeMap<String, Arc<AcceptedModSnapshot>>,
    ) -> Result<(), EncyclopediaError> {
        for (name, snapshot) in accepted {
            self.add_string(name)?;
            self.add_snapshot_once(snapshot)?;
        }
        Ok(())
    }

    fn add_snapshot_once(
        &mut self,
        snapshot: &AcceptedModSnapshot,
    ) -> Result<(), EncyclopediaError> {
        let identity = std::ptr::from_ref(snapshot) as usize;
        if self.snapshots.insert(identity) {
            self.add_snapshot(snapshot)?;
        }
        Ok(())
    }

    fn add_snapshot(&mut self, snapshot: &AcceptedModSnapshot) -> Result<(), EncyclopediaError> {
        self.add_vec_arc(&snapshot.overlay_bytes)?;
        self.add_patches(&snapshot.patches)?;
        for (path, retained) in &snapshot.images {
            self.add_string(path)?;
            self.add_arc(&retained.bytes)?;
            self.add_string(&retained.facts.sha256)?;
            if let Some(image) = &retained.facts.image {
                self.add_string(&image.format)?;
            }
        }
        Ok(())
    }

    fn add_patches(&mut self, patches: &[TopicPatch]) -> Result<(), EncyclopediaError> {
        for patch in patches {
            self.add_string(&patch.id.0)?;
            for (language, localized) in &patch.localized {
                self.add_string(language)?;
                let PatchField::Value(localized) = localized else {
                    continue;
                };
                for text in [&localized.title, &localized.body] {
                    if let PatchField::Value(text) = text {
                        self.add_string(text)?;
                    }
                }
                match &localized.image {
                    PatchField::Value(ImagePatch::Static { path }) => self.add_string(path)?,
                    PatchField::Value(ImagePatch::ViewerFaction(pair)) => {
                        for side in [&pair.alliance, &pair.empire] {
                            if let FactionImagePatch::Value { path } = side {
                                self.add_string(path)?;
                            }
                        }
                    }
                    PatchField::Missing | PatchField::Null => {}
                }
            }
        }
        Ok(())
    }

    fn add_published(
        &mut self,
        published: &EffectiveEncyclopediaSnapshot,
    ) -> Result<(), EncyclopediaError> {
        let identity = Arc::as_ptr(&published.inner) as usize;
        if self.publications.insert(identity) {
            self.add_u64(published.inner.metadata_bytes)?;
            for bytes in published.inner.image_bytes.values() {
                self.add_arc(bytes)?;
            }
        }
        Ok(())
    }

    fn add_external(
        &mut self,
        tracker: &ExternalRetentionTracker,
    ) -> Result<(), EncyclopediaError> {
        let state = tracker
            .state
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        for (identity, (bytes, _)) in &state.publications {
            if self.publications.insert(*identity) {
                self.add_u64(*bytes)?;
            }
        }
        for (identity, (bytes, _)) in &state.buffers {
            if self.storage.insert(*identity) {
                self.add_u64(*bytes)?;
            }
        }
        Ok(())
    }

    fn add_catalog(&mut self, catalog: &EncyclopediaCatalog) -> Result<(), EncyclopediaError> {
        self.add_string(&catalog.default_language)?;
        self.add_string(&catalog.topic_sort.algorithm)?;
        self.add_string(&catalog.topic_sort.representable_encoding)?;
        self.add_string(&catalog.topic_sort.representable_fold)?;
        self.add_string(&catalog.topic_sort.unrepresentable)?;
        self.add_string(&catalog.topic_sort.tie_break)?;
        self.add_string(&catalog.index.command)?;
        self.add_string(&catalog.index.source_ref)?;
        self.add_labels_and_topics(&catalog.index.labels, &catalog.index.topic_ids)?;
        for category in &catalog.categories {
            self.add_string(&category.id)?;
            self.add_string(&category.command)?;
            self.add_string(&category.source_ref)?;
            self.add_labels_and_topics(&category.labels, &category.topic_ids)?;
        }
        for (topic_id, topic) in &catalog.topics {
            self.add_string(&topic_id.0)?;
            self.add_string(&topic.source_ref)?;
            for (language, localized) in &topic.localized {
                self.add_string(language)?;
                self.add_localized(localized)?;
            }
        }
        for (image_id, image) in &catalog.images {
            self.add_string(&image_id.0)?;
            self.add_string(&image.path)?;
            self.add_string(&image.format)?;
            self.add_string(&image.sha256)?;
            self.add_string(&image.source_ref)?;
        }
        for binding in &catalog.bindings {
            self.add_string(&binding.family)?;
            self.add_string(&binding.variant)?;
            self.add_string(&binding.topic_id.0)?;
        }
        Ok(())
    }

    fn add_labels_and_topics(
        &mut self,
        labels: &BTreeMap<String, String>,
        topics: &[rebellion_data::encyclopedia::TopicId],
    ) -> Result<(), EncyclopediaError> {
        for (language, label) in labels {
            self.add_string(language)?;
            self.add_string(label)?;
        }
        for topic in topics {
            self.add_string(&topic.0)?;
        }
        Ok(())
    }

    fn add_localized(&mut self, localized: &LocalizedContent) -> Result<(), EncyclopediaError> {
        self.add_string(&localized.title)?;
        self.add_string(&localized.body)?;
        if let BaseImageIdField::Value(image_id) = &localized.image_id {
            self.add_string(&image_id.0)?;
        }
        if let Some(selector) = &localized.image_selector {
            self.add_string(&selector.kind)?;
            for image_id in [&selector.alliance_image_id, &selector.empire_image_id] {
                if let NullableBaseImageId::Value(image_id) = image_id {
                    self.add_string(&image_id.0)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn engine_from_synthetic_bundle() -> EncyclopediaModEngine {
        let bytes = BTreeMap::from([
            ("catalog.json".to_owned(), Arc::from(VALID_CATALOG)),
            ("manifest.json".to_owned(), Arc::from(VALID_MANIFEST)),
            ("assets/EDATA.001".to_owned(), Arc::from(VALID_IMAGE_1)),
            ("assets/EDATA.002".to_owned(), Arc::from(VALID_IMAGE_2)),
            ("assets/EDATA.003".to_owned(), Arc::from(VALID_IMAGE_3)),
        ]);
        let dats = BTreeMap::from([(
            "SYNTHETIC.DAT".to_owned(),
            "39fb2c329d34fbdd94bb2a2a596b694597eb00b8b402905ec4920f560210d322".to_owned(),
        )]);
        let session = crate::encyclopedia_session::prepare_encyclopedia_session(bytes, &dats)
            .expect("synthetic base session");
        EncyclopediaModEngine::new(&session).expect("mod engine")
    }

    #[test]
    fn shared_arc_storage_is_charged_once_while_distinct_keys_are_charged_separately() {
        let shared: Arc<[u8]> = Arc::from([1_u8, 2, 3]);
        let bytes = BTreeMap::from([
            ("a".to_owned(), Arc::clone(&shared)),
            ("bb".to_owned(), shared),
        ]);
        let mut accounting = RetainedAccounting::default();
        accounting.add_image_bytes(&bytes).unwrap();
        assert_eq!(
            accounting.total, 6,
            "three key bytes plus three shared bytes"
        );
    }

    #[test]
    fn identity_reservation_is_checked_before_building_the_encoded_name() {
        let paths = ["encyclopedia/assets/test.png".to_owned()];
        let identity_length = "demo".len() * 2 + MOD_IMAGE_PREFIX.len() + 1 + paths[0].len();
        let expected = identity_length + "demo".len() + paths[0].len();
        assert_eq!(
            checked_generated_identity_length("demo", &paths[0]).unwrap(),
            identity_length
        );
        assert_eq!(
            checked_identity_reservation("demo", paths.iter()).unwrap(),
            expected
        );
    }

    #[test]
    fn application_copy_reservation_includes_inherited_text_and_image_references() {
        let engine = engine_from_synthetic_bundle();
        let overlay = br#"[{"id":"original:60001","localized":{"1033":{"title":"x"}}}]"#.to_vec();
        let snapshot = AcceptedModSnapshot {
            patches: parse_encyclopedia_overlay(&overlay).unwrap(),
            overlay_bytes: Arc::new(overlay),
            images: BTreeMap::new(),
        };
        let existing = &engine.base_catalog.topics["original:60001"].localized["1033"];
        let inherited_image_bytes = match &existing.image_id {
            BaseImageIdField::Value(image_id) => image_id.0.len(),
            BaseImageIdField::Absent | BaseImageIdField::Null => 0,
        } + existing.image_selector.as_ref().map_or(0, |selector| {
            selector.kind.len()
                + [&selector.alliance_image_id, &selector.empire_image_id]
                    .into_iter()
                    .map(|image_id| match image_id {
                        NullableBaseImageId::Value(image_id) => image_id.0.len(),
                        NullableBaseImageId::Null => 0,
                    })
                    .sum::<usize>()
        });
        let expected = "1033".len() + "x".len() + existing.body.len() + inherited_image_bytes;
        assert_eq!(
            checked_overlay_application_reservation("demo", &engine.base_catalog, &snapshot)
                .unwrap(),
            expected as u64
        );
    }

    #[test]
    fn incoming_identity_and_image_working_reservations_have_exact_nonpartitioned_arithmetic() {
        let root = PathBuf::from("synthetic-root");
        let content = ResolvedEncyclopediaMod {
            name: "name:Δ".to_owned(),
            root: root.clone(),
            content: ModContentTarget::Bytes(b"[]".to_vec()),
        };
        assert_eq!(
            input_nonbuffer_reservation(&[content]).unwrap(),
            ("name:Δ".len() + root.as_os_str().len()) as u64
        );
        let missing = ResolvedEncyclopediaMod {
            name: "unrelated".to_owned(),
            root,
            content: ModContentTarget::Missing,
        };
        assert_eq!(input_nonbuffer_reservation(&[missing]).unwrap(), 0);
        assert_eq!(
            image_transition_reservation(7).unwrap(),
            IMAGE_DECODE_WORKING_BYTES + 14
        );
        assert_eq!(overlay_parse_working_reservation(7).unwrap(), 448);
    }

    #[test]
    fn exact_provider_closure_rejects_each_missing_map_and_each_observation_mismatch() {
        let engine = engine_from_synthetic_bundle();
        let image_id = engine.base_catalog.images.keys().next().unwrap().0.clone();

        let mut bytes = engine.base_image_bytes.clone();
        bytes.remove(&image_id);
        assert_eq!(
            verify_exact_provider_closure(
                &engine.base_catalog,
                &engine.base_image_facts,
                &bytes,
                &engine.base_image_owners,
                |_| {},
            )
            .unwrap_err()
            .code(),
            "missing_image_provider"
        );

        let mut owners = engine.base_image_owners.clone();
        owners.remove(&image_id);
        assert_eq!(
            verify_exact_provider_closure(
                &engine.base_catalog,
                &engine.base_image_facts,
                &engine.base_image_bytes,
                &owners,
                |_| {},
            )
            .unwrap_err()
            .code(),
            "missing_image_provider"
        );

        let mut facts = engine.base_image_facts.clone();
        facts.remove(&image_id);
        assert_eq!(
            verify_exact_provider_closure(
                &engine.base_catalog,
                &facts,
                &engine.base_image_bytes,
                &engine.base_image_owners,
                |_| {},
            )
            .unwrap_err()
            .code(),
            "missing_image_provider"
        );

        let mut wrong_digest = engine.base_image_facts.clone();
        wrong_digest.get_mut(&image_id).unwrap().sha256 = "0".repeat(64);
        assert_eq!(
            verify_exact_provider_closure(
                &engine.base_catalog,
                &wrong_digest,
                &engine.base_image_bytes,
                &engine.base_image_owners,
                |_| {},
            )
            .unwrap_err()
            .code(),
            "image_facts_mismatch"
        );

        let mut wrong_length = engine.base_image_facts.clone();
        wrong_length.get_mut(&image_id).unwrap().byte_len += 1;
        assert_eq!(
            verify_exact_provider_closure(
                &engine.base_catalog,
                &wrong_length,
                &engine.base_image_bytes,
                &engine.base_image_owners,
                |_| {},
            )
            .unwrap_err()
            .code(),
            "image_facts_mismatch"
        );
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    fn pinned_file_identity_requires_device_inode_and_length_to_remain_equal() {
        use std::sync::atomic::{AtomicU64, Ordering};

        static NEXT: AtomicU64 = AtomicU64::new(0);
        let serial = NEXT.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "open-rebellion-e24-file-identity-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir_all(&root).unwrap();
        let first = root.join("first.bin");
        let second = root.join("second.bin");
        fs::write(&first, b"same").unwrap();
        fs::write(&second, b"same").unwrap();

        let before = fs::metadata(&first).unwrap();
        let same = File::open(&first).unwrap().metadata().unwrap();
        let other_inode = File::open(&second).unwrap().metadata().unwrap();
        assert!(same_pinned_file_identity(&before, &same));
        assert!(!same_pinned_file_identity(&before, &other_inode));

        fs::write(&first, b"different length").unwrap();
        let changed_length = File::open(&first).unwrap().metadata().unwrap();
        assert!(!same_pinned_file_identity(&before, &changed_length));
        fs::remove_dir_all(root).unwrap();
    }
}
