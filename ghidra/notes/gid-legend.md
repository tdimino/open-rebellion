---
title: "Galaxy Display Legend and Menu Highlight"
description: "The GID's compact and detailed legends (STRATEGY RT_RCDATA draw scripts, darkened window, drag) and the GID menu's highlight colors"
---

# Galaxy Display Legend and Menu Highlight

Traced 2026-10-08 against `FUN_00426d00`, `FUN_00452090`, `FUN_004522f0`,
`FUN_00452630`, `FUN_00452240`, `FUN_005fe050`, `FUN_00421c70`, `FUN_004511e0`,
`FUN_004aab50`, `FUN_004aba60` and `FUN_004abe10` (decompiled beside this
note), then checked in the original under Wine (Alliance, Popular Support).

## Compact legend

`FUN_00427270` builds it through `FUN_00452ab0` from STRATEGY 10168 (47x25)
at (55, 50) for the Alliance and (113, 50) for the Empire, stored at galaxy
view `+0x248`. A double click sends command 0x134 to `FUN_00422ce0`, which
opens the detailed legend and hides the compact one. In the original both
legends drag around the galaxy view (Wine, 2026-10-08).

10168's border and its text share palette index 255, which is also its first
stored pixel, so a first-pixel key would erase the text. The original shows
no white border. How it draws the compact legend is untraced.

## Detailed legend

`FUN_00426d00(view, mode)` destroys any open legend (`+0x124`). Mode 0x80
(Display Off) shows the compact legend again. Any other mode clamps the
saved position (`+0x11c`, `+0x120`; `FUN_00421c70` starts it at (0x96, 0x78))
so that a 0xb4 by 0xf0 window fits the view. It then creates the window
through `FUN_00452090` at that size, with style 0x56000000. `FUN_00452090`
calls `FUN_006071a0` (not decompiled), the likely drag setup.

### Content scripts

`FUN_004522f0` maps the mode to a STRATEGY `RT_RCDATA` script:

| Mode | Script | Title |
|---|---|---|
| 0x11 | 10183 (side 1) / 10184 | Loyalty to the Alliance / Empire |
| 0x12 | 11600 | Worlds in Uprising |
| 0x21 | 11601 | Idle Fleets |
| 0x22 | 11602 | Fleets Enroute |
| 0x30 | 10201 | Units Enroute |
| 0x43 | 11603 | Idle Personnel |
| 0x44 | 11604 | Active Personnel |
| 0x51 | 10188 | Available Energy |
| 0x52 | 10189 | Available Raw Materials |
| 0x53 | 10190 | Mines |
| 0x54 | 10191 | Refineries |
| 0x61 | 10192 | Manufacturing |
| 0x62 | 10193 | Shipyards |
| 0x63 | 10194 | Training Facilities |
| 0x64 | 10195 | Construction Yards |
| 0x65 | 11605 | Idle Shipyards |
| 0x66 | 11606 | Idle Training Facilities |
| 0x67 | 11607 | Idle Construction Yards |
| 0x71 | 10196 | Trooper Regiments |
| 0x72 | 10197 | Fighter Squadrons |
| 0x73 | 10198 | Death Star Shields |
| 0x74 | 10199 | Shield Generators |
| 0x75 | 10200 | Defense Batteries |

`FUN_00452630` replays a script as a stream of 16-bit words:

- **Header:** width, height, then the bitmap module (7, STRATEGY) and the
  string module (2, TEXTSTRA). The window shrinks to the width and height.
- **1 or 2, x, y, bitmap:** a keyed blit (`FUN_005fd0f0`) at (x, y).
- **3, x, y, string, font, r, g, b:** the string in that font and color. A
  y of 1 centres it across the window's width (`FUN_00403e90`), ignoring x.
- **0:** end.

The scripts are 180 wide.

- **Four-row scripts (135 tall):** glyphs 10181, 10180, 10170 and 10169,
  largest first, then the 210x3 rule 10182 at (0, 100).
- **Two-row scripts (95 tall):** only 10181 and 10169, with the rule at
  (0, 60).
- **The two fleet scripts (11601, 11602, 155 tall):** add 10160 for
  Conflict.
- **Every script:** ends with the key emblems 10243, 10241, 10245 and 10158.

All of these bitmaps have a palette-blue first pixel.

The port transcribes the scripts, with their TEXTSTRA strings, in
`crates/rebellion-render/src/gid_legend.rs`. It leaves out 10192 (0x61,
Manufacturing) and 10201 (0x30, Units Enroute), whose modes the port's
menu does not offer.

### Frame, close button, darkening

`FUN_004522f0` frames the window with STRATEGY 10100..10107
(`FUN_00607740`, the GID popups' frame). It adds a close button, 10108 with
10109 pressed, at (width less the button's width less 3, 3).

`FUN_00452240` paints the window in four steps:
1. It copies the galaxy backbuffer under the window.
2. `FUN_005fe050` darkens it through a palette remap table built from
   `LAB_004ac550` (`FUN_005fddd0`).
3. It keys the content (`+0x148`) and the frame over the darkened copy.
4. It blits the result.

The remap table is untraced. The port uses a dark tint, tuned by eye
against Wine captures.

### Port

- **Opening:** `cockpit.rs` opens the legend on a double click of the
  compact legend. It applies `FUN_00426d00`'s clamp when the legend opens.
- **Switching:** it shows the shown mode's script, so changing the display
  rebuilds the legend. Display Off closes it.
- **Dragging (hyp):** both legends drag from anywhere except the close
  button, and a drag keeps the whole window in the galaxy view. The
  original drags them; its clamp is untraced.
- **Not ported:** the zoom-rect animation (`FUN_00428940`), and closing the
  legend when other panels open.

## GID menu highlight

`FUN_004511e0` sets each popup's colors through `FUN_004ab1b0` →
`FUN_004abe10`:
- **Highlight (`+0x2c`):** `0x20000ff` (red) for side 1, otherwise
  `0x200ff00` (green).
- **Normal (`+0x30`):** `0x2ffffff` (white).

`FUN_004aab50` handles `WM_MOUSEMOVE` by setting the highlighted item
(`+0x46`). `WM_PAINT` paints that item last with `FUN_004aba60(item, hdc, 1)`,
in the highlight color.

While the pointer is in a submenu, the parent popup gets no `WM_MOUSEMOVE`,
so its category stays highlighted. Nothing marks the chosen mode:
`FUN_004511e0` builds every item through `FUN_004ab1f0` with no check
bitmap (the last two arguments 0, which `FUN_004ab560` keeps as `+0x1c`
and `+0x5c`), so `FUN_004aba60` never draws one, and the reference frame
`0450-imperial-gid-menu.png` shows none. (The port once drew `0x277c`, the
windows' Close button, as a check; removed 2026-10-09.)

Each category passes the same bitmap for its highlighted and normal states,
so the icon does not change. The port colors the hovered row and the open
submenu's category (`cockpit.rs`, `gid_menu_row`).
