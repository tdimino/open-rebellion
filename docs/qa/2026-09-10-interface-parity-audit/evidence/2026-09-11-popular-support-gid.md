---
title: "P47B Popular Support GID Evidence"
description: "Recovered active-GID backdrop, faction control, compact legend, native marker families, support thresholds, tests, and browser acceptance"
category: qa
created: 2026-09-11
updated: 2026-09-11
tags: [qa, interface, galaxy, gid, bitmap, wasm]
---

# P47B Popular Support GID Evidence

P47B restores the default active Popular Support view as a bounded `UIP-B02`
checkpoint. It removes the synthetic map labels and glyphs from that view and
uses recovered STRATEGY resources and predicates. This does not accept the
complete GID menu, every display mode, detailed legend, map interaction, or
`CMD-02` matrix.

## Recovered original contract

Static analysis of the original executable and an owned STRATEGY extraction
establishes this default view:

| Contract | Recovered value |
|---|---|
| Display Off backdrop | STRATEGY 902, bright 607x437 starfield |
| Every active GID backdrop | STRATEGY 903, dim 607x437 starfield |
| Alliance GID control | command `0x133`; logical `3,355,27,41`; normal 10013; pressed 10014 |
| Imperial GID control | command `0x133`; logical `79,192,35,57`; normal 10027; pressed 10028 |
| Compact legend | STRATEGY 10168, 47x25; Alliance `55,50`; Imperial `113,50` |
| Alliance markers | 10146 through 10149, largest through smallest |
| Imperial markers | 10150 through 10153, smallest through largest |
| Neutral markers | 10154 through 10157, smallest through largest |
| Unexplored or unpopulated marker | 10158 |
| Popular Support sizes | 81 through 100 largest; 60 through 80 large; 50 through 59 medium; below 50 smallest |

The manual on pages 68 through 71 corroborates the default Popular Support
view, marker color meaning, metric-based size, and Display Off behavior.
`FUN_00427010`, `FUN_00425d00`, `FUN_00426e70`, `FUN_00426ee0`,
`FUN_00427270`, and `FUN_0042b330` provide the executable-level paint,
resource, geometry, command, and threshold evidence.

The nine tall side controls are Message Index categories. They are not GID
filters. The actual faction GID control is the separate `0x133` control above.

## Implementation

- Active Popular Support selects STRATEGY 903. STRATEGY 902 is reserved for
  Display Off.
- Both faction controls use their exact bitmap pairs, recovered rectangles,
  native press capture, release cancellation, strict outer-edge rejection, and
  palette-key hit masks.
- The compact legend renders at its recovered faction position and natural
  size multiplied only by the shared 640x480 canvas scale.
- Original 15x15 marker bitmaps use recovered faction families and Popular
  Support thresholds. Their blue matte is transparent in both egui and
  Macroquad texture paths.
- Synthetic system and sector names, vector circles, selection glow, facility
  squares, fleet glyphs, fog patches, and blockade primitives are withheld
  from the parity surface until their original modes and resources are proven.
- System activation remains functional and opens the recovered bitmap sector
  window.

## Verification

| Gate | Result |
|---|---|
| Renderer tests | 145 passed, 0 failed |
| Workspace tests | 637 passed, 20 ignored, 0 failed |
| Packaged WASM | Built successfully; 5,150,139 bytes; SHA-256 `695ee4e5d49950da7fa0b6caf87d6583a3485dfddf0f5ff092046d70437d7f88` |
| Runtime pack | 52 game files, 2,303 bitmaps, 3,988 advisor frames, five audio files; SHA-256 `1ce8e2370d037423ad20683ff747245548b3eca453305596a45ef3083899a862` |
| Primary browser probe | Both factions passed at 640x480 and 1280x800; fresh loads used four HTTP 200 requests; no page or missing-asset errors |
| Audio safety | Every Chromium process launched with `--mute-audio`; in-game menu music was disabled before campaign entry |
| Astra medium | Ready to commit; no P0, P1, P2, or P3 product findings |

Astra independently passed four isolated Chrome for Testing campaigns: both
factions at 640x480 and 1280x800. Every campaign disabled menu music through
the UI before entry. Each fresh load made exactly four HTTP 200 requests for
the document, `gl.js`, WASM, and `runtime.orpk`, with no page errors, console
errors, failed requests, or missing-asset logs.

Normal and pressed faction controls and the compact legend matched their
packed source pixels. More than 99.49% of sampled map-background pixels matched
STRATEGY 903; the differences contained the native markers. Valid release
emitted one `0x133`, release outside cancelled, and all outer edges rejected at
640x480. At 1280x800, every nearest representable outer pixel rejected; exact
fractional edges cannot reach the integer WASM mouse interface and therefore
remain source-corroborated rather than runtime-proven. Visible systems opened
the bitmap Corellian sector window in all four runs.

Browser captures:

- [Alliance 640x480 default](p47b-popular-support-gid/independent-r2-alliance-640x480-default.png)
- [Alliance 640x480 pressed](p47b-popular-support-gid/independent-r2-alliance-640x480-gid-pressed.png)
- [Alliance 640x480 sector](p47b-popular-support-gid/independent-r2-alliance-640x480-sector.png)
- [Alliance 1280x800 default](p47b-popular-support-gid/independent-r2-alliance-1280x800-default.png)
- [Alliance 1280x800 pressed](p47b-popular-support-gid/independent-r2-alliance-1280x800-gid-pressed.png)
- [Alliance 1280x800 sector](p47b-popular-support-gid/independent-r2-alliance-1280x800-sector.png)
- [Empire 640x480 default](p47b-popular-support-gid/independent-r2-empire-640x480-fresh-default.png)
- [Empire 640x480 pressed](p47b-popular-support-gid/independent-r2-empire-640x480-fresh-gid-pressed.png)
- [Empire 640x480 sector](p47b-popular-support-gid/independent-r2-empire-640x480-fresh-sector.png)
- [Empire 1280x800 default](p47b-popular-support-gid/independent-r2-empire-1280x800-default.png)
- [Empire 1280x800 pressed](p47b-popular-support-gid/independent-r2-empire-1280x800-gid-pressed.png)
- [Empire 1280x800 sector](p47b-popular-support-gid/independent-r2-empire-1280x800-sector.png)

The seeded campaigns did not visibly exercise every marker size or a live
neutral system. The full resource families and selection predicates have
static and packed-resource proof; the captures prove only the variants that
appeared during these campaigns.

## Deliberately open

- the original code-built GID menu and its complete command tree
- the expanded 180x240 legend and every non-default GID filter
- Display Off selection through the original menu
- exact hover, selection, pan, zoom, and keyboard semantics
- original sector, fleet, mission, facility, resource, HQ, blockade, and
  intelligence overlays
- the authentic Message Index rail and every message category
- lossless A0 captures and the full native, browser, viewport, DPR, faction,
  content, and interaction matrix
- complete `CMD-02`, `CMD-09`, `UIP-B02`, and `UIP-T01` acceptance
