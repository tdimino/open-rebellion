# Open Rebellion — Core Parity Audit Index

**Date**: 2026-03-25
**Scope**: Complete simulation parity gap analysis for Star Wars Rebellion (1998)
**Status**: Audit complete, ready for implementation planning

---

## Document Guide

### 1. **core-parity-gaps-complete-audit-2026-03-25.md** (31 KB)
Comprehensive master document covering all 24 gaps.

**Contains**:
- Executive summary (24 gaps, 97%→100% parity, ~1,200–1,400 LOC)
- Gap catalog organized by system (Combat, AI, Seeding, Character, etc.)
- For each gap: location, what original does, what we do, what's missing, impact, files to change, estimated work
- Detailed implementation notes for high-impact gaps
- Cross-references to Ghidra RE, previous audits, AI parity tracker
- Implementation roadmap (Phase 1/2/3 with effort estimates)

**Use When**:
- Planning implementation details
- Understanding full context of a specific gap
- Cross-referencing between related gaps
- Reviewing for completeness

**Key Sections**:
- Gap Summary by System (table, ~10 rows)
- Detailed Gap Catalog (9 sections, one per system)
- Priority Action Plan (Phase 1/2/3 breakdown)
- Effort Estimate (detailed table)

---

### 2. **gap-quick-reference.md** (7 KB)
Concise, sortable index of all gaps.

**Contains**:
- Gap index sorted by priority (HIGH → MEDIUM → LOW)
- By-priority grouping with effort/LOC/blocker status
- By-system summary (status, gap count, LOC, files affected)
- Implementation roadmap summary
- File change summary (which files touch, LOC per file)
- Test vectors for each gap
- Notes and cross-references

**Use When**:
- Quick lookup of specific gap status
- Sorting by priority/effort/system
- Scanning implementation roadmap
- Status reports and planning

**Key Sections**:
- Gap Index (24 gaps, sortable table)
- By Priority (HIGH/MEDIUM/LOW with counts)
- By System (status breakdown)
- Implementation Roadmap (Phase 1/2/3 summary)

---

### 3. **implementation-checklist.md** (5.2 KB)
Task-by-task action plan for Phase 1 and beyond.

**Contains**:
- Phase 1 (3 critical gaps) — task breakdown with function signatures
- Phase 2 (10 medium gaps) — overview + links to full docs
- Phase 3 (11 low gaps) — overview
- Testing strategy (unit, integration, regression)
- Dependencies map (gap relationships)
- Completion checklist (24-item sign-off)

**Use When**:
- Implementing a specific gap (find your task, read pseudocode + test vector)
- Understanding dependencies between gaps
- Planning testing approach
- Tracking implementation progress

**Key Sections**:
- Task 1.1–1.3 (Phase 1 details with code signatures)
- Task 2.1–2.5 (Phase 2 overview)
- Task 3.1–3.6 (Phase 3 overview)
- Dependencies Map (visual dependency graph)

---

## Gap Summary Table

| Priority | Count | LOC | Time | Notes |
|----------|-------|-----|------|-------|
| **HIGH** (Blocks campaigns) | 3 | 580 | 3–4 h | Implement first; unblocks balanced AI |
| **MEDIUM** (Polish) | 10 | 620 | 4–5 h | Reaches 99% parity |
| **LOW** (Complete) | 11 | 630 | 3–4 h | Reaches 100% parity |
| **TOTAL** | **24** | **1,200–1,400** | **10–13 h** | All gaps to full parity |

---

## Critical Gaps (Phase 1)

1. **Gap 3.1: Procedural System Control Distribution** (400 LOC)
   - **Impact**: Galaxy ownership distribution fundamentally broken
   - **Current**: All systems seeded with identical control (3 A, 3 E per sample)
   - **Original**: GNPRTB-driven bucketing (Empire 60%, Alliance 30%, Neutral 10%)
   - **Blocker**: YES — asymmetric galaxy necessary for balanced campaigns
   - **File**: `seeds.rs` (rewrite apply_seeds + new functions)

2. **Gap 2.1: Troop Deployment AI** (100 LOC)
   - **Impact**: Ground forces never deployed; uprisings unchecked
   - **Current**: Comment says "troops needed" but no implementation
   - **Original**: FUN_00508250 validator #8; deploys garrisons to low-support systems
   - **Blocker**: YES — AI losses territory rapidly without garrison defense
   - **File**: `ai.rs` (add evaluate_garrison_troops function)

3. **Gap 3.2: Support/Popularity Initialization** (80 LOC)
   - **Impact**: Initial system control strength meaningless
   - **Current**: All systems start at 0.5 / 0.5 (perfectly neutral)
   - **Original**: GNPRTB-driven support ranges (strong controlled 60+%, weak 20+%, neutral 41–59%)
   - **Blocker**: YES — affects uprising/betrayal thresholds, mission targets
   - **File**: `seeds.rs` (add initialize_support_values function)

---

## How to Use This Audit

### For Planning
1. Read **gap-quick-reference.md** to understand scope
2. Review **core-parity-gaps-complete-audit-2026-03-25.md** section "Priority Action Plan"
3. Use **implementation-checklist.md** to estimate work/timeline

### For Implementation
1. Pick a gap from **implementation-checklist.md** Task list
2. Cross-reference details in **core-parity-gaps-complete-audit-2026-03-25.md**
3. Follow pseudocode + test vector in **implementation-checklist.md**
4. Implement, test, mark complete

### For Communication
- **With stakeholders**: Use **gap-quick-reference.md** for status/priority/effort
- **With developers**: Use **implementation-checklist.md** for technical details
- **In documentation**: Reference **core-parity-gaps-complete-audit-2026-03-25.md** for full context

### For Validation
- Use **implementation-checklist.md** completion checklist to track progress
- Run test vectors for each gap as implemented
- Verify eval_game_quality.py score improvement per phase

---

## Cross-References

**Related Documents in Repo**:
- `agent_docs/systems/ai-parity-tracker.md` — AI function parity matrix (resolved by this audit)
- `agent_docs/roadmap.md` — Previous roadmap (updated by this audit)
- `agent_docs/game-domain.md` — Game mechanics reference (useful context)
- `seeding-parity-audit.md` — Prior seeding analysis (superseded by Gap 3.x section)

**External References**:
- TheArchitect2018 Wiki: [Deep-Dive-into-SW-Rebellion-PC-Game-Internals](https://github.com/TheArchitect2018/Deep-Dive-into-SW-Rebellion-PC-Game-Internals)
- Ghidra RE decompiles: `ghidra/notes/` directory

---

## FAQ

**Q: What's the priority?**
A: Phase 1 (critical) blocks balanced campaigns. Phase 2 polishes to 99%. Phase 3 completes to 100%.

**Q: Can we ship v0.18 without fixing these?**
A: Yes. The game is playable at 97%. Phase 1 gaps affect campaign balance and longevity, not blocking conditions.

**Q: How long will it take?**
A: Phase 1 = 3–4 hours (3 developers = 1 hour parallel). Phases 2–3 = 7–9 hours total.

**Q: Which gaps should we do first?**
A: Phase 1 critical gaps (3.1, 2.1, 3.2) unblock everything else. Then Phase 2 in order of dependencies.

**Q: Are these estimates reliable?**
A: ±20%. Actual LOC may vary; Ghidra decompiles may reveal simpler/harder implementations. Test vectors provide safety net.

**Q: Will fixing these break anything?**
A: Low risk. All gaps are additions (new code paths) or parameter adjustments, not refactors. Existing tests should still pass.

---

## Metrics

**Parity Progress**:
- v0.18.0: 97% (current)
- v0.19.0: ~98% (after Phase 1)
- v0.20.0: ~99% (after Phase 2)
- v0.21.0: 100% (after Phase 3)

**Code Coverage**:
- 15 simulation systems total
- 6 systems fully complete (Movement, Manufacturing, Fog, Blockade, Uprising, etc.)
- 4 systems near-complete (Combat, AI, Missions, Events) — gaps are edge cases
- 2 systems partially complete (Character, Seeding) — most implemented, some procedural missing
- 3 systems complete (Victory, Research, Jedi)

**Gap Distribution**:
- Simulation core (systems): 24 gaps
- Rendering/UI: 0 gaps (not in scope)
- Audio/Video: 0 gaps (not in scope)
- Save/Load: 0 gaps (verified working)

---

**Prepared by**: Core Exploration Agent
**Last Updated**: 2026-03-25 14:09 UTC

---

**Quick Navigation**:
- [Master Audit Doc](core-parity-gaps-complete-audit-2026-03-25.md)
- [Quick Reference](gap-quick-reference.md)
- [Implementation Checklist](implementation-checklist.md)
