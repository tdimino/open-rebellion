//! Shared deterministic fixture for native and browser replay equivalence.

use anyhow::{bail, Context};
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use rebellion_core::ai::{AIState, AiFaction};
use rebellion_core::betrayal::BetrayalState;
use rebellion_core::blockade::BlockadeState;
use rebellion_core::dat::Faction;
use rebellion_core::death_star::DeathStarState;
use rebellion_core::economy::EconomyState;
use rebellion_core::events::EventState;
use rebellion_core::fog::{FogState, FogSystem};
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
use rebellion_core::world::{CampaignConfig, GameWorld, SeedOptions, VictoryConditions};

use serde::{Deserialize, Serialize};

use crate::replay::{
    execute_replay_observed, ReplayActor, ReplayCheckpoint, ReplayCommand, ReplayEnvironment,
    ReplayManifest, SimulationDataManifest,
};
use crate::save::{compute_state_fingerprint, SaveState};

pub const SEED42_FIXTURE_ID: &str = "seed42-v1";
pub const SEED42_ARTIFACT_BYTES: &[u8] = include_bytes!("../tests/fixtures/replay_seed42_v1.json");
pub const SEED42_ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SEED42_SEED: u64 = 42;
pub const SEED42_INITIAL_FINGERPRINT: &str = "v1:0efe11fdf7aa5859";
pub const SEED42_FINAL_FINGERPRINT: &str = "v1:5871d04619e30abd";
pub const SEED42_FINAL_TICK: u64 = 25;
pub const SEED42_DATA_INPUTS: usize = 51;
pub const SEED42_DATA_BYTES: u64 = 50_597;
pub const SEED42_DATA_FINGERPRINT: &str = "5facb1c7ba0e81ad";

pub const SEED42_CHECKPOINTS: &[(u64, u64, &str)] = &[
    (1, 0, "v1:19cb6b69fbc10454"),
    (2, 0, "v1:8fd0b33fcc8fdbfe"),
    (3, 5, "v1:54292fb35ba017f4"),
    (4, 10, "v1:dbb597bbf32e7fb4"),
    (5, 15, "v1:9a6331d251ee99cf"),
    (6, 20, "v1:beddfc6ffdce9b16"),
    (7, 25, "v1:2221aca5685d7fa3"),
    (8, 25, "v1:5871d04619e30abd"),
    (9, 25, "v1:5871d04619e30abd"),
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayGateReport {
    pub schema_version: u16,
    pub fixture_id: String,
    pub platform: String,
    pub status: String,
    pub artifact_text: String,
    pub engine_version: String,
    pub seed: u64,
    pub data_input_count: usize,
    pub data_bytes: u64,
    pub data_fingerprint: String,
    pub initial_fingerprint: Option<String>,
    pub observed_checkpoints: Vec<ReplayCheckpoint>,
    pub final_tick: Option<u64>,
    pub final_fingerprint: Option<String>,
    pub failure_phase: Option<String>,
    pub error: Option<String>,
}

impl ReplayGateReport {
    pub fn failure(platform: &str, phase: &str, error: impl Into<String>) -> Self {
        Self {
            schema_version: 1,
            fixture_id: SEED42_FIXTURE_ID.to_string(),
            platform: platform.to_string(),
            status: "failed".to_string(),
            artifact_text: String::from_utf8_lossy(SEED42_ARTIFACT_BYTES).into_owned(),
            engine_version: SEED42_ENGINE_VERSION.to_string(),
            seed: SEED42_SEED,
            data_input_count: 0,
            data_bytes: 0,
            data_fingerprint: String::new(),
            initial_fingerprint: None,
            observed_checkpoints: Vec::new(),
            final_tick: None,
            final_fingerprint: None,
            failure_phase: Some(phase.to_string()),
            error: Some(error.into()),
        }
    }

    pub fn passed(&self) -> bool {
        self.status == "passed"
    }
}

/// The reviewed command profile shared by the recorder and strict fixture validator.
pub fn seed42_commands() -> Vec<(ReplayActor, ReplayCommand)> {
    let mut commands = vec![
        (
            ReplayActor::Alliance,
            ReplayCommand::SetSpeed {
                speed: GameSpeed::Faster,
            },
        ),
        (ReplayActor::Engine, ReplayCommand::ToggleDualAi),
    ];
    commands.extend((0..5).map(|_| {
        (
            ReplayActor::Engine,
            ReplayCommand::AdvanceTicks { count: 5 },
        )
    }));
    commands.extend([
        (ReplayActor::Engine, ReplayCommand::RevealAllSystems),
        (ReplayActor::Engine, ReplayCommand::ForceVictoryCheck),
    ]);
    commands
}

/// Build the seed-42 campaign state independently from the replay artifact.
pub fn seed42_initial_state(world: GameWorld) -> anyhow::Result<SaveState> {
    if world.systems.len() != 200 {
        bail!(
            "seed-42 fixture requires 200 systems, loaded {}",
            world.systems.len()
        );
    }
    let alliance_hq = world
        .systems
        .iter()
        .find(|(_, system)| {
            system.is_headquarters && system.control.is_controlled_by(Faction::Alliance)
        })
        .map(|(key, _)| key)
        .context("seed-42 fixture is missing the Alliance headquarters")?;
    let empire_hq = world
        .systems
        .iter()
        .find(|(_, system)| {
            system.is_headquarters && system.control.is_controlled_by(Faction::Empire)
        })
        .map(|(key, _)| key)
        .context("seed-42 fixture is missing the Empire headquarters")?;

    let mut fog_alliance = FogState::new(Faction::Alliance);
    let mut fog_empire = FogState::new(Faction::Empire);
    FogSystem::seed(&mut fog_alliance, &world);
    FogSystem::seed(&mut fog_empire, &world);
    let mut events = EventState::new();
    rebellion_core::story_events::define_story_events(&mut events, &world);
    let options = SeedOptions {
        rng_seed: Some(SEED42_SEED),
        ..SeedOptions::default()
    };

    Ok(SaveState {
        world,
        clock: GameClock::new(),
        manufacturing: ManufacturingState::new(),
        missions: MissionState::new(),
        events,
        ai: AIState::new(AiFaction::Empire),
        movement: MovementState::new(),
        fog_alliance,
        fog_empire,
        player_is_alliance: true,
        blockade: BlockadeState::new(),
        uprising: UprisingState::new(),
        death_star: DeathStarState::default(),
        research: ResearchState::new(),
        jedi: JediState::new(),
        victory: VictoryState::new(alliance_hq, empire_hq),
        betrayal: BetrayalState::new(),
        economy: EconomyState::default(),
        sim_rng: Xoshiro256PlusPlus::seed_from_u64(SEED42_SEED),
        ai2: None,
        repair: RepairState::default(),
        combat_cooldowns: std::collections::HashMap::new(),
        game_config: GameConfig::default(),
        campaign_config: CampaignConfig::from_seed_options(options, VictoryConditions::Standard),
    })
}

/// Reject a syntactically valid replay that is not the complete reviewed fixture.
pub fn validate_seed42_artifact(manifest: &ReplayManifest) -> anyhow::Result<()> {
    manifest.validate()?;
    if manifest.engine_version != SEED42_ENGINE_VERSION {
        bail!(
            "seed-42 fixture engine mismatch: expected {:?}, found {:?}",
            SEED42_ENGINE_VERSION,
            manifest.engine_version
        );
    }
    if manifest.seed != SEED42_SEED {
        bail!(
            "seed-42 fixture seed mismatch: expected {}, found {}",
            SEED42_SEED,
            manifest.seed
        );
    }
    if manifest.data.inputs.len() != SEED42_DATA_INPUTS
        || manifest.data.total_bytes != SEED42_DATA_BYTES
        || manifest.data.aggregate_fingerprint.value != SEED42_DATA_FINGERPRINT
    {
        bail!("seed-42 fixture data identity does not match the reviewed original-data set");
    }
    if manifest.initial_state_fingerprint != SEED42_INITIAL_FINGERPRINT {
        bail!("seed-42 fixture initial fingerprint does not match the reviewed golden");
    }

    let expected_commands = seed42_commands();
    if manifest.commands.len() != expected_commands.len() {
        bail!(
            "seed-42 fixture requires {} commands, found {}",
            expected_commands.len(),
            manifest.commands.len()
        );
    }
    for (record, (actor, command)) in manifest.commands.iter().zip(&expected_commands) {
        if record.actor != *actor || record.command != *command {
            bail!(
                "seed-42 fixture command {} does not match the reviewed profile",
                record.sequence
            );
        }
    }

    if manifest.checkpoints.len() != SEED42_CHECKPOINTS.len() {
        bail!(
            "seed-42 fixture requires {} checkpoints, found {}",
            SEED42_CHECKPOINTS.len(),
            manifest.checkpoints.len()
        );
    }
    for (checkpoint, (command_count, tick, fingerprint)) in
        manifest.checkpoints.iter().zip(SEED42_CHECKPOINTS)
    {
        if checkpoint.command_count != *command_count
            || checkpoint.tick != *tick
            || checkpoint.state_fingerprint != *fingerprint
        {
            bail!(
                "seed-42 fixture checkpoint after {} commands does not match the reviewed golden",
                command_count
            );
        }
    }
    Ok(())
}

/// Execute the exact fixture bytes and retain structured diagnostics on failure.
pub fn run_seed42_gate(
    platform: &str,
    artifact_bytes: &[u8],
    data: SimulationDataManifest,
    world: GameWorld,
) -> ReplayGateReport {
    let artifact_text = String::from_utf8_lossy(artifact_bytes).into_owned();
    let mut report = ReplayGateReport {
        schema_version: 1,
        fixture_id: SEED42_FIXTURE_ID.to_string(),
        platform: platform.to_string(),
        status: "failed".to_string(),
        artifact_text,
        engine_version: SEED42_ENGINE_VERSION.to_string(),
        seed: SEED42_SEED,
        data_input_count: data.inputs.len(),
        data_bytes: data.total_bytes,
        data_fingerprint: data.aggregate_fingerprint.value.clone(),
        initial_fingerprint: None,
        observed_checkpoints: Vec::new(),
        final_tick: None,
        final_fingerprint: None,
        failure_phase: Some("artifact_utf8".to_string()),
        error: None,
    };

    let outcome = (|| -> anyhow::Result<()> {
        let artifact_text =
            std::str::from_utf8(artifact_bytes).context("replay artifact is not valid UTF-8")?;
        if artifact_text.as_bytes() != artifact_bytes {
            bail!("replay artifact UTF-8 did not round-trip to its exact bytes");
        }

        report.failure_phase = Some("artifact_decode".to_string());
        let manifest = ReplayManifest::from_json(artifact_bytes)?;
        report.failure_phase = Some("fixture_validation".to_string());
        validate_seed42_artifact(&manifest)?;

        report.failure_phase = Some("initial_state".to_string());
        let initial = seed42_initial_state(world)?;
        let initial_fingerprint = compute_state_fingerprint(&initial)?.to_string();
        report.initial_fingerprint = Some(initial_fingerprint.clone());
        if initial_fingerprint != SEED42_INITIAL_FINGERPRINT {
            bail!(
                "seed-42 initial fingerprint mismatch: expected {}, found {}",
                SEED42_INITIAL_FINGERPRINT,
                initial_fingerprint
            );
        }

        report.failure_phase = Some("execution".to_string());
        let environment = ReplayEnvironment {
            engine_version: SEED42_ENGINE_VERSION,
            seed: SEED42_SEED,
            data: &data,
        };
        let execution = execute_replay_observed(environment, &manifest, initial, |checkpoint| {
            report.observed_checkpoints.push(checkpoint.clone());
        })?;
        if execution.observed_checkpoints != report.observed_checkpoints {
            bail!("replay observer and executor checkpoint streams disagree");
        }

        report.failure_phase = Some("final_state".to_string());
        let final_fingerprint = compute_state_fingerprint(&execution.final_state)?.to_string();
        let final_tick = execution.final_state.clock.tick;
        if final_tick != SEED42_FINAL_TICK || final_fingerprint != SEED42_FINAL_FINGERPRINT {
            bail!(
                "seed-42 final state mismatch: expected tick {} {}, found tick {} {}",
                SEED42_FINAL_TICK,
                SEED42_FINAL_FINGERPRINT,
                final_tick,
                final_fingerprint
            );
        }
        report.final_tick = Some(final_tick);
        report.final_fingerprint = Some(final_fingerprint);
        Ok(())
    })();

    match outcome {
        Ok(()) => {
            report.status = "passed".to_string();
            report.failure_phase = None;
        }
        Err(error) => report.error = Some(format!("{error:#}")),
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn committed_artifact_is_complete_and_reviewed() {
        let manifest = ReplayManifest::from_json(SEED42_ARTIFACT_BYTES).unwrap();
        validate_seed42_artifact(&manifest).unwrap();
    }

    #[test]
    fn fixture_validation_rejects_incomplete_or_changed_profiles() {
        let manifest = ReplayManifest::from_json(SEED42_ARTIFACT_BYTES).unwrap();

        let mut missing_checkpoint = manifest.clone();
        missing_checkpoint.checkpoints.pop();
        assert!(validate_seed42_artifact(&missing_checkpoint)
            .unwrap_err()
            .to_string()
            .contains("requires 9 checkpoints"));

        let mut wrong_seed = manifest.clone();
        wrong_seed.seed += 1;
        assert!(validate_seed42_artifact(&wrong_seed)
            .unwrap_err()
            .to_string()
            .contains("seed mismatch"));

        let mut changed_command = manifest;
        changed_command.commands[2].command = ReplayCommand::AdvanceTicks { count: 4 };
        assert!(validate_seed42_artifact(&changed_command)
            .unwrap_err()
            .to_string()
            .contains("reviewed profile"));
    }
}
