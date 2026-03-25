# Open Rebellion Parity Gaps — Quick Reference

**Date**: 2026-03-25
**Total Gaps**: 24
**Parity**: 97% → 100%
**Est. LOC**: 1,200–1,400

---

## Gap Index (sorted by priority)

| ID | System | Gap | Priority | Impact | LOC | Blocker |
|----|---------|----|----------|--------|-----|---------|
| **3.1** | **Seeding** | **Procedural System Control Distribution** | **HIGH** | **Ownership/balance** | **400** | **YES** |
| **2.1** | **AI** | **Troop Deployment AI** | **HIGH** | **Ground defense** | **100** | **YES** |
| **3.2** | **Seeding** | **Support/Popularity Init** | **HIGH** | **Mission targets** | **80** | **YES** |
| 2.2 | AI | Dispatch Validators (14/18) | MEDIUM | AI order validation | 150 | No |
| 3.3 | Seeding | Energy/Raw Materials | MEDIUM | Facility limits | 60 | No |
| 3.4 | Seeding | Procedural Facility Seeding | MEDIUM | Facility placement | 100 | No |
| 1.1 | Combat | Damage Fire Variance | MEDIUM | Combat RNG | 40 | No |
| 2.3 | AI | Faction-Specific Budgets | MEDIUM | AI doctrine | 50 | No |
| 1.2 | Combat | Death Star Superlaser | MEDIUM | Victory condition | 60 | No |
| 3.5 | Seeding | Garrison Troop Seeding | MEDIUM | Uprising prevention | 40 | No |
| 3.6 | Seeding | Character Stat Rolling | MEDIUM | Officer variety | 50 | No |
| 3.7 | Seeding | Named Character Placement | MEDIUM | Story flavor | 40 | No |
| 2.4–2.7 | AI | Research Doctrine (4 gaps) | MEDIUM | Tech progression | 100 | No |
| 2.8–2.12 | AI | Character Assignment (5 gaps) | MEDIUM | Officer placement | 100 | No |
| 2.13–2.14 | AI | Production Doctrine (2 gaps) | LOW | Facility priority | 50 | No |
| 3.8 | Seeding | Difficulty-Modified Seeding | LOW | Balance tuning | 20 | No |
| 4.2 | Character | Rank Field | LOW | Character progression | 15 | No |
| 4.3 | Character | Empire Leadership Bonus | LOW | Empire stats | 15 | No |
| 5.1 | Missions | Probability Formula Refinement | LOW | Success rates | 30 | No |
| 5.2 | Missions | FDECOYTB Decoy Table | LOW | Decoy flavor | 20 | No |
| 6.1 | Events | Story Event Expansion | LOW | Campaign narrative | 250 | No |
| 6.2 | Events | Event Chaining Depth Limit | LOW | Cascade safety | 10 | No |

---

## By Priority

### HIGH (Blocks Balanced Campaigns) — 3 gaps, ~580 LOC

1. **Gap 3.1**: Procedural System Control Distribution
   - File: `seeds.rs`
   - Impact: All systems currently seeded identically; need GNPRTB-driven bucketing
   - Action: Implement core/rim bucket algorithm, Fisher-Yates shuffle, procedural assignment

2. **Gap 2.1**: Troop Deployment AI
   - File: `ai.rs`
   - Impact: AI never deploys ground forces; low-support systems fall to uprisings
   - Action: Add garrison threshold logic, enqueue troops for undefended systems

3. **Gap 3.2**: Support/Popularity Initialization
   - File: `seeds.rs`
   - Impact: All systems start neutral (0.5/0.5); original has varied initial support
   - Action: Apply support ranges per control bucket using GNPRTB params

### MEDIUM (Completeness → 99%) — 10 gaps, ~620 LOC

- **Gap 2.2**: Dispatch Validators (14/18 missing) — `ai.rs:314–370` → 150 LOC
- **Gap 3.3**: Energy/Raw Materials → 60 LOC
- **Gap 3.4**: Procedural Facility Seeding → 100 LOC
- **Gap 1.1**: Damage Fire Variance Formula → 40 LOC
- **Gap 2.3**: Faction-Specific AI Budgets → 50 LOC
- **Gap 1.2**: Death Star Superlaser → 60 LOC
- **Gap 3.5**: Garrison Troop Seeding → 40 LOC
- **Gap 3.6**: Character Stat Rolling → 50 LOC
- **Gap 3.7**: Named Character Placement → 40 LOC
- **Gap 2.4–2.14** (11 gaps total, rolled up): AI doctrines → 100 LOC

### LOW (Polish → 100%) — 11 gaps, ~630 LOC

- **Gap 3.8**: Difficulty-Modified Seeding → 20 LOC
- **Gap 4.2**: Rank Field → 15 LOC
- **Gap 4.3**: Empire Leadership Bonus → 15 LOC
- **Gap 5.1**: Mission Probability Refinement → 30 LOC
- **Gap 5.2**: FDECOYTB Decoy Table → 20 LOC
- **Gap 6.1**: Story Event Expansion (15 new chains) → 250 LOC
- **Gap 6.2**: Event Chaining Depth Limit → 10 LOC
- Plus minor fixes → 270 LOC

---

## By System

| System | Status | Gaps | LOC | Files |
|--------|--------|------|-----|-------|
| **Seeding** | PARTIAL | 3 high, 5 med | 400+180 | `seeds.rs`, `lib.rs` |
| **AI** | PARTIAL | 0 high, 10 med+low | 450+100 | `ai.rs`, `tuning.rs` |
| **Combat** | NEAR-COMPLETE | 0 high, 2 med | 100 | `combat.rs` |
| **Character** | NEAR-COMPLETE | 0 high, 3 low | 80 | `world/mod.rs` |
| **Missions** | COMPLETE | 0 high, 2 low | 50 | `missions.rs` |
| **Events** | COMPLETE | 0 high, 2 low | 260 | `story_events.rs`, `events.rs` |
| **Movement** | ✅ DONE | — | — | — |
| **Manufacturing** | ✅ DONE | — | — | — |
| **Other (11)** | ✅ DONE | — | — | — |

---

## Implementation Roadmap

### Phase 1 (v0.19.0): High-Impact — 3–4 hours
**Unlocks**: Balanced AI campaigns, proper control distribution, uprising mechanics
- [ ] Gap 3.1: Procedural control distribution
- [ ] Gap 2.1: Troop deployment AI
- [ ] Gap 3.2: Support initialization

### Phase 2 (v0.20.0): Medium-Impact — 4–5 hours
**Achieves**: 99% parity, AI order validation, procedural galaxy features
- [ ] Gap 2.2: Dispatch validators
- [ ] Gap 3.3: Energy/raw materials
- [ ] Gap 3.4: Procedural facility seeding
- [ ] Gap 1.1: Damage formula
- [ ] Gap 2.3: Faction budgets

### Phase 3 (v0.21.0): Low-Impact — 3–4 hours
**Achieves**: 100% parity, full story expansion, character variety
- [ ] Remaining medium gaps (death star, garrison, stat rolling, placement)
- [ ] All low-impact gaps (14 gaps, story events, decorative fields)

---

## File Change Summary

| File | Changes | LOC | Effort |
|------|---------|-----|--------|
| `seeds.rs` | Major rewrite + procedural loops | 500+ | High |
| `ai.rs` | Add validators, garrison logic, doctrines | 450+ | High |
| `combat.rs` | Damage formula, superlaser mechanics | 100 | Medium |
| `tuning.rs` | Faction-specific config | 50 | Low |
| `world/mod.rs` | Rank field, stat rolling | 80 | Low |
| `story_events.rs` | 15 new event chains | 250 | Medium |
| `missions.rs` | Probability refinement, decoy table | 50 | Low |
| `events.rs` | Event depth limit | 10 | Trivial |

---

## Test Vectors

For each gap implemented:
1. **Seeding tests**: Verify control distribution matches GNPRTB params
2. **AI tests**: Verify dispatch validators reject/accept appropriately
3. **Campaign tests**: Run 10×seed autoresearch, verify eval_game_quality.py score improves
4. **Regression tests**: Ensure no breakage in existing passing scenarios

---

## Notes

- **Seeding is interconnected**: Gaps 3.1–3.2–3.3–3.4–3.5–3.6–3.7 all depend on RNG/param threads. Best implemented in sequence.
- **AI validators complex**: 14 validators reference internal C++ memory offsets (+0x5c, +0x88 bits). Requires careful porting.
- **Combat formulas**: Two TODO items have fallbacks; low priority unless combat RNG reproducibility required.
- **Story events**: 250 LOC is estimate; actual implementation depends on Ghidra decompile quality. Can be implemented in parallel with core gaps.

---

**Full Details**: See `core-parity-gaps-complete-audit-2026-03-25.md`
