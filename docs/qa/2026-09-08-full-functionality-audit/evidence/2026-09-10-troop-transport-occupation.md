---
title: "Troop Transport and Occupation Evidence"
description: "F-007E proof for regiment cargo, invasion, occupation, capture, victory semantics, save v13, and browser continuity"
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

This closes the missing transport and occupation root cause in F-007. M1
remains open for player-directed troop assignment, faction balance, battle
volume and distribution, all required Standard victory targets, and five-seed
native/WASM campaign equivalence.

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
- Occupation changes political control and captures living enemy characters at
  that system.
- Headquarters capture reads political control, so orbital supremacy alone
  cannot win and an occupied headquarters remains captured after the fleet
  departs.

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
The remaining no-victory result is now a targeting and balance problem rather
than an absent conquest mechanic.

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

- Add a player-facing troop selection and embark/disembark command path.
- Balance Alliance and Empire production, attacks, and territorial recovery.
- Raise campaign battle volume and geographic distribution to the recorded M1
  bounds without reintroducing fleet churn.
- Make the AI acquire every Standard victory target and exercise permitted
  Death Star victories.
- Extend native/WASM checkpoint equivalence across all five 5,000-tick seeds.
