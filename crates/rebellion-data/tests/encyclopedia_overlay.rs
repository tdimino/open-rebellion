use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use rebellion_data::encyclopedia::{
    apply_encyclopedia_overlay, parse_catalog, parse_encyclopedia_overlay, AssetFacts, BaseImage,
    BaseImageId, BaseImageIdField, EncyclopediaCatalog, FactionImagePatch, ImageFacts, ImagePatch,
    LocalizedPatch, OverlayImageInputs, PatchField, TopicId, TopicPatch,
};

fn corpus_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tests/fixtures/encyclopedia/fixtures")
}

fn fixture(relative: &str) -> Vec<u8> {
    fs::read(corpus_root().join(relative)).unwrap()
}

fn base_catalog() -> EncyclopediaCatalog {
    parse_catalog(&fixture("bundles/valid/catalog.json")).unwrap()
}

fn canonical_facts(catalog: &EncyclopediaCatalog) -> BTreeMap<String, AssetFacts> {
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

fn png_facts(seed: u8) -> AssetFacts {
    AssetFacts {
        sha256: format!("{seed:02x}").repeat(32),
        byte_len: 70,
        image: Some(ImageFacts {
            format: "png".to_owned(),
            width: 1,
            height: 1,
        }),
    }
}

fn replacement_facts(paths: &[&str]) -> BTreeMap<String, AssetFacts> {
    paths
        .iter()
        .enumerate()
        .map(|(index, path)| ((*path).to_owned(), png_facts((index + 1) as u8)))
        .collect()
}

fn apply(
    catalog: &EncyclopediaCatalog,
    mod_name: &str,
    overlay: &[u8],
    replacements: &BTreeMap<String, AssetFacts>,
    retained_identity_bytes: usize,
) -> Result<EncyclopediaCatalog, rebellion_data::encyclopedia::EncyclopediaError> {
    let patches = parse_encyclopedia_overlay(overlay)?;
    let current = canonical_facts(catalog);
    apply_encyclopedia_overlay(
        catalog,
        mod_name,
        &patches,
        OverlayImageInputs {
            current: &current,
            replacements,
            retained_identity_bytes,
        },
    )
}

fn hex_name(name: &str) -> String {
    name.as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn mod_image_id(name: &str, path: &str) -> String {
    format!("mod:v1:{}:{path}", hex_name(name))
}

#[test]
fn overlay_parser_consumes_the_shared_e37_structural_cases() {
    for relative in [
        "overlays/valid-empty.json",
        "overlays/valid-omitted-image.json",
        "overlays/valid-null-image.json",
        "overlays/valid-empty-body.json",
        "overlays/valid-language-delete.json",
        "overlays/valid-static-bmp.json",
        "overlays/valid-static-png.json",
        "overlays/valid-faction-pair.json",
        "overlays/semantic-static-topic-faction-pair.json",
    ] {
        parse_encyclopedia_overlay(&fixture(relative))
            .unwrap_or_else(|error| panic!("{relative} should parse: {error}"));
    }

    for (relative, code) in [
        (
            "overlays/invalid-faction-pair-missing-side.json",
            "missing_field",
        ),
        ("overlays/invalid-unsafe-path.json", "unsafe_asset_path"),
        ("overlays/invalid-generated-id.json", "unknown_field"),
        (
            "overlays/invalid-binding-edit.json",
            "forbidden_overlay_field",
        ),
        (
            "overlays/invalid-membership-edit.json",
            "forbidden_overlay_field",
        ),
        (
            "overlays/invalid-provenance-edit.json",
            "forbidden_overlay_field",
        ),
        ("overlays/invalid-aliases.json", "unknown_field"),
    ] {
        let error = parse_encyclopedia_overlay(&fixture(relative))
            .expect_err("invalid shared overlay fixture must reject");
        assert_eq!(error.code(), code, "{relative}");
    }
}

#[test]
fn overlay_parser_rejects_raw_duplicate_keys_before_map_insertion() {
    for relative in [
        "raw/overlay-duplicate-selector-key.json",
        "raw/overlay-duplicate-langid-key.json",
        "raw/overlay-duplicate-patch-field.json",
    ] {
        let error = parse_encyclopedia_overlay(&fixture(relative))
            .expect_err("raw duplicate key must reject");
        assert_eq!(error.code(), "duplicate_key", "{relative}");
    }

    let duplicate_selector = br#"[
        {"id":"original:60001","localized":{"1033":{"body":"one"}}},
        {"id":"original:60001","localized":{"1033":{"body":"two"}}}
    ]"#;
    let error = parse_encyclopedia_overlay(duplicate_selector)
        .expect_err("duplicate topic selectors must reject as an ambiguous batch");
    assert_eq!(error.code(), "duplicate_topic_selector");
}

#[test]
fn overlay_deserialization_distinguishes_missing_null_and_value() {
    let patches = parse_encyclopedia_overlay(
        br#"[{
          "id":"original:60001",
          "localized":{
            "1033":{"body":"","image":null},
            "1036":null,
            "1041":{"title":"Synthetic title","body":"Synthetic body","image":{"path":"encyclopedia/assets/test.png"}}
          }
        }]"#,
    )
    .unwrap();

    let patch = &patches[0];
    let english = match &patch.localized["1033"] {
        PatchField::Value(value) => value,
        other => panic!("expected localized value, got {other:?}"),
    };
    assert_eq!(english.title, PatchField::Missing);
    assert_eq!(english.body, PatchField::Value(String::new()));
    assert_eq!(english.image, PatchField::Null);
    assert_eq!(patch.localized["1036"], PatchField::Null);

    let japanese = match &patch.localized["1041"] {
        PatchField::Value(value) => value,
        other => panic!("expected localized value, got {other:?}"),
    };
    assert!(matches!(
        &japanese.image,
        PatchField::Value(ImagePatch::Static { path })
            if path == "encyclopedia/assets/test.png"
    ));
}

#[test]
fn overlay_json_byte_and_depth_limits_precede_typed_allocation() {
    let mut exact = b"[]".to_vec();
    exact.resize(16 * 1024 * 1024, b' ');
    parse_encyclopedia_overlay(&exact).unwrap();
    exact.push(b' ');
    let error = parse_encyclopedia_overlay(&exact).expect_err("one byte above must reject");
    assert_eq!(error.code(), "resource_limit:json_bytes");

    let above_depth = br#"[[[[[[[[[]]]]]]]]]"#;
    let error = parse_encyclopedia_overlay(above_depth).expect_err("depth nine must reject");
    assert_eq!(error.code(), "resource_limit:json_depth");

    let error = parse_encyclopedia_overlay(&[b'[', b'"', 0xff, b'"', b']'])
        .expect_err("invalid UTF-8 must reject before typed parsing");
    assert_eq!(error.code(), "invalid_utf8");
}

#[test]
fn overlay_patch_count_accepts_the_exact_limit_and_rejects_one_above() {
    let make_overlay = |count: usize| {
        let mut document = String::from("[");
        for index in 0..count {
            if index != 0 {
                document.push(',');
            }
            document.push_str(&format!(
                r#"{{"id":"synthetic:{index}","localized":{{"1033":{{"body":"x"}}}}}}"#
            ));
        }
        document.push(']');
        document
    };
    let exact = make_overlay(10_000);
    assert_eq!(
        parse_encyclopedia_overlay(exact.as_bytes()).unwrap().len(),
        10_000
    );
    let above = make_overlay(10_001);
    let error = parse_encyclopedia_overlay(above.as_bytes())
        .expect_err("one patch above the frozen bound must reject");
    assert_eq!(error.code(), "resource_limit:patches");
}

#[test]
fn overlay_text_fields_enforce_exact_utf8_byte_limits_for_wire_and_typed_inputs() {
    let parse_field = |field: &str, value: &str| {
        parse_encyclopedia_overlay(
            format!(
                r#"[{{"id":"original:60001","localized":{{"1033":{{"{field}":"{value}"}}}}}}]"#
            )
            .as_bytes(),
        )
    };

    parse_field("title", &"t".repeat(65_536)).unwrap();
    let error = parse_field("title", &"t".repeat(65_537))
        .expect_err("title one byte above the frozen limit must reject");
    assert_eq!(error.code(), "resource_limit:title_bytes");

    parse_field("body", &"b".repeat(1_048_576)).unwrap();
    let error = parse_field("body", &"b".repeat(1_048_577))
        .expect_err("body one byte above the frozen limit must reject");
    assert_eq!(error.code(), "resource_limit:body_bytes");

    let catalog = base_catalog();
    let current = canonical_facts(&catalog);
    let exact_typed = TopicPatch {
        id: TopicId("original:60001".to_owned()),
        localized: BTreeMap::from([(
            "1033".to_owned(),
            PatchField::Value(LocalizedPatch {
                title: PatchField::Value("t".repeat(65_536)),
                ..LocalizedPatch::default()
            }),
        )]),
    };
    apply_encyclopedia_overlay(
        &catalog,
        "demo",
        &[exact_typed],
        OverlayImageInputs {
            current: &current,
            replacements: &BTreeMap::new(),
            retained_identity_bytes: 0,
        },
    )
    .expect("the exact typed title boundary must remain valid");

    let unknown_oversized = TopicPatch {
        id: TopicId("original:69999".to_owned()),
        localized: BTreeMap::from([(
            "1033".to_owned(),
            PatchField::Value(LocalizedPatch {
                title: PatchField::Value("t".repeat(65_537)),
                ..LocalizedPatch::default()
            }),
        )]),
    };
    let error = apply_encyclopedia_overlay(
        &catalog,
        "demo",
        &[unknown_oversized],
        OverlayImageInputs {
            current: &current,
            replacements: &BTreeMap::new(),
            retained_identity_bytes: 0,
        },
    )
    .expect_err("typed text bounds must be enforced before topic lookup");
    assert_eq!(error.code(), "resource_limit:title_bytes");

    for (field, value, code) in [
        (
            "title",
            PatchField::Value("t".repeat(65_537)),
            "resource_limit:title_bytes",
        ),
        (
            "body",
            PatchField::Value("b".repeat(1_048_577)),
            "resource_limit:body_bytes",
        ),
    ] {
        let localized = if field == "title" {
            LocalizedPatch {
                title: value,
                ..LocalizedPatch::default()
            }
        } else {
            LocalizedPatch {
                body: value,
                ..LocalizedPatch::default()
            }
        };
        let patch = TopicPatch {
            id: TopicId("original:60001".to_owned()),
            localized: BTreeMap::from([("1033".to_owned(), PatchField::Value(localized))]),
        };
        let error = apply_encyclopedia_overlay(
            &catalog,
            "demo",
            &[patch],
            OverlayImageInputs {
                current: &current,
                replacements: &BTreeMap::new(),
                retained_identity_bytes: 0,
            },
        )
        .expect_err("typed callers cannot bypass text byte limits");
        assert_eq!(error.code(), code);
    }
}

#[test]
fn omitted_empty_null_and_language_deletion_have_distinct_merge_meanings() {
    let catalog = base_catalog();
    let replacements = BTreeMap::new();

    let changed = apply(
        &catalog,
        "demo",
        &fixture("overlays/valid-empty-body.json"),
        &replacements,
        0,
    )
    .unwrap();
    let content = &changed.topics["original:60001"].localized["1033"];
    assert_eq!(
        content.title,
        catalog.topics["original:60001"].localized["1033"].title
    );
    assert_eq!(content.body, "");
    assert_eq!(
        content.image_id,
        catalog.topics["original:60001"].localized["1033"].image_id
    );

    let no_art = apply(
        &catalog,
        "demo",
        &fixture("overlays/valid-null-image.json"),
        &replacements,
        0,
    )
    .unwrap();
    assert_eq!(
        no_art.topics["original:60001"].localized["1033"].image_id,
        BaseImageIdField::Null
    );
    assert!(no_art.topics["original:60001"].localized["1033"]
        .image_selector
        .is_none());

    let language_deleted = apply(
        &catalog,
        "demo",
        &fixture("overlays/valid-language-delete.json"),
        &replacements,
        0,
    )
    .unwrap();
    assert!(!language_deleted.topics["original:60001"]
        .localized
        .contains_key("1036"));

    let error = apply(
        &catalog,
        "demo",
        br#"[{"id":"original:60002","localized":{"1033":{"body":null}}}]"#,
        &replacements,
        0,
    )
    .expect_err("deleting a required field must reject");
    assert_eq!(error.code(), "incomplete_localized_record");

    let error = apply(
        &catalog,
        "demo",
        br#"[{"id":"original:60002","localized":{"1033":null}}]"#,
        &replacements,
        0,
    )
    .expect_err("deleting a topic's final language must reject");
    assert_eq!(error.code(), "incomplete_localized_record");
}

#[test]
fn valid_static_png_uses_computed_identity_and_preserves_base_metadata() {
    let catalog = base_catalog();
    let replacements = replacement_facts(&["encyclopedia/assets/test.png"]);
    let changed = apply(
        &catalog,
        "demo",
        &fixture("overlays/valid-static-png.json"),
        &replacements,
        4096,
    )
    .unwrap();
    let image_id = mod_image_id("demo", "encyclopedia/assets/test.png");
    assert_eq!(
        changed.topics["original:60001"].localized["1033"].image_id,
        BaseImageIdField::Value(rebellion_data::encyclopedia::BaseImageId(image_id.clone()))
    );
    let descriptor = &changed.images[image_id.as_str()];
    assert_eq!(descriptor.path, "assets/test.png");
    assert_eq!(descriptor.format, "png");
    assert_eq!(
        descriptor.sha256,
        replacements["encyclopedia/assets/test.png"].sha256
    );

    assert_eq!(catalog.schema_version, changed.schema_version);
    assert_eq!(catalog.default_language, changed.default_language);
    assert_eq!(catalog.topic_sort, changed.topic_sort);
    assert_eq!(catalog.index, changed.index);
    assert_eq!(catalog.categories, changed.categories);
    assert_eq!(catalog.bindings, changed.bindings);
    assert_eq!(
        catalog.topics["original:60001"].source_ref,
        changed.topics["original:60001"].source_ref
    );
    for (base_id, descriptor) in &catalog.images {
        assert_eq!(changed.images[base_id], *descriptor);
    }
}

#[test]
fn faction_pairs_use_immutable_topic_capability_including_new_languages() {
    let catalog = base_catalog();
    let replacements = replacement_facts(&[
        "encyclopedia/assets/alliance.png",
        "encyclopedia/assets/empire.png",
    ]);

    let changed = apply(
        &catalog,
        "demo",
        &fixture("overlays/valid-faction-pair.json"),
        &replacements,
        4096,
    )
    .unwrap();
    let japanese = &changed.topics["original:60004"].localized["1041"];
    assert_eq!(japanese.image_id, BaseImageIdField::Absent);
    let selector = japanese.image_selector.as_ref().unwrap();
    assert_eq!(selector.kind, "viewer_faction");
    assert_eq!(
        selector.alliance_image_id,
        rebellion_data::encyclopedia::NullableBaseImageId::Value(
            rebellion_data::encyclopedia::BaseImageId(mod_image_id(
                "demo",
                "encyclopedia/assets/alliance.png"
            ))
        )
    );

    let error = apply(
        &catalog,
        "demo",
        &fixture("overlays/semantic-static-topic-faction-pair.json"),
        &replacements,
        4096,
    )
    .expect_err("a static base capability must reject a faction pair");
    assert_eq!(error.code(), "image_override_capability");
    assert_eq!(error.topic_id().unwrap().0, "original:60001");
}

#[test]
fn viewer_faction_pair_can_restore_after_an_earlier_static_override() {
    let catalog = base_catalog();
    let first_replacements = replacement_facts(&["encyclopedia/assets/test.png"]);
    let first = apply(
        &catalog,
        "first",
        br#"[{"id":"original:60004","localized":{"1033":{"image":{"path":"encyclopedia/assets/test.png"}}}}]"#,
        &first_replacements,
        4096,
    )
    .unwrap();
    assert!(first.topics["original:60004"].localized["1033"]
        .image_selector
        .is_none());

    let second_replacements = replacement_facts(&[
        "encyclopedia/assets/alliance.png",
        "encyclopedia/assets/empire.png",
    ]);
    let patches = parse_encyclopedia_overlay(
        br#"[{"id":"original:60004","localized":{"1033":{"image":{"alliance":{"path":"encyclopedia/assets/alliance.png"},"empire":{"path":"encyclopedia/assets/empire.png"}}}}}]"#,
    )
    .unwrap();
    let current = canonical_facts(&first);
    let restored = apply_encyclopedia_overlay(
        &first,
        "second",
        &patches,
        OverlayImageInputs {
            current: &current,
            replacements: &second_replacements,
            retained_identity_bytes: 4096,
        },
    )
    .unwrap();
    assert!(restored.topics["original:60004"].localized["1033"]
        .image_selector
        .is_some());
    assert!(
        !restored
            .images
            .contains_key(mod_image_id("first", "encyclopedia/assets/test.png").as_str()),
        "a superseded mod descriptor must not remain in the effective candidate"
    );
}

#[test]
fn bmp_replacements_and_complete_null_faction_pairs_are_valid() {
    let catalog = base_catalog();
    let bmp_path = "encyclopedia/assets/test.bmp";
    let bmp_replacements = BTreeMap::from([(
        bmp_path.to_owned(),
        AssetFacts {
            sha256: "ab".repeat(32),
            byte_len: 70,
            image: Some(ImageFacts {
                format: "bmp".to_owned(),
                width: 2,
                height: 2,
            }),
        },
    )]);
    let changed = apply(
        &catalog,
        "demo",
        &fixture("overlays/valid-static-bmp.json"),
        &bmp_replacements,
        4096,
    )
    .unwrap();
    assert_eq!(
        changed.images[mod_image_id("demo", bmp_path).as_str()].format,
        "bmp"
    );

    let no_side_art = apply(
        &catalog,
        "demo",
        br#"[{"id":"original:60004","localized":{"1033":{"image":{"alliance":null,"empire":null}}}}]"#,
        &BTreeMap::new(),
        0,
    )
    .unwrap();
    let selector = no_side_art.topics["original:60004"].localized["1033"]
        .image_selector
        .as_ref()
        .unwrap();
    assert_eq!(
        selector.alliance_image_id,
        rebellion_data::encyclopedia::NullableBaseImageId::Null
    );
    assert_eq!(
        selector.empire_image_id,
        rebellion_data::encyclopedia::NullableBaseImageId::Null
    );
}

#[test]
fn one_missing_faction_provider_rejects_the_complete_pair_atomically() {
    let catalog = base_catalog();
    let patches = parse_encyclopedia_overlay(&fixture("overlays/valid-faction-pair.json")).unwrap();
    let current = canonical_facts(&catalog);
    let replacements = replacement_facts(&["encyclopedia/assets/alliance.png"]);
    let error = apply_encyclopedia_overlay(
        &catalog,
        "demo",
        &patches,
        OverlayImageInputs {
            current: &current,
            replacements: &replacements,
            retained_identity_bytes: 4096,
        },
    )
    .expect_err("one missing side must reject the entire faction pair");
    assert_eq!(error.code(), "missing_image_provider");
    assert_eq!(catalog, base_catalog());
}

#[test]
fn exact_mod_name_bytes_generate_reversible_collision_free_identities() {
    let catalog = base_catalog();
    let path = "encyclopedia/assets/test.png";
    let replacements = replacement_facts(&[path]);
    let names = ["", ":", "demo", "MyMod", "mymod", "é", "e\u{301}"];
    let mut ids = BTreeSet::new();
    for name in names {
        let changed = apply(
            &catalog,
            name,
            &fixture("overlays/valid-static-png.json"),
            &replacements,
            4096,
        )
        .unwrap_or_else(|error| panic!("exact mod name {name:?} should work: {error}"));
        let expected = mod_image_id(name, path);
        assert!(changed.images.contains_key(expected.as_str()));
        assert!(ids.insert(expected), "byte-distinct names must not collide");
    }
}

#[test]
fn long_mod_names_are_governed_by_candidate_budget_not_a_name_rule() {
    let catalog = base_catalog();
    let path = "encyclopedia/assets/test.png";
    let replacements = replacement_facts(&[path]);
    let name = "n".repeat(1_025);
    let identity_len = 7 + 2 * name.len() + 1 + path.len();
    let reservation = identity_len + name.len() + path.len();

    let accepted = apply(
        &catalog,
        &name,
        &fixture("overlays/valid-static-png.json"),
        &replacements,
        reservation,
    )
    .unwrap();
    assert!(accepted
        .images
        .contains_key(mod_image_id(&name, path).as_str()));

    let error = apply(
        &catalog,
        &name,
        &fixture("overlays/valid-static-png.json"),
        &replacements,
        reservation - 1,
    )
    .expect_err("insufficient candidate budget must reject before publication");
    assert_eq!(error.code(), "resource_limit:retained_bytes");
    assert_eq!(catalog, base_catalog(), "the predecessor remains untouched");
}

#[test]
fn overlay_paths_accept_exactly_256_ascii_bytes_and_reject_257() {
    let exact = format!("encyclopedia/assets/{}xx.png", "a/".repeat(113) + "abc/");
    assert_eq!(exact.len(), 256);
    let above = exact.replacen("xx.png", "xxx.png", 1);
    assert_eq!(above.len(), 257);

    let exact_overlay = format!(
        r#"[{{"id":"original:60001","localized":{{"1033":{{"image":{{"path":"{exact}"}}}}}}}}]"#
    );
    parse_encyclopedia_overlay(exact_overlay.as_bytes()).unwrap();

    let above_overlay = format!(
        r#"[{{"id":"original:60001","localized":{{"1033":{{"image":{{"path":"{above}"}}}}}}}}]"#
    );
    let error = parse_encyclopedia_overlay(above_overlay.as_bytes())
        .expect_err("257-byte author path must reject");
    assert_eq!(error.code(), "unsafe_asset_path");
}

#[test]
fn overlay_paths_enforce_each_segment_and_terminal_extension_boundary() {
    let parse_path = |path: &str| {
        parse_encyclopedia_overlay(
            format!(
                r#"[{{"id":"original:60001","localized":{{"1033":{{"image":{{"path":"{path}"}}}}}}}}]"#
            )
            .as_bytes(),
        )
    };

    let directory_127 = format!("encyclopedia/assets/{}/x.png", "a".repeat(127));
    parse_path(&directory_127).unwrap();
    let directory_128 = format!("encyclopedia/assets/{}/x.png", "a".repeat(128));
    assert_eq!(
        parse_path(&directory_128).unwrap_err().code(),
        "unsafe_asset_path"
    );

    let terminal_125 = format!("encyclopedia/assets/{}.png", "a".repeat(121));
    parse_path(&terminal_125).unwrap();
    let terminal_126 = format!("encyclopedia/assets/{}.png", "a".repeat(122));
    assert_eq!(
        parse_path(&terminal_126).unwrap_err().code(),
        "unsafe_asset_path"
    );

    for invalid in [
        "encyclopedia/assets/no-extension",
        "encyclopedia/assets/.png",
        "encyclopedia/assets//x.png",
        "encyclopedia/assets/image.gif",
    ] {
        assert_eq!(parse_path(invalid).unwrap_err().code(), "unsafe_asset_path");
    }
}

#[test]
fn provider_failures_and_corrupt_predecessor_facts_reject_before_publication() {
    let catalog = base_catalog();
    let patches = parse_encyclopedia_overlay(&fixture("overlays/valid-static-png.json")).unwrap();
    let current = canonical_facts(&catalog);
    let replacements = BTreeMap::new();
    let error = apply_encyclopedia_overlay(
        &catalog,
        "demo",
        &patches,
        OverlayImageInputs {
            current: &current,
            replacements: &replacements,
            retained_identity_bytes: 4096,
        },
    )
    .expect_err("a missing exact-byte provider fact must reject");
    assert_eq!(error.code(), "missing_image_provider");

    let text_patch = parse_encyclopedia_overlay(
        br#"[{"id":"original:60001","localized":{"1033":{"body":"changed"}}}]"#,
    )
    .unwrap();
    let mut corrupt = current;
    corrupt.get_mut("edata:1").unwrap().sha256 = "f".repeat(64);
    let error = apply_encyclopedia_overlay(
        &catalog,
        "demo",
        &text_patch,
        OverlayImageInputs {
            current: &corrupt,
            replacements: &replacements,
            retained_identity_bytes: 0,
        },
    )
    .expect_err("a corrupt predecessor must fail before overlay mutation");
    assert_eq!(error.code(), "image_digest_mismatch");
    assert_eq!(catalog, base_catalog());

    let mut extra = replacement_facts(&["encyclopedia/assets/unused.png"]);
    let error = apply(
        &catalog,
        "demo",
        br#"[{"id":"original:60001","localized":{"1033":{"body":"changed"}}}]"#,
        &extra,
        0,
    )
    .expect_err("unreferenced replacement facts must not enter a candidate");
    assert_eq!(error.code(), "unexpected_runtime_file");
    extra.clear();
}

#[test]
fn replacement_facts_enforce_format_digest_and_image_presence_independently() {
    let catalog = base_catalog();
    let path = "encyclopedia/assets/test.png";
    let overlay = fixture("overlays/valid-static-png.json");

    let mut wrong_format = replacement_facts(&[path]);
    wrong_format
        .get_mut(path)
        .unwrap()
        .image
        .as_mut()
        .unwrap()
        .format = "bmp".to_owned();
    assert_eq!(
        apply(&catalog, "demo", &overlay, &wrong_format, 4096)
            .unwrap_err()
            .code(),
        "image_format_mismatch"
    );

    let mut missing_image = replacement_facts(&[path]);
    missing_image.get_mut(path).unwrap().image = None;
    assert_eq!(
        apply(&catalog, "demo", &overlay, &missing_image, 4096)
            .unwrap_err()
            .code(),
        "image_facts_mismatch"
    );

    for invalid_digest in ["a".repeat(63), "A".repeat(64), "g".repeat(64)] {
        let mut invalid = replacement_facts(&[path]);
        invalid.get_mut(path).unwrap().sha256 = invalid_digest;
        assert_eq!(
            apply(&catalog, "demo", &overlay, &invalid, 4096)
                .unwrap_err()
                .code(),
            "image_facts_mismatch"
        );
    }
}

#[test]
fn generated_identity_collision_checks_descriptor_metadata_and_identical_reuse() {
    let catalog = base_catalog();
    let path = "encyclopedia/assets/test.png";
    let overlay = fixture("overlays/valid-static-png.json");
    let replacement = png_facts(1);
    let image_id = mod_image_id("demo", path);
    let matching_descriptor = BaseImage {
        path: "assets/test.png".to_owned(),
        format: "png".to_owned(),
        byte_length: replacement.byte_len,
        width: 1,
        height: 1,
        sha256: replacement.sha256.clone(),
        source_ref: "runtime:mod-snapshot".to_owned(),
    };

    let mut descriptor_mismatch = catalog.clone();
    let mut wrong_descriptor = matching_descriptor.clone();
    wrong_descriptor.source_ref = "runtime:other-owner".to_owned();
    descriptor_mismatch
        .images
        .insert(BaseImageId(image_id.clone()), wrong_descriptor);
    let current = canonical_facts(&descriptor_mismatch);
    let patches = parse_encyclopedia_overlay(&overlay).unwrap();
    let replacements = BTreeMap::from([(path.to_owned(), replacement.clone())]);
    let error = apply_encyclopedia_overlay(
        &descriptor_mismatch,
        "demo",
        &patches,
        OverlayImageInputs {
            current: &current,
            replacements: &replacements,
            retained_identity_bytes: 4096,
        },
    )
    .unwrap_err();
    assert_eq!(error.code(), "asset_identity_collision");

    let mut identical = catalog;
    identical
        .images
        .insert(BaseImageId(image_id.clone()), matching_descriptor);
    let mut current = canonical_facts(&identical);
    current.insert(image_id.clone(), replacement);
    let accepted = apply_encyclopedia_overlay(
        &identical,
        "demo",
        &patches,
        OverlayImageInputs {
            current: &current,
            replacements: &replacements,
            retained_identity_bytes: 4096,
        },
    )
    .unwrap();
    assert!(accepted.images.contains_key(image_id.as_str()));
}

#[test]
fn generated_identity_budget_is_cumulative_across_distinct_images() {
    let catalog = base_catalog();
    let first = "encyclopedia/assets/first.png";
    let second = "encyclopedia/assets/second.png";
    let overlay = format!(
        r#"[
          {{"id":"original:60001","localized":{{"1033":{{"image":{{"path":"{first}"}}}}}}}},
          {{"id":"original:60002","localized":{{"1033":{{"image":{{"path":"{second}"}}}}}}}}
        ]"#
    );
    let replacements = replacement_facts(&[first, second]);
    let one_reservation = mod_image_id("demo", first).len() + "demo".len() + first.len();
    let error = apply(
        &catalog,
        "demo",
        overlay.as_bytes(),
        &replacements,
        one_reservation,
    )
    .expect_err("one image's budget cannot fund two distinct retained identities");
    assert_eq!(error.code(), "resource_limit:retained_bytes");
}

#[test]
fn a_failing_patch_leaves_the_whole_mod_batch_unapplied() {
    let catalog = base_catalog();
    let patches = parse_encyclopedia_overlay(
        br#"[
          {"id":"original:60001","localized":{"1033":{"body":"would change"}}},
          {"id":"original:69999","localized":{"1033":{"body":"unknown"}}}
        ]"#,
    )
    .unwrap();
    let current = canonical_facts(&catalog);
    let replacements = BTreeMap::new();
    let error = apply_encyclopedia_overlay(
        &catalog,
        "demo",
        &patches,
        OverlayImageInputs {
            current: &current,
            replacements: &replacements,
            retained_identity_bytes: 0,
        },
    )
    .expect_err("unknown topic must reject the complete copy-on-write batch");
    assert_eq!(error.code(), "unknown_topic");
    assert_eq!(
        catalog.topics["original:60001"].localized["1033"].body,
        "Contributor-written system description."
    );
}

#[test]
fn image_patch_debug_shape_keeps_complete_faction_sides_explicit() {
    let patches = parse_encyclopedia_overlay(&fixture("overlays/valid-faction-pair.json")).unwrap();
    let localized = match &patches[0].localized["1041"] {
        PatchField::Value(value) => value,
        _ => panic!("expected localized patch"),
    };
    let pair = match &localized.image {
        PatchField::Value(ImagePatch::ViewerFaction(pair)) => pair,
        _ => panic!("expected faction image pair"),
    };
    assert!(matches!(pair.alliance, FactionImagePatch::Value { .. }));
    assert!(matches!(pair.empire, FactionImagePatch::Value { .. }));
    assert_eq!(LocalizedPatch::default().image, PatchField::Missing);
}

#[test]
fn typed_patch_inputs_recheck_duplicates_empty_maps_and_unsafe_paths() {
    let catalog = base_catalog();
    let current = canonical_facts(&catalog);
    let replacements = BTreeMap::new();
    let duplicate = TopicPatch {
        id: TopicId("original:60001".to_owned()),
        localized: BTreeMap::from([(
            "1033".to_owned(),
            PatchField::Value(LocalizedPatch {
                body: PatchField::Value("typed".to_owned()),
                ..LocalizedPatch::default()
            }),
        )]),
    };
    let error = apply_encyclopedia_overlay(
        &catalog,
        "demo",
        &[duplicate.clone(), duplicate],
        OverlayImageInputs {
            current: &current,
            replacements: &replacements,
            retained_identity_bytes: 0,
        },
    )
    .expect_err("typed callers cannot bypass duplicate selector rejection");
    assert_eq!(error.code(), "duplicate_topic_selector");

    let empty = TopicPatch {
        id: TopicId("original:60001".to_owned()),
        localized: BTreeMap::new(),
    };
    let error = apply_encyclopedia_overlay(
        &catalog,
        "demo",
        &[empty],
        OverlayImageInputs {
            current: &current,
            replacements: &replacements,
            retained_identity_bytes: 0,
        },
    )
    .expect_err("typed empty language maps are outside the frozen shape");
    assert_eq!(error.code(), "resource_limit:languages");

    let unsafe_patch = TopicPatch {
        id: TopicId("original:60001".to_owned()),
        localized: BTreeMap::from([(
            "1033".to_owned(),
            PatchField::Value(LocalizedPatch {
                image: PatchField::Value(ImagePatch::Static {
                    path: "encyclopedia/assets/../escape.png".to_owned(),
                }),
                ..LocalizedPatch::default()
            }),
        )]),
    };
    let error = apply_encyclopedia_overlay(
        &catalog,
        "demo",
        &[unsafe_patch],
        OverlayImageInputs {
            current: &current,
            replacements: &replacements,
            retained_identity_bytes: 4096,
        },
    )
    .expect_err("typed callers cannot bypass author path confinement");
    assert_eq!(error.code(), "unsafe_asset_path");

    let unsafe_pair_on_unknown_topic = TopicPatch {
        id: TopicId("original:69999".to_owned()),
        localized: BTreeMap::from([(
            "1033".to_owned(),
            PatchField::Value(LocalizedPatch {
                image: PatchField::Value(ImagePatch::ViewerFaction(
                    rebellion_data::encyclopedia::FactionImagePair {
                        alliance: FactionImagePatch::Value {
                            path: "encyclopedia/assets/../escape.png".to_owned(),
                        },
                        empire: FactionImagePatch::Null,
                    },
                )),
                ..LocalizedPatch::default()
            }),
        )]),
    };
    let error = apply_encyclopedia_overlay(
        &catalog,
        "demo",
        &[unsafe_pair_on_unknown_topic],
        OverlayImageInputs {
            current: &current,
            replacements: &replacements,
            retained_identity_bytes: 4096,
        },
    )
    .expect_err("typed path validation precedes topic capability lookup");
    assert_eq!(error.code(), "unsafe_asset_path");
}

#[test]
fn distinct_mod_owners_may_use_the_same_confined_relative_path() {
    let catalog = base_catalog();
    let path = "encyclopedia/assets/test.png";
    let first_replacements = replacement_facts(&[path]);
    let first = apply(
        &catalog,
        "demo",
        &fixture("overlays/valid-static-png.json"),
        &first_replacements,
        4096,
    )
    .unwrap();

    let second_patch = parse_encyclopedia_overlay(
        br#"[{"id":"original:60002","localized":{"1033":{"image":{"path":"encyclopedia/assets/test.png"}}}}]"#,
    )
    .unwrap();
    let current = canonical_facts(&first);
    let second_replacements = BTreeMap::from([(path.to_owned(), png_facts(2))]);
    let second = apply_encyclopedia_overlay(
        &first,
        "MyMod",
        &second_patch,
        OverlayImageInputs {
            current: &current,
            replacements: &second_replacements,
            retained_identity_bytes: 4096,
        },
    )
    .unwrap();
    assert!(second
        .images
        .contains_key(mod_image_id("demo", path).as_str()));
    assert!(second
        .images
        .contains_key(mod_image_id("MyMod", path).as_str()));
    assert_eq!(
        second.images[mod_image_id("demo", path).as_str()].path,
        second.images[mod_image_id("MyMod", path).as_str()].path
    );
}

#[test]
fn an_existing_conflicting_generated_identity_rejects_atomically() {
    let mut catalog = base_catalog();
    let path = "encyclopedia/assets/test.png";
    let image_id = mod_image_id("demo", path);
    catalog.images.insert(
        BaseImageId(image_id.clone()),
        BaseImage {
            path: "assets/test.png".to_owned(),
            format: "png".to_owned(),
            byte_length: 70,
            width: 1,
            height: 1,
            sha256: "aa".repeat(32),
            source_ref: "runtime:mod-snapshot".to_owned(),
        },
    );
    let mut current = canonical_facts(&catalog);
    current.insert(image_id, png_facts(0xaa));
    let replacements = replacement_facts(&[path]);
    let patches = parse_encyclopedia_overlay(&fixture("overlays/valid-static-png.json")).unwrap();
    let error = apply_encyclopedia_overlay(
        &catalog,
        "demo",
        &patches,
        OverlayImageInputs {
            current: &current,
            replacements: &replacements,
            retained_identity_bytes: 4096,
        },
    )
    .expect_err("one canonical identity cannot silently change ownership facts");
    assert_eq!(error.code(), "asset_identity_collision");
}

#[test]
fn numeric_and_unknown_topic_selectors_fail_at_the_correct_layer() {
    let error = parse_encyclopedia_overlay(br#"[{"id":60001,"localized":{"1033":{"body":"x"}}}]"#)
        .expect_err("topic selectors are strings, not numeric DAT IDs");
    assert_eq!(error.code(), "invalid_type");

    let catalog = base_catalog();
    let replacements = BTreeMap::new();
    let error = apply(
        &catalog,
        "demo",
        &fixture("overlays/semantic-unknown-topic.json"),
        &replacements,
        0,
    )
    .expect_err("valid wire IDs still require an existing canonical topic");
    assert_eq!(error.code(), "unknown_topic");
}
