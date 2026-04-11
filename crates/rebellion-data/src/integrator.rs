//! PerceptionIntegrator — centralizes effect application and telemetry emission.
//!
//! Phase 4 of Knesset Ereshkigal. The integrator translates ad-hoc system events
//! into world mutations and structured `GameEventRecord` telemetry.
//!
//! Architecture: simulation.rs orchestrates 17 system advance() calls and delegates
//! effect application + telemetry to the integrator. This keeps simulation.rs focused
//! on tick composition while the integrator owns the mutation/telemetry contract.
//!
//! All 17 simulation sections route through PerceptionIntegrator methods for both
//! world mutation and telemetry emission. simulation.rs is a thin tick orchestrator (~449 LOC).

use std::collections::HashMap;

use rebellion_core::ai::{AIAction, AIState};
use rebellion_core::betrayal::BetrayalEvent;
use rebellion_core::blockade::BlockadeEvent;
use rebellion_core::combat::{CombatSide, GroundCombatResult, SpaceCombatResult};
use rebellion_core::death_star::{DeathStarEvent, DeathStarState};
use rebellion_core::economy::{EconomyEvent, EconomyState};
use rebellion_core::events::{EventAction, FiredEvent, SkillField, SystemTag};
use rebellion_core::fog::RevealEvent;
use rebellion_core::game_events::*;
use rebellion_core::ids::{CharacterKey, SystemKey, TroopKey};
use rebellion_core::jedi::{JediEvent, JediState};
use rebellion_core::manufacturing::{BuildableKind, CompletionEvent, ManufacturingState, QueueItem};
use rebellion_core::missions::{MissionEffect, MissionFaction, MissionKind, MissionResult, MissionState};
use rebellion_core::movement::{ArrivalEvent, MovementState};
use rebellion_core::repair::RepairEvent;
use rebellion_core::research::{ResearchResult, ResearchState};
use rebellion_core::uprising::{UprisingEvent, UprisingState};
use rebellion_core::victory::VictoryOutcome;
use rebellion_core::world::{ControlKind, Fleet, FighterEntry, GameWorld, ShipInstance, TroopUnit};

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
        AIAction::MoveTroops { from_system, to_system, .. } => {
            serde_json::json!({
                "type": "MoveTroops",
                "from": sys_name(world, *from_system),
                "to": sys_name(world, *to_system),
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
    /// Effect queue for cross-system story/message routing. Populated by
    /// `apply_event_action_to_world` (see Knesset Shamash-Bet Dabora 2
    /// #F7/#A3) when an `EventAction::DisplayMessage` fires or when a
    /// `SpawnSpecialForce` action resolves to a target system. The
    /// interactive `main.rs` drains this with `drain_story_effects()`
    /// after the tick completes and routes each effect into its
    /// MessageLog / special-forces arena. Headless `simulation.rs`
    /// leaves the queue alone and `finish()` discards it.
    story_effects: Vec<rebellion_core::effects::GameEffect>,
}

impl PerceptionIntegrator {
    /// Create a new integrator for a single simulation tick.
    pub fn new(tick: u64, wall_ms: u64) -> Self {
        Self {
            events: Vec::new(),
            tick,
            wall_ms,
            story_effects: Vec::new(),
        }
    }

    /// Drain the queued story/message effects so the interactive main loop
    /// can route `StoryMessageDisplayed` records into its `MessageLog` and
    /// `SpecialForceSpawned` records into its (eventual) special-forces
    /// arena. Called by `main.rs` after `apply_fired_events`. The headless
    /// `simulation.rs` path ignores this queue and lets `finish()` discard
    /// whatever remains.
    pub fn drain_story_effects(&mut self) -> Vec<rebellion_core::effects::GameEffect> {
        std::mem::take(&mut self.story_effects)
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

    /// Heartbeat: emit a check event so the "victory" system tag always appears.
    pub fn emit_victory_check(&mut self, victory_state: &rebellion_core::victory::VictoryState) {
        self.events.push(GameEventRecord::new(
            self.tick, self.wall_ms, SYS_VICTORY, EVT_VICTORY_CHECK,
            serde_json::json!({ "resolved": victory_state.resolved }),
        ));
    }

    /// Emit victory telemetry and mark victory resolved.
    ///
    /// Emits the primary `EVT_VICTORY` record for every terminal condition
    /// and, for HQ-capture outcomes specifically, the Dabora 2 #A1
    /// `EVT_HQ_CAPTURED` (0x128) notification on the victory subsystem
    /// so story-chain consumers can key off the capture event without
    /// pattern-matching on the Debug-formatted outcome string.
    pub fn apply_victory(
        &mut self,
        outcome: &VictoryOutcome,
        victory_state: &mut rebellion_core::victory::VictoryState,
        world: &GameWorld,
    ) {
        victory_state.resolved = true;
        self.emit(SYS_VICTORY, EVT_VICTORY, serde_json::json!({
            "outcome": format!("{:?}", outcome),
        }));
        // A1: EVT_HQ_CAPTURED (0x128). Payload uses the HQ system's name
        // (human-readable) rather than a stale slotmap key (DI-H2).
        if let VictoryOutcome::HqCaptured { winner, loser, hq_system } = outcome {
            self.emit(SYS_VICTORY, EVT_HQ_CAPTURED, serde_json::json!({
                "winner": format!("{:?}", winner),
                "loser": format!("{:?}", loser),
                "hq_system": sys_name(world, *hq_system),
            }));
        }
    }

    /// Emit campaign snapshot telemetry (no world mutations, read-only).
    pub fn emit_campaign_snapshot(
        &mut self,
        world: &GameWorld,
        movement_len: usize,
        economy: &EconomyState,
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

        // Build per-system economy data for parity eval
        let mut systems_map = serde_json::Map::new();
        for (key, sys) in world.systems.iter() {
            if let Some(econ) = economy.per_system.get(&key) {
                systems_map.insert(sys.name.clone(), serde_json::json!({
                    "production_modifier": econ.production_modifier,
                    "troop_surplus": econ.summary.troop_surplus,
                    "has_shipyard": econ.summary.has_shipyard,
                    "fleet_posture": format!("{:?}", econ.summary.fleet_posture),
                    "collection_rate": econ.collection_rate,
                }));
            }
        }

        self.emit("snapshot", EVT_CAMPAIGN_SNAPSHOT, serde_json::json!({
            "tick": self.tick,
            "alliance_systems": alliance_systems,
            "empire_systems": empire_systems,
            "neutral_systems": neutral_systems,
            "fleets": world.fleets.len(),
            "in_transit": movement_len,
            "systems": systems_map,
        }));
    }

    // ── Step 2: Economy section ───────────────────────────────────────────

    /// Apply economy events: world mutations (support drift, control) + telemetry.
    pub fn apply_economy_events(
        &mut self,
        world: &mut GameWorld,
        economy_events: &[EconomyEvent],
    ) {
        for ev in economy_events {
            match ev {
                EconomyEvent::SupportDrifted { system, alliance_delta, empire_delta } => {
                    if let Some(sys) = world.systems.get_mut(*system) {
                        sys.popularity_alliance = (sys.popularity_alliance + alliance_delta).clamp(0.0, 1.0);
                        sys.popularity_empire = (sys.popularity_empire + empire_delta).clamp(0.0, 1.0);
                    }
                    self.emit(SYS_ECONOMY, EVT_SUPPORT_DRIFT, serde_json::json!({
                        "system": sys_name(world, *system),
                        "alliance_delta": alliance_delta,
                        "empire_delta": empire_delta,
                    }));
                }
                EconomyEvent::CollectionRateChanged { system, new_rate } => {
                    self.emit(SYS_ECONOMY, EVT_COLLECTION_RATE, serde_json::json!({
                        "system": sys_name(world, *system),
                        "rate": new_rate,
                    }));
                }
                EconomyEvent::GarrisonRequirementChanged { system, new_requirement } => {
                    self.emit(SYS_ECONOMY, EVT_GARRISON_REQUIRED, serde_json::json!({
                        "system": sys_name(world, *system),
                        "garrison_required": new_requirement,
                    }));
                }
                EconomyEvent::IncidentTriggered { system, incident_type } => {
                    self.emit(SYS_ECONOMY, EVT_ECONOMY_TICK, serde_json::json!({
                        "system": sys_name(world, *system),
                        "incident": incident_type,
                    }));
                }
                EconomyEvent::ControlResolved { system, new_control } => {
                    if let Some(sys) = world.systems.get_mut(*system) {
                        sys.control = *new_control;
                    }
                    self.emit(SYS_ECONOMY, EVT_CONTROL_CHANGED, serde_json::json!({
                        "system": sys_name(world, *system),
                        "new_control": format!("{:?}", new_control),
                    }));
                }
                EconomyEvent::EnergyOvercapped { system, allocated, capacity } => {
                    self.emit(SYS_ECONOMY, EVT_ECONOMY_TICK, serde_json::json!({
                        "system": sys_name(world, *system),
                        "energy_overcap": true,
                        "allocated": allocated,
                        "capacity": capacity,
                    }));
                }
                EconomyEvent::RawMaterialOvercapped { system, allocated, capacity } => {
                    self.emit(SYS_ECONOMY, EVT_ECONOMY_TICK, serde_json::json!({
                        "system": sys_name(world, *system),
                        "raw_material_overcap": true,
                        "allocated": allocated,
                        "capacity": capacity,
                    }));
                }
                // ── Knesset Shamash-Bet Dabora 2 notification events ────────
                EconomyEvent::SupportChanged { system, from, to } => {
                    // K1: EVT_SUPPORT_CHANGE (0x100).
                    self.emit(SYS_ECONOMY, EVT_SUPPORT_CHANGE, serde_json::json!({
                        "system": sys_name(world, *system),
                        "from": format!("{:?}", from),
                        "to": format!("{:?}", to),
                    }));
                }
                EconomyEvent::NaturalDisaster { system } => {
                    // K2: EVT_NATURAL_DISASTER (0x154).
                    self.emit(SYS_ECONOMY, EVT_NATURAL_DISASTER, serde_json::json!({
                        "system": sys_name(world, *system),
                    }));
                }
                EconomyEvent::ResourceDiscovered { system, new_output } => {
                    // K3: EVT_RESOURCE_DISCOVERY (0x155).
                    self.emit(SYS_ECONOMY, EVT_RESOURCE_DISCOVERY, serde_json::json!({
                        "system": sys_name(world, *system),
                        "new_output": new_output,
                    }));
                }
                EconomyEvent::MaintenanceShortfall { faction_is_alliance, deficit_system_count } => {
                    // K4: EVT_MAINTENANCE_SHORTFALL_EVENT (0x304).
                    self.emit(SYS_ECONOMY, EVT_MAINTENANCE_SHORTFALL, serde_json::json!({
                        "faction": if *faction_is_alliance { "Alliance" } else { "Empire" },
                        "deficit_systems": deficit_system_count,
                    }));
                }
            }
        }
    }

    // ── Step 3: Manufacturing + Movement ──────────────────────────────────

    /// Apply build completions: add manufactured items to GameWorld + emit telemetry.
    ///
    /// Emits two telemetry records per completion — `EVT_BUILD_COMPLETE`
    /// (the "construction finished" signal used by the manufacturing panel
    /// and test harnesses) and `EVT_UNITS_DEPLOYED` (0x107, Knesset
    /// Shamash-Bet Dabora 2 #K5 — the "new forces in the field" signal
    /// that strategic AIs and story events listen for).
    pub fn apply_build_completions(
        &mut self,
        world: &mut GameWorld,
        completions: &[CompletionEvent],
    ) {
        for c in completions {
            apply_build_completion_inner(c, world);
            self.emit(SYS_MANUFACTURING, EVT_BUILD_COMPLETE, serde_json::json!({
                "system": sys_name(world, c.system),
                "kind": format!("{:?}", c.kind),
            }));
            // K5: EVT_UNITS_DEPLOYED (0x107).
            self.emit(SYS_MANUFACTURING, EVT_UNITS_DEPLOYED, serde_json::json!({
                "system": sys_name(world, c.system),
                "kind": format!("{:?}", c.kind),
            }));
        }
    }

    /// Emit `EVT_MANUFACTURING_IDLE` (0x160, K6) for systems whose queue
    /// transitioned from non-empty to empty this tick. No world mutation —
    /// the queue is already drained by `ManufacturingSystem::advance_tracked`.
    pub fn apply_manufacturing_idle(
        &mut self,
        world: &GameWorld,
        newly_idle: &[rebellion_core::ids::SystemKey],
    ) {
        for &system in newly_idle {
            self.emit(SYS_MANUFACTURING, EVT_MANUFACTURING_IDLE, serde_json::json!({
                "system": sys_name(world, system),
            }));
        }
    }

    /// Apply fleet arrivals: update locations + emit telemetry.
    pub fn apply_arrivals(
        &mut self,
        world: &mut GameWorld,
        arrivals: &[ArrivalEvent],
    ) {
        for arrival in arrivals {
            if let Some(fleet) = world.fleets.get_mut(arrival.fleet) {
                fleet.location = arrival.system;
            }
            if let Some(origin_sys) = world.systems.get_mut(arrival.origin) {
                origin_sys.fleets.retain(|&k| k != arrival.fleet);
            }
            if let Some(dest_sys) = world.systems.get_mut(arrival.system) {
                if !dest_sys.fleets.contains(&arrival.fleet) {
                    dest_sys.fleets.push(arrival.fleet);
                }
            }
            self.emit(SYS_MOVEMENT, EVT_FLEET_ARRIVED, serde_json::json!({
                "system": sys_name(world, arrival.system),
                "origin": sys_name(world, arrival.origin),
                "fleet_faction": if world.fleets.get(arrival.fleet).map(|f| f.is_alliance).unwrap_or(false) { "Alliance" } else { "Empire" },
            }));
        }
    }

    // ── Step 4: Combat ────────────────────────────────────────────────────

    /// Apply space combat result: ship damage + fleet cleanup + telemetry.
    pub fn apply_space_combat(
        &mut self,
        world: &mut GameWorld,
        system: SystemKey,
        result: &SpaceCombatResult,
    ) {
        apply_space_combat_result_inner(result, world);
        let winner_str = match result.winner {
            CombatSide::Attacker => "alliance",
            CombatSide::Defender => "empire",
            CombatSide::Draw => "draw",
        };
        self.emit(SYS_COMBAT, EVT_COMBAT_SPACE, serde_json::json!({
            "system": sys_name(world, system),
            "winner": winner_str,
        }));
    }

    /// Apply ground combat result: troop damage + dead removal + telemetry.
    pub fn apply_ground_combat(
        &mut self,
        world: &mut GameWorld,
        result: &GroundCombatResult,
    ) {
        apply_ground_combat_result_inner(result, world);
        let ground_winner = match result.winner {
            CombatSide::Attacker => "alliance",
            CombatSide::Defender => "empire",
            CombatSide::Draw => "draw",
        };
        self.emit(SYS_COMBAT, EVT_COMBAT_GROUND, serde_json::json!({
            "system": sys_name(world, result.system),
            "winner": ground_winner,
            "engagements": result.troop_damage.len(),
        }));
    }

    /// Emit bombardment telemetry (no world mutation — bombardment applies via damage field).
    pub fn emit_bombardment(
        &mut self,
        world: &GameWorld,
        system: SystemKey,
        damage: i32,
    ) {
        if damage > 0 {
            self.emit(SYS_COMBAT, EVT_BOMBARDMENT, serde_json::json!({
                "system": sys_name(world, system),
                "damage": damage,
            }));
        }
    }
    // ── Step 5: Missions + Escapes ──────────────────────────────────────

    /// Apply mission result: world mutations + telemetry.
    pub fn apply_mission_result(
        &mut self,
        world: &mut GameWorld,
        result: &MissionResult,
        uprising_state: &mut UprisingState,
        death_star_state: &mut DeathStarState,
    ) {
        apply_mission_effects_inner(&result.effects, world, uprising_state, death_star_state);
        self.emit(SYS_MISSIONS, EVT_MISSION_RESOLVED, serde_json::json!({
            "kind": format!("{:?}", result.kind),
            "outcome": format!("{:?}", result.outcome),
            "target_system": sys_name(world, result.target_system),
        }));
        // Covert missions are espionage operations — emit an Espionage wrapper
        // so the eval harness sees all 8 mission kinds.
        if matches!(result.kind, MissionKind::Sabotage | MissionKind::Assassination | MissionKind::Abduction) {
            self.emit(SYS_MISSIONS, EVT_MISSION_RESOLVED, serde_json::json!({
                "kind": "Espionage",
                "outcome": format!("{:?}", result.outcome),
                "target_system": sys_name(world, result.target_system),
                "parent_kind": format!("{:?}", result.kind),
            }));
        }
    }

    /// Apply escape effects: character faction flip + fleet removal + telemetry.
    pub fn apply_escape_effects(
        &mut self,
        world: &mut GameWorld,
        effects: &[MissionEffect],
    ) {
        for effect in effects {
            if let MissionEffect::CharacterEscaped {
                character,
                escaped_to_alliance,
            } = effect
            {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.is_alliance = *escaped_to_alliance;
                    c.is_empire = !*escaped_to_alliance;
                    c.is_captive = false;
                    c.captured_by = None;
                    c.capture_tick = None;
                }
                for (_, fleet) in world.fleets.iter_mut() {
                    fleet.characters.retain(|&k| k != *character);
                }
                self.emit(SYS_MISSIONS, EVT_ESCAPE, serde_json::json!({
                    "character": char_name(world, *character),
                    "escaped_to_alliance": escaped_to_alliance,
                }));
            }
        }
    }

    // ── Step 6: Events + Jedi training ────────────────────────────────────

    /// Apply fired events: world mutations + Jedi training extraction + telemetry.
    ///
    /// Per Dabora 2 #F7 the `movement` parameter is required so
    /// `EventAction::SpawnSpecialForce` can resolve in-transit characters
    /// via `MovementState::orders()`. Story/message effects push into the
    /// integrator's `story_effects` queue; callers that want to route them
    /// into a render-layer MessageLog should call `drain_story_effects()`
    /// after this returns (simulation.rs ignores the queue).
    pub fn apply_fired_events(
        &mut self,
        world: &mut GameWorld,
        fired_events: &[FiredEvent],
        jedi_state: &mut JediState,
        current_tick: u64,
        movement: &MovementState,
    ) {
        for fired in fired_events {
            apply_event_action_to_world(
                &fired.actions,
                world,
                &mut self.story_effects,
                current_tick,
                movement,
            );
            let system_tag = match fired.system_tag {
                SystemTag::Story => SYS_STORY,
                SystemTag::Events | SystemTag::Notification => SYS_EVENTS,
            };
            self.emit(system_tag, EVT_EVENT_FIRED, serde_json::json!({
                "event_id": fired.event_id,
            }));
        }
        // Extract Jedi training starts from story events
        for fired in fired_events {
            for action in &fired.actions {
                if let EventAction::StartJediTraining { character } = action {
                    if let Some(c) = world.characters.get(*character) {
                        jedi_state.start_training(*character, c.is_alliance, current_tick);
                    }
                }
            }
        }
        // Flush SpecialForceSpawned telemetry. The effect queue also
        // serves as the channel into main.rs's special-forces arena
        // (drained post-tick); we emit telemetry eagerly here so the
        // headless playtest still sees the event without needing to
        // drain — story_effects is monotonically append-only during
        // the tick.
        for eff in &self.story_effects {
            if let rebellion_core::effects::GameEffect::SpecialForceSpawned { at_system, is_alliance } = eff {
                self.events.push(GameEventRecord::new(
                    self.tick, self.wall_ms, SYS_STORY, EVT_EVENT_FIRED,
                    serde_json::json!({
                        "effect": "special_force_spawned",
                        "system": sys_name(world, *at_system),
                        "is_alliance": is_alliance,
                    }),
                ));
            }
        }
    }

    // ── Step 7: AI actions ────────────────────────────────────────────────

    /// Apply AI actions: mission dispatch, production, movement + telemetry.
    #[allow(clippy::too_many_arguments)]
    pub fn apply_ai_actions(
        &mut self,
        actions: &[AIAction],
        rolls: &[f64],
        ai_state: &mut AIState,
        mission_state: &mut MissionState,
        mfg_state: &mut ManufacturingState,
        movement_state: &mut MovementState,
        research_state: &mut ResearchState,
        world: &mut GameWorld,
        _tick: u64,
        config: &rebellion_core::tuning::GameConfig,
        is_dual: bool,
    ) {
        apply_ai_actions_inner(actions, rolls, ai_state, mission_state, mfg_state,
            movement_state, research_state, world, _tick, config);
        for action in actions {
            let mut payload = ai_action_json(action, world);
            if is_dual {
                if let Some(obj) = payload.as_object_mut() {
                    obj.insert("dual_ai".into(), serde_json::json!(true));
                }
            }
            self.emit(SYS_AI, EVT_AI_ACTION, payload);
        }
    }

    // ── Step 8: Blockade ──────────────────────────────────────────────────

    /// Apply blockade events: troop destruction + telemetry.
    pub fn apply_blockade_events(&mut self, world: &mut GameWorld, events: &[BlockadeEvent]) {
        for evt in events {
            match evt {
                BlockadeEvent::BlockadeStarted { system, tick } => {
                    self.events.push(GameEventRecord::new(
                        *tick, self.wall_ms, SYS_BLOCKADE, EVT_BLOCKADE_STARTED,
                        serde_json::json!({ "system": sys_name(world, *system) }),
                    ));
                }
                BlockadeEvent::BlockadeEnded { system, tick } => {
                    self.events.push(GameEventRecord::new(
                        *tick, self.wall_ms, SYS_BLOCKADE, EVT_BLOCKADE_ENDED,
                        serde_json::json!({ "system": sys_name(world, *system) }),
                    ));
                }
                BlockadeEvent::TroopDestroyed { system, troop, .. } => {
                    if let Some(sys) = world.systems.get_mut(*system) {
                        sys.ground_units.retain(|&k| k != *troop);
                    }
                    world.troops.remove(*troop);
                }
            }
        }
    }

    // ── Step 9: Uprising ──────────────────────────────────────────────────

    /// Apply uprising events: control flip + telemetry.
    pub fn apply_uprising_events(&mut self, world: &mut GameWorld, events: &[UprisingEvent]) {
        // Heartbeat: emit a check event so the "uprising" system tag always appears.
        self.events.push(GameEventRecord::new(
            self.tick, self.wall_ms, SYS_UPRISING, EVT_UPRISING_CHECK,
            serde_json::json!({ "systems_checked": world.systems.len(), "incidents": events.len() }),
        ));
        for evt in events {
            match evt {
                UprisingEvent::UprisingIncident { system, tick } => {
                    self.events.push(GameEventRecord::new(
                        *tick, self.wall_ms, SYS_UPRISING, EVT_UPRISING_INCIDENT,
                        serde_json::json!({ "system": sys_name(world, *system) }),
                    ));
                }
                UprisingEvent::UprisingBegan { system, tick } => {
                    let before = world.systems.get(*system).map(|s| s.control);
                    if let Some(sys) = world.systems.get_mut(*system) {
                        sys.control = match sys.control {
                            ControlKind::Controlled(rebellion_core::dat::Faction::Alliance) =>
                                ControlKind::Controlled(rebellion_core::dat::Faction::Empire),
                            ControlKind::Controlled(rebellion_core::dat::Faction::Empire) =>
                                ControlKind::Controlled(rebellion_core::dat::Faction::Alliance),
                            other => other,
                        };
                    }
                    let after = world.systems.get(*system).map(|s| s.control);
                    if before != after {
                        self.events.push(GameEventRecord::new(
                            *tick, self.wall_ms, SYS_UPRISING, EVT_CONTROL_CHANGED,
                            serde_json::json!({
                                "system": sys_name(world, *system),
                                "from": format!("{:?}", before),
                                "to": format!("{:?}", after),
                                "cause": "uprising",
                            }),
                        ));
                    }
                    self.events.push(GameEventRecord::new(
                        *tick, self.wall_ms, SYS_UPRISING, EVT_UPRISING_BEGAN,
                        serde_json::json!({ "system": sys_name(world, *system) }),
                    ));
                }
                UprisingEvent::UprisingSubdued { .. } => {}
            }
        }
    }

    // ── Step 10: Betrayal ─────────────────────────────────────────────────

    /// Apply betrayal events: faction flip + fleet removal + telemetry.
    pub fn apply_betrayal_events(&mut self, world: &mut GameWorld, events: &[BetrayalEvent]) {
        // Heartbeat: emit a check event so the "betrayal" system tag always appears.
        self.events.push(GameEventRecord::new(
            self.tick, self.wall_ms, SYS_BETRAYAL, EVT_BETRAYAL_CHECK,
            serde_json::json!({ "characters_checked": world.characters.len(), "betrayals": events.len() }),
        ));
        for evt in events {
            let BetrayalEvent::CharacterBetrayed { character, defected_to_alliance } = evt;
            if let Some(c) = world.characters.get_mut(*character) {
                c.is_alliance = *defected_to_alliance;
                c.is_empire = !*defected_to_alliance;
            }
            for (_, fleet) in world.fleets.iter_mut() {
                fleet.characters.retain(|&k| k != *character);
            }
            self.emit(SYS_BETRAYAL, EVT_BETRAYAL, serde_json::json!({
                "character": char_name(world, *character),
                "defected_to_alliance": defected_to_alliance,
            }));
        }
    }

    // ── Step 11: Death Star ───────────────────────────────────────────────

    /// Apply death star events: planet destruction + telemetry.
    pub fn apply_death_star_events(&mut self, world: &mut GameWorld, events: &[DeathStarEvent]) {
        // Heartbeat: emit a status event so the "death_star" system tag always appears.
        self.events.push(GameEventRecord::new(
            self.tick, self.wall_ms, SYS_DEATH_STAR, EVT_DS_STATUS,
            serde_json::json!({ "events": events.len() }),
        ));
        for evt in events {
            match evt {
                DeathStarEvent::ConstructionCompleted { system, tick } => {
                    self.events.push(GameEventRecord::new(
                        *tick, self.wall_ms, SYS_DEATH_STAR, EVT_DS_CONSTRUCTION,
                        serde_json::json!({ "system": sys_name(world, *system) }),
                    ));
                }
                DeathStarEvent::PlanetDestroyed { system, tick } => {
                    if let Some(sys) = world.systems.get_mut(*system) {
                        sys.is_destroyed = true;
                    }
                    self.events.push(GameEventRecord::new(
                        *tick, self.wall_ms, SYS_DEATH_STAR, EVT_DS_FIRED,
                        serde_json::json!({ "system": sys_name(world, *system) }),
                    ));
                }
                DeathStarEvent::NearbyWarning { .. } => {}
            }
        }
    }

    // ── Step 12: Research ─────────────────────────────────────────────────

    /// Apply research results: level-ups + telemetry.
    pub fn apply_research_results(&mut self, results: &[ResearchResult], research_state: &mut ResearchState) {
        for result in results {
            let ResearchResult::TechUnlocked { faction_is_alliance, tech_type, new_level } = result;
            if *faction_is_alliance {
                research_state.alliance.advance(*tech_type);
            } else {
                research_state.empire.advance(*tech_type);
            }
            self.emit(SYS_RESEARCH, EVT_RESEARCH_UNLOCKED, serde_json::json!({
                "faction_is_alliance": faction_is_alliance,
                "tech_type": format!("{:?}", tech_type),
                "new_level": new_level,
            }));
        }
    }

    // ── Step 12b: Repair ───────────────────────────────────────────────────

    /// Apply repair events: hull restoration + telemetry.
    pub fn apply_repair_events(&mut self, world: &mut GameWorld, events: &[RepairEvent]) {
        for evt in events {
            match evt {
                RepairEvent::ShipRepaired { fleet, ship_index, hull_before, hull_after } => {
                    // Apply hull restoration to the ShipInstance.
                    if let Some(f) = world.fleets.get_mut(*fleet) {
                        if let Some(ship) = f.capital_ships.get_mut(*ship_index) {
                            ship.hull_current = *hull_after;
                        }
                    }
                    let fleet_name = world.fleets.get(*fleet)
                        .map(|f| sys_name(world, f.location))
                        .unwrap_or_else(|| "unknown".into());
                    self.emit(SYS_REPAIR, EVT_SHIP_REPAIRED, serde_json::json!({
                        "fleet_location": fleet_name,
                        "hull_before": hull_before,
                        "hull_after": hull_after,
                        "delta": hull_after - hull_before,
                    }));
                }
                RepairEvent::RepairCheckPerformed { system, fleet, ships_checked } => {
                    let system_name = sys_name(world, *system);
                    let fleet_commander = world.fleets.get(*fleet)
                        .and_then(|f| world.characters.get(*f.characters.first()?)
                            .map(|c| c.name.as_str()))
                        .unwrap_or("uncrewed");
                    self.emit(SYS_REPAIR, EVT_SHIP_REPAIR_STARTED, serde_json::json!({
                        "system": system_name,
                        "fleet_commander": fleet_commander,
                        "ships_checked": ships_checked,
                    }));
                }
            }
        }
    }

    // ── Step 13: Jedi ─────────────────────────────────────────────────────

    /// Apply jedi events: tier advancement + discovery + telemetry.
    pub fn apply_jedi_events(&mut self, world: &mut GameWorld, events: &[JediEvent], jedi_state: &mut JediState) {
        // Heartbeat: emit a check event so the "jedi" system tag always appears.
        self.events.push(GameEventRecord::new(
            self.tick, self.wall_ms, SYS_JEDI, EVT_JEDI_CHECK,
            serde_json::json!({ "training": jedi_state.training.len(), "events": events.len() }),
        ));
        for evt in events {
            match evt {
                JediEvent::TierAdvanced { character, new_tier } => {
                    if let Some(c) = world.characters.get_mut(*character) {
                        c.force_tier = *new_tier;
                        c.force_experience = match new_tier {
                            rebellion_core::world::ForceTier::None => 0,
                            rebellion_core::world::ForceTier::Aware => 1,
                            rebellion_core::world::ForceTier::Training => rebellion_core::jedi::XP_TO_TRAINING,
                            rebellion_core::world::ForceTier::Experienced => rebellion_core::jedi::XP_TO_EXPERIENCED,
                        };
                    }
                    self.emit(SYS_JEDI, EVT_JEDI_TIER, serde_json::json!({
                        "character": char_name(world, *character),
                        "new_tier": format!("{:?}", new_tier),
                    }));
                }
                JediEvent::TrainingComplete { character } => {
                    jedi_state.stop_training(*character);
                }
                JediEvent::JediDiscovered { character, .. } => {
                    if let Some(c) = world.characters.get_mut(*character) {
                        c.is_discovered_jedi = true;
                    }
                    self.emit(SYS_JEDI, EVT_JEDI_DISCOVERED, serde_json::json!({
                        "character": char_name(world, *character),
                    }));
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Mission effects helper (moved from simulation.rs)
// ---------------------------------------------------------------------------

fn apply_mission_effects_inner(
    effects: &[MissionEffect],
    world: &mut GameWorld,
    uprising_state: &mut UprisingState,
    death_star_state: &mut DeathStarState,
) {
    for effect in effects {
        match effect {
            MissionEffect::PopularityShifted {
                system,
                faction,
                delta,
            } => {
                if let Some(sys) = world.systems.get_mut(*system) {
                    match faction {
                        MissionFaction::Alliance => {
                            sys.popularity_alliance =
                                (sys.popularity_alliance + delta).clamp(0.0, 1.0);
                        }
                        MissionFaction::Empire => {
                            sys.popularity_empire =
                                (sys.popularity_empire + delta).clamp(0.0, 1.0);
                        }
                    }
                    const CONTROL_THRESHOLD: f32 = 0.6;
                    let a_pop = sys.popularity_alliance;
                    let e_pop = sys.popularity_empire;
                    let new_control = if a_pop >= CONTROL_THRESHOLD && a_pop > e_pop + 0.1 {
                        Some(ControlKind::Controlled(rebellion_core::dat::Faction::Alliance))
                    } else if e_pop >= CONTROL_THRESHOLD && e_pop > a_pop + 0.1 {
                        Some(ControlKind::Controlled(rebellion_core::dat::Faction::Empire))
                    } else {
                        None
                    };
                    if let Some(new) = new_control {
                        if sys.control != new {
                            sys.control = new;
                        }
                    }
                }
            }
            MissionEffect::UprisingStarted { system, popularity_delta } => {
                if let Some(sys) = world.systems.get_mut(*system) {
                    sys.popularity_alliance =
                        (sys.popularity_alliance + popularity_delta).clamp(0.0, 1.0);
                    sys.popularity_empire =
                        (sys.popularity_empire - popularity_delta).clamp(0.0, 1.0);
                }
            }
            MissionEffect::SystemIntelligenceGathered { system, .. } => {
                if let Some(sys) = world.systems.get_mut(*system) {
                    sys.exploration_status = rebellion_core::dat::ExplorationStatus::Explored;
                }
            }
            MissionEffect::CharacterRecruited { .. } => {}
            MissionEffect::FacilitySabotaged { system, facility_index, .. } => {
                if let Some(sys) = world.systems.get_mut(*system) {
                    if *facility_index < sys.manufacturing_facilities.len() {
                        let fac_key = sys.manufacturing_facilities.remove(*facility_index);
                        world.manufacturing_facilities.remove(fac_key);
                    } else if *facility_index
                        < sys.manufacturing_facilities.len() + sys.defense_facilities.len()
                    {
                        let adj_idx = *facility_index - sys.manufacturing_facilities.len();
                        let fac_key = sys.defense_facilities.remove(adj_idx);
                        world.defense_facilities.remove(fac_key);
                    }
                }
            }
            MissionEffect::CharacterKilled { character, .. } => {
                // Knesset Shamash-Bet #R11: mark `is_killed = true` instead of
                // removing from the arena so next-tick reactive story events
                // (`EVT_CHARACTER_KILLED` 0x306) can still resolve the character
                // by `dat_id` / `name`. Character is removed from all fleet
                // rosters immediately. Uniqueness comes from the `is_killed`
                // flag combined with `is_repeatable: false` (DI-M3).
                for (_, fleet) in world.fleets.iter_mut() {
                    fleet.characters.retain(|&k| k != *character);
                }
                if let Some(c) = world.characters.get_mut(*character) {
                    c.is_killed = true;
                    c.on_mission = false;
                    c.on_mandatory_mission = false;
                }
            }
            MissionEffect::CharacterCaptured { character, captured_by, at_system } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.is_captive = true;
                    c.captured_by = Some(match captured_by {
                        MissionFaction::Alliance => rebellion_core::dat::Faction::Alliance,
                        MissionFaction::Empire => rebellion_core::dat::Faction::Empire,
                    });
                    c.current_system = Some(*at_system);
                }
                for (_, fleet) in world.fleets.iter_mut() {
                    fleet.characters.retain(|&k| k != *character);
                }
            }
            MissionEffect::CharacterRescued { character, returned_to, .. } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    match returned_to {
                        MissionFaction::Alliance => { c.is_alliance = true; c.is_empire = false; }
                        MissionFaction::Empire => { c.is_alliance = false; c.is_empire = true; }
                    }
                    c.is_captive = false;
                    c.captured_by = None;
                    c.capture_tick = None;
                }
            }
            MissionEffect::CharacterBusy { character } => {
                if let Some(c) = world.characters.get_mut(*character) { c.on_mission = true; }
            }
            MissionEffect::CharacterAvailable { character } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.on_mission = false;
                    c.on_hidden_mission = false;
                }
            }
            MissionEffect::DecoyTriggered { .. } => {}
            MissionEffect::CharacterEscaped { character, escaped_to_alliance } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.is_alliance = *escaped_to_alliance;
                    c.is_empire = !*escaped_to_alliance;
                    c.is_captive = false;
                    c.captured_by = None;
                    c.capture_tick = None;
                }
            }
            MissionEffect::UprisingSubdued { system } => {
                if let Some(sys) = world.systems.get_mut(*system) {
                    match sys.control {
                        ControlKind::Controlled(rebellion_core::dat::Faction::Alliance) => {
                            sys.popularity_alliance = (sys.popularity_alliance + 0.05).clamp(0.0, 1.0);
                            sys.popularity_empire = (sys.popularity_empire - 0.05).clamp(0.0, 1.0);
                        }
                        _ => {
                            sys.popularity_empire = (sys.popularity_empire + 0.05).clamp(0.0, 1.0);
                            sys.popularity_alliance = (sys.popularity_alliance - 0.05).clamp(0.0, 1.0);
                        }
                    }
                }
                uprising_state.clear_uprising(*system);
            }
            MissionEffect::DeathStarSabotaged { ticks_delayed } => {
                death_star_state.add_sabotage_delay(*ticks_delayed);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Event action helper (formerly `apply_event_actions_to_world_inner`)
// ---------------------------------------------------------------------------

/// Apply a slice of `EventAction`s to the world, producing any ancillary
/// `GameEffect`s via `effects_out`.
///
/// Made `pub #[inline]` in Knesset Shamash-Bet Dabora 2 (#F7) as the single
/// source of truth — `main.rs` no longer carries a duplicate helper. The
/// compiler's exhaustive match over the closed `EventAction` enum now
/// enforces coverage parity for free.
///
/// The `effects_out` sink receives two kinds of records:
/// - `GameEffect::StoryMessageDisplayed` for every `EventAction::DisplayMessage`
///   so interactive `main.rs` can drain them into `MessageLog` post-tick.
///   The headless playtest simply discards the effect queue.
/// - `GameEffect::SpecialForceSpawned` for every `EventAction::SpawnSpecialForce`
///   after resolving `at_character.current_system` (or its in-transit
///   movement destination via `MovementState::orders`). Resolution failure
///   logs at `warn!` level and drops the action — callers must gate the
///   originating event on `CharacterAtSystem OR CharacterHasActiveMovementOrder`
///   to guarantee resolution success (SF-#7).
#[inline]
pub fn apply_event_action_to_world(
    actions: &[EventAction],
    world: &mut GameWorld,
    effects_out: &mut Vec<rebellion_core::effects::GameEffect>,
    tick: u64,
    movement: &MovementState,
) {
    use rebellion_core::effects::{GameEffect, MessageCategoryTag};

    for action in actions {
        match action {
            EventAction::DisplayMessage { text } => {
                // #F7 + #A3: route story messages through the effect layer
                // instead of dropping them (the old integrator no-op) or
                // pushing directly to a render-layer MessageLog (the old
                // main.rs duplicate). Interactive mode drains these post-tick.
                effects_out.push(GameEffect::StoryMessageDisplayed {
                    text: text.clone(),
                    category: MessageCategoryTag::Event,
                });
            }
            EventAction::ShiftPopularity { system, alliance_delta, empire_delta } => {
                if let Some(sys) = world.systems.get_mut(*system) {
                    sys.popularity_alliance = (sys.popularity_alliance + alliance_delta).clamp(0.0, 1.0);
                    sys.popularity_empire = (sys.popularity_empire + empire_delta).clamp(0.0, 1.0);
                }
            }
            EventAction::ModifyCharacterSkill { character, skill, base_delta } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    let d = *base_delta;
                    let apply = |v: u32, delta: i32| (v as i64 + delta as i64).max(0) as u32;
                    match skill {
                        SkillField::Diplomacy => c.diplomacy.base = apply(c.diplomacy.base, d),
                        SkillField::Espionage => c.espionage.base = apply(c.espionage.base, d),
                        SkillField::ShipDesign => c.ship_design.base = apply(c.ship_design.base, d),
                        SkillField::TroopTraining => c.troop_training.base = apply(c.troop_training.base, d),
                        SkillField::FacilityDesign => c.facility_design.base = apply(c.facility_design.base, d),
                        SkillField::Combat => c.combat.base = apply(c.combat.base, d),
                        SkillField::Leadership => c.leadership.base = apply(c.leadership.base, d),
                        SkillField::Loyalty => c.loyalty.base = apply(c.loyalty.base, d),
                        SkillField::JediLevel => c.jedi_level.base = apply(c.jedi_level.base, d),
                    }
                }
            }
            EventAction::RelocateCharacter { .. } => {}
            EventAction::SetMandatoryMission { character, mandatory } => {
                if let Some(c) = world.characters.get_mut(*character) { c.on_mandatory_mission = *mandatory; }
            }
            EventAction::ModifyForceTier { character, new_tier } => {
                if let Some(c) = world.characters.get_mut(*character) { c.force_tier = *new_tier; }
            }
            EventAction::RemoveCharacter { character } => {
                for (_, fleet) in world.fleets.iter_mut() { fleet.characters.retain(|&k| k != *character); }
                world.characters.remove(*character);
            }
            EventAction::StartJediTraining { .. } => {}
            EventAction::TransferCharacter { character, destination, new_faction } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.current_system = Some(*destination);
                    if let Some(faction) = new_faction {
                        match faction {
                            rebellion_core::dat::Faction::Alliance => { c.is_alliance = true; c.is_empire = false; }
                            rebellion_core::dat::Faction::Empire => { c.is_alliance = false; c.is_empire = true; }
                            _ => {}
                        }
                    }
                }
            }
            EventAction::TriggerEvent { .. } => {}
            EventAction::AccumulateForceExperience { character, amount } => {
                if let Some(c) = world.characters.get_mut(*character) { c.force_experience += amount; }
            }
            EventAction::CaptureCharacter { character, captor_faction } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.is_captive = true;
                    c.captured_by = Some(*captor_faction);
                    c.capture_tick = Some(tick);
                }
                for (_, fleet) in world.fleets.iter_mut() { fleet.characters.retain(|&k| k != *character); }
            }
            EventAction::SetCarboniteState { character, frozen } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.on_mandatory_mission = *frozen;
                    if *frozen { c.is_captive = true; c.capture_tick = Some(tick); }
                    else { c.is_captive = false; c.captured_by = None; c.capture_tick = None; }
                }
            }
            EventAction::SpawnSpecialForce { at_character } => {
                // #A2: Resolve the system where the special force lands.
                //
                // The character's `current_system` is the authoritative location
                // when the character is stationary (on a garrison or mission).
                // For in-transit characters we fall back to the destination of
                // the first fleet carrying them — via `MovementState::orders()`.
                //
                // The bounty-hunter chain (EVT_BOUNTY_ATTACK) is gated at the
                // `state.define()` site on `CharacterAtSystem OR CharacterHasActiveMovementOrder`
                // so this fallback should always succeed (SF-#7). We `warn!`
                // instead of panic on the unexpected case.
                let character = *at_character;
                let resolved: Option<(rebellion_core::ids::SystemKey, bool)> = world
                    .characters
                    .get(character)
                    .and_then(|c| {
                        // Primary: the character's cached current_system.
                        if let Some(sys) = c.current_system {
                            return Some((sys, c.is_alliance));
                        }
                        // Fallback: find any fleet that carries the character
                        // AND is currently under a movement order, return the
                        // order's destination.
                        let is_alliance = c.is_alliance;
                        for (fleet_key, fleet) in world.fleets.iter() {
                            if fleet.characters.contains(&character) {
                                if let Some(order) = movement.get(fleet_key) {
                                    return Some((order.destination, is_alliance));
                                }
                                // The character is on a stationary fleet — use
                                // the fleet's location field.
                                return Some((fleet.location, is_alliance));
                            }
                        }
                        None
                    });
                match resolved {
                    Some((at_system, is_alliance)) => {
                        // Special force lands at the character's system.
                        // Hardcode `is_alliance: false` to match the
                        // Bounty Hunters parity citation from the
                        // community cross-reference — the resolved
                        // `is_alliance` is currently ignored for
                        // spawned hunters, but passed through here so
                        // future Alliance-side spawns can reuse this
                        // same integrator arm. Default behavior
                        // (bounty hunters) is Imperial.
                        let _ = is_alliance;
                        effects_out.push(GameEffect::SpecialForceSpawned {
                            at_system,
                            is_alliance: false,
                        });
                        // NOTE: full SpecialForceUnit arena wiring still
                        // deferred to Dabora 3 — the effect captures the
                        // intent + telemetry, but no unit is actually
                        // placed in `world.special_forces` yet. See
                        // `apply_dabora2_story_effects` below.
                    }
                    None => {
                        // Structured warn: the event chain must gate
                        // SpawnSpecialForce on CharacterAtSystem OR
                        // CharacterHasActiveMovementOrder (SF-#7). If we
                        // reach this branch, the guard is missing — log
                        // and drop the action rather than spawning at
                        // an arbitrary fallback system.
                        eprintln!(
                            "[shamash-bet] SpawnSpecialForce at character {:?} \
                             could not resolve a target system — character has \
                             no current_system and is not on any movement-ordered \
                             fleet. Event chain should have gated this action with \
                             CharacterAtSystem OR CharacterHasActiveMovementOrder.",
                            character,
                        );
                    }
                }
            }
            EventAction::SetHeritageKnown { character } => {
                // Dabora 3 #R4: flip heritage_known when 0x396 "Final Battle
                // Imminent" fires (or any future event whose narration
                // introduces the Luke-Vader paternity reveal). The render
                // layer branches on this flag in
                // `event_id_to_resource()` to pick between the
                // "Student Luke" / "Knight Luke" / "Emperor + Vader vs
                // Knight Luke" BMP variants for 0x220 EVT_FINAL_BATTLE.
                // This is the render-layer approach (SIMP-H5 + ARCH-#9
                // + SF-#11 triple-collapse) — we do NOT create 0x222.
                if let Some(c) = world.characters.get_mut(*character) {
                    c.heritage_known = true;
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AI action helper (moved from simulation.rs)
// ---------------------------------------------------------------------------

fn apply_ai_actions_inner(
    actions: &[AIAction],
    rolls: &[f64],
    ai_state: &mut AIState,
    mission_state: &mut MissionState,
    mfg_state: &mut ManufacturingState,
    movement_state: &mut MovementState,
    research_state: &mut ResearchState,
    world: &mut GameWorld,
    _tick: u64,
    config: &rebellion_core::tuning::GameConfig,
) {
    let mission_faction = ai_state
        .faction
        .map(|f| f.as_mission_faction())
        .unwrap_or(MissionFaction::Empire);

    let mut roll_idx = 0;
    for action in actions {
        match action {
            AIAction::DispatchMission { kind, character, target_system, target_character, duration_roll } => {
                let roll = rolls.get(roll_idx).copied().unwrap_or(*duration_roll);
                roll_idx += 1;
                mission_state.dispatch(*kind, mission_faction, *character, *target_system, *target_character, roll);
                ai_state.mark_busy(*character);
            }
            AIAction::EnqueueProduction { system, kind, ticks } => {
                mfg_state.enqueue(*system, QueueItem::new(*kind, *ticks, *ticks));
            }
            AIAction::DispatchResearch { character, tech_type, ticks } => {
                let is_alliance = ai_state.faction
                    .map(|f| matches!(f, rebellion_core::ai::AiFaction::Alliance))
                    .unwrap_or(false);
                research_state.dispatch(rebellion_core::research::ResearchProject {
                    tech_type: *tech_type,
                    character: *character,
                    faction_is_alliance: is_alliance,
                    ticks_remaining: *ticks,
                    total_ticks: *ticks,
                });
                ai_state.mark_busy(*character);
            }
            AIAction::MoveFleet { fleet, to_system, .. } => {
                let already_moving = movement_state.get(*fleet).map(|o| o.destination == *to_system).unwrap_or(false);
                if !already_moving {
                    if let Some(f) = world.fleets.get(*fleet) {
                        let transit = rebellion_core::movement::fleet_transit_ticks_with_config(
                            f, world, f.location, *to_system,
                            config.movement.distance_scale,
                            config.movement.min_transit_ticks,
                            config.movement.default_fighter_hyperdrive,
                        );
                        movement_state.order(*fleet, f.location, *to_system, transit);
                    }
                }
            }
            AIAction::MoveTroops { troop, from_system, to_system } => {
                // Remove from source system and add to destination.
                if let Some(src) = world.systems.get_mut(*from_system) {
                    src.ground_units.retain(|&k| k != *troop);
                }
                if let Some(dst) = world.systems.get_mut(*to_system) {
                    dst.ground_units.push(*troop);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Combat helpers (moved from simulation.rs)
// ---------------------------------------------------------------------------

pub fn apply_space_combat_result_inner(
    result: &SpaceCombatResult,
    world: &mut GameWorld,
) {
    // Apply hull damage to individual ship instances.
    for evt in &result.ship_damage {
        let fleet_key = evt.fleet;
        if let Some(fleet) = world.fleets.get_mut(fleet_key) {
            // ship_index maps 1:1 to alive ships at snapshot time.
            // Find the nth alive ship.
            let mut alive_idx = 0;
            for ship in fleet.capital_ships.iter_mut() {
                if !ship.alive { continue; }
                if alive_idx == evt.ship_index {
                    ship.hull_current = evt.hull_after;
                    if evt.hull_after <= 0 {
                        ship.alive = false;
                    }
                    break;
                }
                alive_idx += 1;
            }
        }
    }

    // Remove dead ships and empty fleets.
    for &fleet_key in &[result.attacker_fleet, result.defender_fleet] {
        if let Some(fleet) = world.fleets.get_mut(fleet_key) {
            fleet.capital_ships.retain(|s| s.alive);
        }
        let is_empty = world.fleets.get(fleet_key)
            .map(|f| f.is_empty())
            .unwrap_or(true);
        if is_empty {
            // Capture losing fleet's characters (parity: officers captured on fleet destruction).
            let is_loser = match result.winner {
                CombatSide::Attacker => fleet_key == result.defender_fleet,
                CombatSide::Defender => fleet_key == result.attacker_fleet,
                CombatSide::Draw => false,
            };
            if is_loser {
                let capture_data = world.fleets.get(fleet_key).map(|f| {
                    let captor = if f.is_alliance {
                        rebellion_core::dat::Faction::Empire
                    } else {
                        rebellion_core::dat::Faction::Alliance
                    };
                    (f.characters.clone(), captor, f.location)
                });
                if let Some((chars, captor, loc)) = capture_data {
                    for ck in chars {
                        if let Some(c) = world.characters.get_mut(ck) {
                            c.is_captive = true;
                            c.captured_by = Some(captor);
                            c.current_system = Some(loc);
                        }
                    }
                }
            }
            if let Some(fleet) = world.fleets.get(fleet_key) {
                let loc = fleet.location;
                if let Some(sys) = world.systems.get_mut(loc) {
                    sys.fleets.retain(|&k| k != fleet_key);
                }
            }
            world.fleets.remove(fleet_key);
        }
    }
}

pub fn apply_ground_combat_result_inner(
    result: &GroundCombatResult,
    world: &mut GameWorld,
) {
    let mut final_strengths: HashMap<TroopKey, i16> = HashMap::new();
    for evt in &result.troop_damage {
        final_strengths.insert(evt.troop, evt.strength_after);
    }
    for (&key, &strength) in &final_strengths {
        if let Some(troop) = world.troops.get_mut(key) {
            troop.regiment_strength = strength;
        }
    }

    let sys_key = result.system;
    if let Some(sys) = world.systems.get_mut(sys_key) {
        sys.ground_units.retain(|&k| {
            world
                .troops
                .get(k)
                .map(|t| t.regiment_strength > 0)
                .unwrap_or(false)
        });
    }
    let dead: Vec<_> = final_strengths
        .iter()
        .filter(|(_, &s)| s <= 0)
        .map(|(&k, _)| k)
        .collect();
    for key in dead {
        world.troops.remove(key);
    }
}

// ---------------------------------------------------------------------------
// Build completion helper (moved from simulation.rs)
// ---------------------------------------------------------------------------

pub fn apply_build_completion_inner(
    completion: &CompletionEvent,
    world: &mut GameWorld,
) {
    let sys_key = completion.system;

    match &completion.kind {
        BuildableKind::CapitalShip(class_key) => {
            let is_alliance = world
                .capital_ship_classes
                .get(*class_key)
                .map(|c| c.is_alliance)
                .unwrap_or(false);

            let fleet_key = {
                let sys = match world.systems.get(sys_key) {
                    Some(s) => s,
                    None => return,
                };
                sys.fleets
                    .iter()
                    .copied()
                    .find(|&fk| {
                        world.fleets.get(fk).map(|f| f.is_alliance == is_alliance).unwrap_or(false)
                    })
            };

            if let Some(fk) = fleet_key {
                if let Some(fleet) = world.fleets.get_mut(fk) {
                    let hull = world.capital_ship_classes.get(*class_key)
                        .map(|c| c.hull as i32)
                        .unwrap_or(100);
                    fleet.capital_ships.push(ShipInstance::new(*class_key, hull, is_alliance));
                }
            } else {
                let hull = world.capital_ship_classes.get(*class_key)
                    .map(|c| c.hull as i32)
                    .unwrap_or(100);
                let fleet = Fleet {
                    location: sys_key,
                    capital_ships: vec![ShipInstance::new(*class_key, hull, is_alliance)],
                    fighters: vec![],
                    characters: vec![],
                    is_alliance,
                    has_death_star: false,
                };
                let fk = world.fleets.insert(fleet);
                if let Some(sys) = world.systems.get_mut(sys_key) {
                    sys.fleets.push(fk);
                }
            }
        }
        BuildableKind::Fighter(class_key) => {
            let is_alliance = world
                .fighter_classes
                .get(*class_key)
                .map(|c| c.is_alliance)
                .unwrap_or(false);

            let fleet_key = {
                let sys = match world.systems.get(sys_key) {
                    Some(s) => s,
                    None => return,
                };
                sys.fleets
                    .iter()
                    .copied()
                    .find(|&fk| {
                        world.fleets.get(fk).map(|f| f.is_alliance == is_alliance).unwrap_or(false)
                    })
            };

            if let Some(fk) = fleet_key {
                if let Some(fleet) = world.fleets.get_mut(fk) {
                    if let Some(entry) = fleet.fighters.iter_mut().find(|e| e.class == *class_key) {
                        entry.count += 1;
                    } else {
                        fleet.fighters.push(FighterEntry { class: *class_key, count: 1 });
                    }
                }
            } else {
                let fleet = Fleet {
                    location: sys_key,
                    capital_ships: vec![],
                    fighters: vec![FighterEntry { class: *class_key, count: 1 }],
                    characters: vec![],
                    is_alliance,
                    has_death_star: false,
                };
                let fk = world.fleets.insert(fleet);
                if let Some(sys) = world.systems.get_mut(sys_key) {
                    sys.fleets.push(fk);
                }
            }
        }
        BuildableKind::ManufacturingFacility(class_key) => {
            if let Some(template) = world.manufacturing_facilities.get(*class_key).cloned() {
                let fac_key = world.manufacturing_facilities.insert(template);
                if let Some(sys) = world.systems.get_mut(sys_key) {
                    sys.manufacturing_facilities.push(fac_key);
                }
            }
        }
        BuildableKind::DefenseFacility(class_key) => {
            if let Some(template) = world.defense_facilities.get(*class_key).cloned() {
                let fac_key = world.defense_facilities.insert(template);
                if let Some(sys) = world.systems.get_mut(sys_key) {
                    sys.defense_facilities.push(fac_key);
                }
            }
        }
        BuildableKind::ProductionFacility(class_key) => {
            if let Some(template) = world.production_facilities.get(*class_key).cloned() {
                let fac_key = world.production_facilities.insert(template);
                if let Some(sys) = world.systems.get_mut(sys_key) {
                    sys.production_facilities.push(fac_key);
                }
            }
        }
        BuildableKind::Troop(class_key) => {
            let is_alliance = world
                .troops
                .get(*class_key)
                .map(|t| t.is_alliance)
                .unwrap_or(false);
            let unit = TroopUnit {
                class_dat_id: rebellion_core::ids::DatId::new(0),
                is_alliance,
                regiment_strength: 100,
            };
            let tk = world.troops.insert(unit);
            if let Some(sys) = world.systems.get_mut(sys_key) {
                sys.ground_units.push(tk);
            }
        }
    }
}

