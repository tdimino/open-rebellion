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

Coordinates below are logical source pixels. Bitmap triples mean normal,
pressed/selected, disabled. The six save rows start at y=81 and increment 42.

| Control | Rectangle | Command | COMMON bitmaps |
|---|---|---|---|
| Save row 1–6 | (34,y,42,20) | 0x6f–0x74 | 10046/10047/10048 |
| Save name 1–6 | (119,y,160,20) | 0x83–0x88 | native edit control |
| Load row 1–6 | (287,y,41,20) | 0x79–0x7e | 10049/10050/10051 |
| Effects volume | origin (392,190) | 0x8e | 10053, thumb 10054 |
| Music volume | origin (392,130) | 0x8d | 10052, thumb 10054 |
| Restart | (76,381,42,42) | 0x91 | 10035/10036 |
| Return | (162,382,42,42) | 0x90 | context-specific 10020–10031 |
| Exit | (248,381,42,42) | 0x8f | 10038/10039 |
| Play music | (352,76,19,35) | 0x99 | 10040/10041/10042 |
| Starfield | (357,311,35,22) | 0xa1 | 10043/10044/10045 |
| Planet | (357,337,35,22) | 0xa2 | same |
| Pyrotechnics | (357,365,35,22) | 0xa3 | same |
| High detail | (357,392,35,22) | 0xa4 | same |
| Holocube | (357,419,35,22) | 0xa6 | same |

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
Shuttle, not replaying a battle. Return is unavailable from the Shuttle.
These are navigation operations; opening/closing options must not mutate a
battle roster, apply results, advance simulation, or reset a paused state.

Native confirmation construction is `FUN_00416c50` / `FUN_00416b90`:
REBDLOG bitmap 10623, 412 by 176. `00416e90` constructs yes/no at (138,135)
and (229,135), with bitmap pairs 10624/10625 and 10626/10627; both are 57 by
28. This contribution adds REBDLOG to the staging catalog and cache enum.

## Compatibility and evidence gaps

- This contribution offers six slots in both save interfaces, as requested.
  The backend retains its existing ten-slot storage capacity; do not truncate or migrate those saves.
  The six-row window retains F8 / Ctrl+L for the six load slots and campaign F9 / Ctrl+S for the six save/delete slots, sharing confirmation guards.
- No native delete button is present in this constructor or the inspected
  manual pages. Do not invent one; existing legacy deletion remains separate
  until its native gesture is recovered.
- No Holocube renderer was found in current upstream. A stored boolean alone
  would not implement the switch.
- Exact text metrics, slider transfer function, complete context identities,
  settings persistence and lossless A0 interaction captures remain to verify.
- The Rust confirmation guard tests are implementation evidence only; they
  do not establish original pixel, audio, or full PRE-03 acceptance.
