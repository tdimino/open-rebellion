//! Native fixture proof for the platform-neutral replay data manifest.

use std::path::PathBuf;

use rebellion_data::replay::{
    compute_simulation_data_manifest_from_dir, execute_replay, record_replay, ReplayEnvironment,
    ReplayManifest,
};
use rebellion_data::replay_fixture::{
    seed42_commands, seed42_initial_state, validate_seed42_artifact, SEED42_ARTIFACT_BYTES,
    SEED42_ENGINE_VERSION, SEED42_FINAL_FINGERPRINT, SEED42_FINAL_TICK, SEED42_INITIAL_FINGERPRINT,
    SEED42_SEED,
};
use rebellion_data::save::{compute_state_fingerprint, load_slot, save_slot};

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates directory")
        .parent()
        .expect("repository root")
        .join("data/base")
}

#[test]
#[ignore = "requires original data/base DAT files"]
fn original_simulation_data_has_canonical_identity() {
    let manifest =
        compute_simulation_data_manifest_from_dir(&data_dir()).expect("fingerprint original DATs");

    assert_eq!(manifest.inputs.len(), 51);
    assert_eq!(manifest.total_bytes, 50_597);
    assert_eq!(manifest.aggregate_fingerprint.value, "5facb1c7ba0e81ad");
    assert_eq!(manifest.inputs.first().unwrap().name, "ABDCMSTB.DAT");
    assert_eq!(manifest.inputs.last().unwrap().name, "UPRIS2TB.DAT");
}

#[test]
#[ignore = "requires original data/base DAT files"]
fn original_campaign_replay_matches_after_save_reload() {
    let seed = SEED42_SEED;
    let data =
        compute_simulation_data_manifest_from_dir(&data_dir()).expect("fingerprint original DATs");
    let environment = ReplayEnvironment {
        engine_version: SEED42_ENGINE_VERSION,
        seed,
        data: &data,
    };
    let world = rebellion_data::load_game_data_with_options(
        &data_dir(),
        &rebellion_core::world::SeedOptions {
            rng_seed: Some(seed),
            ..rebellion_core::world::SeedOptions::default()
        },
    )
    .expect("load original campaign data");
    let initial = seed42_initial_state(world).expect("build seed-42 initial state");
    let initial_fingerprint = compute_state_fingerprint(&initial).unwrap();
    let recording =
        record_replay(environment, initial.clone(), seed42_commands()).expect("record replay");
    validate_seed42_artifact(&recording.manifest).expect("validate reviewed replay profile");
    let artifact = ReplayManifest::from_json(SEED42_ARTIFACT_BYTES)
        .expect("decode exact committed replay artifact bytes");
    validate_seed42_artifact(&artifact).expect("validate committed replay artifact");
    assert_eq!(artifact, recording.manifest);

    let saves = tempfile::tempdir().expect("temporary save directory");
    save_slot(saves.path(), 0, "Replay Start", &initial, &[]).expect("save initial state");
    let (_, restored) = load_slot(saves.path(), 0).expect("reload initial state");
    let executed = execute_replay(environment, &artifact, restored)
        .expect("execute exact committed replay artifact");
    let executed_fingerprint = compute_state_fingerprint(&executed.final_state).unwrap();
    let recorded_fingerprint = compute_state_fingerprint(&recording.execution.final_state).unwrap();

    assert_eq!(
        executed.observed_checkpoints,
        recording.manifest.checkpoints
    );
    assert_eq!(executed_fingerprint, recorded_fingerprint);
    assert_eq!(executed.final_state.clock.tick, SEED42_FINAL_TICK);
    assert_eq!(initial_fingerprint.to_string(), SEED42_INITIAL_FINGERPRINT);
    let observed_fingerprints: Vec<_> = recording
        .manifest
        .checkpoints
        .iter()
        .map(|checkpoint| {
            (
                checkpoint.command_count,
                checkpoint.tick,
                checkpoint.state_fingerprint.clone(),
            )
        })
        .collect();
    assert_eq!(
        observed_fingerprints,
        vec![
            (1, 0, "v1:ecf7c467330431a0".into()),
            (2, 0, "v1:673f16d62626d3fa".into()),
            (3, 5, "v1:5e532cc57cfd2d60".into()),
            (4, 10, "v1:ecdbabda150f5f6d".into()),
            (5, 15, "v1:8befe5d04e2f8ad7".into()),
            (6, 20, "v1:e567ee451b540e16".into()),
            (7, 25, "v1:dccc8df2f32031b2".into()),
            (8, 25, "v1:0e114a59d20b10b0".into()),
            (9, 25, "v1:0e114a59d20b10b0".into()),
        ]
    );
    assert_eq!(executed_fingerprint.to_string(), SEED42_FINAL_FINGERPRINT);
}
