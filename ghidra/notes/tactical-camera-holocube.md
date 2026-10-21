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

## Capital meshes

The capital-ship mesh cache (`DAT_006bcbd4+0x198`, vtable `0x66c990`) loads
each mesh builder (`FUN_005d5b10`, slot `0x2c`) and scales it by
`DAT_0066d2b0` = 3.0 on all three axes (slot `0x34`) before any ship uses it.
`FUN_005d26c0` builds a ship's visual from those builders (`base`, `base + 1`,
`base + 2`) and keeps the medium builder's `GetBox` at `+0xcc`;
`FUN_005ab0e0` sizes the collision envelope from the box `FUN_005c1100`
returns, so the envelope is three times the file's bounds too.

`FUN_005d3770` picks a ship's detail from its depth along the camera's view
(camera frame `+0x14`..`+0x1c` dotted with the ship's position in the camera
frame) against `0x66d160` and `0x66d158` divided by the field (camera slot
`+0x14`, `FUN_005d98f0`, which returns `+0x48`); without High Detail Models
(`+0x688`) against `0x66d15c` alone.

## Fighter decals

`FUN_005d43c0` builds each group's close (`base`, origin 16,16, 256
colours), far (`base + 5`) and indicator (`4200 + base % 10`, origin 0,0, one
colour, opaque) decals, all `SetDecalSize(0.3, 0.3)` world units with decal
scaling on (`0x5d44e4` and alike). At the depth of A0 `TAC-05-C017`'s
Alliance lane, about 450 units, that is under a pixel, and the frame shows no
pixel there. `0x5d4af0` -> `FUN_005d4a60` sets the detail from the span, the
depth along the view times `tan(field * 0.5)` (`FUN_005d98f0`): with High
Detail Models close below 5, far from 5 to 10 and the indicator from 10
(`0x66d1d4`, `0x66d1d0`); without it, the indicator from 10 and far below.

A squadron starts in flight when `FUN_005b1b70` returns 1
(`0x5b1b70`-`0x5b1bb2`): `FUN_005b1bc0` (hyperdrive `+0x3ec` less damage
`+0x374`) and slot `+0x90` both non-zero; otherwise it docks
(`FUN_005964c0`, `0x5966cc`).

## Port

`tactical_assets.rs`: `OriginalTacticalCamera::pose` (`FUN_005d9640`, with
the yaw wrap), `scene_camera` and `scene_fovy` (the 3D window's projection),
`draw_holocube`, `scale_capital_mesh`, `original_capital_lod_view`,
`original_fighter_view_span` and `original_fighter_decal_pixels`.
`tactical_view.rs`: `TACTICAL_SCENE_WINDOW`, both highlights' start state and
`battle_paused_label_color`. port: the envelopes reach the battle from the
first frame drawn, and the paused formation is laid out again with them.

## Open: the 8-bit frame

A0 is an 8-bit paletted frame: 69 colours in the whole of `TAC-05-C019` and
24 in its aperture. Every 3D pixel goes through Direct3D RM's colour matching
into the display palette, its wireframe lines through its own line stepping
and its meshes through its software rasterizer and lights; `d3drm.dll` is a
system library outside the executable's trace (`or-d3drm-raster-match-vog4`).
Measured on `TAC-05-C019` against the port's frame of the same battle:

- Background. Direct3D RM draws the 440 by 438 scene background (resource
  5030, `SetSceneBackgroundImage`, frame slot `0xcc`) into the 439 by 437
  viewport at `(17, 30)`; stars land one to two pixels left of and up to one
  pixel below where an unscaled copy at `(18, 29)` puts them, some as 2 by 2
  blocks, and the palette maps the port's greys 24 to black and 40 and 58 to
  48.
- Holocube. The fitted projection puts the box's edges within a pixel of the
  frame's; the line pixels themselves differ where the two line steppers
  round apart.
- Ships. The meshes' shading and colours, and the hulls' positions in the
  Alliance formation.

C6 judges `TAC-05-C019`'s aperture on position alone (displacement, MAE and
changed pixels; the `split` in `tactical-a0-compare.json`); its SSIM and
edge IoU rows are reported and wait on that node.

## Decompiles

`FUN_005c1d30.c`, `FUN_005d9490.c`, `FUN_005d9620.c`, `FUN_005d9640.c`,
`FUN_005d4d10.c`, `FUN_005d4d10.asm.txt`, `FUN_005d58c0.c`, `FUN_0059eb70.c`,
`FUN_0059ef40.c`, `FUN_0059ef20.c`, `FUN_0059fd40.c`, `FUN_0059c4d0.c`,
`FUN_005c14d0.c`, `FUN_005d6700.c`, `FUN_005d6c10.c`, `FUN_005d9eb0.c`,
`FUN_005b1b70.c`, `FUN_005b1bc0.c`, `FUN_005d43c0.c`, `FUN_005d4a60.c`,
`FUN_005d98f0.c`, `FUN_005d5b10.c`, `FUN_005d26c0.c`, `FUN_005d3770.c`,
`FUN_005ab0e0.c`.
