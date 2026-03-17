---
title: "Space Combat"
description: "7-phase auto-resolve pipeline for fleet engagements"
category: "mechanics"
created: 2026-03-16
updated: 2026-03-16
game_system: "space-combat"
sources:
  - type: "ghidra"
    file: "ghidra/notes/space-combat.md"
  - type: "ghidra"
    file: "ghidra/notes/COMBAT-SUMMARY.md"
  - type: "code"
    file: "crates/rebellion-core/src/combat.rs"
  - type: "prima"
    page: "Chapter 7: Fleet Combat"
related:
  - "ground-combat"
  - "bombardment"
  - "gnprtb"
tags: ["combat", "fleet", "auto-resolve", "pipeline"]
---

# Space Combat

## What You See

When two opposing fleets occupy the same system, space combat triggers. Capital ships exchange turbolaser, ion cannon, and laser fire. Shields absorb incoming damage before it reaches the hull. Fighters launch from carriers and engage enemy squadrons and capital ships. Ships that lose all hull points are destroyed. The side with surviving ships wins.

## How It Works

Space combat runs a **7-phase auto-resolve pipeline**, traced from `FUN_005457f0` -> `FUN_00549910` in REBEXE.EXE.

### Phase Pipeline

| # | Phase | Gate | What Happens |
|---|-------|------|-------------|
| 1 | Init | -- | Build mutable hull snapshots for each ship (`FUN_005442f0`) |
| 2 | Fleet eval | -- | Check if either side has armed ships (`FUN_00544da0`, 96 lines) |
| 3 | Weapon fire | `ACTIVE && !PHASES_ENABLED` | Each side fires all weapons at the other (`FUN_00543b60`) |
| 4 | Shield absorb | `PHASES_ENABLED` | Shields absorb a fraction of incoming damage (`FUN_00544130`) |
| 5 | Hull damage | `PHASES_ENABLED` | Remaining damage reduces hull points (`FUN_005443f0`) |
| 6 | Fighter engage | `PHASES_ENABLED` | Squadrons attack enemy ships and each other (`FUN_005444e0`) |
| 7 | Result | -- | Count survivors, determine winner (`FUN_005445d0`, 175 lines) |

Post-combat cleanup runs in phase 8 (`FUN_00544a20`, 86 lines).

### Phase Gate Gotcha

Weapon fire (phase 3) gates on `ACTIVE && !PHASES_ENABLED`. This means weapons fire **before** the pipeline is fully armed. `PHASES_ENABLED` is set only **after** weapon fire completes. Phases 4-6 then gate on `PHASES_ENABLED`.

### CombatPhaseFlags Bitfield

The combat state machine uses a bitfield at C++ offset `+0x58`:

```
bit 0 (0x01)  — Space combat active
bit 1 (0x02)  — Weapon fire phase (attacker side)
bit 2 (0x04)  — Weapon fire type code
bit 5 (0x20)  — Shield absorb type code
bit 6 (0x40)  — Combat phases enabled (shields, hull, fighters)
bit 7 (0x80)  — Hull damage type code
bit 8 (0x100) — Fighter engagement type code
```

### Weapon Arcs

Each capital ship carries weapons across 4 arcs (fore, aft, port, starboard) in 3 types:

- **Turbolaser** — primary damage dealer
- **Ion cannon** — disables systems (shields, weapons)
- **Laser cannon** — light anti-fighter armament

Total firepower is the sum across all arcs, scaled by `weapon_nibble / 15` (a 4-bit allocation value from `0x00` to `0x0F`).

### Shield Mechanics

Shield absorption uses `shield_nibble / 15` to determine what fraction of incoming damage is absorbed before reaching the hull. The shield recharge allocation value (offset `+0x64` bits 0-3) controls shield regeneration rate per phase.

### Difficulty Modifier

Difficulty is extracted from `*(uint*)(this + 0x24) >> 4 & 3` — a 2-bit field encoding difficulty level (0-3). This modifies combat outcomes per the GNPRTB parameter table.

### Winner Determination

Phase 7 checks each unit's alive flag (`offset +0xac, bit 0`). Fighter exception: dead fighters with `+0x50 & 0x08` still count as combat-ready. Family `0x73`/`0x74` entities (Death Star/special) take a separate path via `FUN_00534640`.

The side with surviving ships and/or fighters wins. If both sides still have units, the result is a draw.

## Source Material

- Ghidra: [space-combat.md](../../ghidra/notes/space-combat.md) — full 7-phase table with addresses
- Ghidra: [COMBAT-SUMMARY.md](../../ghidra/notes/COMBAT-SUMMARY.md) — complete call chain
- Rust: `crates/rebellion-core/src/combat.rs` — `CombatSystem::resolve_space()`

## Related

- [Ground Combat](ground-combat.md)
- [Bombardment](bombardment.md)
- [Game Balance Parameters](gnprtb.md)
