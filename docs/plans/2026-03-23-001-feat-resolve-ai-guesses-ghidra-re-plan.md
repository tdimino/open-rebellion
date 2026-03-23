---
title: "Resolve AI Guesses via Ghidra RE — Decode Every Unknown"
type: feat
status: active
date: 2026-03-23
project: open-rebellion
tags: [ai, ghidra, parity, reverse-engineering]
---

# Resolve AI Guesses via Ghidra RE — Decode Every Unknown

## Context

Open Rebellion's AI has 11 "BY DESIGN" guesses where we chose behavior without knowing the original. The decompiled C files for the critical functions either exist but are empty stubs, or haven't been decompiled yet. With Ghidra relaunched, we can systematically decode every unknown and replace guesses with faithful implementations.

Cross-referenced against TheArchitect2018's wiki — our RE is ahead on AI pipeline and combat, but the wiki adds mission probability formulas and GNPRTB accessor details we should incorporate.

## Ghidra RE Tasks (Priority Order)

### Task 1: FUN_0052e970 — Attack Target Scoring Function

**File**: `ghidra/notes/FUN_0052e970.c` (exists, empty — 1 line)
**Called from**: `FUN_005385f0` (secondary deployment pass)
**Replaces**: Our 4-factor weighted model (`score_attack_target()` in `ai.rs:1089`)
**What to decode**: Parameters, scoring formula, return type. How does the original score attack targets?

**Ghidra steps**:
1. Navigate to `0x0052e970` in REBEXE.EXE
2. Decompile fully — note all parameters and local variables
3. Trace what data it reads from the system/fleet entities (offsets)
4. Document the scoring formula
5. Save to `ghidra/notes/FUN_0052e970.c`

**Impact**: Replaces our most significant guess — the 4-factor attack scoring model.

### Task 2: FUN_00506ea0 — Faction-Specific Evaluator

**File**: Does NOT exist in repo
**Called from**: `FUN_005385f0` (secondary deployment pass)
**Replaces**: Our aggression-based Pass 2 redistribution (`ai.rs:1291-1340`)
**What to decode**: Does Alliance redistribute differently from Empire? What parameters differ?

**Ghidra steps**:
1. Navigate to `0x00506ea0`
2. Decompile — look for faction-conditional branches
3. Document Alliance vs Empire behavioral differences
4. Save to `ghidra/notes/FUN_00506ea0.c`

**Impact**: Resolves whether factions have different redistribution logic (our current code treats them the same).

### Task 3: FUN_004927c0 — Master Turn Processing (AI Tick Interval)

**File**: `ghidra/notes/FUN_004927c0.c` + `FUN_004927c0_9k.c` (58KB, exists)
**Replaces**: Our `AI_TICK_INTERVAL = 7` guess
**What to decode**: How often does the AI evaluate? Per-tick? Every N ticks? Event-driven?

**Ghidra steps**:
1. This is 9K lines — don't read it all. Search for:
   - Constants compared against tick/day counters
   - Modulo operations (`%`) on tick values
   - Conditional calls to `FUN_00519d00` (galaxy evaluation) — what gates the call?
2. Look for the pattern: `if (tick % N == 0)` or `if (tick - last_eval > N)`
3. Document the interval mechanism

**Impact**: Replaces our arbitrary 7-tick interval with the real value.

### Task 4: 18 Dispatch Validators in FUN_00508250

**File**: `ghidra/notes/FUN_00508250.c` (139 lines — the chain of 18 AND-checks)
**We implement**: 6 of 18 checks in `can_dispatch()` (`ai.rs:314-341`)
**Sub-functions to decompile** (12 are empty stubs or missing):

| # | Address | File Status | Likely Purpose |
|---|---------|-------------|----------------|
| 1 | `FUN_0051ebb0` | MISSING | Global game state check (called twice — first and 16th) |
| 2 | `FUN_0050ad60` | MISSING | Entity alive/valid check |
| 3 | `FUN_0050ad80` | empty | Faction ownership match |
| 4 | `FUN_0050b0b0` | empty | Not captured / not in transit |
| 5 | `FUN_0050b230` | empty | Not on mission |
| 6 | `FUN_0050b2c0` | MISSING | Capacity/availability check |
| 7 | `FUN_0050b310` | empty | Ship type compatibility |
| 8 | `FUN_0050b610` | empty | Troop type check |
| 9 | `FUN_0050b5a0` | 33 lines ✓ | Faction + status bit check (offset +0x88) |
| 10 | `FUN_0050b500` | empty | Fleet composition check |
| 11 | `FUN_0050ba90` | 26 lines ✓ | Troop iterator + availability |
| 12 | `FUN_0050bb70` | MISSING | Ship slot availability |
| 13 | `FUN_0050bc60` | empty | Character can_be_commander |
| 14 | `FUN_0050be00` | empty | Character not on mandatory mission |
| 15 | `FUN_0050c350` | 95 lines ✓ | Fleet/facility iteration + `FUN_0050c580` per-entity check |
| 16 | `FUN_0050b8e0` | empty | Unknown system-level check |
| 17 | `FUN_0050b800` | empty | System not under bombardment? |
| 18 | `FUN_0050bb00` | MISSING | Final gate check |

**Ghidra steps**:
1. Decompile each sub-function at its address
2. For each, document: what entity field it reads, what condition it checks, what it returns
3. Name them semantically (e.g., `check_not_captured`, `check_has_capacity`)
4. Save all to `ghidra/notes/`

**Impact**: Completes the 18-check dispatch validation — prevents invalid AI deployments.

### Task 5: FUN_00508660 Sub-Handlers — Character Assignment Logic

**File**: `ghidra/notes/FUN_00508660.c` (304 lines, exists — routing table decoded)
**Replaces**: Our simplified role-based assignment in `evaluate_officers()`
**Key sub-handlers to decode**:

| Address | Family | Purpose |
|---------|--------|---------|
| `FUN_0050ac80` | 0x08-0x0f | Character placement (major characters) |
| `FUN_0050a1b0` | multiple | Faction-conditioned placement |
| `FUN_0050b610` | 0x10-0x13 | Troop placement |
| `FUN_0050c9f0` | 0x20-0x21 | Ship type A placement |
| `FUN_0050c6c0` | 0x1c-0x1f | Facility placement |

**What to decode**: Character skill thresholds, assignment priority order, production queue logic.

**Impact**: Replaces our guessed production priority (capships > fighters > yards) and character assignment logic with the original's routing.

### Task 6: Fleet Transit Model

**Current**: Euclidean distance-based (`movement.rs`)
**Original**: "Direct point-to-point" — details unclear
**What to find**: Search `FUN_00520580` (movement order issuance) for:
- How distance is calculated (Euclidean? Manhattan? Hop-based?)
- What constants scale transit time
- Whether hyperdrive rating affects transit differently
- GNPRTB parameters used for movement

**File**: `ghidra/notes/FUN_00520580.c` — check if it exists and has content

### Task 7: GNPRTB Starting Force Parameters

**Current**: Hardcoded Empire 10 / Alliance 3 in `seeds.rs:71-73`
**What to find**: Which GNPRTB parameter IDs control starting system count and selection
**Where to look**: `FUN_004927c0` initialization section, or `sub_518b10` (BASICSD/engine-init)

## Non-Ghidra Tasks (From TheArchitect2018 Wiki)

### Task 8: Port Exact Mission Probability Formulas

The wiki provides specific input formulas that are more precise than our "MSTB probability tables with quadratic fallback":

| Mission | Formula | Wiki Source |
|---------|---------|-------------|
| Diplomacy | `input = (param_3 - param_2) + diplomacy_rating` → DIPLMSTB lookup | wiki/DIPLMSTB.DAT |
| Recruitment | `input = param_1 - param_2` → RCRTMSTB lookup | wiki/RCRTMSTB.DAT |
| Espionage | Direct table lookup → ESPIMSTB | wiki/ESPIMSTB.DAT |
| Subdue uprising | Delta formula → SUBDMSTB | wiki/SUBDMSTB.DAT |
| Escape | `input = ((param_3 + param_2) - param_4) - param_5` → ESCAPETB | wiki/ESCAPETB.DAT |
| DS sabotage | `input = (param_1 + param_2) / 2` → DSSBMSTB | wiki/DSSBMSTB.DAT |
| Rebel evasion | RLEVADTB + Force-user modifier | wiki/RLEVADTB.DAT |

**Files**: `crates/rebellion-core/src/missions.rs`
**Action**: Cross-reference wiki formulas against our existing `MissionSystem::advance()` implementation. Port any missing input calculations.

### Task 9: CSCRHTTB Bombardment Side-Effects

Wiki documents `CSCRHTTB` + `GENERAL_PARAM_2049/2050/2051` — a bombardment classification step that maps context into result buckets 1-7 via `sub_556d50`. We only have the raw damage divisor.

**Files**: `crates/rebellion-core/src/bombardment.rs`

## Critical Files

| File | Changes |
|------|---------|
| `crates/rebellion-core/src/ai.rs` | Replace scoring function, add 12 validators, port character assignment, update tick interval |
| `crates/rebellion-core/src/tuning.rs` | Update AI_TICK_INTERVAL default once decoded, mark parity flags |
| `crates/rebellion-core/src/missions.rs` | Port exact mission probability formulas |
| `crates/rebellion-core/src/bombardment.rs` | Add CSCRHTTB side-effect classification |
| `crates/rebellion-core/src/movement.rs` | Update transit model if original differs |
| `crates/rebellion-data/src/seeds.rs` | Replace hardcoded 10/3 with GNPRTB params |
| `agent_docs/systems/ai-parity-tracker.md` | Update every BY DESIGN row to DONE/FAITHFUL |
| `ghidra/notes/*.c` | New decompiled C files for all undecoded functions |

## Execution Order

1. **Tasks 1-2** (FUN_0052e970 + FUN_00506ea0) — highest impact, resolves Pass 2 scoring
2. **Task 3** (tick interval from FUN_004927c0) — quick search in large file
3. **Task 4** (18 validators) — systematic but each sub-function is small
4. **Task 5** (character assignment handlers) — resolves production priority
5. **Task 6** (fleet transit) — check FUN_00520580
6. **Task 7** (GNPRTB starting forces) — search FUN_004927c0 init section
7. **Tasks 8-9** (wiki formulas) — code changes, no Ghidra needed

## Verification

1. Every BY DESIGN row in `ai-parity-tracker.md` updated to DONE or FAITHFUL
2. Every empty `.c` stub filled with actual decompiled code
3. `FUN_00506ea0.c` created (currently missing)
4. Run `cargo test` — all 280+ tests pass
5. Run headless campaign: `cargo run -p rebellion-playtest -- data/base --seed 42 --ticks 5000 --dual-ai --summary`
6. Run eval: `python3 scripts/eval_game_quality.py campaign.jsonl` — score ≥ 0.55

## Sources

- Ghidra decompiled functions: `ghidra/notes/*.c` (5,127 files)
- AI behavior analysis: `ghidra/notes/ai-behavior-analysis.md`
- AI parity tracker: `agent_docs/systems/ai-parity-tracker.md`
- TheArchitect2018 wiki: `https://github.com/TheArchitect2018/Deep-Dive-into-SW-Rebellion-PC-Game-Internals/wiki`
- Codex researcher comparison report (2026-03-23)
