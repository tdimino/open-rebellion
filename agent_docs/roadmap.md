---
title: "Roadmap"
description: "Audit-driven milestones M0-M5 and the v1.0 release, addons, and known technical debt"
category: "agent-docs"
created: 2026-03-11
updated: 2026-10-19
tags: [roadmap, planning, milestones, parity]
---

# Roadmap

<!-- interface-parity-status:start -->
Required interface families: 43. Complete: 2. Partial: 15. Failing: 26. Blocked: 0. Strictly accepted cells: 28/627.
The current CMD-02 GID catalog has 38 scenarios (29 baseline cells and nine additional native filter variants) and 152 faction/viewport executions. Its strict original-evidence and cross-browser gate remains open.
<!-- interface-parity-status:end -->

Where the project stands is in the [full-functionality audit](../docs/qa/2026-09-08-full-functionality-audit/README.md):
its feature ledger and evidence files, and in `audit-report.md` beside it
"Where work left off", the evidence contract and the definition of done. Its
sister [original-interface audit](../docs/qa/2026-09-10-interface-parity-audit/README.md)
owns visible surface, bitmap, geometry, hotspot, navigation, animation and
audio acceptance. Their JSON ledgers hold the finding, feature and cell IDs
cited below; each ticked item names its evidence, and each open item keeps its
acceptance criteria.

Delivery notes from March and April 2026 are in
[archive/roadmap-history-2026-03-to-04.md](../archive/roadmap-history-2026-03-to-04.md);
they establish no current parity.

## Audit-Driven Parity, Browser, and Multiplayer Roadmap

### M0: Truth and Critical Integration (1 week)

- [x] F-001 Save, Load, Delete and the main-menu save picker: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-save-delete.md)).
- [ ] Surface browser corruption/quota errors and move production saves to asynchronous IndexedDB.
- [x] F-014A native HD root, `TROOPSD.DAT` prefetch and the deterministic `runtime.orpk`: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-runtime-pack.md)).
- [x] F-010B fleet miniatures for all 38 fighter and capital-ship records: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-fleet-miniatures.md)).
- [x] F-011A save-state fingerprint: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-state-fingerprints.md)).
- [x] F-011B1 save continuation envelope: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-state-continuation.md)).
- [x] F-016A cockpit command-to-animation table: pass ([reference](main-menu-parity.md); [evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-cockpit.md)).
- [x] F-016B cockpit faction, difficulty, galaxy size and game type persist in saves: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-game-setup-propagation.md)).
- [x] F-016C / P03-P04 Credits, Multiplayer, audio controls and campaign reset: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-completion.md)).
- [x] F-016D main-menu browser semantics: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-semantics.md)).
- [x] F-016E / P03 final presentation and the music-only extension: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-music-toggle.md)).
- [ ] Make a clean packaged browser artifact boot in CI.
- [x] README parity claims link to current acceptance evidence: done (`README.md`).
- [x] F-011B2 replay format v1: pass ([contract](deterministic-replay.md); [evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-contract.md)).
- [x] F-011B3 replay recorder and executor: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-execution.md)).
- [x] F-011B4 native/WASM replay equivalence: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-wasm-equivalence.md)).

### M1: Simulation Correctness and Determinism (2–3 weeks)

- [x] F-007A in-transit fleet redispatch prevented: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-fleet-redispatch.md)).
- [x] F-007B explicit fleet position, no production onto fleets in transit: pass; its arrival merge is retired as unsourced (`ghidra/notes/fleet-join-split.md`) ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-fleet-position-consolidation.md)).
- [ ] Wire ordinary player fleet dispatch through the validated authoritative
  departure path and pass independent bitmap acceptance for both factions
  ([F-007C evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-player-fleet-dispatch.md)).
  Reopened 2026-10-01: the egui context menu that set the destination lost
  its caller in 23d15da, so the player cannot move a fleet, and the fleet
  panel's destination banner and Dispatch button are unreachable. The
  original Move order (below) restores it: menu Move, Confirmed Move, and the
  system window drag pass a two-sided browser gate (2026-10-02), and the
  blockade bit follows the original. Regiments load through the Fleet window
  (2026-10-04, below). A native check remains.
- [x] F-007D one bounded engagement per system: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-system-combat-resolution.md)).
- [ ] Extend stable ordering and versioned random streams through five 5,000-tick campaign seeds.
- [x] F-007E logistics checkpoint (AI production control, transit, HQ defender, troop classes, repair in saves): pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-ai-campaign-logistics.md)).
- [x] F-007E conquest checkpoint (transport, landing, surface battles, occupation): pass; capture stays open below ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-troop-transport-occupation.md)).
- [ ] Add player troop selection to the bitmap fleet workflow with live
  capacity, authoritative embarkation, transit, and automatic landing
  ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-player-troop-dispatch.md)).
  Reopened 2026-10-02: the fleet panel's troop picker lost its caller in
  23d15da and was deleted in F-007C phase 6a, so no player path loads
  regiments onto a fleet. Restored 2026-10-04 through the original order: a
  regiment's Move released on the Fleet window's fleet, opened from the
  sector window's fleet icon; the `fleet-window.mjs` gate passes both sides
  (load, hold, move, land, full-fleet refusal). A native check remains.
- [x] F-007E cited campaign-history baseline: done ([reference](../docs/reference/campaign-history/)).
- [x] F-007E victory semantics (Coruscant, mobile HQ, Standard conjunctions, nonterminal Death Star loss): pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-victory-contract.md)).
- [ ] Replace deterministic mass capture with validated capture/evasion
  behavior.
- [ ] Establish faction liveness and the full cited campaign loop, including
  production, diplomacy, recruitment, intelligence, research, blockade
  follow-up, uprisings, bombardment, HQ relocation, and principal missions.
  Do not require symmetric attack counts from an asymmetric game.
- [ ] Enable AI Death Star construction/fire/cleanup and repair parity oracles.
  `ghidra/notes/death-star-strategic.md` recovers the original: an ordinary
  shipyard build of CAPSHPSD record 136, one per system; fire is order
  `0x223` (gate `FUN_00582440`, no side or owner test); the destroy cascade
  leaves fleets standing; sabotage has no build-delay rule (F-017). The
  orders, the gate and the per-hit run are ported
  (`docs/plans/2026-10-16-feat-orbital-strike-and-bombardment.md`). Open:
  `or-port-destroy-system-cascade-imrv` (P1; an Alliance-player native
  Destroy System and the browser probes) and whether the AI fires it,
  `or-q-ai-death-star-use-z9ow`.
- [ ] Wire the recovered rules the 2026-09-25 test-pruning pass found uncalled:
  research gating of buildable ships (F-018), the decoy roll and the SUBDMSTB
  Subdue success check (F-019; both now run, and its remaining phases are
  tracked below), mod dependency errors (F-020), and blockade troop
  destruction (F-021; tracking now persists in save v15, browser pass
  pending).
- [ ] Correct or re-source the code behaviors the 2026-09-26 citation audit
  found without a binary source: the economy's KDY production modifier
  (F-023) and the Emperor's 1.5x battle bonus (F-024). Death Star detection
  keys on capital-ship family `0x18` as the original does; its shield's
  battle role waits on the TAC-05 cells (F-025,
  `ghidra/notes/death-star-strategic.md`). The revolt lifecycle, uprising
  incident, and disaster now follow recovered code (F-026, browser pass
  pending).
- [ ] Move the remaining probability-table consumers to the recovered step
  lookup (F-028) and the informant and resource incidents to their recovered
  timers (F-029).
- [x] F-030 en-route delivery of manufactured objects: ported (save v16); browser pass pending (audit F-030).
- [ ] Port the mission phases, decoys, and per-member success roll (F-019).
  The rules are in `ghidra/notes/decoy-roll.md`, `mission-lifecycle.md`,
  `ai-mission-planning.md`, and `mission-dialog.md`. Phases 0-6 have landed:
  the lifecycle, detection, the per-member roll, the recruit pool, sabotage
  targets, legality, the AI team and decoy selectors, the original mission
  dialog, dispatch telemetry, and the dialog's browser gate. Phase 7 ports the
  original entry (right click, Mission, targeting cursor, target), removes the
  invented entry points, and traces refusals to the advisor; the entry passes
  its browser gate for both sides. A native check remains.
- [ ] Port the Move and Confirmed Move orders (`0x201`/`0x202`): the pop-up
  menu items, targeting with the `+0x70` hit test, and the list drag
  (`FUN_006083c0` notification `0x29a`). This restores player fleet movement
  (F-007C). Retire, the Command submenu, and Status share the menu. Phases
  0-8 have landed and pass the `fleet-move.mjs` and `fleet-window.mjs` gates
  for both sides; a native check remains.
- [x] F-007C system blockade bit follows `FUN_0050b8e0`: ported (`ghidra/notes/blockade-bit.md`).
- [x] F-007E regiment loading through the Fleet window: ported, `fleet-window.mjs` passes both sides; native check under F-007C (`ghidra/notes/fleet-window.md`).
- [x] F-007C sector window quadrant icons and the System, System Defenses and Missions windows: ported, `sector-quadrants.mjs` passes both sides; native check open (`ghidra/notes/sector-quadrants.md`).
- [x] F-007E unloading a regiment by hand and regiment travel: ported, `fleet-window.mjs` passes both sides; native check open (`ghidra/notes/regiment-unload.md`).
- [x] F-007C joining and splitting fleets: ported, `fleet-window.mjs` passes both sides; native check open (`ghidra/notes/fleet-join-split.md`).
- [x] F-007C Fleet Finder: ported, `fleet-finder.mjs` passes 18/18 on both sides; transport-limited native journeys open ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-10-05-f007c-native-interface-acceptance.md)).
- [ ] Balance one-ship fleets in the AI. With the arrival merge retired, the
  AI's own fleets no longer combine where they meet: in seed 42's 1,500-tick
  dual-AI run the fleet count at its six snapshots runs 5, 6, 11, 11, 17, 17
  where it ran 4, 4, 9, 9, 15, 15. Teach the AI to avoid lone-ship fleets,
  or join them by condition, and measure against that run (Tom,
  2026-10-05).
- [ ] Source or retire the port's automatic landing (F-007E: cargo lands
  when its side alone orbits a system). The unloading trace finds no
  original path that lands cargo without an order
  (`ghidra/notes/regiment-unload.md`, "What happens to the rest of the
  cargo"); it stays until traced (Tom, 2026-10-04).
- [ ] Port targeting of characters and objects (`+0x68` hit test, the walk up
  from a team member) so the dialog offers Rescue, Assassination, and
  Abduction (F-019).
- [ ] Trace the galaxy view's right click on a system or fleet, then port
  what it opens.
- [ ] Sort a refused order's status by its first word as the advisor does
  (P34; `FUN_004c3bb0`, `FUN_004c19f0`; `ghidra/notes/mission-dialog.md`,
  "Refusal"). Every refusal (missions, fleet, regiment and squadron moves)
  already reaches `AdvisorState::refuse` and files no text; any status but a
  repeat takes the generic reaction.
- [ ] Add the original mission kinds the port lacks: Reconnaissance, Palace,
  Bounty, Jedi Training, and the leisure and movement kinds (F-040).
- [x] F-031 provenance gate on uncited simulation rules: in place, and its baseline may only shrink (`tools/provenance-scan`).
- [x] F-032 fleet transit time and bombardment: ported from `FUN_0055d8c0` and orders `0x220`-`0x223`; fog open (audit F-032).
- [ ] Replace the invented rules found by the invention review, one system at
  a time: fog (F-032), strategic combat (F-033), missions
  (F-034), manufacturing and repair (F-035), AI (F-036), economy and research
  (F-037), character timers and story events (F-038), and false notes (F-039).
- [ ] Pass five reproducibly configured 5,000-tick seeds: transit ≤10%, orders
  ≤1.5× arrivals, and fleet arena ≤3× initial. Record 50–400 engagements, eight
  systems, busiest-system ≤40%, and 10% minority-faction attack share as
  diagnostics until original telemetry or calibrated playtests justify hard
  lower bounds.

Parameter autoresearch remains paused until this milestone closes; otherwise it
would tune around known simulation feedback defects.

### M2: One Authoritative Game Engine (3 weeks)

- [ ] Route the app and playtest through one simulation tick API and event sink.
- [ ] Move the panel-action handling out of `rebellion-app/src/main.rs` into
  modules native tests reach; every mutation run leaves survivors there.
- [ ] Make automatic, tactical, and ground combat share resumable core state and calculations. Unplayed battles run the tactical engine headless; the Planetary Assault (`assault.rs`) resolves in one pass with nothing to resume.
- [ ] Construct the victory modal and unify win/loss transitions.
- [ ] Remove `AdvanceTicks` or make it execute the real simulation.
- [ ] Prove identical fingerprints for the same seed and command stream across app/playtest, native/WASM, and auto/tactical paths.

### M3: Exceptional Browser Runtime (3 weeks)

- [x] F-014A deterministic `runtime.orpk` startup in four requests for both factions: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-runtime-pack.md)).
- [ ] Complete `UIP-T01` through `UIP-T05`: replace the synthetic strategy,
  object-window, report, and tactical surfaces with the original bitmap-driven
  compositions for both factions. No invented visible control may satisfy a
  parity cell.
- [ ] Bring the ground assault to parity (`UIP-F-010`, `TAC-08`,
  `RE-GND-01`).
  - The original report-only flow is ported (manual pp. 119-121, figs. 2.58,
    3.66 and 3.67; `ghidra/notes/ground-assault.md`): the fleet's Planetary
    Assault order (`assault::order_enabled`, greyed out against two
    planetary shields), immediate resolution (`assault::resolve`), and the
    Operation Report window (`rebellion-render/src/operation_report.rs`),
    filed to both sides. The invented ground-combat screen is gone.
  - Cover both factions, win and loss, casualties, occupation and the return
    to the map.
  - Close the eight `TAC-08` cells against A0 captures of the original.
    All eight references are captured (an A0-only family outside the
    106-cell tactical denominator; saves `empire-day53-yaga-orbit` and
    `empire-day128-assault-fails-selonia`).

Strategic interface checkpoints. Each is a bounded checkpoint; the cells it
leaves open belong to the family named.

- [x] P44 / `RE-ADV-01` advisor idle frames in native and WASM: pass; actions and voice under `RE-ADV-02` ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-authentic-droid-advisors.md)).
- [x] P46A / `RE-STR-01` strategic shell canvas: checkpoint; `UIP-T01` open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-11-strategic-shell-canvas.md)).
- [x] P46B / `RE-STR-02` cockpit command bitmaps and input: checkpoint; A0 and `UIP-T01` open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-11-strategic-command-controls.md)).
- [x] `UIP-B01` sector and detailed-system shells and the rail: checkpoint; `CMD-03` and `CMD-04` open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-11-strategic-window-navigation.md)).
- [x] Strategic-window routing boundary and app coordinator: infrastructure; each window family still migrates onto it ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-12-strategic-window-routing-coordinator.md)).
- [x] P46D detailed-system tab items: checkpoint; nested compositions, drag, intelligence and A0 open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-11-detailed-system-tab-items.md)).
- [x] P47A / `UIP-B02` STRATEGY 902 galaxy backdrop (the Display Off resource): pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-11-authored-galaxy-backdrop.md)).
- [x] P47B default Popular Support GID view: checkpoint; `CMD-02` open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-11-popular-support-gid.md)).
- [x] GID browser harness and 38-scenario catalog: in place, 152 probes pass; no original capture accepted, `CMD-02` open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-gid-browser-harness.md)).
- [x] P49 Message Index rail resting artwork: pass; `CMD-08` open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-message-index-rail.md)).
- [x] P60 Game Speed control, stop-day pause and accelerators (save v14): checkpoint; `CMD-05` and `CMD-08` open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-24-game-speed-recovery.md)).
- [x] P63 Message Index shell: checkpoint; `CMD-08` open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-28-message-index-shell.md)).
- [x] P50 GID root and submenu frame: checkpoint; `CMD-02` open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-gid-menu-frame.md)).
- [x] P51 GID row-hover wash removed: checkpoint; original hover art and stacking open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-gid-hover-and-occlusion.md)).

Encyclopedia (`OBJ-01`): all thirteen cells pass against the original under
Wine, accepted by Tom on 2026-10-07; native-Windows parity is not claimed.

- [x] P62 EDATA artwork transport: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-28-encyclopedia-artwork-transport.md)).
- [x] P64 index shells: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-30-encyclopedia-index-shell.md)).
- [x] P65 index catalog, corrected by P68 to 346 visible topics: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-08-encyclopedia-visible-catalog-correction.md)).
- [x] P66A topic sources: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-01-encyclopedia-topic-source-bindings.md)).
- [x] W2 content session: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-06-encyclopedia-content-session.md)).
- [x] W3 presenter: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-06-encyclopedia-presenter.md)).
- [x] W4 topic surface: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-06-encyclopedia-topic-surface.md)).
- [x] W5A canonical publication: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-06-encyclopedia-canonical-publication.md)).
- [x] W6 opt-in native faithful-HD selection: pass; browser stays original-only ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-06-encyclopedia-hd-selection.md)).
- [x] W7 native mod overlays: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-07-encyclopedia-mod-overlays.md)).
- [x] E32 production activation: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-07-encyclopedia-production-activation.md)).
- [x] E34 assembled technical acceptance: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-07-encyclopedia-technical-acceptance.md)).
- [x] E35 original comparison: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-07-encyclopedia-a0-compatibility.md)).

Tactical (`TAC-01` through `TAC-07`, 106 cells). Each checkpoint below passes
its muted browser gate; every strict cell waits on the A0 matrix after them.

- [x] P52 tactical shell (TACTICAL 1000) and bitmap controls ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-tactical-shell-controls.md)).
- [x] P53 tactical control hit masks and held states ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-tactical-control-hit-states.md)).
- [x] P54 tactical 3D resource staging ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-tactical-3d-staging.md)).
- [x] P55 tactical mesh, image and palette conversion ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-tactical-3d-runtime-pack.md)).
- [x] P56 single-resource browser rendering ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-3d-render-proof.md)).
- [x] P57A three-LOD selection ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-3d-lod-family.md)).
- [x] P57B1 live cached-slot switching ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-live-lod-journey.md)).
- [x] P57B2A source camera and D-pad ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-camera-contract.md)).
- [x] P57B2B1 selected-object target ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-target-control.md)).
- [x] P57B2B2 active-force layout ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-battle-layout.md)).
- [x] P57B2B3 participant placement ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-participant-placement.md)).
- [x] P57B2C1 authored transforms and system palettes ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-transform-palette.md)).
- [x] P57B2C2A light rig ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-light-rig.md)).
- [x] P57B2C2B retained-mode render state ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-render-state.md)).
- [x] P58A tactical resource identity ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-resource-join.md)).
- [x] P58B production capital-ship families ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-production-participants.md)).
- [x] P58C fighters and projected selection ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-fighters-selection.md)).
- [x] P58D fighter-detail journey ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-fighter-detail-journey.md)).
- [x] P58E task-force and fighter-group controls ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-group-presentation.md)).
- [x] P58F1 system-selected planets ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-planets.md)).
- [x] P58F2 impact effects ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-impact-effects.md)).
- [x] P58F3 projectiles and fields ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-projectile-fields.md)).
- [x] P58F4 selected-capital damage panel ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-15-tactical-selected-damage.md)).
- [x] P58F5 subsystem and field-source presentation ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-15-tactical-subsystem-field-commands.md)).
- [x] P58F6 live subsystem damage ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-15-tactical-live-subsystem-damage.md)).
- [x] P58F7 subsystem repair and mobility ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-15-tactical-subsystem-repair-mobility.md)).
- [x] P58F8 maneuver and movement ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-16-tactical-maneuver-movement.md)).
- [x] P58F9 command assignment panels ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-17-tactical-command-assignment.md)).
- [x] P58F10 order execution ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-17-tactical-order-execution.md)).
- [x] P58F11 command progression and fighter recovery ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-18-tactical-command-progression.md)).
- [x] P58F12 attack target acquisition ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-18-tactical-attack-targeting.md)).
- [x] P58F13 attack-target lifecycle ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-19-tactical-attack-target-lifecycle.md)).
- [x] `UIP-B06` practical space-battle bundle, P58-B06 through P58-B22: implemented, 144/144 final muted matrix ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-28-tactical-source-completion.md); the B-series files sit beside it).
- [ ] Pass the strict 106-cell tactical matrix against A0 captures of the
  original. 113 of 114 references are captured in the capture VM, with
  TAC-03-C020 settled without a frame. The first strict C6 verdict was pass
  11, fail 66, not-run 28, settled 1, with the negative controls failing
  their rows (6 of 6). The remaining work is the task graph
  (`agent_docs/task-graph.md`, `or-ep-tac-x15`): one acceptance node per
  failing region, fed by Ghidra questions, diagnoses and the fixtures that
  let the not-run cells run. Exact beam dimensions and timing, native
  playback comparison and rare audio routes close with it.

Further M3 work:

- [ ] After original parity, use a separate Fable and `/ce:plan` design pass for
  an optional X-Wing Alliance-style skirmish setup menu. It may select ship
  distribution, planet, and space background, but it must reuse the production
  battle scene and remain outside the original 106-cell parity denominator.
- [ ] After original 640×480 parity, add an optional enhanced-widescreen layout.
  Preserve authentic bitmap panels and control sizes while extending only the
  galaxy, system, and tactical viewports into the additional horizontal space.
  Treat genuine 16:9 support as a separate modernization track outside the
  original 106-cell parity denominator:
  - anchor authentic cockpit panels to the screen edges;
  - extend backgrounds without stretching their bitmaps;
  - adjust tactical-camera projection and galaxy-map bounds;
  - translate pointer coordinates through the widescreen layout;
  - preserve original hit masks and control geometry;
  - pillarbox videos and other fixed 4:3 surfaces; and
  - maintain separate visual baselines for supported 16:9 resolutions.
- [ ] Execute `RE-ADV-02` for authored SPT/BIN/FDT action, cadence, preemption,
  and voice mappings; extend the decoder/pack to briefings and every other
  required resource kind; then consume the proven shell/control/window mappings.
  Done: triggered droid actions, voices, and the opening briefing tour with
  its skip, both factions, native and packed (`ghidra/notes/droid-advisor-triggers.md`).
  Open: the Advice topics the tour leaves, droids during the battle dialog.
- [ ] Add Brotli compression plus bounded raw-byte and decoded-texture caches to the verified runtime-pack foundation.
- [ ] Complete P45 faithful-HD modernization. P45A is complete: original parity
  is separate from opt-in HD, manifests are source-bound, and deterministic
  indexed scaling plus fail-closed fallback are verified. P45B-P45E own complete-family model evaluation, protected masks,
  optional browser packs, and cross-browser acceptance.
- [ ] Enable high DPI; use one egui pass; cache sector geometry.
- [ ] Move saves from synchronous base64 `localStorage` to compressed asynchronous IndexedDB.
- [ ] Add authored advisor voice and preemption after user gesture; advisor idle
  assets are now staged from a contributor-owned installation. Voices play
  natively and are packed for the browser; browser playback is unverified.
- [ ] Exercise real mouse/keyboard input and bitmap screenshots in Chrome, Firefox, and Safari.
- [ ] Meet budgets: ≤3 s cold start at 50 Mbps/30 ms, ≤4 pre-menu requests, ≤8 ms frame, ≤12 ms WASM tick at 1,000 fleets, ≤5 MB optimized WASM, and ≤256 MB combined memory after 10 minutes.

### M4: Authoritative Multiplayer (6–8 weeks after M1–M3)

- [ ] Add validated `{player, tick, sequence}` commands and route single-player through the same boundary.
- [ ] Add `rebellion-net` and `rebellion-server` responsibilities for authoritative simulation and fog-filtered faction deltas/snapshots.
- [ ] Start with in-process test transport and secure WebSockets; never expose the hidden full `GameWorld` to clients.
- [ ] Add prediction/reconciliation, reconnect snapshots plus command log, persistence, authentication, rate limits, TLS, and observability.
- [ ] Pass two clients for 5,000 ticks with hashes matching every 250 ticks; no 200 ms RTT input stalls; ≤1 reconcile per 100 commands; reconnect ≤60 s; reject 100% of illegal commands.
- [ ] Consider trusted lockstep/WebRTC only after deterministic replay is proven.

### M5: Continuous Release Proof (parallel from M0)

- [ ] Restore a reviewed CI provider; GitHub Actions workflow definitions are intentionally local/untracked as of 2026-09-08, so current gates are manual.
- [ ] Enforce format, strict clippy, native, packaged-WASM, and clean-package boot gates.
  `make all` defines tests, formatting, strict all-target Clippy and the native
  build, but runs Cargo in place, so it does not run on this Mac; formatting and
  strict Clippy fail on the audited baseline. Packaged browser and CI
  enforcement remain open.
- [ ] Add at least 20 app/browser integration cases and promote long campaigns when stable.
- [ ] Generate the complete bitmap resource ledger from the entity map; fail on unexplained misses. Fleet-list miniatures are verified in F-010B.
- [x] Interface baseline of 627 required cells across 43 families, 106 of them tactical (`TAC-01` through `TAC-07`): established ([interface audit](../docs/qa/2026-09-10-interface-parity-audit/README.md)).
- [ ] Close every required surface-state cell in the original-interface audit
  with authoritative captures, exact-resource provenance, screenshot and hotspot
  comparison, complete navigation edges, and zero unknown or invented pixels.
- [ ] Retain screenshot, console, network, performance, data-hash, and save-schema evidence. F-010B fleet and F-014A runtime-pack evidence are retained; the full ledger remains open.
- [x] P40 project documentation on GitHub Pages: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-github-pages.md)).
- [ ] Close every supported P00–P40 functionality pass and every required
  interface surface-state cell from release artifacts before claiming 100%
  functionality.

### v1.0: Password-Protected Cloudflare Release

- [ ] Deploy the self-contained browser artifact to Cloudflare Pages with Pages Functions middleware.
- [ ] Keep `SITE_PASSWORD` and `SESSION_SECRET` in encrypted Wrangler/Pages secrets; commit no credentials or reusable auth token.
- [ ] Sign an expiring `HttpOnly; Secure; SameSite=Lax` session cookie and provide logout; reject missing, expired, or tampered sessions.
- [ ] Gate HTML, WASM, DAT, bitmap/HD packs, save endpoints, WebSockets, and multiplayer APIs—not only the landing page.
- [ ] Add CSP and security headers plus immutable caching for hashed public assets; never publicly cache authenticated HTML or auth responses.
- [ ] Maintain preview and production environments, commit-linked deploys, smoke checks, retained artifacts, and documented rollback.
- [ ] Have `/codex-orchestrator` run an independent browser-use reviewer against the deployed URL for authentication, single-player, multiplayer, input, performance, and bitmap screenshot acceptance in Chrome, Firefox, and Safari.

v1.0 exits only when the protected Cloudflare deployment passes P39, the
GitHub Pages documentation passes P40, and every supported earlier pass. This follows the local Pages `functions/_middleware`
pattern while replacing hardcoded passwords/cookie values with encrypted
secrets and signed sessions.

---

## Roadmap: Addons (v0.15.0+)

### Addon 1: Create Your Own Character (Demiurgos)
*Plan at `docs/plans/2026-03-21-addon-create-your-own-character.md`*

**Phase 1: Homeworld System** (universal—enriches base game)
- [ ] Add `homeworld: Option<SystemKey>` to Character
- [ ] Create `data/homeworlds.json` with canon homeworld assignments (Luke→Tatooine, etc.)
- [ ] HomeworldModifier: +10% at homeworld, -5% when enemy controls, -15 loyalty on fall
- [ ] Wire into missions.rs probability + betrayal.rs loyalty
- [ ] Display in the character's Status window and Encyclopedia

**Phase 2: Character Creation Core**
- [ ] Add `CharacterOrigin`, `AcademyArchetype`, `ArchetypeTemplate` to rebellion-core
- [ ] 6 archetypes: Diplomat, Operative, FleetOfficer, GroundCommander, Engineer, Fringer
- [ ] 7-step creation UI as egui modal (faction → archetype → skills → homeworld → Force roll → name → review)
- [ ] Point-buy within archetype floors/ceilings (5-point increments)
- [ ] Max 3 custom characters per game

**Phase 3: Force Sensitivity**
- [ ] 8-16% base chance depending on species + archetype
- [ ] Success: jedi_probability 15-30 (Luke = 100—custom characters strictly inferior)
- [ ] Wire to existing JediSystem progression

**Phase 4: Portraits & Flavor**
- [ ] 128 portrait BMPs (8 per species × 2 factions × 8 species)
- [ ] Backstory templates: "Former Separatist Holdout", "Outer Rim Survivor", "Imperial Defector"
- [ ] Encyclopedia integration

**Phase 5: Mod Integration**
- [ ] Extend mod loader for `"action": "add"` entity creation
- [ ] DatId allocation from reserved custom range (0xFF000000+)
- [ ] Example mod: "Kira Noss" character pack

### Addon 2: Expanded Galaxy (Future)
- Additional star systems from EU (200 → 400)
- New sectors (Unknown Regions, Corporate Sector)
- Hyperspace lane network (optional overlay on Euclidean model)

### Mod surface (Ghidra groundwork)
The tactical engine resolves every space battle, played or not; its parity
is tracked under `or-ep-tac-x15`. Before addons open content to mods, the
`or-ep-mod-surface-vdch` questions (all answered) map what the original
fixes in code rather than data: Force users and training, the story encounters, mission
families, ship-type assets, the 200-system galaxy, the movies, the
per-side switches and the two-side limit, the object ids the executable
names directly, the Death Star, record limits, the character roster, ground
units, audio and text. Each answer names the constants, family checks and
hard-coded ids the port must turn into data for new Jedi, encounters,
missions, ships, systems, movies, sounds and translations, and for era
overhauls that rename the sides, replace the scenario or add a superweapon.
`agent_docs/mod-surface.md` collects the answers and the rules for exposing
a fixed rule as mod data.

- [ ] Port what the answers found the port lacks or invents: officer rank
  and command orders `0x260`-`0x263` (`or-port-officer-rank-x4kn`),
  majors-only recruiters and major-first mission leads
  (`or-port-recruit-major-lead-8grm`), the original music selector in place
  of `track_for_context` (`or-port-music-selector-v9nd`), the Force fields
  (`or-port-force-fields-2u0m`), and the post-battle pass for system
  squadrons (`or-port-system-squadron-pass-oa3p`).
- [ ] Decide whether mods may append records. The original loads any count
  but has no class for a new family byte and the port's loader skips
  patches with unknown ids (`ghidra/notes/record-registry.md`); allowing
  new records is an architecture change.

---

## Known Technical Debt

- dat-dumper in `tools/` is also a library dep of rebellion-data; it works but is unconventional
- Browser data now loads from deterministic `runtime.orpk`; compression, HD
  entries, and bounded caches remain open.
- No additive entity creation in mods; patches only modify existing entities (until Addon 1 Phase 5)
- 27 LOC dead code identified by simplicity reviewer (2026-03-21)
