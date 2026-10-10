---
title: "Battle Alert lists: Alliance Forces, Imperial Forces, System Assets"
description: "The pre-battle alert's three list pages: what FUN_00450770 lists, the row layout of FUN_00450e10, the list's picture and its RCDATA scroll bar"
category: "ghidra"
created: 2026-10-09
updated: 2026-10-09
tags: [battle-alert, tactical, list, scroll-bar, gokres]
---

# Battle Alert lists

Companion to `message-display.md` ("The Battle Alert"). The port's side is
`crates/rebellion-render/src/tactical_view.rs` (`battle_alert_rows`,
`draw_battle_alert_list`).

## The window (`FUN_0044f860`)

- Two layers, each the frame with a picture at (12, 13) and the title
  ("Battle at |system|", font 0xb, red `0xff` Alliance or green `0xff00`
  Empire, centred in 400 at y 16):
  - `0x32`, Summary: the scene `0x29d8`/`0x29d9` and the summary text
    (font 6, 0x148 wide at (0x24, 0xd8), over a black copy one pixel down
    and right);
  - `0x33`, the list pages: the picture `0x29da`/`0x29db` (10714/10715).
- The list: `FUN_00607ea0` at (0x53, 0x28), 0x100 by 0xea, rows 0x28 high,
  white text (`FUN_00609950(.., 0xffffff)`), at `+300`. Its scroll bar comes
  from RCDATA `0x29fc` of module 7 (`FUN_0060a490`).
- The tabs `0x1922..0x1925` (bitmaps `0x29e8..0x29ef` Alliance,
  `0x29f2..0x29f9` Empire); `FUN_00450660` switches: case 1 Summary hides
  the list and shows layer `0x32`; cases 2, 3 and 4 show layer `0x33`,
  refill the list (`FUN_00450770` with 1, 2 or 0) and scroll it to the top
  (`FUN_0060a280`).

## The rows (`FUN_00450770`)

| Page | Heading | Then |
|---|---|---|
| Alliance Forces (1) | `0x1923` "Alliance Forces" | per fleet of side 1 at the system (`FUN_004ffef0`): the fleet; per ship (`FUN_00502db0`): the ship, then its squadrons (`FUN_005039d0`), regiments (`FUN_00504c40`) and personnel (`FUN_00536da0`) indented; last, squadrons at the system (`FUN_00503a50`) under the system's name |
| Imperial Forces (2) | `0x1924` "Imperial Forces" | the same for side 2 |
| System Assets (0) | `0x1925` "System Assets" | facilities, the system's children in families 0x20..0x2f (`FUN_00539d70`); regiments (`FUN_00504c40`); personnel (`FUN_00536da0`) |

`FUN_00450e10(object, font, indent, picture)` builds one row: the name
(`FUN_004f62d0`) in `font`; with a picture, the object's GOKRES mini
(`FUN_0042c3b0(.., 0, 1)`) and text from x 0x46 (`+0x30`); indented, the
mini moved 0x14 right in a widened bitmap and the text from 0x5a. A row
without a picture keeps `+0x50 = 1`. The fonts:

| Row | Font | Picture | Indent |
|---|---|---|---|
| Heading | 0xb, `+0x50 = 1`, `+0x3c \| 4` | — | — |
| Fleet, system name | 5 | — | — |
| Ship | 5 | yes | no |
| Carried unit | 4 (`FUN_00450ce0`) | yes | aboard a ship |
| System asset | 4 | yes | no |

## Measured (steam-guide captures 055 to 057)

The guide captures are cropped: matching the list picture (`0x29db`, at
(12, 13)) puts the window's origin at (1, 5) in 056 and (1, 8) in 057, and
the scene `0x29d9` puts it at (4, 12) in 055. In window coordinates:

- The minis of "LNR Series II" and "GenCore Level II" sit exactly at
  (83, 80) and (83, 120): each row's picture and text cell start at the
  row's top left, the list at (0x53, 0x28) as `FUN_0044f860` places it.
- The centred rows ("System Assets", "Alliance Forces", "Deyer") centre on
  x 203.5.
- hyp: the sizes do not follow `FUN_0060eed0` for fonts 4 and 5 (16-pixel
  cells). "GenCore Level II" is 91 pixels wide and "A-wing" 36 (regular),
  "Deyer" 34 (bold), which Arial gives at a 14-pixel cell; the heading,
  "Alliance Forces" 142 wide, at an em of 20.5 (font 0xb's 24-pixel cell
  would give 148). The Message Index's text measures the same way.
- The summary (055) starts at (36, 219) on 18-pixel lines (font 6's cell),
  breaking "…the Deyer | system. Alliance forces have been detected on | an
  intercept course." inside its 0x148 box. hyp: its widths ("The Imperial
  fleet has entered the Deyer" 275) measure at an em of 15.5, and the title
  ("Battle at Deyer" 139, starting at y 20, centred on x 200) at 21.

## The scroll bar (`FUN_0060a490`, class "Scroll Bar")

The bar is a child of its owner. `FUN_0060fa80` lays it out as wide as the
RCDATA's first word, flush with the owner's right edge, and as tall as the
owner; the 13-pixel pieces centre on it (`width / 2 - 13 / 2`). The list's
word is `0x0a`, so its pieces sit at x 328 (0x53 + 0x100 - 10 - 1), from
y 40, 234 high. The thumb (`FUN_0060fb80`) is the visible share of the
track, raised to 12 pixels when under 13, placed in proportion; the bar
hides while everything is in view (`FUN_0060b5d0`). The arrows send ids 100
and 101 (`FUN_0060f6d0`), one step each; ids 2 and 3 page.

RCDATA `0x29fc` (this list): `0x0a`, then STRATEGY bitmaps `0x29fd` (track
tile, 13x13), `0x29fe`/`0x29ff` (up, normal and pressed, 13x9),
`0x2a00`/`0x2a01` (down), `0x2a02`, `0x2a03`, `0x2a04` (thumb top 13x6,
middle tile 13x12, bottom 13x6) and `0x29bc` (12x12, grey; unused by the
port). RCDATA `0x299d` (the Message Index's list and text field): `0x0c`,
`0x29a0`, `0x29bd`/`0x29be`, `0x29bf`/`0x29c0`, `0x29c1..0x29c3`, `0x29bc`.
The port's `crates/rebellion-render/src/scroll_bar.rs` draws both.

## Port

The rows come from the Fleet window's own listing (`listed_units`, the
right panel's ships, squadrons, regiments and personnel per fleet or
ship) and the Defenses window's sources (facilities, regiments,
`system_members`). port: squadrons stationed at a system are not listed,
because the port keeps squadrons aboard fleets; the facilities follow the
port's lists (defenses, manufacturing, production), not the original's
child order. Verified for both sides on 2026-10-09 (Empire at Coruscant,
Alliance at Yavin, `.artifacts/native-checks/2026-10-09-alert-lists/`;
the corrected geometry and bar on 2026-10-09 in `2026-10-09-alert-parity/`).
