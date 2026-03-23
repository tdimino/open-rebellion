# AI Functions — Complete Decompilation

All AI functions from REBEXE.EXE, decompiled via GhidraMCP HTTP bridge on 2026-03-23. Cross-referenced against [TheArchitect2018's wiki](https://github.com/TheArchitect2018/Deep-Dive-into-SW-Rebellion-PC-Game-Internals/wiki).

## Architecture

The original AI is **event-driven**, not tick-based. Message `0x1f0` (day tick notification) triggers evaluation every game-day through the CNotifyObject observer pattern. There is no tick interval — the AI runs once per day.

```
Master Turn Processing (FUN_004927c0, 2098 lines)
  ← triggered by event 0x1f0 (day tick)
  │
  ├── Galaxy Evaluation (FUN_00519d00, 252 lines)
  │   └── 7-bucket system categorization + ratio scoring (FUN_0053e190)
  │
  ├── Primary Deployment (FUN_00537180, 381 lines)
  │   ├── Per-system fleet movement decisions
  │   └── Dispatch Validation (FUN_00508250, 18 AND-checks)
  │       └── Entity Dispatch (FUN_00508660, 10+ type handlers)
  │
  ├── Secondary Deployment (FUN_005385f0, 252 lines)
  │   ├── Faction-Specific Evaluator (FUN_00506ea0)
  │   └── Capacity Check (FUN_0052e970)
  │
  └── Movement Orders (FUN_00520580, command setter)
```

## Critical Finding: FUN_0052e970 Is NOT a Scoring Function

Previously thought to be a weighted target scoring function. **Actual behavior**: binary capacity check.

```c
bool FUN_0052e970(void *this, uint *param_1, int param_2, void *param_3)
{
  // Check entity family byte is in range 0x10-0x3f (ships/troops/facilities)
  bool bVar1 = false;
  if ((0xf < *param_1 >> 0x18) && (*param_1 >> 0x18 < 0x40)) {
    bVar1 = true;
  }

  if (bVar1) {
    int entity = FUN_0051cab0(param_1);  // Resolve entity pointer
    if (entity != 0) {
      int capacity = FUN_0053b870(entity);  // Read *(entity + 0x4c) = capacity
      capacity = capacity * param_2;        // Scale by multiplier

      // Compare against deployment budget: this+0x58 (allocated) vs this+0x5c (max)
      int remaining = *(this + 0x58) - *(this + 0x5c);
      if (capacity != 0 && remaining != capacity && remaining <= capacity) {
        // Entity fits in budget → dispatch command 0xf3
        FUN_00520580(param_3, {0xf3, 1});
      }
    }
  }
  return bVar4;
}

// FUN_0053b870: entity capacity reader (7 lines)
// Returns *(entity + 0x4c)
```

**Open Rebellion's 4-factor weighted model (weakness × proximity × deconfliction × freshness) is strictly superior.** The original doesn't score targets at all — it just checks if an entity fits the deployment budget.

## Faction-Specific Evaluator — FUN_00506ea0

Returns a faction-specific evaluator object with different deployment budgets:

```c
undefined4 FUN_00506ea0(int param_1)
{
  if (((0 < param_1) && (param_1 < 3)) && (DAT_006b2bb0 != 0)) {
    if (param_1 == 1) {
      return *(DAT_006b2bb0 + 0xc4);  // Alliance evaluator
    }
    return *(DAT_006b2bb0 + 200);      // Empire evaluator (0xc8)
  }
  return 0;
}
```

**Meaning**: Alliance and Empire have different deployment budget thresholds. The evaluator objects at `+0xc4` and `+0xc8` have different values at their `+0x58` and `+0x5c` offsets (the budget fields read by FUN_0052e970).

## Movement Orders — FUN_00520580

A 2-field struct setter, **not** a transit time calculator:

```c
void FUN_00520580(void *this, undefined4 *param_1)
{
  *(this) = *param_1;        // command_type
  *(this + 4) = param_1[1];  // parameter
  return;
}
```

## 18 Dispatch Validators — FUN_00508250

All entity deployments pass through 18 AND-chained boolean checks. If any fails, the deployment is rejected.

```c
// Simplified structure — actual code chains 18 if/else blocks
bool FUN_00508250(void *this, void *param_1)
{
  return FUN_0051ebb0()        // 1.  Always true (no-op)
      && FUN_0050ad60(this, p) // 2.  Capacity overflow: +0x5c < +0x64
      && FUN_0050ad80(this, p) // 3.  Fleet entity count vs capacity
      && FUN_0050b0b0(this, p) // 4.  Entity count via vtable+0x1c8 vs budget
      && FUN_0050b230(this, p) // 5.  Faction + status bits + scoring
      && FUN_0050b2c0(this, p) // 6.  Faction + loyalty scoring
      && FUN_0050b310(this, p) // 7.  Ship type compatibility
      && FUN_0050b610(this, p) // 8.  Troop deployment checks
      && FUN_0050b5a0(this, p) // 9.  Faction + status bits (+0x88)
      && FUN_0050b500(this, p) // 10. Fleet composition (troops - allocation)
      && FUN_0050ba90(this, p) // 11. Troop availability
      && FUN_0050bb70(this, p) // 12. Ship capacity
      && FUN_0050bc60(this, p) // 13. Character iteration + faction match
      && FUN_0050be00(this, p) // 14. Mandatory mission check
      && FUN_0050c350(this, p) // 15. Fleet/facility nested iteration
      && FUN_0050b8e0(this, p) // 16. System strength scoring (both factions)
      && FUN_0051ebb0()        // 17. Always true (no-op, same as #1)
      && FUN_0050b800(this, p) // 18. Status bits + position check
      && FUN_0050bb00(this, p);// 19. Final deployment flag
}
```

### Validator Categories

| Category | Validators | Purpose |
|----------|-----------|---------|
| **No-ops** | #1, #17 (`FUN_0051ebb0`) | Always return 1 — stub gates |
| **Capacity** | #2, #3, #4, #10, #12 | Entity counts vs budget thresholds at +0x5c, +0x64, +0x80 |
| **Faction/Status** | #5, #6, #9, #13, #18, #19 | Faction ownership (+0x24>>6&3), status bits at +0x88 |
| **Type Compatibility** | #7, #8, #11 | Ship type, troop class, availability checks |
| **Complex Iteration** | #14, #15, #16 | Character missions, fleet/facility nesting, system strength |

### Key Offset Map

| Offset | Type | Field | Used By |
|--------|------|-------|---------|
| +0x24 bits 6-7 | uint | Faction (0=neutral, 1=Alliance, 2=Empire) | #5, #6, #9, #13, #18, #19 |
| +0x4c | int | Entity capacity/count | FUN_0052e970 via FUN_0053b870 |
| +0x58 | int | Allocated deployment budget | FUN_0052e970 |
| +0x5c | int | Max deployment budget | #2, #3, FUN_0052e970 |
| +0x64 | int | Secondary budget limit | #2, #4 |
| +0x7c | int | Position/location | #18, #19 |
| +0x80 | int | Troop allocation | #10 |
| +0x88 | uint | Status bitfield | #5, #7, #8, #9, #18, #19 |
| +0x88 bit 0 | bit | Active/exists | #8, #18 |
| +0x88 bit 2 | bit | In-transit flag | #18, #19 |
| +0x88 bit 5 | bit | Ship combat type | #7 |
| +0x88 bit 11 | bit | Special status | #5, #9 |

## Entity Placement Handlers — FUN_00508660

Routes entities to type-specific placement by family byte:

| Family Range | Handler | Entity Type | Lines |
|-------------|---------|-------------|-------|
| 0x08-0x0f | `FUN_0050ac80` | Characters | 49 |
| (multiple) | `FUN_0050a1b0` | Faction-conditioned | 28 |
| 0x10-0x13 | `FUN_0050b610` | Troops | 77 |
| 0x1c-0x1f | `FUN_0050c6c0` | Facilities | 69 |
| 0x20-0x21 | `FUN_0050c9f0` | Ships (type A) | 32 |
| 0x22 | `FUN_0050b310` | Ships (type B) | 73 |
| 0x28 | `FUN_0050bb70` + `FUN_0050c180` | Ships (type C) | 15+ |
| 0x29 | `FUN_0050c1b0` | Ships (type D) | — |
| 0x2a | `FUN_0050c1e0` | Ships (type E) | — |
| 0x2c-0x2f | `FUN_0050c560` | Ships (type F) | — |

## Mission Probability Formulas

From TheArchitect2018 wiki, ported to Open Rebellion `missions.rs::compute_table_input()`:

| Mission | Original Function | Input Formula | Table |
|---------|------------------|---------------|-------|
| Diplomacy | `sub_55ae50` | `(enemy_pop - our_pop) + diplomacy_rating` | DIPLMSTB |
| Recruitment | `sub_55aed0` | `leadership - resistance` | RCRTMSTB |
| Espionage | `sub_55ae90` | Direct skill lookup | ESPIMSTB |
| Subdue | `sub_55af50` | `(enemy_pop - our_pop) + diplomacy` | SUBDMSTB |
| DS Sabotage | `sub_55b0a0` | `(espionage + combat) / 2` | DSSBMSTB |
| Escape | `sub_55cfb0` | `((p3 + p2) - p4) - p5` + RNG | ESCAPETB |

## Parity Analysis

### Where Open Rebellion Matches the Original

| Feature | Original | Open Rebellion | Status |
|---------|----------|---------------|--------|
| Galaxy evaluation | 7-bucket categorization | `evaluate_galaxy_state()` | DONE |
| Garrison strength | Ships + troops + facilities | `system_strength()` | DONE |
| Entity dispatch | 10+ family handlers | Split into 3 functions | DONE |
| Two-pass deployment | Primary + secondary | Two-pass with aggression | DONE |
| Mission formulas | Composite table inputs | `compute_table_input()` | DONE |

### Where Open Rebellion Improves on the Original

| Feature | Original | Open Rebellion | Why Better |
|---------|----------|---------------|-----------|
| Target scoring | Binary capacity check | 4-factor weighted model | Strategic depth |
| AI evaluation | Every game-day | Every 7 ticks (configurable) | Performance |
| Deployment doctrine | Faction pointer lookup | Aggression-scaled redistribution | More dynamic |
| Character assignment | Family-byte routing | Skill-based dispatch | More strategic |

### Remaining Gaps

| Gap | Impact | Effort |
|-----|--------|--------|
| 14 dispatch validators not ported to Rust | Invalid deployments possible | Medium — entity offsets need Rust type modeling |
| Faction-specific deployment budgets | Both factions use same thresholds | Low — add to AiConfig |
