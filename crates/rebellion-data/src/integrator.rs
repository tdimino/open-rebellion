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

use std::collections::HashMap;

use rebellion_core::ai::AIAction;
use rebellion_core::combat::{CombatSide, GroundCombatResult, SpaceCombatResult};
use rebellion_core::death_star::DeathStarState;
use rebellion_core::economy::EconomyEvent;
use rebellion_core::fog::RevealEvent;
use rebellion_core::game_events::*;
use rebellion_core::ids::{CharacterKey, SystemKey, TroopKey};
use rebellion_core::manufacturing::{BuildableKind, CompletionEvent};
use rebellion_core::missions::{MissionEffect, MissionFaction, MissionResult};
use rebellion_core::movement::ArrivalEvent;
use rebellion_core::uprising::UprisingState;
use rebellion_core::victory::VictoryOutcome;
use rebellion_core::world::{ControlKind, Fleet, FighterEntry, GameWorld, ShipEntry, TroopUnit};

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
            }
        }
    }

    // ── Step 3: Manufacturing + Movement ──────────────────────────────────

    /// Apply build completions: add manufactured items to GameWorld + emit telemetry.
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
        if !result.troop_damage.is_empty() {
            let ground_winner = match result.winner {
                CombatSide::Attacker => "alliance",
                CombatSide::Defender => "empire",
                CombatSide::Draw => "draw",
            };
            self.emit(SYS_COMBAT, EVT_COMBAT_GROUND, serde_json::json!({
                "system": sys_name(world, result.system),
                "winner": ground_winner,
            }));
        }
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
                for (_, fleet) in world.fleets.iter_mut() {
                    fleet.characters.retain(|&k| k != *character);
                }
                world.characters.remove(*character);
            }
            MissionEffect::CharacterCaptured { character, captured_by, .. } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.is_captive = true;
                    c.captured_by = Some(match captured_by {
                        MissionFaction::Alliance => rebellion_core::dat::Faction::Alliance,
                        MissionFaction::Empire => rebellion_core::dat::Faction::Empire,
                    });
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
// Combat helpers (moved from simulation.rs)
// ---------------------------------------------------------------------------

fn apply_space_combat_result_inner(
    result: &SpaceCombatResult,
    world: &mut GameWorld,
) {
    for evt in &result.ship_damage {
        if evt.hull_after > 0 {
            continue;
        }
        let fleet_key = evt.fleet;
        let entry_idx = {
            let fleet = match world.fleets.get(fleet_key) {
                Some(f) => f,
                None => continue,
            };
            let mut offset = 0usize;
            let mut found = None;
            for (i, entry) in fleet.capital_ships.iter().enumerate() {
                if evt.ship_index < offset + entry.count as usize {
                    found = Some(i);
                    break;
                }
                offset += entry.count as usize;
            }
            found
        };
        if let Some(idx) = entry_idx {
            if let Some(fleet) = world.fleets.get_mut(fleet_key) {
                if fleet.capital_ships[idx].count > 0 {
                    fleet.capital_ships[idx].count -= 1;
                }
            }
        }
    }

    for &fleet_key in &[result.attacker_fleet, result.defender_fleet] {
        let is_empty = world
            .fleets
            .get(fleet_key)
            .map(|f| {
                f.capital_ships.iter().all(|e| e.count == 0)
                    && f.fighters.iter().all(|e| e.count == 0)
            })
            .unwrap_or(true);
        if is_empty {
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

fn apply_ground_combat_result_inner(
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
                    if let Some(entry) = fleet.capital_ships.iter_mut().find(|e| e.class == *class_key) {
                        entry.count += 1;
                    } else {
                        fleet.capital_ships.push(ShipEntry { class: *class_key, count: 1 });
                    }
                }
            } else {
                let fleet = Fleet {
                    location: sys_key,
                    capital_ships: vec![ShipEntry { class: *class_key, count: 1 }],
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
