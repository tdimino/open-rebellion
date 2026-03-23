---
title: "AI Parity Tracker"
description: "Comprehensive mapping of original REBEXE.EXE AI functions to Open Rebellion implementation status"
category: "agent-docs"
created: 2026-03-21
updated: 2026-03-23
tags: [ai, parity, ghidra, fleet-deployment]
---

# AI Parity Tracker

Maps every decompiled AI function from the original REBEXE.EXE to our Rust implementation. Each row documents: original behavior, our implementation, status, and whether we deviate/augment.

Updated 2026-03-23 with full Ghidra RE of all previously-unknown functions. Cross-referenced against TheArchitect2018's wiki.

## Status Key

| Status | Meaning |
|--------|---------|
| **DONE** | Faithfully ported, behavior matches original |
| **FAITHFUL** | Intentionally matches original (even if the original was flawed) |
| **AUGMENTED** | Ported + improved beyond original. Original behavior available via config. |
| **PARTIAL** | Some aspects ported, others missing |
| **MISSING** | Not yet implemented |

## Core AI Pipeline (6 Functions)

| # | Original Function | Lines | Purpose | Our Code | Status | Notes |
|---|---|---|---|---|---|---|
| 1 | `FUN_00519d00` | 252 | Galaxy-wide system bucketing into 7 categories | `evaluate_galaxy_state()` ai.rs | DONE | 7 buckets + control_ratio + aggression scaling |
| 2 | `FUN_00537180` | 381 | Primary system-level entity deployment | `evaluate_fleet_deployment()` ai.rs | AUGMENTED | Per-fleet scoring (weakness×proximity×deconfliction×freshness). Original was per-system iteration with capacity check only. **Our model is superior** — original has no weighted scoring. |
| 3 | `FUN_005385f0` | 252 | Secondary deployment pass (redistribution) | Pass 2 in `evaluate_fleet_deployment()` ai.rs | AUGMENTED | Original calls FUN_0052e970 (capacity check) + FUN_00506ea0 (faction evaluator). Our aggression-scaled redistribution is more nuanced. |
| 4 | `FUN_00502020` | 897 | Garrison strength assessment (ships+troops+fac) | `system_strength()` ai.rs | DONE | Simplified but correct formula |
| 5 | `FUN_00508250` | 139 | Action validation (18 AND-chained checks) | `can_dispatch()` ai.rs | PARTIAL | 6 of 18 checks implemented. See Dispatch Validators section for full decode of all 18. |
| 6 | `FUN_00520580` | 9 | Movement order issuance (2-field struct setter) | `AIAction::MoveFleet` + simulation.rs | DONE | Original is just a command setter — `*(this) = cmd_type; *(this+4) = param`. |

## AI Sub-Functions (All Decoded 2026-03-23)

| Original Function | Lines | Purpose | Our Code | Status | Ghidra Notes |
|---|---|---|---|---|---|
| `FUN_005597e0` | — | Faction-specific system counts | `galaxy.our_controlled.len()` etc. | DONE | — |
| `FUN_0053e190` | — | Ratio/percentage scoring | `GalaxyState.control_ratio` + `aggression` | DONE | — |
| `FUN_005007a0` | — | Fleet unit counting | `fleet.capital_ships.iter()` | DONE | — |
| `FUN_005039d0` | — | Facility iteration | `sys.defense_facilities.len()` | DONE | — |
| `FUN_00504c40` | — | Troop iteration | `sys.ground_units.iter()` | DONE | — |
| `FUN_0052e970` | 53 | **Secondary pass capacity check** | `score_attack_target()` | AUGMENTED | **Not a scoring function.** Checks if entity (family 0x10-0x3f) fits deployment budget (`this+0x58 - this+0x5c`). Calls FUN_0053b870 (reads +0x4c = capacity). Dispatches cmd 0xf3 if room. **Our 4-factor model is superior.** |
| `FUN_00506ea0` | 13 | Faction-specific evaluator pointer | Faction asymmetry in attack targeting | AUGMENTED | Returns evaluator from global `DAT_006b2bb0`: Alliance at +0xc4, Empire at +0xc8. Different deployment budgets per faction. **TODO**: Add faction-specific budget thresholds to AiConfig. |
| `FUN_00508660` | 304 | Entity dispatch (10+ type handlers) | `evaluate_officers()` + `evaluate_production()` + `evaluate_research()` | DONE | Routes by family byte: chars (0x08-0x0f), troops (0x10-0x13), facilities (0x1c-0x1f), ships (0x20-0x2f). Our split into 3 functions is cleaner. |
| `FUN_005202d0` | 39 | System pre-validation wrapper | `can_dispatch()` | DONE | Exception-handling wrapper around FUN_00520580 + FUN_0051fcb0. |
| `FUN_004927c0` | 2098 | Master turn processing | `AISystem::advance()` | DONE | **Event-driven via message 0x1f0 (day tick).** AI evaluates every game-day, not on an interval. Our AI_TICK_INTERVAL=7 is an augmentation. |
| `FUN_0053b870` | 7 | Entity capacity reader | Not separate — inlined | DONE | Returns `*(entity + 0x4c)` — the entity's capacity/count field. |
| `FUN_0050ac80` | 49 | Character placement handler | Part of `evaluate_officers()` | DONE | Faction-conditioned placement with priority checking. |
| `FUN_0050a1b0` | 28 | Faction-conditioned entity placement | Part of `evaluate_officers()` | DONE | Checks faction match before placing entity. |
| `FUN_0050c9f0` | 32 | Ship placement handler | Part of `evaluate_production()` | DONE | Fleet capacity check before ship placement. |
| `FUN_0050c6c0` | 69 | Facility placement handler | Part of `evaluate_production()` | DONE | Slot iteration + nested fleet check for facility placement. |

## Dispatch Validators — FUN_00508250 (18 AND-Chained Checks)

All 18 sub-functions decompiled 2026-03-23 via Ghidra MCP. Each returns bool; all must pass (AND chain).

| # | Address | Lines | Decoded Purpose | Our Status |
|---|---------|-------|----------------|------------|
| 1 | `FUN_0051ebb0` | 7 | **Always returns 1** (no-op gate) | N/A — skip |
| 2 | `FUN_0050ad60` | 13 | Capacity check: `this+0x5c < this+0x64` → validate overflow | MISSING |
| 3 | `FUN_0050ad80` | 139 | Fleet entity count vs capacity at `+0x5c` (most complex validator) | MISSING |
| 4 | `FUN_0050b0b0` | 65 | Entity count via vtable+0x1c8 vs budget at `+0x64` | MISSING |
| 5 | `FUN_0050b230` | 36 | Faction check (+0x24>>6&3) + status bits (+0x88>>11) + multi-param scoring | PARTIAL (faction check done) |
| 6 | `FUN_0050b2c0` | 27 | Faction check + loyalty scoring via FUN_00559c10 | MISSING |
| 7 | `FUN_0050b310` | 73 | Ship type compatibility: fleet count + facility count + bit5 of +0x88 | MISSING |
| 8 | `FUN_0050b610` | 77 | Troop deployment: +0x88 bit0, troop class via FUN_0055a080 | MISSING |
| 9 | `FUN_0050b5a0` | 33 | Faction + status bits (+0x88 bits 0,2,11) + scoring | PARTIAL (faction done) |
| 10 | `FUN_0050b500` | 42 | Fleet composition: troop count - allocation at +0x80 | MISSING |
| 11 | `FUN_0050ba90` | 26 | Troop iterator + boolean availability | MISSING |
| 12 | `FUN_0050bb70` | 15 | Ship capacity check via FUN_00528040 | MISSING |
| 13 | `FUN_0050bc60` | 87 | Character iteration (family 4) + faction match + counting | DONE (faction match) |
| 14 | `FUN_0050be00` | 99 | Character iteration (family 4) + mandatory mission check | DONE (on_mandatory_mission) |
| 15 | `FUN_0050c350` | 95 | Fleet/facility nested iteration + per-entity FUN_0050c580 check | MISSING |
| 16 | `FUN_0050b8e0` | 89 | System-level strength scoring: both factions via FUN_00509710 | MISSING |
| 17 | `FUN_0050b800` | 34 | Status bits (+0x88 bits 0,2) + position check (+0x7c ≥ 0) | MISSING |
| 18 | `FUN_0050bb00` | 28 | Faction + status bits + position → deployment flag | MISSING |

**Summary**: 4 of 18 checks implemented (2 no-ops + faction match + mandatory mission). 14 remaining are mostly capacity/composition checks. Many reference entity offsets (+0x58, +0x5c, +0x64, +0x80, +0x88) that map to internal allocation budgets and status bitfields not yet modeled in our Rust types.

## AI Behavioral Properties (All Resolved)

| Property | Original (Decoded) | Ours | Status | Decision |
|---|---|---|---|---|
| Map visibility | Omniscient (no fog param in any AI fn) | Omniscient (no FogState param) | FAITHFUL | — |
| Evaluation frequency | **Every game-day** (event 0x1f0 triggers per-day) | Every 7 ticks (`AI_TICK_INTERVAL`) | AUGMENTED | Keep throttled — per-tick with 200 systems is expensive. Config flag `ai.tick_interval` exists. |
| Target selection scope | **Binary capacity check** (FUN_0052e970: fits budget?) | Per-fleet scoring (4 weighted factors) | AUGMENTED | **Our model is superior.** Original has no scoring — just checks if entity fits deployment budget. Keep our 4-factor model. |
| Deployment passes | Two-pass with faction-specific budgets | Two-pass with aggression scaling | AUGMENTED | Original uses different evaluator objects per faction (FUN_00506ea0). Our aggression model is more dynamic. |
| Garrison scoring inputs | Ships + troops + facilities via vtable dispatch | Ships + troops + facilities (simplified) | DONE | — |
| Faction ownership encoding | 2-bit field at entity+0x24 bits 6-7 | `ControlKind` enum (4 states) | AUGMENTED | We add Contested + Uprising states (original has 3). |
| Fleet movement model | **FUN_00520580 is just a struct setter** (not transit calc) | Euclidean distance-based | AUGMENTED | Transit calc is elsewhere in event chain. Our model is intentional upgrade. |
| Character assignment | Family-byte routing (FUN_00508660): chars, troops, facilities, ships each have specific handlers | Role-based (Jedi, diplomat, espionage) + production priority | AUGMENTED | Original routes by entity type. Our skill-based dispatch is cleaner. |
| Production priority | Implicit in entity handler routing order | Capital ships > fighters > yards > troops > defenses | AUGMENTED | Original doesn't have explicit priority — it handles whatever entity type the iterator yields. |
| Faction-specific budgets | **Alliance +0xc4, Empire +0xc8** on evaluator object — different deployment budget thresholds | Same budget for both | PARTIAL | **TODO**: Add faction-specific deployment budgets to AiConfig. |
| Mission probability | Composite input formulas per mission type (wiki: sub_55ae50 etc.) | Composite input (ported 2026-03-23) | DONE | Diplomacy, recruitment, subdue, DS sabotage formulas ported from TheArchitect2018 wiki. |

## Augmentations (Beyond Original — Justified Improvements)

| Feature | Original Behavior | Our Improvement | Why Keep | Config Flag |
|---|---|---|---|---|
| `ControlKind::Contested` + `Uprising` | 3 states (neutral/Alliance/Empire) | 4 states | More strategic depth | Always on |
| Euclidean fleet transit | Struct setter (FUN_00520580), transit calc elsewhere | Distance-based with configurable scale | Tunable, realistic | `movement.distance_scale` |
| 4-factor attack scoring | Binary capacity check (FUN_0052e970) | Weakness × proximity × deconfliction × freshness | **Strictly superior** — original has no target scoring | `weight_*` (4 config flags) |
| Per-fleet targeting | Per-system iteration | Per-fleet scoring with deconfliction | Prevents fleet piling | `ai.per_fleet_targeting` |
| Battle repeat penalty | Not in original | Freshness curve prevents stagnation | Better gameplay | `augmentation.battle_repeat_penalty` |
| Target deconfliction | Not in original | Pile count tracking | Territorial spread | `augmentation.target_deconfliction` |
| Throttled AI evaluation | Every game-day (event 0x1f0) | Every 7 ticks | Performance optimization | `ai.tick_interval` |
| Role-based character AI | Family-byte type routing | Skill-based dispatch (diplomacy/espionage/combat) | More strategic | Always on |
| Faction-asymmetric doctrine | Different evaluator objects (budget only) | Empire→HQ strike, Alliance→guerrilla | Clearer doctrinal difference | `ai.faction_asymmetry` |
| Composite mission inputs | wiki: sub_55ae50 etc. (popularity + skill) | Same formulas ported | Faithful to original | N/A (parity) |

## Validation Process

1. **Run headless campaign**: `cargo run -p rebellion-playtest -- data/base --seed 42 --ticks 5000 --dual-ai --summary`
2. **Check eval script**: `python3 scripts/eval_game_quality.py campaign.jsonl`
3. **Verify non-degenerate**: Score > 0.0, battles at 3+ systems
4. **Cross-reference matrix**: Every row in Core AI Pipeline must be DONE/FAITHFUL/AUGMENTED
5. **Document deviations**: Every AUGMENTED row must explain why and have a config flag

## Sources

- Ghidra RE: 23 functions decompiled 2026-03-23 via GhidraMCP HTTP bridge
- TheArchitect2018 wiki: mission probability formulas (sub_55ae50, sub_55aed0, sub_55ae90, sub_55af50, sub_55b0a0, sub_55cfb0)
- Codex researcher comparison report (2026-03-23)
