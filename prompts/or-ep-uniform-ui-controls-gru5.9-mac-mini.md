# Mac Mini validation prompt: uniform UI controls adoption

Work on bead `or-ep-uniform-ui-controls-gru5.9` in the existing Open Rebellion checkout on Will's Mac Mini. This is Will's Mini, not Tom's `mac-mini-ts`: run Cargo and the repository gates locally on this machine. Do not SSH to Tom's Mini and do not use `scripts/mini-cargo.sh`.

Use the already-pushed branch `refactor/strategic-window-components` from remote `will-forster`. Do not create, delete, or switch branches. Before changing anything, confirm that the current branch is exactly `refactor/strategic-window-components`, fetch `will-forster`, confirm the branch contains the remote tip, and record the resolved commit hash. If the checkout is on another branch or cannot be updated without overwriting local work, stop and report that instead of forcing it.

Read `AGENTS.md`, `progress.json`, `ghidra/notes/cool-control-migration-matrix.md`, `agent_docs/agent-tooling.md`, `agent_docs/test-tooling.md`, and the interface-parity README(s) relevant to the journeys below. Then run:

```sh
br dep tree or-ep-uniform-ui-controls-gru5.9
br show or-ep-uniform-ui-controls-gru5.9
```

Confirm every blocking component bead is closed and `.9` is ready and unclaimed. Atomically claim it with your actual Agent Mail identity:

```sh
br update or-ep-uniform-ui-controls-gru5.9 --claim --actor YOUR_AGENT_MAIL_NAME --json
scripts/task-index.py
```

Preserve all unrelated dirty and staged work. Stage explicit paths only. Do not commit or push unless Will explicitly authorizes it after reviewing the results.

## Objective

Complete the final cross-family acceptance audit for uniform original-game controls. Account for every production consumer in the reviewed migration matrix with either:

- its canonical shared renderer control object and migrated production call sites, plus observable tests; or
- a specific source-backed exclusion.

Remove only genuinely obsolete duplicate helpers, state, or imports found during the audit. Keep domain actions, feature lifecycle, top-level window policy, faction resource selection, and renderer/windowing boundaries in their current owning layers. Do not chase the normalized clone percentage by abstracting semantically different loops, configuration, macros, or tests.

Audit all control families and every matrix row, including the `.10` title-drag work already present on the branch. Pay particular attention to `StrobeButton`, `Checkbox`, `TabControl`, `DragList`, `ScrollBar`, `SelectionBox`, `StringField`, `Slider`, `StatusBar`, tooltip registration/state, and strategic title drag.

## Required static and test evidence

Record exact commands and results. Run the inventory searches documented in the matrix, all applicable component/family tests, and these same-flags clone scans using `jscpd 5.4.3`:

```sh
jscpd --reporters ai --format rust --min-tokens 70 --no-tips \
  crates/rebellion-render/src crates/rebellion-windowing/src

jscpd --reporters ai --format rust --min-tokens 70 \
  --ignore-identifiers --ignore-literals --no-tips \
  crates/rebellion-render/src crates/rebellion-windowing/src
```

The reviewed baseline was zero exact clones and 263 normalized candidates at 3.2 percent. A changed count is not automatically a regression or success: identify removed selected families and document every intentional relevant survivor. Exact clones must remain zero.

Inspect existing per-component mutation evidence. Run scoped `cargo mutants` for any changed control kernel and for any required family whose evidence is missing or stale. Do not substitute an unfocused workspace-wide mutation campaign. No unexplained non-equivalent survivor may remain.

Run all local repository readiness gates on this Mini:

```sh
make fmt
make build
make check
./scripts/build-wasm.sh
bash scripts/build-interface-test-wasm.sh
```

`make build` must be warning-free. Record the production and interface-test WASM hashes. A stale WASM artifact is not acceptance evidence.

## Silent browser and native acceptance

Follow the repository's silence rules exactly. Launch Chromium only through `tools/interface-parity` with `--mute-audio --disable-audio-output`; never use `agent-browser`. Verify `pmset -g assertions | grep -B1 audio-out` remains clean. Close every test browser and local server when finished.

Derive the definitive journey list from every matrix row. At minimum, run the applicable current control/window coverage for both Alliance and Empire and every supported viewport, including:

- `game-options-smoke.mjs`
- `build-selection.mjs`
- `mission-dialog.mjs`
- `fleet-finder.mjs`
- `fleet-move.mjs`
- `fleet-window.mjs`
- `sector-quadrants.mjs`
- `status-window.mjs`
- `orbital-strike.mjs`
- `encyclopedia-index-shell.mjs` and `encyclopedia-canonical-surface.mjs`
- `message-index-shell.mjs`
- `audit-batch.mjs`
- the matrix-cited `run.mjs --all` tactical scenarios for Battle Alert, Battle Results, Game Options, selected-unit/status meters, and Death Star charge

Use only the applicable current harness commands documented in `tools/interface-parity`; do not invent flags from this prompt. Retain inspected screenshots, console and network logs, hashes, faction, and viewport for every required cell. Treat any missing faction/viewport cell, console error, network error, black boot, stale build, or unsupported fixture as a failure to investigate or report—not as a pass. If native GUI acceptance is required for a surface the browser harness cannot prove, use the documented local native loop on this Mini through `scripts/launch-native.sh` and keep it silent.

## Evidence and closure discipline

Update `ghidra/notes/cool-control-migration-matrix.md` and the applicable tracked architecture/control evidence with current facts and commands. Tracked documentation must not cite scratch `.artifacts/` paths. Update `progress.json` and audit/roadmap evidence only where this bead genuinely changes their current facts.

Tick each `.9` acceptance item only after its evidence passes. After every `br` mutation, run `scripts/task-index.py`. Do not close `.9` around any failing or unavailable required gate. If all five acceptance criteria pass, close the bead with a precise evidence summary and regenerate tasks; otherwise leave it in progress with the exact blocker recorded.

Your final report must include:

- resolved branch and commit hash;
- files changed, if any;
- final inventory disposition for every matrix row;
- exact and normalized clone counts and documented survivors;
- component/family test and mutation results, including mutant counts;
- `make fmt`, warning-free `make build`, and `make check` results;
- production and interface-test WASM hashes;
- each interface journey's faction/viewport result and retained evidence location;
- silence checks and confirmation that browsers/servers were closed;
- bead acceptance/closure state; and
- every failure or limitation, without calling the bead ready if a required gate did not pass.
