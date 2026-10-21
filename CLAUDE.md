---
title: "Open Rebellion"
description: "Claude Code instructions for the Open Rebellion Rust reimplementation"
updated: 2026-10-20
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
- Never mark a partial or inferred result complete.
- Work directly on `main`. Remote-tracking refs also hold contributor PR heads
  (`pr-*`) and a few review or test branches; they are not work targets.
  Do not create, delete or check out branches without Tom's say-so.
- Drive the browser build only through the `tools/interface-parity` gates, or
  through `codex-orchestrator` with Astra, always at medium effort; never
  `agent-browser`, which stays for non-game pages
  (`agent_docs/agent-tooling.md`, "Browser Acceptance"). Astra takes live
  browser acceptance only, never source, reverse-engineering or doc reviews.
- Use Codex Sol through `codex-orchestrator`, at high or extra-high effort,
  when risk warrants an independent review (architecture, reverse
  engineering, persistence or network formats, security); routine,
  well-tested slices need none (`agent_docs/agent-tooling.md`, "Required
  Review Lanes").
- Run native GUI acceptance on the Mini with the game's developer commands
  (`agent_docs/dev-commands.md`, "The loop": `launch-native.sh --live`, then
  `dev-send.sh` or a `dev-run.sh` batch). Fall back to `cua-driver` for what
  the game cannot do itself (`agent_docs/agent-tooling.md`, "Native GUI Acceptance").
- Every run is silent on this Mac and on the Mini: native games only
  through `scripts/launch-native.sh`, browsers always `--mute-audio --disable-audio-output`, and
  `pmset -g assertions | grep -B1 audio-out` clean on both machines
  (`agent_docs/agent-tooling.md`, "Keep every run silent"). Close the test
  browser and local server when the run finishes.
- Settle game behaviour against the original game itself, not the trace
  alone. Strategic checks (galaxy play, saves, messages, Battle Alerts,
  play-forward to a day) start on Wine on the Mini
  (`tools/original-capture/wine-game.sh`, up to 16 times faster); tactical
  battles, A0 frames and sound stay on the Windows VM (`vm-game.sh`), which
  also settles any result where the two disagree. Never drive Wine's mouse
  while the VM is in use unless `wine-game.sh status` prints
  `mouse=drive64.exe`. A0 frames come only from `up-a0` and `a0`, never
  `-w`. VM storage stays on the Mini; pull only captures and hashes. Start
  from a save in `tools/original-capture/saves/INDEX.md`, or edit a copy of
  one (`agent_docs/original-save-editing.md`). Everything else:
  `tools/original-capture/README.md`.
- v1.0 is the complete browser build deployed through password-protected
  Cloudflare Pages; credentials belong in encrypted secrets, never in Git.
- GitHub Actions workflow definitions are intentionally local and untracked as
  of 2026-09-08. Until M5 restores a reviewed provider, verification is manual.

## Commands

Cargo runs on the Mac Mini, never on this Mac: `scripts/mini-cargo.sh`
syncs the tree there, runs Cargo with the sanitized PATH, and cuts the Mini's
`target/` back to what its recent builds used (`agent_docs/agent-tooling.md`,
"Builds and the Build Cache"):

```bash
scripts/mini-cargo.sh -- check --workspace
scripts/mini-cargo.sh -- test --workspace
scripts/mini-cargo.sh -- build --release -p rebellion-app   # native runs use it on the Mini
scripts/mini-cargo.sh --run ./scripts/build-wasm.sh --pull web/open-rebellion.wasm
scripts/mini-cargo.sh --run './scripts/package-web.sh dev' --pull dist/open-rebellion-web-dev/
```

This Mac keeps no `target/`; the SessionStart hook deletes one.

Formatting and strict Clippy currently have audited baseline failures. Run
scoped checks for touched code and do not describe those workspace gates as
green until their dedicated findings close.

## Architecture

- `crates/rebellion-core`—headless runtime types and simulation; no rendering.
- `crates/rebellion-data`—DAT loading, seeding, and persistence.
- `crates/rebellion-render`—macroquad/egui rendering and interaction panels.
- `crates/rebellion-app`—native/WASM entry point and interactive loop.
- `crates/rebellion-playtest`—headless campaign runner and JSONL telemetry.
- `crates/rebellion-windowing`—renderer-independent strategic window
  identity, policy and routing vocabulary.
- `tools/dat-dumper`—DAT codecs and TEXTSTRA extraction.
- `tools/provenance-scan`—the citation gate for simulation rules (Testing).
- `web` and `scripts`—browser shell, WASM build, packaging, and asset staging.

## Invariants

- Keep binary-layout `dat` types separate from runtime `world` types.
- Preserve original `DatId` identity; never persist slotmap keys directly.
- Every DAT parser must round-trip the original bytes through `DatRecord`.
- Keep `rebellion-core` free of rendering and platform I/O dependencies.
- Route simulation mutations through ordered `GameEffect` values and the
  authoritative integrator; economy precedes manufacturing.
- Do not claim bitmap correctness from decode or HTTP success alone. Require a
  resource mapping, cache hit, and inspected screenshot on each claimed platform.
- Preserve user changes and unrelated untracked files.
- Tracked docs state current facts and name evidence by save, day and values,
  never by an `.artifacts/` path. Ghidra notes carry no history
  (`agent_docs/ghidra-re.md`, "Porting an original flow").

## Testing

- Every feature, gate and acceptance run covers the Empire and the Alliance
  alike; the sides often run separate classes, slot numbers and cockpit geometry.
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
  explain each surviving mutant (`agent_docs/agent-tooling.md`, "Test Quality Gates").
- New simulation rules carry a source (`FUN_`, `DAT_`/`_DAT_`, GNPRTB, DAT)
  or a `port:`/`hyp:` tag; the provenance baseline of uncited items may only
  shrink (`agent_docs/agent-tooling.md`, "Provenance Gate").
- `scripts/gates.sh` runs the automated pre-commit gates (workspace tests with
  and without the fixture feature, the ignored replay golden, telemetry
  coverage, provenance, WASM, the task-graph leak greps) in one Mini session;
  `cargo mutants` and `scripts/task-index.py --check` run separately
  (`agent_docs/test-tooling.md`).

## Git and Dependencies

- No commit or push until Tom says "commit" or "push". Use conventional
  commits; amend review fixes into a commit only before it is pushed, and
  never force-push `main`. Stage explicit files, never broad paths.
- Never run `gh auth setup-git` or switch the active `gh` account to fix a push.
  Git pushes use the macOS keychain and the repository-scoped credential user.
- Ask before adding production dependencies, changing architecture, or breaking
  save/data/network formats.
- Never commit original game data, generated bitmap packs, secrets, or tokens.
  The one exception is the saves in `tools/original-capture/saves/`: ours
  from the capture VM, plus the one David Adams gave us (credited there).

## Detailed Guides

- `docs/qa/2026-09-08-full-functionality-audit/README.md`—audit entry point
  and feature ledger; `audit-report.md` beside it holds the evidence contract
  and the definition of 100% functional. Read for all work.
- `agent_docs/INDEX.md`—every agent doc and tool README. Read when no guide below fits.
- `agent_docs/manual-cross-check.md`—read the manual before and after each
  interface feature and settle its claims against the trace. Read for UI work.
- `agent_docs/roadmap.md`—active milestones and v1.0 release sequence. Read when
  choosing or checking off work.
- `agent_docs/task-graph.md`—the br/bv task graph in `.beads/`: question,
  review, port and acceptance nodes. Read before choosing, delegating or
  closing tactical, ground-assault or Ghidra work. After any br change, run
  `scripts/task-index.py` to regenerate the public pages in `tasks/`.
- `agent_docs/mod-surface.md`—what the original fixes in code rather than
  data, and the rules for turning a fixed rule into mod data. Read before
  mod or era-overhaul work.
- `agent_docs/architecture.md`—crate graph and data flow. Read before structural
  changes.
- `agent_docs/simulation.md`—tick order and system patterns. Read for game logic.
- `agent_docs/save-load.md`—save schema and the no-migration rule. Read for persistence.
- `agent_docs/assets.md`—bitmap and HD pipeline. Read for visual asset work.
- `agent_docs/dat-formats.md`—binary formats and codec rules. Read for DAT work.
- `agent_docs/dev-commands.md`—the developer commands and live channel that
  drive a native build (capture, scripted input, battles, messages). Read
  before any native acceptance run.
- `tools/interface-parity/README.md`—the browser gates. Read before browser acceptance.
- `agent_docs/agent-tooling.md`—skill routing, browser and native acceptance,
  silent runs, mutants, provenance, Mini builds and the build cache. Read
  before delegating, building or gating.
