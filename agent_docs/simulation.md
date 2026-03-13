# Simulation Systems

Seven modules in `rebellion-core/src/` implement the game simulation. All follow the same stateless advance pattern.

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

### 1. Tick (`tick.rs`, 280 LOC, 13 tests)

The clock. Converts wall-time `dt: f32` into discrete game-day ticks.

| Type | Purpose |
|------|---------|
| `GameClock` | Accumulates dt, emits `Vec<TickEvent>` when threshold crossed |
| `GameSpeed` | Paused / Normal(1x) / Fast(2x) / Faster(4x) |
| `TickEvent` | Marker: `{ tick: u64 }` — consumed by all downstream systems |

```rust
let tick_events = clock.advance(get_frame_time()); // 0 or more TickEvents
```

Constant: `TICK_DURATION = 1.0` seconds at Normal speed.

### 2. Manufacturing (`manufacturing.rs`, 520 LOC, 13 tests)

Per-system production queues.

| Type | Purpose |
|------|---------|
| `ManufacturingState` | `HashMap<SystemKey, VecDeque<QueueItem>>` |
| `QueueItem` | `{ kind: BuildableKind, ticks_remaining, ticks_total }` |
| `BuildableKind` | Enum: CapitalShip/Fighter/Troop/DefenseFacility/ManufacturingFacility/ProductionFacility |
| `CompletionEvent` | `{ system, kind, tick }` |

```rust
let completions = ManufacturingSystem::advance(&mut mfg_state, &tick_events);
```

No GameWorld access needed — pure queue math. Overflow ticks propagate to the next item.

### 3. Missions (`missions.rs`, 880 LOC, 14 tests)

Diplomacy and recruitment mission lifecycle.

| Type | Purpose |
|------|---------|
| `MissionState` | `VecDeque<ActiveMission>` across all factions |
| `ActiveMission` | `{ kind, faction, character, target_system, ticks_remaining }` |
| `MissionResult` | `{ outcome, effects: Vec<MissionEffect>, tick, ... }` |
| `MissionEffect` | PopularityShifted / CharacterRecruited |

```rust
let results = MissionSystem::advance(&mut mission_state, &world, &tick_events, &rng_rolls);
```

Probability formula (from rebellion2's Mission.cs): `prob = clamp(a·score² + b·score + c, min%, max%)`. Coefficients per `MissionKind`.

### 4. Events (`events.rs`, 728 LOC, 17 tests)

Conditional trigger system for scripted game events.

| Type | Purpose |
|------|---------|
| `EventState` | All defined events + set of fired IDs |
| `GameEvent` | `{ id, name, conditions, actions, is_repeatable, enabled }` |
| `EventCondition` | TickReached / TickAtLeast / CharacterAtSystem / Random / EventFired |
| `EventAction` | ShiftPopularity / DisplayMessage / ModifyCharacterSkill / RelocateCharacter |
| `FiredEvent` | `{ id, actions, tick }` |

```rust
let fired = EventSystem::advance(&mut event_state, &world, &tick_events, &rng_rolls);
```

Event chaining: `EventFired` conditions see events fired earlier in the same `advance()` call.

### 5. AI (`ai.rs`, 936 LOC, 13 tests)

Rule-based opponent faction controller (ported from rebellion2's AIManager.cs).

| Type | Purpose |
|------|---------|
| `AIState` | Faction assignment, cooldowns, busy/available character tracking |
| `AIAction` | DispatchMission / EnqueueProduction / MoveFleet |
| `AiFaction` | Empire / Alliance |

```rust
let actions = AISystem::advance(&mut ai_state, &world, &mfg_state, &mission_state, &tick_events);
```

Only re-evaluates every `AI_TICK_INTERVAL` (7) game-days. Three heuristic modules: officer assignment (diplomacy skill > 60), production priority (fighters → facilities), fleet deployment (attack weak / reinforce).

### 6. Movement (`movement.rs`, 453 LOC, 11 tests)

Fleet hyperspace transit between star systems.

| Type | Purpose |
|------|---------|
| `MovementState` | `HashMap<FleetKey, Transit>` — active movements |
| `MovementOrder` | `{ fleet, origin, destination, ticks_per_hop }` |
| `ArrivalEvent` | `{ fleet, system, tick }` |

```rust
let arrivals = MovementSystem::advance(&mut movement_state, &tick_events);
```

Speed: `ticks_per_hop = BASE_TICKS_PER_HOP(30) / slowest_hyperdrive`, clamped to `MIN_TICKS_PER_HOP(5)`. Fleets without capital ships use `DEFAULT_TICKS_PER_HOP(20)`.

### 7. Fog of War (`fog.rs`, 373 LOC, 9 tests)

Per-faction visibility tracking with monotonic reveal.

| Type | Purpose |
|------|---------|
| `FogState` | `{ faction, visible: HashSet<SystemKey> }` |
| `RevealEvent` | `{ system, tick }` |

```rust
FogSystem::seed(&mut fog, &world);           // at game start
let reveals = FogSystem::advance(&mut fog, &world, &movement_state); // each tick
```

Visibility rules: fleet presence reveals instantly; advance intel reveals at 50% transit progress. Systems never become unexplored again.

## Integration Order (main.rs)

```
Each frame:
  tick_events = clock.advance(dt)
  if tick_events not empty:
    completions  = ManufacturingSystem::advance(mfg_state, tick_events)
    results      = MissionSystem::advance(mission_state, world, tick_events, rolls)
    fired_events = EventSystem::advance(event_state, world, tick_events, rolls)
    ai_actions   = AISystem::advance(ai_state, world, mfg_state, mission_state, tick_events)
    // caller applies all effects to GameWorld + MessageLog
```

Order matters: missions read world state that events may have changed; AI reads mission and manufacturing state that earlier systems may have modified.

## State Ownership

| State | Created | Location | Mutated By |
|-------|---------|----------|------------|
| `GameClock` | `main.rs` | local | `clock.advance(dt)` |
| `ManufacturingState` | `main.rs` | local | `ManufacturingSystem::advance`, AI effect application |
| `MissionState` | `main.rs` | local | `MissionSystem::advance`, AI effect application |
| `EventState` | `main.rs` | local | `EventSystem::advance` |
| `AIState` | `main.rs` | local | `AISystem::advance`, mission result callbacks |
| `MovementState` | `main.rs` | local | `MovementSystem::advance` |
| `FogState` | `main.rs` | local | `FogSystem::advance` |
| `GameWorld` | `rebellion-data` | `main.rs` | Effect application only (never inside advance()) |

## Adding a New Simulation System

1. Create `crates/rebellion-core/src/{name}.rs`
2. Define `{Name}State` (persistent data) and `{Name}System` (stateless namespace)
3. Implement `{Name}System::advance(&mut State, ..., &[TickEvent]) -> Vec<ResultEvent>`
4. Accept RNG as caller-provided slices, not internal `rand`
5. Add `pub mod {name};` to `crates/rebellion-core/src/lib.rs`
6. Wire into main loop in `crates/rebellion-app/src/main.rs` at the correct position
7. Write effect application function in `main.rs`
8. Add unit tests — the advance() pattern makes this trivial (no IO mocks needed)
