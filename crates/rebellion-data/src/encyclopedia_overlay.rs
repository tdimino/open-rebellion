//! Presence-aware presentation overlays for immutable Encyclopedia sessions.
//!
//! The base source bytes and sidecar remain the provenance authority. Native
//! mods may contribute a separate `encyclopedia.json`; this module parses that
//! target and applies it only to an off-side effective-session candidate.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::Arc;

use anyhow::{bail, ensure, Context, Result};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

use crate::encyclopedia_catalog::{EncyclopediaCatalog, EncyclopediaCatalogEntry};
use crate::encyclopedia_session::EncyclopediaResourceBytes;
use crate::encyclopedia_topics::{
    system_picture_lookup_key, EncyclopediaSourceCatalog, EncyclopediaSourceText,
};

pub const ENCYCLOPEDIA_OVERLAY_BYTES_LIMIT: usize = 16 * 1024 * 1024;
const OVERLAY_PATCH_LIMIT: usize = 10_000;
const TITLE_BYTES_LIMIT: usize = 65_536;
const BODY_BYTES_LIMIT: usize = 1_048_576;
const MOD_ASSET_PREFIX: &str = "encyclopedia/assets/";

/// Presence-aware field state. Missing inherits the lower layer; `Null`
/// requests removal; `Value` replaces it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PatchField<T> {
    #[default]
    Missing,
    Null,
    Value(T),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EncyclopediaOverlayAction {
    #[default]
    Patch,
    Replace,
    Add,
    Remove,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaImagePatch {
    path: String,
}

impl EncyclopediaImagePatch {
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaTopicPatch {
    object_id: u32,
    action: EncyclopediaOverlayAction,
    text_resource_id: PatchField<u16>,
    title: PatchField<String>,
    body: PatchField<String>,
    image: PatchField<EncyclopediaImagePatch>,
}

/// One parsed mod contribution plus exact already-loaded author artwork.
#[derive(Debug, Clone)]
pub struct EncyclopediaOverlayLayer {
    mod_name: String,
    patches: Vec<EncyclopediaTopicPatch>,
    artwork: BTreeMap<String, Arc<[u8]>>,
}

impl EncyclopediaOverlayLayer {
    /// Construct one layer after filesystem acquisition.
    ///
    /// # Errors
    /// Rejects an unsafe mod identity or an artwork set that is missing a
    /// referenced path or supplies an unreferenced path.
    pub fn new(
        mod_name: String,
        patches: Vec<EncyclopediaTopicPatch>,
        artwork: BTreeMap<String, Arc<[u8]>>,
    ) -> Result<Self> {
        ensure!(
            valid_mod_name(&mod_name),
            "invalid Encyclopedia mod identity {mod_name:?}"
        );
        let expected = referenced_artwork_paths(&patches);
        let supplied: BTreeSet<&str> = artwork.keys().map(String::as_str).collect();
        if let Some(path) = expected.difference(&supplied).next() {
            bail!("missing Encyclopedia mod artwork {path}");
        }
        if let Some(path) = supplied.difference(&expected).next() {
            bail!("unexpected Encyclopedia mod artwork {path}");
        }
        Ok(Self {
            mod_name,
            patches,
            artwork,
        })
    }

    #[must_use]
    pub fn mod_name(&self) -> &str {
        &self.mod_name
    }
}

/// Parse one reserved `encyclopedia.json` target.
///
/// # Errors
/// Rejects oversized input, malformed JSON, duplicate selectors, unknown
/// fields, invalid actions, unsafe paths, and out-of-range text values.
pub fn parse_encyclopedia_overlay(bytes: &[u8]) -> Result<Vec<EncyclopediaTopicPatch>> {
    ensure!(
        bytes.len() <= ENCYCLOPEDIA_OVERLAY_BYTES_LIMIT,
        "Encyclopedia overlay byte limit exceeded"
    );
    let value: Value =
        serde_json::from_slice(bytes).context("parsing Encyclopedia overlay JSON")?;
    let values = value
        .as_array()
        .context("Encyclopedia overlay root must be an array")?;
    ensure!(
        values.len() <= OVERLAY_PATCH_LIMIT,
        "Encyclopedia overlay patch limit exceeded"
    );

    let mut selectors = HashSet::with_capacity(values.len());
    let mut patches = Vec::with_capacity(values.len());
    for (index, value) in values.iter().enumerate() {
        let patch = parse_topic_patch(value, index)?;
        ensure!(
            selectors.insert(patch.object_id),
            "duplicate Encyclopedia topic selector {:#010x}",
            patch.object_id
        );
        patches.push(patch);
    }
    Ok(patches)
}

#[must_use]
pub fn referenced_artwork_paths(patches: &[EncyclopediaTopicPatch]) -> BTreeSet<&str> {
    patches
        .iter()
        .filter_map(|patch| match &patch.image {
            PatchField::Value(image) => Some(image.path()),
            PatchField::Missing | PatchField::Null => None,
        })
        .collect()
}

fn parse_topic_patch(value: &Value, index: usize) -> Result<EncyclopediaTopicPatch> {
    let object = value
        .as_object()
        .with_context(|| format!("Encyclopedia overlay patch {index} must be an object"))?;
    reject_unknown_fields(
        object,
        &["id", "action", "text_resource_id", "title", "body", "image"],
        index,
    )?;
    let object_id = parse_u32(
        object
            .get("id")
            .with_context(|| format!("Encyclopedia overlay patch {index} is missing id"))?,
        "id",
        index,
    )?;
    let action = match object.get("action") {
        None => EncyclopediaOverlayAction::Patch,
        Some(Value::String(value)) if value == "patch" => EncyclopediaOverlayAction::Patch,
        Some(Value::String(value)) if value == "replace" => EncyclopediaOverlayAction::Replace,
        Some(Value::String(value)) if value == "add" => EncyclopediaOverlayAction::Add,
        Some(Value::String(value)) if value == "remove" => EncyclopediaOverlayAction::Remove,
        Some(_) => bail!("Encyclopedia overlay patch {index} has an invalid action"),
    };
    let text_resource_id = parse_optional_u16(object.get("text_resource_id"), index)?;
    let title = parse_optional_text(object.get("title"), "title", TITLE_BYTES_LIMIT, index)?;
    let body = parse_optional_text(object.get("body"), "body", BODY_BYTES_LIMIT, index)?;
    let image = parse_optional_image(object.get("image"), index)?;
    if action == EncyclopediaOverlayAction::Remove {
        ensure!(
            matches!(text_resource_id, PatchField::Missing)
                && matches!(title, PatchField::Missing)
                && matches!(body, PatchField::Missing)
                && matches!(image, PatchField::Missing),
            "Encyclopedia remove patch {index} contains replacement fields"
        );
    }
    Ok(EncyclopediaTopicPatch {
        object_id,
        action,
        text_resource_id,
        title,
        body,
        image,
    })
}

fn reject_unknown_fields(
    object: &Map<String, Value>,
    allowed: &[&str],
    index: usize,
) -> Result<()> {
    if let Some(key) = object.keys().find(|key| !allowed.contains(&key.as_str())) {
        bail!("Encyclopedia overlay patch {index} has unknown field {key:?}");
    }
    Ok(())
}

fn parse_u32(value: &Value, field: &str, index: usize) -> Result<u32> {
    let value = value
        .as_u64()
        .with_context(|| format!("Encyclopedia overlay patch {index} {field} must be unsigned"))?;
    u32::try_from(value)
        .with_context(|| format!("Encyclopedia overlay patch {index} {field} exceeds u32"))
}

fn parse_optional_u16(value: Option<&Value>, index: usize) -> Result<PatchField<u16>> {
    match value {
        None => Ok(PatchField::Missing),
        Some(Value::Null) => Ok(PatchField::Null),
        Some(value) => {
            let value = parse_u32(value, "text_resource_id", index)?;
            Ok(PatchField::Value(u16::try_from(value).with_context(
                || format!("Encyclopedia overlay patch {index} text_resource_id exceeds u16"),
            )?))
        }
    }
}

fn parse_optional_text(
    value: Option<&Value>,
    field: &str,
    limit: usize,
    index: usize,
) -> Result<PatchField<String>> {
    match value {
        None => Ok(PatchField::Missing),
        Some(Value::Null) => Ok(PatchField::Null),
        Some(Value::String(value)) => {
            ensure!(
                !value.is_empty() && value.len() <= limit,
                "Encyclopedia overlay patch {index} {field} is empty or oversized"
            );
            Ok(PatchField::Value(value.clone()))
        }
        Some(_) => bail!("Encyclopedia overlay patch {index} {field} must be a string or null"),
    }
}

fn parse_optional_image(
    value: Option<&Value>,
    index: usize,
) -> Result<PatchField<EncyclopediaImagePatch>> {
    match value {
        None => Ok(PatchField::Missing),
        Some(Value::Null) => Ok(PatchField::Null),
        Some(Value::Object(object)) => {
            ensure!(
                object.len() == 1 && object.contains_key("path"),
                "Encyclopedia overlay patch {index} image must contain only path"
            );
            let path = object["path"].as_str().with_context(|| {
                format!("Encyclopedia overlay patch {index} image path must be a string")
            })?;
            ensure!(
                valid_mod_image_path(path),
                "Encyclopedia overlay patch {index} has unsafe image path {path:?}"
            );
            Ok(PatchField::Value(EncyclopediaImagePatch {
                path: path.to_owned(),
            }))
        }
        Some(_) => bail!("Encyclopedia overlay patch {index} image must be an object or null"),
    }
}

fn valid_mod_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

fn valid_mod_image_path(value: &str) -> bool {
    if value.len() > 256 {
        return false;
    }
    let Some(rest) = value.strip_prefix(MOD_ASSET_PREFIX) else {
        return false;
    };
    let segments: Vec<_> = rest.split('/').collect();
    !segments.is_empty()
        && segments.iter().enumerate().all(|(index, segment)| {
            let last = index + 1 == segments.len();
            !segment.is_empty()
                && segment.len() <= 127
                && segment.as_bytes()[0].is_ascii_alphanumeric()
                && segment
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
                && (!last
                    || segment
                        .rsplit_once('.')
                        .is_some_and(|(stem, extension)| !stem.is_empty() && extension == "bmp"))
        })
}

/// Apply one mod layer to a private candidate. The session builder validates
/// the complete result after every layer.
pub(crate) fn apply_encyclopedia_overlay_layer(
    catalog: &mut EncyclopediaCatalog,
    source: &mut EncyclopediaSourceCatalog,
    system_pictures: &HashMap<u32, u32>,
    artwork: &mut EncyclopediaResourceBytes,
    layer: &EncyclopediaOverlayLayer,
) -> Result<()> {
    for patch in &layer.patches {
        apply_topic_patch(catalog, source, system_pictures, artwork, layer, patch).with_context(
            || {
                format!(
                    "applying Encyclopedia mod {:?} topic {:#010x}",
                    layer.mod_name, patch.object_id
                )
            },
        )?;
    }
    catalog.entries.sort_by(|left, right| {
        left.name
            .to_lowercase()
            .cmp(&right.name.to_lowercase())
            .then(left.object_id.cmp(&right.object_id))
    });
    let referenced: BTreeSet<&str> = source.artwork.values().map(String::as_str).collect();
    artwork.retain(|filename, _| referenced.contains(filename.as_str()));
    Ok(())
}

fn apply_topic_patch(
    catalog: &mut EncyclopediaCatalog,
    source: &mut EncyclopediaSourceCatalog,
    system_pictures: &HashMap<u32, u32>,
    artwork: &mut EncyclopediaResourceBytes,
    layer: &EncyclopediaOverlayLayer,
    patch: &EncyclopediaTopicPatch,
) -> Result<()> {
    let position = catalog
        .entries
        .iter()
        .position(|entry| entry.object_id == patch.object_id);
    match patch.action {
        EncyclopediaOverlayAction::Add => {
            ensure!(position.is_none(), "topic already exists");
            let text_resource_id = require_value(&patch.text_resource_id, "text_resource_id")?;
            ensure!(*text_resource_id != 0, "text_resource_id must be nonzero");
            ensure!(
                !(0x90..0x98).contains(&family(patch.object_id)),
                "mod-added system topics require an immutable system-picture binding"
            );
            let text_key = ordinary_lookup_key(*text_resource_id);
            let art_keys = non_system_artwork_keys(patch.object_id, *text_resource_id);
            ensure!(
                !catalog
                    .entries
                    .iter()
                    .any(|entry| entry.text_resource_id == *text_resource_id),
                "text_resource_id is already used by another topic"
            );
            ensure!(
                !source.texts.contains_key(&text_key)
                    && art_keys.iter().all(|key| !source.artwork.contains_key(key)),
                "derived source identity is already occupied"
            );
            let title = require_value(&patch.title, "title")?.clone();
            catalog.entries.push(EncyclopediaCatalogEntry {
                object_id: patch.object_id,
                text_resource_id: *text_resource_id,
                name: title,
            });
            set_body(source, text_key, require_value(&patch.body, "body")?);
            set_image(
                source,
                artwork,
                layer,
                &art_keys,
                require_value(&patch.image, "image")?,
            )?;
        }
        EncyclopediaOverlayAction::Remove => {
            let position = position.context("cannot remove an unknown topic")?;
            let removed = catalog.entries.remove(position);
            remove_unreferenced_source_for_entry(&removed, catalog, source, system_pictures);
        }
        EncyclopediaOverlayAction::Patch | EncyclopediaOverlayAction::Replace => {
            let position = position.context("cannot patch an unknown topic")?;
            let current_text_resource_id = catalog.entries[position].text_resource_id;
            match &patch.text_resource_id {
                PatchField::Missing => {}
                PatchField::Value(value) if *value == current_text_resource_id => {}
                PatchField::Value(_) | PatchField::Null => {
                    bail!("an existing topic's text_resource_id is immutable")
                }
            }
            if patch.action == EncyclopediaOverlayAction::Replace {
                require_value(&patch.title, "title")?;
                require_value(&patch.body, "body")?;
                require_value(&patch.image, "image")?;
            }
            match &patch.title {
                PatchField::Missing => {}
                PatchField::Null => bail!("a topic title cannot be removed"),
                PatchField::Value(value) => catalog.entries[position].name.clone_from(value),
            }
            let text_key = ordinary_lookup_key(current_text_resource_id);
            match &patch.body {
                PatchField::Missing => {}
                PatchField::Null => {
                    source.texts.remove(&text_key);
                }
                PatchField::Value(value) => set_body(source, text_key, value),
            }
            let art_keys = artwork_keys(&catalog.entries[position], system_pictures)?;
            match &patch.image {
                PatchField::Missing => {}
                PatchField::Null => {
                    for key in art_keys {
                        source.artwork.remove(&key);
                    }
                }
                PatchField::Value(image) => {
                    set_image(source, artwork, layer, &art_keys, image)?;
                }
            }
        }
    }
    Ok(())
}

fn require_value<'a, T>(field: &'a PatchField<T>, name: &str) -> Result<&'a T> {
    match field {
        PatchField::Value(value) => Ok(value),
        PatchField::Missing | PatchField::Null => {
            bail!("{name} must have a replacement value for this action")
        }
    }
}

fn set_body(source: &mut EncyclopediaSourceCatalog, key: u16, value: &str) {
    source.texts.insert(
        key,
        EncyclopediaSourceText {
            body: value.to_owned(),
            body_sha256: format!("{:x}", Sha256::digest(value.as_bytes())),
        },
    );
}

fn set_image(
    source: &mut EncyclopediaSourceCatalog,
    artwork: &mut EncyclopediaResourceBytes,
    layer: &EncyclopediaOverlayLayer,
    keys: &[u16],
    image: &EncyclopediaImagePatch,
) -> Result<()> {
    let bytes = layer
        .artwork
        .get(image.path())
        .with_context(|| format!("missing inspected artwork {}", image.path()))?;
    let identity = format!("mod:v1:{}:{}", layer.mod_name, image.path());
    artwork.insert(identity.clone(), Arc::clone(bytes));
    for key in keys {
        source.artwork.insert(*key, identity.clone());
    }
    Ok(())
}

fn remove_unreferenced_source_for_entry(
    removed: &EncyclopediaCatalogEntry,
    catalog: &EncyclopediaCatalog,
    source: &mut EncyclopediaSourceCatalog,
    system_pictures: &HashMap<u32, u32>,
) {
    let text_key = ordinary_lookup_key(removed.text_resource_id);
    if !catalog
        .entries
        .iter()
        .any(|entry| ordinary_lookup_key(entry.text_resource_id) == text_key)
    {
        source.texts.remove(&text_key);
    }
    if let Ok(removed_keys) = artwork_keys(removed, system_pictures) {
        let retained: BTreeSet<u16> = catalog
            .entries
            .iter()
            .filter_map(|entry| artwork_keys(entry, system_pictures).ok())
            .flatten()
            .collect();
        for key in removed_keys {
            if !retained.contains(&key) {
                source.artwork.remove(&key);
            }
        }
    }
}

fn artwork_keys(
    entry: &EncyclopediaCatalogEntry,
    system_pictures: &HashMap<u32, u32>,
) -> Result<Vec<u16>> {
    if (0x90..0x98).contains(&entry.family()) {
        let picture = system_pictures
            .get(&entry.object_id)
            .context("system topic has no picture binding")?;
        return Ok(system_picture_lookup_key(*picture).into_iter().collect());
    }
    Ok(non_system_artwork_keys(
        entry.object_id,
        entry.text_resource_id,
    ))
}

fn non_system_artwork_keys(object_id: u32, text_resource_id: u16) -> Vec<u16> {
    let ordinary = ordinary_lookup_key(text_resource_id);
    if (0x40..0x80).contains(&family(object_id)) || (0x08..0x10).contains(&family(object_id)) {
        vec![ordinary, (text_resource_id & 0x0fff) + 0x2000]
    } else {
        vec![ordinary]
    }
}

const fn ordinary_lookup_key(text_resource_id: u16) -> u16 {
    (text_resource_id & 0x0fff) + 0x1000
}

const fn family(object_id: u32) -> u8 {
    (object_id >> 24) as u8
}

#[cfg(test)]
mod tests {
    use super::{
        parse_encyclopedia_overlay, valid_mod_image_path, EncyclopediaOverlayAction, PatchField,
        MOD_ASSET_PREFIX,
    };

    #[test]
    fn parser_distinguishes_absent_null_and_replacement_fields() {
        let patches = parse_encyclopedia_overlay(
            br#"[
                {"id":1358954512,"title":"Renamed"},
                {"id":1090519041,"body":null,"image":null},
                {"id":335544385,"action":"remove"}
            ]"#,
        )
        .unwrap();

        assert!(matches!(patches[0].title, PatchField::Value(ref value) if value == "Renamed"));
        assert!(matches!(patches[0].body, PatchField::Missing));
        assert!(matches!(patches[1].body, PatchField::Null));
        assert!(matches!(patches[1].image, PatchField::Null));
        assert_eq!(patches[2].action, EncyclopediaOverlayAction::Remove);
    }

    #[test]
    fn parser_rejects_duplicate_selectors_unknown_fields_and_unsafe_paths() {
        for invalid in [
            br#"[{"id":1},{"id":1}]"#.as_slice(),
            br#"[{"id":1,"bindings":[]}]"#.as_slice(),
            br#"[{"id":1,"image":{"path":"../escape.bmp"}}]"#.as_slice(),
        ] {
            assert!(parse_encyclopedia_overlay(invalid).is_err());
        }
    }

    #[test]
    fn image_path_validator_enforces_every_segment_and_length_boundary() {
        for valid in [
            "encyclopedia/assets/a.bmp",
            "encyclopedia/assets/nested/art_1-2.bmp",
            "encyclopedia/assets/nested.dir/art.bmp",
        ] {
            assert!(valid_mod_image_path(valid), "expected valid path {valid:?}");
        }

        for invalid in [
            "",
            "encyclopedia/assets/",
            "encyclopedia/assets//art.bmp",
            "encyclopedia/assets/nested//art.bmp",
            "encyclopedia/assets/.hidden.bmp",
            "encyclopedia/assets/bad!.bmp",
            "encyclopedia/assets/art.png",
            "encyclopedia/assets/art.BMP",
            "encyclopedia/assets/art.",
        ] {
            assert!(
                !valid_mod_image_path(invalid),
                "expected invalid path {invalid:?}"
            );
        }

        let oversized_segment = format!("{MOD_ASSET_PREFIX}{}/art.bmp", "d".repeat(128));
        assert!(!valid_mod_image_path(&oversized_segment));

        let filename = format!("{}.bmp", "f".repeat(122));
        let exact_directory_len = 256 - MOD_ASSET_PREFIX.len() - 1 - filename.len();
        let exact_limit = format!(
            "{MOD_ASSET_PREFIX}{}/{filename}",
            "d".repeat(exact_directory_len)
        );
        assert_eq!(exact_limit.len(), 256);
        assert!(valid_mod_image_path(&exact_limit));

        let beyond_limit = format!(
            "{MOD_ASSET_PREFIX}{}/{filename}",
            "d".repeat(exact_directory_len + 1)
        );
        assert_eq!(beyond_limit.len(), 257);
        assert!(!valid_mod_image_path(&beyond_limit));
    }
}
