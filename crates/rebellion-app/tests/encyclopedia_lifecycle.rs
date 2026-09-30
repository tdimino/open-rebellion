use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use rebellion_core::world::{GameWorld, GnprtbEntry, GnprtbParams};
use rebellion_data::mods::{ModConfig, ModManifest, ModRuntime};

#[path = "../src/encyclopedia_lifecycle.rs"]
mod encyclopedia_lifecycle;
#[path = "../src/encyclopedia_mods.rs"]
mod encyclopedia_mods;
#[path = "../src/encyclopedia_session.rs"]
mod encyclopedia_session;

use encyclopedia_lifecycle::{
    apply_resolved_mod_update, EncyclopediaLifecycle, ModLifecycleTrigger,
};
use encyclopedia_session::{
    prepare_encyclopedia_session, EncyclopediaAvailability, EncyclopediaBytes,
};

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

fn retained(bytes: &[u8]) -> Arc<[u8]> {
    Arc::from(bytes)
}

fn base_availability() -> EncyclopediaAvailability {
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
    EncyclopediaAvailability::Ready(prepare_encyclopedia_session(bytes, &dats).unwrap())
}

static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

struct TempRoot(PathBuf);

impl TempRoot {
    fn new(label: &str) -> Self {
        let serial = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "open-rebellion-e50-{label}-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir_all(&root).unwrap();
        Self(root)
    }

    fn path(&self) -> &Path {
        &self.0
    }

    fn write_overlay(&self, bytes: &[u8]) {
        fs::write(self.0.join("encyclopedia.json"), bytes).unwrap();
    }
}

impl Drop for TempRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn manifest(name: &str, root: &TempRoot, dependencies: &[(&str, &str)]) -> ModManifest {
    ModManifest {
        name: name.to_owned(),
        version: "1.0.0".to_owned(),
        author: "Contributor".to_owned(),
        description: "Synthetic lifecycle fixture".to_owned(),
        dependencies: dependencies
            .iter()
            .map(|(name, requirement)| ((*name).to_owned(), (*requirement).to_owned()))
            .collect::<HashMap<_, _>>(),
        path: root.path().to_path_buf(),
        enabled: true,
    }
}

fn parameter_world() -> GameWorld {
    let entry = |parameter_id, development| GnprtbEntry {
        parameter_id,
        development,
        alliance_sp_easy: development,
        alliance_sp_medium: development,
        alliance_sp_hard: development,
        empire_sp_easy: development,
        empire_sp_medium: development,
        empire_sp_hard: development,
        multiplayer: development,
    };
    GameWorld {
        gnprtb: GnprtbParams::new(vec![entry(77, 1), entry(78, 314)]),
        difficulty_index: 6,
        ..GameWorld::default()
    }
}

fn simulation_fingerprint(world: &GameWorld, rng: &Xoshiro256PlusPlus) -> Vec<u8> {
    serde_json::to_vec(&(world, rng)).unwrap()
}

#[test]
fn lifecycle_triggers_apply_world_only_for_startup_new_campaign_and_manual_reload() {
    let root = TempRoot::new("trigger-matrix");
    let manifest = manifest("fixture", &root, &[]);
    let ordered = [&manifest];
    let cases = [
        (ModLifecycleTrigger::Startup, 1),
        (ModLifecycleTrigger::NewCampaign, 1),
        (ModLifecycleTrigger::ManualReload, 1),
        (ModLifecycleTrigger::Toggle, 0),
        (ModLifecycleTrigger::SavedWorldLoad, 0),
        (ModLifecycleTrigger::ContentOnly, 0),
    ];

    for (trigger, expected_world_calls) in cases {
        let mut world_calls = 0;
        let mut content_calls = 0;
        let report = apply_resolved_mod_update(
            trigger,
            &ordered,
            |seen| {
                world_calls += 1;
                seen.iter()
                    .map(|item| item.name.clone())
                    .collect::<Vec<_>>()
            },
            |seen| {
                content_calls += 1;
                seen.iter()
                    .map(|item| item.name.clone())
                    .collect::<Vec<_>>()
            },
        );

        assert_eq!(world_calls, expected_world_calls, "trigger={trigger:?}");
        assert_eq!(content_calls, 1, "trigger={trigger:?}");
        assert_eq!(report.content, vec!["fixture".to_owned()]);
        let expected = (expected_world_calls == 1).then(|| vec!["fixture".to_owned()]);
        assert_eq!(report.world, expected);
    }
}

#[test]
fn one_resolved_dependency_order_is_observed_by_both_world_and_content_consumers() {
    let a_root = TempRoot::new("shared-order-a");
    let b_root = TempRoot::new("shared-order-b");
    let a = manifest("A", &a_root, &[]);
    let b = manifest("B", &b_root, &[("A", ">=1.0.0")]);
    let runtime = ModRuntime {
        discovered: vec![b, a],
        config: ModConfig {
            enabled: vec!["A".to_owned(), "B".to_owned()],
        },
        errors: Vec::new(),
        mods_dir: a_root.path().join("mods"),
    };
    let ordered = runtime.enabled_sorted();

    let report = apply_resolved_mod_update(
        ModLifecycleTrigger::Startup,
        &ordered,
        |seen| {
            seen.iter()
                .map(|item| item.name.clone())
                .collect::<Vec<_>>()
        },
        |seen| {
            seen.iter()
                .map(|item| item.name.clone())
                .collect::<Vec<_>>()
        },
    );

    assert_eq!(report.world, Some(vec!["A".to_owned(), "B".to_owned()]));
    assert_eq!(report.content, vec!["A".to_owned(), "B".to_owned()]);
}

#[test]
fn malformed_encyclopedia_content_does_not_block_the_world_consumer() {
    let root = TempRoot::new("malformed-content");
    fs::write(
        root.path().join("gnprtb.json"),
        br#"[{"id":77,"development":9}]"#,
    )
    .unwrap();
    root.write_overlay(br#"[{"id":"original:60001","localized":"#);
    let manifest = manifest("world-still-valid", &root, &[]);
    let ordered = [&manifest];
    let runtime = ModRuntime {
        discovered: vec![manifest.clone()],
        config: ModConfig::default(),
        errors: Vec::new(),
        mods_dir: root.path().to_path_buf(),
    };
    let mut world = parameter_world();
    let mut lifecycle = EncyclopediaLifecycle::from_availability(base_availability());

    let report = apply_resolved_mod_update(
        ModLifecycleTrigger::Startup,
        &ordered,
        |seen| runtime.apply_ordered(&mut world, seen),
        |seen| lifecycle.refresh_resolved(seen),
    );

    assert!(report.world.as_ref().unwrap().is_empty());
    assert_eq!(world.gnprtb.value(77, 0), 9);
    assert_eq!(world.gnprtb.value(78, 0), 314);
    assert!(!report.content.published_changed);
    assert!(
        report.content.diagnostics.iter().any(|diagnostic| {
            diagnostic.mod_name == "world-still-valid"
                && diagnostic.path.contains("encyclopedia.json#$")
                && diagnostic.message.contains("overlay")
        }),
        "diagnostics: {:?}",
        report.content.diagnostics
    );
    let manager_diagnostic = lifecycle
        .diagnostic_for_mod("world-still-valid")
        .expect("the Mod Manager must receive the content diagnostic");
    assert!(manager_diagnostic.contains("invalid_json"));
    assert!(manager_diagnostic.contains("encyclopedia.json#$"));
}

#[test]
fn malformed_world_json_does_not_block_content_only_encyclopedia_refresh() {
    let root = TempRoot::new("malformed-world-valid-content");
    fs::write(root.path().join("systems.json"), b"not valid world JSON").unwrap();
    root.write_overlay(
        br#"[{"id":"original:60001","localized":{"1033":{"title":"Content survives"}}}]"#,
    );
    let manifest = manifest("content-still-valid", &root, &[]);
    let mut lifecycle = EncyclopediaLifecycle::from_availability(base_availability());

    let report = lifecycle.refresh_content_only(&[&manifest]);

    assert!(report.published_changed, "content report: {report:?}");
    assert!(report.diagnostics.is_empty(), "content report: {report:?}");
}

#[test]
fn aggregate_raw_targets_are_admitted_before_the_second_buffer_is_retained() {
    let first_root = TempRoot::new("aggregate-first");
    let second_root = TempRoot::new("aggregate-second");
    let first_bytes =
        br#"[{"id":"original:60001","localized":{"1033":{"title":"First accepted"}}}]"#;
    let second_bytes =
        br#"[{"id":"original:60001","localized":{"1033":{"title":"Second accepted"}}}]"#;
    first_root.write_overlay(first_bytes);
    second_root.write_overlay(second_bytes);
    let first = manifest("first", &first_root, &[]);
    let second = manifest("second", &second_root, &[]);
    let mut lifecycle = EncyclopediaLifecycle::from_availability(base_availability());
    let accepted = lifecycle.refresh_resolved(&[&first]);
    assert!(accepted.published_changed, "initial report: {accepted:?}");
    let old_generation = accepted.generation;

    let identity_bytes = [&first, &second]
        .iter()
        .map(|manifest| manifest.name.len() + manifest.path.as_os_str().len())
        .sum::<usize>() as u64;
    let target_slots =
        (2 * std::mem::size_of::<encyclopedia_mods::ResolvedEncyclopediaMod>()) as u64;
    let diagnostic_bytes = [&first, &second]
        .iter()
        .map(|manifest| {
            manifest.path.as_os_str().as_encoded_bytes().len() + 1 + "encyclopedia.json".len() + 256
        })
        .sum::<usize>() as u64;
    let current = lifecycle.retained_bytes_for_test();
    lifecycle.set_retained_limit_for_test(
        current
            + identity_bytes
            + target_slots
            + diagnostic_bytes
            + u64::try_from(first_bytes.len()).unwrap(),
    );

    let rejected = lifecycle.refresh_resolved(&[&first, &second]);

    assert!(!rejected.published_changed);
    assert_eq!(rejected.generation, old_generation);
    assert_eq!(
        lifecycle.admitted_raw_reads_for_test(),
        2,
        "the initial accepted read plus only the first read in the rejected batch may reach the post-admission/pre-allocation boundary"
    );
    assert_eq!(
        rejected.diagnostics[0].code,
        "resource_limit:retained_bytes"
    );
    lifecycle.set_retained_limit_for_test(512 * 1024 * 1024);
    let retried = lifecycle.refresh_resolved(&[&first, &second]);
    assert!(retried.published_changed, "retry report: {retried:?}");
    assert_eq!(retried.generation, old_generation.map(|value| value + 1));
    assert_eq!(lifecycle.admitted_raw_reads_for_test(), 4);
}

#[test]
fn real_world_application_skips_reserved_bytes_and_content_reads_them_once() {
    const LARGE_VALID_TARGET_BYTES: u64 = 1024 * 1024;

    let root = TempRoot::new("world-skips-content-target");
    fs::write(
        root.path().join("gnprtb.json"),
        br#"[{"id":77,"development":29}]"#,
    )
    .unwrap();
    let mut target = fs::File::create(root.path().join("encyclopedia.json")).unwrap();
    target.write_all(b"[").unwrap();
    std::io::copy(
        &mut std::io::repeat(b' ').take(LARGE_VALID_TARGET_BYTES - 2),
        &mut target,
    )
    .unwrap();
    target.write_all(b"]").unwrap();
    drop(target);

    let manifest = manifest("large-content", &root, &[]);
    let ordered = [&manifest];
    let runtime = ModRuntime {
        discovered: vec![manifest.clone()],
        config: ModConfig::default(),
        errors: Vec::new(),
        mods_dir: root.path().to_path_buf(),
    };
    let mut world = parameter_world();
    let mut lifecycle = EncyclopediaLifecycle::from_availability(base_availability());

    let report = apply_resolved_mod_update(
        ModLifecycleTrigger::Startup,
        &ordered,
        |seen| runtime.apply_ordered(&mut world, seen),
        |seen| lifecycle.refresh_resolved(seen),
    );

    assert!(report.world.as_ref().unwrap().is_empty());
    assert_eq!(world.gnprtb.value(77, 0), 29);
    assert_eq!(lifecycle.admitted_raw_reads_for_test(), 1);
    assert_eq!(lifecycle.attempted_target_reads_for_test(), 1);
    assert!(report.content.diagnostics.is_empty(), "report: {report:?}");
}

#[cfg(any(target_os = "linux", target_os = "android"))]
#[test]
fn read_error_diagnostics_are_pre_admitted_before_any_target_reader_starts() {
    use std::os::unix::fs::symlink;

    const DIAGNOSTIC_MESSAGE_ENVELOPE: u64 = 256;

    let directory_root = TempRoot::new("diagnostic-directory");
    fs::create_dir(directory_root.path().join("encyclopedia.json")).unwrap();
    let symlink_root = TempRoot::new("diagnostic-symlink");
    let outside = symlink_root.path().join("outside.json");
    fs::write(&outside, b"[]").unwrap();
    symlink(&outside, symlink_root.path().join("encyclopedia.json")).unwrap();
    let directory_manifest = manifest("directory-target", &directory_root, &[]);
    let symlink_manifest = manifest("symlink-target", &symlink_root, &[]);
    let ordered = [&directory_manifest, &symlink_manifest];
    let mut lifecycle = EncyclopediaLifecycle::from_availability(base_availability());
    let generation = lifecycle
        .refresh_resolved(&[])
        .generation
        .expect("a validated base has a generation");
    let current = lifecycle.retained_bytes_for_test();
    let vector_slots = u64::try_from(
        ordered.len() * std::mem::size_of::<encyclopedia_mods::ResolvedEncyclopediaMod>(),
    )
    .unwrap();
    let identities = ordered
        .iter()
        .map(|item| item.name.len() + item.path.as_os_str().as_encoded_bytes().len())
        .sum::<usize>() as u64;
    let diagnostic_envelope = ordered
        .iter()
        .map(|item| {
            u64::try_from(
                item.path.as_os_str().as_encoded_bytes().len() + 1 + "encyclopedia.json".len(),
            )
            .unwrap()
                + DIAGNOSTIC_MESSAGE_ENVELOPE
        })
        .sum::<u64>();
    assert!(diagnostic_envelope > 0);
    lifecycle.set_retained_limit_for_test(current + vector_slots + identities);

    let rejected = lifecycle.refresh_resolved(&ordered);

    assert!(!rejected.published_changed);
    assert_eq!(rejected.generation, Some(generation));
    assert_eq!(
        rejected.diagnostics[0].code,
        "resource_limit:retained_bytes"
    );
    assert_eq!(
        lifecycle.attempted_target_reads_for_test(),
        0,
        "diagnostic storage must be reserved before constructing the first ReadError"
    );

    lifecycle.set_retained_limit_for_test(512 * 1024 * 1024);
    let retried = lifecycle.refresh_resolved(&ordered);
    assert!(!retried.published_changed);
    assert_eq!(retried.generation, Some(generation));
    assert_eq!(lifecycle.attempted_target_reads_for_test(), 2);
    assert_eq!(retried.diagnostics.len(), 2, "report: {retried:?}");
    assert!(retried
        .diagnostics
        .iter()
        .all(|diagnostic| diagnostic.code == "mod_content_read_error"));
    assert!(retried.diagnostics.iter().all(|diagnostic| {
        !diagnostic.message.is_empty()
            && diagnostic.message.len() <= DIAGNOSTIC_MESSAGE_ENVELOPE as usize
            && diagnostic.path.contains("encyclopedia.json")
    }));
}

#[test]
fn fixed_list_content_refresh_preserves_simulation_fingerprints_and_stable_generation() {
    let root = TempRoot::new("content-only");
    root.write_overlay(br#"[{"id":"original:60001","localized":{"1033":{"title":"First"}}}]"#);
    let manifest = manifest("text-edit", &root, &[]);
    let ordered = [&manifest];
    let mut lifecycle = EncyclopediaLifecycle::from_availability(base_availability());
    let world = parameter_world();
    let rng = Xoshiro256PlusPlus::seed_from_u64(0xcafe);

    let startup = apply_resolved_mod_update(
        ModLifecycleTrigger::Startup,
        &ordered,
        |_| Vec::<rebellion_data::mods::ModError>::new(),
        |seen| lifecycle.refresh_resolved(seen),
    );
    assert!(
        startup.content.published_changed,
        "content refresh report: {:?}",
        startup.content
    );
    let generation = startup.content.generation.unwrap();
    let stable_simulation = simulation_fingerprint(&world, &rng);

    root.write_overlay(br#"[{"id":"original:60001","localized":{"1033":{"title":"Second"}}}]"#);
    let edited = lifecycle.refresh_content_only(&ordered);
    assert!(edited.published_changed);
    assert_eq!(edited.generation, Some(generation + 1));
    assert_eq!(simulation_fingerprint(&world, &rng), stable_simulation);

    let unchanged = lifecycle.refresh_content_only(&ordered);
    assert!(!unchanged.published_changed);
    assert_eq!(unchanged.generation, edited.generation);
    assert_eq!(simulation_fingerprint(&world, &rng), stable_simulation);
}

#[test]
fn toggle_to_an_empty_resolved_order_removes_content_without_replaying_world_patches() {
    let root = TempRoot::new("toggle-disable");
    root.write_overlay(br#"[{"id":"original:60001","localized":{"1033":{"title":"Enabled"}}}]"#);
    let manifest = manifest("toggle-content", &root, &[]);
    let mut lifecycle = EncyclopediaLifecycle::from_availability(base_availability());
    let initial = lifecycle.refresh_resolved(&[&manifest]);
    assert!(initial.published_changed);
    let generation = initial.generation.unwrap();

    let disabled = apply_resolved_mod_update(
        ModLifecycleTrigger::Toggle,
        &[],
        |_| panic!("a toggle must not replay world patches"),
        |ordered| lifecycle.refresh_resolved(ordered),
    );

    assert!(disabled.world.is_none());
    assert!(disabled.content.published_changed);
    assert_eq!(disabled.content.generation, Some(generation + 1));
}

#[test]
fn toggle_saved_load_and_content_only_never_enter_the_real_world_adapter() {
    let root = TempRoot::new("non-world-triggers");
    fs::write(
        root.path().join("gnprtb.json"),
        br#"[{"id":77,"development":42}]"#,
    )
    .unwrap();
    let manifest = manifest("world-patch", &root, &[]);
    let runtime = ModRuntime {
        discovered: vec![manifest.clone()],
        config: ModConfig::default(),
        errors: Vec::new(),
        mods_dir: root.path().to_path_buf(),
    };

    for trigger in [
        ModLifecycleTrigger::Toggle,
        ModLifecycleTrigger::SavedWorldLoad,
        ModLifecycleTrigger::ContentOnly,
    ] {
        let mut world = parameter_world();
        let mut lifecycle = EncyclopediaLifecycle::from_availability(base_availability());
        let update = apply_resolved_mod_update(
            trigger,
            &[&manifest],
            |ordered| runtime.apply_ordered(&mut world, ordered),
            |ordered| lifecycle.refresh_resolved(ordered),
        );

        assert!(update.world.is_none(), "trigger={trigger:?}");
        assert_eq!(world.gnprtb.value(77, 0), 1, "trigger={trigger:?}");
        assert_eq!(world.gnprtb.value(78, 0), 314, "trigger={trigger:?}");
    }
}

#[test]
fn unavailable_base_content_does_not_block_the_world_consumer() {
    let root = TempRoot::new("unavailable-base");
    let manifest = manifest("world-only", &root, &[]);
    let mut lifecycle = EncyclopediaLifecycle::from_availability(
        EncyclopediaAvailability::Unavailable("bundle_absent: synthetic".to_owned()),
    );
    let mut world_calls = 0;

    let update = apply_resolved_mod_update(
        ModLifecycleTrigger::Startup,
        &[&manifest],
        |_| world_calls += 1,
        |ordered| lifecycle.refresh_resolved(ordered),
    );

    assert_eq!(world_calls, 1);
    assert_eq!(update.world, Some(()));
    assert_eq!(update.content.generation, None);
    assert_eq!(
        update.content.diagnostics[0].code,
        "encyclopedia_unavailable"
    );
    assert!(update.content.diagnostics[0]
        .message
        .contains("bundle_absent"));
    let manager_diagnostic = lifecycle
        .diagnostic_for_mod("world-only")
        .expect("global content failure must reach the Mod Manager");
    assert!(manager_diagnostic.contains("encyclopedia_unavailable"));
}
