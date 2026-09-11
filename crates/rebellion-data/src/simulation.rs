//! Shared simulation tick function for headless and interactive use.
//!
//! Extracts the per-tick system-advance logic from `rebellion-app/main.rs`
//! into a reusable function that returns structured `GameEventRecord`s
//! instead of pushing to a `MessageLog`.

use std::collections::HashMap;

use crate::integrator::{
    apply_ground_combat_result_inner, resolve_system_space_combat, PerceptionIntegrator,
};
use rebellion_core::ai::{AIState, AISystem};
use rebellion_core::betrayal::{BetrayalState, BetrayalSystem};
use rebellion_core::blockade::{BlockadeState, BlockadeSystem};
use rebellion_core::bombardment::BombardmentSystem;
use rebellion_core::combat::{CombatSide, CombatSystem};
use rebellion_core::dat::Faction;
use rebellion_core::death_star::{DeathStarState, DeathStarSystem};
use rebellion_core::economy::{EconomyState, EconomySystem};
use rebellion_core::events::{EventState, EventSystem};
use rebellion_core::fog::{FogState, FogSystem};
use rebellion_core::game_events::GameEventRecord;
use rebellion_core::ids::SystemKey;
use rebellion_core::jedi::{JediState, JediSystem};
use rebellion_core::manufacturing::{ManufacturingState, ManufacturingSystem};
use rebellion_core::missions::{MissionState, MissionSystem};
use rebellion_core::movement::{reconcile_fleet_orbits, MovementState, MovementSystem};
use rebellion_core::repair::{RepairState, RepairSystem};
use rebellion_core::research::{ResearchState, ResearchSystem};
use rebellion_core::tick::{GameClock, TickEvent};
use rebellion_core::troop_transport::TroopTransportState;
use rebellion_core::uprising::{UprisingState, UprisingSystem};
use rebellion_core::victory::{VictoryState, VictorySystem};
use rebellion_core::world::{CampaignConfig, GameWorld, MstbTable};

const COMBAT_RETRY_TICKS: u64 = 5;
const MAX_SYSTEM_GROUND_COMBAT_ROUNDS: u32 = 256;
const UNCHANGED_STALEMATE_COOLDOWN: u64 = u64::MAX;

fn combat_is_on_cooldown(last_battle: u64, current_tick: u64) -> bool {
    last_battle == UNCHANGED_STALEMATE_COOLDOWN
        || current_tick < last_battle.saturating_add(COMBAT_RETRY_TICKS)
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
    pub economy: EconomyState,
    pub repair: RepairState,
    pub troop_transport: TroopTransportState,
    pub combat_cooldowns: HashMap<SystemKey, u64>,
    /// Original new-game choices that continue to govern this campaign.
    pub campaign_config: CampaignConfig,
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

    let mut integrator = PerceptionIntegrator::new(tick_events.last().unwrap().tick, wall_ms);
    let mut roll_cursor = 0usize;
    let current_tick = tick_events.last().unwrap().tick;

    // `MovementState` is authoritative while a fleet is in hyperspace. Repair
    // stale save/index state before economy and manufacturing inspect orbits.
    reconcile_fleet_orbits(&states.movement, world);

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
    // Use advance_tracked so the K6 EVT_MANUFACTURING_IDLE telemetry is
    // emitted for any system whose queue transitioned empty this tick.
    let mfg_advance = ManufacturingSystem::advance_tracked(
        &mut states.manufacturing,
        tick_events,
        states.blockade.blockaded_systems(),
    );
    integrator.apply_build_completions(world, &mfg_advance.completions);
    integrator.apply_manufacturing_idle(world, &mfg_advance.newly_idle);
    for completion in &mfg_advance.completions {
        states.combat_cooldowns.remove(&completion.system);
    }

    // ── 2. Movement ──────────────────────────────────────────────────────
    let arrivals = MovementSystem::advance(&mut states.movement, tick_events);
    integrator.apply_arrivals(world, &mut states.troop_transport, &arrivals);
    for arrival in &arrivals {
        states.combat_cooldowns.remove(&arrival.system);
    }

    // ── 3. Combat ────────────────────────────────────────────────────────
    let combat_triggers: Vec<_> = world
        .systems
        .keys()
        .filter_map(|sys_key| {
            if let Some(&last_battle) = states.combat_cooldowns.get(&sys_key) {
                if combat_is_on_cooldown(last_battle, current_tick) {
                    return None;
                }
            }
            let sys = &world.systems[sys_key];
            let has_alliance = sys
                .fleets
                .iter()
                .copied()
                .any(|k| world.fleets.get(k).map(|f| f.is_alliance).unwrap_or(false));
            let has_empire = sys
                .fleets
                .iter()
                .copied()
                .any(|k| world.fleets.get(k).map(|f| !f.is_alliance).unwrap_or(false));
            (has_alliance && has_empire).then_some(sys_key)
        })
        .collect();

    for sys_key in combat_triggers {
        let combat_rolls = take_rolls(256);
        let space_result = resolve_system_space_combat(
            world,
            sys_key,
            world.difficulty_index,
            &combat_rolls,
            current_tick,
            states.death_star.shield_generator_active,
        );

        integrator.emit_system_space_combat(world, &space_result);
        states.combat_cooldowns.insert(
            sys_key,
            if space_result.stalemate {
                UNCHANGED_STALEMATE_COOLDOWN
            } else {
                current_tick
            },
        );
        // Record battle for AI target scoring (battle repeat penalty)
        AISystem::record_battle(&mut states.ai, sys_key, current_tick);
        if let Some(ref mut ai2) = states.ai2 {
            AISystem::record_battle(ai2, sys_key, current_tick);
        }

        // Orbital bombardment follows a decisive space victory. Ground
        // assault is resolved below after surviving transports have landed.
        // Alliance is always coded as attacker, Empire as defender in the trigger above.
        // The space combat winner gets to follow up with ground assault + orbital bombardment.
        let winner_info = match (space_result.winner, space_result.winner_fleet) {
            (CombatSide::Attacker, Some(fleet)) => Some((fleet, true)), // Alliance won
            (CombatSide::Defender, Some(fleet)) => Some((fleet, false)), // Empire won
            (CombatSide::Draw, _) => None,
            _ => None,
        };
        if let Some((winner_fleet, winner_is_alliance)) = winner_info {
            let brd_result = BombardmentSystem::resolve_bombardment(
                world,
                winner_fleet,
                sys_key,
                world.difficulty_index,
                current_tick,
            );
            let attacker = if winner_is_alliance {
                Faction::Alliance
            } else {
                Faction::Empire
            };
            let headquarters_destroyed = VictorySystem::apply_headquarters_bombardment(
                &states.victory,
                world,
                &brd_result,
                attacker,
            );
            integrator.emit_bombardment(world, sys_key, brd_result.damage, headquarters_destroyed);
        }
    }

    let destroyed_cargo = states.troop_transport.destroy_untransportable_cargo(world);
    integrator.emit_destroyed_transport_cargo(&destroyed_cargo);

    // ── 3b. Troop landing, ground combat, and occupation ────────────────
    // A fleet may land cargo only after it is the sole orbital faction. This
    // also covers unopposed invasions, which do not create a space-combat
    // trigger. Surface combat continues on later ticks until it is decisive.
    let ground_systems: Vec<_> = world.systems.keys().collect();
    for sys_key in ground_systems {
        let orbiting: Vec<_> = world
            .systems
            .get(sys_key)
            .map(|system| system.fleets.clone())
            .unwrap_or_default();
        let has_alliance_fleet = orbiting.iter().any(|fleet| {
            world
                .fleets
                .get(*fleet)
                .map(|value| value.is_alliance)
                .unwrap_or(false)
        });
        let has_empire_fleet = orbiting.iter().any(|fleet| {
            world
                .fleets
                .get(*fleet)
                .map(|value| !value.is_alliance)
                .unwrap_or(false)
        });
        let orbital_winner = match (has_alliance_fleet, has_empire_fleet) {
            (true, false) => Some(Faction::Alliance),
            (false, true) => Some(Faction::Empire),
            _ => None,
        };

        if let Some(faction) = orbital_winner {
            let landing_fleets: Vec<_> = orbiting
                .iter()
                .copied()
                .filter(|fleet| {
                    world
                        .fleets
                        .get(*fleet)
                        .map(|value| value.is_alliance == (faction == Faction::Alliance))
                        .unwrap_or(false)
                })
                .collect();

            // An unopposed Imperial invasion of the mobile Alliance HQ has no
            // space battle to trigger the normal bombardment follow-up. Strike
            // the HQ before landing, but only when a transport is actually
            // carrying an invasion force. Occupation alone is not sufficient.
            if faction == Faction::Empire
                && sys_key == states.victory.alliance_hq
                && world
                    .systems
                    .get(sys_key)
                    .is_some_and(|system| system.is_headquarters && !system.is_destroyed)
            {
                if let Some(bombardment_fleet) = landing_fleets
                    .iter()
                    .copied()
                    .find(|fleet| states.troop_transport.carried_count(*fleet) > 0)
                {
                    let result = BombardmentSystem::resolve_bombardment(
                        world,
                        bombardment_fleet,
                        sys_key,
                        world.difficulty_index,
                        current_tick,
                    );
                    let headquarters_destroyed = VictorySystem::apply_headquarters_bombardment(
                        &states.victory,
                        world,
                        &result,
                        Faction::Empire,
                    );
                    integrator.emit_bombardment(
                        world,
                        sys_key,
                        result.damage,
                        headquarters_destroyed,
                    );
                }
            }

            for fleet in landing_fleets {
                integrator.apply_troop_landing(world, &mut states.troop_transport, fleet, sys_key);
            }
        }

        let (alliance_troops, empire_troops) = world
            .systems
            .get(sys_key)
            .map(|system| {
                system
                    .ground_units
                    .iter()
                    .fold((0_usize, 0_usize), |counts, troop| {
                        match world.troops.get(*troop) {
                            Some(value) if value.regiment_strength > 0 && value.is_alliance => {
                                (counts.0 + 1, counts.1)
                            }
                            Some(value) if value.regiment_strength > 0 => (counts.0, counts.1 + 1),
                            _ => counts,
                        }
                    })
            })
            .unwrap_or_default();

        let occupying_faction = match (alliance_troops > 0, empire_troops > 0) {
            (true, true) => {
                let attacker_is_alliance = orbital_winner
                    .map(|faction| faction == Faction::Alliance)
                    .unwrap_or(true);
                let ground_rolls = take_rolls(256);
                let mut final_winner = CombatSide::Draw;
                let mut total_engagements = 0;
                let mut rounds = 0;
                let mut stalemate = false;
                while rounds < MAX_SYSTEM_GROUND_COMBAT_ROUNDS {
                    let result = CombatSystem::resolve_ground(
                        world,
                        sys_key,
                        attacker_is_alliance,
                        world.difficulty_index,
                        &ground_rolls,
                        current_tick,
                    );
                    let made_progress = result
                        .troop_damage
                        .iter()
                        .any(|event| event.strength_after < event.strength_before);
                    final_winner = result.winner;
                    total_engagements += result.troop_damage.len();
                    rounds += 1;
                    apply_ground_combat_result_inner(&result, world);
                    if final_winner != CombatSide::Draw || !made_progress {
                        stalemate = final_winner == CombatSide::Draw;
                        break;
                    }
                }
                if rounds == MAX_SYSTEM_GROUND_COMBAT_ROUNDS && final_winner == CombatSide::Draw {
                    stalemate = true;
                }
                integrator.emit_system_ground_combat(
                    world,
                    sys_key,
                    final_winner,
                    total_engagements,
                    rounds,
                    stalemate,
                );
                match final_winner {
                    CombatSide::Attacker => Some(if attacker_is_alliance {
                        Faction::Alliance
                    } else {
                        Faction::Empire
                    }),
                    CombatSide::Defender => Some(if attacker_is_alliance {
                        Faction::Empire
                    } else {
                        Faction::Alliance
                    }),
                    CombatSide::Draw => None,
                }
            }
            (true, false) => Some(Faction::Alliance),
            (false, true) => Some(Faction::Empire),
            (false, false) => None,
        };

        if let Some(winner) = occupying_faction {
            let control = world.systems.get(sys_key).map(|system| system.control);
            if control != Some(rebellion_core::world::ControlKind::Controlled(winner)) {
                integrator.apply_ground_occupation(world, sys_key, winner, current_tick);
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
        integrator.apply_mission_result(
            world,
            result,
            &mut states.uprising,
            &mut states.death_star,
        );
        states.ai.mark_available(result.character);
        if let Some(ref mut ai2) = states.ai2 {
            ai2.mark_available(result.character);
        }
        // Knesset Shamash-Bet #R11: emit `EVT_CHARACTER_KILLED` telemetry for
        // mission-side assassinations. The integrator's `MissionEffect::CharacterKilled`
        // arm marks `is_killed = true` via `mark_killed()` instead of deleting
        // from the arena, so the lookup below ALWAYS succeeds under correct
        // invariants — the `<unknown>` fallback is a real invariant break and
        // panics in debug builds.
        for effect in &result.effects {
            if let rebellion_core::missions::MissionEffect::CharacterKilled { character, .. } =
                effect
            {
                debug_assert!(
                    world.characters.contains_key(*character),
                    "EVT_CHARACTER_KILLED target missing from arena — R11 invariant break"
                );
                let (name, dat_id) = world
                    .characters
                    .get(*character)
                    .map(|c| (c.name.clone(), c.dat_id.raw()))
                    .unwrap_or_else(|| (String::from("<unknown>"), 0));
                integrator.emit(
                    rebellion_core::game_events::SYS_MISSIONS,
                    rebellion_core::game_events::EVT_CHARACTER_KILLED,
                    serde_json::json!({
                        "name": name,
                        "dat_id": dat_id,
                        "cause": "assassination",
                    }),
                );
            }
        }

        // Knesset Shamash-Bet #R6/#R7/#R8: mission-side notification events.
        //
        // These are state-transition telemetry markers on the outcome of the
        // mission, NOT `Random`-gated EventSystem entries. The CI guard
        // `notification_events_never_use_random` protects the corresponding
        // story-event IDs from being misused.
        match result.outcome {
            rebellion_core::missions::MissionOutcome::Success => {
                // #R6: Successful espionage yields informant intel.
                if matches!(
                    result.kind,
                    rebellion_core::missions::MissionKind::Espionage
                ) {
                    let char_name = world
                        .characters
                        .get(result.character)
                        .map(|c| c.name.clone())
                        .unwrap_or_else(|| String::from("<unknown>"));
                    let sys_name = world
                        .systems
                        .get(result.target_system)
                        .map(|s| s.name.clone())
                        .unwrap_or_else(|| String::from("<unknown>"));
                    integrator.emit(
                        rebellion_core::game_events::SYS_MISSIONS,
                        rebellion_core::game_events::EVT_INFORMANT_INTEL,
                        serde_json::json!({
                            "character": char_name,
                            "system": sys_name,
                        }),
                    );
                }
            }
            rebellion_core::missions::MissionOutcome::Foiled => {
                // #R7: Counter-intelligence foiled a covert mission → saboteur detected.
                let char_name = world
                    .characters
                    .get(result.character)
                    .map(|c| c.name.clone())
                    .unwrap_or_else(|| String::from("<unknown>"));
                integrator.emit(
                    rebellion_core::game_events::SYS_MISSIONS,
                    rebellion_core::game_events::EVT_SABOTEUR_DETECTED,
                    serde_json::json!({
                        "character": char_name,
                        "mission_kind": format!("{:?}", result.kind),
                    }),
                );
            }
            rebellion_core::missions::MissionOutcome::Failure => {
                // #R8: Mission failure on combat/assassination → character health hit.
                if matches!(
                    result.kind,
                    rebellion_core::missions::MissionKind::Assassination
                        | rebellion_core::missions::MissionKind::Abduction
                        | rebellion_core::missions::MissionKind::Rescue
                ) {
                    let char_name = world
                        .characters
                        .get(result.character)
                        .map(|c| c.name.clone())
                        .unwrap_or_else(|| String::from("<unknown>"));
                    integrator.emit(
                        rebellion_core::game_events::SYS_MISSIONS,
                        rebellion_core::game_events::EVT_CHARACTER_HEALTH,
                        serde_json::json!({
                            "character": char_name,
                            "mission_kind": format!("{:?}", result.kind),
                        }),
                    );
                }
            }
        }
    }

    // ── 5b. Character escapes ────────────────────────────────────────────
    let escape_rolls = take_rolls(world.characters.len());
    let escape_effects = MissionSystem::check_escapes(world, &escape_rolls);
    integrator.apply_escape_effects(world, &escape_effects);

    // ── 6. Events ────────────────────────────────────────────────────────
    let event_rolls: Vec<f32> = take_rolls(16).iter().map(|&r| r as f32).collect();
    let fired_events = EventSystem::advance(&mut states.events, world, tick_events, &event_rolls);
    integrator.apply_fired_events(
        world,
        &fired_events,
        &mut states.jedi,
        current_tick,
        &states.movement,
    );

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
    integrator.apply_ai_actions(
        &ai_actions,
        &ai_rolls,
        &mut states.ai,
        &mut states.missions,
        &mut states.manufacturing,
        &mut states.movement,
        &mut states.troop_transport,
        &mut states.research,
        world,
        current_tick,
        config,
        false,
    );

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
        integrator.apply_ai_actions(
            &ai2_actions,
            &ai2_rolls,
            ai2,
            &mut states.missions,
            &mut states.manufacturing,
            &mut states.movement,
            &mut states.troop_transport,
            &mut states.research,
            world,
            current_tick,
            config,
            true,
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
        &mut states.uprising,
        world,
        tick_events,
        &uprising_rolls,
        upris1tb,
    );
    integrator.apply_uprising_events(world, &uprising_events);

    // ── 10. Betrayal ─────────────────────────────────────────────────────
    let betrayal_rolls = take_rolls(world.characters.len());
    let loyalty_tb = world.mission_tables.get("UPRIS1TB").unwrap_or(&empty_table);
    let betrayal_events = BetrayalSystem::advance(
        &mut states.betrayal,
        world,
        tick_events,
        &betrayal_rolls,
        loyalty_tb,
    );
    integrator.apply_betrayal_events(world, &betrayal_events);

    // ── 11. Death Star ───────────────────────────────────────────────────
    let ds_events = DeathStarSystem::advance(&mut states.death_star, world, tick_events);
    integrator.apply_death_star_events(world, &ds_events);
    // Update victory state and clean up destroyed systems.
    //
    // Knesset Shamash-Bet #R11: `cleanup_destroyed_system` now drains an
    // out-parameter `Vec<GameEffect>` — we turn each `GameEffect::CharacterKilled`
    // into an `EVT_CHARACTER_KILLED` telemetry record (DI-H2 payload uses
    // `name` / `dat_id` rather than `CharacterKey`). Character-death story
    // events are strictly next-tick reactive.
    for evt in &ds_events {
        if let rebellion_core::death_star::DeathStarEvent::PlanetDestroyed { system, .. } = evt {
            states.victory.death_star_location = Some(*system);
            let mut cleanup_effects: Vec<rebellion_core::effects::GameEffect> = Vec::new();
            rebellion_core::death_star::cleanup_destroyed_system(
                world,
                *system,
                &mut states.movement,
                &mut states.death_star,
                &mut states.manufacturing,
                &mut states.blockade,
                &mut cleanup_effects,
            );
            let dest_sys_name = world
                .systems
                .get(*system)
                .map(|s| s.name.clone())
                .unwrap_or_else(|| String::from("<unknown>"));
            for effect in cleanup_effects.drain(..) {
                if let rebellion_core::effects::GameEffect::CharacterKilled { character } = effect {
                    let (name, dat_id) = world
                        .characters
                        .get(character)
                        .map(|c| (c.name.clone(), c.dat_id.raw()))
                        .unwrap_or_else(|| (String::from("<unknown>"), 0));
                    integrator.emit(
                        rebellion_core::game_events::SYS_MISSIONS,
                        rebellion_core::game_events::EVT_CHARACTER_KILLED,
                        serde_json::json!({
                            "name": name,
                            "dat_id": dat_id,
                            "cause": "death_star",
                            "system": dest_sys_name,
                        }),
                    );
                }
            }
        }
    }

    // ── 12. Research ─────────────────────────────────────────────────────
    let research_results = ResearchSystem::advance(&mut states.research, world, tick_events);
    integrator.apply_research_results(&research_results, &mut states.research);

    // ── 12b. Ship repair ────────────────────────────────────────────────
    let repair_events = RepairSystem::advance(&mut states.repair, world, tick_events);
    integrator.apply_repair_events(world, &repair_events);

    // ── 13. Jedi training ────────────────────────────────────────────────
    let jedi_rolls = take_rolls(states.jedi.training.len().max(1));
    let jedi_events = JediSystem::advance(&mut states.jedi, world, tick_events, &jedi_rolls);
    integrator.apply_jedi_events(world, &jedi_events, &mut states.jedi);

    // ── 14. Victory check ────────────────────────────────────────────────
    integrator.emit_victory_check(&states.victory);
    if let Some(outcome) = VictorySystem::check(
        &states.victory,
        world,
        tick_events,
        states.campaign_config.victory_conditions,
    ) {
        integrator.apply_victory(&outcome, &mut states.victory, world);
    }

    // ── 15. Campaign snapshot (every 250 ticks) ────────────────────────
    if current_tick % 250 == 0 && current_tick > 0 {
        integrator.emit_campaign_snapshot(world, states.movement.len(), &states.economy);
    }

    integrator.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::ai::AiFaction;
    use rebellion_core::dat::Faction;
    use rebellion_core::fog::FogState;
    use rebellion_core::game_events::{
        EVT_BOMBARDMENT, EVT_CAPTURE, EVT_COMBAT_GROUND, EVT_CONTROL_CHANGED, EVT_FLEET_ARRIVED,
        EVT_TROOP_MOVED, EVT_VICTORY,
    };
    use rebellion_core::ids::DatId;
    use rebellion_core::movement::begin_fleet_transit;
    use rebellion_core::world::{
        CapitalShipClass, Character, ControlKind, Fleet, ShipInstance, System, TroopClassDef,
        TroopUnit, VictoryConditions,
    };

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
            espionage_rating: 0.0,
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
            espionage_rating: 0.0,
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
            repair: RepairState::default(),
            troop_transport: TroopTransportState::default(),
            combat_cooldowns: HashMap::new(),
            campaign_config: CampaignConfig::default(),
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
            espionage_rating: 0.0,
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
            espionage_rating: 0.0,
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
            repair: RepairState::default(),
            troop_transport: TroopTransportState::default(),
            combat_cooldowns: HashMap::new(),
            campaign_config: CampaignConfig::default(),
        };

        let result = run_simulation_tick(
            &mut world,
            &mut states,
            &[],
            &[],
            0,
            &rebellion_core::tuning::GameConfig::default(),
        );
        assert!(result.is_empty());
    }

    #[test]
    fn unchanged_stalemate_does_not_reopen_on_five_tick_cadence() {
        assert!(combat_is_on_cooldown(UNCHANGED_STALEMATE_COOLDOWN, 5));
        assert!(combat_is_on_cooldown(UNCHANGED_STALEMATE_COOLDOWN, 50_000));
        assert!(combat_is_on_cooldown(10, 14));
        assert!(!combat_is_on_cooldown(10, 15));
    }

    #[test]
    fn surface_battle_continues_on_later_tick_after_transport_empties() {
        let mut world = GameWorld::default();
        let make_system = |dat_id, name: &str, control| System {
            dat_id: DatId::new(dat_id),
            name: name.into(),
            sector: Default::default(),
            x: dat_id as u16 * 100,
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
            control,
            espionage_rating: 0.0,
        };
        let target = world.systems.insert(make_system(
            1,
            "Contested Surface",
            ControlKind::Controlled(Faction::Empire),
        ));
        let other = world.systems.insert(make_system(
            2,
            "Other System",
            ControlKind::Controlled(Faction::Alliance),
        ));
        let fleet = world.fleets.insert(Fleet {
            location: target,
            capital_ships: vec![],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        world.systems[target].fleets.push(fleet);
        let troop_class = DatId::new(20);
        world.troop_classes.insert(
            troop_class,
            TroopClassDef {
                attack_strength: 0,
                defense_strength: 10,
            },
        );
        let alliance_troop = world.troops.insert(TroopUnit {
            class_dat_id: troop_class,
            is_alliance: true,
            regiment_strength: 1_000,
        });
        let empire_troop = world.troops.insert(TroopUnit {
            class_dat_id: troop_class,
            is_alliance: false,
            regiment_strength: 1_000,
        });
        world.systems[target]
            .ground_units
            .extend([alliance_troop, empire_troop]);

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
            victory: VictoryState::new(other, target),
            betrayal: BetrayalState::new(),
            economy: EconomyState::default(),
            repair: RepairState::default(),
            troop_transport: TroopTransportState::default(),
            combat_cooldowns: HashMap::new(),
            campaign_config: CampaignConfig::default(),
        };
        let config = rebellion_core::tuning::GameConfig::default();

        let first_events = run_simulation_tick(
            &mut world,
            &mut states,
            &[TickEvent { tick: 1 }],
            &[0.99; 2_048],
            10,
            &config,
        );
        let first_strength = world.troops[alliance_troop].regiment_strength;
        assert_eq!(first_strength, 744);
        assert!(states.troop_transport.is_empty());
        assert!(first_events
            .iter()
            .any(|event| event.event_type == EVT_COMBAT_GROUND));

        let second_events = run_simulation_tick(
            &mut world,
            &mut states,
            &[TickEvent { tick: 2 }],
            &[0.99; 2_048],
            20,
            &config,
        );

        assert!(world.troops[alliance_troop].regiment_strength < first_strength);
        assert!(world.troops.contains_key(empire_troop));
        assert!(states.troop_transport.is_empty());
        assert!(second_events
            .iter()
            .any(|event| event.event_type == EVT_COMBAT_GROUND));
        assert_ne!(
            world.systems[target].control,
            ControlKind::Controlled(Faction::Alliance)
        );
    }

    #[test]
    fn decisive_imperial_bombardment_destroys_the_alliance_hq_facility() {
        let mut world = GameWorld::default();
        let make_system = |dat_id, name: &str, control, is_headquarters| System {
            dat_id: DatId::new(dat_id),
            name: name.into(),
            sector: Default::default(),
            x: dat_id as u16 * 100,
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
            is_headquarters,
            is_destroyed: false,
            control,
            espionage_rating: 0.0,
        };
        let alliance_hq = world.systems.insert(make_system(
            1,
            "Alliance HQ",
            ControlKind::Controlled(Faction::Alliance),
            true,
        ));
        let coruscant = world.systems.insert(make_system(
            2,
            "Coruscant",
            ControlKind::Controlled(Faction::Empire),
            true,
        ));
        let alliance_class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(61),
            name: "Alliance Scout".into(),
            is_alliance: true,
            is_empire: false,
            hull: 1,
            ..CapitalShipClass::default()
        });
        let empire_class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(62),
            name: "Imperial Bombardment Ship".into(),
            is_alliance: false,
            is_empire: true,
            hull: 1_000,
            turbolaser_fore: 1_000,
            bombardment_modifier: 100,
            ..CapitalShipClass::default()
        });
        let alliance_fleet = world.fleets.insert(Fleet {
            location: alliance_hq,
            capital_ships: vec![ShipInstance::new(alliance_class, 1, true)],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        let empire_fleet = world.fleets.insert(Fleet {
            location: alliance_hq,
            capital_ships: vec![ShipInstance::new(empire_class, 1_000, false)],
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            has_death_star: false,
        });
        world.systems[alliance_hq]
            .fleets
            .extend([alliance_fleet, empire_fleet]);

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
            victory: VictoryState::new(alliance_hq, coruscant),
            betrayal: BetrayalState::new(),
            economy: EconomyState::default(),
            repair: RepairState::default(),
            troop_transport: TroopTransportState::default(),
            combat_cooldowns: HashMap::new(),
            campaign_config: CampaignConfig {
                victory_conditions: VictoryConditions::HeadquartersOnly,
                ..CampaignConfig::default()
            },
        };

        let events = run_simulation_tick(
            &mut world,
            &mut states,
            &[TickEvent { tick: 1 }],
            &[0.5; 2_048],
            10,
            &rebellion_core::tuning::GameConfig::default(),
        );

        assert!(!world.systems[alliance_hq].is_headquarters);
        assert!(
            !states.victory.resolved,
            "bombardment still requires occupation"
        );
        let bombardment = events
            .iter()
            .find(|event| event.event_type == EVT_BOMBARDMENT)
            .expect("bombardment telemetry");
        assert_eq!(bombardment.details["headquarters_destroyed"], true);
    }

    #[test]
    fn unopposed_imperial_invasion_bombards_before_occupying_alliance_hq() {
        let mut world = GameWorld::default();
        let add_system = |world: &mut GameWorld, dat_id, name: &str, control, is_headquarters| {
            world.systems.insert(System {
                dat_id: DatId::new(dat_id),
                name: name.into(),
                sector: Default::default(),
                x: dat_id as u16 * 100,
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
                is_headquarters,
                is_destroyed: false,
                control,
                espionage_rating: 0.0,
            })
        };
        let alliance_hq = add_system(
            &mut world,
            1,
            "Alliance HQ",
            ControlKind::Controlled(Faction::Alliance),
            true,
        );
        let coruscant = add_system(
            &mut world,
            2,
            "Coruscant",
            ControlKind::Controlled(Faction::Empire),
            true,
        );
        let origin = add_system(
            &mut world,
            3,
            "Imperial Staging",
            ControlKind::Controlled(Faction::Empire),
            false,
        );
        let transport_class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(63),
            name: "Imperial Assault Transport".into(),
            is_alliance: false,
            is_empire: true,
            hull: 100,
            troop_capacity: 1,
            bombardment_modifier: 100,
            ..CapitalShipClass::default()
        });
        let fleet = world.fleets.insert(Fleet {
            location: origin,
            capital_ships: vec![ShipInstance::new(transport_class, 100, false)],
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            has_death_star: false,
        });
        world.systems[origin].fleets.push(fleet);
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(20),
            is_alliance: false,
            regiment_strength: 100,
        });
        world.systems[origin].ground_units.push(troop);

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
            victory: VictoryState::new(alliance_hq, coruscant),
            betrayal: BetrayalState::new(),
            economy: EconomyState::default(),
            repair: RepairState::default(),
            troop_transport: TroopTransportState::default(),
            combat_cooldowns: HashMap::new(),
            campaign_config: CampaignConfig {
                victory_conditions: VictoryConditions::HeadquartersOnly,
                ..CampaignConfig::default()
            },
        };
        states
            .troop_transport
            .embark(&mut world, fleet, &[troop])
            .unwrap();
        assert!(begin_fleet_transit(
            &mut states.movement,
            &mut world,
            fleet,
            alliance_hq,
            1,
        ));

        let events = run_simulation_tick(
            &mut world,
            &mut states,
            &[TickEvent { tick: 1 }],
            &[0.99; 2_048],
            10,
            &rebellion_core::tuning::GameConfig::default(),
        );

        assert!(!world.systems[alliance_hq].is_headquarters);
        assert_eq!(
            world.systems[alliance_hq].control,
            ControlKind::Controlled(Faction::Empire)
        );
        assert!(states.victory.resolved);
        assert!(events.iter().any(|event| {
            event.event_type == EVT_BOMBARDMENT && event.details["headquarters_destroyed"] == true
        }));
        assert!(events.iter().any(|event| event.event_type == EVT_VICTORY));
    }

    #[test]
    fn troop_transport_arrives_lands_occupies_and_captures() {
        let mut world = GameWorld::default();
        let add_system = |world: &mut GameWorld, dat_id, name: &str, control, is_headquarters| {
            world.systems.insert(System {
                dat_id: DatId::new(dat_id),
                name: name.into(),
                sector: Default::default(),
                x: dat_id as u16 * 100,
                y: 0,
                exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
                popularity_alliance: if control == ControlKind::Controlled(Faction::Alliance) {
                    0.9
                } else {
                    0.1
                },
                popularity_empire: if control == ControlKind::Controlled(Faction::Empire) {
                    0.9
                } else {
                    0.1
                },
                is_populated: true,
                total_energy: 0,
                raw_materials: 0,
                fleets: vec![],
                ground_units: vec![],
                special_forces: vec![],
                defense_facilities: vec![],
                manufacturing_facilities: vec![],
                production_facilities: vec![],
                is_headquarters,
                is_destroyed: false,
                control,
                espionage_rating: 0.0,
            })
        };
        let origin = add_system(
            &mut world,
            1,
            "Alliance Base",
            ControlKind::Controlled(Faction::Alliance),
            true,
        );
        let target = add_system(
            &mut world,
            2,
            "Imperial HQ",
            ControlKind::Controlled(Faction::Empire),
            true,
        );
        let transport_class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(64),
            name: "Troop Transport".into(),
            is_alliance: true,
            is_empire: false,
            hull: 100,
            troop_capacity: 1,
            ..CapitalShipClass::default()
        });
        let fleet = world.fleets.insert(Fleet {
            location: origin,
            capital_ships: vec![ShipInstance::new(transport_class, 100, true)],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        world.systems[origin].fleets.push(fleet);
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(20),
            is_alliance: true,
            regiment_strength: 100,
        });
        world.systems[origin].ground_units.push(troop);
        let prisoner = world.characters.insert(Character {
            dat_id: DatId::new(4),
            name: "Imperial Leader".into(),
            is_empire: true,
            current_system: Some(target),
            ..Character::default()
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
            victory: VictoryState::new(origin, target),
            betrayal: BetrayalState::new(),
            economy: EconomyState::default(),
            repair: RepairState::default(),
            troop_transport: TroopTransportState::default(),
            combat_cooldowns: HashMap::new(),
            campaign_config: CampaignConfig::default(),
        };
        states
            .troop_transport
            .embark(&mut world, fleet, &[troop])
            .unwrap();
        states.campaign_config.victory_conditions = VictoryConditions::HeadquartersOnly;
        assert!(begin_fleet_transit(
            &mut states.movement,
            &mut world,
            fleet,
            target,
            1,
        ));

        let events = run_simulation_tick(
            &mut world,
            &mut states,
            &[TickEvent { tick: 1 }],
            &[0.99; 2048],
            10,
            &rebellion_core::tuning::GameConfig::default(),
        );

        assert!(states.troop_transport.is_empty());
        assert!(world.systems[target].ground_units.contains(&troop));
        assert_eq!(
            world.systems[target].control,
            ControlKind::Controlled(Faction::Alliance)
        );
        assert!(world.characters[prisoner].is_captive);
        assert_eq!(
            world.characters[prisoner].captured_by,
            Some(Faction::Alliance)
        );
        assert!(states.victory.resolved);
        for event_type in [
            EVT_FLEET_ARRIVED,
            EVT_TROOP_MOVED,
            EVT_CONTROL_CHANGED,
            EVT_CAPTURE,
            EVT_VICTORY,
        ] {
            assert!(
                events.iter().any(|event| event.event_type == event_type),
                "missing {event_type} telemetry"
            );
        }
    }
}
