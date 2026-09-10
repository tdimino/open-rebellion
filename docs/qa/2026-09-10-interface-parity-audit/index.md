---
title: "Original Interface Parity Audit"
description: "Authoritative screenshot corpus, surface ledger, findings, and acceptance gates for reproducing every visible Rebellion interface state"
category: qa
created: 2026-09-10
updated: 2026-09-10
tags: [qa, interface, parity, bitmap, screenshots, rebellion, supremacy]
---

# Original Interface Parity Audit

Open Rebellion does not yet have original interface parity. The shuttle main
menu's scoped composition, destinations, audio, and hotspots are verified, but
its exhaustive original control-state capture matrix remains open. The current strategy screen uses
replacement controls, a synthetic galaxy drawing, an invented system sidebar,
and blank browser advisor apertures. Those are hard failures under this audit.

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
| [Surface ledger](surface-ledger.json) | Stable inventory of every known original surface family and its required visual states. |
| [Screenshot ledger](screenshot-ledger.md) | Human-readable map of the 370 retained reference images and the still-missing owned-executable captures. |
| [Reference ledger](reference-ledger.json) | Source URLs, provenance, confidence, locale, local paths, and coverage. |
| [Reverse-engineering ledger](reverse-engineering-ledger.md) | Ghidra and original-resource findings, proof boundaries, and the implementation evidence queue. |
| [Reverse-engineering data](reverse-engineering-ledger.json) | Machine-readable artifacts, discoveries, resource omissions, and work-package status. |
| [JSON Schema](schemas/interface-parity.schema.json) | Validation contract for the audit summary and status vocabulary. |
| [Reference captures](reference-captures/) | Locally retained manual pages and classified original-game screenshots. |
| [SHA-256 manifest](reference-captures/SHA256SUMS) | Integrity manifest for all 370 retained images. |

## Current result

| Gate | Result |
|------|--------|
| Original surface families inventoried | 43 |
| Reference images retained | 370 |
| Required surface families not at complete parity | 42 |
| Scoped shuttle implementation | **Pass** |
| Complete shuttle state matrix | **Partial** |
| Strategy cockpit parity | **Fail** |
| Browser advisor visibility | **Fail** |
| Original system-window navigation | **Fail** |
| Release claim of 100% interface parity | **Blocked** |

The 370-image corpus is broad, but it is not the final acceptance set. Several
rare or transient states cannot be sourced responsibly or at sufficient quality
from the web. They remain explicit capture gaps and must be recorded from a
legally owned, unmodified English installation before their cells can pass.

Static recovery is tracked separately in the
[reverse-engineering ledger](reverse-engineering-ledger.md). It already provides
authoritative geometry and routing for the strategic shell, but also shows that
the browser pack omits thousands of original advisor, briefing, and tactical
resources.

## What 100% means

The denominator is required surface-state cells, not an estimated percentage.
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

1. exact 640×480 display composition and uniform scaling, with any 481-row
   source bitmap or capture mapped to its authoritative crop rather than stretched;
2. original galaxy/starfield and Galactic Information Display modes;
3. original cockpit controls, state art, hotspots, and window-reference rail;
4. original sector and system windows instead of the right sidebar;
5. packaged Alliance and Imperial droid frames, BIN sequences, voice, and chrome;
6. authoritative system, facility, fleet, HQ, blockade, and intelligence glyphs.

The tranche closes only after both factions pass native and packaged WASM at
required 4:3 and letterboxed widescreen viewports with no fake controls, blank
required apertures, missing resources, or browser errors.
