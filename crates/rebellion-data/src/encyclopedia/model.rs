use std::borrow::Borrow;
use std::collections::BTreeMap;

use serde::de::{self, Visitor};
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(transparent)]
pub struct TopicId(pub String);

impl Borrow<str> for TopicId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(transparent)]
pub struct BaseImageId(pub String);

impl Borrow<str> for BaseImageId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingKey {
    pub family: String,
    pub dat_id: u32,
    pub variant: String,
}

/// Runtime facts about bytes retained by a later content session. This is not
/// a catalog or manifest wire type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetFacts {
    pub sha256: String,
    pub byte_len: u64,
    pub image: Option<ImageFacts>,
}

/// Runtime image facts produced by bounded byte inspection. This is not a
/// catalog image descriptor and deliberately carries no source provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageFacts {
    pub format: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EncyclopediaCatalog {
    pub schema_version: u32,
    pub default_language: String,
    pub topic_sort: TopicSort,
    pub index: CatalogIndex,
    pub categories: Vec<CatalogCategory>,
    pub topics: BTreeMap<TopicId, Topic>,
    pub images: BTreeMap<BaseImageId, BaseImage>,
    pub bindings: Vec<CatalogBinding>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TopicSort {
    pub algorithm: String,
    pub representable_encoding: String,
    pub representable_fold: String,
    pub unrepresentable: String,
    pub tie_break: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogIndex {
    pub command: String,
    pub labels: BTreeMap<String, String>,
    pub topic_ids: Vec<TopicId>,
    pub source_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogCategory {
    pub id: String,
    pub command: String,
    pub labels: BTreeMap<String, String>,
    pub topic_ids: Vec<TopicId>,
    pub source_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Topic {
    pub localized: BTreeMap<String, LocalizedContent>,
    pub source_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalizedContent {
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub image_id: BaseImageIdField,
    pub image_selector: Option<ImageSelector>,
}

/// The catalog distinguishes an omitted `image_id` from an explicit JSON
/// `null`. Both mean no static image, but presence remains significant because
/// either form conflicts with a simultaneously present `image_selector`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BaseImageIdField {
    #[default]
    Absent,
    Null,
    Value(BaseImageId),
}

impl<'de> Deserialize<'de> for BaseImageIdField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BaseImageIdFieldVisitor;

        impl<'de> Visitor<'de> for BaseImageIdFieldVisitor {
            type Value = BaseImageIdField;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a base image ID or null")
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(BaseImageIdField::Null)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(BaseImageIdField::Null)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                BaseImageId::deserialize(deserializer).map(BaseImageIdField::Value)
            }
        }

        deserializer.deserialize_option(BaseImageIdFieldVisitor)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImageSelector {
    pub kind: String,
    pub alliance_image_id: NullableBaseImageId,
    pub empire_image_id: NullableBaseImageId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NullableBaseImageId {
    Null,
    Value(BaseImageId),
}

impl<'de> Deserialize<'de> for NullableBaseImageId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct NullableBaseImageIdVisitor;

        impl<'de> Visitor<'de> for NullableBaseImageIdVisitor {
            type Value = NullableBaseImageId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a base image ID or null")
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(NullableBaseImageId::Null)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(NullableBaseImageId::Null)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                BaseImageId::deserialize(deserializer).map(NullableBaseImageId::Value)
            }
        }

        deserializer.deserialize_option(NullableBaseImageIdVisitor)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BaseImage {
    pub path: String,
    pub format: String,
    pub byte_length: u64,
    pub width: u32,
    pub height: u32,
    pub sha256: String,
    pub source_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogBinding {
    pub family: String,
    pub dat_id: u32,
    pub variant: String,
    pub topic_id: TopicId,
}

impl CatalogBinding {
    #[must_use]
    pub fn key(&self) -> BindingKey {
        BindingKey {
            family: self.family.clone(),
            dat_id: self.dat_id,
            variant: self.variant.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EncyclopediaManifest {
    pub schema_version: u32,
    pub source_profile: String,
    pub extractor_version: String,
    pub catalog_sha256: String,
    pub files: BTreeMap<String, String>,
    pub binding_sources: Vec<BindingSource>,
    pub source_records: BTreeMap<String, SourceRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BindingSource {
    pub basename: String,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ResourceIdentifier {
    Numeric(u32),
    Named(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRecord {
    pub source_basename: String,
    pub source_sha256: String,
    pub resource_type: ResourceIdentifier,
    pub resource_id: ResourceIdentifier,
    pub language_id: u16,
    pub raw_length: u64,
    pub raw_sha256: String,
    pub decoder: String,
    pub encoding: String,
    pub mapping_citations: Vec<String>,
}
