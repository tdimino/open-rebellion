---
title: "AI Order Emission: Every Order the Computer Player Sends"
description: "The computer player's orders by source: 13 of its 14 task modules, the fleet tasks and posture engine under modules 9 and 8, the character-command tasks of module 12, and the mission planners; each order's trigger, side gate and the port's matching path or its absence"
category: "ghidra"
updated: 2026-10-16
tags: [ai, orders, modules, posture, production, fleets, characters, missions]
---

# AI order emission

Answers task-graph node `or-q-ai-order-emission-40h`. Every claim below was
read in the `.c` files cited. The Feature Wiki map pages
`executable/map-04-475890`, `map-06-4ae150`, `map-07-4bca20` and
`map-08-4cfed0` gave the starting addresses.

## How the orders were found

Every order is built by `FUN_004f5cd0(id)`. Its 114 call sites were listed
with the id each pushes and walked up to the vtable slot that runs them. All
but these belong to the computer player:

- the player's Agent modules `FUN_004c70e0`, `FUN_004c7220`, `FUN_004c7a90`,
  `FUN_004c7bd0` (`manage-automation.md`);
- session code: `FUN_00487c50` (order dialogs), `FUN_00487eb0` (game speed,
  `0x280`), `FUN_0048aa00` (`0x205`), the mission dialog `FUN_00492610` and
  `FUN_004927c0`, the fleet window `FUN_0049e380`, `FUN_0049e960`,
  `FUN_0049ea40`, and `FUN_0051d990`, `FUN_00536f00`;
- `FUN_00402720`, the computer player's character-record constructor, which
  builds a Mission order only to ask `FUN_004f52c0` which mission kinds the
  character may take, sets record `+0x34` bits from the answer, and deletes
  the order.

The names are STRATEGY's `RT_RCDATA` records (word 5, TEXTSTRA):

| Id | Name | Id | Name |
|---|---|---|---|
| `0x200` | Scrap | `0x222` | General Bombardment |
| `0x201` | Move | `0x223` | Destroy System |
| `0x203` | Rename | `0x234` | Planetary Assault |
| `0x210`-`0x212` | Build (facility, ship, troops) | `0x240` | Mission |
| `0x214` | Destination | `0x242` | Retire |
| `0x220` | Target Military Facilities | `0x250` | Abort |
| `0x260`-`0x263` | Command: None, Commander, Admiral, General | `0x270` | Create Fleet |

## The task modules

The computer player (`FUN_004857a0`, vtable `0x0065abf0`) adds all 14 task
kinds to its list `+0x64` at start-up (`FUN_004be930`, factory
`FUN_004bea20`), for either side. Each module's slot 5 runs only while its
state `+0x1c` is 1. Shared mechanics:

- **Budget.** A module's share is its weight `+0x24` × side context `+0x184`
  / 100, times a percentage per phase, minus what that context already
  counts as committed; the result is zeroed when it exceeds
  `+0x184 - +0x188` ("Production", below).
- **Goal token.** A builder clears its bit in context `+8` and builds only
  while context `+0xc` equals it; `FUN_004199b0` then rotates `+0xc` to the
  next pending bit.
- **Class choice.** `FUN_0041b230(ctx, out, type, mask, key, dir)` →
  `FUN_0052e580(side, type)` ranks the side's classes of type `0x28`,
  `0x29` or `0x2a` that carry every bit of `mask` by `key`; key 0 picks at
  random.

| Kind | Execute | Weight | Phase | Orders | Trigger |
|---|---|---|---|---|---|
| 11 | `FUN_004c8b30` | 30 | `0x3f8` | Move, Create Fleet | `FUN_004c8e90` moves the damaged ships (`0x1000`) of a free fleet to a shipyard-role system (`ship-damage-repair.md`); `FUN_004c94c0` splits a flagged (`0x4000`) fleet's capital ships into a new fleet, else moves the fleet on when its system record lacks bit 0 or `0x40000`; `FUN_004c9cb0` moves capital ships to a sector system not yet listed in context `+0xd8` |
| | | | `0x3f9` | Build `0x211`, Scrap | Scrap capital ships when 90% of the share minus context `+0x24c` is negative (`FUN_004caad0`, `FUN_004cab20`), else build the class from `FUN_004ca5e0` (`FUN_004cba90`); one class filter is Empire only |
| | | | `0x3ec` | Scrap, Destination, Build `0x210` | goal bit 2, 10%: `FUN_004cb1d0`, `FUN_004cb7b0`, `FUN_004cbbb0`, `FUN_004cbcf0` |
| 10 | `FUN_004cc9c0` | 8 | `0x3f2` | Scrap, Destination, Build `0x211` | 80% minus `+0x254`: Scrap when negative (`FUN_004cd310`), else `FUN_004ce350`, `FUN_004ce490` |
| | | | `0x3ec` | Scrap, Destination, Build `0x210` | goal bit `0x10`, 20%: `FUN_004cda90`, `FUN_004ce070`, `FUN_004ce5b0`, `FUN_004ce6f0` |
| 9 | `FUN_004cf290` | – | – | Create Fleet, Move | the fleet tasks (below) |
| 8 | `FUN_004cf600` | – | – | Create Fleet, Move, assault and bombardment | the regional records and the posture engine (below) |
| 5 | `FUN_004cfed0` | 4 | `0x3f4` | Build `0x212`, Retire | 70% minus `+0x2c4`: trains the special-force class whose counter (`+0x2ec`-`+0x2f8`; classes by literal id in `FUN_00402e80`, `record-registry.md`) is lowest below max(`+0x2fc`, 8) at a system wanting them (`FUN_004d0480`, `FUN_004d01a0`, `FUN_004d15f0`); when that budget is negative, retires the first special force (`FUN_004d0610`) |
| | | | `0x3ec` | Scrap, Destination, Build `0x210` | goal bit 4, 30%: `FUN_004d0d30`, `FUN_004d1310`, `FUN_004d1710`, `FUN_004d1850` |
| 7 | `FUN_004d3100` | 17 | `0x3f1` | Move, Scrap | fighters to a system whose record wants them (`+0x11c`): Move when `FUN_004d3610` finds a source (`FUN_004d3a90`), else Scrap (`FUN_004d3db0`) |
| | | | `0x3f2` | Destination, Build `0x211`; Move, Scrap | 80% minus `+0x1e0` at a system wanting ships (`FUN_004d3ff0`, `FUN_004d5dd0`, `FUN_004d5f10`); when 90% minus `+0x1e0` is negative, `FUN_004d4800` moves and scraps |
| | | | `0x3ec` | Scrap, Destination, Build `0x210` | goal bit 8: `FUN_004d5510`, `FUN_004d5af0`, `FUN_004d6030`, `FUN_004d6170` |
| 3 | `FUN_004d6d30` | 9 | `0x3ed` | Scrap | defences: `FUN_004d7330` scraps families `0x22`-`0x24`; `FUN_004d8520` scraps when 90% minus `+0x1dc` is negative |
| | | | `0x3ee` | Scrap, Destination, Build `0x210` | resource sites (`FUN_004d7ac0`; Coruscant `0x90000109` handled apart in `FUN_004d76f0`): `FUN_004d7cf0`, `FUN_004d9b00`, `FUN_004d9e70`, `FUN_004d9fb0` |
| | | | `0x3ec` | Scrap, Destination, Build `0x210` | goal bit 1: `FUN_004d8fe0`, `FUN_004d95c0` |
| 1 | `FUN_004dabe0` | 4 | `0x3ec` | Scrap, Destination, Build `0x210` | goal bit `0x80`: a Refinery `0x2d000002` when context `+0x1d4 < +0x1d0`, else a Mine `0x2c000001` (`FUN_004db2f0`; `FUN_004db820`, `FUN_004dbe70`, `FUN_004dc650`, `FUN_004dc9f0`, `FUN_004dcb30`) |
| 2 | `FUN_004e2120` | 3 | `0x3ea`/`0x3eb` | Scrap, Destination, Build `0x210` | a Mine (`0x3ea`) or Refinery (`0x3eb`, when `+0x1d4 <= +0x1d0`): `FUN_004e2940`, `FUN_004e31b0`, `FUN_004e4300`, `FUN_004e4440` |
| | | | `0x3ec` | Scrap, Destination | goal bit `0x40`: `FUN_004e3a40`, `FUN_004e4020` |
| 6 | `FUN_004dd7a0` | 22 | `0x3ef` | Move | four regiment passes (`FUN_004ddb70` when context `+4` bit `0x20`, `FUN_004de130` when bit `0x10`, `FUN_004de6f0`, `FUN_004deba0`) |
| | | | `0x3f0` | Scrap, Destination, Build `0x212` | `FUN_004dfa80`, `FUN_004e0a30`, `FUN_004e0b70` |
| | | | `0x3ec` | Scrap, Destination, Build `0x210` | goal bit `0x20`: `FUN_004e0170`, `FUN_004e0750`, `FUN_004e0c90`, `FUN_004e0dd0` |
| 14 | `FUN_004d29b0` | – | – | Rename | up to 10 capital ships per pass get a name from the side's TEXTSTRA pools (`FUN_004d2b30`; `fleet-names.md`) |
| 13 | `FUN_004bf320` | – | `0x3fc` | Scrap, Move | Alliance only (side 1, `FUN_004bf3e0`); the Empire's `FUN_004bf480` finishes at once. Moves the Alliance HQ (below) |
| 12 | `FUN_004c84a0` | – | – | none itself | starts the character tasks (below) |
| 4 | `0x0065c778` slot 5 | – | – | none itself | runs the mission planners (`ai-mission-planning.md`) |

Each Build goes to a producer of its family (`production-destination.md`):
ship producers `0xa2`-`0xa3` take `0x211`, troop producers `0xa4`-`0xa5`
take `0x212`, facility producers `0xa0`-`0xa1` take `0x210`. A Destination
(`0x214`) precedes it everywhere except module 11's ships and module 5's
troops.

## Production

Step by step, with every field, query and phase: `ai-production.md`.

Offsets: `m+` the module, `c+` the side context (`FUN_004182a0`,
`FUN_00418500`; player `+0x84`), `sys+` a system record (`FUN_00433e40`),
`sec+` a sector record (`FUN_004321e0`).

### Budget base

Every unit and facility budget is one expression (`FUN_004cc220`,
`FUN_004caad0`, `FUN_004cec20`, `FUN_004d66a0`, `FUN_004d1d90`,
`FUN_004e19c0`, `FUN_004da4d0`):

    share  = m+0x24 * c+0x184 / 100
    budget = share * PCT / 100 - spent
    budget = 0 when budget > c+0x184 - c+0x188

- `c+0x184` is the side's maintenance capacity, side `+0x58` =
  min(50 × mines, 50 × refineries) (`FUN_0052ec90`, `FUN_005323c0`;
  `maintenance-and-vader-start.md`), written only by `FUN_00418970`
  (player slot `0x0065ac54`, thunk `0x00485e60`).
- `c+0x188` is the maintenance load, side `+0x70` (Σ class `+0x4c` over
  families `0x10`-`0x3f`, `FUN_0052fff0`), written only by `FUN_00418990`
  (slot `0x0065ac58`, thunk `0x00485e80`). Both run once in
  `FUN_004182a0` case 1.
- The weights (`FUN_004bea20`) are budget shares, not scheduling weights:
  11 = 30, 6 = 22, 7 = 17, 3 = 9, 10 = 8, 1 = 4, 5 = 4, 2 = 3 (sum 97).
- The census `FUN_00418500` case 5 fills the spent counters: flag
  `0x20000000` zeroes `c+0x1ac`-`+0x24b` and `FUN_0041b7e0` sums the
  sectors (`FUN_00432350` over system records); `0x40000000` zeroes
  `c+0x24c`-`+0x2c3` and `FUN_0041ba80` sums the own fleets (list `c+0x78`);
  `0x10000000` zeroes `c+0x2c4`-`+0x313` and `FUN_0041bc10` sums the
  characters (list `c+0x8c`). An AI class node's `+0x20` is the class's
  maintenance (`FUN_0053b870`), `+0x1c` its refined cost (`FUN_0053b860`).

| Field | Holds | Source |
|---|---|---|
| `c+0x1d0` | own mines (family `0x2c`) | `sys+0x50` (`FUN_0052d370`) → `sec+0xa8` |
| `c+0x1d4` | own refineries (`0x2d`) | `sys+0x54` (`FUN_0052cf40`) → `sec+0xac` |
| `c+0x1d8` | maintenance of regiments in systems | `sys+0xc0` → `sec+0xfc` |
| `c+0x1dc` | maintenance of defences (`0x22`-`0x25`) in systems | `sys+0xc4` → `sec+0x100` |
| `c+0x1e0` | maintenance of fighters in systems | `sys+0xc8` → `sec+0x104` |
| `c+0x24c` | maintenance of capital ships | fleet `+0x88` = Σ ship `+0x70` |
| `c+0x250` | maintenance of regiments aboard ships | fleet `+0x8c` = Σ ship `+0x74` |
| `c+0x254` | maintenance of fighters aboard ships | fleet `+0x90` = Σ ship `+0x78` |
| `c+0x2c4` | maintenance of special forces | character record `+0x64` (`FUN_004f2990`) |
| `sys+0xd0`/`+0xd4`/`+0xd8` | maintenance of shipyards / training facilities / construction yards | `FUN_00433e40` |

| Module | Unit phase | Unit budget | Facility budget (phase `0x3ec`) |
|---|---|---|---|
| 11 capital ships | `0x3f9` | 90% − `c+0x24c` | 10% − Σ`sys+0xd8` over `m+0x78` |
| 10 fighters for fleets | `0x3f2` | 80% − `c+0x254` | 20% − Σ`sys+0xd0` over `m+0x74` |
| 7 fighters for systems | `0x3f2` | 80% − `c+0x1e0`; scrap test 90% | 20% − Σ`sys+0xd0` over `m+0x6c` |
| 5 special forces | `0x3f4` | 70% − `c+0x2c4` | 30% − Σ`sys+0xd4` over `m+0x78` |
| 6 regiments | `0x3f0` | 80% − `c+0x250` − `c+0x1d8` | 20% − Σ`sys+0xd4` over `m+0x7c` |
| 3 defences | `0x3ee`/`0x3ed` | 90% − `c+0x1dc` | 10% − Σ`sys+0xd8` over `m+0x6c` |
| 1 yards, mines | `0x3ec` only | 100% − Σ`sys+0xd8` over `m+0x58` | the same |
| 2 deficit relief | `0x3ea`/`0x3eb` | only while `c+0x184 < c+0x188` | 100% − Σ`sys+0xd8` over `m+0x60` |

### Goal bits

Context `+8` holds the pending bits (start 0), `+0xc` the token (start
`0x80`), `+0x10` the scan seed (start `0x40`) (`FUN_004178f0`). At the end
of its `0x3ec` pass a module sets its bit when its facility budget is
positive, no usable site exists, and the best facility class
(`FUN_0041b230(c, out, 0x2a, mask, 0x4000, 2)`) costs no more than the
budget. `FUN_004199b0`: with no bit pending, token = seed and the seed
shifts right (0 → `0x80`); else the token shifts right (wrapping at
`0x80`) to the next pending bit and clears it. Only the token holder
builds, so one facility goal runs at a time.

| Bit | Module | Facility (mask) | Setter |
|---|---|---|---|
| `0x2` | 11 | shipyard `0x4000` | `FUN_004cbe10` |
| `0x10` | 10 | shipyard | `FUN_004ce810` |
| `0x8` | 7 | shipyard | `FUN_004d6290` |
| `0x4` | 5 | training facility `0x8000` | `FUN_004d1970` |
| `0x20` | 6 | training facility | `FUN_004e15b0` |
| `0x1` | 3 | construction yard `0x10000` | `FUN_004da0d0` |
| `0x80` | 1 | construction yard | `FUN_004dcc40` |
| `0x40` | 2 | construction yard | `FUN_004e4540` |

Key `0x4000` is class `+0x5c`, the processing rate. A facility order's
count is min(budget / maintenance, the producer's construction yards − 1
(`FUN_004cafa0`), the site's free slots `sys+0x114`/`+0x84`)
(`FUN_004cb710`, `FUN_004cdfd0`).

### Class lists

`FUN_0041b230` and `FUN_0041b270` sort the candidates (`FUN_0041be80`,
compare `FUN_0041c250`) and `FUN_004357b0` returns the last: direction 1
gives the largest key, direction 2 the smallest; ties break at random. The
result is the winner's maintenance. Keys by node:

- Capital ship (`FUN_00473930`): `0x200` attack score `+0x30`, `0x80000`
  score × 10 / maintenance, `0x400` `+0x34`, `0x800` bombardment, `0x80`
  fighter capacity, `0x100` troop capacity, `0x1` maintenance, `0x2`
  `+0x4c`, `0x4` refined cost, `0x10000` shield recharge × 100.
- Fighter (`FUN_00473640`): `0x400` `+0x34`, `0x8000` torpedoes × squadron
  size, `0x1` maintenance. Regiment (`FUN_00473490`): `0x1000` `+0x30`,
  `0x2000` `+0x34`, `0x1` maintenance. Facility `FUN_00473060`, defence
  `FUN_00473250`.

Capital-ship flags (`FUN_004739b0`): `0x80` families `0x18`-`0x1b` (Death
Star), `0x100000` every other capital ship, `0x400` gravity well, `0x800`
bombardment modifier, `0x100` fighter capacity, `0x200` troop capacity,
`0x1` a turbolaser or ion weapon and not `0x400`, `0x2` lasers only, no
troops and not `0x400`, `0x4` troop-capable with none of `0x403`, `0x8`
troop-capable with flag 1, `0x10`/`0x20`/`0x40` hull below 1100, below
2000, larger. Fighters carry `0x200000`; a regiment `0x1000` when
`+0x34 <= +0x30` and `0x2000` when `+0x30 <= +0x34`.

### Module 11: capital ships

| Step | Function | Action |
|---|---|---|
| 0 → 8 | `FUN_004ca300` | producer: the first system of `m+0x78` that is own (`sys+0x30 & 1`), has `sys+0x28` bits `0x200000` and `0x200`, none of `3`, and shipyards (`sys+0x58 > 0`); else the caches `c+0x1c`, `c+0x24`. Producer `FUN_00509670(sys, 0)`. None → `0xd` |
| 5 | `FUN_004ca5e0` | the request: class mask `m+0x60`, key `m+0x64`, blocker `m+0x68` |
| 9 | `FUN_004ca910` | budget `FUN_004cc220`; negative → Scrap; positive → `FUN_0041b270(c, 0x28, mask, key, 1)` and the first class whose maintenance fits; none → `m+0x28 |= blocker` and on to `0x3ec` |
| `0xf` | `FUN_004cba90` | Build `0x211` of one ship at the producer |
| `0xd`/10 | `FUN_004caad0`, `FUN_004cab20` | Scrap `0x200` of one capital ship when 90% − `c+0x24c` < 0 |

`FUN_004ca5e0` counts the regional records of `c+0xd8` whose `rec+0x64`
has bit 1, `0x80`, 4, 8, `0x10` or `0x800` with the blocker clear; the
first matching row wins:

| Trigger | Mask | Key (direction 1) | Blocker |
|---|---|---|---|
| any record with bit 4 | `0x100004` | `0x100` | `0x4000000` |
| bit 1 the most common | `0x100001` | `0x80000` | `0x1000000` |
| bit `0x80` | `0x100002` | `0x400` | `0x2000000` |
| bit `0x10` | `0x100800` | `0x800` | `0x10000000` |
| bit 8 | `0x100102` | `0x80` | `0x8000000` |
| bit `0x800` | `0x100400` | `0x10000` | `0x40000000` |
| none, and `c+0x260 < c+0x20c` | `0x100004` | `0x100` | `0x4000000` |
| side 2, more than one system in `m+0x78`, no own Death Star (`c+4 & 4` clear) | `0x80` | `0x200` | `0x20000000` |
| `c+0x268 < 1` | `0x100001` | `0x80000` | `0x1000000` |
| `c+0x26c < 1` | `0x100002` | `0x400` | `0x2000000` |
| `c+0x27c <= c+0x16c` | `0x100102` | `0x80` | `0x8000000` |

### Modules 10 and 7: fighters

| | 10 `FUN_004cc9c0`, for fleets | 7 `FUN_004d3100`, for systems |
|---|---|---|
| Producer | `FUN_004cccc0`, as module 11 over `m+0x74` | `FUN_004d3ff0` over `m+0x6c` |
| Target | `FUN_004cd040`: a record of `c+0xd8` with `rec+0x9c > 0`, the free hangar slots of the sector's own fleets minus the fighters at the record's system (`FUN_00476f90`); target system `rec+0x40`; else `FUN_0041a4c0(c, 0x100005, 0xc02, 0x17, 2)` | `FUN_004d4370`: a system with `sys+0xe0 < 0` (`FUN_00419b80` queries) |
| Class | `FUN_004cd230`: mask `0x200000`, key `0x8000` when `rand(10) > 5`, else `0x400`, direction 1 | `FUN_004d46e0`: mask `0x200000`; a plain system takes key 1 direction 2 (the cheapest), a system with facility flags `0x83e00000` key `0x400` direction 1 |
| Count | `rec+0x9c`, cut to budget / cost and the producer's yards (`FUN_00528040`) | `−sys+0xe0`, cut the same way |
| Orders | Destination `0x214` (`FUN_004ce350`), Build `0x211` (`FUN_004ce490`) | Destination (`FUN_004d5dd0`), Build (`FUN_004d5f10`) |
| Over budget | 80% − `c+0x254` < 0: Scrap the weakest fighter aboard (`FUN_004cd310`) | 90% − `c+0x1e0` < 0: `FUN_004d4800` moves and scraps |

`sys+0xe0` = fighters at the system (`sys+0xa8`) − target, and `sys+0x11c`
= `sys+0xac` − target; a positive `sys+0x11c` is a surplus that phase
`0x3f1` moves or scraps. The target is `c+0x388` for a plain system,
`c+0x38c` for one with `sys+0x28 & 0x3e00000`, `c+0x390` for
`sys+0x28 & 0x80000000`. The targets `c+0x364`-`+0x390` start at
3, 3, 2, 1, 1, 0, 0, 3, 9, 0, 6, 12 (`FUN_0041bd20`); `FUN_00418240`
raises one by 1 and `FUN_00418270` lowers one. Module 7 raises the fighter
targets when it finds no target and has budget to spare.

### Modules 5, 6, 3, 1 and 2

- **5, special forces** (`FUN_004cfed0`, `0x3f4`). `FUN_004d0480` cycles
  class codes `0x5a`-`0x5d` and wants one while its counter
  (`c+0x2ec`-`+0x2f8`) is below max(`c+0x2fc`, 8). Producer
  `FUN_004d01a0` (flag `0x400000`, `sys+0x5c > 0`, `FUN_00509670(sys, 2)`).
  Class `FUN_0041b230(c, out, 0x29, 0x1000000 << k, 1, 1)`, the dearest;
  one, cut to budget / cost. Build `0x212` (`FUN_004d15f0`), no
  Destination. When 70% − `c+0x2c4` < 0, `FUN_004d0610` retires a special
  force (Retire `0x242`, `FUN_0041b0d0`).
- **6, regiments** (`FUN_004dd7a0`, `0x3f0`). `FUN_004df440` cycles four
  selectors (`FUN_004e1060`, `FUN_004e0ef0`, `FUN_004e11a0`,
  `FUN_004e1430`, sector flags `0x800000`, `0x400000`, `0x40000000`,
  `0x40000`) whose shortage is `sys+0x9c` = target `sys+0xa0` − regiments
  present (targets `c+0x37c`/`+0x380`/`+0x384` = 0, 3, 9; lowered by
  `FUN_00418270` when unaffordable), and `FUN_004e12e0`, an enemy system
  with mask `0x1000`. Defenders take mask `0x2000`, key `0x2000`
  direction 1, or for a system without facilities key 1 direction 2. Count:
  the shortage cut to budget / cost (`FUN_004df9a0`). Destination
  `FUN_004e0a30`, Build `0x212` `FUN_004e0b70`; a surplus is scrapped
  (`FUN_004dfa80`).
- **3, defences** (`FUN_004d6d30`). `0x3ed` scraps families `0x22`-`0x24`
  (`FUN_004d7330`) and scraps when 90% − `c+0x1dc` < 0 (`FUN_004d8520`).
  `0x3ee` takes a yard system (`FUN_004d7ac0`: `sys+0x28 & 0x2000000`,
  `sys+0x60 > 0`, `FUN_00509670(sys, 1)`; Coruscant `0x90000109` in
  `FUN_004d76f0`) with free slots `sys+0x84`, and the class
  (`FUN_004d8460`) mask `0x40000` key `0x10000` on a system with
  `sys+0x28 & 0x40000000` (the Death Star shield), else mask `0x80000` key
  `0x40000` without `0x8000`, else mask `0x20000` key `0x10000`; one when
  it fits the budget. Destination `FUN_004d9e70`, Build `0x210`
  `FUN_004d9fb0`. It also builds a Refinery `0x2d000002` when
  `c+0x1d4 < c+0x1d0`, else a Mine `0x2c000001`, one at a time.
- **1, construction yards** (`FUN_004dabe0`, `0x3ec` with token `0x80`):
  n = budget / maintenance of the best `0x10000` class (`FUN_004dbd60`),
  cut to the site's free slots, then a Mine or Refinery as module 3.
- **2, deficit relief** (`FUN_004e2120`): a Mine (`0x3ea`) or a Refinery
  (`0x3eb`, when `c+0x1d4 <= c+0x1d0`), each opened by a relief Scrap
  (`FUN_004e2940`) taken only while `c+0x184 < c+0x188`; make-room Scrap
  `FUN_004e31b0`, Destination `FUN_004e4300`, Build `0x210` `FUN_004e4440`.

Order fields: `+0x20` side, team = producer, `+0x44` class, `+0x48`
quantity.

### Scheduler and executor

- `FUN_00485b40` (computer player vtable `0x0065abf0` slot 9) steps one
  armed module per call: `FUN_004be980` takes the armed module with the
  largest `m+0x34` (scan start `+0x14` rotating one module per pick,
  `FUN_004bede0.c:12-17`), else the next armed module in list order, which
  is ascending kind, 1 to 14 (the list is a tree keyed by the module's
  kind, `FUN_005f4f10`, `FUN_005f5560.c`; cursor `+0x10`, advanced before
  each test and kept across rounds; a priority pick leaves it unchanged).
  `m+0x34` is cleared before every step (`FUN_00485b40.c:33`) and raised by
  a step that returns an order in the middle of its pass (modules 4, 8, 9,
  12: `FUN_004d2490.c:38-39`, `FUN_004cf630.c:32-33`, `FUN_004cf2c0.c:27-28`,
  `FUN_004c84d0.c:35-36`; module 13 `FUN_004bf3e0.c:32-34`; production
  modules on a Destination or make-room Scrap, `ai-production.md`), so at a
  round boundary at most one module holds it: the one to resume. The order
  is validated (slot `+0x1c`, `FUN_0051ff30`) and issued (`FUN_0048aa90` →
  `FUN_0041cee0` → `FUN_004360f0`); a refusal calls the module's slot 6:
  `FUN_004dd810` for modules 1-13 sets done and clears `m+0x38`, `m+0x3c`
  and `m+0x34`, so the module restarts its cycle; module 14's
  `FUN_004d2b10` clears only `m+0x34`.
- Re-arm (slot 4) is `FUN_004cc990` for modules 1-13: state `+0x1c` becomes
  1 and done 0 unless the state is 1 or 2. Module 14's `FUN_004d2970` arms
  only while context `+0x264 > 0`. Each module's slot 5 does nothing (done,
  phase and step 0) unless its state is 1.
- A round ends when a module finishes (`m+0x20`, then its state `+0x1c` is
  cleared, `:53-58`) having returned an order, valid or refused, in that
  call (`:38`, `:59`), when a module finishes while `c+4 & 0x80000000` is
  set (`FUN_004184f0`), when nothing is armed (`:24-29`), or before any call
  once the counter `+0x420` has reached `+0x428` = 10 (`:21`,
  `FUN_00485a50.c:35`). The census then runs (state 4) and every module is
  re-armed (state 5) at the start of the side's next round. A Destination
  and its Build take two rounds.
- The counter `+0x420` counts the validated orders other than Rename
  `0x203` (`:45-48`). It is cleared once per day: phase 13
  (`FUN_0041e500`) calls `FUN_0041d050` → `FUN_00436530`, which calls slot
  `+0x54` of both players, the computer player's `FUN_00485de0` (vtable
  entry `0x0065ac44`), on the step whose in-day count reached the per-day
  count (scheduler `+0x24`, `FUN_0051dea0.c:30`); `FUN_00485de0` clears
  `+0x420` because `+0x424`, written only by constructors and the save
  codec, is 0. So each computer side issues at most 10 orders a day,
  renames excepted.
- `FUN_0041dff0` phase 8 → `FUN_0041cf20` → `FUN_004361a0` runs one step
  per call and switches side when a round ends; the phase advances only
  after both sides end a round, so each side gets one round per phase
  cycle (`FUN_004361a0.c:8-13`), the Alliance (session `+0x10`) before the
  Empire (`+0x14`; `side-numbering.md`). Phase 0 advances one sub-tick per
  cycle (`FUN_0041e390.c:26-31`). `FUN_0051dea0` counts the in-day steps
  0 to the per-day count, the larger of the sides' `+0xc4`, and advances
  the day on the step after the count is reached (`FUN_0051dea0.c:11-30`),
  so a day is the per-day count plus one steps: 601, 61, 13 or 5 for Very
  Slow, Slow, Medium and Fast. Each computer side runs that many rounds
  per game day.
- Every module sets its done flag when it returns an order (`FUN_004cf630`,
  `FUN_004cf2c0`, `FUN_004d2490`, `FUN_004c84d0`, `FUN_004bf3e0`,
  `ai-production.md`), so a round issues at most one order, renames
  excepted, and a side at most min(rounds, 10) a day. Orders issued in a
  round execute in phase 5 of the next step, before that step's round:
  phase 3 hands them to list `+0xa4`, and phase 5 serves the event and
  timer queues before each order (`blockade-bit.md`, "The case";
  `FUN_0051df30.c`). An order from a day's last round therefore runs after
  the next day's due timers.
- Besides `FUN_00433600` (below), `c+4 & 0x80000000` is set by the census
  (`FUN_00418500` cases 1, 2 and 4) and cleared at its end (case 5,
  `:202`), and set inside a round by a character claim (`FUN_004021b0`,
  from the planners' and module 12's selectors), a planner's creation
  (`FUN_0042f640`, module 4 states 1 and 3) or release (`FUN_0041b530`),
  a character task's release (`FUN_0041b600`), and a sector record write
  (`FUN_004321d0`: plan kinds `FUN_00431790` and `FUN_004317b0` from every
  controller slot 5, record links `FUN_004317d0` from `FUN_004768b0` and
  `FUN_00477190`).
- `FUN_00433600` marks a fleet record dirty (`+0x1c = 1`) and sets
  `c+4 & 0x80000000` (`FUN_004184f0`), which ends the round after the
  current module finishes. `FUN_0042ff70` calls it for a fleet id from the
  object-change handlers `FUN_00418a60`, `FUN_00418db0`, `FUN_00419260` and
  `FUN_00484570`; `FUN_004335e0` (from `FUN_0042ffe0`) also stamps the day
  at `+0x3c`. The census (`FUN_00418500` case 2) refreshes dirty fleet
  records through `FUN_00433620`, which sets `+0x38` bit 2 from object
  `+0x50 & 0x10`, the en-route flag (`FUN_00433620.c:44-46`).
- The Build orders `0x210`/`0x211`/`0x212` (`FUN_00538cc0`, `FUN_00538d90`,
  `FUN_00538e60`) share slot `+0x50` = `FUN_005385f0`, which refuses with
  `0xf3` when quantity × maintenance exceeds side `+0x58 − +0x5c`
  (`FUN_0052e970`), with ledger checks `FUN_00551a20`, `FUN_00552300`,
  `FUN_00552b10` (`0x19` outside their nine location categories).
- Both sides run the same modules and constants. The differences: the
  Death Star request (side 2), the Death Star shield flag
  (`sys+0x28 & 0x40000000`: side 2, class `0x25000004` available
  (`c+4 & 0x1000000`), an own Death Star, no shield there), the class
  side flags, and module 13 (Alliance only). No traced production
  function reads a difficulty or GNPRTB value.

### Module 13: moving the Alliance HQ

`FUN_004bf390` picks a mode 1-7 (`FUN_004bf490`) from side counters and steps
its controller (`FUN_004bf7c0`, `FUN_004e5440`). The move itself:

- `FUN_004bf9e0` needs the HQ `0x20000005` (`FUN_004f3330`) not en route
  (`+0x50` bit 4 clear). It takes a system from the side's records carrying
  record flag `0x80000000` and, unless the sector record's `+0x2c` has bit
  `0x40000`, waits until the time reaches `+0x54`.
- `FUN_004bfe30` scraps facilities at that system.
- `FUN_004c05e0` moves `0x20000005` there and sets `+0x54` to now + 300, so
  the HQ moves at most once every 300 ticks.

## Fleet tasks (module 9)

A fleet task (`FUN_00474070`, list `c+0x11c`) holds one of the side's
headquarters systems with a fleet as strong as its best warship class.

- **Count.** Controller slot 8 (`FUN_004e69d0`, the same in every mode) adds
  one task while the list is shorter than `c+0x224`, else removes the head
  task while longer, but only when the head holds no target (`FUN_0042edc0`).
  `c+0x224` counts the side's own systems tagged `sys+0x28` `0x80000000`
  (Coruscant `0x90000109`, or a system holding the owner's HQ;
  `FUN_00433e40`) and not blockaded (`sys+0x28` bit 2): 0 or 1 for the
  Empire, up to 2 for the Alliance.
- **Driver.** `FUN_004cf290` steps one task one state per call; a task keeps
  the driver until its pass ends in state 7 or 8, and the next pass takes
  the next task (`m+0x40`, `m+0x44`; no wrap within a call).
- **Task fields.** `+0x1c` state, `+0x24` the claimed ships, `+0x2c` its
  fleet, `+0x30` the target system, `+0x34` the need, `+0x38` revalidate
  before the next step (set by every order).

| State | Function | Next | Order |
|---|---|---|---|
| 0 | | 1, revalidate | |
| 1 | `FUN_004744b0`: keep the target while own, tagged and not blockaded; else a random tagged sector (`sec+0x24` `0x40000000`, key `rand(20)`) and in it a random own tagged unblockaded unclaimed system (key `rand(10)`); the target is claimed (`sys+0x30` `0x80000`) | 2, or 8 without a target | |
| 2 | `FUN_004749b0` | 3 | Create Fleet of the fleet's damaged ships, `ship+0x24` `0x801000` (not en route), in its system, when fleet `+0x38` has `0x3000` and none of `0xf0000802` |
| 3 | `FUN_00474b90`: need = fleet `+0xb4` − `c+0x164` (twice `c+0x164` when `c+4` bit 1, a Headquarters Only game) | 4 | |
| 4 | `FUN_00474c00`: nothing while the fleet is en route; need < 0: gather (`FUN_00475490`, then `FUN_00475890`); need > 0: peel (`FUN_00475c70`) | 5 always | Move of the gathered ships onto the task fleet, or Create Fleet at the target without one; Create Fleet of the peeled ship |
| 5 | `FUN_00474ce0` | 6 | Move of the regiments aboard into the fleet's own system, ship with most regiments first |
| 6 | `FUN_00474ff0` | 7 | Move of the side's characters aboard into the fleet's own system |
| 7 | `FUN_00475330` | 0, pass ends | Move of the fleet to the target, unless there, en route, or the target blockaded |
| 8 | `FUN_004742b0`: release every ship, the fleet and the target's claim | 0, pass ends | |

`c+0x164` is the largest attack score (class key `0x200`) of the side's
combat classes its research has reached (`FUN_0042e960`). Fleet `+0xb4` sums
its finished ships' `ship+0x90`.

State 4 always passes to 5: `FUN_00474390` tests its own argument slot after
the call, which `FUN_00474c00` only sets to 1.

Gathering pops candidate fleets best first and takes, in list order, each
free (`ship+0x24` `0x20000`) combat ship (bit 1) not en route, in combat or
unfinished (none of `0x803800`) while the need is below 0; the first
fleet that gives a ship ends the search. `FUN_00475490` ranks the side's new
fleets (`fleet+0x38` `0x8005`, none of `0xa`) by squared distance to the
target, nearest first; `FUN_00475890` ranks the regional records' fleets
(`0x8001`, none of `0xf000000e`, `fleet+0x34 > 0`) by their plan entry,
largest first. A claimed ship (`FUN_00475e70`) loses `0x20000`, gains
`0x10000000`, leaves its record and adds its attack `ship+0x7c` to the need.
Peeling releases the first claimed ship, in id order, not en route nor
unfinished (`0x2800`), whose fleet is neither in an enemy system nor under a
posture need (`FUN_00484510`), and whose attack the surplus covers; a ship
no record holds is free again (`FUN_00475f60`).

Revalidation (`FUN_004747b0`) lets the fleet go unless it is the side's,
owned by the task (`fleet+0x38` bit 8) and not empty (`FUN_00476080`: bit 8
off, bit 4 on), drops claimed ships that left the side or the task fleet,
and, with no fleet, adopts the new fleet (`fleet+0x38` bit 4) that holds a
claimed ship (`FUN_00475fe0`: bit 4 off, bit 8 on). Fleet bit 4 marks a new
fleet no record or task holds: the fleet record's constructor
(`FUN_00432e40`), `FUN_00476080` and `FUN_004791d0` set it.

## Regional records and the posture engine (module 8)

`FUN_004cf600` keeps the regional records of context `+0xd8` and steps
them through four states (`FUN_004cf630`):

- State 1 (`FUN_004cf700`) runs once per record cycle on context `+0x204`,
  the sum over sectors of `+0x18c` (`FUN_00418500` → `FUN_0041b7e0`).
  `FUN_004321e0` sets a sector's `+0x18c` to -1 when it has a posture kind
  (`+0x30`) but no record (`+0x38`), +1 when it has a record but no kind,
  else 0; sector `+0x2c` bit `0x10000` marks the -1 case. A negative sum
  creates a record without a sector (`FUN_0042f350`), unless a record still
  lacks record `+0x60` bit `0x4000000` (a class I task waiting). A positive
  sum deletes (`FUN_0042f3d0`, after `FUN_00477190` releases it) the first
  record whose sector is invalid, else the first whose sector has
  `+0x18c > 0`. So records exist only for sectors with a posture.
- State 2 (`FUN_004cf760`) moves to the next record.
- State 3 (`FUN_004cf7c0`) gives the record a sector when its own is
  invalid, has `+0x18c != 0`, or names another record in `+0x38`: the
  nearest sector with `0x10000`, then one with `0x8010000`, then any with
  `0x10000`. `FUN_004773f0` then syncs the posture object `+0x5c` to the
  sector's kind.
- State 4 steps `FUN_004774c0` once per call until the record finishes.
  An order or a finished record sets the done flag (`FUN_004cf630.c:29-42`),
  so one module-8 turn advances at most one record, and stops at its first
  order. An order before the record finishes raises `m+0x34` (`:33`), and
  `FUN_004bede0` gives the armed module with the largest `m+0x34` the next
  round, so module 8 resumes that record in the next round.
- Posture state 13 (`FUN_004b0f90`) returns the fleet's Move; the execute
  pass (state 14, `FUN_004b1070`) runs in the next round, after the Move has
  executed, and skips a fleet whose `+0x38` has bit 2 (en route,
  `FUN_004b1070.c:51`); a fleet not en route and away from its target loses
  its need (`:52`, `:100-102`). hyp: the departure dirties the fleet record
  through one of the object-change handlers above, so the census sets bit 2
  before that round and the need survives the trip, to be tested at the
  target on arrival (`:52-95`). The handler that the en-route change reaches
  is not traced.

`FUN_004774c0` leaves a record without a posture object idle. Otherwise it
runs one step per call, 1 to 6 and round again:

1. clear `+0x60`/`+0x64`; Create Fleet (`FUN_00477a40`);
2. `FUN_00477cb0` sums the record's fleets into `+0x6c`-`+0x90` (`+0x94`
   and `+0x98` are the sums of fleet `+0x84` and `+0x78`; hyp: troop and
   fighter capacity), then `FUN_004adaf0` subtracts the posture's
   requirement (slot 20: `FUN_004bb510`, `FUN_004b9fe0`, `FUN_004b4920`,
   `FUN_004b3300`, `FUN_004b6a60`, `FUN_004b8b70`), leaving shortfalls
   negative and surplus positive;
3. `FUN_00477e80` sets shortfall bits in `+0x64` and emits Create Fleet for
   each negative need counter (`FUN_004782a0`, then `FUN_00478690`) and each
   positive one (`FUN_00478aa0`);
4. `FUN_00478000`, then `FUN_00478140`;
5. the posture step `FUN_004adb70`;
6. the flagship (`FUN_00479560`), the staging system (`FUN_004799d0`),
   posture slot 8, then `FUN_00479a00`, which copies the satisfaction bits
   from `+0x64` into `+0x60`.

Posture slot 8 (`FUN_004ba5e0`, `FUN_004b9310`, `FUN_004b3830`,
`FUN_004b70a0`, `FUN_004b1760`, `FUN_004b4d90`) always sets record `+0x60`
bits `0x4000000` and `0x8000000`, and sets `0x20000` while the objective
holds, by sector `+0x2c`:

| Kind | Objective holds while |
|---|---|
| 1 | `0x80000` |
| 2 | `0x40000`, or a system not yet targeted exists |
| 3 | bit 4 |
| 4 | the build goal's target system exists and is not the side's |
| 5 | bit 2 clear, or bit 8 |
| 6 | bits 8 and `0x400`, and `0x400000` or record `+0x60` bit `0x200000` |

The record's fleet list `+0x2c` holds the fleets whose `+0x30` is the record
id, with `+0x34` set to record `+0x44` (`FUN_00478fd0`); `FUN_00477950`
drops fleets not of the side or without ships.

### Sector bits

`FUN_00432350` sets sector `+0x2c`: bit 2 when the side holds all ten
systems, 4 when it holds one, 8 when the enemy holds one, `0x10` when one is
neutral, and `0x40000` when a system has `+0x30` bit `0x200`, which
`FUN_00433e40` sets while a fleet of the other side with fleet `+0x38` bit
`0x4000` (`FUN_00433780`; hyp: a fleet with an active capital ship) is
there.

### The controller

`FUN_004e5440` steps the strategy controller: 1 runs slot 5 (the plan) when
context `+4` bit 2 is set and clears it; 2 `FUN_004e5560`; 3 the build
wants (slot 6); 4 the character requests; 5 slot 8; 6 the tagging. In state
2, when two or more records satisfied their objective on the last pass
(record `+0x60` bit `0x10000`) it sets `+0x18`, dropping one plan entry;
otherwise, when every record has `0x80000` and controller `+0x10` bit 1 is
clear, it sets `+0x14`, adding one. A record lacking `0x20000` sets context
`+4` bit 2, forcing a new plan. The plan size n is the last plan's count, +1
for `+0x14`, -1 for `+0x18`; the first plan has n = 1.

Slot 5 builds at most one lead sector of the mode's kind, then fills up to
n from a common tail. Sectors are picked by summed rank over several lists,
highest first, ties by coin flip, sectors of the last plan counted twice
(`FUN_004357b0`, `FUN_0041c250`). `FUN_00431790` writes the kind to sector
`+0x30` from the plan entry's `+0x20`.

| Mode | Slot 5 | Lead |
|---|---|---|
| 1 | `FUN_004e62b0` | none |
| 2 | `FUN_004e7990` | kind 5 |
| 3 | `FUN_004e8940` | kind 6 when a sector has `0x408`, else 5 |
| 4 | `FUN_004e9690` | kind 4 on the build goal's sector (`FUN_004e5b10`) |
| 5 | `FUN_004ea060` | kind 2 |
| 6 | `FUN_004ead50` | kind 6, else 1, else 5 |
| 7 | `FUN_004e6c00` | kind 5, then one kind 6 |

The tail, in every mode: kind 3 on the sector with `+0x24` bit
`0x40000000` and `+0x2c` bit 4 (the tagged HQ or Coruscant sector); kind 3
on sectors with `+0x2c` bits `0x16` and not 8; kind 3 on sectors with bits
6; kind 1 on sectors with `0x80000`, which modes 1, 2 and 7 take only
without bit 4. hyp, in plain words: 3 defend, 1 screen a frontier, 5 expand,
6 attack an enemy system, 2 hunt enemy fleets, 4 escort a build goal.

### Mode selection

Module 13 state 1 (`FUN_004bf490`) picks the mode every cycle from:

- R = context `+0x258` / `+0x288` (`+0x258` alone when `+0x288` is 0), the
  summed fleet word `+0xb4` of the side's fleets over the others';
  threshold 1.5 (`0x0065c49c`);
- A = `+0x25c` - `+0x1fc` + `+0x200`, B = `+0x210`;
- X = `+0x1e8`, Y = `+0x1ec`, own = `+0x1ac` (the side's systems), C =
  `+0x1c8` (sectors with a neutral system), G = `+8` (pending build-goal
  bits).

Modes 0 and 1 are always picked again; mode 4 stays while G is not 0, and
any other mode stays while its own condition holds. A fresh pick:

| Condition | Mode |
|---|---|
| G ≠ 0 | 4 |
| R < 1.5, (X < own/2 + 1 or Y < own/4 + 1), C > 0 | 2 when A ≤ B, else 7 |
| R < 1.5 otherwise | 1 when A ≤ B, else 3 |
| R ≥ 1.5, B < A | 6 |
| R ≥ 1.5 otherwise | 2 when X or Y is low, else 5 |

A fresh pick clears context `+4` bits `0x400000` and `0x800000`, sets
`0x800000` when R < 1.5 or G ≠ 0 and `0x400000` for mode 5. A change of mode
deletes the controller and sets context `+4` bit 2. hyp: X and Y are the
side's free energy and raw materials; A and B are untraced.

The posture step moves its units with Move: slot 14 (`FUN_004afce0` for kinds
1-3, `FUN_004b28c0` kind 5, `FUN_004b5f30` kind 6, `FUN_004b8470` kind 4),
`FUN_004b0120` toward the staging system, `FUN_004b06b0` for ships,
`FUN_004b0c90` to regroup and `FUN_004b0f90` to the target. Then the emitter
`FUN_004b1070` acts on an arrived fleet (`+0x28 == +0x2c`) by its need bits
`+0x38`:

| Need bit | Test | Order |
|---|---|---|
| `0x20000000` | fleet flag `0x10` and system record `+0x110 > 0` | Destroy System |
| | else `FUN_004ae240`: system record `+0x30` bit `0x20`, not `0x101`, fleet `+0x80 >= ` system `+0x118` | Planetary Assault |
| | else `FUN_004ae320` | a bombardment (below) |
| `0x40000000` | the Destroy System test, else `FUN_004ae320` | Destroy System or a bombardment |
| `0x80000000` | `FUN_004ae440`: system bit 2, not `8` or `0x200`, fleet `+0x80 > 0` | Move of the fleet's regiments (`FUN_004b1440`) |
| `0x10000000` | `FUN_004ae150` | none |

`FUN_004ae320` needs system record bit 4 and `+0x3c`, fleet `+0x68 >` system
`+0x68`, and system `+0x74 == 0`. With d = (`[+0x3c]+0x78 - [+0x3c]+0x80 + 1`)
/ 2, it picks General Bombardment when d < 1 and system `+0x90` or `+0xcc` is
positive, else Target Military Facilities when `+0x64 + +0x90 > d` and
`+0x90 > 0`. `FUN_004b1330` builds the chosen order with the target `+0x2c`.

How long a need lasts (`FUN_004b1070.c:58-102`; `:101` is the only writer
that clears `+0x38` bits `0xf0000000` among the dumped functions):

- Engage stays while `FUN_004ae150` passes and emits nothing (`:93-97`).
  At an enemy system (`+0x30` bit 4) the test counts the system's
  squadrons, `+0xb0`/`+0xb4`/`+0xa4`, besides a foreign fleet's
  `+0xe4`/`+0xe8`/`+0xf4`, so the need outlives the enemy fleet while
  squadrons the fleet beats remain there. "Beats" is the fleet's `+0x58`
  above 1.2 times the enemy's first count, `+0x5c` above its second and
  `+0x74` at least its third (above it when `+0x38` bit 4 or the system's
  `+0x30` bit 10 is set; skipped when `+0x9c` is positive otherwise).
- Assault stays while `FUN_004ae240` or `FUN_004ae320` passes, whether or
  not `FUN_004b1330` returns an order (Destroy System, Planetary Assault or
  a bombardment, `:78-91`), and clears when neither applies (`:82` → `:101`).
- Bombard clears on the call that emits its order, or when there is none
  (`:60-70` → `:101`).
- Drop stays while `FUN_004ae440` passes (`:72-76`), whether or not `FUN_004b1440` returns an order, one regiment per call,
  until the fleet's `+0x80` reaches 0.

A fleet with any need bit is never free for state 5 (`FUN_004b04c0.c:52`,
mask `0xf0003802`), so only a kept need holds a fleet at its target.

The posture's side `+4` is the regional record's `+0x20`, which the fleet
tasks also use as the order owner: 1 Alliance, 2 Empire
(`side-numbering.md`).

## Character tasks (module 12)

Module 12 (`FUN_004c8260`, slot 5 `FUN_004c84a0`, steps `FUN_004c84d0`) keeps
the character tasks in the list at context `+0xec`. A task is built by
`FUN_0042ef50` → `FUN_0042efe0` (key = list-assigned id, `+0x24` the side that
owns its orders). Slot `+0xc` of a task returns its class: 2 for class G,
1 for class I. Sides are the game's: 1 Alliance, 2 Empire.

| Kind | Constructor | Class | Order (slot `+0x1c`) | Character requirement |
|---|---|---|---|---|
| 1 | `FUN_0047a430` | G | General `0x263` | `+0x34` bit `0x4000000` (can be general) |
| 6 | `FUN_00479a60` | G | none, the task only moves | `+0x30` bit `0x400`, the Emperor's mark |
| 3 | `FUN_0047a3c0` | I | Admiral `0x262` (`FUN_0047a400`) | `+0x34` bit `0x1000000` (can be admiral) |
| 4 | `FUN_0047a000` | I | General `0x263` (`FUN_0047a040`) | `+0x34` bit `0x4000000` |
| 5 | `FUN_00479f70` | I | Commander `0x261` (`FUN_00479fd0`) | `+0x34` bit `0x2000000` (can be commander) |

Record bits used below (AI character record, `FUN_00401d20`, `FUN_00402230`).
Capability bits at `+0x34`: `0x1000000` admiral, `0x2000000` commander,
`0x4000000` general, set from the class flags (`FUN_004ed1c0`, `FUN_004ed1e0`,
`FUN_004ed200`) for every character except the six main ones (`FUN_004025f0`,
record bit `0x8`), which never carry them. Flags at `+0x30`: `0x800000`
free for a task, `0x2` this side's, `0x1` unrecruited, `0x10` injured, `0x20`
en route, `0x40` prisoner, `0x80` on a mission, `0x400000` on a mandatory
mission, `0x4000000`/`0x1000000`/`0x2000000` holds rank Commander/Admiral/
General (`+0x96` = 1/2/3), `0x8000000` reserved, `0x40000000` one of the four
leaders while context `+4` bit 1 is clear. `+0x38` is the id of the character's
container (the system when it sits in one, a ship or fleet when aboard);
`+0x28` is the id of the task that claimed it and `+0x2c` a tag the task writes.

### Where tasks come from

| Request | Writer | Reader |
|---|---|---|
| context `+0x358` = 1 or 6 (General, Emperor move) | `FUN_004e57e0` from slot 7 `FUN_004eaaa0` of the strategy controller | `FUN_004c85b0` creates the task, clears `+0x358`, counts it with `FUN_0041b5d0` |
| context `+0x354` = id of a class G task to drop | `FUN_004e57e0` | `FUN_004c85b0` → `FUN_0042ef90` sets task flag `0x80000000` |
| regional record `+0xe0` = 3, 4 or 5 (Admiral, General, Commander) | `FUN_00478140` through posture slot 7 | `FUN_004c86b0` creates the task, registers it with `FUN_00476d10` |
| regional record `+0xe4` = id of a class I task to drop | `FUN_00478140` | `FUN_004c86b0` → `FUN_0042ef90` |

**Kinds 1 and 6 (context `+0x358`).** The strategy controller `FUN_004e5440`
(six states, one per call of `FUN_004bf390`, run in the first phase of module
13 `FUN_004bf320` for both sides) reaches slot 7 in state 4. All seven
controller classes (modes 1-7, vtables `0x0065c8c0`..`0x0065c9e0`) share slot 7
`FUN_004eaaa0`. With n1 = context `+0x35c` (kind 1 tasks created), n6 =
`+0x360` (kind 6 tasks created) and g = context `+0x220`:

- n1 < g: request kind 1; n1 > g: set the drop flag.
- Side 2 only: when no kind 1 was requested, context `+0xa4` is 0, context
  flag `+4` bit `0x80` is set and n6 = 0, request kind 6. When no drop flag is
  set, n6 = 1 and (`+0xa4 > 0` or flag `0x80` clear), set the drop flag.
- `FUN_004e57e0` stores a requested kind in `+0x358`. A set drop flag picks the
  first class G task in the list whose flag `0x2` (valid target) is clear, of
  either kind, and stores its id in `+0x354`; it stays 0 when none qualifies.
- `+0x35c`/`+0x360` have no decrement: their writers are the constructor
  (`FUN_004178f0`), `FUN_0041b5d0` (+1 per created class G task, index = slot
  `+0x28`: 0 for kind 1, 1 for kind 6) and the save codec (`FUN_00417cf0`,
  `FUN_00417fa0`). The counters hold the tasks ever created, so a released
  kind 1 task is not replaced while n1 has reached g, and kind 6 is created
  once per game.
- g = Σ over the side's sectors of sector `+0x188` (`FUN_0041b7e0`), the count
  of systems whose record `+0x30` carries bit `0x10000000` (`FUN_00432350`);
  bit `0x10000000` of the sector's `+0x2c` is set while one of them is
  unclaimed (`+0x30` bit `0x20000000` clear).
- Tagging (controller state 6, `FUN_004e5870`, both sides): Coruscant
  (`0x90000109`) carries `0x10000000` while the side holds it (`+0x30` bit 1).
  Side 1 also keeps context `+0x28` on the system holding its HQ: it stays
  tagged while the system is the side's (`+0x30` bit 1) and still holds the HQ
  (`+0x30` bit `0x80000000`, `FUN_00526a60`: family `0x20`-`0x21` object of the
  side); else the tag and `+0x28` are cleared and a new one is picked
  (`FUN_00419a00`, `FUN_0041a340`: a sector whose `+0x2c` has `0x80000000`,
  then a system in it with `+0x30` `0x80000000`) and tagged when the side holds it.
  The Empire has no HQ object, so its g is 1 while it holds Coruscant, else 0.
  The Alliance's g is the HQ system plus Coruscant when it holds it (at most 2).
  The totals refresh when `FUN_00418500` recomputes the sector census
  (context flag `0x20000000`).
- Context flag `+4` bit `0x80` is set by `FUN_0041b7e0` from a sector whose
  `+0x2c` has bits 4 (holds a system of the side) and `0x200` (holds
  Coruscant): the side holds a system in Coruscant's sector. Context `+0xa4`
  is copied from `+0x14c` by `FUN_004182a0` (hyp: meaning untraced).

So kind 1 means "put a General at Coruscant, and at the HQ system for the
Alliance"; kind 6 means "bring the Emperor to Coruscant", Empire only.

**Kinds 3, 4, 5 (regional record `+0xe0`).** A regional record (context list
`+0xd8`, class `FUN_004762b0`; `+0x34` its sector id) is stepped by
`FUN_004774c0` from module 8. State 2 (`FUN_00477cb0`) sums its fleets'
record words into `+0x6c`..`+0x90` and copies `+0x74` to `+0x94` and `+0x78`
to `+0x98` (hyp: fleet words 10 and 7, summed over capital ships; troop and
fighter capacity). State 4 runs `FUN_00478000` and `FUN_00478140`, which
clears `+0xe0`/`+0xe4`, calls posture slot 7 (`+0x1c`) with `&+0xe0` and sets
`+0xe4` to the first id in the record's task list (`+0xd4`) for which
`FUN_004add40` returns 1. Counters `+0xc8`, `+0xcc`, `+0xd0` count the live
Admiral, General and Commander tasks (`FUN_00476d10` adds, `FUN_00476de0`
removes; index by slot `+0x18` through `FUN_00479520`: 3 → 0, 4 → 1, 5 → 2).
The posture kind (1-6, sector `+0x30`) is written by the seven controller
classes' slot 5 (`FUN_004e62b0`, `FUN_004e6c00`, `FUN_004e7990`,
`FUN_004e8940`, `FUN_004e9690`, `FUN_004ea060`, `FUN_004ead50`; controller
state 1 `FUN_004e5530`) through `FUN_00431790`, taking the kind from a plan
entry's `+0x20` ("The controller", above).

| Posture kinds | Slot 7 | Result written to `+0xe0` |
|---|---|---|
| 1, 2 | `FUN_004b9290` | 3 when Admiral count < 1 and the fleet list `+0x2c` is non-empty; else 5 when Commander count < 1, `+0x98 > 0` and sector `+0x2c` has `0x40008`; else 0 |
| 3, 5 | `FUN_004b1800` | the same as kinds 1, 2 |
| 4, 6 | `FUN_004b72d0` | 4 when General count < 1, `+0x94 > 0` and sector `+0x2c` has `0x8`; else 3 or 5 as above |

Sector `+0x2c` bit `0x8` marks an enemy-held system (`FUN_00432350`); bit
`0x40000` is set from a child system's `+0x30` bit `0x200`, which
`FUN_00433e40` sets for a fleet of the other side carrying record flag `0x4000`
(hyp for that reading of the flag).

Cancel test `FUN_004add40` (task kind by slot `+0x18`; 1 when the id is no
longer in the list): Admiral `FUN_004b1650` cancels with count ≥ 2, or count =
1 and the fleet list empty; Commander `FUN_004b16d0` cancels with count > 1,
or count > 0 and `+0x98 = 0`; General: posture kinds 1, 2, 3, 5 cancel when
count > 0 (`FUN_004b1690`), kinds 4, 6 when count > 1 (`FUN_004b8b30`). A
sector whose kind becomes 0 releases all the record's tasks (`FUN_00477190`
via `FUN_004773f0`); a change from one kind to another builds the new
posture without releasing them.

The record never names a target in `+0xe0`. The task reads its targets from
the record at each pass (`FUN_004bc5d0`): `+0x3c` → task `+0x48` the staging
system, `+0x38` → task `+0x28` the flagship capital ship. `+0x3c` is chosen by
posture slot 4 `FUN_004adde0` (keep the anchor `+0x40` while the side holds
it, else a system of the side in the sector: first one with `+0x28` bit
`0x80000000`, then `0x3e00000`, then any, random among equals).
`+0x38` is chosen by `FUN_00479560` while the record has class I tasks (record
`+0x60` bit `0x2000000`, set in `FUN_00478140`): the fleet's best ship by
ship-record word `+0x50`, marked with `0x1000000` in its `+0x24`, among the
record's fleets (`FUN_0041ac80`). `FUN_004799d0` clears `+0x3c` when no task
exists.

### Module 12 steps

Module 12 re-arms each round (slot 4 `FUN_004cc990` sets its state `+0x1c` to
1 and its done flag `+0x20` to 0). Its sub-state `+0x3c` advances one step per
call:

| State | Function | Action |
|---|---|---|
| 1 | `FUN_004c85b0` | drop the task in `+0x354`; create the task for `+0x358` |
| 2 | `FUN_004c8630` | find the next regional record with `+0xe0` or `+0xe4` set (`FUN_00476cf0`), cursor `+0x44`; none → state 4 |
| 3 | `FUN_004c86b0` | on the found record: drop `+0xe4`; create the task for `+0xe0` |
| 4 | `FUN_004c8740` | next runnable task from cursor `+0x4c` in the list (slot `+0x10`, `FUN_004bba90`: 0 while the claimed character is on a mission or en route); none → done |
| 5 | `FUN_004c87c0` | step that task once (slot `+0x14`) |

A step that returns an order sets the module's done flag; a task that finishes
its pass resets the module to state 1. Class G and I tasks share one machine
(`FUN_004bbb00`, `FUN_004bc480`): state 0 → 1; state 1 → 3 (flag
`0x80000000` clear) or release; 3 → 2 target; 2 → 4 when a character is held,
else the pass ends with no order; 4 issues; every pass ends back in state 0.
Each pass therefore re-validates target and character before issuing.

### Class G

State 3 (slot `+0x34`) validates the target `+0x28`; state 2 (slot `+0x38`)
validates or picks the character; state 4 (`FUN_004bbc50`) issues.

- **Target, kind 1** (`FUN_0047a480`): valid while the system's record
  (context `+0x2c`) has `+0x30` bit `0x10000000`; otherwise the old claim bit
  is dropped (`FUN_0047a850` clears `0x20000000`) and `FUN_0047a870` picks:
  a sector with `+0x24` `0x40000000` and `+0x2c` `0x10000000`, then a system in
  it with `+0x28` `0x80000000`, `+0x30` `0x10000000` and without `0x20000000`
  (values: sector word `0x26`, system word `0x21`, largest wins); when that
  yields no system, a sector with `+0x2c` `0x10000000` and a system with
  `+0x30` `0x10000000` and without `0x20000000`.
- **Target, kind 6** (`FUN_00479ad0`): valid while the system's `+0x30` has
  `0x40000000` and bit 1; `FUN_00479e30` picks a sector (`+0x24` and `+0x2c`
  `0x40000000`) and a system in it with `+0x28` `0x80000000` and `+0x30`
  `0x40000000`, at random: Coruscant.
- **Character, kind 1** (`FUN_0047a5b0`): the held character stays while its
  record's `+0x28` equals the task id and `+0x30 & 0x50 = 0`; a prisoner or
  injured character is freed (`+0x30 |= 0x800000`, `+0x24`, `+0x28`, `+0x2c`
  cleared, target claim dropped) and the task drops it. Without a character, a
  target whose record is missing or already claimed (`+0x30` bit `0x20000000`)
  is cleared from the task; otherwise two candidate lists are built and
  combined (`FUN_00435790`, `FUN_004357b0`): A `FUN_00403460`: `+0x30` has `0x800002`, none of
  `0x4f4000f1` (`0x40000000` leaders, `0x0f000000` reserved or any held rank,
  `0x400000`, `0x80`, `0x40`, `0x20`, `0x10`, `0x01`), `+0x34` has `0x4000000`,
  Leadership (record `+0x58`, slot `+0x1f4`) from 50 to 10000, sorted
  ascending (`FUN_0041c250` mode 1); B `FUN_00403750`: the same flags without
  the skill filter, sorted by squared distance (`FUN_0041b7b0`) from the
  character's system to the target, descending (mode 2). Ties break at
  random. A candidate must be in both lists; the one with the largest rank
  sum (best Leadership, nearest) is tried first, the next best after a refusal.
  `FUN_004bc180` accepts a character with a record unless it sits in a system
  (`+0x38` family `0x90`-`0x97`, flag `0x20` clear) whose record `+0x28` has
  bit 2 (hyp: set by `FUN_00433e40` from the system object's `+0x88` bit
  `0x20`; a contested system). The accepted record gets `+0x30 &= ~0x800000`,
  `+0x28` = task id, `+0x2c` = 0, `FUN_004021b0`, and the target's `+0x30`
  gains `0x20000000`.
- **Character, kind 6** (`FUN_00479c00`): list A only, `+0x30` has `0x402`
  (`0x400` marks the Emperor), none of `0x74000f1` (held ranks, `0x400000`,
  `0xf1`), Leadership 0-10000; the target's record must have `+0x30` bit 1.
  The held character stays while `+0x30 & 0x50 = 0` and `+0x30 & 0x400`.
- **State 4** (`FUN_004bbc50`) acts only for a character whose record has none
  of `0x74000f1`: it stops once the character holds any rank. Character
  container (`+0x38`) equal to the target means "at the target system": kind 1
  issues Command General (`FUN_004bbf50`: order `0x263` with the character as
  team, owner = task side); kind 6 issues nothing. A character elsewhere gets
  Move `0x201` to the target (`FUN_004bc040`). A character aboard a fleet has a
  ship or fleet id in `+0x38` and gets the Move too.

### Class I

State 3 (`FUN_004bc5d0`) copies `+0x3c`/`+0x38` of the regional record
`+0x54` into task `+0x48` (system) and `+0x28` (flagship) and sets flag `0x2`
when the ship id has family `0x14`-`0x1b` and the system family `0x90`-`0x97`.
State 2 (slot `+0x28`; kinds 3 and 5 `FUN_004bc6d0`, kind 4 `FUN_0047a050`):
flags `0x1` held, `0x8` aboard the flagship, `0x4` aboard and ranked. The
character list is A (as for kind 1 with the kind's capability bit, mask
`0x4f4000f1`; kind 4 uses `0x474000f1` and so accepts a reserved character)
combined with B ranked by distance to the staging system (the sector's first
system `FUN_00432120` when the record has none). Acceptance, claim and
`+0x2c` = task `+0x4c` (the record's `+0x44` value, set by `FUN_004bc410`).
State 4 (`FUN_004bca20`, stops when the character's `+0x30` has any of
`0x74000f1`):

1. Character container = flagship (ship family `0x14`-`0x1b`): flag `0x4`,
   issue the kind's Command (`FUN_004bbf50`, orders `0x262`, `0x263`, `0x261`).
2. No flagship id: Move to the staging system, or flag `0x10` (waiting) when
   already there.
3. Flagship set, character aboard another ship: same fleet → Move onto the
   flagship (flag `0x8`); different fleets in different systems → Move to the
   staging system; same system → Move onto the flagship.
4. Flagship set, character in a system: the flagship's fleet in another
   system → Move to the staging system (flag `0x10` when already there); same
   system → Move onto the flagship (flag `0x8`).
   The ship records (context `+0x58`) must exist; nothing boards a flagship
   whose `+0x24` has bit `0x800` (en route, object `+0x50` bit 4,
   `FUN_00484630`).

### Release

Flag `0x80000000` is set by `FUN_0042ef90` only, from `FUN_004c85b0`
(`+0x354`), `FUN_004c86b0` (`+0xe4`) and `FUN_00477190` (every class I task
of a record whose posture changed). In state 1 `FUN_004bbba0` (class G) or
`FUN_004bc520` (class I) removes the task from `+0xec` into the finished list
`+0x104` (`FUN_0041b600`), frees the character (record `+0x30 |= 0x800000`,
`+0x2c` = 0) when it still carries the task id, and issues Command None
(`0x260`) when the character holds a rank and has neither `0x80` nor `0x20`;
class G also clears the target's claim bit (slot `+0x2c`). `FUN_0042ef90` also
unregisters a class I task from its record (`FUN_00476de0`). Tasks end no
other way.

### Counts, cadence and sides

- Class G tasks: kind 1 up to g (above; at most 2 for the Alliance, 1 for the
  Empire) and kind 6 once, Empire only. Class I tasks: per regional record at
  most one each of Admiral, General (posture 4, 6) and Commander. A pass of
  module 12 creates at most one class G task (state 1) and one class I task
  (state 3), and steps one task.
- Module 12 is built for both sides: `FUN_00485a50` adds the same 14 modules
  (ids 13, 2, 1, 3, 6, 7, 4, 5, 8, 9, 10, 11, 12, 14) to each computer
  player. Module 12's pass returns at most one order and sets its done flag,
  so each pass ends its round (see "Scheduler and executor").
- Controller proposals (kinds 1, 6) arise once per six calls of module 13's
  first phase; regional proposals (kinds 3-5) once per six steps of the record
  in module 8; module 12 consumes one regional proposal and one controller
  proposal per pass.

### Rank clears

The AI clears a rank on purpose only through release (Command None). The
engine clears it as a side effect of container changes: the moved-object hook
`FUN_00514a60` calls `FUN_00560d50`, which clears the rank of a character whose
container changes and of every character aboard a capital ship whose container
changes (a ship joining another fleet, as in Create Fleet), except a move
between two ships of one fleet. `FUN_004eeed0` clears it when the character is
injured, a prisoner, has `+0x50` bit 6 clear or has `+0x68 >> 24` in
`0x50`-`0x7f`. The task states never move a ranked character (the `0x74000f1`
guard), the candidate queries exclude ranked characters, and the mission
planners drop members with `0x74000f1` (`FUN_0047b360`). The fleet tasks of
modules 8, 9 and 11 test nobody's rank: a Create Fleet that moves a ship
carrying a ranked character clears the rank (hyp: through `FUN_00560d50`; a
fleet Move between systems leaves the ship's container unchanged).

## Mission planners (module 4)

`ai-mission-planning.md` covers the team, decoys and Mission order
(`FUN_0047b360`). The planners also emit:

- **Move**: `FUN_004bd570` from tick `FUN_004bced0`; `FUN_004be230` from the
  second tick `0x004bda60` (vtable `0x0065c408`), which gathers every team and
  decoy member not yet at the target into one Move before the Mission order.
- **Abort**: `FUN_0047b250` aborts the planner's mission while it is in
  state 8 (`FUN_00520b30`) and the planner's flags carry `0x20000000`, or
  `0x40000000` when slot `+0x34` returns 0.

## Port

`crates/rebellion-core/src/ai_scheduler.rs` runs a side's round and returns
its order as an `AIAction`. `ai_step` (`crates/rebellion-data/src/simulation.rs`),
shared by the native game and the headless runs, applies each side's pending
order at the next in-day step through `integrator::apply_ai_order`, and the
strikes through `integrator::resolve_ai_strike`:

| Original order | Port |
|---|---|
| Mission `0x240` | `AIAction::DispatchMission` |
| Build `0x210`-`0x212`, Destination `0x214`, Scrap `0x200`, Retire `0x242` | `AIAction::EnqueueProduction` (with its Destination) and `AIAction::Scrap` from `crate::ai_production` (`ai-production.md`); a Retire arrives as a Scrap |
| Move `0x201` of a fleet | `AIAction::MoveFleet`, optionally with regiments aboard |
| Create Fleet `0x270` | `AIAction::CreateFleet` (`fleet_join::create_fleet_of_tags`) |
| Move `0x201` of a free fleet's damaged ships to a system (`FUN_004c8e90`) | `Order::MoveShips`, then `AIAction::CreateFleet` (`ai_production::repair_move`) |
| Move `0x201` of ships onto a fleet | `AIAction::JoinFleet` (`fleet_join::mover_of_tags`, `join_fleet`) |
| Move `0x201` of regiments, squadrons, characters off a fleet | `AIAction::MoveRegiments`, `MoveSquadrons`, `MoveCharacters` |
| Move `0x201` of a character task's character | `AIAction::MoveCharacter` to a system, `AIAction::MoveCharacterToFleet` onto the flagship's fleet, both through `MissionState::dispatch_move` |
| Command `0x260`-`0x263` | `AIAction::GiveCommand`, through `officer_rank::command_refusal` and `give_command` |
| Planetary Assault `0x234` | `AIAction::Assault` |
| General Bombardment `0x222`, Target Military Facilities `0x220`, Destroy System `0x223` | `AIAction::Bombard`, `AIAction::DestroySystem` (`integrator::resolve_ai_strike`) |
| Move of the HQ, Rename `0x203`, Abort `0x250` | none |

The modules live in `ai_strategy.rs` (module 13 and the controller),
`ai_regions.rs` (module 8), `fleet_tasks.rs` (module 9),
`character_tasks.rs` (module 12), `ai_production.rs` and
`mission_planning.rs`, their state in `AIState`, and the round scheduler
in `ai_scheduler.rs` (see "Scheduler and executor"). port: module 14, the
renames, is not armed. hyp: a kind 1 task takes the first free tagged system
in id order (`FUN_0047a870`'s sector and system values are untraced). The officer
queries forbid the reserved bit `0x8000000` (the General of a record
excepted), which every character carries until its side has finished all
three research trees (`ai-mission-planning.md`, record flags), so those
tasks find no character before then.

## Open

- The destination queries `FUN_00419b80`, `FUN_0041a400`, `FUN_0041a4c0`,
  `FUN_0041ab40` (their flags) and the Create Fleet targets of
  `FUN_004782a0`, `FUN_00478690`, `FUN_00478aa0`.
- Fleet record fields `+0x5c`, `+0x68`, `+0x74`, `+0x80` and system record
  fields `+0x110`, `+0x118`; whether fleet flag `0x10` marks the Death Star.
- Why module 7 scraps fighters when no source is found; module 3's
  Coruscant case.
- Who drives player slots `0x64`/`0x68` (the live `c+0x184`, `+0x188`).
- The writers of the record need bits `rec+0x64` and of `c+0x16c`,
  `c+0x20c`, `c+0x2fc`; the site query flags (`FUN_00419a00`,
  `FUN_00419b80`, `FUN_0041a340`, `FUN_0041a4c0`, `FUN_0041ab40`); the
  Move or Scrap source of modules 7 (`0x3f1`), 3 (`FUN_004d7cf0`) and 2.
- Whether `FUN_00553960` or the ledgers refuse on stock; the sort key of
  `FUN_004d0610`'s Retire.
- The planner slot `+0x34` behind Abort.
- The mode inputs A and B; context `+0xa4` (`FUN_004182a0`). Ship record
  `+0x24` bits `0x1000` and `0x800000` are a damaged ship (object `+0x50` bit 9)
  that can, or cannot, still jump (slot `+0x34`); the splits `FUN_004749b0` and
  `FUN_00477a40` and the move `FUN_004c8e90` read them (`ship-damage-repair.md`).

## Decompiles

`FUN_004f5cd0.c`, `FUN_00402720.c`, `FUN_004be930.c`, `FUN_004bea20.c`,
`FUN_004199b0.c`, `FUN_004c8b30.c`, `FUN_004c8e90.c`, `FUN_004c94c0.c`,
`FUN_004c9cb0.c`, `FUN_004caad0.c`, `FUN_004cab20.c`, `FUN_004ca5e0.c`,
`FUN_004cba90.c`, `FUN_004cb1d0.c`, `FUN_004cb7b0.c`, `FUN_004cbbb0.c`,
`FUN_004cbcf0.c`, `FUN_004cc9c0.c`, `FUN_004cd310.c`, `FUN_004ce350.c`,
`FUN_004ce490.c`, `FUN_004cda90.c`, `FUN_004ce070.c`, `FUN_004ce5b0.c`,
`FUN_004ce6f0.c`, `FUN_004cf290.c`, `FUN_004cf600.c`, `FUN_004cfed0.c`,
`FUN_004d0480.c`, `FUN_004d01a0.c`, `FUN_004d15f0.c`, `FUN_004d0610.c`,
`FUN_004d0d30.c`, `FUN_004d1310.c`, `FUN_004d1710.c`, `FUN_004d1850.c`,
`FUN_004d3100.c`, `FUN_004d3a90.c`, `FUN_004d3db0.c`, `FUN_004d3ff0.c`,
`FUN_004d4800.c`, `FUN_004d5dd0.c`, `FUN_004d5f10.c`, `FUN_004d5510.c`,
`FUN_004d5af0.c`, `FUN_004d6030.c`, `FUN_004d6170.c`, `FUN_004d6d30.c`,
`FUN_004d7330.c`, `FUN_004d8520.c`, `FUN_004d76f0.c`, `FUN_004d7ac0.c`,
`FUN_004d7cf0.c`, `FUN_004d9b00.c`, `FUN_004d9e70.c`, `FUN_004d9fb0.c`,
`FUN_004d8fe0.c`, `FUN_004d95c0.c`, `FUN_004dabe0.c`, `FUN_004db2f0.c`,
`FUN_004db820.c`, `FUN_004dbe70.c`, `FUN_004dc650.c`, `FUN_004dc9f0.c`,
`FUN_004dcb30.c`, `FUN_004e2120.c`, `FUN_004e2940.c`, `FUN_004e31b0.c`,
`FUN_004e4300.c`, `FUN_004e4440.c`, `FUN_004e3a40.c`, `FUN_004e4020.c`,
`FUN_004dd7a0.c`, `FUN_004ddb70.c`, `FUN_004de130.c`, `FUN_004de6f0.c`,
`FUN_004deba0.c`, `FUN_004dfa80.c`, `FUN_004e0a30.c`, `FUN_004e0b70.c`,
`FUN_004e0170.c`, `FUN_004e0750.c`, `FUN_004e0c90.c`, `FUN_004e0dd0.c`,
`FUN_004d29b0.c`, `FUN_004d2b30.c`, `FUN_004bf320.c`, `FUN_004bf390.c`,
`FUN_004bf3e0.c`, `FUN_004bf9e0.c`, `FUN_004bfe30.c`, `FUN_004c05e0.c`,
`FUN_00474390.c`, `FUN_004749b0.c`, `FUN_00474c00.c`, `FUN_00475490.c`,
`FUN_00475890.c`, `FUN_00475c70.c`, `FUN_00474ce0.c`, `FUN_00474ff0.c`,
`FUN_00475330.c`, `FUN_004774c0.c`, `FUN_00477a40.c`, `FUN_00477e80.c`,
`FUN_004782a0.c`, `FUN_00478690.c`, `FUN_00478aa0.c`, `FUN_004adb70.c`,
`FUN_004afce0.c`, `FUN_004b28c0.c`, `FUN_004b5f30.c`, `FUN_004b8470.c`,
`FUN_004b0120.c`, `FUN_004b06b0.c`, `FUN_004b0c90.c`, `FUN_004b0f90.c`,
`FUN_004b1070.c`, `FUN_004ae150.c`, `FUN_004ae240.c`, `FUN_004ae320.c`,
`FUN_004ae440.c`, `FUN_004b1330.c`, `FUN_004b1440.c`, `FUN_004c8260.c`,
`FUN_004c84a0.c`, `FUN_0042ef50.c`, `FUN_0042efe0.c`, `FUN_00479a60.c`,
`FUN_0047a430.c`, `FUN_00479f70.c`, `FUN_0047a000.c`, `FUN_0047a3c0.c`,
`FUN_004bbb00.c`, `FUN_004bbba0.c`, `FUN_004bbc50.c`, `FUN_004bba10.c`,
`FUN_004bc480.c`, `FUN_004bc520.c`, `FUN_004bc5d0.c`, `FUN_004bca20.c`,
`FUN_004bc290.c`, `FUN_004bbf50.c`, `FUN_004bc040.c`, `FUN_004bced0.c`,
`FUN_004bd570.c`, `FUN_004be230.c`, `FUN_0047b250.c`, `FUN_00520b30.c`,
`FUN_0041a340.c`, `FUN_0041b5d0.c`, `FUN_004361a0.c`, `FUN_00476cf0.c`, `FUN_00476de0.c`, `FUN_00479520.c`, `FUN_004799d0.c`, `FUN_00479a00.c`, `FUN_00479ac0.c`, `FUN_0047a040.c`, `FUN_0047a470.c`, `FUN_0047a850.c`, `FUN_0047aa90.c`, `FUN_004b1650.c`, `FUN_004b1690.c`, `FUN_004b16d0.c`, `FUN_004b8b30.c`, `FUN_004bba90.c`, `FUN_004bc410.c`, `FUN_004c87c0.c`, `FUN_004e5530.c`, `FUN_00485a50.c`, `FUN_00485de0.c`,
`FUN_00401d20.c`, `FUN_004021b0.c`, `FUN_00402230.c`, `FUN_004025f0.c`, `FUN_00403460.c`, `FUN_00403750.c`, `FUN_004178f0.c`, `FUN_00417cf0.c`, `FUN_00417fa0.c`, `FUN_004182a0.c`, `FUN_00418500.c`, `FUN_00419a00.c`, `FUN_0041ac80.c`, `FUN_0041b600.c`, `FUN_0041b7b0.c`, `FUN_0041b7e0.c`, `FUN_0041c250.c`, `FUN_0041cf20.c`, `FUN_0042ef90.c`, `FUN_00432120.c`, `FUN_00432350.c`, `FUN_00433e40.c`, `FUN_00435790.c`, `FUN_004357b0.c`, `FUN_004762b0.c`, `FUN_00476d10.c`, `FUN_00477190.c`, `FUN_004773f0.c`, `FUN_00477cb0.c`, `FUN_00478000.c`, `FUN_00478140.c`, `FUN_00479560.c`, `FUN_00479ad0.c`, `FUN_00479c00.c`, `FUN_00479e30.c`, `FUN_0047a050.c`, `FUN_0047a480.c`, `FUN_0047a5b0.c`, `FUN_0047a870.c`, `FUN_0047b360.c`, `FUN_00485b40.c`, `FUN_004add40.c`, `FUN_004adde0.c`, `FUN_004b1800.c`, `FUN_004b72d0.c`, `FUN_004b9290.c`, `FUN_004bc180.c`, `FUN_004bc6d0.c`, `FUN_004be980.c`, `FUN_004bede0.c`, `FUN_004c84d0.c`, `FUN_004c85b0.c`, `FUN_004c8630.c`, `FUN_004c86b0.c`, `FUN_004c8740.c`, `FUN_004cc990.c`, `FUN_004e5440.c`, `FUN_004e57e0.c`, `FUN_004e5870.c`, `FUN_004e62b0.c`, `FUN_004e6c00.c`, `FUN_004e7990.c`, `FUN_004e8940.c`, `FUN_004e9690.c`, `FUN_004ea060.c`, `FUN_004eaaa0.c`, `FUN_004ead50.c`, `FUN_004ed1c0.c`, `FUN_004ed1e0.c`, `FUN_004ed200.c`, `FUN_004eeed0.c`, `FUN_00514a60.c`, `FUN_00526a60.c`, `FUN_00560d50.c`, `FUN_004184f0.c`, `FUN_00431790.c`, `FUN_00479fd0.c`, `FUN_0047a400.c`, `FUN_0041c120.c`, `FUN_0041c2c0.c`, `FUN_004317b0.c`, `FUN_004321d0.c`, `FUN_004b1720.c`, `FUN_004b1760.c`, `FUN_004b37f0.c`, `FUN_004b3830.c`, `FUN_004b4d50.c`, `FUN_004b4d90.c`, `FUN_004b6fb0.c`, `FUN_004b70a0.c`, `FUN_004b8ab0.c`, `FUN_004b8b70.c`, `FUN_004b91a0.c`, `FUN_004ba580.c`, `FUN_004ba5e0.c`, `FUN_004e5260.c`, `FUN_004e54f0.c`, `FUN_004e69d0.c`, `FUN_004e7790.c`, `FUN_004e77c0.c`, `FUN_004e8740.c`, `FUN_004e8770.c`, `FUN_004e94a0.c`, `FUN_004e94d0.c`, `FUN_004e94e0.c`, `FUN_004e9e90.c`, `FUN_004eab80.c`, `FUN_00418240.c`, `FUN_00418270.c`, `FUN_00418970.c`, `FUN_00418990.c`, `FUN_0041b0d0.c`, `FUN_0041b270.c`, `FUN_00473060.c`, `FUN_00473250.c`, `FUN_00473490.c`, `FUN_00473640.c`, `FUN_00473930.c`, `FUN_004cc220.c`, `FUN_004d0570.c`, `FUN_004d1d90.c`, `FUN_004d47b0.c`, `FUN_004d66a0.c`, `FUN_004da4d0.c`, `FUN_004dfa70.c`, `FUN_00538c70.c`, `FUN_00538d40.c`, `FUN_00538e10.c`,
`FUN_00436530.c`, `FUN_0041d050.c`, `FUN_004d2970.c`, `FUN_004d2b10.c`, `FUN_004d2460.c`, `FUN_004dd810.c`, `FUN_004cf2c0.c`, `FUN_004d2490.c`, `FUN_0051df30.c`, `FUN_0041e500.c`, `FUN_0041e390.c`, `FUN_0051dea0.c`, `FUN_004cf630.c`, `FUN_0041b530.c`, `FUN_0042f640.c`, `FUN_004317d0.c`, `FUN_00433600.c`.
