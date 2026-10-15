---
title: "Game Balance Parameters (GNPRTB)"
description: "213 parameter entries x 8 difficulty values controlling combat and economy"
category: "mechanics"
created: 2026-03-16
updated: 2026-10-14
game_system: "gnprtb"
sources:
  - type: "ghidra"
    file: "ghidra/notes/combat-formulas.md"
  - type: "code"
    file: "crates/rebellion-core/src/world/mod.rs"
  - type: "community"
    url: "https://swrebellion.net"
related:
  - "space-combat"
  - "bombardment"
tags: ["gnprtb", "balance", "difficulty", "parameters"]
---

# Game Balance Parameters (GNPRTB)

## What You See

The game has 8 difficulty settings — Development, Alliance Easy/Medium/Hard, Empire Easy/Medium/Hard, and Multiplayer. These affect combat damage, mission success rates, production speeds, and other values. GNPRTB is the master balance table that controls all difficulty-sensitive constants.

## How It Works

### Table Structure

GNPRTB.DAT contains **213 entries**, each with 8 difficulty-indexed values:

```
parameter_id:      u16  — unique ID for this parameter
development:       u32  — value at development difficulty
alliance_sp_easy:  u32  — Alliance single-player easy
alliance_sp_medium: u32
alliance_sp_hard:  u32
empire_sp_easy:    u32
empire_sp_medium:  u32
empire_sp_hard:    u32
multiplayer:       u32
```

### Accessor Pattern

```
value = gnprtb.value(param_id, difficulty)
```

Where `difficulty` maps to column index 0-7 (Development, Alliance Easy/Med/Hard, Empire Easy/Med/Hard, Multiplayer). An object's `+0x24 >> 4 & 3` numbers its copy (master, Alliance view, Empire view; `ghidra/notes/entity-system.md`), not a column.

### Parameter ID Ranges

| Range | Category | Count |
|-------|----------|-------|
| `0x0a00` | General parameters | 34 mapped |
| `0x1400` | `0x1400` group (`0x1400` is the transit divisor) | 77 mapped |
| **Total mapped** | | **111 of 213** |

The remaining 102 parameters have known IDs but unconfirmed semantic names. Metasharp (`StarWarsRebellionEditor.NET`) provides names for 61 of the 213 entries.

### Key Parameters

| ID (hex) | Purpose | Base Value |
|----------|---------|------------|
| `0x1400` | Transit distance divisor (`FUN_0055d8c0`) | 5 |
| `0x0305` | Repair rate modifier | varies |
| `0x0386` | Production rate modifier | varies |

`FUN_0055d8c0` divides the distance between two systems by `GNPRTB[0x1400]` to give a travel time in days (`ghidra/notes/build-delivery.md`). Bombardment reads `0x600`-`0x606` (leadership divisors, the hit-roll range, energy and raw-material values, the Death Star's character roll) and 7705 (the support loss); see [Bombardment](bombardment.md).

### Difficulty Asymmetry

Each faction has its own Easy/Medium/Hard column. This allows asymmetric difficulty — the AI's Alliance can receive production bonuses while the player's Empire faces combat penalties, or vice versa. In the base game data, most values are uniform across difficulty, but the infrastructure supports full asymmetry.

### Modding GNPRTB

GNPRTB is the primary lever for game balance mods. Change any parameter's value per-difficulty to:

- Scale combat damage up/down
- Adjust mission success probability bonuses
- Modify production speed multipliers
- Tune bombardment effectiveness
- Balance repair rates at shipyards

In Open Rebellion, GNPRTB values can be overridden via mod JSON patches against `GNPRTB.json`.

## Source Material

- Ghidra: [combat-formulas.md](../../ghidra/notes/combat-formulas.md) — parameter table structure, key addresses
- Community: Metasharp editor (`~/Desktop/Programming/StarWarsRebellionEditor.NET/`) — 61 named parameters
- Rust: `GnprtbParams` and `GnprtbEntry` in `crates/rebellion-core/src/world/mod.rs`

## Related

- [Space Combat](space-combat.md)
- [Bombardment](bombardment.md)
