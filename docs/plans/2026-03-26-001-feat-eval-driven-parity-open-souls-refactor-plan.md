---
title: "Knesset Ereshkigal — Eval-Driven Parity Sprint + Open Souls Refactor"
type: feat
status: active
date: 2026-03-26
origin: docs/reports/2026-03-26-community-disassembly-cross-reference.md
---

# Knesset Ereshkigal — Eval-Driven Parity Sprint + Open Souls Refactor

## Context

A community RE effort produced 13,036 decompiled C functions from REBEXE.EXE (vs our 5,151). Cross-referencing revealed ~85% parity with specific gaps across combat, AI/missions, economy, events, and multiplayer. This plan integrates ALL P0-P3 missing cross-references, instruments every system with structured telemetry, and refactors toward the Open Souls functional programming paradigm—all gated by measurable eval criteria.

**Baseline (2026-03-26):**
- Eval score: 0.6483 | Tests: 328 | Event types: 26 | Parity: ~85%

**Target:**
- Eval score: >= 0.72 | Tests: >= 400 | Event types: >= 38 | Parity: ~97%

## Functional Programming Manifesto

### Ten Principles

1. **Effects Are the Only Output.** Every system produces `Vec<GameEffect>`. No ad-hoc return types. The caller never pattern-matches on system-specific types. Currently 14 distinct result types across 15 systems—this collapses to one closed enum.

2. **Effects Are Values, Not Instructions.** `GameEffect` describes what happened ("fleet arrived at Coruscant"), not what to do ("update fleet.location"). The PerceptionIntegrator translates effects into mutations. Systems never know how effects are applied.

3. **Every Effect Is Invertible.** Every `GameEffect` carries enough data to produce its inverse. `PopularityShifted { delta: +0.1 }` inverts to `{ delta: -0.1 }`. Enables undo/rollback for autoresearch eval loop without full state cloning.

4. **Effects Carry Partial Order.** Tagged with `EffectPhase` (Economy=0, Manufacturing=1, Movement=2, Combat=3, Diplomacy=4, Intelligence=5, Command=6, Endgame=7). Within a phase, production order preserved. Across phases, discriminant order.

5. **GameWorld Is Read-Only During Advance.** No system receives `&mut GameWorld`. Systems get `&GameWorld` (frozen snapshot) and produce effects. Integrator applies all effects after systems run. Already satisfied by 13 of 15 systems.

6. **Cross-System Communication Is Effect-Only.** No system imports another system's State type. AI currently violates this (reads ManufacturingState, MissionState, MovementState, ResearchState at ai.rs:267-276). Fog violates (reads MovementState at fog.rs:127). Fix: projections on GameWorld.

7. **RNG Is Positional, Not Named.** Replace implicit mutable cursor with `RollCursor` struct tracking consumption per-system. Each system consumes exactly N rolls regardless of others. Enables replay divergence detection.

8. **Effects Are Serializable and Deterministic.** Every `GameEffect` derives `Serialize + Deserialize + Clone + PartialEq`. No `serde_json::Value` or `Box<dyn Any>` in payloads. Autoresearch diffs effect sequences structurally.

9. **Telemetry Is Derived, Not Emitted.** Systems don't produce `GameEventRecord`. The integrator derives records from effects via pure mapping. Eliminates 300+ lines of `serde_json::json!({...})` in simulation.rs. Telemetry is exhaustive by construction.

10. **State Snapshots Are Cheap.** `SimulationStates` and `GameWorld` both derive `Clone`. Full clone ~2-5ms for ~400 entities. Cheaper and more correct than custom diff/patch.

### Modularity Violations Found (8)

| # | Severity | Location | Violation |
|---|----------|----------|-----------|
| 1 | CRITICAL | `ai.rs:267-276` | AI reads ManufacturingState, MissionState, MovementState, ResearchState directly |
| 2 | CRITICAL | `simulation.rs:1360` + `main.rs:2719` | `apply_space_combat_result` duplicated in both files |
| 3 | CRITICAL | `simulation.rs:203-213` | Inline world mutations after MovementSystem::advance() (impure orchestration) |
| 4 | HIGH | `fog.rs:127-131` | Fog reads MovementState directly |
| 5 | HIGH | `simulation.rs:1119` | `apply_ai_actions_to_world` mutates 5 state types non-atomically |
| 6 | HIGH | `simulation.rs:811` vs `main.rs:2344` | `apply_mission_effects` vs `apply_mission_result` — different logic paths |
| 7 | MEDIUM | `simulation.rs:988` vs `main.rs:2569` | `apply_event_actions` duplicated with divergent logic |
| 8 | MEDIUM | `story_events.rs:6` | StoryEvents imports EventSystem types |

**Already correct**: No IO in advance() functions. All RNG uses `&[f64]` slice pattern. Combat/bombardment take immutable GameWorld.

### Effect Algebra (Rust)

Closed enum with ~45 variants across 8 phases. Key design: NOT trait objects (deterministic serialization, exhaustive match, structural diffing). Monoidal composition via `combine_effects()` (concatenate + stable phase sort). Filter via `filter_effects()`. Inversion via `effect.invert() -> Option<GameEffect>`.

### PerceptionIntegrator Architecture

Replaces 12 scattered apply_* functions (7 in simulation.rs + 5 in main.rs):
- **Pre-hooks**: Validation/filtering before application
- **apply_one()**: Single exhaustive match for ALL world mutations
- **Post-hooks**: Telemetry derivation + UI streaming (MessageLog, audio)
- **Transaction log**: Effect sequence for rollback/replay
- **Rollback**: Apply inverses in reverse order

### Open Souls Paradigm Mapping

| Open Souls | Open Rebellion | Refactor Target |
|-----------|----------------|-----------------|
| WorkingMemory | GameWorld | Immutable snapshot at tick boundary |
| Cognitive step | System::advance() | Returns `Vec<GameEffect>` (unified) |
| Mental process | Simulation tick | Composes 17 systems via PerceptionIntegrator |
| MemoryIntegrator | Effect application | Centralized `apply_one()` with hooks |
| useActions hooks | Post-hooks | Telemetry, UI streaming, cross-system notification |
| useSoulMemory | SimulationStates | Persistent across ticks, derives Clone+Serialize |

### Migration Order (16 systems, simplest first)

1. Research → 2. Jedi → 3. Betrayal → 4. Death Star → 5. Blockade → 6. Uprising → 7. Movement → 8. Fog (fix MovementState violation) → 9. Manufacturing → 10. Missions (largest, 15+ effect variants) → 11. Combat → 12. Events → 13. AI (fix 4 cross-state violations) → 14. Victory → 15. Economy (new) → 16. Repair (new)

Each step: change return type to `Vec<GameEffect>`, update simulation.rs to `integrator.collect()`, run tests, verify eval unchanged.

---

## Phase 0: Foundation — Effect Trait + Telemetry Audit

**Gate: `cargo check` + `cargo test` pass, eval score unchanged (0.6483)**

### 0.1 GameEffect Enum

New file `crates/rebellion-core/src/effects.rs`:

```rust
#[derive(Debug, Clone)]
pub enum GameEffect {
    BuildComplete { completion: CompletionEvent },
    FleetArrived { arrival: ArrivalEvent },
    SpaceCombatResolved { result: SpaceCombatResult },
    GroundCombatResolved { result: GroundCombatResult },
    BombardmentResolved { result: BombardmentResult },
    SystemRevealed { system: SystemKey, faction_is_alliance: bool },
    MissionResolved { result: MissionResult },
    CharacterEscaped { character: CharacterKey, escaped_to_alliance: bool },
    EventFired { fired: FiredEvent },
    AiActionTaken { action: AIAction },
    BlockadeChanged { event: BlockadeEvent },
    UprisingOccurred { event: UprisingEvent },
    CharacterBetrayed { event: BetrayalEvent },
    DeathStarEvent { event: DeathStarEvent },
    TechUnlocked { result: ResearchResult },
    JediProgressed { event: JediEvent },
    VictoryReached { outcome: VictoryOutcome },
    // New systems (Phase 1-2)
    EconomyTick { system: SystemKey, support_delta: f32, collection_rate: f32 },
    ShipRepaired { fleet: FleetKey, ship_class: CapitalShipKey, hull_restored: u32 },
    CampaignSnapshot { data: serde_json::Value },
}
```

### 0.2 New Telemetry Constants (12)

Add to `game_events.rs`: `EVT_ECONOMY_TICK`, `EVT_SUPPORT_DRIFT`, `EVT_COLLECTION_RATE`, `EVT_GARRISON_REQUIRED`, `EVT_SHIP_REPAIRED`, `EVT_SHIP_REPAIR_STARTED`, `EVT_DECOY_TRIGGERED`, `EVT_DS_SHIELD_STATUS`, `EVT_CHARACTER_COMBAT_RATING`, `EVT_STORY_JABBA`, `EVT_STORY_FINAL_BATTLE`, `EVT_MAINTENANCE_SHORTFALL`. Add `SYS_ECONOMY`, `SYS_REPAIR`.

### 0.3 Files

| File | Change |
|------|--------|
| `crates/rebellion-core/src/effects.rs` | NEW — GameEffect enum |
| `crates/rebellion-core/src/lib.rs` | Add `pub mod effects;` |
| `crates/rebellion-core/src/game_events.rs` | 12 new EVT_* + 2 SYS_* constants |

---

## Phase 1: P0 Formula Corrections + Economy System

**Gate: eval score >= 0.67, tests >= 345**

### 1.1 P0 Formula Fixes (missions.rs)

| # | Mission | Current | Correct | Line |
|---|---------|---------|---------|------|
| 1 | Sabotage | `skill` (espionage) | `(espionage + combat) / 2` | ~245 |
| 2 | InciteUprising | `pop_delta + diplomacy` | `pop_delta + diplomacy - system_espionage_rating` | ~193 |
| 3 | Abduction | `skill` (espionage) | `espionage - target_defense` | ~248 |
| 4 | Assassination | `skill` (combat) | `combat - target_defense` | ~246 |
| 5 | Recruitment | system-derived resistance | character's loyalty as resistance | ~223 |

**Signature change**: `compute_table_input()` gains `target_character: Option<&Character>` parameter.

**Tests**: `test_sabotage_uses_composite_skill`, `test_incite_uprising_counter_intel`, `test_abduction_target_defense`, `test_assassination_target_defense`, `test_recruitment_character_resistance`

### 1.2 Economy System (NEW: economy.rs)

Per-system tick loop with 18 sub-functions from `FUN_005073d0`:

**State**: `EconomyState { system_resources: HashMap<SystemKey, SystemResources> }` where `SystemResources { energy, energy_allocated, raw_materials, raw_materials_allocated, collection_rate, garrison_requirement, production_modifier }`.

**Popular Support Drift** (GNPRTB 7686-7688, 7732-7737):
```
if support <= 40 AND no_fleet:
    if support > 20 AND <= 30: base = 50
    elif support > 20: base = 25
    else: base = 75
    drift = clamp(base - fighters*5 - troops*2 - fleet*10, 0, 100)
if empire_controlled: drift = -drift
```

**Collection Rate**: `(GNPRTB[7763] * 100) / max(support, 1)`

**Garrison Requirement**: `(60 - support) / GNPRTB[7762]` with faction modifiers

**Tests (12)**: support_drift (3 variants), collection_rate, garrison_requirement, energy_allocation, material_allocation, mine_activation, production_modifier_kdy, maintenance_shortfall, control_flip, telemetry_emission

### 1.3 Eval Enhancement

Add 9th sub-metric `economy_activity` to `scripts/eval_game_quality.py` (weight 0.05, reduce event_diversity from 0.15 to 0.10).

### 1.4 Files

| File | Change |
|------|--------|
| `crates/rebellion-core/src/economy.rs` | NEW — ~800 LOC |
| `crates/rebellion-core/src/missions.rs` | 5 formula fixes + signature change |
| `crates/rebellion-data/src/simulation.rs` | Wire economy into tick loop |
| `crates/rebellion-data/src/save.rs` | EconomyState in SaveState, version bump |
| `crates/rebellion-core/src/tuning.rs` | EconomyConfig with `#[serde(default)]` |
| `scripts/eval_game_quality.py` | economy_activity metric |

---

## Phase 2: P1 Missing Core Systems

**Gate: eval score >= 0.70, tests >= 370**

### 2.1 Ship Repair System (NEW: repair.rs)

- `RepairState` tracks in-progress repairs per fleet
- Ships at systems with shipyards auto-repair each tick
- Fast repair variant (higher cost, faster rate)
- Requires `hull_damage: Vec<u32>` on ShipEntry in world/mod.rs
- Tests (6): repair_at_shipyard, no_repair_without_shipyard, fast_repair, repair_completes, fighter_repair, telemetry

### 2.2 Death Star Shield Generator (Entity 0x25)

- Add `shield_generator_active: bool` to DeathStarState
- Shield must be destroyed before DS takes damage
- Targetable as separate combat unit in resolve_space()
- Tests (4): shield_blocks_damage, shield_destroyed_first, vulnerable_after_shield, telemetry

### 2.3 Decoy Mission Subsystem

- Add `is_decoy: bool` to ActiveMission
- Decoy redirects foil checks away from real mission at same system
- GNPRTB[3588] = 35% penalty factor, TDECOYTB/FDECOYTB tables
- Tests (3): decoy_redirects_foil, penalty_applied, telemetry

### 2.4 Character Combat Rating in Ground Combat

- In `resolve_ground()`, sum officer combat ratings on commanding fleet
- Apply as multiplier: `troop_damage *= 1.0 + (officer_sum / 200.0)`
- Tests (2): officer_combat_bonus, no_officer_no_bonus

### 2.5 Files

| File | Change |
|------|--------|
| `crates/rebellion-core/src/repair.rs` | NEW — ~400 LOC |
| `crates/rebellion-core/src/death_star.rs` | Shield generator |
| `crates/rebellion-core/src/combat.rs` | DS shield check + officer rating |
| `crates/rebellion-core/src/missions.rs` | Decoy support |
| `crates/rebellion-core/src/world/mod.rs` | ShipEntry hull_damage |

---

## Phase 3: P2 Story + Event Completeness

**Gate: eval score >= 0.72, all 4 story chains emit events**

### 3.1 Jabba's Palace Full Chain

Expand from 3 events to 5-case outcome switch with per-character dispatch (Leia, Luke, Chewbacca, Han). Cases: escape_self, escape_rescue, captured_by_faction, captured_generic, jabba_captures_rescuer.

### 3.2 Final Battle with Emperor

Add Emperor Palpatine co-location requirement (Luke + Vader + Emperor at same system). New `EventCondition::CharactersCoLocated { characters: Vec<CharacterKey> }`.

### 3.3 Missing Event IDs (15+)

Register handlers for: 0x100 (support change), 0x105 (fleet arrived), 0x106 (character health), 0x107 (units deployed), 0x128 (HQ captured), 0x153 (informants), 0x154/0x155 (resources/disaster), 0x160 (manufacturing idle), 0x200 (Han rescue), 0x230 (Emperor arrival), 0x231 (Jabba prisoners), 0x304 (maintenance shortfall), 0x305 (saboteurs), 0x306 (character killed), 0x361 (traitor), 0x362 (Leia Force), 0x386 (side-change).

### 3.4 Files

| File | Change |
|------|--------|
| `crates/rebellion-core/src/story_events.rs` | Jabba full chain, Final Battle Emperor, Leia Force |
| `crates/rebellion-core/src/events.rs` | CharactersCoLocated, 15+ event IDs |

---

## Phase 4: Open Souls Refactor — PerceptionIntegrator

**Gate: eval score unchanged (>= 0.72), simulation.rs reduced from ~1,400 to ~200 LOC**

### 4.1 PerceptionIntegrator (NEW: integrator.rs)

Centralizes effect dispatch. Replaces 7 scattered `apply_*` functions:

```rust
pub struct PerceptionIntegrator {
    effects: Vec<GameEffect>,
    transaction_log: Vec<TransactionEntry>,
}

impl PerceptionIntegrator {
    pub fn collect(&mut self, effects: Vec<GameEffect>) { ... }
    pub fn integrate(&mut self, world: &mut GameWorld, states: &mut SimulationStates,
                     tick: u64, wall_ms: u64) -> Vec<GameEventRecord> { ... }
}
```

### 4.2 RollCursor (Functional RNG)

Replace closure-based `take_rolls` with typed cursor for explicit RNG consumption tracking.

### 4.3 Pure Tick Composition

`run_simulation_tick()` becomes 17 `integrator.collect(System::tick(...))` calls + one `integrator.integrate()`.

### 4.4 Files

| File | Change |
|------|--------|
| `crates/rebellion-data/src/integrator.rs` | NEW — ~500 LOC |
| `crates/rebellion-data/src/simulation.rs` | Refactor to use integrator |

---

## Phase 5: P3 Multiplayer Protocol Foundation

**Gate: 100% telemetry coverage, all SYS_* constants appear in playtest JSONL**

### 5.1 NetMessage Enum (NEW: net_protocol.rs)

178 `net_notify_*` message types as Rust enum. Protocol definitions only — no networking implementation.

Categories: character state (~30), game objects (~25), system state (~30), missions (~15), manufacturing (~10), story (~15), roles (~15), fleets (~5), facilities (~5), research (~4).

### 5.2 Telemetry Coverage Test

Integration test verifying every SYS_* constant emits at least one event in a 1000-tick dual-AI playtest.

### 5.3 Files

| File | Change |
|------|--------|
| `crates/rebellion-core/src/net_protocol.rs` | NEW — 178 variants |
| Test files | Telemetry coverage verification |

---

## Codex Agent Work Packages

### Stream 1: P0 Formula Fixes (Agent-C)
```bash
~/.claude/skills/codex-orchestrator/scripts/codex-exec.sh builder \
  "STREAM 1: own crates/rebellion-core/src/missions.rs only. Fix the 5 mission \
  parity formulas and add 5 unit tests. Add local helpers: sabotage_rating, \
  counter_intel_rating, target_defense_rating, target_resistance_rating. \
  Do not touch simulation.rs, main.rs, world/mod.rs, or game_events.rs." --full-auto
```
**Files owned exclusively**: `missions.rs`
**Shared interfaces**: Keep changes local to `compute_table_input()`; missing `target_character` paths default to zero subtraction
**Acceptance**: 5 new tests pass, existing mission tests pass, `cargo check` clean

### Stream 2: Effect Trait + EffectRunner (Agent-A)
```bash
~/.claude/skills/codex-orchestrator/scripts/codex-exec.sh builder \
  "STREAM 2: own new effect runtime files only. Create effects.rs (rebellion-core), \
  effect_runner.rs (rebellion-data), and ui_effect_sink.rs (rebellion-app). Design \
  unified Effect trait: system(), event_type(), apply(&mut ApplyCtx), record(&TelemetryCtx). \
  Wire shared merge points in simulation.rs and main.rs — remove duplicated apply helpers \
  at simulation.rs:811 and main.rs:2344. Do not change domain logic." --full-auto
```
**Files owned exclusively**: `effects.rs` (new), `effect_runner.rs` (new), `ui_effect_sink.rs` (new)
**Shared merge points**: `simulation.rs`, `main.rs`, `lib.rs` (rebellion-core)
**Acceptance**: Both headless and UI paths use one `EffectRunner::apply_all`; no world mutation logic duplicated between app and data crates

### Stream 3: Economy Tick (Agent-B)
```bash
~/.claude/skills/codex-orchestrator/scripts/codex-exec.sh builder \
  "STREAM 3: own new crates/rebellion-core/src/economy.rs only. Implement the \
  per-system economy tick with 18 helpers, tests, and EconomySystem::advance API. \
  Support drift uses GNPRTB 7686-7688 and 7732-7737. Collection rate uses GNPRTB 7763. \
  KDY, join-side, garrison requirement, fleet/fighter modifiers implemented. Place \
  economy BEFORE manufacturing in tick order. Leave simulation.rs/lib.rs/game_events.rs \
  as shared merge points." --full-auto
```
**Files owned exclusively**: `economy.rs` (new)
**Shared merge points**: `simulation.rs`, `effects.rs`, `game_events.rs`, `lib.rs`
**Acceptance**: 12 new tests pass, economy events emitted, economy runs before manufacturing in tick order

### Stream 4: Telemetry Constants (Agent-D)
```bash
~/.claude/skills/codex-orchestrator/scripts/codex-exec.sh builder \
  "STREAM 4: own crates/rebellion-core/src/game_events.rs only. Add 15 missing \
  EVT_* constants (economy_tick, support_drift, collection_rate, ship_repair, \
  decoy_mission, death_star_shield, maintenance_shortfall, manufacturing_idle, \
  fleet_arrived, units_deployed, character_health, informant_intel, resource_discovery, \
  character_killed, system_side_change). Add SYS_ECONOMY, SYS_REPAIR. Update uniqueness \
  test to cover EVT_CONTROL_CHANGED and EVT_CAMPAIGN_SNAPSHOT." --full-auto
```
**Files owned exclusively**: `game_events.rs`
**Acceptance**: All new constants compile, uniqueness test covers full constant set

### Stream 5: Combat Completeness (Agent-F)
```bash
~/.claude/skills/codex-orchestrator/scripts/codex-exec.sh builder \
  "STREAM 5: own crates/rebellion-core/src/combat.rs and death_star.rs only. \
  Add shield-generator gating using DefenseFacilityInstance.class_dat_id.family() == 0x25. \
  Ship repair uses existing damage_control field, never exceeds class hull. \
  Ground combat adds present-character combat modifier. Winner resolution gets \
  Emperor Palpatine check. Deterministic tests for shield block/unblock, repair, \
  ground modifier, Palpatine win. Leave simulation.rs and telemetry to merge points." --full-auto
```
**Files owned exclusively**: `combat.rs`, `death_star.rs`
**Shared**: Expose `advance_repairs` for Stream 2 to wire; consume telemetry constants from Stream 4
**Acceptance**: DS firing blocked while 0x25 shield exists; repair never exceeds class hull; deterministic tests pass

### Parallel Schedule + Merge Order

```
Session 1 (parallel): Stream 4 (Telemetry) || Stream 1 (P0 Formulas)
  → merge: game_events.rs constants + missions.rs fixes
Session 2 (parallel): Stream 2 (Effect Trait) || Stream 3 (Economy) || Stream 5 (Combat)
  → merge: effects.rs + economy.rs + combat changes into simulation.rs
Session 3: Story events (Phase 3) — depends on events.rs from Session 1
Session 4: PerceptionIntegrator (Phase 4) — depends on all Phase 1-3
Session 5: Protocol defs (Phase 5) || Telemetry coverage tests
```

**Dependency matrix:**
- Stream 1 → independent (no deps)
- Stream 4 → independent (no deps), but consumed by Streams 2, 3, 5
- Stream 2 → depends on Stream 4 (telemetry constants)
- Stream 3 → depends on Stream 4 (SYS_ECONOMY), Stream 2 (Effect trait)
- Stream 5 → depends on Stream 4 (telemetry constants)

---

## Success Metrics

| Phase | Eval Score | Tests | Event Types | Parity |
|-------|-----------|-------|-------------|--------|
| Baseline | 0.6483 | 328 | 26 | ~85% |
| Phase 0 | 0.6483 | 328 | 38 | ~85% |
| Phase 1 | >= 0.67 | >= 345 | 38 | ~90% |
| Phase 2 | >= 0.70 | >= 370 | 38 | ~93% |
| Phase 3 | >= 0.72 | >= 390 | 38 | ~95% |
| Phase 4 | >= 0.72 | >= 390 | 38 | ~95% |
| Phase 5 | >= 0.72 | >= 400 | 38 | ~97% |

## Verification

1. `PATH="/usr/bin:$PATH" cargo test` — all tests pass after each phase
2. `PATH="/usr/bin:$PATH" cargo run -p rebellion-playtest -- data/base --seed 42 --ticks 5000 --dual-ai --output /tmp/eval.jsonl` then `python3 scripts/eval_game_quality.py /tmp/eval.jsonl` — score meets gate
3. `grep -c '"system"' /tmp/eval.jsonl | sort | uniq -c` — verify all SYS_* constants appear
4. No warnings: `PATH="/usr/bin:$PATH" cargo check 2>&1 | grep -c warning` = 0

## Critical Files

- `crates/rebellion-core/src/missions.rs` — P0 formula corrections
- `crates/rebellion-core/src/economy.rs` — NEW, biggest gap
- `crates/rebellion-core/src/effects.rs` — NEW, unified effect type
- `crates/rebellion-core/src/game_events.rs` — telemetry constants
- `crates/rebellion-data/src/simulation.rs` — tick orchestrator (refactor target)
- `crates/rebellion-data/src/integrator.rs` — NEW, PerceptionIntegrator
- `crates/rebellion-core/src/combat.rs` — DS shield + officer rating
- `crates/rebellion-core/src/death_star.rs` — shield generator
- `crates/rebellion-core/src/repair.rs` — NEW, ship repair
- `crates/rebellion-core/src/net_protocol.rs` — NEW, 178 message types
- `crates/rebellion-core/src/story_events.rs` — Jabba + Final Battle + Leia
- `crates/rebellion-core/src/events.rs` — 15+ missing event IDs
- `scripts/eval_game_quality.py` — economy metric addition
- `docs/reports/2026-03-26-community-disassembly-cross-reference.md` — source of all P0-P3 gaps
