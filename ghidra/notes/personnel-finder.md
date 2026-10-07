# The Personnel Finder

Recovered 2026-10-06 from REBEXE.EXE with Homebrew Ghidra 12.1.3 (read-only,
no analysis), with STRATEGY.DLL's bitmaps, TEXTSTRA and the manual. Every
function named here has a `FUN_<address>.c` note in this directory.

The manual (pp. 44-45, Fig. 2.39) gives the player's view:

> Click on the **Personnel Finder** icon. This brings up the Personnel Finder
> window (Fig. 2.39). Click on the **SpecForces** button to switch to Special
> Forces. This screen shows the number and location of any Longprobes you
> have available. Select one of the Longprobe locations by clicking on a
> number, then click on the **Open Window** button, or double-click on a
> number. For example, to open the System Defense window for the Longprobe at
> Averam, you would click on number 1 next to Averam.

Fig. 2.39's callouts: "Each type of SpecForce is represented by an icon:
Bothan Spies, Infiltrators, Guerrillas and Longprobes", "List shows the name
of each system, and type and amount of Special Forces on that system",
"Number shows number of that SpecForce regiment in the system. Click on a
number to select it, or double-click to go to that system", "Close button",
"Open Window Button: Go to Sector window for selected SpecForce regiment",
"Characters: From this screen, go back to Character list",
"SpecForces: From Character list, go to this screen".

## Entry

- **The cockpit control.** The Personnel Finder posts `WM_COMMAND 0x12f`.
- **The galaxy view** (`FUN_00422ce0`): `WM_COMMAND` `0x12f`; key F5
  (`WM_KEYDOWN` `0x74`, case `0x74` calls `FUN_0042a180`). Refused while `FUN_004fcee0()` is below 2.
- **The opener** `FUN_0042a180`: unless a window of type `0x17` is already
  open (`FUN_00604500(+0x6c, 0x17)`), it allocates `0x17c` bytes and calls
  `FUN_00463500(.., 0, 0, 0x1d6, 0x14a, view, 0x17)`, a 470 by 330 window,
  shows it and adds it to the view's list.

## The window (type `0x17`, vtable `0x0065a130`)

`FUN_00463500` (the constructor) sets the vtable to `PTR_FUN_0065a130`,
`+0x94` to 2, `+0x98` to 7, `+0xb8` bit 0, `+0x154` to the galaxy view,
`+0x170` a sorted list for characters (name order, `FUN_0060a790(.., 2)`),
and `+0x174` a sorted list for special forces.

The create slot (`+0x38`, `FUN_004637a0`) bounds it to the galaxy view's
rect and builds, by the view's side:

| Part | Alliance | Empire |
|---|---|---|
| Background (two-layer) | 10335, 10586 at overlay | 10336, 10590 |
| Close (`200`, `0x1954` "Close") | `(0x1a7,0x19)`, 10514/10515 | `(0x1aa,0x15)`, 10516/10517 |
| Display (`0xc9`, `0x1953` "Display") | `(0x1a7,0x5d)`, 10518/10519 | `(0x1aa,0x59)`, 10520/10521 |

The Display button (`0xc9`) is not inside the tab group; it is a standalone
button with the same art and position as in the Fleet Finder, placed directly
in the window's child list.

The window has **two tab groups**:

### Side tabs (`+0x144`, control `100`)

At `(0x24,0x48)`, 153 by 41, three buttons with bitmaps:

| Tab | Alliance | Empire | Label |
|---|---|---|---|
| 1 ("All") | 10502/10503 | 10502/10503 | `0x1892` "All Personnel" |
| 2 ("Alliance") | — | — | `0x1893` "Alliance Personnel" |
| 3 ("Imperial") | — | — | `0x1894` "Imperial Personnel" |

### View tabs (`+0x148`, control `0x6e`)

At `(0x1a7,0x93)` Alliance / `(0x1aa,0x8f)` Empire, with two buttons sized
to `(iVar6, iVar7)` (Alliance `0x20` by `0x1f`, Empire `0x2a` by `0x29`):

| Button | Value | Bitmaps | Label |
|---|---|---|---|
| SpecForces (tab 3) | 3 | Alliance 10542/10543, Empire 10544/10545 | `0x1899` "SpecForces" |
| Characters (tab 2) | 2 | Alliance 10546/10547, Empire 10548/10549 | `0x1898` "Characters" |

The Characters button is at y-offset `0x36` within the group. Both buttons
have `FUN_00603150(button, 2, pressed_art)` for pressed-state art.

Every side shares:

- the title, TEXTSTRA `0x1890` "Personnel Finder", at `(0x24,0xe)`, font 5;
- the name label, TEXTSTRA `0x1891` "Name", at `(0x24,0x30)`, font 4;
- the name box (`0xcb`, `FUN_00604cf0`) at `(0x8f,0x2d)`, 250 by 18, white
  text (`0xffffff`);
- the tab's label (`+0x158`) at `(0x28,0x73)` Alliance / `(0x28,0x77)`
  Empire, 283 by 16, font 5;
- the list (`0xca`, `FUN_00607ea0`) at `(0x25,0x8f)`, 349 by 161, rows 350
  by 20 (`0x15e` wide, `0x14` tall), scroll bar art 10653, column mode 2
  (`FUN_00609340`), 36 pixels per column cell (`0x24`).

Bitmaps into the four per-tab background layers:

| Layer | Alliance (side 1) | Empire (side 2) |
|---|---|---|
| Background 1 (icon area) | 10536 | 10538 |
| Background 2 (icon area) | 10537 | 10539 |
| Background 3 (full) | 10335 | 10336 |
| Background 4 (close overlay) | 10586 | 10590 |

There are 4 icon-header bitmaps (10536-10539): two per side. The
`FUN_006075b0` and `FUN_00607610` calls assign them as background layers 2-5
for the list, giving each tab view (Characters vs SpecForces) its own set of
column header icons.

## Characters view (view tab 2)

Built in `FUN_004637a0` after the tab group. Before opening, the code walks
all characters via `FUN_004f3d50(side, {0x30, 0x3c}, 0)`:

- family range `0x30..0x3b` (characters: `0x30` minor Alliance, `0x31`-
  `0x35` major Alliance, `0x38` minor Empire, etc.) plus `0x3c..0x3f`
  (special forces, per `FUN_004f3d50`'s family filter) -- but the filter
  `{0x30, 0x3c}` restricts to characters only.
- Each character's name is taken from `character->+0x2c + 0x34`
  (`FUN_00583c40`), its record's name field.
- `FUN_004edc80(character)` returns the character's rank: 0 = no rank,
  1 = General (has `+0x40`, no `+0x44`), 2 = Admiral (has `+0x44`, no
  `+0x40`). hyp: `+0x2c` is the class record, `+0x40`/`+0x44` are command
  rank flags in the class data.
- Each character is inserted into `+0x174` (the character sorted list) with
  name and rank stored in the row at `+0x54`.

The list is rebuilt via `FUN_00464e10` when the view tab changes.

### List population (`FUN_00464e10`, view tab 2)

The current background is switched (`FUN_006075e0` with layer index 2 or 3).

For each system (`FUN_004f31b0(side)`):

1. `FUN_005040c0(system, 3, tab_side)`: iterator for special forces (family
   `0x3c..0x3f`) at that system. If any exist (`FUN_00513180`), the system is
   listed as a row.
2. The system's fleets (`FUN_004ffef0(system, 3, tab_side)`) are walked.
   For each fleet of the tab's side, `FUN_00502db0(fleet, 3)` iterates
   sub-containers. For each sub-container, `FUN_00504040(sub, 3)` checks
   for special forces. If any fleet has special forces, the system is listed.

Each row renders a 350 by 20 bitmap strip via `FUN_00465540`.

### List population (`FUN_00464e10`, view tab 3: SpecForces)

The same two-level walk (direct at system, then in fleets) but using
`FUN_005040c0` (special forces `0x3c..0x3f`) instead of `FUN_00504cc0`
(regiments `0x10..0x13`). hyp: the SpecForces view therefore lists all
systems that contain special forces on the selected side, whether
in a fleet or directly on the system. Fleets are not listed as
separate rows; the system that contains the fleet is listed.

### The character row text (`FUN_00465bb0`)

For each character in `+0x174`, this function builds the row's display text
as a formatted string. The logic:

1. Call slot `+0xc` on the character to get its location id. If zero (the
   character has no location), skip to the end.
2. Build the location name string from `FUN_004f62d0(character->+0x1c)`
   (the character's container record name).
3. Check the character's state:
   - `+0x14` bit 3 set (`0x08`): the character is dead or captured. The
     state word depends on `+0x10 & 0xff`:
     - If `0x14..0x16` (family byte of a regiment): append a state string
       but the decompile shows both branches doing `FUN_005f3010` with
       an empty string argument. hyp: the state is "Killed" or similar,
       but the exact string is in the alternate branch.
   - `+0xac` bit 0 clear and the character's DatId resolves to a location in
     family `0x50..0x7f`: append `0x8745` "On Mission".
   - `+0xac` bit 0 set: append `0x8746` "Captured".
   - `+0x94` (enhanced loyalty) != 0: append `0x8747` "Injured".
   - `+0x14` bit 4 set (`0x10`): append `0x8744` "Enroute".

4. If any state was appended, format as ` - ` (TEXTSTRA `0x1897`) between
   the name and the state, and append ` (state)` using `DAT_006a8790` "("
   and `DAT_006a878c` ")".
5. If the character has a rank (`+0x96` field):
   - 1 -> `0x8802` "Commander"
   - 2 -> `0x8801` "Admiral"
   - 3 -> `0x8800` "General"
   The rank is appended in parentheses the same way.
6. Final format: `"CharacterName - LocationName (state) (rank)"`, stored at
   `+0x14` of the row item via `FUN_005f35e0`.

The row carries `+0x6c` = 0 initially. `+0x1b` (at `param_1[0x1b]`) is set
to 1 when text was built, controlling whether the final assembly runs.

### State words

| TEXTSTRA | Hex | Text |
|---|---|---|
| `0x8744` | 34628 | "Enroute" |
| `0x8745` | 34629 | "On Mission" |
| `0x8746` | 34630 | "Captured" |
| `0x8747` | 34631 | "Injured" |
| `0x8800` | 34816 | "General" |
| `0x8801` | 34817 | "Admiral" |
| `0x8802` | 34818 | "Commander" |

## SpecForces row bitmap (`FUN_00465540`)

Each row renders a 350 by 20 bitmap strip with 4 columns, each 28 pixels
wide, starting at x=222 (`0xde`). The 4 columns represent 4 special forces
types:

| Column | Alliance (side 1) | Empire (side 2) |
|---|---|---|
| 0 | `0x3c000004` | `0x3c000007` |
| 1 | `0x3c000002` | `0x3c000006` |
| 2 | `0x3c000001` | `0x3c000008` |
| 3 | `0x3c000003` | `0x3c000005` |

The column header brushes are at `+0x15c..+0x16c`, the same 5 colors as the
Troop Finder (red/green/gray/dark-red/dark-green). Each column shows the
type's TEXTSTRA name and a count of units present, with the brush color
indicating own/enemy and active/inactive status.

## Choosing and opening

The command slot (`FUN_00464c20`):

- `0x407` (Enter) with `0xcb` (the name box): if `+0x178` (chosen id) is
  valid and differs from null, call `FUN_00429440(view, +0x178)` to open it,
  then close.
- `0x408` (text changed) with `0xcb`: scan the SpecForces list `+0x174`'s
  `+0xa0` sub-list via `FUN_005f3040` to get the text length. If non-zero,
  `FUN_00609650` finds the best prefix match, and `FUN_0060a860` retrieves
  the matched row's id, stored via `FUN_0042d170` and `FUN_004f26d0` into
  `+0x178`. If no match, `+0x178` is zeroed.

hyp: typing a system name in the name box matches against the character list
(view tab 2) or the SpecForces list (view tab 3), depending on which sorted
list is in the active view. The SpecForces grid itself is not directly
searchable by system name through the name box; the name box searches row
names (character names or system names with special force counts).

### Display/open targets via `FUN_00429440`

`FUN_00429440(view, id)` dispatches on the id's family byte:

| Family | Entity | Action |
|---|---|---|
| `0x20..0x2f` (facilities) | Walk up to system via `+0x1c` chain. If system container is not `0xf2`, open sector (`FUN_00429ce0`). If family `0x22..0x27` (shipyard-like), open Fleet window (kind 10); if `0x28..0x2f` (defense-like), open Defense window (kind 9). |
| `0x14..0x1b` (capital ships) | Get fleet container (`+0x1c`). Check fleet's container is not `0xf2`. Get system. Open sector, then Fleet window (kind 4), and select the ship. |
| `0x1c..0x1f` or `0x10..0x13` (fighters/regiments) | Get parent. If parent is a fleet (`0x14..0x1b`), get fleet's system. Else if parent is a system (`0x90..0x97`), use it directly. Open sector, then appropriate window (kind 10 for fleet-based, kind 4 for fleet). Select the entity. |
| `0x30..0x3f` (characters/specforces) | Get parent. For a fleet parent (`0x14..0x1b`): the fleet's system, its sector and Fleet window (kind 4). For a system parent (`0x90..0x97`): the member's mission key `+0x68` (`FUN_0042d170`) finds its mission (`FUN_004f3000`); a mission that is not hidden (`FUN_00520b70` is 0) opens the Missions window (kind 11, `FUN_0049f130`), anything else the Defenses window (kind 10). |
| `0x90..0x97` (systems) | Direct: open sector window, no sub-window. |
| `0x08..0x0f` (characters, alternate range) | Get container, then system. Open sector and Fleet window (kind 4). |

hyp: for characters, `FUN_004f2ec0` is used instead of `FUN_004f2d10`. The
former validates that the family is `0x30..0x3b` (character, not spec
forces).

### `FUN_00520b70` (hidden mission)

Returns the mission class record's `+0x5c`, its hidden flag
(`decoy-roll.md`). A character on a hidden mission still lists in the
Defenses window, so Display opens that; one on a visible mission opens the
Missions window. Manual p. 100: Display opens "the System Window—system
defenses, fleet or mission—in which the character appears". There is no
separate Personnel window; kind 11 is the Missions window
(`sector-quadrants.md`).

## Port notes

- The Personnel Finder has two views: Characters and SpecForces, toggled by
  the view tab buttons. Characters lists individual characters with their
  state; SpecForces lists systems with special force counts.
- Characters that are killed remain listed with their state word ("Captured",
  "Injured", "On Mission", "Enroute"). The `+0x14` bit 3 check suggests
  dead/captured characters stay in the list.
- The SpecForces view lists systems, not individual special forces. Each row
  is a system that contains special forces, counting by type in 4 columns.
- Fleets containing special forces cause their system to be listed; the fleet
  itself is not a separate row.
- Display opens the sector window and the appropriate sub-window based on the
  selected entity's family. A character goes through
  `personnel_finder::character_target`: its fleet, the Missions window
  (showing its mission, slot 27 `FUN_004a1e10`), or the Defenses window.
- port: the name box searches character names in Characters view, and system
  names in SpecForces view (hyp).
- port: dead characters are listed; this differs from the Troop Finder which
  only lists active regiments.

## Open

- The exact mapping between `FUN_00465bb0`'s `+0x14` bits and character
  states (the dead/captured branch at `+0x14 & 0x08`).
- Whether the Characters view lists characters from all systems or just the
  player's.
- The `+0x6c` field on each row item and its purpose.
- The exact view tab that `FUN_00464e10` reads from `+0x148`'s `+0x94`
  pressed-button state.
- The bitmaps 10536-10539: which pair is which view's column header icons.

## Supporting decompiles

`FUN_0042a180`, `FUN_00463500`, `FUN_004637a0`, `FUN_00464c20`,
`FUN_00464e10`, `FUN_00465bb0`, `FUN_00465540`, `FUN_004edc80`,
`FUN_00520b70`, `FUN_00429440`, `FUN_004f31b0`, `FUN_004f3d50`,
`FUN_004f2d10`, `FUN_004f2ec0`, `FUN_005040c0`, `FUN_00504040`,
`FUN_004fcee0`, `FUN_00604500`.
