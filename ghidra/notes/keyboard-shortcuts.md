---
title: "Original Keyboard Shortcuts and Mouse Modifiers"
description: "Every key the 1998 REBEXE.EXE accepts, by screen: the TEXTCOMM accelerator table, the galaxy view and tactical key handlers, dialog and text-field keys, with manual page and Ghidra source per row"
category: "ghidra"
created: 2026-10-17
updated: 2026-10-17
tags: [keyboard, input, accelerators, manual, vm]
---

# Original keyboard shortcuts and mouse modifiers

A reference for driving the unmodified game in the VM (`tools/original-capture/vm-game.sh`). Each row cites the manual (printed page numbers; the key labels on pp. 64-66 are images, read from the PDF), the decompile, or a VM note. Rows marked "needs VM check" have a handler whose effect is unclear.

## Sources read

- **Manual**: `docs/reference/campaign-history/archive/star-wars-rebellion-manual.pdf`, "Strategic Game Keyboard Commands" pp. 64-65, "Tactical Game Keyboard Commands" pp. 66, tactical camera keys p. 148, plus the prose passages cited per row.
- **Accelerator table**: `TEXTCOMM.DLL` `RT_ACCELERATOR` id 11 (33 entries), the only accelerator resource in `REBEXE.EXE` and the game DLLs, read with `pefile`. `FUN_00421c70` loads it (`LoadAcceleratorsA(textcomm, 0xb)`, `FUN_00421c70.c:166`) into the application object at `+0x98`; the message loop `FUN_00413560` (`FUN_00413560.c:34`) calls `TranslateAcceleratorA` with the main window as target for every message, so the table is global. Flag bits: `0x01` VIRTKEY, `0x02` NOINVERT, `0x04` SHIFT, `0x08` CONTROL, `0x10` ALT, `0x80` last entry.
- **Command handler**: `FUN_00422ce0.c:1318-1580` (the galaxy view's `0x483` case, command ids `0xbb9`-`0xbd7`).
- **Key handlers**: `FUN_00422ce0.c:296-380` (galaxy `WM_KEYDOWN`), `FUN_0059b730.c` (tactical), per-window key slot `+0x4c` handlers (`FUN_00461590`, `FUN_00463360`, `FUN_0045fe60`, `FUN_0046a4d0`, `FUN_00438b60`, `FUN_0044f640`, `FUN_0060c810`), text fields (`FUN_00605160`, `FUN_0041efe0`), movies (`FUN_005f7240`).
- **Existing notes**: `agent-menu.md`, `agent-advice.md`, `strategic-windowing-toolkit.md`, `tactical-group-strips.md`, `move-order.md`, `movies.md`, `game-options.md`, `docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-24-game-speed-recovery.md` ("Speed keys").
- **VM notes**: `tools/original-capture/README.md`, `tools/original-capture/vm-game.sh`. VNC sends vncdotool lowercase names (`esc`, `f1`, `enter`, `end`, `bsp`); keypad keys go through QMP `sendkey alt-kp_add` (README, "Game speed").

Status values: `confirmed: code+manual`, `code only`, `manual only`, `VM-observed`, `needs VM check`.

## 1. Strategic galaxy screen: function keys

`WM_KEYDOWN` in `FUN_00422ce0.c:296`. F1-F7 (`0x70`-`0x76`) are swallowed while any keyed child window has policy bit 0 at `+0xb8` (the strategic-modal windows; `strategic-windowing-toolkit.md`, "Modality and input routing"). While Ctrl is down (`GetKeyState(0x11)`, `FUN_00422ce0.c:314`) the whole `WM_KEYDOWN` returns without acting. Tab (`0x09`) returns without acting. Any other key is handed to the base procedure `FUN_005ff530`.

| Key | Action | Context | Source | Status |
|---|---|---|---|---|
| F1 | Game Options | Galaxy, no modal window open | manual p. 64; `FUN_00422ce0.c:326` posts `WM_COMMAND` `0x133`; `game-options.md` ("Cockpit/F1 uses command 0x133"); `vm-game.sh` `load_slot` | confirmed: code+manual |
| F2 | Planetary System Finder (window type `0x14`) | Galaxy | manual p. 64; `FUN_0042a000` | confirmed: code+manual |
| F3 | Fleet/Ship Finder (type `0x15`) | Galaxy | manual p. 64; `FUN_0042a0c0` | confirmed: code+manual |
| F4 | Troop Finder (type `0x16`) | Galaxy | manual p. 64; `FUN_0042a4d0` | confirmed: code+manual |
| F5 | Personnel Finder (type `0x17`) | Galaxy | manual p. 64; `FUN_0042a180` | confirmed: code+manual |
| F6 | Message window (Display Message Index, type `0x0d`) | Galaxy | manual pp. 64, 78; `FUN_0042a240(view, 1, 0x79)`; opens only when `FUN_004fcee0() > 1` (meaning not read) | confirmed: code+manual |
| F7 | Galactic Encyclopedia (type `0x19`) | Galaxy | manual p. 64; `FUN_00429f30` | confirmed: code+manual |
| Tab | Ignored by the galaxy view | Galaxy | `FUN_00422ce0.c:320` (`case 0x9` returns) | code only |
| R | Does nothing on the galaxy view; the original has no view-reset key (port: none either) | Galaxy | no `0x52` case in the `WM_KEYDOWN` switch, which hands it to `FUN_005ff530`; no R entry in TEXTCOMM 11 (section 2) | code only |
| Escape | Does nothing on the galaxy view; a focused child window takes it first (section 4) | Galaxy | `FUN_00422ce0.c:296` has no `0x1b` case and hands other keys to `FUN_005ff530`, which has no `WM_KEYDOWN` case; no accelerator (section 2); VM, `alliance-day927-orto-orbit.sav` | VM-observed |

## 2. Strategic galaxy screen: accelerator table (TEXTCOMM id 11)

All Alt entries are `VIRTKEY` unless noted. Command ids are `0xbb9`-`0xbd7` (`3001`-`3031`), handled in `FUN_00422ce0.c:1318` (`case 0x483`). Most agent and finder commands also require the strategic view to be in state 1 (`param_1[0x30] == 1`) and the input flag `DAT_006be4e4 == 0` (inferred: an input-lock flag).

| Key | Action | Context | Source | Status |
|---|---|---|---|---|
| Alt+W | Close all windows (`0xbb9`, `FUN_00422ce0.c:1330`) | Galaxy | manual p. 64 | confirmed: code+manual |
| Alt+NumPad+ | Faster one notch (`0xbba`): from speed 1/2/3 selects item `0x22`/`0x23`/`0x24`; nothing at Fast or while paused | Galaxy | manual p. 64; game-speed-recovery "Speed keys"; README (QMP `alt-kp_add`) | confirmed: code+manual |
| Alt+NumPad- | Slower one notch (`0xbbb`): from speed 2/3/4 selects `0x21`/`0x22`/`0x23`; nothing at Very Slow or while paused | Galaxy | manual p. 64; game-speed-recovery "Speed keys"; README (QMP `alt-kp_subtract`) | confirmed: code+manual |
| Alt+= (ASCII, not VIRTKEY) | Same command as Alt+NumPad+ (`0xbba`) | Galaxy | accelerator entry flags `0x12`, key `0x3d` | code only |
| Alt+- (ASCII, not VIRTKEY) | Same command as Alt+NumPad- (`0xbbb`) | Galaxy | accelerator entry flags `0x12`, key `0x2d` | code only |
| Alt+P | Pause (`0xbbc`): selects item `0x20` when the speed field is not 0; does not resume | Galaxy | manual p. 64; game-speed-recovery; README (QMP `alt-p`); `0xbbc` at `FUN_00422ce0.c:1365` | confirmed: code+manual |
| Alt+Y | Compose chat message (`0xbbd`). The handler calls `FUN_0042a240(view, 0x68, 0x79)`, the same opener as F6 (window type `0x0d`, Message Index), when `FUN_00401080() != 1` | Galaxy | manual p. 65 ("head-to-head" only); `FUN_00422ce0.c:1370` | needs VM check |
| Alt+I | View Index: sends `0x483`/`0xbbe` to the Message Index (type `0x0d`), else the Encyclopedia (type `0x19`) | Message Index or Encyclopedia open | manual p. 65; `FUN_00422ce0.c:1375` | confirmed: code+manual |
| Alt+S | Status for the selected item (`0xbbf`, order `0x103`) | Galaxy, a window selected | manual p. 65; `FUN_00422ce0.c:1387` | confirmed: code+manual |
| Alt+M | Mission for the selected character(s) (`0xbc0`, order `0x240`; id high byte `0x30`-`0x3f`) | Galaxy, a character selected | manual p. 65; `FUN_00422ce0.c:1408` | confirmed: code+manual |
| Alt+B | Build Ships (`0xbc1` → `0x110`) | Galaxy | manual p. 65; `agent-menu.md` table | confirmed: code+manual |
| Alt+T | Build Troops (`0xbc2` → `0x111`) | Galaxy | manual p. 65; `agent-menu.md` | confirmed: code+manual |
| Alt+F | Build Facilities (`0xbc3` → `0x112`) | Galaxy | manual p. 65; `agent-menu.md` | confirmed: code+manual |
| Alt+O | Galaxy Overview (`0xbc4` → `0x113`) | Galaxy | manual p. 65; `agent-menu.md` | confirmed: code+manual |
| Alt+G | Manage Garrisons toggle (`0xbc5` → `0x115`) | Galaxy | manual p. 65; `agent-menu.md` | confirmed: code+manual |
| Alt+U | Manage Production toggle (`0xbc6` → `0x116`) | Galaxy | manual p. 65; `agent-menu.md` | confirmed: code+manual |
| Alt+D | Command `0xbc7`; `FUN_00422ce0` has no `case 0xbc7`, so no handler was read | Galaxy | accelerator entry flags `0x13`, key `0x44`; not in the manual | needs VM check |
| Alt+V | Translate Counterpart toggle (`0xbc8` → `0x11b`) | Galaxy | manual p. 65; `agent-menu.md` | confirmed: code+manual |
| Alt+A | Agent Advice toggle (`0xbc9` → `0x11e`) | Galaxy | manual p. 65; `agent-menu.md` | confirmed: code+manual |
| Alt+H | Game Objectives (`0xbd7` → `0x114`) | Galaxy | manual p. 65; `agent-menu.md` | confirmed: code+manual |
| Alt+1 | GID: Popular Support (`0xbca`, `FUN_00425d00(view, 0x11)`) | Galaxy | manual p. 65; `gid-legend.md` | confirmed: code+manual |
| Alt+2 | GID: Uprisings (`0x12`) | Galaxy | manual p. 65; `gid-legend.md` | confirmed: code+manual |
| Alt+3 | GID: Idle Fleets (`0x21`) | Galaxy | manual p. 66; `gid-legend.md` | confirmed: code+manual |
| Alt+4 | GID: Fleets En Route (`0x22`) | Galaxy | manual p. 66; `gid-legend.md` | confirmed: code+manual |
| Alt+5 | GID: Idle Personnel (`0x43`) | Galaxy | manual p. 66; `gid-legend.md` | confirmed: code+manual |
| Alt+6 | GID: Active Personnel (`0x44`) | Galaxy | manual p. 66; `gid-legend.md` | confirmed: code+manual |
| Alt+7 | GID: Idle Shipyards (`0x65`) | Galaxy | manual p. 66; `gid-legend.md` | confirmed: code+manual |
| Alt+8 | GID: Idle Training Facilities (`0x66`) | Galaxy | manual p. 66; `gid-legend.md` | confirmed: code+manual |
| Alt+9 | GID: Idle Construction Yards (`0x67`) | Galaxy | manual p. 66; `gid-legend.md` | confirmed: code+manual |
| Ctrl+Tab | Cycle forward through open windows (`0xbd3`, `FUN_00422ce0.c:1519`, posts `0x467` to restore) | Galaxy | manual pp. 27, 64 | confirmed: code+manual |
| Ctrl+Shift+Tab | Cycle backward through open windows (`0xbd4`, same case) | Galaxy | manual p. 27; accelerator entry flags `0x8f` | confirmed: code+manual |
| Ctrl+X | `FUN_00401040(0xe)`: requests application state `0xe` (not resolved in `FUN_00414330`) | Galaxy | accelerator entry flags `0x0b`, key `0x58`; `FUN_00422ce0.c:1560`; not in the manual | needs VM check |
| Ctrl+S | Command `0xbd6`; no `case 0xbd6` in `FUN_00422ce0`, and no other `0xbd6` in the decompile | Galaxy | accelerator entry flags `0x0b`, key `0x53`; not in the manual | needs VM check |
| Alt+F4 | Exit Game: "Are you sure you want to quit?", whose checkmark closes the game to the desktop | Galaxy | manual p. 65; VM, `alliance-day927-orto-orbit.sav`. Not in the accelerator table, and `FUN_004138a0.c:107` returns 0 for `WM_SYSKEYDOWN`, so another window procedure handles it (not read) | VM-observed |

## 3. Game Options, Saved Games, Pause alert

| Key | Action | Context | Source | Status |
|---|---|---|---|---|
| F1 | Open Game Options (also the route to Saved Games); after Alt+W has closed every window it opens even when a window was open | Galaxy | `vm-game.sh` `load_slot`, `save_slot`; README line 494; VM, `alliance-day927-orto-orbit.sav` | VM-observed |
| Escape | Leaves the save-name edit; the galaxy icon (228,490) then closes the screen | Game Options, a name field focused | `vm-game.sh` `save_slot` comment (2026-10-16) | VM-observed |
| Enter | Commits the typed save name | Saved Games name field | `vm-game.sh` `save_slot`; README line 377 | VM-observed |
| Home, End | End moves the caret to line end; the script presses `end` then 40 backspaces to clear a name | Saved Games name field | `vm-game.sh` `save_slot`; field handler `FUN_00605160` (section 7) | VM-observed |
| Enter | Same as the checkmark: resumes play | "Resume Game Play?" alert after Alt+P | `FUN_00417180`, `FUN_004171a0` (game-speed-recovery, "Pause and the alert") | code only |
| Escape | Does nothing: the alert stays open and the game stays paused (no key handler in `FUN_00417020`/`FUN_00416de0`); the checkmark resumes | "Resume Game Play?" alert | manual p. 64 (Pause); VM, `alliance-day927-orto-orbit.sav` | VM-observed |
| Enter | The checkmark: closes the alert; Escape does nothing | "Selected Game Saved Successfully" / "Selected Game Failed To Save" alert (0x474, 0x475) | `FUN_00408240` → `FUN_00417020`, the "Resume Game Play?" class (`game-options.md`) | code only |
| Enter, Escape | Yes, No | Game Options confirmation (overwrite, load, restart, exit) | `FUN_00416f30` (vtable `0x00658950` slot `+0x4c`) → `FUN_004011a0` posts `0x46a` or `0x46b` (`game-options.md`) | code only |

Escape in Game Options: the window procedure `FUN_00406b80` (vtable `0x00658600`, slot `+0x14`) has no `WM_KEYDOWN` case, and neither have its default path `FUN_005ff530` nor that one's `FUN_006007b0`, which ends in `DefWindowProcA`; Escape reaches only a focused name edit, as the VM note above records (`game-options.md`).

## 4. Dialogs, finders and windows

Per-window key handlers sit in the window vtable slot `+0x4c`, reached from the base procedure `FUN_00606650` (`WM_KEYDOWN` at `FUN_00606650.c:96`). Escape in these handlers calls slot `+0x30`, the asynchronous keyed close (`strategic-windowing-toolkit.md`, "Keyboard ownership and bubbling").

| Key | Action | Context | Source | Status |
|---|---|---|---|---|
| Escape | Close the window | System, Fleet, Troop, Personnel Finders | `FUN_00461590.c:16`, `FUN_00463360.c:16`; `fleet-finder.md` line 124 | code only |
| Left / Right | Walk the finder's tabs (visible neighbour window of the same registry) | Finders | `FUN_00461590.c:19`, `FUN_00463360.c:19` | code only |
| Up | Focus the name box | Finders | `FUN_00463360.c:57`; `fleet-finder.md` line 124 | code only |
| Down | Move into the result list | Finders | `FUN_00463360.c:60` | code only |
| Escape, Left, Right, Up, Down | Same shape as the finders | Message Index (inferred from `FUN_00466160` lying beside its constructor `FUN_00466350`) | `FUN_00466160.c:16-60` | needs VM check |
| Escape | Close the window | Message Index | manual pp. 23, 29 (the Display Message Index closes with ESC); VM, `alliance-day927-orto-orbit.sav` | VM-observed |
| Escape | Close | Encyclopedia | `FUN_0045fe60.c:16`; `strategic-windowing-toolkit.md` line 324 (runtime) | code only |
| PgUp, PgDn, Down | Focus the list and forward the key | Encyclopedia | `FUN_0045fe60.c:19-28` | code only |
| Left / Right | In topic mode, commands `0x83` / `0x84` (previous / next topic) | Encyclopedia | `FUN_0045fe60.c:28`; `encyclopedia-window.md` line 118 | code only |
| Alt+I | Back to the index | Message or Encyclopedia in topic mode | manual p. 65 | confirmed: code+manual |
| Typed text | Prefix search on the title field | Encyclopedia | `strategic-windowing-toolkit.md` line 325 (runtime: `tallon` selects Talon Karrde) | code only |
| Enter | Confirm and close | Build Selection | `FUN_00438b60.c:6`; `manufacturing-build-selection.md` line 247 | code only |
| Escape | Close | Build Selection | `FUN_00438b60.c:10` | code only |
| NumPad+ / NumPad- | Quantity up / down (clamped 1-255) | Build Selection | `FUN_00438b60.c:13-17`; `FUN_00438c30` | code only |
| Enter | Accepts when the dialog is in mode 2 and a list target exists (activates the highlighted button) | Mission creation (type `0x0c`) | `FUN_0046a4d0.c:11` | needs VM check |
| Escape | Closes (mode 1) or sends command `0x65` (mode 2, the close button) | Mission creation | `FUN_0046a4d0.c:41` | code only |
| Left / Right, Delete | Walk windows; Delete key `0x2e` handled | Mission creation | `FUN_0046a4d0.c:48, 86` | needs VM check |
| Enter | Confirm (command `0x14`) | Move confirmation (type `0x10`) | `FUN_0044f640.c`; `move-order.md` lines 330, 410 | code only |
| Escape | Cancel (command `0x15`) | Move confirmation | `FUN_0044f640.c`; `move-order.md` line 330 | code only |
| Escape | Does nothing: the targeting cursor stays, as does a right-click; a left click on empty space ends targeting with no order | Galaxy, after choosing Move | manual p. 121 says ESC cancels; the galaxy `WM_KEYDOWN` has no `VK_ESCAPE` case (`move-order.md` lines 370-377); VM, `alliance-day927-orto-orbit.sav` | VM-observed |
| Enter, Escape | Select the highlighted row; close the menu (in a submenu Escape hides the submenu only) | Object popup menus | `FUN_004aab50.c:127-150`; `object-popup-menu.md` lines 153-154 | code only |
| Escape | Never closes the menu; goes where the keyboard focus is. The first GID press after a load builds the menu, whose `FUN_004511e0` takes the focus: its Escape posts 0x405 for its key 0x140 and the view's 0x467 shows and focuses the top child window. A focused detail window closes, shown or hidden, and the next is shown; a sector window does nothing | GID menu open | `FUN_004aab50.c:127-141`; `gid-legend.md`, "Escape and the keyboard focus"; Wine, empire-fleet-to-yaga-minor days 38-45 and alliance-day54-advice day 61 | Wine-observed |
| Up / Down, Enter, Escape | Open the bar menu, move the highlight, choose, close | Menu-bar window (`FUN_0060c210` → `FUN_0060c810`, class inferred) | `FUN_0060c810.c` | needs VM check |
| Escape | Close the focused one; the view then shows and focuses the top remaining child (0x405 → 0x467) | System, Defenses, Fleet and Mission detail windows | `FUN_004ac3a0`; `FUN_00422ce0` case 0x405; `fleet-window.md` line 271 | code only |
| Escape | Close (clears the modal boundary; the next F-key then works) | Status, Build Selection, Create Mission, Move Confirmation, Encyclopedia | `strategic-windowing-toolkit.md` lines 324-331 (runtime, both factions) | code only |
| Escape | Skip the briefing; the Message Index opens when it ends | Opening briefing | manual pp. 22, 29; README line 490 | confirmed: code+manual |
| Escape | Closes the battle result box; the box ignores an Escape sent before it has drawn | Battle result box after auto-resolve | `vm-game.sh` `resolve` comment | VM-observed |

The Agent menu items `0x20`-`0x29` in `FUN_00486fb0.c` are menu item ids, not virtual-key codes (inferred from `game-speed-recovery.md`, "Menu selection path").

## 5. Tactical battle screen

`FUN_0059b730` (key down: `WM_KEYDOWN` and `WM_SYSKEYDOWN`, key up in the second branch). Numpad digits are `VK_NUMPAD1`-`9` (`0x61`-`0x69`) with NumLock on; the same actions sit on the navigation keys.

| Key | Action | Context | Source | Status |
|---|---|---|---|---|
| 1-8 (`0x31`-`0x38`) | Select Task Force 1-8: releases every task-force and fighter slot, then presses slot `0xad02 + i` | Tactical | manual p. 66; `FUN_0059b730.c` cases `0x31`-`0x38`; `tactical-group-strips.md` line 37 | confirmed: code+manual |
| Space | Attack nearest target of the selected unit's type (`FUN_00595b10(0x20)`) | Tactical, a unit selected | manual pp. 66, 144; `FUN_0059b730.c` case `0x20` | confirmed: code+manual |
| Tab, Shift+Tab | Switch between ships in a task force; the strip forwards the key to child `+0x10c` on the first press only (`lParam` bit 30 clear) | Tactical | manual p. 66; `FUN_0059b730.c` case `9`. The Shift test is not in this function (not read) | needs VM check |
| F1-F8 (`0x70`-`0x77`) | Camera follows Task Force 1-8 (`FUN_005c2990(.., 0xb, i)`), only if that slot exists | Tactical | manual pp. 66, 148; `FUN_0059b730.c`; `tactical-group-strips.md` line 37 | confirmed: code+manual |
| F9, F10, F11, F12 (`0x78`-`0x7b`) | Follow Red, Blue, Green, Gold fighter group (`FUN_005c2990(.., 0xb, 8 + i)`) | Tactical | manual pp. 66, 148; `FUN_0059b730.c` | confirmed: code+manual |
| Enter, NumPad . | Center camera on the selected target (`0xad21`) | Tactical | manual pp. 66, 148 (Enter); `FUN_0059b730.c` cases `0xd`, `0x6e` | confirmed: code+manual |
| Home, NumPad7 | Memorize camera position (`0xad19`) | Tactical | manual p. 66 (Home); `FUN_0059b730.c` cases `0x24`, `0x67` | confirmed: code+manual |
| End, NumPad1 | Return camera to the memorized position (`0xad1a`) | Tactical | manual p. 66 (End); cases `0x23`, `0x61` | confirmed: code+manual |
| PgUp, NumPad9, NumPad+ | Camera zoom in (`0xad1b`) | Tactical | manual pp. 66, 148 (`+`); cases `0x21`, `0x69`, `0x6b` | confirmed: code+manual |
| PgDn, NumPad3, NumPad- | Camera zoom out (`0xad1c`) | Tactical | manual pp. 66, 148 (`-`); cases `0x22`, `0x63`, `0x6d` | confirmed: code+manual |
| Left, NumPad4 | Rotate camera left (`0xad1d`) | Tactical | manual pp. 66, 148; cases `0x25`, `0x64` | confirmed: code+manual |
| Right, NumPad6 | Rotate camera right (`0xad1e`) | Tactical | manual p. 66; cases `0x27`, `0x66` | confirmed: code+manual |
| Up, NumPad8 | Tilt camera up (`0xad1f`) | Tactical | manual p. 66; cases `0x26`, `0x68` | confirmed: code+manual |
| Down, NumPad2 | Tilt camera down (`0xad20`) | Tactical | manual p. 66; cases `0x28`, `0x62` | confirmed: code+manual |
| Alt+P, Alt+NumPad+ | No visible effect: no "Resume Game Play?" alert and no change on the battlefield. The global table still runs (`FUN_00413560.c:34`); its commands go to the strategic view (`FUN_00422ce0`) | Tactical (battle paused) | VM, `alliance-day1169-deathstar-trench-run.sav`, day 1173 | VM-observed |
| Escape | Does nothing | Tactical | `FUN_0059b730.c` has no `0x1b` case; VM, `alliance-day1169-deathstar-trench-run.sav` (Battle at Coruscant, day 1173, paused) | VM-observed |

Key-up posts the matching release (`0xad19`-`0xad21` with arg 0), so a held key repeats the camera move.

## 6. Movies and cutscenes

| Key | Action | Context | Source | Status |
|---|---|---|---|---|
| Escape or left button (held) | Skip the movie: `GetAsyncKeyState(0x1b)` or button 1 sends `0x404` to the owner each frame; the owner closes the window | Any Smacker movie | `FUN_005f7240.c:16-21`; `movies.md` lines 133, 216 | code only |
| Escape | Skip animation scenes or the agent introduction | Briefings, cutscenes | manual pp. 22, 29, 64 | confirmed: code+manual |
| Any other key | Closes the movie window and posts `0x405` | Movies other than the trench movies (ids `0x70`, `0x71`) | `movies.md` line 135 | code only |
| Any key except Escape | Ignored | Trench movies (ids `0x70`, `0x71`) | `movies.md` line 135 | code only |
| Escape (`WM_KEYDOWN`) | Posts message 0 to the application window and calls the callback with message 0; does not itself close | Movie window | `movies.md` line 136 | code only |

## 7. Main menu (shuttle cockpit)

| Key | Action | Context | Source | Status |
|---|---|---|---|---|
| Escape | Steps through the opening briefing | After New Game | README line 490 | VM-observed |
| (none) | No `WM_KEYDOWN` handler was found in the cockpit procedure; the buttons are mouse-driven (manual p. 67, Fig. 3.3). The main window returns 0 for `WM_SYSKEYDOWN` (`FUN_004138a0.c:107`) | Cockpit | decompile search for `0x100`/`0x104` | needs VM check |

## 8. Text entry

Single-line field `FUN_00605160` (CoolStringField; `WM_KEYDOWN` at `FUN_00605160.c:400`, `WM_CHAR` at `:264`). Characters are accepted when `FUN_0061a050` approves them. Keys not listed are forwarded to the owner window's key handler.

| Key | Action | Context | Source | Status |
|---|---|---|---|---|
| Printable characters | Insert (overwrite when Insert mode is on) | Name boxes of the finders, Encyclopedia title, Saved Games names, rename fields | `FUN_00605160.c:264-300`; `vm-game.sh` (names: lowercase, digits, space, hyphen because VNC drops Shift) | VM-observed |
| Backspace | Delete the character before the caret or the selection | Single-line field | `FUN_00605160.c:417` | code only |
| Delete | Delete the character at the caret or the selection | Single-line field | `FUN_00605160.c:573` | code only |
| Left / Right | Move the caret; with Shift extend the selection (`GetKeyState(0x10)`) | Single-line field | `FUN_00605160.c:490, 532` | code only |
| Home / End | Caret to start / end; with Shift extend the selection | Single-line field | `FUN_00605160.c:454, 472` | code only |
| Insert | Toggle overwrite mode (flag `0x20000000`) | Single-line field | `FUN_00605160.c:569` | code only |
| Enter | Posts `0x407` to the owner (commit); `FUN_005fba70` rewrites a field's Enter into the same message | Single-line field | `FUN_00605160.c:449`; `FUN_005fba70.c:9`; `strategic-windowing-toolkit.md` "Keyboard ownership" | code only |
| Enter | Inserts a line break; forwards to the owner when the field is not editing | Multi-line text field (message display) | `FUN_0041efe0.c:213-290`; `message-display.md` | code only |
| Backspace, Delete, Left, Right, Up, Down, Insert | Edit and move the caret; Insert toggles overwrite | Multi-line text field | `FUN_0041efe0.c` cases `8`, `0x2e`, `0x25`-`0x28`, `0x2d` | code only |
| Tab | Ignored by `WM_CHAR` | Multi-line text field | `FUN_0041efe0.c:137` | code only |
| Enter | Sends the chat message | Compose Chat Message (head-to-head) | manual p. 162 ("Press Enter to send it") | manual only |

## 9. Mouse modifiers

| Gesture | Action | Context | Source | Status |
|---|---|---|---|---|
| Double-click | Opens the window for the item (manufacturing from a planet, units from the star under it, Fleet window from a fleet icon) | Sector and System windows | manual p. 62; `FUN_004593e0` case `0x203`; `fleet-window.md` line 30; README line 497 | confirmed: code+manual |
| Double-click | Opens the finder row's target (Fleet window, System window) | Finders | manual pp. 124, 132; `fleet-finder.md`; list notification `0x309` | confirmed: code+manual |
| Double-click | Displays the message | Message Index list | `message-index-rows.md` lines 98-100 | code only |
| Right-click (`WM_RBUTTONUP`) | Object menu | Any object | manual p. 62; `FUN_00422ce0` (`galaxy-view-input.md` line 21); `object-popup-menu.md` | confirmed: code+manual |
| Right-click on the day readout | Game speed menu (Pause, Very Slow ... Fast) | Galaxy | manual p. 23; README "Game speed" (day readout menu) | VM-observed |
| Ctrl held at drop | Confirmed Move (`0x202`) instead of Move (`0x201`) for window types 1, 4 and 10; type 9 issues `0x214` | Dragging a fleet or list entry onto a system | `FUN_00422ce0.c:577` (`GetAsyncKeyState(0x11)`); `move-order.md` line 338; `object-popup-menu.md` line 217 | code only |
| Ctrl+click | Toggle an overlay's selection (left release) | Sector window quadrant icons | `sector-quadrants.md` line 67; `sector-icon-menus.md` line 28 (`MK_CONTROL`) | code only |
| Ctrl+click | Select several ships to create a fleet from | Fleet window | manual p. 120 | manual only |
| Ctrl+click | Toggle a message in the selection | Message Index | manual p. 79; `message-index-rows.md` line 97 | confirmed: code+manual |
| Shift+click | Select the range between two messages | Message Index | manual p. 79; `message-index-rows.md` line 97 | confirmed: code+manual |
| Shift held during a list click | Pass-through click (not further read) | Galaxy window lists | `object-popup-menu.md` line 273 | needs VM check |
| Ctrl+click on ships | Add a ship to a task force, or select several ships | Tactical | manual pp. 143, 148 | manual only |
| Ctrl+click on a task-force blank slot | Create a new task force from the CTRL-selected ships | Tactical strip | manual p. 143 | manual only |
| Right-click on a navigation point | Order the selected unit to go there | Tactical | manual p. 144 | manual only |
| Ctrl+right-click on points | Build a path point by point | Tactical | manual p. 144 | manual only |
| Ctrl+click on a path point | Remove that point; Ctrl+click on a point after right-clicking it clears all points | Tactical | manual p. 144 | manual only |
| Right-click on an enemy ship | Assign a target | Tactical | manual p. 144 | manual only |
| Ctrl+click targets | Add or cancel targets | Tactical | manual p. 145 | manual only |

The tactical mouse procedure was not searched for `MK_CONTROL` (wParam bit `0x8`) or `MK_SHIFT` (bit `0x4`); those rows rest on the manual until a VM check or a read of the tactical window procedure settles them.

## 10. Manual and code disagreements

- **In the manual, not in the game**: Escape cancelling Move targeting (no `VK_ESCAPE` case in the galaxy view; the VM keeps the cursor).
- **In the manual, not handled by the code read**: Shift+Tab in the tactical strip (the Shift test is not in `FUN_0059b730`).
- **In the code, not in the manual**: Alt+= and Alt+- (ASCII), Alt+D (no handler), Ctrl+X (state `0xe`), Ctrl+S (no handler), NumPad `.` and the navigation-key twins of the camera keys (PgUp/PgDn zoom, Home/End memory, NumPad digits), Escape and the arrow keys in finders, Enter and keypad `+`/`-` in Build Selection, the text-field editing keys.
- **Disagreement in meaning**: the manual calls Alt+Y "Compose Chat Message", while the handler opens the Message Index opener `FUN_0042a240`.

## 11. Verification checklist for the VM

Screens to test, one key each. Send keypad keys through QMP (`vm-qmp.py hmp "sendkey alt-kp_add"`); letters and F-keys through `vm-game.sh key`.

1. **Alt+Y**, galaxy view in a single-player game: does a Message Index window open, or nothing?
2. **Alt+D**, galaxy view: any visible effect?
3. **Ctrl+X**, galaxy view: does the game leave to another screen or quit? Save first.
4. **Ctrl+S**, galaxy view: any dialog?
5. **Tab** and **Shift+Tab**, tactical battle with a task force of two or more ships: does the selection step to the next or previous ship?
6. **Enter**, Mission creation window (Set Mission box) with a target chosen: does it accept?
7. **Ctrl+Tab** with two or more windows open on the galaxy view: confirm the order of cycling.
8. **Shift+click** on a fleet list while dragging (Fleet window): does the pass-through click do anything?
9. Menu bar (Game Menu), Up and Down arrows: does the bar menu open and move the highlight?
10. **Ctrl+click**, tactical: ship selection, task-force creation, path points (each its own check).
