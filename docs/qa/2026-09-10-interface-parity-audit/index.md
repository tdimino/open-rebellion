---
title: "Original Interface Parity Audit"
description: "Authoritative screenshot corpus, surface ledger, findings, and acceptance gates for reproducing every visible Rebellion interface state"
category: qa
created: 2026-09-10
updated: 2026-09-18
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
| Practical interface scope materially tackled | approximately 20 to 25% |
| Practical interface scope remaining | approximately 75 to 80% |
| Practical space-battle launcher implementation | approximately 96% |
| Strict tactical acceptance | 0 of 106 cells |
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
| [P46A strategic shell evidence](evidence/2026-09-11-strategic-shell-canvas.md) | Durable canvas, aperture, shared-transform, test, artifact-hash, and browser acceptance record. |
| [P46B strategic-control evidence](evidence/2026-09-11-strategic-command-controls.md) | Recovered six-control geometry, bitmap pairs, input masks, command mapping, tests, and browser acceptance. |
| [PR #11 cockpit-routing correction](evidence/2026-09-14-cockpit-routing-correction.md) | Corrected `0x131` through `0x133` identities, F1/F7 routing, GID open/close behavior, and fail-closed original destinations. |
| [UIP-B01 strategic-navigation evidence](evidence/2026-09-11-strategic-window-navigation.md) | First-pass sector and system shells, tab resources, modeless lifecycle, rail routing, and browser evidence. |
| [P46D detailed-system evidence](evidence/2026-09-11-detailed-system-tab-items.md) | Source-mapped GOKRES tab items, displayed labels, bounded scrollbar navigation, intelligence gating, selection state, tests, and browser acceptance. |
| [P47A galaxy-backdrop evidence](evidence/2026-09-11-authored-galaxy-backdrop.md) | STRATEGY 902 identity, source-aligned placement, native/WASM texture transport, tests, captures, and open GID boundaries. |
| [P47B Popular Support evidence](evidence/2026-09-11-popular-support-gid.md) | Active STRATEGY 903 predicate, faction GID control, compact legend, native marker families, support thresholds, tests, and browser acceptance. |
| [P48 GID browser-harness evidence](evidence/2026-09-12-gid-browser-harness.md) | Code-built GID command tree, test-only fixture isolation, muted browser matrix, and unaccepted visual states. |
| [P49 Message Index rail evidence](evidence/2026-09-12-message-index-rail.md) | Original faction rail positions and BMPs, exact resting-pixel checks, and open category/index behavior. |
| [P50 GID menu-frame evidence](evidence/2026-09-12-gid-menu-frame.md) | Eight source frame tiles, instant menu display, source-pixel border checks, and remaining menu gaps. |
| [P51 GID hover and occlusion evidence](evidence/2026-09-12-gid-hover-and-occlusion.md) | Unsupported hover-wash removal and source-pixel checks on visible frame edges beneath a system window. |
| [P52 tactical shell and controls evidence](evidence/2026-09-12-tactical-shell-controls.md) | Original 640×480 shell, first working bitmap controls, exact source-pixel probes, and explicit tactical gaps. |
| [P53 tactical control-state evidence](evidence/2026-09-12-tactical-control-hit-states.md) | Original palette-key hit masks, zoom pressed/resting resources, round-trip browser proof, and explicit limits. |
| [P54 tactical 3D staging evidence](evidence/2026-09-12-tactical-3d-staging.md) | Complete raw type-301/type-303 inventory, content-addressed extraction, source hashes, and explicit non-rendering limits. |
| [P55 tactical 3D runtime-pack evidence](evidence/2026-09-12-tactical-3d-runtime-pack.md) | Complete binary-X and type-303 decoding, deterministic mesh/texture objects, palette routing, source-bound exceptions, and Assimp oracle results. |
| [P56 tactical 3D render proof](evidence/2026-09-13-tactical-3d-render-proof.md) | Exact typed mesh/texture transport, normals-aware browser rendering, proof/control pixel evidence, runtime observations, and explicit open parity gates. |
| [P57A tactical 3D LOD-family evidence](evidence/2026-09-13-tactical-3d-lod-family.md) | Complete first three-mesh family, source-traced LOD predicate, deterministic two-faction browser matrix, and explicit P57B limits. |
| [P57B1 tactical live LOD evidence](evidence/2026-09-13-tactical-live-lod-journey.md) | Source-traced cached-slot cycle, live close-medium-far-medium-close browser journey, one-load proof, and explicit P57B2 limits. |
| [P57B2A tactical camera evidence](evidence/2026-09-13-tactical-camera-contract.md) | Exact faction pose, field zoom, clip planes, handedness conversion, working bitmap D-pad states, and explicit production/A0 limits. |
| [P57B2B1 tactical target evidence](evidence/2026-09-13-tactical-target-control.md) | Source-recovered selected-object and retained-frame route, working bitmap target states, fallback centering, and explicit production-ID/A0 limits. |
| [P57B2B2 tactical layout evidence](evidence/2026-09-13-tactical-battle-layout.md) | Source-recovered active-force extent, docked-fighter exclusion, four tactical lanes, browser proof, and explicit production-placement/A0 limits. |
| [P57B2B3 tactical participant evidence](evidence/2026-09-13-tactical-participant-placement.md) | Stable DAT and fleet-roster identities, exact source X slots and faction lanes, selected-world-point targeting, and explicit resource-join/A0 limits. |
| [P57B2C1 tactical transform/palette evidence](evidence/2026-09-13-tactical-transform-palette.md) | Direct authored mesh coordinates, complete system-palette transport, exact two-system selection, and explicit render-state/A0 limits. |
| [P57B2C2A tactical light-rig evidence](evidence/2026-09-14-tactical-light-rig.md) | Exact source directional and ambient lights, transformed direction, deterministic browser probe, and explicit filtering/culling/A0 limits. |
| [P57B2C2B tactical render-state evidence](evidence/2026-09-14-tactical-render-state.md) | Source device defaults, explicit dither, filtering, culling, depth, material behavior, browser probe, and production/A0 limits. |
| [P58A tactical resource-join evidence](evidence/2026-09-14-tactical-resource-join.md) | Complete DAT-to-ordinal/resource joins, full 87-mesh/397-texture browser pack, representative participant probes, and production-draw/A0 limits. |
| [P58B production tactical-participant evidence](evidence/2026-09-14-tactical-production-participants.md) | Exact live capital-family rendering, paired 3D-off framebuffer proof, 36-case muted browser gate, and fighter/selection/A0 limits. |
| [P58C tactical fighter and selection evidence](evidence/2026-09-14-tactical-fighters-selection.md) | Source-mapped fighter detail resources, projection-aligned selection and targeting, paired controls, and explicit A0 limits. |
| [P58D fighter-detail journey evidence](evidence/2026-09-14-tactical-fighter-detail-journey.md) | Production indicator/far/close transitions, centered close sprites, no-reload proof, 40-case muted browser gate, and explicit A0 limits. |
| [P58E task-force and fighter-group evidence](evidence/2026-09-14-tactical-group-presentation.md) | Eight task-force controls, four RGBY fighter controls, selected portraits, source-shaped input, 44-case muted browser gate, and explicit A0 limits. |
| [P58F1 tactical-planet evidence](evidence/2026-09-14-tactical-planets.md) | Source-bound system selector, exact 256 by 256 planet and palette pair, 44-case muted browser gate, provisional placement, and explicit A0 limits. |
| [P58F2 tactical-impact evidence](evidence/2026-09-14-tactical-impact-effects.md) | Six source-selected hit, damage, and destruction families, exact 10 Hz lifecycle and priority, transparent target attachment, 48-case muted browser gate, and explicit A0 limits. |
| [P58F3 projectile and field evidence](evidence/2026-09-14-tactical-projectile-fields.md) | Retained projectile variants, exact thresholds, scales, colors, interpolation and lifecycle, both 10 Hz field families, gravity priority, 52-case muted browser gate, and explicit A0 limits. |
| [P58F4 selected-capital damage evidence](evidence/2026-09-15-tactical-selected-damage.md) | Panel 1302, source-ordinal capital portraits, matte transparency, live faction-correct shield and hull meters, 56-case muted browser gate, and explicit A0 limits. |
| [P58F5 subsystem and field-command evidence](evidence/2026-09-15-tactical-subsystem-field-commands.md) | Five subsystem-condition families, exact quantization and placement, exact field-source identity and capacity, 60-case muted browser gate, and explicit A0 limits. |
| [P58F6 live subsystem-damage evidence](evidence/2026-09-15-tactical-live-subsystem-damage.md) | Source-derived live shield overflow, hull loss, subsystem thresholds and limits, condition recalculation, tractor cancellation, 64-case muted browser gate, and explicit A0 limits. |
| [P58F7 subsystem-repair and mobility evidence](evidence/2026-09-15-tactical-subsystem-repair-mobility.md) | Source-derived repair cadence and selection, engine condition, tractor drag, complete field containment, 68-case muted browser gate, and explicit A0 limits. |
| [P58F8 maneuver and movement evidence](evidence/2026-09-16-tactical-maneuver-movement.md) | Recovered maneuver-state bonus, effective-power velocity, signed faction movement, 250-millisecond integration, 72-case muted browser gate, and explicit command/A0 limits. |
| [P58F9 tactical command-assignment evidence](evidence/2026-09-17-tactical-command-assignment.md) | Original command panels, exact order and tactic codes, source-shaped controls, cancel/commit behavior, group delivery, 76-case muted browser gate, and explicit execution/A0 limits. |
| [P58F10 tactical order-execution evidence](evidence/2026-09-17-tactical-order-execution.md) | Recovered maneuver waypoints, Hold stop behavior, Recover carrier reservation and Returning state, 80-case muted browser gate, and explicit completion/A0 limits. |
| [P58F11 tactical command-progression evidence](evidence/2026-09-18-tactical-command-progression.md) | Source-rate turning, maneuver arrival, full fighter recovery states, preserved squadron count, 84-case muted browser gate, and explicit A0 limits. |
| [P58F12 tactical attack-targeting evidence](evidence/2026-09-18-tactical-attack-targeting.md) | Typed Attack Fighters and Attack Capital Ships acquisition for capital and fighter owners, visible target endpoints, 88-case muted browser gate, and explicit A0 limits. |
| [P58F13 tactical attack-target lifecycle evidence](evidence/2026-09-19-tactical-attack-target-lifecycle.md) | Same-class target invalidation and replacement for capital and fighter owners, visible target endpoints, 92-case muted browser gate, and explicit A0 limits. |
| [P58-B06 tactical completion evidence](evidence/2026-09-22-tactical-completion-bundle.md) | Capital and fighter combat, collision, retained formations, Death Star and trench-run routes, original result/options surfaces, clean 120-case muted browser gate, and explicit A0 limits. |
| [P58-B07 tactical matrix-contract evidence](evidence/2026-09-22-tactical-106-matrix-contract.md) | Ledger-derived 106-cell denominator, exact family distribution, lossless A0 manifest contract, proprietary-artifact exclusion, and explicit 0/106 starting boundary. |
| [P58-B08 tactical A0-ingestion evidence](evidence/2026-09-22-tactical-a0-ingestion.md) | Exact original-executable provenance, 640×480/hash/ledger validation, ignored per-cell storage, replacement rejection, and explicit 0/106 offline-host boundary. |
| [P58-B09 tactical A1-crosswalk evidence](evidence/2026-09-22-tactical-a1-crosswalk.md) | Schema-validated journey/snapshot/negative-control classes, 82 mapped cells, 24 explicit scenario gaps, and unchanged 0/106 strict acceptance. |
| [P58-B10 tactical navigation/camera evidence](evidence/2026-09-23-tactical-navigation-camera.md) | Four navigation sets, ordered routes and targets, camera memory/chase, Anvil and Stand Off, 96 mapped cells, ten explicit gaps, and unchanged 0/106 strict acceptance. |
| [P58-B11 tactical trench-run/persistence evidence](evidence/2026-09-23-tactical-trench-persistence.md) | Production order-6 trench-run launch, exact 3-to-2 strategic capital/fighter persistence, 98 mapped cells, eight explicit gaps, and unchanged 0/106 strict acceptance. |
| [P58-B12 tactical Game Options/empty-space evidence](evidence/2026-09-23-tactical-game-options-empty-space.md) | Original COMMON Game Options routing, disabled in-battle display controls, empty-space starfield presentation, 100 mapped cells, six explicit gaps, and unchanged 0/106 strict acceptance. |
| [Evidence index](evidence/README.md) | Reports and artifact bundles, including a required `README.md` inventory for each new bundle. |
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
matrix open. PR #11 corrects `0x131` through `0x133`, F1/F7, and the GID
control's same-button open and close path without introducing substitute
windows. See the
[routing correction](evidence/2026-09-14-cockpit-routing-correction.md).
P48 restores the code-built GID command tree. P50 adds its eight
original frame tiles and removes the non-native fade. The
[frame evidence](evidence/2026-09-12-gid-menu-frame.md) leaves menu interior,
typography, and exact geometry open.
P51 removes the provisional row-hover wash and verifies the part of the GID
frame still visible beneath a detailed system window. Its
[evidence](evidence/2026-09-12-gid-hover-and-occlusion.md) leaves original
hover/focus presentation and full stacking acceptance open.
P52 replaces the active combat replacement panels with the original tactical
shell and first working bitmap pause, highlight, and zoom controls. Its
[evidence](evidence/2026-09-12-tactical-shell-controls.md) keeps procedural
battle contents and all 106 tactical cells open.
P53 applies the original palette-key hit rule to those working controls and
adds the extracted zoom pressed states. Its
[evidence](evidence/2026-09-12-tactical-control-hit-states.md) keeps drag,
repeat, camera, battle-content, A0, and complete tactical acceptance open.
P54 adds dependency-free, content-addressed raw staging for all 87 original
type-301 meshes and 397 type-303 texture or palette resources. Its
[evidence](evidence/2026-09-12-tactical-3d-staging.md) keeps decoding, runtime
transport, semantic mapping, rendering, and every tactical cell open.
P55 decodes the complete corpus into 87 mesh and 397 texture objects and checks
all original mesh topology, bounds, materials, and texture names against
Assimp 6.0.5 raw import. Its
[evidence](evidence/2026-09-12-tactical-3d-runtime-pack.md) keeps browser
transport, semantic mapping, rendering, and every tactical cell open. P56
packages and visibly renders one exact source-bound pair in a proof/control
browser matrix. P57A packages the related `2561` and `2562` meshes and
`SDESTI_M.BMP`, then proves the recovered high- and reduced-detail LOD rule.
Its [evidence](evidence/2026-09-13-tactical-3d-lod-family.md) keeps native GPU
rendering, original camera and render-state acceptance, semantic mapping,
fleet integration, and every tactical cell open.
P57B1 traces `FUN_005c1160` and proves a live same-renderer
close-medium-far-medium-close sequence with exactly one family load. Its
[evidence](evidence/2026-09-13-tactical-live-lod-journey.md) keeps the original
camera, render-state, A0/A1, production binding, and every tactical cell open.
P57B2A then recovers the exact initial faction poses, field and clip constants,
zoom/orbit/pitch command switch, and handedness conversion, and proves the four
bitmap D-pad directions in a seven-state muted browser journey. Its
[evidence](evidence/2026-09-13-tactical-camera-contract.md) keeps general battle
layout, remaining render state, A0 comparison, production binding, and every
tactical cell open. P57B2B1 activates the center target control from switch
case 9, verifies its 1058/1059 states and selected-object record, and centers
the production two-dimensional fallback. Its
[evidence](evidence/2026-09-13-tactical-target-control.md) keeps stable DAT and
tactical identity binding, source world positions, A0 comparison, and every
tactical cell open.
P57B2B2 then recovers `FUN_005ab650`'s active-force object count, extent, and
four tactical lanes, including docked-fighter exclusion. Its
[evidence](evidence/2026-09-13-tactical-battle-layout.md) keeps production
identity and placement open. P57B2B3 then binds stable DAT and original
fleet-roster identities to the exact X slots and faction lanes, and targets
the selected source-world point. Its
[evidence](evidence/2026-09-13-tactical-participant-placement.md) keeps the
DAT-to-tactical-resource join, production 3D rendering, A0 comparison, and
every tactical cell open. P57B2C1 preserves authored mesh coordinates instead
of applying an inferred center-and-fit transform, stages all 27 original
palettes, and selects `5530 + SYSTEMSD.picture_id`. Its
[evidence](evidence/2026-09-13-tactical-transform-palette.md) kept lighting,
filtering, culling, the production resource join, A0 comparison, and every
tactical cell open. P57B2C2A then replaces the guessed proof lighting with the
source `FUN_005d4d10` directional and ambient rig. Its
[evidence](evidence/2026-09-14-tactical-light-rig.md) keeps filtering, culling,
the production resource join, A0 comparison, and every tactical cell open.
P57B2C2B then restores the source filtering, culling, depth, dither, shading,
specular, and diffuse/emissive material behavior. Its
[evidence](evidence/2026-09-14-tactical-render-state.md) keeps the production
resource join, A0 comparison, and every tactical cell open. P58A then joins all
capital ships, fighters, and the Death Star from stable DAT identities to the
original tactical resources and validates full-corpus browser transport. Its
[evidence](evidence/2026-09-14-tactical-resource-join.md) keeps production
family drawing, A0 comparison, and every tactical cell open.
P58B then loads exact joined capital families on demand and draws live
production participants at the recovered source positions. Its
[evidence](evidence/2026-09-14-tactical-production-participants.md) keeps
fighters, selection presentation, effects, commands, results, A0 comparison,
and every tactical cell open.
P58C replaces the synthetic fighter markers with original type-303 detail
resources and moves capital selection, focus, and targeting onto the projected
mesh bounds. Its [evidence](evidence/2026-09-14-tactical-fighters-selection.md)
keeps the complete view and interaction matrix open. P58D proves the full
fighter-detail journey. P58E restores the task-force and RGBY fighter-control
strip, source-shaped input, keyboard routes, mutual exclusion, and selected
fighter portraits. P58F1 restores the exact system-selected planet and paired
palette. P58F2 restores all six source-selected target-impact families with
their exact frame counts, draw sizes, 10 Hz lifecycle, priority, transparency,
and source-position attachment. P58F3 restores retained projectile meshes,
threshold branches, colors, interpolation, duration, and tractor/gravity field
animation and priority. P58F4 and P58F5 restore the selected-capital panel,
portrait, hull and shield meters, five subsystem-condition families, and exact
field-source identity and capacities. P58F6 connects live capital and fighter
damage to those subsystem states and cancels depleted tractor sources. P58F7
restores the repair timer and subsystem selection plus the engine-condition and
tractor-drag mobility calculation. P58F8 restores the maneuver-state bonus,
effective-power velocity, and millisecond position integration. P58F9 restores
both bitmap-driven command panels, exact order and tactic codes, disabled
states, and selected-unit commit and cancel behavior. P58F10 connects the four
maneuver orders to their recovered waypoint constructors, executes Hold, and
starts Recover through carrier reservation and Returning state 2. P58F11 adds
source-rate signed turning, deterministic waypoint completion, and the Docking
and Recovered fighter states while preserving strategic squadron counts.
P58F12 restores typed Attack Fighters and Attack Capital Ships target
acquisition across capital and fighter owners while preserving valid live
engagements. P58F13 adds live target invalidation, stable same-class
replacement, and exhausted-list clearing without random or cross-class
fallback. The P58-B06 checkpoint adds source-backed capital and fighter
combat, collision, automatic grouping, forward-relative retained formations,
the separate Death Star object, original result/options panels, and both
trench-run routes. Exact planet placement, global RNG sequencing, original
arrival callbacks, power allocation, native beam behavior, shared post-battle
bombardment, landing, and navigation orchestration, native playback, A0
comparison, and every tactical cell
remain open.
Runtime pack v3 now
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
