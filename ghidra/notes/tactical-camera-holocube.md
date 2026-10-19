---
title: "Tactical Camera, Holocube, Highlights and the Paused Label"
description: "The tactical camera's pose (FUN_005d9640) and viewport (FUN_005c14d0, FUN_005d9eb0), the blue wireframe battle box (FUN_005d4d10, FUN_005d58c0), the highlight switches' start state (FUN_0059eb70, FUN_0059c4d0) and the Battle Paused label's colour (FUN_005c14d0)"
category: "ghidra"
tags: [tactical, camera, holocube, highlight, label]
---

# Tactical camera, holocube, highlights and the paused label

Coordinates are the original 640x480 canvas; the tactical aperture is
`(16, 28)` 444 by 439.

## Camera

`FUN_005c1d30` builds the camera (`FUN_005d9490`) at `1.7` battle extents
(`_DAT_0066c958`) and sets pitch 30 and yaw 150 for side 1 (the Empire) or
-30 for side 0 (`FUN_005d9620(camera, 0x1e, yaw, 0.2)`; `FUN_00595c40`), and
field 0.2. `FUN_005d9640` poses the camera frame each time: `AddRotation`
replace about Y by the yaw, `AddRotation` before about X by the pitch,
`AddTranslation` before by `(0, 0, -distance)`, then `SetField` and
`SetFront(1.0)`; with a target object it adds `LookAt`. In Direct3D RM's row
vectors the camera sits at `(0, 0, -d) * Rx(pitch) * Ry(yaw)`, that is
`(-d cos p sin y, d sin p, -d cos p cos y)`, looking along
`(cos p sin y, -sin p, cos p cos y)` with up `(sin p sin y, cos p, sin p cos y)`.
Each side's camera stands above and behind its own line: side 1's ships
are placed at `+0.5` extent on Z and side 0's at `-0.5` (`FUN_005ab650`).

## Viewport

The tactical window builds its "3D WINDOW" at `(17, 30)`, 439 by 437
(`0x59abbf`-`0x59abcb` -> `FUN_005c14d0` -> `FUN_005d6700`). The device's
surfaces take the window's client rect (`FUN_005d6c10`, `GetClientRect`), and
`FUN_005c1d30` gives `FUN_005d9490` a viewport rect of zeros, which
`FUN_005d9eb0` widens to the device's width and height before
`IDirect3DRM::CreateViewport` (slot `0x50`). Direct3D puts pixel centres on
integer coordinates. Fitted to the holocube's edge pixels in the two A0 frames
below, the projection's centre is the window's centre, `(236.5, 248.5)`, half a
pixel right of and below OpenGL's, and its scale is 219 pixels per 0.2 of field
at unit depth, `(439 + 437) / 4`; the retained-mode library's projection
formula is not in the executable, so that scale is fitted (0.28 px RMS over
1,335 pixels).

## Holocube

`FUN_005d4d10` builds two mesh builders (`+0x3c`, `+0x40`) with
`IDirect3DRM::CreateMeshBuilder` and gives each the unit cube's eight
corners, `(+-0.5, +-0.5, +-0.5)` (`0x5d4d77`-`0x5d4e35`), and six quads
(`0x5d4e40`-`0x5d4f24`: `0 3 2 1`, `0 4 7 3`, `0 1 5 4`, `6 7 4 5`,
`6 5 1 2`, `6 2 3 7`), the second builder with the opposite winding, through
`AddFaces` (`0x5d51be`, `0x5d5248`). It scales both by the battle extent
(`Scale`, `0x5d51da`), colours them `SetColorRGB(0, 0, 1.0)` (`0x5d51f3`) and
sets quality `0x40`, an unlit wireframe (`0x5d5207`). `FUN_005d58c0` adds both
to the scene frame once (`+0x68`) when option bit `0x02000000` is set
(`FUN_00401a00`), the Game Options' Display Holocube (`game-options.md`).

A0 `TAC-05-C017` (`empire-death-star-pre-alert`, Empire) and the paused frame
of `TAC-05-C023` (`alliance-day1169-deathstar-trench-run`, Alliance) draw the
box in pure blue `(0, 0, 255)`, one pixel wide, with three edges meeting at
`(157, 290)`: one rising, one running down-left at slope -0.82 and one
down-right at 0.30. The camera and cube above project to that corner and
those slopes on both sides.

## Highlights

The controller's constructor zeroes both highlight flags, `+0x10c` (Alliance)
and `+0x110` (Empire) (`FUN_0059eb70`); commands `0x14` and `0x15` toggle
them (`FUN_0059fd40` -> `FUN_0059ef40`, `FUN_0059ef20`). `FUN_0059c4d0`
builds the switches in their first state: `0xad16` at `(515, 305)` 30 by 26
with 1034/1035, hover text TEXTTACT `0xada6` "Highlight Alliance Ships", and
`0xad17` at `(483, 305)` with 1036/1037, `0xada8` "Highlight Empire Ships".
Both A0 frames above open with both switches unlit.

## Battle Paused

`FUN_005c14d0` builds label `0xddf0` (TEXTTACT " Battle Paused") in palette
red `0x020000ff` when the local side (`FUN_00595c40`) is 0, the Alliance,
and palette green `0x0200ff00` otherwise; label `0xddf1` takes the other
colour, and `0xddf2` `0x02ffff00`. A0 `TAC-05-C009` (Alliance) reads red and
`TAC-05-C017` (Empire) green.

## Port

The port does not use the camera pose, the 3D window's projection or the
holocube yet: the tactical fixtures were written against its own camera, and
`or-p-tactical-camera-viewport-aw39` re-lands them with those fixtures
updated. Both highlights' start state and the Battle Paused colour are ported
(`tactical_view.rs`: `TacticalState`'s defaults, `battle_paused_label_color`).

## Open: participant presentation

Measured against A0 `TAC-05-C017`, where the camera and layout above put every
capital ship where the frame shows it:

- Squadrons in flight. A squadron starts in flight when `FUN_005b1b70`
  returns 1 (`0x5b1b70`-`0x5b1bb2`): `FUN_005b1bc0` (hyperdrive `+0x3ec` less
  damage `+0x374`) and slot `+0x90` both non-zero; otherwise it docks
  (`FUN_005964c0`, `0x5966cc`).
- Fighter decals. `FUN_005d43c0` builds each group's close (`base`, origin
  16,16, 256 colours), far (`base + 5`) and indicator (`4200 + base % 10`,
  origin 0,0, one colour, opaque) decals, all `SetDecalSize(0.3, 0.3)` world
  units with decal scaling on (`0x5d44e4` and alike). At the C017 battle's
  depth (about 450) that is under a pixel; A0 shows no pixel on the Alliance
  squadrons' lane.
- Fighter detail (`0x5d4af0` -> `FUN_005d4a60`): the span is the depth along
  the view times `tan(field * 0.5)` (camera slot `+0x14`, `FUN_005d98f0`, the
  field at `+0x48`). With High Detail Models (`+0x688`) the state is close
  below 5, far from 5 to 10 and the indicator from 10 (`0x66d1d4`,
  `0x66d1d0`); without it, the indicator from 10 and far below.
- Capital detail (`FUN_005d3770`): the depth along the view against the
  thresholds `0x66d160` and `0x66d158` divided by the field.
- The strike effect and per-mesh pixels.

## Decompiles

`FUN_005c1d30.c`, `FUN_005d9490.c`, `FUN_005d9620.c`, `FUN_005d9640.c`,
`FUN_005d4d10.c`, `FUN_005d4d10.asm.txt`, `FUN_005d58c0.c`, `FUN_0059eb70.c`,
`FUN_0059ef40.c`, `FUN_0059ef20.c`, `FUN_0059fd40.c`, `FUN_0059c4d0.c`,
`FUN_005c14d0.c`, `FUN_005d6700.c`, `FUN_005d6c10.c`, `FUN_005d9eb0.c`,
`FUN_005b1b70.c`, `FUN_005b1bc0.c`, `FUN_005d43c0.c`, `FUN_005d4a60.c`,
`FUN_005d98f0.c`.
