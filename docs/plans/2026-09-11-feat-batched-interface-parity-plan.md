---
title: "Batched Interface Parity Execution Plan"
description: "Current plan for restoring the original bitmap interface in coherent, independently accepted browser bundles"
category: plan
created: 2026-09-11
updated: 2026-09-11
tags: [interface, parity, bitmap, browser, astra, qa]
status: active
---

# Batched Interface Parity Execution Plan

This is the current implementation plan for original interface parity. It
turns the [interface audit](../qa/2026-09-10-interface-parity-audit/) into
larger user-visible workflows so focused development can proceed without a
full WASM build, Astra review, commit, and push after every small control.

## Progress snapshot

| Measure | Current position |
|---|---:|
| Practical interface scope materially tackled | approximately 20 to 23% |
| Practical interface scope remaining | approximately 77 to 80% |
| Required surface families | 43 |
| Family status | 0 complete, 9 partial, 34 fail |
| Strictly accepted required cells | 0 of 564 |

The practical percentage is a planning estimate based on scoped implementation
checkpoints. It is not a release-parity score. A required cell passes only
after its complete original-evidence, native, browser, viewport, faction,
interaction, audio, and diagnostic matrix passes.

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
6. Ask Astra medium to inspect the complete browser journey, bitmap identity,
   interactions, request log, console, and open parity boundaries.
7. Ask independent subagents for read-only code and documentation review.
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
| UIP-B02 | Galaxy and Galactic Information Display | Original starfield, systems, sector hulls, markers, filters, legends, hover, pan, zoom, selection, and faction variants | Required GID and galaxy states use original resources and rules at both viewport classes |
| UIP-B03 | Command-center completion | Remaining bitmap controls, speed, messages, status, advisor chrome, authored actions, voice, and unobscured apertures | Both command centers contain no replacement strips, blank required apertures, or invented controls |
| UIP-B04 | Strategic management windows | Finders, personnel, fleets, facilities, production, research, missions, object status, and their legal, disabled, empty, populated, transit, and damage states | Each original strategic management journey is usable through its native window graph |
| UIP-B05 | Reports, events, encyclopedia, and endings | Dialog resources, event and battle reports, EData art, cutscenes, rare events, victory, defeat, skip, restart, and return routing | Every non-tactical campaign presentation family passes its required media and navigation matrix |
| UIP-B06 | Space-battle interface | Both tactical HUDs, units, assignments, selection, targeting, navigation, maneuvers, tactics, missions, damage, recovery, retreat, Death Star, trench run, results, and strategic return | All 106 TAC-01 through TAC-07 baseline cells are implemented and accepted |
| UIP-B07 | Multiplayer and release acceptance | Original provider, host, join, ready, wait, sync, chat, pause, save/load, resign, disconnect, reconnect, errors, cross-browser, and A0 closure | Two real peers and every required interface cell pass the release matrix |

## Current execution focus: P47A accepted; UIP-B02 continues

P46C established the first UIP-B01 checkpoint. Selecting a galaxy system opens
the recovered 235x360 modeless sector window instead of the invented right
sidebar. Original planet resources, placement, faction columns, controls,
stacking, pointer boundaries, system double-click routing, the 226x304 system
shell, and both faction rail geometries are active.

P46D adds source-mapped GOKRES miniatures and displayed entity labels to all
six system tabs, including personnel, fleets, defenses, manufacturing, troops,
special forces, and production. Three-column overflow is bounded by the
recovered STRATEGY scrollbar art. Selection, tab clearing, foreground focus,
rail restoration, and coarse current-intelligence gating are preserved. Both
checkpoints pass their scoped browser journeys for both factions at 640x480
and a responsive viewport.

UIP-B01 remains open for nested fleet and object compositions, drag and command
behavior, exact hostile-intelligence semantics, active and inactive rail
thumbnails, complete sector indicators, uncommon system states, and the A0
matrix. No pending cell is promoted solely by these partial checkpoints.

The evidence-blocked rail art and uncommon-state work remains queued inside
UIP-B01. P47A starts the independently provable `UIP-B02` map slice by drawing
the original STRATEGY 902 bright galaxy at the 640x480 canvas origin beneath
all strategic layers. Native and packaged WASM use the same bytes and
nearest-neighbor sampling. STRATEGY 903, original markers, sector hulls, GID
filters and legends, and exact pan, zoom, hover, and selection behavior remain
open. Its renderer, workspace, packaged WASM, parity-ledger, and muted
two-faction Astra gates pass with no findings. See the
[P47A evidence record](../qa/2026-09-10-interface-parity-audit/evidence/2026-09-11-authored-galaxy-backdrop.md).

The next `UIP-B02` slice recovers the original marker and sector-display
resources and their state predicates before replacing any remaining synthetic
map glyphs. STRATEGY 903 stays unwired until its original trigger is proven.

## Bundle acceptance record

For each bundle, its evidence file must record:

- original functions, resources, geometry, and unresolved semantics;
- focused and workspace test counts;
- native and packaged-WASM results;
- artifact and runtime-pack hashes;
- browser screenshots for both factions and viewport classes;
- Astra findings and disposition;
- independent code and documentation review;
- remaining cells that were deliberately not claimed;
- the commit and pushed branch containing the accepted checkpoint.

The [surface ledger](../qa/2026-09-10-interface-parity-audit/surface-ledger.json)
remains the acceptance denominator. This plan controls execution cadence, not
the meaning of parity.
