---
title: "P47A Authored Galaxy Backdrop Evidence"
description: "Original STRATEGY 902 starfield identity, placement, native and WASM transport, tests, browser captures, and open GID boundaries"
category: qa
created: 2026-09-11
updated: 2026-09-11
tags: [qa, interface, galaxy, gid, bitmap, wasm]
---

# P47A Authored Galaxy Backdrop Evidence

P47A begins `UIP-B02` by replacing the flat synthetic strategic-map fill with
the original bright galaxy bitmap from STRATEGY resource 902. System markers,
sector hulls, facility indicators, GID filters and legends, and exact map input
semantics remain open. This is a partial `CMD-02` checkpoint, not acceptance of
the full galaxy display.

## Original evidence

The owned STRATEGY extraction identifies four large command-center resources:

| Resource | Dimensions | Current identity |
|---|---:|---|
| 900 | 640x481 | Alliance command-center shell |
| 901 | 640x481 | Imperial command-center shell |
| 902 | 607x437 | Bright authored galaxy starfield |
| 903 | 607x437 | Dim authored galaxy starfield used by every active GID mode; proven by P47B |

Resource 902 has SHA-256
`89188825234d29a14391f814f5dc6d6a593f6da608b46b2628cd512f9e44fb17`.
The retained original-game
[Alliance command-center frame](../reference-captures/pravus-part1-video-frames/0121-alliance-galaxy-command-center.png)
shows the same bright starfield. Its registration also resolves the placement:
the 607x437 bitmap begins at the 640x480 strategic canvas origin. The shell
reveals a source-aligned crop through the recovered Alliance `55,40,485,350`
or Imperial `120,40,480,355` aperture. The bitmap is not stretched to either
opening.

P47B subsequently recovered the exact predicate: resource 903 is used by every
active GID mode, while 902 is the Display Off backdrop. See the
[Popular Support evidence](2026-09-11-popular-support-gid.md).

## Implementation

- `BmpCache::get_macroquad_original` decodes original BMP bytes into a cached
  Macroquad texture for layers that must appear beneath egui chrome.
- Native loads 902 from the staged owned extraction. WASM loads the same bytes
  from the deterministic runtime pack.
- Original-parity sampling is nearest-neighbor. Deep-blue starfield pixels stay
  opaque and are not processed as shell transparency.
- `draw_galaxy_backdrop` first paints a dark fail-closed aperture, then draws
  902 at canvas origin and native resource size multiplied only by the uniform
  640x480 canvas scale.
- Existing strategic primitives paint above the authored image. The faction
  shell and modeless windows paint afterward, preserving the established layer
  order and clipping.
- Missing or invalid original bytes are negatively cached and logged once per
  resource key.

## Verification

| Gate | Result |
|---|---|
| Renderer tests | 140 passed, 0 failed |
| Workspace tests | 632 passed, 20 ignored, 0 failed |
| Scoped clippy | Passed with audited pre-existing warnings |
| Packaged WASM | Built successfully; 5,163,402 bytes; SHA-256 `bda25fc33c56ad453bd8ac477aa9dd56b7457d2c6163e8b9eab19cdbac7af0b3` |
| Runtime pack | 52 game files, 2,303 bitmaps, 3,988 advisor frames, five audio files; SHA-256 `1ce8e2370d037423ad20683ff747245548b3eca453305596a45ef3083899a862` |
| Primary browser probe | Both factions at 640x480 and 1000x700; two four-request HTTP 200 starts; no page errors |
| Audio safety | Chromium launched with `--mute-audio`; in-game `music_enabled=false` confirmed before both campaigns |
| Astra medium | Ready to commit; no P0, P1, P2, or P3 findings |

Browser captures:

- [Alliance 640x480](p47a-galaxy-backdrop/alliance-640.png)
- [Alliance 1000x700](p47a-galaxy-backdrop/alliance-1000.png)
- [Empire 640x480](p47a-galaxy-backdrop/empire-640.png)
- [Empire 1000x700](p47a-galaxy-backdrop/empire-1000.png)

Astra independently repeated both faction launches at 640x480 and 1000x700 in
a fresh isolated Chromium process with `--mute-audio`. Menu music was disabled
before each campaign. All eight startup requests returned HTTP 200, with no
failed requests, console errors, page errors, missing-asset reports, or
WebGL/WASM errors. Pan, zoom, sector opening, and detailed-system opening all
passed. Pixel comparison found approximately 87 to 90 percent exact source
matches inside the apertures, including the deep-blue field; strategic overlays
account for the remaining changed pixels. The packed 902 bytes exactly match
the owned source resource.

## Deliberately open

- every non-default GID filter, the original GID menu, and expanded legend;
  P47B restores the active 903 predicate, default Popular Support markers, and
  compact legend
- original system, fleet, mission, HQ, blockade, intelligence, facility, and
  resource markers
- exact sector hulls, labels, hover and selection art, pan and zoom rules
- lossless A0 captures and the full native/browser viewport and DPR matrix
- complete `CMD-01`, `CMD-02`, `CMD-09`, `UIP-B02`, and `UIP-T01` acceptance
