---
title: "Original Interface Parity Audit"
description: "Authoritative screenshot corpus, surface ledger, findings, and acceptance gates for reproducing every visible Rebellion interface state"
category: qa
created: 2026-09-10
updated: 2026-09-11
tags: [qa, interface, parity, bitmap, screenshots, rebellion, supremacy]
---

# Original Interface Parity Audit

Open Rebellion does not yet have original interface parity. The shuttle main
menu's scoped composition, destinations, audio, and hotspots are verified, but
its exhaustive original control-state capture matrix remains open. The
strategic shell now uses the recovered faction bitmaps, exact apertures, and a
centered 640x480 canvas. Its six primary faction controls now use their recovered
bitmap pairs, geometry, input masks, and command routing. The strategy screen
still uses synthetic GID and map art, an invented system sidebar, replacement
message and status surfaces, and other incomplete controls. Original advisor idle frames render in the scaled
apertures, but their authored actions, voice, and chrome remain open.

This bundle is the visual and interaction-identity sister to the
[full-functionality audit](../2026-09-08-full-functionality-audit/). The older
audit remains authoritative for simulation behavior. This audit is authoritative
for original surface composition, bitmap identity, geometry, hotspots, window
navigation, animation, audio presentation, and the absence of invented UI.

## Files

| File | Purpose |
|------|---------|
| [Audit report](audit-report.md) | Findings, acceptance rules, immediate implementation order, and the answer to why the current UI looks synthetic or blank. |
| [Audit data](audit-report.json) | Machine-readable baseline, thresholds, findings, tranches, and release gates. |
| [Surface ledger](surface-ledger.json) | Stable inventory of every known original surface family, 569 baseline cells, retrieval-package links, and the derived execution contract. |
| [Screenshot ledger](screenshot-ledger.md) | Human-readable map of the 370 retained reference images and the still-missing owned-executable captures. |
| [Reference ledger](reference-ledger.json) | Source URLs, provenance, confidence, locale, local paths, and coverage. |
| [Reverse-engineering ledger](reverse-engineering-ledger.md) | Ghidra and original-resource findings, proof boundaries, and the implementation evidence queue. |
| [Reverse-engineering data](reverse-engineering-ledger.json) | Machine-readable artifacts, discoveries, resource omissions, and work-package status. |
| [P46A strategic shell evidence](evidence/2026-09-11-strategic-shell-canvas.md) | Durable canvas, aperture, shared-transform, test, artifact-hash, and Astra acceptance record. |
| [P46B strategic-control evidence](evidence/2026-09-11-strategic-command-controls.md) | Recovered six-control geometry, bitmap pairs, input masks, command mapping, tests, and browser acceptance. |
| [JSON Schema](schemas/interface-parity.schema.json) | Validation contract for the audit summary and status vocabulary. |
| [Ledger validator](../../../scripts/validate-interface-parity-ledgers.mjs) | Dependency-free generator and consistency gate for cell IDs and retrieval-package links. |
| [Reference captures](reference-captures/) | Locally retained manual pages and classified original-game screenshots. |
| [SHA-256 manifest](reference-captures/SHA256SUMS) | Integrity manifest for all 370 retained images. |

## Current result

| Gate | Result |
|------|--------|
| Original surface families inventoried | 44 |
| Required baseline acceptance cells | 564 |
| Required baseline cells pending or open | 564 |
| Required families with retrieval packages | 43 of 43 |
| Space-battle baseline cells | 106 across TAC-01 through TAC-07 |
| Reference images retained | 370 |
| Required surface families not at complete parity | 43 |
| Scoped shuttle implementation | **Pass** |
| Complete shuttle state matrix | **Partial** |
| Strategy cockpit parity | **Partial** |
| Browser advisor visibility | **Partial** |
| Original system-window navigation | **Fail** |
| Release claim of 100% interface parity | **Blocked** |

The 370-image corpus is broad, but it is not the final acceptance set. Several
rare or transient states cannot be sourced responsibly or at sufficient quality
from the web. They remain explicit capture gaps and must be recorded from a
legally owned, unmodified English installation before their cells can pass.

Static recovery is tracked separately in the
[reverse-engineering ledger](reverse-engineering-ledger.md). It already provides
authoritative geometry and routing for the strategic shell. P46A implements and
verifies the shell, crop, apertures, clipping, and shared transform in
[its durable evidence](evidence/2026-09-11-strategic-shell-canvas.md). P46B
implements the six primary faction controls from the recovered constructor,
paint, capture, hit-mask, and routing paths. Its durable evidence is
[recorded separately](evidence/2026-09-11-strategic-command-controls.md).
Runtime pack v2 now
includes all 3,988 ALSPRITE and EMSPRITE type-302 frames, while briefing,
tactical, dialog, encyclopedia, advisor-control, and voice resources remain
open. Its 27 packages give every required surface a named recovery or removal
path.

## What 100% means

The denominator is required surface-state cells, not an estimated percentage.
The ledger currently instantiates 564 required baseline cells and five excluded
extension cells. P46A and P46B verify scoped Chromium checkpoints, but all
baseline cells remain pending until their complete execution matrix passes.
Compound requirements split into stable child cells before
their implementation starts, and each cell then runs through its applicable
original, native, browser, viewport, DPR, faction, content, and interaction
matrix.
A cell passes only when it has authoritative original evidence, exact resource
identity, matching layout and interaction geometry, native and browser captures,
clean runtime diagnostics, and correct audio where applicable. One failed,
blocked, partial, or unknown required cell prevents its parent surface and the
project-wide interface gate from passing.

Original parity mode permits no visible invented or unknown elements. The one
approved main-menu music control is tracked as an isolated extension outside the
parity denominator.

## First implementation tranche

`UIP-T01`, Strategic Cockpit Truth, replaces the current strategy composition in
this order:

1. exact 640×480 display composition and uniform scaling, with the 481-row
   faction sources cropped to 480 display rows, completed by P46A;
2. original galaxy/starfield and Galactic Information Display modes;
3. the six primary cockpit controls, state art, hotspots, and routing, completed
   by P46B; remaining speed and GID controls plus the window-reference rail;
4. original sector and system windows instead of the right sidebar;
5. complete Alliance and Imperial droid actions, BIN sequences, voice, chrome,
   and unobscured apertures on top of the packaged idle-frame foundation;
6. authoritative system, facility, fleet, HQ, blockade, and intelligence glyphs.

The tranche closes only after both factions pass native and packaged WASM at
required 4:3 and letterboxed widescreen viewports with no fake controls, blank
required apertures, missing resources, or browser errors.
