use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use rebellion_data::encyclopedia::{
    parse_catalog, parse_manifest, validate_bundle, AssetFacts, EncyclopediaCatalog,
    EncyclopediaError, EncyclopediaManifest, ImageFacts,
};
use rebellion_render::{inspect_encyclopedia_bytes, InspectedBytes};

/// Exact immutable byte buffers supplied by a packed or loose encyclopedia loader.
pub type EncyclopediaBytes = BTreeMap<String, Arc<[u8]>>;

/// Single ownership limit for all retained bundle paths and unique byte buffers.
pub const MAX_ENCYCLOPEDIA_RETAINED_BYTES: u64 = 512 * 1024 * 1024;

const MAX_EFFECTIVE_IMAGE_BYTES: u64 = 134_217_728;
const CATALOG_PATH: &str = "catalog.json";
const MANIFEST_PATH: &str = "manifest.json";

/// A complete validated candidate. Constructing this value has no global side effects.
#[derive(Debug, Clone)]
pub struct EncyclopediaSession {
    base_catalog: EncyclopediaCatalog,
    base_manifest: EncyclopediaManifest,
    base_bytes: Arc<EncyclopediaBytes>,
    effective_catalog: EncyclopediaCatalog,
    effective_bytes: Arc<EncyclopediaBytes>,
    observed_facts: BTreeMap<String, AssetFacts>,
    generation: u64,
}

impl EncyclopediaSession {
    #[must_use]
    pub const fn base_catalog(&self) -> &EncyclopediaCatalog {
        &self.base_catalog
    }

    #[must_use]
    pub const fn base_manifest(&self) -> &EncyclopediaManifest {
        &self.base_manifest
    }

    #[must_use]
    pub fn base_bytes(&self) -> &EncyclopediaBytes {
        self.base_bytes.as_ref()
    }

    #[must_use]
    pub const fn effective_catalog(&self) -> &EncyclopediaCatalog {
        &self.effective_catalog
    }

    #[must_use]
    pub fn effective_bytes(&self) -> &EncyclopediaBytes {
        self.effective_bytes.as_ref()
    }

    #[must_use]
    pub const fn observed_facts(&self) -> &BTreeMap<String, AssetFacts> {
        &self.observed_facts
    }

    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }
}

/// The app-level publication state. Integration installs `Ready` only after preparation.
#[derive(Debug, Clone)]
#[allow(
    clippy::large_enum_variant,
    reason = "the reviewed app contract stores the complete session directly in Ready"
)]
pub enum EncyclopediaAvailability {
    Unavailable(String),
    Ready(EncyclopediaSession),
}

/// Prepares a platform-neutral session from already retained bytes.
///
/// This function performs no file, network, GPU, world, or global-cache mutation.
pub fn prepare_encyclopedia_session(
    bytes: EncyclopediaBytes,
    base_dats: &BTreeMap<String, String>,
) -> Result<EncyclopediaSession, EncyclopediaError> {
    checked_retained_bytes(&bytes, MAX_ENCYCLOPEDIA_RETAINED_BYTES)?;

    let catalog_bytes = required_member(&bytes, CATALOG_PATH)?;
    let manifest_bytes = required_member(&bytes, MANIFEST_PATH)?;
    let catalog_facts = inspect_retained(CATALOG_PATH, catalog_bytes, None)?;
    let manifest_facts = inspect_retained(MANIFEST_PATH, manifest_bytes, None)?;
    let catalog = parse_catalog(catalog_bytes)?;
    let manifest = parse_manifest(manifest_bytes)?;

    validate_supplied_paths(&bytes, &manifest)?;
    validate_image_aggregate_before_decode(&bytes, &catalog)?;

    let mut observed_facts = BTreeMap::new();
    observed_facts.insert(CATALOG_PATH.to_owned(), catalog_facts);
    observed_facts.insert(MANIFEST_PATH.to_owned(), manifest_facts);

    // BTreeMap iteration is deterministic. Each decoded buffer is released by E12
    // before inspection returns, so validation holds at most one temporary image.
    for descriptor in catalog.images.values() {
        let retained = required_member(&bytes, &descriptor.path)?;
        let facts = inspect_retained(&descriptor.path, retained, Some(&descriptor.format))?;
        observed_facts.insert(descriptor.path.clone(), facts);
    }

    let bundle_facts = observed_facts
        .iter()
        .filter(|(path, _)| path.as_str() != MANIFEST_PATH)
        .map(|(path, facts)| (path.clone(), facts.clone()))
        .collect();
    validate_bundle(&catalog, &manifest, &bundle_facts, base_dats)?;

    let retained_bytes = Arc::new(bytes);
    Ok(EncyclopediaSession {
        effective_catalog: catalog.clone(),
        effective_bytes: Arc::clone(&retained_bytes),
        base_catalog: catalog,
        base_manifest: manifest,
        base_bytes: retained_bytes,
        observed_facts,
        generation: 1,
    })
}

fn required_member<'a>(
    bytes: &'a EncyclopediaBytes,
    path: &str,
) -> Result<&'a [u8], EncyclopediaError> {
    bytes
        .get(path)
        .map(AsRef::as_ref)
        .ok_or_else(|| session_error("missing_runtime_file", path, "required bytes are absent"))
}

fn validate_supplied_paths(
    bytes: &EncyclopediaBytes,
    manifest: &EncyclopediaManifest,
) -> Result<(), EncyclopediaError> {
    let mut expected: BTreeSet<&str> = manifest.files.keys().map(String::as_str).collect();
    expected.insert(CATALOG_PATH);
    expected.insert(MANIFEST_PATH);
    let supplied: BTreeSet<&str> = bytes.keys().map(String::as_str).collect();

    if let Some(path) = expected.difference(&supplied).next() {
        return Err(session_error(
            "missing_runtime_file",
            *path,
            "manifest-required bytes are absent",
        ));
    }
    if let Some(path) = supplied.difference(&expected).next() {
        return Err(session_error(
            "unexpected_runtime_file",
            *path,
            "supplied bytes are outside the immutable runtime allowlist",
        ));
    }
    Ok(())
}

fn validate_image_aggregate_before_decode(
    bytes: &EncyclopediaBytes,
    catalog: &EncyclopediaCatalog,
) -> Result<(), EncyclopediaError> {
    let mut aggregate = 0_u64;
    for descriptor in catalog.images.values() {
        let retained = required_member(bytes, &descriptor.path)?;
        let byte_len = u64::try_from(retained.len()).map_err(|_| {
            session_error(
                "resource_limit:effective_image_bytes",
                "$.images",
                "image byte length does not fit u64",
            )
        })?;
        aggregate = aggregate.checked_add(byte_len).ok_or_else(|| {
            session_error(
                "resource_limit:effective_image_bytes",
                "$.images",
                "effective image byte total overflowed",
            )
        })?;
        if aggregate > MAX_EFFECTIVE_IMAGE_BYTES {
            return Err(session_error(
                "resource_limit:effective_image_bytes",
                "$.images",
                "effective image bytes exceed the 128 MiB logical asset-set limit",
            ));
        }
    }
    Ok(())
}

fn inspect_retained(
    path: &str,
    bytes: &[u8],
    format: Option<&str>,
) -> Result<AssetFacts, EncyclopediaError> {
    let inspected = inspect_encyclopedia_bytes(bytes, format).map_err(|detail| {
        session_error(
            if format.is_some() {
                "invalid_image"
            } else {
                "invalid_runtime_file"
            },
            path,
            detail,
        )
    })?;
    inspected_to_asset_facts(path, inspected)
}

fn inspected_to_asset_facts(
    path: &str,
    inspected: InspectedBytes,
) -> Result<AssetFacts, EncyclopediaError> {
    let image = match (inspected.format, inspected.width, inspected.height) {
        (None, None, None) => None,
        (Some(format), Some(width), Some(height)) => Some(ImageFacts {
            format,
            width,
            height,
        }),
        _ => {
            return Err(session_error(
                "invalid_image",
                path,
                "byte inspector returned incomplete image facts",
            ));
        }
    };
    Ok(AssetFacts {
        sha256: inspected.sha256,
        byte_len: inspected.byte_len,
        image,
    })
}

fn checked_retained_bytes(bytes: &EncyclopediaBytes, limit: u64) -> Result<u64, EncyclopediaError> {
    let mut total = 0_u64;
    let mut unique_storage = BTreeSet::new();
    for (path, retained) in bytes {
        let path_len = u64::try_from(path.len()).map_err(|_| {
            session_error(
                "resource_limit:retained_bytes",
                "$",
                "runtime path length does not fit u64",
            )
        })?;
        total = checked_retained_add(total, path_len, limit)?;

        let identity = (retained.as_ptr() as usize, retained.len());
        if unique_storage.insert(identity) {
            let byte_len = u64::try_from(retained.len()).map_err(|_| {
                session_error(
                    "resource_limit:retained_bytes",
                    "$",
                    "retained byte length does not fit u64",
                )
            })?;
            total = checked_retained_add(total, byte_len, limit)?;
        }
    }
    Ok(total)
}

fn checked_retained_add(total: u64, addition: u64, limit: u64) -> Result<u64, EncyclopediaError> {
    let next = total.checked_add(addition).ok_or_else(|| {
        session_error(
            "resource_limit:retained_bytes",
            "$",
            "retained-byte accounting overflowed",
        )
    })?;
    if next > limit {
        return Err(session_error(
            "resource_limit:retained_bytes",
            "$",
            "retained candidate exceeds the global byte cap",
        ));
    }
    Ok(next)
}

fn session_error(
    code: &'static str,
    path: impl Into<String>,
    detail: impl Into<String>,
) -> EncyclopediaError {
    EncyclopediaError::for_session(code, path, detail)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::sync::Arc;

    use rebellion_render::encyclopedia_assets::MAX_ENCYCLOPEDIA_IMAGE_BYTES;
    use rebellion_render::inspect_encyclopedia_bytes;
    use serde_json::Value;

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

    fn retained(bytes: &[u8]) -> Arc<[u8]> {
        Arc::from(bytes)
    }

    fn valid_bytes() -> EncyclopediaBytes {
        BTreeMap::from([
            ("catalog.json".to_owned(), retained(VALID_CATALOG)),
            ("manifest.json".to_owned(), retained(VALID_MANIFEST)),
            ("assets/EDATA.001".to_owned(), retained(VALID_IMAGE_1)),
            ("assets/EDATA.002".to_owned(), retained(VALID_IMAGE_2)),
            ("assets/EDATA.003".to_owned(), retained(VALID_IMAGE_3)),
        ])
    }

    fn valid_dats() -> BTreeMap<String, String> {
        BTreeMap::from([(
            "SYNTHETIC.DAT".to_owned(),
            "39fb2c329d34fbdd94bb2a2a596b694597eb00b8b402905ec4920f560210d322".to_owned(),
        )])
    }

    #[test]
    fn a_valid_bundle_prepares_distinct_base_and_effective_snapshots_from_exact_arcs() {
        let bytes = valid_bytes();
        let original = bytes.clone();

        let session = prepare_encyclopedia_session(bytes, &valid_dats()).unwrap();

        assert_eq!(session.generation(), 1);
        assert_eq!(session.base_catalog(), session.effective_catalog());
        assert_eq!(session.base_bytes().len(), original.len());
        assert_eq!(session.effective_bytes().len(), original.len());
        assert!(Arc::ptr_eq(&session.base_bytes, &session.effective_bytes));
        for (path, retained) in &original {
            assert!(Arc::ptr_eq(retained, &session.base_bytes()[path]));
            assert!(Arc::ptr_eq(retained, &session.effective_bytes()[path]));
        }
        assert_eq!(session.observed_facts().len(), original.len());
        assert_eq!(session.observed_facts()["manifest.json"].image, None);
        assert_eq!(session.base_manifest().schema_version, 1);
        assert!(matches!(
            EncyclopediaAvailability::Ready(session),
            EncyclopediaAvailability::Ready(_)
        ));
    }

    #[test]
    fn a_structurally_valid_catalog_above_32_mib_and_within_64_mib_prepares() {
        const LARGE_CATALOG_SHA256: &str =
            "1ca94ca9fca7b69406633c3a5b15ad6df7000aa598dbf8065b678b860d221819";
        let target_len = MAX_ENCYCLOPEDIA_IMAGE_BYTES + 1;
        let mut catalog = VALID_CATALOG.to_vec();
        catalog.resize(target_len, b' ');
        assert_eq!(catalog.len(), target_len);

        let mut manifest: Value = serde_json::from_slice(VALID_MANIFEST).unwrap();
        manifest["catalog_sha256"] = Value::from(LARGE_CATALOG_SHA256);
        manifest["files"]["catalog.json"] = Value::from(LARGE_CATALOG_SHA256);

        let mut bytes = valid_bytes();
        bytes.insert("catalog.json".to_owned(), Arc::from(catalog));
        bytes.insert(
            "manifest.json".to_owned(),
            Arc::from(serde_json::to_vec(&manifest).unwrap()),
        );

        let session = prepare_encyclopedia_session(bytes, &valid_dats())
            .expect("the approved 64 MiB catalog bound must not inherit the image cap");

        assert_eq!(session.base_bytes()["catalog.json"].len(), target_len);
        assert_eq!(
            session.observed_facts()["catalog.json"].sha256,
            LARGE_CATALOG_SHA256
        );
    }

    #[test]
    fn a_partial_bundle_fails_contextually_and_releases_the_candidate() {
        let mut bytes = valid_bytes();
        bytes.remove("assets/EDATA.003");
        let catalog = retained(VALID_CATALOG);
        let weak = Arc::downgrade(&catalog);
        bytes.insert("catalog.json".to_owned(), catalog);

        let error = prepare_encyclopedia_session(bytes, &valid_dats()).unwrap_err();

        assert_eq!(error.code(), "missing_runtime_file");
        assert_eq!(error.source(), "session");
        assert_eq!(error.path(), "assets/EDATA.003");
        assert!(
            weak.upgrade().is_none(),
            "failed candidates must be released"
        );
    }

    #[test]
    fn corrupt_image_bytes_fail_before_a_session_or_global_state_exists() {
        let mut bytes = valid_bytes();
        let mut corrupt = VALID_IMAGE_1.to_vec();
        corrupt[0] = b'X';
        let corrupt: Arc<[u8]> = corrupt.into();
        let weak = Arc::downgrade(&corrupt);
        bytes.insert("assets/EDATA.001".to_owned(), corrupt);

        let error = prepare_encyclopedia_session(bytes, &valid_dats()).unwrap_err();

        assert_eq!(error.code(), "invalid_image");
        assert_eq!(error.path(), "assets/EDATA.001");
        assert!(weak.upgrade().is_none());

        let valid = prepare_encyclopedia_session(valid_bytes(), &valid_dats()).unwrap();
        assert_eq!(
            valid.generation(),
            1,
            "a failed candidate installs no state"
        );
    }

    #[test]
    fn selected_dat_and_provider_mismatches_remain_distinct_failures() {
        let wrong_dats = BTreeMap::from([(
            "SYNTHETIC.DAT".to_owned(),
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
        )]);
        let error = prepare_encyclopedia_session(valid_bytes(), &wrong_dats).unwrap_err();
        assert_eq!(error.code(), "binding_source_mismatch");

        let mut swapped = valid_bytes();
        let first = swapped["assets/EDATA.001"].clone();
        let second = swapped["assets/EDATA.002"].clone();
        swapped.insert("assets/EDATA.001".to_owned(), second);
        swapped.insert("assets/EDATA.002".to_owned(), first);
        let error = prepare_encyclopedia_session(swapped, &valid_dats()).unwrap_err();
        assert_eq!(error.code(), "file_digest_mismatch");
    }

    #[test]
    fn logical_image_aggregate_is_rejected_before_any_image_decode() {
        let bytes = aggregate_overflow_bytes();

        let error = prepare_encyclopedia_session(bytes, &valid_dats()).unwrap_err();

        assert_eq!(error.code(), "resource_limit:effective_image_bytes");
        assert_eq!(error.path(), "$.images");
    }

    #[test]
    fn logical_image_aggregate_accepts_the_exact_boundary_and_rejects_one_more_member() {
        let bytes = aggregate_overflow_bytes();
        let catalog = parse_catalog(&bytes["catalog.json"]).unwrap();
        let mut exact = catalog.clone();
        exact.images.remove("edata:5");

        validate_image_aggregate_before_decode(&bytes, &exact).unwrap();
        let error = validate_image_aggregate_before_decode(&bytes, &catalog).unwrap_err();
        assert_eq!(error.code(), "resource_limit:effective_image_bytes");
    }

    #[test]
    fn an_extra_supplied_member_is_rejected_instead_of_becoming_session_state() {
        let mut bytes = valid_bytes();
        bytes.insert("assets/UNOWNED.001".to_owned(), Arc::from([1_u8]));

        let error = prepare_encyclopedia_session(bytes, &valid_dats()).unwrap_err();

        assert_eq!(error.code(), "unexpected_runtime_file");
        assert_eq!(error.path(), "assets/UNOWNED.001");
    }

    #[test]
    fn generation_accessor_reports_the_snapshot_generation() {
        let mut session = prepare_encyclopedia_session(valid_bytes(), &valid_dats()).unwrap();
        session.generation = 2;

        assert_eq!(session.generation(), 2);
    }

    #[test]
    fn retained_budget_counts_shared_arc_storage_once_and_checks_before_admission() {
        let shared: Arc<[u8]> = vec![1, 2, 3].into();
        let bytes = BTreeMap::from([("a".to_owned(), shared.clone()), ("b".to_owned(), shared)]);
        assert_eq!(checked_retained_bytes(&bytes, 5).unwrap(), 5);

        let error = checked_retained_bytes(&bytes, 4).unwrap_err();
        assert_eq!(error.code(), "resource_limit:retained_bytes");
        assert_eq!(error.path(), "$");
    }

    fn aggregate_overflow_bytes() -> EncyclopediaBytes {
        let shared: Arc<[u8]> = vec![0; MAX_ENCYCLOPEDIA_IMAGE_BYTES].into();
        let shared_digest = inspect_encyclopedia_bytes(&shared, None).unwrap().sha256;

        let mut catalog: Value = serde_json::from_slice(VALID_CATALOG).unwrap();
        let images = catalog["images"].as_object_mut().unwrap();
        let mut template = images["edata:1"].clone();
        for descriptor in images.values_mut() {
            descriptor["byte_length"] = Value::from(MAX_ENCYCLOPEDIA_IMAGE_BYTES as u64);
            descriptor["sha256"] = Value::from(shared_digest.clone());
        }
        for number in 4..=5 {
            template["path"] = Value::from(format!("assets/EDATA.{number:03}"));
            template["byte_length"] = Value::from(MAX_ENCYCLOPEDIA_IMAGE_BYTES as u64);
            template["sha256"] = Value::from(shared_digest.clone());
            images.insert(format!("edata:{number}"), template.clone());
        }
        let localized = catalog["topics"]["original:60001"]["localized"]
            .as_object_mut()
            .unwrap();
        for (language, number) in [("1034", 4), ("1035", 5)] {
            let mut record = localized["1033"].clone();
            record["image_id"] = Value::from(format!("edata:{number}"));
            localized.insert(language.to_owned(), record);
        }

        let catalog_bytes = serde_json::to_vec(&catalog).unwrap();
        let catalog_digest = inspect_encyclopedia_bytes(&catalog_bytes, None)
            .unwrap()
            .sha256;
        let mut manifest: Value = serde_json::from_slice(VALID_MANIFEST).unwrap();
        manifest["catalog_sha256"] = Value::from(catalog_digest.clone());
        let files = manifest["files"].as_object_mut().unwrap();
        files.clear();
        files.insert("catalog.json".to_owned(), Value::from(catalog_digest));
        for number in 1..=5 {
            files.insert(
                format!("assets/EDATA.{number:03}"),
                Value::from(shared_digest.clone()),
            );
        }
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();

        let mut bytes = BTreeMap::from([
            ("catalog.json".to_owned(), Arc::from(catalog_bytes)),
            ("manifest.json".to_owned(), Arc::from(manifest_bytes)),
        ]);
        for number in 1..=5 {
            bytes.insert(format!("assets/EDATA.{number:03}"), shared.clone());
        }
        bytes
    }
}
