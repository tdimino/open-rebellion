---
title: "Tactical Group Strips: Task-Force and Fighter-Group Buttons"
description: "Entry point for the tactical window's two top strips, both sides: geometry and bitmaps (FUN_0059be80, FUN_0059c030), assigned and unassigned task-force slots, fighter-slot presence, what a click, a number key or an F key does (FUN_0059b130, FUN_0059b450, FUN_0059b620, FUN_0059b730), and the manual's strip passages"
category: "ghidra"
tags: [tactical, strip, task-force, fighters, buttons, hotkeys, manual]
---

# Tactical group strips

Answers task-graph nodes `or-q-task-force-strip-1dy` and
`or-ep-task-force-strip-5kdd.4` from this project's decompiles. This note is
the entry point for the strips. Three notes hold the detail it links to:

- [tactical-strip-buttons.md](tactical-strip-buttons.md)—the button class
  under every strip control: caption string, font and colours, the window
  procedure, hit test, disabled fighter slots, number-key presses, sound.
- [tactical-selection-modes.md](tactical-selection-modes.md)—the
  controller's mode classes, what an unassigned slot does in each, 3D-view
  picks, F-key follow, how a ship joins a task force.
- [strip-bitmap-keys.md](strip-bitmap-keys.md)—the strip bitmaps'
  transparent keys.

Coordinates are in the original 640x480 canvas. Bit names are bits of a
button's state word `+0xc4`: 1 held by the mouse or a key, 2 disabled, 4
selected, 8 toggles bit 4 on release, `0x20` caption shown, `0x10` hit test
on the bitmap's pixels.

## Answer

- Both strips are children of the tactical window (vtable `0x0066bb00`,
  `FUN_0059a8d0.c:21`), built by `FUN_0059be80` and `FUN_0059c030` from
  `FUN_0059b080` (`FUN_0059b080.c:11-12`). Only the two header bitmaps
  depend on the side.
- A click on an assigned task-force slot selects that task force; a second
  click on it leaves nothing selected. A click on an unassigned slot acts
  only while one own capital ship (mode `0xe`, controller flag set) or
  several (mode `0x10`) are selected, and then forms a new task force of
  them in that slot. A click on an active fighter slot selects its group, a
  second click deselects it, and a greyed fighter slot takes no click. The
  headers do nothing.
- Keys `1`-`8` act as a click on task-force slot 1-8 and leave the slot
  held. `F1`-`F8` and `F9`-`F12` move the camera to follow a task force or a
  fighter group that exists and change nothing on the strips.

## Geometry and bitmaps

| Control | Id | Built at | Position (x, y) | Size | Rest `+0x94` | Held `+0x98` | Selected `+0xa0` | Disabled `+0x9c` | Hit test |
|---|---|---|---|---|---|---|---|---|---|
| Task-force header | `0xad01` | `FUN_0059be80.c:39` | (9, 1) | 49 x 19 | 1001 Alliance, 1004 Empire | same | — | — | bitmap pixels |
| Task-force slot `i` (0-7) | `0xad02 + i` | `FUN_0059be80.c:55-56` | (60 + 29 i, 2) | 25 x 17 | 1005 assigned, 1007 unassigned | 1006 assigned, 1007 unassigned | 1006 (`:63`) | 1007 (`:62`), never shown | rectangle |
| Fighter-group header | `0xad0a` | `FUN_0059c030.c:35` | (296, 1) | 49 x 19 | 1008 Alliance, 1010 Empire | same | — | — | bitmap pixels |
| Fighter slot `i` (Red, Blue, Green, Gold) | `0xad0b + i` | `FUN_0059c030.c:52-53` | (347 + 29 i, 2) | 25 x 15 | 1012 + i | 1016 + i | 1016 + i (`:60`) | 1020 (`:59`) | rectangle |

- The task-force loop runs `x = 0x3c` while `x < 0x124`, step `0x1d`
  (`FUN_0059be80.c:46`, `:70-73`), into window `+0xb8`-`+0xd4`; the fighter
  loop runs `x = 0x15b` while `x < 0x1cf`, step `0x1d`
  (`FUN_0059c030.c:43`, `:65-68`), into `+0xa8`-`+0xb4`. The image setter
  `FUN_00603150` maps state 2 to `+0x9c` and state 4 to `+0xa0`
  (`FUN_00603150.c:57-82`); the constructor loads rest and held
  (`FUN_00602150.c:38-66`).
- Side: `FUN_00595c40` is 0 for the Alliance (the window loads `VoiceFxA`
  on 0, `FUN_0059a8d0.c:25-31`). The task-force header is `0x3e9` on 0, else
  `0x3ec` (`FUN_0059be80.c:27-32`); the fighter header `0x3f0` on 0, else
  `0x3f2` (`FUN_0059c030.c:23-28`). The bitmaps come from TACTICAL.DLL,
  resource module 6 (`FUN_0059a8d0.c:24`, `FUN_0059b080.c:9`).
- Headers pass 0 as the constructor's last argument, so they hit-test on
  their bitmap's pixels; slots pass 1 and take their whole rectangle
  (`FUN_00602150.c:115-117`; tactical-strip-buttons.md, "Press, release and
  hit test").
- The squadron bitmaps are 25 x 17 in 15-row buttons. The constructor flags
  the size mismatch (bit `0x20000`, `FUN_00602150.c:93-113`) and the paint
  composes into a buffer of the button's size (`FUN_00602150.c:89`,
  `FUN_00602d30.c:64-68`). A0 `TAC-03-C019` (Empire, save
  `empire-day30-yaga-alert-control`, Red selected) shows the top 15 rows: at
  x 390, rows 2-16 match bitmap 1013's rows 0-14 and rows 17-18 show the
  strip background.
- Every strip bitmap is 8-bit and keys on its first stored byte
  ([strip-bitmap-keys.md](strip-bitmap-keys.md)).
- A task-force slot's caption is TEXTTACT `0xad81 + i`, the digit `i + 1`, in
  application font 4 (Arial, 16-pixel cell), centred in the slot, drawn only
  while the slot is assigned (bit `0x20`): black at rest, white held or
  selected (`FUN_0059be80.c:64`, `:66`, `:69`;
  [tactical-strip-buttons.md](tactical-strip-buttons.md), "Font" and
  "Colours"). Fighter slots and headers have no caption.
- Hover texts: `Task Forces`, `Unassigned Task Force` or `Task Force n`,
  `Fighter Squadrons`, `Inactive Fighter Group` or `Red`/`Blue`/`Green`/`Gold
  Fighter Group` (TEXTTACT ids in tactical-strip-buttons.md, "TEXTTACT ids
  used by both strips"). `FUN_00600c40` sets a control's hover text: it loads
  the string (`FUN_00600c40.c:16-21`), stores it at the control's `+0x84`
  (`FUN_00600cc0.c:13`, `FUN_00600860.c:11-27`) and sends
  `TTM_UPDATETIPTEXTA` to the tooltip window (`FUN_00600cc0.c:14-21`).

## Assigned and unassigned task-force slots

- `FUN_005c52e0` keeps a second image pair `+0x114`, `+0x118`, both 1007
  here, with `+0x11c = 1` (`FUN_005c52e0.c:27-40`; arguments `0x3ef, 0x3ef`
  at `FUN_0059be80.c:55-56`).
- `FUN_0059be80` sets toggle bit 8 (`FUN_0059be80.c:61`) and then calls
  `FUN_005c54d0` on every slot (`:65`). That swaps the pairs, so rest and
  held paint 1007 and the pair holds 1005 and 1006, and clears bits `0x20`
  and 8 (`FUN_005c54d0.c:7-15`). Every slot starts unassigned: no caption,
  no toggle.
- `FUN_005c5480` assigns: it swaps 1005 and 1006 back and sets bits `0x20`
  and 8 (`FUN_005c5480.c:7-15`).
- `FUN_0059b550` refreshes the strip from the controller's eight task-force
  ids at `+0x164` (`FUN_0059fc50.c:12-18`, `FUN_0059f5d0.c:5-6`, through
  `FUN_005c0e00.c:5-6`). A slot whose task force exists gets hover text
  `Task Force n` and `FUN_005c5480`; one without gets `Unassigned Task
  Force` and `FUN_005c54d0` (`FUN_0059b550.c:12-26`). With an index below 8
  it sets bits 1 and 4 on that slot and clears them on the other assigned
  slots (`:27-34`).
- `FUN_005a03f0` stores a new task force's id and refreshes
  (`FUN_005a03f0.c:5-7`). `FUN_005a0410` clears one, installs the base mode
  when that task force is selected, and refreshes
  (`FUN_005a0410.c:16-38`); it runs when the last ship leaves a local task
  force (`FUN_005a8d30.c:13-20`, `FUN_005aae40.c:13-27`).
- No strip code sets bit 2 on a task-force slot, so an unassigned slot is
  enabled and takes a click (tactical-strip-buttons.md, "Task-force slots").
  A0 `TAC-02-C006` (Empire, task forces 1 and 2) shows slot 3 blank.

## Fighter-slot presence

- Each fighter slot starts with toggle bit 8 (`FUN_0059c030.c:58`) and is
  then disabled through vtable `+0x10` (`:63`): `FUN_006028a0` sets bit 2
  and disables the window (tactical-strip-buttons.md, "Fighter slots and
  vtable +0x10").
- `FUN_0059b690` enables the slot of each group whose id the controller
  holds at `+0x154 + 4 i`, with hover text `<colour> Fighter Group`, and
  disables the others, with `Inactive Fighter Group`
  (`FUN_0059b690.c:13-28`). It runs through `FUN_005c0e40`
  (`FUN_005c0e40.c:5-6`) from `FUN_0059fca0`, which reads the four ids
  (`FUN_0059fca0.c:12-18`, `FUN_0059f5f0.c:5-6`), and from `FUN_0059f680`
  (`FUN_0059f680.c:17-18`).
- `FUN_005a0320` clears a group's id, installs the base mode when that
  group's mode (9-12) is current, and refreshes (`FUN_005a0320.c:16-38`).
  It runs when the local side's group takes order 2 or 6
  (`FUN_005ca530.c:65-71`; order 6 is Attack Death Star,
  tactical-command-assignment.md) and when a local group has no squadron
  left (`FUN_005a8d30.c:23-29`, `FUN_005aae40.c:28-40`).
- `FUN_0059f680` writes the group's id back and refreshes
  (`FUN_0059f680.c:9-19`). Its caller is the trench run's success, state 6
  of `FUN_005cfec0` (`FUN_005cfec0.c:27`, `:103-108`;
  tactical-death-star-path.md), for the local side's group.
- A disabled slot paints 1020, colour `+0xc0`, and refuses every point
  (`FUN_00602d30.c:12-15`, `:26-33`; `FUN_006035f0.c:24-26`). A0
  `TAC-05-C023` (Alliance, save `alliance-day1169-deathstar-trench-run`, Red
  committed to Attack Death Star) shows the Red slot greyed
  (tactical-command-assignment.md).

## Click

The button class (tactical-strip-buttons.md, "Press, release and hit
test"): a press on the button sets bit 1, captures the mouse and posts
`0x40e`, which the tactical window ignores. The release, on the button or
one pixel past its right or bottom edge, clears bit 1, flips bit 4 when bit
8 is set, and posts `WM_COMMAND` with the id. The tactical window's
procedure passes it to `FUN_0059b130` (`FUN_0059ab40.c:92-94`), which sends
`0xad02`-`0xad09` to `FUN_0059b450` and `0xad0b`-`0xad0e` to `FUN_0059b620`
(`FUN_0059b130.c:14-29`). The header ids reach the default case and nothing
happens (`:12-13`); headers have no toggle bit (`FUN_00602150.c:37`).

**Task-force slot, `FUN_0059b450`.** It reads which task forces exist
(`FUN_00595ba0` -> `FUN_0059a490` -> `FUN_005a0bf0`, controller `+0x164`;
`FUN_0059b450.c:14`, `FUN_005a0bf0.c:8-15`). Walking the slots, it returns
at the clicked slot when its task force does not exist and `FUN_00595b90`
is 0 (`FUN_0059b450.c:18-19`, `:53`): no command, no slot changed. Else, for
each slot (`:24-49`):

- An existing task force's slot: the clicked one posts command `2 + i`
  (`:31-32`); every other loses bits 1 and 4 (`FUN_006030f0(5)`, `:35`).
- The clicked unassigned slot: hover text `Task Force n` (`FUN_00600c40`
  with record `+0xd8 + 4 i`, `:40`), assigned (`FUN_005c5480`, `:41`), bits
  1 and 4 set (`:42`), command `2 + i` (`:43`).

`FUN_00595b90` -> `FUN_0059a520` -> `FUN_0059f6e0` is true when the current
mode's id is `0xe` with the controller flag set, or `0x10`
(`FUN_0059f6e0.c:7-13`). `FUN_00595b10` -> `FUN_0059a470` -> `FUN_005a0ad0`
queues a command object (vtable `0x0066bd48`) on the controller's `+0xec`
(`FUN_005a0ad0.c:14-25`).

The executor `FUN_0059fd40` runs command `2 + i`
(`FUN_0059fd40.c:24-59`):

- Task force `i` exists and its mode (id `i + 1`) is current: the base mode,
  nothing selected.
- Task force `i` exists otherwise: the task-force mode `FUN_005a33d0(i)`,
  which releases the fighter strip and focuses a ship of the task force
  (`FUN_005a33d0.c:23-57`).
- Task force `i` does not exist and the flag is set: the current mode's
  `+0x14(i)`. In `0xe` (`FUN_005a2160`) and `0x10` (`FUN_005a28a0`) it forms
  a task force of the selected ships in slot `i` through `FUN_005a34e0`,
  which stores its id and presses slot `i` (`FUN_0059fc50(i)`); every other
  mode returns nothing (tactical-selection-modes.md, "Unassigned task-force
  slot").

The base mode's constructor releases both strips (`FUN_005a0c80.c:16-17`).

**Fighter slot, `FUN_0059b620`.** It reads which groups exist
(`FUN_00595bc0` -> `FUN_0059a4b0` -> `FUN_005a0c20`, controller `+0x154`;
`FUN_0059b620.c:12`). For each existing group, the clicked slot posts
command `0xb + i` (Red `0xb`, Blue `0xc`, Green `0xd`, Gold `0xe`) and every
other loses bits 1 and 4 (`:16-30`). The executor installs the base mode
when that group's mode (id `9 + i`) is current, else the fighter mode
`FUN_005a4640(i)` (`FUN_0059fd40.c:61-89`), which releases the task-force
strip and shows the fighter panel `0x17` (`FUN_005a4640.c:20`, `:40`;
tactical-command-assignment.md).

**Selected look.** Assigned task-force slots and fighter slots carry bit 8,
so a click's release flips bit 4: the first click shows the selected look,
and a second click on the same slot clears bit 4 while its command installs
the base mode, whose constructor clears bits 1 and 4 on every strip slot.
The paint (`FUN_00602d30.c:12-62`; the table in tactical-strip-buttons.md,
"Colours"):

- Bit 2: the disabled image.
- Bit 1: the held image and white caption while `+0xf8` is 1, the rest
  image and black caption while it is 0 (`:19-25`, `:43-45`).
- Bit 4 alone: the selected image, white caption.
- Otherwise the rest image.

`+0xf8` holds the last hit test (`FUN_006035f0.c:24-38`) and is 1 after a
click or a key on the slot (`FUN_00603710.c:9`). So a slot holding bits 1
and 4 from `FUN_0059b450` or `FUN_0059b550` paints 1006 with a white digit
when it was the one clicked. Selected, the task-force digit turns white
over the darker 1006; the fighter slot shows 1016 + i, which carries more of
the group colour than 1012 + i (mean saturation 36 against 23 on the
TACTICAL.DLL bitmaps).

**3D-view picks.** A plain left click on a ship releases both strips and
presses no slot, even for a ship in a task force (A0 `TAC-03-C018`;
tactical-selection-modes.md, "A ship pick and the strip").

## Keys

- **`1`-`8`** (`WM_KEYDOWN` or `WM_SYSKEYDOWN`): `FUN_0059b730` clears bits
  1 and 4 on all eight task-force slots (`FUN_0059b430.c:8-14`) and all four
  fighter slots (`FUN_0059b600.c:8-14`), then calls `FUN_0059ca90(this,
  0xad02 + i, 1)` (`FUN_0059b730.c:58-97`). `FUN_0059ca90` finds the slot
  and calls `FUN_00603710` (`FUN_0059ca90.c:7-11`), which sets `+0xf8 = 1`
  and bit 1 and posts `WM_COMMAND` with the slot's id to the tactical window
  (`FUN_00603710.c:9-20`). The key therefore runs `FUN_0059b450` as a click
  does, the assigning branch included, and its slot paints the held look:
  1006 with a white digit when assigned, 1007 when not. Bit 4 is untouched.
- The key-up branch has no case for `0x31`-`0x38`
  (`FUN_0059b730.c:133-179`), so bit 1 stays set until the next number key,
  a click on another slot, a strip refresh, the base mode's release or a
  mouse release on the slot (tactical-strip-buttons.md, "Number keys and
  FUN_00603710").
- The key of the selected task force posts `2 + i` again, and the executor
  installs the base mode (`FUN_0059fd40.c:39-41`, `:51-55`): nothing selected.
- No key selects a fighter group.
- **`F1`-`F8`** (VK `0x70`-`0x77`) give index 0-7 by fall-through
  (`FUN_0059b730.c:98-112`). When that task force exists (`FUN_00595ba0`,
  `:113-114`) the window calls `FUN_005c2990(view, 0xb, i)` (`:115`).
  **`F9`-`F12`** (`0x78`-`0x7b`) give 0-3 and call `FUN_005c2990(view, 0xb,
  i + 8)` when that fighter group exists (`FUN_00595bc0`, `:119-130`).
  `FUN_005c2990`'s default case builds the follow camera `FUN_005d9a70`
  (`FUN_005c2990.c:87-114`), which writes no selection, mode or strip state
  (tactical-selection-modes.md, "F-key follow").
- No strip slot or header plays a press sound
  (tactical-strip-buttons.md, "Press sound").

## Both sides

The header bitmaps (1001 and 1008 Alliance, 1004 and 1010 Empire) and their
transparent keys are the only side branch in the strip code. Geometry, slot
bitmaps, captions, hit test, clicks, keys and the mode classes are the same
for the Empire and the Alliance; ownership is the ship's side against the
battle's local side (tactical-selection-modes.md, "Mode classes"). Order 6
is held only by Alliance groups (tactical-death-star.md), so a fighter slot
greyed by Attack Death Star, and restored by the trench run's success,
occurs on the Alliance side only; empty groups grey a slot on either side.

A0 evidence: Empire `TAC-02-C006`, `TAC-02-C012`, `TAC-03-C012`,
`TAC-03-C018`, `TAC-03-C019`; Alliance `TAC-05-C023`.

## Manual cross-check

Read in full: `FUN_0059b130`, `FUN_0059b450`, `FUN_0059b620`,
`FUN_0059b730`, `FUN_0059be80`, `FUN_0059c030`, `FUN_0059fd40` cases
`2`-`0xe`.

| Manual | Claim | Status | Trace |
|---|---|---|---|
| p. 143 | "Select all the ships in a task force by clicking on the task force number at the top left of the screen (Fig. 4.4)" | Confirmed | `FUN_0059b450.c:31-32` posts `2 + i`; `FUN_0059fd40.c:39-46` installs the task-force mode; the strip spans x 9-288 |
| p. 143 | "Select all the fighters in a fighter group by clicking on the fighter group's color at the top right of the screen" | Confirmed | `FUN_0059b620.c:18-21` posts `0xb + i`; `FUN_0059fd40.c:61-75` installs the fighter mode; the strip spans x 296-459, right of the task forces |
| p. 143, Fig. 4.4 | "Click on a number to select a task force, highlighting that task force's number" | Confirmed | The selected look paints 1006 with a white digit (`FUN_0059be80.c:63-64`, `FUN_00602d30.c:56-58`); rest is a black digit on 1005 |
| p. 143, Fig. 4.4 | "Click on a color to select a fighter group, highlighting that fighter group's color" | Confirmed | Selected image 1016 + i (`FUN_0059c030.c:60`), more saturated than 1012 + i |
| p. 143 and Fig. 4.4 | "CTRL-select the ships to be in the task force, then click on the blank space next to the highest number... A number appears in the blank space"; "Click on the next empty space to create a new task force made up of selected ships" | Confirmed; any unassigned slot works | Modes `0xe` and `0x10` (`FUN_0059f6e0.c:7-13`), `FUN_005a2160`, `FUN_005a28a0`; `FUN_0059b450.c:18-19` tests only the clicked slot; the digit appears through `FUN_005c5480` (`FUN_0059b450.c:41`). One selected ship also forms a task force. With nothing selected the click does nothing (A0 `TAC-02-C012`) |
| p. 146 | "To select a different ship... select a different task force from the panel at the top of the screen" | Confirmed | The task-force mode replaces the controller's focused ship with one of the task force's (`FUN_005a33d0.c:53-57`); which ship is open |
| p. 148 | "To select a task force or fighter group, click on the task force number or fighter group color button at the top of the screen" | Confirmed | As p. 143 |
| p. 148 | "To follow a task force, press F1-F8... F9 for red, F10 for blue, F11 for green, and F12 for gold" | Confirmed; only for a group that exists, and the selection is unchanged | `FUN_0059b730.c:98-130` |
| p. 66 | "1 - 8 Select Task Force 1-8" | Confirmed | `FUN_0059b730.c:58-97` -> `FUN_00603710` -> `FUN_0059b450`. The key of the selected task force deselects it; in modes `0xe` and `0x10` the key of an unassigned slot forms a task force |
| p. 66 | "F1 - F8 Camera Follows Task Force 1-8"; "F9 Follow Red Fighter Group" through "F12 Follow Gold Fighter Group" (the PDF; the Markdown transcription reads `T1 - T3`, `T5`, `T10`-`T12`) | Confirmed | As p. 148 |
| p. 143, Fig. 4.3 | Callouts "Task forces" (top left) and "Fighter groups" (top, right of them) | Confirmed | Geometry table above |

The manual does not mention the headers, the greyed fighter slot (`Inactive
Fighter Group`), that the number key of the selected task force deselects
it, or that a key press leaves the slot held.

## Port

`crates/rebellion-render/src/tactical_view.rs` places the headers and slots
at the original's positions (`TASK_FORCE_HEADER_POINT`,
`TASK_FORCE_BUTTONS`, `FIGHTER_GROUP_HEADER_POINT`,
`FIGHTER_GROUP_BUTTONS`) and draws the fighter bitmaps clipped to 15 rows. A
press lands on a slot anywhere in its rectangle (`group_strip_slot_at`); the
captured slot acts on a release that `group_strip_release_hits` accepts, one
pixel past the right and bottom edges included, through
`activate_group_strip_slot`. Both headers draw through the keyed button blit.
A slot's command (`run_task_force_slot_command`,
`run_fighter_group_slot_command`) installs its group's mode
(`BattleSession::selected_task_force`, `selected_fighter_group`), and a
second command on the selected slot clears the selection. An unassigned
task-force slot forms a task force of the selected ships only under one own
ship with the player-command flag set, or two or more
(`unassigned_task_force_slot`); in `0x10` with the flag clear the slot
keeps its caption for the rest of the battle and its press until a number
key, a pick or a miss releases the strip (`TacticalStripOverride`). Keys
1-8 run the slot's command after releasing both strips, again on each
autorepeat (`FUN_0059b730.c:8-11`; the app queues every key-down through
`TacticalState::set_frame_key_downs`); F1-F12 follow an existing group
without selecting it (`tactical_key_action`). A ship pick releases both
strips, and Ctrl+click on
an own ship outside the selected task force moves it in while the player
commands (`pick_own_ship`); after Withdraw clears that flag every pick
selects one ship (`FUN_005a04f0.c:54-70`). A Ctrl+click on empty space
keeps the selection (`FUN_005c2060.c:117-120`), and a selected task force
that loses its last ship returns to the base mode (`FUN_005a0410`,
`drop_emptied_task_force`). Withdraw empties both strips and returns a
group's mode to the base mode (`FUN_005ad530.c:37-41`, `:61-81`); every
strip check reads membership through `in_player_task_force`, which drops a
jumping ship and a withdrawn side. port: the engine keeps the withdrawing
ships' `task_force` and lets one ship of a task force jump at a time, where
the original gives each capital ship a group of its own first
(`FUN_005aded0.c:21-37`; or-p-withdraw-regroup-98kv).
Assigned task-force slots draw their digit in font 4's size, smoothed
Liberation Sans, at `task_force_label_origin` in `task_force_label_color`;
C6 masks the caption cell of each slot assigned in the A0 frame
(`task-force-caption-1`..`8`), so a digit on an unassigned slot fails. A task
force keeps its members in the order they entered it
(`TacticalShip::task_force_joined`), and selecting it focuses the first.

## Task-force member order

`FUN_005a34e0` focuses the head of the task force's member list
(`FUN_005c6070(piVar2 + 0x11)`, stored by `FUN_0051c9b0`). A0 TAC-03-C014
and C015 (Yaga, day 30) show the list in join order: once the ISD, first in
the fleet roster, joins the Carrack's task force 2 and the force is
reselected, the panel shows the Carrack, next the ISD, and previous the
Carrack again. The member add itself (task-force vtable `+0x4c`, from
`FUN_005a8de0`) is not read. The port cannot yet
take a ship out of a task force, and its other 3D-view picks do not follow
the modes' `+0xc` (`or-p-tactical-mode-picks-mi0z`).

## Open

- `+0xf8` on a slot that was never clicked or keyed: `FUN_005ff910` and
  `FUN_00602150` do not write it, and the allocator does not clear memory
  (`FUN_00618b70` -> `FUN_00619190`). A slot that `FUN_0059b550` presses
  without a click (a task force formed by an order or a ship pick in
  `0x10`, or the press `FUN_0059e760` restores at battle-window setup)
  paints the held look only if `+0xf8` is 1. No A0 frame shows such a slot.
- Whether selecting a group plays a voice: the executor
  and the constructors `FUN_005a33d0` and `FUN_005a4640` call none, and
  `FUN_005a6290`, `FUN_005a6340`, `FUN_005a6460`, `FUN_005a6470` are not
  read.
- In `0x10` with the controller flag clear (after Withdraw), the
  unassigned slot captions, holds and posts, but no task force forms; it
  keeps its caption for the battle and its press until a number key or a
  pick. Code reading; no A0 frame.
- A click on a greyed fighter slot reaches no strip code; where Windows
  sends it is Windows behaviour, and no A0 capture clicks one.
- No Alliance A0 frame of a slot selection, a blank-slot click or a ship
  pick was compared; the code has no side branch there.

## Decompiles

New with this note: `FUN_0059f5d0.c`, `FUN_0059f5f0.c`, `FUN_005c0e40.c`,
`FUN_00619190.c`.

Also cited: `FUN_0059a8d0.c`, `FUN_0059b080.c`, `FUN_0059be80.c`,
`FUN_0059c030.c`, `FUN_005c52e0.c`, `FUN_005c5480.c`, `FUN_005c54d0.c`,
`FUN_00602150.c`, `FUN_00603150.c`, `FUN_00602d30.c`, `FUN_006035f0.c`,
`FUN_00595c40.c`, `FUN_00600c40.c`, `FUN_00600cc0.c`, `FUN_00600860.c`,
`FUN_0059b550.c`, `FUN_0059fc50.c`, `FUN_005c0e00.c`, `FUN_005a03f0.c`,
`FUN_005a0410.c`, `FUN_005a8d30.c`, `FUN_005aae40.c`, `FUN_0059b690.c`,
`FUN_0059fca0.c`, `FUN_0059f680.c`, `FUN_005a0320.c`, `FUN_005ca530.c`,
`FUN_005cfec0.c`, `FUN_0059ab40.c`, `FUN_0059b130.c`, `FUN_0059b450.c`,
`FUN_0059b620.c`, `FUN_00595ba0.c`, `FUN_00595bc0.c`, `FUN_00595b90.c`,
`FUN_00595b10.c`, `FUN_0059a470.c`, `FUN_0059a490.c`, `FUN_0059a4b0.c`,
`FUN_0059a520.c`, `FUN_005a0bf0.c`, `FUN_005a0c20.c`, `FUN_0059f6e0.c`,
`FUN_005a0ad0.c`, `FUN_006030f0.c`, `FUN_0059fd40.c`, `FUN_005a33d0.c`,
`FUN_005a4640.c`, `FUN_005a0c80.c`, `FUN_0059b730.c`, `FUN_0059b430.c`,
`FUN_0059b600.c`, `FUN_0059ca90.c`, `FUN_00603710.c`, `FUN_005c2990.c`,
`FUN_00618b70.c`.
