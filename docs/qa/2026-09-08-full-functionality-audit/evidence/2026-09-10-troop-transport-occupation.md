---
title: "Troop Transport and Occupation Evidence"
description: "F-007E proof for regiment cargo, invasion, occupation, provisional capture, save v13, and browser continuity"
category: qa
created: 2026-09-10
updated: 2026-09-10
tags: [qa, parity, ai, transport, ground-combat, occupation, wasm, astra]
---

# F-007E Troop Transport and Occupation

## Outcome

The conquest path now works from strategic order through campaign state. AI
fleets embark same-faction surface regiments within living ship capacity,
carry them through authoritative transit and fleet consolidation, land after
orbital control is established, resolve ground combat, occupy the system, and
capture enemy characters. Headquarters victory now requires occupation rather
than an enemy fleet merely passing through orbit.

Source review correction: the transport, landing, combat, and political-control
path remains verified, but the Imperial headquarters and character-capture
semantics do not. The original contract requires destruction of the mobile
Alliance HQ, not occupation alone. Current occupation also captures every
living enemy character at the system without an evasion or capture-resolution
step. These two items are reopened as parity work; this evidence must not be
read as accepting them.

This closes the missing transport and occupation root cause in F-007. M1
remains open for faction liveness, battle diagnostics, correct HQ and Death
Star victory semantics, capture/evasion, the wider campaign loop, and five-seed
native/WASM campaign equivalence. Player troop dispatch was verified in the
following checkpoint.

## Implemented contract

- `TroopTransportState` owns deterministic fleet-to-regiment cargo and is part
  of save format v13 and the state fingerprint.
- Embarkation validates the complete request atomically: fleet, system,
  faction, surface location, duplicate state, and living transport capacity.
- Compatible fleet consolidation transfers cargo to the surviving fleet key.
- Destroyed fleets lose all cargo. A surviving escort fleet loses cargo above
  the capacity of its remaining living transport hulls in stable key order.
- Cargo lands only when its faction is the sole orbital faction.
- Unresolved surface combat continues on later ticks after cargo is empty.
- Automatic and tactical result application persist regiment damage and remove
  destroyed units from both the troop arena and the system roster.
- Occupation changes political control.
- Current occupation deterministically captures all living enemy characters at
  that system. This is verified implementation behavior but remains a parity
  placeholder pending capture/evasion fixtures.
- The current victory checker reads political control. Orbital supremacy alone
  cannot win, but treating an occupied Alliance-HQ system as a destroyed mobile
  headquarters is a known parity defect.

## Five-seed campaign probe

Each dual-AI run completed 5,000 ticks without a panic or missing
`TroopClassDef` warning.

| Seed | Attacks | Reinforce | Arrivals | Troops moved | Space | Ground | Captures | A / E / N systems | Victory |
|------|--------:|----------:|---------:|-------------:|------:|-------:|---------:|------------------:|:-------:|
| 7 | 30 | 5 | 35 | 9 | 5 | 7 | 5 | 1 / 14 / 185 | No |
| 42 | 38 | 2 | 40 | 11 | 6 | 7 | 6 | 0 / 11 / 189 | No |
| 99 | 29 | 6 | 35 | 8 | 4 | 6 | 6 | 0 / 12 / 188 | No |
| 1337 | 35 | 3 | 38 | 15 | 10 | 9 | 6 | 0 / 15 / 185 | No |
| 424242 | 27 | 3 | 30 | 8 | 7 | 6 | 6 | 1 / 10 / 189 | No |

The campaigns now produce transport, landing, ground-combat, control-change,
and capture events. Seed 42 occupied Mon Calamari at Smarteel and captured
Luke, Han, Wedge, Chewbacca, and Dodonna at Yavin. Its randomized Rebel
headquarters was elsewhere, so Standard victory correctly remained unresolved.
The missing transport and political-conquest mechanic is closed. Campaign
noncompletion remains multi-causal: victory semantics, capture/evasion,
targeting, faction liveness, production, diplomacy, intelligence, research,
uprisings, bombardment, headquarters relocation, and other campaign behaviors
still require their own acceptance passes.

## Automated verification

| Gate | Result |
|------|--------|
| Workspace | 567 passed, 0 failed, 4 ignored |
| Doc tests | 3 passed, 0 failed, 16 ignored |
| Troop-transport regressions | 8 passed |
| Tactical survivor persistence | 1 passed |
| Later-tick empty-transport surface retry | 1 passed |
| Original-data replay manifest | 2 passed, 0 failed |
| Native/WASM replay | 9 of 9 checkpoints matched |
| Save format | v13 round-trip and v12 migration passed |
| Packaged WASM | Passed with existing warnings |

The reviewed 13,482-byte seed-42 artifact starts at
`v1:b38248eb039a8032` and ends at `v1:cde64607b027b1d1`. Its runtime-data
fingerprint is `5facb1c7ba0e81ad` over 51 simulation inputs and 50,597 bytes.

## Astra medium browser acceptance

GPT-6 Astra medium used the codex-orchestrator in read-only mode. Chrome was
launched with `--mute-audio`; the product music control remained enabled by
default. Astra reported no P0 or P1 blocker.

| Browser check | Result |
|---------------|--------|
| Original bitmap menu | Visibly nonblank |
| Alliance and Empire paths | Galaxy and fleet bitmaps visible |
| Galaxy zoom | Visible scale change |
| Save, reload, load | v13 keys; `v1:b7a69b50bb16a977`; `verified=true` |
| Exact replay | Working-tree artifact echoed; 9 of 9 checkpoints matched native |
| Main navigation requests | 16 of 16 HTTP 200; four deterministic requests per navigation |
| Separate startup smoke | 4 of 4 HTTP 200 |
| Console, page, WASM, request, missing-asset errors | 0 |

Four WebGL `ReadPixels` performance warnings were observed. They are not
functional or asset errors and remain part of the browser-performance backlog.
Astra then passed the dedicated two-tick surface retry test and confirmed that
its earlier evidence gap was closed.

## Exact browser artifacts

| Artifact | Bytes | SHA-256 |
|----------|------:|--------|
| `web/open-rebellion.wasm` | 5,089,398 | `e83665b4481197b9e5ba1899d727c3df58d35afbe6082f9645029f259cbf6535` |
| `web/data/runtime.orpk` | 29,096,058 | `6123deec14cfbfd070573c7001ef1123bd52067273498ad0c36cb7fdb108f509` |

The runtime pack contains 52 game files, 2,231 bitmaps, and 5 audio files. The
served bytes matched the local files.

## Remaining M1 work

- Preserve the separately verified player-facing troop selection and
  embark/disembark command path.
- Correct faction-specific headquarters destruction, Standard conjunctions,
  Death Star outcomes, and the uncited 200-tick victory grace period.
- Replace deterministic mass capture with validated capture/evasion behavior.
- Prove faction liveness and territorial recovery without treating symmetric
  attack counts as an original-game requirement.
- Record battle volume and geographic distribution as diagnostics until
  original telemetry or calibrated playtests justify release bounds.
- Exercise the wider campaign loop: diplomacy, recruitment, intelligence,
  research, production, uprisings, bombardment, HQ relocation, and principal
  missions.
- Extend native/WASM checkpoint equivalence across all five 5,000-tick seeds.

The governing contract is the
[campaign-history reference](../../../reference/campaign-history/official-campaign-contract.md#standard-victory).
