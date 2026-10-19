---
title: "Message display: TEXTSTRA formatter, mode-2 layout, message classes"
description: "How a game message gets its title, body, picture and sound, and how the Message Index draws it full-size; with the Battle Alert's summary choice"
category: "ghidra"
created: 2026-10-09
updated: 2026-10-17
tags: [message-index, textstra, formatter, notifications, battle-alert]
---

# Message display

Companion to `message-index-rows.md` (the list) and
`droid-advisor-triggers.md` (advice codes). The port's side is
`crates/rebellion-data/src/text_templates.rs` (formatter),
`crates/rebellion-render/src/message_index.rs` (`draw_single_message`) and
the builders in `crates/rebellion-app/src/main.rs` (`uprising_messages`,
`blockade_messages`, `fleet_arrival_message`, `loyalty_messages`,
`battle_alert_text`).

## The formatter (`FUN_0060b9d0` → `FUN_0060b840`)

`FUN_0060b7b0` loads TEXTSTRA.DLL and reads two RCDATA bytes: `0xbb8` is the
placeholder prefix `|` (0x7c), `0xbb9` the terminator `0x01`. A template is
an RCDATA resource walked byte by byte:

- `0x01` ends it;
- `|` opens a 6-byte placeholder: the argument index (1–4 for the call's
  `param_3..param_6`; with `0x80` set, the formatter's own array), then the
  kind as a u32 LE;
- any other byte is a Latin-1 literal.

Each argument points at `object + 0x30`, a renderable whose vtable slot 0
takes the kind. A missing argument renders nothing.

| Kind | Renders | Example |
|---|---|---|
| 1 | name | "Coruscant", "Fleet 1"; a side: "Alliance"/"Empire" (`0x4d50`/`0x4d51`) |
| 3 | class | "Star Destroyer" (`0x7087`) |
| 4 | side adjective | "Alliance"/"Imperial" (`0x4d70`/`0x4d71`); hyp: a system's kind 4 is its name (`0x7022`) |
| 5, 6, 7 | quantity, count, Jedi rank | one template each (`0x7055`, `0x7181`); not ported |

List templates end in a newline and are appended to the body (`0x707e`,
`0x707f`, `0x7082`).

## Message fields

A message node carries the title and body ids, the picture (background id
plus an optional centred overlay), and two WAVE ids at `+0x30` and `+0x32`.
When the Message Index shows the message (`FUN_00469de0`, mode 2, not a
battle report), `FUN_0046a6e0` stops both waves of the last one, then each
non-zero id plays: `+0x30` from STRATEGY module 7 (`+0x130 = 7`), `+0x32`
from module 9 (`+0x148 = 9`), the player's sprite DLL, ALSPRITE.DLL or
EMSPRITE.DLL (`FUN_0042d650`). Nothing else gates them; filing plays
neither. The class handlers fill `+0x32` through `FUN_004c4990`, which
reads a module 9 RCDATA list `[u16 n][u16 v0..]` and returns `v0`, or with
more entries the one drawn by `FUN_0041cd80(n)`. The port's
`MessageDisplay { title, body, background, overlay, sound, voice }` carries
both; the Message Index starts them as the original does
(`apply_message_index_actions`, `play_message_sound`), the voice from the
droids' sprite DLL sounds (`AdvisorState::sprite_voice`), native and
browser. Of the classes the port files, two draw `+0x32` for a major
character (`AdvisorState::voice_line` over `major_voice_list`): the mission
end-code message for its lead (lists `0xf3`, `0xdf`, `0xcb`/`0xd5`, `0xe9`,
`0x107`, `0xfd`; `mission-lifecycle.md`) and the escape message for the
escapee's own side (`FUN_00490340.c:207-244`: `0xf6`, `0xe2`, `0xce`/`0xd8`,
`0xec`, `0x10a`, `0x100`; the other side none). Luke's second list is the one
with `+0xb0` bit 3 set (Dagobah done; port: the story event `0x210` having
fired). The other `FUN_004c4990` callers belong to classes the port does not
file: the phase-10 reports (`FUN_004927c0`), Personnel Arrive
(`FUN_004955b0`) and the story messages (such as `FUN_004c4990(0x11d)`).

## Mode 2: one message full-size (`FUN_00469de0`)

Logical coordinates inside the 424×333 window:

| Part | Resource | Position, size |
|---|---|---|
| title strip | `0x2ab5` | (12, 14), 400×18 |
| category icon | rail icon (below) | (17, 16) hyp: measured from a Wine capture |
| title text | font 4 | from x 39 hyp: measured |
| up / down arrows | `0x9a` (bitmap `0x2ac4`) / `0x96` (`0x2aa7`) | (367, 15) / (390, 15), 19×15, blue key |
| picture | message background, overlay centred | (12, 33), 400×200, black behind |
| text frame | `0x2ab4` | (12, 232), 400×87 |
| body | font 4, wrapped | (17, 234), 395×80 |

The body is a TextScrollField (`FUN_0041ecf0`): `FUN_0041fd00` wraps the
text to the field and, when it is taller, shows the scroll bar of RCDATA
`0x299d` at the field's right (`FUN_0041efe0`, `FUN_0060a630`) and wraps
again, narrower by the bar's 12 pixels. Its arrows step one line, the text
metric's height (`FUN_00420a90`, `FUN_00420ac0`); the track pages the
field's height in whole lines (`FUN_00420a10`); showing a message resets it
to the top (`FUN_0041fc30`). The bar's layout is in `battle-alert-lists.md`.
The Wine capture of "Maintenance Points" (2026-10-09) shows the bar, lines
16 pixels apart (font 4's cell; hyp: the glyphs measure at a 14-pixel
cell), five whole lines with the sixth cut, and one line per arrow click.
The mode-1 list (`FUN_00607ea0` at (0x19, 0x6c)) takes the same bar.

hyp: the arrows step to the previous or next message of the category in
list order and stop at either end; their handler is untraced.

The picture loader `FUN_0046a320` takes the overlay (`+0x2c`) and the background
(`+0x2e`) ids and loads each through `FUN_0046a280`: STRATEGY module 7 first
(`FUN_005fbd20`), and when that bitmap is missing or invalid the ENCYBMAP
route, `FUN_0045f970(id, ENCYBMAP module)`, which returns the encyclopedia
EDATA picture keyed by that id (ENCYBMAP RT_STRING `id` = `EDATA.nnn`). A
background that neither source yields (null) draws nothing at all
(`FUN_0046a320.c`). The special-force portrait ids `0x1540`-`0x1543` and
`0x1580`-`0x1584` have no STRATEGY bitmap and resolve to `EDATA.025`-`EDATA.033`
(`mission-lifecycle.md`, "End-code messages"). The port's Message Index takes
the same fallback (`paint_message_picture`).


Rail icons (15×16, blue key in the bottom row): Popular Support
`0x2a9a`/`0x2a9b`, Manufacturing `0x2a9c`/`0x2a9d`, Mission
`0x2a9e`/`0x2a9f`, Chat `0x2aa4`, Defense `0x2ad3`, Fleet `0x2ad4`/`0x2ad5`,
Resource `0x2ad6`, Conflict `0x2ad7`, Advice `0x2ad8`/`0x2ad9` (Alliance /
Empire where two).

## Message classes

| Class | Function, event | Templates | Picture | Sound | Code | Filed to |
|---|---|---|---|---|---|---|
| Uprising begins | `FUN_00499460`, `0x14b` | `0x7030`/`0x7033` | `0x3f2` | `0x454` | 1 or 2 (by holder) | both sides |
| Uprising ends | `FUN_00499460`, `0x14b` | `0x7031`/`0x7032` | `0x3f3` Alliance-held, `0x3f4` Empire-held | `0x455` | 1 or 2 | both sides |
| Blockade | `FUN_004960f0`, `0x14e` | holder `0x70a2`/`0x70a3`, other `0x70a0`/`0x70a1` | `0x404` Alliance-held, `0x403` Empire-held | `0x45e` | holder `0xe`, other `0xd` | both; needs a blockading fleet (`FUN_0052bed0`) and a set bit (`param_4`, the value `FUN_0050a5f0` writes), so a lift files nothing |
| Evacuation Losses | `FUN_00495e90`, `0x340` (`FUN_00504a00`) | `0x70a8`/`0x70a9` (system, argument 2), a `0x70aa` line per regiment (argument 1) | `0x407` side 1, `0x406` side 2 | `0x460` side 1, `0x45f` side 2 | `0x10` | each side holding a view of the regiment and the system; the first regiment sets the title and body (`+0x68`), each regiment adds its line |
| Fleet arrives | `FUN_004981c0` case 1, `0x105` | `0x7078`/`0x7079` | `0x3fa` Alliance, `0x3fb` Empire | `0x45a` | 5 | arriving side |
| Capital ships arrive | case 2 | `0x7076`/`0x7077` + list | — | — | 5 | arriving side; not ported |
| Units arrive | case 3 | `0x707a`/`0x707b` | `0x3fc`/`0x3fd` | — | 6 | not ported |
| HQ arrives | case 4 | `0x707c`/`0x707d` | `0x3fe` | — | 6 | not ported |
| Loyalty joins | `FUN_00499760`, `0x100` | joining side `0x7038`/`0x703b`, other `0x7039`/`0x703c` | `0x3ed` | `0x450` | 1 / 2 | both sides |
| Declares neutrality | `FUN_00499760` | `0x703a`/`0x703d` | `0x3ed` | `0x451` | none | both sides |
| Maintenance | `FUN_00498970` + `FUN_0048bbf0` | lists destroyed units | `0x3f5` Alliance, `0x3f6` Empire | `0x456` | `0xc` | one notice per overdraft scrap (`maintenance_shortfall_message`) |
| Operation Report | `FUN_00496460`, events `0x14f`/`0x150` (+ `0x182`/`0x183`) | `0x7098`-`0x709a` (bombardment), `0x7090`-`0x7094` (assault) | `0x2b9a`-`0x2b9c` / `0x2b98`-`0x2b99` | none | `0x2e`/`0x2f` | popup through `FUN_00469de0` -> window `0x1c` for the fleet owner; the other side to the Index |
| Mission ends (own) | `FUN_00491500`, `0x1f1` (end code) | foiled `0x7148`-`0x714b` (codes 3-5), failed `0x7138`-`0x713d` (6, 7, 9, 10, `0xc`), aborted `0x7140`-`0x7145` (8), then `0x70e6`/`0x70e7` | `0x412`/`0x413`, foiled `0x416`/`0x417`, a character lead's portrait overlay | `0x47c`/`0x47d`, foiled `0x480` | mask `0x10`, advice by leader `0x15`-`0x1b` | the mission's side; codes 0-2 file nothing (`mission-lifecycle.md`, "End-code messages") |
| Enemy Mission Foiled | `FUN_00491500`, `0x1f1` | `0x713e`/`0x713f` | `0x431` Alliance, `0x432` Empire | `0x47c`/`0x47d` | mask `0x10` | the opponent, once a detection ends the mission (codes 3, 4) |
| Construction | `FUN_004c5000` | — | `(id & 0xfff) + 0x1000` | — | — | not ported |

Notification `0x100` comes from every owner-bit write of a system:
`FUN_004f6f40` copies the bits to both side views (`FUN_004f9ca0`), and a
view whose bits changed runs slot `+0x158` (`FUN_004fbbd0`, all three
system vtables), which logs `GameObjControlKindNotif` and posts `0x100` with
the old and the new side (`FUN_0053fcf0`). `read`. `FUN_00499760` then files
the Loyalty Change Message only when the system's view for the player's side
(`FUN_004f3220`) exists, its owner bits equal the new side, and its `+0x88`
bit `0x2000` is set (`FUN_00499760.c:23-29`); otherwise it files nothing for
that change. `read`.

### The loyalty bit (system `+0x88` bit `0x2000`)

One writer sets it, and it is set on every holder change.

- `read`: slot `+0x208` (`FUN_00510d70`, vtable `0x0065e640` entry `0x0065e848`)
  runs when `FUN_0050a300` stores a new control word (`+0x84`; `support-shifts.md`).
  When the old and new words' holder bits differ (`(old ^ new) & 3`,
  `FUN_00510d70.c:11`) it calls `FUN_0050a970(this, value, ctx)` with `value`
  = 1 when neither word has `0x30` bits and 0 otherwise (`:12-18`), and only
  then `FUN_004f6f40(this, new & 3, ctx)`, the owner-bit write that posts
  `0x100` (`:19`). The bit is therefore current when the message is built.
- `read`: bits `0x30` of the control word are the usable-regiment bits
  (`support-shifts.md`; `FUN_0050b610.c:63`, `FUN_00558930`). So the bit
  means: this holder change involved no regiments at either end. It is 1 for
  a change the support alone caused (a support shift past the loyalty
  threshold `DAT_006bb4c0`, `FUN_0055a080`) and for a populated bit clearing
  with no regiments there (holder to 3, Destroy System), and 0 for a change
  where either word has a garrison: a landing capture, a defender kill that
  leaves a side's regiments, a regiment arriving or leaving.
- `read`: `FUN_0050a970` ends at `FUN_0053a640(0x2000, value, +0x88)`, guarded
  by `FUN_0053a000` (the base object: `+0x24` bits 4-5 clear), then copies the
  bit to both side views (`FUN_005103a0`, `FUN_00539fd0(this, 1/2)`,
  `FUN_0053a610` masked copy) and calls slot `+0x240` (the hook of bit
  `0x2000`; not read). `FUN_005103a0` is also run for every view refresh
  (`FUN_0050d5a0`, `FUN_0050e5b0`), which recopies the base value.
- `read`: it is cleared only by the next holder change that has regiments at
  an end. A change of bits `0x30` alone (the holder unchanged) leaves it as
  it was. `inf`: the constructor `FUN_00507130` stores the word's initial value
  (`0x005071b2`, value not read). Grep over REBEXE for `0x2000` operands
  finds no other system `+0x88` writer; `FUN_005103a0` and `FUN_0050a970` are
  the two functions that name the mask, and `FUN_0050a970`'s only caller is
  `FUN_00510d70` (`0x00510dba`).
- `inf`: the other `FUN_004f6f40` callers (`FUN_00530d70` via `FUN_004f9190`,
  `FUN_0052b7d0`, slot `+0xb0`, `FUN_004eec60`, `0x00503df7`) hand other
  objects (characters, facilities, fleets) their side; none reaches a system,
  so a system's `0x100` always follows a `FUN_00510d70` write.
- `inf`: the filed message is "Joins"/"Joins Enemy" for a holder change to 1
  or 2 and "Declares Neutrality" for a change to 3, so a garrison-driven
  capture files none of the three; its notice is the Operation Report.

**Port.** `control_change_messages` (`crates/rebellion-app/src/main.rs:12141`)
files for each served holder change whose side differs, and its four callers
(`fire_support_events`, `main.rs:11539`, `assault_orders::assault_messages`,
`strike_orders::strike_messages`) pass no garrison information, although the
`ControlRerun` they iterate carries it (`troops: (old, new)`,
`crates/rebellion-core/src/support_shift.rs:138`). The gate is missing: the
port files a Loyalty Change Message for an assault capture and for regiment
driven flips, where the original files none. `crates/rebellion-app/src/assault_orders.rs`
(the capture test near `:531`) asserts the two capture messages with a comment
that the original posts them "whatever changed the holder"; that expectation
contradicts `FUN_00510d70.c:12-18`. The fix is one condition in
`control_change_messages`: take the change's `troops` and return no messages
when `troops.0 || troops.1`, which is `0x2000 == 0`. The comment on
`loyalty_messages` (gate "untraced") then names this bit.

## The Battle Alert (`FUN_0049c0d0`)

The title is `0x7020` ("Battle at |system|"). `+0x20` is the player's side,
`+0x48` the side compared with the holder (`sys[9] >> 6 & 3`), and
`sys[0x1e] >> 6 & 3` equals the holder when the holder is blockaded:

| Holder | Blockaded | Template | Arguments |
|---|---|---|---|
| player | no | `0x7021` "The {enemy} fleet is threatening {system}. {player} forces are moving to intercept." | enemy, system, player |
| player | yes | `0x7022` "The {player} fleet is attempting to break the {enemy} blockade of {system}." | player, system (kind 4), enemy |
| neither (0 or 3) | — | `0x7025` "{player} and {enemy} forces are about to engage in battle near the {system} system." | player, enemy, system |
| enemy | yes | `0x7024` "The {enemy} fleet is attempting to break the {player} blockade at {system}" | enemy, player, system |
| enemy | no | `0x7023` "The {player} fleet has entered the {system} system. {enemy} forces have been detected on an intercept course." | player, system, enemy |

Text is red for the Alliance and green for the Empire, in Arial (the port
uses Liberation Sans, metric-compatible): font 6 in a 0x148-wide box at
(0x24, 0xd8), over a black copy one pixel down and right; the title in font
0xb, centred in 400 at y 16 (measured in `battle-alert-lists.md`). Verified
in the port for all five templates on 2026-10-09 with `Battle at` and
`Blockade` (agent_docs/dev-commands.md).

The alert and the battle results are the one 470x331 window over the live
command center: the 22-battle-alert-result capture shows the counters, the
day and open sector windows behind it. The port draws the command center
first with its input held (`TacticalState::over_command_center`), then the
window; Take Command alone leaves for the tactical screen. Verified for both
sides on 2026-10-09 in a native seed-42 game, a staged battle at Tatooine on
the first day, alert then results.
