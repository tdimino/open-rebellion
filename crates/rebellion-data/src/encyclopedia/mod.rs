mod model;

pub use model::{
    AssetFacts, BaseImage, BaseImageId, BaseImageIdField, BindingKey, BindingSource,
    CatalogBinding, CatalogCategory, CatalogIndex, EncyclopediaCatalog, EncyclopediaManifest,
    ImageFacts, ImageSelector, LocalizedContent, NullableBaseImageId, ResourceIdentifier,
    SourceRecord, Topic, TopicId, TopicSort,
};

use std::collections::HashSet;
use std::fmt;
use std::str;

use serde::de::IgnoredAny;
use serde::Deserialize;
use serde_json::{Map, Value};

pub const CATALOG_JSON_BYTES_LIMIT: usize = 64 * 1024 * 1024;
pub const MANIFEST_JSON_BYTES_LIMIT: usize = 32 * 1024 * 1024;
pub const CATALOG_JSON_DEPTH_LIMIT: usize = 16;
pub const MANIFEST_JSON_DEPTH_LIMIT: usize = 16;

const TOPIC_LIMIT: usize = 10_000;
const TITLE_OR_LABEL_BYTES_LIMIT: usize = 65_536;
const BODY_BYTES_LIMIT: usize = 1_048_576;
const IMAGE_BYTES_LIMIT: u64 = 33_554_432;
const IMAGE_DIMENSION_LIMIT: u64 = 16_000_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaError {
    code: &'static str,
    source: &'static str,
    path: String,
    topic_id: Option<TopicId>,
    detail: String,
}

impl EncyclopediaError {
    fn new(
        code: &'static str,
        source: &'static str,
        path: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            source,
            path: path.into(),
            topic_id: None,
            detail: detail.into(),
        }
    }

    fn with_topic(mut self, topic_id: &str) -> Self {
        self.topic_id = Some(TopicId(topic_id.to_owned()));
        self
    }

    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.code
    }

    #[must_use]
    pub const fn source(&self) -> &'static str {
        self.source
    }

    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    #[must_use]
    pub fn topic_id(&self) -> Option<&TopicId> {
        self.topic_id.as_ref()
    }
}

impl fmt::Display for EncyclopediaError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} in {} at {}: {}",
            self.code, self.source, self.path, self.detail
        )
    }
}

impl std::error::Error for EncyclopediaError {}

pub fn parse_catalog(bytes: &[u8]) -> Result<EncyclopediaCatalog, EncyclopediaError> {
    let value = parse_raw_document(
        bytes,
        "catalog",
        CATALOG_JSON_BYTES_LIMIT,
        CATALOG_JSON_DEPTH_LIMIT,
    )?;
    validate_catalog(&value)?;
    serde_json::from_value(value).map_err(|error| {
        EncyclopediaError::new(
            "invalid_type",
            "catalog",
            "$",
            format!("validated catalog could not be typed: {error}"),
        )
    })
}

pub fn parse_manifest(bytes: &[u8]) -> Result<EncyclopediaManifest, EncyclopediaError> {
    let value = parse_raw_document(
        bytes,
        "manifest",
        MANIFEST_JSON_BYTES_LIMIT,
        MANIFEST_JSON_DEPTH_LIMIT,
    )?;
    validate_manifest(&value)?;
    serde_json::from_value(value).map_err(|error| {
        EncyclopediaError::new(
            "invalid_type",
            "manifest",
            "$",
            format!("validated manifest could not be typed: {error}"),
        )
    })
}

fn parse_raw_document(
    bytes: &[u8],
    source: &'static str,
    byte_limit: usize,
    depth_limit: usize,
) -> Result<Value, EncyclopediaError> {
    if bytes.len() > byte_limit {
        return Err(EncyclopediaError::new(
            "resource_limit:json_bytes",
            source,
            "$",
            format!("{} bytes exceeds {byte_limit}", bytes.len()),
        ));
    }
    str::from_utf8(bytes).map_err(|error| {
        EncyclopediaError::new(
            "invalid_utf8",
            source,
            "$",
            format!("invalid UTF-8 at byte {}", error.valid_up_to()),
        )
    })?;
    RawJsonParser::new(bytes, source, depth_limit).parse()?;
    serde_json::from_slice(bytes)
        .map_err(|error| EncyclopediaError::new("invalid_json", source, "$", error.to_string()))
}

struct RawJsonParser<'a> {
    bytes: &'a [u8],
    position: usize,
    source: &'static str,
    max_depth: usize,
}

impl<'a> RawJsonParser<'a> {
    const fn new(bytes: &'a [u8], source: &'static str, max_depth: usize) -> Self {
        Self {
            bytes,
            position: 0,
            source,
            max_depth,
        }
    }

    fn parse(mut self) -> Result<(), EncyclopediaError> {
        self.skip_whitespace();
        self.parse_value(1)?;
        self.skip_whitespace();
        if self.position != self.bytes.len() {
            return self.invalid_json("trailing JSON bytes");
        }
        Ok(())
    }

    fn skip_whitespace(&mut self) {
        while matches!(
            self.bytes.get(self.position),
            Some(b' ' | b'\t' | b'\n' | b'\r')
        ) {
            self.position += 1;
        }
    }

    fn parse_value(&mut self, depth: usize) -> Result<(), EncyclopediaError> {
        self.skip_whitespace();
        match self.bytes.get(self.position) {
            Some(b'{') => self.parse_object(depth),
            Some(b'[') => self.parse_array(depth),
            Some(b'"') => self.parse_string().map(|_| ()),
            Some(_) => self.parse_primitive(),
            None => self.invalid_json("expected a JSON value"),
        }
    }

    fn check_depth(&self, depth: usize) -> Result<(), EncyclopediaError> {
        if depth > self.max_depth {
            return Err(EncyclopediaError::new(
                "resource_limit:json_depth",
                self.source,
                "$",
                format!("JSON depth {depth} exceeds {}", self.max_depth),
            ));
        }
        Ok(())
    }

    fn parse_object(&mut self, depth: usize) -> Result<(), EncyclopediaError> {
        self.check_depth(depth)?;
        self.position += 1;
        self.skip_whitespace();
        let mut keys = HashSet::new();
        if self.bytes.get(self.position) == Some(&b'}') {
            self.position += 1;
            return Ok(());
        }
        loop {
            if self.bytes.get(self.position) != Some(&b'"') {
                return self.invalid_json("object key is not a string");
            }
            let key = self.parse_string()?;
            if !keys.insert(key.clone()) {
                return Err(EncyclopediaError::new(
                    "duplicate_key",
                    self.source,
                    "$",
                    format!("duplicate JSON key {key:?}"),
                ));
            }
            self.skip_whitespace();
            if self.bytes.get(self.position) != Some(&b':') {
                return self.invalid_json("object colon is missing");
            }
            self.position += 1;
            self.parse_value(depth + 1)?;
            self.skip_whitespace();
            match self.bytes.get(self.position) {
                Some(b'}') => {
                    self.position += 1;
                    return Ok(());
                }
                Some(b',') => {
                    self.position += 1;
                    self.skip_whitespace();
                }
                _ => return self.invalid_json("object comma is missing"),
            }
        }
    }

    fn parse_array(&mut self, depth: usize) -> Result<(), EncyclopediaError> {
        self.check_depth(depth)?;
        self.position += 1;
        self.skip_whitespace();
        if self.bytes.get(self.position) == Some(&b']') {
            self.position += 1;
            return Ok(());
        }
        loop {
            self.parse_value(depth + 1)?;
            self.skip_whitespace();
            match self.bytes.get(self.position) {
                Some(b']') => {
                    self.position += 1;
                    return Ok(());
                }
                Some(b',') => {
                    self.position += 1;
                    self.skip_whitespace();
                }
                _ => return self.invalid_json("array comma is missing"),
            }
        }
    }

    fn parse_string(&mut self) -> Result<String, EncyclopediaError> {
        let start = self.position;
        self.position += 1;
        while let Some(byte) = self.bytes.get(self.position).copied() {
            match byte {
                b'"' => {
                    self.position += 1;
                    return serde_json::from_slice(&self.bytes[start..self.position]).map_err(
                        |error| {
                            EncyclopediaError::new(
                                "invalid_json",
                                self.source,
                                "$",
                                format!("invalid JSON string: {error}"),
                            )
                        },
                    );
                }
                b'\\' => {
                    self.position += 1;
                    if self.position >= self.bytes.len() {
                        return self.invalid_json("JSON string escape is truncated");
                    }
                    self.position += 1;
                }
                _ => self.position += 1,
            }
        }
        self.invalid_json("JSON string is unterminated")
    }

    fn parse_primitive(&mut self) -> Result<(), EncyclopediaError> {
        let start = self.position;
        while let Some(byte) = self.bytes.get(self.position) {
            if matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | b',' | b']' | b'}') {
                break;
            }
            self.position += 1;
        }
        if start == self.position {
            return self.invalid_json("unexpected JSON token");
        }
        let mut deserializer =
            serde_json::Deserializer::from_slice(&self.bytes[start..self.position]);
        IgnoredAny::deserialize(&mut deserializer)
            .and_then(|_| deserializer.end())
            .map_err(|error| {
                EncyclopediaError::new(
                    "invalid_json",
                    self.source,
                    "$",
                    format!("invalid JSON primitive: {error}"),
                )
            })
    }

    fn invalid_json<T>(&self, detail: impl Into<String>) -> Result<T, EncyclopediaError> {
        Err(EncyclopediaError::new(
            "invalid_json",
            self.source,
            "$",
            detail,
        ))
    }
}

fn validate_catalog(value: &Value) -> Result<(), EncyclopediaError> {
    let root = object(value, "catalog", "$")?;
    fields(
        root,
        &[
            "schema_version",
            "default_language",
            "topic_sort",
            "index",
            "categories",
            "topics",
            "images",
            "bindings",
        ],
        &[],
        "catalog",
        "$",
    )?;
    let version = unsigned(
        root.get("schema_version").unwrap(),
        "catalog",
        "$.schema_version",
    )?;
    if version != 1 {
        return error(
            "unsupported_version",
            "catalog",
            "$.schema_version",
            format!("schema version {version} is not supported"),
        );
    }
    validate_langid(
        string(
            root.get("default_language").unwrap(),
            "catalog",
            "$.default_language",
        )?,
        "catalog",
        "$.default_language",
    )?;
    validate_topic_sort(root.get("topic_sort").unwrap())?;
    validate_index(root.get("index").unwrap())?;

    let categories = array(root.get("categories").unwrap(), "catalog", "$.categories")?;
    count_range(
        categories.len(),
        1,
        256,
        "catalog",
        "$.categories",
        "resource_limit:categories",
    )?;
    for (index, category) in categories.iter().enumerate() {
        validate_category(category, index)?;
    }
    for left in 0..categories.len() {
        if categories[left + 1..].contains(&categories[left]) {
            return error(
                "duplicate_category",
                "catalog",
                "$.categories",
                "duplicate category object",
            );
        }
    }

    let topics = object(root.get("topics").unwrap(), "catalog", "$.topics")?;
    count_range(
        topics.len(),
        1,
        TOPIC_LIMIT,
        "catalog",
        "$.topics",
        "resource_limit:topics",
    )?;
    for (topic_id, topic) in topics {
        validate_stable_id(
            topic_id,
            "invalid_topic_id",
            "catalog",
            &format!("$.topics.{topic_id}"),
        )?;
        validate_topic(topic, topic_id)?;
    }

    let images = object(root.get("images").unwrap(), "catalog", "$.images")?;
    count_range(
        images.len(),
        0,
        20_000,
        "catalog",
        "$.images",
        "resource_limit:images",
    )?;
    for (image_id, image) in images {
        validate_base_image_id(image_id, "catalog", &format!("$.images.{image_id}"))?;
        validate_base_image(image, image_id)?;
    }

    let bindings = array(root.get("bindings").unwrap(), "catalog", "$.bindings")?;
    count_range(
        bindings.len(),
        0,
        40_000,
        "catalog",
        "$.bindings",
        "resource_limit:bindings",
    )?;
    for (index, binding) in bindings.iter().enumerate() {
        validate_binding(binding, index)?;
    }
    Ok(())
}

fn validate_topic_sort(value: &Value) -> Result<(), EncyclopediaError> {
    let path = "$.topic_sort";
    let object = object(value, "catalog", path)?;
    let required = [
        "algorithm",
        "representable_encoding",
        "representable_fold",
        "unrepresentable",
        "tie_break",
    ];
    fields(object, &required, &[], "catalog", path)?;
    let expected = [
        ("algorithm", "stable_display_title_v1"),
        ("representable_encoding", "windows-1252-strict"),
        ("representable_fold", "ascii-lowercase-only"),
        (
            "unrepresentable",
            "unicode-15.1.0-scalar-lowercase-utf8-after-representable",
        ),
        ("tie_break", "registry-order"),
    ];
    for (field, expected) in expected {
        let actual = string(
            object.get(field).unwrap(),
            "catalog",
            &format!("{path}.{field}"),
        )?;
        if actual != expected {
            return error(
                "invalid_topic_sort",
                "catalog",
                format!("{path}.{field}"),
                format!("expected {expected:?}, got {actual:?}"),
            );
        }
    }
    Ok(())
}

fn validate_index(value: &Value) -> Result<(), EncyclopediaError> {
    let path = "$.index";
    let index = object(value, "catalog", path)?;
    fields(
        index,
        &["command", "labels", "topic_ids", "source_ref"],
        &[],
        "catalog",
        path,
    )?;
    let command = string(index.get("command").unwrap(), "catalog", "$.index.command")?;
    if command != "0x6f" {
        return error(
            "invalid_command",
            "catalog",
            "$.index.command",
            "index command must be 0x6f",
        );
    }
    validate_labels(index.get("labels").unwrap(), "$.index.labels")?;
    validate_topic_ids(index.get("topic_ids").unwrap(), "$.index.topic_ids")?;
    validate_source_ref(
        index.get("source_ref").unwrap(),
        "catalog",
        "$.index.source_ref",
    )
}

fn validate_category(value: &Value, index: usize) -> Result<(), EncyclopediaError> {
    let path = format!("$.categories[{index}]");
    let category = object(value, "catalog", &path)?;
    fields(
        category,
        &["id", "command", "labels", "topic_ids", "source_ref"],
        &[],
        "catalog",
        &path,
    )?;
    validate_stable_id(
        string(
            category.get("id").unwrap(),
            "catalog",
            &format!("{path}.id"),
        )?,
        "invalid_category_id",
        "catalog",
        &format!("{path}.id"),
    )?;
    let command = string(
        category.get("command").unwrap(),
        "catalog",
        &format!("{path}.command"),
    )?;
    if !valid_command(command) {
        return error(
            "invalid_command",
            "catalog",
            format!("{path}.command"),
            "invalid command token",
        );
    }
    validate_labels(category.get("labels").unwrap(), &format!("{path}.labels"))?;
    validate_topic_ids(
        category.get("topic_ids").unwrap(),
        &format!("{path}.topic_ids"),
    )?;
    validate_source_ref(
        category.get("source_ref").unwrap(),
        "catalog",
        &format!("{path}.source_ref"),
    )
}

fn validate_labels(value: &Value, path: &str) -> Result<(), EncyclopediaError> {
    let labels = object(value, "catalog", path)?;
    count_range(
        labels.len(),
        1,
        256,
        "catalog",
        path,
        "resource_limit:labels",
    )?;
    for (language, label) in labels {
        validate_langid(language, "catalog", &format!("{path}.{language}"))?;
        let label = string(label, "catalog", &format!("{path}.{language}"))?;
        if label.len() > TITLE_OR_LABEL_BYTES_LIMIT {
            return error(
                "resource_limit:label_bytes",
                "catalog",
                format!("{path}.{language}"),
                "localized label exceeds byte limit",
            );
        }
    }
    Ok(())
}

fn validate_topic_ids(value: &Value, path: &str) -> Result<(), EncyclopediaError> {
    let ids = array(value, "catalog", path)?;
    count_range(
        ids.len(),
        0,
        TOPIC_LIMIT,
        "catalog",
        path,
        "resource_limit:topics",
    )?;
    let mut seen = HashSet::with_capacity(ids.len());
    for (index, value) in ids.iter().enumerate() {
        let item_path = format!("{path}[{index}]");
        let id = string(value, "catalog", &item_path)?;
        validate_stable_id(id, "invalid_topic_id", "catalog", &item_path)?;
        if !seen.insert(id) {
            return error(
                "duplicate_topic_id",
                "catalog",
                item_path,
                format!("duplicate topic ID {id:?}"),
            );
        }
    }
    Ok(())
}

fn validate_topic(value: &Value, topic_id: &str) -> Result<(), EncyclopediaError> {
    let path = format!("$.topics.{topic_id}");
    let topic = object(value, "catalog", &path)?;
    fields(topic, &["localized", "source_ref"], &[], "catalog", &path)
        .map_err(|error| error.with_topic(topic_id))?;
    validate_source_ref(
        topic.get("source_ref").unwrap(),
        "catalog",
        &format!("{path}.source_ref"),
    )
    .map_err(|error| error.with_topic(topic_id))?;
    let localized_path = format!("{path}.localized");
    let localized = object(topic.get("localized").unwrap(), "catalog", &localized_path)
        .map_err(|error| error.with_topic(topic_id))?;
    count_range(
        localized.len(),
        1,
        256,
        "catalog",
        &localized_path,
        "resource_limit:languages",
    )
    .map_err(|error| error.with_topic(topic_id))?;
    for (language, content) in localized {
        validate_langid(language, "catalog", &format!("{localized_path}.{language}"))
            .map_err(|error| error.with_topic(topic_id))?;
        validate_localized(content, topic_id, language)?;
    }
    Ok(())
}

fn validate_localized(
    value: &Value,
    topic_id: &str,
    language: &str,
) -> Result<(), EncyclopediaError> {
    let path = format!("$.topics.{topic_id}.localized.{language}");
    let localized = object(value, "catalog", &path).map_err(|error| error.with_topic(topic_id))?;
    fields(
        localized,
        &["title", "body"],
        &["image_id", "image_selector"],
        "catalog",
        &path,
    )
    .map_err(|error| error.with_topic(topic_id))?;
    let title = string(
        localized.get("title").unwrap(),
        "catalog",
        &format!("{path}.title"),
    )
    .map_err(|error| error.with_topic(topic_id))?;
    if title.len() > TITLE_OR_LABEL_BYTES_LIMIT {
        return error(
            "resource_limit:title_bytes",
            "catalog",
            format!("{path}.title"),
            "localized title exceeds byte limit",
        )
        .map_err(|error| error.with_topic(topic_id));
    }
    let body = string(
        localized.get("body").unwrap(),
        "catalog",
        &format!("{path}.body"),
    )
    .map_err(|error| error.with_topic(topic_id))?;
    if body.len() > BODY_BYTES_LIMIT {
        return error(
            "resource_limit:body_bytes",
            "catalog",
            format!("{path}.body"),
            "localized body exceeds byte limit",
        )
        .map_err(|error| error.with_topic(topic_id));
    }
    if localized.contains_key("image_id") && localized.contains_key("image_selector") {
        return error(
            "ambiguous_image_selector",
            "catalog",
            &path,
            "image_id and image_selector are mutually exclusive",
        )
        .map_err(|error| error.with_topic(topic_id));
    }
    if let Some(image_id) = localized.get("image_id") {
        if !image_id.is_null() {
            validate_base_image_id(
                string(image_id, "catalog", &format!("{path}.image_id"))?,
                "catalog",
                &format!("{path}.image_id"),
            )?;
        }
    }
    if let Some(selector) = localized.get("image_selector") {
        validate_image_selector(selector, topic_id, language)?;
    }
    Ok(())
}

fn validate_image_selector(
    value: &Value,
    topic_id: &str,
    language: &str,
) -> Result<(), EncyclopediaError> {
    let path = format!("$.topics.{topic_id}.localized.{language}.image_selector");
    let selector = object(value, "catalog", &path).map_err(|error| error.with_topic(topic_id))?;
    fields(
        selector,
        &["kind", "alliance_image_id", "empire_image_id"],
        &[],
        "catalog",
        &path,
    )
    .map_err(|error| error.with_topic(topic_id))?;
    let kind = string(
        selector.get("kind").unwrap(),
        "catalog",
        &format!("{path}.kind"),
    )?;
    if kind != "viewer_faction" {
        return error(
            "unsupported_selector",
            "catalog",
            format!("{path}.kind"),
            "unsupported image selector",
        )
        .map_err(|error| error.with_topic(topic_id));
    }
    for side in ["alliance_image_id", "empire_image_id"] {
        let value = selector.get(side).unwrap();
        if !value.is_null() {
            validate_base_image_id(
                string(value, "catalog", &format!("{path}.{side}"))?,
                "catalog",
                &format!("{path}.{side}"),
            )?;
        }
    }
    Ok(())
}

fn validate_base_image(value: &Value, image_id: &str) -> Result<(), EncyclopediaError> {
    let path = format!("$.images.{image_id}");
    let image = object(value, "catalog", &path)?;
    fields(
        image,
        &[
            "path",
            "format",
            "byte_length",
            "width",
            "height",
            "sha256",
            "source_ref",
        ],
        &[],
        "catalog",
        &path,
    )?;
    validate_runtime_asset_path(
        string(
            image.get("path").unwrap(),
            "catalog",
            &format!("{path}.path"),
        )?,
        "catalog",
        &format!("{path}.path"),
    )?;
    let format = string(
        image.get("format").unwrap(),
        "catalog",
        &format!("{path}.format"),
    )?;
    if !matches!(format, "bmp" | "png") {
        return error(
            "unsupported_image_format",
            "catalog",
            format!("{path}.format"),
            "unsupported image format",
        );
    }
    let byte_length = unsigned(
        image.get("byte_length").unwrap(),
        "catalog",
        &format!("{path}.byte_length"),
    )?;
    if byte_length == 0 || byte_length > IMAGE_BYTES_LIMIT {
        return error(
            "resource_limit:image_bytes",
            "catalog",
            format!("{path}.byte_length"),
            "image byte length is outside v1 bounds",
        );
    }
    for dimension in ["width", "height"] {
        let value = unsigned(
            image.get(dimension).unwrap(),
            "catalog",
            &format!("{path}.{dimension}"),
        )?;
        if value == 0 || value > IMAGE_DIMENSION_LIMIT {
            return error(
                "resource_limit:image_dimensions",
                "catalog",
                format!("{path}.{dimension}"),
                "image dimension is outside v1 bounds",
            );
        }
    }
    validate_sha256(
        image.get("sha256").unwrap(),
        "catalog",
        &format!("{path}.sha256"),
    )?;
    validate_source_ref(
        image.get("source_ref").unwrap(),
        "catalog",
        &format!("{path}.source_ref"),
    )
}

fn validate_binding(value: &Value, index: usize) -> Result<(), EncyclopediaError> {
    let path = format!("$.bindings[{index}]");
    let binding = object(value, "catalog", &path)?;
    fields(
        binding,
        &["family", "dat_id", "variant", "topic_id"],
        &[],
        "catalog",
        &path,
    )?;
    let family = string(
        binding.get("family").unwrap(),
        "catalog",
        &format!("{path}.family"),
    )?;
    if family.is_empty()
        || family.len() > 128
        || !family.as_bytes()[0].is_ascii_lowercase()
        || !family
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
    {
        return error(
            "invalid_family",
            "catalog",
            format!("{path}.family"),
            "invalid binding family",
        );
    }
    let dat_id = unsigned(
        binding.get("dat_id").unwrap(),
        "catalog",
        &format!("{path}.dat_id"),
    )?;
    if dat_id > u64::from(u32::MAX) {
        return error(
            "invalid_dat_id",
            "catalog",
            format!("{path}.dat_id"),
            "DAT ID exceeds u32",
        );
    }
    let variant = string(
        binding.get("variant").unwrap(),
        "catalog",
        &format!("{path}.variant"),
    )?;
    if !matches!(variant, "default" | "viewer_faction") {
        return error(
            "unsupported_variant",
            "catalog",
            format!("{path}.variant"),
            "unsupported binding variant",
        );
    }
    validate_stable_id(
        string(
            binding.get("topic_id").unwrap(),
            "catalog",
            &format!("{path}.topic_id"),
        )?,
        "invalid_topic_id",
        "catalog",
        &format!("{path}.topic_id"),
    )
}

fn validate_manifest(value: &Value) -> Result<(), EncyclopediaError> {
    let root = object(value, "manifest", "$")?;
    fields(
        root,
        &[
            "schema_version",
            "source_profile",
            "extractor_version",
            "catalog_sha256",
            "files",
            "binding_sources",
            "source_records",
        ],
        &[],
        "manifest",
        "$",
    )?;
    let version = unsigned(
        root.get("schema_version").unwrap(),
        "manifest",
        "$.schema_version",
    )?;
    if version != 1 {
        return error(
            "unsupported_version",
            "manifest",
            "$.schema_version",
            "unsupported manifest schema version",
        );
    }
    validate_token(root.get("source_profile").unwrap(), "$.source_profile")?;
    validate_token(
        root.get("extractor_version").unwrap(),
        "$.extractor_version",
    )?;
    validate_sha256(
        root.get("catalog_sha256").unwrap(),
        "manifest",
        "$.catalog_sha256",
    )?;
    validate_manifest_files(root.get("files").unwrap())?;

    let sources = array(
        root.get("binding_sources").unwrap(),
        "manifest",
        "$.binding_sources",
    )?;
    count_range(
        sources.len(),
        1,
        256,
        "manifest",
        "$.binding_sources",
        "resource_limit:binding_sources",
    )?;
    for (index, source) in sources.iter().enumerate() {
        validate_binding_source(source, index)?;
    }
    for left in 0..sources.len() {
        if sources[left + 1..].contains(&sources[left]) {
            return error(
                "duplicate_binding_source",
                "manifest",
                "$.binding_sources",
                "duplicate binding source",
            );
        }
    }

    let records = object(
        root.get("source_records").unwrap(),
        "manifest",
        "$.source_records",
    )?;
    count_range(
        records.len(),
        1,
        100_000,
        "manifest",
        "$.source_records",
        "resource_limit:source_records",
    )?;
    for (source_ref, record) in records {
        validate_source_ref_string(
            source_ref,
            "manifest",
            &format!("$.source_records.{source_ref}"),
        )?;
        validate_source_record(record, source_ref)?;
    }
    Ok(())
}

fn validate_manifest_files(value: &Value) -> Result<(), EncyclopediaError> {
    let files = object(value, "manifest", "$.files")?;
    count_range(
        files.len(),
        1,
        20_001,
        "manifest",
        "$.files",
        "resource_limit:files",
    )?;
    if !files.contains_key("catalog.json") {
        return error(
            "missing_field",
            "manifest",
            "$.files.catalog.json",
            "required catalog.json entry is missing",
        );
    }
    for (path, digest) in files {
        if path != "catalog.json" && !valid_runtime_asset_path(path) {
            let code = if path == "manifest.json" {
                "unexpected_runtime_file"
            } else {
                "unsafe_asset_path"
            };
            return error(
                code,
                "manifest",
                format!("$.files.{path}"),
                "invalid runtime manifest path",
            );
        }
        validate_sha256(digest, "manifest", &format!("$.files.{path}"))?;
    }
    Ok(())
}

fn validate_binding_source(value: &Value, index: usize) -> Result<(), EncyclopediaError> {
    let path = format!("$.binding_sources[{index}]");
    let source = object(value, "manifest", &path)?;
    fields(source, &["basename", "sha256"], &[], "manifest", &path)?;
    validate_basename(source.get("basename").unwrap(), &format!("{path}.basename"))?;
    validate_sha256(
        source.get("sha256").unwrap(),
        "manifest",
        &format!("{path}.sha256"),
    )
}

fn validate_source_record(value: &Value, source_ref: &str) -> Result<(), EncyclopediaError> {
    let path = format!("$.source_records.{source_ref}");
    let record = object(value, "manifest", &path)?;
    fields(
        record,
        &[
            "source_basename",
            "source_sha256",
            "resource_type",
            "resource_id",
            "language_id",
            "raw_length",
            "raw_sha256",
            "decoder",
            "encoding",
            "mapping_citations",
        ],
        &[],
        "manifest",
        &path,
    )?;
    validate_basename(
        record.get("source_basename").unwrap(),
        &format!("{path}.source_basename"),
    )?;
    validate_sha256(
        record.get("source_sha256").unwrap(),
        "manifest",
        &format!("{path}.source_sha256"),
    )?;
    validate_resource_identifier(
        record.get("resource_type").unwrap(),
        &format!("{path}.resource_type"),
    )?;
    validate_resource_identifier(
        record.get("resource_id").unwrap(),
        &format!("{path}.resource_id"),
    )?;
    let language = unsigned(
        record.get("language_id").unwrap(),
        "manifest",
        &format!("{path}.language_id"),
    )?;
    if language > u64::from(u16::MAX) {
        return error(
            "invalid_language_id",
            "manifest",
            format!("{path}.language_id"),
            "language ID exceeds u16",
        );
    }
    let raw_length = unsigned(
        record.get("raw_length").unwrap(),
        "manifest",
        &format!("{path}.raw_length"),
    )?;
    if raw_length > IMAGE_BYTES_LIMIT {
        return error(
            "resource_limit:source_bytes",
            "manifest",
            format!("{path}.raw_length"),
            "source record exceeds byte limit",
        );
    }
    validate_sha256(
        record.get("raw_sha256").unwrap(),
        "manifest",
        &format!("{path}.raw_sha256"),
    )?;
    validate_nonempty_string(
        record.get("decoder").unwrap(),
        128,
        "manifest",
        &format!("{path}.decoder"),
    )?;
    validate_nonempty_string(
        record.get("encoding").unwrap(),
        128,
        "manifest",
        &format!("{path}.encoding"),
    )?;
    let citations = array(
        record.get("mapping_citations").unwrap(),
        "manifest",
        &format!("{path}.mapping_citations"),
    )?;
    count_range(
        citations.len(),
        1,
        64,
        "manifest",
        &format!("{path}.mapping_citations"),
        "resource_limit:mapping_citations",
    )?;
    let mut seen = HashSet::with_capacity(citations.len());
    for (index, citation) in citations.iter().enumerate() {
        let citation_path = format!("{path}.mapping_citations[{index}]");
        let citation = string(citation, "manifest", &citation_path)?;
        if citation.is_empty() || citation.chars().count() > 512 {
            return error(
                "invalid_citation",
                "manifest",
                citation_path,
                "invalid mapping citation",
            );
        }
        if !seen.insert(citation) {
            return error(
                "duplicate_citation",
                "manifest",
                citation_path,
                "duplicate mapping citation",
            );
        }
    }
    Ok(())
}

fn validate_resource_identifier(value: &Value, path: &str) -> Result<(), EncyclopediaError> {
    let identifier = object(value, "manifest", path)?;
    fields(identifier, &["kind", "value"], &[], "manifest", path)?;
    let kind = string(
        identifier.get("kind").unwrap(),
        "manifest",
        &format!("{path}.kind"),
    )?;
    match kind {
        "numeric" => {
            let value = unsigned(
                identifier.get("value").unwrap(),
                "manifest",
                &format!("{path}.value"),
            )?;
            if value > u64::from(u32::MAX) {
                return error(
                    "invalid_resource_id",
                    "manifest",
                    format!("{path}.value"),
                    "numeric resource identity exceeds u32",
                );
            }
        }
        "named" => {
            validate_nonempty_string(
                identifier.get("value").unwrap(),
                256,
                "manifest",
                &format!("{path}.value"),
            )?;
        }
        _ => {
            return error(
                "unsupported_resource_id",
                "manifest",
                format!("{path}.kind"),
                "unsupported resource identifier kind",
            )
        }
    }
    Ok(())
}

fn fields(
    object: &Map<String, Value>,
    required: &[&str],
    optional: &[&str],
    source: &'static str,
    path: &str,
) -> Result<(), EncyclopediaError> {
    for key in object.keys() {
        if !required.contains(&key.as_str()) && !optional.contains(&key.as_str()) {
            return error(
                "unknown_field",
                source,
                format!("{path}.{key}"),
                format!("unknown field {key:?}"),
            );
        }
    }
    for field in required {
        if !object.contains_key(*field) {
            return error(
                "missing_field",
                source,
                format!("{path}.{field}"),
                format!("missing required field {field:?}"),
            );
        }
    }
    Ok(())
}

fn object<'a>(
    value: &'a Value,
    source: &'static str,
    path: &str,
) -> Result<&'a Map<String, Value>, EncyclopediaError> {
    value
        .as_object()
        .ok_or_else(|| EncyclopediaError::new("invalid_type", source, path, "expected object"))
}

fn array<'a>(
    value: &'a Value,
    source: &'static str,
    path: &str,
) -> Result<&'a Vec<Value>, EncyclopediaError> {
    value
        .as_array()
        .ok_or_else(|| EncyclopediaError::new("invalid_type", source, path, "expected array"))
}

fn string<'a>(
    value: &'a Value,
    source: &'static str,
    path: &str,
) -> Result<&'a str, EncyclopediaError> {
    value
        .as_str()
        .ok_or_else(|| EncyclopediaError::new("invalid_type", source, path, "expected string"))
}

fn unsigned(value: &Value, source: &'static str, path: &str) -> Result<u64, EncyclopediaError> {
    value.as_u64().ok_or_else(|| {
        EncyclopediaError::new("invalid_type", source, path, "expected unsigned integer")
    })
}

fn count_range(
    count: usize,
    minimum: usize,
    maximum: usize,
    source: &'static str,
    path: &str,
    code: &'static str,
) -> Result<(), EncyclopediaError> {
    if count < minimum || count > maximum {
        return error(
            code,
            source,
            path,
            format!("item count {count} is outside {minimum}..={maximum}"),
        );
    }
    Ok(())
}

fn validate_langid(value: &str, source: &'static str, path: &str) -> Result<(), EncyclopediaError> {
    let valid = value == "0"
        || (!value.starts_with('0')
            && value.len() <= 5
            && value.bytes().all(|byte| byte.is_ascii_digit()));
    if !valid {
        return error(
            "invalid_language_id",
            source,
            path,
            format!("invalid LANGID {value:?}"),
        );
    }
    Ok(())
}

fn validate_stable_id(
    value: &str,
    code: &'static str,
    source: &'static str,
    path: &str,
) -> Result<(), EncyclopediaError> {
    let Some((namespace, identity)) = value.split_once(':') else {
        return error(code, source, path, "stable ID lacks namespace separator");
    };
    let valid_namespace = !namespace.is_empty()
        && namespace.len() <= 32
        && namespace.as_bytes()[0].is_ascii_lowercase()
        && namespace.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        });
    let valid_identity = !identity.is_empty()
        && identity.len() <= 224
        && identity.as_bytes()[0].is_ascii_alphanumeric()
        && identity
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-'));
    if value.len() < 3 || value.len() > 256 || !valid_namespace || !valid_identity {
        return error(code, source, path, format!("invalid stable ID {value:?}"));
    }
    Ok(())
}

fn validate_base_image_id(
    value: &str,
    source: &'static str,
    path: &str,
) -> Result<(), EncyclopediaError> {
    let Some(number) = value.strip_prefix("edata:") else {
        return error(
            "invalid_base_image_id",
            source,
            path,
            "base image ID must start with edata:",
        );
    };
    let valid = !number.is_empty()
        && number.len() <= 10
        && number.bytes().all(|byte| byte.is_ascii_digit())
        && (number == "0" || !number.starts_with('0'));
    if value.len() > 16 || !valid {
        return error(
            "invalid_base_image_id",
            source,
            path,
            format!("invalid base image ID {value:?}"),
        );
    }
    Ok(())
}

fn validate_source_ref(
    value: &Value,
    source: &'static str,
    path: &str,
) -> Result<(), EncyclopediaError> {
    validate_source_ref_string(string(value, source, path)?, source, path)
}

fn validate_source_ref_string(
    value: &str,
    source: &'static str,
    path: &str,
) -> Result<(), EncyclopediaError> {
    let valid = !value.is_empty()
        && value.len() <= 256
        && value.as_bytes()[0].is_ascii_alphanumeric()
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'/' | b'-')
        });
    if !valid {
        return error(
            "invalid_source_ref",
            source,
            path,
            format!("invalid source reference {value:?}"),
        );
    }
    Ok(())
}

fn validate_sha256(
    value: &Value,
    source: &'static str,
    path: &str,
) -> Result<(), EncyclopediaError> {
    let digest = string(value, source, path)?;
    if digest.len() != 64
        || !digest
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return error(
            "invalid_sha256",
            source,
            path,
            "expected lowercase SHA-256 hex",
        );
    }
    Ok(())
}

fn validate_runtime_asset_path(
    value: &str,
    source: &'static str,
    path: &str,
) -> Result<(), EncyclopediaError> {
    if !valid_runtime_asset_path(value) {
        return error(
            "unsafe_asset_path",
            source,
            path,
            format!("unsafe runtime asset path {value:?}"),
        );
    }
    Ok(())
}

fn valid_runtime_asset_path(value: &str) -> bool {
    if !(8..=256).contains(&value.len()) {
        return false;
    }
    let Some(rest) = value.strip_prefix("assets/") else {
        return false;
    };
    let segments: Vec<_> = rest.split('/').collect();
    !segments.is_empty()
        && segments.iter().enumerate().all(|(index, segment)| {
            let limit = if index + 1 == segments.len() {
                128
            } else {
                127
            };
            !segment.is_empty()
                && segment.len() <= limit
                && segment.as_bytes()[0].is_ascii_alphanumeric()
                && segment
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        })
}

fn validate_token(value: &Value, path: &str) -> Result<(), EncyclopediaError> {
    let token = string(value, "manifest", path)?;
    let valid = !token.is_empty()
        && token.len() <= 128
        && token.as_bytes()[0].is_ascii_alphanumeric()
        && token
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'));
    if !valid {
        return error(
            "invalid_token",
            "manifest",
            path,
            format!("invalid token {token:?}"),
        );
    }
    Ok(())
}

fn validate_basename(value: &Value, path: &str) -> Result<(), EncyclopediaError> {
    validate_token(value, path)
}

fn validate_nonempty_string(
    value: &Value,
    maximum_chars: usize,
    source: &'static str,
    path: &str,
) -> Result<(), EncyclopediaError> {
    let value = string(value, source, path)?;
    if value.is_empty() || value.chars().count() > maximum_chars {
        return error(
            "invalid_string",
            source,
            path,
            "string is empty or exceeds its limit",
        );
    }
    Ok(())
}

fn valid_command(value: &str) -> bool {
    value.len() >= 4
        && value.len() <= 10
        && value.starts_with("0x")
        && value[2..]
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn error<T>(
    code: &'static str,
    source: &'static str,
    path: impl Into<String>,
    detail: impl Into<String>,
) -> Result<T, EncyclopediaError> {
    Err(EncyclopediaError::new(code, source, path, detail))
}
