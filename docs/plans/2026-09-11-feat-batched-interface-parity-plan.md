---
title: "Batched Interface Parity Execution Plan"
description: "Current plan for restoring the original bitmap interface in coherent, independently accepted browser bundles"
category: plan
created: 2026-09-11
tags: [interface, parity, bitmap, browser, qa]
status: active
progress: "2% — 1 of 43 families strictly complete (OBJ-01); 15 partial, 27 failing; 13 of 627 cells accepted; no UIP-B01 to B07 exit gate met"
updated: 2026-10-15
---

# Batched Interface Parity Execution Plan

This is the current implementation plan for original interface parity. It
turns the [interface audit](../qa/2026-09-10-interface-parity-audit/) into
larger user-visible workflows so focused development can proceed without a
full WASM build, browser review, commit, and push after every small control.

The muted browser harness, test-only fixtures, visual comparisons and
generated status prose that execute this plan are documented in
[`tools/interface-parity/README.md`](../../tools/interface-parity/README.md).
They came from the
[acceleration sidecar](archive/2026-09-11-tooling-interface-parity-acceleration-sidecar.md),
now archived; its remaining CMD-02 pilot is a UIP-B02 residual item below.

## Progress snapshot

<!-- interface-parity-status:start -->
Required interface families: 43. Complete: 1. Partial: 15. Failing: 27. Blocked: 0. Strictly accepted cells: 13/627.
The current CMD-02 GID catalog has 38 scenarios (29 baseline cells and nine additional native filter variants) and 152 faction/viewport executions. Its strict original-evidence and cross-browser gate remains open.
<!-- interface-parity-status:end -->

| Measure | Current position |
|---|---:|
| Practical interface scope materially tackled | approximately 25% |
| Practical interface scope remaining | approximately 75% |
| Bounded practical space-battle launcher implementation | complete at P58-B22 |
| Required surface families | 43 |
| Family status | 1 complete (OBJ-01), 15 partial, 27 fail |
| Strictly accepted required cells | 13 of 627 |

The practical percentage is a planning estimate based on scoped implementation
checkpoints. It is not a release-parity score. A required cell passes only
after its complete original-evidence, native, browser, viewport, faction,
interaction, audio, and diagnostic matrix passes.

P58-B22 is the latest tactical checkpoint. It closes the bounded practical
launcher implementation with source-compatible tactical randomness, persisted
power allocation, completion and destruction callbacks, and the timed
source-derived trench-run producer. Deterministic A1 mapping is runnable at
106/106. The tactical C6 comparison against A0 passes 11 of the 106 cells;
the [106-cell acceptance train](2026-09-22-tooling-tactical-106-cell-acceptance.md)
owns the rest.

## Working cadence

Each bundle contains three to six tightly related interface changes that form
one original-game workflow.

1. Recover the original executable, resource, geometry, state, and routing
   contract before implementing a surface.
2. Implement one internal slice at a time and run focused tests after each
   slice.
3. Update the Markdown and JSON audit records while the implementation changes,
   with the primary agent authoring the documentation.
4. Build packaged WASM once the bundle is internally complete.
5. Run one end-to-end browser matrix across both factions and the required
   640x480 and letterboxed viewports.
6. Ask an independent reviewer to inspect the complete browser journey, bitmap identity,
   interactions, request log, console, and open parity boundaries.
7. Run one consolidated read-only code and documentation review at the bundle
   boundary when the change warrants independent review.
8. Run the full workspace suite, scoped clippy, ledger validator, JSON checks,
   artifact hashes, and diff checks once at the bundle boundary.
9. Commit and push the passing bundle with its code, screenshots, evidence,
   ledgers, roadmap, and progress update.

Small failing slices remain unmarked inside the active bundle. Completed slices
are not presented as fully accepted surface families until their required
matrix passes.

## Bundle queue

| Bundle | Original workflow | Included work | Exit gate |
|---|---|---|---|
| UIP-B01 | Strategic window navigation | Modeless sector windows, detailed system window and core tabs, double-click routing, focus and stacking, pointer occlusion, and first usable rail minimize/restore path | Both factions complete galaxy to sector to system to rail journeys without the invented sidebar |
| UIP-B02 | Galaxy and Galactic Information Display | Original starfield, systems, sector hulls, markers, filters, legends, hover, pan, zoom, selection, and faction variants | Required GID and galaxy states use original resources and rules at both viewport classes; the residual items below close |
| UIP-B03 | Command-center completion | Remaining bitmap controls, speed, messages, status, advisor chrome, authored actions, voice, and unobscured apertures | Both command centers contain no replacement strips, blank required apertures, or invented controls |
| UIP-B04 | Strategic management windows | Finders, personnel, fleets, facilities, production, research, missions, object status, and their legal, disabled, empty, populated, transit, and damage states | Each original strategic management journey is usable through its native window graph |
| UIP-B05 | Reports, events, encyclopedia, and endings | Dialog resources, event and battle reports, EData art, cutscenes, rare events, victory, defeat, skip, restart, and return routing | Every non-tactical campaign presentation family passes its required media and navigation matrix |
| UIP-B06 | Space-battle interface | Both tactical HUDs, units, assignments, selection, targeting, navigation, maneuvers, tactics, missions, damage, recovery, retreat, Death Star, trench run, results, and strategic return. Executed by the [106-cell acceptance train](2026-09-22-tooling-tactical-106-cell-acceptance.md) and br epic `or-ep-tac-x15`; references in the [launcher reference](../reference/space-battle-launcher/README.md) and [asset library](../reference/asset-library/README.md). | All 106 TAC-01 through TAC-07 baseline cells are implemented and accepted |
| UIP-B07 | Multiplayer and release acceptance | Original provider, host, join, ready, wait, sync, chat, pause, save/load, resign, disconnect, reconnect, errors, cross-browser, and A0 closure | Two real peers and every required interface cell pass the release matrix |

## Current state

The September P58 to P66 checkpoint narrative that stood here was retired on
2026-10-15. Git history through a76109b8 and the evidence files under the
[interface audit](../qa/2026-09-10-interface-parity-audit/) keep it. Since
then, interface work has run through these owners rather than through this
plan's bundles:

- UIP-B01 and UIP-B04 window behavior: the
  [modality and focus](2026-10-10-feat-strategic-window-modality-and-focus.md)
  and [windowing toolkit](2026-10-10-strategic-windowing-toolkit-implementation.md)
  plans (PR #24 to #45, last 9533b516).
- Single surfaces: full-functionality audit features, among them F-007C move
  orders, Build Selection and the Message Index (99d561ac).
- UIP-B06: the
  [106-cell acceptance train](2026-09-22-tooling-tactical-106-cell-acceptance.md)
  and br epic `or-ep-tac-x15`.

This plan keeps the bundle queue, the cadence and the 627-cell denominator.

### UIP-B02 residual items

These came from the archived
[acceleration sidecar](archive/2026-09-11-tooling-interface-parity-acceleration-sidecar.md)
and [P59 lane](archive/2026-09-14-feat-p59-parallel-gid-message-index-lane.md).

1. **GID claim table (P59B).** Recover the GID menu paint and layout
   (`FUN_00607740`), the non-support marker thresholds (`FUN_00426d00`), the
   caption and display-window construction with the 180x240 expanded legend
   (`FUN_00427010`), and the filter predicates for commands `0x11..0x80`
   (`FUN_00425d00`). Record them as a claim table in
   `ghidra/notes/gid-menu-contract.md` (not yet written;
   `ghidra/notes/gid-legend.md` covers the legend). Wire a resource only when
   the table proves both its predicate and its paint site.
2. **Fixture build guard (P59A0).** `scripts/build-interface-test-wasm.sh`
   still runs the production `build-wasm.sh` first. Add
   `OPEN_REBELLION_SKIP_PRODUCTION_WASM=1` so a fixture build leaves
   `web/open-rebellion.wasm` unchanged.
3. **CMD-02 family gate (ACC-06).** One gate over all 29 baseline cells:
   galaxy, sector and system views, Display Off, every filter mode with its
   legend, the intelligence states, and hover, selection, pan and zoom, for
   both factions at 640x480 and the letterboxed viewport, native and packaged
   WASM. The GID catalog's 38 scenarios are the browser input.

## Bundle acceptance record

For each bundle, its evidence file must record:

- original functions, resources, geometry, and unresolved semantics;
- focused and workspace test counts;
- native and packaged-WASM results;
- artifact and runtime-pack hashes;
- browser screenshots for both factions and viewport classes;
- browser-review findings and disposition;
- independent code and documentation review;
- remaining cells that were deliberately not claimed;
- the commit and pushed branch containing the accepted checkpoint.

The [surface ledger](../qa/2026-09-10-interface-parity-audit/2026-10-06-surface-ledger.json)
remains the acceptance denominator. This plan controls execution cadence, not
the meaning of parity.
