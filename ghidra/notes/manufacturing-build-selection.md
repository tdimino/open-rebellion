---
title: "Manufacturing and Build Selection windows"
description: "Window type 9 and its 210 by 261 Build Selection child: pages, producer bands, resource IDs, controls, commands, quantities, costs, and completion/deployment fields"
---

# Manufacturing and Build Selection windows

Recovered 2026-10-06 from `REBEXE.EXE` with Homebrew Ghidra 12.1.4 in
headless, read-only mode on an isolated copy of the project. The trace was
cross-checked against the owned `STRATEGY.DLL` resources and the preserved
manual's Manufacturing and Production and Build Selection figures. No analysis
or lock files were written to the repository project.

This note separates facts recovered directly from code and resources from the
remaining inferences. See also [production-destination.md](production-destination.md)
for destination state and [build-delivery.md](build-delivery.md) for completion,
transit, and arrival behavior.

## Manufacturing window

`FUN_00452fc0` allocates `0x26c` bytes and constructs window type 9 through
`FUN_0045aac0`. The window is 226 by 304. Its content rectangle is
`(8,57)-(230,282)`, and it starts on page `0x67`.

`FUN_004568a0` selects these pages:

| Page | Object family | Recovered purpose |
|---:|---:|---|
| `0x67` | none | Manufacturing overview |
| `0x68` | `0x28` | Ship construction |
| `0x69` | `0x29` | Troop and special-forces training |
| `0x6a` | `0x2a` | Facility construction |
| `0x6b` | `0x2d` | Refinery facilities |
| `0x6c` | `0x2c` | Mine facilities |

The family meanings are corroborated by `FUN_00537ff0`, which maps production
manager type 0 to `0x28`, type 1 to `0x2a`, and type 2 to `0x29`, and by the
manual's three overview labels.

### Overview producer bands

`FUN_00458480` installs three selectable producer regions:

| Area | Rectangle |
|---|---|
| Ship Construction | `(55,57)-(221,136)` |
| Troops in Training | `(55,138)-(221,217)` |
| Facilities Under Construction | `(55,219)-(221,298)` |

The third source rectangle extends below the nominal content rectangle; the
port should preserve the source hit geometry until an original-runtime probe
proves clipping behavior. `FUN_00457690` draws the active or inactive shell,
the current page label, and overview capacity values at x=6 and y=119, 200,
and 280. Its producer counts and capacities come from `FUN_0052c8c0`,
`FUN_0052c5a0`, and `FUN_0052c270`.

Facility rows use 69 by 40 miniatures. `FUN_00454160` selects these page-label
or facility resource IDs:

| Family/page | Decimal | Hex |
|---|---:|---:|
| `0x2c` / `0x6c` | 10321 | `0x2851` |
| `0x2d` / `0x6b` | 10324 | `0x2854` |
| `0x28` / `0x68` | 10327 | `0x2857` |
| `0x29` / `0x69` | 10330 | `0x285a` |
| `0x2a` / `0x6a` | 10333 | `0x285d` |

`FUN_00458fe0` selects four-state item art from these bases:

| Type code | Families `0x28..0x2a` | Other facility families |
|---:|---:|---:|
| 1 | 9006 (`0x232e`) | 9001 (`0x2329`) |
| 2 | 9014 (`0x2336`) | 9030 (`0x2346`) |
| 3 | 9022 (`0x233e`) | not observed |
| 4 | 9010 (`0x2332`) | not observed |
| 5 | 9018 (`0x233a`) | not observed |
| 6 | 9026 (`0x2342`) | not observed |

The state offset is 0 through 3: see "Facility pages" below. The manual
pictures the three looks (p. 84: completed, under construction, en route).

`FUN_004534f0` owns page selection, overview producer selection, list/item
selection, dragging, and release. Command `0x70` selects a page. The overview
routes one of its three producer regions to its corresponding production page;
non-overview pages route the pointer through the item list.

### Window composition (2026-10-06, second pass)

Recovered by Claude from `FUN_00455060` (slot 14, `WM_CREATE`),
`FUN_00456230`, `FUN_00458080`, `FUN_00457c90`, `FUN_00457b40`,
`FUN_00457f30`, `FUN_00458040` and `FUN_00453ee0`. Type 9 is the window the
port calls the System window (`system_window.rs`); the manual names it the
Manufacturing and Production window (pp. 82–86, Figs. 2.10, 3.24, 3.27).

**Chrome.** Background 10297 (226 by 304). Title strip by the system's side
(`+0x148`, its `+0x24` bits 6..7), focused or not: side 1 10299/10200, side 2
10201/10302, otherwise 10303/10304 (`FUN_00456230`, the Fleet and Defenses
windows' strips). Title text font 5 at (x, 2), x = the sector button's width
plus 5, 179 by 16. Sector button 10209/10208 (`0xc9`) at (3, 3); minimize
10253/10254 (100) and close 10108/10109 (`0x65`) right-aligned.

**Tabs** (`FUN_0060d590` at (0, 20), 226 by 33, command `0x70`; 36 by 33
buttons):

| x | Page | Normal / pressed | Empty | Enabled when | Help |
|---:|---|---|---|---|---|
| 0 | `0x67` overview | side 1 10312/10311, side 2 10315/10314, other 10318/10317 | — | always | 6197 "Manufacturing" |
| 39 | `0x68` shipyards | 10327/10326 | 10328 | `FUN_0052c8c0(system, side, 3)` != 0 | 6184 "Shipyards" |
| 77 | `0x69` training | 10330/10329 | 10331 | `FUN_0052c5a0(.., 3)` != 0 | 6192 "Training Facilities" |
| 115 | `0x6a` construction | 10333/10332 | 10334 | `FUN_0052c270(.., 3)` != 0 | 6194 "Construction Yards" |
| 152 | `0x6b` refineries | 10324/10323 | 10325 | `FUN_0052d270` walk not empty | 6183 "Refineries" |
| 190 | `0x6c` mines | 10321/10320 | 10322 | `FUN_0052d690` walk not empty | 6182 "Mines" |

The empty art replaces the normal one when the count is zero; the creation
code passes the empty id as the normal bitmap. Counts are of the shown side.

**Overview** (page `0x67`, `FUN_00457690`):

- The left column 10298 (46 by 226) at (6, 71): the three yard pictures.
- Each yard count "N:M" (the executable's ":" at `DAT_006a872c`, read
  from REBEXE.EXE) in font 10 at (6, 119), (6, 200),
  (6, 280): N = `FUN_0052c8c0(system, side, 1)`, M = the same with 3
  (ships, then `FUN_0052c5a0` troops, then `FUN_0052c270` facilities). The
  manual: the first is the yards at the site, the second also counts those
  being built or deployed there (Fig. 2.10, Fig. 3.24).
- Three bands (`FUN_00458480`), 166 by 79: (55, 57), (55, 138), (55, 219).
  Band n is manager `FUN_00509670(system, k)`, k = 0 ships, 2 troops,
  1 facilities. Each band paints, into a copy of the background under it,
  10290 keyed, then its title strip: 10291 selected / 10292 not (side 1),
  10293/10294 (side 2), 10295/10296 (other). Then, font 10, white:
  - title at (5, 1): 6185 "Ship Construction", 6193 "Troops in Training",
    6195 "Facilities Under Construction";
  - at (5, 16): the product's name, or 6198 "No Ships are being built",
    6199 "No Troops in training", 6200 "No Facilities are being built";
  - with a product: its GOKRES mini (class `+0x30 & 0xfff`) at (40, 15) and
    6209 "Building: " (ships, facilities) or 6208 "Training: " with the
    units left (`+0x58`) at (5, 47);
  - at (5, 57): 6201 "Destination: " and the destination system's name.
- A progress bar per band (`FUN_004acec0`, ids `0x6d..0x6f`) at (56, 127),
  (56, 208), (56, 289), 160 by 4: position `+0x5c` of range `+0x68`.

**Selection and menu.** A press in a band selects it (band `+0x30` bit 0;
Ctrl toggles). The window's selection is the selected bands' managers
(`FUN_00453ee0`), so a right release opens their menu (`FUN_004ac5c0`).
The manager classes' order lists (vtable slot `+0x3c`; `FUN_0052ae30`,
`FUN_0055b580`, `FUN_0055b900`, `FUN_0055bd30`), with Encyclopedia and
Status from `FUN_0051d990`, in STRATEGY sort order:

| Band | Orders |
|---|---|
| ships | Build (`0x211`), Stop (`0x213`), Destination (`0x214`), Rename (`0x215`), Encyclopedia, Status, Reserved (`0x216`) |
| troops | Build (`0x212`), Stop, Destination, Encyclopedia, Status, Reserved |
| facilities | Build (`0x210`), Stop, Destination, Encyclopedia, Status, Reserved |

STRATEGY `RT_RCDATA`: `0x210..0x212` sort 100, TEXTSTRA 12288 "Build";
`0x213` 110, 12289 "Stop"; `0x214` 120, 12290 "Destination"; `0x215` 500,
12291 "Rename"; `0x216` 1002, 12294 "Reserved" (check mark 11902). A left
release on a selected band's status text issues `0x215` there
(`FUN_00528720`), the in-place rename the Fleet window uses.

**Facility pages** (`0x68..0x6c`, `FUN_004568a0`). The list (`FUN_00607ea0`,
id 2) at (8, 77), 222 by 225, 69 by 40 cells. Each page lists the shown
side's facilities of its family that are not hidden (`+0x50` bit 3), in walk
order. An item's picture is its type's base (`FUN_00458fe0`) plus a state:
0 built, 1 under construction (2 when its side is 1), 3 en route
(`+0x50` bits 2 and 4). Bases, families `0x28..0x2a` by type code 1..6:
9006, 9014, 9022, 9010, 9018, 9026; mines and refineries (`0x2c..0x2f`):
9001 and 9030. Its selected picture adds the side frame (`+0x15c`: 10262,
10263, 10264) keyed. The mines page then adds one empty-slot picture 9005
per raw-material deposit (system `+0x64`) beyond the mines it walked.

## Build Selection window

`FUN_0041d640` calls `FUN_00437df0`, which allocates `0x154` bytes and invokes
`FUN_00437880` with width `0xd2` (210), height `0x105` (261), and background
resource 10800 (`0x2a30`). The owned 210 by 261 STRATEGY bitmap matches the
manual's Build Selection figure. The class vtable is `PTR_FUN_00658db8`.

### List construction

`FUN_00437880` loads GOKRES module 10, initializes a candidate list with
`FUN_0052d720`, and filters it for the selected production manager through
`FUN_00537ff0`. For each buildable class it:

1. reads the class miniature resource from `class + 0x30 & 0xfff`;
2. centers the miniature in a 195 by 63 buffer;
3. attaches the class name; and
4. appends it to the drop-down list.

`FUN_00537ff0` scans production-manager families `0xa0..0xaf`, observes their
current producer state, and maps producer type 0 to ship classes (`0x28`),
type 1 to facility classes (`0x2a`), and type 2 to regiment/special-force
classes (`0x29`). It calls `FUN_0052e580` to add the eligible classes. This is
the recovered per-producer build-list boundary; it is not one global list.

### Layout and fields

`FUN_00437f80` creates these fields:

| Field | Rectangle or origin | Notes |
|---|---|---|
| Selected item | `(6,22)`, 195 by 63 | Resource 10650 (`0x299a`) |
| First cost | `(36,110)`, 64 by 23 | Plain text/value field |
| Second cost | `(138,110)`, 64 by 23 | Plain text/value field |
| Best completion time | `(140,145)`, 60 by 15 | Updated by `FUN_00438f30` |
| Best deployment time | `(140,165)`, 60 by 15 | Updated by `FUN_00438f30` |
| Number to build | `(141,196)`, 45 by 17 | Starts at 1; input limit `0x19` |

`FUN_00439160` recomputes availability, costs, completion, and deployment
whenever the selected class or quantity changes. `FUN_00538220` multiplies the
two class costs by the selected quantity, evaluates the available producers,
and derives the best completion and deployment values. `FUN_00438dd0` caps
displayed costs at 9999; `FUN_00438f30` applies the same display cap to the two
time values. Invalid values use text resource `0x3816`; the time format uses
`0x3815`.

The code confirms that completion and deployment are separate computed values.
It does not support treating the display as one generic ETA or replacing it
with a single destination combo.

### Controls and commands

`FUN_00438620` creates the source controls:

| Control | Origin | Rest/pressed resources | Command | Help/string |
|---|---|---|---:|---:|
| Close | source chrome | 10108/10109 (`0x277c/0x277d`) | 100 (`0x64`) | 6403 (`0x1903`) |
| Drop-down | `(79,90)` | 10606/10607 (`0x296e/0x296f`) | 108 (`0x6c`) | 6406 (`0x1906`) |
| Confirm | `(73,224)` | 10594/10595 (`0x2962/0x2963`) | 101 (`0x65`) | 6409 (`0x1909`) |
| Cancel | `(141,224)` | 10596/10597 (`0x2964/0x2965`) | 102 (`0x66`) | 6410 (`0x190a`) |
| Encyclopedia | `(5,224)` | 10592/10593 (`0x2960/0x2961`) | 103 (`0x67`) | 6411 (`0x190b`) |
| Quantity up | `(189,196)` | 10610/10611 (`0x2972/0x2973`) | 105 (`0x69`) | 6407 (`0x1907`) |
| Quantity down | `(189,205)` | 10612/10613 (`0x2974/0x2975`) | 106 (`0x6a`) | 6408 (`0x1908`) |

The two quantity controls repeat after 500 ms. `FUN_00438500` applies top trim
resource 10801 for side 1 and 10802 for the other side. It also creates label
resources `0x3810..0x3813`; their exact English strings remain to be bound.

`FUN_00438800` dispatches the commands:

- Close and Cancel discard the pending selection and close.
- Confirm calls `FUN_00438980` and closes.
- Encyclopedia opens the selected class through `FUN_0041d6b0`.
- Quantity up and down change the count.
- Drop-down opens the buildable-class list.

`FUN_00438b60` handles Enter as Confirm, Escape as Close, keypad `+` as
increment, and keypad `-` as decrement. `FUN_00438c30` and `FUN_00438c60`
clamp the quantity to 1 through 255; `FUN_00438c90` writes it to the edit.

Confirm copies the selected class key and quantity into the pending order,
updates the remembered selection for its producer category, and calls
`FUN_0041ce20(target, 0)`. `FUN_00439160` disables Confirm when
`FUN_00538220` reports that the order is not currently valid.

### Fields, labels and pricing (2026-10-06, third pass)

Recovered by Claude from `FUN_00437f80`, `FUN_00438500` (disassembled; the
decompile drops the `DrawTextA` arguments), `FUN_00439160`, `FUN_00438dd0`,
`FUN_00438f30`, `FUN_00538220`, `FUN_00528d30`, `FUN_00528b30`,
`FUN_00528960`, `FUN_00520b70`, `FUN_0052e580` and the class accessors
`FUN_0053b860`/`FUN_0053b870`.

**Labels** (`FUN_00438500`, `FUN_00606b40(id, rect, font, color, flags)`),
after the side's strip 10801 (side 1) or 10802 at (2, 2):

| String | Rect | Font | Color | Flags |
|---|---|---:|---|---|
| 14352 "Build Selection" | (2,2)-(204,16) | 5 | `0x2000000` black | `0x21` centered |
| 14353 "Number to build:" | (20,196)-(138,211) | 4 | `0x2f0fbff` | `0x22` right |
| 14354 "Best Time To Completion:" | (10,145)-(135,160) | 10 | `0x2f0fbff` | `0x24` left, v-centered |
| 14355 "Best Time To Deployment:" | (10,165)-(135,180) | 10 | `0x2f0fbff` | `0x24` |

**Values** (`FUN_00437f80`, white `0x2ffffff`): `+0x134` at (36,110) and
`+0x138` at (138,110), 64 by 23, font 4, flags `0x25` (centered);
`+0x12c` at (140,145) and `+0x130` at (140,165), 60 by 15, font 10, flags
`0x26` (right). `FUN_00439160` passes `FUN_00538220`'s outputs in this
order: `+0x134` = refined material, `+0x138` = maintenance, `+0x12c` =
completion, `+0x130` = deployment. The background's left cost box carries
the refined-material picture and the right one the wrench.

**Class records.** A class record in memory sits 0x28 bytes past its DAT
record: troop detection at `+0x5c` is TROOPSD offset 0x34, special-force
skills at `+0x58` are SPECFCSD 0x30, and a yard's `+0x5c` is MANFACSD
`processing_rate` at 0x34. So `FUN_0053b860` (`+0x48`) is the refined
material cost and `FUN_0053b870` (`+0x4c`) the maintenance cost.

**Pricing** (`FUN_00538220`): each cost times the quantity (`+0x48` of the
order). Times come from the manager:

- `FUN_00528960` sums, over the manager's yards whose `+0x60` bit 0 is
  clear, `FUN_0053e1b0(1, period)`; `FUN_00520b70(yard)` is the period, the
  yard class's `+0x5c` (MANFACSD `processing_rate`: 4 for the standard
  yards, 2 for the advanced ones).
- `FUN_00528b30(days)` counts the progress `days / period` summed over the
  same yards (with the active unit's credit when asked);
  `FUN_00528d30` starts from `ceil(work * 100 / rate)` (`DAT_00661a88` =
  100, `FUN_0053e160` the ceiling division) and steps to the least day
  whose progress reaches the work, the class's `+0x48` times the quantity.
  This is the "best case" of manual p. 84.
- Deployment is the trip from the manager's system to its destination
  (`FUN_00555f90` -> `FUN_00555d90` -> `FUN_00555b30`): ship speed
  `+0x6c` for families `0x14..0x1f`, else `DAT_006b9050`.
- Several managers keep the largest (`FUN_0053e130` is `max`).
- An invalid order (`param_5 +4 != -1`) shows 14358 "n/a" in all four
  fields and disables Confirm; times read "N" + 14357 " Days", capped at
  9999.

**List** (`FUN_0052e580`): the classes of the manager's production family
(`0x28` ships, `0x29` regiments and special forces, `0x2a` facilities) that
its side builds and has researched, the side's level at `+0x9c`, `+0xa0`,
`+0xa4`. `FUN_00437f80` reopens on the class last confirmed by the same
kind of manager (`DAT_006b289c` facilities, `DAT_006b28a0` troops,
`DAT_006b28a4` ships), else the first.

## Port contract

An authentic replacement for the current manufacturing panel must preserve:

- a type-9 modeless window with overview plus the five recovered pages;
- three independently selectable production-manager bands on the overview;
- producer-specific buildable lists rather than a global catalog;
- a separate bitmap-backed 210 by 261 Build Selection child window;
- the exact control geometry, source resources, commands, quantity range, and
  keyboard routes above;
- two distinct costs and two distinct best-time values;
- per-production-area Destination behavior and absolute completion/deployment
  day presentation where required by the surrounding runtime contract; and
- source-disabled Confirm behavior for an invalid order.

The invented Personnel, Fleets, Defenses, and Troops tabs are gone
(2026-10-06). Their characters, special forces and regiments open their
menus from the Defenses window (type 10), and their fleets drag from the
Fleet window (type 4), as the original's do.

### Ported (2026-10-06)

`system_window.rs` draws type 9 as traced: the 226 by 304 chrome with the
side's title strip and buttons, the six tabs with their side, pressed and
empty art, the overview's yard column, counts and three bands with their
progress bars, and the five facility pages with their label, pictures and
selected frame (`manufacturing_window.rs` models what they show). A press
on a band selects it; a right release opens its manager's menu
(`MenuObject::Producer`, `object_menu.rs`) with the traced orders. Stop
clears the band's area (manual p. 84) and Destination targets a system for
that area alone (manual p. 83); Build waits on the Build Selection window.

- port: Stop and Destination are enabled on the bands of a system the
  player holds, Stop only while the band builds; the orders' own `+0x18`
  rules are untraced.

- port: an empty page's tab still opens its empty page, as type 10's do;
  the original's empty art suggests a disabled tab (untraced).
- port: one band at a time; Ctrl's toggle is not ported.
- port: no scroll bar, so a page shows its first rows only.
- port: a facility's own menu (Encyclopedia, Status, Scrap) and the
  window's keys (`FUN_00458980`) are not ported; a facility list item is
  not a drag source.
- port: the in-place rename on a selected band's status text (`0x215`,
  `FUN_00528720`) is not ported.
- hyp: the other side's bands and pages show nothing unless the player sees
  that side's objects there (`opposing_contents_visible`).
- The counts' separator is ":" (`DAT_006a872c`), as manual Fig. 3.24
  prints "0:0" and "1:2"; an earlier pass of this note named TEXTSTRA 6181
  "/", which `FUN_00457690` does not load.

Build Selection (`build_selection.rs`, model in
`rebellion-core/src/build_selection.rs`) draws the traced background,
strip, labels, fields and controls; a band's Build opens it on the band's
list, and Confirm replaces the band's units, each queued with its own
days from the yards (`unit_build_days`). Orders now name classes
(`BuildableKind::Troop(DatId)`, `SpecialForce`, and facility classes with
their builder's side), from `GameWorld::buildable_classes`. The invented
Manufacturing side panel (M) is gone.

- port: the drop-down shows one class at a time and scrolls with the
  wheel; its scroll bar (`0x299a`) is not drawn.
- port: the close box sits 17 pixels in from the right, as the mission
  dialog's; the source chrome's placement is untraced.
- port: a disabled Confirm keeps its rest art.
- port: every yard counts toward the times; the busy bit (`+0x60` bit 0)
  is not modelled. A unit's days are fixed when queued, so yards built
  or lost later do not change them.
- port: maintenance is shown, not charged (manual p. 84 deducts it at the
  order); refined material is not drawn down.
- port: a built special force takes its class's base skills, without the
  creation roll.
- hyp: the list is in `DatId` order.

## Open questions

- Where 14356 "Day " is used (not by Build Selection's fields).
- Trace the complete scroll and selection behavior of the drop-down list.
- Name every producer-manager field used by availability and time calculation.
- Recover the exact disabled reasons and any user-facing rejection message.
- Capture every Manufacturing and Build Selection state from an owned English
  640 by 480 installation for A0 comparison.

