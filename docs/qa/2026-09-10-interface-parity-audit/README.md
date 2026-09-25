# Open Rebellion Interface Parity

This folder is the source of truth for restoring every visible surface from
*Star Wars: Rebellion* and *Star Wars: Supremacy*. The target is the original
bitmap-driven interface. Replacement panels, invented controls, approximate
geometry, and unverified asset mappings do not count as parity.

The current P58-B19 space-battle checkpoint maps all 106 canonical cells to
deterministic A1 evidence through 87 journeys and 19 snapshots. Capital and
fighter combat, tactical commands,
original results and options panels, resource 5030 stars, empty-space
presentation, native withdrawal confirmation, destroyed-state presentation,
selected-ship contents, direct Escort, Battle Alert entry, tactical score and
the complete 22-variant weapon-audio bank, all 285 source-addressed faction
tactical voices, selected withdrawal/result/Death Star/trench-run voice
transitions, source-exact mixed-task-force target rejection, production
trench-run launch, and exact strategic loss
persistence now work within their recorded bounds. The strict result remains
0 of 106 accepted tactical cells until every row has authoritative A0 evidence
and passes the complete comparison contract.

## Start here

- Read the [audit overview](index.md) for the current result and definition of
  100% interface parity.
- Follow the current
  [batched execution plan](../../plans/2026-09-11-feat-batched-interface-parity-plan.md)
  for bundle boundaries, verification cadence, browser review, and commit gates.
- Use the
  [Tactical 106-Cell Acceptance Train](../../plans/2026-09-22-tooling-tactical-106-cell-acceptance.md)
  for the final A0 capture, deterministic A1 mapping, comparison, and strict
  `TAC-01` through `TAC-07` completion push.
- Use its
  [acceleration sidecar](../../plans/2026-09-11-tooling-interface-parity-acceleration-sidecar.md)
  for the permanent muted harness, deterministic fixtures, visual comparisons,
  generated summaries, and complete-family GID pilot.
- Choose a bounded surface and state from the
  [machine-readable surface ledger](surface-ledger.json).
- Use the [reverse-engineering ledger](reverse-engineering-ledger.md) to find
  its original resources, executable paths, and named `RE-*` work package.
- Check the [screenshot ledger](screenshot-ledger.md) for available reference
  captures and known evidence gaps.
- Use the [original graphic reference library](../../reference/asset-library/README.md)
  for every decoded entity, staged resource ID, battle-art gap, and Ghidra
  proof route. The [standalone space-battle launcher plan](../../plans/2026-09-12-tooling-standalone-space-battle-launcher.md)
  scopes test-only tactical entry and the `TAC-01` through `TAC-07` gate.
- Use the [space-battle launcher reference](../../reference/space-battle-launcher/README.md)
  for its production route, fixture codes, harness commands, and evidence chain.
- Read the [audit report](audit-report.md) for findings, thresholds, and the
  six-tranche implementation order.
- Review the [P46A strategic shell evidence](evidence/2026-09-11-strategic-shell-canvas.md)
  for the verified canvas, aperture, and browser-transform checkpoint.
- Review the [P46B strategic-control evidence](evidence/2026-09-11-strategic-command-controls.md)
  for the recovered six-control geometry, bitmap, input, and command contract.
- Review the [PR #11 cockpit-routing correction](evidence/2026-09-14-cockpit-routing-correction.md)
  for the corrected `0x131` through `0x133` identities, F1/F7 routing, GID
  open/close behavior, and fail-closed original destinations.
- Review the [UIP-B01 strategic-navigation evidence](evidence/2026-09-11-strategic-window-navigation.md)
  for the sector, system-shell, tab, and rail checkpoint and its remaining
  item-level boundaries.
- Review the [P46D detailed-system evidence](evidence/2026-09-11-detailed-system-tab-items.md)
  for source-mapped item miniatures, bounded scrolling, selection, coarse
  current-intelligence gating, and the remaining object-window boundaries.
- Review the [P47A galaxy-backdrop evidence](evidence/2026-09-11-authored-galaxy-backdrop.md)
  for STRATEGY 902 identity, canvas-origin placement, native/WASM transport,
  browser captures, and the remaining GID boundaries.
- Review the [P47B Popular Support evidence](evidence/2026-09-11-popular-support-gid.md)
  for the active 903 predicate, exact faction GID control, compact legend,
  original marker families, and recovered support thresholds.
- Review the [P48 GID browser-harness evidence](evidence/2026-09-12-gid-browser-harness.md)
  for the code-built menu, isolated fixtures, browser matrix, and remaining
  original-reference and filter-predicate gaps.
- Review the [P49 Message Index rail evidence](evidence/2026-09-12-message-index-rail.md)
  for exact faction resting bitmaps and the still-open index behavior.
- Review the [P60 Game Speed evidence](evidence/p60-game-speed/README.md)
  for the original speed menu, stop-day pause, alert, and remaining gaps.
- Review the [P50 GID menu-frame evidence](evidence/2026-09-12-gid-menu-frame.md)
  for original frame tiles, source-pixel checks, and the remaining menu gaps.
- Review the [P51 GID hover and occlusion evidence](evidence/2026-09-12-gid-hover-and-occlusion.md)
  for removal of the unsupported wash and visible-border checks under windows.
- Review the [P52 tactical shell evidence](evidence/2026-09-12-tactical-shell-controls.md)
  for the first original 640×480 battle composition, native-coordinate pause,
  highlight, and zoom controls, and the remaining 106-cell tactical boundary.
- Review the [P53 tactical control-state evidence](evidence/2026-09-12-tactical-control-hit-states.md)
  for source-shaped hit masks, zoom pressed art, browser round trips, and the
  states that remain deliberately unclaimed.
- Review the [P54 tactical 3D staging evidence](evidence/2026-09-12-tactical-3d-staging.md)
  for the complete raw type-301/type-303 inventory, deterministic extraction,
  source hashes, and the P55 decoding boundary.
- Review the [P55 tactical 3D runtime-pack evidence](evidence/2026-09-12-tactical-3d-runtime-pack.md)
  for complete binary-X and type-303 decoding, palette rules, deterministic
  objects, and the independent Assimp oracle.
- Review the [P56 tactical 3D render proof](evidence/2026-09-13-tactical-3d-render-proof.md)
  for the first exact mesh/texture browser submission, its paired negative
  control, runtime observations, and the original-parity boundaries kept open.
- Review the [P57A tactical LOD-family evidence](evidence/2026-09-13-tactical-3d-lod-family.md)
  for the complete first three-mesh family, source selection predicate,
  two-faction browser matrix, and the view rules still open for P57B.
- Review the [P57B1 live LOD evidence](evidence/2026-09-13-tactical-live-lod-journey.md)
  for the source-traced cached-slot sequence, one-load browser journey, and the
  original camera and A0/A1 boundaries still open for P57B2.
- Review the [P57B2A camera evidence](evidence/2026-09-13-tactical-camera-contract.md)
  for exact faction views, field zoom, clip planes, handedness conversion,
  bitmap D-pad interactions, and the remaining production/A0 boundary.
- Review the [P57B2B1 target evidence](evidence/2026-09-13-tactical-target-control.md)
  for the source-selected object/frame path, normal and held bitmap control,
  fallback centering, and the stable production identity boundary.
- Review the [P57B2B2 layout evidence](evidence/2026-09-13-tactical-battle-layout.md)
  for the active-force extent, docked-fighter exclusion, four tactical lanes,
  muted browser proof, and the remaining production-placement boundary.
- Review the [P57B2B3 participant evidence](evidence/2026-09-13-tactical-participant-placement.md)
  for stable DAT and fleet-roster identity, exact source X slots and faction
  lanes, selected-world-point targeting, and the remaining resource join.
- Review the [P57B2C1 transform and palette evidence](evidence/2026-09-13-tactical-transform-palette.md)
  for direct authored mesh coordinates, the complete system-palette pack,
  exact Abregado/Cathar selection, and the remaining render-state boundary.
- Review the [P57B2C2A light-rig evidence](evidence/2026-09-14-tactical-light-rig.md)
  for the recovered retained-mode directional and ambient lights, exact
  transformed direction, muted browser proof, and the filtering/culling
  boundary.
- Review the [P57B2C2B render-state evidence](evidence/2026-09-14-tactical-render-state.md)
  for filtering, culling, depth, device quality, material fields, muted browser
  proof, and the production-resource/A0 boundary.
- Review the [P58A resource-join evidence](evidence/2026-09-14-tactical-resource-join.md)
  for all ship/fighter DAT joins, complete 87-mesh/397-texture transport,
  representative browser probes, and the production-draw/A0 boundary.
- Review the [P58B production-participant evidence](evidence/2026-09-14-tactical-production-participants.md)
  for exact live capital-family rendering, paired 3D-off proof, the 36-case
  muted browser gate, and the fighter/selection/A0 boundary.
- Review the [P58C fighter and selection evidence](evidence/2026-09-14-tactical-fighters-selection.md)
  for original type-303 fighter resources, exact detail thresholds,
  projection-aligned capital interactions, and the remaining A0 boundary.
- Review the [P58D fighter-detail evidence](evidence/2026-09-14-tactical-fighter-detail-journey.md)
  for the complete two-faction indicator, far, and close journey.
- Review the [P58E group-presentation evidence](evidence/2026-09-14-tactical-group-presentation.md)
  for all eight task-force controls, four RGBY fighter controls, source-shaped
  hit testing, keyboard routes, selected fighter portraits, and A0 limits.
- Review the [P58F1 tactical-planet evidence](evidence/2026-09-14-tactical-planets.md)
  for the system selector chain, exact planet and palette pair, production
  browser submission, provisional placement, and A0 limits.
- Review the [P58F2 tactical-impact evidence](evidence/2026-09-14-tactical-impact-effects.md)
  for the six source-selected frame families, 10 Hz lifecycle, priority,
  transparent target attachment, 48-case browser gate, and A0 limits.
- Review the [P58F3 projectile and field evidence](evidence/2026-09-14-tactical-projectile-fields.md)
  for retained mesh variants, thresholds, material colors, interpolation,
  tractor/gravity animation and priority, 52-case browser gate, and A0 limits.
- Review the [P58F4 selected-damage evidence](evidence/2026-09-15-tactical-selected-damage.md)
  for source-ordinal capital portraits, panel 1302 matte composition, live
  faction-correct shield and hull meters, 56-case browser gate, and A0 limits.
- Review the [P58F5 subsystem and field-command evidence](evidence/2026-09-15-tactical-subsystem-field-commands.md)
  for all five subsystem families, exact quantization and placement, exact
  field-source identity and capacity, 60-case browser gate, and A0 limits.
- Review the [P58F6 live subsystem-damage evidence](evidence/2026-09-15-tactical-live-subsystem-damage.md)
  for source-derived shield overflow, hull loss, subsystem hit selection and
  limits, condition recalculation, tractor cancellation, and the 64-case gate.
- Review the [P58F7 subsystem-repair and mobility evidence](evidence/2026-09-15-tactical-subsystem-repair-mobility.md)
  for source-derived repair cadence and selection, sublight-engine condition,
  tractor drag, complete field containment, and the 68-case gate.
- Review the [P58F8 maneuver and movement evidence](evidence/2026-09-16-tactical-maneuver-movement.md)
  for the recovered maneuver-state bonus, effective-power velocity, signed
  faction movement, 250-millisecond integration, and the 72-case gate.
- Review the [P58F9 tactical command-assignment evidence](evidence/2026-09-17-tactical-command-assignment.md)
  for both original command panels, exact order and tactic codes, source-shaped
  controls, cancel/commit behavior, group delivery, and the 76-case gate.
- Review the [P58F10 tactical order-execution evidence](evidence/2026-09-17-tactical-order-execution.md)
  for recovered maneuver waypoints, Hold stop behavior, Recover carrier
  reservation and Returning state, and the 80-case gate.
- Review the [P58F11 tactical command-progression evidence](evidence/2026-09-18-tactical-command-progression.md)
  for source-rate turning, waypoint completion, fighter docking and recovery,
  preserved squadron counts, and the 84-case gate.
- Review the [P58F12 tactical attack-targeting evidence](evidence/2026-09-18-tactical-attack-targeting.md)
  for typed Attack Fighters and Attack Capital Ships acquisition across both
  owner classes, visible target endpoints, and the 88-case gate.
- Review the [P58F13 tactical attack-target lifecycle evidence](evidence/2026-09-19-tactical-attack-target-lifecycle.md)
  for same-class replacement after capital and fighter targets become
  ineligible, exhausted-list clearing, and the 92-case gate.
- Review the [P58-B06 tactical completion evidence](evidence/2026-09-22-tactical-completion-bundle.md)
  for capital and fighter combat, collision, retained formations, Death Star
  and trench-run routes, original result/options surfaces, the clean 120-case
  browser gate, qualified independent visual review, and remaining A0 boundary.
- Review the [P58-B07 tactical matrix-contract evidence](evidence/2026-09-22-tactical-106-matrix-contract.md)
  for the generated 106-cell denominator, exact family distribution, A0
  provenance schema, exclusion checks, and deliberately unclaimed coverage.
- Review the [P58-B08 tactical A0-ingestion evidence](evidence/2026-09-22-tactical-a0-ingestion.md)
  for the guest provenance extension, fail-closed host registration path, and
  explicit 0/106 capture-host boundary.
- Review the [P58-B09 tactical A1 crosswalk](evidence/2026-09-22-tactical-a1-crosswalk.md)
  for the 82 mapped cells, journey/snapshot distinction, and exact 24-cell
  new-scenario queue.
- Review the [P58-B10 tactical navigation and camera evidence](evidence/2026-09-23-tactical-navigation-camera.md)
  for four navigation sets, ordered routes and targets, camera memory/chase,
  Anvil and Stand Off, and the reduced ten-cell scenario queue.
- Review the [P58-B11 tactical trench-run launch and persistence evidence](evidence/2026-09-23-tactical-trench-persistence.md)
  for production order-6 launch, exact strategic capital/fighter loss
  persistence, two bounded visual follow-ups, and the reduced eight-cell queue.
- Review the [P58-B12 tactical Game Options and empty-space evidence](evidence/2026-09-23-tactical-game-options-empty-space.md)
  for original COMMON `20002` routing, disabled mid-battle display controls,
  empty-space starfield presentation, and the reduced six-cell queue.
- Review the [P58-B13 tactical withdrawal-confirmation evidence](evidence/2026-09-23-tactical-withdraw-confirmation.md)
  for executable-derived panel geometry, exact text/resource identities,
  cancel/confirm routing, and the reduced five-cell queue.
- Review the [P58-B14 tactical detail and Escort evidence](evidence/2026-09-23-tactical-detail-escort.md)
  for destroyed presentation, compact selected-capital contents, source order
  code 1 Escort, direct right-click routing, and the reduced two-cell queue.
- Review the [P58-B15 tactical Battle Alert and audio evidence](evidence/2026-09-24-tactical-battle-alert-audio.md)
  for the source-built pre-battle surface, paused Take Command transition,
  MDATA 307 score, WAVE 13054 event cue, and complete 106-cell A1 crosswalk.
- Review the [P58-B16 tactical weapon-audio evidence](evidence/2026-09-24-tactical-weapon-audio.md)
  for corrected event semantics, all eight fire/impact families, all 22 WAVE
  variants, production dispatch, and focused muted browser routing.
- Review the [P58-B17 tactical command-voice evidence](evidence/2026-09-24-tactical-command-voice.md)
  and [P58-B18 complete tactical voice-bank evidence](evidence/2026-09-25-tactical-complete-voice-bank.md)
  for command acknowledgements, the complete 285-resource source table,
  selected battle-transition dispatch, and focused muted browser proof.
- Use the [evidence index](evidence/README.md) to find each durable report and
  artifact bundle. Every new bundle must include its own `README.md` inventory.

## Suggested contribution lanes

| Lane | Best starting record |
|------|----------------------|
| Original resource extraction or Ghidra analysis | An open `RE-*` package in [reverse-engineering-ledger.md](reverse-engineering-ledger.md) |
| Interface implementation | A required family or cell in [surface-ledger.json](surface-ledger.json) |
| Screenshot and provenance research | A missing state in [screenshot-ledger.md](screenshot-ledger.md) |
| Automated visual verification | The gates and thresholds in [audit-report.md](audit-report.md) |
| Space-battle interface | `TAC-01` through `TAC-07`, covering 106 baseline cells, plus the [ranked Windows/Ghidra recovery map](../../reference/space-battle-launcher/reverse-engineering-map.md) |

## Acceptance rule

A required cell passes only when it has authoritative original evidence, exact
resource identity, matching composition and interaction geometry, native and
packaged-browser captures, clean diagnostics, and correct audio where
applicable. Update the relevant Markdown and JSON records in the same atomic
commit as the verified implementation.

Do not commit copyrighted game assets. Extraction and runtime-pack tooling must
stage them from a contributor-owned installation.

## Progress snapshot

<!-- interface-parity-status:start -->
Required interface families: 43. Complete: 0. Partial: 9. Failing: 34. Blocked: 0. Strictly accepted cells: 0/564.
The current CMD-02 GID catalog has 38 scenarios (29 baseline cells and nine additional native filter variants) and 152 faction/viewport executions. Its strict original-evidence and cross-browser gate remains open.
<!-- interface-parity-status:end -->

The practical implementation estimate is approximately 20 to 25% tackled and
75 to 80% remaining. Formally, the ledger has 43 required families: 0 complete,
9 partial, and 34 failing. All 564 required cells remain pending until their
complete evidence and execution matrices pass. The practical estimate guides
bundle planning; it does not replace strict acceptance.

Within that total, the standalone space-battle launcher is approximately 96%
implemented at the P58-B06 checkpoint. Its formal acceptance remains 0 of 106
tactical cells because exact original-view evidence and the remaining native
beam, playback, and post-battle orchestration contracts are open.

The first recovered implementation tranche now packages and renders the four
original faction-advisor idle runs. See the
[P44 evidence](../2026-09-08-full-functionality-audit/evidence/2026-09-10-authentic-droid-advisors.md).
Full `CMD-07` action, voice, chrome, and shell integration remains open.

P46A now verifies the exact centered 640x480 strategic canvas, faction shell
crop, recovered galaxy apertures, and shared browser transform. See the
[P46A evidence](evidence/2026-09-11-strategic-shell-canvas.md). P46B restores
the six primary faction controls and removes their replacement text strip. See
the [P46B evidence](evidence/2026-09-11-strategic-command-controls.md).
PR #11 corrects `0x131` through `0x133`, F1/F7, and the authentic GID toggle
while keeping unfinished original destinations fail closed. See the
[routing correction](evidence/2026-09-14-cockpit-routing-correction.md).
UIP-B01 now replaces the invented sidebar with first-pass original sector and
system shells and a working 12-slot rail lifecycle for both factions. See the
[navigation evidence](evidence/2026-09-11-strategic-window-navigation.md).
P46D populates all six system tabs with source-mapped GOKRES miniatures,
displayed entity labels, recovered scrollbar art, bounded interaction,
preserved selection, and coarse current-intelligence gating.
See the [detailed-system evidence](evidence/2026-09-11-detailed-system-tab-items.md).
P47A replaces the strategic map's flat fill with the source-aligned bright
STRATEGY 902 galaxy in native and packaged WASM. See the
[galaxy-backdrop evidence](evidence/2026-09-11-authored-galaxy-backdrop.md).
P47B restores the default active Popular Support baseline with STRATEGY 903,
the exact faction GID control, compact legend, and native marker families. It
also withholds the prior synthetic map overlays from this parity view. See the
[Popular Support evidence](evidence/2026-09-11-popular-support-gid.md).
P48 adds the original-command GID menu tree and a test-only muted browser
matrix. The [P48 evidence](evidence/2026-09-12-gid-browser-harness.md) keeps
non-support filter rules, expanded legend, special state overlays, and
original-executable capture acceptance open.
P49 restores the nine original Message Index rail icons per faction. Their
resting pixels match the source BMPs, but the index window and unread states
remain open. See the [rail evidence](evidence/2026-09-12-message-index-rail.md).
P50 restores the eight original GID frame tiles to the root and submenu and
removes the invented fade. Its native-size root border matches the source BMPs
where unobscured. See the [frame evidence](evidence/2026-09-12-gid-menu-frame.md).
P51 removes a provisional hover wash and extends those checks to the visible
frame border beneath a foreground system window. Its
[evidence](evidence/2026-09-12-gid-hover-and-occlusion.md) keeps original hover
and focus states open.
P52 through P55 restore the first tactical shell and controls, stage the full
original 3D corpus, and decode it into a deterministic runtime store. P56
proves one exact mesh and texture pair. P57A extends that path to all three
original LOD meshes, both textures, and the source selection predicate. Each
isolated fixed-view fixture records one initial family-load event. Its
[evidence](evidence/2026-09-13-tactical-3d-lod-family.md) keeps original view
acceptance, production fleet integration, and all 106 tactical cells open.
P57B2A through P57B2B3 recover the source camera, selected-object target path,
active-force extent, four lanes, stable participant identity, exact X slots,
and source-world target point. P57B2C1 removes inferred mesh normalization,
uses authored coordinates with the source handedness boundary, and packages
and selects the original system palette. P57B2C2A replaces the guessed light
with the source directional and ambient rig. P57B2C2B restores filtering,
culling, depth, dither, shading, specular, and diffuse/emissive material state.
P58A replaces approximate sprite arithmetic with all 29 capital-ship, eight
fighter, and Death Star DAT resource joins, and transports all 87 meshes and
397 textures through the four-request browser pack. P58B renders live
production capital ships from their joined families at source positions. P58C
renders source-mapped fighter groups and aligns capital selection and targeting
with the projected meshes. P58D drives both production fighter families through
their independent indicator, far, and close transitions with the original zoom
controls and no family reload. P58E restores all eight task-force controls,
four RGBY fighter controls, group selection, source-shaped hit testing,
F1 through F12 routing, and selected fighter detail panels. P58F1 restores the
exact system-selected planet and paired palette to production battles. P58F2
restores the six source-selected hit, damage, and destruction sprite families,
their 10 Hz lifecycle, priority groups, indexed transparency, and target
attachment. P58F3 restores all retained projectile mesh variants, weapon
thresholds, faction/ion material selectors, source-to-target interpolation,
one- and two-second lifecycles, and both 10 Hz tractor/gravity field families
with gravity priority. P58F4 restores panel 1302, exact source-ordinal capital
portraits, lime-matte composition, and live faction-correct shield and hull
meters. P58F5 restores all five source-quantized subsystem-condition families,
exact tractor and gravity source identities, their source-side capacity limits,
gravity priority, and frame reset. P58F6 connects capital and fighter fire to
the recovered shield-overflow, hull-loss, subsystem-selection, condition, and
tractor-cancellation path. P58F7 restores the 50-second repair cadence,
inclusive damage-control roll, source subsystem order, engine-condition math,
and active tractor drag used by the mobility contract. P58F8 restores the
maneuver-state bonus, effective-power velocity, and millisecond position
integration while preserving the original stationary constructor state.
P58F9 restores the bitmap-driven Maneuvers and Tactics and Missions panels,
exact order and tactic codes, disabled states, and selected-unit commit and
cancel behavior. P58F10 connects the four maneuver orders to recovered waypoint
construction, executes Hold, and starts Recover through carrier reservation and
Returning state 2. P58F11 applies source-rate signed turns, completes reached
waypoints, and advances fighters through Docking and Recovered while preserving
their strategic counts. P58F12 restores typed Attack Fighters and Attack
Capital Ships target acquisition for capital and fighter owners while
preserving valid engagements. P58F13 refreshes invalid attack targets through
a stable same-class list and clears exhausted lists without a cross-class or
random fallback. P58-B06 adds capital and fighter combat, collision, automatic
group distribution, forward-relative retained formations, the separate Death
Star, original result/options panels, both trench-run routes, and exact
strategic roster, officer-capture, and Death Star-state application. Exact
planet placement, global RNG sequencing, original arrival callbacks and
recovery trajectories, power allocation, native beam behavior, shared
post-battle bombardment, landing, and navigation orchestration, native
playback, and A0 comparisons remain open.
`CMD-01`, `CMD-03`, `CMD-04`, `CMD-10`, and `UIP-T01` remain open because the
full control matrix, remaining GID modes and map art, nested object
compositions and commands, exact rail thumbnails, original destination
windows, and replacement surfaces are not complete.
