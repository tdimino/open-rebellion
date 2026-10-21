# Open Rebellion

Open Rebellion is a Rust reimplementation of Star Wars Rebellion (1998), with
native macroquad/egui and browser WebAssembly/WebGL targets.

The September 2026 audit is authoritative. The project is not yet 100%
functional; historical parity percentages are estimates, not acceptance proof.

## Active Work

- Start with `progress.json`, the ignored live recovery record.
- Use `docs/qa/2026-09-08-full-functionality-audit/README.md` for the audit index,
  feature ledger, evidence contract, and definition of done.
- Use `agent_docs/roadmap.md` for milestone order, including the protected
  Cloudflare Pages v1.0 release.
- Use `agent_docs/agent-tooling.md` to select Codex, Fable, Ghidra, QA, asset,
  and deployment workflows without loading unrelated skills.
- Complete one feature at a time. Update progress JSON, audit JSON, audit
  Markdown, and roadmap evidence together. Never check off a partial or
  inferred result.
- Work directly on `main`. Do not create, delete or check out branches without
  Tom's say-so; the `pr-*` and review refs are not work targets.
- Drive the browser build only through the `tools/interface-parity` gates or
  Astra, never `agent-browser` (kept for non-game pages).
- Use Astra only for live browser or computer-use acceptance through
  `codex-orchestrator`, always at medium effort. Do not assign Astra source,
  implementation, reverse-engineering, or documentation reviews. Retain its
  inspected screenshots, console/network logs, and artifact hashes.
- Use Codex Sol through `codex-orchestrator`, at high or extra-high effort,
  when risk warrants independent code, reverse-engineering, architecture,
  persistence/network, security, or documentation review. Routine, well-tested
  feature slices do not require a Sol review.
- Run native GUI acceptance on the Mini with the game's developer commands
  (`agent_docs/dev-commands.md`, "The loop": `launch-native.sh --live`, then
  `dev-send.sh` or a `dev-run.sh` batch); fall back to `cua-driver` for what
  the game cannot do itself (`agent_docs/agent-tooling.md`, "Native GUI
  Acceptance").
- Keep every run silent on this Mac and on the Mini: native games only through
  `scripts/launch-native.sh`, every browser with Chromium
  `--mute-audio --disable-audio-output`, and `pmset -g assertions | grep -B1
  audio-out` clean on both machines (`agent_docs/agent-tooling.md`, "Keep
  every run silent"). Close the test browser and local server when the run
  finishes.
- Settle game behaviour against the original game, not the trace alone:
  strategic checks start on Wine on the Mini (`wine-game.sh`); tactical
  battles, A0 frames and sound stay on the Windows VM (`vm-game.sh`), which
  also settles any result where the two disagree. Drive Wine's mouse while
  the VM is in use only when `wine-game.sh status` prints `mouse=drive64.exe`.
  Take A0 frames only from `up-a0` and `a0`, never `-w`. Keep VM storage on
  the Mini and pull only captures and hashes. Start from a save in
  `tools/original-capture/saves/INDEX.md` or an edited copy of one
  (`agent_docs/original-save-editing.md`); the rest is in
  `tools/original-capture/README.md`.
- Read the manual before and after each interface feature and settle its
  claims against the trace (`agent_docs/manual-cross-check.md`).
- Take tactical, ground-assault and Ghidra work from the br/bv task graph in
  `.beads/` (`agent_docs/task-graph.md`); after any `br` change, run
  `scripts/task-index.py` to regenerate the pages in `tasks/`.
- `agent_docs/INDEX.md` lists every agent doc and tool README.
- GitHub Actions workflow definitions are intentionally local and untracked as
  of 2026-09-08. Verification is manual until M5 restores a reviewed CI provider.

## Commands

Cargo runs on the Mac Mini, never on this Mac: `scripts/mini-cargo.sh` syncs
the tree there, runs Cargo with the sanitized PATH, and cuts the Mini's
`target/` back to what its recent builds used; this Mac keeps no `target/`
(`agent_docs/agent-tooling.md`, "Builds and the Build Cache"):

```bash
scripts/mini-cargo.sh -- check --workspace
scripts/mini-cargo.sh -- test --workspace
scripts/mini-cargo.sh -- build --release -p rebellion-app
scripts/mini-cargo.sh --run ./scripts/build-wasm.sh --pull web/open-rebellion.wasm
scripts/mini-cargo.sh --run './scripts/package-web.sh dev' --pull dist/open-rebellion-web-dev/
scripts/gates.sh   # the pre-commit gates (agent_docs/test-tooling.md)
```

Formatting and strict Clippy have known audited baseline failures. Run scoped
checks for touched code; do not report the workspace gates as green until their
dedicated findings close.

## Structure

- `crates/rebellion-core`—headless runtime types and simulation.
- `crates/rebellion-data`—DAT loading, seeding, and persistence.
- `crates/rebellion-render`—macroquad/egui rendering and interaction.
- `crates/rebellion-app`—native/WASM entry point and interactive loop.
- `crates/rebellion-playtest`—headless campaigns and JSONL telemetry.
- `crates/rebellion-windowing`—renderer-independent strategic window
  identity, policy and routing.
- `tools/dat-dumper`—DAT codecs and TEXTSTRA extraction.
- `tools/provenance-scan`—the citation gate for simulation rules.
- `web` and `scripts`—browser shell, WASM builds, packaging, asset staging.
- `agent_docs` and `docs`—architecture, plans, mechanics, and QA evidence.

## Conventions

- Keep binary-layout `dat` types separate from runtime `world` types.
- Preserve original `DatId` identity; never persist slotmap keys directly.
- Require every DAT parser to round-trip original bytes through `DatRecord`.
- Keep `rebellion-core` free of rendering and platform I/O dependencies.
- Express simulation mutations as ordered `GameEffect` values applied by the
  authoritative integrator; economy runs before manufacturing.
- Prove bitmap display with a resource mapping, cache hit, and inspected runtime
  screenshot. Decode or HTTP success alone is insufficient.
- Keep password/session material in encrypted Cloudflare secrets. Never commit
  credentials, original game data, or generated proprietary asset packs; the
  one exception is the saves in `tools/original-capture/saves/` (ours from the
  capture VM, plus the one David Adams gave us, credited there).
- Follow the testing rules in `CLAUDE.md`: every feature and gate covers the
  Empire and the Alliance, behavior-sentence test names, cited sources for
  original behavior, no weakened assertions, goldens regenerated only for a
  named cause, new tests shown to fail, a scoped `cargo mutants` run before
  committing, and a source or `port:`/`hyp:` tag on every new simulation rule.
- Tracked docs state current facts and name evidence by save, day and values,
  never by an `.artifacts/` path. Ghidra notes carry no history
  (`agent_docs/ghidra-re.md`, "Porting an original flow").

## Boundaries

- Always: Preserve unrelated user changes and stage explicit paths only.
- Always: Verify in proportion to risk and record exact commands/results.
- Always: Wait for Tom to say "commit" or "push"; then use conventional commit
  messages, amend review fixes only before a push, and never force-push `main`.
- Ask: Before adding production dependencies, changing architecture, or breaking
  save, DAT, or network formats.
- Never: Run `gh auth setup-git` or switch the active `gh` account to repair a
  push; use the macOS keychain and repository-scoped credential username.
- Never: Claim 100% functionality until every supported P00–P40 pass is green
  and all P0/P1 findings are closed with release-artifact evidence.

## Troubleshooting

- Linker invokes a Claude/tmux launcher: Cargo ran locally; run it through
  `scripts/mini-cargo.sh`, which sets the sanitized PATH on the Mini.
- Browser boots black: wait for the bitmap manifest and all staged assets, then
  inspect console/network logs before assigning a visual result.
- Push omits `main`: run `git push origin main` and compare `HEAD` with upstream.
- Pages/Jekyll reports a missing local path: inspect tracked symlinks; local
  scratch links must remain ignored.
