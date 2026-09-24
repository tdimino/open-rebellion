---
title: "Full Functionality Audit Index"
description: "Entry point for the September 2026 Open Rebellion functionality, parity, and bitmap audit"
category: qa
created: 2026-09-08
updated: 2026-09-14
tags: [qa, audit, functionality, parity, bitmap, astra, fable]
---

# Open Rebellion Full Functionality Audit

This bundle records the repository baseline at commit
`fc25634be905839dfa6fb477d5fff0faa49d8ae9` and defines the feature-by-feature
acceptance plan required before the project can claim 100% functionality.

## Files

| File | Purpose |
|------|---------|
| [Audit report](audit-report.md) | Human-readable findings, verification results, Astra/Fable reviews, optimization roadmap, feature matrix, and bitmap protocol. |
| [Audit data](audit-report.json) | Machine-readable baseline, findings, optimization milestones, feature passes, and release gates. |
| [Original interface parity audit](../2026-09-10-interface-parity-audit/) | Sister audit for all original surface families, screenshot evidence, bitmap identity, geometry, navigation, and the prohibition on invented visible UI. |
| [Cockpit routing correction](../2026-09-10-interface-parity-audit/evidence/2026-09-14-cockpit-routing-correction.md) | PR #11 command, bitmap, geometry, keyboard, GID-toggle, browser, and fail-closed destination evidence. |
| [Original campaign-history baseline](../../reference/campaign-history/) | Cited official rules, human campaign testimony, original-AI behavior, parity observables, machine-readable sources, and a preserved original manual for F-007E. |
| [Cockpit PR #2 audit](evidence/2026-09-08-cockpit-pr-audit.md) | Resource adjudication, integrated diagnostics, Astra browser evidence, screenshots, and follow-up scope. |
| [Fleet miniature proof](evidence/2026-09-08-fleet-miniatures.md) | Exact GOKRES mappings, transparency checks, Astra browser evidence, interactions, and screenshots for F-010B. |
| [Runtime pack proof](evidence/2026-09-08-runtime-pack.md) | Deterministic package hashes, four-request startup, two-faction bitmap/interaction proof, and zoom/WebGL regression evidence for F-014A. |
| [Save-state fingerprint proof](evidence/2026-09-08-state-fingerprints.md) | Canonical two-run fingerprints, v9 integrity checks, native v8 compatibility, and Astra browser save/reload/load proof for F-011A. |
| [Save-continuation proof](evidence/2026-09-09-state-continuation.md) | Save v10 continuation envelope, historical v9 migration fixture, deterministic typed maps, and Astra 40/40 browser continuation proof for F-011B1. |
| [Replay-contract proof](evidence/2026-09-09-replay-contract.md) | F-011B2 replay format, canonical 51-DAT identity, command ordering, checkpoints, and native/WASM compile proof. |
| [Replay-execution proof](evidence/2026-09-09-replay-execution.md) | F-011B3 recorder/executor, fail-fast checkpoints, save-v11 continuation, stable iteration, and five-process native golden proof. |
| [Native/WASM replay proof](evidence/2026-09-09-replay-wasm-equivalence.md) | F-011B4 exact-artifact execution, nine native/WASM checkpoints, fail-closed browser diagnostics, and normal four-request startup. |
| [Fleet redispatch proof](evidence/2026-09-09-fleet-redispatch.md) | F-007A active-order invariants, AI exclusion, accepted-event telemetry, and the 5,000-tick seed-42 delta. |
| [Fleet position proof](evidence/2026-09-09-fleet-position-consolidation.md) | F-007B authoritative transit position, stale-index repair, orbit-only production, deterministic arrival consolidation, five-seed bounds, and Astra bitmap acceptance. |
| [Player fleet dispatch proof](evidence/2026-09-09-player-fleet-dispatch.md) | F-007C validated destination selection, authoritative departure, transit feedback, stale-menu cleanup, and Astra two-faction bitmap acceptance. |
| [System combat proof](evidence/2026-09-10-system-combat-resolution.md) | F-007D system-scoped multi-fleet resolution, persistent fighter attrition, shield correction, bounded stalemates, five-seed diagnostics, exact replay, and Astra bitmap acceptance. |
| [AI campaign-logistics checkpoint](evidence/2026-09-10-ai-campaign-logistics.md) | F-007E ownership, targeting, HQ defense, blockade, troop-class, repair/save-v12, and five-seed evidence; conquest and balance remain open. |
| [Troop transport and occupation proof](evidence/2026-09-10-troop-transport-occupation.md) | F-007E regiment cargo, invasion, political occupation, provisional character capture, source-review corrections, save-v13, five-seed evidence, and Astra browser acceptance. |
| [Player troop dispatch proof](evidence/2026-09-10-player-troop-dispatch.md) | F-007E bitmap troop selection, capacity enforcement, authoritative embarkation, landing, and Astra two-faction browser acceptance. |
| [Victory contract proof](evidence/2026-09-10-victory-contract.md) | F-007E asymmetric Coruscant/HQ objectives, Standard conjunctions, Death Star semantics, first-tick resolution, save-v13 continuity, and every current bombardment entry path. |
| [Save/Delete UI proof](evidence/2026-09-09-save-delete.md) | F-001 wiring, empty-load-slot regression fix, storage deletion, reload persistence, and Astra 33/33 bitmap/browser proof. |
| [Original main-menu cockpit proof](evidence/2026-09-09-main-menu-cockpit.md) | Binary-mapped controls, responsive bitmaps, direct faction starts, and Astra-verified browser playback. |
| [Campaign setup proof](evidence/2026-09-09-game-setup-propagation.md) | Save v11 setup persistence, Standard/Headquarters Only victory rules, and Astra 18/18 bitmap/browser proof. |
| [Main-menu completion proof](evidence/2026-09-09-main-menu-completion.md) | Credits, explicit multiplayer status, audio controls, campaign reset, corrected `MDATA.300` cue, original button effects, and Astra completion acceptance. |
| [Main-menu semantics proof](evidence/2026-09-09-main-menu-semantics.md) | Fourteen authentic semantic hotspots, selection state, cyclic keyboard focus, shared actions/SFX, destination lifecycle, and Astra browser acceptance. |
| [Main-menu music and final visual proof](evidence/2026-09-09-main-menu-music-toggle.md) | Original-button absence proof, documented holographic extension, native acceptance, and Astra 10/10 browser evidence. |
| [Authentic faction-advisor proof](evidence/2026-09-10-authentic-droid-advisors.md) | Type-302 extraction and decoding, native/WASM transport, both factions, corruption containment, and Astra browser evidence. |
| [Faithful-HD foundation proof](evidence/2026-09-10-faithful-hd-foundation.md) | Explicit render profiles, manifest provenance, verified-byte decoding, fail-closed fallback, and Astra browser-regression evidence for P45A. |
| [Tactical three-LOD family proof](../2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-3d-lod-family.md) | First complete original tactical mesh family, source-traced selection predicate, deterministic browser matrix, and remaining view-acceptance boundary for P24/P57. |
| [Tactical live LOD journey](../2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-live-lod-journey.md) | Same-renderer close-medium-far-medium-close switching, one-load proof, and remaining original-view boundary for P24/P57B2. |
| [Tactical camera and D-pad proof](../2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-camera-contract.md) | Source-traced faction views, field zoom, clip planes, handedness conversion, bitmap directional controls, and remaining original-view boundary for P24/P57B2. |
| [Tactical selected-object target proof](../2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-target-control.md) | Source-traced target command, normal/held bitmap state, selected-ship centering, and remaining production-ID/A0 boundary for P24/P57B2. |
| [Tactical active-force layout proof](../2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-battle-layout.md) | Source-traced active-object count, docked-fighter exclusion, battle extent, four tactical lanes, and remaining production-placement/A0 boundary for P24/P57B2. |
| [Tactical participant placement proof](../2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-participant-placement.md) | Stable DAT and fleet-roster identities, exact source X slots and faction lanes, selected-world-point targeting, and remaining resource-join/A0 boundary for P24/P57B2. |
| [Tactical authored transform and palette proof](../2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-transform-palette.md) | Direct authored mesh coordinates, complete system-palette transport, exact two-system selection, and remaining render-state/A0 boundary for P24/P57B2. |
| [Tactical retained-mode light proof](../2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-light-rig.md) | Directional and ambient light types, colors, frame and target contract, transformed direction, muted browser evidence, and remaining filtering/culling/A0 boundary for P24/P57B2. |
| [Tactical retained-mode render-state proof](../2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-render-state.md) | Dither, Gouraud, filtering, culling, depth, specular, diffuse/emissive material behavior, muted browser evidence, and remaining production/A0 boundary for P24/P57B2. |
| [Tactical resource-identity proof](../2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-resource-join.md) | Every ship/fighter DAT resource join, complete 87-mesh/397-texture browser transport, representative probes, and remaining production-draw/A0 boundary for P24/P58. |
| [Production tactical capital-ship proof](../2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-production-participants.md) | Exact joined capital families rendered by live battle participants, paired 3D-off controls, muted 36-case browser gate, and remaining fighter/selection/A0 boundary for P24/P58. |
| [Tactical fighter and projected-selection proof](../2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-fighters-selection.md) | Exact fighter detail resources, projected capital interaction, paired controls, muted 36-case browser gate, and remaining squadron/task-force/A0 boundary for P24/P58. |
| [Tactical fighter-detail journey proof](../2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-fighter-detail-journey.md) | Independent indicator/far/close transitions, visible centered close sprites, muted 40-case browser gate, and remaining squadron/task-force/A0 boundary for P24/P58. |
| [Tactical group-presentation proof](../2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-group-presentation.md) | Eight task-force controls, four RGBY fighter controls, source-shaped input, selected portraits, muted 44-case browser gate, and remaining automatic-grouping/A0 boundary for P24/P58. |
| [Tactical impact-effect proof](../2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-impact-effects.md) | Six source-selected hit, damage, and destruction families, exact 10 Hz lifecycle and priority, target attachment, muted 48-case browser gate, and remaining projectile/shield/A0 boundary for P24/P58. |
| [Tactical projectile and field proof](../2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-projectile-fields.md) | Retained projectile geometry, source thresholds, colors, interpolation, lifecycle, tractor/gravity field families and priority, muted 52-case browser gate, and remaining command/A0 boundary for P24/P58. |
| [Tactical selected-capital damage proof](../2026-09-10-interface-parity-audit/evidence/2026-09-15-tactical-selected-damage.md) | Panel 1302, source-ordinal portrait identity, live shield/hull meters, muted 56-case browser gate, and remaining subsystem/command/A0 boundary for P24/P58. |
| [Tactical subsystem and field-command proof](../2026-09-10-interface-parity-audit/evidence/2026-09-15-tactical-subsystem-field-commands.md) | Five source-quantized subsystem families, exact field-source identity and capacity, muted 60-case browser gate, and remaining combat-driven/interactive/A0 boundary for P24/P58. |
| [Tactical live subsystem-damage proof](../2026-09-10-interface-parity-audit/evidence/2026-09-15-tactical-live-subsystem-damage.md) | Shared source-derived capital/fighter damage, five subsystem thresholds and limits, tractor cancellation, muted 64-case browser gate, and remaining repair/movement/A0 boundary for P24/P58. |
| [Tactical subsystem-repair and mobility proof](../2026-09-10-interface-parity-audit/evidence/2026-09-15-tactical-subsystem-repair-mobility.md) | Source-derived repair cadence and selection, engine condition, tractor drag, muted 68-case browser gate, and remaining physical-movement/A0 boundary for P24/P58. |
| [Tactical maneuver and movement proof](../2026-09-10-interface-parity-audit/evidence/2026-09-16-tactical-maneuver-movement.md) | Source-derived maneuver bonus, effective-power velocity, 250-millisecond integration, muted 72-case browser gate, and remaining interactive-command/A0 boundary for P24/P58. |
| [Tactical command-assignment proof](../2026-09-10-interface-parity-audit/evidence/2026-09-17-tactical-command-assignment.md) | Original maneuver and mission panels, exact order and tactic codes, commit/cancel behavior, muted 76-case browser gate, and remaining execution/A0 boundary for P24/P58. |
| [Tactical order-execution proof](../2026-09-10-interface-parity-audit/evidence/2026-09-17-tactical-order-execution.md) | Recovered maneuver waypoints, Hold stop behavior, initial Recover carrier state, muted 80-case browser gate, and remaining completion/A0 boundary for P24/P58. |
| [Tactical command-progression proof](../2026-09-10-interface-parity-audit/evidence/2026-09-18-tactical-command-progression.md) | Source-rate turning, maneuver completion, fighter docking and recovery, muted 84-case browser gate, and remaining execution/A0 boundary for P24/P58. |
| [Tactical completion-bundle proof](../2026-09-10-interface-parity-audit/evidence/2026-09-22-tactical-completion-bundle.md) | Source-backed combat, grouping, formation, original result/options surfaces, Death Star and trench-run journeys, and the clean 120-case implementation gate for P24/P58. |
| [Tactical A0-ingestion proof](../2026-09-10-interface-parity-audit/evidence/2026-09-22-tactical-a0-ingestion.md) | Guest capture provenance, exact original executable and 640×480 validation, ignored evidence storage, and the current 0/106 offline-host boundary for P24/P58. |
| [Tactical A1-crosswalk proof](../2026-09-10-interface-parity-audit/evidence/2026-09-22-tactical-a1-crosswalk.md) | Existing deterministic evidence mapped to 82/106 cells, with 64 journeys, 18 snapshots, and a 24-cell implementation queue. |
| [Tactical navigation/camera proof](../2026-09-10-interface-parity-audit/evidence/2026-09-23-tactical-navigation-camera.md) | Four navigation sets, ordered routes and targets, camera memory/chase, Anvil and Stand Off, bringing deterministic A1 coverage to 96/106 while strict acceptance remains 0/106. |
| [Tactical trench-run/persistence proof](../2026-09-10-interface-parity-audit/evidence/2026-09-23-tactical-trench-persistence.md) | Production order-6 trench-run launch and exact strategic capital/fighter loss persistence, bringing deterministic A1 coverage to 98/106 while strict acceptance remains 0/106. |
| [Tactical Game Options/empty-space proof](../2026-09-10-interface-parity-audit/evidence/2026-09-23-tactical-game-options-empty-space.md) | Original Game Options routing and empty-space presentation bring deterministic A1 coverage to 100/106 while strict acceptance remains 0/106. |
| [Tactical withdrawal-confirmation proof](../2026-09-10-interface-parity-audit/evidence/2026-09-23-tactical-withdraw-confirmation.md) | Executable-derived panel, text, controls, cancel/confirm routing, and disabled repeat rejection bring deterministic A1 coverage to 101/106 while strict acceptance remains 0/106. |
| [Tactical detail and Escort proof](../2026-09-10-interface-parity-audit/evidence/2026-09-23-tactical-detail-escort.md) | Destroyed presentation, compact selected-capital contents, and direct source order-code 1 Escort bring deterministic A1 coverage to 104/106 while strict acceptance remains 0/106. |
| [Tactical Battle Alert and audio proof](../2026-09-10-interface-parity-audit/evidence/2026-09-24-tactical-battle-alert-audio.md) | Authentic faction alerts, Take Command entry, MDATA 307, and WAVE 13054 complete deterministic A1 mapping at 106/106 while strict acceptance remains 0/106. |
| [Tactical weapon-audio proof](../2026-09-10-interface-parity-audit/evidence/2026-09-24-tactical-weapon-audio.md) | Correct event semantics and the complete WAVE 13033–13054 fire/impact bank pass focused muted browser routing; exact RNG and strict A0 acceptance remain open. |
| [Tactical command-voice proof](../2026-09-10-interface-parity-audit/evidence/2026-09-24-tactical-command-voice.md) | Exact faction battle-ready and group command mappings route 90 VOICEFXA/VOICEFXE resources through native and browser backends; remaining voice families and strict A0 acceptance remain open. |
| [GitHub Pages proof](evidence/2026-09-08-github-pages.md) | Successful deployment run and public HTTP smoke tests for P40. |
| [Project archive](../../../archive/INDEX.md) | Superseded progress and playtest artifacts retained for provenance. |

## Current conclusion

The project is substantially implemented, but it is not yet demonstrably 100%
functional and does not yet have original in-campaign interface parity. The
[interface sister audit](../2026-09-10-interface-parity-audit/) records hard
failures for incomplete GID modes and overlays, cockpit destinations, partial
system-window contents, approximate rail thumbnails, and other replacement
surfaces. The original galaxy resources and default Popular Support view now
work alongside first-pass sector and system shells for both factions. The four
authentic advisor idle runs
now render in native and WASM;
their complete actions, voice, chrome, and shell integration remain open.
Fleet-miniature acceptance, deterministic four-request browser
startup, F-001 browser Save/Load/Delete, the F-011A fingerprint primitive,
F-011B1 save continuation, the F-011B2 replay/data contract, F-011B3 native
replay execution, F-011B4 native/WASM fixture equivalence, F-007A fleet-order
protection, F-007B fleet-position consolidation, F-007C player fleet dispatch,
F-007D system-combat backlog closure, the F-007E logistics/data/repair,
transport/occupation/player-dispatch, and victory-contract checkpoints, and F-016A/B/C/D/E
cockpit/setup functionality now pass; P03 and P04 are complete. The source
review still leaves capture/evasion open. Faction liveness, the wider
campaign loop, five-seed cross-runtime replay,
interactive/tactical convergence, browser
memory/media work, cross-browser performance, formatting, lint, and
release-level visual acceptance remain incomplete.

The JSON document is the canonical source for stable finding and feature IDs.
The Markdown document explains the evidence and how to execute each pass. Its
recommendations incorporate independent model reviews. F-007E campaign tuning
additionally follows the cited
[Rebellion/Supremacy campaign-history baseline](../../reference/campaign-history/),
which marks unsourced numeric gates as engineering hypotheses.

P40 was revalidated against the current campaign-history revision in deployment
run `34505121163`; the new reference pages, source ledger, and preserved manual
all return HTTP 200.
