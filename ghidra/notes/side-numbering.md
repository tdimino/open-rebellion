---
title: "Side Numbering: Alliance 1, Empire 2, and the Tactical 0/1"
description: "Every side encoding in REBEXE and its converters: object owner bits (1 Alliance, 2 Empire, 3 Neutral), view numbers, the player and AI session slots, galaxy-view side, and the tactical manager's 0/1 (FUN_0040d660)"
category: "ghidra"
tags: [side, faction, alliance, empire, views, tactical, session]
---

# Side numbering

Answers task-graph node `or-q-side-numbering-hxv`. Every claim below was
read in the `.c` files cited.

## Answer

The strategic game numbers the sides 1 (Alliance) and 2 (Empire)
everywhere. Only the tactical battle uses 0 (Alliance) and 1 (Empire), set
once when a battle starts.

| Encoding | Where | Alliance | Empire | Other values |
|---|---|---|---|---|
| Owner | object `+0x24` bits 6-7 | 1 | 2 | 3 Neutral, 0 unset |
| View | object `+0x24` bits 4-5 | 1 | 2 | 0 master copy |
| Event side mask | object `+0x24` bits 0-3 | 1 | 2 | 3 either side |
| Player side | session `+0xc` → `+0x24` (`FUN_00435fc0`) | 1 | 2 | 0 none |
| Session slots | session `+0x10`, `+0x14` | `+0x10` | `+0x14` | |
| Galaxy view side | galaxy view `+0x9c` | 1 | 2 | |
| Tactical side | tactical manager `+0x9ec` (`FUN_00595c40`) | 0 | 1 | |

**Owner and view.** `FUN_004f8c60` prints an object's owner and view by name:
owner 1 "Alliance", 2 "Empire", 3 "Neutral", anything else "UNKNOWN"; view 1
"Alliance", 2 "Empire", anything else "UNKNOWN". The game start agrees:
`FUN_0051ab20` gives every system with owner 1 the Alliance Army Regiment
(`0x10000002`) and owner 2 the Imperial Army Regiment (`0x10000008`)
(TROOPSD). `FUN_004f6f40` sets the owner, accepting 1 to 3. `FUN_0053a000`
is true for the master copy (bits 4-5 clear), `FUN_0053a010` for a view;
`FUN_00539fd0(object, n)` returns copy `n` (`FUN_005844e0`, 0 to 2).
`observation-levels.md` describes the views.

**Event side mask.** `FUN_005404d0(object, side)` matches when the low four
bits equal `side`, or either is 3.

**Player.** `FUN_00435bc0(session, mode, side)` builds the local player's
object (`FUN_004861b0(side)`) at `+0xc` and at its side's slot, `+0x10` for
side 1 or `+0x14` for side 2. The other slot gets the opponent for the other
side: the computer player (`FUN_004857a0`) when the game mode `+0x1c` is 1,
otherwise a remote player (`FUN_00484e70`). `FUN_004861b0` builds the
Alliance advisor (`FUN_004c27f0`) for side 1 and the Empire advisor
(`FUN_004c0710`) for side 2. `FUN_0041cdb0` returns the player side through
the global `DAT_006b14e4` and `FUN_00435fc0`.

**Tactical.** `FUN_0040b0e0` starts a battle with
`FUN_00595d60(FUN_0040d660(FUN_0041cdb0()), ...)`; `FUN_0040d660(side)` is
`side != 1`. `FUN_00599c90` stores the result at the tactical manager's
`+0x9ec`, and `FUN_00595c40` returns it (`FUN_00599c70`). So the Alliance
player plays tactical side 0 and the Empire player side 1, which the strip
headers confirm: `FUN_0059be80` draws bitmap 1001 (Alliance art) when
`FUN_00595c40()` is 0 (`tactical-group-strips.md`).

## Consequences

- An informant (`FUN_0050d510`) reveals an Alliance-held system to the
  Empire's view and an Empire-held one to the Alliance's
  (`observation-levels.md`).
- At the new-game step `0x18`, `FUN_0051b990` reveals Yavin at level 10 to
  the Empire's view.
- Timer `0x393` reveals Han Solo's system at level 5 to the Empire's view.

## Notes checked

Already consistent: `entity-system.md`, `facility-ownership.md`,
`uprising-incident.md`, `mission-lifecycle.md`, `battle-result-text-and-cadence.md`,
`fleet-finder.md`, `troop-finder.md`, `fleet-window.md`,
`mission-dialog.md`, `droid-advisor-triggers.md`, `tactical-group-strips.md`,
`tactical-fighter-group-panel.md`. Brought in line: `agent-menu.md`,
`ai-behavior-analysis.md`, `economy-systems.md`.

## Open

- Where the galaxy view's `+0x9c` is written.
- Whether the computer player's own code (`FUN_004857a0` and the AI
  modules) ever converts to 0/1.

## Decompiles

`FUN_004f8c60.c`, `FUN_0051ab20.c`, `FUN_004f6f40.c`, `FUN_0053a000.c`,
`FUN_0053a010.c`, `FUN_00539fd0.c`, `FUN_005844e0.c`, `FUN_005404d0.c`,
`FUN_00435bc0.c`, `FUN_004861b0.c`, `FUN_00435fc0.c`, `FUN_00435a40.c`,
`FUN_0041cdb0.c`, `FUN_0040b0e0.c`, `FUN_0040d660.c`, `FUN_00595d60.c`,
`FUN_00599c90.c`, `FUN_00599c70.c`, `FUN_00595c40.c`, `FUN_00597530.c`.
