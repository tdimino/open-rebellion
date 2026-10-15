---
title: "Text Sources: String Loader, Text DLLs, Version Gate and Executable Literals"
description: "Where every player-visible string comes from: the string hook FUN_0060aa30 and its module groups, the id ranges of TEXTSTRA, TEXTCOMM and TEXTTACT, the TEXTSTRA version gate that selects English literals, ENCYTEXT, REBDLOG and CDDERR, the data0.reb override, and the literals and glue fixed in REBEXE"
category: "ghidra"
created: 2026-10-14
updated: 2026-10-14
tags: [text, localization, textstra, textcomm, texttact, dll, mod-surface]
---

# Text sources

Answers task-graph node `or-q-mod-text-ju9k` (epic `or-ep-mod-surface-vdch`), from the Feature Wiki page `executable/mod-text`; the string hook, the key-to-string path, the TEXTSTRA version test, the `data0.reb` override and the `Current Latency: ` label were reread here. String and resource counts come from that page's PE resource reader over the shipped DLLs. `hyp:` marks anything not traced through code.

## Summary

Replacing TEXTSTRA, TEXTCOMM and TEXTTACT does not cover all player-facing text. Three more DLLs carry visible text (ENCYTEXT, REBDLOG, CDDERR) besides TEXTSTRA's own RT_RCDATA, the executable holds about 15 visible literals plus separators, five start-up message boxes can be overridden by a file `data0.reb`, and a TEXTSTRA version check decides whether five labels come from the executable or from TEXTSTRA. The RT_RCDATA scripts in ALSPRITE, EMSPRITE, ALBRIEF and EMBRIEF hold id lists, no text. Text baked into bitmaps is not covered by this reading.

## The loader

A string key is four bytes, `u16 id` then `u16 group` (`CONCAT22(group, id)`; the constants `DAT_0065d420`, `DAT_0065d424`, `DAT_0065d428` hold 1, 2, 3).

- `FUN_005f37e0(key, &cstring)` starts from the empty string `DAT_006b120c`. When the hook `DAT_006bde14` is set it calls `hook(key, 0x100, buffer)` and copies the buffer. A string is therefore cut at 255 characters.
- The hook is `FUN_0060aa30(key, size, buffer)` = `LoadStringA(FUN_005fefd0(key.group), key.id, buffer, size)`, installed once by `FUN_0060aa20` (through `FUN_0060aa00`, from the start-up routine `FUN_004165b0` after `FUN_00406600`). `FUN_005fefd0` walks the module list `DAT_006be4c8` (`FUN_005f5920`: node `+0xc` group, `+0x10` `HMODULE`).
- **A missing group or id gives the empty string.** There is no fallback to another DLL.
- `FUN_005ff020(dir, dllName, group)` adds a node: the first load of a group wins; otherwise `LoadLibraryA(dir + "\" + name)`, retried with the bare name (`FUN_0060ed60` builds the node). `dir` is the registry value `DllLocation` (`FUN_00401440`).

The group is the same module id the sound path uses (`audio-dispatch.md`, Module map).

| Group | DLL | Loaded by | Text |
|---|---|---|---|
| 1 | TEXTCOMM | `FUN_00406950`, `FUN_00413000`, `FUN_00414a80`, `FUN_004138a0`, `FUN_00421c70` | RT_STRING: front end, multiplayer |
| 2 | TEXTSTRA | `FUN_00414830`, `FUN_00415440` (via `FUN_005fba30`) | RT_STRING (strategy UI, names) and RT_RCDATA (advisor topics, story text) |
| 3 | TEXTTACT | `FUN_0059a8d0` | RT_STRING: tactical UI |
| 4 | ENCYTEXT | `FUN_0045d400` (via `FUN_00603810`) | RT_RCDATA: encyclopedia bodies |
| 5 | ENCYBMAP | `FUN_0045d400`, `FUN_00466350` | RT_STRING holding file names `EDATA.nnn` |
| 6, 7, 8 | TACTICAL, STRATEGY, COMMON | `FUN_0059a8d0`, `FUN_00421c70`, `FUN_00404c30` and others | art, RCDATA without text |
| 9, 13 | ALSPRITE/EMSPRITE, ALBRIEF/EMBRIEF | `FUN_0042d650` (`param_1 == 1` loads the Alliance pair) | RCDATA without text |
| 10 | GOKRES | eleven window constructors | art |
| 12 | REBDLOG | `FUN_00413000` | RT_STRING: quit and resume dialogs |
| 14 | VOICEFXA / VOICEFXE | `FUN_0059a8d0` | audio |

Group 11 is never loaded.

## Id ranges

| DLL | RT_STRING | RT_RCDATA |
|---|---|---|
| TEXTCOMM | 80: `0x1000-0x1032`, `0x1060-0x1061`, `0x1100-0x1119`, `0x1150-0x1161` | none |
| TEXTSTRA | 1347 in 153 blocks, `0x1300-0x9a9b`: UI labels `0x1500-0x1954`, facilities and units `0x2001-0x2884`, characters `0x2a40-0x2a9b`, mission names `0x2c01-0x2c83`, galaxy and system names `0x2d00-0x2f99`, build and message UI `0x3000-0x3823`, side names `0x4d50-0x4d71`, tactical report and ship names `0x5001-0x52cf`, message settings `0x8000-0x8032`, status windows `0x8500-0x8803`, rank-prefixed names `0x9040-0x9a9b` | 511: text in 425 (`0x6004-0x605f` Alliance and `0x6804-0x685f` Empire topics, `0x7001-0x721b`, `0x7365`: victory, story and encounter text), id lists in 86 |
| TEXTTACT | 149: `0xad81-0xadb9` (object and group names), `0xdd81-0xddf2`, `0xff00-0xff0d` (file names `MDATA.401` and others) | none |

Every system, character and unit name is a TEXTSTRA id; the DAT files hold no names.

## The TEXTSTRA version gate

`FUN_00406600` sets `DAT_006b0df8 = FUN_00406850()`. `FUN_00406850` reads TEXTSTRA's `FileVersion` (`VerQueryValueA`) and returns true when its minor and revision fields are both 0, or when the version cannot be read. The shipped TEXTSTRA is `1.00.00`, so the flag is true; `FUN_004067d0` returns it. Its five users show an executable literal when it is true and a TEXTSTRA id when it is false:

| Function | Literal (flag true) | TEXTSTRA id (flag false) | Id shipped? |
|---|---|---|---|
| `FUN_0049eea0` (battle overlay) | `Battle in Progress...` (`0x006a879c`) | `0x1a00` | no |
| `FUN_0044a2e0` (Character Status row) | `Attached:` (`0x006a8738`) | `0x87b0` | no |
| `FUN_0044a470` (Status row) | `Status:` (`0x006a8744`) | `0x8666` | yes |
| `FUN_00449200` (Fleet Status, hyperdrive) | `Yes` (`0x006a8734`), `No` (`0x006a8730`) | `0x8756`, `0x8755` | yes |
| `FUN_00441520` (strategic view labels) | name only | `0x4da0` plus the name | no |

A TEXTSTRA that keeps `1.00.00` keeps the English literals whatever its strings say. One with a non-zero minor or revision switches to the ids and must add `0x1a00`, `0x87b0` and `0x4da0` (hyp: untested in a run).

## Text outside the three DLLs

| Source | Content | Read by |
|---|---|---|
| ENCYTEXT RT_RCDATA (group 4) | 348 entries from `0x1001`, all English: cost lines and descriptions | `FUN_0045fa60` (`FindResourceA(module +0x158, id, RT_RCDATA)`; `encyclopedia-window.md`) |
| REBDLOG RT_STRING (group 12) | 39 strings `0x1200-0x1911` (`Quit Star Wars Rebellion`, `Resume Game Play?`, `Your Opponent Has Left The Game`, `To Cockpit`) | hyp: the quit and multiplayer dialogs |
| CDDERR RT_STRING | 131 strings, caption `0xddff`, texts `0xde00-0xde93` (DirectDraw, DirectPlay, Direct3DRM errors) | `FUN_00614530`: `LoadStringA`, then `MessageBoxExA(..., 0x409)`; loaded by `FUN_00614050` |
| `data0.reb` | five quoted strings that replace the start-up messages | `FUN_00406600` |
| REBEXE RT_RCDATA | 59 path fragments (`SAVEGAME\`, `GDATA\`, `SECTORSD.DAT`, `C3POACT.SPT`) | `FUN_005f49e0` (`DAT_006bdf18`, set by `FUN_005f49c0`) |

`GData\TEXTSTRA.DLL` is byte-identical to the root copy; hyp: the root copy is the one loaded (`FUN_005ff020` joins `DllLocation` and `textstra.dll`).

## Literals fixed in the executable

**Shown, with no TEXTSTRA id:**

| Address | String | Shown by |
|---|---|---|
| `0x006a86f0` | `Current Latency: ` | `FUN_0042d780` builds the label (`FUN_00601aa0`) |
| `0x006ac824` | `Display resolution must be a minimum of 640 x 480 using 256 colors.` | `FUN_0060a910` → `FUN_0060a9e0` (`MessageBoxA`), from `FUN_004165b0` |
| `0x006ac868` | `StarTrader is alreay running. Only one copy of this application may` + `0x08` + `e executed at a time.` (typo and stray backspace byte in the file) | `FUN_0060a910` |
| `0x006ac8cc` | `No mouse was detected. A mouse is required to play StarTrader.` | `FUN_0060a910` |
| `0x006ac7b8` | `Arial` | `FUN_005ff830` → `FUN_0060ee20` writes the `LOGFONT` face; charset stays 0 |
| `0x006a80ec` | `Rebellion` | pushed at `0x0040dbac` in an undefined function; hyp: window caption or class name |

**Start-up messages with a file override.** `FUN_00406600` copies five defaults into buffers: caption `Error: CD Not Found` (`0x006a7e64` → `DAT_006b0bf8`), the CD text (`0x006a7dec` → `DAT_006b1000`), caption `Error: Versions Incompatible` (`0x006a7dcc` → `DAT_006b0e00`), the older-than-opponent text (`0x006a7d38` → `DAT_006b07f8`) and the opponent-older text (`0x006a7ca4` → `DAT_006b09f8`). `FUN_004067e0`, `FUN_00406800` and `FUN_00406820` show them (`MessageBoxA`, type `0x41010`). When `data0.reb` opens (`FUN_00619540`), quoted strings 1, 3, 5, 7 and 9 of its text replace the five in that order; the file is XOR `0xdd` unless its first byte is `#`, and a `\` becomes a space and a newline. No `data0.reb` ships.

**Glue.** Separators fixed in code: `" "` (`0x006a7f40`; fighter-group panel `FUN_005e1fd0`, `FUN_005e16a0`, ship panels, status and list rows), `"/"` (`0x006aa5cc`; the `Fighters: N a/b` line in `FUN_005e16a0`), `":"` (`0x006a872c`; numeric rows in `FUN_00449200`, `FUN_004444c0`, `FUN_00446250`, `FUN_00446fd0`, `FUN_00457690`), `" ( "` / `" ) "` (`0x006a8790` / `0x006a878c`; the Personnel Finder row in `FUN_00465bb0`, whose ` - ` is TEXTSTRA `0x1897`), `"-"` (`0x006a8788`; empty-count cells in `FUN_00465540`, `FUN_0046d8d0`), `"  "` (`0x006a874c`; `FUN_0044aa20`), `":  "` (`0x006a8798`; `FUN_0049a8b0`), `"\n"` (`0x006a8170`; label builders such as `FUN_00441520`).

The fighter-group panel's `Tactics:`, `Missions` and `Maneuvers & Tactics` are TEXTTACT keys (`DAT_006bd32c` = `0xddda` group 3), as are the group names `0xdd88 + colour` (`tactical-fighter-group-panel.md`).

## Strings that never reach the player

| Strings | Where | Why |
|---|---|---|
| About 600 notification and field names (`CharacterForceNotif`, `Delta=`, `ssDEAD`), side and state words (`Alliance`, `Empire  `, `TRUE `) | `0x006a8854-0x006ab4e0`, `0x006a8eec-0x006a8f28`, `0x006aac28-0x006aac64` | only the debug loggers (`FUN_004f88d0`, `FUN_004f8c60`, `FUN_0053b3e0` and others) read them, behind `FUN_006158b0()` (returns 0) or the flag test `FUN_0053a010` |
| `identify traitor`, `force-detect`, `foil`, `decoy`, `not betray` | `0x006ab3cc-0x006ab3fc` | passed to `FUN_005897b0`, a stub returning 1 |
| `TACT_*`, `STRAT_*` state names | `0x006a7f58-0x006a80dc` | `FUN_0040a700` builds a `CString` that `FUN_0040a050` discards |
| Direct3D and DirectDraw messages | `0x006abd80-0x006ac1a4` | `FUN_005d93f0` ignores the text |
| Window class names (`Game Menu Window`, `CustomDialogBox`, `MOVIE_WINDOW`, `3D_WINDOW`) | | `FUN_005fffb0` stores the class at `+0x48`; the caption is `+0x4c` (`FUN_005fff30`) |
| Tactical event names (`SHIP_FIRELASERCANNON`) and ship-type keys (`CALAMARI`, `XWING`, `VICTORY`) | `0x006ab510-0x006abd40` | `FUN_00597610` builds `CString`s of the type keys (`ship-type-assets.md`) |
| DirectPlay trace, registry keys, paths, C runtime text | | logs, system names, CRT fatal errors |

## Mod surface

1. **TEXTSTRA, TEXTCOMM, TEXTTACT at the same ids.** Ids are literals in code (`0xdd88 + colour`, `0x1060`/`0x1061`, `0x8755`/`0x8756`, side-prefixed name blocks); a missing id shows an empty label. Strings stay under 255 characters.
2. **ENCYTEXT RCDATA, REBDLOG, CDDERR and the TEXTSTRA RCDATA** also hold visible text. The sprite and briefing RCDATA need no change for a translation.
3. **Executable literals:** the five start-up messages can come from a `data0.reb`; `Current Latency: ` and the three start-up boxes need a patched executable, and `.data` has no spare room for longer strings.
4. **Version gate:** keep TEXTSTRA `1.00.00` to keep the English literals, or change it and add `0x1a00`, `0x87b0`, `0x4da0`.
5. **Glue and font:** the separators and the `Arial` face with charset 0 are code; a language with other word order, punctuation or script needs patches there.

## Functions

| Function | Role |
|---|---|
| [FUN_0060aa30](FUN_0060aa30.c), [FUN_0060aa20](FUN_0060aa20.c) | String hook (`LoadStringA` by group) and its installer |
| [FUN_005f37e0](FUN_005f37e0.c) | Key to `CString`: empty default, 0x100 buffer |
| [FUN_005fefd0](FUN_005fefd0.c), [FUN_005f5920](FUN_005f5920.c) | Group to `HMODULE` |
| [FUN_005ff020](FUN_005ff020.c), [FUN_0060ed60](FUN_0060ed60.c), [FUN_00401440](FUN_00401440.c) | Load a DLL into a group; node constructor; `DllLocation` |
| [FUN_004165b0](FUN_004165b0.c) | Start-up: semaphore, `FUN_0060a910`, `FUN_00406600`, hook install |
| [FUN_00406600](FUN_00406600.c) | Message defaults, `data0.reb` override, version flag |
| [FUN_00406850](FUN_00406850.c), [FUN_004067d0](FUN_004067d0.c) | TEXTSTRA version test; flag getter |
| [FUN_004067e0](FUN_004067e0.c), [FUN_00406800](FUN_00406800.c), [FUN_00406820](FUN_00406820.c) | The start-up `MessageBoxA` calls |
| [FUN_0060a910](FUN_0060a910.c), [FUN_0060a9e0](FUN_0060a9e0.c) | Display, mouse and single-instance checks |
| [FUN_00406950](FUN_00406950.c), [FUN_00414a80](FUN_00414a80.c), [FUN_00413000](FUN_00413000.c), [FUN_00421c70](FUN_00421c70.c), [FUN_00414830](FUN_00414830.c), [FUN_00415440](FUN_00415440.c), [FUN_0059a8d0](FUN_0059a8d0.c), [FUN_0042d650](FUN_0042d650.c), [FUN_0045d400](FUN_0045d400.c) | DLL loaders by group |
| [FUN_0045fa60](FUN_0045fa60.c) | Encyclopedia RCDATA text read |
| [FUN_0049eea0](FUN_0049eea0.c), [FUN_0044a2e0](FUN_0044a2e0.c), [FUN_0044a470](FUN_0044a470.c), [FUN_00449200](FUN_00449200.c), [FUN_00441520](FUN_00441520.c) | Users of the version gate |
| [FUN_0042d780](FUN_0042d780.c) | `Current Latency: ` label |
| [FUN_00614530](FUN_00614530.c), [FUN_00614050](FUN_00614050.c) | CDDERR error box and its DLL |
| [FUN_005ff830](FUN_005ff830.c), [FUN_0060ee20](FUN_0060ee20.c) | `Arial` font object |
| [FUN_005ffce0](FUN_005ffce0.c), [FUN_005fff30](FUN_005fff30.c), [FUN_005fffb0](FUN_005fffb0.c) | `CreateWindowExA`; caption and class-name setters |
| [FUN_006158b0](FUN_006158b0.c), [FUN_005897b0](FUN_005897b0.c), [FUN_0053a010](FUN_0053a010.c) | Debug gates |
| [FUN_004f88d0](FUN_004f88d0.c), [FUN_0053b3e0](FUN_0053b3e0.c), [FUN_00588e80](FUN_00588e80.c), [FUN_0058a3f0](FUN_0058a3f0.c) | Debug-only string users |
| [FUN_0040a700](FUN_0040a700.c), [FUN_0040a050](FUN_0040a050.c), [FUN_005d93f0](FUN_005d93f0.c), [FUN_005d6930](FUN_005d6930.c), [FUN_00597610](FUN_00597610.c) | Discarded state names, ignored D3D text, ship-type keys |
| [FUN_005f49c0](FUN_005f49c0.c), [FUN_005f49e0](FUN_005f49e0.c) | Executable RCDATA reader (path fragments) |

## Open questions

- The function containing `0x0040dbac` and whether `Rebellion` is the main window caption.
- Which dialogs show REBDLOG strings, and whether the CDDERR boxes are reachable in single player.
- Text baked into GOKRES, COMMON, STRATEGY and TACTICAL bitmaps.
- What `0x1a00`, `0x87b0` and `0x4da0` were meant to hold, and how `FUN_00441520` joins `0x4da0` to the name; the message `FUN_0049a8b0` builds with `":  "`.
- Label widths shorter than a translation would need (`FUN_00601aa0` and the buffers of `FUN_005f35b0`).
