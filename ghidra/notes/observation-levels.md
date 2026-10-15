---
title: "Observation Levels: What a Side Learns About a System"
description: "FUN_0050d5a0 copies a system's master state into one side's view at observation level 0-12; the level picks which object families and which system fields the side sees. Callers: informants (INFORMTB), espionage, mission ends, the Yavin start, event 0x319 and timer 0x393"
category: "ghidra"
tags: [fog, observation, informant, espionage, views, replicas]
---

# Observation levels

Answers task-graph node `or-q-observation-levels-y93`. The Feature Wiki map page
`executable/map-11-508c53` gave the starting addresses; every claim below was
read in the `.c` files cited.

## Views

Every game object keeps a master copy and one copy per side. `FUN_00539fd0(obj, n)`
returns copy `n` from the object's replica set at `+0x20` (`FUN_005844e0`):
0 is the master, 1 the Alliance's view and 2 the Empire's
(`side-numbering.md`). A view copy stores its view
number in `+0x24` bits 4-5; the object's owner is `+0x24` bits 6-7.
`FUN_0053a010` is true for a view copy.

A side knows only what has been copied into its view. The copy routines pull
from the master:

- `FUN_004f7e30(view, range, ctx)`: for every member of the master whose DatId
  family (`id >> 24`) is in `range`, create or refresh that member's copy for
  this view (`FUN_004f9510`, view number from the caller's `+0x24 >> 4 & 3`);
  then refresh every member the view already holds in that range
  (`FUN_004f7f20`).
- One field-sync routine per system field. Each checks that it runs on a view
  copy whose side the event covers (`FUN_005404d0`), copies the master's value
  when it differs, and calls the view's change slot with old and new values.

| Routine | Field copied |
|---|---|
| `FUN_0050fbf0` | `+0x88` bit 0 (populated) |
| `FUN_0050fc80` | `+0x88` bit 1 |
| `FUN_0050fd10` | `+0x88` bit 2 (uprising in progress) |
| `FUN_005103a0` | `+0x88` bit 13 |
| `FUN_0050f390` | `+0x58` support (0-100) |
| `FUN_0050fa90` | `+0x84` object |
| `FUN_0050f840` | `+0x78` bits 4-5 (uprising side) |
| `FUN_0050f410` | `+0x5c` energy |
| `FUN_0050f490` | `+0x64` raw material |

## FUN_0050d5a0(system, level, side, ctx)

`side` must be 1 or 2, or nothing happens. The function fetches that side's
view of the system, builds up to four DatId family ranges for the system's own
members, one range for the members of each fleet in the side's view of the
system (`FUN_004ffe70`, type 4), and one range for the members of each unit
in those fleets (`FUN_00502db0`), then sets which field groups to
copy. It returns false for a level above 12.

Field groups:

- **base**: `FUN_0050fc80`, `FUN_0050fbf0`, then the generic object sync
  `FUN_004f9ca0`;
- **support**: `FUN_005103a0`, `FUN_0050fa90`, `FUN_0050f390`;
- **uprising**: `FUN_0050fd10`, `FUN_0050f840`;
- **resources**: `FUN_0050f410`, `FUN_0050f490`;
- **full**: the view's slot `+0x24` (replaces the field groups);
- **characters**: `FUN_0050eea0`, over type-3 members (`FUN_004f25c0`)
  whose short `+0x96` is non-zero. It first creates this view's copy of each
  such member of the object's container (`FUN_00504dc0`); then, for each such
  member of the object itself, it creates the copy (`FUN_004f9510`) when the
  member and the character it resolves to (`FUN_00505190`) report the same
  non-null location (`FUN_004f6b70`), and otherwise refreshes it
  (`FUN_004f7f20`). Level 6 also runs it on each unit in the view's fleets.

| Level | Fields | System members (families) | Fleet members | Unit members |
|---|---|---|---|---|
| 0 | none | none | none | none |
| 1 | base, support | none | none | none |
| 2 | base, resources | none | none | none |
| 3 | base | `0x10`-`0x20`, `0x22`-`0x28` | none | none |
| 4 | base | `0xa0`-`0xb0`, `0x28`-`0x30` | none | none |
| 5 | base | `0x08`-`0x10` | all (`1`-`0xff`) | none |
| 6 | base, support, uprising, characters | `0x08`-`0x10` | all | none |
| 7 | base | `0x08`-`0x10` | all | all |
| 8 | full | `0x20`-`0x30`, `0x10`-`0x20`, `0xa0`-`0xb0`, `0x08`-`0x10` | all | `0x10`-`0x20` |
| 9 | base | `0x30`-`0x40` | none | none |
| 10 | full | all | all | all |
| 11 | full | `0x10`-`0x40`, `0xa0`-`0xb0` | none | none |
| 12 | base | `0x20`-`0x30` | none | none |

Ranges are half-open. The family names below come from the encyclopedia's
categories (`crates/rebellion-data/src/encyclopedia_catalog.rs`) and the map
pages: `0x08`-`0x10` fleets, `0x10`-`0x14` troops, `0x14`-`0x20` ships,
`0x20`-`0x30` facilities (shipyards `0x28`-`0x29`, refineries `0x2d`-`0x2e`),
`0x30`-`0x3c` characters, `0x3c`-`0x40` special forces. What `0xa0`-`0xb0`
holds is open (the map pages call `0xa2`-`0xa3` a ship producer).

So level 3 shows troops, ships and part of the facilities; level 5 shows the
fleets and everything in them; level 9 shows characters and special forces;
level 10 shows everything.

## Callers

| Caller | Level | Side | When |
|---|---|---|---|
| `FUN_0050d510` | INFORMTB draw | the holder's opponent | informant incident (bit 17, event `0x38e`; `uprising-incident.md`) |
| `FUN_00573610` | 10 | the mission's side (`+0x24 >> 6 & 3`) | espionage at a system target (`mission-lifecycle.md`) |
| `FUN_00592c80` | 6 | the mission's side | mission end slot at a system target (`mission-lifecycle.md`) |
| `FUN_0055ff60` | 6 | both | only when both arguments' `+0x9c` ids are the null id 2 (`FUN_004ece30` writes it, `FUN_004ece60` tests it) |
| `FUN_0054b040` | 6 | both | at the object's system (family `0x90`-`0x97`) |
| `FUN_005650f0` | 11 | both | event `0x319` at its key's object |
| `FUN_00556c70` | 5 | 2 (Empire) | timer event `0x393` (`FUN_00546fa0`): after creating special force `0x3c000009`, at Han Solo's (`0x33000243`) location when it is a system |
| `FUN_0051b990` | 10 | 2 (Empire) | new-game stage `0x18` (`FUN_0051b940`, run once for each side's view), at Yavin `0x92000121` |

**Informants.** `FUN_0050d510` runs on a master system whose holder (`+0x24`
bits 6-7) is not 3. It draws a level from INFORMTB (`FUN_00559db0`, table
`0x2a`: a random draw plus a GNPRTB offset) and reveals it to the Empire's view
(2) when the Alliance (1) holds it, otherwise to the Alliance's view (1). The shipped INFORMTB maps draws 1-4 to
levels 1-4 and draws 5-8 to levels 7-10, so an informant never yields 5 or 6.

## Port

`crates/rebellion-core/src/fog.rs` keeps one set of visible systems per side
and a per-system list of known facilities; it has no levels and no per-field
views. The table above is what a port of the original's partial knowledge
would follow.

## Open

- What `+0x88` bits 1 and 13 and the `+0x84` object mean.
- What families `0xa0`-`0xb0` hold.
- What `FUN_0055ff60` and `FUN_0054b040` are for.

## Decompiles

`FUN_0050d5a0.c`, `FUN_0050d510.c`, `FUN_00559db0.c`, `FUN_00539fd0.c`,
`FUN_0053a010.c`, `FUN_005404d0.c`, `FUN_004f7e30.c`, `FUN_004f9ca0.c`,
`FUN_0050eea0.c`, `FUN_0050fbf0.c`, `FUN_0050fc80.c`, `FUN_0050fd10.c`,
`FUN_005103a0.c`, `FUN_0050f390.c`, `FUN_0050fa90.c`, `FUN_0050f840.c`,
`FUN_0050f410.c`, `FUN_0050f490.c`, `FUN_00573610.c`, `FUN_00592c80.c`,
`FUN_0055ff60.c`, `FUN_0054b040.c`, `FUN_005650f0.c`, `FUN_00556c70.c`,
`FUN_0051b990.c`, `FUN_0051b940.c`.
