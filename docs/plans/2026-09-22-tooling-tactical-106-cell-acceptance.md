---
title: "Tactical 106-Cell Acceptance Train"
description: "Final local commit train for a runnable and honestly accepted TAC-01 through TAC-07 parity matrix"
category: plan
created: 2026-09-22
updated: 2026-09-28
tags: [interface, parity, tactical, browser, evidence, testing]
status: in_progress
---

# Tactical 106-Cell Acceptance Train

This is the final acceptance companion to the
[Standalone Space-Battle Test Launcher](2026-09-12-tooling-standalone-space-battle-launcher.md).
It scopes the local commit set that makes every `TAC-01` through `TAC-07`
acceptance cell executable, compares the reconstruction with authoritative
original-game evidence, closes remaining production behavior, and produces one
auditable 106-cell verdict before the commits are pushed together.

The 640×480 original composition remains authoritative. Enhanced widescreen is
a later modernization track and is not part of this denominator.

## Current boundary

- P58-B06 passes a 120-case deterministic implementation matrix across both
  factions and two browser viewports.
- Independent A1 review found no P0 or P1 defect and left three bounded review
  findings.
- P58-B21 shares played and automatic post-battle bombardment, landing,
  contested ground continuation, unopposed occupation, and result destination
  routing. Focused tests and four muted browser cases pass.
- P58-B22 closes the bounded practical launcher implementation with source-
  compatible tactical RNG sequencing, persisted power allocation, exact
  completion/destruction callbacks, and the timed trench-run producer. This
  advances implementation, not strict A0 acceptance.
- The strict matrix contains exactly 106 pending cells: 13 in `TAC-01`, 13 in
  `TAC-02`, 20 in `TAC-03`, 14 in `TAC-04`, 23 in `TAC-05`, nine in `TAC-06`,
  and 14 in `TAC-07`.
- The original English executable reaches its main menu in the existing A0
  Windows VM, but the 106-cell lossless tactical capture set does not yet
  exist.
- Original binaries, extracted resources, saves, screenshots, audio, and video
  remain local and ignored. Only metadata, tooling, hashes, and permissible
  evidence summaries enter Git.

## Two numbers, never conflated

`matrix_coverage` is the number of ledger cells with a unique catalog row and
an executable deterministic journey or snapshot. The runner may report
`106/106 runnable` even when cells fail.

`parity_acceptance` is the number of cells whose complete evidence contract
passes. The project may report `106/106 accepted` only when every row has:

1. authoritative original evidence and provenance;
2. exact resource identity where a shipped resource is involved;
3. matching 640×480 composition within the audit thresholds;
4. exact interaction geometry and real action semantics where applicable;
5. native and packaged-WASM behavioral agreement;
6. correct audio or media routing where applicable; and
7. clean console, network, missing-asset, process-cleanup, and state-fingerprint
   results.

Missing evidence is `reference-needed`, not a pass. A fixture-created snapshot
can prove presentation but cannot prove the action that normally reaches it.

## Canonical artifacts

Tracked artifacts:

- `2026-10-06-surface-ledger.json` owns the 106 stable cell IDs and acceptance status.
- `tactical.catalog.json` owns deterministic A1 scenarios and exact `audit_cells`
  mappings.
- `tactical-a0-manifest.schema.json` validates original-capture metadata.
- `tactical-a0-manifest.example.json` documents the local manifest format
  without proprietary paths or media.
- Generated evidence JSON records coverage, results, thresholds, hashes, and
  redacted artifact identifiers.

Ignored local artifacts:

- `a0/<cell-id>/original.png` or the applicable lossless media capture;
- `a1/<cell-id>/reconstruction.png`;
- pixel, edge, geometry, interaction, audio, and state traces;
- original saves and any editor-assisted setup files; and
- full browser logs containing proprietary asset names or paths.

Each A0 record includes the cell ID, original executable SHA-256, installation
identity, language, VM image/version, Windows display settings, unscaled client
rectangle, capture method, fixture/save provenance, ordered input trace,
timestamp, and artifact SHA-256. The captured game client must be exactly
640×480 with no host scaling, filtering, window chrome, or recompression.

## Local commit set

The commits remain local until the final push gate. Each commit is independently
reviewable and must leave all existing focused tests green.

### C1. Define and validate the 106-cell matrix

Status: complete at P58-B07. The generator reports the current honest boundary
as 0 mapped, 0 A0, and 0 accepted while preserving all 106 stable rows.

- Generate the tactical matrix directly from `2026-10-06-surface-ledger.json`.
- Fail on missing, duplicate, unknown, or non-counting cell IDs.
- Require exactly 106 unique rows distributed 13/13/20/14/23/9/14.
- Add A0 manifest schema validation and redacted evidence paths.
- Add unit tests for denominator drift, duplicate mappings, missing provenance,
  and accidental proprietary-file staging.

Gate: the empty matrix can be generated and must report `0/106 accepted,
106 reference-needed`; it cannot report success merely because the runner
started.

### C2. Capture and ingest authoritative A0 evidence

Status: ingestion infrastructure is complete at P58-B08. The guest capture
script now emits executable identity and per-cell provenance, and the host
ingester fails closed on dimensions, hashes, source identity, ledger drift,
unsafe destinations, or replacement bytes. The owned capture host was offline
on 2026-09-22, so capture coverage remains 0/106 and this phase is not complete.

2026-10-11: the capture VM is the A0 route, with the game full screen.
GOG's DDrawCompat ini leaves `DisplayResolution` at its default, `desktop`,
which scales the 640×480 mode to the guest's 800×600 desktop (DDrawCompat
wiki, Configuration), the only mode its `virtio-ramfb` display lists. `-w`
gives an unscaled window, but it renders differently (32-bit colour,
anti-aliased text, other 3D shading), so windowed frames are not A0 and the
three registered that way were withdrawn. The owner chose to keep 800×600.
With `DisplayFilter=point` the scaling only repeats whole pixels in a fixed
pattern (160 columns, 120 rows), so `capture.ps1` checks every repeated line
equals its neighbour and drops it, recovering the game's 640×480 frame
bit for bit, and keeps the 800×600 source beside it. `vm-game.sh up-a0`
starts the game with the capture watcher, and `vm-game.sh a0` takes each
capture (`tools/original-capture/README.md`, "A0 captures"). A Wine prefix
on the development Mac was set up and dropped the same day.

2026-10-12: 75 of the 106 cells have an A0 frame in the ignored manifest
(`.artifacts/interface-parity/a0/manifest.json`); none is compared or
accepted yet. Damage, subsystem and results states come from edited copies
of our day-30 save (`agent_docs/original-save-editing.md`, recipe 3). A
cell about motion (running, observing) takes a burst of frames at a fixed
cadence instead of one hash-stable frame; the ingester stores the frames and
their offsets beside the cell's frame, which is the burst's last. Three
captures of TAC-01-C004 that showed the results screen were withdrawn. The
31 left. Plan for them (2026-10-12), cheapest first, with what the first
75 taught:

- A ship at 10 hull has dark weapon icons and neither fires nor manoeuvres,
  and the day-30 Alliance has one weak Bulk Cruiser, so it hurts nothing:
  our own ship cannot be made to die there. Deaths and shield damage come
  from the enemy taking the ISD's fire.
- With its hyperdrives intact a hurt cruiser withdraws; with them out the
  battle runs until it dies (`stranded`). The battle ends the moment the
  last enemy dies, so a still of a death is taken by pausing as soon as a
  watched region changes (`vm-game.sh wait X,Y,W,H`), and a burst must end
  before the results screen.
- `FUN_005ae460` fills an empty fighter colour with half of the largest
  group, and only when that group holds more than seven squadrons. Fleet 1's
  six TIE squadrons therefore stay Red: Blue needs eight or more squadrons
  on one side and Gold about sixteen.

Session 1, the unedited day-30 save (`empire-day30-yaga-alert-control`):

| Cell | Route | Capture |
|---|---|---|
| TAC-02-C011 selected | click task force 1 | still |
| TAC-03-C014, C015 previous, next | Ctrl-select the ISD and the Carrack, click the blank task-force slot to make one two-ship task force (manual p. 144), then the status panel's arrows | still each, with before |
| TAC-04-C014 chase camera | unpause, F1 follows task force 1 (manual p. 148) | burst |
| TAC-05-C004 hold | Maneuvers and Tactics, Hold Position (game 556,171), commit (`ghidra/notes/tactical-command-assignment.md`) | still |
| TAC-05-C008 attack fighters | Missions, Attack Fighters, commit | still |
| TAC-07-C013 go to system | Battle Results, Go Directly To…, System (manual p. 152); TAC-07-C012 took the Fleet branch | still |
| TAC-07-C014 strategic persistence | finish the `damage` battle, open Fleet 1's ISD Status, save, reload, read it again | still |
| TAC-01-C007 empty space | find the Show Planet control (manual pp. 141–146), turn the planet off | still |

Session 2, edited saves, the enemy as the casualty:

| Cell | Route | Capture |
|---|---|---|
| TAC-03-C006 shield | `stranded`: left-click the Bulk Cruiser, order the ISD to attack it, pause when its shield bar changes | still |
| TAC-03-C017 target lost | `fragile`: ISD attacking the cruiser; pause as its portrait or orders line changes | still, else a short burst |
| TAC-03-C004 destroyed | the same battle, the cruiser selected | still, else a short burst |
| TAC-04-C003 multiple targets | Ctrl-right-click a second enemy (manual p. 145); the day-30 Alliance has one capital, so its fighters or session 3 | still |

Session 3, an Alliance battle: play `alliance-day54-advice.sav` forward,
send a fleet against an Imperial system, and save at the alert as a new
library save (`saves/INDEX.md`). It gives TAC-01-C001, TAC-02-C013,
TAC-05-C016 and the Alliance side of every both-factions gate, and with
two or more capitals a side TAC-01-C009 (multi-ship) and TAC-04-C003. The
audio cell TAC-01-C012 records the guest's sound through the QEMU monitor
(`wavcapture`, `vm-game.sh mon`) during an engagement in this or session 1.

Session 4, long play, one save each side: a battle with eight capitals in
one fleet (TAC-02-C001, task forces 1–8) and sixteen or more fighter
squadrons on one side, fleet and system garrison together (TAC-02-C003–C005
blue, green, gold). Records cannot be inserted, so these are built in play
and saved before the alert. TAC-02-C007 (unassigned) is settled from
`FUN_005ae460` and the manual before it is staged; it may be a ship outside
every task force or a garrison squadron joining late.

Session 5, the Death Star: route 1 of recipe 2. Order a Death Star in an
Empire save, diff saves a cycle apart for the manager's progress, set it one
short of cost, and let the original finish it; then bring it to a battle
the Alliance can attack with fighters (laser ready, loading, fired, the
charge gauge, Attack Death Star enabled and the trench-run launch:
TAC-03-C020, TAC-05-C009, C017–C021, C023; `ghidra/notes/tactical-death-star-path.md`).
Route 2's converted ISD crashes when its fleet moves, so it serves only a
battle where the Death Star already sits.

- Automate the owned original English installation in the existing Windows VM.
- Use original saves, normal inputs, and, where necessary, editor-assisted save
  setup only to reach states. The displayed frame must always come from the
  unmodified original executable.
- Capture held, selected, disabled, keyboard, edge-probe, camera, damage,
  command, Death Star, options, results, return, audio, and media states named by
  the 106 ledger cells.
- Normalize only by lossless client-area crop, or by dropping the verified
  repeated lines of the point-filtered 800×600 frame (owner's decision,
  2026-10-11, confirmed 2026-10-12; above). Never resize, sharpen, recolor, interpolate, or
  otherwise reconstruct an A0 image.
- Hash every artifact and validate complete provenance before comparison.

Gate: all 106 rows have an authoritative source. Any state that cannot be
captured remains `reference-needed` and blocks the completion push.

### C3. Close `TAC-01` through `TAC-05` production gaps

- Correct exact planet placement, tactical-camera framing, global RNG order,
  original arrival callbacks, recovery trajectories, and power allocation.
- Complete Battle Alert entry, faction music/voice routing, and return-to-map
  behavior through the shared production flow.
- Recover and implement the native Death Star beam contract and validate the
  Alliance Death Star framing finding.
- Fill any control, hover, pressed, disabled, keyboard, edge-probe, navigation,
  maneuver, tactic, mission, target, damage, and destroyed-state gaps exposed by
  A0 comparison.

Gate: native tests and packaged-WASM journeys agree on state and action traces;
no fixture-only mutation is credited as functioning behavior.

### C4. Close `TAC-06` and `TAC-07` outcomes

- Restore the authentic Game Options destination used from Battle Options.
- Complete withdraw, simulate, observe/take-command, close, and confirmation
  paths with their native disabled and repeat-rejection behavior.
- Preserve the P58-B21 shared bombardment, landing, ground follow-up,
  occupation, and destination route while completing exact reports,
  exceptional navigation, and save-state fingerprints for every outcome.
- Prove successful and failed trench-run playback, skip/natural completion,
  return routing, persistent Death Star outcome, and victory separation.
- Complete Battle Results overview, both force tabs, damaged/destroyed rows,
  and Go to Fleet/Go to System destinations for every required outcome.

Gate: both factions, victory/loss/draw/withdraw, partial and total losses,
Death Star outcomes, and return destinations preserve exact strategic state.

### C5. Map deterministic A1 journeys to all 106 cells

Status: deterministic A1 mapping complete at P58-B15. The 36 scenarios map all
106 cells: 87 to browser journeys and 19 to deterministic snapshots. A mapped
cell is runnable, not accepted; A0 coverage, comparison, and strict acceptance
remain 0/106, and the strict gate has not been run.

P58-B16 deepens the mapped audio journey without changing the denominator: it
corrects event `0x14` to torpedo impact, routes all eight recovered weapon
events and all 22 WAVE variants, and passes the focused four-case muted browser
gate. Exact original RNG sequencing and audible native comparison remain C6
evidence work.

P58-B19 deepens the existing production group journey without changing the
denominator. It selects two capitals from different task forces, proves the
source rejection leaves their orders unchanged, and verifies exact muted
Alliance and Imperial voice routes. Strict A0 acceptance remains 0/106.

P58-B20 deepens the command-assignment, command-progression, and withdrawal
journeys without changing the denominator. It verifies exact faction/group
capacity and completion feedback, final-group completion gating, and the
disabled-hyperdrive withdrawal warning in twelve fresh muted browser cases.
Strict A0 acceptance remains 0/106.

P58-B21 establishes the common production return route without changing the
denominator. Played and automatic battles now share bombardment, troop landing,
contested ground continuation, unopposed occupation, and Battle Results
destination routing. Strict A0 acceptance remains 0/106.

P58-B22 closes the bounded practical implementation without changing the
denominator. It adds the recovered tactical random stream, power nibbles,
completion/destruction callbacks, and timed trench-run outcome/chatter path.
Whole-process RNG continuity, strategic commander binding, native visual and
audible comparison, and all strict A0 verdicts remain C6 evidence work.

- Give every catalog scenario one or more exact `audit_cells` values.
- Distinguish journeys from snapshots in the schema and report.
- Drive real pointer and keyboard input for every interactive row, including
  four one-pixel inside edges, four one-pixel outside edges, and corners where
  required by the audit contract.
- Capture canonical 640×480 output. Keep 1280×800 letterboxed runs as regression
  coverage rather than additional parity cells.
- Record native/WASM fingerprints, resource hits, action traces, screenshots,
  audio/media events, browser requests, diagnostics, and cleanup.

Gate: the runner reports `matrix_coverage: 106/106`; every row can execute
independently from a clean, muted browser process, except scenarios whose
purpose is explicit audio verification.

### C6. Compare, classify, and render evidence

- Compare each A1 capture with its A0 counterpart using the audit's existing
  SSIM, channel-delta, displacement, unauthorized-pixel, edge-IoU, and geometry
  thresholds.
- Use narrow, named masks only for proven nondeterministic original regions.
  Store every mask justification and never mask an authentic control, label,
  ship, effect, or outcome.
- Produce per-cell JSON plus an aggregate human-readable report with
  `pass`, `fail`, `reference-needed`, and `not-run` states.
- Generate the repeated Markdown status summaries from the canonical JSON.

Gate: deliberate one-pixel geometry, wrong-bitmap, wrong-action, missing-audio,
and stale-state negative controls all fail the correct row.

### C7. Final acceptance and documentation

- Run the complete workspace suite, scoped lint/format gates, production and
  fixture WASM builds, production-fixture exclusion, and all 106 cells.
- Run both factions and required outcome variants with clean four-request
  startup and complete process cleanup.
- Have an independent browser-use reviewer inspect the aggregate evidence and
  selected full-resolution journeys.
- Update `progress.json`, both audits, the surface ledger, this plan, the
  launcher reference, evidence indexes, and roadmap from the same canonical
  results.
- Mark only passing cells accepted. Do not round, extrapolate, or convert
  implementation coverage into parity acceptance.

Gate: `matrix_coverage: 106/106`, `parity_acceptance: 106/106`, no P0/P1
findings, no missing authoritative evidence, and no open tactical release gate.

## Final command contract

The implementation will expose one documented orchestration command that:

1. validates the local A0 manifest and exact 106-cell denominator;
2. builds production and fixture artifacts;
3. runs native functional/state gates;
4. launches one fresh muted browser per A1 scenario;
5. captures and compares A0/A1 evidence;
6. closes every browser and local server;
7. emits a redacted aggregate JSON and Markdown report; and
8. exits nonzero unless the requested gate passes.

It will support a coverage mode for development and a strict mode for release.
Strict mode requires both `106/106 runnable` and `106/106 accepted`.

## Push rule

The next tactical push may contain C1 through C7 as separate local commits.
Push them together only after the strict command passes, documentation matches
the generated result, the independent review has no P0/P1 finding, and
`HEAD` contains no original-game media or fixture path in production artifacts.

If any original state cannot be captured or any parity row fails, retain the
local commits, leave that cell open, and report the exact blocker. Do not call
the launcher 100% complete and do not push a fabricated 106/106 result.

## Definition of done

- The exact denominator is generated from the authoritative ledger.
- Every one of the 106 rows is independently runnable and reproducible.
- Every cell has authoritative original evidence with verified provenance.
- All visual, geometry, interaction, functional, audio/media, and diagnostic
  gates pass.
- Campaign and standalone entry use the same production renderer and return
  path.
- Test-only fixtures are absent from production HTML and WASM.
- No proprietary original-game artifact is committed.
- The audits and roadmap report 106/106 from generated evidence.
- The complete local commit train is pushed to `main` as one verified batch.
