---
title: "The Sector Window's Quadrant Icons, the System Defenses Window, and the Missions Window"
description: "FUN_00459e30's four overlay items around each planet: their show rules (FUN_0045cdc0, FUN_0045ce80, FUN_0045ccc0, FUN_0045d090), art (FUN_0045ca80), the windows they open (types 9, 10, 4, 11), and the layout of the System Defenses (FUN_004a7790) and Missions (FUN_0049f130) windows"
category: "ghidra"
created: 2026-10-04
updated: 2026-10-04
tags: [sector-window, quadrant-icons, system-defenses, missions-window, strategy-resources]
---

# The sector window's quadrant icons

Recovered 2026-10-04 with Ghidra 12.1.3 headless (read-only project). Every
function named here has a `FUN_<address>.c` note in this directory. Functions
that the project leaves undefined were created in memory only
(`CreateAndDecompileTargets.java`). Bitmaps are STRATEGY (`FUN_006037f0(7)`)
unless stated. The fleet quadrant and the Fleet window are in
`fleet-window.md`.

## The four items

`FUN_00459e30` gives each system four overlay items (`FUN_00442130`). Each
item is `w` by `h` = 28 by 19, the 27 by 18 icon `0x2a13` (10771) plus one.
`(cx, cy)` is the planet's center `(x + 18, y + 18)`. The rects are
`(left, top, right, bottom)`, right and bottom exclusive (`:369-447`):

| Flag | Kind (`flag >> 16`) | Quadrant | Rect | Show rule | Opens (`FUN_0045aac0`) |
|---|---|---|---|---|---|
| `0x40000` | 4 | top-left | `(cx - 28, cy - 19, cx, cy)` | `FUN_0045cdc0` | type 9, the System window (`FUN_00452fc0`, 226 by 304) |
| `0x80000` | 8 | bottom-left | `(cx - 28, cy + 1, cx, cy + 20)` | `FUN_0045ce80` | type 10, System Defenses (`FUN_004a7790`, 235 by 304) |
| `0x100000` | `0x10` | top-right | `(cx + 1, cy - 19, cx + 29, cy)` | `FUN_0045ccc0` | type 4, the Fleet window (`FUN_004a2630`) |
| `0x400000` | `0x40` | bottom-right | `(cx + 1, cy + 1, cx + 29, cy + 20)` | `FUN_0045d090` | type 11, the Missions window (`FUN_0049f130`, 235 by 304) |

Each window id is `(system index & 0x3ff) << 6 | type`. There is one window
per id (`FUN_00604500` on the galaxy view's list `+0x6c`, as for the Fleet
window).

## Shared tail: FUN_0045d140(item, state, enabled)

- **State.** It is a side: 1, 2, or 0 and 3 for neither. It is stored in item
  `+0x3c` bits 2..4 (4, 8, `0x10`). A changed state repaints the item's two
  bitmaps from `FUN_0045ca80(kind, state, 0/1)` (`FUN_0060bd20`).
- **Enabled.** It is stored in item `+0x54`. Zero removes the item
  (`FUN_00600db0`) and clears `+0x3c` bit 1. Nonzero sets bit 1 and paints
  it. **An item with nothing to show is not drawn**, which is the same rule
  as the fleet icon.

## Art: FUN_0045ca80(kind, state, second)

| Kind | State 1 | State 2 | State 0 or 3 |
|---|---|---|---|
| 4 | 10771/10772 (`0x2a13/14`) | 10779/10780 (`0x2a1b/1c`) | 10787/10788 (`0x2a23/24`) |
| 8 | 10773/10774 (`0x2a15/16`) | 10781/10782 (`0x2a1d/1e`) | 10789/10790 (`0x2a25/26`) |
| `0x10` | 10775/10776 (`0x2a17/18`) | 10783/10784 (`0x2a1f/20`) | none |
| `0x40` | 10777/10778 (`0x2a19/1a`) | 10785/10786 (`0x2a21/22`) | none |

Any other state returns 0. The second id is the item's other state.
hyp: it is the pressed or highlighted state, as in `fleet-window.md`.

## The show rules

The DatId families come from the counting helpers. Each walks the system's
objects in a family range through `FUN_00513050`, with side filter 3 for
every side, and `FUN_00513180` counts them.

| Family range | Helper | What it covers |
|---|---|---|
| `0x10..0x14` | `FUN_00504c40` | regiments |
| `0x1c..0x20` | `FUN_005039d0` | fighter squadrons |
| `0x22..0x28` | `FUN_00526fd0` | defense facilities |
| `0x28..0x30` | `FUN_0053b6e0` | manufacturing (`0x28..0x2a`) and production (`0x2c..0x2d`) facilities |
| `0x30..0x40` | `FUN_00536da0` | characters and special forces |

A character's mission key is `+0x68` (`FUN_0042d170`). `FUN_004ece60` is
false only for the empty key: id `2` with family 0. Role flags `+0x78` bit 8
is OnHiddenMission (`decoy-roll.md`). A member is **on a visible mission**
when its key is set and bit 8 is clear.

- **Kind 4, top-left** (`FUN_0045cdc0`).
  - The state is the system's side bits `(+0x24 >> 6) & 3`.
  - Enabled is the count of manufacturing and production facilities at the
    system.
  - A zero count forces state 0, but the item is hidden then anyway.
  - The icon shows when the system has a shipyard, training facility,
    construction yard, mine or refinery. It takes the system's side colour.
- **Kind 8, bottom-left** (`FUN_0045ce80`).
  - The state is the system's side bits.
  - Enabled is the sum of the defense facilities, regiments and fighter
    squadrons, plus every character or special force at the system that is
    **not** on a visible mission (no key, or OnHiddenMission).
- **Kind `0x40`, bottom-right** (`FUN_0045d090`). The state is
  `FUN_004a1f60(system, player side)`. It walks the characters and special
  forces on a visible mission and collects their distinct mission keys
  (`FUN_004f5940`, `FUN_004f44b0`), split by the member's side bits into the
  player's set and the other side's set. It returns:
  - the other side when that set is non-empty;
  - otherwise the player's side when the player's set is non-empty;
  - otherwise 3.
  
  The item is disabled for state 0 or 3. **The other side's missions take
  precedence.**
- **Kind `0x10`, top-right** (`FUN_0045ccc0`): see `fleet-window.md`.

The object these rules read is the galaxy view's system. hyp: it is the
player side's view (`FUN_00539fd0`), so enemy objects appear only as the
player knows them. The port must decide this with a `port:` rule, using the
visibility already used by the System and Fleet windows.

## Type 10: the System Defenses window (FUN_004a7790)

Manual p. 125, Fig. 3.73. The class is vtable `0x0065be30`, over
`FUN_004ac120` (`CustomDialogBox`), 235 by 304. GOKRES is `+0x15c`. The
subject system is `+0x144`.

Vtable slots: `[0]` `FUN_004a78d0`, `[5]` `FUN_004a86d0` (window proc),
`[9]`/`[10]` `FUN_004aa6d0`/`FUN_004aa760`, `[14]` `FUN_004a8790` (create),
`[17]` `FUN_004a9800`, `[18]` `FUN_004a9890`, `[19]` `FUN_004aa2b0`,
`[22..25]` `FUN_004a7a20`, `FUN_004a7b40`, `FUN_004a7f10`, `FUN_004a7f50`
(refresh), `[26]` `FUN_004aa380` (the item under a point), `[27]`
`FUN_004aa4d0`, `[29]` `FUN_004aa4a0`.

- **Side shown** (`+0x148`): the system's side bits (`FUN_004a8790:42`).
  `FUN_004a7f50` refreshes it when they change.
- **Background**: `0x2951` (10577, 235 by 304), also the list's backdrop.
  The title bar matches the Fleet window's:
  - the sector button `0x27e1/0x27e0` (10209/10208, id `0xca`) opens the
    system's sector window (`FUN_00429ce0`);
  - minimize `0x280d/0x280e` (10253/10254, id `0xc9`) posts `0x466`, which
    sends the window to the rail;
  - close `0x277c/0x277d` (10108/10109, id 200) runs slot `+0x30`.

  The title strip is `+0x5f` while the window is the galaxy view's focused
  child, else `+0x60` (`FUN_004a9800`):
  - side 1: 10299/10200;
  - side 2: 10201/10302;
  - otherwise: 10303/10304.

  These are the Fleet window's strips.
- **Rail icon** (slot `+0x74`, `FUN_004aa4a0`, from the disassembly at
  `0x004aa4aa`): by the system's side bits, 11533 (`0x2d0d`) for side 1,
  11534 for side 2, 11535 otherwise.
- **Tab strip** (`FUN_0060d590` at (0, 20), 304 by 33). There are five
  buttons, 36 by 33, each with help message `0x1740 + n`:

  | Id | x | Content | Count helpers | Art: normal/selected and empty, for side 1, side 2, other |
  |---|---|---|---|---|
  | 5 | 172 | KDY-150 (`0x22`) and LNR batteries (`0x23`) | `FUN_005273d0` + `FUN_00527150` | `0x2936`/`0x2937`, empty `0x2938`; every side |
  | 4 | 136 | GenCore shields (`0x24`) and the Death Star Shield (`0x25`) | `FUN_005276d0` + `FUN_0051c0d0` | `0x2939`/`0x293a`, empty `0x293b`; every side |
  | 3 | 100 | fighter squadrons (`0x1c..0x20`) | `FUN_00503550` | `0x293c`/`0x293d` and `0x293e`, `0x293f`/`0x2940` and `0x2941`, else `0x2942` |
  | 2 | 64 | regiments (`0x10..0x14`) | `FUN_005044f0` | `0x2943`/`0x2944` and `0x2945`, `0x2946`/`0x2947` and `0x2948`, else `0x2949` |
  | 1 | 28 | personnel not on a visible mission (`0x30..0x40`) | `FUN_00536e20` walk | `0x294a`/`0x294b` and `0x294c`, `0x294d`/`0x294e` and `0x294f`, else `0x2950` |

  `FUN_004a9ce0` sets each button's normal bitmap (state 0): the "has some"
  id when its count is nonzero, the empty id (two higher) otherwise. Tabs 1
  to 3 also set states 1 (pressed) and 4 (selected) to the middle id. Tabs 4
  and 5 set only the constructor's pair, normal and pressed, and
  `FUN_0060d700` copies a button's state 1 bitmap into state 4 when it has
  none, so the selected battery tab is `0x2937` and the selected shield tab
  `0x293a`. A selected button paints its state 4 bitmap (`FUN_00602d30`).
  No tab is ever disabled: an empty tab still opens its empty page. Each button's text is its help
  message, the TEXTSTRA strings:
  - 5952 "Planetary Batteries" (`0x1740`, tab 5);
  - 5953 "Planetary Shields" (tab 4);
  - 5954 "Fighter Squadrons" (tab 3);
  - 5955 "Trooper Regiments" (tab 2);
  - 5956 "Personnel" (`0x1744`, tab 1).

  The defense facility families come from DEFFACSD.DAT:
  - KDY-150 `0x22`;
  - LNR Series I and II `0x23`;
  - GenCore Level I and II `0x24`;
  - Death Star Shield `0x25`.

  Every count is filtered to the shown side (`+0x148`).
- **Side bitmaps** (`FUN_004a9ce0` `+0x5f/+0x60/+0x5d`):
  - side 1: `0x283b`, `0x27d8`, `0x2952`;
  - side 2: `0x27d9`, `0x283e`, `0x2953`;
  - side 3: `0x283f`, `0x2840`, `0x2953`.

  `+0x5d` (10578/10579, 61 by 25) is each list row's frame
  (`FUN_004a9ab0` → `FUN_005fd0f0`), keyed over a copy of the object's
  GOKRES mini (`FUN_0042c3b0`) to make the selected image (see Rows).
- **List** (`FUN_00607ea0` at (7, 81), 222 by 210, id `0xcb`, cells 70 by
  70):
  - scroll bar art `0x29cc`;
  - text offset (1, `0x1b`) in the cell (`+0xe8`, `+0xec`);
  - selected text colour `0x20000ff` for side 1, `0x200ff00` for side 2,
    `0x2ffff00` otherwise (COLORREF: red, green, cyan); white otherwise.
  
  `FUN_004a90d0(page)` fills it, and page 1 is selected first. The pages,
  each filtered to the shown side:
  1. personnel off a visible mission (`FUN_00536e20`);
  2. regiments (`FUN_00504cc0`);
  3. squadrons (`FUN_00503a50`);
  4. the `0x22..0x28` walk keeping families `0x24` and `0x25` (shields);
  5. the same walk keeping `0x22` and `0x23` (batteries).

  Ghidra shows the cases falling through, but the disassembly ends each one
  with `JMP 0x004a963c`, the shared tail. That tail writes the selected tab
  button's text into `+0x5b`. On page 2, when the system's side bits are
  the player's (`FUN_0041cdb0`), it writes TEXTSTRA 6471 "Garrison
  Requirement: " followed by the system's `+0x80` into `+0x5c`; otherwise
  it empties `+0x5c`.
- **Keys** (`FUN_004aa2b0`): Escape closes. Left and right move to the next
  visible tab (`FUN_0060d7e0`). Other keys go to the list.
- **Showing an object** (slot `+0x6c`, `FUN_004aa4d0`) picks the page by
  family:
  - `0x22..0x23` page 5;
  - `0x24..0x25` page 4;
  - `0x1c..0x1f` page 3;
  - `0x10..0x13` page 2;
  - `0x30..0x3b` page 1.
- **Menu** (2026-10-06, Claude, from the vtable at `0x0065be30` read out
  of REBEXE.EXE): the class keeps the base dialog's slot 7 (`+0x1c`,
  `FUN_004ac5c0`, the right-release handler) and slot 8 (`+0x20`,
  `FUN_004ac730`, the item handler), as type 9 (vtable `0x00659e68`) does.
  So a right release on the list opens the object pop-up menu
  (`object-popup-menu.md`) for slot 22's selection: `FUN_004a7a20` walks
  the list's selected items (`FUN_00609410`) into the team. A right press
  selects as a left press does (`FUN_006083c0`), and a press over no item
  clears the selection (`FUN_006094b0`). port: `defenses_window.rs` opens
  it for characters, special forces and regiments; a defense facility's
  class menu is not ported.
- **Release target** (`FUN_004aa380`): the list item under the point,
  offset by (7, 81), else the window's system.
- **Labels**: the title `+0x49` is the system name (`FUN_004f62d0`) at
  (title button width + 5, 2), 16 high, font 5, format `0x24`
  (`DT_VCENTER | DT_SINGLELINE`: left-aligned). `+0x5b` is at (2, 51), 231
  by 16; `+0x5c` is at (2, 63), 228 by 17. Both are white, font 4, format
  `0x25` (centred both ways), as manual Fig. 3.74 shows. `FUN_00601b30`
  sets a label's position, `FUN_00601c60` its font id (`+0x24`) and
  `FUN_00403e90` its `DrawTextA` format (`+0x10`, drawn by `FUN_00601ce0`);
  its `+0`/`+4` fields are its width and height.

- **Rows** (`FUN_004a9ab0`). Each object gets a `CoolDragList` item
  (`FUN_004421d0`, id = object id) whose normal image (`+0x20`) is the
  object's GOKRES mini (`FUN_0042c3b0(.., 0, 1)`) and whose selected image
  (`+0x24`) is a copy of the mini with the side frame `+0x5d` keyed over it
  at (0, 0) (`FUN_005fcc30`, `FUN_005fd0f0`). The name is `FUN_004f62d0`.
  Items go into a container with sort mode 4 (`FUN_00609340`), whose
  comparator (`FUN_0060a890`) is always false, so rows keep the order the
  page's walk yields them.
- **List drawing** (`FUN_006083c0` paint, `FUN_00609a00`, `FUN_00609960`).
  The list's flags are `0xa0000` at creation; pages 1 to 3 clear `0x20000`
  (dragging allowed), pages 4 and 5 set it (no drag; `FUN_006083c0` case
  `0x200`). `0x80000` is the grid layout (`FUN_00609ae0`): cells of 70 by 70
  (`0x46`), row-major from (0, 0), wrapping when the next cell's right edge
  would pass the list width (less a shown scroll bar), so three columns
  without a scroll bar. Each item's image (`FUN_0060bd00`: the selected
  image while selected, else the normal one) is blitted keyed at its cell's
  top-left. Text is drawn per item with `DrawTextA`:
  - the rect is the cell moved by (`+0xe8`, `+0xec`) = (1, 27), trimmed by 3
    on the right;
  - the format is `+0xe4` = `0x10` (`DT_WORDBREAK`, with `DT_NOCLIP` added);
  - the font is 10 (slot `+0x18`);
  - the colour is `+0xd8` = `0x2ffffff` (white) unselected and `+0xdc`, the
    side colour above, when selected (`FUN_00609950`, `FUN_00609940`).
  The scroll bar is `FUN_0060a490` with art `0x29cc` (10700), as in the
  Fleet window.
- **Refresh** (`FUN_004a7a20`..`FUN_004a7f50`). An arriving object
  (`FUN_004a7b40`) refills the page when its family matches the shown page
  (the facility, squadron, regiment and personnel ranges above, personnel
  only off a visible mission). A leaving object (`FUN_004a7f10`) refills too.
  A system side change (`0x90..0x97`, `FUN_004a7f50`) stores the new side in
  `+0x148`, reloads the side art (`FUN_004a9ce0`) and refills.
- **Save state** (`FUN_004aa6d0`/`FUN_004aa760`): the subject id (`+0x144`)
  and the window's `+0x24`. The window list belongs to the original save;
  port: the port saves no open windows, as with the Fleet window.

## Type 11: the Missions window (FUN_0049f130)

The class is vtable `0x0065bd10`, over `FUN_004ac120`, 235 by 304. The
subject system is `+0x144`, `+0x1c4` is the selected mission row, and
`+0x1c8` holds the selected mission's target.

Vtable slots: `[0]` `FUN_0049f320`, `[5]` `FUN_0049f4b0` (window proc), `[9]`/`[10]`
`FUN_004a0b00`/`FUN_004a0b90`, `[14]` `FUN_0049f540` (create), `[17]`
`FUN_0049fef0`, `[18]` `FUN_004a0120`, `[19]` `FUN_004a0e00`, `[22..25]`
`FUN_004a0400`, `FUN_004a0560`, `FUN_004a06d0`, `FUN_004a0870`, `[26]`
`FUN_004a09a0`, `[27]` `FUN_004a1e10`, `[29]` `FUN_004a21c0`.

The left list holds the system's visible missions, and the right list the
selected mission's agents or decoys under a two-tab strip. Above them are
the mission's target picture and its name. In detail:

- **Constructor** (`FUN_0049f130`): `FUN_004ac120` at 235 by 304 (`0xeb`
  by `0x130`); `+0x1b0` is GOKRES.DLL (`FUN_005fefd0(10)`, else loaded).
- **Create** (`FUN_0049f540`):
  - background `0x2b9d`;
  - the sector button `0x27e1`/`0x27e0` (id `0xca`) at (3, 3);
  - the close button `0x277c`/`0x277d` (id `0x14`) at (width less its width
    less 3, 3);
  - the minimize button `0x280d`/`0x280e` (id `0x15`) just left of it;
  - the title label `+0x17c` starts at the sector button's width plus 5.
  The lists' full argument lists (disassembly at `0x0049f8e2`, `0x0049fa14`)
  end `0, 0, 0, 0, 10, 1, 0`: `+0xc0` 10 and `+0xf0` 1, so both lay out
  vertically (`FUN_00609ae0`). Ghidra's call shows only the first ten.
  - Mission list `+0x1b8` (id 10): (5, 24), 94 by 275, items 90 by 50, flags
    `0x20000` (no drag, no grid), scroll art `0x29fc`, unselected text white
    (`FUN_00609950`), selected text the default white (`FUN_00607ea0`
    `+0xdc`), format `0x10` (`DT_WORDBREAK`), font 10, text offset the
    default (1, 0).
  - Member list `+0x1c0` (id `0xb`): (107, 145), 117 by 147, cells 115 by
    43, flags `0xa0000` (grid, no drag), white, format 1.
  - The tab strip (id `0x16`) at (105, 127), 122 by 16, with buttons at
    (0, 0) id `0x17` and (61, 0) id `0x18`. The create call passes 61 by
    43, but their bitmaps (11560..11568) are 61 by 16. Their help strings
    are TEXTSTRA 34048 "Agents" and 34049 "Decoys".
  - `+0x174` at (109, 91), 113 by 32, white, font 5, format `0x11`
    (centred, word-wrapped); `+0x178` at (109, 25), 113 by 16, white, font
    5, format 1 (centred).
  - Then `FUN_004a1590` fills the mission list and the first row is
    selected (`FUN_00609500`).
- **Title strips** (`FUN_004a2200`), by `FUN_004a1f60`'s side, focused and
  not: side 1 `0x283b`/`0x27d8`, side 2 `0x27d9`/`0x283e`, otherwise
  `0x283f`/`0x2840` (the Defenses window's art).
- **Paint** (slot 17, `FUN_0049fef0`): the picture `+0x170` centered in the
  122 by 50 box at (108, 37); `+0x174`, `+0x178`; the strip; the title
  TEXTSTRA 34085 "Mission at " and the system name (`FUN_004f62d0`) at
  (21, 2), 178 by 16 (235 less three times 19), font 5, format `0x24`
  (one line, left-aligned, vertically centred).
- **Mission rows** (`FUN_004a1590`). The walk is `FUN_00536da0(system, 3)`,
  as in `FUN_004a1f60`. Each member on a visible mission (a mission key
  `+0x68`, role `+0x78` bit 8 clear) records its mission once, with its
  class `+0x6c` and the member's side bits (`+0x24 >> 6 & 3`). Rows whose
  mission is gone are removed (`FUN_004a0c20` clears the selection when it
  goes). Each new mission whose class (`FUN_0051cab0`) exists gets a row:
  - picture GOKRES `0x4000 + (class +0x30 & 0xfff)` for side 1, else
    `0x5000 + ...`. `+0x30` is the MISSNSD record's TEXTSTRA id (`0x2c00`
    plus the record index), so Diplomacy is `0x4c10` (19472) or `0x5c10`
    (23568), 73 by 48;
  - the selected image is the picture with `0x2b77` (11127, 73 by 48) keyed
    over it, or `0x2b78` (11128) when the player is not side 1;
  - the label is the class name `+0x34` (TEXTSTRA `0x2c00` + index:
    "Diplomacy", "Espionage", ...).
- **Selecting a mission** (list notification `0x29b`, `FUN_004a0c60`,
  `FUN_004a0ca0`): the tab art follows the mission's side (`FUN_004a24b0`:
  the side bits of a member at the system carrying its key, else the
  player's): side 1 `0x2d28`/`0x2d29` and `0x2d2a`/`0x2d2b`, side 2
  `0x2d2d`/`0x2d2e` and `0x2d2f`/`0x2d30`. The strip and member list show,
  the Agents tab is selected and filled (`FUN_004a1ba0(0x17, 1)`), and the
  details refresh (`FUN_004a10a0`). With no mission selected, the strip and
  the member list hide and `+0x174`/`+0x178` hide.
- **Member rows** (`FUN_004a1ba0`, `FUN_004a0e10`): the members at the system
  carrying the selected mission's key; Agents (`0x17`) keeps role `+0x78`
  bit 0 clear, Decoys (`0x18`) bit 0 set. Each row is a 122 by 43 image with
  the GOKRES mini (`FUN_0042c3b0(.., 0, 1)`) centered, and the name
  (`FUN_004f62d0`) under the mini: item text offset (0, mini height + half
  the spare height), font 10, format `0x21` (centred, one line). No selected
  image.
- **Details** (`FUN_004a10a0`). The target `+0x1c8`:
  - the player's own mission: the mission object's target (`FUN_004f3000`,
    `+0x70`);
  - the other side's mission in families `0x50..0x5f`: the system;
  - otherwise none.
  The picture is a system target's planet (`FUN_0045c970(FUN_00509610)`,
  STRATEGY), else the target's GOKRES image: a character's mini
  (`FUN_0042c3b0(.., 0, 1)`, families `0x30..0x3f`), any other object's
  portrait (`(.., 1, 1)`); centered in the box.
  `+0x174` is the target's name, or TEXTSTRA 34087 "Target Unknown";
  `+0x178` is TEXTSTRA 34081 "Target:".
- **Commands** (`FUN_004a0120`): `0xca` opens the sector window
  (`FUN_00429ce0`); `0x14` closes; `0x15` posts `0x466` (minimize); `0x16`
  picks a tab and refills the member list.
- **Keys** (`FUN_004a0e00`): Escape closes.
- **Release target** (`FUN_004a09a0`): the picture rect gives `+0x1c8` (the
  rect is (108, 37) to the picture's centred left/top, so empty for a
  full-size picture: an original bug); a member row under the point offset
  by (107, 145) gives that member; else the system.
- **Refresh**: slots 23 to 25 (`FUN_004a0560`, `FUN_004a06d0`,
  `FUN_004a0870`) refill or refresh when a member joins, leaves or changes
  mission.
- **Show object** (slot 27, `FUN_004a1e10`): a member picks its tab by bit 0
  and is selected; a mission key `0x50..0x7f` selects its row.
- **Rail icon** (`FUN_004a21c0`): `0x2d13`/`0x2d14`/`0x2d15` (11539..11541)
  by `FUN_004a1f60`'s side.
- **Save** (slots 9 and 10): the subject and `+0x24`. port: not saved.

## Port notes

- The System window (type 9) is already ported (`system_window.rs`). Kind 4
  needs only its icon and the double click.
- The rules above give each icon's state and visibility. Phase 1b ports them
  with these tests:
  - each kind is hidden at zero;
  - kind 4 and kind 8 take the system's side;
  - kind 8 counts personnel only when they are not on a visible mission;
  - kind `0x40` prefers the other side's missions.
- Phase 1c ports the System Defenses window (`defenses_window.rs`) and
  phase 1d the Missions window (`missions_window.rs`). Both share the
  title strips and title buttons, take the rail through `0x466`, and give
  a move released over them their system (`+0x70`). What the port leaves
  out of both:
  - the scroll bars, so the lists show only what fits;
  - the tabs' help text;
  - the windows' keys, since the galaxy view's windows share no keyboard
    focus;
  - list drags;
  - the `+0x68` hit test, which nothing in the port asks;
  - saving open windows.
- The Missions window lists only what the player sees of the other side,
  as the sector window's icons do (`opposing_contents_visible`). Its rows
  follow the mission ids (hyp: `FUN_004a1590`'s map orders them by key).
  Member and target minis carry no status overlays, as in the Mission
  dialog. The picture's hit rect bug (`FUN_004a09a0`) is moot, because the
  port has no `+0x68` hit test.
