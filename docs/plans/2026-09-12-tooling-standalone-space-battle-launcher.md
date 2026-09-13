---
title: "Standalone Space-Battle Test Launcher"
description: "Test-only entry into the production tactical battle path and its original-interface parity matrix"
category: plan
created: 2026-09-12
updated: 2026-09-13
tags: [interface, parity, tactical, browser, testing]
status: in_progress
---

# Standalone Space-Battle Test Launcher

This is a test harness for `UIP-B06` in the [batched interface plan](2026-09-11-feat-batched-interface-parity-plan.md). A developer should be able to choose a deterministic battle, open the actual game directly in that battle, use its real controls, and inspect the result without playing a campaign first. The launcher is outside the game canvas and absent from production builds. It must not introduce another tactical renderer or an invented in-game menu.

The [tactical 3D asset pipeline](2026-09-12-feat-tactical-3d-asset-pipeline.md)
owns recovery and browser rendering of the original DirectX meshes and
textures. This launcher supplies its deterministic production-scene fixtures
and evidence gate.

It is not blocked on finishing the GID, system-window, or other strategic interface families. Start T0 and T1 after the current in-flight GID bundle is committed, then work on tactical asset proof and original HUD composition as a separate `UIP-B06` lane. The final 106-cell tactical acceptance gate still depends on original resources, screenshot baselines, and real browser interaction; a functioning launcher is not that gate.

Every T1 through T5 browser harness gate uses `codex-orchestrator` with Astra
at medium effort. Each scenario starts in a fresh muted Chromium process and
closes its browser and local server when complete.

T0 and T1 are partially complete. The [tactical result-identity regression](../qa/2026-09-08-full-functionality-audit/evidence/2026-09-12-tactical-result-identity.md) preserves surviving hull damage and exact fighter roster losses. Campaign and a test-only browser fixture both call the validated production tactical-entry function. The fixture opens both factions muted at native and letterboxed sizes, with four successful startup requests and no browser errors in each case. [P52](../qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-tactical-shell-controls.md) adds the original 640×480 shell and first working bitmap pause, highlight, and zoom controls. [P53](../qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-tactical-control-hit-states.md) adds palette-key hit masks and held zoom pressed art. [P54](../qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-tactical-3d-staging.md) adds reproducible raw staging for every original type-301 and type-303 tactical resource. [P55](../qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-tactical-3d-runtime-pack.md) converts and independently verifies the full original corpus. [P56](../qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-3d-render-proof.md) packages and visibly renders one exact mesh and texture pair in an isolated proof/control matrix. Shared return/outcome routing, production fleet rendering, complete battle interactions, original comparisons, and all 106 visual cells remain open.

## Current contract and limits

- Campaign combat enters `GameMode::TacticalCombat` through the validated `begin_player_battle` function in `crates/rebellion-app/src/tactical_flow.rs`. The same main loop calls `draw_tactical_view` and applies battle results back to `GameWorld`; return/outcome orchestration is still inline and must be shared before a direct-launch fixture can claim complete campaign-path equivalence.
- `crates/rebellion-render/src/tactical_view.rs` already models placement, combat, results, ships, fighters, selection, retreat, pause, speed, and auto-resolution. Its current HUD and results composition are replacement egui UI, not the original bitmap interface.
- The interactive `BattleSession` simulation and `CombatSystem::resolve_space` auto-resolution are different paths. Interactive result application now preserves surviving hull damage and exact fighter roster identity, but the paths still need a shared production entry/return contract and broader outcome coverage.
- The separate `interface-test-fixtures` WASM build already provides a deterministic, audio-muted GID fixture bridge and browser harness. Its production-exclusion check must remain a release gate.
- The [surface ledger](../qa/2026-09-10-interface-parity-audit/surface-ledger.json) has 106 pending space-battle cells in `TAC-01` through `TAC-07`. `EVT-02` covers Battle Alert. `TAC-08` is the separate ground-assault report flow, not a live space-battle scene.
- The [native tactical lookup](../reference/asset-library/tactical-lookup.json) proves 29 ship and eight fighter ordinal-to-resource bases. Candidate DAT names still need the original vtable identity join. P54 stages and P55 decodes all type-301 and type-303 resources. P56 packages and renders one source-bound pair only; complete browser transport and production selection remain open.
- The [screenshot ledger](../qa/2026-09-10-interface-parity-audit/screenshot-ledger.md) has useful tactical HUD, selection, damage, and results references. Most are compressed, localized, or from an altered campaign. They support reconstruction and provisional comparison, not strict pixel acceptance. Lossless original-executable captures remain open.

## Design

1. Extract one production tactical entry and result-application flow from `main.rs`. Both campaign entry and fixture entry must call it, and both interactive and auto-resolved outcomes must preserve surviving damage, exact unit identity, losses, ownership, and report routing. No fixture may independently set `GameMode` or write strategic results.
2. Extend the existing test-only fixture protocol with a versioned `tactical` family. Keep existing GID fixture codes stable. Validate scenario and faction before mutating any state. The normal packaged HTML and WASM must contain no tactical fixture route or selector.
3. Build each fixture from a deterministic `GameWorld` and real fleets at a known system. Enter combat through the shared production flow.
4. Offer a small standalone page generated only into the test site. Its scenario picker links to the test build and may show evidence metadata outside the 640×480 game canvas. Add a feature-gated native CLI entry to the same fixture catalog. Neither is a second game executable or an in-game control.
5. Provide two fixture modes. **Journey fixtures** begin at a legal battle entry and exercise real clicks and state transitions. **Snapshot fixtures** freeze a named state after valid setup for stable bitmap comparison. Snapshot injection may position or damage units, but it cannot be used to claim the underlying command works.
6. Use fixed seeds and controlled clock steps in the test artifact. Keep music and sound muted on every launch. Never alter the user's production saves or campaign state. Keep one fresh browser process and temporary profile per scenario.
7. Return through the production `TacticalAction` and result-application path. Record before/after world fingerprints, fleet and ship counts, damage, winner, message/report routing, and the strategic destination. A screenshot alone does not prove battle functionality.

## Scenario catalog

| Group | First deterministic journeys and snapshots | Audit target |
|---|---|---|
| Entry and HUD | Battle Alert to command/simulate/observe when implemented; Alliance and Empire placement, paused combat, running combat, empty space and planet backdrop | `EVT-02`, `TAC-01` |
| Forces and selection | Capital ships, fighters, task-force and squadron assignment, friendly/enemy selection, disabled orders, contents, targets, healthy/damaged/critical/destroyed states | `TAC-02`, `TAC-03` |
| Navigation and orders | Camera pan/tilt/rotate/zoom, nav sets, routes, target loss, maneuvers, tactics, missions, recovery, retreat | `TAC-04`, `TAC-05` |
| Special battle | Death Star selection, laser charge/fire, attack enabled/disabled, trench-run launch and both outcomes | `TAC-03`, `TAC-05`, `RE-DS-02` |
| Completion | Options, withdraw confirmation, simulate remainder, observe/take command, both faction victories, damaged and destroyed force tabs, return to strategy with persisted losses | `TAC-06`, `TAC-07` |

Catalog rows must reference exact ledger cell IDs and an original source for every visual assertion. Missing states remain explicitly `reference-needed`, not silently accepted. The first catalog need not implement all 106 cells, but it must make the remaining denominator and missing sources visible.

## Implementation passes

### T0. Shared production tactical flow and outcomes

Move battle entry and completion orchestration out of the inline app loop into one production module. Reconcile interactive and auto-resolve results with the canonical game-data integrator, preserving exact ship and fighter identities and surviving damage. Add native tests for both factions, each outcome, retreat, partial losses, and strategic return.

Gate: campaign battle behavior is unchanged except for demonstrated result-correctness fixes; tests prove the same entry and result code is used by campaign and fixture transports. This is a functional prerequisite, not a visual-parity claim.

### T1. Test-only entry and safety

Add a tactical fixture namespace and scenario catalog beside `tools/interface-parity/scenarios/gid.catalog.json`. Extend `crates/rebellion-app/src/interface_test_fixture.rs` or add a gated sibling for tactical setup. Build a minimal two-faction fleet encounter, call the shared production battle entry, and emit a `battle-ready` record with scenario ID, faction, system, fleet IDs, and deterministic seed. Add a native CLI flag guarded by both `interface-test-fixtures` and an explicit opt-in environment variable. Update `tools/interface-parity/verify-production-exclusion.mjs` to reject every new marker in production HTML and WASM.

Status: browser-only entry fixture and production-exclusion checks are implemented. Both factions and both viewports passed the four-request, muted startup gate. The ready record does not yet contain fleet IDs or a seed, and no native CLI flag exists. Those contract details and battle interactions remain T1/T2 work. The current synthetic battle HUD is not accepted as original bitmap parity.

Gate: both factions open the actual tactical view from the test-only page; production exclusion, fixture validation, muted audio, and four-request startup pass. Do not mark `TAC-01` complete.

### T2. Permanent battle journey harness

Extend the existing Playwright harness with a tactical catalog rather than a second browser runner. Capture native 640×480 and responsive letterboxed viewports for both factions. Exercise placement, select/order, pause/resume, retreat, auto-resolve, results, and return. Emit screenshots, interaction traces, console/network logs, bitmap-resource diagnostics, and before/after campaign state. Distinguish frozen visual checks from real action checks.

Gate: repeatable seeded results, no missing required assets or browser errors, stable viewport hit targets, and all browser processes closed. A test build may pass this gate while original-interface parity remains open.

### T3. Original tactical composition

Use `RE-TAC-01`, `RE-TAC-02`, `RE-BAT-01`, the manual pages 139–150, the [asset reference library](../reference/asset-library/README.md), extracted `TACTICAL.DLL` resources, and the screenshot ledger to replace the current 1200×800 synthetic egui tactical HUD and results screen panel by panel. Map original resource ID, placement, state predicate, hit rectangle, and action for each control before implementation. Current camera fields and several tactical controls are not functional, so a rendered panel alone is insufficient. Keep the battle launcher fixed on the production renderer and expand its snapshot matrix as surfaces become authentic.

Gate: source-mapped art and controls for the selected bundle, exact extracted-BMP pixels where unobscured, documented comparison limits for compressed screenshots, and an Astra medium review of both factions. Do not infer missing hover, disabled, Death Star, or result art from a modern replacement.

Status: the P52 and P53 slices pass this scoped gate for shell `1000`, faction
group headers, the selected-unit frame, pause, highlights, zoom, and visible
camera-control art. Working controls use source-shaped hit masks, and zoom uses
its normal and held pressed resources. The active combat replacement panels are removed. Camera
semantics beyond zoom, task-force and squadron assignment, selected-unit data,
ships, fighters, planets, effects, commands, results, and audio remain T3 work.
No `TAC-*` acceptance cell is closed.

### T4. Battle outcome and rare-state closure

Exercise survival, damage, destruction, retreat, fighter losses, both winners, and draw through the same result path as a campaign battle. Current tactical Death Star and trench-run controls are absent, so add those scenarios only after their native semantics and assets are traced and implemented. Reconcile tactical results, bombardment/landing follow-up, reports, messages, and saves with the campaign state.

Gate: native unit/integration tests and browser journeys agree on state fingerprints and result routing. No fixture-only state change is credited as a working command.

### T5. Strict acceptance

For each `TAC-01` through `TAC-07` cell, run its source evidence, native, packaged-WASM, both-faction, viewport, interaction, audio, and diagnostic matrix from the [interface audit](../qa/2026-09-10-interface-parity-audit/README.md). Astra medium reviews a bounded battle bundle rather than operating every probe. Update the ledger and evidence in the same commit as each verified bundle, then push. Keep any cell lacking authoritative original captures pending.

## Evidence and release rules

The launcher measures functional correctness and visible progress, not automatic 100% parity. A direct original 640×480 lossless capture is the preferred A0 baseline; the available JPEG, WebP, manual scans, and video frames are discovery or provisional comparison evidence. Tests must report which type of evidence supports each assertion. No copyrighted game binaries or extracted BMPs enter the repository. The launcher and its fixtures are excluded from production artifacts and from the proposed password-protected Cloudflare 1.0 build.
