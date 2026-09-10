---
title: "Deterministic Replay Contract"
description: "Versioned command streams, simulation-data identity, checkpoints, and remaining cross-runtime gates"
category: agent-docs
created: 2026-09-09
updated: 2026-09-10
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
runner both embed and decode those exact 13,482 bytes, rebuild the seed-42
campaign independently, and echo the original text into their reports.

The browser gate runs only at `?replay-check=seed42-v1`. It strictly loads
`runtime.orpk`, publishes `window.__openRebellionReplay`, and never falls into
normal gameplay. Invalid query values and a missing pack fail before any
partial replay state is reported. Without the query, ordinary startup still
uses four requests and reaches the authentic bitmap menu.

## Validation

```bash
cargo test -p rebellion-data replay --lib
cargo test -p rebellion-data --test replay_manifest -- --ignored
cargo check -p rebellion-app --target wasm32-unknown-unknown
python3 scripts/check-replay-equivalence.py --skip-build --json
```

The ignored fixture test requires the locally supplied original `.DAT` files.
It records a nine-command, 25-tick, 200-system campaign, reloads its initial
state through save v13, and checks every command-prefix fingerprint against a
cross-process golden. F-007E re-reviewed the unchanged command stream after
adding persisted troop cargo and occupation state. It now starts at
`v1:b38248eb039a8032` and ends at `v1:cde64607b027b1d1`. The unit tests use
synthetic data and run in normal repository test passes.

## Next implementation boundary

F-011B4 proves replay-executor equivalence for one 25-tick campaign. F-012 must
now converge the interactive app and `rebellion-playtest` on the same command,
tick, event, and combat paths. Long-running multi-seed replay remains an M1
gate, so this result is not a claim of full interactive game parity.
