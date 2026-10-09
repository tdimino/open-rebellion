---
title: "Message display: TEXTSTRA formatter, mode-2 layout, message classes"
description: "How a game message gets its title, body, picture and sound, and how the Message Index draws it full-size; with the Battle Alert's summary choice"
category: "ghidra"
created: 2026-10-09
updated: 2026-10-09
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
Those play from STRATEGY module 7 (`+0x130 = 7`) and module 9; they are
sounds, not emblems. The port's `MessageDisplay { title, body, background,
overlay, sound }`. STRATEGY's 66 WAVEs are staged (`strategy-dll/WAVE`);
the port plays `+0x30` when the Message Index shows the message, stopping
the last one first (`FUN_0046a6e0`), and files it without a sound. hyp:
`+0x32` (module 9) has no known ids and is not played.

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
| Blockade | `FUN_004960f0`, `0x14e` | holder `0x70a2`/`0x70a3`, other `0x70a0`/`0x70a1` | `0x404` Alliance-held, `0x403` Empire-held | `0x45e` | holder `0xe`, other `0xd` | both; needs a blockading fleet (`FUN_0052bed0`) |
| Fleet arrives | `FUN_004981c0` case 1, `0x105` | `0x7078`/`0x7079` | `0x3fa` Alliance, `0x3fb` Empire | `0x45a` | 5 | arriving side |
| Capital ships arrive | case 2 | `0x7076`/`0x7077` + list | — | — | 5 | arriving side; not ported |
| Units arrive | case 3 | `0x707a`/`0x707b` | `0x3fc`/`0x3fd` | — | 6 | not ported |
| HQ arrives | case 4 | `0x707c`/`0x707d` | `0x3fe` | — | 6 | not ported |
| Loyalty joins | `FUN_00499760`, `0x100` | joining side `0x7038`/`0x703b`, other `0x7039`/`0x703c` | `0x3ed` | `0x450` | 1 / 2 | both sides |
| Declares neutrality | `FUN_00499760` | `0x703a`/`0x703d` | `0x3ed` | `0x451` | none | both sides |
| Maintenance | `FUN_00498970` + `FUN_0048bbf0` | lists destroyed units | `0x3f5` Alliance, `0x3f6` Empire | `0x456` | `0xc` | not ported (the event lacks the unit list) |
| Construction | `FUN_004c5000` | — | `(id & 0xfff) + 0x1000` | — | — | not ported |

hyp: the port files loyalty where economy control resolution changes the
holder; the original's gate (system flag `0x2000`) is untraced.

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
sides on 2026-10-09 (`.artifacts/native-checks/2026-10-09-battle-window/`).
