use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use rebellion_core::world::{GameWorld, GnprtbEntry, GnprtbParams};
use rebellion_data::mods::{ModRuntime, ModWatchPoll, ModWatcher};

#[path = "../src/encyclopedia_lifecycle.rs"]
mod encyclopedia_lifecycle;
#[path = "../src/encyclopedia_mods.rs"]
mod encyclopedia_mods;
#[path = "../src/encyclopedia_session.rs"]
mod encyclopedia_session;
#[path = "../src/encyclopedia_watcher.rs"]
mod encyclopedia_watcher;

use encyclopedia_lifecycle::EncyclopediaLifecycle;
use encyclopedia_session::{
    prepare_encyclopedia_session, EncyclopediaAvailability, EncyclopediaBytes,
};
use encyclopedia_watcher::EncyclopediaWatcher;

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
            "open-rebellion-e25-{label}-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir_all(&root).unwrap();
        Self(root)
    }

    fn path(&self) -> &Path {
        &self.0
    }

    fn mod_path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }

    fn write_mod(&self, name: &str, dependencies: &[(&str, &str)], overlay: &[u8]) {
        let root = self.mod_path(name);
        fs::create_dir_all(&root).unwrap();
        let mut manifest = format!(
            "name = {:?}\nversion = \"1.0.0\"\nauthor = \"Contributor\"\ndescription = \"Synthetic watcher fixture\"\n",
            name
        );
        if !dependencies.is_empty() {
            manifest.push_str("\n[dependencies]\n");
            for (dependency, requirement) in dependencies {
                manifest.push_str(&format!("{dependency:?} = {requirement:?}\n"));
            }
        }
        fs::write(root.join("mod.toml"), manifest).unwrap();
        fs::write(root.join("encyclopedia.json"), overlay).unwrap();
    }

    fn enable(&self, names: &[&str]) {
        let names = names
            .iter()
            .map(|name| format!("{name:?}"))
            .collect::<Vec<_>>()
            .join(", ");
        fs::write(self.0.join("config.toml"), format!("enabled = [{names}]\n")).unwrap();
    }
}

impl Drop for TempRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn text_overlay(title: &str) -> Vec<u8> {
    format!(
        r#"[{{"id":"original:60001","localized":{{"1033":{{"title":{}}}}}}}]"#,
        serde_json::to_string(title).unwrap()
    )
    .into_bytes()
}

fn image_overlay() -> &'static [u8] {
    br#"[{"id":"original:60001","localized":{"1033":{"image":{"path":"encyclopedia/assets/test.png"}}}}]"#
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

fn runtime_and_lifecycle(root: &TempRoot) -> (ModRuntime, EncyclopediaLifecycle) {
    (
        ModRuntime::discover(root.path()),
        EncyclopediaLifecycle::from_availability(base_availability()),
    )
}

fn changed_poll() -> ModWatchPoll {
    ModWatchPoll {
        changed: true,
        diagnostics: Vec::new(),
    }
}

fn diagnostic_poll(message: impl Into<String>) -> ModWatchPoll {
    let mut message = message.into();
    message.truncate(256);
    ModWatchPoll {
        changed: false,
        diagnostics: vec![message],
    }
}

fn poll_at(
    watcher: &mut EncyclopediaWatcher,
    runtime: &mut ModRuntime,
    lifecycle: &mut EncyclopediaLifecycle,
    milliseconds: u64,
) -> encyclopedia_watcher::EncyclopediaWatchOutcome {
    watcher.poll_and_refresh_at_for_test(runtime, lifecycle, Duration::from_millis(milliseconds))
}

#[test]
fn deterministic_bursts_are_coalesced_to_one_content_only_refresh_in_dependency_order() {
    let root = TempRoot::new("coalesced");
    root.write_mod("A", &[], &text_overlay("A"));
    root.write_mod("B", &[("A", ">=1.0.0")], &text_overlay("B"));
    root.enable(&["A", "B"]);
    let (mut runtime, mut lifecycle) = runtime_and_lifecycle(&root);
    let initial = lifecycle.refresh_resolved(&runtime.enabled_sorted());
    assert!(initial.published_changed);

    fs::write(
        root.mod_path("B").join("encyclopedia.json"),
        text_overlay("B2"),
    )
    .unwrap();
    let mut watcher = EncyclopediaWatcher::from_test_polls(
        root.path(),
        [changed_poll(), ModWatchPoll::default()],
    );

    let first = poll_at(&mut watcher, &mut runtime, &mut lifecycle, 0);
    let second = poll_at(&mut watcher, &mut runtime, &mut lifecycle, 100);

    assert!(first.refresh.is_none());
    assert!(second
        .refresh
        .as_ref()
        .is_some_and(|report| report.published_changed));
    assert_eq!(watcher.refresh_count_for_test(), 1);
    assert_eq!(watcher.last_resolved_order_for_test(), ["A", "B"]);
}

#[test]
fn rename_away_event_does_not_publish_transient_missing_content() {
    let root = TempRoot::new("rename-away-stability");
    root.write_mod("editor", &[], &text_overlay("Accepted"));
    root.enable(&["editor"]);
    let (mut runtime, mut lifecycle) = runtime_and_lifecycle(&root);
    let initial = lifecycle.refresh_resolved(&runtime.enabled_sorted());
    assert!(initial.published_changed);
    let generation = initial.generation.unwrap();
    let retained = lifecycle.retained_bytes_for_test();
    let target = root.mod_path("editor").join("encyclopedia.json");
    let old = root.mod_path("editor").join("encyclopedia.json.old");
    fs::rename(&target, &old).unwrap();
    let mut watcher = EncyclopediaWatcher::from_test_polls(
        root.path(),
        [changed_poll(), changed_poll(), ModWatchPoll::default()],
    );

    let intermediate =
        watcher.poll_and_refresh_at_for_test(&mut runtime, &mut lifecycle, Duration::ZERO);

    assert!(
        intermediate.refresh.is_none(),
        "a rename-away event must wait for the stable boundary before interpreting Missing"
    );
    assert_eq!(watcher.refresh_count_for_test(), 0);
    assert_eq!(lifecycle.retained_bytes_for_test(), retained);

    fs::write(&target, text_overlay("Replacement")).unwrap();
    let replacement_event = watcher.poll_and_refresh_at_for_test(
        &mut runtime,
        &mut lifecycle,
        Duration::from_millis(50),
    );
    assert!(replacement_event.refresh.is_none());

    let stable = watcher.poll_and_refresh_at_for_test(
        &mut runtime,
        &mut lifecycle,
        Duration::from_millis(150),
    );
    let stable = stable
        .refresh
        .expect("the complete replacement must publish at the quiet boundary");
    assert!(stable.published_changed, "report: {stable:?}");
    assert_eq!(stable.generation, Some(generation + 1));
    assert_eq!(watcher.refresh_count_for_test(), 1);
}

#[test]
fn settled_intentional_mod_removal_publishes_at_the_bounded_quiet_deadline() {
    let root = TempRoot::new("settled-removal");
    root.write_mod("removed", &[], &text_overlay("Present"));
    root.enable(&["removed"]);
    let (mut runtime, mut lifecycle) = runtime_and_lifecycle(&root);
    let initial = lifecycle.refresh_resolved(&runtime.enabled_sorted());
    let generation = initial.generation.unwrap();
    let retained = lifecycle.retained_bytes_for_test();
    fs::remove_dir_all(root.mod_path("removed")).unwrap();
    let mut watcher = EncyclopediaWatcher::from_test_polls(
        root.path(),
        [
            changed_poll(),
            ModWatchPoll::default(),
            ModWatchPoll::default(),
        ],
    );

    assert!(poll_at(&mut watcher, &mut runtime, &mut lifecycle, 0)
        .refresh
        .is_none());
    assert!(poll_at(&mut watcher, &mut runtime, &mut lifecycle, 99)
        .refresh
        .is_none());
    assert_eq!(lifecycle.retained_bytes_for_test(), retained);

    let settled = poll_at(&mut watcher, &mut runtime, &mut lifecycle, 100)
        .refresh
        .expect("an intentional removal must settle instead of retaining stale eligibility");
    assert!(runtime.discovered.is_empty());
    assert!(settled.published_changed);
    assert_eq!(settled.generation, Some(generation + 1));
    assert_eq!(watcher.refresh_count_for_test(), 1);
}

#[test]
fn continuous_event_stream_publishes_once_at_the_bounded_maximum_latency() {
    let root = TempRoot::new("bounded-latency");
    root.write_mod("editor", &[], &text_overlay("First"));
    root.enable(&["editor"]);
    let (mut runtime, mut lifecycle) = runtime_and_lifecycle(&root);
    let initial = lifecycle.refresh_resolved(&runtime.enabled_sorted());
    let generation = initial.generation.unwrap();
    fs::write(
        root.mod_path("editor").join("encyclopedia.json"),
        text_overlay("Final"),
    )
    .unwrap();
    let mut watcher =
        EncyclopediaWatcher::from_test_polls(root.path(), (0..7).map(|_| changed_poll()));

    for milliseconds in [0, 90, 180, 270, 360, 450] {
        assert!(
            poll_at(&mut watcher, &mut runtime, &mut lifecycle, milliseconds)
                .refresh
                .is_none()
        );
    }
    let bounded = poll_at(&mut watcher, &mut runtime, &mut lifecycle, 500)
        .refresh
        .expect("continuous events must not postpone a refresh beyond 500 ms");
    assert!(bounded.published_changed);
    assert_eq!(bounded.generation, Some(generation + 1));
    assert_eq!(watcher.refresh_count_for_test(), 1);
}

#[test]
fn automatic_content_reload_never_reapplies_real_world_patches_or_changes_rng() {
    let root = TempRoot::new("world-isolation");
    root.write_mod("world-and-content", &[], &text_overlay("First"));
    root.enable(&["world-and-content"]);
    fs::write(
        root.mod_path("world-and-content").join("gnprtb.json"),
        br#"[{"id":77,"development":7}]"#,
    )
    .unwrap();
    let (mut runtime, mut lifecycle) = runtime_and_lifecycle(&root);
    let ordered = runtime.enabled_sorted();
    let mut world = parameter_world();
    assert!(runtime.apply_ordered(&mut world, &ordered).is_empty());
    assert_eq!(world.gnprtb.value(77, 0), 7);
    assert!(lifecycle.refresh_resolved(&ordered).published_changed);
    let rng = Xoshiro256PlusPlus::seed_from_u64(0xe25);
    let before = simulation_fingerprint(&world, &rng);

    fs::write(
        root.mod_path("world-and-content").join("gnprtb.json"),
        br#"[{"id":77,"development":99}]"#,
    )
    .unwrap();
    fs::write(
        root.mod_path("world-and-content").join("encyclopedia.json"),
        text_overlay("Second"),
    )
    .unwrap();
    let mut watcher = EncyclopediaWatcher::from_test_polls(
        root.path(),
        [changed_poll(), changed_poll(), ModWatchPoll::default()],
    );

    let first = poll_at(&mut watcher, &mut runtime, &mut lifecycle, 0);
    let second = poll_at(&mut watcher, &mut runtime, &mut lifecycle, 50);
    let outcome = poll_at(&mut watcher, &mut runtime, &mut lifecycle, 150);

    assert!(first.refresh.is_none());
    assert!(second.refresh.is_none());
    assert!(outcome
        .refresh
        .as_ref()
        .is_some_and(|report| report.published_changed));
    assert_eq!(world.gnprtb.value(77, 0), 7);
    assert_eq!(simulation_fingerprint(&world, &rng), before);
}

#[test]
fn image_only_edit_refreshes_exact_retained_provider_bytes() {
    let root = TempRoot::new("image-only");
    root.write_mod("image-mod", &[], image_overlay());
    root.enable(&["image-mod"]);
    let image_path = root
        .mod_path("image-mod")
        .join("encyclopedia/assets/test.png");
    fs::create_dir_all(image_path.parent().unwrap()).unwrap();
    fs::write(&image_path, MOD_PNG_RED).unwrap();
    let (mut runtime, mut lifecycle) = runtime_and_lifecycle(&root);
    let initial = lifecycle.refresh_resolved(&runtime.enabled_sorted());
    assert!(initial.published_changed, "initial report: {initial:?}");
    let generation = initial.generation.unwrap();

    let old = root.mod_path("image-mod").join("test.png.old");
    fs::rename(&image_path, &old).unwrap();
    let mut watcher = EncyclopediaWatcher::from_test_polls(
        root.path(),
        [changed_poll(), changed_poll(), ModWatchPoll::default()],
    );
    assert!(poll_at(&mut watcher, &mut runtime, &mut lifecycle, 0)
        .refresh
        .is_none());
    fs::write(&image_path, MOD_PNG_GREEN).unwrap();
    assert!(poll_at(&mut watcher, &mut runtime, &mut lifecycle, 50)
        .refresh
        .is_none());
    let outcome = poll_at(&mut watcher, &mut runtime, &mut lifecycle, 150);
    let refresh = outcome.refresh.expect("image event must refresh content");

    assert!(refresh.published_changed, "refresh report: {refresh:?}");
    assert_eq!(refresh.generation, Some(generation + 1));
    assert_eq!(refresh.changed_image_ids.len(), 1);
    assert!(refresh
        .changed_image_ids
        .iter()
        .all(|image_id| image_id.starts_with("mod:")));
}

#[test]
fn malformed_atomic_save_keeps_last_good_and_a_valid_rename_recovers() {
    let root = TempRoot::new("atomic-save");
    root.write_mod("editor", &[], &text_overlay("First"));
    root.enable(&["editor"]);
    let (mut runtime, mut lifecycle) = runtime_and_lifecycle(&root);
    let initial = lifecycle.refresh_resolved(&runtime.enabled_sorted());
    let generation = initial.generation.unwrap();
    let retained = lifecycle.retained_bytes_for_test();
    let target = root.mod_path("editor").join("encyclopedia.json");

    fs::write(&target, br#"[{"id":"original:60001","localized":"#).unwrap();
    let mut watcher = EncyclopediaWatcher::from_test_polls(
        root.path(),
        [
            changed_poll(),
            ModWatchPoll::default(),
            changed_poll(),
            ModWatchPoll::default(),
        ],
    );
    assert!(poll_at(&mut watcher, &mut runtime, &mut lifecycle, 0)
        .refresh
        .is_none());
    let malformed = poll_at(&mut watcher, &mut runtime, &mut lifecycle, 100)
        .refresh
        .expect("settled malformed bytes are one rejected refresh attempt");
    assert!(!malformed.published_changed);
    assert_eq!(malformed.generation, Some(generation));
    assert!(!malformed.diagnostics.is_empty());
    assert_eq!(
        lifecycle.retained_bytes_for_test(),
        retained,
        "a rejected watcher candidate must not accumulate retained storage"
    );

    let replacement = root.mod_path("editor").join("encyclopedia.json.new");
    fs::write(&replacement, text_overlay("Recovered")).unwrap();
    fs::rename(&replacement, &target).unwrap();
    assert!(poll_at(&mut watcher, &mut runtime, &mut lifecycle, 200)
        .refresh
        .is_none());
    let recovered = poll_at(&mut watcher, &mut runtime, &mut lifecycle, 300);
    let recovered = recovered
        .refresh
        .expect("rename event must refresh content");
    assert!(recovered.published_changed, "report: {recovered:?}");
    assert_eq!(recovered.generation, Some(generation + 1));
    assert!(recovered.diagnostics.is_empty());
}

#[test]
fn malformed_intermediate_replaced_within_the_quiet_window_is_never_published() {
    let root = TempRoot::new("malformed-intermediate");
    root.write_mod("editor", &[], &text_overlay("Accepted"));
    root.enable(&["editor"]);
    let (mut runtime, mut lifecycle) = runtime_and_lifecycle(&root);
    let initial = lifecycle.refresh_resolved(&runtime.enabled_sorted());
    let generation = initial.generation.unwrap();
    let target = root.mod_path("editor").join("encyclopedia.json");
    fs::write(&target, br#"[{"id":"original:60001","localized":"#).unwrap();
    let mut watcher = EncyclopediaWatcher::from_test_polls(
        root.path(),
        [changed_poll(), changed_poll(), ModWatchPoll::default()],
    );

    assert!(poll_at(&mut watcher, &mut runtime, &mut lifecycle, 0)
        .refresh
        .is_none());
    fs::write(&target, text_overlay("Replacement")).unwrap();
    assert!(poll_at(&mut watcher, &mut runtime, &mut lifecycle, 50)
        .refresh
        .is_none());
    let recovered = poll_at(&mut watcher, &mut runtime, &mut lifecycle, 150)
        .refresh
        .expect("the stable valid replacement must be the only parsed candidate");

    assert!(recovered.published_changed, "report: {recovered:?}");
    assert_eq!(recovered.generation, Some(generation + 1));
    assert!(recovered.diagnostics.is_empty());
    assert_eq!(watcher.refresh_count_for_test(), 1);
}

#[test]
fn removing_an_enabled_mod_rebuilds_from_base_instead_of_resurrecting_last_good() {
    let root = TempRoot::new("removed-mod");
    root.write_mod("removed", &[], &text_overlay("Present"));
    root.enable(&["removed"]);
    let (mut runtime, mut lifecycle) = runtime_and_lifecycle(&root);
    let initial = lifecycle.refresh_resolved(&runtime.enabled_sorted());
    let generation = initial.generation.unwrap();

    fs::remove_dir_all(root.mod_path("removed")).unwrap();
    let mut watcher = EncyclopediaWatcher::from_test_polls(
        root.path(),
        [changed_poll(), ModWatchPoll::default()],
    );
    assert!(poll_at(&mut watcher, &mut runtime, &mut lifecycle, 0)
        .refresh
        .is_none());
    let removed = poll_at(&mut watcher, &mut runtime, &mut lifecycle, 100);
    let removed = removed.refresh.expect("remove event must refresh content");

    assert!(runtime.discovered.is_empty());
    assert!(removed.published_changed);
    assert_eq!(removed.generation, Some(generation + 1));
    assert!(removed.diagnostics.is_empty());
}

#[test]
fn disabled_mod_is_ineligible_for_last_good_on_the_next_watcher_refresh() {
    let root = TempRoot::new("disabled-mod");
    root.write_mod("disabled", &[], &text_overlay("Enabled"));
    root.enable(&["disabled"]);
    let (mut runtime, mut lifecycle) = runtime_and_lifecycle(&root);
    let initial = lifecycle.refresh_resolved(&runtime.enabled_sorted());
    let generation = initial.generation.unwrap();

    runtime.toggle_mod("disabled");
    assert!(runtime.enabled_sorted().is_empty());
    let disabled = lifecycle.refresh_content_only(&runtime.enabled_sorted());

    assert!(runtime.enabled_sorted().is_empty());
    assert!(disabled.published_changed);
    assert_eq!(disabled.generation, Some(generation + 1));
    assert!(disabled.diagnostics.is_empty());

    let mut watcher = EncyclopediaWatcher::from_test_polls(
        root.path(),
        [changed_poll(), ModWatchPoll::default()],
    );
    assert!(poll_at(&mut watcher, &mut runtime, &mut lifecycle, 0)
        .refresh
        .is_none());
    let settled = poll_at(&mut watcher, &mut runtime, &mut lifecycle, 100)
        .refresh
        .expect("the config event must settle against current eligibility");
    assert!(!settled.published_changed);
    assert_eq!(settled.generation, Some(generation + 1));
}

#[test]
fn missing_root_keeps_base_available_until_explicit_rearm_after_creation() {
    let parent = TempRoot::new("missing-parent");
    let missing = parent.path().join("mods-not-created-yet");
    let mut watcher = EncyclopediaWatcher::new(&missing);

    assert!(!watcher.is_armed());
    assert!(watcher
        .diagnostic()
        .is_some_and(|message| message.contains("watching mods directory")));
    assert!(!watcher.rearm());

    fs::create_dir(&missing).unwrap();
    assert!(watcher.rearm());
    assert!(watcher.is_armed());
    assert!(watcher.diagnostic().is_none());
}

#[test]
fn mod_manager_root_diagnostic_does_not_require_a_synthetic_mod_row() {
    let mut state = rebellion_render::ModManagerState::default();
    let mods: Vec<rebellion_render::ModInfo> = Vec::new();
    state.root_diagnostic = Some("watch root unavailable; use Reload Mods to rearm".to_owned());

    assert!(mods.is_empty());
    assert!(state
        .root_diagnostic
        .as_deref()
        .is_some_and(|diagnostic| diagnostic.contains("Reload Mods")));
}

#[test]
fn removing_an_armed_root_marks_the_watch_unavailable_without_publishing_empty_discovery() {
    let root = TempRoot::new("armed-root-removal");
    root.write_mod("stable", &[], &text_overlay("Stable"));
    root.enable(&["stable"]);
    let (mut runtime, mut lifecycle) = runtime_and_lifecycle(&root);
    let initial = lifecycle.refresh_resolved(&runtime.enabled_sorted());
    let generation = initial.generation;
    let retained = lifecycle.retained_bytes_for_test();
    let mut watcher = EncyclopediaWatcher::new(root.path());
    assert!(watcher.is_armed());

    fs::remove_dir_all(root.path()).unwrap();
    let unavailable =
        watcher.poll_and_refresh_at_for_test(&mut runtime, &mut lifecycle, Duration::ZERO);

    assert!(!watcher.is_armed(), "a removed watched inode is not armed");
    assert!(unavailable.refresh.is_none());
    assert_eq!(lifecycle.retained_bytes_for_test(), retained);
    assert_eq!(
        lifecycle
            .refresh_content_only(&runtime.enabled_sorted())
            .generation,
        generation
    );
    assert!(unavailable.diagnostics.iter().any(|message| {
        message.contains("watch root unavailable") && message.contains("Reload Mods")
    }));

    root.write_mod("stable", &[], &text_overlay("Stable"));
    root.enable(&["stable"]);
    assert!(watcher.rearm());
    assert!(watcher.is_armed());
    assert!(watcher.diagnostic().is_none());
    fs::write(
        root.mod_path("stable").join("encyclopedia.json"),
        text_overlay("Observed after rearm"),
    )
    .unwrap();

    let event_deadline = Instant::now() + Duration::from_secs(3);
    while !watcher.has_pending_for_test() {
        let outcome =
            watcher.poll_and_refresh_at_for_test(&mut runtime, &mut lifecycle, Duration::ZERO);
        assert!(outcome.refresh.is_none());
        assert!(
            Instant::now() < event_deadline,
            "the rearmed watcher did not observe the next edit"
        );
        std::thread::yield_now();
    }

    let mut recovered = None;
    for milliseconds in [100, 200, 300, 400, 500, 600] {
        let outcome = poll_at(&mut watcher, &mut runtime, &mut lifecycle, milliseconds);
        if outcome.refresh.is_some() {
            recovered = outcome.refresh;
            break;
        }
        std::thread::yield_now();
    }
    let recovered = recovered.expect("the rearmed edit must settle within bounded latency");
    assert!(recovered.published_changed);
    assert_eq!(recovered.generation, generation.map(|value| value + 1));
}

#[test]
fn native_os_watcher_observes_atomic_rename_without_sleep_only_synchronization() {
    let root = TempRoot::new("native-smoke");
    let watcher = ModWatcher::new(root.path()).unwrap();
    let candidate = root.path().join("encyclopedia.json.new");
    let published = root.path().join("encyclopedia.json");
    fs::write(&candidate, b"[]").unwrap();
    fs::rename(candidate, published).unwrap();

    let deadline = Instant::now() + Duration::from_secs(3);
    let observed = loop {
        let poll = watcher.poll();
        if poll.changed {
            break true;
        }
        if Instant::now() >= deadline {
            break false;
        }
        std::thread::yield_now();
    };

    assert!(
        observed,
        "the armed native watcher did not report the atomic rename"
    );
}

#[test]
fn repeated_watch_errors_are_bounded_and_do_not_accumulate_content_candidates() {
    let root = TempRoot::new("bounded-diagnostics");
    root.write_mod("stable", &[], &text_overlay("Stable"));
    root.enable(&["stable"]);
    let (mut runtime, mut lifecycle) = runtime_and_lifecycle(&root);
    let initial = lifecycle.refresh_resolved(&runtime.enabled_sorted());
    let generation = initial.generation;
    let retained = lifecycle.retained_bytes_for_test();
    let mut watcher = EncyclopediaWatcher::from_test_polls(
        root.path(),
        (0..32).map(|_| diagnostic_poll("x".repeat(2048))),
    );

    for _ in 0..32 {
        let outcome = watcher.poll_and_refresh(&mut runtime, &mut lifecycle);
        assert!(outcome.refresh.is_none());
        assert!(outcome
            .diagnostics
            .iter()
            .all(|diagnostic| diagnostic.len() <= 256));
    }

    assert_eq!(watcher.refresh_count_for_test(), 0);
    assert_eq!(lifecycle.retained_bytes_for_test(), retained);
    let stable = lifecycle.refresh_content_only(&runtime.enabled_sorted());
    assert_eq!(stable.generation, generation);
    assert!(!stable.published_changed);
}
