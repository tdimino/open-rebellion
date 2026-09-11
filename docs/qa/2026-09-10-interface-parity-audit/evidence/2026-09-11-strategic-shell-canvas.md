---
title: "P46A Strategic Shell Canvas Evidence"
description: "Recovered strategic shell geometry, shared transforms, tests, and browser acceptance"
category: qa-evidence
created: 2026-09-11
updated: 2026-09-11
tags: [qa, interface, strategy, cockpit, wasm, astra]
---

# P46A Strategic Shell Canvas Evidence

P46A passes its scoped interface gate. The strategic view now uses an exact
640x480 logical canvas, centered and uniformly scaled within the window.
STRATEGY resources 900 and 901 remain 640x481 sources. Rendering crops them to
the first 480 display rows instead of compressing the extra row.

## Implemented contract

- The Alliance galaxy aperture is `55,40,485,350` in logical `x,y,w,h` units.
- The Empire galaxy aperture is `120,40,480,355`.
- One centered canvas transform places the faction shell and advisor apertures.
- Its derived camera transform places galaxy systems, fog, fleets, routes,
  sectors, facilities, blockades, hover checks, selection, and context hits.
- One aperture clip contains all macroquad galaxy layers beneath the shell.

The implementation is in `cockpit.rs`, `lib.rs`, `advisor.rs`, `fog.rs`,
`fleet_movement.rs`, and the strategic composition path in `main.rs`.

## Verification

| Gate | Result |
|---|---|
| Focused rendering tests | 109 passed, 0 failed |
| Workspace tests from the primary agent | 601 passed, 0 failed |
| Packaged WASM SHA-256 | `24b093f29be0b8228a7747ebc4a700ba43cefb4b3bdf667c2de6c49c2520ef60` |
| Runtime pack SHA-256 | `1ce8e2370d037423ad20683ff747245548b3eca453305596a45ef3083899a862` |

The Astra medium r3 run completed 12 muted Chromium boots across both factions,
640x480, 960x720, and 1280x800, each at DPR 1 and 2. It also exercised
selection, hover boundaries, pan, zoom, resize-sensitive overlays, and advisor
animation. Every boot made exactly four requests: the document, `gl.js`, the
WASM module, and `runtime.orpk`.

R3 verified the shell geometry, 481-row crop, nearest sampling, aperture clips,
shared scaling, and interactions. It withheld final acceptance because no
active blockade was established. Astra medium r4 closed that gap through the
ordinary Empire interface: a fleet reached Alliance-controlled Yavin and
formed a real blockade. The ring stayed centered, circular, uniformly scaled,
and clipped at 640x480, 960x720, 1280x800, plus 960x720 at DPR 2.

The combined r3 and r4 result has no scoped P0, P1, P2, or P3 finding. All
browser boots used four requests and produced no console, request, page,
missing-asset, or WASM error.

A read-only review then corrected the aperture hit-test contract to use the
original rectangles' exclusive right and bottom edges. Unit coverage now
probes both included and excluded boundaries. The browser artifact was rebuilt
after that correction. Astra medium r5 launched both factions muted, passed all
24 included and excluded boundary probes at 640x480, reconfirmed centered
uniform scaling at 1280x800, and reported no browser or runtime error.
A final formatting-only rebuild changed the artifact hash. R5 then repeated 16
boundary smoke checks against the recorded artifact and passed both factions
without browser or runtime error.

The r3 and r4 directories under `/tmp` are temporary execution artifacts.
They supported inspection but are not repository acceptance dependencies. This
document preserves the durable result and hashes without adding screenshots or
original game assets.

## Remaining parity boundary

P46A closes only the canvas, shell, aperture, clipping, and shared-transform
checkpoint. `CMD-01` and `UIP-T01` remain incomplete. Authentic command
controls, GID and map art, the window-reference rail, original system windows,
replacement-sidebar removal, and original message and status surfaces remain
open.
