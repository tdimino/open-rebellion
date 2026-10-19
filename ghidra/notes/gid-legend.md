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

## Opening the GID menu

The GID control (command 0x132; `FUN_00427270` builds it at (446,406) 27 by
16 for side 1 and (519,434) 37 by 25 for side 2) has no toggle. `FUN_00422ce0`
case 0x132 first calls slot `+8` on every child in the strategic view's
`+0x6c` registry (the two sector windows open on the Alliance game in slot 5
at day 1173 disappeared under Wine), and, when the detailed legend (`+0x124`)
is visible, hides it and shows the compact one (`+0x248`). It then shows the
menu at `+0x118`, reusing it when it exists or building it with
`FUN_00451130` at (370, 233) for side 1 or (420, 233) for side 2, 160 by 160.
A press on the control while the menu is open leaves the menu open and
unchanged; it stays drawn while that press is held (under Wine on that
Alliance game; on the VM on the Empire Death Star game at day 677). The
hidden windows show again (slot `+4`, show code 5)
on two paths in `FUN_00422ce0`: after a row choice (`WM_COMMAND` 0x140,
once `FUN_00425d00` has set the mode) and on a left release on the galaxy
view (`WM_LBUTTONUP` 0x202, view state 1, with the menu visible), which also
hides the menu. Both skip a child whose key is on the 12-slot minimized rail
at `+0x2f0` (`FUN_004291d0`), so a minimized window stays minimized. The
detailed legend does not come back. Under Wine, on the Empire Death Star
save at days 677-700, two System windows and a sector window hid when the menu
opened and came back as they were after a press on empty galaxy space;
Game Options opened over the menu and its Return left the menu open and the
windows hidden. port: `GidUiState::open_menu` and `close_menu`, at
`gid_menu_origin` and 160 by 160; while `windows_hidden` is set the
galaxy's sector, System, Fleet, Defenses and Missions windows are not drawn
or live unless `GidUiState::shown` names them.

### Presses while the menu is open

The menu (`FUN_004aab50`, vtable `0x659d80`) takes no capture, so only a
release on the galaxy view itself closes it (0x202 above). The command
center's controls are child windows that `FUN_00427270` builds and that send
`WM_COMMAND` to the view: the primary and side controls (0x12d..0x133), the
Message Index rail (0x136..0x13e) and the compact legend (0x134, at (55,50)
or (113,50)). A press on one acts and leaves the menu open with the windows
hidden. On alliance-day54-advice at day 61, a rail light opened the Message
Index over the menu, and after the index closed the menu was still open and
the sector window still hidden. The command-center art belongs to the view,
so a press on it closes the menu. The 0x202 path goes on after the close:
over a sector it opens that sector's window (`param_1[0x97]` →
`FUN_00429ce0`), and otherwise it hit-tests the agent panel
(`FUN_004420b0`). Keys are not the menu's either: its procedure passes all
but Enter and Escape to the view's, so the accelerators and function keys
act while it is open. port: `should_dismiss_gid_menu` with
`on_cockpit_control`; the menu is no input blocker, so the controls, the
keys and the galaxy map stay live, and the press that closes the menu goes
on to the map, which opens the sector window of a system under it.

### Escape and the keyboard focus

Escape never closes the menu. It goes where the keyboard focus is:
- **After a load** the focus is on the top window the save restores; the
  loader sends sector windows to `HWND_BOTTOM`.
- **The first GID press gives the menu the focus.** `FUN_00422ce0` 0x132
  builds the menu (`FUN_00451130`) only when the view has none; its
  `WM_CREATE` step (`FUN_006068f0` → slot `+0x38`, `FUN_004511e0`) ends with
  `SetFocus` on it. Later presses reuse `+0x118` and only show it (slot
  `+0x2c`), leaving the focus alone. The menu's Escape (`FUN_004aab50`)
  gives the focus to the view and calls slot `+0x30` (`FUN_00606960`),
  which posts 0x405 for the menu's key (0x140). The view's 0x405 handler
  drops that key from the `+0x478` registry and calls `FUN_00600f90`; the
  menu is not in `+0x6c`, so nothing is destroyed and it stays open. The
  handler then posts 0x467(0,0). Enter chooses the highlighted row, and
  every other key goes to the view's procedure.
- **0x467 with no window** only focuses the view while its current child
  (`param_1[0x2e]`) is empty. Otherwise it walks the view's children from
  the top of the z-order (`GetWindow(view, GW_CHILD)`) to the first one in
  `+0x6c` and `+0x478` and not on the minimized rail, and shows it
  (`ShowWindow` 5), raises it and gives it the focus, whether or not the
  menu is open.
- **A new window leaves the focus where it was.** `FUN_0045aac0` (a detail
  window, from a sector window's double-click, `FUN_004593e0` 0x203) shows
  the window and raises it (`SetWindowPos` `HWND_TOP`); `FUN_00429ce0` sets
  a new sector window to `HWND_BOTTOM`. Neither focuses nor activates it.
- **A click on a System window gives it the focus:** `FUN_004534f0` sends
  0x467 for itself on the press, which activates it unless it is already
  current, and calls `SetFocus` on the release.
- **A detail window's Escape** (`FUN_004ac3a0`, and `FUN_00458980` for the
  System window) calls the same slot `+0x30`; the view's 0x405 handler
  destroys the window and posts 0x467(0,0), so the next window shows and
  takes the focus. No visibility test stands on that path, so a window the
  menu hid closes unseen (code only).
- **A sector window has no Escape:** `FUN_004593e0` sends every
  `WM_KEYDOWN` to the view's procedure, and the view's falls through
  `FUN_005ff530` to `DefWindowProc`.

Evidence, under Wine:
- empire-fleet-to-yaga-minor loaded at day 38 restores the Sesswenna sector
  window and the Coruscant System window over it. With no GID press, Escape
  #1 closed Coruscant and #2 did nothing. With the first GID press (with or
  without a click in Coruscant's body first), Escape #1 showed Coruscant
  over the open menu, #2 closed it and showed Sesswenna, and #3 did nothing
  (days 38 to 45). Later in that game a double-click on Ghorman in the
  Sesswenna window opened its System window; with the menu open again,
  Escapes did nothing, and the press that closed the menu brought back
  Sesswenna and Ghorman.
- alliance-day54-advice loaded at day 61 restores no window: a galaxy
  double-click opened the Corellian sector window, then the first GID press,
  then Escape changed nothing.

port: `GalaxyFocusState` in `rebellion-render`'s `window_focus` holds the
focus (`GalaxyFocus` menu, child or view) and the current child, and its
`sync` hands the focus on after any closure, or a focus left on a window
that is gone, even in a frame where another window opens. Every child
window kind keeps a z serial (`window_focus::GalaxyChild`, `children()`,
`raise()`), and
`GidUiState::shown` lists the children shown again under the open menu,
which each family's draw filter lets through. `GidUiState::built` marks the
menu built; a load clears it, and the press that builds it gives the menu
the focus. `GalaxyFocusState::after_load` takes the windows a load restored
and focuses the top one that is not a sector window, or the view.

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
