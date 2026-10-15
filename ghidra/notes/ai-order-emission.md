---
title: "AI Order Emission: Every Order the Computer Player Sends"
description: "The computer player's orders by source: 13 of its 14 task modules, the fleet tasks and posture engine under modules 9 and 8, the character-command tasks of module 12, and the mission planners; each order's trigger, side gate and the port's matching path or its absence"
category: "ghidra"
updated: 2026-10-14
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
  `+0x184 - +0x188`.
- **Goal token.** A builder clears its bit in context `+8` and builds only
  while context `+0xc` equals it; `FUN_004199b0` then rotates `+0xc` to the
  next pending bit.
- **Class choice.** `FUN_0041b230(ctx, out, type, …)` →
  `FUN_0052e580(side, type)` picks at random among the side's classes of
  type `0x28`, `0x29` or `0x2a`.

| Kind | Execute | Weight | Phase | Orders | Trigger |
|---|---|---|---|---|---|
| 11 | `FUN_004c8b30` | 30 | `0x3f8` | Move, Create Fleet | `FUN_004c8e90` moves fleet members flagged `0x1000`; `FUN_004c94c0` splits a flagged (`0x4000`) fleet's capital ships into a new fleet, else moves the fleet on when its system record lacks bit 0 or `0x40000`; `FUN_004c9cb0` moves capital ships to a sector system not yet listed in context `+0xd8` |
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

`FUN_004cf290` steps one record of context `+0x11c` per tick through
`FUN_00474390` (fleet `+0x2c`, target `+0x30`, need `+0x34`):

| State | Function | Order |
|---|---|---|
| 2 | `FUN_004749b0` | Create Fleet from the record's listed members, when its `+0x38` has `0x3000` and none of `0xf0000802` |
| 4 | `FUN_00474c00` | need < 0: `FUN_00475490`, `FUN_00475890` gather nearby members into the fleet (Move) or, with no fleet, a new one at the target (Create Fleet); need > 0: `FUN_00475c70` peels members off (Create Fleet) |
| 5 | `FUN_00474ce0` | Move of the units the fleet's capital ships carry, when `+0x80 > 0` |
| 6 | `FUN_00474ff0` | Move of members flagged `0x600000`, when `+0x38` has `0x4000000` |
| 7 | `FUN_00475330` | Move of the fleet to the target system, unless it is there or either record has bit 1 |

## Regional records and the posture engine (module 8)

`FUN_004cf600` keeps the regional records of context `+0xd8`, creating one
(`FUN_0042f350`) when `+0x204 < 0` and finding one (`FUN_0042f3d0`) when it
is positive, and steps each through `FUN_004774c0`:

- state 1: `FUN_00477a40`, Create Fleet;
- state 3: `FUN_00477e80`, Create Fleet for each negative need counter
  (`FUN_004782a0`, then `FUN_00478690`) and each positive one
  (`FUN_00478aa0`);
- state 5: the posture step `FUN_004adb70` on record `+0x5c`, one of six
  posture kinds (`FUN_004773f0`).

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

The posture's side `+4` is the regional record's `+0x20`, which the fleet
tasks also use as the order owner: 1 Alliance, 2 Empire
(`side-numbering.md`).

## Character tasks (module 12)

Module 12 (`FUN_004c8260`, slot 5 `FUN_004c84a0`) makes the tasks through
`FUN_0042ef50` → `FUN_0042efe0`. `FUN_004c85b0` takes kind 1 or 6 from
context `+0x358`; `FUN_004c86b0` takes kind 3, 4 or 5 from a regional
record's `+0xe0` (list `+0xd8`), which the posture fills, and `+0xe4`
cancels it.

| Kind | Constructor | Class | Order (slot `+0x1c`) |
|---|---|---|---|
| 1 | `FUN_0047a430` | G | General `0x263` |
| 6 | `FUN_00479a60` | G | none (moves only) |
| 3 | `FUN_0047a3c0` | I | Admiral `0x262` |
| 4 | `FUN_0047a000` | I | General `0x263` |
| 5 | `FUN_00479f70` | I | Commander `0x261` |

- **Class G** (`FUN_004bbb00`, base `FUN_004bba10`): state 4
  (`FUN_004bbc50`) takes a free character with no command (no `0x7000000`
  bits); at the target system it issues the kind's command, elsewhere it
  moves the character there (`FUN_004bc040`).
- **Class I** (`FUN_004bc480`, base `FUN_004bc290`): state 3
  (`FUN_004bc5d0`) takes a capital ship from the planner; state 4
  (`FUN_004bca20`) issues the command once the character is aboard, else
  moves the character toward the ship.
- **Release** (state 1 with flag `0x80000000`; `FUN_004bbba0`,
  `FUN_004bc520`): frees the character and issues Command: None when it
  holds a command.

`FUN_004bbf50` builds each command order with the character as its team.

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

`crates/rebellion-core/src/ai.rs` emits four actions, applied by
`apply_ai_actions` in `crates/rebellion-app/src/main.rs`:

| Original order | Port |
|---|---|
| Mission `0x240` | `AIAction::DispatchMission` |
| Build `0x210`-`0x212` | `AIAction::EnqueueProduction` from `evaluate_production`, a heuristic with no module weights, goal token or budget percentages |
| Move `0x201` of a fleet | `AIAction::MoveFleet`, optionally with regiments aboard |
| Move of fighters, regiments alone, characters, the HQ or mission members | none; mission members travel in the mission's own transit |
| Destination `0x214` | none; the port queues at the system |
| Scrap `0x200`, Retire `0x242` | none |
| Rename `0x203` | none |
| Create Fleet `0x270` | none; the AI moves whole fleets |
| Command `0x260`-`0x263` | none; `can_be_commander` only filters |
| Planetary Assault `0x234` | none; carried regiments land on arrival (`troop_transport.rs`, `landing_count`) |
| General Bombardment `0x222`, Target Military Facilities `0x220`, Destroy System `0x223` | none; bombardment is only the player's `PanelAction::OrderBombardment` |
| Abort `0x250` | none |
| (no original order) | `AIAction::DispatchResearch`, the port's own |

## Open

- The destination queries `FUN_00419b80`, `FUN_0041a400`, `FUN_0041a4c0`,
  `FUN_0041ab40` (their flags) and the Create Fleet targets of
  `FUN_004782a0`, `FUN_00478690`, `FUN_00478aa0`.
- Fleet record fields `+0x5c`, `+0x68`, `+0x74`, `+0x80` and system record
  fields `+0x110`, `+0x118`; whether fleet flag `0x10` marks the Death Star.
- Why module 7 scraps fighters when no source is found; module 3's
  Coruscant case.
- Who sets the record flag `0x80000000` that module 13 needs.
- Module 6's pass records `+0x9c`; what character kind 6 is for (Empire
  only per `FUN_004eaaa0`, map-09).
- The character tasks' target pickers (slot `+0x30`, `0x0047a870`,
  `0x00479e30`) and the planner slot `+0x34` behind Abort.

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
`FUN_004bd570.c`, `FUN_004be230.c`, `FUN_0047b250.c`, `FUN_00520b30.c`.
