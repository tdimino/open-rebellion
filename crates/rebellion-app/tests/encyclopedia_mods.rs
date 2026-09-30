use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use rebellion_data::mods::ModContentTarget;

#[path = "../src/encyclopedia_mods.rs"]
mod encyclopedia_mods;
#[path = "../src/encyclopedia_session.rs"]
mod encyclopedia_session;

use encyclopedia_mods::ConfinementTestPoint;
use encyclopedia_mods::{EncyclopediaImageOwner, EncyclopediaModEngine, ResolvedEncyclopediaMod};
use encyclopedia_session::{prepare_encyclopedia_session, EncyclopediaBytes};

const VALID_CATALOG: &[u8] =
    include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/catalog.json");
const VALID_MANIFEST: &[u8] =
    include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/manifest.json");
const VALID_IMAGE_1: &[u8] =
    include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.001");
const VALID_IMAGE_2: &[u8] =
    include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.002");
const VALID_IMAGE_3: &[u8] =
    include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.003");
const MOD_PNG_RED: &[u8] =
    include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/images/mod-valid.png");
// Contributor-authored 1x1 RGBA PNG (green); CRCs are part of the literal.
const MOD_PNG_GREEN: &[u8] = &[
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f, 0x15, 0xc4,
    0x89, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9c, 0x63, 0x60, 0xf8, 0xcf, 0xf0,
    0x1f, 0x00, 0x04, 0x01, 0x01, 0xff, 0x71, 0xeb, 0x47, 0xe5, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45,
    0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
];
// Contributor-authored 1x1 RGBA16 PNG. It exercises the renderer's supported
// eight-byte-per-pixel decode path without storing a large fixture.
const MOD_PNG_RGBA16: &[u8] = &[
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x10, 0x06, 0x00, 0x00, 0x00, 0x4f, 0x85, 0x18,
    0xca, 0x00, 0x00, 0x00, 0x11, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9c, 0x63, 0xf8, 0xff, 0x9f, 0x81,
    0xa1, 0x81, 0xe1, 0xff, 0x7f, 0x00, 0x13, 0xf7, 0x04, 0x7d, 0xda, 0x57, 0xb9, 0x9a, 0x00, 0x00,
    0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
];

fn retained(bytes: &[u8]) -> Arc<[u8]> {
    Arc::from(bytes)
}

fn base_session() -> encyclopedia_session::EncyclopediaSession {
    let bytes: EncyclopediaBytes = BTreeMap::from([
        ("catalog.json".to_owned(), retained(VALID_CATALOG)),
        ("manifest.json".to_owned(), retained(VALID_MANIFEST)),
        ("assets/EDATA.001".to_owned(), retained(VALID_IMAGE_1)),
        ("assets/EDATA.002".to_owned(), retained(VALID_IMAGE_2)),
        ("assets/EDATA.003".to_owned(), retained(VALID_IMAGE_3)),
    ]);
    let dats = BTreeMap::from([(
        "SYNTHETIC.DAT".to_owned(),
        "39fb2c329d34fbdd94bb2a2a596b694597eb00b8b402905ec4920f560210d322".to_owned(),
    )]);
    prepare_encyclopedia_session(bytes, &dats).unwrap()
}

static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

struct TempRoot(PathBuf);

impl TempRoot {
    fn new(label: &str) -> Self {
        let serial = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "open-rebellion-e24-{label}-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir_all(&root).unwrap();
        Self(root)
    }

    fn path(&self) -> &Path {
        &self.0
    }

    fn write_image(&self, relative: &str, bytes: &[u8]) {
        let path = self.0.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
    }
}

impl Drop for TempRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn input(
    name: impl Into<String>,
    root: &TempRoot,
    overlay: impl Into<Vec<u8>>,
) -> ResolvedEncyclopediaMod {
    ResolvedEncyclopediaMod {
        name: name.into(),
        root: root.path().to_path_buf(),
        content: ModContentTarget::Bytes(overlay.into()),
    }
}

fn missing(name: impl Into<String>, root: &TempRoot) -> ResolvedEncyclopediaMod {
    ResolvedEncyclopediaMod {
        name: name.into(),
        root: root.path().to_path_buf(),
        content: ModContentTarget::Missing,
    }
}

fn title<'a>(engine: &'a EncyclopediaModEngine, language: &str) -> Option<&'a str> {
    engine.snapshot().catalog().topics["original:60001"]
        .localized
        .get(language)
        .map(|localized| localized.title.as_str())
}

fn text_overlay(title: &str) -> Vec<u8> {
    format!(
        r#"[{{"id":"original:60001","localized":{{"1033":{{"title":{}}}}}}}]"#,
        serde_json::to_string(title).unwrap()
    )
    .into_bytes()
}

fn body_overlay(body: &str) -> Vec<u8> {
    format!(
        r#"[{{"id":"original:60001","localized":{{"1033":{{"body":{}}}}}}}]"#,
        serde_json::to_string(body).unwrap()
    )
    .into_bytes()
}

fn image_overlay() -> Vec<u8> {
    image_overlay_for("original:60001", "encyclopedia/assets/test.png")
}

fn image_overlay_for(topic: &str, path: &str) -> Vec<u8> {
    format!(
        r#"[{{"id":{},"localized":{{"1033":{{"image":{{"path":{}}}}}}}}}]"#,
        serde_json::to_string(topic).unwrap(),
        serde_json::to_string(path).unwrap()
    )
    .into_bytes()
}

#[test]
fn explicit_resolved_order_controls_later_field_precedence_without_another_resolver() {
    let a_root = TempRoot::new("order-a");
    let b_root = TempRoot::new("order-b");
    let base = base_session();

    let mut ab = EncyclopediaModEngine::new(&base).unwrap();
    ab.refresh(vec![
        input("A", &a_root, text_overlay("A title")),
        input("B", &b_root, text_overlay("B title")),
    ])
    .unwrap();
    assert_eq!(title(&ab, "1033"), Some("B title"));

    let mut ba = EncyclopediaModEngine::new(&base).unwrap();
    ba.refresh(vec![
        input("B", &b_root, text_overlay("B title")),
        input("A", &a_root, text_overlay("A title")),
    ])
    .unwrap();
    assert_eq!(title(&ba, "1033"), Some("A title"));
}

#[test]
fn hidden_predecessor_snapshot_updates_without_generation_until_later_owner_is_disabled() {
    let a_root = TempRoot::new("hidden-a");
    let b_root = TempRoot::new("hidden-b");
    let base = base_session();
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();
    engine
        .refresh(vec![
            input("A", &a_root, text_overlay("A one")),
            input("B", &b_root, text_overlay("B visible")),
        ])
        .unwrap();
    let generation = engine.snapshot().generation();

    let hidden = engine
        .refresh(vec![
            input("A", &a_root, text_overlay("A two")),
            input("B", &b_root, text_overlay("B visible")),
        ])
        .unwrap();
    assert!(!hidden.published_changed);
    assert_eq!(engine.snapshot().generation(), generation);
    assert_eq!(title(&engine, "1033"), Some("B visible"));

    let revealed = engine
        .refresh(vec![
            input("A", &a_root, text_overlay("A two")),
            missing("B", &b_root),
        ])
        .unwrap();
    assert!(revealed.published_changed);
    assert_eq!(title(&engine, "1033"), Some("A two"));
    assert_eq!(engine.accepted_mod_names(), vec!["A"]);
}

#[test]
fn malformed_edit_reuses_only_an_independent_last_good_snapshot_and_disabled_mod_never_returns() {
    let a_root = TempRoot::new("recovery-a");
    let b_root = TempRoot::new("recovery-b");
    let base = base_session();
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();
    let a =
        br#"[{"id":"original:60001","localized":{"1041":{"title":"A language","body":"A body"}}}]"#;
    let b = br#"[{"id":"original:60001","localized":{"1041":{"body":"B body"}}}]"#;

    engine
        .refresh(vec![input("A", &a_root, a), input("B", &b_root, b)])
        .unwrap();
    assert_eq!(
        engine.snapshot().catalog().topics["original:60001"].localized["1041"].body,
        "B body"
    );

    let report = engine
        .refresh(vec![
            input("A", &a_root, a),
            input("B", &b_root, b"[truncated".to_vec()),
        ])
        .unwrap();
    assert!(report.diagnostics.iter().any(|item| item.mod_name == "B"));
    assert_eq!(
        engine.snapshot().catalog().topics["original:60001"].localized["1041"].body,
        "B body",
        "B's independently retained patch remains valid while A is present"
    );

    let report = engine
        .refresh(vec![input("B", &b_root, b"[truncated".to_vec())])
        .unwrap();
    assert!(report
        .diagnostics
        .iter()
        .any(|item| { item.mod_name == "B" && item.code == "incomplete_localized_record" }));
    assert!(!engine.snapshot().catalog().topics["original:60001"]
        .localized
        .contains_key("1041"));
    assert_eq!(engine.accepted_mod_names(), vec!["B"]);
}

#[test]
fn invalid_first_load_is_skipped_without_inventing_a_snapshot_or_generation() {
    let root = TempRoot::new("invalid-first");
    let base = base_session();
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();
    let generation = engine.snapshot().generation();

    let report = engine
        .refresh(vec![input("broken", &root, b"{".to_vec())])
        .unwrap();

    assert_eq!(engine.snapshot().generation(), generation);
    assert!(engine.accepted_mod_names().is_empty());
    assert_eq!(report.diagnostics[0].mod_name, "broken");
}

#[test]
fn candidate_parse_reservation_exceeding_the_session_cap_aborts_instead_of_becoming_a_diagnostic() {
    let root = TempRoot::new("candidate-reservation");
    let base = base_session();
    let mut measured = EncyclopediaModEngine::new(&base).unwrap();
    let empty_peak = measured
        .refresh(vec![input("empty", &root, b"[]".to_vec())])
        .unwrap()
        .peak_retained_bytes;

    let mut constrained = EncyclopediaModEngine::new(&base).unwrap();
    constrained.set_retained_limit_for_test(empty_peak);
    let generation = constrained.snapshot().generation();
    let error = constrained
        .refresh(vec![input("large", &root, vec![b' '; 128 * 1024])])
        .expect_err("candidate bytes must be reserved before parsing or diagnostic fallback");

    assert_eq!(error.code(), "resource_limit:retained_bytes");
    assert_eq!(constrained.snapshot().generation(), generation);
    assert!(constrained.accepted_mod_names().is_empty());
}

#[test]
fn incoming_overlay_capacity_is_admitted_before_vec_to_retained_storage_transition() {
    let root = TempRoot::new("incoming-capacity");
    let base = base_session();

    let mut measured = EncyclopediaModEngine::new(&base).unwrap();
    let ordinary_peak = measured
        .refresh(vec![input("ordinary", &root, b"[]".to_vec())])
        .unwrap()
        .peak_retained_bytes;

    let mut oversized_capacity = Vec::with_capacity(256 * 1024);
    oversized_capacity.extend_from_slice(b"[]");
    assert!(oversized_capacity.capacity() >= 256 * 1024);

    let mut constrained = EncyclopediaModEngine::new(&base).unwrap();
    constrained.set_retained_limit_for_test(ordinary_peak);
    let generation = constrained.snapshot().generation();
    let error = constrained
        .refresh(vec![input("capacity", &root, oversized_capacity)])
        .expect_err("incoming allocated capacity must be charged before copying or parsing");

    assert_eq!(error.code(), "resource_limit:retained_bytes");
    assert_eq!(constrained.snapshot().generation(), generation);
    assert!(constrained.accepted_mod_names().is_empty());
}

#[test]
fn base_engine_clones_are_admitted_before_construction_at_the_exact_global_cap() {
    let base = base_session();
    let measured = EncyclopediaModEngine::new(&base).unwrap();
    let exact = measured.retained_bytes();

    let admitted = EncyclopediaModEngine::new_with_retained_limit_for_test(&base, exact)
        .expect("the exact base-engine retained requirement is admitted");
    assert_eq!(admitted.retained_bytes(), exact);

    let error = EncyclopediaModEngine::new_with_retained_limit_for_test(&base, exact - 1)
        .err()
        .expect("base catalog/maps must be rejected before their owned clones are built");
    assert_eq!(error.code(), "resource_limit:retained_bytes");
}

#[test]
fn semantic_apply_failure_is_a_mod_diagnostic_not_a_session_resource_failure() {
    let root = TempRoot::new("semantic-apply-error");
    let base = base_session();
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();
    let generation = engine.snapshot().generation();
    let overlay = br#"[{"id":"original:99999","localized":{"1033":{"title":"unknown"}}}]"#;

    let report = engine
        .refresh(vec![input("bad-topic", &root, overlay)])
        .unwrap();

    assert!(report
        .diagnostics
        .iter()
        .any(|item| item.mod_name == "bad-topic" && item.code == "unknown_topic"));
    assert_eq!(engine.snapshot().generation(), generation);
    assert!(engine.accepted_mod_names().is_empty());
}

#[test]
fn last_good_revalidation_resource_failure_aborts_the_whole_candidate() {
    let root = TempRoot::new("last-good-resource");
    root.write_image("encyclopedia/assets/test.png", MOD_PNG_RED);
    let base = base_session();
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();
    engine
        .refresh(vec![input("images", &root, image_overlay())])
        .unwrap();
    let base_image_bytes: u64 = base
        .base_catalog()
        .images
        .values()
        .map(|image| image.byte_length)
        .sum();
    let effective_image_bytes: u64 = engine
        .snapshot()
        .catalog()
        .images
        .values()
        .map(|image| image.byte_length)
        .sum();
    assert!(effective_image_bytes > base_image_bytes);
    let generation = engine.snapshot().generation();
    let catalog = engine.snapshot().catalog().clone();
    engine.set_effective_image_limit_for_test(base_image_bytes);

    let error = engine
        .refresh(vec![input("images", &root, b"{".to_vec())])
        .expect_err("a resource failure while revalidating last-good state is transaction-wide");

    assert_eq!(error.code(), "resource_limit:effective_image_bytes");
    assert_eq!(engine.snapshot().generation(), generation);
    assert_eq!(engine.snapshot().catalog(), &catalog);
}

#[test]
fn image_bytes_are_inspected_once_retained_by_identity_and_generation_changes_only_on_content() {
    let root = TempRoot::new("image");
    root.write_image("encyclopedia/assets/test.png", MOD_PNG_RED);
    let base = base_session();
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();

    let first = engine
        .refresh(vec![input("MyMod", &root, image_overlay())])
        .unwrap();
    assert!(first.changed_image_ids.len() == 1);
    let image_id = first.changed_image_ids.iter().next().unwrap().clone();
    assert!(image_id.starts_with("mod:v1:4d794d6f64:"));
    assert_eq!(engine.snapshot().image(&image_id).unwrap(), MOD_PNG_RED);
    assert_eq!(
        engine.snapshot().image_owners()[&image_id],
        EncyclopediaImageOwner::Mod {
            mod_name: "MyMod".to_owned(),
            path: "encyclopedia/assets/test.png".to_owned(),
        }
    );
    let first_generation = engine.snapshot().generation();

    let unchanged = engine
        .refresh(vec![input("MyMod", &root, image_overlay())])
        .unwrap();
    assert!(!unchanged.published_changed);
    assert_eq!(engine.snapshot().generation(), first_generation);

    root.write_image("encyclopedia/assets/test.png", MOD_PNG_GREEN);
    let changed = engine
        .refresh(vec![input("MyMod", &root, image_overlay())])
        .unwrap();
    assert!(
        changed.changed_image_ids.contains(&image_id),
        "expected {image_id:?} in {changed:?}"
    );
    assert_eq!(engine.snapshot().generation(), first_generation + 1);
    assert_eq!(engine.snapshot().image(&image_id).unwrap(), MOD_PNG_GREEN);

    fs::write(root.path().join("encyclopedia/assets/test.png"), b"corrupt").unwrap();
    assert_eq!(engine.snapshot().image(&image_id).unwrap(), MOD_PNG_GREEN);
}

#[cfg(any(target_os = "linux", target_os = "android"))]
#[test]
fn rgba16_decode_envelope_is_admitted_before_inspection_and_remains_supported() {
    let root = TempRoot::new("rgba16-admission");
    root.write_image("encyclopedia/assets/test.png", MOD_PNG_RGBA16);
    let base = base_session();
    let inspections = Arc::new(AtomicU64::new(0));

    let mut constrained = EncyclopediaModEngine::new(&base).unwrap();
    constrained.set_retained_limit_for_test(constrained.retained_bytes() + 128 * 1024 * 1024);
    let constrained_inspections = Arc::clone(&inspections);
    constrained.set_confinement_hook_for_test(Some(Arc::new(move |point| {
        if point == ConfinementTestPoint::ImageInspectionStarting {
            constrained_inspections.fetch_add(1, Ordering::SeqCst);
        }
    })));
    let generation = constrained.snapshot().generation();
    let error = constrained
        .refresh(vec![input("rgba16", &root, image_overlay())])
        .expect_err("the full supported decoder envelope must be admitted before inspection");
    assert_eq!(error.code(), "resource_limit:retained_bytes");
    assert_eq!(inspections.load(Ordering::SeqCst), 0);
    assert_eq!(constrained.snapshot().generation(), generation);

    let mut admitted = EncyclopediaModEngine::new(&base).unwrap();
    let admitted_inspections = Arc::clone(&inspections);
    admitted.set_confinement_hook_for_test(Some(Arc::new(move |point| {
        if point == ConfinementTestPoint::ImageInspectionStarting {
            admitted_inspections.fetch_add(1, Ordering::SeqCst);
        }
    })));
    let report = admitted
        .refresh(vec![input("rgba16", &root, image_overlay())])
        .expect("RGBA16 PNG remains a supported replacement with the full budget");
    let image_id = report.changed_image_ids.iter().next().unwrap();
    assert_eq!(admitted.snapshot().image(image_id).unwrap(), MOD_PNG_RGBA16);
    assert!(inspections.load(Ordering::SeqCst) > 0);
}

#[cfg(any(target_os = "linux", target_os = "android"))]
#[test]
fn text_and_last_good_refreshes_do_not_redecode_authenticated_retained_images() {
    let text_root = TempRoot::new("provider-text");
    let base = base_session();
    let mut text_probe = EncyclopediaModEngine::new(&base).unwrap();
    let text_peak = text_probe
        .refresh(vec![input("text", &text_root, text_overlay("changed"))])
        .unwrap()
        .peak_retained_bytes;

    let inspections = Arc::new(AtomicU64::new(0));
    let mut text_engine = EncyclopediaModEngine::new(&base).unwrap();
    text_engine.set_retained_limit_for_test(text_peak);
    let text_inspections = Arc::clone(&inspections);
    text_engine.set_confinement_hook_for_test(Some(Arc::new(move |point| {
        if point == ConfinementTestPoint::ImageInspectionStarting {
            text_inspections.fetch_add(1, Ordering::SeqCst);
        }
    })));
    text_engine
        .refresh(vec![input("text", &text_root, text_overlay("changed"))])
        .expect("text-only refresh fits its exact measured peak");
    assert_eq!(inspections.load(Ordering::SeqCst), 0);

    let image_root = TempRoot::new("provider-last-good");
    image_root.write_image("encyclopedia/assets/test.png", MOD_PNG_RED);
    let mut fallback_probe = EncyclopediaModEngine::new(&base).unwrap();
    fallback_probe
        .refresh(vec![input("image", &image_root, image_overlay())])
        .unwrap();
    let fallback_peak = fallback_probe
        .refresh(vec![input("image", &image_root, b"[".to_vec())])
        .unwrap()
        .peak_retained_bytes;

    let mut fallback = EncyclopediaModEngine::new(&base).unwrap();
    fallback
        .refresh(vec![input("image", &image_root, image_overlay())])
        .unwrap();
    fallback.set_retained_limit_for_test(fallback_peak);
    inspections.store(0, Ordering::SeqCst);
    let fallback_inspections = Arc::clone(&inspections);
    fallback.set_confinement_hook_for_test(Some(Arc::new(move |point| {
        if point == ConfinementTestPoint::ImageInspectionStarting {
            fallback_inspections.fetch_add(1, Ordering::SeqCst);
        }
    })));
    fallback
        .refresh(vec![input("image", &image_root, b"[".to_vec())])
        .expect("last-good replay fits its exact measured peak without redecoding bytes");
    assert_eq!(inspections.load(Ordering::SeqCst), 0);
}

#[test]
fn last_good_large_text_copy_is_admitted_before_overlay_application() {
    let root = TempRoot::new("fallback-copy-admission");
    let base = base_session();
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();
    let large_body = "x".repeat(512 * 1024);
    engine
        .refresh(vec![input("text", &root, body_overlay(&large_body))])
        .unwrap();
    let generation = engine.snapshot().generation();
    let catalog = engine.snapshot().catalog().clone();
    let application_started = Arc::new(AtomicU64::new(0));
    let observed = Arc::clone(&application_started);
    engine.set_confinement_hook_for_test(Some(Arc::new(move |point| {
        if point == ConfinementTestPoint::OverlayApplicationStarting {
            observed.fetch_add(1, Ordering::SeqCst);
        }
    })));
    engine.set_retained_limit_for_test(engine.retained_bytes() + 128 * 1024);

    let error = engine
        .refresh(vec![input("text", &root, b"[".to_vec())])
        .expect_err("last-good localized copies must be admitted before applying the overlay");
    assert_eq!(error.code(), "resource_limit:retained_bytes");
    assert_eq!(application_started.load(Ordering::SeqCst), 0);
    assert_eq!(engine.snapshot().generation(), generation);
    assert_eq!(engine.snapshot().catalog(), &catalog);
    assert_eq!(engine.accepted_mod_names(), vec!["text"]);
}

#[test]
fn externally_held_old_snapshot_remains_charged_until_the_last_owner_drops() {
    let probe_root = TempRoot::new("held-snapshot-probe");
    probe_root.write_image("encyclopedia/assets/test.png", MOD_PNG_RED);
    let base = base_session();
    let mut probe = EncyclopediaModEngine::new(&base).unwrap();
    probe
        .refresh(vec![input("images", &probe_root, image_overlay())])
        .unwrap();
    probe.refresh(vec![missing("images", &probe_root)]).unwrap();
    probe_root.write_image("encyclopedia/assets/test.png", MOD_PNG_GREEN);
    let no_external_peak = probe
        .refresh(vec![input("images", &probe_root, image_overlay())])
        .unwrap()
        .peak_retained_bytes;

    let root = TempRoot::new("held-snapshot");
    root.write_image("encyclopedia/assets/test.png", MOD_PNG_RED);
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();
    engine
        .refresh(vec![input("images", &root, image_overlay())])
        .unwrap();
    let held_old_snapshot = engine.snapshot().clone();
    engine.refresh(vec![missing("images", &root)]).unwrap();
    root.write_image("encyclopedia/assets/test.png", MOD_PNG_GREEN);
    engine.set_retained_limit_for_test(no_external_peak);

    let error = engine
        .refresh(vec![input("images", &root, image_overlay())])
        .expect_err("the externally held old publication must remain globally admitted");
    assert_eq!(error.code(), "resource_limit:retained_bytes");
    assert!(engine.accepted_mod_names().is_empty());

    drop(held_old_snapshot);
    engine
        .refresh(vec![input("images", &root, image_overlay())])
        .expect("capacity returns only after the final old-snapshot owner drops");
}

#[test]
fn externally_held_single_image_remains_charged_after_its_mod_is_removed() {
    let probe_root = TempRoot::new("held-image-probe");
    probe_root.write_image("encyclopedia/assets/test.png", MOD_PNG_RED);
    let base = base_session();
    let mut probe = EncyclopediaModEngine::new(&base).unwrap();
    probe
        .refresh(vec![input("images", &probe_root, image_overlay())])
        .unwrap();
    probe.refresh(vec![missing("images", &probe_root)]).unwrap();
    probe_root.write_image("encyclopedia/assets/test.png", MOD_PNG_GREEN);
    let no_external_peak = probe
        .refresh(vec![input("images", &probe_root, image_overlay())])
        .unwrap()
        .peak_retained_bytes;

    let root = TempRoot::new("held-image");
    root.write_image("encyclopedia/assets/test.png", MOD_PNG_RED);
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();
    let first = engine
        .refresh(vec![input("images", &root, image_overlay())])
        .unwrap();
    let image_id = first.changed_image_ids.iter().next().unwrap().clone();
    let held_image = engine.snapshot().lease_image(&image_id).unwrap();
    engine.refresh(vec![missing("images", &root)]).unwrap();
    root.write_image("encyclopedia/assets/test.png", MOD_PNG_GREEN);
    engine.set_retained_limit_for_test(no_external_peak);

    let error = engine
        .refresh(vec![input("images", &root, image_overlay())])
        .expect_err("a caller-held image buffer must remain globally admitted");
    assert_eq!(error.code(), "resource_limit:retained_bytes");
    assert!(engine.accepted_mod_names().is_empty());

    drop(held_image);
    engine
        .refresh(vec![input("images", &root, image_overlay())])
        .expect("capacity returns only after the final image owner drops");
}

#[test]
fn corrupt_or_missing_image_read_keeps_last_good_bytes_and_releases_failed_candidates() {
    let root = TempRoot::new("image-failure");
    root.write_image("encyclopedia/assets/test.png", MOD_PNG_RED);
    let base = base_session();
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();
    engine
        .refresh(vec![input("images", &root, image_overlay())])
        .unwrap();
    let image_id = engine
        .snapshot()
        .image_ids()
        .find(|id| id.starts_with("mod:v1:"))
        .unwrap()
        .clone();
    let retained_arc = engine.snapshot().lease_image(&image_id).unwrap();
    let generation = engine.snapshot().generation();
    let retained_usage = engine.retained_bytes();

    for bad in [b"plain text".as_slice(), b"still not png".as_slice()] {
        root.write_image("encyclopedia/assets/test.png", bad);
        let report = engine
            .refresh(vec![input("images", &root, image_overlay())])
            .unwrap();
        assert!(report
            .diagnostics
            .iter()
            .any(|item| item.mod_name == "images"));
        assert_eq!(engine.snapshot().generation(), generation);
        assert_eq!(engine.retained_bytes(), retained_usage);
        assert!(std::ptr::eq(
            engine.snapshot().image(&image_id).unwrap(),
            &*retained_arc
        ));
    }

    fs::remove_file(root.path().join("encyclopedia/assets/test.png")).unwrap();
    engine
        .refresh(vec![input("images", &root, image_overlay())])
        .unwrap();
    assert_eq!(engine.snapshot().generation(), generation);
    assert_eq!(engine.retained_bytes(), retained_usage);
    assert!(std::ptr::eq(
        engine.snapshot().image(&image_id).unwrap(),
        &*retained_arc
    ));
}

#[test]
fn combined_mod_effective_image_limit_rejects_the_transaction_not_each_mod() {
    let a_root = TempRoot::new("effective-a");
    let b_root = TempRoot::new("effective-b");
    a_root.write_image("encyclopedia/assets/a.png", MOD_PNG_RED);
    b_root.write_image("encyclopedia/assets/b.png", MOD_PNG_GREEN);
    let base = base_session();
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();
    let base_bytes: u64 = engine
        .snapshot()
        .catalog()
        .images
        .values()
        .map(|image| image.byte_length)
        .sum();
    engine.set_effective_image_limit_for_test(base_bytes + MOD_PNG_RED.len() as u64);

    engine
        .refresh(vec![input(
            "A",
            &a_root,
            image_overlay_for("original:60001", "encyclopedia/assets/a.png"),
        )])
        .unwrap();
    let generation = engine.snapshot().generation();
    let catalog = engine.snapshot().catalog().clone();

    let error = engine
        .refresh(vec![
            input(
                "A",
                &a_root,
                image_overlay_for("original:60001", "encyclopedia/assets/a.png"),
            ),
            input(
                "B",
                &b_root,
                image_overlay_for("original:60004", "encyclopedia/assets/b.png"),
            ),
        ])
        .expect_err("two individually valid replacements exceed the combined session gate");
    assert_eq!(error.code(), "resource_limit:effective_image_bytes");
    assert_eq!(engine.snapshot().generation(), generation);
    assert_eq!(engine.snapshot().catalog(), &catalog);
    assert_eq!(engine.accepted_mod_names(), vec!["A"]);
}

#[test]
fn missing_content_disables_an_old_contribution_and_reports_removed_image_ownership() {
    let root = TempRoot::new("disable");
    root.write_image("encyclopedia/assets/test.png", MOD_PNG_RED);
    let base = base_session();
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();
    let first = engine
        .refresh(vec![input("images", &root, image_overlay())])
        .unwrap();
    let image_id = first.changed_image_ids.iter().next().unwrap().clone();

    let removed = engine.refresh(vec![missing("images", &root)]).unwrap();
    assert!(removed.removed_image_ids.contains(&image_id));
    assert!(!engine.snapshot().contains_image(&image_id));
    assert!(engine.accepted_mod_names().is_empty());
}

#[test]
fn retained_cap_is_checked_at_the_measured_peak_and_missing_mod_names_are_not_preflight_rejected() {
    let root = TempRoot::new("budget");
    root.write_image("encyclopedia/assets/test.png", MOD_PNG_RED);
    let base = base_session();
    let long_name = "x".repeat(2048);

    let mut measured = EncyclopediaModEngine::new(&base).unwrap();
    let first = measured
        .refresh(vec![input(long_name.clone(), &root, image_overlay())])
        .unwrap();
    let exact_peak = first.peak_retained_bytes;

    let mut exact = EncyclopediaModEngine::new(&base).unwrap();
    exact.set_retained_limit_for_test(exact_peak);
    exact
        .refresh(vec![input(long_name.clone(), &root, image_overlay())])
        .expect("the exact measured retained-byte peak is admitted");

    let mut over = EncyclopediaModEngine::new(&base).unwrap();
    let old_generation = over.snapshot().generation();
    over.set_retained_limit_for_test(exact_peak - 1);
    let error = over
        .refresh(vec![input(long_name, &root, image_overlay())])
        .expect_err("one byte below the measured peak must reject");
    assert_eq!(error.code(), "resource_limit:retained_bytes");
    assert_eq!(over.snapshot().generation(), old_generation);

    let mut absent = EncyclopediaModEngine::new(&base).unwrap();
    absent.set_retained_limit_for_test(absent.retained_bytes());
    absent
        .refresh(vec![missing("z".repeat(4096), &root)])
        .expect("an unrelated mod without encyclopedia content has no candidate allocation");
}

#[test]
fn duplicate_exact_names_fail_before_state_change_but_case_and_normalization_distinct_names_do_not_collide(
) {
    let roots = [
        TempRoot::new("names-1"),
        TempRoot::new("names-2"),
        TempRoot::new("names-3"),
        TempRoot::new("names-4"),
        TempRoot::new("names-5"),
    ];
    let base = base_session();
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();
    let before = engine.snapshot().generation();
    let error = engine
        .refresh(vec![
            input("demo", &roots[0], text_overlay("one")),
            input("demo", &roots[1], text_overlay("two")),
        ])
        .unwrap_err();
    assert_eq!(error.code(), "duplicate_mod_name");
    assert_eq!(engine.snapshot().generation(), before);

    engine
        .refresh(vec![
            input("demo", &roots[0], text_overlay("lower")),
            input("Demo", &roots[1], text_overlay("upper")),
            input("e\u{301}", &roots[2], text_overlay("decomposed")),
            input("", &roots[3], text_overlay("empty-name")),
            input("name:with:delimiters", &roots[4], text_overlay("delimited")),
        ])
        .unwrap();
    assert_eq!(title(&engine, "1033"), Some("delimited"));
    assert_eq!(
        engine.accepted_mod_names(),
        vec!["", "Demo", "demo", "e\u{301}", "name:with:delimiters"]
    );
}

#[test]
fn explicit_content_read_error_keeps_last_good_without_reopening_overlay_bytes() {
    let root = TempRoot::new("read-error");
    let base = base_session();
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();
    engine
        .refresh(vec![input("text", &root, text_overlay("retained"))])
        .unwrap();
    let generation = engine.snapshot().generation();

    let report = engine
        .refresh(vec![ResolvedEncyclopediaMod {
            name: "text".to_owned(),
            root: root.path().to_path_buf(),
            content: ModContentTarget::ReadError {
                path: root.path().join("encyclopedia.json"),
                kind: std::io::ErrorKind::PermissionDenied,
                message: "synthetic deterministic read failure".to_owned(),
            },
        }])
        .unwrap();

    assert_eq!(report.diagnostics[0].code, "mod_content_read_error");
    assert_eq!(title(&engine, "1033"), Some("retained"));
    assert_eq!(engine.snapshot().generation(), generation);
}

#[test]
fn faction_pair_loads_both_confined_files_and_publishes_exact_owner_qualified_bytes() {
    let root = TempRoot::new("faction-pair");
    root.write_image("encyclopedia/assets/alliance.png", MOD_PNG_RED);
    root.write_image("encyclopedia/assets/empire.png", MOD_PNG_GREEN);
    let overlay = br#"[{
        "id":"original:60004",
        "localized":{"1041":{
            "title":"Synthetic faction title",
            "body":"Synthetic faction body",
            "image":{
                "alliance":{"path":"encyclopedia/assets/alliance.png"},
                "empire":{"path":"encyclopedia/assets/empire.png"}
            }
        }}
    }]"#;
    let base = base_session();
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();

    let report = engine
        .refresh(vec![input("pair:owner", &root, overlay)])
        .unwrap();

    assert_eq!(report.changed_image_ids.len(), 2);
    let localized = &engine.snapshot().catalog().topics["original:60004"].localized["1041"];
    let selector = localized.image_selector.as_ref().unwrap();
    let alliance = match &selector.alliance_image_id {
        rebellion_data::encyclopedia::NullableBaseImageId::Value(id) => &id.0,
        rebellion_data::encyclopedia::NullableBaseImageId::Null => panic!("alliance image missing"),
    };
    let empire = match &selector.empire_image_id {
        rebellion_data::encyclopedia::NullableBaseImageId::Value(id) => &id.0,
        rebellion_data::encyclopedia::NullableBaseImageId::Null => panic!("empire image missing"),
    };
    assert_eq!(engine.snapshot().image(alliance).unwrap(), MOD_PNG_RED);
    assert_eq!(engine.snapshot().image(empire).unwrap(), MOD_PNG_GREEN);
    assert!(matches!(
        &engine.snapshot().image_owners()[alliance],
        EncyclopediaImageOwner::Mod { mod_name, path }
            if mod_name == "pair:owner" && path.ends_with("alliance.png")
    ));
    assert!(matches!(
        &engine.snapshot().image_owners()[empire],
        EncyclopediaImageOwner::Mod { mod_name, path }
            if mod_name == "pair:owner" && path.ends_with("empire.png")
    ));
}

#[cfg(unix)]
#[test]
fn symlinked_image_components_are_rejected_without_following_the_escape() {
    use std::os::unix::fs::symlink;

    let root = TempRoot::new("symlink-root");
    let outside = TempRoot::new("symlink-outside");
    outside.write_image("test.png", MOD_PNG_RED);
    fs::create_dir_all(root.path().join("encyclopedia")).unwrap();
    symlink(outside.path(), root.path().join("encyclopedia/assets")).unwrap();
    let base = base_session();
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();

    let report = engine
        .refresh(vec![input("escape", &root, image_overlay())])
        .unwrap();

    assert!(report
        .diagnostics
        .iter()
        .any(|item| { item.mod_name == "escape" && item.code == "unsafe_asset_path" }));
    assert!(engine.accepted_mod_names().is_empty());
}

#[cfg(unix)]
#[test]
fn pinned_root_handle_survives_a_deterministic_root_swap_to_an_outside_symlink() {
    use std::os::unix::fs::symlink;
    use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

    assert_eq!(MOD_PNG_RED.len(), MOD_PNG_GREEN.len());
    let root = TempRoot::new("root-swap-inside");
    let outside = TempRoot::new("root-swap-outside");
    root.write_image("encyclopedia/assets/test.png", MOD_PNG_RED);
    outside.write_image("encyclopedia/assets/test.png", MOD_PNG_GREEN);
    let original_root = root.path().to_path_buf();
    let pinned_root = original_root.with_extension("pinned-original");
    let outside_root = outside.path().to_path_buf();
    let swapped = Arc::new(AtomicBool::new(false));

    let base = base_session();
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();
    let swapped_for_hook = Arc::clone(&swapped);
    let original_for_hook = original_root.clone();
    let pinned_for_hook = pinned_root.clone();
    engine.set_confinement_hook_for_test(Some(Arc::new(move |point| {
        if point == ConfinementTestPoint::RootPinned
            && !swapped_for_hook.swap(true, AtomicOrdering::SeqCst)
        {
            fs::rename(&original_for_hook, &pinned_for_hook).unwrap();
            symlink(&outside_root, &original_for_hook).unwrap();
        }
    })));

    let report = engine
        .refresh(vec![input("root-swap", &root, image_overlay())])
        .unwrap();

    fs::remove_file(&original_root).unwrap();
    fs::rename(&pinned_root, &original_root).unwrap();
    assert!(swapped.load(AtomicOrdering::SeqCst));
    let image_id = report.changed_image_ids.iter().next().unwrap();
    assert_eq!(
        engine.snapshot().image(image_id).unwrap(),
        MOD_PNG_RED,
        "the retained bytes must come from the pinned original root, not the swapped path"
    );
}

#[cfg(unix)]
#[test]
fn pinned_parent_handle_survives_a_deterministic_intermediate_swap_to_an_outside_symlink() {
    use std::os::unix::fs::symlink;
    use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

    assert_eq!(MOD_PNG_RED.len(), MOD_PNG_GREEN.len());
    let root = TempRoot::new("parent-swap-inside");
    let outside = TempRoot::new("parent-swap-outside");
    root.write_image("encyclopedia/assets/test.png", MOD_PNG_RED);
    outside.write_image("encyclopedia/assets/test.png", MOD_PNG_GREEN);
    let live_parent = root.path().join("encyclopedia");
    let pinned_parent = root.path().join("encyclopedia.pinned-original");
    let outside_parent = outside.path().join("encyclopedia");
    let swapped = Arc::new(AtomicBool::new(false));

    let base = base_session();
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();
    let swapped_for_hook = Arc::clone(&swapped);
    let live_for_hook = live_parent.clone();
    let pinned_for_hook = pinned_parent.clone();
    engine.set_confinement_hook_for_test(Some(Arc::new(move |point| {
        if point == ConfinementTestPoint::ParentPinned
            && !swapped_for_hook.swap(true, AtomicOrdering::SeqCst)
        {
            fs::rename(&live_for_hook, &pinned_for_hook).unwrap();
            symlink(&outside_parent, &live_for_hook).unwrap();
        }
    })));

    let report = engine
        .refresh(vec![input("parent-swap", &root, image_overlay())])
        .unwrap();

    fs::remove_file(&live_parent).unwrap();
    fs::rename(&pinned_parent, &live_parent).unwrap();
    assert!(swapped.load(AtomicOrdering::SeqCst));
    let image_id = report.changed_image_ids.iter().next().unwrap();
    assert_eq!(
        engine.snapshot().image(image_id).unwrap(),
        MOD_PNG_RED,
        "the retained bytes must come from the pinned parent, not the swapped path"
    );
}

#[cfg(unix)]
#[test]
fn final_symlink_and_nonregular_socket_are_rejected_without_blocking_or_reading() {
    use std::os::unix::fs::symlink;
    use std::os::unix::net::UnixListener;

    let root = TempRoot::new("final-special");
    let outside = TempRoot::new("final-special-outside");
    outside.write_image("outside.png", MOD_PNG_RED);
    fs::create_dir_all(root.path().join("encyclopedia/assets")).unwrap();
    symlink(
        outside.path().join("outside.png"),
        root.path().join("encyclopedia/assets/final.png"),
    )
    .unwrap();
    let socket_path = root.path().join("encyclopedia/assets/socket.png");
    let _listener = UnixListener::bind(&socket_path).unwrap();
    let base = base_session();
    let mut engine = EncyclopediaModEngine::new(&base).unwrap();

    for (name, path) in [
        ("final-link", "encyclopedia/assets/final.png"),
        ("socket", "encyclopedia/assets/socket.png"),
    ] {
        let report = engine
            .refresh(vec![input(
                name,
                &root,
                image_overlay_for("original:60001", path),
            )])
            .unwrap();
        assert!(report.diagnostics.iter().any(|item| {
            item.mod_name == name
                && matches!(item.code, "unsafe_asset_path" | "mod_image_read_error")
        }));
        assert!(engine.accepted_mod_names().is_empty());
    }
}
