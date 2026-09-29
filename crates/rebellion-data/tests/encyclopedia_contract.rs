use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use rebellion_data::encyclopedia::{
    parse_catalog, parse_manifest, validate_bundle, validate_effective_catalog, AssetFacts,
    BaseImageIdField, EncyclopediaCatalog, EncyclopediaManifest, ImageFacts,
    CATALOG_JSON_BYTES_LIMIT, CATALOG_JSON_DEPTH_LIMIT, MANIFEST_JSON_BYTES_LIMIT,
    MANIFEST_JSON_DEPTH_LIMIT,
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

fn parsed_catalog(relative: &str) -> EncyclopediaCatalog {
    parse_catalog(&fixture(relative)).unwrap()
}

fn effective_facts(catalog: &EncyclopediaCatalog) -> BTreeMap<String, AssetFacts> {
    catalog
        .images
        .iter()
        .map(|(image_id, image)| {
            (
                image_id.0.clone(),
                AssetFacts {
                    sha256: image.sha256.clone(),
                    byte_len: image.byte_length,
                    image: Some(ImageFacts {
                        format: image.format.clone(),
                        width: image.width,
                        height: image.height,
                    }),
                },
            )
        })
        .collect()
}

fn valid_base_inputs() -> (
    EncyclopediaCatalog,
    EncyclopediaManifest,
    BTreeMap<String, AssetFacts>,
    BTreeMap<String, String>,
) {
    let catalog_bytes = fixture("bundles/valid/catalog.json");
    let catalog = parse_catalog(&catalog_bytes).unwrap();
    let manifest = parse_manifest(&fixture("bundles/valid/manifest.json")).unwrap();
    let mut files = BTreeMap::from([(
        "catalog.json".to_owned(),
        AssetFacts {
            sha256: "f1087ebf39faef3aef5b702e6ad957a910e7fa5924995d668be42b6eddbc39c9".to_owned(),
            byte_len: catalog_bytes.len() as u64,
            image: None,
        },
    )]);
    for image in catalog.images.values() {
        files.insert(
            image.path.clone(),
            AssetFacts {
                sha256: image.sha256.clone(),
                byte_len: image.byte_length,
                image: Some(ImageFacts {
                    format: image.format.clone(),
                    width: image.width,
                    height: image.height,
                }),
            },
        );
    }
    let base_dats = BTreeMap::from([(
        "synthetic.dat".to_owned(),
        "39fb2c329d34fbdd94bb2a2a596b694597eb00b8b402905ec4920f560210d322".to_owned(),
    )]);
    (catalog, manifest, files, base_dats)
}

fn bundle_inputs(
    name: &str,
) -> (
    EncyclopediaCatalog,
    EncyclopediaManifest,
    BTreeMap<String, AssetFacts>,
    BTreeMap<String, String>,
) {
    let catalog_bytes = fixture(&format!("bundles/{name}/catalog.json"));
    let catalog = parse_catalog(&catalog_bytes).unwrap();
    let manifest = parse_manifest(&fixture(&format!("bundles/{name}/manifest.json"))).unwrap();
    let catalog_digest = match name {
        "invalid-base-format-mismatch" => {
            "fc81e680a38756e0435dff4c90ed299038444b83111d4d45a00e6837bff36159"
        }
        "invalid-base-png" => "dfce4c4b4a20dcf96b89ee5eeacea0fa13903a63d10316bd7865219b77b44677",
        "invalid-dangling-source-ref" => {
            "3862055407583db867934ee76bdef48294adc80a7b71d4e78e1681ff195c2c68"
        }
        "invalid-image-descriptor-hash" => {
            "6e740b0beb1108f4552e5015f2a91b2c79b34845e59ecd1a4ef8ef06a2688b1c"
        }
        _ => "f1087ebf39faef3aef5b702e6ad957a910e7fa5924995d668be42b6eddbc39c9",
    };
    let mut files = BTreeMap::from([(
        "catalog.json".to_owned(),
        AssetFacts {
            sha256: catalog_digest.to_owned(),
            byte_len: catalog_bytes.len() as u64,
            image: None,
        },
    )]);
    let first_is_png = matches!(name, "invalid-base-format-mismatch" | "invalid-base-png");
    for (path, sha256, format, width, height) in [
        (
            "assets/EDATA.001",
            if first_is_png {
                "5b8ce344a9d7fe4bdf8780725fc2fc36dce3f297688e62f18c8848fce6fecb8b"
            } else {
                "5933033f7472f8b9d24b8106b03f0ce46e788608b6ee03018d16ec37804de579"
            },
            if first_is_png { "png" } else { "bmp" },
            if first_is_png { 1 } else { 2 },
            if first_is_png { 1 } else { 2 },
        ),
        (
            "assets/EDATA.002",
            "a93a4e651a970119d8da0386846785163291b7cfd67edaac7f6fc37b719fe592",
            "bmp",
            2,
            2,
        ),
        (
            "assets/EDATA.003",
            "8746c347d4cf14daa2e0cc9d41d1f9abebbbb1aae997f31a172610e9ecdbe2dd",
            "bmp",
            2,
            2,
        ),
    ] {
        files.insert(
            path.to_owned(),
            AssetFacts {
                sha256: sha256.to_owned(),
                byte_len: 70,
                image: Some(ImageFacts {
                    format: format.to_owned(),
                    width,
                    height,
                }),
            },
        );
    }
    let base_dats = BTreeMap::from([(
        "synthetic.dat".to_owned(),
        "39fb2c329d34fbdd94bb2a2a596b694597eb00b8b402905ec4920f560210d322".to_owned(),
    )]);
    (catalog, manifest, files, base_dats)
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

#[test]
fn effective_validation_enforces_shared_relationship_cases() {
    let cases = [
        ("bundles/valid/catalog.json", "ok"),
        (
            "relationships/catalog-frozen-admitted-subset.json",
            "potential_membership_mismatch",
        ),
        (
            "relationships/catalog-dangling-index-topic.json",
            "dangling_topic_reference",
        ),
        (
            "relationships/catalog-duplicate-category-command.json",
            "duplicate_category_command",
        ),
        (
            "relationships/catalog-wrong-category-order.json",
            "category_order",
        ),
        (
            "relationships/catalog-duplicate-binding-tuple.json",
            "ambiguous_binding",
        ),
        (
            "relationships/catalog-dangling-binding-topic.json",
            "dangling_topic_reference",
        ),
        (
            "relationships/catalog-dangling-image-reference.json",
            "dangling_image_reference",
        ),
    ];

    for (relative, expected) in cases {
        let catalog = parsed_catalog(relative);
        let result = validate_effective_catalog(&catalog, &effective_facts(&catalog));
        if expected == "ok" {
            result.unwrap_or_else(|error| panic!("{relative}: {error}"));
        } else {
            assert_eq!(result.unwrap_err().code(), expected, "{relative}");
        }
    }
}

#[test]
fn bundle_validation_closes_manifest_files_sources_and_base_pairing() {
    let (catalog, manifest, files, base_dats) = valid_base_inputs();
    validate_bundle(&catalog, &manifest, &files, &base_dats).unwrap();

    let mut changed_manifest = manifest.clone();
    changed_manifest.catalog_sha256 = "0".repeat(64);
    assert_eq!(
        validate_bundle(&catalog, &changed_manifest, &files, &base_dats)
            .unwrap_err()
            .code(),
        "catalog_digest_mismatch"
    );

    let mut missing_file = manifest.clone();
    missing_file.files.remove("assets/EDATA.003");
    assert_eq!(
        validate_bundle(&catalog, &missing_file, &files, &base_dats)
            .unwrap_err()
            .code(),
        "manifest_file_set_mismatch"
    );

    let mut changed_files = files.clone();
    changed_files.get_mut("assets/EDATA.001").unwrap().sha256 = "0".repeat(64);
    assert_eq!(
        validate_bundle(&catalog, &manifest, &changed_files, &base_dats)
            .unwrap_err()
            .code(),
        "file_digest_mismatch"
    );

    let wrong_dats = BTreeMap::from([("SYNTHETIC.DAT".to_owned(), "0".repeat(64))]);
    assert_eq!(
        validate_bundle(&catalog, &manifest, &files, &wrong_dats)
            .unwrap_err()
            .code(),
        "binding_source_mismatch"
    );
}

#[test]
fn bundle_validation_consumes_every_shared_e37_bundle_case() {
    let cases = [
        ("valid", "ok"),
        ("invalid-catalog-hash", "catalog_digest_mismatch"),
        ("invalid-files-catalog-hash", "catalog_digest_mismatch"),
        (
            "invalid-missing-image-file-entry",
            "manifest_file_set_mismatch",
        ),
        ("invalid-extra-file-entry", "manifest_file_set_mismatch"),
        ("invalid-image-file-hash", "file_digest_mismatch"),
        ("invalid-image-descriptor-hash", "image_digest_mismatch"),
        ("invalid-binding-source-hash", "binding_source_mismatch"),
        ("invalid-dangling-source-ref", "dangling_source_ref"),
        ("invalid-base-png", "unsupported_base_image_format"),
        ("invalid-base-format-mismatch", "image_format_mismatch"),
    ];
    for (name, expected) in cases {
        let (catalog, manifest, files, base_dats) = bundle_inputs(name);
        let result = validate_bundle(&catalog, &manifest, &files, &base_dats);
        if expected == "ok" {
            result.unwrap_or_else(|error| panic!("{name}: {error}"));
        } else {
            assert_eq!(result.unwrap_err().code(), expected, "{name}");
        }
    }
}

#[test]
fn bundle_validation_rejects_dangling_provenance_and_selector_capability_mismatch() {
    let (_, manifest, mut files, base_dats) = valid_base_inputs();
    for (relative, expected) in [
        (
            "bundles/invalid-dangling-source-ref/catalog.json",
            "dangling_source_ref",
        ),
        (
            "relationships/catalog-default-binding-faction-selector.json",
            "binding_selector_mismatch",
        ),
        (
            "relationships/catalog-faction-binding-static-selector.json",
            "binding_selector_mismatch",
        ),
    ] {
        let catalog = parsed_catalog(relative);
        files.get_mut("catalog.json").unwrap().sha256 = match relative {
            "bundles/invalid-dangling-source-ref/catalog.json" => {
                "3862055407583db867934ee76bdef48294adc80a7b71d4e78e1681ff195c2c68".to_owned()
            }
            _ => "0".repeat(64),
        };
        let error = validate_bundle(&catalog, &manifest, &files, &base_dats).unwrap_err();
        assert_eq!(error.code(), expected, "{relative}");
    }
}

#[test]
fn bundle_validation_rejects_unreferenced_base_art_but_effective_overrides_may_retain_it() {
    let (mut catalog, manifest, files, base_dats) = valid_base_inputs();
    let selector = catalog
        .topics
        .get_mut("original:60004")
        .unwrap()
        .localized
        .get_mut("1033")
        .unwrap()
        .image_selector
        .as_mut()
        .unwrap();
    selector.empire_image_id = rebellion_data::encyclopedia::NullableBaseImageId::Null;
    validate_effective_catalog(&catalog, &effective_facts(&catalog)).unwrap();
    assert_eq!(
        validate_bundle(&catalog, &manifest, &files, &base_dats)
            .unwrap_err()
            .code(),
        "unexpected_runtime_file"
    );
}

#[test]
fn effective_validation_allows_verified_png_and_permitted_faction_content_replacement() {
    let (mut catalog, manifest, files, base_dats) = valid_base_inputs();
    let localized = catalog
        .topics
        .get_mut("original:60004")
        .unwrap()
        .localized
        .get_mut("1033")
        .unwrap();
    localized.image_selector = None;
    localized.image_id = BaseImageIdField::Null;
    validate_effective_catalog(&catalog, &effective_facts(&catalog)).unwrap();
    assert_eq!(
        validate_bundle(&catalog, &manifest, &files, &base_dats)
            .unwrap_err()
            .code(),
        "binding_selector_mismatch",
        "immutable base proof must remain stricter than an authorized effective replacement"
    );

    let image = catalog.images.get_mut("edata:1").unwrap();
    image.format = "png".to_owned();
    let mut facts = effective_facts(&catalog);
    facts
        .get_mut("edata:1")
        .unwrap()
        .image
        .as_mut()
        .unwrap()
        .format = "png".to_owned();
    validate_effective_catalog(&catalog, &facts).unwrap();
}

#[test]
fn effective_validation_rechecks_typed_asset_path_safety() {
    let (catalog, _, _, _) = valid_base_inputs();
    let exact = runtime_asset_path_with_total_len(256);
    let above = runtime_asset_path_with_total_len(257);

    for (path, accepted) in [
        ("assets/a".to_owned(), true),
        ("assets/nested/image.png".to_owned(), true),
        (exact, true),
        (above, false),
        ("../escape.png".to_owned(), false),
        ("/assets/escape.png".to_owned(), false),
        ("assets/../escape.png".to_owned(), false),
        (r"assets\escape.png".to_owned(), false),
    ] {
        let mut candidate = catalog.clone();
        candidate.images.get_mut("edata:1").unwrap().path = path.clone();
        let result = validate_effective_catalog(&candidate, &effective_facts(&candidate));
        if accepted {
            result.unwrap_or_else(|error| panic!("valid typed path {path:?}: {error}"));
        } else {
            let error = result.expect_err("unsafe typed path must be rejected");
            assert_eq!(error.code(), "unsafe_asset_path", "{path:?}");
            assert_eq!(error.path(), "$.images.edata:1.path", "{path:?}");
        }
    }
}

#[test]
fn effective_validation_rejects_ambiguous_typed_image_choices() {
    let (catalog, _, _, _) = valid_base_inputs();
    let static_image = catalog.topics["original:60001"].localized["1033"]
        .image_id
        .clone();

    for image_id in [BaseImageIdField::Null, static_image] {
        let mut candidate = catalog.clone();
        candidate
            .topics
            .get_mut("original:60004")
            .unwrap()
            .localized
            .get_mut("1033")
            .unwrap()
            .image_id = image_id;

        let error = validate_effective_catalog(&candidate, &effective_facts(&candidate))
            .expect_err("typed static/null plus selector must be rejected");
        assert_eq!(error.code(), "ambiguous_image_selector");
        assert_eq!(error.path(), "$.topics.original:60004.localized.1033");
    }
}

#[test]
fn effective_validation_accepts_each_unambiguous_typed_image_choice() {
    let (catalog, _, _, _) = valid_base_inputs();

    validate_effective_catalog(&catalog, &effective_facts(&catalog)).unwrap();

    for image_id in [
        BaseImageIdField::Null,
        catalog.topics["original:60001"].localized["1033"]
            .image_id
            .clone(),
    ] {
        let mut candidate = catalog.clone();
        let localized = candidate
            .topics
            .get_mut("original:60004")
            .unwrap()
            .localized
            .get_mut("1033")
            .unwrap();
        localized.image_id = image_id;
        localized.image_selector = None;
        validate_effective_catalog(&candidate, &effective_facts(&candidate)).unwrap();
    }
}

#[test]
fn typed_validation_rechecks_every_frozen_topic_sort_component() {
    for field in [
        "algorithm",
        "representable_encoding",
        "representable_fold",
        "unrepresentable",
        "tie_break",
    ] {
        let (mut catalog, manifest, files, base_dats) = valid_base_inputs();
        match field {
            "algorithm" => catalog.topic_sort.algorithm = "bogus".to_owned(),
            "representable_encoding" => {
                catalog.topic_sort.representable_encoding = "bogus".to_owned();
            }
            "representable_fold" => {
                catalog.topic_sort.representable_fold = "bogus".to_owned();
            }
            "unrepresentable" => catalog.topic_sort.unrepresentable = "bogus".to_owned(),
            "tie_break" => catalog.topic_sort.tie_break = "bogus".to_owned(),
            _ => unreachable!(),
        }

        let error = validate_effective_catalog(&catalog, &effective_facts(&catalog))
            .expect_err("mutated typed sort metadata must be rejected");
        assert_eq!(error.code(), "invalid_topic_sort", "{field}");
        assert_eq!(error.path(), format!("$.topic_sort.{field}"), "{field}");

        let error = validate_bundle(&catalog, &manifest, &files, &base_dats)
            .expect_err("immutable validation must also protect sort metadata");
        assert_eq!(error.code(), "invalid_topic_sort", "{field}");
        assert_eq!(error.path(), format!("$.topic_sort.{field}"), "{field}");
    }
}

#[test]
fn effective_validation_scopes_equal_normalized_paths_by_canonical_image_id() {
    let (mut catalog, _, _, _) = valid_base_inputs();
    let mut descriptor = catalog.images["edata:1"].clone();
    descriptor.path = "assets/encyclopedia/assets/test.png".to_owned();
    let demo_id = "mod:v1:64656d6f:encyclopedia/assets/test.png";
    let my_mod_id = "mod:v1:4d794d6f64:encyclopedia/assets/test.png";
    catalog.images.insert(
        rebellion_data::encyclopedia::BaseImageId(demo_id.to_owned()),
        descriptor.clone(),
    );
    descriptor.sha256 =
        "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_owned();
    catalog.images.insert(
        rebellion_data::encyclopedia::BaseImageId(my_mod_id.to_owned()),
        descriptor,
    );

    let facts = effective_facts(&catalog);
    validate_effective_catalog(&catalog, &facts).unwrap();

    let mut collided_facts = facts;
    collided_facts.insert(my_mod_id.to_owned(), collided_facts[demo_id].clone());
    let error = validate_effective_catalog(&catalog, &collided_facts)
        .expect_err("facts from one owner must not satisfy another canonical image ID");
    assert_eq!(error.code(), "image_digest_mismatch");
    assert_eq!(error.path(), format!("$.images.{my_mod_id}.sha256"));
}

#[test]
fn immutable_bundle_rejects_two_base_ids_for_one_bundle_path() {
    let (mut catalog, manifest, files, base_dats) = valid_base_inputs();
    let duplicate = catalog.images["edata:1"].path.clone();
    catalog.images.get_mut("edata:2").unwrap().path = duplicate;

    let error = validate_bundle(&catalog, &manifest, &files, &base_dats)
        .expect_err("one immutable bundle path cannot own two base identities");
    assert_eq!(error.code(), "asset_identity_collision");
    assert_eq!(error.source(), "bundle");
    assert_eq!(error.path(), "$.images.edata:2.path");
}

#[test]
fn effective_title_changes_do_not_reinterpret_registry_membership_as_display_order() {
    let (mut catalog, _, _, _) = valid_base_inputs();
    let index_order = catalog.index.topic_ids.clone();
    let category_orders: Vec<_> = catalog
        .categories
        .iter()
        .map(|category| category.topic_ids.clone())
        .collect();
    catalog
        .topics
        .get_mut("original:60002")
        .unwrap()
        .localized
        .get_mut("1033")
        .unwrap()
        .title = "A renamed effective title".to_owned();

    validate_effective_catalog(&catalog, &effective_facts(&catalog)).unwrap();
    assert_eq!(catalog.index.topic_ids, index_order);
    assert_eq!(
        catalog
            .categories
            .iter()
            .map(|category| category.topic_ids.clone())
            .collect::<Vec<_>>(),
        category_orders
    );
}

#[test]
fn effective_image_facts_close_exactly_and_enforce_checked_resource_budgets() {
    let (catalog, _, _, _) = valid_base_inputs();
    let mut oversized_body = catalog.clone();
    oversized_body
        .topics
        .get_mut("original:60001")
        .unwrap()
        .localized
        .get_mut("1033")
        .unwrap()
        .body = "é".repeat(524_289);
    assert_eq!(
        validate_effective_catalog(&oversized_body, &effective_facts(&oversized_body))
            .unwrap_err()
            .code(),
        "resource_limit:body_bytes"
    );

    let mut missing = effective_facts(&catalog);
    missing.remove("edata:1");
    assert_eq!(
        validate_effective_catalog(&catalog, &missing)
            .unwrap_err()
            .code(),
        "unexpected_runtime_file"
    );
    let mut extra_facts = effective_facts(&catalog);
    extra_facts.insert(
        "edata:999".to_owned(),
        AssetFacts {
            sha256: "9".repeat(64),
            byte_len: 1,
            image: Some(ImageFacts {
                format: "png".to_owned(),
                width: 1,
                height: 1,
            }),
        },
    );
    assert_eq!(
        validate_effective_catalog(&catalog, &extra_facts)
            .unwrap_err()
            .code(),
        "unexpected_runtime_file"
    );

    let mut facts = effective_facts(&catalog);
    facts
        .get_mut("edata:1")
        .unwrap()
        .image
        .as_mut()
        .unwrap()
        .width = 4_000;
    facts
        .get_mut("edata:1")
        .unwrap()
        .image
        .as_mut()
        .unwrap()
        .height = 4_001;
    assert_eq!(
        validate_effective_catalog(&catalog, &facts)
            .unwrap_err()
            .code(),
        "resource_limit:image_pixels"
    );

    let mut exact_catalog = catalog.clone();
    exact_catalog.images.get_mut("edata:1").unwrap().byte_length = 32 * 1024 * 1024;
    let exact_facts = effective_facts(&exact_catalog);
    validate_effective_catalog(&exact_catalog, &exact_facts).unwrap();
    exact_catalog.images.get_mut("edata:1").unwrap().byte_length += 1;
    let above_facts = effective_facts(&exact_catalog);
    assert_eq!(
        validate_effective_catalog(&exact_catalog, &above_facts)
            .unwrap_err()
            .code(),
        "resource_limit:image_bytes"
    );

    let mut exact_pixels = catalog.clone();
    let exact_image = exact_pixels.images.get_mut("edata:1").unwrap();
    exact_image.width = 4_000;
    exact_image.height = 4_000;
    validate_effective_catalog(&exact_pixels, &effective_facts(&exact_pixels)).unwrap();
    exact_pixels.images.get_mut("edata:1").unwrap().height = 4_001;
    assert_eq!(
        validate_effective_catalog(&exact_pixels, &effective_facts(&exact_pixels))
            .unwrap_err()
            .code(),
        "resource_limit:image_pixels"
    );

    let mut catalog = catalog;
    let template = catalog.images["edata:1"].clone();
    catalog.images.clear();
    for index in 1..=4 {
        let mut descriptor = template.clone();
        descriptor.path = format!("assets/BOUNDARY.{index}");
        descriptor.byte_length = 32 * 1024 * 1024;
        descriptor.sha256 = format!("{index:064x}");
        catalog.images.insert(
            rebellion_data::encyclopedia::BaseImageId(format!("edata:{index}")),
            descriptor,
        );
    }
    for topic in catalog.topics.values_mut() {
        for localized in topic.localized.values_mut() {
            localized.image_id = BaseImageIdField::Absent;
            localized.image_selector = None;
        }
    }
    let facts = effective_facts(&catalog);
    validate_effective_catalog(&catalog, &facts).unwrap();
    let mut extra = template;
    extra.path = "assets/BOUNDARY.5".to_owned();
    extra.byte_length = 1;
    extra.sha256 = format!("{:064x}", 5);
    catalog.images.insert(
        rebellion_data::encyclopedia::BaseImageId("edata:5".to_owned()),
        extra,
    );
    let facts = effective_facts(&catalog);
    assert_eq!(
        validate_effective_catalog(&catalog, &facts)
            .unwrap_err()
            .code(),
        "resource_limit:effective_image_bytes"
    );
}

#[test]
fn effective_typed_collection_and_text_boundaries_are_independently_enforced() {
    let (catalog, _, _, _) = valid_base_inputs();

    let mut empty_topics = catalog.clone();
    empty_topics.topics.clear();
    empty_topics.index.topic_ids.clear();
    empty_topics.bindings.clear();
    for category in &mut empty_topics.categories {
        category.topic_ids.clear();
    }
    assert_eq!(
        validate_effective_catalog(&empty_topics, &effective_facts(&empty_topics))
            .unwrap_err()
            .code(),
        "resource_limit:topics"
    );

    let mut topic_boundary = catalog.clone();
    let template = topic_boundary.topics["original:60005"].clone();
    topic_boundary.topics.clear();
    topic_boundary.index.topic_ids.clear();
    topic_boundary.bindings.clear();
    for category in &mut topic_boundary.categories {
        category.topic_ids.clear();
    }
    for dat_id in 1..=10_000_u32 {
        let topic_id = rebellion_data::encyclopedia::TopicId(format!("synthetic:{dat_id}"));
        topic_boundary
            .topics
            .insert(topic_id.clone(), template.clone());
        topic_boundary.index.topic_ids.push(topic_id.clone());
        topic_boundary
            .bindings
            .push(rebellion_data::encyclopedia::CatalogBinding {
                family: "synthetic".to_owned(),
                dat_id,
                variant: "default".to_owned(),
                topic_id,
            });
    }
    for (index, category) in topic_boundary.categories.iter_mut().enumerate() {
        category
            .topic_ids
            .push(rebellion_data::encyclopedia::TopicId(format!(
                "synthetic:{}",
                index + 1
            )));
    }
    validate_effective_catalog(&topic_boundary, &effective_facts(&topic_boundary)).unwrap();
    let extra_id = rebellion_data::encyclopedia::TopicId("synthetic:10001".to_owned());
    topic_boundary.topics.insert(extra_id.clone(), template);
    topic_boundary.index.topic_ids.push(extra_id.clone());
    topic_boundary
        .bindings
        .push(rebellion_data::encyclopedia::CatalogBinding {
            family: "synthetic".to_owned(),
            dat_id: 10_001,
            variant: "default".to_owned(),
            topic_id: extra_id,
        });
    assert_eq!(
        validate_effective_catalog(&topic_boundary, &effective_facts(&topic_boundary))
            .unwrap_err()
            .code(),
        "resource_limit:topics"
    );

    let mut empty_localized = catalog.clone();
    empty_localized
        .topics
        .get_mut("original:60001")
        .unwrap()
        .localized
        .clear();
    assert_eq!(
        validate_effective_catalog(&empty_localized, &effective_facts(&empty_localized))
            .unwrap_err()
            .code(),
        "invalid_localized_record"
    );

    let mut language_boundary = catalog.clone();
    let topic = language_boundary.topics.get_mut("original:60001").unwrap();
    let content = topic.localized["1033"].clone();
    topic.localized.clear();
    for language in 0..256 {
        topic
            .localized
            .insert(language.to_string(), content.clone());
    }
    validate_effective_catalog(&language_boundary, &effective_facts(&language_boundary)).unwrap();
    language_boundary
        .topics
        .get_mut("original:60001")
        .unwrap()
        .localized
        .insert("256".to_owned(), content);
    assert_eq!(
        validate_effective_catalog(&language_boundary, &effective_facts(&language_boundary))
            .unwrap_err()
            .code(),
        "invalid_localized_record"
    );

    for (field, exact, expected) in [
        ("title", 65_536, "resource_limit:title_bytes"),
        ("body", 1_048_576, "resource_limit:body_bytes"),
    ] {
        let mut boundary = catalog.clone();
        let localized = boundary
            .topics
            .get_mut("original:60001")
            .unwrap()
            .localized
            .get_mut("1033")
            .unwrap();
        if field == "title" {
            localized.title = "x".repeat(exact);
        } else {
            localized.body = "x".repeat(exact);
        }
        validate_effective_catalog(&boundary, &effective_facts(&boundary)).unwrap();
        let localized = boundary
            .topics
            .get_mut("original:60001")
            .unwrap()
            .localized
            .get_mut("1033")
            .unwrap();
        if field == "title" {
            localized.title.push('x');
        } else {
            localized.body.push('x');
        }
        assert_eq!(
            validate_effective_catalog(&boundary, &effective_facts(&boundary))
                .unwrap_err()
                .code(),
            expected,
            "{field}"
        );
    }
}

#[test]
fn effective_image_zero_and_mismatch_components_are_independently_enforced() {
    let (catalog, _, _, _) = valid_base_inputs();
    for (width, height) in [(0, 2), (2, 0)] {
        let mut changed = catalog.clone();
        let descriptor = changed.images.get_mut("edata:1").unwrap();
        descriptor.width = width;
        descriptor.height = height;
        assert_eq!(
            validate_effective_catalog(&changed, &effective_facts(&changed))
                .unwrap_err()
                .code(),
            "resource_limit:image_pixels"
        );
    }

    for mismatch in ["length", "width", "height"] {
        let mut facts = effective_facts(&catalog);
        let observed = facts.get_mut("edata:1").unwrap();
        match mismatch {
            "length" => observed.byte_len += 1,
            "width" => observed.image.as_mut().unwrap().width += 1,
            "height" => observed.image.as_mut().unwrap().height += 1,
            _ => unreachable!(),
        }
        assert_eq!(
            validate_effective_catalog(&catalog, &facts)
                .unwrap_err()
                .code(),
            "image_facts_mismatch",
            "{mismatch}"
        );
    }
}

#[test]
fn category_id_and_command_order_are_independently_enforced() {
    let (catalog, _, _, _) = valid_base_inputs();
    let mut wrong_id = catalog.clone();
    wrong_id.categories[0].id = "command:0x71".to_owned();
    assert_eq!(
        validate_effective_catalog(&wrong_id, &effective_facts(&wrong_id))
            .unwrap_err()
            .code(),
        "category_order"
    );
    let mut wrong_command = catalog.clone();
    wrong_command.categories[0].command = "0x71".to_owned();
    assert_eq!(
        validate_effective_catalog(&wrong_command, &effective_facts(&wrong_command))
            .unwrap_err()
            .code(),
        "category_order"
    );
}
