---
title: "Simulation Systems"
description: "Fifteen stateless advance-pattern modules implementing the game simulation"
category: "agent-docs"
created: 2026-03-14
updated: 2026-03-16
tags: [simulation, advance-pattern, tick, deterministic]
---

# Simulation Systems

Fifteen modules in `rebellion-core/src/` implement the game simulation. All follow the same stateless advance pattern.

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

The caller (rebellion-app `main.rs`) applies effects to `GameWorld` after each system runs.

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

## Integration Order (main.rs)

```
Each frame:
  tick_events = clock.advance(dt)
  if tick_events not empty:
    completions   = ManufacturingSystem::advance_with_blockade(mfg, ticks, blockade.blocked())
    arrivals      = MovementSystem::advance(movement, ticks) → update fleet.location + system.fleets
    combat        = per-system: CombatSystem::resolve_space/ground/bombardment (5-tick cooldown)
    reveals       = FogSystem::advance(fog, world, movement)
    results       = MissionSystem::advance(missions, world, ticks, rolls) → apply effects
    fired_events  = EventSystem::advance(events, world, ticks, rolls) → apply actions
    ai_actions    = AISystem::advance(ai, world, mfg, missions, ticks) → apply
    blockade      = BlockadeSystem::advance(blockade, world, ticks) → apply troop destruction
    uprising      = UprisingSystem::advance(uprising, world, ticks, rolls, upris1tb) → flip factions
    ds_events     = DeathStarSystem::advance(ds, world, ticks) → apply planet destruction
    research      = ResearchSystem::advance(research, world, ticks) → levels auto-applied
    jedi          = JediSystem::advance(jedi, world, ticks, rolls) → apply tier/detection
    victory       = VictorySystem::check(victory, world, ticks) → end game
```

Order matters: manufacturing needs blockade set from prior tick; combat needs fleet positions from movement; AI reads mission and manufacturing state that earlier systems may have modified.

## Shared Simulation Tick (v0.8.0)

The effect-application logic is extracted into `rebellion-data/src/simulation.rs`:

```rust
pub fn run_simulation_tick(world, states, tick_events, rolls, wall_ms, config) -> Vec<GameEventRecord>
```

Both the interactive binary (`rebellion-app`) and the headless binary (`rebellion-playtest`) call this function. `SimulationStates` bundles all 15 state types + combat cooldowns. `GameEventRecord` (in `rebellion-core/src/game_events.rs`) is the structured telemetry type — pure data, no IO. The `config` parameter (`&GameConfig` from `tuning.rs`) controls all tunable AI, movement, and production parameters — the interactive app uses defaults, the playtest binary accepts `--config <path>` for autoresearch.

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
6. Wire into main loop in `crates/rebellion-app/src/main.rs` at the correct position
7. Write effect application function in `main.rs`
8. Add the state to `SaveState` in `crates/rebellion-data/src/save.rs`, bump `SAVE_VERSION`
9. Add unit tests — the advance() pattern makes this trivial (no IO mocks needed)
10. Add doc file at `agent_docs/systems/{name}.md`

## Adding a New Mission Type

1. Add variant to `MissionKind` enum in `missions.rs`
2. Add match arm in `MissionSystem::compute_effects()` — return `Vec<MissionEffect>` for success
3. If new effects are needed, add variant to `MissionEffect` enum
4. Add match arm in `main.rs` `apply_mission_result()` to mutate `GameWorld`
5. Add match arm in `main.rs` mission logging (kind_name mapping)
6. Add to panel UI: `rebellion-render/src/panels/missions.rs`
7. If the mission needs a probability table, add a `*MSTB.DAT` parser and register the table name in `world.mission_tables`
8. Add `target_character` if the mission targets a specific character (assassination, abduction, rescue pattern)
9. Test: create an `ActiveMission`, advance to completion, verify effects

## Adding a New Event Type

1. Add variant to `EventCondition` or `EventAction` in `events.rs`
2. For conditions: add evaluation logic in `EventSystem::evaluate_condition()`
3. For actions: add application logic in `main.rs` `apply_event_actions()`
4. Events are data-driven — defined via `GameEvent` structs with `conditions` and `actions` vectors
5. Event chaining: use `EventCondition::EventFired(id)` to sequence events
