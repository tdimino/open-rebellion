use std::collections::{BTreeMap, BTreeSet, HashSet};

use serde_json::{Map, Value};

use super::{
    array, count_range, error, fields, object, parse_raw_document, string,
    validate_effective_catalog, validate_langid, validate_stable_id, AssetFacts, BaseImage,
    BaseImageId, BaseImageIdField, EncyclopediaCatalog, EncyclopediaError, ImageSelector,
    LocalizedContent, NullableBaseImageId, TopicId,
};

pub const OVERLAY_JSON_BYTES_LIMIT: usize = 16 * 1024 * 1024;
pub const OVERLAY_JSON_DEPTH_LIMIT: usize = 8;
const OVERLAY_PATCH_LIMIT: usize = 10_000;
const LANGUAGE_LIMIT: usize = 256;
const TITLE_BYTES_LIMIT: usize = 65_536;
const BODY_BYTES_LIMIT: usize = 1_048_576;
const MOD_IMAGE_PREFIX: &str = "mod:v1:";
const MOD_ASSET_PREFIX: &str = "encyclopedia/assets/";

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PatchField<T> {
    #[default]
    Missing,
    Null,
    Value(T),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopicPatch {
    pub id: TopicId,
    pub localized: BTreeMap<String, PatchField<LocalizedPatch>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LocalizedPatch {
    pub title: PatchField<String>,
    pub body: PatchField<String>,
    pub image: PatchField<ImagePatch>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImagePatch {
    Static { path: String },
    ViewerFaction(FactionImagePair),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FactionImagePair {
    pub alliance: FactionImagePatch,
    pub empire: FactionImagePatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FactionImagePatch {
    Null,
    Value { path: String },
}

/// Exact observations supplied by the adapter. `current` is keyed by the
/// current catalog's canonical image IDs; `replacements` is keyed by confined
/// author paths. The available byte count is the enclosing session ledger's
/// reservation available for generated ID plus retained owner/path buffers.
pub struct OverlayImageInputs<'a> {
    pub current: &'a BTreeMap<String, AssetFacts>,
    pub replacements: &'a BTreeMap<String, AssetFacts>,
    pub retained_identity_bytes: usize,
}

pub fn parse_encyclopedia_overlay(bytes: &[u8]) -> Result<Vec<TopicPatch>, EncyclopediaError> {
    let value = parse_raw_document(
        bytes,
        "overlay",
        OVERLAY_JSON_BYTES_LIMIT,
        OVERLAY_JSON_DEPTH_LIMIT,
    )?;
    let values = array(&value, "overlay", "$")?;
    count_range(
        values.len(),
        0,
        OVERLAY_PATCH_LIMIT,
        "overlay",
        "$",
        "resource_limit:patches",
    )?;

    let mut selectors = HashSet::with_capacity(values.len());
    let mut patches = Vec::with_capacity(values.len());
    for (index, value) in values.iter().enumerate() {
        let path = format!("$[{index}]");
        let patch = parse_topic_patch(value, &path)?;
        if !selectors.insert(patch.id.0.clone()) {
            return error(
                "duplicate_topic_selector",
                "overlay",
                format!("{path}.id"),
                format!("topic selector {:?} repeats", patch.id.0),
            );
        }
        patches.push(patch);
    }
    Ok(patches)
}

fn parse_topic_patch(value: &Value, path: &str) -> Result<TopicPatch, EncyclopediaError> {
    let patch = object(value, "overlay", path)?;
    overlay_topic_fields(patch, path)?;
    let id_path = format!("{path}.id");
    let id = string(patch.get("id").unwrap(), "overlay", &id_path)?;
    validate_stable_id(id, "invalid_topic_id", "overlay", &id_path)?;

    let localized_path = format!("{path}.localized");
    let localized = object(patch.get("localized").unwrap(), "overlay", &localized_path)?;
    count_range(
        localized.len(),
        1,
        LANGUAGE_LIMIT,
        "overlay",
        &localized_path,
        "resource_limit:languages",
    )?;
    let mut parsed = BTreeMap::new();
    for (language, value) in localized {
        let language_path = format!("{localized_path}.{language}");
        validate_langid(language, "overlay", &language_path)?;
        let field = if value.is_null() {
            PatchField::Null
        } else {
            PatchField::Value(parse_localized_patch(value, &language_path)?)
        };
        parsed.insert(language.clone(), field);
    }
    Ok(TopicPatch {
        id: TopicId(id.to_owned()),
        localized: parsed,
    })
}

fn overlay_topic_fields(patch: &Map<String, Value>, path: &str) -> Result<(), EncyclopediaError> {
    for key in patch.keys() {
        if matches!(key.as_str(), "id" | "localized") {
            continue;
        }
        if matches!(
            key.as_str(),
            "bindings"
                | "categories"
                | "images"
                | "index"
                | "schema_version"
                | "source_ref"
                | "topic_ids"
                | "topic_sort"
        ) {
            return error(
                "forbidden_overlay_field",
                "overlay",
                format!("{path}.{key}"),
                format!("protected field {key:?} cannot be patched"),
            );
        }
        return error(
            "unknown_field",
            "overlay",
            format!("{path}.{key}"),
            format!("unknown field {key:?}"),
        );
    }
    for required in ["id", "localized"] {
        if !patch.contains_key(required) {
            return error(
                "missing_field",
                "overlay",
                format!("{path}.{required}"),
                format!("missing required field {required:?}"),
            );
        }
    }
    Ok(())
}

fn parse_localized_patch(value: &Value, path: &str) -> Result<LocalizedPatch, EncyclopediaError> {
    let localized = object(value, "overlay", path)?;
    if localized.is_empty() {
        return error(
            "empty_overlay_patch",
            "overlay",
            path,
            "localized patch must contain at least one field",
        );
    }
    fields(localized, &[], &["title", "body", "image"], "overlay", path)?;

    Ok(LocalizedPatch {
        title: parse_text_field(
            localized.get("title"),
            &format!("{path}.title"),
            TITLE_BYTES_LIMIT,
        )?,
        body: parse_text_field(
            localized.get("body"),
            &format!("{path}.body"),
            BODY_BYTES_LIMIT,
        )?,
        image: match localized.get("image") {
            None => PatchField::Missing,
            Some(value) if value.is_null() => PatchField::Null,
            Some(value) => PatchField::Value(parse_image_patch(value, &format!("{path}.image"))?),
        },
    })
}

fn parse_text_field(
    value: Option<&Value>,
    path: &str,
    byte_limit: usize,
) -> Result<PatchField<String>, EncyclopediaError> {
    match value {
        None => Ok(PatchField::Missing),
        Some(value) if value.is_null() => Ok(PatchField::Null),
        Some(value) => {
            let text = string(value, "overlay", path)?;
            if text.len() > byte_limit {
                return error(
                    if byte_limit == BODY_BYTES_LIMIT {
                        "resource_limit:body_bytes"
                    } else {
                        "resource_limit:title_bytes"
                    },
                    "overlay",
                    path,
                    format!("{} bytes exceeds {byte_limit}", text.len()),
                );
            }
            Ok(PatchField::Value(text.to_owned()))
        }
    }
}

fn parse_image_patch(value: &Value, path: &str) -> Result<ImagePatch, EncyclopediaError> {
    let image = object(value, "overlay", path)?;
    if image.contains_key("path") {
        fields(image, &["path"], &[], "overlay", path)?;
        return Ok(ImagePatch::Static {
            path: parse_image_path(image.get("path").unwrap(), &format!("{path}.path"))?,
        });
    }

    fields(image, &["alliance", "empire"], &[], "overlay", path)?;
    Ok(ImagePatch::ViewerFaction(FactionImagePair {
        alliance: parse_faction_side(image.get("alliance").unwrap(), &format!("{path}.alliance"))?,
        empire: parse_faction_side(image.get("empire").unwrap(), &format!("{path}.empire"))?,
    }))
}

fn parse_faction_side(value: &Value, path: &str) -> Result<FactionImagePatch, EncyclopediaError> {
    if value.is_null() {
        return Ok(FactionImagePatch::Null);
    }
    let side = object(value, "overlay", path)?;
    fields(side, &["path"], &[], "overlay", path)?;
    Ok(FactionImagePatch::Value {
        path: parse_image_path(side.get("path").unwrap(), &format!("{path}.path"))?,
    })
}

fn parse_image_path(value: &Value, path: &str) -> Result<String, EncyclopediaError> {
    let value = string(value, "overlay", path)?;
    if !valid_mod_image_path(value) {
        return error(
            "unsafe_asset_path",
            "overlay",
            path,
            format!("unsafe mod image path {value:?}"),
        );
    }
    Ok(value.to_owned())
}

fn valid_mod_image_path(value: &str) -> bool {
    if !(25..=256).contains(&value.len()) {
        return false;
    }
    let Some(rest) = value.strip_prefix(MOD_ASSET_PREFIX) else {
        return false;
    };
    let segments: Vec<_> = rest.split('/').collect();
    if segments.is_empty() {
        return false;
    }
    segments.iter().enumerate().all(|(index, segment)| {
        let last = index + 1 == segments.len();
        let maximum = if last { 125 } else { 127 };
        if segment.is_empty()
            || segment.len() > maximum
            || !segment.as_bytes()[0].is_ascii_alphanumeric()
            || !segment
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        {
            return false;
        }
        if !last {
            return true;
        }
        let Some((stem, extension)) = segment.rsplit_once('.') else {
            return false;
        };
        !stem.is_empty() && stem.len() <= 121 && matches!(extension, "bmp" | "png")
    })
}

pub fn apply_encyclopedia_overlay(
    current: &EncyclopediaCatalog,
    mod_name: &str,
    patches: &[TopicPatch],
    images: OverlayImageInputs<'_>,
) -> Result<EncyclopediaCatalog, EncyclopediaError> {
    validate_effective_catalog(current, images.current)?;
    validate_typed_patches(patches)?;
    for path in images.replacements.keys() {
        if !valid_mod_image_path(path) {
            return error(
                "unsafe_asset_path",
                "overlay",
                format!("$.images.{path}"),
                format!("unsafe replacement path {path:?}"),
            );
        }
    }

    let capabilities = topic_capabilities(current)?;
    let mut candidate = current.clone();
    let mut effective_facts = images.current.clone();
    let mut context = ImageContext {
        mod_name,
        replacements: images.replacements,
        remaining_identity_bytes: images.retained_identity_bytes,
        prepared: BTreeMap::new(),
    };

    for (patch_index, patch) in patches.iter().enumerate() {
        let topic_path = format!("$[{patch_index}]");
        let capability = capabilities.get(&patch.id).copied().ok_or_else(|| {
            EncyclopediaError::new(
                "unknown_topic",
                "overlay",
                format!("{topic_path}.id"),
                format!("topic {:?} does not exist", patch.id.0),
            )
            .with_topic(&patch.id.0)
        })?;

        for (language, localized_patch) in &patch.localized {
            let localized_path = format!("{topic_path}.localized.{language}");
            match localized_patch {
                PatchField::Missing => unreachable!("language map entries are always present"),
                PatchField::Null => {
                    candidate
                        .topics
                        .get_mut(&patch.id)
                        .unwrap()
                        .localized
                        .remove(language);
                }
                PatchField::Value(localized_patch) => {
                    let existing = candidate.topics[&patch.id].localized.get(language);
                    let content = apply_localized_patch(
                        existing,
                        localized_patch,
                        capability,
                        &patch.id,
                        &localized_path,
                        &mut candidate.images,
                        &mut effective_facts,
                        &mut context,
                    )?;
                    candidate
                        .topics
                        .get_mut(&patch.id)
                        .unwrap()
                        .localized
                        .insert(language.clone(), content);
                }
            }
        }
    }

    if let Some((topic_id, _)) = candidate
        .topics
        .iter()
        .find(|(_, topic)| topic.localized.is_empty())
    {
        return Err(EncyclopediaError::new(
            "incomplete_localized_record",
            "overlay",
            format!("$.topics.{}.localized", topic_id.0),
            "overlay deleted the topic's final localized record",
        )
        .with_topic(&topic_id.0));
    }

    if let Some(extra) = images
        .replacements
        .keys()
        .find(|path| !context.prepared.contains_key(*path))
    {
        return error(
            "unexpected_runtime_file",
            "overlay",
            format!("$.images.{extra}"),
            "replacement facts were supplied for an unreferenced author path",
        );
    }

    prune_unreferenced_mod_images(&mut candidate, &mut effective_facts);
    validate_effective_catalog(&candidate, &effective_facts)?;
    Ok(candidate)
}

fn validate_typed_patches(patches: &[TopicPatch]) -> Result<(), EncyclopediaError> {
    count_range(
        patches.len(),
        0,
        OVERLAY_PATCH_LIMIT,
        "overlay",
        "$",
        "resource_limit:patches",
    )?;
    let mut selectors = BTreeSet::new();
    for (index, patch) in patches.iter().enumerate() {
        let path = format!("$[{index}]");
        if !selectors.insert(&patch.id) {
            return error(
                "duplicate_topic_selector",
                "overlay",
                format!("{path}.id"),
                format!("topic selector {:?} repeats", patch.id.0),
            );
        }
        let localized_collection_path = format!("{path}.localized");
        count_range(
            patch.localized.len(),
            1,
            LANGUAGE_LIMIT,
            "overlay",
            &localized_collection_path,
            "resource_limit:languages",
        )?;
        for (language, localized) in &patch.localized {
            let localized_path = format!("{path}.localized.{language}");
            validate_langid(language, "overlay", &localized_path)?;
            match localized {
                PatchField::Missing => {
                    return error(
                        "invalid_overlay_patch",
                        "overlay",
                        localized_path,
                        "a typed language entry cannot be Missing",
                    )
                }
                PatchField::Null => {}
                PatchField::Value(localized) => {
                    if matches!(localized.title, PatchField::Missing)
                        && matches!(localized.body, PatchField::Missing)
                        && matches!(localized.image, PatchField::Missing)
                    {
                        return error(
                            "empty_overlay_patch",
                            "overlay",
                            localized_path,
                            "localized patch must contain at least one field",
                        );
                    }
                    validate_typed_text_patch(
                        &localized.title,
                        TITLE_BYTES_LIMIT,
                        &format!("{localized_path}.title"),
                    )?;
                    validate_typed_text_patch(
                        &localized.body,
                        BODY_BYTES_LIMIT,
                        &format!("{localized_path}.body"),
                    )?;
                    validate_typed_image_patch(&localized.image, &localized_path)?;
                }
            }
        }
    }
    Ok(())
}

fn validate_typed_text_patch(
    patch: &PatchField<String>,
    byte_limit: usize,
    path: &str,
) -> Result<(), EncyclopediaError> {
    if let PatchField::Value(text) = patch {
        if text.len() > byte_limit {
            return error(
                if byte_limit == BODY_BYTES_LIMIT {
                    "resource_limit:body_bytes"
                } else {
                    "resource_limit:title_bytes"
                },
                "overlay",
                path,
                format!("{} bytes exceeds {byte_limit}", text.len()),
            );
        }
    }
    Ok(())
}

fn validate_typed_image_patch(
    patch: &PatchField<ImagePatch>,
    localized_path: &str,
) -> Result<(), EncyclopediaError> {
    let validate = |path: &str| {
        if valid_mod_image_path(path) {
            Ok(())
        } else {
            error(
                "unsafe_asset_path",
                "overlay",
                format!("{localized_path}.image"),
                format!("unsafe mod image path {path:?}"),
            )
        }
    };
    match patch {
        PatchField::Missing | PatchField::Null => Ok(()),
        PatchField::Value(ImagePatch::Static { path }) => validate(path),
        PatchField::Value(ImagePatch::ViewerFaction(pair)) => {
            for side in [&pair.alliance, &pair.empire] {
                if let FactionImagePatch::Value { path } = side {
                    validate(path)?;
                }
            }
            Ok(())
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TopicImageCapability {
    Static,
    ViewerFaction,
}

fn topic_capabilities(
    catalog: &EncyclopediaCatalog,
) -> Result<BTreeMap<TopicId, TopicImageCapability>, EncyclopediaError> {
    let mut capabilities = BTreeMap::new();
    for (index, binding) in catalog.bindings.iter().enumerate() {
        let capability = match binding.variant.as_str() {
            "default" => TopicImageCapability::Static,
            "viewer_faction" => TopicImageCapability::ViewerFaction,
            other => {
                return error(
                    "unsupported_variant",
                    "overlay",
                    format!("$.bindings[{index}].variant"),
                    format!("unsupported immutable topic capability {other:?}"),
                )
            }
        };
        capabilities.insert(binding.topic_id.clone(), capability);
    }
    Ok(capabilities)
}

#[allow(clippy::too_many_arguments)]
fn apply_localized_patch(
    existing: Option<&LocalizedContent>,
    patch: &LocalizedPatch,
    capability: TopicImageCapability,
    topic_id: &TopicId,
    path: &str,
    descriptors: &mut BTreeMap<BaseImageId, BaseImage>,
    effective_facts: &mut BTreeMap<String, AssetFacts>,
    context: &mut ImageContext<'_>,
) -> Result<LocalizedContent, EncyclopediaError> {
    let title = merge_required_text(
        existing.map(|content| &content.title),
        &patch.title,
        topic_id,
        &format!("{path}.title"),
    )?;
    let body = merge_required_text(
        existing.map(|content| &content.body),
        &patch.body,
        topic_id,
        &format!("{path}.body"),
    )?;

    let (image_id, image_selector) = match &patch.image {
        PatchField::Missing => existing.map_or((BaseImageIdField::Absent, None), |content| {
            (content.image_id.clone(), content.image_selector.clone())
        }),
        PatchField::Null => (BaseImageIdField::Null, None),
        PatchField::Value(ImagePatch::Static { path: image_path }) => {
            let image_id = prepare_image(image_path, path, descriptors, effective_facts, context)?;
            (BaseImageIdField::Value(image_id), None)
        }
        PatchField::Value(ImagePatch::ViewerFaction(pair)) => {
            if capability != TopicImageCapability::ViewerFaction {
                return Err(EncyclopediaError::new(
                    "image_override_capability",
                    "overlay",
                    format!("{path}.image"),
                    "immutable source-backed topic capability is not viewer_faction",
                )
                .with_topic(&topic_id.0));
            }
            let alliance_image_id =
                prepare_faction_side(&pair.alliance, path, descriptors, effective_facts, context)?;
            let empire_image_id =
                prepare_faction_side(&pair.empire, path, descriptors, effective_facts, context)?;
            (
                BaseImageIdField::Absent,
                Some(ImageSelector {
                    kind: "viewer_faction".to_owned(),
                    alliance_image_id,
                    empire_image_id,
                }),
            )
        }
    };

    Ok(LocalizedContent {
        title,
        body,
        image_id,
        image_selector,
    })
}

fn merge_required_text(
    existing: Option<&String>,
    patch: &PatchField<String>,
    topic_id: &TopicId,
    path: &str,
) -> Result<String, EncyclopediaError> {
    match patch {
        PatchField::Missing => existing.cloned().ok_or_else(|| {
            EncyclopediaError::new(
                "incomplete_localized_record",
                "overlay",
                path,
                "new localized record omits a required field",
            )
            .with_topic(&topic_id.0)
        }),
        PatchField::Null => Err(EncyclopediaError::new(
            "incomplete_localized_record",
            "overlay",
            path,
            "required localized field cannot be deleted",
        )
        .with_topic(&topic_id.0)),
        PatchField::Value(value) => Ok(value.clone()),
    }
}

fn prepare_faction_side(
    side: &FactionImagePatch,
    path: &str,
    descriptors: &mut BTreeMap<BaseImageId, BaseImage>,
    effective_facts: &mut BTreeMap<String, AssetFacts>,
    context: &mut ImageContext<'_>,
) -> Result<NullableBaseImageId, EncyclopediaError> {
    match side {
        FactionImagePatch::Null => Ok(NullableBaseImageId::Null),
        FactionImagePatch::Value { path: image_path } => {
            prepare_image(image_path, path, descriptors, effective_facts, context)
                .map(NullableBaseImageId::Value)
        }
    }
}

struct ImageContext<'a> {
    mod_name: &'a str,
    replacements: &'a BTreeMap<String, AssetFacts>,
    remaining_identity_bytes: usize,
    prepared: BTreeMap<String, BaseImageId>,
}

fn prepare_image(
    author_path: &str,
    localized_path: &str,
    descriptors: &mut BTreeMap<BaseImageId, BaseImage>,
    effective_facts: &mut BTreeMap<String, AssetFacts>,
    context: &mut ImageContext<'_>,
) -> Result<BaseImageId, EncyclopediaError> {
    if !valid_mod_image_path(author_path) {
        return error(
            "unsafe_asset_path",
            "overlay",
            format!("{localized_path}.image"),
            format!("unsafe mod image path {author_path:?}"),
        );
    }
    if let Some(image_id) = context.prepared.get(author_path) {
        return Ok(image_id.clone());
    }
    let facts = context.replacements.get(author_path).ok_or_else(|| {
        EncyclopediaError::new(
            "missing_image_provider",
            "overlay",
            format!("{localized_path}.image"),
            format!("no inspected facts for {author_path:?}"),
        )
    })?;
    let observed = facts.image.as_ref().ok_or_else(|| {
        EncyclopediaError::new(
            "image_facts_mismatch",
            "overlay",
            format!("{localized_path}.image"),
            "replacement facts do not describe a decoded image",
        )
    })?;
    let expected_format = author_path
        .rsplit_once('.')
        .map(|(_, extension)| extension)
        .unwrap_or_default();
    if observed.format != expected_format || !matches!(observed.format.as_str(), "bmp" | "png") {
        return error(
            "image_format_mismatch",
            "overlay",
            format!("{localized_path}.image"),
            "decoded image format does not match the confined path extension",
        );
    }
    if facts.sha256.len() != 64
        || !facts
            .sha256
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return error(
            "image_facts_mismatch",
            "overlay",
            format!("{localized_path}.image"),
            "replacement facts contain an invalid SHA-256 identity",
        );
    }

    let (identity_len, reservation) =
        identity_reservation(context.mod_name.len(), author_path.len()).ok_or_else(|| {
            EncyclopediaError::new(
                "identity_length_overflow",
                "overlay",
                format!("{localized_path}.image"),
                "generated image identity length overflowed",
            )
        })?;
    if reservation > context.remaining_identity_bytes {
        return error(
            "resource_limit:retained_bytes",
            "overlay",
            format!("{localized_path}.image"),
            format!(
                "generated identity and retained owner/path require {reservation} bytes, {} remain",
                context.remaining_identity_bytes
            ),
        );
    }

    let image_id = build_mod_image_id(context.mod_name, author_path, identity_len);
    let runtime_path = author_path
        .strip_prefix("encyclopedia/")
        .expect("validated paths always use the encyclopedia prefix")
        .to_owned();
    let descriptor = BaseImage {
        path: runtime_path,
        format: observed.format.clone(),
        byte_length: facts.byte_len,
        width: observed.width,
        height: observed.height,
        sha256: facts.sha256.clone(),
        // Effective descriptors are never serialized into or closed through
        // the immutable base manifest. The adapter retains real owner/path.
        source_ref: "runtime:mod-snapshot".to_owned(),
    };
    let typed_id = BaseImageId(image_id.clone());
    if let Some(existing) = descriptors.get(&typed_id) {
        if existing != &descriptor || effective_facts.get(&image_id) != Some(facts) {
            return error(
                "asset_identity_collision",
                "overlay",
                format!("{localized_path}.image"),
                format!("generated canonical image ID {image_id:?} is already owned"),
            );
        }
    } else {
        descriptors.insert(typed_id.clone(), descriptor);
        effective_facts.insert(image_id, facts.clone());
    }
    context.remaining_identity_bytes -= reservation;
    context
        .prepared
        .insert(author_path.to_owned(), typed_id.clone());
    Ok(typed_id)
}

fn identity_reservation(name_len: usize, path_len: usize) -> Option<(usize, usize)> {
    let identity_len = name_len
        .checked_mul(2)?
        .checked_add(MOD_IMAGE_PREFIX.len())?
        .checked_add(1)?
        .checked_add(path_len)?;
    let reservation = identity_len.checked_add(name_len)?.checked_add(path_len)?;
    Some((identity_len, reservation))
}

fn build_mod_image_id(mod_name: &str, path: &str, length: usize) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut identity = String::with_capacity(length);
    identity.push_str(MOD_IMAGE_PREFIX);
    for byte in mod_name.as_bytes() {
        identity.push(char::from(HEX[usize::from(byte >> 4)]));
        identity.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    identity.push(':');
    identity.push_str(path);
    debug_assert_eq!(identity.len(), length);
    identity
}

fn prune_unreferenced_mod_images(
    catalog: &mut EncyclopediaCatalog,
    facts: &mut BTreeMap<String, AssetFacts>,
) {
    let mut referenced = BTreeSet::new();
    for topic in catalog.topics.values() {
        for localized in topic.localized.values() {
            if let BaseImageIdField::Value(image_id) = &localized.image_id {
                referenced.insert(image_id.0.clone());
            }
            if let Some(selector) = &localized.image_selector {
                for side in [&selector.alliance_image_id, &selector.empire_image_id] {
                    if let NullableBaseImageId::Value(image_id) = side {
                        referenced.insert(image_id.0.clone());
                    }
                }
            }
        }
    }
    catalog.images.retain(|image_id, _| {
        !image_id.0.starts_with(MOD_IMAGE_PREFIX) || referenced.contains(&image_id.0)
    });
    facts.retain(|image_id, _| catalog.images.contains_key(image_id.as_str()));
}

#[cfg(test)]
mod tests {
    use super::identity_reservation;

    #[test]
    fn generated_identity_length_overflow_is_detected_before_allocation() {
        assert_eq!(identity_reservation(0, 0), Some((8, 8)));
        assert_eq!(identity_reservation(usize::MAX, 256), None);
    }
}
