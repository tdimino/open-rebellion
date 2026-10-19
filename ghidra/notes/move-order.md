# Move and Confirmed Move

This note records how the player moves a fleet: the Fleet pop-up menu, the
Move (`0x201`) and Confirmed Move (`0x202`) orders, the targeting release, the
per-object move command, and the confirmation window. The menu window and the
targeting mode are shared with Mission (`object-popup-menu.md`).

The manual (pp. 121-122): "Issue commands to fleets by right-clicking on the
Fleet icon. This brings up the Fleet menu." Move: "the cursor changes to
targeting cross hairs. Click on the destination system or press the ESC key to
cancel. The fleet immediately goes into hyperspace to reach its destination."
"Any time a fleet, or a ship within a fleet, is in hyperspace, it cannot
receive orders." Confirmed Move "brings up a window that tells you the transit
time (in days) it will take for the fleet to reach its destination. To confirm
the move, click the checkmark. To cancel, click the X button."

## Object families

Slot `+4` of an object returns its family, the top byte of its id. The DAT
headers give the ranges (words 2 and 3 of each `*SD.DAT`):

| Families | File | Objects |
|---|---|---|
| `0x08..0x0f` | FLEETSD | Fleets |
| `0x10..0x13` | TROOPSD | Regiments |
| `0x14..0x1b` | CAPSHPSD | Capital ships |
| `0x1c..0x1f` | FIGHTSD | Fighter squadrons |
| `0x20..0x21` | ALLFACSD | hyp: facility placeholders (two records) |
| `0x22..0x27` | DEFFACSD | Defense facilities |
| `0x28..0x2b` | MANFACSD | Manufacturing facilities |
| `0x2c..0x2f` | PROFACSD | Production facilities |
| `0x30..0x37` | MJCHARSD | Major characters |
| `0x38..0x3b` | MNCHARSD | Minor characters |
| `0x3c..0x3f` | SPECFCSD | Special forces |
| `0x40..0x7f` | MISSNSD | Missions |
| `0x80..0x8f` | SECTORSD | Sectors |
| `0x90..0x97` | SYSTEMSD | Systems |

`FUN_004025b0(object)` returns the object's own id, built as
`family << 24 | +0x18`; `object[7]` is its container.

## The Fleet menu

The fleet class is vtable `0x0065d438` (constructor `FUN_004fd650`, factory
`FUN_004fe710`). Its `+0x3c` slot, `0x004fdad0`, copies a static list that
`FUN_004ff8e0` builds once in `DAT_006b2b10`: `0x201`, `0x202`, `0x204`,
`0x203`, `0x220`, `0x221`, `0x222`, `0x223`, `0x234`, `0x200`. Slot `+0x58`,
`FUN_004fdb00`, accepts a kind only when the list holds it (status `1`/`0x17`
otherwise). `FUN_0051d990` then adds Encyclopedia (`0x100`) and Status
(`0x103`) as for every selection (`object-popup-menu.md`).

The STRATEGY.DLL 26-byte `RT_RCDATA` records (`FUN_00442790`; words 0 kind,
1 parent, 2 sort, 4 submenu flag, 5 TEXTSTRA id, 10 second bitmap, 12 module):

| Kind | Parent | Sort | Submenu | TEXTSTRA | Text |
|---|---|---|---|---|---|
| `0x201` | — | 10 | 0 | 12312 | Move |
| `0x202` | — | 12 | 0 | 12311 | Confirmed Move |
| `0x120` | — | 200 | 1 | 12313 | Planetary Bombardment |
| `0x220` | `0x120` | 202 | 0 | 12314 | Target Military Facilities |
| `0x221` | `0x120` | 204 | 0 | 12315 | Target Civilian Facilities |
| `0x222` | `0x120` | 206 | 0 | 12316 | General Bombardment |
| `0x223` | `0x120` | 208 | 0 | 12317 | Destroy System |
| `0x234` | — | 220 | 0 | 12318 | Planetary Assault |
| `0x203` | — | 500 | 0 | 12291 | Rename |
| `0x100` | — | 1000 | 0 | 12292 | Encyclopedia |
| `0x103` | — | 1001 | 0 | 12293 | Status |
| `0x200` | — | 2000 | 0 | 12295 | Scrap |

`0x204` has no record, so it never becomes an item; it is the per-object move
(below). No record carries an icon (words 7 and 8 are zero). The sorted order
matches the manual's Fig. 3.64: Move, Confirmed Move, Planetary Bombardment,
Planetary Assault, Rename, Encyclopedia, Status, Scrap. The menu is built and
opened by the owning window as in `object-popup-menu.md` (`FUN_00442590`,
`FUN_00442380`); the system window's fleet list is that owner.

## The orders

`FUN_0051f4b0` registers the factories (`FUN_0051f930`): `0x201` is
`FUN_0053cec0` → `FUN_0053ce70` (vtable `0x00661850`), `0x202` is
`FUN_0053cf90` → `FUN_0053cf40` (vtable `0x006618a8`). Both are 0x4c bytes over
the move base `FUN_0053c040` (vtable `0x006617f8`, over the order base
`FUN_0051fa20`), which allocates a sub-order list at `+0x44` and a status pair
at `+0x48` (`FUN_0053c710`). The two vtables differ only in slot `+0xc`, the
kind (`FUN_0053cf30` returns `0x201`, `FUN_0053d000` returns `0x202`).

| Slot | Function | Role |
|---|---|---|
| `+0x10` | `FUN_0040f340` | Listed: always 1 |
| `+0x14` | `FUN_0051fd30` | Checked: always 0 |
| `+0x18` | `FUN_0053c100` | Enabled: `FUN_0053c4b0`, then every sub-order's `+0x18` |
| `+0x1c` | `FUN_0053c1a0` | Validator: `FUN_0053c4b0`, then every sub-order's `+0x1c` |
| `+0x20` | `FUN_0053c240` | Execute: every sub-order's `+0x20` |
| `+0x24` | `FUN_0051fb70` | Set the team (list at `+0x2c`) |
| `+0x28` | `FUN_0051fb80` | The team list (`this + 0x2c`) |
| `+0x2c` | `FUN_0051fbb0` | Set the target (`+0x34`) |

Statuses are two words, `(1, -1)` meaning success (`FUN_00520580` copies;
`object-popup-menu.md`, `mission-dialog.md`).

### Expansion into sub-orders

`FUN_0053c4b0` fails with `1`/`1` unless `+0x44` and `+0x48` exist, then:

1. `FUN_0053f150` resolves each team id through its class handler
   (`FUN_0053efa0`, slot `+0x2c`) into the objects that will move, without
   duplicates. An id with no handler gives `1`/`0x12`; no object at all gives
   `1`/`0x16`.
2. `FUN_0053c810` repeatedly takes the characters and special forces
   (`FUN_0053cbc0`, families `0x30..0x3f` resolved through `FUN_00504dc0`,
   `1`/`0x12` when one does not resolve) and builds one `0x241` order for
   them (`FUN_004f5440`, vtable `0x0065d170`, over the mission order base
   `FUN_004f4690`): side, team, and target copied from the move.
3. `FUN_0053ca50` builds one `0x204` order for the remaining objects
   (`FUN_0053d020`, vtable `0x00661900`, over `FUN_0051fa20`): side, team, and
   target copied from the move.

Each sub-order goes into `+0x44` (`FUN_00536fe0`).

### The per-object move `0x204`

| Slot | Function | Role |
|---|---|---|
| `+0xc` | `FUN_0053d0e0` | Kind `0x204` |
| `+0x18` | `FUN_0051fe20` | Enabled: built, then `+0x4c` |
| `+0x1c` | `FUN_0053d100` | Validator: `+0x4c` (`FUN_0051ff30`), then `FUN_00553aa0` |
| `+0x48` | `FUN_0053d3c0` | Create command `0x201` (`FUN_00578ab0`, vtable `0x00669558`) |
| `+0x4c` | `FUN_005201c0` | The command's checks through `FUN_005535b0` |
| `+0x50` | `FUN_0053d430` | Destination checks for the whole group |

`FUN_00553aa0` turns a status `1`/`0x25` or `1`/`0x27` into `1`/`0x26` when
the target is one of the team (`FUN_004f5940`).

`FUN_0053d430` resolves the destination (`FUN_00504e60`, `1`/`0x22` when it
does not resolve) and the side (`FUN_00553b80`). It then refuses `1`/`0x28`
when all of the following hold:

- the destination's side bits (`+0x24` bits 6..7) differ from the order's side;
- some member is neither a fleet (`0x08..0x0f`) nor a capital ship
  (`0x14..0x1b`);
- and either some member is not a regiment (`0x10..0x13`), or the
  destination is not an existing (`+0x50` bit `0x40`) system whose `+0x88`
  bit 0 (populated, `economy-systems.md`) is clear. (An earlier reading gave
  this as `+0x88` bit 1; the test is `& 1`, `regiment-unload.md`.)

So fleets and capital ships may move to an enemy or neutral destination; a
group holding anything else may not, except an all-regiment group landing on
an unpopulated system. A fleet-only move never meets this
refusal. Then it plans the route (`FUN_00551190`, `FUN_005513a0`, below) and
gives each member its leg (`FUN_00551630`, stored at the member's `+0x48`).
Any failure gives `1`/`1`.

### The move command `0x201` (vtable `0x00669558`)

The command holds the object at `+0x3c`, a route context at `+0x40`, and the
destination at `+0x48`.

- Can move, `+0x18` (`FUN_00578c00`):
  1. The object must resolve (`1`/`0x12`).
  2. The object's slot `+0x6c` must pass. For a fleet that is
     `FUN_004fdc70`, which runs the object check `FUN_004f9860`
     (`object-popup-menu.md`, "When Mission is enabled": side, untraced, en
     route, and the rest) and gives `1`/`1` on failure.
  3. `FUN_005152e0` → `FUN_005555e0` must pass. It refuses `0x90`/4 when an
     ALLFACSD object (`0x20..0x21`) sits in a system with `+0x88` bit `0x20`
     set, its blockade bit (`blockade-troop-withdrawal.md`), and
     `FUN_0055a020(ships, fighters)` over that system's active fleets comes
     to less than 31. It does not apply to a fleet.
- Validate with a destination, `+0x1c` (`FUN_00578d00`):
  1. The galaxy must exist (`FUN_00506e60`).
  2. The object must resolve (`1`/`0x12`), and so must the destination
     (`1`/`0x22`).
  3. The object's `+0x70` must pass (for a fleet, `FUN_004f6800`, which runs
     `+0x6c` again).
  4. The route must hold (`FUN_005542f0` → `FUN_00551190`, `FUN_00554490`).
  5. `FUN_00515390` → `FUN_00555920` must pass:
     - `FUN_00555410` → `FUN_00555460` reports whether the object and the
       destination lie in the same system (`FUN_00555540` on both);
     - in different systems, the object's speed slot `+0x34(1)` must be
       non-zero, or the result is `1`/`0x18`. For a fleet that is
       `FUN_004fd900`, zero when a member capital ship has no hyperdrive
       (`build-delivery.md`): the fleet cannot enter hyperspace;
     - in different systems, and only if no refusal is set yet, a regiment
       (`0x10..0x13`) whose destination's side bits differ from its own gets
       `1`/`0x28`; this does not apply to a fleet;
     - in the same system neither check runs (`regiment-unload.md`);
     - any other failure gives `1`/`1`.
- Execute, `+0x20` (`FUN_00578f30`): validate again, then `FUN_00515440` →
  `FUN_00556390(object, destination, ctx)`. For a deployed object
  (slot `+0x38`, `+0x50` bit 16) it asks whether the leg stays in one system
  (`FUN_00555410`) and, unless the object is autorouting (`+0x50` bit
  `0x800`), writes the in-transit bit (`+0x50` bit 5;
  `blockade-troop-withdrawal.md`, "Departure roll") through `FUN_004f7640`:
  set across systems, clear within one. Unless the object is destroyed
  (`+0x50` bit 3) it then changes container at once (slot `+0xa8`). An
  object that is not deployed changes container with no transit bit.
  Arrival follows the per-object transit in `build-delivery.md`.

So a fleet in hyperspace cannot take a new Move: `FUN_004f9860` refuses an
object with `+0x50` bit 4 (en route).

## Choosing Move or Confirmed Move

`FUN_004ac730` (the system window's `+0x20`) calls `FUN_0041cdf0(kind,
selection, no target, 0)`, which reaches `FUN_00486fb0`. For `0x201` and
`0x202`, `FUN_00487c50` builds the order (side `+0x20`, team `+0x24`,
`+0x2c`). With no target (top byte of the target id zero) the order goes to
`FUN_0041d5e0` → `FUN_00429320`: the galaxy view's targeting mode, cursor
1002, exactly as Mission (`object-popup-menu.md`, "Targeting").

## Hit tests

The galaxy view keeps its modeless child windows in the list at `+0x6c`.
A window's id is `(index << 6) | type` (`FUN_0045aac0`), and the drag path
reads the type as `+0x24 & 0x3f`. Each class answers two hit tests with a
point in its client coordinates: `+0x68` (the object under the point) and
`+0x70` (the container under the point, used for move orders).

| Type | Class | Vtable | `+0x70` | `+0x68` |
|---|---|---|---|---|
| 1 | Sector window, `FUN_004591d0` (opened by `FUN_00429ce0` from a click on a sector) | `0x00659f18` | `FUN_0045c830`: the item in list `+0x164` whose rectangle (`+0x40`) holds the point (`FUN_0045c660`), else no id | `FUN_0045c6b0`: the same item, or the overlay item in list `+0x174` (flag `0x10000`) whose bitmap pixel is hit (`FUN_005fca00`) |
| 9 | System window, `FUN_00452fc0` | `0x00659e68` | `FUN_004aa470`: the window's system (`+0x144`) wherever the point is | `FUN_00458b80`: the list item under the point (list `+0x174`, offset (8, 77)), else the system; on page `0x67` always the system |
| 4 | hyp: fleet window, `FUN_004a2630`, 235 by 304 | `0x0065bda0` | `FUN_004a3130`: the item under the point in the left list (4..95, 29..295, list `+0x160`) or, when `+0x194` is `0x66`, the right list (101..234, 127..291, list `+0x164`); elsewhere the window's subject `+0x144`, or the single selected item | `FUN_004a2f80` (not read) |
| 10 | `FUN_004a7790` | `0x0065be30` | `FUN_004aa470`: the window's subject `+0x144` | `FUN_004aa380`: the item under the point (list `+0x164`, offset (7, 81)), else the subject |
| 11 | `FUN_0049f130`, 235 by 304 | `0x0065bd10` | `FUN_004aa470`: the subject | `FUN_004a09a0`: the picture at (108, 37) gives `+0x1c8`, the list `+0x1c0` gives the item under the point, else the subject |

`FUN_0045aac0` opens types 9, 10, 4 and 11 for sector-window items of kind
`4`, `8`, `0x10` and `0x40` (item `+0xc >> 16`); hyp: kind 4 is a system and
`0x10` a fleet. Type 7 (`FUN_0049ee20`, vtable `0x0065bcb8`) has no hit-test
slots there.

The galaxy map is drawn by the galaxy view itself, not by a child: mode 1
finds the sector under a click with `FUN_004420b0`. A targeting release over
the bare map finds no child window (`FUN_00604540` fails for the view and its
parent), so without Shift the order is destroyed.

So, for a move:

- the bare galaxy map gives no destination; the order is dropped;
- a sector window gives the system under the point, and nothing between
  systems;
- a system window gives its own system wherever the release lands, even over
  a fleet in its list, so it never joins a fleet;
- a fleet window (hyp, type 4) gives the item under the point or the window's
  fleet, so a release there joins that fleet;
- the route then accepts only a system, a fleet or a capital ship as the
  destination (`FUN_005531b0`, below).

## The release

`FUN_00422ce0`'s `WM_LBUTTONUP` (0x202) in mode 2:

1. Finds the child window under the point (`ChildWindowFromPointEx`,
   `FUN_00604540`). With Shift held (`param_3 & 4`) it forwards the click to
   that window and stays in mode 2, as for Mission.
2. For orders `0x201`, `0x202` and `0x214` it asks the child's vtable `+0x70`
   for the container under the point; other orders ask `+0x68` (Hit tests,
   above).
3. When the id is non-zero and valid (`FUN_004ece60`), it sets the target
   (`+0x2c`) and calls `FUN_0041ce20(order, 0)` → `FUN_00436050` →
   `FUN_00487740(order, 0)`. Otherwise the order is destroyed.
4. Mode returns to 1 and the capture is released.

`FUN_00487740(order, force)`:

1. Runs the validator `+0x1c`. A failure goes to `FUN_00487c90`, the side's
   advisor reaction (`mission-dialog.md`, "Refusal"), and destroys the order.
2. With `force` 0, `FUN_00487cc0` decides whether to confirm. It confirms when
   the validator passes and the kind is `0x202`, or `0x200`, `0x213`, `0x242`
   or `0x250`. For `0x201` it confirms only when the first team member's
   container is a system whose `+0x88` bit `0x20` (blockade) is set and whose
   side bits equal the member's. Confirming hands the order to `FUN_0048a340`
   (`+0x6c`, below).
3. Otherwise `FUN_00488030` plays the acknowledgement for capital-ship targets
   (families `0x30..0x37`, sound by class through `FUN_004c4990`), and
   `FUN_0048aa90` → `FUN_0041cee0` → `FUN_004360f0` submits the order for its
   side.

So Move departs at once unless the fleet runs a blockade, and Confirmed Move
always asks.

## The confirmation window

`FUN_0048a340` builds a 0x60-byte confirmation (`FUN_0049a1b0`, vtable
`0x0065bbd0`, over `FUN_004c4d90`) holding the order at `+0x5c`, and
`FUN_0041d7b0` → `FUN_0042a590` posts `0x468` with type `0x10` to the galaxy
view. The view's `0x468` handler builds the window `FUN_0044f060` (0x124
bytes, vtable `0x00659cd0`, `FUN_00606380` with size 0x1a8 by 0x14b, 424 by
331).

`FUN_0049a350` fills the confirmation for `0x201`/`0x202`:

- Its text `+0x50` is TEXTSTRA `RT_RCDATA` 0x7057, "Transit time in days".
  When the first member is not a fleet (families `0x08..0x0f`) and its
  container is a blockaded system (`+0x88` bit `0x20`), it is 0x7056 instead
  (so a fleet's move never shows it, even when a blockade made it ask): "Units evacuating from worlds under blockade risk being
  destroyed by blockading vessels.  Are you sure you want to proceed with
  the evacuation?\nTransit time in days". `FUN_0060b840` loads these templates
  from module 2.
- Its picture id `+0x2e` is 1018 for side 1 and 1019 otherwise, or 1031 for
  side 1 and 1030 otherwise when blockaded.
- `FUN_0049a8b0` appends one line per moving object: its name
  (`FUN_004f6270`), `":  "` (`DAT_006a8798`), and its transit days, which
  `FUN_0053c2e0` collects from the sub-orders (`FUN_0053d200` for `0x204`,
  `FUN_004f4850` for `0x241`). `FUN_00615f00(stream, 10)` puts a newline
  before each line; the same call with 0 ends the string.

`FUN_0044f180` (slot `+0x38`) lays the window out:

| Element | Rect or position | Source |
|---|---|---|
| Background | full window | STRATEGY 11125 when the galaxy view's side `+0x9c` is 1, otherwise 11126 |
| Placement | in the galaxy view's rectangle | `FUN_00606980` with the view's `+0xcc..+0xd8` |
| Text box | (24, 242) 322 by 70 | `FUN_0041ecf0`, the confirmation's `+0x50` |
| Title | (21, 14) | `FUN_00601880`, the confirmation's `+0x44` (empty for moves) |
| Checkmark, control `0x14` | (355, 244) 51 by 35 | `CoolStrobeButton`, STRATEGY 10926 / 10927 |
| X, control `0x15` | (355, 281) 51 by 35 | STRATEGY 10929 / 10930 |
| Picture | (12, 30) | STRATEGY `+0x2e`: 1018, 1019, 1030 or 1031 |

All these bitmaps are staged in `data/base/ui/strategy-dll/BMP/`.

The window's command slot `+0x48`, `FUN_0044f5e0`, handles the two controls:

- Control `0x14` (the checkmark) calls `FUN_0041ce20(order, 1)`. The order
  passes through `FUN_00487740` again with `force` 1: it is validated (a
  refusal still reaches the advisor) and submitted without asking.
- Control `0x15` (the X) destroys the order.

Either way the window closes (`+0x30`). Its key slot `+0x4c`, `FUN_0044f640`,
maps Enter (0xd) to `0x14` and Escape (0x1b) to `0x15`.

## A drag is a move

`FUN_006083c0` posts notification `0x29a` when a list drag ends outside its
list; the galaxy view (`FUN_00422ce0`, default `WM_COMMAND` path) asks the
source window for its selection (`+0x58`), hit-tests the drop point with
`+0x68` and `+0x70`, and issues `0x201`, or `0x202` with Ctrl held
(`GetAsyncKeyState(0x11)`), through `FUN_0041cdf0(kind, selection, target, 0)`:

- Window type 1 (`+0x24 & 0x3f`): members whose list entry has flag 4 are
  split off into a `0x214` order (Destination) against the `+0x68` object;
  the rest move against the `+0x70` object.
- Types 4 and 10: the whole selection moves against the `+0x70` object.
- Type 9: a `0x214` order against the `+0x70` object.

With a target, `FUN_00486fb0` skips targeting and goes straight to
`FUN_00487cc0` (confirm) or `FUN_00487740(order, 0)`.

## Route refusals

The route context (`FUN_00551060`) is set up by `FUN_00551190` →
`FUN_00551100` for the order's side and checked by `FUN_005513a0` (and
`FUN_00551270` per object, `FUN_00554490`). A fleet move refuses to route
when:

- the destination does not resolve, `1`/`0x22` (`FUN_00552e80` →
  `FUN_00553130`);
- the destination's container does not resolve, `1`/`0x23` (`FUN_00552ff0`,
  `FUN_0050c640`);
- the destination is not a system (`0x90..0x97`), a `0x98..0x9f` object, a
  fleet (`0x08..0x0f`) or a capital ship (`0x14..0x1b`), `1`/`0x25`
  (`FUN_005531b0`);
- `FUN_00553aa0` turns `0x25` or `0x27` into `0x26` when the destination is
  one of the team: a fleet cannot move into itself.

Each leg is then built by `FUN_00552d10` (`FUN_00550620`, `FUN_00550700`) and
assigned by `FUN_00551630` (`FUN_005529a0`, `FUN_00552000`, `FUN_00552300`,
`FUN_00552dd0`); those refusals are not read here.

## Escape while targeting

The galaxy view's `WM_KEYDOWN` handles Tab and F1..F7 (`0x70..0x76`) but has
no `VK_ESCAPE` case. The cancel command `0x15e` is posted only by
`FUN_0042db70` (through `FUN_0041db60`), when the global gate
(`FUN_0051de80`, `FUN_0041e390`, `FUN_0041e500`) stops orders. The manual says
Escape cancels Move. hyp: a frame-level key handler maps Escape to `0x15e`;
not found. The confirmation window's own Escape is recovered
(`FUN_0044f640`).

## Port notes

- The port already departs a fleet through `movement::validate_fleet_dispatch`
  and `begin_fleet_transit` (F-007C) with the per-object transit time
  `fleet_transit_ticks` (F-030, `build-delivery.md`). Compare their refusals
  with `FUN_004f9860`, `FUN_005555e0`, `FUN_00555920` and `FUN_0053d430`
  before reusing them.
- The refusal statuses above reach the advisor as in `mission-dialog.md`,
  "Refusal" (for example `1`/`0x18` and `1`/`0x22`).
- Phase 1: a right release on a fleet in the system window's Fleets tab
  opens the Fleet menu (`object_menu.rs`, `MenuObject::Fleet`) with the
  STRATEGY rows above. port: every fleet order is drawn disabled until the
  move order and its targeting are ported; Encyclopedia opens as for any
  single selection.
- Phase 2: `movement::fleet_move_enabled` is the rows' `+0x18` (side, not en
  route); `validate_fleet_dispatch` adds the destination (`1`/`0x22`) and the
  speed refusal (`1`/`0x18`). port: its origin, destroyed-destination,
  same-system and empty-fleet checks are the port's own.
- Phase 3a: every targeting release asks the topmost window under the point
  (`targeting::release_destination`): a system window's `+0x70` gives its
  system, a sector window's the planet whose 37 by 37 rectangle holds the
  point (`sector-window-hit-test.md`), and the bare map nothing. Mission
  uses it too.
- Phase 3b: a fleet's Move and Confirmed Move start targeting; the release
  validates (refusals go to the message log, `port:`), then
  `movement::fleet_move_confirms` applies `FUN_00487cc0`, then the fleet
  departs. A move that asks for confirmation is dropped until the window
  (phase 4) is ported.
- Phase 4a: `move_confirmation` draws the window: background 11125/11126,
  picture 1018/1019, the 0x7057 text with one `name:  days` line, the
  checkmark and X, and Enter and Escape. hyp: font 4 is not mapped, so the
  mission dialog's text size stands in; the placement shares the mission
  dialog's centering.
- Phase 4b: a move that asks opens the window with one line, the fleet's
  label and its days (`fleet_transit_ticks`). port: fleets have no names yet,
  so the line uses the system window's "Fleet N" label. The checkmark
  dispatches the fleet, which validates again; the X and Escape drop the
  order, and the galaxy view's own Escape stands aside while it is open.
- Phase 5a: `validate_fleet_destination` is `0x214`'s check for a fleet:
  the move's refusals (hyp) plus the blockade refusal of step 7 below, with
  no confirmation and no enemy-destination refusal.
- Phase 5b: a left press on a system window item holds a drag until the
  left release, and the galaxy map ignores the pointer meanwhile (the
  list's capture). A release past `sqrt(24)` list pixels and outside the
  list hit-tests the window under it as targeting does (sector window: the
  system under the point; system window: its system; bare map: nothing),
  validates, and dispatches the fleet at once. Ctrl does nothing. port: only
  a fleet's drop moves (characters and special forces are not ported), a
  drop on its own system window meets the port's same-system refusal, and
  the drag image and cursor are not drawn.
- Phase 7: `system_is_blockaded` follows `FUN_0050b8e0` (`blockade-bit.md`).
  `fleet_move_confirms` keeps the `0x201` blockade branch; like the original's
  fresh state it never fires, because a holder's fleet in the system turns
  an enemy fleet into a battle. The original meets it only on a stale bit,
  which the port, recomputing each tick, never holds.

## Order 0x214 (Destination)

`FUN_0051f4b0` registers `FUN_00537090` for kind `0x214`. `FUN_00537090`
allocates 0x44 bytes and calls `FUN_00537040`, which chains the order base
`FUN_0051fa20` and sets vtable `0x00661040`. The kind slot at `+0xc`
(`0x00537100`) is `mov eax, 0x214; ret` (not a named function in Ghidra).

| Slot | Function | Role |
|---|---|---|
| `+0xc` | `0x00537100` | Kind `0x214` |
| `+0x10` | `FUN_0040f340` | Listed: always 1 |
| `+0x14` | `FUN_0051fd30` | Checked: always 0 |
| `+0x18` | `FUN_0051fe20` | Enabled: built, then `+0x4c` |
| `+0x1c` | `FUN_0051ff30` | Validator: enabled, then `+0x50`, then per-command `+0x1c` (`FUN_00553960`) |
| `+0x20` | `FUN_00520040` | Execute: validate, then submit commands (`FUN_005539f0`) |
| `+0x24` | `FUN_0051fb70` | Set the team (list at `+0x2c`) |
| `+0x28` | `FUN_0051fb80` | The team list (`this + 0x2c`) |
| `+0x2c` | `FUN_0051fbb0` | Set the target (`+0x34`) |
| `+0x48` | `FUN_00537110` | Create command (vtable `0x00669a30`, 0x58 bytes, `FUN_0057ddc0`) |
| `+0x4c` | `FUN_005201c0` | Command checks: `FUN_005535b0` |
| `+0x50` | `FUN_00537180` | Destination checks |

Unlike `0x201`/`0x202`, the order extends the order base directly (0x44
bytes), not the move base (0x4c bytes). It has no sub-order list; instead
`FUN_005535b0` creates one command per resolved team member (`FUN_0053f150`),
each holding its object at `+0x3c`, and adds it to the command list at
`+0x40`.

### Destination checks (`FUN_00537180`)

`FUN_00537180` (slot `+0x50`) runs after `FUN_005202d0` confirms the order is
built:

1. `FUN_00553b80` resolves the side from the team.
2. `FUN_00504e60` resolves the destination (`1`/`0x22` when it does not
   resolve).
3. Iterates the commands (order `+0x40`, each object at command `+0x3c`,
   `FUN_004fa150`) and, for each object, its child list `FUN_00528820`,
   clearing a flag when any child is not a capital ship (`0x14..0x1b`).
   `FUN_00528820` returns an empty static list for an object with a side
   (`+0x24 & 0x30`) and `+0x54`→`+0x18` only for one without. A player's
   fleet has a side, so the flag stays set.
4. Refuses `1`/`0x28` when the destination's side bits (`+0x24` bits 6..7)
   differ from the order's side and the flag was cleared. There is no
   regiment exception, unlike `FUN_0053d430` for `0x204`, but for the reason
   above it never refuses a sided fleet.
5. Routes via `FUN_00551060`, `FUN_00551190`, `FUN_005513a0`.
6. Per command whose object's `+0x58` is zero:
   - resolves the destination again through `FUN_00504e60` (`1`/`0x22`);
   - an object of family `0xa0..0xa2` calls `FUN_00553410` on the
     destination;
   - otherwise the checks read the **destination**: `+0x50` bit 2 clear gives
     `1`/`0x20`, bit 4 set `1`/`0x21`, bit 3 set on a non-system `1`/`0x22`;
     the destination must be a system (`0x90..0x98`), a fleet (`0x08..0x10`)
     or a capital ship (`0x14..0x1c`), else `1`/`0x25`;
   - assigns the destination to the command's `+0x48` (`FUN_004f26d0`).
7. Per command whose object's `+0x58` is non-zero: `FUN_00552210` asks
   `FUN_005287f0` for the object's `+0x54`→`+0x18` list, which fails for an
   object with a side, so the check returns 0 and the order is refused with
   `1`/`1`. Otherwise `FUN_00552150` builds a leg per listed object
   (`FUN_00552000`) and iterates them with `FUN_00551900` and
   `FUN_00583f50`. For a fleet, `+0x58` bit 5 is the system's blockade bit,
   copied by `FUN_0050c0b0` to every active fleet in the system
   (`blockade-troop-withdrawal.md`). hyp: a fleet's `+0x58` is otherwise zero
   outside combat, so a drag refuses exactly a fleet in a blockaded system,
   on either side.

### The command (vtable `0x00669a30`)

`FUN_00537110` allocates 0x58 bytes. `FUN_0057ddc0` chains the command base
`FUN_0054f2c0`, initialises a list at `+0x50` (`FUN_00583e80`), and sets
vtable `0x00669a30`.

| Slot | Function | Role |
|---|---|---|
| `+0x18` | `FUN_0057e000` | Can move: resolves object `+0x3c`, calls object's `+0x1e0` |
| `+0x1c` | `FUN_0057e0d0` | Validate: resolves object and destination `+0x48`, routes, calls object's `+0x1e4` |
| `+0x20` | `FUN_0057e380` | Execute: resolves object, calls object's `+0x200` |

For a fleet (vtable `0x0065d438`), slots `+0x1e0` and `+0x1e4` are both
`FUN_00524fb0`, which returns 1 unconditionally. Slot `+0x200` is
`FUN_004ff7a0`:

1. `FUN_0053a010`: checks `object+0x24 & 0x30` (has a player side).
2. `FUN_004f8880`: logs `"FleetBattleNotif"` / `"Battle"` (debug
   notification, not game logic).
3. `FUN_0053fdd0(0x180, fleet, route_ctx, list)`: creates a `0x180` game
   event through a separate factory (`FUN_0051f730`, table at
   `DAT_006b2fdc`).

`FUN_005400f0` populates the event: side from the fleet's `+0x24` bits 4..5,
the fleet id at `+0x20`, and route data from the list's `+0x14`/`+0x18`.
`FUN_004fd3b0` submits it (indirect call through `DAT_006b2ad4`). hyp: the
`0x180` event is the fleet departure that sets the transit bit and enters
hyperspace, analogous to `FUN_004f7640` in the `0x201` path.

### Confirmation

`FUN_00487cc0` has no case for `0x214`; it falls to `default` and returns 0.
The order never confirms, even when the fleet's system is blockaded. A system
window drag departs immediately, where a menu Move (`0x201`) under blockade
would show the evacuation warning.

### STRATEGY record

None. `0x214` is never a menu item; `FUN_00442790` will not find a record for
it. It is issued only from the drag path (`FUN_00422ce0`'s `0x29a` handler,
window types 1 and 9).

### How the drag issues it

From "A drag is a move" above:

- Window type 9 (system window): always `0x214` against the `+0x70` object.
  Ctrl has no effect; there is no confirmed variant.
- Window type 1 (sector window): members whose list entry has flag 4 are split
  into a `0x214` order against the `+0x68` object; the rest get `0x201` or
  `0x202`.

Because the order carries a target, `FUN_00486fb0` skips targeting and goes
straight to `FUN_00487740(order, 0)`.

### Port notes

- The port's list drag path should issue `0x214` for system-window drags, not
  `0x201`. The difference: `0x214` never confirms (no blockade warning on
  drag), creates per-object commands directly (no sub-order split into `0x204`
  and `0x241`), and departs through a `0x180` game event rather than
  `FUN_004f7640`. hyp: the `0x180` path may handle the departure identically;
  this needs verification before collapsing the two.
- Ctrl+drag from a system window is still `0x214`, not `0x202`. The Confirmed
  Move variant applies only to types 1, 4 and 10.
- The enemy-destination refusal (`1`/`0x28`) lacks `0x204`'s regiment
  exception, and never refuses a sided fleet (step 3 above).
- A fleet in a blockaded system is refused (`1`/`1`, step 7); a menu Move
  from the same system would open the evacuation warning instead.
- `CoolDragList` (`FUN_006083c0`) posts `0x29a` on a left release more than
  `sqrt(24)` pixels from the press (squared distance above `0x18`) and
  outside the list's client rect, or anywhere when list flag `+0xf4` bit
  `0x100000` is clear (hyp: set for the system window's list). While dragging it draws the item through
  `FUN_0060dc80`/`FUN_0060dcb0`/`FUN_0060dce0` and sets cursor `+0xb4`; those
  are not read.

## Characters and special forces (0x241)

A character's Move is a Move mission: the `0x241` order makes command `0x250`
(mission create, `FUN_0054cd80`) for mission `0x41000001`, which the mission
machinery of `mission-lifecycle.md` then runs. The player and the AI
(`FUN_004bc040` builds a `0x201` for one character, side `+0x24` of the task,
target from its caller) take the same path.

### The split

`FUN_0053cbc0` takes the first character or special force of the move's team
that has `+0x50` bit 2 set (families `0x30..0x3f`), reads its location (slot
`+0xc`), and takes every later such member at the same location. The loop in
`FUN_0053c4b0` builds one `0x241` per distinct location (`FUN_0053c810`) until
none is left, so a selection spread over several systems becomes several
orders that share one target. A member with bit 2 clear stays in the remainder
and joins the `0x204`. Each `0x241` gets the move's side (`+0x1c`, `+0x20`),
the target (`+0x34`, slot `+0x2c`), `+0x44` = `+0x48` = 0, the mission id
`+0x4c` = `0x41000001` (`FUN_0053c810` at `0x0053c95b`) and an empty decoy
list `+0x58`.

### Vtable `0x0065d170` (`FUN_004f5440`, `0x60` bytes, factory `FUN_004f5490`)

| Slot | Function | Role |
|---|---|---|
| `+0xc` | `FUN_004f5500` | Kind `0x241` |
| `+0x10` | `FUN_006158b0` | Listed: 0 (mission `0x240` lists, `FUN_0040f340`) |
| `+0x14` | `FUN_0051fd30` | Checked |
| `+0x18` | `FUN_0051fe20` | Enabled: built, `+0x4c`, then `FUN_00553770` |
| `+0x1c` | `FUN_0051ff30` | Validator: Enabled, `+0x50`, then `FUN_00553960` |
| `+0x20` | `FUN_00520040` | Execute: validate, `FUN_005539f0` runs the commands |
| `+0x24`..`+0x30` | `FUN_0051fb70`, `FUN_0051fb80`, `FUN_0051fbb0`, `FUN_0051fbc0` | Set and get the team (`+0x2c`) and the target (`+0x34`) |
| `+0x48` | `FUN_004f4990` | Create command `0x250` (vtable `0x00661e28`, `0x74` bytes) |
| `+0x4c` | `FUN_004f4a00` | Fill the command and add it to the list at `+0x40` |
| `+0x50` | `FUN_004f4b60` | Destination, route and container check |

Slots `+0x14..+0x50` are the functions of mission order `0x240` (vtable
`0x0065d118`); only the destructor, the kind and Listed differ.

### Refusals

Enabled (`FUN_0051fe20`, `FUN_00553770`, command slot `+0x18` =
`FUN_0054d1a0` → `FUN_005429e0`), with no target:

- `1`/`1` for any sub-check that sets no code; `1`/`0x12` when the actor
  (`0xf8000006`, `FUN_00506850`) or a member does not resolve; `1`/`0x16` with
  no command; `1`/`0x14` when two commands carry different sides.
- `0x40`/`0x91` when no member is left after `FUN_0054bb90` moves prisoners
  (slot `+0x1d4`, `+0xac` bit 0) to the captured list, which travels with the
  team (so a prisoner alone is refused, one with a free officer is carried).
- `0x40`/3 when the located members differ in location, `1`/`0x16` when none
  has one (`FUN_0054bf00`; the split above makes the first impossible for a
  Move).
- `0x30`/n per member (`FUN_0054c110` → slot `+0x1c8`: `FUN_004ed560` for a
  character, `FUN_00533ce0` for a special force, over `FUN_004f9860`): wrong
  side (1), on a mission (`+0x78` bit 7, 1), destroyed (`+0x50` bit 3, 3),
  en route (`+0x50` bit 4, 4), the base object's own refusal (5). An
  unrecruited member (`+0x50` bit 2 clear) is allowed for `0x241`, and so are
  an injured one and a prisoner: only `0x240` refuses those.

Destination (`FUN_004f4b60`), after the above:

- `1`/`0x12` for the first member, `1`/`0x14` when its side is neither 1 nor 2,
  `0x40`/`0x92` or `0x40`/`0x93` for a missing mission record (never for id
  `0x41000001`).
- `0x40`/`0x20` when the order's side holds no record of the destination
  (`FUN_004f2d10`, the side's view of it).
- `1`/`0x28` when the destination's side bits (`+0x24` bits 6..7, in that
  view) differ from the mover's. The test runs only for mission ids in
  `0x40..0x4f`, which is why Move has it and Mission `0x240` does not
  (`FUN_00542b60` makes the same split: family `0x40..0x4f` for `0x241`,
  `0x50..0x7f` for `0x240`, else `1`/`1`).
- Route (`FUN_005513a0`): `1`/`0x22` destination unresolved, `1`/`0x23` its
  container unresolved, `1`/`0x25` not a system (`0x90..0x97`), a `0x98..0x9f`
  object, a fleet (`0x08..0x0f`) or a capital ship (`0x14..0x1b`)
  (`FUN_005531b0`).
- Acceptance by the candidate container (`FUN_00550700` asks slot `+0x74`
  for the character range `0x30..0x40`, then room at `+0x78`):
  - system (`FUN_005073d0`): `1`/`0x22` destroyed (not lifted for `0x241`),
    `1`/`0x24` not of the mover's side, `1`/`0x28`, `1`/`0x20` not completed
    (`+0x50` bit 2 clear), `1`/`0x21` en route (`FUN_00553410`). The
    blockade refusal `0x90`/4 is lifted for `0x241` characters. Room is
    unlimited (`FUN_00507750`);
  - capital ship (`FUN_00500ac0`, room `FUN_00500b40`: unlimited for
    `0x30..0x40`): the same side, completed, destroyed and en route refusals
    (`FUN_00553410`);
  - fleet (`FUN_004fe0d0`, `FUN_004fe160`): `FUN_00553410` with its own
    completed test (`+0x58 & 6`); its room is for capital ships only, so a
    character never boards the fleet itself.

Creation (the command's `+0x1c` = `FUN_0054d280` → `FUN_00542b60`, run again
at execute): the checks above again, then `1`/`0x14` for two sides, `1`/`0x16`
for none (`FUN_005830a0`), `0x40`/`0x21` when the side's object is missing, and
the Move class's rules through the mission rule table (`FUN_0054cb70`:
resolver `FUN_00523670`, validator `FUN_00523450`): `0x40`/1 when a member is
outside the record's masks, `0x40`/`0x13` when the container is en route,
`0x40`/`0x12` when it is destroyed (record column 11 is 1), `0x40`/`0x23` when
the target's side is not the mission's own (columns 15 own 1, 16 other 0, 17
opponent 0). The Move record (MISSNSD id 1, family `0x41`) has special-force
mask `0xf0f`, character bit `0x10000`, both sides, timer minimum and spread 0,
hidden 1, detection column 1, resign 0, column 11 1, column 13 0, column 14 1.
Special force `0x3c000009` (mask `0x1000`) is outside `0xf0f` and cannot
Move; the Return record (`0x42000002`) takes `0x1f0f`.

So the destination must be a system, fleet or capital ship that the mover's
side knows, that is its own side's (the order's `+0x24` bits 6..7), not
destroyed, completed and not en route. An enemy or neutral system, and a
system held by neither side, are refused (`1`/`0x28`, `1`/`0x24`); an own-side
system is accepted whatever the side keeps there. A fleet or ship is a
destination only when it is the mover's side's.

### The command and the member's trip

`FUN_004f4990` builds command `0x250`; `FUN_004f4a00` copies the team
(`+0x64`), the decoys (`+0x6c`) and the order's `+0x44`, `+0x48` (`+0x50`,
`+0x54`) into it with actor `0xf8000006` at `+0x3c`; `FUN_004f4b60` writes the
side (`+0x58`), the mission id (`+0x5c`), the chosen container (`+0x60`) and
the container id (`+0x48`). The route builder picks the container
(`FUN_00552300`, `regiment-unload.md`, "Choosing the container"): the
destination when it accepts, else for a fleet the first ship that does, for a
system also the first fleet of the side there. A tie keeps the first
(`FUN_0054fbb0`: only a strictly better candidate replaces). A character's
room is unlimited, so a system destination takes the character itself and a
fleet destination puts it aboard its first acceptable capital ship.
Executing runs `FUN_0054d360`: the creation checks, then the mission
manager's slot `+0x1bc` (`FUN_00549bf0` → `FUN_00547ba0`) builds the Move
mission (class vtable `0x00665578`, ctor `FUN_0056ea00` over the transit base
`FUN_00593730`, family `0x41`; phase hook `FUN_00524b70`). Origin `+0x6c` is
the team's location, target location `+0x78` the container's system; `FUN_00547ba0`
fails unless both are systems (`0x90..0x9f`). The command's `+0x48` becomes the
mission's target `+0x70` and its `+0x60` the container `+0x74`
(`FUN_00521ab0`, `FUN_00521b60`). hyp: both name the chosen container, since
`FUN_004f4b60` writes `+0x48` from the chosen id unless that id is the
destination's own.

Each member is added by the mission's slot `+0x1cc` (`FUN_00522b30`): the
mission key at `+0x68` (`FUN_00534230`, which makes `+0x78` bit 7 "on a
mission" through `FUN_00534950`), the hidden bit `+0x78` `0x100`
(`FUN_00534870`, record column 7) and the decoy bit. Phase 4
(`FUN_00524b70`, `mission-lifecycle.md`) then runs for each member:

1. `FUN_00556430(member, origin, target location)`: nothing when both are the
   same system; otherwise the transit of `build-delivery.md`, "Travel" (en route
   active `+0x50` bit 5, autorouting bit 11, event `0x387` at `clock + ticks`);
2. `FUN_00556390(member, container)`: the in-transit bit is set across
   systems and cleared within one (skipped when bit 11 is already set), then
   slot `+0xa8` puts the member into the container at once
   (`FUN_00514a60` calls `FUN_00556430`, which returns at once with bit 11
   set).

So a character sits in the destination system's container, or aboard the chosen
ship, from the tick of the order, flagged en route until the event `0x387`
fires (`FUN_004f7fc0`, `FUN_004fb520` clear bits 5, 11 and 12). Arrival
reaches the mission through `FUN_00515a50` → `FUN_00545820` →
`FUN_00522280`, which refreshes the target location and sets the ready bit;
phases 5 and 6 run the validator (6 clears each member's en route bit),
phase 8's timer `0x38b` has delay `0 + rand(0..=0)`, which is due the day it
is armed (`timer-scheduler.md`, `FUN_0054edc0`), and phase `0xb` ends with
code 1, all within the arrival day. The original shows it: Noghri Death
Commandos ordered from Coruscant to Uvena (4 days) arrive on day 27 with the
pop-up rows still disabled, and later that day every row is enabled
(`.artifacts/original-checks/2026-10-15-move-timer`). The members stay in the container.
A same-system Move therefore lands the character on the day of the order, with
no transit.

If the chosen container is destroyed, the validator, which `FUN_00545240` runs
at once, ends the mission with code 7 (`0x40`/`0x12`, column 11 = 1). If the
target is destroyed, en route or no longer in its container, code 6
(`0x40`/`0x21`, `0x22`, `0x20`; column 14 = 1, `FUN_00523450`,
`FUN_00592600`). A side change of the target ends it with code 8
(`0x40`/`0x23`). The validator runs on entering phases 4, 5, 6 and `0xb`, on leaving 8 and on a
destruction, not on every move of the ship, so a ship that leaves for another
system after the order takes the character with it (the character is its
content) and only a later phase check can end the mission. hyp: what happens
to characters aboard a ship destroyed in hyperspace (the Death Star rule
turns characters adrift, `death-star-strategic.md`).

### Who reads the en route bit

A moving member is a child of its destination container from the order,
with `+0x50` bit 4 set while its leg lasts (`FUN_004f8240`: bit 5, set only
by `FUN_004f7640`, or the container's bit 4). The child-list iterator
`FUN_005131d0` (via `FUN_004f6b90`, `FUN_004f2640`, `FUN_00536e20`) has
mode 1, usable (existing, completed and not en route; `0x004f7b80`), mode 2,
en route only, mode 3, existing, and mode 4, all. Whether a reader sees the
member depends on its mode or its own test:

| Reader | Function | En route member |
|---|---|---|
| Personnel lists | `FUN_004a90d0`, `FUN_004a7f50`, `FUN_004a9ce0`, `FUN_00512700` (mode 3) | listed at the destination |
| Personnel icon | `FUN_004a9ab0` → `FUN_0042c3b0` | STRATEGY 11501 under the mini, in place of its 11500 backdrop, while bit 4 is set (`fleet-window.md`, "En route marks"); a special force's portrait takes 11520 (classes `0x3c000003`, `0x3c000005` take the GOKRES craft marks) |
| Move, Confirmed Move, Mission, Retire, Command | `FUN_00533ae0`, `FUN_004ed560` → `FUN_00533ce0`, `FUN_004ed750` → `FUN_004f9860` | disabled (status 1/4) |
| Mission target, rule 5 | `FUN_00523450` | ends the mission (end 6) |
| Container acceptance | `FUN_00553410` | refused, `0x21` |
| AI agent records and tasks | `FUN_00402230` (flag `0x20`), `FUN_004bba90` | not used |
| Uprising incidents 3-5, regiment counts, mission defenders, Pickup, Palace, Force discovery candidates, story "free character" tests | `FUN_0050d150`, `FUN_005091f0`, `FUN_00587640`, `FUN_005445d0`, `FUN_0056da80`, `FUN_00560a30`, `FUN_005439b0` (mode 1 or `+0x50` bit 0) | skipped |
| The system changing hands | `FUN_00510820` → `FUN_004fae40` (mode 3) → slot `+0xb0`: `FUN_004ef260` character, `FUN_00503d90` special force | handled as present: skipped only on a mission or destroyed |
| Its container destroyed | `FUN_004fb520` → slot `+0xb4`: `FUN_004ef2e0` character | handled as present |
| Counter-intelligence officer by rank | `FUN_00509330`, `FUN_004fd790` → `FUN_005006f0` (mode 3) | counts at the destination |
| Vader, Luke and the Emperor together | `FUN_00544a20`, `FUN_00544be0`, `FUN_00544fc0` (container ids) | counts at the destination |

The retreat chain moves no character (`FUN_00515480` runs only the story
hook `FUN_00545df0` for one).

### Travel time

The original gives the same number in the confirmation window and for the
trip: `ticks = 0` when the origin and destination locations are one system,
`1` when they differ and either is not a system, else
`max(1, ((isqrt(dx² + dy²) / GNPRTB[5120]) * speed) / 100)`
(`FUN_00555b30`, `FUN_0055d8c0`, `build-delivery.md`, "Transit arithmetic";
GNPRTB 5120 is 5). `speed` is a percent: 100 (GNPRTB 1) by default, GNPRTB
3083 (shipped 50) when Han Solo (`0x33000243`) is a member and no special force
is (`FUN_00542990`). The window computes it from the order's lists
(`FUN_005425e0`: Han in the team or decoys, any family `0x3c..0x3f` member among
all three lists); the trip reads each member's `+0x9a` (`FUN_004ed370`), which
phase 2 sets with the same rule (`FUN_00548370`, `mission-lifecycle.md`). The
distance is between the origin system and the destination's system (a ship's
location is its fleet's system). A ship's hyperdrive does not enter (the
character's own `+0x9a` does), nor does a side factor or a skill. A special
force travels at 100.
The port's mission transit (`missions.rs`, `MemberTransit`,
`GNPRTB_HAN_SOLO_SPEED`) is this rule. The member stands in its destination
container from the order on, with its `en_route` flag (`Character`,
`SpecialForceUnit`) set until the transit's arrival day clears it; the
readers follow "Who reads the en route bit".

### Rank, flags

The container change reaches `FUN_00514a60` → `FUN_00560d50`, which clears the
character's rank `+0x96` (`FUN_004ee350(character, 0)`) at the tick of the
order, unless it moves between two capital ships of one fleet. The Move also
starts `FUN_005457f0` (the story-flag check for the named majors). It requires
`+0x50` bit 4 clear (en route) and bit 3 clear, `+0x78` bit 7 clear, and
`+0x50` bit 2 only for the `0x204`; it sets the mission key `+0x68` (so bit 7),
the hidden bit, bits 5 and 11 across systems, and `+0x3c`/`+0x44` of
`build-delivery.md`. The decoy and detection manager (`FUN_00547f60`) runs on
every phase change of a record with column 8 set, which Move's is; its mode
word `0x4112` (`FUN_005236e0`) is the one every class returns, so a Move's
members face the destination system's detection as any mission's do.

### Enabled for a selection

The Move and Confirmed Move rows of a character selection are enabled when the
global gate `FUN_0051de80` passes and `FUN_0053c100` (`FUN_0053c4b0`, then
every sub-order's `+0x18`) passes. For each `0x241` group that means: every
member resolves, is of the side, is not on a mission, travelling or destroyed
(`0x30`/n), and a free member remains. An injured one is enabled; a prisoner
alone is not. The row ignores the target.

### Confirmation line

`FUN_0049a8b0` prints, for each node `FUN_0053c2e0` collects from the `0x241`
(`FUN_004f4850`: validate, then `FUN_005425e0` stores the days at the node's
`+0x1c`), the name of the object from the side's view (`FUN_004f2d10`,
`FUN_004f6270`), `":  "` and the days. The nodes are the team, the decoys and
the captured members. The days use the origin of `FUN_0054bf00` and the chosen
container id (`+0x60`). `FUN_00487cc0` asks for confirmation on a `0x201` only
when the first member is in a blockaded own system, and then the text is
0x7056 (a character is not a fleet).

### Port

`MissionKind::Move` (`missions.rs`) is this mission: `move_order_enabled` is
`FUN_0053c100`, `move_destination_refusal` the destination checks (codes
`0x22`, `0x28`, `0x21`; hyp: a fleet with no capital ship gives `0x27`, as
no ship accepts; port: a special force refuses a fleet with `0x25`),
`move_refusal` the whole order's validation before any group starts (each
group keeps a free member and fits the Move record's masks), `dispatch_move`
the split into one mission per location, and `move_days` the confirmation's
days. The release asks first only when the first member stands in a
blockaded system of its own side; a character aboard a fleet is in the
fleet. A fleet destination boards the member onto the fleet
(`MissionEffect::MemberBoarded`); port: the port has no capital ship
container for a character or special force, so the fleet holds characters
and takes no special force. The interface's character
and special force Move rows, the Confirmed Move window (text 0x7056 when
blockaded) and drags from the Defenses and Fleet windows issue it
(`issue_member_move` in the app). Rank lapse is `officer_rank.rs`,
`clear_lapsed_ranks`.

Decompiles added for this section: `FUN_004f5530`, `FUN_004f4700`,
`FUN_005215e0`, `FUN_005233a0`, `FUN_0056ea00`, `FUN_0056ea20`,
`FUN_0056ea40`, `FUN_0056ea60`, `FUN_0056ebc0`, `FUN_0056eed0`,
`FUN_00593730`.

## Still open

- The names of window types 4, 10 and 11, and `FUN_004a2f80` (type 4's
  `+0x68`); type 4 is read as the fleet window from its size and its opening
  for item kind `0x10`.
- Families `0x20..0x21` (ALLFACSD) and `0x98..0x9f`.
- The leg builders `FUN_005529a0`, `FUN_00552000`, `FUN_00552300` and
  `FUN_00552dd0`.
- What the `0x180` game event does on the simulation side (hyp: fleet
  departure, analogous to `FUN_004f7640`).
- What fleet vtable `+0x200` (`FUN_004ff7a0`) does for non-fleet objects
  (characters, capital ships) in a `0x214` team.
- `FUN_0053efa0`'s class handlers (`+0x2c`) that turn a team id into moving
  objects.
- Where Escape cancels targeting, if anywhere outside the frame.
- The confirmation window's title text and the meaning of `+0x30`, both unused
  for moves.

## Supporting decompiles

`FUN_004ef260`, `FUN_00503d90`, `FUN_004ef5b0` (the character and special
force slots `+0xb0` and `+0x218`), `FUN_00486fb0`, `FUN_00487c50`, `FUN_00487cc0`, `FUN_00487740`,
`FUN_00488030`, `FUN_0048aa90`, `FUN_0048a340`, `FUN_0041cdf0`,
`FUN_0041ce20`, `FUN_0041cee0`, `FUN_004360f0`, `FUN_0041d5e0`,
`FUN_0041d7b0`, `FUN_0042a590`, `FUN_00422ce0`, `FUN_0051f4b0`,
`FUN_0051f8f0`, `FUN_004f5cd0`, `FUN_0053cec0`, `FUN_0053cf90`,
`FUN_0053ce70`, `FUN_0053cf40`, `FUN_0053cf30`, `FUN_0053d000`,
`FUN_0053c040`, `FUN_0053c710`, `FUN_0053c100`, `FUN_0053c1a0`,
`FUN_0053c240`, `FUN_0053c4b0`, `FUN_0053f150`, `FUN_0053efa0`,
`FUN_0053c810`, `FUN_0053cbc0`, `FUN_0053ca50`, `FUN_0053ca30`,
`FUN_0051fb70`, `FUN_0051fb80`, `FUN_0051fbb0`, `FUN_005201c0`,
`FUN_005202d0`, `FUN_004f5440`, `FUN_0053d020`, `FUN_0053d0e0`,
`FUN_0053d100`, `FUN_0053d3c0`, `FUN_0053d430`, `FUN_00553aa0`,
`FUN_00553b80`, `FUN_00504e60`, `FUN_005535b0`, `FUN_00578ab0`,
`FUN_00578c00`, `FUN_00578d00`, `FUN_00578f30`, `FUN_005152e0`,
`FUN_00515390`, `FUN_00515440`, `FUN_005555e0`, `FUN_00555920`,
`FUN_00556390`, `FUN_005542f0`, `FUN_004fdc70`, `FUN_004f6800`,
`FUN_004ff8e0`, `FUN_004fdb00`, `FUN_004fe710`, `FUN_0049a1b0`,
`FUN_0049a350`, `FUN_0049a8b0`, `FUN_0053c2e0`, `FUN_0044f060`,
`FUN_0044f180`, `FUN_0044f5e0`, `FUN_0044f640`, `FUN_0060b840`,
`FUN_0060b9d0`, `FUN_0042db70`, `FUN_0041db60`, `FUN_0041e390`,
`FUN_0041e500`, `FUN_00441190`, `FUN_0045aac0`, `FUN_00429ce0`,
`FUN_004591d0`, `FUN_004a2630`, `FUN_0049ee20`, `FUN_00452fc0`,
`FUN_004a7790`, `FUN_0049f130`, `FUN_004ac120`, `FUN_00606380`,
`FUN_0045c660`, `FUN_0045c6b0`, `FUN_0045c830`, `FUN_00458b80`,
`FUN_004aa470`, `FUN_004a3130`, `FUN_004aa380`, `FUN_004a09a0`,
`FUN_00555460`, `FUN_00551100`, `FUN_00551270`, `FUN_00552e80`,
`FUN_00552ff0`, `FUN_005531b0`, `FUN_00552d10`,
`FUN_00537040`, `FUN_00537060`, `FUN_00537090`, `FUN_00537110`,
`FUN_00537180`, `FUN_00537bc0`, `FUN_0057ddc0`, `FUN_0057e000`,
`FUN_0057e0d0`, `FUN_0057e380`, `FUN_004ff7a0`, `FUN_00524fb0`,
`FUN_0053a010`, `FUN_004f8880`, `FUN_0053fdd0`, `FUN_005400f0`,
`FUN_004fd3b0`, `FUN_004fd2f0`, `FUN_004fd370`, `FUN_0051f730`,
`FUN_00553960`, `FUN_005539f0`, `FUN_00583f50`.
