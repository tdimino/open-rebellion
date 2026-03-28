---
title: "Simulation Systems"
description: "Seventeen stateless advance-pattern modules implementing the game simulation"
category: "agent-docs"
created: 2026-03-14
updated: 2026-03-16
tags: [simulation, advance-pattern, tick, deterministic]
---

# Simulation Systems

Seventeen modules in `rebellion-core/src/` implement the game simulation. All follow the same stateless advance pattern.

## The Advance Contract

Every simulation system is a pure function:
```rust
System::advance(&mut State, &GameWorld, &[TickEvent], ...) -> Vec<ResultEvent>
```

Rules:
- **Never mutates GameWorld** — returns result events for the caller to apply
- **Caller provides RNG** — random rolls are passed as `&[f64]` or `&[f32]` slices, so tests can control outcomes exactly
- **Deterministic** — same inputs always produce same outputs
- **No IO** — rebellion-core has zero IO deps; stays headless-testable

The `PerceptionIntegrator` (rebellion-data `integrator.rs`) applies effects to `GameWorld` and emits telemetry. `simulation.rs` orchestrates the tick; `integrator.rs` owns all mutation and telemetry.

## System Inventory

| # | System | File | Signature | Key Gotcha |
|---|--------|------|-----------|------------|
| 1 | **Tick** | `tick.rs` | `clock.advance(dt) -> Vec<TickEvent>` | `TICK_DURATION = 1.0s` at Normal speed |
| 2 | **Manufacturing** | `manufacturing.rs` | `advance_with_blockade(state, ticks, blocked)` | Blockaded systems skip production |
| 3 | **Missions** | `missions.rs` | `advance(state, world, ticks, rolls)` | 9 kinds, MSTB tables, `target_character` for assassination/abduction/rescue |
| 4 | **Events** | `events.rs` | `advance(state, world, ticks, rolls)` | `EventFired` chaining within same call |
| 5 | **AI** | `ai.rs` | `advance(state, world, mfg, missions, movement, ticks)` | Per-fleet targeting, deconfliction, battle penalty. Re-evaluates every 7 ticks. |
| 6 | **Movement** | `movement.rs` | `advance(state, ticks) -> Vec<ArrivalEvent>` | Caller must update `fleet.location` + `system.fleets` |
| 7 | **Fog** | `fog.rs` | `advance(state, world, movement)` | Fleet presence + sensor radius (detection * 15.0); monotonic |
| 8 | **Combat** | `combat.rs` | `resolve_space()` / `resolve_ground()` | Phase gate: `ACTIVE && !PHASES_ENABLED` (inverted) |
| 9 | **Blockade** | `blockade.rs` | `advance(state, world, ticks)` | Hostile fleet + no defender = blockade |
| 10 | **Uprising** | `uprising.rs` | `advance(state, world, ticks, rolls, upris1tb)` | First incident always fires (no cooldown entry) |
| 11 | **Death Star** | `death_star.rs` | `advance(state, world, ticks)` | Self-clears construction on completion |
| 12 | **Research** | `research.rs` | `advance(state, world, ticks)` | Pure return — caller applies level-ups (fixed v0.6.0) |
| 13 | **Jedi** | `jedi.rs` | `advance(state, world, ticks, rolls)` | XP stored in `JediTrainingRecord`, not world |
| 14 | **Victory** | `victory.rs` | `check(state, world, ticks)` | Death Star checks supersede HQ capture |
| 15 | **Betrayal** | `betrayal.rs` | `advance(state, world, ticks, rolls, loyalty_tb)` | `is_unable_to_betray` immunity; 50-tick cooldown |
| 16 | **Economy** | `economy.rs` | `advance(state, world, ticks, difficulty)` | Full 18-function strategic state rebuild (FUN_005073d0). Runs BEFORE manufacturing (position 0). Resource caps, support drift, collection rate, KDY modifier, side resolution (GNPRTB[7760] energy threshold), garrison, troop/fleet summary, incident generation (state-transition-driven, 4 flags). 17 GNPRTB indices. 30 tests. |
| 17 | **Repair** | `repair.rs` | `advance(state, world, ticks)` | Ships at shipyard systems auto-repair using class `damage_control` rate. Framework — per-hull tracking pending ShipInstance promotion. |

Per-system detail docs: `agent_docs/systems/{combat,blockade,uprising,death-star,research,jedi,victory,betrayal}.md`

## Mission Types and Effects

| Kind | Effect on Success |
|------|-------------------|
| Diplomacy | `PopularityShifted` |
| Recruitment | `CharacterRecruited` |
| Sabotage | `FacilitySabotaged` → remove facility from system + arena |
| Assassination | `CharacterKilled` → remove from fleets + arena |
| Espionage | `SystemIntelligenceGathered` → set explored |
| Rescue | `CharacterRescued` → restore faction, clear captivity |
| Abduction | `CharacterCaptured` → set `is_captive`/`captured_by`/`capture_tick`, flip faction, remove from fleets |
| InciteUprising | `UprisingStarted` → shift popularity |
| Autoscrap | (no world effect) |

Additional per-tick effects (not mission-driven):
- `CharacterEscaped` (from `check_escapes()`) → restore home faction, clear captivity
- `CharacterBusy` → set `on_mission = true`
- `CharacterAvailable` → clear `on_mission`, `on_hidden_mission`
- `DecoyTriggered` → log message, no world mutation

Probability: MSTB table lookup (piecewise-linear) with quadratic fallback. Combined: `total_prob = agent_prob · (1 − foil_prob)`.

## Integration Order (simulation.rs → integrator.rs)

```
run_simulation_tick():
  integrator = PerceptionIntegrator::new(tick, wall_ms)
  0. economy_events  = EconomySystem::advance()       → integrator.apply_economy_events()
  1. completions     = ManufacturingSystem::advance()  → integrator.apply_build_completions()
  2. arrivals        = MovementSystem::advance()       → integrator.apply_arrivals()
  3. combat triggers → CombatSystem::resolve_*()       → integrator.apply_space_combat() / apply_ground_combat()
  4. reveals         = FogSystem::advance()             → integrator.emit_fog_reveals()
  5. results         = MissionSystem::advance()         → integrator.apply_mission_result()
  5b. escapes        = MissionSystem::check_escapes()   → integrator.apply_escape_effects()
  6. fired_events    = EventSystem::advance()           → integrator.apply_fired_events()
  7. ai_actions      = AISystem::advance()              → integrator.apply_ai_actions()
  8. blockade        = BlockadeSystem::advance()        → integrator.apply_blockade_events()
  9. uprising        = UprisingSystem::advance()        → integrator.apply_uprising_events()
  10. betrayal       = BetrayalSystem::advance()        → integrator.apply_betrayal_events()
  11. ds_events      = DeathStarSystem::advance()       → integrator.apply_death_star_events()
  12. research       = ResearchSystem::advance()        → integrator.apply_research_results()
  13. jedi           = JediSystem::advance()            → integrator.apply_jedi_events()
  14. victory        = VictorySystem::check()           → integrator.apply_victory()
  15. snapshot       (every 250 ticks)                  → integrator.emit_campaign_snapshot()
  return integrator.finish()
```

Order matters: economy runs before manufacturing (production modifiers); combat needs fleet positions from movement; AI reads mission and manufacturing state that earlier systems may have modified.

## Shared Simulation Tick (v0.8.0) + PerceptionIntegrator (v0.19.0)

The tick orchestration lives in `rebellion-data/src/simulation.rs` (~449 LOC):

```rust
pub fn run_simulation_tick(world, states, tick_events, rolls, wall_ms, config) -> Vec<GameEventRecord>
```

All world mutation and telemetry emission lives in `rebellion-data/src/integrator.rs` (~1,185 LOC):

```rust
pub struct PerceptionIntegrator { events: Vec<GameEventRecord>, tick: u64, wall_ms: u64 }
impl PerceptionIntegrator {
    pub fn apply_economy_events(&mut self, world, events) // ... 17 methods total
    pub fn finish(self) -> Vec<GameEventRecord>
}
```

Both the interactive binary (`rebellion-app`) and the headless binary (`rebellion-playtest`) call `run_simulation_tick`. `SimulationStates` bundles all 15 state types + combat cooldowns + economy state. `GameEventRecord` (in `rebellion-core/src/game_events.rs`) is the structured telemetry type — pure data, no IO. The `config` parameter (`&GameConfig` from `tuning.rs`) controls all tunable AI, movement, and production parameters — the interactive app uses defaults, the playtest binary accepts `--config <path>` for autoresearch.

**Architecture**: simulation.rs is a thin orchestrator that calls `advance()` on each system and delegates mutation + telemetry to integrator methods. No `events.push()` calls remain in simulation.rs — `integrator.finish()` is the sole return path.

## State Ownership

All simulation states are created in `main.rs` and included in `SaveState` (`rebellion-data/src/save.rs`, version 4) for serialization.

| State | Mutated By |
|-------|------------|
| `GameClock` | `clock.advance(dt)` |
| `ManufacturingState` | `ManufacturingSystem::advance_with_blockade`, AI |
| `MissionState` | `MissionSystem::advance`, AI |
| `EventState` | `EventSystem::advance` |
| `AIState` | `AISystem::advance`, mission callbacks |
| `MovementState` | `MovementSystem::advance`, AI fleet orders |
| `FogState` | `FogSystem::advance` |
| `BlockadeState` | `BlockadeSystem::advance` |
| `UprisingState` | `UprisingSystem::advance`, `try_subdue()` |
| `DeathStarState` | `DeathStarSystem::advance`, `fire()`, `start_construction()` |
| `ResearchState` | `ResearchSystem::advance` (auto-applies level-ups) |
| `JediState` | `JediSystem::advance`, `start_training()`, `stop_training()` |
| `VictoryState` | Caller sets `resolved = true` after outcome |
| `GameWorld` | Effect application only (never inside advance()) |

## Adding a New Simulation System

1. Create `crates/rebellion-core/src/{name}.rs`
2. Define `{Name}State` (derive `Serialize, Deserialize`) and `{Name}System` (stateless namespace)
3. Implement `{Name}System::advance(&mut State, ..., &[TickEvent]) -> Vec<ResultEvent>`
4. Accept RNG as caller-provided slices, not internal `rand`
5. Add `pub mod {name};` to `crates/rebellion-core/src/lib.rs`
6. Add `advance()` call to `run_simulation_tick()` in `crates/rebellion-data/src/simulation.rs`
7. Add `apply_{name}_events()` method to `PerceptionIntegrator` in `crates/rebellion-data/src/integrator.rs` — this method applies world mutations AND emits telemetry
8. Add the state to `SimulationStates` in `simulation.rs` and `SaveState` in `save.rs`, bump `SAVE_VERSION`
9. Add unit tests — the advance() pattern makes this trivial (no IO mocks needed)
10. Add doc file at `agent_docs/systems/{name}.md`
11. If the interactive game also needs the system, add `advance()` + mutation to `main.rs` (mirrors the integrator pattern)

## Adding a New Mission Type

1. Add variant to `MissionKind` enum in `missions.rs`
2. Add match arm in `MissionSystem::compute_effects()` — return `Vec<MissionEffect>` for success
3. If new effects are needed, add variant to `MissionEffect` enum
4. Add match arm in `integrator.rs` `apply_mission_effects_inner()` to mutate `GameWorld`
5. Telemetry is automatic via `apply_mission_result()` on PerceptionIntegrator
6. Add to panel UI: `rebellion-render/src/panels/missions.rs`
7. If the mission needs a probability table, add a `*MSTB.DAT` parser and register the table name in `world.mission_tables`
8. Add `target_character` if the mission targets a specific character (assassination, abduction, rescue pattern)
9. Test: create an `ActiveMission`, advance to completion, verify effects

## Adding a New Event Type

1. Add variant to `EventCondition` or `EventAction` in `events.rs`
2. For conditions: add evaluation logic in `EventSystem::evaluate_condition()`
3. For actions: add application logic in `integrator.rs` `apply_event_actions_to_world_inner()`
4. Events are data-driven — defined via `GameEvent` structs with `conditions` and `actions` vectors
5. Event chaining: use `EventCondition::EventFired(id)` to sequence events
