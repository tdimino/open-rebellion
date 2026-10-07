---
title: "The Status window"
description: "Window type 0x1a (FUN_00442d70): the object menu's Status command (0x103), its generic 379 by 272 dialog, its per-family fillers, and the Character Status rows"
---

# The Status window

Recovered 2026-10-06 from REBEXE.EXE with Homebrew Ghidra 12.1.4 (headless,
read-only, on a copy of the project), with STRATEGY.DLL's bitmaps, TEXTSTRA
and the manual (p. 101, Fig. 3.46; p. 100). Every function named here has a
`FUN_<address>.c` note in this directory; the vtable slots `FUN_004443a0`
and `FUN_0044b250` are saved under those names.

## Opening

- The object menu's Status item is order kind `0x103` (TEXTSTRA 12293).
  `FUN_0051d990` adds it to every menu, enabled for a single selection that
  is not a system (`object-popup-menu.md`). Its factory `FUN_0043e710`
  (registered by `FUN_0041e580`) builds `FUN_0043e6c0`, vtable `0x006596d8`,
  whose kind slot returns `0x103`.
- The galaxy view's command `0xbbf` runs it through `FUN_0041cdf0` →
  `FUN_00436020` → `FUN_00486fb0`. For `0x100` and `0x103` that resolves the
  selection's id: Encyclopedia goes to `FUN_0041d6b0`, Status to
  `FUN_0041d7f0` → `FUN_0042a440`.
- `FUN_0042a440` posts `0x409` (close menus), then `0x468` with window type
  `0x1a` and a copy of the id.
- The galaxy view's `0x468` handler (`FUN_00422ce0`) drains pending messages,
  then for type `0x1a` builds the window only when the id is known to the
  player (`FUN_004f2d10` on the view's side `+0x9c`) or is the empty id. It
  allocates `0x1d8` bytes, calls `FUN_00442d70`, shows it, adds it to the
  view's children and gives it the focus. This is the move confirmation's
  path (type `0x10`).

## The window (`FUN_00442d70`, vtable `0x00659c18`)

`FUN_00606380(.., 0x17b, 0x110, ..)`: 379 by 272. `FUN_00606980` places it
from the galaxy view's rectangle (`+0xcc..+0xd8`). The subject id is `+0x124`,
the player's view of the object `+0x12c` (`FUN_004f2d10`), and GOKRES is
`+0x1d4`.

### Create (slot 14, `FUN_00443130`)

- **Background** (STRATEGY, module 7), by the player's side `+0x9c` and the
  object's side bits `(+0x24 >> 6) & 3`; the second number is for a player
  who is not side 1:

  | Object | Side-1 player | Other player |
  |---|---|---|
  | side 1 | 11554 (`0x2d22`) | 11557 |
  | side 2 | 11555 | 11558 |
  | other | 11556 | 11559 |
  | the empty id or the galaxy (`0xf1`) | 11554 | 11558 |

  All six are 379 by 272.
- **List** `+0x118` (`FUN_00607ea0`): at (18, 47), 208 by 204, with a
  vertical scroll bar. The rows are drawn into one bitmap 207 (`0xcf`) wide.
- **Title** `+0x130`: at (15, 18), 211 by 18, font 5, white, format `0x21`
  (one line, centred).
- **Name** `+0x198`: at (242, 137), 130 by 44, font 5, white, format `0x11`
  (centred, word-wrapped). `FUN_0044a210` fills it: the object's name
  (`FUN_004f62d0`), TEXTSTRA 12563 "Galaxy Overview" for the galaxy, 12564
  "Objectives" for the empty id.
- **Picture**: by family, `FUN_0042c3b0(gokres, id, object, 1, 1)` (the
  portrait, with its marks); for a mission (`0x50..0x7f`), GOKRES
  `class +0x30 & 0xfff`, plus `0x1000` unless its side bits are 1; for a
  production manager, GOKRES 263 (`0xa0..0xa1`), 262 (`0xa2..0xa3`) or 264;
  STRATEGY 11590 (`0x2d46`) for the galaxy and the empty id.
  It is blitted keyed (`FUN_005fd0f0`) centred on (307, 64).
- **Rows**: by the id's family:

  | Family | Filler |
  |---|---|
  | `0x08..0x0f` fleet | `FUN_00449200` |
  | `0x14..0x1b` capital ship | `FUN_00446fd0` |
  | `0x30..0x3b` character | `FUN_004486f0` |
  | `0x20..0x27` facilities | `FUN_00444e20` |
  | `0x1c..0x1f` fighters | `FUN_004444c0` |
  | `0x10..0x13` regiment | `FUN_00445280` |
  | `0x28..0x2f` facilities | `FUN_00446c50` |
  | `0x90..0x97` system | `FUN_00446250` |
  | `0x50..0x7f` mission, not hidden (`FUN_00520b70`) | `FUN_00445c60` |
  | `0x3c..0x3f` special force | `FUN_00445780` |
  | `0xa0..0xaf` production manager | `FUN_0044acd0` |
  | `0xf1` galaxy, or the empty id | lists, `FUN_0044b2d0` (side 1) / `FUN_0044bb70` |

  Each filler appends a label to one list and a value to the other
  (`FUN_0044b1d0(row, text, list)`). `FUN_00449e00` lays them out in two
  columns: labels at x 0, values at x 103, each 103 wide, font 10, white,
  format `0x10` (word-wrapped). A row is as tall as its taller cell, and the
  value is pushed down to the label's bottom when the label is taller.
  `FUN_0044a050` draws them.

### Controls (slot 16, `FUN_00443020`; slot 18, `FUN_004443a0`)

- `0x66` at (258, 218), STRATEGY 11552/11553, help `0x1505`: closes the
  window and opens the Encyclopedia on the object (`FUN_0041d6b0`). It is
  disabled, drawing 11612 (`0x2d5c`), for the galaxy and the empty id.
- `0x65` at (324, 218), STRATEGY 10370/10371, help `0x1954`: closes.
- Message `0x40d` closes (slot 5, `FUN_0044b250`); closing posts `0x467` to
  the galaxy view.

No key slot is overridden; the base dialog's keys are not traced.

## Character Status (`FUN_004486f0`)

The title is TEXTSTRA 34595 "Character Status". The rows, in order:

| Label | Value |
|---|---|
| 34596 "Commanding:" | the name of the object at `+0xa8`, else 34644 "None" |
| "Attached: " (see below) | the name of the character's container `+0x1c` (`FUN_0044a2e0`) |
| 34406 "Status:" | the first that holds: `+0x50` bit 4 (en route) 34628 "Enroute"; `+0xac` bit 0 34630 "Captured"; `+0x94` non-zero 34631 "Injured"; a mission at its key `+0x68` that is not hidden (`FUN_00520b70`) 34629 "On Mission"; else 34632 "Awaiting Orders" |
| 34662 "ETA Destination:" | only when en route: 14356 "Day " and the arrival day `+0x44` (`FUN_004fd2b0` → `FUN_005560b0`) |
| 34597 "Force Ranking:" | by the Force value `+0x8c`: below 10 34644 "None", below 20 34633 "Novice", below 80 34640 "Trainee", below 100 34641 "Jedi Student", below 120 34642 "Jedi Knight", else 34643 "Jedi Master" |
| 34598 "Diplomacy Rating:" | `+0x7c` (slot `+0x1dc`) |
| 34599 "Espionage Rating:" | `+0x7e` (`+0x1e0`) |
| 34600 "Combat Rating:" | `+0x86` (`+0x1f0`) |
| 34601 "Leadership Rating:" | `+0x88` (`+0x1f4`) |
| 34608 "R&&D Capabilities" | empty |
| 34609 " Ship Design" | 34646 "Yes" when `+0x80` is non-zero, else 34645 "No" |
| 34610 " Troop Training" | `+0x82`, the same way |
| 34611 " Facility Design" | `+0x84`, the same way |
| 34615 "Possible Command Ranks" | empty |
| 34435 "Admiral:" | class `+0xa8` (`FUN_004ed1c0`), Yes or No |
| 34433 "General:" | class `+0xb0` (`FUN_004ed200`) |
| 34434 "Commander:" | class `+0xac` (`FUN_004ed1e0`) |

"R&&D" is the DrawText escape for "R&D".

### "Attached"

`FUN_0044a2e0` takes the literal `"Attached: "` (`0x006a8738`) when
`FUN_004067d0` returns `DAT_006b0df8`, else TEXTSTRA 34736. `FUN_00406600`
sets that flag from `FUN_00406850`: true when TEXTSTRA.DLL's version resource
FileVersion has zero in the fields after the first dot. The shipped
TEXTSTRA.DLL reads "1.00.00", so the label is the literal, as Fig. 3.46 shows.
This TEXTSTRA has no string 34736.

### Manual cross-check (p. 101, Fig. 3.46; p. 100)

| Claim | Status |
|---|---|
| Right-click a character, choose Status (p. 101, Fig. 3.45) | Confirmed: `0x103` |
| Rows Commanding, Attached, Status, Force Ranking, the four ratings, R&D Capabilities and its three areas, Possible Command Ranks (p. 101, Fig. 3.46) | Confirmed: `FUN_004486f0` |
| "Attached" is the location, or the destination when en route (p. 101) | Confirmed: the container, which for an object in hyperspace is its destination (`FUN_00556390`) |
| Portrait top right, name under it, two buttons below (Fig. 3.46) | Confirmed: (307, 64), `+0x198`, `0x66`/`0x65` |
| The Finder's Display opens "system defenses, fleet or mission" for a character (p. 100) | Confirmed: `FUN_00429440` opens the Missions window (kind 11) for a character on a visible mission, the Defenses window (kind 10) otherwise |
| Ratings range from 0 to 150 or higher (p. 101) | Not checked: data range |

## Port notes

- `status_window.rs` ports the window for characters only. The object menu
  enables Status for a character; the other families keep it disabled
  until their fillers are ported.
- port: the window is drawn above the modeless windows and takes the pointer
  only over itself, as the move confirmation does.
- port: Commanding is always "None": the port keeps no command assignment
  (`+0xa8`).
- port: "Injured" never shows: the port keeps no injury (`+0x94`).
- port: a character is en route while its mission transit lasts
  (`MissionState::en_route`). Its ETA is that transit's arrival day, and
  Attached names the transit's destination.
- port: the Force value is the character's `jedi_level.base`; the port keeps
  no other per-character Force value.
- port: the ratings are each skill's `base`.
- port: the list scrolls with the mouse wheel and draws no scroll bar.
- port: the Encyclopedia button opens the Encyclopedia as the object menu's
  Encyclopedia does, on its index.
- port: Escape closes the window. No key slot is traced.
- The Personnel Finder's Display follows `FUN_00429440`: a character on a
  visible mission opens its system's Missions window with the mission's row
  and the member's tab selected (slot 27, `FUN_004a1e10`).
