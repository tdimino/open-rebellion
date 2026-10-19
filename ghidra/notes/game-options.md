# Unified Game Options: source contract

Scope: PRE-03 / RE-OPT-01. Recovery against the owned English executable
SHA-256 `b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.
This is static evidence, not original-executable visual acceptance.

## Entry and construction

The main-menu CD-ROM command is **0x68**, not RE-OPT-01's current 0x67
retrieval hint (0x67 is multiplayer). `FUN_00406000`'s jump table at
`004064e0` maps 0x68 to `004061d0`, which requests application state 11
through `FUN_00401040`. Cockpit/F1 uses command 0x133.

`FUN_00406950` constructs the shared window and snapshots options flags.
`FUN_00406b80` handles creation by calling `004070e0` (background),
`004084b0` (text), and `00407180` (controls). The background loader at
`00407119` requests resource **COMMON 20002**, 640 by 480 pixels.

## Controls

Coordinates below follow upstream P58-B12 placement in logical source pixels.
The recovered IDs/commands are retained. Bitmap triples mean normal,
pressed/selected, disabled. The six save rows start at y=81 and increment 42.

| Control | Rectangle | Command | COMMON bitmaps |
|---|---|---|---|
| Save row 1–6 | (35,y,42,20) | 0x6f–0x74 | 10046/10047/10048 |
| Save name 1–6 | (116,y,162,20) | 0x83–0x88 | native edit control |
| Load row 1–6 | (285,y,41,20) | 0x79–0x7e | 10049/10050/10051 |
| Effects volume | origin (391,192) | 0x8e | 10053, thumb 10054 |
| Music volume | origin (391,130) | 0x8d | 10052, thumb 10054 |
| Restart | (77,383,42,42) | 0x91 | 10035/10036 |
| Return | (164,383,42,42) | 0x90 | context-specific 10020–10031 |
| Exit | (251,383,42,42) | 0x8f | 10038/10039 |
| Play music | (351,76,19,35) | 0x99 | 10040/10041/10042 |
| Starfield | (359,310,35,22) | 0xa1 | 10043/10044/10045 |
| Planet | (359,337,35,22) | 0xa2 | same |
| Pyrotechnics | (359,364,35,22) | 0xa3 | same |
| High detail | (359,391,35,22) | 0xa4 | same |
| Holocube | (359,418,35,22) | 0xa6 | same |

`FUN_004084b0` maps labels to TEXTCOMM 0x1025 (music), 0x101f–0x1022
(first four display switches), 0x1024 (Holocube), and 0x1031/0x1032 (On/Off).
`FUN_004082b0` maps the switches to masks 0x100, 0x10000000, 0x01000000,
0x00100000, 0x00010000, and 0x02000000 respectively.

## Context and mutation boundaries

`FUN_00407180` disables save/edit controls when the current state is zero or
20–23; load is also disabled for states 20–23 and for absent/incompatible
save metadata. All five tactical display switches are disabled in states
20–23. The manual's printed pages 75–77 independently confirm six slots,
overwrite/current-game confirmations, and that display options cannot change
during a tactical battle. All five display options default on in the original.

The manual describes Restart as abandoning the campaign and returning to the
Shuttle, not replaying a battle. `FUN_00407180` builds Restart (0x91) with
its normal and pressed art (0x2733, 0x2734) and never disables it, in a
battle too: its battle test (`bVar1`, `+0xa8` in 0x14-0x17) gates only the
save and load rows and the display switches. `FUN_00406b80` sends Restart to
state 3, the Shuttle, at once when no game runs (`+0xa8` = 0) and through the
0x476 confirmation when one does, unless a row flag is set. Return (0x90) is
disabled when no game runs and the window was not opened from state 4 or 5,
so from the Shuttle Restart is the way back; Return sends states 4 and 5 to
state 3 and any other state back to itself.

The row flags (`+0xac`-`+0xc0`, one per row) record the rows this opening of
the window saved. `FUN_00406950` zeroes them; after a save, `FUN_00415600`
reports the result to the open window (application state 0xb or 0xc, child
`+0xc0`), whose `FUN_00408240` shows alert 0x474 (TEXTSTRA 0x1217) and sets
the row's flag on success, or shows 0x475 (0x1219) on failure. A set flag
skips the overwrite confirmation (0x478) for its own row, and any set flag
skips the Load (0x477) and Restart (0x476) confirmations. The alerts use
the "Resume Game Play?" alert's class (`FUN_00417020` → `FUN_00416f80`):
REBDLOG background 0x297e, the text in its message rectangle, the checkmark
labelled REBDLOG 0x1300 ("Yes"); Enter works as the checkmark and Escape
does nothing. port: `GameOptionsState::save_result` and `SaveAlert`.
These are navigation operations; opening/closing options must not mutate a
battle roster, apply results, advance simulation, or reset a paused state.

Native confirmation construction is `FUN_00416c50` / `FUN_00416b90`:
REBDLOG bitmap 10623, 412 by 176. `00416e90` constructs yes/no at (138,135)
and (229,135), with bitmap pairs 10624/10625 and 10626/10627; both are 57 by
28. Upstream stages all 23 numeric REBDLOG bitmaps; the unused named corner is excluded. This contribution requires 10623–10627 at packaging time and provides a visible fallback for older packs.

## Compatibility and evidence gaps

- The window addresses six saved games and no more. `FUN_00407180` builds
  rows 1 to 6 (`local_ec` from 1, the loop continuing while `local_ec < 6`
  before the increment); row `n` reads save file `n` (`FUN_00401860`) and
  carries Save `0x6e + n` (0x6f-0x74), Load `0x78 + n` (0x79-0x7e) and its
  name edit `0x82 + n` (0x83-0x88). `FUN_00406b80` maps a Save command back
  to its file as `command - 0x6e`. No control or command names a seventh
  file. port: `game_options.rs` `SAVE_ROWS` = 6, and the save layer's
  `MAX_SAVE_SLOTS` = 6 to match; the galaxy's save shortcut
  (S) opens this screen, as its cockpit button does.
- No native delete button is present in this constructor or the inspected
  manual pages, and the port has none.
- Escape: the window procedure `FUN_00406b80` (vtable `0x00658600` slot
  `+0x14`, set in `FUN_00406950`) has no `WM_KEYDOWN` case; its default path
  `FUN_005ff530` and that one's `FUN_006007b0` handle none either and end in
  `DefWindowProcA`. Escape in the window therefore does nothing beyond what a
  focused name edit does with it (`keyboard-shortcuts.md`, section 3).
- The confirmation (vtable `0x00658950`, built by `FUN_00416b90`) takes keys:
  its procedure is the base `FUN_00606650`, whose `WM_KEYDOWN` calls slot
  `+0x4c`, `FUN_00416f30`. Enter answers Yes and Escape No through
  `FUN_004011a0`, which posts `0x46a` (accepted, with the action code from
  `DAT_006b1470`) or `0x46b` to the window; `FUN_00406b80` acts on `0x46a`
  only. port: `GameOptionsState::key`.
- Upstream display flags, including Holocube, are preserved. Their full original
  visual behavior remains outside this save/confirmation checkpoint.
- Exact text metrics, slider transfer function, complete context identities,
  settings persistence and lossless A0 interaction captures remain to verify.
- The Rust confirmation guard tests are implementation evidence only; they
  do not establish original pixel, audio, or full PRE-03 acceptance.
