---
title: "AI Parity Tracker"
description: "Comprehensive mapping of original REBEXE.EXE AI functions to Open Rebellion implementation status"
category: "agent-docs"
created: 2026-03-21
updated: 2026-03-21
---

# AI Parity Tracker

Maps every decompiled AI function from the original REBEXE.EXE to our Rust implementation. Each row documents: original behavior, our implementation, status, and whether we deviate/augment.

## Status Key

| Status | Meaning |
|--------|---------|
| **DONE** | Faithfully ported, behavior matches original |
| **FAITHFUL** | Intentionally matches original (even if the original was flawed) |
| **AUGMENTED** | Ported + improved beyond original. Original behavior available via config. |
| **PARTIAL** | Some aspects ported, others missing |
| **MISSING** | Not yet implemented |
| **BY DESIGN** | Intentionally different, documented why |

## Core AI Pipeline (6 Functions)

| # | Original Function | Lines | Purpose | Our Code | Status | Notes |
|---|---|---|---|---|---|---|
| 1 | `FUN_00519d00` | 252 | Galaxy-wide system bucketing into 7 categories | `evaluate_galaxy_state()` ai.rs:838 | PARTIAL | 7 buckets done, ratio scoring missing |
| 2 | `FUN_00537180` | 381 | Primary system-level entity deployment | `evaluate_fleet_deployment()` ai.rs:954 | AUGMENTED | Per-fleet scoring (weakness×proximity×deconfliction×freshness). Original was per-system iteration. |
| 3 | `FUN_005385f0` | 252 | Secondary deployment pass (redistribution) | Pass 2 in `evaluate_fleet_deployment()` ai.rs:1100 | PARTIAL | Picks first undefended system. Original proportionally distributed across ALL systems + handled stranded troops. |
| 4 | `FUN_00502020` | 897 | Garrison strength assessment (ships+troops+fac) | `system_strength()` ai.rs:809 | DONE | Simplified but correct formula |
| 5 | `FUN_00508250` | ~200 | Action validation (pre-dispatch checks) | Inline checks in fleet loop | PARTIAL | No dedicated validation pass |
| 6 | `FUN_00520580` | ~300 | Movement order issuance | `AIAction::MoveFleet` + simulation.rs | DONE | With `already_moving` dedup |

## AI Sub-Functions

| Original Function | Purpose | Our Code | Status |
|---|---|---|---|
| `FUN_005597e0` | Faction-specific system counts | `galaxy.our_controlled.len()` etc. | DONE |
| `FUN_0053e190` | Ratio/percentage scoring | Not implemented | MISSING |
| `FUN_005007a0` | Fleet unit counting | `fleet.capital_ships.iter()` | DONE |
| `FUN_005039d0` | Facility iteration | `sys.defense_facilities.len()` | DONE |
| `FUN_00504c40` | Troop iteration | `sys.ground_units.iter()` | DONE |
| `FUN_0052e970` | Secondary pass scoring | Not implemented | MISSING |
| `FUN_00508660` | Entity dispatch (10+ type handlers) | `evaluate_officers()` + `evaluate_production()` | PARTIAL — no troop/facility dispatch |
| `FUN_00506ea0` | Faction-specific evaluator object | Faction asymmetry in attack targeting only | PARTIAL — no faction-specific Pass 2 |
| `FUN_005202d0` | System pre-validation | Not implemented | MISSING |
| `FUN_004927c0` | Master turn processing (research dispatch) | `evaluate_research()` ai.rs — assigns characters by skill to Ship/Troop/Facility trees | DONE |

## AI Behavioral Properties

| Property | Original | Ours | Status |
|---|---|---|---|
| Map visibility | Omniscient (no fog check) | Omniscient (no FogState param) | FAITHFUL |
| Evaluation frequency | Unknown (possibly per-tick) | Every 7 ticks (`AI_TICK_INTERVAL`) | BY DESIGN |
| Target selection scope | Per-system (distributed) | Per-fleet scoring (4 factors) | AUGMENTED |
| Deployment passes | Two-pass (primary + secondary) | Single pass | MISSING |
| Garrison scoring inputs | Ships + troops + facilities | Ships + troops + facilities | DONE |
| Faction ownership encoding | 2-bit field at entity+0x24 | `ControlKind` enum (4 states) | AUGMENTED |
| Fleet movement model | Direct point-to-point | Euclidean distance-based | AUGMENTED |
| Character assignment | Unknown specifics | Role-based (Jedi, diplomat, espionage) | AUGMENTED |
| Production priority | Unknown specifics | Capital ships > fighters > yards | AUGMENTED |

## Augmentations (Beyond Original)

These are improvements we've added that the original game did not have:

| Feature | Why | Config Flag (planned) |
|---|---|---|
| `ControlKind::Contested` and `Uprising` states | Original only had 3 states (neutral/Alliance/Empire) | Always on |
| Distance-based fleet transit | Original hop model was decoded but our Euclidean model is more realistic | `movement.distance_scale` |
| Starting force distribution (10/3) | Original used GNPRTB params we haven't fully decoded | `seeds.empire_starting_systems` |
| HQ garrison defense | Original may have had this — unclear from RE | Always on |
| Per-fleet attack scoring (planned) | Original had per-system iteration — this is our port of that concept | `ai.per_fleet_targeting` |
| Battle repeat penalty (planned) | Anti-stagnation — original had no such mechanism | `augmentation.battle_repeat_penalty` |
| Target deconfliction (planned) | Prevents fleet piling — original may have handled via two-pass | `augmentation.target_deconfliction` |
| Role-based character AI | Original assignment logic not fully decoded | Always on |
| Faction-asymmetric doctrine | Empire→HQ strike, Alliance→guerrilla | `ai.faction_asymmetry` |

## Validation Process

To verify parity:

1. **Run headless campaign**: `cargo run -p rebellion-playtest -- data/base --seed 42 --ticks 5000 --dual-ai --summary`
2. **Check eval script**: `python3 scripts/eval_game_quality.py campaign.jsonl`
3. **Verify non-degenerate**: Score > 0.0, battles at 3+ systems
4. **Cross-reference matrix**: Every row in Core AI Pipeline must be DONE/FAITHFUL/AUGMENTED
5. **Document deviations**: Every AUGMENTED row must explain why and have a config flag

## How to Update This Document

When modifying AI code:
1. Identify which original function the change corresponds to
2. Update the Status column
3. If adding new behavior not in the original, add to Augmentations table
4. Run eval script to verify the change improves (or doesn't regress) the quality score
