//! PerceptionIntegrator — centralizes effect application and telemetry emission.
//!
//! Phase 4 of Knesset Ereshkigal. The integrator translates ad-hoc system events
//! into world mutations and structured `GameEventRecord` telemetry.
//!
//! Architecture: simulation.rs orchestrates 17 system advance() calls and delegates
//! effect application + telemetry to the integrator. This keeps simulation.rs focused
//! on tick composition while the integrator owns the mutation/telemetry contract.
//!
//! Current state: scaffold with name resolution helpers and the PerceptionIntegrator
//! struct. Effect application methods will be migrated from simulation.rs incrementally.

use rebellion_core::ai::AIAction;
use rebellion_core::fog::RevealEvent;
use rebellion_core::game_events::*;
use rebellion_core::ids::{CharacterKey, SystemKey};
use rebellion_core::victory::VictoryOutcome;
use rebellion_core::world::{ControlKind, GameWorld};

// ---------------------------------------------------------------------------
// Name resolution helpers (shared with simulation.rs)
// ---------------------------------------------------------------------------

/// Resolve a SystemKey to the system's name, or a fallback string.
pub fn sys_name(world: &GameWorld, key: SystemKey) -> String {
    world
        .systems
        .get(key)
        .map(|s| s.name.clone())
        .unwrap_or_else(|| format!("{:?}", key))
}

/// Resolve a CharacterKey to the character's name, or a fallback string.
pub fn char_name(world: &GameWorld, key: CharacterKey) -> String {
    world
        .characters
        .get(key)
        .map(|c| c.name.clone())
        .unwrap_or_else(|| format!("{:?}", key))
}

/// Build a JSON-friendly system reference with both name and key.
#[allow(dead_code)]
pub fn sys_json(world: &GameWorld, key: SystemKey) -> serde_json::Value {
    serde_json::json!({
        "key": format!("{:?}", key),
        "name": sys_name(world, key),
    })
}

/// Format an AIAction as a structured JSON payload with readable names.
pub fn ai_action_json(action: &AIAction, world: &GameWorld) -> serde_json::Value {
    match action {
        AIAction::MoveFleet {
            fleet,
            to_system,
            reason,
        } => {
            let faction = world
                .fleets
                .get(*fleet)
                .map(|f| if f.is_alliance { "Alliance" } else { "Empire" })
                .unwrap_or("unknown");
            let from = world
                .fleets
                .get(*fleet)
                .map(|f| sys_name(world, f.location))
                .unwrap_or_else(|| "unknown".into());
            serde_json::json!({
                "type": "MoveFleet",
                "faction": faction,
                "from": from,
                "to": sys_name(world, *to_system),
                "reason": format!("{:?}", reason),
            })
        }
        AIAction::DispatchMission { kind, target_system, .. } => {
            serde_json::json!({
                "type": "DispatchMission",
                "kind": format!("{:?}", kind),
                "target": sys_name(world, *target_system),
            })
        }
        AIAction::EnqueueProduction { system, kind, ticks } => {
            serde_json::json!({
                "type": "EnqueueProduction",
                "system": sys_name(world, *system),
                "kind": format!("{:?}", kind),
                "ticks": ticks,
            })
        }
        AIAction::DispatchResearch { character, tech_type, ticks } => {
            serde_json::json!({
                "type": "DispatchResearch",
                "character": char_name(world, *character),
                "tech_type": format!("{:?}", tech_type),
                "ticks": ticks,
            })
        }
    }
}

// ---------------------------------------------------------------------------
// PerceptionIntegrator
// ---------------------------------------------------------------------------

/// Centralizes effect application and telemetry emission for one simulation tick.
///
/// Usage:
/// ```ignore
/// let mut integrator = PerceptionIntegrator::new(tick, wall_ms);
/// // ... system advance() calls with integrator.apply_*() ...
/// let telemetry = integrator.finish();
/// ```
pub struct PerceptionIntegrator {
    events: Vec<GameEventRecord>,
    tick: u64,
    wall_ms: u64,
}

impl PerceptionIntegrator {
    /// Create a new integrator for a single simulation tick.
    pub fn new(tick: u64, wall_ms: u64) -> Self {
        Self {
            events: Vec::new(),
            tick,
            wall_ms,
        }
    }

    /// Current tick number.
    pub fn tick(&self) -> u64 {
        self.tick
    }

    /// Wall-clock milliseconds.
    pub fn wall_ms(&self) -> u64 {
        self.wall_ms
    }

    /// Add a pre-built telemetry record.
    pub fn push(&mut self, record: GameEventRecord) {
        self.events.push(record);
    }

    /// Emit a telemetry record from components.
    pub fn emit(&mut self, system: &'static str, event_type: &'static str, payload: serde_json::Value) {
        self.events.push(GameEventRecord::new(
            self.tick, self.wall_ms, system, event_type, payload,
        ));
    }

    /// Consume the integrator, returning all telemetry records.
    pub fn finish(self) -> Vec<GameEventRecord> {
        self.events
    }

    // ── Step 1: Telemetry-only sections ──────────────────────────────────

    /// Emit fog-of-war reveal telemetry (no world mutations).
    pub fn emit_fog_reveals(&mut self, reveals: &[RevealEvent], world: &GameWorld) {
        for reveal in reveals {
            self.emit(SYS_FOG, EVT_FOG_REVEALED, serde_json::json!({
                "system": sys_name(world, reveal.system),
            }));
        }
    }

    /// Emit victory telemetry and mark victory resolved.
    pub fn apply_victory(
        &mut self,
        outcome: &VictoryOutcome,
        victory_state: &mut rebellion_core::victory::VictoryState,
    ) {
        victory_state.resolved = true;
        self.emit(SYS_VICTORY, EVT_VICTORY, serde_json::json!({
            "outcome": format!("{:?}", outcome),
        }));
    }

    /// Emit campaign snapshot telemetry (no world mutations, read-only).
    pub fn emit_campaign_snapshot(
        &mut self,
        world: &GameWorld,
        movement_len: usize,
    ) {
        let mut alliance_systems = 0u32;
        let mut empire_systems = 0u32;
        let mut neutral_systems = 0u32;
        for (_, sys) in world.systems.iter() {
            match sys.control {
                ControlKind::Controlled(rebellion_core::dat::Faction::Alliance) => alliance_systems += 1,
                ControlKind::Controlled(rebellion_core::dat::Faction::Empire) => empire_systems += 1,
                _ => neutral_systems += 1,
            }
        }
        self.emit("snapshot", EVT_CAMPAIGN_SNAPSHOT, serde_json::json!({
            "tick": self.tick,
            "alliance_systems": alliance_systems,
            "empire_systems": empire_systems,
            "neutral_systems": neutral_systems,
            "fleets": world.fleets.len(),
            "in_transit": movement_len,
        }));
    }
}
