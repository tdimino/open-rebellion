# Mission dialog

This note records the player's mission-creation dialog: how it opens, its
controls and resources, its two pages, and what each command does. Read with
`mission-lifecycle.md` (the order it creates) and `ai-mission-planning.md`
(legality, `FUN_00583320`).

## Opening

The player right-clicks a character or team, chooses Mission (order `0x240`)
from the pop-up menu, and clicks the target with the targeting cursor
(`object-popup-menu.md`). A drag onto the map moves the team instead.
`FUN_0042a320` then asks `FUN_004f5380` → `FUN_005422f0` for
the MISSNSD records the team may undertake. With none, the order is handed back
through `FUN_0041ce20(order, 0)` and no dialog opens. Otherwise
`FUN_0046a750` builds the dialog.

## Window

`FUN_0046a750` creates a 259 by 355 (`0x103` by `0x163`) child. `+0x120` holds
the order, `+0x13c` the GOKRES resource DLL (index 10, fallback `gokres_dll`).
`FUN_0046a9c0` builds the controls. Bitmaps are STRATEGY unless stated.

| Resource | Use |
|---|---|
| 10801 / 10802 | Title bar (240 by 17), Alliance / Empire |
| 11100 | Page 1 panel (slot `0x32`) |
| 11101 | Page 2 panel (slot `0x33`) |
| 11121 / 11122 | Page 2 "Agents" / "Decoys" headers at (8, 65) / (136, 65), Alliance |
| 11123 / 11124 | The same, Empire |
| TEXTSTRA 34055 "Create Mission" | Title text at (5, 2) |
| TEXTSTRA 34052 "Target" | Page 1 label at (0x25, 0xc3) |

## Controls

| ID | Position, size | Bitmaps | Tooltip (TEXTSTRA) | Action |
|---|---|---|---|---|
| `0x64` | (242, 3) 14x14 | 10108 / 10109 | — | Close: destroy the order |
| `0x65` | (170, 320) 64x33 | 10596 / 10597 | 34051 "Cancel" | Destroy the order and close |
| `0x66` | (102, 320) 64x33 | 10594 / 10595 | 34050 "Begin Mission" | Set the order's mission (`+0x4c`) to the selected kind, submit it, close |
| `0x67` | (33, 320) 64x33 | 10592 / 10593 | 32817 "Encylopedia" | Open the Encyclopedia (`FUN_00429f30`, window `0x19`) at the selected kind |
| `0x68` | (101, 174) 65x18 | 10606 / 10607 | 34064 "Missions" | Toggle the mission-kind drop-down (`FUN_0060c1f0` posts command 1000) |
| `0x96` | (7, 20) 253x33 | `CoolTabControl` | — | Two tabs; switches pages (`FUN_0046c8a0`) |
| `0x97` | tab (0, 0) 116x33 | 11103/11104 A, 11105/11106 E | — | Page 1 |
| `0x98` | tab (130, 0) 116x33 | 11107/11108 A, 11109/11110 E | — | Page 2 |
| `+0x128` | (35, 62) 200x113 | `CoolSelectionBoxClass` | 34080 "Selected Mission" | The mission kind; 4 rows when open |
| `0xc8` `+0x130` | (8, 93) 108x213 | 107x59 items | — | Agents (the team) |
| `0xc9` `+0x138` | (136, 93) 108x213 | 107x59 items | — | Decoys |
| `0xca` | (120, 136) 16x16 | 11117 / 11118 | 34053 "Use Selected Agents as Decoys" | Move the selected agents to the decoys |
| `0xcb` | (120, 221) 16x16 | 11119 / 11120 | 34054 "Use Selected Decoys as Agents" | Move the selected decoys to the agents |

A double click (notification `0x309`) on an item in `0xc8` or `0xc9` acts as
`0xca` or `0xcb`. The moves rebuild both lists and write them to the order:
decoys to `+0x58` (`FUN_004f43b0`) and the team through vtable `+0x24`.

The OK button is `0x66`. `FUN_0041ce20` → `FUN_00436050` → `FUN_00487740` runs
the order's validator (vtable `+0x1c`, `FUN_004f4b60`). On failure
`FUN_00487c90` reports it; on success the order is dispatched. Either way
case `0x66` zeroes `+0x120` and falls through to the close, so the dialog
closes. `0x64` and `0x65` never submit.

## Pages

`FUN_0046c8a0` keeps the page at `+0x118`:

- Page 1 (`0x97`): panel `0x32`, the mission-kind box, and `0x68`. The target
  from the drop is drawn under the box: a system's planet bitmap or an object's
  portrait, centered in a 165 by 79 box, with its name.
- Page 2 (`0x98`): panel `0x33`, both lists, and the move arrows `0xca`/`0xcb`.

## Mission kinds

The box lists each record `FUN_004f5380` returns, in that order:
`FUN_005422f0` keeps every record that is not hidden (`+0x5c`), whose member
check passes (`FUN_005830a0`), and whose class validator accepts a temporary
mission built from the order (`FUN_0054c590` -> `FUN_00582b90`). Each item is
a 200 by 113 surface holding a 130 by 65 GOKRES icon, id `TEXTSTRA id & 0xfff`
for the Alliance and `+ 0x1000` for the Empire (record `+0x30`), with the
record's name in gray `0x808080`. For example, Diplomacy (TEXTSTRA 11280) is GOKRES 3088 or 7184.
`FUN_0046cab0` gives each item its key (`FUN_0060ba00` stores it at item
`+0x0c`) from the record's family `+0x2c`: Recruitment (`0x55`) 1, Jedi
Training (`0x58`) 2, Subdue Uprising (`0x57`) 3, the three research records
(`0x53`, by `+0x18` `0x20`/`0x21`/`0x22`) 4/5/6, Diplomacy (`0x51`) 7, Incite
Uprising (`0x56`) 8, Espionage (`0x52`) 9, Reconnaissance (`0x54`) 10,
Assassination (`0x63`) 11, Abduction (`0x62`) 12, Rescue (`0x61`) 13, Death
Star Sabotage (`0x6a`) 14, Sabotage (`0x69`) 15. Any other family takes the
next value of the counter `DAT_006a8794`.

The team list starts with the order's team (`+0x28`/`+0x2c`) and the decoy list
with `+0x58`, which is empty for a fresh `0x240` order.

## Composition and text

`FUN_0046a9c0` corrects two readings of the old report. 10801 and 10802 are
240 by 17 title bars, laid twice 30 pixels apart and blitted at (2, 2) over
each page. 11100 and 11101 are the full 259 by 355 page panels. The two
header bitmaps (108 by 27) go at (8, 65) and (136, 65) on the second page.

The text object (`FUN_00601b30` sets the top-left corner, `FUN_00601c60` the
font, `+0x10` the `DrawTextA` format, `+0x2c` bit 0 fixes the width at `+0`):

- "Create Mission" at (5, 2), left-aligned, black on the first page. The second
  page sets no color, so it inherits the white of "Target".
- "Target" at (37, 195), white.
- The target's name at (37, 294), 185 wide, centered, white.

The target art is a system's planet bitmap, `FUN_0045c970(FUN_00509610(system))`
(STRATEGY 10212..10239, as `sector_window::planet_resource_id` maps), or an
object's `FUN_0042c3b0(.., 1, 1)` portrait. It is centered in the 165 by 79
box at (51, 211) by `(0xa5 - w) / 2 + 0x33`, `(0x4f - h) / 2 + 0xd3`.

## Items

Each agent or decoy item is 107 by 59. `FUN_0042c3b0(gokres, .., 0, 1)` draws
a character (family `0x30..0x3b`) or special force (`0x3c..0x3f`) as GOKRES
`(class +0x30 & 0xfff) + 0x4000`: the 61 by 25 mini the system window lists.
With its last argument set, it overlays status bitmaps: STRATEGY 11500 behind
a character, 11501 or 11502 on object flags `+0x50` bits 4 and 9, 11570 or
11572 by side when bit 2 is clear, and GOKRES `+0x7000` on `+0xac` bit 0. The
mini is centered, and the name sits under it (item `+0x34`). A list's names
are gray `0x787878`, and white when selected (`+0xd8`/`+0xdc`).

Each mission item is 200 by 113: the 130 by 65 GOKRES icon centered, and the
record's name in gray `0x808080`.

## The drop-down

`FUN_0060bed0` stores the box's item height (113) at `+0xa4` and 1 at
`+0xa8`. `FUN_0060cac0` opens a 200 by 117 (`113 * 1 + 4`) window
(`FUN_0060cc50`) under the box, or above it when it would leave the screen.
Its window procedure `FUN_0060d020` fills it with a `CoolDragList` at (2, 2),
`w - 4` by `h - 4`, of 113-pixel items: one mission at a time. A click on an
item (command `0xbba`, notification `0x29b`) selects it through
`FUN_0060c970`, and a click outside sends command `0x3e9`, which closes it
(`FUN_0060cbf0`). The lists' scroll bars come from `FUN_0060a490` ->
`FUN_0060f640` with base ids `0x299a` (the drop-down) and `0x29fc` (agents and
decoys). STRATEGY holds 10749..10751 but not 10650 or 10748.

## Refusal

A refused order shows no text: the side's advisor droid reacts.
`FUN_00487740` hands the validator's two-word status to `FUN_00487c90`, which
calls vtable `+0xc` of the object at `+0xc0` and then destroys the order.
`FUN_004861b0` builds that object for the side, 0x194 bytes over the base
`FUN_00439320`: `FUN_004c27f0` (vtable `0x0065c4c8`) for side 1 and
`FUN_004c0710` (`0x0065c4a0`) for side 2. Its `+0x168` table holds 88
(`+0x164`) due times, one per advisor reaction.

Slot `+0xc` (`FUN_004c2940`, `FUN_004c0870`) remembers the status at `+0x174`
and sorts it by its first word: `0x90..0x97`, `0xa0..0xaf`, `8..0xf`,
`0x14..0x1b`, `0xf3`, and everything else (`FUN_004c3bb0`, `FUN_004c19f0`).
Each handler schedules one reaction by writing the clock `DAT_006b28cc` plus
10 into a table slot. For the statuses the port's dispatch refuses
(`MissionRefusal`):

| Status | Side 1 slot | Side 2 slot |
|---|---|---|
| The same status as the last one | `+0x3c`, then `+0x40` and the memory clears (`FUN_004c43d0`) | `+0x40`, then `+0x44` (`FUN_004c21d0`) |
| `0x40`/`0x91` for a Mission (`0x240`) whose target passes `FUN_004ece60`, with no target object and a `+0x58` (decoy) list | `+0x50` | `+0x54` |
| `0x40`/`0x01`, `0x28`, `0x29`, or any other `0x91` | `+0x44` or `+0x48` | `+0x48` or `+0x4c` |

The last row is the generic reaction: `FUN_004c4390` and `FUN_004c2230` pick
one of two slots with `FUN_0041cd80(2)`. Which clip and voice line each slot
plays is not traced; that belongs to the droid advisors' authored behavior
(audit P34).

port: until P34 plays these reactions, the dispatch writes a message-log line
for each refusal (`apply_panel_action`, `PanelAction::DispatchMission`).

## Ported (F-019 phase 5b)

`crates/rebellion-render/src/mission_dialog.rs` draws the dialog and handles
its controls. `rebellion_core::missions::available_kinds` builds the kind list.
Port decisions:

- The dialog opens from the object pop-up menu's Mission item and the
  targeting cursor (`object-popup-menu.md`, phase 7c). Phase 7d removed the
  missions panel's character and target pickers that stood in for them.
- port: the target is a system, so only kinds with a system target are
  listed; targeting a character or object is not ported
  (`object-popup-menu.md`, "Ported").
- hyp: `FUN_00606980` places the window in the galaxy view's rectangle; the port
  centers it there, rounded to whole pixels (both 640 by 480 galaxy views put
  the center on a half pixel).
- port: the scroll bars are not drawn; the wheel scrolls a list or the
  drop-down by one item.
- hyp: choosing a drop-down item closes it.
- port: the status overlays on items are not drawn.
- port: "Encylopedia" opens the Encyclopedia without the selected kind's entry
  (P66 binds contextual entries).
- port: a list click toggles an item's selection; `FUN_00609410`'s selection
  rules are not traced.

## Bitmap size

`FUN_00602150` (`CoolStrobeButton`) paints through `FUN_00602d30`, which calls
`FUN_005fc140(bitmap, dc, '@', SRCCOPY, 0, 0, 0, 0, 0, 0)`. A zero width or
height means the bitmap's own, so each bitmap is blitted at its native size
and the control's window clips it. "Encylopedia", "Begin Mission" and
"Cancel" are 64 by 33 controls holding 66 by 33 bitmaps (10592..10597): they
show the left 64 columns. The port draws every dialog bitmap this way.

## Browser gate (F-019 phase 6)

`tools/interface-parity/mission-dialog.mjs` opens fixture scenarios
`MissionDialogMission` (code 43) and `MissionDialogAgents` (44) for both sides.
It compares the static STRATEGY chrome exactly: the panel, the title bars,
the close box, the tabs, the headers, the arrows, the Missions button and the
three bottom buttons. Text, the kind item, the target art and the lists are
masked and kept as captures. The Mission scenario also clicks the Agents tab
and checks the second page.

## Still open

- `FUN_00606980`'s placement, and `FUN_0060f640`'s scroll bar.
- `FUN_0060c970` and whether a selection closes the drop-down.
- The reader of the advisor's `+0x168` table and the clip each refusal slot
  plays (P34).

## Supporting decompiles

`FUN_0046a750`, `FUN_0046a9c0`, `FUN_0046c3c0`, `FUN_0046c8a0`, `FUN_0046c850`,
`FUN_0046c880`, `FUN_0042a320`, `FUN_004f5380`, `FUN_004f4b60`, `FUN_0041ce20`,
`FUN_00436050`, `FUN_00487740`, `FUN_0041d6b0`, `FUN_00429f30`, `FUN_0060bed0`,
`FUN_0060c1f0`, `FUN_0060c210`, `FUN_0060d590`, `FUN_0060cac0`, `FUN_0060cc50`,
`FUN_0060d020`, `FUN_00607ea0`, `FUN_0060a490`, `FUN_0042c3b0`, `FUN_0045c970`,
`FUN_00601b30`, `FUN_00601b80`, `FUN_00601c60`, `FUN_00601c90`, `FUN_00601ce0`,
`FUN_0054c590`, `FUN_005422f0`, `FUN_00487c90`, `FUN_004c2940`,
`FUN_004c0870`, `FUN_004c3bb0`, `FUN_004c19f0`, `FUN_004c4390`,
`FUN_004c2230`, `FUN_004c43d0`, `FUN_004c21d0`.
