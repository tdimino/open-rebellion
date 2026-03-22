---
title: "Space Combat Auto-Resolve"
description: "Decompiled space combat entry point and 7-phase auto-resolve pipeline"
category: "ghidra"
created: 2026-03-13
updated: 2026-03-16
---

# Space Combat Auto-Resolve

## Entry Point

`FUN_005457f0` → `FUN_00549910` (77 lines)

Called from the system battle orchestrator (`FUN_00514a60`) when space combat is triggered.

## 7-Phase Pipeline

Each phase follows the same pattern:
1. Get stats via a getter function → returns two sides' combat objects
2. Call per-side resolver with side=1 (attacker fires first), then side=0 (defender fires)
3. Resolver dispatches via vtable to the actual combat calculation

| # | Phase | Gate Flag | Stat Getter | Per-Side Resolver | Vtable | Notif Handler | Type Code |
|---|-------|-----------|-------------|-------------------|--------|---------------|-----------|
| 1 | Init | — | — | FUN_005442f0 | — | — | — |
| 2 | Fleet eval | — | — | FUN_00544da0 (96 lines) | — | — | — |
| 3 | Weapon fire | `+0x58 & 0x01` (active) `& ~0x40` (enabled) | FUN_005434b0 | FUN_00543b60 | `+0x1c4` | FUN_0054a1d0 → vtable `+0x1f4` | `0x04` |
| 4 | Shield absorb | `+0x58 & 0x40` | FUN_005434b0 | FUN_00543bd0 | `+0x1c8` | FUN_0054a260 | `0x20` |
| 5 | Hull damage | `+0x58 & 0x40` | FUN_00543550 | FUN_00543cb0 | `+0x1d0` | FUN_0054a380 | `0x80` |
| 6 | Fighter engage | `+0x58 & 0x40` | FUN_00543690 | FUN_00543d20 | `+0x1d4` | FUN_0054a410 | `0x100` |
| 7 | Result | — | 5 getters | FUN_005445d0 (175 lines) | — | — | — |
| 8 | Post-combat | — | — | FUN_00544a20 (86 lines) | — | — | — |

## Flag Register at `this+0x58`

Bitfield controlling combat state:

| Bit | Mask | Meaning |
|-----|------|---------|
| 0 | `0x01` | Space combat active |
| 1 | `0x02` | Weapon fire phase (attacker side) |
| 2 | `0x04` | Weapon fire type code |
| 5 | `0x20` | Shield absorb type code |
| 6 | `0x40` | Combat phases enabled (shields, hull, fighters) |
| 7 | `0x80` | Hull damage type code |
| 8 | `0x100` | Fighter engagement type code |

## Notification Handlers

Each combat phase sends notifications to two observer channels (sides 1 and 2) via `FUN_00539fd0(this, side)`:

| Handler | Calls | Purpose |
|---------|-------|---------|
| FUN_0054a1d0 | `FUN_0054a1d0` → vtable+500 | Weapon damage applied |
| FUN_0054a260 | `FUN_0054a260` → vtable+0x1c8 | Shield absorption applied |
| FUN_0054a380 | `FUN_0054a380` → vtable+0x1d0 | Hull damage applied |
| FUN_0054a410 | `FUN_0054a410` → vtable+0x1d4 | Fighter damage applied |

## Difficulty Modifier

Line 21 of FUN_0054a1d0: `FUN_004fd600(param_1, *(uint *)((int)this + 0x24) >> 4 & 3)`

Extracts **2 bits from offset +0x24 bits 4-5** = difficulty level (0-3). This modifies combat outcomes per difficulty setting.

## Combat Result (FUN_005445d0)

1. Gathers all 5 stat types (weapons, shields, specials, fighters, 5th unknown)
2. Checks "alive" flag at `offset +0xac, bit 0` for each unit
3. Fighter exception: `+0x50 & 0x08` allows dead fighters to count as combat-ready
4. Iterates remaining fleet units to count survivors
5. Family ID `0x73`: special entity check (Death Star?)
6. Calls `FUN_00543d90` per side to determine winner
7. Alternative path via `FUN_00534640` for special entities

## Decompiled Files

- `FUN_00549910.c` — orchestrator (not saved separately, in FUN_005457f0)
- `FUN_005442f0_combat_init.c` — initialization
- `FUN_00544da0_fleet_eval.c` — fleet evaluation (96 lines)
- `FUN_00544030_weapon_damage.c` — weapon phase
- `FUN_00544130_shield_absorb.c` — shield phase (83 lines)
- `FUN_005443f0_hull_damage_apply.c` — hull phase
- `FUN_005444e0_fighter_engage.c` — fighter phase
- `FUN_005445d0_combat_result.c` — result determination (175 lines)
- `FUN_00544a20_post_combat.c` — post-combat cleanup (86 lines)
- `FUN_00543b60.c` through `FUN_00543d20.c` — per-side resolvers
- `FUN_005434b0.c` through `FUN_00543690.c` — stat getters
- `FUN_0054a1d0.c` through `FUN_0054a410.c` — notification handlers
