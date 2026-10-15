---
title: "Side Switches and the Two-Side Limit"
description: "Every place a side picks its interface DLLs, cockpit art, advisor, voices, movies and side buttons; the structures that hold exactly two sides (session slots, support, the win word, the tactical manager); why owner 3 is never a player; and what a renamed or third side touches"
category: "ghidra"
created: 2026-10-14
updated: 2026-10-14
tags: [side, faction, alliance, empire, interface, advisor, two-side, mod-surface]
---

# Side switches

Answers task-graph node `or-q-mod-factions-z6qn` (epic `or-ep-mod-surface-vdch`), from the Feature Wiki page `executable/mod-factions`; the DLL slot loader, the advisor dispatch and the support store were reread here. Sides are the game's: 1 Alliance, 2 Empire. `hyp:` marks anything not traced through code.

## Answer

1. **The side is one integer, and the test is almost always `== 1` with
   everything else treated as the Empire.** The player's side comes from the
   new-game buttons (command `0x65` Alliance, `0x66` Empire,
   [FUN_00406000](FUN_00406000.c)) and is kept at galaxy
   view `+0x9c`. Interface DLLs, resource id offsets, window colours, droids,
   voices and music are chosen by `side == 1 ? A : B` or `base + (side != 1)`
   (table 1). A value of 3 would silently get the Empire's DLLs and art.
2. **Two playable sides are fixed by structure, not by one constant.** The
   session object holds exactly two player slots (`+0x10`, `+0x14`), built
   by [FUN_00435bc0](FUN_00435bc0.c); the save writes
   exactly two (FUN_00435db0); the strategic support value is one scalar
   (Alliance `v`, Empire `100 - v`, FUN_00507270); the win word has one bit
   pair per side and names four fixed leaders and Coruscant by id
   (FUN_005180e0); the two advisor classes are built by an `if side == 1 /
   if side == 2` pair (FUN_004861b0); and the tactical manager holds two side
   objects (`+8`, `+0xc`).
3. **Owner value 3 (Neutral) is never a playing side.** It is an owner value
   only. `FUN_00435fc0` returns the player side as 1, 2 or 0 and nothing else;
   `FUN_00435bc0` treats any non-1 side argument as the Empire; `FUN_004861b0` would crash on it; the start-up garrison, the class-availability test and the AI branches test 1 and 2 only (tables 2 and 3).
4. **Limits by kind.** Executable tables: the DLL-name strings and
   the slot loader, the movie-id switch, the MDATA music playlist, the
   side-numbered `if` ladders. DAT fields: the per-class `is_alliance` /
   `is_empire` flags, two-column SDPRTB difficulty rows, the SPT advisor
   tables, per-side CMUN*TB seed tables. Resource DLLs: everything drawn,
   voiced or played per side. The count of two is a literal/array-size limit
   in code, never a DAT count.
5. **A third playable side** would have to touch every row of tables 1-4
   (the rows of tables 1 and 2) and the save layout.
   There is no table whose length can simply be raised.

## Table 1: presentation, by switch

Kind: **rid** = resource id offset (`base + (side != 1)` or two literals),
**dll** = DLL name chosen by a literal branch, **branch** = literal
`side == 1` / `== 2` test, **array** = two-entry array or two slots,
**data** = DAT or resource table read by side.

| What | Function (address) | Kind | Alliance / Empire |
|---|---|---|---|
| Sprite DLL (module slot 9): droids, advisor art, action scripts | [FUN_0042d650](FUN_0042d650.c) (`0042d650`; strings `0x6a86e0`, `0x6a86d0`) | dll, `param_1 != 1` | ALSPRITE.DLL / EMSPRITE.DLL |
| Briefing DLL (module slot 0xd): opening tour clips | same; strings `0x6a86c4`, `0x6a86b8`; only when `param_2 != 0` | dll | ALBRIEF.DLL / EMBRIEF.DLL |
| Who calls the DLL loader | [FUN_0041daf0](FUN_0041daf0.c) from FUN_00406000 cases `0x65`/`0x66` with `(1,1)`/`(2,1)`; [FUN_0042adb0](FUN_0042adb0.c) (galaxy view creation) with `+0x9c` | | |
| Side-select buttons | FUN_00406000 `0x65`, `0x66` (`004010a0` stores the side at app `+0x108`, [FUN_004142d0](FUN_004142d0.c)) | branch (command id) | `0x65` / `0x66`; `0x67` is multiplayer |
| Cockpit art, 7 buttons | [FUN_00427270](FUN_00427270.c) (`+0x9c == 1`) | rid | STRATEGY `0x2711..0x271e` / `0x271f..0x272c` |
| Starfield shell | [FUN_00427010](FUN_00427010.c) | rid | `900` / `0x385 = 901`; the backdrop blit origin is `(0x15,0x19)` for side 1 and `(0x54,0x1b)` for any other side (`(side != 1) & 0x3f` and `& 2` added) |
| Window rail sound/zoom | [FUN_00428b40](FUN_00428b40.c), [FUN_00429020](FUN_00429020.c) | rid | `0x263`/`0x265` minus `(side != 1)` |
| GID caption art | [FUN_0042d230](FUN_0042d230.c) | rid | `0x2d3c..0x2d3f` + `(side != 1) * 4`; `0x2d44 + (side != 1)` |
| Game speed menu | [FUN_0042d190](FUN_0042d190.c) | branch | item ids `0x20..0x24` colour `0x020000ff` / `0x25..0x29` colour `0x0200ff00` |
| Window colour `0x20000ff` vs `0x200ff00` | [FUN_00425d00](FUN_00425d00.c) `(-(side != 1) & 0xfe01) + 0x20000ff`; FUN_00459e30 | literal colours | red / green |
| Droid players | [FUN_0042adb0](FUN_0042adb0.c) `+0x9c == 1` | branch | C-3PO + R2-D2 rects / IMP-22 + SD-7 rects (`droid-advisor-triggers.md`) |
| Advisor agent class | [FUN_004861b0](FUN_004861b0.c) `+0x24 == 1` then `== 2`; a third value builds no agent, and the next line writes through the null agent pointer (`*(agent + 8)`), so owner 3 as a player would crash here (read) | branch | [FUN_004c27f0](FUN_004c27f0.c) vtable `0x65c4c8` / [FUN_004c0710](FUN_004c0710.c) vtable `0x65c4a0` |
| Advisor action table | [FUN_004c2c70](FUN_004c2c70.c) / [FUN_004c0ba0](FUN_004c0ba0.c) (strings TEXTSTRA `0x61b` / `0x618`+`0x61c`) | data (SPT file name by class) | GData/C3POACT.SPT / IMP22ACT.SPT |
| Advisor slot maps (message code to slot) | [FUN_004c44b0](FUN_004c44b0.c) / [FUN_004c22f0](FUN_004c22f0.c) (47-entry jump tables) | array (88 / 86 slots) | different slot numbers per code |
| Advisor mood variant | [FUN_004c48e0](FUN_004c48e0.c) / [FUN_004c2700](FUN_004c2700.c) | branch, reversed tables | `1->4..0x40->9` / `1->9..0x40->4` |
| Advice topics | [FUN_0048b2e0](FUN_0048b2e0.c) | rid | TEXTSTRA RCDATA `0x6000` / `0x6800`; picture `0x42f` / `0x430`; sound `0x461` / `0x462` |
| Message pictures and sounds | FUN_0048b2e0 and the message classes (map-05) | branch | e.g. picture `0x412` / `0x413`, sound `0x47c` / `0x47d` |
| Window art | FUN_0044c630 (16 tests), FUN_0046f360 (14), FUN_004665f0 (10), FUN_0044f860 (10), FUN_004637a0 (7), FUN_0045ddc0 (5), FUN_00461960 (4), FUN_0046a9c0 (6), and 20 more windows | rid / literal colour | `+0x9c == 1` tests in 35 functions, 155 tests; bitmap pairs such as `10335` / `10336` (Message Index, Encyclopedia, finders), `0x285f` / `0x2860` |
| System window art by system owner | [FUN_00456230](FUN_00456230.c) (`+0x148` 1, 2, 3) | rid, three-way | Alliance / Empire / Neutral systems; this is the viewed system's owner, not a player |
| Fleet rail icon | `FUN_004a76e0` (`fleet-window.md`) | rid | `11536` / `11537` / `11538` for owner 1 / 2 / 3 |
| Ending code | [FUN_00425c90](FUN_00425c90.c) | branch | viewer 1: EW `0x11`, AW `0x10`; viewer 2: EW `0x12`, AW `0x13` |
| Ending movie | [FUN_00414830](FUN_00414830.c) switch on app state `0x10..0x13` to ids `0x6b, 0x6a, 0x6d, 0x6c` | array (switch) | MDATA.105, .106, .108, .107 |
| Opening movie | FUN_00414830 states `1`, `2` (ids `0x65`, `0x66`) and state `7` (`(side != 1) + 'h'`, ids `0x68` / `0x69`) | branch / rid | id to file by [FUN_00421ba0](FUN_00421ba0.c): `0x65`..`0x74` to MDATA.000, .001, .002, .003, .004, .106, .105, .107, .108, .005, .101, .202, .201, .104, .102, .103 |
| Music | [FUN_00417520](FUN_00417520.c) (track `0..15`, table `0x6a8478` of 16 string pointers MDATA.300..315), [FUN_0041d3b0](FUN_0041d3b0.c) via FUN_004369a0 / FUN_00487fb0 to the advisor's mood: FUN_0043a250 | array (16 tracks) | no side test; `FUN_0041d3b0` asks the player's advisor (`FUN_0043a250`: three calls return `rand(6)/2+1`, then the advisor's mood variant `FUN_004c48e0` / `FUN_004c2700`, `1..9`, reversed by side). hyp: that number is the track index passed to `FUN_00417520`; the callers of `FUN_00417610` (`FUN_0044f860`, `FUN_00470e70`) pick a track directly |
| Tactical voices | [FUN_0059a8d0](FUN_0059a8d0.c) (`FUN_00595c40() == 0`) | dll | VoiceFxA.DLL / VoiceFxE.DLL (module 0xe) |
| Tactical art | FUN_0059be80 (bitmap `1001` when the side is 0), FUN_00595c40 callers (14: FUN_0059c030, FUN_0059c1c0, FUN_005c14d0, FUN_005c1c10, FUN_005c1d30, FUN_005c2990, FUN_005e16a0, FUN_005e23b0, FUN_005e4b40, FUN_005e7b10, FUN_005e9c53, FUN_005e9c60) | rid / branch | tactical side `0` / `1` |
| Faction names in text | TEXTSTRA ids (status window `34648` / `34649`, `status-window.md`); the literals `Alliance`, `Empire`, `Neutral` at `0x6a8f04`, `0x6a8f10`, `0x6a8f1c` are debug strings in FUN_004f8c60 and FUN_005589f0 | data | not a code switch |

Nothing in the executable names a faction beyond those debug literals and
the DLL names: the player-visible name of a side is TEXTSTRA text and
bitmaps. A mod that only renames sides needs TEXTSTRA, the sprite/brief DLLs
and the art ids, and no executable change (hyp: the DLL file names are not
checked against a hash; nothing read verifies them).

## Table 2: where the second side is built in

| Structure | Function (address) | Kind | Detail |
|---|---|---|---|
| Session player slots | [FUN_00435bc0](FUN_00435bc0.c) | array of 2, branch `param_3 == 1` | `+0x10` Alliance, `+0x14` Empire; `+0xc` = the local player; any non-1 side is the Empire. `+0x1c` game mode (1 = computer opponent, [FUN_004857a0](FUN_004857a0.c), else remote player FUN_00484e70) |
| Session teardown | [FUN_00435d40](FUN_00435d40.c) | array | frees `+0x10`, `+0x14` |
| Session type factory | [FUN_00436690](FUN_00436690.c) | array, type 1/2/3 to class | builds a slot object from its saved type, index 0 or 1 |
| Turn alternation | [FUN_00435f70](FUN_00435f70.c) | array index `1 - idx` | `+0x18` flips between slot 0 and 1 |
| Player side query | [FUN_00435fc0](FUN_00435fc0.c), FUN_0041cdb0 | branch | returns 1, 2, else 0 |
| Opponent side | FUN_00435bc0 | branch | the computer player gets 2 when the human is 1, else 1 |
| Side-object id counter | [FUN_004884f0](FUN_004884f0.c) | branch | `1` for side 1, `0x77359401` otherwise: the object-id spaces of the two sides |
| Support scalar | [FUN_00507270](FUN_00507270.c), FUN_00509c00 | branch | system `+0x58` is the Alliance's share; side 1 reads `v`, any other side reads `100 - v`. Two sides share one value; there is no per-side support field |
| Start-up garrison | [FUN_0051ab20](FUN_0051ab20.c) | branch | holder 1 gets `0x10000002` regiments, 2 gets `0x10000008`; owner 3 gets none |
| Start-up steps | `FUN_0051aa50` steps 5 and 6 (`starting-forces-seeding.md`) | data | CMUNAFTB (Alliance fleets) and CMUNEFTB (Empire fleet); CMUNALTB / CMUNEMTB for step 7, `FUN_0051b1f0` called for side 1 then side 2 |
| Difficulty and size rows | SDPRTB `5168..5170`, `7680`, `7681`, `0x1430 + size - 1` | data, two columns | one value per side (Alliance / Empire) |
| Core-system dealing | `FUN_00519d00` | branch (fixed order) | Alliance strong, Alliance weak, Empire strong, Empire weak, then neutral; Coruscant `0x90000109` is Empire's, Yavin `0x92000121` and the HQ are set aside |
| Class availability | [FUN_00567880](FUN_00567880.c) | branch, DAT field | `side == 1` reads class `+0x40` (`is_alliance`), `side == 2` reads `+0x44` (`is_empire`); the guard `param_2 < 1 || 2 < param_2` rejects any other side |
| Win word | [FUN_005180e0](FUN_005180e0.c) | literal ids | `0x32000242` Luke, `0x30000240` Mon Mothma, `0x35000281` Vader, `0x34000280` Palpatine, `0x90000109` Coruscant; Alliance HQ = first family `0x20`-`0x21` object; bit pairs AD/LC/MC and DC/EC/CA/CD, EW `0x100000`, AW `0x1000000` (`victory-conditions.md`) |
| Win word to side | `FUN_004c3010` / `FUN_004c0f10` (advisor slots 87, 86 / 84, 85), `FUN_00485ea0` -> `FUN_004189b0` (AI progress counters) | array / branch | one slot pair per side |
| Game type | root `+0x60` (0 standard, 1 headquarters only) | data (UI) | rule is "Empire wins by AD", "Alliance wins by CA or CD" |
| AI context | [FUN_00417cb0](FUN_00417cb0.c) | array | writes the side into 10 sub-objects at `+0`, `+0x34`, `+0x4c`, `+0x60`, `+0x80`, `+0x94`, `+0xb4`, `+0xf8`, `+0xe4`, `+0x128` (words 0, 0xd, 0x13, 0x18, 0x20, 0x25, 0x2d, 0x3e, 0x39, 0x4a) |
| AI modules | FUN_004bf320 / FUN_004bf3e0 / FUN_004bf480 | branch `+0x30 == 1` | module 13 (Scrap and Move of `0x20000005`, the Alliance HQ) does something only for the Alliance; the Empire path completes |
| AI auto-namer | [FUN_004d2b30](FUN_004d2b30.c) | branch `+0x30 == 1`, TEXTSTRA id pools | Alliance `0x5200+n` (n < 20), `0x52c0+n`, `0x5260+n`; Empire `0x5100+n` (n <= 0x26) and further pools; ids past the table's end are absent |
| AI mission table | FUN_004e8290 row 8 `this+4 == 1`, FUN_004eaaa0 `this+4 == 2`, FUN_004ead50 | branch | Death Star sabotage proposals are Alliance only; kind 6 proposal is Empire only |
| AI proposal side | FUN_004b7150 | branch `this+4 != 1` | writes `0x62` (Alliance) / `0x63` (Empire) |
| AI records | FUN_00401d20, FUN_00433e40 | branch | `rec+0x1c` and `ctx+0x1c` are compared with 1 and 2; owner bits compared with 1 and 2 |
| Tactical sides | `FUN_005aaf90`, FUN_005af5f0, FUN_005bf830 | array of 2 | manager `+8` Alliance (vtable `0x0066c7a0`, facing +z), `+0xc` Empire (`0x0066c778`, -z); played battle vtables `0x0066c428` / `0x0066c3f8`; local side at `+0x9ec`, set by [FUN_00599c90](FUN_00599c90.c) from [FUN_0040d660](FUN_0040d660.c) (`side != 1`) |
| Tactical AI | `ComputeStrength` (`005c7f70`), `CanRetreat` (`005c7e80`), `005c8780` | branch on side id | Empire gets the Death Star bonus; Alliance only in two cases; trench run (order 6) Alliance only; voice ids `0x87..0x8e` Alliance, `0x105..0x10c` Empire |

## Table 3: owner value 3 (Neutral)

Neutral is a holder value, never a player:

- Names: FUN_004f8c60 / [FUN_004f88d0](FUN_004f88d0.c) print 3 as "Neutral"; FUN_005589f0 does so for bits 4-5, 2-3 and 0-1 of an event mask.
- FUN_004f6f40 accepts owner values 1 to 3 (`side-numbering.md`); `FUN_005404d0` treats mask value 3 as "either side".
- The session never creates a player for 3: FUN_00435fc0 returns 0, FUN_00435bc0 maps it to the Empire slot, FUN_004861b0 builds no advisor, and FUN_00567880 rejects it.
- Fleet rail icon `11538` exists for owner 3 and `FUN_00456230` has a Neutral branch for system art: these are owner art, so a Neutral-owned object can already be drawn.
- Blockade, informant, support and uprising code treat 3 as "no side" (`blockade-bit.md`, `uprising-incident.md`).

## What a third playable side would have to touch

All of these, hyp where noted:

1. Side-select: a third button command in FUN_00406000 and a new
   `FUN_004142d0` store (only `+0x108` 1 or 2 is meaningful).
2. DLL loader: FUN_0042d650 becomes a three-way choice; two more DLL name
   strings and slots (the loader maps every non-1 side to the Empire).
3. Every `+0x9c == 1` test and every `(side != 1)` offset: about 155 tests in
   35 functions (table 1), plus the same tests in `FUN_00425d00`,
   `FUN_0042b330`, FUN_00427270. Each needs a third art id range in
   STRATEGY/COMMON and a third colour.
4. Session: a third slot beyond `+0x10`/`+0x14` (the session object is a
   fixed layout and `FUN_00435db0` / `FUN_00435ec0` read and write exactly two
   slots, so the save format changes).
5. Advisor: a third agent class and SPT table beside FUN_004861b0's pair
   (the agent array has 88 and 86 slots; the third class would need its own
   message-code switch, 47 entries).
6. Support: `+0x58` stores only the Alliance's share, so "three-way support"
   needs a new field in the system record (and in `FUN_00509a20` counters,
   which already carry three per-side counters `A[1..3]` where 3 is shared).
7. Win word: the bit layout in FUN_005180e0 and FUN_00530a50 is
   Alliance-vs-Empire, and the ending movie switch has four cases.
8. DAT: each class record carries only `is_alliance` / `is_empire`; a third
   flag has no field (hyp: the record layout is fixed-size in the DAT files).
9. AI: `FUN_004857a0` builds a computer player from one side parameter and
   the AI modules have literal `== 1` and `== 2` branches; a third AI is a
   third `FUN_00417cb0` context (hyp: the AI would work if the side is
   written correctly, since most AI code uses ctx fields without testing
   the side).
10. Tactical: two side objects; the singleton offsets `+8`, `+0xc` and the
    side-id-from-`+0x9ec` logic in about 30 functions (map-18) assume two.

## Evidence

- Side select, DLL load, interface art: `FUN_00406000` -> `FUN_004010a0` ->
  `FUN_004142d0` (app `+0x108`); `FUN_0041daf0` -> `FUN_0042d650`.
- Player build: `FUN_00435a50` -> `FUN_00435bc0` -> `FUN_004861b0` ->
  `FUN_004c27f0` / `FUN_004c0710`.
- Movies: `FUN_00414330` states `0x10..0x13` -> `FUN_00414830` ->
  `FUN_004146e0` -> `FUN_004213e0`; `FUN_00421ba0` maps ids to MDATA files.
- Music: `FUN_004173c0` (registry `MusicSwitch`), `FUN_00417520`,
  `FUN_00417780` (end-of-track event `0x4c9`), `FUN_0041d3b0`, `FUN_00487fb0`,
  `FUN_0043a250`, `FUN_00439bf0` (mood byte `+0x170`).
- Counts: a script over the decompiles found `+0x9c ==/!= 1|2` in 35 functions
  (155 tests), owner-bit compares (`>> 6 & 3`) in 66 functions (114 tests) and
  `FUN_00595c40()` in 14 functions.

## Mod surface

For a mod that adds an era overhaul to REBEXE (Old Republic, New Republic, New
Jedi Order):

- **Rename and reskin both sides: no code change.** Replace TEXTSTRA ids,
  ALSPRITE/EMSPRITE/ALBRIEF/EMBRIEF/VoiceFxA/VoiceFxE, STRATEGY/COMMON bitmaps,
  the `C3POACT.SPT` / `IMP22ACT.SPT` action tables and the MDATA files.
  `FUN_0042d650`, `FUN_00414830` and `FUN_00421ba0` keep the file names.
- **Rename the leaders or the win conditions: patch code.** FUN_005180e0 holds
  five object ids as literals and the ending movie switch is a four-way
  `switch`.
- **A third playable side: patch about ten structures** (the list above). The
  cheapest variant is a third *non-playing* faction using owner 3: it needs
  art (the `11538` and Neutral system art paths exist) and DAT rows, and no
  session change.
- **Hard limits to fix first:** session slot count (`FUN_00435bc0`, the save
  pair `FUN_00435db0` / `FUN_00435ec0`), the support scalar (`FUN_00507270`), the
  DLL loader (`FUN_0042d650`), the advisor factory (`FUN_004861b0`).

### Proposed follow-up questions

1. How does the save record each side (`FUN_00435db0`, `FUN_00435ec0`,
   `FUN_0048a960`, `FUN_0048a9f0`): does a slot store its side, or is the side
   implied by position, and what is the file's per-side layout?
2. Which window colours, sounds and bitmap ids does each of the 35 `+0x9c`
   functions hold, as a per-side resource table the port can read from a
   data file? Start from `FUN_0044c630`, `FUN_0046f360`, `FUN_004665f0`.
3. Where is the music track picked from the advisor mood
   (`FUN_0043a250` -> `FUN_004c48e0`/`FUN_004c2700`) and does the game use
   side-specific tracks at all? Start from `FUN_00417520`, `FUN_00417610`
   (callers `FUN_0044f860`, `FUN_00470e70`).
4. What is each class record's DAT layout for `is_alliance` / `is_empire`
   (`FUN_00567880`, `FUN_0053b860`), and which classes read them?
5. Which AI side-object fields are not side-specific (FUN_00417cb0 context
   words) and can a third computer player run them? Start from `FUN_004178f0`,
   `FUN_004bee50`, `FUN_004be7e0`.

## Open questions

- Where the galaxy view's `+0x9c` is written (open in `side-numbering.md`);
  hyp: the galaxy view constructor receives it from the strategic window.
- Whether the music track is picked at all by side (the advisor mood tables
  differ by side; nothing else read).
- The save layout of the side objects (question 1 above).
- Which of the 35 `+0x9c` functions are reachable by a Neutral viewer (none
  traced).

## Functions

| Function | Address | Role |
|---|---|---|
| FUN_0042d650 | 0042d650 | loads sprite (slot 9) and briefing (slot 0xd) DLL by side |
| FUN_0041daf0 | 0041daf0 | forwarder to FUN_0042d650 |
| FUN_00406000 | 00406000 | main-menu command switch (0x65 Alliance, 0x66 Empire) |
| FUN_004142d0 | 004142d0 | stores side/size/game type in the application |
| FUN_00414830 | 00414830 | opening and ending movie ids |
| FUN_00421ba0 | 00421ba0 | movie id to MDATA file |
| FUN_00435bc0 | 00435bc0 | builds the two session player slots |
| FUN_00435fc0 | 00435fc0 | player side 1, 2 or 0 |
| FUN_004861b0 | 004861b0 | per-side player object and advisor agent |
| FUN_004857a0 | 004857a0 | computer player (side argument) |
| FUN_004884f0 | 004884f0 | side object with an id counter per side |
| FUN_0042adb0 | 0042adb0 | galaxy view droids by side |
| FUN_00427010 | 00427010 | starfield and shell art by side |
| FUN_00456230 | 00456230 | system window art by owner 1, 2, 3 |
| FUN_00425c90 | 00425c90 | ending code by viewer side |
| FUN_00417520 | 00417520 | plays music track 0..15 |
| FUN_0041d3b0 | 0041d3b0 | track number from the advisor |
| FUN_004c48e0, FUN_004c2700 | 004c48e0, 004c2700 | advisor mood to number (reversed by side) |
| FUN_0059a8d0 | 0059a8d0 | tactical window: VoiceFxA or VoiceFxE |
| FUN_00507270 | 00507270 | support for side 1 is `v`, else `100 - v` |
| FUN_0051ab20 | 0051ab20 | start-up garrisons by holder |
| FUN_00567880 | 00567880 | classes buildable by side 1 or 2 (DAT flags) |
| FUN_005180e0 | 005180e0 | win word with fixed object ids |
| FUN_004d2b30 | 004d2b30 | AI ship names by side |
| FUN_00417cb0 | 00417cb0 | AI context: side written into ten sub-objects |
| FUN_00599c90 | 00599c90 | tactical manager side `+0x9ec` |
| FUN_0040d660 | 0040d660 | tactical side `side != 1` |

Decompiles are under `../assets/decompiles/` (those linked above).
