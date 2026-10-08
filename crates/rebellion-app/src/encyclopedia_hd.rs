//! Original-first native Encyclopedia artwork selection.
//!
//! Filesystem and manifest work happens once at a session/profile boundary.
//! Frame rendering performs only an exact identity lookup and therefore never
//! reads, hashes, or decodes candidate files while the viewer is open.

use std::collections::BTreeMap;
#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::{Read, Take};
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use rebellion_data::encyclopedia_session::{EncyclopediaSession, MAX_ENCYCLOPEDIA_IMAGE_BYTES};
#[cfg(not(target_arch = "wasm32"))]
use rebellion_render::{approved_hd_assets_from_bytes, AssetRenderProfile};
use rebellion_render::{EncyclopediaArtworkView, EncyclopediaTextureSampling};

#[cfg(not(target_arch = "wasm32"))]
const MAX_HD_MANIFEST_BYTES: usize = 8 * 1024 * 1024;
#[cfg(not(target_arch = "wasm32"))]
const MAX_ENCYCLOPEDIA_HD_ARTWORK_BYTES: usize = 128 * 1024 * 1024;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EncyclopediaHdDiagnostic {
    pub code: &'static str,
    pub asset_id: Option<String>,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OriginalArtworkIdentity {
    filename: String,
    digest: String,
    width: u32,
    height: u32,
}

#[derive(Debug, Clone)]
struct PreparedHdArtwork {
    original: OriginalArtworkIdentity,
    filename: String,
    digest: String,
    width: u32,
    height: u32,
    bytes: Arc<[u8]>,
}

/// Immutable HD choices prepared for one validated base session.
#[derive(Debug, Clone, Default)]
pub(crate) struct PreparedEncyclopediaHd {
    selections: BTreeMap<String, PreparedHdArtwork>,
    #[cfg(not(target_arch = "wasm32"))]
    diagnostics: Vec<EncyclopediaHdDiagnostic>,
}

impl PreparedEncyclopediaHd {
    #[must_use]
    pub(crate) const fn original_only() -> Self {
        Self {
            selections: BTreeMap::new(),
            #[cfg(not(target_arch = "wasm32"))]
            diagnostics: Vec::new(),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub(crate) fn diagnostics(&self) -> &[EncyclopediaHdDiagnostic] {
        &self.diagnostics
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub(crate) fn selected_count(&self) -> usize {
        self.selections.len()
    }

    /// Apply a prepared choice only to the exact original identity for which
    /// it was approved. A later session or overlay mismatch falls back to the
    /// bytes supplied by that newer validated view.
    #[must_use]
    pub(crate) fn select<'a>(
        &'a self,
        original: EncyclopediaArtworkView<'a>,
    ) -> EncyclopediaArtworkView<'a> {
        let Some(selected) = self.selections.get(original.filename) else {
            return original;
        };
        let observed = OriginalArtworkIdentity {
            filename: original.filename.to_owned(),
            digest: original.digest.to_owned(),
            width: original.width,
            height: original.height,
        };
        if observed != selected.original {
            return original;
        }
        EncyclopediaArtworkView {
            resource_id: original.resource_id,
            filename: &selected.filename,
            digest: &selected.digest,
            width: selected.width,
            height: selected.height,
            bytes: &selected.bytes,
            sampling: EncyclopediaTextureSampling::Linear,
        }
    }
}

/// Prepare deterministic native faithful-HD choices for a validated session.
///
/// Original parity returns without touching the supplied path. Every failure
/// in faithful-HD mode records a diagnostic and retains exact original bytes.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn prepare_native_encyclopedia_hd(
    profile: AssetRenderProfile,
    hd_root: Option<&Path>,
    session: &EncyclopediaSession,
) -> PreparedEncyclopediaHd {
    prepare_native_encyclopedia_hd_with_limit(
        profile,
        hd_root,
        session,
        MAX_ENCYCLOPEDIA_HD_ARTWORK_BYTES,
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn prepare_native_encyclopedia_hd_with_limit(
    profile: AssetRenderProfile,
    hd_root: Option<&Path>,
    session: &EncyclopediaSession,
    aggregate_limit: usize,
) -> PreparedEncyclopediaHd {
    let mut prepared = PreparedEncyclopediaHd::original_only();
    if profile == AssetRenderProfile::OriginalParity {
        return prepared;
    }
    let Some(hd_root) = hd_root else {
        prepared.diagnostics.push(global_diagnostic(
            "hd_root_unavailable",
            "faithful-HD was requested without a configured HD root",
        ));
        return prepared;
    };
    let manifest = match read_bounded_file(&hd_root.join("manifest.json"), MAX_HD_MANIFEST_BYTES) {
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
    let approvals = match approved_hd_assets_from_bytes(&manifest) {
        Ok(approvals) => approvals,
        Err(detail) => {
            prepared
                .diagnostics
                .push(global_diagnostic("hd_manifest_invalid", detail));
            return prepared;
        }
    };

    let mut retained_bytes = 0_usize;
    for (filename, metadata) in session.resource_metadata() {
        let Some(resource_id) = edata_resource_id(filename) else {
            // Presentation overlays use confined `mod:v1:*` identities and
            // are outside the original EDATA approval namespace.
            continue;
        };
        let approval_key = format!("edata/EDATA_{resource_id:03}");
        let Some(approval) = approvals.get(&approval_key) else {
            continue;
        };
        let original_bytes = session
            .artwork_bytes(filename)
            .expect("session metadata retains exact artwork bytes");
        if let Err(detail) = approval.validate_source_bytes(original_bytes) {
            prepared
                .diagnostics
                .push(asset_diagnostic("hd_source_mismatch", filename, detail));
            continue;
        }
        if let Err(detail) =
            approval.validate_source_dimensions(metadata.width(), metadata.height())
        {
            prepared.diagnostics.push(asset_diagnostic(
                "hd_source_size_mismatch",
                filename,
                detail,
            ));
            continue;
        }

        let output_filename = format!("EData/EDATA_{resource_id:03}.png");
        let remaining_bytes = aggregate_limit.saturating_sub(retained_bytes);
        let output_read_limit = MAX_ENCYCLOPEDIA_IMAGE_BYTES.min(remaining_bytes);
        let output_bytes =
            match read_bounded_file(&hd_root.join(&output_filename), output_read_limit) {
                Ok(bytes) => bytes,
                Err(BoundedReadError::Unavailable(detail)) => {
                    prepared.diagnostics.push(asset_diagnostic(
                        "hd_output_unavailable",
                        filename,
                        detail,
                    ));
                    continue;
                }
                Err(BoundedReadError::ResourceLimit(detail)) => {
                    let code = if output_read_limit < MAX_ENCYCLOPEDIA_IMAGE_BYTES {
                        "hd_output_aggregate_limit"
                    } else {
                        "hd_output_resource_limit"
                    };
                    prepared
                        .diagnostics
                        .push(asset_diagnostic(code, filename, detail));
                    continue;
                }
            };
        if let Err(detail) = approval.validate_output_bytes(&output_bytes) {
            prepared
                .diagnostics
                .push(asset_diagnostic("hd_output_mismatch", filename, detail));
            continue;
        }
        let (width, height) = match approval.validate_encyclopedia_output_dimensions(
            &output_bytes,
            metadata.width(),
            metadata.height(),
        ) {
            Ok(dimensions) => dimensions,
            Err(detail) => {
                prepared.diagnostics.push(asset_diagnostic(
                    "hd_output_size_mismatch",
                    filename,
                    detail,
                ));
                continue;
            }
        };
        let Some(next_retained_bytes) = retained_bytes.checked_add(output_bytes.len()) else {
            prepared.diagnostics.push(asset_diagnostic(
                "hd_output_aggregate_limit",
                filename,
                "faithful-HD retained byte length overflow",
            ));
            continue;
        };
        if next_retained_bytes > aggregate_limit {
            prepared.diagnostics.push(asset_diagnostic(
                "hd_output_aggregate_limit",
                filename,
                format!("faithful-HD retained artwork exceeds the {aggregate_limit}-byte limit"),
            ));
            continue;
        }
        retained_bytes = next_retained_bytes;
        prepared.selections.insert(
            filename.clone(),
            PreparedHdArtwork {
                original: OriginalArtworkIdentity {
                    filename: filename.clone(),
                    digest: metadata.sha256().to_owned(),
                    width: metadata.width(),
                    height: metadata.height(),
                },
                filename: output_filename,
                digest: approval.output_sha256().to_owned(),
                width,
                height,
                bytes: Arc::from(output_bytes.into_boxed_slice()),
            },
        );
    }
    prepared
}

#[cfg(not(target_arch = "wasm32"))]
fn edata_resource_id(filename: &str) -> Option<u16> {
    let digits = filename.strip_prefix("EDATA.")?;
    (digits.len() == 3 && digits.bytes().all(|byte| byte.is_ascii_digit()))
        .then(|| digits.parse().ok())?
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
    use std::collections::{BTreeMap, HashMap};
    use std::io::Cursor;
    use std::sync::Arc;

    use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};
    use rebellion_data::encyclopedia_catalog::{
        EncyclopediaCatalog, EncyclopediaCatalogEntry, EncyclopediaCategory,
    };
    use rebellion_data::encyclopedia_presenter::{EncyclopediaEntryIntent, EncyclopediaPresenter};
    use rebellion_data::encyclopedia_session::{
        EncyclopediaResourceBytes, EncyclopediaSession, EncyclopediaSessionInput,
        EncyclopediaSessionStore,
    };
    use rebellion_data::encyclopedia_topics::EncyclopediaAudience;
    use rebellion_render::{AssetRenderProfile, EncyclopediaTextureSampling};
    use serde_json::json;
    use sha2::{Digest, Sha256};
    use tempfile::TempDir;

    use super::{prepare_native_encyclopedia_hd, prepare_native_encyclopedia_hd_with_limit};
    use crate::encyclopedia_surface::{
        adapt_encyclopedia_surface, adapt_encyclopedia_surface_with_hd,
    };

    const SOURCE: &[u8] = include_bytes!("../../../tests/fixtures/encyclopedia/p66a/source.json");
    const MANIFEST: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/p66a/source.json.manifest.json");
    const OBJECT_ID: u32 = 0x5100_0010;
    const RESOURCE_ID: u16 = 0x1c50;
    const ARTWORK: &str = "EDATA.115";
    const APPROVAL_KEY: &str = "edata/EDATA_115";
    const OUTPUT_PATH: &str = "EData/EDATA_115.png";

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
                    object_id: OBJECT_ID,
                    text_resource_id: RESOURCE_ID,
                    name: "Diplomacy".into(),
                },
                EncyclopediaCatalogEntry {
                    object_id: 0x1400_0040,
                    text_resource_id: 0x2740,
                    name: "Mon Calamari Cruiser".into(),
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

    fn session() -> Arc<EncyclopediaSession> {
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

    fn png(width: u32, height: u32) -> Vec<u8> {
        let image = RgbaImage::from_pixel(width, height, Rgba([21, 42, 84, 255]));
        let mut bytes = Vec::new();
        DynamicImage::ImageRgba8(image)
            .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .unwrap();
        bytes
    }

    fn digest(bytes: &[u8]) -> String {
        format!("{:x}", Sha256::digest(bytes))
    }

    fn write_approval(
        root: &TempDir,
        source_digest: &str,
        output_digest: &str,
        output_dimensions: (u32, u32),
    ) {
        let manifest = json!({
            "schema_version": 1,
            "profile": "faithful-hd",
            "assets": {
                APPROVAL_KEY: {
                    "approved": true,
                    "review": {"reviewer": "synthetic-reviewer", "evidence": "W6 test"},
                    "gates": {"human_review": "pass"},
                    "source": {
                        "sha256": source_digest,
                        "width": 400,
                        "height": 200
                    },
                    "output": {
                        "sha256": output_digest,
                        "width": output_dimensions.0,
                        "height": output_dimensions.1,
                        "format": "png"
                    }
                }
            }
        });
        std::fs::write(
            root.path().join("manifest.json"),
            serde_json::to_vec(&manifest).unwrap(),
        )
        .unwrap();
    }

    fn write_output(root: &TempDir, bytes: &[u8]) {
        let output = root.path().join(OUTPUT_PATH);
        std::fs::create_dir_all(output.parent().unwrap()).unwrap();
        std::fs::write(output, bytes).unwrap();
    }

    #[test]
    fn edata_hd_identity_requires_exactly_three_decimal_digits() {
        assert_eq!(super::edata_resource_id("EDATA.001"), Some(1));
        assert_eq!(super::edata_resource_id("EDATA.999"), Some(999));
        assert_eq!(super::edata_resource_id("EDATA.01"), None);
        assert_eq!(super::edata_resource_id("EDATA.0001"), None);
        assert_eq!(super::edata_resource_id("EDATA.A01"), None);
        assert_eq!(super::edata_resource_id("edata.001"), None);
    }

    fn original_surface(
        session: &EncyclopediaSession,
    ) -> rebellion_render::EncyclopediaSurface<'_> {
        let presentation = EncyclopediaPresenter::present(
            session,
            EncyclopediaEntryIntent::Object {
                audience: EncyclopediaAudience::Empire,
                category_command: 0x6f,
                object_id: OBJECT_ID,
            },
        )
        .unwrap();
        adapt_encyclopedia_surface(session, &presentation)
    }

    #[test]
    fn approved_exact_four_x_png_replaces_only_the_rendered_view() {
        let session = session();
        let original = original_surface(&session)
            .active_topic
            .unwrap()
            .artwork
            .unwrap();
        let root = TempDir::new().unwrap();
        let enhanced = png(1600, 800);
        write_output(&root, &enhanced);
        write_approval(&root, original.digest, &digest(&enhanced), (1600, 800));

        let prepared = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            Some(root.path()),
            &session,
        );
        let selected = prepared.select(original);

        assert!(prepared.diagnostics().is_empty());
        assert_eq!(prepared.selected_count(), 1);
        assert_eq!(selected.filename, OUTPUT_PATH);
        assert_eq!(selected.digest, digest(&enhanced));
        assert_eq!((selected.width, selected.height), (1600, 800));
        assert_eq!(selected.bytes, enhanced);
        assert_eq!(selected.sampling, EncyclopediaTextureSampling::Linear);
        assert_eq!(session.artwork_bytes(ARTWORK).unwrap(), original.bytes);
    }

    #[test]
    fn faithful_hd_retained_bytes_respect_the_aggregate_limit() {
        let session = session();
        let original = original_surface(&session)
            .active_topic
            .unwrap()
            .artwork
            .unwrap();
        let root = TempDir::new().unwrap();
        let enhanced = png(1600, 800);
        write_output(&root, &enhanced);
        write_approval(&root, original.digest, &digest(&enhanced), (1600, 800));

        let prepared = prepare_native_encyclopedia_hd_with_limit(
            AssetRenderProfile::FaithfulHd,
            Some(root.path()),
            &session,
            enhanced.len() - 1,
        );

        assert_eq!(prepared.selected_count(), 0);
        assert_eq!(prepared.select(original), original);
        assert!(prepared
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.code == "hd_output_aggregate_limit"));
    }

    #[test]
    fn every_invalid_or_missing_hd_case_falls_back_to_the_exact_original() {
        let session = session();
        let original = original_surface(&session)
            .active_topic
            .unwrap()
            .artwork
            .unwrap();

        let original_only = prepare_native_encyclopedia_hd(
            AssetRenderProfile::OriginalParity,
            Some(std::path::Path::new("/must/not/be/read")),
            &session,
        );
        assert!(original_only.diagnostics().is_empty());
        assert_eq!(original_only.selected_count(), 0);
        assert_eq!(original_only.select(original), original);

        let missing = TempDir::new().unwrap();
        write_approval(&missing, original.digest, &digest(b"missing"), (1600, 800));
        let prepared = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            Some(missing.path()),
            &session,
        );
        assert_eq!(prepared.select(original), original);
        assert!(prepared
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.code == "hd_output_unavailable"));

        let bad_digest = TempDir::new().unwrap();
        let enhanced = png(1600, 800);
        write_output(&bad_digest, &enhanced);
        write_approval(&bad_digest, original.digest, &"a".repeat(64), (1600, 800));
        let prepared = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            Some(bad_digest.path()),
            &session,
        );
        assert_eq!(prepared.select(original), original);
        assert!(prepared
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.code == "hd_output_mismatch"));

        let bad_size = TempDir::new().unwrap();
        let half_size = png(800, 400);
        write_output(&bad_size, &half_size);
        write_approval(&bad_size, original.digest, &digest(&half_size), (1600, 800));
        let prepared = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            Some(bad_size.path()),
            &session,
        );
        assert_eq!(prepared.select(original), original);
        assert!(prepared
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.code == "hd_output_size_mismatch"));
    }

    #[test]
    fn the_w4_adapter_carries_selected_identity_and_sampling_without_changing_the_presenter() {
        let session = session();
        let presentation = EncyclopediaPresenter::present(
            &session,
            EncyclopediaEntryIntent::Object {
                audience: EncyclopediaAudience::Empire,
                category_command: 0x6f,
                object_id: OBJECT_ID,
            },
        )
        .unwrap();
        let original = adapt_encyclopedia_surface(&session, &presentation)
            .active_topic
            .unwrap()
            .artwork
            .unwrap();
        let root = TempDir::new().unwrap();
        let enhanced = png(1600, 800);
        write_output(&root, &enhanced);
        write_approval(&root, original.digest, &digest(&enhanced), (1600, 800));
        let prepared = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            Some(root.path()),
            &session,
        );

        let surface = adapt_encyclopedia_surface_with_hd(&session, &presentation, &prepared);
        let selected = surface.active_topic.unwrap().artwork.unwrap();

        assert_eq!(selected.filename, OUTPUT_PATH);
        assert_eq!(selected.sampling, EncyclopediaTextureSampling::Linear);
        assert_eq!(presentation.active_topic.unwrap().object_id, OBJECT_ID);
    }

    #[test]
    #[ignore = "requires owned canonical source/EData and a separately approved faithful-HD pack"]
    fn owned_approved_hd_pack_selects_only_validated_linear_artwork() {
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
        let hd_root = std::path::PathBuf::from(
            std::env::var_os("REBELLION_HD_ROOT").expect("set REBELLION_HD_ROOT"),
        );
        let payload = crate::encyclopedia_content::read_native_encyclopedia(gdata, &edata)
            .unwrap()
            .expect("owned native Encyclopedia is published");
        let catalog =
            rebellion_data::encyclopedia_catalog::load_encyclopedia_catalog(gdata).unwrap();
        let system_pictures =
            rebellion_data::encyclopedia_topics::load_encyclopedia_system_pictures(gdata).unwrap();
        let mut store = EncyclopediaSessionStore::default();
        store
            .replace(payload.into_session_input(catalog, system_pictures))
            .unwrap();
        let session = store.current().unwrap();
        let prepared = prepare_native_encyclopedia_hd(
            AssetRenderProfile::FaithfulHd,
            Some(&hd_root),
            &session,
        );

        assert!(
            prepared.diagnostics().is_empty(),
            "approved pack must have no fallback diagnostics: {:?}",
            prepared.diagnostics()
        );
        assert!(
            prepared.selected_count() > 0,
            "approved pack has no EDATA choices"
        );
        let mut visible_selections = 0;
        for audience in [EncyclopediaAudience::Alliance, EncyclopediaAudience::Empire] {
            for entry in &session.catalog().entries {
                let presentation = EncyclopediaPresenter::present(
                    &session,
                    EncyclopediaEntryIntent::Object {
                        audience,
                        category_command: 0x6f,
                        object_id: entry.object_id,
                    },
                )
                .unwrap();
                let surface =
                    adapt_encyclopedia_surface_with_hd(&session, &presentation, &prepared);
                if let Some(artwork) = surface
                    .active_topic
                    .and_then(|topic| topic.artwork)
                    .filter(|artwork| artwork.sampling == EncyclopediaTextureSampling::Linear)
                {
                    assert_eq!((artwork.width, artwork.height), (1600, 800));
                    visible_selections += 1;
                }
            }
        }
        assert!(
            visible_selections > 0,
            "approved choices are not presenter-visible"
        );
    }
}
