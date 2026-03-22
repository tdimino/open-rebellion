---
title: "AI Behavior Analysis—Original REBEXE.EXE"
description: "Complete reverse engineering of the original Star Wars Rebellion AI: fleet deployment, character assignment, production, threat evaluation, and galaxy-wide strategic planning"
category: "ghidra"
created: 2026-03-20
updated: 2026-03-20
sources:
  - type: "ghidra"
    files:
      - "FUN_00502020.c"
      - "FUN_00508660.c"
      - "FUN_00519d00.c"
      - "FUN_00537180.c"
      - "FUN_005385f0.c"
      - "FUN_00508250.c"
      - "FUN_004927c0.c"
      - "FUN_0050d5a0.c"
  - type: "scholar"
    files:
      - "modders-taxonomy.md"
      - "annotated-functions.md"
      - "entity-system.md"
      - "economy-systems.md"
      - "cpp-class-hierarchy.md"
---

# AI Behavior Analysis—Original REBEXE.EXE

## Architecture: No Single AI Function

The original Rebellion AI is **event-driven and distributed** across a 6-function pipeline, not a monolithic decision function. It uses the CRebObject → CNotifyObject → CCombatUnit hierarchy with a setter-notify-event pattern. The AI never runs as a single top-level call—it's triggered through the notification/observer system and evaluated per-system.

## The AI Pipeline (6 Functions)

```
1. Galaxy Evaluation (FUN_00519d00)
   ↓ categorizes all 200 systems into 7 faction-specific buckets
2. System Iteration (FUN_00537180 / FUN_005385f0)
   ↓ for each system: check faction, evaluate entities, trigger deployment
3. Strength Assessment (FUN_00502020)
   ↓ per-system: count ships (0x30-0x3f), troops (0x10-0x13), facilities (0x1c-0x1f)
4. System Validation (FUN_00508250)
   ↓ pre-check: can this operation proceed? (~18 validators)
5. Entity Dispatch (FUN_00508660)
   ↓ route entities to placement handlers by family type (10+ handlers)
6. Movement Orders (FUN_00520580)
   ↓ set fleet deployment/movement parameters
```

## Function Details

### 1. Galaxy-Wide System Distribution—`FUN_00519d00` (252 lines)

**Address**: 0x00519d00 | **Purpose**: Strategic evaluation of all 200 systems

- Iterates ALL systems via `FUN_0051cad0(0x90)` (family byte 0x90 = explored systems)
- Checks specific DatIds: `0x92000121` (unexplored), `0x90000109` (Coruscant)
- Calls `FUN_005597e0(faction, type)` with 4 combinations:
  - `(1, 0)`—Alliance systems, type 0
  - `(2, 0)`—Empire systems, type 0
  - `(1, 1)`—Alliance systems, type 1
  - `(2, 1)`—Empire systems, type 1
- Distributes systems into **7 strategic buckets** at offsets:
  - `+0x100`, `+0x110`—primary faction buckets
  - `+0xe0`, `+0xf0`—secondary buckets
  - `+0x120`, `+0x130`, `+0x140`—tertiary buckets
- Validates: total across all buckets == total system count
- Calls `FUN_0053e190`—ratio/percentage scoring function

**Implication**: The original AI has a global strategic awareness layer that our Rust implementation lacks. It doesn't just evaluate systems one-at-a-time—it categorizes the entire galaxy first, then makes decisions based on the distribution.

### 2. System-Level Entity Deployment—`FUN_00537180` (381 lines)

**Address**: 0x00537180 | **Purpose**: Per-system fleet movement decisions

- Iterates systems via linked list at `this+0x40` using `FUN_0041c210`
- **Faction ownership check**: `(*(uint*)(entity + 0x24) >> 6 & 3) != local_c8`
  - Offset +0x24 is the entity's ownership byte
  - Bits 6-7 encode faction: 0=neutral, 1=Alliance, 2=Empire
- Validates troops at each system (family 0x14-0x1b)
- Calls `FUN_00553410` for faction-specific system operations
- Checks sectors (family 0xa0-0xa2)
- **Triggers fleet movement** via `FUN_00520580` to set deployment parameters

**This is the fleet movement decision point.** When this function determines a system needs reinforcement or an enemy system is vulnerable, it calls FUN_00520580 to issue the movement order.

### 3. Garrison/Fleet Strength Aggregator—`FUN_00502020` (897 lines)

**Address**: 0x00502020 | **Purpose**: Evaluate whether a system/fleet is strong enough

- Checks entity family ranges:
  - `0x30-0x3f`—capital ships
  - `0x1c-0x1f`—facilities
  - `0x10-0x13`—troops
- **Faction alignment**: `((byte)*(uint*)(entity + 0x24) ^ (byte)param_1[9]) & 0xc0) == 0`
  - XOR of ownership bytes, masked to bits 6-7
  - Returns true only if same faction
- Calls sub-functions:
  - `FUN_005007a0`—fleet unit counter (iterates type=3 entities)
  - `FUN_004f6b50`—entity resolver
  - `FUN_005039d0`—facility iterator (family 0x1c-0x20)
  - `FUN_00504c40`—troop iterator (family 0x10-0x14)
- Compares entity counts against capacity:
  - `vtable+0x248`—`GetMaxHullValue()` (ship capacity)
  - `vtable+0x26c` / `+0x270`—additional capacity checks
- **Returns boolean**: is this system/fleet adequate?

**Implication**: The original AI evaluates garrison strength by counting EVERY entity type at a system—ships, troops, and facilities. Our AI only checks fleet presence (is_empty/is_not_empty). The original is much more nuanced.

### 4. Entity Placement Dispatcher—`FUN_00508660` (304 lines)

**Address**: 0x00508660 | **Purpose**: Route entities to type-specific placement handlers

Uses `vtable+0x04` (`GetFamilyId`) to dispatch by entity family:

| Family Range | Handler | Entity Type |
|-------------|---------|-------------|
| 0x08-0x0f | FUN_0050ac80 + FUN_0050bc60 + FUN_0050be00 + FUN_0050a1b0 | **Characters** |
| 0x10-0x13 | FUN_0050b610 + FUN_0050ba90 + FUN_0050b500 | **Troops** |
| 0x1c-0x1f | FUN_0050a1b0 + FUN_0050c6c0 | **Facilities** |
| 0x20-0x21 | FUN_0050c9f0 | Ships (type A) |
| 0x22 | FUN_0050b310 | Ships (type B) |
| 0x28 | FUN_0050bb70 + FUN_0050c180 | Ships (type C) |
| 0x29 | FUN_0050c1b0 | Ships (type D) |
| 0x2a | FUN_0050c1e0 | Ships (type E) |
| 0x2c-0x2f | FUN_0050c560 | Ships (type F) |

**Character assignment** happens through `FUN_0050ac80` (character placement) + `FUN_0050a1b0` (faction-conditioned placement). The faction check uses `param_1[9] >> 6 & 3`—the same ownership bit pattern.

### 5. System-Level Deployment Companion—`FUN_005385f0` (252 lines)

**Address**: 0x005385f0 | **Purpose**: Second pass of system evaluation

- Same linked-list iteration as FUN_00537180
- Same faction check: `(*(uint*)(local_94 + 0x24) >> 6 & 3) != local_7c`
- Calls `FUN_00520580` for deployment parameters
- Calls `FUN_00551190`—collection setup for deployment
- Calls `FUN_00506ea0`—faction-specific evaluator
- Calls `FUN_0052e970`—scoring/evaluation function

This appears to be a **second evaluation pass** that handles edge cases or secondary deployments after FUN_00537180's primary pass.

### 6. System Validation—`FUN_00508250` (137 lines)

**Address**: 0x00508250 | **Purpose**: Pre-check for whether an operation is valid

Calls approximately 18 validator functions before allowing entity deployment. This is the gatekeeper that prevents invalid fleet movements or character assignments.

## Helper Functions

| Address | Lines | Purpose |
|---------|-------|---------|
| FUN_005039d0 |—| Facility iterator (family 0x1c-0x20) |
| FUN_00504c40 |—| Troop iterator (family 0x10-0x14) |
| FUN_005007a0 |—| Fleet unit counter |
| FUN_004f6b50 |—| Entity resolver by DatId |
| FUN_00520580 |—| **Set movement/deployment orders** |
| FUN_00553b80 |—| Faction resolver from system |
| FUN_005597e0 |—| Faction-specific system count getter |
| FUN_0053e190 |—| Math/scoring function (ratio or percentage) |
| FUN_005202d0 |—| System pre-validation |
| FUN_00551190 |—| Collection setup for deployment |
| FUN_00504e60 |—| Entity lookup with family filter |
| FUN_0050a1b0 |—| Faction-conditioned entity placement |
| FUN_0050ac80 |—| Character placement |
| FUN_00506ea0 |—| Faction-specific evaluator |
| FUN_0052e970 |—| Scoring/evaluation function |

## Key Patterns

### Faction Ownership Encoding

The original encodes faction as bits 6-7 of offset +0x24 on every entity:
```c
faction = (*(uint*)(entity + 0x24) >> 6) & 3;
// 0 = neutral, 1 = Alliance, 2 = Empire
```

Same-faction check via XOR:
```c
same_faction = (((byte)*(uint*)(entity + 0x24) ^ (byte)param_1[9]) & 0xc0) == 0;
```

### Entity Family Byte Ranges

| Range | Entity Type |
|-------|------------|
| 0x08-0x0f | Characters (major + minor) |
| 0x10-0x13 | Troops |
| 0x14-0x1b | Troop classes |
| 0x1c-0x1f | Facilities |
| 0x20-0x2f | Ships (various sub-types) |
| 0x30-0x3f | Capital ships |
| 0x90 | Explored systems |
| 0x92 | Unexplored systems |
| 0xa0-0xa2 | Sectors |

## What Our Rust AI Is Missing

| Feature | Original (REBEXE.EXE) | Current (ai.rs) |
|---------|----------------------|-----------------|
| Galaxy-wide evaluation | FUN_00519d00: 7-bucket system categorization | None—evaluates systems individually |
| Strength assessment | FUN_00502020: counts ships + troops + facilities | Only checks `s.fleets.is_empty()` |
| Entity-type routing | FUN_00508660: 10+ family-specific handlers | Only places characters on missions |
| Two-pass evaluation | FUN_00537180 + FUN_005385f0 | Single pass |
| System validation | FUN_00508250: 18 pre-checks | No validation beyond `on_mission` check |
| Faction bit encoding | `+0x24 >> 6 & 3` | `is_alliance: bool` (simpler but equivalent) |

## Recommendations for Open Rebellion

1. **Add galaxy evaluation phase**: Before per-system decisions, categorize all systems into strategic buckets (our-controlled, enemy-controlled, contested, unoccupied, HQ, frontier)
2. **Implement garrison strength scoring**: Count total ship hulls + troop strength + facility count at each system, not just fleet presence/absence
3. **Add two-pass evaluation**: First pass identifies needs (defense, reinforcement, attack targets). Second pass matches available assets to needs.
4. **Use `controlling_faction` + asset counting**: The original checks every entity's faction bits. Our simplified `controlling_faction` field serves the same purpose when combined with fleet/facility counting.

## Connection to AutoResearch

The original AI has ~20 tunable thresholds across these functions (strength adequacy, bucket boundaries, scoring ratios). These map directly to the `AiConfig` struct in our autoresearch plan. The Karpathy loop can discover optimal values for these thresholds without hand-tuning—something LucasArts spent months doing manually.
