---
title: "Operation Report window: message class 0xf, the report window, its lists, text and routing"
description: "The bombardment and assault report: message kind 0xf (vtable 0x0065b870), the 470x331 report window (vtable 0x00659c78), its rail, six category tabs and lists, the events that fill them, picture and text choice, the live popup versus the Message Index route"
category: "ghidra"
created: 2026-10-17
updated: 2026-10-18
tags: [operation-report, bombardment, assault, message-class, notifications, window]
---

# Operation Report window

Companion to `bombardment.md` ("Messages"), `message-display.md` (formatter, classes) and
`battle-alert-lists.md` (the sibling alert, same art family). Confidence tags: `read-in-full`
(handler read end to end), `inferred`, `hyp:` (untraced).

## Pieces

| Piece | Address | Role |
|---|---|---|
| Message kind `0xf` | factory `FUN_00489740` case `0xf` → ctor `FUN_00496460` (`0xa4` bytes), vtable `0x0065b870` | the stored message; slot 3 `FUN_005a56b0` returns kind `0xf` |
| Report window | opened by window message `0x468`, wParam `0x1c` (`FUN_00422ce0` case `0x1c`) → ctor `FUN_0044c410` (`0x188` bytes), vtable `0x00659c78` slots 0-22 | the 470x331 window |
| Window builder | `FUN_0044c630` (vtable slot 14, `+0x38`) | art, rail, tabs, lists |
| Window slots | slot 17 `FUN_0044e290` (overlay paint), slot 18 `FUN_0044df90` (commands), slot 19 `FUN_0044e180` (keys) | |

`FUN_0044c410` sizes the window `0x1d6` x `0x14b` (470x331), `+0x118` = the command center,
`+0x174` = the message, `+0x178` = GOKRES (module 10) handle, `+0x17c` = rail state,
`+0x180` = tab. The parent's `+0x9c` is the player's side (1 Alliance, 2 Empire) and picks the
art set. It plays no sound (`audio-dispatch.md`, module 10 row "none"). read-in-full.

## 1. Message fields (vtable `0x0065b870`)

Base (`FUN_004c4d90`, vtable `0x0065c538`): `+0x20` side that owns this copy, `+0x28` advice code,
`+0x2e` picture id (STRATEGY module 7), `+0x30`/`+0x32` WAVE ids (left 0: no sound),
`+0x34` category mask (`FUN_0048bc60` = `0x100`, Conflict), `+0x40` filed flag, `+0x44` title,
`+0x50` body. Report fields (`FUN_00496460`, `FUN_00496380`):

| Offset | Meaning | Set by |
|---|---|---|
| `+0x5c` | system id | events `0x14f`, `0x150` |
| `+0x64` | 0 bombardment, 1 assault | event `0x150` (always 1) |
| `+0x68` | system holder's side (0, 1, 2) at the event | `0x14f`/`0x150` event's side field, stored only when the event value is non-zero |
| `+0x6c` | fleet owner's side, `fleet+0x24 >> 6 & 3` | events `0x182`, `0x183` |
| `+0x70`, `+0x78`, `+0x80`, `+0x88`, `+0x90`, `+0x98` | six object-id lists (`FUN_004f5510`) | below |
| `+0xa0` | 0 | ctor |

Save/load (`FUN_00496670`/`FUN_004966f0`) writes `+0xa0`, `+0x64`, `+0x68`, `+0x6c` and the lists
`+0x70`, `+0x80`, `+0x78`, `+0x88` only: `+0x90` and `+0x98` (Captured) are not persisted.
Slot 4 (`FUN_00497530`, "class") returns 7 when `+0x20 == +0x6c`, else 3. Slot 12 (`FUN_006158b0`)
returns 0: the message never expires (base `FUN_004c4fe0` expires after 300 ticks).

## 2. What fills the lists

Event ids: `0x14f` SystemBombard (`FUN_00512320`), `0x150` SystemAssault (`FUN_00512370`),
`0x182` FleetBombard (`FUN_004ff840`), `0x183` FleetAssault (`FUN_004ff890`), `0x302` generic
object destroyed. A system event node carries `[9]` = the system's holder side, `[10]` = value,
`[11]` = extra (`FUN_0053fcf0`); a fleet event carries value and extra only (`FUN_0053fdd0`).
Family byte = id >> 24: fleets `0x08`-`0x0f` (`FUN_004f36a0`), systems `0x90`-`0x97`
(`FUN_004f3220`), any object `0x01`-`0xfe` (`FUN_004f2d10`). The Personnel family is `0x30`-`0x3b`,
and its `+0xac` bit 0 is the Captured flag. All visitors `read-in-full`.

| Event (handler) | Condition | Effect |
|---|---|---|
| `0x182` (`FUN_00496770`) | fleet found in this side's view (`FUN_004f36a0`); else return | `+0x6c` = fleet owner. Value 0: each ship of the fleet (`FUN_00502db0`) goes to `+0x78`, then each child of the ship (`FUN_004fcd00`) to `+0x78`, except a Personnel child with `+0xac` bit 0, which goes to `+0x90`. Always `+0x40` = 1 |
| `0x183` (`FUN_00496770`) | same lookup | `+0x6c` = owner; value 0: the same ship/children fill; `+0x40` not touched |
| `0x14f` (`FUN_00496f50`) | system found in view | `+0x5c` = system id. Value 0: `+0x40` = 1 and each child of the system (`FUN_004fcd00`) goes to `+0x70`, except a Personnel child with `+0xac` bit 0, which goes to `+0x98`. Value non-zero: `+0x68` = the event's side |
| `0x150` (`FUN_00496f50`) | system found in view | `+100` = 1, `+0x5c` = id. Value 0: `+0x40` = 1 and each child: troop families `0x10`-`0x13` owned by `+0x68` go to `+0x70`, other troops to `+0x78`; Personnel without bit 0 to `+0x70`; Personnel with bit 0 to `+0x90` when `FUN_004edc80(obj) == +0x68`, else `+0x98`; everything else to `+0x70`. Value non-zero: `+0x68` = the event's side |
| `0x302` (slot 7 `FUN_00497380`, destroyed id `param_3`, second id `param_4`) | each in the group | id goes to `+0x80` when the object exists in the view and its `+0x50` bit 2 (Completed) is clear, or when `param_4`'s family is `0x90`-`0x97` (hyp: the object's container is a system); otherwise `+0x88` |

Reading: `+0x70` operational objects at the system, `+0x80` destroyed at the system, `+0x98` its
captured Personnel; `+0x78` operational objects of the fleet side, `+0x88` destroyed with a ship
or fleet, `+0x90` its captured Personnel. For an assault the troops are split by owner against
`+0x68` instead (hyp: `+0x68` = the defending holder, so `+0x70` = holder's troops, `+0x78` =
attackers; the `0x150` filter on `+0x68` and the title test below agree). Confidence: lists by
event `read-in-full`; the "holder" reading of `+0x68` inferred.
The system lists come from the message side's view of the system (`FUN_004f3220(side, id)`,
`FUN_004fcd00` over its children). VM evidence that the attacker's view holds the holder's
children: the Empire's report on Selonia lists the Alliance regiment and the GenCore facility under
System Forces (section "In the original"). That the holder's view
holds the attacker's landed regiments is untraced.

`0x302` also reaches the base handler `FUN_004c5020` (adds the id to the side's list), so both
paths see it. The Operational/Destroyed pairing below is exact; which `0x182`/`0x14f` value
the VM run produced (set versus clear) is open (Open, 1).

## 3. Filing, the live alert and Message Index reopen

Per side, the notification manager takes an event group:

1. `FUN_00488b50` → `FUN_00488de0` maps the group to a kind, walking nodes in order until one
   maps. `0x14f` maps to `0xf` only when the value is 0 and the system's `+0x50` bit 3
   (destroyed) is clear (a destroyed system files kind 0, no report; Planet Destroyed carries
   it, `bombardment.md`). `0x150` maps to `0xf` for any value, except Coruscant
   (`0x90000109`) with value non-zero and holder 2 while this side's view shows the holder as
   Alliance (`piVar3[9] & 0xc0 == 0x40`), which maps to 0. `0x182` and `0x183` have no mapping:
   they only supply the fleet lists. read-in-full.
2. `FUN_00489740` case `0xf` builds the message: the ctor visits every node of the group through
   `FUN_004974f0` (node slot `+0x1c`). A message whose `+0x40` is still 0 is deleted; else it is
   filed (`FUN_005f5600`). So a bombardment needs the `0x14f` value-0 node (or a fleet event
   that sets `+0x40`), an assault the `0x150` value-0 node.
3. `FUN_0048a060` (manager, by the message's slot-4 class):
   - class 7 (message side == fleet owner): the message joins the list, `FUN_0041d7d0` →
     `FUN_0042a410` posts `0x409` then `0x468`/wParam `0x1c`/lParam = message to the command
     center. That is the self-opening alert: the **bombarding/assaulting fleet owner**.
   - class 3 (the other side, or a side whose view lacks the fleet, `+0x6c` = 0): filed to the
     Message Index only, category `0x100`; if `+0x28` is non-zero (advice `0x2e` bombardment,
     `0x2f` assault) the droid advisor is told (`droid-advisor-triggers.md`). So the
     **target side gets a Message Index row plus advisor line, no popup**.
   The switch has further classes: 1 and 2 file to side lists, 3/4/5/8 file as class 3 does
   (class 7 differs only by the popup), 6 files to a separate list.
   Confidence: read-in-full for `FUN_0048a060`; "only kind `0xf` returns class 7" inferred
   (the `0x1c` window reads report fields).
4. Message Index (`FUN_00469de0` mode 2): a message whose slot 3 returns `0xf` does not use the
   single-message layout. It stores `DAT_006b2904` (message id), `DAT_006b2908/290c`, sets
   `DAT_006b2910 = 0x1c`, posts the same `0x468`/`0x1c`, and closes the index. The report window
   closes with `FUN_00469d60`: when `DAT_006b2910` matches and the message id equals
   `DAT_006b2904` it posts the Message Index back (`FUN_0041d770(1, DAT_006b290c)`). A live
   alert leaves `DAT_006b2910` at 0, so closing it returns nowhere. read-in-full.

Both routes show the same window with the tabs. A text-only row in the Index is not the
original behaviour.

## 4. Text and picture (`FUN_00496770`, code 2)

Visit with code 2 builds the text (hyp: the caller, after the group's events, is untraced).
Inputs: `iVar9` = side object of `+0x6c` (`FUN_004f3dd0`; any value other than 1 reads as side 2),
`iVar13` = side object of `+0x68` (1 or 2, else none), system `+0x5c`. Arguments use the
`message-display.md` kinds (4 = adjective/name, 1 = name); nothing counts numbers.

Bombardment (`+0x64` = 0), one text for all four orders (`0x220`-`0x223` are not read):

| Item | Id | Text | Arguments |
|---|---|---|---|
| title | `0x7098` | "Orbital bombardment of \|1" | system |
| body, holder 1 or 2 | `0x7099` | "\|1 ships have conducted an orbital strike on the \|2 system of \|3" | fleet side (k4), holder (k4), system (k1) |
| body, holder 0 | `0x709a` | "\|1 ships have conducted an orbital strike on the non-aligned system \|2" | fleet side, system |
| advice | `0x2e` | when `+0x20 != +0x6c` | |
| picture `+0x2e` | `0x2b9b` if `+0x80` non-empty, else `0x2b9c` if `+0x88` non-empty, else `0x2b9a` | ruined landscape / ship exploding in space / intact landscape with shield generator (STRATEGY BMP 11163 / 11164 / 11162, 400x310; checked against the files) | |

Shield-blocked: no separate branch. Nothing destroyed leaves `+0x80`/`+0x88` empty, so the
text is the same and the picture is `0x2b9a` (the Bespin capture, guide-063; the ruined
Tokmia capture, guide-061, has destroyed regiments, so `0x2b9b`). read-in-full.

Assault (`+0x64` = 1): title `0x7090` "Assault on \|1" (system); picture `0x2b98` (BMP 11160) when
`+0x6c` == 1 else `0x2b99` (11161); advice `0x2f` when `+0x20 != +0x6c`. Let `held` =
(system's current holder, `piVar4[9] >> 6 & 3`) differs from `+0x6c`:

| Holder object `iVar13` | `held` | Id | Text |
|---|---|---|---|
| present | no | `0x7091` | "\|1 troops have taken control of the \|2 system \|3" (attacker, previous holder, system) |
| present | yes | `0x7092` | "\|2 Troops have defended \|3 from an \|1 assault." |
| none | no | `0x7093` | "\|1 troops have seized control of the neutral system \|2." |
| none | yes | `0x7094` | "The neutral system \|2 has repulsed an attack by \|1 troops." |

No movie or WAVE is attached: `+0x30`/`+0x32` stay 0, `FUN_0044c410` plays no sound, and the
path calls no movie. The Death Star's Planet Destroyed message (kind `0x15`, movie `0x6f`,
`FUN_004950d0`) is a different message (`bombardment.md`).

## 5. The window (`FUN_0044c630`, `FUN_0044c410`)

Window coordinates, 470x331, Alliance art / Empire art (player's side):

| Part | Geometry | Resource |
|---|---|---|
| shell | (0, 0) | STRATEGY `0x285f` (10335) / `0x2860` (10336), 470x331, blitted keyed (`FUN_005fd0f0`) over the picture, so its palette-blue 400x310 aperture shows it |
| picture | (12, 13), 400x310 | `+0x2e` (module 7), copied first (`FUN_005fcc30`) |
| rail art | (412, 0) / (415, 0), 58x330 | `0x2a44` / `0x2a45`, blitted keyed |
| title | font `0xb`, centred in 400 at (12, 13); on every view | message `+0x44` |
| body (Summary) | font 6, 0x15e wide at (0x19, 0xe1) over a copy at (0x1a, 0xe2) | message `+0x50` |

Text colour by `+0x6c`: 1 red `0x20000ff`, 2 green `0x200ff00`, else cyan `0x2ffff00` (COLORREF `0x00BBGGRR`).

Rail (right edge), five buttons:

| Id | Rect | Bitmaps (A / E) | Tooltip | Action |
|---|---|---|---|---|
| `0x0a` | A (0x1a7, 0x19) 32x31; E (0x1aa, 0x15) 44x41 | `0x2882`/`0x2883`; `0x2888`/`0x2889` | none | close (slot `+0x30`, then `FUN_00469d60`); Esc does the same (`FUN_0044e180`, key `0x1b`) |
| group `100` | A (0x1a2, 0x58) 41x203; E (0x1aa, 0x59) 44x218; radio, default `0x65` | | | `FUN_0044e3d0` |
| `0x65` | y 0 (A), 0 (E) | `0x29e8`/`0x29e9`; `0x29f2`/`0x29f3` | `0x1944` Summary | layer 1: picture, title, body |
| `0x66` | y 0x35 / 0x3b | `+0x6c`==1: `0x29ea`/`0x29eb`, E `0x29f4`/`0x29f5`; else `0x29ec`/`0x29ed`, E `0x29f6`/`0x29f7` | `0x1937` Alliance Forces / `0x1938` Imperial Forces | list view of the fleet side (`+0x78`, `+0x88`, `+0x90`) |
| `0x67` | y 0x6c / 0x76 | by `+0x68`: 1 `0x29ea`/`0x29eb`, 2 `0x29ec`/`0x29ed`, else `0x29f0`/`0x29f1` (E: `0x29f4`.., `0x29f6`.., `0x29fa`/`0x29fb`) | `0x1943` System Forces | list view of the system (`+0x70`, `+0x80`, `+0x98`) |
| `0x68` | y 0xa2 / 0xb1 | `0x2a27`/`0x2a28`; `0x2918`/`0x2919` | `0x1939` Display | layer 5 |

Buttons are 41x41 (A) / 44x41 (E). Display (layer 5): the starfield BMP `0x2a0b`, the text
`0x7013` "Select one of the following buttons to close this display and go directly to...",
and two 169x96 picture buttons at y 0xb6: system at x 0x1d (id `0x0b`, picture `0x2a0c`, caption
the system name or `0x1945` "Goto System") and fleet at x 0xe2 (id `0x0c`, `0x2a0d` Alliance or
`0x2a0e` Empire, fallback frames `0x2a0f` red / `0x2a10` green, caption `0x1942` "Fleet"). Id
`0x0b` closes the window and opens the system's sector window (`FUN_00429ce0`); `0x0c` does the
same and then `FUN_0045c8e0(sector window, system id, 4)` (`FUN_0044df90.c:20-35`). Kind 4 maps to
the sector window's item kind `0x10`, the top-right Fleet icon (`FUN_0045c8e0.c`;
`sector-quadrants.md`, `fleet-window.md`), and `FUN_0045aac0` opens that item's Fleet window
(type 4, `FUN_004a2630`) at the icon's stored point. It selects no fleet and does not test that
the report's fleets are still at the system. Both buttons resolve the message's system through
the player's view (`FUN_004f2d10(parent +0x9c, message +0x5c)`), skip the opening when it is not
found, and then close the report (`FUN_00469db0`, slot `+0x30`).
Layer-5 text second position (`x 0x1d`, `y = block height + 0xb1`) not measured.

The two picture buttons are CoolStrobeButtons (`FUN_00602150`, 169x96). A caption set through
`FUN_00602520` (a TEXTSTRA id) or `FUN_00602650` (a string) is laid out at once
(`FUN_006026e0.c:28-47`): `DrawTextA` with `DT_CALCRECT | DT_CENTER` (`0x401`) into a rectangle
of the button's client width measures the text, and the rectangle is centred in the button on both
axes (left = (w - text width) / 2, top = (h - text height) / 2). `FUN_0044c630` then shifts it down
by `h / 2 - 12` (`FUN_006027e0(button, 0, h / 2 - 12)`, 36 for h = 96; `FUN_0044c630.c:851`,
`:909`) and sets the text colour white (`FUN_00603010(button, 0, 0xffffff)`). The paint
(`FUN_00602d30.c:84-89`) draws it with `DT_CENTER` in that rectangle. The font is the window
default: every CoolGameWindow takes the application object's `+0xb0` at construction
(`FUN_005ff910.c:145-147` → `FUN_00603850` → `FUN_00600290`), and `FUN_00413000.c:141` sets
that field to 4. The caption is therefore font 4, centred across the button, with its top
(96 - line height) / 2 + 36 below the button's top.

Tabs (id `0x96` group at (0x24, 0x3b), 0x169x0x29; each button 0x31x0x29 at x 0, 0x3e, 0x7c,
0xba, 0xf8, 0x138, only on the list views). A tab filters: `FUN_0044e500` keeps each list object
whose `id >> 24` lies in `[lo, hi)` (`FUN_0044e500.c:167-212`) and appends it in the list's own
order (`FUN_004f57b0`, `FUN_005f4f10`); nothing sorts by family, so a Personnel object of family
`0x30`-`0x3b` (a major, `0x32` Luke, or a minor, `0x38`) lands on the same tab in list order.
Family ranges `[lo, hi)` of `FUN_0044e500`:

| Id | Tab (tooltip) | Families | Bitmaps A (args) / E |
|---|---|---|---|
| `0x97` | Capital Ships `0x1928` | `0x14`-`0x1b` | `0x286c`,`0x286b` / `0x2878`,`0x2877` |
| `0x98` | Fighter Squadrons `0x1929` | `0x1c`-`0x1f` | `0x2872`,`0x2871` / `0x287c`,`0x287b` |
| `0x99` | Manufacturing `0x1835` | `0x28`-`0x2f` | `0x2868`,`0x2867` / `0x2874`,`0x2873` |
| `0x9a` | Defense Facilities `0x1946` | `0x22`-`0x27` | `0x2866`,`0x2865` both |
| `0x9b` | Trooper Regiments `0x1930` | `0x10`-`0x13` | `0x2870`,`0x286f` / `0x287a`,`0x2879` |
| `0x9c` | Personnel `0x1931` | `0x30`-`0x3f` | `0x286a`,`0x2869` / `0x2876`,`0x2875` |

Initial tab `0x97` for a bombardment (`+0x64` = 0), `0x9b` for an assault; initial view Summary.
Selecting a tab (or rail `0x66`/`0x67`) refills the list (`FUN_0044e500`): each list is filtered by
family range, then rows are built pairwise (`FUN_0044edb0`). The scroll list is `FUN_00607ea0` at
(0x25, 0x88), 0x15b x 0xa6, bar RCDATA `0x29fc`, white text.

| View | Layer art | Column headers (font 4, white, y 0x75, x) | Row cell |
|---|---|---|---|
| tabs 0x97-0x9b | `0x2a09` | `0x1932` "Operational" x 0x26, `0x1933` "Destroyed" x 0xcc, each 0xa5 wide | 165x70 |
| tab 0x9c | `0x2a0a` | `0x1934` "Survivors" x 0x26, `0x1935` "Captured" x 0xa5, `0x1936` "Killed" x 0x113, each 0x6e wide | 110x50 |

Columns: Operational/Survivors = `+0x78` (rail `0x66`) or `+0x70` (rail `0x67`); Destroyed/Killed =
`+0x88` or `+0x80`; Captured = `+0x90` or `+0x98`. An empty Operational column shows the first
row "None" (RCDATA `0x8526`), an empty Destroyed column "No Casualties" (`0x1941`). A cell is the
object's GOKRES mini (`FUN_0042c3b0`, module 10), centred, with its name (`FUN_004f62d0`) beneath.
Overlay (`FUN_0044e290`, rail states `0x66`/`0x67`): subtitle (the rail tooltip) font 6 centred in
0x15b at (12, 0x24) in the side colour, and the tab's tooltip left-aligned at (0x26, 100), font
4, width 400. Keys `0x25`/`0x27` (Left/Right) step the visible tab. Resource sizes read from
STRATEGY.DLL; geometry `read-in-full`.

### List cell: damage marks and column

The lists hold object ids only (`FUN_004f5510` nodes, `FUN_004eccd0`). Each tab fill
(`FUN_0044e500`) resolves every id through the player's side view
(`FUN_004f2d10(parent +0x9c, id)` → `FUN_0053ef00` → `FUN_0053efd0` → `FUN_005844e0`, the object's
copy for that side, `cpp-class-hierarchy.md`) and hands it to `FUN_0044edb0`, which calls
`FUN_0042c3b0(module-10 handle, id, object, param_4 = (param_2 == 0), param_5 = 1)`. `FUN_0044e500`
passes `param_2 = 0` on tabs `0x97`-`0x9b` and 1 on Personnel `0x9c`, so every ship, squadron,
facility and regiment cell takes the `param_4 != 0` art. The report copies no state when it
is filled: the picture is chosen from the view object's flags each time a tab is selected, and
the view's flags are what the damage setters write (below). read-in-full.

`FUN_0042c3b0` draws the portrait (art `type & 0xfff`, `type` = object slot `+0x30`), then overlays:

| Condition (`obj` = the view object) | Overlay art (module 10 unless noted) | Line |
|---|---|---|
| `(obj[0x14] & 0x200) != 0` (`+0x50` bit 9, damaged) and `param_5 != 0` | family `0x30`-`0x3f`: `0x2cee` (`0xffffd312` less when `param_4 != 0`) from module 7; fleets `0x08`-`0x0f`: `0x28b8`/`0x28ea` by side (`+0x24` bits 6-7); every other family (capital ships `0x14`-`0x1b` included): `(type & 0xfff) + 0x2000` (`+0x6000` when `param_4 == 0`), module 10, blitted keyed over the portrait (`FUN_005fd0f0`) | `FUN_0042c3b0.c:350-410` |
| `(obj[0x14] & 0x10) != 0` and `(obj[0x14] & 8) == 0` and `param_5 != 0` | per family: `(type & 0xfff) + 0x1000` (`+0x5000`) for the default; facilities, manufacturing, regiments and fleets use their own ids | `FUN_0042c3b0.c:197-348` |
| `(*(byte *)(obj + 0x14) & 4) == 0` (`+0x50` bit 2, Completed, clear) | `0x2d32` / `0x2d34` by side, module 7 | `FUN_0042c3b0.c:412-430` |

So a damaged ship's cell is the plain portrait plus the damage art, not a plain portrait. The
overlay's resource existence in GOKRES module 10 is `inferred` (the art id is computed, not read
from a resource listing).

The Destroyed column draws the same way. `FUN_0044e500` resolves a destroyed entry through the
same view lookup (`FUN_004f2d10`) and hands it to `FUN_0044edb0` like an operational one, and
`FUN_0042c3b0` tests the view object's `+0x50` bit 9 with no test of the column. A ship the
batteries kill keeps the bit: `FUN_00501ae0` writes the hull loss through `FUN_00501490`, whose
slot `+0x29c` → `+0x298` (`FUN_00501c00`) sets bit 9 on both views, before it calls the destroy
slot `+0xac` (`FUN_004f8660` → `FUN_004f71d0`), which writes only the cause byte `+0x40` and
clears nothing. So a ship destroyed by hull loss is drawn under Destroyed with the damage art.
read-in-full. Port: the lost ship leaves the world, so its entry carries the bit it died with
(`ReportObject::Destroyed`, `operation_report::destroyed_ship`).

Which column: there is no damaged category. The columns are Operational (`+0x78` / `+0x70`),
Destroyed (`+0x88` / `+0x80`) and, on Personnel, Captured. The `0x182` fill puts every ship of the
bombarding fleet into `+0x78` (section 2), and a ship that survives is never moved: only the
`0x302` destroyed-object event adds an id to `+0x88` / `+0x80`. A damaged survivor therefore
sits under Operational, drawn with the damage art.

Where the bit comes from: a hull change `FUN_00501490` (hull damage `+0x60`) copies the new
value into both side views (`FUN_005023b0`) and calls slot `+0x29c` (`FUN_00502930`) → slot
`+0x298` (`FUN_00501c00`), which sets `+0x50` bit 9 through `FUN_004f7800` when the hull damage
or `FUN_005006a0` (the sum of the six damage counters) is non-zero; `FUN_004f7800` also pushes
the bit into both views (`FUN_004fa8f0`, which copies it only when the view's side is covered
by the event, `FUN_004fd600`). The bombarding side's view of its own ship is covered. Whether the
target side's view of an enemy ship holds the bit is `hyp:` (the observation test was not traced
for this path). Slot `+0x254` / `FUN_00503340` is the same setter for the family it covers.

Reachability: the defence phase `FUN_0058dc10` runs KDY-150 fire into the ship's first
accumulator and LNR fire into the second; `FUN_0058d540` then spends X on the shields (leftover
X stays and never reaches the hull), Y on what shields remain and then on the hull, and applies
the hull loss through `FUN_00501ae0(hull loss, 0xf, ...)`. That call adds the loss to `+0x60`
(clamped to the class maximum `slot +0x248`), runs `FUN_00501490` (which sets the damaged bit),
and destroys the ship only when hull damage then equals the maximum. LNR damage that exceeds the
shields but not the remaining hull therefore leaves a damaged, alive ship, so the case occurs in
normal play. `bombardment.md` ("Defence phase") has the damage values.

## 6. Modality, a second alert and the movie

The window is modal. `FUN_0044c410` builds on `FUN_00606380(this, ..., 0x1d6, 0x14b, parent, 0, 7,
0, ctx, 0, 1, 0, 0)`, whose twelfth argument is stored at `+0xb8` (`FUN_00606380.c:69`), and
`FUN_00607390(window, 1)` tests that bit's value in the show slot `FUN_006068f0` (vtable slot 13),
which then pushes the window on the modal stack (`FUN_006010e0`; popped by `FUN_006011f0`).
`FUN_00601320` returns the stack top's HWND.

Window message `0x468` (every `FUN_00422ce0` window open, wParam `0x1c` the report) and `0x482` (the
movie message, below) both begin the same way (`FUN_00422ce0.c:1192-1232`): return at once while the
step counter `FUN_004fcee0` is below 2; post `0x409`; then loop on `FUN_00601340` until it reports
none. `FUN_00601340` sends `0x40d` to the mouse-capture window (`GetCapture`) or, with none, to the
modal stack's top (`FUN_00601320`), and returns true when there was none to send it to. The loop
dispatches the pending messages, runs the frame once (`DAT_006b14cc` slot `+0x2c`, then
`FUN_00401bd0`, which calls `FUN_0040a050` when `FUN_00409270` returns an object) and asks again,
so the game keeps running meanwhile. A window opens only after it.

Whether `0x40d` closes depends on the window's procedure: the Status window (`FUN_0044b250`),
the Encyclopedia (`FUN_0045d8f0`) and the window procedure at `FUN_00467e80` map it to slot `+0x30`
(close); the base window procedure `FUN_006083c0` and the sector window `FUN_004593e0` only release
the capture; `FUN_00606650`, the procedure this report's vtable names at slot 5, has no `0x40d`
case. So a second `0x468` alert (a second Operation Report, or any other window) waits, with the
game running, until the player closes the open report, and then opens: one report at a time,
later ones in the order their messages were posted. A movie waits for an open report the same way.

## In the original (VM A0 at 640x480, and Wine)

From `empire-day128-assault-fails-selonia.sav` (Fleet 1: an Imperial Star Destroyer, a Carrack
and a Stormtrooper Regiment in orbit of Alliance-held Selonia):

- Planetary Assault opens "Assault on Selonia" at once for the Empire, on the Summary view: the
  Imperial picture (AT-ATs in a city) and `0x7092` "Alliance Troops have defended Selonia from
  an Imperial assault." The game clock keeps running while the window is open.
- The rail, top to bottom: close, Summary, Imperial Forces (the green emblem, the fleet owner
  `+0x6c` = 2), System Forces (the red emblem, `+0x68` = 1), Display. The
  list views' subtitles "Imperial Forces" and "System Forces" and the Display text are green.
- The assault opens its list views on Trooper Regiments, a bombardment on Capital Ships.
- Imperial Forces: Capital Ships "Imperial Star Destroyer 1" and "Carrack Light Cruiser 2" (the
  ships' own names); Fighter Squadrons one "TIE Fighter" cell per squadron (the class name), with
  the scroll bar past two rows; Trooper Regiments the Stormtrooper Regiment; each Destroyed
  column "No Casualties".
- System Forces: Trooper Regiments one Alliance Army Regiment operational and one destroyed, so
  the assault splits the holder's regiments to `+0x70`/`+0x80` and the attacker's to the fleet
  side; Defense Facilities "GenCore Level I"; Capital Ships "None" / "No Casualties"; Personnel
  "None" / "None" / "No Casualties" (an empty Captured column reads "None").
- Display: the starfield, `0x7013`, the planet captioned "Selonia" and the Imperial fleet
  picture captioned "Fleet".
- The Message Index's All Messages lists "Assault on Selonia", "Assault on Drall" and "Assault on
  Yaga Minor" under the crossed-sabers (Conflict) icon; a row reopens the same window, and
  Escape from it returns to the Index. Closing the live alert returns to the galaxy.
- The same game on day 242, with the regiment gone from Fleet 1 and Selonia (Planetary Assault
  greyed, the fleet without regiments): General Bombardment opens "Orbital bombardment of
  Selonia" within a second, picture `0x2b9a`, `0x7099` "Imperial ships have conducted an orbital
  strike on the Alliance system of Selonia"; System Forces lists GenCore Level I and the Alliance
  regiment operational, nothing destroyed.

From `alliance-day1169-deathstar-trench-run.sav` (day 1171), the Alliance fleet's assault takes
Ord Pardon: the Alliance art set, title and body red, picture `0x2b98`, `0x7091`; its System
Forces button shows the Imperial emblem, so `+0x68` is the side that held the system before the
run. The list views open on Trooper Regiments; Alliance Forces lists the Alliance Army Regiment,
the Mon Calamari Regiment and the rest of the six; System Forces' Trooper Regiments, Defense
Facilities and Personnel read "None".

From `empire-day53-yaga-orbit.sav` (unshielded, Alliance-held Yaga Minor, days 57-80): Target
Military Facilities, Target Civilian Facilities and three General Bombardments each opened the
report at once with picture `0x2b9a`; System Forces Manufacturing lists the mines operational and
"No Casualties".

From `alliance-day927-orto-orbit.sav` under Wine (Alliance Fleet 1 in orbit of Imperial Orto,
day 927), Target Military Facilities opens "Orbital bombardment of Orto"; Alliance Forces,
Capital Ships lists Medium Transport 1 under Operational with a plain portrait and Corellian
Corvette 1 under Destroyed with the damage art. From `alliance-day927-orto-transport-damaged.sav`
(the same save with Medium Transport 1's damaged bit and 120 of its 360 hull damage set in all
three graph copies) the same order gives the same outcome, and Medium Transport 1 under
Operational carries the damage art over its portrait: a damaged survivor is listed Operational
with the marks, as the list cell's condition reads.

## Open (needs a VM A0 capture)

1. Which edge of the bombard bit raises the alert: the run sets and clears the bit in the step
   of the order, and the report opens within a second.
2. Display with no fleet in view, and the Personnel view with a captured officer.
3. A bombardment that destroys something at the system (`0x2b9b`) and one that kills only ships
   (`0x2b9c`).
4. The caller of the code-2 visit.
5. Where the target side's class-3 row appears in its Index (category `0x100` Conflict).
6. Kind-`0xf` lists for a bystander: whether a side whose view does not hold the bombarding fleet
   gets a report with empty lists (`FUN_00496770` returns without a fleet in view). Strike a
   neutral system with one side and read the other side's Message Index.
