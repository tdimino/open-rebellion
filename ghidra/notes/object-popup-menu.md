# Object pop-up menu and targeting

This note records how the player starts an order on an object: the right-click
pop-up menu, its items, and the targeting cursor that picks the order's target.
The mission dialog (`mission-dialog.md`) opens at the end of this path.

The manual says the same: "The general procedure to initiate a mission is to
right-click on a character or team and select Mission. When you do this, the
cursor changes to the cross hair, then select a target for the mission"
(p. 100, also pp. 39 and 57). A left-button drag onto the map is a move, not a
mission (below).

## Opening the menu

`FUN_006007b0`, the base window procedure, sends `WM_RBUTTONUP` (0x205) to the
window's vtable slot `+0x1c` with the cursor point. A list control
(`CoolDragList`, vtable `0x0066e0f0`) passes it to its parent through
`FUN_00609fc0`, which maps the point into the parent's coordinates. A system
window (vtable `0x00659e68`) handles it in `FUN_004ac5c0`:

1. Nothing happens while any window holds the mouse capture.
2. The window's selection (vtable `+0x58`) is copied to `+0x11c`.
3. `FUN_0041dcc0` → `FUN_004fcf20` → `FUN_0051d990` lists the orders the
   selection may take (below).
4. Each order becomes an item through `FUN_00442590(order, module 7, flags,
   x, y, galaxy view, text, 0x2ffffff, 0x2808080)`. Flags bit 0 is set when
   the order is disabled and bit 1 when it is checked. The text color is
   `0x20000ff` for side 1 and `0x200ff00` otherwise, as in the speed menu
   (`game_speed.rs`).
5. `FUN_00442380` opens the menu at the point, in the galaxy view.

## The items

`FUN_0051d990(side, selection, out)`:

- Each selected object's class lists its orders (class vtable `+0x3c`, given
  the object's status). The first object's list is kept, and each later list
  is intersected with it.
- For each order kind, a temporary order is built with the selection as its
  team and no target. The kind is listed when the order's vtable `+0x10`
  accepts it. The item is enabled when `FUN_0051de80` passes and vtable
  `+0x18` accepts it, and checked when `+0x14` says so.
- Encyclopedia (0x100) is always added, enabled for a single selection.
  Status (0x103) is always added, enabled for a single selection that is not
  a system (family `0x90..0x97`).

Each item reads a 26-byte `RT_RCDATA` record named by the order kind from
STRATEGY.DLL (`FUN_00442790`). Its words are: 0 the kind, 1 the parent
submenu, 2 the sort key, 5 the TEXTSTRA string, 7 and 8 the icon, 10 a second
bitmap, and 12 the module. The items a character may show:

| Kind | Sort | TEXTSTRA | Text |
|---|---|---|---|
| `0x201` | 10 | 12312 | Move |
| `0x202` | 12 | 12311 | Confirmed Move |
| `0x240` | 300 | 12320 | Mission |
| `0x260..` | — | 12354.. | Command submenu (parent `0x160`): None, Commander, Admiral, General |
| `0x100` | 1000 | 12292 | Encyclopedia |
| `0x103` | 1001 | 12293 | Status |
| `0x242` | 2002 | 12336 | Retire |

The manual's character menu (p. 99) shows Move, Confirmed Move, Mission,
Command, Encyclopedia and Status in that order.

### A character's orders

`FUN_00504dc0` resolves each selected id to the object itself, so `+0x3c` is
the object's own slot. In the character vtable `0x0065ca70` it is
`FUN_004ed350`, which copies a static list built once by `FUN_004f2400`: the
unit list of `FUN_00536bc0` (`0x201`, `0x202`, `0x242`, `0x240`, `0x204`,
`0x241`, after `FUN_004f2a10`'s empty object list) plus `0x260..0x263` and
`0x268`. The status argument is not read. STRATEGY holds no record for
`0x204`, `0x241` or `0x268`, so they never become items. The Command parent
`0x160` (word 4 = 1, a submenu) has the children None (sort 402), Admiral
(406), General (408) and Commander (409), each with bitmap 11902.

Orders are made by `FUN_004f5cd0` → `FUN_0051f8f0`, which looks the kind up
in a table that `FUN_0051f930(kind, factory)` fills (`FUN_0041ec50`,
`FUN_0051f4b0`). Mission `0x240` is `FUN_004f5250` → `FUN_004f5200`, vtable
`0x0065d118` over the base `FUN_004f4690` / `FUN_0051fa20`.

### When Mission is enabled

- Listed: `+0x10` is `FUN_0040f340`, which returns 1.
- Checked: `+0x14` is `FUN_0051fd30`, which returns 0.
- Enabled: the global gate `FUN_0051de80` (bit `0x10000000` of
  `+0xc0`→`+0x74` clear) and `+0x18`, `FUN_0051fe20`. That needs the order
  built (`+0x1c`, set when `FUN_0051fa20` allocated the list at `+0x40`),
  then `+0x4c` (`FUN_004f4a00`): it creates mission-create command `0x250`
  (`FUN_004f4990` → `FUN_0054cd80`, vtable `0x00661e28`), copies the team
  and decoys into it with placeholder target `0xf8000006`, and adds it to
  `+0x40`. `FUN_00553770` then needs a command in the list (status `0x16`
  otherwise), runs each command's `+0x18`, and requires all of them to share
  the location state bits (`0x14`).
- The command's `+0x18`, `FUN_0054d1a0`, resolves the target and calls
  `FUN_005429e0(.., team, decoys, ..)`:
  - `FUN_0054bb90` moves prisoners to the captured list and fails on an
    empty team (`0x40`/`0x91`).
  - `FUN_0054bf00` requires every member at one location (`0x40`/3) and
    at least one located member (`0x16`).
  - `FUN_0054c110` asks each member's slot `+0x1c8`.
- Slot `+0x1c8` for a character is `FUN_004ed560`, for a special force
  `FUN_00533ce0`, over the object check `FUN_004f9860`. A member is refused
  (status `0x30`) when it:
  - is not the order's side (`+0x24` bits 6..7, 1);
  - is unrecruited (`+0x50` bit 2 clear), except for orders `0x203`,
    `0x204`, `0x215` and `0x241` (2);
  - has `+0x50` bit 3 set (3, meaning untraced);
  - is en route (`+0x50` bit 4, 4);
  - is on a mission (`+0x78` bit 7, 1);
  - for `0x240`, is injured (`+0x94` != 0, 3);
  - for `0x240`, is a prisoner (`+0xac` bit 0, 2).

So Mission is enabled for a selection of free members of the player's side,
not on a mission, not travelling, not injured, all at one place. No target
or kind is checked; those wait for the dialog.

## The window

`FUN_00442860` builds the "Game Menu Window" already ported for the speed
menu (`2026-09-24-game-speed-recovery.md`): STRATEGY frame tiles
10100..10107, rows laid out by `FUN_00442a80`, submenu arrows 10117/10118
(side 1) or 10128/10129. A left-button release on an item (`FUN_004424c0`)
calls the owner's vtable `+0x20` with the item's kind; a press outside closes
the menu. For a system window that is `FUN_004ac730`, which calls
`FUN_0041cdf0(kind, selection, no target, 0)`.

## Targeting

`FUN_0041cdf0` → `FUN_00436020` → `FUN_00486fb0` handles the kind. For
`0x240`, `FUN_00487c50` builds the order (team = the selection) and
`FUN_0041d5e0` → `FUN_00429320` hands it to the galaxy view: mode `+0xc0` = 2,
the order at `+0xc4`, the mouse captured, and the cursor `+0x46c` set to the
targeting cursor `+0x468`. That cursor is REBEXE.EXE cursor group 1002
(`LoadCursorA(.., 0x3ea)` in `FUN_00422ce0`'s `WM_CREATE`): 32 by 32, 8-bit,
hotspot (12, 12).

The next `WM_LBUTTONUP` (0x202) in mode 2 (`FUN_00422ce0`):

1. Finds the child window under the cursor and the object under the point.
   For a `0x240` order it asks the child's vtable `+0x68` (the object under
   the point in that window); move orders (`0x201`, `0x202`, `0x214`) ask
   `+0x70` instead. The galaxy map is drawn by the view, not by a child, so a
   release over the bare map finds no window and destroys the order
   (`move-order.md`, "Hit tests").
2. With no object, or an object the order rejects, the order is destroyed.
3. Otherwise, when the object is one of the team's own members
   (`FUN_004f5940`), the target becomes the first system found walking up
   its parents. The target is set (vtable `+0x2c`) and `FUN_0042a320` opens
   the mission dialog, or hands the order back when no kind is legal.
4. Mode returns to 1 and the capture is released.

Command `0x15e` in mode 2 destroys the pending order and restores mode 1.

## A drag is a move

`FUN_006083c0` (the `CoolDragList` procedure) captures the mouse on a press.
A release more than 5 pixels away and outside the list posts notification
`0x29a` with the screen point; the system window forwards it to the galaxy
view (`FUN_004534f0`). `FUN_00422ce0` takes the source's selection, hit-tests
the drop point, and issues order `0x201` (Move), or `0x202` (Confirmed Move)
with Ctrl held, for window types 1, 4 and 10; type 9 issues `0x214`.

## Ported

- 7a: `crates/rebellion-render/src/game_menu.rs` draws the Game Menu Window
  for any owner, and the speed menu now uses it. Items are white, the
  highlighted item takes the faction color, and disabled items are gray
  (`FUN_004abe10`, `FUN_004aba60`). The P60 speed menu had the first two
  swapped. A submenu parent without an icon reserves 20 pixels
  (`FUN_004abf60`). port: Escape closes the menu.
- 7b: a right press selects a list item and a right release on a
  character, a special force, a regiment or empty list space opens the menu
  in the galaxy view. Since 2026-10-06 that list is the Defenses window's
  (type 10, `defenses_window.rs`, `sector-quadrants.md` "Menu"); the
  invented System window tabs that held it are gone, and type 9 opens the
  menu for its producer bands (`system_window.rs`,
  `manufacturing-build-selection.md`). Rows
  follow the STRATEGY records; Mission follows
  `MissionState::mission_order_enabled` and Encyclopedia opens the
  Encyclopedia. Status opens the Status window (`status-window.md`).
  port: Move, Confirmed Move, Command and Retire are drawn disabled, the other tabs' classes open no menu, and the global gate
  `FUN_0051de80` is taken as clear.
- 7c: Mission starts targeting (`targeting.rs`). The galaxy view takes the
  capture, so no window or cockpit control gets a press, and a press on the
  map selects nothing (`FUN_00422ce0` has no `WM_LBUTTONDOWN` case). Cursor
  1002 is drawn with its hotspot on the pointer, scaled with the canvas; it
  is staged by `extract-dll-resources.py REBEXE.EXE --cursors`, and the
  system cursor stays visible when it is missing. A left release on a map
  system sets the target and opens the mission dialog through
  `available_kinds`; a release anywhere else drops the order. Since F-007C
  phase 3a the release asks the window under the point, as the original
  does (`targeting::release_destination`, `move-order.md`, "Hit tests"): a
  system window gives its system, a sector window the planet under the
  point, and the bare map nothing. port: `+0x68`'s character and fleet
  answers reduce to their system, there is no walk up from a team member,
  Shift's pass-through click is not ported, and Escape cancels as `0x15e`
  does.
- 7d: the stand-in entry points are gone: the system context menu's Send
  Diplomat and Send Spy buttons (`PanelAction::OpenMissionTo`) and the
  missions panel's Dispatch tab (`PanelAction::OpenMissionDialog`). The
  missions panel lists active missions only.
- A sector window's quadrant icons are selected by a left or right press
  and open this menu on a right release, with their kind's orders
  (`sector-icon-menus.md`).

## Supporting decompiles

`FUN_006007b0`, `FUN_00609fc0`, `FUN_004ac5c0`, `FUN_004fcf20`,
`FUN_0051d990`, `FUN_00442590`, `FUN_00442790`, `FUN_00442380`,
`FUN_00442430`, `FUN_00442860`, `FUN_00442a80`, `FUN_004422f0`,
`FUN_00442d10`, `FUN_004424c0`, `FUN_004aab50`, `FUN_004ab560`,
`FUN_004aba60`, `FUN_004ac730`, `FUN_0041cdf0`, `FUN_00436020`,
`FUN_00486fb0`, `FUN_00487c50`, `FUN_0041d5e0`, `FUN_00429320`,
`FUN_006083c0`, `FUN_004534f0`, `FUN_00422ce0`, `FUN_004ed350`,
`FUN_004f2400`, `FUN_00536bc0`, `FUN_004f2a10`, `FUN_0051f8f0`,
`FUN_0051f4b0`, `FUN_004f5250`, `FUN_004f5200`, `FUN_0040f340`,
`FUN_0051fd30`, `FUN_0051fe20`, `FUN_0051de80`, `FUN_004f4a00`,
`FUN_004f4990`, `FUN_0051fa20`, `FUN_00553770`, `FUN_0054d1a0`,
`FUN_005429e0`, `FUN_0054bf00`, `FUN_0054c110`, `FUN_004ed560`,
`FUN_00533ce0`, `FUN_004f9860`.
