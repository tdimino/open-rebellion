//! Query-gated browser runner for the native/WASM replay-equivalence fixture.

use std::collections::HashMap;
use std::path::Path;

use rebellion_core::world::SeedOptions;
use rebellion_data::replay::compute_simulation_data_manifest;
use rebellion_data::replay_fixture::{
    run_seed42_gate, ReplayGateReport, SEED42_ARTIFACT_BYTES, SEED42_SEED,
};

const REPLAY_MODE_ABSENT: u32 = 0;
const REPLAY_MODE_SEED42_V1: u32 = 1;

extern "C" {
    fn open_rebellion_replay_mode() -> u32;
    fn open_rebellion_replay_emit(ptr: *const u8, len: usize);
}

pub fn requested() -> bool {
    unsafe { open_rebellion_replay_mode() != REPLAY_MODE_ABSENT }
}

fn emit(report: &ReplayGateReport) {
    let json = serde_json::to_vec(report).expect("serialize browser replay-gate report");
    unsafe { open_rebellion_replay_emit(json.as_ptr(), json.len()) };
}

pub async fn run(data_path: &Path) {
    let mode = unsafe { open_rebellion_replay_mode() };
    let report = if mode != REPLAY_MODE_SEED42_V1 {
        ReplayGateReport::failure(
            "wasm32",
            "query",
            format!("unsupported replay-check query mode {mode}"),
        )
    } else {
        run_seed42_from_runtime_pack(data_path).await
    };
    let passed = report.passed();
    let detail = report
        .error
        .as_deref()
        .unwrap_or("all nine checkpoints matched");
    emit(&report);

    loop {
        macroquad::prelude::clear_background(macroquad::prelude::Color::new(0.02, 0.02, 0.06, 1.0));
        macroquad::prelude::draw_text(
            if passed {
                "Replay equivalence: PASS"
            } else {
                "Replay equivalence: FAIL"
            },
            32.0,
            58.0,
            30.0,
            if passed {
                macroquad::prelude::GREEN
            } else {
                macroquad::prelude::RED
            },
        );
        macroquad::prelude::draw_text(detail, 32.0, 92.0, 18.0, macroquad::prelude::LIGHTGRAY);
        macroquad::prelude::next_frame().await;
    }
}

async fn run_seed42_from_runtime_pack(data_path: &Path) -> ReplayGateReport {
    let bytes = match macroquad::file::load_file("data/runtime.orpk").await {
        Ok(bytes) => bytes,
        Err(error) => {
            return ReplayGateReport::failure(
                "wasm32",
                "runtime_pack_fetch",
                format!("data/runtime.orpk failed to load: {error:?}"),
            )
        }
    };
    let mut pack = match crate::runtime_pack::parse_runtime_pack(&bytes) {
        Ok(pack) => pack,
        Err(error) => {
            return ReplayGateReport::failure("wasm32", "runtime_pack_decode", error.to_string())
        }
    };
    let data = match compute_simulation_data_manifest(
        pack.game_files
            .iter()
            .filter(|(name, _)| name.to_ascii_uppercase().ends_with(".DAT")),
    ) {
        Ok(data) => data,
        Err(error) => {
            return ReplayGateReport::failure("wasm32", "data_manifest", format!("{error:#}"))
        }
    };
    let strings: HashMap<u16, String> = match pack.game_files.remove("textstra.json") {
        Some(bytes) => match serde_json::from_slice(&bytes) {
            Ok(strings) => strings,
            Err(error) => {
                return ReplayGateReport::failure(
                    "wasm32",
                    "string_table",
                    format!("textstra.json is malformed: {error}"),
                )
            }
        },
        None => {
            return ReplayGateReport::failure(
                "wasm32",
                "string_table",
                "runtime pack is missing textstra.json",
            )
        }
    };
    rebellion_data::set_string_table(strings);
    rebellion_data::set_file_cache(pack.game_files);

    let options = SeedOptions {
        rng_seed: Some(SEED42_SEED),
        ..SeedOptions::default()
    };
    let world = match rebellion_data::load_game_data_with_options(data_path, &options) {
        Ok(world) => world,
        Err(error) => {
            return ReplayGateReport::failure("wasm32", "game_data", format!("{error:#}"))
        }
    };
    run_seed42_gate("wasm32", SEED42_ARTIFACT_BYTES, &data, world)
}
