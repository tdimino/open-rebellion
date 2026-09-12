use std::path::PathBuf;

use rebellion_core::world::SeedOptions;
use rebellion_data::replay::compute_simulation_data_manifest_from_dir;
use rebellion_data::replay_fixture::{run_seed42_gate, SEED42_ARTIFACT_BYTES, SEED42_SEED};

fn main() {
    let data_dir = std::env::args_os()
        .nth(1)
        .map_or_else(|| PathBuf::from("data/base"), PathBuf::from);
    let data = match compute_simulation_data_manifest_from_dir(&data_dir) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("failed to fingerprint {}: {error:#}", data_dir.display());
            std::process::exit(1);
        }
    };
    let options = SeedOptions {
        rng_seed: Some(SEED42_SEED),
        ..SeedOptions::default()
    };
    let world = match rebellion_data::load_game_data_with_options(&data_dir, &options) {
        Ok(world) => world,
        Err(error) => {
            eprintln!("failed to load {}: {error:#}", data_dir.display());
            std::process::exit(1);
        }
    };
    let report = run_seed42_gate("native", SEED42_ARTIFACT_BYTES, &data, world);
    println!(
        "{}",
        serde_json::to_string(&report).expect("serialize native replay-gate report")
    );
    if !report.passed() {
        std::process::exit(1);
    }
}
