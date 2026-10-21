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

use rebellion_core::ai::{AIAction, AIState};
use rebellion_core::betrayal::BetrayalEvent;
use rebellion_core::blockade::{BlockadeEvent, BlockadeSystem};
use rebellion_core::combat::{side_present, CombatForce, CombatSide};
use rebellion_core::economy::{EconomyEvent, EconomyState};
use rebellion_core::events::{EventAction, FiredEvent, SkillField, SystemTag};
use rebellion_core::fog::RevealEvent;
use rebellion_core::game_events::{
    GameEventRecord, EVT_AI_ACTION, EVT_ASSAULT, EVT_BETRAYAL_CHECK, EVT_BLOCKADE_ENDED,
    EVT_BLOCKADE_STARTED, EVT_BLOCKADE_SUPPORT_SHIFT, EVT_BLOCKADE_TROOP_DESTROYED,
    EVT_BOMBARDMENT, EVT_BUILD_COMPLETE, EVT_CAMPAIGN_SNAPSHOT, EVT_CHARACTER_HEALTH,
    EVT_CHARACTER_KILLED, EVT_COMBAT_SPACE, EVT_CONTROL_CHANGED, EVT_DESTROYED_ON_ARRIVAL,
    EVT_DS_FIRED, EVT_ESCAPE, EVT_EVENT_FIRED, EVT_FLEET_ARRIVED, EVT_FOG_REVEALED,
    EVT_HQ_CAPTURED, EVT_INFORMANT_INTEL, EVT_JEDI_CHECK, EVT_JEDI_DISCOVERED, EVT_JEDI_TIER,
    EVT_MAINTENANCE_SHORTFALL, EVT_MANUFACTURING_IDLE, EVT_SCRAPPED_ABOARD, EVT_MISSION_DISPATCHED,
    EVT_MISSION_RESOLVED, EVT_NATURAL_DISASTER, EVT_RESEARCH_UNLOCKED, EVT_REVOLT_SUPPORT_LOSS,
    EVT_SABOTEUR_DETECTED, EVT_SIDE_CHANGE, EVT_SUPPORT_SHIFT, EVT_TRAITOR_REVEALED,
    EVT_TROOP_MOVED, EVT_UNITS_DEPLOYED, EVT_UPRISING_BEGAN, EVT_UPRISING_CHECK,
    EVT_UPRISING_ENDED, EVT_UPRISING_INCIDENT, EVT_VICTORY, EVT_VICTORY_CHECK, SYS_AI,
    SYS_BETRAYAL, SYS_BLOCKADE, SYS_COMBAT, SYS_DEATH_STAR, SYS_ECONOMY, SYS_EVENTS, SYS_FOG,
    SYS_JEDI, SYS_MANUFACTURING, SYS_MISSIONS, SYS_MOVEMENT, SYS_REPAIR, SYS_RESEARCH, SYS_STORY,
    SYS_UPRISING, SYS_VICTORY,
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
    ControlKind, DefenseFacilityInstance, GameWorld, ManufacturingFacilityInstance,
    ProductionFacilityInstance, SpecialForceUnit, TroopUnit,
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
        AIAction::Assault { system, .. } => {
            serde_json::json!({ "type": "Assault", "at": sys_name(world, *system) })
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

    /// Serve the support shifts (event `0x373`), regiment reruns and holder
    /// changes (event `0x31b`) the day's operation raised, in the day's
    /// first step (`FUN_0051df30`), and record each one; a holder change's
    /// managers lose their products (`manufacturing::serve_manager_sides`).
    pub fn fire_support_shifts(
        &mut self,
        world: &mut GameWorld,
        manufacturing: &mut ManufacturingState,
    ) {
        let fired = fire_holder_changes(
            world,
            manufacturing,
            rebellion_core::support_shift::step(self.tick, 0),
        );
        self.emit_fired(world, &fired.shifts, &fired.controls);
    }

    /// One `control_changed` record per holder change, then one
    /// `support_shift` record per fired shift.
    pub fn emit_fired(
        &mut self,
        world: &GameWorld,
        shifts: &[rebellion_core::support_shift::AppliedShift],
        controls: &[rebellion_core::support_shift::ControlRerun],
    ) {
        for change in controls {
            self.emit(
                SYS_ECONOMY,
                EVT_CONTROL_CHANGED,
                crate::telemetry::control_resolved_payload(world, change.system, change.control),
            );
        }
        for shift in shifts {
            self.emit(
                SYS_ECONOMY,
                EVT_SUPPORT_SHIFT,
                crate::telemetry::applied_shift_payload(world, shift),
            );
        }
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
                        "collection_rate": rebellion_core::economy::stored_support_values(world, key)
                            .collection_rate,
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

    /// Record the economy's events. They change nothing in the world: the
    /// holder is rebuilt by its hooks (`support_shift::rebuild_control`).
    pub fn apply_economy_events(&mut self, world: &GameWorld, economy_events: &[EconomyEvent]) {
        for ev in economy_events {
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

    /// One `scrapped_aboard` record per ship an overdraft scrap took
    /// squadrons or regiments down with.
    pub fn emit_scrapped_aboard(
        &mut self,
        world: &GameWorld,
        losses: &[rebellion_core::scrap::AboardLoss],
    ) {
        for loss in losses {
            self.emit(
                SYS_ECONOMY,
                EVT_SCRAPPED_ABOARD,
                serde_json::json!({
                    "faction": format!("{:?}", loss.side),
                    "system": sys_name(world, loss.system),
                    "ship": loss.ship,
                    "squadrons": loss.aboard.squadrons,
                    "regiments": loss.aboard.regiments,
                }),
            );
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
        manufacturing: &mut ManufacturingState,
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
        // A finished regiment's holder rerun (FUN_00508660).
        self.fire_support_shifts(world, manufacturing);
    }

    /// Place products delivered to a fleet (`crate::delivery`), with the
    /// same telemetry as [`Self::apply_build_completions`].
    pub fn apply_boarded(
        &mut self,
        world: &mut GameWorld,
        manufacturing: &mut ManufacturingState,
        transport: &mut TroopTransportState,
        boarded: &[rebellion_core::delivery::Boarded],
    ) {
        for arrived in boarded {
            apply_boarding_inner(arrived, world, transport);
            let c = &arrived.completion;
            let payload = serde_json::json!({
                "system": sys_name(world, c.system),
                "fleet": world.fleet_name(arrived.fleet),
                "kind": format!("{:?}", c.kind),
            });
            self.emit(SYS_MANUFACTURING, EVT_BUILD_COMPLETE, payload.clone());
            self.emit(SYS_MANUFACTURING, EVT_UNITS_DEPLOYED, payload);
        }
        self.fire_support_shifts(world, manufacturing);
    }

    /// Capital ships that ended their own legs in their fleets
    /// (`delivery::advance_ship_legs`), with the same telemetry as
    /// [`Self::apply_boarded`].
    pub fn apply_ship_arrivals(
        &mut self,
        world: &mut GameWorld,
        arrivals: &[rebellion_core::delivery::ShipArrival],
    ) {
        apply_ship_arrivals_inner(world, arrivals);
        for arrival in arrivals {
            let payload = serde_json::json!({
                "system": sys_name(world, arrival.system),
                "fleet": world.fleet_name(arrival.fleet),
                "kind": format!("{:?}", BuildableKind::CapitalShip(arrival.class)),
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

    /// One `EVT_BOMBARDMENT` record per bombardment run, with or without
    /// kills.
    pub fn emit_bombardment(
        &mut self,
        world: &GameWorld,
        report: &rebellion_core::bombardment::StrikeReport,
        support_change: i32,
    ) {
        let Some(system) = report.system else {
            return;
        };
        self.emit(
            SYS_COMBAT,
            EVT_BOMBARDMENT,
            serde_json::json!({
                "system": sys_name(world, system),
                "order": report.kind.map(|kind| format!("{:#x}", kind.order())),
                "bombarder": report.bombarder.map(|side| format!("{side:?}")),
                "hits": report.hits,
                "regiments": report.regiments.len(),
                "facilities": report.facilities.len(),
                "headquarters_destroyed": report.headquarters,
                "energy": report.energy,
                "raw_material": report.raw_material,
                "defence_damage": report.defence_damage,
                "ships_destroyed": report.ships_destroyed,
                "support_loss": report.run.support_loss,
                "support_change": support_change,
            }),
        );
    }
    // ── Step 5: Missions + Escapes ──────────────────────────────────────

    /// Apply mission result: world mutations + telemetry.
    pub fn apply_mission_result(
        &mut self,
        world: &mut GameWorld,
        manufacturing: &mut ManufacturingState,
        result: &MissionResult,
        uprising_state: &mut UprisingState,
        research_state: &mut ResearchState,
    ) {
        let ended =
            apply_mission_effects_inner(&result.effects, world, result.tick, uprising_state);
        // A support gain's control change (FUN_005109f0, event 0x31b).
        self.fire_support_shifts(world, manufacturing);
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

    /// Record each escape; the release itself is
    /// `rebellion_core::death_star::free_escapee`.
    pub fn apply_escape_effects(&mut self, world: &GameWorld, effects: &[MissionEffect]) {
        for effect in effects {
            if let MissionEffect::CharacterEscaped {
                character,
                escaped_to_alliance,
            } = effect
            {
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
            AIAction::Scrap { targets },
            AiOutcome::Applied { aboard, .. },
        ) = (action, outcome)
        {
            if *aboard != rebellion_core::scrap::Aboard::default() {
                self.emit(
                    SYS_ECONOMY,
                    EVT_SCRAPPED_ABOARD,
                    serde_json::json!({
                        "faction": origin.faction,
                        "targets": targets.iter().map(|target| rebellion_core::scrap::target_name(world, *target)).collect::<Vec<_>>(),
                        "squadrons": aboard.squadrons,
                        "regiments": aboard.regiments,
                    }),
                );
            }
        }
        if let (
            AIAction::DispatchMission {
                kind,
                team,
                decoys,
                target_system,
                ..
            },
            AiOutcome::Applied { mission, .. },
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

    /// The records of what a strike did: the run, the planet destroyed and
    /// the characters killed.
    pub fn emit_strike(&mut self, world: &GameWorld, strike: &StrikeEffects) {
        if let (Some(system), Some(previous)) = (strike.destroyed(), strike.previous_control) {
            self.emit(
                SYS_DEATH_STAR,
                EVT_CONTROL_CHANGED,
                crate::telemetry::control_destroyed_payload(world, system, previous),
            );
        }
        self.emit_strike_report(world, strike);
        // The shifts fire at the end of the run; the destroyed system's own
        // change has its record above.
        let controls: Vec<_> = strike
            .controls
            .iter()
            .filter(|change| Some(change.system) != strike.destroyed())
            .copied()
            .collect();
        self.emit_fired(world, &strike.shifts, &controls);
        self.emit_swept(world, &strike.swept);
    }

    /// The records of what an assault did: the run, the control change its
    /// landing or losses made, and the shifts that change fired.
    pub fn emit_assault(&mut self, world: &GameWorld, assault: &AssaultEffects) {
        let Some(report) = assault.report.as_ref() else {
            return;
        };
        let Some(system) = report.system else {
            return;
        };
        self.emit(
            SYS_COMBAT,
            EVT_ASSAULT,
            crate::telemetry::assault_payload(world, report),
        );
        if let Some(previous) = assault.previous_control {
            self.emit(
                SYS_COMBAT,
                EVT_CONTROL_CHANGED,
                crate::telemetry::control_assault_payload(world, system, previous),
            );
        }
        for (event, payload) in &assault.blockade {
            let event_type = match event {
                BlockadeEvent::BlockadeEnded { .. } => EVT_BLOCKADE_ENDED,
                _ => EVT_BLOCKADE_STARTED,
            };
            self.emit(SYS_BLOCKADE, event_type, payload.clone());
        }
        // The assaulted system's own change has its record above.
        let controls: Vec<_> = assault
            .controls
            .iter()
            .filter(|change| Some(change.system) != report.system)
            .copied()
            .collect();
        self.emit_fired(world, &assault.shifts, &controls);
        self.emit_swept(world, &assault.swept);
    }

    /// The run's own record: `EVT_DS_FIRED` with its kills and trips for a
    /// destroyed system, else `EVT_BOMBARDMENT`.
    fn emit_strike_report(&mut self, world: &GameWorld, strike: &StrikeEffects) {
        let Some(report) = strike.report.as_ref() else {
            return;
        };
        let Some(system) = report.system else {
            return;
        };
        if report.planet_destroyed {
            let cascade = strike.cascade.clone().unwrap_or_default();
            let names = |keys: &[CharacterKey]| -> Vec<String> {
                keys.iter()
                    .filter_map(|&key| world.characters.get(key).map(|c| c.name.clone()))
                    .collect()
            };
            self.emit(
                SYS_DEATH_STAR,
                EVT_DS_FIRED,
                serde_json::json!({
                    "system": sys_name(world, system),
                    "regiments": cascade.regiments.len(),
                    "squadrons": cascade.squadrons.iter().map(|entry| entry.count).sum::<u32>(),
                    "facilities": cascade.facilities.len(),
                    "special_forces": cascade.special_forces.len(),
                    "headquarters": cascade.headquarters,
                    "fleets": world.systems.get(system).map_or_else(Vec::new, |sys| {
                        sys.fleets
                            .iter()
                            .filter_map(|&fleet| {
                                world.fleets.get(fleet).map(|value| {
                                    format!(
                                        "{} {}",
                                        world.fleet_name(fleet).unwrap_or_default(),
                                        if value.is_alliance { "Alliance" } else { "Empire" }
                                    )
                                })
                            })
                            .collect::<Vec<_>>()
                    }),
                    "support_change": strike.support_change,
                    "adrift": names(&cascade.adrift),
                    "autoscrapped": cascade
                        .autoscrapped
                        .iter()
                        .map(|(object, outcome)| format!("{object:?} {outcome:?}"))
                        .collect::<Vec<_>>(),
                    "products_destroyed": cascade
                        .products
                        .iter()
                        .map(|kind| format!("{kind:?}"))
                        .collect::<Vec<_>>(),
                }),
            );
            self.emit_killed(world, system, &report.characters);
            for trip in &strike.adrift_trips {
                self.emit_adrift_trip(world, system, trip);
            }
        } else {
            self.emit_bombardment(world, report, strike.support_change);
        }
    }

    /// [`Self::emit_adrift_trips`] for each system [`sweep_requests`]
    /// swept.
    pub fn emit_swept(&mut self, world: &GameWorld, swept: &[(SystemKey, Vec<AdriftTrip>)]) {
        for (system, trips) in swept {
            for trip in trips {
                self.emit_adrift_trip(world, *system, trip);
            }
        }
    }

    /// One `EVT_MISSION_DISPATCHED` record per hidden trip the adrift sweep
    /// started from `system` (`death_star::sweep_adrift`). The original
    /// raises no message for it (`FUN_00536ac0` only logs).
    pub fn emit_adrift_trips(
        &mut self,
        world: &GameWorld,
        missions: &MissionState,
        system: SystemKey,
        started: &[u64],
    ) {
        for trip in AdriftTrip::started(missions, started) {
            self.emit_adrift_trip(world, system, &trip);
        }
    }

    fn emit_adrift_trip(&mut self, world: &GameWorld, system: SystemKey, trip: &AdriftTrip) {
        self.emit(
            SYS_MISSIONS,
            EVT_MISSION_DISPATCHED,
            serde_json::json!({
                "mission_id": trip.mission_id,
                "kind": "Adrift",
                "faction": trip.faction,
                "team": member_names(world, &trip.team),
                "from_system": sys_name(world, system),
                "target_system": sys_name(world, trip.target_system),
            }),
        );
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

    /// Apply blockade events: troop destruction + telemetry. A blockade's
    /// end posts its loss shift from `blockade`'s counters
    /// (`support_shift::blockade_ended`), fired after the events.
    pub fn apply_blockade_events(
        &mut self,
        world: &mut GameWorld,
        manufacturing: &mut ManufacturingState,
        transport: &mut TroopTransportState,
        blockade: &rebellion_core::blockade::BlockadeState,
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
                        crate::telemetry::blockade_end_payload(world, blockade, *system),
                    ));
                    rebellion_core::support_shift::blockade_ended(world, blockade, *system);
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
        self.fire_support_shifts(world, manufacturing);
    }

    // ── Step 9: Uprising ──────────────────────────────────────────────────

    /// Apply uprising and disaster events to the world, with telemetry.
    /// Revolts never change who holds a system (`FUN_0050a130`).
    pub fn apply_uprising_events(
        &mut self,
        world: &mut GameWorld,
        manufacturing: &mut ManufacturingState,
        events: &[UprisingEvent],
    ) {
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
                UprisingEvent::RevoltSupportLoss {
                    system,
                    tick,
                    side,
                    delta,
                } => (
                    *tick,
                    EVT_REVOLT_SUPPORT_LOSS,
                    crate::telemetry::support_shift_payload(world, *system, *side, *delta),
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
        self.fire_support_shifts(world, manufacturing);
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
                    c.leave_abode();
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
            // The escape step's own release (death_star::free_escapee); no
            // mission result carries one.
            MissionEffect::CharacterEscaped { .. } => {}
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
    world: &mut GameWorld,
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
// Event action helper
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
                    c.leave_abode();
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
                    c.leave_abode();
                }
                for (_, fleet) in &mut world.fleets {
                    fleet.characters.retain(|&k| k != *character);
                }
            }
            EventAction::SetCarboniteState { character, frozen } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.on_mandatory_mission = *frozen;
                    c.leave_abode();
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
    /// A strike's system is gone, or its fleet is elsewhere.
    StrikeSystemMissing,
    /// The bombardment order's enable test refused (`bombardment::gate`).
    Strike(rebellion_core::bombardment::BombardRefusal),
    /// The assault order's enable test refused (`assault::order_enabled`).
    Assault(rebellion_core::assault::AssaultRefusal),
    /// The build order's validator refused (`FUN_005385f0`).
    Build(rebellion_core::build_selection::OrderRefusal),
    /// A Scrap order named another side's object (the object check
    /// `FUN_004f9860`).
    ScrapOtherSide,
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
                    F::Unfinished => "Unfinished",
                    F::Dispatch(dispatch) => match dispatch {
                        D::MissingFleet => "Dispatch/MissingFleet",
                        D::MissingOrigin => "Dispatch/MissingOrigin",
                        D::MissingDestination => "Dispatch/MissingDestination",
                        D::WrongFaction => "Dispatch/WrongFaction",
                        D::AlreadyInTransit => "Dispatch/AlreadyInTransit",
                        D::AlreadyAtDestination => "Dispatch/AlreadyAtDestination",
                        D::NoHyperdrive => "Dispatch/NoHyperdrive",
                        D::Blockaded => "Dispatch/Blockaded",
                        D::DeathStarRoom => "Dispatch/DeathStarRoom",
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
            Self::StrikeSystemMissing => "strike/MissingSystem".into(),
            Self::Strike(refusal) => format!("strike/{refusal:?}"),
            Self::Assault(refusal) => format!("assault/{refusal:?}"),
            Self::Build(refusal) => format!("build/{refusal:?}"),
            Self::ScrapOtherSide => "scrap/OtherSide".into(),
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
                R::TargetFamily => "TargetFamily",
                R::TargetIsDeathStar => "TargetIsDeathStar",
                R::TargetNotDeathStar => "TargetNotDeathStar",
                R::TargetIncomplete => "TargetIncomplete",
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
        /// What a Scrap of a capital ship took down with it.
        aboard: rebellion_core::scrap::Aboard,
    },
    Refused(AiRefusal),
    /// The order touches a system with an open battle (native only).
    Held {
        system: SystemKey,
    },
}

impl AiOutcome {
    /// An applied order that took nothing else down with it.
    #[must_use]
    pub const fn applied(mission: Option<u64>) -> Self {
        Self::Applied {
            mission,
            aboard: rebellion_core::scrap::Aboard {
                squadrons: 0,
                regiments: 0,
            },
        }
    }

    #[must_use]
    pub const fn is_applied(&self) -> bool {
        matches!(self, Self::Applied { .. })
    }

    /// The outcome as the `ai_action` record reads it.
    #[must_use]
    pub fn view(&self) -> crate::telemetry::AiOutcomeView {
        match self {
            Self::Applied { mission, .. } => crate::telemetry::AiOutcomeView {
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
    deliveries: &rebellion_core::delivery::DeliveryState,
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
    let applied = AiOutcome::applied(None);
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
                    AiOutcome::applied(Some(id))
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
            // The Destination runs first and stays when the Build is then
            // refused: the computer player sends them as two orders in two
            // scheduler rounds (ai-production.md:45; FUN_004d9e70 ->
            // step 0xc, FUN_004dc9f0 and FUN_004e4300 -> step 9,
            // FUN_004ce350 -> FUN_004ce490, FUN_004d5dd0 -> FUN_004d5f10,
            // FUN_004e0a30 -> FUN_004e0b70, FUN_004cbbb0 -> FUN_004cbcf0),
            // so the Build's validator (FUN_005385f0) judges the new
            // destination.
            if let Some(destination) = destination {
                if let Err(refusal) = rebellion_core::build_selection::order_destination(
                    world,
                    mfg_state,
                    troop_transport,
                    deliveries,
                    *system,
                    ProductionArea::of(*kind),
                    *destination,
                    movement_state,
                ) {
                    return refused(AiRefusal::Build(refusal));
                }
            }
            // The AI's order goes through the same validator as the
            // player's (FUN_005385f0).
            if let Some(refusal) = rebellion_core::build_selection::order_refusal(
                world,
                mfg_state,
                troop_transport,
                deliveries,
                *system,
                ProductionArea::of(*kind),
                *kind,
                *count,
                ai_state.faction == Some(rebellion_core::ai::AiFaction::Alliance),
                movement_state,
            ) {
                return refused(AiRefusal::Build(refusal));
            }
            // The Build replaces what the area was building (FUN_00529ab0:
            // FUN_0052a2c0, then FUN_0052a140), as the player's does.
            mfg_state.build(*system, &QueueItem::new(*kind, *work), *count);
            // FUN_0052a140 creates the ordered products at once, a fleet's
            // squadrons and regiments aboard the ships the leg chose.
            rebellion_core::carriage::seat_products(world, troop_transport, deliveries, mfg_state);
            rebellion_core::manufacturing::seat_ordered_ships(world, mfg_state);
            applied
        }
        AIAction::Scrap { targets } => {
            // FUN_004f9860: the order's object check refuses another side's
            // object.
            let side = rebellion_core::dat::Faction::from(mission_faction);
            if targets.iter().any(|&target| {
                rebellion_core::scrap::target_side(world, target).is_some_and(|owner| owner != side)
            }) {
                return refused(AiRefusal::ScrapOtherSide);
            }
            let report = rebellion_core::scrap::scrap_all_reporting(
                world,
                stockpiles,
                troop_transport,
                targets.iter().copied(),
            );
            AiOutcome::Applied {
                mission: None,
                aboard: report.losses.iter().fold(
                    rebellion_core::scrap::Aboard::default(),
                    |total, loss| rebellion_core::scrap::Aboard {
                        squadrons: total.squadrons + loss.aboard.squadrons,
                        regiments: total.regiments + loss.aboard.regiments,
                    },
                ),
            }
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
                tick,
            )
            .map_or_else(|error| refused(AiRefusal::Fleet(error)), |_| applied)
        }
        AIAction::MoveRegiments { troops, to } => {
            use rebellion_core::troop_transport::TroopTransportError;
            let is_alliance = mission_faction == MissionFaction::Alliance;
            let blockaded = BlockadeSystem::blockaded_now(world);
            let products = rebellion_core::carriage::ProductRoom::new(mfg_state, deliveries);
            let mut first_error = None;
            let mut moved = false;
            for troop in troops {
                // FUN_004f9860: the object check refuses another side's
                // regiment.
                if world.troops.get(*troop).is_some_and(|value| value.is_alliance != is_alliance) {
                    first_error.get_or_insert(TroopTransportError::WrongFaction);
                    continue;
                }
                match troop_transport.move_regiment(
                    world,
                    movement_state,
                    &blockaded,
                    &products,
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
            use rebellion_core::troop_transport::{SquadronSource, TroopTransportError};
            let is_alliance = mission_faction == MissionFaction::Alliance;
            let blockaded = BlockadeSystem::blockaded_now(world);
            let products = rebellion_core::carriage::ProductRoom::new(mfg_state, deliveries);
            let mut first_error = None;
            let mut moved = false;
            for source in squadrons {
                // FUN_004f9860: the object check refuses another side's
                // squadron.
                let owner = match *source {
                    SquadronSource::Fleet { fleet, .. } => {
                        world.fleets.get(fleet).map(|value| value.is_alliance)
                    }
                    SquadronSource::System { is_alliance, .. } => Some(is_alliance),
                };
                if owner.is_some_and(|owner| owner != is_alliance) {
                    first_error.get_or_insert(TroopTransportError::WrongFaction);
                    continue;
                }
                match troop_transport.move_squadron(
                    world,
                    movement_state,
                    &blockaded,
                    &products,
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
        // The strikes and the assault go to resolve_ai_strike and
        // resolve_ai_assault, which hold the blockade and Death Star states.
        AIAction::Bombard { .. } | AIAction::DestroySystem { .. } | AIAction::Assault { .. } => {
            refused(AiRefusal::StrikeSystemMissing)
        }
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
            tick,
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
            // The player's Move validator: the object check (FUN_004f9860:
            // side, en route), the origin and destination, the speed and
            // the one-Death-Star room (FUN_00551630).
            if let Err(error) = rebellion_core::movement::validate_fleet_dispatch(
                movement_state,
                world,
                *fleet,
                *to_system,
                mission_faction == MissionFaction::Alliance,
            ) {
                return refused(AiRefusal::Fleet(FleetOrderError::Dispatch(error)));
            }
            let f = &world.fleets[*fleet];
            // FUN_004fda10: a fleet with no usable ship is not mobile and
            // changes container at once (FUN_00555920.c:31-33,
            // FUN_00556390.c:12-14); a mobile one needs a speed.
            let at_once = rebellion_core::movement::moves_at_once(f);
            let transit =
                rebellion_core::movement::fleet_transit_ticks(f, world, f.location, *to_system);
            if !troops.is_empty() {
                let products = rebellion_core::carriage::ProductRoom::new(mfg_state, deliveries);
                if let Err(error) = troop_transport.embark(world, &products, *fleet, troops) {
                    return refused(AiRefusal::Troops(error));
                }
            }
            let departed = if at_once {
                rebellion_core::movement::move_at_once(world, *fleet, *to_system);
                true
            } else {
                transit.is_some_and(|ticks| {
                    begin_fleet_transit(movement_state, world, *fleet, *to_system, ticks)
                })
            };
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

/// What a strike did, for the caller's records and messages.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StrikeEffects {
    /// The run (`bombardment::StrikeReport`); `None` when nothing ran (no
    /// flagged fleet at the system, or two sides flagged).
    pub report: Option<rebellion_core::bombardment::StrikeReport>,
    /// The bombarder's support points the finalize moved at the system
    /// (GNPRTB 7705, applied at once).
    pub support_change: i32,
    /// The shifts the run posted, fired at its end (event `0x373`).
    pub shifts: Vec<rebellion_core::support_shift::AppliedShift>,
    /// The control changes those shifts made through the support hook
    /// (`FUN_005109f0`, event `0x31b`), in the step of the run.
    pub controls: Vec<rebellion_core::support_shift::ControlRerun>,
    /// The destroy cascade, when the Death Star destroyed the system: what
    /// died with it (the HQ among them, whose removal shift `resolve_strike`
    /// already posted and fired, `FUN_00517cd0`; a caller adds none), the
    /// characters left adrift and the autoscrapped objects.
    pub cascade: Option<rebellion_core::death_star::CascadeReport>,
    /// The destroyed system's control before the kill turned it neutral;
    /// `None` when it was neutral already or nothing died.
    pub previous_control: Option<ControlKind>,
    /// The hidden trips the destroyed system's adrift sweep started the day
    /// of the kill (event `0x30e`, `death_star::sweep_adrift`).
    pub adrift_trips: Vec<AdriftTrip>,
    /// The trips of the characters the run's holder changes sent off
    /// ([`sweep_requests`]), by the system they left.
    pub swept: Vec<(SystemKey, Vec<AdriftTrip>)>,
    /// The Operation Report an ordinary run files (message kind `0xf`).
    /// Destroy System files none: `FUN_00488de0` maps a destroyed system's
    /// event `0x14f` to no kind.
    pub operation: Option<rebellion_core::operation_report::OperationReport>,
}

/// Phase 5 in `step` ([`rebellion_core::support_shift::fire`]), then each
/// holder change's production managers, which took the new side inside
/// it, lose their products
/// ([`rebellion_core::manufacturing::serve_manager_sides`],
/// `FUN_0052b790`). Every operation that serves its support events serves
/// them through this, so no save holds a holder change unserved.
pub fn fire_holder_changes(
    world: &mut GameWorld,
    manufacturing: &mut ManufacturingState,
    step: u64,
) -> rebellion_core::support_shift::Fired {
    let fired = rebellion_core::support_shift::fire(world, step);
    rebellion_core::manufacturing::serve_manager_sides(world, manufacturing);
    fired
}

/// Event `0x307` for each holder change an order made: what is bound for
/// the system takes its autoscrap request
/// ([`rebellion_core::death_star::serve_holder_requests`]). Served right
/// after the order's phase 5 ([`fire_holder_changes`]), so no save holds
/// one unserved.
fn serve_holder_requests(world: &mut GameWorld, states: &mut StrikeStates<'_>) {
    rebellion_core::death_star::serve_holder_requests(
        world,
        states.movement,
        states.transport,
        states.manufacturing,
        states.deliveries,
    );
}

/// [`rebellion_core::death_star::sweep_requests`]: the sweep of each system
/// whose characters a holder change flagged (event `0x307`), with each
/// trip as its record names it.
pub fn sweep_requests(
    world: &mut GameWorld,
    missions: &mut MissionState,
    transport: &TroopTransportState,
    movement: &MovementState,
    blockade: &rebellion_core::blockade::BlockadeState,
    tick: u64,
) -> Vec<(SystemKey, Vec<AdriftTrip>)> {
    rebellion_core::death_star::sweep_requests(world, missions, transport, movement, blockade, tick)
        .into_iter()
        .map(|(system, started)| (system, AdriftTrip::started(missions, &started)))
        .collect()
}

/// A hidden trip the adrift sweep started, as its `mission_dispatched`
/// record names it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdriftTrip {
    pub mission_id: u64,
    pub faction: MissionFaction,
    pub team: Vec<MissionMember>,
    pub target_system: SystemKey,
}

impl AdriftTrip {
    /// The trips among `missions` whose ids the sweep returned.
    fn started(missions: &MissionState, started: &[u64]) -> Vec<Self> {
        missions
            .missions()
            .iter()
            .filter(|mission| started.contains(&mission.id))
            .map(|mission| Self {
                mission_id: mission.id,
                faction: mission.faction,
                team: mission.team.clone(),
                target_system: mission.target_system,
            })
            .collect()
    }
}

impl StrikeEffects {
    /// The system the Death Star destroyed.
    #[must_use]
    pub fn destroyed(&self) -> Option<SystemKey> {
        self.report
            .as_ref()
            .filter(|report| report.planet_destroyed)
            .and_then(|report| report.system)
    }
}

/// The states a strike reads and writes besides the world.
pub struct StrikeStates<'a> {
    pub movement: &'a mut MovementState,
    pub manufacturing: &'a mut ManufacturingState,
    pub transport: &'a mut TroopTransportState,
    pub deliveries: &'a mut rebellion_core::delivery::DeliveryState,
    pub missions: &'a mut MissionState,
    pub uprisings: &'a UprisingState,
    /// The blockade loss counters a run's kills feed (`FUN_00508c60`).
    pub blockade: &'a mut rebellion_core::blockade::BlockadeState,
    /// The day the strike runs.
    pub tick: u64,
    /// The in-day step it runs in (`FUN_0051dea0` `+0x20`), whose
    /// control-change counters its changes share (`FUN_00518040`,
    /// [`rebellion_core::support_shift::step`]).
    pub step: u16,
}

/// One message-`0x224` run at `system` (`FUN_005637c0` -> `FUN_0058d800`):
/// Destroy System's cascade and character roll when a Death Star is among
/// the flagged fleets, else the defence and effect phases; then the
/// finalize (`FUN_0058e320`) and the flags' clearing. `roll` seeds the run.
pub fn resolve_strike(
    world: &mut GameWorld,
    system: SystemKey,
    kind: rebellion_core::bombardment::BombardKind,
    states: &mut StrikeStates<'_>,
    roll: f64,
) -> StrikeEffects {
    use rebellion_core::bombardment::{self, BombardmentRun, Opening, StrikeReport};
    let mut effects = StrikeEffects::default();
    let points = |world: &GameWorld, side| {
        world
            .systems
            .get(system)
            .map_or(0, |sys| rebellion_core::uprising::support_points(sys, side))
    };
    match bombardment::open(world, system, kind) {
        Opening::Idle => {}
        Opening::Kill { bombarder } => {
            let holder = bombardment::holder(world, system);
            // FUN_0058ce70: the system's units die with reason 0x10 at the
            // bombarder's own system, else 0xf; each one's event 0x300
            // feeds the loss counters (FUN_00508c60).
            let reason = if holder == bombarder {
                rebellion_core::blockade::REASON_OWN_BOMBARDMENT
            } else {
                rebellion_core::blockade::REASON_BOMBARDMENT
            };
            for loss in rebellion_core::support_shift::system_unit_losses(world, system, reason) {
                states.blockade.record_loss(system, loss);
            }
            // FUN_0050b760 clears the populated bit before the children
            // die; its hook +0x20c (FUN_00510e30) reruns FUN_0050b610 at
            // once. With no regiments the holder is 3 (FUN_0050b610.c:26-28):
            // the system turns neutral (the VM's Yaga Minor) and its owner
            // change runs first, the HQ's removal (FUN_004f8680, +0xac(3)).
            // With regiments the troop bits keep the troop side
            // (.c:34-70) until the last dies: each kill clears its usable
            // bit (FUN_004f7410 -> FUN_004fb310, event 0x301), whose
            // FUN_00508660 reruns FUN_0050b610 when phase 5 serves it, and
            // the holder then turns neutral with the troop bits' record
            // (SDPRTB 0x1442, FUN_0055de50).
            let had_headquarters =
                rebellion_core::support_shift::alliance_headquarters(world, system);
            world.systems[system].is_populated = false;
            rebellion_core::support_shift::rebuild_control(world, system);
            // The cascade's own destroy of an HQ the control change left
            // (a system already neutral): FUN_00517cd0.
            rebellion_core::support_shift::remove_headquarters(world, system);
            let mut cascade = destroy_system(world, system, states);
            cascade.headquarters |= had_headquarters;
            let headquarters = cascade.headquarters;
            let adrift = !cascade.adrift.is_empty();
            effects.cascade = Some(cascade);
            let characters = bombardment::roll_characters(world, system, roll);
            let run = BombardmentRun {
                support_loss: true,
                planet_destroyed: true,
            };
            let before = points(world, bombarder);
            bombardment::finalize(world, system, bombarder, run);
            effects.support_change = points(world, bombarder) - before;
            bombardment::clear_flags(world, system, bombarder);
            // Each adrift flag posts event 0x30e (FUN_00534790 ->
            // FUN_00536460), served after the order in its frame step, so
            // after the character roll: the first sweeps the system, and a
            // change of the battle bit posts event 0x304, whose sweep
            // dispatches. The neutral holder is side 3 (FUN_0050b610), which
            // the rule rerun (FUN_0050b8e0) treats as held: with fleets of
            // both sides in orbit the bit stays set and the sweep only
            // gathers. The dead regiments' events (0x301, FUN_00508660) turn
            // a garrisoned system neutral; that change's owner hook
            // requests the rule rerun (FUN_00510820 -> FUN_0050ac10, event
            // 0x311 at 0x400, FUN_0048e670), which phase 5 serves after
            // every holder event (0x10) and shift (0x100) below it
            // (FUN_0054ee90, FUN_005f61c0).
            let fired = fire_holder_changes(
                world,
                states.manufacturing,
                rebellion_core::support_shift::step(states.tick, states.step),
            );
            serve_holder_requests(world, states);
            effects.shifts = fired.shifts;
            effects.controls = fired.controls;
            let battle_changed = states.blockade.refresh_battle(world, system).is_some();
            if adrift || battle_changed {
                let started = rebellion_core::death_star::sweep_adrift(
                    world,
                    states.missions,
                    states.transport,
                    states.movement,
                    system,
                    states.blockade.in_battle(system),
                    states.tick,
                );
                effects.adrift_trips = AdriftTrip::started(states.missions, &started);
            }
            effects.report = Some(StrikeReport {
                system: Some(system),
                kind: Some(kind),
                bombarder: Some(bombarder),
                holder: Some(holder),
                planet_destroyed: true,
                headquarters,
                characters,
                run,
                ..StrikeReport::default()
            });
        }
        Opening::Run { bombarder, fleets } => {
            let mut report = bombardment::bombard(world, system, kind, bombarder, &fleets, roll);
            // Event 0x300 for each destroyed unit (FUN_00508c60).
            for &loss in &report.losses {
                states.blockade.record_loss(system, loss);
            }
            // A fleet the batteries emptied goes, as a battle's does
            // (`post_battle::destroy_fleet`), and regiments aboard a lost
            // transport with it.
            if report.ships_destroyed > 0 {
                // Each regiment lost with its transport is a destroyed
                // object aboard a fleet (FUN_00497380: +0x88). It dies with
                // its ship's reason 0xf: the ship's hook +0x11c reaches
                // FUN_004fb520, whose walk calls the regiment's slot +0xb4,
                // FUN_004f8710 (vtable 0x0065e438), and FUN_004f7ed0 passes
                // the container's reason on; its event 0x300 adds its slot
                // +0x1c0 (TROOPSD uprising_defense) to B (FUN_00508c60).
                let cargo: Vec<_> = fleets
                    .iter()
                    .flat_map(|&fleet| states.transport.cargo(fleet).to_vec())
                    .filter_map(|troop| {
                        let unit = rebellion_core::operation_report::regiment(world, troop)?;
                        let regiment = world.troops.get(troop)?;
                        let loss = rebellion_core::blockade::UnitLoss {
                            side: rebellion_core::dat::Faction::of_alliance(regiment.is_alliance),
                            value: rebellion_core::support_shift::regiment_value(
                                world,
                                regiment.class_dat_id,
                            ),
                            reason: rebellion_core::blockade::REASON_BOMBARDMENT,
                        };
                        Some((troop, unit, loss))
                    })
                    .collect();
                for &fleet in &fleets {
                    if world
                        .fleets
                        .get(fleet)
                        .is_some_and(rebellion_core::world::Fleet::is_empty)
                    {
                        rebellion_core::post_battle::destroy_fleet(
                            world,
                            states.transport,
                            fleet,
                            states.tick,
                        );
                    }
                }
                states.transport.destroy_untransportable_cargo(world);
                for (troop, unit, loss) in cargo {
                    if world.troops.contains_key(troop) {
                        continue;
                    }
                    states.blockade.record_loss(system, loss);
                    report.losses.push(loss);
                    report.fleet_destroyed.push(unit);
                }
            }
            let before = points(world, bombarder);
            bombardment::finalize(world, system, bombarder, report.run);
            effects.support_change = points(world, bombarder) - before;
            bombardment::clear_flags(world, system, bombarder);
            // The flags' clearing raises event 0x182 with value 0 for each
            // fleet (FUN_004fe460(0) -> FUN_004ff840) and the system's
            // 0x14f with value 0, which fill the report's operational lists
            // after the run (FUN_00496770, FUN_00496f50).
            let mut operation = rebellion_core::operation_report::OperationReport::new(
                system,
                false,
                report.holder.unwrap_or_default(),
                bombarder,
            );
            operation.fill_fleets(world, states.transport, &fleets);
            operation.fill_system(world);
            operation
                .fleet_destroyed
                .clone_from(&report.fleet_destroyed);
            operation
                .system_destroyed
                .clone_from(&report.system_destroyed);
            effects.operation = Some(operation);
            effects.report = Some(report);
        }
    }
    // Phase 5: the regiments' holder reruns and the holder changes (event
    // 0x31b), then the shifts (event 0x373), the run's and those the
    // battle rerun posted.
    let fired = fire_holder_changes(
        world,
        states.manufacturing,
        rebellion_core::support_shift::step(states.tick, states.step),
    );
    serve_holder_requests(world, states);
    effects.shifts.extend(fired.shifts);
    effects.controls.extend(fired.controls);
    if let Some(system) = effects.destroyed() {
        if let Some(previous) = effects
            .controls
            .iter()
            .find(|change| change.system == system)
            .map(|change| change.previous)
        {
            effects.previous_control = Some(previous);
        }
    }
    // The characters the holder changes flagged leave (event 0x307).
    effects.swept = sweep_requests(
        world,
        states.missions,
        states.transport,
        states.movement,
        states.blockade,
        states.tick,
    );
    effects
}

/// A Planetary Bombardment order (`0x220`-`0x223`) for a team of fleets:
/// the enable test (`bombardment::gate`), one flag command per fleet
/// (`FUN_004fec00`; a fleet in transit is refused), then one `0x224` run per
/// system holding a flagged fleet, all in the step that issued it
/// (`FUN_005820a0`; `ghidra/notes/bombardment.md`). `roll` draws one seed
/// per run.
///
/// # Errors
/// The enable test's refusal.
pub fn order_strike(
    world: &mut GameWorld,
    fleets: &[FleetKey],
    kind: rebellion_core::bombardment::BombardKind,
    states: &mut StrikeStates<'_>,
    roll: &mut dyn FnMut() -> f64,
) -> Result<Vec<StrikeEffects>, rebellion_core::bombardment::BombardRefusal> {
    rebellion_core::bombardment::gate(world, fleets, kind)?;
    let mut systems: Vec<SystemKey> = Vec::new();
    for &fleet in fleets {
        let in_transit = states.movement.is_in_transit(fleet);
        if rebellion_core::bombardment::flag_fleet(world, fleet, in_transit) {
            let location = world.fleets[fleet].location;
            if !systems.contains(&location) {
                systems.push(location);
            }
        }
    }
    Ok(systems
        .into_iter()
        .map(|system| resolve_strike(world, system, kind, states, roll()))
        .collect())
}

/// Resolve an AI strike, a Bombard or a DestroySystem, through the player's
/// order (`order_strike`). It writes no record; the caller emits from the
/// effects.
pub fn resolve_ai_strike(
    action: &AIAction,
    side: rebellion_core::dat::Faction,
    world: &mut GameWorld,
    states: &mut StrikeStates<'_>,
    roll: &mut dyn FnMut() -> f64,
) -> (AiOutcome, StrikeEffects) {
    use rebellion_core::bombardment::BombardKind;
    let (fleet, system, kind) = match action {
        AIAction::Bombard {
            fleet,
            system,
            kind,
        } => (
            *fleet,
            *system,
            match kind {
                rebellion_core::ai_regions::Strike::General => BombardKind::General,
                rebellion_core::ai_regions::Strike::Military => BombardKind::Military,
            },
        ),
        AIAction::DestroySystem { fleet, system } => (*fleet, *system, BombardKind::DestroySystem),
        _ => {
            return (
                AiOutcome::Refused(AiRefusal::StrikeSystemMissing),
                StrikeEffects::default(),
            )
        }
    };
    if !world.fleets.contains_key(fleet) {
        return (
            AiOutcome::Refused(AiRefusal::Fleet(
                rebellion_core::fleet_join::FleetOrderError::MissingFleet,
            )),
            StrikeEffects::default(),
        );
    }
    // FUN_004f9860: the order's object check refuses another side's fleet.
    if world.fleets[fleet].is_alliance != (side == rebellion_core::dat::Faction::Alliance) {
        return (
            AiOutcome::Refused(AiRefusal::Fleet(
                rebellion_core::fleet_join::FleetOrderError::WrongFaction,
            )),
            StrikeEffects::default(),
        );
    }
    if world.fleets[fleet].location != system || !world.systems.contains_key(system) {
        return (
            AiOutcome::Refused(AiRefusal::StrikeSystemMissing),
            StrikeEffects::default(),
        );
    }
    match order_strike(world, &[fleet], kind, states, roll) {
        // The fleet command refused the fleet (FUN_004fec00, en route
        // 1/0x11): nothing was flagged, so the order failed.
        Ok(runs) if runs.is_empty() => (
            AiOutcome::Refused(AiRefusal::Strike(
                rebellion_core::bombardment::BombardRefusal::FleetRefused,
            )),
            StrikeEffects::default(),
        ),
        Ok(mut runs) => (
            AiOutcome::applied(None),
            runs.pop().unwrap_or_default(),
        ),
        Err(refusal) => (
            AiOutcome::Refused(AiRefusal::Strike(refusal)),
            StrikeEffects::default(),
        ),
    }
}

/// What an assault did, for the caller's records and messages.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AssaultEffects {
    /// The run (`assault::AssaultReport`) with its `holder_after`; `None`
    /// when nothing ran.
    pub report: Option<rebellion_core::assault::AssaultReport>,
    /// The system's control before the run, when the run changed it.
    pub previous_control: Option<ControlKind>,
    /// The shifts the control change posted, fired at the run's end (event
    /// `0x373`).
    pub shifts: Vec<rebellion_core::support_shift::AppliedShift>,
    /// The control changes those shifts made through the support hook
    /// (`FUN_005109f0`, event `0x31b`), in the step of the run.
    pub controls: Vec<rebellion_core::support_shift::ControlRerun>,
    /// The Operation Report the run's events fill (message kind `0xf`).
    pub operation: Option<rebellion_core::operation_report::OperationReport>,
    /// The blockade transition the holder change made at the system (event
    /// `0x311`, `BlockadeSystem::refresh_system`), already applied: the
    /// state holds it and an end's support shift fired with `shifts`. Each
    /// carries its telemetry payload, taken as it happened.
    pub blockade: Vec<(BlockadeEvent, serde_json::Value)>,
    /// The trips of the characters the holder change sent off
    /// ([`sweep_requests`]), by the system they left.
    pub swept: Vec<(SystemKey, Vec<AdriftTrip>)>,
}

/// One `0x234` system run at `system` for the team's `fleets`
/// (`FUN_00563390` -> `FUN_0058c1e0`, `assault::resolve`): each dead
/// regiment's event `0x300` feeds the loss counters (`FUN_00508c60`); then
/// phase 5 serves each kill's event `0x301` and each landing's `0x320`,
/// whose system hook `FUN_00508660` rebuilds the holder from the regiments
/// left (`FUN_0050b610`), with the facilities' hand-over, the owner
/// change's child walk and event `0x31b`'s shifts (`FUN_00510d70`), which
/// fire before the blockade rule's rerun (event `0x311`, priority `0x400`
/// against `0x10` and `0x100`, `FUN_0054ee90`; `support_shift::fire`). A
/// holder change reruns the rule for the system
/// (`BlockadeSystem::refresh_system`): a blockade the new holder's fleet
/// ends posts its loss shift (`support_shift::blockade_ended`), which the
/// run's end fires. The run itself touches no character; the
/// holder change sends the old holder's off and frees its prisoners of the
/// new side (`FUN_004ef260`, event `0x307`, [`sweep_requests`];
/// `ghidra/notes/ground-assault.md`, section 4). `roll` seeds the run.
pub fn resolve_assault(
    world: &mut GameWorld,
    system: SystemKey,
    fleets: &[FleetKey],
    states: &mut StrikeStates<'_>,
    roll: f64,
) -> AssaultEffects {
    use rebellion_core::operation_report;
    use rebellion_core::world::FacilityRef;
    use std::collections::HashMap;
    // Each regiment and facility a kill may take, named before it leaves
    // the world (event 0x302 carries the dead object, FUN_00497380).
    let team: Vec<FleetKey> = fleets
        .iter()
        .copied()
        .filter(|&fleet| world.fleets.get(fleet).is_some_and(|f| f.location == system))
        .collect();
    let units: HashMap<TroopKey, operation_report::ReportUnit> = team
        .iter()
        .flat_map(|&fleet| states.transport.cargo(fleet).to_vec())
        .chain(world.systems.get(system).map_or_else(Vec::new, |sys| sys.ground_units.clone()))
        .filter_map(|troop| operation_report::regiment(world, troop).map(|unit| (troop, unit)))
        .collect();
    let facilities: Vec<(FacilityRef, operation_report::ReportUnit)> = world
        .systems
        .get(system)
        .map_or_else(Vec::new, |sys| {
            sys.defense_facilities
                .iter()
                .map(|&key| FacilityRef::Defense(key))
                .chain(sys.manufacturing_facilities.iter().map(|&key| FacilityRef::Manufacturing(key)))
                .chain(sys.production_facilities.iter().map(|&key| FacilityRef::Production(key)))
                .collect()
        })
        .into_iter()
        .filter_map(|key| operation_report::facility(world, key).map(|unit| (key, unit)))
        .collect();
    let previous = world.systems.get(system).map(|sys| sys.control);
    let mut report =
        rebellion_core::assault::resolve(world, states.transport, system, fleets, roll);
    let (Some(assaulter), Some(holder)) = (report.assaulter, report.holder_before) else {
        return AssaultEffects::default();
    };
    for &loss in &report.losses {
        states.blockade.record_loss(system, loss);
    }
    let mut effects = AssaultEffects::default();
    let step = rebellion_core::support_shift::step(states.tick, states.step);
    // Phase 5 serves the run's regiment events (FUN_00508660 ->
    // FUN_0050b610), the holder change they make (event 0x31b) and its
    // shifts (0x373), whose owner hook requests the blockade rule
    // (FUN_00510820 -> FUN_0050ac10, event 0x311): 0x311's priority 0x400
    // (FUN_0048e670) runs it after every event below it (FUN_0054ee90,
    // FUN_005f61c0).
    let fired = fire_holder_changes(world, states.manufacturing, step);
    serve_holder_requests(world, states);
    effects.shifts = fired.shifts;
    effects.controls = fired.controls;
    effects.previous_control =
        previous.filter(|&previous| world.systems[system].control != previous);
    // Each owner change at the system requests the rerun (FUN_00511b30),
    // one that a shift turned back included.
    if effects.controls.iter().any(|change| change.system == system) {
        if let Some(event) =
            BlockadeSystem::refresh_system(states.blockade, world, system, states.tick)
        {
            let payload = match event {
                BlockadeEvent::BlockadeEnded { .. } => {
                    let payload =
                        crate::telemetry::blockade_end_payload(world, states.blockade, system);
                    rebellion_core::support_shift::blockade_ended(world, states.blockade, system);
                    payload
                }
                _ => crate::telemetry::blockade_payload(world, system),
            };
            effects.blockade.push((event, payload));
        }
    }
    report.holder_after = Some(rebellion_core::bombardment::holder(world, system));
    let fired = fire_holder_changes(world, states.manufacturing, step);
    serve_holder_requests(world, states);
    effects.shifts.extend(fired.shifts);
    effects.controls.extend(fired.controls);
    // The characters the holder change flagged leave (event 0x307).
    effects.swept = sweep_requests(
        world,
        states.missions,
        states.transport,
        states.movement,
        states.blockade,
        states.tick,
    );
    // The cleanup (FUN_0058c8b0) raises the system's event 0x150 with
    // value 0 (:19), then 0x183 with value 0 for each fleet (:27), which
    // fill the report's operational lists after the run in that order
    // (FUN_00496f50, FUN_00496770): the landed regiments list before the
    // ones still aboard. Each kill's event 0x302 lists a regiment aboard a
    // ship with the fleet side (+0x88) and an object at the system with it
    // (+0x80, FUN_00497380).
    let mut operation = operation_report::OperationReport::new(system, true, holder, assaulter);
    operation.fill_assaulted_system(world);
    operation.fill_fleets(world, states.transport, &team);
    operation.fleet_destroyed = report
        .attackers_lost
        .iter()
        .filter_map(|(troop, _)| units.get(troop).cloned())
        .collect();
    operation.system_destroyed = report
        .defenders_lost
        .iter()
        .filter_map(|troop| units.get(troop).cloned())
        .chain(report.facilities_lost.iter().filter_map(|lost| {
            facilities
                .iter()
                .find(|(key, _)| key == lost)
                .map(|(_, unit)| unit.clone())
        }))
        .collect();
    effects.operation = Some(operation);
    effects.report = Some(report);
    effects
}

/// A Planetary Assault order (`0x234`) by `side` for a team of fleets: the
/// enable test (`assault::order_enabled`, `FUN_005814d0`), each fleet
/// command's flag (`FUN_004feb60`, which the enable test already passed),
/// then one system run per distinct system of the team (`FUN_00581850`),
/// all in the step that issued it. `roll` draws one seed per run.
///
/// # Errors
/// The enable test's refusal.
pub fn order_assault(
    world: &mut GameWorld,
    fleets: &[FleetKey],
    side: rebellion_core::dat::Faction,
    states: &mut StrikeStates<'_>,
    roll: &mut dyn FnMut() -> f64,
) -> Result<Vec<AssaultEffects>, rebellion_core::assault::AssaultRefusal> {
    rebellion_core::assault::order_enabled(
        world,
        states.movement,
        states.transport,
        states.blockade,
        fleets,
        side,
    )?;
    let mut systems: Vec<SystemKey> = Vec::new();
    for &fleet in fleets {
        let location = world.fleets[fleet].location;
        if !systems.contains(&location) {
            systems.push(location);
        }
    }
    Ok(systems
        .into_iter()
        .map(|system| resolve_assault(world, system, fleets, states, roll()))
        .collect())
}

/// Resolve an AI Planetary Assault (`FUN_004b1070` -> `FUN_004b1330(fleet,
/// 0x234)`, the fleet as the team) through the player's order
/// (`order_assault`). A refusal fails the order; the AI tries no
/// bombardment in its place. It writes no record; the caller emits from
/// the effects.
pub fn resolve_ai_assault(
    action: &AIAction,
    side: rebellion_core::dat::Faction,
    world: &mut GameWorld,
    states: &mut StrikeStates<'_>,
    roll: &mut dyn FnMut() -> f64,
) -> (AiOutcome, AssaultEffects) {
    let AIAction::Assault { fleet, system } = *action else {
        return (
            AiOutcome::Refused(AiRefusal::StrikeSystemMissing),
            AssaultEffects::default(),
        );
    };
    let Some(value) = world.fleets.get(fleet) else {
        return (
            AiOutcome::Refused(AiRefusal::Fleet(
                rebellion_core::fleet_join::FleetOrderError::MissingFleet,
            )),
            AssaultEffects::default(),
        );
    };
    // FUN_004f9860: the order's object check refuses another side's fleet.
    if value.is_alliance != (side == rebellion_core::dat::Faction::Alliance) {
        return (
            AiOutcome::Refused(AiRefusal::Fleet(
                rebellion_core::fleet_join::FleetOrderError::WrongFaction,
            )),
            AssaultEffects::default(),
        );
    }
    if value.location != system || !world.systems.contains_key(system) {
        return (
            AiOutcome::Refused(AiRefusal::StrikeSystemMissing),
            AssaultEffects::default(),
        );
    }
    match order_assault(world, &[fleet], side, states, roll) {
        Ok(mut runs) => (
            AiOutcome::applied(None),
            runs.pop().unwrap_or_default(),
        ),
        Err(refusal) => (
            AiOutcome::Refused(AiRefusal::Assault(refusal)),
            AssaultEffects::default(),
        ),
    }
}

/// A planet the Death Star destroyed (`FUN_0058ce70` → system slot `+0xac`):
/// its destroyed bit (`FUN_004f7560`), the destroy cascade
/// (`death_star::cleanup_destroyed_system`), then the mission validator
/// its event `0x300` runs (`MissionState::validate_destroyed`). The win
/// word follows from the same event (`FUN_005180e0`: AD, CD), which the
/// victory check reads. hyp: the validator runs after the child walk; the
/// order of event `0x300`'s handlers against the walk is not read.
pub fn destroy_system(
    world: &mut GameWorld,
    system: SystemKey,
    states: &mut StrikeStates<'_>,
) -> rebellion_core::death_star::CascadeReport {
    if let Some(sys) = world.systems.get_mut(system) {
        sys.is_destroyed = true;
    }
    let report = rebellion_core::death_star::cleanup_destroyed_system(
        world,
        system,
        states.movement,
        states.transport,
        states.manufacturing,
        states.deliveries,
    );
    // The managers' queues died with their products (FUN_0052a2c0).
    rebellion_core::manufacturing::seat_ordered_ships(world, states.manufacturing);
    states
        .missions
        .validate_destroyed(world, states.uprisings, system);
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::ai::{AiFaction, FleetMoveReason};
    use rebellion_core::carriage::ProductRoom;
    use rebellion_core::dat::{ExplorationStatus, Faction, SectorGroup};
    use rebellion_core::missions::{MissionKind, MissionOutcome};
    use rebellion_core::world::{CapitalShipClass, Fleet, Sector, ShipInstance, System};

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
        });
        let transit = |leg| rebellion_core::troop_transport::RegimentTransit {
            troop,
            origin,
            leg,
            carrier: 0,
            arrival_tick: 5,
            via: None,
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
                next_support_tick: None,
            },
        );
        let ended = apply_mission_state_effects(
            &[MissionEffect::UprisingSubdued {
                system,
                side: Faction::Empire,
                support_gain: 5,
            }],
            &mut world,
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
                    next_support_tick: None,
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
                &mut ManufacturingState::new(),
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
                next_support_tick: None,
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
            &mut ManufacturingState::new(),
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
            &mut ManufacturingState::new(),
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
    fn a_holder_change_hands_facilities_to_the_new_holder() {
        // FUN_00510d70 → FUN_004f6f40 → FUN_004fae40: a facility whose class
        // serves the new holder passes to it, any other is removed
        // (ghidra/notes/facility-ownership.md). The holder changes as an
        // Alliance regiment lands (FUN_00508660 -> FUN_0050b610).
        let mut world = GameWorld::default();
        let system = empire_facilities(&mut world, "Kuat");
        garrison(&mut world, system, Faction::Alliance);
        rebellion_core::support_shift::regiments_changed(&mut world, system);
        PerceptionIntegrator::new(1, 0)
            .fire_support_shifts(&mut world, &mut ManufacturingState::new());
        assert_alliance_took_over(&world, system);
    }

    /// GNPRTB 7761 (60) and 7762 (-10) as shipped: the assault's landing
    /// limit `FUN_00559fe0(side, 0)` is 6.
    fn landing_limit() -> Vec<rebellion_core::world::GnprtbEntry> {
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
            .collect()
    }

    /// `side`'s fleet of one transport at `system` with `count` regiments
    /// of `class` aboard. port: test fixture values, not rules.
    fn assault_fleet(
        world: &mut GameWorld,
        transport: &mut TroopTransportState,
        system: SystemKey,
        side: Faction,
        class: DatId,
        count: usize,
    ) -> (FleetKey, Vec<TroopKey>) {
        let is_alliance = side == Faction::Alliance;
        let ship = world.capital_ship_classes.insert(CapitalShipClass {
            family: 0x14,
            hull: 100,
            troop_capacity: 10,
            ..Default::default()
        });
        let fleet = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ShipInstance::new(ship, 100, is_alliance)],
            is_alliance,
            ..Fleet::default()
        });
        world.systems[system].fleets.push(fleet);
        let troops: Vec<TroopKey> = (0..count)
            .map(|_| {
                let troop = world.troops.insert(rebellion_core::world::TroopUnit {
                    class_dat_id: class,
                    is_alliance,
                });
                world.systems[system].ground_units.push(troop);
                troop
            })
            .collect();
        transport
            .embark(world, &ProductRoom::default(), fleet, &troops)
            .unwrap();
        (fleet, troops)
    }

    // The holder change an assault makes serves its autoscrap requests in the
    // same order, after its phase 5 (event 0x307, FUN_00515480): the old
    // holder's product queued at its other yard and bound for the system
    // goes back to that yard, the hint (FUN_00515480.c:82-124).
    #[test]
    fn an_assault_that_takes_a_system_sends_the_old_holders_products_for_it_back_to_their_yard() {
        for (holder, assaulter) in [
            (Faction::Empire, Faction::Alliance),
            (Faction::Alliance, Faction::Empire),
        ] {
            let mut world = GameWorld::default();
            world.gnprtb = rebellion_core::world::GnprtbParams::new(landing_limit());
            let support = if holder == Faction::Alliance { 1.0 } else { 0.0 };
            let [system, yard] = ["Kuat", "Fondor"].map(|name| {
                let key = add_system(&mut world, name);
                let held = &mut world.systems[key];
                held.popularity_alliance = support;
                held.popularity_empire = 1.0 - support;
                held.control = ControlKind::Controlled(holder);
                key
            });
            let facility = BuildableKind::ManufacturingFacility(
                rebellion_core::manufacturing::FacilityBuild {
                    class: DatId(0x2900_0001),
                    is_alliance: holder == Faction::Alliance,
                },
            );
            let mut fixture = StrikeFixture::new();
            fixture.manufacturing.enqueue(
                yard,
                QueueItem::new(facility, 10).delivered_to(
                    rebellion_core::manufacturing::Destination::System(system),
                ),
            );
            let (fleet, _) = assault_fleet(
                &mut world,
                &mut fixture.transport,
                system,
                assaulter,
                DatId::new(0x1000_0001),
                1,
            );

            resolve_assault(&mut world, system, &[fleet], &mut fixture.states(), 0.5);

            assert_eq!(
                world.systems[system].control,
                ControlKind::Controlled(assaulter),
                "{holder:?}"
            );
            let destination = fixture
                .manufacturing
                .queue(yard, ProductionArea::ConstructionYard)
                .map(|queue| queue.items()[0].destination);
            assert_eq!(destination, Some(None), "{holder:?}");
            assert!(world.autoscrap_systems.is_empty(), "{holder:?}");
        }
    }

    #[test]
    fn an_assault_landing_hands_the_facilities_to_the_assaulter() {
        // The landing's regiment rebuilds the holder (FUN_0050b610), whose
        // change hands the facilities over as a resolved control change's
        // does (FUN_00510d70).
        let mut world = GameWorld::default();
        world.gnprtb = rebellion_core::world::GnprtbParams::new(landing_limit());
        let system = empire_facilities(&mut world, "Kuat");
        let mut fixture = StrikeFixture::new();
        let (fleet, troops) = assault_fleet(
            &mut world,
            &mut fixture.transport,
            system,
            Faction::Alliance,
            DatId::new(0x1000_0001),
            1,
        );
        let effects = resolve_assault(&mut world, system, &[fleet], &mut fixture.states(), 0.5);
        assert_eq!(world.systems[system].ground_units, troops);
        assert_alliance_took_over(&world, system);
        let report = effects.report.unwrap();
        assert_eq!(report.landed, troops);
        assert_eq!(report.holder_after, Some(Faction::Alliance));
        assert_eq!(
            effects.previous_control,
            Some(ControlKind::Controlled(Faction::Empire))
        );
    }

    #[test]
    fn a_partial_landing_lists_the_landed_regiments_before_those_still_aboard() {
        // FUN_0058c8b0 raises the system's 0x150 (:19) before each fleet's
        // 0x183 (:27); FUN_00496f50 lists the non-holder regiments at the
        // system in the fleet column (+0x78), then FUN_00496770 the ones
        // aboard. The landing stops at the limit of 6 (GNPRTB 7761/7762).
        for assaulter in [Faction::Alliance, Faction::Empire] {
            let holder = if assaulter == Faction::Alliance {
                Faction::Empire
            } else {
                Faction::Alliance
            };
            let mut world = GameWorld::default();
            world.gnprtb = rebellion_core::world::GnprtbParams::new(landing_limit());
            let system = add_system(&mut world, "Kuat");
            world.systems[system].control = ControlKind::Controlled(holder);
            for (raw, name) in [(0x1000_0001, "Alpha"), (0x1000_0002, "Beta")] {
                world.buildable_classes.insert(
                    DatId::new(raw),
                    rebellion_core::world::BuildableClass {
                        name: name.into(),
                        ..Default::default()
                    },
                );
            }
            let mut fixture = StrikeFixture::new();
            let (first, _) = assault_fleet(
                &mut world,
                &mut fixture.transport,
                system,
                assaulter,
                DatId::new(0x1000_0001),
                4,
            );
            let (second, _) = assault_fleet(
                &mut world,
                &mut fixture.transport,
                system,
                assaulter,
                DatId::new(0x1000_0002),
                4,
            );
            for fleet in [first, second] {
                let class = world.fleets[fleet].capital_ships[0].class;
                world.capital_ship_classes[class].dat_id = DatId::new(0x1400_0040);
                world.capital_ship_classes[class].name = "Transport".into();
            }
            let effects =
                resolve_assault(&mut world, system, &[first, second], &mut fixture.states(), 0.5);
            assert_eq!(effects.report.unwrap().landed.len(), 6, "{assaulter:?}");
            let report = effects.operation.expect("the Operation Report");
            assert_eq!(
                names(&report.fleet_operational),
                [
                    "Alpha",
                    "Alpha",
                    "Alpha",
                    "Alpha",
                    "Beta",
                    "Beta",
                    "Transport",
                    "Transport",
                    "Beta",
                    "Beta"
                ],
                "{assaulter:?}"
            );
        }
    }

    #[test]
    fn an_economy_event_files_its_own_record() {
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Naboo");
        let mut integrator = PerceptionIntegrator::new(1, 0);
        integrator.apply_economy_events(
            &world,
            &[EconomyEvent::CollectionRateChanged {
                system,
                new_rate: 0.5,
            }],
        );
        let records = integrator.finish();
        let record = records
            .iter()
            .find(|record| record.event_type == rebellion_core::game_events::EVT_COLLECTION_RATE)
            .expect("a collection_rate record");
        assert_eq!(record.details["system"], "Naboo");
        assert_eq!(record.details["rate"], 0.5);
    }

    #[test]
    fn a_days_drift_leaves_popular_support_unchanged() {
        // FUN_0050b230 hands the drift (FUN_00559c40) to FUN_00509ec0,
        // which stores it at system +0x6c for the diversion roll
        // (FUN_005166a0); nothing adds it to support (+0x58, whose only
        // setter is FUN_00509c30).
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Naboo");
        world.gnprtb = rebellion_core::world::GnprtbParams::new(
            [
                (7686, 10),
                (7687, 5),
                (7688, 2),
                (7732, 40),
                (7733, 25),
                (7734, 30),
                (7735, 50),
                (7736, 20),
                (7737, 75),
            ]
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
        world.systems[system].control = ControlKind::Controlled(Faction::Alliance);
        world.systems[system].popularity_alliance = 0.1;
        world.systems[system].popularity_empire = 0.9;
        let mut economy = rebellion_core::economy::EconomyState::default();
        let events = rebellion_core::economy::EconomySystem::advance(
            &mut economy,
            &mut world,
            &[rebellion_core::tick::TickEvent { tick: 1 }],
        );
        let mut integrator = PerceptionIntegrator::new(1, 0);
        integrator.apply_economy_events(&world, &events);
        assert!((world.systems[system].popularity_alliance - 0.1).abs() < 1e-6);
        assert!((world.systems[system].popularity_empire - 0.9).abs() < 1e-6);
        // 10 support, no fleet: base 75 (GNPRTB 7737), positive for side 1.
        assert_eq!(world.support_values[&system].support_drift, 75);
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
        apply_orders_of(
            AiFaction::Empire,
            integrator,
            world,
            movement,
            troop_transport,
            actions,
        )
    }

    /// `faction`'s AI applies `actions` in order, each recorded.
    fn apply_orders_of(
        faction: AiFaction,
        integrator: &mut PerceptionIntegrator,
        world: &mut GameWorld,
        movement: &mut MovementState,
        troop_transport: &mut TroopTransportState,
        actions: &[AIAction],
    ) -> Vec<AiOutcome> {
        let mut ai = AIState::new(faction);
        let mut missions = MissionState::new();
        let mut manufacturing = ManufacturingState::new();
        let mut stockpiles = StockpileState::new();
        let origin = crate::telemetry::AiOrderOrigin {
            faction: if faction == AiFaction::Alliance {
                "Alliance"
            } else {
                "Empire"
            },
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
                    &rebellion_core::delivery::DeliveryState::default(),
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

    // The AI's Build is the player's order 0x211 (FUN_004cba90): its
    // execute, FUN_00529ab0, runs FUN_0052a2c0 (+0xac(0x16) on every product
    // the manager lists) before FUN_0052a140 creates the new units, so a
    // second Build replaces the first and its unfinished ship is destroyed;
    // the new ship takes its fleet (Wine, the ds-b save, day 151: a Build at
    // Coruscant put `0x18000e4f` in Fleet 6, whose only ship it replaced).
    // The AI issues 0x211 only for a producer that is a manager (family
    // 0xa2-0xa3, FUN_004cba90.c:26-35), so the system holds a shipyard of
    // the side.
    #[test]
    fn an_ai_build_replaces_what_its_area_was_building_and_destroys_the_replaced_ship_for_either_side(
    ) {
        for faction in [AiFaction::Alliance, AiFaction::Empire] {
            let is_alliance = faction == AiFaction::Alliance;
            let mut world = GameWorld::default();
            let yard = add_system(&mut world, "Yard");
            let shipyard = world.manufacturing_facilities.insert(
                rebellion_core::world::ManufacturingFacilityInstance {
                    class_dat_id: DatId::new(0x2800_0001),
                    side: Faction::of_alliance(is_alliance),
                    is_shipyard: true,
                },
            );
            world.systems[yard].manufacturing_facilities.push(shipyard);
            let class = |world: &mut GameWorld| {
                world.capital_ship_classes.insert(CapitalShipClass {
                    is_alliance,
                    is_empire: !is_alliance,
                    hull: 100,
                    refined_material_cost: 5,
                    ..CapitalShipClass::default()
                })
            };
            let (first, second) = (class(&mut world), class(&mut world));
            let mut ai = AIState::new(faction);
            let mut manufacturing = ManufacturingState::new();
            let mut order = |world: &mut GameWorld, class, count| {
                apply_ai_order(
                    &AIAction::EnqueueProduction {
                        system: yard,
                        kind: BuildableKind::CapitalShip(class),
                        work: 5,
                        count,
                        destination: None,
                    },
                    &mut ai,
                    &mut MissionState::new(),
                    &mut manufacturing,
                    &rebellion_core::delivery::DeliveryState::default(),
                    &mut MovementState::new(),
                    &mut TroopTransportState::default(),
                    &mut StockpileState::new(),
                    world,
                    &UprisingState::default(),
                    5,
                )
            };

            assert_eq!(order(&mut world, first, 2), AiOutcome::applied(None));
            let replaced = world.systems[yard].fleets[0];
            assert_eq!(order(&mut world, second, 1), AiOutcome::applied(None));

            let queued: Vec<BuildableKind> = manufacturing
                .queue(yard, ProductionArea::Shipyard)
                .unwrap()
                .items()
                .iter()
                .map(|item| item.kind)
                .collect();
            assert_eq!(queued, [BuildableKind::CapitalShip(second)], "{faction:?}");
            assert_eq!(world.systems[yard].fleets, [replaced], "{faction:?}");
            let classes: Vec<_> = world
                .fleets
                .values()
                .flat_map(|fleet| &fleet.capital_ships)
                .map(|ship| ship.class)
                .collect();
            assert_eq!(classes, [second], "{faction:?}");
        }
    }

    // The AI's Build goes through the player's validator (FUN_005385f0),
    // whose maintenance test reads the ordering side's Maintenance Monitor
    // number (FUN_00422620: side +0x58 − +0x74), not the other side's.
    #[test]
    fn an_ai_build_its_own_side_cannot_maintain_is_refused_for_either_side() {
        for faction in [AiFaction::Alliance, AiFaction::Empire] {
            let is_alliance = faction == AiFaction::Alliance;
            let mut world = GameWorld::default();
            let yard = add_system(&mut world, "Yard");
            // A mine matched by a refinery: 50 maintenance for the other side.
            for raw in [0x2c00_0001, 0x2d00_0002] {
                world
                    .production_facilities
                    .insert(ProductionFacilityInstance {
                        class_dat_id: DatId::new(raw),
                        side: Faction::of_alliance(!is_alliance),
                        is_mine: raw == 0x2c00_0001,
                    });
            }
            let class = world.capital_ship_classes.insert(CapitalShipClass {
                is_alliance,
                is_empire: !is_alliance,
                hull: 100,
                refined_material_cost: 5,
                maintenance_cost: 1,
                ..CapitalShipClass::default()
            });
            let mut manufacturing = ManufacturingState::new();

            let outcome = apply_ai_order(
                &AIAction::EnqueueProduction {
                    system: yard,
                    kind: BuildableKind::CapitalShip(class),
                    work: 5,
                    count: 1,
                    destination: None,
                },
                &mut AIState::new(faction),
                &mut MissionState::new(),
                &mut manufacturing,
                &rebellion_core::delivery::DeliveryState::default(),
                &mut MovementState::new(),
                &mut TroopTransportState::default(),
                &mut StockpileState::new(),
                &mut world,
                &UprisingState::default(),
                5,
            );

            assert_eq!(
                outcome,
                AiOutcome::Refused(AiRefusal::Build(
                    rebellion_core::build_selection::OrderRefusal::Maintenance
                )),
                "{faction:?}"
            );
            assert!(world.fleets.is_empty(), "{faction:?}");
        }
    }

    /// One AI order of `faction` applied to `world` alone.
    fn ai_alone(world: &mut GameWorld, faction: AiFaction, action: &AIAction) -> AiOutcome {
        apply_ai_order(
            action,
            &mut AIState::new(faction),
            &mut MissionState::new(),
            &mut ManufacturingState::new(),
            &rebellion_core::delivery::DeliveryState::default(),
            &mut MovementState::new(),
            &mut TroopTransportState::default(),
            &mut StockpileState::new(),
            world,
            &UprisingState::default(),
            5,
        )
    }

    /// A fleet of `faction`'s side at `at` with one tagged ship.
    fn tagged_fleet(
        world: &mut GameWorld,
        at: SystemKey,
        faction: AiFaction,
        tag: u32,
    ) -> FleetKey {
        let is_alliance = faction == AiFaction::Alliance;
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            is_alliance,
            is_empire: !is_alliance,
            hull: 100,
            ..CapitalShipClass::default()
        });
        let mut ship = ShipInstance::new(class, 100, is_alliance);
        ship.tag = tag;
        let fleet = world.fleets.insert(Fleet {
            location: at,
            is_alliance,
            capital_ships: vec![ship],
            ..Fleet::default()
        });
        world.systems[at].fleets.push(fleet);
        fleet
    }

    // The Command submenu's object check refuses another side's character
    // (FUN_004f9860, side +0x24 bits 6..7): the AI's GiveCommand is checked
    // for its own side.
    #[test]
    fn an_ai_gives_a_command_to_its_own_sides_character_only_for_either_side() {
        for faction in [AiFaction::Alliance, AiFaction::Empire] {
            let mut world = GameWorld::default();
            let at = add_system(&mut world, "Post");
            let mut officer = |is_alliance: bool| {
                world.characters.insert(rebellion_core::world::Character {
                    is_alliance,
                    is_empire: !is_alliance,
                    recruited: true,
                    can_be_commander: true,
                    current_system: Some(at),
                    ..Default::default()
                })
            };
            let ours = officer(faction == AiFaction::Alliance);
            let theirs = officer(faction != AiFaction::Alliance);
            let give = |character| AIAction::GiveCommand {
                character,
                kind: rebellion_core::officer_rank::CommandKind::Commander,
            };

            assert_eq!(
                ai_alone(&mut world, faction, &give(ours)),
                AiOutcome::applied(None),
                "{faction:?}"
            );
            assert_eq!(
                ai_alone(&mut world, faction, &give(theirs)),
                AiOutcome::Refused(AiRefusal::Command(
                    rebellion_core::officer_rank::CommandRefusal::OtherSide
                )),
                "{faction:?}"
            );
        }
    }

    // A Move onto a fleet checks the order's side (FUN_005202d0,
    // FUN_00553b80) and refuses another side's destination fleet
    // (FUN_00553410): the AI's JoinFleet is checked for its own side.
    #[test]
    fn an_ai_joins_its_own_fleet_to_its_own_sides_fleet_only_for_either_side() {
        for faction in [AiFaction::Alliance, AiFaction::Empire] {
            let other = if faction == AiFaction::Alliance {
                AiFaction::Empire
            } else {
                AiFaction::Alliance
            };
            let mut world = GameWorld::default();
            let at = add_system(&mut world, "Muster");
            let mover = tagged_fleet(&mut world, at, faction, 1);
            let target = tagged_fleet(&mut world, at, faction, 2);
            let theirs = tagged_fleet(&mut world, at, other, 3);
            let join = |target| AIAction::JoinFleet {
                fleet: mover,
                ships: vec![1],
                target,
            };

            assert_eq!(
                ai_alone(&mut world, faction, &join(theirs)),
                AiOutcome::Refused(AiRefusal::Fleet(
                    rebellion_core::fleet_join::FleetOrderError::OtherSideDestination
                )),
                "{faction:?}"
            );
            assert_eq!(
                ai_alone(&mut world, faction, &join(target)),
                AiOutcome::applied(None),
                "{faction:?}"
            );
            assert_eq!(world.fleets[target].capital_ships.len(), 2, "{faction:?}");
        }
    }

    // Create Fleet (0x270) routes the team for its own side (FUN_00553b80):
    // the AI's CreateFleet moves only its own side's ships.
    #[test]
    fn an_ai_creates_a_fleet_from_its_own_sides_ships_only_for_either_side() {
        for faction in [AiFaction::Alliance, AiFaction::Empire] {
            let other = if faction == AiFaction::Alliance {
                AiFaction::Empire
            } else {
                AiFaction::Alliance
            };
            let mut world = GameWorld::default();
            let at = add_system(&mut world, "Muster");
            let ours = tagged_fleet(&mut world, at, faction, 1);
            let theirs = tagged_fleet(&mut world, at, other, 2);
            let create = |fleet, tag| AIAction::CreateFleet {
                fleet,
                ships: vec![tag],
                to_system: None,
            };

            assert_eq!(
                ai_alone(&mut world, faction, &create(theirs, 2)),
                AiOutcome::Refused(AiRefusal::Fleet(
                    rebellion_core::fleet_join::FleetOrderError::WrongFaction
                )),
                "{faction:?}"
            );
            assert_eq!(
                ai_alone(&mut world, faction, &create(ours, 1)),
                AiOutcome::applied(None),
                "{faction:?}"
            );
        }
    }

    // Every order's object check refuses another side's object
    // (FUN_004f9860): the AI's regiment and squadron moves and its Scrap
    // act only on its own side's.
    #[test]
    fn an_ai_moves_and_scraps_only_its_own_sides_regiments_and_squadrons_for_either_side() {
        use rebellion_core::troop_transport::{RegimentTarget, SquadronSource, TroopTransportError};
        for faction in [AiFaction::Alliance, AiFaction::Empire] {
            let other = if faction == AiFaction::Alliance {
                AiFaction::Empire
            } else {
                AiFaction::Alliance
            };
            let theirs = other == AiFaction::Alliance;
            let mut world = GameWorld::default();
            let at = add_system(&mut world, "Muster");
            let to = add_system(&mut world, "Target");
            for system in [at, to] {
                world.systems[system].control = ControlKind::Controlled(Faction::of_alliance(theirs));
            }
            let troop = world.troops.insert(rebellion_core::world::TroopUnit {
                class_dat_id: DatId::new(0x1000_0001),
                is_alliance: theirs,
            });
            world.systems[at].ground_units.push(troop);
            let fighter = world
                .fighter_classes
                .insert(rebellion_core::world::FighterClass::default());
            world.systems[at].add_squadrons(fighter, 1, theirs);
            let carrier = tagged_fleet(&mut world, at, other, 1);
            let class = world.fleets[carrier].capital_ships[0].class;
            world.capital_ship_classes[class].fighter_capacity = 4;
            let wrong = AiOutcome::Refused(AiRefusal::Troops(TroopTransportError::WrongFaction));

            let regiments = AIAction::MoveRegiments {
                troops: vec![troop],
                to: RegimentTarget::System(to),
            };
            assert_eq!(ai_alone(&mut world, faction, &regiments), wrong, "{faction:?}");
            assert!(world.systems[at].ground_units.contains(&troop), "{faction:?}");
            let squadrons = AIAction::MoveSquadrons {
                squadrons: vec![SquadronSource::System {
                    system: at,
                    class: fighter,
                    is_alliance: theirs,
                }],
                fleet: carrier,
            };
            assert_eq!(ai_alone(&mut world, faction, &squadrons), wrong, "{faction:?}");
            assert_eq!(world.systems[at].squadron_count(theirs), 1, "{faction:?}");
            let scrap = AIAction::Scrap {
                targets: vec![rebellion_core::scrap::ScrapTarget::Troop(troop)],
            };
            assert_eq!(
                ai_alone(&mut world, faction, &scrap),
                AiOutcome::Refused(AiRefusal::ScrapOtherSide),
                "{faction:?}"
            );
            assert!(world.troops.contains_key(troop), "{faction:?}");
        }
    }

    // The Move command's object check refuses another side's fleet
    // (FUN_004f9860), and its validator a destination that does not resolve
    // (FUN_00578d00, 1/0x22): the AI's Move runs the player's validator, so
    // neither moves, nor is a fleet that changes container at once left
    // without a system.
    #[test]
    fn an_ai_moves_only_its_own_sides_fleet_to_a_system_that_exists_for_either_side() {
        use rebellion_core::fleet_join::FleetOrderError;
        use rebellion_core::movement::FleetDispatchError;
        for faction in [AiFaction::Alliance, AiFaction::Empire] {
            let other = if faction == AiFaction::Alliance {
                AiFaction::Empire
            } else {
                AiFaction::Alliance
            };
            let mut world = GameWorld::default();
            let at = add_system(&mut world, "Muster");
            let to = add_system(&mut world, "Target");
            let gone = add_system(&mut world, "Gone");
            world.systems.remove(gone);
            let ours = tagged_fleet(&mut world, at, faction, 1);
            let theirs = tagged_fleet(&mut world, at, other, 2);
            // Only an unfinished ship: the fleet changes container at once.
            let unfinished = tagged_fleet(&mut world, at, faction, 3);
            world.fleets[unfinished].capital_ships[0].completed = false;
            let order = |fleet, to_system| AIAction::MoveFleet {
                fleet,
                to_system,
                reason: rebellion_core::ai::FleetMoveReason::Attack,
                troops: Vec::new(),
            };
            let refusal = |error| AiOutcome::Refused(AiRefusal::Fleet(FleetOrderError::Dispatch(error)));

            assert_eq!(
                ai_alone(&mut world, faction, &order(theirs, to)),
                refusal(FleetDispatchError::WrongFaction),
                "{faction:?}"
            );
            assert_eq!(world.fleets[theirs].location, at, "{faction:?}");
            assert_eq!(
                ai_alone(&mut world, faction, &order(unfinished, gone)),
                refusal(FleetDispatchError::MissingDestination),
                "{faction:?}"
            );
            assert_eq!(world.fleets[unfinished].location, at, "{faction:?}");
            assert!(world.systems[at].fleets.contains(&unfinished), "{faction:?}");
            assert_eq!(
                ai_alone(&mut world, faction, &order(ours, to)),
                AiOutcome::applied(None),
                "{faction:?}"
            );
        }
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
            bombarding: false,
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
        ai_events_of(AiFaction::Empire, world, actions)
    }

    /// The records `faction`'s AI writes applying `actions`.
    fn ai_events_of(
        faction: AiFaction,
        world: &mut GameWorld,
        actions: &[AIAction],
    ) -> Vec<GameEventRecord> {
        let mut integrator = PerceptionIntegrator::new(5, 0);
        apply_orders_of(
            faction,
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

    /// A fleet at `system` of one ship (tag 1) carrying `squadrons`
    /// squadrons of a class that costs 10.
    fn laden_ship(
        world: &mut GameWorld,
        system: SystemKey,
        is_alliance: bool,
        squadrons: u32,
    ) -> rebellion_core::ids::FleetKey {
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            hull: 100,
            ..CapitalShipClass::default()
        });
        let fighters = world
            .fighter_classes
            .insert(rebellion_core::world::FighterClass {
                refined_material_cost: 10,
                ..Default::default()
            });
        let mut ship = ShipInstance::new(class, 100, is_alliance);
        ship.tag = 1;
        let fleet = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ship],
            fighters: if squadrons == 0 {
                vec![]
            } else {
                vec![rebellion_core::world::FighterEntry {
                    class: fighters,
                    count: squadrons,
                    carrier: 1,
                }]
            },
            characters: vec![],
            is_alliance,
            bombarding: false,
        });
        world.systems[system].fleets.push(fleet);
        fleet
    }

    // FUN_004fb520's child walk destroys the squadrons aboard a scrapped
    // ship with reason 0x14 and FUN_00530270 refunds each; the order's
    // record names what went down with the ship.
    #[test]
    fn an_ai_scrap_of_a_laden_ship_records_the_squadrons_lost_aboard() {
        for is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let system = add_system(&mut world, "Bortras");
            let fleet = laden_ship(&mut world, system, is_alliance, 2);
            // The ship's own side scraps it (FUN_004f9860).
            let side = if is_alliance {
                AiFaction::Alliance
            } else {
                AiFaction::Empire
            };

            let events = ai_events_of(
                side,
                &mut world,
                &[AIAction::Scrap {
                    targets: vec![rebellion_core::scrap::ScrapTarget::CapitalShip {
                        fleet,
                        index: 0,
                    }],
                }],
            );

            assert_eq!(events.len(), 2, "{is_alliance}");
            assert_eq!(events[0].event_type, EVT_AI_ACTION);
            assert_eq!(events[1].event_type, EVT_SCRAPPED_ABOARD);
            assert_eq!(events[1].system, SYS_ECONOMY);
            assert_eq!(events[1].details["squadrons"], 2, "{is_alliance}");
            assert_eq!(events[1].details["regiments"], 0, "{is_alliance}");
        }
    }

    #[test]
    fn an_ai_scrap_of_a_ship_carrying_regiments_records_them_lost() {
        // FUN_004f8710 destroys a regiment aboard with the ship's reason 0x14.
        for is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let system = add_system(&mut world, "Bortras");
            let fleet = laden_ship(&mut world, system, is_alliance, 0);
            let mut transport = TroopTransportState::default();
            let troops = [0, 1].map(|_| {
                let troop = world.troops.insert(TroopUnit {
                    class_dat_id: DatId::new(0x1000_0001),
                    is_alliance,
                });
                transport.board_product(fleet, troop, 1);
                troop
            });
            let mut integrator = PerceptionIntegrator::new(5, 0);
            // The ship's own side scraps it (FUN_004f9860).
            let side = if is_alliance {
                AiFaction::Alliance
            } else {
                AiFaction::Empire
            };

            apply_orders_of(
                side,
                &mut integrator,
                &mut world,
                &mut MovementState::new(),
                &mut transport,
                &[AIAction::Scrap {
                    targets: vec![rebellion_core::scrap::ScrapTarget::CapitalShip {
                        fleet,
                        index: 0,
                    }],
                }],
            );

            let events = integrator.finish();
            assert_eq!(events[1].event_type, EVT_SCRAPPED_ABOARD, "{is_alliance}");
            assert_eq!(events[1].details["regiments"], 2, "{is_alliance}");
            assert_eq!(events[1].details["squadrons"], 0, "{is_alliance}");
            assert!(troops.iter().all(|&troop| !world.troops.contains_key(troop)));
        }
    }

    #[test]
    fn an_ai_scrap_of_an_empty_ship_records_nothing_aboard() {
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Bortras");
        let fleet = laden_ship(&mut world, system, false, 0);

        let events = empire_ai_events(
            &mut world,
            &[AIAction::Scrap {
                targets: vec![rebellion_core::scrap::ScrapTarget::CapitalShip {
                    fleet,
                    index: 0,
                }],
            }],
        );

        assert_eq!(
            events.iter().map(|event| event.event_type).collect::<Vec<_>>(),
            [EVT_AI_ACTION]
        );
    }

    #[test]
    fn an_overdraft_scrap_records_each_ships_losses_aboard() {
        for side in [
            rebellion_core::dat::Faction::Alliance,
            rebellion_core::dat::Faction::Empire,
        ] {
            let mut world = GameWorld::default();
            let system = add_system(&mut world, "Bortras");
            let fleet = laden_ship(
                &mut world,
                system,
                side == rebellion_core::dat::Faction::Alliance,
                3,
            );
            let report = rebellion_core::scrap::scrap_all_reporting(
                &mut world,
                &mut StockpileState::new(),
                &mut TroopTransportState::default(),
                [rebellion_core::scrap::ScrapTarget::CapitalShip { fleet, index: 0 }],
            );
            let mut integrator = PerceptionIntegrator::new(5, 0);

            integrator.emit_scrapped_aboard(&world, &report.losses);

            let events = integrator.finish();
            assert_eq!(events.len(), 1, "{side:?}");
            assert_eq!(events[0].event_type, EVT_SCRAPPED_ABOARD);
            assert_eq!(
                events[0].details["faction"],
                serde_json::json!(format!("{side:?}"))
            );
            assert_eq!(events[0].details["system"], "Bortras");
            assert_eq!(events[0].details["squadrons"], 3);
        }
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
            bombarding: false,
        });
        world.systems[origin].fleets.push(fleet);
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: false,
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

    // FUN_00555920.c:31-33 and FUN_00556390.c:12-14: a fleet with no ship
    // completed and not on its own leg is not mobile (FUN_004fda10) and
    // changes container at once. The route-1 Empire save, its day-123
    // Death Star played on: Fleet 6, holding only that unfinished Death
    // Star, moved from Coruscant to Averam without going en route.
    #[test]
    fn an_ai_move_of_a_fleet_with_no_usable_ship_changes_system_at_once_for_either_side() {
        for (is_alliance, faction) in [
            (true, rebellion_core::ai::AiFaction::Alliance),
            (false, rebellion_core::ai::AiFaction::Empire),
        ] {
            let mut world = GameWorld::default();
            let origin = add_system(&mut world, "Origin");
            let destination = add_system(&mut world, "Destination");
            let class = world.capital_ship_classes.insert(CapitalShipClass {
                is_alliance,
                is_empire: !is_alliance,
                hull: 100,
                hyperdrive: 80,
                ..CapitalShipClass::default()
            });
            let fleet = world.fleets.insert(Fleet {
                location: origin,
                capital_ships: vec![ShipInstance::ordered(class, 100, is_alliance)],
                is_alliance,
                ..Fleet::default()
            });
            world.systems[origin].fleets.push(fleet);
            let mut movement = MovementState::new();

            let outcome = apply_ai_order(
                &AIAction::MoveFleet {
                    fleet,
                    to_system: destination,
                    reason: FleetMoveReason::Attack,
                    troops: vec![],
                },
                &mut AIState::new(faction),
                &mut MissionState::new(),
                &mut ManufacturingState::new(),
                &rebellion_core::delivery::DeliveryState::default(),
                &mut movement,
                &mut TroopTransportState::default(),
                &mut StockpileState::new(),
                &mut world,
                &UprisingState::default(),
                5,
            );

            assert!(matches!(outcome, AiOutcome::Applied { .. }), "{is_alliance}: {outcome:?}");
            assert!(!movement.is_in_transit(fleet), "{is_alliance}");
            assert_eq!(world.fleets[fleet].location, destination, "{is_alliance}");
            assert!(world.systems[destination].fleets.contains(&fleet), "{is_alliance}");
            assert!(!world.systems[origin].fleets.contains(&fleet), "{is_alliance}");
        }
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
            bombarding: false,
        });
        world.systems[origin].fleets.push(fleet);
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: false,
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
            &rebellion_core::delivery::DeliveryState::default(),
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

    /// One unit of `class` ordered at `yard` for `destination`, seated at
    /// the order (`manufacturing::seat_ordered_ships`), then finished on day
    /// `tick` (`FUN_0052bee0`, `manufacturing::finish_ships`).
    fn order_and_finish(
        world: &mut GameWorld,
        yard: SystemKey,
        destination: rebellion_core::manufacturing::Destination,
        class: rebellion_core::ids::CapitalShipKey,
        tick: u64,
    ) {
        let mut mfg = ManufacturingState::new();
        mfg.enqueue(
            yard,
            QueueItem::new(BuildableKind::CapitalShip(class), 0).delivered_to(destination),
        );
        rebellion_core::manufacturing::seat_ordered_ships(world, &mut mfg);
        let advance = rebellion_core::manufacturing::ManufacturingSystem::advance_tracked(
            &mut mfg,
            &[rebellion_core::tick::TickEvent { tick }],
            &std::collections::HashSet::new(),
        );
        rebellion_core::manufacturing::finish_ships(
            world,
            &MovementState::new(),
            &advance.finished_ships,
        );
        for completion in &advance.completions {
            apply_build_completion_inner(completion, world);
        }
    }

    #[test]
    fn a_death_star_finished_in_place_marks_its_sector_at_once_and_one_on_its_leg_on_arrival() {
        // Completion sets the product's usable bit (FUN_004f7b80), whose
        // event 0x301 reruns FUN_005178b0 for a family 0x18-0x1b object
        // (FUN_00514510): bit 11 holds before any later step reads it. One
        // finished for another system is en route on its own leg
        // (FUN_005150c0.c:90-102), so it is usable only on arrival.
        let mut world = GameWorld::default();
        let yard = add_system(&mut world, "Yard");
        let far = add_system(&mut world, "Far");
        world.systems[far].x = 100;
        let sector = world.systems[yard].sector;
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            family: 0x18,
            is_empire: true,
            hull: 100,
            hyperdrive: 100,
            ..CapitalShipClass::default()
        });

        order_and_finish(
            &mut world,
            yard,
            rebellion_core::manufacturing::Destination::System(yard),
            class,
            1,
        );
        assert!(world.death_star_sectors.contains(&sector));

        world.fleets.clear();
        world.systems[yard].fleets.clear();
        world.death_star_sectors.clear();
        order_and_finish(
            &mut world,
            yard,
            rebellion_core::manufacturing::Destination::System(far),
            class,
            1,
        );
        assert!(world.death_star_sectors.is_empty());
    }

    // A fleet-bound squadron or regiment was created aboard the ship its
    // Build chose (FUN_0052a140 -> FUN_004f7d50) and its completion only
    // sets the completed bit (FUN_0052bee0): it is aboard that ship, never
    // on the fleet's planet.
    #[test]
    fn a_squadron_or_regiment_delivered_to_a_fleet_is_aboard_the_ship_its_order_chose() {
        for is_alliance in [false, true] {
            let mut world = GameWorld::default();
            let system = add_system(&mut world, "Yard");
            let class = world.capital_ship_classes.insert(CapitalShipClass {
                is_alliance,
                hull: 100,
                fighter_capacity: 1,
                troop_capacity: 1,
                ..Default::default()
            });
            let fleet = world.fleets.insert(Fleet {
                location: system,
                is_alliance,
                capital_ships: vec![
                    ShipInstance::new(class, 100, is_alliance),
                    ShipInstance::new(class, 100, is_alliance),
                ],
                ..Fleet::default()
            });
            world.systems[system].fleets.push(fleet);
            let tag = rebellion_core::carriage::tag_ship(&mut world, fleet, 1).unwrap();
            let fighter = world.fighter_classes.insert(rebellion_core::world::FighterClass {
                is_alliance,
                ..Default::default()
            });
            let regiment = rebellion_core::ids::DatId::new(0x1000_0001);
            world.buildable_classes.insert(
                regiment,
                rebellion_core::world::BuildableClass {
                    is_alliance,
                    is_empire: !is_alliance,
                    ..Default::default()
                },
            );
            let mut transport = TroopTransportState::default();
            let boarded: Vec<_> = [
                BuildableKind::Fighter(fighter),
                BuildableKind::Troop(regiment),
            ]
            .into_iter()
            .map(|kind| rebellion_core::delivery::Boarded {
                fleet,
                carrier: tag,
                completion: CompletionEvent {
                    system,
                    tick: 1,
                    kind,
                },
            })
            .collect();
            let mut integrator = PerceptionIntegrator::new(1, 0);
            integrator.apply_boarded(
                &mut world,
                &mut ManufacturingState::new(),
                &mut transport,
                &boarded,
            );
            let events: Vec<_> = integrator
                .finish()
                .iter()
                .map(|event| event.event_type)
                .collect();
            assert_eq!(
                events,
                [
                    EVT_BUILD_COMPLETE,
                    EVT_UNITS_DEPLOYED,
                    EVT_BUILD_COMPLETE,
                    EVT_UNITS_DEPLOYED
                ],
                "alliance {is_alliance}"
            );
            let squadrons: Vec<_> = world.fleets[fleet]
                .fighters
                .iter()
                .map(|entry| (entry.class, entry.count, entry.carrier))
                .collect();
            assert_eq!(squadrons, [(fighter, 1, tag)], "alliance {is_alliance}");
            let carried: Vec<u32> = transport
                .cargo(fleet)
                .iter()
                .map(|&troop| transport.carrier(troop))
                .collect();
            assert_eq!(carried, [tag], "alliance {is_alliance}");
            assert!(world.systems[system].ground_units.is_empty(), "alliance {is_alliance}");
            assert!(world.systems[system].fighters.is_empty(), "alliance {is_alliance}");
        }
    }

    // Each completed product bound elsewhere leaves its yard (FUN_0052bee0);
    // a capital ship is already in its destination fleet and takes its own
    // leg (FUN_005150c0.c:90-102), so only the rest travel in
    // DeliveryState.
    #[test]
    fn only_the_products_that_are_not_capital_ships_travel_as_deliveries_for_either_side() {
        for is_alliance in [false, true] {
            let (mut world, yard, destination, class) = far_yard(is_alliance);
            let fighter = world
                .fighter_classes
                .insert(rebellion_core::world::FighterClass {
                    is_alliance,
                    ..Default::default()
                });
            let leaving = |kind| rebellion_core::manufacturing::Departure {
                origin: yard,
                destination: rebellion_core::manufacturing::Destination::System(destination),
                tick: 5,
                kind,
                carrier: 0,
            };
            let squadron = leaving(BuildableKind::Fighter(fighter));
            let travelling = travelling_products(&[
                leaving(BuildableKind::CapitalShip(class)),
                squadron.clone(),
            ]);
            assert_eq!(travelling, [squadron], "{is_alliance}");
        }
    }

    /// A yard system and a destination 100 units away, and a capital ship
    /// class of `side` with maintenance 40 and hyperdrive 100. port: test
    /// fixture values, not rules.
    fn far_yard(
        is_alliance: bool,
    ) -> (
        GameWorld,
        SystemKey,
        SystemKey,
        rebellion_core::ids::CapitalShipKey,
    ) {
        let mut world = GameWorld::default();
        let yard = add_system(&mut world, "Yard");
        let destination = add_system(&mut world, "Far");
        world.systems[destination].x = 100;
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            is_alliance,
            is_empire: !is_alliance,
            hull: 100,
            maintenance_cost: 40,
            hyperdrive: 100,
            ..Default::default()
        });
        (world, yard, destination, class)
    }

    // The ship sits in its destination's spare fleet from its order
    // (FUN_0052a140 -> FUN_004f7d50; death-star-product.md section 1).
    // FUN_0052bee0 sets en route active on a product whose location is not
    // its yard's, and deploy event 0x308 starts its own leg from the yard
    // (+0x38) to that fleet (FUN_005150c0.c:90-102 -> FUN_00556430). FUN_00433780 adds its +0x70 to fleet
    // +0x88 whatever its en-route bits, so c+0x24c counts it on the way.
    #[test]
    fn a_capital_ship_finished_for_another_system_travels_in_its_destination_spare_fleet_for_either_side(
    ) {
        for is_alliance in [false, true] {
            let (mut world, yard, destination, class) = far_yard(is_alliance);
            let kind = BuildableKind::CapitalShip(class);
            order_and_finish(
                &mut world,
                yard,
                rebellion_core::manufacturing::Destination::System(destination),
                class,
                5,
            );
            let ticks = rebellion_core::delivery::transit_days(&world, kind, yard, destination);
            assert!(ticks > 1, "{is_alliance}");
            let fleets: Vec<_> = world.systems[destination]
                .fleets
                .iter()
                .map(|&fleet| &world.fleets[fleet])
                .collect();
            assert_eq!(fleets.len(), 1, "{is_alliance}");
            assert_eq!(fleets[0].is_alliance, is_alliance);
            assert_eq!(
                fleets[0]
                    .capital_ships
                    .iter()
                    .map(|ship| ship.transit)
                    .collect::<Vec<_>>(),
                [Some(rebellion_core::world::ShipTransit {
                    from: yard,
                    to: destination,
                    arrival_tick: 5 + u64::from(ticks),
                })],
                "{is_alliance}"
            );
            let side = rebellion_core::dat::Faction::of_alliance(is_alliance);
            let census = rebellion_core::ai_production::Census::take(
                &world,
                &ManufacturingState::new(),
                &TroopTransportState::new(),
                side,
            );
            assert_eq!(census.capital_ships, 40, "{is_alliance}");
        }
    }

    // Galaxy event 0x387 on the leg's day -> FUN_00515a50: the ship's place
    // is its leg's end, so FUN_004f7640(ship, 0) clears en route active
    // (FUN_00515a50.c:63); before that day it stays on its way. When its
    // fleet has moved on, FUN_00556620 starts the next leg from the leg's
    // end to the fleet's system instead (FUN_00556620.c:27-31).
    #[test]
    fn a_travelling_capital_ship_arrives_on_its_day_or_follows_its_moved_fleet_for_either_side() {
        use rebellion_core::tick::TickEvent;
        for is_alliance in [false, true] {
            let (mut world, yard, destination, class) = far_yard(is_alliance);
            let kind = BuildableKind::CapitalShip(class);
            order_and_finish(
                &mut world,
                yard,
                rebellion_core::manufacturing::Destination::System(destination),
                class,
                5,
            );
            let fleet = world.systems[destination].fleets[0];
            let due = 5 + u64::from(rebellion_core::delivery::transit_days(
                &world,
                kind,
                yard,
                destination,
            ));
            let movement = MovementState::new();
            let legs = |world: &mut GameWorld, tick| {
                rebellion_core::delivery::advance_ship_legs(world, &movement, &[TickEvent { tick }])
            };
            assert!(legs(&mut world, due - 1).is_empty(), "{is_alliance}");
            assert!(world.fleets[fleet].capital_ships[0].transit.is_some());
            let mut moved = world.clone();
            assert_eq!(
                legs(&mut world, due),
                [rebellion_core::delivery::ShipArrival {
                    fleet,
                    class,
                    system: destination,
                    tick: due,
                }],
                "{is_alliance}"
            );
            assert_eq!(world.fleets[fleet].capital_ships[0].transit, None);

            // The fleet went back to the yard meanwhile.
            moved.fleets[fleet].location = yard;
            assert!(legs(&mut moved, due).is_empty(), "{is_alliance}");
            let back = u64::from(rebellion_core::delivery::transit_days(
                &moved,
                kind,
                destination,
                yard,
            ));
            assert_eq!(
                moved.fleets[fleet].capital_ships[0].transit,
                Some(rebellion_core::world::ShipTransit {
                    from: destination,
                    to: yard,
                    arrival_tick: due + back,
                }),
                "{is_alliance}"
            );
        }
    }

    /// An Imperial fleet of one ship with bombard strength 5 at an
    /// Alliance-held system holding one regiment of value 0.
    fn strike_world() -> (
        GameWorld,
        SystemKey,
        FleetKey,
        rebellion_core::ids::TroopKey,
    ) {
        let mut world = GameWorld::default();
        // GNPRTB 0x601-0x603 as shipped: 40, 1, 10.
        let entry = |id: u32, value: i32| rebellion_core::world::GnprtbEntry {
            parameter_id: id,
            development: value,
            alliance_sp_easy: value,
            alliance_sp_medium: value,
            alliance_sp_hard: value,
            empire_sp_easy: value,
            empire_sp_medium: value,
            empire_sp_hard: value,
            multiplayer: value,
        };
        world.gnprtb = rebellion_core::world::GnprtbParams::new(vec![
            entry(0x601, 40),
            entry(0x602, 1),
            entry(0x603, 10),
        ]);
        let system = add_system(&mut world, "Target");
        world.systems[system].control = ControlKind::Controlled(Faction::Alliance);
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            family: 0x14,
            hull: 100,
            bombardment_modifier: 5,
            ..Default::default()
        });
        let fleet = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ShipInstance::new(class, 100, false)],
            ..Fleet::default()
        });
        world.systems[system].fleets.push(fleet);
        let troop = world.troops.insert(rebellion_core::world::TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: true,
        });
        world.systems[system].ground_units.push(troop);
        (world, system, fleet, troop)
    }

    /// The states a strike reaches, owned for a test.
    struct StrikeFixture {
        movement: MovementState,
        manufacturing: ManufacturingState,
        transport: TroopTransportState,
        deliveries: rebellion_core::delivery::DeliveryState,
        missions: MissionState,
        uprisings: UprisingState,
        blockade: rebellion_core::blockade::BlockadeState,
    }

    impl StrikeFixture {
        fn new() -> Self {
            Self {
                movement: MovementState::new(),
                manufacturing: ManufacturingState::new(),
                transport: TroopTransportState::default(),
                deliveries: rebellion_core::delivery::DeliveryState::new(),
                missions: MissionState::new(),
                uprisings: UprisingState::new(),
                blockade: rebellion_core::blockade::BlockadeState::new(),
            }
        }

        fn states(&mut self) -> StrikeStates<'_> {
            StrikeStates {
                movement: &mut self.movement,
                manufacturing: &mut self.manufacturing,
                transport: &mut self.transport,
                deliveries: &mut self.deliveries,
                missions: &mut self.missions,
                uprisings: &self.uprisings,
                blockade: &mut self.blockade,
                tick: 1,
                step: 0,
            }
        }
    }

    /// The side of `fleet`, the AI whose order names it.
    fn fleet_side(world: &GameWorld, fleet: FleetKey) -> Faction {
        Faction::of_alliance(world.fleets[fleet].is_alliance)
    }

    // The strike and assault orders run the object check of every order
    // (FUN_004f9860), which refuses another side's object: an AI that names
    // the other side's fleet strikes nothing and lands nothing.
    #[test]
    fn an_ai_strike_or_assault_by_the_other_sides_fleet_is_refused_for_either_side() {
        let wrong = AiOutcome::Refused(AiRefusal::Fleet(
            rebellion_core::fleet_join::FleetOrderError::WrongFaction,
        ));
        for flip in [false, true] {
            let (mut world, system, fleet, troop) = strike_world();
            if flip {
                world.fleets[fleet].is_alliance = !world.fleets[fleet].is_alliance;
            }
            let other = Faction::of_alliance(!world.fleets[fleet].is_alliance);
            let mut fixture = StrikeFixture::new();
            let (outcome, strike) = resolve_ai_strike(
                &AIAction::Bombard {
                    fleet,
                    system,
                    kind: rebellion_core::ai_regions::Strike::Military,
                },
                other,
                &mut world,
                &mut fixture.states(),
                &mut || 0.5,
            );
            assert_eq!((outcome, strike.report), (wrong, None), "{other:?}");
            assert!(world.troops.contains_key(troop), "{other:?}");
            let (outcome, _) = resolve_ai_assault(
                &AIAction::Assault { fleet, system },
                other,
                &mut world,
                &mut fixture.states(),
                &mut || 0.5,
            );
            assert_eq!(outcome, wrong, "{other:?}");
        }
    }

    #[test]
    fn an_ai_target_military_order_runs_the_players_bombardment() {
        // FUN_004b1070 issues 0x220; the order runs FUN_0058d800 as the
        // player's does, and the finalize clears the fleet's flag.
        let (mut world, system, fleet, troop) = strike_world();
        let mut fixture = StrikeFixture::new();
        let (outcome, strike) = resolve_ai_strike(
            &AIAction::Bombard {
                fleet,
                system,
                kind: rebellion_core::ai_regions::Strike::Military,
            },
            fleet_side(&world, fleet),
            &mut world,
            &mut fixture.states(),
            &mut || 0.5,
        );
        assert!(outcome.is_applied());
        let report = strike.report.expect("the run's report");
        assert_eq!(
            report.kind,
            Some(rebellion_core::bombardment::BombardKind::Military)
        );
        assert_eq!(report.hits, 5);
        assert_eq!(report.regiments, vec![(troop, Faction::Alliance)]);
        assert!(!world.troops.contains_key(troop));
        assert!(!world.fleets[fleet].bombarding);
    }

    #[test]
    fn an_ai_destroy_system_without_a_death_star_is_refused() {
        // FUN_00582440: 0x223 needs a family 0x18-0x1b ship (8/8).
        let (mut world, system, fleet, troop) = strike_world();
        let mut fixture = StrikeFixture::new();
        let (outcome, strike) = resolve_ai_strike(
            &AIAction::DestroySystem { fleet, system },
            fleet_side(&world, fleet),
            &mut world,
            &mut fixture.states(),
            &mut || 0.5,
        );
        assert_eq!(
            outcome,
            AiOutcome::Refused(AiRefusal::Strike(
                rebellion_core::bombardment::BombardRefusal::NoDeathStar
            ))
        );
        assert!(strike.report.is_none());
        assert!(world.troops.contains_key(troop));
    }

    #[test]
    fn a_fleet_in_transit_is_left_out_of_the_order() {
        // FUN_004ffa70 refuses an en-route fleet's command (1/0x11): the
        // order flags nothing and runs nothing.
        let (mut world, system, fleet, troop) = strike_world();
        let other = add_system(&mut world, "Elsewhere");
        let mut fixture = StrikeFixture::new();
        assert!(begin_fleet_transit(
            &mut fixture.movement,
            &mut world,
            fleet,
            other,
            5
        ));
        let runs = order_strike(
            &mut world,
            &[fleet],
            rebellion_core::bombardment::BombardKind::General,
            &mut fixture.states(),
            &mut || 0.5,
        )
        .expect("the gate passes");
        assert!(runs.is_empty());
        assert!(!world.fleets[fleet].bombarding);
        assert!(world.troops.contains_key(troop));
        let _ = system;
    }

    #[test]
    fn an_ai_strike_by_a_fleet_in_transit_is_refused() {
        // FUN_004fec00 refuses an en-route fleet (1/0x11): the order flags
        // nothing, so it failed, and the AI restarts the module.
        let (mut world, system, fleet, _) = strike_world();
        let other = add_system(&mut world, "Elsewhere");
        let mut fixture = StrikeFixture::new();
        assert!(begin_fleet_transit(
            &mut fixture.movement,
            &mut world,
            fleet,
            other,
            5
        ));
        let action = AIAction::Bombard {
            fleet,
            system,
            kind: rebellion_core::ai_regions::Strike::General,
        };
        let (outcome, effects) =
            resolve_ai_strike(
                &action,
                fleet_side(&world, fleet),
                &mut world,
                &mut fixture.states(),
                &mut || 0.5,
            );
        assert_eq!(
            outcome,
            AiOutcome::Refused(AiRefusal::Strike(
                rebellion_core::bombardment::BombardRefusal::FleetRefused
            ))
        );
        assert!(effects.report.is_none());
    }

    #[test]
    fn a_fleet_the_batteries_empty_is_removed_and_its_crew_taken() {
        // FUN_0058d540 destroys the last hull; the emptied fleet goes as a
        // battle's does (post_battle::destroy_fleet).
        let (mut world, system, fleet, _) = strike_world();
        let battery = DatId::new(0x2300_0001);
        world.buildable_classes.insert(
            battery,
            rebellion_core::world::BuildableClass {
                stats: rebellion_core::world::ClassStats {
                    attack_strength: 500,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        let lnr = world.defense_facilities.insert(DefenseFacilityInstance {
            class_dat_id: battery,
            side: Faction::Alliance,
        });
        world.systems[system].defense_facilities.push(lnr);
        let admiral = world.characters.insert(rebellion_core::world::Character {
            current_fleet: Some(fleet),
            current_system: Some(system),
            ..Default::default()
        });
        world.fleets[fleet].characters.push(admiral);
        let mut fixture = StrikeFixture::new();
        let runs = order_strike(
            &mut world,
            &[fleet],
            rebellion_core::bombardment::BombardKind::General,
            &mut fixture.states(),
            &mut || 0.5,
        )
        .expect("the gate passes");
        assert_eq!(
            runs[0].report.as_ref().map(|report| report.ships_destroyed),
            Some(1)
        );
        assert!(!world.fleets.contains_key(fleet));
        assert!(!world.systems[system].fleets.contains(&fleet));
        assert!(world.characters[admiral].is_captive);
        assert_eq!(world.characters[admiral].current_fleet, None);
    }

    /// [`strike_world`] with `bombarder` the fleet's side and the other
    /// holding the system and its regiment; the ship is a named record.
    ///
    /// port: test fixture values, not rules.
    fn named_strike_world(
        bombarder: Faction,
    ) -> (
        GameWorld,
        SystemKey,
        FleetKey,
        rebellion_core::ids::TroopKey,
    ) {
        let (mut world, system, fleet, troop) = strike_world();
        let holder = if bombarder == Faction::Empire {
            Faction::Alliance
        } else {
            Faction::Empire
        };
        world.systems[system].control = ControlKind::Controlled(holder);
        world.troops[troop].is_alliance = holder == Faction::Alliance;
        world.fleets[fleet].is_alliance = bombarder == Faction::Alliance;
        let class = world.fleets[fleet].capital_ships[0].class;
        world.capital_ship_classes[class].dat_id = DatId::new(0x1400_0040);
        world.capital_ship_classes[class].name = "Cruiser".into();
        for (raw, name) in [(0x1000_0001, "Regiment"), (0x2800_0001, "Shipyard")] {
            world.buildable_classes.insert(
                DatId::new(raw),
                rebellion_core::world::BuildableClass {
                    name: name.into(),
                    ..Default::default()
                },
            );
        }
        (world, system, fleet, troop)
    }

    fn names(units: &[rebellion_core::operation_report::ReportUnit]) -> Vec<&str> {
        units.iter().map(|unit| unit.name.as_str()).collect()
    }

    #[test]
    fn an_ordinary_run_reports_what_stands_after_it_and_what_it_destroyed() {
        // The finalize's flag clearing raises 0x182 and 0x14f with value 0,
        // which list the fleet's and the system's survivors after the run
        // (FUN_00496770, FUN_00496f50); each kill's 0x302 lists it as
        // destroyed at the system (FUN_00497380); the picture is 0x2b9b.
        for bombarder in [Faction::Empire, Faction::Alliance] {
            let (mut world, system, fleet, troop) = named_strike_world(bombarder);
            let holder = world.systems[system].control.faction().unwrap();
            let yard = world
                .manufacturing_facilities
                .insert(ManufacturingFacilityInstance {
                    class_dat_id: DatId::new(0x2800_0001),
                    side: holder,
                    ..Default::default()
                });
            world.systems[system].manufacturing_facilities.push(yard);
            let mut fixture = StrikeFixture::new();
            let runs = order_strike(
                &mut world,
                &[fleet],
                rebellion_core::bombardment::BombardKind::Military,
                &mut fixture.states(),
                &mut || 0.5,
            )
            .expect("the gate passes");
            assert!(!world.troops.contains_key(troop));
            let report = runs[0].operation.as_ref().expect("the Operation Report");
            assert_eq!(
                (
                    report.system,
                    report.assault,
                    report.holder,
                    report.fleet_owner
                ),
                (system, false, holder, bombarder)
            );
            assert_eq!(report.fleets, [fleet]);
            assert_eq!(names(&report.fleet_operational), ["Cruiser"]);
            assert!(report.fleet_destroyed.is_empty());
            assert_eq!(names(&report.system_operational), ["Shipyard"]);
            assert_eq!(names(&report.system_destroyed), ["Regiment"]);
            assert_eq!(report.picture(), 0x2b9b, "{bombarder:?}");
        }
    }

    #[test]
    fn a_fleet_the_batteries_destroy_reports_its_ship_and_the_regiments_aboard() {
        // FUN_00497380: objects aboard a ship or fleet go to +0x88; the
        // regiments lost with their transport go with it
        // (destroy_untransportable_cargo); the picture is 0x2b9c.
        for bombarder in [Faction::Empire, Faction::Alliance] {
            let (mut world, system, fleet, _) = named_strike_world(bombarder);
            let holder = world.systems[system].control.faction().unwrap();
            let class = world.fleets[fleet].capital_ships[0].class;
            world.capital_ship_classes[class].troop_capacity = 1;
            let aboard = world.troops.insert(rebellion_core::world::TroopUnit {
                class_dat_id: DatId::new(0x1000_0001),
                is_alliance: bombarder == Faction::Alliance,
            });
            world.systems[system].ground_units.push(aboard);
            let mut fixture = StrikeFixture::new();
            fixture
                .transport
                .embark(&mut world, &ProductRoom::default(), fleet, &[aboard])
                .expect("the ship carries one regiment");
            let battery = DatId::new(0x2300_0001);
            world.buildable_classes.insert(
                battery,
                rebellion_core::world::BuildableClass {
                    stats: rebellion_core::world::ClassStats {
                        attack_strength: 500,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
            let lnr = world.defense_facilities.insert(DefenseFacilityInstance {
                class_dat_id: battery,
                side: holder,
            });
            world.systems[system].defense_facilities.push(lnr);
            let runs = order_strike(
                &mut world,
                &[fleet],
                rebellion_core::bombardment::BombardKind::General,
                &mut fixture.states(),
                &mut || 0.5,
            )
            .expect("the gate passes");
            assert!(!world.troops.contains_key(aboard));
            let report = runs[0].operation.as_ref().expect("the Operation Report");
            assert!(report.fleets.is_empty(), "the emptied fleet is gone");
            assert!(report.fleet_operational.is_empty());
            assert_eq!(names(&report.fleet_destroyed), ["Cruiser", "Regiment"]);
            assert_eq!(report.picture(), 0x2b9c, "{bombarder:?}");
        }
    }

    #[test]
    fn a_regiment_lost_with_the_ship_the_batteries_destroy_feeds_the_bombarders_counter_b() {
        // The lost ship's hook +0x11c (FUN_00502840 -> FUN_004fb520) calls
        // each child's slot +0xb4; a regiment's (vtable 0x0065e438) is
        // FUN_004f8710, which destroys it with the ship's reason 0xf
        // (FUN_004f7ed0), and FUN_00508c60 adds its slot +0x1c0 (TROOPSD
        // uprising_defense) to B beside the ship's (FUN_00501ae0(.., 0xf)).
        for bombarder in [Faction::Empire, Faction::Alliance] {
            let (mut world, system, fleet, _) = named_strike_world(bombarder);
            let holder = world.systems[system].control.faction().unwrap();
            let class = world.fleets[fleet].capital_ships[0].class;
            world.capital_ship_classes[class].troop_capacity = 1;
            world.capital_ship_classes[class].uprising_defense = 4;
            let regiment = DatId::new(0x1000_0001);
            world
                .buildable_classes
                .get_mut(&regiment)
                .expect("the named Regiment class")
                .stats
                .uprising_defense = 6;
            let aboard = world.troops.insert(rebellion_core::world::TroopUnit {
                class_dat_id: regiment,
                is_alliance: bombarder == Faction::Alliance,
            });
            world.systems[system].ground_units.push(aboard);
            let mut fixture = StrikeFixture::new();
            fixture
                .transport
                .embark(&mut world, &ProductRoom::default(), fleet, &[aboard])
                .expect("the ship carries one regiment");
            let battery = DatId::new(0x2300_0001);
            world.buildable_classes.insert(
                battery,
                rebellion_core::world::BuildableClass {
                    stats: rebellion_core::world::ClassStats {
                        attack_strength: 500,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
            let lnr = world.defense_facilities.insert(DefenseFacilityInstance {
                class_dat_id: battery,
                side: holder,
            });
            world.systems[system].defense_facilities.push(lnr);
            order_strike(
                &mut world,
                &[fleet],
                rebellion_core::bombardment::BombardKind::General,
                &mut fixture.states(),
                &mut || 0.5,
            )
            .expect("the gate passes");
            assert!(!world.troops.contains_key(aboard), "{bombarder:?}");
            let losses = fixture.blockade.losses(system).expect("a loss record");
            let index = usize::from(bombarder == Faction::Empire);
            assert_eq!(losses.b[index], 4 + 6, "{bombarder:?}");
        }
    }

    #[test]
    fn destroy_system_files_no_operation_report() {
        // FUN_00488de0 maps a destroyed system's 0x14f to no kind.
        for side in [Faction::Empire, Faction::Alliance] {
            let (mut world, system, _, _) = strike_world();
            let death_star = death_star_fleet(&mut world, system, side);
            let runs = order_strike(
                &mut world,
                &[death_star],
                rebellion_core::bombardment::BombardKind::DestroySystem,
                &mut StrikeFixture::new().states(),
                &mut || 0.5,
            )
            .expect("the gate passes");
            assert_eq!(runs[0].destroyed(), Some(system));
            assert!(runs[0].operation.is_none(), "{side:?}");
        }
    }

    /// A Death Star fleet of `side` at `system`: one CAPSHPSD family-0x18
    /// hull with bombard strength (`FUN_00582440`).
    fn death_star_fleet(world: &mut GameWorld, system: SystemKey, side: Faction) -> FleetKey {
        let is_alliance = side == Faction::Alliance;
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(rebellion_core::world::DEATH_STAR_CLASS_ID),
            family: 0x18,
            is_alliance,
            is_empire: !is_alliance,
            hull: 25_000,
            bombardment_modifier: 15,
            ..Default::default()
        });
        let fleet = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ShipInstance::new(class, 25_000, is_alliance)],
            is_alliance,
            ..Fleet::default()
        });
        world.systems[system].fleets.push(fleet);
        fleet
    }

    /// Destroy System (`0x223`): the cascade leaves every fleet in orbit and
    /// kills the system's own regiment (`FUN_00524fb0`, `FUN_004f8710`);
    /// the destroyed system then refuses a second order (`1`/`0x22`,
    /// `FUN_00507f10`). A Death Star of either side runs it, since nothing
    /// tests the side (`death-star-strategic.md`, "A second superweapon").
    #[test]
    fn destroy_system_keeps_the_fleets_and_a_second_order_is_refused() {
        for side in [Faction::Empire, Faction::Alliance] {
            let (mut world, system, fleet, troop) = strike_world();
            let victim = if side == Faction::Empire {
                Faction::Alliance
            } else {
                Faction::Empire
            };
            world.systems[system].control = ControlKind::Controlled(victim);
            world.troops[troop].is_alliance = victim == Faction::Alliance;
            world.fleets[fleet].is_alliance = victim == Faction::Alliance;
            let death_star = death_star_fleet(&mut world, system, side);
            let mut fixture = StrikeFixture::new();
            let kind = rebellion_core::bombardment::BombardKind::DestroySystem;

            let runs = order_strike(
                &mut world,
                &[death_star],
                kind,
                &mut fixture.states(),
                &mut || 0.5,
            )
            .expect("the gate passes");

            assert_eq!(runs.len(), 1);
            assert_eq!(runs[0].destroyed(), Some(system));
            let cascade = runs[0].cascade.as_ref().expect("the cascade ran");
            assert_eq!(cascade.regiments, vec![(troop, victim)]);
            assert!(world.systems[system].is_destroyed);
            assert_eq!(world.systems[system].fleets, vec![fleet, death_star]);
            assert!(!world.troops.contains_key(troop));
            assert!(!world.fleets[death_star].bombarding);
            assert_eq!(
                order_strike(
                    &mut world,
                    &[death_star],
                    kind,
                    &mut fixture.states(),
                    &mut || 0.5
                ),
                Err(rebellion_core::bombardment::BombardRefusal::Destroyed)
            );
        }
    }

    /// Each adrift flag the cascade sets posts event `0x30e` (`FUN_00534790`
    /// → `FUN_00536460`), served in the order's frame step. With the battle
    /// bit clear the first one dispatches, so each side's character leaves on
    /// a hidden trip to its side's system the day of the kill. With fleets
    /// of both sides in orbit the bit stays set: the destroyed system's
    /// holder is neutral, side 3 (`FUN_0050b610`: bits 2-3 are 3 once
    /// unpopulated), which `FUN_0050b8e0` treats as held, so the sweep only
    /// gathers and the characters stay, still adrift
    /// (`death-star-strategic.md`, "When a Destroy System sweeps its adrift
    /// characters").
    #[test]
    fn destroy_system_sends_adrift_characters_away_the_day_of_the_kill_unless_both_sides_orbit() {
        for both_sides_in_orbit in [false, true] {
            let (mut world, system, fleet, _) = strike_world();
            world.fleets[fleet].is_alliance = both_sides_in_orbit;
            let death_star = death_star_fleet(&mut world, system, Faction::Empire);
            let mut refuges = Vec::new();
            for side in [Faction::Alliance, Faction::Empire] {
                let refuge = add_system(&mut world, "Refuge");
                world.systems[refuge].control = ControlKind::Controlled(side);
                let character = world.characters.insert(rebellion_core::world::Character {
                    is_alliance: side == Faction::Alliance,
                    is_empire: side == Faction::Empire,
                    current_system: Some(system),
                    ..Default::default()
                });
                refuges.push((character, refuge));
            }
            let mut fixture = StrikeFixture::new();

            order_strike(
                &mut world,
                &[death_star],
                rebellion_core::bombardment::BombardKind::DestroySystem,
                &mut fixture.states(),
                &mut || 0.5,
            )
            .expect("the gate passes");

            for (character, refuge) in refuges {
                let value = &world.characters[character];
                let sent = fixture.missions.missions().iter().any(|mission| {
                    mission.target_system == refuge
                        && mission.team == vec![MissionMember::Character(character)]
                });
                if both_sides_in_orbit {
                    assert!(value.adrift && !value.on_hidden_mission && !sent);
                    assert_eq!(value.current_system, Some(system));
                } else {
                    assert!(!value.adrift && value.on_hidden_mission && sent);
                }
            }
        }
    }

    /// The cascade's adrift flags post event `0x30e` (`FUN_00534790` →
    /// `FUN_00536460`), served in the order's frame step: with one side in
    /// orbit the battle bit stays clear, so the kill's own sweep starts each
    /// side's trip (`adrift_trips`), and the holder change's request sweep
    /// (event `0x307`) finds no one left to send.
    #[test]
    fn a_destroy_system_kill_starts_its_adrift_trips_from_its_own_sweep() {
        for side in [Faction::Empire, Faction::Alliance] {
            let (mut world, system, fleet, troop) = strike_world();
            world.systems[system].fleets.retain(|&each| each != fleet);
            world.fleets.remove(fleet);
            let victim = if side == Faction::Empire {
                Faction::Alliance
            } else {
                Faction::Empire
            };
            world.systems[system].control = ControlKind::Controlled(victim);
            world.troops[troop].is_alliance = victim == Faction::Alliance;
            let death_star = death_star_fleet(&mut world, system, side);
            let mut characters = Vec::new();
            for each in [Faction::Alliance, Faction::Empire] {
                let refuge = add_system(&mut world, "Refuge");
                world.systems[refuge].control = ControlKind::Controlled(each);
                characters.push(world.characters.insert(rebellion_core::world::Character {
                    is_alliance: each == Faction::Alliance,
                    is_empire: each == Faction::Empire,
                    current_system: Some(system),
                    ..Default::default()
                }));
            }
            let mut fixture = StrikeFixture::new();

            let runs = order_strike(
                &mut world,
                &[death_star],
                rebellion_core::bombardment::BombardKind::DestroySystem,
                &mut fixture.states(),
                &mut || 0.5,
            )
            .expect("the gate passes");

            let mut sent: Vec<_> = runs[0]
                .adrift_trips
                .iter()
                .flat_map(|trip| trip.team.clone())
                .collect();
            sent.sort_by_key(|member| format!("{member:?}"));
            let mut expected: Vec<_> = characters
                .iter()
                .map(|&character| MissionMember::Character(character))
                .collect();
            expected.sort_by_key(|member| format!("{member:?}"));
            assert_eq!(sent, expected, "{side:?}");
            assert!(
                runs[0].swept.iter().all(|(_, trips)| trips.is_empty()),
                "{side:?}"
            );
        }
    }

    /// `FUN_0058d990` rolls the system's minor characters inside the kill
    /// phase; the adrift flags' events `0x30e` are served after the order,
    /// so a character the roll kills (GNPRTB `0x606`) joins no trip.
    #[test]
    fn a_character_the_destroy_system_roll_kills_joins_no_adrift_trip() {
        let (mut world, system, _, _) = strike_world();
        let entry = |id: u32, value: i32| rebellion_core::world::GnprtbEntry {
            parameter_id: id,
            development: value,
            alliance_sp_easy: value,
            alliance_sp_medium: value,
            alliance_sp_hard: value,
            empire_sp_easy: value,
            empire_sp_medium: value,
            empire_sp_hard: value,
            multiplayer: value,
        };
        // strike_world's GNPRTB 0x601-0x603, and a certain character roll.
        world.gnprtb = rebellion_core::world::GnprtbParams::new(vec![
            entry(0x601, 40),
            entry(0x602, 1),
            entry(0x603, 10),
            entry(0x606, 100),
        ]);
        let death_star = death_star_fleet(&mut world, system, Faction::Empire);
        let refuge = add_system(&mut world, "Refuge");
        world.systems[refuge].control = ControlKind::Controlled(Faction::Alliance);
        let minor = world.characters.insert(rebellion_core::world::Character {
            is_alliance: true,
            current_system: Some(system),
            ..Default::default()
        });
        let mut fixture = StrikeFixture::new();

        order_strike(
            &mut world,
            &[death_star],
            rebellion_core::bombardment::BombardKind::DestroySystem,
            &mut fixture.states(),
            &mut || 0.5,
        )
        .expect("the gate passes");

        assert!(world.characters[minor].is_killed);
        assert!(fixture.missions.missions().iter().all(|mission| !mission
            .members()
            .any(|m| m == MissionMember::Character(minor))));
    }

    /// A jail abode holds a prisoner with no captor in no system or fleet
    /// (`FUN_00549580`; `death-star-strategic.md`, "Jail abodes"). Every
    /// change of its captivity takes it out: a story capture naming a
    /// captor, the carbonite release and a rescue (slots `+0x20c`,
    /// `+0x210`, `+0x218`, "Statuses"); an escape's release is
    /// `death_star::free_escapee`.
    #[test]
    fn a_change_of_captivity_takes_a_character_out_of_its_jail() {
        use rebellion_core::events::EventAction;
        for is_alliance in [true, false] {
            let (mut world, system, _, _) = strike_world();
            let mut jailed = || {
                world.characters.insert(rebellion_core::world::Character {
                    is_alliance,
                    is_empire: !is_alliance,
                    is_captive: true,
                    abode: Some(rebellion_core::world::Abode::jail_of(!is_alliance)),
                    ..Default::default()
                })
            };
            let (captured, thawed, rescued) = (jailed(), jailed(), jailed());
            let enemy = if is_alliance {
                Faction::Empire
            } else {
                Faction::Alliance
            };

            apply_event_action_to_world(
                &[
                    EventAction::CaptureCharacter {
                        character: captured,
                        captor_faction: enemy,
                    },
                    EventAction::SetCarboniteState {
                        character: thawed,
                        frozen: false,
                    },
                ],
                &mut world,
                &mut Vec::new(),
                3,
                &MovementState::new(),
            );
            apply_mission_effects_inner(
                &[MissionEffect::CharacterRescued {
                    character: rescued,
                    returned_to: if is_alliance {
                        MissionFaction::Alliance
                    } else {
                        MissionFaction::Empire
                    },
                    at_system: system,
                }],
                &mut world,
                3,
                &mut UprisingState::new(),
            );

            for key in [captured, thawed, rescued] {
                assert_eq!(world.characters[key].abode, None, "{is_alliance} {key:?}");
            }
        }
    }

    /// `FUN_005180e0` after the cascade's event `0x300`: Coruscant destroyed
    /// sets CD (the Alliance's objective); the Alliance HQ destroyed with
    /// its system sets AD (the Empire's), in a Headquarters Only game.
    #[test]
    fn destroy_system_on_coruscant_or_the_alliance_hq_decides_the_game() {
        use rebellion_core::victory::{VictoryOutcome, VictoryState, VictorySystem};
        for (target_is_coruscant, winner) in [(true, Faction::Alliance), (false, Faction::Empire)] {
            let mut world = GameWorld::default();
            let coruscant = add_system(&mut world, "Coruscant");
            world.systems[coruscant].dat_id = DatId::new(0x9000_0109);
            world.systems[coruscant].is_headquarters = true;
            let base = add_system(&mut world, "Yavin");
            world.systems[base].dat_id = DatId::new(0x9000_0121);
            world.systems[base].is_headquarters = true;
            world.systems[base].control = ControlKind::Controlled(Faction::Alliance);
            // FUN_005180e0 reads the four principal leaders by id.
            for id in [
                rebellion_core::victory::LUKE,
                rebellion_core::victory::MON_MOTHMA,
                rebellion_core::victory::VADER,
                rebellion_core::victory::PALPATINE,
            ] {
                world.characters.insert(rebellion_core::world::Character {
                    dat_id: DatId::new(id),
                    ..rebellion_core::world::Character::default()
                });
            }
            let target = if target_is_coruscant { coruscant } else { base };
            let death_star = death_star_fleet(&mut world, target, Faction::Empire);
            let state = VictoryState::new(base, coruscant);
            let tick = [rebellion_core::tick::TickEvent { tick: 1 }];
            assert_eq!(
                VictorySystem::check(
                    &state,
                    &world,
                    &tick,
                    rebellion_core::world::VictoryConditions::HeadquartersOnly
                ),
                None
            );

            order_strike(
                &mut world,
                &[death_star],
                rebellion_core::bombardment::BombardKind::DestroySystem,
                &mut StrikeFixture::new().states(),
                &mut || 0.5,
            )
            .expect("the gate passes");

            let outcome = VictorySystem::check(
                &state,
                &world,
                &tick,
                rebellion_core::world::VictoryConditions::HeadquartersOnly,
            );
            match outcome {
                Some(VictoryOutcome::HqCaptured { winner: won, .. }) => {
                    assert!(target_is_coruscant);
                    assert_eq!(won, winner);
                }
                Some(VictoryOutcome::DeathStarVictory { target_system }) => {
                    assert!(!target_is_coruscant);
                    assert_eq!(target_system, base);
                    assert!(!world.systems[base].is_headquarters);
                }
                other => panic!("no decision: {other:?}"),
            }
        }
    }

    /// The only refusal is a destroyed target (`FUN_00507f10`): the last
    /// system, the bombarder's own, is legal, and the Death Star's fleet
    /// stays in its orbit.
    #[test]
    fn destroy_system_on_the_last_system_runs_and_the_fleet_stays() {
        let mut world = GameWorld::default();
        let last = add_system(&mut world, "Last");
        let death_star = death_star_fleet(&mut world, last, Faction::Empire);

        let runs = order_strike(
            &mut world,
            &[death_star],
            rebellion_core::bombardment::BombardKind::DestroySystem,
            &mut StrikeFixture::new().states(),
            &mut || 0.5,
        )
        .expect("the gate passes");

        assert_eq!(runs[0].destroyed(), Some(last));
        assert!(world.systems[last].is_destroyed);
        assert_eq!(world.systems[last].fleets, vec![death_star]);
        assert_eq!(world.fleets[death_star].location, last);
    }

    /// The Kill branch's report and records (`FUN_0058d990`): the cascade
    /// first, then the minor-character roll (GNPRTB `0x606`, here 100, so
    /// the adrift character dies), the finalize flags for a kill; the
    /// strike's records are EVT_CONTROL_CHANGED for the turn to neutral,
    /// EVT_DS_FIRED with the cascade's counts and one
    /// EVT_CHARACTER_KILLED per rolled character.
    #[test]
    fn a_destroy_system_reports_its_run_and_records_the_cascade_and_the_dead() {
        let (mut world, system, _, _) = strike_world();
        world.gnprtb =
            rebellion_core::world::GnprtbParams::new(vec![rebellion_core::world::GnprtbEntry {
                parameter_id: 0x606,
                development: 100,
                alliance_sp_easy: 100,
                alliance_sp_medium: 100,
                alliance_sp_hard: 100,
                empire_sp_easy: 100,
                empire_sp_medium: 100,
                empire_sp_hard: 100,
                multiplayer: 100,
            }]);
        let minor = world.characters.insert(rebellion_core::world::Character {
            name: "Lieutenant".into(),
            is_alliance: true,
            current_system: Some(system),
            ..Default::default()
        });
        let death_star = death_star_fleet(&mut world, system, Faction::Empire);

        let mut runs = order_strike(
            &mut world,
            &[death_star],
            rebellion_core::bombardment::BombardKind::DestroySystem,
            &mut StrikeFixture::new().states(),
            &mut || 0.5,
        )
        .expect("the gate passes");
        let strike = runs.remove(0);

        let report = strike.report.clone().expect("the run's report");
        assert_eq!(
            report.kind,
            Some(rebellion_core::bombardment::BombardKind::DestroySystem)
        );
        assert_eq!(report.bombarder, Some(Faction::Empire));
        assert_eq!(report.holder, Some(Faction::Alliance));
        assert_eq!(report.characters, vec![minor]);
        assert_eq!(
            report.run,
            rebellion_core::bombardment::BombardmentRun {
                support_loss: true,
                planet_destroyed: true,
            }
        );
        assert_eq!(
            strike.cascade.as_ref().map(|c| c.adrift.clone()),
            Some(vec![minor])
        );
        assert!(world.characters[minor].is_killed);

        let mut integrator = PerceptionIntegrator::new(3, 0);
        integrator.emit_strike(&world, &strike);
        let records = integrator.finish();
        let kinds: Vec<_> = records.iter().map(|record| record.event_type).collect();
        assert_eq!(
            kinds,
            [EVT_CONTROL_CHANGED, EVT_DS_FIRED, EVT_CHARACTER_KILLED]
        );
        assert_eq!(records[0].details["to"], "Uncontrolled");
        assert_eq!(records[1].details["system"], "Target");
        assert_eq!(records[1].details["regiments"], 1);
        assert_eq!(
            records[1].details["adrift"],
            serde_json::json!(["Lieutenant"])
        );
        assert_eq!(records[2].details["name"], "Lieutenant");
    }

    /// The AI order names the system it strikes; a fleet that is elsewhere
    /// strikes nothing (port: the posture emitter `FUN_004b1070` acts on an
    /// arrived fleet).
    #[test]
    fn an_ai_strike_on_a_system_its_fleet_is_not_at_is_refused() {
        let (mut world, system, fleet, troop) = strike_world();
        let elsewhere = add_system(&mut world, "Elsewhere");
        let (outcome, strike) = resolve_ai_strike(
            &AIAction::Bombard {
                fleet,
                system: elsewhere,
                kind: rebellion_core::ai_regions::Strike::Military,
            },
            fleet_side(&world, fleet),
            &mut world,
            &mut StrikeFixture::new().states(),
            &mut || 0.5,
        );
        assert_eq!(outcome, AiOutcome::Refused(AiRefusal::StrikeSystemMissing));
        assert!(strike.report.is_none());
        assert!(world.troops.contains_key(troop));
        assert!(!world.systems[system].is_destroyed);
    }

    #[test]
    fn a_completed_death_star_can_fire_from_its_new_fleet() {
        // CAPSHPSD.DAT record 136 is the Death Star (TEXTSTRA 10120).
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Yard");
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(rebellion_core::world::DEATH_STAR_CLASS_ID),
            family: 0x18,
            is_empire: true,
            hull: 25_000,
            bombardment_modifier: 15,
            ..Default::default()
        });
        let target = add_system(&mut world, "Target");
        world.systems[target].control = ControlKind::Controlled(Faction::Alliance);

        order_and_finish(
            &mut world,
            system,
            rebellion_core::manufacturing::Destination::System(system),
            class,
            1,
        );

        let fleet = world.systems[system].fleets[0];
        assert!(world.fleet_has_death_star(fleet));
        world.systems[system].fleets.clear();
        world.systems[target].fleets.push(fleet);
        world.fleets[fleet].location = target;
        // FUN_00582440: the fleet's Death Star ship passes Destroy System's gate.
        assert_eq!(
            rebellion_core::bombardment::gate(
                &world,
                &[fleet],
                rebellion_core::bombardment::BombardKind::DestroySystem
            ),
            Ok(())
        );
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
            bombarding: false,
        });
        world.systems[system].fleets.push(fleet);
        let troop = world.troops.insert(rebellion_core::world::TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: false,
        });
        world.systems[system].ground_units.push(troop);
        let mut transport = TroopTransportState::default();
        transport
            .embark(&mut world, &ProductRoom::default(), fleet, &[troop])
            .unwrap();
        // The load's own phase 5 (FUN_00508660).
        rebellion_core::support_shift::fire(&mut world, 0);
        let mut integrator = PerceptionIntegrator::new(9, 0);

        integrator.apply_blockade_events(
            &mut world,
            &mut ManufacturingState::new(),
            &mut transport,
            &rebellion_core::blockade::BlockadeState::new(),
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
    fn a_fleet_has_the_death_star_once_the_ship_joins_it_and_not_for_other_ships() {
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Yard");
        let mut add_class = |id| {
            world.capital_ship_classes.insert(CapitalShipClass {
                dat_id: DatId::new(id),
                family: if id == rebellion_core::world::DEATH_STAR_CLASS_ID {
                    0x18
                } else {
                    0x14
                },
                is_empire: true,
                hull: 100,
                ..Default::default()
            })
        };
        let star_destroyer = add_class(131);
        let death_star = add_class(rebellion_core::world::DEATH_STAR_CLASS_ID);
        let here = rebellion_core::manufacturing::Destination::System(system);
        order_and_finish(&mut world, system, here, star_destroyer, 1);
        let fleet = world.systems[system].fleets[0];
        assert!(!world.fleet_has_death_star(fleet));

        // Ordered with the fleet as its Destination, the Death Star is seated
        // in it (FUN_004c27b0 -> FUN_00552300's fleet branch).
        let into = rebellion_core::manufacturing::Destination::Fleet(fleet);
        order_and_finish(&mut world, system, into, death_star, 2);
        assert_eq!(world.systems[system].fleets, [fleet]);
        assert!(world.fleet_has_death_star(fleet));
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
            bombarding: false,
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

    // ── Support shifts (support_shift, ghidra/notes/support-shifts.md) ──

    /// A sector of two populated core systems held by `holder` at 50
    /// support, with the shipped GNPRTB 0x1407/0x1408/0x1420 (-10, 101, 10)
    /// and SDPRTB 0x1410-0x1415 (101, 20, 100, 150, 0, 100) and 0x1420/0x1421
    /// (1, 100).
    fn shift_sector(holder: Faction) -> (GameWorld, SystemKey, SystemKey) {
        shift_sector_at(holder, 0)
    }

    /// [`shift_sector`] with GNPRTB 7760, the support a side needs to hold
    /// a system without regiments (`FUN_0055a080`), at `threshold`.
    fn shift_sector_at(holder: Faction, threshold: i32) -> (GameWorld, SystemKey, SystemKey) {
        let gn = |id: u32, value: i32| rebellion_core::world::GnprtbEntry {
            parameter_id: id,
            development: value,
            alliance_sp_easy: value,
            alliance_sp_medium: value,
            alliance_sp_hard: value,
            empire_sp_easy: value,
            empire_sp_medium: value,
            empire_sp_hard: value,
            multiplayer: value,
        };
        let sd = |id: u32, value: i32| rebellion_core::world::SdprtbEntry {
            parameter_id: id,
            dev_alliance: value,
            dev_empire: value,
            alliance_sp_easy_alliance: value,
            alliance_sp_easy_empire: value,
            alliance_sp_medium_alliance: value,
            alliance_sp_medium_empire: value,
            alliance_sp_hard_alliance: value,
            alliance_sp_hard_empire: value,
            empire_sp_easy_alliance: value,
            empire_sp_easy_empire: value,
            empire_sp_medium_alliance: value,
            empire_sp_medium_empire: value,
            empire_sp_hard_alliance: value,
            empire_sp_hard_empire: value,
            multiplayer_alliance: value,
            multiplayer_empire: value,
        };
        let mut world = GameWorld {
            gnprtb: rebellion_core::world::GnprtbParams::new(vec![
                gn(0x1407, -10),
                gn(0x1408, 101),
                gn(0x1420, 10),
                gn(7760, threshold),
                gn(0x601, 40),
                gn(0x602, 1),
                gn(0x603, 10),
                gn(7761, 60),
                gn(7762, -10),
                gn(0x1e03, 2),
            ]),
            sdprtb: rebellion_core::world::SdprtbParams::new(vec![
                sd(0x1410, 101),
                sd(0x1411, 20),
                sd(0x1412, 100),
                sd(0x1413, 150),
                sd(0x1414, 0),
                sd(0x1415, 100),
                sd(0x1420, 1),
                sd(0x1421, 100),
            ]),
            ..GameWorld::default()
        };
        let sector = world.sectors.insert(Sector::default());
        let mut keys = Vec::new();
        for _ in 0..2 {
            let key = world.systems.insert(System {
                sector,
                is_populated: true,
                popularity_alliance: 0.5,
                popularity_empire: 0.5,
                control: ControlKind::Controlled(holder),
                ..System::default()
            });
            world.sectors[sector].systems.push(key);
            world.system_families.insert(key, 0x90);
            keys.push(key);
        }
        (world, keys[0], keys[1])
    }

    /// One regiment of `side` (a class with no slot `+0x1c0` value) as
    /// `system`'s only regiment: it holds the system when the support hook
    /// reruns `FUN_0050b610` (troop bits `.c:34-70`).
    fn garrison(world: &mut GameWorld, system: SystemKey, side: Faction) {
        for unit in std::mem::take(&mut world.systems[system].ground_units) {
            world.troops.remove(unit);
        }
        let unit = world.troops.insert(rebellion_core::world::TroopUnit {
            class_dat_id: DatId::new(0x1000_0f00),
            is_alliance: side == Faction::Alliance,
        });
        world.systems[system].ground_units.push(unit);
    }

    fn alliance_points(world: &GameWorld, system: SystemKey) -> i32 {
        rebellion_core::uprising::support_points(&world.systems[system], Faction::Alliance)
    }

    /// A regiment class whose slot `+0x1c0` (TROOPSD `uprising_defense`)
    /// is `value`.
    fn valued_regiment_class(world: &mut GameWorld, value: u32) -> DatId {
        let class = DatId::new(0x1000_0001);
        world.buildable_classes.insert(
            class,
            rebellion_core::world::BuildableClass {
                stats: rebellion_core::world::ClassStats {
                    uprising_defense: value,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        class
    }

    #[test]
    fn a_holder_change_by_support_shifts_its_sector_toward_the_new_holder() {
        // FUN_00517aa0 -> FUN_0055de50 on event 0x31b: 2 when the sides
        // trade a system with no troop bits. Twenty points to the captor
        // take the system to 70 (GNPRTB 7760 (60), FUN_0055a080), and its
        // setter's hook reruns the holder (FUN_005109f0 -> FUN_0050b610);
        // the other system holds by its regiment.
        for (holder, captor, expected, other_expected) in [
            (Faction::Alliance, Faction::Empire, 28, 48),
            (Faction::Empire, Faction::Alliance, 72, 52),
        ] {
            let (mut world, system, other) = shift_sector_at(holder, 60);
            garrison(&mut world, other, holder);
            rebellion_core::uprising::apply_support_change(&mut world, system, captor, 20);
            PerceptionIntegrator::new(1, 0)
                .fire_support_shifts(&mut world, &mut ManufacturingState::new());
            assert_eq!(
                world.systems[system].control,
                ControlKind::Controlled(captor)
            );
            assert_eq!(alliance_points(&world, system), expected, "{captor:?}");
            assert_eq!(alliance_points(&world, other), other_expected, "{captor:?}");
            assert!(world.support_events.is_empty());
        }
    }

    #[test]
    fn an_ai_outcome_view_carries_each_outcomes_own_fields() {
        let applied = AiOutcome::applied(Some(7)).view();
        assert_eq!((applied.outcome, applied.mission_id), ("applied", Some(7)));
        let refusal = AiRefusal::Mission(MissionRefusal::TargetRefused(
            rebellion_core::missions::TargetRefusal::ContainerDestroyed,
        ));
        let refused = AiOutcome::Refused(refusal).view();
        assert_eq!(refused.outcome, "refused");
        assert_eq!(refused.refusal, Some(refusal.name()));
        // FUN_00522480: a destroyed container ends the mission with code 7.
        assert_eq!(refused.end_code, Some(7));
        let mut world = GameWorld::default();
        let system = world.systems.insert(System::default());
        let held = AiOutcome::Held { system }.view();
        assert_eq!((held.outcome, held.held_at), ("held", Some(system)));
    }

    #[test]
    fn a_space_force_summary_counts_its_sides_held_squadrons_and_each_fleets_living_ships() {
        let mut world = GameWorld::default();
        let mut sys = System::default();
        sys.add_squadrons(Default::default(), 3, true);
        sys.add_squadrons(Default::default(), 2, false);
        let system = world.systems.insert(sys);
        assert_eq!(
            summarize_space_force(&world, system, true, &[]).fighter_squadrons,
            3
        );
        assert_eq!(
            summarize_space_force(&world, system, false, &[]).fighter_squadrons,
            2
        );
        // Each fleet adds its living ships, their hull and its squadrons.
        let ship = |hull, alive| {
            let mut ship = rebellion_core::world::ShipInstance::new(Default::default(), hull, true);
            ship.alive = alive;
            ship
        };
        let fleets: Vec<FleetKey> = [100, 200]
            .into_iter()
            .map(|hull| {
                world.fleets.insert(rebellion_core::world::Fleet {
                    capital_ships: vec![ship(hull, true), ship(50, false)],
                    fighters: vec![rebellion_core::world::FighterEntry {
                        class: Default::default(),
                        count: 2,
                        carrier: 0,
                    }],
                    ..Default::default()
                })
            })
            .collect();
        let summary = summarize_space_force(&world, system, true, &fleets);
        assert_eq!(
            (
                summary.capital_ships,
                summary.fighter_squadrons,
                summary.hull
            ),
            (2, 7, 300)
        );
    }

    #[test]
    fn an_assault_that_takes_a_system_leaves_its_enemy_characters_free() {
        // No code on the control-change path captures, ejects or moves a
        // character (FUN_00515c00 -> FUN_005613f0 acts only at Coruscant;
        // ghidra/notes/ground-assault.md, section 4).
        for (holder, captor) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system, _) = shift_sector(holder);
            let officer = world.characters.insert(rebellion_core::world::Character {
                is_alliance: holder == Faction::Alliance,
                is_empire: holder == Faction::Empire,
                current_system: Some(system),
                ..Default::default()
            });
            let mut fixture = StrikeFixture::new();
            let (fleet, _) = assault_fleet(
                &mut world,
                &mut fixture.transport,
                system,
                captor,
                DatId::new(0x1000_0001),
                1,
            );
            let effects = resolve_assault(&mut world, system, &[fleet], &mut fixture.states(), 0.5);
            assert_eq!(
                effects.report.unwrap().holder_after,
                Some(captor),
                "{captor:?}"
            );
            let character = &world.characters[officer];
            assert!(!character.is_captive, "{captor:?}");
            assert_eq!(character.captured_by, None);
            assert_eq!(character.current_system, Some(system));
        }
    }

    #[test]
    fn a_capture_from_garrison_to_garrison_takes_the_troop_record() {
        // FUN_0050b610 sets the word's troop bits while one side's
        // regiments stand there; both words having them gives SDPRTB
        // 0x1444 (0 as shipped, here unset), not 0x1420's point per side
        // (FUN_0055de50.c:82-88).
        for (holder, captor) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system, other) = shift_sector(holder);
            let regiment = |world: &mut GameWorld, side: Faction| {
                world.troops.insert(rebellion_core::world::TroopUnit {
                    is_alliance: side == Faction::Alliance,
                    ..Default::default()
                })
            };
            let garrison = regiment(&mut world, holder);
            world.systems[system].ground_units.push(garrison);
            rebellion_core::support_shift::refresh_all_troop_bits(&mut world);
            world.systems[system].ground_units.clear();
            let mut fixture = StrikeFixture::new();
            let (fleet, _) = assault_fleet(
                &mut world,
                &mut fixture.transport,
                system,
                captor,
                DatId::new(0x1000_0001),
                1,
            );
            resolve_assault(&mut world, system, &[fleet], &mut fixture.states(), 0.5);
            assert_eq!(
                world.systems[system].control,
                ControlKind::Controlled(captor)
            );
            assert_eq!(alliance_points(&world, other), 50, "{captor:?}");
            assert!(world.troop_held.contains(&system));
        }
    }

    /// `captor`'s assault with one regiment on `system`, where no regiment
    /// stands: the landing takes it.
    fn land_one(world: &mut GameWorld, system: SystemKey, captor: Faction) -> AssaultEffects {
        let mut fixture = StrikeFixture::new();
        let (fleet, _) = assault_fleet(
            world,
            &mut fixture.transport,
            system,
            captor,
            DatId::new(0x1000_0001),
            1,
        );
        resolve_assault(world, system, &[fleet], &mut fixture.states(), 0.5)
    }

    /// A troop class with TROOPSD `attack`. port: test fixture values, not
    /// rules.
    fn striking_class(world: &mut GameWorld, attack: u32) -> DatId {
        let class = DatId::new(0x1000_0002);
        world.troop_classes.insert(
            class,
            rebellion_core::world::TroopClassDef {
                attack_strength: attack,
                defense_strength: 0,
                detection: 0,
            },
        );
        class
    }

    fn blockaded(fixture: &mut StrikeFixture, world: &GameWorld) {
        BlockadeSystem::advance(
            &mut fixture.blockade,
            world,
            &[rebellion_core::tick::TickEvent { tick: 1 }],
        );
    }

    #[test]
    fn an_ai_assault_runs_the_players_order_and_takes_an_undefended_system() {
        // FUN_004b1070 builds 0x234 with the fleet as the team; the order
        // runs FUN_0058c120 as the player's does, and the landing turns the
        // holder (FUN_0050b610).
        for (holder, captor) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system, _) = shift_sector(holder);
            let mut fixture = StrikeFixture::new();
            let (fleet, troops) = assault_fleet(
                &mut world,
                &mut fixture.transport,
                system,
                captor,
                DatId::new(0x1000_0001),
                2,
            );
            blockaded(&mut fixture, &world);
            let (outcome, effects) = resolve_ai_assault(
                &AIAction::Assault { fleet, system },
                fleet_side(&world, fleet),
                &mut world,
                &mut fixture.states(),
                &mut || 0.5,
            );
            assert!(outcome.is_applied(), "{captor:?}");
            assert_eq!(effects.report.unwrap().landed, troops);
            assert_eq!(
                world.systems[system].control,
                ControlKind::Controlled(captor)
            );
            assert_eq!(
                effects.previous_control,
                Some(ControlKind::Controlled(holder))
            );
        }
    }

    #[test]
    fn an_ai_assault_on_an_unblockaded_or_shielded_system_is_refused() {
        // FUN_004fddd0 (8/7) and FUN_00507d00 (0x90/5): the gate
        // FUN_004ae240 does not mirror them, so the order fails and the AI
        // restarts its module; it tries no bombardment instead.
        for (holder, captor) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system, _) = shift_sector(holder);
            let mut fixture = StrikeFixture::new();
            let (fleet, troops) = assault_fleet(
                &mut world,
                &mut fixture.transport,
                system,
                captor,
                DatId::new(0x1000_0001),
                1,
            );
            let action = AIAction::Assault { fleet, system };
            let (outcome, effects) =
                resolve_ai_assault(
                    &action,
                    fleet_side(&world, fleet),
                    &mut world,
                    &mut fixture.states(),
                    &mut || 0.5,
                );
            assert_eq!(
                outcome,
                AiOutcome::Refused(AiRefusal::Assault(
                    rebellion_core::assault::AssaultRefusal::NotBlockaded
                )),
                "{captor:?}"
            );
            assert!(effects.report.is_none());
            world.gnprtb = rebellion_core::world::GnprtbParams::new(vec![
                rebellion_core::world::GnprtbEntry {
                    parameter_id: 0x1e03,
                    development: 2,
                    alliance_sp_easy: 2,
                    alliance_sp_medium: 2,
                    alliance_sp_hard: 2,
                    empire_sp_easy: 2,
                    empire_sp_medium: 2,
                    empire_sp_hard: 2,
                    multiplayer: 2,
                },
            ]);
            for _ in 0..2 {
                let shield = world.defense_facilities.insert(DefenseFacilityInstance {
                    class_dat_id: DatId::new(0x2400_0001),
                    side: holder,
                });
                world.systems[system].defense_facilities.push(shield);
            }
            blockaded(&mut fixture, &world);
            let (outcome, _) =
                resolve_ai_assault(
                    &action,
                    fleet_side(&world, fleet),
                    &mut world,
                    &mut fixture.states(),
                    &mut || 0.5,
                );
            assert_eq!(
                outcome,
                AiOutcome::Refused(AiRefusal::Assault(
                    rebellion_core::assault::AssaultRefusal::Shields
                ))
            );
            assert_eq!(fixture.transport.cargo(fleet), troops);
            assert!(!world.fleets[fleet].bombarding);
        }
    }

    #[test]
    fn a_landing_that_takes_the_system_ends_its_blockade_before_another_order() {
        // The holder write runs the side-bits hook (FUN_00510820 ->
        // FUN_0050ac10(1)), whose event 0x311 reruns FUN_0050b8e0 at once:
        // the captor's own fleet clears the bit, so a second order in the
        // same step is refused 8/7 (FUN_004fddd0), and the end posts its
        // loss shift (FUN_00517ee0).
        for (holder, captor) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system, _) = shift_sector(holder);
            let mut fixture = StrikeFixture::new();
            let (fleet, _) = assault_fleet(
                &mut world,
                &mut fixture.transport,
                system,
                captor,
                DatId::new(0x1000_0001),
                8,
            );
            blockaded(&mut fixture, &world);
            assert!(fixture.blockade.is_blockaded(system));
            let first = order_assault(
                &mut world,
                &[fleet],
                captor,
                &mut fixture.states(),
                &mut || 0.5,
            )
            .unwrap();
            assert_eq!(first[0].report.as_ref().unwrap().landed.len(), 6);
            assert_eq!(world.systems[system].control, ControlKind::Controlled(captor));
            assert!(!fixture.blockade.is_blockaded(system), "{captor:?}");
            let [(BlockadeEvent::BlockadeEnded { system: ended, .. }, payload)] =
                first[0].blockade.as_slice()
            else {
                panic!("one blockade end: {:?}", first[0].blockade);
            };
            assert_eq!(*ended, system);
            // The end's record carries the blockader and its counters
            // (blockade_end_payload), and its record is written with the run.
            assert_eq!(payload["blockader"], format!("{captor:?}"));
            let mut integrator = PerceptionIntegrator::new(1, 0);
            integrator.emit_assault(&world, &first[0]);
            let records = integrator.finish();
            assert!(records.iter().any(|r| r.event_type == EVT_BLOCKADE_ENDED));
            assert!(!records.iter().any(|r| r.event_type == EVT_BLOCKADE_STARTED));
            assert_eq!(fixture.transport.cargo(fleet).len(), 2);
            assert_eq!(
                order_assault(
                    &mut world,
                    &[fleet],
                    captor,
                    &mut fixture.states(),
                    &mut || 0.5,
                ),
                Err(rebellion_core::assault::AssaultRefusal::NotBlockaded),
                "{captor:?}"
            );
            // A later tick finds nothing left to end.
            assert!(BlockadeSystem::advance(
                &mut fixture.blockade,
                &world,
                &[rebellion_core::tick::TickEvent { tick: 2 }],
            )
            .iter()
            .all(|event| !matches!(event, BlockadeEvent::BlockadeEnded { .. })));
        }
    }

    #[test]
    fn the_report_names_each_facility_the_collateral_hits_destroyed() {
        // Each kill's event 0x302 carries the dead object (FUN_00497380),
        // listed at the system (+0x80). GNPRTB 0x203/0x204 out of reach of
        // rand[0, 10] leave every engagement standing; 0x205 = 100 makes
        // each one's collateral roll hit (FUN_0055eb80).
        for (holder, captor) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let mut world = GameWorld::default();
            let mut entries = landing_limit();
            for (id, value) in [(0x202, 10), (0x203, -1), (0x204, 11), (0x205, 100)] {
                let mut entry = entries[0].clone();
                entry.parameter_id = id;
                for slot in [
                    &mut entry.development,
                    &mut entry.alliance_sp_easy,
                    &mut entry.alliance_sp_medium,
                    &mut entry.alliance_sp_hard,
                    &mut entry.empire_sp_easy,
                    &mut entry.empire_sp_medium,
                    &mut entry.empire_sp_hard,
                    &mut entry.multiplayer,
                ] {
                    *slot = value;
                }
                entries.push(entry);
            }
            world.gnprtb = rebellion_core::world::GnprtbParams::new(entries);
            let system = add_system(&mut world, "Kuat");
            world.systems[system].control = ControlKind::Controlled(holder);
            let defender = world.troops.insert(rebellion_core::world::TroopUnit {
                class_dat_id: DatId::new(0x1000_0001),
                is_alliance: holder == Faction::Alliance,
            });
            world.systems[system].ground_units.push(defender);
            let mut named = Vec::new();
            for (raw, name) in [(0x2800_0001, "Yard A"), (0x2800_0002, "Yard B")] {
                world.buildable_classes.insert(
                    DatId::new(raw),
                    rebellion_core::world::BuildableClass {
                        name: name.into(),
                        ..Default::default()
                    },
                );
                let yard = world.manufacturing_facilities.insert(ManufacturingFacilityInstance {
                    class_dat_id: DatId::new(raw),
                    side: holder,
                    is_shipyard: false,
                });
                world.systems[system].manufacturing_facilities.push(yard);
                named.push((rebellion_core::world::FacilityRef::Manufacturing(yard), name));
            }
            let mut fixture = StrikeFixture::new();
            let (fleet, _) = assault_fleet(
                &mut world,
                &mut fixture.transport,
                system,
                captor,
                DatId::new(0x1000_0001),
                3,
            );
            let effects = resolve_assault(&mut world, system, &[fleet], &mut fixture.states(), 0.5);
            let lost = &effects.report.as_ref().unwrap().facilities_lost;
            assert_eq!(lost.len(), 2, "{captor:?}");
            let expected: Vec<&str> = lost
                .iter()
                .map(|key| named.iter().find(|(each, _)| each == key).unwrap().1)
                .collect();
            let report = effects.operation.expect("the Operation Report");
            assert_eq!(names(&report.system_destroyed), expected, "{captor:?}");
        }
    }

    #[test]
    fn a_second_order_runs_the_pass_again_with_the_survivors() {
        // FUN_0058c8b0 clears the flags and nothing repeats the pass; a
        // second order counts the survivors afresh (FUN_0058c2d0). Attack
        // 100 kills every defender it engages (FUN_0055eb00).
        for (holder, captor) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system, _) = shift_sector(holder);
            let defenders: Vec<TroopKey> = (0..2)
                .map(|_| {
                    let troop = world.troops.insert(rebellion_core::world::TroopUnit {
                        class_dat_id: DatId::new(0x1000_0001),
                        is_alliance: holder == Faction::Alliance,
                    });
                    world.systems[system].ground_units.push(troop);
                    troop
                })
                .collect();
            let class = striking_class(&mut world, 100);
            let mut fixture = StrikeFixture::new();
            let (fleet, troops) =
                assault_fleet(&mut world, &mut fixture.transport, system, captor, class, 1);
            blockaded(&mut fixture, &world);
            let first = order_assault(
                &mut world,
                &[fleet],
                captor,
                &mut fixture.states(),
                &mut || 0.5,
            )
            .unwrap();
            let report = first[0].report.as_ref().unwrap();
            assert_eq!(report.defenders_lost.len(), 1, "{captor:?}");
            assert!(report.landed.is_empty());
            assert_eq!(
                world.systems[system].control,
                ControlKind::Controlled(holder)
            );
            let second = order_assault(
                &mut world,
                &[fleet],
                captor,
                &mut fixture.states(),
                &mut || 0.5,
            )
            .unwrap();
            let report = second[0].report.as_ref().unwrap();
            assert_eq!((report.attackers, report.defenders), (1, 1));
            assert_eq!(report.landed, troops);
            assert!(defenders
                .iter()
                .all(|&troop| !world.troops.contains_key(troop)));
            assert_eq!(
                world.systems[system].control,
                ControlKind::Controlled(captor)
            );
        }
    }

    #[test]
    fn an_assault_kill_feeds_its_sides_loss_counter_a_and_a_lone_loss_changes_no_holder() {
        // FUN_0058c3c0.c:44: a battery's kill is slot +0xac(0xe), whose
        // event 0x300 adds the regiment's value to its side's A
        // (FUN_00508c60); with every attacker dead and the garrison
        // standing, FUN_0050b610 keeps the holder.
        for (holder, captor) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system, _) = shift_sector(holder);
            let class = valued_regiment_class(&mut world, 9);
            let garrison = world.troops.insert(rebellion_core::world::TroopUnit {
                class_dat_id: class,
                is_alliance: holder == Faction::Alliance,
            });
            world.systems[system].ground_units.push(garrison);
            let battery = DatId::new(0x2200_0001);
            world.buildable_classes.insert(
                battery,
                rebellion_core::world::BuildableClass {
                    stats: rebellion_core::world::ClassStats {
                        attack_strength: 2000,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
            let kdy = world.defense_facilities.insert(DefenseFacilityInstance {
                class_dat_id: battery,
                side: holder,
            });
            world.systems[system].defense_facilities.push(kdy);
            let mut fixture = StrikeFixture::new();
            let (fleet, troops) =
                assault_fleet(&mut world, &mut fixture.transport, system, captor, class, 1);
            let effects = resolve_assault(&mut world, system, &[fleet], &mut fixture.states(), 0.5);
            let report = effects.report.unwrap();
            assert_eq!(
                report.attackers_lost,
                [(troops[0], rebellion_core::assault::Cause::DefenceFire)],
                "{captor:?}"
            );
            let index = usize::from(captor == Faction::Empire);
            assert_eq!(fixture.blockade.losses(system).unwrap().a[index], 9);
            assert_eq!(effects.previous_control, None);
            assert_eq!(report.holder_after, Some(holder));
            assert!(world.troops.contains_key(garrison));
        }
    }

    #[test]
    fn an_assault_landing_shifts_its_sector_and_removes_a_captured_hq() {
        // Event 0x31b after the landing; the Empire's capture of the
        // Alliance HQ also destroys it with reason 3 (FUN_004f8680) and
        // costs the Alliance GNPRTB 0x1407 (-10) over the sector
        // (FUN_00517cd0). The landed regiment gives the new word troop
        // bits, so the change's own record is SDPRTB 0x1440/0x1441
        // (FUN_0055cb60.c:70-73), unset here, not 0x1420's point per side.
        let (mut world, system, other) = shift_sector(Faction::Alliance);
        world.systems[system].is_headquarters = true;
        let effects = land_one(&mut world, system, Faction::Empire);
        assert!(!world.systems[system].is_headquarters);
        assert_eq!(alliance_points(&world, other), 40);
        assert!(!effects.shifts.is_empty());
        let (mut world, system, other) = shift_sector(Faction::Empire);
        land_one(&mut world, system, Faction::Alliance);
        assert_eq!(
            world.systems[system].control,
            ControlKind::Controlled(Faction::Alliance)
        );
        assert_eq!(alliance_points(&world, other), 50);
    }

    #[test]
    fn a_lifted_blockade_shifts_its_sector_by_the_loss_counters() {
        // FUN_00515f40 -> FUN_00517ee0 at BlockadeEnded. With no units
        // T1 = T2 = 0, both weights are 0 and nothing moves. With one
        // regiment a side (S = the Empire, d = 0) the blockader's 40
        // points of losses give the holder (2*40 - 0) / 10 = 8.
        for (holder, expected) in [(Faction::Alliance, 58), (Faction::Empire, 42)] {
            let blockader = if holder == Faction::Alliance {
                Faction::Empire
            } else {
                Faction::Alliance
            };
            // GNPRTB 7760 at 50: the shifted systems keep their holder
            // when the support hook reruns FUN_0050b610 (FUN_0055a080).
            let (mut world, system, other) = shift_sector_at(holder, 50);
            let mut blockade = rebellion_core::blockade::BlockadeState::new();
            blockade.start_losses(system, blockader);
            blockade.record_loss(
                system,
                rebellion_core::blockade::UnitLoss {
                    side: blockader,
                    value: 40,
                    reason: rebellion_core::blockade::REASON_BOMBARDMENT,
                },
            );
            let ended = [BlockadeEvent::BlockadeEnded { system, tick: 3 }];
            let mut integrator = PerceptionIntegrator::new(3, 0);
            let mut transport = TroopTransportState::default();
            integrator.apply_blockade_events(
                &mut world,
                &mut ManufacturingState::new(),
                &mut transport,
                &blockade,
                &ended,
            );
            assert_eq!(
                alliance_points(&world, other),
                50,
                "no strength, {holder:?}"
            );
            let class = valued_regiment_class(&mut world, 5);
            for is_alliance in [true, false] {
                world.troops.insert(rebellion_core::world::TroopUnit {
                    class_dat_id: class,
                    is_alliance,
                });
            }
            integrator.apply_blockade_events(
                &mut world,
                &mut ManufacturingState::new(),
                &mut transport,
                &blockade,
                &ended,
            );
            assert_eq!(alliance_points(&world, system), expected, "{holder:?}");
            assert_eq!(alliance_points(&world, other), expected, "{holder:?}");
            // Each record carries the counters and strength sums the shift
            // read: none at the first lift, one regiment of value 5 a side
            // at the second.
            let records = integrator.finish();
            let ended: Vec<_> = records
                .iter()
                .filter(|record| record.event_type == EVT_BLOCKADE_ENDED)
                .collect();
            assert_eq!(ended.len(), 2, "{holder:?}");
            let losses = |side: Faction| if side == blockader { 40 } else { 0 };
            for (record, (strength, amount)) in ended.iter().zip([(0, 0), (5, 8)]) {
                let details = &record.details;
                assert_eq!(details["blockader"], format!("{blockader:?}"), "{holder:?}");
                assert_eq!(
                    details["alliance_losses"],
                    losses(Faction::Alliance),
                    "{holder:?}"
                );
                assert_eq!(
                    details["empire_losses"],
                    losses(Faction::Empire),
                    "{holder:?}"
                );
                assert_eq!(details["alliance_strength"], strength, "{holder:?}");
                assert_eq!(details["empire_strength"], strength, "{holder:?}");
                assert_eq!(details["shift"]["amount"], amount, "{holder:?}");
            }
            assert_eq!(
                ended[1].details["shift"]["direction"],
                format!("{holder:?}")
            );

            // With no loss record no shift is evaluated, and the record
            // says so rather than reading as a zero.
            let mut integrator = PerceptionIntegrator::new(3, 0);
            let empty = rebellion_core::blockade::BlockadeState::new();
            let ended_events = [BlockadeEvent::BlockadeEnded { system, tick: 3 }];
            integrator.apply_blockade_events(
                &mut world,
                &mut ManufacturingState::new(),
                &mut transport,
                &empty,
                &ended_events,
            );
            let records = integrator.finish();
            let record = records
                .iter()
                .find(|record| record.event_type == EVT_BLOCKADE_ENDED)
                .expect("a blockade_ended record");
            assert!(record.details["blockader"].is_null(), "{holder:?}");
            assert!(record.details["shift"].is_null(), "{holder:?}");
        }
    }

    /// `bombarder`'s one-ship fleet (strength 5) at the first system of a
    /// [`shift_sector`] the other side holds.
    fn bombard_sector(bombarder: Faction) -> (GameWorld, SystemKey, SystemKey, FleetKey) {
        let holder = if bombarder == Faction::Alliance {
            Faction::Empire
        } else {
            Faction::Alliance
        };
        let (mut world, system, other) = shift_sector(holder);
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            family: 0x14,
            hull: 100,
            bombardment_modifier: 5,
            ..Default::default()
        });
        let is_alliance = bombarder == Faction::Alliance;
        let fleet = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ShipInstance::new(class, 100, is_alliance)],
            is_alliance,
            ..Fleet::default()
        });
        world.systems[system].fleets.push(fleet);
        (world, system, other, fleet)
    }

    #[test]
    fn a_bombardment_kill_feeds_the_holders_counter_b() {
        // FUN_00508c60 on event 0x300: reason 0xf at an enemy system adds
        // the regiment's slot +0x1c0 (TROOPSD uprising_defense) to B.
        for bombarder in [Faction::Empire, Faction::Alliance] {
            let (mut world, system, _, fleet) = bombard_sector(bombarder);
            let holder = rebellion_core::bombardment::holder(&world, system);
            let class = valued_regiment_class(&mut world, 6);
            let troop = world.troops.insert(rebellion_core::world::TroopUnit {
                class_dat_id: class,
                is_alliance: holder == Faction::Alliance,
            });
            world.systems[system].ground_units.push(troop);
            let mut fixture = StrikeFixture::new();
            order_strike(
                &mut world,
                &[fleet],
                rebellion_core::bombardment::BombardKind::Military,
                &mut fixture.states(),
                &mut || 0.5,
            )
            .expect("the gate passes");
            assert!(!world.troops.contains_key(troop), "{bombarder:?}");
            let losses = fixture.blockade.losses(system).expect("a loss record");
            let index = usize::from(holder == Faction::Empire);
            assert_eq!(losses.b[index], 6, "{bombarder:?}");
            assert_eq!(losses.a, [0; 3], "{bombarder:?}");
        }
    }

    /// port: a run's cargo cleanup follows its own ship losses (a fleet the
    /// batteries emptied goes, and its regiments with it); a regiment
    /// stranded elsewhere is the tick's own cleanup's
    /// (`simulation.rs`, `destroy_untransportable_cargo`), which reports it.
    #[test]
    fn a_bombardment_that_loses_no_ship_leaves_other_fleets_stranded_cargo_to_the_ticks_cleanup() {
        for bombarder in [Faction::Empire, Faction::Alliance] {
            let (mut world, system, other, fleet) = bombard_sector(bombarder);
            let holder = rebellion_core::bombardment::holder(&world, system);
            let target = world.troops.insert(rebellion_core::world::TroopUnit {
                class_dat_id: DatId::new(0x1000_0001),
                is_alliance: holder == Faction::Alliance,
            });
            world.systems[system].ground_units.push(target);
            let is_alliance = bombarder == Faction::Alliance;
            let class = world.capital_ship_classes.insert(CapitalShipClass {
                family: 0x14,
                hull: 100,
                troop_capacity: 1,
                ..Default::default()
            });
            let carrier = world.fleets.insert(Fleet {
                location: other,
                capital_ships: vec![ShipInstance::new(class, 100, is_alliance)],
                is_alliance,
                ..Fleet::default()
            });
            world.systems[other].fleets.push(carrier);
            let troop = world.troops.insert(rebellion_core::world::TroopUnit {
                class_dat_id: DatId::new(0x1000_0001),
                is_alliance,
            });
            world.systems[other].ground_units.push(troop);
            let mut fixture = StrikeFixture::new();
            fixture
                .transport
                .embark(&mut world, &ProductRoom::default(), carrier, &[troop])
                .expect("the regiment boards");
            rebellion_core::support_shift::fire(&mut world, 0);
            world.fleets[carrier].capital_ships[0].alive = false;

            let runs = order_strike(
                &mut world,
                &[fleet],
                rebellion_core::bombardment::BombardKind::Military,
                &mut fixture.states(),
                &mut || 0.5,
            )
            .expect("the gate passes");

            let report = runs[0].report.as_ref().expect("the run's report");
            assert_eq!(report.ships_destroyed, 0, "{bombarder:?}");
            assert_eq!(fixture.transport.cargo(carrier), [troop], "{bombarder:?}");
            assert!(world.troops.contains_key(troop), "{bombarder:?}");
        }
    }

    #[test]
    fn a_bombardment_hit_on_the_alliance_hq_costs_its_sector_ten_points() {
        // FUN_0058ce70 destroys the HQ (no class value here, so every hit
        // lands); its event 0x300 runs FUN_00517cd0 (GNPRTB 0x1407).
        let (mut world, system, other, fleet) = bombard_sector(Faction::Empire);
        world.systems[system].is_headquarters = true;
        let mut fixture = StrikeFixture::new();
        order_strike(
            &mut world,
            &[fleet],
            rebellion_core::bombardment::BombardKind::Military,
            &mut fixture.states(),
            &mut || 0.5,
        )
        .expect("the gate passes");
        assert!(!world.systems[system].is_headquarters);
        assert_eq!(alliance_points(&world, other), 40);
        assert!(world.support_events.is_empty());
    }

    #[test]
    fn a_civilian_kill_moves_the_bombarders_support_and_its_record_says_so() {
        // FUN_0058e320: a civilian kill sets +0x28, and GNPRTB 7705 (-20)
        // moves the bombarder's support at the system at once.
        for bombarder in [Faction::Empire, Faction::Alliance] {
            let (mut world, system, _, fleet) = bombard_sector(bombarder);
            let gn = |id: u32, value: i32| rebellion_core::world::GnprtbEntry {
                parameter_id: id,
                development: value,
                alliance_sp_easy: value,
                alliance_sp_medium: value,
                alliance_sp_hard: value,
                empire_sp_easy: value,
                empire_sp_medium: value,
                empire_sp_hard: value,
                multiplayer: value,
            };
            world.gnprtb = rebellion_core::world::GnprtbParams::new(vec![
                gn(0x601, 40),
                gn(0x602, 1),
                gn(0x603, 10),
                gn(0x1e19, -20),
                // The holder keeps the system at 70 when the setter's hook
                // reruns FUN_0050b610 (GNPRTB 7760, FUN_0055a080).
                gn(7760, 50),
            ]);
            let holder = rebellion_core::bombardment::holder(&world, system);
            let yard = world.manufacturing_facilities.insert(
                rebellion_core::world::ManufacturingFacilityInstance {
                    side: holder,
                    ..Default::default()
                },
            );
            world.systems[system].manufacturing_facilities.push(yard);
            let before =
                rebellion_core::uprising::support_points(&world.systems[system], bombarder);
            let mut fixture = StrikeFixture::new();
            let runs = order_strike(
                &mut world,
                &[fleet],
                rebellion_core::bombardment::BombardKind::Civilian,
                &mut fixture.states(),
                &mut || 0.5,
            )
            .expect("the gate passes");
            assert_eq!(runs[0].support_change, -20, "{bombarder:?}");
            assert_eq!(
                rebellion_core::uprising::support_points(&world.systems[system], bombarder),
                before - 20,
                "{bombarder:?}"
            );
            let mut integrator = PerceptionIntegrator::new(1, 0);
            integrator.emit_strike(&world, &runs[0]);
            let records = integrator.finish();
            let record = records
                .iter()
                .find(|record| record.event_type == EVT_BOMBARDMENT)
                .expect("a bombardment record");
            assert_eq!(record.details["support_change"], -20, "{bombarder:?}");
            assert_eq!(record.details["facilities"], 1, "{bombarder:?}");
        }
    }

    /// A Death Star ship added to `fleet` (family 0x18).
    fn arm_with_death_star(world: &mut GameWorld, fleet: FleetKey) {
        let is_alliance = world.fleets[fleet].is_alliance;
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(rebellion_core::world::DEATH_STAR_CLASS_ID),
            family: 0x18,
            hull: 25_000,
            bombardment_modifier: 15,
            ..Default::default()
        });
        world.fleets[fleet]
            .capital_ships
            .push(ShipInstance::new(class, 25_000, is_alliance));
    }

    #[test]
    fn destroy_system_turns_the_system_neutral_and_feeds_its_units_to_counter_b() {
        // FUN_0058ce70 destroys the system's units with reason 0xf at an
        // enemy system (FUN_004f8710); each event 0x300 adds the unit's
        // slot +0x1c0 to B (FUN_00508c60). The populated clear reads the
        // owner as neutral (FUN_0050b5a0; the VM's Yaga Minor).
        for bombarder in [Faction::Empire, Faction::Alliance] {
            let (mut world, system, _, fleet) = bombard_sector(bombarder);
            arm_with_death_star(&mut world, fleet);
            let holder = rebellion_core::bombardment::holder(&world, system);
            let class = valued_regiment_class(&mut world, 6);
            let troop = world.troops.insert(rebellion_core::world::TroopUnit {
                class_dat_id: class,
                is_alliance: holder == Faction::Alliance,
            });
            world.systems[system].ground_units.push(troop);
            let previous = world.systems[system].control;
            let mut fixture = StrikeFixture::new();
            let runs = order_strike(
                &mut world,
                &[fleet],
                rebellion_core::bombardment::BombardKind::DestroySystem,
                &mut fixture.states(),
                &mut || 0.5,
            )
            .expect("the gate passes");
            assert!(world.systems[system].is_destroyed, "{bombarder:?}");
            assert_eq!(
                world.systems[system].control,
                ControlKind::Uncontrolled,
                "{bombarder:?}"
            );
            let losses = fixture.blockade.losses(system).expect("a loss record");
            assert_eq!(
                losses.b[usize::from(holder == Faction::Empire)],
                6,
                "{bombarder:?}"
            );

            // The change of holder is recorded like every other.
            let mut integrator = PerceptionIntegrator::new(1, 0);
            integrator.emit_strike(&world, &runs[0]);
            let records = integrator.finish();
            let change = records
                .iter()
                .find(|record| record.event_type == EVT_CONTROL_CHANGED)
                .expect("a control_changed record");
            assert_eq!(
                change.details["from"],
                format!("{previous:?}"),
                "{bombarder:?}"
            );
            assert_eq!(change.details["to"], "Uncontrolled", "{bombarder:?}");
            assert_eq!(change.details["cause"], "planet_destroyed", "{bombarder:?}");
        }
    }

    #[test]
    fn destroy_system_on_the_alliance_hq_costs_its_sector_ten_points_and_one_for_turning_neutral() {
        // The populated clear turns the system neutral (FUN_0050b760 ->
        // FUN_0050b5a0): the control change removes the HQ (FUN_004f8680,
        // +0xac(3)), whose event 0x300 posts FUN_00517cd0's 10, and shifts
        // the sector 1 away from the Alliance (FUN_0055de50; the VM's Yaga
        // Minor "took 1 back", ghidra/notes/support-shifts.md).
        let (mut world, system, other, fleet) = bombard_sector(Faction::Empire);
        world.systems[system].is_headquarters = true;
        arm_with_death_star(&mut world, fleet);
        let mut fixture = StrikeFixture::new();
        let runs = order_strike(
            &mut world,
            &[fleet],
            rebellion_core::bombardment::BombardKind::DestroySystem,
            &mut fixture.states(),
            &mut || 0.5,
        )
        .expect("the gate passes");
        assert_eq!(runs.len(), 1);
        assert!(runs[0]
            .cascade
            .as_ref()
            .is_some_and(|cascade| cascade.headquarters));
        assert!(runs[0]
            .report
            .as_ref()
            .is_some_and(|report| report.headquarters));
        // The system is unpopulated before the finalize: 7705 moves nothing.
        assert_eq!(runs[0].support_change, 0);
        assert!(!world.systems[system].is_headquarters);
        assert_eq!(world.systems[system].control, ControlKind::Uncontrolled);
        assert_eq!(alliance_points(&world, other), 50 - 10 - 1);

        // Each fired shift is recorded with what posted it.
        let mut integrator = PerceptionIntegrator::new(1, 0);
        integrator.emit_strike(&world, &runs[0]);
        let causes: Vec<(String, i64)> = integrator
            .finish()
            .iter()
            .filter(|record| record.event_type == EVT_SUPPORT_SHIFT)
            .map(|record| {
                (
                    record.details["cause"]
                        .as_str()
                        .unwrap_or_default()
                        .to_owned(),
                    record.details["delta"].as_i64().unwrap_or_default(),
                )
            })
            .collect();
        assert!(
            causes.contains(&("Headquarters".to_owned(), -10)),
            "{causes:?}"
        );
        assert!(
            causes.contains(&("ControlChange".to_owned(), -1)),
            "{causes:?}"
        );
    }

    #[test]
    fn each_started_adrift_trip_is_recorded_as_a_dispatched_adrift_mission() {
        let mut world = GameWorld::default();
        let from = add_system(&mut world, "Ruin");
        let to = add_system(&mut world, "Refuge");
        let character = world.characters.insert(rebellion_core::world::Character {
            name: "Wedge".to_owned(),
            is_alliance: true,
            ..Default::default()
        });
        let mut missions = MissionState::new();
        let request = |tick| {
            MissionRequest::single(
                rebellion_core::missions::MissionKind::Move,
                rebellion_core::missions::MissionFaction::Alliance,
                character,
                to,
                None,
                tick,
            )
        };
        let trip = missions.dispatch(request(1));
        missions.dispatch(request(2));
        let mut integrator = PerceptionIntegrator::new(1, 0);
        integrator.emit_adrift_trips(&world, &missions, from, &[trip]);
        let records = integrator.finish();
        assert_eq!(records.len(), 1);
        let details = &records[0].details;
        assert_eq!(records[0].event_type, EVT_MISSION_DISPATCHED);
        assert_eq!(details["mission_id"], trip);
        assert_eq!(details["kind"], "Adrift");
        assert_eq!(details["team"], serde_json::json!(["Wedge"]));
        assert_eq!(details["from_system"], "Ruin");
        assert_eq!(details["target_system"], "Refuge");
    }

    #[test]
    fn destroying_an_imperial_system_shifts_its_sector_one_point_toward_the_alliance() {
        // Old holder 2 and new 3 give from 2, to 1 (FUN_0055de50): the
        // Empire's SDPRTB 0x1420 amount toward the Alliance, whoever fired.
        for bombarder in [Faction::Alliance, Faction::Empire] {
            let (mut world, system, other) = shift_sector(Faction::Empire);
            let class = world.capital_ship_classes.insert(CapitalShipClass {
                family: 0x14,
                hull: 100,
                bombardment_modifier: 5,
                ..Default::default()
            });
            let is_alliance = bombarder == Faction::Alliance;
            let fleet = world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![ShipInstance::new(class, 100, is_alliance)],
                is_alliance,
                ..Fleet::default()
            });
            world.systems[system].fleets.push(fleet);
            arm_with_death_star(&mut world, fleet);
            let mut fixture = StrikeFixture::new();
            let runs = order_strike(
                &mut world,
                &[fleet],
                rebellion_core::bombardment::BombardKind::DestroySystem,
                &mut fixture.states(),
                &mut || 0.5,
            )
            .expect("the gate passes");
            assert!(world.systems[system].is_destroyed, "{bombarder:?}");
            assert!(
                runs[0]
                    .shifts
                    .iter()
                    .any(|shift| shift.cause
                        == rebellion_core::support_shift::ShiftCause::ControlChange),
                "{bombarder:?} {:?}",
                runs[0].shifts
            );
            assert_eq!(alliance_points(&world, other), 50 + 1, "{bombarder:?}");
        }
    }

    #[test]
    fn a_finished_regiment_takes_its_system_the_day_it_completes() {
        // The finished regiment's events 0x320 and 0x301 at its system run
        // FUN_00508660 -> FUN_0050b610: its side's regiments alone set the
        // holder (.c:34-70); the support at 50 holds for no side (GNPRTB
        // 7760 (60), FUN_0055a080).
        for (holder, builder) in [
            (Faction::Alliance, Faction::Empire),
            (Faction::Empire, Faction::Alliance),
        ] {
            let (mut world, system, _) = shift_sector_at(holder, 60);
            let class = DatId::new(0x1000_0003);
            world.buildable_classes.insert(
                class,
                rebellion_core::world::BuildableClass {
                    is_alliance: builder == Faction::Alliance,
                    is_empire: builder == Faction::Empire,
                    ..Default::default()
                },
            );
            let mut integrator = PerceptionIntegrator::new(1, 0);
            integrator.apply_build_completions(
                &mut world,
                &mut ManufacturingState::new(),
                &[CompletionEvent {
                    system,
                    tick: 1,
                    kind: BuildableKind::Troop(class),
                }],
            );
            assert_eq!(
                world.systems[system].control,
                ControlKind::Controlled(builder),
                "{builder:?}"
            );
        }
    }

    #[test]
    fn a_kill_of_a_garrisoned_system_turns_it_neutral_as_its_last_regiment_dies() {
        // The populated clear's FUN_0050b610 keeps the troop side
        // (FUN_0050b610.c:34-70); each regiment's death clears its usable
        // bit (FUN_004f7410 -> hook +0x110 -> FUN_004fb310, event 0x301),
        // whose FUN_00514510 -> FUN_00508660 reruns FUN_0050b610 in the
        // kill's phase 5: with none left the holder is 3, and the change
        // carries the troop bits of the old word only (FUN_0055de50).
        for holder in [Faction::Alliance, Faction::Empire] {
            let bombarder = if holder == Faction::Alliance {
                Faction::Empire
            } else {
                Faction::Alliance
            };
            let (mut world, system, _) = shift_sector_at(holder, 60);
            garrison(&mut world, system, holder);
            world.troop_held.insert(system);
            let class = world.capital_ship_classes.insert(CapitalShipClass {
                family: 0x14,
                hull: 100,
                ..Default::default()
            });
            let is_alliance = bombarder == Faction::Alliance;
            let fleet = world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![ShipInstance::new(class, 100, is_alliance)],
                is_alliance,
                ..Fleet::default()
            });
            world.systems[system].fleets.push(fleet);
            arm_with_death_star(&mut world, fleet);
            let mut fixture = StrikeFixture::new();
            let runs = order_strike(
                &mut world,
                &[fleet],
                rebellion_core::bombardment::BombardKind::DestroySystem,
                &mut fixture.states(),
                &mut || 0.5,
            )
            .expect("the gate passes");
            assert_eq!(world.systems[system].control, ControlKind::Uncontrolled);
            assert_eq!(
                runs[0]
                    .controls
                    .iter()
                    .filter(|change| change.system == system)
                    .map(|change| (change.previous, change.control, change.troops))
                    .collect::<Vec<_>>(),
                [(
                    ControlKind::Controlled(holder),
                    ControlKind::Uncontrolled,
                    (true, false)
                )],
                "{holder:?}"
            );
            assert_eq!(runs[0].previous_control, Some(ControlKind::Controlled(holder)));
        }
    }

    #[test]
    fn a_kill_whose_shift_turns_a_neutral_system_makes_that_join_in_its_step() {
        // The kill's control change posts +1 toward the Alliance
        // (FUN_0055de50); its fire goes through the support setter
        // FUN_00509c30, whose hook FUN_005109f0 reruns FUN_0050b610 at once,
        // and phase 5 serves the join's event 0x31b before the step ends
        // (FUN_0051df30). GNPRTB 7760 (60) is the support a side needs.
        for bombarder in [Faction::Alliance, Faction::Empire] {
            let (mut world, system, other) = shift_sector_at(Faction::Empire, 60);
            world.systems[other].control = ControlKind::Uncontrolled;
            world.systems[other].popularity_alliance = 0.59;
            world.systems[other].popularity_empire = 0.41;
            let class = world.capital_ship_classes.insert(CapitalShipClass {
                family: 0x14,
                hull: 100,
                ..Default::default()
            });
            let is_alliance = bombarder == Faction::Alliance;
            let fleet = world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![ShipInstance::new(class, 100, is_alliance)],
                is_alliance,
                ..Fleet::default()
            });
            world.systems[system].fleets.push(fleet);
            arm_with_death_star(&mut world, fleet);
            let mut fixture = StrikeFixture::new();
            let runs = order_strike(
                &mut world,
                &[fleet],
                rebellion_core::bombardment::BombardKind::DestroySystem,
                &mut fixture.states(),
                &mut || 0.5,
            )
            .expect("the gate passes");
            assert_eq!(
                world.systems[other].control,
                ControlKind::Controlled(Faction::Alliance),
                "{bombarder:?}"
            );
            assert_eq!(
                runs[0]
                    .controls
                    .iter()
                    .filter(|change| change.system == other)
                    .map(|change| (change.previous, change.control))
                    .collect::<Vec<_>>(),
                [(
                    ControlKind::Uncontrolled,
                    ControlKind::Controlled(Faction::Alliance)
                )],
                "{bombarder:?}"
            );
        }
    }

    /// The control-change shifts of an AI Destroy System order run in
    /// in-day step `step` of day 1, after the day's control pass took a
    /// system from the target's holder.
    fn destroy_after_daily_pass(
        holder: Faction,
        step: u16,
    ) -> Vec<rebellion_core::support_shift::AppliedShift> {
        let bombarder = if holder == Faction::Alliance {
            Faction::Empire
        } else {
            Faction::Alliance
        };
        // Each system holds by support (GNPRTB 7760, 60) whatever the shifts
        // do to it (FUN_005109f0 -> FUN_0050b610): `captured` at 80 for the
        // bombarder, `target` at 80 for its holder.
        let (mut world, captured, target) = shift_sector_at(holder, 60);
        for (system, side) in [(captured, bombarder), (target, holder)] {
            let alliance = if side == Faction::Alliance { 0.8 } else { 0.2 };
            world.systems[system].popularity_alliance = alliance;
            world.systems[system].popularity_empire = 1.0 - alliance;
        }
        // The day's first step: a point of support to the bombarder runs the
        // setter's hook (FUN_005109f0 -> FUN_0050b610), changing `captured`
        // from old holder `holder` with no troop bits.
        rebellion_core::uprising::apply_support_change(&mut world, captured, bombarder, 1);
        PerceptionIntegrator::new(1, 0)
            .fire_support_shifts(&mut world, &mut ManufacturingState::new());
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            family: 0x14,
            hull: 100,
            ..Default::default()
        });
        let is_alliance = bombarder == Faction::Alliance;
        let fleet = world.fleets.insert(Fleet {
            location: target,
            capital_ships: vec![ShipInstance::new(class, 100, is_alliance)],
            is_alliance,
            ..Fleet::default()
        });
        world.systems[target].fleets.push(fleet);
        arm_with_death_star(&mut world, fleet);
        let mut ai = AIState::new(if is_alliance {
            AiFaction::Alliance
        } else {
            AiFaction::Empire
        });
        ai.scheduler.pending = Some(rebellion_core::ai_scheduler::Issued {
            kind: 9,
            action: AIAction::DestroySystem {
                fleet,
                system: target,
            },
            day: 1,
            round: step.saturating_sub(1),
        });
        let mut fixture = StrikeFixture::new();
        let mut stockpiles = rebellion_core::stockpiles::StockpileState::new();
        let mut victory = rebellion_core::victory::VictoryState::new(captured, target);
        let research = rebellion_core::research::ResearchState::new();
        let config = rebellion_core::tuning::GameConfig::default();
        let mut ctx = crate::simulation::AiCtx {
            world: &mut world,
            missions: &mut fixture.missions,
            manufacturing: &mut fixture.manufacturing,
            movement: &mut fixture.movement,
            transport: &mut fixture.transport,
            deliveries: &mut fixture.deliveries,
            stockpiles: &mut stockpiles,
            victory: &mut victory,
            blockade: &mut fixture.blockade,
            research: &research,
            uprising: &fixture.uprisings,
            config: &config,
        };
        let (records, _) = crate::simulation::ai_step(
            &mut ctx,
            &mut [(&mut ai, false)],
            None,
            step,
            1,
            &|_, _, _| None,
            &mut |n| vec![0.5; n],
        );
        assert!(ctx.world.systems[target].is_destroyed, "{holder:?} {step}");
        records[0].strike.shifts.clone()
    }

    #[test]
    fn a_destroy_system_kill_in_a_later_step_shifts_though_the_daily_pass_spent_its_holders_cap() {
        // FUN_00518040 counts changes per old holder in one scheduler step
        // (FUN_004fd330, +0xc), raised on every frame step
        // (FUN_0051dea0.c:11). The day's control pass runs in its first step
        // (FUN_0051df30); an AI order runs in the in-day step that executes
        // it (FUN_00436250), whose counters are its own.
        let control_change = |shifts: &[rebellion_core::support_shift::AppliedShift]| {
            shifts.iter().any(|shift| {
                shift.cause == rebellion_core::support_shift::ShiftCause::ControlChange
            })
        };
        for holder in [Faction::Empire, Faction::Alliance] {
            assert!(
                control_change(&destroy_after_daily_pass(holder, 3)),
                "{holder:?} step 3"
            );
            assert!(
                !control_change(&destroy_after_daily_pass(holder, 0)),
                "{holder:?} step 0"
            );
        }
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

/// What a battle reports once the post-battle pass has run
/// (`FUN_00517270.c:79-93`): the side still present (a usable fleet or
/// squadrons the system holds, [`side_present`]) holds the system, the
/// Alliance as the attacker, and with both present the system stays
/// contested, a stalemate. `FUN_0050a040` writes that side to the system and
/// `FUN_00517ee0` builds the result from it; the battle's own winner
/// (`FUN_0040c780`, `+0x64`) only steers the pass. Unwatched and played
/// battles report the same.
pub(crate) fn post_battle_outcome(world: &GameWorld, system: SystemKey) -> (CombatSide, bool) {
    match (side_present(world, system, true), side_present(world, system, false)) {
        (true, false) => (CombatSide::Attacker, false),
        (false, true) => (CombatSide::Defender, false),
        (true, true) => (CombatSide::Draw, true),
        (false, false) => (CombatSide::Draw, false),
    }
}

/// The Alliance's and the Empire's fleets that fight at `system`
/// ([`rebellion_core::combat::fighting_fleets`]: the usable ones).
pub(crate) fn orbiting_fleets_by_faction(
    world: &GameWorld,
    system: SystemKey,
) -> (Vec<FleetKey>, Vec<FleetKey>) {
    use rebellion_core::combat::fighting_fleets;
    (
        fighting_fleets(world, system, true),
        fighting_fleets(world, system, false),
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
        None,
        &rebellion_core::carriage::ProductRoom::default(),
    )
}

/// [`resolve_system_space_combat`] with a person commanding a side
/// (`human_is_alliance`): that side's withdrawal threshold is 0.0
/// (`FUN_004865b0`), a computer side's `FUN_004e6110`'s. `products` is the
/// room squadron and regiment products hold aboard each ship
/// (`tactical::apply::apply_results`).
#[expect(
    clippy::too_many_arguments,
    reason = "The battle reads the world, the transport, movement and product states and its seed."
)]
pub fn resolve_system_space_combat_for(
    world: &mut GameWorld,
    troop_transport: &mut TroopTransportState,
    movement: &mut MovementState,
    system: SystemKey,
    rng_seed: u32,
    tick: u64,
    human_is_alliance: Option<bool>,
    manufacturing: Option<&rebellion_core::manufacturing::ManufacturingState>,
    products: &rebellion_core::carriage::ProductRoom,
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
    if let Some(manufacturing) = manufacturing {
        session.seat_unfinished_station(world, manufacturing);
    }
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
        products,
        &mut withdrawal_log,
    );

    let (alliance, empire) = orbiting_fleets_by_faction(world, system);
    let alliance_after = summarize_space_force(world, system, true, &alliance);
    let empire_after = summarize_space_force(world, system, false, &empire);
    let (winner, stalemate) = post_battle_outcome(world, system);
    let winner_fleet = match winner {
        CombatSide::Attacker => alliance.first().copied(),
        CombatSide::Defender => empire.first().copied(),
        CombatSide::Draw => None,
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
        stalemate,
        log: withdrawal_log,
    }
}

#[cfg(test)]
mod combat_application_tests {
    use super::*;
    use rebellion_core::dat::ExplorationStatus;
    use rebellion_core::ids::SectorKey;
    use rebellion_core::world::{CapitalShipClass, Fleet, ShipInstance, System};

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
        // The battery count cancels out of the damage rate (FUN_005b7780:
        // count x factor x recharge / the arc's weapon count); a stronger
        // ship needs a faster weapon_recharge_rate or shields.
        turbolasers: u32,
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
            turbolaser_fore: turbolasers,
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
            bombarding: false,
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

    // FUN_0040b0e0 exports only a fleet's usable children, and the
    // post-battle pass FUN_00516ef0 walks the side's fleets in mode 1
    // (FUN_004ffef0): a fleet listed first whose only ship is still on its
    // leg from the yard (bit 5, FUN_00556430) puts nothing in the battle,
    // so the side fights, wins and follows up under its usable fleet.
    #[test]
    fn a_fleet_whose_only_ship_is_on_its_own_leg_neither_fights_nor_wins_for_either_side() {
        for is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let system = add_combat_system(&mut world);
            let yard = add_combat_system(&mut world);
            let new_hull = add_ship_fleet(&mut world, system, is_alliance, 200, 100);
            world.fleets[new_hull].capital_ships[0].transit =
                Some(rebellion_core::world::ShipTransit {
                    from: yard,
                    to: system,
                    arrival_tick: 30,
                });
            let standing = add_ship_fleet(&mut world, system, is_alliance, 200, 100);
            let enemy = add_ship_fleet(&mut world, system, !is_alliance, 30, 0);

            let result = resolve_system_space_combat(
                &mut world,
                &mut TroopTransportState::default(),
                &mut MovementState::default(),
                system,
                0x5eed,
                11,
            );

            let (fleets, winner) = if is_alliance {
                (result.alliance_fleets, CombatSide::Attacker)
            } else {
                (result.empire_fleets, CombatSide::Defender)
            };
            assert_eq!(fleets, 1, "{is_alliance}");
            assert_eq!(
                result.winner,
                winner,
                "{is_alliance}: {:?} {:?} {:?}",
                result.log,
                result.alliance_after,
                result.empire_after
            );
            assert_eq!(result.winner_fleet, Some(standing), "{is_alliance}");
            assert!(!world.fleets.contains_key(enemy), "{is_alliance}");
            assert!(
                world.fleets[new_hull].capital_ships[0].transit.is_some(),
                "{is_alliance}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Build completion helper (moved from simulation.rs)
// ---------------------------------------------------------------------------

/// A product delivered to a fleet, whose container held it en route
/// (`crate::delivery`): a squadron or regiment is aboard the ship its order
/// chose (`Boarded::carrier`), where the original created it
/// (`FUN_0052a140` -> `FUN_004f7d50`) and its completion only sets the
/// completed bit (`FUN_0052bee0`); any other product lands at the fleet's
/// system. A capital ship never travels this way: it is a member of its
/// fleet from its order (`manufacturing::seat_ordered_ships`).
pub fn apply_boarding_inner(
    boarded: &rebellion_core::delivery::Boarded,
    world: &mut GameWorld,
    transport: &mut TroopTransportState,
) {
    let (fleet, carrier, completion) = (boarded.fleet, boarded.carrier, &boarded.completion);
    match completion.kind {
        BuildableKind::Fighter(class_key) => {
            rebellion_core::carriage::board_squadron(world, fleet, class_key, carrier);
        }
        BuildableKind::Troop(class) => {
            let Some(is_alliance) = world.buildable_classes.get(&class).map(|c| c.is_alliance)
            else {
                return;
            };
            let troop = world.troops.insert(TroopUnit {
                class_dat_id: class,
                is_alliance,
            });
            transport.board_product(fleet, troop, carrier);
        }
        _ => apply_build_completion_inner(completion, world),
    }
}

/// A finished Death Star's usable bit (`FUN_004f7b80`) posts event `0x301`,
/// which reruns bit 11 for a family `0x18`-`0x1b` object (`FUN_00514510`).
fn refresh_death_star_sectors(world: &mut GameWorld, class: rebellion_core::ids::CapitalShipKey) {
    if world
        .capital_ship_classes
        .get(class)
        .is_some_and(rebellion_core::world::CapitalShipClass::is_death_star)
    {
        rebellion_core::death_star::refresh_sectors(world);
    }
}

/// Capital ships that ended their own legs: a Death Star's usable bit
/// posts event `0x301` only now (`FUN_004f7b80`).
pub fn apply_ship_arrivals_inner(
    world: &mut GameWorld,
    arrivals: &[rebellion_core::delivery::ShipArrival],
) {
    for arrival in arrivals {
        refresh_death_star_sectors(world, arrival.class);
    }
}

/// Products leaving their yards for another place (`FUN_0052bee0` sets en
/// route active when the product's location is not the yard's), less the
/// capital ships: each is a member of its destination fleet from its order
/// (`death-star-product.md` section 1), and `manufacturing::finish_ships`
/// starts its own leg there (`FUN_005150c0.c:90-102` -> `FUN_00556430`).
/// The rest travel in `DeliveryState`.
#[must_use]
pub fn travelling_products(
    departures: &[rebellion_core::manufacturing::Departure],
) -> Vec<rebellion_core::manufacturing::Departure> {
    departures
        .iter()
        .filter(|departure| !matches!(departure.kind, BuildableKind::CapitalShip(_)))
        .cloned()
        .collect()
}

pub fn apply_build_completion_inner(completion: &CompletionEvent, world: &mut GameWorld) {
    let sys_key = completion.system;

    match &completion.kind {
        // The ship was completed in place by `manufacturing::finish_ships`;
        // its usable bit posts event 0x301 now.
        BuildableKind::CapitalShip(class_key) => refresh_death_star_sectors(world, *class_key),
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
            });
            if let Some(sys) = world.systems.get_mut(sys_key) {
                sys.ground_units.push(tk);
                // The finished regiment's events 0x320 and 0x301 at its
                // system (FUN_00508660).
                rebellion_core::support_shift::regiments_changed(world, sys_key);
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
