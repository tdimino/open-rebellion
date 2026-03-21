# Build Autoresearch Telemetry, Tunable Config, and a Fast 3-Seed Balance Loop

This ExecPlan is a living document. Sections Progress, Surprises &
Discoveries, Decision Log, and Outcomes & Retrospective must be
kept up to date as work proceeds.

## Purpose / Big Picture

Open Rebellion already has the core pieces of an autoresearch pipeline: a headless binary in `crates/rebellion-playtest/src/main.rs`, structured JSONL events emitted from `crates/rebellion-data/src/simulation.rs`, and a scalar evaluator in `scripts/eval_game_quality.py`. What it does not have yet is the right observability or the right control surface. Today a run can score well while still being strategically stagnant, and neighboring seeds produce almost the same campaign because the AI is dominated by deterministic rules and only lightly perturbed by randomness.

After the work in this plan, a designer should be able to edit one JSON config file, run a 3-seed batch, and answer these practical questions without reading Rust code: how often map control changed, whether fleet engagements were fair or one-sided, which missions and which characters were actually used, whether research advanced, how often uprisings happened, whether both factions stayed strategically active, and whether the campaign landed inside the desired target ranges. The visible proof is a new config-driven playtest path, richer per-run summary output, a corrected evaluator, and an autoresearch loop that can complete at least 12 iterations per hour with 3 seeds per iteration.

The current measured baseline from the checked-out binary is important context. Running `./target/debug/rebellion-playtest data/base --seed 42 --ticks 5000 --dual-ai --summary` on 2026-03-21 produced 11,991 events, 991 `combat_space`, 677 `fleet_arrived`, 6 `manufacturing_complete`, 10 attack orders, 1,348 reinforce orders, no victory, 3 Alliance-controlled systems, and 9 Empire-controlled systems. Running the same command for seeds 41, 42, and 43 produced effectively the same strategic outcome each time. The current evaluator scores the seed-42 run at `0.6562`, which is too generous for a campaign that never reaches victory, barely manufactures anything, leaves 188 systems neutral, and concentrates all 991 space battles at Yavin.

## Progress

- [x] (2026-03-21 15:02Z) Inspected the current headless pipeline in `crates/rebellion-playtest/src/main.rs`, `crates/rebellion-playtest/src/logger.rs`, `crates/rebellion-core/src/game_events.rs`, `crates/rebellion-data/src/simulation.rs`, `crates/rebellion-core/src/ai.rs`, `crates/rebellion-core/src/movement.rs`, and `scripts/eval_game_quality.py`.
- [x] (2026-03-21 15:02Z) Verified the current seed-42 dual-AI 5000-tick baseline from `./target/debug/rebellion-playtest`: 11,991 total events, 991 space battles, 677 fleet arrivals, 6 manufacturing completions, 10 attack orders, 1,348 reinforce orders, no victory.
- [x] (2026-03-21 15:02Z) Compared seeds 41, 42, and 43 and confirmed that combat count, fleet arrivals, manufacturing completions, control counts, and attack-versus-reinforce mix are effectively identical.
- [x] (2026-03-21 15:02Z) Confirmed that committed sample artifacts such as `playtest-seed42.jsonl` and `playtest-dual-sample.jsonl` are older than the current binary behavior and should not be treated as the present balance baseline.
- [x] (2026-03-21 15:02Z) Identified concrete missing metrics and scoring flaws, including absent control-change telemetry, absent research activity, missing character and faction fields on mission events, and a faulty faction-balance calculation in `scripts/eval_game_quality.py`.
- [ ] Implement richer telemetry and a per-seed summary sidecar that expose control changes, fleet-strength comparisons, mission diversity, character utilization, research progress, uprising frequency, and battle concentration.
- [ ] Externalize autoresearch knobs into a JSON config file and thread those knobs through AI evaluation, movement timing, production policy, research policy, telemetry, and scoring.
- [ ] Rework the AI around an explicit parity-versus-augmentation split so the original omniscient, rule-based, six-stage pipeline is reproduced intentionally before any anti-stagnation help is enabled.
- [ ] Add a config-driven 3-seed autoresearch runner that prebuilds once, runs the binary directly, evaluates aggregated results, and keeps or discards candidate configs.
- [ ] Validate that the default augmented config can hit the desired target ranges or, if it misses, emits enough structured evidence to explain exactly why.

## Surprises & Discoveries

- Observation: The committed JSONL samples in the repository do not reflect the current checked-out binary.
  Evidence: `playtest-seed42.jsonl` contains only 6 event types and 0 combat events, while `./target/debug/rebellion-playtest data/base --seed 42 --ticks 5000 --dual-ai --summary` on 2026-03-21 reported 991 `combat_space` events and 10 total event types.

- Observation: The current strategic outcome is nearly seed-invariant even though total event count wiggles slightly.
  Evidence: Seeds 41, 42, and 43 all produced 991 `combat_space`, 677 `fleet_arrived`, 6 `manufacturing_complete`, 10 attack orders, 1,348 reinforce orders, 3 Alliance-controlled systems, 9 Empire-controlled systems, and no victory.

- Observation: The current battle load is not “healthy combat variety.” It is one local stalemate repeated hundreds of times.
  Evidence: Parsing `/tmp/open-rebellion-seed42-current.jsonl` showed all 991 `combat_space` events occurred at `Yavin`.

- Observation: The current fleet movement pattern is concentrated into a few reinforcement shuttles.
  Evidence: Seed 42 produced 178 arrivals at `Geedon V`, 178 at `Boordii`, 157 at `Coruscant`, 157 at `Averam`, and only 5 at `Yavin`.

- Observation: The current evaluator can label a strategically weak run as “good enough” because it overweights raw combat ratio and underweights stagnation.
  Evidence: `python3 scripts/eval_game_quality.py /tmp/open-rebellion-seed42-current.jsonl --json` returned `score: 0.6562` even though the run had no victory, only 6 manufacturing completions, 188 neutral systems, and zero visible control changes.

- Observation: The current faction-balance sub-score is not measuring faction balance reliably.
  Evidence: `scripts/eval_game_quality.py` counts Alliance versus Empire actions from the presence of `details.dual_ai` instead of from a stable faction field, while `DispatchMission` telemetry currently omits explicit faction and character identity.

- Observation: The simulation already mutates control in some paths, but there is no dedicated control-change event or summary metric.
  Evidence: `crates/rebellion-data/src/simulation.rs` flips `System.control` during `UprisingEvent::UprisingBegan`, but only logs `uprising_began` with the system name; there is no `control_changed` event and `UprisingSubdued` currently emits no event at all.

- Observation: The research system exists but the current headless AI does not appear to ever schedule research work.
  Evidence: `crates/rebellion-core/src/research.rs` and `crates/rebellion-data/src/simulation.rs` support research state and `research_unlocked` logging, but the AI action enum in `crates/rebellion-core/src/ai.rs` has no research action and the seed-42 run emitted zero research events.

- Observation: Throughput is already comfortably inside the 12-iterations-per-hour target if the loop avoids `cargo run` inside each iteration.
  Evidence: The prebuilt `./target/debug/rebellion-playtest` completed a 5000-tick seed in about 1.0 to 1.2 seconds on 2026-03-21. Three seeds therefore take only a few seconds before scoring, so the real throughput risk is loop design and unnecessary recompilation, not the simulation core itself.

## Decision Log

- Decision: Add two telemetry layers, not one.
  Rationale: Raw JSONL is still useful for forensics, but the autoresearch loop needs a cheap per-seed summary file that can be scored without reparsing every event every time. The implementation should therefore emit both JSONL and a structured run summary.
  Date/Author: 2026-03-21 / Codex

- Decision: Keep telemetry mostly event-driven and add coarse snapshots at a fixed interval such as 250 ticks.
  Rationale: Logging every system every tick would bury the useful information in noise and would threaten loop throughput. The right compromise is “log on change” plus “sample coarse world snapshots.”
  Date/Author: 2026-03-21 / Codex

- Decision: Split “original-faithful” behavior from “augmentation” behavior in the config.
  Rationale: The user explicitly wants both parity and improvement. Those goals are compatible only if the code can run in a parity mode that preserves the original AI’s omniscient, distributed, rule-based character, while keeping anti-stagnation helpers behind explicit switches.
  Date/Author: 2026-03-21 / Codex

- Decision: Do not duplicate the full contents of `data/base/json/GNPRTB.json` into the new autoresearch config.
  Rationale: GNPRTB, mission tables, and uprising tables are already original-data inputs. The new config should contain high-level tuning knobs and optional parameter overrides by ID, not a second copy of the entire balance database.
  Date/Author: 2026-03-21 / Codex

- Decision: Use one top-level JSON config for runtime, telemetry, AI, movement, production, research, scoring, parity flags, augmentation flags, and optional data overrides.
  Rationale: The Karpathy loop described by the user is “edit JSON config -> run headless -> measure -> keep or discard.” A single file keeps iteration simple and makes candidate promotion easy.
  Date/Author: 2026-03-21 / Codex

- Decision: Prebuild once and run `target/release/rebellion-playtest` directly from the loop.
  Rationale: Recompiling during every iteration would be wasteful and is the easiest way to miss the throughput target. The loop should build once, then treat the binary as a stable executable.
  Date/Author: 2026-03-21 / Codex

- Decision: Treat battle concentration as a first-class negative signal.
  Rationale: A run with 991 battles all at Yavin is not “healthy combat.” The scorer should explicitly penalize extreme combat concentration even when the raw combat percentage looks good.
  Date/Author: 2026-03-21 / Codex

## Outcomes & Retrospective

Planning is complete and implementation has not started yet. The most important outcome from this review is that the current pipeline is not blocked by runtime speed. It is blocked by observability quality, parameterization quality, and scoring quality. The existing headless binary is already fast enough for an aggressive autoresearch loop, but the loop would currently optimize the wrong signals because the telemetry does not expose map change, research, or character usage cleanly, and the evaluator still mistakes repetitive combat for strategic quality. The next person picking up this document should focus on making the metrics truthful before attempting broad balance sweeps.

## Context and Orientation

The repository already contains the core ingredients for this work.

`crates/rebellion-playtest/src/main.rs` is the headless entry point. It owns the CLI, runs the tick loop, streams JSONL when `--jsonl` is present, and prints a human-readable summary. It does not yet read a tuning config and it does not emit a structured summary sidecar.

`crates/rebellion-playtest/src/logger.rs` owns the in-memory event list and the human summary. The summary prints event counts, galaxy control counts, active transit orders, and basic combat diagnostics. It does not yet compute control-change totals, battle concentration, mission diversity, character utilization, research progress, or uprising rates.

`crates/rebellion-core/src/game_events.rs` defines the event envelope and the existing event type names. It currently has no dedicated event type for a system control change, no summary schema, and no snapshot schema.

`crates/rebellion-data/src/simulation.rs` is the real telemetry hub. It applies manufacturing completions, movement arrivals, combat, missions, events, AI actions, blockade changes, uprisings, betrayal, Death Star work, research unlocks, Jedi training, and victory checks. This file is where most new event fields and new summary accumulation should be added.

`crates/rebellion-core/src/ai.rs` contains the current hardcoded AI thresholds and fleet policy. Today these include `AI_TICK_INTERVAL = 7`, `DIPLOMACY_SKILL_THRESHOLD = 60`, `DIPLOMACY_TARGET_POPULARITY_CAP = 0.8`, `ESPIONAGE_SKILL_THRESHOLD = 50`, `COVERT_MIN_SUCCESS_PROB = 0.30`, `MAX_COVERT_OPS_PER_EVAL = 3`, and `MAX_CONSTRUCTION_YARDS = 5`. Fleet policy is currently reinforcement-heavy and still much simpler than the original reverse-engineered AI.

`crates/rebellion-core/src/movement.rs` hardcodes `DISTANCE_SCALE = 2` and `MIN_TRANSIT_TICKS = 10`. Those are exactly the kind of parameters that belong in the autoresearch config instead of in fixed Rust constants.

`crates/rebellion-core/src/research.rs` supports research progression and unlock timing, but the AI has no way to initiate research today, so current playtest runs never surface research behavior.

`scripts/eval_game_quality.py` computes the current scalar score. It looks only at event diversity, combat ratio, victory timing, faction balance, and manufacturing activity. It does not score control changes, mission diversity, research progress, uprising frequency, battle spread, or character usage. It also derives faction balance incorrectly in dual-AI runs because the telemetry is not explicit enough.

`ghidra/notes/ai-behavior-analysis.md` is the parity anchor for AI work. It documents the original game’s six-function distributed AI pipeline: galaxy evaluation, system iteration, strength assessment, system validation, entity dispatch, and movement orders. It also confirms that the original AI was omniscient and assessed garrison strength using ships, troops, and facilities, not only fleet presence.

The following terms are used throughout this plan.

A “summary sidecar” is a second JSON file written once per seed run. It contains aggregated metrics such as control changes, battle concentration, and per-faction activity totals. It complements JSONL rather than replacing it.

A “parity mode” is a config mode where the AI honors the original reverse-engineered design constraints: omniscient map knowledge, distributed rule-based evaluation, two-pass-style deployment, and garrison scoring that includes ships, troops, and facilities.

An “augmentation mode” is a config mode where carefully isolated anti-stagnation helpers can be enabled. Examples are target-repeat penalties, research auto-assignment, or retarget cooldowns.

A “seed batch” is the set of seeds scored together for one candidate config. This plan assumes three seeds per batch by default, because a single deterministic seed is too brittle and the current simulation is already fast enough to afford three.

A “control change” means a system moved from one control state to another, for example `Alliance -> Empire`, `Empire -> Alliance`, `Controlled -> Contested`, or `Contested -> Controlled`. This is distinct from simply counting how many systems each faction controls at the end.

## Plan of Work

Start by expanding the telemetry model in `crates/rebellion-core/src/game_events.rs` and `crates/rebellion-data/src/simulation.rs`. The goal is not “more logging everywhere.” The goal is to log the smallest set of events and summary facts that make balance diagnosis possible. Add dedicated telemetry for system control changes, battle start context, fleet-order context, mission dispatch context, research progress, and uprising outcomes. Also add a per-run accumulator that produces a structured summary JSON file without needing to re-read the JSONL after the run finishes.

The new telemetry should answer the missing questions directly. System control should be visible in two ways: first by an event such as `system_control_changed` with `system`, `from_control`, `to_control`, `cause`, `tick`, and the loyalty or popularity context that caused the flip; second by a coarse `campaign_snapshot` every 250 ticks that records control counts, contested count, active fleets, in-transit fleets, active blockades, active uprisings, active missions, and research levels. Fleet strength should be visible at the moments that matter, not every tick: when a move order is issued, when a fleet arrives, and when space combat begins. The relevant fields are attacker and defender total hull count, fighter count, troop count, facility count at the target system, and the ratio or margin that drove the move decision. Mission diversity and character utilization should become measurable by adding `faction`, `character`, `character_name`, `skill_used`, `predicted_success`, and `target_system` to `mission_dispatched` and `mission_resolved`. Research should emit `research_started`, `research_progress_snapshot`, and `research_unlocked`, or at minimum maintain the same information in the summary sidecar. Uprisings should log incident, began, subdued, and whether they caused control to flip.

While doing that telemetry pass, clean up the event payloads that are still too opaque for scoring. `manufacturing_complete` currently writes `kind` as a Rust `Debug` string such as `CapitalShip(CapitalShipKey(12v1))`. Replace this with stable structured fields such as `kind_family`, `class_name`, `class_key`, `system`, and `faction`. Do the same for move orders and combat events so the evaluator never has to parse human text.

Next, introduce a JSON tuning schema. Create a new data-only module such as `crates/rebellion-core/src/tuning.rs` and export it from `crates/rebellion-core/src/lib.rs`. Keep it `serde`-serializable so the headless binary can load it directly. Add a default config file at `configs/autoresearch/default.json`. The schema should look like this at a high level:

```json
{
  "runtime": {
    "ticks": 5000,
    "seeds": [41, 42, 43],
    "snapshot_interval_ticks": 250,
    "binary_path": "target/release/rebellion-playtest"
  },
  "telemetry": {
    "emit_jsonl": true,
    "emit_summary_json": true,
    "summary_only": false,
    "log_control_changes": true,
    "log_fleet_strength": true,
    "log_character_utilization": true
  },
  "ai": {
    "tick_interval": 7,
    "diplomacy_skill_threshold": 60,
    "diplomacy_target_popularity_cap": 0.8,
    "espionage_skill_threshold": 50,
    "covert_min_success_prob": 0.30,
    "max_covert_ops_per_eval": 3,
    "attack_reinforce_ratio_target": 0.35,
    "hq_garrison_threshold": 1.0,
    "frontier_garrison_threshold": 0.8,
    "retarget_cooldown_ticks": 21
  },
  "movement": {
    "distance_scale": 2,
    "min_transit_ticks": 10
  },
  "production": {
    "max_construction_yards": 5,
    "priority_weights": {
      "capital_ship": 1.0,
      "fighter": 0.8,
      "manufacturing_facility": 0.6,
      "production_facility": 0.5,
      "troop": 0.4,
      "defense_facility": 0.4
    }
  },
  "research": {
    "enabled": true,
    "min_skill_threshold": 40,
    "priority_weights": {
      "ship": 1.0,
      "troop": 0.6,
      "facility": 0.5
    }
  },
  "scoring": {
    "victory_tick_min": 2000,
    "victory_tick_max": 4500,
    "combat_pct_min": 0.05,
    "combat_pct_max": 0.15,
    "manufacturing_complete_min": 20,
    "control_changes_min": 5,
    "control_changes_max": 25,
    "faction_min_activity_share": 0.35
  },
  "parity": {
    "omniscient_ai": true,
    "distributed_pipeline": true,
    "count_ships_troops_facilities_in_garrison": true,
    "two_pass_deployment": true
  },
  "augmentation": {
    "enabled": true,
    "battle_repeat_penalty": 0.25,
    "force_research_after_idle_ticks": 250,
    "attack_bias_floor": 0.15
  },
  "data_overrides": {
    "gnprtb": {},
    "mission_tables": {},
    "uprising_tables": {}
  }
}
```

Not every field above must become active on day one, but the file should reserve the shape. The important design rule is that “current hardcoded constant” becomes “default value in the config,” while “original DAT table” stays in data and is only overridden explicitly by ID or by overlay file.

After the schema exists, thread it through the simulation. In `crates/rebellion-core/src/ai.rs`, add config-aware entry points such as `AISystem::advance_with_tuning(...)` and keep the current public `advance(...)` as a wrapper that uses defaults so the rest of the game does not break. Do the same in `crates/rebellion-core/src/movement.rs` with a `fleet_transit_ticks_with_tuning(...)` helper. In `crates/rebellion-playtest/src/main.rs`, add `--config <path>` and `--summary-json <path>`. The headless binary should be able to run with no config and preserve current behavior, or run with a config and change only the requested knobs.

Then do the parity pass before the augmentation pass. The parity pass belongs mainly in `crates/rebellion-core/src/ai.rs`, using `ghidra/notes/ai-behavior-analysis.md` as the acceptance anchor. The AI should explicitly evaluate the full galaxy, bucket systems strategically, score garrisons using ships plus troops plus facilities, validate candidate actions, and then issue movement or assignment decisions in a predictable distributed order. This is where the project becomes faithful to the original omniscient six-stage pipeline. Once that baseline exists, add only the weakest known missing pieces under `augmentation`. Examples are a penalty for repeating the same battle at the same system forever, a cooldown on retargeting fleets already in transit, and an optional research auto-assignment rule when both factions have gone too long without tech progress. These must be disableable so a parity-only run remains possible.

At the same time, add research policy rather than only research logging. The current simulation can score research progress only if some actor actually dispatches research. If reverse-engineered original research-assignment logic is not yet available, implement a minimal, explicit research policy under a clearly labeled config section and note that it is an augmentation until parity evidence improves. The policy should choose one project per faction based on skill threshold and per-tree weights, and it should emit clean telemetry for starts, progress, and unlocks.

Once telemetry and tuning are in place, rewrite `scripts/eval_game_quality.py` around the new run summary. Keep the JSONL parser as a fallback, but make the preferred input the summary sidecar. Fix faction-balance measurement by using explicit faction fields. Add new sub-scores and hard-fail checks for control changes, mission diversity, research progress, uprising frequency, character utilization, and battle spread. The score should explicitly penalize “all combat at one system,” “no research,” “too few manufacturing completions,” and “no meaningful control movement,” even if the raw combat percentage looks ideal. The target ranges supplied by the user should live in the config so the scorer and the loop read the same thresholds.

Finally, add the actual loop runner. A new script such as `scripts/autoresearch_loop.py` should load `configs/autoresearch/default.json`, spawn the prebuilt binary once per seed, write each seed’s JSONL and summary to a temporary iteration directory such as `/tmp/open-rebellion-autoresearch/<iteration>/<seed>/`, invoke `scripts/eval_game_quality.py`, aggregate the three seed scores with a robust statistic such as median, and then keep or discard the candidate config. Build the binary before the loop starts, not during the loop. The loop should print one short per-iteration report containing wall time, per-seed score, aggregated score, hard-fail reasons if any, and whether the candidate replaced the incumbent. This is the artifact the user actually needs for autoresearch, not just more telemetry.

## Concrete Steps

All commands in this section assume the working directory is `/Users/tomdimino/Desktop/Programming/open-rebellion`.

1. Reproduce the current baseline with the already-built debug binary rather than `cargo run`.

```bash
./target/debug/rebellion-playtest data/base --seed 42 --ticks 5000 --dual-ai --summary
```

Expected current output before implementation: `Total events: 11991`, `combat_space 991`, `fleet_arrived 677`, `manufacturing_complete 6`, `Fleet attack orders: 10`, `Fleet reinforce orders: 1348`, `victory: false`.

2. Confirm the seed-invariance problem.

```bash
./target/debug/rebellion-playtest data/base --seed 41 --ticks 5000 --dual-ai --summary
./target/debug/rebellion-playtest data/base --seed 42 --ticks 5000 --dual-ai --summary
./target/debug/rebellion-playtest data/base --seed 43 --ticks 5000 --dual-ai --summary
```

Expected current output before implementation: the three runs differ only slightly in total event count and mission totals, while combat count, fleet arrivals, manufacturing completions, control counts, and attack-versus-reinforce counts remain effectively identical.

3. Inspect the current telemetry, scoring, and AI tuning surfaces before editing.

```bash
sed -n '1,220p' crates/rebellion-playtest/src/main.rs
sed -n '1,220p' crates/rebellion-playtest/src/logger.rs
sed -n '1,220p' crates/rebellion-core/src/game_events.rs
sed -n '1,220p' crates/rebellion-data/src/simulation.rs
sed -n '1,220p' crates/rebellion-core/src/ai.rs
sed -n '1,140p' crates/rebellion-core/src/movement.rs
sed -n '1,220p' scripts/eval_game_quality.py
```

Expected current output before implementation: hardcoded AI and movement constants, no config loader, no summary sidecar, and no scorer awareness of control changes, research progress, mission diversity, or battle concentration.

4. Implement the telemetry pass, then run formatting and focused tests.

```bash
cargo fmt
cargo test -p rebellion-core ai movement research uprising
cargo test -p rebellion-data simulation
cargo test -p rebellion-playtest
```

Expected output after implementation: the new telemetry types compile cleanly, tests cover control-change logging, fleet-strength summaries, mission payload identity fields, and summary sidecar generation.

5. Build the release binary once and verify config-driven execution.

```bash
cargo build -p rebellion-playtest --release
./target/release/rebellion-playtest data/base \
  --seed 42 --ticks 5000 --dual-ai \
  --config configs/autoresearch/default.json \
  --output /tmp/seed42.jsonl \
  --summary-json /tmp/seed42.summary.json
```

Expected output after implementation: the run succeeds without recompiling inside the loop, `/tmp/seed42.jsonl` is valid JSONL, and `/tmp/seed42.summary.json` contains aggregated fields such as `control_changes`, `combat_system_concentration`, `mission_kind_counts`, `character_utilization`, `research_levels`, `research_unlocks`, and `uprising_counts`.

6. Verify that the evaluator uses the richer summary and reports the new failure modes.

```bash
python3 scripts/eval_game_quality.py /tmp/seed42.summary.json --json
```

Expected output after implementation: JSON containing the total score, individual sub-scores, hard-fail reasons, and explicit fields for `victory_tick`, `combat_pct`, `manufacturing_complete`, `control_changes`, `mission_diversity`, `research_progress`, `uprising_frequency`, `battle_spread`, and per-faction activity.

7. Run one 3-seed autoresearch batch and time it.

```bash
python3 scripts/autoresearch_loop.py \
  --config configs/autoresearch/default.json \
  --iterations 1 \
  --workdir /tmp/open-rebellion-autoresearch
```

Expected output after implementation: a single iteration directory containing three seed subdirectories, one aggregated iteration summary, a keep-or-discard decision, and a total batch wall time well under the 300-second ceiling required for 12 iterations per hour.

8. Validate the target ranges on the default augmented config.

```bash
python3 scripts/autoresearch_loop.py \
  --config configs/autoresearch/default.json \
  --iterations 12 \
  --workdir /tmp/open-rebellion-autoresearch
```

Expected output after implementation: 12 completed iterations, each using 3 seeds, with either accepted configs that move the game toward the target ranges or explicit reject reasons such as `victory_too_late`, `manufacturing_too_low`, `control_changes_too_low`, `combat_concentrated`, or `faction_activity_unbalanced`.

## Validation and Acceptance

This work is accepted only when the headless pipeline becomes genuinely useful for balance research rather than merely verbose.

A single seed run must expose enough information to diagnose game balance. The summary JSON must include the number of control changes, the list or count of systems that changed control, battle concentration by system, move-order reasons and strength comparisons, mission-kind counts, per-character utilization counts or busy ticks, research state by faction, and uprising counts. It must be possible to determine whether a run stagnated because fleets never left home, because battles clustered at one choke point, because both factions stopped researching, or because diplomacy and uprisings never moved territory.

The config surface must be complete enough for real autoresearch work. At minimum the JSON file must cover `AI_TICK_INTERVAL`, `DISTANCE_SCALE`, `MIN_TRANSIT_TICKS`, production priorities, attack-versus-reinforce policy, garrison thresholds, diplomacy thresholds, telemetry controls, scoring targets, and parity-versus-augmentation flags. Running without `--config` must preserve current defaults so the interactive game is not broken by this work.

Original-faithful behavior must be an explicit supported mode. With augmentation disabled, the AI should still be omniscient, rule-based, and globally evaluative, and its garrison assessment must account for ships, troops, and facilities. The implementation does not need to claim perfect parity with every original function, but it must visibly align with the six-stage pipeline documented in `ghidra/notes/ai-behavior-analysis.md`.

The loop must be operationally fast enough. A 3-seed iteration using the prebuilt release binary must finish within 300 seconds on the target machine. Given the current measured runtime of roughly one second per seed with the debug binary, a properly built release loop should be far below this threshold. The loop must not invoke `cargo run` per seed.

The default augmented config should either hit or intentionally score against these target ranges: victory tick between 2000 and 4500, combat between 5 percent and 15 percent of all events, at least 20 manufacturing completions, between 5 and 25 system control changes, and meaningful activity from both factions. If the default config misses these targets, the evaluator must name the specific miss rather than hiding it inside one blended score.

## Idempotence and Recovery

The implementation steps in this plan are safe to repeat if they are done carefully. Telemetry additions are additive. Re-running `cargo fmt`, the test suites, and deterministic seed runs should not change game state beyond overwriting generated output files.

The autoresearch loop should write to a disposable work directory such as `/tmp/open-rebellion-autoresearch`. Deleting that directory must fully reset loop outputs. The chosen config file in `configs/autoresearch/` should remain the only durable human-edited input.

If the telemetry becomes too noisy, recover by reducing snapshot frequency and trimming redundant event fields before deleting whole telemetry categories. The highest-priority telemetry to keep is control changes, battle context, mission identity, research progress, and per-faction activity.

If parity and augmentation start fighting each other, recover by forcing `augmentation.enabled = false` and validating the parity baseline first. Only then re-enable one augmentation at a time.

If the loop misses the throughput target, recover in this order: stop using `cargo run`, remove unnecessary JSON reparsing by leaning on the summary sidecar, reduce snapshot frequency, and only then consider parallel seed execution.

## Interfaces and Dependencies

At completion, the codebase should expose a data-only tuning type that can be deserialized from JSON. A concrete interface such as `AutoresearchConfig`, plus subtypes like `RuntimeTuning`, `TelemetryTuning`, `AiTuning`, `MovementTuning`, `ProductionTuning`, `ResearchTuning`, `ScoringTargets`, `ParityTuning`, and `AugmentationTuning`, should exist in a central Rust module such as `crates/rebellion-core/src/tuning.rs`.

The AI and movement systems must accept tuning explicitly. The intended shape is a config-aware `AISystem::advance_with_tuning(...) -> Vec<AIAction>` alongside the existing defaulting wrapper, and a config-aware `fleet_transit_ticks_with_tuning(...) -> u32` alongside the current defaulting helper. This keeps the main app stable while letting playtest and autoresearch override behavior.

The simulation layer should own a structured run summary. A type such as `CampaignMetrics` or `RunSummary` should capture per-seed aggregate outputs including control changes, control snapshots, battle spread, move-order totals, mission diversity, character utilization, manufacturing counts, research state, uprising counts, victory timing, and total wall time. `crates/rebellion-data/src/simulation.rs` should populate it while emitting JSONL.

The playtest binary should grow two file-oriented interfaces: `--config <path>` to load tuning, and `--summary-json <path>` to write the aggregated summary sidecar. The existing `--output` JSONL path should remain.

The evaluator should expose a stable function contract such as `evaluate_run(summary: dict, targets: dict) -> dict` even if the CLI stays simple. The autoresearch runner should read that result and make its keep-or-discard decision from aggregated 3-seed statistics rather than from a single run.

No heavy new dependencies are required. Rust can stay on the existing workspace crates plus `serde` and `serde_json`, which are already present. Python should stay in the standard library. If seed parallelism is later needed, use `concurrent.futures` from the standard library rather than adding external orchestration packages.
