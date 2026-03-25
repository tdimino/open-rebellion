# Combat Parity Gap Analysis — Complete Index

**Exploration Date**: 2026-03-25
**Scope**: Open Rebellion v0.18.0 | Combat System 90% → 100%
**Output Location**: `.subdaimon-output/`

---

## GENERATED REPORTS

### 1. **combat-parity-gap-analysis.md** (686 lines, 31 KB)
**Main Report** — Comprehensive gap analysis with forensic detail.

**Sections**:
- Executive summary (11 gaps across 5 categories)
- 11 gap descriptions (each with: original C++, current Rust, impact, closure path, LOC estimate)
- Ship field promotion status (25 fields promoted, data availability map)
- Special cases (Death Star, fighter launch/recall, retreat)
- Test coverage audit (13 missing test cases)
- Implementation roadmap (6 phases, 22-day critical path)
- Total effort estimate (1,330 LOC, 13–22 days depending on parallelization)
- Glossary & decompilation checklist

**How to use**: Read this for the complete narrative. Every gap has its own subsection with forensic detail on what the original does vs. our stub.

---

### 2. **combat-gaps-quickref.txt** (184 lines, 4.2 KB)
**Quick Reference** — ASCII-formatted TL;DR with priority matrix.

**Sections**:
- Status summary (90% parity, 11 gaps, effort estimates)
- Space combat gaps (1.1–1.4) with: status, impact, decompile target, LOC, test count
- Ground combat gaps (2.1–2.2)
- Bombardment gaps (3.1)
- Difficulty gaps (4.1)
- GNPRTB utilization (5.1, informational)
- Special cases with impact assessment
- 6-phase roadmap with effort per phase
- Decompilation priority queue (5 functions, ~315 lines)

**How to use**: Print this. Pin it on the wall. Use for sprint planning and weekly status updates.

---

## ANALYSIS STRUCTURE

### Gaps by Category

**1. SPACE COMBAT (7-Phase Pipeline)** — 4 gaps
- 1.1: Shield Absorption (Phase 4) — STUBBED, 25 LOC, 4 days
- 1.2: Hull Damage (Phase 5) — STUBBED, 20 LOC, 4 days
- 1.3: Weapon Fire Per-Arc (Phase 3) — PARTIAL, 100 LOC, 3 days
- 1.4: Fighter Engagement (Phase 6) — MAJOR MISSING, 150 LOC, 4 days

**2. GROUND COMBAT** — 2 gaps
- 2.1: Ground Combat Resolution — MISSING, 300 LOC, 5 days
- 2.2: Validation Checks — PARTIAL (dependent on 2.1)

**3. BOMBARDMENT** — 1 gap
- 3.1: Bombardment Blocked Flag — LOW PRIORITY, 10 LOC, 1 day

**4. DIFFICULTY MODIFIERS** — 1 gap
- 4.1: Difficulty Application — UNTESTED, 30 LOC, 2 days

**5. GNPRTB PARAMETERS** — 1 gap
- 5.1: 74 Unimplemented Parameters — INFORMATIONAL (not blockers)

**SPECIAL CASES** (Lower priority, design choices)
- Death Star combat (100 LOC if closure needed)
- Fighter launch/recall (design choice, tactical feature)
- Retreat/surrender (design choice, auto-resolve OK)

---

## KEY FINDINGS

### Data Readiness: ✓ 25 FIELDS PROMOTED (2026-03-24)

| System | Fields | Impact |
|--------|--------|--------|
| CapitalShipClass | 15 fields | Weapon fire (1.3), combat attack stats |
| FighterClass | 10 fields | Fighter engagement (1.4), attack/defense |
| **Total** | **25 fields** | Covers 6 of 11 gaps partially |

All data exists. Formulas remain to be decompiled.

### Blocker Dependencies

```
Decompile 5 C++ functions (315 lines total)
├─ FUN_00544130 (shield, 83 lines) ─ unblocks GAP 1.1 ─ CRITICAL
├─ FUN_005443f0 (hull, 54 lines) ─── unblocks GAP 1.2 ─ CRITICAL
├─ FUN_005444e0 (fighters, 53 lines) unblocks GAP 1.4 ─ CRITICAL
├─ FUN_00560d50 (ground, 232 lines) ─ unblocks GAP 2.1 ─ CRITICAL
└─ FUN_004fd600 (difficulty, ~50 lines) unblocks GAP 4.1 ─ MEDIUM

Sequential Ghidra work: ~8–10 hours (90 min per function avg)
Parallel decompilation: ~2 hours (4 agents × 90 min)
```

### Effort Summary

| Effort | Metric |
|--------|--------|
| **Total LOC** | ~1,330 lines (1 impl + 0.25 tests) |
| **Test LOC** | ~300 lines (13 new test cases) |
| **Single-threaded** | ~22 days (sequential phases A → B/C/D → E → F) |
| **2-agent parallel** | ~13 days (A → {B, C, D} → E → F) |
| **Critical path** | Shield/Hull → Fighters + Ground + Weapons → Difficulty |
| **Decompilation work** | ~8–10 hours (separate from impl) |

---

## IMPLEMENTATION ROADMAP

### Phase A: Shield & Hull Phases (Days 1–4) ✓ BLOCKS 2 gaps

**Decompilation**:
- FUN_00544130 (shield absorption, 83 lines)
- FUN_005443f0 (hull damage, 54 lines)

**Implementation**:
1. Refactor space combat pipeline to apply shields BEFORE hull
2. Implement damage reduction: `residual_damage = weapon_damage - shield_absorption`
3. Apply residual to hull: `hull_current -= residual_damage`
4. Mark alive = false when hull == 0

**Closure**: 200 LOC, 4 days, 6 tests

---

### Phase B: Weapon Fire Arc Modeling (Days 5–7) ✓ BLOCKS 1 gap + enhances weapon balance

**Decompilation**:
- FUN_00543b60 (per-side weapon fire, ~50 lines)

**Implementation**:
1. Refactor phase_weapon_fire() to iterate arcs: fore, aft, port, starboard
2. Per arc: sum turbolaser, ion, laser damage separately
3. Apply per-type attack strength scalars (from promoted fields)
4. Add RNG roll-to-hit per arc
5. Map GNPRTB parameters for weapon damage scaling

**Closure**: 150 LOC, 3 days, 4 tests

---

### Phase C: Fighter Engagement (Days 5–8, parallel with B) ✓ BLOCKS 1 gap + enables fleet tactics

**Decompilation**:
- FUN_005444e0 (fighter phase orchestrator, 53 lines)
- FUN_00543d20 (per-side fighter resolver, ~60 lines)

**Implementation**:
1. Per-squadron matchups: `attacker_losses = (defender_attack * defender_count) / (attacker_defense + baseline)`
2. Torpedo bombing as separate engagement vs capital ships
3. Squadron-specific loss calculation
4. Persist loss counts in result events

**Closure**: 250 LOC, 4 days, 6 tests

---

### Phase D: Ground Combat (Days 5–9, parallel with B/C) ✓ BLOCKS 2 gaps

**Decompilation**:
- FUN_00560d50 (main ground combat loop, 232 lines)
- FUN_004ee350 (per-unit resolution, 30 lines — already done)

**Implementation**:
1. Create ground_combat.rs module
2. Implement GroundCombatSystem::resolve() following advance() pattern
3. Iterate troops by family byte (0x14–0x1b)
4. Per unit: apply formula to regiment_strength
5. Return Vec<TroopDamageEvent> for main.rs to apply
6. Wire into simulation tick

**Closure**: 300 LOC, 5 days, 4 tests

---

### Phase E: Difficulty Modifiers (Days 10–11) ✓ BLOCKS 1 gap

**Decompilation**:
- FUN_004fd600 (difficulty modifier function, ~50 lines)

**Implementation**:
1. Extract difficulty multiplier table (0–3 difficulty levels)
2. Apply to weapon damage in Phase 3: `damage *= difficulty_multiplier[difficulty]`
3. Propagate through Phases 4–6 as implemented
4. Add parametric tests for each difficulty level

**Closure**: 80 LOC, 2 days, 4 tests (parametric)

---

### Phase F: Integration & Polish (Day 12) ✓ BLOCKS 0 gaps (improvement)

**Low-priority enhancements**:
1. Add bombardment_blocked guard (10 LOC)
2. Resolve GNPRTB parameters as decompilation yields formulas (cumulative)
3. Death Star special case if time permits (100 LOC)

**Closure**: 50+ LOC cumulative, 1+ days

---

## RISK ASSESSMENT

### Known Risks

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Decompilation yields different formula than expected | High | Have fallback approximations; extensive tests catch divergence |
| Ground combat math is highly nonlinear | Medium | Test against original game scenarios to calibrate |
| GNPRTB parameters are interdependent | Medium | Implement phase by phase; test in isolation first |
| Fighter engagement is complex (9+ inputs) | Medium | Break into matcher + bomber + losses sub-phases |

### Test Strategy

- **Unit tests**: Each formula in isolation (1–2 inputs, deterministic)
- **Integration tests**: Multi-phase scenarios (3+ inputs, RNG seeded)
- **Parity tests**: Exact scenarios from original game, match outcomes (within RNG variance)
- **Regression tests**: Existing 13 tests continue to pass after refactoring

---

## USAGE GUIDE

### For Architects
1. Read **executive summary** (combat-parity-gap-analysis.md top section)
2. Review **roadmap** (6 phases, 22-day estimate)
3. Plan sprint assignments based on **critical path** (Shield/Hull first, then parallel tracks)

### For Implementation Agents
1. Get **your gap** from quickref
2. Follow **closure path** (decompile → implement → test)
3. Check **data readiness** (promoted fields already exist)
4. Run **unit tests** to validate formula
5. Add integration tests for your phase

### For QA/Test
1. Review **test coverage audit** (13 missing test cases)
2. Follow **test strategy** (unit → integration → parity)
3. Use **original game scenarios** for parity validation

### For Decompilation Agents
1. Priority queue: FUN_00544130, FUN_005444e0, FUN_00560d50, FUN_00543b60, FUN_004fd600
2. ~90 min per function (Ghidra + annotation)
3. Deliver as `.c` files in `ghidra/notes/`
4. Include formula summary in header comments

---

## VALIDATION CHECKLIST

- [ ] Read full gap analysis (686 lines, ~20 min)
- [ ] Review quickref for executive summary (2 min)
- [ ] Identify your assigned gap(s)
- [ ] Confirm decompilation blocker is in priority queue
- [ ] Verify promoted ship fields are available (✓ 25 fields, 2026-03-24)
- [ ] Plan sprint with PE estimate (days/LOC from roadmap)
- [ ] Set up Ghidra for decompilation (if needed)
- [ ] Create branch for implementation phase
- [ ] Run existing tests to establish baseline (9 combat + 4 bombardment)
- [ ] Mark gap as "in progress" in team tracker

---

## REFERENCES

**Inside this .subdaimon-output/ directory**:
- `combat-parity-gap-analysis.md` — Full forensic detail (686 lines)
- `combat-gaps-quickref.txt` — ASCII TL;DR (184 lines)
- `COMBAT-ANALYSIS-INDEX.md` ← **You are here**

**In Open Rebellion codebase**:
- `crates/rebellion-core/src/combat.rs` — Space combat implementation (200 LOC, 9 tests)
- `crates/rebellion-core/src/bombardment.rs` — Bombardment implementation (180 LOC, 4 tests)
- `ghidra-site/docs/combat/` — 5 RE documents (COMBAT-SUMMARY.md, space-combat.md, etc.)
- `agent_docs/ghidra-re.md` — RE overview (111 GNPRTB parameters mapped)
- `CLAUDE.md` — Project status: Combat 90%, overall 93%

---

**Analysis completed**: 2026-03-25 | Claude Code (Opus 4.6)
**Status**: Ready for implementation sprints
**Next step**: Assign phases A–D to implementation agents; prioritize Ghidra decompilation
