# Strengthen Rust Test, Lint, Type-Check, and Build Gates

This ExecPlan is a living document. Sections Progress, Surprises &
Discoveries, Decision Log, and Outcomes & Retrospective must be
kept up to date as work proceeds.

## Purpose / Big Picture

Open Rebellion already has a large amount of Rust test coverage, a working headless playtest binary, a working WebAssembly build script, and GitHub Actions. What it does not yet have is an enforced, shared quality gate that makes formatting drift, warning drift, parity drift, smoke regressions, and build regressions visible immediately. After this plan is implemented, a contributor should be able to run one local command set and get the same answer that CI gets: formatting is clean, linting is clean, the workspace type-checks, the workspace tests pass, the short playtest smoke run proves the simulation still behaves like a game, the long parity oracle reports whether the implementation still matches the captured golden values, coverage is measured and published, and the browser packaging path is verified by the same script the release process uses.

The visible proof is straightforward. A pull request should show separate green checks for format, clippy, tests, smoke playtest, coverage collection, and WebAssembly packaging. Longer-running parity checks should either be green and blocking or explicitly marked informational until the current baseline is ratcheted into an accepted target. A novice should be able to reproduce every gate locally with the commands in this document.

## Progress

- [x] (2026-04-07 02:10Z) Read the workspace manifests, `rustfmt.toml`, `clippy.toml`, the CI workflow, the WebAssembly build script, and the evaluation scripts.
- [x] (2026-04-07 02:18Z) Enumerated the current test inventory with `PATH="/usr/bin:$PATH" cargo test --workspace -- --list`.
- [x] (2026-04-07 02:24Z) Measured the current baseline for formatting, clippy, WebAssembly packaging, a 100-tick playtest smoke run, and a 5000-tick parity candidate run.
- [x] (2026-04-07 02:31Z) Wrote this ExecPlan with milestone order, effort estimates, dependencies, validation commands, and acceptance criteria.
- [ ] Milestone 1: Clean the existing formatting drift and warning debt until the repository can support enforced gates.
- [ ] Milestone 2: Make CI and local hooks use one shared set of quality commands.
- [ ] Milestone 3: Add coverage measurement and publish reports without creating flaky gates.
- [ ] Milestone 4: Add deterministic smoke and parity playtest jobs to CI with the right blocking policy.
- [ ] Milestone 5: Expand missing test surfaces with integration, property-based, and fuzz testing.

## Surprises & Discoveries

- Observation: The repository is not currently warning-free, even though project documentation describes a zero-warning policy.
  Evidence: `PATH="/usr/bin:$PATH" cargo test --workspace -- --list` emitted warnings from `rebellion-core`, `rebellion-data`, `rebellion-render`, and `rebellion-app`.

- Observation: Formatting is not currently enforced and the repository is not rustfmt-clean.
  Evidence: `cargo fmt --all --check` exited with code 1 and printed large diffs across `crates/rebellion-app/` and `tools/dat-dumper/`.

- Observation: Clippy configuration exists, but the repository is far from being able to enforce `-D warnings`.
  Evidence: `PATH="/usr/bin:$PATH" cargo clippy --workspace --all-targets -- -D warnings` failed with many findings in `tools/dat-dumper/` and `crates/rebellion-core/`, including `new_without_default`, `too_many_arguments`, `doc_lazy_continuation`, `field_reassign_with_default`, and the repo-specific `disallowed-names` rule.

- Observation: The test surface is broad in unit tests but narrow in integration tests.
  Evidence: `cargo test --workspace -- --list` reported 322 tests in `rebellion-core`, 50 unit tests in `rebellion-data`, 35 tests in `rebellion-render`, and zero tests in `rebellion-app`, `rebellion-playtest`, and `dat-dumper`. `find crates -maxdepth 2 -type d -name tests` found only `crates/rebellion-data/tests`.

- Observation: One data-backed integration test already exists and is intentionally excluded from the default developer path.
  Evidence: `crates/rebellion-data/tests/telemetry_coverage.rs` is marked `#[ignore]` and requires `data/base/`.

- Observation: The current GitHub Actions workflow verifies a raw WASM compile, not the actual packaging script used by developers.
  Evidence: `.github/workflows/ci.yml` runs `cargo build --target wasm32-unknown-unknown -p rebellion-app --release`, while the repository’s end-to-end packaging path is `scripts/build-wasm.sh`.

- Observation: The packaging script is healthy enough to use as a CI source of truth.
  Evidence: `bash scripts/build-wasm.sh` exited with code 0, copied `web/open-rebellion.wasm`, verified vendored `web/gl.js`, staged 51 DAT files, extracted `textstra.json`, and staged 2231 UI BMP files.

- Observation: A 100-tick deterministic playtest is fast and rich enough for a smoke gate, but it is too short for the existing quality and parity evaluators.
  Evidence: `cargo run -q -p rebellion-playtest -- data/base --seed 42 --ticks 100 --dual-ai --output /tmp/open-rebellion-smoke-100.jsonl --summary` finished in about 0.2s and produced 4742 events across 20 event types, but `python3 scripts/eval_game_quality.py /tmp/open-rebellion-smoke-100.jsonl --json` returned `degenerate: true` because there was no combat, and `python3 scripts/eval_parity.py /tmp/open-rebellion-smoke-100.jsonl --json` failed several long-horizon checks.

- Observation: The 5000-tick parity candidate run is practical for CI, but the current oracle is not yet green.
  Evidence: `cargo run -q -p rebellion-playtest -- data/base --seed 42 --ticks 5000 --dual-ai --output /tmp/open-rebellion-parity-5000.jsonl` finished in about 15.4s and produced 1,700,567 events. `python3 scripts/eval_parity.py /tmp/open-rebellion-parity-5000.jsonl --json` failed on AI interval, Death Star event presence, and research completion event IDs. `python3 scripts/eval_game_quality.py /tmp/open-rebellion-parity-5000.jsonl --json` returned `degenerate: false` with score `0.3075`.

- Observation: The documented older caution about `cargo test --workspace` doctests is now stale.
  Evidence: `cargo test --workspace -- --list` completed successfully and listed 19 doctests across `rebellion-core`, `rebellion-data`, and `rebellion-render`.

## Decision Log

- Decision: Use `cargo llvm-cov`, not Tarpaulin, for coverage.
  Rationale: Tarpaulin is primarily Linux-only and ptrace-based. This repository is actively developed on macOS, already builds on both macOS and Ubuntu in CI, and needs a workspace-friendly tool that can emit HTML and LCOV without forcing Linux-only local workflows.
  Date/Author: 2026-04-07 / Codex

- Decision: Make `scripts/build-wasm.sh` the CI source of truth for the browser path.
  Rationale: A direct `cargo build --target wasm32-unknown-unknown` verifies compilation only. The script also verifies the vendored loader, the `.wasm` artifact name, DAT staging, `textstra.json` extraction, and UI asset staging.
  Date/Author: 2026-04-07 / Codex

- Decision: Add a dedicated short smoke validator instead of reusing `scripts/eval_game_quality.py` for 100-tick CI runs.
  Rationale: The current game-quality evaluator is intentionally balance-oriented and treats “no combat yet” as degenerate. That is correct for a long-quality run and incorrect for a short boot-and-simulate smoke run.
  Date/Author: 2026-04-07 / Codex

- Decision: Introduce the parity oracle in CI as informational first, then make it blocking only after the current deterministic baseline is accepted or repaired.
  Rationale: The current 5000-tick seed-42 run is reproducible and fast enough, but it fails the oracle today. Turning it on as a blocking PR gate immediately would create noise instead of trust.
  Date/Author: 2026-04-07 / Codex

- Decision: Use repo-owned Git hooks under `.githooks/` and a shared shell script instead of adopting a new hook framework first.
  Rationale: The repository currently has no `pre-commit` framework, no Python tooling requirement for contributors, and no `Justfile` or `Makefile`. Repo-owned hooks plus one shared script keep the local workflow transparent and dependency-light.
  Date/Author: 2026-04-07 / Codex

- Decision: Treat coverage as a reporting and ratcheting system, not a blind absolute threshold on day one.
  Rationale: The current baseline has never been measured. A fixed threshold chosen before measurement would be guesswork and would likely fail for reasons unrelated to regression risk.
  Date/Author: 2026-04-07 / Codex

## Outcomes & Retrospective

Planning is complete and implementation is intentionally still pending. The audit shows that Open Rebellion is in a better place than the missing enforcement suggests: the workspace already has substantial unit coverage, the playtest binary is deterministic and fast enough for CI, the browser packaging script works, and the parity and quality scripts are already shaped like useful oracles. The missing pieces are mostly enforcement, command unification, and the addition of a few targeted test modalities around the crates that currently rely on manual confidence.

The most important lesson is that the repository does not need a giant tooling rewrite. It needs a baseline cleanup, a single source of truth for commands, and a careful split between fast blocking gates and slower informational gates. The parity oracle is valuable, but it should not be made mandatory until its current failures are either fixed or consciously encoded as the new expected truth.

## Context and Orientation

“Type-check” means compiling the workspace without running tests so the compiler verifies types and trait bounds. In Rust, that is usually `cargo check`. “Smoke test” means a short end-to-end run whose job is to prove the system starts, executes meaningful work, and emits evidence that key subsystems fired. “Coverage” means measuring which lines or regions were executed by tests. “Property-based testing” means generating many legal random inputs to verify an invariant, such as “movement progress never exceeds 100%.” “Fuzz testing” means feeding mutated or malformed bytes into parsers and loaders to catch panics, crashes, and memory-safety-adjacent logic bugs. “Golden-value oracle” means the script `scripts/eval_parity.py`, which compares a deterministic playtest log against reverse-engineered expected values stored in `scripts/golden_values.json`.

The current quality infrastructure is spread across a small number of important files. `Cargo.toml` defines the six-crate workspace and a small `workspace.lints.clippy` policy. `rustfmt.toml` and `clippy.toml` already exist, which means the repository has chosen tools but has not yet turned them into enforced gates. `.github/workflows/ci.yml` defines the present GitHub Actions policy. `scripts/build-wasm.sh` is the true browser-packaging path. `scripts/eval_game_quality.py` and `scripts/eval_parity.py` are the current evaluation oracles for long headless runs. `crates/rebellion-playtest/src/main.rs` and `crates/rebellion-playtest/src/logger.rs` define the headless simulation harness and summary output. `crates/rebellion-data/tests/telemetry_coverage.rs` is the only existing integration-test directory in the workspace.

The current measured test inventory, from `cargo test --workspace -- --list`, is as follows. `crates/rebellion-core` has 322 unit tests. `crates/rebellion-data` has 50 unit tests and 1 ignored integration test. `crates/rebellion-render` has 35 unit tests. `crates/rebellion-app`, `crates/rebellion-playtest`, and `tools/dat-dumper` each currently have zero tests. The workspace also has 19 doctests. This means the repository already has meaningful depth in engine logic, but thin coverage in the top-level app, the headless binary, and the binary/data-tool boundaries where integration mistakes often hide.

## Plan of Work

Milestone 1 establishes a clean and enforceable baseline. Estimated effort is 1 to 2 days. It depends on no earlier implementation work, but it must finish before format and clippy enforcement can be made blocking. The work in this milestone is deliberately mechanical: fix the current rustfmt drift, remove current compiler warnings, decide whether every current clippy finding should be fixed or locally allowed with a comment, and make sure the repository has one explicit type-check command that includes all relevant targets. The files most likely to change are `Cargo.toml`, `rustfmt.toml`, `clippy.toml`, and the currently warning-heavy Rust sources under `crates/rebellion-core/`, `crates/rebellion-data/`, `crates/rebellion-render/`, `crates/rebellion-app/`, and `tools/dat-dumper/`.

Milestone 2 turns those commands into the required fast gates for both CI and local developers. Estimated effort is 0.5 to 1 day. It depends on Milestone 1 because broken baselines make enforcement useless. Update `.github/workflows/ci.yml` so the test job becomes several explicit jobs: format, type-check, clippy, tests, WebAssembly packaging, and playtest smoke. Add a small shared script such as `scripts/quality-gate.sh` so local and CI invocations do not drift apart. Add repo-owned hooks in `.githooks/pre-commit` and `.githooks/pre-push`, plus a small setup helper such as `scripts/setup-git-hooks.sh`, so a novice can enable the same policy with one command.

Milestone 3 adds coverage measurement without making day-to-day development brittle. Estimated effort is 0.5 to 1 day. It depends on Milestone 2 because the test command must already be stable. Add `cargo llvm-cov` installation in CI, probably on Ubuntu for speed and simplicity, emit both HTML and LCOV artifacts, and publish them as workflow artifacts. In the first pass, make the job non-blocking except for tool or command failure. Once a baseline exists, add a ratchet rule such as “overall line coverage must not drop relative to the checked-in baseline” or “`rebellion-core` and `rebellion-data` must not drop by more than one percentage point.”

Milestone 4 integrates the two headless simulation gates. Estimated effort is 1 to 1.5 days. It depends on Milestone 2 for command sharing, but it can run in parallel with coverage once the shared scripts exist. Add a new script such as `scripts/check_playtest_smoke.py` that validates a short deterministic log. This script should check that the log is non-empty, reaches tick 100, has several distinct event types, and includes at least one `ai_action`, one `economy_tick`, and one `fleet_arrived`. Then add the long deterministic parity job that runs the existing `scripts/eval_parity.py` on a 5000-tick seed-42 log. Because the current baseline fails, make this job informational at first. Save the JSON report as an artifact and, if useful, summarize failures in the GitHub Actions job summary so a reviewer can see exactly which golden checks changed.

Milestone 5 expands the missing test surfaces. Estimated effort is 2 to 5 days, but it is intentionally incremental and can continue after the core gates are already protecting the repository. It depends only loosely on earlier milestones. Add integration tests for `rebellion-playtest` and `rebellion-app` at the boundary where commands, assets, and data loading meet. Add property-based tests to engine crates such as `rebellion-core` and `rebellion-data` for movement, economy invariants, save/load round-trips, and event serialization. Add fuzz targets for the DAT parsing and save/mod loading paths, especially in `tools/dat-dumper` and the loaders under `crates/rebellion-data/`. This is where the repository gets the largest confidence increase for malformed input and unanticipated state combinations.

The recommended priority order by confidence per hour is deliberate. First, clean the baseline and enforce rustfmt, type-checking, and warnings because these are cheap, always-on regressions. Second, make CI run `scripts/build-wasm.sh` instead of a raw target build, because that closes a real browser-packaging gap with little effort. Third, add the 100-tick smoke playtest because it is fast, deterministic, and exercises the full simulation stack. Fourth, add coverage reporting because visibility helps direct future test work even before thresholds are enforced. Fifth, wire in the 5000-tick parity oracle as informational, then make it blocking only after the baseline is accepted. Sixth, spend deeper effort on property-based and fuzz testing, which are high value but less “confidence per hour” than the missing enforcement steps.

## Concrete Steps

1. Audit and record the current baseline from the repository root.
   Working directory: `/Users/tomdimino/Desktop/Programming/open-rebellion`
   Commands:
   `PATH="/usr/bin:$PATH" cargo test --workspace -- --list`
   `cargo fmt --all --check`
   `PATH="/usr/bin:$PATH" cargo clippy --workspace --all-targets -- -D warnings`
   `bash scripts/build-wasm.sh`
   Expected output today: the test listing succeeds and shows 322 `rebellion-core` tests, 50 `rebellion-data` unit tests plus 1 ignored integration test, 35 `rebellion-render` tests, and zero tests in `rebellion-app`, `rebellion-playtest`, and `dat-dumper`; rustfmt fails; clippy fails; the WebAssembly build script succeeds.

2. Clean the formatting and warning baseline.
   Working directory: `/Users/tomdimino/Desktop/Programming/open-rebellion`
   Commands:
   `cargo fmt --all`
   `PATH="/usr/bin:$PATH" cargo check --workspace --all-targets`
   `PATH="/usr/bin:$PATH" cargo clippy --workspace --all-targets -- -D warnings`
   Expected output after Milestone 1: all three commands exit with code 0. `cargo check` and `cargo clippy` must produce no warnings because the repository wants a true zero-warning policy rather than a manual convention.

3. Introduce one shared command script and local hooks.
   Working directory: `/Users/tomdimino/Desktop/Programming/open-rebellion`
   Commands:
   `git config core.hooksPath .githooks`
   `.githooks/pre-commit`
   `.githooks/pre-push`
   Expected output after Milestone 2: the pre-commit hook runs `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings`; the pre-push hook runs `PATH="/usr/bin:$PATH" cargo test --workspace`. If the team insists on tests in pre-commit instead of pre-push, that policy should still call the same shared script so CI and local hooks stay aligned.

4. Update GitHub Actions to use explicit quality jobs.
   Working directory: `/Users/tomdimino/Desktop/Programming/open-rebellion`
   Commands:
   `gh workflow view` is not required; edit `.github/workflows/ci.yml` directly and validate with local command parity.
   `PATH="/usr/bin:$PATH" cargo check --workspace --all-targets`
   `PATH="/usr/bin:$PATH" cargo test --workspace`
   `bash scripts/build-wasm.sh`
   Expected output after Milestone 2: each local command matches a named CI job. CI should no longer have a hidden difference where browser packaging in developer docs is stronger than the CI browser build check.

5. Add and validate the short playtest smoke script.
   Working directory: `/Users/tomdimino/Desktop/Programming/open-rebellion`
   Commands:
   `PATH="/usr/bin:$PATH" cargo run -q -p rebellion-playtest -- data/base --seed 42 --ticks 100 --dual-ai --output /tmp/open-rebellion-smoke-100.jsonl`
   `python3 scripts/check_playtest_smoke.py /tmp/open-rebellion-smoke-100.jsonl`
   Expected output after Milestone 4: the checker exits with code 0 and prints a short summary such as `final_tick=100`, `total_events>=100`, `distinct_event_types>=5`, and confirmation that `ai_action`, `economy_tick`, and `fleet_arrived` were present.

6. Add and validate the long parity oracle job.
   Working directory: `/Users/tomdimino/Desktop/Programming/open-rebellion`
   Commands:
   `PATH="/usr/bin:$PATH" cargo run -q -p rebellion-playtest -- data/base --seed 42 --ticks 5000 --dual-ai --output /tmp/open-rebellion-parity-5000.jsonl`
   `python3 scripts/eval_parity.py /tmp/open-rebellion-parity-5000.jsonl --json`
   Expected output after Milestone 4 first pass: the job exits successfully as a workflow step, publishes the JSON report, and marks the run informational if the oracle still reports `overall: fail`. Expected output after the baseline is accepted or repaired: the command exits with code 0 and CI treats failures as blocking.

7. Add and validate coverage collection.
   Working directory: `/Users/tomdimino/Desktop/Programming/open-rebellion`
   Commands:
   `cargo llvm-cov --workspace --lcov --output-path target/llvm-cov/lcov.info`
   `cargo llvm-cov --workspace --html --output-dir target/llvm-cov/html`
   Expected output after Milestone 3: the coverage job exits with code 0, writes `target/llvm-cov/lcov.info`, writes HTML output, and uploads both as CI artifacts. The first pass should report the measured baseline without failing on an arbitrary threshold.

8. Add the missing deeper test modalities in small slices.
   Working directory: `/Users/tomdimino/Desktop/Programming/open-rebellion`
   Commands:
   `PATH="/usr/bin:$PATH" cargo test -p rebellion-playtest`
   `PATH="/usr/bin:$PATH" cargo test -p rebellion-app`
   `PATH="/usr/bin:$PATH" cargo test -p rebellion-core`
   `cargo fuzz run <target-name>` once fuzz targets exist
   Expected output after Milestone 5: `rebellion-playtest` and `rebellion-app` are no longer at zero tests, at least one property-based test module exists in a core crate, and at least one fuzz target exists for a binary or data loader boundary.

## Validation and Acceptance

Acceptance for Milestone 1 is that `cargo fmt --all --check`, `PATH="/usr/bin:$PATH" cargo check --workspace --all-targets`, and `PATH="/usr/bin:$PATH" cargo clippy --workspace --all-targets -- -D warnings` all exit with code 0 on a clean checkout. This milestone is not complete until the repository can honestly claim “zero warnings” as an enforced fact rather than a manual convention.

Acceptance for Milestone 2 is that `.github/workflows/ci.yml` contains explicit jobs for format, type-check, clippy, tests, and WebAssembly packaging, and that each job runs the same commands that local developers are instructed to run. This milestone is not complete until a novice can enable the hooks with `git config core.hooksPath .githooks` and see the same policy locally.

Acceptance for Milestone 3 is that a CI run uploads coverage artifacts and prints a measured baseline. A later ratcheting change is acceptable, but the first coverage milestone is not complete until the measurement is reproducible and visible.

Acceptance for Milestone 4 is split in two. The short smoke gate is complete when the 100-tick run is deterministic and the custom smoke script passes in CI. The long parity gate is complete in its first phase when the job runs in CI, publishes its JSON result, and summarizes failures without blocking unrelated pull requests. The parity gate is complete in its second phase when the baseline is green and blocking.

Acceptance for Milestone 5 is that at least one new integration test exists for a currently untested crate boundary, at least one property-based test exists for a nontrivial invariant, and at least one fuzz target exists for a parser or loader that consumes untrusted bytes.

The full repository-level acceptance gate, once all milestones are complete, is this sequence from the repository root: `cargo fmt --all --check`; `PATH="/usr/bin:$PATH" cargo check --workspace --all-targets`; `PATH="/usr/bin:$PATH" cargo clippy --workspace --all-targets -- -D warnings`; `PATH="/usr/bin:$PATH" cargo test --workspace`; `bash scripts/build-wasm.sh`; the 100-tick playtest smoke command plus its checker; the 5000-tick parity command plus `scripts/eval_parity.py`; and `cargo llvm-cov --workspace`. Every command should either pass or fail with a clear, intentional reason.

## Idempotence and Recovery

Every verification command in this plan is safe to rerun. `cargo fmt --all --check`, `cargo check`, `cargo clippy`, `cargo test`, and the playtest evaluation scripts are idempotent. `bash scripts/build-wasm.sh` is also repeatable, but it writes generated artifacts into `web/`, including `web/open-rebellion.wasm`, staged DAT files, and staged UI assets. The plan should treat those outputs as generated build products and should document clearly whether they are meant to stay tracked or remain CI artifacts only.

Recovery should be handled conservatively. Temporary playtest logs belong in `/tmp/` and can be deleted freely. Coverage artifacts belong under `target/` and can be deleted freely. Generated browser artifacts under `web/` should only be restored or deleted after confirming they were produced by a local verification run and not intentionally edited by a user. Because this repository may already have a dirty worktree, do not use destructive rollback commands blindly.

## Interfaces and Dependencies

The completed system should assume Rust stable plus the `rustfmt` and `clippy` components, and the `wasm32-unknown-unknown` target. If the repository wants a fully reproducible contributor setup, add `rust-toolchain.toml` so those components and targets are declared explicitly instead of relying on contributor memory.

The shared local and CI interface should be a small number of human-readable commands, ideally wrapped in `scripts/quality-gate.sh` with modes such as `fmt`, `check`, `clippy`, `test`, `smoke`, `parity`, and `wasm`. That script is not required by Cargo, but it is required by maintainability because it removes policy drift between local docs and CI configuration.

The proposed smoke interface is a new script such as `scripts/check_playtest_smoke.py`. Its contract should be simple: accept the path to a JSONL log, parse it, assert a small set of deterministic minimums, print a concise summary, and exit with status 0 on success and nonzero on failure. The thresholds should be deliberately loose enough to catch “the simulation never really ran” without turning a smoke gate into a balance gate.

The proposed coverage dependency is `cargo-llvm-cov`, installed locally through Cargo and in CI through a tool-install action. The artifact interface should include LCOV for machine consumption and HTML for human inspection.

The proposed parity interface remains the existing `scripts/eval_parity.py` plus `scripts/golden_values.json`. The critical policy dependency is not technical but procedural: before this job becomes blocking, the team must decide whether the current failing seed-42 result is incorrect and should be fixed, or correct enough to become the new golden baseline.
