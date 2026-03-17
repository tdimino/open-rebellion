---
title: "Game Balance Parameters (GNPRTB)"
description: "213 parameter entries x 8 difficulty values controlling combat and economy"
category: "mechanics"
created: 2026-03-16
updated: 2026-03-16
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

Where `difficulty` maps to column index 0-7. The packed C++ field `*(uint*)(this + 0x24) >> 4 & 3` provides a 2-bit selector (0-3) per faction side; the full 8-level index is derived at the caller layer by combining side and faction (Development, Alliance Easy/Med/Hard, Empire Easy/Med/Hard, Multiplayer).

### Parameter ID Ranges

| Range | Category | Count |
|-------|----------|-------|
| `0x0a00` | General parameters | 34 mapped |
| `0x1400` | Combat parameters | 77 mapped |
| **Total mapped** | | **111 of 213** |

The remaining 102 parameters have known IDs but unconfirmed semantic names. Metasharp (`StarWarsRebellionEditor.NET`) provides names for 61 of the 213 entries.

### Key Parameters

| ID (hex) | Purpose | Base Value |
|----------|---------|------------|
| `0x1400` | Bombardment damage divisor | 5 |
| `0x0305` | Repair rate modifier | varies |
| `0x0386` | Production rate modifier | varies |

The bombardment formula divides raw power by `GNPRTB[0x1400]`, so a higher value means less bombardment damage. Combat damage, shield absorption rates, and mission modifiers all reference other GNPRTB entries.

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
