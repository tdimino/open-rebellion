---
title: "Ground Combat"
description: "Troop-by-troop regiment engagement resolution"
category: "mechanics"
created: 2026-03-16
updated: 2026-03-16
game_system: "ground-combat"
sources:
  - type: "ghidra"
    file: "ghidra/notes/ground-combat.md"
  - type: "code"
    file: "crates/rebellion-core/src/combat.rs"
  - type: "prima"
    page: "Chapter 8: Ground Operations"
related:
  - "space-combat"
  - "bombardment"
tags: ["combat", "troops", "ground", "regiment"]
---

# Ground Combat

## What You See

When opposing ground forces occupy the same system, ground combat triggers. Troop regiments from each side engage one another. Each unit's regiment strength determines its combat effectiveness. Units lose strength during combat, and those reduced to zero are destroyed. The side with surviving troops wins control of the system.

## How It Works

Ground combat is handled by `FUN_00560d50` (232 lines), called from the system battle orchestrator (`FUN_00514a60`).

### Entity Classification

The combat dispatcher classifies entities by DatId family byte (`id >> 24`):

| Family Range | Type | Role |
|-------------|------|------|
| `0x14-0x1b` | Troops / Special Forces | Combat-eligible ground units |
| `0x30-0x3b` | Capital Ships | Delegated to space combat path |
| `0x34` | Death Star | Special handler (`FUN_005617b0`, 68 lines) |
| `0x08-0x0f` | Characters | Side validation only |

### Combat Flow

1. Check entity type — ships go to space path, troops enter ground combat
2. Validate parent object is also a troop (family `0x14-0x1b`)
3. If Death Star (`0x34`): call special handler `FUN_005617b0`
4. Iterate all troops at the system via `FUN_004f25c0(type=3)`
5. For each troop: check `regiment_strength` at offset `+0x96` (short)
6. If strength > 0: call per-unit resolution `FUN_004ee350`
7. Validate both sides are valid characters — skip same-side pairs (no friendly fire)

### Per-Unit Resolution (FUN_004ee350)

Each engagement between an attacker and defender unit follows this 30-line function:

```
1. Read old_strength from unit offset +0x96
2. If old_strength != new_value:
   a. Set unit->strength = new_value
   b. Notify attacker side (side 1)
   c. Notify defender side (side 2)
   d. Fire vtable +0x330 dispatch (HullValueDamage, SquadSizeDamage, etc.)
```

The hit probability is proportional to attacker strength versus combined strength. When the attacker hits, the defender loses approximately 25% of its current strength (minimum 1). When the defender wins the exchange, the attacker takes the same proportional loss.

### Regiment Strength

- **Type**: `short` (16-bit signed integer) at C++ offset `+0x96`
- **Zero**: unit is destroyed / non-combat
- **Modified by**: `FUN_004ee350` which sets the new value and notifies observers
- **Matches**: `CapitalShipEntry` / `TroopEntry` record layout

### Death Star Handler

Family `0x34` triggers `FUN_005617b0` (68 lines) instead of normal ground combat. This handles planet destruction via the superlaser — the Death Star always wins ground engagements at its target system.

## Source Material

- Ghidra: [ground-combat.md](../../ghidra/notes/ground-combat.md) — full decompilation with pseudocode
- Ghidra: [COMBAT-SUMMARY.md](../../ghidra/notes/COMBAT-SUMMARY.md) — call chain context
- Rust: `crates/rebellion-core/src/combat.rs` — `CombatSystem::resolve_ground()`

## Related

- [Space Combat](space-combat.md)
- [Bombardment](bombardment.md)
