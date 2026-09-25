# Game Speed control recovery

Status: static recovery for `CMD-05`, not acceptance. No cell is promoted.

The original command center has no speed buttons. The manual's first
mini-mission (printed page 23) says to right-click the **Game Speed** control at
the top of the screen and pick Pause, Very Slow, Slow, Medium, or Fast from the
menu that drops down. Pausing shows the "Resume Game Play?" alert (Fig. 2.4),
and play resumes when the player clicks its checkmark. The keyboard reference
(printed page 64) adds Alt+NumPad+ and Alt+NumPad− to step the speed and
Alt+P to pause. Fig. 3.4 labels the same top-left readout "Number of Days Since
Game Started", and the executable confirms that the day readout is the Game
Speed control.

The surface ledger's `very fast` state does not exist. The original has four
speeds plus Pause.

## Executable path

Decompiled from the saved project with Ghidra 12.1.3 in a read-only scratch
copy. New decompilations fill their previously empty `ghidra/notes/`
placeholders. The scalar, operand, vtable, and caller scripts used for the
trace are in `ghidra/scripts/`.

| Function | Role |
|---|---|
| `FUN_00422ce0` lines 185–258 | Builds the top status strip. The day readout sits at Alliance `(103,21)–(185,33)` and Empire `(499,19)–(580,31)`, in faction text color `0x20000ff` / `0x200ff00`. Three further 63x17 readouts show raw materials, refined materials, and maintenance capacity. |
| `FUN_00422ce0` line 1039 | On `WM_RBUTTONUP`, when the game state from `FUN_004fcee0` is at least 2, a hit inside the day-readout rectangle calls `FUN_0042d190`. Droid hits route to `FUN_0042d050` first. |
| `FUN_0042d190` | Builds the five-item speed menu through `FUN_00442590(item, module 7, flags 0, x, y, owner, text, highlight 0x2ffffff, disabled 0x2808080)` and opens it at the cursor with `FUN_00442380`. Alliance items are `0x20..0x24`; Empire items are `0x25..0x29`. |
| `FUN_00442790` | Reads each item from a 26-byte `RT_RCDATA` record in STRATEGY.DLL (module 7). |
| `FUN_004ac730` → `FUN_0041cdf0` → `FUN_00436020` → `FUN_00486fb0` | Menu selection path. `FUN_00486fb0` maps items `0x20/0x25` to `FUN_0041d2f0(0)` (Pause) and `0x21..0x24` / `0x26..0x29` to `FUN_00487eb0(this, 1..4)`. |
| `FUN_00487eb0` | Stores the speed index at `this+0x54` and sends game message `0x280` with the rate value. Very Slow = 600, Slow = 60, Medium = 12, Fast = 4. Out-of-range input falls back to 60 with index 1. |
| `FUN_0051f4b0`, `FUN_00628e60`, vtable `0x0066f488` slot 8 | The `0x280` message class. It resolves the target object and calls `FUN_005305e0`, which stores the non-zero value at `+0xc4`. `FUN_00531330` serializes that field, so the original save carries the speed. |
| `FUN_0041d2f0` → `FUN_00436890` | Pause takes the larger of both sides' current day plus one, raises the stop day through `FUN_0041dba0` → `FUN_0041e290`, and notifies both sides' vtable `+0x174`. It does not touch the speed index at `+0x54`. |

## STRATEGY `RT_RCDATA` item records

Each record is 13 little-endian words: item, parent, key, 0, 0, TEXTSTRA
label, 0, icon, highlighted icon, 0, 0, 2, module 7.

| Item (Alliance / Empire) | Label (TEXTSTRA) | Alliance icon | Empire icon | Result |
|---|---|---|---|---|
| `0x20` / `0x25` | 34304 "Pause" | 11580 | 11584 | `FUN_0041d2f0(0)` |
| `0x21` / `0x26` | 34305 "Very Slow" | 11581 | 11585 | index 1, value 600 |
| `0x22` / `0x27` | 34306 "Slow" | 11582 | 11586 | index 2, value 60 |
| `0x23` / `0x28` | 34307 "Medium" | 11583 | 11587 | index 3, value 12 |
| `0x24` / `0x29` | 34308 "Fast" | 11588 | 11589 | index 4, value 4 |

The icons are 16x10 8-bit STRATEGY bitmaps. The builder passes flags 0, so the
menu marks no current speed.

## Game Menu Window and day readout

| Function | Recovered rule |
|---|---|
| `FUN_00442860` | Creates the "Game Menu Window" with STRATEGY frame tiles `0x2774..0x277b` (10100–10107, the same eight tiles as the GID menu) and Alliance / Empire submenu arrows `0x2785/0x2786` / `0x2790/0x2791`. It moves the menu up by its height when it would cross the owner's bottom edge, and left by its width when it would cross the right edge. |
| `FUN_00442a80` | The icon column is the widest icon (`FUN_004abf60`). Rows start at y = 2, and the window height is the sum of row heights + 2. |
| `FUN_004abb80`, `FUN_004abbf0`, `FUN_004abc70` | Row width = icon column + 12 + text width. Row height = max(icon + 4, text + 4). The icon is drawn at x = 6; the text at x = icon column + 6, centered vertically in the row. |
| `FUN_004ab840`, `FUN_004abe10` | Items default to white / black / gray `0x2808080`. `FUN_0042d190` supplies the faction text color, white highlight, and gray disabled color. |
| `FUN_00601ce0` | Text objects draw with `DrawTextA`, a transparent background, and their stored color. The day readout uses format `1` (`DT_CENTER`) plus `DT_NOCLIP`; the resource readouts use `0x26`. |
| `FUN_00601b30` | Anchors the day text at Alliance `(104,18)` and Empire `(500,18)`. The `(103,21)–(185,33)` / `(499,19)–(580,31)` rectangles are only the right-click hit areas. |
| `FUN_0060eed0` | The game font table. Entry 10, used by the day readout, is 14 px at weight 400. The face name is copied from the table; "Arial" is the only font face string in `REBEXE.EXE`. |

## Pause and the "Resume Game Play?" alert

| Function | Recovered rule |
|---|---|
| `FUN_0041e290` | Raises the stop day at `DAT_006b1218+0x10` only when the new value is larger, then calls `FUN_00401930`. A second pause for the same day neither moves the stop nor reopens the alert. |
| `FUN_0041dff0` state 2 → `FUN_0041e300` | The simulation state machine holds while the current day equals the stop day. This is the only pause mechanism; the rate at `+0xc4` is unchanged. |
| `FUN_00415e60` | Opens the alert through `FUN_00401150(0x46c, 0, 0x1203, 0x190d)` unless the game state is `0xb` or `0xc` or the suppression flag `DAT_006b14bc` is set. |
| `FUN_00417020` / `FUN_00416f80` | Builds a 412x176 child window (`0x44000000`) on REBDLOG bitmap `0x297e`, then centers it in the owner with `FUN_005ffeb0` (integer halves, so `(114,152)` in a 640x480 owner). `0x46c` is the reply tag stored in `DAT_006b1470`, not a sound. |
| `FUN_00416de0` | Paints REBDLOG string 4611 "Resume Game Play?" with game-font entry 5 (16 px, weight 700), color `0x2f0fbff`, and `DrawTextA` format `0x25` (centered, vertically centered, single line) in `(43,25)-(368,112)` from `0x00658920`. |
| `FUN_00417150` | Adds the checkmark button at `(176,134)`: normal `0x2981`, pressed `0x2980` (both 57x28), command `0x65`, tooltip string 6413 "Resume". |
| `FUN_00417180`, `FUN_004171a0` | Command `0x65` or the Enter key (`0xd`) calls `FUN_004011a0(1, 0)`, which posts `0x47a`, `0x405(0xd)`, and `0x46a` with the reply tag to the main window. |
| `FUN_004361d0` → `FUN_0041dbf0` → `FUN_0041e2d0` | Each step, when both sides' pause hold at `+0x48` is clear, the stop day is cleared and play continues at the unchanged speed. |

REBDLOG also carries 4614 "Waiting For Opponent To Resume" for multiplayer, and a
two-button background `0x297f` with a cancel button `0x2982` used by other alerts.

The implementation models this directly: `GameClock` stores the stop day,
finishes the current day, and holds there without changing its speed. The alert
opens at once, as `FUN_00401930` calls `FUN_00415e60` immediately after raising
the stop. A speed chosen from the menu while paused changes the rate but does
not resume, because `FUN_00487eb0` changes the rate without clearing the stop
day. Save v14 persists the stop day, so a game saved while paused reloads
paused at its kept speed.

## Speed keys

`FUN_00421c70` loads TEXTCOMM accelerator table 11 into the main window, and
the message loop in `FUN_00413560` translates it. All three entries carry
`FVIRTKEY | FNOINVERT | FALT` (`0x13`).

| Key | Command | `FUN_00422ce0` handler |
|---|---|---|
| Alt+NumPad+ (`0x6b`) | `0xbba` | From speed 1, 2, or 3, selects menu item `0x22`, `0x23`, or `0x24`. Nothing happens at Fast or while paused. |
| Alt+NumPad− (`0x6d`) | `0xbbb` | From speed 2, 3, or 4, selects `0x21`, `0x22`, or `0x23`. Nothing happens at Very Slow or while paused. |
| Alt+P (`0x50`) | `0xbbc` | When the command center's speed field is not 0, selects item `0x20` (Pause). Alt+P does not resume. |

Both keys reuse the Alliance item IDs, which `FUN_00486fb0` accepts for either
side. The handler reads the command center's speed field `param_1[0x45]`; its
writer is not traced, so the claim that it reads 0 while paused rests on the
Alt+P guard.

## Open

- **Wall-clock unit of the rate value.** The reader that turns `+0xc4` into
  day advancement has not been found. A 100 ms unit gives Very Slow ≈ 60 s per
  day, which is consistent with the reported one to two hours per 100 days at
  Very Slow ([campaign testimony](../../../reference/campaign-history/human-campaign-testimony.md)).
  The 500 ms strategic timer (`FUN_00410e90`, timer 400) as the unit would give
  five minutes per day, which the testimony contradicts. This is an inference.
  An A0 timing capture at each speed closes it.
- The Game Menu Window's default font entry (`FUN_00603850`) and its
  background fill. The implementation reuses the GID menu's provisional fill.
  Browser text uses egui's proportional face at the recovered height, because
  Arial is not redistributable.
- The day readout's string format. The implementation shows the bare day
  number, as Fig. 3.4 suggests.
- The handler for the alert's `0x46a` reply, which clears this side's pause
  hold at `+0x48`, is reached through a dispatch table and is not decompiled.
- The speed a new game starts at. The implementation starts paused without
  the alert, and the first menu choice runs the game.
- The "Resume" tooltip's style; the implementation shows no tooltip.
- Whether the menu's Pause item is ignored while paused. Alt+P is, by the
  `FUN_00422ce0` guard. Choosing Pause from the menu while held would move the
  stop one day later through `FUN_0041e290`, so the implementation ignores it
  on every path. This is an inference.
- The source of the day-readout font.
