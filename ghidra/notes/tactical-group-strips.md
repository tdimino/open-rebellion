---
title: "Tactical Group Strips: Task-Force and Fighter-Group Buttons"
description: "Where the tactical window's top strips draw their headers and group buttons (FUN_0059be80, FUN_0059c030), and what a click or a number key on a slot selects (FUN_0059b130, FUN_0059b450, FUN_0059b620, FUN_0059b730)"
category: "ghidra"
tags: [tactical, strip, task-force, fighters, buttons, hotkeys]
---

# Tactical group strips

Answers task-graph node `or-q-task-force-strip-1dy` from this project's decompiles. The Feature Wiki map page `executable/map-16-58ef80` gave the starting addresses; every claim below was read in the `.c` files cited. Coordinates are in the original 640x480 canvas.

## Question

How does the task-force strip draw its slots and headers, and what does clicking a group slot select? The same question for the fighter-group strip. Both factions.

## Answer

Both strips are child buttons of the tactical window (vtable `0x0066bb00`, ctor `FUN_0059a8d0`). Their geometry does not depend on the side; only the header bitmap does.

| Control | Id | Built by | Position (x, y) | Size | Bitmaps |
|---|---|---|---|---|---|
| Task-force header | `0xad01` | `FUN_0059be80` | (9, 1) | 49 x 19 | 1001 (`0x3e9`) Alliance, 1004 (`0x3ec`) Empire |
| Task-force slots 1-8 | `0xad02`-`0xad09` | `FUN_0059be80` | x = 60 + 29 i, y 2 | 25 x 17 | normal 1005, state 4 (pressed) 1006, state 2 (unassigned) 1007 |
| Fighter-group header | `0xad0a` | `FUN_0059c030` | (296, 1) | 49 x 19 | 1008 (`0x3f0`) Alliance, 1010 (`0x3f2`) Empire |
| Fighter-group slots Red, Blue, Green, Gold | `0xad0b`-`0xad0e` | `FUN_0059c030` | x = 347 + 29 i, y 2 | 25 x 15 | normal 1012 + i, state 4 1016 + i, state 2 1020 |

- `FUN_0059be80` loops `x = 0x3c; x += 0x1d` over eight `FUN_005c52e0` buttons of `0x19 x 0x11`; `FUN_0059c030` loops `x = 0x15b; x += 0x1d` while `x < 0x1cf` over four `FUN_00602150` buttons of `0x19 x 0xf`. Both headers pick their bitmap by `FUN_00595c40()` (0 Alliance).
- The squadron bitmaps are 25 x 17 but their buttons are 15 tall, so only the top 15 rows show. A0 `TAC-03-C019` confirms it: at x 390, rows 2-16 match bitmap 1013's rows 0-14 and rows 17-18 show the strip background.
- Each fighter button starts with state bit 8 set (`FUN_006030c0(8)`) and disabled (vtable `+0x10`); the hover text ids are `0xad9d + i`.
- The task-force slots are `FUN_005c52e0` toggle buttons with an alternate image pair, used when a slot is assigned (`FUN_005c5480`).

**Click.** The window's command handler `FUN_0059b130` sends `0xad02`-`0xad09` to `FUN_0059b450` and `0xad0b`-`0xad0e` to `FUN_0059b620`.

- `FUN_0059b620` (fighter groups) reads which of the four groups exist (`FUN_00595bc0` → `FUN_005a0c20`, UI controller `+0x154`). For each existing group, the clicked button posts UI command `0xb + i` (`FUN_00595b10` → `FUN_0059a470` → `FUN_005a0ad0`, which queues a command object, vtable `0x0066bd48`, on the controller at `+0xec`); every other existing button is released (`FUN_006030f0(5)`). So Red = `0xb`, Blue = `0xc`, Green = `0xd`, Gold = `0xe`.
- `FUN_0059b450` (task forces) reads which of the eight exist (`FUN_00595ba0` → `FUN_005a0bf0`, controller `+0x164`). An assigned slot posts command `2 + i` and releases the others. An unassigned slot acts only while `FUN_00595b90` → `FUN_0059f6e0` is true (current mode object `DAT_006bc464`, vtable `+8`, is id `0xe` with the controller flag set, or id `0x10`): it sets the slot's caption (`FUN_00600c40`), switches it to its alternate images (`FUN_005c5480`), presses it (state 5) and posts `2 + i`.

**Keys.** `FUN_0059b730` (key down) maps `1`-`8` to: release every task-force slot (`FUN_0059b430`) and every fighter slot (`FUN_0059b600`), then press `0xad02 + i` through `FUN_0059ca90`; the key-up branch releases it. `F1`-`F8` post camera mode `0xb` with the task-force index when that slot exists (`FUN_005c2990`).

**Slot presence.** A fighter slot shows a group while the controller's
`+0x154 + 4 i` holds its id (`FUN_005a0c20`, `FUN_0059f5f0`); one without
draws state 2 and takes no click (`FUN_0059b620`). `FUN_005a0320` clears
it: from `FUN_005ca530` when the local side's group takes order 2 or 6, and
from `FUN_005a8d30` and `FUN_005aae40` when the group has no squadron left.
`FUN_0059fca0` then redraws the strip (`FUN_005c0e40`). `FUN_0059f680`
gives it back from the trench run's success, state 6 of `FUN_005cfec0`.

## Port

`crates/rebellion-render/src/tactical_view.rs`: `TASK_FORCE_BUTTONS`, `FIGHTER_GROUP_BUTTONS`, `TASK_FORCE_HEADER_POINT`, `FIGHTER_GROUP_HEADER_POINT` and the strip draw. Before this note the port drew both rows at a 26-pixel pitch from x 60 and 330, the task-force header at (10, 1) and the fighter header at (275, 1), and drew the squadron bitmaps 17 rows tall; so a click at (360, 10), the original's Red slot, hit the port's Blue slot.

## Open

- The port's hit test checks the bitmap's opaque pixels; the original's buttons take the whole rectangle (`FUN_00602150`). Not yet compared in play.
- What mode ids `0xe` and `0x10` are, beyond the map page's reading of `0xe` as the own-ship mode.

## Decompiles

`FUN_0059be80.c`, `FUN_0059c030.c`, `FUN_0059b130.c`, `FUN_0059b450.c`, `FUN_0059b620.c`, `FUN_0059b730.c`, `FUN_0059b430.c`, `FUN_0059b600.c`, `FUN_0059ca90.c`, `FUN_00595b10.c`, `FUN_00595b90.c`, `FUN_00595ba0.c`, `FUN_00595bc0.c`, `FUN_0059a470.c`, `FUN_0059a490.c`, `FUN_0059a4b0.c`, `FUN_0059a520.c`, `FUN_005a0bf0.c`, `FUN_005a0c20.c`, `FUN_0059f6e0.c`, `FUN_005a0ad0.c`.
