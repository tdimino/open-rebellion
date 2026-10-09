//! Canonical native, packed, and development-loose Encyclopedia byte readers.

use std::collections::{BTreeMap, BTreeSet, HashMap};
#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Read;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
use std::sync::Arc;

use anyhow::{bail, ensure, Context, Result};
use rebellion_data::encyclopedia_catalog::EncyclopediaCatalog;
use rebellion_data::encyclopedia_session::MAX_ENCYCLOPEDIA_IMAGE_BYTES;
use rebellion_data::encyclopedia_session::{EncyclopediaResourceBytes, EncyclopediaSessionInput};
use rebellion_data::encyclopedia_topics::parse_encyclopedia_source_with_manifest;
#[cfg(any(test, target_arch = "wasm32"))]
use serde::Deserialize;

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) const ENCYCLOPEDIA_NAMESPACE: &str = "encyclopedia/";
const CATALOG_KEY: &str = "catalog.json";
const MANIFEST_KEY: &str = "manifest.json";
const ASSET_PREFIX: &str = "assets/";
const MAX_ENCYCLOPEDIA_ARTWORK_BYTES: usize = 128 * 1024 * 1024;
#[cfg(not(target_arch = "wasm32"))]
const CATALOG_BYTES_LIMIT: usize = 64 * 1024 * 1024;
#[cfg(not(target_arch = "wasm32"))]
const MANIFEST_BYTES_LIMIT: usize = 32 * 1024 * 1024;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg(any(test, target_arch = "wasm32"))]
struct LooseEncyclopediaPointer {
    schema_version: u32,
    available: bool,
    generation: Option<String>,
}

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) fn parse_loose_pointer(bytes: &[u8]) -> Result<Option<String>> {
    let pointer: LooseEncyclopediaPointer =
        serde_json::from_slice(bytes).context("parsing loose Encyclopedia pointer")?;
    ensure!(
        pointer.schema_version == 1,
        "unsupported loose Encyclopedia pointer version {}",
        pointer.schema_version
    );
    if !pointer.available {
        ensure!(
            pointer.generation.is_none(),
            "unavailable loose Encyclopedia pointer names a generation"
        );
        return Ok(None);
    }
    let generation = pointer
        .generation
        .context("available loose Encyclopedia pointer has no generation")?;
    ensure!(
        generation.len() == 64
            && generation
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
        "loose Encyclopedia pointer has an invalid generation"
    );
    Ok(Some(generation))
}

pub(crate) fn checked_artwork_transfer_total(retained: usize, byte_len: usize) -> Result<usize> {
    ensure!(byte_len > 0, "empty Encyclopedia artwork resource");
    ensure!(
        byte_len <= MAX_ENCYCLOPEDIA_IMAGE_BYTES,
        "Encyclopedia artwork resource exceeds the byte limit"
    );
    let total = retained
        .checked_add(byte_len)
        .context("Encyclopedia artwork aggregate byte length overflow")?;
    ensure!(
        total <= MAX_ENCYCLOPEDIA_ARTWORK_BYTES,
        "Encyclopedia artwork aggregate byte limit exceeded"
    );
    Ok(total)
}

/// Complete, already-loaded bytes shared by every platform reader.
#[derive(Debug, Clone)]
pub(crate) struct EncyclopediaContentPayload {
    source_catalog_bytes: Arc<[u8]>,
    source_manifest_bytes: Arc<[u8]>,
    artwork: EncyclopediaResourceBytes,
}

impl EncyclopediaContentPayload {
    /// Remove and validate the optional Encyclopedia namespace from an ORPK map.
    ///
    /// A wholly absent namespace is compatible with older user-provided packs.
    /// Any present namespace must be complete and valid.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn take_from_runtime_pack(
        entries: &mut HashMap<String, Vec<u8>>,
    ) -> Result<Option<Self>> {
        let keys: Vec<String> = entries
            .keys()
            .filter(|key| key.starts_with(ENCYCLOPEDIA_NAMESPACE))
            .cloned()
            .collect();
        if keys.is_empty() {
            return Ok(None);
        }
        let mut relative = HashMap::with_capacity(keys.len());
        for key in keys {
            let bytes = entries
                .remove(&key)
                .expect("collected runtime-pack key remains present");
            let stripped = key
                .strip_prefix(ENCYCLOPEDIA_NAMESPACE)
                .expect("collected runtime-pack key has the namespace");
            if relative.insert(stripped.to_owned(), bytes).is_some() {
                bail!("duplicate Encyclopedia runtime key {key}");
            }
        }
        Self::from_relative_entries(relative).map(Some)
    }

    /// Validate a development-loose entry set using the same content boundary.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn from_loose_entries(mut entries: HashMap<String, Vec<u8>>) -> Result<Self> {
        Self::take_from_runtime_pack(&mut entries)?
            .context("development-loose Encyclopedia namespace is absent")
    }

    fn from_relative_entries(mut entries: HashMap<String, Vec<u8>>) -> Result<Self> {
        for key in entries.keys() {
            ensure!(
                safe_relative_key(key),
                "unsafe Encyclopedia content key {key:?}"
            );
        }
        let source_catalog_bytes = entries
            .remove(CATALOG_KEY)
            .with_context(|| format!("Encyclopedia namespace is missing {CATALOG_KEY}"))?;
        let source_manifest_bytes = entries
            .remove(MANIFEST_KEY)
            .with_context(|| format!("Encyclopedia namespace is missing {MANIFEST_KEY}"))?;
        let (source, _) =
            parse_encyclopedia_source_with_manifest(&source_catalog_bytes, &source_manifest_bytes)?;
        let expected: BTreeSet<String> = source.artwork.values().cloned().collect();
        let supplied: BTreeSet<String> = entries
            .keys()
            .filter_map(|key| key.strip_prefix(ASSET_PREFIX).map(str::to_owned))
            .collect();
        if let Some(filename) = expected.difference(&supplied).next() {
            bail!("Encyclopedia namespace is missing assets/{filename}");
        }
        if let Some(filename) = supplied.difference(&expected).next() {
            bail!("unexpected Encyclopedia asset {filename}");
        }
        ensure!(
            entries.len() == supplied.len(),
            "unexpected Encyclopedia metadata entry"
        );

        let mut artwork = BTreeMap::new();
        let mut retained_bytes = 0;
        for filename in expected {
            let key = format!("{ASSET_PREFIX}{filename}");
            let bytes = entries
                .remove(&key)
                .expect("validated Encyclopedia artwork key remains present");
            retained_bytes = checked_artwork_transfer_total(retained_bytes, bytes.len())?;
            artwork.insert(filename, Arc::from(bytes));
        }
        Ok(Self {
            source_catalog_bytes: Arc::from(source_catalog_bytes),
            source_manifest_bytes: Arc::from(source_manifest_bytes),
            artwork,
        })
    }

    pub(crate) fn into_session_input(
        self,
        catalog: EncyclopediaCatalog,
        system_pictures: HashMap<u32, u32>,
    ) -> EncyclopediaSessionInput {
        EncyclopediaSessionInput {
            catalog,
            source_catalog_bytes: self.source_catalog_bytes,
            source_manifest_bytes: self.source_manifest_bytes,
            system_pictures,
            artwork: self.artwork,
        }
    }

    /// Temporary compatibility bytes for the pre-W5 renderer fixture.
    ///
    /// Production topic rendering consumes the session-owned `Arc` bytes. The
    /// existing artwork transport probe still observes this basename cache.
    #[cfg(any(test, all(target_arch = "wasm32", feature = "interface-test-fixtures")))]
    pub(crate) fn legacy_renderer_artwork(&self) -> HashMap<String, Vec<u8>> {
        self.artwork
            .iter()
            .map(|(filename, bytes)| (filename.clone(), bytes.to_vec()))
            .collect()
    }

    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn artwork_count(&self) -> usize {
        self.artwork.len()
    }
}

fn safe_relative_key(key: &str) -> bool {
    key == CATALOG_KEY
        || key == MANIFEST_KEY
        || key.strip_prefix(ASSET_PREFIX).is_some_and(|filename| {
            !filename.is_empty()
                && !filename.contains(['/', '\\'])
                && filename.starts_with("EDATA.")
        })
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn read_native_encyclopedia(
    gdata_path: &Path,
    edata_path: &Path,
) -> Result<Option<EncyclopediaContentPayload>> {
    read_native_encyclopedia_with_artwork_limit(
        gdata_path,
        edata_path,
        MAX_ENCYCLOPEDIA_ARTWORK_BYTES,
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn read_native_encyclopedia_with_artwork_limit(
    gdata_path: &Path,
    edata_path: &Path,
    artwork_limit: usize,
) -> Result<Option<EncyclopediaContentPayload>> {
    let catalog_path = gdata_path.join("encyclopedia/source.json");
    let manifest_path = gdata_path.join("encyclopedia/source.json.manifest.json");
    let catalog_present = catalog_path.exists();
    let manifest_present = manifest_path.exists();
    if !catalog_present && !manifest_present {
        return Ok(None);
    }
    ensure!(
        catalog_present && manifest_present,
        "native Encyclopedia publication is partial"
    );
    let catalog = read_regular_bounded(
        &catalog_path,
        CATALOG_BYTES_LIMIT,
        "native Encyclopedia catalog",
    )?;
    let manifest = read_regular_bounded(
        &manifest_path,
        MANIFEST_BYTES_LIMIT,
        "native Encyclopedia manifest",
    )?;
    let (source, _) = parse_encyclopedia_source_with_manifest(&catalog, &manifest)?;
    let mut entries = HashMap::from([
        (CATALOG_KEY.to_owned(), catalog),
        (MANIFEST_KEY.to_owned(), manifest),
    ]);
    let mut retained_artwork_bytes = 0_usize;
    for filename in source.artwork.values().collect::<BTreeSet<_>>() {
        let path = edata_path.join(filename);
        let remaining_artwork_bytes = artwork_limit
            .checked_sub(retained_artwork_bytes)
            .context("Encyclopedia artwork aggregate byte limit exceeded")?;
        let read_limit = MAX_ENCYCLOPEDIA_IMAGE_BYTES.min(remaining_artwork_bytes);
        let bytes = read_regular_bounded(&path, read_limit, "native Encyclopedia artwork");
        let bytes = if read_limit < MAX_ENCYCLOPEDIA_IMAGE_BYTES {
            bytes.context("Encyclopedia artwork aggregate byte limit exceeded")?
        } else {
            bytes?
        };
        retained_artwork_bytes =
            checked_artwork_transfer_total(retained_artwork_bytes, bytes.len())?;
        entries.insert(format!("{ASSET_PREFIX}{filename}"), bytes);
    }
    EncyclopediaContentPayload::from_relative_entries(entries).map(Some)
}

#[cfg(not(target_arch = "wasm32"))]
fn read_regular_bounded(path: &Path, limit: usize, description: &str) -> Result<Vec<u8>> {
    let path_metadata = path
        .symlink_metadata()
        .with_context(|| format!("reading {description} metadata from {}", path.display()))?;
    ensure!(
        path_metadata.is_file(),
        "{description} is not a regular file"
    );
    let file =
        File::open(path).with_context(|| format!("opening {description} at {}", path.display()))?;
    let metadata = file.metadata().with_context(|| {
        format!(
            "reading open {description} metadata from {}",
            path.display()
        )
    })?;
    ensure!(metadata.is_file(), "{description} is not a regular file");
    let byte_len = usize::try_from(metadata.len()).context("content length does not fit usize")?;
    ensure!(byte_len <= limit, "{description} exceeds the byte limit");
    let mut bytes = Vec::with_capacity(byte_len.min(limit));
    let take_limit = u64::try_from(limit)
        .context("configured content byte limit does not fit u64")?
        .checked_add(1)
        .context("configured content byte limit overflow")?;
    file.take(take_limit)
        .read_to_end(&mut bytes)
        .with_context(|| format!("reading {description} from {}", path.display()))?;
    ensure!(
        bytes.len() <= limit,
        "{description} exceeded the byte limit while reading"
    );
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rebellion_data::encyclopedia_catalog::{
        EncyclopediaCatalog, EncyclopediaCatalogEntry, EncyclopediaCategory,
    };
    use rebellion_data::encyclopedia_session::EncyclopediaSessionStore;
    use rebellion_data::encyclopedia_topics::EncyclopediaAudience;
    use tempfile::tempdir;

    use super::{
        checked_artwork_transfer_total, parse_encyclopedia_source_with_manifest,
        parse_loose_pointer, read_native_encyclopedia, read_native_encyclopedia_with_artwork_limit,
        safe_relative_key, EncyclopediaContentPayload, CATALOG_BYTES_LIMIT, MANIFEST_BYTES_LIMIT,
        MAX_ENCYCLOPEDIA_ARTWORK_BYTES, MAX_ENCYCLOPEDIA_IMAGE_BYTES,
    };

    const SOURCE: &[u8] = include_bytes!("../../../tests/fixtures/encyclopedia/w4/source.json");
    const MANIFEST: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/w4/source.json.manifest.json");

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
                    object_id: 0x5100_0010,
                    text_resource_id: 0x2c50,
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

    fn indexed_bmp() -> Vec<u8> {
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
        bytes.resize(FILE_BYTES as usize, 1);
        bytes
    }

    fn namespaced_entries() -> HashMap<String, Vec<u8>> {
        namespaced_entries_with(SOURCE, MANIFEST, &indexed_bmp())
    }

    fn namespaced_entries_with(
        source: &[u8],
        manifest: &[u8],
        artwork: &[u8],
    ) -> HashMap<String, Vec<u8>> {
        let mut entries = HashMap::from([
            ("encyclopedia/catalog.json".into(), source.to_vec()),
            ("encyclopedia/manifest.json".into(), manifest.to_vec()),
        ]);
        for filename in ["EDATA.001", "EDATA.014", "EDATA.015", "EDATA.115"] {
            entries.insert(format!("encyclopedia/assets/{filename}"), artwork.to_vec());
        }
        entries
    }

    fn read_native_fixture(
        source: &[u8],
        manifest: &[u8],
        artwork: &[u8],
    ) -> anyhow::Result<Option<EncyclopediaContentPayload>> {
        let root = tempdir().unwrap();
        let gdata = root.path().join("GData");
        let source_dir = gdata.join("encyclopedia");
        let edata = root.path().join("EData");
        std::fs::create_dir_all(&source_dir).unwrap();
        std::fs::create_dir_all(&edata).unwrap();
        std::fs::write(source_dir.join("source.json"), source).unwrap();
        std::fs::write(source_dir.join("source.json.manifest.json"), manifest).unwrap();
        for filename in ["EDATA.001", "EDATA.014", "EDATA.015", "EDATA.115"] {
            std::fs::write(edata.join(filename), artwork).unwrap();
        }
        read_native_encyclopedia(&gdata, &edata)
    }

    #[test]
    fn absent_namespace_is_unavailable_but_partial_or_unknown_namespace_fails() {
        let mut absent = HashMap::from([("SYSTEMSD.DAT".into(), vec![1])]);
        assert!(
            EncyclopediaContentPayload::take_from_runtime_pack(&mut absent)
                .unwrap()
                .is_none()
        );

        let mut partial = HashMap::from([("encyclopedia/catalog.json".into(), SOURCE.to_vec())]);
        let error = EncyclopediaContentPayload::take_from_runtime_pack(&mut partial).unwrap_err();
        assert!(error.to_string().contains("manifest.json"));

        let mut unknown = namespaced_entries();
        unknown.insert("encyclopedia/../escape".into(), vec![1]);
        let error = EncyclopediaContentPayload::take_from_runtime_pack(&mut unknown).unwrap_err();
        assert!(error.to_string().contains("unsafe"));
    }

    #[test]
    fn native_packed_and_loose_bytes_install_identical_sessions() {
        let root = tempdir().unwrap();
        let gdata = root.path().join("GData");
        let source_dir = gdata.join("encyclopedia");
        let edata = root.path().join("EData");
        std::fs::create_dir_all(&source_dir).unwrap();
        std::fs::create_dir_all(&edata).unwrap();
        std::fs::write(source_dir.join("source.json"), SOURCE).unwrap();
        std::fs::write(source_dir.join("source.json.manifest.json"), MANIFEST).unwrap();
        for filename in ["EDATA.001", "EDATA.014", "EDATA.015", "EDATA.115"] {
            std::fs::write(edata.join(filename), indexed_bmp()).unwrap();
        }

        let native = read_native_encyclopedia(&gdata, &edata).unwrap().unwrap();
        let mut packed_entries = namespaced_entries();
        let packed = EncyclopediaContentPayload::take_from_runtime_pack(&mut packed_entries)
            .unwrap()
            .unwrap();
        let loose = EncyclopediaContentPayload::from_loose_entries(namespaced_entries()).unwrap();

        let mut fingerprints = Vec::new();
        let mut counts = Vec::new();
        for payload in [native, packed, loose] {
            let mut store = EncyclopediaSessionStore::default();
            store
                .replace(payload.into_session_input(catalog(), HashMap::new()))
                .unwrap();
            let session = store.current().unwrap();
            fingerprints.push(session.logical_fingerprint().to_owned());
            counts.push((
                session.topics(EncyclopediaAudience::Alliance).entries.len(),
                session.topics(EncyclopediaAudience::Empire).entries.len(),
            ));
        }
        assert!(fingerprints.windows(2).all(|pair| pair[0] == pair[1]));
        assert_eq!(counts, vec![(2, 2); 3]);
    }

    #[test]
    fn native_acquisition_enforces_the_aggregate_limit_while_reading() {
        let root = tempdir().unwrap();
        let gdata = root.path().join("GData");
        let source_dir = gdata.join("encyclopedia");
        let edata = root.path().join("EData");
        std::fs::create_dir_all(&source_dir).unwrap();
        std::fs::create_dir_all(&edata).unwrap();
        std::fs::write(source_dir.join("source.json"), SOURCE).unwrap();
        std::fs::write(source_dir.join("source.json.manifest.json"), MANIFEST).unwrap();
        let artwork = indexed_bmp();
        for filename in ["EDATA.001", "EDATA.014", "EDATA.015", "EDATA.115"] {
            std::fs::write(edata.join(filename), &artwork).unwrap();
        }

        let error = read_native_encyclopedia_with_artwork_limit(&gdata, &edata, artwork.len() * 3)
            .unwrap_err();
        assert!(error
            .to_string()
            .contains("artwork aggregate byte limit exceeded"));
    }

    #[test]
    fn native_packed_and_loose_readers_reject_the_same_invalid_candidates() {
        let malformed = b"{";
        assert!(read_native_fixture(malformed, MANIFEST, &indexed_bmp()).is_err());
        let mut packed = namespaced_entries_with(malformed, MANIFEST, &indexed_bmp());
        assert!(EncyclopediaContentPayload::take_from_runtime_pack(&mut packed).is_err());
        assert!(
            EncyclopediaContentPayload::from_loose_entries(namespaced_entries_with(
                malformed,
                MANIFEST,
                &indexed_bmp(),
            ))
            .is_err()
        );

        let malformed_artwork = [1_u8];
        let native = read_native_fixture(SOURCE, MANIFEST, &malformed_artwork)
            .unwrap()
            .unwrap();
        let mut packed_entries = namespaced_entries_with(SOURCE, MANIFEST, &malformed_artwork);
        let packed = EncyclopediaContentPayload::take_from_runtime_pack(&mut packed_entries)
            .unwrap()
            .unwrap();
        let loose = EncyclopediaContentPayload::from_loose_entries(namespaced_entries_with(
            SOURCE,
            MANIFEST,
            &malformed_artwork,
        ))
        .unwrap();
        for payload in [native, packed, loose] {
            let mut store = EncyclopediaSessionStore::default();
            assert!(store
                .replace(payload.into_session_input(catalog(), HashMap::new()))
                .is_err());
            assert!(store.current().is_none());
        }
    }

    #[test]
    fn payload_rejects_unexpected_or_missing_artwork_before_session_install() {
        let mut missing = namespaced_entries();
        missing.remove("encyclopedia/assets/EDATA.115");
        let error = EncyclopediaContentPayload::from_loose_entries(missing).unwrap_err();
        assert!(error.to_string().contains("EDATA.115"));

        let mut unexpected = namespaced_entries();
        unexpected.insert("encyclopedia/assets/EDATA.999".into(), indexed_bmp());
        let error = EncyclopediaContentPayload::from_loose_entries(unexpected).unwrap_err();
        assert!(error.to_string().contains("EDATA.999"));
    }

    #[test]
    fn content_key_and_native_presence_boundaries_fail_closed() {
        assert!(safe_relative_key("catalog.json"));
        assert!(safe_relative_key("manifest.json"));
        assert!(safe_relative_key("assets/EDATA.001"));
        assert!(!safe_relative_key("assets/"));
        assert!(!safe_relative_key("assets/not-edata"));
        assert!(!safe_relative_key("assets/EDATA.001/nested"));
        assert!(!safe_relative_key("assets/EDATA.001\\nested"));

        let root = tempdir().unwrap();
        let gdata = root.path().join("GData");
        let source_dir = gdata.join("encyclopedia");
        let edata = root.path().join("EData");
        std::fs::create_dir_all(&source_dir).unwrap();
        std::fs::create_dir_all(&edata).unwrap();
        assert!(read_native_encyclopedia(&gdata, &edata).unwrap().is_none());

        std::fs::write(source_dir.join("source.json"), b"{}").unwrap();
        assert!(read_native_encyclopedia(&gdata, &edata).is_err());
        std::fs::remove_file(source_dir.join("source.json")).unwrap();
        std::fs::write(source_dir.join("source.json.manifest.json"), b"{}").unwrap();
        assert!(read_native_encyclopedia(&gdata, &edata).is_err());
    }

    #[test]
    fn loose_generation_pointer_is_bounded_and_fail_closed() {
        assert_eq!(
            parse_loose_pointer(br#"{"schema_version":1,"available":false,"generation":null}"#)
                .unwrap(),
            None
        );
        let generation = "a".repeat(64);
        assert_eq!(
            parse_loose_pointer(
                format!(r#"{{"schema_version":1,"available":true,"generation":"{generation}"}}"#)
                    .as_bytes()
            )
            .unwrap(),
            Some(generation)
        );
        for invalid in [
            br#"{"schema_version":1,"available":true,"generation":null}"#.as_slice(),
            br#"{"schema_version":1,"available":false,"generation":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}"#,
            br#"{"schema_version":1,"available":true,"generation":"../escape"}"#,
            br#"{"schema_version":1,"available":false,"generation":null,"extra":1}"#,
        ] {
            assert!(parse_loose_pointer(invalid).is_err());
        }
    }

    #[test]
    fn artwork_transfer_limits_reject_empty_single_and_aggregate_overflow() {
        assert_eq!(MAX_ENCYCLOPEDIA_ARTWORK_BYTES, 128 * 1024 * 1024);
        assert_eq!(CATALOG_BYTES_LIMIT, 64 * 1024 * 1024);
        assert_eq!(MANIFEST_BYTES_LIMIT, 32 * 1024 * 1024);
        assert!(checked_artwork_transfer_total(0, 0).is_err());
        assert!(checked_artwork_transfer_total(0, MAX_ENCYCLOPEDIA_IMAGE_BYTES + 1).is_err());
        assert!(checked_artwork_transfer_total(128 * 1024 * 1024, 1).is_err());
        assert_eq!(checked_artwork_transfer_total(7, 11).unwrap(), 18);
    }

    #[test]
    fn fixture_transport_views_preserve_validated_artwork() {
        let payload = EncyclopediaContentPayload::from_loose_entries(namespaced_entries()).unwrap();
        assert_eq!(payload.artwork_count(), 4);
        let legacy = payload.legacy_renderer_artwork();
        assert_eq!(legacy.len(), 4);
        for filename in ["EDATA.001", "EDATA.014", "EDATA.015", "EDATA.115"] {
            assert_eq!(legacy[filename], indexed_bmp());
        }
    }

    #[test]
    #[ignore = "requires an owned ignored P66A source, EData directory, ORPK, and loose mirror"]
    fn owned_native_packed_and_loose_publications_match_exactly() {
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
        let pack_path = std::path::PathBuf::from(
            std::env::var_os("REBELLION_ENCYCLOPEDIA_TEST_PACK")
                .expect("set REBELLION_ENCYCLOPEDIA_TEST_PACK"),
        );
        let mirror = std::path::PathBuf::from(
            std::env::var_os("REBELLION_ENCYCLOPEDIA_TEST_MIRROR")
                .expect("set REBELLION_ENCYCLOPEDIA_TEST_MIRROR"),
        );

        let native = read_native_encyclopedia(gdata, &edata).unwrap().unwrap();
        let mut runtime_pack =
            crate::runtime_pack::parse_runtime_pack(&std::fs::read(pack_path).unwrap()).unwrap();
        let packed =
            EncyclopediaContentPayload::take_from_runtime_pack(&mut runtime_pack.game_files)
                .unwrap()
                .unwrap();
        let generation = parse_loose_pointer(&std::fs::read(mirror.join("current.json")).unwrap())
            .unwrap()
            .unwrap();
        let generation_root = mirror.join("generations").join(generation);
        let catalog_bytes = std::fs::read(generation_root.join("catalog.json")).unwrap();
        let manifest_bytes = std::fs::read(generation_root.join("manifest.json")).unwrap();
        let (source_catalog, _) =
            parse_encyclopedia_source_with_manifest(&catalog_bytes, &manifest_bytes).unwrap();
        let mut loose_entries = HashMap::from([
            ("encyclopedia/catalog.json".into(), catalog_bytes),
            ("encyclopedia/manifest.json".into(), manifest_bytes),
        ]);
        for filename in source_catalog
            .artwork
            .values()
            .collect::<std::collections::BTreeSet<_>>()
        {
            loose_entries.insert(
                format!("encyclopedia/assets/{filename}"),
                std::fs::read(generation_root.join("assets").join(filename)).unwrap(),
            );
        }
        let loose = EncyclopediaContentPayload::from_loose_entries(loose_entries).unwrap();

        assert_eq!(native.source_catalog_bytes, packed.source_catalog_bytes);
        assert_eq!(native.source_catalog_bytes, loose.source_catalog_bytes);
        assert_eq!(native.source_manifest_bytes, packed.source_manifest_bytes);
        assert_eq!(native.source_manifest_bytes, loose.source_manifest_bytes);
        assert_eq!(native.artwork, packed.artwork);
        assert_eq!(native.artwork, loose.artwork);

        let catalog =
            rebellion_data::encyclopedia_catalog::load_encyclopedia_catalog(gdata).unwrap();
        let system_pictures =
            rebellion_data::encyclopedia_topics::load_encyclopedia_system_pictures(gdata).unwrap();
        let mut fingerprints = Vec::new();
        for payload in [native, packed, loose] {
            let mut store = EncyclopediaSessionStore::default();
            store
                .replace(payload.into_session_input(catalog.clone(), system_pictures.clone()))
                .unwrap();
            let session = store.current().unwrap();
            assert_eq!(
                session.topics(EncyclopediaAudience::Alliance).entries.len(),
                346
            );
            assert_eq!(
                session.topics(EncyclopediaAudience::Empire).entries.len(),
                346
            );
            assert_eq!(
                session
                    .topics(EncyclopediaAudience::Alliance)
                    .complete_count(),
                346
            );
            assert_eq!(
                session
                    .topics(EncyclopediaAudience::Empire)
                    .complete_count(),
                346
            );
            fingerprints.push(session.logical_fingerprint().to_owned());
        }
        assert!(fingerprints.windows(2).all(|pair| pair[0] == pair[1]));
        eprintln!("owned Encyclopedia logical fingerprint={}", fingerprints[0]);
    }
}
