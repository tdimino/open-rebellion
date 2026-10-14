---
title: "Tactical Fighter-Group Panel: Labels, Count Line and Art"
description: "What the selected fighter-group panel (id 0x17, FUN_005e1170, refresh FUN_005e16a0) draws: group name, order, tactics and 'Fighters: N a/b' labels in font 8, the 102x107 fighter art by side, and the Missions and Maneuvers buttons"
category: "ghidra"
tags: [tactical, panel, fighters, labels, names]
---

# Tactical fighter-group panel

Answers task-graph node `or-q-tac-fighter-panel-baj` (delegated Ghidra 12.1.4 pass from a fresh import, spot-checked against this project's decompiles: vtable, buttons, `FUN_00601b80`). All coordinates are in the original 640x480 canvas; "local" means relative to the panel window, whose container sits at screen (482, 28) (`FUN_0059cac0`: `FUN_005c5520(..., 0x1e2, 0x1c, 0x95, 0xec, ...)`, so 149 x 236).

## Question

In the original game's tactical screen, what does the selected fighter-group panel show, where, and in what font and colour? Both factions.

## Answer

**Correction to earlier notes.** `FUN_005e7030` / `FUN_005e7800` are not the fighter-group panel. They build the panel with id `0x16`, a ship-style panel: background bitmap 1302, two Hull/Shield meters, five subsystem bitmaps, a list control and "Task Force #" labels. Its refresh `FUN_005e7b10` reads a unit at `+0xd4`, not a fighter group. The fighter-group panel is a third class, panel id `0x17`, built by `FUN_005e1170` (vtable `0x0066d670`) and refreshed by **`FUN_005e16a0`**. Its labels are made by `FUN_005e1fd0`. That is the panel whose strings match the A0 frames.

How the panel is chosen: `FUN_005c5800` creates seven panels; the three ship/fighter ones set their id in their window procedure (`0x16` at `FUN_005e6d30`, `0x17` at `FUN_005e1320`, `0x18` at `FUN_005e3e10`). A selected fighter group goes through `FUN_005a5270`, whose guard is `FUN_005ad8d0` (a fighter-group lookup, not the ship lookup `FUN_005ad900`), then `FUN_005c0f00` → `FUN_0059b710(0x17, group)` → `FUN_005c5a50` → panel vtable slot `+0x3c` = `FUN_005e16a0(group)`.

What the panel shows (panel-local coordinates; add (482, 28) for screen):

| # | Item | Local box (x, y, w, h) | Text / content |
|---|---|---|---|
| 1 | Group name label (`+0xa4`) | (26, 9, 97, 9) | "Red Group", "Blue Group", "Green Group" or "Gold Group" |
| 2 | Order label (`+0xa8`) | (20, 20, 102, 9) | order name, plus a target name for Escort and Attacking |
| 3 | Tactics label (`+0xac`) | (20, 31, 102, 9) | "Tactics:" + " " + "Stand Off" or "Surround" |
| 4 | Count label (`+0xb0`) | moved at refresh to (23, 146); box 102 x 9 | `<type label> N a/b`, e.g. "Fighters: 6 72/72" |
| 5 | Fighter art | blit at (23, 59), 102 x 107 | TACTICAL.DLL bitmap 2030-2033 (Alliance) or 2034-2037 (Empire) |
| 6 | Panel background | whole panel, 149 x 236 | TACTICAL.DLL bitmap 1307 |
| 7 | Buttons | Missions (12, 210, 58, 22) id `0xdd02`, bitmaps 1107/1108; Maneuvers and Tactics (80, 210, 58, 22) id `0xdd01`, bitmaps 1105/1106 | `FUN_005e1550` |

There are no meters: the fighter panel has no `FUN_0060e150` meter controls and its refresh never calls `FUN_0060e440`. Labels `+0xb4`, `+0xb8`, `+0xbc` are created (off-screen at (-100, -100)) but the refresh never gives them text.

**Font and colour, all seven labels:** `FUN_00601c60(label, 8)`; font index 8 is Arial, `lfHeight` 12, `lfWeight` 400, `lfQuality` 2 (`FUN_0060eed0` case 8; face name "Arial" per `galaxy-view-framing.md` section 5). Colour `FUN_00601c90(label, 0x02ffffff, 0)`: white, high byte 2 (palette-relative RGB). Painted with `SetBkMode(TRANSPARENT)`, `SetTextColor`, `DrawTextA` in `FUN_00601ce0`.

Everything in the table is set by `FUN_005e16a0`. Where it branches by side (items 4 and 5), the test is `FUN_00595c40()` = the local player's side (`FUN_005aaf90()+0x9ec`): 0 takes the Alliance branch, nonzero the Empire branch.

### Per-label detail

**1. Group name.** The key id is `0xdd88 + FUN_005c7150(group)` for index 0 to 3 (switch at `0x005e1709`); `FUN_005c7150` returns `group+0xbc`. Strings (TEXTTACT.DLL): `0xdd88` "Red Group", `0xdd89` "Blue Group", `0xdd8a` "Green Group", `0xdd8b` "Gold Group". It is set through `FUN_00601de0` (set text from key, invalidate, measure). `+0xbc` defaults to 4 (`FUN_005c62b0`), which matches no case, leaving id 0.

Where the index comes from: `FUN_005adfa0` builds a group and calls `FUN_005c7140(index)` (writes `+0xbc`). Its four call sites in `FUN_005ab650` (`0x005abe22`, `0x005abf2c`, `0x005ac037`, `0x005ac141`) run in two blocks, one per side, selected by the squadron owner field `+0x168` (0 Alliance, 1 Empire). In each block, for type slot i = 0..3 with any squadrons of that type (type `0x1d + i` Alliance, `0x21 + i` Empire), one group is made with colour index i. So Red = first type (X-wing / TIE Fighter), Blue = second (Y-wing / TIE Interceptor), Green = third (A-wing / TIE Bomber), Gold = fourth (B-wing / TIE Defender). For a colour slot with no squadrons, the type with the most squadrons (at least 8) is split in half into a new group with that colour index (`0x005abe5f`-`0x005abf2c`). `FUN_005ae330` is a variant with the same signature shape (callers `FUN_005ae460`).

**2. Order line.** Switch on `group+0x68` (`FUN_00596af0`-style order field, read directly in `FUN_005e16a0`):

| `+0x68` | key | text |
|---|---|---|
| 1 | `0xdde1` | Escort |
| 2 | `0xdde2` | Recover |
| 3 | `0xdde3` | Withdraw |
| 4 | `0xddeb` | Attack Fighters |
| 5 | `0xdde5` | Attack Capital Ships |
| 6 | `0xdde6` | Attack Death Star |
| 7 / 8 / 9 / 10 | `0xdde7` `0xdde8` `0xdde9` `0xddea` | Left Hook / Right Hook / Hammer / Anvil |
| other, including 0 | `0xdd8f` | No Orders |

Then: if `(group+0x80)->vslot 0xc()` is nonzero the key becomes `0xdd8e` "Navigating" and no target is appended. Otherwise, if `(group+0x98)->vslot 0x10()` returns a target: with order 1 the text is "Escort" + " " + target name; with any other order the key becomes `0xdd8d` "Attacking" + " " + target name. The target name is `FUN_005b1610` (CString at target `+0x144`). The text is assembled in an ostream (`FUN_00616110` string append, `lpString_006a7f40` = a single space) and handed to `FUN_00601e60`. `FUN_00617480` (append integer) is not used in this label. For "No Orders", the A0 string, no target is read because `(group+0x98)->vslot 0x10()` is null.

**3. Tactics line.** `"Tactics:"` is the string at `DAT_006bd32c` (key id `0xddda`, written by the static initialiser at `0x005e3c06`), then a space, then key `0xdddc - (tactic != 1)` where `tactic = FUN_00596b60(group)` = `group+0x38`. So tactic 1 = "Surround" (`0xdddc`) and anything else, including the usual 2, = "Stand Off" (`0xdddb`). Result: "Tactics: Stand Off".

**4. Count line.** Moved at every refresh with `FUN_00601b30(label, 0x17, 0x92)` (`0x005e1e29`-`0x005e1e30`), so it sits at local (23, 146), screen (505, 174), which is inside the lower part of the 102 x 107 art (art spans local y 59 to 166). The text is built from, in order:

1. key string `"Fighters:"`-style label (below),
2. a space (`lpString_006a7f40`),
3. N = `group->vslot 0x3c(type)` for the chosen type (hyp: count of squadrons of that type in the group),
4. a space,
5. a = `FUN_005c6530(group)`: sum over every squadron in the group's list at `group+0x1c` of `FUN_005ba330` = `max(1, trunc(squadron+0x3cc / 8.0))`,
6. `"/"` (the string at `0x006aa5cc`, byte `0x2f`),
7. b = `FUN_005c6570(group)`: the same sum with `FUN_005ba380` = `max(1, trunc(squadron+0x3c8 / 8.0))`.

`8.0` is the float at `0x0066c6b8`. `+0x3cc` is the numerator and `+0x3c8` the denominator of the hull ratio used by `FUN_005b1640`/`FUN_005b1650`, so (hyp) a = living fighters and b = fighter capacity, one fighter per 8 hull points. 72 / 72 is then six squadrons at 12 fighters each. a and b cover all of the group's squadrons, while N and the label cover only the first non-empty type. `FUN_00617480` (append integer) is used three times for N, a and b; `lpString_006a7f40` appears twice.

The chosen type is the first with `group->vslot 0x3c(type) != 0`, tested in this order (`0x005e1b__`-`0x005e1d__`):

| Side branch | type tested | label key | label text | art bitmap |
|---|---|---|---|---|
| Alliance (`FUN_00595c40() == 0`) | `0x1d` | `0xddc5` | "X-wings:" | 2030 (`0x7ee`) |
| | `0x1e` | `0xddc6` | "Y-wings:" | 2031 (`0x7ef`) |
| | `0x1f` | `0xddc7` | "A-wings:" | 2032 (`0x7f0`) |
| | `0x20` | `0xddc8` | "B-wings:" | 2033 (`0x7f1`) |
| Empire (nonzero) | `0x21` | `0xddc9` | "Fighters:" | 2034 (`0x7f2`) |
| | `0x22` | `0xddca` | "Intrcpts:" | 2035 (`0x7f3`) |
| | `0x23` | `0xddcb` | "Bombers:" | 2036 (`0x7f4`) |
| | `0x24` | `0xddcc` | "Defenders:" | 2037 (`0x7f5`) |

If none of the four types is present the count label and art are not touched (the `unaff_SI != 0` guard at `0x005e1e1d`).

**5. Fighter art.** `FUN_005fbd20(module = FUN_006037f0(6), id, 2)` loads the bitmap; `FUN_005fcc30` copies it to the panel backing store at dest (23, 59), source (0, 0), 102 x 107 (`0x005e1dd4`-`0x005e1de3`). All of 1307 and 2030-2037 live in TACTICAL.DLL (read from the resource table; sizes 149 x 236 and 102 x 107, 8 bpp). Bitmap 2030 is an X-wing on a green grid; 2034 is a TIE fighter. `FUN_005e2320` (at `WM_CREATE`) copies the same 102 x 107 backing region into a save buffer at `+0xa0`.

**6. Draw order.** `WM_PAINT` (`FUN_005e1320`): `BeginPaint`, vtable `+0x28` (background and backing store), then `FUN_005e22a0` calls `FUN_00601ce0` for labels `+0xa4`..`+0xbc` in order. Labels paint after the art, so the count line is drawn over the art's lower part.

**Side summary.** The code branches by side only for the type table (item 4) and art (item 5), via the local player's side, and at group creation via the squadron owner `+0x168`. The group name, order, tactics and label geometry, font and colour are identical for both factions.

## Functions

| Function | Address | Role |
|---|---|---|
| [FUN_005e16a0](FUN_005e16a0.c) | `005e16a0` | fighter-group panel refresh (vtable `0x0066d670` + `0x3c`); Ghidra did not define it |
| [FUN_005e1fd0](FUN_005e1fd0.c) | `005e1fd0` | creates the seven labels, font 8, white |
| [FUN_005e22a0](FUN_005e22a0.c) | `005e22a0` | paints the seven labels |
| [FUN_005e1320](FUN_005e1320.c) | `005e1320` | window procedure; sets panel id `0x17`; loads background 1307 |
| [FUN_005e1550](FUN_005e1550.c) | `005e1550` | creates the Missions and Maneuvers buttons |
| [FUN_005e1660](FUN_005e1660.c) | `005e1660` | button commands `0xdd01`, `0xdd02` |
| [FUN_005e2320](FUN_005e2320.c) | `005e2320` | 102 x 107 art save buffer at `+0xa0` |
| [FUN_005e1170](FUN_005e1170.c) | `005e1170` | panel constructor, vtable `0x0066d670` |
| [FUN_005e23b0](FUN_005e23b0.c) | `005e23b0` | vtable slot (calls `FUN_00595c40`) |
| [FUN_005c5800](FUN_005c5800.c) | `005c5800` | creates the seven panels |
| [FUN_005c5a50](FUN_005c5a50.c) | `005c5a50` | shows panel by id and calls its `+0x3c` refresh |
| [FUN_005c5760](FUN_005c5760.c) | `005c5760` | moves a panel on screen (0, 0) or off (600, 600) |
| [FUN_005c0f00](FUN_005c0f00.c) | `005c0f00` | selects panel `0x17` for a fighter group |
| [FUN_005a5270](FUN_005a5270.c) | `005a5270` | caller; fighter-group guard `FUN_005ad8d0` |
| [FUN_005a4640](FUN_005a4640.c) | `005a4640` | second caller of `FUN_005c0f00` |
| [FUN_005ad8d0](FUN_005ad8d0.c) | `005ad8d0` | fighter-group lookup guard |
| [FUN_005c6530](FUN_005c6530.c) | `005c6530` | a: sum of `FUN_005ba330` over the group |
| [FUN_005c6570](FUN_005c6570.c) | `005c6570` | b: sum of `FUN_005ba380` over the group |
| [FUN_005ba330](FUN_005ba330.c) | `005ba330` | per-squadron `max(1, trunc(+0x3cc / 8))` |
| [FUN_005ba380](FUN_005ba380.c) | `005ba380` | per-squadron `max(1, trunc(+0x3c8 / 8))`; Ghidra lost the `this`-pointer reads |
| [FUN_005c7150](FUN_005c7150.c) | `005c7150` | returns group colour index `+0xbc` |
| [FUN_005c7140](FUN_005c7140.c) | `005c7140` | sets group colour index |
| [FUN_005c62b0](FUN_005c62b0.c) | `005c62b0` | group constructor; `+0xbc = 4` |
| [FUN_005adfa0](FUN_005adfa0.c) | `005adfa0` | builds one group for a type and colour index |
| [FUN_005ae330](FUN_005ae330.c) | `005ae330` | same-shaped group builder |
| [FUN_005ab650](FUN_005ab650.c) | `005ab650` | groups squadrons per type and side; colour assignment |
| [FUN_00595c40](FUN_00595c40.c) | `00595c40` | local player's side |
| [FUN_00599c70](FUN_00599c70.c) | `00599c70` | side field `+0x9ec` |
| [FUN_00596850](FUN_00596850.c) | `00596850` | button enable guard for the fighter panel |
| [FUN_00596b60](FUN_00596b60.c) | `00596b60` | tactic field `+0x38` |
| [FUN_00601880](FUN_00601880.c) | `00601880` | label constructor |
| [FUN_00601930](FUN_00601930.c) | `00601930` | label geometry, format and flags |
| [FUN_00601b30](FUN_00601b30.c) | `00601b30` | label set position |
| [FUN_00601b80](FUN_00601b80.c) | `00601b80` | label measure (`DT_CALCRECT`) |
| [FUN_00601ce0](FUN_00601ce0.c) | `00601ce0` | label paint (`DrawTextA`) |
| [FUN_00601e60](FUN_00601e60.c) | `00601e60` | set label text from a stream, invalidate, measure |
| [FUN_00601de0](FUN_00601de0.c) | `00601de0` | set label text from a key, invalidate, measure |
| [FUN_00601a20](FUN_00601a20.c) | `00601a20` | take the text out of a stream |
| [FUN_00601aa0](FUN_00601aa0.c) | `00601aa0` | set label text |
| [FUN_006019a0](FUN_006019a0.c) | `006019a0` | set label text from a key |
| [FUN_00601c60](FUN_00601c60.c) | `00601c60` | set label font index |
| [FUN_00601c90](FUN_00601c90.c) | `00601c90` | set label colour |
| [FUN_0060eed0](FUN_0060eed0.c) | `0060eed0` | creates font index N (index 8 = Arial 12, weight 400) |
| [FUN_0060eea0](FUN_0060eea0.c) | `0060eea0` | font table getter |
| [FUN_0060aa30](FUN_0060aa30.c) | `0060aa30` | string loader: `LoadStringA(module(group), id)` |
| [FUN_005f37e0](FUN_005f37e0.c) | `005f37e0` | key `{id, group}` to CString |
| [FUN_005e7b10](FUN_005e7b10.c) | `005e7b10` | contrast: refresh of panel `0x16` (not the fighter panel) |
| [FUN_005e7030](FUN_005e7030.c) | `005e7030` | contrast: panel `0x16` constructor |
| [FUN_005e7800](FUN_005e7800.c) | `005e7800` | contrast: panel `0x16` labels |
| [FUN_005e6d30](FUN_005e6d30.c) | `005e6d30` | contrast: panel `0x16` window procedure |
| [FUN_005e4b40](FUN_005e4b40.c) | `005e4b40` | contrast: capital-ship refresh, panel `0x18` |
| [FUN_005e3e10](FUN_005e3e10.c) | `005e3e10` | contrast: capital window procedure, panel id `0x18` |

## Evidence

- Fighter-panel identity: `FUN_005c5800` instantiates seven panels, including `FUN_005e6b90` (class of `FUN_005e7030`), `FUN_005e3c80` (capital) and `FUN_005e1170`. Their window procedures set ids `0x16`, `0x18`, `0x17` (`FUN_005e6d30`, `FUN_005e3e10`, `FUN_005e1320`). Refreshes are at vtable `+0x3c`: `FUN_005e7b10` (slot `0x0066d78c`), `FUN_005e4b40` (`0x0066d71c`), `FUN_005e16a0` (`0x0066d6ac`). `FUN_005e16a0` reads Red/Blue/Green/Gold names, type ids `0x1d`-`0x24`, bitmaps 2030-2037 and the squadron list at `group+0x1c`; `FUN_005e7b10` reads Hull/Shield meters, a task-force index and `unit+0x144`.
- Label geometry: `FUN_005e1fd0` calls `FUN_00601880(text, x, y, w, h, fmt, flags)`: (26, 9, 97, 9, 1, 0x10), (20, 20, 102, 9, 0, 0x10), (20, 31, 102, 9, 0, 0x10) and four (-100, -100, 102, 9, 1, 0x10). `FUN_00601930` stores width/height at `+0x00`/`+0x04`, x/y at `+0x14`/`+0x18`, format at `+0x10`, flags at `+0x2c`.
- Strings: the ids exist only in TEXTTACT.DLL (`RT_STRING` parsed with a small script; TEXTCOMM.DLL spans `0x1000`-`0x1161` and TEXTSTRA.DLL `0x1300`-`0x9a9b`). Key `{id, 3}` where `DAT_0065d428 = 3` is the group; `FUN_0060aa30` calls `LoadStringA(module(key.group), key.id)`. Values read: `0xdd88`-`0xdd8b`, `0xdd8d`-`0xdd8f`, `0xddc5`-`0xddcc`, `0xddda`-`0xdddc`, `0xdde1`-`0xdde3`, `0xdde5`-`0xdde9`, `0xddea`, `0xddeb`. A0's three top lines and "Fighters: 6 72/72" are each reproduced by the formulas above.
- Position moved: the only `FUN_00601b30` call between `0x005e0000` and `0x005f0000` is `0x005e1e30` inside `FUN_005e16a0`: `PUSH 0x92; PUSH 0x17; CALL 0x00601b30` on `[ESI+0xb0]`. No refresh code touches the other six labels' positions.
- Font and colour: `FUN_005e1fd0` calls `FUN_00601c60(label, 8)` and `FUN_00601c90(label, 0x2ffffff, 0)` for all seven; `FUN_0060eed0` case 8: `lfHeight 0xc`, `lfWeight 400`, `lfQuality 2`.
- Background and art: bitmap sizes read from the DIB headers in TACTICAL.DLL: 1307 is 149 x 236; 2030-2037 are 102 x 107. 
- Not run: no game run, no Wine, no A0 comparison; every pixel claim below rests on reading code.

## Open questions

- Alignment. Labels 1 and 4 are created with format `1` (`DT_CENTER`). `FUN_00601b80` (measure) clears that bit on its first pass and then sets the box to `[x - 1, x + textwidth + 1]` (the setters zero the right edge first), so by code reading the text appears left-aligned starting one pixel left of the listed x, not centred. hyp: confirm on an A0 frame of the "Red Group" line.
- Off-by-one origin. The container is created at (482, 28); the P58F9 command-assignment note uses (481, 27) for the same panel. Both can be right if the container window has a one-pixel border. Settle with an A0 capture of a known pixel.
- Button name keys. `FUN_005e1550` passes key `DAT_006bd364` (`0xddcd` "Maneuvers & Tactics") with id `0xdd02` (x = 12, bitmaps 1107/1108) and `DAT_006bd448` (`0xdd8c` "Missions") with id `0xdd01` (x = 80, bitmaps 1105/1106). Settled by A0: the left button opens Maneuvers and Tactics and the right one Missions, for a fighter group too (`tactical-command-assignment.md`, TAC-05-C012, TAC-05-C021); which handler reads `0xdd02` is still unread (vtable slot `+0x4c`).
- Which units select panel `0x16` and which `0x18`. `FUN_005ad870` (`unit+0x170` in {2, 4, 6}) guards `0x16`; `FUN_005ad900` and `+0x68 == 3` guard `0x18`. Not needed here.
- The side test in `FUN_005e16a0` is the local player's side, not the selected group's owner. If an enemy group can be selected, the Alliance player would see no type match and no art. hyp: settle by finding whether `FUN_005a5270`'s selection set can contain enemy groups.
- `(group+0x80)->vslot 0xc` and `(group+0x98)->vslot 0x10` are read as "is navigating" and "current target"; the vtables were not decoded.
- The enable guard `FUN_00596850` (the `FUN_005ae170` half) decides whether the two buttons are shown/enabled for the player.
- Group split rule at `0x005abe5f`: I read "largest type with at least 8 squadrons is halved into an unused colour". hyp: not checked against a battle.
