---
title: "AI Behavior Analysis — Original REBEXE.EXE"
description: "The original's new-game stage `0xb` and per-family dispatch checks; the computer player itself is in ai-order-emission.md"
category: "ghidra"
created: 2026-03-20
updated: 2026-10-14
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

# AI Behavior Analysis — Original REBEXE.EXE

## Architecture: No Single AI Function

The original Rebellion AI is **event-driven and distributed** across a 6-function pipeline, not a monolithic decision function. It uses the CRebObject → CNotifyObject hierarchy with a setter-notify-event pattern (`cpp-class-hierarchy.md`). The AI never runs as a single top-level call — it's triggered through the notification/observer system and evaluated per-system.

## The AI Pipeline (6 Functions)

```
0. New-game stage 0xb (FUN_00519d00), before play starts
   ↓ divides the systems between the sides (galaxy-limits.md)
2. System Iteration (FUN_00537180 / FUN_005385f0)
   ↓ for each system: check faction, evaluate entities, trigger deployment
3. Capital-ship contents check (FUN_00502020)
   ↓ per ship: characters and special forces (0x30-0x3f), fighters (0x1c-0x1f), regiments (0x10-0x13)
4. System Validation (FUN_00508250)
   ↓ pre-check: can this operation proceed? (~18 validators)
5. System object check (FUN_00508660)
   ↓ run the checks for the object's family (fleets, regiments, fighters, HQ, facilities)
6. Movement Orders (FUN_00520580)
   ↓ set fleet deployment/movement parameters
```

## Function Details

### 1. New-Game System Distribution — `FUN_00519d00` (252 lines)

**Address**: 0x00519d00 | **Purpose**: Stage `0xb` of the new-game driver `FUN_005136d0`: splits the created systems into side lists by SDPRTB `0x1e00`/`0x1e01` (`galaxy-limits.md`, `facility-ownership.md`). It runs once at setup and is not part of the computer player.

- Iterates the systems via `FUN_0051cad0(0x90)` (systems are families `0x90` core and `0x92` rim)
- Checks specific DatIds: `0x92000121` (unexplored), `0x90000109` (Coruscant)
- Calls `FUN_005597e0(faction, type)` with 4 combinations:
  - `(1, 0)` — Alliance systems, type 0
  - `(2, 0)` — Empire systems, type 0
  - `(1, 1)` — Alliance systems, type 1
  - `(2, 1)` — Empire systems, type 1
- Distributes systems into **7 strategic buckets** at offsets:
  - `+0x100`, `+0x110` — primary faction buckets
  - `+0xe0`, `+0xf0` — secondary buckets
  - `+0x120`, `+0x130`, `+0x140` — tertiary buckets
- Validates: total across all buckets == total system count
- Calls `FUN_0053e190` — ratio/percentage scoring function

The bucket counts come from list sizes (`FUN_004f4290`), so they follow the DAT and the galaxy-size filter; stage `0xc` (`FUN_0051a280`) creates each system with its list's side.

### 2. System-Level Entity Deployment — `FUN_00537180` (381 lines)

**Address**: 0x00537180 | **Purpose**: Per-system fleet movement decisions

- Iterates systems via linked list at `this+0x40` using `FUN_0041c210`
- **Faction ownership check**: `(*(uint*)(entity + 0x24) >> 6 & 3) != local_c8`
  - Offset +0x24 is the entity's ownership byte
  - Bits 6-7 encode faction: 1=Alliance, 2=Empire, 3=Neutral, 0 unset (`side-numbering.md`)
- Tests objects at each system for the capital-ship families `0x14`-`0x1b` (`FUN_00537180.c:112`)
- Calls `FUN_00553410` for faction-specific system operations
- Checks per-system objects of family `0xa0`-`0xa1` (`galaxy-limits.md`; sectors are `0x80`-`0x8f`)
- **Triggers fleet movement** via `FUN_00520580` to set deployment parameters

**This is the fleet movement decision point.** When this function determines a system needs reinforcement or an enemy system is vulnerable, it calls FUN_00520580 to issue the movement order.

### 3. Capital-Ship Contents Check — `FUN_00502020` (897 lines)

**Address**: 0x00502020 | **Purpose**: Validate a capital ship (`param_1`, which answers slot `+0x248` max hull) and what it carries

- Every child must be in one of three family ranges and on the ship's side:
  - `0x30-0x3f` — characters (major `0x30`-`0x35`, minor `0x38`) and special forces (`0x3c`)
  - `0x1c-0x1f` — fighter squadrons
  - `0x10-0x13` — regiments
- **Side alignment**: `((byte)*(uint*)(entity + 0x24) ^ (byte)param_1[9]) & 0xc0) == 0`
  - XOR of the side bits 6-7
- Then checks the ship's own fields: hull `+0x60` within 0..slot `+0x248`, `+0x50` bit 3 at full hull, and `FUN_00500670`/`FUN_00500680`
- Calls sub-functions:
  - `FUN_005007a0` — walks the ship's children (`FUN_004f25c0`, kind 3)
  - `FUN_004f6b50` — entity resolver
  - `FUN_005039d0` — fighter iterator (`[0x1c,0x20)`), count against slot `+0x26c`
  - `FUN_00504c40` — regiment iterator (`[0x10,0x14)`), count against slot `+0x270`
- **Returns boolean**: whether the ship and its contents are valid

**Port**: `system_strength()` in `ai.rs` scores a system by ship hulls, regiments and defence facilities and cites no source; this function checks one capital ship.

### 4. System Object Check — `FUN_00508660` (304 lines)

**Address**: 0x00508660 | **Purpose**: Run the system's checks for one object by its family

Uses `vtable+0x04` (`GetFamilyId`) and ANDs the results of the checks for each matching range:

| Family Range | Checks | Object |
|-------------|---------|-------------|
| 0x20-0x2f | FUN_0050b760 | Facilities (any) |
| 0x20-0x21 | FUN_0050c9f0 | Alliance HQ |
| 0x22 | FUN_0050b310 | KDY-150 battery |
| 0x2c-0x2f | FUN_0050c560 | Mines, refineries |
| 0x28 | FUN_0050bb70 + FUN_0050c180 | Shipyard |
| 0x29 | FUN_0050c1b0 | Training facility |
| 0x2a | FUN_0050c1e0 | Construction yard |
| 0x08-0x0f | FUN_0050ac80 + FUN_0050bc60 + FUN_0050be00 + FUN_0050a1b0 + FUN_0050c0b0 + FUN_0050c6c0 + FUN_0050ac10 | Fleets |
| 0x1c-0x1f | FUN_0050a1b0 + FUN_0050c6c0 + FUN_0050ac10 | Fighter squadrons |
| 0x10-0x13 | FUN_0050b610 + FUN_0050ba90 + FUN_0050b500 + FUN_0050c540 | Regiments |

`FUN_0050a1b0` takes the object's side (`param_1[9] >> 6 & 3`) and runs only when `param_2` is nonzero. Facility identities: `facility-order-lists.md`.

### 5. System-Level Deployment Companion — `FUN_005385f0` (252 lines)

**Address**: 0x005385f0 | **Purpose**: Second pass of system evaluation

- Same linked-list iteration as FUN_00537180
- Same faction check: `(*(uint*)(local_94 + 0x24) >> 6 & 3) != local_7c`
- Calls `FUN_00520580` for deployment parameters
- Calls `FUN_00551190` — collection setup for deployment
- Calls `FUN_00506ea0` — faction-specific evaluator
- Calls `FUN_0052e970` — scoring/evaluation function

This appears to be a **second evaluation pass** that handles edge cases or secondary deployments after FUN_00537180's primary pass.

### 6. System Validation — `FUN_00508250` (137 lines)

**Address**: 0x00508250 | **Purpose**: Pre-check for whether an operation is valid

Calls approximately 18 validator functions before allowing entity deployment. This is the gatekeeper that prevents invalid fleet movements or character assignments.

## Helper Functions

| Address | Lines | Purpose |
|---------|-------|---------|
| FUN_005039d0 | — | Fighter iterator (families `[0x1c,0x20)`) |
| FUN_00504c40 | — | Regiment iterator (families `[0x10,0x14)`) |
| FUN_005007a0 | — | Fleet unit counter |
| FUN_004f6b50 | — | Entity resolver by DatId |
| FUN_00520580 | — | **Set movement/deployment orders** |
| FUN_00553b80 | — | Faction resolver from system |
| FUN_005597e0 | — | Returns a side's SDPRTB `0x1e00`/`0x1e01` share (side 3: the remainder of 100) |
| FUN_0053e190 | — | Math/scoring function (ratio or percentage) |
| FUN_005202d0 | — | System pre-validation |
| FUN_00551190 | — | Collection setup for deployment |
| FUN_00504e60 | — | Entity lookup with family filter |
| FUN_0050a1b0 | — | Side-conditioned check (fleets, fighters) |
| FUN_0050ac80 | — | Fleet check |
| FUN_00506ea0 | 13 | Faction-specific evaluator pointer (Alliance +0xc4, Empire +0xc8) |
| FUN_0052e970 | 53 | Binary capacity check (NOT a scoring function — see 2026-03-23 findings) |

## Key Patterns

### Faction Ownership Encoding

The original encodes faction as bits 6-7 of offset +0x24 on every entity:
```c
faction = (*(uint*)(entity + 0x24) >> 6) & 3;
// 1 = Alliance, 2 = Empire, 3 = Neutral, 0 = unset
```

Same-faction check via XOR:
```c
same_faction = (((byte)*(uint*)(entity + 0x24) ^ (byte)param_1[9]) & 0xc0) == 0;
```

### Entity Family Byte Ranges

From the DAT `family_id` fields and the code's range literals (`ground-unit-assets.md`, `record-registry.md`, `death-star-strategic.md`):

| Range | Entity Type |
|-------|------------|
| 0x08-0x0f | Fleets |
| 0x10-0x13 | Regiments (TROOPSD `0x10`) |
| 0x14-0x1b | Capital ships (CAPSHPSD `0x14`; the Death Star `0x18`) |
| 0x1c-0x1f | Fighter squadrons (FIGHTSD `0x1c`) |
| 0x20-0x21 | Alliance HQ |
| 0x22-0x25 | Defence facilities (KDY-150, LNR, GenCore, Death Star shield) |
| 0x28-0x2a | Shipyard, training facility, construction yard |
| 0x2c-0x2d | Mine, refinery |
| 0x30-0x35 | Major characters (`0x34` Palpatine) |
| 0x38 | Minor characters |
| 0x3c | Special forces |
| 0x41-0x73 | Missions |
| 0x80-0x8f | Sectors (SECTORSD `0x80`) |
| 0x90 | Core systems |
| 0x92 | Rim systems |
| 0xa0-0xa5 | Per-system objects (`galaxy-limits.md`) |

Port status: `agent_docs/systems/ai-parity-tracker.md`.
