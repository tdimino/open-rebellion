---
title: "Standalone Space-Battle Test Launcher"
description: "Test-only entry into the production tactical battle path and its original-interface parity matrix"
category: plan
created: 2026-09-12
updated: 2026-09-22
tags: [interface, parity, tactical, browser, testing]
status: in_progress
---

# Standalone Space-Battle Test Launcher

This is a test harness for `UIP-B06` in the [batched interface plan](2026-09-11-feat-batched-interface-parity-plan.md). A developer should be able to choose a deterministic battle, open the actual game directly in that battle, use its real controls, and inspect the result without playing a campaign first. The launcher is outside the game canvas and absent from production builds. It must not introduce another tactical renderer or an invented in-game menu.

The [Tactical 106-Cell Acceptance Train](2026-09-22-tooling-tactical-106-cell-acceptance.md)
is the final execution plan for authoritative original captures, exact catalog
coverage, A0/A1 comparison, remaining production closure, and the completion
push.

Use the [space-battle launcher reference](../reference/space-battle-launcher/README.md)
for current routes, fixture codes, commands, evidence, and verified limits.
Its [reverse-engineering map](../reference/space-battle-launcher/reverse-engineering-map.md)
ranks the exact Windows/Ghidra targets that unlock each remaining tactical seam.

The [tactical 3D asset pipeline](2026-09-12-feat-tactical-3d-asset-pipeline.md)
owns recovery and browser rendering of the original DirectX meshes and
textures. This launcher supplies its deterministic production-scene fixtures
and evidence gate.

It is not blocked on finishing the GID, system-window, or other strategic interface families. Start T0 and T1 after the current in-flight GID bundle is committed, then work on tactical asset proof and original HUD composition as a separate `UIP-B06` lane. The final 106-cell tactical acceptance gate still depends on original resources, screenshot baselines, and real browser interaction; a functioning launcher is not that gate.

Every T1 through T5 browser harness gate uses `codex-orchestrator` for
independent live browser/computer-use acceptance. Use Sol high or extra-high
for code or evidence review only when the slice's risk warrants an independent
pass. Each scenario starts in a fresh muted Chromium
process and closes its browser and local server when complete.

T0 and T1 are partially complete. The [tactical result-identity regression](../qa/2026-09-08-full-functionality-audit/evidence/2026-09-12-tactical-result-identity.md)
preserves surviving hull damage and exact fighter roster losses. Campaign and
test-only browser entry share the validated production tactical-entry function.
P52 through the current P58F17C checkpoint, following
[P58F13](../qa/2026-09-10-interface-parity-audit/evidence/2026-09-19-tactical-attack-target-lifecycle.md),
restore the first authentic shell controls, complete tactical asset decode, one
three-LOD family, source camera and layout, stable participants, authored
coordinates, system palette, lighting, retained-mode render state, and exact
DAT resource joins. Live capital ships render from their joined families;
fighter groups use the original detail resources; and capital interactions use
projected mesh bounds. The original task-force and RGBY fighter-control strip,
group selection, keyboard routes, and selected fighter portraits also work.
The full muted harness passes 92 of 92 cases, and the four-request pack carries
all 87 meshes and 397 textures. Production battles now load the exact
system-selected 256 by 256 planet and paired palette. Six target-impact
families now preserve exact frames, sizes, priority, transparency, attachment,
and 10 Hz timing. Retained projectiles now preserve exact geometry variants,
thresholds, scales, material selectors, interpolation, and duration. Tractor
and gravity fields use their original 128-by-128 frame families, exact source
IDs, source-side capacities, 10 Hz timing, and shared gravity priority.
Selected-capital panel 1302 now uses source-ordinal portraits, live
faction-correct shield and hull meters, and all five source-quantized subsystem
condition families.
Capital and fighter fire now share the recovered shield-overflow, hull-loss,
subsystem-hit, condition, and tractor-cancellation path.
The source repair cadence and selection plus engine-condition and active
tractor-drag mobility now complete the live subsystem loop.
The recovered maneuver-state producer, effective-power velocity, and
250-millisecond position integration now drive production source positions.
The original Maneuvers and Tactics and Missions panels now commit exact shared
order and tactic codes to selected capital ships and fighter groups, with
source-shaped controls, original disabled states, and cancel semantics.
Committed Left Hook, Right Hook, Hammer, and Anvil orders now construct their
recovered 0.75-scale, pi-over-eight waypoints. Hold clears movement intent, and
Recover reserves a compatible carrier and enters Returning state 2. Capital
ships then use the source-rate signed turn and clear reached waypoints. Fighters
advance through Docking and Recovered while retaining their strategic counts.
Attack Fighters and Attack Capital Ships now acquire the first eligible typed
target in stable tactical order for both capital and fighter owners, while
preserving a valid live engagement. Invalid targets now reacquire the first
eligible hostile target of the same requested class, and exhausted target
lists clear without random or cross-class fallback. The completion bundle
now adds source-backed capital and fighter fire, shield recharge, collision,
automatic colored fighter groups and bounded task forces, the separate Death
Star object, the authentic Battle Results and Battle Options surfaces, the
authored type-303 resource 5030 star field, and the 1021-through-1024
superlaser control journey through target destruction.
Exact global RNG sequencing, exact original arrival callbacks and recovery
trajectories, power allocation, exact planet placement, the native Death Star
beam contract, native trench-run playback, shared post-battle bombardment,
landing, and navigation orchestration, original view acceptance, and all 106
visual cells remain open. The deterministic A1 matrix now covers 100 of 106
canonical tactical cells through 81 browser journeys and 19 snapshots. The
complete 120-case implementation matrix passes both factions and both
viewports with clean muted-browser isolation. This implementation coverage
does not replace the strict 0 of 106 tactical-cell acceptance result.

## Current contract and limits

- Campaign combat and the direct fixture enter `GameMode::TacticalCombat` through the validated `begin_player_battle` function in `crates/rebellion-app/src/tactical_flow.rs`. P58-B06 also moved hull, fighter-roster, Death Star, empty-fleet, and cargo-loss transport into the shared `apply_results` function. Bombardment, landing, result presentation, and final navigation orchestration remain inline and must be shared before the direct-launch fixture can claim complete campaign-path equivalence.
- `crates/rebellion-render/src/tactical_view.rs` already models placement, combat, results, ships, fighters, selection, retreat, pause, speed, and auto-resolution. The selected-capital path now uses original panel 1302, but power allocation and results composition remain incomplete or replacement UI.
- The interactive `BattleSession` simulation and `CombatSystem::resolve_space` auto-resolution are different paths. Interactive result application now preserves surviving hull damage and exact fighter roster identity, but the paths still need a shared production entry/return contract and broader outcome coverage.
- The separate `interface-test-fixtures` WASM build already provides a deterministic, audio-muted GID fixture bridge and browser harness. Its production-exclusion check must remain a release gate.
- The [surface ledger](../qa/2026-09-10-interface-parity-audit/surface-ledger.json) has 106 pending space-battle cells in `TAC-01` through `TAC-07`. `EVT-02` covers Battle Alert. `TAC-08` is the separate ground-assault report flow, not a live space-battle scene.
- The [native tactical lookup](../reference/asset-library/tactical-lookup.json) proves all 29 ship, eight fighter, and Death Star DAT-to-ordinal resource joins. P54 through the current completion bundle prove staging, decoding, camera, placement, render state, exact identity joins, live capital and fighter rendering, interactions, commands, weapon behavior, collision, automatic group assignment, capability-sorted task forces, forward-relative retained follower geometry, the separate Death Star object, original result/options panels, resource 5030 stars, the superlaser control journey, exact 201/202 trench-run routing, and exact strategic roster, capture, and Death Star-state application. The [P58-B06 evidence](../qa/2026-09-10-interface-parity-audit/evidence/2026-09-22-tactical-completion-bundle.md) records the clean 120-case gate. Exact planet placement, exact global RNG sequencing, exact original arrival callbacks and recovery trajectories, power allocation, the native Death Star beam contract, shared post-battle bombardment, landing, and navigation orchestration, native playback, and original view acceptance remain open. The source tactical 3D window uses the standard arrow cursor.
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

Battle entry and core tactical-result transport now live in one production module. Continue moving bombardment, landing, result presentation, and navigation out of the inline app loop. Reconcile interactive and auto-resolve results with the canonical game-data integrator, preserving exact ship and fighter identities, the separate Death Star, and surviving damage. Add native tests for both factions, each outcome, retreat, partial losses, and strategic return.

Gate: campaign battle behavior is unchanged except for demonstrated result-correctness fixes; tests prove the same entry and result code is used by campaign and fixture transports. This is a functional prerequisite, not a visual-parity claim.

### T1. Test-only entry and safety

Add a tactical fixture namespace and scenario catalog beside `tools/interface-parity/scenarios/gid.catalog.json`. Extend `crates/rebellion-app/src/interface_test_fixture.rs` or add a gated sibling for tactical setup. Build a minimal two-faction fleet encounter, call the shared production battle entry, and emit a `battle-ready` record with scenario ID, faction, system, fleet IDs, and deterministic seed. Add a native CLI flag guarded by both `interface-test-fixtures` and an explicit opt-in environment variable. Update `tools/interface-parity/verify-production-exclusion.mjs` to reject every new marker in production HTML and WASM.

Status: browser-only entry fixture and production-exclusion checks are implemented. Both factions and both viewports passed the four-request, muted startup gate. The ready record does not yet contain fleet IDs or a seed, and no native CLI flag exists. Those contract details and battle interactions remain T1/T2 work. The current synthetic battle HUD is not accepted as original bitmap parity.

Gate: both factions open the actual tactical view from the test-only page; production exclusion, fixture validation, muted audio, and four-request startup pass. Do not mark `TAC-01` complete.

### T2. Permanent battle journey harness

Extend the existing Playwright harness with a tactical catalog rather than a second browser runner. Capture native 640×480 and responsive letterboxed viewports for both factions. Exercise placement, select/order, pause/resume, retreat, auto-resolve, results, and return. Emit screenshots, interaction traces, console/network logs, bitmap-resource diagnostics, and before/after campaign state. Distinguish frozen visual checks from real action checks.

Gate: repeatable seeded results, no missing required assets or browser errors, stable viewport hit targets, and all browser processes closed. A test build may pass this gate while original-interface parity remains open.

### T3. Original tactical composition

Use `RE-TAC-01`, `RE-TAC-02`, `RE-BAT-01`, the manual pages 139–150, the [asset reference library](../reference/asset-library/README.md), extracted `TACTICAL.DLL` resources, and the screenshot ledger to replace the current 1200×800 synthetic egui tactical HUD and results screen panel by panel. Map original resource ID, placement, state predicate, hit rectangle, and action for each control before implementation. Production 3D camera binding and several tactical controls remain incomplete, so a rendered panel alone is insufficient. Keep the battle launcher fixed on the production renderer and expand its snapshot matrix as surfaces become authentic.

Gate: source-mapped art and controls for the selected bundle, exact extracted-BMP pixels where unobscured, documented comparison limits for compressed screenshots, and task-proportional browser acceptance of both factions. Do not infer missing hover, disabled, Death Star, or result art from a modern replacement.

Status: the P52, P53, P57B2A, P57B2B1, P57B2B2, and P57B2B3 slices pass this scoped gate for shell `1000`, faction
group headers, the selected-unit frame, pause, highlights, zoom, and visible
camera-control art. Working controls use source-shaped hit masks, and zoom plus
the four D-pad controls use their normal and held resources. The source faction
view, field zoom, clip planes, yaw, pitch, and handedness conversion now run in
the isolated camera fixture. The center target button now uses its source
normal/held resources and focuses the selected fixture object. Active force
counts now derive the source battle extent and four lanes. The active combat
replacement panels are removed. Production participants retain stable DAT and
fleet-roster identity, use the recovered X slots and faction lanes, and route
the target camera to the selected source-world point. P58A completes the exact
DAT-to-resource join. P58B draws live production capital ships from their
joined families. P58C renders original fighter detail resources and aligns
capital selection and targeting with projected mesh bounds. P58E restores the
task-force and squadron controls. P58F1 binds the exact system-selected planet
and paired palette. P58F2 restores all six target-impact sprite families and
their lifecycle. P58F3 restores retained projectile and tractor/gravity field
presentation. P58F4 restores the selected-capital panel, portrait identity,
and live shield and hull meters. P58F5 restores the five subsystem-condition
families and exact field-source identity and capacity. P58F6 through P58F8
restore live damage, repair, mobility, maneuver-state production, velocity,
and physical integration. P58F9 restores the two bitmap command panels, exact
order and tactic codes, source-shaped input, disabled states, and commit/cancel
behavior. P58F10 connects the four maneuver orders to recovered waypoint
construction, executes Hold, and starts Recover through carrier reservation and
Returning state 2. P58F11 adds source-rate turning, waypoint completion, and
the Docking and Recovered fighter states while preserving strategic squadron
counts. P58F12 adds typed Attack Fighters and Attack Capital Ships target
acquisition for capital and fighter owners. P58F13 adds stable same-class
replacement and exhausted-list clearing. P58F14 through P58F17C
bundle adds capital and fighter combat, collision, group assignment,
capability-sorted task forces and forward-relative retained follower geometry,
Death Star transport,
original result/options panels, resource 5030 stars, the superlaser control
journey, exact 201/202 trench-run routing, and strategic result application.
Power allocation, exact planet placement, native Death Star beam behavior,
shared post-battle bombardment, landing, and navigation orchestration, audio,
native playback, and A0 view acceptance remain T3 and T4 work.
No `TAC-*` acceptance cell is closed.

### Consolidated completion boundary

The next implementation commit carries the remaining T3, T4, and production
entry/return behavior as one coherent space-battle bundle. Focused tests may
run during construction, but the complete workspace, packaged WASM, 106-cell
muted browser harness, and independent review run once at the bundle gate. This
makes space battles available through the campaign and direct test launcher
without introducing a second renderer or an invented in-game menu.

A later optional skirmish setup menu is a separate feature. It will use an
X-Wing Alliance-style flow to configure ships, planet, and space background,
then enter this same production battle scene. It remains outside original
interface parity and awaits a dedicated Fable and `/ce:plan` design pass.

### T4. Battle outcome and rare-state closure

Exercise survival, damage, destruction, retreat, fighter losses, both winners, and draw through the same result path as a campaign battle. The Death Star gauge, fire control, target assignment, beam, destruction journey, and both source result-to-film branches now exist provisionally. Reconcile tactical results, bombardment/landing follow-up, reports, messages, saves, native playback, and the exact beam contract with the campaign state.

Gate: native unit/integration tests and browser journeys agree on state fingerprints and result routing. No fixture-only state change is credited as a working command.

### T5. Strict acceptance

For each `TAC-01` through `TAC-07` cell, run its source evidence, native, packaged-WASM, both-faction, viewport, interaction, audio, and diagnostic matrix from the [interface audit](../qa/2026-09-10-interface-parity-audit/README.md). An independent reviewer operates selected live browser journeys after the harness performs deterministic probes. Use Sol high or extra-high review only for risk-significant bundles. Update the ledger and evidence in the same commit as each verified bundle, then push. Keep any cell lacking authoritative original captures pending.

## Evidence and release rules

The launcher measures functional correctness and visible progress, not automatic 100% parity. A direct original 640×480 lossless capture is the preferred A0 baseline; the available JPEG, WebP, manual scans, and video frames are discovery or provisional comparison evidence. Tests must report which type of evidence supports each assertion. No copyrighted game binaries or extracted BMPs enter the repository. The launcher and its fixtures are excluded from production artifacts and from the proposed password-protected Cloudflare 1.0 build.
