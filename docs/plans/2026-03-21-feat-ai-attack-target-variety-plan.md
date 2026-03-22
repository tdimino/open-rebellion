---
title: "feat: AI Attack Target Variety — Port Original + Augment"
type: feat
status: active
date: 2026-03-21
project: open-rebellion
tags: [ai, fleet-deployment, targeting, deconfliction]
---

# AI Attack Target Variety

## Overview

All 991 space battles in a 5000-tick dual-AI campaign occur at a single system (Yavin). The AI computes one `attack_target` for the entire faction and sends every fleet there. The eval script correctly flags this as degenerate. This plan ports the original game's distributed target selection faithfully, then augments with anti-stagnation improvements.

## Problem Statement

**Root cause** (`ai.rs:907-916`): A single `attack_target` is computed per faction per evaluation. Every remaining fleet after HQ garrison and one reinforcement is sent to the same system.

**Evidence**: Seed 42, 5000 ticks — 991 `combat_space` all at Yavin, 10 attack orders vs 1,348 reinforce, 0 control changes, 188 neutral systems. Evaluator score: 0.00 (degenerate).

**Original game behavior** (from Ghidra RE of REBEXE.EXE):
- `FUN_00519d00`: Global galaxy evaluation — categorizes ALL 200 systems into 7 strategic buckets
- `FUN_00537180`: Primary system-level deployment — iterates systems, issues movement orders per-fleet
- `FUN_005385f0`: Secondary deployment pass — handles redistribution and edge cases
- `FUN_00502020`: Garrison strength — counts ships + troops + facilities (897 lines)
- AI was **omniscient** (no fog check), **distributed** (per-system not per-faction targeting), **two-pass**

## Parity Tracking Matrix

To ensure we can measure/document exactly where we match vs deviate from the original:

| Original Behavior | Ghidra Function | Our Implementation | Status | Deviation |
|---|---|---|---|---|
| Global galaxy bucketing (7 categories) | FUN_00519d00 | `evaluate_galaxy_state()` | PARTIAL — 7 buckets but no ratio scoring |  |
| Per-system deployment iteration | FUN_00537180 | `evaluate_fleet_deployment()` | MISSING — uses per-faction, not per-system |  |
| Two-pass deployment | FUN_005385f0 | Not implemented | MISSING |  |
| Garrison strength (ships+troops+fac) | FUN_00502020 | `system_strength()` | DONE — simplified but correct |  |
| Faction ownership check (bit pattern) | `entity+0x24 >> 6 & 3` | `ControlKind` enum | DONE — improved (4 states vs 3 bits) |  |
| Omniscient map vision | No fog check in FUN_00519d00 | No `FogState` param in AI | FAITHFUL — matches original |  |
| ~20 tunable thresholds | Various constants in decompiled C | 7 hardcoded constants | PARTIAL — need config externalization |  |
| Fleet-level target selection | FUN_00520580 movement orders | Single `attack_target` for all | BROKEN — root cause of single-system combat |  |
| Ratio/percentage scoring | FUN_0053e190 | Not implemented | MISSING |  |
| Linked-list system iteration | FUN_00537180 inner loop | `world.systems.iter()` | EQUIVALENT |  |

**Rule**: Every cell in "Status" must be DONE, FAITHFUL, or AUGMENTED before shipping. PARTIAL and MISSING cells are the work items.

## Proposed Solution

### Phase 1: Per-Fleet Target Selection (CRITICAL)

Replace the single `attack_target` with per-fleet targeting. Each fleet picks its own target based on a scoring function:

```rust
fn score_attack_target(
    world: &GameWorld,
    fleet: &Fleet,
    fleet_location: SystemKey,
    target: SystemKey,
    targeted_count: &HashMap<SystemKey, usize>,
    battle_history: &HashMap<SystemKey, u64>,
    current_tick: u64,
) -> f64 {
    let weakness = 1.0 / (1.0 + system_strength(world, target_sys, !is_alliance) as f64);
    let distance = euclidean_distance(fleet_location, target);
    let proximity = 1.0 / (1.0 + distance / 100.0);
    let deconfliction = 1.0 / (1.0 + *targeted_count.get(&target).unwrap_or(&0) as f64);
    let freshness = battle_decay(battle_history, target, current_tick);

    weakness * 0.3 + proximity * 0.3 + deconfliction * 0.25 + freshness * 0.15
}
```

**Files**: `crates/rebellion-core/src/ai.rs` — rewrite `evaluate_fleet_deployment()`

### Phase 2: Target Deconfliction

Track which systems are already targeted **transiently** (rebuilt each eval pass from `MovementState.orders()`):

```rust
// At start of evaluate_fleet_deployment:
let mut targeted_counts: HashMap<SystemKey, usize> = HashMap::new();
for (_, order) in movement.orders() {
    if /* fleet belongs to our faction */ {
        *targeted_counts.entry(order.destination).or_default() += 1;
    }
}
// Each fleet assignment increments the count
```

**Design decision**: Transient, not persistent. Avoids stale-state bugs (fleet arrives, combat resolves, but target still marked). Rebuilt from live `MovementState` each eval.

### Phase 3: Two-Pass Deployment (Original Parity)

Port the original's two-pass pattern:

**Pass 1** (primary): Assign fleets to attack targets using the scoring function. Each fleet picks its highest-scoring target. HQ garrison assigned first.

**Pass 2** (secondary): Redistribute idle fleets. If any fleet has no assignment (all targets deconflicted away, or fleet just won combat), assign to: (a) consolidate with nearest friendly fleet, (b) reinforce weakest friendly system, or (c) hold position.

### Phase 4: Battle Repeat Penalty (AUGMENTATION)

Add `battle_cooldowns: HashMap<SystemKey, u64>` to `AIState` (`#[serde(default)]` for save compat):

```rust
fn battle_decay(history: &HashMap<SystemKey, u64>, target: SystemKey, current_tick: u64) -> f64 {
    match history.get(&target) {
        Some(&last_battle_tick) => {
            let ticks_since = current_tick.saturating_sub(last_battle_tick);
            (ticks_since as f64 / 100.0).min(1.0) // Full recovery after 100 ticks
        }
        None => 1.0, // Never fought here — full priority
    }
}
```

Update after combat: `ai_state.battle_cooldowns.insert(sys_key, current_tick);`

### Phase 5: Retarget Cooldown

Don't re-issue movement orders to fleets that are already in transit AND their destination still makes strategic sense. Only retarget if:
- Destination was captured by our faction during transit (no longer an enemy target)
- Destination was destroyed (Death Star)
- Fleet has been in transit > `RETARGET_COOLDOWN_TICKS` (default 50) without arriving

### Phase 6: Movement State Access

`AISystem::advance` needs read access to `MovementState` for deconfliction. Solve borrow checker by destructuring `SimulationStates` at call sites:

```rust
// In simulation.rs:
let ai_actions = AISystem::advance(
    &mut states.ai,
    world,
    &states.manufacturing,
    &states.missions,
    &states.movement,  // NEW: read-only access
    tick_events,
    current_tick,       // NEW: for battle decay
);
```

### Phase 7: Death Star Exemption

```rust
if fleet.has_death_star {
    // Death Star always targets enemy HQ (decisive strike doctrine)
    if let Some(hq) = galaxy.enemy_hq {
        actions.push(AIAction::MoveFleet { fleet: fleet_key, to_system: hq, reason: Attack });
    }
    continue;
}
```

## Edge Cases (from SpecFlow Analysis)

| Edge Case | Resolution |
|-----------|-----------|
| System flips faction during fleet transit | Fleet arrives at now-friendly system, treated as garrison. No cooldown penalty. |
| All enemy targets deconflicted away | Pass 2 assigns to consolidation or garrison. Never leave fleet idle. |
| Death Star fleet in generic targeting | Exempt — dedicated HQ-targeting logic. |
| Fleet wins combat, system captured | Deconfliction rebuilt from `MovementState` next eval — system no longer targeted. |
| Save/load with new AIState fields | `#[serde(default)]` on all new fields. No save version bump needed. |
| Borrow checker: mutable AI + immutable Movement | Destructure `SimulationStates` at call site. |
| Battle repeat penalty never decays | 100-tick linear decay to full priority. |
| AI omniscience vs fog | FAITHFUL to original: AI sees all systems. Document as parity choice. |
| Max simultaneous fronts | Cap at `min(fleet_count, 3)` attack targets to avoid spreading too thin. |

## Acceptance Criteria

### Functional
- [ ] Per-fleet targeting: different fleets attack different systems
- [ ] Target deconfliction: no more than 2 fleets at the same target (unless consolidating)
- [ ] Two-pass deployment: pass 1 assigns, pass 2 redistributes idle fleets
- [ ] Battle repeat penalty: recently-fought systems deprioritized
- [ ] Death Star exempt from generic targeting
- [ ] HQ always garrisoned (existing behavior preserved)

### Metrics (from eval_game_quality.py)
- [ ] `combat_spread` > 0.5 (battles at 3+ distinct systems)
- [ ] `fleet_engagement` > 0.2 (attack orders > 20% of fleet moves)
- [ ] `control_dynamism` > 0.3 (some control changes occur)
- [ ] Game NOT flagged as degenerate by eval script
- [ ] Composite score > 0.40 (up from 0.00)

### Parity
- [ ] Every row in the Parity Tracking Matrix is DONE, FAITHFUL, or AUGMENTED
- [ ] Augmentations clearly labeled in code with `// AUGMENTATION:` comments
- [ ] Parity behaviors documented in `agent_docs/systems/ai-fleet-deployment.md`

### Tests
- [ ] Unit test: 3 fleets target 3 different enemy systems
- [ ] Unit test: deconfliction prevents 3rd fleet from piling on when 2 targets exist
- [ ] Unit test: Death Star fleet targets enemy HQ regardless of weakness scoring
- [ ] Unit test: battle repeat penalty reduces score for recently-fought system
- [ ] Integration: 5000-tick campaign produces battles at 3+ distinct systems
- [ ] All existing 280 tests pass

## Implementation Order

1. Add `&MovementState` + `current_tick` to `AISystem::advance` signature → fix both call sites + 13 tests
2. Build transient deconfliction map from `MovementState.orders()`
3. Implement `score_attack_target()` scoring function
4. Rewrite fleet loop to use per-fleet scoring instead of single target
5. Add pass 2 (redistribute idle fleets)
6. Add `battle_cooldowns` to `AIState` with `#[serde(default)]`
7. Wire battle_cooldowns update in simulation.rs after combat
8. Add Death Star exemption guard
9. Run eval_game_quality.py on seed 42 — verify non-degenerate
10. Run full test suite — verify 280 pass
11. Update parity tracking matrix
12. Document in agent_docs/

## Files Modified

| File | Changes |
|------|---------|
| `crates/rebellion-core/src/ai.rs` | Rewrite `evaluate_fleet_deployment`, add `score_attack_target`, add `battle_cooldowns` to `AIState`, Death Star guard |
| `crates/rebellion-data/src/simulation.rs` | Update `AISystem::advance` call with `&states.movement` + `current_tick`, wire `battle_cooldowns` after combat |
| `crates/rebellion-app/src/main.rs` | Update `AISystem::advance` call signature |

## References

- Ghidra RE: `ghidra/notes/ai-behavior-analysis.md`
- Codex planner: `docs/plans/2026-03-21-autoresearch-metrics-plan.md`
- Eval script: `scripts/eval_game_quality.py`
- Current AI: `crates/rebellion-core/src/ai.rs:896-1022`
- Original plan: `~/.claude/plans/cozy-petting-eagle.md` (Phase 5: Faction Asymmetry)
