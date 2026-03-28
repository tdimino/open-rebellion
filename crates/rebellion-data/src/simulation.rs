//! Shared simulation tick function for headless and interactive use.
//!
//! Extracts the per-tick system-advance logic from `rebellion-app/main.rs`
//! into a reusable function that returns structured `GameEventRecord`s
//! instead of pushing to a `MessageLog`.

use std::collections::HashMap;

use crate::integrator::PerceptionIntegrator;
use rebellion_core::ai::{AIAction, AIState, AISystem};
use rebellion_core::economy::{EconomyEvent, EconomyState, EconomySystem};
use rebellion_core::betrayal::{BetrayalEvent, BetrayalState, BetrayalSystem};
use rebellion_core::blockade::{BlockadeEvent, BlockadeState, BlockadeSystem};
use rebellion_core::bombardment::BombardmentSystem;
use rebellion_core::combat::{CombatSide, CombatSystem};
use rebellion_core::death_star::{DeathStarEvent, DeathStarState, DeathStarSystem};
use rebellion_core::events::{EventAction, EventState, EventSystem};
use rebellion_core::fog::{FogState, FogSystem};
use rebellion_core::game_events::*;
use rebellion_core::ids::SystemKey;
use rebellion_core::jedi::{JediEvent, JediState, JediSystem};
use rebellion_core::manufacturing::{ManufacturingState, ManufacturingSystem, QueueItem};
use rebellion_core::missions::{MissionEffect, MissionState, MissionSystem};
use rebellion_core::movement::{MovementState, MovementSystem};
use rebellion_core::research::{ResearchResult, ResearchState, ResearchSystem};
use rebellion_core::tick::{GameClock, TickEvent};
use rebellion_core::uprising::{UprisingEvent, UprisingState, UprisingSystem};
use rebellion_core::victory::{VictoryState, VictorySystem};
use rebellion_core::world::{
    ControlKind, DefenseFacilityInstance, GameWorld, ManufacturingFacilityInstance, MstbTable,
    TroopUnit,
};

// Payload helpers live in integrator.rs — import them here.
use crate::integrator::{sys_name, char_name, ai_action_json};

/// Bundles all mutable simulation state needed for a tick.
///
/// Mirrors the set of `*State` locals in `rebellion-app/src/main.rs`.
pub struct SimulationStates {
    pub clock: GameClock,
    pub manufacturing: ManufacturingState,
    pub missions: MissionState,
    pub events: EventState,
    pub ai: AIState,
    /// Optional second AI for dual-AI mode (controls the opposite faction).
    pub ai2: Option<AIState>,
    pub movement: MovementState,
    pub fog: FogState,
    pub blockade: BlockadeState,
    pub uprising: UprisingState,
    pub death_star: DeathStarState,
    pub research: ResearchState,
    pub jedi: JediState,
    pub victory: VictoryState,
    pub betrayal: BetrayalState,
    pub economy: EconomyState,
    pub combat_cooldowns: HashMap<SystemKey, u64>,
}

/// Run one simulation tick across all 15 systems.
///
/// Advances each system in the canonical order (economy → manufacturing → movement →
/// combat → fog → missions → events → AI → blockade → uprising → betrayal →
/// death_star → research → jedi → victory), applies effects to `world`, and
/// returns a `Vec<GameEventRecord>` describing everything that happened.
///
/// `tick_events` comes from `GameClock::advance()`. `rolls` is a pre-generated
/// slice of uniform `[0,1)` f64 values consumed sequentially by systems that
/// need randomness. Pass at least 1024 rolls to cover a typical tick.
///
/// `wall_ms` is the wall-clock milliseconds since session start, used for
/// the `wall_ms` field on each event record.
pub fn run_simulation_tick(
    world: &mut GameWorld,
    states: &mut SimulationStates,
    tick_events: &[TickEvent],
    rolls: &[f64],
    wall_ms: u64,
    config: &rebellion_core::tuning::GameConfig,
) -> Vec<GameEventRecord> {
    if tick_events.is_empty() {
        return Vec::new();
    }

    let mut events = Vec::new();
    let mut integrator = PerceptionIntegrator::new(
        tick_events.last().unwrap().tick,
        wall_ms,
    );
    let mut roll_cursor = 0usize;
    let current_tick = tick_events.last().unwrap().tick;

    // Helper: consume N rolls from the slice, padding with 1.0 if exhausted.
    let mut take_rolls = |n: usize| -> Vec<f64> {
        let end = (roll_cursor + n).min(rolls.len());
        let slice = &rolls[roll_cursor..end];
        roll_cursor = end;
        let mut v: Vec<f64> = slice.to_vec();
        // Pad with 1.0 (safe default: never fires probability checks)
        v.resize(n, 1.0);
        v
    };

    // ── 0. Economy (runs BEFORE manufacturing — affects production) ──────
    let economy_events = EconomySystem::advance(
        &mut states.economy,
        world,
        tick_events,
        world.difficulty_index,
    );
    for ev in &economy_events {
        match ev {
            EconomyEvent::SupportDrifted { system, alliance_delta, empire_delta } => {
                if let Some(sys) = world.systems.get_mut(*system) {
                    sys.popularity_alliance = (sys.popularity_alliance + alliance_delta).clamp(0.0, 1.0);
                    sys.popularity_empire = (sys.popularity_empire + empire_delta).clamp(0.0, 1.0);
                }
                events.push(GameEventRecord::new(
                    current_tick, wall_ms, SYS_ECONOMY, EVT_SUPPORT_DRIFT,
                    serde_json::json!({
                        "system": sys_name(world, *system),
                        "alliance_delta": alliance_delta,
                        "empire_delta": empire_delta,
                    }),
                ));
            }
            EconomyEvent::CollectionRateChanged { system, new_rate } => {
                events.push(GameEventRecord::new(
                    current_tick, wall_ms, SYS_ECONOMY, EVT_COLLECTION_RATE,
                    serde_json::json!({
                        "system": sys_name(world, *system),
                        "rate": new_rate,
                    }),
                ));
            }
            EconomyEvent::GarrisonRequirementChanged { system, new_requirement } => {
                events.push(GameEventRecord::new(
                    current_tick, wall_ms, SYS_ECONOMY, EVT_GARRISON_REQUIRED,
                    serde_json::json!({
                        "system": sys_name(world, *system),
                        "garrison_required": new_requirement,
                    }),
                ));
            }
            EconomyEvent::IncidentTriggered { system, incident_type } => {
                events.push(GameEventRecord::new(
                    current_tick, wall_ms, SYS_ECONOMY, EVT_ECONOMY_TICK,
                    serde_json::json!({
                        "system": sys_name(world, *system),
                        "incident": incident_type,
                    }),
                ));
            }
            EconomyEvent::ControlResolved { system, new_control } => {
                if let Some(sys) = world.systems.get_mut(*system) {
                    sys.control = *new_control;
                }
                events.push(GameEventRecord::new(
                    current_tick, wall_ms, SYS_ECONOMY, EVT_CONTROL_CHANGED,
                    serde_json::json!({
                        "system": sys_name(world, *system),
                        "new_control": format!("{:?}", new_control),
                    }),
                ));
            }
            EconomyEvent::EnergyOvercapped { system, allocated, capacity } => {
                events.push(GameEventRecord::new(
                    current_tick, wall_ms, SYS_ECONOMY, EVT_ECONOMY_TICK,
                    serde_json::json!({
                        "system": sys_name(world, *system),
                        "energy_overcap": true,
                        "allocated": allocated,
                        "capacity": capacity,
                    }),
                ));
            }
            EconomyEvent::RawMaterialOvercapped { system, allocated, capacity } => {
                events.push(GameEventRecord::new(
                    current_tick, wall_ms, SYS_ECONOMY, EVT_ECONOMY_TICK,
                    serde_json::json!({
                        "system": sys_name(world, *system),
                        "raw_material_overcap": true,
                        "allocated": allocated,
                        "capacity": capacity,
                    }),
                ));
            }
        }
    }

    // ── 1. Manufacturing ─────────────────────────────────────────────────
    let completions = ManufacturingSystem::advance_with_blockade(
        &mut states.manufacturing,
        tick_events,
        states.blockade.blockaded_systems(),
    );
    for c in &completions {
        // Apply the built item to the game world.
        apply_build_completion(c, world);
        events.push(GameEventRecord::new(
            c.tick,
            wall_ms,
            SYS_MANUFACTURING,
            EVT_BUILD_COMPLETE,
            serde_json::json!({
                "system": sys_name(world, c.system),
                "kind": format!("{:?}", c.kind),
            }),
        ));
    }

    // ── 2. Movement ──────────────────────────────────────────────────────
    let arrivals = MovementSystem::advance(&mut states.movement, tick_events);
    for arrival in &arrivals {
        // Apply fleet location update
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
        events.push(GameEventRecord::new(
            arrival.tick,
            wall_ms,
            SYS_MOVEMENT,
            EVT_FLEET_ARRIVED,
            serde_json::json!({
                "system": sys_name(world, arrival.system),
                "origin": sys_name(world, arrival.origin),
                "fleet_faction": if world.fleets.get(arrival.fleet).map(|f| f.is_alliance).unwrap_or(false) { "Alliance" } else { "Empire" },
            }),
        ));
    }

    // ── 3. Combat ────────────────────────────────────────────────────────
    let combat_triggers: Vec<_> = world
        .systems
        .keys()
        .filter_map(|sys_key| {
            if let Some(&last_battle) = states.combat_cooldowns.get(&sys_key) {
                if current_tick < last_battle + 5 {
                    return None;
                }
            }
            let sys = &world.systems[sys_key];
            let alliance_fleets: Vec<_> = sys
                .fleets
                .iter()
                .copied()
                .filter(|&k| world.fleets.get(k).map(|f| f.is_alliance).unwrap_or(false))
                .collect();
            let empire_fleets: Vec<_> = sys
                .fleets
                .iter()
                .copied()
                .filter(|&k| {
                    world
                        .fleets
                        .get(k)
                        .map(|f| !f.is_alliance)
                        .unwrap_or(false)
                })
                .collect();
            if !alliance_fleets.is_empty() && !empire_fleets.is_empty() {
                Some((sys_key, alliance_fleets[0], empire_fleets[0]))
            } else {
                None
            }
        })
        .collect();

    for (sys_key, atk_fleet, def_fleet) in combat_triggers {
        let combat_rolls = take_rolls(256);
        let space_result = CombatSystem::resolve_space(
            world,
            atk_fleet,
            def_fleet,
            sys_key,
            world.difficulty_index,
            &combat_rolls,
            current_tick,
            states.death_star.shield_generator_active,
        );

        // Apply ship damage
        apply_space_combat_result(&space_result, world);
        states.combat_cooldowns.insert(sys_key, current_tick);
        // Record battle for AI target scoring (battle repeat penalty)
        AISystem::record_battle(&mut states.ai, sys_key, current_tick);
        if let Some(ref mut ai2) = states.ai2 {
            AISystem::record_battle(ai2, sys_key, current_tick);
        }

        let winner_str = match space_result.winner {
            CombatSide::Attacker => "alliance",
            CombatSide::Defender => "empire",
            CombatSide::Draw => "draw",
        };
        events.push(GameEventRecord::new(
            current_tick,
            wall_ms,
            SYS_COMBAT,
            EVT_COMBAT_SPACE,
            serde_json::json!({
                "system": sys_name(world, sys_key),
                "winner": winner_str,
            }),
        ));

        // Ground combat after attacker wins space
        if space_result.winner == CombatSide::Attacker {
            let ground_rolls = take_rolls(256);
            let ground_result =
                CombatSystem::resolve_ground(world, sys_key, true, world.difficulty_index, &ground_rolls, current_tick);
            apply_ground_combat_result(&ground_result, world);

            if !ground_result.troop_damage.is_empty() {
                let ground_winner = match ground_result.winner {
                    CombatSide::Attacker => "alliance",
                    CombatSide::Defender => "empire",
                    CombatSide::Draw => "draw",
                };
                events.push(GameEventRecord::new(
                    current_tick,
                    wall_ms,
                    SYS_COMBAT,
                    EVT_COMBAT_GROUND,
                    serde_json::json!({
                        "system": sys_name(world, sys_key),
                        "winner": ground_winner,
                    }),
                ));
            }

            let brd_result =
                BombardmentSystem::resolve_bombardment(world, atk_fleet, sys_key, world.difficulty_index, current_tick);
            if brd_result.damage > 0 {
                events.push(GameEventRecord::new(
                    current_tick,
                    wall_ms,
                    SYS_COMBAT,
                    EVT_BOMBARDMENT,
                    serde_json::json!({
                        "system": sys_name(world, sys_key),
                        "damage": brd_result.damage,
                    }),
                ));
            }
        }
    }

    // ── 4. Fog of war ────────────────────────────────────────────────────
    let reveals = FogSystem::advance(&mut states.fog, world, &states.movement);
    integrator.emit_fog_reveals(&reveals, world);

    // ── 5. Missions ──────────────────────────────────────────────────────
    let mission_rolls = take_rolls(states.missions.len());
    let mission_results =
        MissionSystem::advance(&mut states.missions, world, tick_events, &mission_rolls);
    for result in &mission_results {
        apply_mission_effects(&result.effects, world, &mut states.uprising, &mut states.death_star);
        states.ai.mark_available(result.character);
        if let Some(ref mut ai2) = states.ai2 {
            ai2.mark_available(result.character);
        }
        events.push(GameEventRecord::new(
            result.tick,
            wall_ms,
            SYS_MISSIONS,
            EVT_MISSION_RESOLVED,
            serde_json::json!({
                "kind": format!("{:?}", result.kind),
                "outcome": format!("{:?}", result.outcome),
                "target_system": sys_name(world, result.target_system),
            }),
        ));
    }

    // ── 5b. Character escapes ────────────────────────────────────────────
    let escape_rolls = take_rolls(world.characters.len());
    let escape_effects = MissionSystem::check_escapes(world, &escape_rolls);
    for effect in &escape_effects {
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
            events.push(GameEventRecord::new(
                current_tick,
                wall_ms,
                SYS_MISSIONS,
                EVT_ESCAPE,
                serde_json::json!({
                    "character": char_name(world, *character),
                    "escaped_to_alliance": escaped_to_alliance,
                }),
            ));
        }
    }

    // ── 6. Events ────────────────────────────────────────────────────────
    let event_rolls: Vec<f32> = take_rolls(16).iter().map(|&r| r as f32).collect();
    let fired_events = EventSystem::advance(&mut states.events, world, tick_events, &event_rolls);
    for fired in &fired_events {
        apply_event_actions_to_world(&fired.actions, world, current_tick);
        events.push(GameEventRecord::new(
            fired.tick,
            wall_ms,
            SYS_EVENTS,
            EVT_EVENT_FIRED,
            serde_json::json!({ "event_id": fired.event_id }),
        ));
    }

    // Apply Jedi training from story events
    for fired in &fired_events {
        for action in &fired.actions {
            if let EventAction::StartJediTraining { character } = action {
                if let Some(c) = world.characters.get(*character) {
                    states
                        .jedi
                        .start_training(*character, c.is_alliance, current_tick);
                }
            }
        }
    }

    // ── 7. AI ────────────────────────────────────────────────────────────
    let ai_actions = AISystem::advance(
        &mut states.ai,
        world,
        &states.manufacturing,
        &states.missions,
        &states.movement,
        tick_events,
        config,
        &states.research,
    );
    let ai_rolls = take_rolls(8);
    apply_ai_actions_to_world(
        &ai_actions,
        &ai_rolls,
        &mut states.ai,
        &mut states.missions,
        &mut states.manufacturing,
        &mut states.movement,
        &mut states.research,
        world,
        current_tick,
        config,
    );
    for action in &ai_actions {
        events.push(GameEventRecord::new(
            current_tick,
            wall_ms,
            SYS_AI,
            EVT_AI_ACTION,
            ai_action_json(action, world),
        ));
    }

    // ── 7b. AI (second faction, dual-AI mode) ───────────────────────────
    if let Some(ref mut ai2) = states.ai2 {
        let ai2_actions = AISystem::advance(
            ai2,
            world,
            &states.manufacturing,
            &states.missions,
            &states.movement,
            tick_events,
            config,
            &states.research,
        );
        let ai2_rolls = take_rolls(8);
        apply_ai_actions_to_world(
            &ai2_actions,
            &ai2_rolls,
            ai2,
            &mut states.missions,
            &mut states.manufacturing,
            &mut states.movement,
            &mut states.research,
            world,
            current_tick,
            config,
        );
        for action in &ai2_actions {
            let mut payload = ai_action_json(action, world);
            if let Some(obj) = payload.as_object_mut() {
                obj.insert("dual_ai".into(), serde_json::json!(true));
            }
            events.push(GameEventRecord::new(
                current_tick,
                wall_ms,
                SYS_AI,
                EVT_AI_ACTION,
                payload,
            ));
        }
    }

    // ── 8. Blockade ──────────────────────────────────────────────────────
    let blockade_events = BlockadeSystem::advance(&mut states.blockade, world, tick_events);
    for evt in &blockade_events {
        match evt {
            BlockadeEvent::BlockadeStarted { system, tick } => {
                events.push(GameEventRecord::new(
                    *tick,
                    wall_ms,
                    SYS_BLOCKADE,
                    EVT_BLOCKADE_STARTED,
                    serde_json::json!({ "system": sys_name(world, *system) }),
                ));
            }
            BlockadeEvent::BlockadeEnded { system, tick } => {
                events.push(GameEventRecord::new(
                    *tick,
                    wall_ms,
                    SYS_BLOCKADE,
                    EVT_BLOCKADE_ENDED,
                    serde_json::json!({ "system": sys_name(world, *system) }),
                ));
            }
            BlockadeEvent::TroopDestroyed {
                system,
                troop,
                tick: _,
            } => {
                if let Some(sys) = world.systems.get_mut(*system) {
                    sys.ground_units.retain(|&k| k != *troop);
                }
                world.troops.remove(*troop);
            }
        }
    }

    // ── 9. Uprising ──────────────────────────────────────────────────────
    let uprising_rolls = take_rolls(world.systems.len());
    let empty_table = MstbTable::new(vec![]);
    let upris1tb = world
        .mission_tables
        .get("UPRIS1TB")
        .unwrap_or(&empty_table);
    let uprising_events = UprisingSystem::advance(
        &mut states.uprising,
        world,
        tick_events,
        &uprising_rolls,
        upris1tb,
    );
    for evt in &uprising_events {
        match evt {
            UprisingEvent::UprisingIncident { system, tick } => {
                events.push(GameEventRecord::new(
                    *tick,
                    wall_ms,
                    SYS_UPRISING,
                    EVT_UPRISING_INCIDENT,
                    serde_json::json!({ "system": sys_name(world, *system) }),
                ));
            }
            UprisingEvent::UprisingBegan { system, tick } => {
                // Capture before state for control_changed event
                let before = world.systems.get(*system).map(|s| s.control);
                // Flip controlling faction
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
                // Emit control_changed event
                let after = world.systems.get(*system).map(|s| s.control);
                if before != after {
                    events.push(GameEventRecord::new(
                        *tick,
                        wall_ms,
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
                events.push(GameEventRecord::new(
                    *tick,
                    wall_ms,
                    SYS_UPRISING,
                    EVT_UPRISING_BEGAN,
                    serde_json::json!({ "system": sys_name(world, *system) }),
                ));
            }
            UprisingEvent::UprisingSubdued { .. } => {
                // No event record for subdued — it's a non-event
            }
        }
    }

    // ── 10. Betrayal ─────────────────────────────────────────────────────
    let betrayal_rolls = take_rolls(world.characters.len());
    let loyalty_tb = world
        .mission_tables
        .get("UPRIS1TB")
        .unwrap_or(&empty_table);
    let betrayal_events = BetrayalSystem::advance(
        &mut states.betrayal,
        world,
        tick_events,
        &betrayal_rolls,
        loyalty_tb,
    );
    for evt in &betrayal_events {
        let BetrayalEvent::CharacterBetrayed {
            character,
            defected_to_alliance,
        } = evt;
        if let Some(c) = world.characters.get_mut(*character) {
            c.is_alliance = *defected_to_alliance;
            c.is_empire = !*defected_to_alliance;
        }
        for (_, fleet) in world.fleets.iter_mut() {
            fleet.characters.retain(|&k| k != *character);
        }
        events.push(GameEventRecord::new(
            current_tick,
            wall_ms,
            SYS_BETRAYAL,
            EVT_BETRAYAL,
            serde_json::json!({
                "character": char_name(world, *character),
                "defected_to_alliance": defected_to_alliance,
            }),
        ));
    }

    // ── 11. Death Star ───────────────────────────────────────────────────
    let ds_events = DeathStarSystem::advance(&mut states.death_star, world, tick_events);
    for evt in &ds_events {
        match evt {
            DeathStarEvent::ConstructionCompleted { system, tick } => {
                events.push(GameEventRecord::new(
                    *tick,
                    wall_ms,
                    SYS_DEATH_STAR,
                    EVT_DS_CONSTRUCTION,
                    serde_json::json!({ "system": sys_name(world, *system) }),
                ));
            }
            DeathStarEvent::PlanetDestroyed { system, tick } => {
                if let Some(sys) = world.systems.get_mut(*system) {
                    sys.is_destroyed = true;
                }
                events.push(GameEventRecord::new(
                    *tick,
                    wall_ms,
                    SYS_DEATH_STAR,
                    EVT_DS_FIRED,
                    serde_json::json!({ "system": sys_name(world, *system) }),
                ));
            }
            DeathStarEvent::NearbyWarning { .. } => {
                // Warning only — no event record
            }
        }
    }

    // ── 12. Research ─────────────────────────────────────────────────────
    let research_results = ResearchSystem::advance(&mut states.research, world, tick_events);
    for result in &research_results {
        let ResearchResult::TechUnlocked {
            faction_is_alliance,
            tech_type,
            new_level,
        } = result;
        // Apply level-ups (advance() is pure — caller must apply)
        if *faction_is_alliance {
            states.research.alliance.advance(*tech_type);
        } else {
            states.research.empire.advance(*tech_type);
        }
        events.push(GameEventRecord::new(
            current_tick,
            wall_ms,
            SYS_RESEARCH,
            EVT_RESEARCH_UNLOCKED,
            serde_json::json!({
                "faction_is_alliance": faction_is_alliance,
                "tech_type": format!("{:?}", tech_type),
                "new_level": new_level,
            }),
        ));
    }

    // ── 13. Jedi training ────────────────────────────────────────────────
    let jedi_rolls = take_rolls(states.jedi.training.len().max(1));
    let jedi_events = JediSystem::advance(&mut states.jedi, world, tick_events, &jedi_rolls);
    for evt in &jedi_events {
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
                events.push(GameEventRecord::new(
                    current_tick,
                    wall_ms,
                    SYS_JEDI,
                    EVT_JEDI_TIER,
                    serde_json::json!({
                        "character": char_name(world, *character),
                        "new_tier": format!("{:?}", new_tier),
                    }),
                ));
            }
            JediEvent::TrainingComplete { character } => {
                states.jedi.stop_training(*character);
            }
            JediEvent::JediDiscovered { character, .. } => {
                if let Some(c) = world.characters.get_mut(*character) {
                    c.is_discovered_jedi = true;
                }
                events.push(GameEventRecord::new(
                    current_tick,
                    wall_ms,
                    SYS_JEDI,
                    EVT_JEDI_DISCOVERED,
                    serde_json::json!({ "character": char_name(world, *character) }),
                ));
            }
        }
    }

    // ── 14. Victory check ────────────────────────────────────────────────
    if let Some(outcome) = VictorySystem::check(&states.victory, world, tick_events) {
        integrator.apply_victory(&outcome, &mut states.victory);
    }

    // ── 15. Campaign snapshot (every 250 ticks) ────────────────────────
    if current_tick % 250 == 0 && current_tick > 0 {
        integrator.emit_campaign_snapshot(world, states.movement.len());
    }

    // Merge integrator telemetry with inline events.
    events.extend(integrator.finish());
    events
}

// ---------------------------------------------------------------------------
// Effect application helpers (extracted from main.rs, no MessageLog)
// ---------------------------------------------------------------------------

fn apply_mission_effects(
    effects: &[MissionEffect],
    world: &mut GameWorld,
    uprising_state: &mut rebellion_core::uprising::UprisingState,
    death_star_state: &mut rebellion_core::death_star::DeathStarState,
) {
    use rebellion_core::missions::MissionFaction;

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

                    // Diplomacy-to-control: when popularity crosses 0.6 and the
                    // system is uncontrolled or controlled by the weaker faction,
                    // flip control to the dominant faction. This creates more
                    // controlled territory for diverse battle locations.
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
                for (_, fleet) in world.fleets.iter_mut() {
                    fleet.characters.retain(|&k| k != *character);
                }
                world.characters.remove(*character);
            }
            MissionEffect::CharacterCaptured {
                character,
                captured_by,
                ..
            } => {
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
                // Subdue uprising: shift popularity toward controlling faction and
                // remove the uprising from active state.
                if let Some(sys) = world.systems.get_mut(*system) {
                    // Shift toward whichever faction controls the system
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
                // Clear the uprising from active state
                uprising_state.clear_uprising(*system);
            }
            MissionEffect::DeathStarSabotaged { ticks_delayed } => {
                // Delay Death Star construction by adding ticks to countdown
                death_star_state.add_sabotage_delay(*ticks_delayed);
            }
        }
    }
}

fn apply_event_actions_to_world(actions: &[EventAction], world: &mut GameWorld, tick: u64) {
    use rebellion_core::events::SkillField;

    for action in actions {
        match action {
            EventAction::DisplayMessage { .. } => {}
            EventAction::ShiftPopularity {
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
                        SkillField::ShipDesign => {
                            c.ship_design.base = apply(c.ship_design.base, d)
                        }
                        SkillField::TroopTraining => {
                            c.troop_training.base = apply(c.troop_training.base, d)
                        }
                        SkillField::FacilityDesign => {
                            c.facility_design.base = apply(c.facility_design.base, d)
                        }
                        SkillField::Combat => c.combat.base = apply(c.combat.base, d),
                        SkillField::Leadership => {
                            c.leadership.base = apply(c.leadership.base, d)
                        }
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
            EventAction::StartJediTraining { .. } => {
                // Handled by the StartJediTraining extraction loop in the caller.
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
        }
    }
}

fn apply_ai_actions_to_world(
    actions: &[AIAction],
    rolls: &[f64],
    ai_state: &mut AIState,
    mission_state: &mut MissionState,
    mfg_state: &mut ManufacturingState,
    movement_state: &mut MovementState,
    research_state: &mut rebellion_core::research::ResearchState,
    world: &GameWorld,
    _tick: u64,
    config: &rebellion_core::tuning::GameConfig,
) {
    // Derive faction from the AIState rather than hardcoding Empire.
    let mission_faction = ai_state
        .faction
        .map(|f| f.as_mission_faction())
        .unwrap_or(rebellion_core::missions::MissionFaction::Empire);

    let mut roll_idx = 0;
    for action in actions {
        match action {
            AIAction::DispatchMission {
                kind,
                character,
                target_system,
                duration_roll,
            } => {
                let roll = rolls.get(roll_idx).copied().unwrap_or(*duration_roll);
                roll_idx += 1;
                mission_state.dispatch(
                    *kind,
                    mission_faction,
                    *character,
                    *target_system,
                    roll,
                );
                ai_state.mark_busy(*character);
            }
            AIAction::EnqueueProduction {
                system,
                kind,
                ticks,
            } => {
                mfg_state.enqueue(*system, QueueItem::new(*kind, *ticks, *ticks));
            }
            AIAction::DispatchResearch {
                character, tech_type, ticks,
            } => {
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
            AIAction::MoveFleet {
                fleet, to_system, ..
            } => {
                // Only issue a new order if the fleet isn't already in transit
                // to the same destination. Re-issuing would reset ticks_elapsed to 0.
                let already_moving = movement_state
                    .get(*fleet)
                    .map(|o| o.destination == *to_system)
                    .unwrap_or(false);
                if !already_moving {
                    if let Some(f) = world.fleets.get(*fleet) {
                        let transit =
                            rebellion_core::movement::fleet_transit_ticks_with_config(
                                f, world, f.location, *to_system,
                                config.movement.distance_scale,
                                config.movement.min_transit_ticks,
                                config.movement.default_fighter_hyperdrive,
                            );
                        movement_state.order(*fleet, f.location, *to_system, transit);
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Build completion application
// ---------------------------------------------------------------------------

/// Apply a manufacturing completion to the game world.
/// Capital ships and fighters are added to the first friendly fleet at the system
/// (or a new fleet is created). Facilities are added to the system's lists.
pub fn apply_build_completion(
    completion: &rebellion_core::manufacturing::CompletionEvent,
    world: &mut GameWorld,
) {
    use rebellion_core::manufacturing::BuildableKind;
    use rebellion_core::world::{Fleet, ShipEntry, FighterEntry};

    let sys_key = completion.system;

    match &completion.kind {
        BuildableKind::CapitalShip(class_key) => {
            // Determine faction from class
            let is_alliance = world
                .capital_ship_classes
                .get(*class_key)
                .map(|c| c.is_alliance)
                .unwrap_or(false);

            // Find existing friendly fleet at this system, or create one.
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
                // Create a new fleet at this system.
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

            // Add to existing friendly fleet or create one.
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
            // Clone the template facility's fields.
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

// ---------------------------------------------------------------------------
// Combat effect application (mirror of main.rs helpers)
// ---------------------------------------------------------------------------

fn apply_space_combat_result(
    result: &rebellion_core::combat::SpaceCombatResult,
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

fn apply_ground_combat_result(
    result: &rebellion_core::combat::GroundCombatResult,
    world: &mut GameWorld,
) {
    let mut final_strengths: HashMap<rebellion_core::ids::TroopKey, i16> = HashMap::new();
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

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::ai::AiFaction;
    use rebellion_core::dat::Faction;
    use rebellion_core::fog::FogState;

    fn make_test_states() -> SimulationStates {
        let mut world = GameWorld::default();
        // Need at least 2 systems for VictoryState
        let s1 = world.systems.insert(rebellion_core::world::System {
            dat_id: rebellion_core::ids::DatId::new(1),
            name: "System A".into(),
            sector: Default::default(),
            x: 0,
            y: 0,
            exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Uncontrolled,
        });
        let s2 = world.systems.insert(rebellion_core::world::System {
            dat_id: rebellion_core::ids::DatId::new(2),
            name: "System B".into(),
            sector: Default::default(),
            x: 100,
            y: 100,
            exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Uncontrolled,
        });
        let states = SimulationStates {
            clock: GameClock::new(),
            manufacturing: ManufacturingState::new(),
            missions: MissionState::new(),
            events: EventState::new(),
            ai: AIState::new(AiFaction::Empire),
            ai2: None,
            movement: MovementState::new(),
            fog: FogState::new(Faction::Alliance),
            blockade: BlockadeState::new(),
            uprising: UprisingState::new(),
            death_star: DeathStarState::default(),
            research: ResearchState::new(),
            jedi: JediState::new(),
            victory: VictoryState::new(s1, s2),
            betrayal: BetrayalState::new(),
            economy: EconomyState::default(),
            combat_cooldowns: HashMap::new(),
        };
        states
    }

    #[test]
    fn simulation_states_can_be_constructed() {
        let _states = make_test_states();
    }

    #[test]
    fn empty_tick_events_returns_empty() {
        let mut world = GameWorld::default();
        // Insert two systems for VictoryState
        let s1 = world.systems.insert(rebellion_core::world::System {
            dat_id: rebellion_core::ids::DatId::new(1),
            name: "A".into(),
            sector: Default::default(),
            x: 0,
            y: 0,
            exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Uncontrolled,
        });
        let s2 = world.systems.insert(rebellion_core::world::System {
            dat_id: rebellion_core::ids::DatId::new(2),
            name: "B".into(),
            sector: Default::default(),
            x: 0,
            y: 0,
            exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Uncontrolled,
        });
        let mut states = SimulationStates {
            clock: GameClock::new(),
            manufacturing: ManufacturingState::new(),
            missions: MissionState::new(),
            events: EventState::new(),
            ai: AIState::new(AiFaction::Empire),
            ai2: None,
            movement: MovementState::new(),
            fog: FogState::new(Faction::Alliance),
            blockade: BlockadeState::new(),
            uprising: UprisingState::new(),
            death_star: DeathStarState::default(),
            research: ResearchState::new(),
            jedi: JediState::new(),
            victory: VictoryState::new(s1, s2),
            betrayal: BetrayalState::new(),
            economy: EconomyState::default(),
            combat_cooldowns: HashMap::new(),
        };

        let result = run_simulation_tick(&mut world, &mut states, &[], &[], 0, &rebellion_core::tuning::GameConfig::default());
        assert!(result.is_empty());
    }
}
