use std::fs;
use std::path::{Path, PathBuf};

use rebellion_data::encyclopedia::{
    parse_catalog, parse_manifest, BaseImageIdField, CATALOG_JSON_BYTES_LIMIT,
    CATALOG_JSON_DEPTH_LIMIT, MANIFEST_JSON_BYTES_LIMIT, MANIFEST_JSON_DEPTH_LIMIT,
};
use serde::Deserialize;

fn corpus_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tests/fixtures/encyclopedia")
}

fn fixture(relative: &str) -> Vec<u8> {
    fs::read(corpus_root().join("fixtures").join(relative)).unwrap()
}

fn catalog_error(bytes: &[u8]) -> String {
    parse_catalog(bytes).unwrap_err().code().to_owned()
}

fn manifest_error(bytes: &[u8]) -> String {
    parse_manifest(bytes).unwrap_err().code().to_owned()
}

#[derive(Deserialize)]
struct FixtureInventory {
    cases: Vec<FixtureCase>,
}

#[derive(Deserialize)]
struct FixtureCase {
    id: String,
    validation_layer: String,
    fixture: Option<String>,
    consuming_validator: String,
    expect: FixtureExpectation,
}

#[derive(Deserialize)]
struct FixtureExpectation {
    accepted: bool,
    code: String,
}

fn fixture_inventory() -> FixtureInventory {
    serde_json::from_slice(&fs::read(corpus_root().join("cases.json")).unwrap()).unwrap()
}

#[test]
fn valid_catalog_and_manifest_preserve_typed_wire_fields() {
    let catalog = parse_catalog(&fixture("bundles/valid/catalog.json")).unwrap();
    assert_eq!(catalog.schema_version, 1);
    assert_eq!(catalog.default_language, "1033");
    assert_eq!(catalog.topic_sort.algorithm, "stable_display_title_v1");
    assert_eq!(catalog.index.command, "0x6f");
    assert_eq!(catalog.categories.len(), 6);
    assert_eq!(catalog.topics.len(), 7);
    assert_eq!(catalog.images.len(), 3);
    assert_eq!(catalog.bindings.len(), 7);

    let selector = &catalog.topics["original:60004"].localized["1033"];
    assert!(selector.image_selector.is_some());
    assert_eq!(selector.image_id, BaseImageIdField::Absent);
    let explicit_no_art = &catalog.topics["original:60006"].localized["1033"];
    assert_eq!(explicit_no_art.image_id, BaseImageIdField::Null);

    let manifest = parse_manifest(&fixture("bundles/valid/manifest.json")).unwrap();
    assert_eq!(manifest.schema_version, 1);
    assert_eq!(manifest.source_profile, "e37-synthetic-v1");
    assert!(manifest.files.contains_key("catalog.json"));
    assert!(!manifest.binding_sources.is_empty());
    assert!(!manifest.source_records.is_empty());
}

#[test]
fn shared_e37_schema_cases_return_their_stable_diagnostics() {
    let inventory = fixture_inventory();
    let mut exercised = 0;
    for case in inventory.cases.iter().filter(|case| {
        case.consuming_validator.contains("E11 Rust")
            && case.validation_layer == "schema_structure"
            && case.fixture.is_some()
    }) {
        let bytes = fixture(case.fixture.as_deref().unwrap());
        let result = if case.id.starts_with("manifest-") {
            parse_manifest(&bytes).map(|_| ())
        } else {
            parse_catalog(&bytes).map(|_| ())
        };
        match (&case.expect.accepted, result) {
            (true, Ok(())) => {}
            (false, Err(error)) => assert_eq!(error.code(), case.expect.code, "{}", case.id),
            (expected, actual) => {
                panic!("{} expected accepted={expected}, got {actual:?}", case.id)
            }
        }
        exercised += 1;
    }
    assert_eq!(
        exercised, 27,
        "the approved E37 structural inventory changed"
    );
}

#[test]
fn duplicate_keys_are_rejected_before_nested_maps_can_overwrite_them() {
    let inventory = fixture_inventory();
    let cases: Vec<_> = inventory
        .cases
        .iter()
        .filter(|case| {
            case.consuming_validator.contains("E11 Rust")
                && case.validation_layer == "raw_parsing"
                && !case.expect.accepted
                && case.expect.code == "duplicate_key"
        })
        .collect();
    assert_eq!(cases.len(), 7);
    for case in cases {
        let bytes = fixture(case.fixture.as_deref().unwrap());
        let code = if case.id.starts_with("raw-manifest-") {
            manifest_error(&bytes)
        } else {
            catalog_error(&bytes)
        };
        assert_eq!(code, "duplicate_key", "{}", case.id);
    }
}

#[test]
fn raw_scanner_accepts_nested_values_and_repeated_keys_in_distinct_scopes() {
    for relative in [
        "raw/valid-nested-values.json",
        "raw/valid-repeated-keys-distinct-scopes.json",
    ] {
        let error = parse_catalog(&fixture(relative)).unwrap_err();
        assert_ne!(error.code(), "duplicate_key", "{relative}");
        assert_ne!(error.code(), "invalid_json", "{relative}");
        assert_ne!(error.code(), "invalid_utf8", "{relative}");
    }
}

#[test]
fn invalid_utf8_is_rejected_before_typed_deserialization() {
    let error = parse_catalog(&fixture("raw/catalog-invalid-utf8.json")).unwrap_err();
    assert_eq!(error.code(), "invalid_utf8");
    assert_eq!(error.source(), "catalog");
}

#[test]
fn required_nullable_and_exact_case_fields_preserve_presence() {
    let catalog = fixture("bundles/valid/catalog.json");
    let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();

    value["topics"]["original:60001"]["localized"]["1033"]
        .as_object_mut()
        .unwrap()
        .remove("body");
    let error = parse_catalog(&serde_json::to_vec(&value).unwrap()).unwrap_err();
    assert_eq!(error.code(), "missing_field");
    assert!(error.path().ends_with(".body"));
    assert_eq!(error.topic_id().unwrap().0, "original:60001");

    let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
    value["topics"]["original:60001"]["localized"]["1033"]["body"] = serde_json::Value::Null;
    assert_eq!(
        catalog_error(&serde_json::to_vec(&value).unwrap()),
        "invalid_type"
    );

    let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
    let localized = value["topics"]["original:60001"]["localized"]["1033"]
        .as_object_mut()
        .unwrap();
    let body = localized.remove("body").unwrap();
    localized.insert("Body".to_owned(), body);
    assert_eq!(
        catalog_error(&serde_json::to_vec(&value).unwrap()),
        "unknown_field"
    );

    let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
    value["topics"]["original:60004"]["localized"]["1033"]["image_id"] = serde_json::Value::Null;
    assert_eq!(
        catalog_error(&serde_json::to_vec(&value).unwrap()),
        "ambiguous_image_selector"
    );

    let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
    value["bindings"][0]
        .as_object_mut()
        .unwrap()
        .remove("dat_id");
    assert_eq!(
        catalog_error(&serde_json::to_vec(&value).unwrap()),
        "missing_field"
    );
}

#[test]
fn manifest_nested_fields_reject_missing_null_and_wrong_types() {
    let manifest = fixture("bundles/valid/manifest.json");
    let mut value: serde_json::Value = serde_json::from_slice(&manifest).unwrap();
    let first = value["source_records"]
        .as_object_mut()
        .unwrap()
        .values_mut()
        .next()
        .unwrap();
    first.as_object_mut().unwrap().remove("raw_length");
    assert_eq!(
        manifest_error(&serde_json::to_vec(&value).unwrap()),
        "missing_field"
    );

    let mut value: serde_json::Value = serde_json::from_slice(&manifest).unwrap();
    let first = value["source_records"]
        .as_object_mut()
        .unwrap()
        .values_mut()
        .next()
        .unwrap();
    first["language_id"] = serde_json::Value::Null;
    assert_eq!(
        manifest_error(&serde_json::to_vec(&value).unwrap()),
        "invalid_type"
    );

    let mut value: serde_json::Value = serde_json::from_slice(&manifest).unwrap();
    let first = value["source_records"]
        .as_object_mut()
        .unwrap()
        .values_mut()
        .next()
        .unwrap();
    first["resource_id"]["value"] = serde_json::Value::Null;
    assert_eq!(
        manifest_error(&serde_json::to_vec(&value).unwrap()),
        "invalid_type"
    );
}

#[test]
fn runtime_only_and_alias_fields_are_not_wire_fields() {
    let catalog = fixture("bundles/valid/catalog.json");
    for field in ["aliases", "effective_images", "retained_bytes", "mod_name"] {
        let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
        value[field] = serde_json::json!({});
        let error = parse_catalog(&serde_json::to_vec(&value).unwrap()).unwrap_err();
        assert_eq!(error.code(), "unknown_field", "{field}");
        assert_eq!(error.source(), "catalog");
        assert_eq!(error.path(), format!("$.{field}"));
    }
}

#[test]
fn schema_valid_png_descriptors_remain_structural_while_base_policy_is_deferred() {
    let mut value: serde_json::Value =
        serde_json::from_slice(&fixture("bundles/valid/catalog.json")).unwrap();
    value["images"]["edata:1"]["format"] = serde_json::Value::String("png".to_owned());
    parse_catalog(&serde_json::to_vec(&value).unwrap()).unwrap();
}

fn runtime_asset_path_with_total_len(total_len: usize) -> String {
    let final_segment_len = total_len - 171;
    format!(
        "assets/{}/{}/{}/{}/{}",
        "a".repeat(40),
        "b".repeat(40),
        "c".repeat(40),
        "d".repeat(40),
        "e".repeat(final_segment_len),
    )
}

#[test]
fn runtime_asset_paths_accept_256_bytes_and_reject_257_bytes() {
    let exact = runtime_asset_path_with_total_len(256);
    let above = runtime_asset_path_with_total_len(257);
    assert_eq!(exact.len(), 256);
    assert_eq!(above.len(), 257);

    let catalog = fixture("bundles/valid/catalog.json");
    for (path, accepted) in [(&exact, true), (&above, false)] {
        let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
        value["images"]["edata:1"]["path"] = serde_json::Value::String(path.clone());
        let result = parse_catalog(&serde_json::to_vec(&value).unwrap());
        if accepted {
            result.unwrap();
        } else {
            assert_eq!(result.unwrap_err().code(), "unsafe_asset_path");
        }
    }

    let manifest = fixture("bundles/valid/manifest.json");
    for (path, accepted) in [(&exact, true), (&above, false)] {
        let mut value: serde_json::Value = serde_json::from_slice(&manifest).unwrap();
        value["files"]
            .as_object_mut()
            .unwrap()
            .insert(path.clone(), serde_json::Value::String("0".repeat(64)));
        let result = parse_manifest(&serde_json::to_vec(&value).unwrap());
        if accepted {
            result.unwrap();
        } else {
            assert_eq!(result.unwrap_err().code(), "unsafe_asset_path");
        }
    }
}

#[test]
fn runtime_asset_path_segments_use_distinct_directory_and_final_limits() {
    let catalog = fixture("bundles/valid/catalog.json");
    for (path, accepted) in [
        (format!("assets/{}/x", "a".repeat(127)), true),
        (format!("assets/{}/x", "a".repeat(128)), false),
        (format!("assets/{}", "z".repeat(128)), true),
        (format!("assets/{}", "z".repeat(129)), false),
    ] {
        let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
        value["images"]["edata:1"]["path"] = serde_json::Value::String(path);
        let result = parse_catalog(&serde_json::to_vec(&value).unwrap());
        if accepted {
            result.unwrap();
        } else {
            assert_eq!(result.unwrap_err().code(), "unsafe_asset_path");
        }
    }
}

#[test]
fn localized_text_limits_count_utf8_bytes_at_exact_and_above_boundaries() {
    let catalog = fixture("bundles/valid/catalog.json");

    for (field, exact, code) in [
        ("title", "t".repeat(65_536), "resource_limit:title_bytes"),
        ("body", "b".repeat(1_048_576), "resource_limit:body_bytes"),
    ] {
        let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
        value["topics"]["original:60001"]["localized"]["1033"][field] =
            serde_json::Value::String(exact.clone());
        parse_catalog(&serde_json::to_vec(&value).unwrap()).unwrap();
        value["topics"]["original:60001"]["localized"]["1033"][field] =
            serde_json::Value::String(format!("{exact}x"));
        assert_eq!(catalog_error(&serde_json::to_vec(&value).unwrap()), code);
    }

    let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
    value["topics"]["original:60001"]["localized"]["1033"]["body"] =
        serde_json::Value::String("é".repeat(524_289));
    assert_eq!(
        catalog_error(&serde_json::to_vec(&value).unwrap()),
        "resource_limit:body_bytes"
    );

    let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
    value["index"]["labels"]["1033"] = serde_json::Value::String("l".repeat(65_536));
    parse_catalog(&serde_json::to_vec(&value).unwrap()).unwrap();
    value["index"]["labels"]["1033"] = serde_json::Value::String("l".repeat(65_537));
    assert_eq!(
        catalog_error(&serde_json::to_vec(&value).unwrap()),
        "resource_limit:label_bytes"
    );
}

#[test]
fn topic_count_accepts_ten_thousand_and_rejects_one_more() {
    let mut value: serde_json::Value =
        serde_json::from_slice(&fixture("bundles/valid/catalog.json")).unwrap();
    let template = value["topics"]["original:60001"].clone();
    let topics = value["topics"].as_object_mut().unwrap();
    topics.clear();
    for id in 1..=10_000 {
        topics.insert(format!("original:{id}"), template.clone());
    }
    parse_catalog(&serde_json::to_vec(&value).unwrap()).unwrap();
    value["topics"]
        .as_object_mut()
        .unwrap()
        .insert("original:10001".to_owned(), template);
    assert_eq!(
        catalog_error(&serde_json::to_vec(&value).unwrap()),
        "resource_limit:topics"
    );
}

fn nested_array(depth: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(depth * 2 + 1);
    bytes.extend(std::iter::repeat_n(b'[', depth));
    bytes.push(b'0');
    bytes.extend(std::iter::repeat_n(b']', depth));
    bytes
}

fn nested_object(depth: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(depth * 6 + 1);
    for _ in 0..depth {
        bytes.extend_from_slice(br#"{"a":"#);
    }
    bytes.push(b'0');
    bytes.extend(std::iter::repeat_n(b'}', depth));
    bytes
}

#[test]
fn json_depth_limits_accept_the_boundary_before_structural_rejection() {
    let catalog_exact = parse_catalog(&nested_array(CATALOG_JSON_DEPTH_LIMIT)).unwrap_err();
    assert_ne!(catalog_exact.code(), "resource_limit:json_depth");
    assert_eq!(
        catalog_error(&nested_array(CATALOG_JSON_DEPTH_LIMIT + 1)),
        "resource_limit:json_depth"
    );

    let manifest_exact = parse_manifest(&nested_array(MANIFEST_JSON_DEPTH_LIMIT)).unwrap_err();
    assert_ne!(manifest_exact.code(), "resource_limit:json_depth");
    assert_eq!(
        manifest_error(&nested_array(MANIFEST_JSON_DEPTH_LIMIT + 1)),
        "resource_limit:json_depth"
    );

    let object_exact = parse_catalog(&nested_object(CATALOG_JSON_DEPTH_LIMIT)).unwrap_err();
    assert_ne!(object_exact.code(), "resource_limit:json_depth");
    assert_eq!(
        catalog_error(&nested_object(CATALOG_JSON_DEPTH_LIMIT + 1)),
        "resource_limit:json_depth"
    );
}

#[test]
fn json_byte_limits_accept_the_boundary_before_syntax_rejection() {
    let catalog_exact = vec![b' '; CATALOG_JSON_BYTES_LIMIT];
    assert_ne!(
        parse_catalog(&catalog_exact).unwrap_err().code(),
        "resource_limit:json_bytes"
    );
    let catalog_above = vec![b' '; CATALOG_JSON_BYTES_LIMIT + 1];
    assert_eq!(catalog_error(&catalog_above), "resource_limit:json_bytes");

    let manifest_exact = vec![b' '; MANIFEST_JSON_BYTES_LIMIT];
    assert_ne!(
        parse_manifest(&manifest_exact).unwrap_err().code(),
        "resource_limit:json_bytes"
    );
    let manifest_above = vec![b' '; MANIFEST_JSON_BYTES_LIMIT + 1];
    assert_eq!(manifest_error(&manifest_above), "resource_limit:json_bytes");
}
