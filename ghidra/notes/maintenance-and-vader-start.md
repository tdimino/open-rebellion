---
title: "Maintenance computation and Vader's starting location"
description: "Original-game trace: how maintenance capacity/load work and where Vader starts"
category: "ghidra"
created: 2026-10-10
updated: 2026-10-14
tags: [maintenance, seeding, characters, vader]
---

# Maintenance computation and Vader's starting location

Two traces against the Ghidra decompilation: (a) how the original computes
maintenance capacity and load, and (b) where Darth Vader is placed at game
start.

## 1. Maintenance

### 1a. Capacity

The original recomputes maintenance totals in `FUN_0052fff0`
(`FUN_0052fff0.c`). Capacity is `min(Σ mine_cap, Σ refinery_cap)` via
`FUN_005323c0`. Each mine and refinery holds a capacity pair `(50, 0)`
initialised by `FUN_0052d540` (mine) and `FUN_0052d100` (refinery). The
minimum ensures unmatched mines or refineries do not inflate capacity.

Our code (`crates/rebellion-core/src/resources.rs:19–31`,
`maintenance_capacity`): `FACILITY_MAINTENANCE_CAPACITY * mines.min(refineries)`
where `FACILITY_MAINTENANCE_CAPACITY = 50`. **Matches the original.**

### 1b. Load

`FUN_0052fff0` walks every game object in families `0x10..0x3f` (troops
`0x10..0x13`, capital ships `0x14..0x1b`, fighters `0x1c..0x23`, facilities
`0x24..0x2f`, characters `0x30..0x3b`, special forces `0x3c..0x3f`). For each
with `+0x50` bit `0x40` (exists) and `+0x24 & 0xc0` matching the side, it
sums the object's class `+0x4c` (maintenance cost) via `FUN_004f2990` →
`FUN_0053b870`.

DAT verification:

- PROFACSD: mines and refineries both carry `maintenance_cost = 0` — never
  counted in load.
- MJCHARSD / MNCHARSD: all characters carry `maintenance_cost = 0`.
- FIGHTSD entry 5 (TIE Fighter): `maintenance_cost = 3`.
- CAPSHPSD entry `0x85` (Empire fleet's starting capital ship):
  `maintenance_cost = 71`.
- TROOPSD entry 6 (Empire stormtrooper): `maintenance_cost = 6`.
- MANFACSD: yards carry `maintenance_cost = 10–20`.
- DEFFACSD: defenses carry `maintenance_cost = 4–20`.

Our code (`crates/rebellion-core/src/resources.rs:43–125`,
`maintenance_used`): sums ships, fleet fighters, system fighters, troops,
special forces, defense facilities, manufacturing facilities, production
facilities, and queued manufacturing items. **The scope and per-class costs
match the original.**

One difference: our code counts queued manufacturing items
(`resources.rs:109–123`). The original's `FUN_0052fff0` walks objects with
the `+0x50` exists bit. hyp: queued units ARE game objects with the exists
bit set, so the original counts them too (manual p. 84: maintenance deducted
at the order). This makes no difference at game start (no queue).

### 1c. Display

The top-bar number is `side +0x58 − side +0x74` (`FUN_00422620`, message
`0x14e`; `top-bar-resource-counters.md`). Our
`maintenance_surplus` (`resources.rs:130–136`) computes the same subtraction.
**Matches.**

### 1d. The seed-42 deficit

Seed 42, Easy, Empire reads 350 available, 352 used = −2.

CMUNCRTB places 10 TIE squadrons (`0x1c000005`, maint 3 each = 30) and 6
stormtroopers (`0x10000006`, maint 6 each = 36) at Coruscant as fixed seeds
(seeding step 4, `FUN_0051ae20`). CMUNEFTB places 1 capital ship
(`0x14000085`, maint 71), 6 TIE squadrons (maint 18), and 3 stormtroopers
(maint 18) aboard the Empire fleet (step 6, `FUN_0051b080`). These fixed
seeds total 173 maintenance and are placed before the budget pass (step 7,
`FUN_0051b1f0`).

The budget pass computes `surplus = capacity − load` and spends
`surplus × SDPRTB_pct / 100` on random CMUNEMTB bundles. A side already at
or over capacity gets zero budget. **The budget cannot refuse the fixed
seeds.**

With 7 mines and 7 refineries (350 capacity), the fixed 173 plus
low-support garrisons and facilities can push load past 350. The two extra
maintenance points come from budget rolls that land one group too many, or
from garrison troops at low-support systems.

hyp: the original can start overdrawn too. The budget measures spare capacity
AFTER steps 1–6 have placed their objects and each creation has called
`FUN_0052fff0` to recompute (`starting-forces-seeding.md:67–69`). A side
with few systems and unlucky facility rolls can have less capacity than its
fixed seeds demand.

**The user's observation of "a large surplus" in the original at day 3 may
reflect different random facility rolls (more mines/refineries = higher
capacity) or different seed/difficulty conditions. No formula difference has
been found.**

## 2. Vader's starting location

### 2a. Character creation (stage 0x14)

The seeding driver `FUN_005136d0` (`FUN_005136d0.c`) runs stages 2–0x19.
Stage 0xc (`FUN_0051a280` → `FUN_0051bc70`) creates the systems, each keyed
`0x80000000 | record +0x40` to its sector (`galaxy-limits.md`); it creates no
characters. Characters come later:

- **Stage 0x14** (`FUN_0051b6d0` → `FUN_0055f730`) creates every character
  class of families `0x30` and `0x38` (`FUN_0051cad0`) through
  `FUN_00562520`, which creates the object (`FUN_004f7d50`) with its side
  from the class `+0x40`/`+0x44` flags: Alliance only 1, Empire only 2;
  a class with both flags or neither is not created.
- **Stage 0x15** (`FUN_0051b710` → `FUN_0055f7b0`) places the named
  characters (`FUN_0055fe70` moves an object into a container and sets its
  created and completed bits).

### 2b. Where each named character starts (`FUN_0055f7b0`)

| Character | Container |
|---|---|
| Palpatine `0x34000280` | Coruscant `0x90000109` |
| Vader `0x35000281`, minor `0x38000380`-`0x38000383`, `0x3800038c` | `FUN_00562680`: a random container of the character's side |
| Mon Mothma `0x30000240` | the Alliance HQ's container (`FUN_00506110()` `+0x1c`) |
| Luke `0x32000242`, Leia `0x31000241`, Han `0x33000243`, minor `0x38000341`, `0x38000343`, `0x38000344` | Yavin `0x92000121` |

Each group is followed by `FUN_0055d7d0` further characters placed through
`FUN_00562610`; hyp: these are the starting recruits of each side.

`FUN_00562710` picks the random container: it counts the active (`+0x50`
bit 0) systems (families `0x90`-`0x97`) and fleets (families `0x08`-`0x0f`)
of the side and rolls one index uniformly over both counts (`FUN_0053e290`).
An index below the system count returns that system. Otherwise it takes that
fleet and rolls again over the fleet's mode-1 children (`FUN_00502e30`),
returning the chosen child; hyp: the children are its capital ships. Stage
0x12 (`FUN_0051aa50`) has already created the fleets, so Vader starts at an
Imperial system or aboard a ship of an Imperial fleet; a ship of Fleet 1 at
Coruscant is one outcome.

### 2c. Difference from our code

Our `place_named_characters` (`seeds.rs:2044–2072`) places Vader at
Coruscant the planet through the `CORUSCANT_CHARACTERS` list. The original
places Palpatine there and gives Vader and the five Imperial minor
characters a random container of their side, fleets included.

## Summary of differences

| Claim | Our code | Original | Confidence |
|---|---|---|---|
| Maintenance capacity formula | `min(mines, refs) × 50` (`resources.rs:19`) | `min(Σ mine_cap, Σ ref_cap)` each 50 (`FUN_005323c0`) | confirmed |
| Maintenance load formula | Σ class maint over units (`resources.rs:43`) | Σ class `+0x4c` families `0x10..0x3f` (`FUN_0052fff0`) | confirmed |
| Characters' maintenance cost | 0 (DAT verified) | 0 (class `+0x4c`, DAT verified) | confirmed |
| Mines/refineries' maintenance cost | 0 (DAT verified) | 0 (class `+0x4c`, DAT verified) | confirmed |
| Queued items in load | counted (`resources.rs:109`) | hyp: counted if they are game objects with exists bit | medium |
| Seed-42 deficit | expected: fixed seeds can exceed capacity | hyp: original can also start overdrawn | medium |
| Vader's starting container | Coruscant planet (`seeds.rs:2068`) | a random Imperial system, or a ship of a random Imperial fleet (`FUN_0055f7b0` → `FUN_00562680` → `FUN_00562710`) | confirmed |
