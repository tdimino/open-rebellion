# Open Rebellion Parity Gaps — Implementation Checklist

**Last Updated**: 2026-03-25
**Status**: Ready for implementation (task planning phase)

---

## PHASE 1: CRITICAL GAPS (Block Balanced Campaigns)

### Task 1.1: Gap 3.1 — Procedural System Control Distribution

**Files to Change**:
- `crates/rebellion-data/src/seeds.rs` (main)
- `crates/rebellion-core/src/world/mod.rs` (add fields if needed)
- `crates/rebellion-core/src/ids.rs` (verify DatId encoding for family bytes)

**Function Changes**:
```rust
// seeds.rs — REWRITE apply_seeds() signature and internals
pub fn apply_seeds(
    world: &mut GameWorld,
    seed: u64,
    rng_seed: u64,  // NEW: needed for bucketing shuffle
) -> Result<(), String>

// NEW FUNCTIONS to add:
fn compute_control_buckets(
    world: &GameWorld,
    rng: &mut RngState,  // DeterministicRng from crate root
) -> ControlBuckets {
    // Returns: alliance_strong, alliance_weak, empire_strong, empire_weak, neutral, unoccupied counts
}

fn assign_control_distribution(
    world: &mut GameWorld,
    buckets: &ControlBuckets,
    systems: &[SystemKey],  // shuffled
)

fn shuffle_systems(systems: &mut [SystemKey], rng: &mut RngState)  // Fisher-Yates
```

**Pseudocode**:
```
1. Load GNPRTB params: 7680 (alliance%), 7681 (empire%)
2. Count core systems (exploration_status == Explored)
3. Compute bucket percentages
4. Shuffle all systems (Fisher-Yates with RNG seed)
5. Drain buckets in shuffled order:
   - Assign alliance_strong systems to ControlKind::Controlled(Alliance)
   - Assign empire_strong systems to ControlKind::Controlled(Empire)
   - etc.
6. Repeat for rim systems with params 7732/7733
7. Handle special systems (Coruscant=always Empire, Yavin=always Alliance)
```

**Test Case**:
```rust
#[test]
fn test_control_distribution_seeding() {
    let mut world = make_test_world();
    apply_seeds(&mut world, 0xDEADBEEF, 12345).unwrap();

    let controlled = world.systems.values()
        .filter(|s| s.controlling_faction == ControlKind::Controlled(Faction::Alliance))
        .count();

    // Should be ~30% of core systems (param 7680 default)
    assert!(controlled > 10 && controlled < 50);
}
```

**Effort**: 300–400 LOC | **Time**: 2–3 hours

---

## PHASE 2: MEDIUM-IMPACT GAPS (Polish to 99%)

### Task 2.1: Gap 2.2 — Dispatch Validation Checks (14/18)

**Files to Change**:
- `crates/rebellion-core/src/ai.rs` (expand can_dispatch)

**Current Implementation** (4 checks):
- Check 1: Jedi reserved
- Check 2: Mandatory mission
- Check 3: Faction match
- Check 4: Cooldown per character

**Expansion** (add checks from Ghidra decode):
- Check 2: FUN_0050ad60 — capacity overflow (+0x5c < +0x64)
- Check 3: FUN_0050ad80 — fleet entity count validation
- Check 5: FUN_0050b230 — faction + status bits (+0x88)
- Check 6: FUN_0050b2c0 — faction + loyalty scoring
- Check 9: FUN_0050b5a0 — faction + status bits (alternate path)
- Check 13: FUN_0050bc60 — character iteration + faction match
- Check 14: FUN_0050be00 — character iteration + mandatory mission

**Effort**: 150–180 LOC | **Time**: 2 hours

---

## PRIORITY ORDER

**Recommended Implementation Order** (dependencies):
1. Gap 3.1 (control distribution) — unblocks Gap 3.2
2. Gap 3.2 (support init) — depends on 3.1
3. Gap 2.1 (troop AI) — independent
4. Gap 2.2 (validators) — independent
5. All remaining gaps — independent or lowpriority

---

## TESTING STRATEGY

### Unit Tests (Per Gap)
- `test_control_distribution_seeding()`
- `test_garrison_troops_low_support()`
- `test_support_initialization()`
- `test_damage_variance_formula()`

### Integration Tests
- Run headless campaign (playtest binary) with each config
- Verify eval_game_quality.py score improves with each phase

### Regression Tests
```bash
# After Phase 1
cargo test -p rebellion-core
cargo run -p rebellion-playtest -- data/base --seed 42 --ticks 5000 --summary

# After Phase 2
# ... same checks, verify score >0.65
```

---

## DEPENDENCIES MAP

```
Gap 3.1: Procedural Control Distribution
├─ Gap 3.2: Support/Popularity Init (depends on control assignment)
├─ Gap 3.3: Energy/Raw Materials (independent)
├─ Gap 3.4: Procedural Facility Seeding (depends on 3.3)
└─ Gap 3.5: Garrison Troop Seeding (independent)

Gap 2.1: Troop Deployment AI (independent)
├─ Gap 2.2: Dispatch Validators (independent)
├─ Gap 2.3: Faction-Specific Budgets (independent)
└─ Gap 2.4–2.14: AI Doctrines (independent)

Gap 1.1: Damage Fire Variance (independent)
├─ Gap 1.2: Death Star Superlaser (independent)
└─ Gap 3.6: Character Stat Rolling (independent)

Gap 3.7: Named Character Placement (independent)
├─ Gap 3.8: Difficulty-Modified Seeding (independent)
└─ Gap 4.2–4.3, 5.1–5.2, 6.1–6.2: Low-impact (all independent)
```

---

## COMPLETION CHECKLIST

- [ ] Phase 1: Critical gaps (3 gaps, ~580 LOC)
- [ ] Phase 2: Medium gaps (10 gaps, ~620 LOC)
- [ ] Phase 3: Low gaps (11 gaps, ~630 LOC)
- [ ] All 24 gaps closed, 1,200–1,400 LOC total
- [ ] Campaign tests pass (victory achievable)
- [ ] eval_game_quality.py baseline increases
- [ ] Zero warnings, zero panics
- [ ] Documentation updated
- [ ] Release notes drafted

---

**Full Details**: See `core-parity-gaps-complete-audit-2026-03-25.md`
**Quick Reference**: See `gap-quick-reference.md`
