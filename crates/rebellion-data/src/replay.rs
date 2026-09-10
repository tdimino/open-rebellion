//! Versioned, platform-neutral replay artifacts.
//!
//! A replay identifies the exact simulation `.DAT` inputs and tuning
//! configuration, then records state-changing commands in one total order.
//! Native and WASM runners can therefore reject incompatible inputs before
//! comparing versioned save-state fingerprints at declared checkpoints.

use std::collections::HashSet;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

use anyhow::{bail, Context};
use rand::Rng;
use rebellion_core::ai::{AIState, AiFaction};
use rebellion_core::fog::FogState;
use rebellion_core::tick::{GameSpeed, TickEvent};
use rebellion_core::tuning::GameConfig;
use rebellion_core::victory::VictorySystem;
use rebellion_core::world::GameWorld;
use serde::{Deserialize, Serialize};

use crate::save::{compute_state_fingerprint, SaveState, StateFingerprint};
use crate::simulation::{run_simulation_tick, SimulationStates};

/// Current JSON replay envelope version.
pub const REPLAY_FORMAT_VERSION: u16 = 1;
/// Current canonical simulation-input manifest version.
pub const DATA_MANIFEST_VERSION: u16 = 1;
/// Current tuning-configuration fingerprint version.
pub const CONFIG_FINGERPRINT_VERSION: u16 = 1;
/// Random values reserved for every simulation tick in replay format v1.
pub const REPLAY_ROLLS_PER_TICK: u16 = 1024;
/// Defensive limit for one `advance_ticks` command.
pub const MAX_ADVANCE_TICKS_PER_COMMAND: u64 = 1_000_000;

const FINGERPRINT_ALGORITHM: &str = "fnv1a64";
const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

/// JSON-safe representation of a versioned 64-bit fingerprint.
///
/// The hexadecimal string avoids the precision loss that JavaScript numbers
/// can introduce above `2^53`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContentFingerprint {
    pub algorithm: String,
    pub version: u16,
    pub value: String,
}

impl ContentFingerprint {
    fn fnv1a(version: u16, value: u64) -> Self {
        Self {
            algorithm: FINGERPRINT_ALGORITHM.to_string(),
            version,
            value: format!("{value:016x}"),
        }
    }

    fn validate(&self, expected_version: u16, label: &str) -> anyhow::Result<()> {
        if self.algorithm != FINGERPRINT_ALGORITHM {
            bail!(
                "unsupported {label} fingerprint algorithm {:?}",
                self.algorithm
            );
        }
        if self.version != expected_version {
            bail!(
                "unsupported {label} fingerprint version {} (expected {})",
                self.version,
                expected_version
            );
        }
        if self.value.len() != 16
            || !self
                .value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            bail!("invalid {label} fingerprint value {:?}", self.value);
        }
        Ok(())
    }
}

/// Fingerprint and size of one canonical simulation input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataInputFingerprint {
    pub name: String,
    pub byte_length: u64,
    pub fingerprint: ContentFingerprint,
}

/// Canonical identity of every `.DAT` byte consumed by the simulation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationDataManifest {
    pub format_version: u16,
    pub total_bytes: u64,
    pub aggregate_fingerprint: ContentFingerprint,
    pub inputs: Vec<DataInputFingerprint>,
}

impl SimulationDataManifest {
    /// Reject unsupported, malformed, incomplete, or ambiguously ordered data.
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.format_version != DATA_MANIFEST_VERSION {
            bail!(
                "unsupported data-manifest version {} (expected {})",
                self.format_version,
                DATA_MANIFEST_VERSION
            );
        }
        if self.inputs.is_empty() {
            bail!("simulation data manifest contains no inputs");
        }
        self.aggregate_fingerprint
            .validate(DATA_MANIFEST_VERSION, "aggregate data")?;

        let mut previous_name: Option<&str> = None;
        let mut observed_total = 0_u64;
        for input in &self.inputs {
            validate_data_name(&input.name)?;
            if previous_name.is_some_and(|previous| previous >= input.name.as_str()) {
                bail!(
                    "simulation data inputs are not in unique canonical order at {}",
                    input.name
                );
            }
            previous_name = Some(&input.name);
            observed_total = observed_total
                .checked_add(input.byte_length)
                .context("simulation data byte count overflow")?;
            input
                .fingerprint
                .validate(DATA_MANIFEST_VERSION, &format!("data input {}", input.name))?;
        }
        if observed_total != self.total_bytes {
            bail!(
                "simulation data byte count mismatch: manifest {}, inputs {}",
                self.total_bytes,
                observed_total
            );
        }
        let observed_aggregate = aggregate_data_fingerprint(&self.inputs)?;
        if observed_aggregate != self.aggregate_fingerprint {
            bail!(
                "simulation data aggregate mismatch: manifest {}, inputs {}",
                self.aggregate_fingerprint.value,
                observed_aggregate.value
            );
        }
        Ok(())
    }
}

/// Identity responsible for a replayed command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayActor {
    Engine,
    Alliance,
    Empire,
}

/// State-changing commands supported by replay format v1.
///
/// These are the current headless/control commands. Gameplay orders will join
/// this enum when the app and playtest runner move behind one command boundary;
/// changes to existing command semantics require a replay format bump.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReplayCommand {
    AdvanceTicks { count: u64 },
    SetSpeed { speed: GameSpeed },
    ToggleDualAi,
    RevealAllSystems,
    ForceVictoryCheck,
}

impl ReplayCommand {
    fn validate(&self, actor: ReplayActor) -> anyhow::Result<()> {
        if let Self::AdvanceTicks { count: 0 } = self {
            bail!("advance_ticks command count must be greater than zero");
        }
        if let Self::AdvanceTicks { count } = self {
            if *count > MAX_ADVANCE_TICKS_PER_COMMAND {
                bail!(
                    "advance_ticks command count {} exceeds the format-v1 limit {}",
                    count,
                    MAX_ADVANCE_TICKS_PER_COMMAND
                );
            }
        }
        if matches!(
            self,
            Self::AdvanceTicks { .. }
                | Self::ToggleDualAi
                | Self::RevealAllSystems
                | Self::ForceVictoryCheck
        ) && actor != ReplayActor::Engine
        {
            bail!("engine control command cannot be issued by {actor:?}");
        }
        Ok(())
    }
}

/// One command at a precise position in the replay's total order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayCommandRecord {
    /// Simulation tick immediately before the command is applied.
    pub tick: u64,
    /// Zero-based, gap-free global command sequence.
    pub sequence: u64,
    pub actor: ReplayActor,
    pub command: ReplayCommand,
}

/// Expected state after a declared prefix of the command stream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayCheckpoint {
    pub tick: u64,
    /// Number of commands already applied at this checkpoint.
    pub command_count: u64,
    /// Display form emitted by `StateFingerprint`, such as `v1:0123abcd...`.
    pub state_fingerprint: String,
}

/// Complete replay identity and ordered command/checkpoint stream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayManifest {
    pub format: String,
    pub format_version: u16,
    pub engine_version: String,
    pub seed: u64,
    pub rng: String,
    pub rolls_per_tick: u16,
    pub data: SimulationDataManifest,
    pub config_fingerprint: ContentFingerprint,
    pub initial_state_fingerprint: String,
    pub commands: Vec<ReplayCommandRecord>,
    pub checkpoints: Vec<ReplayCheckpoint>,
}

impl ReplayManifest {
    pub fn new(
        engine_version: impl Into<String>,
        seed: u64,
        data: SimulationDataManifest,
        config: &GameConfig,
        initial_state_fingerprint: StateFingerprint,
    ) -> anyhow::Result<Self> {
        let manifest = Self {
            format: "open-rebellion-replay".to_string(),
            format_version: REPLAY_FORMAT_VERSION,
            engine_version: engine_version.into(),
            seed,
            rng: "xoshiro256++/0.6".to_string(),
            rolls_per_tick: REPLAY_ROLLS_PER_TICK,
            data,
            config_fingerprint: compute_config_fingerprint(config)?,
            initial_state_fingerprint: initial_state_fingerprint.to_string(),
            commands: Vec::new(),
            checkpoints: Vec::new(),
        };
        manifest.validate()?;
        Ok(manifest)
    }

    /// Append a command while assigning the only valid next sequence number.
    pub fn record_command(
        &mut self,
        tick: u64,
        actor: ReplayActor,
        command: ReplayCommand,
    ) -> anyhow::Result<u64> {
        if self
            .commands
            .last()
            .is_some_and(|previous| tick < previous.tick)
        {
            bail!("command tick {tick} precedes the previous command");
        }
        command.validate(actor)?;
        let sequence = u64::try_from(self.commands.len()).context("too many replay commands")?;
        self.commands.push(ReplayCommandRecord {
            tick,
            sequence,
            actor,
            command,
        });
        Ok(sequence)
    }

    /// Append a checkpoint after `command_count` commands have been applied.
    pub fn record_checkpoint(
        &mut self,
        tick: u64,
        command_count: u64,
        state_fingerprint: StateFingerprint,
    ) -> anyhow::Result<()> {
        if command_count > self.commands.len() as u64 {
            bail!(
                "checkpoint references {} commands but stream contains {}",
                command_count,
                self.commands.len()
            );
        }
        let checkpoint = ReplayCheckpoint {
            tick,
            command_count,
            state_fingerprint: state_fingerprint.to_string(),
        };
        if self.checkpoints.last().is_some_and(|previous| {
            checkpoint.tick < previous.tick || checkpoint.command_count <= previous.command_count
        }) {
            bail!("checkpoint ticks must be monotonic and command counts strictly increasing");
        }
        validate_state_fingerprint(&checkpoint.state_fingerprint)?;
        self.checkpoints.push(checkpoint);
        Ok(())
    }

    /// Validate the full replay before execution or comparison.
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.format != "open-rebellion-replay" {
            bail!("unsupported replay format {:?}", self.format);
        }
        if self.format_version != REPLAY_FORMAT_VERSION {
            bail!(
                "unsupported replay format version {} (expected {})",
                self.format_version,
                REPLAY_FORMAT_VERSION
            );
        }
        if self.engine_version.trim().is_empty() {
            bail!("replay engine version is empty");
        }
        if self.rng != "xoshiro256++/0.6" {
            bail!("unsupported replay RNG {:?}", self.rng);
        }
        if self.rolls_per_tick != REPLAY_ROLLS_PER_TICK {
            bail!(
                "unsupported replay roll budget {} (expected {})",
                self.rolls_per_tick,
                REPLAY_ROLLS_PER_TICK
            );
        }
        self.data.validate()?;
        self.config_fingerprint
            .validate(CONFIG_FINGERPRINT_VERSION, "configuration")?;
        validate_state_fingerprint(&self.initial_state_fingerprint)?;

        let mut previous_tick = 0_u64;
        for (index, record) in self.commands.iter().enumerate() {
            let expected_sequence = index as u64;
            if record.sequence != expected_sequence {
                bail!(
                    "replay command sequence is not gap-free: expected {}, found {}",
                    expected_sequence,
                    record.sequence
                );
            }
            if index > 0 && record.tick < previous_tick {
                bail!("replay command ticks are not monotonic at sequence {index}");
            }
            record.command.validate(record.actor)?;
            previous_tick = record.tick;
        }

        let mut previous_position: Option<(u64, u64)> = None;
        for checkpoint in &self.checkpoints {
            if checkpoint.command_count > self.commands.len() as u64 {
                bail!(
                    "checkpoint references {} commands but stream contains {}",
                    checkpoint.command_count,
                    self.commands.len()
                );
            }
            let position = (checkpoint.tick, checkpoint.command_count);
            if previous_position
                .is_some_and(|previous| position.0 < previous.0 || position.1 <= previous.1)
            {
                bail!("checkpoint ticks must be monotonic and command counts strictly increasing");
            }
            if checkpoint.command_count > 0 {
                let command = &self.commands[checkpoint.command_count as usize - 1];
                if checkpoint.tick < command.tick {
                    bail!(
                        "checkpoint tick {} precedes applied command {} at tick {}",
                        checkpoint.tick,
                        command.sequence,
                        command.tick
                    );
                }
            }
            validate_state_fingerprint(&checkpoint.state_fingerprint)?;
            previous_position = Some(position);
        }
        Ok(())
    }

    /// Serialize a validated replay using stable struct field ordering.
    pub fn to_json_pretty(&self) -> anyhow::Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serializing replay manifest")
    }

    /// Decode and validate a replay before it reaches an executor.
    pub fn from_json(bytes: &[u8]) -> anyhow::Result<Self> {
        let manifest: Self =
            serde_json::from_slice(bytes).context("decoding replay manifest JSON")?;
        manifest.validate()?;
        Ok(manifest)
    }
}

/// Runtime identity supplied independently of the replay artifact.
///
/// Keeping these values outside the JSON prevents an artifact from validating
/// itself when the caller loaded a different engine or simulation data set.
#[derive(Debug, Clone, Copy)]
pub struct ReplayEnvironment<'a> {
    pub engine_version: &'a str,
    pub seed: u64,
    pub data: &'a SimulationDataManifest,
}

/// Result of executing a complete replay stream.
#[derive(Debug)]
pub struct ReplayExecutionResult {
    pub final_state: SaveState,
    pub observed_checkpoints: Vec<ReplayCheckpoint>,
}

/// Artifact and final state produced by a recording pass.
#[derive(Debug)]
pub struct ReplayRecording {
    pub manifest: ReplayManifest,
    pub execution: ReplayExecutionResult,
}

/// Apply commands to a state and record a fingerprint after each command.
///
/// Command ticks and sequence numbers come from the runtime, not the caller.
/// This makes the resulting artifact suitable for strict playback.
pub fn record_replay<I>(
    environment: ReplayEnvironment<'_>,
    initial_state: SaveState,
    commands: I,
) -> anyhow::Result<ReplayRecording>
where
    I: IntoIterator<Item = (ReplayActor, ReplayCommand)>,
{
    environment.data.validate()?;
    if environment.engine_version.trim().is_empty() {
        bail!("replay engine version is empty");
    }

    let initial_fingerprint = compute_state_fingerprint(&initial_state)?;
    let mut manifest = ReplayManifest::new(
        environment.engine_version,
        environment.seed,
        environment.data.clone(),
        &initial_state.game_config,
        initial_fingerprint,
    )?;
    let mut runtime = ReplayRuntime::from_save_state(initial_state);
    let mut observed_checkpoints = Vec::new();

    for (actor, command) in commands {
        let tick = runtime.current_tick();
        manifest.record_command(tick, actor, command.clone())?;
        runtime.apply_command(&command)?;

        let command_count = manifest.commands.len() as u64;
        let fingerprint = compute_state_fingerprint(&runtime.snapshot())?;
        let checkpoint = ReplayCheckpoint {
            tick: runtime.current_tick(),
            command_count,
            state_fingerprint: fingerprint.to_string(),
        };
        manifest.record_checkpoint(checkpoint.tick, checkpoint.command_count, fingerprint)?;
        observed_checkpoints.push(checkpoint);
    }

    manifest.validate()?;
    Ok(ReplayRecording {
        manifest,
        execution: ReplayExecutionResult {
            final_state: runtime.snapshot(),
            observed_checkpoints,
        },
    })
}

/// Execute a validated replay and fail at the first incompatible input,
/// command position, or state checkpoint.
pub fn execute_replay(
    environment: ReplayEnvironment<'_>,
    manifest: &ReplayManifest,
    initial_state: SaveState,
) -> anyhow::Result<ReplayExecutionResult> {
    execute_replay_observed(environment, manifest, initial_state, |_| {})
}

/// Execute a replay while reporting each actual checkpoint before comparison.
///
/// The observer preserves the checkpoint that caused a failure, which lets a
/// native or browser gate explain the first divergent prefix without changing
/// strict fail-fast execution semantics.
pub fn execute_replay_observed<F>(
    environment: ReplayEnvironment<'_>,
    manifest: &ReplayManifest,
    initial_state: SaveState,
    mut observe: F,
) -> anyhow::Result<ReplayExecutionResult>
where
    F: FnMut(&ReplayCheckpoint),
{
    validate_execution_environment(environment, manifest, &initial_state)?;

    let mut runtime = ReplayRuntime::from_save_state(initial_state);
    let mut checkpoint_index = 0_usize;
    let mut observed_checkpoints = Vec::with_capacity(manifest.checkpoints.len());
    verify_checkpoints_at_prefix(
        manifest,
        &runtime,
        0,
        &mut checkpoint_index,
        &mut observed_checkpoints,
        &mut observe,
    )?;

    for (index, record) in manifest.commands.iter().enumerate() {
        if record.tick != runtime.current_tick() {
            bail!(
                "replay command {} tick mismatch: artifact {}, runtime {}",
                record.sequence,
                record.tick,
                runtime.current_tick()
            );
        }
        runtime.apply_command(&record.command)?;
        verify_checkpoints_at_prefix(
            manifest,
            &runtime,
            (index + 1) as u64,
            &mut checkpoint_index,
            &mut observed_checkpoints,
            &mut observe,
        )?;
    }

    if checkpoint_index != manifest.checkpoints.len() {
        bail!(
            "replay ended before checkpoint {} after {} commands",
            checkpoint_index,
            manifest.commands.len()
        );
    }

    Ok(ReplayExecutionResult {
        final_state: runtime.snapshot(),
        observed_checkpoints,
    })
}

fn validate_execution_environment(
    environment: ReplayEnvironment<'_>,
    manifest: &ReplayManifest,
    initial_state: &SaveState,
) -> anyhow::Result<()> {
    manifest.validate()?;
    environment.data.validate()?;
    if manifest.engine_version != environment.engine_version {
        bail!(
            "replay engine version mismatch: artifact {:?}, runtime {:?}",
            manifest.engine_version,
            environment.engine_version
        );
    }
    if manifest.seed != environment.seed {
        bail!(
            "replay seed mismatch: artifact {}, runtime {}",
            manifest.seed,
            environment.seed
        );
    }
    if manifest.data != *environment.data {
        bail!(
            "replay simulation data mismatch: artifact {}, runtime {}",
            manifest.data.aggregate_fingerprint.value,
            environment.data.aggregate_fingerprint.value
        );
    }

    let config_fingerprint = compute_config_fingerprint(&initial_state.game_config)?;
    if manifest.config_fingerprint != config_fingerprint {
        bail!(
            "replay configuration mismatch: artifact {}, runtime {}",
            manifest.config_fingerprint.value,
            config_fingerprint.value
        );
    }

    let state_fingerprint = compute_state_fingerprint(initial_state)?;
    if manifest.initial_state_fingerprint != state_fingerprint.to_string() {
        bail!(
            "replay initial-state mismatch: artifact {}, runtime {}",
            manifest.initial_state_fingerprint,
            state_fingerprint
        );
    }
    Ok(())
}

fn verify_checkpoints_at_prefix<F>(
    manifest: &ReplayManifest,
    runtime: &ReplayRuntime,
    command_count: u64,
    checkpoint_index: &mut usize,
    observed: &mut Vec<ReplayCheckpoint>,
    observe: &mut F,
) -> anyhow::Result<()>
where
    F: FnMut(&ReplayCheckpoint),
{
    let Some(checkpoint) = manifest.checkpoints.get(*checkpoint_index) else {
        return Ok(());
    };
    if checkpoint.command_count < command_count {
        bail!(
            "replay skipped checkpoint {} after {} commands",
            *checkpoint_index,
            checkpoint.command_count
        );
    }
    if checkpoint.command_count != command_count {
        return Ok(());
    }
    let actual = ReplayCheckpoint {
        tick: runtime.current_tick(),
        command_count,
        state_fingerprint: compute_state_fingerprint(&runtime.snapshot())?.to_string(),
    };
    observe(&actual);

    if checkpoint.tick != actual.tick {
        bail!(
            "replay checkpoint {} tick mismatch after {} commands: artifact {}, runtime {}",
            *checkpoint_index,
            command_count,
            checkpoint.tick,
            actual.tick
        );
    }

    if checkpoint.state_fingerprint != actual.state_fingerprint {
        bail!(
            "replay checkpoint {} state mismatch at tick {} after {} commands: artifact {}, runtime {}",
            *checkpoint_index,
            checkpoint.tick,
            command_count,
            checkpoint.state_fingerprint,
            actual.state_fingerprint
        );
    }
    observed.push(actual);
    *checkpoint_index += 1;
    Ok(())
}

struct ReplayRuntime {
    world: GameWorld,
    states: SimulationStates,
    inactive_fog: FogState,
    player_is_alliance: bool,
    sim_rng: rand_xoshiro::Xoshiro256PlusPlus,
    game_config: GameConfig,
}

impl ReplayRuntime {
    fn from_save_state(state: SaveState) -> Self {
        let SaveState {
            world,
            clock,
            manufacturing,
            missions,
            events,
            ai,
            movement,
            fog_alliance,
            fog_empire,
            player_is_alliance,
            blockade,
            uprising,
            death_star,
            research,
            jedi,
            victory,
            betrayal,
            economy,
            sim_rng,
            ai2,
            repair,
            combat_cooldowns,
            game_config,
            campaign_config,
        } = state;
        let (fog, inactive_fog) = if player_is_alliance {
            (fog_alliance, fog_empire)
        } else {
            (fog_empire, fog_alliance)
        };
        Self {
            world,
            states: SimulationStates {
                clock,
                manufacturing,
                missions,
                events,
                ai,
                ai2,
                movement,
                fog,
                blockade,
                uprising,
                death_star,
                research,
                jedi,
                victory,
                betrayal,
                economy,
                repair,
                combat_cooldowns,
                campaign_config,
            },
            inactive_fog,
            player_is_alliance,
            sim_rng,
            game_config,
        }
    }

    fn current_tick(&self) -> u64 {
        self.states.clock.tick
    }

    fn apply_command(&mut self, command: &ReplayCommand) -> anyhow::Result<()> {
        match command {
            ReplayCommand::AdvanceTicks { count } => {
                for _ in 0..*count {
                    let tick = self
                        .states
                        .clock
                        .tick
                        .checked_add(1)
                        .context("replay tick overflow")?;
                    self.states.clock.tick = tick;
                    let tick_events = [TickEvent { tick }];
                    let rolls = (0..usize::from(REPLAY_ROLLS_PER_TICK))
                        .map(|_| self.sim_rng.gen::<f64>())
                        .collect::<Vec<_>>();
                    let _ = run_simulation_tick(
                        &mut self.world,
                        &mut self.states,
                        &tick_events,
                        &rolls,
                        tick,
                        &self.game_config,
                    );
                }
            }
            ReplayCommand::SetSpeed { speed } => self.states.clock.set_speed(*speed),
            ReplayCommand::ToggleDualAi => {
                self.states.ai2 = if self.states.ai2.is_some() {
                    None
                } else {
                    let faction = match self.states.ai.faction {
                        Some(AiFaction::Empire) => AiFaction::Alliance,
                        _ => AiFaction::Empire,
                    };
                    Some(AIState::new(faction))
                };
            }
            ReplayCommand::RevealAllSystems => {
                let systems = self.world.systems.keys().collect::<Vec<_>>();
                for system in systems {
                    self.states.fog.reveal(system);
                    self.inactive_fog.reveal(system);
                }
            }
            ReplayCommand::ForceVictoryCheck => {
                let tick_events = [TickEvent {
                    tick: self.current_tick(),
                }];
                if VictorySystem::check(
                    &self.states.victory,
                    &self.world,
                    &tick_events,
                    self.states.campaign_config.victory_conditions,
                )
                .is_some()
                {
                    self.states.victory.resolved = true;
                }
            }
        }
        Ok(())
    }

    fn snapshot(&self) -> SaveState {
        let (fog_alliance, fog_empire) = if self.player_is_alliance {
            (self.states.fog.clone(), self.inactive_fog.clone())
        } else {
            (self.inactive_fog.clone(), self.states.fog.clone())
        };
        SaveState {
            world: self.world.clone(),
            clock: self.states.clock.clone(),
            manufacturing: self.states.manufacturing.clone(),
            missions: self.states.missions.clone(),
            events: self.states.events.clone(),
            ai: self.states.ai.clone(),
            movement: self.states.movement.clone(),
            fog_alliance,
            fog_empire,
            player_is_alliance: self.player_is_alliance,
            blockade: self.states.blockade.clone(),
            uprising: self.states.uprising.clone(),
            death_star: self.states.death_star.clone(),
            research: self.states.research.clone(),
            jedi: self.states.jedi.clone(),
            victory: self.states.victory.clone(),
            betrayal: self.states.betrayal.clone(),
            economy: self.states.economy.clone(),
            sim_rng: self.sim_rng.clone(),
            ai2: self.states.ai2.clone(),
            repair: self.states.repair.clone(),
            combat_cooldowns: self.states.combat_cooldowns.clone(),
            game_config: self.game_config.clone(),
            campaign_config: self.states.campaign_config,
        }
    }
}

/// Hash an in-memory set of `.DAT` files identically on native and WASM.
pub fn compute_simulation_data_manifest<I, K, V>(
    inputs: I,
) -> anyhow::Result<SimulationDataManifest>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: AsRef<[u8]>,
{
    let mut canonical_inputs = Vec::new();
    for (name, bytes) in inputs {
        let name = canonical_data_name(name.as_ref())?;
        canonical_inputs.push((name, bytes.as_ref().to_vec()));
    }
    if canonical_inputs.is_empty() {
        bail!("cannot fingerprint an empty simulation data set");
    }
    canonical_inputs.sort_by(|left, right| left.0.cmp(&right.0));

    let mut names = HashSet::with_capacity(canonical_inputs.len());
    let mut manifest_inputs = Vec::with_capacity(canonical_inputs.len());
    let mut total_bytes = 0_u64;
    for (name, bytes) in canonical_inputs {
        if !names.insert(name.clone()) {
            bail!("duplicate canonical simulation data input {name}");
        }
        let byte_length = u64::try_from(bytes.len()).context("simulation data input too large")?;
        total_bytes = total_bytes
            .checked_add(byte_length)
            .context("simulation data byte count overflow")?;
        let value = fingerprint_bytes(
            b"OPENREB-DATA-INPUT\0",
            DATA_MANIFEST_VERSION,
            &name,
            &bytes,
        );
        manifest_inputs.push(DataInputFingerprint {
            name,
            byte_length,
            fingerprint: ContentFingerprint::fnv1a(DATA_MANIFEST_VERSION, value),
        });
    }

    let aggregate_fingerprint = aggregate_data_fingerprint(&manifest_inputs)?;

    let manifest = SimulationDataManifest {
        format_version: DATA_MANIFEST_VERSION,
        total_bytes,
        aggregate_fingerprint,
        inputs: manifest_inputs,
    };
    manifest.validate()?;
    Ok(manifest)
}

fn aggregate_data_fingerprint(
    inputs: &[DataInputFingerprint],
) -> anyhow::Result<ContentFingerprint> {
    let mut aggregate = Fnv1a64::new();
    aggregate.update(b"OPENREB-DATA-MANIFEST\0");
    aggregate.update(&DATA_MANIFEST_VERSION.to_le_bytes());
    aggregate.update(&(inputs.len() as u64).to_le_bytes());
    for input in inputs {
        aggregate.update(&(input.name.len() as u64).to_le_bytes());
        aggregate.update(input.name.as_bytes());
        aggregate.update(&input.byte_length.to_le_bytes());
        let value = u64::from_str_radix(&input.fingerprint.value, 16)
            .with_context(|| format!("decoding data input fingerprint for {}", input.name))?;
        aggregate.update(&value.to_le_bytes());
    }
    Ok(ContentFingerprint::fnv1a(
        DATA_MANIFEST_VERSION,
        aggregate.finish(),
    ))
}

/// Read and fingerprint every `.DAT` file in a native game-data directory.
#[cfg(not(target_arch = "wasm32"))]
pub fn compute_simulation_data_manifest_from_dir(
    data_dir: &Path,
) -> anyhow::Result<SimulationDataManifest> {
    let mut inputs = Vec::new();
    for entry in std::fs::read_dir(data_dir)
        .with_context(|| format!("reading simulation data directory {}", data_dir.display()))?
    {
        let entry = entry.context("reading simulation data directory entry")?;
        let path = entry.path();
        if !path.is_file()
            || path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_none_or(|extension| !extension.eq_ignore_ascii_case("dat"))
        {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .context("simulation data filename is not valid UTF-8")?
            .to_string();
        let bytes = std::fs::read(&path)
            .with_context(|| format!("reading simulation data input {}", path.display()))?;
        inputs.push((name, bytes));
    }
    compute_simulation_data_manifest(inputs)
}

/// Fingerprint the serialized tuning configuration embedded in a replay.
pub fn compute_config_fingerprint(config: &GameConfig) -> anyhow::Result<ContentFingerprint> {
    let bytes = serde_json::to_vec(config).context("serializing replay configuration")?;
    Ok(ContentFingerprint::fnv1a(
        CONFIG_FINGERPRINT_VERSION,
        fingerprint_bytes(
            b"OPENREB-GAME-CONFIG\0",
            CONFIG_FINGERPRINT_VERSION,
            "game-config",
            &bytes,
        ),
    ))
}

fn canonical_data_name(name: &str) -> anyhow::Result<String> {
    if name.is_empty() || !name.is_ascii() || name.contains(['/', '\\']) {
        bail!("invalid flat simulation data filename {name:?}");
    }
    let canonical = name.to_ascii_uppercase();
    validate_data_name(&canonical)?;
    Ok(canonical)
}

fn validate_data_name(name: &str) -> anyhow::Result<()> {
    if name.is_empty()
        || !name.is_ascii()
        || name != name.to_ascii_uppercase()
        || name.contains(['/', '\\'])
        || !name.ends_with(".DAT")
    {
        bail!("invalid canonical simulation data filename {name:?}");
    }
    Ok(())
}

fn validate_state_fingerprint(value: &str) -> anyhow::Result<()> {
    let Some((version, digest)) = value.split_once(':') else {
        bail!("invalid state fingerprint {value:?}");
    };
    if version.len() < 2
        || !version.starts_with('v')
        || !version[1..].bytes().all(|byte| byte.is_ascii_digit())
        || digest.len() != 16
        || !digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        bail!("invalid state fingerprint {value:?}");
    }
    Ok(())
}

fn fingerprint_bytes(domain: &[u8], version: u16, name: &str, bytes: &[u8]) -> u64 {
    let mut hash = Fnv1a64::new();
    hash.update(domain);
    hash.update(&version.to_le_bytes());
    hash.update(&(name.len() as u64).to_le_bytes());
    hash.update(name.as_bytes());
    hash.update(&(bytes.len() as u64).to_le_bytes());
    hash.update(bytes);
    hash.finish()
}

struct Fnv1a64(u64);

impl Fnv1a64 {
    fn new() -> Self {
        Self(FNV_OFFSET_BASIS)
    }

    fn update(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(FNV_PRIME);
        }
    }

    fn finish(self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;
    use rebellion_core::betrayal::BetrayalState;
    use rebellion_core::blockade::BlockadeState;
    use rebellion_core::dat::{ExplorationStatus, Faction, SectorGroup};
    use rebellion_core::death_star::DeathStarState;
    use rebellion_core::economy::EconomyState;
    use rebellion_core::events::EventState;
    use rebellion_core::jedi::JediState;
    use rebellion_core::manufacturing::ManufacturingState;
    use rebellion_core::missions::MissionState;
    use rebellion_core::movement::MovementState;
    use rebellion_core::repair::RepairState;
    use rebellion_core::research::ResearchState;
    use rebellion_core::tick::GameClock;
    use rebellion_core::uprising::UprisingState;
    use rebellion_core::victory::VictoryState;
    use rebellion_core::world::{CampaignConfig, ControlKind, Sector, System};

    fn sample_data() -> SimulationDataManifest {
        compute_simulation_data_manifest([
            ("systemsD.dat", b"systems".as_slice()),
            ("CAPSHPSD.DAT", b"ships".as_slice()),
        ])
        .unwrap()
    }

    fn sample_state_fingerprint() -> StateFingerprint {
        StateFingerprint {
            version: 1,
            value: 0x0123_4567_89ab_cdef,
        }
    }

    fn sample_save_state(seed: u64) -> SaveState {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: rebellion_core::ids::DatId::new(0x9200_0000),
            name: "Test Sector".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: Vec::new(),
        });
        let alliance_hq = world.systems.insert(System {
            dat_id: rebellion_core::ids::DatId::new(0x9000_0000),
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
            dat_id: rebellion_core::ids::DatId::new(0x9000_0001),
            name: "Empire HQ".into(),
            sector,
            x: 100,
            y: 100,
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
            combat_cooldowns: std::collections::HashMap::new(),
            game_config: GameConfig::default(),
            campaign_config: CampaignConfig::default(),
        }
    }

    #[test]
    fn data_manifest_is_order_independent_and_json_safe() {
        let forward = sample_data();
        let reverse = compute_simulation_data_manifest([
            ("CAPSHPSD.DAT", b"ships".as_slice()),
            ("SYSTEMSD.DAT", b"systems".as_slice()),
        ])
        .unwrap();

        assert_eq!(forward, reverse);
        assert_eq!(forward.total_bytes, 12);
        assert_eq!(forward.inputs[0].name, "CAPSHPSD.DAT");
        assert_eq!(forward.inputs[1].name, "SYSTEMSD.DAT");
        assert_eq!(forward.aggregate_fingerprint.value.len(), 16);
        assert!(forward
            .aggregate_fingerprint
            .value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()));
    }

    #[test]
    fn data_manifest_detects_content_and_identity_changes() {
        let original = sample_data();
        let content_changed = compute_simulation_data_manifest([
            ("SYSTEMSD.DAT", b"systemz".as_slice()),
            ("CAPSHPSD.DAT", b"ships".as_slice()),
        ])
        .unwrap();
        let name_changed = compute_simulation_data_manifest([
            ("SYSTEMXD.DAT", b"systems".as_slice()),
            ("CAPSHPSD.DAT", b"ships".as_slice()),
        ])
        .unwrap();

        assert_ne!(
            original.aggregate_fingerprint,
            content_changed.aggregate_fingerprint
        );
        assert_ne!(
            original.aggregate_fingerprint,
            name_changed.aggregate_fingerprint
        );

        let mut tampered = original;
        tampered.inputs[0].byte_length += 1;
        tampered.total_bytes += 1;
        assert!(tampered
            .validate()
            .unwrap_err()
            .to_string()
            .contains("aggregate mismatch"));
    }

    #[test]
    fn data_manifest_rejects_empty_ambiguous_and_non_dat_inputs() {
        let empty: Vec<(String, Vec<u8>)> = Vec::new();
        assert!(compute_simulation_data_manifest(empty).is_err());
        assert!(compute_simulation_data_manifest([
            ("systemsD.dat", b"a".as_slice()),
            ("SYSTEMSD.DAT", b"b".as_slice()),
        ])
        .unwrap_err()
        .to_string()
        .contains("duplicate canonical"));
        assert!(compute_simulation_data_manifest([("../SYSTEMSD.DAT", b"a".as_slice())]).is_err());
        assert!(compute_simulation_data_manifest([("notes.txt", b"a".as_slice())]).is_err());
    }

    #[test]
    fn replay_round_trip_preserves_total_order_and_checkpoints() {
        let mut replay = ReplayManifest::new(
            "0.1.0-test",
            42,
            sample_data(),
            &GameConfig::default(),
            sample_state_fingerprint(),
        )
        .unwrap();
        assert_eq!(
            replay
                .record_command(
                    0,
                    ReplayActor::Engine,
                    ReplayCommand::AdvanceTicks { count: 10 },
                )
                .unwrap(),
            0
        );
        assert_eq!(
            replay
                .record_command(
                    10,
                    ReplayActor::Alliance,
                    ReplayCommand::SetSpeed {
                        speed: GameSpeed::Paused,
                    },
                )
                .unwrap(),
            1
        );
        replay
            .record_checkpoint(10, 2, sample_state_fingerprint())
            .unwrap();

        let json = replay.to_json_pretty().unwrap();
        let decoded = ReplayManifest::from_json(&json).unwrap();
        assert_eq!(decoded, replay);
        assert!(std::str::from_utf8(&json)
            .unwrap()
            .contains("\"state_fingerprint\": \"v1:0123456789abcdef\""));
    }

    #[test]
    fn replay_validation_rejects_version_order_and_authority_errors() {
        let base = ReplayManifest::new(
            "0.1.0-test",
            42,
            sample_data(),
            &GameConfig::default(),
            sample_state_fingerprint(),
        )
        .unwrap();

        let mut unsupported = base.clone();
        unsupported.format_version += 1;
        assert!(unsupported.validate().is_err());

        let mut bad_sequence = base.clone();
        bad_sequence.commands.push(ReplayCommandRecord {
            tick: 0,
            sequence: 2,
            actor: ReplayActor::Engine,
            command: ReplayCommand::AdvanceTicks { count: 1 },
        });
        assert!(bad_sequence
            .validate()
            .unwrap_err()
            .to_string()
            .contains("gap-free"));

        let mut bad_actor = base;
        bad_actor.commands.push(ReplayCommandRecord {
            tick: 0,
            sequence: 0,
            actor: ReplayActor::Empire,
            command: ReplayCommand::RevealAllSystems,
        });
        assert!(bad_actor
            .validate()
            .unwrap_err()
            .to_string()
            .contains("engine control"));
    }

    #[test]
    fn replay_recording_rejects_time_travel_and_invalid_checkpoints() {
        let mut replay = ReplayManifest::new(
            "0.1.0-test",
            42,
            sample_data(),
            &GameConfig::default(),
            sample_state_fingerprint(),
        )
        .unwrap();
        replay
            .record_command(
                10,
                ReplayActor::Engine,
                ReplayCommand::AdvanceTicks { count: 1 },
            )
            .unwrap();
        assert!(replay
            .record_command(
                9,
                ReplayActor::Engine,
                ReplayCommand::AdvanceTicks { count: 1 },
            )
            .is_err());
        assert!(replay
            .record_command(
                10,
                ReplayActor::Engine,
                ReplayCommand::AdvanceTicks { count: 0 },
            )
            .is_err());
        assert!(replay
            .record_checkpoint(10, 2, sample_state_fingerprint())
            .is_err());
    }

    #[test]
    fn configuration_fingerprint_is_deterministic_and_sensitive() {
        let original = GameConfig::default();
        let same = GameConfig::default();
        let mut changed = GameConfig::default();
        changed.ai.tick_interval += 1;

        assert_eq!(
            compute_config_fingerprint(&original).unwrap(),
            compute_config_fingerprint(&same).unwrap()
        );
        assert_ne!(
            compute_config_fingerprint(&original).unwrap(),
            compute_config_fingerprint(&changed).unwrap()
        );
    }

    #[test]
    fn recorder_and_executor_match_every_checkpoint() {
        let data = sample_data();
        let environment = ReplayEnvironment {
            engine_version: "0.1.0-test",
            seed: 42,
            data: &data,
        };
        let initial = sample_save_state(environment.seed);
        let recording = record_replay(
            environment,
            initial.clone(),
            [
                (
                    ReplayActor::Alliance,
                    ReplayCommand::SetSpeed {
                        speed: GameSpeed::Fast,
                    },
                ),
                (ReplayActor::Engine, ReplayCommand::ToggleDualAi),
                (ReplayActor::Engine, ReplayCommand::RevealAllSystems),
                (
                    ReplayActor::Engine,
                    ReplayCommand::AdvanceTicks { count: 3 },
                ),
                (ReplayActor::Engine, ReplayCommand::ForceVictoryCheck),
            ],
        )
        .unwrap();

        assert_eq!(recording.manifest.commands.len(), 5);
        assert_eq!(recording.manifest.checkpoints.len(), 5);
        assert_eq!(recording.manifest.commands[3].tick, 0);
        assert_eq!(recording.manifest.checkpoints[3].tick, 3);

        let json = recording.manifest.to_json_pretty().unwrap();
        let decoded = ReplayManifest::from_json(&json).unwrap();
        let executed = execute_replay(environment, &decoded, initial).unwrap();

        assert_eq!(
            executed.observed_checkpoints,
            recording.manifest.checkpoints
        );
        assert_eq!(
            compute_state_fingerprint(&executed.final_state).unwrap(),
            compute_state_fingerprint(&recording.execution.final_state).unwrap()
        );
        assert_eq!(executed.final_state.clock.tick, 3);
        assert!(executed.final_state.ai2.is_some());
        assert_eq!(executed.final_state.fog_alliance.len(), 2);
        assert_eq!(executed.final_state.fog_empire.len(), 2);
    }

    #[test]
    fn executor_accepts_an_initial_state_restored_from_save_v12() {
        let data = sample_data();
        let environment = ReplayEnvironment {
            engine_version: "0.1.0-test",
            seed: 77,
            data: &data,
        };
        let initial = sample_save_state(environment.seed);
        let recording = record_replay(
            environment,
            initial.clone(),
            [(
                ReplayActor::Engine,
                ReplayCommand::AdvanceTicks { count: 2 },
            )],
        )
        .unwrap();

        let saves = tempfile::tempdir().unwrap();
        crate::save::save_slot(saves.path(), 0, "Replay Start", &initial, &[]).unwrap();
        let (_, restored) = crate::save::load_slot(saves.path(), 0).unwrap();
        let executed = execute_replay(environment, &recording.manifest, restored).unwrap();

        assert_eq!(
            compute_state_fingerprint(&executed.final_state).unwrap(),
            compute_state_fingerprint(&recording.execution.final_state).unwrap()
        );
    }

    #[test]
    fn executor_rejects_environment_command_and_checkpoint_mismatches() {
        let data = sample_data();
        let environment = ReplayEnvironment {
            engine_version: "0.1.0-test",
            seed: 42,
            data: &data,
        };
        let initial = sample_save_state(environment.seed);
        let recording = record_replay(
            environment,
            initial.clone(),
            [(
                ReplayActor::Engine,
                ReplayCommand::AdvanceTicks { count: 1 },
            )],
        )
        .unwrap();

        let wrong_seed = ReplayEnvironment {
            seed: 43,
            ..environment
        };
        assert!(
            execute_replay(wrong_seed, &recording.manifest, initial.clone())
                .unwrap_err()
                .to_string()
                .contains("seed mismatch")
        );

        let wrong_engine = ReplayEnvironment {
            engine_version: "0.2.0-test",
            ..environment
        };
        assert!(
            execute_replay(wrong_engine, &recording.manifest, initial.clone())
                .unwrap_err()
                .to_string()
                .contains("engine version mismatch")
        );

        let changed_data = compute_simulation_data_manifest([
            ("SYSTEMSD.DAT", b"changed".as_slice()),
            ("CAPSHPSD.DAT", b"ships".as_slice()),
        ])
        .unwrap();
        let wrong_data = ReplayEnvironment {
            data: &changed_data,
            ..environment
        };
        assert!(
            execute_replay(wrong_data, &recording.manifest, initial.clone())
                .unwrap_err()
                .to_string()
                .contains("simulation data mismatch")
        );

        let mut changed_config = initial.clone();
        changed_config.game_config.ai.tick_interval += 1;
        assert!(
            execute_replay(environment, &recording.manifest, changed_config)
                .unwrap_err()
                .to_string()
                .contains("configuration mismatch")
        );

        let mut changed_state = initial.clone();
        changed_state.clock.set_speed(GameSpeed::Fast);
        assert!(
            execute_replay(environment, &recording.manifest, changed_state)
                .unwrap_err()
                .to_string()
                .contains("initial-state mismatch")
        );

        let mut wrong_tick = recording.manifest.clone();
        wrong_tick.commands[0].tick = 1;
        assert!(execute_replay(environment, &wrong_tick, initial.clone())
            .unwrap_err()
            .to_string()
            .contains("command 0 tick mismatch"));

        let mut wrong_checkpoint = recording.manifest;
        wrong_checkpoint.checkpoints[0].state_fingerprint = "v1:0000000000000000".into();
        assert!(execute_replay(environment, &wrong_checkpoint, initial)
            .unwrap_err()
            .to_string()
            .contains("checkpoint 0 state mismatch"));
    }

    #[test]
    fn observed_executor_retains_the_failing_actual_checkpoint() {
        let data = sample_data();
        let environment = ReplayEnvironment {
            engine_version: "0.1.0-test",
            seed: 42,
            data: &data,
        };
        let initial = sample_save_state(environment.seed);
        let recording = record_replay(
            environment,
            initial.clone(),
            [
                (ReplayActor::Engine, ReplayCommand::ToggleDualAi),
                (
                    ReplayActor::Engine,
                    ReplayCommand::AdvanceTicks { count: 1 },
                ),
            ],
        )
        .unwrap();
        let expected_actual = recording.manifest.checkpoints[1].clone();
        let mut changed = recording.manifest;
        changed.checkpoints[1].state_fingerprint = "v1:0000000000000000".into();
        let mut observed = Vec::new();

        let error = execute_replay_observed(environment, &changed, initial, |checkpoint| {
            observed.push(checkpoint.clone());
        })
        .unwrap_err();

        assert!(error.to_string().contains("checkpoint 1 state mismatch"));
        assert_eq!(observed.len(), 2);
        assert_eq!(observed[1], expected_actual);
    }

    #[test]
    fn replay_validation_rejects_decreasing_checkpoint_prefixes_and_huge_advances() {
        let mut replay = ReplayManifest::new(
            "0.1.0-test",
            42,
            sample_data(),
            &GameConfig::default(),
            sample_state_fingerprint(),
        )
        .unwrap();
        replay
            .record_command(0, ReplayActor::Engine, ReplayCommand::ToggleDualAi)
            .unwrap();
        replay
            .record_command(0, ReplayActor::Engine, ReplayCommand::ToggleDualAi)
            .unwrap();
        replay
            .record_checkpoint(0, 2, sample_state_fingerprint())
            .unwrap();
        assert!(replay
            .record_checkpoint(1, 1, sample_state_fingerprint())
            .unwrap_err()
            .to_string()
            .contains("command counts strictly increasing"));

        assert!(ReplayCommand::AdvanceTicks {
            count: MAX_ADVANCE_TICKS_PER_COMMAND + 1
        }
        .validate(ReplayActor::Engine)
        .unwrap_err()
        .to_string()
        .contains("format-v1 limit"));
    }
}
