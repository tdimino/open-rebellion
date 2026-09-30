use std::collections::BTreeMap;
use std::path::Path;

use rebellion_render::TopicImageView;

#[cfg(not(target_arch = "wasm32"))]
use rebellion_render::{
    approved_hd_assets_from_bytes, inspect_encyclopedia_bytes, AssetRenderProfile,
    TopicImageRenderProfile, MAX_ENCYCLOPEDIA_IMAGE_BYTES,
};
#[cfg(not(target_arch = "wasm32"))]
use std::collections::BTreeSet;
#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::{Read, Take};
#[cfg(not(target_arch = "wasm32"))]
use std::path::Component;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
const MAX_HD_MANIFEST_BYTES: usize = 8 * 1024 * 1024;

/// Explicit provenance for an image selected by the validated effective
/// catalog. No source scope, approval identity, or output path is inferred
/// from an asset ID or catalog filename.
pub enum EncyclopediaImageCandidate<'a> {
    Base {
        topic_id: &'a str,
        image: &'a TopicImageView,
        approval_key: &'a str,
        output_relative_path: &'a Path,
    },
    Mod {
        topic_id: &'a str,
        image: &'a TopicImageView,
    },
    Null {
        topic_id: &'a str,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaHdDiagnostic {
    pub code: &'static str,
    pub asset_id: Option<String>,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BaseImageKey {
    asset_id: String,
    source_digest: String,
}

impl BaseImageKey {
    fn from_image(image: &TopicImageView) -> Self {
        Self {
            asset_id: image.asset_id.clone(),
            source_digest: image.digest.clone(),
        }
    }
}

/// Image replacements prepared at a session/profile boundary. Selection is a
/// pure map lookup over exact base identity plus digest, so navigation never
/// reads files, hashes bytes, or decodes images.
#[derive(Debug, Clone, Default)]
pub struct PreparedEncyclopediaImages {
    selections: BTreeMap<String, PreparedImageSelection>,
    diagnostics: Vec<EncyclopediaHdDiagnostic>,
}

#[derive(Debug, Clone)]
struct PreparedImageSelection {
    expected: Option<BaseImageKey>,
    selected: Option<TopicImageView>,
}

impl PreparedEncyclopediaImages {
    #[must_use]
    pub fn original_only() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn diagnostics(&self) -> &[EncyclopediaHdDiagnostic] {
        &self.diagnostics
    }

    /// Apply a prepared base/mod/null decision to an already validated image.
    /// Exact topic, asset identity, and digest matching prevents stale profile
    /// state from replacing a later effective selection.
    #[must_use]
    pub fn select(
        &self,
        topic_id: &str,
        selected: Option<&TopicImageView>,
    ) -> Option<TopicImageView> {
        let Some(prepared) = self.selections.get(topic_id) else {
            return selected.cloned();
        };
        let observed = selected.map(BaseImageKey::from_image);
        if observed == prepared.expected {
            prepared.selected.clone()
        } else {
            // A caller installed preparation for a different effective
            // selection. Preserve the currently validated bytes rather than
            // applying stale base/mod/null provenance.
            selected.cloned()
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn prepare_native_encyclopedia_hd(
    profile: AssetRenderProfile,
    hd_root: Option<&Path>,
    candidates: &[EncyclopediaImageCandidate<'_>],
) -> PreparedEncyclopediaImages {
    let mut prepared = PreparedEncyclopediaImages::default();

    // Preparation is an atomic snapshot with exactly one effective source
    // decision per topic. Reject the whole batch before any HD filesystem I/O
    // when that contract is violated; otherwise iteration order could silently
    // choose Base over Mod (or resurrect art after Null).
    let mut seen_topics = BTreeSet::new();
    let mut duplicate_topics = BTreeSet::new();
    for candidate in candidates {
        let topic_id = match candidate {
            EncyclopediaImageCandidate::Base { topic_id, .. }
            | EncyclopediaImageCandidate::Mod { topic_id, .. }
            | EncyclopediaImageCandidate::Null { topic_id } => *topic_id,
        };
        if !seen_topics.insert(topic_id) {
            duplicate_topics.insert(topic_id);
        }
    }
    if !duplicate_topics.is_empty() {
        prepared
            .diagnostics
            .extend(duplicate_topics.into_iter().map(|topic_id| {
                global_diagnostic(
                    "duplicate_hd_topic_candidate",
                    format!(
                        "topic {topic_id:?} has more than one image candidate; HD preparation requires exactly one effective candidate per topic"
                    ),
                )
            }));
        return prepared;
    }

    for candidate in candidates {
        let (topic_id, expected, selected) = match candidate {
            EncyclopediaImageCandidate::Base {
                topic_id, image, ..
            }
            | EncyclopediaImageCandidate::Mod { topic_id, image } => (
                *topic_id,
                Some(BaseImageKey::from_image(image)),
                Some((*image).clone()),
            ),
            EncyclopediaImageCandidate::Null { topic_id } => (*topic_id, None, None),
        };
        prepared.selections.insert(
            topic_id.to_owned(),
            PreparedImageSelection { expected, selected },
        );
    }

    if profile == AssetRenderProfile::OriginalParity || candidates.is_empty() {
        return prepared;
    }

    let has_base_candidates = candidates
        .iter()
        .any(|candidate| matches!(candidate, EncyclopediaImageCandidate::Base { .. }));
    if !has_base_candidates {
        return prepared;
    }

    let Some(hd_root) = hd_root else {
        prepared.diagnostics.push(global_diagnostic(
            "hd_root_unavailable",
            "faithful-HD was requested without a configured HD root",
        ));
        return prepared;
    };
    let manifest_bytes =
        match read_bounded_file(&hd_root.join("manifest.json"), MAX_HD_MANIFEST_BYTES) {
            Ok(bytes) => bytes,
            Err(BoundedReadError::Unavailable(detail)) => {
                prepared
                    .diagnostics
                    .push(global_diagnostic("hd_manifest_unavailable", detail));
                return prepared;
            }
            Err(BoundedReadError::ResourceLimit(detail)) => {
                prepared
                    .diagnostics
                    .push(global_diagnostic("hd_manifest_resource_limit", detail));
                return prepared;
            }
        };
    let approvals = match approved_hd_assets_from_bytes(&manifest_bytes) {
        Ok(approvals) => approvals,
        Err(detail) => {
            prepared
                .diagnostics
                .push(global_diagnostic("hd_manifest_invalid", detail));
            return prepared;
        }
    };

    for candidate in candidates {
        let EncyclopediaImageCandidate::Base {
            topic_id,
            image,
            approval_key,
            output_relative_path,
        } = candidate
        else {
            continue;
        };
        let asset_id = image.asset_id.clone();
        let Some(approval) = approvals.get(*approval_key) else {
            prepared.diagnostics.push(asset_diagnostic(
                "hd_approval_missing",
                &asset_id,
                format!("no approved faithful-HD record exists for {approval_key:?}"),
            ));
            continue;
        };
        if let Err(detail) = approval.validate_source_bytes(&image.bytes) {
            prepared
                .diagnostics
                .push(asset_diagnostic("hd_source_mismatch", &asset_id, detail));
            continue;
        }
        if !safe_relative_path(output_relative_path) {
            prepared.diagnostics.push(asset_diagnostic(
                "hd_output_path_invalid",
                &asset_id,
                format!(
                    "HD output path {:?} is not a confined relative path",
                    output_relative_path
                ),
            ));
            continue;
        }

        let output_path = hd_root.join(output_relative_path);
        let output_bytes = match read_bounded_file(&output_path, MAX_ENCYCLOPEDIA_IMAGE_BYTES) {
            Ok(bytes) => bytes,
            Err(BoundedReadError::Unavailable(detail)) => {
                prepared.diagnostics.push(asset_diagnostic(
                    "hd_output_unavailable",
                    &asset_id,
                    detail,
                ));
                continue;
            }
            Err(BoundedReadError::ResourceLimit(detail)) => {
                prepared.diagnostics.push(asset_diagnostic(
                    "hd_output_resource_limit",
                    &asset_id,
                    detail,
                ));
                continue;
            }
        };
        if let Err(detail) = approval.validate_output_bytes(&output_bytes) {
            prepared
                .diagnostics
                .push(asset_diagnostic("hd_output_mismatch", &asset_id, detail));
            continue;
        }
        let inspected = match inspect_encyclopedia_bytes(&output_bytes, Some("png")) {
            Ok(inspected) => inspected,
            Err(detail) => {
                prepared
                    .diagnostics
                    .push(asset_diagnostic("hd_output_invalid", &asset_id, detail));
                continue;
            }
        };
        let Some(format) = inspected.format else {
            unreachable!("PNG inspection always returns an image format")
        };
        let (Some(width), Some(height)) = (inspected.width, inspected.height) else {
            unreachable!("PNG inspection always returns dimensions")
        };
        prepared.selections.insert(
            (*topic_id).to_owned(),
            PreparedImageSelection {
                expected: Some(BaseImageKey::from_image(image)),
                selected: Some(TopicImageView {
                    asset_id,
                    digest: inspected.sha256,
                    format,
                    width,
                    height,
                    bytes: Arc::from(output_bytes.into_boxed_slice()),
                    render_profile: TopicImageRenderProfile::FaithfulHdLinear,
                }),
            },
        );
    }

    prepared
}

#[cfg(not(target_arch = "wasm32"))]
enum BoundedReadError {
    Unavailable(String),
    ResourceLimit(String),
}

#[cfg(not(target_arch = "wasm32"))]
fn read_bounded_file(path: &Path, max_bytes: usize) -> Result<Vec<u8>, BoundedReadError> {
    let file = File::open(path).map_err(|error| {
        BoundedReadError::Unavailable(format!("cannot open {}: {error}", path.display()))
    })?;
    let metadata = file.metadata().map_err(|error| {
        BoundedReadError::Unavailable(format!("cannot inspect {}: {error}", path.display()))
    })?;
    if !metadata.is_file() {
        return Err(BoundedReadError::Unavailable(format!(
            "{} is not a regular file",
            path.display()
        )));
    }
    let max_bytes_u64 = u64::try_from(max_bytes).map_err(|_| {
        BoundedReadError::ResourceLimit("configured byte limit does not fit u64".to_owned())
    })?;
    if metadata.len() > max_bytes_u64 {
        return Err(BoundedReadError::ResourceLimit(format!(
            "{} exceeds the {max_bytes}-byte read limit",
            path.display()
        )));
    }

    let capacity = usize::try_from(metadata.len())
        .unwrap_or(max_bytes)
        .min(max_bytes);
    let mut bytes = Vec::with_capacity(capacity);
    let take_limit = max_bytes_u64.checked_add(1).ok_or_else(|| {
        BoundedReadError::ResourceLimit("configured read limit overflowed".to_owned())
    })?;
    let mut bounded: Take<File> = file.take(take_limit);
    bounded.read_to_end(&mut bytes).map_err(|error| {
        BoundedReadError::Unavailable(format!("cannot read {}: {error}", path.display()))
    })?;
    if bytes.len() > max_bytes {
        return Err(BoundedReadError::ResourceLimit(format!(
            "{} exceeded the {max_bytes}-byte read limit while reading",
            path.display()
        )));
    }
    Ok(bytes)
}

#[cfg(not(target_arch = "wasm32"))]
fn safe_relative_path(path: &Path) -> bool {
    !path.as_os_str().is_empty()
        && !path.to_string_lossy().contains('\\')
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

#[cfg(not(target_arch = "wasm32"))]
fn global_diagnostic(code: &'static str, detail: impl Into<String>) -> EncyclopediaHdDiagnostic {
    EncyclopediaHdDiagnostic {
        code,
        asset_id: None,
        detail: detail.into(),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn asset_diagnostic(
    code: &'static str,
    asset_id: &str,
    detail: impl Into<String>,
) -> EncyclopediaHdDiagnostic {
    EncyclopediaHdDiagnostic {
        code,
        asset_id: Some(asset_id.to_owned()),
        detail: detail.into(),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;
    use std::time::{SystemTime, UNIX_EPOCH};

    use rebellion_render::{
        inspect_encyclopedia_bytes, AssetRenderProfile, TopicImageRenderProfile, TopicImageView,
        MAX_ENCYCLOPEDIA_IMAGE_BYTES,
    };
    use serde_json::json;

    use super::{
        prepare_native_encyclopedia_hd, EncyclopediaHdDiagnostic, EncyclopediaImageCandidate,
        MAX_HD_MANIFEST_BYTES,
    };

    const ORIGINAL_BMP: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.001"
    );
    const HD_PNG: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/images/mod-valid.png");
    const APPROVAL_KEY: &str = "edata/EDATA_001";
    const OUTPUT_PATH: &str = "EData/EDATA_001.png";
    const TOPIC_ID: &str = "original:60001";

    struct TestRoot(PathBuf);

    impl TestRoot {
        fn new(name: &str) -> Self {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("test clock must follow the Unix epoch")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "open-rebellion-e52-{name}-{}-{nonce}",
                std::process::id()
            ));
            fs::create_dir_all(&path).expect("synthetic HD root must be created");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }

        fn write_output(&self, bytes: &[u8]) {
            let output = self.0.join(OUTPUT_PATH);
            fs::create_dir_all(output.parent().expect("output has a parent")).unwrap();
            fs::write(output, bytes).unwrap();
        }

        fn write_manifest(&self, source_digest: &str, output_digest: &str) {
            let manifest = json!({
                "schema_version": 1,
                "profile": "faithful-hd",
                "assets": {
                    APPROVAL_KEY: {
                        "approved": true,
                        "review": {"reviewer": "synthetic-reviewer", "evidence": "fixture"},
                        "gates": {"human_review": "pass"},
                        "source": {"sha256": source_digest},
                        "output": {"sha256": output_digest}
                    }
                }
            });
            fs::write(
                self.0.join("manifest.json"),
                serde_json::to_vec(&manifest).unwrap(),
            )
            .unwrap();
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn image(asset_id: &str, bytes: &[u8], format: &str) -> TopicImageView {
        let observed = inspect_encyclopedia_bytes(bytes, Some(format)).unwrap();
        TopicImageView {
            asset_id: asset_id.to_owned(),
            digest: observed.sha256,
            format: observed.format.unwrap(),
            width: observed.width.unwrap(),
            height: observed.height.unwrap(),
            bytes: Arc::from(bytes),
            render_profile: TopicImageRenderProfile::OriginalNearest,
        }
    }

    fn digest(bytes: &[u8], format: &str) -> String {
        inspect_encyclopedia_bytes(bytes, Some(format))
            .unwrap()
            .sha256
    }

    fn raw_digest(bytes: &[u8]) -> String {
        inspect_encyclopedia_bytes(bytes, None).unwrap().sha256
    }

    fn candidate(image: &TopicImageView) -> EncyclopediaImageCandidate<'_> {
        EncyclopediaImageCandidate::Base {
            topic_id: TOPIC_ID,
            image,
            approval_key: APPROVAL_KEY,
            output_relative_path: Path::new(OUTPUT_PATH),
        }
    }

    fn assert_diagnostic(
        diagnostics: &[EncyclopediaHdDiagnostic],
        code: &'static str,
        asset_id: Option<&str>,
    ) {
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.code == code && diagnostic.asset_id.as_deref() == asset_id
        }));
    }

    #[test]
    fn approved_staged_source_selects_fully_inspected_hd_bytes_without_mutating_original() {
        let root = TestRoot::new("approved");
        let original = image("edata:1", ORIGINAL_BMP, "bmp");
        let original_bytes = Arc::clone(&original.bytes);
        root.write_output(HD_PNG);
        root.write_manifest(&original.digest, &digest(HD_PNG, "png"));

        let prepared = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            Some(root.path()),
            &[candidate(&original)],
        );
        let selected = prepared.select(TOPIC_ID, Some(&original)).unwrap();

        assert!(prepared.diagnostics().is_empty());
        assert_eq!(selected.asset_id, "edata:1");
        assert_eq!(selected.digest, digest(HD_PNG, "png"));
        assert_eq!(selected.format, "png");
        assert_eq!((selected.width, selected.height), (1, 1));
        assert_eq!(
            selected.render_profile,
            TopicImageRenderProfile::FaithfulHdLinear
        );
        assert_eq!(selected.bytes.as_ref(), HD_PNG);
        assert!(Arc::ptr_eq(&original.bytes, &original_bytes));
        assert_eq!(original.bytes.as_ref(), ORIGINAL_BMP);
    }

    #[test]
    fn missing_or_invalid_approval_falls_back_to_the_exact_original() {
        let original = image("edata:1", ORIGINAL_BMP, "bmp");
        let missing_root = TestRoot::new("missing-approval");
        missing_root.write_manifest(&original.digest, &digest(HD_PNG, "png"));
        missing_root.write_output(HD_PNG);
        let manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(missing_root.path().join("manifest.json")).unwrap())
                .unwrap();
        let mut without_asset = manifest;
        without_asset["assets"] = json!({});
        fs::write(
            missing_root.path().join("manifest.json"),
            serde_json::to_vec(&without_asset).unwrap(),
        )
        .unwrap();

        let missing = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            Some(missing_root.path()),
            &[candidate(&original)],
        );
        let missing_selected = missing.select(TOPIC_ID, Some(&original)).unwrap();
        assert!(Arc::ptr_eq(&missing_selected.bytes, &original.bytes));
        assert_diagnostic(
            missing.diagnostics(),
            "hd_approval_missing",
            Some("edata:1"),
        );

        let invalid_root = TestRoot::new("invalid-manifest");
        fs::write(invalid_root.path().join("manifest.json"), b"not json").unwrap();
        let invalid = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            Some(invalid_root.path()),
            &[candidate(&original)],
        );
        let invalid_selected = invalid.select(TOPIC_ID, Some(&original)).unwrap();
        assert!(Arc::ptr_eq(&invalid_selected.bytes, &original.bytes));
        assert_diagnostic(invalid.diagnostics(), "hd_manifest_invalid", None);
    }

    #[test]
    fn manifest_read_limit_accepts_the_exact_boundary_and_rejects_one_more_byte() {
        assert_eq!(MAX_HD_MANIFEST_BYTES, 8_388_608);
        let original = image("edata:1", ORIGINAL_BMP, "bmp");
        let boundary_root = TestRoot::new("manifest-boundary");
        let boundary_manifest =
            fs::File::create(boundary_root.path().join("manifest.json")).unwrap();
        boundary_manifest
            .set_len(u64::try_from(MAX_HD_MANIFEST_BYTES).unwrap())
            .unwrap();
        let boundary = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            Some(boundary_root.path()),
            &[candidate(&original)],
        );
        assert_diagnostic(boundary.diagnostics(), "hd_manifest_invalid", None);
        assert!(!boundary
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.code == "hd_manifest_resource_limit"));

        let oversized_root = TestRoot::new("manifest-oversized");
        let oversized_manifest =
            fs::File::create(oversized_root.path().join("manifest.json")).unwrap();
        oversized_manifest
            .set_len(u64::try_from(MAX_HD_MANIFEST_BYTES).unwrap() + 1)
            .unwrap();
        let oversized = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            Some(oversized_root.path()),
            &[candidate(&original)],
        );
        assert_diagnostic(oversized.diagnostics(), "hd_manifest_resource_limit", None);
    }

    #[test]
    fn source_and_output_digest_mismatches_are_diagnosed_and_fall_back() {
        let original = image("edata:1", ORIGINAL_BMP, "bmp");
        let root = TestRoot::new("digest-mismatch");
        root.write_output(HD_PNG);
        root.write_manifest(&"a".repeat(64), &digest(HD_PNG, "png"));

        let source_mismatch = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            Some(root.path()),
            &[candidate(&original)],
        );
        assert_diagnostic(
            source_mismatch.diagnostics(),
            "hd_source_mismatch",
            Some("edata:1"),
        );
        assert!(Arc::ptr_eq(
            &source_mismatch
                .select(TOPIC_ID, Some(&original))
                .unwrap()
                .bytes,
            &original.bytes
        ));

        root.write_manifest(&original.digest, &"b".repeat(64));
        let output_mismatch = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            Some(root.path()),
            &[candidate(&original)],
        );
        assert_diagnostic(
            output_mismatch.diagnostics(),
            "hd_output_mismatch",
            Some("edata:1"),
        );
        assert!(Arc::ptr_eq(
            &output_mismatch
                .select(TOPIC_ID, Some(&original))
                .unwrap()
                .bytes,
            &original.bytes
        ));
    }

    #[test]
    fn missing_corrupt_and_oversized_outputs_never_replace_the_original() {
        let original = image("edata:1", ORIGINAL_BMP, "bmp");

        let missing_root = TestRoot::new("missing-output");
        missing_root.write_manifest(&original.digest, &digest(HD_PNG, "png"));
        let missing = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            Some(missing_root.path()),
            &[candidate(&original)],
        );
        assert_diagnostic(
            missing.diagnostics(),
            "hd_output_unavailable",
            Some("edata:1"),
        );

        let corrupt_root = TestRoot::new("corrupt-output");
        corrupt_root.write_output(b"not a png");
        corrupt_root.write_manifest(&original.digest, &raw_digest(b"not a png"));
        let corrupt = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            Some(corrupt_root.path()),
            &[candidate(&original)],
        );
        assert_diagnostic(corrupt.diagnostics(), "hd_output_invalid", Some("edata:1"));

        let oversized_root = TestRoot::new("oversized-output");
        let oversized_path = oversized_root.path().join(OUTPUT_PATH);
        fs::create_dir_all(oversized_path.parent().unwrap()).unwrap();
        let oversized = fs::File::create(&oversized_path).unwrap();
        oversized
            .set_len(u64::try_from(MAX_ENCYCLOPEDIA_IMAGE_BYTES).unwrap() + 1)
            .unwrap();
        oversized_root.write_manifest(&original.digest, &"c".repeat(64));
        let oversized = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            Some(oversized_root.path()),
            &[candidate(&original)],
        );
        assert_diagnostic(
            oversized.diagnostics(),
            "hd_output_resource_limit",
            Some("edata:1"),
        );

        for prepared in [&missing, &corrupt, &oversized] {
            let selected = prepared.select(TOPIC_ID, Some(&original)).unwrap();
            assert!(Arc::ptr_eq(&selected.bytes, &original.bytes));
            assert_eq!(
                selected.render_profile,
                TopicImageRenderProfile::OriginalNearest
            );
        }
    }

    #[test]
    fn unsafe_output_paths_are_diagnosed_before_any_candidate_read() {
        let root = TestRoot::new("unsafe-paths");
        let original = image("edata:1", ORIGINAL_BMP, "bmp");
        root.write_manifest(&original.digest, &digest(HD_PNG, "png"));

        for output_relative_path in [
            Path::new(""),
            Path::new("../outside.png"),
            Path::new("nested\\outside.png"),
            Path::new("/absolute.png"),
        ] {
            let prepared = prepare_native_encyclopedia_hd(
                AssetRenderProfile::FaithfulHd,
                Some(root.path()),
                &[EncyclopediaImageCandidate::Base {
                    topic_id: TOPIC_ID,
                    image: &original,
                    approval_key: APPROVAL_KEY,
                    output_relative_path,
                }],
            );
            assert_diagnostic(
                prepared.diagnostics(),
                "hd_output_path_invalid",
                Some("edata:1"),
            );
            assert!(Arc::ptr_eq(
                &prepared.select(TOPIC_ID, Some(&original)).unwrap().bytes,
                &original.bytes
            ));
        }
    }

    #[test]
    fn profile_preparation_preserves_base_hd_mod_and_null_precedence_without_late_io() {
        let root = TestRoot::new("precedence");
        let original = image("edata:1", ORIGINAL_BMP, "bmp");
        let mod_image = image("mod:test:encyclopedia/replacement.png", HD_PNG, "png");
        root.write_output(HD_PNG);
        root.write_manifest(&original.digest, &digest(HD_PNG, "png"));

        let original_profile = prepare_native_encyclopedia_hd(
            AssetRenderProfile::OriginalParity,
            None,
            &[candidate(&original)],
        );
        assert!(original_profile.diagnostics().is_empty());
        assert!(Arc::ptr_eq(
            &original_profile
                .select(TOPIC_ID, Some(&original))
                .unwrap()
                .bytes,
            &original.bytes
        ));

        let hd_profile = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            Some(root.path()),
            &[candidate(&original)],
        );
        fs::remove_dir_all(root.path()).unwrap();

        let hd = hd_profile.select(TOPIC_ID, Some(&original)).unwrap();
        let repeated_hd = hd_profile.select(TOPIC_ID, Some(&original)).unwrap();
        assert_eq!(hd.render_profile, TopicImageRenderProfile::FaithfulHdLinear);
        assert!(Arc::ptr_eq(&hd.bytes, &repeated_hd.bytes));

        let mod_profile = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            None,
            &[EncyclopediaImageCandidate::Mod {
                topic_id: TOPIC_ID,
                image: &mod_image,
            }],
        );
        let selected_mod = mod_profile.select(TOPIC_ID, Some(&mod_image)).unwrap();
        assert!(Arc::ptr_eq(&selected_mod.bytes, &mod_image.bytes));
        assert_eq!(
            selected_mod.render_profile,
            TopicImageRenderProfile::OriginalNearest
        );
        let null_profile = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            None,
            &[EncyclopediaImageCandidate::Null { topic_id: TOPIC_ID }],
        );
        assert!(null_profile.select(TOPIC_ID, None).is_none());
    }

    #[test]
    fn duplicate_topic_candidates_fail_closed_before_hd_reads_in_every_source_order() {
        fn assert_rejected(
            candidates: &[EncyclopediaImageCandidate<'_>],
            current: Option<&TopicImageView>,
        ) {
            let prepared = prepare_native_encyclopedia_hd(
                AssetRenderProfile::FaithfulHd,
                Some(Path::new("/synthetic/e52/duplicate-must-not-read")),
                candidates,
            );

            assert_eq!(prepared.diagnostics().len(), 1);
            let diagnostic = &prepared.diagnostics()[0];
            assert_eq!(diagnostic.code, "duplicate_hd_topic_candidate");
            assert_eq!(diagnostic.asset_id, None);
            assert!(diagnostic.detail.contains(TOPIC_ID));
            assert!(!diagnostic.detail.contains("manifest.json"));

            let selected = prepared.select(TOPIC_ID, current);
            match (selected, current) {
                (Some(selected), Some(current)) => {
                    assert!(Arc::ptr_eq(&selected.bytes, &current.bytes));
                    assert_eq!(selected.render_profile, current.render_profile);
                }
                (None, None) => {}
                _ => panic!("duplicate preparation changed the current art decision"),
            }
        }

        let base = image("edata:1", ORIGINAL_BMP, "bmp");
        let same_identity_mod = base.clone();
        let different_mod = image("mod:test/replacement.png", HD_PNG, "png");

        assert_rejected(
            &[
                candidate(&base),
                EncyclopediaImageCandidate::Mod {
                    topic_id: TOPIC_ID,
                    image: &same_identity_mod,
                },
            ],
            Some(&same_identity_mod),
        );
        assert_rejected(
            &[
                EncyclopediaImageCandidate::Mod {
                    topic_id: TOPIC_ID,
                    image: &same_identity_mod,
                },
                candidate(&base),
            ],
            Some(&same_identity_mod),
        );
        assert_rejected(
            &[
                candidate(&base),
                EncyclopediaImageCandidate::Mod {
                    topic_id: TOPIC_ID,
                    image: &different_mod,
                },
            ],
            Some(&different_mod),
        );
        assert_rejected(
            &[
                EncyclopediaImageCandidate::Mod {
                    topic_id: TOPIC_ID,
                    image: &different_mod,
                },
                candidate(&base),
            ],
            Some(&different_mod),
        );
        assert_rejected(
            &[
                candidate(&base),
                EncyclopediaImageCandidate::Null { topic_id: TOPIC_ID },
            ],
            Some(&base),
        );
        assert_rejected(&[candidate(&base), candidate(&base)], Some(&base));
    }

    #[test]
    fn explicit_mod_provenance_outranks_base_hd_even_when_identity_and_bytes_match() {
        let root = TestRoot::new("same-identity-mod");
        let original = image("edata:1", ORIGINAL_BMP, "bmp");
        let mod_selected = original.clone();
        root.write_output(HD_PNG);
        root.write_manifest(&original.digest, &digest(HD_PNG, "png"));

        let prepared = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            Some(root.path()),
            &[
                EncyclopediaImageCandidate::Base {
                    topic_id: "base-topic",
                    image: &original,
                    approval_key: APPROVAL_KEY,
                    output_relative_path: Path::new(OUTPUT_PATH),
                },
                EncyclopediaImageCandidate::Mod {
                    topic_id: "mod-topic",
                    image: &mod_selected,
                },
            ],
        );

        assert_eq!(
            prepared
                .select("base-topic", Some(&original))
                .unwrap()
                .render_profile,
            TopicImageRenderProfile::FaithfulHdLinear
        );
        let selected_mod = prepared.select("mod-topic", Some(&mod_selected)).unwrap();
        assert!(Arc::ptr_eq(&selected_mod.bytes, &mod_selected.bytes));
        assert_eq!(
            selected_mod.render_profile,
            TopicImageRenderProfile::OriginalNearest
        );
    }
}
