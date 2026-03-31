# Autoresearch: Simulation Parity

You are implementing missing game mechanics in Open Rebellion, a Rust reimplementation of Star Wars Rebellion (1998). Your goal is to improve the parity eval score by implementing ONE missing feature per iteration.

## Current Parity Score

Baseline: 0.6426 (average across 3 seeds)

## Eval Sub-Metrics (what to improve)

| Sub-Metric | Current | Target | How to improve |
|------------|---------|--------|----------------|
| economy_activity | 1.0 | 1.0 | Already at max |
| mission_completeness | 0.625 | 1.0 | AI dispatches Assassination/Abduction with target_character; Rescue missions fire |
| combat_completeness | 0.333 | 1.0 | Ground combat + bombardment must occur in campaigns |
| event_coverage | 0.78 | 1.0 | uprising, betrayal, victory systems must emit events |
| system_state_completeness | 0.0 | 1.0 | Enrich campaign_snapshot with economy fields |
| repair_activity | 1.0 | 1.0 | Already at max |

## Rules

1. **Single mutable surface**: Change ONE thing per iteration. One function, one formula, one field addition.
2. **Tests must pass**: `PATH="/usr/bin:$PATH" cargo test` must pass after your change.
3. **Never touch tuning.rs or config JSON** — this is code mutation only.
4. **Never remove existing functionality** — only add or fix.
5. **Prefer the simplest change that moves the score** — don't over-engineer.

## Priority Order (try these first)

### P1: Enrich campaign_snapshot (system_state_completeness = 0.0 → 1.0)
The `campaign_snapshot` event in `crates/rebellion-data/src/integrator.rs` needs to include economy fields in its details JSON. Add these fields to the snapshot: `production_modifier`, `troop_surplus`, `has_shipyard`, `fleet_posture`, `collection_rate`. These are computed by `EconomySystem::advance()` in `crates/rebellion-core/src/economy.rs` — the `SystemSummary` struct has them.

### P2: Ground combat + bombardment occurrence (combat_completeness = 0.333 → 1.0)
Ground combat and bombardment already have systems (`crates/rebellion-core/src/combat.rs`, `crates/rebellion-core/src/bombardment.rs`). The issue is the AI rarely triggers them. Check that the headless simulation loop in `crates/rebellion-data/src/simulation.rs` actually calls ground combat resolve and bombardment when conditions are met.

### P3: Missing mission kinds (mission_completeness = 0.625 → 1.0)
The AI dispatches Diplomacy, Recruitment, Espionage, Sabotage, Assassination. Missing from campaigns: Abduction, Rescue, InciteUprising. The AI's `evaluate_espionage()` in `crates/rebellion-core/src/ai.rs` needs to dispatch these kinds when appropriate.

### P4: Missing system events (event_coverage = 0.78 → 1.0)
Systems that don't emit events: uprising, betrayal, victory. Check that `crates/rebellion-data/src/simulation.rs` calls these systems and that their advance() functions produce events under campaign conditions.

## File Map

- `crates/rebellion-core/src/` — simulation systems (pure logic, no IO)
- `crates/rebellion-data/src/simulation.rs` — tick orchestrator
- `crates/rebellion-data/src/integrator.rs` — world mutation + telemetry
- `crates/rebellion-core/src/economy.rs` — economy tick (SystemSummary)
- `crates/rebellion-core/src/combat.rs` — space + ground combat
- `crates/rebellion-core/src/bombardment.rs` — orbital bombardment
- `crates/rebellion-core/src/ai.rs` — AI dispatch (missions, production, movement)
- `crates/rebellion-core/src/missions.rs` — mission resolution
- `crates/rebellion-core/src/uprising.rs` — uprising system
- `crates/rebellion-core/src/betrayal.rs` — betrayal system
- `crates/rebellion-core/src/victory.rs` — victory conditions

## Build & Test

```bash
PATH="/usr/bin:$PATH" cargo test
PATH="/usr/bin:$PATH" cargo build -p rebellion-playtest --release
```

## Eval

```bash
./target/release/rebellion-playtest data/base --seed 42 --ticks 5000 --dual-ai --output /tmp/parity.jsonl
python3 scripts/eval_parity.py /tmp/parity.jsonl --json
```
