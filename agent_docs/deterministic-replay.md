---
title: "Deterministic Replay Contract"
description: "Versioned command streams, simulation-data identity, checkpoints, and remaining cross-runtime gates"
category: agent-docs
created: 2026-09-09
updated: 2026-10-16
tags: [determinism, replay, wasm, multiplayer, testing]
---

# Deterministic Replay Contract

`rebellion-data/src/replay.rs` owns the portable replay artifact. It is the
boundary that native, browser, test, and eventual authoritative multiplayer
runners must share. The contract records the inputs needed to explain a run;
it does not yet make the app and playtest simulation loops equivalent.

## Current status

| Capability | Status | Evidence |
|---|---|---|
| Canonical save-state fingerprint | F-011A complete | Save fingerprint v1 |
| Exact post-load continuation state | F-011B1 complete | Save v10+ continuation envelope |
| Versioned replay JSON envelope | F-011B2 complete | Format v1 with strict decoding |
| Canonical `.DAT` identity | F-011B2 complete | Per-file and aggregate fingerprints |
| Gap-free command ordering | F-011B2 complete | `{tick, sequence, actor}` validation |
| Replay executor and recorder | F-011B3 complete | Per-command state checkpoints |
| Fresh-process native replay | F-011B3 complete | Stable 25-tick original-data golden |
| App/playtest tick convergence | Open | F-012 and M2 |
| Native/WASM checkpoint equality | F-011B4 complete | Exact seed-42 artifact, nine checkpoints |
| Automatic/tactical combat equality | Open | M2 |

## Format v1

`ReplayManifest` contains:

- a format and engine version;
- the deterministic seed, RNG identity, and fixed roll budget;
- every simulation `.DAT` input in canonical filename order;
- the active `GameConfig` fingerprint;
- the initial state fingerprint;
- ordered `ReplayCommandRecord` values; and
- expected `ReplayCheckpoint` values.

Each command has the simulation tick immediately before application, a
zero-based global sequence, an actor, and a typed command. Format v1 supports
the current headless control commands. Existing command semantics may not
change without incrementing `REPLAY_FORMAT_VERSION`.

Each checkpoint identifies a tick, the exact number of commands already
applied, and the expected versioned state fingerprint. This disambiguates
multiple commands at one tick and supports prefix-by-prefix failure reports.

## Recording and execution

`record_replay()` derives command ticks and gap-free sequence numbers from the
runtime, applies each typed command, and records a canonical `SaveState`
fingerprint after every command. `execute_replay()` independently validates the
engine version, seed, data manifest, active configuration, initial state,
command position, and each checkpoint. Playback stops at the first mismatch
with the checkpoint index, tick, command prefix, and expected/observed hashes.
`execute_replay_observed()` also reports the actual failing checkpoint to a
diagnostic caller without weakening fail-fast behavior.

Format v1 reserves exactly 1,024 Xoshiro256++ rolls per simulated tick and
limits one advance command to 1,000,000 ticks. The runtime supports speed
changes, dual-AI toggling, visibility controls, forced victory checks, and
explicit tick advances through the shared `run_simulation_tick()` path.

F-011B3 also removes process-random iteration from state-affecting
manufacturing completions, simultaneous fleet arrivals, blockade transitions,
and the AI's equal-count reinforcement choice. Regression tests require stable
key order at each boundary.

## Simulation-data identity

`compute_simulation_data_manifest()` accepts in-memory file bytes and therefore
runs identically in native and WASM builds. The native convenience function
reads every `.DAT` in a directory. Names are ASCII, flat, uppercased, sorted,
and unique. Non-DAT paths, traversal, case-colliding duplicates, malformed
hashes, inconsistent byte totals, and inconsistent aggregate hashes fail
closed.

Fingerprint v1 uses domain-separated FNV-1a 64 over length-framed names and
bytes. It is a deterministic equivalence and corruption signal, not a
cryptographic authenticator. Hash values serialize as fixed-width lowercase
hex strings so JavaScript cannot lose integer precision.

The local original-data fixture contains 51 inputs and 50,597 bytes. Its
aggregate fingerprint is `5facb1c7ba0e81ad`.

## Cross-runtime fixture

`crates/rebellion-data/tests/fixtures/replay_seed42_v1.json` is the single
reviewed artifact used by both runtimes. It contains nine commands and nine
checkpoints through tick 25. Native `replay-gate` and the query-gated WASM
runner both embed and decode those exact 13,480 bytes, rebuild the seed-42
campaign independently, and echo the original text into their reports.

The browser gate runs only at `?replay-check=seed42-v1`. It strictly loads
`runtime.orpk`, publishes `window.__openRebellionReplay`, and never falls into
normal gameplay. Invalid query values and a missing pack fail before any
partial replay state is reported. Without the query, ordinary startup still
uses four requests and reaches the authentic bitmap menu.

## Validation

```bash
scripts/mini-cargo.sh -- test -p rebellion-data replay --lib
scripts/mini-cargo.sh -- test -p rebellion-data --test replay_manifest -- --ignored
scripts/mini-cargo.sh -- check -p rebellion-app --target wasm32-unknown-unknown
python3 scripts/check-replay-equivalence.py --skip-build --json
```

The ignored fixture test requires the locally supplied original `.DAT` files.
It records a nine-command, 25-tick, 200-system campaign, reloads its initial
state through the current save format, and checks every command-prefix
fingerprint against a cross-process golden, which currently runs
`v1:dba544d28bce7aa4` -> `v1:df820646127bbe3b`. Its causes, by save version:

- v39: characters gained the officer rank, its post and ranked names.
- The AI's mission dispatch does not require `can_be_commander`, a check
  none of `FUN_00508250`'s validators makes.
- v40: the world gained each system's last-arrived side, with the
  post-battle pass that reads it.
- v41: a character's Move became a Move mission whose travelling members
  record the fleet they board.
- Han Solo's mission speed (GNPRTB 3083) matches his loaded MJCHARSD record
  id `0x243`, not the original's object id `0x33000243`.
- v42: the AI gained module 12's class G character tasks, saved with each
  AI side, with characters reserved for research only while the tree is
  unfinished.
- v43: research became the original Research missions, with per-side
  points pools, the passive timer `0x381` and exhausted trees.
- v44: the AI's production became the original modules 1, 2, 3, 5, 6, 7, 10
  and 11, with their census, goal token and budgets; a production
  Destination may name a fleet.
- v45: a travelling character or special force stands at its destination
  from the order with an en route flag; a member of a hidden record's
  mission (Move among them) carries the hidden-mission flag; a member's
  transit keeps only its arrival day.
- v46: the AI became the original strategic AI: module 13's modes and
  controller, module 8's regional records and postures, module 9's fleet
  tasks and module 12's class I officer tasks, saved with each AI side.
- A system without regiments goes to the side whose support reaches
  GNPRTB 7760 (`FUN_0050b610` through `FUN_0055a080`), else to neither.
- The AI's class census marks only the Death Star's class (CAPSHPSD family
  `0x18`, `FUN_004739b0`) as no combat class, so the best attack class is
  the strongest warship.
- The production modules run in the scheduler's ascending kind order
  (`FUN_004ec940`, `FUN_004be980`).
- v47: the AI's research runs through persistent per-tree planners, whose
  order at a system without a working yard is refused before a mission
  exists (`FUN_004bced0`, `FUN_0056c510`).
- v48: a save restores each slotmap's free order, so a loaded game reuses
  freed slots as the uninterrupted one does.
- v49: ships carry the original six damage counters; the AI moves damaged
  ships to its nearest shipyard (`FUN_004c8e90`); repair runs on the 30-day
  and 2-day timers with the damage-control roll (`FUN_005010a0`,
  `FUN_00501c50`).
- A battle's loser withdraws by the original morale retreat
  (`FUN_005d19a0`).
- v50: the twelve AI tuning fields nothing read are gone.
- v51: the slot order survives an emptied arena.
- Repair's 2-day interval needs a working own yard (`FUN_00528040`).
- v52: the AI runs the original round scheduler: rate+1 rounds a day (5 at
  Fast), one order a round, at most 10 orders a side a day (5 at Fast),
  with every module taking one turn a round (`FUN_00485b40`,
  `FUN_004361a0`, `FUN_004bede0`), as the day's last stage. The Diplomacy
  support cap and the per-evaluation covert and recon caps are gone.
  Module 11 reads module 8's live record bits, production resumes after
  any step-2 or step-3 order (`FUN_004c8ba0`), and a Research planner sits
  out while its mission flies (`FUN_004bda40`). The player's advisor runs one module a
  round in its slot (`FUN_00439a10`), and a saved `day_started` flag closes
  a day whose first round waited for a battle.
- A mission whose target the original forbids is refused at creation
  (`FUN_00542b60`), and the AI's pickers take only the targets the
  original's planner queries return.
- A bombardment ends with the original's finalize (`FUN_0058e320`).
- Who holds a system by support alone is `FUN_0055a080`'s three outcomes,
  which also drive the support-change record.
- A blockade runs timer `0x386`, shifting the blockader's support every 30
  days (`FUN_0050cb20`), and the sourceless maintenance cooldowns are gone.
- System bit 11 is a usable Death Star in the sector (`FUN_005178b0`), not
  high support.
- A neutral system's seeded support is the Alliance's, the Empire's 100
  minus it (`FUN_00559700`).
- v53: the economy record keeps no copy of system bit 11.

Any save layout or
simulation change moves every fingerprint; regenerate the golden only for a
named cause and name it in the commit. The v13 goldens did not reproduce
even at their own commit `e8d4945`; that cause remains open. The unit tests use
synthetic data and run in normal repository test passes.

## Next implementation boundary

F-011B4 proves replay-executor equivalence for one 25-tick campaign. F-012 must
now converge the interactive app and `rebellion-playtest` on the same command,
tick, event, and combat paths. Long-running multi-seed replay remains an M1
gate, so this result is not a claim of full interactive game parity.
