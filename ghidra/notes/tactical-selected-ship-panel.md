---
title: "Tactical Selected-Ship Panel: Labels, Meters, Art and Contents"
description: "What the capital-ship panel draws (FUN_005e4110, FUN_005e4880, FUN_005e4b40, FUN_005e4a30): four font-8 labels, hull and shield status bars, 96x39 ship art and the contents list, with the A0 pixel checks"
category: "ghidra"
tags: [tactical, panel, meters, labels, names]
---

# Tactical selected-ship panel

Recovered with Ghidra 12.1.4 headless (read-only project copy). Panel-local
coordinates add the right-hand container's origin `(482,28)`
(`FUN_0059cac0`, `tactical-command-assignment.md`) to give the 640x480
screen position.

## Labels

`FUN_005e4880` creates four text objects with
`FUN_00601880(text, x, y, w, h, 0, 0x10)`, font 8 (`FUN_00601c60`), white
`0x02ffffff` (`FUN_00601c90`):

| Box (panel) | Size | Screen top-left | Shows |
|---|---|---|---|
| `(26,7)` | 97x13 | `(508,35)` | the ship's name |
| `(26,52)` | 97x9 | `(508,80)` | "Task Force # N" |
| `(15,182)` | 121x13 | `(497,210)` | the order ("No Orders") |
| `(15,193)` | 121x9 | `(497,221)` | "Tactics: Stand Off" |

A text object sizes itself with DT_CALCRECT (`FUN_00601b80`), draws left and
top aligned, and never wraps (`FUN_00601ce0`, DrawTextA). Font 8 is Arial with
`lfHeight` 12, but A0 measures a 9-pixel em: cap height 7, "No Orders" 41 px
wide (TAC-03-C001). hyp: the port draws Liberation Sans at that measured em.

The name is the battle unit's `+0x144` (`FUN_005b1610`), set by the capital
ship constructor `FUN_005b05c0` from `FUN_004f62d0(strategic object)`: its
own name, so a numbered "Imperial Star Destroyer 1" (`fleet-names.md`).

## Hull and shield meters

`FUN_005e4110` builds two CoolwinStatusBar meters (`FUN_0060e150`) at panel
`(34,81)` and `(103,81)`, each 37x10: screen `(516,109)` and `(585,109)`.
The fill brush is `0x02ff0000` (blue) and the background `0x020000ff` (red).
`FUN_0060e460` paints the background rectangle, then a fill
`width * value / range` wide. A red meter is an empty one, not a side's
colour.

The refresh `FUN_005e4b40` (not defined by Ghidra; recovered at
`0x005e4b40`, its meter writes at `0x005e50ce`) sets them through
`FUN_0060e440`:

- left (`+0xa0`): hull, `FUN_005b1640` (`+0x3cc / +0x3c8` through
  `FUN_005b1770`);
- right (`+0xa4`): shields, `FUN_005b1650` (`+0x3c0 / +0x3bc`, 0.0 when the
  maximum is 0, `_DAT_0066c4f0`);
- each fraction times 100.0 (`0x0066d6d8`), truncated (`_ftol`).

Pixel checks on A0: TAC-03-C001, C002 (a Carrack at 60% hull fills 22 of 37
px), C003 (20%, 7 px), C006, TAC-01-C009.

## Ship art

`FUN_005e4b40` draws the class's 96x39 art at panel `(0x1a,0x11)`, screen
`(508,45)`.

## Contents monitor

`FUN_005e4a30` builds a list at panel `(13,126)`, 124x45, with the shared
scroll bar (`FUN_0060a560`, RCDATA `0x546`); `FUN_005e4b40` fills it (list
at `+0xb8`). The original lists regiments as well as characters (Yaga
Minor: three Stormtrooper Regiments beside Vader). port: the port lists
characters only; which units and in what order is an open question.

## Fighter-group panel

Panel id `0x17`: `FUN_005e1170` builds it, `FUN_005e1fd0` creates its
labels and `FUN_005e16a0` refreshes it. A0 TAC-03-C019 shows "Red Group",
"No Orders", "Tactics: Stand Off" at the top and "Fighters: 6 72/72" under
the art. Labels, count line, art and buttons:
`tactical-fighter-group-panel.md`. `FUN_005e7030`/`FUN_005e7800` build panel
`0x16`, not the fighter-group panel.

## Supporting decompiles

`FUN_005e4b40`, `FUN_0060e440`, `FUN_005b1640`, `FUN_005b1650`,
`FUN_005b1610`, `FUN_0060e460`, `FUN_0060e2d0`, `FUN_0060e340`,
`FUN_005b1770`, `FUN_0059cac0`, `FUN_005c5520`, `FUN_005c55a0`.
