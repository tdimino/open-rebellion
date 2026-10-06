//! Source-derived Galactic Encyclopedia topic bindings.
//!
//! The original executable resolves an Encyclopedia object through a stable
//! resource identity, then loads prose from `ENCYTEXT.DLL` and an `EDATA.NNN`
//! filename from `ENCYBMAP.DLL`. This module joins the staged, ignored source
//! catalog to the immutable index catalog without putting presentation data in
//! campaign saves or inventing a fallback topic.

use std::collections::HashMap;
use std::marker::PhantomData;
use std::path::Path;

use anyhow::{ensure, Context, Result};
use dat_dumper::types::systems::SystemsFile;
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use sha2::{Digest, Sha256};

use crate::encyclopedia_catalog::{
    compound_object_id, EncyclopediaCatalog, EncyclopediaCatalogEntry,
};
use crate::read_dat_file;

const SOURCE_SCHEMA_VERSION: u32 = 1;
const ENGLISH_LANGUAGE_ID: u32 = 1033;
const ENGLISH_ENCODING: &str = "windows-1252";
const SOURCE_JSON_BYTES_LIMIT: usize = 64 * 1024 * 1024;
const SOURCE_MANIFEST_JSON_BYTES_LIMIT: usize = 32 * 1024 * 1024;
const SOURCE_RESOURCE_LIMIT: usize = 10_000;
const SOURCE_BODY_BYTES_LIMIT: usize = 1_048_576;

/// Faction-specific Encyclopedia artwork profile used by mission objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaAudience {
    Alliance,
    Empire,
}

/// One decoded `ENCYTEXT.DLL` resource from the local staging tool.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EncyclopediaSourceText {
    pub body: String,
    pub body_sha256: String,
}

/// Ignored local source data emitted by `stage-ui-assets --encyclopedia-only`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EncyclopediaSourceCatalog {
    pub schema_version: u32,
    pub language_id: u32,
    pub encoding: String,
    pub source_code_page: u32,
    #[serde(deserialize_with = "deserialize_unique_u16_map")]
    pub texts: HashMap<u16, EncyclopediaSourceText>,
    #[serde(deserialize_with = "deserialize_unique_u16_map")]
    pub artwork: HashMap<u16, String>,
}

fn deserialize_unique_u16_map<'de, D, V>(deserializer: D) -> Result<HashMap<u16, V>, D::Error>
where
    D: Deserializer<'de>,
    V: Deserialize<'de>,
{
    struct UniqueMapVisitor<V>(PhantomData<V>);

    impl<'de, V> Visitor<'de> for UniqueMapVisitor<V>
    where
        V: Deserialize<'de>,
    {
        type Value = HashMap<u16, V>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("an object with unique u16 resource identities")
        }

        fn visit_map<A>(self, mut entries: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut values = HashMap::with_capacity(entries.size_hint().unwrap_or(0));
            while let Some((key, value)) = entries.next_entry::<u16, V>()? {
                if values.insert(key, value).is_some() {
                    return Err(de::Error::custom(format!(
                        "duplicate Encyclopedia resource identity {key}"
                    )));
                }
            }
            Ok(values)
        }
    }

    deserializer.deserialize_map(UniqueMapVisitor(PhantomData))
}

/// Source-DLL identities retained by the local extraction manifest.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EncyclopediaSourceFiles {
    pub encytext_sha256: String,
    pub encybmap_sha256: String,
}

/// Source inventory retained by the local extraction manifest.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EncyclopediaSourceCounts {
    pub texts: usize,
    pub artwork_mappings: usize,
}

/// Integrity sidecar emitted with the ignored source catalog.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EncyclopediaSourceManifest {
    pub schema_version: u32,
    pub catalog_sha256: String,
    pub source_files: EncyclopediaSourceFiles,
    pub counts: EncyclopediaSourceCounts,
}

/// Exact reason a source topic cannot yet be composed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaMissingPart {
    Text,
    ArtworkMapping,
    SystemPicture,
}

/// Source-proven mission objects that have neither topic prose nor artwork.
///
/// The owned English P66A audit establishes this exact set for both factions.
/// Keeping the identities here gives runtime validation and the source audit a
/// single authority without inventing fallback content.
pub const ENCYCLOPEDIA_SOURCE_EMPTY_OBJECT_IDS: [u32; 10] = [
    0x4100_0001,
    0x4200_0002,
    0x4300_0003,
    0x4400_0004,
    0x6400_0044,
    0x6500_0083,
    0x7100_0043,
    0x7200_0045,
    0x7200_0046,
    0x7300_0082,
];

/// One index object joined to its source topic resources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaTopicBinding {
    pub object_id: u32,
    pub title: String,
    /// `ENCYTEXT.DLL` type-10 resource ID.
    pub topic_text_resource_id: u16,
    /// `ENCYBMAP.DLL` logical string ID, when the object has a proven image key.
    pub artwork_resource_id: Option<u16>,
    pub body: Option<String>,
    pub artwork_filename: Option<String>,
    pub missing: Vec<EncyclopediaMissingPart>,
}

impl EncyclopediaTopicBinding {
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.missing.is_empty()
    }
}

/// Complete, deterministic topic-binding view for one audience.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaTopicCatalog {
    pub language_id: u32,
    pub audience: EncyclopediaAudience,
    pub entries: Vec<EncyclopediaTopicBinding>,
}

impl EncyclopediaTopicCatalog {
    #[must_use]
    pub fn topic(&self, object_id: u32) -> Option<&EncyclopediaTopicBinding> {
        self.entries
            .iter()
            .find(|entry| entry.object_id == object_id)
    }

    #[must_use]
    pub fn complete_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.is_complete())
            .count()
    }
}

/// Return a versioned, platform-neutral digest of the complete logical view.
///
/// The digest covers index chrome, categories, display order, both source and
/// compound identities, bound text and artwork, audience, and explicit missing
/// parts. It uses explicit little-endian integers, length-prefixed UTF-8, and
/// stable enum tags, so it does not depend on map iteration, filesystem paths,
/// native object layout, or presentation cache state.
#[must_use]
pub fn encyclopedia_logical_fingerprint(
    index: &EncyclopediaCatalog,
    topics: &EncyclopediaTopicCatalog,
) -> String {
    let mut digest = Sha256::new();
    digest.update(b"open-rebellion:encyclopedia-logical-catalog:v1\0");
    hash_bytes(&mut digest, index.title.as_bytes());
    hash_bytes(&mut digest, index.topic_label.as_bytes());
    hash_length(&mut digest, index.categories.len());
    for category in &index.categories {
        digest.update(category.command_id.to_le_bytes());
        digest.update(category.label_resource_id.to_le_bytes());
        hash_bytes(&mut digest, category.label.as_bytes());
        match &category.family_range {
            Some(range) => {
                digest.update([1, range.start, range.end]);
            }
            None => {
                digest.update([0]);
            }
        }
    }
    hash_length(&mut digest, index.entries.len());
    for entry in &index.entries {
        digest.update(entry.object_id.to_le_bytes());
        digest.update(entry.text_resource_id.to_le_bytes());
        hash_bytes(&mut digest, entry.name.as_bytes());
    }

    digest.update(topics.language_id.to_le_bytes());
    digest.update([match topics.audience {
        EncyclopediaAudience::Alliance => 0,
        EncyclopediaAudience::Empire => 1,
    }]);
    hash_length(&mut digest, topics.entries.len());
    for entry in &topics.entries {
        digest.update(entry.object_id.to_le_bytes());
        hash_bytes(&mut digest, entry.title.as_bytes());
        digest.update(entry.topic_text_resource_id.to_le_bytes());
        match entry.artwork_resource_id {
            Some(resource_id) => {
                digest.update([1]);
                digest.update(resource_id.to_le_bytes());
            }
            None => {
                digest.update([0]);
            }
        }
        hash_optional_bytes(&mut digest, entry.body.as_deref().map(str::as_bytes));
        hash_optional_bytes(
            &mut digest,
            entry.artwork_filename.as_deref().map(str::as_bytes),
        );
        hash_length(&mut digest, entry.missing.len());
        for missing in &entry.missing {
            digest.update([match missing {
                EncyclopediaMissingPart::Text => 0,
                EncyclopediaMissingPart::ArtworkMapping => 1,
                EncyclopediaMissingPart::SystemPicture => 2,
            }]);
        }
    }
    format!("{:x}", digest.finalize())
}

fn hash_length(digest: &mut Sha256, length: usize) {
    let length = u64::try_from(length).expect("Encyclopedia length fits u64");
    digest.update(length.to_le_bytes());
}

fn hash_bytes(digest: &mut Sha256, bytes: &[u8]) {
    hash_length(digest, bytes.len());
    digest.update(bytes);
}

fn hash_optional_bytes(digest: &mut Sha256, bytes: Option<&[u8]>) {
    match bytes {
        Some(bytes) => {
            digest.update([1]);
            hash_bytes(digest, bytes);
        }
        None => {
            digest.update([0]);
        }
    }
}

/// Parse and validate an extracted source catalog.
///
/// # Errors
/// Returns an error for an unsupported source profile, malformed identity, or
/// invalid checksums or filename metadata. Missing object-level bindings are
/// not a parse error; they remain explicit in
/// [`EncyclopediaTopicBinding::missing`].
pub fn parse_encyclopedia_source(bytes: &[u8]) -> Result<EncyclopediaSourceCatalog> {
    ensure!(
        bytes.len() <= SOURCE_JSON_BYTES_LIMIT,
        "Encyclopedia source JSON byte limit exceeded"
    );
    let source: EncyclopediaSourceCatalog =
        serde_json::from_slice(bytes).context("parsing Encyclopedia source catalog")?;
    ensure!(
        source.schema_version == SOURCE_SCHEMA_VERSION,
        "unsupported Encyclopedia source schema {}",
        source.schema_version
    );
    ensure!(
        source.language_id == ENGLISH_LANGUAGE_ID
            && source.encoding == ENGLISH_ENCODING
            && source.source_code_page == 0,
        "unsupported Encyclopedia source profile"
    );
    ensure!(
        !source.texts.is_empty() && !source.artwork.is_empty(),
        "Encyclopedia source catalog is empty"
    );
    ensure!(
        source.texts.len() <= SOURCE_RESOURCE_LIMIT
            && source.artwork.len() <= SOURCE_RESOURCE_LIMIT,
        "Encyclopedia source catalog exceeds the resource limit"
    );
    for (&resource_id, text) in &source.texts {
        ensure!(resource_id != 0, "Encyclopedia text resource 0 is invalid");
        ensure!(
            !text.body.is_empty()
                && text.body.len() <= SOURCE_BODY_BYTES_LIMIT
                && is_lower_hex_sha256(&text.body_sha256),
            "Encyclopedia text resource {resource_id} has invalid content metadata"
        );
        ensure!(
            sha256_hex(text.body.as_bytes()) == text.body_sha256,
            "Encyclopedia text resource {resource_id} checksum mismatch"
        );
    }
    for (&resource_id, filename) in &source.artwork {
        ensure!(
            resource_id != 0 && is_edata_filename(filename),
            "Encyclopedia artwork resource {resource_id} has invalid filename {filename:?}"
        );
    }
    Ok(source)
}

/// Validate the catalog sidecar and parse its source data as one integrity
/// boundary.
///
/// # Errors
/// Returns an error if the manifest is malformed, its catalog digest or counts
/// disagree, its source identities are invalid, or the catalog itself fails
/// [`parse_encyclopedia_source`].
pub fn parse_encyclopedia_source_with_manifest(
    catalog_bytes: &[u8],
    manifest_bytes: &[u8],
) -> Result<(EncyclopediaSourceCatalog, EncyclopediaSourceManifest)> {
    ensure!(
        manifest_bytes.len() <= SOURCE_MANIFEST_JSON_BYTES_LIMIT,
        "Encyclopedia source manifest JSON byte limit exceeded"
    );
    let manifest: EncyclopediaSourceManifest =
        serde_json::from_slice(manifest_bytes).context("parsing Encyclopedia source manifest")?;
    ensure!(
        manifest.schema_version == SOURCE_SCHEMA_VERSION,
        "unsupported Encyclopedia source manifest schema {}",
        manifest.schema_version
    );
    ensure!(
        is_lower_hex_sha256(&manifest.catalog_sha256)
            && sha256_hex(catalog_bytes) == manifest.catalog_sha256,
        "Encyclopedia source catalog checksum mismatch"
    );
    ensure!(
        is_lower_hex_sha256(&manifest.source_files.encytext_sha256)
            && is_lower_hex_sha256(&manifest.source_files.encybmap_sha256),
        "Encyclopedia source manifest has invalid source identity"
    );
    let source = parse_encyclopedia_source(catalog_bytes)?;
    ensure!(
        manifest.counts.texts == source.texts.len()
            && manifest.counts.artwork_mappings == source.artwork.len(),
        "Encyclopedia source manifest count mismatch"
    );
    Ok((source, manifest))
}

/// Load system picture identities and join every index object to source topic
/// text/artwork. This function performs no filesystem lookup for `EDATA` bytes.
///
/// # Errors
/// Returns an error if `SYSTEMSD.DAT` is malformed or contains an identity that
/// cannot be represented by the source object model.
pub fn load_encyclopedia_topics(
    gdata_path: &Path,
    index: &EncyclopediaCatalog,
    source: &EncyclopediaSourceCatalog,
    audience: EncyclopediaAudience,
) -> Result<EncyclopediaTopicCatalog> {
    let system_pictures = load_encyclopedia_system_pictures(gdata_path)?;
    Ok(bind_encyclopedia_topics(
        index,
        source,
        &system_pictures,
        audience,
    ))
}

/// Load the source system-object to picture-id join used by both audiences.
///
/// Keeping this outside the session builder lets native and browser readers
/// converge after their selected `SYSTEMSD.DAT` bytes have entered the normal
/// data cache.
///
/// # Errors
/// Returns an error if `SYSTEMSD.DAT` is malformed, contains an out-of-range
/// family identity, or repeats a compound object identity.
pub fn load_encyclopedia_system_pictures(gdata_path: &Path) -> Result<HashMap<u32, u32>> {
    let systems: SystemsFile = read_dat_file(&gdata_path.join("SYSTEMSD.DAT"))?;
    let mut system_pictures = HashMap::with_capacity(systems.systems.len());
    for system in systems.systems {
        let family = u8::try_from(system.family_id).with_context(|| {
            format!(
                "system family {:#x} does not fit one byte",
                system.family_id
            )
        })?;
        let object_id = compound_object_id(system.id, family);
        ensure!(
            system_pictures
                .insert(object_id, system.picture_id)
                .is_none(),
            "duplicate Encyclopedia system object {object_id:#010x}"
        );
    }
    Ok(system_pictures)
}

/// Pure binding helper used by the native loader, WASM loader, and tests.
#[must_use]
pub fn bind_encyclopedia_topics(
    index: &EncyclopediaCatalog,
    source: &EncyclopediaSourceCatalog,
    system_pictures: &HashMap<u32, u32>,
    audience: EncyclopediaAudience,
) -> EncyclopediaTopicCatalog {
    let mut entries = Vec::with_capacity(index.entries.len());
    for entry in &index.entries {
        let topic_text_resource_id = ordinary_lookup_key(entry.text_resource_id);
        let system_picture = system_pictures.get(&entry.object_id).copied();
        let artwork_resource_id = if is_system(entry) {
            system_picture.and_then(system_picture_lookup_key)
        } else {
            Some(object_artwork_lookup_key(entry, audience))
        };
        let body = source
            .texts
            .get(&topic_text_resource_id)
            .map(|text| text.body.clone());
        let artwork_filename =
            artwork_resource_id.and_then(|resource_id| source.artwork.get(&resource_id).cloned());

        let mut missing = Vec::new();
        if body.is_none() {
            missing.push(EncyclopediaMissingPart::Text);
        }
        if is_system(entry) && artwork_resource_id.is_none() {
            missing.push(EncyclopediaMissingPart::SystemPicture);
        } else if artwork_filename.is_none() {
            missing.push(EncyclopediaMissingPart::ArtworkMapping);
        }

        entries.push(EncyclopediaTopicBinding {
            object_id: entry.object_id,
            title: entry.name.clone(),
            topic_text_resource_id,
            artwork_resource_id,
            body,
            artwork_filename,
            missing,
        });
    }
    EncyclopediaTopicCatalog {
        language_id: source.language_id,
        audience,
        entries,
    }
}

#[must_use]
pub const fn system_picture_lookup_key(picture_id: u32) -> Option<u16> {
    match picture_id {
        1..=23 => Some(0x2b5c + picture_id as u16 - 1),
        24 => Some(0x2b75),
        25 => Some(0x2b73),
        26 => Some(0x2b74),
        _ => None,
    }
}

const fn ordinary_lookup_key(name_resource_id: u16) -> u16 {
    (name_resource_id & 0x0fff) + 0x1000
}

fn object_artwork_lookup_key(
    entry: &EncyclopediaCatalogEntry,
    audience: EncyclopediaAudience,
) -> u16 {
    let family = entry.family();
    let faction_variant = (0x40..0x80).contains(&family) || (0x08..0x10).contains(&family);
    if faction_variant && matches!(audience, EncyclopediaAudience::Empire) {
        (entry.text_resource_id & 0x0fff) + 0x2000
    } else {
        ordinary_lookup_key(entry.text_resource_id)
    }
}

const fn is_system(entry: &EncyclopediaCatalogEntry) -> bool {
    entry.family() >= 0x90 && entry.family() < 0x98
}

fn is_edata_filename(value: &str) -> bool {
    value.len() == 9
        && value.starts_with("EDATA.")
        && value.as_bytes()[6..].iter().all(u8::is_ascii_digit)
}

fn is_lower_hex_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|value| value.is_ascii_digit() || (b'a'..=b'f').contains(&value))
}

fn sha256_hex(value: &[u8]) -> String {
    format!("{:x}", Sha256::digest(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encyclopedia_catalog::{EncyclopediaCatalog, EncyclopediaCategory};

    const SHARED_SOURCE: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/p66a/source.json");
    const SHARED_MANIFEST: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/p66a/source.json.manifest.json");

    fn fixture_index() -> EncyclopediaCatalog {
        let category = |command_id| EncyclopediaCategory {
            command_id,
            label_resource_id: command_id,
            label: format!("Category {command_id}"),
            family_range: None,
        };
        EncyclopediaCatalog {
            title: "Galactic Encyclopedia".into(),
            topic_label: "Topic".into(),
            categories: [
                category(0x6f),
                category(0x70),
                category(0x71),
                category(0x72),
                category(0x73),
                category(0x74),
                category(0x75),
            ],
            entries: vec![
                EncyclopediaCatalogEntry {
                    object_id: 0x1400_0040,
                    text_resource_id: 0x2740,
                    name: "Mon Calamari Cruiser".into(),
                },
                EncyclopediaCatalogEntry {
                    object_id: 0x5100_0010,
                    text_resource_id: 0x2c50,
                    name: "Diplomacy".into(),
                },
                EncyclopediaCatalogEntry {
                    object_id: 0x9200_0064,
                    text_resource_id: 0x2e00,
                    name: "Abregado-rae".into(),
                },
            ],
        }
    }

    fn fixture_source() -> EncyclopediaSourceCatalog {
        let text = |body: &str| EncyclopediaSourceText {
            body: body.into(),
            body_sha256: "0".repeat(64),
        };
        EncyclopediaSourceCatalog {
            schema_version: 1,
            language_id: 1033,
            encoding: "windows-1252".into(),
            source_code_page: 0,
            texts: HashMap::from([
                (0x1740, text("Ship body")),
                (0x1c50, text("Mission body")),
                (0x1e00, text("System body")),
            ]),
            artwork: HashMap::from([
                (0x1740, "EDATA.014".into()),
                (0x1c50, "EDATA.015".into()),
                (0x2c50, "EDATA.115".into()),
                (0x2b5c, "EDATA.001".into()),
            ]),
        }
    }

    fn minimal_source_json(body: &str) -> String {
        format!(
            r#"{{"schema_version":1,"language_id":1033,"encoding":"windows-1252","source_code_page":0,"texts":{{"5952":{{"body":{},"body_sha256":"{}"}}}},"artwork":{{"5952":"EDATA.014"}}}}"#,
            serde_json::to_string(body).unwrap(),
            sha256_hex(body.as_bytes()),
        )
    }

    fn generated_source_json(text_count: usize) -> String {
        let digest = sha256_hex(b"x");
        let mut texts = String::new();
        for resource_id in 1..=text_count {
            if resource_id > 1 {
                texts.push(',');
            }
            texts.push_str(&format!(
                r#""{resource_id}":{{"body":"x","body_sha256":"{digest}"}}"#
            ));
        }
        format!(
            r#"{{"schema_version":1,"language_id":1033,"encoding":"windows-1252","source_code_page":0,"texts":{{{texts}}},"artwork":{{"1":"EDATA.001"}}}}"#
        )
    }

    #[test]
    fn shared_p66a_fixture_parses_and_binds_both_audiences() {
        let (source, manifest) =
            parse_encyclopedia_source_with_manifest(SHARED_SOURCE, SHARED_MANIFEST).unwrap();
        assert_eq!(manifest.counts.texts, 3);
        assert_eq!(manifest.counts.artwork_mappings, 4);

        let index = fixture_index();
        let pictures = HashMap::from([(0x9200_0064, 1)]);
        for audience in [EncyclopediaAudience::Alliance, EncyclopediaAudience::Empire] {
            let topics = bind_encyclopedia_topics(&index, &source, &pictures, audience);
            assert_eq!(topics.entries.len(), 3);
            assert_eq!(topics.complete_count(), 3);
        }
    }

    #[test]
    fn logical_fingerprint_is_stable_and_content_sensitive() {
        let source = parse_encyclopedia_source(SHARED_SOURCE).unwrap();
        let index = fixture_index();
        let pictures = HashMap::from([(0x9200_0064, 1)]);
        let alliance =
            bind_encyclopedia_topics(&index, &source, &pictures, EncyclopediaAudience::Alliance);
        let repeated =
            bind_encyclopedia_topics(&index, &source, &pictures, EncyclopediaAudience::Alliance);
        let empire =
            bind_encyclopedia_topics(&index, &source, &pictures, EncyclopediaAudience::Empire);

        assert_eq!(
            encyclopedia_logical_fingerprint(&index, &alliance),
            encyclopedia_logical_fingerprint(&index, &repeated)
        );
        assert_ne!(
            encyclopedia_logical_fingerprint(&index, &alliance),
            encyclopedia_logical_fingerprint(&index, &empire)
        );
        assert!(is_lower_hex_sha256(&encyclopedia_logical_fingerprint(
            &index, &alliance
        )));

        let mut changed_source = source;
        changed_source
            .texts
            .get_mut(&0x1740)
            .unwrap()
            .body
            .push('!');
        let changed = bind_encyclopedia_topics(
            &index,
            &changed_source,
            &pictures,
            EncyclopediaAudience::Alliance,
        );
        assert_ne!(
            encyclopedia_logical_fingerprint(&index, &alliance),
            encyclopedia_logical_fingerprint(&index, &changed)
        );
    }

    #[test]
    fn source_parser_rejects_duplicate_resource_identities() {
        let digest = sha256_hex(b"Synthetic");
        let duplicate = format!(
            r#"{{"schema_version":1,"language_id":1033,"encoding":"windows-1252","source_code_page":0,"texts":{{"5952":{{"body":"Synthetic","body_sha256":"{digest}"}},"5952":{{"body":"Synthetic","body_sha256":"{digest}"}}}},"artwork":{{"5952":"EDATA.014"}}}}"#
        );

        assert!(parse_encyclopedia_source(duplicate.as_bytes()).is_err());
    }

    #[test]
    fn source_parser_rejects_closed_schema_and_malformed_values() {
        let valid = minimal_source_json("Synthetic");
        let invalid = [
            valid.replace(r#""body_sha256":"#, r#""unexpected":true,"body_sha256":"#),
            valid.replace(
                r#""body":"Synthetic""#,
                r#""body":"duplicate","body":"Synthetic""#,
            ),
            valid.replacen(r#""5952":"#, r#""65536":"#, 1),
            valid.replace(r#""artwork":"#, r#""missing_artwork":"#),
            format!("{valid}{{}}"),
        ];
        for bytes in invalid {
            assert!(
                parse_encyclopedia_source(bytes.as_bytes()).is_err(),
                "accepted invalid source: {}",
                &bytes[..bytes.len().min(160)]
            );
        }
        assert!(parse_encyclopedia_source(&[0xff]).is_err());
    }

    #[test]
    fn source_parser_enforces_the_documented_body_byte_limit() {
        let boundary = "x".repeat(1_048_576);
        parse_encyclopedia_source(minimal_source_json(&boundary).as_bytes()).unwrap();

        let oversized = format!("{boundary}x");
        assert!(parse_encyclopedia_source(minimal_source_json(&oversized).as_bytes()).is_err());
    }

    #[test]
    fn source_parser_enforces_the_generated_resource_count_limit() {
        parse_encyclopedia_source(generated_source_json(SOURCE_RESOURCE_LIMIT).as_bytes()).unwrap();
        assert!(parse_encyclopedia_source(
            generated_source_json(SOURCE_RESOURCE_LIMIT + 1).as_bytes()
        )
        .is_err());
    }

    #[test]
    fn source_parser_rejects_oversized_json_before_deserialization() {
        let oversized = vec![b' '; 64 * 1024 * 1024 + 1];
        let error = parse_encyclopedia_source(&oversized).unwrap_err();
        assert!(error.to_string().contains("byte limit"));
    }

    #[test]
    fn parses_only_the_supported_source_profile() {
        let json = format!(
            r#"{{
          "schema_version":1,
          "language_id":1033,
          "encoding":"windows-1252",
          "source_code_page":0,
          "texts":{{"5952":{{"body":"Synthetic","body_sha256":"{}"}}}},
          "artwork":{{"5952":"EDATA.014"}}
        }}"#,
            sha256_hex(b"Synthetic")
        );
        let source = parse_encyclopedia_source(json.as_bytes()).unwrap();
        assert_eq!(source.texts[&5952].body, "Synthetic");

        for invalid in [
            json.replace("\"schema_version\":1", "\"schema_version\":2"),
            json.replace("windows-1252", "utf-8"),
            json.replace("EDATA.014", "../EDATA.014"),
            json.replace("Synthetic", "Fabricated"),
        ] {
            assert!(parse_encyclopedia_source(invalid.as_bytes()).is_err());
        }
    }

    #[test]
    fn manifest_binds_the_exact_catalog_bytes_and_counts() {
        let body = "Synthetic";
        let catalog = format!(
            r#"{{"schema_version":1,"language_id":1033,"encoding":"windows-1252","source_code_page":0,"texts":{{"5952":{{"body":"{body}","body_sha256":"{}"}}}},"artwork":{{"5952":"EDATA.014"}}}}"#,
            sha256_hex(body.as_bytes())
        );
        let manifest = format!(
            r#"{{"schema_version":1,"catalog_sha256":"{}","source_files":{{"encytext_sha256":"{}","encybmap_sha256":"{}"}},"counts":{{"texts":1,"artwork_mappings":1}}}}"#,
            sha256_hex(catalog.as_bytes()),
            "a".repeat(64),
            "b".repeat(64),
        );
        parse_encyclopedia_source_with_manifest(catalog.as_bytes(), manifest.as_bytes()).unwrap();
        let tampered = catalog.replace("Synthetic", "Fabricated");
        assert!(
            parse_encyclopedia_source_with_manifest(tampered.as_bytes(), manifest.as_bytes())
                .is_err()
        );
    }

    #[test]
    fn binds_text_system_art_and_faction_mission_art_without_fallbacks() {
        let index = fixture_index();
        let source = fixture_source();
        let pictures = HashMap::from([(0x9200_0064, 1)]);

        let alliance =
            bind_encyclopedia_topics(&index, &source, &pictures, EncyclopediaAudience::Alliance);
        assert_eq!(alliance.complete_count(), 3);
        assert_eq!(alliance.entries[0].topic_text_resource_id, 0x1740);
        assert_eq!(
            alliance.entries[0].artwork_filename.as_deref(),
            Some("EDATA.014")
        );
        assert_eq!(alliance.entries[1].artwork_resource_id, Some(0x1c50));
        assert_eq!(alliance.entries[2].artwork_resource_id, Some(0x2b5c));
        let empire =
            bind_encyclopedia_topics(&index, &source, &pictures, EncyclopediaAudience::Empire);
        assert_eq!(empire.entries[1].topic_text_resource_id, 0x1c50);
        assert_eq!(empire.entries[1].artwork_resource_id, Some(0x2c50));
        assert_eq!(
            empire.entries[1].artwork_filename.as_deref(),
            Some("EDATA.115")
        );
    }

    #[test]
    fn missing_source_parts_remain_explicit() {
        let index = fixture_index();
        let mut source = fixture_source();
        source.texts.remove(&0x1740);
        source.artwork.remove(&0x1740);
        let topics = bind_encyclopedia_topics(
            &index,
            &source,
            &HashMap::from([(0x9200_0064, 99)]),
            EncyclopediaAudience::Alliance,
        );
        assert_eq!(
            topics.entries[0].missing,
            [
                EncyclopediaMissingPart::Text,
                EncyclopediaMissingPart::ArtworkMapping,
            ]
        );
        assert_eq!(
            topics.entries[2].missing,
            [EncyclopediaMissingPart::SystemPicture]
        );
        assert!(!topics.entries[0].is_complete());
    }

    #[test]
    fn system_picture_table_matches_the_recovered_switch() {
        assert_eq!(system_picture_lookup_key(1), Some(0x2b5c));
        assert_eq!(system_picture_lookup_key(23), Some(0x2b72));
        assert_eq!(system_picture_lookup_key(24), Some(0x2b75));
        assert_eq!(system_picture_lookup_key(25), Some(0x2b73));
        assert_eq!(system_picture_lookup_key(26), Some(0x2b74));
        assert_eq!(system_picture_lookup_key(0), None);
        assert_eq!(system_picture_lookup_key(27), None);
    }
}
