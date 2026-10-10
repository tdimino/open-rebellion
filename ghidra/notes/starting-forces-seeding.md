---
title: "Starting forces: the new-game unit seeding and its maintenance budget"
description: "FUN_0051aa50's seven steps, and how FUN_0051b1f0 spends each side's spare maintenance on CMUNALTB/CMUNEMTB bundles"
category: "ghidra"
created: 2026-10-09
updated: 2026-10-09
tags: [seeding, maintenance, difficulty, cmun, sdprtb]
---

# Starting forces

Companion to `top-bar-resource-counters.md` (what maintenance is). The port's
side is `crates/rebellion-data/src/seeds.rs`.

## The steps (`FUN_0051aa50`)

| Step | Function | Table (registry id → file, RCDATA name) | Where |
|---|---|---|---|
| 1 | `FUN_0051ab20` | — | every held system: low-support garrison |
| 2 | `FUN_0051acb0` | `0x50` CMUNYVTB (`0x758`) | Yavin, `0x92000121` |
| 3 | `FUN_0051ad90` | `0x51` CMUNHQTB (`0x759`) | the Rebel HQ |
| 4 | `FUN_0051ae20` | `0x52` CMUNCRTB (`0x75a`) | Coruscant |
| 5 | `FUN_0051af00` | `0x53` CMUNAFTB (`0x75b`) | Alliance fleets |
| 6 | `FUN_0051b080` | `0x54` CMUNEFTB (`0x75c`) | Empire fleet |
| 7 | `FUN_0051b1b0`, `FUN_0051b1d0` | `3` CMUNALTB (`0x70b`), `4` CMUNEMTB (`0x70c`) | `FUN_0051b1f0` for side 1, then side 2 |

- File names: REBEXE RCDATA `0x70b`/`0x70c`/`0x758..0x75e` (read with
  `pefile`); ids from `FUN_0058b420`. `0x55`/`0x56` are FACLHQTB/FACLCRTB.
- Steps 2–6 place every entry of the table whose key lies in a parameter
  range (`FUN_0051be90`; ranges from `FUN_0055d520..FUN_0055d5e0`, GNPRTB
  `0x140a..0x1413` bound in `FUN_0055cb60`).
- CMUNALTB and CMUNEMTB are read only by step 7 (`FUN_0055d730`,
  `FUN_0055d780`, whose sole caller is `FUN_0051b1f0`). Nothing places them
  at the capitals.

## Step 1: low-support garrisons (`FUN_0051ab20`)

For each system (`0x90..0x92`) passing `FUN_005092f0` and held by side 1 or
2: `FUN_0055a050(support)` regiments of `0x10000002` (Alliance) or
`0x10000008` (Empire), the support being the holder's (`FUN_00507270`).
They exist before step 7 measures the budget.

## Step 7: the budget pass (`FUN_0051b1f0`)

```
surplus = side+0x58 − side+0x5c                 // capacity − allocated load
budget  = surplus × SDPRTB[0x1430 + size − 1][side] / 100
loop:
    roll  = rand(0..=99) + 1                     // FUN_0053e2c0(1, 100)
    group = last group with key ≤ roll           // FUN_00595090
    cost  = group+0x24 × Σ class maintenance     // FUN_0051be20 → FUN_0053b870
    budget −= cost
    if budget < 0: stop; the group is not placed
    system = r-th side system, r = rand(0..=n−1) // systems 0x90..0x97 whose side bits match
    place the group there
```

- `FUN_0055d670` (side 1) / `FUN_0055d6d0` (side 2) take the galaxy size
  1..3 (the seeder's `+0x5c`) and pick SDPRTB `0x1430`/`0x1431`/`0x1432`
  (5168–5170): `DAT_006bb664`/`6f4`/`69c` for side 1, `DAT_006bb600`/
  `610`/`654` for side 2 (`FUN_0055cb60`), each holding the value for the
  game's difficulty column. `FUN_0053e190` → `FUN_0053e170` →
  `FUN_0053e150`: `surplus × pct / DAT_00661a88` (100), truncated.
- `+0x5c` is the allocated half of the side's capacity pair, which the load
  spread fills to `min(load, capacity)` (`top-bar-resource-counters.md`,
  "Load allocation"). So the budget is `max(capacity − load, 0) × pct / 100`:
  a side never seeds past its capacity. hyp: the spread has run for every
  object created in steps 1–6 when step 7 reads it (object creation calls
  `FUN_0052fff0`).
- A group's key (in-memory `+0x20`) is the DAT `entry_bis`: 1, 9, 20, 24, 29,
  39, 61, 69, 84, 89, 94, 98 in CMUNEMTB; 1, 9, 13, 27, 36, 40, 54, 58, 89,
  90, 93, 97 in CMUNALTB. A group's chance is the gap to the next key (the
  last runs to 100). `+0x24` is 1 in every group (hyp: the DAT's next field).
- Every item is costed by its class's maintenance (`FUN_0051cab0` class
  lookup, `FUN_0053b870` = class `+0x4c`): ships, fighters, regiments and
  special forces (`0x3c`) alike.
- Placement: a group holding a capital ship (`0x14..0x1b`) joins the side's
  first fleet at the system that `FUN_005131b0` accepts, else a new fleet
  (`FUN_004f7d50`, `0x8000004`); hyp: `FUN_005131b0` is untraced. Then
  `FUN_0051bf30` places the items.

## Difficulty

SDPRTB 5168–5170 per column (Alliance / Empire value):

| Size | Easy (either player) | Medium, Alliance player | Hard, Alliance player | Medium, Empire player | Hard, Empire player |
|---|---|---|---|---|---|
| Standard (5168) | 33 / 33 | 25 / 38 | 25 / 38 | 38 / 25 | 38 / 25 |
| Large (5169) | 25 / 25 | 20 / 30 | 20 / 30 | 30 / 20 | 30 / 20 |
| Huge (5170) | 20 / 20 | 15 / 25 | 15 / 25 | 25 / 15 | 25 / 15 |

At Easy every seeding row in SDPRTB is the same for both player sides
(5168–5170, 7680 strong support 10/10), so Easy seeds identically whichever
side the player takes. The computer's side gets the larger share at Medium
and Hard.

## What sets capacity before step 7

- Control (`FUN_00519d00`): N core systems; Alliance strong and weak
  `N × SDPRTB 7680/7681[Alliance] / 100`, Empire likewise less one strong
  slot for Coruscant (`0x90000109`); Yavin (`0x92000121`) and a random rim
  system (the HQ) are set aside; the other core systems are dealt one at a
  time at random (`FUN_0053e290`) to Alliance strong, Alliance weak, Empire
  strong, Empire weak, then neutral. The percentages come from
  `FUN_005597e0` (GNPRTB/SDPRTB `0x1e00`/`0x1e01` bound in `FUN_00558bb0`).
- Facilities (`FUN_00566de0`, a system method): one per energy slot
  (`+0x5c`). Each slot first rolls a mine with chance
  `(raw − mines) × DAT_006bb4bc` (`FUN_00559850`), else places one roll of
  SYFCCRTB (core, table 1) or SYFCRMTB (rim, `FUN_00559a60`, table 2); a
  failed roll ends the loop.
- So a side's capacity is whatever its few held systems' mines and
  refineries give, and its fixed seeds (Coruscant's yards and CMUNCRTB, the
  HQ's FACLHQTB) and rolled yards weigh on it before step 7 measures the
  budget. Step 7 adds nothing to a side already at or over capacity; it
  never takes one below. hyp: a side can therefore start overdrawn in the
  original too, when it holds few systems (Easy, or Hard as the Empire:
  3 systems) and its rolls favour yards over mines.

## Port (before this note)

- `seed_maintenance_budget_units` multiplied the percentage by the side's
  maintenance in use, not its spare capacity; rolled groups uniformly;
  costed regiments and special forces at 1 and skipped what it could not
  afford instead of stopping; and filtered systems by population.
- `apply_army_seed` placed every CMUNEMTB group at Coruscant and every
  CMUNALTB group at Yavin and the Rebel HQ: no such step exists.
- `seed_low_support_garrisons` ran after the budget pass instead of first.

## Port (now)

`seed_maintenance_budget_units` follows step 7 as above, measuring
capacity and load with `rebellion_core::resources` (the top strip's own
functions); `seed_low_support_garrisons` runs first; facility sides are
assigned before the budget; troop, defense and buildable classes load
before seeding (`rebellion-data/src/lib.rs`), so the budget prices
regiments, special forces and facilities. `apply_army_seed` is gone. Seed
42, Easy: Alliance 550 − 279 = +271, Empire 350 − 331 = +19, on day 0 in
the native build (`.artifacts/native-checks/2026-10-09-starting-forces/`).
