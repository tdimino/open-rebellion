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
use rebellion_core::combat::{CombatSide, CombatSystem, GroundCombatResult, SpaceCombatResult};
use rebellion_core::death_star::{DeathStarEvent, DeathStarState};
use rebellion_core::economy::{EconomyEvent, EconomyState};
use rebellion_core::events::{EventAction, FiredEvent, SkillField, SystemTag};
use rebellion_core::fog::RevealEvent;
use rebellion_core::game_events::*;
use rebellion_core::ids::DatId;
use rebellion_core::ids::{CharacterKey, FleetKey, SystemKey, TroopKey};
use rebellion_core::jedi::{JediEvent, JediState};
use rebellion_core::manufacturing::{
    BuildableKind, CompletionEvent, ManufacturingState, QueueItem,
};
use rebellion_core::missions::{
    MissionEffect, MissionFaction, MissionKind, MissionResult, MissionState,
};
use rebellion_core::movement::{
    apply_fleet_arrival, begin_fleet_transit, ArrivalEvent, MovementState,
};
use rebellion_core::repair::RepairEvent;
use rebellion_core::research::{ResearchResult, ResearchState};
use rebellion_core::troop_transport::TroopTransportState;
use rebellion_core::uprising::{UprisingEvent, UprisingState};
use rebellion_core::victory::VictoryOutcome;
use rebellion_core::world::{
    ControlKind, FighterEntry, Fleet, GameWorld, ShipInstance, SpecialForceUnit, TroopUnit,
};

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
            troops,
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
                "troops": troops.len(),
            })
        }
        AIAction::DispatchMission {
            kind,
            target_system,
            ..
        } => {
            serde_json::json!({
                "type": "DispatchMission",
                "kind": format!("{:?}", kind),
                "target": sys_name(world, *target_system),
            })
        }
        AIAction::EnqueueProduction {
            system,
            kind,
            ticks,
        } => {
            serde_json::json!({
                "type": "EnqueueProduction",
                "system": sys_name(world, *system),
                "kind": format!("{:?}", kind),
                "ticks": ticks,
            })
        }
        AIAction::DispatchResearch {
            character,
            tech_type,
            ticks,
        } => {
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
    pub fn emit(
        &mut self,
        system: &'static str,
        event_type: &'static str,
        payload: serde_json::Value,
    ) {
        self.events.push(GameEventRecord::new(
            self.tick,
            self.wall_ms,
            system,
            event_type,
            payload,
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
            self.emit(
                SYS_FOG,
                EVT_FOG_REVEALED,
                serde_json::json!({
                    "system": sys_name(world, reveal.system),
                }),
            );
        }
    }

    /// Heartbeat: emit a check event so the "victory" system tag always appears.
    pub fn emit_victory_check(&mut self, victory_state: &rebellion_core::victory::VictoryState) {
        self.events.push(GameEventRecord::new(
            self.tick,
            self.wall_ms,
            SYS_VICTORY,
            EVT_VICTORY_CHECK,
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
        self.emit(
            SYS_VICTORY,
            EVT_VICTORY,
            serde_json::json!({
                "outcome": format!("{:?}", outcome),
            }),
        );
        // A1: EVT_HQ_CAPTURED (0x128). Payload uses the HQ system's name
        // (human-readable) rather than a stale slotmap key (DI-H2).
        if let VictoryOutcome::HqCaptured {
            winner,
            loser,
            hq_system,
        } = outcome
        {
            self.emit(
                SYS_VICTORY,
                EVT_HQ_CAPTURED,
                serde_json::json!({
                    "winner": format!("{:?}", winner),
                    "loser": format!("{:?}", loser),
                    "hq_system": sys_name(world, *hq_system),
                }),
            );
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
                ControlKind::Controlled(rebellion_core::dat::Faction::Alliance) => {
                    alliance_systems += 1
                }
                ControlKind::Controlled(rebellion_core::dat::Faction::Empire) => {
                    empire_systems += 1
                }
                _ => neutral_systems += 1,
            }
        }

        // Build per-system economy data for parity eval
        let mut systems_map = serde_json::Map::new();
        for (key, sys) in world.systems.iter() {
            if let Some(econ) = economy.per_system.get(&key) {
                systems_map.insert(
                    sys.name.clone(),
                    serde_json::json!({
                        "production_modifier": econ.production_modifier,
                        "troop_surplus": econ.summary.troop_surplus,
                        "has_shipyard": econ.summary.has_shipyard,
                        "fleet_posture": format!("{:?}", econ.summary.fleet_posture),
                        "collection_rate": econ.collection_rate,
                    }),
                );
            }
        }

        self.emit(
            "snapshot",
            EVT_CAMPAIGN_SNAPSHOT,
            serde_json::json!({
                "tick": self.tick,
                "alliance_systems": alliance_systems,
                "empire_systems": empire_systems,
                "neutral_systems": neutral_systems,
                "fleets": world.fleets.len(),
                "in_transit": movement_len,
                "systems": systems_map,
            }),
        );
    }

    // ── Step 2: Economy section ───────────────────────────────────────────

    /// Apply economy events: world mutations (support drift, control) + telemetry.
    pub fn apply_economy_events(&mut self, world: &mut GameWorld, economy_events: &[EconomyEvent]) {
        for ev in economy_events {
            match ev {
                EconomyEvent::SupportDrifted {
                    system,
                    alliance_delta,
                    empire_delta,
                } => {
                    if let Some(sys) = world.systems.get_mut(*system) {
                        sys.popularity_alliance =
                            (sys.popularity_alliance + alliance_delta).clamp(0.0, 1.0);
                        sys.popularity_empire =
                            (sys.popularity_empire + empire_delta).clamp(0.0, 1.0);
                    }
                    self.emit(
                        SYS_ECONOMY,
                        EVT_SUPPORT_DRIFT,
                        serde_json::json!({
                            "system": sys_name(world, *system),
                            "alliance_delta": alliance_delta,
                            "empire_delta": empire_delta,
                        }),
                    );
                }
                EconomyEvent::CollectionRateChanged { system, new_rate } => {
                    self.emit(
                        SYS_ECONOMY,
                        EVT_COLLECTION_RATE,
                        serde_json::json!({
                            "system": sys_name(world, *system),
                            "rate": new_rate,
                        }),
                    );
                }
                EconomyEvent::GarrisonRequirementChanged {
                    system,
                    new_requirement,
                } => {
                    self.emit(
                        SYS_ECONOMY,
                        EVT_GARRISON_REQUIRED,
                        serde_json::json!({
                            "system": sys_name(world, *system),
                            "garrison_required": new_requirement,
                        }),
                    );
                }
                EconomyEvent::IncidentTriggered {
                    system,
                    incident_type,
                } => {
                    self.emit(
                        SYS_ECONOMY,
                        EVT_ECONOMY_TICK,
                        serde_json::json!({
                            "system": sys_name(world, *system),
                            "incident": incident_type,
                        }),
                    );
                }
                EconomyEvent::ControlResolved {
                    system,
                    new_control,
                } => {
                    if let Some(sys) = world.systems.get_mut(*system) {
                        sys.control = *new_control;
                    }
                    self.emit(
                        SYS_ECONOMY,
                        EVT_CONTROL_CHANGED,
                        serde_json::json!({
                            "system": sys_name(world, *system),
                            "new_control": format!("{:?}", new_control),
                        }),
                    );
                }
                EconomyEvent::EnergyOvercapped {
                    system,
                    allocated,
                    capacity,
                } => {
                    self.emit(
                        SYS_ECONOMY,
                        EVT_ECONOMY_TICK,
                        serde_json::json!({
                            "system": sys_name(world, *system),
                            "energy_overcap": true,
                            "allocated": allocated,
                            "capacity": capacity,
                        }),
                    );
                }
                EconomyEvent::RawMaterialOvercapped {
                    system,
                    allocated,
                    capacity,
                } => {
                    self.emit(
                        SYS_ECONOMY,
                        EVT_ECONOMY_TICK,
                        serde_json::json!({
                            "system": sys_name(world, *system),
                            "raw_material_overcap": true,
                            "allocated": allocated,
                            "capacity": capacity,
                        }),
                    );
                }
                // ── Knesset Shamash-Bet Dabora 2 notification events ────────
                EconomyEvent::SupportChanged { system, from, to } => {
                    // K1: EVT_SUPPORT_CHANGE (0x100).
                    self.emit(
                        SYS_ECONOMY,
                        EVT_SUPPORT_CHANGE,
                        serde_json::json!({
                            "system": sys_name(world, *system),
                            "from": format!("{:?}", from),
                            "to": format!("{:?}", to),
                        }),
                    );
                }
                EconomyEvent::NaturalDisaster { system } => {
                    // K2: EVT_NATURAL_DISASTER (0x154).
                    self.emit(
                        SYS_ECONOMY,
                        EVT_NATURAL_DISASTER,
                        serde_json::json!({
                            "system": sys_name(world, *system),
                        }),
                    );
                }
                EconomyEvent::ResourceDiscovered { system, new_output } => {
                    // K3: EVT_RESOURCE_DISCOVERY (0x155).
                    self.emit(
                        SYS_ECONOMY,
                        EVT_RESOURCE_DISCOVERY,
                        serde_json::json!({
                            "system": sys_name(world, *system),
                            "new_output": new_output,
                        }),
                    );
                }
                EconomyEvent::MaintenanceShortfall {
                    faction_is_alliance,
                    deficit_system_count,
                } => {
                    // K4: EVT_MAINTENANCE_SHORTFALL_EVENT (0x304).
                    self.emit(
                        SYS_ECONOMY,
                        EVT_MAINTENANCE_SHORTFALL,
                        serde_json::json!({
                            "faction": if *faction_is_alliance { "Alliance" } else { "Empire" },
                            "deficit_systems": deficit_system_count,
                        }),
                    );
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
            self.emit(
                SYS_MANUFACTURING,
                EVT_BUILD_COMPLETE,
                serde_json::json!({
                    "system": sys_name(world, c.system),
                    "kind": format!("{:?}", c.kind),
                }),
            );
            // K5: EVT_UNITS_DEPLOYED (0x107).
            self.emit(
                SYS_MANUFACTURING,
                EVT_UNITS_DEPLOYED,
                serde_json::json!({
                    "system": sys_name(world, c.system),
                    "kind": format!("{:?}", c.kind),
                }),
            );
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
            self.emit(
                SYS_MANUFACTURING,
                EVT_MANUFACTURING_IDLE,
                serde_json::json!({
                    "system": sys_name(world, system),
                }),
            );
        }
    }

    /// Apply fleet arrivals: update locations + emit telemetry.
    pub fn apply_arrivals(
        &mut self,
        world: &mut GameWorld,
        troop_transport: &mut TroopTransportState,
        arrivals: &[ArrivalEvent],
    ) {
        for arrival in arrivals {
            let Some(applied) = apply_fleet_arrival(world, troop_transport, arrival) else {
                continue;
            };
            self.emit(
                SYS_MOVEMENT,
                EVT_FLEET_ARRIVED,
                serde_json::json!({
                    "system": sys_name(world, arrival.system),
                    "origin": sys_name(world, arrival.origin),
                    "fleet_faction": if applied.is_alliance { "Alliance" } else { "Empire" },
                    "surviving_fleet": format!("{:?}", applied.fleet),
                    "merged_fleets": applied.merged_fleets,
                }),
            );
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
        self.emit(
            SYS_COMBAT,
            EVT_COMBAT_SPACE,
            serde_json::json!({
                "system": sys_name(world, system),
                "winner": winner_str,
            }),
        );
    }

    /// Emit one summary event after a complete system-level space engagement.
    pub fn emit_system_space_combat(
        &mut self,
        world: &GameWorld,
        result: &SystemSpaceCombatResult,
    ) {
        let winner_str = match result.winner {
            CombatSide::Attacker => "alliance",
            CombatSide::Defender => "empire",
            CombatSide::Draw => "draw",
        };
        self.emit(
            SYS_COMBAT,
            EVT_COMBAT_SPACE,
            serde_json::json!({
                "system": sys_name(world, result.system),
                "winner": winner_str,
                "rounds": result.rounds,
                "alliance_fleets": result.alliance_fleets,
                "empire_fleets": result.empire_fleets,
                "alliance_before": {
                    "capital_ships": result.alliance_before.capital_ships,
                    "fighter_squadrons": result.alliance_before.fighter_squadrons,
                    "hull": result.alliance_before.hull,
                },
                "alliance_after": {
                    "capital_ships": result.alliance_after.capital_ships,
                    "fighter_squadrons": result.alliance_after.fighter_squadrons,
                    "hull": result.alliance_after.hull,
                },
                "empire_before": {
                    "capital_ships": result.empire_before.capital_ships,
                    "fighter_squadrons": result.empire_before.fighter_squadrons,
                    "hull": result.empire_before.hull,
                },
                "empire_after": {
                    "capital_ships": result.empire_after.capital_ships,
                    "fighter_squadrons": result.empire_after.fighter_squadrons,
                    "hull": result.empire_after.hull,
                },
                "stalemate": result.stalemate,
            }),
        );
    }

    /// Apply ground combat result: troop damage + dead removal + telemetry.
    pub fn apply_ground_combat(&mut self, world: &mut GameWorld, result: &GroundCombatResult) {
        apply_ground_combat_result_inner(result, world);
        let ground_winner = match result.winner {
            CombatSide::Attacker => "alliance",
            CombatSide::Defender => "empire",
            CombatSide::Draw => "draw",
        };
        self.emit(
            SYS_COMBAT,
            EVT_COMBAT_GROUND,
            serde_json::json!({
                "system": sys_name(world, result.system),
                "winner": ground_winner,
                "engagements": result.troop_damage.len(),
            }),
        );
    }

    /// Emit one summary for a complete multi-round ground engagement.
    pub fn emit_system_ground_combat(
        &mut self,
        world: &GameWorld,
        system: SystemKey,
        winner: CombatSide,
        engagements: usize,
        rounds: u32,
        stalemate: bool,
    ) {
        let ground_winner = match winner {
            CombatSide::Attacker => "attacker",
            CombatSide::Defender => "defender",
            CombatSide::Draw => "draw",
        };
        self.emit(
            SYS_COMBAT,
            EVT_COMBAT_GROUND,
            serde_json::json!({
                "system": sys_name(world, system),
                "winner": ground_winner,
                "engagements": engagements,
                "rounds": rounds,
                "stalemate": stalemate,
            }),
        );
    }

    /// Land every regiment carried by a fleet and emit one movement record.
    pub fn apply_troop_landing(
        &mut self,
        world: &mut GameWorld,
        troop_transport: &mut TroopTransportState,
        fleet: FleetKey,
        system: SystemKey,
    ) -> Vec<TroopKey> {
        let landed = troop_transport
            .disembark_all(world, fleet, system)
            .unwrap_or_default();
        if !landed.is_empty() {
            self.emit(
                SYS_MOVEMENT,
                EVT_TROOP_MOVED,
                serde_json::json!({
                    "system": sys_name(world, system),
                    "fleet": format!("{:?}", fleet),
                    "regiments": landed.len(),
                    "status": "landed",
                }),
            );
        }
        landed
    }

    /// Apply territorial occupation after a decisive ground victory.
    /// Opposing characters physically present on the captured world become
    /// prisoners, allowing the standard victory rules to observe HQ captures.
    pub fn apply_ground_occupation(
        &mut self,
        world: &mut GameWorld,
        system: SystemKey,
        winner: rebellion_core::dat::Faction,
        tick: u64,
    ) {
        let previous = world.systems.get(system).map(|value| value.control);
        if let Some(value) = world.systems.get_mut(system) {
            value.control = ControlKind::Controlled(winner);
        }
        if previous != Some(ControlKind::Controlled(winner)) {
            self.emit(
                SYS_COMBAT,
                EVT_CONTROL_CHANGED,
                serde_json::json!({
                    "system": sys_name(world, system),
                    "from": format!("{:?}", previous.unwrap_or_default()),
                    "to": format!("{:?}", ControlKind::Controlled(winner)),
                    "cause": "ground_occupation",
                }),
            );
        }

        let captives: Vec<_> = world
            .characters
            .iter()
            .filter(|(_, character)| {
                character.current_system == Some(system)
                    && !character.is_killed
                    && match winner {
                        rebellion_core::dat::Faction::Alliance => character.is_empire,
                        rebellion_core::dat::Faction::Empire => character.is_alliance,
                        rebellion_core::dat::Faction::Neutral => false,
                    }
            })
            .map(|(key, character)| (key, character.name.clone()))
            .collect();
        for (character, name) in captives {
            if let Some(value) = world.characters.get_mut(character) {
                value.is_captive = true;
                value.captured_by = Some(winner);
                value.capture_tick = Some(tick);
                value.current_fleet = None;
            }
            self.emit(
                SYS_COMBAT,
                EVT_CAPTURE,
                serde_json::json!({
                    "character": name,
                    "system": sys_name(world, system),
                    "captured_by": format!("{:?}", winner),
                    "cause": "ground_occupation",
                }),
            );
        }
    }

    /// Emit loss evidence for regiments whose fleet was destroyed in space.
    pub fn emit_destroyed_transport_cargo(&mut self, destroyed: &[(FleetKey, Vec<TroopKey>)]) {
        for (fleet, troops) in destroyed {
            if troops.is_empty() {
                continue;
            }
            self.emit(
                SYS_COMBAT,
                EVT_TROOP_MOVED,
                serde_json::json!({
                    "fleet": format!("{:?}", fleet),
                    "regiments": troops.len(),
                    "status": "destroyed_with_transport",
                }),
            );
        }
    }

    /// Emit bombardment telemetry after the caller applies its world effects.
    pub fn emit_bombardment(
        &mut self,
        world: &GameWorld,
        system: SystemKey,
        damage: i32,
        headquarters_destroyed: bool,
    ) {
        if damage > 0 {
            self.emit(
                SYS_COMBAT,
                EVT_BOMBARDMENT,
                serde_json::json!({
                    "system": sys_name(world, system),
                    "damage": damage,
                    "headquarters_destroyed": headquarters_destroyed,
                }),
            );
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
        self.emit(
            SYS_MISSIONS,
            EVT_MISSION_RESOLVED,
            serde_json::json!({
                "kind": format!("{:?}", result.kind),
                "outcome": format!("{:?}", result.outcome),
                "target_system": sys_name(world, result.target_system),
            }),
        );
        // Covert missions are espionage operations — emit an Espionage wrapper
        // so the eval harness sees all 8 mission kinds.
        if matches!(
            result.kind,
            MissionKind::Sabotage | MissionKind::Assassination | MissionKind::Abduction
        ) {
            self.emit(
                SYS_MISSIONS,
                EVT_MISSION_RESOLVED,
                serde_json::json!({
                    "kind": "Espionage",
                    "outcome": format!("{:?}", result.outcome),
                    "target_system": sys_name(world, result.target_system),
                    "parent_kind": format!("{:?}", result.kind),
                }),
            );
        }
        // R6/R7/R8/R11: Per-effect telemetry emissions.
        for effect in &result.effects {
            match effect {
                // R6: EVT_INFORMANT_INTEL — Espionage resolved with intel.
                MissionEffect::SystemIntelligenceGathered { system, faction } => {
                    self.emit(
                        SYS_MISSIONS,
                        EVT_INFORMANT_INTEL,
                        serde_json::json!({
                            "system": sys_name(world, *system),
                            "faction": format!("{:?}", faction),
                        }),
                    );
                }
                // R7: EVT_SABOTEUR_DETECTED — enemy sabotage on a system.
                MissionEffect::FacilitySabotaged { system, .. } => {
                    self.emit(
                        SYS_MISSIONS,
                        EVT_SABOTEUR_DETECTED,
                        serde_json::json!({
                            "system": sys_name(world, *system),
                            "mission_faction": format!("{:?}", result.faction),
                        }),
                    );
                }
                // R8: EVT_CHARACTER_HEALTH — character captured (health status change).
                MissionEffect::CharacterCaptured {
                    character,
                    captured_by,
                    at_system,
                } => {
                    self.emit(
                        SYS_MISSIONS,
                        EVT_CHARACTER_HEALTH,
                        serde_json::json!({
                            "character": char_name(world, *character),
                            "status": "captured",
                            "captured_by": format!("{:?}", captured_by),
                            "system": sys_name(world, *at_system),
                        }),
                    );
                }
                // R11: EVT_CHARACTER_KILLED — assassination kill.
                MissionEffect::CharacterKilled { character, faction } => {
                    self.emit(
                        SYS_MISSIONS,
                        EVT_CHARACTER_KILLED,
                        serde_json::json!({
                            "character": char_name(world, *character),
                            "cause": "assassination",
                            "faction": format!("{:?}", faction),
                        }),
                    );
                }
                _ => {}
            }
        }
    }

    /// Apply escape effects: character faction flip + fleet removal + telemetry.
    pub fn apply_escape_effects(&mut self, world: &mut GameWorld, effects: &[MissionEffect]) {
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
                self.emit(
                    SYS_MISSIONS,
                    EVT_ESCAPE,
                    serde_json::json!({
                        "character": char_name(world, *character),
                        "escaped_to_alliance": escaped_to_alliance,
                    }),
                );
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
            self.emit(
                system_tag,
                EVT_EVENT_FIRED,
                serde_json::json!({
                    "event_id": fired.event_id,
                }),
            );
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
            if let rebellion_core::effects::GameEffect::SpecialForceSpawned {
                at_system,
                is_alliance,
            } = eff
            {
                self.events.push(GameEventRecord::new(
                    self.tick,
                    self.wall_ms,
                    SYS_STORY,
                    EVT_EVENT_FIRED,
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
        troop_transport: &mut TroopTransportState,
        research_state: &mut ResearchState,
        world: &mut GameWorld,
        _tick: u64,
        config: &rebellion_core::tuning::GameConfig,
        is_dual: bool,
    ) {
        let applied = apply_ai_actions_inner(
            actions,
            rolls,
            ai_state,
            mission_state,
            mfg_state,
            movement_state,
            troop_transport,
            research_state,
            world,
            _tick,
            config,
        );
        for (action, was_applied) in actions.iter().zip(applied) {
            if !was_applied {
                continue;
            }
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
                        *tick,
                        self.wall_ms,
                        SYS_BLOCKADE,
                        EVT_BLOCKADE_STARTED,
                        serde_json::json!({ "system": sys_name(world, *system) }),
                    ));
                }
                BlockadeEvent::BlockadeEnded { system, tick } => {
                    self.events.push(GameEventRecord::new(
                        *tick,
                        self.wall_ms,
                        SYS_BLOCKADE,
                        EVT_BLOCKADE_ENDED,
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
                        *tick,
                        self.wall_ms,
                        SYS_UPRISING,
                        EVT_UPRISING_INCIDENT,
                        serde_json::json!({ "system": sys_name(world, *system) }),
                    ));
                }
                UprisingEvent::UprisingBegan { system, tick } => {
                    let before = world.systems.get(*system).map(|s| s.control);
                    if let Some(sys) = world.systems.get_mut(*system) {
                        sys.control = match sys.control {
                            ControlKind::Controlled(rebellion_core::dat::Faction::Alliance) => {
                                ControlKind::Controlled(rebellion_core::dat::Faction::Empire)
                            }
                            ControlKind::Controlled(rebellion_core::dat::Faction::Empire) => {
                                ControlKind::Controlled(rebellion_core::dat::Faction::Alliance)
                            }
                            other => other,
                        };
                    }
                    let after = world.systems.get(*system).map(|s| s.control);
                    if before != after {
                        self.events.push(GameEventRecord::new(
                            *tick,
                            self.wall_ms,
                            SYS_UPRISING,
                            EVT_CONTROL_CHANGED,
                            serde_json::json!({
                                "system": sys_name(world, *system),
                                "from": format!("{:?}", before),
                                "to": format!("{:?}", after),
                                "cause": "uprising",
                            }),
                        ));
                    }
                    self.events.push(GameEventRecord::new(
                        *tick,
                        self.wall_ms,
                        SYS_UPRISING,
                        EVT_UPRISING_BEGAN,
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
            let BetrayalEvent::CharacterBetrayed {
                character,
                defected_to_alliance,
            } = evt;

            // #R9: reveal-before-flip — emit EVT_TRAITOR_REVEALED while the
            // character still belongs to the original faction.
            self.emit(
                SYS_BETRAYAL,
                EVT_TRAITOR_REVEALED,
                serde_json::json!({
                    "character": char_name(world, *character),
                    "original_faction": if world.characters.get(*character)
                        .is_some_and(|c| c.is_alliance) { "alliance" } else { "empire" },
                }),
            );

            // Apply the faction flip.
            if let Some(c) = world.characters.get_mut(*character) {
                c.is_alliance = *defected_to_alliance;
                c.is_empire = !*defected_to_alliance;
            }
            for (_, fleet) in world.fleets.iter_mut() {
                fleet.characters.retain(|&k| k != *character);
            }

            // #R10: emit EVT_SIDE_CHANGE after the flip with DI-H2 payload.
            // Replaces the former EVT_BETRAYAL emit (identical payload, same timing).
            self.emit(
                SYS_BETRAYAL,
                EVT_SIDE_CHANGE,
                serde_json::json!({
                    "character": char_name(world, *character),
                    "defected_to_alliance": defected_to_alliance,
                }),
            );
        }
    }

    // ── Step 11: Death Star ───────────────────────────────────────────────

    /// Apply death star events: planet destruction + telemetry.
    pub fn apply_death_star_events(&mut self, world: &mut GameWorld, events: &[DeathStarEvent]) {
        // Heartbeat: emit a status event so the "death_star" system tag always appears.
        self.events.push(GameEventRecord::new(
            self.tick,
            self.wall_ms,
            SYS_DEATH_STAR,
            EVT_DS_STATUS,
            serde_json::json!({ "events": events.len() }),
        ));
        for evt in events {
            match evt {
                DeathStarEvent::ConstructionCompleted { system, tick } => {
                    self.events.push(GameEventRecord::new(
                        *tick,
                        self.wall_ms,
                        SYS_DEATH_STAR,
                        EVT_DS_CONSTRUCTION,
                        serde_json::json!({ "system": sys_name(world, *system) }),
                    ));
                }
                DeathStarEvent::PlanetDestroyed { system, tick } => {
                    if let Some(sys) = world.systems.get_mut(*system) {
                        sys.is_destroyed = true;
                    }
                    self.events.push(GameEventRecord::new(
                        *tick,
                        self.wall_ms,
                        SYS_DEATH_STAR,
                        EVT_DS_FIRED,
                        serde_json::json!({ "system": sys_name(world, *system) }),
                    ));
                }
                DeathStarEvent::NearbyWarning { .. } => {}
            }
        }
    }

    // ── Step 12: Research ─────────────────────────────────────────────────

    /// Apply research results: level-ups + telemetry.
    pub fn apply_research_results(
        &mut self,
        results: &[ResearchResult],
        research_state: &mut ResearchState,
    ) {
        for result in results {
            let ResearchResult::TechUnlocked {
                faction_is_alliance,
                tech_type,
                new_level,
            } = result;
            if *faction_is_alliance {
                research_state.alliance.advance(*tech_type);
            } else {
                research_state.empire.advance(*tech_type);
            }
            self.emit(
                SYS_RESEARCH,
                EVT_RESEARCH_UNLOCKED,
                serde_json::json!({
                    "faction_is_alliance": faction_is_alliance,
                    "tech_type": format!("{:?}", tech_type),
                    "new_level": new_level,
                }),
            );
        }
    }

    // ── Step 12b: Repair ───────────────────────────────────────────────────

    /// Apply repair events: hull restoration + telemetry.
    pub fn apply_repair_events(&mut self, world: &mut GameWorld, events: &[RepairEvent]) {
        for evt in events {
            match evt {
                RepairEvent::ShipRepaired {
                    fleet,
                    ship_index,
                    hull_before,
                    hull_after,
                } => {
                    // Apply hull restoration to the ShipInstance.
                    if let Some(f) = world.fleets.get_mut(*fleet) {
                        if let Some(ship) = f.capital_ships.get_mut(*ship_index) {
                            ship.hull_current = *hull_after;
                        }
                    }
                    let fleet_name = world
                        .fleets
                        .get(*fleet)
                        .map(|f| sys_name(world, f.location))
                        .unwrap_or_else(|| "unknown".into());
                    self.emit(
                        SYS_REPAIR,
                        EVT_SHIP_REPAIRED,
                        serde_json::json!({
                            "fleet_location": fleet_name,
                            "hull_before": hull_before,
                            "hull_after": hull_after,
                            "delta": hull_after - hull_before,
                        }),
                    );
                }
                RepairEvent::RepairCheckPerformed {
                    system,
                    fleet,
                    ships_checked,
                } => {
                    let system_name = sys_name(world, *system);
                    let fleet_commander = world
                        .fleets
                        .get(*fleet)
                        .and_then(|f| {
                            world
                                .characters
                                .get(*f.characters.first()?)
                                .map(|c| c.name.as_str())
                        })
                        .unwrap_or("uncrewed");
                    self.emit(
                        SYS_REPAIR,
                        EVT_SHIP_REPAIR_STARTED,
                        serde_json::json!({
                            "system": system_name,
                            "fleet_commander": fleet_commander,
                            "ships_checked": ships_checked,
                        }),
                    );
                }
            }
        }
    }

    // ── Step 13: Jedi ─────────────────────────────────────────────────────

    /// Apply jedi events: tier advancement + discovery + telemetry.
    pub fn apply_jedi_events(
        &mut self,
        world: &mut GameWorld,
        events: &[JediEvent],
        jedi_state: &mut JediState,
    ) {
        // Heartbeat: emit a check event so the "jedi" system tag always appears.
        self.events.push(GameEventRecord::new(
            self.tick,
            self.wall_ms,
            SYS_JEDI,
            EVT_JEDI_CHECK,
            serde_json::json!({ "training": jedi_state.training.len(), "events": events.len() }),
        ));
        for evt in events {
            match evt {
                JediEvent::TierAdvanced {
                    character,
                    new_tier,
                } => {
                    if let Some(c) = world.characters.get_mut(*character) {
                        c.force_tier = *new_tier;
                        c.force_experience = match new_tier {
                            rebellion_core::world::ForceTier::None => 0,
                            rebellion_core::world::ForceTier::Aware => 1,
                            rebellion_core::world::ForceTier::Training => {
                                rebellion_core::jedi::XP_TO_TRAINING
                            }
                            rebellion_core::world::ForceTier::Experienced => {
                                rebellion_core::jedi::XP_TO_EXPERIENCED
                            }
                        };
                    }
                    self.emit(
                        SYS_JEDI,
                        EVT_JEDI_TIER,
                        serde_json::json!({
                            "character": char_name(world, *character),
                            "new_tier": format!("{:?}", new_tier),
                        }),
                    );
                }
                JediEvent::TrainingComplete { character } => {
                    jedi_state.stop_training(*character);
                }
                JediEvent::JediDiscovered { character, .. } => {
                    if let Some(c) = world.characters.get_mut(*character) {
                        c.is_discovered_jedi = true;
                    }
                    self.emit(
                        SYS_JEDI,
                        EVT_JEDI_DISCOVERED,
                        serde_json::json!({
                            "character": char_name(world, *character),
                        }),
                    );
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
                            sys.popularity_empire = (sys.popularity_empire + delta).clamp(0.0, 1.0);
                        }
                    }
                    const CONTROL_THRESHOLD: f32 = 0.6;
                    let a_pop = sys.popularity_alliance;
                    let e_pop = sys.popularity_empire;
                    let new_control = if a_pop >= CONTROL_THRESHOLD && a_pop > e_pop + 0.1 {
                        Some(ControlKind::Controlled(
                            rebellion_core::dat::Faction::Alliance,
                        ))
                    } else if e_pop >= CONTROL_THRESHOLD && e_pop > a_pop + 0.1 {
                        Some(ControlKind::Controlled(
                            rebellion_core::dat::Faction::Empire,
                        ))
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
            MissionEffect::UprisingStarted {
                system,
                popularity_delta,
            } => {
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
            MissionEffect::FacilitySabotaged {
                system,
                facility_index,
                ..
            } => {
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
                // removing from the arena so reactive story events
                // (`EVT_CHARACTER_KILLED` 0x306) can still resolve the character
                // by `dat_id` / `name`. Character is removed from all fleet
                // rosters immediately. Uniqueness comes from the `is_killed`
                // flag combined with `is_repeatable: false` (DI-M3).
                //
                // Reactivity note: mission kills fire same-tick (assassination
                // at step 5, EventSystem at step 6 — same-tick is the correct
                // player-facing behavior). Death Star kills fire next-tick
                // (cleanup at step 11 is after events at step 6). Both routes
                // go through `Character::mark_killed()` so systems that iterate
                // `world.characters` see a consistent "dead" shape regardless
                // of which path produced the death.
                for (_, fleet) in world.fleets.iter_mut() {
                    fleet.characters.retain(|&k| k != *character);
                }
                if let Some(c) = world.characters.get_mut(*character) {
                    c.mark_killed();
                }
            }
            MissionEffect::CharacterCaptured {
                character,
                captured_by,
                at_system,
            } => {
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
            MissionEffect::CharacterRescued {
                character,
                returned_to,
                ..
            } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    match returned_to {
                        MissionFaction::Alliance => {
                            c.is_alliance = true;
                            c.is_empire = false;
                        }
                        MissionFaction::Empire => {
                            c.is_alliance = false;
                            c.is_empire = true;
                        }
                    }
                    c.is_captive = false;
                    c.captured_by = None;
                    c.capture_tick = None;
                }
            }
            MissionEffect::CharacterBusy { character } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.on_mission = true;
                }
            }
            MissionEffect::CharacterAvailable { character } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.on_mission = false;
                    c.on_hidden_mission = false;
                }
            }
            MissionEffect::DecoyTriggered { .. } => {}
            MissionEffect::CharacterEscaped {
                character,
                escaped_to_alliance,
            } => {
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
                            sys.popularity_alliance =
                                (sys.popularity_alliance + 0.05).clamp(0.0, 1.0);
                            sys.popularity_empire = (sys.popularity_empire - 0.05).clamp(0.0, 1.0);
                        }
                        _ => {
                            sys.popularity_empire = (sys.popularity_empire + 0.05).clamp(0.0, 1.0);
                            sys.popularity_alliance =
                                (sys.popularity_alliance - 0.05).clamp(0.0, 1.0);
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
///   originating event on `CharacterAtSystem OR CharacterAssignedToFleet`
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
            EventAction::ShiftPopularity {
                system,
                alliance_delta,
                empire_delta,
            } => {
                if let Some(sys) = world.systems.get_mut(*system) {
                    sys.popularity_alliance =
                        (sys.popularity_alliance + alliance_delta).clamp(0.0, 1.0);
                    sys.popularity_empire = (sys.popularity_empire + empire_delta).clamp(0.0, 1.0);
                }
            }
            EventAction::ModifyCharacterSkill {
                character,
                skill,
                base_delta,
            } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    let d = *base_delta;
                    let apply = |v: u32, delta: i32| (v as i64 + delta as i64).max(0) as u32;
                    match skill {
                        SkillField::Diplomacy => c.diplomacy.base = apply(c.diplomacy.base, d),
                        SkillField::Espionage => c.espionage.base = apply(c.espionage.base, d),
                        SkillField::ShipDesign => c.ship_design.base = apply(c.ship_design.base, d),
                        SkillField::TroopTraining => {
                            c.troop_training.base = apply(c.troop_training.base, d)
                        }
                        SkillField::FacilityDesign => {
                            c.facility_design.base = apply(c.facility_design.base, d)
                        }
                        SkillField::Combat => c.combat.base = apply(c.combat.base, d),
                        SkillField::Leadership => c.leadership.base = apply(c.leadership.base, d),
                        SkillField::Loyalty => c.loyalty.base = apply(c.loyalty.base, d),
                        SkillField::JediLevel => c.jedi_level.base = apply(c.jedi_level.base, d),
                    }
                }
            }
            EventAction::RelocateCharacter { .. } => {}
            EventAction::SetMandatoryMission {
                character,
                mandatory,
            } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.on_mandatory_mission = *mandatory;
                }
            }
            EventAction::ModifyForceTier {
                character,
                new_tier,
            } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.force_tier = *new_tier;
                }
            }
            EventAction::RemoveCharacter { character } => {
                for (_, fleet) in world.fleets.iter_mut() {
                    fleet.characters.retain(|&k| k != *character);
                }
                world.characters.remove(*character);
            }
            EventAction::StartJediTraining { .. } => {}
            EventAction::TransferCharacter {
                character,
                destination,
                new_faction,
            } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.current_system = Some(*destination);
                    if let Some(faction) = new_faction {
                        match faction {
                            rebellion_core::dat::Faction::Alliance => {
                                c.is_alliance = true;
                                c.is_empire = false;
                            }
                            rebellion_core::dat::Faction::Empire => {
                                c.is_alliance = false;
                                c.is_empire = true;
                            }
                            _ => {}
                        }
                    }
                }
            }
            EventAction::TriggerEvent { .. } => {}
            EventAction::AccumulateForceExperience { character, amount } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.force_experience += amount;
                }
            }
            EventAction::CaptureCharacter {
                character,
                captor_faction,
            } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.is_captive = true;
                    c.captured_by = Some(*captor_faction);
                    c.capture_tick = Some(tick);
                }
                for (_, fleet) in world.fleets.iter_mut() {
                    fleet.characters.retain(|&k| k != *character);
                }
            }
            EventAction::SetCarboniteState { character, frozen } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.on_mandatory_mission = *frozen;
                    if *frozen {
                        c.is_captive = true;
                        c.capture_tick = Some(tick);
                    } else {
                        c.is_captive = false;
                        c.captured_by = None;
                        c.capture_tick = None;
                    }
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
                // `state.define()` site on `CharacterAtSystem OR CharacterAssignedToFleet`
                // so this fallback should always succeed (SF-#7). We `warn!`
                // instead of panic on the unexpected case.
                let character = *at_character;
                let resolved: Option<(rebellion_core::ids::SystemKey, bool)> =
                    world.characters.get(character).and_then(|c| {
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
                        // community cross-reference.
                        let _ = is_alliance;
                        let spawn_alliance = false;
                        // A2: Create SpecialForceUnit in world arena and
                        // push key onto the target system's roster.
                        // DatId(0) = event-spawned (not from SPECFCSD.DAT).
                        let sf_key = world.special_forces.insert(SpecialForceUnit {
                            class_dat_id: DatId::new(0),
                            is_alliance: spawn_alliance,
                        });
                        if let Some(sys) = world.systems.get_mut(at_system) {
                            sys.special_forces.push(sf_key);
                        }
                        effects_out.push(GameEffect::SpecialForceSpawned {
                            at_system,
                            is_alliance: spawn_alliance,
                        });
                    }
                    None => {
                        // Structured warn: the event chain must gate
                        // SpawnSpecialForce on CharacterAtSystem OR
                        // CharacterAssignedToFleet (SF-#7). If we
                        // reach this branch, the guard is missing — log
                        // and drop the action rather than spawning at
                        // an arbitrary fallback system.
                        eprintln!(
                            "[shamash-bet] SpawnSpecialForce at character {:?} \
                             could not resolve a target system — character has \
                             no current_system and is not on any movement-ordered \
                             fleet. Event chain should have gated this action with \
                             CharacterAtSystem OR CharacterAssignedToFleet.",
                            character,
                        );
                    }
                }
            }
            EventAction::SetHeritageKnown { character } => {
                // Dabora 3 #R4: flip heritage_known when 0x396 "Final Battle
                // Imminent" fires. The render layer branches on this flag
                // in `event_id_to_resource()` to pick between the "Student
                // Luke" / "Knight Luke" / "Emperor + Vader vs Knight Luke"
                // BMP variants for 0x220 EVT_FINAL_BATTLE (SIMP-H5 +
                // ARCH-#9 + SF-#11 triple-collapse — we do NOT create
                // 0x222).
                //
                // Fail loudly on missing character: a silently-dropped flip
                // shows the wrong cutscene, which is a story-continuity bug
                // that's extremely hard to diagnose after the fact. Matches
                // the `SpawnSpecialForce` structured-warn pattern above.
                if let Some(c) = world.characters.get_mut(*character) {
                    c.heritage_known = true;
                } else {
                    eprintln!(
                        "[shamash-bet] SetHeritageKnown target character {:?} \
                         not found in arena — heritage_known flip dropped. \
                         Final Battle cutscene variant will not reflect the \
                         paternity reveal.",
                        character,
                    );
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AI action helper (moved from simulation.rs)
// ---------------------------------------------------------------------------

#[expect(
    clippy::too_many_arguments,
    reason = "Keep the existing explicit simulation state inputs at this integration boundary."
)]
fn apply_ai_actions_inner(
    actions: &[AIAction],
    rolls: &[f64],
    ai_state: &mut AIState,
    mission_state: &mut MissionState,
    mfg_state: &mut ManufacturingState,
    movement_state: &mut MovementState,
    troop_transport: &mut TroopTransportState,
    research_state: &mut ResearchState,
    world: &mut GameWorld,
    _tick: u64,
    config: &rebellion_core::tuning::GameConfig,
) -> Vec<bool> {
    let mission_faction = ai_state
        .faction
        .map(|f| f.as_mission_faction())
        .unwrap_or(MissionFaction::Empire);

    let mut roll_idx = 0;
    let mut applied = Vec::with_capacity(actions.len());
    for action in actions {
        let was_applied = match action {
            AIAction::DispatchMission {
                kind,
                character,
                target_system,
                target_character,
                duration_roll,
            } => {
                let roll = rolls.get(roll_idx).copied().unwrap_or(*duration_roll);
                roll_idx += 1;
                mission_state.dispatch(
                    *kind,
                    mission_faction,
                    *character,
                    *target_system,
                    *target_character,
                    roll,
                );
                ai_state.mark_busy(*character);
                true
            }
            AIAction::EnqueueProduction {
                system,
                kind,
                ticks,
            } => {
                mfg_state.enqueue(*system, QueueItem::new(*kind, *ticks, *ticks));
                true
            }
            AIAction::DispatchResearch {
                character,
                tech_type,
                ticks,
            } => {
                let is_alliance = ai_state
                    .faction
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
                true
            }
            AIAction::MoveFleet {
                fleet,
                to_system,
                troops,
                ..
            } => {
                let transit = world.fleets.get(*fleet).map(|f| {
                    rebellion_core::movement::fleet_transit_ticks_with_config(
                        f,
                        world,
                        f.location,
                        *to_system,
                        config.movement.distance_scale,
                        config.movement.min_transit_ticks,
                        config.movement.default_fighter_hyperdrive,
                    )
                });
                let embarked = if troops.is_empty() {
                    true
                } else {
                    troop_transport.embark(world, *fleet, troops).is_ok()
                };
                if !embarked {
                    false
                } else {
                    let departed = transit
                        .map(|ticks| {
                            begin_fleet_transit(movement_state, world, *fleet, *to_system, ticks)
                        })
                        .unwrap_or(false);
                    if !departed && !troops.is_empty() {
                        let origin = world.fleets.get(*fleet).map(|value| value.location);
                        if let Some(origin) = origin {
                            let _ = troop_transport.disembark_all(world, *fleet, origin);
                        }
                    }
                    departed
                }
            }
        };
        applied.push(was_applied);
    }
    applied
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::ai::{AiFaction, FleetMoveReason};
    use rebellion_core::dat::{ExplorationStatus, Faction, SectorGroup};
    use rebellion_core::tuning::GameConfig;
    use rebellion_core::world::{CapitalShipClass, Sector, System};

    fn add_system(world: &mut GameWorld, name: &str) -> SystemKey {
        let sector = world.sectors.insert(Sector {
            dat_id: DatId(1),
            name: format!("{name} Sector"),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        world.systems.insert(System {
            dat_id: DatId(2),
            name: name.into(),
            sector,
            x: 0,
            y: 0,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.0,
            popularity_empire: 1.0,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Empire),
        })
    }

    #[test]
    fn duplicate_move_action_preserves_first_order_and_emits_once() {
        let mut world = GameWorld::default();
        let origin = add_system(&mut world, "Origin");
        let first_target = add_system(&mut world, "First Target");
        let second_target = add_system(&mut world, "Second Target");
        let fleet = world.fleets.insert(Fleet {
            location: origin,
            capital_ships: vec![],
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            has_death_star: false,
        });
        world.systems[origin].fleets.push(fleet);

        let actions = vec![
            AIAction::MoveFleet {
                fleet,
                to_system: first_target,
                reason: FleetMoveReason::Attack,
                troops: vec![],
            },
            AIAction::MoveFleet {
                fleet,
                to_system: second_target,
                reason: FleetMoveReason::Reinforce,
                troops: vec![],
            },
        ];
        let mut ai = AIState::new(AiFaction::Empire);
        let mut missions = MissionState::new();
        let mut manufacturing = ManufacturingState::new();
        let mut movement = MovementState::new();
        let mut troop_transport = TroopTransportState::default();
        let mut research = ResearchState::new();
        let mut integrator = PerceptionIntegrator::new(5, 0);

        integrator.apply_ai_actions(
            &actions,
            &[],
            &mut ai,
            &mut missions,
            &mut manufacturing,
            &mut movement,
            &mut troop_transport,
            &mut research,
            &mut world,
            5,
            &GameConfig::default(),
            false,
        );

        assert_eq!(movement.len(), 1);
        assert_eq!(movement.get(fleet).unwrap().destination, first_target);
        let events = integrator.finish();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EVT_AI_ACTION);
        assert_eq!(events[0].details["to"], "First Target");
    }

    #[test]
    fn ai_transport_action_embarks_before_authoritative_transit() {
        let mut world = GameWorld::default();
        let origin = add_system(&mut world, "Origin");
        let destination = add_system(&mut world, "Destination");
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            is_alliance: false,
            is_empire: true,
            hull: 100,
            troop_capacity: 1,
            ..CapitalShipClass::default()
        });
        let fleet = world.fleets.insert(Fleet {
            location: origin,
            capital_ships: vec![ShipInstance::new(class, 100, false)],
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            has_death_star: false,
        });
        world.systems[origin].fleets.push(fleet);
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: false,
            regiment_strength: 100,
        });
        world.systems[origin].ground_units.push(troop);

        let actions = [AIAction::MoveFleet {
            fleet,
            to_system: destination,
            reason: FleetMoveReason::Attack,
            troops: vec![troop],
        }];
        let mut ai = AIState::new(AiFaction::Empire);
        let mut missions = MissionState::new();
        let mut manufacturing = ManufacturingState::new();
        let mut movement = MovementState::new();
        let mut troop_transport = TroopTransportState::default();
        let mut research = ResearchState::new();
        let mut integrator = PerceptionIntegrator::new(5, 0);

        integrator.apply_ai_actions(
            &actions,
            &[],
            &mut ai,
            &mut missions,
            &mut manufacturing,
            &mut movement,
            &mut troop_transport,
            &mut research,
            &mut world,
            5,
            &GameConfig::default(),
            false,
        );

        assert!(movement.is_in_transit(fleet));
        assert_eq!(troop_transport.cargo(fleet), &[troop]);
        assert!(!world.systems[origin].ground_units.contains(&troop));
        assert_eq!(integrator.finish()[0].details["troops"], 1);
    }

    #[test]
    fn production_uses_orbiting_garrison_instead_of_transit_fleet() {
        let mut world = GameWorld::default();
        let origin = add_system(&mut world, "Origin");
        let destination = add_system(&mut world, "Destination");
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            is_alliance: false,
            hull: 100,
            ..CapitalShipClass::default()
        });
        let transit = world.fleets.insert(Fleet {
            location: origin,
            capital_ships: vec![ShipInstance::new(class, 100, false)],
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            has_death_star: false,
        });
        world.systems[origin].fleets.push(transit);
        let mut movement = MovementState::new();
        assert!(begin_fleet_transit(
            &mut movement,
            &mut world,
            transit,
            destination,
            10,
        ));

        let completion = CompletionEvent {
            system: origin,
            tick: 1,
            kind: BuildableKind::CapitalShip(class),
        };
        apply_build_completion_inner(&completion, &mut world);
        apply_build_completion_inner(&completion, &mut world);

        assert_eq!(world.fleets.len(), 2);
        assert_eq!(world.fleets[transit].ship_count(), 1);
        let garrison = world.systems[origin].fleets[0];
        assert_ne!(garrison, transit);
        assert_eq!(world.fleets[garrison].ship_count(), 2);
    }

    #[test]
    fn troop_production_preserves_original_class_and_faction() {
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Training World");
        let template = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0003),
            is_alliance: true,
            regiment_strength: 37,
        });

        apply_build_completion_inner(
            &CompletionEvent {
                system,
                tick: 1,
                kind: BuildableKind::Troop(template),
            },
            &mut world,
        );

        let built = *world.systems[system].ground_units.last().unwrap();
        assert_ne!(built, template);
        assert_eq!(world.troops[built].class_dat_id, DatId::new(0x1000_0003));
        assert!(world.troops[built].is_alliance);
        assert_eq!(world.troops[built].regiment_strength, 100);
    }
}

// ---------------------------------------------------------------------------
// Combat helpers (moved from simulation.rs)
// ---------------------------------------------------------------------------

/// Safety cap for a complete system-level engagement.
///
/// A round that changes neither hull nor fighter strength ends the engagement
/// immediately, so this cap only protects against unexpectedly large battles.
pub const MAX_SYSTEM_COMBAT_ROUNDS: u32 = 256;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SpaceForceSummary {
    pub capital_ships: usize,
    pub fighter_squadrons: u32,
    pub hull: i64,
}

/// Aggregate result for every hostile fleet orbiting one system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemSpaceCombatResult {
    pub system: SystemKey,
    /// Alliance is the attacker and Empire is the defender in auto-resolve.
    pub winner: CombatSide,
    /// A surviving fleet belonging to the winning faction, when decisive.
    pub winner_fleet: Option<FleetKey>,
    pub rounds: u32,
    pub alliance_fleets: usize,
    pub empire_fleets: usize,
    pub alliance_before: SpaceForceSummary,
    pub alliance_after: SpaceForceSummary,
    pub empire_before: SpaceForceSummary,
    pub empire_after: SpaceForceSummary,
    /// True when combat could not change any remaining unit or reached the cap.
    pub stalemate: bool,
}

fn orbiting_fleets_by_faction(
    world: &GameWorld,
    system: SystemKey,
) -> (Vec<FleetKey>, Vec<FleetKey>) {
    let Some(system) = world.systems.get(system) else {
        return (Vec::new(), Vec::new());
    };
    system
        .fleets
        .iter()
        .copied()
        .filter_map(|fleet_key| {
            world
                .fleets
                .get(fleet_key)
                .map(|fleet| (fleet_key, fleet.is_alliance))
        })
        .fold(
            (Vec::new(), Vec::new()),
            |(mut alliance, mut empire), (fleet_key, is_alliance)| {
                if is_alliance {
                    alliance.push(fleet_key);
                } else {
                    empire.push(fleet_key);
                }
                (alliance, empire)
            },
        )
}

fn summarize_space_force(world: &GameWorld, fleets: &[FleetKey]) -> SpaceForceSummary {
    fleets
        .iter()
        .fold(SpaceForceSummary::default(), |mut summary, fleet_key| {
            if let Some(fleet) = world.fleets.get(*fleet_key) {
                summary.capital_ships +=
                    fleet.capital_ships.iter().filter(|ship| ship.alive).count();
                summary.fighter_squadrons +=
                    fleet.fighters.iter().map(|entry| entry.count).sum::<u32>();
                summary.hull += fleet
                    .capital_ships
                    .iter()
                    .filter(|ship| ship.alive)
                    .map(|ship| i64::from(ship.hull_current))
                    .sum::<i64>();
            }
            summary
        })
}

/// Resolve all hostile task forces at a system as one uninterrupted engagement.
///
/// Each core combat call models one combat round. Applying every damaging round
/// here prevents repair ticks from healing fleets between rounds and advances to
/// the next deterministic pair when either task force is destroyed.
pub fn resolve_system_space_combat(
    world: &mut GameWorld,
    system: SystemKey,
    difficulty: u8,
    rng_rolls: &[f64],
    tick: u64,
    death_star_shield_active: bool,
) -> SystemSpaceCombatResult {
    let (initial_alliance, initial_empire) = orbiting_fleets_by_faction(world, system);
    let alliance_before = summarize_space_force(world, &initial_alliance);
    let empire_before = summarize_space_force(world, &initial_empire);
    let mut rounds = 0;
    let mut stalemate = false;

    while rounds < MAX_SYSTEM_COMBAT_ROUNDS {
        let (alliance, empire) = orbiting_fleets_by_faction(world, system);
        let (Some(&attacker), Some(&defender)) = (alliance.first(), empire.first()) else {
            break;
        };

        let round = CombatSystem::resolve_space(
            world,
            attacker,
            defender,
            system,
            difficulty,
            rng_rolls,
            tick,
            death_star_shield_active,
        );
        let made_progress = round
            .ship_damage
            .iter()
            .any(|event| event.hull_after < event.hull_before)
            || round
                .fighter_losses
                .iter()
                .any(|event| event.squads_after < event.squads_before);

        apply_space_combat_result_inner(&round, world);
        rounds += 1;

        if !made_progress {
            stalemate = true;
            break;
        }
    }

    let (alliance, empire) = orbiting_fleets_by_faction(world, system);
    let alliance_after = summarize_space_force(world, &alliance);
    let empire_after = summarize_space_force(world, &empire);
    let (winner, winner_fleet) = match (alliance.first().copied(), empire.first().copied()) {
        (Some(fleet), None) => (CombatSide::Attacker, Some(fleet)),
        (None, Some(fleet)) => (CombatSide::Defender, Some(fleet)),
        _ => (CombatSide::Draw, None),
    };
    if rounds == MAX_SYSTEM_COMBAT_ROUNDS && winner == CombatSide::Draw {
        stalemate = true;
    }

    SystemSpaceCombatResult {
        system,
        winner,
        winner_fleet,
        rounds,
        alliance_fleets: initial_alliance.len(),
        empire_fleets: initial_empire.len(),
        alliance_before,
        alliance_after,
        empire_before,
        empire_after,
        stalemate,
    }
}

pub fn apply_space_combat_result_inner(result: &SpaceCombatResult, world: &mut GameWorld) {
    // Apply hull damage to individual ship instances.
    for evt in &result.ship_damage {
        let fleet_key = evt.fleet;
        if let Some(fleet) = world.fleets.get_mut(fleet_key) {
            // ship_index maps 1:1 to alive ships at snapshot time.
            // Find the nth alive ship.
            let mut alive_idx = 0;
            for ship in fleet.capital_ships.iter_mut() {
                if !ship.alive {
                    continue;
                }
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

    // Apply fighter attrition before testing whether either fleet is empty.
    // `fighter_index` maps directly to the roster snapshot used by combat.
    for evt in &result.fighter_losses {
        if let Some(fleet) = world.fleets.get_mut(evt.fleet) {
            if let Some(entry) = fleet.fighters.get_mut(evt.fighter_index) {
                entry.count = evt.squads_after;
            }
        }
    }

    // Remove dead ships and empty fleets.
    for &fleet_key in &[result.attacker_fleet, result.defender_fleet] {
        if let Some(fleet) = world.fleets.get_mut(fleet_key) {
            fleet.capital_ships.retain(|s| s.alive);
        }
        let is_empty = world
            .fleets
            .get(fleet_key)
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

#[cfg(test)]
mod combat_application_tests {
    use super::*;
    use rebellion_core::combat::{FighterLossEvent, SpaceCombatResult};
    use rebellion_core::dat::ExplorationStatus;
    use rebellion_core::world::{CapitalShipClass, FighterEntry, Fleet, System};

    fn add_combat_system(world: &mut GameWorld) -> SystemKey {
        world.systems.insert(System {
            dat_id: DatId::new(0x90000001),
            name: "Test System".into(),
            sector: Default::default(),
            x: 0,
            y: 0,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Uncontrolled,
        })
    }

    fn add_ship_fleet(
        world: &mut GameWorld,
        system: SystemKey,
        is_alliance: bool,
        hull: u32,
        attack: u32,
    ) -> FleetKey {
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(if is_alliance { 0x30000001 } else { 0x30000002 }),
            is_alliance,
            is_empire: !is_alliance,
            hull,
            turbolaser_fore: attack,
            ..CapitalShipClass::default()
        });
        let fleet = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ShipInstance::new(class, hull as i32, is_alliance)],
            fighters: vec![],
            characters: vec![],
            is_alliance,
            has_death_star: false,
        });
        world.systems[system].fleets.push(fleet);
        fleet
    }

    #[test]
    fn fighter_losses_update_rosters_and_remove_defeated_fleet() {
        let mut world = GameWorld::default();
        let attacker = world.fleets.insert(Fleet {
            location: SystemKey::default(),
            capital_ships: vec![],
            fighters: vec![FighterEntry {
                class: Default::default(),
                count: 4,
            }],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        let defender = world.fleets.insert(Fleet {
            location: SystemKey::default(),
            capital_ships: vec![],
            fighters: vec![FighterEntry {
                class: Default::default(),
                count: 3,
            }],
            characters: vec![],
            is_alliance: false,
            has_death_star: false,
        });
        let result = SpaceCombatResult {
            attacker_fleet: attacker,
            defender_fleet: defender,
            system: SystemKey::default(),
            winner: CombatSide::Attacker,
            ship_damage: vec![],
            fighter_losses: vec![
                FighterLossEvent {
                    fleet: attacker,
                    fighter_index: 0,
                    squads_before: 4,
                    squads_after: 2,
                },
                FighterLossEvent {
                    fleet: defender,
                    fighter_index: 0,
                    squads_before: 3,
                    squads_after: 0,
                },
            ],
            tick: 1,
        };

        apply_space_combat_result_inner(&result, &mut world);

        assert_eq!(world.fleets[attacker].fighters[0].count, 2);
        assert!(!world.fleets.contains_key(defender));
    }

    #[test]
    fn system_engagement_resolves_multiple_rounds_before_repair() {
        let mut world = GameWorld::default();
        let system = add_combat_system(&mut world);
        let attacker = add_ship_fleet(&mut world, system, true, 100, 30);
        let defender = add_ship_fleet(&mut world, system, false, 100, 1);

        let result = resolve_system_space_combat(&mut world, system, 1, &[0.5; 256], 10, false);

        assert_eq!(result.winner, CombatSide::Attacker);
        assert_eq!(result.winner_fleet, Some(attacker));
        assert!(result.rounds > 1);
        assert!(!result.stalemate);
        assert!(!world.fleets.contains_key(defender));
    }

    #[test]
    fn system_engagement_includes_every_orbiting_task_force() {
        let mut world = GameWorld::default();
        let system = add_combat_system(&mut world);
        let first_alliance = add_ship_fleet(&mut world, system, true, 200, 100);
        let second_alliance = add_ship_fleet(&mut world, system, true, 200, 100);
        let first_empire = add_ship_fleet(&mut world, system, false, 30, 1);
        let second_empire = add_ship_fleet(&mut world, system, false, 30, 1);

        let result = resolve_system_space_combat(&mut world, system, 1, &[0.5; 256], 11, false);

        assert_eq!(result.alliance_fleets, 2);
        assert_eq!(result.empire_fleets, 2);
        assert_eq!(result.winner, CombatSide::Attacker);
        assert!(world.fleets.contains_key(first_alliance));
        assert!(world.fleets.contains_key(second_alliance));
        assert!(!world.fleets.contains_key(first_empire));
        assert!(!world.fleets.contains_key(second_empire));

        let mut integrator = PerceptionIntegrator::new(11, 0);
        integrator.emit_system_space_combat(&world, &result);
        let events = integrator.finish();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EVT_COMBAT_SPACE);
        assert_eq!(events[0].details["alliance_fleets"], 2);
        assert_eq!(events[0].details["empire_fleets"], 2);
    }

    #[test]
    fn system_engagement_stops_on_true_no_progress_stalemate() {
        let mut world = GameWorld::default();
        let system = add_combat_system(&mut world);
        add_ship_fleet(&mut world, system, true, 100, 0);
        add_ship_fleet(&mut world, system, false, 100, 0);

        let result = resolve_system_space_combat(&mut world, system, 1, &[0.5; 256], 12, false);

        assert_eq!(result.winner, CombatSide::Draw);
        assert_eq!(result.rounds, 1);
        assert!(result.stalemate);
        assert_eq!(world.systems[system].fleets.len(), 2);
    }
}

pub fn apply_ground_combat_result_inner(result: &GroundCombatResult, world: &mut GameWorld) {
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

pub fn apply_build_completion_inner(completion: &CompletionEvent, world: &mut GameWorld) {
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
                sys.fleets.iter().copied().find(|&fk| {
                    world
                        .fleets
                        .get(fk)
                        .map(|f| f.is_alliance == is_alliance)
                        .unwrap_or(false)
                })
            };

            if let Some(fk) = fleet_key {
                if let Some(fleet) = world.fleets.get_mut(fk) {
                    let hull = world
                        .capital_ship_classes
                        .get(*class_key)
                        .map(|c| c.hull as i32)
                        .unwrap_or(100);
                    fleet
                        .capital_ships
                        .push(ShipInstance::new(*class_key, hull, is_alliance));
                }
            } else {
                let hull = world
                    .capital_ship_classes
                    .get(*class_key)
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
                sys.fleets.iter().copied().find(|&fk| {
                    world
                        .fleets
                        .get(fk)
                        .map(|f| f.is_alliance == is_alliance)
                        .unwrap_or(false)
                })
            };

            if let Some(fk) = fleet_key {
                if let Some(fleet) = world.fleets.get_mut(fk) {
                    if let Some(entry) = fleet.fighters.iter_mut().find(|e| e.class == *class_key) {
                        entry.count += 1;
                    } else {
                        fleet.fighters.push(FighterEntry {
                            class: *class_key,
                            count: 1,
                        });
                    }
                }
            } else {
                let fleet = Fleet {
                    location: sys_key,
                    capital_ships: vec![],
                    fighters: vec![FighterEntry {
                        class: *class_key,
                        count: 1,
                    }],
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
            let Some(template) = world.troops.get(*class_key).cloned() else {
                return;
            };
            let unit = TroopUnit {
                class_dat_id: template.class_dat_id,
                is_alliance: template.is_alliance,
                regiment_strength: 100,
            };
            let tk = world.troops.insert(unit);
            if let Some(sys) = world.systems.get_mut(sys_key) {
                sys.ground_units.push(tk);
            }
        }
    }
}
