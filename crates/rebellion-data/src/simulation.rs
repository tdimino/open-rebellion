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
    integrator.apply_economy_events(world, &economy_events);

    // ── 1. Manufacturing ─────────────────────────────────────────────────
    let completions = ManufacturingSystem::advance_with_blockade(
        &mut states.manufacturing,
        tick_events,
        states.blockade.blockaded_systems(),
    );
    integrator.apply_build_completions(world, &completions);

    // ── 2. Movement ──────────────────────────────────────────────────────
    let arrivals = MovementSystem::advance(&mut states.movement, tick_events);
    integrator.apply_arrivals(world, &arrivals);

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

        // Apply ship damage + telemetry via integrator
        integrator.apply_space_combat(world, sys_key, &space_result);
        states.combat_cooldowns.insert(sys_key, current_tick);
        // Record battle for AI target scoring (battle repeat penalty)
        AISystem::record_battle(&mut states.ai, sys_key, current_tick);
        if let Some(ref mut ai2) = states.ai2 {
            AISystem::record_battle(ai2, sys_key, current_tick);
        }

        // Ground combat after attacker wins space
        if space_result.winner == CombatSide::Attacker {
            let ground_rolls = take_rolls(256);
            let ground_result =
                CombatSystem::resolve_ground(world, sys_key, true, world.difficulty_index, &ground_rolls, current_tick);
            integrator.apply_ground_combat(world, &ground_result);

            let brd_result =
                BombardmentSystem::resolve_bombardment(world, atk_fleet, sys_key, world.difficulty_index, current_tick);
            integrator.emit_bombardment(world, sys_key, brd_result.damage);
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
        integrator.apply_mission_result(world, result, &mut states.uprising, &mut states.death_star);
        states.ai.mark_available(result.character);
        if let Some(ref mut ai2) = states.ai2 {
            ai2.mark_available(result.character);
        }
    }

    // ── 5b. Character escapes ────────────────────────────────────────────
    let escape_rolls = take_rolls(world.characters.len());
    let escape_effects = MissionSystem::check_escapes(world, &escape_rolls);
    integrator.apply_escape_effects(world, &escape_effects);

    // ── 6. Events ────────────────────────────────────────────────────────
    let event_rolls: Vec<f32> = take_rolls(16).iter().map(|&r| r as f32).collect();
    let fired_events = EventSystem::advance(&mut states.events, world, tick_events, &event_rolls);
    integrator.apply_fired_events(world, &fired_events, &mut states.jedi, current_tick);

    // ── 7. AI ────────────────────────────────────────────────────────────
    let ai_actions = AISystem::advance(
        &mut states.ai, world, &states.manufacturing, &states.missions,
        &states.movement, tick_events, config, &states.research,
    );
    let ai_rolls = take_rolls(8);
    integrator.apply_ai_actions(
        &ai_actions, &ai_rolls, &mut states.ai, &mut states.missions,
        &mut states.manufacturing, &mut states.movement, &mut states.research,
        world, current_tick, config, false,
    );

    // ── 7b. AI (second faction, dual-AI mode) ───────────────────────────
    if let Some(ref mut ai2) = states.ai2 {
        let ai2_actions = AISystem::advance(
            ai2, world, &states.manufacturing, &states.missions,
            &states.movement, tick_events, config, &states.research,
        );
        let ai2_rolls = take_rolls(8);
        integrator.apply_ai_actions(
            &ai2_actions, &ai2_rolls, ai2, &mut states.missions,
            &mut states.manufacturing, &mut states.movement, &mut states.research,
            world, current_tick, config, true,
        );
    }

    // ── 8. Blockade ──────────────────────────────────────────────────────
    let blockade_events = BlockadeSystem::advance(&mut states.blockade, world, tick_events);
    integrator.apply_blockade_events(world, &blockade_events);

    // ── 9. Uprising ──────────────────────────────────────────────────────
    let uprising_rolls = take_rolls(world.systems.len());
    let empty_table = MstbTable::new(vec![]);
    let upris1tb = world.mission_tables.get("UPRIS1TB").unwrap_or(&empty_table);
    let uprising_events = UprisingSystem::advance(
        &mut states.uprising, world, tick_events, &uprising_rolls, upris1tb,
    );
    integrator.apply_uprising_events(world, &uprising_events);

    // ── 10. Betrayal ─────────────────────────────────────────────────────
    let betrayal_rolls = take_rolls(world.characters.len());
    let loyalty_tb = world.mission_tables.get("UPRIS1TB").unwrap_or(&empty_table);
    let betrayal_events = BetrayalSystem::advance(
        &mut states.betrayal, world, tick_events, &betrayal_rolls, loyalty_tb,
    );
    integrator.apply_betrayal_events(world, &betrayal_events);

    // ── 11. Death Star ───────────────────────────────────────────────────
    let ds_events = DeathStarSystem::advance(&mut states.death_star, world, tick_events);
    integrator.apply_death_star_events(world, &ds_events);

    // ── 12. Research ─────────────────────────────────────────────────────
    let research_results = ResearchSystem::advance(&mut states.research, world, tick_events);
    integrator.apply_research_results(&research_results, &mut states.research);

    // ── 13. Jedi training ────────────────────────────────────────────────
    let jedi_rolls = take_rolls(states.jedi.training.len().max(1));
    let jedi_events = JediSystem::advance(&mut states.jedi, world, tick_events, &jedi_rolls);
    integrator.apply_jedi_events(world, &jedi_events, &mut states.jedi);

    // ── 14. Victory check ────────────────────────────────────────────────
    if let Some(outcome) = VictorySystem::check(&states.victory, world, tick_events) {
        integrator.apply_victory(&outcome, &mut states.victory);
    }

    // ── 15. Campaign snapshot (every 250 ticks) ────────────────────────
    if current_tick % 250 == 0 && current_tick > 0 {
        integrator.emit_campaign_snapshot(world, states.movement.len());
    }

    integrator.finish()
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
