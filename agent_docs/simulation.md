---
title: "Simulation Systems"
description: "Seventeen stateless advance-pattern modules implementing the game simulation"
category: "agent-docs"
created: 2026-03-14
updated: 2026-10-16
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
| 5 | **AI** | `ai.rs`, `ai_scheduler.rs` | `AISystem::round(state, inputs)`, driven by `simulation::ai_step` | A round a side per in-day step, rate+1 steps a day (`AiConfig::rounds_per_day`: 5 at Fast, 13, 61, 601), at most `day_orders` (10) orders a side a day; each round's order applies at the next step. Module 13, regional records, fleet tasks, character tasks, production modules, mission planners (`systems/ai-parity-tracker.md`) |
| 6 | **Movement** | `movement.rs` | `advance(state, ticks) -> Vec<ArrivalEvent>` | Caller must update `fleet.location` + `system.fleets` |
| 7 | **Fog** | `fog.rs` | `advance(state, world, movement)` | Fleet presence + sensor radius (detection * 15.0); monotonic |
| 8 | **Combat** | `tactical.rs`, `combat.rs` | `resolve_system_space_combat()` (integrator) | Space battles run the tactical engine headless; one session per contested system, seeded from one roll. Each headless step runs the Empire's side first, then the Alliance's, and after a side's damage the morale check (`BattleSession::morale_withdraws`, `FUN_005d19a0`) withdraws a side whose strength is 0 or below its threshold of the enemy's; `apply::seat_morale` sets each side's threshold and retreat permission (`ghidra/notes/morale-retreat.md`) |
| 9 | **Blockade** | `blockade.rs` | `advance(state, world, ticks)` | Hostile fleet + no defender = blockade. The transitions run after combat and before the AI, so a fleet sent on the day it arrives still blockades (`FUN_0050b8e0`, event 0x311). Timer `0x386` per blockaded system shifts the blockader's support by GNPRTB 7693/7695 every 30 days (`blockade.md`) |
| 10 | **Uprising** | `uprising.rs` | `advance(state, world, economy, missions, ticks, rolls)` | Needs the economy summary; incidents and disasters run on recovered timers |
| 11 | **Death Star** | `death_star.rs` | `advance(state, world, ticks)` | Self-clears construction on completion |
| 12 | **Research** | `research.rs` | `advance(state, world, uprisings, ticks, rolls)` | Passive timer `0x381` adds points and spends them into levels; Research missions credit points at their outcome (`apply_mission_points`) |
| 13 | **Jedi** | `jedi.rs` | `advance(state, world, ticks, rolls)` | XP stored in `JediTrainingRecord`, not world |
| 14 | **Victory** | `victory.rs` | `check(state, world, ticks)` | The win word: EW for AD (the HQ gone, `is_headquarters` clear or its system destroyed), plus LC and MC in Standard; AW for CA or CD, plus DC and EC in Standard; in a tie the previous winner keeps it, else the Alliance wins |
| 15 | **Betrayal** | `betrayal.rs` | `advance(state, world, ticks, rolls, loyalty_tb)` | `is_unable_to_betray` immunity; 50-tick cooldown |
| 16 | **Economy** | `economy.rs` | `advance(state, world, ticks, difficulty)` | Full 18-function strategic state rebuild (FUN_00508250). Runs BEFORE manufacturing (position 0). Resource caps, support drift (system `+0x6c` for the diversion roll, never added to support), collection rate, KDY modifier, the holder rule `system_control` (regiments, else support against GNPRTB 7760; FUN_0050b610; run by `support_shift`'s hooks, not here), garrison, troop/fleet summary, incident generation (state-transition-driven, 4 flags). 17 GNPRTB indices. 30 tests. |
| 17 | **Repair** | `repair.rs` | `advance(state, world, ticks, rolls)` | Timer `0x380` per damaged ship not in hyperspace: every GNPRTB `0x805` days (30), or `0x804` days (2) at a system holding a shipyard of its side. Each step heals max hull / 10 and, at the class damage control percent, removes one damage-counter point (`FUN_00501c50`); two rolls from the simulation stream per firing ship (`roll_budget`). `RepairCheckPerformed` marks a fleet's first timer, `ShipRepaired` each step, `TimerSet` an arm, rearm or cancel; callers write them with `RepairEvent::apply`. Module 11 phase `0x3f8` (`FUN_004c8e90`) moves free damaged fleets' mobile ships to the nearest own shipyard system. Active episodes and timers persist in save v49. |

Per-system detail docs: `agent_docs/systems/{combat,blockade,uprising,death-star,research,jedi,victory,betrayal,captivity,story-events,ai-parity-tracker}.md`

## Mission Types and Effects

| Kind | Effect on Success |
|------|-------------------|
| Diplomacy | `PopularityShifted` |
| Recruitment | `CharacterRecruited` |
| Sabotage | `FacilitySabotaged` → remove facility from system + arena |
| Assassination | `CharacterInjured`; a minor dies on GNPRTB 2577 (`CharacterKilled` → leaves its fleets, marked killed), a major never (`victory-flag-word.md`) |
| Espionage | `SystemIntelligenceGathered` → set explored |
| Rescue | `CharacterRescued` → restore faction, clear captivity |
| Abduction | `CharacterCaptured` → set `is_captive`/`captured_by`/`capture_tick`, remove from fleets (`systems/captivity.md`) |
| InciteUprising | `UprisingStarted` → shift popularity |
| Autoscrap | (no world effect) |

Additional per-tick effects (not mission-driven):
- `CharacterEscaped` (from `escape_step()`, timer `0x38a`) → `death_star::free_escapee` (`systems/captivity.md`)
- `CharacterBusy` → set `on_mission = true`
- `CharacterAvailable` → clear `on_mission`, `on_hidden_mission`
- `DecoyTriggered` → log message, no world mutation

Probability: MSTB table lookup (piecewise-linear) with quadratic fallback. Combined: `total_prob = agent_prob · (1 − foil_prob)`.

## Integration Order (simulation.rs → integrator.rs)

```
run_simulation_tick():
  integrator = PerceptionIntegrator::new(tick, wall_ms)
  bit 11: death_star::refresh_sectors() → GameWorld::death_star_sectors (FUN_005178b0); again after steps 2 and 3, and after a load
  prologue: a day a native save left open runs its remaining steps (ai_step), then close_ai_days() → ai_day records, order counts cleared
  0. economy_events  = EconomySystem::advance()       → integrator.apply_economy_events()
  0b. stockpiles     = StockpileSystem::advance()      → scrap::scrap_all_reporting() for overdraft scraps (a `scrapped_aboard` record names the squadrons and regiments a scrapped ship took down; each is refunded half its class cost, `ghidra/notes/top-bar-resource-counters.md`)
  1. completions     = ManufacturingSystem::advance()  → integrator.apply_build_completions()
  2. arrivals        = MovementSystem::advance()       → integrator.apply_arrivals()
  3. combat triggers → resolve_system_space_combat() (headless BattleSession); regiments land and fight only on the Planetary Assault order (AI: ai_step → integrator::resolve_ai_assault)
  3c. blockade       = BlockadeSystem::advance()        (transitions and timer 0x386; applied at step 8); then the battle bit (both sides present at a system with a side), each change of which sweeps the system's adrift characters (death_star::sweep_battle_changes; no destination: `systems/captivity.md`)
  4. reveals         = FogSystem::advance()             → integrator.emit_fog_reveals()
  5. results         = MissionSystem::advance()         → integrator.apply_mission_result()
  5b. escapes        = MissionSystem::escape_step()     → integrator.apply_escape_effects(), death_star::free_escapee()
  6. fired_events    = EventSystem::advance()           → integrator.apply_fired_events()
  8. running regiments = BlockadeSystem::resolve_running() → integrator.apply_blockade_events()
  9. uprising        = UprisingSystem::advance()        → integrator.apply_uprising_events()
  10. betrayal       = BetrayalSystem::advance()        → integrator.apply_betrayal_events()
  12. research       = ResearchSystem::advance()        → integrator.apply_research_results()
  12b. repair        = RepairSystem::advance()          → RepairEvent::apply()
  13. jedi           = JediSystem::advance()            → integrator.apply_jedi_events()
  13b. AI            = ai_step() for steps 0..=rate      → integrator.emit_ai_order() (+ emit_strike, emit_assault), apply_blockade_events() (each step: pending orders applied Alliance first, strikes through resolve_ai_strike, assaults through resolve_ai_assault, regiment runs rolled, then each side's round)
  name_flagship_fleets(), clear_lapsed_ranks()
  14. victory        = VictorySystem::check()           → integrator.apply_victory()
  15. snapshot       (every 250 ticks)                  → integrator.emit_campaign_snapshot()
  return integrator.finish()
```

Order matters: economy runs before manufacturing (production modifiers); combat needs fleet positions from movement; AI reads mission and manufacturing state that earlier systems may have modified.

A Planetary Bombardment order (`0x220`-`0x223`) resolves in the step that
issues it, as the original's message `0x224` does: the player's when the menu
row or dev command runs (`strike_orders::issue`, seeded from `sim_rng`), the
AI's inside `ai_step` as its pending order applies (`resolve_ai_strike`, one
roll from the step's stream). Both go through `integrator::order_strike`
(`ghidra/notes/bombardment.md`, Port). Destroy System's cascade runs the
mission validator at once (`MissionState::validate_destroyed`), not at the
missions' next step, and when it set an adrift flag the strike sweeps the
system after the character roll (`death_star::sweep_adrift`, event `0x30e`),
so its adrift characters leave that day (`systems/captivity.md` for one with no place).
An ordinary run also returns its Operation Report (`StrikeEffects::operation`,
`operation_report::OperationReport`): what each destroyed object's event
`0x302` lists (`StrikeReport::system_destroyed`, `fleet_destroyed`), then the
fleets' and the system's survivors once the finalize clears the flags. The
report message (kind `0xf`) carries it; the fleet owner's copy opens the
report window (`ghidra/notes/operation-report-window.md`). Destroy System
files none.
A Planetary Assault (`0x234`) resolves in the step that issues it too
(`integrator::order_assault` for the player's, `resolve_ai_assault` for the
AI's; `ghidra/notes/ground-assault.md`) and returns the same report
(`AssaultEffects::operation`).

Support shifts, regiment events and holder changes queue on
`GameWorld::support_events` in priority order (`support_shift::raise`,
`FUN_005f61c0`: holder events `0x10` before shifts `0x100`, raising order
within one priority) and are served head-first at the end of the operation
that raised them (`support_shift::fire`, `FUN_0051df30`): a strike, an assault, an applied AI
order (`simulation::ai_step`), a player's order or scrap, the day's
overdraft scraps, builds and regiment arrivals, the blockade events, the
missions, the uprising events (`ghidra/notes/support-shifts.md`, Port). No
pass resolves control on a clock. A system's holder (`FUN_0050b610`,
`support_shift::rebuild_control`, with its troop bits
`GameWorld::troop_held`) is rebuilt by the original's hooks: every support
change through the setter (`uprising::apply_support_change`, at once;
`FUN_005109f0`), every regiment that joins or leaves a system
(`support_shift::regiments_changed`, served with the queue; `FUN_00508660`)
and the populated bit's clear (Destroy System; `FUN_00510e30`). A holder
change's owner hook frees and flags the characters of another side there
and turns or removes its special forces; after each operation
`integrator::sweep_requests` (event `0x307`) sends the flagged characters
to their side's nearest system.

## Shared Simulation Tick (v0.8.0) + PerceptionIntegrator (v0.19.0)

The tick orchestration lives in `rebellion-data/src/simulation.rs`:

```rust
pub fn run_simulation_tick(world, states, tick_events, rolls, wall_ms, config) -> Vec<GameEventRecord>
```

All world mutation and telemetry emission lives in `rebellion-data/src/integrator.rs`:

```rust
pub struct PerceptionIntegrator { events: Vec<GameEventRecord>, tick: u64, wall_ms: u64 }
impl PerceptionIntegrator {
    pub fn apply_economy_events(&mut self, world, events) // ... one apply_/emit_ method per system
    pub fn finish(self) -> Vec<GameEventRecord>
}
```

`rebellion-playtest` calls `run_simulation_tick`; the interactive
`rebellion-app` still duplicates this sequence. That divergence is tracked as
F-012 and must close before native/WASM replay equivalence can pass.
The AI is shared: native calls the same `simulation::ai_step`,
`close_ai_days` and appliers. Its tick runs the prologue after
`reconcile_fleet_orbits` and the day's first round after Jedi; the later
rounds run over the day's frames, each frame running the steps
`GameClock::in_day_step(rounds_per_day)` has reached. A fast-forward day (one
day a frame) runs its later rounds in the next tick's prologue. Each order
writes one `ai_action` record (`telemetry::native_ai_order`), and each closed
day an `ai_day`. The player's advisor agent (`PlayerAgent::step`) takes the
player's slot of each step.
The native player's battle waits for the Battle Alert, while the rest of
that frame's tick still runs, and the frame opens one battle; later contested
systems wait for a later pass. The original resolves a battle before its
computer player orders again (`FUN_0040a050` states 5-0x17; the day stands
still under the alert), so a battle or cutscene opened in the frame holds the
day's first round until the galaxy returns (hyp: the Battle Alert is modal
before the step's phase 8; how `FUN_0040a050`'s alert states gate the
steps of `FUN_0051dea0` is untraced). The systems left for a later pass hold every AI
order that moves or changes a fleet, squadron, regiment or planet there
(`held_at`): moves, splits, joins, assaults, bombardments, Destroy System,
scraps and squadron or regiment moves. A held order records `outcome:
"held"` with `held_at`, and its module restarts (`AISystem::settle`).
`SimulationStates` bundles all 15 state types plus economy
state. `GameEventRecord` (in `rebellion-core/src/game_events.rs`) is structured
telemetry with no IO. The `config` parameter (`&GameConfig` from `tuning.rs`)
holds the AI tempo (`AiConfig`: `day_orders`, `rounds_per_day`) and scoring targets; the playtest binary
accepts `--config <path>` for autoresearch.

**Architecture**: simulation.rs is a thin orchestrator that calls `advance()` on each system and delegates mutation + telemetry to integrator methods. No `events.push()` calls remain in simulation.rs — `integrator.finish()` is the sole return path.

## Telemetry and Traces

Both tick codes write `GameEventRecord` lines, one JSON object per line:
`tick`, `wall_ms`, `system`, `event_type`, `details`, `source`
(`headless` from `run_simulation_tick`, `native` from the game's own tick)
and, on native lines only, `seq`, the developer-channel command last run
(`CommandScript::last_seq`) when the record was emitted. The payload
builders for combat, mission resolved, AI action, blockade, research
unlocked and control changed live in `rebellion-data/src/telemetry.rs`; the
integrator and the native call sites both use them, so a query reads either.

| Run | Switch | Output |
|---|---|---|
| Playtest | `--output FILE`, `--jsonl` | all records |
| Native | `scripts/launch-native.sh --evidence DIR --telemetry` (`OPEN_REBELLION_TELEMETRY=1`) | `DIR/telemetry.jsonl` |

A native battle through the Battle Alert or the tactical window returns through
`tactical_flow::apply_results`, which emits `pre_battle` (no `rng_seed`) and
`combat_space` like the unwatched path. Both report who holds the system
once the post-battle pass has run (`integrator::post_battle_outcome`,
`FUN_00517270.c:79-93`): the side still present wins, and with both present
the system stays contested (`stalemate`).

The native hooks cover combat, mission resolved, AI action (applied ones
only where an arm refuses with `continue`), blockade, research unlocked and
control changes. The other event types are headless-only.

Decision traces (`rebellion-core/src/trace.rs`) record the inputs of a rule
and its choice. A subsystem is one bit of `TraceMask`; with the bit clear a
trace point costs one relaxed atomic load and a branch and builds nothing.
Switch them on with `--trace missions,ai,research,blockade,combat,control`
(playtest; `all` selects every one) or `OPEN_REBELLION_TRACE=` with the same
list (native, which also needs `--telemetry` and the developer gate
`OPEN_REBELLION_DEV=1`, and sets the mask only once `telemetry.jsonl` opens). They join the same JSONL as
`event_type: "trace"`, `system` the subsystem and `details.decision` the
decision; the other `details` fields are that decision's own, so `kind` there
is a payload field (the mission kind), never the decision:

| `system` / `kind` | `details` |
|---|---|
| `missions` / `member_roll` | `mission_id`, `kind` (the mission kind), `member`, `target_system`, `row` (`table`, `input`, `threshold`: the MSTB row read), `chance`, `skill` (the member's effective skill, null without one), `input_source` (which input the chance used), `rolled` (0-99), `succeeded` |
| `missions` / `table_roll` | `table` (FOILTB, TDECOYTB, FDECOYTB), `input`, `threshold`, `chance`, `rolled`, `passed`: the detection and decoy rolls |
| `ai` / `dispatch` | `kind` (the mission kind), `target_system`, `target_character`, `chosen`, `team`, `decoys` (sizes) |
| `ai` / `*_candidates`, `incite_candidate`, `recruitment_base` | `candidates`: systems with the evaluator's own sort `score` (popularity, facilities, strength) |
| `ai` / `posture_execute`, `posture_regroup` | per fleet: `faction`, `fleet`, `location`, `target`, `need_before`, `need_after` (engage, assault, bombard, drop), `outcome` (`kept`, `cleared`, `cleared_away`, `en_route`, `order`), `order` kind; then the regroup `action` (`join`, `move`, `none`), `join_target`, `move_to` |
| `ai` / `covert_character_candidates`, `rescue_candidates`, `research_tree` | the characters and tree flags the round considered |
| `ai` / `round` | one per side's round: `side`, `tick`, `round_in_day`, `visited` (module kinds stepped, in order), `kind` (the module that ended it), `priority_pick`, `result` (`order`, `spent`, `dirty`, `lap`, `capped`), `resume`, `order`, `orders_today`, `cursor`, `scan` |
| `ai` / `production_turn`, `fleet_task_turn`, `character_task_turn`, `mission_planner_turn` | one per module turn: the production `module`, `phase`, `spent`, `queued`; the fleet tasks' `tasks`, `attack_class_threshold`, `task`; the character `task` and `next`; the mission `planner` and `next`; each with its `order`, and `resume` or `dirty` |
| `research` / `points_added`, `level_threshold`, `tree_exhausted` | `faction`, `tech`, `added`, `pool`, `level`, then `next_order`, `difficulty`, `cost`, `unlocks` |
| `blockade` / `transition`, `running_roll` | `system`, `control`, `populated`, `alliance_present`, `empire_present`, `withdraw_percent`; then `withdraw_percent`, `draw`, `destroyed` |
| `combat` / `pre_battle` | `system`, `tick`, `rng_seed`, `alliance` and `empire` (`capital_ships`, `fighter_squadrons`, `hull`), fleet counts |
| `combat` / `pass_winner`, `relocation` | the post-battle pass: `system`, `winner` and `source` (`recorded` or `holder_tie`); then per object `system`, `object` (`fleet`, `squadron`, `winner_squadron`), `outcome`, `winner` |
| `control` / `control_resolved` | `system`, `from`, `to`, `populated`, `alliance_troops`, `empire_troops`, `alliance_support`, `threshold` |

A trace only reads. Building one never draws from or advances an RNG,
reorders a collection or changes state. Recording the seed-42 replay with
every subsystem on and with all off gives identical fingerprints at all nine
checkpoints and at the end (compared to each other, by hand). The mask and
the record buffer are per thread, because the playtest and the native main
loop each set the mask and run the simulation on one thread; a parallel test
sees its own clear mask. The tick's thread drains its buffer
(`run_simulation_tick` at its end, the native frame after the tick). Query with `scripts/telemetry.py` (`test-tooling.md`).

The playtest stages from saves: `--load PATH` starts from a save in the
game's own format, with its RNG position, AI sides and config (so `--seed`
and `--ai-faction` do not apply; `--config`, when given, replaces the
save's config), and `--save-out PATH` writes one
at the end; the side the playtest does not view keeps a freshly seeded fog.
The clock's `tick` is the day counter, so REPL `tick N` and `advance_N`
continue from it. A loaded run continues exactly as the uninterrupted one: the playtest's
roll generator (`rng`) is saved as `sim_rng` and restored, and each world
slotmap is saved with its free-list order (`save-load.md`), so a new object
takes the slot and version it would have taken without the save. Seed 42 and
seed 7 dual-AI, saved at day 60 and loaded for 15 days, end at the day-75
fingerprint of the straight run, and seed 42 saved at day 300 matches the
straight run at day 1000.

## State Ownership

All simulation states are created in `main.rs` and included in `SaveState` (`rebellion-data/src/save.rs`, the current `SAVE_VERSION`, `save-load.md`) for serialization.

| State | Mutated By |
|-------|------------|
| `GameClock` | `clock.advance(dt)` |
| `ManufacturingState` | `ManufacturingSystem::advance_with_blockade`, AI |
| `MissionState` | `MissionSystem::advance`, AI |
| `EventState` | `EventSystem::advance` |
| `AIState` | `AISystem::round` (`ai_scheduler.rs`), `close_ai_days`, mission callbacks |
| `MovementState` | `MovementSystem::advance`, AI fleet orders |
| `FogState` | `FogSystem::advance` |
| `BlockadeState` | `BlockadeSystem::advance` |
| `UprisingState` | `UprisingSystem::advance`, `end_if_garrisoned()` |
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
