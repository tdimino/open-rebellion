---
title: "Agent Advice menu and flag"
description: "The Agent menu items 0x110-0x11e, the advice on/off flag, its difficulty default, and the speed slowdown on Advice category view"
category: "ghidra"
created: 2026-10-06
updated: 2026-10-06
tags: [agent-menu, advice, difficulty, speed, accelerator]
---

# Agent Advice menu and flag

> **Correction (2026-10-06, checked against the decompiles).** A module's
> state 2 means *suspended*: `FUN_004cc990` (VT[4]) readies only a module
> whose state is neither 1 nor 2, so a state-2 module never runs, and
> `FUN_00439950` starts every module in state 2, so all start **off**.
> `FUN_00439e30` returns 1 when the state is not 2, and `FUN_00487900`
> checks the menu item then: a check means the automation runs.
> `FUN_00439d60` turns a module **on** (2 → 0, counter +1) with bit `0x10`
> (Garrisons) or `0x20` (Production), or `0x1000000` once the counter
> passes 3, and **off** (→ 2, counter −1) with `0x100` or `0x200`. Tables
> below that label those bits or states the other way round are wrong.

The Agent menu is built from STRATEGY RT_RCDATA list `0xdead`, items
`0x110..0x11e`. The accelerator table in `FUN_00422ce0` maps Alt-key combos
to timer IDs `0xbc1..0xbd7`, which fire the corresponding command IDs.
`FUN_00486fb0.c` dispatches them. All are gated by `bVar25` (game active) and
mode 1 (`param_1[0x30] == 1`).

## Menu items

| Command | Alt | Timer | Handler chain | Action |
|---------|-----|-------|---------------|--------|
| `0x110` | B | `0xbc1` | FUN_00439d10 → FUN_0049e360(+0x184, 2) | hyp: advisor view "Both" |
| `0x111` | T | `0xbc2` | FUN_00439d10 → FUN_0049e360(+0x184, 3) | hyp: advisor view "Toggle" |
| `0x112` | F | `0xbc3` | FUN_00439d10 → FUN_0049e360(+0x184, 1) | hyp: advisor view "Friendly only" |
| `0x113` | O | `0xbc4` | FUN_0041d7f0 (with target) | Open display (encyclopedia) for target |
| `0x114` | H | `0xbd7` | FUN_0041d7f0 + FUN_004ece30 | Open display (help encyclopedia) |
| `0x115` | G | `0xbc5` | FUN_00439d60 (node 0x15, mask 0x100/0x10) | Manage Garrisons (module 0x15) |
| `0x116` | U | `0xbc6` | FUN_00439d60 (node 0x14, mask 0x200/0x20) | Manage Production (module 0x14) |
| `0x119` | — | — | FUN_00439e80 (no-op) | No-op (separator or disabled) |
| `0x11b` | V | `0xbc8` | FUN_00439e80 → DAT_006b28b0 ^= 0x1000 | hyp: View advice messages toggle |
| `0x11c` | — | — | FUN_0041d770(1, 0x79) | Open Message Index ("All Messages") |
| `0x11e` | A | `0xbc9` | FUN_00439e80 → DAT_006b28b0 ^= 0x8000 | hyp: Agent advice on/off toggle |

Items `0x117`, `0x118`, `0x11a`, `0x11d` are not in the dispatch table; they
may be separators or disabled entries. `0x11c` has no accelerator binding in
the decompiled section.

## Accelerator routing (`FUN_00422ce0.c:1316-1568`)

The WM_TIMER handler (message `0x113`) dispatches virtual-key → timer-id
pairs. Timer ids `0xbc1..0xbc9` and `0xbd7` are killed, then PostMessage sends
the corresponding command to the galaxy view:

```
0xbc1 → 0x110  (Alt+B)     0xbc5 → 0x115  (Alt+G)
0xbc2 → 0x111  (Alt+T)     0xbc6 → 0x116  (Alt+U)
0xbc3 → 0x112  (Alt+F)     0xbc8 → 0x11b  (Alt+V)
0xbc4 → 0x113  (Alt+O)     0xbc9 → 0x11e  (Alt+A)
0xbd7 → 0x114  (Alt+H)
```

Each fires only when `bVar25` (game active) and `param_1[0x30] == 1`
(strategy mode).

## Advisor view cycling (`FUN_0049e360`)

`FUN_00439d10.c` calls `FUN_0049e360(this + 0x184, param)` where param
encodes the advisor frame state:

| param | Meaning (hyp) |
|-------|---------------|
| 1 | Friendly advisors only |
| 2 | Both sides' advisors |
| 3 | Toggle between views |

`FUN_0049e360` stores the difficulty-related value at `+0x34` and resets
`+0x3c` and `+0x40` via `FUN_004ece80`. The exact mapping of `+0x184` to
visible advisor characters is untraced.

## Galaxy overlay toggles (`FUN_00439d60`)

For 0x115 and 0x116, the handler finds a node in the galaxy view's `+0x18`
list (node ids `0x15` and `0x14` respectively) and toggles its `+0x1c` state
between 0 and 2:

- State 2 → 0: shows the overlay, sets counter `+0x160` += 1. If the counter
  exceeds 3, `DAT_006b28b0 |= 0x1000000`; otherwise `DAT_006b28b0 |= uVar4`
  (`0x10` for 0x115, `0x20` for 0x116).
- State 0 → 2: hides the overlay, `DAT_006b28b0 |= param_1` (`0x100` for
  0x115, `0x200` for 0x116), counter -= 1.

hyp: nodes 0x15 and 0x14 are the galaxy map's unit/fleet and reachability
overlays.

## The advice flag (`DAT_006b28b0`)

`FUN_00439e80` handles three commands:

```c
void FUN_00439e80(int param_1) {
    if (param_1 != 0x119) {
        if (param_1 == 0x11b) {
            DAT_006b28b0 = DAT_006b28b0 ^ 0x1000;
        } else if (param_1 == 0x11e) {
            DAT_006b28b0 = DAT_006b28b0 ^ 0x8000;
        }
    }
}
```

- `0x119`: no-op.
- `0x11b` (Alt+V): toggles bit `0x1000` in `DAT_006b28b0`.
- `0x11e` (Alt+A): toggles bit `0x8000` in `DAT_006b28b0`.

`0x11e` (Alt+A) is Agent Advice (TEXTSTRA 12574). Bit `0x8000` set means
advice is **off**: `FUN_00487900` checks 0x11e when it is clear, and
`FUN_00439320` sets it unless the mode is 1 or 4 (`agent-menu.md`).

hyp: `0x11b` (Alt+V) is a secondary toggle, possibly "Verbose" or "View"
advice in the message index. Bit `0x1000` may gate the visual presentation
or notification of advice messages separately from their generation.

## Difficulty default

The task specifies: Easy = on, Medium/Hard = off. The initialization of
`DAT_006b28b0` is in the new-game setup path. hyp: the game-start routine sets
bit `0x8000` of `DAT_006b28b0` when difficulty is Easy (0 or 1) and clears it
for Medium (2) and Hard (3). The exact initializer is untraced; tracing it
requires following the new-game factory through `FUN_004878f0` or
`FUN_00487eb0`.

## Save/load of the flag

hyp: `DAT_006b28b0` is a global options word saved as part of the game state.
The save path would include it alongside speed, side, and other global flags.
The exact save/load offset is untraced.

## Speed hold for Advice and the Encyclopedia

Showing the Message Index's Advice category (button `0x82`) drops the rate
to Very Slow so the player can read; hiding it puts the rate back:

```
Show Advice:   FUN_0041d3f0 -> FUN_004369f0 -> FUN_00487ff0(+0xc, 1)
Hide / other:  FUN_0041d410 -> FUN_00436a00 -> FUN_00487ff0(+0xc, 0)
Index close:   FUN_0046a6a0 -> FUN_0041d410
```

`FUN_004697b0` calls the show thunk on category `0x82` and the hide thunk on
every other category. The Encyclopedia holds the speed the same way: its
layout `FUN_0045ddc0` ends with `FUN_0041d3f0()` (line 570) and its teardown
`FUN_00460070` calls `FUN_0041d410()`.

### `FUN_00487ff0` and `FUN_00487eb0`

The game object keeps the speed index at `+0x54` and the saved index at
`+0x58` (0 means none saved). The index is 1 to 4 and has no Paused value:
`FUN_00487eb0(index)` stores it at `+0x54` and posts message `0x280` with the
rate 600, 60, 12 or 4 (Very Slow, Slow, Medium, Fast); any other index
becomes rate 60 with index 1.

- Show (`param_1 != 0`) with nothing saved: `+0x58 = +0x54`, then
  `FUN_00487eb0(this, 1)`. A second show changes nothing.
- Hide (`param_1 == 0`) with a saved index: `FUN_00487eb0(this, +0x58)`
  with the saved index as stored, then `+0x58 = 0`. Nothing saved: no-op.

Pause is not a speed. It is the stop day at `DAT_006b1218+0x10`
(`FUN_0041d2f0` -> `FUN_00436890` -> `FUN_0041dba0` -> `FUN_0041e290`, which
also calls `FUN_00401930` -> `FUN_00415e60`, the "Resume Game Play?" alert).
`FUN_00487eb0` never touches the stop day, so showing or hiding Advice
neither raises nor lifts a pause: a game held at the stop day stays held, and
hiding raises no alert.

### What sets the speed or the pause

A complete reference scan of `REBEXE.EXE` finds only these callers.

| Effect | Callers |
|---|---|
| Pause (`FUN_0041d2f0`) | `FUN_00486fb0` (the Game Speed menu's Pause item), `FUN_00485030` (game-message type `0x15` with a non-zero `+0x20`; it also stores the day at `+0x48`), `FUN_00406b80` (creating the Game Options window) |
| Speed index (`FUN_00487eb0`) | `FUN_00486fb0` (items 1 to 4), `FUN_00487ff0` (the Advice and Encyclopedia hold), `FUN_00486490` (index 2, Slow, after the `FUN_00439950` check; reached by no direct call) |

No message-manager path (`FUN_0048a060` and the posters) and no Game Options
control appears: a message posted to the Message Index, a kind-6 Death Star
message (`0x6f`) and an Operation Report change neither the speed nor the
pause. The original has no option for auto-pause or auto-slow on an alert
(`game-options.md` lists every control).

### Port

`hold_speed_for_advice` (`crates/rebellion-render/src/game_speed.rs`) saves
`clock.speed` once on show and drops to Very Slow, and on hide restores it
with `GameClock::set_speed`; neither direction touches the stop day. The port
models "no speed chosen yet" as `GameSpeed::Paused` with no stop day, the
index 0 that `FUN_004861b0` builds at `+0x54`. Since 0 is also `+0x58`'s empty
value, a show from it saves nothing and still drops to Very Slow, and the hide
restores nothing, so a speed chosen during the hold stays.
`hold_speed_while_open` applies the same hold to the Encyclopedia on its open
and close edges, through the one saved slot that `+0x58` is in the original.

## Cited decompiles

- `FUN_00486fb0.c` — Command dispatcher (0x110-0x11e cases)
- `FUN_00422ce0.c` — Accelerator table and timer routing
- `FUN_00439d10.c` — Advisor view cycling (0x110-0x112)
- `FUN_00439d60.c` — Galaxy overlay toggle (0x115-0x116)
- `FUN_00439e80.c` — Flag toggle (0x119, 0x11b, 0x11e)
- `FUN_0049e360.c` — Difficulty setter
- `FUN_004697b0.c` — Category selection, advice slowdown trigger
- `FUN_0041d3f0.c` — Advice slow: thunk → FUN_004369f0
- `FUN_0041d410.c` — Advice restore: thunk → FUN_00436a00
- `FUN_00487eb0.c`, `FUN_00487ff0.c`, `FUN_00486490.c` — speed index and hold
- `FUN_00485030.c`, `FUN_00486fb0.c`, `FUN_0041d2f0.c`, `FUN_00415e60.c` — pause callers and alert
- `FUN_0045ddc0.c`, `FUN_00460070.c` — Encyclopedia speed hold and release
- `FUN_004369f0.c` — Calls FUN_00487ff0(+0xc, 1)
- `FUN_00436a00.c` — Calls FUN_00487ff0(+0xc, 0)
- `FUN_00487ff0.c` — Speed save/restore

## Open

- The exact TEXTSTRA label ids for each menu item 0x110-0x11e. These are in the
  STRATEGY RT_RCDATA records for list `0xdead`, extractable via `dat-dumper`.
- The difficulty default initializer for `DAT_006b28b0`: trace the new-game
  factory to confirm bit `0x8000` is set only on Easy.
- The save/load path for `DAT_006b28b0`: find it in the save-game serializer.
- Whether `0x11b` (bit `0x1000`) gates advice message display vs. generation.
