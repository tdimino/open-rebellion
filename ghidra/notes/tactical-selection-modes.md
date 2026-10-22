---
title: "Tactical Selection Modes"
description: "The tactical controller's mode classes (ids 0-0x11), what installs each, what an unassigned task-force slot does in each, how a 3D-view pick, Ctrl and the strips interact, the Death Star targeting mode, and how a ship joins a task force"
category: "ghidra"
tags: [tactical, selection, task-force, strip, modes, ctrl-click, death-star, camera]
---

# Tactical selection modes

Answers task-graph node `or-ep-task-force-strip-5kdd.3` from REBEXE.EXE
decompiles. Every function named below was read in full unless marked
otherwise. The mode object lives at `DAT_006bc464`; the controller is the
singleton `DAT_006bc468` built by `FUN_0059ed70` (0x188 bytes,
`FUN_0059eb70`). The strip buttons and their ids are in
[tactical-group-strips.md](tactical-group-strips.md) and
[tactical-strip-buttons.md](tactical-strip-buttons.md).

## Answer

1. **Mode classes.** Nine classes derive from the abstract mode
   `FUN_0059e880` (vtable `0x0066bd68`, slots 1-7 `FUN_00618b80`). Their ids
   (vtable `+8`) and unassigned-slot handlers (`+0x14`) are in the table
   below. `0xe` is one own capital ship selected, `0x10` is two or more own
   capital ships selected, `0x11` is the Death Star superlaser's targeting
   mode.
2. **Entering 0xe and 0x10.** A left click on an own capital ship in the
   3D view installs `0xe`; Ctrl+click (either button) on a second own capital
   ship while `0xe` holds installs `0x10`. An unassigned task-force slot acts
   only in `0xe` with the controller flag set, or in `0x10`: in both it makes
   a new task force of the selected ships in the clicked slot. In `0x10` with
   the flag clear the button presses and posts, but no task force forms.
   Every other mode ignores the click.
3. **Base-mode entry.** `FUN_005c0e60` and `FUN_005c0e70` release the eight
   task-force and four fighter-group strip buttons; they hold no selection.
   The selection lives in the mode object, which `FUN_0059f350` deletes when
   the base mode replaces it, and the base constructor zeroes the
   controller's focused ship. So a second click on the selected slot leaves
   nothing selected, no slot pressed and panel `0x15`.
4. **3D-view pick.** A left click on a ship releases both strips and
   installs `0xe` (own) or `0xf` (enemy); it presses no slot, even for a ship
   in a task force. A0 TAC-03-C018 agrees.
5. **F-key follow.** F1-F12 change only the camera. `FUN_005d9a70` and its
   methods read the group and the current mode but write no selection,
   mode or strip state.
6. **Joining a task force.** With a task force selected, Ctrl+click an own
   ship that is not in it and the ship moves into that task force (manual
   p. 143); Ctrl+click a member and it leaves. A new task force forms from a
   one-ship (`0xe`) or multi-ship (`0x10`) selection on a click on any
   unassigned slot, or automatically in the first free slot when a
   multi-ship selection is given an order.

## Mode classes

| `+8` id | Constructor | Vtable | Selection it holds | Installed by | `+0x14` (unassigned slot) |
|---|---|---|---|---|---|
| 0 | `FUN_005a0c80` | `0x0066be30` | nothing | controller start (`FUN_0059eb70.c:84`); every mode's empty click `+0x18`; a second click on the selected slot (`FUN_0059fd40.c:51-58`, `:80-87`); a cleared slot of the current group (`FUN_005a0410.c:19-34`) | `FUN_0048b440`: no mode |
| 1-8 (`+0x54` + 1, `FUN_005a3a10.c:5`) | `FUN_005a33d0` (slot click), `FUN_005a34e0` (new task force) | `0x0066bc48` | task force `+0x54` | task-force slot `2 + i` (`FUN_0059fd40.c:43-46`); `FUN_005a34e0` callers `FUN_005a2160`, `FUN_005a28a0`, `FUN_005a24d0`, `FUN_005a3020`, `FUN_005a3aa0`; order commit in `0x10` (`FUN_005a2c70.c:81`) | `FUN_005a4120`: base |
| 9-12 (`+0x54` + 9, `FUN_005a48c0.c:5`) | `FUN_005a4640` | `0x0066bbf8` | fighter group `+0x54` | fighter slot `0xb + i` (`FUN_0059fd40.c:72-75`) | `FUN_005a5070`: base |
| `0xd` (`FUN_005a5600.c:5`) | `FUN_005a0e70` | `0x0066bb58` | a picked object that is not a capital ship (list `+0x40`) | a non-ship pick (`FUN_005a04f0.c:84-97`, base `+0x10` `FUN_005a0e10`) | `FUN_0048b440`: no mode |
| `0xe` (`FUN_005a5920.c:5`) | `FUN_005a1550` | `0x0066bce8` | one own capital ship (list `+4`) | an own-ship pick (`FUN_005a04f0.c:60-70`, `FUN_005a0d10.c:30-36`); `FUN_005a24d0` when one ship is left; order fallbacks | `FUN_005a2160`: new task force |
| `0xf` (`FUN_005a56b0.c:5`) | `FUN_005a1110` | `0x0066bba8` | one enemy capital ship (list `+0x28`) | an enemy-ship pick (`FUN_005a04f0.c:72-81`, `FUN_005a0d10.c:45`); an untargetable enemy in `0xe`, `0x10`, 1-8 | `FUN_0048b440`: no mode |
| `0x10` (`FUN_005a5980.c:5`) | `FUN_005a2280` | `0x0066bc98` | two or more own capital ships (list `+4`) | Ctrl pick of a second own ship in `0xe` (`FUN_005a1af0.c:113`) | `FUN_005a28a0`: new task force |
| `0x11` (`FUN_005a5b70.c:5`) | `FUN_005a5630` | `0x0066be80` | none | command `0xf` from the superlaser button (below) | `FUN_005a5a90`: base |

- Vtable slots the controller calls: `+0xc` capital-ship pick (id, Ctrl),
  `+0x10` other pick (id, Ctrl) (`FUN_005a04f0.c:56`, `:86`); `+0x14`
  unassigned slot (`FUN_0059fd40.c:36`); `+0x18` empty click
  (`FUN_005a04f0.c:28`); `+0x1c` Space, command `0x20` (`FUN_0059fd40.c:134-139`,
  `FUN_0059b730.c:21-22`); `+0x20` order (`FUN_0059eec0.c:10`); `+0x28`
  tactic (`FUN_0059eef0.c:10`); `+0x2c` superlaser (`FUN_0059fd40.c:96`);
  `+0x30` per update (`FUN_0059f380.c:19-22`); `+0x34`, `+0x38` queued events
  (`FUN_0059fce0`); `+0x3c` (`FUN_0059e760.c:35`). A returned mode replaces
  the current one and deletes it (`FUN_0059f350.c:5-9`).
- Every mode's `+0x18` installs the base mode (`FUN_005a1080`,
  `FUN_005a13a0`, `FUN_005a2100`, `FUN_005a31f0`, `FUN_005a40c0`,
  `FUN_005a5010`, `FUN_005a5a30`); the base mode's is `FUN_006158b0`, no mode.
- Constructors and the strips: base, `0xd`, `0xe`, `0xf`, `0x10` and `0x11`
  release both strips (`FUN_005a0c80.c:16-17`, `FUN_005a0e70.c:18-19`,
  `FUN_005a1550.c:20-21`, `FUN_005a1110.c:20-21`, `FUN_005a2280.c:20-21`,
  `FUN_005a5630.c:16-17`); task-force modes release the fighter strip only
  (`FUN_005a33d0.c:23`, `FUN_005a34e0.c:23`), the fighter mode the
  task-force strip only (`FUN_005a4640.c:20`). Base, `0xd` and `0x10` show
  panel `0x15` (`FUN_005c0e80`; `FUN_005a0c80.c:18`, `FUN_005a0e70.c:20`,
  `FUN_005a2280.c:38`).
- No class branches by side. Ownership is the ship's side `+0x168` against
  the battle's local side `+0x9ec` (`FUN_005a04f0.c:60-62`,
  `FUN_005a0d10.c:30`); only voice ids differ by side (Alliance/Empire:
  escort `0x77`/`0xf5`, attack `0x3b`/`0xb5`, task force n `0x3c`+n/`0xb6`+n
  and `0x78`+n/`0xf6`+n, no free slot `0x84`/`0x102`).

## The controller flag `*this`

The controller's first word is the player-command flag. `FUN_0059eb70`
sets it to 1 (`FUN_0059eb70.c:77`).

- Command `0x7a`, Withdraw, clears it after `FUN_005a8a90` allows the local
  side to withdraw, runs `FUN_005ad530`, releases both strips and shows panel
  `0x15` (`FUN_0059fd40.c:141-167`). `FUN_005ad530` empties every slot of
  the local side's strips: each fighter group's through `FUN_005a0320`
  (`FUN_005ad530.c:37-41`), and each capital ship leaves its task force for
  a group of its own (`:61-81`, `FUN_005a0410`, `FUN_005a8d30`,
  `FUN_005aded0.c:21-37`). A group's mode therefore gives way to the base
  mode (`FUN_005a0320.c:19-35`, `FUN_005a0410.c:19-35`); a ship
  selection's mode (`0xe`, `0x10`) stays. Only
  `FUN_00596a70` posts `0x7a` (`FUN_00596a70.c`, after `FUN_00596a90`),
  called from 0x005f1183, which Ghidra has not defined as a function. The
  Withdraw button is in [morale-retreat.md](morale-retreat.md).
- Command `0x7b` toggles it with the local side controller's `+8` through
  `FUN_005c7e10` (`FUN_0059fd40.c:168-193`, `FUN_005c7e10.c:14`): `+8` 1
  becomes 0 and the flag 1; otherwise `+8` becomes 1 and the flag 0.
  `FUN_005be350` sets `+8` to 0 for the side it is given and 1 for the other
  (`FUN_005be350.c:7-8`), so `+8` 1 marks a computer-driven side.
- Nothing posts `0x7b` or `0x7c`. Commands reach the queue `+0xec` only
  through `FUN_005a0ad0`, whose one caller `FUN_0059a470` is called by
  `FUN_00595b10` and `FUN_00596a70`. The `FUN_00595b10` call sites post
  `0xf`-`0x15` (`FUN_0059b130`), `2`+i (`FUN_0059b450`), `0xb`+i
  (`FUN_0059b620`), `0x20` (`FUN_0059b730`), `0xf` (`FUN_0059cba0`) and
  `0x7d` (0x005daf8f). In play the flag is 1 until Withdraw.
- With the flag clear, a pick bypasses the mode and builds `0xe`, `0xf` or
  `0xd` itself (`FUN_005a04f0.c:54-97`), `FUN_0059f6e0` rejects `0xe`
  (`FUN_0059f6e0.c:7`), the executor skips `+0x14` (`FUN_0059fd40.c:35`),
  command `0xf` does nothing (`FUN_0059fd40.c:92-93`), and `FUN_005a2160` and
  `FUN_005a28a0` return no mode (`:16` in each).

## The 3D view: left click, right click, Ctrl

`FUN_005c2060` is the 3D view's window procedure (vtable entry
`0x0066c974`; recovered from an undefined gap). Ctrl is `wParam & 8`
(MK_CONTROL).

- `WM_LBUTTONDOWN` (0x201) picks at the point and its four diagonal
  neighbours (`FUN_005c2d20`, not read) (`FUN_005c2060.c:108-116`). A miss
  with Ctrl does nothing; without Ctrl it posts pick (0, 0) (`:117-121`). A
  hit with Ctrl posts (id, 1) (`:126-128`). A hit without Ctrl releases both
  strips (`FUN_005c0e60`, `FUN_005c0e70`), then posts (0, 0) and (id, 0)
  (`:130-133`).
- `WM_RBUTTONDOWN` (0x204) picks over a wider ring (`:136-164`); a miss does
  nothing, a hit posts (id, 1) with Ctrl or (id, 0) without (`:165-169`). It
  releases no strip.
- Picks queue on the controller's `+0xfc` (`FUN_00595b70` ->
  `FUN_0059a4d0` -> `FUN_005a0b50`), first in first out (`FUN_005f5e40.c:10-17`
  appends at the tail, `FUN_005f5e90.c:12` pops the head). `FUN_005a0820`
  runs queued events, then commands (`FUN_005a0790`), then picks
  (`FUN_005a0710` -> `FUN_005a04f0`), then the update `FUN_0059f380`
  (`FUN_005a0820.c:5-8`).
- `FUN_005a04f0`: (0, 0) runs `+0x18` (`:23-29`); (0, 1) does nothing
  (`:24-26`). A capital ship (object `vt+4` == 1) runs `+0xc(id, Ctrl)` with
  the flag set (`:54-58`); any other object runs `+0x10(id, Ctrl)` (`:84-88`).

A plain left click therefore always passes through the base mode: (0, 0)
installs it, and (id, 0) runs its `+0xc`, `FUN_005a0d10`, which installs
`0xe` for an own ship and `0xf` for an enemy (`FUN_005a0d10.c:30-45`). A
right click or a Ctrl click reaches the current mode's `+0xc` with the
modifier.

## Ship picks by mode (`+0xc`)

- **`0xe`** (`FUN_005a1af0`): own other ship with Ctrl installs `0x10` with
  both ships (`:113`); without Ctrl the ship is escorted (`FUN_005a8fc0`,
  voice `0x77`) when `FUN_005ae260` allows (`:65-101`). Enemy: without Ctrl
  it becomes the sole target (`:167-171`); with Ctrl it is added or removed
  (`:187-219`); an untargetable one (`FUN_005ae2a0`) installs `0xf`
  (`:134-139`).
- **`0x10`** (`FUN_005a24d0`): own ship with Ctrl toggles membership; one
  ship left installs `0xe` (`:88-103`). Own ship without Ctrl forms a task
  force in the first free slot (`FUN_0059f610`) that escorts it (`:55-71`),
  or installs `0xe` on it when no slot is free (`:80`). An enemy, with or
  without Ctrl, forms a task force in the first free slot with that target
  (`:180-191`); with no free slot each ship targets it alone and `0xe`
  takes the first ship (`:143-169`).
- **`0x10` orders** (`+0x20`, `FUN_005a2c70`): a free slot gets a new task
  force with the order and installs its mode (`:56-81`); with none, each ship
  gets its own group and the order, and `0xe` takes the first (`:25-48`).
  Space (`+0x1c`) finds an enemy (`FUN_005aa970`) and runs `+0xc` on it in
  `0xe` and `0x10` (`FUN_005a21e0.c:25-40`, `FUN_005a3250.c:53-68`).
- **`0xe` orders** (`+0x20`, `FUN_005a1a60`): the ship gets a new group of
  its own (`FUN_005aded0`, `:13`), which takes it out of its task force
  (`FUN_005aded0.c:21`).
- **1-8** (`FUN_005a3aa0`), own ship with Ctrl: one in the group's target
  list is dropped (`:101-106`); a non-member moves into this task force
  (`FUN_005ade50`, `:109-113`); a member leaves it (`FUN_005a8d30`), and the
  last member also clears the slot and installs the base mode (`:116-137`).
  Own ship without Ctrl: a member makes the other members a new task force in
  the first free slot that escorts it (`:53-72`), else `0xe`; a non-member is
  escorted by this task force (`:87-96`).

## Unassigned task-force slot

- The button: `FUN_0059b450` acts on an unassigned slot only while
  `FUN_00595b90` -> `FUN_0059f6e0` holds (`FUN_0059b450.c:18-19`, `:39-44`):
  id `0xe` with the flag set, or id `0x10` (`FUN_0059f6e0.c:7-13`). It then
  sets the caption, switches to the alternate images, presses the slot
  (state 5) and posts `2 + i`. Otherwise it returns at once: no command, no
  release of the other slots. A0 TAC-02-C012 (Empire, save
  `empire-day30-yaga-alert-control`, nothing selected) clicks blank slot 3 at
  (132, 10): the strip (60, 2)-(292, 19) is pixel-identical to TAC-02-C006's.
- The executor: for an unassigned slot with the flag set it runs the mode's
  `+0x14(i)` (`FUN_0059fd40.c:33-38`).
- `0xe`: `FUN_005a2160` builds `FUN_005a34e0(ship list +4, fighter targets
  +0x34, targets +0x1c, i)`, a one-ship task force in slot i
  (`FUN_005a2160.c:16-20`).
- `0x10`: `FUN_005a28a0` does the same with the selected ships
  (`FUN_005a28a0.c:16-20`).
- `0x10` with the flag clear: the button passes (`FUN_0059f6e0.c:10-12`
  ignores the flag), captions, presses and posts, but the executor skips
  `+0x14` and `FUN_005a28a0` also requires the flag, so no task force forms
  and no mode changes. The slot keeps its caption until a `FUN_0059fc50`
  refresh restores unassigned slots (`FUN_0059b550.c:12-17`); after Withdraw
  no task force forms or empties, so none follows. Its press lasts until a
  number key (`FUN_0059b430`) or a new mode releases the task-force strip: a
  pick's `0xe` or `0x10` (`FUN_005a1550.c:20-21`, `FUN_005a2280.c:20-21`)
  or the base mode (`FUN_005a0c80.c:16-18`). Withdraw is the only clearing
  path and keeps a ship selection's mode, so this follows a Withdraw while
  ships are Ctrl-selected.
- Any unassigned slot works: `FUN_0059b450` tests only the clicked slot and
  the executor passes its index. The slot need not be the next blank.
- `FUN_005a34e0` (task-force mode, vtable `0x0066bc48`,
  `FUN_005a34e0.c:22`): `FUN_005ade70` gives the first ship a new group
  (`FUN_005aded0`: out of its old group `FUN_005ade20`, new `FUN_005cbf50`,
  `+0x178` set, `FUN_005aded0.c:21-37`) and moves every other ship in
  (`FUN_005ade50` = `FUN_005ade20` + `FUN_005a8de0`)
  (`FUN_005ade70.c:15-21`); `FUN_005a03f0` puts its id in the controller's
  `+0x164 + 4i` (`FUN_005a34e0.c:30`); the targets carry over
  (`:36`, `:43`); `FUN_0059fc50(i)` presses slot i (`:47`); the mode's
  `+0x54` is i (`:48`). A ship leaving a task force through `FUN_005a8d30`
  clears that task force's slot when no ship is left (`FUN_005ad900` answers
  0; `FUN_005a8d30.c:15-19` -> `FUN_005a0410.c:18-38`).

## Which slot `FUN_0059e760` presses

`FUN_0059e760` runs from the battle window's setup `FUN_005ab0e0`
(from `FUN_0059a4f0`) after `FUN_0059fc50(-1)` and `FUN_0059fca0`
(`FUN_005ab0e0.c:86-90`).

- Modes 1-8: `FUN_0059fc50(id - 1)` (`FUN_0059e760.c:20-21`) ->
  `FUN_005c0e00` -> `FUN_0059b550`: task-force slot id-1 gets state 5 and
  every other assigned slot loses it (`FUN_0059b550.c:27-33`), after the
  captions and images are refreshed (`:13-24`).
- Modes 9-12: `FUN_005c0e70`, then `FUN_005c1040(id - 9, 1)`
  (`FUN_0059e760.c:27-30`) -> `FUN_0059cce0`: all four fighter slots are
  released and slot id-9 gets state 4 (`FUN_0059cce0.c:7`, `:22-23`).
- Other modes: no slot. Then, unpaused (`+0x8c8` 0), the mode's `+0x3c(1)`
  (`FUN_0059e760.c:33-36`).

## Base-mode entry: `FUN_005c0e60` and `FUN_005c0e70`

- `FUN_005c0e60` -> `FUN_0059b430` clears state 5 on the eight task-force
  buttons at window `+0xb8` (`FUN_005c0e60.c:6`; `FUN_0059b430.c:8-11`).
- `FUN_005c0e70` -> `FUN_0059b600` clears state 5 on the four fighter
  buttons at `+0xa8` (`FUN_005c0e70.c:6`; `FUN_0059b600.c:8-11`).
- The selected ships, targets and group belong to the mode object (`+4`,
  `+0x1c`, `+0x28`, `+0x34`, `+0x40`, `+0x54`-`+0x5c`); `FUN_0059f350`
  deletes it (`:5-9`). The base constructor's `FUN_0059e880` clears the
  controller's focused ship `+4` (`FUN_0051c9b0`), `+8` (`FUN_0059f050`) and
  the snapshot block `+0xc`-`+0xd8` (`FUN_0059ef60`) (`FUN_0059e880.c:36-43`).
  The ship and fighter-group selections are cleared; the panel is `0x15`.

## A ship pick and the strip; A0 TAC-03-C018

A left click on an own capital ship releases both strips
(`FUN_005c2060.c:130-131`), installs the base mode, then `0xe`, whose
constructor releases both again (`FUN_005a1550.c:20-21`). No slot is
pressed or released for the ship's own task force. A right click or a Ctrl
click changes the strips only through the constructor of the mode it
installs.

A0 TAC-03-C018 (Empire, save `empire-day30-yaga-alert-control`: task force 1
clicked, empty space clicked, then a left click on the Imperial Star
Destroyer, a task force 1 ship, at (29, 429)): slot 1 (60, 2)-(85, 19) is
pixel-identical to TAC-02-C006 (nothing selected) and differs from
TAC-03-C012 (task force 1 selected) in 365 of 425 pixels; slots 2-8 and the
four fighter slots are identical in all three.

## F-key follow (`FUN_005d9a70`)

- `FUN_0059b730` maps F1-F8 to `FUN_005c2990(view, 0xb, i)` when task force
  i exists and F9-F12 to `FUN_005c2990(view, 0xb, i + 8)` when fighter group
  i exists (`FUN_0059b730.c:98-130`); no other call.
- `FUN_005c2990`'s default case builds `FUN_005d9a70` (vtable
  `0x0066d478`) and keeps or builds `FUN_005d9580` (`FUN_005c2990.c:87-114`).
- The follow camera's methods (`FUN_005d9a40`, `FUN_005d9e00`,
  `FUN_005d9e40`, `FUN_005d9b10`, `FUN_005d9d80`, `FUN_005d9df0`,
  `FUN_005da000`) call no pick, command, mode or strip function. The update
  `FUN_005d9b10` reads the group's two ids through `FUN_00595d00` ->
  `FUN_0059a710` -> `FUN_005aafc0` (`FUN_005d9b10.c:18-20`), which reads the
  controller's slots and the current mode id (`FUN_005a5fb0`) and, in modes
  1-8, the focused ship (`FUN_005aafc0.c:17-26`).
- Where the camera goes is `or-q-tac-camera-e1d`'s question (no note yet);
  the default pose is in [tactical-camera-holocube.md](tactical-camera-holocube.md).

## Mode `0x11`: the Death Star superlaser

- The panel `FUN_005c4df0` at (474, 0), 166 x 25 (`FUN_005c4df0.c:5`),
  exists for a local Empire with a Death Star (`FUN_0059c1c0.c:17-27`). Its
  procedure `FUN_005c4ed0` builds the charge gauge `0xde01`
  (`FUN_005c4ed0.c:49`) and the toggle button `0xde02` at panel (0x89, 2),
  21 x 20, bitmaps 1021 normal, 1022 pressed, 1023 state 2
  (`FUN_005c4ed0.c:63`, `:73-74`).
- Pressing `0xde02` calls `FUN_005c0f60(1)` (`FUN_005c4ed0.c:89-96`) ->
  `FUN_0059cba0(1)`: the view's `+0x648` = 1, the targeting cursor
  (`FUN_005c2060.c:98-101`), and command `0xf` (`FUN_0059cba0.c:5-10`).
  Releasing it calls `FUN_005c0f60(0)` and posts nothing
  (`FUN_005c4ed0.c:100`).
- Command `0xf` with the flag set: from `0x11` it installs the base mode,
  from any other mode it runs `+0x2c` (`FUN_0059fd40.c:90-105`),
  `FUN_0059eae0`, which installs `0x11` when the station exists with charge
  `+0x6c` = 100.0 and `+0x64` above 0.0 (`FUN_0059eae0.c:18-24`;
  `DAT_0066bb44`, `DAT_0066bb48`).
- A pick in `0x11` (`FUN_005a5740`): an own ship installs the base mode
  (`:46-55`); an enemy ship is fired on (`FUN_005ba720`, `:72`), voice
  `0x111` for a local Empire (`:76`), the gauge reset (`:81`), base mode
  (`:89`). Leaving `0x11` runs `FUN_005a56c0` -> `FUN_005c0f60(0)`
  (`FUN_005a56c0.c:14`), which resets the cursor and releases the button.
- `FUN_0059b130` also maps control `0xad10` to command `0xf`; no traced
  window builder creates `0xad10`.

## Manual cross-check (pp. 143, 148)

| Claim | Status | Trace |
|---|---|---|
| p. 143: "To add a ship to an existing task force, select the task force, hold down the CTRL key and then click on the ship" | Confirmed | `FUN_005a3aa0.c:109-113`; either button; Ctrl+click on a member removes it (`:116-137`) |
| p. 143, Fig. 4.4: CTRL-select the ships, then click the blank space next to the highest number / "the next empty space" | Confirmed for the gesture; any unassigned slot works | `0x10` + `FUN_005a28a0`; `FUN_0059b450.c:18-19` tests only the clicked slot |
| p. 143: "To select an individual capital ship, left-click on that ship" | Confirmed | `FUN_005c2060.c:130-133`, `FUN_005a0d10.c:30-36` |
| p. 148: "Select multiple ships by holding down the CTRL key while left-clicking" | Confirmed | `FUN_005a1af0.c:113`, `FUN_005a24d0.c:88-103` |
| p. 148: F1-F8 follow task forces 1-8, F9-F12 fighter groups | Confirmed; the selection is unchanged | `FUN_0059b730.c:98-130` |

The manual does not mention a one-ship task force from `0xe`, the
automatic task force in the first free slot when a multi-ship selection
gets an order, a target or an escort (`FUN_005a24d0`, `FUN_005a2c70`), or
that an order to one selected ship gives it a group of its own
(`FUN_005a1a60.c:13`).

## Port

`assign_selected_to_task_force` (`crates/rebellion-render/src/tactical_view.rs`)
assigns the selected ships to an assigned slot or the next blank one, tested
by `task_force_assignment_accepts_only_an_existing_or_next_blank_slot`. The
original makes a new task force on any unassigned slot, only in modes `0xe`
and `0x10`; a click on an assigned slot selects that task force, and ships
join an existing task force only by Ctrl+click while it is selected.

## Open

- What the Observe and Take Command control (TAC-06-C005, TAC-06-C006) does
  to the controller flag: nothing posts `0x7b`, and the Battle Options code
  at 0x005daf8f posts `0x7d`. Not traced.
- The pick test `FUN_005c2d20` (which object a point hits) is not read.
- The stale pressed slot in `0x10` after Withdraw is a code reading; no A0
  frame shows it.
- A0 checks here are Empire frames only; the code has no side branch, and
  no Alliance frame of a ship pick or a blank-slot click was compared.
- The code at 0x005f1183 that posts Withdraw is not a defined function.

## Decompiles

Mode classes and ids: `FUN_0059e880.c`, `FUN_005a0c80.c`, `FUN_005a33d0.c`,
`FUN_005a34e0.c`, `FUN_005a4640.c`, `FUN_005a0e70.c`, `FUN_005a1550.c`,
`FUN_005a1110.c`, `FUN_005a2280.c`, `FUN_005a5630.c`, `FUN_005a3a10.c`,
`FUN_005a48c0.c`, `FUN_005a5600.c`, `FUN_005a5920.c`, `FUN_005a56b0.c`,
`FUN_005a5980.c`, `FUN_005a5b70.c`, `FUN_005a5b60.c`, `FUN_0048b440.c`,
`FUN_006158b0.c`.
Slots: `FUN_005a0d10.c`, `FUN_005a0e10.c`, `FUN_005a1af0.c`,
`FUN_005a1a60.c`, `FUN_005a1f30.c`, `FUN_005a2160.c`, `FUN_005a2100.c`,
`FUN_005a21e0.c`, `FUN_005a24d0.c`, `FUN_005a28a0.c`, `FUN_005a2c70.c`,
`FUN_005a3020.c`, `FUN_005a31f0.c`, `FUN_005a3250.c`, `FUN_005a3aa0.c`,
`FUN_005a40c0.c`, `FUN_005a4120.c`, `FUN_005a5010.c`, `FUN_005a5070.c`,
`FUN_005a1080.c`, `FUN_005a13a0.c`, `FUN_005a5740.c`, `FUN_005a5a30.c`,
`FUN_005a5a90.c`, `FUN_005a5b80.c`, `FUN_005a56c0.c`, `FUN_0059eae0.c`.
Controller and queues: `FUN_0059eb70.c`, `FUN_0059ed70.c`, `FUN_0059ef60.c`,
`FUN_0059f050.c`, `FUN_0051c9b0.c`, `FUN_0059f350.c`, `FUN_0059f6e0.c`,
`FUN_0059fd40.c`, `FUN_0059eec0.c`, `FUN_0059eef0.c`, `FUN_0059fce0.c`,
`FUN_0059f380.c`, `FUN_005a0820.c`, `FUN_005a0790.c`, `FUN_005a0710.c`,
`FUN_005a04f0.c`, `FUN_005a0b50.c`, `FUN_005a0ad0.c`, `FUN_0059a4d0.c`,
`FUN_0059a470.c`, `FUN_00595b70.c`, `FUN_00595b10.c`, `FUN_00596a70.c`,
`FUN_00596a90.c`, `FUN_005a8a90.c`, `FUN_005ad530.c`, `FUN_005f5e40.c`,
`FUN_005f5e90.c`, `FUN_005c7e10.c`, `FUN_005be350.c`.
Strips: `FUN_0059b450.c`, `FUN_0059b730.c`, `FUN_0059b130.c`,
`FUN_005c0e60.c`, `FUN_005c0e70.c`, `FUN_005c0e80.c`, `FUN_005c1040.c`,
`FUN_005c0e00.c`, `FUN_0059b430.c`, `FUN_0059b600.c`, `FUN_0059b550.c`,
`FUN_0059cce0.c`, `FUN_0059fc50.c`, `FUN_0059e760.c`, `FUN_005ab0e0.c`,
`FUN_0059a4f0.c`, `FUN_00595b90.c`, `FUN_0059fca0.c`, `FUN_0059b620.c`.
Groups: `FUN_0059f610.c`, `FUN_005a03f0.c`, `FUN_005a0410.c`,
`FUN_005a0c50.c`, `FUN_005ade70.c`, `FUN_005aded0.c`, `FUN_005ade50.c`,
`FUN_005ade20.c`, `FUN_005a8de0.c`, `FUN_005a8d30.c`, `FUN_005a8fc0.c`,
`FUN_005ae260.c`, `FUN_005ae2a0.c`.
3D view and camera: `FUN_005c2060.c`, `FUN_005c2990.c`, `FUN_005d9a70.c`,
`FUN_005d9a40.c`, `FUN_005d9e00.c`, `FUN_005d9e40.c`, `FUN_005d9b10.c`,
`FUN_005d9d80.c`, `FUN_005d9df0.c`, `FUN_005da000.c`, `FUN_00595d00.c`,
`FUN_0059a710.c`, `FUN_005aafc0.c`, `FUN_005a5fb0.c`.
Death Star: `FUN_0059c1c0.c`, `FUN_005c4df0.c`, `FUN_005c4ed0.c`,
`FUN_005c0f60.c`, `FUN_0059cba0.c`, `FUN_005ba720.c`.
