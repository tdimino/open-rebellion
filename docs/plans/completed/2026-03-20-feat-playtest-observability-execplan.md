---
title: "plan: playtest logging and observability for balance diagnosis"
description: "ExecPlan for richer headless playtest diagnostics, JSONL streaming, REPL observability, and zero-combat diagnosis"
category: "plan"
status: "planned"
created: 2026-03-20
updated: 2026-03-20
tags: ["playtest", "observability", "logging", "repl", "jsonl", "combat"]
---

# Make Headless Playtests Explain Fleet Movement and Missing Combat

This ExecPlan is a living document. Sections Progress, Surprises &
Discoveries, Decision Log, and Outcomes & Retrospective must be
kept up to date as work proceeds.

## Purpose / Big Picture

The headless playtest binary should be useful for balance work, not just for producing a pile of event counts. After this work, a designer should be able to run one deterministic dual-AI simulation and immediately answer five practical questions: who controls the map right now, which fleets are still in hyperspace transit, whether combat was ever eligible to fire, what raw events actually happened in machine-readable form, and why fleets kept moving without producing battles. The visible proof is a richer `--summary`, a new `--jsonl` stdout mode, new REPL inspection commands, and event payloads that name real systems instead of printing `Debug` handles such as `SystemKey(3v1)`.

The primary gameplay diagnosis target is the current reproduction: `cargo run -q -p rebellion-playtest -- data/base --seed 42 --ticks 3000 --dual-ai --summary` produces 8013 events and 642 fleet arrivals, but zero `combat_space` events. This plan keeps code changes minimal and focuses on exposing the cause before changing AI or combat behavior.

## Progress

- [x] (2026-03-21 01:11Z) Reproduced the dual-AI baseline: `--seed 42 --ticks 3000 --dual-ai --summary` produced 8013 events, 642 `fleet_arrived`, and 0 `combat_space`.
- [x] (2026-03-21 01:11Z) Inspected the current headless CLI, event logger, simulation event emission, movement state, and AI fleet-deployment logic.
- [x] (2026-03-21 01:11Z) Identified the minimal file set for this work: `crates/rebellion-playtest/src/main.rs`, `crates/rebellion-playtest/src/logger.rs`, `crates/rebellion-data/src/simulation.rs`, `crates/rebellion-core/src/ai.rs`, `crates/rebellion-core/src/movement.rs`, and `docs/mechanics/play-testing.md`.
- [ ] Add summary-time diagnostics for control, transit, combat triggers, hostile arrivals, and transit retargeting.
- [ ] Add `--jsonl` raw-event streaming to stdout without breaking REPL or `--exec`.
- [ ] Add REPL commands `systems`, `transit`, and `events N`.
- [ ] Replace `Debug`-formatted event payload fields with human-readable names while preserving machine-readable IDs where useful.
- [ ] Validate the zero-combat diagnosis with the reproduced seed-42 dual-AI run and update this document with the final evidence.

## Surprises & Discoveries

- Observation: The current documentation already describes friendly JSON event payloads, but the actual JSONL output still uses `Debug` formatting.
  Evidence: `docs/mechanics/play-testing.md` shows examples like `"system_name":"Coruscant"`, while `playtest-seed42.jsonl` begins with lines such as `{"event_type":"fog_revealed","details":{"system":"SystemKey(3v1)"}}`.

- Observation: The user-reported dual-AI reproduction is accurate.
  Evidence: `cargo run -q -p rebellion-playtest -- data/base --seed 42 --ticks 3000 --dual-ai --summary` printed `Total events: 8013`, `fleet_arrived 642`, and no `combat_space` line.

- Observation: The AI prefers reinforcement over attack whenever any friendly controlled system has no fleet present.
  Evidence: `crates/rebellion-core/src/ai.rs` at lines 704-712 computes a single `reinforce_target` as the first friendly controlled empty system, and lines 742-750 always choose reinforcement before attack.

- Observation: Movement orders can be reissued mid-transit if the new destination differs from the old one, because deduplication only skips identical destinations.
  Evidence: `crates/rebellion-data/src/simulation.rs` at lines 985-996 only checks `o.destination == *to_system`, while `crates/rebellion-core/src/movement.rs` at lines 197-210 states that `order()` replaces any existing transit order.

- Observation: Combat only fires when both factions already occupy the same `System.fleets` list during the combat phase.
  Evidence: `crates/rebellion-data/src/simulation.rs` at lines 135-169 builds combat triggers solely from systems containing at least one Alliance fleet and one Empire fleet.

## Decision Log

- Decision: Keep this task diagnostic-only. Do not change AI behavior or combat rules in the same patch.
  Rationale: The request is about observability and diagnosing why combat never happens. Mixing in behavioral fixes would make balance conclusions harder to trust and would enlarge the patch unnecessarily.
  Date/Author: 2026-03-21 / Codex

- Decision: Preserve the `GameEventRecord` top-level schema and improve the `details` payload contents instead of inventing a second event envelope.
  Rationale: This keeps downstream JSONL tooling stable. Existing consumers can keep reading `tick`, `wall_ms`, `system`, `event_type`, and `details`.
  Date/Author: 2026-03-21 / Codex

- Decision: Make `--jsonl` stream raw JSONL to stdout and treat it as incompatible with `--repl` and `--exec`.
  Rationale: REPL and `--exec` already use stdout as their protocol channel. Mixing protocols would make the tool harder to script and harder for an LLM agent to parse.
  Date/Author: 2026-03-21 / Codex

- Decision: Keep `systems`, `transit`, and `events N` as playtest REPL commands first, not shared command-palette commands.
  Rationale: The request is specifically for REPL diagnostics. Avoiding `crates/rebellion-core/src/commands.rs` keeps GUI churn out of scope and reduces risk.
  Date/Author: 2026-03-21 / Codex

## Outcomes & Retrospective

Planning is complete, implementation is not. The key outcome so far is a concrete diagnosis path: the current zero-combat run is explainable from the existing code, and the requested observability work can be concentrated in the headless playtest and simulation event-formatting layers. The likely explanation is not “combat is broken,” but “fleets are spending their time reinforcing friendly empty systems and being retargeted mid-transit, so hostile co-location almost never occurs.” This document should be updated after implementation with the exact counters from the improved summary.

## Context and Orientation

The relevant terms of art are small but important:

`JSONL` means “JSON Lines,” which is one valid JSON object per line. It is useful because tools such as `jq`, DuckDB, and scripts can process it incrementally.

`REPL` means “read-eval-print loop.” In this repository it is the persistent `rebellion-playtest --repl` mode that accepts commands on stdin and writes JSON responses on stdout.

A `transit order` is an active hyperspace movement instruction stored in `MovementState`. It records origin system, destination system, total transit time, and ticks elapsed.

A `controlled system` is a star system whose `controlling_faction` is `Some(Faction::Alliance)` or `Some(Faction::Empire)`. `None` means neutral or contested.

A `combat trigger` in the current implementation is not “a fleet arrived at an enemy system.” It is specifically “the combat phase found both an Alliance fleet and an Empire fleet in the same `System.fleets` list.”

The current state of the relevant files is:

`crates/rebellion-playtest/src/main.rs` defines CLI flags, the summary execution path, and the REPL command loop. The current `Args` type exposes `--summary`, `--output`, `--exec`, and `--repl`, but not a stdout JSONL streaming mode. `dispatch_command()` currently returns human-readable strings only and does not surface transit orders, controlled-system listings, or recent events.

`crates/rebellion-playtest/src/logger.rs` stores all emitted `GameEventRecord` values and prints a summary that only counts event types. It has no access to world state, movement state, or diagnostic counters.

`crates/rebellion-data/src/simulation.rs` emits the event records. Most payloads currently use `format!("{:?}", key)` or `format!("{:?}", action)`, which is why event JSON contains `SystemKey(...)` and Rust `Debug` dumps instead of useful names.

`crates/rebellion-core/src/ai.rs` contains the fleet movement heuristic. The current fleet deployment logic will reinforce a friendly empty system before attacking an enemy system, and it chooses one shared reinforce target for all fleets of that faction in that evaluation pass.

`crates/rebellion-core/src/movement.rs` defines `MovementState` and makes clear that a new order replaces any previous order for the same fleet.

`docs/mechanics/play-testing.md` needs a cleanup pass after implementation because it currently documents a friendlier JSON shape than the binary actually emits.

Useful baseline commands and current outputs are:

Working directory: `/Users/tomdimino/Desktop/Programming/open-rebellion`

`cargo run -q -p rebellion-playtest -- data/base --exec list_active_fleets --seed 42`

Expected current output before this work:
`Empire fleet at Coruscant — 7 ships`
`Alliance fleet at Yavin — 1 ships`
`Alliance fleet at Boordii — 1 ships`

`cargo run -q -p rebellion-playtest -- data/base --seed 42 --ticks 3000 --dual-ai --summary`

Expected current output before this work:
`Total events: 8013`
`fleet_arrived 642`
No `combat_space` line at all.

## Plan of Work

Start in `crates/rebellion-data/src/simulation.rs` because that is the narrowest place to improve event payload quality without touching core game rules. Add small helper functions that turn internal keys into stable JSON objects containing both an identifier and a human-readable name. For a system this should include at least the slotmap key string and the actual system name. Apply those helpers to movement, combat, mission, fog, blockade, uprising, research, Jedi, and AI event payloads. The important rule is that payloads should become readable without losing the machine-reference fields that make them scriptable.

While still in `crates/rebellion-data/src/simulation.rs`, add a small diagnostic struct to `SimulationStates` that accumulates the counters the summary actually needs. Keep it focused. It should count how many times combat was eligible to fire, how many hostile arrivals occurred, how many transit orders were issued, how many were retargeted while already in transit, and how often systems were targeted for movement. This is enough to explain “why did fleets keep moving but never fight?” without introducing new gameplay behavior or flooding the event log with synthetic events.

Then update `crates/rebellion-playtest/src/logger.rs` so summary rendering can accept the final `GameWorld`, `SimulationStates`, and the stored events. The new summary should still print event counts, but it also needs three new sections. The first is control, showing Alliance-controlled, Empire-controlled, and neutral or contested system counts, plus a short list of the actual controlled system names when that list is small enough to be readable. The second is transit, showing active movement orders with side, origin, destination, progress, and ticks remaining. The third is combat diagnostics, showing how many combat triggers actually fired, how many hostile arrivals occurred, how many transit retargets happened, and the top movement destinations for each faction. This is the smallest useful set that can distinguish “fleets are not meeting” from “combat resolution is failing.”

After that, update `crates/rebellion-playtest/src/main.rs`. Add a new `--jsonl` boolean flag that streams each emitted `GameEventRecord` to stdout immediately as JSONL. Keep `--output` for file export. Make `--jsonl` conflict with `--repl` and `--exec`, because those modes already define stdout formats. When `--jsonl` is active, keep progress updates and the final summary on stderr so stdout remains a clean JSONL stream. Do not remove the in-memory logger, because the REPL and summary still need access to the stored records.

Still in `crates/rebellion-playtest/src/main.rs`, add three REPL-only inspection commands. `systems` should print a JSON object containing the current counts and lists of controlled systems by faction. `transit` should print the current active movement orders with readable origin and destination names, progress fraction, and remaining ticks. `events N` should print the last `N` stored event records from `EventLogger`, using the now-human-readable payloads. Implement these as explicit REPL parser branches before the generic `dispatch_command()` path so that no GUI command registry changes are required.

Finish with `docs/mechanics/play-testing.md` and any focused unit tests. The documentation must stop promising payload fields that do not exist. The tests should stay small and structural: event-format helper tests, summary-format tests using a minimal synthetic world, and REPL helper tests for `events N` and `transit` rendering. The goal is to lock in readability and diagnostic usefulness without writing a large integration harness.

## Concrete Steps

All commands in this section assume the working directory is `/Users/tomdimino/Desktop/Programming/open-rebellion`.

1. Inspect the existing playtest entry point and summary path.

```bash
sed -n '1,240p' crates/rebellion-playtest/src/main.rs
sed -n '1,220p' crates/rebellion-playtest/src/logger.rs
```

Expected result: `Args` includes `--summary` and `--output` but no `--jsonl`; `EventLogger::print_summary()` only prints total events, final tick, and counts by event type.

2. Inspect the event emission path and the current combat trigger logic.

```bash
sed -n '1,260p' crates/rebellion-data/src/simulation.rs
sed -n '930,1010p' crates/rebellion-data/src/simulation.rs
sed -n '688,770p' crates/rebellion-core/src/ai.rs
sed -n '170,230p' crates/rebellion-core/src/movement.rs
```

Expected result: event payloads contain `format!("{:?}", ...)`, combat triggers require mixed-faction fleets in the same system, fleet deployment prefers reinforcement, and a new movement order replaces the previous one.

3. Reproduce the current zero-combat baseline before making changes.

```bash
cargo run -q -p rebellion-playtest -- data/base --seed 42 --ticks 3000 --dual-ai --summary
```

Expected result before implementation: `Total events: 8013`, `fleet_arrived 642`, and no `combat_space` count.

4. Implement human-readable event payload helpers and summary diagnostics, then run formatting and tests.

```bash
cargo fmt
cargo test -p rebellion-playtest
cargo test -p rebellion-data simulation
```

Expected result: formatting succeeds and the focused test suites pass. If there are no existing package-specific tests, add narrow tests and run the package tests again until they pass.

5. Verify stdout JSONL mode stays machine-readable.

```bash
cargo run -q -p rebellion-playtest -- data/base --seed 42 --ticks 15 --dual-ai --jsonl | head -n 5
```

Expected result after implementation: five JSON objects are printed to stdout, each on a single line. `details` values use real names such as `"system_name":"Coruscant"` or `"origin_name":"Yavin"` and do not include raw `Debug` strings like `SystemKey(` or `DispatchMission {`.

6. Verify the improved summary exposes balance-useful diagnostics.

```bash
cargo run -q -p rebellion-playtest -- data/base --seed 42 --ticks 3000 --dual-ai --summary
```

Expected result after implementation: the summary still prints event counts, and also prints sections for controlled systems, active transit orders, and combat diagnostics. The combat section should make the missing-combat reason visible, for example by showing zero hostile arrivals and a nonzero number of reinforcement-targeted or retargeted transit orders.

7. Verify the new REPL inspection commands.

```bash
cargo run -q -p rebellion-playtest -- data/base --repl --seed 42 <<'EOF'
systems
transit
tick 50
transit
events 5
quit
EOF
```

Expected result after implementation: `systems` returns JSON that names controlled systems, `transit` returns JSON listing active orders with origin and destination names plus progress, and `events 5` returns the five most recent event records in the same human-readable JSON shape used by `--jsonl`.

8. Update the play-testing documentation to match the real interface.

```bash
sed -n '1,220p' docs/mechanics/play-testing.md
```

Expected result after implementation: the doc mentions `--jsonl`, `systems`, `transit`, and `events N`, and its JSON examples match the real event payload format.

## Validation and Acceptance

This work is accepted only when a designer can answer “why did fleets not fight?” from a single deterministic run without reading Rust code.

The required acceptance checks are:

`cargo run -q -p rebellion-playtest -- data/base --seed 42 --ticks 3000 --dual-ai --summary`

The summary must show current map control, fleets still in transit, and combat diagnostics beyond raw event counts. It must give enough evidence to distinguish “fleets never co-located” from “combat co-located but failed to resolve.”

`cargo run -q -p rebellion-playtest -- data/base --seed 42 --ticks 30 --dual-ai --jsonl`

Every stdout line must be valid JSON. Event payloads must use human-readable names and structured fields, not raw Rust `Debug` text.

`cargo run -q -p rebellion-playtest -- data/base --repl --seed 42`

Inside the REPL, `systems`, `transit`, and `events 5` must return information that is immediately useful for an LLM agent or a human operator diagnosing fleet movement and missing combat.

The diagnosis for the known seed-42 dual-AI case must be explicit in either the summary or the recent events. A successful outcome is any concrete explanation backed by the new counters, such as “all fleet movement went to friendly empty systems,” “hostile arrivals stayed at zero,” or “orders were repeatedly retargeted before fleets could meet.”

## Idempotence and Recovery

The code-editing steps in this plan are additive and should be safe to rerun. Re-running `cargo fmt`, `cargo test`, or the deterministic playtest commands should not change game state outside generated output files.

If the implementation becomes noisy or too invasive, recover by trimming scope back to the agreed minimum. The rollback order is: remove documentation drift last, keep human-readable event payloads, keep summary diagnostics, and only drop REPL niceties if needed. Do not “fix combat” as part of recovery; that would change the question being answered.

If `--jsonl` causes stdout or stderr mixing problems, recover by preserving stdout for JSONL only and moving all human-readable progress and summary text to stderr. That keeps the machine interface stable.

## Interfaces and Dependencies

No new third-party dependencies should be required. Use the crates already present in this workspace: `clap`, `serde`, `serde_json`, and the existing game-world types.

At completion, the following interfaces should exist or be meaningfully expanded:

`crates/rebellion-playtest/src/main.rs`

The CLI `Args` type should include a `jsonl: bool` flag. The REPL loop should recognize `systems`, `transit`, and `events N` as first-class commands.

`crates/rebellion-playtest/src/logger.rs`

`EventLogger` should still own the full event list, and it should expose enough access for recent-event inspection and richer summary rendering. A summary-rendering function should accept the final `GameWorld` and `SimulationStates`, or equivalent read-only data derived from them.

`crates/rebellion-data/src/simulation.rs`

`SimulationStates` should carry a small diagnostic accumulator for summary-time counters. Event payload construction should use helper functions that include readable names for systems and other common references.

`crates/rebellion-core/src/ai.rs` and `crates/rebellion-core/src/movement.rs`

These files are not expected to change behavior in this task, but the plan relies on their current semantics: reinforcement priority in fleet deployment and replacement semantics for new transit orders.
