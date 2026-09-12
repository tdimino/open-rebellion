---
title: "Original Interface Parity Audit"
description: "Authoritative screenshot corpus, surface ledger, findings, and acceptance gates for reproducing every visible Rebellion interface state"
category: qa
created: 2026-09-10
updated: 2026-09-11
tags: [qa, interface, parity, bitmap, screenshots, rebellion, supremacy]
---

# Original Interface Parity Audit

<!-- interface-parity-status:start -->
Required interface families: 43. Complete: 0. Partial: 9. Failing: 34. Blocked: 0. Strictly accepted cells: 0/564.
The current CMD-02 GID catalog has 38 scenarios (29 baseline cells and nine additional native filter variants) and 152 faction/viewport executions. Its strict original-evidence and cross-browser gate remains open.
<!-- interface-parity-status:end -->

## Progress snapshot

| Measure | Current position |
|---|---:|
| Practical interface scope materially tackled | approximately 20 to 23% |
| Practical interface scope remaining | approximately 77 to 80% |
| Required surface families | 43 |
| Family status | 0 complete, 9 partial, 34 fail |
| Strictly accepted required cells | 0 of 564 |

The practical percentage is an implementation-planning estimate. The strict
cell count remains the release authority because each cell still needs its
complete original-evidence, native, browser, viewport, faction, interaction,
audio, and diagnostic matrix. Work proceeds through the current
[batched execution plan](../../plans/2026-09-11-feat-batched-interface-parity-plan.md).

Open Rebellion does not yet have original interface parity. The shuttle main
menu's scoped composition, destinations, audio, and hotspots are verified, but
its exhaustive original control-state capture matrix remains open. The
strategic shell now uses the recovered faction bitmaps, exact apertures, and a
centered 640x480 canvas. Its six primary faction controls use their recovered
bitmap pairs, geometry, input masks, and command routing. First-pass original
sector and system shells replace the invented sidebar, with working tabs and
rail lifecycle for both factions. The original galaxy resources now back both
Display Off and the default active Popular Support view. Popular Support uses
the exact faction GID control, compact legend, native marker families, and
recovered size thresholds. Other GID modes, sector and object overlays,
incomplete system contents, approximate window thumbnails, authentic message
and status surfaces, and other controls remain open. The nine Message Index
side-rail resting icons per faction now match their original bitmaps exactly;
their unread states and destination window remain open.
Original advisor idle frames render in the scaled
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
| [Surface ledger](surface-ledger.json) | Stable inventory of every known original surface family, 564 required cells plus five excluded extension cells, retrieval-package links, and the derived execution contract. |
| [Screenshot ledger](screenshot-ledger.md) | Human-readable map of the 370 retained reference images and the still-missing owned-executable captures. |
| [Reference ledger](reference-ledger.json) | Source URLs, provenance, confidence, locale, local paths, and coverage. |
| [Reverse-engineering ledger](reverse-engineering-ledger.md) | Ghidra and original-resource findings, proof boundaries, and the implementation evidence queue. |
| [Reverse-engineering data](reverse-engineering-ledger.json) | Machine-readable artifacts, discoveries, resource omissions, and work-package status. |
| [P46A strategic shell evidence](evidence/2026-09-11-strategic-shell-canvas.md) | Durable canvas, aperture, shared-transform, test, artifact-hash, and Astra acceptance record. |
| [P46B strategic-control evidence](evidence/2026-09-11-strategic-command-controls.md) | Recovered six-control geometry, bitmap pairs, input masks, command mapping, tests, and browser acceptance. |
| [UIP-B01 strategic-navigation evidence](evidence/2026-09-11-strategic-window-navigation.md) | First-pass sector and system shells, tab resources, modeless lifecycle, rail routing, and browser evidence. |
| [P46D detailed-system evidence](evidence/2026-09-11-detailed-system-tab-items.md) | Source-mapped GOKRES tab items, displayed labels, bounded scrollbar navigation, intelligence gating, selection state, tests, and browser acceptance. |
| [P47A galaxy-backdrop evidence](evidence/2026-09-11-authored-galaxy-backdrop.md) | STRATEGY 902 identity, source-aligned placement, native/WASM texture transport, tests, captures, and open GID boundaries. |
| [P47B Popular Support evidence](evidence/2026-09-11-popular-support-gid.md) | Active STRATEGY 903 predicate, faction GID control, compact legend, native marker families, support thresholds, tests, and browser acceptance. |
| [P48 GID browser-harness evidence](evidence/2026-09-12-gid-browser-harness.md) | Code-built GID command tree, test-only fixture isolation, muted browser matrix, and unaccepted visual states. |
| [P49 Message Index rail evidence](evidence/2026-09-12-message-index-rail.md) | Original faction rail positions and BMPs, exact resting-pixel checks, and open category/index behavior. |
| [P50 GID menu-frame evidence](evidence/2026-09-12-gid-menu-frame.md) | Eight source frame tiles, instant menu display, source-pixel border checks, and remaining menu gaps. |
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
| Original system-window navigation | **Partial** |
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
The UIP-B01 checkpoint adds recovered modeless sector and system shells,
double-click routing, core tabs, and rail minimize/restore. Its
[evidence record](evidence/2026-09-11-strategic-window-navigation.md) keeps
active/inactive rail thumbnails and uncommon state coverage open. P46D adds
the first item composition for every core tab with GOKRES miniatures, displayed
labels, selection, bounded scrolling, foreground focus, and coarse
current-intelligence gating. Its
[evidence record](evidence/2026-09-11-detailed-system-tab-items.md) keeps nested
object contents, exact intelligence semantics, commands, uncommon states, and
complete A0 coverage open.
P47A starts UIP-B02 with the original bright STRATEGY 902 starfield at the
strategic canvas origin for both factions. P47B then proves that 902 is Display
Off and restores the default active Popular Support view with STRATEGY 903,
the exact faction GID control, compact legend, native marker families, and
support thresholds. Its
[evidence record](evidence/2026-09-11-popular-support-gid.md) keeps the detailed
legend, other modes and overlays, exact map input, and complete acceptance
matrix open. P48 restores the code-built GID command tree. P50 adds its eight
original frame tiles and removes the non-native fade. The
[frame evidence](evidence/2026-09-12-gid-menu-frame.md) leaves menu interior,
typography, and exact geometry open.
Runtime pack v2 now
includes all 3,988 ALSPRITE and EMSPRITE type-302 frames, while briefing,
tactical, dialog, encyclopedia, advisor-control, and voice resources remain
open. Its 27 packages give every required surface a named recovery or removal
path.

## What 100% means

The denominator is required surface-state cells, not an estimated percentage.
The ledger currently instantiates 564 required baseline cells and five excluded
extension cells. P46A through P47B verify scoped Chromium checkpoints, but all
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
2. original galaxy/starfield and Galactic Information Display modes; P47A
   restores the bright STRATEGY 902 Display Off backdrop, and P47B restores the
   default active Popular Support view with 903, the faction GID control,
   compact legend, and native markers; the menu, detailed legend, remaining
   modes and overlays, and exact map interactions remain open;
3. the six primary cockpit controls, state art, hotspots, and routing, completed
   by P46B; P47B adds the recovered GID control while speed controls and the
   GID destination menu remain open;
4. first-pass original sector and system shells, rail lifecycle, and core-tab
   GOKRES item compositions, completed as partial UIP-B01 checkpoints;
   remaining nested compositions, commands, rail thumbnails, and uncommon
   states;
5. complete Alliance and Imperial droid actions, BIN sequences, voice, chrome,
   and unobscured apertures on top of the packaged idle-frame foundation;
6. authoritative system, facility, fleet, HQ, blockade, and intelligence glyphs.

The tranche closes only after both factions pass native and packaged WASM at
required 4:3 and letterboxed widescreen viewports with no fake controls, blank
required apertures, missing resources, or browser errors.
