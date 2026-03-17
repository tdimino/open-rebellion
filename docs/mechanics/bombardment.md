---
title: "Orbital Bombardment"
description: "Euclidean distance damage formula with GNPRTB scaling"
category: "mechanics"
created: 2026-03-16
updated: 2026-03-16
game_system: "bombardment"
sources:
  - type: "ghidra"
    file: "ghidra/notes/bombardment.md"
  - type: "ghidra"
    file: "ghidra/notes/COMBAT-SUMMARY.md"
  - type: "code"
    file: "crates/rebellion-core/src/bombardment.rs"
  - type: "prima"
    page: "Chapter 7: Fleet Combat"
related:
  - "space-combat"
  - "gnprtb"
tags: ["combat", "bombardment", "orbital", "formula"]
---

# Orbital Bombardment

## What You See

A fleet in orbit over an enemy-controlled system can bombard the planet. Bombardment damages planetary defenses and shifts control. Larger fleets with higher bombardment ratings deal more damage. Even a weak fleet deals at least 1 damage per strike. You cannot bombard your own systems.

## How It Works

Bombardment is handled by `FUN_00556430` (78 lines), called from the system battle orchestrator.

### Full Call Chain

```
FUN_00556430 (bombardment handler)
  -> FUN_00555d30 (setup)
    -> FUN_00555b30 (calculation)
      -> FUN_00509620 (get combat stats as short[2])
      -> FUN_0055d8c0 (damage formula)
        -> FUN_0055d860 (power ratio = Euclidean distance)
        -> / DAT_006bb6e8 (GNPRTB divisor, param_id=0x1400)
        -> FUN_0053e190 (difficulty modifier)
```

### Damage Formula

```
atk = (fleet_bombardment_stat, fleet_secondary_stat)
def = (system_defense_stat, system_secondary_stat)

raw_power = sqrt((atk[0] - def[0])^2 + (atk[1] - def[1])^2)

damage = floor(raw_power / GNPRTB[0x1400])

if damage == 0 and raw_power > 0:
    damage = 1    # minimum 1 damage
```

The stat pairs from `FUN_00509620` are:
- `[0]` = bombardment attack strength (attacker) / bombardment defense (defender)
- `[1]` = secondary stat (maneuverability or detection modifier)

### GNPRTB Divisor

The bombardment divisor is GNPRTB parameter `0x1400`. In the base game data, this value is **5** across all difficulty modes. It scales raw power down to final damage — higher divisor means less damage per unit of raw power.

### Guards

1. **No self-bombardment**: if the attacking fleet's faction matches the system's controlling faction, damage is 0.
2. **Bombardment disabled flag**: `this[0x14] & 0x800` blocks bombardment when the system is already under bombardment or protected.
3. **Zero raw power**: if `raw_power == 0`, the function returns 0 (minimum-1 only applies when there was some power but it divided to zero).

### Difficulty Modifier

Applied via `FUN_0053e190` using the difficulty modifier table at `DAT_00661a88`. In practice, the base game's GNPRTB values are uniform across difficulty, so this modifier is effectively 1.0.

## Source Material

- Ghidra: [bombardment.md](../../ghidra/notes/bombardment.md) — full formula chain with pseudocode
- Ghidra: [COMBAT-SUMMARY.md](../../ghidra/notes/COMBAT-SUMMARY.md) — confirmed formulas section
- Rust: `crates/rebellion-core/src/bombardment.rs` — `BombardmentSystem::resolve_bombardment()`

## Related

- [Space Combat](space-combat.md)
- [Game Balance Parameters](gnprtb.md)
