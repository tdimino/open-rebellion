---
title: "Combat System—Reverse Engineering Summary"
description: "Complete combat call chain discovery from the system battle orchestrator"
category: "ghidra"
created: 2026-03-13
updated: 2026-03-16
---

# Combat System—Reverse Engineering Summary

## Discovery: Complete Combat Call Chain

### System Battle Orchestrator: FUN_00514a60
**The entry point for all combat resolution at a star system.**

```
FUN_00514a60(system, attacker_side, defender_side, battle_context)
  ├── FUN_005457f0 → FUN_00549910  (SPACE COMBAT)
  │     ├── FUN_005442f0  (initialize combat state)
  │     ├── FUN_00544da0  (fleet composition evaluation)
  │     ├── FUN_00544030  (WEAPON DAMAGE—calls FUN_00543b60 for each side)
  │     │     └── FUN_00543b60  (per-side weapon fire, vtable +0x1c4 dispatch)
  │     ├── FUN_00544130  (SHIELD ABSORPTION—83 lines)
  │     ├── FUN_005443f0  (HULL DAMAGE APPLICATION—54 lines)
  │     ├── FUN_005444e0  (FIGHTER ENGAGEMENT—53 lines)
  │     ├── FUN_005445d0  (combat result determination)
  │     └── FUN_00544a20  (post-combat cleanup)
  │
  ├── FUN_00560d50  (GROUND COMBAT—232 lines)
  │     ├── Iterates troops (family_id 0x14-0x1b)
  │     ├── Checks regiment strength at offset +0x96
  │     ├── FUN_005617b0  (Death Star handler, family_id 0x34)
  │     └── FUN_004ee350  (per-unit combat resolution)
  │           └── vtable +0x330  (notification dispatch → HullValueDamage etc.)
  │
  ├── FUN_00529d20  (fleet combat at object +0xe)
  │
  └── FUN_00556430  (BOMBARDMENT—78 lines)
        ├── FUN_00555540  (get fleet strength for a side)
        ├── FUN_00509620  (get combat stats as short[2])
        └── FUN_00555d30 → FUN_00555b30 → FUN_0055d8c0  (BOMBARDMENT FORMULA)
              ├── FUN_0055d860  (power ratio = EUCLIDEAN DISTANCE)
              │     = sqrt((atk[0]-def[0])² + (atk[1]-def[1])²)
              ├── / DAT_006bb6e8  (GNPRTB divisor parameter)
              └── FUN_0053e190  (apply difficulty modifier via DAT_00661a88)
```

## Confirmed Formulas

### Bombardment Damage
```
raw_power = sqrt((attacker_bombardment - defender_defense)² + (atk[1] - def[1])²)
damage = apply_modifier(raw_power / GNPRTB_PARAM[DAT_006bb6e8], difficulty)
if damage == 0: damage = 1  // minimum 1
```

### Ground Combat
- Iterates all troops at a system (DatId family 0x14-0x1b)
- Checks regiment strength at object offset +0x96 (short)
- Per-unit resolution via FUN_004ee350 → sets new value, fires notification

### Space Combat (Auto-Resolve)
7-phase pipeline:
1. Initialize → 2. Evaluate fleets → 3. Weapon damage → 4. Shield absorption
→ 5. Hull damage → 6. Fighter engagement → 7. Determine result

Weapon damage calls FUN_00543b60 twice (once per side) with vtable dispatch at +0x1c4 for weapon-type-specific calculation.

## Key Global Data Addresses

| Address | Purpose |
|---------|---------|
| DAT_006bb6e8 | Bombardment GNPRTB divisor |
| DAT_00661a88 | Difficulty modifier table |

## Entity Type Codes (DatId Family Byte)

| Range | Type |
|-------|------|
| 0x08-0x0f | Characters |
| 0x10-0x13 | Troops (general) |
| 0x14-0x1b | Troops/Special Forces (combat-eligible) |
| 0x1c-0x1f | Facilities |
| 0x30-0x3b | Capital Ships |
| 0x34 | Special (Death Star?) |
| 0x90-0x98 | Systems |

## Mission Type Codes

| Code | Mission |
|------|---------|
| 6 | Sabotage |
| 7 | Assassination |
| 0x15 (21) | Autoscrap |

## What We Have vs What We Need

### Have (sufficient to implement)
- Complete combat call chain traced from system battle to individual damage
- Bombardment formula (Euclidean distance / GNPRTB divisor)
- Space combat 7-phase pipeline identified
- Ground combat iteration pattern
- Entity type classification
- Notification/Observer dispatch architecture understood

### Need (next session)
- Decompile FUN_00544130 (shield absorption—83 lines, should contain shield formula)
- Decompile FUN_005443f0 (hull damage application—should contain hull damage formula)
- Decompile FUN_005444e0 (fighter engagement—squadron combat)
- Find the specific vtable handler for weapon damage calculation (vtable +0x1c4)
- Read DAT_006bb6e8 value to identify which GNPRTB parameter it is
- Trace FUN_00509620 to understand the combat stat pair format (short[2])

### Sufficient to Start Implementation
Even without all formulas perfectly decoded, we have enough to implement a **reasonable approximation**:
- Space combat: aggregate fleet firepower vs aggregate fleet defense
- Ground combat: iterate troops, compare attack vs defense ratings
- Bombardment: Euclidean distance formula with GNPRTB scaling
- Shield absorption: likely shield_strength - damage_remaining pattern
- Hull damage: remaining damage after shields → subtract from hull

The quadratic mission probability formula from rebellion2 still applies for all mission types. The *MSTB probability tables provide the exact coefficients.
