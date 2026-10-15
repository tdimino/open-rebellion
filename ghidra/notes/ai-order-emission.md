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
entry's `+0x20`; which mode assigns which kind is untraced.

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
posture change releases all the record's tasks (`FUN_00477190` via
`FUN_004773f0`).

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
   The ship records (context `+0x58`) must exist; a flagship record with `+0x24`
   bit `0x800` is skipped (hyp: no writer found for that bit).

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
  player. Each main-loop pass makes one call for the current computer player
  (`FUN_0041cf20` → `FUN_004361a0` alternates the sides). `FUN_00485b40`
  state 3 picks the armed module (state `+0x1c` = 1) with the largest `+0x34`
  (a counter that module 12 raises when a task step returned an order before
  finishing), else the next armed module in list order (`FUN_004be980`,
  `FUN_004bede0`), and steps it until its done flag `+0x20` is set. The round
  ends, with a record refresh (`FUN_00418500`, state 4) and one re-arm per call
  (state 5, slot 4 of each module), when a module finishes in a call that
  returned an order, when the context's dirty flag (`+4` bit `0x80000000`, set by `FUN_004184f0`)
  is set, when 10 orders have been counted (`+0x428`; the counter
  `+0x420` is cleared every pass by `FUN_00485de0`, because `+0x424`, written
  only by constructors and the save codec, is 0), or when no armed module
  remains. Module 12's pass returns at most one order and then sets its done
  flag, so each of its passes ends the round it runs in.
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

`crates/rebellion-core/src/ai.rs` emits four actions, applied by
`apply_ai_actions` in `crates/rebellion-app/src/main.rs`:

| Original order | Port |
|---|---|
| Mission `0x240` | `AIAction::DispatchMission` |
| Build `0x210`-`0x212` | `AIAction::EnqueueProduction` from `evaluate_production`, a heuristic with no module weights, goal token or budget percentages |
| Move `0x201` of a fleet | `AIAction::MoveFleet`, optionally with regiments aboard |
| Move of fighters, regiments alone, characters, the HQ or mission members | none; mission members travel in the mission's own transit (the player's character Move is `MissionState::dispatch_move`) |
| Destination `0x214` | none; the port queues at the system |
| Scrap `0x200`, Retire `0x242` | none |
| Rename `0x203` | none |
| Create Fleet `0x270` | none; the AI moves whole fleets |
| Command `0x260`-`0x263` | none: module 12 moves its characters (Move `0x201` of a character, onto a system or a flagship); the port has the Move (`MissionState::dispatch_move`) but no AI action that issues it |
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
- Module 6's pass records `+0x9c`.
- The planner slot `+0x34` behind Abort.
- Which controller mode assigns which posture kind; context `+0xa4`
  (`FUN_004182a0`); the meaning of system record bit `+0x28` `0x2` and of ship
  record `+0x24` bit `0x800` in the character tasks.

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
`FUN_00401d20.c`, `FUN_004021b0.c`, `FUN_00402230.c`, `FUN_004025f0.c`, `FUN_00403460.c`, `FUN_00403750.c`, `FUN_004178f0.c`, `FUN_00417cf0.c`, `FUN_00417fa0.c`, `FUN_004182a0.c`, `FUN_00418500.c`, `FUN_00419a00.c`, `FUN_0041ac80.c`, `FUN_0041b600.c`, `FUN_0041b7b0.c`, `FUN_0041b7e0.c`, `FUN_0041c250.c`, `FUN_0041cf20.c`, `FUN_0042ef90.c`, `FUN_00432120.c`, `FUN_00432350.c`, `FUN_00433e40.c`, `FUN_00435790.c`, `FUN_004357b0.c`, `FUN_004762b0.c`, `FUN_00476d10.c`, `FUN_00477190.c`, `FUN_004773f0.c`, `FUN_00477cb0.c`, `FUN_00478000.c`, `FUN_00478140.c`, `FUN_00479560.c`, `FUN_00479ad0.c`, `FUN_00479c00.c`, `FUN_00479e30.c`, `FUN_0047a050.c`, `FUN_0047a480.c`, `FUN_0047a5b0.c`, `FUN_0047a870.c`, `FUN_0047b360.c`, `FUN_00485b40.c`, `FUN_004add40.c`, `FUN_004adde0.c`, `FUN_004b1800.c`, `FUN_004b72d0.c`, `FUN_004b9290.c`, `FUN_004bc180.c`, `FUN_004bc6d0.c`, `FUN_004be980.c`, `FUN_004bede0.c`, `FUN_004c84d0.c`, `FUN_004c85b0.c`, `FUN_004c8630.c`, `FUN_004c86b0.c`, `FUN_004c8740.c`, `FUN_004cc990.c`, `FUN_004e5440.c`, `FUN_004e57e0.c`, `FUN_004e5870.c`, `FUN_004e62b0.c`, `FUN_004e6c00.c`, `FUN_004e7990.c`, `FUN_004e8940.c`, `FUN_004e9690.c`, `FUN_004ea060.c`, `FUN_004eaaa0.c`, `FUN_004ead50.c`, `FUN_004ed1c0.c`, `FUN_004ed1e0.c`, `FUN_004ed200.c`, `FUN_004eeed0.c`, `FUN_00514a60.c`, `FUN_00526a60.c`, `FUN_00560d50.c`, `FUN_004184f0.c`, `FUN_00431790.c`, `FUN_00479fd0.c`, `FUN_0047a400.c`.
