//! `PerceptionIntegrator` — centralizes effect application and telemetry emission.
//!
//! Phase 4 of Knesset Ereshkigal. The integrator translates ad-hoc system events
//! into world mutations and structured `GameEventRecord` telemetry.
//!
//! Architecture: simulation.rs orchestrates 17 system `advance()` calls and delegates
//! effect application + telemetry to the integrator. This keeps simulation.rs focused
//! on tick composition while the integrator owns the mutation/telemetry contract.
//!
//! All 17 simulation sections route through `PerceptionIntegrator` methods for both
//! world mutation and telemetry emission. simulation.rs is a thin tick orchestrator (~449 LOC).

use std::collections::HashMap;

use rebellion_core::ai::{AIAction, AIState};
use rebellion_core::betrayal::BetrayalEvent;
use rebellion_core::blockade::{BlockadeEvent, BlockadeSystem};
use rebellion_core::combat::{side_present, CombatForce, CombatSide, GroundCombatResult};
use rebellion_core::death_star::DeathStarEvent;
use rebellion_core::economy::{EconomyEvent, EconomyState};
use rebellion_core::events::{EventAction, FiredEvent, SkillField, SystemTag};
use rebellion_core::fog::RevealEvent;
use rebellion_core::game_events::{
    GameEventRecord, EVT_AI_ACTION, EVT_BETRAYAL_CHECK, EVT_BLOCKADE_ENDED, EVT_BLOCKADE_STARTED,
    EVT_BLOCKADE_SUPPORT_SHIFT, EVT_BLOCKADE_TROOP_DESTROYED, EVT_BOMBARDMENT, EVT_BUILD_COMPLETE,
    EVT_CAMPAIGN_SNAPSHOT, EVT_CAPTURE, EVT_CHARACTER_HEALTH, EVT_CHARACTER_KILLED,
    EVT_COMBAT_GROUND, EVT_COMBAT_SPACE, EVT_CONTROL_CHANGED, EVT_DESTROYED_ON_ARRIVAL,
    EVT_DS_CONSTRUCTION, EVT_DS_FIRED, EVT_DS_STATUS, EVT_ESCAPE, EVT_EVENT_FIRED,
    EVT_FLEET_ARRIVED, EVT_FOG_REVEALED, EVT_HQ_CAPTURED, EVT_INFORMANT_INTEL, EVT_JEDI_CHECK,
    EVT_JEDI_DISCOVERED, EVT_JEDI_TIER, EVT_MAINTENANCE_SHORTFALL, EVT_MANUFACTURING_IDLE,
    EVT_MISSION_DISPATCHED, EVT_MISSION_RESOLVED, EVT_NATURAL_DISASTER, EVT_RESEARCH_UNLOCKED,
    EVT_SABOTEUR_DETECTED, EVT_SIDE_CHANGE, EVT_TRAITOR_REVEALED, EVT_TROOP_MOVED,
    EVT_UNITS_DEPLOYED, EVT_UPRISING_BEGAN, EVT_UPRISING_CHECK, EVT_UPRISING_ENDED,
    EVT_UPRISING_INCIDENT, EVT_VICTORY, EVT_VICTORY_CHECK, SYS_AI, SYS_BETRAYAL, SYS_BLOCKADE,
    SYS_COMBAT, SYS_DEATH_STAR, SYS_ECONOMY, SYS_EVENTS, SYS_FOG, SYS_JEDI, SYS_MANUFACTURING,
    SYS_MISSIONS, SYS_MOVEMENT, SYS_REPAIR, SYS_RESEARCH, SYS_STORY, SYS_UPRISING, SYS_VICTORY,
};
use rebellion_core::ids::DatId;
use rebellion_core::ids::{CharacterKey, FleetKey, SystemKey, TroopKey};
use rebellion_core::jedi::{JediEvent, JediState};
use rebellion_core::manufacturing::{
    BuildableKind, CompletionEvent, ManufacturingState, ProductionArea, QueueItem,
};
use rebellion_core::missions::{
    MissionEffect, MissionFaction, MissionMember, MissionRefusal, MissionRequest, MissionResult,
    MissionState, MoveDestination,
};
use rebellion_core::movement::{
    apply_fleet_arrival, begin_fleet_transit, ArrivalEvent, MovementState,
};
use rebellion_core::repair::RepairEvent;
use rebellion_core::research::{ResearchResult, ResearchState};
use rebellion_core::stockpiles::StockpileState;
use rebellion_core::tactical::apply::apply_results;
use rebellion_core::tactical::BattleSession;
use rebellion_core::troop_transport::{RegimentArrivals, RegimentLeg, TroopTransportState};
use rebellion_core::uprising::{UprisingEvent, UprisingState};
use rebellion_core::victory::VictoryOutcome;
use rebellion_core::world::{
    CapitalShipClass, ControlKind, DefenseFacilityInstance, Fleet, GameWorld,
    ManufacturingFacilityInstance, ProductionFacilityInstance, ShipInstance, SpecialForceUnit,
    TroopUnit,
};

// ---------------------------------------------------------------------------
// Name resolution helpers (shared with simulation.rs)
// ---------------------------------------------------------------------------

/// Resolve a `SystemKey` to the system's name, or a fallback string.
#[must_use]
pub fn sys_name(world: &GameWorld, key: SystemKey) -> String {
    world
        .systems
        .get(key)
        .map_or_else(|| format!("{key:?}"), |s| s.name.clone())
}

/// A production destination's name: its system's, or its fleet's.
#[must_use]
pub fn destination_name(
    world: &GameWorld,
    destination: rebellion_core::manufacturing::Destination,
) -> String {
    match destination {
        rebellion_core::manufacturing::Destination::System(system) => sys_name(world, system),
        rebellion_core::manufacturing::Destination::Fleet(fleet) => world
            .fleet_name(fleet)
            .map_or_else(|| format!("{fleet:?}"), str::to_string),
    }
}

/// Resolve a `CharacterKey` to the character's name, or a fallback string.
#[must_use]
pub fn char_name(world: &GameWorld, key: CharacterKey) -> String {
    world
        .characters
        .get(key)
        .map_or_else(|| format!("{key:?}"), |c| c.name.clone())
}

/// Name a mission member: a character's name, or a special force's SPECFCSD
/// class id (units carry no name of their own).
#[must_use]
pub fn member_name(world: &GameWorld, member: MissionMember) -> String {
    match member {
        MissionMember::Character(key) => char_name(world, key),
        MissionMember::SpecialForce(key) => world.special_forces.get(key).map_or_else(
            || format!("{key:?}"),
            |unit| format!("special force {:#010x}", unit.class_dat_id.raw()),
        ),
    }
}

fn member_names(world: &GameWorld, members: &[MissionMember]) -> Vec<String> {
    members.iter().map(|&m| member_name(world, m)).collect()
}

/// Format an `AIAction` as a structured JSON payload with readable names.
#[must_use]
pub fn ai_action_json(action: &AIAction, world: &GameWorld) -> serde_json::Value {
    match action {
        AIAction::MoveFleet {
            fleet,
            to_system,
            reason,
            troops,
        } => {
            let faction = world.fleets.get(*fleet).map_or("unknown", |f| {
                if f.is_alliance {
                    "Alliance"
                } else {
                    "Empire"
                }
            });
            let from = world
                .fleets
                .get(*fleet)
                .map_or_else(|| "unknown".into(), |f| sys_name(world, f.location));
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
            team,
            decoys,
            target_system,
            ..
        } => {
            serde_json::json!({
                "type": "DispatchMission",
                "kind": format!("{:?}", kind),
                "target": sys_name(world, *target_system),
                "team": member_names(world, team),
                "decoys": member_names(world, decoys),
            })
        }
        AIAction::EnqueueProduction {
            system,
            kind,
            work,
            count,
            destination,
        } => {
            serde_json::json!({
                "type": "EnqueueProduction",
                "system": sys_name(world, *system),
                "kind": format!("{:?}", kind),
                "work": work,
                "count": count,
                "destination": destination.map(|d| destination_name(world, d)),
            })
        }
        AIAction::Scrap { targets } => {
            serde_json::json!({
                "type": "Scrap",
                "targets": targets.iter().map(|t| format!("{t:?}")).collect::<Vec<_>>(),
            })
        }
        AIAction::MoveCharacter {
            character,
            to_system,
        } => {
            serde_json::json!({
                "type": "MoveCharacter",
                "character": char_name(world, *character),
                "to": sys_name(world, *to_system),
            })
        }
        AIAction::MoveCharacterToFleet { character, fleet } => {
            serde_json::json!({
                "type": "MoveCharacterToFleet",
                "character": char_name(world, *character),
                "at": world
                    .fleets
                    .get(*fleet)
                    .map_or_else(|| "unknown".into(), |f| sys_name(world, f.location)),
            })
        }
        AIAction::CreateFleet {
            fleet,
            ships,
            to_system,
        } => {
            let from = world
                .fleets
                .get(*fleet)
                .map_or_else(|| "unknown".into(), |f| sys_name(world, f.location));
            serde_json::json!({
                "type": "CreateFleet",
                "from": from,
                "to": to_system.map(|s| sys_name(world, s)),
                "ships": ships.len(),
            })
        }
        AIAction::JoinFleet {
            fleet,
            ships,
            target,
        } => {
            let at = |key: &FleetKey| {
                world
                    .fleets
                    .get(*key)
                    .map_or_else(|| "unknown".into(), |f| sys_name(world, f.location))
            };
            serde_json::json!({
                "type": "JoinFleet",
                "from": at(fleet),
                "to": at(target),
                "ships": ships.len(),
            })
        }
        AIAction::MoveRegiments { troops, to } => {
            serde_json::json!({
                "type": "MoveRegiments",
                "regiments": troops.len(),
                "to": format!("{to:?}"),
            })
        }
        AIAction::MoveSquadrons { squadrons, fleet } => {
            let at = world
                .fleets
                .get(*fleet)
                .map_or_else(|| "unknown".into(), |f| sys_name(world, f.location));
            serde_json::json!({
                "type": "MoveSquadrons",
                "squadrons": squadrons.len(),
                "at": at,
            })
        }
        AIAction::MoveCharacters {
            characters,
            to_system,
        } => {
            serde_json::json!({
                "type": "MoveCharacters",
                "characters": characters.iter().map(|c| char_name(world, *c)).collect::<Vec<_>>(),
                "to": sys_name(world, *to_system),
            })
        }
        AIAction::Assault { fleet } => {
            let at = world
                .fleets
                .get(*fleet)
                .map_or_else(|| "unknown".into(), |f| sys_name(world, f.location));
            serde_json::json!({ "type": "Assault", "at": at })
        }
        AIAction::Bombard { system, kind, .. } => {
            serde_json::json!({
                "type": "Bombard",
                "at": sys_name(world, *system),
                "kind": format!("{kind:?}"),
            })
        }
        AIAction::DestroySystem { system, .. } => {
            serde_json::json!({ "type": "DestroySystem", "at": sys_name(world, *system) })
        }
        AIAction::GiveCommand { character, kind } => {
            serde_json::json!({
                "type": "GiveCommand",
                "character": char_name(world, *character),
                "kind": format!("{:?}", kind),
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
    /// `MessageLog` / special-forces arena. Headless `simulation.rs`
    /// leaves the queue alone and `finish()` discards it.
    story_effects: Vec<rebellion_core::effects::GameEffect>,
}

impl PerceptionIntegrator {
    /// Create a new integrator for a single simulation tick.
    #[must_use]
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
    #[must_use]
    pub fn tick(&self) -> u64 {
        self.tick
    }

    /// Wall-clock milliseconds.
    #[must_use]
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
    #[must_use]
    pub fn finish(self) -> Vec<GameEventRecord> {
        self.events
    }

    /// The records `emit` writes, stamped `tick` in place of this
    /// integrator's own: the AI rounds a tick's prologue finishes belong to
    /// the day they ran in.
    pub fn at_tick(&mut self, tick: u64, emit: impl FnOnce(&mut Self)) {
        let own = std::mem::replace(&mut self.tick, tick);
        emit(self);
        self.tick = own;
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
        for (_, sys) in &world.systems {
            match sys.control {
                ControlKind::Controlled(rebellion_core::dat::Faction::Alliance) => {
                    alliance_systems += 1;
                }
                ControlKind::Controlled(rebellion_core::dat::Faction::Empire) => {
                    empire_systems += 1;
                }
                _ => neutral_systems += 1,
            }
        }

        // Build per-system economy data for parity eval
        let mut systems_map = serde_json::Map::new();
        for (key, sys) in &world.systems {
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
                }
                EconomyEvent::ControlResolved {
                    system,
                    new_control,
                } => {
                    if let Some(sys) = world.systems.get_mut(*system) {
                        sys.control = *new_control;
                    }
                    // FUN_00510d70 → FUN_004f6f40: the system's facilities
                    // follow its new holder.
                    world.hand_over_facilities(*system);
                    self.emit(
                        SYS_ECONOMY,
                        EVT_CONTROL_CHANGED,
                        crate::telemetry::control_resolved_payload(world, *system, *new_control),
                    );
                }
                _ => {}
            }
            if let Some((event, details)) = crate::telemetry::economy_record(world, ev) {
                self.emit(SYS_ECONOMY, event, details);
            }
        }
    }

    /// One record per unit the overdraft timer scraps (`FUN_00530350`),
    /// the original's event `0x304` (`FUN_004fc080`, reason `0x15`) behind
    /// the Maintenance Shortfall notice (`FUN_00498970`). Call it before
    /// the scrap removes the units.
    pub fn emit_maintenance_shortfall(
        &mut self,
        world: &GameWorld,
        transport: &rebellion_core::troop_transport::TroopTransportState,
        events: &[rebellion_core::stockpiles::StockpileEvent],
    ) {
        for event in events {
            if let rebellion_core::stockpiles::StockpileEvent::OverdraftScrap { side, target } =
                *event
            {
                let system = rebellion_core::scrap::target_system(world, transport, target);
                self.emit(
                    SYS_ECONOMY,
                    EVT_MAINTENANCE_SHORTFALL,
                    serde_json::json!({
                        "faction": format!("{side:?}"),
                        "system": system.map(|key| sys_name(world, key)),
                        "unit": rebellion_core::scrap::target_name(world, target),
                    }),
                );
            }
        }
    }

    // ── Step 3: Manufacturing + Movement ──────────────────────────────────

    /// Apply build completions: add manufactured items to `GameWorld` + emit telemetry.
    ///
    /// Emits two telemetry records per completion — `EVT_BUILD_COMPLETE`
    /// (the "construction finished" signal test harnesses use) and `EVT_UNITS_DEPLOYED` (0x107, Knesset
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

    /// Place products delivered to a fleet (`crate::delivery`), with the
    /// same telemetry as [`Self::apply_build_completions`].
    pub fn apply_boarded(
        &mut self,
        world: &mut GameWorld,
        boarded: &[(FleetKey, CompletionEvent)],
    ) {
        for (fleet, c) in boarded {
            apply_boarding_inner(*fleet, c, world);
            let payload = serde_json::json!({
                "system": sys_name(world, c.system),
                "fleet": world.fleet_name(*fleet),
                "kind": format!("{:?}", c.kind),
            });
            self.emit(SYS_MANUFACTURING, EVT_BUILD_COMPLETE, payload.clone());
            self.emit(SYS_MANUFACTURING, EVT_UNITS_DEPLOYED, payload);
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

    /// Report manufactured objects lost on arrival: their destination was
    /// destroyed while they travelled (event `0x303`, `FUN_004fc080`).
    pub fn apply_deliveries_lost(
        &mut self,
        world: &GameWorld,
        lost: &[rebellion_core::delivery::Delivery],
    ) {
        for delivery in lost {
            self.emit(
                SYS_MANUFACTURING,
                EVT_DESTROYED_ON_ARRIVAL,
                serde_json::json!({
                    "origin": sys_name(world, delivery.origin),
                    "destination": sys_name(world, delivery.destination),
                    "kind": format!("{:?}", delivery.kind),
                }),
            );
        }
    }

    /// Apply fleet arrivals: update locations + emit telemetry.
    pub fn apply_arrivals(
        &mut self,
        world: &mut GameWorld,
        movement: &MovementState,
        troop_transport: &mut TroopTransportState,
        arrivals: &[ArrivalEvent],
    ) {
        for arrival in arrivals {
            let Some(applied) = apply_fleet_arrival(world, movement, troop_transport, arrival)
            else {
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

    /// Emit one summary event after a complete system-level space engagement.
    pub fn emit_system_space_combat(
        &mut self,
        world: &GameWorld,
        result: &SystemSpaceCombatResult,
    ) {
        self.emit(
            SYS_COMBAT,
            EVT_COMBAT_SPACE,
            crate::telemetry::combat_space_payload(world, result),
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

    /// Record regiments whose own transit ended (`FUN_00556430`, event
    /// `0x387`): one movement record each, arrived or lost on arrival
    /// (`FUN_004fc080`).
    pub fn apply_regiment_arrivals(&mut self, world: &GameWorld, ended: &RegimentArrivals) {
        let records = ended
            .arrived
            .iter()
            .map(|transit| (transit, "arrived"))
            .chain(
                ended
                    .lost
                    .iter()
                    .map(|transit| (transit, "lost_on_arrival")),
            );
        for (transit, status) in records {
            let destination = match transit.leg {
                RegimentLeg::Surface(system) => sys_name(world, system),
                RegimentLeg::Fleet(fleet) => format!("{fleet:?}"),
            };
            self.emit(
                SYS_MOVEMENT,
                EVT_TROOP_MOVED,
                serde_json::json!({
                    "origin": sys_name(world, transit.origin),
                    "destination": destination,
                    "regiments": 1,
                    "status": status,
                }),
            );
        }
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
        // FUN_00510d70 → FUN_004f6f40: the system's facilities follow its
        // new holder.
        world.hand_over_facilities(system);
        if previous != Some(ControlKind::Controlled(winner)) {
            self.emit(
                SYS_COMBAT,
                EVT_CONTROL_CHANGED,
                crate::telemetry::control_occupation_payload(world, system, previous, winner),
            );
        }

        let captives: Vec<_> = world
            .characters
            .iter()
            .filter(|(_, character)| {
                // A fleet's crew is aboard the fleet, not on the world.
                character.current_system == Some(system)
                    && character.current_fleet.is_none()
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
        research_state: &mut ResearchState,
    ) {
        let ended =
            apply_mission_effects_inner(&result.effects, world, result.tick, uprising_state);
        let research = rebellion_core::research::ResearchSystem::apply_mission_points(
            world,
            research_state,
            &result.effects,
        );
        self.apply_research_results(&research);
        for event in &ended {
            if let UprisingEvent::UprisingEnded { system, tick } = event {
                self.events.push(GameEventRecord::new(
                    *tick,
                    self.wall_ms,
                    SYS_UPRISING,
                    EVT_UPRISING_ENDED,
                    serde_json::json!({ "system": sys_name(world, *system), "cause": "subdued" }),
                ));
            }
        }
        self.emit(
            SYS_MISSIONS,
            EVT_MISSION_RESOLVED,
            crate::telemetry::mission_resolved_payload(world, result),
        );
        // Covert missions are espionage operations — emit an Espionage wrapper
        // so the eval harness sees all 8 mission kinds.
        if let Some(payload) = crate::telemetry::mission_espionage_payload(world, result) {
            self.emit(SYS_MISSIONS, EVT_MISSION_RESOLVED, payload);
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
                MissionEffect::TargetSabotaged { system, .. } => {
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
                for (_, fleet) in &mut world.fleets {
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
    /// into a render-layer `MessageLog` should call `drain_story_effects()`
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

    /// One AI order's `ai_action` record, and for an applied dispatch its
    /// `mission_dispatched` record.
    pub fn emit_ai_order(
        &mut self,
        world: &GameWorld,
        action: &AIAction,
        origin: &crate::telemetry::AiOrderOrigin<'_>,
        outcome: &AiOutcome,
    ) {
        self.emit(
            SYS_AI,
            EVT_AI_ACTION,
            crate::telemetry::ai_order_payload(world, action, origin, &outcome.view()),
        );
        if let (
            AIAction::DispatchMission {
                kind,
                team,
                decoys,
                target_system,
                ..
            },
            AiOutcome::Applied { mission },
        ) = (action, outcome)
        {
            self.emit(
                SYS_MISSIONS,
                EVT_MISSION_DISPATCHED,
                serde_json::json!({
                    "mission_id": mission,
                    "kind": format!("{kind:?}"),
                    "faction": origin.faction,
                    "team": member_names(world, team),
                    "decoys": member_names(world, decoys),
                    "target_system": sys_name(world, *target_system),
                }),
            );
        }
    }

    /// The records of what an AI strike did: the bombardment, the planet
    /// destroyed and the characters killed with it.
    pub fn emit_strike(&mut self, world: &GameWorld, strike: &StrikeEffects) {
        if let Some((system, damage, headquarters_destroyed)) = strike.bombardment {
            self.emit_bombardment(world, system, damage, headquarters_destroyed);
        }
        if let Some(system) = strike.destroyed {
            self.emit(
                SYS_DEATH_STAR,
                EVT_DS_FIRED,
                serde_json::json!({ "system": sys_name(world, system) }),
            );
            self.emit_killed(world, system, &strike.killed);
        }
    }

    /// One `EVT_CHARACTER_KILLED` record per character the Death Star
    /// killed at `system`.
    pub fn emit_killed(&mut self, world: &GameWorld, system: SystemKey, killed: &[CharacterKey]) {
        let system_name = sys_name(world, system);
        for &character in killed {
            let (name, dat_id) = world.characters.get(character).map_or_else(
                || (String::from("<unknown>"), 0),
                |c| (c.name.clone(), c.dat_id.raw()),
            );
            self.emit(
                SYS_MISSIONS,
                rebellion_core::game_events::EVT_CHARACTER_KILLED,
                serde_json::json!({
                    "name": name,
                    "dat_id": dat_id,
                    "cause": "death_star",
                    "system": system_name,
                }),
            );
        }
    }

    // ── Step 8: Blockade ──────────────────────────────────────────────────

    /// Apply blockade events: troop destruction + telemetry.
    pub fn apply_blockade_events(
        &mut self,
        world: &mut GameWorld,
        transport: &mut TroopTransportState,
        events: &[BlockadeEvent],
    ) {
        for evt in events {
            match evt {
                BlockadeEvent::BlockadeStarted { system, tick } => {
                    self.events.push(GameEventRecord::new(
                        *tick,
                        self.wall_ms,
                        SYS_BLOCKADE,
                        EVT_BLOCKADE_STARTED,
                        crate::telemetry::blockade_payload(world, *system),
                    ));
                }
                BlockadeEvent::BlockadeEnded { system, tick } => {
                    self.events.push(GameEventRecord::new(
                        *tick,
                        self.wall_ms,
                        SYS_BLOCKADE,
                        EVT_BLOCKADE_ENDED,
                        crate::telemetry::blockade_payload(world, *system),
                    ));
                }
                BlockadeEvent::TroopDestroyed {
                    system,
                    troop,
                    tick,
                } => {
                    // The regiment was aboard a fleet leaving the system.
                    if transport.destroy_embarked(world, *troop) {
                        self.events.push(GameEventRecord::new(
                            *tick,
                            self.wall_ms,
                            SYS_BLOCKADE,
                            EVT_BLOCKADE_TROOP_DESTROYED,
                            crate::telemetry::blockade_payload(world, *system),
                        ));
                    }
                }
                BlockadeEvent::SupportShift {
                    system,
                    side,
                    delta,
                    tick,
                } => {
                    // FUN_0050c9f0 through the port's setter (the Death Star
                    // halving and the 0..100 clamp).
                    rebellion_core::uprising::apply_support_change(world, *system, *side, *delta);
                    self.events.push(GameEventRecord::new(
                        *tick,
                        self.wall_ms,
                        SYS_BLOCKADE,
                        EVT_BLOCKADE_SUPPORT_SHIFT,
                        crate::telemetry::support_shift_payload(world, *system, *side, *delta),
                    ));
                }
            }
        }
    }

    // ── Step 9: Uprising ──────────────────────────────────────────────────

    /// Apply uprising and disaster events to the world, with telemetry.
    /// Revolts never change who holds a system (`FUN_0050a130`).
    pub fn apply_uprising_events(&mut self, world: &mut GameWorld, events: &[UprisingEvent]) {
        // Heartbeat: emit a check event so the "uprising" system tag always appears.
        self.events.push(GameEventRecord::new(
            self.tick, self.wall_ms, SYS_UPRISING, EVT_UPRISING_CHECK,
            serde_json::json!({ "systems_checked": world.systems.len(), "incidents": events.len() }),
        ));
        for evt in events {
            let (tick, event_type, payload) = match evt {
                UprisingEvent::UprisingIncident {
                    system,
                    tick,
                    codes,
                    losses,
                    support_delta,
                    ..
                } => (
                    *tick,
                    EVT_UPRISING_INCIDENT,
                    serde_json::json!({
                        "system": sys_name(world, *system),
                        "codes": codes,
                        "losses": losses.len(),
                        "support_delta": support_delta,
                    }),
                ),
                UprisingEvent::UprisingBegan { system, tick } => (
                    *tick,
                    EVT_UPRISING_BEGAN,
                    serde_json::json!({ "system": sys_name(world, *system) }),
                ),
                UprisingEvent::UprisingEnded { system, tick } => (
                    *tick,
                    EVT_UPRISING_ENDED,
                    serde_json::json!({ "system": sys_name(world, *system) }),
                ),
                UprisingEvent::Disaster {
                    system,
                    tick,
                    destroyed,
                    ..
                } => (
                    *tick,
                    EVT_NATURAL_DISASTER,
                    serde_json::json!({
                        "system": sys_name(world, *system),
                        "facilities_destroyed": destroyed.len(),
                    }),
                ),
            };
            self.events.push(GameEventRecord::new(
                tick,
                self.wall_ms,
                SYS_UPRISING,
                event_type,
                payload,
            ));
            rebellion_core::uprising::apply_uprising_event(world, evt);
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
            for (_, fleet) in &mut world.fleets {
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

    /// Research telemetry: the levels the sides bought this tick.
    pub fn apply_research_results(&mut self, results: &[ResearchResult]) {
        for result in results {
            self.emit(
                SYS_RESEARCH,
                EVT_RESEARCH_UNLOCKED,
                crate::telemetry::research_unlocked_payload(result),
            );
        }
    }

    // ── Step 12b: Repair ───────────────────────────────────────────────────

    /// Apply repair events: hull, counters and timers, plus telemetry.
    pub fn apply_repair_events(&mut self, world: &mut GameWorld, events: &[RepairEvent]) {
        for evt in events {
            evt.apply(world);
            if let Some((event_type, details)) = crate::telemetry::repair_payload(world, evt) {
                self.emit(SYS_REPAIR, event_type, details);
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

#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
fn apply_mission_effects_inner(
    effects: &[MissionEffect],
    world: &mut GameWorld,
    tick: u64,
    uprising_state: &mut UprisingState,
) -> Vec<UprisingEvent> {
    for effect in effects {
        match effect {
            MissionEffect::SupportGained {
                system,
                side,
                points,
            } => rebellion_core::uprising::apply_support_change(world, *system, *side, *points),
            MissionEffect::SkillRaised {
                character,
                skill,
                amount,
            } => rebellion_core::missions::raise_skill(world, *character, *skill, *amount),
            // Credited by ResearchSystem::apply_mission_points.
            MissionEffect::ResearchPoints { .. } => {}
            MissionEffect::UprisingIncident(event) => {
                rebellion_core::uprising::apply_uprising_event(world, event);
            }
            MissionEffect::SystemIntelligenceGathered { system, .. } => {
                if let Some(sys) = world.systems.get_mut(*system) {
                    sys.exploration_status = rebellion_core::dat::ExplorationStatus::Explored;
                }
            }
            MissionEffect::CharacterRecruited {
                character,
                system,
                faction,
                pool_emptied,
            } => rebellion_core::missions::recruit_character(
                world,
                *character,
                *system,
                *faction,
                *pool_emptied,
            ),
            MissionEffect::TargetSabotaged { target, .. } => {
                rebellion_core::missions::destroy_target(world, *target);
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
                for (_, fleet) in &mut world.fleets {
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
            } => rebellion_core::missions::capture_character(
                world,
                *character,
                *captured_by,
                *at_system,
            ),
            MissionEffect::SpecialForceDestroyed { unit } => {
                rebellion_core::missions::destroy_special_force(world, *unit);
            }
            // port: the port stores no injury (F-026).
            MissionEffect::CharacterInjured { .. } => {}
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
            MissionEffect::MemberAvailable { member } => {
                rebellion_core::missions::set_on_mission(world, *member, false);
            }
            MissionEffect::MemberMoved { member, to } => {
                rebellion_core::missions::move_member(world, *member, *to);
            }
            MissionEffect::MemberBoarded { member, fleet } => {
                rebellion_core::missions::board_member(world, *member, *fleet);
            }
            MissionEffect::EnRoute { member, en_route } => {
                rebellion_core::missions::set_en_route(world, *member, *en_route);
            }
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
            MissionEffect::UprisingSubdued {
                system,
                side,
                support_gain,
            } => {
                rebellion_core::uprising::apply_support_change(
                    world,
                    *system,
                    *side,
                    *support_gain,
                );
            }
        }
    }
    apply_mission_state_effects(effects, world, tick, uprising_state)
}

/// Apply the mission effects that live outside `GameWorld`, after the world
/// effects: a Subdue Uprising success ends the revolt when the system is
/// garrisoned (`FUN_0050c910`). Returns the revolts that ended. The native and browser app shares this
/// with the headless integrator.
pub fn apply_mission_state_effects(
    effects: &[MissionEffect],
    world: &GameWorld,
    tick: u64,
    uprising_state: &mut UprisingState,
) -> Vec<UprisingEvent> {
    let mut ended = Vec::new();
    for effect in effects {
        if let MissionEffect::UprisingSubdued { system, .. } = effect {
            ended.extend(rebellion_core::uprising::end_if_garrisoned(
                uprising_state,
                world,
                *system,
                tick,
            ));
        }
    }
    ended
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
#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
)]
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
                    let apply =
                        |v: u32, delta: i32| (i64::from(v) + i64::from(delta)).max(0) as u32;
                    match skill {
                        SkillField::Diplomacy => c.diplomacy.base = apply(c.diplomacy.base, d),
                        SkillField::Espionage => c.espionage.base = apply(c.espionage.base, d),
                        SkillField::ShipDesign => c.ship_design.base = apply(c.ship_design.base, d),
                        SkillField::TroopTraining => {
                            c.troop_training.base = apply(c.troop_training.base, d);
                        }
                        SkillField::FacilityDesign => {
                            c.facility_design.base = apply(c.facility_design.base, d);
                        }
                        SkillField::Combat => c.combat.base = apply(c.combat.base, d),
                        SkillField::Leadership => c.leadership.base = apply(c.leadership.base, d),
                        SkillField::Loyalty => c.loyalty.base = apply(c.loyalty.base, d),
                        SkillField::JediLevel => c.jedi_level.base = apply(c.jedi_level.base, d),
                    }
                }
            }
            EventAction::RelocateCharacter { .. }
            | EventAction::StartJediTraining { .. }
            | EventAction::TriggerEvent { .. } => {}
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
                for (_, fleet) in &mut world.fleets {
                    fleet.characters.retain(|&k| k != *character);
                }
                world.characters.remove(*character);
            }
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
                            rebellion_core::dat::Faction::Neutral => {}
                        }
                    }
                }
            }
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
                for (_, fleet) in &mut world.fleets {
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
                        for (fleet_key, fleet) in &world.fleets {
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
                            skills: [0; 8],
                            on_mission: false,
                            en_route: false,
                            on_hidden_mission: false,
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
                            "[shamash-bet] SpawnSpecialForce at character {character:?} \
                             could not resolve a target system — character has \
                             no current_system and is not on any movement-ordered \
                             fleet. Event chain should have gated this action with \
                             CharacterAtSystem OR CharacterAssignedToFleet.",
                        );
                    }
                }
            }
            EventAction::SetHeritageKnown { character } => {
                // Flip heritage_known when 0x396 "Final Battle Imminent"
                // fires. Nothing reads the flag; the story chains' port to
                // the original's rules is `or-port-story-research-c80`.
                //
                // Fail loudly on missing character: a silently-dropped flip
                // is a story-continuity bug that's hard to diagnose after
                // the fact. Matches
                // the `SpawnSpecialForce` structured-warn pattern above.
                if let Some(c) = world.characters.get_mut(*character) {
                    c.heritage_known = true;
                } else {
                    eprintln!(
                        "[shamash-bet] SetHeritageKnown target character {character:?} \
                         not found in arena — heritage_known flip dropped. \
                         Final Battle cutscene variant will not reflect the \
                         paternity reveal.",
                    );
                }
            }
        }
    }
}

/// Why the applier refused an AI order: the `refusal` of its `ai_action`
/// record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiRefusal {
    Mission(MissionRefusal),
    Command(rebellion_core::officer_rank::CommandRefusal),
    Fleet(rebellion_core::fleet_join::FleetOrderError),
    Troops(rebellion_core::troop_transport::TroopTransportError),
    /// port: an Assault with no regiments aboard releases nothing.
    NoCargo,
    /// A strike's system is gone.
    StrikeSystemMissing,
    /// The Death Star cannot fire there (`DeathStarSystem::fire`).
    DeathStarRefused,
}

impl AiRefusal {
    /// The refusal's name, `family/Variant`.
    #[must_use]
    pub fn name(&self) -> String {
        use rebellion_core::fleet_join::FleetOrderError as F;
        use rebellion_core::movement::FleetDispatchError as D;
        use rebellion_core::officer_rank::CommandRefusal as C;
        use rebellion_core::troop_transport::TroopTransportError as T;
        match self {
            Self::Mission(refusal) => format!("mission/{}", mission_refusal_name(refusal)),
            Self::Command(refusal) => format!(
                "command/{}",
                match refusal {
                    C::OtherSide => "OtherSide",
                    C::Unrecruited => "Unrecruited",
                    C::EnRoute => "EnRoute",
                    C::OnMission => "OnMission",
                    C::Prisoner => "Prisoner",
                    C::NotEligible => "NotEligible",
                }
            ),
            Self::Fleet(error) => format!(
                "fleet/{}",
                match error {
                    F::MissingFleet => "MissingFleet",
                    F::MissingShip => "MissingShip",
                    F::ShipsChanged => "ShipsChanged",
                    F::WrongFaction => "WrongFaction",
                    F::InTransit => "InTransit",
                    F::IntoItself => "IntoItself",
                    F::OtherSideDestination => "OtherSideDestination",
                    F::DestinationInTransit => "DestinationInTransit",
                    F::Dispatch(dispatch) => match dispatch {
                        D::MissingFleet => "Dispatch/MissingFleet",
                        D::MissingOrigin => "Dispatch/MissingOrigin",
                        D::MissingDestination => "Dispatch/MissingDestination",
                        D::DestinationDestroyed => "Dispatch/DestinationDestroyed",
                        D::WrongFaction => "Dispatch/WrongFaction",
                        D::AlreadyInTransit => "Dispatch/AlreadyInTransit",
                        D::AlreadyAtDestination => "Dispatch/AlreadyAtDestination",
                        D::NoHyperdrive => "Dispatch/NoHyperdrive",
                        D::Blockaded => "Dispatch/Blockaded",
                    },
                }
            ),
            Self::Troops(error) => format!(
                "troops/{}",
                match error {
                    T::MissingFleet => "MissingFleet",
                    T::MissingSystem => "MissingSystem",
                    T::NoTroops => "NoTroops",
                    T::MissingTroop => "MissingTroop",
                    T::WrongFaction => "WrongFaction",
                    T::TroopNotAtFleetSystem => "TroopNotAtFleetSystem",
                    T::AlreadyEmbarked => "AlreadyEmbarked",
                    T::CapacityExceeded { .. } => "CapacityExceeded",
                    T::FleetNotAtDestination => "FleetNotAtDestination",
                    T::OtherSideDestination => "OtherSideDestination",
                    T::DestinationDestroyed => "DestinationDestroyed",
                    T::DestinationBlockaded => "DestinationBlockaded",
                    T::DestinationInTransit => "DestinationInTransit",
                    T::RegimentEnRoute => "RegimentEnRoute",
                    T::MissingSquadron => "MissingSquadron",
                    T::SquadronEnRoute => "SquadronEnRoute",
                    T::NoSquadronRoom => "NoSquadronRoom",
                }
            ),
            Self::NoCargo => "troops/NoCargo".into(),
            Self::StrikeSystemMissing => "strike/MissingSystem".into(),
            Self::DeathStarRefused => "strike/DeathStarRefused".into(),
        }
    }

    /// The validator's two-word status of a mission refusal.
    #[must_use]
    pub fn status(&self) -> Option<[u32; 2]> {
        match self {
            Self::Mission(refusal) => {
                let (w0, w1) = refusal.status();
                Some([w0, w1])
            }
            _ => None,
        }
    }

    /// The running end a creation refusal pre-empts.
    #[must_use]
    pub fn end_code(&self) -> Option<u8> {
        match self {
            Self::Mission(MissionRefusal::TargetRefused(refusal)) => refusal.end_code(),
            _ => None,
        }
    }
}

/// A mission refusal's variant name; the target refusals as
/// `TargetRefused/<variant>`.
fn mission_refusal_name(refusal: &MissionRefusal) -> String {
    use rebellion_core::missions::TargetRefusal as R;
    match refusal {
        MissionRefusal::EmptyTeam => "EmptyTeam".into(),
        MissionRefusal::MemberUnavailable(_) => "MemberUnavailable".into(),
        MissionRefusal::MembersNotAllowed => "MembersNotAllowed".into(),
        MissionRefusal::RecruiterNotMajor => "RecruiterNotMajor".into(),
        MissionRefusal::NoResearchSkill => "NoResearchSkill".into(),
        MissionRefusal::DestinationRefused(_) => "DestinationRefused".into(),
        MissionRefusal::TargetRefused(target) => format!(
            "TargetRefused/{}",
            match target {
                R::TargetMissing => "TargetMissing",
                R::ContainerDestroyed => "ContainerDestroyed",
                R::TargetWrongSide => "TargetWrongSide",
                R::TargetDestroyed => "TargetDestroyed",
                R::TargetEnRoute => "TargetEnRoute",
                R::TargetElsewhere => "TargetElsewhere",
                R::ContainerUnpopulated => "ContainerUnpopulated",
                R::UprisingState => "UprisingState",
                R::PrisonerTarget => "PrisonerTarget",
                R::FreeTarget => "FreeTarget",
                R::TargetIsDeathStar => "TargetIsDeathStar",
                R::TargetNotDeathStar => "TargetNotDeathStar",
                R::SupportFull => "SupportFull",
                R::RecruitPoolEmpty => "RecruitPoolEmpty",
                R::NoYard => "NoYard",
                R::TreeExhausted => "TreeExhausted",
            }
        ),
    }
}

/// What became of one AI order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiOutcome {
    /// Applied; a dispatch names the mission it created.
    Applied {
        mission: Option<u64>,
    },
    Refused(AiRefusal),
    /// The order touches a system with an open battle (native only).
    Held {
        system: SystemKey,
    },
}

impl AiOutcome {
    #[must_use]
    pub const fn is_applied(&self) -> bool {
        matches!(self, Self::Applied { .. })
    }

    /// The outcome as the `ai_action` record reads it.
    #[must_use]
    pub fn view(&self) -> crate::telemetry::AiOutcomeView {
        match self {
            Self::Applied { mission } => crate::telemetry::AiOutcomeView {
                outcome: "applied",
                mission_id: *mission,
                ..Default::default()
            },
            Self::Refused(refusal) => crate::telemetry::AiOutcomeView {
                outcome: "refused",
                refusal: Some(refusal.name()),
                status: refusal.status(),
                end_code: refusal.end_code(),
                ..Default::default()
            },
            Self::Held { system } => crate::telemetry::AiOutcomeView {
                outcome: "held",
                held_at: Some(*system),
                ..Default::default()
            },
        }
    }
}

/// Apply one AI order to the world: the side's order executed in phase 5
/// (`FUN_0051df30`). Strikes go to [`resolve_ai_strike`].
#[expect(
    clippy::too_many_arguments,
    reason = "Keep the existing explicit simulation state inputs at this integration boundary."
)]
pub fn apply_ai_order(
    action: &AIAction,
    ai_state: &mut AIState,
    mission_state: &mut MissionState,
    mfg_state: &mut ManufacturingState,
    movement_state: &mut MovementState,
    troop_transport: &mut TroopTransportState,
    stockpiles: &mut StockpileState,
    world: &mut GameWorld,
    uprisings: &UprisingState,
    tick: u64,
) -> AiOutcome {
    let mission_faction = ai_state.faction.map_or(
        MissionFaction::Empire,
        rebellion_core::ai::AiFaction::as_mission_faction,
    );
    let applied = AiOutcome::Applied { mission: None };
    let refused = |refusal| AiOutcome::Refused(refusal);
    match action {
        AIAction::DispatchMission {
            kind,
            team,
            decoys,
            target_system,
            target_character,
            target_object,
        } => {
            let dispatched = mission_state.dispatch_validated(
                MissionRequest {
                    kind: *kind,
                    faction: mission_faction,
                    team: team.clone(),
                    decoys: decoys.clone(),
                    target_system: *target_system,
                    target_character: *target_character,
                    target_object: *target_object,
                    tick,
                },
                world,
                uprisings,
            );
            match dispatched {
                Ok(id) => {
                    for character in team.iter().chain(decoys).filter_map(|m| m.character()) {
                        ai_state.mark_busy(character);
                    }
                    AiOutcome::Applied { mission: Some(id) }
                }
                Err(refusal) => refused(AiRefusal::Mission(refusal)),
            }
        }
        AIAction::EnqueueProduction {
            system,
            kind,
            work,
            count,
            destination,
        } => {
            if let Some(destination) = destination {
                mfg_state.set_destination(*system, ProductionArea::of(*kind), *destination);
            }
            for _ in 0..*count {
                mfg_state.enqueue(*system, QueueItem::new(*kind, *work));
            }
            applied
        }
        AIAction::Scrap { targets } => {
            rebellion_core::scrap::scrap_all(
                world,
                stockpiles,
                troop_transport,
                targets.iter().copied(),
            );
            applied
        }
        AIAction::MoveCharacter {
            character,
            to_system,
        } => mission_state
            .dispatch_move(
                world,
                movement_state,
                uprisings,
                mission_faction,
                &[MissionMember::Character(*character)],
                MoveDestination::System(*to_system),
                tick,
            )
            .map_or_else(|refusal| refused(AiRefusal::Mission(refusal)), |_| applied),
        AIAction::MoveCharacterToFleet { character, fleet } => mission_state
            .dispatch_move(
                world,
                movement_state,
                uprisings,
                mission_faction,
                &[MissionMember::Character(*character)],
                MoveDestination::Fleet(*fleet),
                tick,
            )
            .map_or_else(|refusal| refused(AiRefusal::Mission(refusal)), |_| applied),
        AIAction::GiveCommand { character, kind } => {
            let is_alliance = mission_faction == MissionFaction::Alliance;
            match rebellion_core::officer_rank::command_refusal(
                world,
                *character,
                is_alliance,
                *kind,
            ) {
                Some(refusal) => refused(AiRefusal::Command(refusal)),
                None => {
                    rebellion_core::officer_rank::give_command(world, *character, *kind);
                    applied
                }
            }
        }
        AIAction::JoinFleet {
            fleet,
            ships,
            target,
        } => {
            let Some(mover) = rebellion_core::fleet_join::mover_of_tags(world, *fleet, ships)
            else {
                return refused(AiRefusal::Fleet(
                    rebellion_core::fleet_join::FleetOrderError::MissingFleet,
                ));
            };
            rebellion_core::fleet_join::join_fleet(
                world,
                movement_state,
                troop_transport,
                &mover,
                *target,
                mission_faction == MissionFaction::Alliance,
            )
            .map_or_else(|error| refused(AiRefusal::Fleet(error)), |_| applied)
        }
        AIAction::MoveRegiments { troops, to } => {
            let blockaded = BlockadeSystem::blockaded_now(world);
            let mut first_error = None;
            let mut moved = false;
            for troop in troops {
                match troop_transport.move_regiment(
                    world,
                    movement_state,
                    &blockaded,
                    *troop,
                    *to,
                    tick,
                ) {
                    Ok(_) => moved = true,
                    Err(error) => {
                        first_error.get_or_insert(error);
                    }
                }
            }
            // port: the original issues one order per regiment; the port's
            // order is applied when any of them moves.
            if moved {
                applied
            } else {
                refused(AiRefusal::Troops(first_error.unwrap_or(
                    rebellion_core::troop_transport::TroopTransportError::NoTroops,
                )))
            }
        }
        AIAction::MoveSquadrons { squadrons, fleet } => {
            let blockaded = BlockadeSystem::blockaded_now(world);
            let mut first_error = None;
            let mut moved = false;
            for source in squadrons {
                match troop_transport.move_squadron(
                    world,
                    movement_state,
                    &blockaded,
                    *source,
                    rebellion_core::troop_transport::RegimentTarget::Fleet(*fleet),
                    tick,
                ) {
                    Ok(_) => moved = true,
                    Err(error) => {
                        first_error.get_or_insert(error);
                    }
                }
            }
            if moved {
                applied
            } else {
                refused(AiRefusal::Troops(first_error.unwrap_or(
                    rebellion_core::troop_transport::TroopTransportError::NoTroops,
                )))
            }
        }
        AIAction::MoveCharacters {
            characters,
            to_system,
        } => {
            let members: Vec<MissionMember> = characters
                .iter()
                .map(|c| MissionMember::Character(*c))
                .collect();
            mission_state
                .dispatch_move(
                    world,
                    movement_state,
                    uprisings,
                    mission_faction,
                    &members,
                    rebellion_core::missions::MoveDestination::System(*to_system),
                    tick,
                )
                .map_or_else(|refusal| refused(AiRefusal::Mission(refusal)), |_| applied)
        }
        AIAction::Assault { fleet } => {
            let carrying = troop_transport.carried_count(*fleet) > 0;
            troop_transport.release(*fleet);
            if carrying {
                applied
            } else {
                refused(AiRefusal::NoCargo)
            }
        }
        // The strikes go to resolve_ai_strike, which holds the victory and
        // Death Star states.
        AIAction::Bombard { .. } => refused(AiRefusal::StrikeSystemMissing),
        AIAction::DestroySystem { .. } => refused(AiRefusal::DeathStarRefused),
        AIAction::CreateFleet {
            fleet,
            ships,
            to_system,
        } => rebellion_core::fleet_join::create_fleet_of_tags(
            world,
            movement_state,
            troop_transport,
            *fleet,
            ships,
            *to_system,
            mission_faction == MissionFaction::Alliance,
        )
        .map_or_else(|error| refused(AiRefusal::Fleet(error)), |_| applied),
        AIAction::MoveFleet {
            fleet,
            to_system,
            troops,
            ..
        } => {
            use rebellion_core::fleet_join::FleetOrderError;
            use rebellion_core::movement::FleetDispatchError;
            let Some(f) = world.fleets.get(*fleet) else {
                return refused(AiRefusal::Fleet(FleetOrderError::MissingFleet));
            };
            // FUN_004fda10: no speed, no hyperdrive.
            let transit =
                rebellion_core::movement::fleet_transit_ticks(f, world, f.location, *to_system);
            if !troops.is_empty() {
                if let Err(error) = troop_transport.embark(world, *fleet, troops) {
                    return refused(AiRefusal::Troops(error));
                }
            }
            let departed = transit.is_some_and(|ticks| {
                begin_fleet_transit(movement_state, world, *fleet, *to_system, ticks)
            });
            if departed {
                return applied;
            }
            if !troops.is_empty() {
                let origin = world.fleets.get(*fleet).map(|value| value.location);
                if let Some(origin) = origin {
                    let _ = troop_transport.disembark_all(world, *fleet, origin);
                }
            }
            refused(AiRefusal::Fleet(FleetOrderError::Dispatch(
                if transit.is_none() {
                    FleetDispatchError::NoHyperdrive
                } else {
                    FleetDispatchError::AlreadyInTransit
                },
            )))
        }
    }
}

/// What an AI strike did, for the caller's records and messages.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StrikeEffects {
    /// The bombarded system, the damage, and whether it destroyed the
    /// Alliance headquarters.
    pub bombardment: Option<(SystemKey, i32, bool)>,
    /// The system the Death Star destroyed.
    pub destroyed: Option<SystemKey>,
    /// The characters that died with it.
    pub killed: Vec<CharacterKey>,
}

/// Resolve an AI strike, a Bombard or a DestroySystem, against the world.
/// It writes no record; the caller emits from the effects.
#[expect(
    clippy::too_many_arguments,
    reason = "A strike reaches the victory, Death Star, movement, production and blockade states."
)]
pub fn resolve_ai_strike(
    action: &AIAction,
    world: &mut GameWorld,
    victory: &mut rebellion_core::victory::VictoryState,
    death_star: &mut rebellion_core::death_star::DeathStarState,
    movement: &mut MovementState,
    manufacturing: &mut ManufacturingState,
    blockade: &mut rebellion_core::blockade::BlockadeState,
    tick: u64,
) -> (AiOutcome, StrikeEffects) {
    let mut effects = StrikeEffects::default();
    let applied = AiOutcome::Applied { mission: None };
    match action {
        AIAction::Bombard { fleet, system, .. } => {
            // port: the two bombardment orders resolve alike
            // (`BombardmentSystem::resolve_bombardment`).
            let Some(attacker_is_alliance) = world.fleets.get(*fleet).map(|f| f.is_alliance) else {
                return (
                    AiOutcome::Refused(AiRefusal::Fleet(
                        rebellion_core::fleet_join::FleetOrderError::MissingFleet,
                    )),
                    effects,
                );
            };
            if !world.systems.contains_key(*system) {
                return (AiOutcome::Refused(AiRefusal::StrikeSystemMissing), effects);
            }
            let result = rebellion_core::bombardment::BombardmentSystem::resolve_bombardment(
                world,
                *fleet,
                *system,
                world.difficulty_index,
                tick,
            );
            let attacker = if attacker_is_alliance {
                rebellion_core::dat::Faction::Alliance
            } else {
                rebellion_core::dat::Faction::Empire
            };
            let headquarters_destroyed =
                rebellion_core::victory::VictorySystem::apply_headquarters_bombardment(
                    victory, world, &result, attacker,
                );
            effects.bombardment = Some((*system, result.damage, headquarters_destroyed));
            // FUN_0058e320. +0x28 is set by a hit that destroys a civilian
            // facility (FUN_0058ce70) or lowers energy or raw material
            // (FUN_0058cf20, FUN_0058cf50). port: the port's bombardment
            // rolls no hits, so the flag stays clear; once the hit loop
            // (FUN_0058df60) is ported it only has to set it here.
            rebellion_core::bombardment::finalize(
                world,
                *system,
                attacker,
                rebellion_core::bombardment::BombardmentRun::default(),
            );
            (applied, effects)
        }
        AIAction::DestroySystem { fleet, system } => {
            let side = world
                .fleets
                .get(*fleet)
                .map_or(rebellion_core::dat::Faction::Empire, |f| {
                    if f.is_alliance {
                        rebellion_core::dat::Faction::Alliance
                    } else {
                        rebellion_core::dat::Faction::Empire
                    }
                });
            let fired =
                rebellion_core::death_star::DeathStarSystem::fire(death_star, world, *system, tick);
            if fired.is_none() {
                return (AiOutcome::Refused(AiRefusal::DeathStarRefused), effects);
            }
            effects.killed = destroy_system(
                world,
                *system,
                victory,
                death_star,
                movement,
                manufacturing,
                blockade,
            );
            effects.destroyed = Some(*system);
            // FUN_0058e320 after a kill (FUN_0058d990 sets +0x28 and +0x2c).
            rebellion_core::bombardment::finalize(
                world,
                *system,
                side,
                rebellion_core::bombardment::BombardmentRun {
                    support_loss: true,
                    planet_destroyed: true,
                },
            );
            (applied, effects)
        }
        _ => (AiOutcome::Refused(AiRefusal::StrikeSystemMissing), effects),
    }
}

/// A planet the Death Star destroyed: its flag, the victory record and the
/// cleanup (`cleanup_destroyed_system`). Returns the characters killed.
pub fn destroy_system(
    world: &mut GameWorld,
    system: SystemKey,
    victory: &mut rebellion_core::victory::VictoryState,
    death_star: &mut rebellion_core::death_star::DeathStarState,
    movement: &mut MovementState,
    manufacturing: &mut ManufacturingState,
    blockade: &mut rebellion_core::blockade::BlockadeState,
) -> Vec<CharacterKey> {
    if let Some(sys) = world.systems.get_mut(system) {
        sys.is_destroyed = true;
    }
    victory.death_star_location = Some(system);
    let mut cleanup_effects: Vec<rebellion_core::effects::GameEffect> = Vec::new();
    rebellion_core::death_star::cleanup_destroyed_system(
        world,
        system,
        movement,
        death_star,
        manufacturing,
        blockade,
        &mut cleanup_effects,
    );
    cleanup_effects
        .into_iter()
        .filter_map(|effect| match effect {
            rebellion_core::effects::GameEffect::CharacterKilled { character } => Some(character),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::ai::{AiFaction, FleetMoveReason};
    use rebellion_core::dat::{ExplorationStatus, Faction, SectorGroup};
    use rebellion_core::missions::{MissionKind, MissionOutcome};
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
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Empire),
        })
    }

    #[test]
    fn each_ended_regiment_transit_records_one_movement_line() {
        let mut world = GameWorld::default();
        let origin = add_system(&mut world, "Kuat");
        let destination = add_system(&mut world, "Naboo");
        let troop = world.troops.insert(rebellion_core::world::TroopUnit {
            class_dat_id: DatId(0x1000_0001),
            is_alliance: false,
            regiment_strength: 100,
        });
        let transit = |leg| rebellion_core::troop_transport::RegimentTransit {
            troop,
            origin,
            leg,
            arrival_tick: 5,
        };
        let ended = RegimentArrivals {
            arrived: vec![transit(RegimentLeg::Surface(destination))],
            lost: vec![transit(RegimentLeg::Surface(destination))],
            ..RegimentArrivals::default()
        };
        let mut integrator = PerceptionIntegrator::new(5, 0);

        integrator.apply_regiment_arrivals(&world, &ended);

        let records = integrator.finish();
        let statuses: Vec<_> = records
            .iter()
            .map(|record| (record.event_type, record.details["status"].clone()))
            .collect();
        assert_eq!(
            statuses,
            [
                (EVT_TROOP_MOVED, serde_json::json!("arrived")),
                (EVT_TROOP_MOVED, serde_json::json!("lost_on_arrival")),
            ]
        );
        assert_eq!(records[0].details["destination"], "Naboo");
        assert_eq!(records[0].details["origin"], "Kuat");
    }

    #[test]
    fn a_subdue_success_at_a_garrisoned_system_ends_the_uprising() {
        // FUN_00569c20 runs FUN_0050c910 after a Subdue Uprising success; with
        // no GNPRTB rows the garrison requirement is 0, so it ends.
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Naboo");
        let mut uprisings = UprisingState::new();
        uprisings.active_uprisings.insert(
            system,
            rebellion_core::uprising::ActiveUprising {
                started_tick: 0,
                next_incident_tick: None,
            },
        );
        let ended = apply_mission_state_effects(
            &[MissionEffect::UprisingSubdued {
                system,
                side: Faction::Empire,
                support_gain: 5,
            }],
            &world,
            9,
            &mut uprisings,
        );

        assert_eq!(
            ended,
            vec![UprisingEvent::UprisingEnded { system, tick: 9 }]
        );
        assert!(!uprisings.is_uprising(system));
    }

    #[test]
    fn a_subdue_gain_counts_toward_the_garrison_check_that_follows_it() {
        // FUN_00569c20 adds the FUN_0055cb10 gain before FUN_0050c910. At
        // Alliance support 40 the requirement is ceil((60 - 40) / 10) = 2, so
        // one regiment falls short; a gain of 10 makes it ceil(10 / 10) = 1.
        for (support_gain, ends) in [(10, true), (0, false)] {
            let mut world = GameWorld::default();
            world.gnprtb = rebellion_core::world::GnprtbParams::new(
                [(7761, 60), (7762, -10)]
                    .into_iter()
                    .map(|(id, value)| rebellion_core::world::GnprtbEntry {
                        parameter_id: id,
                        development: value,
                        alliance_sp_easy: value,
                        alliance_sp_medium: value,
                        alliance_sp_hard: value,
                        empire_sp_easy: value,
                        empire_sp_medium: value,
                        empire_sp_hard: value,
                        multiplayer: value,
                    })
                    .collect(),
            );
            let system = add_system(&mut world, "Naboo");
            world.systems[system].control = ControlKind::Controlled(Faction::Alliance);
            world.systems[system].popularity_alliance = 0.4;
            world.systems[system].popularity_empire = 0.6;
            let regiment = world.troops.insert(rebellion_core::world::TroopUnit {
                class_dat_id: DatId(0x1000_0002),
                is_alliance: true,
                regiment_strength: 1,
            });
            world.systems[system].ground_units.push(regiment);
            let agent = world
                .characters
                .insert(rebellion_core::world::Character::default());
            let mut uprisings = UprisingState::new();
            uprisings.active_uprisings.insert(
                system,
                rebellion_core::uprising::ActiveUprising {
                    started_tick: 0,
                    next_incident_tick: None,
                },
            );
            let result = MissionResult {
                mission_id: 1,
                tick: 9,
                kind: MissionKind::SubdueUprising,
                faction: MissionFaction::Alliance,
                character: Some(agent),
                target_system: system,
                outcome: MissionOutcome::Success,
                effects: vec![MissionEffect::UprisingSubdued {
                    system,
                    side: Faction::Alliance,
                    support_gain,
                }],
            };
            let mut integrator = PerceptionIntegrator::new(9, 0);
            integrator.apply_mission_result(
                &mut world,
                &result,
                &mut uprisings,
                &mut ResearchState::new(),
            );
            assert_eq!(uprisings.is_uprising(system), !ends, "gain {support_gain}");
        }
    }

    #[test]
    fn a_subdue_mission_result_raises_support_and_reports_the_revolt_it_ends() {
        // FUN_00569c20: support += FUN_0055cb10, then FUN_0050c910 ends the
        // revolt; with no GNPRTB rows the garrison requirement is 0.
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Naboo");
        world.systems[system].popularity_alliance = 0.4;
        world.systems[system].popularity_empire = 0.6;
        let agent = world
            .characters
            .insert(rebellion_core::world::Character::default());
        let mut uprisings = UprisingState::new();
        uprisings.active_uprisings.insert(
            system,
            rebellion_core::uprising::ActiveUprising {
                started_tick: 0,
                next_incident_tick: None,
            },
        );
        let result = MissionResult {
            mission_id: 1,
            tick: 9,
            kind: MissionKind::SubdueUprising,
            faction: MissionFaction::Empire,
            character: Some(agent),
            target_system: system,
            outcome: MissionOutcome::Success,
            effects: vec![MissionEffect::UprisingSubdued {
                system,
                side: Faction::Empire,
                support_gain: 5,
            }],
        };
        let mut integrator = PerceptionIntegrator::new(9, 0);
        integrator.apply_mission_result(
            &mut world,
            &result,
            &mut uprisings,
            &mut ResearchState::new(),
        );

        assert!((world.systems[system].popularity_empire - 0.65).abs() < 1e-6);
        assert!(!uprisings.is_uprising(system));
        assert!(integrator
            .events
            .iter()
            .any(|e| e.event_type == EVT_UPRISING_ENDED && e.tick == 9));
    }

    #[test]
    fn applying_a_disaster_sets_the_eroded_resources_and_records_it() {
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Naboo");
        world.systems[system].total_energy = 5;
        world.systems[system].raw_materials = 5;
        let mut integrator = PerceptionIntegrator::new(4, 0);
        integrator.apply_uprising_events(
            &mut world,
            &[UprisingEvent::Disaster {
                system,
                tick: 4,
                total_energy: 2,
                raw_materials: 1,
                destroyed: vec![],
            }],
        );
        assert_eq!(world.systems[system].total_energy, 2);
        assert_eq!(world.systems[system].raw_materials, 1);
        assert!(integrator
            .events
            .iter()
            .any(|e| e.event_type == EVT_NATURAL_DISASTER));
    }

    /// An Empire-held system with a yard both sides can run and a defense
    /// only the Empire can field.
    fn empire_facilities(world: &mut GameWorld, name: &str) -> SystemKey {
        let system = add_system(world, name);
        for (id, is_alliance) in [(0x2900_0001, true), (0x2200_0001, false)] {
            world.buildable_classes.insert(
                DatId(id),
                rebellion_core::world::BuildableClass {
                    is_alliance,
                    is_empire: true,
                    ..Default::default()
                },
            );
        }
        let yard = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: DatId(0x2900_0001),
                side: Faction::Empire,
                is_shipyard: false,
            });
        let defense =
            world
                .defense_facilities
                .insert(rebellion_core::world::DefenseFacilityInstance {
                    class_dat_id: DatId(0x2200_0001),
                    side: Faction::Empire,
                });
        world.systems[system].manufacturing_facilities.push(yard);
        world.systems[system].defense_facilities.push(defense);
        system
    }

    fn assert_alliance_took_over(world: &GameWorld, system: SystemKey) {
        let held = &world.systems[system];
        assert!(
            held.defense_facilities.is_empty(),
            "the Empire-only defense is removed"
        );
        let sides: Vec<Faction> = held
            .manufacturing_facilities
            .iter()
            .map(|&k| world.manufacturing_facilities[k].side)
            .collect();
        assert_eq!(sides, vec![Faction::Alliance]);
    }

    #[test]
    fn a_resolved_control_change_hands_facilities_to_the_new_holder() {
        // FUN_00510d70 → FUN_004f6f40 → FUN_004fae40: a facility whose class
        // serves the new holder passes to it, any other is removed
        // (ghidra/notes/facility-ownership.md).
        let mut world = GameWorld::default();
        let system = empire_facilities(&mut world, "Kuat");
        let mut integrator = PerceptionIntegrator::new(1, 0);
        integrator.apply_economy_events(
            &mut world,
            &[EconomyEvent::ControlResolved {
                system,
                new_control: ControlKind::Controlled(Faction::Alliance),
            }],
        );
        assert_alliance_took_over(&world, system);
    }

    #[test]
    fn a_ground_occupation_hands_facilities_to_the_victor() {
        // Same path as a resolved control change (FUN_00510d70).
        let mut world = GameWorld::default();
        let system = empire_facilities(&mut world, "Kuat");
        let mut integrator = PerceptionIntegrator::new(1, 0);
        integrator.apply_ground_occupation(&mut world, system, Faction::Alliance, 1);
        assert_alliance_took_over(&world, system);
    }

    #[test]
    fn support_drift_moves_both_sides_popularity() {
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Naboo");
        let mut integrator = PerceptionIntegrator::new(1, 0);
        integrator.apply_economy_events(
            &mut world,
            &[EconomyEvent::SupportDrifted {
                system,
                alliance_delta: 0.25,
                empire_delta: -0.25,
            }],
        );
        assert!((world.systems[system].popularity_alliance - 0.25).abs() < 1e-6);
        assert!((world.systems[system].popularity_empire - 0.75).abs() < 1e-6);
    }

    /// Apply each order for the Empire AI and write its record, as the round
    /// driver does (`simulation::ai_step`).
    fn apply_empire_orders(
        integrator: &mut PerceptionIntegrator,
        world: &mut GameWorld,
        movement: &mut MovementState,
        troop_transport: &mut TroopTransportState,
        actions: &[AIAction],
    ) -> Vec<AiOutcome> {
        let mut ai = AIState::new(AiFaction::Empire);
        let mut missions = MissionState::new();
        let mut manufacturing = ManufacturingState::new();
        let mut stockpiles = StockpileState::new();
        let origin = crate::telemetry::AiOrderOrigin {
            faction: "Empire",
            module: 0,
            round: 0,
            issued: 5,
            dual: false,
        };
        actions
            .iter()
            .map(|action| {
                let outcome = apply_ai_order(
                    action,
                    &mut ai,
                    &mut missions,
                    &mut manufacturing,
                    movement,
                    troop_transport,
                    &mut stockpiles,
                    world,
                    &UprisingState::default(),
                    5,
                );
                integrator.emit_ai_order(world, action, &origin, &outcome);
                outcome
            })
            .collect()
    }

    /// Every AI order writes its outcome record (one `ai_action` per order,
    /// applied, refused with its reason, or held).
    #[test]
    fn a_second_move_of_a_departed_fleet_is_reported_refused() {
        let mut world = GameWorld::default();
        let origin = add_system(&mut world, "Origin");
        let first_target = add_system(&mut world, "First Target");
        let second_target = add_system(&mut world, "Second Target");
        // A fleet needs a capital ship to enter hyperspace (FUN_004fda10).
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            hyperdrive: 100,
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
        let mut movement = MovementState::new();
        let mut troop_transport = TroopTransportState::default();
        let mut integrator = PerceptionIntegrator::new(5, 0);

        let outcomes = apply_empire_orders(
            &mut integrator,
            &mut world,
            &mut movement,
            &mut troop_transport,
            &actions,
        );

        assert_eq!(movement.len(), 1);
        assert_eq!(movement.get(fleet).unwrap().destination, first_target);
        assert_eq!(
            outcomes[1],
            AiOutcome::Refused(AiRefusal::Fleet(
                rebellion_core::fleet_join::FleetOrderError::Dispatch(
                    rebellion_core::movement::FleetDispatchError::AlreadyInTransit
                )
            ))
        );
        let events = integrator.finish();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type, EVT_AI_ACTION);
        assert_eq!(events[0].details["to"], "First Target");
        assert_eq!(events[0].details["outcome"], "applied");
        assert_eq!(events[1].event_type, EVT_AI_ACTION);
        assert_eq!(events[1].details["outcome"], "refused");
        assert_eq!(
            events[1].details["refusal"],
            "fleet/Dispatch/AlreadyInTransit"
        );
    }

    /// Apply `actions` for the Empire AI and return the telemetry.
    fn empire_ai_events(world: &mut GameWorld, actions: &[AIAction]) -> Vec<GameEventRecord> {
        let mut integrator = PerceptionIntegrator::new(5, 0);
        apply_empire_orders(
            &mut integrator,
            world,
            &mut MovementState::new(),
            &mut TroopTransportState::default(),
            actions,
        );
        integrator.finish()
    }

    fn imperial_at(world: &mut GameWorld, system: SystemKey, name: &str) -> MissionMember {
        MissionMember::Character(world.characters.insert(rebellion_core::world::Character {
            name: name.into(),
            is_empire: true,
            current_system: Some(system),
            ..Default::default()
        }))
    }

    #[test]
    fn an_applied_ai_mission_reports_its_team_and_decoys_as_dispatched() {
        let mut world = GameWorld::default();
        let target = add_system(&mut world, "Target");
        let envoy = imperial_at(&mut world, target, "Envoy");
        let double = imperial_at(&mut world, target, "Double");

        let events = empire_ai_events(
            &mut world,
            &[AIAction::DispatchMission {
                kind: MissionKind::Diplomacy,
                team: vec![envoy],
                decoys: vec![double],
                target_system: target,
                target_character: None,
                target_object: None,
            }],
        );

        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type, EVT_AI_ACTION);
        assert_eq!(events[0].details["team"], serde_json::json!(["Envoy"]));
        assert_eq!(events[0].details["decoys"], serde_json::json!(["Double"]));
        assert_eq!(events[1].system, SYS_MISSIONS);
        assert_eq!(events[1].event_type, EVT_MISSION_DISPATCHED);
        assert_eq!(
            events[1].details,
            serde_json::json!({
                "mission_id": 0,
                "kind": "Diplomacy",
                "faction": "Empire",
                "team": ["Envoy"],
                "decoys": ["Double"],
                "target_system": "Target",
            })
        );
    }

    #[test]
    fn a_refused_ai_mission_is_reported_refused_and_not_dispatched() {
        let mut world = GameWorld::default();
        let target = add_system(&mut world, "Target");
        let rebel =
            MissionMember::Character(world.characters.insert(rebellion_core::world::Character {
                name: "Rebel".into(),
                is_alliance: true,
                current_system: Some(target),
                ..Default::default()
            }));

        let events = empire_ai_events(
            &mut world,
            &[AIAction::DispatchMission {
                kind: MissionKind::Diplomacy,
                team: vec![rebel],
                decoys: vec![],
                target_system: target,
                target_character: None,
                target_object: None,
            }],
        );

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EVT_AI_ACTION);
        assert_eq!(events[0].details["outcome"], "refused");
        assert_eq!(events[0].details["refusal"], "mission/MemberUnavailable");
        assert_eq!(events[0].details["status"], serde_json::json!([0x40, 0x91]));
    }

    #[test]
    fn a_special_force_is_named_by_its_class() {
        let mut world = GameWorld::default();
        let unit = world.special_forces.insert(SpecialForceUnit {
            class_dat_id: DatId::new(0x3c00_0007),
            is_alliance: false,
            skills: [0; 8],
            on_mission: false,
            en_route: false,
            on_hidden_mission: false,
        });

        assert_eq!(
            member_name(&world, MissionMember::SpecialForce(unit)),
            "special force 0x3c000007"
        );
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
        let mut movement = MovementState::new();
        let mut troop_transport = TroopTransportState::default();
        let mut integrator = PerceptionIntegrator::new(5, 0);

        apply_empire_orders(
            &mut integrator,
            &mut world,
            &mut movement,
            &mut troop_transport,
            &actions,
        );

        assert!(movement.is_in_transit(fleet));
        assert_eq!(troop_transport.cargo(fleet), &[troop]);
        assert!(!world.systems[origin].ground_units.contains(&troop));
        assert_eq!(integrator.finish()[0].details["troops"], 1);
    }

    #[test]
    fn troops_embarked_for_a_move_that_cannot_depart_land_again() {
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
        let mut movement = MovementState::new();
        // An order already held refuses the new one (MovementState::order).
        assert!(movement.order(fleet, origin, destination, 3));
        let mut troop_transport = TroopTransportState::default();

        let outcome = apply_ai_order(
            &AIAction::MoveFleet {
                fleet,
                to_system: destination,
                reason: FleetMoveReason::Attack,
                troops: vec![troop],
            },
            &mut AIState::new(AiFaction::Empire),
            &mut MissionState::new(),
            &mut ManufacturingState::new(),
            &mut movement,
            &mut troop_transport,
            &mut StockpileState::new(),
            &mut world,
            &UprisingState::default(),
            5,
        );

        assert_eq!(
            outcome,
            AiOutcome::Refused(AiRefusal::Fleet(
                rebellion_core::fleet_join::FleetOrderError::Dispatch(
                    rebellion_core::movement::FleetDispatchError::AlreadyInTransit
                )
            ))
        );
        assert!(troop_transport.cargo(fleet).is_empty());
        assert!(world.systems[origin].ground_units.contains(&troop));
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
    fn a_ship_completed_where_its_side_has_no_fleet_starts_a_numbered_fleet() {
        // FUN_00517760: each new fleet takes its side's next "Fleet N"
        // (ghidra/notes/fleet-names.md).
        let mut world = GameWorld::default();
        let first = add_system(&mut world, "First");
        let second = add_system(&mut world, "Second");
        let ship = world.capital_ship_classes.insert(CapitalShipClass {
            is_alliance: true,
            hull: 100,
            ..Default::default()
        });
        for system in [first, second] {
            apply_build_completion_inner(
                &CompletionEvent {
                    system,
                    tick: 1,
                    kind: BuildableKind::CapitalShip(ship),
                },
                &mut world,
            );
        }
        let name = |system: SystemKey| world.fleet_name(world.systems[system].fleets[0]);
        assert_eq!(name(first), Some("Fleet 1"));
        assert_eq!(name(second), Some("Fleet 2"));
    }

    #[test]
    fn a_completed_death_star_can_fire_from_its_new_fleet() {
        // CAPSHPSD.DAT record 136 is the Death Star (TEXTSTRA 10120).
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Yard");
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(rebellion_core::world::DEATH_STAR_CLASS_ID),
            is_empire: true,
            hull: 25_000,
            ..Default::default()
        });
        let target = add_system(&mut world, "Target");
        world.systems[target].control = ControlKind::Controlled(Faction::Alliance);

        apply_build_completion_inner(
            &CompletionEvent {
                system,
                tick: 1,
                kind: BuildableKind::CapitalShip(class),
            },
            &mut world,
        );

        let fleet = world.systems[system].fleets[0];
        assert!(world.fleets[fleet].has_death_star);
        world.systems[system].fleets.clear();
        world.systems[target].fleets.push(fleet);
        world.fleets[fleet].location = target;
        assert!(rebellion_core::death_star::DeathStarSystem::fire(
            &rebellion_core::death_star::DeathStarState::default(),
            &world,
            target,
            2
        )
        .is_some());
    }

    #[test]
    fn a_regiment_lost_running_a_blockade_leaves_its_fleet_and_is_reported() {
        // FUN_00504990 destroys the regiment; FUN_00504a00 raises event 0x340.
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Bespin");
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            troop_capacity: 1,
            ..CapitalShipClass::default()
        });
        let fleet = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ShipInstance::new(class, 100, false)],
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            has_death_star: false,
        });
        world.systems[system].fleets.push(fleet);
        let troop = world.troops.insert(rebellion_core::world::TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: false,
            regiment_strength: 100,
        });
        world.systems[system].ground_units.push(troop);
        let mut transport = TroopTransportState::default();
        transport.embark(&mut world, fleet, &[troop]).unwrap();
        let mut integrator = PerceptionIntegrator::new(9, 0);

        integrator.apply_blockade_events(
            &mut world,
            &mut transport,
            &[BlockadeEvent::TroopDestroyed {
                system,
                troop,
                tick: 9,
            }],
        );

        assert!(transport.cargo(fleet).is_empty());
        assert!(!world.troops.contains_key(troop));
        let events = integrator.finish();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EVT_BLOCKADE_TROOP_DESTROYED);
        assert_eq!(events[0].details["system"], "Bespin");
    }

    #[test]
    fn a_death_star_joining_a_fleet_marks_it_and_other_ships_do_not() {
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Yard");
        let mut add_class = |id| {
            world.capital_ship_classes.insert(CapitalShipClass {
                dat_id: DatId::new(id),
                is_empire: true,
                hull: 100,
                ..Default::default()
            })
        };
        let star_destroyer = add_class(131);
        let death_star = add_class(rebellion_core::world::DEATH_STAR_CLASS_ID);
        let build = |world: &mut GameWorld, class| {
            apply_build_completion_inner(
                &CompletionEvent {
                    system,
                    tick: 1,
                    kind: BuildableKind::CapitalShip(class),
                },
                world,
            );
        };

        build(&mut world, star_destroyer);
        let fleet = world.systems[system].fleets[0];
        assert!(!world.fleets[fleet].has_death_star);

        build(&mut world, death_star);
        assert_eq!(world.systems[system].fleets, [fleet]);
        assert!(world.fleets[fleet].has_death_star);
    }

    #[test]
    fn completed_squadrons_stack_on_their_system_for_their_side() {
        // FUN_0052bee0: a finished squadron becomes a child of the system it
        // lands at, which has room for any number (FUN_00507750); no fleet
        // takes it and none is made for it.
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Yard");
        let enemy = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        world.systems[system].fleets.push(enemy);
        let tie = world
            .fighter_classes
            .insert(rebellion_core::world::FighterClass {
                is_empire: true,
                ..Default::default()
            });
        for _ in 0..2 {
            apply_build_completion_inner(
                &CompletionEvent {
                    system,
                    tick: 1,
                    kind: BuildableKind::Fighter(tie),
                },
                &mut world,
            );
        }

        assert!(world.fleets[enemy].fighters.is_empty());
        assert_eq!(world.systems[system].fleets, [enemy]);
        assert_eq!(world.systems[system].squadron_count(false), 2);
        assert_eq!(world.systems[system].squadron_count(true), 0);
    }

    #[test]
    fn troop_production_builds_its_class_for_the_classes_side() {
        // TROOPSD record 3 (Sullustan) serves the Alliance only (+0x18).
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Training World");
        world.buildable_classes.insert(
            DatId::new(0x1000_0003),
            rebellion_core::world::BuildableClass {
                is_alliance: true,
                ..Default::default()
            },
        );

        apply_build_completion_inner(
            &CompletionEvent {
                system,
                tick: 1,
                kind: BuildableKind::Troop(DatId::new(0x1000_0003)),
            },
            &mut world,
        );

        let built = *world.systems[system].ground_units.last().unwrap();
        assert_eq!(world.troops[built].class_dat_id, DatId::new(0x1000_0003));
        assert!(world.troops[built].is_alliance);
        assert_eq!(world.troops[built].regiment_strength, 100);
    }

    #[test]
    fn facility_and_special_force_production_build_their_class_for_the_builder() {
        // MANFACSD and PROFACSD serve both sides, so the order carries its
        // builder's; family 0x28 is a shipyard and 0x2c a mine
        // (FUN_00537ff0, FUN_00458fe0).
        use rebellion_core::manufacturing::FacilityBuild;
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Kuat");
        let empire = |class| FacilityBuild {
            class: DatId::new(class),
            is_alliance: false,
        };
        world.buildable_classes.insert(
            DatId::new(0x3c00_0007),
            rebellion_core::world::BuildableClass {
                is_empire: true,
                ..Default::default()
            },
        );
        world.special_force_classes.insert(
            DatId::new(0x3c00_0007),
            rebellion_core::world::SpecialForceClassDef {
                skills: [rebellion_core::world::SkillPair {
                    base: 30,
                    variance: 10,
                }; 8],
                mission_mask: 0,
            },
        );
        for kind in [
            BuildableKind::ManufacturingFacility(empire(0x2800_0004)),
            BuildableKind::ProductionFacility(empire(0x2c00_0001)),
            BuildableKind::DefenseFacility(empire(0x2200_0001)),
            BuildableKind::SpecialForce(DatId::new(0x3c00_0007)),
        ] {
            apply_build_completion_inner(
                &CompletionEvent {
                    system,
                    tick: 1,
                    kind,
                },
                &mut world,
            );
        }

        let at = &world.systems[system];
        let yard = &world.manufacturing_facilities[at.manufacturing_facilities[0]];
        assert_eq!(yard.class_dat_id, DatId::new(0x2800_0004));
        assert!(yard.side == rebellion_core::dat::Faction::Empire && yard.is_shipyard);
        let mine = &world.production_facilities[at.production_facilities[0]];
        assert!(mine.side == rebellion_core::dat::Faction::Empire && mine.is_mine);
        let defense = &world.defense_facilities[at.defense_facilities[0]];
        assert_eq!(defense.class_dat_id, DatId::new(0x2200_0001));
        assert_eq!(defense.side, rebellion_core::dat::Faction::Empire);
        let unit = &world.special_forces[at.special_forces[0]];
        assert!(!unit.is_alliance);
        assert_eq!(unit.skills, [30; 8]);
    }
}

// ---------------------------------------------------------------------------
// Combat helpers (moved from simulation.rs)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SpaceForceSummary {
    pub capital_ships: usize,
    pub fighter_squadrons: u32,
    pub hull: i64,
}

/// Aggregate result for every hostile fleet orbiting one system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemSpaceCombatResult {
    pub system: SystemKey,
    /// Alliance is the attacker and Empire is the defender in auto-resolve.
    pub winner: CombatSide,
    /// A surviving fleet belonging to the winning faction, when decisive.
    pub winner_fleet: Option<FleetKey>,
    /// Simulated seconds the headless battle ran.
    pub seconds: u32,
    pub alliance_fleets: usize,
    pub empire_fleets: usize,
    pub alliance_before: SpaceForceSummary,
    pub alliance_after: SpaceForceSummary,
    pub empire_before: SpaceForceSummary,
    pub empire_after: SpaceForceSummary,
    /// True when both sides remain, so the system fights again next pass.
    pub stalemate: bool,
    /// The write-back's log lines: withdrawals and the post-battle pass.
    pub log: Vec<String>,
}

pub(crate) fn orbiting_fleets_by_faction(
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

/// A side's fleets at `system` and the squadrons the system holds for it.
pub(crate) fn summarize_space_force(
    world: &GameWorld,
    system: SystemKey,
    is_alliance: bool,
    fleets: &[FleetKey],
) -> SpaceForceSummary {
    let held = SpaceForceSummary {
        fighter_squadrons: world
            .systems
            .get(system)
            .map_or(0, |value| value.squadron_count(is_alliance)),
        ..SpaceForceSummary::default()
    };
    fleets.iter().fold(held, |mut summary, fleet_key| {
        if let Some(fleet) = world.fleets.get(*fleet_key) {
            summary.capital_ships += fleet.capital_ships.iter().filter(|ship| ship.alive).count();
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

/// Fight one system's battle without its window, as the original does for a
/// battle nobody watches (`FUN_0040a050` state 8,
/// `ghidra/notes/simulate-runs-tactical-engine.md`): every fleet of each side
/// and the squadrons the system holds for it enter one tactical session, which
/// runs one-second headless steps to its end (`BattleSession::simulate_to_end`)
/// and is written back as a played battle is (`tactical::apply::apply_results`).
/// port: the Alliance is the session's attacker, as in the player's battles.
pub fn resolve_system_space_combat(
    world: &mut GameWorld,
    troop_transport: &mut TroopTransportState,
    movement: &mut MovementState,
    system: SystemKey,
    rng_seed: u32,
    tick: u64,
) -> SystemSpaceCombatResult {
    resolve_system_space_combat_for(
        world,
        troop_transport,
        movement,
        system,
        rng_seed,
        tick,
        None,
    )
}

/// [`resolve_system_space_combat`] with a person commanding a side
/// (`human_is_alliance`): that side's withdrawal threshold is 0.0
/// (`FUN_004865b0`), a computer side's `FUN_004e6110`'s.
pub fn resolve_system_space_combat_for(
    world: &mut GameWorld,
    troop_transport: &mut TroopTransportState,
    movement: &mut MovementState,
    system: SystemKey,
    rng_seed: u32,
    tick: u64,
    human_is_alliance: Option<bool>,
) -> SystemSpaceCombatResult {
    let (initial_alliance, initial_empire) = orbiting_fleets_by_faction(world, system);
    let alliance_before = summarize_space_force(world, system, true, &initial_alliance);
    let empire_before = summarize_space_force(world, system, false, &initial_empire);
    rebellion_core::trace::record(
        rebellion_core::trace::TraceMask::COMBAT,
        "pre_battle",
        || {
            serde_json::json!({
                "system": sys_name(world, system),
                "tick": tick,
                "rng_seed": rng_seed,
                "alliance_fleets": initial_alliance.len(),
                "empire_fleets": initial_empire.len(),
                "alliance": crate::telemetry::force_payload(&alliance_before),
                "empire": crate::telemetry::force_payload(&empire_before),
            })
        },
    );

    // FUN_00433620 reads the fleets' ship tags; the AI tags them each pass.
    rebellion_core::carriage::tag_all(world);
    let mut session = BattleSession::new_with_forces(
        world,
        system,
        CombatForce {
            fleet: initial_alliance.first().copied(),
            is_alliance: true,
        },
        CombatForce {
            fleet: initial_empire.first().copied(),
            is_alliance: false,
        },
        false,
        tick,
        rng_seed,
    );
    // The Alliance is the session's attacker (see above).
    rebellion_core::tactical::apply::seat_morale(
        &mut session,
        world,
        troop_transport,
        movement,
        human_is_alliance,
    );
    session.simulate_to_end();
    let mut withdrawal_log = Vec::new();
    apply_results(
        &session,
        world,
        troop_transport,
        movement,
        &mut withdrawal_log,
    );

    let (alliance, empire) = orbiting_fleets_by_faction(world, system);
    let alliance_after = summarize_space_force(world, system, true, &alliance);
    let empire_after = summarize_space_force(world, system, false, &empire);
    // FUN_00517270 re-checks presence: a side left with only the squadrons
    // the system holds is still present, so the system stays contested.
    let (winner, winner_fleet) = match (
        side_present(world, system, true),
        side_present(world, system, false),
    ) {
        (true, false) => (CombatSide::Attacker, alliance.first().copied()),
        (false, true) => (CombatSide::Defender, empire.first().copied()),
        _ => (CombatSide::Draw, None),
    };

    SystemSpaceCombatResult {
        system,
        winner,
        winner_fleet,
        seconds: session.combat_tick / 4,
        alliance_fleets: initial_alliance.len(),
        empire_fleets: initial_empire.len(),
        alliance_before,
        alliance_after,
        empire_before,
        empire_after,
        stalemate: winner == CombatSide::Draw
            && side_present(world, system, true)
            && side_present(world, system, false),
        log: withdrawal_log,
    }
}

#[cfg(test)]
mod combat_application_tests {
    use super::*;
    use rebellion_core::dat::ExplorationStatus;
    use rebellion_core::ids::SectorKey;
    use rebellion_core::world::{CapitalShipClass, Fleet, System};

    fn add_combat_system(world: &mut GameWorld) -> SystemKey {
        world.systems.insert(System {
            dat_id: DatId::new(0x9000_0001),
            name: "Test System".into(),
            sector: SectorKey::default(),
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
            fighters: Vec::new(),
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
            dat_id: DatId::new(if is_alliance {
                0x3000_0001
            } else {
                0x3000_0002
            }),
            is_alliance,
            is_empire: !is_alliance,
            hull,
            turbolaser_fore: attack,
            // The tactical engine fires only with a range and a recharge, as
            // every armed shipped class has (CAPSHPSD: recharge 6-18, range 50-60).
            turbolaser_range: 60,
            weapon_recharge_rate: 8,
            ..CapitalShipClass::default()
        });
        let fleet = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ShipInstance::new(class, hull.cast_signed(), is_alliance)],
            fighters: vec![],
            characters: vec![],
            is_alliance,
            has_death_star: false,
        });
        world.systems[system].fleets.push(fleet);
        fleet
    }

    #[test]
    fn system_engagement_includes_every_orbiting_task_force() {
        let mut world = GameWorld::default();
        let system = add_combat_system(&mut world);
        let first_alliance = add_ship_fleet(&mut world, system, true, 200, 100);
        let second_alliance = add_ship_fleet(&mut world, system, true, 200, 100);
        let first_empire = add_ship_fleet(&mut world, system, false, 30, 1);
        let second_empire = add_ship_fleet(&mut world, system, false, 30, 1);

        let result = resolve_system_space_combat(
            &mut world,
            &mut TroopTransportState::default(),
            &mut MovementState::default(),
            system,
            0x5eed,
            11,
        );

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
        sys.ground_units
            .retain(|&k| world.troops.get(k).is_some_and(|t| t.regiment_strength > 0));
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

/// A product delivered to `fleet`, whose container held it en route
/// (`crate::delivery`): a capital ship joins the fleet and a squadron goes
/// aboard (`board_squadrons`); any other product lands at the fleet's
/// system. hyp: the fleet container's acceptance test is untraced, so the
/// fleet takes every ship and squadron.
pub fn apply_boarding_inner(fleet: FleetKey, completion: &CompletionEvent, world: &mut GameWorld) {
    match completion.kind {
        BuildableKind::CapitalShip(class_key) => {
            let Some(class) = world.capital_ship_classes.get(class_key) else {
                return;
            };
            let (hull, is_alliance, is_death_star) = (
                class.hull.cast_signed(),
                class.is_alliance,
                class.is_death_star(),
            );
            let Some(value) = world.fleets.get_mut(fleet) else {
                return;
            };
            value
                .capital_ships
                .push(ShipInstance::new(class_key, hull, is_alliance));
            value.has_death_star |= is_death_star;
            world.number_capital_ships();
        }
        BuildableKind::Fighter(class_key) => {
            rebellion_core::carriage::board_squadrons(world, fleet, class_key, 1);
        }
        _ => apply_build_completion_inner(completion, world),
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
pub fn apply_build_completion_inner(completion: &CompletionEvent, world: &mut GameWorld) {
    let sys_key = completion.system;

    match &completion.kind {
        BuildableKind::CapitalShip(class_key) => {
            let is_alliance = world
                .capital_ship_classes
                .get(*class_key)
                .is_some_and(|c| c.is_alliance);
            let is_death_star = world
                .capital_ship_classes
                .get(*class_key)
                .is_some_and(CapitalShipClass::is_death_star);

            let fleet_key = {
                let Some(sys) = world.systems.get(sys_key) else {
                    return;
                };
                sys.fleets.iter().copied().find(|&fk| {
                    world
                        .fleets
                        .get(fk)
                        .is_some_and(|f| f.is_alliance == is_alliance)
                })
            };

            if let Some(fk) = fleet_key {
                if let Some(fleet) = world.fleets.get_mut(fk) {
                    let hull = world
                        .capital_ship_classes
                        .get(*class_key)
                        .map_or(100, |c| c.hull.cast_signed());
                    fleet
                        .capital_ships
                        .push(ShipInstance::new(*class_key, hull, is_alliance));
                    fleet.has_death_star |= is_death_star;
                }
            } else {
                let hull = world
                    .capital_ship_classes
                    .get(*class_key)
                    .map_or(100, |c| c.hull.cast_signed());
                let fleet = Fleet {
                    location: sys_key,
                    capital_ships: vec![ShipInstance::new(*class_key, hull, is_alliance)],
                    fighters: vec![],
                    characters: vec![],
                    is_alliance,
                    has_death_star: is_death_star,
                };
                let fk = world.insert_fleet(fleet);
                if let Some(sys) = world.systems.get_mut(sys_key) {
                    sys.fleets.push(fk);
                }
            }
            // The new ship takes its class's next number (FUN_00517760).
            world.number_capital_ships();
        }
        BuildableKind::Fighter(class_key) => {
            let is_alliance = world
                .fighter_classes
                .get(*class_key)
                .is_some_and(|c| c.is_alliance);
            // A finished squadron becomes a child of the system it lands at,
            // which has room for any number (`FUN_0052bee0`, `FUN_00507750`).
            if let Some(sys) = world.systems.get_mut(sys_key) {
                sys.add_squadrons(*class_key, 1, is_alliance);
            }
        }
        BuildableKind::ManufacturingFacility(build) => {
            let fac_key = world
                .manufacturing_facilities
                .insert(ManufacturingFacilityInstance {
                    class_dat_id: build.class,
                    side: rebellion_core::dat::Faction::of_alliance(build.is_alliance),
                    is_shipyard: build.class.family() == 0x28,
                });
            if let Some(sys) = world.systems.get_mut(sys_key) {
                sys.manufacturing_facilities.push(fac_key);
            }
        }
        BuildableKind::DefenseFacility(build) => {
            let fac_key = world.defense_facilities.insert(DefenseFacilityInstance {
                class_dat_id: build.class,
                side: rebellion_core::dat::Faction::of_alliance(build.is_alliance),
            });
            if let Some(sys) = world.systems.get_mut(sys_key) {
                sys.defense_facilities.push(fac_key);
            }
        }
        BuildableKind::ProductionFacility(build) => {
            let fac_key = world
                .production_facilities
                .insert(ProductionFacilityInstance {
                    class_dat_id: build.class,
                    side: rebellion_core::dat::Faction::of_alliance(build.is_alliance),
                    is_mine: build.class.family() == 0x2c,
                });
            if let Some(sys) = world.systems.get_mut(sys_key) {
                sys.production_facilities.push(fac_key);
            }
        }
        // A regiment or special-force class serves one side.
        BuildableKind::Troop(class) => {
            let Some(is_alliance) = world.buildable_classes.get(class).map(|c| c.is_alliance)
            else {
                return;
            };
            let tk = world.troops.insert(TroopUnit {
                class_dat_id: *class,
                is_alliance,
                regiment_strength: 100,
            });
            if let Some(sys) = world.systems.get_mut(sys_key) {
                sys.ground_units.push(tk);
            }
        }
        // port: a built unit takes its class's base skills; the creation
        // roll (`FUN_00535e40`, base + rand(0..=variance)) needs the
        // simulation's random stream, which completion does not carry.
        BuildableKind::SpecialForce(class) => {
            let Some(is_alliance) = world.buildable_classes.get(class).map(|c| c.is_alliance)
            else {
                return;
            };
            let skills = world
                .special_force_classes
                .get(class)
                .map_or([0; 8], |def| def.skills.map(|pair| pair.base));
            let key = world.special_forces.insert(SpecialForceUnit {
                class_dat_id: *class,
                is_alliance,
                skills,
                on_mission: false,
                en_route: false,
                on_hidden_mission: false,
            });
            if let Some(sys) = world.systems.get_mut(sys_key) {
                sys.special_forces.push(key);
            }
        }
    }
}
