# Autoresearch: Simulation Parity

You are implementing missing game mechanics in Open Rebellion, a Rust reimplementation of Star Wars Rebellion (1998). Your goal is to improve the parity eval score by implementing ONE missing feature per iteration.

## Before You Start

1. Read `CLAUDE.md` in the project root for codebase conventions (advance() pattern, effects, build commands).
2. Read `crates/rebellion-core/src/game_events.rs` for event type and system tag constants.
3. Read `scripts/eval_parity.py` to understand exactly what the eval measures.

## Current Parity Score

Baseline: 0.6093 (average across 3 seeds)

## Eval Sub-Metrics

| Sub-Metric | Weight | Current | Target | How to improve |
|------------|--------|---------|--------|----------------|
| economy_activity | 0.25 | 1.0 | 1.0 | Already at max |
| mission_completeness | 0.20 | 0.625 | 1.0 | AI dispatches InciteUprising, Abduction, Rescue kinds |
| combat_completeness | 0.20 | 0.333 | 1.0 | Ground combat + bombardment events must appear in JSONL |
| event_coverage | 0.15 | 0.78 | 1.0 | uprising, betrayal, victory system tags must emit events |
| system_state_completeness | 0.15 | 0.0 | 1.0 | Enrich campaign_snapshot with economy fields |
| repair_activity | 0.05 | 1.0 | 1.0 | Already at max |

## Rules

1. **Single mutable surface**: Change ONE thing per iteration.
2. **Tests must pass**: `PATH="/usr/bin:$PATH" cargo test -p rebellion-core -p rebellion-data`
3. **FORBIDDEN files — do NOT modify**: `scripts/eval_parity.py`, `scripts/autoresearch_parity_loop.py`, `crates/rebellion-core/src/tuning.rs`, anything in `configs/`.
4. **Never remove existing functionality** — only add or fix.
5. **Prefer the simplest change that moves the score**.
6. **Run the eval after your change** to confirm the targeted sub-metric improved (see Eval section).
7. **Do not create new files or modules** — modify existing code only.

## Priority Order

### P1: Enrich campaign_snapshot (system_state_completeness = 0.0 → 1.0)

**File**: `crates/rebellion-data/src/integrator.rs`, function `emit_campaign_snapshot` (~line 188).

**What the eval expects**: The `campaign_snapshot` event's `details` JSON must contain a `"systems"` dict keyed by system name, where each value has these 5 fields:
- `production_modifier` (i8) — from `System` economy data
- `troop_surplus` (i32) — from economy `SystemSummary`
- `has_shipyard` (bool) — from economy `SystemSummary`
- `fleet_posture` (String) — from economy `SystemSummary`, serialize enum with `format!("{:?}", ...)`
- `collection_rate` (f32) — from economy data

**Where the data lives**: The economy system computes `SystemSummary` per system during `EconomySystem::advance()` in `crates/rebellion-core/src/economy.rs`. Check if the summary is stored on the `System` struct or if it needs to be recomputed. The snapshot is emitted AFTER economy runs.

**How**: In `emit_campaign_snapshot`, iterate `world.systems`, build a `serde_json::Map` keyed by system name with the 5 fields. Add as `"systems"` key in the snapshot JSON. This is a single-function ~15 LOC change.

### P2: Ground combat + bombardment (combat_completeness = 0.333 → 1.0)

**Current state**: Space combat fires correctly. Ground combat and bombardment are called in `crates/rebellion-data/src/simulation.rs` lines ~180-190 but ONLY when attacker wins space combat.

**Why they don't fire** — do NOT "check that the simulation calls them" (the calls are correct). Instead:
1. The attacker rarely wins space combat (investigate force balance).
2. Bombardment emits events only when `damage > 0`. If no bombardment-capable ships, damage is 0.
3. Ground combat requires opposing troops at the target system.

**Approach**: Trace WHY conditions aren't met. Check AI fleet composition (do attack fleets have bombardment-capable ships?), check troop seeding at contested systems, check attacker win rate.

### P3: Missing mission kinds (mission_completeness = 0.625 → 1.0)

**Currently dispatched**: Diplomacy, Recruitment, Espionage, Sabotage, Assassination (5/8).
**Missing**: InciteUprising, Abduction, Rescue.

**Pick ONE of these three per iteration**:
- **InciteUprising**: Add to `evaluate_espionage()` in `ai.rs`. Target enemy systems with low popularity. Requires espionage skill.
- **Abduction**: Like Assassination but `MissionKind::Abduction`. Target enemy characters. Add after assassination block.
- **Rescue**: Scan for captive allies (`is_captive == true`). Dispatch to system where captive is held.

**Important**: The eval checks `details.kind` on `mission_resolved` events. Kind must match enum variant name exactly (e.g. `"InciteUprising"`, `"Abduction"`, `"Rescue"`).

### P4: Missing system events (event_coverage = 0.78 → 1.0)

Missing system tags likely: `uprising`, `betrayal`, `victory`, `story`. These require specific conditions:
- **uprising**: Needs InciteUprising missions or loyalty thresholds. May need P3 first.
- **betrayal**: Needs character loyalty below threshold. May need longer campaigns.
- **victory**: Fires on game end. May already fire in some seeds.
- **story**: Needs character co-location conditions (Luke+Vader, etc.).

Pick the easiest tag to trigger. Run the eval to check which are actually missing.

## Dead Ends (do not retry these)

_None yet._

## Codebase Conventions

- **Build prefix**: `PATH="/usr/bin:$PATH"` on all cargo commands (cc shadow workaround).
- **Stateless advance**: Systems receive `&GameWorld`, return events. Integrator applies mutations.
- **Telemetry via integrator**: `self.emit(system_tag, event_type, json_payload)`.
- **System tags**: Use constants from `game_events.rs` (e.g. `SYS_COMBAT`).
- **Event types**: Use constants from `game_events.rs` (e.g. `EVT_COMBAT_GROUND`).

## File Map

- `crates/rebellion-data/src/integrator.rs` — world mutation + telemetry (~1,185 LOC)
- `crates/rebellion-data/src/simulation.rs` — tick orchestrator (~449 LOC)
- `crates/rebellion-core/src/economy.rs` — economy tick, SystemSummary, SystemEconomy
- `crates/rebellion-core/src/combat.rs` — space + ground combat
- `crates/rebellion-core/src/bombardment.rs` — orbital bombardment
- `crates/rebellion-core/src/ai.rs` — AI dispatch (missions, production, movement)
- `crates/rebellion-core/src/missions.rs` — mission resolution
- `crates/rebellion-core/src/uprising.rs` — uprising system
- `crates/rebellion-core/src/betrayal.rs` — betrayal system
- `crates/rebellion-core/src/victory.rs` — victory conditions
- `crates/rebellion-core/src/game_events.rs` — event type and system tag constants

## Build & Test

```bash
PATH="/usr/bin:$PATH" cargo test -p rebellion-core -p rebellion-data
PATH="/usr/bin:$PATH" cargo build -p rebellion-playtest --release
```

## Eval

```bash
./target/release/rebellion-playtest data/base --seed 42 --ticks 5000 --dual-ai --output /tmp/parity.jsonl
python3 scripts/eval_parity.py /tmp/parity.jsonl --json
```
