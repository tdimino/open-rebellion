---
title: "Orbital Bombardment"
description: "Decompiled orbital bombardment entry point and resolution logic (FUN_00556430)"
category: "ghidra"
created: 2026-03-13
updated: 2026-03-16
---

# Orbital Bombardment

## Entry Point

`FUN_00556430` (78 lines)—called from system battle orchestrator (`FUN_00514a60`).

## Flow

1. Check bombardment is allowed: vtable+0x38 must return true AND `this[0x14] & 0x800 == 0`
2. Get attacker side via `FUN_00504dc0(param_2)` → `piVar5`
3. Get defender side via `FUN_00504dc0(param_3)` → `puVar6`
4. Get fleet strength for each side via `FUN_00555540`
5. Extract IDs: `FUN_004025b0` for attacker (`uVar1`) and defender (`uVar2`)
6. If same faction (`uVar1 == uVar2`): skip—no self-bombardment
7. Call `FUN_00555d30` → `FUN_00555b30` for bombardment calculation
8. Apply result: `FUN_004f7390(system, base + damage, context)`

## Bombardment Formula

### Full Call Chain
```
FUN_00556430 (bombardment handler)
  → FUN_00555d30 (setup)
    → FUN_00555b30 (calculation)
      → FUN_00509620 (get combat stats as short[2])
      → FUN_0055d8c0 (damage formula)
        → FUN_0055d860 (power ratio)
        → / DAT_006bb6e8 (GNPRTB divisor)
        → FUN_0053e190 (difficulty modifier)
```

### FUN_0055d8c0—Damage Formula
```c
int bombardment_damage(short* defender, short* attacker, int difficulty_param) {
    int raw_power = power_ratio(defender, attacker);  // FUN_0055d860
    if (raw_power == 0) return 0;
    int result = apply_modifier(raw_power / GNPRTB_DIVISOR, difficulty_param);
    return result == 0 ? 1 : result;  // minimum 1 damage
}
```

### FUN_0055d860—Power Ratio (Euclidean Distance)
```c
float power_ratio(short* defender, short* attacker) {
    int dx = attacker[0] - defender[0];
    int dy = attacker[1] - defender[1];
    return sqrt(dx*dx + dy*dy);  // FUN_0053e1d0
}
```

The `short[2]` stats from `FUN_00509620` are likely:
- `[0]` = bombardment attack strength (attacker) / bombardment defense (defender)
- `[1]` = secondary stat (possibly detection or maneuverability modifier)

### Global Constants

| Address | Purpose | Notes |
|---------|---------|-------|
| DAT_006bb6e8 | GNPRTB bombardment divisor | Scales raw power to damage |
| DAT_00661a88 | Difficulty modifier table | Used by FUN_0053e190 |

## Bombardment Disabled Flag

`this[0x14] & 0x800`—when set, bombardment is blocked (system already under bombardment or protected).

## Decompiled Files

- `FUN_00556430_bombardment.c`—main bombardment handler (78 lines)
- `FUN_00555d30.c`—bombardment setup (calls FUN_00555b30)
- `FUN_00555b30.c`—bombardment calculation orchestrator
- `FUN_0055d8c0.c`—damage formula (Euclidean distance / GNPRTB)
- `FUN_0055d860.c`—power ratio (sqrt of stat deltas)
- `FUN_0053e190.c`—difficulty modifier application
- `FUN_0053e1d0.c`—sqrt function
- `FUN_0053e170.c`—modifier helper
- `FUN_00509620_combat_stats.c`—combat stat extractor
- `FUN_00555540.c`—fleet strength getter
