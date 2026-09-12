# Tactical shell and first control pass (P52)

Status: partial implementation evidence, not original-interface acceptance. All 106 `TAC-01` through `TAC-07` baseline cells remain pending.

## Original contract and resource map

The preserved [manual, pp. 139–150](../../../reference/campaign-history/archive/star-wars-rebellion-manual.pdf) and the [paused battle capture](../reference-captures/mobygames/tactical-paused-dauntless.webp) show a 640×480 tactical display that begins paused after Take Command. It has a battle aperture, a right control housing, top task-force and fighter-group strips, faction highlights, pause, and camera controls. The original capture is compressed, so it is provisional layout evidence rather than a lossless A0 baseline. The [space-battle asset inventory](../../../reference/asset-library/space-battle.md) identifies the extracted `TACTICAL.DLL` BMPs.

| Layer/control | TACTICAL BMP | Current logical position | Current action |
|---|---:|---|---|
| Full tactical shell | 1000 | (0, 0), 640×480 | Fixed original composition |
| Alliance/Imperial task-force header | 1001 / 1004 | (10, 1) | Display only; assignment is pending |
| Alliance/Imperial fighter header | 1008 / 1010 | (275, 1) | Display only; assignment is pending |
| Selected/empty ship panel | 1302 / 1301 | (481, 27) | Partial data; native model and gauges pending |
| Empire and Alliance highlight states | 1036/1037 and 1034/1035 | (482, 304) and (517, 304) | Toggle wireframe overlay |
| Pause/resume | 1061 / 1060 | (560, 307) | Toggle combat pause |
| Zoom in/out | 1044 / 1046 | (486, 343) / (603, 343) | Change battlefield zoom |
| Navigation and camera art | 1026–1033, 1048–1059 | Right control housing | Display only; original 3D camera semantics pending |

The small control overlays use a pure-blue transparency matte. The decoder now removes it only for the identified tactical control range; the selected-ship panel's authored blue fields stay opaque. Its source-pixel regression test distinguishes these cases.

## Implemented and tested

- `BattleSession::new` enters the tactical display in Combat, initially paused, rather than presenting the invented drag-placement screen. The battle state and production entry path remain shared with the campaign.
- The original 1000 BMP anchors one centered, letterboxed 640×480 canvas. Battlefield input is restricted to the original aperture, while right-side hit rectangles remain in native coordinates after scaling.
- The active combat screen no longer paints its synthetic egui top, side, and bottom command menus. The first original bitmap controls above are visible and their implemented actions work. The old result UI remains pending replacement.
- The isolated browser harness runs a fresh, muted Chrome process for each faction and viewport. Its corrected four-case run `2026-09-12T22-11-42-006Z-59286` passed 4/4 at 640×480 and 1280×800 letterboxed, with four successful requests per launch, zero console/page/missing-asset errors, stable initial and re-paused captures, and every browser closed. Native-size probes checked 708 unobscured shell pixels and 2,948 control-state pixels per faction against the extracted BMPs. They also verified an outside-edge pause miss, resume, re-pause, Alliance highlight toggle, zoom-in redraw, and exact isolation of every pixel outside the scaled battle aperture. The isolation probe covered 112,284 pixels per native execution and 483,060 per letterboxed execution.
- The Rust focused test gate passed 11 app and 153 renderer tests. The production and isolated fixture WASM builds, fixture validation, and production-fixture-exclusion check passed.
- Production WASM SHA-256: `b401799825b32d1007ea87a5a11fd0a09f405d8289cb9f078ba14b9596bd25d9`. Test-fixture WASM SHA-256: `d7c06b258858388eb8d35c519e6e80c7def8f9752cef500f1e9dcd28f5c861e7`. Runtime-pack SHA-256: `1ce8e2370d037423ad20683ff747245548b3eca453305596a45ef3083899a862`.

Astra medium's first read-only review found that center culling did not prevent
labels, health bars, beams, and targeting effects from escaping the aperture,
and that letterboxed interaction probes could be satisfied by continued battle
animation. The renderer now applies one hard scissor to every battlefield
layer and resets it before HUD composition. The harness now proves stable
re-pause before highlight and zoom probes, then compares every out-of-aperture
pixel across the zoom transition. The corrected captures show no shell leak.
Astra's second review inspected all four corrected runs, found both P2 issues
closed, reported no remaining P0–P2 finding, and approved this only as a scoped
partial-parity commit. It did not accept any `TAC-*` cell.

## Still open

The battle aperture still uses procedural stars, primitive ship/fighter markers, approximate health bars, and a two-dimensional transform. The original type-301 meshes, type-303 textures, planet backdrop, effects, selected-ship model, gauges, fighter and task-force assignments, navigation, camera rotation/tilt/memory, tactics, missions, options, Battle Alert, damage and destruction states, results composition, Death Star paths, and tactical audio need separate source mapping and implementation. Drawn but unwired bitmap controls are not counted as working controls. The four browser screenshots are unbaselined implementation captures, not A0 original comparisons.

Next: map the camera and selected-unit command subpanels from native constructors and original frames, stage the non-BMP tactical asset types, then expand the deterministic battle fixture before reconsidering any `TAC-*` cell.
