---
title: "F-007E AI Campaign Logistics Checkpoint"
description: "Ownership, targeting, blockade, troop-data, repair-transition, save-v12, and five-seed evidence"
category: qa-evidence
created: 2026-09-10
updated: 2026-09-10
tags: [ai, campaign, combat, repair, save-load, troops, parity]
---

# F-007E AI Campaign Logistics Checkpoint

F-007E remains in progress. This checkpoint closes the fleet-arena, transit,
repair-start, and missing troop-class defects exposed by F-007D. It does not
claim campaign-balance or victory acceptance.

## Corrections

- AI production now requires political control of the manufacturing system.
  Seeded facilities on neutral or enemy worlds can no longer create fleets for
  the wrong faction.
- Military target validation no longer treats low local popular support as a
  veto or turns `FUN_0050b8e0` readiness scoring into a global strength gate.
  Per-fleet force allocation still rejects defenders above a 3× bound.
- Target candidates include hostile fleets on politically neutral systems.
  Fleets exclude their current location, do not fall back to friendly cycling,
  and hold an established blockade instead of returning through friendly
  systems.
- Each active headquarters retains a stable defender while assembled waves may
  launch once production grows beyond that reserve.
- Yavin is seeded as the Alliance base, not a second headquarters. The selected
  Rim system remains the sole Alliance HQ.
- Starting a blockade no longer destroys surface garrisons. Event `0x340`
  remains reserved for a future troop that is actually running a blockade.
- `TROOPSD.DAT` sequential IDs are normalized with family `0x10`, matching the
  compound IDs stored on deployed regiments. Ground combat now uses original
  attack/defense data and emits no missing-class fallback warnings in the
  five-seed run.
- `RepairCheckPerformed` now fires only when a fleet enters a repair episode.
  Hull restoration remains per tick. Save v12 persists active repair episodes,
  and the exact v11 body migrates with an empty episode set.

## Five-seed result

All runs used the original 51-DAT set, dual AI, and 5,000 ticks.

| Seed | Initial/final fleets | Transit | Moves/arrivals | Attacks A/E | Space + ground | Systems | Top share | Repair starts/steps | Victory |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 7 | 7 / 6 | 0 | 97 / 97 | 3 / 88 | 5 + 3 | 3 | 62.5% | 2 / 158 | No |
| 42 | 5 / 6 | 0 | 60 / 60 | 2 / 52 | 6 + 3 | 4 | 33.3% | 2 / 187 | No |
| 99 | 8 / 4 | 0 | 73 / 73 | 4 / 61 | 6 + 5 | 3 | 63.6% | 3 / 187 | No |
| 1,337 | 7 / 5 | 0 | 76 / 76 | 4 / 70 | 8 + 3 | 3 | 45.5% | 0 / 0 | No |
| 424,242 | 8 / 6 | 0 | 70 / 70 | 3 / 61 | 7 + 7 | 4 | 42.9% | 5 / 882 | No |

The five runs pass the M1 fleet-arena maximum (1.2× observed, 3× limit), final
transit maximum (0%, 10% limit), and move/arrival maximum (1.0×, 1.5× limit).
Repair starts fell from as many as 192,902 per run at the F-007D checkpoint to
0–5 while real hull work continues.

The balance gate remains red. Only 8–14 combined battles occur rather than
50–400, only three or four systems host them rather than at least eight, three
runs exceed the 40% concentration limit, and Alliance attack volume remains
far below Empire volume. No run reaches victory.

## Victory interpretation

Zero controlled systems is not sufficient for Standard victory. Standard mode
requires the qualifying headquarters and both principal enemy leaders to be
captured. Headquarters Only mode requires the qualifying HQ alone. The current
AI cannot transport surface regiments with fleets or complete an occupation and
principal-leader capture sequence, so the observed no-victory result is a real
campaign-completion gap, not evidence that the victory checker ignored a valid
Standard state.

## Verification

- Focused repair tests: 7 passed.
- Save tests: 23 passed, including current v12 round-trip and v11 migration.
- Original troop-class integration: 1 passed.
- Original-data replay manifest: 2 passed after review of the unchanged command
  stream and the intentional save-v12/HQ fingerprint update.
- Five native campaigns: completed without a missing `TroopClassDef` warning.
- Full workspace: 551 passed, 0 failed, 4 ignored; doc tests: 3 passed, 16
  ignored.
- WASM build: passed with pre-existing warnings. `open-rebellion.wasm` is
  5,027,605 bytes with SHA-256
  `5d13d0f1e8cf22b61d3d9866b3ce89cb17ba2abda837979bd49a1f2bb0d3ed95`.
- Exact native/WASM replay: 9/9 checkpoints, final fingerprint
  `v1:5871d04619e30abd`, four browser requests, and zero runtime or request
  errors. The reviewed artifact is 13,482 bytes with SHA-256
  `a302d0763bbf1c258001d1367ff9f1f1a80d19b4ce454700cebcf1aa6629a8b2`.
- Runtime pack: 52 game-data files, 2,231 UI bitmaps, and 5 audio files in
  29,096,058 bytes with SHA-256
  `6123deec14cfbfd070573c7001ef1123bd52067273498ad0c36cb7fdb108f509`.
- Astra medium: PASS for muted browser-test output, enabled-by-default product
  music, both visible audio states, the exact 30x22 painted click boundary,
  both-faction bitmap/system/fleet/zoom flows, save-v12 reload with verified
  fingerprint `v1:2b0fbda682f8ae09`, replay 9/9, all HTTP 200 responses, and
  zero console warnings/errors, page exceptions, request failures, WASM
  errors, or missing-asset reports.

## Next boundary

The next F-007E slice must model troop embarkation, capacity, arrival, ground
assault, occupation, and principal-leader capture through authoritative
commands and events. AI force allocation can then be balanced against actual
conquest progress instead of manufacturing extra fleet motion merely to reach a
battle-count target.
