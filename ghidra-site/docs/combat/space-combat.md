---
title: "Story-Check Chain at MissionMgr +0x58"
description: "FUN_005457f0 → FUN_00549910: the ten MissionMgr story checks, their flag bits at +0x58, the fixed-id accessors and the bit broadcasters; no space-combat resolution lives here"
category: "ghidra"
created: 2026-03-13
updated: 2026-10-14
---

# Story-Check Chain at MissionMgr `+0x58`

The functions from `FUN_005434b0` to `FUN_0054a770` belong to the
MissionMgr. They test where the story characters are and set bits in the
story-flag word at `+0x58`; they apply no damage and resolve no battle.
Every space battle runs the tactical engine
([simulate-runs-tactical-engine.md](https://github.com/tdimino/open-rebellion/blob/main/ghidra/notes/simulate-runs-tactical-engine.md)).
The full story reading, with each bit's handler and mission, is
[story-missions.md](https://github.com/tdimino/open-rebellion/blob/main/ghidra/notes/story-missions.md) and
[story-encounters.md](https://github.com/tdimino/open-rebellion/blob/main/ghidra/notes/story-encounters.md).

## Entry

`FUN_005457f0` is the MissionMgr's hook for event `0x320`. It returns 1
unless `FUN_005438a0` says the changed object is one of the six story
characters (Leia `0x31000241`, Luke `0x32000242`, Han `0x33000243`,
Palpatine `0x34000280`, Vader `0x35000281`, Chewbacca `0x38000343`); then
it returns `FUN_00549910`. The other hooks gated the same way are listed in
`story-encounters.md`.

## The chain (`FUN_00549910`)

Ten checks in fixed order, results ANDed, none short-circuits:

| # | Check | Bit | Story event |
|---|---|---|---|
| 1 | `FUN_005442f0` | 6 (held) | Han captured at Jabba's Palace |
| 2 | `FUN_00544da0` | 12 (held) | Final battle ready |
| 3 | `FUN_00544030` | 2 (pulse) | Luke to Dagobah |
| 4 | `FUN_00544130` | 5 (pulse) | Luke to the Palace |
| 5 | `FUN_005443f0` | 7 (pulse) | Leia to the Palace |
| 6 | `FUN_005444e0` | 8 (pulse) | Chewbacca to the Palace |
| 7 | `FUN_005445d0` | 9 (pulse) | Vader's Pickup |
| 8 | `FUN_00544a20` | 10 (pulse) | Vader to Luke |
| 9 | `FUN_00544be0` | 11 (pulse) | Vader to the Emperor |
| 10 | `FUN_00544fc0` | 13 | Final battle |

Conditions and handlers per bit: `story-missions.md`, "The story-flag word".
Character `+0xac` bit 0 in these checks is the prisoner bit; `+0x50` bit 3
is destroyed.

## Fixed-id accessors

`FUN_005434b0` Luke (`FUN_00507010`), `FUN_00543550` Leia (`FUN_00506fb0`),
`FUN_005435f0` Han (`FUN_00506f50`), `FUN_00543690` Chewbacca
(`FUN_005066a0`, family `0x38`..`0x3b`), `FUN_00543760` Vader
(`FUN_00507070`), `FUN_00543800` Palpatine (`FUN_005070d0`). Each looks up
one literal id and its location.

## Flag setters and broadcasters

One setter per bit (`FUN_00543af0` bit 0, `FUN_00543b60` bit 2,
`FUN_00543bd0`..`FUN_00543fc0` bits 5-14), each `FUN_0053a640` with one
mask. A real change runs the bit's broadcaster (`FUN_0054a140`..
`FUN_0054a770`) and the manager's handler slot (`+0x1c0`..`+0x1ec`).

The broadcasters share one shape. `FUN_0054a1d0` (bit 2): `FUN_0053a610`
with mask `4` compares the view's word, `FUN_004fd600(param_1,
this+0x24 >> 4 & 3)` picks the Alliance (1) or Empire (2) view, and view
vtable `+0x1f4` receives the bit. `FUN_0054a260` (mask `0x20`, `+0x1f8`),
`FUN_0054a380` (mask `0x80`, `+0x200`) and `FUN_0054a410` (mask `0x100`,
`+0x204`) do the same for bits 5, 7 and 8.

## Decompiled files

`FUN_005457f0.c`, `FUN_005438a0.c`, `FUN_00549910.c`; the checks
`FUN_005442f0.c`, `FUN_00544da0.c`, `FUN_00544030.c`, `FUN_00544130.c`,
`FUN_005443f0.c`, `FUN_005444e0.c`, `FUN_005445d0.c`, `FUN_00544a20.c`,
`FUN_00544be0.c`, `FUN_00544fc0.c`; the accessors `FUN_005434b0.c`..
`FUN_00543800.c`; the setters `FUN_00543af0.c`..`FUN_00543fc0.c`; the
broadcasters `FUN_0054a140.c`..`FUN_0054a770.c`; `FUN_004fd600.c`.
