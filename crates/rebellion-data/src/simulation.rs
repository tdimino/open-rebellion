//! Shared simulation tick function for headless and interactive use.
//!
//! Extracts the per-tick system-advance logic from `rebellion-app/main.rs`
//! into a reusable function that returns structured `GameEventRecord`s
//! instead of pushing to a `MessageLog`.

use std::collections::HashMap;

use rebellion_core::ai::{AIAction, AIState, AISystem};
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
use rebellion_core::world::{GameWorld, MstbTable};

// ---------------------------------------------------------------------------
// Payload helpers — resolve slotmap keys to human-readable names
// ---------------------------------------------------------------------------

/// Resolve a SystemKey to the system's name, or a fallback string.
fn sys_name(world: &GameWorld, key: SystemKey) -> String {
    world
        .systems
        .get(key)
        .map(|s| s.name.clone())
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

/// Resolve a CharacterKey to the character's name.
fn char_name(world: &GameWorld, key: rebellion_core::ids::CharacterKey) -> String {
    world
        .characters
        .get(key)
        .map(|c| c.name.clone())
        .unwrap_or_else(|| format!("{:?}", key))
}

/// Format an AIAction as a structured JSON payload with readable names.
fn ai_action_json(action: &AIAction, world: &GameWorld) -> serde_json::Value {
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
    }
}

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
    pub combat_cooldowns: HashMap<SystemKey, u64>,
}

/// Run one simulation tick across all 15 systems.
///
/// Advances each system in the canonical order (manufacturing → movement →
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
) -> Vec<GameEventRecord> {
    if tick_events.is_empty() {
        return Vec::new();
    }

    let mut events = Vec::new();
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

    // ── 1. Manufacturing ─────────────────────────────────────────────────
    let completions = ManufacturingSystem::advance_with_blockade(
        &mut states.manufacturing,
        tick_events,
        states.blockade.blockaded_systems(),
    );
    for c in &completions {
        events.push(GameEventRecord::new(
            c.tick,
            wall_ms,
            SYS_MANUFACTURING,
            EVT_BUILD_COMPLETE,
            serde_json::json!({ "system": sys_name(world, c.system) }),
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
            2,
            &combat_rolls,
            current_tick,
        );

        // Apply ship damage
        apply_space_combat_result(&space_result, world);
        states.combat_cooldowns.insert(sys_key, current_tick);

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
                CombatSystem::resolve_ground(world, sys_key, true, &ground_rolls, current_tick);
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
                BombardmentSystem::resolve_bombardment(world, atk_fleet, sys_key, 2, current_tick);
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
    for reveal in &reveals {
        events.push(GameEventRecord::new(
            current_tick,
            wall_ms,
            SYS_FOG,
            EVT_FOG_REVEALED,
            serde_json::json!({ "system": sys_name(world, reveal.system) }),
        ));
    }

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
        tick_events,
    );
    let ai_rolls = take_rolls(8);
    apply_ai_actions_to_world(
        &ai_actions,
        &ai_rolls,
        &mut states.ai,
        &mut states.missions,
        &mut states.manufacturing,
        &mut states.movement,
        world,
        current_tick,
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
            tick_events,
        );
        let ai2_rolls = take_rolls(8);
        apply_ai_actions_to_world(
            &ai2_actions,
            &ai2_rolls,
            ai2,
            &mut states.missions,
            &mut states.manufacturing,
            &mut states.movement,
            world,
            current_tick,
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
                // Flip controlling faction
                if let Some(sys) = world.systems.get_mut(*system) {
                    sys.controlling_faction = match sys.controlling_faction {
                        Some(rebellion_core::dat::Faction::Alliance) => {
                            Some(rebellion_core::dat::Faction::Empire)
                        }
                        Some(rebellion_core::dat::Faction::Empire) => {
                            Some(rebellion_core::dat::Faction::Alliance)
                        }
                        other => other,
                    };
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
        states.victory.resolved = true;
        events.push(GameEventRecord::new(
            current_tick,
            wall_ms,
            SYS_VICTORY,
            EVT_VICTORY,
            serde_json::json!({ "outcome": format!("{:?}", outcome) }),
        ));
    }

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
                    match sys.controlling_faction {
                        Some(rebellion_core::dat::Faction::Alliance) => {
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
    world: &GameWorld,
    _tick: u64,
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
                            rebellion_core::movement::fleet_transit_ticks(f, world, f.location, *to_system);
                        movement_state.order(*fleet, f.location, *to_system, transit);
                    }
                }
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
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            controlling_faction: None,
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
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            controlling_faction: None,
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
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            controlling_faction: None,
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
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            controlling_faction: None,
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
            combat_cooldowns: HashMap::new(),
        };

        let result = run_simulation_tick(&mut world, &mut states, &[], &[], 0);
        assert!(result.is_empty());
    }
}
