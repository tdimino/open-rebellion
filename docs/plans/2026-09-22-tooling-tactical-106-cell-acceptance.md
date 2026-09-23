---
title: "Tactical 106-Cell Acceptance Train"
description: "Final local commit train for a runnable and honestly accepted TAC-01 through TAC-07 parity matrix"
category: plan
created: 2026-09-22
updated: 2026-09-23
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

- `surface-ledger.json` owns the 106 stable cell IDs and acceptance status.
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

- Generate the tactical matrix directly from `surface-ledger.json`.
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

- Automate the owned original English installation in the existing Windows VM.
- Use original saves, normal inputs, and, where necessary, editor-assisted save
  setup only to reach states. The displayed frame must always come from the
  unmodified original executable.
- Capture held, selected, disabled, keyboard, edge-probe, camera, damage,
  command, Death Star, options, results, return, audio, and media states named by
  the 106 ledger cells.
- Normalize only by lossless client-area crop. Never resize, sharpen, recolor,
  interpolate, or reconstruct an A0 image.
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
- Share bombardment, landing, reports, final navigation, and save-state updates
  between campaign and direct-launch return paths.
- Prove successful and failed trench-run playback, skip/natural completion,
  return routing, persistent Death Star outcome, and victory separation.
- Complete Battle Results overview, both force tabs, damaged/destroyed rows,
  and Go to Fleet/Go to System destinations for every required outcome.

Gate: both factions, victory/loss/draw/withdraw, partial and total losses,
Death Star outcomes, and return destinations preserve exact strategic state.

### C5. Map deterministic A1 journeys to all 106 cells

Status: partial at P58-B12. The 33 existing scenarios now map 100 cells: 81 to
browser journeys and 19 to deterministic snapshots. Six cells have an explicit
new-scenario queue. A mapped cell is runnable, not accepted; A0 and comparison
gates remain separate.

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
