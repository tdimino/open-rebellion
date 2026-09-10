---
title: "Full Functionality Audit Index"
description: "Entry point for the September 2026 Open Rebellion functionality, parity, and bitmap audit"
category: qa
created: 2026-09-08
updated: 2026-09-10
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
| [Save/Delete UI proof](evidence/2026-09-09-save-delete.md) | F-001 wiring, empty-load-slot regression fix, storage deletion, reload persistence, and Astra 33/33 bitmap/browser proof. |
| [Original main-menu cockpit proof](evidence/2026-09-09-main-menu-cockpit.md) | Binary-mapped controls, responsive bitmaps, direct faction starts, and Astra-verified browser playback. |
| [Campaign setup proof](evidence/2026-09-09-game-setup-propagation.md) | Save v11 setup persistence, Standard/Headquarters Only victory rules, and Astra 18/18 bitmap/browser proof. |
| [Main-menu completion proof](evidence/2026-09-09-main-menu-completion.md) | Credits, explicit multiplayer status, audio controls, campaign reset, corrected `MDATA.300` cue, original button effects, and Astra completion acceptance. |
| [Main-menu semantics proof](evidence/2026-09-09-main-menu-semantics.md) | Fourteen authentic semantic hotspots, selection state, cyclic keyboard focus, shared actions/SFX, destination lifecycle, and Astra browser acceptance. |
| [Main-menu music and final visual proof](evidence/2026-09-09-main-menu-music-toggle.md) | Original-button absence proof, documented holographic extension, native acceptance, and Astra 10/10 browser evidence. |
| [GitHub Pages proof](evidence/2026-09-08-github-pages.md) | Successful deployment run and public HTTP smoke tests for P40. |
| [Project archive](../../../archive/INDEX.md) | Superseded progress and playtest artifacts retained for provenance. |

## Current conclusion

The project is substantially implemented, but it is not yet demonstrably 100%
functional. Fleet-miniature acceptance, deterministic four-request browser
startup, F-001 browser Save/Load/Delete, the F-011A fingerprint primitive,
F-011B1 save continuation, the F-011B2 replay/data contract, F-011B3 native
replay execution, F-011B4 native/WASM fixture equivalence, F-007A fleet-order
protection, F-007B fleet-position consolidation, F-007C player fleet dispatch,
F-007D system-combat backlog closure, the F-007E logistics/data/repair checkpoint,
and F-016A/B/C/D/E cockpit/setup functionality now pass; P03 and P04 are
complete. Troop transport and occupation, faction balance, victory, five-seed
cross-runtime replay, interactive/tactical convergence, browser
memory/media work, cross-browser performance, formatting, lint, and
release-level visual acceptance remain incomplete.

The JSON document is the canonical source for stable finding and feature IDs.
The Markdown document explains the evidence and how to execute each pass. Its
recommendations incorporate independent GPT-6-Astra and verified Claude Fable
5.1 reviews.
