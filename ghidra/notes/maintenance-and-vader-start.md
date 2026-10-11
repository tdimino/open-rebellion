---
title: "Maintenance computation and Vader's starting location"
description: "Original-game trace: how maintenance capacity/load work and where Vader starts"
category: "ghidra"
created: 2026-10-10
updated: 2026-10-10
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

### 2a. Character creation (stage 0xc)

The seeding driver `FUN_005136d0` (`FUN_005136d0.c`) runs stages 2–0x19.
Stage 0xc calls `FUN_0051a280` (`FUN_0051a280.c`), which loops 7 linked
lists at offsets `+0x100..+0x140` of the seeder object. For the `+0x140`
list it checks DatIds against Yavin (`0x92000121`) and Coruscant
(`0x90000109`) to decide the side. For each character it calls
`FUN_0051bc70` (`FUN_0051bc70.c`).

`FUN_0051bc70` looks up the character's class via `FUN_0051cab0`, extracts
the home system from class `+0x40`, and creates the character as a child of
that system via `FUN_004f7d50`. **Characters are placed at their home
system, not aboard fleets.**

Vader (`0x35000281`) gets Coruscant from his class `+0x40`.

Our code (`crates/rebellion-data/src/seeds.rs:2044–2072`,
`place_named_characters`): places Vader at Coruscant (the planet) via the
`CORUSCANT_CHARACTERS` list at line 2059. **Matches stage 0xc.**

### 2b. The fleet question

The user observes Vader aboard Imperial Fleet 1 at Coruscant in the
original's day-0 state. Stage 0xc places him at the planet, not the fleet.
Starting forces (stage 0x12, `FUN_0051aa50`) creates fleets. Fleet naming
(stage 0x17, `FUN_0051b7f0`) numbers them.

Five seeding stages are **undecompiled** and have no `.c` files in
`ghidra/notes/`:

| Stage | Function | When (relative to stage 0xc characters) |
|---|---|---|
| 0xd | `FUN_0051a530` | right after characters, before forces |
| 0xe | `FUN_0051a550` | right after characters, before forces |
| 0x14 | `FUN_0051b6d0` | after forces (0x12), before fleet naming (0x17) |
| 0x15 | `FUN_0051b710` | after forces, before fleet naming |
| 0x18 | `FUN_0051b940` | after fleet naming |

hyp: one of stages 0x14, 0x15, or 0x18 boards specific characters (Vader,
Palpatine, Mon Mothma, and possibly others) onto the fleet at their home
system. Stages 0xd and 0xe run before fleets exist, so they are less likely
candidates for fleet boarding, though they could prepare a boarding list.

### 2c. Difference from our code

Our `place_named_characters` (`seeds.rs:2044–2072`) places Vader at
Coruscant the planet. The original does the same in stage 0xc. But the
original likely boards him onto Imperial Fleet 1 in one of the undecompiled
stages.

**The fix is to add a post-fleet-creation step that boards Vader (and
likely Palpatine) onto the Empire's first fleet at Coruscant, and Mon
Mothma onto the Alliance's first fleet at the Rebel HQ.** The exact
original mechanism is untraced pending decompilation of `FUN_0051b6d0`,
`FUN_0051b710`, or `FUN_0051b940`.

## Summary of differences

| Claim | Our code | Original | Confidence |
|---|---|---|---|
| Maintenance capacity formula | `min(mines, refs) × 50` (`resources.rs:19`) | `min(Σ mine_cap, Σ ref_cap)` each 50 (`FUN_005323c0`) | confirmed |
| Maintenance load formula | Σ class maint over units (`resources.rs:43`) | Σ class `+0x4c` families `0x10..0x3f` (`FUN_0052fff0`) | confirmed |
| Characters' maintenance cost | 0 (DAT verified) | 0 (class `+0x4c`, DAT verified) | confirmed |
| Mines/refineries' maintenance cost | 0 (DAT verified) | 0 (class `+0x4c`, DAT verified) | confirmed |
| Queued items in load | counted (`resources.rs:109`) | hyp: counted if they are game objects with exists bit | medium |
| Seed-42 deficit | expected: fixed seeds can exceed capacity | hyp: original can also start overdrawn | medium |
| Vader at Coruscant planet (stage 0xc) | `seeds.rs:2068` | `FUN_0051bc70` → class `+0x40` home system | confirmed |
| Vader aboard Imperial Fleet 1 | not implemented | hyp: one of 0x14/0x15/0x18 boards him | high (observed in game, mechanism untraced) |
