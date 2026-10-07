---
title: "The Fleet Window"
description: "Window type 4 (FUN_004a2630): how it opens, its geometry and resources, its lists and tabs, its hit tests, and how a regiment is loaded onto a fleet through it"
category: "ghidra"
created: 2026-10-02
updated: 2026-10-02
tags: [fleet-window, sector-window, move-order, regiments, loading, strategy-resources]
---

# The Fleet Window

Recovered 2026-10-02 with Ghidra 12.1.3 headless (read-only project). Every
function named here has a `FUN_<address>.c` note in this directory. Bitmaps
are STRATEGY (`FUN_006037f0(7)`, module 7) unless stated; GOKRES is the
window's `+0x1b0` (`FUN_005fefd0(10)`, fallback `gokres_dll`, as in
`mission-dialog.md`). The manual shows the window as Fig. 3.65 (p. 122): a
system's title, its fleets and ships on the left, the selected fleet's
contents under four tabs on the right.

Class `FUN_004a2630`, vtable `0x0065bda0`, over the shared modeless base
`FUN_004ac120` (vtable `0x0065bf80`, class name `CustomDialogBox`). Window
type 4: its id is `(system index & 0x3ff) << 6 | 4` (`FUN_0045aac0`).

## Opening

- **Sector window, top-right quadrant.** `FUN_00459e30` gives every system in
  a sector window four quadrant overlay items (`sector-window-hit-test.md`,
  "Quadrant fleet-position overlays"). The top-right one has flag
  `sys_id | 0x100000`, rect `(cx + 1, cy - 19, cx + 29, cy)` around the
  planet's center. A double click (`WM_LBUTTONDBLCLK`, `FUN_004593e0` case
  `0x203`) finds the overlay under the point (`FUN_0045cc10`, list `+0x174`)
  and, when its `+0x3c` bit 2 (shown) is set, calls `FUN_0045aac0`, which maps
  the item's kind (`+0xc >> 16` = `0x10`) to window type 4.
- **The fleet icon.** Per system refresh (`FUN_0045b770`), `FUN_0045ccc0`
  picks a side: the player's (galaxy `+0x9c`) when it has fleets there,
  else the other side's, else the system's own side bits; the count is
  `FUN_004feea0(system, side, 3)`, active fleets (types `0x08..0x0f`) of that
  side with `+0x50` bit 6 (mode 3). `FUN_0045d140` hides the overlay when the
  count is 0 (clears `+0x3c` bit 2, `FUN_00600db0`) and otherwise shows it
  with `FUN_0045ca80(0x10, side, 0/1)`: **10775/10776** (`0x2a17`/`0x2a18`)
  for side 1, **10783/10784** (`0x2a1f`/`0x2a20`) for side 2, none for
  sides 0 and 3. The two ids are the item's two states (`FUN_0060bd20`).
  hyp: the second is the pressed or highlighted state. The overlay rect is
  one pixel larger than the 27 by 18 icon 10771 (`0x2a13`).
- The other quadrants open other windows: kind 4 (top-left, flag
  `0x40000`, icons `0x2a13/14`, `0x2a1b/1c`, `0x2a23/24`) the system window
  (type 9, `FUN_00452fc0`, 226 by 304); kind 8 (bottom-left, `0x80000`,
  `0x2a15/16`, `0x2a1d/1e`, `0x2a25/26`) type 10 (`FUN_004a7790`, the System
  Defenses window of Fig. 3.73, opened by the Defense icon, manual p. 125);
  kind `0x40` (bottom-right, `0x400000`, `0x2a19/1a`, `0x2a21/22`) type 11
  (`FUN_0049f130`). Their show rules are `FUN_0045cdc0`, `FUN_0045ce80`,
  `FUN_0045d090` (`sector-quadrants.md`).
- **By object.** `FUN_00429440` (galaxy view) opens the window for an object
  id through the sector window's `FUN_0045c8e0` (type 4 → kind `0x10`).
  Callers `FUN_00464c20`, `FUN_0046df90`, `FUN_0046e5f0` (hyp: the Fleet
  Finder's "Open Fleet window and Sector window", manual p. 125, and message
  links).
- **One per system.** `FUN_0045aac0` first looks the id up among the galaxy
  view's windows (`FUN_00604500`, list `+0x6c`). An existing window is shown
  and brought to the front (`FUN_004291d0`, `FUN_00429020` restore it from
  the rail). A new window is placed at the double-click point, clamped into
  the galaxy view's rect (`+0xcc..+0xd8`).
- **Subject.** The window's `+0x144` is the system (the object of the sector
  item, `FUN_004f3220`); `FUN_004a25c0` resolves it and closes the window
  (slot `+0x30`) when it no longer resolves.

## Geometry (235 by 304)

`FUN_004ac120(.., 0xeb, 0x130, ..)`: 235 by 304. All coordinates below are
window client pixels. The create handler is `FUN_004a4b10`; the paint is
`FUN_004a29c0` case `WM_PAINT`.

`side` below is the viewer's side `+0x170` (1 Alliance, 2 Empire; `(galaxy
+0x9c != 1) + 1`). Art pairs follow one rule: Alliance art is the lower id
and Empire art is `+0x32` (50) higher. "Own" art is the viewer's side and
"other" art is the opponent's (`uVar12 = (side != 1) * 0x32`).

| Element | Rect / position | Resources | Source |
|---|---|---|---|
| Background | (0, 0), native | **10770** (`0x2a12`), copied into the window surface `+0xa4` | `FUN_004a4b10`, `FUN_00607610` |
| Title bar strip | (2, 2), 231 by 18 (`w - 4`, `0x12`), blitted into `+0xa4` each paint | by `+0x16c` side: 1 → active **10299** (`0x283b`), inactive **10200** (`0x27d8`); 2 → **10201** (`0x27d9`) / **10302** (`0x283e`); else **10303** (`0x283f`) / **10304** (`0x2840`); active when the window is the galaxy view's `+0xb8` | `FUN_004a29c0`, `FUN_004a3340` |
| Title text | label `+0x124`: x = restore button width + 5 (2 without it), y = 2, rect (2, 2, w - 3, 16) | the system's name (`FUN_004f62d0`), font `0x24`, color `0x2000000` | `FUN_004a4b10`, `FUN_004ac120` |
| Restore-sector button (id `0xca`) | (3, 3), the bitmap's size | **10209** (`0x27e1`) / **10208** (`0x27e0`) | `FUN_004a4b10` |
| Close button (id `0x65`) | (w - bw - 3, 3) | **10108** (`0x277c`) / **10109** (`0x277d`) | `FUN_004a4b10` |
| Minimize button (id `100`) | (close x - bw, 3) | **10253** (`0x280d`) / **10254** (`0x280e`) | `FUN_004a4b10` |
| Right pane art | (97, 29), native, drawn only while a fleet or ship is selected | own **10407** (`0x28a7`, Empire `0x28d9`) `+0x180`; other `+0x184` (`0x28d9 - uVar12`); own when `+0x14c` bit 4, other when bit `0x40` | `FUN_004a29c0`, `FUN_004a4b10` |
| Selected-name label (id `0x13`) | (100, 29), 128 by 25 | the selected item's text; color `0x20000ff` for viewer side 1, `0x200ff00` otherwise; hyp: style 5 centers it | `FUN_004a4b10` (`FUN_00601880`), `FUN_004a5c00` |
| Picture panel `+0x168` | (100, 42)–(225, 91), 125 by 49 (`lpRect_006a8818`) | see "Picture" | `FUN_004a5c00` |
| Tab strips `+0x158` (own), `+0x15c` (other) | (99, 96), 131 by 28 (`0x83 x 0x1c`), strip ids `0x6a`, `0x6b`; background from the pane art at its position minus (97, 29) (`FUN_0060da10`) | tabs below | `FUN_004a4b10` (`FUN_0060d590`), `FUN_004a6980` |
| Left list `+0x160` (id `0xc9`) | (4, 29), 91 by 266 (`0x5b x 0x10a`), items 91 by 50 | background from `+0xa4` | `FUN_004a4b10` (`FUN_00607ea0`) |
| Right list `+0x164` (id `200`) | (101, 127), 133 by 164 (`0x85 x 0xa4`), items 125 by 50 | background from the pane art at (4, 98) (`FUN_00608300(.., 4, 0x62)`) | `FUN_004a4b10`, `FUN_004a5c00` |
| Scroll bars | one per list, base id **10699** (`0x29cb`), height 266 / 164 | `FUN_0060a490` | `FUN_004a4b10` |

Tabs: four `CoolStrobeButton`s (`FUN_00602150`) per strip, 31 by 28
(`0x1f x 0x1c`) at x = 1, 33, 65, 97 inside the strip, ids `0x66..0x69`,
tooltips TEXTSTRA `0x3823`, `0x3820`, `0x3821`, `0x3822` (the tooltip words
are paired with `DAT_0065d424`). For tab `k` (0..3) with base `b` = 10409
(`0x28a9`, Alliance) or 10459 (`0x28db`, Empire):

- normal `b + k`, selected (state 4) `b + 7 + k`, disabled (state 2, tabs
  1..3 only) `b + 3 + k`.
- The own strip uses the viewer's base and the other strip the opponent's.

| Tab | Id | Lists | Source |
|---|---|---|---|
| Capital ships | `0x66` | the ships | `FUN_004a6be0` |
| Fighters | `0x67` | squadrons aboard (`FUN_005039d0(ship, 3)`) | `FUN_004a6be0` |
| Troops | `0x68` | regiments aboard (`FUN_00504c40(ship, 3)`) | `FUN_004a6be0` |
| Personnel | `0x69` | characters and special forces aboard (`FUN_00536da0(ship, 3)`) | `FUN_004a6be0` |

As in `mission-dialog.md`, buttons blit each bitmap at its native size,
clipped by the control (`FUN_00602d30`).

## The left list

`FUN_004a3340` fills it each refresh:

1. The subject (the system) is registered first (`FUN_004acd10`, `+0x114`).
   hyp: as a hidden entry; it adds no visible row.
2. Every active fleet at the system, of **both** sides (`FUN_004ffe70(system,
   3)`, types `0x08..0x0f`, `+0x50` bit 6), in id order (the merge loop
   compares `+0x6c` ids), gets an entry (`FUN_004a37c0`). The entry is 91
   by 50 on the window background:
   - frame **10400** (`0x28a0`) for side 1, **10450** (`0x28d2`) for side 2,
     at (5, 5); selected image **10401** / **10451** (`0x28a1` / `0x28d3`);
   - overlays at (5, 5): **10423** (`0x28b7`, +50 for side 2) when the fleet
     is en route (`+0x50` bit 4; the blue streams of Fig. 3.65), **10424**
     (`0x28b8`, +50) on `+0x50` bit 9 (`0x200`; hyp: in combat);
   - a white dotted tree stub (`CreatePen(PS_DOT, 1, 0x2ffffff)`, `+0x1a0`):
     (2, 6)–(5, 6), and (2, 6)–(2, 50) while expanded;
   - text: the fleet's name (`FUN_004f62d0`), font 10 (`FUN_006002b0(10)`,
     `+0x1a4`/`+0x1a8`), drawn by the list item `FUN_004c7e10`;
   - item `+0x68` bit 8 when the fleet's side is the viewer's; bit 4 marks a
     fleet entry; bit 2 is expanded, clear at first.
3. Under each fleet, one entry per capital ship (`FUN_004a3d40`,
   `FUN_00502db0(fleet, 3)`), 91 by 50: the ship's GOKRES mini
   (`FUN_0042c3b0(gokres, id, ship, 0, 1)`: class `& 0xfff` + `0x4000`) at
   (5, 15), the tree lines (2, 0)–(2, 50) (or (2, 0)–(2, 6) and (2, 6)–(5, 6)
   for the last ship), and the indicators below. Ships are hidden (`+0x68`
   bit 0) while their fleet is collapsed.

Ship indicators (`FUN_004a67a0(bitmap, side, flags, left)`, y = 33 in the left
list and 23 in the right; +50 for side 2), from `FUN_004a66a0`'s flags:

| Flag | Meaning | Bitmap | x |
|---|---|---|---|
| 1 | squadrons aboard | **10404** (`0x28a4`) | 25 |
| 2 | regiments aboard | **10405** (`0x28a5`) | 41 |
| 4 | characters or special forces aboard | **10406** (`0x28a6`) | 57 |
| `0x40` | no hyperdrive (slot `+0x34(1)` is 0) | **10430** (`0x28be`) | 9 |

Flags 8 (`+0x50` bit 4), `0x10` (`+0x200`) and `0x20` (`+0x50` bit 2 clear)
are stored but draw nothing here.

Selection: a click selects (`0x29b`, `FUN_004a6390` id `0xc9`) and refreshes
the right side (`FUN_004a5c00`). A double click (notification `0x309`)
toggles the selected fleets' expanded bit and rebuilds them (`FUN_004a3d40`).
hyp: `0x309` is the list's double click; the list's own selection rules
(`FUN_00609410`) are untraced, as in `mission-dialog.md`.

## Picture and right list (`FUN_004a5c00`)

From the selected left entries (`+0x3c` bit 0):

- Selecting own-side and other-side entries together keeps only the
  own-side ones.
- **Nothing selected**: the label is empty, the picture panel shows the
  background at (100, 42), the right list is hidden, and both strips are
  hidden (`FUN_004a6980`).
- **One or more fleets**: the picture panel copies the pane art at (3, 13).
  When exactly one fleet is selected, it then draws the fleet picture
  **10425** (`0x28b9`, +50 for the Empire's art) horizontally centered. It
  adds **10426** (`0x28ba`) when the fleet is en route and **10427**
  (`0x28bb`) on `+0x200`. The label shows the name; two or more selected
  clear it.
- **One ship** (no `+0x68` bit 4; `+0x14c` bit 8): the picture is the ship's
  GOKRES portrait (`FUN_0042c3b0(gokres, id, ship, 1, 1)`); the Capital ships
  tab is disabled and a selected `0x66` moves to `0x67`.
- The strip shown is the own strip when an own-side entry is selected
  (`+0x14c` bit 4), else the other strip (bit `0x40`). A tab is disabled
  (state `0x40`) unless some selected entry carries its contents flag (1, 2,
  4 above).
- The right list holds, for every selected fleet's ships (or the selected
  ship), the current tab's objects (table above), each once. Each item is
  125 by 50 (`FUN_004a6e70`): frame **10420** (`0x28b4`, +50 by side) drawn
  at (1, 1) on the selected image only. The object's GOKRES mini is at
  (28, 4). A capital ship also gets its indicators at y = 23. The name comes
  from the item `FUN_004c7e10`.
- Tabs `0x67` and `0x68` also print two numbers into the picture panel's
  rect (1, 2)–(123, 47), white, font 4: the left-aligned number is the
  squadrons or regiments aboard (ship slots `+0x23c` / `+0x240`, summed),
  and the right-aligned one is the capacity (`+0x26c` / `+0x270`, summed)
  (`DrawTextA` flags `0x20` and `0x22`).

## En route marks (`FUN_0042c3b0`)

The left list's ship entries (`FUN_004a3d40`), the right list's items
(`FUN_004a6e70`) and the one-ship picture (`FUN_004a5c00`) draw their
object through `FUN_0042c3b0(gokres, id, object, picture, 1)`: `picture` 0
for a mini (GOKRES `(class & 0xfff) + 0x4000`), 1 for a portrait (`& 0xfff`).
Its last argument draws the status marks. The en route mark needs `+0x50`
bit 4 set and bit 3 (destroyed) clear, and goes at the image's origin
(`FUN_005fd0f0(.., 0, 0)`, keyed):

| Object (`id >> 24`) | Mini | Portrait |
|---|---|---|
| Default (capital ships, squadrons) | GOKRES `(class & 0xfff) + 0x5000` | GOKRES `+ 0x1000` |
| Regiments `0x10..0x13`, but classes `0x10000002`, `0x10000008` | STRATEGY 11515 (`0x2cfb`) | 11516 |
| Facilities `0x22..0x27` | 11509 (`0x2cf5`) | 11510 |
| Other facilities `0x20..0x2f` | 11505 (`0x2cf1`) | 11506 |
| Characters `0x30..0x3b` | 11501 (`0x2ced`) | none (`0x2ced + 0xffffd313` wraps to 0) |
| Special forces `0x3c..0x3f`, but classes `0x3c000003`, `0x3c000005` | 11501 | 11520 (`0x2d00`) |
| Fleets `0x08..0x0f` | 10423 (side 2: 10473) | 10426 (10476) |

The excepted regiment and special force classes keep the GOKRES default,
and GOKRES has exactly those four marks (21569, 21634, 21826, 21888). The
craft marks are engine glows on the blue key; the STRATEGY marks and the
four class marks are starfields with one key pixel, so they cover the mini.
`FUN_004f8240` makes an object en route while its container is, so every
ship, squadron, regiment and character aboard a travelling fleet carries
its mark. The manual's "blue engine glow" (Fleet window, pp. 112-113) and
"starfield behind the portrait" (hyperspace, pp. 96-97) are these static
bitmaps; nothing animates them.

In the picture panel (`FUN_004a5c00`), a fleet's 10426 (and 10427 on
`+0x200`) is blitted first, then the picture 10425 keyed over it at the
same centered left edge, so the glow shows behind the ships. One selected
ship's portrait, with its mark already drawn in, is blitted keyed at the
panel's (0, 0); the craft portraits are GOKRES `0x640..0x78f` (122 by 50)
and their marks `0x1640..0x178f`.

Where travelling mission members show: their target's container holds them
at once (`FUN_00556430`), but the System Defenses window's personnel page
lists only personnel not on a visible mission (`sector-quadrants.md`), and
the System window (type 9) builds only facility pages (`FUN_004568a0`,
pages `0x67..0x6c`, items from `FUN_00458fe0`, families `0x28..0x2f`; its
object-added slot `FUN_00454160` at vtable `0x00659ec4`). So only the
Missions window draws them (`FUN_004a0e10`).

## Input

`FUN_004a29c0` (slot `+0x14`, the window procedure) and `FUN_004a6390`
(`WM_COMMAND`):

| Control | Action |
|---|---|
| `100` minimize | posts `0x466` to the galaxy view: to the rail; rail icon from slot `+0x74` (`FUN_004a76e0`): **11536** (`0x2d10`) side 1, **11537** (`0x2d11`) side 2, **11538** (`0x2d12`) neutral, by the same side choice as the sector icon |
| `0x65` close, Escape | slot `+0x30` closes (`FUN_004a7390`, `FUN_004ac3a0`) |
| `0xca` restore sector | opens the subject's sector window (`FUN_00429ce0`) |
| strip `0x6a` / `0x6b` | the tab in the high word becomes `+0x194`; refresh |
| list `0xc9` / `200` | `0x29a` (drag ended outside, `CoolDragList`) posts to the galaxy view; `0x29b` selection; `0x309` expand (left only) |

- **Right click**: slot `+0x1c` (`FUN_004a2c40`) records which list was hit
  (id 200 sets `+0x14c` bit 0, `0xc9` clears it). It then opens the shared
  object pop-up menu (`FUN_004ac5c0`, `object-popup-menu.md`) for the
  selection (slot `+0x58`, `FUN_004a2c80`: the selected items of the hit
  list).
- **Rename**: slots `+0x78`/`+0x7c`/`+0x80` (`FUN_004ac7a0`, `FUN_004ac950`,
  `FUN_004aca40`) put an edit box over the selection rect (`FUN_004a71d0`).
  They submit its text through the pending order `+0x13c`, or cancel it. A
  click or message `0x407`/`0x40e` ends the edit. untraced: what starts it
  (hyp: the Rename order `0x203`).
- **Drag source**: a drag out of either list posts `0x29a`. The galaxy view
  (`FUN_00422ce0`, `move-order.md` "A drag is a move") moves the whole
  selection against the drop window's `+0x70`, as `0x201` (`0x202` with
  Ctrl), because the source is type 4. port: `fleet_window.rs` drags a
  left-list fleet or ship entry and a right-list regiment or ship; Ctrl's
  `0x202` is not ported. A fleet's drop goes through the Move path
  (`issue_fleet_move`): it joins a fleet under the point, else moves.
- **Refresh**: object notifications (slots `+0x5c`/`+0x60`, `FUN_004a2e70`,
  `FUN_004a2d60`) set `+0x14c` bit `0x10000000` for ships and fighters
  (`0x14..0x1f`), fleets, regiments and characters or special forces. Slot
  `+0x8c` (`FUN_004a7370`) refills when it is set.
- **Save state**: slots `+0x24`/`+0x28` (`FUN_004a7300`, `FUN_004a7290`)
  save and restore `+0x16c`, `+0x170`, `+0x174`, the subject, `+0xac` and
  the tab.
- hyp: at the end of the create handler, `FUN_00610b20`/`FUN_00610c30` play
  a sound with id `0x25e` from module 7 unless galaxy `+0xc0` bit 2 is set.

## Hit tests

`+0x70` (`FUN_004a3130`), used by move orders (`0x201`, `0x202`, `0x214`):

1. The default target:
   - the single selected own-side fleet entry (`+0x3c` bit 0 and `+0x68`
     bit 8);
   - none (an empty id) when two or more are selected;
   - none when own-side entries exist but none is selected;
   - the subject (the system) when the list has no own-side entries, or no
     entries at all.
2. A point in the left list (x 4..95, y 29..295): the subject, overridden by
   the entry under the point: a fleet, or a ship.
3. A point in the right list (x 101..234, y 127..291) while the tab is `0x66`:
   the ship under the point, if any.

`+0x68` (`FUN_004a2f80`), used by other orders: the subject, overridden by the
left-list entry under the point. In the right list it is the item under the
point, or else the left list's selected entry (`FUN_00609410`, `+0x188`).

## Loading a regiment onto a fleet

The route accepts a fleet (`0x08..0x0f`) or a capital ship (`0x14..0x1b`) as
a destination (`FUN_005531b0`, `move-order.md` "Route refusals"). A regiment
therefore joins a fleet when a move names the Fleet window's `+0x70` target.

- **Its orders.** A regiment's class (vtable `0x0065e438`, slot `+0x3c` →
  `FUN_005041c0` → `FUN_00504b30`) lists `0x201` Move, `0x202` Confirmed Move,
  `0x204` and `0x200` Scrap, after the base list of `FUN_00558380`
  (untraced). Its pop-up menu thus shows Move and Confirmed Move. Targeting
  released on a Fleet window asks `+0x70` (`move-order.md` "The release") and
  issues `0x201` against that fleet or ship.
- **The `0x201` path.** `0x204`'s group check (`FUN_0053d430`) allows an
  all-regiment group only toward the order's side, or onto an existing
  system (`+0x50` bit `0x40`) that is unpopulated (`+0x88` bit 0 clear;
  corrected from "bit 1", `regiment-unload.md`). The command's check
  `FUN_00555920` refuses `1`/`0x18` across systems when the object's speed
  slot `+0x34(1)` is 0. A regiment's speed (`FUN_004f63f0`) is
  `DAT_006b9050`, GNPRTB 1 (100), while it exists (`build-delivery.md`), so
  this never refuses a regiment: the earlier "regiments have no speed" was a
  misreading (`regiment-unload.md`). Across systems only, `0x28` refuses a
  regiment whose destination's side differs from its own.
- **Capacity.** `FUN_00500b40`, a capital-ship vtable slot (in the vtables at
  `0x0065d6c8`, `0x0065d9a8`, `0x0065dc88`), reports a ship's room for a
  type range:
  - regiments (`0x10..0x14`): `+0x270` (capacity) minus the regiments
    aboard (`FUN_00504c40(ship, ..)`, `FUN_00513180`), status `0x14`/2;
  - squadrons (`0x1c..0x20`): `+0x26c` minus the squadrons aboard, status
    `0x14`/1;
  - characters and special forces (`0x30..0x40`): unlimited;
  - anything else: refused.

  untraced: the leg builders (`FUN_00552000`, `FUN_005529a0`,
  `FUN_00552300`, `FUN_00552dd0`) that call it and pick the ship within a
  fleet, and the refusal a full fleet gives.
- **From the system window** (type 9), a drag issues `0x214` (`move-order.md`
  "Order 0x214"). Its per-object command (vtable `0x00669a30`) calls the
  object's slots `+0x1e0`, `+0x1e4` and `+0x200`. A regiment's `+0x1e4` is
  `FUN_006158b0` (`xor eax, eax; ret`), and its `+0x1e0` a thunk to
  `FUN_00520ba0`. hyp: `FUN_0057e0d0`'s validation therefore fails, and a
  regiment dragged out of a system window onto a Fleet window is refused
  (status untraced). Confirm natively before porting the refusal.
- **From the Fleet window or type 10** (System Defenses, which lists troops
  in its Troops tab, Fig. 3.73), a drag issues `0x201`, so the `0x201` path
  above applies.
- The container change itself is the move command's execute
  (`FUN_00578f30` → `FUN_00556390`). For a destination in the same system
  the leg is local. hyp: the regiment enters the ship's container through
  `FUN_00514a60`, which also gives it the system's withdraw percent
  (`blockade-troop-withdrawal.md`, "Regiment copy").

## Port notes

- `crates/rebellion-render/src/fleet_window.rs` is type 4; `system_window.rs`
  is type 9; `defenses_window.rs` and `missions_window.rs` are types 10 and
  11 (`sector-quadrants.md`).
- **Entry.** The sector window paints the top-right overlay (10775/10776,
  10783/10784) at `(cx + 1, cy - 19)`, 28 by 19, with `FUN_0045ccc0`'s side
  rule, and a double click on it opens the Fleet window at the click
  (clamped into the galaxy view). The other three quadrants open the
  System, System Defenses and Missions windows (`sector-quadrants.md`).
  A minimized Fleet window goes to the galaxy view's rail with icon
  11536..11538 (`FUN_004a76e0`) and reopens from it.
- **Loading.** A regiment's pop-up menu enables Move (hyp: `FUN_004f9860`'s
  side and en-route rules stand in for the untraced object check); Confirmed
  Move and Scrap are drawn disabled (port:). Targeting released on a Fleet
  window's fleet loads the regiment through F-007E's embark
  (`TroopTransportState::move_regiment`): in another system the regiment
  travels there and boards (`regiment-unload.md`), and `FUN_00500b40`'s room,
  counting regiments on their way, refuses a full fleet ("troop capacity
  exceeded"). A fleet's Move released on a Fleet window is refused
  ("joining fleets is not ported").
- **The hold** (port:). The original keeps a loaded regiment in its ship's
  container until a landing order. The port lands cargo at any uncontested
  system each tick, so `TroopTransportState` holds a fleet loaded by order
  while it orbits the loading system; any arrival releases it, and the cargo
  lands as before. The hold is saved (save v24, no migration).
- **Port rules.** The left list follows the system window's fog rule; one
  entry is selected at a time; there are no scroll bars; a listed ship
  stands for its fleet as a release target and draws no selected look; the
  pressed state of the sector window's icon is not drawn. Only a Troops tab
  regiment drags out of the window (`regiment-unload.md`).
- **Gate.** `tools/interface-parity/fleet-window.mjs` (fixture codes 48, 49,
  51, 52 and 54, both sides): the icon opens the window, whose chrome matches
  STRATEGY.DLL pixel for pixel outside masked text, the tree's dotted pen and
  the right list; a regiment's Move onto the fleet loads it (Troops tab,
  `1`/capacity) and holds it while the clock runs; the fleet's Move lands it
  at the target; a full fleet refuses it; fleets join and split; Rename
  (`0x203`, `rename-order.md`) keeps an emptied field open and submits the
  typed name; and the facility icon's Destination (`0x214`,
  `production-destination.md`) released on a planet sets it.
- **En route marks.** The ship entries and right-list items of a travelling
  fleet draw the marks above (`fleet_window::en_route_mark`), as the
  Missions window's members do, and one selected ship's portrait (its mini
  less `0x4000`) carries its mark. port: the other status marks (`+0x200`,
  `+0x50` bit 2, GOKRES `+0x7000`) are not drawn. The port's System window
  lists personnel, fleets, defenses and troops, which the original's type 9
  window does not (see above); that deviation is open.
- The port's capacity rules (F-007E: 0/0, 2/2, 3/3 by class) stand in for
  `FUN_00500b40`'s `+0x270`/`+0x26c`. Check them against the capital ships'
  DAT capacities.
- The Fleet Finder opens the window from the sector window's fleet icon
  with the fleet or ship selected, expanding the fleet for a ship (slot
  `+0x6c`, `FUN_00429440`; `fleet-finder.md`).

## Still open

- The list item drawing (`FUN_004c7e10`, `CoolDragList`): text position, color
  and the selected look; the scroll bar `FUN_0060f640`.
- `FUN_00558380`'s base order list; the regiment `0x214` refusal's status
  (native check pending). The leg builders and the full-fleet refusal are
  traced in `regiment-unload.md`.
