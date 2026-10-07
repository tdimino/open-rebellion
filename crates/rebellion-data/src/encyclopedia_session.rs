//! Immutable Galactic Encyclopedia content sessions and atomic publication.
//!
//! Loaders provide already-loaded bytes plus the source-derived index. This
//! module validates and binds a complete candidate before publishing it, so a
//! failed native or browser replacement cannot disturb the last-known-good
//! session. The store owns no filesystem, network, renderer, or campaign state.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Arc;

use anyhow::{ensure, Context, Result};
use sha2::{Digest, Sha256};

use crate::encyclopedia_catalog::EncyclopediaCatalog;
use crate::encyclopedia_overlay::{apply_encyclopedia_overlay_layer, EncyclopediaOverlayLayer};
use crate::encyclopedia_topics::{
    bind_encyclopedia_topics, encyclopedia_logical_fingerprint,
    parse_encyclopedia_source_with_manifest, EncyclopediaAudience, EncyclopediaMissingPart,
    EncyclopediaSourceCatalog, EncyclopediaSourceManifest, EncyclopediaTopicCatalog,
    ENCYCLOPEDIA_SOURCE_EMPTY_OBJECT_IDS,
};

/// Maximum retained size of one original Encyclopedia bitmap.
pub const MAX_ENCYCLOPEDIA_IMAGE_BYTES: usize = 33_554_432;
const MAX_ENCYCLOPEDIA_ARTWORK_BYTES: usize = 134_217_728;

/// Exact, already-loaded artwork bytes keyed by validated `EDATA.NNN` name.
pub type EncyclopediaResourceBytes = BTreeMap<String, Arc<[u8]>>;

/// Platform-neutral input used to prepare a complete replacement session.
///
/// Native and browser loaders intentionally converge here only after acquiring
/// bytes. Constructing this value has no global side effects.
#[derive(Debug, Clone)]
pub struct EncyclopediaSessionInput {
    pub catalog: EncyclopediaCatalog,
    pub source_catalog_bytes: Arc<[u8]>,
    pub source_manifest_bytes: Arc<[u8]>,
    pub system_pictures: HashMap<u32, u32>,
    pub artwork: EncyclopediaResourceBytes,
}

/// Integrity metadata for one retained artwork resource.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaResourceMetadata {
    byte_len: usize,
    sha256: String,
    width: u32,
    height: u32,
    bits_per_pixel: u16,
}

impl EncyclopediaResourceMetadata {
    #[must_use]
    pub const fn byte_len(&self) -> usize {
        self.byte_len
    }

    #[must_use]
    pub fn sha256(&self) -> &str {
        &self.sha256
    }

    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    #[must_use]
    pub const fn bits_per_pixel(&self) -> u16 {
        self.bits_per_pixel
    }
}

/// One complete read-only Encyclopedia content snapshot.
#[derive(Debug)]
pub struct EncyclopediaSession {
    catalog: EncyclopediaCatalog,
    source_catalog: EncyclopediaSourceCatalog,
    source_manifest: EncyclopediaSourceManifest,
    alliance_topics: EncyclopediaTopicCatalog,
    empire_topics: EncyclopediaTopicCatalog,
    artwork: EncyclopediaResourceBytes,
    resource_metadata: BTreeMap<String, EncyclopediaResourceMetadata>,
    logical_fingerprint: String,
    content_fingerprint: String,
    texture_generation: u64,
}

impl EncyclopediaSession {
    #[must_use]
    pub const fn catalog(&self) -> &EncyclopediaCatalog {
        &self.catalog
    }

    #[must_use]
    pub const fn source_catalog(&self) -> &EncyclopediaSourceCatalog {
        &self.source_catalog
    }

    #[must_use]
    pub const fn source_manifest(&self) -> &EncyclopediaSourceManifest {
        &self.source_manifest
    }

    #[must_use]
    pub const fn topics(&self, audience: EncyclopediaAudience) -> &EncyclopediaTopicCatalog {
        match audience {
            EncyclopediaAudience::Alliance => &self.alliance_topics,
            EncyclopediaAudience::Empire => &self.empire_topics,
        }
    }

    #[must_use]
    pub const fn resource_metadata(&self) -> &BTreeMap<String, EncyclopediaResourceMetadata> {
        &self.resource_metadata
    }

    #[must_use]
    pub fn artwork_bytes(&self, filename: &str) -> Option<&[u8]> {
        self.artwork.get(filename).map(AsRef::as_ref)
    }

    #[must_use]
    pub fn logical_fingerprint(&self) -> &str {
        &self.logical_fingerprint
    }

    /// Identity consumed by Encyclopedia-owned texture caches.
    #[must_use]
    pub const fn texture_generation(&self) -> u64 {
        self.texture_generation
    }
}

/// Whether a replacement published a new session or reused the active one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaInstallDisposition {
    Installed,
    Unchanged,
}

/// Application-owned publication point for immutable Encyclopedia sessions.
///
/// Existing readers retain an [`Arc`] to their complete snapshot. A candidate
/// is fully prepared before the store swaps that pointer, and any error leaves
/// both the pointer and the Encyclopedia-only texture generation unchanged.
#[derive(Debug, Default)]
pub struct EncyclopediaSessionStore {
    active: Option<Arc<EncyclopediaSession>>,
    texture_generation: u64,
}

impl EncyclopediaSessionStore {
    /// Validate, bind, and atomically publish one candidate.
    ///
    /// # Errors
    /// Returns an error for malformed source metadata, an incomplete or
    /// unexpected artwork set, empty artwork bytes, or exhausted generation
    /// identity. The active session remains untouched on every error.
    pub fn replace(
        &mut self,
        input: EncyclopediaSessionInput,
    ) -> Result<EncyclopediaInstallDisposition> {
        self.replace_with_overlays(input, Vec::new())
    }

    /// Validate the immutable base, apply ordered presentation overlays to a
    /// private candidate, then atomically publish the complete result.
    ///
    /// # Errors
    /// Returns an error without changing the active session or texture
    /// generation if any layer or complete effective session is invalid.
    pub fn replace_with_overlays(
        &mut self,
        input: EncyclopediaSessionInput,
        overlays: Vec<EncyclopediaOverlayLayer>,
    ) -> Result<EncyclopediaInstallDisposition> {
        let prepared = PreparedEncyclopediaSession::new(input, &overlays)?;
        if self
            .active
            .as_ref()
            .is_some_and(|active| active.content_fingerprint == prepared.content_fingerprint)
        {
            return Ok(EncyclopediaInstallDisposition::Unchanged);
        }

        let texture_generation = self.next_generation()?;
        let session = Arc::new(prepared.publish(texture_generation));
        self.active = Some(session);
        self.texture_generation = texture_generation;
        Ok(EncyclopediaInstallDisposition::Installed)
    }

    #[must_use]
    pub fn current(&self) -> Option<Arc<EncyclopediaSession>> {
        self.active.clone()
    }

    /// Drop the published snapshot and invalidate only Encyclopedia textures.
    ///
    /// # Errors
    /// Returns an error, without dropping the active session, if the generation
    /// identity has been exhausted.
    pub fn teardown(&mut self) -> Result<bool> {
        if self.active.is_none() {
            return Ok(false);
        }
        let texture_generation = self.next_generation()?;
        self.active = None;
        self.texture_generation = texture_generation;
        Ok(true)
    }

    #[must_use]
    pub const fn texture_generation(&self) -> u64 {
        self.texture_generation
    }

    fn next_generation(&self) -> Result<u64> {
        self.texture_generation
            .checked_add(1)
            .context("Encyclopedia texture generation exhausted")
    }
}

struct PreparedEncyclopediaSession {
    catalog: EncyclopediaCatalog,
    source_catalog: EncyclopediaSourceCatalog,
    source_manifest: EncyclopediaSourceManifest,
    alliance_topics: EncyclopediaTopicCatalog,
    empire_topics: EncyclopediaTopicCatalog,
    artwork: EncyclopediaResourceBytes,
    resource_metadata: BTreeMap<String, EncyclopediaResourceMetadata>,
    logical_fingerprint: String,
    content_fingerprint: String,
}

impl PreparedEncyclopediaSession {
    fn new(
        mut input: EncyclopediaSessionInput,
        overlays: &[EncyclopediaOverlayLayer],
    ) -> Result<Self> {
        let (mut source_catalog, source_manifest) = parse_encyclopedia_source_with_manifest(
            &input.source_catalog_bytes,
            &input.source_manifest_bytes,
        )?;
        let (mut resource_metadata, mut alliance_topics, mut empire_topics) =
            validate_effective_session(
                &input.catalog,
                &source_catalog,
                &input.system_pictures,
                &input.artwork,
            )?;
        for layer in overlays {
            apply_encyclopedia_overlay_layer(
                &mut input.catalog,
                &mut source_catalog,
                &input.system_pictures,
                &mut input.artwork,
                layer,
            )?;
            (resource_metadata, alliance_topics, empire_topics) = validate_effective_session(
                &input.catalog,
                &source_catalog,
                &input.system_pictures,
                &input.artwork,
            )
            .with_context(|| format!("validating Encyclopedia mod {:?}", layer.mod_name()))?;
        }
        let logical_fingerprint =
            combined_logical_fingerprint(&input.catalog, &alliance_topics, &empire_topics);
        let content_fingerprint = complete_content_fingerprint(
            &logical_fingerprint,
            &source_manifest,
            &resource_metadata,
        );
        Ok(Self {
            catalog: input.catalog,
            source_catalog,
            source_manifest,
            alliance_topics,
            empire_topics,
            artwork: input.artwork,
            resource_metadata,
            logical_fingerprint,
            content_fingerprint,
        })
    }

    fn publish(self, texture_generation: u64) -> EncyclopediaSession {
        EncyclopediaSession {
            catalog: self.catalog,
            source_catalog: self.source_catalog,
            source_manifest: self.source_manifest,
            alliance_topics: self.alliance_topics,
            empire_topics: self.empire_topics,
            artwork: self.artwork,
            resource_metadata: self.resource_metadata,
            logical_fingerprint: self.logical_fingerprint,
            content_fingerprint: self.content_fingerprint,
            texture_generation,
        }
    }
}

fn validate_effective_session(
    catalog: &EncyclopediaCatalog,
    source_catalog: &EncyclopediaSourceCatalog,
    system_pictures: &HashMap<u32, u32>,
    artwork: &EncyclopediaResourceBytes,
) -> Result<(
    BTreeMap<String, EncyclopediaResourceMetadata>,
    EncyclopediaTopicCatalog,
    EncyclopediaTopicCatalog,
)> {
    validate_catalog(catalog)?;
    let resource_metadata = validate_artwork(source_catalog, artwork)?;
    let alliance_topics = bind_encyclopedia_topics(
        catalog,
        source_catalog,
        system_pictures,
        EncyclopediaAudience::Alliance,
    );
    let empire_topics = bind_encyclopedia_topics(
        catalog,
        source_catalog,
        system_pictures,
        EncyclopediaAudience::Empire,
    );
    validate_topic_bindings(&alliance_topics)?;
    validate_topic_bindings(&empire_topics)?;
    Ok((resource_metadata, alliance_topics, empire_topics))
}

fn validate_catalog(catalog: &EncyclopediaCatalog) -> Result<()> {
    ensure!(
        !catalog.title.trim().is_empty() && !catalog.topic_label.trim().is_empty(),
        "Encyclopedia catalog chrome is empty"
    );
    let expected_categories = [
        (0x6f, 0x1850, None),
        (0x70, 0x1855, Some(0x90..0x98)),
        (0x71, 0x1854, Some(0x14..0x20)),
        (0x72, 0x1852, Some(0x20..0x30)),
        (0x73, 0x1851, Some(0x40..0x80)),
        (0x74, 0x1856, Some(0x10..0x14)),
        (0x75, 0x1853, Some(0x30..0x40)),
    ];
    for (category, (command_id, label_resource_id, family_range)) in
        catalog.categories.iter().zip(expected_categories)
    {
        ensure!(
            category.command_id == command_id
                && category.label_resource_id == label_resource_id
                && category.family_range == family_range,
            "invalid Encyclopedia category {:#04x}",
            category.command_id
        );
        ensure!(
            !category.label.trim().is_empty(),
            "Encyclopedia category {command_id:#04x} has an empty label"
        );
    }
    ensure!(
        !catalog.entries.is_empty(),
        "Encyclopedia catalog has no entries"
    );

    let mut object_ids = BTreeSet::new();
    for entry in &catalog.entries {
        ensure!(
            object_ids.insert(entry.object_id),
            "duplicate Encyclopedia catalog object {:#010x}",
            entry.object_id
        );
        ensure!(
            entry.text_resource_id != 0 && !entry.name.trim().is_empty(),
            "invalid Encyclopedia catalog object {:#010x}",
            entry.object_id
        );
        ensure!(
            catalog.categories[1..]
                .iter()
                .any(|category| category.contains(entry)),
            "Encyclopedia catalog object {:#010x} has an unsupported family",
            entry.object_id
        );
    }
    for entries in catalog.entries.windows(2) {
        let left = (&entries[0].name.to_lowercase(), entries[0].object_id);
        let right = (&entries[1].name.to_lowercase(), entries[1].object_id);
        ensure!(
            left <= right,
            "Encyclopedia catalog entries are not in source alphabetical order"
        );
    }
    Ok(())
}

fn validate_topic_bindings(topics: &EncyclopediaTopicCatalog) -> Result<()> {
    for topic in &topics.entries {
        if topic
            .missing
            .contains(&EncyclopediaMissingPart::SystemPicture)
        {
            anyhow::bail!(
                "Encyclopedia object {:#010x} has a missing system picture",
                topic.object_id
            );
        }
        let source_empty = ENCYCLOPEDIA_SOURCE_EMPTY_OBJECT_IDS.contains(&topic.object_id);
        if source_empty {
            ensure!(
                topic.missing.is_empty()
                    || topic.missing
                        == [
                            EncyclopediaMissingPart::Text,
                            EncyclopediaMissingPart::ArtworkMapping,
                        ],
                "source-empty Encyclopedia object {:#010x} has unexpected bindings",
                topic.object_id
            );
        } else {
            ensure!(
                topic.missing.is_empty(),
                "unapproved missing Encyclopedia binding for object {:#010x}",
                topic.object_id
            );
        }
    }
    Ok(())
}

fn validate_artwork(
    source: &EncyclopediaSourceCatalog,
    artwork: &EncyclopediaResourceBytes,
) -> Result<BTreeMap<String, EncyclopediaResourceMetadata>> {
    let expected: BTreeSet<&str> = source.artwork.values().map(String::as_str).collect();
    let supplied: BTreeSet<&str> = artwork.keys().map(String::as_str).collect();
    if let Some(filename) = expected.difference(&supplied).next() {
        anyhow::bail!("missing Encyclopedia artwork resource {filename}");
    }
    if let Some(filename) = supplied.difference(&expected).next() {
        anyhow::bail!("unexpected Encyclopedia artwork resource {filename}");
    }

    let mut metadata = BTreeMap::new();
    let mut total_bytes = 0_usize;
    for (filename, bytes) in artwork {
        ensure!(
            !bytes.is_empty(),
            "empty Encyclopedia artwork resource {filename}"
        );
        ensure!(
            bytes.len() <= MAX_ENCYCLOPEDIA_IMAGE_BYTES,
            "Encyclopedia artwork resource {filename} exceeds the byte limit"
        );
        total_bytes = total_bytes
            .checked_add(bytes.len())
            .context("Encyclopedia artwork aggregate byte length overflow")?;
        ensure!(
            total_bytes <= MAX_ENCYCLOPEDIA_ARTWORK_BYTES,
            "Encyclopedia artwork aggregate byte limit exceeded"
        );
        let bitmap = validate_bmp(bytes)
            .with_context(|| format!("invalid BMP artwork resource {filename}"))?;
        metadata.insert(
            filename.clone(),
            EncyclopediaResourceMetadata {
                byte_len: bytes.len(),
                sha256: format!("{:x}", Sha256::digest(bytes)),
                width: bitmap.width,
                height: bitmap.height,
                bits_per_pixel: bitmap.bits_per_pixel,
            },
        );
    }
    Ok(metadata)
}

#[derive(Debug, Clone, Copy)]
struct BmpMetadata {
    width: u32,
    height: u32,
    bits_per_pixel: u16,
}

fn validate_bmp(bytes: &[u8]) -> Result<BmpMetadata> {
    ensure!(bytes.len() >= 54, "BMP is too short");
    ensure!(&bytes[..2] == b"BM", "BMP signature is invalid");
    let declared_len = usize::try_from(read_u32(bytes, 2)?)
        .context("BMP declared byte length does not fit this platform")?;
    ensure!(
        declared_len == bytes.len(),
        "BMP declared byte length does not match the supplied bytes"
    );

    let dib_header_len = usize::try_from(read_u32(bytes, 14)?)
        .context("BMP DIB header length does not fit this platform")?;
    ensure!(dib_header_len >= 40, "BMP DIB header is unsupported");
    let dib_end = 14_usize
        .checked_add(dib_header_len)
        .context("BMP DIB header length overflow")?;
    ensure!(dib_end <= bytes.len(), "BMP DIB header is truncated");

    let pixel_offset = usize::try_from(read_u32(bytes, 10)?)
        .context("BMP pixel offset does not fit this platform")?;
    ensure!(
        pixel_offset >= dib_end && pixel_offset <= bytes.len(),
        "BMP pixel offset is invalid"
    );

    let signed_width = read_i32(bytes, 18)?;
    let signed_height = read_i32(bytes, 22)?;
    ensure!(
        signed_width == 400 && signed_height.unsigned_abs() == 200,
        "BMP dimensions must match the original 400x200 artwork"
    );
    let width = u32::try_from(signed_width).context("BMP width is invalid")?;
    let height = signed_height.unsigned_abs();

    ensure!(read_u16(bytes, 26)? == 1, "BMP plane count is invalid");
    let bits_per_pixel = read_u16(bytes, 28)?;
    ensure!(
        bits_per_pixel == 8,
        "BMP pixels must use the original indexed 8-bit format"
    );
    let compression = read_u32(bytes, 30)?;
    ensure!(compression == 0, "BMP compression is unsupported");
    let colors_used = read_u32(bytes, 46)?;
    ensure!(
        matches!(colors_used, 0 | 256),
        "BMP palette length is invalid"
    );
    let minimum_pixel_offset = dib_end
        .checked_add(1024)
        .context("BMP metadata byte length overflow")?;
    ensure!(
        pixel_offset >= minimum_pixel_offset,
        "BMP pixel offset precedes required metadata"
    );
    validate_uncompressed_pixels(bytes, pixel_offset, width, height)?;

    Ok(BmpMetadata {
        width,
        height,
        bits_per_pixel,
    })
}

fn validate_uncompressed_pixels(
    bytes: &[u8],
    pixel_offset: usize,
    width: u32,
    height: u32,
) -> Result<()> {
    // The source-backed 400-pixel, 8-bit rows are already DWORD aligned.
    let pixel_bytes = u64::from(width)
        .checked_mul(u64::from(height))
        .context("BMP pixel byte length overflow")?;
    let pixel_end = u64::try_from(pixel_offset)
        .expect("usize fits u64")
        .checked_add(pixel_bytes)
        .context("BMP pixel range overflow")?;
    ensure!(
        pixel_end <= u64::try_from(bytes.len()).expect("usize fits u64"),
        "BMP pixel data is truncated"
    );
    Ok(())
}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16> {
    let raw = bytes
        .get(offset..)
        .and_then(|tail| tail.get(..2))
        .context("BMP header field is truncated")?;
    Ok(u16::from_le_bytes([raw[0], raw[1]]))
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32> {
    let raw = bytes
        .get(offset..)
        .and_then(|tail| tail.get(..4))
        .context("BMP header field is truncated")?;
    Ok(u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]))
}

fn read_i32(bytes: &[u8], offset: usize) -> Result<i32> {
    let raw = bytes
        .get(offset..)
        .and_then(|tail| tail.get(..4))
        .context("BMP header field is truncated")?;
    Ok(i32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]))
}

fn combined_logical_fingerprint(
    catalog: &EncyclopediaCatalog,
    alliance: &EncyclopediaTopicCatalog,
    empire: &EncyclopediaTopicCatalog,
) -> String {
    let alliance = encyclopedia_logical_fingerprint(catalog, alliance);
    let empire = encyclopedia_logical_fingerprint(catalog, empire);
    let mut digest = Sha256::new();
    digest.update(b"open-rebellion:encyclopedia-session-logical:v1\0");
    digest.update(alliance.as_bytes());
    digest.update(empire.as_bytes());
    format!("{:x}", digest.finalize())
}

fn complete_content_fingerprint(
    logical_fingerprint: &str,
    manifest: &EncyclopediaSourceManifest,
    resources: &BTreeMap<String, EncyclopediaResourceMetadata>,
) -> String {
    let mut digest = Sha256::new();
    digest.update(b"open-rebellion:encyclopedia-session-content:v1\0");
    digest.update(logical_fingerprint.as_bytes());
    digest.update(manifest.catalog_sha256.as_bytes());
    digest.update(manifest.source_files.encytext_sha256.as_bytes());
    digest.update(manifest.source_files.encybmap_sha256.as_bytes());
    // The manifest catalog digest already commits to the exact validated
    // filename set. BTreeMap order then binds each metadata record to that set.
    for metadata in resources.values() {
        digest.update(
            u64::try_from(metadata.byte_len)
                .expect("Encyclopedia resource length fits u64")
                .to_le_bytes(),
        );
        digest.update(metadata.sha256.as_bytes());
    }
    format!("{:x}", digest.finalize())
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};
    use std::sync::Arc;

    use rebellion_core::world::GameWorld;

    use super::*;
    use crate::encyclopedia_catalog::{
        EncyclopediaCatalog, EncyclopediaCatalogEntry, EncyclopediaCategory,
    };
    use crate::encyclopedia_overlay::{parse_encyclopedia_overlay, EncyclopediaOverlayLayer};
    use crate::encyclopedia_topics::EncyclopediaAudience;

    const SHARED_SOURCE: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/p66a/source.json");
    const SHARED_MANIFEST: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/p66a/source.json.manifest.json");

    fn fixture_index() -> EncyclopediaCatalog {
        let category = |command_id, label_resource_id, family_range| EncyclopediaCategory {
            command_id,
            label_resource_id,
            label: format!("Category {command_id}"),
            family_range,
        };
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
                    object_id: 0x9200_0064,
                    text_resource_id: 0x2e00,
                    name: "Abregado-rae".into(),
                },
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

    fn bmp(pixel: [u8; 3]) -> Arc<[u8]> {
        const WIDTH: u32 = 400;
        const HEIGHT: u32 = 200;
        const PIXEL_OFFSET: u32 = 54;
        const PIXEL_BYTES: u32 = WIDTH * HEIGHT * 3;
        const FILE_BYTES: u32 = PIXEL_OFFSET + PIXEL_BYTES;
        let mut bytes = Vec::with_capacity(FILE_BYTES as usize);
        bytes.extend_from_slice(b"BM");
        bytes.extend_from_slice(&FILE_BYTES.to_le_bytes());
        bytes.extend_from_slice(&[0; 4]);
        bytes.extend_from_slice(&PIXEL_OFFSET.to_le_bytes());
        bytes.extend_from_slice(&40_u32.to_le_bytes());
        bytes.extend_from_slice(&(WIDTH as i32).to_le_bytes());
        bytes.extend_from_slice(&(HEIGHT as i32).to_le_bytes());
        bytes.extend_from_slice(&1_u16.to_le_bytes());
        bytes.extend_from_slice(&24_u16.to_le_bytes());
        bytes.extend_from_slice(&0_u32.to_le_bytes());
        bytes.extend_from_slice(&PIXEL_BYTES.to_le_bytes());
        bytes.extend_from_slice(&[0; 16]);
        for _ in 0..WIDTH * HEIGHT {
            bytes.extend_from_slice(&[pixel[2], pixel[1], pixel[0]]);
        }
        bytes.into()
    }

    fn indexed_bmp(pixel: [u8; 3]) -> Arc<[u8]> {
        const WIDTH: u32 = 400;
        const HEIGHT: u32 = 200;
        const PIXEL_OFFSET: u32 = 14 + 40 + 1024;
        const PIXEL_BYTES: u32 = WIDTH * HEIGHT;
        const FILE_BYTES: u32 = PIXEL_OFFSET + PIXEL_BYTES;
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
        bytes.extend_from_slice(&PIXEL_BYTES.to_le_bytes());
        bytes.extend_from_slice(&[0; 8]);
        bytes.extend_from_slice(&256_u32.to_le_bytes());
        bytes.extend_from_slice(&0_u32.to_le_bytes());
        bytes.resize(PIXEL_OFFSET as usize, 0);
        bytes[58..62].copy_from_slice(&[pixel[2], pixel[1], pixel[0], 0]);
        bytes.resize(FILE_BYTES as usize, 1);
        bytes.into()
    }

    fn indexed_resources() -> EncyclopediaResourceBytes {
        BTreeMap::from([
            ("EDATA.001".into(), indexed_bmp([1, 2, 3])),
            ("EDATA.014".into(), indexed_bmp([14, 15, 16])),
            ("EDATA.015".into(), indexed_bmp([15, 16, 17])),
            ("EDATA.115".into(), indexed_bmp([115, 116, 117])),
        ])
    }

    fn resources() -> EncyclopediaResourceBytes {
        indexed_resources()
    }

    fn bmp_with_len(byte_len: usize) -> Arc<[u8]> {
        let mut bytes = indexed_bmp([1, 2, 3]).to_vec();
        bytes.resize(byte_len, 0);
        bytes[2..6].copy_from_slice(
            &u32::try_from(byte_len)
                .expect("test BMP length fits u32")
                .to_le_bytes(),
        );
        bytes.into()
    }

    fn input() -> EncyclopediaSessionInput {
        EncyclopediaSessionInput {
            catalog: fixture_index(),
            source_catalog_bytes: Arc::from(SHARED_SOURCE),
            source_manifest_bytes: Arc::from(SHARED_MANIFEST),
            system_pictures: HashMap::from([(0x9200_0064, 1)]),
            artwork: resources(),
        }
    }

    fn overlay_layer(
        name: &str,
        json: &[u8],
        images: &[(&str, Arc<[u8]>)],
    ) -> EncyclopediaOverlayLayer {
        EncyclopediaOverlayLayer::new(
            name.to_owned(),
            parse_encyclopedia_overlay(json).unwrap(),
            images
                .iter()
                .map(|(path, bytes)| ((*path).to_owned(), Arc::clone(bytes)))
                .collect(),
        )
        .unwrap()
    }

    #[test]
    fn valid_input_publishes_one_complete_immutable_session() {
        let mut store = EncyclopediaSessionStore::default();

        assert_eq!(
            store.replace(input()).unwrap(),
            EncyclopediaInstallDisposition::Installed
        );
        let session = store.current().unwrap();

        assert_eq!(session.texture_generation(), 1);
        assert_eq!(session.catalog().entries.len(), 3);
        assert_eq!(session.source_manifest().counts.texts, 3);
        assert_eq!(
            session
                .topics(EncyclopediaAudience::Alliance)
                .complete_count(),
            3
        );
        assert_eq!(
            session
                .topics(EncyclopediaAudience::Empire)
                .complete_count(),
            3
        );
        assert_eq!(session.resource_metadata().len(), 4);
        assert_eq!(session.resource_metadata()["EDATA.014"].byte_len(), 81_078);
        assert_eq!(
            session.resource_metadata()["EDATA.014"].sha256(),
            "b023679e1dffd68c2f880a81ace88a13e84c032bd5e85f1c563673160d0186e9"
        );
        assert_eq!(session.resource_metadata()["EDATA.014"].width(), 400);
        assert_eq!(session.resource_metadata()["EDATA.014"].height(), 200);
        assert_eq!(session.resource_metadata()["EDATA.014"].bits_per_pixel(), 8);
        assert_eq!(
            session.artwork_bytes("EDATA.014").unwrap(),
            &*indexed_bmp([14, 15, 16])
        );
        assert_eq!(session.logical_fingerprint().len(), 64);
        assert_eq!(
            session.logical_fingerprint(),
            "dc504009a224e8a985b13710b2c0afb34e01bc50cd25db7278543019e7a4bb64"
        );
        assert!(session
            .logical_fingerprint()
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)));
    }

    #[test]
    fn invalid_replacement_preserves_the_last_known_good_session() {
        let mut store = EncyclopediaSessionStore::default();
        store.replace(input()).unwrap();
        let previous = store.current().unwrap();
        let previous_generation = store.texture_generation();
        let mut invalid = input();
        let mut tampered = invalid.source_catalog_bytes.to_vec();
        tampered.push(b' ');
        invalid.source_catalog_bytes = tampered.into();

        assert!(store.replace(invalid).is_err());

        let retained = store.current().unwrap();
        assert!(Arc::ptr_eq(&previous, &retained));
        assert_eq!(store.texture_generation(), previous_generation);
        assert_eq!(
            retained.artwork_bytes("EDATA.014").unwrap(),
            &*indexed_bmp([14, 15, 16])
        );
    }

    #[test]
    fn repeat_replace_and_teardown_have_deterministic_cache_generations() {
        let mut store = EncyclopediaSessionStore::default();
        store.replace(input()).unwrap();
        let first = store.current().unwrap();

        assert_eq!(
            store.replace(input()).unwrap(),
            EncyclopediaInstallDisposition::Unchanged
        );
        assert!(Arc::ptr_eq(&first, &store.current().unwrap()));
        assert_eq!(store.texture_generation(), 1);

        let mut replacement = input();
        replacement
            .artwork
            .insert("EDATA.014".into(), indexed_bmp([99, 100, 101]));
        assert_eq!(
            store.replace(replacement).unwrap(),
            EncyclopediaInstallDisposition::Installed
        );
        let second = store.current().unwrap();
        assert_eq!(second.texture_generation(), 2);
        assert_eq!(
            second.artwork_bytes("EDATA.014").unwrap(),
            &*indexed_bmp([99, 100, 101])
        );
        assert_eq!(
            first.artwork_bytes("EDATA.014").unwrap(),
            &*indexed_bmp([14, 15, 16])
        );

        assert!(store.teardown().unwrap());
        assert!(store.current().is_none());
        assert_eq!(store.texture_generation(), 3);
        assert!(!store.teardown().unwrap());
        assert_eq!(store.texture_generation(), 3);

        store.replace(input()).unwrap();
        assert_eq!(store.current().unwrap().texture_generation(), 4);
    }

    #[test]
    fn resource_set_must_exactly_match_validated_source_metadata() {
        for (label, mutate) in [
            (
                "missing",
                Box::new(|artwork: &mut EncyclopediaResourceBytes| {
                    artwork.remove("EDATA.014");
                }) as Box<dyn Fn(&mut EncyclopediaResourceBytes)>,
            ),
            (
                "unexpected",
                Box::new(|artwork: &mut EncyclopediaResourceBytes| {
                    artwork.insert("EDATA.999".into(), bmp([9, 9, 9]));
                }),
            ),
            (
                "empty",
                Box::new(|artwork: &mut EncyclopediaResourceBytes| {
                    artwork.insert("EDATA.014".into(), Arc::from(&b""[..]));
                }),
            ),
        ] {
            let mut invalid = input();
            mutate(&mut invalid.artwork);
            let error = EncyclopediaSessionStore::default()
                .replace(invalid)
                .unwrap_err();
            assert!(error.to_string().contains(label), "{error:#}");
        }
    }

    #[test]
    fn malformed_catalogs_and_bindings_cannot_replace_the_valid_session() {
        let cases = [
            (
                "duplicate",
                Box::new(|input: &mut EncyclopediaSessionInput| {
                    input.catalog.entries.push(input.catalog.entries[2].clone());
                }) as Box<dyn Fn(&mut EncyclopediaSessionInput)>,
            ),
            (
                "unapproved missing",
                Box::new(|input: &mut EncyclopediaSessionInput| {
                    input.catalog.entries[1].text_resource_id = 0x2fff;
                }),
            ),
            (
                "system picture",
                Box::new(|input: &mut EncyclopediaSessionInput| {
                    input.system_pictures.clear();
                }),
            ),
        ];
        for (label, mutate) in cases {
            let mut store = EncyclopediaSessionStore::default();
            store.replace(input()).unwrap();
            let previous = store.current().unwrap();
            let mut invalid = input();
            mutate(&mut invalid);

            let error = store.replace(invalid).unwrap_err();

            assert!(error.to_string().contains(label), "{error:#}");
            assert!(Arc::ptr_eq(&previous, &store.current().unwrap()));
            assert_eq!(store.texture_generation(), 1);
        }
    }

    #[test]
    fn only_the_source_proven_empty_mission_identities_may_remain_unbound() {
        let mut valid = input();
        valid.catalog.entries.push(EncyclopediaCatalogEntry {
            object_id: 0x7200_0045,
            text_resource_id: 0x2450,
            name: "Vacation".into(),
        });

        let mut store = EncyclopediaSessionStore::default();
        store.replace(valid).unwrap();

        for audience in [EncyclopediaAudience::Alliance, EncyclopediaAudience::Empire] {
            let topics = store.current().unwrap();
            let vacation = topics.topics(audience).topic(0x7200_0045).unwrap();
            assert_eq!(
                vacation.missing,
                [
                    crate::encyclopedia_topics::EncyclopediaMissingPart::Text,
                    crate::encyclopedia_topics::EncyclopediaMissingPart::ArtworkMapping,
                ]
            );
        }
    }

    #[test]
    fn corrupt_and_oversized_artwork_cannot_replace_the_valid_session() {
        let mut store = EncyclopediaSessionStore::default();
        store.replace(input()).unwrap();
        let previous = store.current().unwrap();

        let mut corrupt = input();
        corrupt
            .artwork
            .insert("EDATA.014".into(), Arc::from(&b"not-a-bmp"[..]));
        assert!(store
            .replace(corrupt)
            .unwrap_err()
            .to_string()
            .contains("BMP"));
        assert!(Arc::ptr_eq(&previous, &store.current().unwrap()));

        let mut truncated = input();
        let mut truncated_bitmap = indexed_bmp([14, 15, 16]).to_vec();
        truncated_bitmap.pop();
        let truncated_len = truncated_bitmap.len();
        truncated_bitmap[2..6].copy_from_slice(
            &u32::try_from(truncated_len)
                .expect("test BMP length fits u32")
                .to_le_bytes(),
        );
        truncated
            .artwork
            .insert("EDATA.014".into(), truncated_bitmap.into());
        let error = store.replace(truncated).unwrap_err();
        assert!(format!("{error:#}").contains("truncated"), "{error:#}");
        assert!(Arc::ptr_eq(&previous, &store.current().unwrap()));

        let mut oversized = input();
        oversized.artwork.insert(
            "EDATA.014".into(),
            vec![0_u8; MAX_ENCYCLOPEDIA_IMAGE_BYTES + 1].into(),
        );
        assert!(store
            .replace(oversized)
            .unwrap_err()
            .to_string()
            .contains("byte limit"));
        assert!(Arc::ptr_eq(&previous, &store.current().unwrap()));
    }

    #[test]
    fn nonindexed_and_compressed_bitmaps_fail_closed() {
        let mut compressed = indexed_bmp([14, 15, 16]).to_vec();
        compressed[30..34].copy_from_slice(&1_u32.to_le_bytes());
        let cases = [
            ("indexed", bmp([14, 15, 16])),
            ("compression", compressed.into()),
        ];

        for (label, invalid_bitmap) in cases {
            let mut invalid = input();
            invalid.artwork = indexed_resources();
            invalid.artwork.insert("EDATA.014".into(), invalid_bitmap);

            let error = EncyclopediaSessionStore::default()
                .replace(invalid)
                .unwrap_err();

            assert!(format!("{error:#}").contains(label), "{error:#}");
        }
    }

    #[test]
    fn aggregate_artwork_limit_rejects_many_individually_bounded_images() {
        let mut source: serde_json::Value = serde_json::from_slice(SHARED_SOURCE).unwrap();
        source["artwork"]["2"] = serde_json::Value::String("EDATA.002".into());
        let source_bytes = serde_json::to_vec(&source).unwrap();
        let mut manifest: serde_json::Value = serde_json::from_slice(SHARED_MANIFEST).unwrap();
        manifest["catalog_sha256"] =
            serde_json::Value::String(format!("{:x}", Sha256::digest(&source_bytes)));
        manifest["counts"]["artwork_mappings"] = serde_json::Value::from(5);

        let shared = bmp_with_len(MAX_ENCYCLOPEDIA_IMAGE_BYTES);
        let mut oversized = input();
        oversized.source_catalog_bytes = source_bytes.into();
        oversized.source_manifest_bytes = serde_json::to_vec(&manifest).unwrap().into();
        oversized.artwork = BTreeMap::from([
            ("EDATA.001".into(), Arc::clone(&shared)),
            ("EDATA.002".into(), Arc::clone(&shared)),
            ("EDATA.014".into(), Arc::clone(&shared)),
            ("EDATA.015".into(), Arc::clone(&shared)),
            ("EDATA.115".into(), shared),
        ]);

        let error = EncyclopediaSessionStore::default()
            .replace(oversized)
            .unwrap_err();

        assert!(
            error.to_string().contains("aggregate byte limit"),
            "{error:#}"
        );
    }

    #[test]
    fn generation_exhaustion_preserves_the_active_session() {
        let mut store = EncyclopediaSessionStore::default();
        store.replace(input()).unwrap();
        let previous = store.current().unwrap();
        store.texture_generation = u64::MAX;
        let mut replacement = input();
        replacement
            .artwork
            .insert("EDATA.014".into(), indexed_bmp([99, 100, 101]));

        assert!(store.replace(replacement).is_err());
        assert!(Arc::ptr_eq(&previous, &store.current().unwrap()));
        assert_eq!(store.texture_generation(), u64::MAX);
        assert!(store.teardown().is_err());
        assert!(Arc::ptr_eq(&previous, &store.current().unwrap()));
        assert_eq!(store.texture_generation(), u64::MAX);
    }

    #[test]
    fn independent_installations_match_without_touching_world_state() {
        let world = GameWorld::default();
        let world_before = bincode::serialize(&world).unwrap();
        let mut native = EncyclopediaSessionStore::default();
        let mut browser = EncyclopediaSessionStore::default();

        native.replace(input()).unwrap();
        browser.replace(input()).unwrap();

        let native = native.current().unwrap();
        let browser = browser.current().unwrap();
        assert_eq!(native.logical_fingerprint(), browser.logical_fingerprint());
        for audience in [EncyclopediaAudience::Alliance, EncyclopediaAudience::Empire] {
            assert_eq!(
                native.topics(audience).entries.len(),
                browser.topics(audience).entries.len()
            );
            assert_eq!(
                native.topics(audience).complete_count(),
                browser.topics(audience).complete_count()
            );
        }
        assert_eq!(bincode::serialize(&world).unwrap(), world_before);
    }

    #[test]
    fn overlays_inherit_absent_fields_and_later_layers_win_in_supplied_order() {
        let world = GameWorld::default();
        let world_before = bincode::serialize(&world).unwrap();
        let replacement = indexed_bmp([90, 91, 92]);
        let first = overlay_layer(
            "alpha",
            br#"[{"id":335544384,"action":"replace","title":"Alpha Cruiser","body":"Alpha body","image":{"path":"encyclopedia/assets/alpha.bmp"}}]"#,
            &[("encyclopedia/assets/alpha.bmp", Arc::clone(&replacement))],
        );
        let second = overlay_layer("zulu", br#"[{"id":335544384,"title":"Zulu Cruiser"}]"#, &[]);
        let mut store = EncyclopediaSessionStore::default();
        store.replace(input()).unwrap();
        let base_lease = store.current().unwrap();

        store
            .replace_with_overlays(input(), vec![first, second])
            .unwrap();

        let session = store.current().unwrap();
        assert_eq!(session.texture_generation(), 2);
        assert_eq!(
            base_lease.artwork_bytes("EDATA.014"),
            Some(indexed_bmp([14, 15, 16]).as_ref())
        );
        let topic = session
            .topics(EncyclopediaAudience::Alliance)
            .topic(0x1400_0040)
            .unwrap();
        assert_eq!(topic.title, "Zulu Cruiser");
        assert_eq!(topic.body.as_deref(), Some("Alpha body"));
        assert_eq!(
            topic.artwork_filename.as_deref(),
            Some("mod:v1:alpha:encyclopedia/assets/alpha.bmp")
        );
        assert_eq!(
            session.artwork_bytes(topic.artwork_filename.as_deref().unwrap()),
            Some(replacement.as_ref())
        );
        assert_eq!(bincode::serialize(&world).unwrap(), world_before);
    }

    #[test]
    fn whole_topic_add_then_remove_restores_the_exact_base_snapshot() {
        let mut store = EncyclopediaSessionStore::default();
        store.replace(input()).unwrap();
        let base_fingerprint = store.current().unwrap().logical_fingerprint().to_owned();
        let addition = overlay_layer(
            "addition",
            br#"[{"id":335544385,"action":"add","text_resource_id":10049,"title":"Nebulon Escort","body":"Added topic","image":{"path":"encyclopedia/assets/added.bmp"}}]"#,
            &[("encyclopedia/assets/added.bmp", indexed_bmp([31, 32, 33]))],
        );

        store
            .replace_with_overlays(input(), vec![addition])
            .unwrap();
        assert!(store
            .current()
            .unwrap()
            .catalog()
            .entries
            .iter()
            .any(|entry| entry.object_id == 0x1400_0041));

        store.replace(input()).unwrap();
        assert_eq!(
            store.current().unwrap().logical_fingerprint(),
            base_fingerprint,
            "disabling all overlay layers rebuilds the immutable base"
        );

        let remove = overlay_layer("removal", br#"[{"id":335544385,"action":"remove"}]"#, &[]);
        let mut restored_store = EncyclopediaSessionStore::default();
        restored_store
            .replace_with_overlays(input(), vec![
                overlay_layer(
                    "addition",
                    br#"[{"id":335544385,"action":"add","text_resource_id":10049,"title":"Nebulon Escort","body":"Added topic","image":{"path":"encyclopedia/assets/added.bmp"}}]"#,
                    &[("encyclopedia/assets/added.bmp", indexed_bmp([31, 32, 33]))],
                ),
                remove,
            ])
            .unwrap();

        let restored = restored_store.current().unwrap();
        assert_eq!(restored.logical_fingerprint(), base_fingerprint);
        assert_eq!(restored.resource_metadata().len(), 4);
        assert!(!restored.source_catalog().texts.contains_key(&0x1741));
        assert!(!restored.source_catalog().artwork.contains_key(&0x1741));
        assert!(restored
            .resource_metadata()
            .keys()
            .all(|identity| !identity.starts_with("mod:v1:")));
    }

    #[test]
    fn removing_one_topic_retains_source_records_shared_by_another_topic() {
        let mut base = input();
        base.catalog
            .entries
            .retain(|entry| entry.object_id == 0x1400_0040);
        base.catalog.entries.push(EncyclopediaCatalogEntry {
            object_id: 0x1400_0041,
            text_resource_id: 0x2740,
            name: "Twin Cruiser".into(),
        });
        base.catalog.entries.sort_by(|left, right| {
            left.name
                .to_lowercase()
                .cmp(&right.name.to_lowercase())
                .then(left.object_id.cmp(&right.object_id))
        });
        let remove = overlay_layer(
            "remove-one-twin",
            br#"[{"id":335544384,"action":"remove"}]"#,
            &[],
        );
        let mut store = EncyclopediaSessionStore::default();

        store.replace_with_overlays(base, vec![remove]).unwrap();

        let session = store.current().unwrap();
        assert!(session
            .catalog()
            .entries
            .iter()
            .all(|entry| entry.object_id != 0x1400_0040));
        assert!(session.source_catalog().texts.contains_key(&0x1740));
        assert!(session.source_catalog().artwork.contains_key(&0x1740));
        for audience in [EncyclopediaAudience::Alliance, EncyclopediaAudience::Empire] {
            assert!(session
                .topics(audience)
                .topic(0x1400_0041)
                .unwrap()
                .is_complete());
        }
    }

    #[test]
    fn explicit_null_legally_restores_a_source_empty_topic_after_a_complete_overlay() {
        let mut base = input();
        base.catalog.entries.push(EncyclopediaCatalogEntry {
            object_id: 0x7200_0045,
            text_resource_id: 0x2450,
            name: "Vacation".into(),
        });
        base.catalog.entries.sort_by(|left, right| {
            left.name
                .to_lowercase()
                .cmp(&right.name.to_lowercase())
                .then(left.object_id.cmp(&right.object_id))
        });
        let mut base_store = EncyclopediaSessionStore::default();
        base_store.replace(base.clone()).unwrap();
        let base_fingerprint = base_store
            .current()
            .unwrap()
            .logical_fingerprint()
            .to_owned();
        let fill = overlay_layer(
            "fill-empty",
            br#"[{"id":1912602693,"body":"Now documented","image":{"path":"encyclopedia/assets/vacation.bmp"}}]"#,
            &[("encyclopedia/assets/vacation.bmp", indexed_bmp([71, 72, 73]))],
        );
        let remove = overlay_layer(
            "restore-empty",
            br#"[{"id":1912602693,"body":null,"image":null}]"#,
            &[],
        );
        let mut store = EncyclopediaSessionStore::default();

        store
            .replace_with_overlays(base, vec![fill, remove])
            .unwrap();

        let restored = store.current().unwrap();
        assert_eq!(restored.logical_fingerprint(), base_fingerprint);
        for audience in [EncyclopediaAudience::Alliance, EncyclopediaAudience::Empire] {
            let topic = restored.topics(audience).topic(0x7200_0045).unwrap();
            assert_eq!(topic.body, None);
            assert_eq!(topic.artwork_filename, None);
        }
    }

    #[test]
    fn invalid_overlay_candidate_preserves_the_published_arc_and_generation() {
        let mut store = EncyclopediaSessionStore::default();
        store.replace(input()).unwrap();
        let previous = store.current().unwrap();
        let invalid = overlay_layer("invalid", br#"[{"id":335544384,"body":null}]"#, &[]);

        assert!(store.replace_with_overlays(input(), vec![invalid]).is_err());
        assert!(Arc::ptr_eq(&previous, &store.current().unwrap()));
        assert_eq!(store.texture_generation(), 1);
    }

    #[test]
    fn existing_topic_may_restate_but_not_change_its_text_resource_identity() {
        let mut store = EncyclopediaSessionStore::default();
        store.replace(input()).unwrap();

        let disposition = store
            .replace_with_overlays(
                input(),
                vec![overlay_layer(
                    "same-identity",
                    br#"[{"id":335544384,"text_resource_id":10048}]"#,
                    &[],
                )],
            )
            .unwrap();
        assert_eq!(disposition, EncyclopediaInstallDisposition::Unchanged);

        let previous = store.current().unwrap();
        let generation = store.texture_generation();
        let changed = overlay_layer(
            "changed-identity",
            br#"[{"id":335544384,"text_resource_id":10049}]"#,
            &[],
        );
        assert!(store.replace_with_overlays(input(), vec![changed]).is_err());
        assert!(Arc::ptr_eq(&previous, &store.current().unwrap()));
        assert_eq!(store.texture_generation(), generation);
    }
}
