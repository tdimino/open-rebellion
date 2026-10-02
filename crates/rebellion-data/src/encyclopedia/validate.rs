use std::collections::{BTreeMap, BTreeSet};

use super::{
    valid_runtime_asset_path, AssetFacts, BaseImageIdField, EncyclopediaCatalog, EncyclopediaError,
    EncyclopediaManifest, NullableBaseImageId, TopicId, BODY_BYTES_LIMIT,
    TITLE_OR_LABEL_BYTES_LIMIT, TOPIC_LIMIT, TOPIC_SORT_V1,
};

const MAX_IMAGE_BYTES: u64 = 32 * 1024 * 1024;
const MAX_IMAGE_PIXELS: u64 = 16_000_000;
const MAX_EFFECTIVE_IMAGE_BYTES: u64 = 128 * 1024 * 1024;

/// Validates an immutable base bundle from already parsed wire values and
/// caller-supplied observations. This function performs no file I/O, hashing,
/// or image decoding: `files` and `base_dats` must describe bytes the caller
/// has independently inspected.
pub fn validate_bundle(
    catalog: &EncyclopediaCatalog,
    manifest: &EncyclopediaManifest,
    files: &BTreeMap<String, AssetFacts>,
    base_dats: &BTreeMap<String, String>,
) -> Result<(), EncyclopediaError> {
    validate_effective_content(catalog)?;
    validate_views(catalog)?;
    validate_bindings(catalog)?;
    validate_base_selector_capabilities(catalog)?;
    validate_base_image_allowlist(catalog)?;
    validate_source_ref_closure(catalog, manifest)?;
    validate_binding_sources(manifest, base_dats)?;

    let mut wanted_paths = BTreeSet::from(["catalog.json".to_owned()]);
    wanted_paths.extend(catalog.images.values().map(|image| image.path.clone()));
    let manifest_paths: BTreeSet<_> = manifest.files.keys().cloned().collect();
    let observed_paths: BTreeSet<_> = files.keys().cloned().collect();
    if manifest_paths != wanted_paths || observed_paths != wanted_paths {
        return fail(
            "manifest_file_set_mismatch",
            "bundle",
            "$.files",
            "manifest, observed, and catalog-required runtime file sets differ",
        );
    }

    let catalog_manifest_digest = manifest.files.get("catalog.json").ok_or_else(|| {
        EncyclopediaError::new(
            "manifest_file_set_mismatch",
            "bundle",
            "$.files.catalog.json",
            "manifest omits catalog.json",
        )
    })?;
    let catalog_facts = files.get("catalog.json").ok_or_else(|| {
        EncyclopediaError::new(
            "manifest_file_set_mismatch",
            "bundle",
            "catalog.json",
            "observed facts omit catalog.json",
        )
    })?;
    if manifest.catalog_sha256 != *catalog_manifest_digest
        || manifest.catalog_sha256 != catalog_facts.sha256
    {
        return fail(
            "catalog_digest_mismatch",
            "bundle",
            "catalog.json",
            "catalog observation does not match both immutable manifest digests",
        );
    }
    if catalog_facts.image.is_some() {
        return fail(
            "file_digest_mismatch",
            "bundle",
            "catalog.json",
            "catalog.json was reported as an image",
        );
    }

    for (image_id, descriptor) in &catalog.images {
        if descriptor.format != "bmp" {
            return fail(
                "unsupported_base_image_format",
                "bundle",
                format!("$.images.{}.format", image_id.0),
                "immutable v1 base art must be BMP",
            );
        }
        let facts = &files[&descriptor.path];
        let manifest_digest = &manifest.files[&descriptor.path];
        if facts.sha256 != *manifest_digest {
            return fail(
                "file_digest_mismatch",
                "bundle",
                descriptor.path.clone(),
                "observed image digest differs from the immutable manifest",
            );
        }
    }
    let effective_facts = effective_facts_from_files(catalog, files)?;
    validate_effective_catalog(catalog, &effective_facts)?;
    Ok(())
}

fn validate_base_image_allowlist(catalog: &EncyclopediaCatalog) -> Result<(), EncyclopediaError> {
    let mut paths = BTreeMap::<&str, &str>::new();
    for (image_id, descriptor) in &catalog.images {
        if let Some(previous) = paths.insert(&descriptor.path, &image_id.0) {
            return fail(
                "asset_identity_collision",
                "bundle",
                format!("$.images.{}.path", image_id.0),
                format!("{previous:?} and {:?} share a base bundle path", image_id.0),
            );
        }
    }

    let referenced = referenced_images(catalog);
    if let Some(unused) = catalog
        .images
        .keys()
        .map(|image_id| &image_id.0)
        .find(|image_id| !referenced.contains(*image_id))
    {
        return fail(
            "unexpected_runtime_file",
            "bundle",
            format!("$.images.{unused}"),
            "immutable base catalog describes an unreferenced runtime image",
        );
    }
    Ok(())
}

fn validate_effective_content(catalog: &EncyclopediaCatalog) -> Result<(), EncyclopediaError> {
    for (field, expected) in TOPIC_SORT_V1 {
        let actual = match field {
            "algorithm" => &catalog.topic_sort.algorithm,
            "representable_encoding" => &catalog.topic_sort.representable_encoding,
            "representable_fold" => &catalog.topic_sort.representable_fold,
            "unrepresentable" => &catalog.topic_sort.unrepresentable,
            "tie_break" => &catalog.topic_sort.tie_break,
            _ => unreachable!(),
        };
        if actual != expected {
            return fail(
                "invalid_topic_sort",
                "effective_catalog",
                format!("$.topic_sort.{field}"),
                format!("expected {expected:?}, got {actual:?}"),
            );
        }
    }

    if catalog.topics.is_empty() || catalog.topics.len() > TOPIC_LIMIT {
        return fail(
            "resource_limit:topics",
            "effective_catalog",
            "$.topics",
            "effective topic count is outside v1 bounds",
        );
    }
    for (topic_id, topic) in &catalog.topics {
        if topic.localized.is_empty() || topic.localized.len() > 256 {
            return fail(
                "invalid_localized_record",
                "effective_catalog",
                format!("$.topics.{}.localized", topic_id.0),
                "localized record count is outside v1 bounds",
            );
        }
        for (language, localized) in &topic.localized {
            let base = format!("$.topics.{}.localized.{language}", topic_id.0);
            if localized.title.len() > TITLE_OR_LABEL_BYTES_LIMIT {
                return fail(
                    "resource_limit:title_bytes",
                    "effective_catalog",
                    format!("{base}.title"),
                    "effective title exceeds the UTF-8 byte limit",
                );
            }
            if localized.body.len() > BODY_BYTES_LIMIT {
                return fail(
                    "resource_limit:body_bytes",
                    "effective_catalog",
                    format!("{base}.body"),
                    "effective body exceeds the UTF-8 byte limit",
                );
            }
            if localized.image_selector.is_some()
                && !matches!(localized.image_id, BaseImageIdField::Absent)
            {
                return fail(
                    "ambiguous_image_selector",
                    "effective_catalog",
                    &base,
                    "image_id and image_selector are mutually exclusive",
                );
            }
            if let Some(selector) = &localized.image_selector {
                if selector.kind != "viewer_faction" {
                    return fail(
                        "unsupported_selector",
                        "effective_catalog",
                        format!("{base}.image_selector.kind"),
                        "effective selector kind is unsupported",
                    );
                }
            }
        }
    }
    Ok(())
}

/// Validates relationships and observed image facts for a candidate catalog.
/// The map is keyed by canonical catalog image ID. Unlike `validate_bundle`,
/// this entry point intentionally does not compare the candidate to the base
/// manifest or reapply immutable base selector shape to authorized overrides.
/// Distinct owner-qualified IDs may use the same confined author-relative path;
/// E22 must derive those IDs, reject duplicate IDs before `BTreeMap` insertion,
/// and retain the owner/path association. This layer rechecks safe normalized
/// paths and requires facts to close exactly by canonical ID.
pub fn validate_effective_catalog(
    catalog: &EncyclopediaCatalog,
    images: &BTreeMap<String, AssetFacts>,
) -> Result<(), EncyclopediaError> {
    validate_effective_content(catalog)?;
    validate_views(catalog)?;
    validate_bindings(catalog)?;

    let referenced = referenced_images(catalog);
    let described: BTreeSet<_> = catalog.images.keys().map(|id| id.0.clone()).collect();
    if let Some(missing) = referenced.difference(&described).next() {
        return fail(
            "dangling_image_reference",
            "effective_catalog",
            "$.images",
            format!("catalog refers to missing image {missing:?}"),
        );
    }

    let observed: BTreeSet<_> = images.keys().cloned().collect();
    if observed != described {
        return fail(
            "unexpected_runtime_file",
            "effective_catalog",
            "$.images",
            "effective image observations do not exactly match catalog image IDs",
        );
    }

    let mut aggregate_bytes = 0_u64;
    for (image_id, descriptor) in &catalog.images {
        if !valid_runtime_asset_path(&descriptor.path) {
            return fail(
                "unsafe_asset_path",
                "effective_catalog",
                format!("$.images.{}.path", image_id.0),
                format!("unsafe runtime asset path {:?}", descriptor.path),
            );
        }
        let facts = &images[&image_id.0];
        let image = facts.image.as_ref().ok_or_else(|| {
            EncyclopediaError::new(
                "image_facts_mismatch",
                "effective_catalog",
                format!("$.images.{}", image_id.0),
                "retained bytes lack decoded image facts",
            )
        })?;
        if !matches!(image.format.as_str(), "bmp" | "png") {
            return fail(
                "unsupported_image_format",
                "effective_catalog",
                format!("$.images.{}.format", image_id.0),
                format!("unsupported observed format {:?}", image.format),
            );
        }
        if facts.byte_len == 0 || facts.byte_len > MAX_IMAGE_BYTES {
            return fail(
                "resource_limit:image_bytes",
                "effective_catalog",
                format!("$.images.{}.byte_length", image_id.0),
                "observed image byte length is outside v1 bounds",
            );
        }
        let pixels = u64::from(image.width)
            .checked_mul(u64::from(image.height))
            .ok_or_else(|| {
                EncyclopediaError::new(
                    "resource_limit:image_pixels",
                    "effective_catalog",
                    format!("$.images.{}", image_id.0),
                    "image pixel count overflowed",
                )
            })?;
        if image.width == 0 || image.height == 0 || pixels > MAX_IMAGE_PIXELS {
            return fail(
                "resource_limit:image_pixels",
                "effective_catalog",
                format!("$.images.{}", image_id.0),
                format!("observed image has {pixels} pixels"),
            );
        }
        if descriptor.format != image.format {
            return fail(
                "image_format_mismatch",
                "effective_catalog",
                format!("$.images.{}.format", image_id.0),
                "descriptor and observed image formats differ",
            );
        }
        if descriptor.sha256 != facts.sha256 {
            return fail(
                "image_digest_mismatch",
                "effective_catalog",
                format!("$.images.{}.sha256", image_id.0),
                "descriptor and observed image digests differ",
            );
        }
        if descriptor.byte_length != facts.byte_len
            || descriptor.width != image.width
            || descriptor.height != image.height
        {
            return fail(
                "image_facts_mismatch",
                "effective_catalog",
                format!("$.images.{}", image_id.0),
                "descriptor length or dimensions differ from observed facts",
            );
        }
        aggregate_bytes = aggregate_bytes.checked_add(facts.byte_len).ok_or_else(|| {
            EncyclopediaError::new(
                "resource_limit:effective_image_bytes",
                "effective_catalog",
                "$.images",
                "effective image byte total overflowed",
            )
        })?;
        if aggregate_bytes > MAX_EFFECTIVE_IMAGE_BYTES {
            return fail(
                "resource_limit:effective_image_bytes",
                "effective_catalog",
                "$.images",
                "effective image bytes exceed the 128 MiB logical asset-set limit",
            );
        }
    }
    Ok(())
}

fn validate_views(catalog: &EncyclopediaCatalog) -> Result<(), EncyclopediaError> {
    if catalog.index.command != "0x6f" {
        return fail(
            "category_order",
            "effective_catalog",
            "$.index.command",
            "v1 aggregate command must be 0x6f",
        );
    }
    if catalog.categories.len() != 6 {
        return fail(
            "category_order",
            "effective_catalog",
            "$.categories",
            "v1 requires the six recovered filtered commands",
        );
    }
    let mut commands = BTreeSet::new();
    for (index, category) in catalog.categories.iter().enumerate() {
        if !commands.insert(&category.command) {
            return fail(
                "duplicate_category_command",
                "effective_catalog",
                format!("$.categories[{index}].command"),
                "category command repeats",
            );
        }
        let expected = format!("0x{:x}", 0x70 + index);
        if category.command != expected || category.id != format!("command:{expected}") {
            return fail(
                "category_order",
                "effective_catalog",
                format!("$.categories[{index}]"),
                "category ID/command is not in recovered tab order",
            );
        }
    }

    let topic_ids: BTreeSet<_> = catalog.topics.keys().cloned().collect();
    let mut index_ids = BTreeSet::new();
    for topic_id in &catalog.index.topic_ids {
        if !catalog.topics.contains_key(topic_id) {
            return dangling_topic("$.index.topic_ids", topic_id);
        }
        if !index_ids.insert(topic_id.clone()) {
            return fail(
                "duplicate_topic_id",
                "effective_catalog",
                "$.index.topic_ids",
                format!("aggregate repeats topic {:?}", topic_id.0),
            );
        }
    }
    if index_ids != topic_ids {
        return fail(
            "potential_membership_mismatch",
            "effective_catalog",
            "$.index.topic_ids",
            "aggregate membership must contain every catalog candidate",
        );
    }

    let mut filtered = BTreeMap::<TopicId, &str>::new();
    for category in &catalog.categories {
        let mut category_topics = BTreeSet::new();
        for topic_id in &category.topic_ids {
            if !catalog.topics.contains_key(topic_id) {
                return dangling_topic("$.categories", topic_id);
            }
            if !category_topics.insert(topic_id) {
                return fail(
                    "duplicate_topic_id",
                    "effective_catalog",
                    "$.categories",
                    format!("{} repeats topic {:?}", category.command, topic_id.0),
                );
            }
            if !index_ids.contains(topic_id) {
                return fail(
                    "potential_membership_mismatch",
                    "effective_catalog",
                    "$.categories",
                    format!(
                        "filtered topic {:?} is absent from the aggregate",
                        topic_id.0
                    ),
                );
            }
            if let Some(previous) = filtered.insert(topic_id.clone(), &category.command) {
                return fail(
                    "filtered_membership_mismatch",
                    "effective_catalog",
                    "$.categories",
                    format!(
                        "topic {:?} appears in both {previous} and {}",
                        topic_id.0, category.command
                    ),
                );
            }
        }
    }
    Ok(())
}

fn validate_bindings(catalog: &EncyclopediaCatalog) -> Result<(), EncyclopediaError> {
    let mut keys = BTreeSet::new();
    let mut topics = BTreeMap::<TopicId, usize>::new();
    for (index, binding) in catalog.bindings.iter().enumerate() {
        if !catalog.topics.contains_key(&binding.topic_id) {
            return dangling_topic(&format!("$.bindings[{index}].topic_id"), &binding.topic_id);
        }
        if !keys.insert((
            binding.family.as_str(),
            binding.dat_id,
            binding.variant.as_str(),
        )) {
            return fail(
                "ambiguous_binding",
                "effective_catalog",
                format!("$.bindings[{index}]"),
                "binding tuple repeats",
            );
        }
        if topics.insert(binding.topic_id.clone(), index).is_some() {
            return fail(
                "ambiguous_binding",
                "effective_catalog",
                format!("$.bindings[{index}].topic_id"),
                "one topic has multiple binding tuples",
            );
        }
    }
    if let Some(unbound) = catalog
        .topics
        .keys()
        .find(|topic_id| !topics.contains_key(*topic_id))
    {
        return dangling_topic("$.bindings", unbound);
    }
    Ok(())
}

fn referenced_images(catalog: &EncyclopediaCatalog) -> BTreeSet<String> {
    let mut referenced = BTreeSet::new();
    for topic in catalog.topics.values() {
        for localized in topic.localized.values() {
            if let BaseImageIdField::Value(image_id) = &localized.image_id {
                referenced.insert(image_id.0.clone());
            }
            if let Some(selector) = &localized.image_selector {
                for image_id in [&selector.alliance_image_id, &selector.empire_image_id] {
                    if let NullableBaseImageId::Value(image_id) = image_id {
                        referenced.insert(image_id.0.clone());
                    }
                }
            }
        }
    }
    referenced
}

fn effective_facts_from_files(
    catalog: &EncyclopediaCatalog,
    files: &BTreeMap<String, AssetFacts>,
) -> Result<BTreeMap<String, AssetFacts>, EncyclopediaError> {
    let mut result = BTreeMap::new();
    for (image_id, descriptor) in &catalog.images {
        let facts = files.get(&descriptor.path).ok_or_else(|| {
            EncyclopediaError::new(
                "manifest_file_set_mismatch",
                "bundle",
                descriptor.path.clone(),
                "observed facts omit a catalog image path",
            )
        })?;
        result.insert(image_id.0.clone(), facts.clone());
    }
    Ok(result)
}

fn validate_base_selector_capabilities(
    catalog: &EncyclopediaCatalog,
) -> Result<(), EncyclopediaError> {
    let by_topic: BTreeMap<_, _> = catalog
        .bindings
        .iter()
        .map(|binding| (&binding.topic_id, binding))
        .collect();
    for (topic_id, topic) in &catalog.topics {
        let binding = by_topic.get(topic_id).ok_or_else(|| {
            EncyclopediaError::new(
                "dangling_topic_reference",
                "bundle",
                "$.bindings",
                format!("topic {:?} has no binding", topic_id.0),
            )
        })?;
        for (language, localized) in &topic.localized {
            let has_selector = localized.image_selector.is_some();
            if (binding.variant == "viewer_faction") != has_selector {
                return fail(
                    "binding_selector_mismatch",
                    "bundle",
                    format!("$.topics.{}.localized.{language}", topic_id.0),
                    "immutable binding capability and localized selector disagree",
                );
            }
        }
    }
    Ok(())
}

fn validate_source_ref_closure(
    catalog: &EncyclopediaCatalog,
    manifest: &EncyclopediaManifest,
) -> Result<(), EncyclopediaError> {
    let references = std::iter::once((&catalog.index.source_ref, "$.index.source_ref".to_owned()))
        .chain(
            catalog
                .categories
                .iter()
                .enumerate()
                .map(|(index, category)| {
                    (
                        &category.source_ref,
                        format!("$.categories[{index}].source_ref"),
                    )
                }),
        )
        .chain(catalog.topics.iter().map(|(topic_id, topic)| {
            (
                &topic.source_ref,
                format!("$.topics.{}.source_ref", topic_id.0),
            )
        }))
        .chain(catalog.images.iter().map(|(image_id, image)| {
            (
                &image.source_ref,
                format!("$.images.{}.source_ref", image_id.0),
            )
        }));
    for (source_ref, path) in references {
        if !manifest.source_records.contains_key(source_ref) {
            return fail(
                "dangling_source_ref",
                "bundle",
                path,
                format!("manifest has no source record for {source_ref:?}"),
            );
        }
    }
    Ok(())
}

fn validate_binding_sources(
    manifest: &EncyclopediaManifest,
    base_dats: &BTreeMap<String, String>,
) -> Result<(), EncyclopediaError> {
    let mut observed = BTreeMap::new();
    for (basename, digest) in base_dats {
        let folded = basename.to_ascii_lowercase();
        if observed.insert(folded, digest).is_some() {
            return fail(
                "binding_source_mismatch",
                "bundle",
                "base_dats",
                format!("observed DAT basename {basename:?} is ambiguous"),
            );
        }
    }
    let mut required = BTreeSet::new();
    for (index, source) in manifest.binding_sources.iter().enumerate() {
        let folded = source.basename.to_ascii_lowercase();
        if !required.insert(folded.clone()) {
            return fail(
                "binding_source_mismatch",
                "bundle",
                format!("$.binding_sources[{index}].basename"),
                "binding source basename repeats case-insensitively",
            );
        }
        if observed.get(&folded).copied() != Some(&source.sha256) {
            return fail(
                "binding_source_mismatch",
                "bundle",
                format!("$.binding_sources[{index}]"),
                format!("{} does not match observed DAT facts", source.basename),
            );
        }
    }
    Ok(())
}

fn dangling_topic<T>(path: &str, topic_id: &TopicId) -> Result<T, EncyclopediaError> {
    Err(EncyclopediaError::new(
        "dangling_topic_reference",
        "effective_catalog",
        path,
        format!("catalog refers to missing topic {:?}", topic_id.0),
    )
    .with_topic(&topic_id.0))
}

fn fail<T>(
    code: &'static str,
    source: &'static str,
    path: impl Into<String>,
    detail: impl Into<String>,
) -> Result<T, EncyclopediaError> {
    Err(EncyclopediaError::new(code, source, path, detail))
}
