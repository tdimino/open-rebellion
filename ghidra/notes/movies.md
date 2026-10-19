---
title: "Movies: Which Events Play Which MDATA File, and the Return Path"
description: "The one movie player, the closed id-to-file switch FUN_00421ba0, the app-state and message-class triggers for every MDATA movie, the trench-run pair, the return routing, and what a mod changes to add or replace a movie"
category: "ghidra"
created: 2026-10-14
updated: 2026-10-18
tags: [movies, smacker, mdata, endings, death-star, messages, mod-surface]
---

# Movies

Answers task-graph node `or-q-mod-movies-ftl8` (epic `or-ep-mod-surface-vdch`),
from the Feature Wiki page `executable/mod-movies`; the movie switch, the
app-state picks and the Coruscant trigger were reread here. Side numbers are
the game's: 1 Alliance, 2 Empire. Anything not traced through code is marked
`hyp:`.

## Answer

**The mapping is code, in four places, and one id-to-file switch. There is no
movie table in the executable and no DAT field that names a movie.**

1. **One player, three ways in.** The movie window class (`MOVIE_WINDOW`,
   `FUN_004213e0`) is only ever created by `FUN_004146e0` (the application
   window's `+0x1c4` slot). `FUN_004146e0` has exactly two callers:
   - `FUN_00414830` (twice), which picks an id from the application state
     machine's state at `+0xdc`: intro, faction start, the four endings;
   - the thunk `FUN_004019a0`, which has four callers: the main menu's Credits
     button (`FUN_00406000`), the strategic window's `0x482` handler
     (`FUN_00422ce0`, both for message movies) and the two tactical
     trench-run result wrappers (`FUN_0059cc60`, `FUN_0059cc80`).
2. **Id to file is a `switch`.** `FUN_00421ba0` maps ids `0x65`-`0x74` to 15
   file-name strings (`MDATA\MDATA.000` and so on; id `0x67`, MDATA.002, has a
   string but no file ships). The path is the registry value `TooBigLocation`
   under `SOFTWARE\LucasArts Entertainment\Star Wars Rebellion` plus that
   string (`FUN_004213e0`).
3. **In-game story-style movies are not story code.** Four message classes
   write a movie id into field `+0x5c` of the game message they raise:
   `0x72` (event `0x305`), `0x73` and `0x74` (event `0x128`) and `0x6f` (event
   `0x182`). That is MDATA.104, .102, .103 and .101. The message pump plays
   any kind-6 message's `+0x5c` (`FUN_00488b90` → `FUN_0041d850` →
   `FUN_00425ce0` → message `0x482` wParam 1 → `FUN_00422ce0` →
   `FUN_004019a0`).
   Message `0x482` (like `0x468`, the window-open message) first clears the
   screen of mouse capture and modal windows: it returns at once while the step
   counter `FUN_004fcee0` is below 2, then loops on `FUN_00601340`, which sends
   `0x40d` to the `GetCapture` window or, with none, to the modal stack's top
   (`FUN_00601320`) and reports done when there was none to send it to
   (`FUN_00422ce0.c:1192-1232`, `FUN_00601340.c`). Each pass dispatches the
   pending messages and runs the frame, so the game keeps running while it
   waits. `0x40d` closes a window only where its procedure maps it to slot
   `+0x30` (the Status window `FUN_0044b250`, the Encyclopedia `FUN_0045d8f0`,
   `FUN_00467e80`); the base procedure `FUN_006083c0` and the sector window
   `FUN_004593e0` release the capture and nothing more, and the Operation Report
   (`FUN_00606650`) has no `0x40d` case, so the movie waits for it to be closed
   (`operation-report-window.md`, section 6). With wParam 1 the message is then
   `FUN_00610c00(0)` and `FUN_004019a0(message +0x5c, message +0x50, 0)`; with
   wParam 0 `FUN_00401040(ending code)`.
4. **The Luke, Dagobah, Vader, Han chains play no movie.** None of the
   `story-missions.md` functions reaches a movie caller.
5. **Return.** The Smacker frame pump (`FUN_005f7240`) sends `0x402` to the
   movie window at the last frame, `0x404` on Esc or left button, and the
   open call (`FUN_005f6920`) posts `0x403` when the path is unknown. All three
   close the window and post `0x405` to the application window
   (`FUN_004138a0`), which routes by movie id (table below).

## Callers of the movie player

| # | Trigger | Caller | State / path | Movie id | File |
|---|---|---|---|---|---|
| 1 | Start of the program, no session flag (`FUN_00597480` = 0) | `FUN_004138a0` (first activate) → `FUN_00414330(1)` → `FUN_00414830` case 1 | state 1 | `0x65` | MDATA.000 |
| 2 | End of movie `0x65` | `FUN_004138a0` `0x405` → `FUN_00414330(2)` → `FUN_00414830` case 2 | state 2 | `0x66` | MDATA.001 |
| 3 | Menu button `0x65` (start as Alliance) | `FUN_00406000` → `FUN_004010a0(1, ...)` → `FUN_00401040(7)` → `FUN_00414830` case 7 | state 7, side `+0x108` = 1 | `0x68` | MDATA.003 |
| 4 | Menu button `0x66` (start as Empire) | same, side 2 | state 7, side 2 | `0x69` | MDATA.004 |
| 5 | Multiplayer lobby start | `FUN_004104b0` → `FUN_004010a0(side, 2, ...)` → `FUN_00401040(7)` | state 7, the lobby's side at `+0x120` | `0x68` or `0x69` | MDATA.003 or .004 |
| 6 | Menu button `0x73` (LucasArts logo, Credits per `main-menu-parity.md`) | `FUN_00406000` case `0x73` → `FUN_004019a0(0x6e, 0, FUN_00406520)` | movie window, callback restores the menu | `0x6e` | MDATA.005 |
| 7 | Ending, Alliance viewer, Alliance wins (code `0x10`) | `FUN_00414330(0x10)` → `FUN_00414830` | state `0x10` | `0x6b` | MDATA.105 |
| 8 | Ending, Alliance viewer, Empire wins (`0x11`) | same | state `0x11` | `0x6a` | MDATA.106 |
| 9 | Ending, Empire viewer, Empire wins (`0x12`) | same | state `0x12` | `0x6d` | MDATA.108 |
| 10 | Ending, Empire viewer, Alliance wins (`0x13`) | same | state `0x13` | `0x6c` | MDATA.107 |
| 11 | Message raised by event `0x305` (sabotage destroyed): object family `0x18`-`0x1b` in a system (`0x90`-`0x97`); title TEXTSTRA `0x71bd` "Death Star Sabotaged", body `0x71be` "The Rebel Alliance has sabotaged the Death Star at \|." (the system's name) | `FUN_0048d470` sets `+0x5c` | pump → `FUN_004019a0` | `0x72` | MDATA.104 |
| 12 | Message raised by event `0x128` with the AD bit (Alliance HQ missing or destroyed): TEXTSTRA `0x7188`/`0x7189` | `FUN_0048e5a0` | pump | `0x73` | MDATA.102 |
| 13 | Message raised by event `0x128` while Coruscant (`0x90000109`) is Alliance-owned: TEXTSTRA `0x7170`/`0x7171` | `FUN_0048f790` | pump | `0x74` | MDATA.103 |
| 14 | Message raised by event `0x182` (FleetBombard): the fleet holds a child of family `0x18`-`0x1b`; title TEXTSTRA `0x70c8` "\| Destroyed!", body `0x70c9` "The Death Star has destroyed the \| system \|." or `0x70ca` "The Death Star has destroyed the neutral system \|." | `FUN_004950d0` | pump | `0x6f` | MDATA.101 |
| 15 | Tactical trench run succeeded (result state 6) | `FUN_0059cc60` | `FUN_004019a0`, callback 0 | `0x71` | MDATA.201 |
| 16 | Tactical trench run failed (result state 7) | `FUN_0059cc80` | same | `0x70` | MDATA.202 |

Ids no caller passes: `0x67` (MDATA.002, no file in `MDATA/`; the `0x405`
handler would route it to state 7). The 16 `MDATA.300`-`.315` files are not
movies: `FUN_00417520` indexes their names as music cues
(`PTR_s_MDATA_300_006a8478`).

### Both sides

- Rows 1-2, 6: no side. Rows 3-4 and 7-10 branch by side in code (state 7 by
  `+0x108`; the ending state by viewer and winner). Row 5 takes the side from
  the lobby.
- Rows 11-14 are raised for either side's message manager (`+0x20` is the
  side). Row 12 (`FUN_0048e5a0`) reads no side: both sides get the same
  message and movie. Row 13 (`FUN_0048f790`) tests Coruscant's owner, not the
  viewer; the advice code `+0x28` is `(side != 1) + 1`. Row 11 tests no
  side: it looks the system (the event's last argument, family `0x90`-`0x97`)
  up in the side's view (`FUN_004f2d10(side, system)`) and names it in the
  body, so both sides file it, whichever side sabotaged. hyp: every side's
  view holds every system.
  Row 14 reads the fleet through the side's view (`FUN_004f36a0`); the body
  text depends on the owner (`0x70c9` owned, `0x70ca` neutral).
- Only the local human's pump plays them: `FUN_00486520` runs for the local
  player object (`FUN_004861b0`); hyp: the computer player never calls
  `FUN_00488b90`.
- Rows 15-16: the wrappers have no side test. hyp: the Empire player's
  battle against an Alliance trench run uses the same result wrappers; not
  traced.

## The ending path in one line

`FUN_005180e0` win word (`0x128`) → advisor slot 84-87 → `FUN_00425c90` stores
the code at strategic window `+0xa8` → `FUN_0042db70` posts `0x482` wParam 0 →
`FUN_00422ce0` calls `FUN_00401040(code)` → `FUN_00414330` state `0x10`-`0x13`
(teardown) → `FUN_00414830` → `FUN_004146e0`. Details are in
`victory-conditions.md` and `game-end-path.md`; row 12 shows that the AD bit
alone also raises a movie (MDATA.102) before the ending, and row 13 does the
same for Coruscant.

## How the app state machine reaches the player

`FUN_00414330(state)` (`this` is the application object) dispatches through a
byte table at `0x004146c4` (index `state - 1`) into a 12-entry address table
at `0x00414694`. States that run `FUN_00414830` are `1`, `2`, `7` and
`0x10`-`0x13` (the last four after tearing the game down). `FUN_00414830` is a
`switch` on `+0xdc`:

| `+0xdc` | Movie |
|---|---|
| 1 | `0x65` |
| 2 | `0x66` |
| 7 | `(+0x108 != 1) + 0x68`: `0x68` for side 1, `0x69` for side 2. Before the movie it fills the settings record and `DAT_006b1208` and builds the game session: `FUN_00401b90` → `FUN_00409fe0` → `FUN_0040adb0` makes the session object (`DAT_006b1218`, `FUN_0041dd20`) and hands it the settings (`FUN_0041df40`; hyp: the galaxy is seeded there) |
| `0x10` / `0x11` / `0x12` / `0x13` | `0x6b` / `0x6a` / `0x6d` / `0x6c` |
| other | none |

## The return path

`FUN_00421790` is the movie window's procedure (`this+0x94` = `0xf` is its
window code, `this+0x98` the movie id, `this+0xb4` an optional callback):

| Message | Source | Effect |
|---|---|---|
| `0x402` | `FUN_005f7240` at the last Smacker frame (`SendMessageA` to the window's owner) | close window; post `0x405` to the application window; call the callback with message 0 |
| `0x403` | `FUN_005f6920` when `FUN_005f6be0` cannot open the path ("PathName unknown to VideoManager") | same as `0x402` |
| `0x404` | `FUN_005f7240` when Esc or the left button is down while waiting for a frame | close window; post `0x405`; callback with message `0x405` |
| `WM_COMMAND` `0xf1` | not traced (hyp: a menu or accelerator) | close window; post `0x405` |
| `WM_KEYDOWN` other than Esc | keyboard | ignored for ids `0x70` and `0x71` (the trench movies); otherwise close and post `0x405` |
| `WM_KEYDOWN` Esc | keyboard | posts message 0 to the application window and calls the callback with message 0: no close (the `0x404` poll in `FUN_005f7240` is what skips) |

`0x405` reaches the application window procedure `FUN_004138a0` with wParam
`0xf` and the movie id in lParam (`0x465` has a near-identical body that sends
`0x65`, `0x66` and `0x6a`-`0x6d` to state 3 and `0x67`-`0x69` to state 8; who
posts `0x465` is not traced):

| Id | `0x405` handler | Next |
|---|---|---|
| `0x65` | `FUN_00414330(2)` | movie `0x66` |
| `0x66`, `0x6a`-`0x6d` | `ShowCursor(1)`, `FUN_00414330(3)` | main menu (`FUN_00414a80`), campaign fields cleared (state 3 path, `game-end-path.md`) |
| `0x67` | `FUN_00414330(7)` | movie `0x68`/`0x69` (hyp: unreachable) |
| `0x68`, `0x69` | `FUN_00414330(8)` | `FUN_00414eb0`: start the game (`+0xfc` = 1) |
| any other (`0x6e`-`0x74`) | posts `0x402` (wParam 0, lParam id) to the current child window at `+0xc0` | the child decides |

The child for message movies (rows 11-14) is the strategic window:
`FUN_00422ce0` case `0x402` runs only when `+0x498` is set (the message pointer
`FUN_00425ce0` stored), removes that message (`FUN_0041d330`), restores the
cursor and palette and resumes the strategic view. The credits movie (row 6)
returns through its callback `FUN_00406520`, which repaints the menu and
restarts the menu music. The trench movies (rows 15-16) return to the
paused tactical view (`tactical-death-star-path.md`); hyp: through the same
`0x402` to the tactical window.

## Functions

| Function | Address | Role |
|---|---|---|
| [`FUN_00414830`](FUN_00414830.c) | `0x00414830` | state to movie id for states 1, 2, 7, `0x10`-`0x13`; calls `FUN_004146e0` |
| [`FUN_00414330`](FUN_00414330.c) | `0x00414330` | application state machine; tables at `0x004146c4`, `0x00414694` |
| [`FUN_004146e0`](FUN_004146e0.c) | `0x004146e0` | the movie player: destroys the old window, builds `MOVIE_WINDOW`, shows it |
| [`FUN_004019a0`](FUN_004019a0.c) | `0x004019a0` | thunk to `FUN_004146e0` when the application object exists (`DAT_006be3b8`) |
| [`FUN_00401040`](FUN_00401040.c) | `0x00401040` | thunk to `FUN_00414330` (set state) |
| [`FUN_004213e0`](FUN_004213e0.c) | `0x004213e0` | movie window constructor: registry `TooBigLocation` + `FUN_00421ba0` name |
| [`FUN_00421ba0`](FUN_00421ba0.c) | `0x00421ba0` | movie id `0x65`-`0x74` to file-name string |
| [`FUN_00421790`](FUN_00421790.c) | `0x00421790` | movie window procedure: `0x402`-`0x404`, `0x111`, `0x100` |
| [`FUN_00421300`](FUN_00421300.c), [`FUN_005f6920`](FUN_005f6920.c) | `0x00421300`, `0x005f6920` | start Smacker playback; `0x403` on a bad path |
| [`FUN_005f7240`](FUN_005f7240.c) | `0x005f7240` | per-frame Smacker pump; sends `0x402` at the end, `0x404` on Esc or click |
| [`FUN_004138a0`](FUN_004138a0.c) | `0x004138a0` | application window procedure; `0x405` and `0x465` routing |
| [`FUN_00406000`](FUN_00406000.c) | `0x00406000` | main menu `WM_COMMAND`: faction start, Credits movie |
| [`FUN_00406520`](FUN_00406520.c) | `0x00406520` | Credits movie callback (restore the menu) |
| [`FUN_00488b90`](FUN_00488b90.c) | `0x00488b90` | message pump: plays the head of the movie list (`+0x28`) |
| [`FUN_00488b50`](FUN_00488b50.c), [`FUN_0048a060`](FUN_0048a060.c) | `0x00488b50`, `0x0048a060` | event to message, then file by kind; kind 6 goes to the `+0x28` list |
| [`FUN_0041d850`](FUN_0041d850.c), [`FUN_00425ce0`](FUN_00425ce0.c) | `0x0041d850`, `0x00425ce0` | store the message at `+0x498`, post `0x482` wParam 1 |
| [`FUN_0041db60`](FUN_0041db60.c), [`FUN_0042db70`](FUN_0042db70.c) | `0x0041db60`, `0x0042db70` | post `0x482` wParam 0 with the ending code |
| [`FUN_00422ce0`](FUN_00422ce0.c) | `0x00422ce0` | strategic window: `0x482` (ending code to `FUN_00401040`, or message movie to `FUN_004019a0`), `0x402` return |
| [`FUN_0048d470`](FUN_0048d470.c) | `0x0048d470` | sabotage message class: movie `0x72` |
| [`FUN_0048e5a0`](FUN_0048e5a0.c) | `0x0048e5a0` | HQ destroyed message class: movie `0x73` |
| [`FUN_0048f790`](FUN_0048f790.c) | `0x0048f790` | Coruscant captured message class: movie `0x74` |
| [`FUN_004950d0`](FUN_004950d0.c) | `0x004950d0` | Death Star system destroyed message class: movie `0x6f` |
| [`FUN_0048d640`](FUN_0048d640.c), [`FUN_004953a0`](FUN_004953a0.c) | `0x0048d640`, `0x004953a0` | the kind getters (return 6) of those four classes |
| [`FUN_0059cc60`](FUN_0059cc60.c), [`FUN_0059cc80`](FUN_0059cc80.c) | `0x0059cc60`, `0x0059cc80` | tactical trench-run results: movies `0x71`, `0x70` |
| [`FUN_004104b0`](FUN_004104b0.c) | `0x004104b0` | multiplayer lobby start: state 7 |
| [`FUN_00401b90`](FUN_00401b90.c), [`FUN_00409fe0`](FUN_00409fe0.c), [`FUN_0040adb0`](FUN_0040adb0.c) | `0x00401b90`, `0x00409fe0`, `0x0040adb0` | state 7: build the game session from the settings record before the side's movie |
| [`FUN_00414eb0`](FUN_00414eb0.c) | `0x00414eb0` | state 8: build and show the strategic window (`FUN_00421c70`) |

## Evidence

- **Callers are complete for the two entry points.** Ghidra's caller lists:
  `FUN_004146e0` ← `FUN_004019a0`, `FUN_00414830` ×2; `FUN_004019a0` ←
  `FUN_00406000`, `FUN_00422ce0`, `FUN_0059cc60`, `FUN_0059cc80`;
  `FUN_00414830` ← `FUN_00414330` ×2; `FUN_004213e0` ← `FUN_004146e0` only;
  `FUN_00421ba0` ← `FUN_004213e0` only. A reference search for the movie
  window's direct constructor found no other creator. hyp: a call through a
  vtable would not show in these lists; the movie window vtable
  (`0x00658bb0`) has no other creator in the data references checked.
- **Message movies are exactly four.** Instruction search for `MOV [reg+0x5c],
  imm` over `0x00480000`-`0x004a0000` finds `0x6f`, `0x72`, `0x73`, `0x74`
  (the other `0x5c` stores belong to unrelated classes, for example
  `FUN_0049a1b0`, which stores an order object). Each of the four classes'
  slot 4 (`+0x10`) is a stub returning 6 (`FUN_0048d640`, `FUN_004953a0`);
  `FUN_0048a060` files kind 6 into the list at `+0x28` and the pump
  (`FUN_00488b90`) plays the head through `FUN_0041d850`. Other classes also
  share the `FUN_0048d640` stub in slot 4 of unrelated vtables
  (`0x0065bb94`, `0x0065c1cc`, `0x0065c9ec`, `0x0066c92c`); hyp: those are
  not message classes (their slot 5 differs).
- **Ids.** Movie ids `0x6e`-`0x74` appear as plain immediates in many
  unrelated places (control ids); only the sites in the table pass them to the
  player.
- **File names.** `FUN_00421ba0` case list read from the decompile; file
  list from `MDATA/` on disk (no `MDATA.002`).
- **Return codes.** `FUN_005f7240`: `SendMessageA(owner, 0x402)` after the
  last frame, `SendMessageA(owner, 0x404)` on `GetAsyncKeyState` of Esc or the
  left button; `FUN_005f6920`: `PostMessageA(owner, 0x403)`. The `& 0x10000000`
  test on a `short` in the decompile is a decompiler artefact (hyp: it tests
  the key-down bit `0x8000`).

## Mod surface

A mod adding this kind of content (a movie for a new event) must change, in
REBEXE:

1. **Nothing, to replace a movie.** Keep the file name: the path is
   `TooBigLocation` + `MDATA\MDATA.xxx`, built per play (`FUN_004213e0`).
   Smacker 2 only (`smackw32.DLL`).
2. **`FUN_00421ba0`, to add a movie id.** The ids are a closed switch,
   `0x65`-`0x74`; a new id needs a new case, a new string for the file name
   (the strings sit in `.rdata` at `0x006a85b8`-`0x006a8698`), and nothing
   else in the player. A table or a DAT field does not exist, so DAT edits
   alone cannot add a movie.
3. **A trigger, in code.** Pick one:
   - a message class that writes `+0x5c` (copy `FUN_0048e5a0`, the smallest:
     it needs a vtable with the kind-6 getter, a handler for one event, two
     TEXTSTRA ids, a constructor and a registry entry in the class table that
     `FUN_00436a40` builds, plus the save and load slots `FUN_0048e580`-style,
     because `+0x5c` is serialised);
   - or a direct call to `FUN_004019a0(id, 0, callback)`, as the Credits
     button and the trench wrappers do.
4. **A return handler**, unless the id is in `FUN_004138a0`'s `0x405` switch:
   a new id falls into the default branch and is delivered as `0x402` to the
   current child window; the strategic window already handles it for message
   movies (it removes the stored message), a new window needs its own
   `0x402` case or a callback.
5. **Endings** are a fixed 4-way code table: `FUN_00414830` states `0x10`-`0x13`
   and the codes in `FUN_00425c90`. A third ending needs a new state in
   `FUN_00414330`'s tables, a new `FUN_00414830` case, a new `0x405` row and a
   new win-word bit in `FUN_005180e0` (`victory-conditions.md`).
6. **Port.** `crates/rebellion-app/src/movies.rs` holds the id-to-file
   switch, the state 7 pick (`side_movie`), the `0x405` routing with the
   next state's movie (`after`, `Scene`), the skip of a movie that cannot
   open (`after_movie`, the `0x403` path), the ending codes and movies, and
   the queue of kind-6 message movies. In `main.rs`, `CutsceneKind::ended`
   routes every movie's end through `after_movie`: the program starts on
   row 1 and chains row 2, then the main menu; a new game is seeded first
   and then plays row 3 or 4 by the player's side, then the galaxy; the
   Credits control plays row 6 and returns to the menu; a `Child` movie
   returns to the window it played over (the galaxy, the tactical view, the
   menu). The browser plays each through `or_movie_play`, `or_movie_stop` and
   `or_movie_ended` (`crates/rebellion-render/src/video_player.rs`,
   `web/index.html`). `strike_orders.rs` builds rows 11-14
   (`sabotage_movies`, `word_movies`, `strike_movies`), each playing with
   its body as the caption and filing no Message Index row and no advice.
   Row 11 goes to both sides; row 14 to each side `strike_orders::recipients`
   tells. Rows 7-10 end the game. Row 5 has no port: the port has no
   multiplayer lobby.

### Proposed follow-up questions

- Who posts `0x465` and `WM_COMMAND 0xf1` to the movie window and the
  application window, and does `0x465` skip movie `0x66` after `0x65`?
  (`FUN_004138a0`, `FUN_00421790`)
- What exactly gates `FUN_00597480` (`DAT_006be354`) at the first activation:
  state 1 (intro movies) or state 5 (menu)? (`FUN_005974c0`, `FUN_005f8600`,
  `FUN_005fa5e0`)
- Whether MDATA.101 plays for every strike by a fleet holding a family
  `0x18`-`0x1b` ship or for Destroy System only. The FleetBombard slot
  (`FUN_004ff840`, fleet vtable `0x0065d438` slot `+0x208`, entry
  `0x0065d640`) is called by `FUN_004ff450` when a side's view of the fleet's
  bombard bit changes, which `FUN_004fe460` runs for both sides whenever the
  order sets the bit or the finalize or an arrival clears it (`bombardment.md`,
  "Messages"); `FUN_004950d0` walks the fleet's ships through the
  `[0x18,0x1c)` iterator `FUN_005003d0`.
- Is a kind-6 message without a movie id possible (other classes), and what
  happens when `+0x5c` is 0 (`FUN_00421ba0` returns null)?
- Does the Empire human player see the trench-run movies (`0x70`, `0x71`)
  when the AI Alliance runs them? (`FUN_005cfec0`, `FUN_005c0fb0`)
- Where does the strategic `0x402` handler (`FUN_00422ce0`, `FUN_0041d330`)
  resume the clock?

## Open questions

- Why row 12 and row 13 can fire on every `0x128` while the condition holds
  (the AD bit, Coruscant owned by the Alliance): the message classes test the
  state, not a change. `0x128` only posts when the word changes
  (`victory-conditions.md`), hyp: so each fires once per change.
- Whether the Death Star meaning of rows 11 and 14 (taken from the TEXTSTRA
  titles) matches the object families `0x18`-`0x1c`; `game-end-path.md` ties
  `0x18`-`0x1b` to Destroy System.
