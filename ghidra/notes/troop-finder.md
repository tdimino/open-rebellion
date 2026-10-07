# The Troop Finder

Recovered 2026-10-06 from REBEXE.EXE with Homebrew Ghidra 12.1.3 (read-only,
no analysis), with STRATEGY.DLL's bitmaps, TEXTSTRA and the manual. Every
function named here has a `FUN_<address>.c` note in this directory.

The manual (pp. 128-129; Figs. 3.75-3.76) gives the player's view: the
Manufacturing and Production window for a system with a Training Yard shows
"Troops in Training". The Troop Finder is opened from the cockpit button or
the F5 key.

## Entry

- **The cockpit control.** `FUN_00427270` builds the cockpit's buttons. The
  Troop Finder posts `WM_COMMAND 0x130`: the Alliance's and Empire's cockpit
  buttons share the same control IDs used by the other finders.
- **The galaxy view** (`FUN_00422ce0`): `WM_COMMAND` `0x130` opens the Troop
  Finder; the key F4 (`WM_KEYDOWN` `0x73`, case `0x73` calls
  `FUN_0042a4d0`) reaches the same opener. Refused
  while `FUN_004fcee0()` (`FUN_0051ce00()->+0xc`) is below 2.
- **The opener** `FUN_0042a4d0`: unless a window of type `0x16` is already
  open (`FUN_00604500(+0x6c, 0x16)`), it allocates `0x180` bytes and calls
  `FUN_0046cbc0(.., 0, 0, 0x1d6, 0x14a, view, 0x16)`, a 470 by 330 window,
  shows it and adds it to the view's list. The other finders are
  `FUN_0042a000` (System, type `0x14`), `FUN_0042a0c0` (Fleet, type `0x15`)
  and `FUN_0042a180` (Personnel, type `0x17`).

## The window (type `0x16`, vtable `0x0065a270`)

`FUN_0046cbc0` (the constructor) inherits `FUN_004ac120` and sets the
vtable to `PTR_FUN_0065a270`, `+0x94` to 2, `+0x98` to 7, `+0xb8` bit 0
(modeless finder flag), `+0x150` to the galaxy view, `+0x178` a sorted list
(name order, `FUN_0060a790(.., 2)`), and `+0x15c` a font loaded from
`gokres.dll` resource 10.

The create slot (`+0x38`, `FUN_0046ce40`) bounds it to the galaxy view's
rect (`FUN_00606980`) and builds, by the view's side (`+0x9c`, 1 Alliance,
2 Empire):

| Part | Alliance | Empire |
|---|---|---|
| Background (two-layer) | 10335, 10584 at `(12,13)` | 10336, 10588 |
| Close (`200`, TEXTSTRA `0x1954` "Close") | `(0x1ab,0x19)`, 10514/10515 | `(0x1aa,0x15)`, 10516/10517 |
| Display (`0xc9`, `0x1953` "Display") | `(0x1ab,0x5d)`, 10518/10519 | `(0x1aa,0x59)`, 10520/10521 |
| Side tabs (`100`, `FUN_0060d590`) | `(0x24,0x48)`, 153 by 41 | same position |
| Tab 1 ("Alliance Troops" `0x1950`) | 10502/10503 | 10502/10503 |
| Tab 2 ("Imperial Troops" `0x1951`) | 10506/10507 | 10506/10507 |

Both Close and Display play sound `0x260`. Every side shares:

- the title, TEXTSTRA `0x1900` "Troop Finder", at `(0x24,0xe)`, font 5,
  drawn into the background;
- the subtitle, TEXTSTRA `0x1901` "Troop Location", at `(0x24,0x30)`,
  font 4, drawn into the background;
- the name box (`0xcb`, `FUN_00604cf0`) at `(0x8f,0x2d)`, 250 by 18, white
  text (`0xffffff`);
- the tab's label (`+0x154`) at `(0x28,0x77)`, 283 by 16, font 5: the
  selected tab's text;
- the list (`0xca`, `FUN_00607ea0`) at `(0x25,0x90)`, 349 by 159, rows 330
  by 25 (`0x15e` wide, `0x19` tall), scroll bar art 10653 (`0x299d`).

It opens with the player's side tab selected (`FUN_0060d7e0(+0x144, side,
1)`) and the focus in the name box.

## The list (`FUN_0046ea10`)

When the tab changes, `FUN_006075e0` switches the background bitmap, the
list scrollbar is repositioned, and the sorted list `+0x178` is cleared and
rebuilt:

1. Walk all systems (`FUN_004f31b0(side)`: family `0x90..0x97`).
2. For each system, check `FUN_00504cc0(system, 3, tab_side)`: this iterator
   covers regiments (family `0x10..0x13`) in that system for the given side.
   If any exists (`FUN_00513180`), the system is listed.
3. Additionally, `FUN_004ffef0(system, 3, tab_side)` iterates the system's
   fleets. For each fleet belonging to the tab's side (bits `+0x24 >> 6`),
   `FUN_00502db0(fleet, 3)` iterates its sub-containers (ships). For each
   ship, `FUN_00504c40(ship, 3)` checks for regiments. If any fleet has
   regiments, the system is listed again (under its fleet).
4. Each row stores the system's id (`FUN_004025b0`), name
   (`FUN_004f62d0` -> `FUN_00583c40`), and a bitmap strip rendered by
   `FUN_0046d8d0`.

Rows are inserted in name order (`FUN_005f59f0`).

### The row bitmap (`FUN_0046d8d0`)

Each row renders a 330 by 25 bitmap strip with 5 columns, each 28 pixels
wide (`0x1c`), starting at x=194 (`0xc2`). The 5 columns represent 5 troop
types, loaded by `FUN_004f26d0` with DatIds:

| Column | Alliance (side 1) | Empire (side 2) |
|---|---|---|
| 0 | `0x10000002` (Army Regiment) | `0x10000008` (Stormtrooper Reg.) |
| 1 | `0x10000001` (Fleet Regiment) | `0x1000000a` (ScoutTrooper Reg.) |
| 2 | `0x10000005` | `0x10000007` |
| 3 | `0x10000003` | `0x10000006` |
| 4 | `0x10000004` | `0x10000009` |

For the given system or fleet parent, the code counts how many regiments of
each type are present via `FUN_00402d80` (type extractor). Each column shows
the troop type icon (`FUN_00583c40` -> string ID) and a count. The column
header icon uses 5 GDI brushes at `+0x164..+0x174`:

| Index | Color | Meaning |
|---|---|---|
| 0 | `0x20000ff` (red) | All on player's side, all active (bit 0 set) |
| 1 | `0x200ff00` (green) | All on enemy side, all active |
| 2 | `0x808080` (gray) | None active (bit 0 clear), `+0x50 & 1` == 0 |
| 3 | `0x80` (dark red) | Player side, mix of active and inactive |
| 4 | `0x8000` (dark green) | Enemy side, mix of active and inactive |

hyp: the brushes draw a small status indicator (PatBlt 21 by 16 at the
column header position). Active means `+0x50` bit 0 is set on the regiment.

## Choosing and opening

The command slot (`FUN_0046df90`):

- `100`, a tab: rebuild the list for it via `FUN_0046ea10`.
- `0xca`, the list: notification `0x29b` (click) copies the row's name into
  the name box; `0x309` (double click) resolves the chosen id and opens it.
- `0xc9`, Display: resolve the chosen id `+0x17c` and open it. `200`,
  Close: close.

### Display/open logic (within `FUN_0046df90`, case `0xc9`)

The chosen id at `+0x17c` is resolved via `FUN_004f2d10(side, id)`:

1. If the id is a system (family `0x90..0x97`): get the system's id via
   `FUN_004025b0`, store it, and call `FUN_00429440(view, id)`.
2. If the id is a character (family `0x08..0x0f`): walk up to the system
   container. Check if the system container is not `0xf2`. Call
   `FUN_00429440` with the system.
   hyp: characters are not listed in the Troop Finder, but the Display
   handler shares code with the Personnel Finder.

Then close.

### Name box behavior (`FUN_00464c20`, `0x408`)

When the name box text changes (`0x408` with `0xcb`), the code reads the
name box text and scans the sorted list `+0x174` (the character list in the
Personnel Finder) via `FUN_00609650` for the best prefix match.

hyp: in the Troop Finder, `+0x174` is not used (only `+0x178`). The
name-box handler here (`FUN_00464c20`) is the Personnel Finder's handler
(vtable `0x0065a130`), not the Troop Finder's. The Troop Finder's own
name-box handler is in its vtable. The prefix-match picks the first row
sharing the longest common prefix, scrolls to it and selects it.

### Typed-name behavior (system name search)

The name box accepts typed text. `0x408` (text changed) picks the first row
in list order with the longest prefix in common with the typed text, ignoring
case, selects it and scrolls to it, and keeps its id. Since each row is a
system name, typing a system name moves the grid to that system. This is the
same `FUN_00609650` prefix-match the Fleet Finder uses.

## `FUN_004fcee0` state < 2 refusal

`FUN_004fcee0` returns `FUN_0051ce00()->+0xc`. The function
`FUN_0051ce00()` returns a global singleton (the game state object). Field
`+0xc` is the game phase: a value below 2 means the game has not yet
started the strategic phase (the galaxy view). All four finders (System,
Fleet, Personnel, Troop) refuse to open while the phase is below 2. The F2-
F5 keys and `WM_COMMAND` `0x12d`-`0x130` all check this.

## Port notes

- The Troop Finder lists systems, not individual regiments. Each row is a
  system that has regiments of any type.
- Rows are grouped by troop type in the bitmap strip. The port should show
  the same 5-column layout with counts per type.
- The name box does a prefix search against system names and scrolls the list.
- Display/double-click opens the sector window for the system and its
  Defense tab (kind 10), same as the fleet finder opening kind 4.
- port: the list does not list killed or destroyed regiments; the
  `FUN_00504cc0` iterator filters by side bits and the `FUN_00513180`
  existence check.
- port: the row bitmap column header brush colors indicate own/enemy status
  and active/inactive, paralleling the Personnel Finder.

## Open

- The exact correspondence between troop DatIds (`0x10000001`..`0x1000000a`)
  and the port's regiment types.
- Whether the bitmap strip renders the troop's icon sprite or a text count
  (the decompile shows `FUN_00583c40`, which returns a string from a
  TEXTSTRA-style resource, written at a column position, suggesting text).
- The Troop Finder's own `0x408` name-box handler (if different from
  `FUN_00464c20`).

## Supporting decompiles

`FUN_0042a4d0`, `FUN_0046cbc0`, `FUN_0046ce40`, `FUN_0046ea10`,
`FUN_0046d8d0`, `FUN_0046df90`, `FUN_00429440`, `FUN_004fcee0`,
`FUN_004f31b0`, `FUN_00504cc0`, `FUN_004f2d10`, `FUN_00604500`.
