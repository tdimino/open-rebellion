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

## Speed slowdown for Advice category

When the Message Index shows the Advice category (button `0x82`), the game
drops to Very Slow to let the player read advice at leisure:

```
Category 0x82 (Advice):
    FUN_0041d3f0 → FUN_004369f0 → FUN_00487ff0(+0xc, 1)

All other categories:
    FUN_0041d410 → FUN_00436a00 → FUN_00487ff0(+0xc, 0)

Window close (FUN_0046a6a0):
    FUN_0041d410 → restore
```

### `FUN_00487ff0` (speed save/restore)

The game object's speed state lives at two offsets:

- `+0x54`: current speed index (0 = Paused, 1 = Very Slow, 2 = Slow, 3 = Normal, 4 = Fast).
- `+0x58`: saved speed (non-zero only while advice is being shown).

When `param_1 = 1` (slow down):
1. If `+0x58 == 0`: save `+0x54` → `+0x58`, set speed to 1 (Very Slow) via
   `FUN_00487eb0(this, 1)`.
2. If `+0x58 != 0`: already saved; no-op.

When `param_1 = 0` (restore):
1. If `+0x58 != 0`: restore speed from `+0x58` via
   `FUN_00487eb0(this, +0x58 - 1)`, clear `+0x58`.
2. If `+0x58 == 0`: nothing saved; no-op.

The `- 1` on restore accounts for the 1-based encoding of the saved speed
(0 means "no save active"). The guard prevents double-saves and double-restores.

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
