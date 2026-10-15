---
title: "Open Rebellion"
description: "Claude Code instructions for the Open Rebellion Rust reimplementation"
updated: 2026-09-08
---

# Open Rebellion

Open Rebellion is a Rust, macroquad, and egui reimplementation of Star Wars
Rebellion (1998). It runs natively and as WebAssembly/WebGL in the browser.

The September 2026 audit is the source of truth: the project is not yet 100%
functional. Do not repeat historical parity percentages as verified results.

## Current Workflow

- Read `progress.json` first. It is the ignored live recovery record; the older
  native-video record is `archive/progress.archived-2026-04-07-native-video.json`.
- Work one audit feature at a time and keep its JSON, Markdown, roadmap, and
  progress evidence synchronized.
- After a feature passes its required gates, commit and push it before starting
  the next feature. Never mark a partial or inferred result complete.
- Work directly on `main`; the repository intentionally carries no other local
  or origin branches during this audit.
- Run browser acceptance yourself with `agent-browser`, or through
  `codex-orchestrator` with Astra at medium effort. Either way, retain
  screenshots, network logs, console logs, and artifact hashes.
- Run native GUI acceptance with the game's developer commands:
  `scripts/launch-native.sh --live`, then `scripts/dev-send.sh` per step
  (`Capture`, `Click`, `Battle at` …; `agent_docs/dev-commands.md`). Fall
  back to `cua-driver` for what the game cannot do itself
  (`agent_docs/agent-tooling.md`, "Native GUI Acceptance").
- Settle game behaviour against the original game itself, not the trace
  alone. The unmodified original runs in the Windows VM on the Mac Mini;
  drive it from this Mac with `tools/original-capture/vm-game.sh` (`up`,
  `click`, `key`, `cap`) and start from a save in
  `tools/original-capture/saves/INDEX.md`. Setup, VNC caveats and save
  transfer: `tools/original-capture/README.md`. The game is scaled to
  800×600, so A0 frames come only from `up-a0` and `a0`, which capture in the
  guest and drop the point filter's repeated lines exactly; never `-w`, which
  renders differently (README, "A0 captures"). VM storage stays on the Mini;
  pull only captures and hashes. To reach a state normal play cannot stage
  (a damaged hyperdrive, a Death Star), edit a copy of one of our saves:
  `agent_docs/original-save-editing.md`.
- v1.0 is the complete browser build deployed through password-protected
  Cloudflare Pages; credentials belong in encrypted secrets, never in Git.
- GitHub Actions workflow definitions are intentionally local and untracked as
  of 2026-09-08. Until M5 restores a reviewed provider, verification is manual.

## Commands

Cargo runs on the Mac Mini, never on this Mac: `scripts/mini-cargo.sh`
syncs the tree there, runs Cargo with the sanitized PATH, and cuts the Mini's
`target/` back to what that build used (`agent_docs/agent-tooling.md`,
"Builds and the Build Cache"):

```bash
scripts/mini-cargo.sh -- check --workspace
scripts/mini-cargo.sh -- test --workspace
scripts/mini-cargo.sh -- build --release -p rebellion-app   # native runs use it on the Mini
scripts/mini-cargo.sh --run ./scripts/build-wasm.sh --pull web/open-rebellion.wasm
./scripts/package-web.sh dev
```

A local build leaves a `target/` here that the SessionStart hook deletes.
If one is unavoidable, `~/.local/bin/cc` is not a C compiler: use
`env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/tomdimino/.cargo/bin cargo …`.

Formatting and strict Clippy currently have audited baseline failures. Run
scoped checks for touched code and do not describe those workspace gates as
green until their dedicated findings close.

## Architecture

- `crates/rebellion-core` — headless runtime types and simulation; no rendering.
- `crates/rebellion-data` — DAT loading, seeding, and persistence.
- `crates/rebellion-render` — macroquad/egui rendering and interaction panels.
- `crates/rebellion-app` — native/WASM entry point and interactive loop.
- `crates/rebellion-playtest` — headless campaign runner and JSONL telemetry.
- `tools/dat-dumper` — DAT codecs and TEXTSTRA extraction.
- `web` and `scripts` — browser shell, WASM build, packaging, and asset staging.

## Invariants

- Keep binary-layout `dat` types separate from runtime `world` types.
- Preserve original `DatId` identity; never persist slotmap keys directly.
- Every DAT parser must round-trip the original bytes through `DatRecord`.
- Keep `rebellion-core` free of rendering and platform I/O dependencies.
- Route simulation mutations through ordered `GameEffect` values and the
  authoritative integrator; economy precedes manufacturing.
- Do not claim bitmap correctness from decode or HTTP success alone. Require a
  resource mapping, cache hit, and inspected screenshot on each claimed platform.
- Preserve user changes and unrelated untracked files. Do not stage broad paths.

## Testing

- Name each test as a behavior sentence, such as
  `pause_finishes_the_current_day_then_holds`.
- A test that asserts original-game behavior cites its source in a comment or
  constant: a Ghidra function, a resource ID, or a manual page. Tests of our
  own helpers and layout need no citation.
- Never delete, weaken, or broaden an assertion to make code pass. Change an
  expectation only with evidence that it was wrong.
- Regenerate replay goldens or fixtures only for a named cause, such as a save
  format change or recovered original behavior, and name it in the commit.
- When one change writes both code and its test, show the test fails without
  the change before calling it done.
- Before committing, run `cargo mutants` on the touched files and close or
  explain each surviving mutant. See `agent_docs/agent-tooling.md`.
- Before committing simulation code, run
  `scripts/mini-cargo.sh -- run -q -p provenance-scan -- check`. New rules
  carry a source (`FUN_`, `DAT_`/`_DAT_`, GNPRTB, DAT) or a `port:`/`hyp:`
  tag; the baseline of uncited items may only shrink.

## Git and Dependencies

- Use conventional, atomic commits and stage explicit files.
- Never run `gh auth setup-git` or switch the active `gh` account to fix a push.
  Git pushes use the macOS keychain and the repository-scoped credential user.
- Ask before adding production dependencies, changing architecture, or breaking
  save/data/network formats.
- Never commit original game data, generated bitmap packs, secrets, or tokens.
  The one exception is the saves in `tools/original-capture/saves/`: ours
  from the capture VM, plus the one David Adams gave us (credited there).

## Detailed Guides

- `docs/qa/2026-09-08-full-functionality-audit/README.md` — audit entry point,
  evidence contract, feature ledger, and definition of done. Read for all work.
- `agent_docs/manual-cross-check.md` — read the manual before and after each
  interface feature and settle its claims against the trace. Read for UI work.
- `agent_docs/roadmap.md` — active milestones and v1.0 release sequence. Read when
  choosing or checking off work.
- `agent_docs/task-graph.md` — the br/bv task graph in `.beads/`: question,
  review, port and acceptance nodes. Read before choosing, delegating or
  closing tactical, ground-assault or Ghidra work. After any br change, run
  `scripts/task-index.py` to regenerate the public pages in `tasks/`.
- `agent_docs/mod-surface.md` — what the original fixes in code rather than
  data, and the rules for turning a fixed rule into mod data. Read before
  mod or era-overhaul work.
- `agent_docs/architecture.md` — crate graph and data flow. Read before structural
  changes.
- `agent_docs/simulation.md` — tick order and system patterns. Read for game logic.
- `agent_docs/save-load.md` — save schema and migration notes. Read for persistence.
- `agent_docs/assets.md` — bitmap and HD pipeline. Read for visual asset work.
- `agent_docs/dat-formats.md` — binary formats and codec rules. Read for DAT work.
- `agent_docs/dev-commands.md` — the developer commands and live channel that
  drive a native build (capture, scripted input, battles, messages). Read
  before any native acceptance run.
- `agent_docs/agent-tooling.md` — Codex, Fable, Ghidra, QA, asset, and deployment
  skill routing. Read before delegating or selecting a specialized workflow.
