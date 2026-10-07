---
title: "Message Index rows and selection"
description: "The list control, item fill, selection model, delete semantics, and single-message display mode of the Message Index window"
category: "ghidra"
created: 2026-10-06
updated: 2026-10-06
tags: [message-index, coolDragList, selection, delete, advice-slowdown]
---

# Message Index rows and selection

Extends `message-index-window.md` with the per-row data, selection, delete,
sort, and mode 2 (single-message) display. Functions cited here have `FUN_*.c`
notes in this directory; `FUN_004697b0.c` (fill), `FUN_00468fb0.c` (mode
switch), `FUN_00468ab0.c` (rebuild), and `FUN_00469de0.c` (display) are the
primary sources.

## The list control (`+0x168`)

`FUN_004665f0` (layout) creates the list at (25, 108) with size 368 by 194
(`0x170 x 0xc2`), backed by a 4-column `CoolDragList` at `+0x16c`:

```
FUN_004ad620(local_10, 4);   // CoolDragList with 4 data columns, vtable 0x0065c080
```

The data list (`+0x16c`) holds message nodes; the view list (`+0x168`) is a
`FUN_00607ea0` widget with item size 346 by 21 (`FUN_006082c0` in
`FUN_00468ab0.c`, args `0x15a, 0x15`). A scroll bar (`FUN_0060a490`) at
`+0x94` uses bitmaps `0x299d` and the list's height.

## Item fill (`FUN_00468ab0`)

Called when the list rebuilds (category change, message arrival, mode 1 entry).

1. The old selection is saved. When `param_1 = 0`, only the current `+0x170`
   pointer is kept; when `param_1 = 1`, every item with `+0x3c` bit 0 (selected)
   is saved.
2. The data list is cleared (`FUN_005f5b20`).
3. `FUN_0041d1b0` returns the message manager (`thunk_FUN_00435a40` → the
   singleton at `DAT_006be3b8`). `FUN_00436780` returns the player's message
   list. The loop `thunk_FUN_005f5080 / FUN_005f5c60` iterates every message
   node in list order (arrival order, oldest first).
4. For each node, a list item is built (`FUN_004ad6b0(pvVar5, iVar4)`).
   The category mask at message `+0x34` selects a category icon bitmap
   from STRATEGY module 7, side-dependent:

   | Mask | Alliance icon | Empire icon |
   |---|---|---|
   | `0x001` Popular Support | `0x2a9a` | `0x2a9b` |
   | `0x008` Manufacturing | `0x2a9c` | `0x2a9d` |
   | `0x010` Mission | `0x2a9e` | `0x2a9f` |
   | `0x020` Chat | `0x2aa4` | `0x2aa4` |
   | `0x040` Defense | `0x2ad3` | `0x2ad3` |
   | `0x080` Fleet | `0x2ad4` | `0x2ad5` |
   | `0x004` Resource | `0x2ad6` | `0x2ad6` |
   | `0x100` Conflict | `0x2ad7` | `0x2ad7` |
   | `0x200` Advice | `0x2ad8` | `0x2ad9` |

   When the mask matches no row, no icon is drawn. The background for the row
   pair (normal / selected) is `0x2aa2` (Alliance) or `0x2aa3` (Empire).

5. Each item carries three layout fields:
   - `+0x30` = `0x20` (32): hyp: left pixel offset for text within the row.
   - `+0x34` = `1`: hyp: text line count or row type.
   - `+0x38` = font index: **`0x0d` (13)** for unread (`+0x24` bit 0 clear),
     **`0x0a` (10)** for read (`+0x24` bit 0 set). Unread messages use a
     bolder font.

6. The item is added to the data list (`FUN_005f59f0`). If the node was in
   the saved selection, it is pre-selected (`FUN_00609500`, bit 1).

## Category filter (`FUN_00468f20`)

After fill, `FUN_00468f20` shows or hides each item by its mask:

- The active filter at `+0x158` matches the category button (e.g. `0x80` for
  Fleet). An item is visible when `(+0x158 & item_mask) != 0`.
- **All Messages (0x79)**: `+0x158` is 0, so the mask test always fails.
  The special case `(+0x11c != 0x79 || mask == 0x200)` means that when the
  category is All, Advice (0x200) items are **hidden**. Every other category is
  shown in All.
- If no item is selected after filtering, the first visible item is selected.
- hyp: item visibility is `+0x6c` (1 shown, 0 hidden); the `+0x3c` bit 0 is
  the selection flag.

## Selection in the view (`FUN_00609410`)

`FUN_00609410` builds a result list of selected items by iterating the view
list's `+0xa0` children and copying those with `+0x3c` bit 0 set
(`piVar1[0xf] & 1`). Each copy wraps the list item via `FUN_0060bac0`.

The list control's own click handler is internal to the `CoolDragList` window
procedure (`FUN_006083c0`, vtable `0x0066e0f0`). untraced: single click,
Ctrl-click toggle, Shift-click range. The notification sent to the parent is
`0x29b` for selection change and `0x309` for double-click.

## Double-click: Display Message (0x65)

`FUN_00468fb0(this, 0x65)` transitions from mode 1 (index) to mode 2 (single
message):

1. `FUN_0060a790 / FUN_00609410` copies the selection to a temporary.
2. If the selection is empty, no transition occurs.
3. One item is resolved from the selection via `FUN_0060a860` (find by key).
   Its node pointer is stored at `+0x170`.
4. The Display Message button (command `0x65`) label changes to TEXTSTRA
   `0x8019` ("Message Index"), with bitmaps:
   - Alliance: `0x2884` / `0x2885`
   - Empire: `0x288a` / `0x288b`
5. The list and its controls are hidden; the navigation buttons `0x96` (scroll
   up) and `0x9a` (scroll down) are shown. Delete (`0x91`), navigate (`0x90`),
   Display toggle (`0x97`), Encyclopedia (`0x98`) and Detail (`0x99`) are
   hidden.
6. `FUN_00469de0` lays out the single message.

## Single-message display (`FUN_00469de0`, mode 2)

Receives the message node at `+0x170`. The node's `+0x68` points to its data
record (the message object).

- **Battle reports** (type `0x0f`, vtable slot `+0xc` returns `0x0f`): the
  window saves its state (`DAT_006b2904..DAT_006b2914`) and opens the tactical
  report via `FUN_0042a410`. The message index closes (slot `+0x30`).
- **All other types**: the message text at `+0x14` (resolved by
  `FUN_004f6270 → FUN_005f2f90`) is loaded into the text area widget at
  `+0x1dc` via `FUN_005f3090`. The window's `SetWindowPos` adjusts the text
  area width:
  - Type 5 (encyclopedia-linked): `0x140` (320 pixels)
  - Others: `0x18b` (395 pixels)
- Types 3 and 5 (with `FUN_004ece60` guard): the Display button (`0x67`) shows
  bitmaps `0x2916/0x2917` (Alliance) or `0x2918/0x2919` (Empire) and label
  `0x8030`. hyp: this enables "Go To" for messages with a display target.
- Type 4: the Display button shows `0x2d20/0x2d21` (Alliance) or
  `0x2d47/0x2d48` (Empire) and label `0x8031`.
- Type 5 without a target: the Display button is hidden.
- After layout, the artwork overlays are started (`FUN_00610c30`) when the
  message has resource ids at `+0x30` and `+0x32`.
- The message is marked read: `*(param_1 + 0x38) = 10` (font 10, the read
  font).

## Navigate (0x90, 0x96, 0x9a)

| Button | Position | Bitmaps | TEXTSTRA | Visible in mode |
|---|---|---|---|---|
| `0x90` navigate | (282, 87) | 10900 / 10901 | `0x8008` | 1 |
| `0x91` delete | (340, 87) | 10902 / 10903 | `0x8009` | 1 |
| `0x96` scroll up | (390, 15) | 10919 / 10920 | — | 2 |
| `0x9a` scroll down | (367, 15) | 10948 / 10949 | — | 2 |
| `0x97` display toggle | (352, 255) | 10934 / 10935 | `0x8030` | 2 (type 5) |
| `0x98` encyclopedia | (355, 244) | 10934 / 10935 | — | 2 (type 5) |
| `0x99` detail | (355, 281) | 10938 / 10939 | — | 2 (type 5) |

In mode 2, scroll up/down navigate through the filtered message list. In mode
1, navigate (`0x90`) advances to the next message. hyp: `0x96` and `0x9a`
are the list view's scroll arrows, reused in mode 2 as prev/next.

## Close (0x28) and Display Message (0x65)

`FUN_004665f0` builds both on the right rail, by side (`+0x114 +0x9c`):

| Button | Alliance | Empire | Bitmaps (Alliance / Empire) | Label |
|---|---|---|---|---|
| `0x28` close | (423, 25) 32x31 | (426, 21) 44x41 | `0x2882`/`0x2883`, `0x2888`/`0x2889` | `0x1954` |
| `0x65` display | (423, 93) 32x31 | (426, 89) 44x41 | `0x2884`/`0x2885`, `0x288a`/`0x288b` | — |

Navigate (`0x90`, 10900/10901) and Delete (`0x91`, 10902/10903) are 56x20
at (282, 87) and (340, 87). `FUN_0042a240` opens the window only while
window `0x0d` is not open; a rail light (`0x136..0x13e`) or F6 (`0x75`)
while it is open does nothing.

## Delete Selected (0x91)

Button 0x91 is enabled in mode 1 only. The command's handler is in the message
index window procedure (vtable `0x0065a1c0`). From the evidence
(`message-index-recovery.md`), `FUN_005f54a0` deletes a single message by key
from the manager's list:

```c
FUN_005f54a0(this, param_1):
    puVar1 = FUN_005f5500(this, param_1);  // find by key
    if (puVar1 != NULL) {
        FUN_005f4fa0(this, puVar1);        // unlink from list
        (**(code **)*puVar1)(1);           // destroy
    }
```

hyp: the handler iterates the selection (`FUN_00609410`) and calls
`FUN_005f54a0` for each selected message's key. After deletion, the unread mask
at `+0x50` is recomputed by `FUN_0048a2a0`, and the rail rests for any
category that no longer has unread messages. The list rebuilds with
`FUN_00468ab0`.

## Sort order

Messages appear in arrival order (the manager's list, iterated by
`thunk_FUN_005f5080 / FUN_005f5c60`). New messages are appended at the tail
(`FUN_005f59f0` in `FUN_0048a060`). There is no column sort. The 4 columns
stored in the data list are layout data (icon, text, font), not sortable
column headers.

## Advice slowdown

From `FUN_004697b0.c`:

- Category 0x82 (Advice) calls `FUN_0041d3f0` → `FUN_004369f0` →
  `FUN_00487ff0(this, 1)`: saves the current speed index at `+0x58` and drops
  to Very Slow (index 1).
- Every other category calls `FUN_0041d410` → `FUN_00436a00` →
  `FUN_00487ff0(this, 0)`: restores the saved speed.
- Close (`FUN_0046a6a0`) also calls `FUN_0041d410` (restore).
- `FUN_00487ff0` guards against double-save: it stores `+0x54` → `+0x58` only
  when `+0x58` is 0, and restores only when `+0x58` is non-zero.

## Open

- The CoolDragList's internal selection rules: single click, Ctrl-click toggle,
  Shift-click range. The list control vtable `0x0066e0f0` handles these in
  `FUN_006083c0`; untraced.
- Column widths and their visual contents. The 4-column data list carries
  message data, but the visual rendering of columns in the 346x21 item is in
  the item draw function (`FUN_004c7e10`, also open in `fleet-window.md`).
- The exact `0x91` delete handler in the window procedure.
- Mode 2 scroll-through: how `0x96` and `0x9a` advance through the filtered
  list in single-message mode.
