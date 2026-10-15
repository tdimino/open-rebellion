---
title: "Audio Dispatch: Sound Descriptors, Module Map, Tactical Table and Music"
description: "How REBEXE routes game events to sound effects, voices and music: a module-id plus wave-id descriptor played from the DLLs, literal ids at each call, the 317-event tactical table, the 16-track music table and its selectors, and what a total conversion must change"
category: "ghidra"
created: 2026-10-14
updated: 2026-10-14
tags: [audio, sound, music, voice, dll, tactical, mod-surface]
---

# Audio dispatch

Answers task-graph node `or-q-mod-audio-igij` (epic `or-ep-mod-surface-vdch`), from the Feature Wiki page `executable/mod-audio`; the module registration, the tactical table, the module split, the side arithmetic and every music selector were reread here. Call-site lists for `FUN_00602840`, `FUN_00610c30`, `FUN_00417610` and `FUN_005ba980` are in that page's assets (`assets/map/mod-audio/`). `hyp:` marks anything not traced through code.

## Summary

1. **No central table for strategy sound.** Every sound is a *sound descriptor* (vtable `0x658518`): `{+4 wave id, +8 module id, +0xc, +0x10 flags}`, built by `FUN_00610b20` and played by `FUN_00610c30` (→ `FUN_006115f0`). The play path turns the module id into a loaded DLL (`FUN_005fefd0`, `FUN_00611440`) and loads `RT "WAVE"` resource `wave id` from it (`FindResourceA` in `FUN_00611890`). Wave id and module id are literals at each call site, except three data-driven paths: advisor scripts, one dialog record, and the tactical table.
2. **Module id → DLL** is fixed by `FUN_005ff020(app, name, id)` calls (table below). Side branching is by DLL name (modules 9, 13, 14) or by arithmetic on a side field.
3. **Three other mechanisms:** (a) UI controls store a sound id and take the module from their own window's DLL (`FUN_00602840`: 53 call sites, 6 ids); (b) the tactical engine's singleton fills a 317-event table in its constructor with 351 literal wave ids (13000..15163), played from TACTICAL.DLL (module 6) for events below `0x20` and VOICEFXA/VOICEFXE (module 14) from `0x20` up; (c) music and video bypass the DLLs: 16 music files `MDATA\MDATA.300`..`.315` by a 0..15 track index, and 16 video files by a switch (`movies.md`).
4. **Music** is a 16-slot pointer table at `0x006a8478`. Tracks 4..9 are chosen by the side's advice agent (the two sides use opposite orders), 1..3 are random calm tracks, 10 and 11 the Battle Alert window, 12..15 the battle result window, 0 the fallback.
5. **Swapping DLLs alone does not replace all audio.** It replaces strategy and tactical effects and voices if the new DLLs carry `WAVE` resources at the same ids. It cannot replace music or video soundtracks (loose `MDATA` files) and cannot add an event.

| Path | Entry | Id source | Module source | Limit |
|---|---|---|---|---|
| Control sound (buttons, list rows) | `FUN_00602840(this, id)` sets `+0xdc`, `+0xe4`, `+0xe8 = FUN_005feff0(+0x1c)`, `+0xf0 \|= 1` | literal at each of 53 sites | the control's owning DLL (`+0x1c` handle → module id) | literal per call site |
| Strategy event sound | `FUN_00610b20` then `FUN_00610c30(desc)` | literal (some `base - (side != 1)`) | literal `7` (STRATEGY) | literal per call site |
| Advisor scripts and briefings | `FUN_00403f70` / `FUN_00403f00` copy `(wave, module)` from a script word pair, then `FUN_00610c30` | script data (RT_RCDATA in ALSPRITE/EMSPRITE/ALBRIEF/EMBRIEF) | script data: 9 or 13 | DLL data |
| Tactical sounds | `FUN_005ba980(mgr, event, flag)` → `FUN_005baa60` / `FUN_005bad50` | ushort table at `mgr+0x51c + event*8`, filled by `FUN_005bae60` | event `< 0x20` → 6, else 14 | table in constructor code, 317 events × 4 variants |
| Music | `FUN_00417520(track)` | `PTR_s_MDATA_300_006a8478[track]`, track 0..15 | none: path `MDATA\` + name | table plus literal track numbers |
| Video | `FUN_00421ba0` (switch on `+0x98`, `0x65..0x74`) | literal file names | none | switch in code |

## Module map

`FUN_005ff020(app, dllName, moduleId)` loads a DLL and registers it.

| Module | DLL | Registered by | Side branch |
|---|---|---|---|
| 1 | TEXTCOMM.DLL | `FUN_00406950`, `FUN_00413000`, `FUN_00414a80`, `FUN_00421c70` | none |
| 2 | TEXTSTRA.DLL | `FUN_00414830`, `FUN_00415440` (via `FUN_005fba30`) | none |
| 3 | TEXTTACT.DLL | `FUN_0059a8d0` | none |
| 4 | ENCYTEXT.DLL | hyp: through `FUN_00603810` from `FUN_0045d400` (not read) | hyp |
| 5 | ENCYBMAP.DLL | `FUN_00466350` | none |
| 6 | TACTICAL.DLL | `FUN_0059a8d0` | none: tactical effects, events `0..0x1f` |
| 7 | STRATEGY.DLL | `FUN_00421c70` | none: strategy sounds `0x258..0x265` |
| 8 | COMMON.DLL | `FUN_00404c30`, `FUN_00406950`, `FUN_00414a80` | none: main menu sounds `0x1f40..0x1f44` |
| 9 | ALSPRITE.DLL / EMSPRITE.DLL | `FUN_0042d650` | `param_1 == 1` selects ALSPRITE |
| 10 | GOKRES.DLL | `FUN_00437880`, `FUN_0044c410`, `FUN_00442d70`, `FUN_0046f140`, `FUN_0049f130`, `FUN_0046cbc0`, `FUN_004a2630`, `FUN_004a7790`, `FUN_00452fc0`, `FUN_0046a750`, `FUN_0044f670` | none |
| 12 | REBDLOG.DLL | `FUN_00413000` | none |
| 13 | ALBRIEF.DLL / EMBRIEF.DLL | `FUN_0042d650` | `param_1 == 1` selects ALBRIEF |
| 14 | VOICEFXA.DLL / VOICEFXE.DLL | `FUN_0059a8d0` | `FUN_00595c40()` (the tactical session's `+0x9ec`): 0 loads VoiceFxA, else VoiceFxE. hyp: tactical side 0 is the Alliance, from the A/E naming and the Empire-only `iVar == 1` branch in `FUN_005ba7f0` |

The module id and the DLL are tied only by this registration. A conversion that keeps the DLL names keeps every id valid; renaming a DLL means changing the string tables (`0x006a86a8..0x006a8760`, `0x006ab6e4..0x006ab714`) and the registration calls.

## Strategy sound effects and voices

- **Descriptor:** `FUN_00610b20` stores `+4` wave id, `+8` module id, `+0xc` extra, `+0x10` flags (low 3 bits type, `1` = WAVE resource; bit 3 loop; bit 4 sticky). Most callers construct it with zeros and write the fields (`FUN_00429020`: wave id, module 7, `flags & ~6 | 1`).
- **Play:** `FUN_006115f0` looks up a cache keyed `wave id + module * 0x10000` (`FUN_00612bd0`), else loads the `WAVE` resource from the module's DLL (`FUN_00611440` → `FUN_005fefd0` → `FUN_00611890`). A missing wave returns an error and plays nothing.
- **Per-side ids by arithmetic:** `FUN_00429020` plays `0x265 - (side != 1)`: `0x265` for the Alliance (`+0x9c == 1`), `0x264` for the Empire. `FUN_00428b40` plays `0x263` / `0x262` the same way. Both use module 7, so both sides' sounds live in STRATEGY.DLL.
- **Literal ids at descriptor call sites (module 7):** `0x259` (`FUN_00422ce0`, `FUN_00425d00`); `0x25c` (`FUN_00459e30`); `0x25d` and `0x25f` (`FUN_00422ce0`, switch on `param_3 & 0x3f`: case 1 → `0x25d`; 4, 9, 10, 11 → `0x25f`); `0x25e` (`FUN_00455060`, `FUN_0049f540`, `FUN_004a4b10`, `FUN_004a8790`; the Fleet Window create sound of `fleet-window.md`). No literal site plays `0x25a`, `0x25b` or `0x261`.
- **Control sounds (`FUN_00602840`):** 53 call sites in 15 windows. `0x260` at 41 (Fleet Finder, Fleet Window, Battle Alert and similar buttons), `0x258` at 1 (`FUN_004286b0`), and in the main menu (`FUN_00405560`) `0x1f44` at 8 and `0x1f40`, `0x1f41`, `0x1f42` at one each. The function only stores the id; the control's event path plays it (`FUN_00603710`, `FUN_006028c0` play the descriptor at `+0xe0` / `+0x38`). Because the module comes from `FUN_005feff0(+0x1c)`, the id resolves in the DLL the control's window belongs to: `0x260` in STRATEGY.DLL for strategy windows, `0x1f44` in COMMON.DLL for the main menu (hyp for the per-window DLL until `+0x1c` of each window is read).
- **Advisor scripts** (modules 9 and 13): `FUN_00403f70` / `FUN_00403f00` take wave and module from the script word pair `(wave, module)`; `droid-advisor-triggers.md` gives the script format. Both are DLL data. `FUN_004c4a20` plays a caller-supplied wave from module 9.
- **Dialog record:** `FUN_00469de0` plays two waves from the mission dialog record (`FUN_0046a320`'s words `+0x2c` and `+0x32`), the first from module 7 and the second from module 9. Where that record comes from is not traced.

## Tactical sounds and ship voices

- **Singleton:** `FUN_005ba8b0` creates the manager (`DAT_006bc4c8`, `0x1420` bytes) with `FUN_005bae60`. For each of `0x13d` (317) events the constructor fills four ushort wave-id variants at `+0x51c + event*8` (default `0x32c8` = 13000) and stores 350 further literal ids up to 15163 (`0x3b3b`), all distinct: 351 ids with the default. It also sets a per-event class at `+0x50` (1..4) and caps at `+0x1c`.
- **Event call sites:** `FUN_005ba980(mgr, event, flag)` has 102 callers: 60 pass a literal event (50 distinct, among them 0..14, 19, 20, 27, 30, 31, 46, 85, 98, 135..138, 145, 149, 155, 168, 181, 194, 207, 220, 245, 258, 260..264, 268..270, 273, 275, 277, 284, 285), 42 compute `base + index` (`+0xed`, `+0xf6`, `+0xb6`, `+0x9c`). hyp: each function in `0x005a2000..0x005a5000` covers one ship or weapon family; settle by reading `FUN_005a3740`, `FUN_005a3850`, `FUN_005a3630`.
- **Module split:** `FUN_005ba980` stores module 6 at `+0x2a` when `event < 0x20` and module 14 otherwise; `FUN_005bad50` copies it into the descriptor. Events `0..0x1f` are TACTICAL.DLL effects, `0x20..0x13c` command and ship voices from VOICEFXA/VOICEFXE. `FUN_005baa60` and `FUN_005bad50` guard `event < 0x13d`.
- **Side:** event ids and the wave table are shared; the side only chooses the DLL loaded as module 14 when the tactical window is created (`FUN_0059a8d0`). Per-side voice replacement is a DLL swap at the same wave ids (153 and 132 WAVE resources, `agent_docs/game-media.md`).
- The `.data` strings `VICTORY` (`0x006ab5f8`) and `VICTORY_II` (`0x006ab5b0`) belong to `FUN_00597610`'s list of tactical ship class names (beside `DAUNTLESS`, `STRIKE_CRUISER`, `LANCER`, `INTERDICTOR`, `CARRAK`, `STAR_DESTROYER`, `SUPER_DESTROYER`, `ASSAULT_TRANSPORT`, `GALLEON`, `DESTROYER_II`, `STAR_GALLEON`); they have no audio role.

## Music

- **Table:** `PTR_s_MDATA_300_006a8478[i]` points to `MDATA.(300+i)` for i = 0..15. The directory string `MDATA\` (`0x006a8590`) is joined to the install or data location. Music streams from loose files.
- **Start and stop:** `FUN_00417520(track)` runs only for `track < 0x10` with the music switch on (`DAT_006a8470`): it stops a playing track (`FUN_00417640`, which clears the forced flag `DAT_006b1490` bit 2), sets `DAT_006b147c = track + 1`, builds the path and plays it once (`FUN_00610c40`, then volume `FUN_00610cb0`). `MusicSwitch` and `MusicVolume` are registry values (`FUN_004173c0` reads, `FUN_004176e0` / `FUN_00417670` save; `game-options.md`). `FUN_00417610(track)` stops then plays, marking the track forced (flag 2).
- **End of track:** `FUN_00417780` handles message `0x4c9` for the playing track: a forced track plays again, any other is replaced by the default selector's pick.
- **Default selector:** `FUN_0041d3b0` returns 0 without a session (`DAT_006b14e4` null), else `FUN_004369a0` → `FUN_00487fb0` → `FUN_0043a250(advice agent at session +0xc0)`. `FUN_0043a250` keeps a counter at `+0x190`: while it is above 0 it decrements and returns `rand(6)/2 + 1` (tracks 1..3); at 0 it resets to 3 and returns the agent's vtable slot `+0x1c`. The situational track therefore plays once in four calls. The agent's constructor `FUN_00439320` sets the counter to 0, so a new game's first pick is situational; the save writes and reads it (`FUN_004397a0`, `FUN_00439550`). `rand(n)` is `FUN_0041cd80`: the session generator `DAT_006b14d8` through `FUN_005f5700`, 0 without a session.
- **Situational tracks** (vtable slot `+0x1c`, switch on the agent's state byte `+0x170`; any other value → `rand(3) + 1`):

  | `+0x170` | 1 | 2 | 4 | `0x10` | `0x20` | `0x40` |
  |---|---|---|---|---|---|---|
  | Alliance agent (vtable `0x65c4c8`, `FUN_004c48e0`) | 4 | 5 | 6 | 7 | 8 | 9 |
  | Empire agent (vtable `0x65c4a0`, `FUN_004c2700`) | 9 | 8 | 7 | 6 | 5 | 4 |

  hyp: `+0x170` is the agent's current situation flag; its writers (`FUN_004c27f0`, `FUN_004c0710`) are not read.
- **Fixed tracks:** `FUN_0044f860` (Battle Alert) calls `FUN_00417610(10)` when the window's side `+0x9c == 1` (Alliance), `FUN_00417610(11)` otherwise. The alert's command handler `FUN_00450540` (its vtable at `0x659d70`) takes commands `0x65`, `0x66` and `0x67`, calls `FUN_0041ce60`, stops the music (`FUN_00417640`) and closes the window, so combat itself plays no music. `FUN_0046f360` (battle result window) stores a track at `+0x18c` and sets `+0x188`; the window's paint handler `FUN_00471200` calls `FUN_00470e70`, which plays it forced and clears `+0x188`, so the track starts when the window first paints. The report's winner `+0x50` against the window's side picks it: the player's side won → `0xc` (Alliance) or `0xe` (Empire); no winner (`+0x50 == 0`), the player's side lost, or the other side won → `0xd` or `0xf`. The headline bitmaps on the same branches confirm the reading (10759 Alliance victory or 10758 Empire defeated with `0xc`; 10757 Alliance defeated with `0xd`). The window's close, `FUN_004727a0` (vtable `0x65a300`, slot `+0x30`), stops the track, starts the default selector's pick and requests the close (`FUN_00606960`).
- **Restart:** `FUN_00405050` (the menu window's creation) starts the default selector's pick, track 0 without a session. The cockpit window `FUN_00422ce0` calls `FUN_00422cb0`, which starts the default selector's pick when `+0xb4` is set, on its creation and on message `0x402`, a movie's last frame (`movies.md`), after rereading the music settings (`FUN_004173c0`). The switch's save handler `FUN_004176e0` stops and starts the music. `FUN_004139a4` holds the string "Start Of Intro Music Failed." (hyp: the intro's own music start).
- **Audio engine:** `FUN_00610d00` creates DirectSound (`FUN_00610e30`, `FUN_00611330`) and an MCI sequencer device (`FUN_00611e90`, device type `0x204`); hyp: the shipped files never use the MIDI device. Smacker audio goes through DirectSound (`FUN_00611dc0` calls `SmackSoundUseDirectSound`).

## Mod surface

What a total conversion that replaces or adds audio must change:

1. **DLL swap alone** covers strategy effects (STRATEGY.DLL `0x258..0x265`), control clicks (`0x258`, `0x260` in the window's DLL, `0x1f40..0x1f44` in COMMON.DLL), advisor lines (ALSPRITE/EMSPRITE, ALBRIEF/EMBRIEF scripts and waves) and tactical effects and voices (TACTICAL.DLL, VOICEFXA/VOICEFXE, 13000..15163), at the same ids, DLL names and `WAVE` type.
2. **Music:** the 16 files `MDATA.300..315` can be replaced; their count and names are fixed by the table and the `track < 0x10` bound. Which track plays when is code (`FUN_004c48e0`, `FUN_004c2700`, `FUN_0044f860`, `FUN_0046f360`, `FUN_0043a250`).
3. **Video:** names and switch values `0x65..0x74` are code (`FUN_00421ba0`).
4. **New events:** a new strategy sound needs a new literal at a new call site. A new tactical event needs the table in `FUN_005bae60` extended (317 events fixed by its loop and the `< 0x13d` guards) and a call site.
5. **New DLLs** need another `FUN_005ff020` call; ids 1..14 are taken.
6. **Per-side voices:** the tactical branch is the single `FUN_00595c40()` test in `FUN_0059a8d0`; a third side needs a third DLL name there. Strategy sounds branch by `side != 1` arithmetic in `FUN_00429020` and `FUN_00428b40`.

## Functions

| Function | Role |
|---|---|
| [FUN_00602840](FUN_00602840.c) | Control sound setter: stores id and module on a control |
| [FUN_005feff0](FUN_005feff0.c) | DLL handle → module id (registry list `DAT_006be4d0`) |
| [FUN_005fefd0](FUN_005fefd0.c), [FUN_005f5920](FUN_005f5920.c) | Module id → DLL handle |
| [FUN_005ff020](FUN_005ff020.c) | Load a DLL and register it as a module id |
| [FUN_00610b20](FUN_00610b20.c) | Sound descriptor constructor |
| [FUN_00610c30](FUN_00610c30.c) | Play a descriptor (→ `FUN_006115f0`) |
| [FUN_006115f0](FUN_006115f0.c), [FUN_00611440](FUN_00611440.c), [FUN_00611890](FUN_00611890.c) | Cache lookup, load by module, `FindResourceA(wave id, "WAVE")` |
| [FUN_00610d00](FUN_00610d00.c) | DirectSound and MCI initialisation |
| [FUN_00403f70](FUN_00403f70.c), [FUN_00403f00](FUN_00403f00.c) | Advisor script sound: `(wave, module)` from script data |
| [FUN_00429020](FUN_00429020.c), [FUN_00428b40](FUN_00428b40.c) | Strategy sounds with side arithmetic |
| [FUN_00422ce0](FUN_00422ce0.c) | Strategy window: `0x259`, `0x25d`, `0x25f` by switch |
| [FUN_00455060](FUN_00455060.c), [FUN_00459e30](FUN_00459e30.c), [FUN_0049f540](FUN_0049f540.c), [FUN_004c4a20](FUN_004c4a20.c), [FUN_00469de0](FUN_00469de0.c) | Literal and record-driven descriptor plays |
| [FUN_005ba980](FUN_005ba980.c), [FUN_005baa60](FUN_005baa60.c), [FUN_005bad50](FUN_005bad50.c) | Tactical event play and variant choice |
| [FUN_005bae60](FUN_005bae60.c) | Tactical sound table (317 events, 351 ids) |
| [FUN_0059a8d0](FUN_0059a8d0.c), [FUN_00595c40](FUN_00595c40.c), [FUN_00599c70](FUN_00599c70.c) | Registers TEXTTACT (3), TACTICAL (6), VOICEFXA/E (14) by side |
| [FUN_0042d650](FUN_0042d650.c) | Registers the side's sprite (9) and briefing (13) DLLs |
| [FUN_00597610](FUN_00597610.c) | Tactical ship class name list |
| [FUN_00417520](FUN_00417520.c), [FUN_00417640](FUN_00417640.c), [FUN_00417610](FUN_00417610.c) | Music start, stop, force |
| [FUN_004173c0](FUN_004173c0.c), [FUN_004176e0](FUN_004176e0.c), [FUN_00417670](FUN_00417670.c) | Music init, switch and volume (registry) |
| [FUN_0041d3b0](FUN_0041d3b0.c), [FUN_004369a0](FUN_004369a0.c), [FUN_00487fb0](FUN_00487fb0.c), [FUN_0043a250](FUN_0043a250.c) | Default track selector |
| [FUN_004c48e0](FUN_004c48e0.c), [FUN_004c2700](FUN_004c2700.c) | Alliance / Empire agent situational track (vtable slot 7) |
| [FUN_00417780](FUN_00417780.c) | End of track: replay a forced track, else the default pick |
| [FUN_0041cd80](FUN_0041cd80.c), [FUN_00439320](FUN_00439320.c) | Session random draw; advice agent constructor (`+0x190 = 0`) |
| [FUN_0044f860](FUN_0044f860.c), [FUN_00450540](FUN_00450540.c), [FUN_0046f360](FUN_0046f360.c), [FUN_00470e70](FUN_00470e70.c), [FUN_00471200](FUN_00471200.c), [FUN_004727a0](FUN_004727a0.c) | Battle Alert (tracks 10/11, its commands stop the music) and result window (12..15, its close picks again) |
| [FUN_00422cb0](FUN_00422cb0.c) | Cockpit restart from the default pick (creation, movie end) |
| [FUN_00421ba0](FUN_00421ba0.c) | Video file switch |

## Open questions

- Which DLL each of the 15 windows' `+0x1c` names for its control sounds (module 7 or 8).
- Writers of the advice agents' `+0x170` (`1, 2, 4, 0x10, 0x20, 0x40`).
- Whether `0x25a`, `0x25b`, `0x261` and `0x1f43` are ever played (computed-id callers of `FUN_00610c30` are not all traced).
- The source of `FUN_00469de0`'s dialog record, and what plays module-13 briefing voices besides scripts (`FUN_0042b1d0`, `FUN_00404440`).
- The family behind each `base + index` tactical event caller.
