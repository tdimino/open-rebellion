//! Query-gated browser runner for the native/WASM replay-equivalence fixture.

use std::collections::HashMap;
#[cfg(target_arch = "wasm32")]
use std::path::Path;

#[cfg(target_arch = "wasm32")]
use rebellion_core::world::SeedOptions;
use rebellion_data::replay::compute_simulation_data_manifest;
#[cfg(target_arch = "wasm32")]
use rebellion_data::replay_fixture::{
    run_seed42_gate, ReplayGateReport, SEED42_ARTIFACT_BYTES, SEED42_SEED,
};

use crate::encyclopedia_session::EncyclopediaAvailability;

#[derive(Debug)]
struct PreparedReplayRuntimePack {
    data: rebellion_data::replay::SimulationDataManifest,
    game_files: HashMap<String, Vec<u8>>,
    string_table: HashMap<u16, String>,
    encyclopedia: EncyclopediaAvailability,
}

fn prepare_replay_runtime_pack(
    mut pack: crate::runtime_pack::RuntimePack,
) -> Result<PreparedReplayRuntimePack, (&'static str, String)> {
    let encyclopedia =
        crate::encyclopedia_runtime::prepare_packed_encyclopedia(&mut pack.game_files)
            .map_err(|error| ("encyclopedia", error))?;
    let data = compute_simulation_data_manifest(
        pack.game_files
            .iter()
            .filter(|(name, _)| name.to_ascii_uppercase().ends_with(".DAT")),
    )
    .map_err(|error| ("data_manifest", format!("{error:#}")))?;
    let string_table = match pack.game_files.remove("textstra.json") {
        Some(bytes) => serde_json::from_slice(&bytes).map_err(|error| {
            (
                "string_table",
                format!("textstra.json is malformed: {error}"),
            )
        })?,
        None => {
            return Err((
                "string_table",
                "runtime pack is missing textstra.json".to_owned(),
            ));
        }
    };
    Ok(PreparedReplayRuntimePack {
        data,
        game_files: pack.game_files,
        string_table,
        encyclopedia,
    })
}

#[cfg(target_arch = "wasm32")]
const REPLAY_MODE_ABSENT: u32 = 0;
#[cfg(target_arch = "wasm32")]
const REPLAY_MODE_SEED42_V1: u32 = 1;

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn open_rebellion_replay_mode() -> u32;
    fn open_rebellion_replay_emit(ptr: *const u8, len: usize);
}

#[cfg(target_arch = "wasm32")]
pub fn requested() -> bool {
    unsafe { open_rebellion_replay_mode() != REPLAY_MODE_ABSENT }
}

#[cfg(target_arch = "wasm32")]
fn emit(report: &ReplayGateReport) {
    let json = serde_json::to_vec(report).expect("serialize browser replay-gate report");
    unsafe { open_rebellion_replay_emit(json.as_ptr(), json.len()) };
}

#[cfg(target_arch = "wasm32")]
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

#[cfg(target_arch = "wasm32")]
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
    let pack = match crate::runtime_pack::parse_runtime_pack(&bytes) {
        Ok(pack) => pack,
        Err(error) => {
            return ReplayGateReport::failure("wasm32", "runtime_pack_decode", error.to_string())
        }
    };
    let prepared = match prepare_replay_runtime_pack(pack) {
        Ok(prepared) => prepared,
        Err((phase, error)) => return ReplayGateReport::failure("wasm32", phase, error),
    };
    match &prepared.encyclopedia {
        EncyclopediaAvailability::Ready(session) => {
            macroquad::logging::info!(
                "[encyclopedia] replay validated packed session generation={}",
                session.generation()
            );
        }
        EncyclopediaAvailability::Unavailable(diagnostic) => {
            macroquad::logging::warn!("[encyclopedia] replay pack unavailable: {}", diagnostic);
        }
    }
    rebellion_data::set_string_table(prepared.string_table);
    rebellion_data::set_file_cache(prepared.game_files);

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
    run_seed42_gate("wasm32", SEED42_ARTIFACT_BYTES, &prepared.data, world)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;
    use rebellion_core::ai::{AIState, AiFaction};
    use rebellion_core::betrayal::BetrayalState;
    use rebellion_core::blockade::BlockadeState;
    use rebellion_core::dat::{ExplorationStatus, Faction, SectorGroup};
    use rebellion_core::death_star::DeathStarState;
    use rebellion_core::economy::EconomyState;
    use rebellion_core::events::EventState;
    use rebellion_core::fog::FogState;
    use rebellion_core::jedi::JediState;
    use rebellion_core::manufacturing::ManufacturingState;
    use rebellion_core::missions::MissionState;
    use rebellion_core::movement::MovementState;
    use rebellion_core::repair::RepairState;
    use rebellion_core::research::ResearchState;
    use rebellion_core::tick::{GameClock, GameSpeed};
    use rebellion_core::tuning::GameConfig;
    use rebellion_core::uprising::UprisingState;
    use rebellion_core::victory::VictoryState;
    use rebellion_core::world::{CampaignConfig, ControlKind, GameWorld, Sector, System};
    use rebellion_data::replay::{record_replay, ReplayActor, ReplayCommand, ReplayEnvironment};
    use rebellion_data::save::SaveState;

    use super::*;

    const VALID_CATALOG: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/catalog.json");
    const VALID_MANIFEST: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/manifest.json");
    const VALID_DAT: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/sources/SYNTHETIC.DAT"
    );

    fn replay_pack(include_namespace: bool) -> crate::runtime_pack::RuntimePack {
        let mut pack = crate::runtime_pack::RuntimePack::default();
        pack.game_files = HashMap::from([
            ("SYNTHETIC.DAT".to_owned(), VALID_DAT.to_vec()),
            ("textstra.json".to_owned(), b"{}".to_vec()),
        ]);
        if include_namespace {
            pack.game_files.insert(
                "encyclopedia/catalog.json".to_owned(),
                VALID_CATALOG.to_vec(),
            );
            pack.game_files.insert(
                "encyclopedia/manifest.json".to_owned(),
                VALID_MANIFEST.to_vec(),
            );
            for (name, bytes) in [
                (
                    "EDATA.001",
                    include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.001").as_slice(),
                ),
                (
                    "EDATA.002",
                    include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.002").as_slice(),
                ),
                (
                    "EDATA.003",
                    include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.003").as_slice(),
                ),
            ] {
                pack.game_files
                    .insert(format!("encyclopedia/assets/{name}"), bytes.to_vec());
            }
        }
        pack
    }

    fn replay_state(seed: u64) -> SaveState {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: rebellion_core::ids::DatId::new(1),
            name: "Synthetic Sector".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: Vec::new(),
        });
        let alliance_hq = world.systems.insert(System {
            dat_id: rebellion_core::ids::DatId::new(2),
            name: "Alliance HQ".into(),
            sector,
            x: 0,
            y: 0,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.75,
            popularity_empire: 0.25,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            fleets: Vec::new(),
            ground_units: Vec::new(),
            special_forces: Vec::new(),
            defense_facilities: Vec::new(),
            manufacturing_facilities: Vec::new(),
            production_facilities: Vec::new(),
            is_headquarters: true,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Alliance),
            espionage_rating: 0.0,
        });
        let empire_hq = world.systems.insert(System {
            dat_id: rebellion_core::ids::DatId::new(3),
            name: "Empire HQ".into(),
            sector,
            x: 10,
            y: 10,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.25,
            popularity_empire: 0.75,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            fleets: Vec::new(),
            ground_units: Vec::new(),
            special_forces: Vec::new(),
            defense_facilities: Vec::new(),
            manufacturing_facilities: Vec::new(),
            production_facilities: Vec::new(),
            is_headquarters: true,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Empire),
            espionage_rating: 0.0,
        });
        world.sectors[sector].systems = vec![alliance_hq, empire_hq];
        SaveState {
            world,
            clock: GameClock::default(),
            manufacturing: ManufacturingState::new(),
            missions: MissionState::new(),
            events: EventState::new(),
            ai: AIState::new(AiFaction::Empire),
            movement: MovementState::new(),
            fog_alliance: FogState::new(Faction::Alliance),
            fog_empire: FogState::new(Faction::Empire),
            player_is_alliance: true,
            blockade: BlockadeState::new(),
            uprising: UprisingState::new(),
            death_star: DeathStarState::default(),
            research: ResearchState::new(),
            jedi: JediState::new(),
            victory: VictoryState::new(alliance_hq, empire_hq),
            betrayal: BetrayalState::new(),
            economy: EconomyState::default(),
            sim_rng: Xoshiro256PlusPlus::seed_from_u64(seed),
            ai2: None,
            repair: RepairState::default(),
            combat_cooldowns: HashMap::new(),
            game_config: GameConfig::default(),
            campaign_config: CampaignConfig::default(),
            troop_transport: rebellion_core::troop_transport::TroopTransportState::default(),
            deliveries: rebellion_core::delivery::DeliveryState::default(),
        }
    }

    #[test]
    fn replay_with_or_without_valid_encyclopedia_has_identical_manifest_and_checkpoints() {
        let old_pack = prepare_replay_runtime_pack(replay_pack(false)).unwrap();
        let presented_pack = prepare_replay_runtime_pack(replay_pack(true)).unwrap();
        assert_eq!(old_pack.data, presented_pack.data);
        assert_eq!(old_pack.game_files, presented_pack.game_files);
        assert_eq!(old_pack.string_table, presented_pack.string_table);
        assert!(matches!(
            old_pack.encyclopedia,
            EncyclopediaAvailability::Unavailable(_)
        ));
        assert!(matches!(
            presented_pack.encyclopedia,
            EncyclopediaAvailability::Ready(_)
        ));
        assert!(presented_pack
            .game_files
            .keys()
            .all(|key| !key.starts_with("encyclopedia/")));

        let commands = [
            (
                ReplayActor::Alliance,
                ReplayCommand::SetSpeed {
                    speed: GameSpeed::Fast,
                },
            ),
            (
                ReplayActor::Engine,
                ReplayCommand::AdvanceTicks { count: 2 },
            ),
        ];
        let record = |data| {
            record_replay(
                ReplayEnvironment {
                    engine_version: "e16-synthetic",
                    seed: 42,
                    data,
                },
                replay_state(42),
                commands.clone(),
            )
            .unwrap()
        };
        let old_recording = record(&old_pack.data);
        let presented_recording = record(&presented_pack.data);
        assert_eq!(
            old_recording.manifest.checkpoints,
            presented_recording.manifest.checkpoints
        );
        assert_eq!(
            old_recording.execution.final_state.clock.tick,
            presented_recording.execution.final_state.clock.tick
        );
    }

    #[test]
    fn replay_rejects_partial_encyclopedia_before_cache_inputs_are_returned() {
        let mut pack = replay_pack(true);
        pack.game_files.remove("encyclopedia/catalog.json");

        let (phase, error) = prepare_replay_runtime_pack(pack).unwrap_err();

        assert_eq!(phase, "encyclopedia");
        assert!(error.contains("invalid_encyclopedia_bundle"), "{error}");
    }
}
