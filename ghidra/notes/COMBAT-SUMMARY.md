---
title: "Combat Notes Map and the Object-Move Dispatcher"
description: "Where each kind of combat is resolved, the object-move dispatcher FUN_00514a60 and its hooks, and the DatId family table"
category: "ghidra"
created: 2026-03-13
updated: 2026-10-14
---

# Combat Notes Map and the Object-Move Dispatcher

## Where combat is resolved

| Kind | Resolver | Note |
|---|---|---|
| Space battle | The battle session `FUN_0040a050`: the tactical engine, windowed or headless for Simulate and computer battles | [simulate-runs-tactical-engine.md](simulate-runs-tactical-engine.md), the `tactical-*.md` notes |
| Death Star in battle | Tactical Death Star object and trench run | [tactical-death-star-path.md](tactical-death-star-path.md) |
| Bombardment, Destroy System | Orders `0x220`-`0x223`, gate `FUN_00582440`, run `FUN_0058d800` | [bombardment.md](bombardment.md), [death-star-strategic.md](death-star-strategic.md) |
| Regiments lost to a blockade | Withdraw percent | [blockade-troop-withdrawal.md](blockade-troop-withdrawal.md) |
| Ground units and their art | Regiment and special-force classes | [ground-unit-assets.md](ground-unit-assets.md) |
| GNPRTB parameters | Parameter table | [combat-formulas.md](combat-formulas.md) |

## The object-move dispatcher: `FUN_00514a60`

`FUN_00514a60(obj, from, to, ctx)` runs on every change of container
(event `0x320`; only caller `FUN_0057ba50`, slot 8). It runs these hooks in
order and ANDs their results:

```
FUN_00514a60(obj, from, to, ctx)
  ├── FUN_005457f0  MissionMgr hook (thunk_FUN_00506e40): story-check chain
  │                 FUN_00549910 when obj is a story character      → space-combat.md
  ├── FUN_00560d50  character manager hook (thunk_FUN_00506e80): clears
  │                 CommandKind +0x96; Emperor-at-Coruscant flag     → ground-combat.md
  ├── FUN_00529d20  producing manager's hook (key +0x38, FUN_00505750),
  │                 when obj's +0x50 bit 2 (Completed) is clear: if obj
  │                 is the manager's product (FUN_0040d760), FUN_0052ac40
  │                 → FUN_00529820 (hyp: product destination update)
  ├── FUN_00556430  travel start: transit time from the two systems'
  │                 coordinates                                       → build-delivery.md
  └── from/to container slots +0xc4, +0xc8, +0xcc by obj +0x50 bits
      0x40, 0x01, 0x200; then FUN_00518530 on both containers
```

None of these hooks applies damage. The transit chain under
`FUN_00556430` (`FUN_00555d30` → `FUN_00555b30` → `FUN_0055d8c0`,
coordinates from `FUN_00509620`, `isqrt` in `FUN_0055d860`, divisor
`DAT_006bb6e8` = GNPRTB `0x1400`) computes travel ticks
`max(1, isqrt(d²) / GNPRTB 5120 * speed / 100)` (`build-delivery.md`,
"Travel").

## DatId family byte (`id >> 24`)

From the DAT `family_id` fields and the executable's range checks:

| Family | Kind | DAT |
|---|---|---|
| `0x08`-`0x0f` | Fleets | (executable use) |
| `0x10` | Regiments | `TROOPSD` |
| `0x14` | Capital ships | `CAPSHPSD` |
| `0x18` | Death Star | `CAPSHPSD` record 136 |
| `0x1c` | Fighter squadrons | `FIGHTSD` |
| `0x22`-`0x2a` | Facilities | |
| `0x30`-`0x35` | Major characters (`0x34` Palpatine, `0x35` Vader) | `MJCHARSD` |
| `0x38` | Minor characters | `MNCHARSD` |
| `0x3c` | Special forces | |
| `0x41`-`0x73` | Missions (`0x71` Dagobah, `0x73` Pickup) | |
| `0x80` | Sectors | |
| `0x90`, `0x92` | Systems | |

Range checks in code use the class blocks: `[0x10,0x14)` regiments,
`[0x14,0x1c)` capital ships, `[0x18,0x1c)` Death Star class, `[0x30,0x3c)`
characters, `[0x3c,0x40)` special forces.

## Decompiled files

`FUN_00514a60.c`, `FUN_005457f0.c`, `FUN_00560d50.c`, `FUN_00529d20.c`,
`FUN_0052ac40.c`, `FUN_00556430.c`,
`FUN_00555d30.c`, `FUN_00555b30.c`, `FUN_0055d8c0.c`, `FUN_0055d860.c`,
`FUN_00509620.c`.
