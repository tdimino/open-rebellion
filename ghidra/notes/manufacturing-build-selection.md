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
  10293/10294 (side 2), 10295/10296 (other). Then, in font 10
  (`FUN_00601c60(label, 10)`, `FUN_00458080.c:77`, 91, 106, 121; Arial,
  14-pixel cell, weight 400):
  - title at (5, 1), in the label's default colour, black (no
    `FUN_00601c90`): 6185 "Ship Construction", 6193 "Troops in Training",
    6195 "Facilities Under Construction"; the lines below are white
    (`0x2ffffff`);
  - at (5, 16): the product's name, or 6198 "No Ships are being built",
    6199 "No Troops in training", 6200 "No Facilities are being built";
  - with a product (`+0x178`): its GOKRES portrait, class `+0x30 & 0xfff`
    (`FUN_00457c90.c:48-55`; the picture `FUN_0042c3b0` draws with flag 1,
    the mini being `+ 0x4000`), blitted keyed at (40, 15), and 6209
    "Building: " (ships, facilities) or 6208 "Training: " with the units
    left (`+0x58`) at (5, 47);
  - at (5, 57): 6201 "Destination: " and the destination system's name.
- The band paints the title, the portrait, the units line, the name and
  the destination in that order (`FUN_00457690.c:76-82`), so the lines lie
  over the portrait: under Wine the route-1 Empire save's ships band shows
  the Death Star's portrait with "Death Star 1" whole across it. Port:
  `manufacturing_window::band` gives the portrait and `system_window`
  paints it at `BAND_PORTRAIT` before the lines.
- hyp (measured): under Wine font 10 draws Arial's hinted glyphs at an
  11-pixel em: the idle lines "No Troops in training" and "No Facilities
  are being built" are 98 and 126 pixels of ink (route-1 Empire save, day
  123 played on). Port: the overview's text at a 10.6 em
  (`system_window::OVERVIEW_FONT_EM`), which egui's outline advances need
  for the same widths: the yard counts, the three bands' titles and lines,
  and the band name edit.
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
12291 "Rename"; `0x216` 1002, 12294 "Reserved" (check mark 11902).

**An idle band greys Stop and Rename.** Both orders run the base enable
`FUN_0051fe20`, which asks each command's `+0x18` through `FUN_00553770`.
Each command resolves its manager (`1`/`0x12`) and calls a manager slot
(vtables `0x0065fbf8` and `0x006629a0` agree):

- Stop (order vtable `0x00661250`, command vtable `0x00669a80`): `+0x18`
  `FUN_0057eaa0` → manager `+0x1cc` `FUN_00528270`. After the object check
  `FUN_004f9860` it refuses `0xa0`/1 when `FUN_0052be30` finds no product:
  the key at `+0x74` (`FUN_0040d760`) is unset or resolves to nothing
  (`FUN_00528270.c:22-31`).
- Rename (order vtable `0x00661098`, command vtable `0x00669a58`): `+0x18`
  `FUN_0057e6a0` → manager `+0x1c4` `FUN_00527f40`. It refuses `0xa0`/1 in
  the same case. When a product exists it returns the product's `+0x64`
  for kind `0x203` (`FUN_00527f40.c:23-41`). The order's own `+0x18`
  `FUN_00537c70` also refuses `1`/`0x15` when more than one band is
  selected.

Port: `order_gates` greys Stop on a band with an empty queue, and greys a
band's Rename unless `band_rename_target` finds the product ship
(`rebellion-app`).

**Rename names the band's product.** The band shows its manager's
product name (`+0x80`), which `FUN_0052ad30` copies from the current
product (`FUN_004f62d0`): an ordered capital ship's own name. On the
route-1 Empire save (day 123 played on) Coruscant's ships band reads "Ship
Construction / Death Star 1 / Building: 1 / Destination: Fleet 6".

- Enabled: the product's `+0x64` for `0x203` (`FUN_00527f40`). A capital
  ship runs the object check `FUN_004f9860`, which passes an unfinished
  ship of the side that is not en route (`0x203` is exempt from its
  completed test); a squadron always refuses (`FUN_004f66a0`, `1`/`0x17`).
  Rename was enabled on Coruscant's ships band building Death Star 1; no
  band building fighters could be staged there, so the squadron case rests
  on the code.
- The menu row: `FUN_00486fb0` → `FUN_0041d600` → `FUN_00429350` →
  `FUN_00458b50` keeps the order at window `+0x268` and calls
  `FUN_00458640`, which builds a CoolStringField (`+0x170`) over the
  band's name label: the label's rect (`+0x14..+0x20`) offset by the band,
  two pixels wider, font 10, the label's text selected and focused. On the
  route-1 save the selected "Death Star 1" showed black on a white box 61
  by 13 pixels; typed text shows white on the band.
- The commit `FUN_00458780` (Enter, which the field turns into `0x407`, or
  any press in the window, `FUN_004534f0` `0x201`/`0x204`): a changed,
  non-empty name goes into the order (`+0x44`) and `FUN_0041ce20` submits
  it; an unchanged or empty one drops the order. The order runs
  `FUN_0057e870` → manager `+0x1f4` (`FUN_00529b00`): `FUN_00529980` sets
  `+0x80` on the master copy and both sides' copies (`FUN_0052b650`), and
  the notifier `+0x22c` (`FUN_0052ba00`) renames the current product
  through `FUN_004f6e60`. Typing "Tarkin" and Enter on the route-1 save
  renamed the unfinished ship at once: the band and Fleet 6 read "Tarkin".
- Escape in the field goes to the window, which closes (`FUN_00458980`):
  on the route-1 save (day 127 played on to day 140) the Coruscant System
  window closed and the name stayed unchanged.
- A second click on a selected band's name label is the other path
  (`FUN_004534f0`: a release with bit 4 set, armed by `FUN_00528720` on an
  earlier release, calls `FUN_00458640`). Two clicks and a double-click on
  "Death Star 1" opened no edit on the route-1 save (day 127 played on to
  day 140). hyp: the release tests the window-relative cursor against the
  label's band-relative rect, while the same branch needs the cursor inside
  the bands' area (8, 57)-(230, 282), so no release reaches it.

Port: `SystemWindowState::begin_band_rename` opens the edit for the band
menu's Rename; Enter or a press elsewhere in the window sends
`SystemWindowAction::RenameProduct`, which renames the product ship
(`PanelAction::Rename`); Escape closes the window. The band's line reads
the product ship's name (`manufacturing_window::band`). The click path is
not ported.

**Stop asks first.** `FUN_00487cc0` confirms `0x213` as it does Scrap, in
the move confirmation window (`move-order.md`, "The confirmation window").
`FUN_0049a350` case `0x213` fills it: TEXTSTRA `RT_RCDATA` `0x7053` "Are
you sure you want to stop construction of the following?", then, for each
band the order stops (each entry of the order's list, `FUN_004f2f60`),
`0x7055`, `"\n|6  |5"`: the band's units, two spaces and its product's
name ("1  Death Star"). The picture `+0x2e` is `0x40e` (1038, a Rebel
soldier raising his hand) for side 1 and `0x40f` (1039, an Imperial one)
otherwise; ship, troop and facility bands share it. The checkmark submits
the Stop, the X destroys it, Enter and Escape press them.

Port: `move_confirmation::StopConfirmation`, opened by the band menu's Stop
(`rebellion-app`); the checkmark sends `PanelAction::StopProduction`. Seen
in both sides' native runs against the original's Wine frame: same picture,
window, line and font. The question wraps after "the" in both (the text
field's font: `move-order.md`, "The confirmation window").

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

1. reads the class's GOKRES portrait, resource `class + 0x30 & 0xfff`
   (`FUN_005fbd20(.., id & 0xfff, 2)`, `FUN_00437880.c:94-102`), the
   picture the manufacturing band shows, not the `+ 0x4000` mini;
2. centers the portrait in a 195 by 63 buffer (`0xc3` by `0x3f`,
   `:111-131`);
3. attaches the class name, with the portrait's height as the item's text
   offset (`+0x34`, `:138-139`): the box (`FUN_0060c210.c:60-74`) and the
   list (`FUN_006083c0.c:124-127`) add it to the top of the text rect and
   draw with `DT_CENTER | DT_NOCLIP` (`0x101`, `FUN_00437f80.c:55`), so the
   name starts that far below the item's top, over the lower rows of the
   centred portrait; and
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
- An invalid order shows 14358 "n/a" in all four fields and disables
  Confirm; times read "N" + 14357 " Days", capped at 9999. Validity is the
  status word below.

**List** (`FUN_0052e580`): the classes of the manager's production family
(`0x28` ships, `0x29` regiments and special forces, `0x2a` facilities) that
its side builds and has researched, the side's level at `+0x9c`, `+0xa0`,
`+0xa4`. `FUN_00437f80` reopens on the class last confirmed by the same
kind of manager (`DAT_006b289c` facilities, `DAT_006b28a0` troops,
`DAT_006b28a4` ships), else the first.

### Validity and refusals (2026-10-18)

Recovered by Claude from `FUN_00439160`, `FUN_00538220`, the build order's
vtable `0x006611a0` (slot 6 `+0x18` `FUN_0051fe20`, slot 7 `+0x1c`
`FUN_0051ff30`, slot 20 `+0x50` `FUN_005385f0`), `FUN_00553960`,
`FUN_0052e970`, `FUN_00438b60`, `FUN_00438980` and `FUN_0041ce20`.

**The status word.** Every validator writes a two-word status through
`FUN_00520580` (`[0]` the category, `[+4]` the code). It starts at
`{1, -1}`; the order is valid while `[+4]` is -1. `FUN_00439160` builds the
order from the window (class `+0x44`, quantity `+0x48`), calls
`FUN_00538220(order, ..., status)`, and with `[+4] != -1` writes "n/a" into
all four fields and disables Confirm (control `0x65`, slot `+0x10`; slot
`+0xc` enables it).

**What `FUN_00538220` checks,** in order; each step runs only while the
status is still valid, and a step that fails without writing a status
leaves `{1, 1}`:

1. The order's validator, slot `+0x1c` (`FUN_0051ff30`), which needs the
   order's `+0x1c` set and runs:
   1. slot `+0x18` (`FUN_0051fe20`): slot `+0x4c` (`FUN_005201c0`) and
      `FUN_00553770` over the order's producers (`+0x40`);
   2. slot `+0x50`, the build order's own check (`FUN_005385f0`):
      - `FUN_005202d0` and `FUN_00553b80`: the order and its producers'
        side;
      - per producer, its system through `FUN_00504e60` (that call's status
        when it finds none), then `{1, 0x28}` when the system's side
        (`+0x24 >> 6 & 3`) is not the order's and the class is not a capital
        ship (family `0x14..0x1b`);
      - the route (`FUN_00551190`, mode 3);
      - maintenance, `FUN_0052e970`: `{0xf3, 1}` when the class's `+0x4c`
        times the quantity is not 0 and exceeds the side's spare (`+0x58` −
        `+0x5c`; equality passes; `death-star-product.md`, section 3);
      - per producer, its manager's own products give their room back
        (`FUN_00552210`), since the Build replaces them; then per unit, the
        destination's room (`FUN_00551a20`): `{0x90, 3}` for a second family
        `0x18..0x1b` ship at one system, either side's, other uncompleted
        products included (`FUN_00507ab0`; `death-star-product.md`,
        section 2);
   3. `FUN_00553960`: each producer's own slot `+0x1c`.
2. `FUN_00506e60` (`DAT_006b2bb0 +0xbc`) is not 0.
3. Per producer, the completion time (`FUN_00555f00` -> `FUN_00529090`) and
   the deployment trip (`FUN_00555f90` -> `FUN_00555d90`) both succeed.

So a ship class whose maintenance exceeds the spare reads "n/a" with Confirm
disabled, whatever the class; at a system holding a Death Star, built or in
production, a second Death Star does too, unless the one in production is
the ordering yard's own.

**Enter still orders.** `FUN_00438b60` case `0xd` calls `FUN_00438980`
without looking at Confirm, then closes the window (slot `+0x30`).
`FUN_00438980` remembers the class for its kind of manager and calls
`FUN_0041ce20(order, 0)` -> `FUN_00436050` -> `FUN_00487740`, which runs the
same slot `+0x1c`; a refusal goes to `FUN_00487c90` and the side's advisor
reacts (`mission-dialog.md`, "Refusal"): `{0xf3, 1}` table slot `+0x28` on
both sides, `{0x90, 3}` the Empire's `+0x2c` (`death-star-product.md`).
The Confirm button cannot reach this path while disabled; Enter can.

### Drop-down list (2026-10-18)

Recovered by Claude from `FUN_00437880`, `FUN_00437f80`, `FUN_0060bed0`,
`FUN_0060c1f0`, `FUN_0060c210`, `FUN_0060cac0`, `FUN_0060cbf0`,
`FUN_0060d020`, `FUN_00608280`,
`FUN_00609ed0`, `FUN_006083c0`, `FUN_006094b0`, `FUN_00609500` and
`FUN_0060c970`.

**Geometry.** The class box (`CoolSelectionBoxClass`, `FUN_0060bed0`) has
2 rows (`+0xa8`, `FUN_00437f80`'s argument) of 70 pixels (`+0xa4` = `0x46`,
set after construction). Command `0x6c` posts 1000 to the box
(`FUN_0060c1f0`), which opens a popup (`FUN_0060cac0`) at the box's left
edge, under it, the box's 195 wide and 2 × 70 + 4 = 144 high: window
(6, 85)-(201, 229). It opens above the box instead when it would pass the
screen's bottom. The popup's list (`FUN_00607ea0`, id `0xbba`) sits at
(2, 2) in it, 191 by 140, with flags `0x70000`, and shares the box's items
(`FUN_00608280`). On creation `FUN_00609ed0` scrolls the list from 0 just
far enough to show the box's current class, so a class past the second row
ends up last. The popup covers the drop-down button (79, 90).

**Capture.** The box opens the popup on command 1000 through `0x3ea`
(`FUN_0060c210`): with a popup open it closes it, else it waits for the
button's release and opens one. The popup takes the mouse capture
(`FUN_0060d020`, `WM_CREATE`). A press outside it sends the box command
`0x3e9`, which closes it (`FUN_0060cbf0`); a press on it goes only to its
list, so no other control of the window sees either.

**Picking.** A press on a row (`FUN_006083c0`, `WM_LBUTTONDOWN`) clears bit
0 of every item's `+0x3c` (`FUN_006094b0`, flag `0x10000`), sets it on that
row, and posts notification `0x29b`; on a row already picked the release
does. The popup (`FUN_0060d020`) answers `0x29b` with the item holding bit
0: `FUN_0060c970` makes it the box's class, repaints, posts `0x29b` to the
window (which recomputes, `FUN_00439160`) and closes the popup. A press
below the last row only clears bit 0.

**Row colors** (`FUN_006083c0`, `WM_PAINT`). Each item built by
`FUN_00437880` carries `+0x58` = 1 (own colors), `+0x5c` = white (picked),
`+0x60` = `0x808080` (normal) and `+0x64` = white (under the pointer)
(`FUN_0060ba00` defaults all three to white); font 10, its portrait centered
in 195 by 63. A row's name takes `+0x5c` when its `+0x3c` bit 0 is set, else
`+0x64` with bit 1, else `+0x60`. Only the name changes; the portrait does
not.
So every row is grey except the picked one and the one under the pointer.
No availability test colors a row: an unbuildable class and a buildable one
look alike.

**Under the pointer.** With flag `0x40000` a mouse move over the list
(`WM_MOUSEMOVE`) clears bit 1 on the row the list marked last (`+0x100`),
then sets it on the row now under the pointer (`FUN_00609d10`) and remembers
it; off every row, none. That move handler is the only code that clears
bit 1, and the items outlive each popup, so a row hovered in an earlier
popup stays white until a later popup marks it and moves off it. Bit 0
also outlives the popup: the picked row stays white in the next one.

Because the popup covers the drop-down button, the pointer that opened it
rests on the first row, so its first move lights that row: with Carrack
Light Cruiser first and Death Star second, Carrack shows white and the
Death Star grey.

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

- The drop-down is the traced popup: two 70-pixel rows under the box,
  scrolled to the class on opening, its names grey but for the picked row
  and the rows marked under the pointer (both bits kept per class for the
  window's life), a click on a row picking it, and a click elsewhere in the
  window only closing it.
- port: its scroll bar (`0x299a`) is not drawn; the wheel scrolls the rows.
  The list's keys (arrows, Page Up/Down, Home, End, Enter) are not ported.
  The capture covers the window only, and a press acts on its release.
- An order reads "n/a" with Confirm disabled when no yard can build it or
  `order_refusal` (`rebellion-core/src/build_selection.rs`) refuses it for
  maintenance (`{0xf3, 1}`) or a second Death Star (`{0x90, 3}`); Enter
  submits it anyway and the advisor reacts to the refusal.
- port: the validator's other refusals (`{1, 0x28}`, the producers' side
  and system, the route, the producers' own slot `+0x1c`, and
  `FUN_00529090`'s and `FUN_00555d90`'s failures) are not modelled.
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
- The drop-down list's scroll bar (`FUN_0060a490`, base `0x299a`) and its
  keys (`FUN_006083c0`, `WM_KEYDOWN`).
- Name every producer-manager field used by availability and time calculation.
- The checks behind the generic `{1, 1}`: `FUN_005202d0`, `FUN_00553b80`,
  `FUN_00504e60`, `FUN_005201c0`, `FUN_00553770`, `FUN_00529090`.
- Capture every Manufacturing and Build Selection state from an owned English
  640 by 480 installation for A0 comparison.

